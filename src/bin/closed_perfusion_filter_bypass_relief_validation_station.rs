use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed perfusion filter bypass and relief-valve validation station.
//
// Intent:
// - Validate closed tissue-chip fluid handling behavior around inline filter
//   bypass routing and pressure relief events before a live-cell run.
// - Keep cartridge nests, bypass loop witnesses, surrogate relief ports,
//   pressure sensor pockets, bubble/wetness windows, waste capture, traceable
//   barcode/certificate lands, and disposition lanes in one contained fixture.
// - Represent human-readable station markings as CSG label lands, raised bars,
//   arrows, tokens, and certificate plaques so the geometry remains self
//   describing without requiring a separate decal file.
//
// This is architecture CAD only. It is not a validated sterile barrier,
// pressure safety device, wetted-path specification, or release procedure.
//
// Exports:
//   output/closed_perfusion_filter_bypass_relief_validation_station_base_containment_tray.stl
//   output/closed_perfusion_filter_bypass_relief_validation_station_filter_cartridge_nests.stl
//   output/closed_perfusion_filter_bypass_relief_validation_station_bypass_loop_witness_channels.stl
//   output/closed_perfusion_filter_bypass_relief_validation_station_relief_valve_surrogate_ports.stl
//   output/closed_perfusion_filter_bypass_relief_validation_station_pressure_sensor_pockets.stl
//   output/closed_perfusion_filter_bypass_relief_validation_station_bubble_wetness_windows.stl
//   output/closed_perfusion_filter_bypass_relief_validation_station_waste_capture_wells.stl
//   output/closed_perfusion_filter_bypass_relief_validation_station_barcode_certificate_lands.stl
//   output/closed_perfusion_filter_bypass_relief_validation_station_release_hold_reject_lanes.stl
//   output/closed_perfusion_filter_bypass_relief_validation_station_csg_zone_labels.stl
//   output/closed_perfusion_filter_bypass_relief_validation_station_assembly.stl

const OUTPUTS: [&str; 11] = [
    "output/closed_perfusion_filter_bypass_relief_validation_station_base_containment_tray.stl",
    "output/closed_perfusion_filter_bypass_relief_validation_station_filter_cartridge_nests.stl",
    "output/closed_perfusion_filter_bypass_relief_validation_station_bypass_loop_witness_channels.stl",
    "output/closed_perfusion_filter_bypass_relief_validation_station_relief_valve_surrogate_ports.stl",
    "output/closed_perfusion_filter_bypass_relief_validation_station_pressure_sensor_pockets.stl",
    "output/closed_perfusion_filter_bypass_relief_validation_station_bubble_wetness_windows.stl",
    "output/closed_perfusion_filter_bypass_relief_validation_station_waste_capture_wells.stl",
    "output/closed_perfusion_filter_bypass_relief_validation_station_barcode_certificate_lands.stl",
    "output/closed_perfusion_filter_bypass_relief_validation_station_release_hold_reject_lanes.stl",
    "output/closed_perfusion_filter_bypass_relief_validation_station_csg_zone_labels.stl",
    "output/closed_perfusion_filter_bypass_relief_validation_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 10] = [
    "filter_cartridge_nests",
    "bypass_loop_witness_channels",
    "relief_valve_surrogate_ports",
    "pressure_sensor_pockets",
    "bubble_wetness_windows",
    "waste_capture_wells",
    "barcode_certificate_lands",
    "release_hold_reject_lanes",
    "csg_zone_labels",
    "base_containment_tray",
];

const PREFIX: &str = "closed_perfusion_filter_bypass_relief_validation_station";

const STATION_X: f64 = 1220.0;
const STATION_Y: f64 = 780.0;
const BASE_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const SOCKET_DEPTH: f64 = 5.0;
const DRAIN_D: f64 = 12.0;
const MOUNT_HOLE_D: f64 = 6.6;

const FILTER_ROWS: usize = 2;
const FILTER_COLS: usize = 4;
const FILTER_COUNT: usize = FILTER_ROWS * FILTER_COLS;
const FILTER_NEST_X: f64 = 468.0;
const FILTER_NEST_Y: f64 = 242.0;
const FILTER_NEST_Z: f64 = 58.0;
const FILTER_NEST_POS: (f64, f64) = (-344.0, 208.0);
const FILTER_SLOT_X: f64 = 86.0;
const FILTER_SLOT_Y: f64 = 42.0;
const FILTER_SLOT_Z: f64 = 27.0;
const FILTER_PITCH_X: f64 = 108.0;
const FILTER_PITCH_Y: f64 = 78.0;
const FILTER_BODY_D: f64 = 31.0;
const FILTER_TUBE_BORE_D: f64 = 6.6;

const BYPASS_X: f64 = 500.0;
const BYPASS_Y: f64 = 242.0;
const BYPASS_Z: f64 = 46.0;
const BYPASS_POS: (f64, f64) = (244.0, 208.0);
const BYPASS_CHANNELS: usize = FILTER_COUNT;
const BYPASS_PITCH_X: f64 = 58.0;
const BYPASS_CHANNEL_W: f64 = 8.0;
const BYPASS_CHANNEL_Z: f64 = 12.0;
const FLOW_ARROW_COUNT: usize = 8;

const RELIEF_X: f64 = 338.0;
const RELIEF_Y: f64 = 190.0;
const RELIEF_Z: f64 = 74.0;
const RELIEF_POS: (f64, f64) = (-384.0, -36.0);
const RELIEF_PORTS: usize = 6;
const RELIEF_PORT_PITCH_X: f64 = 49.0;
const RELIEF_PORT_D: f64 = 24.0;
const RELIEF_BORE_D: f64 = 8.0;
const RELIEF_HEADER_D: f64 = 10.0;

const PRESSURE_X: f64 = 426.0;
const PRESSURE_Y: f64 = 190.0;
const PRESSURE_Z: f64 = 58.0;
const PRESSURE_POS: (f64, f64) = (20.0, -36.0);
const PRESSURE_SENSOR_COUNT: usize = FILTER_COUNT * 2;
const PRESSURE_COLUMNS: usize = FILTER_COUNT;
const PRESSURE_BANKS: usize = 2;
const SENSOR_PITCH_X: f64 = 48.0;
const SENSOR_BANK_PITCH_Y: f64 = 62.0;
const SENSOR_POCKET_X: f64 = 34.0;
const SENSOR_POCKET_Y: f64 = 24.0;
const SENSOR_POCKET_Z: f64 = 14.0;

const WINDOW_X: f64 = 286.0;
const WINDOW_Y: f64 = 190.0;
const WINDOW_Z: f64 = 34.0;
const WINDOW_POS: (f64, f64) = (402.0, -36.0);
const WINDOW_COUNT: usize = 4;
const WETNESS_PAD_COUNT: usize = FILTER_COUNT;
const WINDOW_PITCH_X: f64 = 62.0;
const BUBBLE_DOME_D: f64 = 32.0;

const WASTE_X: f64 = 384.0;
const WASTE_Y: f64 = 162.0;
const WASTE_Z: f64 = 62.0;
const WASTE_POS: (f64, f64) = (-346.0, -268.0);
const WASTE_WELLS: usize = 8;
const WASTE_WELL_D: f64 = 42.0;
const WASTE_WELL_PITCH_X: f64 = 80.0;

const TRACE_X: f64 = 332.0;
const TRACE_Y: f64 = 162.0;
const TRACE_Z: f64 = 12.0;
const TRACE_POS: (f64, f64) = (36.0, -268.0);
const BARCODE_LANDS: usize = FILTER_COUNT;
const CERTIFICATE_LANDS: usize = 4;
const LABEL_BAR_COUNT: usize = 8;

const LANE_X: f64 = 332.0;
const LANE_Y: f64 = 162.0;
const LANE_Z: f64 = 44.0;
const LANE_POS: (f64, f64) = (410.0, -268.0);
const DISPOSITION_LANES: usize = 3;
const LANE_SLOTS_PER_LANE: usize = 4;
const LANE_SLOT_X: f64 = 54.0;
const LANE_SLOT_Y: f64 = 26.0;

const GLOBAL_LABEL_COUNT: usize = 9;
const DATUM_PIN_COUNT: usize = 4;
const ROBOT_FIDUCIAL_COUNT: usize = 4;
const LEAK_WITNESS_RAILS: usize = 6;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_containment_tray();
    export(OUTPUTS[0], &base);

    let nests = filter_cartridge_nests();
    export(OUTPUTS[1], &nests);

    let bypass = bypass_loop_witness_channels();
    export(OUTPUTS[2], &bypass);

    let relief = relief_valve_surrogate_ports();
    export(OUTPUTS[3], &relief);

    let pressure = pressure_sensor_pockets();
    export(OUTPUTS[4], &pressure);

    let windows = bubble_wetness_windows();
    export(OUTPUTS[5], &windows);

    let waste = waste_capture_wells();
    export(OUTPUTS[6], &waste);

    let trace = barcode_certificate_lands();
    export(OUTPUTS[7], &trace);

    let lanes = release_hold_reject_lanes();
    export(OUTPUTS[8], &lanes);

    let labels = csg_zone_labels();
    export(OUTPUTS[9], &labels);

    let assembly =
        base + nests.translate(
            FILTER_NEST_POS.0,
            FILTER_NEST_POS.1,
            insert_z(FILTER_NEST_Z),
        ) + bypass.translate(BYPASS_POS.0, BYPASS_POS.1, insert_z(BYPASS_Z))
            + relief.translate(RELIEF_POS.0, RELIEF_POS.1, insert_z(RELIEF_Z))
            + pressure.translate(PRESSURE_POS.0, PRESSURE_POS.1, insert_z(PRESSURE_Z))
            + windows.translate(WINDOW_POS.0, WINDOW_POS.1, insert_z(WINDOW_Z))
            + waste.translate(WASTE_POS.0, WASTE_POS.1, insert_z(WASTE_Z))
            + trace.translate(TRACE_POS.0, TRACE_POS.1, insert_z(TRACE_Z))
            + lanes.translate(LANE_POS.0, LANE_POS.1, insert_z(LANE_Z))
            + labels.translate(0.0, 0.0, BASE_Z / 2.0 + 6.0);
    export(OUTPUTS[10], &assembly);

    println!();
    println!("Closed perfusion filter bypass and relief-valve validation station:");
    println!(
        "  Footprint:              {STATION_X:.0}mm x {STATION_Y:.0}mm contained tray with {LEAK_WITNESS_RAILS} leak witness rails"
    );
    println!(
        "  Filter handling:        {FILTER_COUNT} cartridge nests in a {FILTER_ROWS}x{FILTER_COLS} array with keyed cradles and tube bores"
    );
    println!(
        "  Bypass validation:      {BYPASS_CHANNELS} visible loop witness channels and {FLOW_ARROW_COUNT} raised flow-direction tokens"
    );
    println!(
        "  Relief behavior:        {RELIEF_PORTS} surrogate relief-valve ports tied to a relief header and cracking-reference token set"
    );
    println!(
        "  Sensing/windows:        {PRESSURE_SENSOR_COUNT} upstream/downstream pressure pockets, {WINDOW_COUNT} bubble windows, {WETNESS_PAD_COUNT} wetness pads"
    );
    println!(
        "  Traceability/status:    {BARCODE_LANDS} barcode lands, {CERTIFICATE_LANDS} certificate lands, {DISPOSITION_LANES} release/hold/reject lanes"
    );
    println!("  Required feature groups: {}", REQUIRED_FEATURES.len());
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

fn filter_center(row: usize, col: usize) -> (f64, f64) {
    (
        centered_index(col, FILTER_COLS, FILTER_PITCH_X),
        centered_index(row, FILTER_ROWS, FILTER_PITCH_Y),
    )
}

fn assert_layout() {
    assert_eq!(FILTER_COUNT, FILTER_ROWS * FILTER_COLS);
    assert_eq!(BYPASS_CHANNELS, FILTER_COUNT);
    assert_eq!(PRESSURE_SENSOR_COUNT, PRESSURE_BANKS * PRESSURE_COLUMNS);
    assert_eq!(BARCODE_LANDS, FILTER_COUNT);
    assert_eq!(DATUM_PIN_COUNT, 4);
    assert_eq!(ROBOT_FIDUCIAL_COUNT, 4);
    assert!(DISPOSITION_LANES * LANE_SLOTS_PER_LANE >= FILTER_COUNT);
    assert_eq!(OUTPUTS.len(), REQUIRED_FEATURES.len() + 1);

    for (name, pos, width, depth) in insert_specs() {
        assert!(
            fits_on_station(pos, width, depth),
            "{name} exceeds station envelope"
        );
    }
}

fn insert_specs() -> [(&'static str, (f64, f64), f64, f64); 8] {
    [
        (
            "filter_cartridge_nests",
            FILTER_NEST_POS,
            FILTER_NEST_X,
            FILTER_NEST_Y,
        ),
        (
            "bypass_loop_witness_channels",
            BYPASS_POS,
            BYPASS_X,
            BYPASS_Y,
        ),
        (
            "relief_valve_surrogate_ports",
            RELIEF_POS,
            RELIEF_X,
            RELIEF_Y,
        ),
        (
            "pressure_sensor_pockets",
            PRESSURE_POS,
            PRESSURE_X,
            PRESSURE_Y,
        ),
        ("bubble_wetness_windows", WINDOW_POS, WINDOW_X, WINDOW_Y),
        ("waste_capture_wells", WASTE_POS, WASTE_X, WASTE_Y),
        ("barcode_certificate_lands", TRACE_POS, TRACE_X, TRACE_Y),
        ("release_hold_reject_lanes", LANE_POS, LANE_X, LANE_Y),
    ]
}

fn fits_on_station(pos: (f64, f64), width: f64, depth: f64) -> bool {
    pos.0.abs() + width / 2.0 <= STATION_X / 2.0 - RIM_W - 8.0
        && pos.1.abs() + depth / 2.0 <= STATION_Y / 2.0 - RIM_W - 8.0
}

fn base_containment_tray() -> Part {
    let deck = centered_cube(format!("{PREFIX}_base_deck"), STATION_X, STATION_Y, BASE_Z);
    let shallow_basin = centered_cube(
        format!("{PREFIX}_base_shallow_containment_basin"),
        STATION_X - 120.0,
        STATION_Y - 112.0,
        8.0,
    )
    .translate(0.0, -8.0, BASE_Z / 2.0 - 3.5);
    let wet_zone_recess = centered_cube(
        format!("{PREFIX}_base_upper_wet_bypass_recess"),
        1070.0,
        242.0,
        8.0,
    )
    .translate(0.0, 208.0, BASE_Z / 2.0 - 4.0);
    let event_recess = centered_cube(
        format!("{PREFIX}_base_relief_event_recess"),
        1070.0,
        170.0,
        8.0,
    )
    .translate(0.0, -38.0, BASE_Z / 2.0 - 4.0);
    let disposition_recess = centered_cube(
        format!("{PREFIX}_base_disposition_recess"),
        1080.0,
        144.0,
        8.0,
    )
    .translate(0.0, -268.0, BASE_Z / 2.0 - 4.0);
    let front_drain = centered_cylinder(
        format!("{PREFIX}_base_front_low_point_drain"),
        DRAIN_D / 2.0,
        50.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 96.0, -STATION_Y / 2.0 - 3.0, -1.0);
    let rear_service_drain = centered_cylinder(
        format!("{PREFIX}_base_rear_service_drain"),
        DRAIN_D / 2.0,
        50.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 148.0, STATION_Y / 2.0 - 94.0, -1.0);

    deck - shallow_basin
        - wet_zone_recess
        - event_recess
        - disposition_recess
        - front_drain
        - rear_service_drain
        - insert_sockets()
        - mounting_slots()
        - datum_pin_holes()
        + perimeter_rims()
        + zone_dividers()
        + leak_witness_rails()
        + station_flow_gutters()
        + robot_fiducials()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty(format!("{PREFIX}_insert_registration_sockets"));
    for (name, pos, width, depth) in insert_specs() {
        sockets = sockets
            + centered_cube(
                format!("{PREFIX}_{name}_socket_recess"),
                width + 8.0,
                depth + 8.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(pos.0, pos.1, BASE_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.2);
    }
    sockets
}

fn mounting_slots() -> Part {
    let mut slots = Part::empty(format!("{PREFIX}_mounting_slots"));
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 58.0), -(STATION_Y / 2.0 - 52.0)),
        (STATION_X / 2.0 - 58.0, -(STATION_Y / 2.0 - 52.0)),
        (-(STATION_X / 2.0 - 58.0), STATION_Y / 2.0 - 52.0),
        (STATION_X / 2.0 - 58.0, STATION_Y / 2.0 - 52.0),
        (0.0, STATION_Y / 2.0 - 52.0),
        (0.0, -(STATION_Y / 2.0 - 52.0)),
        (-(STATION_X / 2.0 - 58.0), 0.0),
        (STATION_X / 2.0 - 58.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        slots = slots
            + centered_cylinder(
                format!("{PREFIX}_m6_clearance_hole_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 4.0,
                28,
            )
            .translate(*x, *y, 0.0)
            + centered_cube(
                format!("{PREFIX}_m6_slot_relief_{i}"),
                25.0,
                MOUNT_HOLE_D + 0.4,
                BASE_Z + 4.0,
            )
            .translate(*x, *y, 0.0);
    }
    slots
}

fn datum_pin_holes() -> Part {
    let mut holes = Part::empty(format!("{PREFIX}_datum_pin_holes"));
    for (i, (x, y)) in [
        (-540.0, 324.0),
        (540.0, 324.0),
        (-540.0, -324.0),
        (540.0, -324.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("{PREFIX}_datum_pin_clearance_{i}"),
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
        format!("{PREFIX}_left_containment_rim"),
        RIM_W,
        STATION_Y - 58.0,
        RIM_Z,
    )
    .translate(
        -(STATION_X / 2.0 - RIM_W / 2.0),
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(
        format!("{PREFIX}_right_containment_rim"),
        RIM_W,
        STATION_Y - 58.0,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let rear = centered_cube(
        format!("{PREFIX}_rear_containment_rim"),
        STATION_X - 38.0,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let front_low_lip = centered_cube(
        format!("{PREFIX}_front_low_waste_lip"),
        STATION_X - 180.0,
        14.0,
        22.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 27.0, BASE_Z / 2.0 + 11.0);

    left + right + rear + front_low_lip
}

fn zone_dividers() -> Part {
    let upper_to_event = centered_cube(
        format!("{PREFIX}_filter_bypass_to_relief_divider"),
        STATION_X - 170.0,
        12.0,
        30.0,
    )
    .translate(0.0, 82.0, BASE_Z / 2.0 + 15.0);
    let event_to_status = centered_cube(
        format!("{PREFIX}_relief_to_disposition_divider"),
        STATION_X - 170.0,
        12.0,
        30.0,
    )
    .translate(0.0, -168.0, BASE_Z / 2.0 + 15.0);
    let nest_to_bypass = centered_cube(
        format!("{PREFIX}_filter_nest_to_bypass_divider"),
        10.0,
        236.0,
        28.0,
    )
    .translate(-62.0, 208.0, BASE_Z / 2.0 + 14.0);
    let relief_to_pressure = centered_cube(
        format!("{PREFIX}_relief_to_pressure_divider"),
        10.0,
        184.0,
        28.0,
    )
    .translate(-188.0, -36.0, BASE_Z / 2.0 + 14.0);
    let pressure_to_windows = centered_cube(
        format!("{PREFIX}_pressure_to_window_divider"),
        10.0,
        184.0,
        28.0,
    )
    .translate(222.0, -36.0, BASE_Z / 2.0 + 14.0);
    let trace_to_lanes = centered_cube(
        format!("{PREFIX}_traceability_to_disposition_divider"),
        10.0,
        154.0,
        28.0,
    )
    .translate(232.0, -268.0, BASE_Z / 2.0 + 14.0);

    upper_to_event
        + event_to_status
        + nest_to_bypass
        + relief_to_pressure
        + pressure_to_windows
        + trace_to_lanes
}

fn leak_witness_rails() -> Part {
    let mut rails = Part::empty(format!("{PREFIX}_base_leak_witness_rails"));
    for i in 0..LEAK_WITNESS_RAILS {
        let x = centered_index(i, LEAK_WITNESS_RAILS, 174.0);
        rails = rails
            + centered_cube(
                format!("{PREFIX}_base_front_leak_witness_rail_{i}"),
                120.0,
                5.0,
                7.0,
            )
            .translate(x, -342.0, BASE_Z / 2.0 + 3.5);
    }
    rails
}

fn station_flow_gutters() -> Part {
    let bypass_to_waste = centered_cube(
        format!("{PREFIX}_base_bypass_to_waste_gutter"),
        8.0,
        396.0,
        6.0,
    )
    .translate(6.0, -38.0, BASE_Z / 2.0 + 3.0);
    let relief_to_waste = centered_cube(
        format!("{PREFIX}_base_relief_to_waste_gutter"),
        320.0,
        8.0,
        6.0,
    )
    .translate(-260.0, -168.0, BASE_Z / 2.0 + 3.0);
    let window_to_waste = centered_cube(
        format!("{PREFIX}_base_window_to_waste_gutter"),
        352.0,
        8.0,
        6.0,
    )
    .translate(268.0, -168.0, BASE_Z / 2.0 + 3.0);

    bypass_to_waste + relief_to_waste + window_to_waste
}

fn robot_fiducials() -> Part {
    let mut fiducials = Part::empty(format!("{PREFIX}_robot_fiducials"));
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 78.0), STATION_Y / 2.0 - 82.0),
        (STATION_X / 2.0 - 78.0, STATION_Y / 2.0 - 82.0),
        (-(STATION_X / 2.0 - 78.0), -(STATION_Y / 2.0 - 82.0)),
        (STATION_X / 2.0 - 78.0, -(STATION_Y / 2.0 - 82.0)),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + fiducial_disc(format!("{PREFIX}_robot_fiducial_{i}")).translate(
                *x,
                *y,
                BASE_Z / 2.0 + 3.0,
            );
    }
    fiducials
}

fn fiducial_disc(name: impl Into<String>) -> Part {
    let name = name.into();
    let disc = centered_cylinder(format!("{name}_disc"), 13.0, 4.0, 40);
    let cross_x = centered_cube(format!("{name}_cross_x"), 22.0, 3.0, 5.0);
    let cross_y = centered_cube(format!("{name}_cross_y"), 3.0, 22.0, 5.0);
    disc + cross_x + cross_y
}

fn filter_cartridge_nests() -> Part {
    let tray = centered_cube(
        format!("{PREFIX}_filter_cartridge_nest_block"),
        FILTER_NEST_X,
        FILTER_NEST_Y,
        FILTER_NEST_Z,
    );
    let lightening = centered_cube(
        format!("{PREFIX}_filter_nest_underbody_lightening_pocket"),
        FILTER_NEST_X - 58.0,
        FILTER_NEST_Y - 48.0,
        18.0,
    )
    .translate(0.0, 0.0, -FILTER_NEST_Z / 2.0 + 9.0);

    tray - lightening - cartridge_recesses() - cartridge_tube_bores()
        + cartridge_v_rails()
        + cartridge_end_clamps()
        + nest_key_tabs()
        + filter_drip_lands()
        + nest_barcode_index_ticks()
}

fn cartridge_recesses() -> Part {
    let mut recesses = Part::empty(format!("{PREFIX}_filter_cartridge_recesses"));
    for row in 0..FILTER_ROWS {
        for col in 0..FILTER_COLS {
            let idx = row * FILTER_COLS + col;
            let (x, y) = filter_center(row, col);
            let slot = centered_cube(
                format!("{PREFIX}_filter_cartridge_slot_{idx}"),
                FILTER_SLOT_X,
                FILTER_SLOT_Y,
                FILTER_SLOT_Z,
            )
            .translate(x, y, FILTER_NEST_Z / 2.0 - FILTER_SLOT_Z / 2.0 + 0.2);
            let barrel = centered_cylinder(
                format!("{PREFIX}_filter_cartridge_round_cradle_{idx}"),
                FILTER_BODY_D / 2.0,
                FILTER_SLOT_X + 8.0,
                36,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(x, y, FILTER_NEST_Z / 2.0 - 17.0);
            recesses = recesses + slot + barrel;
        }
    }
    recesses
}

fn cartridge_tube_bores() -> Part {
    let mut bores = Part::empty(format!("{PREFIX}_filter_cartridge_tube_bores"));
    for row in 0..FILTER_ROWS {
        for col in 0..FILTER_COLS {
            let idx = row * FILTER_COLS + col;
            let (x, y) = filter_center(row, col);
            let upstream = centered_cylinder(
                format!("{PREFIX}_filter_upstream_tube_bore_{idx}"),
                FILTER_TUBE_BORE_D / 2.0,
                42.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(
                x - FILTER_SLOT_X / 2.0 - 12.0,
                y,
                FILTER_NEST_Z / 2.0 - 14.0,
            );
            let downstream = centered_cylinder(
                format!("{PREFIX}_filter_downstream_tube_bore_{idx}"),
                FILTER_TUBE_BORE_D / 2.0,
                42.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(
                x + FILTER_SLOT_X / 2.0 + 12.0,
                y,
                FILTER_NEST_Z / 2.0 - 14.0,
            );
            bores = bores + upstream + downstream;
        }
    }
    bores
}

fn cartridge_v_rails() -> Part {
    let mut rails = Part::empty(format!("{PREFIX}_filter_cartridge_v_rails"));
    for row in 0..FILTER_ROWS {
        for col in 0..FILTER_COLS {
            let idx = row * FILTER_COLS + col;
            let (x, y) = filter_center(row, col);
            rails = rails
                + centered_cube(
                    format!("{PREFIX}_filter_lower_v_rail_left_{idx}"),
                    FILTER_SLOT_X + 10.0,
                    5.0,
                    9.0,
                )
                .translate(
                    x,
                    y - FILTER_SLOT_Y / 2.0 - 8.0,
                    FILTER_NEST_Z / 2.0 + 4.5,
                )
                + centered_cube(
                    format!("{PREFIX}_filter_lower_v_rail_right_{idx}"),
                    FILTER_SLOT_X + 10.0,
                    5.0,
                    9.0,
                )
                .translate(
                    x,
                    y + FILTER_SLOT_Y / 2.0 + 8.0,
                    FILTER_NEST_Z / 2.0 + 4.5,
                );
        }
    }
    rails
}

fn cartridge_end_clamps() -> Part {
    let mut clamps = Part::empty(format!("{PREFIX}_filter_cartridge_end_clamps"));
    for row in 0..FILTER_ROWS {
        for col in 0..FILTER_COLS {
            let idx = row * FILTER_COLS + col;
            let (x, y) = filter_center(row, col);
            clamps = clamps
                + centered_cube(
                    format!("{PREFIX}_filter_upstream_clamp_finger_{idx}"),
                    10.0,
                    FILTER_SLOT_Y + 20.0,
                    20.0,
                )
                .translate(
                    x - FILTER_SLOT_X / 2.0 - 8.0,
                    y,
                    FILTER_NEST_Z / 2.0 + 10.0,
                )
                + centered_cube(
                    format!("{PREFIX}_filter_downstream_clamp_finger_{idx}"),
                    10.0,
                    FILTER_SLOT_Y + 20.0,
                    20.0,
                )
                .translate(
                    x + FILTER_SLOT_X / 2.0 + 8.0,
                    y,
                    FILTER_NEST_Z / 2.0 + 10.0,
                );
        }
    }
    clamps
}

fn nest_key_tabs() -> Part {
    let mut tabs = Part::empty(format!("{PREFIX}_filter_nest_key_tabs"));
    for col in 0..FILTER_COLS {
        let x = centered_index(col, FILTER_COLS, FILTER_PITCH_X);
        tabs =
            tabs + centered_cube(
                format!("{PREFIX}_filter_column_key_tab_{col}"),
                18.0,
                12.0,
                12.0,
            )
            .translate(
                x - 28.0,
                FILTER_NEST_Y / 2.0 - 18.0,
                FILTER_NEST_Z / 2.0 + 6.0,
            ) + centered_cube(
                format!("{PREFIX}_filter_column_orientation_notch_{col}"),
                10.0,
                12.0,
                13.0,
            )
            .translate(
                x + 28.0,
                -(FILTER_NEST_Y / 2.0 - 18.0),
                FILTER_NEST_Z / 2.0 + 6.5,
            );
    }
    tabs
}

fn filter_drip_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_filter_drip_lands"));
    for row in 0..FILTER_ROWS {
        for col in 0..FILTER_COLS {
            let idx = row * FILTER_COLS + col;
            let (x, y) = filter_center(row, col);
            lands = lands
                + centered_cube(
                    format!("{PREFIX}_filter_drip_shadow_land_{idx}"),
                    FILTER_SLOT_X + 28.0,
                    FILTER_SLOT_Y + 22.0,
                    3.0,
                )
                .translate(x, y, FILTER_NEST_Z / 2.0 + 1.5);
        }
    }
    lands
}

fn nest_barcode_index_ticks() -> Part {
    let mut ticks = Part::empty(format!("{PREFIX}_filter_nest_barcode_index_ticks"));
    for i in 0..FILTER_COUNT {
        let row = i / FILTER_COLS;
        let col = i % FILTER_COLS;
        let (x, y) = filter_center(row, col);
        ticks = ticks
            + csg_label_plaque(
                format!("{PREFIX}_filter_nest_csg_index_label_{i}"),
                34.0,
                14.0,
                4.0,
                i,
            )
            .translate(x, y - 31.0, FILTER_NEST_Z / 2.0 + 4.5);
    }
    ticks
}

fn bypass_loop_witness_channels() -> Part {
    let plate = centered_cube(
        format!("{PREFIX}_bypass_loop_witness_channel_plate"),
        BYPASS_X,
        BYPASS_Y,
        BYPASS_Z,
    );
    let underside_relief = centered_cube(
        format!("{PREFIX}_bypass_plate_underside_lightening_pocket"),
        BYPASS_X - 54.0,
        BYPASS_Y - 46.0,
        14.0,
    )
    .translate(0.0, 0.0, -BYPASS_Z / 2.0 + 7.0);

    plate - underside_relief - bypass_channel_recesses()
        + bypass_channel_rims()
        + bypass_reference_loop_bridges()
        + bypass_flow_arrows()
        + bypass_event_flags()
}

fn bypass_channel_recesses() -> Part {
    let mut channels = Part::empty(format!("{PREFIX}_bypass_loop_recesses"));
    for i in 0..BYPASS_CHANNELS {
        let x = centered_index(i, BYPASS_CHANNELS, BYPASS_PITCH_X);
        let inlet = centered_cube(
            format!("{PREFIX}_bypass_inlet_channel_recess_{i}"),
            BYPASS_CHANNEL_W,
            BYPASS_Y - 62.0,
            BYPASS_CHANNEL_Z,
        )
        .translate(x, 0.0, BYPASS_Z / 2.0 - BYPASS_CHANNEL_Z / 2.0 + 0.2);
        let cross = centered_cube(
            format!("{PREFIX}_bypass_cross_loop_recess_{i}"),
            40.0,
            BYPASS_CHANNEL_W,
            BYPASS_CHANNEL_Z,
        )
        .translate(
            x + 18.0,
            BYPASS_Y / 2.0 - 48.0,
            BYPASS_Z / 2.0 - BYPASS_CHANNEL_Z / 2.0 + 0.2,
        );
        let witness_pool = centered_cylinder(
            format!("{PREFIX}_bypass_visible_witness_pool_recess_{i}"),
            13.0,
            BYPASS_CHANNEL_Z + 2.0,
            32,
        )
        .translate(
            x,
            -BYPASS_Y / 2.0 + 52.0,
            BYPASS_Z / 2.0 - BYPASS_CHANNEL_Z / 2.0,
        );
        channels = channels + inlet + cross + witness_pool;
    }
    channels
}

fn bypass_channel_rims() -> Part {
    let mut rims = Part::empty(format!("{PREFIX}_bypass_loop_raised_rims"));
    for i in 0..BYPASS_CHANNELS {
        let x = centered_index(i, BYPASS_CHANNELS, BYPASS_PITCH_X);
        rims = rims
            + centered_cube(
                format!("{PREFIX}_bypass_channel_left_rim_{i}"),
                4.0,
                BYPASS_Y - 56.0,
                8.0,
            )
            .translate(x - 9.0, 0.0, BYPASS_Z / 2.0 + 4.0)
            + centered_cube(
                format!("{PREFIX}_bypass_channel_right_rim_{i}"),
                4.0,
                BYPASS_Y - 56.0,
                8.0,
            )
            .translate(x + 9.0, 0.0, BYPASS_Z / 2.0 + 4.0);
    }
    rims
}

fn bypass_reference_loop_bridges() -> Part {
    let upper_bridge = centered_cube(
        format!("{PREFIX}_bypass_upper_reference_loop_bridge"),
        BYPASS_X - 72.0,
        16.0,
        14.0,
    )
    .translate(0.0, BYPASS_Y / 2.0 - 30.0, BYPASS_Z / 2.0 + 7.0);
    let lower_bridge = centered_cube(
        format!("{PREFIX}_bypass_lower_reference_loop_bridge"),
        BYPASS_X - 72.0,
        16.0,
        14.0,
    )
    .translate(0.0, -(BYPASS_Y / 2.0 - 30.0), BYPASS_Z / 2.0 + 7.0);
    let drain_bridge = centered_cube(
        format!("{PREFIX}_bypass_relief_drain_bridge"),
        18.0,
        BYPASS_Y - 62.0,
        14.0,
    )
    .translate(BYPASS_X / 2.0 - 54.0, 0.0, BYPASS_Z / 2.0 + 7.0);

    upper_bridge + lower_bridge + drain_bridge
}

fn bypass_flow_arrows() -> Part {
    let mut arrows = Part::empty(format!("{PREFIX}_bypass_flow_direction_arrows"));
    for i in 0..FLOW_ARROW_COUNT {
        let x = centered_index(i, FLOW_ARROW_COUNT, 54.0);
        arrows = arrows
            + flow_arrow(format!("{PREFIX}_bypass_flow_arrow_{i}"), 28.0, 16.0, 5.0)
                .rotate(0.0, 0.0, if i % 2 == 0 { 90.0 } else { -90.0 })
                .translate(x, 0.0, BYPASS_Z / 2.0 + 4.0);
    }
    arrows
}

fn bypass_event_flags() -> Part {
    let mut flags = Part::empty(format!("{PREFIX}_bypass_event_flags"));
    for i in 0..BYPASS_CHANNELS {
        let x = centered_index(i, BYPASS_CHANNELS, BYPASS_PITCH_X);
        flags = flags
            + csg_label_plaque(
                format!("{PREFIX}_bypass_channel_csg_event_flag_{i}"),
                34.0,
                16.0,
                4.0,
                10 + i,
            )
            .translate(x, BYPASS_Y / 2.0 - 16.0, BYPASS_Z / 2.0 + 4.0);
    }
    flags
}

fn relief_valve_surrogate_ports() -> Part {
    let block = centered_cube(
        format!("{PREFIX}_relief_valve_surrogate_port_block"),
        RELIEF_X,
        RELIEF_Y,
        RELIEF_Z,
    );
    let rear_manifold_pocket = centered_cube(
        format!("{PREFIX}_relief_rear_manifold_pocket"),
        RELIEF_X - 48.0,
        24.0,
        20.0,
    )
    .translate(0.0, RELIEF_Y / 2.0 - 32.0, RELIEF_Z / 2.0 - 12.0);
    let front_spill_shelf = centered_cube(
        format!("{PREFIX}_relief_front_spill_shelf_recess"),
        RELIEF_X - 54.0,
        28.0,
        12.0,
    )
    .translate(0.0, -(RELIEF_Y / 2.0 - 28.0), RELIEF_Z / 2.0 - 8.0);

    block - rear_manifold_pocket - front_spill_shelf - relief_port_bores()
        + relief_port_bosses()
        + relief_header_tubes()
        + cracking_pressure_tokens()
        + relief_latch_tabs()
}

fn relief_port_bosses() -> Part {
    let mut bosses = Part::empty(format!("{PREFIX}_relief_port_bosses"));
    for i in 0..RELIEF_PORTS {
        let x = centered_index(i, RELIEF_PORTS, RELIEF_PORT_PITCH_X);
        bosses = bosses
            + centered_cylinder(
                format!("{PREFIX}_relief_surrogate_port_boss_{i}"),
                RELIEF_PORT_D / 2.0,
                18.0,
                36,
            )
            .translate(x, -12.0, RELIEF_Z / 2.0 + 9.0)
            + centered_cube(
                format!("{PREFIX}_relief_port_anti_rotation_flat_{i}"),
                22.0,
                8.0,
                16.0,
            )
            .translate(x, 15.0, RELIEF_Z / 2.0 + 8.0);
    }
    bosses
}

fn relief_port_bores() -> Part {
    let mut bores = Part::empty(format!("{PREFIX}_relief_port_bores"));
    for i in 0..RELIEF_PORTS {
        let x = centered_index(i, RELIEF_PORTS, RELIEF_PORT_PITCH_X);
        bores = bores
            + centered_cylinder(
                format!("{PREFIX}_relief_surrogate_port_vertical_bore_{i}"),
                RELIEF_BORE_D / 2.0,
                RELIEF_Z + 28.0,
                28,
            )
            .translate(x, -12.0, 4.0)
            + centered_cylinder(
                format!("{PREFIX}_relief_surrogate_side_exhaust_bore_{i}"),
                RELIEF_BORE_D / 2.0,
                58.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, RELIEF_Y / 2.0 - 18.0, RELIEF_Z / 2.0 - 12.0);
    }
    bores
}

fn relief_header_tubes() -> Part {
    let header = centered_cylinder(
        format!("{PREFIX}_relief_common_exhaust_header"),
        RELIEF_HEADER_D / 2.0,
        RELIEF_X - 56.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, RELIEF_Y / 2.0 - 28.0, RELIEF_Z / 2.0 + 8.0);
    let drain = centered_cylinder(
        format!("{PREFIX}_relief_event_drain_stub"),
        RELIEF_HEADER_D / 2.0,
        64.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        RELIEF_X / 2.0 - 44.0,
        -(RELIEF_Y / 2.0 + 8.0),
        RELIEF_Z / 2.0 + 8.0,
    );
    let inlet_reference = centered_cylinder(
        format!("{PREFIX}_relief_reference_pressure_inlet_stub"),
        RELIEF_HEADER_D / 2.0,
        64.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        -(RELIEF_X / 2.0 - 44.0),
        RELIEF_Y / 2.0 + 8.0,
        RELIEF_Z / 2.0 + 8.0,
    );

    header + drain + inlet_reference
}

fn cracking_pressure_tokens() -> Part {
    let mut tokens = Part::empty(format!("{PREFIX}_relief_cracking_pressure_tokens"));
    for i in 0..RELIEF_PORTS {
        let x = centered_index(i, RELIEF_PORTS, RELIEF_PORT_PITCH_X);
        let height = 8.0 + i as f64 * 2.0;
        tokens = tokens
            + centered_cube(
                format!("{PREFIX}_relief_crack_reference_token_{i}"),
                20.0,
                14.0,
                height,
            )
            .translate(x, -(RELIEF_Y / 2.0 - 24.0), RELIEF_Z / 2.0 + height / 2.0);
    }
    tokens
}

fn relief_latch_tabs() -> Part {
    let left = centered_cube(
        format!("{PREFIX}_relief_left_module_latch_tab"),
        18.0,
        48.0,
        18.0,
    )
    .translate(-(RELIEF_X / 2.0 - 16.0), 0.0, RELIEF_Z / 2.0 + 9.0);
    let right = centered_cube(
        format!("{PREFIX}_relief_right_module_latch_tab"),
        18.0,
        48.0,
        18.0,
    )
    .translate(RELIEF_X / 2.0 - 16.0, 0.0, RELIEF_Z / 2.0 + 9.0);

    left + right
}

fn pressure_sensor_pockets() -> Part {
    let bar = centered_cube(
        format!("{PREFIX}_pressure_sensor_pocket_bar"),
        PRESSURE_X,
        PRESSURE_Y,
        PRESSURE_Z,
    );
    let cable_raceway = centered_cube(
        format!("{PREFIX}_pressure_sensor_rear_cable_raceway"),
        PRESSURE_X - 42.0,
        20.0,
        18.0,
    )
    .translate(0.0, PRESSURE_Y / 2.0 - 26.0, PRESSURE_Z / 2.0 - 10.0);

    bar - cable_raceway - sensor_pocket_recesses() - sensor_tube_bores()
        + sensor_retainer_clips()
        + sensor_cable_comb()
        + sensor_mapping_tokens()
}

fn sensor_pocket_recesses() -> Part {
    let mut pockets = Part::empty(format!("{PREFIX}_pressure_sensor_recesses"));
    for bank in 0..PRESSURE_BANKS {
        for col in 0..PRESSURE_COLUMNS {
            let idx = bank * PRESSURE_COLUMNS + col;
            let x = centered_index(col, PRESSURE_COLUMNS, SENSOR_PITCH_X);
            let y = centered_index(bank, PRESSURE_BANKS, SENSOR_BANK_PITCH_Y);
            pockets = pockets
                + centered_cube(
                    format!("{PREFIX}_pressure_sensor_rectangular_pocket_{idx}"),
                    SENSOR_POCKET_X,
                    SENSOR_POCKET_Y,
                    SENSOR_POCKET_Z,
                )
                .translate(x, y, PRESSURE_Z / 2.0 - SENSOR_POCKET_Z / 2.0 + 0.2)
                + centered_cylinder(
                    format!("{PREFIX}_pressure_sensor_o_ring_groove_{idx}"),
                    13.0,
                    SENSOR_POCKET_Z + 2.0,
                    32,
                )
                .translate(x, y, PRESSURE_Z / 2.0 - SENSOR_POCKET_Z / 2.0);
        }
    }
    pockets
}

fn sensor_tube_bores() -> Part {
    let mut bores = Part::empty(format!("{PREFIX}_pressure_sensor_tube_bores"));
    for bank in 0..PRESSURE_BANKS {
        for col in 0..PRESSURE_COLUMNS {
            let idx = bank * PRESSURE_COLUMNS + col;
            let x = centered_index(col, PRESSURE_COLUMNS, SENSOR_PITCH_X);
            let y = centered_index(bank, PRESSURE_BANKS, SENSOR_BANK_PITCH_Y);
            bores = bores
                + centered_cylinder(
                    format!("{PREFIX}_pressure_sensor_tube_bore_{idx}"),
                    3.3,
                    52.0,
                    22,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(x, y + 24.0, PRESSURE_Z / 2.0 - 18.0);
        }
    }
    bores
}

fn sensor_retainer_clips() -> Part {
    let mut clips = Part::empty(format!("{PREFIX}_pressure_sensor_retainer_clips"));
    for bank in 0..PRESSURE_BANKS {
        for col in 0..PRESSURE_COLUMNS {
            let idx = bank * PRESSURE_COLUMNS + col;
            let x = centered_index(col, PRESSURE_COLUMNS, SENSOR_PITCH_X);
            let y = centered_index(bank, PRESSURE_BANKS, SENSOR_BANK_PITCH_Y);
            clips = clips
                + centered_cube(
                    format!("{PREFIX}_pressure_sensor_left_clip_{idx}"),
                    5.0,
                    SENSOR_POCKET_Y + 12.0,
                    10.0,
                )
                .translate(
                    x - SENSOR_POCKET_X / 2.0 - 5.0,
                    y,
                    PRESSURE_Z / 2.0 + 5.0,
                )
                + centered_cube(
                    format!("{PREFIX}_pressure_sensor_right_clip_{idx}"),
                    5.0,
                    SENSOR_POCKET_Y + 12.0,
                    10.0,
                )
                .translate(
                    x + SENSOR_POCKET_X / 2.0 + 5.0,
                    y,
                    PRESSURE_Z / 2.0 + 5.0,
                );
        }
    }
    clips
}

fn sensor_cable_comb() -> Part {
    let mut comb = Part::empty(format!("{PREFIX}_pressure_sensor_cable_comb"));
    for col in 0..PRESSURE_COLUMNS {
        let x = centered_index(col, PRESSURE_COLUMNS, SENSOR_PITCH_X);
        comb = comb
            + centered_cube(
                format!("{PREFIX}_pressure_sensor_cable_comb_tooth_{col}"),
                6.0,
                36.0,
                16.0,
            )
            .translate(x, PRESSURE_Y / 2.0 - 18.0, PRESSURE_Z / 2.0 + 8.0);
    }
    comb
}

fn sensor_mapping_tokens() -> Part {
    let upstream = csg_label_plaque(
        format!("{PREFIX}_pressure_upstream_bank_csg_label"),
        126.0,
        18.0,
        4.0,
        24,
    )
    .translate(-126.0, -PRESSURE_Y / 2.0 + 18.0, PRESSURE_Z / 2.0 + 4.0);
    let downstream = csg_label_plaque(
        format!("{PREFIX}_pressure_downstream_bank_csg_label"),
        126.0,
        18.0,
        4.0,
        25,
    )
    .translate(126.0, -PRESSURE_Y / 2.0 + 18.0, PRESSURE_Z / 2.0 + 4.0);

    upstream + downstream
}

fn bubble_wetness_windows() -> Part {
    let frame = centered_cube(
        format!("{PREFIX}_bubble_wetness_window_frame"),
        WINDOW_X,
        WINDOW_Y,
        WINDOW_Z,
    );
    let underside_relief = centered_cube(
        format!("{PREFIX}_bubble_window_underside_relief"),
        WINDOW_X - 42.0,
        WINDOW_Y - 36.0,
        10.0,
    )
    .translate(0.0, 0.0, -WINDOW_Z / 2.0 + 5.0);

    frame - underside_relief - bubble_window_openings()
        + wetness_indicator_pads()
        + bubble_trap_domelike_witnesses()
        + window_light_baffles()
}

fn bubble_window_openings() -> Part {
    let mut openings = Part::empty(format!("{PREFIX}_bubble_window_openings"));
    for i in 0..WINDOW_COUNT {
        let x = centered_index(i, WINDOW_COUNT, WINDOW_PITCH_X);
        openings = openings
            + centered_cube(
                format!("{PREFIX}_bubble_witness_rect_window_{i}"),
                42.0,
                72.0,
                WINDOW_Z + 4.0,
            )
            .translate(x, 18.0, 0.0)
            + centered_cylinder(
                format!("{PREFIX}_bubble_witness_upper_radius_{i}"),
                21.0,
                WINDOW_Z + 4.0,
                32,
            )
            .translate(x, 54.0, 0.0)
            + centered_cylinder(
                format!("{PREFIX}_bubble_witness_lower_radius_{i}"),
                21.0,
                WINDOW_Z + 4.0,
                32,
            )
            .translate(x, -18.0, 0.0);
    }
    openings
}

fn wetness_indicator_pads() -> Part {
    let mut pads = Part::empty(format!("{PREFIX}_wetness_indicator_pads"));
    for i in 0..WETNESS_PAD_COUNT {
        let col = i % FILTER_COLS;
        let row = i / FILTER_COLS;
        let x = centered_index(col, FILTER_COLS, 54.0);
        let y = if row == 0 { -70.0 } else { -92.0 };
        pads =
            pads + centered_cube(format!("{PREFIX}_wetness_pad_land_{i}"), 42.0, 12.0, 5.0)
                .translate(x, y, WINDOW_Z / 2.0 + 2.5)
                + centered_cube(
                    format!("{PREFIX}_wetness_pad_raised_tick_{i}"),
                    4.0,
                    16.0,
                    7.0,
                )
                .translate(x - 15.0, y, WINDOW_Z / 2.0 + 3.5);
    }
    pads
}

fn bubble_trap_domelike_witnesses() -> Part {
    let mut domes = Part::empty(format!("{PREFIX}_bubble_trap_surrogate_domelike_witnesses"));
    for i in 0..WINDOW_COUNT {
        let x = centered_index(i, WINDOW_COUNT, WINDOW_PITCH_X);
        domes = domes
            + centered_cylinder(
                format!("{PREFIX}_bubble_witness_dome_ring_{i}"),
                BUBBLE_DOME_D / 2.0,
                7.0,
                40,
            )
            .translate(x, 72.0, WINDOW_Z / 2.0 + 3.5)
            - centered_cylinder(
                format!("{PREFIX}_bubble_witness_dome_sight_clearance_{i}"),
                BUBBLE_DOME_D / 2.0 - 4.0,
                8.0,
                40,
            )
            .translate(x, 72.0, WINDOW_Z / 2.0 + 3.5);
    }
    domes
}

fn window_light_baffles() -> Part {
    let mut baffles = Part::empty(format!("{PREFIX}_window_light_baffles"));
    for i in 0..=WINDOW_COUNT {
        let x = centered_index(i, WINDOW_COUNT + 1, WINDOW_PITCH_X);
        baffles = baffles
            + centered_cube(
                format!("{PREFIX}_window_light_baffle_{i}"),
                4.0,
                WINDOW_Y - 42.0,
                20.0,
            )
            .translate(x, 12.0, WINDOW_Z / 2.0 + 10.0);
    }
    baffles
}

fn waste_capture_wells() -> Part {
    let tray = centered_cube(
        format!("{PREFIX}_waste_capture_well_tray"),
        WASTE_X,
        WASTE_Y,
        WASTE_Z,
    );
    let wash_slope = centered_cube(
        format!("{PREFIX}_waste_capture_sloped_floor_relief"),
        WASTE_X - 48.0,
        WASTE_Y - 42.0,
        16.0,
    )
    .translate(0.0, 0.0, WASTE_Z / 2.0 - 10.0);

    tray - wash_slope - waste_well_recesses() - waste_drain_bores()
        + waste_well_rims()
        + waste_volume_tick_bars()
        + waste_inlet_weirs()
}

fn waste_well_recesses() -> Part {
    let mut wells = Part::empty(format!("{PREFIX}_waste_capture_well_recesses"));
    for i in 0..WASTE_WELLS {
        let col = i % 4;
        let row = i / 4;
        let x = centered_index(col, 4, WASTE_WELL_PITCH_X);
        let y = centered_index(row, 2, 64.0);
        wells = wells
            + centered_cylinder(
                format!("{PREFIX}_waste_capture_bypass_relief_well_{i}"),
                WASTE_WELL_D / 2.0,
                WASTE_Z + 8.0,
                40,
            )
            .translate(x, y, 8.0);
    }
    wells
}

fn waste_drain_bores() -> Part {
    let left = centered_cylinder(format!("{PREFIX}_waste_left_drain_bore"), 4.5, 62.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(-(WASTE_X / 2.0 - 64.0), -(WASTE_Y / 2.0 + 6.0), -6.0);
    let right = centered_cylinder(format!("{PREFIX}_waste_right_drain_bore"), 4.5, 62.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(WASTE_X / 2.0 - 64.0, -(WASTE_Y / 2.0 + 6.0), -6.0);
    left + right
}

fn waste_well_rims() -> Part {
    let mut rims = Part::empty(format!("{PREFIX}_waste_capture_well_rims"));
    for i in 0..WASTE_WELLS {
        let col = i % 4;
        let row = i / 4;
        let x = centered_index(col, 4, WASTE_WELL_PITCH_X);
        let y = centered_index(row, 2, 64.0);
        rims = rims
            + centered_cylinder(
                format!("{PREFIX}_waste_capture_well_rim_{i}"),
                WASTE_WELL_D / 2.0 + 5.0,
                8.0,
                40,
            )
            .translate(x, y, WASTE_Z / 2.0 + 4.0)
            - centered_cylinder(
                format!("{PREFIX}_waste_capture_well_rim_inner_clearance_{i}"),
                WASTE_WELL_D / 2.0 - 1.0,
                9.0,
                40,
            )
            .translate(x, y, WASTE_Z / 2.0 + 4.0);
    }
    rims
}

fn waste_volume_tick_bars() -> Part {
    let mut ticks = Part::empty(format!("{PREFIX}_waste_volume_tick_bars"));
    for i in 0..WASTE_WELLS {
        let col = i % 4;
        let row = i / 4;
        let x = centered_index(col, 4, WASTE_WELL_PITCH_X) + 26.0;
        let y = centered_index(row, 2, 64.0);
        for tick in 0..3 {
            ticks = ticks
                + centered_cube(
                    format!("{PREFIX}_waste_volume_tick_{i}_{tick}"),
                    12.0 + tick as f64 * 4.0,
                    3.0,
                    5.0,
                )
                .translate(x, y - 12.0 + tick as f64 * 12.0, WASTE_Z / 2.0 + 2.5);
        }
    }
    ticks
}

fn waste_inlet_weirs() -> Part {
    let bypass_weir = centered_cube(
        format!("{PREFIX}_waste_bypass_loop_inlet_weir"),
        WASTE_X - 52.0,
        8.0,
        16.0,
    )
    .translate(0.0, WASTE_Y / 2.0 - 20.0, WASTE_Z / 2.0 + 8.0);
    let relief_weir = centered_cube(
        format!("{PREFIX}_waste_relief_event_inlet_weir"),
        WASTE_X - 52.0,
        8.0,
        16.0,
    )
    .translate(0.0, -(WASTE_Y / 2.0 - 20.0), WASTE_Z / 2.0 + 8.0);
    bypass_weir + relief_weir
}

fn barcode_certificate_lands() -> Part {
    let plate = centered_cube(
        format!("{PREFIX}_barcode_certificate_land_plate"),
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    );

    plate
        + traceability_border_rails()
        + barcode_lands()
        + certificate_lands()
        + traceability_route_arrows()
}

fn traceability_border_rails() -> Part {
    let rear = centered_cube(
        format!("{PREFIX}_traceability_rear_certificate_rail"),
        TRACE_X - 32.0,
        5.0,
        7.0,
    )
    .translate(0.0, TRACE_Y / 2.0 - 12.0, TRACE_Z / 2.0 + 3.5);
    let front = centered_cube(
        format!("{PREFIX}_traceability_front_barcode_rail"),
        TRACE_X - 32.0,
        5.0,
        7.0,
    )
    .translate(0.0, -(TRACE_Y / 2.0 - 12.0), TRACE_Z / 2.0 + 3.5);
    let left = centered_cube(
        format!("{PREFIX}_traceability_left_scan_stop"),
        5.0,
        TRACE_Y - 34.0,
        7.0,
    )
    .translate(-(TRACE_X / 2.0 - 12.0), 0.0, TRACE_Z / 2.0 + 3.5);
    let right = centered_cube(
        format!("{PREFIX}_traceability_right_scan_stop"),
        5.0,
        TRACE_Y - 34.0,
        7.0,
    )
    .translate(TRACE_X / 2.0 - 12.0, 0.0, TRACE_Z / 2.0 + 3.5);

    rear + front + left + right
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_barcode_lands"));
    for i in 0..BARCODE_LANDS {
        let col = i % FILTER_COLS;
        let row = i / FILTER_COLS;
        let x = centered_index(col, FILTER_COLS, 70.0);
        let y = 40.0 - row as f64 * 34.0;
        lands = lands
            + csg_label_plaque(
                format!("{PREFIX}_barcode_land_filter_{i}"),
                54.0,
                22.0,
                4.0,
                40 + i,
            )
            .translate(x, y, TRACE_Z / 2.0 + 2.0);
    }
    lands
}

fn certificate_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_certificate_lands"));
    for i in 0..CERTIFICATE_LANDS {
        let x = centered_index(i, CERTIFICATE_LANDS, 68.0);
        lands = lands
            + certificate_plaque(
                format!("{PREFIX}_certificate_land_{i}"),
                58.0,
                34.0,
                4.0,
                60 + i,
            )
            .translate(x, -TRACE_Y / 2.0 + 28.0, TRACE_Z / 2.0 + 2.0);
    }
    lands
}

fn traceability_route_arrows() -> Part {
    let left = flow_arrow(
        format!("{PREFIX}_traceability_to_release_arrow"),
        34.0,
        18.0,
        4.0,
    )
    .translate(-TRACE_X / 2.0 + 48.0, 0.0, TRACE_Z / 2.0 + 2.0);
    let right = flow_arrow(
        format!("{PREFIX}_traceability_to_hold_arrow"),
        34.0,
        18.0,
        4.0,
    )
    .rotate(0.0, 0.0, 180.0)
    .translate(TRACE_X / 2.0 - 48.0, 0.0, TRACE_Z / 2.0 + 2.0);
    left + right
}

fn release_hold_reject_lanes() -> Part {
    let tray = centered_cube(
        format!("{PREFIX}_release_hold_reject_lane_tray"),
        LANE_X,
        LANE_Y,
        LANE_Z,
    );
    let lane_floor_recess = centered_cube(
        format!("{PREFIX}_release_hold_reject_lane_floor_relief"),
        LANE_X - 38.0,
        LANE_Y - 34.0,
        12.0,
    )
    .translate(0.0, 0.0, LANE_Z / 2.0 - 8.0);

    tray - lane_floor_recess - disposition_slot_recesses()
        + lane_separators()
        + disposition_gate_tabs()
        + lane_label_tokens()
        + lane_status_flags()
}

fn disposition_slot_recesses() -> Part {
    let mut slots = Part::empty(format!("{PREFIX}_disposition_slot_recesses"));
    for lane in 0..DISPOSITION_LANES {
        for slot in 0..LANE_SLOTS_PER_LANE {
            let x = centered_index(slot, LANE_SLOTS_PER_LANE, 66.0);
            let y = centered_index(lane, DISPOSITION_LANES, 48.0);
            slots = slots
                + centered_cube(
                    format!("{PREFIX}_disposition_lane_{lane}_slot_{slot}"),
                    LANE_SLOT_X,
                    LANE_SLOT_Y,
                    18.0,
                )
                .translate(x, y, LANE_Z / 2.0 - 9.0 + 0.2);
        }
    }
    slots
}

fn lane_separators() -> Part {
    let mut ribs = Part::empty(format!("{PREFIX}_disposition_lane_separators"));
    for i in 0..=DISPOSITION_LANES {
        let y = -LANE_Y / 2.0 + 18.0 + i as f64 * 42.0;
        ribs = ribs
            + centered_cube(
                format!("{PREFIX}_disposition_lane_separator_{i}"),
                LANE_X - 38.0,
                5.0,
                22.0,
            )
            .translate(0.0, y, LANE_Z / 2.0 + 11.0);
    }
    ribs
}

fn disposition_gate_tabs() -> Part {
    let mut tabs = Part::empty(format!("{PREFIX}_disposition_gate_tabs"));
    for lane in 0..DISPOSITION_LANES {
        let y = centered_index(lane, DISPOSITION_LANES, 48.0);
        tabs = tabs
            + centered_cube(
                format!("{PREFIX}_disposition_lane_{lane}_front_gate_tab"),
                20.0,
                30.0,
                20.0,
            )
            .translate(-LANE_X / 2.0 + 24.0, y, LANE_Z / 2.0 + 10.0)
            + centered_cube(
                format!("{PREFIX}_disposition_lane_{lane}_rear_gate_tab"),
                20.0,
                30.0,
                20.0,
            )
            .translate(LANE_X / 2.0 - 24.0, y, LANE_Z / 2.0 + 10.0);
    }
    tabs
}

fn lane_label_tokens() -> Part {
    let mut labels = Part::empty(format!("{PREFIX}_release_hold_reject_csg_lane_labels"));
    for (lane, label) in ["release", "hold", "reject"].iter().enumerate() {
        let y = centered_index(lane, DISPOSITION_LANES, 48.0);
        labels = labels
            + csg_label_plaque(
                format!("{PREFIX}_{label}_lane_csg_label"),
                74.0,
                18.0,
                4.0,
                80 + lane,
            )
            .translate(0.0, y, LANE_Z / 2.0 + 4.0);
    }
    labels
}

fn lane_status_flags() -> Part {
    let release = centered_cylinder(
        format!("{PREFIX}_release_lane_round_status_token"),
        9.0,
        6.0,
        32,
    )
    .translate(
        -LANE_X / 2.0 + 58.0,
        centered_index(0, DISPOSITION_LANES, 48.0),
        LANE_Z / 2.0 + 3.0,
    );
    let hold = centered_cube(
        format!("{PREFIX}_hold_lane_square_status_token"),
        16.0,
        16.0,
        6.0,
    )
    .translate(
        -LANE_X / 2.0 + 58.0,
        centered_index(1, DISPOSITION_LANES, 48.0),
        LANE_Z / 2.0 + 3.0,
    );
    let reject = centered_cube(
        format!("{PREFIX}_reject_lane_cross_status_token_a"),
        24.0,
        5.0,
        6.0,
    )
    .rotate(0.0, 0.0, 45.0)
    .translate(
        -LANE_X / 2.0 + 58.0,
        centered_index(2, DISPOSITION_LANES, 48.0),
        LANE_Z / 2.0 + 3.0,
    ) + centered_cube(
        format!("{PREFIX}_reject_lane_cross_status_token_b"),
        24.0,
        5.0,
        6.0,
    )
    .rotate(0.0, 0.0, -45.0)
    .translate(
        -LANE_X / 2.0 + 58.0,
        centered_index(2, DISPOSITION_LANES, 48.0),
        LANE_Z / 2.0 + 3.0,
    );

    release + hold + reject
}

fn csg_zone_labels() -> Part {
    let mut labels = Part::empty(format!("{PREFIX}_global_csg_zone_labels"));
    for (i, (name, x, y, w, h)) in [
        ("filter_nest", -488.0, 332.0, 112.0, 22.0),
        ("bypass_loop", 110.0, 332.0, 112.0, 22.0),
        ("relief_ports", -486.0, 70.0, 112.0, 22.0),
        ("pressure_map", -70.0, 70.0, 116.0, 22.0),
        ("bubble_wet", 316.0, 70.0, 110.0, 22.0),
        ("waste", -488.0, -184.0, 92.0, 22.0),
        ("barcode_cert", -84.0, -184.0, 112.0, 22.0),
        ("disposition", 300.0, -184.0, 116.0, 22.0),
        ("closed_path", 0.0, 356.0, 148.0, 24.0),
    ]
    .iter()
    .enumerate()
    {
        labels = labels
            + csg_label_plaque(
                format!("{PREFIX}_zone_label_{name}_{i}"),
                *w,
                *h,
                4.0,
                100 + i,
            )
            .translate(*x, *y, 2.0);
    }
    labels + global_route_arrows() + global_status_tokens()
}

fn global_route_arrows() -> Part {
    let filter_to_bypass = flow_arrow(
        format!("{PREFIX}_global_filter_to_bypass_arrow"),
        42.0,
        20.0,
        5.0,
    )
    .translate(-52.0, 208.0, 2.5);
    let bypass_to_relief = flow_arrow(
        format!("{PREFIX}_global_bypass_to_relief_arrow"),
        42.0,
        20.0,
        5.0,
    )
    .rotate(0.0, 0.0, -90.0)
    .translate(164.0, 82.0, 2.5);
    let relief_to_waste = flow_arrow(
        format!("{PREFIX}_global_relief_to_waste_arrow"),
        42.0,
        20.0,
        5.0,
    )
    .rotate(0.0, 0.0, -90.0)
    .translate(-346.0, -168.0, 2.5);
    filter_to_bypass + bypass_to_relief + relief_to_waste
}

fn global_status_tokens() -> Part {
    let mut tokens = Part::empty(format!("{PREFIX}_global_status_tokens"));
    for i in 0..GLOBAL_LABEL_COUNT {
        let x = centered_index(i, GLOBAL_LABEL_COUNT, 56.0);
        let token = if i % 3 == 0 {
            centered_cylinder(
                format!("{PREFIX}_global_round_status_token_{i}"),
                7.0,
                5.0,
                24,
            )
        } else {
            centered_cube(
                format!("{PREFIX}_global_rect_status_token_{i}"),
                14.0,
                9.0,
                5.0,
            )
        };
        tokens = tokens + token.translate(x, -356.0, 2.5);
    }
    tokens
}

fn flow_arrow(name: impl Into<String>, length: f64, width: f64, height: f64) -> Part {
    let name = name.into();
    let shaft = centered_cube(format!("{name}_shaft"), length * 0.58, width * 0.28, height)
        .translate(-length * 0.13, 0.0, 0.0);
    let head = centered_cube(format!("{name}_head"), width * 0.58, width * 0.58, height)
        .rotate(0.0, 0.0, 45.0)
        .translate(length * 0.28, 0.0, 0.0);
    shaft + head
}

fn csg_label_plaque(name: impl Into<String>, x: f64, y: f64, z: f64, seed: usize) -> Part {
    let name = name.into();
    let base = centered_cube(format!("{name}_base"), x, y, z);
    let mut bars = Part::empty(format!("{name}_raised_bars"));
    for i in 0..LABEL_BAR_COUNT {
        let width = 2.0 + ((seed + i) % 4) as f64 * 1.6;
        let height = (y - 7.0 - (i % 3) as f64 * 2.0).max(3.0);
        let x_offset = -x / 2.0 + 9.0 + i as f64 * ((x - 20.0) / LABEL_BAR_COUNT as f64);
        bars =
            bars + centered_cube(format!("{name}_raised_bar_{i}"), width, height, z + 1.6)
                .translate(x_offset, 0.0, z / 2.0 + 0.8);
    }
    let orientation_tab = centered_cube(format!("{name}_orientation_tab"), 12.0, 4.0, z + 1.8)
        .translate(x / 2.0 - 11.0, y / 2.0 - 5.0, z / 2.0 + 0.9);
    base + bars + orientation_tab
}

fn certificate_plaque(name: impl Into<String>, x: f64, y: f64, z: f64, seed: usize) -> Part {
    let name = name.into();
    let sheet = centered_cube(format!("{name}_sheet"), x, y, z);
    let barcode = csg_label_plaque(format!("{name}_barcode"), x - 12.0, 10.0, z + 0.8, seed)
        .translate(0.0, y / 2.0 - 10.0, z / 2.0 + 0.4);
    let signature_line = centered_cube(format!("{name}_signature_line"), x - 14.0, 2.5, z + 1.0)
        .translate(0.0, -y / 2.0 + 9.0, z / 2.0 + 0.5);
    let seal = centered_cylinder(
        format!("{name}_raised_circular_certificate_seal"),
        7.0,
        z + 1.2,
        30,
    )
    .translate(x / 2.0 - 12.0, -y / 2.0 + 11.0, z / 2.0 + 0.6);

    sheet + barcode + signature_line + seal
}
