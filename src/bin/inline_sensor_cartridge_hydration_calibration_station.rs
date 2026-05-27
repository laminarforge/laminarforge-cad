use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Inline sensor cartridge hydration, calibration, and release station.
//
// Intent:
// - Precondition closed-path pH, DO, optical, TEER, pressure, flow, and bubble
//   sensor cartridges before they enter culture modules.
// - Keep hydration and calibration standards contained, traceable, and
//   physically separated from released cartridges.
// - Provide mechanical envelopes for wet hydration, reference standards,
//   dark/temperature equilibration, pogo/electrical checks, status lanes,
//   waste wells, sample-loop handoff, and robot/service keepouts.
//
// This is packaging and architecture CAD only. Sensor chemistry, certified
// standard values, release criteria, and lot traceability rules are validation
// system decisions.

const OUTPUTS: &[&str] = &[
    "output/inline_sensor_cartridge_hydration_calibration_station_base_leak_tray.stl",
    "output/inline_sensor_cartridge_hydration_calibration_station_clean_cartridge_rack.stl",
    "output/inline_sensor_cartridge_hydration_calibration_station_hydration_buffer_wells.stl",
    "output/inline_sensor_cartridge_hydration_calibration_station_calibration_standard_pockets.stl",
    "output/inline_sensor_cartridge_hydration_calibration_station_dark_temperature_equilibration_cover.stl",
    "output/inline_sensor_cartridge_hydration_calibration_station_electrical_pogo_check_fixture.stl",
    "output/inline_sensor_cartridge_hydration_calibration_station_wet_dry_release_lanes.stl",
    "output/inline_sensor_cartridge_hydration_calibration_station_barcode_certificate_lands.stl",
    "output/inline_sensor_cartridge_hydration_calibration_station_spent_standard_waste_wells.stl",
    "output/inline_sensor_cartridge_hydration_calibration_station_sample_loop_handoff_ports.stl",
    "output/inline_sensor_cartridge_hydration_calibration_station_clean_used_segregation.stl",
    "output/inline_sensor_cartridge_hydration_calibration_station_robot_service_keepouts.stl",
    "output/inline_sensor_cartridge_hydration_calibration_station_assembly.stl",
];

const REQUIRED_FEATURES: &[&str] = &[
    "clean_sensor_cartridge_rack",
    "hydration_buffer_wells",
    "calibration_standard_pockets",
    "dark_temperature_equilibration_cover",
    "electrical_pogo_check_fixture",
    "wet_dry_status_lanes",
    "barcode_calibration_certificate_lands",
    "spent_standard_waste_wells",
    "sample_loop_handoff_ports",
    "clean_used_segregation",
    "leak_tray",
    "robot_service_keepouts",
];

const STATION_X: f64 = 1280.0;
const STATION_Y: f64 = 820.0;
const BASE_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const SOCKET_DEPTH: f64 = 5.0;

const SENSOR_TYPES: usize = 7;
const CARTRIDGE_COUNT: usize = 24;
const CARTRIDGE_ROWS: usize = 4;
const CARTRIDGE_COLS: usize = 6;
const CARTRIDGE_SLOT_X: f64 = 44.0;
const CARTRIDGE_SLOT_Y: f64 = 24.0;
const CARTRIDGE_SLOT_Z: f64 = 36.0;
const CARTRIDGE_PITCH_X: f64 = 52.0;
const CARTRIDGE_PITCH_Y: f64 = 42.0;

const RACK_X: f64 = 430.0;
const RACK_Y: f64 = 226.0;
const RACK_Z: f64 = 58.0;
const RACK_POS: (f64, f64) = (-390.0, 230.0);

const HYDRATION_X: f64 = 360.0;
const HYDRATION_Y: f64 = 182.0;
const HYDRATION_Z: f64 = 54.0;
const HYDRATION_WELL_COUNT: usize = 24;
const HYDRATION_POS: (f64, f64) = (-24.0, 236.0);

const CAL_BANK_X: f64 = 420.0;
const CAL_BANK_Y: f64 = 196.0;
const CAL_BANK_Z: f64 = 60.0;
const CAL_STANDARD_COUNT: usize = 12;
const CAL_POS: (f64, f64) = (404.0, 232.0);

const DARK_COVER_X: f64 = 846.0;
const DARK_COVER_Y: f64 = 284.0;
const DARK_COVER_Z: f64 = 200.0;
const DARK_COVER_POS: (f64, f64) = (190.0, 230.0);

const POGO_X: f64 = 492.0;
const POGO_Y: f64 = 176.0;
const POGO_Z: f64 = 76.0;
const POGO_CHECK_POSITIONS: usize = 8;
const POGO_PIN_LANES: usize = 7;
const POGO_POS: (f64, f64) = (-360.0, -28.0);

const STATUS_X: f64 = 390.0;
const STATUS_Y: f64 = 178.0;
const STATUS_Z: f64 = 44.0;
const STATUS_LANES: usize = 5;
const STATUS_SLOTS_PER_LANE: usize = 6;
const STATUS_POS: (f64, f64) = (82.0, -30.0);

const TRACE_X: f64 = 340.0;
const TRACE_Y: f64 = 150.0;
const TRACE_Z: f64 = 12.0;
const BARCODE_LANDS: usize = 12;
const CERTIFICATE_LANDS: usize = 6;
const TRACE_POS: (f64, f64) = (426.0, -38.0);

const WASTE_X: f64 = 342.0;
const WASTE_Y: f64 = 160.0;
const WASTE_Z: f64 = 54.0;
const SPENT_WASTE_WELLS: usize = 12;
const WASTE_POS: (f64, f64) = (-430.0, -254.0);

const HANDOFF_X: f64 = 392.0;
const HANDOFF_Y: f64 = 150.0;
const HANDOFF_Z: f64 = 48.0;
const SAMPLE_LOOP_PORTS: usize = 14;
const HANDOFF_POS: (f64, f64) = (-34.0, -258.0);

const SEGREGATION_X: f64 = 316.0;
const SEGREGATION_Y: f64 = 154.0;
const SEGREGATION_Z: f64 = 48.0;
const SEGREGATION_POS: (f64, f64) = (420.0, -256.0);

const LEAK_CHANNEL_COUNT: usize = 5;
const KEEP_OUT_ZONE_COUNT: usize = 5;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_leak_tray();
    export(OUTPUTS[0], &base);

    let rack = clean_cartridge_rack();
    export(OUTPUTS[1], &rack);

    let hydration = hydration_buffer_wells();
    export(OUTPUTS[2], &hydration);

    let calibration = calibration_standard_pockets();
    export(OUTPUTS[3], &calibration);

    let dark_cover = dark_temperature_equilibration_cover();
    export(OUTPUTS[4], &dark_cover);

    let pogo = electrical_pogo_check_fixture();
    export(OUTPUTS[5], &pogo);

    let status = wet_dry_release_lanes();
    export(OUTPUTS[6], &status);

    let traceability = barcode_certificate_lands();
    export(OUTPUTS[7], &traceability);

    let waste = spent_standard_waste_wells();
    export(OUTPUTS[8], &waste);

    let handoff = sample_loop_handoff_ports();
    export(OUTPUTS[9], &handoff);

    let segregation = clean_used_segregation();
    export(OUTPUTS[10], &segregation);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[11], &keepouts);

    let assembly = base
        + rack.translate(RACK_POS.0, RACK_POS.1, insert_z(RACK_Z))
        + hydration.translate(HYDRATION_POS.0, HYDRATION_POS.1, insert_z(HYDRATION_Z))
        + calibration.translate(CAL_POS.0, CAL_POS.1, insert_z(CAL_BANK_Z))
        + dark_cover.translate(DARK_COVER_POS.0, DARK_COVER_POS.1, insert_z(DARK_COVER_Z))
        + pogo.translate(POGO_POS.0, POGO_POS.1, insert_z(POGO_Z))
        + status.translate(STATUS_POS.0, STATUS_POS.1, insert_z(STATUS_Z))
        + traceability.translate(TRACE_POS.0, TRACE_POS.1, insert_z(TRACE_Z))
        + waste.translate(WASTE_POS.0, WASTE_POS.1, insert_z(WASTE_Z))
        + handoff.translate(HANDOFF_POS.0, HANDOFF_POS.1, insert_z(HANDOFF_Z))
        + segregation.translate(
            SEGREGATION_POS.0,
            SEGREGATION_POS.1,
            insert_z(SEGREGATION_Z),
        )
        + keepouts.translate(0.0, 0.0, BASE_Z / 2.0 + 3.0);
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Inline sensor cartridge hydration/calibration/release station:");
    println!("  Footprint:                 {STATION_X:.0}mm x {STATION_Y:.0}mm contained tray");
    println!(
        "  Sensor coverage:           {SENSOR_TYPES} cartridge families covering pH, DO, optical, TEER, pressure, flow, and bubble detection"
    );
    println!(
        "  Throughput:                {CARTRIDGE_COUNT} clean cartridge rack slots, {HYDRATION_WELL_COUNT} hydration wells, {CAL_STANDARD_COUNT} standard pockets"
    );
    println!(
        "  Electrical release checks: {POGO_CHECK_POSITIONS} cartridge nests with {POGO_PIN_LANES} pogo/check lanes"
    );
    println!(
        "  Traceability/segregation:  {BARCODE_LANDS} barcode lands, {CERTIFICATE_LANDS} certificate lands, {STATUS_LANES} wet/dry/release lanes, {SPENT_WASTE_WELLS} spent-standard wells"
    );
    println!(
        "  Closed-fluid interfaces:   {SAMPLE_LOOP_PORTS} sample-loop handoff ports, leak tray, clean/used barrier, and {KEEP_OUT_ZONE_COUNT} robot/service keepouts"
    );
    println!("  Required feature groups:   {}", REQUIRED_FEATURES.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn insert_z(height: f64) -> f64 {
    BASE_Z / 2.0 + height / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn assert_layout() {
    for (name, pos, width, depth) in insert_specs() {
        assert!(
            fits_on_station(pos, width, depth),
            "{name} exceeds station envelope"
        );
    }
    assert_eq!(CARTRIDGE_COUNT, CARTRIDGE_ROWS * CARTRIDGE_COLS);
    assert_eq!(HYDRATION_WELL_COUNT, CARTRIDGE_COUNT);
}

fn insert_specs() -> [(&'static str, (f64, f64), f64, f64); 9] {
    [
        ("clean_cartridge_rack", RACK_POS, RACK_X, RACK_Y),
        (
            "hydration_buffer_wells",
            HYDRATION_POS,
            HYDRATION_X,
            HYDRATION_Y,
        ),
        (
            "calibration_standard_pockets",
            CAL_POS,
            CAL_BANK_X,
            CAL_BANK_Y,
        ),
        ("electrical_pogo_check_fixture", POGO_POS, POGO_X, POGO_Y),
        ("wet_dry_release_lanes", STATUS_POS, STATUS_X, STATUS_Y),
        ("barcode_certificate_lands", TRACE_POS, TRACE_X, TRACE_Y),
        ("spent_standard_waste_wells", WASTE_POS, WASTE_X, WASTE_Y),
        (
            "sample_loop_handoff_ports",
            HANDOFF_POS,
            HANDOFF_X,
            HANDOFF_Y,
        ),
        (
            "clean_used_segregation",
            SEGREGATION_POS,
            SEGREGATION_X,
            SEGREGATION_Y,
        ),
    ]
}

fn fits_on_station(pos: (f64, f64), width: f64, depth: f64) -> bool {
    pos.0.abs() + width / 2.0 <= STATION_X / 2.0 - RIM_W - 8.0
        && pos.1.abs() + depth / 2.0 <= STATION_Y / 2.0 - RIM_W - 8.0
}

fn base_leak_tray() -> Part {
    let deck = centered_cube(
        "inline_sensor_cartridge_hydration_calibration_station_base_floor",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let washdown_recess = centered_cube(
        "inline_sensor_cartridge_hydration_calibration_station_washdown_recess",
        STATION_X - 116.0,
        STATION_Y - 114.0,
        7.0,
    )
    .translate(0.0, -6.0, BASE_Z / 2.0 - 3.5);
    let wet_zone_sump = centered_cube(
        "inline_sensor_cartridge_hydration_calibration_station_wet_zone_sump",
        890.0,
        210.0,
        8.0,
    )
    .translate(140.0, 214.0, BASE_Z / 2.0 - 4.0);
    let lower_zone_sump = centered_cube(
        "inline_sensor_cartridge_hydration_calibration_station_lower_release_sump",
        1120.0,
        118.0,
        8.0,
    )
    .translate(0.0, -252.0, BASE_Z / 2.0 - 4.0);
    let front_drain = centered_cylinder(
        "inline_sensor_cartridge_hydration_calibration_station_front_drain",
        9.0 / 2.0,
        40.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 86.0, -STATION_Y / 2.0 - 2.0, -1.0);

    deck - washdown_recess
        - wet_zone_sump
        - lower_zone_sump
        - front_drain
        - insert_sockets()
        - mounting_slots()
        - datum_pin_holes()
        + perimeter_rims()
        + zone_dividers()
        + rear_service_bulkheads()
        + leak_witness_channels()
        + robot_fiducial_targets()
}

fn insert_sockets() -> Part {
    let mut sockets =
        Part::empty("inline_sensor_cartridge_hydration_calibration_station_insert_sockets");
    for (name, pos, width, depth) in insert_specs() {
        sockets = sockets
            + centered_cube(
                format!("inline_sensor_cartridge_hydration_calibration_station_{name}_socket"),
                width + 8.0,
                depth + 8.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(pos.0, pos.1, BASE_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.2);
    }
    sockets
}

fn mounting_slots() -> Part {
    let mut slots =
        Part::empty("inline_sensor_cartridge_hydration_calibration_station_mounting_slots");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 54.0), -(STATION_Y / 2.0 - 48.0)),
        (STATION_X / 2.0 - 54.0, -(STATION_Y / 2.0 - 48.0)),
        (-(STATION_X / 2.0 - 54.0), STATION_Y / 2.0 - 48.0),
        (STATION_X / 2.0 - 54.0, STATION_Y / 2.0 - 48.0),
        (0.0, STATION_Y / 2.0 - 48.0),
        (0.0, -(STATION_Y / 2.0 - 48.0)),
        (-(STATION_X / 2.0 - 54.0), 0.0),
        (STATION_X / 2.0 - 54.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        slots = slots
            + centered_cylinder(
                format!("inline_sensor_cartridge_hydration_calibration_station_m6_clearance_{i}"),
                6.6 / 2.0,
                BASE_Z + 4.0,
                24,
            )
            .translate(*x, *y, 0.0)
            + centered_cube(
                format!("inline_sensor_cartridge_hydration_calibration_station_m6_slot_relief_{i}"),
                24.0,
                6.8,
                BASE_Z + 4.0,
            )
            .translate(*x, *y, 0.0);
    }
    slots
}

fn datum_pin_holes() -> Part {
    let mut holes =
        Part::empty("inline_sensor_cartridge_hydration_calibration_station_datum_pin_holes");
    for (i, (x, y)) in [
        (-566.0, 344.0),
        (566.0, 344.0),
        (-566.0, -344.0),
        (566.0, -344.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!(
                    "inline_sensor_cartridge_hydration_calibration_station_datum_pin_clearance_{i}"
                ),
                5.0 / 2.0,
                BASE_Z + 4.0,
                24,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    let left = centered_cube(
        "inline_sensor_cartridge_hydration_calibration_station_left_rim",
        RIM_W,
        STATION_Y - 56.0,
        RIM_Z,
    )
    .translate(
        -(STATION_X / 2.0 - RIM_W / 2.0),
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(
        "inline_sensor_cartridge_hydration_calibration_station_right_rim",
        RIM_W,
        STATION_Y - 56.0,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "inline_sensor_cartridge_hydration_calibration_station_rear_rim",
        STATION_X - 36.0,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let front_lip = centered_cube(
        "inline_sensor_cartridge_hydration_calibration_station_front_low_lip",
        STATION_X - 170.0,
        14.0,
        22.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 25.0, BASE_Z / 2.0 + 11.0);
    left + right + rear + front_lip
}

fn zone_dividers() -> Part {
    let wet_to_dry = centered_cube(
        "inline_sensor_cartridge_hydration_calibration_station_wet_to_dry_row_divider",
        STATION_X - 150.0,
        12.0,
        30.0,
    )
    .translate(0.0, 104.0, BASE_Z / 2.0 + 15.0);
    let release_to_waste = centered_cube(
        "inline_sensor_cartridge_hydration_calibration_station_release_to_waste_row_divider",
        STATION_X - 166.0,
        10.0,
        26.0,
    )
    .translate(0.0, -154.0, BASE_Z / 2.0 + 13.0);
    let incoming_to_hydration = centered_cube(
        "inline_sensor_cartridge_hydration_calibration_station_incoming_to_hydration_column_divider",
        10.0,
        230.0,
        28.0,
    )
    .translate(-216.0, 230.0, BASE_Z / 2.0 + 14.0);
    let hydration_to_calibration = centered_cube(
        "inline_sensor_cartridge_hydration_calibration_station_hydration_to_calibration_column_divider",
        10.0,
        230.0,
        28.0,
    )
    .translate(188.0, 230.0, BASE_Z / 2.0 + 14.0);
    let used_barrier = centered_cube(
        "inline_sensor_cartridge_hydration_calibration_station_used_consumable_vertical_barrier",
        12.0,
        190.0,
        28.0,
    )
    .translate(242.0, -256.0, BASE_Z / 2.0 + 14.0);
    wet_to_dry + release_to_waste + incoming_to_hydration + hydration_to_calibration + used_barrier
}

fn rear_service_bulkheads() -> Part {
    let mut tabs =
        Part::empty("inline_sensor_cartridge_hydration_calibration_station_rear_bulkheads");
    for (i, x) in [-480.0, -320.0, -160.0, 0.0, 160.0, 320.0, 480.0]
        .iter()
        .enumerate()
    {
        let tab = centered_cube(
            format!("inline_sensor_cartridge_hydration_calibration_station_rear_bulkhead_{i}"),
            54.0,
            20.0,
            28.0,
        )
        .translate(*x, STATION_Y / 2.0 - 46.0, BASE_Z / 2.0 + 14.0);
        let bore = centered_cylinder(
            format!("inline_sensor_cartridge_hydration_calibration_station_rear_bulkhead_bore_{i}"),
            7.2 / 2.0,
            26.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, STATION_Y / 2.0 - 46.0, BASE_Z / 2.0 + 14.0);
        tabs = tabs + (tab - bore);
    }
    tabs
}

fn leak_witness_channels() -> Part {
    let mut channels =
        Part::empty("inline_sensor_cartridge_hydration_calibration_station_leak_witness_channels");
    for i in 0..LEAK_CHANNEL_COUNT {
        let x = centered_index(i, LEAK_CHANNEL_COUNT, 214.0);
        channels = channels
            + centered_cube(
                format!(
                    "inline_sensor_cartridge_hydration_calibration_station_leak_witness_rib_{i}"
                ),
                132.0,
                6.0,
                7.0,
            )
            .translate(x, -342.0, BASE_Z / 2.0 + 3.5);
    }
    channels
}

fn robot_fiducial_targets() -> Part {
    let mut targets =
        Part::empty("inline_sensor_cartridge_hydration_calibration_station_robot_fiducials");
    for (i, (x, y)) in [(-548.0, 328.0), (548.0, 328.0), (-548.0, -326.0)]
        .iter()
        .enumerate()
    {
        targets = targets
            + fiducial_disc(&format!(
                "inline_sensor_cartridge_hydration_calibration_station_robot_fiducial_{i}"
            ))
            .translate(*x, *y, BASE_Z / 2.0 + 3.0);
    }
    targets
}

fn clean_cartridge_rack() -> Part {
    let body = centered_cube(
        "inline_sensor_cartridge_clean_rack_body",
        RACK_X,
        RACK_Y,
        RACK_Z,
    );
    let rear_fence = centered_cube(
        "inline_sensor_cartridge_clean_rack_rear_fence",
        RACK_X,
        14.0,
        RACK_Z + 28.0,
    )
    .translate(0.0, RACK_Y / 2.0 - 7.0, 14.0);

    let mut slot_cuts = Part::empty("inline_sensor_cartridge_clean_rack_slot_cuts");
    let mut sensor_type_keys = Part::empty("inline_sensor_cartridge_clean_rack_type_keys");
    for row in 0..CARTRIDGE_ROWS {
        for col in 0..CARTRIDGE_COLS {
            let index = row * CARTRIDGE_COLS + col;
            let x = centered_index(col, CARTRIDGE_COLS, CARTRIDGE_PITCH_X);
            let y = centered_index(row, CARTRIDGE_ROWS, CARTRIDGE_PITCH_Y);
            let slot = centered_cube(
                format!("inline_sensor_cartridge_clean_slot_{index}"),
                CARTRIDGE_SLOT_X,
                CARTRIDGE_SLOT_Y,
                CARTRIDGE_SLOT_Z,
            )
            .translate(x, y, RACK_Z / 2.0 - CARTRIDGE_SLOT_Z / 2.0 + 1.0);
            let finger_relief = centered_cube(
                format!("inline_sensor_cartridge_clean_slot_finger_relief_{index}"),
                14.0,
                CARTRIDGE_SLOT_Y + 8.0,
                20.0,
            )
            .translate(x, y - 2.0, RACK_Z / 2.0 - 3.0);
            slot_cuts = slot_cuts + slot + finger_relief;

            if row == 0 && col < SENSOR_TYPES {
                sensor_type_keys = sensor_type_keys
                    + centered_cube(
                        format!("inline_sensor_cartridge_family_key_land_{col}"),
                        28.0,
                        6.0,
                        5.0,
                    )
                    .translate(x, -(RACK_Y / 2.0 - 22.0), RACK_Z / 2.0 + 2.5);
            }
        }
    }

    body + rear_fence + sensor_type_keys - slot_cuts + gripper_fiducials("clean_rack", 146.0)
}

fn hydration_buffer_wells() -> Part {
    let body = centered_cube(
        "inline_sensor_cartridge_hydration_buffer_block",
        HYDRATION_X,
        HYDRATION_Y,
        HYDRATION_Z,
    );
    let lid_land = centered_cube(
        "inline_sensor_cartridge_hydration_evaporation_lid_land",
        HYDRATION_X - 26.0,
        HYDRATION_Y - 26.0,
        8.0,
    )
    .translate(0.0, 0.0, HYDRATION_Z / 2.0 + 4.0);

    let mut wells = Part::empty("inline_sensor_cartridge_hydration_well_cuts");
    for row in 0..CARTRIDGE_ROWS {
        for col in 0..CARTRIDGE_COLS {
            let index = row * CARTRIDGE_COLS + col;
            wells = wells
                + centered_cylinder(
                    format!("inline_sensor_cartridge_hydration_buffer_well_{index}"),
                    13.0,
                    HYDRATION_Z + 4.0,
                    40,
                )
                .translate(
                    centered_index(col, CARTRIDGE_COLS, CARTRIDGE_PITCH_X),
                    centered_index(row, CARTRIDGE_ROWS, 36.0),
                    4.0,
                );
        }
    }

    let fill_channel = centered_cylinder(
        "inline_sensor_cartridge_hydration_fill_manifold_bore",
        5.5 / 2.0,
        HYDRATION_X - 50.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, HYDRATION_Y / 2.0 - 28.0, 4.0);
    let drain_channel = centered_cylinder(
        "inline_sensor_cartridge_hydration_drain_manifold_bore",
        5.5 / 2.0,
        HYDRATION_X - 50.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -(HYDRATION_Y / 2.0 - 28.0), 4.0);
    let temp_probe_wells = centered_cylinder(
        "inline_sensor_cartridge_hydration_left_temp_probe_well",
        4.0 / 2.0,
        HYDRATION_Z + 8.0,
        24,
    )
    .translate(-(HYDRATION_X / 2.0 - 36.0), 0.0, 2.0)
        + centered_cylinder(
            "inline_sensor_cartridge_hydration_right_temp_probe_well",
            4.0 / 2.0,
            HYDRATION_Z + 8.0,
            24,
        )
        .translate(HYDRATION_X / 2.0 - 36.0, 0.0, 2.0);

    body + lid_land - wells - fill_channel - drain_channel - temp_probe_wells
        + gripper_fiducials("hydration_block", 138.0)
}

fn calibration_standard_pockets() -> Part {
    let body = centered_cube(
        "inline_sensor_cartridge_calibration_standard_bank",
        CAL_BANK_X,
        CAL_BANK_Y,
        CAL_BANK_Z,
    );
    let rear_backer = centered_cube(
        "inline_sensor_cartridge_calibration_standard_rear_backer",
        CAL_BANK_X,
        14.0,
        CAL_BANK_Z + 34.0,
    )
    .translate(0.0, CAL_BANK_Y / 2.0 - 7.0, 17.0);

    let mut pockets = Part::empty("inline_sensor_cartridge_calibration_standard_pocket_cuts");
    for i in 0..CAL_STANDARD_COUNT {
        let col = i % 6;
        let row = i / 6;
        let x = centered_index(col, 6, 58.0);
        let y = centered_index(row, 2, 58.0);
        pockets = pockets
            + centered_cylinder(
                format!("inline_sensor_cartridge_calibration_standard_pocket_{i}"),
                17.0,
                CAL_BANK_Z + 4.0,
                40,
            )
            .translate(x, y, 3.0)
            + centered_cube(
                format!("inline_sensor_cartridge_calibration_standard_label_recess_{i}"),
                36.0,
                8.0,
                5.0,
            )
            .translate(x, y - 27.0, CAL_BANK_Z / 2.0 - 2.5);
    }

    let chilled_standard_rail = centered_cube(
        "inline_sensor_cartridge_calibration_chilled_standard_rail",
        CAL_BANK_X - 42.0,
        14.0,
        16.0,
    )
    .translate(0.0, -(CAL_BANK_Y / 2.0 - 18.0), CAL_BANK_Z / 2.0 + 8.0);
    let optical_blank_lane = centered_cube(
        "inline_sensor_cartridge_calibration_dark_blank_lane",
        94.0,
        26.0,
        18.0,
    )
    .translate(CAL_BANK_X / 2.0 - 72.0, 0.0, CAL_BANK_Z / 2.0 + 9.0);
    let pressure_flow_cal_ports = calibration_pressure_flow_ports();

    body + rear_backer + chilled_standard_rail + optical_blank_lane + pressure_flow_cal_ports
        - pockets
        + gripper_fiducials("calibration_bank", 164.0)
}

fn calibration_pressure_flow_ports() -> Part {
    let mut ports = Part::empty("inline_sensor_cartridge_pressure_flow_calibration_ports");
    for (i, y) in [-54.0, -18.0, 18.0, 54.0].iter().enumerate() {
        let boss = centered_cube(
            format!("inline_sensor_cartridge_pressure_flow_calibration_port_boss_{i}"),
            44.0,
            24.0,
            24.0,
        )
        .translate(-(CAL_BANK_X / 2.0 - 36.0), *y, CAL_BANK_Z / 2.0 + 12.0);
        let bore = centered_cylinder(
            format!("inline_sensor_cartridge_pressure_flow_calibration_port_bore_{i}"),
            6.0 / 2.0,
            30.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(-(CAL_BANK_X / 2.0 - 36.0), *y, CAL_BANK_Z / 2.0 + 12.0);
        ports = ports + (boss - bore);
    }
    ports
}

fn dark_temperature_equilibration_cover() -> Part {
    let roof = centered_cube(
        "inline_sensor_cartridge_dark_temperature_cover_roof",
        DARK_COVER_X,
        DARK_COVER_Y,
        18.0,
    )
    .translate(0.0, 0.0, DARK_COVER_Z / 2.0 - 9.0);
    let left_wall = centered_cube(
        "inline_sensor_cartridge_dark_temperature_cover_left_wall",
        18.0,
        DARK_COVER_Y,
        DARK_COVER_Z,
    )
    .translate(-(DARK_COVER_X / 2.0 - 9.0), 0.0, 0.0);
    let right_wall = centered_cube(
        "inline_sensor_cartridge_dark_temperature_cover_right_wall",
        18.0,
        DARK_COVER_Y,
        DARK_COVER_Z,
    )
    .translate(DARK_COVER_X / 2.0 - 9.0, 0.0, 0.0);
    let rear_wall = centered_cube(
        "inline_sensor_cartridge_dark_temperature_cover_rear_wall",
        DARK_COVER_X,
        18.0,
        DARK_COVER_Z,
    )
    .translate(0.0, DARK_COVER_Y / 2.0 - 9.0, 0.0);
    let front_service_lip = centered_cube(
        "inline_sensor_cartridge_dark_temperature_cover_front_service_lip",
        DARK_COVER_X - 150.0,
        14.0,
        36.0,
    )
    .translate(
        0.0,
        -(DARK_COVER_Y / 2.0 - 14.0),
        -(DARK_COVER_Z / 2.0 - 18.0),
    );
    let thermal_jacket_channel = centered_cylinder(
        "inline_sensor_cartridge_dark_temperature_cover_thermal_jacket_channel",
        7.0 / 2.0,
        DARK_COVER_X - 92.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, DARK_COVER_Y / 2.0 - 42.0, DARK_COVER_Z / 2.0 - 28.0);

    let mut inspection_ports = Part::empty("inline_sensor_cartridge_dark_cover_shuttered_ports");
    for (i, x) in [-286.0, -96.0, 96.0, 286.0].iter().enumerate() {
        inspection_ports = inspection_ports
            + centered_cube(
                format!("inline_sensor_cartridge_dark_cover_shuttered_window_{i}"),
                58.0,
                10.0,
                34.0,
            )
            .translate(*x, -(DARK_COVER_Y / 2.0 - 7.0), 0.0);
    }

    roof + left_wall + right_wall + rear_wall + front_service_lip
        - thermal_jacket_channel
        - inspection_ports
}

fn electrical_pogo_check_fixture() -> Part {
    let base = centered_cube(
        "inline_sensor_cartridge_pogo_check_fixture_base",
        POGO_X,
        POGO_Y,
        POGO_Z,
    );
    let rear_connector_bulkhead = centered_cube(
        "inline_sensor_cartridge_pogo_check_rear_connector_bulkhead",
        POGO_X - 40.0,
        18.0,
        POGO_Z + 36.0,
    )
    .translate(0.0, POGO_Y / 2.0 - 9.0, 18.0);

    let mut nest_cuts = Part::empty("inline_sensor_cartridge_pogo_check_nest_cuts");
    for i in 0..POGO_CHECK_POSITIONS {
        let x = centered_index(i, POGO_CHECK_POSITIONS, 54.0);
        nest_cuts = nest_cuts
            + centered_cube(
                format!("inline_sensor_cartridge_pogo_check_cartridge_nest_{i}"),
                42.0,
                50.0,
                28.0,
            )
            .translate(x, -30.0, POGO_Z / 2.0 - 12.0)
            + centered_cube(
                format!("inline_sensor_cartridge_pogo_check_lead_clearance_{i}"),
                22.0,
                84.0,
                16.0,
            )
            .translate(x, 18.0, POGO_Z / 2.0 - 8.0);
    }

    let mut pogo_lanes = Part::empty("inline_sensor_cartridge_pogo_pin_lane_bosses");
    for lane in 0..POGO_PIN_LANES {
        let y = centered_index(lane, POGO_PIN_LANES, 16.0) + 18.0;
        pogo_lanes = pogo_lanes
            + centered_cube(
                format!("inline_sensor_cartridge_pogo_pin_lane_{lane}"),
                POGO_X - 86.0,
                5.0,
                6.0,
            )
            .translate(0.0, y, POGO_Z / 2.0 + 3.0);
    }

    let teer_resistor_sockets = centered_cube(
        "inline_sensor_cartridge_teer_reference_resistor_socket_low",
        92.0,
        22.0,
        18.0,
    )
    .translate(
        -(POGO_X / 2.0 - 72.0),
        -(POGO_Y / 2.0 - 24.0),
        POGO_Z / 2.0 - 9.0,
    ) + centered_cube(
        "inline_sensor_cartridge_teer_reference_resistor_socket_high",
        92.0,
        22.0,
        18.0,
    )
    .translate(
        POGO_X / 2.0 - 72.0,
        -(POGO_Y / 2.0 - 24.0),
        POGO_Z / 2.0 - 9.0,
    );

    base + rear_connector_bulkhead + pogo_lanes - nest_cuts - teer_resistor_sockets
        + gripper_fiducials("pogo_fixture", 204.0)
}

fn wet_dry_release_lanes() -> Part {
    let body = centered_cube(
        "inline_sensor_cartridge_wet_dry_release_lane_body",
        STATUS_X,
        STATUS_Y,
        STATUS_Z,
    );
    let mut lane_cuts = Part::empty("inline_sensor_cartridge_wet_dry_release_lane_slot_cuts");
    let mut lane_labels = Part::empty("inline_sensor_cartridge_wet_dry_release_lane_labels");
    for lane in 0..STATUS_LANES {
        let y = centered_index(lane, STATUS_LANES, 31.0);
        lane_labels = lane_labels
            + centered_cube(
                format!("inline_sensor_cartridge_release_lane_label_land_{lane}"),
                54.0,
                6.0,
                5.0,
            )
            .translate(-(STATUS_X / 2.0 - 42.0), y, STATUS_Z / 2.0 + 2.5);
        for slot in 0..STATUS_SLOTS_PER_LANE {
            let x = centered_index(slot, STATUS_SLOTS_PER_LANE, 46.0) + 34.0;
            lane_cuts = lane_cuts
                + centered_cube(
                    format!("inline_sensor_cartridge_release_lane_{lane}_slot_{slot}"),
                    34.0,
                    20.0,
                    24.0,
                )
                .translate(x, y, STATUS_Z / 2.0 - 11.0);
        }
    }
    let front_pick_bevel = centered_cube(
        "inline_sensor_cartridge_release_lane_front_pick_relief",
        STATUS_X - 58.0,
        14.0,
        18.0,
    )
    .translate(22.0, -(STATUS_Y / 2.0 - 16.0), STATUS_Z / 2.0 - 6.0);
    body + lane_labels - lane_cuts - front_pick_bevel + gripper_fiducials("status_lanes", 154.0)
}

fn barcode_certificate_lands() -> Part {
    let panel = centered_cube(
        "inline_sensor_cartridge_barcode_certificate_panel",
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    );
    let mut barcode_lands = Part::empty("inline_sensor_cartridge_barcode_lands");
    for i in 0..BARCODE_LANDS {
        let col = i % 4;
        let row = i / 4;
        barcode_lands = barcode_lands
            + centered_cube(
                format!("inline_sensor_cartridge_barcode_land_{i}"),
                58.0,
                18.0,
                4.0,
            )
            .translate(
                centered_index(col, 4, 70.0),
                38.0 - row as f64 * 30.0,
                TRACE_Z / 2.0 + 2.0,
            );
    }

    let mut certificate_lands = Part::empty("inline_sensor_cartridge_certificate_lands");
    for i in 0..CERTIFICATE_LANDS {
        certificate_lands = certificate_lands
            + centered_cube(
                format!("inline_sensor_cartridge_calibration_certificate_land_{i}"),
                86.0,
                20.0,
                5.0,
            )
            .translate(
                centered_index(i % 3, 3, 96.0),
                -52.0 + (i / 3) as f64 * 26.0,
                TRACE_Z / 2.0 + 2.5,
            );
    }

    let rfid_edge = centered_cube(
        "inline_sensor_cartridge_rfid_reader_edge_reference",
        TRACE_X - 44.0,
        8.0,
        9.0,
    )
    .translate(0.0, TRACE_Y / 2.0 - 12.0, TRACE_Z / 2.0 + 4.5);
    panel + barcode_lands + certificate_lands + rfid_edge
}

fn spent_standard_waste_wells() -> Part {
    let body = centered_cube(
        "inline_sensor_cartridge_spent_standard_waste_block",
        WASTE_X,
        WASTE_Y,
        WASTE_Z,
    );
    let rear_splash_wall = centered_cube(
        "inline_sensor_cartridge_spent_standard_rear_splash_wall",
        WASTE_X,
        12.0,
        WASTE_Z + 34.0,
    )
    .translate(0.0, WASTE_Y / 2.0 - 6.0, 17.0);

    let mut wells = Part::empty("inline_sensor_cartridge_spent_standard_well_cuts");
    for i in 0..SPENT_WASTE_WELLS {
        wells = wells
            + centered_cylinder(
                format!("inline_sensor_cartridge_spent_standard_waste_well_{i}"),
                15.5,
                WASTE_Z + 5.0,
                40,
            )
            .translate(
                centered_index(i % 6, 6, 48.0),
                centered_index(i / 6, 2, 52.0),
                4.0,
            );
    }

    let drain_channel = centered_cylinder(
        "inline_sensor_cartridge_spent_standard_drain_manifold",
        6.5 / 2.0,
        WASTE_X - 56.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -(WASTE_Y / 2.0 - 22.0), 0.0);
    let absorbent_pad_land = centered_cube(
        "inline_sensor_cartridge_spent_standard_absorbent_pad_land",
        WASTE_X - 58.0,
        20.0,
        8.0,
    )
    .translate(0.0, WASTE_Y / 2.0 - 32.0, WASTE_Z / 2.0 + 4.0);
    body + rear_splash_wall + absorbent_pad_land - wells - drain_channel
}

fn sample_loop_handoff_ports() -> Part {
    let body = centered_cube(
        "inline_sensor_cartridge_sample_loop_handoff_body",
        HANDOFF_X,
        HANDOFF_Y,
        HANDOFF_Z,
    );
    let rear_bulkhead = centered_cube(
        "inline_sensor_cartridge_sample_loop_handoff_rear_bulkhead",
        HANDOFF_X,
        16.0,
        HANDOFF_Z + 34.0,
    )
    .translate(0.0, HANDOFF_Y / 2.0 - 8.0, 17.0);

    let mut ports = Part::empty("inline_sensor_cartridge_sample_loop_handoff_port_cuts");
    for i in 0..SAMPLE_LOOP_PORTS {
        let col = i % 7;
        let row = i / 7;
        ports = ports
            + centered_cylinder(
                format!("inline_sensor_cartridge_sample_loop_handoff_port_{i}"),
                6.2 / 2.0,
                HANDOFF_Y + 8.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(col, 7, 46.0),
                0.0,
                centered_index(row, 2, 20.0),
            );
    }

    let valve_label_lands = centered_cube(
        "inline_sensor_cartridge_sample_loop_handoff_inlet_labels",
        HANDOFF_X - 60.0,
        10.0,
        6.0,
    )
    .translate(0.0, -(HANDOFF_Y / 2.0 - 22.0), HANDOFF_Z / 2.0 + 3.0);
    let robot_grip_ears = grip_ear("inline_sensor_cartridge_sample_loop_handoff_left_grip")
        .translate(-(HANDOFF_X / 2.0 - 24.0), 0.0, 4.0)
        + grip_ear("inline_sensor_cartridge_sample_loop_handoff_right_grip").translate(
            HANDOFF_X / 2.0 - 24.0,
            0.0,
            4.0,
        );

    body + rear_bulkhead + valve_label_lands + robot_grip_ears - ports
}

fn clean_used_segregation() -> Part {
    let body = centered_cube(
        "inline_sensor_cartridge_clean_used_segregation_body",
        SEGREGATION_X,
        SEGREGATION_Y,
        SEGREGATION_Z,
    );
    let divider = centered_cube(
        "inline_sensor_cartridge_clean_used_physical_barrier",
        16.0,
        SEGREGATION_Y,
        SEGREGATION_Z + 36.0,
    )
    .translate(0.0, 0.0, 18.0);

    let clean_bin = centered_cube(
        "inline_sensor_cartridge_clean_return_bin_cut",
        122.0,
        SEGREGATION_Y - 34.0,
        SEGREGATION_Z - 8.0,
    )
    .translate(-(SEGREGATION_X / 4.0), 0.0, 2.0);
    let used_bin = centered_cube(
        "inline_sensor_cartridge_used_quarantine_bin_cut",
        122.0,
        SEGREGATION_Y - 34.0,
        SEGREGATION_Z - 8.0,
    )
    .translate(SEGREGATION_X / 4.0, 0.0, 2.0);
    let cap_park_posts = cap_and_plug_posts();
    let wipe_coupon_slots = wipe_coupon_slots();

    body + divider + cap_park_posts - clean_bin - used_bin - wipe_coupon_slots
}

fn cap_and_plug_posts() -> Part {
    let mut posts = Part::empty("inline_sensor_cartridge_cap_and_plug_posts");
    for i in 0..12 {
        posts = posts
            + centered_cylinder(
                format!("inline_sensor_cartridge_clean_cap_post_{i}"),
                5.0,
                22.0,
                24,
            )
            .translate(
                -(SEGREGATION_X / 4.0) + centered_index(i % 6, 6, 18.0),
                SEGREGATION_Y / 2.0 - 20.0 - (i / 6) as f64 * 24.0,
                SEGREGATION_Z / 2.0 + 11.0,
            );
    }
    posts
}

fn wipe_coupon_slots() -> Part {
    let mut slots = Part::empty("inline_sensor_cartridge_wipe_coupon_slot_cuts");
    for i in 0..6 {
        slots = slots
            + centered_cube(
                format!("inline_sensor_cartridge_wipe_coupon_slot_{i}"),
                42.0,
                8.0,
                12.0,
            )
            .translate(
                SEGREGATION_X / 4.0,
                centered_index(i, 6, 18.0),
                SEGREGATION_Z / 2.0 - 5.0,
            );
    }
    slots
}

fn robot_service_keepouts() -> Part {
    let mut zones =
        Part::empty("inline_sensor_cartridge_hydration_calibration_station_keepout_zones");
    for (i, (name, x, y, width, depth)) in [
        ("front_robot_pick_sweep", 0.0, -366.0, 1060.0, 10.0),
        ("rear_tube_cable_sweep", 0.0, 356.0, 1120.0, 10.0),
        ("left_service_lane", -594.0, 0.0, 10.0, 660.0),
        ("right_standard_change_lane", 594.0, 0.0, 10.0, 660.0),
        (
            "dark_cover_lift_envelope",
            190.0,
            230.0,
            DARK_COVER_X + 36.0,
            8.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        zones = zones
            + centered_cube(
                format!("inline_sensor_cartridge_keepout_{i}_{name}"),
                *width,
                *depth,
                6.0,
            )
            .translate(*x, *y, 0.0);
    }
    let centerline = centered_cube(
        "inline_sensor_cartridge_keepout_centerline_x",
        STATION_X - 140.0,
        4.0,
        6.0,
    ) + centered_cube(
        "inline_sensor_cartridge_keepout_centerline_y",
        4.0,
        STATION_Y - 140.0,
        6.0,
    );
    zones + centerline
}

fn grip_ear(name: &str) -> Part {
    let ear = centered_cube(format!("{name}_body"), 24.0, 44.0, 18.0);
    let clearance = centered_cylinder(format!("{name}_m4_clearance"), 4.4 / 2.0, 24.0, 22)
        .translate(0.0, 0.0, 0.0);
    ear - clearance
}

fn gripper_fiducials(name: &str, x_offset: f64) -> Part {
    let mut fiducials = Part::empty(format!("inline_sensor_cartridge_{name}_gripper_fiducials"));
    for (i, x) in [-x_offset, x_offset].iter().enumerate() {
        fiducials = fiducials
            + fiducial_disc(&format!("inline_sensor_cartridge_{name}_fiducial_{i}"))
                .translate(*x, 0.0, 4.0);
    }
    fiducials
}

fn fiducial_disc(name: &str) -> Part {
    let disc = centered_cylinder(format!("{name}_disc"), 5.0, 2.0, 32);
    let center = centered_cylinder(format!("{name}_center"), 1.2, 3.0, 18);
    disc - center
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_and_station_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 13);
        for path in OUTPUTS {
            assert!(
                path.starts_with("output/inline_sensor_cartridge_hydration_calibration_station_"),
                "{path}"
            );
            assert!(path.ends_with(".stl"), "{path}");
        }
    }

    #[test]
    fn station_covers_required_release_features() {
        assert_eq!(REQUIRED_FEATURES.len(), 12);
        assert!(REQUIRED_FEATURES.contains(&"clean_sensor_cartridge_rack"));
        assert!(REQUIRED_FEATURES.contains(&"hydration_buffer_wells"));
        assert!(REQUIRED_FEATURES.contains(&"calibration_standard_pockets"));
        assert!(REQUIRED_FEATURES.contains(&"dark_temperature_equilibration_cover"));
        assert!(REQUIRED_FEATURES.contains(&"electrical_pogo_check_fixture"));
        assert!(REQUIRED_FEATURES.contains(&"wet_dry_status_lanes"));
        assert!(REQUIRED_FEATURES.contains(&"barcode_calibration_certificate_lands"));
        assert!(REQUIRED_FEATURES.contains(&"spent_standard_waste_wells"));
        assert!(REQUIRED_FEATURES.contains(&"sample_loop_handoff_ports"));
        assert!(REQUIRED_FEATURES.contains(&"clean_used_segregation"));
        assert!(REQUIRED_FEATURES.contains(&"leak_tray"));
        assert!(REQUIRED_FEATURES.contains(&"robot_service_keepouts"));
    }

    #[test]
    fn all_insert_modules_fit_inside_station_rims() {
        for (_name, pos, width, depth) in insert_specs() {
            assert!(fits_on_station(pos, width, depth));
        }
        assert!(DARK_COVER_POS.0.abs() + DARK_COVER_X / 2.0 <= STATION_X / 2.0 - RIM_W - 8.0);
        assert!(DARK_COVER_POS.1.abs() + DARK_COVER_Y / 2.0 <= STATION_Y / 2.0 - RIM_W - 8.0);
    }

    #[test]
    fn cartridge_hydration_and_release_counts_match_throughput() {
        assert_eq!(CARTRIDGE_COUNT, 24);
        assert_eq!(CARTRIDGE_COUNT, CARTRIDGE_ROWS * CARTRIDGE_COLS);
        assert_eq!(HYDRATION_WELL_COUNT, CARTRIDGE_COUNT);
        assert_eq!(CAL_STANDARD_COUNT, 12);
        assert_eq!(STATUS_LANES * STATUS_SLOTS_PER_LANE, 30);
        assert!(STATUS_LANES >= 5);
    }

    #[test]
    fn sensor_families_cover_inline_culture_signals() {
        assert_eq!(SENSOR_TYPES, 7);
        assert!(POGO_CHECK_POSITIONS >= SENSOR_TYPES);
        assert_eq!(POGO_PIN_LANES, SENSOR_TYPES);
        assert!(SAMPLE_LOOP_PORTS >= SENSOR_TYPES * 2);
    }

    #[test]
    fn traceability_and_waste_are_sized_for_batch_release() {
        assert_eq!(BARCODE_LANDS, 12);
        assert_eq!(CERTIFICATE_LANDS, 6);
        assert_eq!(SPENT_WASTE_WELLS, CAL_STANDARD_COUNT);
        assert!(TRACE_X >= 330.0);
        assert!(WASTE_Z >= 50.0);
    }

    #[test]
    fn clean_and_used_zones_are_physically_separated() {
        let clean_bin_center_x = SEGREGATION_POS.0 - SEGREGATION_X / 4.0;
        let used_bin_center_x = SEGREGATION_POS.0 + SEGREGATION_X / 4.0;
        let bin_half_width = 122.0 / 2.0;
        let clear_gap = used_bin_center_x - bin_half_width - (clean_bin_center_x + bin_half_width);
        assert!(clear_gap >= 28.0);
        assert!(SEGREGATION_Z >= 48.0);
    }

    #[test]
    fn containment_and_robot_access_are_explicit() {
        assert_eq!(LEAK_CHANNEL_COUNT, 5);
        assert_eq!(KEEP_OUT_ZONE_COUNT, 5);
        assert!(STATION_X >= 1200.0);
        assert!(STATION_Y >= 800.0);
        assert!(RIM_Z >= 40.0);
    }
}
