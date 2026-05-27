use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed module clean-in-place / sterilize-in-place service station.
//
// Intent:
// - Dock a sealed 4x5 culture module without opening the culture volume.
// - Expose closed flush inlet, return, drain, sensor, and waste interfaces at
//   gasketed bulkheads so service can be automated and traceable.
// - Keep validation coupons, material compatibility labels, cycle/run scan
//   lands, clean/dirty segregation, and robot/service keepouts visible.
//
// This is product concept packaging CAD only. Sterilant, VHP/H2O2, cleaning,
// SIP, and validation geometry is an envelope/interface placeholder, not a
// cycle recipe, efficacy claim, or compliance statement.

const OUTPUTS: [&str; 13] = [
    "output/closed_module_cip_sip_service_station_base_deck.stl",
    "output/closed_module_cip_sip_service_station_sealed_module_dock.stl",
    "output/closed_module_cip_sip_service_station_flush_inlet_bulkhead.stl",
    "output/closed_module_cip_sip_service_station_return_drain_bulkhead.stl",
    "output/closed_module_cip_sip_service_station_drain_waste_containment.stl",
    "output/closed_module_cip_sip_service_station_validation_coupon_carrier.stl",
    "output/closed_module_cip_sip_service_station_temp_pressure_sensor_pockets.stl",
    "output/closed_module_cip_sip_service_station_sterilant_service_clearances.stl",
    "output/closed_module_cip_sip_service_station_material_compatibility_lands.stl",
    "output/closed_module_cip_sip_service_station_cycle_barcode_run_lands.stl",
    "output/closed_module_cip_sip_service_station_clean_dirty_segregation.stl",
    "output/closed_module_cip_sip_service_station_robot_service_keepouts.stl",
    "output/closed_module_cip_sip_service_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "sealed_module_dock",
    "closed_flush_inlet_bulkhead",
    "closed_return_drain_bulkhead",
    "drain_waste_containment",
    "validation_coupon_carrier",
    "temperature_pressure_sensor_pockets",
    "sterilant_service_clearances",
    "material_compatibility_lands",
    "cycle_barcode_run_lands",
    "clean_dirty_segregation",
    "robot_service_keepouts",
];

const COLS: usize = 4;
const ROWS: usize = 5;
const CHIP_COUNT: usize = COLS * ROWS;
const GUTTER: f64 = 5.0;
const ARRAY_X: f64 = COLS as f64 * REVC_CHIP_LENGTH + (COLS as f64 - 1.0) * GUTTER;
const ARRAY_Y: f64 = ROWS as f64 * REVC_CHIP_WIDTH + (ROWS as f64 - 1.0) * GUTTER;

const DECK_X: f64 = 1360.0;
const DECK_Y: f64 = 900.0;
const DECK_Z: f64 = 22.0;
const RAIL_Z: f64 = 38.0;
const RAIL_W: f64 = 24.0;
const MOUNT_HOLE_D: f64 = 6.6;

const MODULE_X: f64 = ARRAY_X + 190.0;
const MODULE_Y: f64 = ARRAY_Y + 155.0;
const MODULE_Z: f64 = 132.0;
const MODULE_DOCK_X: f64 = MODULE_X + 122.0;
const MODULE_DOCK_Y: f64 = MODULE_Y + 116.0;
const MODULE_DOCK_Z: f64 = 32.0;
const MODULE_CENTER_X: f64 = -112.0;
const MODULE_CENTER_Y: f64 = -14.0;
const MODULE_CLEARANCE: f64 = 6.0;
const MODULE_LOCATORS: usize = 6;
const MODULE_CLAMPS: usize = 8;

const PANEL_Y: f64 = 30.0;
const PANEL_Z: f64 = 252.0;
const PANEL_BASE_Y: f64 = DECK_Y / 2.0 - 54.0;
const CLEAN_PANEL_X: f64 = 500.0;
const DIRTY_PANEL_X: f64 = 520.0;
const CLEAN_PANEL_CENTER_X: f64 = -382.0;
const DIRTY_PANEL_CENTER_X: f64 = 380.0;
const PANEL_BASE_Z: f64 = DECK_Z / 2.0 + PANEL_Z / 2.0;

const FLUSH_INLET_PORTS: usize = 8;
const RETURN_PORTS: usize = 8;
const DRAIN_PORTS: usize = 4;
const SAMPLE_PORTS: usize = 4;
const PORT_PITCH: f64 = 55.0;
const CLEAN_PORT_D: f64 = 12.7;
const RETURN_PORT_D: f64 = 15.9;
const DRAIN_PORT_D: f64 = 19.0;

const WASTE_TRAY_X: f64 = MODULE_DOCK_X + 126.0;
const WASTE_TRAY_Y: f64 = MODULE_DOCK_Y + 92.0;
const WASTE_TRAY_Z: f64 = 42.0;
const WASTE_BOTTLE_COUNT: usize = 3;
const WASTE_BOTTLE_D: f64 = 94.0;
const WASTE_BOTTLE_Z: f64 = 190.0;
const WASTE_MANIFOLD_X: f64 = 410.0;
const WASTE_MANIFOLD_Y: f64 = 82.0;
const WASTE_MANIFOLD_Z: f64 = 72.0;

const COUPON_CARRIER_X: f64 = 468.0;
const COUPON_CARRIER_Y: f64 = 242.0;
const COUPON_CARRIER_Z: f64 = 62.0;
const COUPON_SLOTS: usize = 16;
const WITNESS_DISC_POCKETS: usize = 6;
const COUPON_CENTER_X: f64 = -438.0;
const COUPON_CENTER_Y: f64 = -DECK_Y / 2.0 + 154.0;

const TEMP_SENSOR_POCKETS: usize = 8;
const PRESSURE_SENSOR_POCKETS: usize = 8;
const AUX_SENSOR_POCKETS: usize = 4;
const SENSOR_RAIL_X: f64 = 785.0;
const SENSOR_RAIL_Y: f64 = 84.0;
const SENSOR_RAIL_Z: f64 = 72.0;
const SENSOR_RAIL_CENTER_Y: f64 = -DECK_Y / 2.0 + 96.0;

const MATERIAL_LABEL_LANDS: usize = 12;
const CYCLE_BARCODE_LANDS: usize = 10;
const RUN_TOKEN_SLOTS: usize = 4;

const FRONT_ROBOT_CLEARANCE: f64 = 520.0;
const REAR_SERVICE_CLEARANCE: f64 = 280.0;
const SIDE_STERILANT_CLEARANCE: f64 = 230.0;
const ROBOT_Z_CLEARANCE: f64 = 430.0;
const MODULE_LIFT_CLEARANCE_Z: f64 = 260.0;

fn main() {
    fs::create_dir_all("output").unwrap();

    let base = base_deck();
    export(OUTPUTS[0], &base);

    let dock = sealed_module_dock();
    export(OUTPUTS[1], &dock);

    let inlet = flush_inlet_bulkhead();
    export(OUTPUTS[2], &inlet);

    let return_bulkhead = return_drain_bulkhead();
    export(OUTPUTS[3], &return_bulkhead);

    let containment = drain_waste_containment();
    export(OUTPUTS[4], &containment);

    let coupons = validation_coupon_carrier();
    export(OUTPUTS[5], &coupons);

    let sensors = temp_pressure_sensor_pockets();
    export(OUTPUTS[6], &sensors);

    let clearances = sterilant_service_clearances();
    export(OUTPUTS[7], &clearances);

    let material_lands = material_compatibility_lands();
    export(OUTPUTS[8], &material_lands);

    let cycle_lands = cycle_barcode_run_lands();
    export(OUTPUTS[9], &cycle_lands);

    let segregation = clean_dirty_segregation();
    export(OUTPUTS[10], &segregation);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[11], &keepouts);

    let assembly = base
        + dock
        + inlet
        + return_bulkhead
        + containment
        + coupons
        + sensors
        + clearances
        + material_lands
        + cycle_lands
        + segregation
        + keepouts;
    export(OUTPUTS[12], &assembly);

    println!(
        "Closed module CIP/SIP service station: {:.0} x {:.0}mm deck, sealed module dock {:.1} x {:.1}mm for {} Rev C chips, {} flush inlets, {} return ports, {} drains, {} temp pockets, {} pressure pockets, {} coupon slots, {} barcode/run lands, and {} required feature groups.",
        DECK_X,
        DECK_Y,
        MODULE_DOCK_X,
        MODULE_DOCK_Y,
        CHIP_COUNT,
        FLUSH_INLET_PORTS,
        RETURN_PORTS,
        DRAIN_PORTS,
        TEMP_SENSOR_POCKETS,
        PRESSURE_SENSOR_POCKETS,
        COUPON_SLOTS,
        CYCLE_BARCODE_LANDS,
        REQUIRED_FEATURES.len()
    );
    println!(
        "Service envelopes modeled: {:.0}mm front robot approach, {:.0}mm rear utility access, {:.0}mm side sterilant cart clearance, {:.0}mm Z robot clearance. Geometry is packaging/interface CAD only.",
        FRONT_ROBOT_CLEARANCE,
        REAR_SERVICE_CLEARANCE,
        SIDE_STERILANT_CLEARANCE,
        ROBOT_Z_CLEARANCE
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn base_deck() -> Part {
    let deck = centered_cube("cip_sip_station_base_deck", DECK_X, DECK_Y, DECK_Z);
    let module_socket = centered_cube(
        "cip_sip_station_module_dock_registration_recess",
        MODULE_DOCK_X + 28.0,
        MODULE_DOCK_Y + 24.0,
        8.0,
    )
    .translate(MODULE_CENTER_X, MODULE_CENTER_Y, DECK_Z / 2.0 - 4.0);
    let rear_panel_socket = centered_cube(
        "cip_sip_station_rear_bulkhead_socket",
        DECK_X - 130.0,
        18.0,
        8.0,
    )
    .translate(0.0, PANEL_BASE_Y, DECK_Z / 2.0 - 3.0);
    let front_coupon_socket = centered_cube(
        "cip_sip_station_coupon_carrier_socket",
        COUPON_CARRIER_X + 26.0,
        COUPON_CARRIER_Y + 28.0,
        7.0,
    )
    .translate(COUPON_CENTER_X, COUPON_CENTER_Y, DECK_Z / 2.0 - 3.0);
    let dirty_side_drain_channel = centered_cube(
        "cip_sip_station_dirty_side_drain_channel",
        30.0,
        MODULE_DOCK_Y + 150.0,
        10.0,
    )
    .translate(
        DIRTY_PANEL_CENTER_X - 170.0,
        MODULE_CENTER_Y - 4.0,
        DECK_Z / 2.0 - 4.0,
    );
    let low_point_drain = centered_cylinder(
        "cip_sip_station_base_low_point_drain_to_waste",
        11.0,
        48.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DIRTY_PANEL_CENTER_X - 172.0, -DECK_Y / 2.0 + 48.0, 0.0);

    deck - module_socket
        - rear_panel_socket
        - front_coupon_socket
        - dirty_side_drain_channel
        - low_point_drain
        - base_mounting_holes()
        + base_perimeter_rails()
        + panel_gussets()
        + base_robot_fiducials()
        + fork_lift_pockets()
}

fn base_perimeter_rails() -> Part {
    let left = centered_cube(
        "cip_sip_station_left_low_rail",
        RAIL_W,
        DECK_Y - 96.0,
        RAIL_Z,
    )
    .translate(-(DECK_X / 2.0 - 42.0), 0.0, DECK_Z / 2.0 + RAIL_Z / 2.0);
    let right = centered_cube(
        "cip_sip_station_right_low_rail",
        RAIL_W,
        DECK_Y - 96.0,
        RAIL_Z,
    )
    .translate(DECK_X / 2.0 - 42.0, 0.0, DECK_Z / 2.0 + RAIL_Z / 2.0);
    let rear = centered_cube(
        "cip_sip_station_rear_bulkhead_mount_rail",
        DECK_X - 124.0,
        RAIL_W,
        RAIL_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - 42.0, DECK_Z / 2.0 + RAIL_Z / 2.0);
    let front_left = centered_cube(
        "cip_sip_station_front_clean_side_lip",
        DECK_X / 2.0 - 130.0,
        16.0,
        30.0,
    )
    .translate(
        -(DECK_X / 4.0 + 24.0),
        -DECK_Y / 2.0 + 35.0,
        DECK_Z / 2.0 + 15.0,
    );
    let front_right = centered_cube(
        "cip_sip_station_front_dirty_side_lip",
        DECK_X / 2.0 - 130.0,
        16.0,
        30.0,
    )
    .translate(
        DECK_X / 4.0 + 24.0,
        -DECK_Y / 2.0 + 35.0,
        DECK_Z / 2.0 + 15.0,
    );

    left + right + rear + front_left + front_right
}

fn base_mounting_holes() -> Part {
    let mut holes = Part::empty("cip_sip_station_base_mounting_holes");
    for (i, (x, y)) in base_mount_points().iter().enumerate() {
        let round = centered_cylinder(
            format!("cip_sip_station_m6_mount_clearance_{i}"),
            MOUNT_HOLE_D / 2.0,
            DECK_Z + 6.0,
            24,
        )
        .translate(*x, *y, 0.0);
        let slot = centered_cube(
            format!("cip_sip_station_m6_mount_slot_{i}"),
            24.0,
            MOUNT_HOLE_D + 0.6,
            DECK_Z + 6.0,
        )
        .translate(*x, *y, 0.0);
        holes = holes + round + slot;
    }
    holes
}

fn base_mount_points() -> [(f64, f64); 10] {
    [
        (-(DECK_X / 2.0 - 72.0), -(DECK_Y / 2.0 - 72.0)),
        (DECK_X / 2.0 - 72.0, -(DECK_Y / 2.0 - 72.0)),
        (-(DECK_X / 2.0 - 72.0), DECK_Y / 2.0 - 72.0),
        (DECK_X / 2.0 - 72.0, DECK_Y / 2.0 - 72.0),
        (0.0, -(DECK_Y / 2.0 - 72.0)),
        (0.0, DECK_Y / 2.0 - 72.0),
        (
            MODULE_CENTER_X - MODULE_DOCK_X / 2.0 + 70.0,
            MODULE_CENTER_Y,
        ),
        (
            MODULE_CENTER_X + MODULE_DOCK_X / 2.0 - 70.0,
            MODULE_CENTER_Y,
        ),
        (DIRTY_PANEL_CENTER_X, PANEL_BASE_Y - 92.0),
        (CLEAN_PANEL_CENTER_X, PANEL_BASE_Y - 92.0),
    ]
}

fn panel_gussets() -> Part {
    let mut gussets = Part::empty("cip_sip_station_rear_panel_gussets");
    for (i, x) in [
        -560.0, -420.0, -280.0, -140.0, 0.0, 140.0, 280.0, 420.0, 560.0,
    ]
    .iter()
    .enumerate()
    {
        let web = centered_cube(format!("cip_sip_station_gusset_web_{i}"), 14.0, 72.0, 96.0)
            .translate(*x, PANEL_BASE_Y - 42.0, DECK_Z / 2.0 + 48.0);
        let foot = centered_cube(format!("cip_sip_station_gusset_foot_{i}"), 50.0, 62.0, 10.0)
            .translate(*x, PANEL_BASE_Y - 42.0, DECK_Z / 2.0 + 5.0);
        let bolt = centered_cylinder(
            format!("cip_sip_station_gusset_m5_clearance_{i}"),
            5.4 / 2.0,
            14.0,
            24,
        )
        .translate(*x, PANEL_BASE_Y - 56.0, DECK_Z / 2.0 + 5.0);
        gussets = gussets + (web + foot - bolt);
    }
    gussets
}

fn base_robot_fiducials() -> Part {
    let mut fiducials = Part::empty("cip_sip_station_base_robot_fiducials");
    for (i, (x, y)) in [
        (-(DECK_X / 2.0 - 82.0), -(DECK_Y / 2.0 - 88.0)),
        (DECK_X / 2.0 - 82.0, -(DECK_Y / 2.0 - 88.0)),
        (-(DECK_X / 2.0 - 82.0), DECK_Y / 2.0 - 88.0),
        (DECK_X / 2.0 - 82.0, DECK_Y / 2.0 - 88.0),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + fiducial_target(&format!("cip_sip_station_robot_fiducial_{i}")).translate(
                *x,
                *y,
                DECK_Z / 2.0 + 2.0,
            );
    }
    fiducials
}

fn fork_lift_pockets() -> Part {
    let front = centered_cube(
        "cip_sip_station_front_fork_pocket_proxy",
        DECK_X - 210.0,
        34.0,
        24.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 88.0, DECK_Z / 2.0 + 12.0);
    let rear = centered_cube(
        "cip_sip_station_rear_fork_pocket_proxy",
        DECK_X - 210.0,
        34.0,
        24.0,
    )
    .translate(0.0, DECK_Y / 2.0 - 112.0, DECK_Z / 2.0 + 12.0);
    front + rear
}

fn sealed_module_dock() -> Part {
    let outer = centered_cube(
        "cip_sip_station_module_dock_outer_tray",
        MODULE_DOCK_X,
        MODULE_DOCK_Y,
        MODULE_DOCK_Z,
    );
    let basin = centered_cube(
        "cip_sip_station_module_dock_recess_for_sealed_module",
        MODULE_X + MODULE_CLEARANCE,
        MODULE_Y + MODULE_CLEARANCE,
        MODULE_DOCK_Z - 8.0,
    )
    .translate(0.0, 0.0, 7.0);
    let gasket_channel = centered_cube(
        "cip_sip_station_module_dock_gasket_shadow",
        MODULE_X + 32.0,
        MODULE_Y + 32.0,
        6.0,
    )
    .translate(0.0, 0.0, MODULE_DOCK_Z / 2.0 - 2.0);
    let dock_body = outer - basin - gasket_channel;

    let rails = module_dock_rails();
    let locators = module_dock_locators();
    let clamps = module_dock_clamps();
    let envelope = clearance_frame(
        "cip_sip_station_sealed_module_envelope_gauge",
        MODULE_X,
        MODULE_Y,
        MODULE_Z,
    )
    .translate(0.0, 0.0, MODULE_DOCK_Z / 2.0 + MODULE_Z / 2.0 + 8.0);
    let connector_shadow = module_connector_shadow();

    (dock_body + rails + locators + clamps + envelope + connector_shadow).translate(
        MODULE_CENTER_X,
        MODULE_CENTER_Y,
        DECK_Z / 2.0 + MODULE_DOCK_Z / 2.0 + 2.0,
    )
}

fn module_dock_rails() -> Part {
    let left = centered_cube(
        "cip_sip_station_module_left_datum_rail",
        22.0,
        MODULE_Y + 76.0,
        34.0,
    )
    .translate(-(MODULE_X / 2.0 + 35.0), 0.0, MODULE_DOCK_Z / 2.0 + 17.0);
    let right = centered_cube(
        "cip_sip_station_module_right_datum_rail",
        22.0,
        MODULE_Y + 76.0,
        34.0,
    )
    .translate(MODULE_X / 2.0 + 35.0, 0.0, MODULE_DOCK_Z / 2.0 + 17.0);
    let rear = centered_cube(
        "cip_sip_station_module_rear_hard_stop",
        MODULE_X + 96.0,
        24.0,
        40.0,
    )
    .translate(0.0, MODULE_Y / 2.0 + 40.0, MODULE_DOCK_Z / 2.0 + 20.0);
    let front_left = centered_cube(
        "cip_sip_station_module_front_left_stop",
        MODULE_X / 2.0 - 38.0,
        20.0,
        34.0,
    )
    .translate(
        -(MODULE_X / 4.0 + 24.0),
        -(MODULE_Y / 2.0 + 31.0),
        MODULE_DOCK_Z / 2.0 + 17.0,
    );
    let front_right = centered_cube(
        "cip_sip_station_module_front_right_stop",
        MODULE_X / 2.0 - 38.0,
        20.0,
        34.0,
    )
    .translate(
        MODULE_X / 4.0 + 24.0,
        -(MODULE_Y / 2.0 + 31.0),
        MODULE_DOCK_Z / 2.0 + 17.0,
    );
    left + right + rear + front_left + front_right
}

fn module_dock_locators() -> Part {
    let mut locators = Part::empty("cip_sip_station_module_locator_bosses");
    for (i, (x, y)) in module_locator_points().iter().enumerate() {
        let boss = centered_cylinder(
            format!("cip_sip_station_module_locator_boss_{i}"),
            14.0,
            12.0,
            36,
        )
        .translate(*x, *y, MODULE_DOCK_Z / 2.0 + 6.0);
        let dowel = centered_cylinder(
            format!("cip_sip_station_module_locator_dowel_socket_{i}"),
            3.2,
            14.0,
            24,
        )
        .translate(*x, *y, MODULE_DOCK_Z / 2.0 + 6.0);
        locators = locators + (boss - dowel);
    }
    locators
}

fn module_locator_points() -> [(f64, f64); MODULE_LOCATORS] {
    [
        (-(MODULE_X / 2.0 - 62.0), -(MODULE_Y / 2.0 - 58.0)),
        (MODULE_X / 2.0 - 62.0, -(MODULE_Y / 2.0 - 58.0)),
        (-(MODULE_X / 2.0 - 62.0), MODULE_Y / 2.0 - 58.0),
        (MODULE_X / 2.0 - 62.0, MODULE_Y / 2.0 - 58.0),
        (0.0, -(MODULE_Y / 2.0 - 58.0)),
        (0.0, MODULE_Y / 2.0 - 58.0),
    ]
}

fn module_dock_clamps() -> Part {
    let mut clamps = Part::empty("cip_sip_station_module_overcenter_clamps");
    for (i, (x, y, rot)) in clamp_points().iter().enumerate() {
        let pad = centered_cube(
            format!("cip_sip_station_module_clamp_pad_{i}"),
            58.0,
            30.0,
            18.0,
        )
        .rotate(0.0, 0.0, *rot)
        .translate(*x, *y, MODULE_DOCK_Z / 2.0 + 28.0);
        let hinge = centered_cylinder(
            format!("cip_sip_station_module_clamp_hinge_pin_{i}"),
            6.0,
            48.0,
            24,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(*x, *y, MODULE_DOCK_Z / 2.0 + 15.0);
        clamps = clamps + pad + hinge;
    }
    clamps
}

fn clamp_points() -> [(f64, f64, f64); MODULE_CLAMPS] {
    [
        (-(MODULE_X / 2.0 + 49.0), -210.0, 0.0),
        (-(MODULE_X / 2.0 + 49.0), 0.0, 0.0),
        (-(MODULE_X / 2.0 + 49.0), 210.0, 0.0),
        (MODULE_X / 2.0 + 49.0, -210.0, 0.0),
        (MODULE_X / 2.0 + 49.0, 0.0, 0.0),
        (MODULE_X / 2.0 + 49.0, 210.0, 0.0),
        (-160.0, MODULE_Y / 2.0 + 58.0, 90.0),
        (160.0, MODULE_Y / 2.0 + 58.0, 90.0),
    ]
}

fn module_connector_shadow() -> Part {
    let inlet_shadow = centered_cube(
        "cip_sip_station_module_clean_connector_shadow",
        210.0,
        40.0,
        32.0,
    )
    .translate(
        -MODULE_X / 2.0 + 145.0,
        MODULE_Y / 2.0 + 70.0,
        MODULE_DOCK_Z / 2.0 + 38.0,
    );
    let return_shadow = centered_cube(
        "cip_sip_station_module_dirty_connector_shadow",
        230.0,
        40.0,
        32.0,
    )
    .translate(
        MODULE_X / 2.0 - 155.0,
        MODULE_Y / 2.0 + 70.0,
        MODULE_DOCK_Z / 2.0 + 38.0,
    );
    inlet_shadow + return_shadow
}

fn flush_inlet_bulkhead() -> Part {
    let panel = centered_cube(
        "cip_sip_station_clean_flush_inlet_bulkhead_panel",
        CLEAN_PANEL_X,
        PANEL_Y,
        PANEL_Z,
    )
    .translate(CLEAN_PANEL_CENTER_X, PANEL_BASE_Y, PANEL_BASE_Z);
    let mut holes = Part::empty("cip_sip_station_clean_flush_port_holes");
    let mut collars = Part::empty("cip_sip_station_clean_flush_port_collars");
    let mut tube_stubs = Part::empty("cip_sip_station_clean_flush_tube_stubs");

    for i in 0..FLUSH_INLET_PORTS {
        let x = CLEAN_PANEL_CENTER_X + port_x(i, FLUSH_INLET_PORTS, PORT_PITCH);
        let z = PANEL_BASE_Z + 52.0;
        holes = holes
            + centered_cylinder(
                format!("cip_sip_station_flush_inlet_port_clearance_{i}"),
                CLEAN_PORT_D / 2.0,
                PANEL_Y + 8.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, PANEL_BASE_Y, z);
        collars = collars
            + port_collar(
                &format!("cip_sip_station_flush_inlet_tri_clamp_land_{i}"),
                36.0,
                CLEAN_PORT_D,
            )
            .translate(x, PANEL_BASE_Y - 24.0, z);
        tube_stubs = tube_stubs
            + centered_cylinder(
                format!("cip_sip_station_flush_inlet_closed_tube_stub_{i}"),
                4.2,
                86.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, PANEL_BASE_Y - 70.0, z);
    }

    let air_break_shadow = centered_cube(
        "cip_sip_station_clean_flush_pump_airbreak_shadow",
        CLEAN_PANEL_X - 78.0,
        22.0,
        54.0,
    )
    .translate(
        CLEAN_PANEL_CENTER_X,
        PANEL_BASE_Y - 34.0,
        PANEL_BASE_Z - 62.0,
    );

    panel - holes + collars + tube_stubs + air_break_shadow + inlet_bulkhead_label_strip()
}

fn inlet_bulkhead_label_strip() -> Part {
    let strip = centered_cube(
        "cip_sip_station_clean_inlet_lot_label_strip",
        CLEAN_PANEL_X - 72.0,
        10.0,
        38.0,
    )
    .translate(
        CLEAN_PANEL_CENTER_X,
        PANEL_BASE_Y - 22.0,
        PANEL_BASE_Z + 112.0,
    );
    let mut lands = Part::empty("cip_sip_station_clean_inlet_scan_lands");
    for i in 0..FLUSH_INLET_PORTS {
        let x = CLEAN_PANEL_CENTER_X + port_x(i, FLUSH_INLET_PORTS, PORT_PITCH);
        lands = lands
            + centered_cube(
                format!("cip_sip_station_clean_inlet_port_label_land_{i}"),
                38.0,
                6.0,
                14.0,
            )
            .translate(x, PANEL_BASE_Y - 29.0, PANEL_BASE_Z + 112.0);
    }
    strip + lands
}

fn return_drain_bulkhead() -> Part {
    let panel = centered_cube(
        "cip_sip_station_return_drain_bulkhead_panel",
        DIRTY_PANEL_X,
        PANEL_Y,
        PANEL_Z,
    )
    .translate(DIRTY_PANEL_CENTER_X, PANEL_BASE_Y, PANEL_BASE_Z);
    let mut holes = Part::empty("cip_sip_station_return_drain_port_holes");
    let mut collars = Part::empty("cip_sip_station_return_drain_port_collars");
    let mut tubes = Part::empty("cip_sip_station_return_drain_tube_stubs");

    for i in 0..RETURN_PORTS {
        let x = DIRTY_PANEL_CENTER_X + port_x(i, RETURN_PORTS, PORT_PITCH);
        let z = PANEL_BASE_Z + 58.0;
        holes = holes
            + centered_cylinder(
                format!("cip_sip_station_return_port_clearance_{i}"),
                RETURN_PORT_D / 2.0,
                PANEL_Y + 8.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, PANEL_BASE_Y, z);
        collars = collars
            + port_collar(
                &format!("cip_sip_station_return_port_collar_{i}"),
                42.0,
                RETURN_PORT_D,
            )
            .translate(x, PANEL_BASE_Y - 24.0, z);
        tubes = tubes
            + centered_cylinder(
                format!("cip_sip_station_return_closed_tube_stub_{i}"),
                5.4,
                92.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, PANEL_BASE_Y - 74.0, z);
    }

    for i in 0..DRAIN_PORTS {
        let x = DIRTY_PANEL_CENTER_X + port_x(i, DRAIN_PORTS, 88.0);
        let z = PANEL_BASE_Z - 74.0;
        holes = holes
            + centered_cylinder(
                format!("cip_sip_station_drain_port_clearance_{i}"),
                DRAIN_PORT_D / 2.0,
                PANEL_Y + 8.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, PANEL_BASE_Y, z);
        collars = collars
            + port_collar(
                &format!("cip_sip_station_drain_port_collar_{i}"),
                50.0,
                DRAIN_PORT_D,
            )
            .translate(x, PANEL_BASE_Y - 24.0, z);
    }

    panel - holes + collars + tubes + return_sample_port_bank() + dirty_bulkhead_gutter()
}

fn return_sample_port_bank() -> Part {
    let panel = centered_cube(
        "cip_sip_station_return_sample_port_panel",
        300.0,
        20.0,
        54.0,
    )
    .translate(
        DIRTY_PANEL_CENTER_X,
        PANEL_BASE_Y - 34.0,
        PANEL_BASE_Z + 116.0,
    );
    let mut ports = Part::empty("cip_sip_station_return_sample_ports");
    for i in 0..SAMPLE_PORTS {
        let x = DIRTY_PANEL_CENTER_X + port_x(i, SAMPLE_PORTS, 66.0);
        let collar = port_collar(
            &format!("cip_sip_station_return_sample_collar_{i}"),
            30.0,
            10.0,
        )
        .translate(x, PANEL_BASE_Y - 48.0, PANEL_BASE_Z + 116.0);
        ports = ports + collar;
    }
    panel + ports
}

fn dirty_bulkhead_gutter() -> Part {
    centered_cube(
        "cip_sip_station_dirty_return_drip_gutter",
        DIRTY_PANEL_X - 56.0,
        42.0,
        22.0,
    )
    .translate(
        DIRTY_PANEL_CENTER_X,
        PANEL_BASE_Y - 38.0,
        PANEL_BASE_Z - 120.0,
    )
}

fn drain_waste_containment() -> Part {
    let tray_outer = centered_cube(
        "cip_sip_station_drain_waste_secondary_tray",
        WASTE_TRAY_X,
        WASTE_TRAY_Y,
        WASTE_TRAY_Z,
    )
    .translate(
        MODULE_CENTER_X,
        MODULE_CENTER_Y - 6.0,
        DECK_Z / 2.0 + WASTE_TRAY_Z / 2.0 + 4.0,
    );
    let basin = centered_cube(
        "cip_sip_station_drain_waste_basin_cut",
        WASTE_TRAY_X - 64.0,
        WASTE_TRAY_Y - 64.0,
        WASTE_TRAY_Z - 14.0,
    )
    .translate(
        MODULE_CENTER_X,
        MODULE_CENTER_Y - 6.0,
        DECK_Z / 2.0 + WASTE_TRAY_Z / 2.0 + 13.0,
    );
    let sump = centered_cube(
        "cip_sip_station_dirty_side_low_point_sump",
        190.0,
        74.0,
        24.0,
    )
    .translate(
        DIRTY_PANEL_CENTER_X - 170.0,
        -DECK_Y / 2.0 + 94.0,
        DECK_Z / 2.0 + 20.0,
    );
    let sump_drain = centered_cylinder("cip_sip_station_sump_drain_nozzle", 9.5, 64.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(
            DIRTY_PANEL_CENTER_X - 170.0,
            -DECK_Y / 2.0 + 56.0,
            DECK_Z / 2.0 + 20.0,
        );

    (tray_outer - basin)
        + containment_flow_ribs()
        + leak_sensor_wells()
        + sump
        + sump_drain
        + waste_bottle_cradles()
        + waste_manifold()
}

fn containment_flow_ribs() -> Part {
    let mut ribs = Part::empty("cip_sip_station_containment_flow_ribs");
    for row in 0..ROWS {
        let y = MODULE_CENTER_Y - ARRAY_Y / 2.0
            + REVC_CHIP_WIDTH / 2.0
            + row as f64 * (REVC_CHIP_WIDTH + GUTTER);
        ribs = ribs
            + centered_cube(
                format!("cip_sip_station_containment_row_rib_{row}"),
                ARRAY_X + 110.0,
                4.0,
                8.0,
            )
            .translate(MODULE_CENTER_X, y, DECK_Z / 2.0 + WASTE_TRAY_Z + 2.0);
    }
    ribs
}

fn leak_sensor_wells() -> Part {
    let mut wells = Part::empty("cip_sip_station_leak_sensor_wells");
    for (i, (x, y)) in [
        (
            MODULE_CENTER_X - WASTE_TRAY_X / 2.0 + 64.0,
            MODULE_CENTER_Y - WASTE_TRAY_Y / 2.0 + 64.0,
        ),
        (
            MODULE_CENTER_X + WASTE_TRAY_X / 2.0 - 64.0,
            MODULE_CENTER_Y - WASTE_TRAY_Y / 2.0 + 64.0,
        ),
        (
            MODULE_CENTER_X + WASTE_TRAY_X / 2.0 - 64.0,
            MODULE_CENTER_Y + WASTE_TRAY_Y / 2.0 - 64.0,
        ),
        (
            MODULE_CENTER_X - WASTE_TRAY_X / 2.0 + 64.0,
            MODULE_CENTER_Y + WASTE_TRAY_Y / 2.0 - 64.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        let rim = centered_cube(
            format!("cip_sip_station_leak_sensor_rim_{i}"),
            50.0,
            34.0,
            10.0,
        )
        .translate(*x, *y, DECK_Z / 2.0 + WASTE_TRAY_Z + 6.0);
        let pocket = centered_cube(
            format!("cip_sip_station_leak_sensor_pocket_{i}"),
            36.0,
            20.0,
            12.0,
        )
        .translate(*x, *y, DECK_Z / 2.0 + WASTE_TRAY_Z + 6.0);
        wells = wells + (rim - pocket);
    }
    wells
}

fn waste_bottle_cradles() -> Part {
    let mut cradles = Part::empty("cip_sip_station_waste_bottle_cradles");
    for i in 0..WASTE_BOTTLE_COUNT {
        let x = DIRTY_PANEL_CENTER_X + port_x(i, WASTE_BOTTLE_COUNT, 125.0);
        let y = -DECK_Y / 2.0 + 224.0;
        let shell = centered_cylinder(
            format!("cip_sip_station_waste_bottle_cradle_{i}"),
            WASTE_BOTTLE_D / 2.0 + 12.0,
            WASTE_BOTTLE_Z,
            56,
        )
        .translate(x, y, DECK_Z / 2.0 + WASTE_BOTTLE_Z / 2.0 + 16.0);
        let cavity = centered_cylinder(
            format!("cip_sip_station_waste_bottle_cavity_{i}"),
            WASTE_BOTTLE_D / 2.0 + 2.0,
            WASTE_BOTTLE_Z + 4.0,
            56,
        )
        .translate(x, y, DECK_Z / 2.0 + WASTE_BOTTLE_Z / 2.0 + 28.0);
        let sight_slot = centered_cube(
            format!("cip_sip_station_waste_bottle_sight_slot_{i}"),
            12.0,
            WASTE_BOTTLE_D + 26.0,
            WASTE_BOTTLE_Z - 66.0,
        )
        .translate(
            x + WASTE_BOTTLE_D / 2.0,
            y,
            DECK_Z / 2.0 + WASTE_BOTTLE_Z / 2.0 + 16.0,
        );
        cradles = cradles + (shell - cavity - sight_slot);
    }
    cradles
}

fn waste_manifold() -> Part {
    let block = centered_cube(
        "cip_sip_station_waste_return_manifold_block",
        WASTE_MANIFOLD_X,
        WASTE_MANIFOLD_Y,
        WASTE_MANIFOLD_Z,
    )
    .translate(
        DIRTY_PANEL_CENTER_X,
        -DECK_Y / 2.0 + 344.0,
        DECK_Z / 2.0 + WASTE_MANIFOLD_Z / 2.0 + 26.0,
    );
    let mut bores = Part::empty("cip_sip_station_waste_manifold_bores");
    for i in 0..(RETURN_PORTS / 2) {
        bores = bores
            + centered_cylinder(
                format!("cip_sip_station_waste_manifold_lane_bore_{i}"),
                5.8,
                WASTE_MANIFOLD_Y + 8.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                DIRTY_PANEL_CENTER_X + port_x(i, RETURN_PORTS / 2, 68.0),
                -DECK_Y / 2.0 + 344.0,
                DECK_Z / 2.0 + WASTE_MANIFOLD_Z / 2.0 + 26.0,
            );
    }
    block - bores
}

fn validation_coupon_carrier() -> Part {
    let body = centered_cube(
        "cip_sip_station_validation_coupon_carrier_body",
        COUPON_CARRIER_X,
        COUPON_CARRIER_Y,
        COUPON_CARRIER_Z,
    )
    .translate(
        COUPON_CENTER_X,
        COUPON_CENTER_Y,
        DECK_Z / 2.0 + COUPON_CARRIER_Z / 2.0 + 6.0,
    );
    let tray_cavity = centered_cube(
        "cip_sip_station_validation_coupon_open_tray",
        COUPON_CARRIER_X - 44.0,
        COUPON_CARRIER_Y - 42.0,
        COUPON_CARRIER_Z - 22.0,
    )
    .translate(
        COUPON_CENTER_X,
        COUPON_CENTER_Y,
        DECK_Z / 2.0 + COUPON_CARRIER_Z / 2.0 + 18.0,
    );
    let mut coupon_slots = Part::empty("cip_sip_station_validation_coupon_slots");
    for i in 0..COUPON_SLOTS {
        let col = i % 8;
        let row = i / 8;
        let x = COUPON_CENTER_X + port_x(col, 8, 50.0);
        let y = COUPON_CENTER_Y - 42.0 + row as f64 * 84.0;
        let slot = centered_cube(
            format!("cip_sip_station_validation_coupon_slot_{i}"),
            34.0,
            54.0,
            13.0,
        )
        .translate(x, y, DECK_Z / 2.0 + COUPON_CARRIER_Z + 4.0);
        coupon_slots = coupon_slots + slot;
    }
    let mut witness_disc_pockets = Part::empty("cip_sip_station_witness_disc_pockets");
    for i in 0..WITNESS_DISC_POCKETS {
        let x = COUPON_CENTER_X + port_x(i, WITNESS_DISC_POCKETS, 62.0);
        let y = COUPON_CENTER_Y + COUPON_CARRIER_Y / 2.0 - 44.0;
        witness_disc_pockets = witness_disc_pockets
            + centered_cylinder(
                format!("cip_sip_station_witness_disc_pocket_{i}"),
                16.0,
                14.0,
                36,
            )
            .translate(x, y, DECK_Z / 2.0 + COUPON_CARRIER_Z + 5.0);
    }
    let handle = centered_cube(
        "cip_sip_station_validation_coupon_drawer_pull",
        COUPON_CARRIER_X - 120.0,
        18.0,
        28.0,
    )
    .translate(
        COUPON_CENTER_X,
        COUPON_CENTER_Y - COUPON_CARRIER_Y / 2.0 - 17.0,
        DECK_Z / 2.0 + COUPON_CARRIER_Z / 2.0 + 22.0,
    );
    let barcode_land = centered_cube(
        "cip_sip_station_coupon_carrier_barcode_land",
        190.0,
        8.0,
        22.0,
    )
    .translate(
        COUPON_CENTER_X,
        COUPON_CENTER_Y - COUPON_CARRIER_Y / 2.0 - 30.0,
        DECK_Z / 2.0 + COUPON_CARRIER_Z + 2.0,
    );

    body - tray_cavity - coupon_slots - witness_disc_pockets + handle + barcode_land
}

fn temp_pressure_sensor_pockets() -> Part {
    let rail = centered_cube(
        "cip_sip_station_temp_pressure_sensor_rail",
        SENSOR_RAIL_X,
        SENSOR_RAIL_Y,
        SENSOR_RAIL_Z,
    )
    .translate(
        MODULE_CENTER_X,
        SENSOR_RAIL_CENTER_Y,
        DECK_Z / 2.0 + SENSOR_RAIL_Z / 2.0 + 8.0,
    );
    let mut cuts = Part::empty("cip_sip_station_sensor_pocket_cuts");
    let mut collars = Part::empty("cip_sip_station_sensor_pocket_collars");

    for i in 0..TEMP_SENSOR_POCKETS {
        let x = MODULE_CENTER_X + port_x(i, TEMP_SENSOR_POCKETS, 78.0);
        let y = SENSOR_RAIL_CENTER_Y - 20.0;
        let z = DECK_Z / 2.0 + SENSOR_RAIL_Z / 2.0 + 26.0;
        cuts = cuts
            + centered_cylinder(
                format!("cip_sip_station_temperature_probe_pocket_cut_{i}"),
                8.0,
                SENSOR_RAIL_Y + 8.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, y, z);
        collars = collars
            + port_collar(
                &format!("cip_sip_station_temperature_probe_collar_{i}"),
                28.0,
                12.0,
            )
            .translate(x, y - 34.0, z);
    }

    for i in 0..PRESSURE_SENSOR_POCKETS {
        let x = MODULE_CENTER_X + port_x(i, PRESSURE_SENSOR_POCKETS, 78.0);
        let y = SENSOR_RAIL_CENTER_Y - 20.0;
        let z = DECK_Z / 2.0 + SENSOR_RAIL_Z / 2.0 - 14.0;
        cuts = cuts
            + centered_cylinder(
                format!("cip_sip_station_pressure_transducer_pocket_cut_{i}"),
                10.5,
                SENSOR_RAIL_Y + 8.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, y, z);
        collars = collars
            + port_collar(
                &format!("cip_sip_station_pressure_transducer_collar_{i}"),
                34.0,
                16.0,
            )
            .translate(x, y - 34.0, z);
    }

    let aux_panel = centered_cube(
        "cip_sip_station_aux_sensor_coupon_panel",
        278.0,
        28.0,
        112.0,
    )
    .translate(
        MODULE_CENTER_X + MODULE_DOCK_X / 2.0 + 78.0,
        MODULE_CENTER_Y,
        DECK_Z / 2.0 + 116.0,
    );
    let mut aux = Part::empty("cip_sip_station_aux_sensor_pockets");
    for i in 0..AUX_SENSOR_POCKETS {
        let y = MODULE_CENTER_Y + port_x(i, AUX_SENSOR_POCKETS, 56.0);
        aux = aux
            + port_collar(
                &format!("cip_sip_station_aux_sensor_collar_{i}"),
                32.0,
                13.0,
            )
            .rotate(0.0, 0.0, 90.0)
            .translate(
                MODULE_CENTER_X + MODULE_DOCK_X / 2.0 + 102.0,
                y,
                DECK_Z / 2.0 + 116.0,
            );
    }

    rail - cuts + collars + aux_panel + aux
}

fn sterilant_service_clearances() -> Part {
    let rear_cart = clearance_frame(
        "cip_sip_station_rear_sterilant_generator_cart_clearance",
        DECK_X - 250.0,
        REAR_SERVICE_CLEARANCE,
        360.0,
    )
    .translate(0.0, DECK_Y / 2.0 + REAR_SERVICE_CLEARANCE / 2.0, 236.0);
    let side_service = clearance_frame(
        "cip_sip_station_left_sterilant_hose_bottle_swing_clearance",
        SIDE_STERILANT_CLEARANCE,
        620.0,
        320.0,
    )
    .translate(
        -DECK_X / 2.0 - SIDE_STERILANT_CLEARANCE / 2.0,
        MODULE_CENTER_Y,
        232.0,
    );
    let top_plenum = clearance_frame(
        "cip_sip_station_module_top_aeration_plenum_clearance",
        MODULE_DOCK_X + 110.0,
        MODULE_DOCK_Y + 90.0,
        220.0,
    )
    .translate(
        MODULE_CENTER_X,
        MODULE_CENTER_Y,
        DECK_Z / 2.0 + MODULE_Z + 172.0,
    );
    let hose_sweep = clearance_frame(
        "cip_sip_station_rear_hose_sweep_keepclear",
        760.0,
        170.0,
        230.0,
    )
    .translate(0.0, PANEL_BASE_Y - 136.0, DECK_Z / 2.0 + 214.0);

    rear_cart + side_service + top_plenum + hose_sweep
}

fn material_compatibility_lands() -> Part {
    let panel = centered_cube(
        "cip_sip_station_material_compatibility_label_panel",
        536.0,
        18.0,
        82.0,
    )
    .translate(
        -DECK_X / 2.0 + 118.0,
        MODULE_CENTER_Y + 80.0,
        DECK_Z / 2.0 + 96.0,
    )
    .rotate(0.0, 0.0, 90.0);
    let mut lands = Part::empty("cip_sip_station_material_compatibility_label_lands");
    for i in 0..MATERIAL_LABEL_LANDS {
        let row = i / 3;
        let col = i % 3;
        let x = -DECK_X / 2.0 + 70.0 + col as f64 * 45.0;
        let y = MODULE_CENTER_Y - 154.0 + row as f64 * 84.0;
        lands = lands
            + centered_cube(
                format!("cip_sip_station_material_label_land_{i}"),
                34.0,
                54.0,
                8.0,
            )
            .translate(x, y, DECK_Z / 2.0 + 142.0);
    }
    let compatibility_coupon_strip = centered_cube(
        "cip_sip_station_material_coupon_strip_datum",
        78.0,
        424.0,
        18.0,
    )
    .translate(
        -DECK_X / 2.0 + 158.0,
        MODULE_CENTER_Y + 10.0,
        DECK_Z / 2.0 + 42.0,
    );
    panel + lands + compatibility_coupon_strip
}

fn cycle_barcode_run_lands() -> Part {
    let header = centered_cube(
        "cip_sip_station_cycle_record_front_header",
        DECK_X - 240.0,
        18.0,
        74.0,
    )
    .translate(0.0, -DECK_Y / 2.0 - 28.0, DECK_Z / 2.0 + 82.0);
    let mut lands = Part::empty("cip_sip_station_cycle_barcode_lands");
    for i in 0..CYCLE_BARCODE_LANDS {
        let x = port_x(i, CYCLE_BARCODE_LANDS, 96.0);
        lands = lands
            + centered_cube(
                format!("cip_sip_station_cycle_barcode_land_{i}"),
                72.0,
                8.0,
                24.0,
            )
            .translate(x, -DECK_Y / 2.0 - 40.0, DECK_Z / 2.0 + 102.0)
            + centered_cube(
                format!("cip_sip_station_cycle_status_flag_slot_{i}"),
                46.0,
                8.0,
                12.0,
            )
            .translate(x, -DECK_Y / 2.0 - 40.0, DECK_Z / 2.0 + 58.0);
    }
    let mut run_tokens = Part::empty("cip_sip_station_run_token_slots");
    for i in 0..RUN_TOKEN_SLOTS {
        let x = DIRTY_PANEL_CENTER_X + port_x(i, RUN_TOKEN_SLOTS, 70.0);
        run_tokens = run_tokens
            + centered_cube(
                format!("cip_sip_station_run_token_slot_{i}"),
                52.0,
                42.0,
                12.0,
            )
            .translate(x, -DECK_Y / 2.0 + 88.0, DECK_Z / 2.0 + 18.0);
    }
    header + lands + run_tokens
}

fn clean_dirty_segregation() -> Part {
    let front_wall = centered_cube(
        "cip_sip_station_clean_dirty_front_segregation_wall",
        28.0,
        250.0,
        122.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 168.0, DECK_Z / 2.0 + 61.0);
    let rear_wall = centered_cube(
        "cip_sip_station_clean_dirty_rear_segregation_wall",
        28.0,
        286.0,
        206.0,
    )
    .translate(0.0, DECK_Y / 2.0 - 194.0, DECK_Z / 2.0 + 103.0);
    let bridge = centered_cube(
        "cip_sip_station_clean_dirty_over_module_bridge",
        32.0,
        MODULE_DOCK_Y - 78.0,
        34.0,
    )
    .translate(0.0, MODULE_CENTER_Y, DECK_Z / 2.0 + MODULE_Z + 82.0);
    let pass_window_cut = centered_cube(
        "cip_sip_station_clean_dirty_gasketed_harness_pass_cut",
        34.0,
        124.0,
        76.0,
    )
    .translate(0.0, PANEL_BASE_Y - 126.0, DECK_Z / 2.0 + 104.0);
    let drip_gutter = centered_cube(
        "cip_sip_station_clean_dirty_barrier_drip_gutter",
        78.0,
        MODULE_DOCK_Y + 170.0,
        12.0,
    )
    .translate(
        DIRTY_PANEL_CENTER_X - 242.0,
        MODULE_CENTER_Y,
        DECK_Z / 2.0 + 22.0,
    );
    let wall = front_wall + rear_wall + bridge - pass_window_cut;
    wall + segregation_labels() + drip_gutter
}

fn segregation_labels() -> Part {
    let clean = centered_cube("cip_sip_station_clean_side_label_land", 118.0, 8.0, 22.0).translate(
        -72.0,
        -DECK_Y / 2.0 + 42.0,
        DECK_Z / 2.0 + 88.0,
    );
    let dirty = centered_cube("cip_sip_station_dirty_side_label_land", 118.0, 8.0, 22.0).translate(
        72.0,
        -DECK_Y / 2.0 + 42.0,
        DECK_Z / 2.0 + 88.0,
    );
    clean + dirty
}

fn robot_service_keepouts() -> Part {
    let front_robot = clearance_frame(
        "cip_sip_station_front_robot_approach_keepout",
        DECK_X - 240.0,
        FRONT_ROBOT_CLEARANCE,
        ROBOT_Z_CLEARANCE,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 - FRONT_ROBOT_CLEARANCE / 2.0,
        DECK_Z / 2.0 + ROBOT_Z_CLEARANCE / 2.0,
    );
    let module_lift = clearance_frame(
        "cip_sip_station_module_vertical_lift_keepout",
        MODULE_DOCK_X + 180.0,
        MODULE_DOCK_Y + 160.0,
        MODULE_LIFT_CLEARANCE_Z,
    )
    .translate(
        MODULE_CENTER_X,
        MODULE_CENTER_Y,
        DECK_Z / 2.0 + MODULE_Z + MODULE_LIFT_CLEARANCE_Z / 2.0 + 36.0,
    );
    let rear_service = clearance_frame(
        "cip_sip_station_rear_bulkhead_service_keepout",
        DECK_X - 180.0,
        REAR_SERVICE_CLEARANCE,
        360.0,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 + REAR_SERVICE_CLEARANCE / 2.0 + 22.0,
        DECK_Z / 2.0 + 204.0,
    );
    let dirty_bottle_pull = clearance_frame(
        "cip_sip_station_dirty_waste_bottle_pull_keepout",
        520.0,
        360.0,
        260.0,
    )
    .translate(
        DIRTY_PANEL_CENTER_X,
        -DECK_Y / 2.0 - 176.0,
        DECK_Z / 2.0 + 166.0,
    );
    front_robot + module_lift + rear_service + dirty_bottle_pull + keepout_datum_targets()
}

fn keepout_datum_targets() -> Part {
    let mut targets = Part::empty("cip_sip_station_keepout_datum_targets");
    for (i, (x, y)) in [
        (
            MODULE_CENTER_X - MODULE_DOCK_X / 2.0,
            MODULE_CENTER_Y - MODULE_DOCK_Y / 2.0,
        ),
        (
            MODULE_CENTER_X + MODULE_DOCK_X / 2.0,
            MODULE_CENTER_Y - MODULE_DOCK_Y / 2.0,
        ),
        (
            MODULE_CENTER_X - MODULE_DOCK_X / 2.0,
            MODULE_CENTER_Y + MODULE_DOCK_Y / 2.0,
        ),
        (
            MODULE_CENTER_X + MODULE_DOCK_X / 2.0,
            MODULE_CENTER_Y + MODULE_DOCK_Y / 2.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        targets = targets
            + fiducial_target(&format!("cip_sip_station_keepout_corner_datum_{i}")).translate(
                *x,
                *y,
                DECK_Z / 2.0 + 154.0,
            );
    }
    targets
}

fn port_x(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn port_collar(name: &str, outside_d: f64, inside_d: f64) -> Part {
    let collar = centered_cylinder(format!("{name}_outer"), outside_d / 2.0, 12.0, 40)
        .rotate(90.0, 0.0, 0.0);
    let bore = centered_cylinder(format!("{name}_inner_bore"), inside_d / 2.0, 16.0, 32)
        .rotate(90.0, 0.0, 0.0);
    collar - bore
}

fn fiducial_target(name: &str) -> Part {
    let plate = centered_cube(format!("{name}_plate"), 34.0, 34.0, 4.0);
    let outer = centered_cylinder(format!("{name}_outer_ring"), 11.0, 5.0, 36);
    let inner = centered_cylinder(format!("{name}_center_mark"), 4.0, 6.0, 28);
    plate + outer - inner
}

fn clearance_frame(name: &str, x: f64, y: f64, z: f64) -> Part {
    let rail = 10.0;
    let mut frame = Part::empty(format!("{name}_frame"));

    for (i, sx) in [-1.0, 1.0].iter().enumerate() {
        for (j, sy) in [-1.0, 1.0].iter().enumerate() {
            frame = frame
                + centered_cube(format!("{name}_vertical_post_{i}_{j}"), rail, rail, z).translate(
                    sx * (x / 2.0 - rail / 2.0),
                    sy * (y / 2.0 - rail / 2.0),
                    0.0,
                );
        }
    }

    for (k, zsign) in [-1.0, 1.0].iter().enumerate() {
        let zpos = zsign * (z / 2.0 - rail / 2.0);
        frame = frame
            + centered_cube(format!("{name}_front_x_rail_{k}"), x, rail, rail).translate(
                0.0,
                -(y / 2.0 - rail / 2.0),
                zpos,
            )
            + centered_cube(format!("{name}_rear_x_rail_{k}"), x, rail, rail).translate(
                0.0,
                y / 2.0 - rail / 2.0,
                zpos,
            )
            + centered_cube(format!("{name}_left_y_rail_{k}"), rail, y, rail).translate(
                -(x / 2.0 - rail / 2.0),
                0.0,
                zpos,
            )
            + centered_cube(format!("{name}_right_y_rail_{k}"), rail, y, rail).translate(
                x / 2.0 - rail / 2.0,
                0.0,
                zpos,
            );
    }

    frame
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
            assert!(path.starts_with("output/closed_module_cip_sip_service_station_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn feature_list_covers_requested_station_interfaces() {
        assert_eq!(REQUIRED_FEATURES.len(), 11);
        assert!(REQUIRED_FEATURES.contains(&"sealed_module_dock"));
        assert!(REQUIRED_FEATURES.contains(&"closed_flush_inlet_bulkhead"));
        assert!(REQUIRED_FEATURES.contains(&"closed_return_drain_bulkhead"));
        assert!(REQUIRED_FEATURES.contains(&"drain_waste_containment"));
        assert!(REQUIRED_FEATURES.contains(&"validation_coupon_carrier"));
        assert!(REQUIRED_FEATURES.contains(&"temperature_pressure_sensor_pockets"));
        assert!(REQUIRED_FEATURES.contains(&"sterilant_service_clearances"));
        assert!(REQUIRED_FEATURES.contains(&"material_compatibility_lands"));
        assert!(REQUIRED_FEATURES.contains(&"cycle_barcode_run_lands"));
        assert!(REQUIRED_FEATURES.contains(&"clean_dirty_segregation"));
        assert!(REQUIRED_FEATURES.contains(&"robot_service_keepouts"));
    }

    #[test]
    fn sealed_module_dock_encloses_twenty_chip_array() {
        assert_eq!(COLS, 4);
        assert_eq!(ROWS, 5);
        assert_eq!(CHIP_COUNT, 20);
        assert!(MODULE_X > ARRAY_X + 180.0);
        assert!(MODULE_Y > ARRAY_Y + 145.0);
        assert!(MODULE_DOCK_X > MODULE_X + 100.0);
        assert!(MODULE_DOCK_Y > MODULE_Y + 100.0);
        assert!(MODULE_CENTER_X.abs() + MODULE_DOCK_X / 2.0 < DECK_X / 2.0 - 58.0);
        assert!(MODULE_CENTER_Y.abs() + MODULE_DOCK_Y / 2.0 < DECK_Y / 2.0 - 72.0);
    }

    #[test]
    fn bulkhead_ports_fit_their_panels() {
        let clean_edge = port_x(FLUSH_INLET_PORTS - 1, FLUSH_INLET_PORTS, PORT_PITCH).abs() + 26.0;
        let return_edge = port_x(RETURN_PORTS - 1, RETURN_PORTS, PORT_PITCH).abs() + 30.0;
        let drain_edge = port_x(DRAIN_PORTS - 1, DRAIN_PORTS, 88.0).abs() + 34.0;
        assert!(clean_edge < CLEAN_PANEL_X / 2.0);
        assert!(return_edge < DIRTY_PANEL_X / 2.0);
        assert!(drain_edge < DIRTY_PANEL_X / 2.0);
        assert!(CLEAN_PANEL_CENTER_X + CLEAN_PANEL_X / 2.0 < -18.0);
        assert!(DIRTY_PANEL_CENTER_X - DIRTY_PANEL_X / 2.0 > 18.0);
    }

    #[test]
    fn validation_and_instrumentation_counts_are_explicit() {
        assert_eq!(COUPON_SLOTS, 16);
        assert_eq!(WITNESS_DISC_POCKETS, 6);
        assert_eq!(TEMP_SENSOR_POCKETS, 8);
        assert_eq!(PRESSURE_SENSOR_POCKETS, 8);
        assert_eq!(AUX_SENSOR_POCKETS, 4);
        assert_eq!(MATERIAL_LABEL_LANDS, 12);
        assert_eq!(CYCLE_BARCODE_LANDS, 10);
        assert_eq!(RUN_TOKEN_SLOTS, 4);
    }

    #[test]
    fn waste_and_clean_dirty_layout_remain_segregated() {
        assert!(COUPON_CENTER_X + COUPON_CARRIER_X / 2.0 < -120.0);
        assert!(DIRTY_PANEL_CENTER_X - DIRTY_PANEL_X / 2.0 > 100.0);
        assert!(CLEAN_PANEL_CENTER_X + CLEAN_PANEL_X / 2.0 < -100.0);
        assert!(WASTE_BOTTLE_COUNT >= 3);
        assert!(WASTE_MANIFOLD_X < DIRTY_PANEL_X);
        assert!(WASTE_TRAY_X < DECK_X - 160.0);
        assert!(WASTE_TRAY_Y < DECK_Y - 40.0);
    }

    #[test]
    fn service_clearances_are_large_enough_for_robot_and_utilities() {
        assert!(FRONT_ROBOT_CLEARANCE >= 500.0);
        assert!(REAR_SERVICE_CLEARANCE >= 260.0);
        assert!(SIDE_STERILANT_CLEARANCE >= 220.0);
        assert!(ROBOT_Z_CLEARANCE >= 400.0);
        assert!(MODULE_LIFT_CLEARANCE_Z >= 240.0);
        assert!(PANEL_Z > 220.0);
    }

    #[test]
    fn station_dimensions_stay_in_modular_floor_cart_range() {
        assert!(DECK_X <= 1400.0);
        assert!(DECK_Y <= 950.0);
        assert!(MODULE_DOCK_X <= 900.0);
        assert!(MODULE_DOCK_Y <= 760.0);
        assert!(PANEL_BASE_Y < DECK_Y / 2.0);
    }
}
