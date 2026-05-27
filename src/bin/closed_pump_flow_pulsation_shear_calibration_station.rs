use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed pump flow-rate, pulsation, and shear surrogate calibration station.
//
// Intent:
// - Package no-cell validation references for closed tissue-chip pump cartridges:
//   pump cartridge nest, gravimetric collection pads/load-cell placeholders,
//   inline flow sensor coupon slots, pulsation dampener witness chamber, shear
//   surrogate channel coupons, pressure tap manifold, occlusion challenge comb,
//   bubble witness windows, barcode/status lanes, leak tray, and robot/service
//   keepouts.
// - Keep this as validation/interface CAD only. It does not define biological
//   protocols, calibration algorithms, pump settings, or acceptance thresholds.

const OUTPUTS: [&str; 12] = [
    "output/closed_pump_flow_pulsation_shear_calibration_station_leak_tray_base.stl",
    "output/closed_pump_flow_pulsation_shear_calibration_station_pump_cartridge_nest.stl",
    "output/closed_pump_flow_pulsation_shear_calibration_station_gravimetric_load_cell_collection_pads.stl",
    "output/closed_pump_flow_pulsation_shear_calibration_station_inline_flow_sensor_coupon_slots.stl",
    "output/closed_pump_flow_pulsation_shear_calibration_station_pulsation_dampener_witness_chamber.stl",
    "output/closed_pump_flow_pulsation_shear_calibration_station_shear_surrogate_channel_coupons.stl",
    "output/closed_pump_flow_pulsation_shear_calibration_station_pressure_tap_manifold.stl",
    "output/closed_pump_flow_pulsation_shear_calibration_station_occlusion_challenge_comb.stl",
    "output/closed_pump_flow_pulsation_shear_calibration_station_bubble_witness_windows.stl",
    "output/closed_pump_flow_pulsation_shear_calibration_station_barcode_status_lanes.stl",
    "output/closed_pump_flow_pulsation_shear_calibration_station_robot_service_keepouts.stl",
    "output/closed_pump_flow_pulsation_shear_calibration_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 12] = [
    "leak_tray_base",
    "pump_cartridge_nest",
    "gravimetric_load_cell_collection_pads",
    "inline_flow_sensor_coupon_slots",
    "pulsation_dampener_witness_chamber",
    "shear_surrogate_channel_coupons",
    "pressure_tap_manifold",
    "occlusion_challenge_comb",
    "bubble_witness_windows",
    "barcode_status_lanes",
    "robot_service_keepouts",
    "assembly",
];

const STATION_X: f64 = 1280.0;
const STATION_Y: f64 = 780.0;
const BASE_Z: f64 = 24.0;
const CURB_W: f64 = 24.0;
const CURB_Z: f64 = 54.0;
const SUMP_X: f64 = 1130.0;
const SUMP_Y: f64 = 620.0;
const SUMP_DEPTH: f64 = 8.0;
const DRAIN_D: f64 = 18.0;
const MOUNT_HOLES: usize = 6;
const LEAK_WITNESS_WELLS: usize = 8;

const PUMP_POS: (f64, f64) = (-405.0, 196.0);
const PUMP_NEST_X: f64 = 350.0;
const PUMP_NEST_Y: f64 = 230.0;
const PUMP_NEST_Z: f64 = 52.0;
const PUMP_CARTRIDGES: usize = 4;
const PUMP_PITCH_X: f64 = 78.0;
const CARTRIDGE_SLOT_X: f64 = 54.0;
const CARTRIDGE_SLOT_Y: f64 = 142.0;
const CARTRIDGE_SLOT_DEPTH: f64 = 22.0;
const DRIVE_CLEARANCE_WINDOWS: usize = 4;

const GRAV_POS: (f64, f64) = (20.0, 208.0);
const GRAV_X: f64 = 470.0;
const GRAV_Y: f64 = 188.0;
const GRAV_Z: f64 = 34.0;
const COLLECTION_PADS: usize = 6;
const PAD_PITCH_X: f64 = 70.0;
const LOAD_CELL_PAD_X: f64 = 54.0;
const LOAD_CELL_PAD_Y: f64 = 76.0;
const COLLECTION_CUP_D: f64 = 38.0;

const FLOW_POS: (f64, f64) = (440.0, 230.0);
const FLOW_X: f64 = 285.0;
const FLOW_Y: f64 = 136.0;
const FLOW_Z: f64 = 38.0;
const FLOW_SENSOR_COUPONS: usize = 4;
const FLOW_COUPON_X: f64 = 48.0;
const FLOW_COUPON_Y: f64 = 84.0;
const FLOW_COUPON_PITCH_X: f64 = 62.0;
const FLOW_BORE_D: f64 = 7.0;

const PULSE_POS: (f64, f64) = (-420.0, -48.0);
const PULSE_X: f64 = 330.0;
const PULSE_Y: f64 = 172.0;
const PULSE_Z: f64 = 58.0;
const PULSE_CHAMBERS: usize = 3;
const PULSE_WINDOW_X: f64 = 66.0;
const PULSE_WINDOW_Y: f64 = 10.0;
const PULSE_WINDOW_Z: f64 = 38.0;
const PULSE_TICK_COUNT: usize = 9;

const SHEAR_POS: (f64, f64) = (0.0, -52.0);
const SHEAR_X: f64 = 430.0;
const SHEAR_Y: f64 = 188.0;
const SHEAR_Z: f64 = 32.0;
const SHEAR_COUPONS: usize = 6;
const SHEAR_COUPON_X: f64 = 54.0;
const SHEAR_COUPON_Y: f64 = 118.0;
const SHEAR_COUPON_PITCH_X: f64 = 64.0;
const SHEAR_CHANNELS_PER_COUPON: usize = 3;

const PRESSURE_POS: (f64, f64) = (430.0, -54.0);
const PRESSURE_X: f64 = 300.0;
const PRESSURE_Y: f64 = 158.0;
const PRESSURE_Z: f64 = 44.0;
const PRESSURE_TAPS: usize = 8;
const PRESSURE_TAP_D: f64 = 10.0;
const PRESSURE_TAP_PITCH_X: f64 = 32.0;
const MANIFOLD_BORE_D: f64 = 8.0;

const OCCLUSION_POS: (f64, f64) = (-415.0, -258.0);
const OCCLUSION_X: f64 = 340.0;
const OCCLUSION_Y: f64 = 118.0;
const OCCLUSION_Z: f64 = 40.0;
const OCCLUSION_TEETH: usize = 10;
const OCCLUSION_TOOTH_PITCH: f64 = 28.0;
const OCCLUSION_GAP_D: f64 = 9.0;

const BUBBLE_POS: (f64, f64) = (-58.0, -258.0);
const BUBBLE_X: f64 = 360.0;
const BUBBLE_Y: f64 = 118.0;
const BUBBLE_Z: f64 = 40.0;
const BUBBLE_WINDOWS: usize = 5;
const BUBBLE_WINDOW_X: f64 = 50.0;
const BUBBLE_WINDOW_Y: f64 = 12.0;
const BUBBLE_WINDOW_Z: f64 = 26.0;
const BUBBLE_REFERENCE_BEADS: usize = 10;

const STATUS_POS: (f64, f64) = (356.0, -258.0);
const STATUS_X: f64 = 320.0;
const STATUS_Y: f64 = 118.0;
const STATUS_Z: f64 = 18.0;
const BARCODE_LANES: usize = 8;
const STATUS_LANES: usize = 4;
const RUN_TOKEN_WELLS: usize = 8;

const FRONT_ROBOT_CLEARANCE: f64 = 420.0;
const REAR_PUMP_SERVICE_CLEARANCE: f64 = 280.0;
const RIGHT_SENSOR_SERVICE_CLEARANCE: f64 = 210.0;
const LEFT_TUBING_SERVICE_CLEARANCE: f64 = 190.0;
const TOP_CARTRIDGE_LIFT_CLEARANCE: f64 = 260.0;
const KEEP_OUT_RAIL: f64 = 8.0;

#[derive(Clone, Copy)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_tray(self) -> bool {
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

    let pump = pump_cartridge_nest();
    export(OUTPUTS[1], &pump);

    let gravimetric = gravimetric_load_cell_collection_pads();
    export(OUTPUTS[2], &gravimetric);

    let flow = inline_flow_sensor_coupon_slots();
    export(OUTPUTS[3], &flow);

    let pulsation = pulsation_dampener_witness_chamber();
    export(OUTPUTS[4], &pulsation);

    let shear = shear_surrogate_channel_coupons();
    export(OUTPUTS[5], &shear);

    let pressure = pressure_tap_manifold();
    export(OUTPUTS[6], &pressure);

    let occlusion = occlusion_challenge_comb();
    export(OUTPUTS[7], &occlusion);

    let bubbles = bubble_witness_windows();
    export(OUTPUTS[8], &bubbles);

    let status = barcode_status_lanes();
    export(OUTPUTS[9], &status);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let assembly = station_assembly();
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed pump flow/pulsation/shear calibration station:");
    println!(
        "  Leak tray:     {STATION_X:.0}mm x {STATION_Y:.0}mm with {LEAK_WITNESS_WELLS} witness wells, {MOUNT_HOLES} mount holes, and {DRAIN_D:.0}mm drain"
    );
    println!(
        "  Pump nest:     {PUMP_CARTRIDGES} closed-cartridge bays with {DRIVE_CLEARANCE_WINDOWS} drive clearance windows"
    );
    println!(
        "  Flow checks:   {COLLECTION_PADS} gravimetric/load-cell pads and {FLOW_SENSOR_COUPONS} inline flow sensor coupon slots"
    );
    println!(
        "  Dynamics:      {PULSE_CHAMBERS} pulsation dampener witness chambers, {SHEAR_COUPONS} shear surrogate coupons, {PRESSURE_TAPS} pressure taps"
    );
    println!(
        "  Challenges:    {OCCLUSION_TEETH} occlusion comb teeth, {BUBBLE_WINDOWS} bubble witness windows, {BUBBLE_REFERENCE_BEADS} bubble reference beads"
    );
    println!(
        "  Traceability:  {BARCODE_LANES} barcode lanes, {STATUS_LANES} status lanes, {RUN_TOKEN_WELLS} run-token wells"
    );
    println!(
        "  Keepouts:      front robot {FRONT_ROBOT_CLEARANCE:.0}mm, rear pump service {REAR_PUMP_SERVICE_CLEARANCE:.0}mm, right sensor service {RIGHT_SENSOR_SERVICE_CLEARANCE:.0}mm, left tubing service {LEFT_TUBING_SERVICE_CLEARANCE:.0}mm"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    leak_tray_base()
        + pump_cartridge_nest().translate(PUMP_POS.0, PUMP_POS.1, BASE_Z)
        + gravimetric_load_cell_collection_pads().translate(GRAV_POS.0, GRAV_POS.1, BASE_Z)
        + inline_flow_sensor_coupon_slots().translate(FLOW_POS.0, FLOW_POS.1, BASE_Z)
        + pulsation_dampener_witness_chamber().translate(PULSE_POS.0, PULSE_POS.1, BASE_Z)
        + shear_surrogate_channel_coupons().translate(SHEAR_POS.0, SHEAR_POS.1, BASE_Z)
        + pressure_tap_manifold().translate(PRESSURE_POS.0, PRESSURE_POS.1, BASE_Z)
        + occlusion_challenge_comb().translate(OCCLUSION_POS.0, OCCLUSION_POS.1, BASE_Z)
        + bubble_witness_windows().translate(BUBBLE_POS.0, BUBBLE_POS.1, BASE_Z)
        + barcode_status_lanes().translate(STATUS_POS.0, STATUS_POS.1, BASE_Z)
        + robot_service_keepouts()
}

fn leak_tray_base() -> Part {
    let deck = centered_cube(
        "closed_pump_flow_shear_station_leak_tray_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let sump = centered_cube(
        "closed_pump_flow_shear_station_recessed_sump_cut",
        SUMP_X,
        SUMP_Y,
        SUMP_DEPTH + 1.0,
    )
    .translate(0.0, -8.0, BASE_Z - SUMP_DEPTH / 2.0 + 0.5);
    let drain = centered_cylinder(
        "closed_pump_flow_shear_station_front_drain_cut",
        DRAIN_D / 2.0,
        CURB_W + 38.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, -STATION_Y / 2.0 + 13.0, BASE_Z - 6.0);

    deck - sump - drain - mount_holes()
        + tray_curbs()
        + leak_witness_wells()
        + module_floor_markers()
        + datum_targets()
}

fn tray_curbs() -> Part {
    let front = centered_cube(
        "closed_pump_flow_shear_station_front_curb",
        STATION_X,
        CURB_W,
        CURB_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + CURB_W / 2.0, BASE_Z + CURB_Z / 2.0);
    let rear = centered_cube(
        "closed_pump_flow_shear_station_rear_curb",
        STATION_X,
        CURB_W,
        CURB_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - CURB_W / 2.0, BASE_Z + CURB_Z / 2.0);
    let left = centered_cube(
        "closed_pump_flow_shear_station_left_curb",
        CURB_W,
        STATION_Y,
        CURB_Z,
    )
    .translate(-STATION_X / 2.0 + CURB_W / 2.0, 0.0, BASE_Z + CURB_Z / 2.0);
    let right = centered_cube(
        "closed_pump_flow_shear_station_right_curb",
        CURB_W,
        STATION_Y,
        CURB_Z,
    )
    .translate(STATION_X / 2.0 - CURB_W / 2.0, 0.0, BASE_Z + CURB_Z / 2.0);
    front + rear + left + right
}

fn mount_holes() -> Part {
    let mut holes = Part::empty("closed_pump_flow_shear_station_mount_holes");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 66.0, -STATION_Y / 2.0 + 62.0),
        (STATION_X / 2.0 - 66.0, -STATION_Y / 2.0 + 62.0),
        (-STATION_X / 2.0 + 66.0, STATION_Y / 2.0 - 62.0),
        (STATION_X / 2.0 - 66.0, STATION_Y / 2.0 - 62.0),
        (0.0, -STATION_Y / 2.0 + 62.0),
        (0.0, STATION_Y / 2.0 - 62.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("closed_pump_flow_shear_station_mount_hole_{i}"),
                3.4,
                BASE_Z + 4.0,
                28,
            )
            .translate(*x, *y, BASE_Z / 2.0);
    }
    holes
}

fn leak_witness_wells() -> Part {
    let mut wells = Part::empty("closed_pump_flow_shear_station_leak_witness_wells");
    for i in 0..LEAK_WITNESS_WELLS {
        wells = wells
            + centered_cylinder(
                format!("closed_pump_flow_shear_station_leak_witness_well_{i}"),
                13.0,
                7.0,
                30,
            )
            .translate(
                centered_index(i % 4, 4, 72.0),
                -330.0 + (i / 4) as f64 * 34.0,
                BASE_Z + 3.5,
            );
    }
    wells
}

fn module_floor_markers() -> Part {
    let mut markers = Part::empty("closed_pump_flow_shear_station_module_floor_markers");
    for rect in module_rects() {
        markers = markers
            + centered_cube(
                format!("closed_pump_flow_shear_station_{}_floor_marker", rect.name),
                rect.x + 18.0,
                rect.y + 18.0,
                3.0,
            )
            .translate(rect.center.0, rect.center.1, BASE_Z + 1.5);
    }
    markers
}

fn datum_targets() -> Part {
    let mut targets = Part::empty("closed_pump_flow_shear_station_datum_targets");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 98.0, -STATION_Y / 2.0 + 98.0),
        (STATION_X / 2.0 - 98.0, -STATION_Y / 2.0 + 98.0),
        (-STATION_X / 2.0 + 98.0, STATION_Y / 2.0 - 98.0),
        (STATION_X / 2.0 - 98.0, STATION_Y / 2.0 - 98.0),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("closed_pump_flow_shear_station_datum_boss_{i}"),
            16.0,
            7.0,
            36,
        )
        .translate(*x, *y, BASE_Z + 3.5);
        let center = centered_cylinder(
            format!("closed_pump_flow_shear_station_datum_center_cut_{i}"),
            3.5,
            8.0,
            24,
        )
        .translate(*x, *y, BASE_Z + 4.0);
        targets = targets + (boss - center);
    }
    targets
}

fn pump_cartridge_nest() -> Part {
    let body = centered_cube(
        "closed_pump_flow_shear_pump_cartridge_nest_body",
        PUMP_NEST_X,
        PUMP_NEST_Y,
        PUMP_NEST_Z,
    )
    .translate(0.0, 0.0, PUMP_NEST_Z / 2.0);
    let mut slots = Part::empty("closed_pump_flow_shear_pump_cartridge_slot_cuts");
    for i in 0..PUMP_CARTRIDGES {
        slots = slots
            + centered_cube(
                format!("closed_pump_flow_shear_pump_cartridge_slot_cut_{i}"),
                CARTRIDGE_SLOT_X,
                CARTRIDGE_SLOT_Y,
                CARTRIDGE_SLOT_DEPTH + 1.0,
            )
            .translate(
                centered_index(i, PUMP_CARTRIDGES, PUMP_PITCH_X),
                0.0,
                PUMP_NEST_Z - CARTRIDGE_SLOT_DEPTH / 2.0 + 0.5,
            );
    }
    body - slots + cartridge_locator_posts() + drive_clearance_windows() + tubing_exit_keys()
}

fn cartridge_locator_posts() -> Part {
    let mut posts = Part::empty("closed_pump_flow_shear_cartridge_locator_posts");
    for i in 0..PUMP_CARTRIDGES {
        let x = centered_index(i, PUMP_CARTRIDGES, PUMP_PITCH_X);
        for (j, y) in [-82.0, 82.0].iter().enumerate() {
            posts = posts
                + centered_cylinder(
                    format!("closed_pump_flow_shear_cartridge_{i}_locator_post_{j}"),
                    5.0,
                    16.0,
                    24,
                )
                .translate(x, *y, PUMP_NEST_Z + 8.0);
        }
    }
    posts
}

fn drive_clearance_windows() -> Part {
    let mut windows = Part::empty("closed_pump_flow_shear_drive_clearance_windows");
    for i in 0..DRIVE_CLEARANCE_WINDOWS {
        windows = windows
            + centered_cube(
                format!("closed_pump_flow_shear_drive_clearance_window_{i}"),
                42.0,
                12.0,
                30.0,
            )
            .translate(
                centered_index(i, DRIVE_CLEARANCE_WINDOWS, PUMP_PITCH_X),
                -PUMP_NEST_Y / 2.0 - 1.0,
                PUMP_NEST_Z / 2.0,
            );
    }
    windows
}

fn tubing_exit_keys() -> Part {
    let mut keys = Part::empty("closed_pump_flow_shear_tubing_exit_keys");
    for i in 0..PUMP_CARTRIDGES {
        keys = keys
            + centered_cube(
                format!("closed_pump_flow_shear_tubing_exit_key_{i}"),
                36.0,
                16.0,
                10.0,
            )
            .translate(
                centered_index(i, PUMP_CARTRIDGES, PUMP_PITCH_X),
                PUMP_NEST_Y / 2.0 + 8.0,
                PUMP_NEST_Z + 5.0,
            );
    }
    keys
}

fn gravimetric_load_cell_collection_pads() -> Part {
    let base = centered_cube(
        "closed_pump_flow_shear_gravimetric_pad_plate",
        GRAV_X,
        GRAV_Y,
        GRAV_Z,
    )
    .translate(0.0, 0.0, GRAV_Z / 2.0);
    let mut pads = Part::empty("closed_pump_flow_shear_load_cell_collection_pads");
    let mut cup_cuts = Part::empty("closed_pump_flow_shear_collection_cup_recesses");
    for i in 0..COLLECTION_PADS {
        let x = centered_index(i, COLLECTION_PADS, PAD_PITCH_X);
        pads = pads
            + centered_cube(
                format!("closed_pump_flow_shear_load_cell_placeholder_pad_{i}"),
                LOAD_CELL_PAD_X,
                LOAD_CELL_PAD_Y,
                7.0,
            )
            .translate(x, -26.0, GRAV_Z + 3.5);
        cup_cuts = cup_cuts
            + centered_cylinder(
                format!("closed_pump_flow_shear_collection_cup_recess_cut_{i}"),
                COLLECTION_CUP_D / 2.0,
                GRAV_Z + 3.0,
                32,
            )
            .translate(x, 42.0, GRAV_Z / 2.0);
    }
    base - cup_cuts + pads + gravimetric_drip_edges()
}

fn gravimetric_drip_edges() -> Part {
    let front = centered_cube(
        "closed_pump_flow_shear_gravimetric_front_drip_edge",
        GRAV_X - 34.0,
        8.0,
        12.0,
    )
    .translate(0.0, -GRAV_Y / 2.0 + 14.0, GRAV_Z + 6.0);
    let rear = centered_cube(
        "closed_pump_flow_shear_gravimetric_rear_drip_edge",
        GRAV_X - 34.0,
        8.0,
        12.0,
    )
    .translate(0.0, GRAV_Y / 2.0 - 14.0, GRAV_Z + 6.0);
    front + rear
}

fn inline_flow_sensor_coupon_slots() -> Part {
    let body = centered_cube(
        "closed_pump_flow_shear_inline_flow_sensor_slot_block",
        FLOW_X,
        FLOW_Y,
        FLOW_Z,
    )
    .translate(0.0, 0.0, FLOW_Z / 2.0);
    let mut slots = Part::empty("closed_pump_flow_shear_inline_flow_sensor_coupon_slots");
    for i in 0..FLOW_SENSOR_COUPONS {
        let x = centered_index(i, FLOW_SENSOR_COUPONS, FLOW_COUPON_PITCH_X);
        slots = slots
            + centered_cube(
                format!("closed_pump_flow_shear_inline_flow_sensor_coupon_slot_cut_{i}"),
                FLOW_COUPON_X,
                FLOW_COUPON_Y,
                FLOW_Z + 3.0,
            )
            .translate(x, 0.0, FLOW_Z / 2.0);
    }
    body - slots + flow_sensor_bores() + flow_direction_arrows()
}

fn flow_sensor_bores() -> Part {
    let mut bores = Part::empty("closed_pump_flow_shear_flow_sensor_bores");
    for i in 0..FLOW_SENSOR_COUPONS {
        bores = bores
            + centered_cylinder(
                format!("closed_pump_flow_shear_flow_sensor_coupon_bore_{i}"),
                FLOW_BORE_D / 2.0,
                FLOW_COUPON_Y + 22.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(i, FLOW_SENSOR_COUPONS, FLOW_COUPON_PITCH_X),
                0.0,
                FLOW_Z + 8.0,
            );
    }
    bores
}

fn flow_direction_arrows() -> Part {
    let mut arrows = Part::empty("closed_pump_flow_shear_flow_direction_arrow_lands");
    for i in 0..FLOW_SENSOR_COUPONS {
        arrows = arrows
            + centered_cube(
                format!("closed_pump_flow_shear_flow_direction_arrow_land_{i}"),
                32.0,
                10.0,
                5.0,
            )
            .translate(
                centered_index(i, FLOW_SENSOR_COUPONS, FLOW_COUPON_PITCH_X),
                -FLOW_Y / 2.0 - 7.0,
                FLOW_Z + 2.5,
            );
    }
    arrows
}

fn pulsation_dampener_witness_chamber() -> Part {
    let body = centered_cube(
        "closed_pump_flow_shear_pulsation_dampener_witness_body",
        PULSE_X,
        PULSE_Y,
        PULSE_Z,
    )
    .translate(0.0, 0.0, PULSE_Z / 2.0);
    let mut chamber_cuts = Part::empty("closed_pump_flow_shear_pulsation_chamber_cuts");
    for i in 0..PULSE_CHAMBERS {
        chamber_cuts = chamber_cuts
            + centered_cylinder(
                format!("closed_pump_flow_shear_pulsation_chamber_cut_{i}"),
                28.0,
                PULSE_Z + 3.0,
                40,
            )
            .translate(centered_index(i, PULSE_CHAMBERS, 84.0), 0.0, PULSE_Z / 2.0);
    }
    body - chamber_cuts + pulsation_sight_windows() + pulsation_tick_ladder()
}

fn pulsation_sight_windows() -> Part {
    let mut windows = Part::empty("closed_pump_flow_shear_pulsation_sight_windows");
    for i in 0..PULSE_CHAMBERS {
        windows = windows
            + centered_cube(
                format!("closed_pump_flow_shear_pulsation_dampener_witness_window_{i}"),
                PULSE_WINDOW_X,
                PULSE_WINDOW_Y,
                PULSE_WINDOW_Z,
            )
            .translate(
                centered_index(i, PULSE_CHAMBERS, 84.0),
                -PULSE_Y / 2.0 - 5.0,
                PULSE_Z / 2.0 + 6.0,
            );
    }
    windows
}

fn pulsation_tick_ladder() -> Part {
    let mut ticks = Part::empty("closed_pump_flow_shear_pulsation_tick_ladder");
    for chamber in 0..PULSE_CHAMBERS {
        for tick in 0..PULSE_TICK_COUNT {
            ticks = ticks
                + centered_cube(
                    format!("closed_pump_flow_shear_pulse_chamber_{chamber}_tick_{tick}"),
                    3.0,
                    12.0,
                    2.0,
                )
                .translate(
                    centered_index(chamber, PULSE_CHAMBERS, 84.0) - 28.0 + tick as f64 * 7.0,
                    PULSE_Y / 2.0 + 7.0,
                    PULSE_Z + 1.0,
                );
        }
    }
    ticks
}

fn shear_surrogate_channel_coupons() -> Part {
    let rack = centered_cube(
        "closed_pump_flow_shear_surrogate_coupon_rack",
        SHEAR_X,
        SHEAR_Y,
        SHEAR_Z,
    )
    .translate(0.0, 0.0, SHEAR_Z / 2.0);
    let mut coupons = Part::empty("closed_pump_flow_shear_surrogate_channel_coupons");
    let mut slots = Part::empty("closed_pump_flow_shear_surrogate_coupon_slot_cuts");
    for i in 0..SHEAR_COUPONS {
        let x = centered_index(i, SHEAR_COUPONS, SHEAR_COUPON_PITCH_X);
        slots = slots
            + centered_cube(
                format!("closed_pump_flow_shear_surrogate_coupon_slot_cut_{i}"),
                SHEAR_COUPON_X + 4.0,
                SHEAR_COUPON_Y + 4.0,
                SHEAR_Z + 3.0,
            )
            .translate(x, 0.0, SHEAR_Z / 2.0);
        coupons = coupons + shear_coupon(i).translate(x, 0.0, SHEAR_Z + 5.0);
    }
    rack - slots + coupons + shear_coupon_lock_bar()
}

fn shear_coupon(index: usize) -> Part {
    let body = centered_cube(
        format!("closed_pump_flow_shear_surrogate_coupon_{index}_body"),
        SHEAR_COUPON_X,
        SHEAR_COUPON_Y,
        10.0,
    );
    let mut channels = Part::empty(format!(
        "closed_pump_flow_shear_surrogate_coupon_{index}_channels"
    ));
    for lane in 0..SHEAR_CHANNELS_PER_COUPON {
        channels = channels
            + centered_cube(
                format!("closed_pump_flow_shear_coupon_{index}_channel_{lane}_cut"),
                SHEAR_COUPON_X + 2.0,
                3.5,
                4.0,
            )
            .translate(
                0.0,
                centered_index(lane, SHEAR_CHANNELS_PER_COUPON, 22.0),
                2.0,
            );
    }
    body - channels
}

fn shear_coupon_lock_bar() -> Part {
    centered_cube(
        "closed_pump_flow_shear_surrogate_coupon_lock_bar",
        SHEAR_X - 48.0,
        10.0,
        18.0,
    )
    .translate(0.0, SHEAR_Y / 2.0 + 8.0, SHEAR_Z + 9.0)
}

fn pressure_tap_manifold() -> Part {
    let body = centered_cube(
        "closed_pump_flow_shear_pressure_tap_manifold_body",
        PRESSURE_X,
        PRESSURE_Y,
        PRESSURE_Z,
    )
    .translate(0.0, 0.0, PRESSURE_Z / 2.0);
    let main_bore = centered_cylinder(
        "closed_pump_flow_shear_pressure_manifold_common_bore_cut",
        MANIFOLD_BORE_D / 2.0,
        PRESSURE_X + 8.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, PRESSURE_Z / 2.0);

    body - main_bore - pressure_tap_cuts() + pressure_transducer_lands() + pressure_port_labels()
}

fn pressure_tap_cuts() -> Part {
    let mut taps = Part::empty("closed_pump_flow_shear_pressure_tap_cuts");
    for i in 0..PRESSURE_TAPS {
        taps = taps
            + centered_cylinder(
                format!("closed_pump_flow_shear_pressure_tap_cut_{i}"),
                PRESSURE_TAP_D / 2.0,
                PRESSURE_Z + 8.0,
                24,
            )
            .translate(
                centered_index(i, PRESSURE_TAPS, PRESSURE_TAP_PITCH_X),
                0.0,
                PRESSURE_Z / 2.0,
            );
    }
    taps
}

fn pressure_transducer_lands() -> Part {
    let mut lands = Part::empty("closed_pump_flow_shear_pressure_transducer_lands");
    for i in 0..PRESSURE_TAPS {
        lands = lands
            + centered_cube(
                format!("closed_pump_flow_shear_pressure_transducer_land_{i}"),
                24.0,
                30.0,
                7.0,
            )
            .translate(
                centered_index(i, PRESSURE_TAPS, PRESSURE_TAP_PITCH_X),
                42.0,
                PRESSURE_Z + 3.5,
            );
    }
    lands
}

fn pressure_port_labels() -> Part {
    let mut labels = Part::empty("closed_pump_flow_shear_pressure_port_label_lands");
    for i in 0..PRESSURE_TAPS {
        labels = labels
            + centered_cube(
                format!("closed_pump_flow_shear_pressure_port_label_land_{i}"),
                20.0,
                10.0,
                4.0,
            )
            .translate(
                centered_index(i, PRESSURE_TAPS, PRESSURE_TAP_PITCH_X),
                -PRESSURE_Y / 2.0 - 7.0,
                PRESSURE_Z + 2.0,
            );
    }
    labels
}

fn occlusion_challenge_comb() -> Part {
    let spine = centered_cube(
        "closed_pump_flow_shear_occlusion_challenge_comb_spine",
        OCCLUSION_X,
        OCCLUSION_Y,
        OCCLUSION_Z,
    )
    .translate(0.0, 0.0, OCCLUSION_Z / 2.0);
    spine - occlusion_tube_gaps() + occlusion_teeth() + occlusion_index_tabs()
}

fn occlusion_tube_gaps() -> Part {
    let mut gaps = Part::empty("closed_pump_flow_shear_occlusion_tube_gap_cuts");
    for i in 0..OCCLUSION_TEETH {
        gaps = gaps
            + centered_cylinder(
                format!("closed_pump_flow_shear_occlusion_tube_gap_cut_{i}"),
                OCCLUSION_GAP_D / 2.0,
                OCCLUSION_Y + 4.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(i, OCCLUSION_TEETH, OCCLUSION_TOOTH_PITCH),
                0.0,
                OCCLUSION_Z - 10.0,
            );
    }
    gaps
}

fn occlusion_teeth() -> Part {
    let mut teeth = Part::empty("closed_pump_flow_shear_occlusion_challenge_teeth");
    for i in 0..OCCLUSION_TEETH {
        let height = 11.0 + (i % 5) as f64 * 2.0;
        teeth = teeth
            + centered_cube(
                format!("closed_pump_flow_shear_occlusion_challenge_tooth_{i}"),
                12.0,
                34.0,
                height,
            )
            .translate(
                centered_index(i, OCCLUSION_TEETH, OCCLUSION_TOOTH_PITCH),
                -OCCLUSION_Y / 2.0 + 24.0,
                OCCLUSION_Z + height / 2.0,
            );
    }
    teeth
}

fn occlusion_index_tabs() -> Part {
    let mut tabs = Part::empty("closed_pump_flow_shear_occlusion_index_tabs");
    for i in 0..OCCLUSION_TEETH {
        tabs = tabs
            + centered_cube(
                format!("closed_pump_flow_shear_occlusion_index_tab_{i}"),
                16.0,
                9.0,
                4.0,
            )
            .translate(
                centered_index(i, OCCLUSION_TEETH, OCCLUSION_TOOTH_PITCH),
                OCCLUSION_Y / 2.0 + 7.0,
                OCCLUSION_Z + 2.0,
            );
    }
    tabs
}

fn bubble_witness_windows() -> Part {
    let body = centered_cube(
        "closed_pump_flow_shear_bubble_witness_window_block",
        BUBBLE_X,
        BUBBLE_Y,
        BUBBLE_Z,
    )
    .translate(0.0, 0.0, BUBBLE_Z / 2.0);
    body - bubble_window_cuts() + bubble_window_frames() + bubble_reference_beads()
}

fn bubble_window_cuts() -> Part {
    let mut cuts = Part::empty("closed_pump_flow_shear_bubble_window_cuts");
    for i in 0..BUBBLE_WINDOWS {
        cuts = cuts
            + centered_cube(
                format!("closed_pump_flow_shear_bubble_witness_window_cut_{i}"),
                BUBBLE_WINDOW_X,
                BUBBLE_Y + 4.0,
                BUBBLE_WINDOW_Z,
            )
            .translate(
                centered_index(i, BUBBLE_WINDOWS, 62.0),
                0.0,
                BUBBLE_Z / 2.0 + 2.0,
            );
    }
    cuts
}

fn bubble_window_frames() -> Part {
    let mut frames = Part::empty("closed_pump_flow_shear_bubble_window_frames");
    for i in 0..BUBBLE_WINDOWS {
        frames = frames
            + centered_cube(
                format!("closed_pump_flow_shear_bubble_witness_window_frame_{i}"),
                BUBBLE_WINDOW_X + 10.0,
                BUBBLE_WINDOW_Y,
                BUBBLE_WINDOW_Z + 8.0,
            )
            .translate(
                centered_index(i, BUBBLE_WINDOWS, 62.0),
                -BUBBLE_Y / 2.0 - 4.0,
                BUBBLE_Z / 2.0 + 2.0,
            );
    }
    frames
}

fn bubble_reference_beads() -> Part {
    let mut beads = Part::empty("closed_pump_flow_shear_bubble_reference_beads");
    for i in 0..BUBBLE_REFERENCE_BEADS {
        beads = beads
            + centered_cylinder(
                format!("closed_pump_flow_shear_bubble_reference_bead_{i}"),
                4.0 + (i % 5) as f64,
                5.0,
                18,
            )
            .translate(
                centered_index(i, BUBBLE_REFERENCE_BEADS, 28.0),
                BUBBLE_Y / 2.0 + 10.0,
                BUBBLE_Z + 2.5,
            );
    }
    beads
}

fn barcode_status_lanes() -> Part {
    let plate = centered_cube(
        "closed_pump_flow_shear_barcode_status_lane_plate",
        STATUS_X,
        STATUS_Y,
        STATUS_Z,
    )
    .translate(0.0, 0.0, STATUS_Z / 2.0);
    plate + barcode_lanes() + status_lanes() + run_token_wells()
}

fn barcode_lanes() -> Part {
    let mut lanes = Part::empty("closed_pump_flow_shear_barcode_lanes");
    for i in 0..BARCODE_LANES {
        lanes = lanes
            + centered_cube(
                format!("closed_pump_flow_shear_barcode_lane_{i}"),
                26.0,
                58.0,
                4.0,
            )
            .translate(centered_index(i, BARCODE_LANES, 36.0), 18.0, STATUS_Z + 2.0);
    }
    lanes
}

fn status_lanes() -> Part {
    let mut lanes = Part::empty("closed_pump_flow_shear_status_lanes");
    for i in 0..STATUS_LANES {
        lanes = lanes
            + centered_cube(
                format!("closed_pump_flow_shear_status_lane_{i}"),
                56.0,
                18.0,
                5.0,
            )
            .translate(centered_index(i, STATUS_LANES, 70.0), -34.0, STATUS_Z + 2.5);
    }
    lanes
}

fn run_token_wells() -> Part {
    let mut wells = Part::empty("closed_pump_flow_shear_run_token_wells");
    for i in 0..RUN_TOKEN_WELLS {
        wells = wells
            + centered_cylinder(
                format!("closed_pump_flow_shear_run_token_well_{i}"),
                8.0,
                6.0,
                24,
            )
            .translate(
                centered_index(i, RUN_TOKEN_WELLS, 28.0),
                -STATUS_Y / 2.0 + 14.0,
                STATUS_Z + 3.0,
            );
    }
    wells
}

fn robot_service_keepouts() -> Part {
    let front = centered_cube(
        "closed_pump_flow_shear_front_robot_service_keepout",
        STATION_X - 170.0,
        FRONT_ROBOT_CLEARANCE,
        80.0,
    )
    .translate(
        0.0,
        -(STATION_Y / 2.0 + FRONT_ROBOT_CLEARANCE / 2.0),
        BASE_Z + 40.0,
    );
    let rear = centered_cube(
        "closed_pump_flow_shear_rear_pump_cartridge_service_keepout",
        STATION_X - 250.0,
        REAR_PUMP_SERVICE_CLEARANCE,
        90.0,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 + REAR_PUMP_SERVICE_CLEARANCE / 2.0,
        BASE_Z + 45.0,
    );
    let right = centered_cube(
        "closed_pump_flow_shear_right_sensor_service_keepout",
        RIGHT_SENSOR_SERVICE_CLEARANCE,
        STATION_Y - 150.0,
        80.0,
    )
    .translate(
        STATION_X / 2.0 + RIGHT_SENSOR_SERVICE_CLEARANCE / 2.0,
        0.0,
        BASE_Z + 40.0,
    );
    let left = centered_cube(
        "closed_pump_flow_shear_left_tubing_service_keepout",
        LEFT_TUBING_SERVICE_CLEARANCE,
        STATION_Y - 150.0,
        80.0,
    )
    .translate(
        -(STATION_X / 2.0 + LEFT_TUBING_SERVICE_CLEARANCE / 2.0),
        0.0,
        BASE_Z + 40.0,
    );
    let top = centered_cube(
        "closed_pump_flow_shear_top_cartridge_lift_keepout_gauge",
        PUMP_NEST_X + 60.0,
        18.0,
        KEEP_OUT_RAIL,
    )
    .translate(
        PUMP_POS.0,
        PUMP_POS.1,
        BASE_Z + TOP_CARTRIDGE_LIFT_CLEARANCE,
    );
    front + rear + right + left + top
}

fn module_rects() -> [Rect; 9] {
    [
        Rect {
            name: "pump_cartridge_nest",
            center: PUMP_POS,
            x: PUMP_NEST_X,
            y: PUMP_NEST_Y,
        },
        Rect {
            name: "gravimetric_load_cell_collection_pads",
            center: GRAV_POS,
            x: GRAV_X,
            y: GRAV_Y,
        },
        Rect {
            name: "inline_flow_sensor_coupon_slots",
            center: FLOW_POS,
            x: FLOW_X,
            y: FLOW_Y,
        },
        Rect {
            name: "pulsation_dampener_witness_chamber",
            center: PULSE_POS,
            x: PULSE_X,
            y: PULSE_Y,
        },
        Rect {
            name: "shear_surrogate_channel_coupons",
            center: SHEAR_POS,
            x: SHEAR_X,
            y: SHEAR_Y,
        },
        Rect {
            name: "pressure_tap_manifold",
            center: PRESSURE_POS,
            x: PRESSURE_X,
            y: PRESSURE_Y,
        },
        Rect {
            name: "occlusion_challenge_comb",
            center: OCCLUSION_POS,
            x: OCCLUSION_X,
            y: OCCLUSION_Y,
        },
        Rect {
            name: "bubble_witness_windows",
            center: BUBBLE_POS,
            x: BUBBLE_X,
            y: BUBBLE_Y,
        },
        Rect {
            name: "barcode_status_lanes",
            center: STATUS_POS,
            x: STATUS_X,
            y: STATUS_Y,
        },
    ]
}

fn assert_design_constraints() {
    assert!(STATION_X >= 1200.0);
    assert!(STATION_Y >= 720.0);
    assert!(SUMP_X < STATION_X - CURB_W * 2.0);
    assert!(SUMP_Y < STATION_Y - CURB_W * 2.0);
    assert_eq!(PUMP_CARTRIDGES, FLOW_SENSOR_COUPONS);
    assert!(COLLECTION_PADS >= PUMP_CARTRIDGES);
    assert!(SHEAR_COUPONS >= COLLECTION_PADS);
    assert!(PRESSURE_TAPS >= PUMP_CARTRIDGES * 2);
    assert!(OCCLUSION_TEETH >= 8);
    assert!(BUBBLE_WINDOWS >= 5);
    assert!(BARCODE_LANES >= PRESSURE_TAPS);
    for rect in module_rects() {
        assert!(rect.fits_inside_tray(), "{} outside leak tray", rect.name);
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
    fn output_names_are_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        for path in OUTPUTS {
            assert!(
                path.starts_with("output/closed_pump_flow_pulsation_shear_calibration_station_")
            );
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn required_feature_coverage_is_visible_in_outputs() {
        for feature in REQUIRED_FEATURES {
            assert!(
                OUTPUTS.iter().any(|path| path.contains(feature)),
                "missing output for {feature}"
            );
        }
    }

    #[test]
    fn modules_fit_inside_secondary_containment_bounds() {
        assert_design_constraints();
        for rect in module_rects() {
            assert!(rect.fits_inside_tray(), "{} outside leak tray", rect.name);
        }
    }

    #[test]
    fn validation_station_counts_cover_requested_fixture_scope() {
        assert_eq!(PUMP_CARTRIDGES, 4);
        assert_eq!(FLOW_SENSOR_COUPONS, 4);
        assert_eq!(COLLECTION_PADS, 6);
        assert_eq!(PULSE_CHAMBERS, 3);
        assert_eq!(SHEAR_COUPONS, 6);
        assert_eq!(SHEAR_CHANNELS_PER_COUPON, 3);
        assert_eq!(PRESSURE_TAPS, 8);
        assert_eq!(OCCLUSION_TEETH, 10);
        assert_eq!(BUBBLE_WINDOWS, 5);
    }

    #[test]
    fn robot_and_service_keepouts_are_explicit() {
        assert!(FRONT_ROBOT_CLEARANCE >= 400.0);
        assert!(REAR_PUMP_SERVICE_CLEARANCE >= 260.0);
        assert!(RIGHT_SENSOR_SERVICE_CLEARANCE >= 200.0);
        assert!(LEFT_TUBING_SERVICE_CLEARANCE >= 180.0);
        assert!(TOP_CARTRIDGE_LIFT_CLEARANCE >= 240.0);
    }
}
