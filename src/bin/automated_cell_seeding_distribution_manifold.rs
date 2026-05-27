#![allow(dead_code)]

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
use vcad::{centered_cube, centered_cylinder, Part};

// Automated cell seeding distribution manifold for a closed 20-chip cassette.
//
// Intent:
// - Keep the sterile wetted path single-use and closed from cell suspension
//   bag/reservoir interface through 20 chip outlets and return/waste handling.
// - Represent reproducibility controls that matter during automated seeding:
//   gentle mixing, bubble removal, equalized 20-way routing, channel isolation,
//   priming, pressure/shear sensing, calibration coupons, and robotic access.
// - Model purchased sensors/valves as pockets and envelopes only. This is a
//   mechanical CAD planning module, not a validated culture protocol.

const COLS: usize = 4;
const ROWS: usize = 5;
const CHANNELS: usize = COLS * ROWS;
const ISOLATION_VALVES: usize = CHANNELS;
const PRIME_PORTS: usize = CHANNELS;
const SENSOR_POCKETS: usize = CHANNELS;
const CALIBRATION_COUPONS: usize = 5;

const CHIP_GUTTER: f64 = 5.0;
const PITCH_X: f64 = REVC_CHIP_LENGTH + CHIP_GUTTER;
const PITCH_Y: f64 = REVC_CHIP_WIDTH + CHIP_GUTTER;
const ARRAY_X: f64 = COLS as f64 * REVC_CHIP_LENGTH + (COLS as f64 - 1.0) * CHIP_GUTTER;
const ARRAY_Y: f64 = ROWS as f64 * REVC_CHIP_WIDTH + (ROWS as f64 - 1.0) * CHIP_GUTTER;

const CASSETTE_MARGIN_X: f64 = 32.0;
const CASSETTE_MARGIN_Y: f64 = 32.0;
const CASSETTE_X: f64 = ARRAY_X + 2.0 * CASSETTE_MARGIN_X;
const CASSETTE_Y: f64 = ARRAY_Y + 2.0 * CASSETTE_MARGIN_Y;

const FLUID_CASSETTE_X: f64 = CASSETTE_X + 142.0;
const FLUID_CASSETTE_Y: f64 = CASSETTE_Y + 124.0;
const MODULE_X: f64 = FLUID_CASSETTE_X + 118.0;
const MODULE_Y: f64 = FLUID_CASSETTE_Y + 112.0;

const BASE_Z: f64 = 12.0;
const DATUM_PAD_Z: f64 = 8.0;
const FLUID_CASSETTE_Z: f64 = 18.0;
const COVER_Z: f64 = 8.0;
const CHANNEL_D: f64 = 2.2;
const CHANNEL_Z: f64 = 4.0;
const OUTLET_BORE_D: f64 = 1.8;
const OUTLET_ORING_D: f64 = 4.0;
const LANE_SPACING: f64 = 7.2;

const RESERVOIR_X: f64 = -FLUID_CASSETTE_X / 2.0 + 68.0;
const RESERVOIR_Y: f64 = FLUID_CASSETTE_Y / 2.0 - 112.0;
const RESERVOIR_INTERFACE_D: f64 = 72.0;
const RESERVOIR_INTERFACE_Z: f64 = 42.0;
const MIXING_PLENUM_D: f64 = 26.0;

const DEGASSER_X: f64 = -FLUID_CASSETTE_X / 2.0 + 190.0;
const DEGASSER_Y: f64 = FLUID_CASSETTE_Y / 2.0 - 72.0;
const DEGASSER_X_LEN: f64 = 172.0;
const DEGASSER_Y_LEN: f64 = 72.0;
const DEGASSER_Z: f64 = 30.0;

const RETURN_MANIFOLD_Y: f64 = -FLUID_CASSETTE_Y / 2.0 + 56.0;
const RETURN_MANIFOLD_X_LEN: f64 = FLUID_CASSETTE_X - 136.0;
const WASTE_DIVERTER_X: f64 = FLUID_CASSETTE_X / 2.0 - 92.0;
const WASTE_DIVERTER_Y: f64 = RETURN_MANIFOLD_Y;
const WASTE_DIVERTER_Z: f64 = 34.0;

const SENSOR_BLOCK_X: f64 = 34.0;
const SENSOR_BLOCK_Y: f64 = 24.0;
const SENSOR_BLOCK_Z: f64 = 14.0;
const ISOLATION_BLOCK_X: f64 = 24.0;
const ISOLATION_BLOCK_Y: f64 = 16.0;
const ISOLATION_BLOCK_Z: f64 = 12.0;
const PRIME_PORT_D: f64 = 8.0;
const PRIME_PORT_Z: f64 = 18.0;

const ROBOT_KEEP_OUT_X: f64 = CASSETTE_X + 144.0;
const ROBOT_KEEP_OUT_Y: f64 = CASSETTE_Y + 126.0;
const ROBOT_KEEPOUT_Z: f64 = 158.0;
const ROBOT_FRONT_APPROACH: f64 = 96.0;
const GRIPPER_FINGER_CLEARANCE: f64 = 34.0;

const DATUM_PIN_D: f64 = 4.0;
const DATUM_SOCKET_D: f64 = 4.25;
const DATUM_SOCKET_DEPTH: f64 = 7.0;

const CAL_STRIP_X: f64 = 344.0;
const CAL_STRIP_Y: f64 = 58.0;
const CAL_STRIP_Z: f64 = 12.0;
const CAL_STRIP_X_POS: f64 = -FLUID_CASSETTE_X / 2.0 + 224.0;
const CAL_STRIP_Y_POS: f64 = -FLUID_CASSETTE_Y / 2.0 + 38.0;

const EQUALIZATION_RESERVE_LENGTH: f64 = 42.0;
const OUTLET_DROP_LENGTH: f64 = 14.0;

fn main() {
    std::fs::create_dir_all("output").unwrap();

    let datum = datum_plate();
    datum
        .write_stl("output/automated_cell_seeding_distribution_manifold_datum_plate.stl")
        .unwrap();
    println!("Exported: output/automated_cell_seeding_distribution_manifold_datum_plate.stl");

    let cassette = closed_fluid_path_cassette();
    cassette
        .write_stl(
            "output/automated_cell_seeding_distribution_manifold_closed_fluid_path_cassette.stl",
        )
        .unwrap();
    println!(
        "Exported: output/automated_cell_seeding_distribution_manifold_closed_fluid_path_cassette.stl"
    );

    let reservoir = reservoir_mixer_interface();
    reservoir
        .write_stl(
            "output/automated_cell_seeding_distribution_manifold_reservoir_mixer_interface.stl",
        )
        .unwrap();
    println!(
        "Exported: output/automated_cell_seeding_distribution_manifold_reservoir_mixer_interface.stl"
    );

    let channels = equalized_twenty_way_channel_plate();
    channels
        .write_stl(
            "output/automated_cell_seeding_distribution_manifold_equalized_20_way_channels.stl",
        )
        .unwrap();
    println!(
        "Exported: output/automated_cell_seeding_distribution_manifold_equalized_20_way_channels.stl"
    );

    let valves = isolation_prime_port_array();
    valves
        .write_stl("output/automated_cell_seeding_distribution_manifold_isolation_prime_ports.stl")
        .unwrap();
    println!(
        "Exported: output/automated_cell_seeding_distribution_manifold_isolation_prime_ports.stl"
    );

    let degas = bubble_trap_degas_return();
    degas
        .write_stl(
            "output/automated_cell_seeding_distribution_manifold_bubble_trap_degas_return.stl",
        )
        .unwrap();
    println!(
        "Exported: output/automated_cell_seeding_distribution_manifold_bubble_trap_degas_return.stl"
    );

    let sensors = pressure_shear_sensor_pockets();
    sensors
        .write_stl(
            "output/automated_cell_seeding_distribution_manifold_pressure_shear_sensor_pockets.stl",
        )
        .unwrap();
    println!(
        "Exported: output/automated_cell_seeding_distribution_manifold_pressure_shear_sensor_pockets.stl"
    );

    let keepout = robotic_load_unload_keepout_frame();
    keepout
        .write_stl("output/automated_cell_seeding_distribution_manifold_robotic_keepout_frame.stl")
        .unwrap();
    println!(
        "Exported: output/automated_cell_seeding_distribution_manifold_robotic_keepout_frame.stl"
    );

    let coupons = calibration_coupon_strip();
    coupons
        .write_stl(
            "output/automated_cell_seeding_distribution_manifold_calibration_coupon_strip.stl",
        )
        .unwrap();
    println!(
        "Exported: output/automated_cell_seeding_distribution_manifold_calibration_coupon_strip.stl"
    );

    let assembly = datum
        + cassette.translate(
            0.0,
            0.0,
            BASE_Z / 2.0 + FLUID_CASSETTE_Z / 2.0 + DATUM_PAD_Z,
        )
        + reservoir.translate(0.0, 0.0, BASE_Z / 2.0 + FLUID_CASSETTE_Z + DATUM_PAD_Z)
        + channels.translate(0.0, 0.0, BASE_Z / 2.0 + FLUID_CASSETTE_Z + DATUM_PAD_Z)
        + valves.translate(0.0, 0.0, BASE_Z / 2.0 + FLUID_CASSETTE_Z + DATUM_PAD_Z)
        + degas.translate(0.0, 0.0, BASE_Z / 2.0 + FLUID_CASSETTE_Z + DATUM_PAD_Z)
        + sensors.translate(0.0, 0.0, BASE_Z / 2.0 + FLUID_CASSETTE_Z + DATUM_PAD_Z)
        + keepout.translate(0.0, 0.0, BASE_Z / 2.0)
        + coupons.translate(0.0, 0.0, BASE_Z / 2.0 + FLUID_CASSETTE_Z + DATUM_PAD_Z);

    assembly
        .write_stl("output/automated_cell_seeding_distribution_manifold_assembly.stl")
        .unwrap();
    println!("Exported: output/automated_cell_seeding_distribution_manifold_assembly.stl");

    println!(
        "Automated cell seeding distribution manifold: {:.0}mm x {:.0}mm module, {:.0}mm x {:.0}mm single-use cassette, {} equalized outlets over a {}x{} Rev C chip cassette, {} isolation valves, {} prime ports, {} pressure/shear pockets, {:.1}mm target branch length, {:.0}mm robotic Z keepout.",
        MODULE_X,
        MODULE_Y,
        FLUID_CASSETTE_X,
        FLUID_CASSETTE_Y,
        CHANNELS,
        COLS,
        ROWS,
        ISOLATION_VALVES,
        PRIME_PORTS,
        SENSOR_POCKETS,
        target_equalized_path_length(),
        ROBOT_KEEPOUT_Z
    );
}

fn datum_plate() -> Part {
    let deck = centered_cube(
        "cell_seeding_manifold_datum_plate",
        MODULE_X,
        MODULE_Y,
        BASE_Z,
    );

    let cassette_relief = centered_cube(
        "cell_seeding_manifold_single_use_cassette_relief",
        FLUID_CASSETTE_X + 14.0,
        FLUID_CASSETTE_Y + 14.0,
        5.0,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0 - 2.0);

    let robot_lift_window = centered_cube(
        "cell_seeding_manifold_robot_lift_finger_window",
        CASSETTE_X + 62.0,
        GRIPPER_FINGER_CLEARANCE,
        7.0,
    )
    .translate(0.0, -CASSETTE_Y / 2.0 - 42.0, BASE_Z / 2.0 - 1.5);

    let rear_drain_sump = centered_cube(
        "cell_seeding_manifold_rear_return_spill_sump",
        FLUID_CASSETTE_X - 110.0,
        38.0,
        7.0,
    )
    .translate(0.0, RETURN_MANIFOLD_Y, BASE_Z / 2.0 - 1.5);

    let mut mount_holes = Part::empty("cell_seeding_manifold_mount_holes");
    for (i, (x, y)) in deck_mount_points().iter().enumerate() {
        let hole = centered_cylinder(
            format!("cell_seeding_manifold_m6_mount_clearance_{i}"),
            6.6 / 2.0,
            BASE_Z + 3.0,
            24,
        )
        .translate(*x, *y, 0.0);
        let slot = centered_cube(
            format!("cell_seeding_manifold_m6_mount_slot_{i}"),
            24.0,
            6.8,
            BASE_Z + 3.0,
        )
        .translate(*x, *y, 0.0);
        mount_holes = mount_holes + hole + slot;
    }

    deck - cassette_relief - robot_lift_window - rear_drain_sump - mount_holes
        + datum_kinematic_pads()
        + cassette_edge_stops()
        + base_splash_rails()
}

fn datum_kinematic_pads() -> Part {
    let mut pads = Part::empty("cell_seeding_manifold_kinematic_datum_pads");
    for (i, (x, y)) in datum_points().iter().enumerate() {
        let pad = centered_cylinder(
            format!("cell_seeding_manifold_datum_pad_{i}"),
            14.0 / 2.0,
            DATUM_PAD_Z,
            32,
        )
        .translate(*x, *y, BASE_Z / 2.0 + DATUM_PAD_Z / 2.0);
        let pin = centered_cylinder(
            format!("cell_seeding_manifold_datum_pin_{i}"),
            DATUM_PIN_D / 2.0,
            DATUM_PAD_Z + 6.0,
            24,
        )
        .translate(*x, *y, BASE_Z / 2.0 + DATUM_PAD_Z + 3.0);
        pads = pads + pad + pin;
    }
    pads
}

fn cassette_edge_stops() -> Part {
    let left = centered_cube(
        "cell_seeding_manifold_left_cassette_hardstop",
        12.0,
        CASSETTE_Y + 34.0,
        18.0,
    )
    .translate(-CASSETTE_X / 2.0 - 12.0, 0.0, BASE_Z / 2.0 + 9.0);
    let rear = centered_cube(
        "cell_seeding_manifold_rear_cassette_hardstop",
        CASSETTE_X + 36.0,
        12.0,
        18.0,
    )
    .translate(0.0, CASSETTE_Y / 2.0 + 12.0, BASE_Z / 2.0 + 9.0);
    let keyed_corner = centered_cube(
        "cell_seeding_manifold_single_use_keyed_corner_stop",
        42.0,
        42.0,
        22.0,
    )
    .translate(
        -CASSETTE_X / 2.0 - 16.0,
        CASSETTE_Y / 2.0 + 16.0,
        BASE_Z / 2.0 + 11.0,
    );

    left + rear + keyed_corner
}

fn base_splash_rails() -> Part {
    let front = centered_cube(
        "cell_seeding_manifold_front_splash_rail",
        MODULE_X - 96.0,
        12.0,
        16.0,
    )
    .translate(0.0, -MODULE_Y / 2.0 + 24.0, BASE_Z / 2.0 + 8.0);
    let rear = centered_cube(
        "cell_seeding_manifold_rear_splash_rail",
        MODULE_X - 96.0,
        12.0,
        16.0,
    )
    .translate(0.0, MODULE_Y / 2.0 - 24.0, BASE_Z / 2.0 + 8.0);
    let left = centered_cube(
        "cell_seeding_manifold_left_splash_rail",
        12.0,
        MODULE_Y - 96.0,
        16.0,
    )
    .translate(-MODULE_X / 2.0 + 24.0, 0.0, BASE_Z / 2.0 + 8.0);
    let right = centered_cube(
        "cell_seeding_manifold_right_splash_rail",
        12.0,
        MODULE_Y - 96.0,
        16.0,
    )
    .translate(MODULE_X / 2.0 - 24.0, 0.0, BASE_Z / 2.0 + 8.0);

    front + rear + left + right
}

fn closed_fluid_path_cassette() -> Part {
    let body = centered_cube(
        "cell_seeding_single_use_closed_fluid_path_body",
        FLUID_CASSETTE_X,
        FLUID_CASSETTE_Y,
        FLUID_CASSETTE_Z,
    );
    let cover = centered_cube(
        "cell_seeding_single_use_welded_cover_plate",
        FLUID_CASSETTE_X - 28.0,
        FLUID_CASSETTE_Y - 28.0,
        COVER_Z,
    )
    .translate(0.0, 0.0, FLUID_CASSETTE_Z / 2.0 + COVER_Z / 2.0);

    let mut cuts = Part::empty("cell_seeding_single_use_body_cuts");
    for (idx, x, y) in outlet_positions() {
        let outlet = centered_cylinder(
            format!("cell_seeding_channel_{idx}_chip_outlet_bore"),
            OUTLET_BORE_D / 2.0,
            FLUID_CASSETTE_Z + COVER_Z + 4.0,
            24,
        )
        .translate(x, y, COVER_Z / 2.0);
        let oring = centered_cylinder(
            format!("cell_seeding_channel_{idx}_outlet_oring_counterbore"),
            OUTLET_ORING_D / 2.0,
            3.0,
            28,
        )
        .translate(x, y, -FLUID_CASSETTE_Z / 2.0 + 1.0);
        cuts = cuts + outlet + oring;
    }

    for (i, (x, y)) in datum_points().iter().enumerate() {
        cuts = cuts
            + centered_cylinder(
                format!("cell_seeding_single_use_datum_socket_{i}"),
                DATUM_SOCKET_D / 2.0,
                DATUM_SOCKET_DEPTH,
                24,
            )
            .translate(*x, *y, -FLUID_CASSETTE_Z / 2.0 + DATUM_SOCKET_DEPTH / 2.0);
    }

    let bag_spike_socket = centered_cylinder(
        "cell_seeding_single_use_reservoir_spike_socket",
        10.0 / 2.0,
        FLUID_CASSETTE_Z + COVER_Z + 4.0,
        30,
    )
    .translate(RESERVOIR_X, RESERVOIR_Y, COVER_Z / 2.0);
    let waste_socket = centered_cylinder(
        "cell_seeding_single_use_waste_return_socket",
        8.0 / 2.0,
        FLUID_CASSETTE_Z + COVER_Z + 4.0,
        28,
    )
    .translate(WASTE_DIVERTER_X, WASTE_DIVERTER_Y, COVER_Z / 2.0);
    cuts = cuts + bag_spike_socket + waste_socket;

    body + cover - cuts + welded_perimeter_lips() + chip_outlet_gasket_lands()
}

fn welded_perimeter_lips() -> Part {
    let front = centered_cube(
        "cell_seeding_single_use_front_weld_lip",
        FLUID_CASSETTE_X - 34.0,
        9.0,
        8.0,
    )
    .translate(
        0.0,
        -FLUID_CASSETTE_Y / 2.0 + 18.0,
        FLUID_CASSETTE_Z / 2.0 + COVER_Z + 4.0,
    );
    let rear = centered_cube(
        "cell_seeding_single_use_rear_weld_lip",
        FLUID_CASSETTE_X - 34.0,
        9.0,
        8.0,
    )
    .translate(
        0.0,
        FLUID_CASSETTE_Y / 2.0 - 18.0,
        FLUID_CASSETTE_Z / 2.0 + COVER_Z + 4.0,
    );
    let left = centered_cube(
        "cell_seeding_single_use_left_weld_lip",
        9.0,
        FLUID_CASSETTE_Y - 34.0,
        8.0,
    )
    .translate(
        -FLUID_CASSETTE_X / 2.0 + 18.0,
        0.0,
        FLUID_CASSETTE_Z / 2.0 + COVER_Z + 4.0,
    );
    let right = centered_cube(
        "cell_seeding_single_use_right_weld_lip",
        9.0,
        FLUID_CASSETTE_Y - 34.0,
        8.0,
    )
    .translate(
        FLUID_CASSETTE_X / 2.0 - 18.0,
        0.0,
        FLUID_CASSETTE_Z / 2.0 + COVER_Z + 4.0,
    );

    front + rear + left + right
}

fn chip_outlet_gasket_lands() -> Part {
    let mut lands = Part::empty("cell_seeding_single_use_chip_outlet_gasket_lands");
    for (idx, x, y) in outlet_positions() {
        let land = centered_cylinder(
            format!("cell_seeding_channel_{idx}_outlet_gasket_land"),
            7.0 / 2.0,
            1.4,
            32,
        )
        .translate(x, y, -FLUID_CASSETTE_Z / 2.0 - 0.7);
        lands = lands + land;
    }
    lands
}

fn reservoir_mixer_interface() -> Part {
    let outer = centered_cylinder(
        "cell_seeding_reservoir_gentle_mixer_outer_cup",
        RESERVOIR_INTERFACE_D / 2.0,
        RESERVOIR_INTERFACE_Z,
        64,
    );
    let well = centered_cylinder(
        "cell_seeding_reservoir_gentle_mixer_low_shear_well",
        (RESERVOIR_INTERFACE_D - 12.0) / 2.0,
        RESERVOIR_INTERFACE_Z - 8.0,
        64,
    )
    .translate(0.0, 0.0, 6.0);
    let bag_spike = centered_cylinder(
        "cell_seeding_reservoir_sterile_bag_spike_clearance",
        10.0 / 2.0,
        RESERVOIR_INTERFACE_Z + 8.0,
        32,
    )
    .translate(-12.0, 0.0, 0.0);
    let vent_filter = centered_cylinder(
        "cell_seeding_reservoir_vent_filter_boss",
        9.0 / 2.0,
        16.0,
        30,
    )
    .translate(20.0, 0.0, RESERVOIR_INTERFACE_Z / 2.0 + 8.0);

    let cup = outer - well - bag_spike + vent_filter + reservoir_swirl_baffles();

    let stir_bar_cradle = centered_cube(
        "cell_seeding_reservoir_noncontact_stir_bar_cradle",
        54.0,
        14.0,
        8.0,
    )
    .translate(0.0, -28.0, -RESERVOIR_INTERFACE_Z / 2.0 - 4.0);
    let interface_tabs = reservoir_interface_tabs();

    let feed = tube_run_x(
        "cell_seeding_reservoir_to_mixing_plenum_feed",
        RESERVOIR_X + RESERVOIR_INTERFACE_D / 2.0,
        0.0,
        RESERVOIR_Y,
        CHANNEL_Z,
        CHANNEL_D,
    );
    let plenum = centered_cylinder(
        "cell_seeding_central_low_shear_mixing_plenum",
        MIXING_PLENUM_D / 2.0,
        10.0,
        48,
    )
    .translate(0.0, 0.0, CHANNEL_Z);

    cup.translate(RESERVOIR_X, RESERVOIR_Y, RESERVOIR_INTERFACE_Z / 2.0)
        + stir_bar_cradle.translate(RESERVOIR_X, RESERVOIR_Y, RESERVOIR_INTERFACE_Z / 2.0)
        + interface_tabs.translate(RESERVOIR_X, RESERVOIR_Y, RESERVOIR_INTERFACE_Z / 2.0)
        + feed
        + plenum
}

fn reservoir_swirl_baffles() -> Part {
    let mut baffles = Part::empty("cell_seeding_reservoir_low_shear_swirl_baffles");
    for i in 0..4 {
        let angle = i as f64 * 90.0 + 28.0;
        let baffle = centered_cube(
            format!("cell_seeding_reservoir_swirl_baffle_{i}"),
            4.0,
            26.0,
            20.0,
        )
        .rotate(0.0, 0.0, angle)
        .translate(0.0, 16.0, 0.0);
        baffles = baffles + baffle;
    }
    baffles
}

fn reservoir_interface_tabs() -> Part {
    let mut tabs = Part::empty("cell_seeding_reservoir_interface_tabs");
    for (i, y) in [-42.0, 42.0].iter().enumerate() {
        let tab = centered_cube(
            format!("cell_seeding_reservoir_clamp_tab_{i}"),
            42.0,
            20.0,
            10.0,
        )
        .translate(0.0, *y, -14.0);
        let screw = centered_cylinder(
            format!("cell_seeding_reservoir_clamp_tab_screw_{i}"),
            3.4 / 2.0,
            14.0,
            20,
        )
        .translate(0.0, *y, -14.0);
        tabs = tabs + (tab - screw);
    }
    tabs
}

fn equalized_twenty_way_channel_plate() -> Part {
    let mut channels = Part::empty("cell_seeding_equalized_twenty_way_channels");

    let feed_header = centered_cylinder(
        "cell_seeding_central_distribution_plenum",
        MIXING_PLENUM_D / 2.0,
        8.0,
        48,
    )
    .translate(0.0, 0.0, CHANNEL_Z);
    channels = channels + feed_header;

    for row in 0..ROWS {
        for col in 0..COLS {
            let idx = channel_index(row, col);
            let x = chip_x(col);
            let y = chip_y(row);
            let tap_x = channel_tap_x(row, col);
            let lane_z = CHANNEL_Z + 0.18 * idx as f64;

            let fanout = tube_run_x(
                &format!("cell_seeding_channel_{idx}_plenum_to_lane_tap"),
                0.0,
                tap_x,
                0.0,
                lane_z,
                CHANNEL_D,
            );
            let row_run = tube_run_y(
                &format!("cell_seeding_channel_{idx}_lane_tap_to_row"),
                tap_x,
                0.0,
                y,
                lane_z,
                CHANNEL_D,
            );
            let chip_run = tube_run_x(
                &format!("cell_seeding_channel_{idx}_row_to_chip_outlet"),
                tap_x,
                x,
                y,
                lane_z,
                CHANNEL_D,
            );
            let outlet_drop = centered_cylinder(
                format!("cell_seeding_channel_{idx}_vertical_outlet_drop"),
                OUTLET_BORE_D / 2.0,
                OUTLET_DROP_LENGTH,
                24,
            )
            .translate(x, y, lane_z - OUTLET_DROP_LENGTH / 2.0);
            let equalizer = equalization_delay_line(row, col, lane_z);

            channels = channels + fanout + row_run + chip_run + outlet_drop + equalizer;
        }
    }

    channels + row_balance_ribs()
}

fn equalization_delay_line(row: usize, col: usize, z: f64) -> Part {
    let idx = channel_index(row, col);
    let comp = compensation_length(row, col);
    let x = chip_x(col);
    let y = chip_y(row);
    let side = if x >= 0.0 { -1.0 } else { 1.0 };
    let lane_y = y + if row >= ROWS / 2 { -15.0 } else { 15.0 };
    let folded_len = (comp / 4.0).clamp(12.0, 78.0);
    let fold_offset = 10.0 + (idx % COLS) as f64 * 3.2;

    let a = tube_run_x(
        &format!("cell_seeding_channel_{idx}_equalizer_serpentine_a"),
        x,
        x + side * folded_len,
        lane_y,
        z,
        CHANNEL_D,
    );
    let b = tube_run_y(
        &format!("cell_seeding_channel_{idx}_equalizer_serpentine_b"),
        x + side * folded_len,
        lane_y,
        lane_y + fold_offset,
        z,
        CHANNEL_D,
    );
    let c = tube_run_x(
        &format!("cell_seeding_channel_{idx}_equalizer_serpentine_c"),
        x + side * folded_len,
        x + side * (folded_len * 0.25),
        lane_y + fold_offset,
        z,
        CHANNEL_D,
    );
    let coupon = centered_cube(
        format!("cell_seeding_channel_{idx}_equalization_resistance_coupon"),
        16.0 + comp / 18.0,
        6.0,
        4.0,
    )
    .translate(
        x + side * (folded_len * 0.52),
        lane_y + fold_offset,
        z + 3.0,
    );

    a + b + c + coupon
}

fn row_balance_ribs() -> Part {
    let mut ribs = Part::empty("cell_seeding_row_balance_reference_ribs");
    for row in 0..ROWS {
        let y = chip_y(row);
        let rib = centered_cube(
            format!("cell_seeding_row_{row}_equal_pressure_reference_rib"),
            CASSETTE_X - 64.0,
            3.0,
            5.0,
        )
        .translate(0.0, y, CHANNEL_Z + 8.0);
        ribs = ribs + rib;
    }
    ribs
}

fn isolation_prime_port_array() -> Part {
    let mut array = Part::empty("cell_seeding_isolation_and_prime_port_array");
    for (idx, x, y) in outlet_positions() {
        let valve = isolation_valve(idx).translate(x - 21.0, y + 18.0, ISOLATION_BLOCK_Z / 2.0);
        let prime = prime_port(idx).translate(x + 21.0, y - 18.0, PRIME_PORT_Z / 2.0);
        let purge = tube_run_y(
            &format!("cell_seeding_channel_{idx}_prime_to_return_microchannel"),
            x + 21.0,
            y - 18.0,
            RETURN_MANIFOLD_Y,
            CHANNEL_Z + 7.0,
            CHANNEL_D * 0.8,
        );
        array = array + valve + prime + purge;
    }
    array + prime_return_header()
}

fn isolation_valve(idx: usize) -> Part {
    let body = centered_cube(
        format!("cell_seeding_channel_{idx}_pinch_isolation_valve_body"),
        ISOLATION_BLOCK_X,
        ISOLATION_BLOCK_Y,
        ISOLATION_BLOCK_Z,
    );
    let pinch_window = centered_cube(
        format!("cell_seeding_channel_{idx}_pinch_valve_actuator_window"),
        8.0,
        ISOLATION_BLOCK_Y + 2.0,
        7.0,
    );
    let tube = centered_cylinder(
        format!("cell_seeding_channel_{idx}_pinch_valve_tube_clearance"),
        CHANNEL_D / 2.0,
        ISOLATION_BLOCK_X + 3.0,
        18,
    )
    .rotate(0.0, 90.0, 0.0);

    body - pinch_window - tube
}

fn prime_port(idx: usize) -> Part {
    let tower = centered_cylinder(
        format!("cell_seeding_channel_{idx}_capped_prime_luer_tower"),
        PRIME_PORT_D / 2.0,
        PRIME_PORT_Z,
        28,
    );
    let bore = centered_cylinder(
        format!("cell_seeding_channel_{idx}_prime_luer_bore"),
        3.0 / 2.0,
        PRIME_PORT_Z + 2.0,
        20,
    );
    let cap_tether = centered_cube(
        format!("cell_seeding_channel_{idx}_prime_cap_tether_tab"),
        14.0,
        4.0,
        4.0,
    )
    .translate(8.0, 0.0, PRIME_PORT_Z / 2.0 - 2.0);

    tower - bore + cap_tether
}

fn prime_return_header() -> Part {
    let header = tube_run_x(
        "cell_seeding_prime_waste_return_header",
        -RETURN_MANIFOLD_X_LEN / 2.0,
        RETURN_MANIFOLD_X_LEN / 2.0,
        RETURN_MANIFOLD_Y,
        CHANNEL_Z + 7.0,
        CHANNEL_D * 1.4,
    );
    let waste_socket = centered_cylinder(
        "cell_seeding_prime_waste_bulkhead_socket",
        7.0 / 2.0,
        18.0,
        28,
    )
    .translate(WASTE_DIVERTER_X, RETURN_MANIFOLD_Y, CHANNEL_Z + 7.0);

    header + waste_socket
}

fn bubble_trap_degas_return() -> Part {
    let block = centered_cube(
        "cell_seeding_bubble_trap_degas_membrane_block",
        DEGASSER_X_LEN,
        DEGASSER_Y_LEN,
        DEGASSER_Z,
    )
    .translate(DEGASSER_X, DEGASSER_Y, DEGASSER_Z / 2.0);

    let inlet_chamber = centered_cylinder(
        "cell_seeding_bubble_trap_upstream_expansion_chamber",
        28.0 / 2.0,
        DEGASSER_Z + 2.0,
        40,
    )
    .translate(DEGASSER_X - 54.0, DEGASSER_Y, DEGASSER_Z / 2.0);
    let vent_plenum = centered_cube(
        "cell_seeding_bubble_trap_hydrophobic_vent_membrane_placeholder",
        DEGASSER_X_LEN - 52.0,
        22.0,
        4.0,
    )
    .translate(DEGASSER_X + 6.0, DEGASSER_Y + 12.0, DEGASSER_Z + 2.0);
    let vent_ports = degasser_vent_ports();
    let inlet = tube_run_x(
        "cell_seeding_degas_feed_from_reservoir",
        RESERVOIR_X + RESERVOIR_INTERFACE_D / 2.0,
        DEGASSER_X - DEGASSER_X_LEN / 2.0,
        DEGASSER_Y,
        CHANNEL_Z + 6.0,
        CHANNEL_D * 1.2,
    );
    let outlet = tube_run_x(
        "cell_seeding_degas_to_distribution_plenum",
        DEGASSER_X + DEGASSER_X_LEN / 2.0,
        0.0,
        0.0,
        CHANNEL_Z + 6.0,
        CHANNEL_D * 1.2,
    );

    let return_block = waste_recirculation_diverter();
    let return_to_reservoir = tube_run_x(
        "cell_seeding_recirculation_return_to_reservoir_interface",
        WASTE_DIVERTER_X,
        RESERVOIR_X,
        RETURN_MANIFOLD_Y,
        CHANNEL_Z + 11.0,
        CHANNEL_D * 1.25,
    );
    let waste_bulkhead = tube_run_y(
        "cell_seeding_waste_export_bulkhead_path",
        WASTE_DIVERTER_X,
        RETURN_MANIFOLD_Y,
        -FLUID_CASSETTE_Y / 2.0 + 12.0,
        CHANNEL_Z + 11.0,
        CHANNEL_D * 1.25,
    );

    (block - inlet_chamber)
        + vent_plenum
        + vent_ports
        + inlet
        + outlet
        + return_block
        + return_to_reservoir
        + waste_bulkhead
}

fn degasser_vent_ports() -> Part {
    let mut ports = Part::empty("cell_seeding_degas_vent_ports");
    for i in 0..4 {
        let x = DEGASSER_X - 54.0 + i as f64 * 36.0;
        let port = centered_cylinder(
            format!("cell_seeding_degas_hydrophobic_vent_port_{i}"),
            6.0 / 2.0,
            14.0,
            24,
        )
        .translate(x, DEGASSER_Y + 28.0, DEGASSER_Z + 7.0);
        ports = ports + port;
    }
    ports
}

fn waste_recirculation_diverter() -> Part {
    let body = centered_cube(
        "cell_seeding_waste_recirculation_return_diverter",
        76.0,
        54.0,
        WASTE_DIVERTER_Z,
    )
    .translate(WASTE_DIVERTER_X, WASTE_DIVERTER_Y, WASTE_DIVERTER_Z / 2.0);
    let selector_slot = centered_cube(
        "cell_seeding_waste_recirculation_selector_valve_slot",
        34.0,
        18.0,
        18.0,
    )
    .translate(WASTE_DIVERTER_X, WASTE_DIVERTER_Y, WASTE_DIVERTER_Z / 2.0);
    let waste_port = centered_cylinder(
        "cell_seeding_waste_recirculation_waste_port",
        7.0 / 2.0,
        62.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        WASTE_DIVERTER_X + 18.0,
        WASTE_DIVERTER_Y,
        WASTE_DIVERTER_Z / 2.0,
    );
    let recirc_port = centered_cylinder(
        "cell_seeding_waste_recirculation_recirc_port",
        7.0 / 2.0,
        62.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(
        WASTE_DIVERTER_X,
        WASTE_DIVERTER_Y + 14.0,
        WASTE_DIVERTER_Z / 2.0,
    );

    body - selector_slot - waste_port - recirc_port
}

fn pressure_shear_sensor_pockets() -> Part {
    let mut pockets = Part::empty("cell_seeding_pressure_shear_sensor_pockets");
    for (idx, x, y) in outlet_positions() {
        let sensor = sensor_pocket(idx).translate(x, y + 36.0, SENSOR_BLOCK_Z / 2.0);
        let shear_link = tube_run_y(
            &format!("cell_seeding_channel_{idx}_sensor_shear_takeoff"),
            x,
            y,
            y + 36.0,
            CHANNEL_Z + 10.0,
            CHANNEL_D * 0.7,
        );
        pockets = pockets + sensor + shear_link;
    }
    pockets
}

fn sensor_pocket(idx: usize) -> Part {
    let body = centered_cube(
        format!("cell_seeding_channel_{idx}_pressure_shear_sensor_pocket_body"),
        SENSOR_BLOCK_X,
        SENSOR_BLOCK_Y,
        SENSOR_BLOCK_Z,
    );
    let pressure_boss = centered_cylinder(
        format!("cell_seeding_channel_{idx}_pressure_sensor_diaphragm_boss"),
        8.5 / 2.0,
        SENSOR_BLOCK_Z + 2.0,
        26,
    );
    let shear_window = centered_cube(
        format!("cell_seeding_channel_{idx}_shear_witness_window"),
        22.0,
        4.0,
        6.0,
    )
    .translate(0.0, -7.0, 2.0);
    let wire_strain_relief = centered_cube(
        format!("cell_seeding_channel_{idx}_sensor_flex_tail_relief"),
        8.0,
        SENSOR_BLOCK_Y + 2.0,
        4.0,
    )
    .translate(10.0, 0.0, -4.0);

    body - pressure_boss - shear_window - wire_strain_relief
}

fn robotic_load_unload_keepout_frame() -> Part {
    let rear_rail = centered_cube(
        "cell_seeding_robot_keepout_rear_rail",
        ROBOT_KEEP_OUT_X,
        16.0,
        18.0,
    )
    .translate(0.0, ROBOT_KEEP_OUT_Y / 2.0, ROBOT_KEEPOUT_Z);
    let left_rail = centered_cube(
        "cell_seeding_robot_keepout_left_rail",
        16.0,
        ROBOT_KEEP_OUT_Y,
        18.0,
    )
    .translate(-ROBOT_KEEP_OUT_X / 2.0, 0.0, ROBOT_KEEPOUT_Z);
    let right_rail = centered_cube(
        "cell_seeding_robot_keepout_right_rail",
        16.0,
        ROBOT_KEEP_OUT_Y,
        18.0,
    )
    .translate(ROBOT_KEEP_OUT_X / 2.0, 0.0, ROBOT_KEEPOUT_Z);
    let front_approach = centered_cube(
        "cell_seeding_robot_front_approach_clearance_marker",
        ROBOT_KEEP_OUT_X,
        8.0,
        8.0,
    )
    .translate(0.0, -ROBOT_KEEP_OUT_Y / 2.0 - ROBOT_FRONT_APPROACH, 4.0);
    let vertical_lift_window = centered_cube(
        "cell_seeding_robot_vertical_cassette_lift_window",
        CASSETTE_X + 74.0,
        CASSETTE_Y + 64.0,
        4.0,
    )
    .translate(0.0, 0.0, ROBOT_KEEPOUT_Z + 24.0);

    rear_rail
        + left_rail
        + right_rail
        + front_approach
        + vertical_lift_window
        + robotic_keepout_posts()
}

fn robotic_keepout_posts() -> Part {
    let mut posts = Part::empty("cell_seeding_robot_keepout_posts");
    for (i, (x, y)) in [
        (-ROBOT_KEEP_OUT_X / 2.0, -ROBOT_KEEP_OUT_Y / 2.0),
        (ROBOT_KEEP_OUT_X / 2.0, -ROBOT_KEEP_OUT_Y / 2.0),
        (-ROBOT_KEEP_OUT_X / 2.0, ROBOT_KEEP_OUT_Y / 2.0),
        (ROBOT_KEEP_OUT_X / 2.0, ROBOT_KEEP_OUT_Y / 2.0),
    ]
    .iter()
    .enumerate()
    {
        let post = centered_cube(
            format!("cell_seeding_robot_keepout_post_{i}"),
            18.0,
            18.0,
            ROBOT_KEEPOUT_Z,
        )
        .translate(*x, *y, ROBOT_KEEPOUT_Z / 2.0);
        let foot = centered_cube(
            format!("cell_seeding_robot_keepout_foot_{i}"),
            42.0,
            42.0,
            8.0,
        )
        .translate(*x, *y, 4.0);
        let screw = centered_cylinder(
            format!("cell_seeding_robot_keepout_foot_screw_{i}"),
            5.3 / 2.0,
            10.0,
            24,
        )
        .translate(*x, *y, 4.0);
        posts = posts + post + (foot - screw);
    }
    posts
}

fn calibration_coupon_strip() -> Part {
    let strip = centered_cube(
        "cell_seeding_calibration_coupon_strip_carrier",
        CAL_STRIP_X,
        CAL_STRIP_Y,
        CAL_STRIP_Z,
    )
    .translate(CAL_STRIP_X_POS, CAL_STRIP_Y_POS, CAL_STRIP_Z / 2.0);

    let mut coupons = Part::empty("cell_seeding_calibration_coupons");
    for i in 0..CALIBRATION_COUPONS {
        let x = calibration_coupon_x(i);
        let body = centered_cube(
            format!("cell_seeding_calibration_coupon_{i}_body"),
            48.0,
            36.0,
            9.0,
        )
        .translate(x, CAL_STRIP_Y_POS, CAL_STRIP_Z + 4.5);
        let channel = centered_cylinder(
            format!("cell_seeding_calibration_coupon_{i}_reference_channel"),
            (1.2 + i as f64 * 0.18) / 2.0,
            52.0,
            18,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(x, CAL_STRIP_Y_POS, CAL_STRIP_Z + 4.5);
        let bubble_well = centered_cylinder(
            format!("cell_seeding_calibration_coupon_{i}_bubble_witness_well"),
            (6.0 + i as f64) / 2.0,
            4.0,
            24,
        )
        .translate(x + 13.0, CAL_STRIP_Y_POS + 8.0, CAL_STRIP_Z + 9.0);
        let fiducial = centered_cylinder(
            format!("cell_seeding_calibration_coupon_{i}_optical_fiducial"),
            3.0 / 2.0,
            4.0,
            18,
        )
        .translate(x - 16.0, CAL_STRIP_Y_POS - 9.0, CAL_STRIP_Z + 9.0);

        coupons = coupons + (body - channel - bubble_well - fiducial);
    }

    strip + coupons
}

fn tube_run_x(name: &str, x_a: f64, x_b: f64, y: f64, z: f64, diameter: f64) -> Part {
    let len = (x_b - x_a).abs().max(0.1);
    centered_cylinder(format!("{name}_tube_placeholder"), diameter / 2.0, len, 24)
        .rotate(0.0, 90.0, 0.0)
        .translate((x_a + x_b) / 2.0, y, z)
}

fn tube_run_y(name: &str, x: f64, y_a: f64, y_b: f64, z: f64, diameter: f64) -> Part {
    let len = (y_b - y_a).abs().max(0.1);
    centered_cylinder(format!("{name}_tube_placeholder"), diameter / 2.0, len, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(x, (y_a + y_b) / 2.0, z)
}

fn deck_mount_points() -> [(f64, f64); 8] {
    [
        (-(MODULE_X / 2.0 - 42.0), -(MODULE_Y / 2.0 - 42.0)),
        (MODULE_X / 2.0 - 42.0, -(MODULE_Y / 2.0 - 42.0)),
        (-(MODULE_X / 2.0 - 42.0), MODULE_Y / 2.0 - 42.0),
        (MODULE_X / 2.0 - 42.0, MODULE_Y / 2.0 - 42.0),
        (0.0, -(MODULE_Y / 2.0 - 42.0)),
        (0.0, MODULE_Y / 2.0 - 42.0),
        (-(MODULE_X / 2.0 - 42.0), 0.0),
        (MODULE_X / 2.0 - 42.0, 0.0),
    ]
}

fn datum_points() -> [(f64, f64); 3] {
    [
        (-CASSETTE_X / 2.0 + 22.0, -CASSETTE_Y / 2.0 + 22.0),
        (CASSETTE_X / 2.0 - 22.0, -CASSETTE_Y / 2.0 + 22.0),
        (-CASSETTE_X / 2.0 + 22.0, CASSETTE_Y / 2.0 - 22.0),
    ]
}

fn outlet_positions() -> Vec<(usize, f64, f64)> {
    let mut positions = Vec::with_capacity(CHANNELS);
    for row in 0..ROWS {
        for col in 0..COLS {
            positions.push((channel_index(row, col), chip_x(col), chip_y(row)));
        }
    }
    positions
}

fn channel_index(row: usize, col: usize) -> usize {
    row * COLS + col
}

fn chip_x(col: usize) -> f64 {
    -((COLS as f64 - 1.0) * PITCH_X) / 2.0 + col as f64 * PITCH_X
}

fn chip_y(row: usize) -> f64 {
    -((ROWS as f64 - 1.0) * PITCH_Y) / 2.0 + row as f64 * PITCH_Y
}

fn channel_tap_x(row: usize, col: usize) -> f64 {
    let lane = channel_index(row, col) as f64 - (CHANNELS as f64 - 1.0) / 2.0;
    lane * LANE_SPACING
}

fn raw_path_length(row: usize, col: usize) -> f64 {
    let tap = channel_tap_x(row, col);
    tap.abs() + chip_y(row).abs() + (chip_x(col) - tap).abs() + OUTLET_DROP_LENGTH
}

fn target_equalized_path_length() -> f64 {
    let mut max_len = 0.0;
    for row in 0..ROWS {
        for col in 0..COLS {
            let len = raw_path_length(row, col);
            if len > max_len {
                max_len = len;
            }
        }
    }
    max_len + EQUALIZATION_RESERVE_LENGTH
}

fn compensation_length(row: usize, col: usize) -> f64 {
    target_equalized_path_length() - raw_path_length(row, col)
}

fn equalized_path_length(row: usize, col: usize) -> f64 {
    raw_path_length(row, col) + compensation_length(row, col)
}

fn calibration_coupon_x(i: usize) -> f64 {
    let pitch = (CAL_STRIP_X - 74.0) / (CALIBRATION_COUPONS as f64 - 1.0);
    CAL_STRIP_X_POS - (CAL_STRIP_X - 74.0) / 2.0 + i as f64 * pitch
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cassette_layout_matches_twenty_chip_workflow() {
        assert_eq!(COLS * ROWS, 20);
        assert_eq!(CHANNELS, 20);
        assert_eq!(outlet_positions().len(), CHANNELS);
        assert_eq!(PITCH_X, REVC_CHIP_LENGTH + CHIP_GUTTER);
        assert_eq!(PITCH_Y, REVC_CHIP_WIDTH + CHIP_GUTTER);
        assert!(CASSETTE_X > ARRAY_X);
        assert!(CASSETTE_Y > ARRAY_Y);
    }

    #[test]
    fn every_channel_has_reproducibility_control_features() {
        assert_eq!(ISOLATION_VALVES, CHANNELS);
        assert_eq!(PRIME_PORTS, CHANNELS);
        assert_eq!(SENSOR_POCKETS, CHANNELS);
        assert_eq!(CALIBRATION_COUPONS, 5);
    }

    #[test]
    fn equalized_channel_lengths_are_sane_and_uniform() {
        let target = target_equalized_path_length();
        for row in 0..ROWS {
            for col in 0..COLS {
                let raw = raw_path_length(row, col);
                let compensated = equalized_path_length(row, col);
                assert!(raw > OUTLET_DROP_LENGTH);
                assert!(compensation_length(row, col) >= EQUALIZATION_RESERVE_LENGTH - 0.001);
                assert!((compensated - target).abs() < 0.001);
            }
        }
        assert!(target > 350.0);
        assert!(target < 700.0);
    }

    #[test]
    fn sterile_cassette_and_robot_keepouts_fit_on_module() {
        assert!(FLUID_CASSETTE_X < MODULE_X - 80.0);
        assert!(FLUID_CASSETTE_Y < MODULE_Y - 80.0);
        assert!(ROBOT_KEEP_OUT_X > CASSETTE_X + 120.0);
        assert!(ROBOT_KEEP_OUT_Y > CASSETTE_Y + 110.0);
        assert!(ROBOT_KEEPOUT_Z >= 150.0);
        assert!(ROBOT_FRONT_APPROACH >= 90.0);
    }

    #[test]
    fn datum_and_calibration_features_have_clear_geometry() {
        assert_eq!(datum_points().len(), 3);
        assert!(DATUM_SOCKET_D > DATUM_PIN_D);
        assert!(DATUM_SOCKET_DEPTH < FLUID_CASSETTE_Z);
        assert!(calibration_coupon_x(0) < calibration_coupon_x(CALIBRATION_COUPONS - 1));
        assert!(CAL_STRIP_X < MODULE_X / 2.0);
    }
}
