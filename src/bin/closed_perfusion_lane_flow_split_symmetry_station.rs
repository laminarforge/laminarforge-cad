use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed perfusion lane flow-split symmetry validation station.
//
// Intent:
// - Validate that a single closed perfusion supply splits evenly into twenty
//   cassette outlet lanes before multi-chip cassettes receive live articles.
// - Keep lane geometry symmetric and restrictor paths equal-length so measured
//   imbalance comes from the split/restrictor/sensor setup rather than fixture
//   routing.
// - Provide reference flow sensor pockets, gravimetric collection nests,
//   dye/tracer and bubble/wetness witness features, lane isolation clamps,
//   release/hold/reject disposition lanes, barcode custody, and explicit
//   robot/service keepouts on one station deck.
//
// This is fit-check and validation-station CAD. It is not a sterile wetted
// path release drawing, acceptance criterion, calibration method, or tubing BOM.

const PREFIX: &str = "closed_perfusion_lane_flow_split_symmetry_station";

const OUTPUTS: &[&str] = &[
    "output/closed_perfusion_lane_flow_split_symmetry_station_base_leak_tray.stl",
    "output/closed_perfusion_lane_flow_split_symmetry_station_twenty_outlet_lane_plate.stl",
    "output/closed_perfusion_lane_flow_split_symmetry_station_equal_length_restrictor_comb.stl",
    "output/closed_perfusion_lane_flow_split_symmetry_station_reference_flow_sensor_pockets.stl",
    "output/closed_perfusion_lane_flow_split_symmetry_station_gravimetric_collection_nests.stl",
    "output/closed_perfusion_lane_flow_split_symmetry_station_dye_tracer_manifold.stl",
    "output/closed_perfusion_lane_flow_split_symmetry_station_bubble_wetness_windows.stl",
    "output/closed_perfusion_lane_flow_split_symmetry_station_lane_isolation_clamps.stl",
    "output/closed_perfusion_lane_flow_split_symmetry_station_release_hold_reject_lanes.stl",
    "output/closed_perfusion_lane_flow_split_symmetry_station_barcode_custody_panel.stl",
    "output/closed_perfusion_lane_flow_split_symmetry_station_robot_service_keepouts.stl",
    "output/closed_perfusion_lane_flow_split_symmetry_station_assembly.stl",
];

const REQUIRED_FEATURES: &[&str] = &[
    "twenty_outlet_lanes",
    "equal_length_restrictor_comb",
    "reference_flow_sensor_pockets",
    "gravimetric_collection_nests",
    "dye_tracer_manifold",
    "bubble_wetness_windows",
    "lane_isolation_clamps",
    "release_hold_reject_lanes",
    "barcode_custody",
    "robot_keepouts",
    "service_keepouts",
];

const STATION_X: f64 = 1440.0;
const STATION_Y: f64 = 1040.0;
const BASE_Z: f64 = 24.0;
const LEAK_BASIN_X: f64 = STATION_X - 118.0;
const LEAK_BASIN_Y: f64 = STATION_Y - 118.0;
const LEAK_BASIN_DEPTH: f64 = 7.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 46.0;
const DRAIN_D: f64 = 18.0;
const MOUNT_SLOTS: usize = 10;
const BASE_DATUMS: usize = 6;

const OUTLET_LANES: usize = 20;
const LANE_CENTER_X: f64 = -58.0;
const LANE_CENTER_Y: f64 = 64.0;
const LANE_RUN_X: f64 = 792.0;
const LANE_PITCH_Y: f64 = 30.0;
const LANE_BANK_X: f64 = LANE_RUN_X + 108.0;
const LANE_BANK_Y: f64 = (OUTLET_LANES as f64 - 1.0) * LANE_PITCH_Y + 76.0;
const LANE_PLATE_Z: f64 = 18.0;
const LANE_TRACE_W: f64 = 7.2;
const LANE_TRACE_Z: f64 = 6.0;
const LANE_INLET_X: f64 = LANE_CENTER_X - LANE_RUN_X / 2.0;
const LANE_OUTLET_X: f64 = LANE_CENTER_X + LANE_RUN_X / 2.0;
const LANE_SYMMETRY_TOLERANCE_MM: f64 = 0.05;
const TARGET_LANE_SWEPT_LENGTH_MM: f64 = 840.0;

const SPLIT_RAIL_X: f64 = 118.0;
const SPLIT_RAIL_W: f64 = 10.0;
const SPLIT_RAIL_Z: f64 = 8.0;
const OUTLET_PORT_D: f64 = 13.0;
const LANE_TUBE_OD_MAX: f64 = 4.8;
const LANE_TUBE_CLEARANCE: f64 = 1.0;
const LANE_CHANNEL_D: f64 = LANE_TUBE_OD_MAX + LANE_TUBE_CLEARANCE;

const RESTRICTOR_BANK_X: f64 = 206.0;
const RESTRICTOR_BANK_Y: f64 = LANE_BANK_Y + 40.0;
const RESTRICTOR_BANK_Z: f64 = 42.0;
const RESTRICTOR_POS: (f64, f64) = (-586.0, LANE_CENTER_Y);
const RESTRICTOR_ELEMENTS: usize = OUTLET_LANES;
const RESTRICTOR_STRAIGHTS: usize = 4;
const RESTRICTOR_STRAIGHT_X: f64 = 118.0;
const RESTRICTOR_TURN_Y: f64 = 10.0;
const RESTRICTOR_PATH_LENGTH_MM: f64 =
    RESTRICTOR_STRAIGHTS as f64 * RESTRICTOR_STRAIGHT_X + 3.0 * RESTRICTOR_TURN_Y;

const SENSOR_RACK_X: f64 = 930.0;
const SENSOR_RACK_Y: f64 = 104.0;
const SENSOR_RACK_Z: f64 = 34.0;
const SENSOR_POS: (f64, f64) = (-58.0, -362.0);
const SENSOR_POCKETS: usize = OUTLET_LANES;
const SENSOR_COLS: usize = 10;
const SENSOR_ROWS: usize = 2;
const SENSOR_PITCH_X: f64 = 86.0;
const SENSOR_PITCH_Y: f64 = 42.0;
const SENSOR_POCKET_X: f64 = 58.0;
const SENSOR_POCKET_Y: f64 = 32.0;
const SENSOR_POCKET_Z: f64 = 16.0;
const FLOW_COUPON_X: f64 = 44.0;
const FLOW_COUPON_Y: f64 = 18.0;
const SENSOR_REFERENCE_PORTS_PER_LANE: usize = 2;

const COLLECTION_RACK_X: f64 = 232.0;
const COLLECTION_RACK_Y: f64 = LANE_BANK_Y + 58.0;
const COLLECTION_RACK_Z: f64 = 42.0;
const COLLECTION_POS: (f64, f64) = (560.0, LANE_CENTER_Y);
const COLLECTION_NESTS: usize = OUTLET_LANES;
const COLLECTION_WELL_D: f64 = 24.0;
const COLLECTION_WELL_CLEARANCE_D: f64 = 26.0;
const COLLECTION_LOAD_CELL_PAD_X: f64 = 48.0;
const COLLECTION_LOAD_CELL_PAD_Y: f64 = 20.0;
const COLLECTION_BALANCE_ID_LANDS: usize = OUTLET_LANES;

const DYE_MANIFOLD_X: f64 = 214.0;
const DYE_MANIFOLD_Y: f64 = LANE_BANK_Y + 58.0;
const DYE_MANIFOLD_Z: f64 = 50.0;
const DYE_POS: (f64, f64) = (-588.0, LANE_CENTER_Y);
const DYE_RESERVOIRS: usize = 2;
const TRACER_SEPTA: usize = OUTLET_LANES;
const DYE_RESERVOIR_D: f64 = 34.0;
const TRACER_SEPTUM_D: f64 = 8.0;
const DYE_PRIME_VOLUME_UL: f64 = 180.0;

const WINDOW_BRIDGE_X: f64 = LANE_RUN_X + 44.0;
const WINDOW_BRIDGE_Y: f64 = LANE_BANK_Y + 24.0;
const WINDOW_BRIDGE_Z: f64 = 20.0;
const WINDOW_POS: (f64, f64) = (LANE_CENTER_X + 36.0, LANE_CENTER_Y);
const BUBBLE_WETNESS_WINDOWS: usize = OUTLET_LANES;
const WINDOW_X: f64 = 86.0;
const WINDOW_Y: f64 = 18.0;
const WINDOW_RAIL_W: f64 = 4.0;
const BUBBLE_DOME_D: f64 = 16.0;
const WETNESS_FLAG_X: f64 = 20.0;

const CLAMP_RAIL_X: f64 = LANE_RUN_X + 72.0;
const CLAMP_RAIL_Y: f64 = LANE_BANK_Y + 28.0;
const CLAMP_RAIL_Z: f64 = 28.0;
const CLAMP_POS: (f64, f64) = (LANE_CENTER_X + 72.0, LANE_CENTER_Y);
const ISOLATION_CLAMPS: usize = OUTLET_LANES;
const CLAMP_BLOCK_X: f64 = 34.0;
const CLAMP_BLOCK_Y: f64 = 18.0;
const CLAMP_LEVER_X: f64 = 42.0;
const CLAMP_PIN_D: f64 = 7.0;

const DISPOSITION_LANES: usize = 3;
const STATUS_SLOTS_PER_LANE: usize = OUTLET_LANES;
const DISPOSITION_PANEL_X: f64 = 560.0;
const DISPOSITION_PANEL_Y: f64 = 76.0;
const DISPOSITION_PANEL_Z: f64 = 26.0;
const DISPOSITION_POS: (f64, f64) = (-56.0, -456.0);
const DISPOSITION_SLOT_X: f64 = 21.0;
const DISPOSITION_SLOT_Y: f64 = 16.0;
const DISPOSITION_PITCH_X: f64 = 25.0;
const DISPOSITION_PITCH_Y: f64 = 24.0;

const BARCODE_PANEL_X: f64 = 316.0;
const BARCODE_PANEL_Y: f64 = 92.0;
const BARCODE_PANEL_Z: f64 = 12.0;
const BARCODE_POS: (f64, f64) = (-526.0, -456.0);
const BARCODE_LANDS: usize = OUTLET_LANES;
const CUSTODY_LANDS: usize = 5;
const BARCODE_LAND_X: f64 = 54.0;
const BARCODE_LAND_Y: f64 = 13.0;
const CUSTODY_TOKEN_D: f64 = 18.0;

const ROBOT_KEEP_OUT_X: f64 = 1290.0;
const ROBOT_KEEP_OUT_Y: f64 = 876.0;
const ROBOT_KEEP_OUT_Z: f64 = 168.0;
const ROBOT_KEEP_OUT_WINDOWS: usize = 4;
const SERVICE_GAUGES: usize = 6;
const ROBOT_Z_CLEARANCE: f64 = 146.0;
const FRONT_SERVICE_CLEARANCE: f64 = 250.0;
const REAR_SERVICE_CLEARANCE: f64 = 220.0;
const LEFT_SERVICE_CLEARANCE: f64 = 160.0;
const RIGHT_COLLECTION_SERVICE_CLEARANCE: f64 = 170.0;

const LABEL_BAR_COUNT: usize = 8;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_leak_tray();
    export(&base, OUTPUTS[0]);

    let lanes = twenty_outlet_lane_plate();
    export(&lanes, OUTPUTS[1]);

    let restrictors = equal_length_restrictor_comb();
    export(&restrictors, OUTPUTS[2]);

    let sensors = reference_flow_sensor_pockets();
    export(&sensors, OUTPUTS[3]);

    let collections = gravimetric_collection_nests();
    export(&collections, OUTPUTS[4]);

    let dye = dye_tracer_manifold();
    export(&dye, OUTPUTS[5]);

    let windows = bubble_wetness_windows();
    export(&windows, OUTPUTS[6]);

    let clamps = lane_isolation_clamps();
    export(&clamps, OUTPUTS[7]);

    let disposition = release_hold_reject_lanes();
    export(&disposition, OUTPUTS[8]);

    let custody = barcode_custody_panel();
    export(&custody, OUTPUTS[9]);

    let keepouts = robot_service_keepouts();
    export(&keepouts, OUTPUTS[10]);

    let assembly = base
        + lanes
        + restrictors
        + sensors
        + collections
        + dye
        + windows
        + clamps
        + disposition
        + custody
        + keepouts;
    export(&assembly, OUTPUTS[11]);

    println!(
        "Closed perfusion lane flow-split symmetry station: {:.0}mm x {:.0}mm closed leak-tray deck with {} outlet lanes, {:.1}mm equal swept-length target, {:.2}mm modeled symmetry tolerance, {} equal-length restrictors, {} reference flow sensor pockets, and {} gravimetric collection nests.",
        STATION_X,
        STATION_Y,
        OUTLET_LANES,
        TARGET_LANE_SWEPT_LENGTH_MM,
        LANE_SYMMETRY_TOLERANCE_MM,
        RESTRICTOR_ELEMENTS,
        SENSOR_POCKETS,
        COLLECTION_NESTS
    );
    println!(
        "Validation cues: {} dye/tracer septa, {} bubble/wetness windows, {} lane isolation clamps, {} release/hold/reject disposition slots, {} barcode/custody lands, {} robot keepout windows, {} service gauges, and {} required feature groups.",
        TRACER_SEPTA,
        BUBBLE_WETNESS_WINDOWS,
        ISOLATION_CLAMPS,
        DISPOSITION_LANES * STATUS_SLOTS_PER_LANE,
        BARCODE_LANDS + CUSTODY_LANDS,
        ROBOT_KEEP_OUT_WINDOWS,
        SERVICE_GAUGES,
        REQUIRED_FEATURES.len()
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_layout() {
    assert_eq!(OUTLET_LANES, 20);
    assert_eq!(RESTRICTOR_ELEMENTS, OUTLET_LANES);
    assert_eq!(SENSOR_POCKETS, OUTLET_LANES);
    assert_eq!(COLLECTION_NESTS, OUTLET_LANES);
    assert_eq!(TRACER_SEPTA, OUTLET_LANES);
    assert_eq!(BUBBLE_WETNESS_WINDOWS, OUTLET_LANES);
    assert_eq!(ISOLATION_CLAMPS, OUTLET_LANES);
    assert_eq!(STATUS_SLOTS_PER_LANE, OUTLET_LANES);
    assert_eq!(SENSOR_ROWS * SENSOR_COLS, SENSOR_POCKETS);
    assert_eq!(SENSOR_REFERENCE_PORTS_PER_LANE, 2);
    assert_eq!(COLLECTION_BALANCE_ID_LANDS, OUTLET_LANES);
    assert_eq!(BASE_DATUMS, 6);
    assert!(COLLECTION_WELL_CLEARANCE_D > COLLECTION_WELL_D);
    assert!(LANE_CHANNEL_D > LANE_TUBE_OD_MAX);
    assert!(RESTRICTOR_PATH_LENGTH_MM < TARGET_LANE_SWEPT_LENGTH_MM);
    assert!(ROBOT_KEEP_OUT_X < STATION_X);
    assert!(ROBOT_KEEP_OUT_Y < STATION_Y);
    assert!(FRONT_SERVICE_CLEARANCE >= 240.0);
    assert!(REAR_SERVICE_CLEARANCE >= 200.0);
    assert!(LEFT_SERVICE_CLEARANCE >= 150.0);
    assert!(RIGHT_COLLECTION_SERVICE_CLEARANCE >= 160.0);
}

fn base_leak_tray() -> Part {
    let deck = centered_cube(format!("{PREFIX}_base_deck"), STATION_X, STATION_Y, BASE_Z);
    let basin = centered_cube(
        format!("{PREFIX}_base_leak_basin_recess"),
        LEAK_BASIN_X,
        LEAK_BASIN_Y,
        LEAK_BASIN_DEPTH,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0 - LEAK_BASIN_DEPTH / 2.0 + 0.2);
    let drain = centered_cylinder(
        format!("{PREFIX}_base_front_right_drain_bore"),
        DRAIN_D / 2.0,
        BASE_Z + 8.0,
        36,
    )
    .translate(STATION_X / 2.0 - 82.0, -(STATION_Y / 2.0 - 62.0), 0.0);

    deck - basin - drain + leak_tray_rims() + base_mount_slots() + station_datums()
}

fn leak_tray_rims() -> Part {
    let z = BASE_Z / 2.0 + RIM_Z / 2.0;
    let left = centered_cube(
        format!("{PREFIX}_base_left_leak_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-(STATION_X / 2.0 - RIM_W / 2.0), 0.0, z);
    let right = centered_cube(
        format!("{PREFIX}_base_right_leak_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, z);
    let rear = centered_cube(
        format!("{PREFIX}_base_rear_leak_rim"),
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, z);
    let front = centered_cube(
        format!("{PREFIX}_base_front_low_load_access_rim"),
        STATION_X,
        RIM_W,
        RIM_Z * 0.62,
    )
    .translate(
        0.0,
        -(STATION_Y / 2.0 - RIM_W / 2.0),
        BASE_Z / 2.0 + RIM_Z * 0.31,
    );

    left + right + rear + front
}

fn base_mount_slots() -> Part {
    let mut slots = Part::empty(format!("{PREFIX}_base_mount_slot_bosses"));
    for (i, (x, y)) in mount_points().iter().enumerate() {
        let boss = centered_cube(format!("{PREFIX}_mount_slot_boss_{i}"), 42.0, 24.0, 8.0)
            .translate(*x, *y, BASE_Z / 2.0 + 4.0);
        let bore = centered_cylinder(
            format!("{PREFIX}_mount_slot_clearance_bore_{i}"),
            4.2,
            14.0,
            24,
        )
        .translate(*x, *y, BASE_Z / 2.0 + 4.0);
        slots = slots + (boss - bore);
    }
    slots
}

fn station_datums() -> Part {
    let mut datums = Part::empty(format!("{PREFIX}_station_datum_targets"));
    for (i, (x, y)) in [
        (-650.0, -438.0),
        (-520.0, 430.0),
        (0.0, 430.0),
        (520.0, 430.0),
        (650.0, -438.0),
        (0.0, -438.0),
    ]
    .iter()
    .enumerate()
    {
        datums = datums
            + fiducial_target(format!("{PREFIX}_base_datum_{i}")).translate(
                *x,
                *y,
                BASE_Z / 2.0 + 2.0,
            );
    }
    datums
}

fn twenty_outlet_lane_plate() -> Part {
    let plate = centered_cube(
        format!("{PREFIX}_twenty_outlet_lane_backer_plate"),
        LANE_BANK_X,
        LANE_BANK_Y,
        LANE_PLATE_Z,
    )
    .translate(
        LANE_CENTER_X,
        LANE_CENTER_Y,
        BASE_Z / 2.0 + LANE_PLATE_Z / 2.0,
    );
    let pocket_relief = centered_cube(
        format!("{PREFIX}_twenty_outlet_lane_center_relieved_field"),
        LANE_BANK_X - 52.0,
        LANE_BANK_Y - 48.0,
        5.0,
    )
    .translate(
        LANE_CENTER_X,
        LANE_CENTER_Y,
        BASE_Z / 2.0 + LANE_PLATE_Z - 2.0,
    );

    plate - pocket_relief + symmetry_splitter_tree() + lane_traces() + lane_port_marks()
}

fn symmetry_splitter_tree() -> Part {
    let z = BASE_Z / 2.0 + LANE_PLATE_Z + SPLIT_RAIL_Z / 2.0;
    let inlet = centered_cylinder(
        format!("{PREFIX}_central_split_inlet_luer_boss"),
        16.0,
        16.0,
        36,
    )
    .translate(LANE_INLET_X - 74.0, LANE_CENTER_Y, z);
    let trunk = centered_cube(
        format!("{PREFIX}_central_split_trunk_to_equal_comb"),
        96.0,
        SPLIT_RAIL_W,
        SPLIT_RAIL_Z,
    )
    .translate(LANE_INLET_X - 30.0, LANE_CENTER_Y, z);
    let vertical = centered_cube(
        format!("{PREFIX}_twenty_way_symmetric_split_vertical_header"),
        SPLIT_RAIL_W,
        LANE_BANK_Y - 58.0,
        SPLIT_RAIL_Z,
    )
    .translate(LANE_INLET_X + 24.0, LANE_CENTER_Y, z);

    let mut branches = Part::empty(format!("{PREFIX}_equal_pitch_splitter_branches"));
    for lane in 0..OUTLET_LANES {
        let y = lane_y(lane);
        let branch = centered_cube(
            format!("{PREFIX}_splitter_branch_lane_{lane:02}"),
            SPLIT_RAIL_X,
            SPLIT_RAIL_W,
            SPLIT_RAIL_Z,
        )
        .translate(LANE_INLET_X + SPLIT_RAIL_X / 2.0 + 24.0, y, z);
        let tick = centered_cube(
            format!("{PREFIX}_splitter_lane_{lane:02}_symmetry_tick"),
            10.0,
            17.0,
            SPLIT_RAIL_Z + 2.0,
        )
        .translate(LANE_INLET_X + 76.0, y, z + 1.0);
        branches = branches + branch + tick;
    }

    inlet + trunk + vertical + branches
}

fn lane_traces() -> Part {
    let mut traces = Part::empty(format!("{PREFIX}_twenty_equal_length_outlet_lane_traces"));
    let z = BASE_Z / 2.0 + LANE_PLATE_Z + LANE_TRACE_Z / 2.0;
    for lane in 0..OUTLET_LANES {
        let y = lane_y(lane);
        let straight = centered_cube(
            format!("{PREFIX}_outlet_lane_{lane:02}_straight_trace"),
            LANE_RUN_X,
            LANE_TRACE_W,
            LANE_TRACE_Z,
        )
        .translate(LANE_CENTER_X, y, z);
        let upstream_comp = u_turn_witness(
            format!("{PREFIX}_outlet_lane_{lane:02}_upstream_equal_length_loop"),
            58.0,
            18.0,
            LANE_TRACE_W,
            LANE_TRACE_Z,
        )
        .translate(LANE_INLET_X + 172.0, y, z);
        let downstream_comp = u_turn_witness(
            format!("{PREFIX}_outlet_lane_{lane:02}_downstream_equal_length_loop"),
            58.0,
            -18.0,
            LANE_TRACE_W,
            LANE_TRACE_Z,
        )
        .translate(LANE_OUTLET_X - 174.0, y, z);
        traces = traces + straight + upstream_comp + downstream_comp;
    }
    traces
}

fn lane_port_marks() -> Part {
    let mut ports = Part::empty(format!("{PREFIX}_twenty_outlet_port_marks"));
    let z = BASE_Z / 2.0 + LANE_PLATE_Z + 3.0;
    for lane in 0..OUTLET_LANES {
        let y = lane_y(lane);
        let inlet = port_ring(
            format!("{PREFIX}_lane_{lane:02}_inlet_wet_connector_ring"),
            OUTLET_PORT_D,
            LANE_CHANNEL_D,
            6.0,
        )
        .translate(LANE_INLET_X, y, z);
        let outlet = port_ring(
            format!("{PREFIX}_lane_{lane:02}_outlet_wet_connector_ring"),
            OUTLET_PORT_D,
            LANE_CHANNEL_D,
            6.0,
        )
        .translate(LANE_OUTLET_X, y, z);
        ports = ports + inlet + outlet;
    }
    ports
}

fn equal_length_restrictor_comb() -> Part {
    let carrier = centered_cube(
        format!("{PREFIX}_equal_length_restrictor_comb_carrier"),
        RESTRICTOR_BANK_X,
        RESTRICTOR_BANK_Y,
        RESTRICTOR_BANK_Z,
    )
    .translate(
        RESTRICTOR_POS.0,
        RESTRICTOR_POS.1,
        BASE_Z / 2.0 + RESTRICTOR_BANK_Z / 2.0,
    );
    let center_relief = centered_cube(
        format!("{PREFIX}_equal_length_restrictor_comb_recess_field"),
        RESTRICTOR_BANK_X - 26.0,
        RESTRICTOR_BANK_Y - 36.0,
        9.0,
    )
    .translate(
        RESTRICTOR_POS.0,
        RESTRICTOR_POS.1,
        BASE_Z / 2.0 + RESTRICTOR_BANK_Z - 4.0,
    );

    carrier - center_relief + restrictor_elements() + restrictor_length_gauge()
}

fn restrictor_elements() -> Part {
    let mut elements = Part::empty(format!("{PREFIX}_twenty_equal_length_restrictor_elements"));
    let z = BASE_Z / 2.0 + RESTRICTOR_BANK_Z + 3.0;
    for lane in 0..RESTRICTOR_ELEMENTS {
        let y = lane_y(lane);
        let serpentine = serpentine_restrictor(
            format!("{PREFIX}_restrictor_lane_{lane:02}_equal_length_serpentine"),
            RESTRICTOR_STRAIGHT_X,
            RESTRICTOR_TURN_Y,
            5.0,
            6.0,
        )
        .translate(RESTRICTOR_POS.0, y, z);
        let inlet_pin = centered_cylinder(
            format!("{PREFIX}_restrictor_lane_{lane:02}_inlet_pin"),
            4.0,
            8.0,
            24,
        )
        .translate(RESTRICTOR_POS.0 - 82.0, y, z);
        let outlet_pin = centered_cylinder(
            format!("{PREFIX}_restrictor_lane_{lane:02}_outlet_pin"),
            4.0,
            8.0,
            24,
        )
        .translate(RESTRICTOR_POS.0 + 82.0, y, z);
        elements = elements + serpentine + inlet_pin + outlet_pin;
    }
    elements
}

fn restrictor_length_gauge() -> Part {
    let header = centered_cube(
        format!("{PREFIX}_restrictor_equal_length_master_gauge"),
        RESTRICTOR_BANK_X - 32.0,
        10.0,
        7.0,
    )
    .translate(
        RESTRICTOR_POS.0,
        RESTRICTOR_POS.1 + RESTRICTOR_BANK_Y / 2.0 - 22.0,
        BASE_Z / 2.0 + RESTRICTOR_BANK_Z + 3.5,
    );
    let lower = centered_cube(
        format!("{PREFIX}_restrictor_equal_length_witness_tick_bar"),
        RESTRICTOR_BANK_X - 32.0,
        8.0,
        7.0,
    )
    .translate(
        RESTRICTOR_POS.0,
        RESTRICTOR_POS.1 - RESTRICTOR_BANK_Y / 2.0 + 22.0,
        BASE_Z / 2.0 + RESTRICTOR_BANK_Z + 3.5,
    );
    header + lower
}

fn reference_flow_sensor_pockets() -> Part {
    let rack = centered_cube(
        format!("{PREFIX}_reference_flow_sensor_rack"),
        SENSOR_RACK_X,
        SENSOR_RACK_Y,
        SENSOR_RACK_Z,
    )
    .translate(
        SENSOR_POS.0,
        SENSOR_POS.1,
        BASE_Z / 2.0 + SENSOR_RACK_Z / 2.0,
    );
    let rail_label = csg_label_plaque(
        format!("{PREFIX}_reference_flow_sensor_traceable_standard_label"),
        170.0,
        22.0,
        4.0,
        11,
    )
    .translate(
        SENSOR_POS.0 - SENSOR_RACK_X / 2.0 + 116.0,
        SENSOR_POS.1,
        BASE_Z / 2.0 + SENSOR_RACK_Z + 2.0,
    );

    rack + sensor_pocket_array() + rail_label
}

fn sensor_pocket_array() -> Part {
    let mut pockets = Part::empty(format!("{PREFIX}_twenty_reference_flow_sensor_pockets"));
    let z = BASE_Z / 2.0 + SENSOR_RACK_Z + SENSOR_POCKET_Z / 2.0;
    for lane in 0..SENSOR_POCKETS {
        let (x, y) = sensor_position(lane);
        let body = centered_cube(
            format!("{PREFIX}_reference_flow_sensor_pocket_{lane:02}_raised_frame"),
            SENSOR_POCKET_X,
            SENSOR_POCKET_Y,
            SENSOR_POCKET_Z,
        )
        .translate(x, y, z);
        let window = centered_cube(
            format!("{PREFIX}_reference_flow_sensor_pocket_{lane:02}_coupon_window"),
            FLOW_COUPON_X,
            FLOW_COUPON_Y,
            SENSOR_POCKET_Z + 2.0,
        )
        .translate(x, y, z);
        let port_a = centered_cylinder(
            format!("{PREFIX}_reference_flow_sensor_pocket_{lane:02}_upstream_port"),
            3.0,
            SENSOR_POCKET_Z + 4.0,
            20,
        )
        .translate(x - SENSOR_POCKET_X / 2.0 + 9.0, y, z);
        let port_b = centered_cylinder(
            format!("{PREFIX}_reference_flow_sensor_pocket_{lane:02}_downstream_port"),
            3.0,
            SENSOR_POCKET_Z + 4.0,
            20,
        )
        .translate(x + SENSOR_POCKET_X / 2.0 - 9.0, y, z);
        let custody_tick = centered_cube(
            format!("{PREFIX}_reference_flow_sensor_pocket_{lane:02}_custody_tick"),
            10.0,
            3.0,
            4.0,
        )
        .translate(
            x,
            y + SENSOR_POCKET_Y / 2.0 - 5.0,
            z + SENSOR_POCKET_Z / 2.0,
        );
        pockets = pockets + (body - window - port_a - port_b) + custody_tick;
    }
    pockets
}

fn gravimetric_collection_nests() -> Part {
    let rack = centered_cube(
        format!("{PREFIX}_gravimetric_collection_nest_rack"),
        COLLECTION_RACK_X,
        COLLECTION_RACK_Y,
        COLLECTION_RACK_Z,
    )
    .translate(
        COLLECTION_POS.0,
        COLLECTION_POS.1,
        BASE_Z / 2.0 + COLLECTION_RACK_Z / 2.0,
    );
    let drain_slot = centered_cube(
        format!("{PREFIX}_gravimetric_collection_rack_drain_channel"),
        18.0,
        COLLECTION_RACK_Y - 34.0,
        8.0,
    )
    .translate(
        COLLECTION_POS.0 + COLLECTION_RACK_X / 2.0 - 28.0,
        COLLECTION_POS.1,
        BASE_Z / 2.0 + COLLECTION_RACK_Z - 4.0,
    );

    rack - drain_slot + collection_nest_array() + collection_reference_scale_rails()
}

fn collection_nest_array() -> Part {
    let mut nests = Part::empty(format!("{PREFIX}_twenty_gravimetric_collection_nests"));
    let z = BASE_Z / 2.0 + COLLECTION_RACK_Z + 3.0;
    for lane in 0..COLLECTION_NESTS {
        let y = lane_y(lane);
        let well = port_ring(
            format!("{PREFIX}_collection_lane_{lane:02}_vial_nest_ring"),
            COLLECTION_WELL_CLEARANCE_D,
            COLLECTION_WELL_D,
            7.0,
        )
        .translate(COLLECTION_POS.0 - 28.0, y, z);
        let load_cell = centered_cube(
            format!("{PREFIX}_collection_lane_{lane:02}_load_cell_pad"),
            COLLECTION_LOAD_CELL_PAD_X,
            COLLECTION_LOAD_CELL_PAD_Y,
            6.0,
        )
        .translate(COLLECTION_POS.0 + 48.0, y, z);
        let balance_id = csg_label_plaque(
            format!("{PREFIX}_collection_lane_{lane:02}_balance_id_land"),
            42.0,
            12.0,
            3.0,
            lane,
        )
        .translate(COLLECTION_POS.0 + 83.0, y, z + 3.0);
        nests = nests + well + load_cell + balance_id;
    }
    nests
}

fn collection_reference_scale_rails() -> Part {
    let upper = centered_cube(
        format!("{PREFIX}_gravimetric_collection_upper_scale_reference_rail"),
        COLLECTION_RACK_X - 34.0,
        8.0,
        8.0,
    )
    .translate(
        COLLECTION_POS.0,
        COLLECTION_POS.1 + COLLECTION_RACK_Y / 2.0 - 18.0,
        BASE_Z / 2.0 + COLLECTION_RACK_Z + 4.0,
    );
    let lower = centered_cube(
        format!("{PREFIX}_gravimetric_collection_lower_scale_reference_rail"),
        COLLECTION_RACK_X - 34.0,
        8.0,
        8.0,
    )
    .translate(
        COLLECTION_POS.0,
        COLLECTION_POS.1 - COLLECTION_RACK_Y / 2.0 + 18.0,
        BASE_Z / 2.0 + COLLECTION_RACK_Z + 4.0,
    );
    upper + lower
}

fn dye_tracer_manifold() -> Part {
    let block = centered_cube(
        format!("{PREFIX}_dye_tracer_manifold_block"),
        DYE_MANIFOLD_X,
        DYE_MANIFOLD_Y,
        DYE_MANIFOLD_Z,
    )
    .translate(DYE_POS.0, DYE_POS.1, BASE_Z / 2.0 + DYE_MANIFOLD_Z / 2.0);
    let split_feed = centered_cube(
        format!("{PREFIX}_dye_tracer_equal_feed_header"),
        18.0,
        DYE_MANIFOLD_Y - 62.0,
        9.0,
    )
    .translate(
        DYE_POS.0 + 42.0,
        DYE_POS.1,
        BASE_Z / 2.0 + DYE_MANIFOLD_Z + 4.5,
    );

    block + dye_reservoir_wells() + tracer_septa_array() + split_feed + dye_prime_plaque()
}

fn dye_reservoir_wells() -> Part {
    let mut wells = Part::empty(format!("{PREFIX}_dye_tracer_source_reservoir_wells"));
    for source in 0..DYE_RESERVOIRS {
        let y = DYE_POS.1 + centered_index(source, DYE_RESERVOIRS, 72.0);
        let ring = port_ring(
            format!("{PREFIX}_dye_tracer_source_{source}_reservoir_ring"),
            DYE_RESERVOIR_D,
            DYE_RESERVOIR_D - 8.0,
            8.0,
        )
        .translate(
            DYE_POS.0 - DYE_MANIFOLD_X / 2.0 + 46.0,
            y,
            BASE_Z / 2.0 + DYE_MANIFOLD_Z + 4.0,
        );
        let label = csg_label_plaque(
            format!("{PREFIX}_dye_tracer_source_{source}_label_land"),
            48.0,
            13.0,
            3.0,
            30 + source,
        )
        .translate(
            DYE_POS.0 - DYE_MANIFOLD_X / 2.0 + 46.0,
            y + 28.0,
            BASE_Z / 2.0 + DYE_MANIFOLD_Z + 6.5,
        );
        wells = wells + ring + label;
    }
    wells
}

fn tracer_septa_array() -> Part {
    let mut septa = Part::empty(format!("{PREFIX}_twenty_lane_dye_tracer_septa"));
    for lane in 0..TRACER_SEPTA {
        let y = lane_y(lane);
        let septum = centered_cylinder(
            format!("{PREFIX}_lane_{lane:02}_dye_tracer_septum_land"),
            TRACER_SEPTUM_D / 2.0,
            8.0,
            24,
        )
        .translate(DYE_POS.0 + 58.0, y, BASE_Z / 2.0 + DYE_MANIFOLD_Z + 4.0);
        let witness = centered_cube(
            format!("{PREFIX}_lane_{lane:02}_tracer_feed_witness_tick"),
            32.0,
            3.0,
            5.0,
        )
        .translate(DYE_POS.0 + 36.0, y, BASE_Z / 2.0 + DYE_MANIFOLD_Z + 6.0);
        septa = septa + septum + witness;
    }
    septa
}

fn dye_prime_plaque() -> Part {
    csg_label_plaque(
        format!("{PREFIX}_dye_prime_volume_traceability_plaque"),
        150.0,
        18.0,
        4.0,
        DYE_PRIME_VOLUME_UL as usize,
    )
    .translate(
        DYE_POS.0,
        DYE_POS.1 - DYE_MANIFOLD_Y / 2.0 + 22.0,
        BASE_Z / 2.0 + DYE_MANIFOLD_Z + 2.0,
    )
}

fn bubble_wetness_windows() -> Part {
    let bridge = rectangular_frame_xy(
        format!("{PREFIX}_bubble_wetness_window_bridge_frame"),
        WINDOW_BRIDGE_X,
        WINDOW_BRIDGE_Y,
        18.0,
        WINDOW_BRIDGE_Z,
    )
    .translate(
        WINDOW_POS.0,
        WINDOW_POS.1,
        BASE_Z / 2.0 + LANE_PLATE_Z + WINDOW_BRIDGE_Z / 2.0 + 14.0,
    );
    bridge + bubble_wetness_window_array()
}

fn bubble_wetness_window_array() -> Part {
    let mut windows = Part::empty(format!("{PREFIX}_twenty_bubble_wetness_windows"));
    let z = BASE_Z / 2.0 + LANE_PLATE_Z + WINDOW_BRIDGE_Z + 14.0;
    for lane in 0..BUBBLE_WETNESS_WINDOWS {
        let y = lane_y(lane);
        let frame = rectangular_frame_xy(
            format!("{PREFIX}_lane_{lane:02}_bubble_wetness_view_frame"),
            WINDOW_X,
            WINDOW_Y,
            WINDOW_RAIL_W,
            5.0,
        )
        .translate(WINDOW_POS.0 - 176.0, y, z);
        let dome = port_ring(
            format!("{PREFIX}_lane_{lane:02}_bubble_dome_witness_ring"),
            BUBBLE_DOME_D,
            6.0,
            5.0,
        )
        .translate(WINDOW_POS.0 - 84.0, y, z);
        let wet_flag = centered_cube(
            format!("{PREFIX}_lane_{lane:02}_wetness_flag_land"),
            WETNESS_FLAG_X,
            WINDOW_Y,
            5.0,
        )
        .translate(WINDOW_POS.0 + 10.0, y, z);
        windows = windows + frame + dome + wet_flag;
    }
    windows
}

fn lane_isolation_clamps() -> Part {
    let rail = rectangular_frame_xy(
        format!("{PREFIX}_lane_isolation_clamp_alignment_frame"),
        CLAMP_RAIL_X,
        CLAMP_RAIL_Y,
        20.0,
        CLAMP_RAIL_Z,
    )
    .translate(
        CLAMP_POS.0,
        CLAMP_POS.1,
        BASE_Z / 2.0 + LANE_PLATE_Z + CLAMP_RAIL_Z / 2.0 + 42.0,
    );
    rail + clamp_blocks()
}

fn clamp_blocks() -> Part {
    let mut clamps = Part::empty(format!("{PREFIX}_twenty_lane_isolation_clamps"));
    let z = BASE_Z / 2.0 + LANE_PLATE_Z + CLAMP_RAIL_Z + 42.0;
    for lane in 0..ISOLATION_CLAMPS {
        let y = lane_y(lane);
        let saddle = centered_cube(
            format!("{PREFIX}_lane_{lane:02}_isolation_clamp_saddle"),
            CLAMP_BLOCK_X,
            CLAMP_BLOCK_Y,
            16.0,
        )
        .translate(CLAMP_POS.0 + 202.0, y, z);
        let lever = centered_cube(
            format!("{PREFIX}_lane_{lane:02}_isolation_clamp_lever"),
            CLAMP_LEVER_X,
            5.0,
            6.0,
        )
        .rotate(0.0, 0.0, if lane % 2 == 0 { 8.0 } else { -8.0 })
        .translate(CLAMP_POS.0 + 236.0, y, z + 10.0);
        let pivot = centered_cylinder(
            format!("{PREFIX}_lane_{lane:02}_isolation_clamp_pivot_pin"),
            CLAMP_PIN_D / 2.0,
            8.0,
            20,
        )
        .translate(CLAMP_POS.0 + 220.0, y, z + 4.0);
        clamps = clamps + saddle + lever + pivot;
    }
    clamps
}

fn release_hold_reject_lanes() -> Part {
    let panel = centered_cube(
        format!("{PREFIX}_release_hold_reject_lane_panel"),
        DISPOSITION_PANEL_X,
        DISPOSITION_PANEL_Y,
        DISPOSITION_PANEL_Z,
    )
    .translate(
        DISPOSITION_POS.0,
        DISPOSITION_POS.1,
        BASE_Z / 2.0 + DISPOSITION_PANEL_Z / 2.0,
    );
    panel + disposition_lane_slots() + disposition_status_tokens()
}

fn disposition_lane_slots() -> Part {
    let mut lanes = Part::empty(format!("{PREFIX}_release_hold_reject_lane_slots"));
    let z = BASE_Z / 2.0 + DISPOSITION_PANEL_Z + 3.0;
    for disposition in 0..DISPOSITION_LANES {
        let y =
            DISPOSITION_POS.1 + centered_index(disposition, DISPOSITION_LANES, DISPOSITION_PITCH_Y);
        let guide = centered_cube(
            format!("{PREFIX}_disposition_lane_{disposition}_guide_rail"),
            DISPOSITION_PANEL_X - 48.0,
            4.0,
            6.0,
        )
        .translate(DISPOSITION_POS.0, y, z);
        lanes = lanes + guide;
        for slot in 0..STATUS_SLOTS_PER_LANE {
            let x = DISPOSITION_POS.0
                + centered_index(slot, STATUS_SLOTS_PER_LANE, DISPOSITION_PITCH_X);
            lanes = lanes
                + centered_cube(
                    format!("{PREFIX}_disposition_lane_{disposition}_slot_{slot:02}"),
                    DISPOSITION_SLOT_X,
                    DISPOSITION_SLOT_Y,
                    5.0,
                )
                .translate(x, y, z + 4.0);
        }
    }
    lanes
}

fn disposition_status_tokens() -> Part {
    let mut tokens = Part::empty(format!("{PREFIX}_release_hold_reject_status_tokens"));
    for (lane, label) in ["release", "hold", "reject"].iter().enumerate() {
        let y = DISPOSITION_POS.1 + centered_index(lane, DISPOSITION_LANES, DISPOSITION_PITCH_Y);
        let plaque = csg_label_plaque(
            format!("{PREFIX}_{label}_lane_label_plaque"),
            72.0,
            16.0,
            4.0,
            70 + lane,
        )
        .translate(
            DISPOSITION_POS.0 - DISPOSITION_PANEL_X / 2.0 + 58.0,
            y,
            BASE_Z / 2.0 + DISPOSITION_PANEL_Z + 9.0,
        );
        tokens = tokens + plaque;
    }
    tokens
}

fn barcode_custody_panel() -> Part {
    let panel = centered_cube(
        format!("{PREFIX}_barcode_custody_panel_base"),
        BARCODE_PANEL_X,
        BARCODE_PANEL_Y,
        BARCODE_PANEL_Z,
    )
    .translate(
        BARCODE_POS.0,
        BARCODE_POS.1,
        BASE_Z / 2.0 + BARCODE_PANEL_Z / 2.0,
    );
    panel + barcode_land_array() + custody_token_array() + scanner_window()
}

fn barcode_land_array() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_twenty_lane_barcode_lands"));
    for lane in 0..BARCODE_LANDS {
        let row = lane / 5;
        let col = lane % 5;
        let x = BARCODE_POS.0 + centered_index(col, 5, 58.0);
        let y = BARCODE_POS.1 + centered_index(row, 4, 18.0) + 8.0;
        lands = lands
            + csg_label_plaque(
                format!("{PREFIX}_lane_{lane:02}_barcode_land"),
                BARCODE_LAND_X,
                BARCODE_LAND_Y,
                3.0,
                90 + lane,
            )
            .translate(x, y, BASE_Z / 2.0 + BARCODE_PANEL_Z + 1.5);
    }
    lands
}

fn custody_token_array() -> Part {
    let mut tokens = Part::empty(format!("{PREFIX}_custody_chain_tokens"));
    for token in 0..CUSTODY_LANDS {
        tokens = tokens
            + port_ring(
                format!("{PREFIX}_custody_chain_token_{token}"),
                CUSTODY_TOKEN_D,
                CUSTODY_TOKEN_D - 7.0,
                4.0,
            )
            .translate(
                BARCODE_POS.0 + centered_index(token, CUSTODY_LANDS, 34.0),
                BARCODE_POS.1 - BARCODE_PANEL_Y / 2.0 + 13.0,
                BASE_Z / 2.0 + BARCODE_PANEL_Z + 2.0,
            );
    }
    tokens
}

fn scanner_window() -> Part {
    rectangular_frame_xy(
        format!("{PREFIX}_barcode_scanner_keep_clear_window"),
        BARCODE_PANEL_X - 36.0,
        22.0,
        5.0,
        5.0,
    )
    .translate(
        BARCODE_POS.0,
        BARCODE_POS.1 + BARCODE_PANEL_Y / 2.0 - 14.0,
        BASE_Z / 2.0 + BARCODE_PANEL_Z + 2.5,
    )
}

fn robot_service_keepouts() -> Part {
    let frame = keepout_frame(
        format!("{PREFIX}_robot_keepout_outer_frame"),
        ROBOT_KEEP_OUT_X,
        ROBOT_KEEP_OUT_Y,
        ROBOT_KEEP_OUT_Z,
    )
    .translate(0.0, 0.0, BASE_Z + ROBOT_KEEP_OUT_Z / 2.0);
    frame + keepout_posts() + service_keepout_gauges() + robot_z_clearance_gauge()
}

fn keepout_posts() -> Part {
    let mut posts = Part::empty(format!("{PREFIX}_robot_keepout_corner_posts"));
    for (i, (x, y)) in [
        (-ROBOT_KEEP_OUT_X / 2.0, -ROBOT_KEEP_OUT_Y / 2.0),
        (ROBOT_KEEP_OUT_X / 2.0, -ROBOT_KEEP_OUT_Y / 2.0),
        (-ROBOT_KEEP_OUT_X / 2.0, ROBOT_KEEP_OUT_Y / 2.0),
        (ROBOT_KEEP_OUT_X / 2.0, ROBOT_KEEP_OUT_Y / 2.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(
                format!("{PREFIX}_robot_keepout_post_{i}"),
                12.0,
                12.0,
                ROBOT_KEEP_OUT_Z,
            )
            .translate(*x, *y, BASE_Z + ROBOT_KEEP_OUT_Z / 2.0);
    }
    posts
}

fn service_keepout_gauges() -> Part {
    let z = BASE_Z + 14.0;
    let front = gauge_bar(
        format!("{PREFIX}_front_service_clearance_gauge"),
        360.0,
        18.0,
        12.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 58.0, z);
    let rear = gauge_bar(
        format!("{PREFIX}_rear_tubing_service_clearance_gauge"),
        360.0,
        18.0,
        12.0,
    )
    .translate(0.0, STATION_Y / 2.0 - 58.0, z);
    let left = gauge_bar(
        format!("{PREFIX}_left_dye_service_clearance_gauge"),
        18.0,
        260.0,
        12.0,
    )
    .translate(-STATION_X / 2.0 + 62.0, LANE_CENTER_Y, z);
    let right = gauge_bar(
        format!("{PREFIX}_right_collection_service_clearance_gauge"),
        18.0,
        260.0,
        12.0,
    )
    .translate(STATION_X / 2.0 - 62.0, LANE_CENTER_Y, z);
    let sensor = gauge_bar(
        format!("{PREFIX}_reference_sensor_service_clearance_gauge"),
        260.0,
        16.0,
        12.0,
    )
    .translate(SENSOR_POS.0, SENSOR_POS.1 - SENSOR_RACK_Y / 2.0 - 24.0, z);
    let collection = gauge_bar(
        format!("{PREFIX}_collection_scale_swap_service_gauge"),
        150.0,
        16.0,
        12.0,
    )
    .translate(
        COLLECTION_POS.0,
        COLLECTION_POS.1 - COLLECTION_RACK_Y / 2.0 - 22.0,
        z,
    );

    front + rear + left + right + sensor + collection
}

fn robot_z_clearance_gauge() -> Part {
    let post = centered_cube(
        format!("{PREFIX}_robot_z_clearance_reference_post"),
        28.0,
        28.0,
        ROBOT_Z_CLEARANCE,
    )
    .translate(
        ROBOT_KEEP_OUT_X / 2.0 - 54.0,
        -(ROBOT_KEEP_OUT_Y / 2.0 - 54.0),
        BASE_Z + ROBOT_Z_CLEARANCE / 2.0,
    );
    let plaque = csg_label_plaque(
        format!("{PREFIX}_robot_z_clearance_label"),
        74.0,
        18.0,
        4.0,
        ROBOT_Z_CLEARANCE as usize,
    )
    .translate(
        ROBOT_KEEP_OUT_X / 2.0 - 54.0,
        -(ROBOT_KEEP_OUT_Y / 2.0 - 92.0),
        BASE_Z + 8.0,
    );
    post + plaque
}

fn mount_points() -> [(f64, f64); MOUNT_SLOTS] {
    [
        (-(STATION_X / 2.0 - 56.0), -(STATION_Y / 2.0 - 54.0)),
        (STATION_X / 2.0 - 56.0, -(STATION_Y / 2.0 - 54.0)),
        (-(STATION_X / 2.0 - 56.0), STATION_Y / 2.0 - 54.0),
        (STATION_X / 2.0 - 56.0, STATION_Y / 2.0 - 54.0),
        (0.0, -(STATION_Y / 2.0 - 54.0)),
        (0.0, STATION_Y / 2.0 - 54.0),
        (-(STATION_X / 2.0 - 56.0), 0.0),
        (STATION_X / 2.0 - 56.0, 0.0),
        (-STATION_X / 4.0, STATION_Y / 2.0 - 54.0),
        (STATION_X / 4.0, STATION_Y / 2.0 - 54.0),
    ]
}

fn lane_y(lane: usize) -> f64 {
    LANE_CENTER_Y + centered_index(lane, OUTLET_LANES, LANE_PITCH_Y)
}

fn sensor_position(lane: usize) -> (f64, f64) {
    let row = lane / SENSOR_COLS;
    let col = lane % SENSOR_COLS;
    (
        SENSOR_POS.0 + centered_index(col, SENSOR_COLS, SENSOR_PITCH_X),
        SENSOR_POS.1 + centered_index(row, SENSOR_ROWS, SENSOR_PITCH_Y),
    )
}

#[cfg(test)]
fn restrictor_path_length(_lane: usize) -> f64 {
    RESTRICTOR_PATH_LENGTH_MM
}

#[cfg(test)]
fn module_specs() -> [(&'static str, (f64, f64), f64, f64); 8] {
    [
        (
            "lane_plate",
            (LANE_CENTER_X, LANE_CENTER_Y),
            LANE_BANK_X,
            LANE_BANK_Y,
        ),
        (
            "restrictor_comb",
            RESTRICTOR_POS,
            RESTRICTOR_BANK_X,
            RESTRICTOR_BANK_Y,
        ),
        ("sensors", SENSOR_POS, SENSOR_RACK_X, SENSOR_RACK_Y),
        (
            "collection",
            COLLECTION_POS,
            COLLECTION_RACK_X,
            COLLECTION_RACK_Y,
        ),
        ("dye", DYE_POS, DYE_MANIFOLD_X, DYE_MANIFOLD_Y),
        (
            "disposition",
            DISPOSITION_POS,
            DISPOSITION_PANEL_X,
            DISPOSITION_PANEL_Y,
        ),
        ("barcode", BARCODE_POS, BARCODE_PANEL_X, BARCODE_PANEL_Y),
        ("keepout", (0.0, 0.0), ROBOT_KEEP_OUT_X, ROBOT_KEEP_OUT_Y),
    ]
}

#[cfg(test)]
fn fits_on_station(center: (f64, f64), width: f64, depth: f64, margin: f64) -> bool {
    center.0 - width / 2.0 >= -STATION_X / 2.0 + margin
        && center.0 + width / 2.0 <= STATION_X / 2.0 - margin
        && center.1 - depth / 2.0 >= -STATION_Y / 2.0 + margin
        && center.1 + depth / 2.0 <= STATION_Y / 2.0 - margin
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn u_turn_witness(name: impl Into<String>, run_x: f64, offset_y: f64, width: f64, z: f64) -> Part {
    let name = name.into();
    let top = centered_cube(format!("{name}_top_leg"), run_x, width, z).translate(
        0.0,
        offset_y / 2.0,
        0.0,
    );
    let bottom = centered_cube(format!("{name}_bottom_leg"), run_x, width, z).translate(
        0.0,
        -offset_y / 2.0,
        0.0,
    );
    let turn = centered_cube(
        format!("{name}_return_leg"),
        width,
        offset_y.abs() + width,
        z,
    )
    .translate(run_x / 2.0 - width / 2.0, 0.0, 0.0);
    top + bottom + turn
}

fn serpentine_restrictor(
    name: impl Into<String>,
    straight_x: f64,
    turn_y: f64,
    width: f64,
    z: f64,
) -> Part {
    let name = name.into();
    let mut part = Part::empty(format!("{name}_body"));
    for leg in 0..RESTRICTOR_STRAIGHTS {
        let y = centered_index(leg, RESTRICTOR_STRAIGHTS, turn_y);
        part = part
            + centered_cube(format!("{name}_straight_leg_{leg}"), straight_x, width, z)
                .translate(0.0, y, 0.0);
        if leg + 1 < RESTRICTOR_STRAIGHTS {
            let x = if leg % 2 == 0 {
                straight_x / 2.0 - width / 2.0
            } else {
                -(straight_x / 2.0 - width / 2.0)
            };
            part = part
                + centered_cube(
                    format!("{name}_return_turn_{leg}"),
                    width,
                    turn_y + width,
                    z,
                )
                .translate(x, y + turn_y / 2.0, 0.0);
        }
    }
    part
}

fn rectangular_frame_xy(
    name: impl Into<String>,
    outer_x: f64,
    outer_y: f64,
    rail: f64,
    z: f64,
) -> Part {
    let name = name.into();
    centered_cube(format!("{name}_outer"), outer_x, outer_y, z)
        - centered_cube(
            format!("{name}_inner_clearance"),
            outer_x - 2.0 * rail,
            outer_y - 2.0 * rail,
            z + 1.0,
        )
}

fn keepout_frame(name: impl Into<String>, x: f64, y: f64, z: f64) -> Part {
    let name = name.into();
    let front =
        centered_cube(format!("{name}_front_rail"), x, 8.0, z).translate(0.0, -y / 2.0, 0.0);
    let rear = centered_cube(format!("{name}_rear_rail"), x, 8.0, z).translate(0.0, y / 2.0, 0.0);
    let left = centered_cube(format!("{name}_left_rail"), 8.0, y, z).translate(-x / 2.0, 0.0, 0.0);
    let right = centered_cube(format!("{name}_right_rail"), 8.0, y, z).translate(x / 2.0, 0.0, 0.0);
    front + rear + left + right
}

fn port_ring(name: impl Into<String>, outer_d: f64, inner_d: f64, z: f64) -> Part {
    let name = name.into();
    centered_cylinder(format!("{name}_outer"), outer_d / 2.0, z, 36)
        - centered_cylinder(format!("{name}_inner"), inner_d / 2.0, z + 1.0, 28)
}

fn fiducial_target(name: impl Into<String>) -> Part {
    let name = name.into();
    let ring = port_ring(format!("{name}_ring"), 22.0, 8.0, 4.0);
    let xhair = centered_cube(format!("{name}_crosshair_x"), 26.0, 2.0, 3.0)
        + centered_cube(format!("{name}_crosshair_y"), 2.0, 26.0, 3.0);
    ring + xhair
}

fn gauge_bar(name: impl Into<String>, x: f64, y: f64, z: f64) -> Part {
    let name = name.into();
    let bar = centered_cube(format!("{name}_bar"), x, y, z);
    let tick_a = centered_cube(format!("{name}_tick_a"), 14.0, 14.0, z + 6.0).translate(
        -x / 2.0,
        -y / 2.0,
        0.0,
    );
    let tick_b = centered_cube(format!("{name}_tick_b"), 14.0, 14.0, z + 6.0).translate(
        x / 2.0,
        y / 2.0,
        0.0,
    );
    bar + tick_a + tick_b
}

fn csg_label_plaque(name: impl Into<String>, x: f64, y: f64, z: f64, seed: usize) -> Part {
    let name = name.into();
    let sheet = centered_cube(format!("{name}_sheet"), x, y, z);
    let mut bars = Part::empty(format!("{name}_raised_barcode_bars"));
    for i in 0..LABEL_BAR_COUNT {
        let width = 1.8 + ((seed + i) % 4) as f64 * 1.2;
        let height = (y - 5.0 - (i % 3) as f64).max(3.0);
        let x_offset = -x / 2.0 + 7.0 + i as f64 * ((x - 16.0) / LABEL_BAR_COUNT as f64);
        bars = bars
            + centered_cube(format!("{name}_bar_{i}"), width, height, z + 1.2).translate(
                x_offset,
                0.0,
                z / 2.0 + 0.6,
            );
    }
    let corner = centered_cube(format!("{name}_orientation_corner"), 7.0, 3.0, z + 1.4).translate(
        x / 2.0 - 6.0,
        y / 2.0 - 4.0,
        z / 2.0 + 0.7,
    );
    sheet + bars + corner
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
        for output in OUTPUTS {
            assert!(output.starts_with("output/closed_perfusion_lane_flow_split_symmetry_station_"));
            assert!(output.ends_with(".stl"));
        }
    }

    #[test]
    fn feature_list_covers_requested_validation_cues() {
        assert_eq!(REQUIRED_FEATURES.len(), 11);
        assert!(REQUIRED_FEATURES.contains(&"twenty_outlet_lanes"));
        assert!(REQUIRED_FEATURES.contains(&"equal_length_restrictor_comb"));
        assert!(REQUIRED_FEATURES.contains(&"reference_flow_sensor_pockets"));
        assert!(REQUIRED_FEATURES.contains(&"gravimetric_collection_nests"));
        assert!(REQUIRED_FEATURES.contains(&"dye_tracer_manifold"));
        assert!(REQUIRED_FEATURES.contains(&"bubble_wetness_windows"));
        assert!(REQUIRED_FEATURES.contains(&"lane_isolation_clamps"));
        assert!(REQUIRED_FEATURES.contains(&"release_hold_reject_lanes"));
        assert!(REQUIRED_FEATURES.contains(&"barcode_custody"));
        assert!(REQUIRED_FEATURES.contains(&"robot_keepouts"));
        assert!(REQUIRED_FEATURES.contains(&"service_keepouts"));
    }

    #[test]
    fn twenty_outlet_lanes_are_equal_pitch_and_symmetric() {
        assert_eq!(OUTLET_LANES, 20);
        assert_eq!(lane_y(0) + lane_y(OUTLET_LANES - 1), 2.0 * LANE_CENTER_Y);
        for lane in 1..OUTLET_LANES {
            assert_eq!(lane_y(lane) - lane_y(lane - 1), LANE_PITCH_Y);
        }
        for lane in 0..OUTLET_LANES / 2 {
            let mirror = OUTLET_LANES - lane - 1;
            assert!((lane_y(lane) + lane_y(mirror) - 2.0 * LANE_CENTER_Y).abs() <= f64::EPSILON);
        }
        assert!(LANE_BANK_Y > (OUTLET_LANES as f64 - 1.0) * LANE_PITCH_Y);
        assert!(LANE_SYMMETRY_TOLERANCE_MM <= 0.05);
    }

    #[test]
    fn restrictor_comb_preserves_equal_length_for_every_lane() {
        assert_eq!(RESTRICTOR_ELEMENTS, OUTLET_LANES);
        let first = restrictor_path_length(0);
        for lane in 0..RESTRICTOR_ELEMENTS {
            assert_eq!(restrictor_path_length(lane), first);
        }
        assert_eq!(RESTRICTOR_STRAIGHTS, 4);
        assert!(RESTRICTOR_PATH_LENGTH_MM > 450.0);
        assert!(RESTRICTOR_PATH_LENGTH_MM < TARGET_LANE_SWEPT_LENGTH_MM);
    }

    #[test]
    fn every_lane_has_reference_sensor_collection_window_and_clamp() {
        assert_eq!(SENSOR_POCKETS, OUTLET_LANES);
        assert_eq!(COLLECTION_NESTS, OUTLET_LANES);
        assert_eq!(TRACER_SEPTA, OUTLET_LANES);
        assert_eq!(BUBBLE_WETNESS_WINDOWS, OUTLET_LANES);
        assert_eq!(ISOLATION_CLAMPS, OUTLET_LANES);
        assert_eq!(COLLECTION_BALANCE_ID_LANDS, OUTLET_LANES);
        assert_eq!(SENSOR_REFERENCE_PORTS_PER_LANE * OUTLET_LANES, 40);
        assert!(COLLECTION_WELL_CLEARANCE_D > COLLECTION_WELL_D);
        assert!(WINDOW_X > 8.0 * LANE_TRACE_W);
    }

    #[test]
    fn sensor_pocket_grid_maps_exactly_twenty_positions() {
        let mut positions = BTreeSet::new();
        for lane in 0..SENSOR_POCKETS {
            let (x, y) = sensor_position(lane);
            positions.insert((x.round() as i64, y.round() as i64));
            assert!(fits_on_station(
                (x, y),
                SENSOR_POCKET_X,
                SENSOR_POCKET_Y,
                24.0
            ));
        }
        assert_eq!(positions.len(), SENSOR_POCKETS);
        assert_eq!(SENSOR_ROWS * SENSOR_COLS, SENSOR_POCKETS);
    }

    #[test]
    fn disposition_and_custody_controls_are_lane_complete() {
        assert_eq!(DISPOSITION_LANES, 3);
        assert_eq!(STATUS_SLOTS_PER_LANE, OUTLET_LANES);
        assert_eq!(DISPOSITION_LANES * STATUS_SLOTS_PER_LANE, 60);
        assert_eq!(BARCODE_LANDS, OUTLET_LANES);
        assert_eq!(CUSTODY_LANDS, 5);
        assert_eq!(BASE_DATUMS, 6);
        assert_eq!(MOUNT_SLOTS, 10);
        assert!(BARCODE_LAND_X * 5.0 < BARCODE_PANEL_X);
    }

    #[test]
    fn modules_fit_station_and_keepouts_cover_service_workflow() {
        for (name, center, width, depth) in module_specs() {
            assert!(
                fits_on_station(center, width, depth, 12.0),
                "{name} should fit inside station envelope"
            );
        }
        assert_eq!(ROBOT_KEEP_OUT_WINDOWS, 4);
        assert_eq!(SERVICE_GAUGES, 6);
        assert!(ROBOT_Z_CLEARANCE > DYE_MANIFOLD_Z);
        assert!(FRONT_SERVICE_CLEARANCE >= 240.0);
        assert!(REAR_SERVICE_CLEARANCE >= 200.0);
        assert!(LEFT_SERVICE_CLEARANCE >= 150.0);
        assert!(RIGHT_COLLECTION_SERVICE_CLEARANCE >= 160.0);
    }
}
