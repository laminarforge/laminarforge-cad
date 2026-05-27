use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Calibration drawer/module for automated tissue-chip workcell qualification.
//
// Intent:
// - Stage known standards and adapters in a repeatable drawer module before a
//   workcell qualification run.
// - Keep fluidic, electrical, chemistry, imaging, environmental mapping, and
//   clean/used handling fixtures physically separated but reachable from one
//   robot/technician access plane.
// - Reserve barcode/lot label lands beside the calibration articles so scanned
//   lots and qualification records can be tied to the physical drawer layout.
//
// This is packaging and fixture geometry only. Calibration methods, acceptance
// limits, sensor traceability, and biological workflow release stay separate
// validation gates.

const OUTPUTS: [&str; 10] = [
    "output/workcell_calibration_drawer_base_tray.stl",
    "output/workcell_calibration_drawer_flow_restrictor_caddy.stl",
    "output/workcell_calibration_drawer_pressure_leak_adapter_panel.stl",
    "output/workcell_calibration_drawer_teer_phantom_holder.stl",
    "output/workcell_calibration_drawer_chemistry_standard_block.stl",
    "output/workcell_calibration_drawer_imaging_target_cassette.stl",
    "output/workcell_calibration_drawer_environment_logger_rack.stl",
    "output/workcell_calibration_drawer_clean_used_segregation_tray.stl",
    "output/workcell_calibration_drawer_barcode_lot_label_lands.stl",
    "output/workcell_calibration_drawer_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 8] = [
    "flow_restrictor_standards",
    "pressure_leak_test_adapters",
    "teer_phantom_resistor_board",
    "ph_do_o2_calibration_standards",
    "imaging_slides_targets",
    "environmental_mapping_loggers",
    "barcode_lot_label_lands",
    "clean_used_segregation",
];

const DRAWER_X: f64 = 640.0;
const DRAWER_Y: f64 = 420.0;
const BASE_Z: f64 = 18.0;
const RIM_W: f64 = 16.0;
const RIM_Z: f64 = 34.0;
const SOCKET_DEPTH: f64 = 5.0;
const DATUM_PIN_D: f64 = 6.0;

const FLOW_CADDY_X: f64 = 260.0;
const FLOW_CADDY_Y: f64 = 74.0;
const FLOW_CADDY_Z: f64 = 24.0;
const FLOW_RESTRICTOR_COUNT: usize = 8;
const FLOW_RESTRICTOR_PITCH: f64 = 28.0;
const FLOW_POS: (f64, f64) = (-160.0, 130.0);

const PRESSURE_PANEL_X: f64 = 220.0;
const PRESSURE_PANEL_Y: f64 = 96.0;
const PRESSURE_PANEL_Z: f64 = 30.0;
const PRESSURE_ADAPTER_COUNT: usize = 6;
const PRESSURE_ADAPTER_PITCH: f64 = 31.0;
const PRESSURE_POS: (f64, f64) = (175.0, 130.0);

const TEER_HOLDER_X: f64 = 190.0;
const TEER_HOLDER_Y: f64 = 108.0;
const TEER_HOLDER_Z: f64 = 26.0;
const TEER_BOARD_X: f64 = 152.0;
const TEER_BOARD_Y: f64 = 78.0;
const TEER_POS: (f64, f64) = (-200.0, 8.0);

const CHEM_BLOCK_X: f64 = 250.0;
const CHEM_BLOCK_Y: f64 = 118.0;
const CHEM_BLOCK_Z: f64 = 46.0;
const CHEM_STANDARD_COUNT: usize = 6;
const CHEM_STANDARD_PITCH: f64 = 33.0;
const CHEM_POS: (f64, f64) = (70.0, 8.0);

const IMAGING_CASSETTE_X: f64 = 220.0;
const IMAGING_CASSETTE_Y: f64 = 94.0;
const IMAGING_CASSETTE_Z: f64 = 16.0;
const IMAGING_POS: (f64, f64) = (-190.0, -118.0);

const LOGGER_RACK_X: f64 = 190.0;
const LOGGER_RACK_Y: f64 = 92.0;
const LOGGER_RACK_Z: f64 = 58.0;
const LOGGER_COUNT: usize = 3;
const LOGGER_PITCH: f64 = 54.0;
const LOGGER_POS: (f64, f64) = (40.0, -118.0);

const SEG_TRAY_X: f64 = 150.0;
const SEG_TRAY_Y: f64 = 122.0;
const SEG_TRAY_Z: f64 = 32.0;
const SEG_POS: (f64, f64) = (220.0, -118.0);

const LABEL_LANDS_X: f64 = 564.0;
const LABEL_LANDS_Y: f64 = 24.0;
const LABEL_LANDS_Z: f64 = 4.0;
const LABEL_LAND_COUNT: usize = 9;
const LABEL_LAND_PITCH: f64 = 62.0;
const LABEL_POS: (f64, f64) = (0.0, -188.0);

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = drawer_base_tray();
    export(OUTPUTS[0], &base);

    let flow = flow_restrictor_caddy();
    export(OUTPUTS[1], &flow);

    let pressure = pressure_leak_adapter_panel();
    export(OUTPUTS[2], &pressure);

    let teer = teer_phantom_holder();
    export(OUTPUTS[3], &teer);

    let chemistry = chemistry_standard_block();
    export(OUTPUTS[4], &chemistry);

    let imaging = imaging_target_cassette();
    export(OUTPUTS[5], &imaging);

    let loggers = environmental_logger_rack();
    export(OUTPUTS[6], &loggers);

    let segregation = clean_used_segregation_tray();
    export(OUTPUTS[7], &segregation);

    let labels = barcode_lot_label_lands();
    export(OUTPUTS[8], &labels);

    let assembly = base
        + flow.translate(FLOW_POS.0, FLOW_POS.1, insert_z(FLOW_CADDY_Z))
        + pressure.translate(PRESSURE_POS.0, PRESSURE_POS.1, insert_z(PRESSURE_PANEL_Z))
        + teer.translate(TEER_POS.0, TEER_POS.1, insert_z(TEER_HOLDER_Z))
        + chemistry.translate(CHEM_POS.0, CHEM_POS.1, insert_z(CHEM_BLOCK_Z))
        + imaging.translate(IMAGING_POS.0, IMAGING_POS.1, insert_z(IMAGING_CASSETTE_Z))
        + loggers.translate(LOGGER_POS.0, LOGGER_POS.1, insert_z(LOGGER_RACK_Z))
        + segregation.translate(SEG_POS.0, SEG_POS.1, insert_z(SEG_TRAY_Z))
        + labels.translate(LABEL_POS.0, LABEL_POS.1, insert_z(LABEL_LANDS_Z));
    export(OUTPUTS[9], &assembly);

    println!();
    println!("Workcell calibration drawer:");
    println!("  Drawer footprint:             {DRAWER_X:.0}mm x {DRAWER_Y:.0}mm");
    println!(
        "  Standards staged:             {FLOW_RESTRICTOR_COUNT} flow restrictors, {PRESSURE_ADAPTER_COUNT} pressure/leak adapters, TEER phantom board, {CHEM_STANDARD_COUNT} pH/DO/O2 standards"
    );
    println!(
        "  Qualification fixtures:       imaging slides/targets, {LOGGER_COUNT} environmental logger holders, clean/used segregation tray"
    );
    println!(
        "  Traceability:                 {LABEL_LAND_COUNT} barcode/lot label lands across front service edge"
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
    assert!(
        fits_on_drawer(FLOW_POS, FLOW_CADDY_X, FLOW_CADDY_Y),
        "flow restrictor caddy exceeds drawer envelope"
    );
    assert!(
        fits_on_drawer(PRESSURE_POS, PRESSURE_PANEL_X, PRESSURE_PANEL_Y),
        "pressure/leak adapter panel exceeds drawer envelope"
    );
    assert!(
        fits_on_drawer(TEER_POS, TEER_HOLDER_X, TEER_HOLDER_Y),
        "TEER phantom holder exceeds drawer envelope"
    );
    assert!(
        fits_on_drawer(CHEM_POS, CHEM_BLOCK_X, CHEM_BLOCK_Y),
        "chemistry standard block exceeds drawer envelope"
    );
    assert!(
        fits_on_drawer(IMAGING_POS, IMAGING_CASSETTE_X, IMAGING_CASSETTE_Y),
        "imaging target cassette exceeds drawer envelope"
    );
    assert!(
        fits_on_drawer(LOGGER_POS, LOGGER_RACK_X, LOGGER_RACK_Y),
        "logger rack exceeds drawer envelope"
    );
    assert!(
        fits_on_drawer(SEG_POS, SEG_TRAY_X, SEG_TRAY_Y),
        "clean/used tray exceeds drawer envelope"
    );
}

fn fits_on_drawer(pos: (f64, f64), width: f64, depth: f64) -> bool {
    pos.0.abs() + width / 2.0 <= DRAWER_X / 2.0 - RIM_W - 4.0
        && pos.1.abs() + depth / 2.0 <= DRAWER_Y / 2.0 - RIM_W - 4.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn drawer_base_tray() -> Part {
    let deck = centered_cube(
        "workcell_calibration_drawer_base_floor",
        DRAWER_X,
        DRAWER_Y,
        BASE_Z,
    );

    let spill_sump = centered_cube(
        "workcell_calibration_drawer_washdown_sump",
        DRAWER_X - 92.0,
        DRAWER_Y - 84.0,
        6.0,
    )
    .translate(0.0, 10.0, BASE_Z / 2.0 - 3.0);

    let drain = centered_cylinder("workcell_calibration_drawer_sump_drain", 6.0, 30.0, 28)
        .rotate(90.0, 0.0, 0.0)
        .translate(DRAWER_X / 2.0 - 58.0, -DRAWER_Y / 2.0 + 26.0, 0.0);

    let robot_pull_slot = centered_cube(
        "workcell_calibration_drawer_front_robot_pull_slot",
        96.0,
        14.0,
        12.0,
    )
    .translate(0.0, -DRAWER_Y / 2.0 + 14.0, BASE_Z / 2.0 + 2.0);

    let sockets = insert_sockets();
    let mount_holes = drawer_mount_holes();
    let datum_holes = datum_pin_holes();

    deck - spill_sump - drain - robot_pull_slot - sockets - mount_holes - datum_holes
        + perimeter_rims()
        + zone_dividers()
        + drawer_slide_rails()
        + rear_datum_bosses()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("workcell_calibration_drawer_insert_sockets");
    for (name, pos, width, depth) in [
        (
            "flow_restrictor_caddy",
            FLOW_POS,
            FLOW_CADDY_X,
            FLOW_CADDY_Y,
        ),
        (
            "pressure_leak_adapter_panel",
            PRESSURE_POS,
            PRESSURE_PANEL_X,
            PRESSURE_PANEL_Y,
        ),
        (
            "teer_phantom_holder",
            TEER_POS,
            TEER_HOLDER_X,
            TEER_HOLDER_Y,
        ),
        (
            "chemistry_standard_block",
            CHEM_POS,
            CHEM_BLOCK_X,
            CHEM_BLOCK_Y,
        ),
        (
            "imaging_target_cassette",
            IMAGING_POS,
            IMAGING_CASSETTE_X,
            IMAGING_CASSETTE_Y,
        ),
        (
            "environmental_logger_rack",
            LOGGER_POS,
            LOGGER_RACK_X,
            LOGGER_RACK_Y,
        ),
        (
            "clean_used_segregation_tray",
            SEG_POS,
            SEG_TRAY_X,
            SEG_TRAY_Y,
        ),
    ] {
        sockets = sockets
            + centered_cube(
                format!("workcell_calibration_drawer_{name}_socket"),
                width + 6.0,
                depth + 6.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(pos.0, pos.1, BASE_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.2);
    }
    sockets
}

fn drawer_mount_holes() -> Part {
    let mut holes = Part::empty("workcell_calibration_drawer_mount_holes");
    for (i, (x, y)) in [
        (-(DRAWER_X / 2.0 - 36.0), -(DRAWER_Y / 2.0 - 34.0)),
        (DRAWER_X / 2.0 - 36.0, -(DRAWER_Y / 2.0 - 34.0)),
        (-(DRAWER_X / 2.0 - 36.0), DRAWER_Y / 2.0 - 34.0),
        (DRAWER_X / 2.0 - 36.0, DRAWER_Y / 2.0 - 34.0),
        (0.0, DRAWER_Y / 2.0 - 34.0),
        (0.0, -(DRAWER_Y / 2.0 - 34.0)),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("workcell_calibration_drawer_m5_clearance_{i}"),
                5.4 / 2.0,
                BASE_Z + 4.0,
                24,
            )
            .translate(*x, *y, 0.0)
            + centered_cube(
                format!("workcell_calibration_drawer_m5_slot_relief_{i}"),
                18.0,
                5.4,
                BASE_Z + 4.0,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn datum_pin_holes() -> Part {
    let mut holes = Part::empty("workcell_calibration_drawer_datum_pin_holes");
    for (i, (x, y)) in [
        (-292.0, 170.0),
        (292.0, 170.0),
        (-292.0, -170.0),
        (292.0, -170.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("workcell_calibration_drawer_datum_pin_clearance_{i}"),
                DATUM_PIN_D / 2.0,
                BASE_Z + 4.0,
                28,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    let left = centered_cube(
        "workcell_calibration_drawer_left_rim",
        RIM_W,
        DRAWER_Y - 48.0,
        RIM_Z,
    )
    .translate(
        -(DRAWER_X / 2.0 - RIM_W / 2.0),
        8.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(
        "workcell_calibration_drawer_right_rim",
        RIM_W,
        DRAWER_Y - 48.0,
        RIM_Z,
    )
    .translate(
        DRAWER_X / 2.0 - RIM_W / 2.0,
        8.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "workcell_calibration_drawer_rear_rim",
        DRAWER_X - 32.0,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        DRAWER_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let front = centered_cube(
        "workcell_calibration_drawer_front_low_lip",
        DRAWER_X - 130.0,
        12.0,
        20.0,
    )
    .translate(0.0, -DRAWER_Y / 2.0 + 20.0, BASE_Z / 2.0 + 10.0);

    left + right + rear + front
}

fn zone_dividers() -> Part {
    let horizontal_top = centered_cube(
        "workcell_calibration_drawer_rear_row_divider",
        DRAWER_X - 88.0,
        10.0,
        24.0,
    )
    .translate(0.0, 78.0, BASE_Z / 2.0 + 12.0);
    let horizontal_front = centered_cube(
        "workcell_calibration_drawer_front_row_divider",
        DRAWER_X - 124.0,
        10.0,
        20.0,
    )
    .translate(-30.0, -54.0, BASE_Z / 2.0 + 10.0);
    let vertical_left = centered_cube(
        "workcell_calibration_drawer_left_column_divider",
        10.0,
        244.0,
        22.0,
    )
    .translate(-62.0, 40.0, BASE_Z / 2.0 + 11.0);
    let vertical_right = centered_cube(
        "workcell_calibration_drawer_right_column_divider",
        10.0,
        244.0,
        22.0,
    )
    .translate(146.0, -40.0, BASE_Z / 2.0 + 11.0);

    horizontal_top + horizontal_front + vertical_left + vertical_right
}

fn drawer_slide_rails() -> Part {
    let left = centered_cube(
        "workcell_calibration_drawer_left_underside_slide_rail",
        18.0,
        DRAWER_Y - 76.0,
        12.0,
    )
    .translate(-(DRAWER_X / 2.0 - 50.0), 0.0, -BASE_Z / 2.0 - 6.0);
    let right = centered_cube(
        "workcell_calibration_drawer_right_underside_slide_rail",
        18.0,
        DRAWER_Y - 76.0,
        12.0,
    )
    .translate(DRAWER_X / 2.0 - 50.0, 0.0, -BASE_Z / 2.0 - 6.0);
    let rear_stop = centered_cube(
        "workcell_calibration_drawer_rear_slide_stop",
        DRAWER_X - 132.0,
        16.0,
        12.0,
    )
    .translate(0.0, DRAWER_Y / 2.0 - 42.0, -BASE_Z / 2.0 - 6.0);

    left + right + rear_stop
}

fn rear_datum_bosses() -> Part {
    let mut bosses = Part::empty("workcell_calibration_drawer_rear_datum_bosses");
    for (i, x) in [-250.0, 0.0, 250.0].iter().enumerate() {
        let boss = centered_cylinder(
            format!("workcell_calibration_drawer_rear_datum_boss_{i}"),
            11.0,
            10.0,
            32,
        )
        .translate(*x, DRAWER_Y / 2.0 - 42.0, BASE_Z / 2.0 + 5.0);
        let center = centered_cylinder(
            format!("workcell_calibration_drawer_rear_datum_center_{i}"),
            3.0,
            12.0,
            24,
        )
        .translate(*x, DRAWER_Y / 2.0 - 42.0, BASE_Z / 2.0 + 5.0);
        bosses = bosses + (boss - center);
    }
    bosses
}

fn flow_restrictor_caddy() -> Part {
    let body = centered_cube(
        "workcell_calibration_flow_restrictor_caddy_body",
        FLOW_CADDY_X,
        FLOW_CADDY_Y,
        FLOW_CADDY_Z,
    );
    let relief = centered_cube(
        "workcell_calibration_flow_restrictor_caddy_lightweight_recess",
        FLOW_CADDY_X - 34.0,
        FLOW_CADDY_Y - 26.0,
        7.0,
    )
    .translate(0.0, 0.0, -FLOW_CADDY_Z / 2.0 + 4.0);

    let mut channels = Part::empty("workcell_calibration_flow_restrictor_channels");
    for i in 0..FLOW_RESTRICTOR_COUNT {
        let x = centered_index(i, FLOW_RESTRICTOR_COUNT, FLOW_RESTRICTOR_PITCH);
        let tube_groove = centered_cylinder(
            format!("workcell_calibration_flow_restrictor_v_groove_{i}"),
            4.3 / 2.0,
            FLOW_CADDY_Y + 6.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 0.0, 4.5);
        let top_access = centered_cube(
            format!("workcell_calibration_flow_restrictor_top_access_{i}"),
            7.0,
            FLOW_CADDY_Y + 8.0,
            14.0,
        )
        .translate(x, 0.0, FLOW_CADDY_Z / 2.0 - 4.0);
        let finger_relief = centered_cylinder(
            format!("workcell_calibration_flow_restrictor_finger_relief_{i}"),
            8.0,
            10.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -FLOW_CADDY_Y / 2.0 + 8.0, FLOW_CADDY_Z / 2.0 - 3.0);
        channels = channels + tube_groove + top_access + finger_relief;
    }

    let mut stop_tabs = Part::empty("workcell_calibration_flow_restrictor_end_stops");
    for (i, y) in [-(FLOW_CADDY_Y / 2.0 - 8.0), FLOW_CADDY_Y / 2.0 - 8.0]
        .iter()
        .enumerate()
    {
        stop_tabs = stop_tabs
            + centered_cube(
                format!("workcell_calibration_flow_restrictor_end_stop_{i}"),
                FLOW_CADDY_X - 28.0,
                6.0,
                12.0,
            )
            .translate(0.0, *y, FLOW_CADDY_Z / 2.0 + 6.0);
    }

    body - relief - channels + stop_tabs + caddy_gripper_fiducials("flow_restrictor_caddy")
}

fn pressure_leak_adapter_panel() -> Part {
    let body = centered_cube(
        "workcell_calibration_pressure_leak_adapter_panel_body",
        PRESSURE_PANEL_X,
        PRESSURE_PANEL_Y,
        PRESSURE_PANEL_Z,
    );
    let rear_backer = centered_cube(
        "workcell_calibration_pressure_leak_adapter_rear_backer",
        PRESSURE_PANEL_X,
        12.0,
        PRESSURE_PANEL_Z + 20.0,
    )
    .translate(0.0, PRESSURE_PANEL_Y / 2.0 - 6.0, 10.0);

    let mut adapter_holes = Part::empty("workcell_calibration_pressure_adapter_holes");
    for i in 0..PRESSURE_ADAPTER_COUNT {
        let x = centered_index(i, PRESSURE_ADAPTER_COUNT, PRESSURE_ADAPTER_PITCH);
        let bulkhead = centered_cylinder(
            format!("workcell_calibration_pressure_bulkhead_socket_{i}"),
            10.2 / 2.0,
            PRESSURE_PANEL_Y + 6.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 0.0, 5.0);
        let wrench_flat = centered_cube(
            format!("workcell_calibration_pressure_bulkhead_wrench_flat_{i}"),
            17.0,
            18.0,
            9.0,
        )
        .translate(x, -PRESSURE_PANEL_Y / 2.0 + 12.0, 5.0);
        adapter_holes = adapter_holes + bulkhead + wrench_flat;
    }

    let mut leak_plug_wells = Part::empty("workcell_calibration_leak_plug_wells");
    for (i, x) in [-78.0, -26.0, 26.0, 78.0].iter().enumerate() {
        leak_plug_wells = leak_plug_wells
            + centered_cylinder(
                format!("workcell_calibration_leak_test_plug_well_{i}"),
                12.0 / 2.0,
                18.0,
                32,
            )
            .translate(*x, 25.0, PRESSURE_PANEL_Z / 2.0 - 8.5);
    }

    let leak_witness_channel = centered_cube(
        "workcell_calibration_pressure_panel_leak_witness_channel",
        PRESSURE_PANEL_X - 36.0,
        8.0,
        6.0,
    )
    .translate(
        0.0,
        -PRESSURE_PANEL_Y / 2.0 + 9.0,
        PRESSURE_PANEL_Z / 2.0 - 3.0,
    );

    body + rear_backer - adapter_holes - leak_plug_wells - leak_witness_channel
        + panel_latch_tabs("pressure_leak_adapter")
}

fn teer_phantom_holder() -> Part {
    let body = centered_cube(
        "workcell_calibration_teer_phantom_holder_body",
        TEER_HOLDER_X,
        TEER_HOLDER_Y,
        TEER_HOLDER_Z,
    );
    let board_pocket = centered_cube(
        "workcell_calibration_teer_resistor_board_pocket",
        TEER_BOARD_X + 1.0,
        TEER_BOARD_Y + 1.0,
        8.0,
    )
    .translate(0.0, 0.0, TEER_HOLDER_Z / 2.0 - 4.0);
    let pull_notch = centered_cube(
        "workcell_calibration_teer_board_pull_notch",
        38.0,
        16.0,
        12.0,
    )
    .translate(0.0, -TEER_HOLDER_Y / 2.0 + 6.0, TEER_HOLDER_Z / 2.0 - 3.0);
    let cable_notch = centered_cube(
        "workcell_calibration_teer_kelvin_cable_notch",
        42.0,
        18.0,
        16.0,
    )
    .translate(TEER_HOLDER_X / 2.0 - 18.0, 0.0, TEER_HOLDER_Z / 2.0 - 3.0);

    let mut pogo_slots = Part::empty("workcell_calibration_teer_pogo_pin_slots");
    for i in 0..8 {
        let x = centered_index(i, 8, 15.0);
        pogo_slots = pogo_slots
            + centered_cylinder(
                format!("workcell_calibration_teer_pogo_access_{i}"),
                2.2 / 2.0,
                TEER_HOLDER_Z + 3.0,
                18,
            )
            .translate(x, TEER_BOARD_Y / 2.0 - 9.0, 0.0);
    }

    let mut board_screws = Part::empty("workcell_calibration_teer_board_screw_holes");
    for (i, (x, y)) in [
        (-(TEER_BOARD_X / 2.0 - 12.0), -(TEER_BOARD_Y / 2.0 - 10.0)),
        (TEER_BOARD_X / 2.0 - 12.0, -(TEER_BOARD_Y / 2.0 - 10.0)),
        (-(TEER_BOARD_X / 2.0 - 12.0), TEER_BOARD_Y / 2.0 - 10.0),
        (TEER_BOARD_X / 2.0 - 12.0, TEER_BOARD_Y / 2.0 - 10.0),
    ]
    .iter()
    .enumerate()
    {
        board_screws = board_screws
            + centered_cylinder(
                format!("workcell_calibration_teer_board_screw_{i}"),
                3.0 / 2.0,
                TEER_HOLDER_Z + 4.0,
                20,
            )
            .translate(*x, *y, 0.0);
    }

    let mut resistor_windows = Part::empty("workcell_calibration_teer_resistor_windows");
    for (i, x) in [-48.0, 0.0, 48.0].iter().enumerate() {
        resistor_windows = resistor_windows
            + centered_cube(
                format!("workcell_calibration_teer_precision_resistor_window_{i}"),
                34.0,
                14.0,
                10.0,
            )
            .translate(*x, -18.0, TEER_HOLDER_Z / 2.0 - 5.0);
    }

    body - board_pocket - pull_notch - cable_notch - pogo_slots - board_screws - resistor_windows
        + panel_latch_tabs("teer_phantom_holder")
}

fn chemistry_standard_block() -> Part {
    let body = centered_cube(
        "workcell_calibration_chemistry_standard_block_body",
        CHEM_BLOCK_X,
        CHEM_BLOCK_Y,
        CHEM_BLOCK_Z,
    );
    let spill_moat = centered_cube(
        "workcell_calibration_chemistry_standard_spill_moat",
        CHEM_BLOCK_X - 28.0,
        CHEM_BLOCK_Y - 26.0,
        7.0,
    )
    .translate(0.0, 0.0, CHEM_BLOCK_Z / 2.0 - 3.5);

    let mut standard_wells = Part::empty("workcell_calibration_chemistry_standard_wells");
    for i in 0..CHEM_STANDARD_COUNT {
        let x = centered_index(i, CHEM_STANDARD_COUNT, CHEM_STANDARD_PITCH);
        let standard_name = match i {
            0 => "ph_low",
            1 => "ph_neutral",
            2 => "ph_high",
            3 => "do_zero",
            4 => "do_air_saturated",
            _ => "o2_span",
        };
        let well = centered_cylinder(
            format!("workcell_calibration_{standard_name}_standard_well"),
            16.0 / 2.0,
            42.0,
            40,
        )
        .translate(x, 20.0, CHEM_BLOCK_Z / 2.0 - 20.0);
        let lead_in = centered_cylinder(
            format!("workcell_calibration_{standard_name}_standard_lead_in"),
            22.0 / 2.0,
            8.0,
            40,
        )
        .translate(x, 20.0, CHEM_BLOCK_Z / 2.0 - 4.0);
        standard_wells = standard_wells + well + lead_in;
    }

    let mut probe_sleeves = Part::empty("workcell_calibration_chemistry_probe_sleeves");
    for (i, x) in [-74.0, 0.0, 74.0].iter().enumerate() {
        probe_sleeves = probe_sleeves
            + centered_cylinder(
                format!("workcell_calibration_chemistry_probe_sleeve_{i}"),
                8.4 / 2.0,
                CHEM_BLOCK_Y + 8.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, -22.0, 8.0)
            + centered_cube(
                format!("workcell_calibration_chemistry_probe_sleeve_top_slot_{i}"),
                12.0,
                58.0,
                18.0,
            )
            .translate(*x, -24.0, CHEM_BLOCK_Z / 2.0 - 4.0);
    }

    let mut ampoule_slots = Part::empty("workcell_calibration_chemistry_ampoule_slots");
    for i in 0..5 {
        let x = centered_index(i, 5, 30.0);
        ampoule_slots = ampoule_slots
            + centered_cylinder(
                format!("workcell_calibration_chemistry_break_ampoule_slot_{i}"),
                4.8 / 2.0,
                60.0,
                20,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(x, -42.0, 7.0);
    }

    body - spill_moat - standard_wells - probe_sleeves - ampoule_slots
        + chemistry_keyed_lip()
        + caddy_gripper_fiducials("chemistry_standard_block")
}

fn chemistry_keyed_lip() -> Part {
    let rear = centered_cube(
        "workcell_calibration_chemistry_block_rear_key_lip",
        CHEM_BLOCK_X - 40.0,
        8.0,
        12.0,
    )
    .translate(0.0, CHEM_BLOCK_Y / 2.0 + 4.0, 2.0);
    let left = centered_cube(
        "workcell_calibration_chemistry_block_left_key_lip",
        8.0,
        CHEM_BLOCK_Y - 30.0,
        12.0,
    )
    .translate(-CHEM_BLOCK_X / 2.0 - 4.0, 0.0, 2.0);
    rear + left
}

fn imaging_target_cassette() -> Part {
    let body = centered_cube(
        "workcell_calibration_imaging_target_cassette_body",
        IMAGING_CASSETTE_X,
        IMAGING_CASSETTE_Y,
        IMAGING_CASSETTE_Z,
    );

    let slide_a = centered_cube(
        "workcell_calibration_imaging_slide_a_recess",
        76.5,
        26.5,
        5.5,
    )
    .translate(-46.0, 18.0, IMAGING_CASSETTE_Z / 2.0 - 2.75);
    let slide_b = centered_cube(
        "workcell_calibration_imaging_slide_b_recess",
        76.5,
        26.5,
        5.5,
    )
    .translate(46.0, 18.0, IMAGING_CASSETTE_Z / 2.0 - 2.75);
    let resolution_target = centered_cube(
        "workcell_calibration_imaging_resolution_target_recess",
        54.0,
        38.0,
        5.5,
    )
    .translate(-60.0, -28.0, IMAGING_CASSETTE_Z / 2.0 - 2.75);
    let color_target = centered_cube(
        "workcell_calibration_imaging_color_checker_recess",
        64.0,
        38.0,
        5.5,
    )
    .translate(42.0, -28.0, IMAGING_CASSETTE_Z / 2.0 - 2.75);

    let mut hold_downs = Part::empty("workcell_calibration_imaging_hold_downs");
    for (i, (x, y)) in [
        (-92.0, 37.0),
        (-2.0, 37.0),
        (2.0, 37.0),
        (92.0, 37.0),
        (-92.0, -47.0),
        (92.0, -47.0),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("workcell_calibration_imaging_target_hold_down_boss_{i}"),
            5.0,
            5.0,
            24,
        )
        .translate(*x, *y, IMAGING_CASSETTE_Z / 2.0 + 2.5);
        let screw = centered_cylinder(
            format!("workcell_calibration_imaging_target_hold_down_screw_{i}"),
            2.4 / 2.0,
            7.0,
            18,
        )
        .translate(*x, *y, IMAGING_CASSETTE_Z / 2.0 + 2.5);
        hold_downs = hold_downs + (boss - screw);
    }

    body - slide_a - slide_b - resolution_target - color_target + hold_downs + imaging_fiducials()
}

fn imaging_fiducials() -> Part {
    let mut fiducials = Part::empty("workcell_calibration_imaging_fiducials");
    for (i, (x, y)) in [(-98.0, 33.0), (98.0, 33.0), (-98.0, -33.0), (98.0, -33.0)]
        .iter()
        .enumerate()
    {
        let target = centered_cylinder(
            format!("workcell_calibration_imaging_fiducial_disc_{i}"),
            6.0,
            2.0,
            36,
        )
        .translate(*x, *y, IMAGING_CASSETTE_Z / 2.0 + 1.0);
        let center = centered_cylinder(
            format!("workcell_calibration_imaging_fiducial_center_{i}"),
            1.2,
            3.0,
            18,
        )
        .translate(*x, *y, IMAGING_CASSETTE_Z / 2.0 + 1.0);
        fiducials = fiducials + (target - center);
    }
    fiducials
}

fn environmental_logger_rack() -> Part {
    let base = centered_cube(
        "workcell_calibration_environment_logger_rack_base",
        LOGGER_RACK_X,
        LOGGER_RACK_Y,
        18.0,
    );
    let back = centered_cube(
        "workcell_calibration_environment_logger_rack_backstop",
        LOGGER_RACK_X,
        10.0,
        LOGGER_RACK_Z,
    )
    .translate(0.0, LOGGER_RACK_Y / 2.0 - 5.0, LOGGER_RACK_Z / 2.0 - 9.0);
    let side_left = centered_cube(
        "workcell_calibration_environment_logger_rack_left_side",
        10.0,
        LOGGER_RACK_Y,
        LOGGER_RACK_Z,
    )
    .translate(-LOGGER_RACK_X / 2.0 + 5.0, 0.0, LOGGER_RACK_Z / 2.0 - 9.0);
    let side_right = centered_cube(
        "workcell_calibration_environment_logger_rack_right_side",
        10.0,
        LOGGER_RACK_Y,
        LOGGER_RACK_Z,
    )
    .translate(LOGGER_RACK_X / 2.0 - 5.0, 0.0, LOGGER_RACK_Z / 2.0 - 9.0);

    let mut pockets = Part::empty("workcell_calibration_environment_logger_pockets");
    for i in 0..LOGGER_COUNT {
        let x = centered_index(i, LOGGER_COUNT, LOGGER_PITCH);
        let logger_pocket = centered_cube(
            format!("workcell_calibration_environment_logger_body_pocket_{i}"),
            38.0,
            58.0,
            24.0,
        )
        .translate(x, -6.0, 12.0);
        let strap_slot = centered_cube(
            format!("workcell_calibration_environment_logger_strap_slot_{i}"),
            8.0,
            LOGGER_RACK_Y + 6.0,
            7.0,
        )
        .translate(x, 0.0, 24.0);
        let sensor_gland = centered_cylinder(
            format!("workcell_calibration_environment_sensor_probe_gland_{i}"),
            4.2 / 2.0,
            18.0,
            20,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, LOGGER_RACK_Y / 2.0 - 6.0, 28.0);
        pockets = pockets + logger_pocket + strap_slot + sensor_gland;
    }

    let mut mapping_pin_posts = Part::empty("workcell_calibration_environment_mapping_pin_posts");
    for (i, x) in [-74.0, -37.0, 0.0, 37.0, 74.0].iter().enumerate() {
        let post = centered_cylinder(
            format!("workcell_calibration_environment_map_pin_post_{i}"),
            4.0,
            16.0,
            24,
        )
        .translate(*x, -LOGGER_RACK_Y / 2.0 + 11.0, 17.0);
        let bore = centered_cylinder(
            format!("workcell_calibration_environment_map_pin_bore_{i}"),
            1.5,
            18.0,
            16,
        )
        .translate(*x, -LOGGER_RACK_Y / 2.0 + 11.0, 17.0);
        mapping_pin_posts = mapping_pin_posts + (post - bore);
    }

    base + back + side_left + side_right - pockets + mapping_pin_posts
}

fn clean_used_segregation_tray() -> Part {
    let body = centered_cube(
        "workcell_calibration_clean_used_segregation_tray_body",
        SEG_TRAY_X,
        SEG_TRAY_Y,
        SEG_TRAY_Z,
    );
    let clean_well = centered_cube(
        "workcell_calibration_clean_standards_well",
        SEG_TRAY_X - 32.0,
        44.0,
        SEG_TRAY_Z - 8.0,
    )
    .translate(0.0, 24.0, SEG_TRAY_Z / 2.0 - (SEG_TRAY_Z - 8.0) / 2.0 + 0.2);
    let used_well = centered_cube(
        "workcell_calibration_used_return_well",
        SEG_TRAY_X - 32.0,
        44.0,
        SEG_TRAY_Z - 8.0,
    )
    .translate(
        0.0,
        -32.0,
        SEG_TRAY_Z / 2.0 - (SEG_TRAY_Z - 8.0) / 2.0 + 0.2,
    );
    let divider_label_slot = centered_cube(
        "workcell_calibration_clean_used_divider_label_slot",
        SEG_TRAY_X - 46.0,
        5.0,
        4.0,
    )
    .translate(0.0, -4.0, SEG_TRAY_Z / 2.0 - 2.0);
    let used_drain = centered_cylinder("workcell_calibration_used_return_drain", 3.0, 22.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(SEG_TRAY_X / 2.0 - 24.0, -SEG_TRAY_Y / 2.0 + 12.0, 0.0);

    let biohazard_keepout = centered_cube(
        "workcell_calibration_used_side_quarantine_flag_land",
        SEG_TRAY_X - 40.0,
        12.0,
        4.0,
    )
    .translate(0.0, -SEG_TRAY_Y / 2.0 + 10.0, SEG_TRAY_Z / 2.0 + 2.0);
    let clean_cap_parking = clean_cap_parking_posts();

    body - clean_well - used_well - divider_label_slot - used_drain
        + biohazard_keepout
        + clean_cap_parking
}

fn clean_cap_parking_posts() -> Part {
    let mut posts = Part::empty("workcell_calibration_clean_cap_parking_posts");
    for (i, x) in [-45.0, -15.0, 15.0, 45.0].iter().enumerate() {
        let post = centered_cylinder(
            format!("workcell_calibration_clean_cap_parking_post_{i}"),
            4.6,
            10.0,
            24,
        )
        .translate(*x, SEG_TRAY_Y / 2.0 - 12.0, SEG_TRAY_Z / 2.0 + 5.0);
        let pilot = centered_cylinder(
            format!("workcell_calibration_clean_cap_parking_pilot_{i}"),
            1.6,
            12.0,
            16,
        )
        .translate(*x, SEG_TRAY_Y / 2.0 - 12.0, SEG_TRAY_Z / 2.0 + 5.0);
        posts = posts + (post - pilot);
    }
    posts
}

fn barcode_lot_label_lands() -> Part {
    let carrier = centered_cube(
        "workcell_calibration_barcode_lot_label_land_carrier",
        LABEL_LANDS_X,
        LABEL_LANDS_Y,
        LABEL_LANDS_Z,
    );
    let mut lands = Part::empty("workcell_calibration_barcode_lot_label_lands");
    for i in 0..LABEL_LAND_COUNT {
        let x = centered_index(i, LABEL_LAND_COUNT, LABEL_LAND_PITCH);
        let land = centered_cube(
            format!("workcell_calibration_barcode_lot_label_land_{i}"),
            52.0,
            20.0,
            2.0,
        )
        .translate(x, 0.0, LABEL_LANDS_Z / 2.0 + 1.0);
        let scanner_relief = centered_cube(
            format!("workcell_calibration_barcode_lot_scanner_relief_{i}"),
            46.0,
            1.6,
            1.4,
        )
        .translate(x, -8.6, LABEL_LANDS_Z / 2.0 + 1.7);
        lands = lands + (land - scanner_relief);
    }

    let mut clip_holes = Part::empty("workcell_calibration_label_land_clip_holes");
    for (i, x) in [
        -(LABEL_LANDS_X / 2.0 - 16.0),
        0.0,
        LABEL_LANDS_X / 2.0 - 16.0,
    ]
    .iter()
    .enumerate()
    {
        clip_holes = clip_holes
            + centered_cylinder(
                format!("workcell_calibration_label_land_clip_hole_{i}"),
                3.0 / 2.0,
                LABEL_LANDS_Z + 4.0,
                18,
            )
            .translate(*x, 0.0, 0.0);
    }

    carrier - clip_holes + lands
}

fn caddy_gripper_fiducials(name: &str) -> Part {
    let mut fiducials = Part::empty(format!("workcell_calibration_{name}_gripper_fiducials"));
    for (i, x) in [-32.0, 32.0].iter().enumerate() {
        let disc = centered_cylinder(
            format!("workcell_calibration_{name}_fiducial_disc_{i}"),
            5.0,
            2.0,
            32,
        )
        .translate(*x, 0.0, 13.0);
        let center = centered_cylinder(
            format!("workcell_calibration_{name}_fiducial_center_{i}"),
            1.2,
            3.0,
            18,
        )
        .translate(*x, 0.0, 13.0);
        fiducials = fiducials + (disc - center);
    }
    fiducials
}

fn panel_latch_tabs(name: &str) -> Part {
    let left = centered_cube(
        format!("workcell_calibration_{name}_left_latch_tab"),
        18.0,
        9.0,
        8.0,
    )
    .translate(-72.0, -52.0, -3.0);
    let right = centered_cube(
        format!("workcell_calibration_{name}_right_latch_tab"),
        18.0,
        9.0,
        8.0,
    )
    .translate(72.0, -52.0, -3.0);
    left + right
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_and_scoped_to_calibration_drawer() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 10);
        for path in OUTPUTS {
            assert!(path.starts_with("output/workcell_calibration_drawer_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn drawer_covers_required_workcell_qualification_articles() {
        assert_eq!(REQUIRED_FEATURES.len(), 8);
        assert!(REQUIRED_FEATURES.contains(&"flow_restrictor_standards"));
        assert!(REQUIRED_FEATURES.contains(&"pressure_leak_test_adapters"));
        assert!(REQUIRED_FEATURES.contains(&"teer_phantom_resistor_board"));
        assert!(REQUIRED_FEATURES.contains(&"ph_do_o2_calibration_standards"));
        assert!(REQUIRED_FEATURES.contains(&"imaging_slides_targets"));
        assert!(REQUIRED_FEATURES.contains(&"environmental_mapping_loggers"));
        assert!(REQUIRED_FEATURES.contains(&"barcode_lot_label_lands"));
        assert!(REQUIRED_FEATURES.contains(&"clean_used_segregation"));
    }

    #[test]
    fn inserts_fit_inside_drawer_rims() {
        assert!(fits_on_drawer(FLOW_POS, FLOW_CADDY_X, FLOW_CADDY_Y));
        assert!(fits_on_drawer(
            PRESSURE_POS,
            PRESSURE_PANEL_X,
            PRESSURE_PANEL_Y
        ));
        assert!(fits_on_drawer(TEER_POS, TEER_HOLDER_X, TEER_HOLDER_Y));
        assert!(fits_on_drawer(CHEM_POS, CHEM_BLOCK_X, CHEM_BLOCK_Y));
        assert!(fits_on_drawer(
            IMAGING_POS,
            IMAGING_CASSETTE_X,
            IMAGING_CASSETTE_Y
        ));
        assert!(fits_on_drawer(LOGGER_POS, LOGGER_RACK_X, LOGGER_RACK_Y));
        assert!(fits_on_drawer(SEG_POS, SEG_TRAY_X, SEG_TRAY_Y));
    }

    #[test]
    fn standard_counts_match_qualification_use_cases() {
        assert_eq!(FLOW_RESTRICTOR_COUNT, 8);
        assert_eq!(PRESSURE_ADAPTER_COUNT, 6);
        assert_eq!(CHEM_STANDARD_COUNT, 6);
        assert_eq!(LOGGER_COUNT, 3);
        assert!(FLOW_RESTRICTOR_PITCH > 24.0);
        assert!(CHEM_STANDARD_PITCH > 30.0);
    }

    #[test]
    fn clean_used_tray_preserves_physical_separation() {
        let clean_center_y = 24.0;
        let used_center_y = -32.0;
        let well_depth = 44.0;
        let gap = clean_center_y - well_depth / 2.0 - (used_center_y + well_depth / 2.0);
        assert!(gap >= 10.0);
        assert!(SEG_TRAY_Z >= 30.0);
    }

    #[test]
    fn front_label_lands_clear_drawer_rim_and_match_exports() {
        assert_eq!(LABEL_LAND_COUNT, OUTPUTS.len() - 1);
        assert!(LABEL_POS.1.abs() + LABEL_LANDS_Y / 2.0 <= DRAWER_Y / 2.0 - 8.0);
        assert!(LABEL_LANDS_X <= DRAWER_X - 70.0);
    }
}
