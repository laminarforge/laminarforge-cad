use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Incubated cassette storage/recovery rack for sealed 20-chip culture cassettes.
//
// This is architectural workflow geometry for planning an incubator-resident
// storage module. It reserves cassette envelopes, humid CO2 airflow gaps,
// condensate/leak handling, traceability lands, logger pockets, mapping points,
// sealed tray docking, and robot/service clearances. It is not a validated
// incubator, sterility, or cell-culture process.

const OUTPUTS: [&str; 11] = [
    "output/cassette_storage_recovery_incubator_rack_leak_tray_base.stl",
    "output/cassette_storage_recovery_incubator_rack_upright_frame.stl",
    "output/cassette_storage_recovery_incubator_rack_multi_cassette_slot_rails.stl",
    "output/cassette_storage_recovery_incubator_rack_airflow_thermal_spacing.stl",
    "output/cassette_storage_recovery_incubator_rack_condensate_drip_controls.stl",
    "output/cassette_storage_recovery_incubator_rack_barcode_position_lands.stl",
    "output/cassette_storage_recovery_incubator_rack_sealed_transfer_tray_interface.stl",
    "output/cassette_storage_recovery_incubator_rack_environmental_logger_pockets.stl",
    "output/cassette_storage_recovery_incubator_rack_edge_center_mapping_points.stl",
    "output/cassette_storage_recovery_incubator_rack_robot_service_keepouts.stl",
    "output/cassette_storage_recovery_incubator_rack_assembly.stl",
];

const CASSETTE_COLS: usize = 4;
const CASSETTE_ROWS: usize = 5;
const CASSETTE_POSITION_COUNT: usize = CASSETTE_COLS * CASSETTE_ROWS;
const STORAGE_SLOT_COUNT: usize = 6;
const ENV_LOGGER_POCKET_COUNT: usize = 4;
const MAP_LEVEL_COUNT: usize = 3;
const EDGE_MAPPING_POINTS_PER_LEVEL: usize = 8;
const CENTER_MAPPING_POINTS_PER_LEVEL: usize = 1;
const MAPPING_POINTS_PER_LEVEL: usize =
    EDGE_MAPPING_POINTS_PER_LEVEL + CENTER_MAPPING_POINTS_PER_LEVEL;

const CHIP_GUTTER: f64 = 5.0;
const CASSETTE_MARGIN_X: f64 = 34.0;
const CASSETTE_MARGIN_Y: f64 = 34.0;
const CASSETTE_Z: f64 = 46.0;

const ARRAY_X: f64 =
    CASSETTE_COLS as f64 * REVC_CHIP_LENGTH + (CASSETTE_COLS as f64 - 1.0) * CHIP_GUTTER;
const ARRAY_Y: f64 =
    CASSETTE_ROWS as f64 * REVC_CHIP_WIDTH + (CASSETTE_ROWS as f64 - 1.0) * CHIP_GUTTER;
const CASSETTE_X: f64 = ARRAY_X + 2.0 * CASSETTE_MARGIN_X;
const CASSETTE_Y: f64 = ARRAY_Y + 2.0 * CASSETTE_MARGIN_Y;

const FRAME_W: f64 = 24.0;
const BASE_Z: f64 = 32.0;
const SLOT_CENTER_Y: f64 = -24.0;
const SLOT_FLOOR_Z0: f64 = BASE_Z + 34.0;
const SLOT_PITCH_Z: f64 = 82.0;
const SLOT_AIR_GAP_Z: f64 = SLOT_PITCH_Z - CASSETTE_Z;
const TOP_SERVICE_Z: f64 = 52.0;
const SIDE_AIR_GAP_X: f64 = 38.0;
const FRONT_TRANSFER_CLEARANCE_Y: f64 = 96.0;
const REAR_PLENUM_Y: f64 = 72.0;

const RACK_X: f64 = CASSETTE_X + 2.0 * SIDE_AIR_GAP_X + 2.0 * FRAME_W + 20.0;
const RACK_Y: f64 = CASSETTE_Y + FRONT_TRANSFER_CLEARANCE_Y + REAR_PLENUM_Y + 2.0 * FRAME_W + 40.0;
const RACK_Z: f64 =
    SLOT_FLOOR_Z0 + (STORAGE_SLOT_COUNT as f64 - 1.0) * SLOT_PITCH_Z + CASSETTE_Z + TOP_SERVICE_Z;

const SLOT_RAIL_W: f64 = 16.0;
const SLOT_RAIL_Z: f64 = 18.0;
const SHELF_LEDGE_W: f64 = 20.0;
const SHELF_LEDGE_Z: f64 = 8.0;
const DATUM_PIN_D: f64 = 8.0;
const SOFT_RAIL_Z_SCALE: f64 = 0.62;

const LEAK_BASIN_DEPTH: f64 = 14.0;
const LEAK_TRAY_RIM_Z: f64 = 26.0;
const DRAIN_PORT_D: f64 = 9.0;
const CONDENSATE_GUTTER_W: f64 = 12.0;
const CONDENSATE_GUTTER_Z: f64 = 8.0;
const DOWNSPOUT_W: f64 = 14.0;
const DRIP_SHIELD_Z: f64 = 4.0;

const TRANSFER_TONGUE_X: f64 = CASSETTE_X + 92.0;
const TRANSFER_TONGUE_Y: f64 = 42.0;
const TRANSFER_TONGUE_Z: f64 = 28.0;
const TRANSFER_GASKET_Z: f64 = 6.0;
const TRANSFER_PIN_D: f64 = 12.0;

const LOGGER_POCKET_X: f64 = 92.0;
const LOGGER_POCKET_Y: f64 = 48.0;
const LOGGER_POCKET_Z: f64 = 20.0;
const LOGGER_POCKET_RECESS_X: f64 = 72.0;
const LOGGER_POCKET_RECESS_Y: f64 = 34.0;
const LOGGER_RECESS_DEPTH: f64 = 8.0;
const LOGGER_BANK_PITCH_Z: f64 = 104.0;

const BARCODE_LAND_X: f64 = 150.0;
const BARCODE_LAND_Y: f64 = 5.0;
const BARCODE_LAND_Z: f64 = 32.0;
const POSITION_LAND_X: f64 = 18.0;
const POSITION_LAND_Y: f64 = 7.0;
const POSITION_LAND_Z: f64 = 2.0;

const FRONT_ROBOT_KEEP_OUT_Y: f64 = 280.0;
const SIDE_ROBOT_KEEP_OUT_X: f64 = 70.0;
const REAR_SERVICE_CLEARANCE_Y: f64 = 190.0;
const ROBOT_VERTICAL_CLEARANCE_Z: f64 = CASSETTE_Z + 42.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = leak_tray_base();
    export(OUTPUTS[0], &base);

    let frame = upright_frame();
    export(OUTPUTS[1], &frame);

    let slots = multi_cassette_slot_rails();
    export(OUTPUTS[2], &slots);

    let airflow = airflow_thermal_spacing_features();
    export(OUTPUTS[3], &airflow);

    let drip = condensate_drip_controls();
    export(OUTPUTS[4], &drip);

    let labels = barcode_position_lands();
    export(OUTPUTS[5], &labels);

    let transfer = sealed_transfer_tray_interface();
    export(OUTPUTS[6], &transfer);

    let loggers = environmental_logger_pockets();
    export(OUTPUTS[7], &loggers);

    let mapping = edge_center_mapping_points();
    export(OUTPUTS[8], &mapping);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[9], &keepouts);

    let assembly =
        base + frame + slots + airflow + drip + labels + transfer + loggers + mapping + keepouts;
    export(OUTPUTS[10], &assembly);

    println!();
    println!("Cassette storage/recovery incubator rack:");
    println!("  Rack envelope:               {RACK_X:.1}mm W x {RACK_Y:.1}mm D x {RACK_Z:.1}mm H");
    println!(
        "  Storage capacity:            {STORAGE_SLOT_COUNT} sealed cassette slots, each {CASSETTE_X:.1}mm x {CASSETTE_Y:.1}mm x {CASSETTE_Z:.1}mm"
    );
    println!(
        "  Cassette basis:              {CASSETTE_COLS}x{CASSETTE_ROWS} Rev C chip envelope, {CASSETTE_POSITION_COUNT} position lands per slot"
    );
    println!(
        "  Incubation spacing:          {SLOT_AIR_GAP_Z:.1}mm vertical humid-air gap between slots, {SIDE_AIR_GAP_X:.1}mm side air chimneys"
    );
    println!(
        "  Traceability/logger mapping: {STORAGE_SLOT_COUNT} barcode lands, {} position lands, {ENV_LOGGER_POCKET_COUNT} logger pockets, {} edge/center map targets",
        total_position_lands(),
        total_mapping_points()
    );
    println!(
        "  Transfer/service:            {TRANSFER_TONGUE_X:.1}mm sealed tray tongue, {FRONT_ROBOT_KEEP_OUT_Y:.1}mm front robot keepout, {REAR_SERVICE_CLEARANCE_Y:.1}mm rear service keepout"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_design_constraints() {
    assert_eq!(CASSETTE_POSITION_COUNT, 20);
    assert_eq!(STORAGE_SLOT_COUNT, 6);
    assert!(CASSETTE_Z > REVC_TOTAL_HEIGHT + 24.0);
    assert!(SLOT_AIR_GAP_Z >= 32.0);
    assert!(SIDE_AIR_GAP_X >= 36.0);
    assert!(front_transfer_clearance() >= FRONT_TRANSFER_CLEARANCE_Y - 4.0);
    assert!(rear_plenum_clearance() >= REAR_PLENUM_Y + 18.0);
    assert!(last_slot_top_z() + TOP_SERVICE_Z <= RACK_Z + 0.01);
    assert!(drain_port_y() < leak_basin_front_y());
    assert!(total_position_lands() == STORAGE_SLOT_COUNT * CASSETTE_POSITION_COUNT);
    assert_eq!(
        total_mapping_points(),
        MAP_LEVEL_COUNT * MAPPING_POINTS_PER_LEVEL
    );
    assert_eq!(
        edge_mapping_point_count(),
        MAP_LEVEL_COUNT * EDGE_MAPPING_POINTS_PER_LEVEL
    );
    assert_eq!(
        center_mapping_point_count(),
        MAP_LEVEL_COUNT * CENTER_MAPPING_POINTS_PER_LEVEL
    );
}

fn leak_tray_base() -> Part {
    let pan = centered_cube(
        "cassette_storage_recovery_leak_tray_pan",
        RACK_X,
        RACK_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);

    let basin = centered_cube(
        "cassette_storage_recovery_recessed_leak_basin",
        CASSETTE_X + 112.0,
        CASSETTE_Y + 88.0,
        LEAK_BASIN_DEPTH + 0.4,
    )
    .translate(0.0, SLOT_CENTER_Y, BASE_Z - LEAK_BASIN_DEPTH / 2.0 + 0.2);

    let front_gutter = centered_cube(
        "cassette_storage_recovery_front_leak_gutter_cut",
        RACK_X - 110.0,
        14.0,
        8.0,
    )
    .translate(0.0, leak_basin_front_y() - 18.0, BASE_Z - 4.0);
    let left_gutter = centered_cube(
        "cassette_storage_recovery_left_leak_gutter_cut",
        12.0,
        CASSETTE_Y + 90.0,
        8.0,
    )
    .translate(-(CASSETTE_X / 2.0 + 38.0), SLOT_CENTER_Y, BASE_Z - 4.0);
    let right_gutter = centered_cube(
        "cassette_storage_recovery_right_leak_gutter_cut",
        12.0,
        CASSETTE_Y + 90.0,
        8.0,
    )
    .translate(CASSETTE_X / 2.0 + 38.0, SLOT_CENTER_Y, BASE_Z - 4.0);

    let sump = centered_cube(
        "cassette_storage_recovery_leak_sump_cut",
        84.0,
        54.0,
        LEAK_BASIN_DEPTH + 2.0,
    )
    .translate(
        RACK_X / 2.0 - 72.0,
        leak_basin_front_y() - 32.0,
        BASE_Z - LEAK_BASIN_DEPTH / 2.0,
    );

    let drain = centered_cylinder(
        "cassette_storage_recovery_forward_drain_port_cut",
        DRAIN_PORT_D / 2.0,
        42.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(RACK_X / 2.0 - 72.0, drain_port_y(), BASE_Z - 9.0);

    pan - basin - front_gutter - left_gutter - right_gutter - sump - drain - base_mount_holes()
        + leak_tray_rims()
        + leak_sensor_lands()
        + absorbent_pad_retainers()
}

fn base_mount_holes() -> Part {
    let mut holes = Part::empty("cassette_storage_recovery_base_mount_holes");
    for (i, (x, y)) in [
        (-(RACK_X / 2.0 - 36.0), -(RACK_Y / 2.0 - 36.0)),
        (RACK_X / 2.0 - 36.0, -(RACK_Y / 2.0 - 36.0)),
        (-(RACK_X / 2.0 - 36.0), RACK_Y / 2.0 - 36.0),
        (RACK_X / 2.0 - 36.0, RACK_Y / 2.0 - 36.0),
        (0.0, -(RACK_Y / 2.0 - 36.0)),
        (0.0, RACK_Y / 2.0 - 36.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("cassette_storage_recovery_m5_base_clearance_{i}"),
                5.4 / 2.0,
                BASE_Z + 4.0,
                24,
            )
            .translate(*x, *y, BASE_Z / 2.0);
    }
    holes
}

fn leak_tray_rims() -> Part {
    let front = centered_cube(
        "cassette_storage_recovery_front_low_leak_rim",
        RACK_X,
        FRAME_W,
        LEAK_TRAY_RIM_Z,
    )
    .translate(
        0.0,
        rack_front_y() + FRAME_W / 2.0,
        BASE_Z + LEAK_TRAY_RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "cassette_storage_recovery_rear_leak_rim",
        RACK_X,
        FRAME_W,
        LEAK_TRAY_RIM_Z,
    )
    .translate(
        0.0,
        rack_rear_y() - FRAME_W / 2.0,
        BASE_Z + LEAK_TRAY_RIM_Z / 2.0,
    );
    let left = centered_cube(
        "cassette_storage_recovery_left_leak_rim",
        FRAME_W,
        RACK_Y,
        LEAK_TRAY_RIM_Z,
    )
    .translate(
        -(RACK_X / 2.0 - FRAME_W / 2.0),
        0.0,
        BASE_Z + LEAK_TRAY_RIM_Z / 2.0,
    );
    let right = centered_cube(
        "cassette_storage_recovery_right_leak_rim",
        FRAME_W,
        RACK_Y,
        LEAK_TRAY_RIM_Z,
    )
    .translate(
        RACK_X / 2.0 - FRAME_W / 2.0,
        0.0,
        BASE_Z + LEAK_TRAY_RIM_Z / 2.0,
    );

    front + rear + left + right
}

fn leak_sensor_lands() -> Part {
    let mut lands = Part::empty("cassette_storage_recovery_leak_sensor_lands");
    for (i, x) in [-220.0, 0.0, 220.0].iter().enumerate() {
        lands = lands
            + centered_cube(
                format!("cassette_storage_recovery_leak_sensor_land_{i}"),
                54.0,
                30.0,
                4.0,
            )
            .translate(*x, leak_basin_front_y() + 22.0, BASE_Z + 2.0)
            + centered_cube(
                format!("cassette_storage_recovery_leak_sensor_cable_relief_{i}"),
                9.0,
                42.0,
                5.0,
            )
            .translate(*x + 32.0, leak_basin_front_y() + 8.0, BASE_Z + 2.5);
    }
    lands
}

fn absorbent_pad_retainers() -> Part {
    let front = centered_cube(
        "cassette_storage_recovery_absorbent_pad_front_retainer",
        CASSETTE_X + 72.0,
        8.0,
        12.0,
    )
    .translate(0.0, leak_basin_front_y() + 62.0, BASE_Z + 6.0);
    let rear = centered_cube(
        "cassette_storage_recovery_absorbent_pad_rear_retainer",
        CASSETTE_X + 72.0,
        8.0,
        12.0,
    )
    .translate(0.0, slot_rear_edge() + 36.0, BASE_Z + 6.0);

    front + rear
}

fn upright_frame() -> Part {
    let mut frame = Part::empty("cassette_storage_recovery_upright_frame");
    for (i, (x, y)) in frame_post_positions().iter().enumerate() {
        frame = frame
            + centered_cube(
                format!("cassette_storage_recovery_upright_post_{i}"),
                FRAME_W,
                FRAME_W,
                RACK_Z - BASE_Z,
            )
            .translate(*x, *y, BASE_Z + (RACK_Z - BASE_Z) / 2.0);
    }

    for (i, z) in [BASE_Z + 30.0, RACK_Z - 28.0].iter().enumerate() {
        frame = frame
            + centered_cube(
                format!("cassette_storage_recovery_front_cross_rail_{i}"),
                RACK_X,
                FRAME_W,
                FRAME_W,
            )
            .translate(0.0, rack_front_y() + FRAME_W / 2.0, *z)
            + centered_cube(
                format!("cassette_storage_recovery_rear_cross_rail_{i}"),
                RACK_X,
                FRAME_W,
                FRAME_W,
            )
            .translate(0.0, rack_rear_y() - FRAME_W / 2.0, *z)
            + centered_cube(
                format!("cassette_storage_recovery_left_side_cross_rail_{i}"),
                FRAME_W,
                RACK_Y,
                FRAME_W,
            )
            .translate(-(RACK_X / 2.0 - FRAME_W / 2.0), 0.0, *z)
            + centered_cube(
                format!("cassette_storage_recovery_right_side_cross_rail_{i}"),
                FRAME_W,
                RACK_Y,
                FRAME_W,
            )
            .translate(RACK_X / 2.0 - FRAME_W / 2.0, 0.0, *z);
    }

    frame + slot_level_side_ties()
}

fn slot_level_side_ties() -> Part {
    let mut ties = Part::empty("cassette_storage_recovery_slot_level_side_ties");
    for slot in 0..STORAGE_SLOT_COUNT {
        let z = slot_floor_z(slot) + SLOT_RAIL_Z + 5.0;
        ties = ties
            + centered_cube(
                format!("cassette_storage_recovery_left_slot_tie_{slot}"),
                FRAME_W,
                CASSETTE_Y + 112.0,
                14.0,
            )
            .translate(-(RACK_X / 2.0 - FRAME_W / 2.0), SLOT_CENTER_Y, z)
            + centered_cube(
                format!("cassette_storage_recovery_right_slot_tie_{slot}"),
                FRAME_W,
                CASSETTE_Y + 112.0,
                14.0,
            )
            .translate(RACK_X / 2.0 - FRAME_W / 2.0, SLOT_CENTER_Y, z);
    }
    ties
}

fn multi_cassette_slot_rails() -> Part {
    let mut slots = Part::empty("cassette_storage_recovery_multi_cassette_slot_rails");
    for slot in 0..STORAGE_SLOT_COUNT {
        slots = slots + single_slot_rails(slot);
    }
    slots
}

fn single_slot_rails(slot: usize) -> Part {
    let floor_z = slot_floor_z(slot);
    let rail_z = floor_z + SLOT_RAIL_Z / 2.0;

    let left_datum = centered_cube(
        format!("cassette_storage_recovery_slot_{slot}_left_x_datum_rail"),
        SLOT_RAIL_W,
        CASSETTE_Y + 28.0,
        SLOT_RAIL_Z,
    )
    .translate(
        -(CASSETTE_X / 2.0 + SLOT_RAIL_W / 2.0 + 3.0),
        SLOT_CENTER_Y,
        rail_z,
    );
    let rear_datum = centered_cube(
        format!("cassette_storage_recovery_slot_{slot}_rear_y_datum_rail"),
        CASSETTE_X + 42.0,
        SLOT_RAIL_W,
        SLOT_RAIL_Z,
    )
    .translate(0.0, slot_rear_edge() + SLOT_RAIL_W / 2.0 + 3.0, rail_z);
    let right_soft_rail = centered_cube(
        format!("cassette_storage_recovery_slot_{slot}_right_soft_preload_rail"),
        SLOT_RAIL_W,
        CASSETTE_Y + 22.0,
        SLOT_RAIL_Z * SOFT_RAIL_Z_SCALE,
    )
    .translate(
        CASSETTE_X / 2.0 + SLOT_RAIL_W / 2.0 + 3.0,
        SLOT_CENTER_Y,
        floor_z + SLOT_RAIL_Z * SOFT_RAIL_Z_SCALE / 2.0,
    );
    let front_low_lip = centered_cube(
        format!("cassette_storage_recovery_slot_{slot}_front_low_loading_lip"),
        CASSETTE_X + 42.0,
        10.0,
        SLOT_RAIL_Z * 0.55,
    )
    .translate(0.0, slot_front_edge() - 6.0, floor_z + SLOT_RAIL_Z * 0.275);

    let left_ledge = centered_cube(
        format!("cassette_storage_recovery_slot_{slot}_left_slide_ledge"),
        SHELF_LEDGE_W,
        CASSETTE_Y + 16.0,
        SHELF_LEDGE_Z,
    )
    .translate(
        -(CASSETTE_X / 2.0 - 28.0),
        SLOT_CENTER_Y,
        floor_z + SHELF_LEDGE_Z / 2.0,
    );
    let right_ledge = centered_cube(
        format!("cassette_storage_recovery_slot_{slot}_right_slide_ledge"),
        SHELF_LEDGE_W,
        CASSETTE_Y + 16.0,
        SHELF_LEDGE_Z,
    )
    .translate(
        CASSETTE_X / 2.0 - 28.0,
        SLOT_CENTER_Y,
        floor_z + SHELF_LEDGE_Z / 2.0,
    );
    let center_sacrificial_wear_strip = centered_cube(
        format!("cassette_storage_recovery_slot_{slot}_center_sacrificial_wear_strip"),
        CASSETTE_X - 112.0,
        9.0,
        4.0,
    )
    .translate(0.0, SLOT_CENTER_Y, floor_z + 2.0);

    left_datum
        + rear_datum
        + right_soft_rail
        + front_low_lip
        + left_ledge
        + right_ledge
        + center_sacrificial_wear_strip
        + slot_datum_pin_bosses(slot)
        + slot_recovery_pull_reliefs(slot)
}

fn slot_datum_pin_bosses(slot: usize) -> Part {
    let mut bosses = Part::empty(format!(
        "cassette_storage_recovery_slot_{slot}_datum_pin_bosses"
    ));
    let z = slot_floor_z(slot) + SLOT_RAIL_Z + 5.0;
    for (i, (x, y)) in [
        (
            -(CASSETTE_X / 2.0 - 34.0),
            slot_rear_edge() + SLOT_RAIL_W + 9.0,
        ),
        (
            CASSETTE_X / 2.0 - 34.0,
            slot_rear_edge() + SLOT_RAIL_W + 9.0,
        ),
        (-(CASSETTE_X / 2.0 - 34.0), slot_front_edge() - SLOT_RAIL_W),
        (CASSETTE_X / 2.0 - 34.0, slot_front_edge() - SLOT_RAIL_W),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("cassette_storage_recovery_slot_{slot}_datum_boss_{i}"),
            DATUM_PIN_D,
            10.0,
            32,
        )
        .translate(*x, *y, z);
        let pilot = centered_cylinder(
            format!("cassette_storage_recovery_slot_{slot}_datum_pin_relief_{i}"),
            2.0,
            12.0,
            20,
        )
        .translate(*x, *y, z);
        bosses = bosses + (boss - pilot);
    }
    bosses
}

fn slot_recovery_pull_reliefs(slot: usize) -> Part {
    let z = slot_floor_z(slot) + SLOT_RAIL_Z / 2.0;
    let left = centered_cube(
        format!("cassette_storage_recovery_slot_{slot}_left_recovery_finger_land"),
        56.0,
        16.0,
        SLOT_RAIL_Z,
    )
    .translate(-118.0, slot_front_edge() - 20.0, z);
    let right = centered_cube(
        format!("cassette_storage_recovery_slot_{slot}_right_recovery_finger_land"),
        56.0,
        16.0,
        SLOT_RAIL_Z,
    )
    .translate(118.0, slot_front_edge() - 20.0, z);

    left + right
}

fn airflow_thermal_spacing_features() -> Part {
    rear_diffuser_plenum()
        + side_air_chimneys()
        + slot_air_gap_spacers()
        + thermal_buffer_strips()
        + top_recovery_baffle()
}

fn rear_diffuser_plenum() -> Part {
    let plenum_z = RACK_Z - BASE_Z - 44.0;
    let plenum_y = rack_rear_y() - FRAME_W - REAR_PLENUM_Y / 2.0;
    let body = centered_cube(
        "cassette_storage_recovery_rear_laminar_diffuser_plenum",
        RACK_X - 112.0,
        REAR_PLENUM_Y,
        plenum_z,
    )
    .translate(0.0, plenum_y, BASE_Z + plenum_z / 2.0);

    let mut vents = Part::empty("cassette_storage_recovery_rear_plenum_slot_vents");
    for slot in 0..STORAGE_SLOT_COUNT {
        vents = vents
            + centered_cube(
                format!("cassette_storage_recovery_rear_plenum_slot_{slot}_vent_window"),
                CASSETTE_X + 24.0,
                REAR_PLENUM_Y + 4.0,
                26.0,
            )
            .translate(0.0, plenum_y, slot_cassette_center_z(slot));
    }

    let inlet_stub = centered_cylinder(
        "cassette_storage_recovery_rear_plenum_hepa_supply_stub",
        34.0,
        42.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-RACK_X / 2.0 + 104.0, rack_rear_y() + 6.0, RACK_Z - 92.0);
    let return_stub = centered_cylinder(
        "cassette_storage_recovery_rear_plenum_return_stub",
        26.0,
        42.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(RACK_X / 2.0 - 104.0, rack_rear_y() + 6.0, BASE_Z + 98.0);

    body - vents + inlet_stub + return_stub
}

fn side_air_chimneys() -> Part {
    let chimney_z = RACK_Z - BASE_Z - 72.0;
    let y = SLOT_CENTER_Y;
    let z = BASE_Z + chimney_z / 2.0 + 22.0;
    let left = centered_cube(
        "cassette_storage_recovery_left_side_humid_air_chimney",
        16.0,
        CASSETTE_Y + 56.0,
        chimney_z,
    )
    .translate(-(CASSETTE_X / 2.0 + SIDE_AIR_GAP_X / 2.0), y, z);
    let right = centered_cube(
        "cassette_storage_recovery_right_side_humid_air_chimney",
        16.0,
        CASSETTE_Y + 56.0,
        chimney_z,
    )
    .translate(CASSETTE_X / 2.0 + SIDE_AIR_GAP_X / 2.0, y, z);

    let mut windows = Part::empty("cassette_storage_recovery_side_chimney_slot_windows");
    for slot in 0..STORAGE_SLOT_COUNT {
        windows = windows
            + centered_cube(
                format!("cassette_storage_recovery_left_air_chimney_window_{slot}"),
                20.0,
                CASSETTE_Y - 70.0,
                24.0,
            )
            .translate(
                -(CASSETTE_X / 2.0 + SIDE_AIR_GAP_X / 2.0),
                y,
                slot_cassette_center_z(slot),
            )
            + centered_cube(
                format!("cassette_storage_recovery_right_air_chimney_window_{slot}"),
                20.0,
                CASSETTE_Y - 70.0,
                24.0,
            )
            .translate(
                CASSETTE_X / 2.0 + SIDE_AIR_GAP_X / 2.0,
                y,
                slot_cassette_center_z(slot),
            );
    }

    left + right - windows
}

fn slot_air_gap_spacers() -> Part {
    let mut spacers = Part::empty("cassette_storage_recovery_air_gap_spacer_combs");
    for slot in 0..STORAGE_SLOT_COUNT - 1 {
        let z = slot_floor_z(slot) + CASSETTE_Z + SLOT_AIR_GAP_Z / 2.0;
        spacers = spacers
            + centered_cube(
                format!("cassette_storage_recovery_slot_{slot}_front_air_gap_gauge"),
                CASSETTE_X + 38.0,
                8.0,
                SLOT_AIR_GAP_Z - 8.0,
            )
            .translate(0.0, slot_front_edge() - 26.0, z)
            + centered_cube(
                format!("cassette_storage_recovery_slot_{slot}_rear_air_gap_gauge"),
                CASSETTE_X + 38.0,
                8.0,
                SLOT_AIR_GAP_Z - 8.0,
            )
            .translate(0.0, slot_rear_edge() + 28.0, z);
    }
    spacers
}

fn thermal_buffer_strips() -> Part {
    let mut strips = Part::empty("cassette_storage_recovery_thermal_buffer_strips");
    for slot in 0..STORAGE_SLOT_COUNT {
        let z = slot_floor_z(slot) + SHELF_LEDGE_Z + 3.0;
        strips = strips
            + centered_cube(
                format!("cassette_storage_recovery_slot_{slot}_left_thermal_buffer_strip"),
                28.0,
                CASSETTE_Y - 38.0,
                4.0,
            )
            .translate(-(CASSETTE_X / 2.0 + 6.0), SLOT_CENTER_Y, z)
            + centered_cube(
                format!("cassette_storage_recovery_slot_{slot}_right_thermal_buffer_strip"),
                28.0,
                CASSETTE_Y - 38.0,
                4.0,
            )
            .translate(CASSETTE_X / 2.0 + 6.0, SLOT_CENTER_Y, z)
            + centered_cube(
                format!("cassette_storage_recovery_slot_{slot}_rear_thermal_equalizer_bar"),
                CASSETTE_X - 96.0,
                10.0,
                5.0,
            )
            .translate(0.0, slot_rear_edge() + 24.0, z);
    }
    strips
}

fn top_recovery_baffle() -> Part {
    let roof = centered_cube(
        "cassette_storage_recovery_top_warm_recovery_baffle",
        RACK_X - 118.0,
        RACK_Y - 148.0,
        12.0,
    )
    .translate(0.0, SLOT_CENTER_Y + 18.0, last_slot_top_z() + 22.0);
    let center_slot = centered_cube(
        "cassette_storage_recovery_top_baffle_center_relief",
        CASSETTE_X - 120.0,
        18.0,
        14.0,
    )
    .translate(0.0, SLOT_CENTER_Y, last_slot_top_z() + 22.0);

    roof - center_slot
}

fn condensate_drip_controls() -> Part {
    let mut controls = Part::empty("cassette_storage_recovery_condensate_drip_controls");
    for slot in 0..STORAGE_SLOT_COUNT {
        controls = controls + slot_condensate_features(slot);
    }
    controls + vertical_drain_downspouts() + condensate_collection_header()
}

fn slot_condensate_features(slot: usize) -> Part {
    let top_z = slot_floor_z(slot) + CASSETTE_Z + 4.0;
    let rear_gutter = centered_cube(
        format!("cassette_storage_recovery_slot_{slot}_rear_condensate_gutter"),
        CASSETTE_X + 52.0,
        CONDENSATE_GUTTER_W,
        CONDENSATE_GUTTER_Z,
    )
    .translate(
        0.0,
        slot_rear_edge() + 20.0,
        top_z + CONDENSATE_GUTTER_Z / 2.0,
    );
    let front_drip_lip = centered_cube(
        format!("cassette_storage_recovery_slot_{slot}_front_drip_shadow_lip"),
        CASSETTE_X + 42.0,
        8.0,
        DRIP_SHIELD_Z,
    )
    .translate(0.0, slot_front_edge() - 20.0, top_z + DRIP_SHIELD_Z / 2.0);
    let left_gutter = centered_cube(
        format!("cassette_storage_recovery_slot_{slot}_left_side_condensate_gutter"),
        CONDENSATE_GUTTER_W,
        CASSETTE_Y + 34.0,
        CONDENSATE_GUTTER_Z,
    )
    .translate(
        -(CASSETTE_X / 2.0 + 22.0),
        SLOT_CENTER_Y,
        top_z + CONDENSATE_GUTTER_Z / 2.0,
    );
    let right_gutter = centered_cube(
        format!("cassette_storage_recovery_slot_{slot}_right_side_condensate_gutter"),
        CONDENSATE_GUTTER_W,
        CASSETTE_Y + 34.0,
        CONDENSATE_GUTTER_Z,
    )
    .translate(
        CASSETTE_X / 2.0 + 22.0,
        SLOT_CENTER_Y,
        top_z + CONDENSATE_GUTTER_Z / 2.0,
    );
    let drip_breaks = slot_drip_break_teeth(slot, top_z);

    rear_gutter + front_drip_lip + left_gutter + right_gutter + drip_breaks
}

fn slot_drip_break_teeth(slot: usize, top_z: f64) -> Part {
    let mut teeth = Part::empty(format!(
        "cassette_storage_recovery_slot_{slot}_drip_break_teeth"
    ));
    for i in 0..8 {
        let x = centered_index(i, 8, 68.0);
        teeth = teeth
            + centered_cube(
                format!("cassette_storage_recovery_slot_{slot}_drip_break_tooth_{i}"),
                12.0,
                5.0,
                12.0,
            )
            .translate(x, slot_front_edge() - 27.0, top_z + 6.0);
    }
    teeth
}

fn vertical_drain_downspouts() -> Part {
    let downspout_z = RACK_Z - BASE_Z - 86.0;
    let z = BASE_Z + downspout_z / 2.0 + 28.0;
    let left = centered_cube(
        "cassette_storage_recovery_left_condensate_downspout",
        DOWNSPOUT_W,
        DOWNSPOUT_W,
        downspout_z,
    )
    .translate(-(CASSETTE_X / 2.0 + 44.0), leak_basin_front_y() + 30.0, z);
    let right = centered_cube(
        "cassette_storage_recovery_right_condensate_downspout",
        DOWNSPOUT_W,
        DOWNSPOUT_W,
        downspout_z,
    )
    .translate(CASSETTE_X / 2.0 + 44.0, leak_basin_front_y() + 30.0, z);

    left + right
}

fn condensate_collection_header() -> Part {
    let header = centered_cube(
        "cassette_storage_recovery_condensate_collection_header",
        CASSETTE_X + 128.0,
        18.0,
        18.0,
    )
    .translate(0.0, leak_basin_front_y() + 20.0, BASE_Z + 18.0);
    let drain_boss = centered_cylinder(
        "cassette_storage_recovery_condensate_header_drain_boss",
        14.0,
        20.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        RACK_X / 2.0 - 72.0,
        leak_basin_front_y() + 8.0,
        BASE_Z + 18.0,
    );

    header + drain_boss
}

fn barcode_position_lands() -> Part {
    let front_spine = centered_cube(
        "cassette_storage_recovery_front_barcode_spine",
        RACK_X - 96.0,
        10.0,
        RACK_Z - BASE_Z - 84.0,
    )
    .translate(
        0.0,
        rack_front_y() + FRAME_W + 6.0,
        BASE_Z + (RACK_Z - BASE_Z - 84.0) / 2.0 + 38.0,
    );

    let mut lands = Part::empty("cassette_storage_recovery_barcode_position_lands");
    for slot in 0..STORAGE_SLOT_COUNT {
        lands = lands + slot_barcode_land(slot) + cassette_position_lands(slot);
    }

    front_spine + lands + rack_identity_land()
}

fn slot_barcode_land(slot: usize) -> Part {
    let z = slot_cassette_center_z(slot);
    let land = centered_cube(
        format!("cassette_storage_recovery_slot_{slot}_barcode_land"),
        BARCODE_LAND_X,
        BARCODE_LAND_Y,
        BARCODE_LAND_Z,
    )
    .translate(0.0, rack_front_y() + FRAME_W + 1.5, z);
    let scan_relief = centered_cube(
        format!("cassette_storage_recovery_slot_{slot}_barcode_scan_line_relief"),
        BARCODE_LAND_X - 16.0,
        BARCODE_LAND_Y + 1.0,
        3.0,
    )
    .translate(0.0, rack_front_y() + FRAME_W + 1.0, z + 9.0);

    land - scan_relief + slot_index_ticks(slot)
}

fn slot_index_ticks(slot: usize) -> Part {
    let mut ticks = Part::empty(format!("cassette_storage_recovery_slot_{slot}_index_ticks"));
    for i in 0..=slot {
        ticks = ticks
            + centered_cube(
                format!("cassette_storage_recovery_slot_{slot}_index_tick_{i}"),
                8.0,
                6.0,
                8.0,
            )
            .translate(
                -BARCODE_LAND_X / 2.0 + 16.0 + i as f64 * 12.0,
                rack_front_y() + FRAME_W - 2.0,
                slot_cassette_center_z(slot) - 12.0,
            );
    }
    ticks
}

fn cassette_position_lands(slot: usize) -> Part {
    let mut lands = Part::empty(format!(
        "cassette_storage_recovery_slot_{slot}_cassette_position_lands"
    ));
    let z = slot_floor_z(slot) + SLOT_RAIL_Z + POSITION_LAND_Z / 2.0 + 1.0;
    for row in 0..CASSETTE_ROWS {
        for col in 0..CASSETTE_COLS {
            let index = position_index(col, row);
            let (x, y) = cassette_position_center(col, row);
            lands = lands
                + centered_cube(
                    format!("cassette_storage_recovery_slot_{slot}_position_land_{index}"),
                    POSITION_LAND_X,
                    POSITION_LAND_Y,
                    POSITION_LAND_Z,
                )
                .translate(x, y - REVC_CHIP_WIDTH / 2.0 + 13.0, z);
        }
    }
    lands
}

fn rack_identity_land() -> Part {
    let land = centered_cube(
        "cassette_storage_recovery_rack_master_barcode_land",
        230.0,
        8.0,
        38.0,
    )
    .translate(
        -(RACK_X / 2.0 - 150.0),
        rack_front_y() + FRAME_W - 1.0,
        RACK_Z - 76.0,
    );
    let data_matrix_patch = centered_cube(
        "cassette_storage_recovery_rack_datamatrix_patch_land",
        38.0,
        9.0,
        38.0,
    )
    .translate(
        RACK_X / 2.0 - 96.0,
        rack_front_y() + FRAME_W - 1.0,
        RACK_Z - 76.0,
    );

    land + data_matrix_patch
}

fn sealed_transfer_tray_interface() -> Part {
    let tongue_y = rack_front_y() - TRANSFER_TONGUE_Y / 2.0 + 8.0;
    let tongue = centered_cube(
        "cassette_storage_recovery_sealed_transfer_tray_tongue",
        TRANSFER_TONGUE_X,
        TRANSFER_TONGUE_Y,
        TRANSFER_TONGUE_Z,
    )
    .translate(0.0, tongue_y, BASE_Z / 2.0 + TRANSFER_TONGUE_Z / 2.0);

    let key_slot = centered_cube(
        "cassette_storage_recovery_transfer_tray_center_key_slot",
        112.0,
        TRANSFER_TONGUE_Y + 4.0,
        10.0,
    )
    .translate(0.0, tongue_y, BASE_Z / 2.0 + TRANSFER_TONGUE_Z - 5.0);

    let gasket = rectangular_frame_xy(
        "cassette_storage_recovery_transfer_tray_gasket_land",
        CASSETTE_X + 84.0,
        CASSETTE_Y + 70.0,
        TRANSFER_GASKET_Z,
        CASSETTE_X + 34.0,
        CASSETTE_Y + 28.0,
    )
    .translate(0.0, SLOT_CENTER_Y, BASE_Z + TRANSFER_GASKET_Z / 2.0 + 3.0);

    tongue - key_slot + gasket + transfer_docking_features()
}

fn transfer_docking_features() -> Part {
    let front_y = rack_front_y() - 2.0;
    let left_rail = centered_cube(
        "cassette_storage_recovery_transfer_left_docking_rail",
        18.0,
        FRONT_TRANSFER_CLEARANCE_Y + 44.0,
        24.0,
    )
    .translate(
        -(TRANSFER_TONGUE_X / 2.0 - 44.0),
        front_y + FRONT_TRANSFER_CLEARANCE_Y / 2.0,
        BASE_Z + 12.0,
    );
    let right_rail = centered_cube(
        "cassette_storage_recovery_transfer_right_docking_rail",
        18.0,
        FRONT_TRANSFER_CLEARANCE_Y + 44.0,
        24.0,
    )
    .translate(
        TRANSFER_TONGUE_X / 2.0 - 44.0,
        front_y + FRONT_TRANSFER_CLEARANCE_Y / 2.0,
        BASE_Z + 12.0,
    );

    let mut pins = Part::empty("cassette_storage_recovery_transfer_docking_pins");
    for (i, x) in [
        -(TRANSFER_TONGUE_X / 2.0 - 86.0),
        TRANSFER_TONGUE_X / 2.0 - 86.0,
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("cassette_storage_recovery_transfer_pin_boss_{i}"),
            TRANSFER_PIN_D,
            12.0,
            32,
        )
        .translate(*x, rack_front_y() + FRAME_W + 30.0, BASE_Z + 8.0);
        let pilot = centered_cylinder(
            format!("cassette_storage_recovery_transfer_pin_pilot_{i}"),
            2.8,
            14.0,
            20,
        )
        .translate(*x, rack_front_y() + FRAME_W + 30.0, BASE_Z + 8.0);
        pins = pins + (boss - pilot);
    }

    let latch_keeper = centered_cube(
        "cassette_storage_recovery_transfer_tray_latch_keeper",
        180.0,
        20.0,
        34.0,
    )
    .translate(0.0, rack_front_y() + FRAME_W + 48.0, BASE_Z + 22.0);

    left_rail + right_rail + pins + latch_keeper
}

fn environmental_logger_pockets() -> Part {
    let panel_z = RACK_Z - BASE_Z - 128.0;
    let x = RACK_X / 2.0 - FRAME_W - LOGGER_POCKET_Z / 2.0;
    let panel = centered_cube(
        "cassette_storage_recovery_environmental_logger_side_panel",
        LOGGER_POCKET_Z,
        CASSETTE_Y + 86.0,
        panel_z,
    )
    .translate(x, SLOT_CENTER_Y, BASE_Z + panel_z / 2.0 + 58.0);

    let mut pockets = Part::empty("cassette_storage_recovery_environmental_logger_pockets");
    for i in 0..ENV_LOGGER_POCKET_COUNT {
        pockets =
            pockets + logger_pocket(i).translate(x - 3.0, logger_pocket_y(i), logger_pocket_z(i));
    }

    panel + pockets + logger_cable_raceway()
}

fn logger_pocket(index: usize) -> Part {
    let body = centered_cube(
        format!("cassette_storage_recovery_logger_pocket_body_{index}"),
        LOGGER_POCKET_Z,
        LOGGER_POCKET_X,
        LOGGER_POCKET_Y,
    );
    let recess = centered_cube(
        format!("cassette_storage_recovery_logger_pocket_recess_{index}"),
        LOGGER_RECESS_DEPTH + 0.4,
        LOGGER_POCKET_RECESS_X,
        LOGGER_POCKET_RECESS_Y,
    )
    .translate(LOGGER_POCKET_Z / 2.0 - LOGGER_RECESS_DEPTH / 2.0, 0.0, 0.0);

    let mut louvers = Part::empty(format!("cassette_storage_recovery_logger_louvers_{index}"));
    for i in 0..4 {
        louvers = louvers
            + centered_cube(
                format!("cassette_storage_recovery_logger_louver_{index}_{i}"),
                LOGGER_RECESS_DEPTH + 1.0,
                52.0,
                2.6,
            )
            .translate(
                LOGGER_POCKET_Z / 2.0 - LOGGER_RECESS_DEPTH / 2.0,
                0.0,
                -13.0 + i as f64 * 8.0,
            );
    }

    let cable_exit = centered_cube(
        format!("cassette_storage_recovery_logger_cable_exit_{index}"),
        LOGGER_RECESS_DEPTH + 1.0,
        10.0,
        28.0,
    )
    .translate(
        LOGGER_POCKET_Z / 2.0 - LOGGER_RECESS_DEPTH / 2.0,
        LOGGER_POCKET_X / 2.0 - 8.0,
        0.0,
    );

    body - recess - louvers - cable_exit + logger_mount_bosses(index)
}

fn logger_mount_bosses(index: usize) -> Part {
    let mut bosses = Part::empty(format!(
        "cassette_storage_recovery_logger_mount_bosses_{index}"
    ));
    for (i, z) in [-15.0, 15.0].iter().enumerate() {
        let boss = centered_cylinder(
            format!("cassette_storage_recovery_logger_mount_boss_{index}_{i}"),
            4.0,
            3.0,
            24,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(3.0, -LOGGER_POCKET_X / 2.0 + 14.0, *z);
        let pilot = centered_cylinder(
            format!("cassette_storage_recovery_logger_mount_pilot_{index}_{i}"),
            1.3,
            4.0,
            18,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(3.0, -LOGGER_POCKET_X / 2.0 + 14.0, *z);
        bosses = bosses + (boss - pilot);
    }
    bosses
}

fn logger_cable_raceway() -> Part {
    let x = RACK_X / 2.0 - FRAME_W - LOGGER_POCKET_Z - 10.0;
    let raceway = centered_cube(
        "cassette_storage_recovery_logger_cable_raceway",
        18.0,
        CASSETTE_Y + 64.0,
        28.0,
    )
    .translate(x, SLOT_CENTER_Y, RACK_Z - 94.0);
    let rear_gland = centered_cylinder(
        "cassette_storage_recovery_logger_rear_cable_gland",
        13.0,
        28.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(x, rack_rear_y() - 52.0, RACK_Z - 94.0);

    raceway + rear_gland
}

fn edge_center_mapping_points() -> Part {
    let mut markers = Part::empty("cassette_storage_recovery_edge_center_mapping_points");
    for (level_index, slot) in map_level_slots().iter().enumerate() {
        let z = slot_floor_z(*slot) + SLOT_RAIL_Z + 5.0;
        for (point_index, (x, y, is_center)) in mapping_point_layout().iter().enumerate() {
            let marker = if *is_center {
                centered_cylinder(
                    format!(
                        "cassette_storage_recovery_mapping_center_level_{level_index}_point_{point_index}"
                    ),
                    8.0,
                    3.0,
                    36,
                )
            } else {
                centered_cylinder(
                    format!(
                        "cassette_storage_recovery_mapping_edge_level_{level_index}_point_{point_index}"
                    ),
                    5.0,
                    2.6,
                    28,
                )
            }
            .translate(*x, *y, z);
            markers = markers + marker;
        }
    }
    markers
}

fn robot_service_keepouts() -> Part {
    let front_load = centered_cube(
        "cassette_storage_recovery_front_robot_load_keepout",
        CASSETTE_X + 170.0,
        FRONT_ROBOT_KEEP_OUT_Y,
        ROBOT_VERTICAL_CLEARANCE_Z,
    )
    .translate(
        0.0,
        rack_front_y() - FRONT_ROBOT_KEEP_OUT_Y / 2.0 + 4.0,
        slot_cassette_center_z(2),
    );

    let mut side_grip = Part::empty("cassette_storage_recovery_side_gripper_keepouts");
    for slot in 0..STORAGE_SLOT_COUNT {
        let z = slot_cassette_center_z(slot);
        side_grip = side_grip
            + centered_cube(
                format!("cassette_storage_recovery_left_robot_gripper_keepout_slot_{slot}"),
                SIDE_ROBOT_KEEP_OUT_X,
                CASSETTE_Y + 54.0,
                ROBOT_VERTICAL_CLEARANCE_Z,
            )
            .translate(
                -(CASSETTE_X / 2.0 + SIDE_AIR_GAP_X + SIDE_ROBOT_KEEP_OUT_X / 2.0),
                SLOT_CENTER_Y,
                z,
            )
            + centered_cube(
                format!("cassette_storage_recovery_right_robot_gripper_keepout_slot_{slot}"),
                SIDE_ROBOT_KEEP_OUT_X,
                CASSETTE_Y + 54.0,
                ROBOT_VERTICAL_CLEARANCE_Z,
            )
            .translate(
                CASSETTE_X / 2.0 + SIDE_AIR_GAP_X + SIDE_ROBOT_KEEP_OUT_X / 2.0,
                SLOT_CENTER_Y,
                z,
            );
    }

    let rear_service = centered_cube(
        "cassette_storage_recovery_rear_plenum_service_keepout",
        RACK_X - 120.0,
        REAR_SERVICE_CLEARANCE_Y,
        RACK_Z - BASE_Z - 60.0,
    )
    .translate(
        0.0,
        rack_rear_y() + REAR_SERVICE_CLEARANCE_Y / 2.0 - 4.0,
        BASE_Z + (RACK_Z - BASE_Z - 60.0) / 2.0 + 30.0,
    );

    let top_logger_service = centered_cube(
        "cassette_storage_recovery_top_logger_calibration_keepout",
        300.0,
        250.0,
        92.0,
    )
    .translate(
        RACK_X / 2.0 - 178.0,
        SLOT_CENTER_Y,
        RACK_Z + 92.0 / 2.0 - 8.0,
    );

    front_load + side_grip + rear_service + top_logger_service
}

fn frame_post_positions() -> [(f64, f64); 8] {
    [
        (
            -(RACK_X / 2.0 - FRAME_W / 2.0),
            rack_front_y() + FRAME_W / 2.0,
        ),
        (RACK_X / 2.0 - FRAME_W / 2.0, rack_front_y() + FRAME_W / 2.0),
        (
            -(RACK_X / 2.0 - FRAME_W / 2.0),
            rack_rear_y() - FRAME_W / 2.0,
        ),
        (RACK_X / 2.0 - FRAME_W / 2.0, rack_rear_y() - FRAME_W / 2.0),
        (-(RACK_X / 2.0 - FRAME_W / 2.0), SLOT_CENTER_Y),
        (RACK_X / 2.0 - FRAME_W / 2.0, SLOT_CENTER_Y),
        (
            -(CASSETTE_X / 2.0 + SIDE_AIR_GAP_X + FRAME_W / 2.0),
            rack_rear_y() - FRAME_W / 2.0,
        ),
        (
            CASSETTE_X / 2.0 + SIDE_AIR_GAP_X + FRAME_W / 2.0,
            rack_rear_y() - FRAME_W / 2.0,
        ),
    ]
}

fn map_level_slots() -> [usize; MAP_LEVEL_COUNT] {
    [0, STORAGE_SLOT_COUNT / 2, STORAGE_SLOT_COUNT - 1]
}

fn mapping_point_layout() -> [(f64, f64, bool); MAPPING_POINTS_PER_LEVEL] {
    let left = -(CASSETTE_X / 2.0 - 36.0);
    let right = CASSETTE_X / 2.0 - 36.0;
    let front = slot_front_edge() + 36.0;
    let rear = slot_rear_edge() - 36.0;
    let mid_x = 0.0;
    let mid_y = SLOT_CENTER_Y;

    [
        (left, front, false),
        (mid_x, front, false),
        (right, front, false),
        (left, mid_y, false),
        (mid_x, mid_y, true),
        (right, mid_y, false),
        (left, rear, false),
        (mid_x, rear, false),
        (right, rear, false),
    ]
}

fn rectangular_frame_xy(
    name: impl Into<String>,
    outer_x: f64,
    outer_y: f64,
    z: f64,
    inner_x: f64,
    inner_y: f64,
) -> Part {
    let name = name.into();
    let outer = centered_cube(name.clone(), outer_x, outer_y, z);
    let inner = centered_cube(format!("{name}_inner_clearance"), inner_x, inner_y, z + 0.2);
    outer - inner
}

fn cassette_position_center(col: usize, row: usize) -> (f64, f64) {
    (
        cassette_left_edge() + CASSETTE_MARGIN_X + REVC_CHIP_LENGTH / 2.0 + col as f64 * pitch_x(),
        cassette_bottom_edge() + CASSETTE_MARGIN_Y + REVC_CHIP_WIDTH / 2.0 + row as f64 * pitch_y(),
    )
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn pitch_x() -> f64 {
    REVC_CHIP_LENGTH + CHIP_GUTTER
}

fn pitch_y() -> f64 {
    REVC_CHIP_WIDTH + CHIP_GUTTER
}

fn position_index(col: usize, row: usize) -> usize {
    row * CASSETTE_COLS + col
}

fn total_position_lands() -> usize {
    STORAGE_SLOT_COUNT * CASSETTE_POSITION_COUNT
}

fn total_mapping_points() -> usize {
    MAP_LEVEL_COUNT * MAPPING_POINTS_PER_LEVEL
}

fn edge_mapping_point_count() -> usize {
    MAP_LEVEL_COUNT * EDGE_MAPPING_POINTS_PER_LEVEL
}

fn center_mapping_point_count() -> usize {
    MAP_LEVEL_COUNT * CENTER_MAPPING_POINTS_PER_LEVEL
}

fn cassette_left_edge() -> f64 {
    -CASSETTE_X / 2.0
}

fn cassette_bottom_edge() -> f64 {
    SLOT_CENTER_Y - CASSETTE_Y / 2.0
}

fn slot_front_edge() -> f64 {
    SLOT_CENTER_Y - CASSETTE_Y / 2.0
}

fn slot_rear_edge() -> f64 {
    SLOT_CENTER_Y + CASSETTE_Y / 2.0
}

fn rack_front_y() -> f64 {
    -RACK_Y / 2.0
}

fn rack_rear_y() -> f64 {
    RACK_Y / 2.0
}

fn leak_basin_front_y() -> f64 {
    SLOT_CENTER_Y - (CASSETTE_Y + 88.0) / 2.0
}

fn drain_port_y() -> f64 {
    rack_front_y() + 16.0
}

fn slot_floor_z(slot: usize) -> f64 {
    SLOT_FLOOR_Z0 + slot as f64 * SLOT_PITCH_Z
}

fn slot_cassette_center_z(slot: usize) -> f64 {
    slot_floor_z(slot) + CASSETTE_Z / 2.0
}

fn last_slot_top_z() -> f64 {
    slot_floor_z(STORAGE_SLOT_COUNT - 1) + CASSETTE_Z
}

fn front_transfer_clearance() -> f64 {
    slot_front_edge() - rack_front_y()
}

fn rear_plenum_clearance() -> f64 {
    rack_rear_y() - slot_rear_edge()
}

fn logger_pocket_y(index: usize) -> f64 {
    SLOT_CENTER_Y + centered_index(index, ENV_LOGGER_POCKET_COUNT, 88.0)
}

fn logger_pocket_z(index: usize) -> f64 {
    BASE_Z + 92.0 + index as f64 * LOGGER_BANK_PITCH_Z
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cassette_geometry_has_twenty_positions_per_slot() {
        assert_eq!(CASSETTE_COLS, 4);
        assert_eq!(CASSETTE_ROWS, 5);
        assert_eq!(CASSETTE_POSITION_COUNT, 20);
        assert_eq!(total_position_lands(), 120);
        assert!(CASSETTE_X > ARRAY_X);
        assert!(CASSETTE_Y > ARRAY_Y);

        for row in 0..CASSETTE_ROWS {
            for col in 0..CASSETTE_COLS {
                let (x, y) = cassette_position_center(col, row);
                assert!(x > cassette_left_edge());
                assert!(x < -cassette_left_edge());
                assert!(y > slot_front_edge());
                assert!(y < slot_rear_edge());
            }
        }
    }

    #[test]
    fn storage_slots_have_incubator_airflow_spacing() {
        assert_eq!(STORAGE_SLOT_COUNT, 6);
        assert!(SLOT_AIR_GAP_Z >= 32.0);
        assert!(SIDE_AIR_GAP_X >= 36.0);
        for slot in 1..STORAGE_SLOT_COUNT {
            let spacing = slot_floor_z(slot) - slot_floor_z(slot - 1);
            assert!((spacing - SLOT_PITCH_Z).abs() < 0.001);
        }
        assert!(last_slot_top_z() + TOP_SERVICE_Z <= RACK_Z + 0.01);
    }

    #[test]
    fn mapping_and_logger_counts_are_sane() {
        assert_eq!(ENV_LOGGER_POCKET_COUNT, 4);
        assert_eq!(map_level_slots(), [0, 3, 5]);
        assert_eq!(edge_mapping_point_count(), 24);
        assert_eq!(center_mapping_point_count(), 3);
        assert_eq!(total_mapping_points(), 27);

        let center_points = mapping_point_layout()
            .iter()
            .filter(|(_, _, is_center)| *is_center)
            .count();
        assert_eq!(center_points, 1);
    }

    #[test]
    fn transfer_datum_and_robot_keepouts_clear_the_rack() {
        assert!(front_transfer_clearance() >= FRONT_TRANSFER_CLEARANCE_Y - 4.0);
        assert!(rear_plenum_clearance() >= REAR_PLENUM_Y + 18.0);
        assert!(TRANSFER_TONGUE_X > CASSETTE_X);
        assert!(FRONT_ROBOT_KEEP_OUT_Y >= 250.0);
        assert!(SIDE_ROBOT_KEEP_OUT_X >= 60.0);
        assert!(ROBOT_VERTICAL_CLEARANCE_Z > CASSETTE_Z + REVC_TOTAL_HEIGHT);
    }

    #[test]
    fn condensate_and_leak_features_route_to_front_sump() {
        assert!(leak_basin_front_y() > rack_front_y());
        assert!(drain_port_y() < leak_basin_front_y());
        assert!(CONDENSATE_GUTTER_W >= 10.0);
        assert!(DOWNSPOUT_W >= DRAIN_PORT_D);
        assert!(LEAK_BASIN_DEPTH >= 12.0);
    }
}
