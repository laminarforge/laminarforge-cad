use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Environmental sensor calibration station for incubator / isolator / cassette monitoring.
//
// Research assumptions captured in geometry:
// - Incubator O2/CO2 checks are serviced through sample ports using independent
//   instruments after the chamber environment is stable.
// - Reference gas delivery is a valved manifold with leak-tested 1/8 inch tube
//   standards and separate zero/span/purge ports.
// - RH checks can use saturated salt fixed-point standards staged in sealed
//   pockets, with used standards physically segregated from clean ones.
//
// This is packaging/fixture CAD only. Calibration intervals, gas traceability,
// acceptance limits, and release records remain validation-system decisions.

const OUTPUTS: [&str; 12] = [
    "output/environmental_sensor_calibration_station_base_tray.stl",
    "output/environmental_sensor_calibration_station_probe_dock_plate.stl",
    "output/environmental_sensor_calibration_station_gas_reference_manifold.stl",
    "output/environmental_sensor_calibration_station_humidity_standard_block.stl",
    "output/environmental_sensor_calibration_station_logger_docking_rack.stl",
    "output/environmental_sensor_calibration_station_flow_pressure_panel.stl",
    "output/environmental_sensor_calibration_station_barcode_certificate_lands.stl",
    "output/environmental_sensor_calibration_station_clean_used_segregation_tray.stl",
    "output/environmental_sensor_calibration_station_thermal_reference_block.stl",
    "output/environmental_sensor_calibration_station_leak_capture_tray.stl",
    "output/environmental_sensor_calibration_station_robot_service_keepout_gauge.stl",
    "output/environmental_sensor_calibration_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 10] = [
    "temp_rh_co2_o2_probe_docks",
    "calibration_gas_ports",
    "humidity_salt_standard_pockets",
    "logger_docking_slots",
    "flow_pressure_reference_ports",
    "barcode_certificate_lands",
    "clean_used_standard_segregation",
    "thermal_reference_block",
    "leak_capture_tray",
    "robot_service_keepouts",
];

const STATION_X: f64 = 820.0;
const STATION_Y: f64 = 560.0;
const BASE_Z: f64 = 18.0;
const RIM_W: f64 = 16.0;
const RIM_Z: f64 = 34.0;
const SOCKET_DEPTH: f64 = 5.0;

const PROBE_DOCK_X: f64 = 300.0;
const PROBE_DOCK_Y: f64 = 108.0;
const PROBE_DOCK_Z: f64 = 34.0;
const PROBE_DOCK_COUNT: usize = 4;
const PROBE_DOCK_PITCH: f64 = 62.0;
const PROBE_POS: (f64, f64) = (-220.0, 170.0);

const GAS_MANIFOLD_X: f64 = 320.0;
const GAS_MANIFOLD_Y: f64 = 92.0;
const GAS_MANIFOLD_Z: f64 = 52.0;
const CAL_GAS_PORT_COUNT: usize = 5;
const CAL_GAS_PORT_PITCH: f64 = 58.0;
const GAS_TUBE_OD_MM: f64 = 3.175;
const GAS_TUBE_BORE_D: f64 = 4.4;
const GAS_POS: (f64, f64) = (200.0, 170.0);

const HUMIDITY_BLOCK_X: f64 = 260.0;
const HUMIDITY_BLOCK_Y: f64 = 118.0;
const HUMIDITY_BLOCK_Z: f64 = 44.0;
const HUMIDITY_STANDARD_COUNT: usize = 6;
const HUMIDITY_STANDARD_PITCH: f64 = 35.0;
const HUMIDITY_POS: (f64, f64) = (-245.0, 30.0);

const LOGGER_RACK_X: f64 = 220.0;
const LOGGER_RACK_Y: f64 = 114.0;
const LOGGER_RACK_Z: f64 = 60.0;
const LOGGER_SLOT_COUNT: usize = 4;
const LOGGER_SLOT_PITCH: f64 = 46.0;
const LOGGER_POS: (f64, f64) = (15.0, 30.0);

const FLOW_PRESSURE_X: f64 = 230.0;
const FLOW_PRESSURE_Y: f64 = 105.0;
const FLOW_PRESSURE_Z: f64 = 48.0;
const FLOW_REFERENCE_PORT_COUNT: usize = 4;
const PRESSURE_REFERENCE_PORT_COUNT: usize = 4;
const FLOW_PRESSURE_POS: (f64, f64) = (260.0, 30.0);

const SEG_TRAY_X: f64 = 220.0;
const SEG_TRAY_Y: f64 = 100.0;
const SEG_TRAY_Z: f64 = 36.0;
const STANDARD_WELL_COUNT_PER_SIDE: usize = 4;
const SEG_POS: (f64, f64) = (-270.0, -130.0);

const THERMAL_BLOCK_X: f64 = 160.0;
const THERMAL_BLOCK_Y: f64 = 96.0;
const THERMAL_BLOCK_Z: f64 = 42.0;
const THERMAL_PROBE_WELL_COUNT: usize = 4;
const THERMAL_POS: (f64, f64) = (-60.0, -130.0);

const LABEL_PANEL_X: f64 = 310.0;
const LABEL_PANEL_Y: f64 = 72.0;
const LABEL_PANEL_Z: f64 = 8.0;
const BARCODE_LAND_COUNT: usize = 6;
const CERTIFICATE_LAND_COUNT: usize = 2;
const LABEL_POS: (f64, f64) = (180.0, -130.0);

const LEAK_TRAY_X: f64 = 720.0;
const LEAK_TRAY_Y: f64 = 54.0;
const LEAK_TRAY_Z: f64 = 20.0;
const LEAK_CHANNEL_COUNT: usize = 3;
const LEAK_POS: (f64, f64) = (0.0, -225.0);

const KEEP_OUT_ZONE_COUNT: usize = 4;
const KEEP_OUT_GAUGE_X: f64 = STATION_X - 92.0;
const KEEP_OUT_GAUGE_Y: f64 = STATION_Y - 94.0;
const KEEP_OUT_GAUGE_Z: f64 = 6.0;
const KEEP_OUT_POS: (f64, f64) = (0.0, 0.0);

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = station_base_tray();
    export(OUTPUTS[0], &base);

    let probe_dock = probe_dock_plate();
    export(OUTPUTS[1], &probe_dock);

    let gas_manifold = gas_reference_manifold();
    export(OUTPUTS[2], &gas_manifold);

    let humidity_block = humidity_standard_block();
    export(OUTPUTS[3], &humidity_block);

    let logger_rack = logger_docking_rack();
    export(OUTPUTS[4], &logger_rack);

    let flow_pressure = flow_pressure_reference_panel();
    export(OUTPUTS[5], &flow_pressure);

    let labels = barcode_certificate_lands();
    export(OUTPUTS[6], &labels);

    let segregation = clean_used_segregation_tray();
    export(OUTPUTS[7], &segregation);

    let thermal = thermal_reference_block();
    export(OUTPUTS[8], &thermal);

    let leak_tray = leak_capture_tray();
    export(OUTPUTS[9], &leak_tray);

    let keepouts = robot_service_keepout_gauge();
    export(OUTPUTS[10], &keepouts);

    let assembly = base
        + probe_dock.translate(PROBE_POS.0, PROBE_POS.1, insert_z(PROBE_DOCK_Z))
        + gas_manifold.translate(GAS_POS.0, GAS_POS.1, insert_z(GAS_MANIFOLD_Z))
        + humidity_block.translate(HUMIDITY_POS.0, HUMIDITY_POS.1, insert_z(HUMIDITY_BLOCK_Z))
        + logger_rack.translate(LOGGER_POS.0, LOGGER_POS.1, insert_z(LOGGER_RACK_Z))
        + flow_pressure.translate(
            FLOW_PRESSURE_POS.0,
            FLOW_PRESSURE_POS.1,
            insert_z(FLOW_PRESSURE_Z),
        )
        + labels.translate(LABEL_POS.0, LABEL_POS.1, insert_z(LABEL_PANEL_Z))
        + segregation.translate(SEG_POS.0, SEG_POS.1, insert_z(SEG_TRAY_Z))
        + thermal.translate(THERMAL_POS.0, THERMAL_POS.1, insert_z(THERMAL_BLOCK_Z))
        + leak_tray.translate(LEAK_POS.0, LEAK_POS.1, insert_z(LEAK_TRAY_Z))
        + keepouts.translate(
            KEEP_OUT_POS.0,
            KEEP_OUT_POS.1,
            BASE_Z / 2.0 + KEEP_OUT_GAUGE_Z / 2.0,
        );
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Environmental sensor calibration station:");
    println!("  Footprint:                    {STATION_X:.0}mm x {STATION_Y:.0}mm tray");
    println!(
        "  Probe docks:                  {PROBE_DOCK_COUNT} reference docks for temp, RH, CO2, and O2 probes"
    );
    println!(
        "  Gas/RH standards:             {CAL_GAS_PORT_COUNT} reference gas ports using {GAS_TUBE_OD_MM:.3}mm OD tubing assumption; {HUMIDITY_STANDARD_COUNT} saturated salt pockets"
    );
    println!(
        "  Data and pressure checks:     {LOGGER_SLOT_COUNT} logger slots; {} flow/pressure reference ports",
        FLOW_REFERENCE_PORT_COUNT + PRESSURE_REFERENCE_PORT_COUNT
    );
    println!(
        "  Traceability/containment:     {BARCODE_LAND_COUNT} barcode lands, {CERTIFICATE_LAND_COUNT} certificate lands, clean/used standard tray, leak capture tray, and {KEEP_OUT_ZONE_COUNT} keepout zones"
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

fn insert_specs() -> [(&'static str, (f64, f64), f64, f64); 9] {
    [
        ("probe_dock_plate", PROBE_POS, PROBE_DOCK_X, PROBE_DOCK_Y),
        (
            "gas_reference_manifold",
            GAS_POS,
            GAS_MANIFOLD_X,
            GAS_MANIFOLD_Y,
        ),
        (
            "humidity_standard_block",
            HUMIDITY_POS,
            HUMIDITY_BLOCK_X,
            HUMIDITY_BLOCK_Y,
        ),
        (
            "logger_docking_rack",
            LOGGER_POS,
            LOGGER_RACK_X,
            LOGGER_RACK_Y,
        ),
        (
            "flow_pressure_reference_panel",
            FLOW_PRESSURE_POS,
            FLOW_PRESSURE_X,
            FLOW_PRESSURE_Y,
        ),
        (
            "clean_used_segregation_tray",
            SEG_POS,
            SEG_TRAY_X,
            SEG_TRAY_Y,
        ),
        (
            "thermal_reference_block",
            THERMAL_POS,
            THERMAL_BLOCK_X,
            THERMAL_BLOCK_Y,
        ),
        (
            "barcode_certificate_lands",
            LABEL_POS,
            LABEL_PANEL_X,
            LABEL_PANEL_Y,
        ),
        ("leak_capture_tray", LEAK_POS, LEAK_TRAY_X, LEAK_TRAY_Y),
    ]
}

fn fits_on_station(pos: (f64, f64), width: f64, depth: f64) -> bool {
    pos.0.abs() + width / 2.0 <= STATION_X / 2.0 - RIM_W - 8.0
        && pos.1.abs() + depth / 2.0 <= STATION_Y / 2.0 - RIM_W - 8.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn station_base_tray() -> Part {
    let deck = centered_cube(
        "environmental_sensor_calibration_station_base_floor",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let washdown_recess = centered_cube(
        "environmental_sensor_calibration_station_washdown_recess",
        STATION_X - 92.0,
        STATION_Y - 88.0,
        6.0,
    )
    .translate(0.0, -4.0, BASE_Z / 2.0 - 3.0);
    let front_drain = centered_cylinder(
        "environmental_sensor_calibration_station_front_drain",
        8.0 / 2.0,
        36.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 74.0, -STATION_Y / 2.0 - 2.0, -1.0);

    deck - washdown_recess
        - front_drain
        - insert_sockets()
        - station_mounting_slots()
        - datum_pin_holes()
        + perimeter_rims()
        + zone_dividers()
        + rear_service_bulkhead_tabs()
        + robot_datum_targets()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("environmental_sensor_calibration_station_insert_sockets");
    for (name, pos, width, depth) in insert_specs() {
        sockets = sockets
            + centered_cube(
                format!("environmental_sensor_calibration_station_{name}_socket"),
                width + 6.0,
                depth + 6.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(pos.0, pos.1, BASE_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.2);
    }
    sockets
}

fn station_mounting_slots() -> Part {
    let mut slots = Part::empty("environmental_sensor_calibration_station_mounting_slots");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 46.0), -(STATION_Y / 2.0 - 42.0)),
        (STATION_X / 2.0 - 46.0, -(STATION_Y / 2.0 - 42.0)),
        (-(STATION_X / 2.0 - 46.0), STATION_Y / 2.0 - 42.0),
        (STATION_X / 2.0 - 46.0, STATION_Y / 2.0 - 42.0),
        (0.0, STATION_Y / 2.0 - 42.0),
        (0.0, -(STATION_Y / 2.0 - 42.0)),
    ]
    .iter()
    .enumerate()
    {
        slots = slots
            + centered_cylinder(
                format!("environmental_sensor_calibration_station_m6_clearance_{i}"),
                6.6 / 2.0,
                BASE_Z + 4.0,
                24,
            )
            .translate(*x, *y, 0.0)
            + centered_cube(
                format!("environmental_sensor_calibration_station_m6_slot_relief_{i}"),
                22.0,
                6.8,
                BASE_Z + 4.0,
            )
            .translate(*x, *y, 0.0);
    }
    slots
}

fn datum_pin_holes() -> Part {
    let mut holes = Part::empty("environmental_sensor_calibration_station_datum_pin_holes");
    for (i, (x, y)) in [
        (-360.0, 230.0),
        (360.0, 230.0),
        (-360.0, -232.0),
        (360.0, -232.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("environmental_sensor_calibration_station_datum_pin_clearance_{i}"),
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
        "environmental_sensor_calibration_station_left_rim",
        RIM_W,
        STATION_Y - 48.0,
        RIM_Z,
    )
    .translate(
        -(STATION_X / 2.0 - RIM_W / 2.0),
        4.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(
        "environmental_sensor_calibration_station_right_rim",
        RIM_W,
        STATION_Y - 48.0,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        4.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "environmental_sensor_calibration_station_rear_rim",
        STATION_X - 32.0,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let front_low_lip = centered_cube(
        "environmental_sensor_calibration_station_front_low_lip",
        STATION_X - 160.0,
        12.0,
        20.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 18.0, BASE_Z / 2.0 + 10.0);

    left + right + rear + front_low_lip
}

fn zone_dividers() -> Part {
    let rear_row = centered_cube(
        "environmental_sensor_calibration_station_probe_gas_row_divider",
        STATION_X - 112.0,
        10.0,
        24.0,
    )
    .translate(0.0, 101.0, BASE_Z / 2.0 + 12.0);
    let middle_row = centered_cube(
        "environmental_sensor_calibration_station_standards_row_divider",
        STATION_X - 130.0,
        10.0,
        22.0,
    )
    .translate(0.0, -50.0, BASE_Z / 2.0 + 11.0);
    let front_row = centered_cube(
        "environmental_sensor_calibration_station_traceability_row_divider",
        STATION_X - 170.0,
        8.0,
        18.0,
    )
    .translate(10.0, -183.0, BASE_Z / 2.0 + 9.0);
    let left_column = centered_cube(
        "environmental_sensor_calibration_station_clean_column_divider",
        10.0,
        238.0,
        22.0,
    )
    .translate(-118.0, -40.0, BASE_Z / 2.0 + 11.0);
    let right_column = centered_cube(
        "environmental_sensor_calibration_station_monitor_column_divider",
        10.0,
        238.0,
        22.0,
    )
    .translate(132.0, -40.0, BASE_Z / 2.0 + 11.0);

    rear_row + middle_row + front_row + left_column + right_column
}

fn rear_service_bulkhead_tabs() -> Part {
    let mut tabs = Part::empty("environmental_sensor_calibration_station_rear_bulkhead_tabs");
    for (i, x) in [-300.0, -180.0, -60.0, 60.0, 180.0, 300.0]
        .iter()
        .enumerate()
    {
        let tab = centered_cube(
            format!("environmental_sensor_calibration_station_bulkhead_tab_{i}"),
            48.0,
            18.0,
            24.0,
        )
        .translate(*x, STATION_Y / 2.0 - 43.0, BASE_Z / 2.0 + 12.0);
        let bore = centered_cylinder(
            format!("environmental_sensor_calibration_station_bulkhead_tube_bore_{i}"),
            7.0 / 2.0,
            24.0,
            22,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, STATION_Y / 2.0 - 43.0, BASE_Z / 2.0 + 12.0);
        tabs = tabs + (tab - bore);
    }
    tabs
}

fn robot_datum_targets() -> Part {
    let mut targets = Part::empty("environmental_sensor_calibration_station_robot_datum_targets");
    for (i, (x, y)) in [(-348.0, 218.0), (348.0, 218.0), (-348.0, -210.0)]
        .iter()
        .enumerate()
    {
        targets = targets
            + fiducial_disc(&format!(
                "environmental_sensor_calibration_station_robot_fiducial_{i}"
            ))
            .translate(*x, *y, BASE_Z / 2.0 + 2.5);
    }
    targets
}

fn probe_dock_plate() -> Part {
    let body = centered_cube(
        "environmental_sensor_probe_dock_plate_body",
        PROBE_DOCK_X,
        PROBE_DOCK_Y,
        PROBE_DOCK_Z,
    );
    let rear_fence = centered_cube(
        "environmental_sensor_probe_dock_rear_fence",
        PROBE_DOCK_X,
        14.0,
        PROBE_DOCK_Z + 26.0,
    )
    .translate(0.0, PROBE_DOCK_Y / 2.0 - 7.0, 13.0);

    let mut dock_cuts = Part::empty("environmental_sensor_reference_probe_dock_cuts");
    for i in 0..PROBE_DOCK_COUNT {
        let x = centered_index(i, PROBE_DOCK_COUNT, PROBE_DOCK_PITCH);
        let probe_name = match i {
            0 => "temperature",
            1 => "rh",
            2 => "co2",
            _ => "o2",
        };
        let sleeve = centered_cylinder(
            format!("environmental_sensor_{probe_name}_probe_sleeve"),
            probe_sleeve_radius(i),
            PROBE_DOCK_Y + 10.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -4.0, 7.0);
        let top_access = centered_cube(
            format!("environmental_sensor_{probe_name}_probe_top_access_slot"),
            16.0,
            PROBE_DOCK_Y - 14.0,
            18.0,
        )
        .translate(x, -8.0, PROBE_DOCK_Z / 2.0 - 5.0);
        let cable_saddle = centered_cube(
            format!("environmental_sensor_{probe_name}_cable_saddle"),
            21.0,
            18.0,
            12.0,
        )
        .translate(x, -PROBE_DOCK_Y / 2.0 + 12.0, PROBE_DOCK_Z / 2.0 - 3.0);
        dock_cuts = dock_cuts + sleeve + top_access + cable_saddle;
    }

    let mut nose_stops = Part::empty("environmental_sensor_reference_probe_nose_stops");
    for i in 0..PROBE_DOCK_COUNT {
        let x = centered_index(i, PROBE_DOCK_COUNT, PROBE_DOCK_PITCH);
        nose_stops = nose_stops
            + centered_cube(
                format!("environmental_sensor_reference_probe_nose_stop_{i}"),
                24.0,
                6.0,
                16.0,
            )
            .translate(x, PROBE_DOCK_Y / 2.0 - 22.0, PROBE_DOCK_Z / 2.0 + 8.0);
    }

    body + rear_fence - dock_cuts + nose_stops + gripper_fiducials("probe_dock_plate", 104.0)
}

fn probe_sleeve_radius(index: usize) -> f64 {
    match index {
        0 => 5.0 / 2.0,
        1 => 8.0 / 2.0,
        2 => 10.0 / 2.0,
        _ => 8.5 / 2.0,
    }
}

fn gas_reference_manifold() -> Part {
    let body = centered_cube(
        "environmental_sensor_gas_reference_manifold_body",
        GAS_MANIFOLD_X,
        GAS_MANIFOLD_Y,
        GAS_MANIFOLD_Z,
    );
    let rear_backer = centered_cube(
        "environmental_sensor_gas_reference_rear_backer",
        GAS_MANIFOLD_X,
        14.0,
        GAS_MANIFOLD_Z + 32.0,
    )
    .translate(0.0, GAS_MANIFOLD_Y / 2.0 - 7.0, 16.0);

    let mut gas_ports = Part::empty("environmental_sensor_calibration_gas_port_bores");
    for i in 0..CAL_GAS_PORT_COUNT {
        let x = centered_index(i, CAL_GAS_PORT_COUNT, CAL_GAS_PORT_PITCH);
        let port_name = match i {
            0 => "zero_air",
            1 => "co2_span",
            2 => "o2_air",
            3 => "nitrogen_low_o2",
            _ => "purge_return",
        };
        let bulkhead_bore = centered_cylinder(
            format!("environmental_sensor_{port_name}_bulkhead_bore"),
            10.2 / 2.0,
            GAS_MANIFOLD_Y + 8.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -1.0, 2.0);
        let tube_bore = centered_cylinder(
            format!("environmental_sensor_{port_name}_one_eighth_tube_bore"),
            GAS_TUBE_BORE_D / 2.0,
            GAS_MANIFOLD_X + 12.0,
            24,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, 0.0, 2.0);
        let wrench_flat = centered_cube(
            format!("environmental_sensor_{port_name}_wrench_flat"),
            20.0,
            16.0,
            10.0,
        )
        .translate(x, -GAS_MANIFOLD_Y / 2.0 + 12.0, 2.0);
        gas_ports = gas_ports + bulkhead_bore + tube_bore + wrench_flat;
    }

    let mut valve_pockets = Part::empty("environmental_sensor_calibration_valve_pockets");
    for i in 0..CAL_GAS_PORT_COUNT {
        let x = centered_index(i, CAL_GAS_PORT_COUNT, CAL_GAS_PORT_PITCH);
        valve_pockets = valve_pockets
            + centered_cylinder(
                format!("environmental_sensor_calibration_valve_stem_clearance_{i}"),
                13.0 / 2.0,
                24.0,
                32,
            )
            .translate(x, 25.0, GAS_MANIFOLD_Z / 2.0 - 10.0)
            + centered_cube(
                format!("environmental_sensor_calibration_valve_label_recess_{i}"),
                34.0,
                18.0,
                5.0,
            )
            .translate(x, 25.0, GAS_MANIFOLD_Z / 2.0 - 2.5);
    }

    let leak_test_trench = centered_cube(
        "environmental_sensor_calibration_gas_leak_witness_trench",
        GAS_MANIFOLD_X - 36.0,
        8.0,
        7.0,
    )
    .translate(0.0, -GAS_MANIFOLD_Y / 2.0 + 8.0, GAS_MANIFOLD_Z / 2.0 - 3.5);

    body + rear_backer - gas_ports - valve_pockets - leak_test_trench
        + gripper_fiducials("gas_reference_manifold", 116.0)
}

fn humidity_standard_block() -> Part {
    let body = centered_cube(
        "environmental_sensor_humidity_standard_block_body",
        HUMIDITY_BLOCK_X,
        HUMIDITY_BLOCK_Y,
        HUMIDITY_BLOCK_Z,
    );
    let spill_moat = centered_cube(
        "environmental_sensor_humidity_standard_spill_moat",
        HUMIDITY_BLOCK_X - 30.0,
        HUMIDITY_BLOCK_Y - 26.0,
        8.0,
    )
    .translate(0.0, 0.0, HUMIDITY_BLOCK_Z / 2.0 - 4.0);

    let mut standard_wells = Part::empty("environmental_sensor_saturated_salt_standard_wells");
    for i in 0..HUMIDITY_STANDARD_COUNT {
        let x = centered_index(i, HUMIDITY_STANDARD_COUNT, HUMIDITY_STANDARD_PITCH);
        let salt_name = match i {
            0 => "licl_11rh",
            1 => "mgcl2_33rh",
            2 => "nabr_58rh",
            3 => "nacl_75rh",
            4 => "kcl_85rh",
            _ => "k2so4_97rh",
        };
        let well = centered_cylinder(
            format!("environmental_sensor_{salt_name}_salt_cup_well"),
            18.0 / 2.0,
            38.0,
            40,
        )
        .translate(x, 18.0, HUMIDITY_BLOCK_Z / 2.0 - 19.0);
        let lead_in = centered_cylinder(
            format!("environmental_sensor_{salt_name}_salt_cup_lead_in"),
            25.0 / 2.0,
            8.0,
            40,
        )
        .translate(x, 18.0, HUMIDITY_BLOCK_Z / 2.0 - 4.0);
        let cap_lip = centered_cylinder(
            format!("environmental_sensor_{salt_name}_salt_cup_cap_lip"),
            28.0 / 2.0,
            3.0,
            40,
        )
        .translate(x, 18.0, HUMIDITY_BLOCK_Z / 2.0 + 1.5);
        standard_wells = standard_wells + well + lead_in - cap_lip;
    }

    let mut probe_sleeves = Part::empty("environmental_sensor_humidity_probe_sleeves");
    for (i, x) in [-72.0, 0.0, 72.0].iter().enumerate() {
        probe_sleeves = probe_sleeves
            + centered_cylinder(
                format!("environmental_sensor_humidity_probe_sleeve_{i}"),
                8.5 / 2.0,
                HUMIDITY_BLOCK_Y + 8.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, -28.0, 7.0)
            + centered_cube(
                format!("environmental_sensor_humidity_probe_top_slot_{i}"),
                13.0,
                48.0,
                16.0,
            )
            .translate(*x, -29.0, HUMIDITY_BLOCK_Z / 2.0 - 5.0);
    }

    body - spill_moat - standard_wells - probe_sleeves
        + gripper_fiducials("humidity_standard_block", 88.0)
}

fn logger_docking_rack() -> Part {
    let body = centered_cube(
        "environmental_sensor_logger_docking_rack_body",
        LOGGER_RACK_X,
        LOGGER_RACK_Y,
        LOGGER_RACK_Z,
    );
    let rear_fence = centered_cube(
        "environmental_sensor_logger_rear_connector_fence",
        LOGGER_RACK_X,
        12.0,
        LOGGER_RACK_Z + 20.0,
    )
    .translate(0.0, LOGGER_RACK_Y / 2.0 - 6.0, 10.0);

    let mut logger_slots = Part::empty("environmental_sensor_logger_docking_slots");
    for i in 0..LOGGER_SLOT_COUNT {
        let x = centered_index(i, LOGGER_SLOT_COUNT, LOGGER_SLOT_PITCH);
        let slot = centered_cube(
            format!("environmental_sensor_logger_body_slot_{i}"),
            31.0,
            74.0,
            LOGGER_RACK_Z + 4.0,
        )
        .translate(x, -9.0, 8.0);
        let finger_relief = centered_cylinder(
            format!("environmental_sensor_logger_finger_relief_{i}"),
            12.0 / 2.0,
            34.0,
            28,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(x, -LOGGER_RACK_Y / 2.0 + 11.0, LOGGER_RACK_Z / 2.0 - 10.0);
        let pogo_pocket = centered_cube(
            format!("environmental_sensor_logger_contact_pogo_pocket_{i}"),
            24.0,
            8.0,
            16.0,
        )
        .translate(x, LOGGER_RACK_Y / 2.0 - 14.0, LOGGER_RACK_Z / 2.0 - 10.0);
        logger_slots = logger_slots + slot + finger_relief + pogo_pocket;
    }

    let cable_race = centered_cube(
        "environmental_sensor_logger_cable_race",
        LOGGER_RACK_X - 24.0,
        14.0,
        16.0,
    )
    .translate(0.0, LOGGER_RACK_Y / 2.0 - 14.0, -4.0);

    body + rear_fence - logger_slots - cable_race + gripper_fiducials("logger_docking_rack", 82.0)
}

fn flow_pressure_reference_panel() -> Part {
    let body = centered_cube(
        "environmental_sensor_flow_pressure_reference_panel_body",
        FLOW_PRESSURE_X,
        FLOW_PRESSURE_Y,
        FLOW_PRESSURE_Z,
    );
    let rear_backer = centered_cube(
        "environmental_sensor_flow_pressure_reference_panel_backer",
        FLOW_PRESSURE_X,
        13.0,
        FLOW_PRESSURE_Z + 22.0,
    )
    .translate(0.0, FLOW_PRESSURE_Y / 2.0 - 6.5, 11.0);

    let mut flow_ports = Part::empty("environmental_sensor_flow_reference_ports");
    for i in 0..FLOW_REFERENCE_PORT_COUNT {
        let x = centered_index(i, FLOW_REFERENCE_PORT_COUNT, 44.0);
        flow_ports = flow_ports
            + centered_cylinder(
                format!("environmental_sensor_flow_reference_orifice_port_{i}"),
                7.0 / 2.0,
                FLOW_PRESSURE_Y + 8.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -12.0, 8.0)
            + centered_cube(
                format!("environmental_sensor_flow_reference_orifice_card_slot_{i}"),
                24.0,
                8.0,
                22.0,
            )
            .translate(x, 22.0, 7.0);
    }

    let mut pressure_ports = Part::empty("environmental_sensor_pressure_reference_ports");
    for i in 0..PRESSURE_REFERENCE_PORT_COUNT {
        let x = centered_index(i, PRESSURE_REFERENCE_PORT_COUNT, 44.0);
        pressure_ports = pressure_ports
            + centered_cylinder(
                format!("environmental_sensor_pressure_reference_luer_port_{i}"),
                8.6 / 2.0,
                FLOW_PRESSURE_Y + 8.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -12.0, -13.0)
            + centered_cube(
                format!("environmental_sensor_pressure_reference_transducer_pad_{i}"),
                30.0,
                16.0,
                6.0,
            )
            .translate(x, 25.0, FLOW_PRESSURE_Z / 2.0 - 3.0);
    }

    let manometer_slot = centered_cube(
        "environmental_sensor_pressure_reference_manometer_slot",
        FLOW_PRESSURE_X - 38.0,
        10.0,
        12.0,
    )
    .translate(
        0.0,
        -FLOW_PRESSURE_Y / 2.0 + 13.0,
        FLOW_PRESSURE_Z / 2.0 - 6.0,
    );

    body + rear_backer - flow_ports - pressure_ports - manometer_slot
        + gripper_fiducials("flow_pressure_reference_panel", 84.0)
}

fn barcode_certificate_lands() -> Part {
    let carrier = centered_cube(
        "environmental_sensor_barcode_certificate_carrier",
        LABEL_PANEL_X,
        LABEL_PANEL_Y,
        LABEL_PANEL_Z,
    );
    let mut barcode_lands = Part::empty("environmental_sensor_barcode_lands");
    for i in 0..BARCODE_LAND_COUNT {
        let x = centered_index(i, BARCODE_LAND_COUNT, 43.0);
        barcode_lands = barcode_lands
            + centered_cube(
                format!("environmental_sensor_barcode_label_land_{i}"),
                36.0,
                20.0,
                2.0,
            )
            .translate(x, 19.0, LABEL_PANEL_Z / 2.0 + 1.0);
    }

    let mut certificate_lands = Part::empty("environmental_sensor_calibration_certificate_lands");
    for i in 0..CERTIFICATE_LAND_COUNT {
        let x = centered_index(i, CERTIFICATE_LAND_COUNT, 134.0);
        certificate_lands = certificate_lands
            + centered_cube(
                format!("environmental_sensor_calibration_certificate_card_land_{i}"),
                120.0,
                34.0,
                2.0,
            )
            .translate(x, -18.0, LABEL_PANEL_Z / 2.0 + 1.0);
    }

    let mut clip_holes = Part::empty("environmental_sensor_traceability_panel_clip_holes");
    for (i, x) in [
        -(LABEL_PANEL_X / 2.0 - 16.0),
        0.0,
        LABEL_PANEL_X / 2.0 - 16.0,
    ]
    .iter()
    .enumerate()
    {
        clip_holes = clip_holes
            + centered_cylinder(
                format!("environmental_sensor_traceability_panel_clip_hole_{i}"),
                3.0 / 2.0,
                LABEL_PANEL_Z + 4.0,
                18,
            )
            .translate(*x, 0.0, 0.0);
    }

    carrier - clip_holes + barcode_lands + certificate_lands
}

fn clean_used_segregation_tray() -> Part {
    let body = centered_cube(
        "environmental_sensor_clean_used_standard_tray_body",
        SEG_TRAY_X,
        SEG_TRAY_Y,
        SEG_TRAY_Z,
    );
    let clean_bin = centered_cube(
        "environmental_sensor_clean_standard_bin",
        SEG_TRAY_X - 26.0,
        36.0,
        SEG_TRAY_Z - 8.0,
    )
    .translate(0.0, clean_standard_center_y(), SEG_TRAY_Z / 2.0 - 14.0);
    let used_bin = centered_cube(
        "environmental_sensor_used_standard_quarantine_bin",
        SEG_TRAY_X - 26.0,
        36.0,
        SEG_TRAY_Z - 8.0,
    )
    .translate(0.0, used_standard_center_y(), SEG_TRAY_Z / 2.0 - 14.0);
    let divider = centered_cube(
        "environmental_sensor_clean_used_physical_divider",
        SEG_TRAY_X - 18.0,
        8.0,
        SEG_TRAY_Z + 18.0,
    )
    .translate(0.0, -1.0, 9.0);

    let mut standard_wells = Part::empty("environmental_sensor_clean_used_standard_tube_wells");
    for i in 0..STANDARD_WELL_COUNT_PER_SIDE {
        let x = centered_index(i, STANDARD_WELL_COUNT_PER_SIDE, 42.0);
        standard_wells = standard_wells
            + centered_cylinder(
                format!("environmental_sensor_clean_standard_tube_well_{i}"),
                13.0 / 2.0,
                28.0,
                28,
            )
            .translate(x, clean_standard_center_y(), SEG_TRAY_Z / 2.0 - 12.0)
            + centered_cylinder(
                format!("environmental_sensor_used_standard_tube_well_{i}"),
                13.0 / 2.0,
                28.0,
                28,
            )
            .translate(x, used_standard_center_y(), SEG_TRAY_Z / 2.0 - 12.0);
    }

    body - clean_bin - used_bin - standard_wells
        + divider
        + gripper_fiducials("segregation_tray", 78.0)
}

fn clean_standard_center_y() -> f64 {
    24.0
}

fn used_standard_center_y() -> f64 {
    -26.0
}

fn thermal_reference_block() -> Part {
    let body = centered_cube(
        "environmental_sensor_thermal_reference_block_body",
        THERMAL_BLOCK_X,
        THERMAL_BLOCK_Y,
        THERMAL_BLOCK_Z,
    );
    let cassette_sensor_pocket = centered_cube(
        "environmental_sensor_thermal_block_cassette_sensor_pocket",
        THERMAL_BLOCK_X - 34.0,
        32.0,
        10.0,
    )
    .translate(0.0, 20.0, THERMAL_BLOCK_Z / 2.0 - 5.0);

    let mut probe_wells = Part::empty("environmental_sensor_thermal_probe_wells");
    for i in 0..THERMAL_PROBE_WELL_COUNT {
        let x = centered_index(i, THERMAL_PROBE_WELL_COUNT, 32.0);
        probe_wells = probe_wells
            + centered_cylinder(
                format!("environmental_sensor_thermal_reference_probe_well_{i}"),
                4.2 / 2.0,
                THERMAL_BLOCK_Y + 8.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -22.0, 5.0)
            + centered_cube(
                format!("environmental_sensor_thermal_probe_top_access_{i}"),
                9.0,
                45.0,
                12.0,
            )
            .translate(x, -22.0, THERMAL_BLOCK_Z / 2.0 - 5.0);
    }

    let heater_bore = centered_cylinder(
        "environmental_sensor_thermal_block_cartridge_heater_bore",
        6.0 / 2.0,
        THERMAL_BLOCK_X + 8.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -31.0, -7.0);
    let rtd_bore = centered_cylinder(
        "environmental_sensor_thermal_block_reference_rtd_bore",
        3.2 / 2.0,
        THERMAL_BLOCK_X + 8.0,
        20,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -12.0, -7.0);
    let cable_gland = centered_cube(
        "environmental_sensor_thermal_block_cable_gland_slot",
        38.0,
        14.0,
        12.0,
    )
    .translate(
        THERMAL_BLOCK_X / 2.0 - 17.0,
        -32.0,
        THERMAL_BLOCK_Z / 2.0 - 6.0,
    );

    body - cassette_sensor_pocket - probe_wells - heater_bore - rtd_bore - cable_gland
        + gripper_fiducials("thermal_reference_block", 58.0)
}

fn leak_capture_tray() -> Part {
    let body = centered_cube(
        "environmental_sensor_leak_capture_tray_body",
        LEAK_TRAY_X,
        LEAK_TRAY_Y,
        LEAK_TRAY_Z,
    );
    let trough = centered_cube(
        "environmental_sensor_leak_capture_absorbent_pad_trough",
        LEAK_TRAY_X - 42.0,
        LEAK_TRAY_Y - 16.0,
        LEAK_TRAY_Z - 6.0,
    )
    .translate(0.0, 0.0, LEAK_TRAY_Z / 2.0 - 9.0);
    let drain = centered_cylinder(
        "environmental_sensor_leak_capture_front_drain_port",
        7.0 / 2.0,
        24.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(LEAK_TRAY_X / 2.0 - 38.0, -LEAK_TRAY_Y / 2.0 + 4.0, 0.0);

    let mut witness_ribs = Part::empty("environmental_sensor_leak_capture_witness_ribs");
    for i in 0..LEAK_CHANNEL_COUNT {
        let x = centered_index(i, LEAK_CHANNEL_COUNT, 205.0);
        witness_ribs = witness_ribs
            + centered_cube(
                format!("environmental_sensor_leak_capture_witness_rib_{i}"),
                140.0,
                5.0,
                6.0,
            )
            .translate(x, 0.0, LEAK_TRAY_Z / 2.0 + 3.0);
    }

    body - trough - drain + witness_ribs
}

fn robot_service_keepout_gauge() -> Part {
    let mut zones = Part::empty("environmental_sensor_robot_service_keepout_zones");
    for (i, (name, x, y, width, depth)) in [
        ("front_service_sweep", 0.0, -205.0, 690.0, 14.0),
        ("rear_tube_cable_sweep", 0.0, 238.0, 704.0, 12.0),
        ("left_robot_gripper_lane", -382.0, 8.0, 12.0, 404.0),
        ("right_service_hand_lane", 382.0, 8.0, 12.0, 404.0),
    ]
    .iter()
    .enumerate()
    {
        zones = zones
            + centered_cube(
                format!("environmental_sensor_keepout_{i}_{name}"),
                *width,
                *depth,
                KEEP_OUT_GAUGE_Z,
            )
            .translate(*x, *y, 0.0);
    }

    let robot_reach_cross = centered_cube(
        "environmental_sensor_robot_keepout_centerline_x",
        KEEP_OUT_GAUGE_X,
        4.0,
        KEEP_OUT_GAUGE_Z,
    ) + centered_cube(
        "environmental_sensor_robot_keepout_centerline_y",
        4.0,
        KEEP_OUT_GAUGE_Y,
        KEEP_OUT_GAUGE_Z,
    );
    zones + robot_reach_cross
}

fn gripper_fiducials(name: &str, x_offset: f64) -> Part {
    let mut fiducials = Part::empty(format!("environmental_sensor_{name}_gripper_fiducials"));
    for (i, x) in [-x_offset, x_offset].iter().enumerate() {
        fiducials = fiducials
            + fiducial_disc(&format!("environmental_sensor_{name}_fiducial_{i}"))
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
        assert_eq!(OUTPUTS.len(), 12);
        for path in OUTPUTS {
            assert!(path.starts_with("output/environmental_sensor_calibration_station_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn station_covers_required_sensor_calibration_features() {
        assert_eq!(REQUIRED_FEATURES.len(), 10);
        assert!(REQUIRED_FEATURES.contains(&"temp_rh_co2_o2_probe_docks"));
        assert!(REQUIRED_FEATURES.contains(&"calibration_gas_ports"));
        assert!(REQUIRED_FEATURES.contains(&"humidity_salt_standard_pockets"));
        assert!(REQUIRED_FEATURES.contains(&"logger_docking_slots"));
        assert!(REQUIRED_FEATURES.contains(&"flow_pressure_reference_ports"));
        assert!(REQUIRED_FEATURES.contains(&"barcode_certificate_lands"));
        assert!(REQUIRED_FEATURES.contains(&"clean_used_standard_segregation"));
        assert!(REQUIRED_FEATURES.contains(&"thermal_reference_block"));
        assert!(REQUIRED_FEATURES.contains(&"leak_capture_tray"));
        assert!(REQUIRED_FEATURES.contains(&"robot_service_keepouts"));
    }

    #[test]
    fn all_inserts_fit_inside_station_rims() {
        for (_name, pos, width, depth) in insert_specs() {
            assert!(fits_on_station(pos, width, depth));
        }
        assert!(KEEP_OUT_GAUGE_X <= STATION_X - 80.0);
        assert!(KEEP_OUT_GAUGE_Y <= STATION_Y - 80.0);
    }

    #[test]
    fn sensor_and_standard_counts_match_station_scope() {
        assert_eq!(PROBE_DOCK_COUNT, 4);
        assert_eq!(CAL_GAS_PORT_COUNT, 5);
        assert_eq!(HUMIDITY_STANDARD_COUNT, 6);
        assert_eq!(LOGGER_SLOT_COUNT, 4);
        assert_eq!(FLOW_REFERENCE_PORT_COUNT + PRESSURE_REFERENCE_PORT_COUNT, 8);
        assert_eq!(STANDARD_WELL_COUNT_PER_SIDE * 2, 8);
        assert_eq!(THERMAL_PROBE_WELL_COUNT, 4);
        assert_eq!(KEEP_OUT_ZONE_COUNT, 4);
    }

    #[test]
    fn clean_and_used_standard_bins_are_physically_separated() {
        let bin_depth = 36.0;
        let gap = clean_standard_center_y()
            - bin_depth / 2.0
            - (used_standard_center_y() + bin_depth / 2.0);
        assert!(gap >= 12.0);
        assert!(SEG_TRAY_Z >= 34.0);
    }

    #[test]
    fn gas_reference_assumption_uses_one_eighth_tube_clearance() {
        assert!((GAS_TUBE_OD_MM - 3.175).abs() < 0.001);
        assert!(GAS_TUBE_BORE_D > GAS_TUBE_OD_MM + 1.0);
        assert!(CAL_GAS_PORT_PITCH >= 55.0);
    }

    #[test]
    fn traceability_lands_cover_barcode_and_certificate_records() {
        assert_eq!(BARCODE_LAND_COUNT, 6);
        assert_eq!(CERTIFICATE_LAND_COUNT, 2);
        assert!(LABEL_PANEL_X >= 300.0);
        assert!(LABEL_PANEL_Y >= 70.0);
    }
}
