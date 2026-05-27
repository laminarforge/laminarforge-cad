use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed calibration-standard custody and expiry gate station.
//
// Research assumptions captured in geometry:
// - Purchased traceable standards need physically distinct released, quarantine,
//   and expired lanes so automation cannot silently present the wrong lot.
// - pH, dissolved oxygen, conductivity, TEER, and imaging standards benefit from
//   temperature-controlled pockets or repeatable reader nests near certificate
//   and lot-scan lands.
// - As-found and as-left sample nests must be kept paired but separated, with
//   tamper-evident seal pads and clean/used segregation visible at handoff.
//
// This is packaging/interface CAD for custody control. It does not define
// calibration limits, expiry rules, standard operating procedures, or claims of
// metrological compliance.

const OUTPUTS: [&str; 14] = [
    "output/closed_calibration_standard_custody_gate_base_tray.stl",
    "output/closed_calibration_standard_custody_gate_lane_gate_bridge.stl",
    "output/closed_calibration_standard_custody_gate_released_lane_caddy.stl",
    "output/closed_calibration_standard_custody_gate_quarantine_lane_caddy.stl",
    "output/closed_calibration_standard_custody_gate_expired_lane_caddy.stl",
    "output/closed_calibration_standard_custody_gate_temperature_pocket_block.stl",
    "output/closed_calibration_standard_custody_gate_reader_dock_panel.stl",
    "output/closed_calibration_standard_custody_gate_barcode_rfid_certificate_lands.stl",
    "output/closed_calibration_standard_custody_gate_as_found_as_left_nests.stl",
    "output/closed_calibration_standard_custody_gate_tamper_seal_pad_array.stl",
    "output/closed_calibration_standard_custody_gate_clean_used_segregation_tray.stl",
    "output/closed_calibration_standard_custody_gate_handoff_datum_tray.stl",
    "output/closed_calibration_standard_custody_gate_leak_tray_keepout_gauge.stl",
    "output/closed_calibration_standard_custody_gate_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 13] = [
    "released_lane",
    "quarantine_lane",
    "expired_lane",
    "temperature_controlled_ph_do_conductivity_teer_imaging_pockets",
    "reader_docks",
    "barcode_rfid_certificate_lands",
    "as_found_sample_nests",
    "as_left_sample_nests",
    "tamper_evident_seal_pads",
    "clean_used_segregation",
    "handoff_tray_datum",
    "leak_tray",
    "robot_service_keepouts",
];

const STATION_X: f64 = 980.0;
const STATION_Y: f64 = 660.0;
const BASE_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const SOCKET_DEPTH: f64 = 6.0;

const LANE_CADDY_X: f64 = 250.0;
const LANE_CADDY_Y: f64 = 126.0;
const LANE_CADDY_Z: f64 = 44.0;
const LANE_STANDARD_SLOTS: usize = 6;
const LANE_SLOT_PITCH: f64 = 34.0;
const RELEASED_POS: (f64, f64) = (-315.0, 205.0);
const QUARANTINE_POS: (f64, f64) = (0.0, 205.0);
const EXPIRED_POS: (f64, f64) = (315.0, 205.0);

const GATE_BRIDGE_X: f64 = 920.0;
const GATE_BRIDGE_Y: f64 = 54.0;
const GATE_BRIDGE_Z: f64 = 108.0;
const GATE_POS: (f64, f64) = (0.0, 108.0);
const STATUS_SHUTTER_COUNT: usize = 3;

const TEMP_BLOCK_X: f64 = 420.0;
const TEMP_BLOCK_Y: f64 = 158.0;
const TEMP_BLOCK_Z: f64 = 58.0;
const TEMP_POCKET_COUNT: usize = 5;
const TEMP_POCKET_PITCH: f64 = 72.0;
const TEMP_POS: (f64, f64) = (-240.0, 18.0);

const READER_PANEL_X: f64 = 330.0;
const READER_PANEL_Y: f64 = 146.0;
const READER_PANEL_Z: f64 = 52.0;
const READER_DOCK_COUNT: usize = 5;
const READER_DOCK_PITCH: f64 = 58.0;
const READER_POS: (f64, f64) = (270.0, 18.0);

const TRACE_PANEL_X: f64 = 430.0;
const TRACE_PANEL_Y: f64 = 116.0;
const TRACE_PANEL_Z: f64 = 12.0;
const BARCODE_LANDS: usize = 8;
const RFID_LANDS: usize = 4;
const CERTIFICATE_LANDS: usize = 3;
const TRACE_POS: (f64, f64) = (-245.0, -154.0);

const SAMPLE_NEST_X: f64 = 300.0;
const SAMPLE_NEST_Y: f64 = 126.0;
const SAMPLE_NEST_Z: f64 = 36.0;
const SAMPLE_PAIR_COUNT: usize = 4;
const SAMPLE_POS: (f64, f64) = (250.0, -154.0);

const SEAL_PAD_X: f64 = 200.0;
const SEAL_PAD_Y: f64 = 88.0;
const SEAL_PAD_Z: f64 = 16.0;
const SEAL_PAD_COUNT: usize = 8;
const SEAL_POS: (f64, f64) = (-360.0, -256.0);

const SEG_TRAY_X: f64 = 240.0;
const SEG_TRAY_Y: f64 = 94.0;
const SEG_TRAY_Z: f64 = 38.0;
const SEG_STANDARD_WELLS: usize = 6;
const SEG_POS: (f64, f64) = (-100.0, -256.0);

const HANDOFF_TRAY_X: f64 = 210.0;
const HANDOFF_TRAY_Y: f64 = 94.0;
const HANDOFF_TRAY_Z: f64 = 30.0;
const HANDOFF_DATUM_PINS: usize = 3;
const HANDOFF_POS: (f64, f64) = (150.0, -256.0);

const LEAK_GAUGE_X: f64 = 180.0;
const LEAK_GAUGE_Y: f64 = 94.0;
const LEAK_GAUGE_Z: f64 = 24.0;
const KEEP_OUT_ZONE_COUNT: usize = 4;
const LEAK_POS: (f64, f64) = (370.0, -256.0);

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_tray();
    export(OUTPUTS[0], &base);

    let bridge = lane_gate_bridge();
    export(OUTPUTS[1], &bridge);

    let released = lane_caddy("released", "green_released_lane");
    export(OUTPUTS[2], &released);

    let quarantine = lane_caddy("quarantine", "amber_quarantine_lane");
    export(OUTPUTS[3], &quarantine);

    let expired = lane_caddy("expired", "red_expired_lane");
    export(OUTPUTS[4], &expired);

    let temp = temperature_pocket_block();
    export(OUTPUTS[5], &temp);

    let readers = reader_dock_panel();
    export(OUTPUTS[6], &readers);

    let trace = barcode_rfid_certificate_lands();
    export(OUTPUTS[7], &trace);

    let samples = as_found_as_left_nests();
    export(OUTPUTS[8], &samples);

    let seals = tamper_seal_pad_array();
    export(OUTPUTS[9], &seals);

    let segregation = clean_used_segregation_tray();
    export(OUTPUTS[10], &segregation);

    let handoff = handoff_datum_tray();
    export(OUTPUTS[11], &handoff);

    let leak = leak_tray_keepout_gauge();
    export(OUTPUTS[12], &leak);

    let assembly = base
        + bridge.translate(GATE_POS.0, GATE_POS.1, insert_z(GATE_BRIDGE_Z))
        + released.translate(RELEASED_POS.0, RELEASED_POS.1, insert_z(LANE_CADDY_Z))
        + quarantine.translate(QUARANTINE_POS.0, QUARANTINE_POS.1, insert_z(LANE_CADDY_Z))
        + expired.translate(EXPIRED_POS.0, EXPIRED_POS.1, insert_z(LANE_CADDY_Z))
        + temp.translate(TEMP_POS.0, TEMP_POS.1, insert_z(TEMP_BLOCK_Z))
        + readers.translate(READER_POS.0, READER_POS.1, insert_z(READER_PANEL_Z))
        + trace.translate(TRACE_POS.0, TRACE_POS.1, insert_z(TRACE_PANEL_Z))
        + samples.translate(SAMPLE_POS.0, SAMPLE_POS.1, insert_z(SAMPLE_NEST_Z))
        + seals.translate(SEAL_POS.0, SEAL_POS.1, insert_z(SEAL_PAD_Z))
        + segregation.translate(SEG_POS.0, SEG_POS.1, insert_z(SEG_TRAY_Z))
        + handoff.translate(HANDOFF_POS.0, HANDOFF_POS.1, insert_z(HANDOFF_TRAY_Z))
        + leak.translate(LEAK_POS.0, LEAK_POS.1, insert_z(LEAK_GAUGE_Z));
    export(OUTPUTS[13], &assembly);

    println!();
    println!("Closed calibration-standard custody and expiry gate station:");
    println!("  Footprint:                    {STATION_X:.0}mm x {STATION_Y:.0}mm tray");
    println!(
        "  Custody lanes:                released, quarantine, and expired caddies with {LANE_STANDARD_SLOTS} standard slots each"
    );
    println!(
        "  Gate interlocks:              {STATUS_SHUTTER_COUNT} lane status shutters above released/quarantine/expired paths"
    );
    println!(
        "  Controlled pockets:           {TEMP_POCKET_COUNT} pockets for pH, DO, conductivity, TEER, and imaging standards"
    );
    println!(
        "  Traceability:                 {BARCODE_LANDS} barcode lands, {RFID_LANDS} RFID chip lands, {CERTIFICATE_LANDS} certificate lands"
    );
    println!(
        "  Handoff/QC:                   {SAMPLE_PAIR_COUNT} as-found/as-left sample pairs, {SEAL_PAD_COUNT} tamper-seal pads, {HANDOFF_DATUM_PINS} handoff datum pins, clean/used tray, leak tray, and {KEEP_OUT_ZONE_COUNT} keepout zones"
    );
    println!(
        "  Feature groups covered:       {}",
        REQUIRED_FEATURES.len()
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn insert_z(height: f64) -> f64 {
    BASE_Z / 2.0 + height / 2.0
}

fn assert_layout() {
    for (name, pos, width, depth) in insert_specs() {
        assert!(
            fits_on_station(pos, width, depth),
            "{name} exceeds station envelope"
        );
    }
}

fn insert_specs() -> [(&'static str, (f64, f64), f64, f64); 12] {
    [
        ("lane_gate_bridge", GATE_POS, GATE_BRIDGE_X, GATE_BRIDGE_Y),
        (
            "released_lane_caddy",
            RELEASED_POS,
            LANE_CADDY_X,
            LANE_CADDY_Y,
        ),
        (
            "quarantine_lane_caddy",
            QUARANTINE_POS,
            LANE_CADDY_X,
            LANE_CADDY_Y,
        ),
        (
            "expired_lane_caddy",
            EXPIRED_POS,
            LANE_CADDY_X,
            LANE_CADDY_Y,
        ),
        (
            "temperature_pocket_block",
            TEMP_POS,
            TEMP_BLOCK_X,
            TEMP_BLOCK_Y,
        ),
        (
            "reader_dock_panel",
            READER_POS,
            READER_PANEL_X,
            READER_PANEL_Y,
        ),
        (
            "barcode_rfid_certificate_lands",
            TRACE_POS,
            TRACE_PANEL_X,
            TRACE_PANEL_Y,
        ),
        (
            "as_found_as_left_nests",
            SAMPLE_POS,
            SAMPLE_NEST_X,
            SAMPLE_NEST_Y,
        ),
        ("tamper_seal_pad_array", SEAL_POS, SEAL_PAD_X, SEAL_PAD_Y),
        (
            "clean_used_segregation_tray",
            SEG_POS,
            SEG_TRAY_X,
            SEG_TRAY_Y,
        ),
        (
            "handoff_datum_tray",
            HANDOFF_POS,
            HANDOFF_TRAY_X,
            HANDOFF_TRAY_Y,
        ),
        (
            "leak_tray_keepout_gauge",
            LEAK_POS,
            LEAK_GAUGE_X,
            LEAK_GAUGE_Y,
        ),
    ]
}

fn fits_on_station(pos: (f64, f64), width: f64, depth: f64) -> bool {
    pos.0.abs() + width / 2.0 <= STATION_X / 2.0 - RIM_W - 8.0
        && pos.1.abs() + depth / 2.0 <= STATION_Y / 2.0 - RIM_W - 8.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn base_tray() -> Part {
    let deck = centered_cube(
        "closed_calibration_standard_custody_gate_base_floor",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let washdown_recess = centered_cube(
        "closed_calibration_standard_custody_gate_washdown_recess",
        STATION_X - 104.0,
        STATION_Y - 102.0,
        7.0,
    )
    .translate(0.0, -8.0, BASE_Z / 2.0 - 3.5);
    let front_drain = centered_cylinder(
        "closed_calibration_standard_custody_gate_front_drain",
        7.0 / 2.0,
        42.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 68.0, -STATION_Y / 2.0 - 2.0, -1.0);

    deck - washdown_recess - front_drain - insert_sockets() - mount_slots() - datum_pin_holes()
        + perimeter_rims()
        + lane_dividers()
        + rear_certificate_bulkhead()
        + robot_datum_targets()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("closed_calibration_standard_custody_gate_insert_sockets");
    for (name, pos, width, depth) in insert_specs() {
        sockets = sockets
            + centered_cube(
                format!("closed_calibration_standard_custody_gate_{name}_socket"),
                width + 7.0,
                depth + 7.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(pos.0, pos.1, BASE_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.2);
    }
    sockets
}

fn mount_slots() -> Part {
    let mut slots = Part::empty("closed_calibration_standard_custody_gate_mount_slots");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 48.0), -(STATION_Y / 2.0 - 42.0)),
        (STATION_X / 2.0 - 48.0, -(STATION_Y / 2.0 - 42.0)),
        (-(STATION_X / 2.0 - 48.0), STATION_Y / 2.0 - 42.0),
        (STATION_X / 2.0 - 48.0, STATION_Y / 2.0 - 42.0),
        (0.0, STATION_Y / 2.0 - 42.0),
        (0.0, -(STATION_Y / 2.0 - 42.0)),
    ]
    .iter()
    .enumerate()
    {
        slots = slots
            + centered_cylinder(
                format!("closed_calibration_standard_custody_gate_m6_clearance_{i}"),
                6.6 / 2.0,
                BASE_Z + 4.0,
                24,
            )
            .translate(*x, *y, 0.0)
            + centered_cube(
                format!("closed_calibration_standard_custody_gate_mount_slot_relief_{i}"),
                24.0,
                6.8,
                BASE_Z + 4.0,
            )
            .translate(*x, *y, 0.0);
    }
    slots
}

fn datum_pin_holes() -> Part {
    let mut holes = Part::empty("closed_calibration_standard_custody_gate_datum_pin_holes");
    for (i, (x, y)) in [
        (-430.0, 278.0),
        (430.0, 278.0),
        (-430.0, -278.0),
        (430.0, -278.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("closed_calibration_standard_custody_gate_datum_pin_hole_{i}"),
                6.0 / 2.0,
                BASE_Z + 4.0,
                28,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    let left = centered_cube(
        "closed_calibration_standard_custody_gate_left_rim",
        RIM_W,
        STATION_Y - 54.0,
        RIM_Z,
    )
    .translate(
        -(STATION_X / 2.0 - RIM_W / 2.0),
        4.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(
        "closed_calibration_standard_custody_gate_right_rim",
        RIM_W,
        STATION_Y - 54.0,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        4.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "closed_calibration_standard_custody_gate_rear_rim",
        STATION_X - 36.0,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let front_low_lip = centered_cube(
        "closed_calibration_standard_custody_gate_front_low_lip",
        STATION_X - 180.0,
        12.0,
        22.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 20.0, BASE_Z / 2.0 + 11.0);

    left + right + rear + front_low_lip
}

fn lane_dividers() -> Part {
    let rear_lane_rail = centered_cube(
        "closed_calibration_standard_custody_gate_lane_row_divider",
        STATION_X - 110.0,
        10.0,
        28.0,
    )
    .translate(0.0, 136.0, BASE_Z / 2.0 + 14.0);
    let center_row_rail = centered_cube(
        "closed_calibration_standard_custody_gate_standards_reader_divider",
        STATION_X - 136.0,
        10.0,
        24.0,
    )
    .translate(0.0, -78.0, BASE_Z / 2.0 + 12.0);
    let front_row_rail = centered_cube(
        "closed_calibration_standard_custody_gate_handoff_divider",
        STATION_X - 160.0,
        8.0,
        20.0,
    )
    .translate(0.0, -218.0, BASE_Z / 2.0 + 10.0);
    let left_lane_split = centered_cube(
        "closed_calibration_standard_custody_gate_released_quarantine_split",
        10.0,
        154.0,
        28.0,
    )
    .translate(-158.0, 205.0, BASE_Z / 2.0 + 14.0);
    let right_lane_split = centered_cube(
        "closed_calibration_standard_custody_gate_quarantine_expired_split",
        10.0,
        154.0,
        28.0,
    )
    .translate(158.0, 205.0, BASE_Z / 2.0 + 14.0);

    rear_lane_rail + center_row_rail + front_row_rail + left_lane_split + right_lane_split
}

fn rear_certificate_bulkhead() -> Part {
    let mut bulkhead =
        Part::empty("closed_calibration_standard_custody_gate_rear_certificate_bulkhead");
    for (i, x) in [-360.0, -180.0, 0.0, 180.0, 360.0].iter().enumerate() {
        let tab = centered_cube(
            format!("closed_calibration_standard_custody_gate_rear_cert_tab_{i}"),
            78.0,
            18.0,
            28.0,
        )
        .translate(*x, STATION_Y / 2.0 - 48.0, BASE_Z / 2.0 + 14.0);
        let slot = centered_cube(
            format!("closed_calibration_standard_custody_gate_rear_cert_slot_{i}"),
            52.0,
            22.0,
            5.0,
        )
        .translate(*x, STATION_Y / 2.0 - 48.0, BASE_Z / 2.0 + 16.0);
        bulkhead = bulkhead + (tab - slot);
    }
    bulkhead
}

fn robot_datum_targets() -> Part {
    let mut targets = Part::empty("closed_calibration_standard_custody_gate_robot_datum_targets");
    for (i, (x, y)) in [(-438.0, 284.0), (438.0, 284.0), (-438.0, -284.0)]
        .iter()
        .enumerate()
    {
        targets = targets
            + fiducial_disc(&format!(
                "closed_calibration_standard_custody_gate_robot_fiducial_{i}"
            ))
            .translate(*x, *y, BASE_Z / 2.0 + 2.5);
    }
    targets
}

fn lane_gate_bridge() -> Part {
    let beam = centered_cube(
        "closed_calibration_standard_custody_gate_lane_gate_bridge_beam",
        GATE_BRIDGE_X,
        GATE_BRIDGE_Y,
        GATE_BRIDGE_Z,
    );
    let front_window = centered_cube(
        "closed_calibration_standard_custody_gate_front_gate_window",
        GATE_BRIDGE_X - 96.0,
        GATE_BRIDGE_Y + 8.0,
        GATE_BRIDGE_Z - 24.0,
    )
    .translate(0.0, 0.0, 6.0);
    let mut shutters = Part::empty("closed_calibration_standard_custody_gate_status_shutters");
    for (i, x) in [-315.0, 0.0, 315.0].iter().enumerate() {
        shutters = shutters
            + centered_cube(
                format!("closed_calibration_standard_custody_gate_lane_status_shutter_{i}"),
                138.0,
                12.0,
                64.0,
            )
            .translate(*x, -GATE_BRIDGE_Y / 2.0 - 6.0, 4.0)
            - centered_cube(
                format!("closed_calibration_standard_custody_gate_lane_scan_window_{i}"),
                88.0,
                16.0,
                34.0,
            )
            .translate(*x, -GATE_BRIDGE_Y / 2.0 - 6.0, 6.0);
    }
    let mut interlock_slots =
        Part::empty("closed_calibration_standard_custody_gate_interlock_slots");
    for (i, x) in [-382.0, -248.0, -68.0, 68.0, 248.0, 382.0]
        .iter()
        .enumerate()
    {
        interlock_slots = interlock_slots
            + centered_cylinder(
                format!("closed_calibration_standard_custody_gate_interlock_pin_slot_{i}"),
                6.0 / 2.0,
                GATE_BRIDGE_Y + 8.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 0.0, -GATE_BRIDGE_Z / 2.0 + 18.0);
    }

    beam - front_window - interlock_slots + shutters
}

fn lane_caddy(status: &str, body_name: &str) -> Part {
    let body = centered_cube(
        format!("closed_calibration_standard_custody_gate_{body_name}_body"),
        LANE_CADDY_X,
        LANE_CADDY_Y,
        LANE_CADDY_Z,
    );
    let rear_fence = centered_cube(
        format!("closed_calibration_standard_custody_gate_{status}_rear_fence"),
        LANE_CADDY_X,
        12.0,
        LANE_CADDY_Z + 32.0,
    )
    .translate(0.0, LANE_CADDY_Y / 2.0 - 6.0, 16.0);
    let status_key = centered_cube(
        format!("closed_calibration_standard_custody_gate_{status}_status_key"),
        62.0,
        14.0,
        14.0,
    )
    .translate(0.0, -LANE_CADDY_Y / 2.0 + 11.0, LANE_CADDY_Z / 2.0 + 7.0);

    let mut pockets = Part::empty(format!(
        "closed_calibration_standard_custody_gate_{status}_standard_pockets"
    ));
    for i in 0..LANE_STANDARD_SLOTS {
        let x = centered_index(i, LANE_STANDARD_SLOTS, LANE_SLOT_PITCH);
        pockets = pockets
            + centered_cylinder(
                format!("closed_calibration_standard_custody_gate_{status}_vial_well_{i}"),
                13.0 / 2.0,
                LANE_CADDY_Z + 8.0,
                30,
            )
            .translate(x, -18.0, 8.0)
            + centered_cube(
                format!("closed_calibration_standard_custody_gate_{status}_box_socket_{i}"),
                22.0,
                28.0,
                20.0,
            )
            .translate(x, 28.0, LANE_CADDY_Z / 2.0 - 10.0);
    }

    body + rear_fence + status_key - pockets + lane_grip_fiducials(status)
}

fn temperature_pocket_block() -> Part {
    let body = centered_cube(
        "closed_calibration_standard_custody_gate_temperature_block_body",
        TEMP_BLOCK_X,
        TEMP_BLOCK_Y,
        TEMP_BLOCK_Z,
    );
    let thermal_plate = centered_cube(
        "closed_calibration_standard_custody_gate_peltier_cold_plate_land",
        TEMP_BLOCK_X - 34.0,
        TEMP_BLOCK_Y - 34.0,
        8.0,
    )
    .translate(0.0, 0.0, -TEMP_BLOCK_Z / 2.0 + 5.0);
    let service_channel = centered_cube(
        "closed_calibration_standard_custody_gate_temperature_cable_channel",
        TEMP_BLOCK_X - 56.0,
        12.0,
        14.0,
    )
    .translate(0.0, TEMP_BLOCK_Y / 2.0 - 18.0, -8.0);

    let mut pockets =
        Part::empty("closed_calibration_standard_custody_gate_temperature_standard_pockets");
    for (i, label) in ["ph", "do", "conductivity", "teer", "imaging"]
        .iter()
        .enumerate()
    {
        let x = centered_index(i, TEMP_POCKET_COUNT, TEMP_POCKET_PITCH);
        pockets = pockets
            + centered_cylinder(
                format!("closed_calibration_standard_custody_gate_{label}_round_standard_well"),
                20.0 / 2.0,
                TEMP_BLOCK_Z + 10.0,
                36,
            )
            .translate(x, -22.0, 10.0)
            + centered_cube(
                format!("closed_calibration_standard_custody_gate_{label}_cartridge_pocket"),
                38.0,
                42.0,
                24.0,
            )
            .translate(x, 28.0, TEMP_BLOCK_Z / 2.0 - 12.0)
            + centered_cube(
                format!("closed_calibration_standard_custody_gate_{label}_expiry_label_land"),
                42.0,
                12.0,
                3.0,
            )
            .translate(x, -TEMP_BLOCK_Y / 2.0 + 12.0, TEMP_BLOCK_Z / 2.0 - 1.5);
    }

    body - thermal_plate - service_channel - pockets + block_latch_tabs("temperature_pocket")
}

fn reader_dock_panel() -> Part {
    let body = centered_cube(
        "closed_calibration_standard_custody_gate_reader_panel_body",
        READER_PANEL_X,
        READER_PANEL_Y,
        READER_PANEL_Z,
    );
    let rear_backer = centered_cube(
        "closed_calibration_standard_custody_gate_reader_panel_rear_backer",
        READER_PANEL_X,
        14.0,
        READER_PANEL_Z + 34.0,
    )
    .translate(0.0, READER_PANEL_Y / 2.0 - 7.0, 17.0);
    let cable_trough = centered_cube(
        "closed_calibration_standard_custody_gate_reader_panel_cable_trough",
        READER_PANEL_X - 42.0,
        12.0,
        18.0,
    )
    .translate(0.0, READER_PANEL_Y / 2.0 - 25.0, -4.0);

    let mut docks = Part::empty("closed_calibration_standard_custody_gate_reader_docks");
    for i in 0..READER_DOCK_COUNT {
        let x = centered_index(i, READER_DOCK_COUNT, READER_DOCK_PITCH);
        docks = docks
            + centered_cube(
                format!("closed_calibration_standard_custody_gate_reader_sled_pocket_{i}"),
                38.0,
                72.0,
                26.0,
            )
            .translate(x, -10.0, READER_PANEL_Z / 2.0 - 13.0)
            + centered_cylinder(
                format!("closed_calibration_standard_custody_gate_reader_probe_bore_{i}"),
                7.0 / 2.0,
                READER_PANEL_Y + 8.0,
                22,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, 6.0);
    }

    body + rear_backer - cable_trough - docks + block_latch_tabs("reader_panel")
}

fn barcode_rfid_certificate_lands() -> Part {
    let panel = centered_cube(
        "closed_calibration_standard_custody_gate_traceability_panel",
        TRACE_PANEL_X,
        TRACE_PANEL_Y,
        TRACE_PANEL_Z,
    );
    let mut lands = Part::empty("closed_calibration_standard_custody_gate_traceability_lands");
    for i in 0..BARCODE_LANDS {
        let x = centered_index(i, BARCODE_LANDS, 48.0);
        lands = lands
            + centered_cube(
                format!("closed_calibration_standard_custody_gate_barcode_land_{i}"),
                36.0,
                16.0,
                3.0,
            )
            .translate(x, -36.0, TRACE_PANEL_Z / 2.0 + 1.5);
    }
    for i in 0..RFID_LANDS {
        let x = centered_index(i, RFID_LANDS, 74.0);
        lands = lands
            + centered_cube(
                format!("closed_calibration_standard_custody_gate_rfid_chip_land_{i}"),
                48.0,
                30.0,
                3.0,
            )
            .translate(x, 0.0, TRACE_PANEL_Z / 2.0 + 1.5);
    }
    for i in 0..CERTIFICATE_LANDS {
        let x = centered_index(i, CERTIFICATE_LANDS, 126.0);
        lands = lands
            + centered_cube(
                format!("closed_calibration_standard_custody_gate_certificate_card_land_{i}"),
                94.0,
                28.0,
                3.0,
            )
            .translate(x, 38.0, TRACE_PANEL_Z / 2.0 + 1.5);
    }
    panel + lands
}

fn as_found_as_left_nests() -> Part {
    let body = centered_cube(
        "closed_calibration_standard_custody_gate_as_found_as_left_body",
        SAMPLE_NEST_X,
        SAMPLE_NEST_Y,
        SAMPLE_NEST_Z,
    );
    let center_wall = centered_cube(
        "closed_calibration_standard_custody_gate_as_found_as_left_center_wall",
        SAMPLE_NEST_X - 22.0,
        8.0,
        SAMPLE_NEST_Z + 18.0,
    )
    .translate(0.0, 0.0, 9.0);

    let mut nests = Part::empty("closed_calibration_standard_custody_gate_sample_nests");
    for i in 0..SAMPLE_PAIR_COUNT {
        let x = centered_index(i, SAMPLE_PAIR_COUNT, 62.0);
        nests = nests
            + centered_cylinder(
                format!("closed_calibration_standard_custody_gate_as_found_nest_{i}"),
                17.0 / 2.0,
                SAMPLE_NEST_Z + 8.0,
                32,
            )
            .translate(x, -32.0, 6.0)
            + centered_cylinder(
                format!("closed_calibration_standard_custody_gate_as_left_nest_{i}"),
                17.0 / 2.0,
                SAMPLE_NEST_Z + 8.0,
                32,
            )
            .translate(x, 32.0, 6.0)
            + centered_cube(
                format!("closed_calibration_standard_custody_gate_sample_pair_label_land_{i}"),
                44.0,
                10.0,
                3.0,
            )
            .translate(x, 0.0, SAMPLE_NEST_Z / 2.0 + 1.5);
    }

    body + center_wall - nests + block_latch_tabs("sample_nest")
}

fn tamper_seal_pad_array() -> Part {
    let pad = centered_cube(
        "closed_calibration_standard_custody_gate_tamper_seal_pad_base",
        SEAL_PAD_X,
        SEAL_PAD_Y,
        SEAL_PAD_Z,
    );
    let mut features = Part::empty("closed_calibration_standard_custody_gate_tamper_seal_features");
    for i in 0..SEAL_PAD_COUNT {
        let x = centered_index(i % 4, 4, 42.0);
        let y = if i < 4 { -21.0 } else { 21.0 };
        features = features
            + centered_cube(
                format!("closed_calibration_standard_custody_gate_seal_lanyard_slot_{i}"),
                30.0,
                7.0,
                SEAL_PAD_Z + 4.0,
            )
            .translate(x, y, 0.0)
            + centered_cylinder(
                format!("closed_calibration_standard_custody_gate_seal_wire_bore_{i}"),
                2.4 / 2.0,
                40.0,
                16,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, y, 3.0);
    }
    pad - features
}

fn clean_used_segregation_tray() -> Part {
    let tray = centered_cube(
        "closed_calibration_standard_custody_gate_clean_used_tray_body",
        SEG_TRAY_X,
        SEG_TRAY_Y,
        SEG_TRAY_Z,
    );
    let divider = centered_cube(
        "closed_calibration_standard_custody_gate_clean_used_divider",
        8.0,
        SEG_TRAY_Y,
        SEG_TRAY_Z + 16.0,
    )
    .translate(0.0, 0.0, 8.0);
    let mut wells = Part::empty("closed_calibration_standard_custody_gate_clean_used_wells");
    for i in 0..SEG_STANDARD_WELLS {
        let y = centered_index(i % 3, 3, 26.0);
        let x = if i < 3 { -58.0 } else { 58.0 };
        wells = wells
            + centered_cylinder(
                format!("closed_calibration_standard_custody_gate_clean_used_well_{i}"),
                15.0 / 2.0,
                SEG_TRAY_Z + 8.0,
                28,
            )
            .translate(x, y, 6.0);
    }
    let spill_channel = centered_cube(
        "closed_calibration_standard_custody_gate_clean_used_spill_channel",
        SEG_TRAY_X - 34.0,
        7.0,
        7.0,
    )
    .translate(0.0, -SEG_TRAY_Y / 2.0 + 12.0, SEG_TRAY_Z / 2.0 - 3.5);

    tray + divider - wells - spill_channel
}

fn handoff_datum_tray() -> Part {
    let tray = centered_cube(
        "closed_calibration_standard_custody_gate_handoff_tray_body",
        HANDOFF_TRAY_X,
        HANDOFF_TRAY_Y,
        HANDOFF_TRAY_Z,
    );
    let nest = centered_cube(
        "closed_calibration_standard_custody_gate_handoff_carrier_nest",
        HANDOFF_TRAY_X - 36.0,
        HANDOFF_TRAY_Y - 34.0,
        16.0,
    )
    .translate(0.0, 0.0, HANDOFF_TRAY_Z / 2.0 - 8.0);
    let robot_pull = centered_cube(
        "closed_calibration_standard_custody_gate_handoff_robot_pull_slot",
        64.0,
        10.0,
        12.0,
    )
    .translate(0.0, -HANDOFF_TRAY_Y / 2.0 + 9.0, 2.0);
    let mut datums = Part::empty("closed_calibration_standard_custody_gate_handoff_datum_pins");
    for (i, (x, y)) in [(-70.0, 30.0), (70.0, 30.0), (-70.0, -30.0)]
        .iter()
        .enumerate()
    {
        datums = datums
            + centered_cylinder(
                format!("closed_calibration_standard_custody_gate_handoff_datum_socket_{i}"),
                5.0 / 2.0,
                HANDOFF_TRAY_Z + 6.0,
                24,
            )
            .translate(*x, *y, 0.0);
    }

    tray - nest - robot_pull - datums + block_latch_tabs("handoff_tray")
}

fn leak_tray_keepout_gauge() -> Part {
    let tray = centered_cube(
        "closed_calibration_standard_custody_gate_leak_tray_body",
        LEAK_GAUGE_X,
        LEAK_GAUGE_Y,
        LEAK_GAUGE_Z,
    );
    let sump = centered_cube(
        "closed_calibration_standard_custody_gate_leak_tray_sump",
        LEAK_GAUGE_X - 30.0,
        LEAK_GAUGE_Y - 30.0,
        10.0,
    )
    .translate(0.0, 0.0, LEAK_GAUGE_Z / 2.0 - 5.0);
    let drain = centered_cylinder(
        "closed_calibration_standard_custody_gate_leak_tray_drain",
        5.0 / 2.0,
        28.0,
        20,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(LEAK_GAUGE_X / 2.0 - 26.0, -LEAK_GAUGE_Y / 2.0 + 8.0, 0.0);

    let mut keepouts = Part::empty("closed_calibration_standard_custody_gate_keepout_gauge_posts");
    for (i, (x, y)) in [
        (-LEAK_GAUGE_X / 2.0 + 20.0, -LEAK_GAUGE_Y / 2.0 + 20.0),
        (LEAK_GAUGE_X / 2.0 - 20.0, -LEAK_GAUGE_Y / 2.0 + 20.0),
        (-LEAK_GAUGE_X / 2.0 + 20.0, LEAK_GAUGE_Y / 2.0 - 20.0),
        (LEAK_GAUGE_X / 2.0 - 20.0, LEAK_GAUGE_Y / 2.0 - 20.0),
    ]
    .iter()
    .enumerate()
    {
        keepouts = keepouts
            + centered_cylinder(
                format!("closed_calibration_standard_custody_gate_robot_keepout_post_{i}"),
                6.0,
                58.0,
                24,
            )
            .translate(*x, *y, LEAK_GAUGE_Z / 2.0 + 29.0);
    }

    tray - sump - drain + keepouts
}

fn lane_grip_fiducials(prefix: &str) -> Part {
    let mut fiducials = Part::empty(format!(
        "closed_calibration_standard_custody_gate_{prefix}_grip_fiducials"
    ));
    for (i, x) in [-LANE_CADDY_X / 2.0 + 28.0, LANE_CADDY_X / 2.0 - 28.0]
        .iter()
        .enumerate()
    {
        fiducials = fiducials
            + fiducial_disc(&format!(
                "closed_calibration_standard_custody_gate_{prefix}_fiducial_{i}"
            ))
            .translate(*x, -LANE_CADDY_Y / 2.0 + 20.0, LANE_CADDY_Z / 2.0 + 2.5);
    }
    fiducials
}

fn block_latch_tabs(prefix: &str) -> Part {
    let left = centered_cube(
        format!("closed_calibration_standard_custody_gate_{prefix}_left_latch_tab"),
        24.0,
        10.0,
        12.0,
    )
    .translate(-34.0, 0.0, 6.0);
    let right = centered_cube(
        format!("closed_calibration_standard_custody_gate_{prefix}_right_latch_tab"),
        24.0,
        10.0,
        12.0,
    )
    .translate(34.0, 0.0, 6.0);
    left + right
}

fn fiducial_disc(name: &str) -> Part {
    centered_cylinder(name, 12.0, 3.0, 32)
        - centered_cylinder(format!("{name}_center"), 3.0, 4.0, 24)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_count_matches_exported_components() {
        assert_eq!(OUTPUTS.len(), 14);
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
        assert!(OUTPUTS
            .iter()
            .any(|path| path.ends_with("closed_calibration_standard_custody_gate_assembly.stl")));
    }

    #[test]
    fn required_features_cover_custody_and_calibration_needs() {
        for feature in [
            "released_lane",
            "quarantine_lane",
            "expired_lane",
            "barcode_rfid_certificate_lands",
            "as_found_sample_nests",
            "as_left_sample_nests",
            "tamper_evident_seal_pads",
            "clean_used_segregation",
            "handoff_tray_datum",
            "leak_tray",
            "robot_service_keepouts",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
        assert_eq!(TEMP_POCKET_COUNT, 5);
        assert_eq!(READER_DOCK_COUNT, 5);
    }

    #[test]
    fn all_inserts_fit_station_envelope() {
        assert_layout();
        assert!(insert_specs()
            .iter()
            .all(|(_, pos, width, depth)| fits_on_station(*pos, *width, *depth)));
    }

    #[test]
    fn custody_capacity_is_balanced_across_lanes() {
        let total_lane_slots = LANE_STANDARD_SLOTS * 3;
        assert_eq!(total_lane_slots, 18);
        assert!(LANE_SLOT_PITCH * (LANE_STANDARD_SLOTS as f64 - 1.0) < LANE_CADDY_X - 50.0);
        assert_eq!(STATUS_SHUTTER_COUNT, 3);
    }

    #[test]
    fn traceability_and_sample_counts_are_sufficient() {
        assert!(BARCODE_LANDS >= LANE_STANDARD_SLOTS);
        assert!(RFID_LANDS >= 3);
        assert!(CERTIFICATE_LANDS >= 3);
        assert_eq!(SAMPLE_PAIR_COUNT * 2, 8);
        assert_eq!(SEAL_PAD_COUNT, 8);
        assert_eq!(HANDOFF_DATUM_PINS, 3);
    }
}
