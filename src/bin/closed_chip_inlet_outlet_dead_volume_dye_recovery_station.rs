use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed-chip inlet/outlet dead-volume and dye-recovery validation station.
//
// Intent:
// - Validate inlet/outlet adapter hold-up, dye carryover, flush recovery, and
//   no-cell chip/cassette handling before any live culture article is loaded.
// - Keep the 20-position cassette footprint explicit while replacing each chip
//   with a sealed no-cell flow coupon and low-volume inlet/outlet adapters.
// - Provide dye/tracer injection, flush/recovery nests, optical color/reference
//   lands, pressure/flow witness points, waste capture, traceability, disposition
//   lanes, and robot/service keepouts in one bench-scale station.
//
// This is architecture/fit CAD for validation planning. It is not a wetted-path
// specification, release protocol, analytical method, or sterility claim.

const OUTPUTS: &[&str] = &[
    "output/closed_chip_inlet_outlet_dead_volume_dye_recovery_station_base_leak_tray.stl",
    "output/closed_chip_inlet_outlet_dead_volume_dye_recovery_station_chip_cassette_surrogate.stl",
    "output/closed_chip_inlet_outlet_dead_volume_dye_recovery_station_low_volume_adapters.stl",
    "output/closed_chip_inlet_outlet_dead_volume_dye_recovery_station_dye_tracer_injection_ports.stl",
    "output/closed_chip_inlet_outlet_dead_volume_dye_recovery_station_flush_recovery_collection_nests.stl",
    "output/closed_chip_inlet_outlet_dead_volume_dye_recovery_station_optical_color_reference_lands.stl",
    "output/closed_chip_inlet_outlet_dead_volume_dye_recovery_station_pressure_flow_witness_points.stl",
    "output/closed_chip_inlet_outlet_dead_volume_dye_recovery_station_waste_capture.stl",
    "output/closed_chip_inlet_outlet_dead_volume_dye_recovery_station_barcode_fiducials.stl",
    "output/closed_chip_inlet_outlet_dead_volume_dye_recovery_station_release_hold_reject_lanes.stl",
    "output/closed_chip_inlet_outlet_dead_volume_dye_recovery_station_robot_service_keepouts.stl",
    "output/closed_chip_inlet_outlet_dead_volume_dye_recovery_station_assembly.stl",
];

const REQUIRED_FEATURES: &[&str] = &[
    "base_leak_tray",
    "twenty_position_no_cell_chip_cassette_surrogate",
    "inlet_outlet_low_volume_adapters",
    "dye_tracer_injection_ports",
    "flush_recovery_collection_nests",
    "optical_color_reference_lands",
    "pressure_flow_witness_points",
    "waste_capture",
    "barcode_fiducials",
    "release_hold_reject_lanes",
    "robot_service_keepouts",
    "assembly_export",
];

const STATION_X: f64 = 1280.0;
const STATION_Y: f64 = 860.0;
const BASE_Z: f64 = 22.0;
const LEAK_BASIN_X: f64 = STATION_X - 112.0;
const LEAK_BASIN_Y: f64 = STATION_Y - 104.0;
const LEAK_BASIN_DEPTH: f64 = 7.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const DRAIN_D: f64 = 16.0;
const LEAK_SENSOR_WELLS: usize = 4;

const CHIP_COLS: usize = 4;
const CHIP_ROWS: usize = 5;
const CHIP_POSITIONS: usize = CHIP_COLS * CHIP_ROWS;
const CHIP_GUTTER: f64 = 13.0;
const CHIP_PITCH_X: f64 = REVC_CHIP_LENGTH + CHIP_GUTTER;
const CHIP_PITCH_Y: f64 = REVC_CHIP_WIDTH + CHIP_GUTTER;
const CHIP_ARRAY_X: f64 =
    CHIP_COLS as f64 * REVC_CHIP_LENGTH + (CHIP_COLS as f64 - 1.0) * CHIP_GUTTER;
const CHIP_ARRAY_Y: f64 =
    CHIP_ROWS as f64 * REVC_CHIP_WIDTH + (CHIP_ROWS as f64 - 1.0) * CHIP_GUTTER;
const CASSETTE_MARGIN_X: f64 = 42.0;
const CASSETTE_MARGIN_Y: f64 = 40.0;
const CASSETTE_X: f64 = CHIP_ARRAY_X + 2.0 * CASSETTE_MARGIN_X;
const CASSETTE_Y: f64 = CHIP_ARRAY_Y + 2.0 * CASSETTE_MARGIN_Y;
const CASSETTE_Z: f64 = 18.0;
const CASSETTE_CENTER_X: f64 = -262.0;
const CASSETTE_CENTER_Y: f64 = 8.0;
const CHIP_POCKET_CLEARANCE: f64 = 0.65;
const CHIP_POCKET_DEPTH: f64 = 5.8;
const NO_CELL_COUPON_Z: f64 = 8.0;
const NO_CELL_OBSERVATION_WINDOW_X: f64 = 70.0;
const NO_CELL_OBSERVATION_WINDOW_Y: f64 = 18.0;
const DATUM_PIN_D: f64 = 6.0;
const CASSETTE_DOCK_CLAMPS: usize = 8;

const INLET_ADAPTERS: usize = CHIP_POSITIONS;
const OUTLET_ADAPTERS: usize = CHIP_POSITIONS;
const ADAPTER_X: f64 = 32.0;
const ADAPTER_Y: f64 = 24.0;
const ADAPTER_Z: f64 = 16.0;
const ADAPTER_BORE_D: f64 = 1.6;
const ADAPTER_TUBE_D: f64 = 4.2;
const ADAPTER_HOLDUP_UL: f64 = 9.5;
const ADAPTER_EDGE_INSET_X: f64 = 17.0;
const ADAPTER_Y_OFFSET: f64 = 0.0;
const CAPILLARY_TRACE_W: f64 = 3.0;
const CAPILLARY_TRACE_Z: f64 = 2.2;
const ADAPTER_CLAMP_D: f64 = 7.0;

const DYE_INJECTION_PORTS: usize = CHIP_POSITIONS;
const DYE_BANK_X: f64 = 690.0;
const DYE_BANK_Y: f64 = 74.0;
const DYE_BANK_Z: f64 = 30.0;
const DYE_BANK_CENTER_X: f64 = -258.0;
const DYE_BANK_CENTER_Y: f64 = STATION_Y / 2.0 - 84.0;
const DYE_PORT_D: f64 = 9.0;
const DYE_PORT_PITCH_X: f64 = 32.0;
const DYE_REFERENCE_WELLS: usize = 4;

const COLLECTION_COLS: usize = 5;
const COLLECTION_ROWS: usize = 4;
const COLLECTION_NESTS: usize = COLLECTION_COLS * COLLECTION_ROWS;
const COLLECTION_RACK_X: f64 = 370.0;
const COLLECTION_RACK_Y: f64 = 348.0;
const COLLECTION_RACK_Z: f64 = 34.0;
const COLLECTION_CENTER_X: f64 = 418.0;
const COLLECTION_CENTER_Y: f64 = -36.0;
const COLLECTION_PITCH_X: f64 = 64.0;
const COLLECTION_PITCH_Y: f64 = 76.0;
const RECOVERY_VIAL_D: f64 = 18.2;
const RECOVERY_VIAL_CLEARANCE_D: f64 = 20.6;
const FLUSH_FUNNEL_D: f64 = 28.0;
const RECOVERY_SPLASH_RIM_D: f64 = 33.0;
const FLUSH_PORTS: usize = CHIP_POSITIONS;

const OPTICAL_PANEL_X: f64 = 390.0;
const OPTICAL_PANEL_Y: f64 = 106.0;
const OPTICAL_PANEL_Z: f64 = 12.0;
const OPTICAL_CENTER_X: f64 = 414.0;
const OPTICAL_CENTER_Y: f64 = 302.0;
const COLOR_SAMPLE_LANDS: usize = CHIP_POSITIONS;
const REFERENCE_LANDS: usize = 6;
const COLOR_SWATCH_X: f64 = 25.0;
const COLOR_SWATCH_Y: f64 = 20.0;
const REFERENCE_SWATCH_X: f64 = 36.0;
const REFERENCE_SWATCH_Y: f64 = 24.0;

const WITNESS_BAR_X: f64 = 720.0;
const WITNESS_BAR_Y: f64 = 76.0;
const WITNESS_BAR_Z: f64 = 34.0;
const WITNESS_CENTER_X: f64 = -252.0;
const WITNESS_CENTER_Y: f64 = -STATION_Y / 2.0 + 82.0;
const PRESSURE_WITNESS_POINTS: usize = CHIP_POSITIONS;
const FLOW_WITNESS_POINTS: usize = CHIP_POSITIONS;
const WITNESS_PITCH_X: f64 = 33.0;
const PRESSURE_TAP_D: f64 = 5.4;
const FLOW_SIGHT_X: f64 = 19.0;
const FLOW_SIGHT_Y: f64 = 7.0;

const WASTE_PAN_X: f64 = 214.0;
const WASTE_PAN_Y: f64 = 250.0;
const WASTE_PAN_Z: f64 = 54.0;
const WASTE_CENTER_X: f64 = 510.0;
const WASTE_CENTER_Y: f64 = -286.0;
const WASTE_BOTTLE_NESTS: usize = 2;
const WASTE_BOTTLE_D: f64 = 58.0;
const WASTE_TROUGH_X: f64 = 180.0;
const WASTE_TROUGH_Y: f64 = 42.0;
const WASTE_TROUGH_Z: f64 = 26.0;
const WASTE_CAPTURE_VOLUME_ML: f64 = 900.0;

const TRACE_PANEL_X: f64 = 315.0;
const TRACE_PANEL_Y: f64 = 106.0;
const TRACE_PANEL_Z: f64 = 10.0;
const TRACE_CENTER_X: f64 = -455.0;
const TRACE_CENTER_Y: f64 = -350.0;
const BARCODE_LANDS: usize = 8;
const FIDUCIALS: usize = 6;
const BARCODE_LAND_X: f64 = 74.0;
const BARCODE_LAND_Y: f64 = 18.0;
const FIDUCIAL_D: f64 = 11.0;

const DISPOSITION_LANES: usize = 3;
const LANE_BANK_X: f64 = 508.0;
const LANE_BANK_Y: f64 = 116.0;
const LANE_BANK_Z: f64 = 22.0;
const LANE_CENTER_X: f64 = 35.0;
const LANE_CENTER_Y: f64 = -342.0;
const LANE_WIDTH: f64 = 148.0;
const LANE_PITCH_X: f64 = 164.0;
const LANE_GATE_Z: f64 = 44.0;
const RELEASE_LANE_INDEX: usize = 0;
const HOLD_LANE_INDEX: usize = 1;
const REJECT_LANE_INDEX: usize = 2;

const ROBOT_KEEP_OUT_Z: f64 = 148.0;
const ROBOT_APPROACH_WINDOW_X: f64 = 760.0;
const ROBOT_APPROACH_WINDOW_Y: f64 = 590.0;
const ROBOT_KEEP_OUT_WINDOWS: usize = 4;
const FRONT_SERVICE_CLEARANCE: f64 = 360.0;
const REAR_TUBE_SERVICE_CLEARANCE: f64 = 210.0;
const RIGHT_COLLECTION_SERVICE_CLEARANCE: f64 = 190.0;
const LEFT_SCAN_SERVICE_CLEARANCE: f64 = 160.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_leak_tray();
    export(&base, OUTPUTS[0]);

    let cassette = chip_cassette_surrogate();
    export(&cassette, OUTPUTS[1]);

    let adapters = inlet_outlet_low_volume_adapters();
    export(&adapters, OUTPUTS[2]);

    let dye = dye_tracer_injection_ports();
    export(&dye, OUTPUTS[3]);

    let collection = flush_recovery_collection_nests();
    export(&collection, OUTPUTS[4]);

    let optics = optical_color_reference_lands();
    export(&optics, OUTPUTS[5]);

    let witness = pressure_flow_witness_points();
    export(&witness, OUTPUTS[6]);

    let waste = waste_capture();
    export(&waste, OUTPUTS[7]);

    let traceability = barcode_fiducials();
    export(&traceability, OUTPUTS[8]);

    let lanes = release_hold_reject_lanes();
    export(&lanes, OUTPUTS[9]);

    let keepouts = robot_service_keepouts();
    export(&keepouts, OUTPUTS[10]);

    let assembly = base
        + cassette
        + adapters
        + dye
        + collection
        + optics
        + witness
        + waste
        + traceability
        + lanes
        + keepouts;
    export(&assembly, OUTPUTS[11]);

    println!(
        "Closed-chip inlet/outlet dead-volume and dye-recovery validation station: {:.0}mm x {:.0}mm deck with {} no-cell chip/cassette surrogate positions, {} inlet adapters, {} outlet adapters, {} dye/tracer injection ports, and {} flush/recovery collection nests.",
        STATION_X,
        STATION_Y,
        CHIP_POSITIONS,
        INLET_ADAPTERS,
        OUTLET_ADAPTERS,
        DYE_INJECTION_PORTS,
        COLLECTION_NESTS
    );
    println!(
        "Validation features: {} optical sample/reference lands, {} pressure taps, {} flow witness windows, {} leak sensor wells, {} waste bottle nests, {} barcode/fiducial features, release/hold/reject disposition lanes, {} robot/service keepout windows, and {} required feature groups.",
        COLOR_SAMPLE_LANDS + REFERENCE_LANDS,
        PRESSURE_WITNESS_POINTS,
        FLOW_WITNESS_POINTS,
        LEAK_SENSOR_WELLS,
        WASTE_BOTTLE_NESTS,
        BARCODE_LANDS + FIDUCIALS,
        ROBOT_KEEP_OUT_WINDOWS,
        REQUIRED_FEATURES.len()
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_design_constraints() {
    assert_eq!(
        CHIP_POSITIONS, 20,
        "station must model the full 20-position cassette"
    );
    assert_eq!(INLET_ADAPTERS, CHIP_POSITIONS);
    assert_eq!(OUTLET_ADAPTERS, CHIP_POSITIONS);
    assert_eq!(DYE_INJECTION_PORTS, CHIP_POSITIONS);
    assert_eq!(COLLECTION_NESTS, CHIP_POSITIONS);
    assert_eq!(PRESSURE_WITNESS_POINTS, CHIP_POSITIONS);
    assert_eq!(FLOW_WITNESS_POINTS, CHIP_POSITIONS);
    assert!(
        RECOVERY_VIAL_CLEARANCE_D > RECOVERY_VIAL_D,
        "recovery vial nests need insertion clearance"
    );
    assert!(
        ADAPTER_HOLDUP_UL <= 10.0 && ADAPTER_BORE_D <= 1.6,
        "adapter envelope is not low-dead-volume"
    );
    assert!(
        CASSETTE_X < STATION_X * 0.56 && CASSETTE_Y < STATION_Y * 0.72,
        "cassette surrogate does not fit the station envelope"
    );
    assert!(
        WASTE_CAPTURE_VOLUME_ML >= 2.0 * CHIP_POSITIONS as f64 * 20.0,
        "waste capture should hold at least two 20mL flushes across the cassette"
    );
}

fn base_leak_tray() -> Part {
    let deck = centered_cube(
        "closed_chip_dye_station_base_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let basin = centered_cube(
        "closed_chip_dye_station_recessed_leak_basin",
        LEAK_BASIN_X,
        LEAK_BASIN_Y,
        LEAK_BASIN_DEPTH + 1.0,
    )
    .translate(0.0, 0.0, BASE_Z - LEAK_BASIN_DEPTH / 2.0 + 0.5);
    let drain = centered_cylinder(
        "closed_chip_dye_station_low_point_waste_drain",
        DRAIN_D / 2.0,
        BASE_Z + 8.0,
        36,
    )
    .translate(WASTE_CENTER_X - 74.0, WASTE_CENTER_Y + 94.0, BASE_Z / 2.0);

    deck - basin - drain
        + perimeter_rim()
        + leak_sensor_wells()
        + cassette_registration_rails()
        + deck_mount_slots()
}

fn perimeter_rim() -> Part {
    let front = centered_cube(
        "closed_chip_dye_station_front_spill_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -(STATION_Y / 2.0 - RIM_W / 2.0), BASE_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        "closed_chip_dye_station_rear_tubing_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "closed_chip_dye_station_left_scan_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-(STATION_X / 2.0 - RIM_W / 2.0), 0.0, BASE_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "closed_chip_dye_station_right_collection_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);

    front + rear + left + right
}

fn leak_sensor_wells() -> Part {
    let mut wells = Part::empty("closed_chip_dye_station_leak_sensor_wells");
    for (i, (x, y)) in [
        (-548.0, -360.0),
        (-548.0, 352.0),
        (548.0, -360.0),
        (548.0, 352.0),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cube(
            format!("closed_chip_dye_station_leak_sensor_boss_{i}"),
            46.0,
            34.0,
            8.0,
        )
        .translate(*x, *y, BASE_Z + 4.0);
        let cup = centered_cylinder(
            format!("closed_chip_dye_station_leak_sensor_cup_{i}"),
            9.0,
            10.0,
            30,
        )
        .translate(*x, *y, BASE_Z + 4.0);
        wells = wells + (boss - cup);
    }
    wells
}

fn cassette_registration_rails() -> Part {
    let rail_y = CASSETTE_Y + 28.0;
    let left = centered_cube(
        "closed_chip_dye_station_cassette_left_registration_rail",
        12.0,
        rail_y,
        18.0,
    )
    .translate(
        CASSETTE_CENTER_X - CASSETTE_X / 2.0 - 18.0,
        CASSETTE_CENTER_Y,
        BASE_Z + 9.0,
    );
    let right = centered_cube(
        "closed_chip_dye_station_cassette_right_registration_rail",
        12.0,
        rail_y,
        13.0,
    )
    .translate(
        CASSETTE_CENTER_X + CASSETTE_X / 2.0 + 18.0,
        CASSETTE_CENTER_Y,
        BASE_Z + 6.5,
    );
    let rear = centered_cube(
        "closed_chip_dye_station_cassette_rear_hard_stop_rail",
        CASSETTE_X + 60.0,
        12.0,
        18.0,
    )
    .translate(
        CASSETTE_CENTER_X,
        CASSETTE_CENTER_Y + CASSETTE_Y / 2.0 + 18.0,
        BASE_Z + 9.0,
    );
    let front = centered_cube(
        "closed_chip_dye_station_cassette_front_soft_stop_rail",
        CASSETTE_X + 60.0,
        10.0,
        12.0,
    )
    .translate(
        CASSETTE_CENTER_X,
        CASSETTE_CENTER_Y - CASSETTE_Y / 2.0 - 18.0,
        BASE_Z + 6.0,
    );

    left + right + rear + front
}

fn deck_mount_slots() -> Part {
    let mut slots = Part::empty("closed_chip_dye_station_deck_mount_slots");
    for (i, (x, y)) in [
        (-565.0, -370.0),
        (-365.0, -370.0),
        (-165.0, -370.0),
        (165.0, -370.0),
        (365.0, -370.0),
        (565.0, -370.0),
        (-565.0, 370.0),
        (565.0, 370.0),
    ]
    .iter()
    .enumerate()
    {
        let land = centered_cube(
            format!("closed_chip_dye_station_mount_slot_land_{i}"),
            54.0,
            24.0,
            4.0,
        )
        .translate(*x, *y, BASE_Z + 2.0);
        let slot = centered_cube(
            format!("closed_chip_dye_station_mount_slot_cut_{i}"),
            34.0,
            8.0,
            6.0,
        )
        .translate(*x, *y, BASE_Z + 2.0);
        slots = slots + (land - slot);
    }
    slots
}

fn chip_cassette_surrogate() -> Part {
    let cassette = centered_cube(
        "closed_chip_dye_station_twenty_position_no_cell_cassette_body",
        CASSETTE_X,
        CASSETTE_Y,
        CASSETTE_Z,
    )
    .translate(
        CASSETTE_CENTER_X,
        CASSETTE_CENTER_Y,
        BASE_Z + CASSETTE_Z / 2.0,
    );

    cassette - chip_pocket_cuts()
        + no_cell_coupon_grid()
        + cassette_grid_ribs()
        + cassette_datum_features()
        + cassette_dock_clamps()
}

fn chip_pocket_cuts() -> Part {
    let mut cuts = Part::empty("closed_chip_dye_station_chip_pocket_cuts");
    for row in 0..CHIP_ROWS {
        for col in 0..CHIP_COLS {
            let (x, y) = chip_center(col, row);
            cuts = cuts
                + centered_cube(
                    format!("closed_chip_dye_station_chip_recess_{col}_{row}"),
                    REVC_CHIP_LENGTH + CHIP_POCKET_CLEARANCE * 2.0,
                    REVC_CHIP_WIDTH + CHIP_POCKET_CLEARANCE * 2.0,
                    CHIP_POCKET_DEPTH + 0.4,
                )
                .translate(
                    x,
                    y,
                    BASE_Z + CASSETTE_Z - CHIP_POCKET_DEPTH / 2.0 + 0.2,
                );
        }
    }
    cuts
}

fn no_cell_coupon_grid() -> Part {
    let mut grid = Part::empty("closed_chip_dye_station_no_cell_flow_coupon_grid");
    for row in 0..CHIP_ROWS {
        for col in 0..CHIP_COLS {
            let (x, y) = chip_center(col, row);
            grid = grid + no_cell_coupon(col, row).translate(x, y, BASE_Z + CASSETTE_Z + 1.2);
        }
    }
    grid
}

fn no_cell_coupon(col: usize, row: usize) -> Part {
    let coupon = centered_cube(
        format!("closed_chip_dye_station_no_cell_coupon_body_{col}_{row}"),
        REVC_CHIP_LENGTH - 10.0,
        REVC_CHIP_WIDTH - 12.0,
        NO_CELL_COUPON_Z,
    );
    let window = centered_cube(
        format!("closed_chip_dye_station_no_cell_observation_window_{col}_{row}"),
        NO_CELL_OBSERVATION_WINDOW_X,
        NO_CELL_OBSERVATION_WINDOW_Y,
        NO_CELL_COUPON_Z + 0.8,
    )
    .translate(0.0, 0.0, 0.0);
    let inlet_channel = centered_cube(
        format!("closed_chip_dye_station_coupon_inlet_trace_{col}_{row}"),
        REVC_CHIP_LENGTH * 0.36,
        CAPILLARY_TRACE_W,
        CAPILLARY_TRACE_Z,
    )
    .translate(-REVC_CHIP_LENGTH * 0.22, 0.0, NO_CELL_COUPON_Z / 2.0);
    let outlet_channel = centered_cube(
        format!("closed_chip_dye_station_coupon_outlet_trace_{col}_{row}"),
        REVC_CHIP_LENGTH * 0.36,
        CAPILLARY_TRACE_W,
        CAPILLARY_TRACE_Z,
    )
    .translate(REVC_CHIP_LENGTH * 0.22, 0.0, NO_CELL_COUPON_Z / 2.0);
    let reference_tick = centered_cube(
        format!("closed_chip_dye_station_no_cell_coupon_reference_tick_{col}_{row}"),
        5.0,
        REVC_CHIP_WIDTH - 26.0,
        1.8,
    )
    .translate(0.0, 0.0, NO_CELL_COUPON_Z / 2.0 + 0.9);

    coupon - window + inlet_channel + outlet_channel + reference_tick
}

fn cassette_grid_ribs() -> Part {
    let mut ribs = Part::empty("closed_chip_dye_station_cassette_position_grid_ribs");
    for col in 0..CHIP_COLS - 1 {
        let (left_x, _) = chip_center(col, 0);
        let (right_x, _) = chip_center(col + 1, 0);
        ribs = ribs
            + centered_cube(
                format!("closed_chip_dye_station_cassette_column_rib_{col}"),
                6.0,
                CHIP_ARRAY_Y + 18.0,
                7.0,
            )
            .translate(
                (left_x + right_x) / 2.0,
                CASSETTE_CENTER_Y,
                BASE_Z + CASSETTE_Z + 3.5,
            );
    }
    for row in 0..CHIP_ROWS - 1 {
        let (_, lower_y) = chip_center(0, row);
        let (_, upper_y) = chip_center(0, row + 1);
        ribs = ribs
            + centered_cube(
                format!("closed_chip_dye_station_cassette_row_rib_{row}"),
                CHIP_ARRAY_X + 18.0,
                6.0,
                7.0,
            )
            .translate(
                CASSETTE_CENTER_X,
                (lower_y + upper_y) / 2.0,
                BASE_Z + CASSETTE_Z + 3.5,
            );
    }
    ribs
}

fn cassette_datum_features() -> Part {
    let mut datums = Part::empty("closed_chip_dye_station_cassette_datum_features");
    for (i, (x, y)) in cassette_corner_points(26.0).iter().enumerate() {
        let pin = centered_cylinder(
            format!("closed_chip_dye_station_cassette_datum_pin_{i}"),
            DATUM_PIN_D / 2.0,
            16.0,
            28,
        )
        .translate(*x, *y, BASE_Z + CASSETTE_Z + 8.0);
        let target = centered_cube(
            format!("closed_chip_dye_station_cassette_robot_crosshair_{i}"),
            28.0,
            4.0,
            2.0,
        )
        .translate(*x, *y, BASE_Z + CASSETTE_Z + 17.0)
            + centered_cube(
                format!("closed_chip_dye_station_cassette_robot_crosshair_orthogonal_{i}"),
                4.0,
                28.0,
                2.0,
            )
            .translate(*x, *y, BASE_Z + CASSETTE_Z + 17.0);
        datums = datums + pin + target;
    }
    datums
}

fn cassette_dock_clamps() -> Part {
    let mut clamps = Part::empty("closed_chip_dye_station_cassette_dock_clamps");
    for i in 0..CASSETTE_DOCK_CLAMPS {
        let left_side = i < CASSETTE_DOCK_CLAMPS / 2;
        let idx = if left_side {
            i
        } else {
            i - CASSETTE_DOCK_CLAMPS / 2
        };
        let x = if left_side {
            CASSETTE_CENTER_X - CASSETTE_X / 2.0 - 3.0
        } else {
            CASSETTE_CENTER_X + CASSETTE_X / 2.0 + 3.0
        };
        let y = CASSETTE_CENTER_Y - CHIP_ARRAY_Y / 2.0 + idx as f64 * (CHIP_ARRAY_Y / 3.0);
        clamps = clamps
            + centered_cube(
                format!("closed_chip_dye_station_cassette_hold_down_clamp_{i}"),
                22.0,
                34.0,
                10.0,
            )
            .translate(x, y, BASE_Z + CASSETTE_Z + 5.0);
    }
    clamps
}

fn inlet_outlet_low_volume_adapters() -> Part {
    let mut adapters = Part::empty("closed_chip_dye_station_low_volume_adapter_array");
    for row in 0..CHIP_ROWS {
        for col in 0..CHIP_COLS {
            let (x, y) = chip_center(col, row);
            adapters = adapters
                + low_volume_adapter(col, row, AdapterSide::Inlet).translate(
                    x - REVC_CHIP_LENGTH / 2.0 + ADAPTER_EDGE_INSET_X,
                    y + ADAPTER_Y_OFFSET,
                    BASE_Z + CASSETTE_Z + ADAPTER_Z / 2.0 + NO_CELL_COUPON_Z + 1.4,
                )
                + low_volume_adapter(col, row, AdapterSide::Outlet).translate(
                    x + REVC_CHIP_LENGTH / 2.0 - ADAPTER_EDGE_INSET_X,
                    y + ADAPTER_Y_OFFSET,
                    BASE_Z + CASSETTE_Z + ADAPTER_Z / 2.0 + NO_CELL_COUPON_Z + 1.4,
                );
        }
    }
    adapters + adapter_tube_comb_guides()
}

#[derive(Clone, Copy)]
enum AdapterSide {
    Inlet,
    Outlet,
}

impl AdapterSide {
    fn label(self) -> &'static str {
        match self {
            AdapterSide::Inlet => "inlet",
            AdapterSide::Outlet => "outlet",
        }
    }
}

fn low_volume_adapter(col: usize, row: usize, side: AdapterSide) -> Part {
    let label = side.label();
    let body = centered_cube(
        format!("closed_chip_dye_station_{label}_low_volume_adapter_body_{col}_{row}"),
        ADAPTER_X,
        ADAPTER_Y,
        ADAPTER_Z,
    );
    let vertical_bore = centered_cylinder(
        format!("closed_chip_dye_station_{label}_low_volume_adapter_capillary_bore_{col}_{row}"),
        ADAPTER_BORE_D / 2.0,
        ADAPTER_Z + 2.0,
        24,
    );
    let tube_socket = centered_cylinder(
        format!("closed_chip_dye_station_{label}_tube_socket_{col}_{row}"),
        ADAPTER_TUBE_D / 2.0,
        ADAPTER_X + 3.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0);
    let clamp_boss = centered_cylinder(
        format!("closed_chip_dye_station_{label}_adapter_clamp_boss_{col}_{row}"),
        ADAPTER_CLAMP_D / 2.0,
        5.0,
        24,
    )
    .translate(0.0, ADAPTER_Y / 2.0 - 6.0, ADAPTER_Z / 2.0 + 2.5);
    let microchannel = centered_cube(
        format!("closed_chip_dye_station_{label}_adapter_microchannel_land_{col}_{row}"),
        18.0,
        CAPILLARY_TRACE_W,
        2.0,
    )
    .translate(0.0, 0.0, -ADAPTER_Z / 2.0 + 1.0);

    body - vertical_bore - tube_socket + clamp_boss + microchannel
}

fn adapter_tube_comb_guides() -> Part {
    let mut guides = Part::empty("closed_chip_dye_station_adapter_tube_comb_guides");
    for row in 0..CHIP_ROWS {
        let y = chip_center(0, row).1;
        for (i, (x, side_label)) in [
            (CASSETTE_CENTER_X - CASSETTE_X / 2.0 - 54.0, "inlet"),
            (CASSETTE_CENTER_X + CASSETTE_X / 2.0 + 54.0, "outlet"),
        ]
        .iter()
        .enumerate()
        {
            let comb = centered_cube(
                format!("closed_chip_dye_station_{side_label}_row_{row}_tube_comb_body_{i}"),
                38.0,
                58.0,
                20.0,
            );
            let mut slots = Part::empty(format!(
                "closed_chip_dye_station_{side_label}_row_{row}_tube_comb_slots"
            ));
            for lane in 0..4 {
                slots = slots
                    + centered_cylinder(
                        format!("closed_chip_dye_station_{side_label}_row_{row}_tube_slot_{lane}"),
                        ADAPTER_TUBE_D / 2.0 + 0.8,
                        42.0,
                        22,
                    )
                    .rotate(90.0, 0.0, 0.0)
                    .translate(-12.0 + lane as f64 * 8.0, 0.0, 0.0);
            }
            guides = guides
                + (comb - slots).translate(*x, y, BASE_Z + CASSETTE_Z + NO_CELL_COUPON_Z + 17.0);
        }
    }
    guides
}

fn dye_tracer_injection_ports() -> Part {
    let bank = centered_cube(
        "closed_chip_dye_station_dye_tracer_injection_port_bank",
        DYE_BANK_X,
        DYE_BANK_Y,
        DYE_BANK_Z,
    )
    .translate(
        DYE_BANK_CENTER_X,
        DYE_BANK_CENTER_Y,
        BASE_Z + DYE_BANK_Z / 2.0,
    );

    let mut port_cuts = Part::empty("closed_chip_dye_station_dye_tracer_port_cuts");
    let mut collars = Part::empty("closed_chip_dye_station_dye_tracer_port_collars");
    for i in 0..DYE_INJECTION_PORTS {
        let x = DYE_BANK_CENTER_X + indexed_lane(i, DYE_INJECTION_PORTS, DYE_PORT_PITCH_X);
        let y = DYE_BANK_CENTER_Y + if i % 2 == 0 { -16.0 } else { 16.0 };
        port_cuts = port_cuts
            + centered_cylinder(
                format!("closed_chip_dye_station_dye_tracer_injection_bore_{i}"),
                DYE_PORT_D / 2.0,
                DYE_BANK_Z + 4.0,
                30,
            )
            .translate(x, y, BASE_Z + DYE_BANK_Z / 2.0);
        collars = collars
            + centered_cylinder(
                format!("closed_chip_dye_station_dye_tracer_luer_collar_{i}"),
                (DYE_PORT_D + 8.0) / 2.0,
                6.0,
                30,
            )
            .translate(x, y, BASE_Z + DYE_BANK_Z + 3.0);
    }

    let mut references = Part::empty("closed_chip_dye_station_dye_reference_wells");
    for i in 0..DYE_REFERENCE_WELLS {
        let x = DYE_BANK_CENTER_X - DYE_BANK_X / 2.0 + 40.0 + i as f64 * 26.0;
        references = references
            + centered_cylinder(
                format!("closed_chip_dye_station_dye_reference_well_{i}"),
                8.0,
                8.0,
                28,
            )
            .translate(x, DYE_BANK_CENTER_Y, BASE_Z + DYE_BANK_Z + 4.0);
    }

    (bank - port_cuts) + collars + references + dye_route_stubs()
}

fn dye_route_stubs() -> Part {
    let mut routes = Part::empty("closed_chip_dye_station_dye_route_stub_lands");
    for col in 0..CHIP_COLS {
        let x = chip_center(col, CHIP_ROWS - 1).0;
        routes = routes
            + centered_cube(
                format!("closed_chip_dye_station_dye_route_column_stub_{col}"),
                12.0,
                68.0,
                4.0,
            )
            .translate(x, DYE_BANK_CENTER_Y - DYE_BANK_Y / 2.0 - 40.0, BASE_Z + 2.0);
    }
    routes
}

fn flush_recovery_collection_nests() -> Part {
    let rack = centered_cube(
        "closed_chip_dye_station_flush_recovery_collection_rack",
        COLLECTION_RACK_X,
        COLLECTION_RACK_Y,
        COLLECTION_RACK_Z,
    )
    .translate(
        COLLECTION_CENTER_X,
        COLLECTION_CENTER_Y,
        BASE_Z + COLLECTION_RACK_Z / 2.0,
    );

    let mut nest_cuts = Part::empty("closed_chip_dye_station_flush_recovery_vial_nest_cuts");
    let mut funnels = Part::empty("closed_chip_dye_station_flush_recovery_funnels");
    for row in 0..COLLECTION_ROWS {
        for col in 0..COLLECTION_COLS {
            let i = row * COLLECTION_COLS + col;
            let (x, y) = collection_center(col, row);
            nest_cuts = nest_cuts
                + centered_cylinder(
                    format!("closed_chip_dye_station_recovery_vial_nest_cut_{i}"),
                    RECOVERY_VIAL_CLEARANCE_D / 2.0,
                    COLLECTION_RACK_Z + 3.0,
                    36,
                )
                .translate(x, y, BASE_Z + COLLECTION_RACK_Z / 2.0);
            let funnel = centered_cylinder(
                format!("closed_chip_dye_station_flush_funnel_rim_{i}"),
                FLUSH_FUNNEL_D / 2.0,
                8.0,
                36,
            )
            .translate(x, y, BASE_Z + COLLECTION_RACK_Z + 4.0);
            let vial_guard = centered_cylinder(
                format!("closed_chip_dye_station_recovery_splash_guard_outer_{i}"),
                RECOVERY_SPLASH_RIM_D / 2.0,
                15.0,
                36,
            )
            .translate(x, y, BASE_Z + COLLECTION_RACK_Z + 7.5);
            let guard_cut = centered_cylinder(
                format!("closed_chip_dye_station_recovery_splash_guard_inner_{i}"),
                (RECOVERY_SPLASH_RIM_D - 7.0) / 2.0,
                17.0,
                36,
            )
            .translate(x, y, BASE_Z + COLLECTION_RACK_Z + 7.5);
            funnels = funnels + funnel + (vial_guard - guard_cut);
        }
    }

    (rack - nest_cuts) + funnels + flush_collection_lane_index_marks()
}

fn flush_collection_lane_index_marks() -> Part {
    let mut marks = Part::empty("closed_chip_dye_station_flush_collection_index_marks");
    for i in 0..FLUSH_PORTS {
        let col = i % COLLECTION_COLS;
        let row = i / COLLECTION_COLS;
        let (x, y) = collection_center(col, row);
        marks = marks
            + centered_cube(
                format!("closed_chip_dye_station_flush_collection_index_tick_{i}"),
                18.0,
                3.0,
                2.0,
            )
            .translate(x, y + 24.0, BASE_Z + COLLECTION_RACK_Z + 1.0);
    }
    marks
}

fn optical_color_reference_lands() -> Part {
    let panel = centered_cube(
        "closed_chip_dye_station_optical_color_reference_panel",
        OPTICAL_PANEL_X,
        OPTICAL_PANEL_Y,
        OPTICAL_PANEL_Z,
    )
    .translate(
        OPTICAL_CENTER_X,
        OPTICAL_CENTER_Y,
        BASE_Z + OPTICAL_PANEL_Z / 2.0,
    );

    let mut lands = Part::empty("closed_chip_dye_station_optical_sample_color_lands");
    for i in 0..COLOR_SAMPLE_LANDS {
        let x = OPTICAL_CENTER_X + indexed_lane(i % 10, 10, 31.0);
        let y = OPTICAL_CENTER_Y + if i < 10 { -24.0 } else { 18.0 };
        lands = lands
            + centered_cube(
                format!("closed_chip_dye_station_optical_sample_color_land_{i}"),
                COLOR_SWATCH_X,
                COLOR_SWATCH_Y,
                2.4,
            )
            .translate(x, y, BASE_Z + OPTICAL_PANEL_Z + 1.2);
    }

    let mut references = Part::empty("closed_chip_dye_station_optical_reference_lands");
    for i in 0..REFERENCE_LANDS {
        references = references
            + centered_cube(
                format!("closed_chip_dye_station_optical_reference_land_{i}"),
                REFERENCE_SWATCH_X,
                REFERENCE_SWATCH_Y,
                3.0,
            )
            .translate(
                OPTICAL_CENTER_X - OPTICAL_PANEL_X / 2.0 + 38.0,
                OPTICAL_CENTER_Y - 34.0 + i as f64 * 13.0,
                BASE_Z + OPTICAL_PANEL_Z + 1.5,
            );
    }

    panel + lands + references + optical_camera_fiducial_strip()
}

fn optical_camera_fiducial_strip() -> Part {
    let mut strip = Part::empty("closed_chip_dye_station_optical_camera_fiducial_strip");
    for i in 0..4 {
        let x = OPTICAL_CENTER_X + OPTICAL_PANEL_X / 2.0 - 42.0;
        let y = OPTICAL_CENTER_Y - 36.0 + i as f64 * 24.0;
        strip = strip
            + centered_cylinder(
                format!("closed_chip_dye_station_optical_camera_fiducial_{i}"),
                5.0,
                2.6,
                28,
            )
            .translate(x, y, BASE_Z + OPTICAL_PANEL_Z + 1.3);
    }
    strip
}

fn pressure_flow_witness_points() -> Part {
    let bar = centered_cube(
        "closed_chip_dye_station_pressure_flow_witness_bar",
        WITNESS_BAR_X,
        WITNESS_BAR_Y,
        WITNESS_BAR_Z,
    )
    .translate(
        WITNESS_CENTER_X,
        WITNESS_CENTER_Y,
        BASE_Z + WITNESS_BAR_Z / 2.0,
    );

    let mut taps = Part::empty("closed_chip_dye_station_pressure_tap_cuts");
    let mut sight_windows = Part::empty("closed_chip_dye_station_flow_sight_windows");
    let mut tick_marks = Part::empty("closed_chip_dye_station_pressure_flow_witness_ticks");
    for i in 0..PRESSURE_WITNESS_POINTS {
        let x = WITNESS_CENTER_X + indexed_lane(i, PRESSURE_WITNESS_POINTS, WITNESS_PITCH_X);
        taps = taps
            + centered_cylinder(
                format!("closed_chip_dye_station_pressure_witness_tap_{i}"),
                PRESSURE_TAP_D / 2.0,
                WITNESS_BAR_Z + 2.0,
                24,
            )
            .translate(x, WITNESS_CENTER_Y - 18.0, BASE_Z + WITNESS_BAR_Z / 2.0);
        sight_windows = sight_windows
            + centered_cube(
                format!("closed_chip_dye_station_flow_witness_window_{i}"),
                FLOW_SIGHT_X,
                FLOW_SIGHT_Y,
                3.0,
            )
            .translate(x, WITNESS_CENTER_Y + 16.0, BASE_Z + WITNESS_BAR_Z + 1.5);
        tick_marks = tick_marks
            + centered_cube(
                format!("closed_chip_dye_station_flow_direction_tick_{i}"),
                11.0,
                2.4,
                2.0,
            )
            .translate(
                x + 1.5,
                WITNESS_CENTER_Y + 29.0,
                BASE_Z + WITNESS_BAR_Z + 1.0,
            );
    }

    (bar - taps) + sight_windows + tick_marks
}

fn waste_capture() -> Part {
    let pan = centered_cube(
        "closed_chip_dye_station_waste_capture_secondary_pan",
        WASTE_PAN_X,
        WASTE_PAN_Y,
        WASTE_PAN_Z,
    )
    .translate(WASTE_CENTER_X, WASTE_CENTER_Y, BASE_Z + WASTE_PAN_Z / 2.0);
    let cavity = centered_cube(
        "closed_chip_dye_station_waste_capture_pan_cavity",
        WASTE_PAN_X - 36.0,
        WASTE_PAN_Y - 34.0,
        WASTE_PAN_Z - 14.0,
    )
    .translate(
        WASTE_CENTER_X,
        WASTE_CENTER_Y,
        BASE_Z + WASTE_PAN_Z / 2.0 + 10.0,
    );

    let mut bottle_cuts = Part::empty("closed_chip_dye_station_waste_bottle_nest_cuts");
    let mut collars = Part::empty("closed_chip_dye_station_waste_bottle_collars");
    for i in 0..WASTE_BOTTLE_NESTS {
        let y = WASTE_CENTER_Y - 54.0 + i as f64 * 108.0;
        bottle_cuts = bottle_cuts
            + centered_cylinder(
                format!("closed_chip_dye_station_waste_bottle_cut_{i}"),
                WASTE_BOTTLE_D / 2.0,
                WASTE_PAN_Z + 3.0,
                40,
            )
            .translate(WASTE_CENTER_X, y, BASE_Z + WASTE_PAN_Z / 2.0);
        collars = collars
            + centered_cylinder(
                format!("closed_chip_dye_station_waste_bottle_collar_{i}"),
                (WASTE_BOTTLE_D + 12.0) / 2.0,
                9.0,
                40,
            )
            .translate(WASTE_CENTER_X, y, BASE_Z + WASTE_PAN_Z + 4.5);
    }

    let trough = centered_cube(
        "closed_chip_dye_station_waste_overflow_trough",
        WASTE_TROUGH_X,
        WASTE_TROUGH_Y,
        WASTE_TROUGH_Z,
    )
    .translate(
        WASTE_CENTER_X - 4.0,
        WASTE_CENTER_Y + WASTE_PAN_Y / 2.0 + WASTE_TROUGH_Y / 2.0 + 8.0,
        BASE_Z + WASTE_TROUGH_Z / 2.0,
    );
    let trough_cut = centered_cube(
        "closed_chip_dye_station_waste_overflow_trough_recess",
        WASTE_TROUGH_X - 24.0,
        WASTE_TROUGH_Y - 14.0,
        WASTE_TROUGH_Z - 8.0,
    )
    .translate(
        WASTE_CENTER_X - 4.0,
        WASTE_CENTER_Y + WASTE_PAN_Y / 2.0 + WASTE_TROUGH_Y / 2.0 + 8.0,
        BASE_Z + WASTE_TROUGH_Z / 2.0 + 6.0,
    );

    (pan - cavity - bottle_cuts) + collars + (trough - trough_cut) + waste_route_arrow_lands()
}

fn waste_route_arrow_lands() -> Part {
    let mut arrows = Part::empty("closed_chip_dye_station_waste_route_arrow_lands");
    for i in 0..5 {
        arrows = arrows
            + centered_cube(
                format!("closed_chip_dye_station_waste_route_arrow_{i}"),
                24.0,
                4.0,
                2.0,
            )
            .translate(
                WASTE_CENTER_X - 82.0 + i as f64 * 36.0,
                WASTE_CENTER_Y + 148.0,
                BASE_Z + 2.0,
            );
    }
    arrows
}

fn barcode_fiducials() -> Part {
    let panel = centered_cube(
        "closed_chip_dye_station_barcode_fiducial_trace_panel",
        TRACE_PANEL_X,
        TRACE_PANEL_Y,
        TRACE_PANEL_Z,
    )
    .translate(TRACE_CENTER_X, TRACE_CENTER_Y, BASE_Z + TRACE_PANEL_Z / 2.0);

    let mut barcode_lands = Part::empty("closed_chip_dye_station_barcode_lands");
    for i in 0..BARCODE_LANDS {
        let x = TRACE_CENTER_X + indexed_lane(i % 4, 4, 76.0);
        let y = TRACE_CENTER_Y + if i < 4 { -24.0 } else { 24.0 };
        barcode_lands = barcode_lands
            + centered_cube(
                format!("closed_chip_dye_station_barcode_land_{i}"),
                BARCODE_LAND_X,
                BARCODE_LAND_Y,
                2.6,
            )
            .translate(x, y, BASE_Z + TRACE_PANEL_Z + 1.3);
    }

    let mut fiducials = Part::empty("closed_chip_dye_station_robot_scan_fiducials");
    for (i, (x, y)) in [
        (
            TRACE_CENTER_X - TRACE_PANEL_X / 2.0 + 28.0,
            TRACE_CENTER_Y - TRACE_PANEL_Y / 2.0 + 24.0,
        ),
        (
            TRACE_CENTER_X + TRACE_PANEL_X / 2.0 - 28.0,
            TRACE_CENTER_Y - TRACE_PANEL_Y / 2.0 + 24.0,
        ),
        (
            TRACE_CENTER_X - TRACE_PANEL_X / 2.0 + 28.0,
            TRACE_CENTER_Y + TRACE_PANEL_Y / 2.0 - 24.0,
        ),
        (
            TRACE_CENTER_X + TRACE_PANEL_X / 2.0 - 28.0,
            TRACE_CENTER_Y + TRACE_PANEL_Y / 2.0 - 24.0,
        ),
        (
            CASSETTE_CENTER_X - CASSETTE_X / 2.0 + 36.0,
            CASSETTE_CENTER_Y + CASSETTE_Y / 2.0 - 36.0,
        ),
        (
            CASSETTE_CENTER_X + CASSETTE_X / 2.0 - 36.0,
            CASSETTE_CENTER_Y - CASSETTE_Y / 2.0 + 36.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + centered_cylinder(
                format!("closed_chip_dye_station_global_fiducial_{i}"),
                FIDUCIAL_D / 2.0,
                3.0,
                32,
            )
            .translate(*x, *y, BASE_Z + 1.5);
    }

    panel + barcode_lands + fiducials
}

fn release_hold_reject_lanes() -> Part {
    let lane_deck = centered_cube(
        "closed_chip_dye_station_release_hold_reject_lane_bank",
        LANE_BANK_X,
        LANE_BANK_Y,
        LANE_BANK_Z,
    )
    .translate(LANE_CENTER_X, LANE_CENTER_Y, BASE_Z + LANE_BANK_Z / 2.0);

    let mut channels = Part::empty("closed_chip_dye_station_disposition_lane_recesses");
    let mut gates = Part::empty("closed_chip_dye_station_disposition_lane_gates");
    for lane in 0..DISPOSITION_LANES {
        let x = LANE_CENTER_X + indexed_lane(lane, DISPOSITION_LANES, LANE_PITCH_X);
        channels = channels
            + centered_cube(
                format!(
                    "closed_chip_dye_station_{}_lane_recess",
                    disposition_lane_label(lane)
                ),
                LANE_WIDTH,
                LANE_BANK_Y - 32.0,
                8.0,
            )
            .translate(x, LANE_CENTER_Y, BASE_Z + LANE_BANK_Z - 3.0);
        gates = gates
            + centered_cube(
                format!(
                    "closed_chip_dye_station_{}_lane_gate",
                    disposition_lane_label(lane)
                ),
                LANE_WIDTH - 26.0,
                9.0,
                LANE_GATE_Z,
            )
            .translate(
                x,
                LANE_CENTER_Y + LANE_BANK_Y / 2.0 - 18.0,
                BASE_Z + LANE_GATE_Z / 2.0,
            );
    }

    (lane_deck - channels) + gates + lane_status_keys()
}

fn lane_status_keys() -> Part {
    let mut keys = Part::empty("closed_chip_dye_station_disposition_lane_status_keys");
    for lane in 0..DISPOSITION_LANES {
        let x = LANE_CENTER_X + indexed_lane(lane, DISPOSITION_LANES, LANE_PITCH_X);
        let key_count = match lane {
            RELEASE_LANE_INDEX => 1,
            HOLD_LANE_INDEX => 2,
            REJECT_LANE_INDEX => 3,
            _ => 0,
        };
        for i in 0..key_count {
            keys = keys
                + centered_cube(
                    format!("closed_chip_dye_station_lane_{lane}_status_key_{i}"),
                    16.0,
                    6.0,
                    3.0,
                )
                .translate(
                    x - 22.0 + i as f64 * 22.0,
                    LANE_CENTER_Y - LANE_BANK_Y / 2.0 + 20.0,
                    BASE_Z + LANE_BANK_Z + 1.5,
                );
        }
    }
    keys
}

fn robot_service_keepouts() -> Part {
    let robot_window = keepout_frame(
        "closed_chip_dye_station_robot_approach_keepout_window",
        CASSETTE_CENTER_X,
        CASSETTE_CENTER_Y,
        ROBOT_APPROACH_WINDOW_X,
        ROBOT_APPROACH_WINDOW_Y,
        ROBOT_KEEP_OUT_Z,
    );
    let front_service = keepout_frame(
        "closed_chip_dye_station_front_operator_service_keepout",
        0.0,
        -STATION_Y / 2.0 + FRONT_SERVICE_CLEARANCE / 2.0,
        STATION_X - 160.0,
        FRONT_SERVICE_CLEARANCE,
        52.0,
    );
    let rear_tube = keepout_frame(
        "closed_chip_dye_station_rear_tubing_service_keepout",
        DYE_BANK_CENTER_X,
        STATION_Y / 2.0 - REAR_TUBE_SERVICE_CLEARANCE / 2.0,
        DYE_BANK_X + 60.0,
        REAR_TUBE_SERVICE_CLEARANCE,
        62.0,
    );
    let right_collection = keepout_frame(
        "closed_chip_dye_station_right_collection_service_keepout",
        STATION_X / 2.0 - RIGHT_COLLECTION_SERVICE_CLEARANCE / 2.0,
        COLLECTION_CENTER_Y,
        RIGHT_COLLECTION_SERVICE_CLEARANCE,
        COLLECTION_RACK_Y + 92.0,
        58.0,
    );
    let left_scan = keepout_frame(
        "closed_chip_dye_station_left_scan_service_keepout",
        -STATION_X / 2.0 + LEFT_SCAN_SERVICE_CLEARANCE / 2.0,
        TRACE_CENTER_Y,
        LEFT_SCAN_SERVICE_CLEARANCE,
        TRACE_PANEL_Y + 96.0,
        46.0,
    );

    robot_window + front_service + rear_tube + right_collection + left_scan
}

fn keepout_frame(name: &str, center_x: f64, center_y: f64, x: f64, y: f64, z: f64) -> Part {
    let rail_w = 8.0;
    let rear = centered_cube(format!("{name}_rear_rail"), x, rail_w, rail_w).translate(
        center_x,
        center_y + y / 2.0 - rail_w / 2.0,
        BASE_Z + z,
    );
    let front = centered_cube(format!("{name}_front_rail"), x, rail_w, rail_w).translate(
        center_x,
        center_y - y / 2.0 + rail_w / 2.0,
        BASE_Z + z,
    );
    let left = centered_cube(format!("{name}_left_rail"), rail_w, y, rail_w).translate(
        center_x - x / 2.0 + rail_w / 2.0,
        center_y,
        BASE_Z + z,
    );
    let right = centered_cube(format!("{name}_right_rail"), rail_w, y, rail_w).translate(
        center_x + x / 2.0 - rail_w / 2.0,
        center_y,
        BASE_Z + z,
    );
    let mut posts = Part::empty(format!("{name}_posts"));
    for (i, (sx, sy)) in [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)]
        .iter()
        .enumerate()
    {
        posts = posts
            + centered_cube(format!("{name}_corner_post_{i}"), rail_w, rail_w, z).translate(
                center_x + sx * (x / 2.0 - rail_w / 2.0),
                center_y + sy * (y / 2.0 - rail_w / 2.0),
                BASE_Z + z / 2.0,
            );
    }

    front + rear + left + right + posts
}

fn chip_center(col: usize, row: usize) -> (f64, f64) {
    (
        CASSETTE_CENTER_X - CHIP_ARRAY_X / 2.0 + REVC_CHIP_LENGTH / 2.0 + col as f64 * CHIP_PITCH_X,
        CASSETTE_CENTER_Y - CHIP_ARRAY_Y / 2.0 + REVC_CHIP_WIDTH / 2.0 + row as f64 * CHIP_PITCH_Y,
    )
}

fn collection_center(col: usize, row: usize) -> (f64, f64) {
    (
        COLLECTION_CENTER_X - (COLLECTION_COLS as f64 - 1.0) * COLLECTION_PITCH_X / 2.0
            + col as f64 * COLLECTION_PITCH_X,
        COLLECTION_CENTER_Y - (COLLECTION_ROWS as f64 - 1.0) * COLLECTION_PITCH_Y / 2.0
            + row as f64 * COLLECTION_PITCH_Y,
    )
}

fn cassette_corner_points(inset: f64) -> [(f64, f64); 4] {
    [
        (
            CASSETTE_CENTER_X - CASSETTE_X / 2.0 + inset,
            CASSETTE_CENTER_Y - CASSETTE_Y / 2.0 + inset,
        ),
        (
            CASSETTE_CENTER_X + CASSETTE_X / 2.0 - inset,
            CASSETTE_CENTER_Y - CASSETTE_Y / 2.0 + inset,
        ),
        (
            CASSETTE_CENTER_X - CASSETTE_X / 2.0 + inset,
            CASSETTE_CENTER_Y + CASSETTE_Y / 2.0 - inset,
        ),
        (
            CASSETTE_CENTER_X + CASSETTE_X / 2.0 - inset,
            CASSETTE_CENTER_Y + CASSETTE_Y / 2.0 - inset,
        ),
    ]
}

fn indexed_lane(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn disposition_lane_label(lane: usize) -> &'static str {
    match lane {
        RELEASE_LANE_INDEX => "release",
        HOLD_LANE_INDEX => "hold",
        REJECT_LANE_INDEX => "reject",
        _ => "unknown",
    }
}

#[cfg(test)]
fn rect_fits_station(center_x: f64, center_y: f64, x: f64, y: f64, margin: f64) -> bool {
    center_x.abs() + x / 2.0 <= STATION_X / 2.0 - margin
        && center_y.abs() + y / 2.0 <= STATION_Y / 2.0 - margin
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        assert_eq!(REQUIRED_FEATURES.len(), 12);
        for path in OUTPUTS {
            assert!(path
                .starts_with("output/closed_chip_inlet_outlet_dead_volume_dye_recovery_station_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn cassette_surrogate_has_twenty_no_cell_positions() {
        let mut positions = BTreeSet::new();
        for row in 0..CHIP_ROWS {
            for col in 0..CHIP_COLS {
                positions.insert((col, row));
                let (x, y) = chip_center(col, row);
                assert!(x < CASSETTE_CENTER_X + CASSETTE_X / 2.0);
                assert!(x > CASSETTE_CENTER_X - CASSETTE_X / 2.0);
                assert!(y < CASSETTE_CENTER_Y + CASSETTE_Y / 2.0);
                assert!(y > CASSETTE_CENTER_Y - CASSETTE_Y / 2.0);
            }
        }

        assert_eq!(CHIP_COLS, 4);
        assert_eq!(CHIP_ROWS, 5);
        assert_eq!(positions.len(), 20);
        assert_eq!(CHIP_POSITIONS, 20);
        assert!(NO_CELL_OBSERVATION_WINDOW_X < REVC_CHIP_LENGTH);
        assert!(NO_CELL_OBSERVATION_WINDOW_Y < REVC_CHIP_WIDTH);
    }

    #[test]
    fn low_volume_and_witness_interfaces_are_per_position() {
        assert_eq!(INLET_ADAPTERS, CHIP_POSITIONS);
        assert_eq!(OUTLET_ADAPTERS, CHIP_POSITIONS);
        assert_eq!(DYE_INJECTION_PORTS, CHIP_POSITIONS);
        assert_eq!(FLUSH_PORTS, CHIP_POSITIONS);
        assert_eq!(COLLECTION_NESTS, CHIP_POSITIONS);
        assert_eq!(PRESSURE_WITNESS_POINTS, CHIP_POSITIONS);
        assert_eq!(FLOW_WITNESS_POINTS, CHIP_POSITIONS);
        assert!(ADAPTER_BORE_D <= 1.6);
        assert!(ADAPTER_HOLDUP_UL <= 10.0);
        assert!(ADAPTER_TUBE_D > ADAPTER_BORE_D);
    }

    #[test]
    fn collection_and_disposition_cover_recovery_workflow() {
        assert_eq!(COLLECTION_COLS * COLLECTION_ROWS, 20);
        assert!(RECOVERY_VIAL_CLEARANCE_D > RECOVERY_VIAL_D);
        assert!(FLUSH_FUNNEL_D > RECOVERY_VIAL_CLEARANCE_D);
        assert_eq!(DISPOSITION_LANES, 3);
        assert_eq!(disposition_lane_label(RELEASE_LANE_INDEX), "release");
        assert_eq!(disposition_lane_label(HOLD_LANE_INDEX), "hold");
        assert_eq!(disposition_lane_label(REJECT_LANE_INDEX), "reject");
        assert!(WASTE_CAPTURE_VOLUME_ML >= 800.0);
    }

    #[test]
    fn traceability_and_optical_references_are_explicit() {
        assert_eq!(COLOR_SAMPLE_LANDS, CHIP_POSITIONS);
        assert_eq!(REFERENCE_LANDS, 6);
        assert!(REFERENCE_SWATCH_X > COLOR_SWATCH_X);
        assert_eq!(BARCODE_LANDS, 8);
        assert_eq!(FIDUCIALS, 6);
        assert_eq!(DYE_REFERENCE_WELLS, 4);
        assert_eq!(LEAK_SENSOR_WELLS, 4);
    }

    #[test]
    fn modules_fit_within_station_footprint() {
        assert!(rect_fits_station(
            CASSETTE_CENTER_X,
            CASSETTE_CENTER_Y,
            CASSETTE_X,
            CASSETTE_Y,
            24.0
        ));
        assert!(rect_fits_station(
            COLLECTION_CENTER_X,
            COLLECTION_CENTER_Y,
            COLLECTION_RACK_X,
            COLLECTION_RACK_Y,
            24.0
        ));
        assert!(rect_fits_station(
            OPTICAL_CENTER_X,
            OPTICAL_CENTER_Y,
            OPTICAL_PANEL_X,
            OPTICAL_PANEL_Y,
            24.0
        ));
        assert!(rect_fits_station(
            WITNESS_CENTER_X,
            WITNESS_CENTER_Y,
            WITNESS_BAR_X,
            WITNESS_BAR_Y,
            24.0
        ));
        assert!(rect_fits_station(
            WASTE_CENTER_X,
            WASTE_CENTER_Y,
            WASTE_PAN_X,
            WASTE_PAN_Y,
            18.0
        ));
        assert!(rect_fits_station(
            TRACE_CENTER_X,
            TRACE_CENTER_Y,
            TRACE_PANEL_X,
            TRACE_PANEL_Y,
            18.0
        ));
        assert!(rect_fits_station(
            LANE_CENTER_X,
            LANE_CENTER_Y,
            LANE_BANK_X,
            LANE_BANK_Y,
            18.0
        ));
    }

    #[test]
    fn service_keepouts_are_modeled_as_clearance_envelopes() {
        assert_eq!(ROBOT_KEEP_OUT_WINDOWS, 4);
        assert!(ROBOT_APPROACH_WINDOW_X > CASSETTE_X);
        assert!(ROBOT_APPROACH_WINDOW_Y > CASSETTE_Y);
        assert!(FRONT_SERVICE_CLEARANCE >= 300.0);
        assert!(REAR_TUBE_SERVICE_CLEARANCE >= 200.0);
        assert!(RIGHT_COLLECTION_SERVICE_CLEARANCE >= 180.0);
        assert!(LEFT_SCAN_SERVICE_CLEARANCE >= 150.0);
    }
}
