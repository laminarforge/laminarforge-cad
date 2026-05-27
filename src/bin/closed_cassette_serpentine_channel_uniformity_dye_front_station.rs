use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed cassette serpentine-channel uniformity dye-front validation station.
//
// Source-only validation CAD for a sealed tissue-chip cassette surrogate. The
// fixture checks whether parallel serpentine channels seed and perfuse evenly
// by comparing dye-front arrival timing, window coupons, splitter witness
// paths, trapped-bubble windows, and release/hold/reject gates. This is station
// architecture CAD, not a sterile wetted-path release drawing or protocol.

const PREFIX: &str = "closed_cassette_serpentine_channel_uniformity_dye_front_station";

const OUTPUTS: [&str; 12] = [
    "output/closed_cassette_serpentine_channel_uniformity_dye_front_station_closed_leak_tray_base.stl",
    "output/closed_cassette_serpentine_channel_uniformity_dye_front_station_cassette_datum_nest.stl",
    "output/closed_cassette_serpentine_channel_uniformity_dye_front_station_parallel_serpentine_channel_plate.stl",
    "output/closed_cassette_serpentine_channel_uniformity_dye_front_station_dye_front_timing_marker_bridge.stl",
    "output/closed_cassette_serpentine_channel_uniformity_dye_front_station_flow_splitter_witness_paths.stl",
    "output/closed_cassette_serpentine_channel_uniformity_dye_front_station_channel_window_coupon_carrier.stl",
    "output/closed_cassette_serpentine_channel_uniformity_dye_front_station_trapped_bubble_window_array.stl",
    "output/closed_cassette_serpentine_channel_uniformity_dye_front_station_flow_balance_reference_sensor_pockets.stl",
    "output/closed_cassette_serpentine_channel_uniformity_dye_front_station_timed_fraction_capture_rack.stl",
    "output/closed_cassette_serpentine_channel_uniformity_dye_front_station_reject_hold_release_gate_panel.stl",
    "output/closed_cassette_serpentine_channel_uniformity_dye_front_station_barcode_custody_keepout_gauges.stl",
    "output/closed_cassette_serpentine_channel_uniformity_dye_front_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "closed_leak_tray_base",
    "cassette_datum_nest",
    "parallel_serpentine_channel_plate",
    "dye_front_timing_marker_bridge",
    "flow_splitter_witness_paths",
    "channel_window_coupon_carrier",
    "trapped_bubble_window_array",
    "flow_balance_reference_sensor_pockets",
    "timed_fraction_capture_rack",
    "reject_hold_release_gate_panel",
    "barcode_custody_keepout_gauges",
];

const STATION_X: f64 = 1540.0;
const STATION_Y: f64 = 980.0;
const BASE_Z: f64 = 22.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 42.0;
const LEAK_BASIN_X: f64 = STATION_X - 122.0;
const LEAK_BASIN_Y: f64 = STATION_Y - 118.0;
const LEAK_BASIN_DEPTH: f64 = 7.0;
const DRAIN_D: f64 = 18.0;
const MOUNT_SLOT_COUNT: usize = 8;
const LEAK_SENSOR_WELLS: usize = 5;
const DATUM_TARGETS: usize = 6;

const CHANNEL_LANES: usize = 12;
const CHANNEL_ROWS: usize = 3;
const CHANNEL_COLS: usize = 4;
const EDGE_CHANNELS: usize = 10;
const CENTER_CHANNELS: usize = CHANNEL_LANES - EDGE_CHANNELS;
const CHANNEL_PITCH_Y: f64 = 33.0;
const CHANNEL_TRACE_W: f64 = 7.2;
const CHANNEL_TRACE_Z: f64 = 5.4;
const CHANNEL_CLEAR_BORE_D: f64 = 4.8;
const CHANNEL_CENTER: (f64, f64) = (190.0, 102.0);
const CHANNEL_PLATE_X: f64 = 640.0;
const CHANNEL_PLATE_Y: f64 = 432.0;
const CHANNEL_PLATE_Z: f64 = 20.0;
const CHANNEL_SERP_STRAIGHT_X: f64 = 430.0;
const CHANNEL_SERP_LEGS: usize = 5;
const CHANNEL_SERP_TURN_Y: f64 = CHANNEL_PITCH_Y;
const CHANNEL_MODELED_PATH_MM: f64 = CHANNEL_SERP_LEGS as f64 * CHANNEL_SERP_STRAIGHT_X
    + (CHANNEL_SERP_LEGS - 1) as f64 * CHANNEL_SERP_TURN_Y;
const CHANNEL_LENGTH_TOLERANCE_MM: f64 = 0.05;
const SEED_PERFUSION_TARGET_UL_PER_MIN: f64 = 55.0;
const DYE_FRONT_TARGET_S: f64 = 42.0;
const DYE_FRONT_WINDOW_S: f64 = 4.0;

const CASSETTE_CENTER: (f64, f64) = (-405.0, 118.0);
const CASSETTE_MARGIN_X: f64 = 42.0;
const CASSETTE_MARGIN_Y: f64 = 38.0;
const CHIP_GAP_X: f64 = 18.0;
const CHIP_GAP_Y: f64 = 20.0;
const CASSETTE_CHIP_PITCH_X: f64 = REVC_CHIP_LENGTH + CHIP_GAP_X;
const CASSETTE_CHIP_PITCH_Y: f64 = REVC_CHIP_WIDTH + CHIP_GAP_Y;
const CASSETTE_SLOT_ARRAY_X: f64 =
    CHANNEL_COLS as f64 * REVC_CHIP_LENGTH + (CHANNEL_COLS as f64 - 1.0) * CHIP_GAP_X;
const CASSETTE_SLOT_ARRAY_Y: f64 =
    CHANNEL_ROWS as f64 * REVC_CHIP_WIDTH + (CHANNEL_ROWS as f64 - 1.0) * CHIP_GAP_Y;
const CASSETTE_NEST_X: f64 = CASSETTE_SLOT_ARRAY_X + 2.0 * CASSETTE_MARGIN_X;
const CASSETTE_NEST_Y: f64 = CASSETTE_SLOT_ARRAY_Y + 2.0 * CASSETTE_MARGIN_Y;
const CASSETTE_NEST_Z: f64 = 36.0;
const CASSETTE_RECESS_DEPTH: f64 = 11.0;
const CASSETTE_SLOT_CLEARANCE: f64 = 7.0;
const CASSETTE_DATUM_PIN_D: f64 = 8.0;
const CASSETTE_CLAMPS: usize = 6;

const TIMING_CENTER: (f64, f64) = (190.0, 377.0);
const TIMING_BRIDGE_X: f64 = 720.0;
const TIMING_BRIDGE_Y: f64 = 108.0;
const TIMING_BRIDGE_Z: f64 = 24.0;
const TIMING_TICKS_PER_LANE: usize = 7;
const TIMING_TICK_PITCH_X: f64 = 64.0;
const TIMING_CAMERA_PODS: usize = 3;
const TIMING_FLAG_X: f64 = 32.0;
const TIMING_FLAG_Y: f64 = 14.0;

const SPLITTER_CENTER: (f64, f64) = (-150.0, 102.0);
const SPLITTER_BODY_X: f64 = 170.0;
const SPLITTER_BODY_Y: f64 = 432.0;
const SPLITTER_BODY_Z: f64 = 30.0;
const SPLITTER_BRANCH_X: f64 = 128.0;
const SPLITTER_TRACE_W: f64 = 6.4;
const SPLITTER_WITNESS_PATHS: usize = CHANNEL_LANES;
const SPLITTER_EQUALIZATION_LOOPS: usize = 2;

const COUPON_CENTER: (f64, f64) = (575.0, 112.0);
const COUPON_CARRIER_X: f64 = 310.0;
const COUPON_CARRIER_Y: f64 = 420.0;
const COUPON_CARRIER_Z: f64 = 26.0;
const WINDOW_COUPONS: usize = CHANNEL_LANES;
const COUPON_SLOT_X: f64 = 86.0;
const COUPON_SLOT_Y: f64 = 23.0;
const COUPON_SLOT_DEPTH: f64 = 9.0;
const WINDOW_LENS_X: f64 = 104.0;
const WINDOW_LENS_Y: f64 = 16.0;
const WINDOW_GRATICULE_TICKS: usize = 5;

const BUBBLE_CENTER: (f64, f64) = (550.0, -198.0);
const BUBBLE_ARRAY_X: f64 = 390.0;
const BUBBLE_ARRAY_Y: f64 = 168.0;
const BUBBLE_ARRAY_Z: f64 = 28.0;
const BUBBLE_WINDOWS: usize = CHANNEL_LANES;
const BUBBLE_WINDOW_D: f64 = 17.0;
const BUBBLE_DOME_D: f64 = 24.0;
const WETNESS_STRIP_X: f64 = 24.0;
const WETNESS_STRIP_Y: f64 = 62.0;

const SENSOR_CENTER: (f64, f64) = (120.0, -198.0);
const SENSOR_RACK_X: f64 = 420.0;
const SENSOR_RACK_Y: f64 = 168.0;
const SENSOR_RACK_Z: f64 = 28.0;
const REFERENCE_SENSOR_POCKETS: usize = CHANNEL_LANES;
const SENSOR_POCKET_X: f64 = 42.0;
const SENSOR_POCKET_Y: f64 = 32.0;
const SENSOR_PITCH_X: f64 = 62.0;
const SENSOR_PITCH_Y: f64 = 52.0;
const SENSOR_ROWS: usize = 2;
const SENSOR_COLS: usize = 6;
const PRESSURE_TAPS_PER_LANE: usize = 2;

const FRACTION_CENTER: (f64, f64) = (-265.0, -188.0);
const FRACTION_RACK_X: f64 = 360.0;
const FRACTION_RACK_Y: f64 = 190.0;
const FRACTION_RACK_Z: f64 = 34.0;
const FRACTION_TIMEPOINTS: usize = 4;
const FRACTION_POCKETS: usize = CHANNEL_LANES * FRACTION_TIMEPOINTS;
const FRACTION_ROWS: usize = 4;
const FRACTION_COLS: usize = 12;
const FRACTION_PITCH_X: f64 = 26.0;
const FRACTION_PITCH_Y: f64 = 39.0;
const FRACTION_VIAL_D: f64 = 13.0;
const FRACTION_CLEARANCE_D: f64 = 15.2;

const GATE_CENTER: (f64, f64) = (225.0, -388.0);
const GATE_PANEL_X: f64 = 470.0;
const GATE_PANEL_Y: f64 = 138.0;
const GATE_PANEL_Z: f64 = 26.0;
const DISPOSITION_LANES: usize = 3;
const GATE_TOKENS_PER_LANE: usize = CHANNEL_LANES;
const GATE_TOKEN_X: f64 = 26.0;
const GATE_TOKEN_Y: f64 = 16.0;
const GATE_LANE_PITCH_Y: f64 = 34.0;
const GATE_TOKEN_PITCH_X: f64 = 33.0;

const CUSTODY_CENTER: (f64, f64) = (-440.0, -390.0);
const CUSTODY_PANEL_X: f64 = 420.0;
const CUSTODY_PANEL_Y: f64 = 140.0;
const CUSTODY_PANEL_Z: f64 = 18.0;
const BARCODE_LANDS: usize = CHANNEL_LANES;
const CUSTODY_LANDS: usize = 6;
const BARCODE_LAND_X: f64 = 58.0;
const BARCODE_LAND_Y: f64 = 14.0;
const LABEL_BAR_COUNT: usize = 8;

const ROBOT_KEEP_OUT_X: f64 = 1460.0;
const ROBOT_KEEP_OUT_Y: f64 = 900.0;
const KEEP_OUT_RAIL_Z: f64 = 6.0;
const FRONT_ROBOT_CLEARANCE: f64 = 330.0;
const REAR_TUBING_CLEARANCE: f64 = 240.0;
const LEFT_CASSETTE_SERVICE_CLEARANCE: f64 = 220.0;
const RIGHT_WINDOW_SERVICE_CLEARANCE: f64 = 190.0;
const CAMERA_Z_CLEARANCE: f64 = 178.0;

#[derive(Clone, Copy)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_on_deck(self, margin: f64) -> bool {
        self.center.0 - self.x / 2.0 >= -STATION_X / 2.0 + margin
            && self.center.0 + self.x / 2.0 <= STATION_X / 2.0 - margin
            && self.center.1 - self.y / 2.0 >= -STATION_Y / 2.0 + margin
            && self.center.1 + self.y / 2.0 <= STATION_Y / 2.0 - margin
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = closed_leak_tray_base();
    export(&base, OUTPUTS[0]);

    let nest = cassette_datum_nest();
    export(&nest, OUTPUTS[1]);

    let channels = parallel_serpentine_channel_plate();
    export(&channels, OUTPUTS[2]);

    let timing = dye_front_timing_marker_bridge();
    export(&timing, OUTPUTS[3]);

    let splitter = flow_splitter_witness_paths();
    export(&splitter, OUTPUTS[4]);

    let coupons = channel_window_coupon_carrier();
    export(&coupons, OUTPUTS[5]);

    let bubbles = trapped_bubble_window_array();
    export(&bubbles, OUTPUTS[6]);

    let sensors = flow_balance_reference_sensor_pockets();
    export(&sensors, OUTPUTS[7]);

    let fractions = timed_fraction_capture_rack();
    export(&fractions, OUTPUTS[8]);

    let gates = reject_hold_release_gate_panel();
    export(&gates, OUTPUTS[9]);

    let custody = barcode_custody_keepout_gauges();
    export(&custody, OUTPUTS[10]);

    let assembly = base
        + nest
        + channels
        + timing
        + splitter
        + coupons
        + bubbles
        + sensors
        + fractions
        + gates
        + custody;
    export(&assembly, OUTPUTS[11]);

    println!(
        "Closed cassette serpentine-channel uniformity dye-front station: {:.0}mm x {:.0}mm leak-tray deck with {} parallel serpentine lanes over {:.0}mm modeled path, {:.2}mm length tolerance, {:.0} uL/min seed/perfusion target, and {:.0}s dye-front target +/- {:.0}s.",
        STATION_X,
        STATION_Y,
        CHANNEL_LANES,
        CHANNEL_MODELED_PATH_MM,
        CHANNEL_LENGTH_TOLERANCE_MM,
        SEED_PERFUSION_TARGET_UL_PER_MIN,
        DYE_FRONT_TARGET_S,
        DYE_FRONT_WINDOW_S
    );
    println!(
        "Validation features: {} splitter witness paths, {} channel-window coupons, {} trapped-bubble windows, {} reference sensor pockets, {} timed fraction pockets, {} reject/hold/release gate tokens, {} barcode/custody lands, and {} required feature groups.",
        SPLITTER_WITNESS_PATHS,
        WINDOW_COUPONS,
        BUBBLE_WINDOWS,
        REFERENCE_SENSOR_POCKETS,
        FRACTION_POCKETS,
        DISPOSITION_LANES * GATE_TOKENS_PER_LANE,
        BARCODE_LANDS + CUSTODY_LANDS,
        REQUIRED_FEATURES.len()
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_design_constraints() {
    assert_eq!(CHANNEL_LANES, CHANNEL_ROWS * CHANNEL_COLS);
    assert_eq!(edge_channel_count(), EDGE_CHANNELS);
    assert_eq!(center_channel_count(), CENTER_CHANNELS);
    assert_eq!(SPLITTER_WITNESS_PATHS, CHANNEL_LANES);
    assert_eq!(WINDOW_COUPONS, CHANNEL_LANES);
    assert_eq!(BUBBLE_WINDOWS, CHANNEL_LANES);
    assert_eq!(REFERENCE_SENSOR_POCKETS, CHANNEL_LANES);
    assert_eq!(SENSOR_ROWS * SENSOR_COLS, REFERENCE_SENSOR_POCKETS);
    assert_eq!(FRACTION_POCKETS, CHANNEL_LANES * FRACTION_TIMEPOINTS);
    assert_eq!(FRACTION_ROWS * FRACTION_COLS, FRACTION_POCKETS);
    assert_eq!(DISPOSITION_LANES, 3);
    assert_eq!(GATE_TOKENS_PER_LANE, CHANNEL_LANES);
    assert_eq!(DATUM_TARGETS, 6);
    assert_eq!(MOUNT_SLOT_COUNT, 8);
    assert!(CHANNEL_LENGTH_TOLERANCE_MM <= 0.05);
    assert!(CHANNEL_MODELED_PATH_MM > 2_200.0);
    assert!(CHANNEL_CLEAR_BORE_D < CHANNEL_TRACE_W);
    assert!(FRACTION_CLEARANCE_D > FRACTION_VIAL_D);
    assert!(CAMERA_Z_CLEARANCE > REVC_TOTAL_HEIGHT + CASSETTE_NEST_Z);
    assert!(FRONT_ROBOT_CLEARANCE >= 300.0);
    assert!(REAR_TUBING_CLEARANCE >= 220.0);
    assert!(LEFT_CASSETTE_SERVICE_CLEARANCE >= 200.0);
    assert!(RIGHT_WINDOW_SERVICE_CLEARANCE >= 180.0);

    for rect in module_rects() {
        assert!(
            rect.fits_on_deck(24.0),
            "{} exceeds station footprint",
            rect.name
        );
    }
}

fn module_rects() -> [Rect; 9] {
    [
        Rect {
            name: "cassette_datum_nest",
            center: CASSETTE_CENTER,
            x: CASSETTE_NEST_X,
            y: CASSETTE_NEST_Y,
        },
        Rect {
            name: "parallel_serpentine_channel_plate",
            center: CHANNEL_CENTER,
            x: CHANNEL_PLATE_X,
            y: CHANNEL_PLATE_Y,
        },
        Rect {
            name: "dye_front_timing_marker_bridge",
            center: TIMING_CENTER,
            x: TIMING_BRIDGE_X,
            y: TIMING_BRIDGE_Y,
        },
        Rect {
            name: "flow_splitter_witness_paths",
            center: SPLITTER_CENTER,
            x: SPLITTER_BODY_X,
            y: SPLITTER_BODY_Y,
        },
        Rect {
            name: "channel_window_coupon_carrier",
            center: COUPON_CENTER,
            x: COUPON_CARRIER_X,
            y: COUPON_CARRIER_Y,
        },
        Rect {
            name: "trapped_bubble_window_array",
            center: BUBBLE_CENTER,
            x: BUBBLE_ARRAY_X,
            y: BUBBLE_ARRAY_Y,
        },
        Rect {
            name: "flow_balance_reference_sensor_pockets",
            center: SENSOR_CENTER,
            x: SENSOR_RACK_X,
            y: SENSOR_RACK_Y,
        },
        Rect {
            name: "timed_fraction_capture_rack",
            center: FRACTION_CENTER,
            x: FRACTION_RACK_X,
            y: FRACTION_RACK_Y,
        },
        Rect {
            name: "reject_hold_release_gate_panel",
            center: GATE_CENTER,
            x: GATE_PANEL_X,
            y: GATE_PANEL_Y,
        },
    ]
}

fn closed_leak_tray_base() -> Part {
    let deck = centered_cube(
        format!("{PREFIX}_closed_leak_tray_deck"),
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let basin = centered_cube(
        format!("{PREFIX}_closed_leak_tray_recessed_basin"),
        LEAK_BASIN_X,
        LEAK_BASIN_Y,
        LEAK_BASIN_DEPTH + 1.0,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0 - LEAK_BASIN_DEPTH / 2.0 + 0.4);
    let drain = centered_cylinder(
        format!("{PREFIX}_front_right_low_point_drain"),
        DRAIN_D / 2.0,
        BASE_Z + 6.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 84.0, -STATION_Y / 2.0 + 10.0, 0.0);

    deck - basin - drain
        + perimeter_rims()
        + deck_mount_slots()
        + leak_sensor_wells()
        + datum_targets()
}

fn perimeter_rims() -> Part {
    let left = centered_cube(
        format!("{PREFIX}_left_closed_leak_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(
        -STATION_X / 2.0 + RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(
        format!("{PREFIX}_right_window_service_leak_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let rear = centered_cube(
        format!("{PREFIX}_rear_tubing_high_leak_rim"),
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let front = centered_cube(
        format!("{PREFIX}_front_low_load_access_lip"),
        STATION_X - 140.0,
        12.0,
        24.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 18.0, BASE_Z / 2.0 + 12.0);
    left + right + rear + front
}

fn deck_mount_slots() -> Part {
    let mut slots = Part::empty(format!("{PREFIX}_deck_mount_slots"));
    for (i, (x, y)) in mount_points().iter().enumerate() {
        let boss = centered_cube(format!("{PREFIX}_deck_mount_boss_{i}"), 54.0, 26.0, 6.0)
            .translate(*x, *y, BASE_Z / 2.0 + 3.0);
        let bore = centered_cylinder(
            format!("{PREFIX}_deck_mount_m6_bore_{i}"),
            3.4,
            BASE_Z + 8.0,
            24,
        )
        .translate(*x, *y, 0.0);
        let slot = centered_cube(
            format!("{PREFIX}_deck_mount_slot_relief_{i}"),
            25.0,
            7.2,
            BASE_Z + 8.0,
        )
        .translate(*x, *y, 0.0);
        slots = slots + boss - bore - slot;
    }
    slots
}

fn leak_sensor_wells() -> Part {
    let mut wells = Part::empty(format!("{PREFIX}_leak_sensor_wells"));
    for i in 0..LEAK_SENSOR_WELLS {
        let x = centered_index(i, LEAK_SENSOR_WELLS, 178.0);
        let boss = centered_cylinder(format!("{PREFIX}_leak_sensor_boss_{i}"), 14.0, 6.0, 32)
            .translate(x, -STATION_Y / 2.0 + 58.0, BASE_Z / 2.0 + 3.0);
        let pocket = centered_cylinder(format!("{PREFIX}_leak_sensor_recess_{i}"), 7.0, 7.4, 28)
            .translate(x, -STATION_Y / 2.0 + 58.0, BASE_Z / 2.0 + 3.4);
        wells = wells + (boss - pocket);
    }
    wells
}

fn datum_targets() -> Part {
    let mut targets = Part::empty(format!("{PREFIX}_datum_targets"));
    for (i, (x, y)) in [
        (-690.0, 420.0),
        (-360.0, 420.0),
        (0.0, 420.0),
        (360.0, 420.0),
        (690.0, 420.0),
        (0.0, -430.0),
    ]
    .iter()
    .enumerate()
    {
        targets = targets
            + fiducial_target(format!("{PREFIX}_robot_datum_target_{i}")).translate(
                *x,
                *y,
                BASE_Z / 2.0 + 2.0,
            );
    }
    targets
}

fn cassette_datum_nest() -> Part {
    let base = centered_cube(
        format!("{PREFIX}_cassette_datum_nest_body"),
        CASSETTE_NEST_X,
        CASSETTE_NEST_Y,
        CASSETTE_NEST_Z,
    )
    .translate(
        CASSETTE_CENTER.0,
        CASSETTE_CENTER.1,
        insert_z(CASSETTE_NEST_Z),
    );
    let recess = centered_cube(
        format!("{PREFIX}_cassette_datum_nest_recess"),
        CASSETTE_SLOT_ARRAY_X + 24.0,
        CASSETTE_SLOT_ARRAY_Y + 24.0,
        CASSETTE_RECESS_DEPTH + 1.0,
    )
    .translate(
        CASSETTE_CENTER.0,
        CASSETTE_CENTER.1,
        BASE_Z / 2.0 + CASSETTE_NEST_Z - CASSETTE_RECESS_DEPTH / 2.0 + 0.2,
    );

    base - recess + cassette_slot_pockets() + cassette_datums_and_clamps()
}

fn cassette_slot_pockets() -> Part {
    let mut features = Part::empty(format!("{PREFIX}_cassette_slot_pockets"));
    for lane in 0..CHANNEL_LANES {
        let (x, y) = cassette_slot_center(lane);
        let pocket = centered_cube(
            format!("{PREFIX}_cassette_slot_{lane:02}_chip_clearance_pocket"),
            REVC_CHIP_LENGTH + CASSETTE_SLOT_CLEARANCE,
            REVC_CHIP_WIDTH + CASSETTE_SLOT_CLEARANCE,
            9.0,
        )
        .translate(
            x,
            y,
            BASE_Z / 2.0 + CASSETTE_NEST_Z - CASSETTE_RECESS_DEPTH + 4.0,
        );
        let lane_id = centered_cube(
            format!("{PREFIX}_cassette_slot_{lane:02}_raised_id_tab"),
            35.0,
            10.0,
            4.0,
        )
        .translate(
            x,
            y - REVC_CHIP_WIDTH / 2.0 - 11.0,
            BASE_Z / 2.0 + CASSETTE_NEST_Z + 2.0,
        );
        let window = centered_cube(
            format!("{PREFIX}_cassette_slot_{lane:02}_optical_settle_window"),
            52.0,
            20.0,
            4.0,
        )
        .translate(x, y, BASE_Z / 2.0 + CASSETTE_NEST_Z + 2.0);
        features = features + pocket + lane_id + window;
    }
    features
}

fn cassette_datums_and_clamps() -> Part {
    let mut features = Part::empty(format!("{PREFIX}_cassette_datums_and_clamps"));
    for (i, (x, y)) in cassette_corner_points(22.0).iter().enumerate() {
        let pin = centered_cylinder(
            format!("{PREFIX}_cassette_datum_pin_{i}"),
            CASSETTE_DATUM_PIN_D / 2.0,
            15.0,
            28,
        )
        .translate(*x, *y, BASE_Z / 2.0 + CASSETTE_NEST_Z + 7.5);
        features = features + pin;
    }
    for i in 0..CASSETTE_CLAMPS {
        let x = CASSETTE_CENTER.0 + centered_index(i, CASSETTE_CLAMPS, 72.0);
        let y = CASSETTE_CENTER.1 - CASSETTE_NEST_Y / 2.0 + 18.0;
        let clamp = centered_cube(
            format!("{PREFIX}_cassette_front_toggle_clamp_{i}"),
            42.0,
            16.0,
            12.0,
        )
        .translate(x, y, BASE_Z / 2.0 + CASSETTE_NEST_Z + 6.0);
        features = features + clamp;
    }
    features
}

fn parallel_serpentine_channel_plate() -> Part {
    let plate = centered_cube(
        format!("{PREFIX}_parallel_serpentine_channel_plate_body"),
        CHANNEL_PLATE_X,
        CHANNEL_PLATE_Y,
        CHANNEL_PLATE_Z,
    )
    .translate(
        CHANNEL_CENTER.0,
        CHANNEL_CENTER.1,
        insert_z(CHANNEL_PLATE_Z),
    );
    let cleanout = centered_cube(
        format!("{PREFIX}_parallel_serpentine_channel_plate_center_relief"),
        CHANNEL_PLATE_X - 42.0,
        CHANNEL_PLATE_Y - 40.0,
        5.0,
    )
    .translate(
        CHANNEL_CENTER.0,
        CHANNEL_CENTER.1,
        BASE_Z / 2.0 + CHANNEL_PLATE_Z - 2.0,
    );

    plate - cleanout
        + serpentine_channels()
        + channel_inlet_outlet_ports()
        + edge_center_channel_marks()
}

fn serpentine_channels() -> Part {
    let mut channels = Part::empty(format!("{PREFIX}_twelve_equal_length_serpentine_channels"));
    let z = BASE_Z + CHANNEL_PLATE_Z + CHANNEL_TRACE_Z / 2.0 + 0.8;
    for lane in 0..CHANNEL_LANES {
        let y = channel_y(lane);
        let trace = serpentine_trace(
            format!("{PREFIX}_serpentine_lane_{lane:02}_equal_length_trace"),
            CHANNEL_SERP_STRAIGHT_X,
            CHANNEL_SERP_TURN_Y,
            CHANNEL_TRACE_W,
            CHANNEL_TRACE_Z,
        )
        .translate(CHANNEL_CENTER.0, y, z);
        let wetting_bar = centered_cube(
            format!("{PREFIX}_serpentine_lane_{lane:02}_seed_wetting_witness_bar"),
            62.0,
            3.0,
            3.0,
        )
        .translate(
            CHANNEL_CENTER.0 - CHANNEL_SERP_STRAIGHT_X / 2.0 - 20.0,
            y,
            z + 5.0,
        );
        channels = channels + trace + wetting_bar;
    }
    channels
}

fn channel_inlet_outlet_ports() -> Part {
    let mut ports = Part::empty(format!("{PREFIX}_channel_inlet_outlet_ports"));
    let z = BASE_Z + CHANNEL_PLATE_Z + 4.0;
    for lane in 0..CHANNEL_LANES {
        let y = channel_y(lane);
        let inlet = port_ring(
            format!("{PREFIX}_channel_lane_{lane:02}_seed_inlet_port"),
            18.0,
            CHANNEL_CLEAR_BORE_D,
            7.0,
        )
        .translate(channel_inlet_x(), y, z);
        let outlet = port_ring(
            format!("{PREFIX}_channel_lane_{lane:02}_perfusion_outlet_port"),
            18.0,
            CHANNEL_CLEAR_BORE_D,
            7.0,
        )
        .translate(channel_outlet_x(), y, z);
        ports = ports + inlet + outlet;
    }
    ports
}

fn edge_center_channel_marks() -> Part {
    let mut marks = Part::empty(format!("{PREFIX}_edge_center_channel_marks"));
    for lane in 0..CHANNEL_LANES {
        let y = channel_y(lane);
        let d = if is_edge_channel(lane) { 12.0 } else { 19.0 };
        let mark = centered_cylinder(
            format!("{PREFIX}_channel_lane_{lane:02}_edge_center_marker"),
            d / 2.0,
            4.0,
            28,
        )
        .translate(
            CHANNEL_CENTER.0 + CHANNEL_PLATE_X / 2.0 - 28.0,
            y,
            BASE_Z + CHANNEL_PLATE_Z + 2.0,
        );
        marks = marks + mark;
    }
    marks
}

fn dye_front_timing_marker_bridge() -> Part {
    let bridge = centered_cube(
        format!("{PREFIX}_dye_front_timing_marker_bridge_body"),
        TIMING_BRIDGE_X,
        TIMING_BRIDGE_Y,
        TIMING_BRIDGE_Z,
    )
    .translate(TIMING_CENTER.0, TIMING_CENTER.1, insert_z(TIMING_BRIDGE_Z));
    let field = centered_cube(
        format!("{PREFIX}_dye_front_timing_marker_bridge_window_relief"),
        TIMING_BRIDGE_X - 58.0,
        TIMING_BRIDGE_Y - 36.0,
        6.0,
    )
    .translate(
        TIMING_CENTER.0,
        TIMING_CENTER.1,
        BASE_Z / 2.0 + TIMING_BRIDGE_Z - 2.8,
    );

    bridge - field + timing_ticks() + timing_camera_pods()
}

fn timing_ticks() -> Part {
    let mut ticks = Part::empty(format!("{PREFIX}_dye_front_timing_ticks"));
    let z = BASE_Z + TIMING_BRIDGE_Z + 2.0;
    for lane in 0..CHANNEL_LANES {
        let y = TIMING_CENTER.1 + centered_index(lane, CHANNEL_LANES, 5.8);
        for tick in 0..TIMING_TICKS_PER_LANE {
            let x =
                TIMING_CENTER.0 + centered_index(tick, TIMING_TICKS_PER_LANE, TIMING_TICK_PITCH_X);
            let marker = centered_cube(
                format!("{PREFIX}_lane_{lane:02}_dye_front_tick_{tick}"),
                if tick == 3 { TIMING_FLAG_X } else { 8.0 },
                TIMING_FLAG_Y,
                4.0,
            )
            .translate(x, y, z);
            ticks = ticks + marker;
        }
    }
    ticks
}

fn timing_camera_pods() -> Part {
    let mut pods = Part::empty(format!("{PREFIX}_timing_camera_pods"));
    for i in 0..TIMING_CAMERA_PODS {
        let x = TIMING_CENTER.0 + centered_index(i, TIMING_CAMERA_PODS, 250.0);
        let pod = centered_cube(
            format!("{PREFIX}_dye_front_camera_pod_{i}"),
            74.0,
            42.0,
            32.0,
        )
        .translate(x, TIMING_CENTER.1, BASE_Z + TIMING_BRIDGE_Z + 16.0);
        let lens = centered_cylinder(
            format!("{PREFIX}_dye_front_camera_lens_bore_{i}"),
            10.0,
            34.0,
            36,
        )
        .translate(x, TIMING_CENTER.1, BASE_Z + TIMING_BRIDGE_Z + 16.0);
        pods = pods + (pod - lens);
    }
    pods
}

fn flow_splitter_witness_paths() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_flow_splitter_witness_body"),
        SPLITTER_BODY_X,
        SPLITTER_BODY_Y,
        SPLITTER_BODY_Z,
    )
    .translate(
        SPLITTER_CENTER.0,
        SPLITTER_CENTER.1,
        insert_z(SPLITTER_BODY_Z),
    );
    let inlet = port_ring(
        format!("{PREFIX}_single_seed_perfusion_supply_port"),
        30.0,
        8.0,
        10.0,
    )
    .translate(
        SPLITTER_CENTER.0 - SPLITTER_BODY_X / 2.0 + 28.0,
        SPLITTER_CENTER.1,
        BASE_Z + SPLITTER_BODY_Z + 5.0,
    );
    body + inlet + splitter_tree() + splitter_equalization_witness_loops()
}

fn splitter_tree() -> Part {
    let mut tree = Part::empty(format!("{PREFIX}_balanced_splitter_tree"));
    let z = BASE_Z + SPLITTER_BODY_Z + 3.0;
    let trunk = centered_cube(
        format!("{PREFIX}_splitter_common_trunk"),
        SPLITTER_BODY_X - 56.0,
        SPLITTER_TRACE_W,
        6.0,
    )
    .translate(SPLITTER_CENTER.0 - 10.0, SPLITTER_CENTER.1, z);
    let header = centered_cube(
        format!("{PREFIX}_splitter_vertical_header"),
        SPLITTER_TRACE_W,
        SPLITTER_BODY_Y - 52.0,
        6.0,
    )
    .translate(SPLITTER_CENTER.0 + 32.0, SPLITTER_CENTER.1, z);
    tree = tree + trunk + header;

    for lane in 0..CHANNEL_LANES {
        let y = channel_y(lane);
        let branch = centered_cube(
            format!("{PREFIX}_splitter_witness_branch_lane_{lane:02}"),
            SPLITTER_BRANCH_X,
            SPLITTER_TRACE_W,
            6.0,
        )
        .translate(SPLITTER_CENTER.0 + 80.0, y, z);
        let dot = centered_cylinder(
            format!("{PREFIX}_splitter_witness_arrival_dot_lane_{lane:02}"),
            5.0,
            7.0,
            24,
        )
        .translate(
            SPLITTER_CENTER.0 + SPLITTER_BRANCH_X / 2.0 + 90.0,
            y,
            z + 0.5,
        );
        tree = tree + branch + dot;
    }
    tree
}

fn splitter_equalization_witness_loops() -> Part {
    let mut loops = Part::empty(format!("{PREFIX}_splitter_equalization_witness_loops"));
    let z = BASE_Z + SPLITTER_BODY_Z + 8.0;
    for lane in 0..CHANNEL_LANES {
        for loop_idx in 0..SPLITTER_EQUALIZATION_LOOPS {
            let x = SPLITTER_CENTER.0 - 14.0 + loop_idx as f64 * 28.0;
            let y = channel_y(lane);
            let loop_part = u_loop(
                format!("{PREFIX}_splitter_lane_{lane:02}_equalization_loop_{loop_idx}"),
                22.0,
                if loop_idx == 0 { 12.0 } else { -12.0 },
                4.5,
                4.0,
            )
            .translate(x, y, z);
            loops = loops + loop_part;
        }
    }
    loops
}

fn channel_window_coupon_carrier() -> Part {
    let carrier = centered_cube(
        format!("{PREFIX}_channel_window_coupon_carrier_body"),
        COUPON_CARRIER_X,
        COUPON_CARRIER_Y,
        COUPON_CARRIER_Z,
    )
    .translate(COUPON_CENTER.0, COUPON_CENTER.1, insert_z(COUPON_CARRIER_Z));
    let handle = centered_cube(
        format!("{PREFIX}_channel_window_coupon_carrier_pull_handle"),
        34.0,
        COUPON_CARRIER_Y - 44.0,
        16.0,
    )
    .translate(
        COUPON_CENTER.0 + COUPON_CARRIER_X / 2.0 - 18.0,
        COUPON_CENTER.1,
        BASE_Z + COUPON_CARRIER_Z + 8.0,
    );

    carrier + handle - coupon_slots() + coupon_window_lenses() + coupon_graticules()
}

fn coupon_slots() -> Part {
    let mut slots = Part::empty(format!("{PREFIX}_channel_window_coupon_slots"));
    for lane in 0..WINDOW_COUPONS {
        let y = channel_y(lane);
        let slot = centered_cube(
            format!("{PREFIX}_window_coupon_lane_{lane:02}_slot_clearance"),
            COUPON_SLOT_X,
            COUPON_SLOT_Y,
            COUPON_SLOT_DEPTH + 1.0,
        )
        .translate(
            COUPON_CENTER.0 - 34.0,
            y,
            BASE_Z / 2.0 + COUPON_CARRIER_Z - COUPON_SLOT_DEPTH / 2.0 + 0.3,
        );
        slots = slots + slot;
    }
    slots
}

fn coupon_window_lenses() -> Part {
    let mut lenses = Part::empty(format!("{PREFIX}_coupon_window_lens_lands"));
    for lane in 0..WINDOW_COUPONS {
        let y = channel_y(lane);
        let lens = centered_cube(
            format!("{PREFIX}_window_coupon_lane_{lane:02}_transparent_lens_land"),
            WINDOW_LENS_X,
            WINDOW_LENS_Y,
            4.0,
        )
        .translate(COUPON_CENTER.0 - 34.0, y, BASE_Z + COUPON_CARRIER_Z + 2.0);
        lenses = lenses + lens;
    }
    lenses
}

fn coupon_graticules() -> Part {
    let mut ticks = Part::empty(format!("{PREFIX}_coupon_graticule_ticks"));
    for lane in 0..WINDOW_COUPONS {
        let y = channel_y(lane);
        for tick in 0..WINDOW_GRATICULE_TICKS {
            let x = COUPON_CENTER.0 - 34.0 + centered_index(tick, WINDOW_GRATICULE_TICKS, 18.0);
            let mark = centered_cube(
                format!("{PREFIX}_window_coupon_lane_{lane:02}_graticule_tick_{tick}"),
                2.0,
                19.0,
                5.0,
            )
            .translate(x, y, BASE_Z + COUPON_CARRIER_Z + 4.5);
            ticks = ticks + mark;
        }
    }
    ticks
}

fn trapped_bubble_window_array() -> Part {
    let base = centered_cube(
        format!("{PREFIX}_trapped_bubble_window_array_body"),
        BUBBLE_ARRAY_X,
        BUBBLE_ARRAY_Y,
        BUBBLE_ARRAY_Z,
    )
    .translate(BUBBLE_CENTER.0, BUBBLE_CENTER.1, insert_z(BUBBLE_ARRAY_Z));
    base - bubble_window_cutouts() + bubble_domescale_markers() + wetness_strips()
}

fn bubble_window_cutouts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_bubble_window_cutouts"));
    for lane in 0..BUBBLE_WINDOWS {
        let (x, y) = bubble_window_position(lane);
        let cut = centered_cylinder(
            format!("{PREFIX}_bubble_window_lane_{lane:02}_clear_optical_bore"),
            BUBBLE_WINDOW_D / 2.0,
            BUBBLE_ARRAY_Z + 2.0,
            32,
        )
        .translate(x, y, insert_z(BUBBLE_ARRAY_Z));
        cuts = cuts + cut;
    }
    cuts
}

fn bubble_domescale_markers() -> Part {
    let mut domes = Part::empty(format!("{PREFIX}_bubble_dome_scale_markers"));
    for lane in 0..BUBBLE_WINDOWS {
        let (x, y) = bubble_window_position(lane);
        let ring = port_ring(
            format!("{PREFIX}_bubble_window_lane_{lane:02}_trap_dome_ring"),
            BUBBLE_DOME_D,
            BUBBLE_WINDOW_D,
            5.0,
        )
        .translate(x, y, BASE_Z + BUBBLE_ARRAY_Z + 2.5);
        let tick = centered_cube(
            format!("{PREFIX}_bubble_window_lane_{lane:02}_reject_threshold_tick"),
            18.0,
            3.0,
            4.0,
        )
        .translate(x, y + 16.0, BASE_Z + BUBBLE_ARRAY_Z + 2.0);
        domes = domes + ring + tick;
    }
    domes
}

fn wetness_strips() -> Part {
    let mut strips = Part::empty(format!("{PREFIX}_bubble_wetness_strips"));
    for lane in 0..BUBBLE_WINDOWS {
        let (x, y) = bubble_window_position(lane);
        let strip = centered_cube(
            format!("{PREFIX}_bubble_window_lane_{lane:02}_wetness_strip"),
            WETNESS_STRIP_X,
            WETNESS_STRIP_Y,
            3.0,
        )
        .translate(x + 32.0, y, BASE_Z + BUBBLE_ARRAY_Z + 1.5);
        strips = strips + strip;
    }
    strips
}

fn flow_balance_reference_sensor_pockets() -> Part {
    let rack = centered_cube(
        format!("{PREFIX}_flow_balance_reference_sensor_rack"),
        SENSOR_RACK_X,
        SENSOR_RACK_Y,
        SENSOR_RACK_Z,
    )
    .translate(SENSOR_CENTER.0, SENSOR_CENTER.1, insert_z(SENSOR_RACK_Z));
    rack - sensor_pocket_cutouts() + pressure_tap_pairs() + sensor_barcode_lands()
}

fn sensor_pocket_cutouts() -> Part {
    let mut pockets = Part::empty(format!("{PREFIX}_reference_sensor_pocket_cutouts"));
    for lane in 0..REFERENCE_SENSOR_POCKETS {
        let (x, y) = sensor_position(lane);
        let pocket = centered_cube(
            format!("{PREFIX}_reference_sensor_lane_{lane:02}_coupon_pocket"),
            SENSOR_POCKET_X,
            SENSOR_POCKET_Y,
            10.0,
        )
        .translate(x, y, BASE_Z / 2.0 + SENSOR_RACK_Z - 4.8);
        pockets = pockets + pocket;
    }
    pockets
}

fn pressure_tap_pairs() -> Part {
    let mut taps = Part::empty(format!("{PREFIX}_reference_sensor_pressure_taps"));
    for lane in 0..REFERENCE_SENSOR_POCKETS {
        let (x, y) = sensor_position(lane);
        for tap in 0..PRESSURE_TAPS_PER_LANE {
            let tx = x + centered_index(tap, PRESSURE_TAPS_PER_LANE, 24.0);
            let port = port_ring(
                format!("{PREFIX}_reference_sensor_lane_{lane:02}_pressure_tap_{tap}"),
                11.0,
                4.0,
                5.0,
            )
            .translate(tx, y + 24.0, BASE_Z + SENSOR_RACK_Z + 2.5);
            taps = taps + port;
        }
    }
    taps
}

fn sensor_barcode_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_reference_sensor_barcode_lands"));
    for lane in 0..REFERENCE_SENSOR_POCKETS {
        let (x, y) = sensor_position(lane);
        let land = centered_cube(
            format!("{PREFIX}_reference_sensor_lane_{lane:02}_barcode_land"),
            34.0,
            8.0,
            3.0,
        )
        .translate(x, y - 24.0, BASE_Z + SENSOR_RACK_Z + 1.5);
        lands = lands + land;
    }
    lands
}

fn timed_fraction_capture_rack() -> Part {
    let rack = centered_cube(
        format!("{PREFIX}_timed_fraction_capture_rack_body"),
        FRACTION_RACK_X,
        FRACTION_RACK_Y,
        FRACTION_RACK_Z,
    )
    .translate(
        FRACTION_CENTER.0,
        FRACTION_CENTER.1,
        insert_z(FRACTION_RACK_Z),
    );
    rack - fraction_pocket_cutouts() + fraction_time_bars()
}

fn fraction_pocket_cutouts() -> Part {
    let mut pockets = Part::empty(format!("{PREFIX}_timed_fraction_pocket_cutouts"));
    for lane in 0..CHANNEL_LANES {
        for timepoint in 0..FRACTION_TIMEPOINTS {
            let (x, y) = fraction_position(lane, timepoint);
            let pocket = centered_cylinder(
                format!("{PREFIX}_fraction_lane_{lane:02}_timepoint_{timepoint}_vial_pocket"),
                FRACTION_CLEARANCE_D / 2.0,
                16.0,
                28,
            )
            .translate(x, y, BASE_Z / 2.0 + FRACTION_RACK_Z - 7.0);
            pockets = pockets + pocket;
        }
    }
    pockets
}

fn fraction_time_bars() -> Part {
    let mut bars = Part::empty(format!("{PREFIX}_timed_fraction_time_bars"));
    for timepoint in 0..FRACTION_TIMEPOINTS {
        let y =
            FRACTION_CENTER.1 + centered_index(timepoint, FRACTION_TIMEPOINTS, FRACTION_PITCH_Y);
        let bar = centered_cube(
            format!("{PREFIX}_fraction_timepoint_{timepoint}_dye_front_bar"),
            FRACTION_RACK_X - 46.0,
            4.0,
            4.0,
        )
        .translate(FRACTION_CENTER.0, y + 15.0, BASE_Z + FRACTION_RACK_Z + 2.0);
        bars = bars + bar;
    }
    bars
}

fn reject_hold_release_gate_panel() -> Part {
    let panel = centered_cube(
        format!("{PREFIX}_reject_hold_release_gate_panel_body"),
        GATE_PANEL_X,
        GATE_PANEL_Y,
        GATE_PANEL_Z,
    )
    .translate(GATE_CENTER.0, GATE_CENTER.1, insert_z(GATE_PANEL_Z));
    panel - gate_slot_cutouts() + gate_status_tokens() + gate_labels()
}

fn gate_slot_cutouts() -> Part {
    let mut slots = Part::empty(format!("{PREFIX}_reject_hold_release_gate_slot_cutouts"));
    for lane in 0..DISPOSITION_LANES {
        let y = GATE_CENTER.1 + centered_index(lane, DISPOSITION_LANES, GATE_LANE_PITCH_Y);
        let slot = centered_cube(
            format!("{PREFIX}_gate_lane_{}_longitudinal_slot", gate_name(lane)),
            GATE_PANEL_X - 42.0,
            18.0,
            9.0,
        )
        .translate(GATE_CENTER.0, y, BASE_Z / 2.0 + GATE_PANEL_Z - 4.0);
        slots = slots + slot;
    }
    slots
}

fn gate_status_tokens() -> Part {
    let mut tokens = Part::empty(format!("{PREFIX}_reject_hold_release_gate_tokens"));
    for gate in 0..DISPOSITION_LANES {
        let y = GATE_CENTER.1 + centered_index(gate, DISPOSITION_LANES, GATE_LANE_PITCH_Y);
        for lane in 0..GATE_TOKENS_PER_LANE {
            let x = GATE_CENTER.0 + centered_index(lane, GATE_TOKENS_PER_LANE, GATE_TOKEN_PITCH_X);
            let token = centered_cube(
                format!("{PREFIX}_{}_gate_token_lane_{lane:02}", gate_name(gate)),
                GATE_TOKEN_X,
                GATE_TOKEN_Y,
                5.0,
            )
            .translate(x, y, BASE_Z + GATE_PANEL_Z + 2.5);
            tokens = tokens + token;
        }
    }
    tokens
}

fn gate_labels() -> Part {
    let mut labels = Part::empty(format!("{PREFIX}_gate_labels"));
    for gate in 0..DISPOSITION_LANES {
        let y = GATE_CENTER.1 + centered_index(gate, DISPOSITION_LANES, GATE_LANE_PITCH_Y);
        let label = centered_cube(
            format!("{PREFIX}_{}_gate_label_land", gate_name(gate)),
            54.0,
            12.0,
            4.0,
        )
        .translate(
            GATE_CENTER.0 - GATE_PANEL_X / 2.0 + 46.0,
            y,
            BASE_Z + GATE_PANEL_Z + 2.0,
        );
        labels = labels + label;
    }
    labels
}

fn barcode_custody_keepout_gauges() -> Part {
    let panel = centered_cube(
        format!("{PREFIX}_barcode_custody_panel"),
        CUSTODY_PANEL_X,
        CUSTODY_PANEL_Y,
        CUSTODY_PANEL_Z,
    )
    .translate(
        CUSTODY_CENTER.0,
        CUSTODY_CENTER.1,
        insert_z(CUSTODY_PANEL_Z),
    );
    panel + barcode_lands() + custody_token_lands() + keepout_gauges()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_barcode_lands"));
    for lane in 0..BARCODE_LANDS {
        let col = lane % 4;
        let row = lane / 4;
        let x = CUSTODY_CENTER.0 + centered_index(col, 4, 78.0) - 22.0;
        let y = CUSTODY_CENTER.1 + centered_index(row, 3, 32.0) + 20.0;
        lands = lands
            + label_plaque(
                format!("{PREFIX}_barcode_lane_{lane:02}"),
                BARCODE_LAND_X,
                BARCODE_LAND_Y,
                3.0,
                lane,
            )
            .translate(x, y, BASE_Z + CUSTODY_PANEL_Z + 1.5);
    }
    lands
}

fn custody_token_lands() -> Part {
    let mut tokens = Part::empty(format!("{PREFIX}_custody_token_lands"));
    for i in 0..CUSTODY_LANDS {
        let x = CUSTODY_CENTER.0 + centered_index(i, CUSTODY_LANDS, 45.0) + 18.0;
        let token = centered_cylinder(format!("{PREFIX}_custody_token_land_{i}"), 10.0, 4.0, 32)
            .translate(x, CUSTODY_CENTER.1 - 45.0, BASE_Z + CUSTODY_PANEL_Z + 2.0);
        tokens = tokens + token;
    }
    tokens
}

fn keepout_gauges() -> Part {
    let front = centered_cube(
        format!("{PREFIX}_front_robot_clearance_keepout"),
        ROBOT_KEEP_OUT_X,
        7.0,
        KEEP_OUT_RAIL_Z,
    )
    .translate(0.0, -ROBOT_KEEP_OUT_Y / 2.0, BASE_Z + KEEP_OUT_RAIL_Z / 2.0);
    let rear = centered_cube(
        format!("{PREFIX}_rear_tubing_service_keepout"),
        ROBOT_KEEP_OUT_X,
        7.0,
        KEEP_OUT_RAIL_Z,
    )
    .translate(0.0, ROBOT_KEEP_OUT_Y / 2.0, BASE_Z + KEEP_OUT_RAIL_Z / 2.0);
    let left = centered_cube(
        format!("{PREFIX}_left_cassette_service_keepout"),
        7.0,
        ROBOT_KEEP_OUT_Y,
        KEEP_OUT_RAIL_Z,
    )
    .translate(-ROBOT_KEEP_OUT_X / 2.0, 0.0, BASE_Z + KEEP_OUT_RAIL_Z / 2.0);
    let right = centered_cube(
        format!("{PREFIX}_right_window_service_keepout"),
        7.0,
        ROBOT_KEEP_OUT_Y,
        KEEP_OUT_RAIL_Z,
    )
    .translate(ROBOT_KEEP_OUT_X / 2.0, 0.0, BASE_Z + KEEP_OUT_RAIL_Z / 2.0);
    let camera = centered_cube(
        format!("{PREFIX}_dye_front_camera_z_clearance_gauge"),
        38.0,
        38.0,
        CAMERA_Z_CLEARANCE,
    )
    .translate(TIMING_CENTER.0, TIMING_CENTER.1, CAMERA_Z_CLEARANCE / 2.0);
    front + rear + left + right + camera
}

fn serpentine_trace(
    name: impl Into<String>,
    straight_x: f64,
    pitch_y: f64,
    width: f64,
    z: f64,
) -> Part {
    let name = name.into();
    let mut part = Part::empty(format!("{name}_path"));
    for leg in 0..CHANNEL_SERP_LEGS {
        let y = centered_index(leg, CHANNEL_SERP_LEGS, pitch_y);
        let x = if leg % 2 == 0 { 0.0 } else { 14.0 };
        let straight = centered_cube(format!("{name}_straight_leg_{leg}"), straight_x, width, z)
            .translate(x, y, 0.0);
        part = part + straight;
        if leg + 1 < CHANNEL_SERP_LEGS {
            let turn_x = if leg % 2 == 0 {
                straight_x / 2.0 + width / 2.0
            } else {
                -straight_x / 2.0 + width / 2.0
            };
            let turn_y = y + pitch_y / 2.0;
            let turn = centered_cube(
                format!("{name}_return_turn_{leg}"),
                width,
                pitch_y + width,
                z,
            )
            .translate(turn_x, turn_y, 0.0);
            part = part + turn;
        }
    }
    part
}

fn u_loop(name: impl Into<String>, x: f64, y: f64, width: f64, z: f64) -> Part {
    let name = name.into();
    let top = centered_cube(format!("{name}_top_leg"), x, width, z).translate(0.0, y / 2.0, 0.0);
    let bottom =
        centered_cube(format!("{name}_bottom_leg"), x, width, z).translate(0.0, -y / 2.0, 0.0);
    let end = centered_cube(format!("{name}_return"), width, y.abs() + width, z).translate(
        x / 2.0 - width / 2.0,
        0.0,
        0.0,
    );
    top + bottom + end
}

fn port_ring(name: impl Into<String>, outer_d: f64, inner_d: f64, z: f64) -> Part {
    let name = name.into();
    centered_cylinder(format!("{name}_outer"), outer_d / 2.0, z, 36)
        - centered_cylinder(format!("{name}_inner"), inner_d / 2.0, z + 1.0, 28)
}

fn fiducial_target(name: impl Into<String>) -> Part {
    let name = name.into();
    let ring = port_ring(format!("{name}_ring"), 22.0, 7.0, 4.0);
    let cross = centered_cube(format!("{name}_crosshair_x"), 25.0, 2.0, 3.0)
        + centered_cube(format!("{name}_crosshair_y"), 2.0, 25.0, 3.0);
    ring + cross
}

fn label_plaque(name: impl Into<String>, x: f64, y: f64, z: f64, seed: usize) -> Part {
    let name = name.into();
    let sheet = centered_cube(format!("{name}_sheet"), x, y, z);
    let mut bars = Part::empty(format!("{name}_bars"));
    for i in 0..LABEL_BAR_COUNT {
        let width = 1.8 + ((seed + i) % 4) as f64 * 1.1;
        let bar_x = -x / 2.0 + 6.0 + i as f64 * ((x - 12.0) / LABEL_BAR_COUNT as f64);
        let bar = centered_cube(format!("{name}_bar_{i}"), width, y - 4.0, z + 1.2).translate(
            bar_x,
            0.0,
            z / 2.0 + 0.6,
        );
        bars = bars + bar;
    }
    sheet + bars
}

fn insert_z(height: f64) -> f64 {
    BASE_Z / 2.0 + height / 2.0
}

fn mount_points() -> [(f64, f64); MOUNT_SLOT_COUNT] {
    [
        (-STATION_X / 2.0 + 56.0, -STATION_Y / 2.0 + 52.0),
        (STATION_X / 2.0 - 56.0, -STATION_Y / 2.0 + 52.0),
        (-STATION_X / 2.0 + 56.0, STATION_Y / 2.0 - 52.0),
        (STATION_X / 2.0 - 56.0, STATION_Y / 2.0 - 52.0),
        (0.0, -STATION_Y / 2.0 + 52.0),
        (0.0, STATION_Y / 2.0 - 52.0),
        (-STATION_X / 2.0 + 56.0, 0.0),
        (STATION_X / 2.0 - 56.0, 0.0),
    ]
}

fn cassette_slot_center(slot: usize) -> (f64, f64) {
    let col = slot % CHANNEL_COLS;
    let row = slot / CHANNEL_COLS;
    (
        CASSETTE_CENTER.0 + centered_index(col, CHANNEL_COLS, CASSETTE_CHIP_PITCH_X),
        CASSETTE_CENTER.1 + centered_index(row, CHANNEL_ROWS, CASSETTE_CHIP_PITCH_Y),
    )
}

fn cassette_corner_points(inset: f64) -> [(f64, f64); 4] {
    [
        (
            CASSETTE_CENTER.0 - CASSETTE_NEST_X / 2.0 + inset,
            CASSETTE_CENTER.1 - CASSETTE_NEST_Y / 2.0 + inset,
        ),
        (
            CASSETTE_CENTER.0 + CASSETTE_NEST_X / 2.0 - inset,
            CASSETTE_CENTER.1 - CASSETTE_NEST_Y / 2.0 + inset,
        ),
        (
            CASSETTE_CENTER.0 - CASSETTE_NEST_X / 2.0 + inset,
            CASSETTE_CENTER.1 + CASSETTE_NEST_Y / 2.0 - inset,
        ),
        (
            CASSETTE_CENTER.0 + CASSETTE_NEST_X / 2.0 - inset,
            CASSETTE_CENTER.1 + CASSETTE_NEST_Y / 2.0 - inset,
        ),
    ]
}

fn channel_y(lane: usize) -> f64 {
    CHANNEL_CENTER.1 + centered_index(lane, CHANNEL_LANES, CHANNEL_PITCH_Y)
}

fn channel_inlet_x() -> f64 {
    CHANNEL_CENTER.0 - CHANNEL_SERP_STRAIGHT_X / 2.0 - 26.0
}

fn channel_outlet_x() -> f64 {
    CHANNEL_CENTER.0 + CHANNEL_SERP_STRAIGHT_X / 2.0 + 40.0
}

fn is_edge_channel(lane: usize) -> bool {
    let col = lane % CHANNEL_COLS;
    let row = lane / CHANNEL_COLS;
    row == 0 || row == CHANNEL_ROWS - 1 || col == 0 || col == CHANNEL_COLS - 1
}

fn edge_channel_count() -> usize {
    (0..CHANNEL_LANES)
        .filter(|lane| is_edge_channel(*lane))
        .count()
}

fn center_channel_count() -> usize {
    CHANNEL_LANES - edge_channel_count()
}

fn sensor_position(lane: usize) -> (f64, f64) {
    let col = lane % SENSOR_COLS;
    let row = lane / SENSOR_COLS;
    (
        SENSOR_CENTER.0 + centered_index(col, SENSOR_COLS, SENSOR_PITCH_X),
        SENSOR_CENTER.1 + centered_index(row, SENSOR_ROWS, SENSOR_PITCH_Y),
    )
}

fn bubble_window_position(lane: usize) -> (f64, f64) {
    let col = lane % 6;
    let row = lane / 6;
    (
        BUBBLE_CENTER.0 + centered_index(col, 6, 54.0),
        BUBBLE_CENTER.1 + centered_index(row, 2, 66.0),
    )
}

fn fraction_position(lane: usize, timepoint: usize) -> (f64, f64) {
    (
        FRACTION_CENTER.0 + centered_index(lane, CHANNEL_LANES, FRACTION_PITCH_X),
        FRACTION_CENTER.1 + centered_index(timepoint, FRACTION_TIMEPOINTS, FRACTION_PITCH_Y),
    )
}

fn gate_name(gate: usize) -> &'static str {
    match gate {
        0 => "reject",
        1 => "hold",
        2 => "release",
        _ => "unknown",
    }
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_scoped_unique_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(OUTPUTS.len(), 12);
        assert_eq!(unique.len(), OUTPUTS.len());
        assert!(OUTPUTS.iter().all(|path| path.starts_with("output/")));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
        assert!(OUTPUTS.iter().all(|path| path.contains(PREFIX)));
        assert!(OUTPUTS[0].ends_with("_closed_leak_tray_base.stl"));
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }

    #[test]
    fn required_features_have_named_outputs() {
        assert_eq!(REQUIRED_FEATURES.len(), 11);
        for feature in REQUIRED_FEATURES {
            assert!(
                OUTPUTS.iter().any(|output| output.contains(feature)),
                "missing output for feature {feature}"
            );
        }
    }

    #[test]
    fn dimensions_are_plausible_for_closed_cassette_station() {
        assert_design_constraints();
        assert!(STATION_X >= 1400.0);
        assert!(STATION_Y >= 900.0);
        assert!(LEAK_BASIN_X < STATION_X);
        assert!(LEAK_BASIN_Y < STATION_Y);
        assert!(CASSETTE_NEST_X > CASSETTE_SLOT_ARRAY_X);
        assert!(CASSETTE_NEST_Y > CASSETTE_SLOT_ARRAY_Y);
        assert!(CHANNEL_PLATE_X > CHANNEL_SERP_STRAIGHT_X + 160.0);
        assert!(CHANNEL_PLATE_Y > (CHANNEL_LANES as f64 - 1.0) * CHANNEL_PITCH_Y);
    }

    #[test]
    fn serpentine_channels_are_equal_length_and_channel_complete() {
        assert_eq!(CHANNEL_LANES, 12);
        assert_eq!(CHANNEL_ROWS, 3);
        assert_eq!(CHANNEL_COLS, 4);
        assert_eq!(edge_channel_count(), EDGE_CHANNELS);
        assert_eq!(center_channel_count(), CENTER_CHANNELS);
        let first_y = channel_y(0);
        let last_y = channel_y(CHANNEL_LANES - 1);
        assert_eq!(first_y + last_y, 2.0 * CHANNEL_CENTER.1);
        for lane in 1..CHANNEL_LANES {
            assert_eq!(channel_y(lane) - channel_y(lane - 1), CHANNEL_PITCH_Y);
        }
        assert_eq!(CHANNEL_SERP_LEGS, 5);
        assert!(CHANNEL_MODELED_PATH_MM > 2_200.0);
        assert!(CHANNEL_LENGTH_TOLERANCE_MM <= 0.05);
    }

    #[test]
    fn witness_features_cover_every_channel() {
        assert_eq!(SPLITTER_WITNESS_PATHS, CHANNEL_LANES);
        assert_eq!(WINDOW_COUPONS, CHANNEL_LANES);
        assert_eq!(BUBBLE_WINDOWS, CHANNEL_LANES);
        assert_eq!(REFERENCE_SENSOR_POCKETS, CHANNEL_LANES);
        assert_eq!(PRESSURE_TAPS_PER_LANE * CHANNEL_LANES, 24);
        assert_eq!(FRACTION_POCKETS, CHANNEL_LANES * FRACTION_TIMEPOINTS);
        assert_eq!(DISPOSITION_LANES * GATE_TOKENS_PER_LANE, 36);
    }

    #[test]
    fn sensor_fraction_and_bubble_positions_are_unique() {
        let mut sensors = BTreeSet::new();
        let mut bubbles = BTreeSet::new();
        let mut fractions = BTreeSet::new();
        for lane in 0..CHANNEL_LANES {
            let (sx, sy) = sensor_position(lane);
            sensors.insert((sx.round() as i64, sy.round() as i64));
            let (bx, by) = bubble_window_position(lane);
            bubbles.insert((bx.round() as i64, by.round() as i64));
            for timepoint in 0..FRACTION_TIMEPOINTS {
                let (fx, fy) = fraction_position(lane, timepoint);
                fractions.insert((fx.round() as i64, fy.round() as i64));
            }
        }
        assert_eq!(sensors.len(), REFERENCE_SENSOR_POCKETS);
        assert_eq!(bubbles.len(), BUBBLE_WINDOWS);
        assert_eq!(fractions.len(), FRACTION_POCKETS);
    }
}
