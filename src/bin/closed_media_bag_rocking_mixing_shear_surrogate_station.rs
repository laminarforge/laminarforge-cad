use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed media-bag rocking/mixing shear surrogate station.
//
// Intent:
// - Validate a no-cell closed media/additive bag surrogate before feed into
//   multiplexed chips, with visible witness features for gentle rocking,
//   mixing homogeneity, bubble/degas behavior, and service clearances.
// - Make sample-loop pockets, tracer windows, tilt/acceleration logging, tubing
//   strain relief, load-cell placement, leak containment, status traceability,
//   and robot/service keepouts explicit in the fixture geometry.
// - This is validation/interface CAD only. It is not a cell-culture protocol,
//   shear acceptance criterion, sterility claim, or pump/rocker control system.

const OUTPUTS: [&str; 13] = [
    "output/closed_media_bag_rocking_mixing_shear_surrogate_station_leak_tray_base.stl",
    "output/closed_media_bag_rocking_mixing_shear_surrogate_station_bag_cradle.stl",
    "output/closed_media_bag_rocking_mixing_shear_surrogate_station_rocker_arc_witness_rails.stl",
    "output/closed_media_bag_rocking_mixing_shear_surrogate_station_strain_relief_tubing_comb.stl",
    "output/closed_media_bag_rocking_mixing_shear_surrogate_station_top_middle_bottom_sample_loop_pockets.stl",
    "output/closed_media_bag_rocking_mixing_shear_surrogate_station_tracer_coupon_windows.stl",
    "output/closed_media_bag_rocking_mixing_shear_surrogate_station_bubble_degas_witnesses.stl",
    "output/closed_media_bag_rocking_mixing_shear_surrogate_station_accelerometer_tilt_logger_nests.stl",
    "output/closed_media_bag_rocking_mixing_shear_surrogate_station_load_cell_pad_placeholder.stl",
    "output/closed_media_bag_rocking_mixing_shear_surrogate_station_barcode_status_lanes.stl",
    "output/closed_media_bag_rocking_mixing_shear_surrogate_station_robot_service_keepouts.stl",
    "output/closed_media_bag_rocking_mixing_shear_surrogate_station_run_reference_tokens.stl",
    "output/closed_media_bag_rocking_mixing_shear_surrogate_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 11] = [
    "bag_cradle",
    "rocker_arc_witness_rails",
    "strain_relief_tubing_comb",
    "top_middle_bottom_sample_loop_pockets",
    "tracer_coupon_windows",
    "bubble_degas_witnesses",
    "accelerometer_tilt_logger_nests",
    "load_cell_pad_placeholder",
    "leak_tray_base",
    "barcode_status_lanes",
    "robot_service_keepouts",
];

const STATION_X: f64 = 1420.0;
const STATION_Y: f64 = 900.0;
const BASE_Z: f64 = 24.0;
const CURB_W: f64 = 22.0;
const CURB_Z: f64 = 58.0;
const SUMP_X: f64 = 1240.0;
const SUMP_Y: f64 = 720.0;
const SUMP_DEPTH: f64 = 8.0;
const DRAIN_PORT_D: f64 = 18.0;
const MOUNT_HOLE_D: f64 = 6.8;
const LEAK_SENSOR_WELLS: usize = 8;

const CRADLE_POS: (f64, f64) = (-390.0, 160.0);
const CRADLE_X: f64 = 520.0;
const CRADLE_Y: f64 = 300.0;
const CRADLE_Z: f64 = 54.0;
const BAG_POCKET_X: f64 = 400.0;
const BAG_POCKET_Y: f64 = 185.0;
const BAG_POCKET_DEPTH: f64 = 20.0;
const CRADLE_RIBS: usize = 9;
const CRADLE_LOCATOR_PINS: usize = 6;
const BAG_PORT_KEYS: usize = 6;

const ROCKER_ARC_POS: (f64, f64) = CRADLE_POS;
const ROCKER_ARC_RADIUS: f64 = 255.0;
const ROCKER_ARC_WIDTH: f64 = 22.0;
const ROCKER_ARC_SEGMENTS: usize = 15;
const ROCKER_LIMIT_DEG: f64 = 10.0;
const ROCKER_LIMIT_STOPS: usize = 4;
const ROCKER_PIVOT_D: f64 = 32.0;

const TUBING_POS: (f64, f64) = (95.0, 320.0);
const COMB_X: f64 = 590.0;
const COMB_Y: f64 = 96.0;
const COMB_Z: f64 = 38.0;
const TUBE_CHANNELS: usize = 12;
const TUBE_CHANNEL_D: f64 = 12.0;
const TUBE_CHANNEL_PITCH: f64 = 42.0;

const SAMPLE_POS: (f64, f64) = (425.0, 150.0);
const SAMPLE_X: f64 = 350.0;
const SAMPLE_Y: f64 = 305.0;
const SAMPLE_Z: f64 = 44.0;
const SAMPLE_LEVELS: usize = 3;
const SAMPLE_LOOPS_PER_LEVEL: usize = 4;
const SAMPLE_LOOP_D: f64 = 18.0;
const SAMPLE_LOOP_PITCH_X: f64 = 64.0;
const SAMPLE_LEVEL_PITCH_Y: f64 = 82.0;

const TRACER_POS: (f64, f64) = (-445.0, -155.0);
const TRACER_X: f64 = 380.0;
const TRACER_Y: f64 = 172.0;
const TRACER_Z: f64 = 26.0;
const TRACER_WINDOWS: usize = 8;
const TRACER_WINDOW_X: f64 = 72.0;
const TRACER_WINDOW_Y: f64 = 46.0;
const TRACER_COLS: usize = 4;
const COLOR_PATCHES: usize = 6;

const BUBBLE_POS: (f64, f64) = (-20.0, -155.0);
const BUBBLE_X: f64 = 350.0;
const BUBBLE_Y: f64 = 172.0;
const BUBBLE_Z: f64 = 42.0;
const SIGHT_CHANNELS: usize = 4;
const BUBBLE_TICKS: usize = 10;
const DEGAS_CHAMBERS: usize = 3;

const LOGGER_POS: (f64, f64) = (390.0, -155.0);
const LOGGER_X: f64 = 360.0;
const LOGGER_Y: f64 = 172.0;
const LOGGER_Z: f64 = 42.0;
const ACCEL_NESTS: usize = 3;
const TILT_LOGGER_NESTS: usize = 2;
const TILT_TICKS: usize = 13;

const LOAD_POS: (f64, f64) = (-455.0, -332.0);
const LOAD_X: f64 = 290.0;
const LOAD_Y: f64 = 110.0;
const LOAD_Z: f64 = 26.0;
const LOAD_PAD_X: f64 = 190.0;
const LOAD_PAD_Y: f64 = 66.0;
const LOAD_DATUM_PADS: usize = 4;

const STATUS_POS: (f64, f64) = (-70.0, -332.0);
const STATUS_X: f64 = 420.0;
const STATUS_Y: f64 = 110.0;
const STATUS_Z: f64 = 18.0;
const BARCODE_LANES: usize = 6;
const STATUS_LANES: usize = 3;
const STATUS_TOKENS_PER_LANE: usize = 4;

const TOKENS_POS: (f64, f64) = (385.0, -332.0);
const TOKEN_X: f64 = 330.0;
const TOKEN_Y: f64 = 110.0;
const TOKEN_Z: f64 = 20.0;
const REFERENCE_TOKENS: usize = 10;

const FRONT_ROBOT_CLEARANCE: f64 = 420.0;
const REAR_SERVICE_CLEARANCE: f64 = 260.0;
const LEFT_BAG_SERVICE_CLEARANCE: f64 = 220.0;
const RIGHT_SAMPLE_SERVICE_CLEARANCE: f64 = 230.0;
const TOP_ROCKER_LIFT_CLEARANCE: f64 = 360.0;
const KEEP_OUT_RAIL: f64 = 8.0;

#[derive(Clone, Copy)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_station(self) -> bool {
        let usable_x = STATION_X / 2.0 - CURB_W - 26.0;
        let usable_y = STATION_Y / 2.0 - CURB_W - 26.0;
        self.center.0 - self.x / 2.0 >= -usable_x
            && self.center.0 + self.x / 2.0 <= usable_x
            && self.center.1 - self.y / 2.0 >= -usable_y
            && self.center.1 + self.y / 2.0 <= usable_y
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = leak_tray_base();
    export(OUTPUTS[0], &base);

    let cradle = bag_cradle();
    export(OUTPUTS[1], &cradle);

    let rocker = rocker_arc_witness_rails();
    export(OUTPUTS[2], &rocker);

    let comb = strain_relief_tubing_comb();
    export(OUTPUTS[3], &comb);

    let samples = top_middle_bottom_sample_loop_pockets();
    export(OUTPUTS[4], &samples);

    let tracer = tracer_coupon_windows();
    export(OUTPUTS[5], &tracer);

    let bubbles = bubble_degas_witnesses();
    export(OUTPUTS[6], &bubbles);

    let loggers = accelerometer_tilt_logger_nests();
    export(OUTPUTS[7], &loggers);

    let load_cell = load_cell_pad_placeholder();
    export(OUTPUTS[8], &load_cell);

    let status = barcode_status_lanes();
    export(OUTPUTS[9], &status);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let tokens = run_reference_tokens();
    export(OUTPUTS[11], &tokens);

    let assembly = station_assembly();
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed media-bag rocking/mixing shear surrogate station:");
    println!(
        "  Leak tray:       {STATION_X:.0}mm x {STATION_Y:.0}mm with {LEAK_SENSOR_WELLS} wells, {DRAIN_PORT_D:.0}mm drain, raised secondary curbs"
    );
    println!(
        "  Bag cradle:      {CRADLE_X:.0}mm x {CRADLE_Y:.0}mm no-cell bag pocket, {CRADLE_RIBS} saddle ribs, {CRADLE_LOCATOR_PINS} locator pins, {BAG_PORT_KEYS} port key lands"
    );
    println!(
        "  Rocking witness: +/-{ROCKER_LIMIT_DEG:.0}deg arc rails with {ROCKER_ARC_SEGMENTS} witness segments, pivot bosses, and {ROCKER_LIMIT_STOPS} hard-stop flags"
    );
    println!(
        "  Mixing checks:   {SAMPLE_LEVELS} top/middle/bottom rows, {SAMPLE_LOOPS_PER_LEVEL} sample-loop pockets per row, {TRACER_WINDOWS} tracer coupon windows"
    );
    println!(
        "  Bubbles/logging: {SIGHT_CHANNELS} bubble sight channels, {DEGAS_CHAMBERS} degas witnesses, {ACCEL_NESTS} accelerometer nests, {TILT_LOGGER_NESTS} tilt logger nests"
    );
    println!(
        "  Service space:   front robot {FRONT_ROBOT_CLEARANCE:.0}mm, rear service {REAR_SERVICE_CLEARANCE:.0}mm, left bag {LEFT_BAG_SERVICE_CLEARANCE:.0}mm, right sample {RIGHT_SAMPLE_SERVICE_CLEARANCE:.0}mm, top lift {TOP_ROCKER_LIFT_CLEARANCE:.0}mm"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    leak_tray_base()
        + bag_cradle().translate(CRADLE_POS.0, CRADLE_POS.1, BASE_Z)
        + rocker_arc_witness_rails().translate(ROCKER_ARC_POS.0, ROCKER_ARC_POS.1, BASE_Z + 2.0)
        + strain_relief_tubing_comb().translate(TUBING_POS.0, TUBING_POS.1, BASE_Z)
        + top_middle_bottom_sample_loop_pockets().translate(SAMPLE_POS.0, SAMPLE_POS.1, BASE_Z)
        + tracer_coupon_windows().translate(TRACER_POS.0, TRACER_POS.1, BASE_Z)
        + bubble_degas_witnesses().translate(BUBBLE_POS.0, BUBBLE_POS.1, BASE_Z)
        + accelerometer_tilt_logger_nests().translate(LOGGER_POS.0, LOGGER_POS.1, BASE_Z)
        + load_cell_pad_placeholder().translate(LOAD_POS.0, LOAD_POS.1, BASE_Z)
        + barcode_status_lanes().translate(STATUS_POS.0, STATUS_POS.1, BASE_Z)
        + run_reference_tokens().translate(TOKENS_POS.0, TOKENS_POS.1, BASE_Z)
        + robot_service_keepouts()
}

fn leak_tray_base() -> Part {
    let deck = centered_cube(
        "closed_media_bag_rocking_leak_tray_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let sump = centered_cube(
        "closed_media_bag_rocking_recessed_leak_sump_cut",
        SUMP_X,
        SUMP_Y,
        SUMP_DEPTH + 1.0,
    )
    .translate(0.0, -8.0, BASE_Z - SUMP_DEPTH / 2.0 + 0.5);
    let drain = centered_cylinder(
        "closed_media_bag_rocking_front_drain_port_cut",
        DRAIN_PORT_D / 2.0,
        CURB_W + 36.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, -STATION_Y / 2.0 + 13.0, BASE_Z - 6.0);

    deck - sump - drain - deck_mount_holes()
        + tray_curbs()
        + leak_sensor_wells()
        + module_floor_markers()
        + datum_targets()
}

fn tray_curbs() -> Part {
    let front = centered_cube(
        "closed_media_bag_rocking_front_leak_tray_curb",
        STATION_X,
        CURB_W,
        CURB_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + CURB_W / 2.0, BASE_Z + CURB_Z / 2.0);
    let rear = centered_cube(
        "closed_media_bag_rocking_rear_leak_tray_curb",
        STATION_X,
        CURB_W,
        CURB_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - CURB_W / 2.0, BASE_Z + CURB_Z / 2.0);
    let left = centered_cube(
        "closed_media_bag_rocking_left_leak_tray_curb",
        CURB_W,
        STATION_Y,
        CURB_Z,
    )
    .translate(-STATION_X / 2.0 + CURB_W / 2.0, 0.0, BASE_Z + CURB_Z / 2.0);
    let right = centered_cube(
        "closed_media_bag_rocking_right_leak_tray_curb",
        CURB_W,
        STATION_Y,
        CURB_Z,
    )
    .translate(STATION_X / 2.0 - CURB_W / 2.0, 0.0, BASE_Z + CURB_Z / 2.0);
    front + rear + left + right
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("closed_media_bag_rocking_deck_mount_holes");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 62.0, -STATION_Y / 2.0 + 60.0),
        (STATION_X / 2.0 - 62.0, -STATION_Y / 2.0 + 60.0),
        (-STATION_X / 2.0 + 62.0, STATION_Y / 2.0 - 60.0),
        (STATION_X / 2.0 - 62.0, STATION_Y / 2.0 - 60.0),
        (0.0, -STATION_Y / 2.0 + 60.0),
        (0.0, STATION_Y / 2.0 - 60.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("closed_media_bag_rocking_m6_mount_hole_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 4.0,
                28,
            )
            .translate(*x, *y, BASE_Z / 2.0);
    }
    holes
}

fn leak_sensor_wells() -> Part {
    let mut wells = Part::empty("closed_media_bag_rocking_leak_sensor_wells");
    for i in 0..LEAK_SENSOR_WELLS {
        let x = centered_index(i % 4, 4, 175.0);
        let y = -330.0 + (i / 4) as f64 * 36.0;
        wells = wells
            + centered_cylinder(
                format!("closed_media_bag_rocking_leak_sensor_well_{i}"),
                17.0,
                8.0,
                32,
            )
            .translate(x, y, BASE_Z + 4.0);
    }
    wells
}

fn module_floor_markers() -> Part {
    let mut markers = Part::empty("closed_media_bag_rocking_module_floor_markers");
    for rect in module_rects() {
        markers = markers
            + centered_cube(
                format!("closed_media_bag_rocking_{}_floor_marker", rect.name),
                rect.x + 22.0,
                rect.y + 18.0,
                3.0,
            )
            .translate(rect.center.0, rect.center.1, BASE_Z + 1.5);
    }
    markers
}

fn datum_targets() -> Part {
    let mut targets = Part::empty("closed_media_bag_rocking_datum_targets");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 104.0, -STATION_Y / 2.0 + 100.0),
        (STATION_X / 2.0 - 104.0, -STATION_Y / 2.0 + 100.0),
        (-STATION_X / 2.0 + 104.0, STATION_Y / 2.0 - 100.0),
        (STATION_X / 2.0 - 104.0, STATION_Y / 2.0 - 100.0),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("closed_media_bag_rocking_datum_boss_{i}"),
            18.0,
            7.0,
            36,
        )
        .translate(*x, *y, BASE_Z + 3.5);
        let center = centered_cylinder(
            format!("closed_media_bag_rocking_datum_center_cut_{i}"),
            4.0,
            8.0,
            24,
        )
        .translate(*x, *y, BASE_Z + 4.0);
        targets = targets + (boss - center);
    }
    targets
}

fn bag_cradle() -> Part {
    let body = centered_cube(
        "closed_media_bag_rocking_bag_cradle_body",
        CRADLE_X,
        CRADLE_Y,
        CRADLE_Z,
    )
    .translate(0.0, 0.0, CRADLE_Z / 2.0);
    let pocket = centered_cube(
        "closed_media_bag_rocking_no_cell_media_bag_pocket_cut",
        BAG_POCKET_X,
        BAG_POCKET_Y,
        BAG_POCKET_DEPTH + 1.0,
    )
    .translate(0.0, -8.0, CRADLE_Z - BAG_POCKET_DEPTH / 2.0 + 0.5);
    body - pocket + cradle_saddle_ribs() + cradle_locator_pins() + bag_port_key_lands()
}

fn cradle_saddle_ribs() -> Part {
    let mut ribs = Part::empty("closed_media_bag_rocking_gentle_bag_saddle_ribs");
    for i in 0..CRADLE_RIBS {
        ribs = ribs
            + centered_cube(
                format!("closed_media_bag_rocking_low_shear_saddle_rib_{i}"),
                BAG_POCKET_X - 34.0,
                6.0,
                12.0,
            )
            .translate(
                0.0,
                centered_index(i, CRADLE_RIBS, 23.0) - 8.0,
                CRADLE_Z + 6.0,
            );
    }
    ribs
}

fn cradle_locator_pins() -> Part {
    let mut pins = Part::empty("closed_media_bag_rocking_bag_locator_pins");
    for i in 0..CRADLE_LOCATOR_PINS {
        let x = centered_index(i % 3, 3, 150.0);
        let y = if i < 3 { -128.0 } else { 108.0 };
        pins = pins
            + centered_cylinder(
                format!("closed_media_bag_rocking_soft_bag_locator_pin_{i}"),
                10.0,
                18.0,
                28,
            )
            .translate(x, y, CRADLE_Z + 9.0);
    }
    pins
}

fn bag_port_key_lands() -> Part {
    let mut keys = Part::empty("closed_media_bag_rocking_bag_port_key_lands");
    for i in 0..BAG_PORT_KEYS {
        keys = keys
            + centered_cube(
                format!("closed_media_bag_rocking_bag_port_key_land_{i}"),
                38.0,
                24.0,
                12.0,
            )
            .translate(
                centered_index(i, BAG_PORT_KEYS, 54.0),
                -CRADLE_Y / 2.0 + 24.0,
                CRADLE_Z + 6.0,
            );
    }
    keys
}

fn rocker_arc_witness_rails() -> Part {
    rocker_arc_segments(-76.0, "left")
        + rocker_arc_segments(76.0, "right")
        + rocker_pivots()
        + rocker_limit_stops()
}

fn rocker_arc_segments(y_offset: f64, side: &str) -> Part {
    let mut rail = Part::empty(format!(
        "closed_media_bag_rocking_{side}_rocker_arc_witness_rail"
    ));
    for i in 0..ROCKER_ARC_SEGMENTS {
        let frac = i as f64 / (ROCKER_ARC_SEGMENTS - 1) as f64;
        let deg = -ROCKER_LIMIT_DEG + frac * ROCKER_LIMIT_DEG * 2.0;
        let rad = deg.to_radians();
        let x = rad.sin() * ROCKER_ARC_RADIUS;
        let z = 30.0 + (1.0 - rad.cos()) * ROCKER_ARC_RADIUS;
        rail = rail
            + centered_cube(
                format!("closed_media_bag_rocking_{side}_arc_witness_segment_{i}"),
                ROCKER_ARC_WIDTH,
                18.0,
                10.0,
            )
            .translate(x, y_offset, z);
    }
    rail
}

fn rocker_pivots() -> Part {
    let left = centered_cylinder(
        "closed_media_bag_rocking_left_rocker_pivot_boss",
        ROCKER_PIVOT_D / 2.0,
        28.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, -106.0, 34.0);
    let right = centered_cylinder(
        "closed_media_bag_rocking_right_rocker_pivot_boss",
        ROCKER_PIVOT_D / 2.0,
        28.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, 106.0, 34.0);
    left + right
}

fn rocker_limit_stops() -> Part {
    let mut stops = Part::empty("closed_media_bag_rocking_limit_stop_flags");
    for i in 0..ROCKER_LIMIT_STOPS {
        let x = if i % 2 == 0 { -84.0 } else { 84.0 };
        let y = if i < 2 { -130.0 } else { 130.0 };
        stops = stops
            + centered_cube(
                format!("closed_media_bag_rocking_shear_spike_limit_stop_flag_{i}"),
                28.0,
                18.0,
                54.0,
            )
            .translate(x, y, 27.0);
    }
    stops
}

fn strain_relief_tubing_comb() -> Part {
    let base = centered_cube(
        "closed_media_bag_rocking_strain_relief_tubing_comb_body",
        COMB_X,
        COMB_Y,
        COMB_Z,
    )
    .translate(0.0, 0.0, COMB_Z / 2.0);
    let mut channels = Part::empty("closed_media_bag_rocking_tubing_comb_channels");
    for i in 0..TUBE_CHANNELS {
        channels = channels
            + centered_cylinder(
                format!("closed_media_bag_rocking_tubing_comb_channel_cut_{i}"),
                TUBE_CHANNEL_D / 2.0,
                COMB_Y + 4.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(i, TUBE_CHANNELS, TUBE_CHANNEL_PITCH),
                0.0,
                COMB_Z - 10.0,
            );
    }
    base - channels + tubing_number_tabs()
}

fn tubing_number_tabs() -> Part {
    let mut tabs = Part::empty("closed_media_bag_rocking_tubing_number_tabs");
    for i in 0..TUBE_CHANNELS {
        tabs = tabs
            + centered_cube(
                format!("closed_media_bag_rocking_tubing_route_barcode_tab_{i}"),
                24.0,
                12.0,
                4.0,
            )
            .translate(
                centered_index(i, TUBE_CHANNELS, TUBE_CHANNEL_PITCH),
                COMB_Y / 2.0 + 8.0,
                COMB_Z + 2.0,
            );
    }
    tabs
}

fn top_middle_bottom_sample_loop_pockets() -> Part {
    let body = centered_cube(
        "closed_media_bag_rocking_sample_loop_pocket_block",
        SAMPLE_X,
        SAMPLE_Y,
        SAMPLE_Z,
    )
    .translate(0.0, 0.0, SAMPLE_Z / 2.0);
    let mut loops = Part::empty("closed_media_bag_rocking_top_middle_bottom_sample_loop_pockets");
    for level in 0..SAMPLE_LEVELS {
        let y = centered_index(level, SAMPLE_LEVELS, SAMPLE_LEVEL_PITCH_Y);
        for loop_i in 0..SAMPLE_LOOPS_PER_LEVEL {
            let x = centered_index(loop_i, SAMPLE_LOOPS_PER_LEVEL, SAMPLE_LOOP_PITCH_X);
            loops = loops
                + centered_cylinder(
                    format!(
                        "closed_media_bag_rocking_sample_level_{level}_loop_pocket_{loop_i}_cut"
                    ),
                    SAMPLE_LOOP_D / 2.0,
                    SAMPLE_Z + 3.0,
                    32,
                )
                .translate(x, y, SAMPLE_Z / 2.0 + 1.5);
        }
        loops = loops
            + centered_cube(
                format!("closed_media_bag_rocking_sample_level_{level}_label_land"),
                SAMPLE_X - 40.0,
                10.0,
                5.0,
            )
            .translate(0.0, y + 28.0, SAMPLE_Z + 2.5);
    }
    body - loops + sample_loop_retainer_posts()
}

fn sample_loop_retainer_posts() -> Part {
    let mut posts = Part::empty("closed_media_bag_rocking_sample_loop_retainer_posts");
    for level in 0..SAMPLE_LEVELS {
        let y = centered_index(level, SAMPLE_LEVELS, SAMPLE_LEVEL_PITCH_Y);
        for i in 0..5 {
            posts = posts
                + centered_cylinder(
                    format!("closed_media_bag_rocking_sample_level_{level}_retainer_post_{i}"),
                    4.5,
                    14.0,
                    18,
                )
                .translate(centered_index(i, 5, 58.0), y - 28.0, SAMPLE_Z + 7.0);
        }
    }
    posts
}

fn tracer_coupon_windows() -> Part {
    let base = centered_cube(
        "closed_media_bag_rocking_tracer_coupon_window_plate",
        TRACER_X,
        TRACER_Y,
        TRACER_Z,
    )
    .translate(0.0, 0.0, TRACER_Z / 2.0);
    let mut windows = Part::empty("closed_media_bag_rocking_tracer_coupon_windows");
    for i in 0..TRACER_WINDOWS {
        let x = centered_index(i % TRACER_COLS, TRACER_COLS, 86.0);
        let y = if i < TRACER_COLS { -34.0 } else { 42.0 };
        let frame = centered_cube(
            format!("closed_media_bag_rocking_tracer_coupon_window_frame_{i}"),
            TRACER_WINDOW_X,
            TRACER_WINDOW_Y,
            10.0,
        )
        .translate(x, y, TRACER_Z + 5.0);
        let pane = centered_cube(
            format!("closed_media_bag_rocking_tracer_coupon_clear_window_cut_{i}"),
            TRACER_WINDOW_X - 18.0,
            TRACER_WINDOW_Y - 14.0,
            12.0,
        )
        .translate(x, y, TRACER_Z + 6.0);
        windows = windows + (frame - pane);
    }
    base + windows + tracer_color_reference_patches()
}

fn tracer_color_reference_patches() -> Part {
    let mut patches = Part::empty("closed_media_bag_rocking_tracer_color_reference_patches");
    for i in 0..COLOR_PATCHES {
        patches = patches
            + centered_cube(
                format!("closed_media_bag_rocking_tracer_color_patch_{i}"),
                34.0,
                18.0,
                4.0,
            )
            .translate(
                centered_index(i, COLOR_PATCHES, 42.0),
                -TRACER_Y / 2.0 + 16.0,
                TRACER_Z + 2.0,
            );
    }
    patches
}

fn bubble_degas_witnesses() -> Part {
    let body = centered_cube(
        "closed_media_bag_rocking_bubble_degas_witness_block",
        BUBBLE_X,
        BUBBLE_Y,
        BUBBLE_Z,
    )
    .translate(0.0, 0.0, BUBBLE_Z / 2.0);
    body - bubble_sight_channel_cuts() + bubble_ladder_ticks() + degas_witness_chambers()
}

fn bubble_sight_channel_cuts() -> Part {
    let mut cuts = Part::empty("closed_media_bag_rocking_bubble_sight_channel_cuts");
    for i in 0..SIGHT_CHANNELS {
        cuts = cuts
            + centered_cylinder(
                format!("closed_media_bag_rocking_bubble_sight_tube_channel_{i}"),
                9.0,
                BUBBLE_Y + 4.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(i, SIGHT_CHANNELS, 56.0),
                0.0,
                BUBBLE_Z - 15.0,
            );
    }
    cuts
}

fn bubble_ladder_ticks() -> Part {
    let mut ticks = Part::empty("closed_media_bag_rocking_bubble_ladder_ticks");
    for i in 0..BUBBLE_TICKS {
        ticks = ticks
            + centered_cube(
                format!("closed_media_bag_rocking_bubble_ladder_tick_{i}"),
                if i % 5 == 0 { 34.0 } else { 20.0 },
                4.0,
                5.0,
            )
            .translate(
                -BUBBLE_X / 2.0 + 36.0,
                centered_index(i, BUBBLE_TICKS, 12.0),
                BUBBLE_Z + 2.5,
            );
    }
    ticks
}

fn degas_witness_chambers() -> Part {
    let mut chambers = Part::empty("closed_media_bag_rocking_degas_witness_chambers");
    for i in 0..DEGAS_CHAMBERS {
        let boss = centered_cylinder(
            format!("closed_media_bag_rocking_degas_witness_chamber_{i}"),
            22.0,
            18.0,
            36,
        )
        .translate(
            centered_index(i, DEGAS_CHAMBERS, 82.0) + 60.0,
            BUBBLE_Y / 2.0 - 34.0,
            BUBBLE_Z + 9.0,
        );
        let port = centered_cylinder(
            format!("closed_media_bag_rocking_degas_witness_vent_port_cut_{i}"),
            7.0,
            20.0,
            24,
        )
        .translate(
            centered_index(i, DEGAS_CHAMBERS, 82.0) + 60.0,
            BUBBLE_Y / 2.0 - 34.0,
            BUBBLE_Z + 10.0,
        );
        chambers = chambers + (boss - port);
    }
    chambers
}

fn accelerometer_tilt_logger_nests() -> Part {
    let body = centered_cube(
        "closed_media_bag_rocking_accelerometer_tilt_logger_nest_block",
        LOGGER_X,
        LOGGER_Y,
        LOGGER_Z,
    )
    .translate(0.0, 0.0, LOGGER_Z / 2.0);
    body - logger_pocket_cuts() + tilt_witness_scale() + logger_cable_clip_lands()
}

fn logger_pocket_cuts() -> Part {
    let mut cuts = Part::empty("closed_media_bag_rocking_logger_pocket_cuts");
    for i in 0..ACCEL_NESTS {
        cuts = cuts
            + centered_cube(
                format!("closed_media_bag_rocking_accelerometer_nest_cut_{i}"),
                72.0,
                44.0,
                22.0,
            )
            .translate(
                centered_index(i, ACCEL_NESTS, 88.0) - 24.0,
                -36.0,
                LOGGER_Z - 10.0,
            );
    }
    for i in 0..TILT_LOGGER_NESTS {
        cuts = cuts
            + centered_cube(
                format!("closed_media_bag_rocking_tilt_logger_nest_cut_{i}"),
                92.0,
                46.0,
                24.0,
            )
            .translate(
                centered_index(i, TILT_LOGGER_NESTS, 116.0),
                42.0,
                LOGGER_Z - 10.0,
            );
    }
    cuts
}

fn tilt_witness_scale() -> Part {
    let mut scale = Part::empty("closed_media_bag_rocking_tilt_witness_scale");
    for i in 0..TILT_TICKS {
        scale = scale
            + centered_cube(
                format!("closed_media_bag_rocking_tilt_scale_tick_{i}"),
                4.0,
                if i == TILT_TICKS / 2 { 34.0 } else { 20.0 },
                5.0,
            )
            .translate(
                centered_index(i, TILT_TICKS, 16.0),
                -LOGGER_Y / 2.0 + 18.0,
                LOGGER_Z + 2.5,
            );
    }
    scale
}

fn logger_cable_clip_lands() -> Part {
    let mut clips = Part::empty("closed_media_bag_rocking_logger_cable_clip_lands");
    for i in 0..5 {
        clips = clips
            + centered_cube(
                format!("closed_media_bag_rocking_logger_cable_clip_land_{i}"),
                32.0,
                12.0,
                8.0,
            )
            .translate(
                centered_index(i, 5, 58.0),
                LOGGER_Y / 2.0 - 14.0,
                LOGGER_Z + 4.0,
            );
    }
    clips
}

fn load_cell_pad_placeholder() -> Part {
    let body = centered_cube(
        "closed_media_bag_rocking_load_cell_pad_placeholder_base",
        LOAD_X,
        LOAD_Y,
        LOAD_Z,
    )
    .translate(0.0, 0.0, LOAD_Z / 2.0);
    let pad = centered_cube(
        "closed_media_bag_rocking_load_cell_active_pad_envelope",
        LOAD_PAD_X,
        LOAD_PAD_Y,
        8.0,
    )
    .translate(0.0, 0.0, LOAD_Z + 4.0);
    let mut datums = Part::empty("closed_media_bag_rocking_load_cell_datum_pads");
    for i in 0..LOAD_DATUM_PADS {
        datums = datums
            + centered_cube(
                format!("closed_media_bag_rocking_load_cell_corner_datum_pad_{i}"),
                30.0,
                22.0,
                6.0,
            )
            .translate(
                if i % 2 == 0 {
                    -LOAD_X / 2.0 + 32.0
                } else {
                    LOAD_X / 2.0 - 32.0
                },
                if i < 2 {
                    -LOAD_Y / 2.0 + 22.0
                } else {
                    LOAD_Y / 2.0 - 22.0
                },
                LOAD_Z + 3.0,
            );
    }
    body + pad + datums
}

fn barcode_status_lanes() -> Part {
    let base = centered_cube(
        "closed_media_bag_rocking_barcode_status_lane_plate",
        STATUS_X,
        STATUS_Y,
        STATUS_Z,
    )
    .translate(0.0, 0.0, STATUS_Z / 2.0);
    base + barcode_lanes() + disposition_status_lanes()
}

fn barcode_lanes() -> Part {
    let mut lanes = Part::empty("closed_media_bag_rocking_barcode_lanes");
    for i in 0..BARCODE_LANES {
        lanes = lanes
            + centered_cube(
                format!("closed_media_bag_rocking_barcode_lane_{i}"),
                54.0,
                24.0,
                4.0,
            )
            .translate(
                centered_index(i, BARCODE_LANES, 60.0),
                -STATUS_Y / 2.0 + 24.0,
                STATUS_Z + 2.0,
            );
    }
    lanes
}

fn disposition_status_lanes() -> Part {
    let mut lanes = Part::empty("closed_media_bag_rocking_status_disposition_lanes");
    for lane in 0..STATUS_LANES {
        let x = centered_index(lane, STATUS_LANES, 120.0);
        lanes = lanes
            + centered_cube(
                format!("closed_media_bag_rocking_status_lane_{lane}"),
                96.0,
                24.0,
                5.0,
            )
            .translate(x, 22.0, STATUS_Z + 2.5);
        for token in 0..STATUS_TOKENS_PER_LANE {
            lanes = lanes
                + centered_cube(
                    format!("closed_media_bag_rocking_status_lane_{lane}_token_slot_{token}"),
                    18.0,
                    18.0,
                    6.0,
                )
                .translate(x - 33.0 + token as f64 * 22.0, 47.0, STATUS_Z + 3.0);
        }
    }
    lanes
}

fn robot_service_keepouts() -> Part {
    let front = centered_cube(
        "closed_media_bag_rocking_front_robot_keepout_gauge",
        STATION_X - 120.0,
        KEEP_OUT_RAIL,
        90.0,
    )
    .translate(0.0, -STATION_Y / 2.0 - FRONT_ROBOT_CLEARANCE, 45.0);
    let rear = centered_cube(
        "closed_media_bag_rocking_rear_service_keepout_gauge",
        STATION_X - 120.0,
        KEEP_OUT_RAIL,
        90.0,
    )
    .translate(0.0, STATION_Y / 2.0 + REAR_SERVICE_CLEARANCE, 45.0);
    let left = centered_cube(
        "closed_media_bag_rocking_left_bag_service_keepout_gauge",
        KEEP_OUT_RAIL,
        STATION_Y - 100.0,
        90.0,
    )
    .translate(-STATION_X / 2.0 - LEFT_BAG_SERVICE_CLEARANCE, 0.0, 45.0);
    let right = centered_cube(
        "closed_media_bag_rocking_right_sample_service_keepout_gauge",
        KEEP_OUT_RAIL,
        STATION_Y - 100.0,
        90.0,
    )
    .translate(STATION_X / 2.0 + RIGHT_SAMPLE_SERVICE_CLEARANCE, 0.0, 45.0);
    let top = centered_cube(
        "closed_media_bag_rocking_top_rocker_lift_keepout_gauge",
        540.0,
        18.0,
        KEEP_OUT_RAIL,
    )
    .translate(
        CRADLE_POS.0,
        CRADLE_POS.1,
        BASE_Z + TOP_ROCKER_LIFT_CLEARANCE,
    );
    front + rear + left + right + top
}

fn run_reference_tokens() -> Part {
    let base = centered_cube(
        "closed_media_bag_rocking_run_reference_token_plate",
        TOKEN_X,
        TOKEN_Y,
        TOKEN_Z,
    )
    .translate(0.0, 0.0, TOKEN_Z / 2.0);
    let mut tokens = Part::empty("closed_media_bag_rocking_run_reference_tokens");
    for i in 0..REFERENCE_TOKENS {
        tokens = tokens
            + centered_cylinder(
                format!("closed_media_bag_rocking_run_reference_token_nest_{i}"),
                13.0,
                8.0,
                28,
            )
            .translate(
                centered_index(i, REFERENCE_TOKENS, 30.0),
                16.0,
                TOKEN_Z + 4.0,
            );
    }
    base + tokens
}

fn module_rects() -> [Rect; 9] {
    [
        Rect {
            name: "bag_cradle",
            center: CRADLE_POS,
            x: CRADLE_X,
            y: CRADLE_Y,
        },
        Rect {
            name: "tubing_comb",
            center: TUBING_POS,
            x: COMB_X,
            y: COMB_Y,
        },
        Rect {
            name: "sample_loops",
            center: SAMPLE_POS,
            x: SAMPLE_X,
            y: SAMPLE_Y,
        },
        Rect {
            name: "tracer_windows",
            center: TRACER_POS,
            x: TRACER_X,
            y: TRACER_Y,
        },
        Rect {
            name: "bubble_witnesses",
            center: BUBBLE_POS,
            x: BUBBLE_X,
            y: BUBBLE_Y,
        },
        Rect {
            name: "logger_nests",
            center: LOGGER_POS,
            x: LOGGER_X,
            y: LOGGER_Y,
        },
        Rect {
            name: "load_cell_pad",
            center: LOAD_POS,
            x: LOAD_X,
            y: LOAD_Y,
        },
        Rect {
            name: "barcode_status_lanes",
            center: STATUS_POS,
            x: STATUS_X,
            y: STATUS_Y,
        },
        Rect {
            name: "run_reference_tokens",
            center: TOKENS_POS,
            x: TOKEN_X,
            y: TOKEN_Y,
        },
    ]
}

fn assert_design_constraints() {
    assert!(STATION_X >= 1300.0);
    assert!(STATION_Y >= 820.0);
    assert!(BAG_POCKET_X > 350.0);
    assert!(ROCKER_LIMIT_DEG <= 12.0);
    assert!(SAMPLE_LEVELS == 3);
    assert!(TUBE_CHANNELS >= 10);
    assert!(TRACER_WINDOWS >= 8);
    assert!(SIGHT_CHANNELS >= 3);
    assert!(ACCEL_NESTS >= 3);
    assert!(BARCODE_LANES >= 6);
    for rect in module_rects() {
        assert!(rect.fits_inside_station(), "{} outside station", rect.name);
    }
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_set_is_deterministic_and_complete() {
        assert_eq!(OUTPUTS.len(), 13);
        assert!(OUTPUTS.iter().all(|path| path
            .starts_with("output/closed_media_bag_rocking_mixing_shear_surrogate_station_")));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
    }

    #[test]
    fn required_station_features_are_named() {
        for feature in REQUIRED_FEATURES {
            assert!(
                OUTPUTS.iter().any(|path| path.contains(feature)),
                "missing output for {feature}"
            );
        }
    }

    #[test]
    fn layout_keeps_modules_inside_secondary_containment() {
        assert_design_constraints();
    }

    #[test]
    fn no_cell_shear_surrogate_has_sampling_and_witness_capacity() {
        assert_eq!(SAMPLE_LEVELS, 3);
        assert!(SAMPLE_LOOPS_PER_LEVEL >= 4);
        assert!(ROCKER_ARC_SEGMENTS >= 11);
        assert!(ROCKER_LIMIT_DEG <= 12.0);
        assert!(TRACER_WINDOWS >= 8);
        assert!(DEGAS_CHAMBERS >= 3);
    }
}
