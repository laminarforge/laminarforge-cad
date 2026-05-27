use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed cleaning/sanitization validation cart for culture automation hardware.
//
// Intent:
// - Package cleaning fluid, rinse fluid, and closed waste return as explicit
//   segregated cart modules for a sealed culture workcell.
// - Provide drawers and lands for swab coupons, contact plates, barcode/lot
//   identifiers, cycle records, environmental sensors, and surface sensors.
// - Reserve VHP/H2O2 service clearances, tubing flush-port access, leak
//   containment, robot approach volume, and technician service keepouts.
//
// This is architecture/fit CAD only. It is not a sterilization cycle, a cleaning
// validation protocol, or a material compatibility certification.

const OUTPUTS: &[&str] = &[
    "output/closed_cleaning_sanitization_validation_cart_frame.stl",
    "output/closed_cleaning_sanitization_validation_cart_leak_tray.stl",
    "output/closed_cleaning_sanitization_validation_cart_clean_reservoirs.stl",
    "output/closed_cleaning_sanitization_validation_cart_rinse_reservoirs.stl",
    "output/closed_cleaning_sanitization_validation_cart_waste_return.stl",
    "output/closed_cleaning_sanitization_validation_cart_coupon_drawers.stl",
    "output/closed_cleaning_sanitization_validation_cart_vhp_clearance.stl",
    "output/closed_cleaning_sanitization_validation_cart_flush_ports.stl",
    "output/closed_cleaning_sanitization_validation_cart_segregation_bulkhead.stl",
    "output/closed_cleaning_sanitization_validation_cart_barcode_cycle_lands.stl",
    "output/closed_cleaning_sanitization_validation_cart_sensor_pockets.stl",
    "output/closed_cleaning_sanitization_validation_cart_robot_service_keepouts.stl",
    "output/closed_cleaning_sanitization_validation_cart_assembly.stl",
];

const CART_X: f64 = 1180.0;
const CART_Y: f64 = 620.0;
const CART_Z: f64 = 1220.0;
const FRAME_W: f64 = 34.0;
const BASE_TRAY_Z: f64 = 74.0;
const SHELF_Z: f64 = 18.0;

const CLEAN_SIDE_LIMIT_X: f64 = -34.0;
const DIRTY_SIDE_LIMIT_X: f64 = 34.0;
const BULKHEAD_X: f64 = 34.0;
const BULKHEAD_Y: f64 = CART_Y - 92.0;
const BULKHEAD_Z: f64 = 790.0;

const CLEAN_RESERVOIR_COUNT: usize = 3;
const CLEAN_RESERVOIR_D: f64 = 116.0;
const CLEAN_RESERVOIR_Z: f64 = 326.0;
const CLEAN_RESERVOIR_PITCH_Y: f64 = 150.0;
const CLEAN_RESERVOIR_CENTER_X: f64 = -370.0;
const CLEAN_RESERVOIR_BASE_Z: f64 = 236.0;

const RINSE_RESERVOIR_COUNT: usize = 2;
const RINSE_RESERVOIR_D: f64 = 104.0;
const RINSE_RESERVOIR_Z: f64 = 250.0;
const RINSE_RESERVOIR_PITCH_Y: f64 = 138.0;
const RINSE_RESERVOIR_CENTER_X: f64 = -160.0;
const RINSE_RESERVOIR_BASE_Z: f64 = 690.0;

const WASTE_RETURN_COUNT: usize = 2;
const WASTE_RETURN_D: f64 = 132.0;
const WASTE_RETURN_Z: f64 = 342.0;
const WASTE_RETURN_PITCH_Y: f64 = 164.0;
const WASTE_RETURN_CENTER_X: f64 = 332.0;
const WASTE_RETURN_BASE_Z: f64 = 242.0;

const COUPON_DRAWER_COUNT: usize = 3;
const COUPON_DRAWER_X: f64 = 420.0;
const COUPON_DRAWER_Y: f64 = 218.0;
const COUPON_DRAWER_Z: f64 = 54.0;
const COUPON_DRAWER_PITCH_Z: f64 = 78.0;
const COUPON_DRAWER_CENTER_X: f64 = 300.0;
const COUPON_DRAWER_CENTER_Y: f64 = -238.0;
const COUPON_DRAWER_BASE_Z: f64 = 612.0;
const CONTACT_PLATE_RECESSES: usize = 6;
const SWAB_COUPON_SLOTS: usize = 12;

const FRONT_SERVICE_CLEARANCE: f64 = 520.0;
const REAR_VHP_CLEARANCE: f64 = 260.0;
const SIDE_H2O2_CLEARANCE: f64 = 180.0;
const ROBOT_KEEP_OUT_Z: f64 = 440.0;

const FLUSH_PORT_COUNT: usize = 8;
const RETURN_PORT_COUNT: usize = 6;
const SURFACE_SENSOR_COUNT: usize = 6;
const ENV_SENSOR_COUNT: usize = 5;
const BARCODE_LAND_COUNT: usize = 12;

fn main() {
    fs::create_dir_all("output").unwrap();

    let frame = cart_frame();
    export(&frame, OUTPUTS[0]);

    let leak_tray = leak_tray();
    export(&leak_tray, OUTPUTS[1]);

    let clean = clean_reservoir_module();
    export(&clean, OUTPUTS[2]);

    let rinse = rinse_reservoir_module();
    export(&rinse, OUTPUTS[3]);

    let waste = waste_return_module();
    export(&waste, OUTPUTS[4]);

    let drawers = coupon_drawer_bank();
    export(&drawers, OUTPUTS[5]);

    let vhp_clearance = vhp_h2o2_service_clearances();
    export(&vhp_clearance, OUTPUTS[6]);

    let flush_ports = tubing_flush_port_manifold();
    export(&flush_ports, OUTPUTS[7]);

    let bulkhead = clean_dirty_segregation_bulkhead();
    export(&bulkhead, OUTPUTS[8]);

    let barcode_lands = barcode_lot_cycle_lands();
    export(&barcode_lands, OUTPUTS[9]);

    let sensors = environmental_surface_sensor_pockets();
    export(&sensors, OUTPUTS[10]);

    let keepouts = robot_service_keepouts();
    export(&keepouts, OUTPUTS[11]);

    let assembly = frame
        + leak_tray
        + clean
        + rinse
        + waste
        + drawers
        + vhp_clearance
        + flush_ports
        + bulkhead
        + barcode_lands
        + sensors
        + keepouts;
    export(&assembly, OUTPUTS[12]);

    println!(
        "Closed cleaning/sanitization validation cart: {:.0}mm W x {:.0}mm D x {:.0}mm H with {} cleaning reservoirs, {} rinse reservoirs, {} closed waste returns, {} flush ports, {} return ports, and {} validation drawers.",
        CART_X,
        CART_Y,
        CART_Z,
        CLEAN_RESERVOIR_COUNT,
        RINSE_RESERVOIR_COUNT,
        WASTE_RETURN_COUNT,
        FLUSH_PORT_COUNT,
        RETURN_PORT_COUNT,
        COUPON_DRAWER_COUNT
    );
    println!(
        "Modeled clearances: {:.0}mm front pull/service, {:.0}mm rear VHP/H2O2 hose access, {:.0}mm side oxidizer bottle swing, and {:.0}mm robot approach keepout height.",
        FRONT_SERVICE_CLEARANCE, REAR_VHP_CLEARANCE, SIDE_H2O2_CLEARANCE, ROBOT_KEEP_OUT_Z
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn cart_frame() -> Part {
    let lower_shelf = centered_cube(
        "cleaning_validation_cart_lower_shelf",
        CART_X - 118.0,
        CART_Y - 112.0,
        SHELF_Z,
    )
    .translate(0.0, 0.0, 164.0);
    let mid_shelf = centered_cube(
        "cleaning_validation_cart_validation_shelf",
        CART_X - 160.0,
        CART_Y - 128.0,
        SHELF_Z,
    )
    .translate(0.0, 0.0, 556.0);
    let top_rail_deck = centered_cube(
        "cleaning_validation_cart_top_service_deck",
        CART_X - 220.0,
        CART_Y - 156.0,
        SHELF_Z,
    )
    .translate(0.0, 0.0, 908.0);

    let mut posts = Part::empty("cleaning_validation_cart_frame_posts");
    for (i, (x, y)) in frame_post_points().iter().enumerate() {
        posts = posts
            + centered_cube(
                format!("cleaning_validation_cart_post_{i}"),
                FRAME_W,
                FRAME_W,
                CART_Z,
            )
            .translate(*x, *y, CART_Z / 2.0);
    }

    let mut rails = Part::empty("cleaning_validation_cart_perimeter_rails");
    for (i, z) in [BASE_TRAY_Z + 62.0, 380.0, 696.0, 986.0, CART_Z - 26.0]
        .iter()
        .enumerate()
    {
        rails = rails
            + centered_cube(
                format!("cleaning_validation_cart_front_rail_{i}"),
                CART_X,
                FRAME_W,
                FRAME_W,
            )
            .translate(0.0, -(CART_Y / 2.0 - FRAME_W / 2.0), *z)
            + centered_cube(
                format!("cleaning_validation_cart_rear_rail_{i}"),
                CART_X,
                FRAME_W,
                FRAME_W,
            )
            .translate(0.0, CART_Y / 2.0 - FRAME_W / 2.0, *z)
            + centered_cube(
                format!("cleaning_validation_cart_left_rail_{i}"),
                FRAME_W,
                CART_Y,
                FRAME_W,
            )
            .translate(-(CART_X / 2.0 - FRAME_W / 2.0), 0.0, *z)
            + centered_cube(
                format!("cleaning_validation_cart_right_rail_{i}"),
                FRAME_W,
                CART_Y,
                FRAME_W,
            )
            .translate(CART_X / 2.0 - FRAME_W / 2.0, 0.0, *z);
    }

    lower_shelf + mid_shelf + top_rail_deck + posts + rails + caster_plates() + dock_datum_bars()
}

fn leak_tray() -> Part {
    let pan = centered_cube(
        "cleaning_validation_cart_secondary_leak_pan",
        CART_X,
        CART_Y,
        BASE_TRAY_Z,
    )
    .translate(0.0, 0.0, BASE_TRAY_Z / 2.0);
    let cavity = centered_cube(
        "cleaning_validation_cart_secondary_leak_pan_cavity",
        CART_X - 94.0,
        CART_Y - 90.0,
        BASE_TRAY_Z - 16.0,
    )
    .translate(0.0, 0.0, BASE_TRAY_Z / 2.0 + 18.0);
    let dirty_sump = centered_cube(
        "cleaning_validation_cart_dirty_return_sump",
        420.0,
        58.0,
        18.0,
    )
    .translate(284.0, CART_Y / 2.0 - 84.0, BASE_TRAY_Z - 12.0);
    let clean_sump = centered_cube(
        "cleaning_validation_cart_clean_side_leak_witness_channel",
        390.0,
        30.0,
        14.0,
    )
    .translate(-296.0, -(CART_Y / 2.0 - 72.0), BASE_TRAY_Z - 10.0);
    let drain = centered_cylinder(
        "cleaning_validation_cart_secondary_containment_drain",
        12.0 / 2.0,
        44.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(CART_X / 2.0 - 92.0, CART_Y / 2.0 + 2.0, 24.0);
    let leak_sensor_groove = centered_cube(
        "cleaning_validation_cart_leak_sensor_trace_groove",
        CART_X - 168.0,
        12.0,
        18.0,
    )
    .translate(0.0, -(CART_Y / 2.0 - 48.0), BASE_TRAY_Z - 6.0);

    pan - cavity - dirty_sump - clean_sump - drain - leak_sensor_groove
        + clean_dirty_floor_lip()
        + tray_corner_witness_pads()
}

fn clean_reservoir_module() -> Part {
    let cradle = centered_cube(
        "cleaning_validation_clean_reservoir_cradle",
        360.0,
        510.0,
        28.0,
    )
    .translate(CLEAN_RESERVOIR_CENTER_X, 0.0, CLEAN_RESERVOIR_BASE_Z - 26.0);
    let rear_retainer = centered_cube(
        "cleaning_validation_clean_reservoir_rear_retainer",
        342.0,
        18.0,
        92.0,
    )
    .translate(
        CLEAN_RESERVOIR_CENTER_X,
        CART_Y / 2.0 - 96.0,
        CLEAN_RESERVOIR_BASE_Z + 38.0,
    );

    let mut reservoirs = Part::empty("cleaning_validation_clean_reservoirs");
    for i in 0..CLEAN_RESERVOIR_COUNT {
        let y = reservoir_y(i, CLEAN_RESERVOIR_COUNT, CLEAN_RESERVOIR_PITCH_Y);
        reservoirs = reservoirs
            + reservoir_shell(
                &format!("cleaning_validation_clean_reservoir_{i}"),
                CLEAN_RESERVOIR_D,
                CLEAN_RESERVOIR_Z,
            )
            .translate(
                CLEAN_RESERVOIR_CENTER_X,
                y,
                CLEAN_RESERVOIR_BASE_Z + CLEAN_RESERVOIR_Z / 2.0,
            )
            + reservoir_cap_and_tube(
                &format!("cleaning_validation_clean_reservoir_{i}"),
                CLEAN_RESERVOIR_D,
                CLEAN_RESERVOIR_Z,
            )
            .translate(
                CLEAN_RESERVOIR_CENTER_X,
                y,
                CLEAN_RESERVOIR_BASE_Z + CLEAN_RESERVOIR_Z / 2.0,
            )
            + centered_cube(
                format!("cleaning_validation_clean_reservoir_level_land_{i}"),
                72.0,
                9.0,
                38.0,
            )
            .translate(
                CLEAN_RESERVOIR_CENTER_X + CLEAN_RESERVOIR_D / 2.0 + 9.0,
                y,
                CLEAN_RESERVOIR_BASE_Z + 188.0,
            );
    }

    cradle + rear_retainer + reservoirs + clean_reservoir_label_strip()
}

fn rinse_reservoir_module() -> Part {
    let shelf = centered_cube(
        "cleaning_validation_rinse_reservoir_shelf",
        390.0,
        392.0,
        26.0,
    )
    .translate(RINSE_RESERVOIR_CENTER_X, 0.0, RINSE_RESERVOIR_BASE_Z - 24.0);
    let guard = centered_cube(
        "cleaning_validation_rinse_reservoir_guard",
        370.0,
        18.0,
        88.0,
    )
    .translate(
        RINSE_RESERVOIR_CENTER_X,
        CART_Y / 2.0 - 118.0,
        RINSE_RESERVOIR_BASE_Z + 26.0,
    );

    let mut reservoirs = Part::empty("cleaning_validation_rinse_reservoirs");
    for i in 0..RINSE_RESERVOIR_COUNT {
        let y = reservoir_y(i, RINSE_RESERVOIR_COUNT, RINSE_RESERVOIR_PITCH_Y);
        reservoirs = reservoirs
            + reservoir_shell(
                &format!("cleaning_validation_rinse_reservoir_{i}"),
                RINSE_RESERVOIR_D,
                RINSE_RESERVOIR_Z,
            )
            .translate(
                RINSE_RESERVOIR_CENTER_X,
                y,
                RINSE_RESERVOIR_BASE_Z + RINSE_RESERVOIR_Z / 2.0,
            )
            + reservoir_cap_and_tube(
                &format!("cleaning_validation_rinse_reservoir_{i}"),
                RINSE_RESERVOIR_D,
                RINSE_RESERVOIR_Z,
            )
            .translate(
                RINSE_RESERVOIR_CENTER_X,
                y,
                RINSE_RESERVOIR_BASE_Z + RINSE_RESERVOIR_Z / 2.0,
            );
    }

    shelf + guard + reservoirs + rinse_reservoir_label_strip()
}

fn waste_return_module() -> Part {
    let basin = centered_cube(
        "cleaning_validation_closed_waste_return_basin",
        430.0,
        430.0,
        30.0,
    )
    .translate(WASTE_RETURN_CENTER_X, 0.0, WASTE_RETURN_BASE_Z - 28.0);
    let rear_filter_panel = centered_cube(
        "cleaning_validation_waste_return_filter_panel",
        330.0,
        30.0,
        210.0,
    )
    .translate(
        WASTE_RETURN_CENTER_X + 18.0,
        CART_Y / 2.0 - 76.0,
        WASTE_RETURN_BASE_Z + 240.0,
    );

    let mut returns = Part::empty("cleaning_validation_closed_waste_returns");
    for i in 0..WASTE_RETURN_COUNT {
        let y = reservoir_y(i, WASTE_RETURN_COUNT, WASTE_RETURN_PITCH_Y);
        returns = returns
            + reservoir_shell(
                &format!("cleaning_validation_waste_return_carboy_{i}"),
                WASTE_RETURN_D,
                WASTE_RETURN_Z,
            )
            .translate(
                WASTE_RETURN_CENTER_X,
                y,
                WASTE_RETURN_BASE_Z + WASTE_RETURN_Z / 2.0,
            )
            + waste_return_cap_pack(i).translate(
                WASTE_RETURN_CENTER_X,
                y,
                WASTE_RETURN_BASE_Z + WASTE_RETURN_Z,
            );
    }

    basin + rear_filter_panel + returns + waste_vent_filter_bank() + waste_sample_ports()
}

fn coupon_drawer_bank() -> Part {
    let tower = centered_cube(
        "cleaning_validation_coupon_drawer_side_tower",
        COUPON_DRAWER_X + 66.0,
        28.0,
        COUPON_DRAWER_PITCH_Z * COUPON_DRAWER_COUNT as f64 + 46.0,
    )
    .translate(
        COUPON_DRAWER_CENTER_X,
        COUPON_DRAWER_CENTER_Y + COUPON_DRAWER_Y / 2.0 + 20.0,
        COUPON_DRAWER_BASE_Z + COUPON_DRAWER_PITCH_Z,
    );

    let mut drawers = Part::empty("cleaning_validation_coupon_drawer_bank");
    for i in 0..COUPON_DRAWER_COUNT {
        let z = coupon_drawer_z(i);
        drawers = drawers
            + coupon_drawer_body(i).translate(COUPON_DRAWER_CENTER_X, COUPON_DRAWER_CENTER_Y, z);
    }

    tower + drawers + contact_plate_recess_array() + swab_coupon_slot_array()
}

fn vhp_h2o2_service_clearances() -> Part {
    let rear = clearance_frame(
        "cleaning_validation_rear_vhp_h2o2_service_clearance",
        CART_X - 180.0,
        REAR_VHP_CLEARANCE,
        660.0,
    )
    .translate(0.0, CART_Y / 2.0 + REAR_VHP_CLEARANCE / 2.0, 650.0);
    let clean_side = clearance_frame(
        "cleaning_validation_clean_side_h2o2_bottle_swing_clearance",
        SIDE_H2O2_CLEARANCE,
        CART_Y - 160.0,
        520.0,
    )
    .translate(-(CART_X / 2.0 + SIDE_H2O2_CLEARANCE / 2.0), -8.0, 480.0);
    let front_pull = clearance_frame(
        "cleaning_validation_front_drawer_pull_and_flush_service_clearance",
        CART_X - 220.0,
        FRONT_SERVICE_CLEARANCE,
        330.0,
    )
    .translate(0.0, -(CART_Y / 2.0 + FRONT_SERVICE_CLEARANCE / 2.0), 630.0);

    rear + clean_side + front_pull + oxidizer_material_notice_lands()
}

fn tubing_flush_port_manifold() -> Part {
    let panel = centered_cube(
        "cleaning_validation_tubing_flush_bulkhead_panel",
        930.0,
        32.0,
        252.0,
    )
    .translate(0.0, -(CART_Y / 2.0 + 28.0), 674.0);

    let mut holes = Part::empty("cleaning_validation_tubing_flush_bulkhead_holes");
    let mut collars = Part::empty("cleaning_validation_tubing_flush_bulkhead_collars");
    for i in 0..FLUSH_PORT_COUNT {
        let x = port_x(i, FLUSH_PORT_COUNT, 58.0) - 228.0;
        holes = holes
            + centered_cylinder(
                format!("cleaning_validation_clean_flush_port_clearance_{i}"),
                10.0 / 2.0,
                40.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -(CART_Y / 2.0 + 28.0), 708.0);
        collars = collars
            + port_collar(
                &format!("cleaning_validation_clean_flush_port_collar_{i}"),
                26.0,
                10.0,
            )
            .translate(x, -(CART_Y / 2.0 + 48.0), 708.0);
    }
    for i in 0..RETURN_PORT_COUNT {
        let x = port_x(i, RETURN_PORT_COUNT, 62.0) + 250.0;
        holes = holes
            + centered_cylinder(
                format!("cleaning_validation_dirty_return_port_clearance_{i}"),
                12.0 / 2.0,
                40.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -(CART_Y / 2.0 + 28.0), 632.0);
        collars = collars
            + port_collar(
                &format!("cleaning_validation_dirty_return_port_collar_{i}"),
                30.0,
                12.0,
            )
            .translate(x, -(CART_Y / 2.0 + 48.0), 632.0);
    }

    let tubing_comb = tubing_guide_comb().translate(0.0, -(CART_Y / 2.0 + 70.0), 552.0);
    panel - holes + collars + tubing_comb + flush_direction_datum_arrows()
}

fn clean_dirty_segregation_bulkhead() -> Part {
    let panel = centered_cube(
        "cleaning_validation_clean_dirty_segregation_bulkhead",
        BULKHEAD_X,
        BULKHEAD_Y,
        BULKHEAD_Z,
    )
    .translate(0.0, 0.0, 520.0);
    let pass_through_cut = centered_cube(
        "cleaning_validation_gasketed_tubing_pass_through_cut",
        BULKHEAD_X + 6.0,
        132.0,
        84.0,
    )
    .translate(0.0, CART_Y / 2.0 - 166.0, 714.0);
    let drawer_transfer_cut = centered_cube(
        "cleaning_validation_coupon_transfer_slot_cut",
        BULKHEAD_X + 6.0,
        182.0,
        66.0,
    )
    .translate(0.0, -(CART_Y / 2.0 - 176.0), 610.0);

    panel - pass_through_cut - drawer_transfer_cut
        + gasket_lip("flush_pass", CART_Y / 2.0 - 166.0, 714.0, 168.0, 112.0)
        + gasket_lip(
            "coupon_transfer",
            -(CART_Y / 2.0 - 176.0),
            610.0,
            218.0,
            92.0,
        )
        + drip_gutter()
}

fn barcode_lot_cycle_lands() -> Part {
    let header = centered_cube(
        "cleaning_validation_barcode_lot_cycle_header_panel",
        CART_X - 170.0,
        16.0,
        118.0,
    )
    .translate(0.0, -(CART_Y / 2.0 + 54.0), 1010.0);

    let mut lands = Part::empty("cleaning_validation_barcode_lot_cycle_lands");
    for i in 0..BARCODE_LAND_COUNT {
        let x = port_x(i, BARCODE_LAND_COUNT, 78.0);
        lands = lands
            + centered_cube(
                format!("cleaning_validation_barcode_lot_cycle_land_{i}"),
                62.0,
                8.0,
                24.0,
            )
            .translate(x, -(CART_Y / 2.0 + 66.0), 1026.0)
            + centered_cube(
                format!("cleaning_validation_cycle_status_flag_slot_{i}"),
                42.0,
                8.0,
                12.0,
            )
            .translate(x, -(CART_Y / 2.0 + 66.0), 982.0);
    }

    header + lands + lot_chain_of_custody_rail()
}

fn environmental_surface_sensor_pockets() -> Part {
    let panel = centered_cube(
        "cleaning_validation_environmental_sensor_panel",
        520.0,
        28.0,
        276.0,
    )
    .translate(-215.0, CART_Y / 2.0 + 20.0, 760.0);
    let mut sensor_cuts = Part::empty("cleaning_validation_environmental_sensor_pocket_cuts");
    let mut sensor_bosses = Part::empty("cleaning_validation_environmental_sensor_bosses");
    for i in 0..ENV_SENSOR_COUNT {
        let x = -215.0 + port_x(i, ENV_SENSOR_COUNT, 82.0);
        sensor_cuts = sensor_cuts
            + centered_cylinder(
                format!("cleaning_validation_env_sensor_pocket_cut_{i}"),
                19.0 / 2.0,
                38.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, CART_Y / 2.0 + 20.0, 808.0);
        sensor_bosses = sensor_bosses
            + port_collar(
                &format!("cleaning_validation_env_sensor_boss_{i}"),
                36.0,
                19.0,
            )
            .translate(x, CART_Y / 2.0 + 42.0, 808.0);
    }

    let mut surface_pockets = Part::empty("cleaning_validation_surface_sensor_pockets");
    for i in 0..SURFACE_SENSOR_COUNT {
        let x = port_x(i, SURFACE_SENSOR_COUNT, 116.0) + 120.0;
        surface_pockets =
            surface_pockets + surface_sensor_pocket(i).translate(x, CART_Y / 2.0 - 86.0, 930.0);
    }

    panel - sensor_cuts + sensor_bosses + surface_pockets + sensor_lot_label_strip()
}

fn robot_service_keepouts() -> Part {
    let robot_approach = clearance_frame(
        "cleaning_validation_robot_arm_approach_keepout",
        860.0,
        300.0,
        ROBOT_KEEP_OUT_Z,
    )
    .translate(-60.0, -(CART_Y / 2.0 + 150.0), 780.0);
    let gripper_sweep = clearance_frame(
        "cleaning_validation_gripper_sweep_keepout_over_flush_bulkhead",
        680.0,
        180.0,
        250.0,
    )
    .translate(-80.0, -(CART_Y / 2.0 + 42.0), 848.0);
    let technician_access = clearance_frame(
        "cleaning_validation_technician_rear_service_keepout",
        960.0,
        220.0,
        520.0,
    )
    .translate(0.0, CART_Y / 2.0 + 120.0, 600.0);

    robot_approach + gripper_sweep + technician_access + robot_datum_targets()
}

fn reservoir_shell(name: &str, diameter: f64, height: f64) -> Part {
    let wall = 5.0;
    let shell = centered_cylinder(format!("{name}_outer_shell"), diameter / 2.0, height, 64);
    let cavity = centered_cylinder(
        format!("{name}_open_cavity"),
        diameter / 2.0 - wall,
        height + 4.0,
        64,
    )
    .translate(0.0, 0.0, 8.0);
    let sight_slot = centered_cube(
        format!("{name}_vertical_sight_window_proxy"),
        10.0,
        diameter + 4.0,
        height - 88.0,
    )
    .translate(diameter / 2.0 - 8.0, 0.0, 10.0);
    shell - cavity - sight_slot
}

fn reservoir_cap_and_tube(name: &str, diameter: f64, height: f64) -> Part {
    let cap = centered_cylinder(
        format!("{name}_sealed_cap_land"),
        diameter / 2.0 + 7.0,
        18.0,
        64,
    )
    .translate(0.0, 0.0, height / 2.0 + 9.0);
    let pickup = centered_cylinder(
        format!("{name}_ptfe_fep_pickup_tube_proxy"),
        4.0,
        height,
        20,
    )
    .translate(-diameter * 0.18, 0.0, 0.0);
    let return_stub = centered_cylinder(format!("{name}_return_stub_proxy"), 5.0, 54.0, 20)
        .rotate(90.0, 0.0, 0.0)
        .translate(
            diameter * 0.18,
            -(diameter / 2.0 + 17.0),
            height / 2.0 - 34.0,
        );
    let vent_stub = centered_cylinder(format!("{name}_filtered_vent_stub"), 4.5, 48.0, 20)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, diameter / 2.0 + 16.0, height / 2.0 - 34.0);
    cap + pickup + return_stub + vent_stub
}

fn waste_return_cap_pack(index: usize) -> Part {
    let cap = centered_cylinder(
        format!("cleaning_validation_waste_return_cap_{index}"),
        WASTE_RETURN_D / 2.0 + 8.0,
        20.0,
        64,
    )
    .translate(0.0, 0.0, 10.0);
    let check_valve_block = centered_cube(
        format!("cleaning_validation_waste_return_check_valve_block_{index}"),
        74.0,
        34.0,
        42.0,
    )
    .translate(0.0, -WASTE_RETURN_D / 2.0 - 24.0, 38.0);
    let filter = centered_cylinder(
        format!("cleaning_validation_waste_return_hydrophobic_vent_filter_{index}"),
        24.0,
        42.0,
        36,
    )
    .translate(0.0, WASTE_RETURN_D / 2.0 + 24.0, 38.0);
    cap + check_valve_block + filter
}

fn coupon_drawer_body(index: usize) -> Part {
    let body = centered_cube(
        format!("cleaning_validation_coupon_drawer_body_{index}"),
        COUPON_DRAWER_X,
        COUPON_DRAWER_Y,
        COUPON_DRAWER_Z,
    );
    let tray_cavity = centered_cube(
        format!("cleaning_validation_coupon_drawer_open_tray_{index}"),
        COUPON_DRAWER_X - 42.0,
        COUPON_DRAWER_Y - 44.0,
        COUPON_DRAWER_Z - 16.0,
    )
    .translate(0.0, 0.0, 9.0);
    let pull = centered_cube(
        format!("cleaning_validation_coupon_drawer_pull_handle_{index}"),
        COUPON_DRAWER_X - 90.0,
        18.0,
        22.0,
    )
    .translate(0.0, -(COUPON_DRAWER_Y / 2.0 + 12.0), 0.0);
    let dirty_flag_land = centered_cube(
        format!("cleaning_validation_coupon_drawer_dirty_status_land_{index}"),
        58.0,
        10.0,
        18.0,
    )
    .translate(
        COUPON_DRAWER_X / 2.0 - 50.0,
        -(COUPON_DRAWER_Y / 2.0 + 14.0),
        0.0,
    );
    body - tray_cavity + pull + dirty_flag_land + drawer_slide_keys(index)
}

fn contact_plate_recess_array() -> Part {
    let mut pockets = Part::empty("cleaning_validation_contact_plate_recess_array");
    for i in 0..CONTACT_PLATE_RECESSES {
        let row = i / 3;
        let col = i % 3;
        let x = COUPON_DRAWER_CENTER_X - 118.0 + col as f64 * 118.0;
        let y = COUPON_DRAWER_CENTER_Y - 18.0 + row as f64 * 72.0;
        pockets = pockets
            + centered_cylinder(
                format!("cleaning_validation_contact_plate_recess_{i}"),
                46.0,
                12.0,
                56,
            )
            .translate(x, y, coupon_drawer_z(1) + COUPON_DRAWER_Z / 2.0 - 2.0)
            - centered_cylinder(
                format!("cleaning_validation_contact_plate_inner_well_{i}"),
                40.0,
                14.0,
                56,
            )
            .translate(x, y, coupon_drawer_z(1) + COUPON_DRAWER_Z / 2.0 - 2.0);
    }
    pockets
}

fn swab_coupon_slot_array() -> Part {
    let mut slots = Part::empty("cleaning_validation_swab_coupon_slot_array");
    for i in 0..SWAB_COUPON_SLOTS {
        let row = i / 4;
        let col = i % 4;
        let x = COUPON_DRAWER_CENTER_X - 145.0 + col as f64 * 96.0;
        let y = COUPON_DRAWER_CENTER_Y - 62.0 + row as f64 * 58.0;
        slots = slots
            + centered_cube(
                format!("cleaning_validation_swab_coupon_slot_{i}"),
                62.0,
                15.0,
                10.0,
            )
            .translate(x, y, coupon_drawer_z(0) + COUPON_DRAWER_Z / 2.0 + 1.0);
    }
    slots
}

fn waste_vent_filter_bank() -> Part {
    let mut bank = Part::empty("cleaning_validation_waste_vent_filter_bank");
    for i in 0..3 {
        bank = bank
            + centered_cylinder(
                format!("cleaning_validation_waste_vent_filter_{i}"),
                28.0,
                52.0,
                36,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                WASTE_RETURN_CENTER_X - 112.0 + i as f64 * 112.0,
                CART_Y / 2.0 - 54.0,
                WASTE_RETURN_BASE_Z + 320.0,
            );
    }
    bank
}

fn waste_sample_ports() -> Part {
    let panel = centered_cube(
        "cleaning_validation_waste_sample_neutralization_panel",
        236.0,
        24.0,
        98.0,
    )
    .translate(
        WASTE_RETURN_CENTER_X,
        -(CART_Y / 2.0 - 92.0),
        WASTE_RETURN_BASE_Z + 228.0,
    );
    let mut ports = Part::empty("cleaning_validation_waste_sample_neutralization_ports");
    for i in 0..4 {
        ports = ports
            + port_collar(
                &format!("cleaning_validation_waste_sample_port_{i}"),
                24.0,
                9.0,
            )
            .translate(
                WASTE_RETURN_CENTER_X - 78.0 + i as f64 * 52.0,
                -(CART_Y / 2.0 - 108.0),
                WASTE_RETURN_BASE_Z + 228.0,
            );
    }
    panel + ports
}

fn tubing_guide_comb() -> Part {
    let rail = centered_cube(
        "cleaning_validation_flush_tubing_guide_comb_rail",
        760.0,
        16.0,
        16.0,
    );
    let mut fingers = Part::empty("cleaning_validation_flush_tubing_guide_comb_fingers");
    for i in 0..(FLUSH_PORT_COUNT + RETURN_PORT_COUNT) {
        fingers = fingers
            + centered_cube(
                format!("cleaning_validation_flush_tubing_comb_finger_{i}"),
                8.0,
                44.0,
                50.0,
            )
            .translate(
                port_x(i, FLUSH_PORT_COUNT + RETURN_PORT_COUNT, 54.0),
                0.0,
                20.0,
            );
    }
    rail + fingers
}

fn flush_direction_datum_arrows() -> Part {
    let clean_arrow = centered_cube(
        "cleaning_validation_clean_flush_direction_datum",
        220.0,
        8.0,
        10.0,
    )
    .translate(-248.0, -(CART_Y / 2.0 + 72.0), 772.0);
    let dirty_arrow = centered_cube(
        "cleaning_validation_dirty_return_direction_datum",
        200.0,
        8.0,
        10.0,
    )
    .translate(260.0, -(CART_Y / 2.0 + 72.0), 572.0);
    clean_arrow + dirty_arrow
}

fn clean_dirty_floor_lip() -> Part {
    centered_cube(
        "cleaning_validation_clean_dirty_floor_lip",
        18.0,
        CART_Y - 112.0,
        42.0,
    )
    .translate(0.0, 0.0, BASE_TRAY_Z + 21.0)
}

fn tray_corner_witness_pads() -> Part {
    let mut pads = Part::empty("cleaning_validation_tray_corner_witness_pads");
    for (i, (x, y)) in [
        (-(CART_X / 2.0 - 70.0), -(CART_Y / 2.0 - 70.0)),
        (CART_X / 2.0 - 70.0, -(CART_Y / 2.0 - 70.0)),
        (-(CART_X / 2.0 - 70.0), CART_Y / 2.0 - 70.0),
        (CART_X / 2.0 - 70.0, CART_Y / 2.0 - 70.0),
    ]
    .iter()
    .enumerate()
    {
        pads = pads
            + centered_cube(
                format!("cleaning_validation_leak_witness_pad_{i}"),
                58.0,
                42.0,
                6.0,
            )
            .translate(*x, *y, BASE_TRAY_Z + 3.0);
    }
    pads
}

fn clean_reservoir_label_strip() -> Part {
    let mut strip = Part::empty("cleaning_validation_clean_reservoir_label_strip");
    for i in 0..CLEAN_RESERVOIR_COUNT {
        strip = strip
            + centered_cube(
                format!("cleaning_validation_clean_reservoir_lot_land_{i}"),
                86.0,
                8.0,
                24.0,
            )
            .translate(
                CLEAN_RESERVOIR_CENTER_X,
                reservoir_y(i, CLEAN_RESERVOIR_COUNT, CLEAN_RESERVOIR_PITCH_Y),
                CLEAN_RESERVOIR_BASE_Z + CLEAN_RESERVOIR_Z + 42.0,
            );
    }
    strip
}

fn rinse_reservoir_label_strip() -> Part {
    let mut strip = Part::empty("cleaning_validation_rinse_reservoir_label_strip");
    for i in 0..RINSE_RESERVOIR_COUNT {
        strip = strip
            + centered_cube(
                format!("cleaning_validation_rinse_reservoir_lot_land_{i}"),
                80.0,
                8.0,
                22.0,
            )
            .translate(
                RINSE_RESERVOIR_CENTER_X,
                reservoir_y(i, RINSE_RESERVOIR_COUNT, RINSE_RESERVOIR_PITCH_Y),
                RINSE_RESERVOIR_BASE_Z + RINSE_RESERVOIR_Z + 38.0,
            );
    }
    strip
}

fn drawer_slide_keys(index: usize) -> Part {
    let z = -(COUPON_DRAWER_Z / 2.0 - 8.0);
    centered_cube(
        format!("cleaning_validation_coupon_drawer_left_slide_key_{index}"),
        COUPON_DRAWER_X - 58.0,
        10.0,
        14.0,
    )
    .translate(0.0, -(COUPON_DRAWER_Y / 2.0 + 9.0), z)
        + centered_cube(
            format!("cleaning_validation_coupon_drawer_right_slide_key_{index}"),
            COUPON_DRAWER_X - 58.0,
            10.0,
            14.0,
        )
        .translate(0.0, COUPON_DRAWER_Y / 2.0 + 9.0, z)
}

fn clearance_frame(name: &str, width: f64, depth: f64, height: f64) -> Part {
    let rail = 10.0;
    let mut frame = Part::empty(format!("{name}_frame"));
    for (i, z) in [-(height / 2.0), height / 2.0].iter().enumerate() {
        frame = frame
            + centered_cube(format!("{name}_front_x_rail_{i}"), width, rail, rail).translate(
                0.0,
                -(depth / 2.0),
                *z,
            )
            + centered_cube(format!("{name}_rear_x_rail_{i}"), width, rail, rail).translate(
                0.0,
                depth / 2.0,
                *z,
            )
            + centered_cube(format!("{name}_left_y_rail_{i}"), rail, depth, rail).translate(
                -(width / 2.0),
                0.0,
                *z,
            )
            + centered_cube(format!("{name}_right_y_rail_{i}"), rail, depth, rail).translate(
                width / 2.0,
                0.0,
                *z,
            );
    }
    for (i, (x, y)) in [
        (-(width / 2.0), -(depth / 2.0)),
        (width / 2.0, -(depth / 2.0)),
        (-(width / 2.0), depth / 2.0),
        (width / 2.0, depth / 2.0),
    ]
    .iter()
    .enumerate()
    {
        frame = frame
            + centered_cube(format!("{name}_vertical_corner_{i}"), rail, rail, height)
                .translate(*x, *y, 0.0);
    }
    frame
}

fn oxidizer_material_notice_lands() -> Part {
    centered_cube(
        "cleaning_validation_vhp_h2o2_compatible_material_land",
        360.0,
        8.0,
        28.0,
    )
    .translate(-378.0, CART_Y / 2.0 + 44.0, 1010.0)
        + centered_cube(
            "cleaning_validation_silicone_vhp_uptake_warning_land",
            330.0,
            8.0,
            28.0,
        )
        .translate(294.0, CART_Y / 2.0 + 44.0, 1010.0)
}

fn port_collar(name: &str, outer_d: f64, inner_d: f64) -> Part {
    centered_cylinder(format!("{name}_outer"), outer_d / 2.0, 14.0, 36).rotate(90.0, 0.0, 0.0)
        - centered_cylinder(format!("{name}_inner"), inner_d / 2.0, 16.0, 32).rotate(90.0, 0.0, 0.0)
}

fn gasket_lip(name: &str, y: f64, z: f64, width_y: f64, height_z: f64) -> Part {
    let upper = centered_cube(
        format!("cleaning_validation_{name}_upper_gasket_lip"),
        22.0,
        width_y,
        10.0,
    )
    .translate(0.0, y, z + height_z / 2.0);
    let lower = centered_cube(
        format!("cleaning_validation_{name}_lower_gasket_lip"),
        22.0,
        width_y,
        10.0,
    )
    .translate(0.0, y, z - height_z / 2.0);
    let left = centered_cube(
        format!("cleaning_validation_{name}_left_gasket_lip"),
        22.0,
        10.0,
        height_z,
    )
    .translate(0.0, y - width_y / 2.0, z);
    let right = centered_cube(
        format!("cleaning_validation_{name}_right_gasket_lip"),
        22.0,
        10.0,
        height_z,
    )
    .translate(0.0, y + width_y / 2.0, z);
    upper + lower + left + right
}

fn drip_gutter() -> Part {
    let dirty_gutter = centered_cube(
        "cleaning_validation_bulkhead_dirty_side_drip_gutter",
        42.0,
        BULKHEAD_Y - 70.0,
        22.0,
    )
    .translate(DIRTY_SIDE_LIMIT_X + 12.0, 0.0, 486.0);
    let clean_witness_rail = centered_cube(
        "cleaning_validation_bulkhead_clean_side_witness_rail",
        14.0,
        BULKHEAD_Y - 96.0,
        18.0,
    )
    .translate(CLEAN_SIDE_LIMIT_X - 10.0, 0.0, 456.0);
    dirty_gutter + clean_witness_rail
}

fn lot_chain_of_custody_rail() -> Part {
    centered_cube(
        "cleaning_validation_lot_chain_of_custody_clip_rail",
        CART_X - 240.0,
        14.0,
        20.0,
    )
    .translate(0.0, -(CART_Y / 2.0 + 74.0), 942.0)
}

fn surface_sensor_pocket(index: usize) -> Part {
    let body = centered_cube(
        format!("cleaning_validation_surface_sensor_pocket_body_{index}"),
        72.0,
        52.0,
        18.0,
    );
    let pocket = centered_cube(
        format!("cleaning_validation_surface_sensor_probe_cavity_{index}"),
        48.0,
        30.0,
        20.0,
    )
    .translate(0.0, 0.0, 4.0);
    body - pocket
}

fn sensor_lot_label_strip() -> Part {
    centered_cube(
        "cleaning_validation_sensor_calibration_lot_label_strip",
        480.0,
        8.0,
        22.0,
    )
    .translate(-215.0, CART_Y / 2.0 + 44.0, 666.0)
}

fn robot_datum_targets() -> Part {
    let mut targets = Part::empty("cleaning_validation_robot_datum_targets");
    for (i, (x, y, z)) in [
        (-420.0, -238.0, 936.0),
        (-120.0, -238.0, 936.0),
        (210.0, -238.0, 936.0),
        (420.0, -238.0, 936.0),
    ]
    .iter()
    .enumerate()
    {
        targets = targets
            + centered_cylinder(
                format!("cleaning_validation_robot_datum_target_{i}"),
                24.0,
                8.0,
                40,
            )
            .translate(*x, *y, *z)
            - centered_cylinder(
                format!("cleaning_validation_robot_datum_cross_bore_{i}"),
                4.0,
                10.0,
                20,
            )
            .translate(*x, *y, *z);
    }
    targets
}

fn caster_plates() -> Part {
    let mut plates = Part::empty("cleaning_validation_cart_caster_plates");
    for (i, (x, y)) in [
        (-(CART_X / 2.0 - 72.0), -(CART_Y / 2.0 - 72.0)),
        (CART_X / 2.0 - 72.0, -(CART_Y / 2.0 - 72.0)),
        (-(CART_X / 2.0 - 72.0), CART_Y / 2.0 - 72.0),
        (CART_X / 2.0 - 72.0, CART_Y / 2.0 - 72.0),
        (0.0, -(CART_Y / 2.0 - 72.0)),
        (0.0, CART_Y / 2.0 - 72.0),
    ]
    .iter()
    .enumerate()
    {
        plates = plates
            + centered_cube(
                format!("cleaning_validation_cart_caster_plate_{i}"),
                96.0,
                96.0,
                12.0,
            )
            .translate(*x, *y, -6.0)
            - centered_cylinder(
                format!("cleaning_validation_cart_caster_stem_clearance_{i}"),
                12.0 / 2.0,
                14.0,
                24,
            )
            .translate(*x, *y, -6.0);
    }
    plates
}

fn dock_datum_bars() -> Part {
    centered_cube(
        "cleaning_validation_cart_front_workcell_dock_datum_bar",
        CART_X - 260.0,
        18.0,
        38.0,
    )
    .translate(0.0, -(CART_Y / 2.0 - 36.0), 118.0)
        + centered_cube(
            "cleaning_validation_cart_rear_workcell_dock_datum_bar",
            CART_X - 260.0,
            18.0,
            38.0,
        )
        .translate(0.0, CART_Y / 2.0 - 36.0, 118.0)
}

fn frame_post_points() -> [(f64, f64); 8] {
    [
        (
            -(CART_X / 2.0 - FRAME_W / 2.0),
            -(CART_Y / 2.0 - FRAME_W / 2.0),
        ),
        (
            CART_X / 2.0 - FRAME_W / 2.0,
            -(CART_Y / 2.0 - FRAME_W / 2.0),
        ),
        (
            -(CART_X / 2.0 - FRAME_W / 2.0),
            CART_Y / 2.0 - FRAME_W / 2.0,
        ),
        (CART_X / 2.0 - FRAME_W / 2.0, CART_Y / 2.0 - FRAME_W / 2.0),
        (0.0, -(CART_Y / 2.0 - FRAME_W / 2.0)),
        (0.0, CART_Y / 2.0 - FRAME_W / 2.0),
        (-(CART_X / 2.0 - FRAME_W / 2.0), 0.0),
        (CART_X / 2.0 - FRAME_W / 2.0, 0.0),
    ]
}

fn reservoir_y(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn port_x(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn coupon_drawer_z(index: usize) -> f64 {
    COUPON_DRAWER_BASE_Z + index as f64 * COUPON_DRAWER_PITCH_Z
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 13);
        for path in OUTPUTS {
            assert!(path.starts_with("output/closed_cleaning_sanitization_validation_cart_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn reservoir_modules_fit_their_clean_dirty_sides() {
        let clean_max_x = CLEAN_RESERVOIR_CENTER_X + CLEAN_RESERVOIR_D / 2.0;
        let rinse_max_x = RINSE_RESERVOIR_CENTER_X + RINSE_RESERVOIR_D / 2.0;
        let waste_min_x = WASTE_RETURN_CENTER_X - WASTE_RETURN_D / 2.0;
        assert!(clean_max_x < CLEAN_SIDE_LIMIT_X);
        assert!(rinse_max_x < CLEAN_SIDE_LIMIT_X);
        assert!(waste_min_x > DIRTY_SIDE_LIMIT_X);
        assert_eq!(
            CLEAN_RESERVOIR_COUNT + RINSE_RESERVOIR_COUNT + WASTE_RETURN_COUNT,
            7
        );
    }

    #[test]
    fn reservoir_arrays_fit_inside_cart_depth() {
        let clean_edge = reservoir_y(0, CLEAN_RESERVOIR_COUNT, CLEAN_RESERVOIR_PITCH_Y).abs()
            + CLEAN_RESERVOIR_D / 2.0;
        let rinse_edge = reservoir_y(0, RINSE_RESERVOIR_COUNT, RINSE_RESERVOIR_PITCH_Y).abs()
            + RINSE_RESERVOIR_D / 2.0;
        let waste_edge =
            reservoir_y(0, WASTE_RETURN_COUNT, WASTE_RETURN_PITCH_Y).abs() + WASTE_RETURN_D / 2.0;
        assert!(clean_edge < CART_Y / 2.0 - 70.0);
        assert!(rinse_edge < CART_Y / 2.0 - 92.0);
        assert!(waste_edge < CART_Y / 2.0 - 84.0);
    }

    #[test]
    fn validation_sampling_capacity_is_explicit() {
        assert_eq!(COUPON_DRAWER_COUNT, 3);
        assert_eq!(CONTACT_PLATE_RECESSES, 6);
        assert_eq!(SWAB_COUPON_SLOTS, 12);
        assert!(COUPON_DRAWER_X > 380.0);
        assert!(coupon_drawer_z(COUPON_DRAWER_COUNT - 1) + COUPON_DRAWER_Z / 2.0 < CART_Z);
    }

    #[test]
    fn flush_ports_and_sensors_cover_required_lands() {
        assert_eq!(FLUSH_PORT_COUNT, 8);
        assert_eq!(RETURN_PORT_COUNT, 6);
        assert_eq!(SURFACE_SENSOR_COUNT, 6);
        assert_eq!(ENV_SENSOR_COUNT, 5);
        assert_eq!(BARCODE_LAND_COUNT, 12);
        assert!(port_x(0, BARCODE_LAND_COUNT, 78.0).abs() + 31.0 < (CART_X - 170.0) / 2.0);
    }

    #[test]
    fn service_and_robot_keepouts_are_large_enough() {
        assert!(FRONT_SERVICE_CLEARANCE >= 500.0);
        assert!(REAR_VHP_CLEARANCE >= 250.0);
        assert!(SIDE_H2O2_CLEARANCE >= 160.0);
        assert!(ROBOT_KEEP_OUT_Z >= 400.0);
        assert!(BULKHEAD_Z < CART_Z);
    }
}
