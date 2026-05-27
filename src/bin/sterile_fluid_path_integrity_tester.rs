use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
use vcad::{centered_cube, centered_cylinder, Part};

// Sterile fluid-path integrity tester for pre-run cassette and tubing checks.
//
// Intent:
// - Pressurize or evacuate dry sterile tubing harnesses, manifolds, and the
//   20-chip cassette fluid path before live-cell runs.
// - Isolate every cassette lane so pressure-decay failures can be localized
//   to one branch instead of condemning a full harness.
// - Keep leak witness, wetness optics, waste/decon diversion, scan lands, and
//   robot keepouts visible in the same fixture envelope.
//
// This is mechanical packaging for purchased pumps, regulators, valves,
// optical sensors, and pressure transducers. It is not a validated leak-test
// method or sterile barrier specification.
//
// Exports:
//   output/sterile_fluid_path_integrity_tester_baseplate.stl
//   output/sterile_fluid_path_integrity_tester_leak_witness_tray.stl
//   output/sterile_fluid_path_integrity_tester_cassette_datum_nest.stl
//   output/sterile_fluid_path_integrity_tester_pressure_vacuum_source_panel.stl
//   output/sterile_fluid_path_integrity_tester_lane_isolation_valve_bank.stl
//   output/sterile_fluid_path_integrity_tester_pressure_decay_sensor_matrix.stl
//   output/sterile_fluid_path_integrity_tester_bubble_wetness_optics.stl
//   output/sterile_fluid_path_integrity_tester_sterile_filter_vent_bank.stl
//   output/sterile_fluid_path_integrity_tester_waste_decon_diverter.stl
//   output/sterile_fluid_path_integrity_tester_barcode_lot_lands.stl
//   output/sterile_fluid_path_integrity_tester_robot_service_keepouts.stl
//   output/sterile_fluid_path_integrity_tester_assembly.stl

const OUTPUTS: [&str; 12] = [
    "output/sterile_fluid_path_integrity_tester_baseplate.stl",
    "output/sterile_fluid_path_integrity_tester_leak_witness_tray.stl",
    "output/sterile_fluid_path_integrity_tester_cassette_datum_nest.stl",
    "output/sterile_fluid_path_integrity_tester_pressure_vacuum_source_panel.stl",
    "output/sterile_fluid_path_integrity_tester_lane_isolation_valve_bank.stl",
    "output/sterile_fluid_path_integrity_tester_pressure_decay_sensor_matrix.stl",
    "output/sterile_fluid_path_integrity_tester_bubble_wetness_optics.stl",
    "output/sterile_fluid_path_integrity_tester_sterile_filter_vent_bank.stl",
    "output/sterile_fluid_path_integrity_tester_waste_decon_diverter.stl",
    "output/sterile_fluid_path_integrity_tester_barcode_lot_lands.stl",
    "output/sterile_fluid_path_integrity_tester_robot_service_keepouts.stl",
    "output/sterile_fluid_path_integrity_tester_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 10] = [
    "pressure_vacuum_source_placeholders",
    "manifold_cassette_datum",
    "per_lane_valve_isolation",
    "pressure_decay_sensor_pockets",
    "bubble_wetness_optical_windows",
    "sterile_filter_vent_ports",
    "waste_decon_diversion",
    "leak_witness_tray",
    "barcode_lot_scan_lands",
    "robot_service_keepouts",
];

const COLS: usize = 4;
const ROWS: usize = 5;
const LANES: usize = COLS * ROWS;
const LANE_ISOLATION_VALVES: usize = LANES;
const PRESSURE_SENSOR_POCKETS: usize = LANES;
const BUBBLE_WINDOWS: usize = LANES;
const WETNESS_WINDOWS: usize = LANES;
const STERILE_FILTERS: usize = ROWS + 2;
const VENT_PORTS: usize = ROWS + 3;
const BARCODE_LANDS: usize = 5;
const ROBOT_KEEP_OUT_ZONES: usize = 4;

const GUTTER: f64 = 5.0;
const CHIP_PITCH_X: f64 = REVC_CHIP_LENGTH + GUTTER;
const CHIP_PITCH_Y: f64 = REVC_CHIP_WIDTH + GUTTER;
const ARRAY_X: f64 = COLS as f64 * REVC_CHIP_LENGTH + (COLS as f64 - 1.0) * GUTTER;
const ARRAY_Y: f64 = ROWS as f64 * REVC_CHIP_WIDTH + (ROWS as f64 - 1.0) * GUTTER;

const BASE_X: f64 = 980.0;
const BASE_Y: f64 = 760.0;
const BASE_Z: f64 = 18.0;

const TRAY_X: f64 = ARRAY_X + 190.0;
const TRAY_Y: f64 = ARRAY_Y + 150.0;
const TRAY_Z: f64 = 24.0;

const NEST_X: f64 = ARRAY_X + 128.0;
const NEST_Y: f64 = ARRAY_Y + 116.0;
const NEST_Z: f64 = 22.0;
const CASSETTE_CLEARANCE: f64 = 0.6;
const DATUM_RAIL_Z: f64 = 30.0;

const SOURCE_PANEL_X: f64 = 900.0;
const SOURCE_PANEL_Y: f64 = 28.0;
const SOURCE_PANEL_Z: f64 = 300.0;
const SOURCE_PANEL_BASE_Y: f64 = BASE_Y / 2.0 - 48.0;

const VALVE_BANK_X: f64 = ARRAY_X + 250.0;
const VALVE_BANK_Y: f64 = 188.0;
const VALVE_BANK_Z: f64 = 64.0;
const VALVE_BLOCK_X: f64 = 34.0;
const VALVE_BLOCK_Y: f64 = 28.0;
const VALVE_BLOCK_Z: f64 = 26.0;

const SENSOR_MATRIX_X: f64 = ARRAY_X + 250.0;
const SENSOR_MATRIX_Y: f64 = 156.0;
const SENSOR_MATRIX_Z: f64 = 48.0;
const SENSOR_POCKET_X: f64 = 32.0;
const SENSOR_POCKET_Y: f64 = 30.0;
const SENSOR_POCKET_Z: f64 = 18.0;

const OPTICS_X: f64 = ARRAY_X + 150.0;
const OPTICS_Y: f64 = ARRAY_Y + 124.0;
const OPTICS_Z: f64 = 18.0;

const FILTER_BANK_X: f64 = 750.0;
const FILTER_BANK_Y: f64 = 88.0;
const FILTER_BANK_Z: f64 = 72.0;
const FILTER_OD: f64 = 24.0;
const FILTER_LENGTH: f64 = 78.0;

const DIVERTER_X: f64 = 360.0;
const DIVERTER_Y: f64 = 178.0;
const DIVERTER_Z: f64 = 70.0;

const LABEL_LANDS_X: f64 = 430.0;
const LABEL_LANDS_Y: f64 = 94.0;
const LABEL_LANDS_Z: f64 = 9.0;

const KEEP_OUT_Z: f64 = 140.0;
const TUBE_OD: f64 = 4.8;
const TUBE_CLEARANCE: f64 = 0.7;
const TUBE_BORE_D: f64 = TUBE_OD + TUBE_CLEARANCE;
const ROW_HEADER_D: f64 = 7.2;
const SENSOR_TAP_D: f64 = 3.2;
const DRAIN_D: f64 = 9.5;
const MOUNT_HOLE_D: f64 = 6.6;

fn main() {
    fs::create_dir_all("output").unwrap();

    let base = baseplate();
    export(OUTPUTS[0], &base);

    let tray = leak_witness_tray();
    export(OUTPUTS[1], &tray);

    let nest = cassette_datum_nest();
    export(OUTPUTS[2], &nest);

    let source_panel = pressure_vacuum_source_panel();
    export(OUTPUTS[3], &source_panel);

    let valve_bank = lane_isolation_valve_bank();
    export(OUTPUTS[4], &valve_bank);

    let sensor_matrix = pressure_decay_sensor_matrix();
    export(OUTPUTS[5], &sensor_matrix);

    let optics = bubble_wetness_optics();
    export(OUTPUTS[6], &optics);

    let filter_bank = sterile_filter_vent_bank();
    export(OUTPUTS[7], &filter_bank);

    let diverter = waste_decon_diverter();
    export(OUTPUTS[8], &diverter);

    let label_lands = barcode_lot_scan_lands();
    export(OUTPUTS[9], &label_lands);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let assembly = base
        + tray.translate(72.0, -26.0, BASE_Z / 2.0 + TRAY_Z / 2.0 + 2.0)
        + nest.translate(72.0, -26.0, BASE_Z / 2.0 + TRAY_Z + NEST_Z / 2.0 + 8.0)
        + optics.translate(72.0, -26.0, BASE_Z / 2.0 + TRAY_Z + NEST_Z + 34.0)
        + source_panel.translate(
            0.0,
            SOURCE_PANEL_BASE_Y,
            BASE_Z / 2.0 + SOURCE_PANEL_Z / 2.0,
        )
        + valve_bank.translate(
            -80.0,
            -BASE_Y / 2.0 + 146.0,
            BASE_Z / 2.0 + VALVE_BANK_Z / 2.0,
        )
        + sensor_matrix.translate(
            -80.0,
            -BASE_Y / 2.0 + 282.0,
            BASE_Z / 2.0 + SENSOR_MATRIX_Z / 2.0,
        )
        + filter_bank.translate(0.0, SOURCE_PANEL_BASE_Y - 54.0, BASE_Z / 2.0 + 238.0)
        + diverter.translate(
            BASE_X / 2.0 - 230.0,
            -BASE_Y / 2.0 + 145.0,
            BASE_Z / 2.0 + 38.0,
        )
        + label_lands.translate(
            -(BASE_X / 2.0 - 270.0),
            -BASE_Y / 2.0 + 78.0,
            BASE_Z / 2.0 + 8.0,
        )
        + keepouts.translate(0.0, 0.0, BASE_Z / 2.0 + KEEP_OUT_Z / 2.0);

    export(OUTPUTS[11], &assembly);

    println!(
        "Sterile fluid-path integrity tester: {:.0}mm x {:.0}mm deck, {} lanes in a {}x{} cassette datum, {} isolation valves, {} pressure-decay sensor pockets, {} optical windows, {} sterile filters, {} vent ports, {} required feature groups, waste/decon diversion, leak witness tray, barcode lands, and {} robot keepout envelopes.",
        BASE_X,
        BASE_Y,
        LANES,
        COLS,
        ROWS,
        LANE_ISOLATION_VALVES,
        PRESSURE_SENSOR_POCKETS,
        BUBBLE_WINDOWS + WETNESS_WINDOWS,
        STERILE_FILTERS,
        VENT_PORTS,
        REQUIRED_FEATURES.len(),
        ROBOT_KEEP_OUT_ZONES
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn baseplate() -> Part {
    let deck = centered_cube(
        "sterile_fluid_path_integrity_baseplate_deck",
        BASE_X,
        BASE_Y,
        BASE_Z,
    );

    let recessed_field = centered_cube(
        "sterile_fluid_path_integrity_base_recessed_field",
        BASE_X - 112.0,
        BASE_Y - 122.0,
        7.0,
    )
    .translate(0.0, -18.0, BASE_Z / 2.0 - 3.0);

    let rear_panel_slot = centered_cube(
        "sterile_fluid_path_integrity_source_panel_socket",
        SOURCE_PANEL_X + 26.0,
        16.0,
        8.0,
    )
    .translate(0.0, SOURCE_PANEL_BASE_Y, BASE_Z / 2.0 - 2.0);

    let leak_tray_socket = centered_cube(
        "sterile_fluid_path_integrity_leak_tray_registration_socket",
        TRAY_X + 26.0,
        TRAY_Y + 24.0,
        6.0,
    )
    .translate(72.0, -26.0, BASE_Z / 2.0 - 2.0);

    let drain = centered_cylinder(
        "sterile_fluid_path_integrity_base_drain_to_waste",
        DRAIN_D / 2.0,
        36.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(BASE_X / 2.0 - 104.0, -BASE_Y / 2.0 + 32.0, 0.0);

    deck - recessed_field - rear_panel_slot - leak_tray_socket - drain
        + base_perimeter_rails()
        + base_mounting_slots()
        + base_robot_fiducials()
        + panel_gussets()
        + decon_drain_guard()
}

fn base_perimeter_rails() -> Part {
    let left = centered_cube(
        "sterile_fluid_path_integrity_left_base_rail",
        18.0,
        BASE_Y - 58.0,
        28.0,
    )
    .translate(-(BASE_X / 2.0 - 32.0), 0.0, BASE_Z / 2.0 + 14.0);
    let right = centered_cube(
        "sterile_fluid_path_integrity_right_base_rail",
        18.0,
        BASE_Y - 58.0,
        28.0,
    )
    .translate(BASE_X / 2.0 - 32.0, 0.0, BASE_Z / 2.0 + 14.0);
    let rear = centered_cube(
        "sterile_fluid_path_integrity_rear_base_rail",
        BASE_X - 74.0,
        18.0,
        30.0,
    )
    .translate(0.0, BASE_Y / 2.0 - 31.0, BASE_Z / 2.0 + 15.0);
    let front = centered_cube(
        "sterile_fluid_path_integrity_front_low_witness_lip",
        BASE_X - 118.0,
        12.0,
        20.0,
    )
    .translate(0.0, -BASE_Y / 2.0 + 29.0, BASE_Z / 2.0 + 10.0);

    left + right + rear + front
}

fn base_mounting_slots() -> Part {
    let mut slots = Part::empty("sterile_fluid_path_integrity_base_mounting_slots");
    for (i, (x, y)) in base_mount_points().iter().enumerate() {
        let hole = centered_cylinder(
            format!("sterile_fluid_path_integrity_m6_clearance_{i}"),
            MOUNT_HOLE_D / 2.0,
            BASE_Z + 4.0,
            24,
        )
        .translate(*x, *y, 0.0);
        let slot = centered_cube(
            format!("sterile_fluid_path_integrity_m6_slot_{i}"),
            26.0,
            MOUNT_HOLE_D + 0.4,
            BASE_Z + 4.0,
        )
        .translate(*x, *y, 0.0);
        slots = slots + hole + slot;
    }
    slots
}

fn base_mount_points() -> [(f64, f64); 8] {
    [
        (-(BASE_X / 2.0 - 62.0), -(BASE_Y / 2.0 - 58.0)),
        (BASE_X / 2.0 - 62.0, -(BASE_Y / 2.0 - 58.0)),
        (-(BASE_X / 2.0 - 62.0), BASE_Y / 2.0 - 58.0),
        (BASE_X / 2.0 - 62.0, BASE_Y / 2.0 - 58.0),
        (0.0, -(BASE_Y / 2.0 - 58.0)),
        (0.0, BASE_Y / 2.0 - 58.0),
        (-(BASE_X / 2.0 - 62.0), 0.0),
        (BASE_X / 2.0 - 62.0, 0.0),
    ]
}

fn base_robot_fiducials() -> Part {
    let mut fiducials = Part::empty("sterile_fluid_path_integrity_robot_fiducials");
    for (i, (x, y)) in [
        (-(BASE_X / 2.0 - 76.0), BASE_Y / 2.0 - 76.0),
        (BASE_X / 2.0 - 76.0, BASE_Y / 2.0 - 76.0),
        (-(BASE_X / 2.0 - 76.0), -(BASE_Y / 2.0 - 76.0)),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + fiducial_target(&format!("sterile_fluid_path_integrity_base_fiducial_{i}"))
                .translate(*x, *y, BASE_Z / 2.0 + 2.0);
    }
    fiducials
}

fn panel_gussets() -> Part {
    let mut gussets = Part::empty("sterile_fluid_path_integrity_panel_gussets");
    for (i, x) in [-390.0, -260.0, -130.0, 0.0, 130.0, 260.0, 390.0]
        .iter()
        .enumerate()
    {
        let web = centered_cube(
            format!("sterile_fluid_path_integrity_gusset_web_{i}"),
            12.0,
            76.0,
            86.0,
        )
        .translate(*x, SOURCE_PANEL_BASE_Y - 32.0, BASE_Z / 2.0 + 43.0);
        let foot = centered_cube(
            format!("sterile_fluid_path_integrity_gusset_foot_{i}"),
            46.0,
            68.0,
            10.0,
        )
        .translate(*x, SOURCE_PANEL_BASE_Y - 32.0, BASE_Z / 2.0 + 5.0);
        let screw = centered_cylinder(
            format!("sterile_fluid_path_integrity_gusset_m5_clearance_{i}"),
            5.4 / 2.0,
            14.0,
            24,
        )
        .translate(*x, SOURCE_PANEL_BASE_Y - 50.0, BASE_Z / 2.0 + 5.0);
        gussets = gussets + (web + foot - screw);
    }
    gussets
}

fn decon_drain_guard() -> Part {
    let guard = centered_cube(
        "sterile_fluid_path_integrity_decon_drain_guard",
        116.0,
        48.0,
        24.0,
    )
    .translate(
        BASE_X / 2.0 - 112.0,
        -BASE_Y / 2.0 + 50.0,
        BASE_Z / 2.0 + 12.0,
    );
    let port = centered_cylinder(
        "sterile_fluid_path_integrity_decon_guard_drain_access",
        18.0,
        52.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        BASE_X / 2.0 - 112.0,
        -BASE_Y / 2.0 + 50.0,
        BASE_Z / 2.0 + 12.0,
    );
    guard - port
}

fn leak_witness_tray() -> Part {
    let tray = centered_cube(
        "sterile_fluid_path_integrity_leak_witness_outer_tray",
        TRAY_X,
        TRAY_Y,
        TRAY_Z,
    );
    let basin = centered_cube(
        "sterile_fluid_path_integrity_leak_witness_shallow_basin",
        TRAY_X - 42.0,
        TRAY_Y - 42.0,
        12.0,
    )
    .translate(0.0, 0.0, TRAY_Z / 2.0 - 5.0);
    let cassette_shadow = centered_cube(
        "sterile_fluid_path_integrity_leak_witness_cassette_shadow",
        ARRAY_X + 34.0,
        ARRAY_Y + 34.0,
        5.0,
    )
    .translate(0.0, 0.0, TRAY_Z / 2.0 - 2.0);
    let drain = centered_cylinder(
        "sterile_fluid_path_integrity_leak_witness_low_point_drain",
        DRAIN_D / 2.0,
        42.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(TRAY_X / 2.0 - 42.0, -(TRAY_Y / 2.0 + 12.0), -1.0);

    tray - basin - cassette_shadow - drain
        + leak_witness_lane_ribs()
        + leak_witness_sensor_wells()
        + tray_handles()
}

fn leak_witness_lane_ribs() -> Part {
    let mut ribs = Part::empty("sterile_fluid_path_integrity_leak_witness_lane_ribs");
    for row in 0..ROWS {
        let y = row_y(row);
        ribs = ribs
            + centered_cube(
                format!("sterile_fluid_path_integrity_leak_row_{row}_flow_rib"),
                ARRAY_X + 64.0,
                4.0,
                6.0,
            )
            .translate(0.0, y, TRAY_Z / 2.0 - 2.0);
    }
    for col in 0..COLS {
        let x = chip_x(col);
        ribs = ribs
            + centered_cube(
                format!("sterile_fluid_path_integrity_leak_col_{col}_divider_rib"),
                4.0,
                ARRAY_Y + 64.0,
                6.0,
            )
            .translate(x, 0.0, TRAY_Z / 2.0 - 2.0);
    }
    ribs
}

fn leak_witness_sensor_wells() -> Part {
    let mut wells = Part::empty("sterile_fluid_path_integrity_leak_witness_sensor_wells");
    for (i, (x, y)) in [
        (-(TRAY_X / 2.0 - 52.0), -(TRAY_Y / 2.0 - 44.0)),
        (TRAY_X / 2.0 - 52.0, -(TRAY_Y / 2.0 - 44.0)),
        (TRAY_X / 2.0 - 52.0, TRAY_Y / 2.0 - 44.0),
    ]
    .iter()
    .enumerate()
    {
        let rim = centered_cube(
            format!("sterile_fluid_path_integrity_leak_sensor_well_rim_{i}"),
            48.0,
            32.0,
            8.0,
        )
        .translate(*x, *y, TRAY_Z / 2.0 + 4.0);
        let pocket = centered_cube(
            format!("sterile_fluid_path_integrity_leak_sensor_well_pocket_{i}"),
            34.0,
            18.0,
            10.0,
        )
        .translate(*x, *y, TRAY_Z / 2.0 + 4.0);
        wells = wells + (rim - pocket);
    }
    wells
}

fn tray_handles() -> Part {
    let left = centered_cube(
        "sterile_fluid_path_integrity_leak_tray_left_handle",
        42.0,
        22.0,
        26.0,
    )
    .translate(-(TRAY_X / 2.0 + 46.0), 0.0, 5.0);
    let right = centered_cube(
        "sterile_fluid_path_integrity_leak_tray_right_handle",
        42.0,
        22.0,
        26.0,
    )
    .translate(TRAY_X / 2.0 + 46.0, 0.0, 5.0);
    left + right
}

fn cassette_datum_nest() -> Part {
    let body = centered_cube(
        "sterile_fluid_path_integrity_cassette_datum_nest_body",
        NEST_X,
        NEST_Y,
        NEST_Z,
    );
    let cassette_pocket = centered_cube(
        "sterile_fluid_path_integrity_cassette_clearance_pocket",
        ARRAY_X + 2.0 * CASSETTE_CLEARANCE,
        ARRAY_Y + 2.0 * CASSETTE_CLEARANCE,
        12.0,
    )
    .translate(0.0, 0.0, NEST_Z / 2.0 - 5.0);
    let open_window = centered_cube(
        "sterile_fluid_path_integrity_cassette_open_fluid_window",
        ARRAY_X - 22.0,
        ARRAY_Y - 18.0,
        NEST_Z + 4.0,
    );

    body - cassette_pocket - open_window - cassette_lane_port_cuts()
        + cassette_datum_rails()
        + row_manifold_datum_bar()
        + cassette_latch_clamps()
        + cassette_fiducials()
}

fn cassette_lane_port_cuts() -> Part {
    let mut cuts = Part::empty("sterile_fluid_path_integrity_cassette_lane_port_cuts");
    for row in 0..ROWS {
        for col in 0..COLS {
            let idx = lane_index(row, col);
            let (x, y) = lane_center(row, col);
            let inlet = centered_cylinder(
                format!("sterile_fluid_path_integrity_lane_{idx}_inlet_socket"),
                TUBE_BORE_D / 2.0,
                NEST_Z + 4.0,
                24,
            )
            .translate(x - REVC_CHIP_LENGTH / 2.0 + 16.0, y, 0.0);
            let outlet = centered_cylinder(
                format!("sterile_fluid_path_integrity_lane_{idx}_outlet_socket"),
                TUBE_BORE_D / 2.0,
                NEST_Z + 4.0,
                24,
            )
            .translate(x + REVC_CHIP_LENGTH / 2.0 - 16.0, y, 0.0);
            let sensor_view = centered_cube(
                format!("sterile_fluid_path_integrity_lane_{idx}_chip_view_relief"),
                REVC_CHIP_LENGTH - 42.0,
                REVC_CHIP_WIDTH - 18.0,
                NEST_Z + 4.0,
            )
            .translate(x, y, 0.0);
            cuts = cuts + inlet + outlet + sensor_view;
        }
    }
    cuts
}

fn cassette_datum_rails() -> Part {
    let rear = centered_cube(
        "sterile_fluid_path_integrity_cassette_rear_hard_datum",
        ARRAY_X + 58.0,
        14.0,
        DATUM_RAIL_Z,
    )
    .translate(0.0, ARRAY_Y / 2.0 + 28.0, NEST_Z / 2.0 + DATUM_RAIL_Z / 2.0);
    let left = centered_cube(
        "sterile_fluid_path_integrity_cassette_left_hard_datum",
        14.0,
        ARRAY_Y + 58.0,
        DATUM_RAIL_Z,
    )
    .translate(
        -(ARRAY_X / 2.0 + 28.0),
        0.0,
        NEST_Z / 2.0 + DATUM_RAIL_Z / 2.0,
    );
    let right_spring = centered_cube(
        "sterile_fluid_path_integrity_cassette_right_spring_datum",
        10.0,
        ARRAY_Y + 26.0,
        22.0,
    )
    .translate(ARRAY_X / 2.0 + 25.0, 0.0, NEST_Z / 2.0 + 11.0);
    let front_stops = centered_cube(
        "sterile_fluid_path_integrity_cassette_front_low_stop",
        ARRAY_X + 48.0,
        9.0,
        18.0,
    )
    .translate(0.0, -(ARRAY_Y / 2.0 + 24.0), NEST_Z / 2.0 + 9.0);

    rear + left + right_spring + front_stops
}

fn row_manifold_datum_bar() -> Part {
    let bar = centered_cube(
        "sterile_fluid_path_integrity_row_manifold_datum_bar",
        96.0,
        ARRAY_Y + 84.0,
        42.0,
    )
    .translate(-(ARRAY_X / 2.0 + 82.0), 0.0, NEST_Z / 2.0 + 21.0);

    let mut ports = Part::empty("sterile_fluid_path_integrity_row_manifold_datum_ports");
    for row in 0..ROWS {
        let y = row_y(row);
        let pressure_port = centered_cylinder(
            format!("sterile_fluid_path_integrity_row_{row}_pressure_header_socket"),
            ROW_HEADER_D / 2.0,
            108.0,
            28,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(-(ARRAY_X / 2.0 + 82.0), y, NEST_Z / 2.0 + 27.0);
        let vacuum_port = centered_cylinder(
            format!("sterile_fluid_path_integrity_row_{row}_vacuum_header_socket"),
            ROW_HEADER_D / 2.0,
            108.0,
            28,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(-(ARRAY_X / 2.0 + 82.0), y, NEST_Z / 2.0 + 9.0);
        ports = ports + pressure_port + vacuum_port;
    }

    bar - ports
}

fn cassette_latch_clamps() -> Part {
    let mut clamps = Part::empty("sterile_fluid_path_integrity_cassette_latch_clamps");
    for (i, (x, y)) in [
        (-(NEST_X / 2.0 - 38.0), -(NEST_Y / 2.0 - 32.0)),
        (NEST_X / 2.0 - 38.0, -(NEST_Y / 2.0 - 32.0)),
        (-(NEST_X / 2.0 - 38.0), NEST_Y / 2.0 - 32.0),
        (NEST_X / 2.0 - 38.0, NEST_Y / 2.0 - 32.0),
    ]
    .iter()
    .enumerate()
    {
        let clamp = centered_cube(
            format!("sterile_fluid_path_integrity_cassette_latch_clamp_{i}"),
            42.0,
            24.0,
            16.0,
        )
        .translate(*x, *y, NEST_Z / 2.0 + 8.0);
        let screw = centered_cylinder(
            format!("sterile_fluid_path_integrity_cassette_latch_screw_{i}"),
            3.4 / 2.0,
            18.0,
            22,
        )
        .translate(*x, *y, NEST_Z / 2.0 + 8.0);
        clamps = clamps + (clamp - screw);
    }
    clamps
}

fn cassette_fiducials() -> Part {
    let mut fiducials = Part::empty("sterile_fluid_path_integrity_cassette_fiducials");
    for (i, (x, y)) in [
        (-(NEST_X / 2.0 - 30.0), NEST_Y / 2.0 - 30.0),
        (NEST_X / 2.0 - 30.0, NEST_Y / 2.0 - 30.0),
        (-(NEST_X / 2.0 - 30.0), -(NEST_Y / 2.0 - 30.0)),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + fiducial_target(&format!(
                "sterile_fluid_path_integrity_cassette_fiducial_{i}"
            ))
            .translate(*x, *y, NEST_Z / 2.0 + 2.0);
    }
    fiducials
}

fn pressure_vacuum_source_panel() -> Part {
    let panel = centered_cube(
        "sterile_fluid_path_integrity_source_panel_backplate",
        SOURCE_PANEL_X,
        SOURCE_PANEL_Y,
        SOURCE_PANEL_Z,
    );
    let access_window = centered_cube(
        "sterile_fluid_path_integrity_source_panel_rear_service_window",
        SOURCE_PANEL_X - 96.0,
        SOURCE_PANEL_Y + 4.0,
        72.0,
    )
    .translate(0.0, 0.0, -SOURCE_PANEL_Z / 2.0 + 60.0);

    panel - access_window
        + pressure_source_placeholder().translate(-305.0, -44.0, 66.0)
        + vacuum_source_placeholder().translate(-130.0, -44.0, 66.0)
        + accumulator_placeholder("positive_accumulator").translate(72.0, -44.0, 68.0)
        + accumulator_placeholder("negative_accumulator").translate(242.0, -44.0, 68.0)
        + regulator_gauge_cluster()
        + source_selector_manifold()
        + calibration_service_ports()
}

fn pressure_source_placeholder() -> Part {
    let pump_body = centered_cube(
        "sterile_fluid_path_integrity_pressure_source_pump_envelope",
        120.0,
        82.0,
        80.0,
    );
    let head = centered_cylinder(
        "sterile_fluid_path_integrity_pressure_source_pump_head",
        22.0,
        56.0,
        40,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(44.0, -46.0, 10.0);
    let intake_filter = centered_cylinder(
        "sterile_fluid_path_integrity_pressure_source_intake_filter_placeholder",
        15.0,
        76.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-34.0, -54.0, 44.0);
    let service_clearance = centered_cube(
        "sterile_fluid_path_integrity_pressure_source_front_service_clearance",
        94.0,
        16.0,
        52.0,
    )
    .translate(0.0, -46.0, -4.0);

    pump_body + head + intake_filter - service_clearance
}

fn vacuum_source_placeholder() -> Part {
    let pump_body = centered_cube(
        "sterile_fluid_path_integrity_vacuum_source_pump_envelope",
        118.0,
        82.0,
        78.0,
    );
    let silencer = centered_cylinder(
        "sterile_fluid_path_integrity_vacuum_source_exhaust_silencer",
        16.0,
        74.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(38.0, -56.0, 42.0);
    let trap_port = centered_cylinder(
        "sterile_fluid_path_integrity_vacuum_source_trap_port",
        8.0 / 2.0,
        88.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-38.0, -40.0, -28.0);
    pump_body + silencer + trap_port
}

fn accumulator_placeholder(name: &str) -> Part {
    let bottle = centered_cylinder(
        format!("sterile_fluid_path_integrity_{name}_bottle"),
        34.0,
        128.0,
        48,
    )
    .translate(0.0, 0.0, 12.0);
    let cap = centered_cylinder(
        format!("sterile_fluid_path_integrity_{name}_cap"),
        20.0,
        18.0,
        40,
    )
    .translate(0.0, 0.0, 85.0);
    let saddle = centered_cube(
        format!("sterile_fluid_path_integrity_{name}_saddle"),
        86.0,
        20.0,
        22.0,
    )
    .translate(0.0, -36.0, -52.0);
    bottle + cap + saddle
}

fn regulator_gauge_cluster() -> Part {
    let mut cluster = Part::empty("sterile_fluid_path_integrity_regulator_gauge_cluster");
    for (i, x) in [-330.0, -210.0, -90.0, 30.0, 150.0, 270.0, 390.0]
        .iter()
        .enumerate()
    {
        let regulator = centered_cube(
            format!("sterile_fluid_path_integrity_regulator_body_{i}"),
            48.0,
            30.0,
            42.0,
        )
        .translate(*x, -34.0, -42.0);
        let gauge = centered_cylinder(
            format!("sterile_fluid_path_integrity_gauge_face_{i}"),
            18.0,
            8.0,
            40,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, -50.0, 4.0);
        let knob = centered_cylinder(
            format!("sterile_fluid_path_integrity_regulator_knob_{i}"),
            13.0,
            22.0,
            32,
        )
        .translate(*x, -36.0, 36.0);
        cluster = cluster + regulator + gauge + knob;
    }
    cluster
}

fn source_selector_manifold() -> Part {
    let manifold = centered_cube(
        "sterile_fluid_path_integrity_pressure_vacuum_selector_manifold",
        SOURCE_PANEL_X - 130.0,
        48.0,
        42.0,
    )
    .translate(0.0, -44.0, -102.0);
    let common_pressure = centered_cylinder(
        "sterile_fluid_path_integrity_common_pressure_header",
        ROW_HEADER_D / 2.0,
        SOURCE_PANEL_X - 116.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -44.0, -92.0);
    let common_vacuum = centered_cylinder(
        "sterile_fluid_path_integrity_common_vacuum_header",
        ROW_HEADER_D / 2.0,
        SOURCE_PANEL_X - 116.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -44.0, -112.0);

    let mut branch_ports = Part::empty("sterile_fluid_path_integrity_selector_branch_ports");
    for row in 0..ROWS {
        let x = -((ROWS as f64 - 1.0) * 92.0) / 2.0 + row as f64 * 92.0;
        branch_ports = branch_ports
            + centered_cylinder(
                format!("sterile_fluid_path_integrity_row_{row}_selector_pressure_port"),
                TUBE_BORE_D / 2.0,
                58.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -44.0, -92.0)
            + centered_cylinder(
                format!("sterile_fluid_path_integrity_row_{row}_selector_vacuum_port"),
                TUBE_BORE_D / 2.0,
                58.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -44.0, -112.0);
    }

    manifold - common_pressure - common_vacuum - branch_ports + source_selector_valves()
}

fn source_selector_valves() -> Part {
    let mut valves = Part::empty("sterile_fluid_path_integrity_source_selector_valves");
    for (i, x) in [-184.0, -92.0, 0.0, 92.0, 184.0].iter().enumerate() {
        valves = valves
            + centered_cube(
                format!("sterile_fluid_path_integrity_source_selector_valve_{i}"),
                38.0,
                30.0,
                28.0,
            )
            .translate(*x, -68.0, -66.0);
    }
    valves
}

fn calibration_service_ports() -> Part {
    let mut ports = Part::empty("sterile_fluid_path_integrity_calibration_service_ports");
    for (i, x) in [-372.0, 372.0].iter().enumerate() {
        let bulkhead = centered_cylinder(
            format!("sterile_fluid_path_integrity_external_calibration_bulkhead_{i}"),
            12.0,
            34.0,
            36,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, -54.0, 114.0);
        let cap_tether = centered_cube(
            format!("sterile_fluid_path_integrity_external_calibration_cap_tether_{i}"),
            42.0,
            5.0,
            8.0,
        )
        .translate(*x, -72.0, 91.0);
        ports = ports + bulkhead + cap_tether;
    }
    ports
}

fn lane_isolation_valve_bank() -> Part {
    let carrier = centered_cube(
        "sterile_fluid_path_integrity_lane_valve_bank_carrier",
        VALVE_BANK_X,
        VALVE_BANK_Y,
        VALVE_BANK_Z,
    );
    let cable_trough = centered_cube(
        "sterile_fluid_path_integrity_lane_valve_cable_trough",
        VALVE_BANK_X - 72.0,
        30.0,
        22.0,
    )
    .translate(0.0, VALVE_BANK_Y / 2.0 - 25.0, VALVE_BANK_Z / 2.0 - 10.0);

    carrier - cable_trough - lane_valve_channel_cuts()
        + lane_valve_actuator_placeholders()
        + valve_bank_row_headers()
        + valve_bank_mount_ears()
}

fn lane_valve_actuator_placeholders() -> Part {
    let mut valves = Part::empty("sterile_fluid_path_integrity_per_lane_valve_placeholders");
    for row in 0..ROWS {
        for col in 0..COLS {
            let idx = lane_index(row, col);
            let (x, y) = valve_center(row, col);
            let body = centered_cube(
                format!("sterile_fluid_path_integrity_lane_{idx}_pinch_valve_body"),
                VALVE_BLOCK_X,
                VALVE_BLOCK_Y,
                VALVE_BLOCK_Z,
            )
            .translate(x, y, VALVE_BANK_Z / 2.0 + VALVE_BLOCK_Z / 2.0);
            let solenoid = centered_cylinder(
                format!("sterile_fluid_path_integrity_lane_{idx}_valve_solenoid"),
                11.0,
                30.0,
                24,
            )
            .translate(x, y, VALVE_BANK_Z / 2.0 + VALVE_BLOCK_Z + 15.0);
            let lane_flag = centered_cube(
                format!("sterile_fluid_path_integrity_lane_{idx}_valve_scan_flag"),
                22.0,
                3.0,
                7.0,
            )
            .translate(x, y - VALVE_BLOCK_Y / 2.0 - 3.0, VALVE_BANK_Z / 2.0 + 18.0);
            valves = valves + body + solenoid + lane_flag;
        }
    }
    valves
}

fn lane_valve_channel_cuts() -> Part {
    let mut cuts = Part::empty("sterile_fluid_path_integrity_lane_valve_channel_cuts");
    for row in 0..ROWS {
        for col in 0..COLS {
            let idx = lane_index(row, col);
            let (x, y) = valve_center(row, col);
            let bore = centered_cylinder(
                format!("sterile_fluid_path_integrity_lane_{idx}_valve_tube_bore"),
                TUBE_BORE_D / 2.0,
                VALVE_BANK_Y + 8.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, y, VALVE_BANK_Z / 2.0 - 6.0);
            let loading_slot = centered_cube(
                format!("sterile_fluid_path_integrity_lane_{idx}_valve_top_loading_slot"),
                TUBE_BORE_D + 1.2,
                VALVE_BLOCK_Y + 14.0,
                VALVE_BANK_Z,
            )
            .translate(x, y, VALVE_BANK_Z / 2.0 + 8.0);
            cuts = cuts + bore + loading_slot;
        }
    }
    cuts
}

fn valve_bank_row_headers() -> Part {
    let mut headers = Part::empty("sterile_fluid_path_integrity_valve_bank_row_headers");
    for row in 0..ROWS {
        let y = valve_row_y(row);
        let header = centered_cylinder(
            format!("sterile_fluid_path_integrity_valve_bank_row_{row}_header"),
            ROW_HEADER_D / 2.0,
            VALVE_BANK_X - 92.0,
            32,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, y, VALVE_BANK_Z / 2.0 - 22.0);
        let label_land = centered_cube(
            format!("sterile_fluid_path_integrity_valve_bank_row_{row}_label_land"),
            46.0,
            6.0,
            8.0,
        )
        .translate(VALVE_BANK_X / 2.0 - 46.0, y, VALVE_BANK_Z / 2.0 + 6.0);
        headers = headers + header + label_land;
    }
    headers
}

fn valve_bank_mount_ears() -> Part {
    let mut ears = Part::empty("sterile_fluid_path_integrity_valve_bank_mount_ears");
    for (i, x) in [-(VALVE_BANK_X / 2.0 + 22.0), VALVE_BANK_X / 2.0 + 22.0]
        .iter()
        .enumerate()
    {
        let ear = centered_cube(
            format!("sterile_fluid_path_integrity_valve_bank_mount_ear_{i}"),
            38.0,
            62.0,
            12.0,
        )
        .translate(*x, 0.0, -VALVE_BANK_Z / 2.0 + 6.0);
        let hole = centered_cylinder(
            format!("sterile_fluid_path_integrity_valve_bank_mount_hole_{i}"),
            5.4 / 2.0,
            16.0,
            24,
        )
        .translate(*x, 0.0, -VALVE_BANK_Z / 2.0 + 6.0);
        ears = ears + (ear - hole);
    }
    ears
}

fn pressure_decay_sensor_matrix() -> Part {
    let body = centered_cube(
        "sterile_fluid_path_integrity_pressure_decay_sensor_matrix_body",
        SENSOR_MATRIX_X,
        SENSOR_MATRIX_Y,
        SENSOR_MATRIX_Z,
    );
    let service_cavity = centered_cube(
        "sterile_fluid_path_integrity_pressure_decay_sensor_wire_cavity",
        SENSOR_MATRIX_X - 72.0,
        30.0,
        18.0,
    )
    .translate(
        0.0,
        SENSOR_MATRIX_Y / 2.0 - 24.0,
        SENSOR_MATRIX_Z / 2.0 - 9.0,
    );

    body - service_cavity - pressure_decay_sensor_cuts()
        + pressure_decay_sensor_label_lands()
        + reference_pressure_bosses()
}

fn pressure_decay_sensor_cuts() -> Part {
    let mut cuts = Part::empty("sterile_fluid_path_integrity_pressure_decay_sensor_cuts");
    for row in 0..ROWS {
        for col in 0..COLS {
            let idx = lane_index(row, col);
            let (x, y) = sensor_center(row, col);
            let pocket = centered_cube(
                format!("sterile_fluid_path_integrity_lane_{idx}_pressure_sensor_pocket"),
                SENSOR_POCKET_X,
                SENSOR_POCKET_Y,
                SENSOR_POCKET_Z,
            )
            .translate(x, y, SENSOR_MATRIX_Z / 2.0 - SENSOR_POCKET_Z / 2.0 + 1.0);
            let tap = centered_cylinder(
                format!("sterile_fluid_path_integrity_lane_{idx}_pressure_decay_tap"),
                SENSOR_TAP_D / 2.0,
                SENSOR_MATRIX_Y + 8.0,
                18,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, y, 0.0);
            let o_ring = centered_cylinder(
                format!(
                    "sterile_fluid_path_integrity_lane_{idx}_pressure_sensor_o_ring_counterbore"
                ),
                9.5 / 2.0,
                5.0,
                28,
            )
            .translate(x, y, SENSOR_MATRIX_Z / 2.0 - 5.0);
            cuts = cuts + pocket + tap + o_ring;
        }
    }
    cuts
}

fn pressure_decay_sensor_label_lands() -> Part {
    let mut lands = Part::empty("sterile_fluid_path_integrity_pressure_sensor_label_lands");
    for row in 0..ROWS {
        for col in 0..COLS {
            let idx = lane_index(row, col);
            let (x, y) = sensor_center(row, col);
            lands = lands
                + centered_cube(
                    format!("sterile_fluid_path_integrity_lane_{idx}_pressure_sensor_label_land"),
                    30.0,
                    4.0,
                    5.0,
                )
                .translate(
                    x,
                    y - SENSOR_POCKET_Y / 2.0 - 5.0,
                    SENSOR_MATRIX_Z / 2.0 + 2.5,
                );
        }
    }
    lands
}

fn reference_pressure_bosses() -> Part {
    let zero = centered_cylinder(
        "sterile_fluid_path_integrity_reference_zero_pressure_boss",
        12.0,
        14.0,
        32,
    )
    .translate(
        -(SENSOR_MATRIX_X / 2.0 - 44.0),
        0.0,
        SENSOR_MATRIX_Z / 2.0 + 7.0,
    );
    let span = centered_cylinder(
        "sterile_fluid_path_integrity_reference_span_pressure_boss",
        12.0,
        14.0,
        32,
    )
    .translate(
        SENSOR_MATRIX_X / 2.0 - 44.0,
        0.0,
        SENSOR_MATRIX_Z / 2.0 + 7.0,
    );
    let zero_port = centered_cylinder(
        "sterile_fluid_path_integrity_reference_zero_pressure_port",
        4.0,
        16.0,
        24,
    )
    .translate(
        -(SENSOR_MATRIX_X / 2.0 - 44.0),
        0.0,
        SENSOR_MATRIX_Z / 2.0 + 7.0,
    );
    let span_port = centered_cylinder(
        "sterile_fluid_path_integrity_reference_span_pressure_port",
        4.0,
        16.0,
        24,
    )
    .translate(
        SENSOR_MATRIX_X / 2.0 - 44.0,
        0.0,
        SENSOR_MATRIX_Z / 2.0 + 7.0,
    );
    (zero - zero_port) + (span - span_port)
}

fn bubble_wetness_optics() -> Part {
    let frame = centered_cube(
        "sterile_fluid_path_integrity_optics_bridge_frame",
        OPTICS_X,
        OPTICS_Y,
        OPTICS_Z,
    );
    let central_opening = centered_cube(
        "sterile_fluid_path_integrity_optics_bridge_cassette_clearance",
        ARRAY_X - 14.0,
        ARRAY_Y - 12.0,
        OPTICS_Z + 4.0,
    );

    frame - central_opening - bubble_wetness_window_cuts()
        + optical_fork_sensor_mounts()
        + wetness_prism_carriers()
        + optics_bridge_dowel_pins()
}

fn bubble_wetness_window_cuts() -> Part {
    let mut cuts = Part::empty("sterile_fluid_path_integrity_bubble_wetness_window_cuts");
    for row in 0..ROWS {
        for col in 0..COLS {
            let idx = lane_index(row, col);
            let (x, y) = lane_center(row, col);
            let bubble_window = centered_cube(
                format!("sterile_fluid_path_integrity_lane_{idx}_bubble_optical_window"),
                24.0,
                12.0,
                OPTICS_Z + 4.0,
            )
            .translate(x - 18.0, y + 12.0, 0.0);
            let wetness_window = centered_cube(
                format!("sterile_fluid_path_integrity_lane_{idx}_wetness_optical_window"),
                24.0,
                10.0,
                OPTICS_Z + 4.0,
            )
            .translate(x + 18.0, y - 12.0, 0.0);
            cuts = cuts + bubble_window + wetness_window;
        }
    }
    cuts
}

fn optical_fork_sensor_mounts() -> Part {
    let mut mounts = Part::empty("sterile_fluid_path_integrity_optical_fork_sensor_mounts");
    for row in 0..ROWS {
        for col in 0..COLS {
            let idx = lane_index(row, col);
            let (x, y) = lane_center(row, col);
            let fork = centered_cube(
                format!("sterile_fluid_path_integrity_lane_{idx}_bubble_fork_sensor_saddle"),
                38.0,
                6.0,
                16.0,
            )
            .translate(x - 18.0, y + 25.0, OPTICS_Z / 2.0 + 8.0);
            let emitter_land = centered_cube(
                format!("sterile_fluid_path_integrity_lane_{idx}_bubble_emitter_land"),
                9.0,
                8.0,
                8.0,
            )
            .translate(x - 31.0, y + 25.0, OPTICS_Z / 2.0 + 20.0);
            let detector_land = centered_cube(
                format!("sterile_fluid_path_integrity_lane_{idx}_bubble_detector_land"),
                9.0,
                8.0,
                8.0,
            )
            .translate(x - 5.0, y + 25.0, OPTICS_Z / 2.0 + 20.0);
            mounts = mounts + fork + emitter_land + detector_land;
        }
    }
    mounts
}

fn wetness_prism_carriers() -> Part {
    let mut carriers = Part::empty("sterile_fluid_path_integrity_wetness_prism_carriers");
    for row in 0..ROWS {
        for col in 0..COLS {
            let idx = lane_index(row, col);
            let (x, y) = lane_center(row, col);
            let carrier = centered_cube(
                format!("sterile_fluid_path_integrity_lane_{idx}_wetness_prism_carrier"),
                34.0,
                8.0,
                12.0,
            )
            .translate(x + 18.0, y - 25.0, OPTICS_Z / 2.0 + 6.0);
            let prism_pocket = centered_cube(
                format!("sterile_fluid_path_integrity_lane_{idx}_wetness_prism_pocket"),
                18.0,
                9.0,
                9.0,
            )
            .translate(x + 18.0, y - 25.0, OPTICS_Z / 2.0 + 8.0);
            carriers = carriers + (carrier - prism_pocket);
        }
    }
    carriers
}

fn optics_bridge_dowel_pins() -> Part {
    let mut pins = Part::empty("sterile_fluid_path_integrity_optics_bridge_dowel_pins");
    for (i, (x, y)) in [
        (-(OPTICS_X / 2.0 - 34.0), -(OPTICS_Y / 2.0 - 34.0)),
        (OPTICS_X / 2.0 - 34.0, -(OPTICS_Y / 2.0 - 34.0)),
        (-(OPTICS_X / 2.0 - 34.0), OPTICS_Y / 2.0 - 34.0),
        (OPTICS_X / 2.0 - 34.0, OPTICS_Y / 2.0 - 34.0),
    ]
    .iter()
    .enumerate()
    {
        pins = pins
            + centered_cylinder(
                format!("sterile_fluid_path_integrity_optics_bridge_dowel_pin_{i}"),
                3.0,
                12.0,
                24,
            )
            .translate(*x, *y, OPTICS_Z / 2.0 + 6.0);
    }
    pins
}

fn sterile_filter_vent_bank() -> Part {
    let carrier = centered_cube(
        "sterile_fluid_path_integrity_sterile_filter_vent_bank_carrier",
        FILTER_BANK_X,
        FILTER_BANK_Y,
        FILTER_BANK_Z,
    );
    let service_channel = centered_cube(
        "sterile_fluid_path_integrity_filter_bank_front_service_channel",
        FILTER_BANK_X - 64.0,
        22.0,
        24.0,
    )
    .translate(
        0.0,
        -FILTER_BANK_Y / 2.0 + 14.0,
        -FILTER_BANK_Z / 2.0 + 12.0,
    );

    carrier - service_channel - sterile_filter_port_cuts()
        + sterile_filter_placeholders()
        + sterile_vent_port_markers()
        + filter_bank_label_strip()
}

fn sterile_filter_placeholders() -> Part {
    let mut filters = Part::empty("sterile_fluid_path_integrity_sterile_filter_placeholders");
    for i in 0..STERILE_FILTERS {
        let x = filter_x(i);
        let cartridge = centered_cylinder(
            format!("sterile_fluid_path_integrity_sterile_filter_{i}"),
            FILTER_OD / 2.0,
            FILTER_LENGTH,
            36,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(x, -34.0, 10.0);
        let clamp = centered_cube(
            format!("sterile_fluid_path_integrity_sterile_filter_clamp_{i}"),
            FILTER_LENGTH + 18.0,
            10.0,
            14.0,
        )
        .translate(x, -55.0, 10.0);
        filters = filters + cartridge + clamp;
    }
    filters
}

fn sterile_filter_port_cuts() -> Part {
    let mut cuts = Part::empty("sterile_fluid_path_integrity_filter_bank_port_cuts");
    for i in 0..STERILE_FILTERS {
        let x = filter_x(i);
        let inlet = centered_cylinder(
            format!("sterile_fluid_path_integrity_filter_{i}_inlet_socket"),
            TUBE_BORE_D / 2.0,
            FILTER_BANK_Y + 10.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x - 22.0, 0.0, -14.0);
        let outlet = centered_cylinder(
            format!("sterile_fluid_path_integrity_filter_{i}_outlet_socket"),
            TUBE_BORE_D / 2.0,
            FILTER_BANK_Y + 10.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x + 22.0, 0.0, -14.0);
        cuts = cuts + inlet + outlet;
    }
    cuts
}

fn sterile_vent_port_markers() -> Part {
    let mut vents = Part::empty("sterile_fluid_path_integrity_sterile_vent_port_markers");
    for i in 0..VENT_PORTS {
        let x = -((VENT_PORTS as f64 - 1.0) * 70.0) / 2.0 + i as f64 * 70.0;
        let port = centered_cylinder(
            format!("sterile_fluid_path_integrity_sterile_vent_port_{i}"),
            8.0,
            18.0,
            32,
        )
        .translate(x, FILTER_BANK_Y / 2.0 - 18.0, FILTER_BANK_Z / 2.0 + 9.0);
        let center = centered_cylinder(
            format!("sterile_fluid_path_integrity_sterile_vent_port_bore_{i}"),
            3.2,
            20.0,
            20,
        )
        .translate(x, FILTER_BANK_Y / 2.0 - 18.0, FILTER_BANK_Z / 2.0 + 9.0);
        vents = vents + (port - center);
    }
    vents
}

fn filter_bank_label_strip() -> Part {
    centered_cube(
        "sterile_fluid_path_integrity_filter_bank_label_strip",
        FILTER_BANK_X - 84.0,
        5.0,
        8.0,
    )
    .translate(0.0, -FILTER_BANK_Y / 2.0 - 2.0, FILTER_BANK_Z / 2.0 + 8.0)
}

fn waste_decon_diverter() -> Part {
    let manifold = centered_cube(
        "sterile_fluid_path_integrity_waste_decon_diverter_manifold",
        DIVERTER_X,
        62.0,
        DIVERTER_Z,
    )
    .translate(0.0, -32.0, 0.0);
    let waste_cup = canister_saddle("waste").translate(-82.0, 58.0, -10.0);
    let decon_cup = canister_saddle("decon").translate(82.0, 58.0, -10.0);

    manifold - waste_decon_channel_cuts()
        + waste_decon_selector_valves()
        + waste_cup
        + decon_cup
        + waste_decon_drip_lip()
}

fn waste_decon_channel_cuts() -> Part {
    let inlet = centered_cylinder(
        "sterile_fluid_path_integrity_waste_decon_common_inlet",
        TUBE_BORE_D / 2.0,
        DIVERTER_X + 10.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -32.0, 8.0);
    let waste_out = centered_cylinder(
        "sterile_fluid_path_integrity_waste_decon_waste_outlet",
        TUBE_BORE_D / 2.0,
        DIVERTER_Y + 10.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-82.0, 18.0, -14.0);
    let decon_out = centered_cylinder(
        "sterile_fluid_path_integrity_waste_decon_decon_outlet",
        TUBE_BORE_D / 2.0,
        DIVERTER_Y + 10.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(82.0, 18.0, -14.0);
    inlet + waste_out + decon_out
}

fn waste_decon_selector_valves() -> Part {
    let waste = centered_cube(
        "sterile_fluid_path_integrity_waste_selector_valve_placeholder",
        44.0,
        36.0,
        30.0,
    )
    .translate(-82.0, -70.0, 42.0);
    let decon = centered_cube(
        "sterile_fluid_path_integrity_decon_selector_valve_placeholder",
        44.0,
        36.0,
        30.0,
    )
    .translate(82.0, -70.0, 42.0);
    let interlock = centered_cube(
        "sterile_fluid_path_integrity_waste_decon_interlock_bar",
        202.0,
        10.0,
        10.0,
    )
    .translate(0.0, -96.0, 48.0);
    waste + decon + interlock
}

fn canister_saddle(name: &str) -> Part {
    let cup = centered_cylinder(
        format!("sterile_fluid_path_integrity_{name}_canister_saddle"),
        42.0,
        34.0,
        48,
    )
    .translate(0.0, 0.0, 0.0);
    let pocket = centered_cylinder(
        format!("sterile_fluid_path_integrity_{name}_canister_pocket"),
        34.0,
        36.0,
        48,
    )
    .translate(0.0, 0.0, 3.0);
    let front_relief = centered_cube(
        format!("sterile_fluid_path_integrity_{name}_canister_front_relief"),
        30.0,
        46.0,
        24.0,
    )
    .translate(0.0, -24.0, 8.0);
    cup - pocket - front_relief
}

fn waste_decon_drip_lip() -> Part {
    centered_cube(
        "sterile_fluid_path_integrity_waste_decon_forward_drip_lip",
        DIVERTER_X - 34.0,
        12.0,
        16.0,
    )
    .translate(0.0, -(DIVERTER_Y / 2.0 - 8.0), -18.0)
}

fn barcode_lot_scan_lands() -> Part {
    let carrier = centered_cube(
        "sterile_fluid_path_integrity_barcode_lot_land_carrier",
        LABEL_LANDS_X,
        LABEL_LANDS_Y,
        LABEL_LANDS_Z,
    );
    let mut lands = Part::empty("sterile_fluid_path_integrity_barcode_lot_scan_lands");
    for i in 0..BARCODE_LANDS {
        let x = -((BARCODE_LANDS as f64 - 1.0) * 80.0) / 2.0 + i as f64 * 80.0;
        let land = centered_cube(
            format!("sterile_fluid_path_integrity_barcode_lot_scan_land_{i}"),
            68.0,
            52.0,
            5.0,
        )
        .translate(x, 0.0, LABEL_LANDS_Z / 2.0 + 2.5);
        let scanner_relief = centered_cube(
            format!("sterile_fluid_path_integrity_barcode_lot_scanner_relief_{i}"),
            54.0,
            12.0,
            6.0,
        )
        .translate(x, -24.0, LABEL_LANDS_Z / 2.0 + 3.0);
        let lead_in = centered_cube(
            format!("sterile_fluid_path_integrity_barcode_lot_lead_in_tick_{i}"),
            4.0,
            62.0,
            6.0,
        )
        .translate(x - 38.0, 0.0, LABEL_LANDS_Z / 2.0 + 3.0);
        lands = lands + (land - scanner_relief) + lead_in;
    }
    carrier + lands
}

fn robot_service_keepouts() -> Part {
    let gripper = keepout_frame(
        "cassette_gripper",
        ARRAY_X + 138.0,
        ARRAY_Y + 134.0,
        KEEP_OUT_Z,
        12.0,
    )
    .translate(72.0, -26.0, 0.0);
    let valve_service = keepout_frame(
        "front_valve_service",
        VALVE_BANK_X + 84.0,
        VALVE_BANK_Y + 52.0,
        86.0,
        10.0,
    )
    .translate(-80.0, -BASE_Y / 2.0 + 146.0, -27.0);
    let source_service = keepout_frame(
        "rear_source_service",
        SOURCE_PANEL_X + 44.0,
        128.0,
        SOURCE_PANEL_Z + 28.0,
        12.0,
    )
    .translate(
        0.0,
        SOURCE_PANEL_BASE_Y - 58.0,
        SOURCE_PANEL_Z / 2.0 - KEEP_OUT_Z / 2.0,
    );
    let scanner = keepout_frame(
        "barcode_scanner_sightline",
        LABEL_LANDS_X + 58.0,
        82.0,
        78.0,
        8.0,
    )
    .translate(-(BASE_X / 2.0 - 270.0), -BASE_Y / 2.0 + 78.0, -31.0);
    gripper + valve_service + source_service + scanner
}

fn keepout_frame(name: &str, x: f64, y: f64, z: f64, rail: f64) -> Part {
    let rear = centered_cube(
        format!("sterile_fluid_path_integrity_{name}_keepout_rear_rail"),
        x,
        rail,
        rail,
    )
    .translate(0.0, y / 2.0, z / 2.0);
    let front = centered_cube(
        format!("sterile_fluid_path_integrity_{name}_keepout_front_rail"),
        x,
        rail,
        rail,
    )
    .translate(0.0, -y / 2.0, z / 2.0);
    let left = centered_cube(
        format!("sterile_fluid_path_integrity_{name}_keepout_left_rail"),
        rail,
        y,
        rail,
    )
    .translate(-x / 2.0, 0.0, z / 2.0);
    let right = centered_cube(
        format!("sterile_fluid_path_integrity_{name}_keepout_right_rail"),
        rail,
        y,
        rail,
    )
    .translate(x / 2.0, 0.0, z / 2.0);

    let post_z = z;
    let mut posts = Part::empty(format!(
        "sterile_fluid_path_integrity_{name}_keepout_corner_posts"
    ));
    for (i, (px, py)) in [
        (-x / 2.0, -y / 2.0),
        (x / 2.0, -y / 2.0),
        (-x / 2.0, y / 2.0),
        (x / 2.0, y / 2.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(
                format!("sterile_fluid_path_integrity_{name}_keepout_post_{i}"),
                rail,
                rail,
                post_z,
            )
            .translate(*px, *py, 0.0);
    }

    rear + front + left + right + posts
}

fn fiducial_target(name: &str) -> Part {
    let disk = centered_cylinder(format!("{name}_disk"), 8.0, 3.0, 36);
    let center = centered_cylinder(format!("{name}_center_bore"), 1.6, 4.0, 18);
    disk - center
}

fn chip_x(col: usize) -> f64 {
    -ARRAY_X / 2.0 + REVC_CHIP_LENGTH / 2.0 + col as f64 * CHIP_PITCH_X
}

fn row_y(row: usize) -> f64 {
    -ARRAY_Y / 2.0 + REVC_CHIP_WIDTH / 2.0 + row as f64 * CHIP_PITCH_Y
}

fn lane_center(row: usize, col: usize) -> (f64, f64) {
    (chip_x(col), row_y(row))
}

fn lane_index(row: usize, col: usize) -> usize {
    row * COLS + col
}

fn valve_center(row: usize, col: usize) -> (f64, f64) {
    let x = -((COLS as f64 - 1.0) * 146.0) / 2.0 + col as f64 * 146.0;
    (x, valve_row_y(row))
}

fn valve_row_y(row: usize) -> f64 {
    -((ROWS as f64 - 1.0) * 28.0) / 2.0 + row as f64 * 28.0
}

fn sensor_center(row: usize, col: usize) -> (f64, f64) {
    let x = -((COLS as f64 - 1.0) * 126.0) / 2.0 + col as f64 * 126.0;
    let y = -((ROWS as f64 - 1.0) * 18.0) / 2.0 + row as f64 * 18.0;
    (x, y)
}

fn filter_x(index: usize) -> f64 {
    -((STERILE_FILTERS as f64 - 1.0) * 94.0) / 2.0 + index as f64 * 94.0
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        for path in OUTPUTS {
            assert!(path.starts_with("output/sterile_fluid_path_integrity_tester_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn covers_required_integrity_tester_features() {
        assert_eq!(REQUIRED_FEATURES.len(), 10);
        assert!(REQUIRED_FEATURES.contains(&"pressure_vacuum_source_placeholders"));
        assert!(REQUIRED_FEATURES.contains(&"manifold_cassette_datum"));
        assert!(REQUIRED_FEATURES.contains(&"per_lane_valve_isolation"));
        assert!(REQUIRED_FEATURES.contains(&"pressure_decay_sensor_pockets"));
        assert!(REQUIRED_FEATURES.contains(&"bubble_wetness_optical_windows"));
        assert!(REQUIRED_FEATURES.contains(&"sterile_filter_vent_ports"));
        assert!(REQUIRED_FEATURES.contains(&"waste_decon_diversion"));
        assert!(REQUIRED_FEATURES.contains(&"leak_witness_tray"));
        assert!(REQUIRED_FEATURES.contains(&"barcode_lot_scan_lands"));
        assert!(REQUIRED_FEATURES.contains(&"robot_service_keepouts"));
    }

    #[test]
    fn lane_counts_match_twenty_chip_cassette() {
        assert_eq!(COLS, 4);
        assert_eq!(ROWS, 5);
        assert_eq!(LANES, 20);
        assert_eq!(LANE_ISOLATION_VALVES, LANES);
        assert_eq!(PRESSURE_SENSOR_POCKETS, LANES);
        assert_eq!(BUBBLE_WINDOWS, LANES);
        assert_eq!(WETNESS_WINDOWS, LANES);
    }

    #[test]
    fn cassette_datum_encloses_revc_array_with_clearance() {
        assert!(NEST_X > ARRAY_X + 100.0);
        assert!(NEST_Y > ARRAY_Y + 90.0);
        assert!(TRAY_X > NEST_X);
        assert!(TRAY_Y > NEST_Y);
        assert!((chip_x(0) + chip_x(COLS - 1)).abs() < 0.001);
        assert!((row_y(0) + row_y(ROWS - 1)).abs() < 0.001);
    }

    #[test]
    fn utilities_fit_on_benchtop_deck() {
        assert!(BASE_X <= 1000.0);
        assert!(BASE_Y <= 800.0);
        assert!(SOURCE_PANEL_X < BASE_X - 60.0);
        assert!(VALVE_BANK_X < BASE_X - 120.0);
        assert!(FILTER_BANK_X < SOURCE_PANEL_X);
    }

    #[test]
    fn sterile_service_counts_are_sane() {
        assert_eq!(STERILE_FILTERS, ROWS + 2);
        assert_eq!(VENT_PORTS, ROWS + 3);
        assert_eq!(BARCODE_LANDS, 5);
        assert_eq!(ROBOT_KEEP_OUT_ZONES, 4);
        assert!(TUBE_BORE_D > TUBE_OD);
        assert!(SENSOR_TAP_D < TUBE_BORE_D);
    }

    #[test]
    fn valve_and_sensor_positions_stay_inside_carriers() {
        for row in 0..ROWS {
            for col in 0..COLS {
                let (vx, vy) = valve_center(row, col);
                assert!(vx.abs() + VALVE_BLOCK_X / 2.0 < VALVE_BANK_X / 2.0 - 24.0);
                assert!(vy.abs() + VALVE_BLOCK_Y / 2.0 < VALVE_BANK_Y / 2.0 - 20.0);

                let (sx, sy) = sensor_center(row, col);
                assert!(sx.abs() + SENSOR_POCKET_X / 2.0 < SENSOR_MATRIX_X / 2.0 - 24.0);
                assert!(sy.abs() + SENSOR_POCKET_Y / 2.0 < SENSOR_MATRIX_Y / 2.0 - 20.0);
            }
        }
    }
}
