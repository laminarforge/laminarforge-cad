use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Sterile tote docking and transfer station concept.
//
// This is a product CAD concept for moving sealed cassettes, tubing kits,
// samples, or reagent lots between closed modules. It models mechanical
// interfaces only: tote datums, gasket/door envelopes, identity lands,
// segregated clean/return lanes, leak management, rail/tongue handoff, sensor
// placeholders, and robot/service keepouts. It is not a sterilization protocol
// or compliance claim.

const COLS: usize = 4;
const ROWS: usize = 5;
const GUTTER: f64 = 5.0;
const CASSETTE_MARGIN_X: f64 = 34.0;
const CASSETTE_MARGIN_Y: f64 = 32.0;
const CASSETTE_Z: f64 = 46.0;

const ARRAY_X: f64 = COLS as f64 * REVC_CHIP_LENGTH + (COLS as f64 - 1.0) * GUTTER;
const ARRAY_Y: f64 = ROWS as f64 * REVC_CHIP_WIDTH + (ROWS as f64 - 1.0) * GUTTER;
const CASSETTE_X: f64 = ARRAY_X + CASSETTE_MARGIN_X * 2.0;
const CASSETTE_Y: f64 = ARRAY_Y + CASSETTE_MARGIN_Y * 2.0;

const TOTE_INNER_X: f64 = CASSETTE_X + 188.0;
const TOTE_INNER_Y: f64 = CASSETTE_Y + 176.0;
const TOTE_INNER_Z: f64 = 255.0;
const TOTE_WALL: f64 = 22.0;
const TOTE_OUTER_X: f64 = TOTE_INNER_X + TOTE_WALL * 2.0;
const TOTE_OUTER_Y: f64 = TOTE_INNER_Y + TOTE_WALL * 2.0;
const TOTE_OUTER_Z: f64 = TOTE_INNER_Z + 46.0;

const DECK_X: f64 = 1540.0;
const DECK_Y: f64 = 980.0;
const DECK_Z: f64 = 22.0;
const TRAY_RIM_W: f64 = 24.0;
const TRAY_RIM_H: f64 = 36.0;
const BASIN_DEPTH: f64 = 12.0;
const SUMP_X: f64 = 132.0;
const SUMP_Y: f64 = 72.0;
const DRAIN_PORT_DIA: f64 = 14.0;

const RECEIVER_RAIL_Z: f64 = 38.0;
const RECEIVER_RAIL_W: f64 = 26.0;
const RECEIVER_CLEARANCE: f64 = 6.0;
const LOCATOR_PIN_DIA: f64 = 18.0;
const LOCATOR_RECEIVER_DIA: f64 = 24.0;
const LOCATOR_COUNT: usize = 4;

const DOOR_FRAME_X: f64 = TOTE_OUTER_X + 130.0;
const DOOR_FRAME_Y: f64 = 34.0;
const DOOR_FRAME_Z: f64 = 430.0;
const DOOR_OPENING_X: f64 = TOTE_INNER_X + 38.0;
const DOOR_OPENING_Z: f64 = TOTE_INNER_Z + 68.0;
const DOOR_CENTER_Y: f64 = TOTE_OUTER_Y / 2.0 + DOOR_FRAME_Y / 2.0 + 34.0;
const DOOR_CENTER_Z: f64 = DECK_Z + DOOR_FRAME_Z / 2.0 + 18.0;
const GASKET_LAND_Y: f64 = 8.0;

const CLEAN_LANE_COUNT: usize = 3;
const DIRTY_LANE_COUNT: usize = 2;
const LANE_SLOT_X: f64 = 214.0;
const LANE_SLOT_Y: f64 = 128.0;
const LANE_WALL_Z: f64 = 42.0;
const SEGREGATION_BARRIER_Y: f64 = 30.0;

const IDENTITY_LAND_COUNT: usize = 12;
const IDENTITY_LAND_X: f64 = 124.0;
const IDENTITY_LAND_Y: f64 = 42.0;
const IDENTITY_LAND_Z: f64 = 8.0;

const TONGUE_X: f64 = CASSETTE_X + 146.0;
const TONGUE_Y: f64 = 430.0;
const TONGUE_Z: f64 = 18.0;
const TONGUE_REAR_OVERHANG: f64 = 270.0;
const TRANSFER_RAIL_PITCH_X: f64 = CASSETTE_X + 74.0;
const TRANSFER_RAIL_W: f64 = 22.0;
const TRANSFER_RAIL_Z: f64 = 28.0;

const LOGGER_POCKET_COUNT: usize = 2;
const LOGGER_POCKET_X: f64 = 122.0;
const LOGGER_POCKET_Y: f64 = 72.0;
const LOGGER_POCKET_Z: f64 = 36.0;

const TOTE_PRESENT_SENSOR_COUNT: usize = 4;
const LATCH_SENSOR_COUNT: usize = 4;
const SENSOR_BLOCK_X: f64 = 42.0;
const SENSOR_BLOCK_Y: f64 = 20.0;
const SENSOR_BLOCK_Z: f64 = 30.0;

const HANDOFF_CLEARANCE_Z: f64 = REVC_TOTAL_HEIGHT + 72.0;
const HANDOFF_ENVELOPE_X: f64 = CASSETTE_X + 44.0;
const HANDOFF_ENVELOPE_Y: f64 = CASSETTE_Y + 44.0;
const SHUTTLE_TONGUE_CLEARANCE_Y: f64 = 360.0;
const RACK_HANDOFF_CLEARANCE_Y: f64 = 280.0;

const ROBOT_FRONT_KEEP_OUT_Y: f64 = 520.0;
const REAR_DOOR_SWING_KEEP_OUT_Y: f64 = 410.0;
const SIDE_SERVICE_KEEP_OUT_X: f64 = 230.0;
const OVERHEAD_LIFT_KEEP_OUT_Z: f64 = 610.0;
const KEEP_OUT_RAIL_W: f64 = 14.0;

const PART_NAMES: [&str; 10] = [
    "base_leak_condensate_tray",
    "sealed_tote_receiver_datum",
    "gasket_door_envelope",
    "barcode_rfid_identity_lands",
    "clean_dirty_lane_segregation",
    "transfer_tongue_rail_interface",
    "environmental_logger_pocket",
    "tote_present_latch_sensors",
    "handoff_clearance_gauges",
    "robot_service_keepouts",
];
const ASSEMBLY_NAME: &str = "assembly";

fn main() {
    let tray = base_leak_condensate_tray();
    write_part(PART_NAMES[0], &tray);

    let receiver = sealed_tote_receiver_datum();
    write_part(PART_NAMES[1], &receiver);

    let door = gasket_door_envelope();
    write_part(PART_NAMES[2], &door);

    let identity = barcode_rfid_identity_lands();
    write_part(PART_NAMES[3], &identity);

    let lanes = clean_dirty_lane_segregation();
    write_part(PART_NAMES[4], &lanes);

    let tongue = transfer_tongue_rail_interface();
    write_part(PART_NAMES[5], &tongue);

    let logger = environmental_logger_pocket();
    write_part(PART_NAMES[6], &logger);

    let sensors = tote_present_latch_sensors();
    write_part(PART_NAMES[7], &sensors);

    let handoff = handoff_clearance_gauges();
    write_part(PART_NAMES[8], &handoff);

    let keepouts = robot_service_keepouts();
    write_part(PART_NAMES[9], &keepouts);

    let assembly =
        tray + receiver + door + identity + lanes + tongue + logger + sensors + handoff + keepouts;
    write_part(ASSEMBLY_NAME, &assembly);

    println!(
        "Sterile tote docking station: {DECK_X:.0} x {DECK_Y:.0} mm deck for {TOTE_OUTER_X:.1} x {TOTE_OUTER_Y:.1} x {TOTE_OUTER_Z:.0} mm sealed tote envelope."
    );
    println!(
        "Cassette payload clearance: {CASSETTE_X:.1} x {CASSETTE_Y:.1} x {CASSETTE_Z:.0} mm for a {COLS}x{ROWS} Rev C cassette."
    );
    println!(
        "Transfer tongue: {TONGUE_X:.1} x {TONGUE_Y:.0} mm with rail pitch {TRANSFER_RAIL_PITCH_X:.1} mm and {TONGUE_REAR_OVERHANG:.0} mm rear overhang."
    );
    println!(
        "Traceability and sensing: {IDENTITY_LAND_COUNT} barcode/RFID lands, {LOGGER_POCKET_COUNT} logger pockets, {TOTE_PRESENT_SENSOR_COUNT} tote-present sensors, {LATCH_SENSOR_COUNT} latch sensors."
    );
    println!(
        "Handoff clearances: {HANDOFF_ENVELOPE_X:.1} x {HANDOFF_ENVELOPE_Y:.1} mm cassette gauge, {HANDOFF_CLEARANCE_Z:.1} mm Z clearance."
    );
}

fn write_part(name: &str, part: &Part) {
    let path = format!("output/sterile_tote_docking_and_transfer_station_{name}.stl");
    part.write_stl(&path).unwrap();
    println!("Exported: {path}");
}

fn base_leak_condensate_tray() -> Part {
    let deck = centered_cube("sterile_tote_station_deck_plate", DECK_X, DECK_Y, DECK_Z).translate(
        0.0,
        0.0,
        DECK_Z / 2.0,
    );
    let recessed_basin = centered_cube(
        "sterile_tote_station_recessed_basin_cut",
        TOTE_OUTER_X + 150.0,
        TOTE_OUTER_Y + 104.0,
        BASIN_DEPTH + 2.0,
    )
    .translate(0.0, 0.0, DECK_Z - BASIN_DEPTH / 2.0 + 1.0);
    let condensate_gutter = centered_cube(
        "sterile_tote_station_condensate_front_gutter_cut",
        TOTE_OUTER_X + 220.0,
        24.0,
        BASIN_DEPTH + 5.0,
    )
    .translate(0.0, -(TOTE_OUTER_Y / 2.0 + 54.0), DECK_Z - 5.0);
    let sump_cut = centered_cube(
        "sterile_tote_station_sump_well_cut",
        SUMP_X,
        SUMP_Y,
        BASIN_DEPTH + 8.0,
    )
    .translate(
        DECK_X / 2.0 - SUMP_X / 2.0 - 52.0,
        -DECK_Y / 2.0 + SUMP_Y / 2.0 + 38.0,
        DECK_Z - 6.0,
    );
    let drain_port = centered_cylinder(
        "sterile_tote_station_drain_port_cut",
        DRAIN_PORT_DIA / 2.0,
        70.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        DECK_X / 2.0 - SUMP_X / 2.0 - 52.0,
        -DECK_Y / 2.0 + 12.0,
        DECK_Z - BASIN_DEPTH / 2.0,
    );

    deck - recessed_basin - condensate_gutter - sump_cut - drain_port
        + tray_rim_walls()
        + deck_mount_bosses()
        + drain_sump_flag()
}

fn tray_rim_walls() -> Part {
    let z = DECK_Z + TRAY_RIM_H / 2.0;
    let front = centered_cube(
        "sterile_tote_station_front_rim",
        DECK_X,
        TRAY_RIM_W,
        TRAY_RIM_H,
    )
    .translate(0.0, -DECK_Y / 2.0 + TRAY_RIM_W / 2.0, z);
    let rear = centered_cube(
        "sterile_tote_station_rear_rim",
        DECK_X,
        TRAY_RIM_W,
        TRAY_RIM_H,
    )
    .translate(0.0, DECK_Y / 2.0 - TRAY_RIM_W / 2.0, z);
    let left = centered_cube(
        "sterile_tote_station_left_rim",
        TRAY_RIM_W,
        DECK_Y,
        TRAY_RIM_H,
    )
    .translate(-DECK_X / 2.0 + TRAY_RIM_W / 2.0, 0.0, z);
    let right = centered_cube(
        "sterile_tote_station_right_rim",
        TRAY_RIM_W,
        DECK_Y,
        TRAY_RIM_H,
    )
    .translate(DECK_X / 2.0 - TRAY_RIM_W / 2.0, 0.0, z);
    front + rear + left + right
}

fn deck_mount_bosses() -> Part {
    let mut bosses = Part::empty("sterile_tote_station_deck_mount_bosses");
    for (i, (x, y)) in deck_mount_points().iter().enumerate() {
        let boss = centered_cylinder(format!("deck_mount_boss_{i}"), 18.0, 8.0, 32).translate(
            *x,
            *y,
            DECK_Z + 4.0,
        );
        let hole = centered_cylinder(format!("deck_mount_m6_clearance_{i}"), 3.4, 12.0, 24)
            .translate(*x, *y, DECK_Z + 4.0);
        bosses = bosses + (boss - hole);
    }
    bosses
}

fn drain_sump_flag() -> Part {
    centered_cube("sterile_tote_station_sump_visual_flag", 84.0, 18.0, 18.0).translate(
        DECK_X / 2.0 - SUMP_X / 2.0 - 52.0,
        -DECK_Y / 2.0 + SUMP_Y + 58.0,
        DECK_Z + 9.0,
    )
}

fn sealed_tote_receiver_datum() -> Part {
    let z = DECK_Z + RECEIVER_RAIL_Z / 2.0;
    let receiver_x = TOTE_OUTER_X + RECEIVER_CLEARANCE * 2.0;
    let receiver_y = TOTE_OUTER_Y + RECEIVER_CLEARANCE * 2.0;

    let rear_stop = centered_cube(
        "sealed_tote_receiver_rear_y_datum",
        receiver_x + 58.0,
        RECEIVER_RAIL_W,
        RECEIVER_RAIL_Z,
    )
    .translate(0.0, receiver_y / 2.0 + RECEIVER_RAIL_W / 2.0, z);
    let left_stop = centered_cube(
        "sealed_tote_receiver_left_x_datum",
        RECEIVER_RAIL_W,
        receiver_y + 44.0,
        RECEIVER_RAIL_Z,
    )
    .translate(-(receiver_x / 2.0 + RECEIVER_RAIL_W / 2.0), 0.0, z);
    let right_soft_rail = centered_cube(
        "sealed_tote_receiver_right_soft_rail",
        RECEIVER_RAIL_W,
        receiver_y + 44.0,
        RECEIVER_RAIL_Z * 0.70,
    )
    .translate(
        receiver_x / 2.0 + RECEIVER_RAIL_W / 2.0,
        0.0,
        z - RECEIVER_RAIL_Z * 0.15,
    );
    let front_low_lip = centered_cube(
        "sealed_tote_receiver_front_low_lip",
        receiver_x + 58.0,
        16.0,
        RECEIVER_RAIL_Z * 0.55,
    )
    .translate(0.0, -(receiver_y / 2.0 + 10.0), z - RECEIVER_RAIL_Z * 0.225);

    rear_stop
        + left_stop
        + right_soft_rail
        + front_low_lip
        + tote_locator_receivers()
        + docking_latch_pockets()
}

fn tote_locator_receivers() -> Part {
    let mut locators = Part::empty("sealed_tote_receiver_kinematic_locators");
    for (i, (x, y)) in tote_locator_points().iter().enumerate() {
        let receiver = centered_cylinder(
            format!("sealed_tote_locator_receiver_{i}"),
            LOCATOR_RECEIVER_DIA / 2.0,
            RECEIVER_RAIL_Z,
            40,
        )
        .translate(*x, *y, DECK_Z + RECEIVER_RAIL_Z / 2.0);
        let pin_relief = centered_cylinder(
            format!("sealed_tote_locator_pin_relief_{i}"),
            LOCATOR_PIN_DIA / 2.0,
            RECEIVER_RAIL_Z + 4.0,
            32,
        )
        .translate(*x, *y, DECK_Z + RECEIVER_RAIL_Z / 2.0);
        locators = locators + (receiver - pin_relief);
    }
    locators
}

fn docking_latch_pockets() -> Part {
    let y_front = -(TOTE_OUTER_Y / 2.0 + 58.0);
    let y_rear = TOTE_OUTER_Y / 2.0 + 58.0;
    let mut pockets = Part::empty("sealed_tote_receiver_latch_pockets");
    for (i, (x, y)) in [
        (-(TOTE_OUTER_X / 2.0 - 96.0), y_front),
        (TOTE_OUTER_X / 2.0 - 96.0, y_front),
        (-(TOTE_OUTER_X / 2.0 - 96.0), y_rear),
        (TOTE_OUTER_X / 2.0 - 96.0, y_rear),
    ]
    .iter()
    .enumerate()
    {
        let body = centered_cube(format!("sealed_tote_latch_pocket_{i}"), 76.0, 42.0, 24.0)
            .translate(*x, *y, DECK_Z + 12.0);
        let relief = centered_cube(
            format!("sealed_tote_latch_hook_clearance_{i}"),
            46.0,
            22.0,
            26.0,
        )
        .translate(*x, *y, DECK_Z + 14.0);
        pockets = pockets + (body - relief);
    }
    pockets
}

fn gasket_door_envelope() -> Part {
    let body_frame = rectangular_frame_xz(
        "sterile_tote_docking_door_body_frame",
        DOOR_FRAME_X,
        DOOR_FRAME_Y,
        DOOR_FRAME_Z,
        DOOR_OPENING_X,
        DOOR_OPENING_Z,
    )
    .translate(0.0, DOOR_CENTER_Y, DOOR_CENTER_Z);
    let tote_gasket = rectangular_frame_xz(
        "sterile_tote_docking_gasket_land",
        DOOR_OPENING_X + 62.0,
        GASKET_LAND_Y,
        DOOR_OPENING_Z + 54.0,
        DOOR_OPENING_X + 10.0,
        DOOR_OPENING_Z + 8.0,
    )
    .translate(
        0.0,
        DOOR_CENTER_Y - DOOR_FRAME_Y / 2.0 - GASKET_LAND_Y / 2.0,
        DOOR_CENTER_Z,
    );
    let outer_door = split_door_leaf("outer", -1.0).translate(0.0, DOOR_CENTER_Y + 34.0, 0.0);
    let inner_door = split_door_leaf("inner", 1.0).translate(0.0, DOOR_CENTER_Y - 34.0, 0.0);
    let hinge = vertical_hinge_barrel_stack();
    let latch = docking_door_latches();

    body_frame + tote_gasket + outer_door + inner_door + hinge + latch
}

fn split_door_leaf(name: &str, face_sign: f64) -> Part {
    let leaf_y = 18.0;
    let leaf_x = DOOR_OPENING_X / 2.0 + 64.0;
    let leaf_z = DOOR_OPENING_Z + 64.0;
    let left_leaf = centered_cube(
        format!("{name}_left_split_door_leaf"),
        leaf_x,
        leaf_y,
        leaf_z,
    )
    .translate(-(leaf_x / 2.0 - 8.0), 0.0, DOOR_CENTER_Z);
    let right_leaf = centered_cube(
        format!("{name}_right_split_door_leaf"),
        leaf_x,
        leaf_y,
        leaf_z,
    )
    .translate(leaf_x / 2.0 - 8.0, 0.0, DOOR_CENTER_Z);
    let seam_keepout = centered_cube(
        format!("{name}_center_overlap_seal_land"),
        28.0,
        leaf_y + 8.0,
        leaf_z,
    )
    .translate(0.0, face_sign * 8.0, DOOR_CENTER_Z);
    let window_left = centered_cube(
        format!("{name}_left_observation_slot_cut"),
        168.0,
        leaf_y + 4.0,
        42.0,
    )
    .translate(-leaf_x / 2.0, 0.0, DOOR_CENTER_Z + 76.0);
    let window_right = centered_cube(
        format!("{name}_right_observation_slot_cut"),
        168.0,
        leaf_y + 4.0,
        42.0,
    )
    .translate(leaf_x / 2.0, 0.0, DOOR_CENTER_Z + 76.0);

    left_leaf + right_leaf + seam_keepout - window_left - window_right
}

fn vertical_hinge_barrel_stack() -> Part {
    let x = -DOOR_FRAME_X / 2.0 + 38.0;
    let y = DOOR_CENTER_Y + 34.0;
    let mut stack = Part::empty("sterile_tote_door_hinge_barrel_stack");
    for (i, z_offset) in [-138.0, 0.0, 138.0].iter().enumerate() {
        let barrel = centered_cylinder(
            format!("sterile_tote_door_hinge_barrel_{i}"),
            14.0,
            82.0,
            32,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(x, y, DOOR_CENTER_Z + *z_offset);
        let pin = centered_cylinder(
            format!("sterile_tote_door_hinge_pin_cut_{i}"),
            3.2,
            86.0,
            20,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(x, y, DOOR_CENTER_Z + *z_offset);
        stack = stack + (barrel - pin);
    }
    stack
}

fn docking_door_latches() -> Part {
    let x = DOOR_FRAME_X / 2.0 - 76.0;
    let y = DOOR_CENTER_Y + 45.0;
    let mut latches = Part::empty("sterile_tote_door_latches");
    for (i, z_offset) in [-116.0, 116.0].iter().enumerate() {
        let keeper = centered_cube(
            format!("sterile_tote_door_latch_keeper_{i}"),
            72.0,
            28.0,
            42.0,
        )
        .translate(x, y, DOOR_CENTER_Z + *z_offset);
        let latch_flag = centered_cube(
            format!("sterile_tote_door_latch_sensor_flag_{i}"),
            22.0,
            10.0,
            52.0,
        )
        .translate(x - 56.0, y + 18.0, DOOR_CENTER_Z + *z_offset);
        latches = latches + keeper + latch_flag;
    }
    latches
}

fn barcode_rfid_identity_lands() -> Part {
    let mut lands = Part::empty("sterile_tote_identity_lands");
    for i in 0..IDENTITY_LAND_COUNT {
        let (x, y, rotation) = identity_land_pose(i);
        let land = identity_land(i);
        let placed = if rotation {
            land.rotate(0.0, 0.0, 90.0)
                .translate(x, y, DECK_Z + IDENTITY_LAND_Z / 2.0)
        } else {
            land.translate(x, y, DECK_Z + IDENTITY_LAND_Z / 2.0)
        };
        lands = lands + placed;
    }
    lands + route_arrow_datums()
}

fn identity_land(index: usize) -> Part {
    let body = centered_cube(
        format!("identity_land_{index}_body"),
        IDENTITY_LAND_X,
        IDENTITY_LAND_Y,
        IDENTITY_LAND_Z,
    );
    let barcode_recess = centered_cube(
        format!("identity_land_{index}_barcode_recess"),
        78.0,
        18.0,
        IDENTITY_LAND_Z + 2.0,
    )
    .translate(-14.0, 0.0, 1.5);
    let rfid_pad = centered_cube(format!("identity_land_{index}_rfid_pad"), 28.0, 28.0, 4.0)
        .translate(43.0, 0.0, IDENTITY_LAND_Z / 2.0 + 2.0);
    body - barcode_recess + rfid_pad
}

fn route_arrow_datums() -> Part {
    let z = DECK_Z + 4.0;
    let clean_arrow = centered_cube("clean_flow_route_arrow_datum", 172.0, 18.0, 8.0).translate(
        -DECK_X / 2.0 + 280.0,
        -DECK_Y / 2.0 + 82.0,
        z,
    );
    let dirty_arrow = centered_cube("return_flow_route_arrow_datum", 172.0, 18.0, 8.0).translate(
        DECK_X / 2.0 - 280.0,
        -DECK_Y / 2.0 + 82.0,
        z,
    );
    let clean_head = centered_cube("clean_flow_route_arrow_head", 32.0, 32.0, 8.0)
        .rotate(0.0, 0.0, 45.0)
        .translate(-DECK_X / 2.0 + 380.0, -DECK_Y / 2.0 + 82.0, z);
    let dirty_head = centered_cube("return_flow_route_arrow_head", 32.0, 32.0, 8.0)
        .rotate(0.0, 0.0, 45.0)
        .translate(DECK_X / 2.0 - 380.0, -DECK_Y / 2.0 + 82.0, z);
    clean_arrow + dirty_arrow + clean_head + dirty_head
}

fn clean_dirty_lane_segregation() -> Part {
    let mut lanes = Part::empty("sterile_tote_clean_dirty_lane_slots");
    for i in 0..CLEAN_LANE_COUNT {
        let x = lane_x(i, CLEAN_LANE_COUNT, -1.0);
        lanes = lanes + lane_slot(format!("clean_input_lane_{i}"), x, -DECK_Y / 2.0 + 170.0);
    }
    for i in 0..DIRTY_LANE_COUNT {
        let x = lane_x(i, DIRTY_LANE_COUNT, 1.0);
        lanes = lanes + lane_slot(format!("dirty_return_lane_{i}"), x, -DECK_Y / 2.0 + 170.0);
    }

    let barrier = centered_cube(
        "clean_dirty_physical_segregation_barrier",
        DECK_X - 240.0,
        SEGREGATION_BARRIER_Y,
        LANE_WALL_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + 286.0, DECK_Z + LANE_WALL_Z / 2.0);
    let one_way_gate = centered_cube("one_way_transfer_gate_placeholder", 210.0, 40.0, 58.0)
        .translate(0.0, -DECK_Y / 2.0 + 286.0, DECK_Z + 29.0);
    let reject_pocket = centered_cube("sealed_tote_reject_hold_pocket", 252.0, 112.0, 46.0)
        .translate(DECK_X / 2.0 - 224.0, -DECK_Y / 2.0 + 344.0, DECK_Z + 23.0);
    let reject_recess = centered_cube("sealed_tote_reject_hold_recess", 218.0, 78.0, 48.0)
        .translate(DECK_X / 2.0 - 224.0, -DECK_Y / 2.0 + 344.0, DECK_Z + 27.0);

    lanes + barrier + one_way_gate + (reject_pocket - reject_recess)
}

fn lane_slot(name: String, x: f64, y: f64) -> Part {
    let plate = centered_cube(format!("{name}_floor_land"), LANE_SLOT_X, LANE_SLOT_Y, 8.0)
        .translate(x, y, DECK_Z + 4.0);
    let left_rail = centered_cube(format!("{name}_left_rail"), 12.0, LANE_SLOT_Y, LANE_WALL_Z)
        .translate(x - LANE_SLOT_X / 2.0, y, DECK_Z + LANE_WALL_Z / 2.0);
    let right_rail = centered_cube(format!("{name}_right_rail"), 12.0, LANE_SLOT_Y, LANE_WALL_Z)
        .translate(x + LANE_SLOT_X / 2.0, y, DECK_Z + LANE_WALL_Z / 2.0);
    let rear_stop = centered_cube(format!("{name}_rear_stop"), LANE_SLOT_X, 12.0, 28.0).translate(
        x,
        y + LANE_SLOT_Y / 2.0,
        DECK_Z + 14.0,
    );
    plate + left_rail + right_rail + rear_stop
}

fn transfer_tongue_rail_interface() -> Part {
    let tongue_center_y = DECK_Y / 2.0 - TONGUE_Y / 2.0 + TONGUE_REAR_OVERHANG;
    let tongue_base = centered_cube(
        "sterile_tote_transfer_tongue_base",
        TONGUE_X,
        TONGUE_Y,
        TONGUE_Z,
    )
    .translate(0.0, tongue_center_y, DECK_Z + TONGUE_Z / 2.0);

    let left_rail = transfer_rail("left", -1.0, tongue_center_y);
    let right_rail = transfer_rail("right", 1.0, tongue_center_y);
    let center_key = centered_cube(
        "sterile_tote_transfer_tongue_center_key",
        42.0,
        TONGUE_Y - 74.0,
        16.0,
    )
    .translate(0.0, tongue_center_y, DECK_Z + TONGUE_Z + 8.0);
    let rack_datum_stop = centered_cube(
        "incubator_rack_handoff_datum_stop",
        TONGUE_X - 86.0,
        20.0,
        46.0,
    )
    .translate(
        0.0,
        tongue_center_y + TONGUE_Y / 2.0 - 44.0,
        DECK_Z + TONGUE_Z + 23.0,
    );
    let shuttle_latch = centered_cube(
        "cassette_shuttle_airlock_latch_receiver",
        TONGUE_X - 190.0,
        24.0,
        34.0,
    )
    .translate(
        0.0,
        tongue_center_y - TONGUE_Y / 2.0 + 66.0,
        DECK_Z + TONGUE_Z + 17.0,
    );

    tongue_base
        + left_rail
        + right_rail
        + center_key
        + rack_datum_stop
        + shuttle_latch
        + rail_fiducials(tongue_center_y)
}

fn transfer_rail(name: &str, side: f64, tongue_center_y: f64) -> Part {
    let x = side * TRANSFER_RAIL_PITCH_X / 2.0;
    let rail = centered_cube(
        format!("{name}_transfer_slide_rail"),
        TRANSFER_RAIL_W,
        TONGUE_Y - 56.0,
        TRANSFER_RAIL_Z,
    )
    .translate(
        x,
        tongue_center_y,
        DECK_Z + TONGUE_Z + TRANSFER_RAIL_Z / 2.0,
    );
    let chamfer_relief = centered_cube(
        format!("{name}_transfer_slide_lead_in_relief"),
        TRANSFER_RAIL_W + 10.0,
        58.0,
        12.0,
    )
    .translate(
        x,
        tongue_center_y - TONGUE_Y / 2.0 + 42.0,
        DECK_Z + TONGUE_Z + TRANSFER_RAIL_Z - 2.0,
    );
    rail - chamfer_relief
}

fn rail_fiducials(tongue_center_y: f64) -> Part {
    let mut fiducials = Part::empty("sterile_tote_transfer_rail_fiducials");
    for (i, (x, y)) in [
        (
            -TRANSFER_RAIL_PITCH_X / 2.0,
            tongue_center_y - TONGUE_Y / 2.0 + 94.0,
        ),
        (
            TRANSFER_RAIL_PITCH_X / 2.0,
            tongue_center_y - TONGUE_Y / 2.0 + 94.0,
        ),
        (
            -TRANSFER_RAIL_PITCH_X / 2.0,
            tongue_center_y + TONGUE_Y / 2.0 - 94.0,
        ),
        (
            TRANSFER_RAIL_PITCH_X / 2.0,
            tongue_center_y + TONGUE_Y / 2.0 - 94.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        let disc = centered_cylinder(format!("transfer_rail_fiducial_disc_{i}"), 11.0, 4.0, 36)
            .translate(*x, *y, DECK_Z + TONGUE_Z + TRANSFER_RAIL_Z + 2.0);
        let center = centered_cylinder(format!("transfer_rail_fiducial_center_{i}"), 2.0, 6.0, 20)
            .translate(*x, *y, DECK_Z + TONGUE_Z + TRANSFER_RAIL_Z + 2.0);
        fiducials = fiducials + (disc - center);
    }
    fiducials
}

fn environmental_logger_pocket() -> Part {
    let mut pockets = Part::empty("sterile_tote_environmental_logger_pockets");
    for i in 0..LOGGER_POCKET_COUNT {
        let x = -DECK_X / 2.0 + 132.0;
        let y = -120.0 + i as f64 * 150.0;
        let body = centered_cube(
            format!("environmental_logger_pocket_{i}_body"),
            LOGGER_POCKET_X,
            LOGGER_POCKET_Y,
            LOGGER_POCKET_Z,
        )
        .translate(x, y, DECK_Z + LOGGER_POCKET_Z / 2.0);
        let recess = centered_cube(
            format!("environmental_logger_pocket_{i}_recess"),
            LOGGER_POCKET_X - 26.0,
            LOGGER_POCKET_Y - 22.0,
            LOGGER_POCKET_Z + 2.0,
        )
        .translate(x, y, DECK_Z + LOGGER_POCKET_Z / 2.0 + 5.0);
        let cable_exit = centered_cube(
            format!("environmental_logger_pocket_{i}_cable_exit"),
            18.0,
            LOGGER_POCKET_Y + 8.0,
            12.0,
        )
        .translate(x + LOGGER_POCKET_X / 2.0 - 18.0, y, DECK_Z + 12.0);
        pockets = pockets + (body - recess - cable_exit);
    }
    pockets + logger_probe_reference_lands()
}

fn logger_probe_reference_lands() -> Part {
    let mut lands = Part::empty("logger_probe_reference_lands");
    for (i, y) in [-210.0, -60.0, 90.0, 240.0].iter().enumerate() {
        let land = centered_cube(format!("logger_probe_reference_land_{i}"), 54.0, 28.0, 8.0)
            .translate(-DECK_X / 2.0 + 238.0, *y, DECK_Z + 4.0);
        lands = lands + land;
    }
    lands
}

fn tote_present_latch_sensors() -> Part {
    let mut sensors = Part::empty("sterile_tote_present_latch_sensor_placeholders");
    for (i, (x, y)) in tote_present_sensor_points().iter().enumerate() {
        let fork = optical_fork_sensor(format!("tote_present_sensor_{i}")).translate(
            *x,
            *y,
            DECK_Z + SENSOR_BLOCK_Z / 2.0,
        );
        sensors = sensors + fork;
    }
    for (i, (x, y)) in latch_sensor_points().iter().enumerate() {
        let block = centered_cube(
            format!("tote_latch_closed_sensor_block_{i}"),
            SENSOR_BLOCK_X,
            SENSOR_BLOCK_Y,
            SENSOR_BLOCK_Z,
        )
        .translate(*x, *y, DECK_Z + SENSOR_BLOCK_Z / 2.0);
        let target = centered_cube(format!("tote_latch_magnet_target_{i}"), 16.0, 8.0, 24.0)
            .translate(*x, *y + 22.0, DECK_Z + SENSOR_BLOCK_Z / 2.0);
        sensors = sensors + block + target;
    }
    sensors + sensor_cable_raceways()
}

fn optical_fork_sensor(name: String) -> Part {
    let base = centered_cube(format!("{name}_base"), SENSOR_BLOCK_X, SENSOR_BLOCK_Y, 12.0);
    let left = centered_cube(
        format!("{name}_left_fork"),
        8.0,
        SENSOR_BLOCK_Y,
        SENSOR_BLOCK_Z,
    )
    .translate(-13.0, 0.0, 5.0);
    let right = centered_cube(
        format!("{name}_right_fork"),
        8.0,
        SENSOR_BLOCK_Y,
        SENSOR_BLOCK_Z,
    )
    .translate(13.0, 0.0, 5.0);
    let beam_gap = centered_cube(format!("{name}_beam_gap"), 18.0, SENSOR_BLOCK_Y + 2.0, 16.0)
        .translate(0.0, 0.0, 8.0);
    base + left + right - beam_gap
}

fn sensor_cable_raceways() -> Part {
    let left_raceway = centered_cube(
        "left_sensor_cable_raceway",
        22.0,
        TOTE_OUTER_Y + 120.0,
        14.0,
    )
    .translate(-(TOTE_OUTER_X / 2.0 + 72.0), 0.0, DECK_Z + 7.0);
    let right_raceway = centered_cube(
        "right_sensor_cable_raceway",
        22.0,
        TOTE_OUTER_Y + 120.0,
        14.0,
    )
    .translate(TOTE_OUTER_X / 2.0 + 72.0, 0.0, DECK_Z + 7.0);
    left_raceway + right_raceway
}

fn handoff_clearance_gauges() -> Part {
    let shuttle_gauge = clearance_wireframe(
        "cassette_shuttle_handoff_clearance",
        HANDOFF_ENVELOPE_X,
        SHUTTLE_TONGUE_CLEARANCE_Y,
        HANDOFF_CLEARANCE_Z,
    )
    .translate(0.0, DECK_Y / 2.0 + 72.0, DECK_Z + HANDOFF_CLEARANCE_Z / 2.0);
    let rack_gauge = clearance_wireframe(
        "incubator_rack_handoff_clearance",
        HANDOFF_ENVELOPE_X + 70.0,
        RACK_HANDOFF_CLEARANCE_Y,
        HANDOFF_CLEARANCE_Z + 38.0,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 + 72.0 + SHUTTLE_TONGUE_CLEARANCE_Y,
        DECK_Z + HANDOFF_CLEARANCE_Z / 2.0 + 19.0,
    );
    let cassette_payload_gauge = rectangular_frame_xy(
        "sealed_cassette_payload_footprint_gauge",
        CASSETTE_X + 18.0,
        CASSETTE_Y + 18.0,
        6.0,
        CASSETTE_X - 18.0,
        CASSETTE_Y - 18.0,
    )
    .translate(0.0, 0.0, DECK_Z + RECEIVER_RAIL_Z + 6.0);
    shuttle_gauge + rack_gauge + cassette_payload_gauge
}

fn clearance_wireframe(name: &str, x: f64, y: f64, z: f64) -> Part {
    let bottom = rectangular_frame_xy(
        format!("{name}_bottom_frame"),
        x,
        y,
        KEEP_OUT_RAIL_W,
        x - 2.0 * KEEP_OUT_RAIL_W,
        y - 2.0 * KEEP_OUT_RAIL_W,
    )
    .translate(0.0, 0.0, -z / 2.0);
    let top = rectangular_frame_xy(
        format!("{name}_top_frame"),
        x,
        y,
        KEEP_OUT_RAIL_W,
        x - 2.0 * KEEP_OUT_RAIL_W,
        y - 2.0 * KEEP_OUT_RAIL_W,
    )
    .translate(0.0, 0.0, z / 2.0);
    let mut posts = Part::empty(format!("{name}_corner_posts"));
    for (i, (px, py)) in [
        (
            -(x / 2.0 - KEEP_OUT_RAIL_W / 2.0),
            -(y / 2.0 - KEEP_OUT_RAIL_W / 2.0),
        ),
        (
            x / 2.0 - KEEP_OUT_RAIL_W / 2.0,
            -(y / 2.0 - KEEP_OUT_RAIL_W / 2.0),
        ),
        (
            -(x / 2.0 - KEEP_OUT_RAIL_W / 2.0),
            y / 2.0 - KEEP_OUT_RAIL_W / 2.0,
        ),
        (
            x / 2.0 - KEEP_OUT_RAIL_W / 2.0,
            y / 2.0 - KEEP_OUT_RAIL_W / 2.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(
                format!("{name}_clearance_post_{i}"),
                KEEP_OUT_RAIL_W,
                KEEP_OUT_RAIL_W,
                z,
            )
            .translate(*px, *py, 0.0);
    }
    bottom + top + posts
}

fn robot_service_keepouts() -> Part {
    let front_robot_approach = clearance_wireframe(
        "front_robot_tote_approach_keepout",
        TOTE_OUTER_X + 360.0,
        ROBOT_FRONT_KEEP_OUT_Y,
        TOTE_OUTER_Z + 120.0,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 - ROBOT_FRONT_KEEP_OUT_Y / 2.0 + 44.0,
        DECK_Z + (TOTE_OUTER_Z + 120.0) / 2.0,
    );
    let rear_door_swing = clearance_wireframe(
        "rear_door_swing_service_keepout",
        DOOR_FRAME_X + 90.0,
        REAR_DOOR_SWING_KEEP_OUT_Y,
        DOOR_FRAME_Z + 80.0,
    )
    .translate(
        0.0,
        DOOR_CENTER_Y + REAR_DOOR_SWING_KEEP_OUT_Y / 2.0,
        DOOR_CENTER_Z,
    );
    let left_service = centered_cube(
        "left_tote_service_pull_keepout",
        SIDE_SERVICE_KEEP_OUT_X,
        DECK_Y - 120.0,
        190.0,
    )
    .translate(
        -DECK_X / 2.0 - SIDE_SERVICE_KEEP_OUT_X / 2.0 + 48.0,
        0.0,
        DECK_Z + 95.0,
    );
    let right_service = centered_cube(
        "right_return_lane_service_keepout",
        SIDE_SERVICE_KEEP_OUT_X,
        DECK_Y - 120.0,
        190.0,
    )
    .translate(
        DECK_X / 2.0 + SIDE_SERVICE_KEEP_OUT_X / 2.0 - 48.0,
        0.0,
        DECK_Z + 95.0,
    );
    let overhead_lift = clearance_wireframe(
        "overhead_tote_lift_clearance_keepout",
        TOTE_OUTER_X + 160.0,
        TOTE_OUTER_Y + 160.0,
        OVERHEAD_LIFT_KEEP_OUT_Z,
    )
    .translate(0.0, 0.0, DECK_Z + OVERHEAD_LIFT_KEEP_OUT_Z / 2.0);

    front_robot_approach + rear_door_swing + left_service + right_service + overhead_lift
}

fn rectangular_frame_xy(
    name: impl Into<String>,
    outer_x: f64,
    outer_y: f64,
    height_z: f64,
    inner_x: f64,
    inner_y: f64,
) -> Part {
    let name = name.into();
    let outer = centered_cube(format!("{name}_outer"), outer_x, outer_y, height_z);
    let inner = centered_cube(
        format!("{name}_inner_cut"),
        inner_x,
        inner_y,
        height_z + 2.0,
    );
    outer - inner
}

fn rectangular_frame_xz(
    name: impl Into<String>,
    outer_x: f64,
    depth_y: f64,
    outer_z: f64,
    inner_x: f64,
    inner_z: f64,
) -> Part {
    let name = name.into();
    let outer = centered_cube(format!("{name}_outer"), outer_x, depth_y, outer_z);
    let inner = centered_cube(format!("{name}_inner_cut"), inner_x, depth_y + 2.0, inner_z);
    outer - inner
}

fn deck_mount_points() -> [(f64, f64); 8] {
    [
        (-(DECK_X / 2.0 - 46.0), -(DECK_Y / 2.0 - 46.0)),
        (DECK_X / 2.0 - 46.0, -(DECK_Y / 2.0 - 46.0)),
        (-(DECK_X / 2.0 - 46.0), DECK_Y / 2.0 - 46.0),
        (DECK_X / 2.0 - 46.0, DECK_Y / 2.0 - 46.0),
        (0.0, -(DECK_Y / 2.0 - 46.0)),
        (0.0, DECK_Y / 2.0 - 46.0),
        (-(DECK_X / 2.0 - 46.0), 0.0),
        (DECK_X / 2.0 - 46.0, 0.0),
    ]
}

fn tote_locator_points() -> [(f64, f64); LOCATOR_COUNT] {
    [
        (-(TOTE_OUTER_X / 2.0 - 76.0), -(TOTE_OUTER_Y / 2.0 - 76.0)),
        (TOTE_OUTER_X / 2.0 - 76.0, -(TOTE_OUTER_Y / 2.0 - 76.0)),
        (-(TOTE_OUTER_X / 2.0 - 76.0), TOTE_OUTER_Y / 2.0 - 76.0),
        (TOTE_OUTER_X / 2.0 - 76.0, TOTE_OUTER_Y / 2.0 - 76.0),
    ]
}

fn identity_land_pose(index: usize) -> (f64, f64, bool) {
    match index {
        0..=3 => (
            -TOTE_OUTER_X / 2.0 + 90.0 + index as f64 * 150.0,
            -TOTE_OUTER_Y / 2.0 - 62.0,
            false,
        ),
        4..=7 => (
            TOTE_OUTER_X / 2.0 + 62.0,
            -TOTE_OUTER_Y / 2.0 + 90.0 + (index - 4) as f64 * 140.0,
            true,
        ),
        8..=11 => (
            -TOTE_OUTER_X / 2.0 + 90.0 + (index - 8) as f64 * 150.0,
            TOTE_OUTER_Y / 2.0 + 62.0,
            false,
        ),
        _ => unreachable!("identity index outside export count"),
    }
}

fn lane_x(index: usize, count: usize, side: f64) -> f64 {
    let pitch = LANE_SLOT_X + 28.0;
    let local = -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch;
    side * (DECK_X / 4.0) + local
}

fn tote_present_sensor_points() -> [(f64, f64); TOTE_PRESENT_SENSOR_COUNT] {
    [
        (-(TOTE_OUTER_X / 2.0 + 28.0), -(TOTE_OUTER_Y / 2.0 - 92.0)),
        (TOTE_OUTER_X / 2.0 + 28.0, -(TOTE_OUTER_Y / 2.0 - 92.0)),
        (-(TOTE_OUTER_X / 2.0 + 28.0), TOTE_OUTER_Y / 2.0 - 92.0),
        (TOTE_OUTER_X / 2.0 + 28.0, TOTE_OUTER_Y / 2.0 - 92.0),
    ]
}

fn latch_sensor_points() -> [(f64, f64); LATCH_SENSOR_COUNT] {
    [
        (-(TOTE_OUTER_X / 2.0 - 116.0), -(TOTE_OUTER_Y / 2.0 + 36.0)),
        (TOTE_OUTER_X / 2.0 - 116.0, -(TOTE_OUTER_Y / 2.0 + 36.0)),
        (-(TOTE_OUTER_X / 2.0 - 116.0), TOTE_OUTER_Y / 2.0 + 36.0),
        (TOTE_OUTER_X / 2.0 - 116.0, TOTE_OUTER_Y / 2.0 + 36.0),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_plan_contains_ten_parts_plus_assembly() {
        assert_eq!(PART_NAMES.len(), 10);
        let mut names = PART_NAMES.to_vec();
        names.push(ASSEMBLY_NAME);
        assert_eq!(names.len(), 11);
        assert!(names.iter().all(|name| !name.is_empty()));
        assert_eq!(
            names.iter().filter(|name| **name == ASSEMBLY_NAME).count(),
            1
        );
    }

    #[test]
    fn tote_receiver_clears_twenty_chip_sealed_cassette() {
        assert_eq!(COLS * ROWS, 20);
        assert!((ARRAY_X - 526.04).abs() < 0.01);
        assert!((ARRAY_Y - 447.40).abs() < 0.01);
        assert!(TOTE_INNER_X > CASSETTE_X + 180.0);
        assert!(TOTE_INNER_Y > CASSETTE_Y + 168.0);
        assert!(TOTE_INNER_Z > CASSETTE_Z + 190.0);
        assert!(TOTE_OUTER_X + 2.0 * RECEIVER_RAIL_W < DECK_X);
        assert!(TOTE_OUTER_Y + 2.0 * RECEIVER_RAIL_W < DECK_Y);
    }

    #[test]
    fn door_and_gasket_envelope_cover_tote_opening() {
        assert!(DOOR_FRAME_X > TOTE_OUTER_X + 120.0);
        assert!(DOOR_OPENING_X > TOTE_INNER_X + 30.0);
        assert!(DOOR_OPENING_Z > TOTE_INNER_Z + 60.0);
        assert!(DOOR_FRAME_Z > DOOR_OPENING_Z + 90.0);
        assert!(DOOR_CENTER_Y > TOTE_OUTER_Y / 2.0);
        assert!(GASKET_LAND_Y < DOOR_FRAME_Y / 3.0);
    }

    #[test]
    fn transfer_interface_matches_shuttle_and_rack_clearance_targets() {
        assert!(TRANSFER_RAIL_PITCH_X > CASSETTE_X + 70.0);
        assert!(TONGUE_X > TRANSFER_RAIL_PITCH_X + 2.0 * TRANSFER_RAIL_W);
        assert!(TONGUE_REAR_OVERHANG > RACK_HANDOFF_CLEARANCE_Y - 20.0);
        assert!(HANDOFF_ENVELOPE_X > CASSETTE_X + 40.0);
        assert!(HANDOFF_ENVELOPE_Y > CASSETTE_Y + 40.0);
        assert!(HANDOFF_CLEARANCE_Z > REVC_TOTAL_HEIGHT + 70.0);
    }

    #[test]
    fn identity_sensors_and_lanes_are_counted_and_segregated() {
        assert_eq!(IDENTITY_LAND_COUNT, 12);
        assert_eq!(LOGGER_POCKET_COUNT, 2);
        assert_eq!(TOTE_PRESENT_SENSOR_COUNT, LOCATOR_COUNT);
        assert_eq!(LATCH_SENSOR_COUNT, LOCATOR_COUNT);
        assert_eq!(CLEAN_LANE_COUNT + DIRTY_LANE_COUNT, 5);
        assert!(lane_x(0, CLEAN_LANE_COUNT, -1.0) < 0.0);
        assert!(lane_x(0, DIRTY_LANE_COUNT, 1.0) > 0.0);
        assert!(SEGREGATION_BARRIER_Y >= 30.0);
    }
}
