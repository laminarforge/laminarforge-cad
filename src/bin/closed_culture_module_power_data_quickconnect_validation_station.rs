use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed culture module power/data quick-connect validation station.
//
// Intent:
// - Dock a sealed culture module against hard datums without opening its
//   sterile boundary.
// - Check keyed power and data quick-connect geometry before mating to the
//   workcell receiver.
// - Present pogo continuity, bend strain-relief, seal witness, traceability,
//   and pass/fail quarantine features in one controlled fixture.
//
// This CAD captures fixture datums, purchased instrument envelopes, and
// validation gauge features. Electrical safety, connector qualification, and
// software release logic remain separate verification gates.

const OUTPUTS: [&str; 11] = [
    "output/closed_culture_module_power_data_quickconnect_validation_station_deck.stl",
    "output/closed_culture_module_power_data_quickconnect_validation_station_dockable_module_nest.stl",
    "output/closed_culture_module_power_data_quickconnect_validation_station_keyed_power_connector_gauges.stl",
    "output/closed_culture_module_power_data_quickconnect_validation_station_keyed_data_connector_gauges.stl",
    "output/closed_culture_module_power_data_quickconnect_validation_station_continuity_pogo_fixture.stl",
    "output/closed_culture_module_power_data_quickconnect_validation_station_strain_relief_bend_gauge.stl",
    "output/closed_culture_module_power_data_quickconnect_validation_station_seal_witness_lands.stl",
    "output/closed_culture_module_power_data_quickconnect_validation_station_barcode_rfid_camera_bridge.stl",
    "output/closed_culture_module_power_data_quickconnect_validation_station_pass_fail_quarantine_lanes.stl",
    "output/closed_culture_module_power_data_quickconnect_validation_station_robot_service_keepouts.stl",
    "output/closed_culture_module_power_data_quickconnect_validation_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 8] = [
    "dockable_module_nest",
    "keyed_electrical_connector_gauge_blocks",
    "keyed_data_connector_gauge_blocks",
    "continuity_pogo_fixture",
    "strain_relief_bend_gauge",
    "seal_witness_lands",
    "barcode_rfid_camera_bridge",
    "pass_fail_quarantine_lanes",
];

const DECK_X: f64 = 1120.0;
const DECK_Y: f64 = 760.0;
const DECK_Z: f64 = 18.0;
const DECK_RIM_W: f64 = 14.0;
const DECK_RIM_Z: f64 = 20.0;
const MOUNT_HOLE_D: f64 = 6.6;

const MODULE_X: f64 = REVC_CHIP_LENGTH * 3.0 + 145.0;
const MODULE_Y: f64 = REVC_CHIP_WIDTH * 2.0 + 128.0;
const MODULE_Z_KEEP_OUT: f64 = 86.0;
const MODULE_CLEARANCE: f64 = 0.8;
const NEST_X: f64 = MODULE_X + 86.0;
const NEST_Y: f64 = MODULE_Y + 86.0;
const NEST_Z: f64 = 34.0;
const NEST_CENTER: (f64, f64) = (-225.0, 72.0);
const NEST_RAIL_W: f64 = 26.0;
const DOCKING_PIN_D: f64 = 8.0;
const DOCKING_PIN_BOSS_D: f64 = 22.0;

const POWER_CENTER: (f64, f64) = (330.0, 190.0);
const POWER_GAUGE_X: f64 = 330.0;
const POWER_GAUGE_Y: f64 = 140.0;
const POWER_GAUGE_Z: f64 = 48.0;
const POWER_CONNECTOR_COUNT: usize = 8;
const POWER_CONNECTOR_PITCH: f64 = 36.0;
const POWER_BORE_D: f64 = 9.2;
const POWER_KEY_W: f64 = 6.0;

const DATA_CENTER: (f64, f64) = (330.0, 38.0);
const DATA_GAUGE_X: f64 = 330.0;
const DATA_GAUGE_Y: f64 = 130.0;
const DATA_GAUGE_Z: f64 = 42.0;
const DATA_CONNECTOR_COUNT: usize = 6;
const DATA_CONNECTOR_PITCH: f64 = 44.0;
const DATA_SLOT_X: f64 = 18.0;
const DATA_SLOT_Y: f64 = 9.0;

const POGO_CENTER: (f64, f64) = (330.0, -126.0);
const POGO_X: f64 = 390.0;
const POGO_Y: f64 = 150.0;
const POGO_Z: f64 = 56.0;
const POGO_ROWS: usize = 2;
const POGO_COLS: usize = 16;
const POGO_PIN_COUNT: usize = POGO_ROWS * POGO_COLS;
const POGO_PITCH_X: f64 = 18.0;
const POGO_PITCH_Y: f64 = 38.0;
const POGO_PIN_D: f64 = 2.2;
const POGO_COLLAR_D: f64 = 5.4;
const POGO_OVERTRAVEL_Z: f64 = 6.0;

const BEND_CENTER: (f64, f64) = (-295.0, -260.0);
const BEND_X: f64 = 410.0;
const BEND_Y: f64 = 154.0;
const BEND_Z: f64 = 32.0;
const BEND_LANES: usize = 4;
const BEND_LANE_PITCH_Y: f64 = 28.0;
const BEND_CABLE_D: f64 = 7.5;
const BEND_MANDREL_COUNT: usize = 3;

const SEAL_CENTER: (f64, f64) = (65.0, -286.0);
const SEAL_X: f64 = 270.0;
const SEAL_Y: f64 = 120.0;
const SEAL_Z: f64 = 14.0;
const SEAL_WITNESS_LANDS: usize = 12;
const SEAL_LAND_COLS: usize = 6;
const SEAL_LAND_X: f64 = 30.0;
const SEAL_LAND_Y: f64 = 18.0;
const SEAL_LAND_PITCH_X: f64 = 38.0;
const SEAL_LAND_PITCH_Y: f64 = 42.0;

const BRIDGE_SPAN_X: f64 = NEST_X + 42.0;
const BRIDGE_POST_X: f64 = 30.0;
const BRIDGE_POST_Y: f64 = 58.0;
const BRIDGE_BEAM_Z: f64 = 24.0;
const BRIDGE_UNDERSIDE_Z: f64 = 142.0;
const BRIDGE_POST_Z: f64 = BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z;
const CAMERA_COUNT: usize = 3;
const CAMERA_PITCH_X: f64 = 160.0;
const RFID_ANTENNA_X: f64 = 156.0;
const RFID_ANTENNA_Y: f64 = 72.0;

const PASS_FAIL_CENTER: (f64, f64) = (390.0, -285.0);
const PASS_FAIL_X: f64 = 300.0;
const PASS_FAIL_Y: f64 = 150.0;
const PASS_FAIL_Z: f64 = 42.0;
const PASS_LANES: usize = 3;
const FAIL_QUARANTINE_LANES: usize = 3;
const LANE_SLOT_X: f64 = 104.0;
const LANE_SLOT_Y: f64 = 30.0;
const LANE_PITCH_Y: f64 = 38.0;
const QUARANTINE_WALL: f64 = 8.0;
const PASS_FAIL_SEGREGATION_MIN: f64 = 44.0;

const KEEP_OUT_Z: f64 = 10.0;
const FRONT_ROBOT_KEEP_OUT_Y: f64 = 88.0;
const REAR_CABLE_KEEP_OUT_Y: f64 = 72.0;
const LEFT_MODULE_LOAD_KEEP_OUT_X: f64 = 82.0;
const RIGHT_SERVICE_KEEP_OUT_X: f64 = 98.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let deck = station_deck();
    export(OUTPUTS[0], &deck);

    let nest = dockable_module_nest();
    export(OUTPUTS[1], &nest);

    let power_gauges = keyed_power_connector_gauge_blocks();
    export(OUTPUTS[2], &power_gauges);

    let data_gauges = keyed_data_connector_gauge_blocks();
    export(OUTPUTS[3], &data_gauges);

    let pogo = continuity_pogo_fixture();
    export(OUTPUTS[4], &pogo);

    let bend = strain_relief_bend_gauge();
    export(OUTPUTS[5], &bend);

    let seals = seal_witness_lands();
    export(OUTPUTS[6], &seals);

    let bridge = barcode_rfid_camera_bridge();
    export(OUTPUTS[7], &bridge);

    let lanes = pass_fail_quarantine_lanes();
    export(OUTPUTS[8], &lanes);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[9], &keepouts);

    let assembly = deck
        + nest.translate(NEST_CENTER.0, NEST_CENTER.1, deck_insert_z(NEST_Z))
        + power_gauges.translate(POWER_CENTER.0, POWER_CENTER.1, deck_insert_z(POWER_GAUGE_Z))
        + data_gauges.translate(DATA_CENTER.0, DATA_CENTER.1, deck_insert_z(DATA_GAUGE_Z))
        + pogo.translate(POGO_CENTER.0, POGO_CENTER.1, deck_insert_z(POGO_Z))
        + bend.translate(BEND_CENTER.0, BEND_CENTER.1, deck_insert_z(BEND_Z))
        + seals.translate(SEAL_CENTER.0, SEAL_CENTER.1, deck_insert_z(SEAL_Z))
        + bridge.translate(NEST_CENTER.0, NEST_CENTER.1, DECK_Z / 2.0)
        + lanes.translate(
            PASS_FAIL_CENTER.0,
            PASS_FAIL_CENTER.1,
            deck_insert_z(PASS_FAIL_Z),
        )
        + keepouts.translate(0.0, 0.0, DECK_Z / 2.0 + KEEP_OUT_Z / 2.0);

    export(OUTPUTS[10], &assembly);

    println!();
    println!("Closed culture module power/data quick-connect validation station:");
    println!("  Deck:                         {DECK_X:.0}mm x {DECK_Y:.0}mm x {DECK_Z:.0}mm");
    println!(
        "  Dockable module nest:         {NEST_X:.0}mm x {NEST_Y:.0}mm, {MODULE_CLEARANCE:.1}mm module clearance, {MODULE_Z_KEEP_OUT:.0}mm keepout height"
    );
    println!(
        "  Keyed connector gauges:       {POWER_CONNECTOR_COUNT} power positions and {DATA_CONNECTOR_COUNT} data positions"
    );
    println!(
        "  Continuity pogo fixture:      {POGO_PIN_COUNT} pogo pins in {POGO_ROWS}x{POGO_COLS} grid with {POGO_OVERTRAVEL_Z:.0}mm overtravel gauge"
    );
    println!(
        "  Strain relief bend gauge:     {BEND_LANES} cable lanes, {BEND_MANDREL_COUNT} mandrel radii, {BEND_CABLE_D:.1}mm cable channels"
    );
    println!(
        "  Seal witness lands:           {SEAL_WITNESS_LANDS} lands plus barcode/RFID/camera bridge and pass/fail quarantine lanes"
    );
    println!(
        "  Control feature groups:       {} required fixture groups",
        REQUIRED_FEATURES.len()
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn deck_insert_z(component_z: f64) -> f64 {
    DECK_Z / 2.0 + component_z / 2.0
}

fn station_deck() -> Part {
    let deck = centered_cube(
        "power_data_quickconnect_validation_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    );

    let mut recesses = Part::empty("power_data_quickconnect_validation_deck_recesses");
    for (i, rect) in [
        Rect::new(NEST_CENTER.0, NEST_CENTER.1, NEST_X + 34.0, NEST_Y + 30.0),
        Rect::new(
            POWER_CENTER.0,
            POWER_CENTER.1,
            POWER_GAUGE_X + 24.0,
            POWER_GAUGE_Y + 24.0,
        ),
        Rect::new(
            DATA_CENTER.0,
            DATA_CENTER.1,
            DATA_GAUGE_X + 24.0,
            DATA_GAUGE_Y + 20.0,
        ),
        Rect::new(POGO_CENTER.0, POGO_CENTER.1, POGO_X + 24.0, POGO_Y + 22.0),
        Rect::new(BEND_CENTER.0, BEND_CENTER.1, BEND_X + 22.0, BEND_Y + 20.0),
        Rect::new(SEAL_CENTER.0, SEAL_CENTER.1, SEAL_X + 20.0, SEAL_Y + 18.0),
        Rect::new(
            PASS_FAIL_CENTER.0,
            PASS_FAIL_CENTER.1,
            PASS_FAIL_X + 20.0,
            PASS_FAIL_Y + 18.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        recesses = recesses
            + centered_cube(
                format!("power_data_quickconnect_validation_deck_recess_{i}"),
                rect.w,
                rect.h,
                6.0,
            )
            .translate(rect.x, rect.y, DECK_Z / 2.0 - 2.0);
    }

    let wipe_gutter = centered_cube(
        "power_data_quickconnect_validation_deck_wipe_gutter",
        DECK_X - 96.0,
        10.0,
        5.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 118.0, DECK_Z / 2.0 - 1.8);

    let mut mount_holes = Part::empty("power_data_quickconnect_validation_deck_mount_holes");
    for (i, (x, y)) in deck_mount_points().iter().enumerate() {
        mount_holes = mount_holes
            + centered_cylinder(
                format!("power_data_quickconnect_validation_m6_mount_{i}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 3.0,
                28,
            )
            .translate(*x, *y, 0.0);
    }

    deck - recesses - wipe_gutter - mount_holes + deck_perimeter_rims()
}

fn deck_perimeter_rims() -> Part {
    let rear = centered_cube(
        "power_data_quickconnect_validation_rear_cable_rim",
        DECK_X - 100.0,
        DECK_RIM_W,
        DECK_RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - 32.0, DECK_Z / 2.0 + DECK_RIM_Z / 2.0);
    let left = centered_cube(
        "power_data_quickconnect_validation_left_loading_rim",
        DECK_RIM_W,
        DECK_Y - 120.0,
        DECK_RIM_Z,
    )
    .translate(-DECK_X / 2.0 + 34.0, 0.0, DECK_Z / 2.0 + DECK_RIM_Z / 2.0);
    let front_low = centered_cube(
        "power_data_quickconnect_validation_front_low_wipe_lip",
        DECK_X - 260.0,
        10.0,
        12.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 32.0, DECK_Z / 2.0 + 6.0);

    rear + left + front_low
}

fn dockable_module_nest() -> Part {
    let tray = centered_cube(
        "power_data_quickconnect_dockable_module_nest_tray",
        NEST_X,
        NEST_Y,
        NEST_Z,
    );
    let module_relief = centered_cube(
        "power_data_quickconnect_dockable_module_body_relief",
        MODULE_X + MODULE_CLEARANCE * 2.0,
        MODULE_Y + MODULE_CLEARANCE * 2.0,
        NEST_Z + 2.0,
    )
    .translate(0.0, 0.0, 8.0);

    let left_rail = centered_cube(
        "power_data_quickconnect_module_left_datum_rail",
        NEST_RAIL_W,
        MODULE_Y + 56.0,
        NEST_Z + 24.0,
    )
    .translate(-(MODULE_X / 2.0 + NEST_RAIL_W / 2.0 + 16.0), 0.0, 12.0);
    let right_rail = centered_cube(
        "power_data_quickconnect_module_right_datum_rail",
        NEST_RAIL_W,
        MODULE_Y + 56.0,
        NEST_Z + 24.0,
    )
    .translate(MODULE_X / 2.0 + NEST_RAIL_W / 2.0 + 16.0, 0.0, 12.0);
    let rear_stop = centered_cube(
        "power_data_quickconnect_module_rear_hard_stop",
        MODULE_X + 96.0,
        22.0,
        NEST_Z + 28.0,
    )
    .translate(0.0, MODULE_Y / 2.0 + 34.0, 14.0);
    let front_lead_in = centered_cube(
        "power_data_quickconnect_module_front_lead_in_lip",
        MODULE_X + 82.0,
        14.0,
        16.0,
    )
    .translate(0.0, -(MODULE_Y / 2.0 + 31.0), -5.0);

    let fork_slots = centered_cube(
        "power_data_quickconnect_module_left_robot_fork_slot",
        128.0,
        26.0,
        NEST_Z + 4.0,
    )
    .translate(-160.0, -NEST_Y / 2.0 + 24.0, -2.0)
        + centered_cube(
            "power_data_quickconnect_module_right_robot_fork_slot",
            128.0,
            26.0,
            NEST_Z + 4.0,
        )
        .translate(160.0, -NEST_Y / 2.0 + 24.0, -2.0);

    tray - module_relief - fork_slots
        + left_rail
        + right_rail
        + rear_stop
        + front_lead_in
        + module_docking_pin_bosses()
        + module_present_sensor_flags()
}

fn module_docking_pin_bosses() -> Part {
    let mut bosses = Part::empty("power_data_quickconnect_module_docking_pin_bosses");
    for (i, (x, y)) in [
        (-(MODULE_X / 2.0 - 48.0), -(MODULE_Y / 2.0 - 42.0)),
        (MODULE_X / 2.0 - 48.0, -(MODULE_Y / 2.0 - 42.0)),
        (-(MODULE_X / 2.0 - 48.0), MODULE_Y / 2.0 - 42.0),
        (MODULE_X / 2.0 - 48.0, MODULE_Y / 2.0 - 42.0),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("power_data_quickconnect_docking_pin_boss_{i}"),
            DOCKING_PIN_BOSS_D / 2.0,
            12.0,
            32,
        )
        .translate(*x, *y, NEST_Z / 2.0 + 6.0);
        let pin_gauge = centered_cylinder(
            format!("power_data_quickconnect_docking_pin_clearance_gauge_{i}"),
            DOCKING_PIN_D / 2.0,
            15.0,
            28,
        )
        .translate(*x, *y, NEST_Z / 2.0 + 6.5);
        bosses = bosses + (boss - pin_gauge);
    }
    bosses
}

fn module_present_sensor_flags() -> Part {
    let rear_flag = centered_cube(
        "power_data_quickconnect_module_rear_present_sensor_flag",
        44.0,
        8.0,
        18.0,
    )
    .translate(
        -(MODULE_X / 2.0 - 92.0),
        MODULE_Y / 2.0 + 21.0,
        NEST_Z / 2.0 + 9.0,
    );
    let side_flag = centered_cube(
        "power_data_quickconnect_module_side_present_sensor_flag",
        8.0,
        44.0,
        18.0,
    )
    .translate(
        MODULE_X / 2.0 + 19.0,
        -(MODULE_Y / 2.0 - 92.0),
        NEST_Z / 2.0 + 9.0,
    );

    rear_flag + side_flag
}

fn keyed_power_connector_gauge_blocks() -> Part {
    let body = centered_cube(
        "power_data_quickconnect_keyed_power_gauge_body",
        POWER_GAUGE_X,
        POWER_GAUGE_Y,
        POWER_GAUGE_Z,
    );
    let mut connector_cuts = Part::empty("power_data_quickconnect_keyed_power_connector_cuts");
    let mut polarity_ribs = Part::empty("power_data_quickconnect_power_polarity_ribs");

    for i in 0..POWER_CONNECTOR_COUNT {
        let x = connector_x(i, POWER_CONNECTOR_COUNT, POWER_CONNECTOR_PITCH);
        connector_cuts = connector_cuts
            + centered_cylinder(
                format!("power_data_quickconnect_power_pin_bore_{i}"),
                POWER_BORE_D / 2.0,
                POWER_GAUGE_Z + 4.0,
                30,
            )
            .translate(x, -18.0, 0.0)
            + centered_cube(
                format!("power_data_quickconnect_power_keyway_{i}"),
                POWER_KEY_W,
                19.0,
                POWER_GAUGE_Z + 4.0,
            )
            .translate(x + 8.5, -18.0, 0.0)
            + centered_cube(
                format!("power_data_quickconnect_power_shell_slot_{i}"),
                18.0,
                8.0,
                16.0,
            )
            .translate(x, 25.0, POWER_GAUGE_Z / 2.0 - 8.0);

        let rib_y = if i % 2 == 0 { -50.0 } else { 50.0 };
        polarity_ribs = polarity_ribs
            + centered_cube(
                format!("power_data_quickconnect_power_polarity_rib_{i}"),
                16.0,
                6.0,
                6.0,
            )
            .translate(x, rib_y, POWER_GAUGE_Z / 2.0 + 3.0);
    }

    let high_current_bus = centered_cube(
        "power_data_quickconnect_power_high_current_bus_witness",
        POWER_GAUGE_X - 44.0,
        10.0,
        5.0,
    )
    .translate(0.0, -POWER_GAUGE_Y / 2.0 + 16.0, POWER_GAUGE_Z / 2.0 + 2.5);
    let low_voltage_bus = centered_cube(
        "power_data_quickconnect_power_low_voltage_bus_witness",
        POWER_GAUGE_X - 82.0,
        8.0,
        4.0,
    )
    .translate(0.0, POWER_GAUGE_Y / 2.0 - 18.0, POWER_GAUGE_Z / 2.0 + 2.0);

    body - connector_cuts + polarity_ribs + high_current_bus + low_voltage_bus
}

fn keyed_data_connector_gauge_blocks() -> Part {
    let body = centered_cube(
        "power_data_quickconnect_keyed_data_gauge_body",
        DATA_GAUGE_X,
        DATA_GAUGE_Y,
        DATA_GAUGE_Z,
    );
    let mut connector_cuts = Part::empty("power_data_quickconnect_keyed_data_connector_cuts");
    let mut shield_lands = Part::empty("power_data_quickconnect_data_shield_lands");

    for i in 0..DATA_CONNECTOR_COUNT {
        let x = connector_x(i, DATA_CONNECTOR_COUNT, DATA_CONNECTOR_PITCH);
        connector_cuts = connector_cuts
            + centered_cube(
                format!("power_data_quickconnect_data_blade_slot_{i}"),
                DATA_SLOT_X,
                DATA_SLOT_Y,
                DATA_GAUGE_Z + 4.0,
            )
            .translate(x, -18.0, 0.0)
            + centered_cube(
                format!("power_data_quickconnect_data_asymmetric_key_slot_{i}"),
                6.0,
                15.0,
                DATA_GAUGE_Z + 4.0,
            )
            .translate(x - 11.0, -18.0, 0.0)
            + centered_cylinder(
                format!("power_data_quickconnect_data_round_m12_gauge_{i}"),
                11.0 / 2.0,
                DATA_GAUGE_Z + 4.0,
                28,
            )
            .translate(x, 28.0, 0.0);

        shield_lands = shield_lands
            + centered_cube(
                format!("power_data_quickconnect_data_shield_continuity_land_{i}"),
                28.0,
                8.0,
                3.0,
            )
            .translate(x, DATA_GAUGE_Y / 2.0 - 15.0, DATA_GAUGE_Z / 2.0 + 1.5);
    }

    let orientation_bar = centered_cube(
        "power_data_quickconnect_data_orientation_gauge_bar",
        DATA_GAUGE_X - 48.0,
        9.0,
        6.0,
    )
    .translate(0.0, -DATA_GAUGE_Y / 2.0 + 18.0, DATA_GAUGE_Z / 2.0 + 3.0);

    body - connector_cuts + shield_lands + orientation_bar
}

fn continuity_pogo_fixture() -> Part {
    let plate = centered_cube(
        "power_data_quickconnect_continuity_pogo_fixture_plate",
        POGO_X,
        POGO_Y,
        POGO_Z,
    );
    let mut pin_holes = Part::empty("power_data_quickconnect_continuity_pogo_pin_holes");
    let mut collars = Part::empty("power_data_quickconnect_continuity_pogo_pin_collars");

    for row in 0..POGO_ROWS {
        for col in 0..POGO_COLS {
            let pin = row * POGO_COLS + col;
            let (x, y) = pogo_center(row, col);
            pin_holes = pin_holes
                + centered_cylinder(
                    format!("power_data_quickconnect_pogo_pin_clearance_{pin}"),
                    POGO_PIN_D / 2.0,
                    POGO_Z + 5.0,
                    18,
                )
                .translate(x, y, 0.0);
            collars = collars
                + centered_cylinder(
                    format!("power_data_quickconnect_pogo_pin_collar_{pin}"),
                    POGO_COLLAR_D / 2.0,
                    4.0,
                    20,
                )
                .translate(x, y, POGO_Z / 2.0 + 2.0);
        }
    }

    let overtravel_bar = centered_cube(
        "power_data_quickconnect_pogo_overtravel_stop_bar",
        POGO_X - 48.0,
        16.0,
        POGO_OVERTRAVEL_Z,
    )
    .translate(
        0.0,
        POGO_Y / 2.0 - 24.0,
        POGO_Z / 2.0 + POGO_OVERTRAVEL_Z / 2.0,
    );
    let harness_exit = centered_cube(
        "power_data_quickconnect_pogo_rear_harness_exit",
        POGO_X - 70.0,
        20.0,
        16.0,
    )
    .translate(0.0, POGO_Y / 2.0 - 8.0, -POGO_Z / 2.0 + 8.0);
    let continuity_reference_land = centered_cube(
        "power_data_quickconnect_continuity_reference_coupon_land",
        86.0,
        28.0,
        4.0,
    )
    .translate(
        -(POGO_X / 2.0 - 64.0),
        -(POGO_Y / 2.0 - 26.0),
        POGO_Z / 2.0 + 2.0,
    );

    plate - pin_holes - harness_exit + collars + overtravel_bar + continuity_reference_land
}

fn strain_relief_bend_gauge() -> Part {
    let plate = centered_cube(
        "power_data_quickconnect_strain_relief_bend_gauge_plate",
        BEND_X,
        BEND_Y,
        BEND_Z,
    );

    let mut cable_channels = Part::empty("power_data_quickconnect_strain_relief_cable_channels");
    for lane in 0..BEND_LANES {
        let y = lane_y(lane, BEND_LANES, BEND_LANE_PITCH_Y);
        cable_channels = cable_channels
            + centered_cylinder(
                format!("power_data_quickconnect_bend_lane_channel_{lane}"),
                BEND_CABLE_D / 2.0,
                BEND_X + 4.0,
                20,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, y, 4.0);
    }

    let mut mandrels = Part::empty("power_data_quickconnect_bend_radius_mandrels");
    for (i, (x, radius)) in [(-118.0, 25.0), (0.0, 40.0), (140.0, 60.0)]
        .iter()
        .enumerate()
    {
        let mandrel = centered_cylinder(
            format!("power_data_quickconnect_bend_radius_r{radius:.0}_mandrel_{i}"),
            *radius,
            16.0,
            48,
        )
        .translate(*x, 0.0, BEND_Z / 2.0 + 8.0);
        let center_bore = centered_cylinder(
            format!("power_data_quickconnect_bend_radius_r{radius:.0}_finger_relief_{i}"),
            radius - 12.0,
            18.0,
            48,
        )
        .translate(*x, 0.0, BEND_Z / 2.0 + 8.0);
        mandrels = mandrels + (mandrel - center_bore);
    }

    let clamp_comb = centered_cube(
        "power_data_quickconnect_strain_relief_clamp_comb",
        BEND_X - 44.0,
        20.0,
        16.0,
    )
    .translate(0.0, -BEND_Y / 2.0 + 24.0, BEND_Z / 2.0 + 8.0);
    let no_go_stop = centered_cube(
        "power_data_quickconnect_strain_relief_no_go_stop",
        22.0,
        BEND_Y - 36.0,
        18.0,
    )
    .translate(BEND_X / 2.0 - 36.0, 0.0, BEND_Z / 2.0 + 9.0);

    plate - cable_channels + mandrels + clamp_comb + no_go_stop
}

fn seal_witness_lands() -> Part {
    let plate = centered_cube(
        "power_data_quickconnect_seal_witness_land_plate",
        SEAL_X,
        SEAL_Y,
        SEAL_Z,
    );
    let mut lands = Part::empty("power_data_quickconnect_seal_witness_lands");

    for i in 0..SEAL_WITNESS_LANDS {
        let col = i % SEAL_LAND_COLS;
        let row = i / SEAL_LAND_COLS;
        let x = connector_x(col, SEAL_LAND_COLS, SEAL_LAND_PITCH_X);
        let y = lane_y(row, 2, SEAL_LAND_PITCH_Y);
        lands = lands
            + centered_cube(
                format!("power_data_quickconnect_seal_witness_land_{i}"),
                SEAL_LAND_X,
                SEAL_LAND_Y,
                3.0,
            )
            .translate(x, y, SEAL_Z / 2.0 + 1.5);
    }

    let witness_frame = centered_cube(
        "power_data_quickconnect_seal_witness_frame_front",
        SEAL_X - 28.0,
        5.0,
        8.0,
    )
    .translate(0.0, -SEAL_Y / 2.0 + 12.0, SEAL_Z / 2.0 + 4.0)
        + centered_cube(
            "power_data_quickconnect_seal_witness_frame_rear",
            SEAL_X - 28.0,
            5.0,
            8.0,
        )
        .translate(0.0, SEAL_Y / 2.0 - 12.0, SEAL_Z / 2.0 + 4.0);
    let coupon_pocket = centered_cube(
        "power_data_quickconnect_seal_witness_reference_coupon_pocket",
        58.0,
        28.0,
        6.0,
    )
    .translate(SEAL_X / 2.0 - 45.0, 0.0, SEAL_Z / 2.0 - 2.0);

    plate - coupon_pocket + lands + witness_frame
}

fn barcode_rfid_camera_bridge() -> Part {
    let left_post = bridge_post("left").translate(
        -(BRIDGE_SPAN_X / 2.0 - BRIDGE_POST_X / 2.0),
        0.0,
        BRIDGE_POST_Z / 2.0,
    );
    let right_post = bridge_post("right").translate(
        BRIDGE_SPAN_X / 2.0 - BRIDGE_POST_X / 2.0,
        0.0,
        BRIDGE_POST_Z / 2.0,
    );
    let beam = centered_cube(
        "power_data_quickconnect_barcode_rfid_camera_bridge_beam",
        BRIDGE_SPAN_X,
        BRIDGE_POST_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, 0.0, BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0);

    let mut cameras = Part::empty("power_data_quickconnect_camera_sleds");
    for i in 0..CAMERA_COUNT {
        let x = connector_x(i, CAMERA_COUNT, CAMERA_PITCH_X);
        cameras = cameras
            + centered_cube(
                format!("power_data_quickconnect_camera_sled_{i}"),
                72.0,
                50.0,
                28.0,
            )
            .translate(x, 0.0, BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z + 14.0)
            + centered_cylinder(
                format!("power_data_quickconnect_camera_lens_clearance_{i}"),
                14.0 / 2.0,
                22.0,
                28,
            )
            .translate(x, 0.0, BRIDGE_UNDERSIDE_Z - 11.0);
    }

    let rfid_antenna = centered_cube(
        "power_data_quickconnect_rfid_antenna_bridge_plate",
        RFID_ANTENNA_X,
        RFID_ANTENNA_Y,
        6.0,
    )
    .translate(
        -(BRIDGE_SPAN_X / 2.0 - RFID_ANTENNA_X / 2.0 - 44.0),
        BRIDGE_POST_Y / 2.0 + 26.0,
        BRIDGE_UNDERSIDE_Z + 12.0,
    );
    let barcode_light_bar = centered_cube(
        "power_data_quickconnect_barcode_illumination_bar",
        MODULE_X - 42.0,
        12.0,
        14.0,
    )
    .translate(0.0, -(MODULE_Y / 2.0 - 28.0), BRIDGE_UNDERSIDE_Z - 18.0);
    let rear_light_bar = centered_cube(
        "power_data_quickconnect_rear_connector_illumination_bar",
        MODULE_X - 42.0,
        12.0,
        14.0,
    )
    .translate(0.0, MODULE_Y / 2.0 - 28.0, BRIDGE_UNDERSIDE_Z - 18.0);
    let cable_chain_keepout = centered_cube(
        "power_data_quickconnect_bridge_cable_chain_keepout",
        BRIDGE_SPAN_X - 82.0,
        14.0,
        16.0,
    )
    .translate(0.0, BRIDGE_POST_Y / 2.0 + 16.0, BRIDGE_UNDERSIDE_Z + 4.0);

    left_post
        + right_post
        + beam
        + cameras
        + rfid_antenna
        + barcode_light_bar
        + rear_light_bar
        + cable_chain_keepout
}

fn bridge_post(name: &str) -> Part {
    let post = centered_cube(
        format!("power_data_quickconnect_bridge_{name}_post"),
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_POST_Z,
    );
    let lightening_slot = centered_cube(
        format!("power_data_quickconnect_bridge_{name}_post_lightening_slot"),
        BRIDGE_POST_X + 2.0,
        BRIDGE_POST_Y - 20.0,
        BRIDGE_POST_Z - 44.0,
    )
    .translate(0.0, 0.0, 10.0);
    let mount_holes = centered_cylinder(
        format!("power_data_quickconnect_bridge_{name}_front_m5"),
        5.3 / 2.0,
        BRIDGE_POST_Y + 2.0,
        20,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, -14.0, -BRIDGE_POST_Z / 2.0 + 18.0)
        + centered_cylinder(
            format!("power_data_quickconnect_bridge_{name}_rear_m5"),
            5.3 / 2.0,
            BRIDGE_POST_Y + 2.0,
            20,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, 14.0, -BRIDGE_POST_Z / 2.0 + 18.0);

    post - lightening_slot - mount_holes
}

fn pass_fail_quarantine_lanes() -> Part {
    let tray = centered_cube(
        "power_data_quickconnect_pass_fail_quarantine_tray",
        PASS_FAIL_X,
        PASS_FAIL_Y,
        PASS_FAIL_Z,
    );
    let mut lane_pockets = Part::empty("power_data_quickconnect_pass_fail_lane_pockets");

    for i in 0..PASS_LANES {
        let y = lane_y(i, PASS_LANES, LANE_PITCH_Y);
        lane_pockets = lane_pockets
            + centered_cube(
                format!("power_data_quickconnect_pass_lane_slot_{i}"),
                LANE_SLOT_X,
                LANE_SLOT_Y,
                PASS_FAIL_Z - 6.0,
            )
            .translate(-74.0, y, 5.0);
    }

    for i in 0..FAIL_QUARANTINE_LANES {
        let y = lane_y(i, FAIL_QUARANTINE_LANES, LANE_PITCH_Y);
        lane_pockets = lane_pockets
            + centered_cube(
                format!("power_data_quickconnect_fail_quarantine_lane_slot_{i}"),
                LANE_SLOT_X,
                LANE_SLOT_Y,
                PASS_FAIL_Z - 6.0,
            )
            .translate(74.0, y, 5.0);
    }

    let divider = centered_cube(
        "power_data_quickconnect_pass_fail_physical_divider",
        QUARANTINE_WALL,
        PASS_FAIL_Y - 18.0,
        PASS_FAIL_Z + 18.0,
    )
    .translate(0.0, 0.0, 9.0);
    let quarantine_lid_ledge = centered_cube(
        "power_data_quickconnect_quarantine_lane_lid_ledge",
        PASS_FAIL_X / 2.0 - 26.0,
        8.0,
        6.0,
    )
    .translate(74.0, PASS_FAIL_Y / 2.0 - 17.0, PASS_FAIL_Z / 2.0 + 3.0);
    let reject_reason_scan_tab = centered_cube(
        "power_data_quickconnect_reject_reason_scan_tab",
        PASS_FAIL_X - 44.0,
        18.0,
        4.0,
    )
    .translate(0.0, -PASS_FAIL_Y / 2.0 + 16.0, PASS_FAIL_Z / 2.0 + 2.0);

    tray - lane_pockets + divider + quarantine_lid_ledge + reject_reason_scan_tab
}

fn robot_service_keepouts() -> Part {
    let front_robot = centered_cube(
        "power_data_quickconnect_front_robot_handoff_keepout",
        DECK_X - 190.0,
        10.0,
        KEEP_OUT_Z,
    )
    .translate(20.0, -DECK_Y / 2.0 + FRONT_ROBOT_KEEP_OUT_Y, 0.0);
    let rear_cable = centered_cube(
        "power_data_quickconnect_rear_cable_bundle_keepout",
        DECK_X - 130.0,
        10.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - REAR_CABLE_KEEP_OUT_Y, 0.0);
    let left_module_load = centered_cube(
        "power_data_quickconnect_left_module_loading_keepout",
        10.0,
        DECK_Y - 190.0,
        KEEP_OUT_Z,
    )
    .translate(-DECK_X / 2.0 + LEFT_MODULE_LOAD_KEEP_OUT_X, 0.0, 0.0);
    let right_service = centered_cube(
        "power_data_quickconnect_right_service_tool_keepout",
        10.0,
        DECK_Y - 210.0,
        KEEP_OUT_Z,
    )
    .translate(DECK_X / 2.0 - RIGHT_SERVICE_KEEP_OUT_X, -10.0, 0.0);

    front_robot + rear_cable + left_module_load + right_service
}

fn connector_x(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn lane_y(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn pogo_center(row: usize, col: usize) -> (f64, f64) {
    (
        connector_x(col, POGO_COLS, POGO_PITCH_X),
        lane_y(row, POGO_ROWS, POGO_PITCH_Y),
    )
}

fn deck_mount_points() -> [(f64, f64); 8] {
    [
        (-(DECK_X / 2.0 - 34.0), -(DECK_Y / 2.0 - 34.0)),
        (DECK_X / 2.0 - 34.0, -(DECK_Y / 2.0 - 34.0)),
        (-(DECK_X / 2.0 - 34.0), DECK_Y / 2.0 - 34.0),
        (DECK_X / 2.0 - 34.0, DECK_Y / 2.0 - 34.0),
        (0.0, -(DECK_Y / 2.0 - 34.0)),
        (0.0, DECK_Y / 2.0 - 34.0),
        (-(DECK_X / 2.0 - 34.0), 0.0),
        (DECK_X / 2.0 - 34.0, 0.0),
    ]
}

fn bridge_clearance_above_module_keepout() -> f64 {
    BRIDGE_UNDERSIDE_Z - MODULE_Z_KEEP_OUT
}

fn pass_fail_lane_gap() -> f64 {
    (74.0 - LANE_SLOT_X / 2.0) - (-74.0 + LANE_SLOT_X / 2.0)
}

fn assert_layout() {
    let rects = [
        deck_rect(),
        nest_rect(),
        power_rect(),
        data_rect(),
        pogo_rect(),
        bend_rect(),
        seal_rect(),
        pass_fail_rect(),
    ];

    for rect in rects.iter().skip(1) {
        assert!(rect_inside(*rect, deck_rect(), 10.0));
    }
    assert!(!rects_overlap(nest_rect(), power_rect()));
    assert!(!rects_overlap(nest_rect(), data_rect()));
    assert!(!rects_overlap(nest_rect(), pogo_rect()));
    assert!(!rects_overlap(bend_rect(), seal_rect()));
    assert!(!rects_overlap(seal_rect(), pass_fail_rect()));
    assert!(horizontal_gap(nest_rect(), power_rect()) >= 60.0);
    assert!(pass_fail_lane_gap() >= PASS_FAIL_SEGREGATION_MIN);
    assert!(bridge_clearance_above_module_keepout() >= 48.0);
}

#[derive(Clone, Copy)]
struct Rect {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

impl Rect {
    const fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Self { x, y, w, h }
    }
}

fn deck_rect() -> Rect {
    Rect::new(0.0, 0.0, DECK_X, DECK_Y)
}

fn nest_rect() -> Rect {
    Rect::new(NEST_CENTER.0, NEST_CENTER.1, NEST_X, NEST_Y)
}

fn power_rect() -> Rect {
    Rect::new(POWER_CENTER.0, POWER_CENTER.1, POWER_GAUGE_X, POWER_GAUGE_Y)
}

fn data_rect() -> Rect {
    Rect::new(DATA_CENTER.0, DATA_CENTER.1, DATA_GAUGE_X, DATA_GAUGE_Y)
}

fn pogo_rect() -> Rect {
    Rect::new(POGO_CENTER.0, POGO_CENTER.1, POGO_X, POGO_Y)
}

fn bend_rect() -> Rect {
    Rect::new(BEND_CENTER.0, BEND_CENTER.1, BEND_X, BEND_Y)
}

fn seal_rect() -> Rect {
    Rect::new(SEAL_CENTER.0, SEAL_CENTER.1, SEAL_X, SEAL_Y)
}

fn pass_fail_rect() -> Rect {
    Rect::new(
        PASS_FAIL_CENTER.0,
        PASS_FAIL_CENTER.1,
        PASS_FAIL_X,
        PASS_FAIL_Y,
    )
}

fn rect_inside(inner: Rect, outer: Rect, margin: f64) -> bool {
    inner.x - inner.w / 2.0 >= outer.x - outer.w / 2.0 + margin
        && inner.x + inner.w / 2.0 <= outer.x + outer.w / 2.0 - margin
        && inner.y - inner.h / 2.0 >= outer.y - outer.h / 2.0 + margin
        && inner.y + inner.h / 2.0 <= outer.y + outer.h / 2.0 - margin
}

fn rects_overlap(a: Rect, b: Rect) -> bool {
    let ax0 = a.x - a.w / 2.0;
    let ax1 = a.x + a.w / 2.0;
    let ay0 = a.y - a.h / 2.0;
    let ay1 = a.y + a.h / 2.0;
    let bx0 = b.x - b.w / 2.0;
    let bx1 = b.x + b.w / 2.0;
    let by0 = b.y - b.h / 2.0;
    let by1 = b.y + b.h / 2.0;

    ax0 < bx1 && ax1 > bx0 && ay0 < by1 && ay1 > by0
}

fn horizontal_gap(a: Rect, b: Rect) -> f64 {
    let ax0 = a.x - a.w / 2.0;
    let ax1 = a.x + a.w / 2.0;
    let bx0 = b.x - b.w / 2.0;
    let bx1 = b.x + b.w / 2.0;

    if ax1 < bx0 {
        bx0 - ax1
    } else if bx1 < ax0 {
        ax0 - bx1
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 11);
        for path in OUTPUTS {
            assert!(path.starts_with(
                "output/closed_culture_module_power_data_quickconnect_validation_station_"
            ));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn required_feature_groups_are_explicit() {
        assert_eq!(REQUIRED_FEATURES.len(), 8);
        assert!(REQUIRED_FEATURES.contains(&"dockable_module_nest"));
        assert!(REQUIRED_FEATURES.contains(&"keyed_electrical_connector_gauge_blocks"));
        assert!(REQUIRED_FEATURES.contains(&"keyed_data_connector_gauge_blocks"));
        assert!(REQUIRED_FEATURES.contains(&"continuity_pogo_fixture"));
        assert!(REQUIRED_FEATURES.contains(&"strain_relief_bend_gauge"));
        assert!(REQUIRED_FEATURES.contains(&"seal_witness_lands"));
        assert!(REQUIRED_FEATURES.contains(&"barcode_rfid_camera_bridge"));
        assert!(REQUIRED_FEATURES.contains(&"pass_fail_quarantine_lanes"));
    }

    #[test]
    fn connector_and_pogo_counts_cover_validation_points() {
        assert_eq!(POWER_CONNECTOR_COUNT, 8);
        assert_eq!(DATA_CONNECTOR_COUNT, 6);
        assert_eq!(POGO_PIN_COUNT, POGO_ROWS * POGO_COLS);
        assert_eq!(POGO_PIN_COUNT, 32);
        assert!(POGO_PIN_D < POGO_COLLAR_D);
        assert!(POWER_BORE_D > DATA_SLOT_Y);
    }

    #[test]
    fn station_layout_fits_without_overlaps() {
        assert_layout();
        assert!(DECK_X <= 1120.0);
        assert!(DECK_Y <= 760.0);
        assert!(NEST_X > MODULE_X + 80.0);
        assert!(NEST_Y > MODULE_Y + 80.0);
        assert!(horizontal_gap(nest_rect(), power_rect()) >= 60.0);
    }

    #[test]
    fn traceability_and_quarantine_controls_are_segregated() {
        assert_eq!(SEAL_WITNESS_LANDS, 12);
        assert_eq!(PASS_LANES, FAIL_QUARANTINE_LANES);
        assert!(pass_fail_lane_gap() >= PASS_FAIL_SEGREGATION_MIN);
        assert!(QUARANTINE_WALL >= 8.0);
        assert!(!rects_overlap(seal_rect(), pass_fail_rect()));
    }

    #[test]
    fn bridge_clearance_and_strain_relief_are_sane() {
        assert!(BRIDGE_SPAN_X > NEST_X + 36.0);
        assert!(bridge_clearance_above_module_keepout() >= 48.0);
        assert_eq!(CAMERA_COUNT, 3);
        assert_eq!(BEND_LANES, 4);
        assert_eq!(BEND_MANDREL_COUNT, 3);
        assert!(BEND_CABLE_D > 6.0);
    }
}
