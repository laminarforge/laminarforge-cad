use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed sterile connector cap/plug management and reconciliation station.
//
// Design intent:
// - Prevent sterile connector, cap, and plug mixups during closed fluid-path
//   assembly and post-run teardown.
// - Keep connector family keying, clean cap parks, used cap quarantine, extra
//   and missing reconciliation lands, identity scan lands, leak-test handoff,
//   inspection optics, custody tokens, and robot/service keepouts on one
//   auditable fixture.
// - Model mechanical datums and purchased-equipment envelopes only. Actual
//   sterile connectors, barcode/RFID readers, cameras, pressure transducers,
//   and integrity-test methods remain validated purchased subsystems.
//
// Exports:
//   output/sterile_connector_cap_plug_reconciliation_station_deck_leak_tray.stl
//   output/sterile_connector_cap_plug_reconciliation_station_keyed_connector_family_sockets.stl
//   output/sterile_connector_cap_plug_reconciliation_station_clean_cap_plug_parks.stl
//   output/sterile_connector_cap_plug_reconciliation_station_used_cap_quarantine_wells.stl
//   output/sterile_connector_cap_plug_reconciliation_station_missing_extra_reconciliation_lands.stl
//   output/sterile_connector_cap_plug_reconciliation_station_barcode_rfid_scan_lands.stl
//   output/sterile_connector_cap_plug_reconciliation_station_leak_test_handoff_ports.stl
//   output/sterile_connector_cap_plug_reconciliation_station_clean_used_reject_lanes.stl
//   output/sterile_connector_cap_plug_reconciliation_station_optical_inspection_bridge.stl
//   output/sterile_connector_cap_plug_reconciliation_station_waste_offcut_trap.stl
//   output/sterile_connector_cap_plug_reconciliation_station_chain_of_custody_tokens.stl
//   output/sterile_connector_cap_plug_reconciliation_station_robot_service_keepouts.stl
//   output/sterile_connector_cap_plug_reconciliation_station_assembly.stl

const OUTPUTS: [&str; 13] = [
    "output/sterile_connector_cap_plug_reconciliation_station_deck_leak_tray.stl",
    "output/sterile_connector_cap_plug_reconciliation_station_keyed_connector_family_sockets.stl",
    "output/sterile_connector_cap_plug_reconciliation_station_clean_cap_plug_parks.stl",
    "output/sterile_connector_cap_plug_reconciliation_station_used_cap_quarantine_wells.stl",
    "output/sterile_connector_cap_plug_reconciliation_station_missing_extra_reconciliation_lands.stl",
    "output/sterile_connector_cap_plug_reconciliation_station_barcode_rfid_scan_lands.stl",
    "output/sterile_connector_cap_plug_reconciliation_station_leak_test_handoff_ports.stl",
    "output/sterile_connector_cap_plug_reconciliation_station_clean_used_reject_lanes.stl",
    "output/sterile_connector_cap_plug_reconciliation_station_optical_inspection_bridge.stl",
    "output/sterile_connector_cap_plug_reconciliation_station_waste_offcut_trap.stl",
    "output/sterile_connector_cap_plug_reconciliation_station_chain_of_custody_tokens.stl",
    "output/sterile_connector_cap_plug_reconciliation_station_robot_service_keepouts.stl",
    "output/sterile_connector_cap_plug_reconciliation_station_assembly.stl",
];

const CONNECTOR_FAMILIES: usize = 6;
const SOCKETS_PER_FAMILY: usize = 4;
const CONNECTOR_SOCKET_COUNT: usize = CONNECTOR_FAMILIES * SOCKETS_PER_FAMILY;
const CLEAN_CAP_ROWS: usize = 4;
const CLEAN_CAP_COLS: usize = 8;
const CLEAN_CAP_WELLS: usize = CLEAN_CAP_ROWS * CLEAN_CAP_COLS;
const CLEAN_PLUG_ROWS: usize = 2;
const CLEAN_PLUG_COLS: usize = 8;
const CLEAN_PLUG_WELLS: usize = CLEAN_PLUG_ROWS * CLEAN_PLUG_COLS;
const USED_CAP_WELLS: usize = 24;
const MISSING_LANDS: usize = CONNECTOR_FAMILIES;
const EXTRA_LANDS: usize = CONNECTOR_FAMILIES;
const BARCODE_LANDS: usize = 12;
const RFID_LANDS: usize = 8;
const LEAK_TEST_PORTS: usize = 12;
const CUSTODY_TOKEN_SLOTS: usize = 12;
const LANE_COUNT: usize = 3;
const ROBOT_KEEP_OUTS: usize = 4;

const DECK_X: f64 = 1180.0;
const DECK_Y: f64 = 760.0;
const DECK_Z: f64 = 18.0;
const LEAK_TRAY_X: f64 = 1070.0;
const LEAK_TRAY_Y: f64 = 648.0;
const LEAK_TRAY_Z: f64 = 24.0;
const RECESSED_FIELD_Z: f64 = 6.0;
const DRAIN_D: f64 = 12.0;
const MOUNT_HOLE_D: f64 = 6.6;

const SOCKET_PLATE_X: f64 = 540.0;
const SOCKET_PLATE_Y: f64 = 220.0;
const SOCKET_PLATE_Z: f64 = 26.0;
const SOCKET_CENTER_X: f64 = -260.0;
const SOCKET_CENTER_Y: f64 = 145.0;
const SOCKET_PITCH_X: f64 = 78.0;
const SOCKET_PITCH_Y: f64 = 42.0;
const SOCKET_D: f64 = 19.0;
const SOCKET_KEY_W: f64 = 6.0;

const CAP_PARK_X: f64 = 420.0;
const CAP_PARK_Y: f64 = 250.0;
const CAP_PARK_Z: f64 = 24.0;
const CAP_PARK_CENTER_X: f64 = 290.0;
const CAP_PARK_CENTER_Y: f64 = 132.0;
const CAP_WELL_D: f64 = 17.0;
const PLUG_WELL_D: f64 = 12.0;
const CAP_PITCH_X: f64 = 44.0;
const CAP_PITCH_Y: f64 = 38.0;

const QUARANTINE_X: f64 = 310.0;
const QUARANTINE_Y: f64 = 196.0;
const QUARANTINE_Z: f64 = 34.0;
const QUARANTINE_CENTER_X: f64 = 420.0;
const QUARANTINE_CENTER_Y: f64 = -142.0;
const QUARANTINE_WELL_D: f64 = 18.0;
const QUARANTINE_PITCH_X: f64 = 44.0;
const QUARANTINE_PITCH_Y: f64 = 42.0;

const RECON_PLATE_X: f64 = 500.0;
const RECON_PLATE_Y: f64 = 136.0;
const RECON_PLATE_Z: f64 = 18.0;
const RECON_CENTER_X: f64 = -250.0;
const RECON_CENTER_Y: f64 = -178.0;
const RECON_LAND_X: f64 = 58.0;
const RECON_LAND_Y: f64 = 42.0;
const RECON_LED_D: f64 = 6.0;

const SCAN_PANEL_X: f64 = 360.0;
const SCAN_PANEL_Y: f64 = 108.0;
const SCAN_PANEL_Z: f64 = 12.0;
const SCAN_CENTER_X: f64 = -385.0;
const SCAN_CENTER_Y: f64 = -314.0;

const LEAK_BAR_X: f64 = 440.0;
const LEAK_BAR_Y: f64 = 86.0;
const LEAK_BAR_Z: f64 = 30.0;
const LEAK_BAR_CENTER_X: f64 = 290.0;
const LEAK_BAR_CENTER_Y: f64 = -310.0;
const LEAK_PORT_D: f64 = 8.5;
const LEAK_PORT_PITCH_X: f64 = 32.0;

const LANE_BODY_X: f64 = 1030.0;
const LANE_BODY_Y: f64 = 70.0;
const LANE_BODY_Z: f64 = 22.0;
const LANE_CENTER_X: f64 = 0.0;
const LANE_CENTER_Y: f64 = 2.0;
const LANE_PITCH_Y: f64 = 26.0;

const BRIDGE_SPAN_X: f64 = 1010.0;
const BRIDGE_POST_X: f64 = 30.0;
const BRIDGE_POST_Y: f64 = 54.0;
const BRIDGE_CLEARANCE_Z: f64 = 136.0;
const BRIDGE_BEAM_Z: f64 = 28.0;
const BRIDGE_CENTER_X: f64 = 0.0;
const BRIDGE_CENTER_Y: f64 = 20.0;
const CAMERA_POD_COUNT: usize = 4;
const CAMERA_POD_X: f64 = 72.0;
const CAMERA_POD_Y: f64 = 46.0;
const CAMERA_POD_Z: f64 = 34.0;

const WASTE_TRAP_X: f64 = 196.0;
const WASTE_TRAP_Y: f64 = 132.0;
const WASTE_TRAP_Z: f64 = 42.0;
const WASTE_TRAP_CENTER_X: f64 = 0.0;
const WASTE_TRAP_CENTER_Y: f64 = -316.0;

const TOKEN_PLATE_X: f64 = 300.0;
const TOKEN_PLATE_Y: f64 = 96.0;
const TOKEN_PLATE_Z: f64 = 16.0;
const TOKEN_CENTER_X: f64 = -485.0;
const TOKEN_CENTER_Y: f64 = 286.0;
const TOKEN_SLOT_X: f64 = 42.0;
const TOKEN_SLOT_Y: f64 = 18.0;

const FRONT_ROBOT_KEEP_Y: f64 = 110.0;
const REAR_SERVICE_KEEP_Y: f64 = 86.0;
const LEFT_SERVICE_KEEP_X: f64 = 88.0;
const RIGHT_BIN_CLEARANCE_X: f64 = 92.0;
const KEEP_Z: f64 = 126.0;

fn main() {
    fs::create_dir_all("output").unwrap();

    let deck = deck_leak_tray();
    export(OUTPUTS[0], &deck);

    let sockets = keyed_connector_family_sockets();
    export(OUTPUTS[1], &sockets);

    let cap_parks = clean_cap_plug_parks();
    export(OUTPUTS[2], &cap_parks);

    let quarantine = used_cap_quarantine_wells();
    export(OUTPUTS[3], &quarantine);

    let recon = missing_extra_reconciliation_lands();
    export(OUTPUTS[4], &recon);

    let scans = barcode_rfid_scan_lands();
    export(OUTPUTS[5], &scans);

    let leak_ports = leak_test_handoff_ports();
    export(OUTPUTS[6], &leak_ports);

    let lanes = clean_used_reject_lanes();
    export(OUTPUTS[7], &lanes);

    let bridge = optical_inspection_bridge();
    export(OUTPUTS[8], &bridge);

    let waste = waste_offcut_trap();
    export(OUTPUTS[9], &waste);

    let tokens = chain_of_custody_tokens();
    export(OUTPUTS[10], &tokens);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[11], &keepouts);

    let assembly =
        deck + sockets.translate(
            SOCKET_CENTER_X,
            SOCKET_CENTER_Y,
            DECK_Z / 2.0 + LEAK_TRAY_Z + SOCKET_PLATE_Z / 2.0 + 4.0,
        ) + cap_parks.translate(
            CAP_PARK_CENTER_X,
            CAP_PARK_CENTER_Y,
            DECK_Z / 2.0 + LEAK_TRAY_Z + CAP_PARK_Z / 2.0 + 4.0,
        ) + quarantine.translate(
            QUARANTINE_CENTER_X,
            QUARANTINE_CENTER_Y,
            DECK_Z / 2.0 + QUARANTINE_Z / 2.0 + 4.0,
        ) + recon.translate(
            RECON_CENTER_X,
            RECON_CENTER_Y,
            DECK_Z / 2.0 + RECON_PLATE_Z / 2.0 + 4.0,
        ) + scans.translate(
            SCAN_CENTER_X,
            SCAN_CENTER_Y,
            DECK_Z / 2.0 + SCAN_PANEL_Z / 2.0 + 4.0,
        ) + leak_ports.translate(
            LEAK_BAR_CENTER_X,
            LEAK_BAR_CENTER_Y,
            DECK_Z / 2.0 + LEAK_BAR_Z / 2.0 + 4.0,
        ) + lanes.translate(
            LANE_CENTER_X,
            LANE_CENTER_Y,
            DECK_Z / 2.0 + LANE_BODY_Z / 2.0 + 4.0,
        ) + bridge.translate(
            BRIDGE_CENTER_X,
            BRIDGE_CENTER_Y,
            DECK_Z / 2.0 + LEAK_TRAY_Z + 4.0,
        ) + waste.translate(
            WASTE_TRAP_CENTER_X,
            WASTE_TRAP_CENTER_Y,
            DECK_Z / 2.0 + WASTE_TRAP_Z / 2.0 + 4.0,
        ) + tokens.translate(
            TOKEN_CENTER_X,
            TOKEN_CENTER_Y,
            DECK_Z / 2.0 + TOKEN_PLATE_Z / 2.0 + 4.0,
        ) + keepouts.translate(0.0, 0.0, DECK_Z / 2.0 + KEEP_Z / 2.0);

    export(OUTPUTS[12], &assembly);

    println!(
        "Sterile connector cap/plug reconciliation station: {:.0}mm x {:.0}mm deck, {} keyed connector sockets across {} families, {} clean cap wells, {} clean plug wells, {} used-cap quarantine wells, {} missing/extra reconciliation lands, {} barcode lands, {} RFID lands, {} leak-test handoff ports, {} custody token slots, and {} robot/service keepouts.",
        DECK_X,
        DECK_Y,
        CONNECTOR_SOCKET_COUNT,
        CONNECTOR_FAMILIES,
        CLEAN_CAP_WELLS,
        CLEAN_PLUG_WELLS,
        USED_CAP_WELLS,
        MISSING_LANDS + EXTRA_LANDS,
        BARCODE_LANDS,
        RFID_LANDS,
        LEAK_TEST_PORTS,
        CUSTODY_TOKEN_SLOTS,
        ROBOT_KEEP_OUTS
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn deck_leak_tray() -> Part {
    let deck = centered_cube("cap_plug_reconciliation_deck", DECK_X, DECK_Y, DECK_Z);
    let tray_socket = centered_cube(
        "cap_plug_reconciliation_leak_tray_registration_pocket",
        LEAK_TRAY_X + 28.0,
        LEAK_TRAY_Y + 28.0,
        RECESSED_FIELD_Z,
    )
    .translate(0.0, 12.0, DECK_Z / 2.0 - RECESSED_FIELD_Z / 2.0 + 0.5);
    let clean_field = centered_cube(
        "cap_plug_reconciliation_clean_field_recess",
        470.0,
        300.0,
        RECESSED_FIELD_Z,
    )
    .translate(CAP_PARK_CENTER_X, CAP_PARK_CENTER_Y, DECK_Z / 2.0 - 2.0);
    let used_field = centered_cube(
        "cap_plug_reconciliation_used_field_recess",
        360.0,
        238.0,
        RECESSED_FIELD_Z,
    )
    .translate(QUARANTINE_CENTER_X, QUARANTINE_CENTER_Y, DECK_Z / 2.0 - 2.0);
    let drain = centered_cylinder(
        "cap_plug_reconciliation_low_point_drain",
        DRAIN_D / 2.0,
        42.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 76.0, -DECK_Y / 2.0 + 40.0, 0.0);

    deck - tray_socket - clean_field - used_field - drain - deck_mount_holes()
        + leak_tray_insert()
        + deck_perimeter_lips()
        + deck_fiducials()
}

fn leak_tray_insert() -> Part {
    let tray = centered_cube(
        "cap_plug_reconciliation_removable_leak_tray",
        LEAK_TRAY_X,
        LEAK_TRAY_Y,
        LEAK_TRAY_Z,
    )
    .translate(0.0, 12.0, DECK_Z / 2.0 + LEAK_TRAY_Z / 2.0);
    let basin = centered_cube(
        "cap_plug_reconciliation_leak_tray_basin",
        LEAK_TRAY_X - 48.0,
        LEAK_TRAY_Y - 50.0,
        13.0,
    )
    .translate(0.0, 12.0, DECK_Z / 2.0 + LEAK_TRAY_Z - 5.0);
    let low_point = centered_cylinder(
        "cap_plug_reconciliation_leak_tray_drain_sump",
        28.0,
        14.0,
        48,
    )
    .translate(
        DECK_X / 2.0 - 76.0,
        -DECK_Y / 2.0 + 58.0,
        DECK_Z / 2.0 + LEAK_TRAY_Z - 5.0,
    );

    tray - basin - low_point + leak_tray_ribs()
}

fn leak_tray_ribs() -> Part {
    let mut ribs = Part::empty("cap_plug_reconciliation_leak_tray_ribs");
    for (i, x) in [-360.0, -180.0, 0.0, 180.0, 360.0].iter().enumerate() {
        ribs = ribs
            + centered_cube(
                format!("cap_plug_reconciliation_longitudinal_rib_{i}"),
                4.0,
                LEAK_TRAY_Y - 92.0,
                6.0,
            )
            .translate(*x, 12.0, DECK_Z / 2.0 + LEAK_TRAY_Z / 2.0 + 3.0);
    }
    for (i, y) in [-232.0, -116.0, 0.0, 116.0, 232.0].iter().enumerate() {
        ribs = ribs
            + centered_cube(
                format!("cap_plug_reconciliation_lateral_rib_{i}"),
                LEAK_TRAY_X - 96.0,
                4.0,
                6.0,
            )
            .translate(0.0, 12.0 + *y, DECK_Z / 2.0 + LEAK_TRAY_Z / 2.0 + 3.0);
    }
    ribs
}

fn deck_perimeter_lips() -> Part {
    let rear = centered_cube(
        "cap_plug_reconciliation_rear_wipe_lip",
        DECK_X - 110.0,
        14.0,
        28.0,
    )
    .translate(0.0, DECK_Y / 2.0 - 30.0, DECK_Z / 2.0 + 14.0);
    let left = centered_cube(
        "cap_plug_reconciliation_left_low_lip",
        14.0,
        DECK_Y - 136.0,
        24.0,
    )
    .translate(-DECK_X / 2.0 + 30.0, 0.0, DECK_Z / 2.0 + 12.0);
    let right = centered_cube(
        "cap_plug_reconciliation_right_low_lip",
        14.0,
        DECK_Y - 136.0,
        24.0,
    )
    .translate(DECK_X / 2.0 - 30.0, 0.0, DECK_Z / 2.0 + 12.0);
    let front = centered_cube(
        "cap_plug_reconciliation_front_robot_low_lip",
        DECK_X - 300.0,
        10.0,
        16.0,
    )
    .translate(-40.0, -DECK_Y / 2.0 + 28.0, DECK_Z / 2.0 + 8.0);

    rear + left + right + front
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("cap_plug_reconciliation_mount_holes");
    for (i, (x, y)) in deck_mount_points().iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("cap_plug_reconciliation_m6_mount_clearance_{i}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 4.0,
                28,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn deck_mount_points() -> [(f64, f64); 8] {
    [
        (-(DECK_X / 2.0 - 62.0), -(DECK_Y / 2.0 - 58.0)),
        (DECK_X / 2.0 - 62.0, -(DECK_Y / 2.0 - 58.0)),
        (-(DECK_X / 2.0 - 62.0), DECK_Y / 2.0 - 58.0),
        (DECK_X / 2.0 - 62.0, DECK_Y / 2.0 - 58.0),
        (-210.0, -(DECK_Y / 2.0 - 58.0)),
        (210.0, -(DECK_Y / 2.0 - 58.0)),
        (-210.0, DECK_Y / 2.0 - 58.0),
        (210.0, DECK_Y / 2.0 - 58.0),
    ]
}

fn deck_fiducials() -> Part {
    let mut fiducials = Part::empty("cap_plug_reconciliation_robot_fiducials");
    for (i, (x, y)) in [
        (-(DECK_X / 2.0 - 82.0), DECK_Y / 2.0 - 86.0),
        (DECK_X / 2.0 - 82.0, DECK_Y / 2.0 - 86.0),
        (-(DECK_X / 2.0 - 82.0), -(DECK_Y / 2.0 - 86.0)),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + fiducial_target(&format!("cap_plug_reconciliation_fiducial_{i}")).translate(
                *x,
                *y,
                DECK_Z / 2.0 + 2.0,
            );
    }
    fiducials
}

fn keyed_connector_family_sockets() -> Part {
    let plate = centered_cube(
        "cap_plug_reconciliation_keyed_connector_socket_plate",
        SOCKET_PLATE_X,
        SOCKET_PLATE_Y,
        SOCKET_PLATE_Z,
    );
    let mut socket_cuts = Part::empty("cap_plug_reconciliation_connector_socket_cuts");
    let mut family_labels = Part::empty("cap_plug_reconciliation_connector_family_label_lands");

    for family in 0..CONNECTOR_FAMILIES {
        for slot in 0..SOCKETS_PER_FAMILY {
            let x = socket_x(family);
            let y = socket_y(slot);
            let socket = centered_cylinder(
                format!("cap_plug_reconciliation_family_{family}_socket_{slot}"),
                (SOCKET_D + family as f64 * 1.2) / 2.0,
                SOCKET_PLATE_Z + 4.0,
                36,
            )
            .translate(x, y, 0.0);
            let key = family_key_cut(family, slot, x, y);
            socket_cuts = socket_cuts + socket + key;
        }

        family_labels = family_labels
            + centered_cube(
                format!("cap_plug_reconciliation_family_{family}_engraved_label_land"),
                58.0,
                10.0,
                2.5,
            )
            .translate(
                socket_x(family),
                SOCKET_PLATE_Y / 2.0 - 20.0,
                SOCKET_PLATE_Z / 2.0 + 1.5,
            );
    }

    plate - socket_cuts + family_labels + socket_family_dividers()
}

fn family_key_cut(family: usize, slot: usize, x: f64, y: f64) -> Part {
    let angle = (family as f64 % 4.0) * 45.0;
    let key = centered_cube(
        format!("cap_plug_reconciliation_family_{family}_socket_{slot}_asymmetric_key"),
        SOCKET_KEY_W,
        SOCKET_D + 8.0,
        SOCKET_PLATE_Z + 6.0,
    )
    .rotate(0.0, 0.0, angle)
    .translate(x, y, 0.0);

    let notch_x = if family % 2 == 0 { 8.0 } else { -8.0 };
    let notch = centered_cube(
        format!("cap_plug_reconciliation_family_{family}_socket_{slot}_notch"),
        8.0,
        18.0,
        SOCKET_PLATE_Z + 6.0,
    )
    .translate(x + notch_x, y + 8.0, 0.0);

    key + notch
}

fn socket_family_dividers() -> Part {
    let mut dividers = Part::empty("cap_plug_reconciliation_socket_family_dividers");
    for family in 1..CONNECTOR_FAMILIES {
        let x = socket_x(family) - SOCKET_PITCH_X / 2.0;
        dividers = dividers
            + centered_cube(
                format!("cap_plug_reconciliation_socket_family_divider_{family}"),
                3.0,
                SOCKET_PLATE_Y - 34.0,
                8.0,
            )
            .translate(x, 0.0, SOCKET_PLATE_Z / 2.0 + 4.0);
    }
    dividers
}

fn clean_cap_plug_parks() -> Part {
    let tray = centered_cube(
        "cap_plug_reconciliation_clean_cap_plug_park_tray",
        CAP_PARK_X,
        CAP_PARK_Y,
        CAP_PARK_Z,
    );

    let mut wells = Part::empty("cap_plug_reconciliation_clean_cap_plug_wells");
    for row in 0..CLEAN_CAP_ROWS {
        for col in 0..CLEAN_CAP_COLS {
            wells = wells
                + centered_cylinder(
                    format!("cap_plug_reconciliation_clean_cap_well_{row}_{col}"),
                    CAP_WELL_D / 2.0,
                    CAP_PARK_Z + 4.0,
                    32,
                )
                .translate(cap_grid_x(col), cap_grid_y(row), 0.0);
        }
    }
    for row in 0..CLEAN_PLUG_ROWS {
        for col in 0..CLEAN_PLUG_COLS {
            wells = wells
                + centered_cylinder(
                    format!("cap_plug_reconciliation_clean_plug_well_{row}_{col}"),
                    PLUG_WELL_D / 2.0,
                    CAP_PARK_Z + 4.0,
                    28,
                )
                .translate(cap_grid_x(col), plug_grid_y(row), 0.0);
        }
    }

    let mut lanes = Part::empty("cap_plug_reconciliation_clean_family_color_lanes");
    for family in 0..CONNECTOR_FAMILIES {
        lanes = lanes
            + centered_cube(
                format!("cap_plug_reconciliation_clean_family_{family}_swatch_land"),
                46.0,
                12.0,
                3.0,
            )
            .translate(
                -CAP_PARK_X / 2.0 + 44.0 + family as f64 * 62.0,
                CAP_PARK_Y / 2.0 - 24.0,
                CAP_PARK_Z / 2.0 + 1.5,
            );
    }

    tray - wells + lanes + clean_cap_pick_datums()
}

fn clean_cap_pick_datums() -> Part {
    let mut datums = Part::empty("cap_plug_reconciliation_clean_cap_pick_datums");
    for (i, (x, y)) in [
        (-(CAP_PARK_X / 2.0 - 26.0), -(CAP_PARK_Y / 2.0 - 24.0)),
        (CAP_PARK_X / 2.0 - 26.0, -(CAP_PARK_Y / 2.0 - 24.0)),
        (-(CAP_PARK_X / 2.0 - 26.0), CAP_PARK_Y / 2.0 - 54.0),
        (CAP_PARK_X / 2.0 - 26.0, CAP_PARK_Y / 2.0 - 54.0),
    ]
    .iter()
    .enumerate()
    {
        datums = datums
            + centered_cylinder(
                format!("cap_plug_reconciliation_clean_cap_datum_pin_{i}"),
                6.0,
                11.0,
                28,
            )
            .translate(*x, *y, CAP_PARK_Z / 2.0 + 5.5);
    }
    datums
}

fn used_cap_quarantine_wells() -> Part {
    let tray = centered_cube(
        "cap_plug_reconciliation_used_cap_quarantine_tray",
        QUARANTINE_X,
        QUARANTINE_Y,
        QUARANTINE_Z,
    );
    let basin = centered_cube(
        "cap_plug_reconciliation_used_cap_quarantine_basin",
        QUARANTINE_X - 28.0,
        QUARANTINE_Y - 30.0,
        16.0,
    )
    .translate(0.0, 0.0, QUARANTINE_Z / 2.0 - 6.0);

    let mut wells = Part::empty("cap_plug_reconciliation_used_cap_quarantine_well_cuts");
    for i in 0..USED_CAP_WELLS {
        let col = i % 6;
        let row = i / 6;
        wells = wells
            + centered_cylinder(
                format!("cap_plug_reconciliation_used_cap_quarantine_well_{i}"),
                QUARANTINE_WELL_D / 2.0,
                QUARANTINE_Z + 4.0,
                32,
            )
            .translate(
                (col as f64 - 2.5) * QUARANTINE_PITCH_X,
                (row as f64 - 1.5) * QUARANTINE_PITCH_Y,
                0.0,
            );
    }

    let quarantine_door_shadow = centered_cube(
        "cap_plug_reconciliation_quarantine_clear_lid_envelope",
        QUARANTINE_X + 24.0,
        QUARANTINE_Y + 18.0,
        10.0,
    )
    .translate(0.0, 0.0, QUARANTINE_Z / 2.0 + 14.0);

    tray - basin - wells + quarantine_door_shadow + quarantine_status_tabs()
}

fn quarantine_status_tabs() -> Part {
    let mut tabs = Part::empty("cap_plug_reconciliation_quarantine_status_tabs");
    for (i, x) in [-96.0, 0.0, 96.0].iter().enumerate() {
        tabs = tabs
            + centered_cube(
                format!("cap_plug_reconciliation_quarantine_status_tab_{i}"),
                72.0,
                18.0,
                4.0,
            )
            .translate(*x, -(QUARANTINE_Y / 2.0 - 18.0), QUARANTINE_Z / 2.0 + 2.0);
    }
    tabs
}

fn missing_extra_reconciliation_lands() -> Part {
    let plate = centered_cube(
        "cap_plug_reconciliation_missing_extra_plate",
        RECON_PLATE_X,
        RECON_PLATE_Y,
        RECON_PLATE_Z,
    );
    let mut lands = Part::empty("cap_plug_reconciliation_missing_extra_lands");
    let mut led_cuts = Part::empty("cap_plug_reconciliation_missing_extra_led_cuts");

    for family in 0..CONNECTOR_FAMILIES {
        let x = (family as f64 - 2.5) * 76.0;
        let missing = reconciliation_land(
            &format!("cap_plug_reconciliation_family_{family}_missing_land"),
            x,
            34.0,
        );
        let extra = reconciliation_land(
            &format!("cap_plug_reconciliation_family_{family}_extra_land"),
            x,
            -34.0,
        );
        lands = lands + missing + extra;

        for (j, y) in [34.0, -34.0].iter().enumerate() {
            led_cuts = led_cuts
                + centered_cylinder(
                    format!("cap_plug_reconciliation_family_{family}_status_led_{j}"),
                    RECON_LED_D / 2.0,
                    RECON_PLATE_Z + 4.0,
                    20,
                )
                .translate(x + 20.0, *y, 0.0);
        }
    }

    plate - led_cuts + lands
}

fn reconciliation_land(name: &str, x: f64, y: f64) -> Part {
    let land = centered_cube(
        format!("{name}_raised_pad"),
        RECON_LAND_X,
        RECON_LAND_Y,
        4.0,
    )
    .translate(x, y, RECON_PLATE_Z / 2.0 + 2.0);
    let scan_target = centered_cube(format!("{name}_scan_target"), RECON_LAND_X - 18.0, 8.0, 2.5)
        .translate(x, y - 12.0, RECON_PLATE_Z / 2.0 + 5.0);
    land + scan_target
}

fn barcode_rfid_scan_lands() -> Part {
    let panel = centered_cube(
        "cap_plug_reconciliation_barcode_rfid_panel",
        SCAN_PANEL_X,
        SCAN_PANEL_Y,
        SCAN_PANEL_Z,
    );
    let mut lands = Part::empty("cap_plug_reconciliation_barcode_rfid_lands");

    for i in 0..BARCODE_LANDS {
        let col = i % 6;
        let row = i / 6;
        lands = lands
            + centered_cube(
                format!("cap_plug_reconciliation_barcode_land_{i}"),
                46.0,
                16.0,
                3.0,
            )
            .translate(
                (col as f64 - 2.5) * 54.0,
                22.0 - row as f64 * 34.0,
                SCAN_PANEL_Z / 2.0 + 1.5,
            );
    }

    for i in 0..RFID_LANDS {
        let col = i % 4;
        let row = i / 4;
        lands = lands
            + centered_cube(
                format!("cap_plug_reconciliation_rfid_land_{i}"),
                34.0,
                20.0,
                2.5,
            )
            .translate(
                -135.0 + col as f64 * 90.0,
                -42.0 - row as f64 * 26.0,
                SCAN_PANEL_Z / 2.0 + 1.25,
            );
    }

    panel + lands
}

fn leak_test_handoff_ports() -> Part {
    let bar = centered_cube(
        "cap_plug_reconciliation_leak_test_port_bar",
        LEAK_BAR_X,
        LEAK_BAR_Y,
        LEAK_BAR_Z,
    );
    let mut port_cuts = Part::empty("cap_plug_reconciliation_leak_test_port_cuts");
    let mut port_labels = Part::empty("cap_plug_reconciliation_leak_test_port_labels");

    for i in 0..LEAK_TEST_PORTS {
        let x = (i as f64 - (LEAK_TEST_PORTS as f64 - 1.0) / 2.0) * LEAK_PORT_PITCH_X;
        let port = centered_cylinder(
            format!("cap_plug_reconciliation_leak_test_handoff_port_{i}"),
            LEAK_PORT_D / 2.0,
            LEAK_BAR_Y + 6.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 0.0, 3.0);
        let land = centered_cube(
            format!("cap_plug_reconciliation_leak_test_port_{i}_identity_land"),
            22.0,
            12.0,
            3.0,
        )
        .translate(x, -LEAK_BAR_Y / 2.0 - 8.0, LEAK_BAR_Z / 2.0 - 4.0);
        port_cuts = port_cuts + port;
        port_labels = port_labels + land;
    }

    bar - port_cuts + port_labels + leak_test_manifold_shadow()
}

fn leak_test_manifold_shadow() -> Part {
    centered_cube(
        "cap_plug_reconciliation_leak_test_manifold_clearance_shadow",
        LEAK_BAR_X - 54.0,
        16.0,
        12.0,
    )
    .translate(0.0, LEAK_BAR_Y / 2.0 + 10.0, LEAK_BAR_Z / 2.0 - 6.0)
}

fn clean_used_reject_lanes() -> Part {
    let body = centered_cube(
        "cap_plug_reconciliation_clean_used_reject_lane_body",
        LANE_BODY_X,
        LANE_BODY_Y,
        LANE_BODY_Z,
    );
    let mut grooves = Part::empty("cap_plug_reconciliation_lane_grooves");
    let mut labels = Part::empty("cap_plug_reconciliation_lane_status_lands");

    for lane in 0..LANE_COUNT {
        let y = (lane as f64 - 1.0) * LANE_PITCH_Y;
        let groove = centered_cube(
            format!("cap_plug_reconciliation_lane_{lane}_shallow_groove"),
            LANE_BODY_X - 62.0,
            12.0,
            LANE_BODY_Z + 2.0,
        )
        .translate(0.0, y, 4.0);
        let label = centered_cube(
            format!("cap_plug_reconciliation_lane_{lane}_status_label_land"),
            92.0,
            10.0,
            3.0,
        )
        .translate(-(LANE_BODY_X / 2.0 - 78.0), y, LANE_BODY_Z / 2.0 + 1.5);
        grooves = grooves + groove;
        labels = labels + label;
    }

    body - grooves + labels + segregation_fence()
}

fn segregation_fence() -> Part {
    let mut fences = Part::empty("cap_plug_reconciliation_lane_segregation_fences");
    for (i, y) in [-LANE_PITCH_Y / 2.0, LANE_PITCH_Y / 2.0].iter().enumerate() {
        fences = fences
            + centered_cube(
                format!("cap_plug_reconciliation_lane_fence_{i}"),
                LANE_BODY_X - 92.0,
                4.0,
                18.0,
            )
            .translate(28.0, *y, LANE_BODY_Z / 2.0 + 9.0);
    }
    fences
}

fn optical_inspection_bridge() -> Part {
    let left_post = centered_cube(
        "cap_plug_reconciliation_optics_left_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_CLEARANCE_Z + BRIDGE_BEAM_Z,
    )
    .translate(
        -BRIDGE_SPAN_X / 2.0,
        0.0,
        (BRIDGE_CLEARANCE_Z + BRIDGE_BEAM_Z) / 2.0,
    );
    let right_post = centered_cube(
        "cap_plug_reconciliation_optics_right_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_CLEARANCE_Z + BRIDGE_BEAM_Z,
    )
    .translate(
        BRIDGE_SPAN_X / 2.0,
        0.0,
        (BRIDGE_CLEARANCE_Z + BRIDGE_BEAM_Z) / 2.0,
    );
    let beam = centered_cube(
        "cap_plug_reconciliation_optics_camera_light_bridge",
        BRIDGE_SPAN_X + BRIDGE_POST_X,
        46.0,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, 0.0, BRIDGE_CLEARANCE_Z + BRIDGE_BEAM_Z / 2.0);

    let mut pods = Part::empty("cap_plug_reconciliation_optics_pods");
    for i in 0..CAMERA_POD_COUNT {
        let x = (i as f64 - 1.5) * 210.0;
        let pod = centered_cube(
            format!("cap_plug_reconciliation_camera_pod_{i}"),
            CAMERA_POD_X,
            CAMERA_POD_Y,
            CAMERA_POD_Z,
        )
        .translate(x, -28.0, BRIDGE_CLEARANCE_Z - CAMERA_POD_Z / 2.0 - 8.0);
        let lens_cut = centered_cylinder(
            format!("cap_plug_reconciliation_camera_pod_{i}_lens_clearance"),
            11.0,
            CAMERA_POD_Z + 4.0,
            32,
        )
        .translate(x, -28.0, BRIDGE_CLEARANCE_Z - CAMERA_POD_Z / 2.0 - 8.0);
        pods = pods + (pod - lens_cut);
    }

    let front_light_bar = centered_cube(
        "cap_plug_reconciliation_front_light_bar",
        BRIDGE_SPAN_X - 120.0,
        12.0,
        10.0,
    )
    .translate(0.0, -34.0, BRIDGE_CLEARANCE_Z - 18.0);
    let rear_light_bar = centered_cube(
        "cap_plug_reconciliation_rear_light_bar",
        BRIDGE_SPAN_X - 120.0,
        12.0,
        10.0,
    )
    .translate(0.0, 34.0, BRIDGE_CLEARANCE_Z - 18.0);

    left_post + right_post + beam + pods + front_light_bar + rear_light_bar
}

fn waste_offcut_trap() -> Part {
    let tray = centered_cube(
        "cap_plug_reconciliation_waste_offcut_trap_outer",
        WASTE_TRAP_X,
        WASTE_TRAP_Y,
        WASTE_TRAP_Z,
    );
    let basin = centered_cube(
        "cap_plug_reconciliation_waste_offcut_trap_basin",
        WASTE_TRAP_X - 26.0,
        WASTE_TRAP_Y - 24.0,
        24.0,
    )
    .translate(0.0, 0.0, WASTE_TRAP_Z / 2.0 - 8.0);
    let sweep_slot = centered_cube(
        "cap_plug_reconciliation_waste_offcut_sweep_slot",
        WASTE_TRAP_X - 48.0,
        20.0,
        18.0,
    )
    .translate(0.0, -WASTE_TRAP_Y / 2.0 + 18.0, WASTE_TRAP_Z / 2.0 - 5.0);
    let sharps_screen = centered_cube(
        "cap_plug_reconciliation_waste_offcut_retention_screen",
        WASTE_TRAP_X - 50.0,
        6.0,
        22.0,
    )
    .translate(0.0, 20.0, WASTE_TRAP_Z / 2.0 + 5.0);

    tray - basin - sweep_slot + sharps_screen + waste_trap_handles()
}

fn waste_trap_handles() -> Part {
    let left = centered_cube(
        "cap_plug_reconciliation_waste_trap_left_handle",
        18.0,
        68.0,
        12.0,
    )
    .translate(-(WASTE_TRAP_X / 2.0 + 9.0), 0.0, WASTE_TRAP_Z / 2.0 + 2.0);
    let right = centered_cube(
        "cap_plug_reconciliation_waste_trap_right_handle",
        18.0,
        68.0,
        12.0,
    )
    .translate(WASTE_TRAP_X / 2.0 + 9.0, 0.0, WASTE_TRAP_Z / 2.0 + 2.0);
    left + right
}

fn chain_of_custody_tokens() -> Part {
    let plate = centered_cube(
        "cap_plug_reconciliation_chain_of_custody_plate",
        TOKEN_PLATE_X,
        TOKEN_PLATE_Y,
        TOKEN_PLATE_Z,
    );
    let mut slots = Part::empty("cap_plug_reconciliation_chain_of_custody_slot_cuts");
    let mut land = Part::empty("cap_plug_reconciliation_chain_of_custody_scan_lands");

    for i in 0..CUSTODY_TOKEN_SLOTS {
        let col = i % 6;
        let row = i / 6;
        let x = (col as f64 - 2.5) * 46.0;
        let y = (row as f64 - 0.5) * 36.0;
        slots = slots
            + centered_cube(
                format!("cap_plug_reconciliation_custody_token_slot_{i}"),
                TOKEN_SLOT_X,
                TOKEN_SLOT_Y,
                TOKEN_PLATE_Z + 3.0,
            )
            .translate(x, y, 0.0);
        land = land
            + centered_cube(
                format!("cap_plug_reconciliation_custody_token_{i}_scan_land"),
                TOKEN_SLOT_X - 8.0,
                5.0,
                2.5,
            )
            .translate(x, y + 13.0, TOKEN_PLATE_Z / 2.0 + 1.25);
    }

    plate - slots + land
}

fn robot_service_keepouts() -> Part {
    let front_robot = centered_cube(
        "cap_plug_reconciliation_front_robot_approach_keepout",
        DECK_X - 180.0,
        FRONT_ROBOT_KEEP_Y,
        KEEP_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + FRONT_ROBOT_KEEP_Y / 2.0 + 10.0, 0.0);
    let rear_service = centered_cube(
        "cap_plug_reconciliation_rear_service_panel_keepout",
        DECK_X - 120.0,
        REAR_SERVICE_KEEP_Y,
        KEEP_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - REAR_SERVICE_KEEP_Y / 2.0 - 10.0, 0.0);
    let left_service = centered_cube(
        "cap_plug_reconciliation_left_scanner_service_keepout",
        LEFT_SERVICE_KEEP_X,
        DECK_Y - 170.0,
        KEEP_Z,
    )
    .translate(-DECK_X / 2.0 + LEFT_SERVICE_KEEP_X / 2.0 + 10.0, 0.0, 0.0);
    let right_bin = centered_cube(
        "cap_plug_reconciliation_right_quarantine_bin_clearance",
        RIGHT_BIN_CLEARANCE_X,
        DECK_Y - 190.0,
        KEEP_Z,
    )
    .translate(
        DECK_X / 2.0 - RIGHT_BIN_CLEARANCE_X / 2.0 - 10.0,
        -14.0,
        0.0,
    );

    front_robot + rear_service + left_service + right_bin
}

fn socket_x(family: usize) -> f64 {
    (family as f64 - (CONNECTOR_FAMILIES as f64 - 1.0) / 2.0) * SOCKET_PITCH_X
}

fn socket_y(slot: usize) -> f64 {
    (slot as f64 - (SOCKETS_PER_FAMILY as f64 - 1.0) / 2.0) * SOCKET_PITCH_Y - 10.0
}

fn cap_grid_x(col: usize) -> f64 {
    (col as f64 - (CLEAN_CAP_COLS as f64 - 1.0) / 2.0) * CAP_PITCH_X
}

fn cap_grid_y(row: usize) -> f64 {
    60.0 - row as f64 * CAP_PITCH_Y
}

fn plug_grid_y(row: usize) -> f64 {
    -112.0 + row as f64 * 32.0
}

fn fiducial_target(name: &str) -> Part {
    let pad = centered_cylinder(format!("{name}_pad"), 14.0, 3.0, 40);
    let center = centered_cylinder(format!("{name}_center_dot"), 2.2, 4.0, 20);
    let horizontal = centered_cube(format!("{name}_horizontal_crosshair"), 22.0, 1.8, 4.0);
    let vertical = centered_cube(format!("{name}_vertical_crosshair"), 1.8, 22.0, 4.0);
    pad - center - horizontal - vertical
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn station_fits_single_bench_tile() {
        assert!(DECK_X <= 1200.0);
        assert!(DECK_Y <= 800.0);
        assert!(BRIDGE_CLEARANCE_Z >= 120.0);
    }

    #[test]
    fn connector_reconciliation_capacity_matches_family_matrix() {
        assert_eq!(
            CONNECTOR_SOCKET_COUNT,
            CONNECTOR_FAMILIES * SOCKETS_PER_FAMILY
        );
        assert!(CONNECTOR_SOCKET_COUNT >= 24);
        assert_eq!(MISSING_LANDS + EXTRA_LANDS, CONNECTOR_FAMILIES * 2);
    }

    #[test]
    fn clean_inventory_exceeds_teardown_quarantine() {
        assert!(CLEAN_CAP_WELLS + CLEAN_PLUG_WELLS > USED_CAP_WELLS);
        assert!(CLEAN_CAP_WELLS >= CONNECTOR_SOCKET_COUNT);
        assert!(CLEAN_PLUG_WELLS >= LEAK_TEST_PORTS);
    }

    #[test]
    fn traceability_features_cover_every_family() {
        assert!(BARCODE_LANDS >= CONNECTOR_FAMILIES * 2);
        assert!(RFID_LANDS >= CONNECTOR_FAMILIES);
        assert!(CUSTODY_TOKEN_SLOTS >= CONNECTOR_FAMILIES * 2);
    }

    #[test]
    fn leak_handoff_and_lanes_are_sufficient_for_reconciliation() {
        assert!(LEAK_TEST_PORTS >= CONNECTOR_FAMILIES * 2);
        assert_eq!(LANE_COUNT, 3);
        assert!(WASTE_TRAP_X < QUARANTINE_X);
    }

    #[test]
    fn output_manifest_is_complete() {
        assert_eq!(OUTPUTS.len(), 13);
        assert_eq!(ROBOT_KEEP_OUTS, 4);
    }
}
