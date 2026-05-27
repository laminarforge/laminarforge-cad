use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed CO2/O2 gas-cylinder empty/changeover fault recovery station.
//
// This generator models validation and recovery packaging for bought,
// certified gas-handling hardware used in an automated cell-culture clean
// cabinet. The printed/machined geometry is for containment, location,
// cassette service, fault simulation, sensing, and traceability. It is not a
// pressure-vessel, regulator, oxygen-safety, or acceptance-limit design.

const OUTPUT_PREFIX: &str =
    "output/closed_co2_o2_gas_cylinder_empty_changeover_fault_recovery_station_";

const OUTPUTS: [&str; 11] = [
    "output/closed_co2_o2_gas_cylinder_empty_changeover_fault_recovery_station_base_recovery_tray.stl",
    "output/closed_co2_o2_gas_cylinder_empty_changeover_fault_recovery_station_dual_supply_inlet_panel.stl",
    "output/closed_co2_o2_gas_cylinder_empty_changeover_fault_recovery_station_regulator_mfc_cartridge_bank.stl",
    "output/closed_co2_o2_gas_cylinder_empty_changeover_fault_recovery_station_check_valve_bank.stl",
    "output/closed_co2_o2_gas_cylinder_empty_changeover_fault_recovery_station_purge_vent_manifold.stl",
    "output/closed_co2_o2_gas_cylinder_empty_changeover_fault_recovery_station_empty_cylinder_simulator_pockets.stl",
    "output/closed_co2_o2_gas_cylinder_empty_changeover_fault_recovery_station_o2_co2_sensor_tap_block.stl",
    "output/closed_co2_o2_gas_cylinder_empty_changeover_fault_recovery_station_pressure_decay_challenge_ports.stl",
    "output/closed_co2_o2_gas_cylinder_empty_changeover_fault_recovery_station_alarm_status_tower.stl",
    "output/closed_co2_o2_gas_cylinder_empty_changeover_fault_recovery_station_removable_service_cassette.stl",
    "output/closed_co2_o2_gas_cylinder_empty_changeover_fault_recovery_station_assembly.stl",
];

const REQUIRED_FEATURE_GROUPS: [&str; 10] = [
    "dual_co2_o2_supply_inlets",
    "regulator_mass_flow_controller_cartridges",
    "source_and_downstream_check_valves",
    "purge_vent_manifold",
    "empty_cylinder_simulator_pockets",
    "o2_co2_sensor_taps",
    "pressure_decay_challenge_ports",
    "alarm_status_tower",
    "removable_service_cassette",
    "closed_cabinet_fault_recovery_keepouts",
];

const GAS_CHANNELS: usize = 2;
const GAS_NAMES: [&str; GAS_CHANNELS] = ["co2", "o2"];
const SOURCES_PER_GAS: usize = 2;
const SOURCE_NAMES: [&str; SOURCES_PER_GAS] = ["a", "b"];
const SOURCE_COUNT: usize = GAS_CHANNELS * SOURCES_PER_GAS;
const SUPPLY_INLET_COUNT: usize = SOURCE_COUNT;
const REGULATOR_MFC_CARTRIDGES: usize = GAS_CHANNELS;
const CHECK_VALVES: usize = SOURCE_COUNT + GAS_CHANNELS;
const PURGE_BRANCHES: usize = GAS_CHANNELS;
const VENT_PORTS: usize = GAS_CHANNELS + 1;
const EMPTY_CYLINDER_SIMULATORS: usize = SOURCE_COUNT;
const SENSOR_TAPS_PER_GAS: usize = 2;
const SENSOR_TAPS: usize = GAS_CHANNELS * SENSOR_TAPS_PER_GAS;
const PRESSURE_DECAY_PORTS: usize = SOURCE_COUNT + GAS_CHANNELS;
const STATUS_LIGHTS: usize = 5;
const ALARM_BUZZER_COUNT: usize = 1;
const SERVICE_CASSETTE_BAYS: usize = GAS_CHANNELS;
const SERVICE_LATCHES: usize = 4;
const BARCODE_LANDS: usize = SOURCE_COUNT + REGULATOR_MFC_CARTRIDGES + PRESSURE_DECAY_PORTS;
const CSG_LABEL_LANDS: usize = 18;
const KEEP_OUT_GAUGES: usize = 5;

const STATION_X: f64 = 1500.0;
const STATION_Y: f64 = 860.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const SOCKET_DEPTH: f64 = 6.0;

const INLET_PANEL_X: f64 = 1140.0;
const INLET_PANEL_Y: f64 = 90.0;
const INLET_PANEL_Z: f64 = 170.0;
const INLET_PANEL_POS: (f64, f64) = (0.0, 310.0);
const INLET_PORT_D: f64 = 18.0;
const SOURCE_PITCH_X: f64 = 166.0;
const GAS_PITCH_X: f64 = 520.0;

const CARTRIDGE_BANK_X: f64 = 610.0;
const CARTRIDGE_BANK_Y: f64 = 160.0;
const CARTRIDGE_BANK_Z: f64 = 74.0;
const CARTRIDGE_BANK_POS: (f64, f64) = (-320.0, 120.0);
const CARTRIDGE_PITCH_X: f64 = 250.0;
const REGULATOR_ENV_X: f64 = 108.0;
const REGULATOR_ENV_Y: f64 = 82.0;
const REGULATOR_ENV_Z: f64 = 64.0;
const MFC_ENV_X: f64 = 118.0;
const MFC_ENV_Y: f64 = 96.0;
const MFC_ENV_Z: f64 = 42.0;
const CASSETTE_KEY_D: f64 = 14.0;

const CHECK_BANK_X: f64 = 450.0;
const CHECK_BANK_Y: f64 = 160.0;
const CHECK_BANK_Z: f64 = 64.0;
const CHECK_BANK_POS: (f64, f64) = (320.0, 120.0);
const CHECK_VALVE_D: f64 = 22.0;
const CHECK_VALVE_LEN: f64 = 72.0;

const PURGE_MANIFOLD_X: f64 = 680.0;
const PURGE_MANIFOLD_Y: f64 = 130.0;
const PURGE_MANIFOLD_Z: f64 = 66.0;
const PURGE_MANIFOLD_POS: (f64, f64) = (0.0, -50.0);
const MANIFOLD_TUBE_D: f64 = 32.0;
const PURGE_PORT_D: f64 = 12.0;
const VENT_PORT_D: f64 = 18.0;

const EMPTY_SIM_X: f64 = 430.0;
const EMPTY_SIM_Y: f64 = 150.0;
const EMPTY_SIM_Z: f64 = 54.0;
const EMPTY_SIM_POS: (f64, f64) = (-450.0, -200.0);
const SIMULATOR_POCKET_D: f64 = 58.0;
const SIMULATOR_POCKET_DEPTH: f64 = 34.0;

const SENSOR_BLOCK_X: f64 = 300.0;
const SENSOR_BLOCK_Y: f64 = 150.0;
const SENSOR_BLOCK_Z: f64 = 70.0;
const SENSOR_BLOCK_POS: (f64, f64) = (60.0, -200.0);
const SENSOR_TAP_D: f64 = 16.0;
const SENSOR_WELL_X: f64 = 54.0;
const SENSOR_WELL_Y: f64 = 40.0;

const PRESSURE_BLOCK_X: f64 = 320.0;
const PRESSURE_BLOCK_Y: f64 = 150.0;
const PRESSURE_BLOCK_Z: f64 = 70.0;
const PRESSURE_BLOCK_POS: (f64, f64) = (430.0, -200.0);
const DECAY_PORT_D: f64 = 10.0;
const DECAY_TOKEN_D: f64 = 28.0;

const STATUS_TOWER_X: f64 = 110.0;
const STATUS_TOWER_Y: f64 = 150.0;
const STATUS_TOWER_Z: f64 = 270.0;
const STATUS_TOWER_POS: (f64, f64) = (650.0, -200.0);
const STATUS_LIGHT_D: f64 = 30.0;

const SERVICE_CASSETTE_X: f64 = 760.0;
const SERVICE_CASSETTE_Y: f64 = 104.0;
const SERVICE_CASSETTE_Z: f64 = 48.0;
const SERVICE_CASSETTE_POS: (f64, f64) = (0.0, -340.0);

const FRONT_ROBOT_CLEARANCE: f64 = 430.0;
const REAR_CYLINDER_CHANGE_CLEARANCE: f64 = 320.0;
const SIDE_SERVICE_CLEARANCE: f64 = 210.0;
const TOP_CASSETTE_LIFT_CLEARANCE: f64 = 260.0;
const ALARM_VISIBILITY_CLEARANCE: f64 = 350.0;

#[derive(Clone, Copy, Debug)]
struct Footprint {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Footprint {
    fn fits_inside_station(self) -> bool {
        self.center.0.abs() + self.x / 2.0 <= STATION_X / 2.0 - RIM_W - 8.0
            && self.center.1.abs() + self.y / 2.0 <= STATION_Y / 2.0 - RIM_W - 8.0
    }

    fn overlaps(self, other: Footprint) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();
        dx < (self.x + other.x) / 2.0 && dy < (self.y + other.y) / 2.0
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_recovery_tray();
    export(OUTPUTS[0], &base);

    let inlet_panel = dual_supply_inlet_panel();
    export(OUTPUTS[1], &inlet_panel);

    let cartridges = regulator_mfc_cartridge_bank();
    export(OUTPUTS[2], &cartridges);

    let checks = check_valve_bank();
    export(OUTPUTS[3], &checks);

    let purge = purge_vent_manifold();
    export(OUTPUTS[4], &purge);

    let simulators = empty_cylinder_simulator_pockets();
    export(OUTPUTS[5], &simulators);

    let sensors = o2_co2_sensor_tap_block();
    export(OUTPUTS[6], &sensors);

    let pressure = pressure_decay_challenge_ports();
    export(OUTPUTS[7], &pressure);

    let alarm = alarm_status_tower();
    export(OUTPUTS[8], &alarm);

    let cassette = removable_service_cassette();
    export(OUTPUTS[9], &cassette);

    let assembly = base
        + inlet_panel
        + cartridges
        + checks
        + purge
        + simulators
        + sensors
        + pressure
        + alarm
        + cassette;
    export(OUTPUTS[10], &assembly);

    println!();
    println!("Closed CO2/O2 cylinder empty/changeover fault recovery station:");
    println!(
        "  Footprint:          {STATION_X:.0}mm x {STATION_Y:.0}mm closed-cabinet recovery tray"
    );
    println!(
        "  Gas supply:         {:?} with {SUPPLY_INLET_COUNT} A/B cylinder inlets, {REGULATOR_MFC_CARTRIDGES} removable regulator/MFC cartridges, and {CHECK_VALVES} check-valve envelopes",
        GAS_NAMES
    );
    println!(
        "  Fault simulation:   {EMPTY_CYLINDER_SIMULATORS} empty-cylinder simulator pockets, {PRESSURE_DECAY_PORTS} pressure-decay challenge ports, and purge/vent manifold with {PURGE_BRANCHES} purge branches plus {VENT_PORTS} vent ports"
    );
    println!(
        "  Sensing/status:     {SENSOR_TAPS} O2/CO2 sensor taps, {STATUS_LIGHTS} stack-light segments, {ALARM_BUZZER_COUNT} audible alarm envelope, and {BARCODE_LANDS} traceability lands"
    );
    println!(
        "  Service geometry:   removable cassette with {SERVICE_CASSETTE_BAYS} keyed bays, {SERVICE_LATCHES} latch keepers, {KEEP_OUT_GAUGES} cabinet keepout gauges, and {CSG_LABEL_LANDS} raised label lands"
    );
    println!(
        "  Safety boundary:    packaging/interface model for purchased gas hardware; no pressure-rated printed parts"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn deck_top_z() -> f64 {
    BASE_Z
}

fn on_deck_z(height: f64) -> f64 {
    deck_top_z() + height / 2.0 - SOCKET_DEPTH / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn gas_x(index: usize) -> f64 {
    centered_index(index, GAS_CHANNELS, GAS_PITCH_X)
}

fn source_x(gas: usize, source: usize) -> f64 {
    gas_x(gas) + centered_index(source, SOURCES_PER_GAS, SOURCE_PITCH_X)
}

fn source_label(source: usize) -> &'static str {
    SOURCE_NAMES[source]
}

fn module_footprints() -> [Footprint; 9] {
    [
        Footprint {
            name: "dual_supply_inlet_panel",
            center: INLET_PANEL_POS,
            x: INLET_PANEL_X,
            y: INLET_PANEL_Y,
        },
        Footprint {
            name: "regulator_mfc_cartridge_bank",
            center: CARTRIDGE_BANK_POS,
            x: CARTRIDGE_BANK_X,
            y: CARTRIDGE_BANK_Y,
        },
        Footprint {
            name: "check_valve_bank",
            center: CHECK_BANK_POS,
            x: CHECK_BANK_X,
            y: CHECK_BANK_Y,
        },
        Footprint {
            name: "purge_vent_manifold",
            center: PURGE_MANIFOLD_POS,
            x: PURGE_MANIFOLD_X,
            y: PURGE_MANIFOLD_Y,
        },
        Footprint {
            name: "empty_cylinder_simulator_pockets",
            center: EMPTY_SIM_POS,
            x: EMPTY_SIM_X,
            y: EMPTY_SIM_Y,
        },
        Footprint {
            name: "o2_co2_sensor_tap_block",
            center: SENSOR_BLOCK_POS,
            x: SENSOR_BLOCK_X,
            y: SENSOR_BLOCK_Y,
        },
        Footprint {
            name: "pressure_decay_challenge_ports",
            center: PRESSURE_BLOCK_POS,
            x: PRESSURE_BLOCK_X,
            y: PRESSURE_BLOCK_Y,
        },
        Footprint {
            name: "alarm_status_tower",
            center: STATUS_TOWER_POS,
            x: STATUS_TOWER_X,
            y: STATUS_TOWER_Y,
        },
        Footprint {
            name: "removable_service_cassette",
            center: SERVICE_CASSETTE_POS,
            x: SERVICE_CASSETTE_X,
            y: SERVICE_CASSETTE_Y,
        },
    ]
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 11);
    assert!(OUTPUTS
        .iter()
        .all(|path| path.starts_with(OUTPUT_PREFIX) && path.ends_with(".stl")));
    assert_eq!(REQUIRED_FEATURE_GROUPS.len(), 10);
    assert_eq!(GAS_NAMES, ["co2", "o2"]);
    assert_eq!(SOURCE_COUNT, GAS_CHANNELS * SOURCES_PER_GAS);
    assert_eq!(SUPPLY_INLET_COUNT, SOURCE_COUNT);
    assert_eq!(REGULATOR_MFC_CARTRIDGES, GAS_CHANNELS);
    assert_eq!(CHECK_VALVES, SOURCE_COUNT + GAS_CHANNELS);
    assert_eq!(PURGE_BRANCHES, GAS_CHANNELS);
    assert_eq!(VENT_PORTS, GAS_CHANNELS + 1);
    assert_eq!(EMPTY_CYLINDER_SIMULATORS, SOURCE_COUNT);
    assert_eq!(SENSOR_TAPS, GAS_CHANNELS * SENSOR_TAPS_PER_GAS);
    assert_eq!(PRESSURE_DECAY_PORTS, SOURCE_COUNT + GAS_CHANNELS);
    assert_eq!(SERVICE_CASSETTE_BAYS, GAS_CHANNELS);
    assert_eq!(SERVICE_LATCHES, 4);
    assert_eq!(KEEP_OUT_GAUGES, 5);
    assert!(FRONT_ROBOT_CLEARANCE >= 420.0);
    assert!(REAR_CYLINDER_CHANGE_CLEARANCE >= 300.0);
    assert!(SIDE_SERVICE_CLEARANCE >= 200.0);
    assert!(TOP_CASSETTE_LIFT_CLEARANCE > SERVICE_CASSETTE_Z + 180.0);
    assert!(ALARM_VISIBILITY_CLEARANCE > STATUS_TOWER_Z);

    for required in [
        "dual_co2_o2_supply_inlets",
        "regulator_mass_flow_controller_cartridges",
        "source_and_downstream_check_valves",
        "purge_vent_manifold",
        "empty_cylinder_simulator_pockets",
        "o2_co2_sensor_taps",
        "pressure_decay_challenge_ports",
        "alarm_status_tower",
        "removable_service_cassette",
        "closed_cabinet_fault_recovery_keepouts",
    ] {
        assert!(REQUIRED_FEATURE_GROUPS.contains(&required));
    }

    let footprints = module_footprints();
    for module in footprints {
        assert!(
            module.fits_inside_station(),
            "{} exceeds recovery tray envelope",
            module.name
        );
    }
    for (index, a) in footprints.iter().enumerate() {
        for b in footprints.iter().skip(index + 1) {
            assert!(!a.overlaps(*b), "{} overlaps {}", a.name, b.name);
        }
    }
}

fn base_recovery_tray() -> Part {
    let deck = centered_cube(
        "co2_o2_empty_changeover_base_recovery_tray_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let spill_sump = centered_cube(
        "co2_o2_empty_changeover_recessed_fault_recovery_sump",
        STATION_X - 150.0,
        STATION_Y - 136.0,
        8.0,
    )
    .translate(0.0, -12.0, BASE_Z - 4.0);
    let drain = centered_cylinder("co2_o2_empty_changeover_front_sump_drain", 7.0, 58.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(
            STATION_X / 2.0 - 92.0,
            -STATION_Y / 2.0 + 24.0,
            BASE_Z - 8.0,
        );

    deck - spill_sump - drain - module_sockets() - mounting_slots() - datum_pin_holes()
        + perimeter_rims()
        + cabinet_locator_rails()
        + zone_divider_ribs()
        + traceability_lands()
        + keepout_gauges()
}

fn module_sockets() -> Part {
    let mut sockets = Part::empty("co2_o2_empty_changeover_module_sockets");
    for module in module_footprints() {
        sockets = sockets
            + centered_cube(
                format!("co2_o2_empty_changeover_{}_cassette_socket", module.name),
                module.x + 10.0,
                module.y + 10.0,
                SOCKET_DEPTH + 0.6,
            )
            .translate(
                module.center.0,
                module.center.1,
                BASE_Z - SOCKET_DEPTH / 2.0 + 0.2,
            );
    }
    sockets
}

fn mounting_slots() -> Part {
    let mut slots = Part::empty("co2_o2_empty_changeover_mounting_slots");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 62.0, -STATION_Y / 2.0 + 58.0),
        (STATION_X / 2.0 - 62.0, -STATION_Y / 2.0 + 58.0),
        (-STATION_X / 2.0 + 62.0, STATION_Y / 2.0 - 58.0),
        (STATION_X / 2.0 - 62.0, STATION_Y / 2.0 - 58.0),
        (-310.0, -STATION_Y / 2.0 + 58.0),
        (310.0, -STATION_Y / 2.0 + 58.0),
        (-310.0, STATION_Y / 2.0 - 58.0),
        (310.0, STATION_Y / 2.0 - 58.0),
    ]
    .iter()
    .enumerate()
    {
        slots = slots
            + centered_cylinder(
                format!("co2_o2_empty_changeover_m6_mount_clearance_{i}"),
                3.4,
                BASE_Z + 4.0,
                28,
            )
            .translate(*x, *y, BASE_Z / 2.0)
            + centered_cube(
                format!("co2_o2_empty_changeover_mount_service_slot_{i}"),
                28.0,
                7.0,
                BASE_Z + 4.0,
            )
            .translate(*x, *y, BASE_Z / 2.0);
    }
    slots
}

fn datum_pin_holes() -> Part {
    let mut holes = Part::empty("co2_o2_empty_changeover_datum_pin_holes");
    for (i, (x, y)) in [
        (-625.0, 340.0),
        (625.0, 340.0),
        (-625.0, -356.0),
        (625.0, -356.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("co2_o2_empty_changeover_datum_pin_hole_{i}"),
                3.0,
                BASE_Z + 3.0,
                28,
            )
            .translate(*x, *y, BASE_Z / 2.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    let front = centered_cube(
        "co2_o2_empty_changeover_low_front_robot_service_lip",
        STATION_X - 220.0,
        12.0,
        24.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 18.0, BASE_Z + 12.0);
    let rear = centered_cube(
        "co2_o2_empty_changeover_rear_cylinder_change_rim",
        STATION_X - 80.0,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "co2_o2_empty_changeover_left_spill_rim",
        RIM_W,
        STATION_Y - 86.0,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "co2_o2_empty_changeover_right_spill_rim",
        RIM_W,
        STATION_Y - 86.0,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);

    front + rear + left + right
}

fn cabinet_locator_rails() -> Part {
    let rear_key = centered_cube(
        "co2_o2_empty_changeover_rear_clean_cabinet_locator_key",
        STATION_X - 260.0,
        14.0,
        18.0,
    )
    .translate(0.0, STATION_Y / 2.0 - 76.0, BASE_Z + 9.0);
    let left_slide = centered_cube(
        "co2_o2_empty_changeover_left_cabinet_slide_land",
        16.0,
        STATION_Y - 230.0,
        16.0,
    )
    .translate(-STATION_X / 2.0 + 84.0, 0.0, BASE_Z + 8.0);
    let right_slide = centered_cube(
        "co2_o2_empty_changeover_right_cabinet_slide_land",
        16.0,
        STATION_Y - 230.0,
        16.0,
    )
    .translate(STATION_X / 2.0 - 84.0, 0.0, BASE_Z + 8.0);
    rear_key + left_slide + right_slide
}

fn zone_divider_ribs() -> Part {
    let rear_service = centered_cube(
        "co2_o2_empty_changeover_inlet_cartridge_zone_rib",
        STATION_X - 250.0,
        9.0,
        24.0,
    )
    .translate(0.0, 228.0, BASE_Z + 12.0);
    let middle_fault = centered_cube(
        "co2_o2_empty_changeover_purge_sensor_fault_zone_rib",
        STATION_X - 300.0,
        9.0,
        22.0,
    )
    .translate(0.0, 12.0, BASE_Z + 11.0);
    let front_cassette = centered_cube(
        "co2_o2_empty_changeover_front_service_cassette_zone_rib",
        STATION_X - 360.0,
        8.0,
        20.0,
    )
    .translate(0.0, -285.0, BASE_Z + 10.0);
    rear_service + middle_fault + front_cassette
}

fn traceability_lands() -> Part {
    let mut lands = Part::empty("co2_o2_empty_changeover_traceability_and_csg_label_lands");
    for i in 0..CSG_LABEL_LANDS {
        let col = i % 9;
        let row = i / 9;
        lands = lands
            + label_land(
                format!("co2_o2_empty_changeover_csg_label_land_{i}"),
                56.0,
                18.0,
            )
            .translate(
                -312.0 + col as f64 * 78.0,
                -410.0 + row as f64 * 30.0,
                BASE_Z + 1.5,
            );
    }
    lands
}

fn label_land(name: impl Into<String>, x: f64, y: f64) -> Part {
    let plate = centered_cube(name, x, y, 3.0);
    let underline = centered_cube(
        "co2_o2_empty_changeover_label_underline",
        x - 10.0,
        2.0,
        1.2,
    )
    .translate(0.0, -y / 2.0 + 4.0, 2.1);
    plate + underline
}

fn keepout_gauges() -> Part {
    let front_robot = centered_cube(
        "co2_o2_empty_changeover_front_robot_recovery_clearance_gauge",
        640.0,
        10.0,
        24.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + FRONT_ROBOT_CLEARANCE, BASE_Z + 12.0);
    let rear_change = centered_cube(
        "co2_o2_empty_changeover_rear_cylinder_change_clearance_gauge",
        720.0,
        10.0,
        24.0,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - REAR_CYLINDER_CHANGE_CLEARANCE,
        BASE_Z + 12.0,
    );
    let left_service = centered_cube(
        "co2_o2_empty_changeover_left_service_cart_clearance_gauge",
        10.0,
        440.0,
        24.0,
    )
    .translate(
        -STATION_X / 2.0 + SIDE_SERVICE_CLEARANCE,
        -42.0,
        BASE_Z + 12.0,
    );
    let right_service = centered_cube(
        "co2_o2_empty_changeover_right_service_cart_clearance_gauge",
        10.0,
        440.0,
        24.0,
    )
    .translate(
        STATION_X / 2.0 - SIDE_SERVICE_CLEARANCE,
        -42.0,
        BASE_Z + 12.0,
    );
    let cassette_lift = centered_cube(
        "co2_o2_empty_changeover_service_cassette_vertical_lift_gauge",
        SERVICE_CASSETTE_X,
        14.0,
        8.0,
    )
    .translate(
        SERVICE_CASSETTE_POS.0,
        SERVICE_CASSETTE_POS.1,
        BASE_Z + TOP_CASSETTE_LIFT_CLEARANCE,
    );

    front_robot + rear_change + left_service + right_service + cassette_lift
}

fn dual_supply_inlet_panel() -> Part {
    let panel = centered_cube(
        "co2_o2_empty_changeover_dual_supply_inlet_panel",
        INLET_PANEL_X,
        INLET_PANEL_Y,
        INLET_PANEL_Z,
    )
    .translate(
        INLET_PANEL_POS.0,
        INLET_PANEL_POS.1,
        on_deck_z(INLET_PANEL_Z),
    );
    let rear_flange = centered_cube(
        "co2_o2_empty_changeover_inlet_panel_rear_mount_flange",
        INLET_PANEL_X + 44.0,
        18.0,
        42.0,
    )
    .translate(
        INLET_PANEL_POS.0,
        INLET_PANEL_POS.1 + INLET_PANEL_Y / 2.0 + 9.0,
        BASE_Z + 30.0,
    );

    let mut cuts = Part::empty("co2_o2_empty_changeover_inlet_panel_cutouts");
    let mut lands = Part::empty("co2_o2_empty_changeover_inlet_panel_lands");
    for gas in 0..GAS_CHANNELS {
        for source in 0..SOURCES_PER_GAS {
            let x = source_x(gas, source);
            let z = BASE_Z + 96.0 + source as f64 * 42.0;
            cuts = cuts
                + panel_port_cut(
                    format!(
                        "co2_o2_empty_changeover_{}_source_{}_supply_inlet_bore",
                        GAS_NAMES[gas],
                        source_label(source)
                    ),
                    INLET_PORT_D,
                    INLET_PANEL_Y,
                )
                .translate(x, INLET_PANEL_POS.1, z)
                + keyed_flat_cut(
                    format!(
                        "co2_o2_empty_changeover_{}_source_{}_inlet_key_flat",
                        GAS_NAMES[gas],
                        source_label(source)
                    ),
                    30.0,
                    INLET_PANEL_Y,
                    6.0,
                )
                .translate(x, INLET_PANEL_POS.1, z + 20.0);
            lands = lands
                + label_land(
                    format!(
                        "co2_o2_empty_changeover_{}_source_{}_inlet_label_land",
                        GAS_NAMES[gas],
                        source_label(source)
                    ),
                    76.0,
                    20.0,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(
                    x,
                    INLET_PANEL_POS.1 - INLET_PANEL_Y / 2.0 - 2.0,
                    z - 32.0,
                )
                + inlet_status_flag(gas, source).translate(
                    x,
                    INLET_PANEL_POS.1 - INLET_PANEL_Y / 2.0 - 8.0,
                    z + 34.0,
                );
        }
    }

    let common_output_cut = panel_port_cut(
        "co2_o2_empty_changeover_common_blended_gas_output_port",
        12.7,
        INLET_PANEL_Y,
    )
    .translate(0.0, INLET_PANEL_POS.1, BASE_Z + 44.0);
    let output_label = label_land(
        "co2_o2_empty_changeover_common_output_label_land",
        130.0,
        20.0,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        0.0,
        INLET_PANEL_POS.1 - INLET_PANEL_Y / 2.0 - 2.0,
        BASE_Z + 16.0,
    );

    panel + rear_flange - cuts - common_output_cut + lands + output_label + inlet_panel_gasket()
}

fn panel_port_cut(name: impl Into<String>, diameter: f64, depth: f64) -> Part {
    centered_cylinder(name, diameter / 2.0, depth + 8.0, 36).rotate(90.0, 0.0, 0.0)
}

fn keyed_flat_cut(name: impl Into<String>, x: f64, y: f64, z: f64) -> Part {
    centered_cube(name, x, y + 8.0, z)
}

fn inlet_status_flag(gas: usize, source: usize) -> Part {
    centered_cube(
        format!(
            "co2_o2_empty_changeover_{}_source_{}_mechanical_empty_status_flag",
            GAS_NAMES[gas],
            source_label(source)
        ),
        46.0,
        8.0,
        18.0,
    )
}

fn inlet_panel_gasket() -> Part {
    let outer = centered_cube(
        "co2_o2_empty_changeover_inlet_panel_outer_gasket_land",
        INLET_PANEL_X - 42.0,
        4.0,
        INLET_PANEL_Z - 28.0,
    )
    .translate(
        INLET_PANEL_POS.0,
        INLET_PANEL_POS.1 - INLET_PANEL_Y / 2.0 - 4.0,
        on_deck_z(INLET_PANEL_Z),
    );
    let inner = centered_cube(
        "co2_o2_empty_changeover_inlet_panel_inner_gasket_relief",
        INLET_PANEL_X - 92.0,
        6.0,
        INLET_PANEL_Z - 74.0,
    )
    .translate(
        INLET_PANEL_POS.0,
        INLET_PANEL_POS.1 - INLET_PANEL_Y / 2.0 - 4.0,
        on_deck_z(INLET_PANEL_Z),
    );
    outer - inner
}

fn regulator_mfc_cartridge_bank() -> Part {
    let base = centered_cube(
        "co2_o2_empty_changeover_regulator_mfc_cartridge_bank_base",
        CARTRIDGE_BANK_X,
        CARTRIDGE_BANK_Y,
        CARTRIDGE_BANK_Z,
    )
    .translate(
        CARTRIDGE_BANK_POS.0,
        CARTRIDGE_BANK_POS.1,
        on_deck_z(CARTRIDGE_BANK_Z),
    );
    let rear_bus = centered_cube(
        "co2_o2_empty_changeover_regulator_mfc_rear_pneumatic_bus_envelope",
        CARTRIDGE_BANK_X - 56.0,
        18.0,
        42.0,
    )
    .translate(
        CARTRIDGE_BANK_POS.0,
        CARTRIDGE_BANK_POS.1 + CARTRIDGE_BANK_Y / 2.0 - 22.0,
        BASE_Z + 62.0,
    );

    let mut pockets = Part::empty("co2_o2_empty_changeover_cartridge_pockets");
    let mut hardware = Part::empty("co2_o2_empty_changeover_purchased_regulator_mfc_envelopes");
    for gas in 0..GAS_CHANNELS {
        let x = CARTRIDGE_BANK_POS.0 + centered_index(gas, GAS_CHANNELS, CARTRIDGE_PITCH_X);
        pockets = pockets
            + centered_cube(
                format!(
                    "co2_o2_empty_changeover_{}_removable_cartridge_socket",
                    GAS_NAMES[gas]
                ),
                218.0,
                122.0,
                CARTRIDGE_BANK_Z + 4.0,
            )
            .translate(x, CARTRIDGE_BANK_POS.1, on_deck_z(CARTRIDGE_BANK_Z) + 6.0)
            + centered_cylinder(
                format!(
                    "co2_o2_empty_changeover_{}_cassette_key_bore",
                    GAS_NAMES[gas]
                ),
                CASSETTE_KEY_D / 2.0,
                CARTRIDGE_BANK_Z + 10.0,
                24,
            )
            .translate(
                x - 84.0,
                CARTRIDGE_BANK_POS.1 - 46.0,
                on_deck_z(CARTRIDGE_BANK_Z),
            );
        hardware = hardware
            + purchased_regulator_envelope(gas).translate(
                x - 48.0,
                CARTRIDGE_BANK_POS.1 + 4.0,
                BASE_Z + REGULATOR_ENV_Z / 2.0 + 20.0,
            )
            + purchased_mfc_envelope(gas).translate(
                x + 62.0,
                CARTRIDGE_BANK_POS.1 + 2.0,
                BASE_Z + MFC_ENV_Z / 2.0 + 18.0,
            )
            + cassette_pull_handle(gas).translate(
                x,
                CARTRIDGE_BANK_POS.1 - CARTRIDGE_BANK_Y / 2.0 - 10.0,
                BASE_Z + 48.0,
            )
            + label_land(
                format!(
                    "co2_o2_empty_changeover_{}_cartridge_barcode_land",
                    GAS_NAMES[gas]
                ),
                92.0,
                18.0,
            )
            .translate(
                x,
                CARTRIDGE_BANK_POS.1 - 18.0,
                BASE_Z + CARTRIDGE_BANK_Z + 4.0,
            );
    }

    base + rear_bus - pockets + hardware + cartridge_alignment_rails()
}

fn purchased_regulator_envelope(gas: usize) -> Part {
    let body = centered_cube(
        format!(
            "co2_o2_empty_changeover_{}_purchased_regulator_envelope",
            GAS_NAMES[gas]
        ),
        REGULATOR_ENV_X,
        REGULATOR_ENV_Y,
        REGULATOR_ENV_Z,
    );
    let knob = centered_cylinder(
        format!(
            "co2_o2_empty_changeover_{}_regulator_knob_clearance_envelope",
            GAS_NAMES[gas]
        ),
        24.0,
        18.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        0.0,
        -REGULATOR_ENV_Y / 2.0 - 9.0,
        REGULATOR_ENV_Z / 2.0 - 14.0,
    );
    body + knob
}

fn purchased_mfc_envelope(gas: usize) -> Part {
    let body = centered_cube(
        format!(
            "co2_o2_empty_changeover_{}_purchased_mfc_envelope",
            GAS_NAMES[gas]
        ),
        MFC_ENV_X,
        MFC_ENV_Y,
        MFC_ENV_Z,
    );
    let flow_arrow = centered_cube(
        format!(
            "co2_o2_empty_changeover_{}_mfc_flow_arrow_rib",
            GAS_NAMES[gas]
        ),
        MFC_ENV_X - 30.0,
        5.0,
        4.0,
    )
    .translate(0.0, -MFC_ENV_Y / 2.0 + 18.0, MFC_ENV_Z / 2.0 + 2.0);
    body + flow_arrow
}

fn cassette_pull_handle(gas: usize) -> Part {
    let bridge = centered_cube(
        format!(
            "co2_o2_empty_changeover_{}_cassette_pull_handle",
            GAS_NAMES[gas]
        ),
        118.0,
        12.0,
        22.0,
    );
    let finger_cut = centered_cube(
        format!(
            "co2_o2_empty_changeover_{}_cassette_handle_finger_relief",
            GAS_NAMES[gas]
        ),
        76.0,
        16.0,
        12.0,
    )
    .translate(0.0, 0.0, 1.0);
    bridge - finger_cut
}

fn cartridge_alignment_rails() -> Part {
    let mut rails = Part::empty("co2_o2_empty_changeover_cartridge_alignment_rails");
    for (i, y_offset) in [-68.0, 68.0].iter().enumerate() {
        rails = rails
            + centered_cube(
                format!("co2_o2_empty_changeover_cartridge_alignment_rail_{i}"),
                CARTRIDGE_BANK_X - 64.0,
                10.0,
                26.0,
            )
            .translate(
                CARTRIDGE_BANK_POS.0,
                CARTRIDGE_BANK_POS.1 + *y_offset,
                BASE_Z + 13.0,
            );
    }
    rails
}

fn check_valve_bank() -> Part {
    let base = centered_cube(
        "co2_o2_empty_changeover_check_valve_bank_base",
        CHECK_BANK_X,
        CHECK_BANK_Y,
        CHECK_BANK_Z,
    )
    .translate(CHECK_BANK_POS.0, CHECK_BANK_POS.1, on_deck_z(CHECK_BANK_Z));
    let mut checks = Part::empty("co2_o2_empty_changeover_check_valve_envelopes");

    for gas in 0..GAS_CHANNELS {
        for source in 0..SOURCES_PER_GAS {
            let local_index = gas * SOURCES_PER_GAS + source;
            checks = checks
                + check_valve_envelope(
                    format!(
                        "co2_o2_empty_changeover_{}_source_{}_check_valve",
                        GAS_NAMES[gas],
                        source_label(source)
                    ),
                    local_index,
                )
                .translate(
                    CHECK_BANK_POS.0 + centered_index(source, SOURCES_PER_GAS, 116.0),
                    CHECK_BANK_POS.1 + centered_index(gas, GAS_CHANNELS, 72.0),
                    BASE_Z + 54.0,
                );
        }
        checks = checks
            + check_valve_envelope(
                format!(
                    "co2_o2_empty_changeover_{}_downstream_isolation_check_valve",
                    GAS_NAMES[gas]
                ),
                SOURCE_COUNT + gas,
            )
            .translate(
                CHECK_BANK_POS.0 + 152.0,
                CHECK_BANK_POS.1 + centered_index(gas, GAS_CHANNELS, 72.0),
                BASE_Z + 54.0,
            );
    }

    base - check_valve_cradles() + checks + check_valve_direction_tags()
}

fn check_valve_envelope(name: impl Into<String>, index: usize) -> Part {
    let valve =
        centered_cylinder(name, CHECK_VALVE_D / 2.0, CHECK_VALVE_LEN, 32).rotate(0.0, 90.0, 0.0);
    let arrow = centered_cube(
        format!("co2_o2_empty_changeover_check_valve_flow_arrow_{index}"),
        34.0,
        5.0,
        6.0,
    )
    .translate(0.0, -CHECK_VALVE_D / 2.0 - 4.0, 0.0);
    valve + arrow
}

fn check_valve_cradles() -> Part {
    let mut cradles = Part::empty("co2_o2_empty_changeover_check_valve_cradles");
    for gas in 0..GAS_CHANNELS {
        for source in 0..SOURCES_PER_GAS {
            cradles = cradles
                + centered_cube(
                    format!(
                        "co2_o2_empty_changeover_{}_source_{}_check_valve_saddle",
                        GAS_NAMES[gas],
                        source_label(source)
                    ),
                    86.0,
                    28.0,
                    20.0,
                )
                .translate(
                    CHECK_BANK_POS.0 + centered_index(source, SOURCES_PER_GAS, 116.0),
                    CHECK_BANK_POS.1 + centered_index(gas, GAS_CHANNELS, 72.0),
                    BASE_Z + 47.0,
                );
        }
        cradles = cradles
            + centered_cube(
                format!(
                    "co2_o2_empty_changeover_{}_downstream_check_valve_saddle",
                    GAS_NAMES[gas]
                ),
                86.0,
                28.0,
                20.0,
            )
            .translate(
                CHECK_BANK_POS.0 + 152.0,
                CHECK_BANK_POS.1 + centered_index(gas, GAS_CHANNELS, 72.0),
                BASE_Z + 47.0,
            );
    }
    cradles
}

fn check_valve_direction_tags() -> Part {
    let mut tags = Part::empty("co2_o2_empty_changeover_check_valve_direction_tags");
    for gas in 0..GAS_CHANNELS {
        tags = tags
            + label_land(
                format!(
                    "co2_o2_empty_changeover_{}_check_valve_direction_tag",
                    GAS_NAMES[gas]
                ),
                92.0,
                18.0,
            )
            .translate(
                CHECK_BANK_POS.0 - CHECK_BANK_X / 2.0 + 78.0,
                CHECK_BANK_POS.1 + centered_index(gas, GAS_CHANNELS, 72.0),
                BASE_Z + CHECK_BANK_Z + 3.0,
            );
    }
    tags
}

fn purge_vent_manifold() -> Part {
    let base = centered_cube(
        "co2_o2_empty_changeover_purge_vent_manifold_base",
        PURGE_MANIFOLD_X,
        PURGE_MANIFOLD_Y,
        PURGE_MANIFOLD_Z,
    )
    .translate(
        PURGE_MANIFOLD_POS.0,
        PURGE_MANIFOLD_POS.1,
        on_deck_z(PURGE_MANIFOLD_Z),
    );
    let manifold_tube = centered_cylinder(
        "co2_o2_empty_changeover_common_purge_vent_manifold_tube",
        MANIFOLD_TUBE_D / 2.0,
        PURGE_MANIFOLD_X - 96.0,
        40,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(
        PURGE_MANIFOLD_POS.0,
        PURGE_MANIFOLD_POS.1,
        BASE_Z + PURGE_MANIFOLD_Z / 2.0 + 16.0,
    );

    let mut cuts = Part::empty("co2_o2_empty_changeover_purge_vent_port_cuts");
    let mut ports = Part::empty("co2_o2_empty_changeover_purge_vent_port_envelopes");
    for gas in 0..GAS_CHANNELS {
        let x = PURGE_MANIFOLD_POS.0 + centered_index(gas, GAS_CHANNELS, 250.0);
        cuts = cuts
            + vertical_port_cut(
                format!(
                    "co2_o2_empty_changeover_{}_purge_injection_port_cut",
                    GAS_NAMES[gas]
                ),
                PURGE_PORT_D,
                PURGE_MANIFOLD_Z,
            )
            .translate(x, PURGE_MANIFOLD_POS.1 - 34.0, BASE_Z + 44.0);
        ports = ports
            + centered_cylinder(
                format!(
                    "co2_o2_empty_changeover_{}_purge_knob_witness",
                    GAS_NAMES[gas]
                ),
                18.0,
                10.0,
                32,
            )
            .translate(
                x,
                PURGE_MANIFOLD_POS.1 - 34.0,
                BASE_Z + PURGE_MANIFOLD_Z + 7.0,
            )
            + label_land(
                format!(
                    "co2_o2_empty_changeover_{}_purge_label_land",
                    GAS_NAMES[gas]
                ),
                70.0,
                16.0,
            )
            .translate(
                x,
                PURGE_MANIFOLD_POS.1 - 62.0,
                BASE_Z + PURGE_MANIFOLD_Z + 3.0,
            );
    }

    for i in 0..VENT_PORTS {
        let x = PURGE_MANIFOLD_POS.0 + centered_index(i, VENT_PORTS, 136.0);
        cuts = cuts
            + panel_port_cut(
                format!("co2_o2_empty_changeover_vent_port_cut_{i}"),
                VENT_PORT_D,
                PURGE_MANIFOLD_Y,
            )
            .translate(x, PURGE_MANIFOLD_POS.1 + 38.0, BASE_Z + 50.0);
        ports = ports
            + centered_cube(
                format!("co2_o2_empty_changeover_vent_filter_envelope_{i}"),
                54.0,
                22.0,
                32.0,
            )
            .translate(x, PURGE_MANIFOLD_POS.1 + 64.0, BASE_Z + 54.0);
    }

    base + manifold_tube - cuts + ports + purge_path_witness_channels()
}

fn vertical_port_cut(name: impl Into<String>, diameter: f64, height: f64) -> Part {
    centered_cylinder(name, diameter / 2.0, height + 8.0, 32)
}

fn purge_path_witness_channels() -> Part {
    let mut channels = Part::empty("co2_o2_empty_changeover_purge_path_witness_channels");
    for gas in 0..GAS_CHANNELS {
        channels = channels
            + centered_cube(
                format!(
                    "co2_o2_empty_changeover_{}_purge_path_visual_witness_channel",
                    GAS_NAMES[gas]
                ),
                210.0,
                8.0,
                8.0,
            )
            .translate(
                PURGE_MANIFOLD_POS.0 + centered_index(gas, GAS_CHANNELS, 250.0),
                PURGE_MANIFOLD_POS.1,
                BASE_Z + PURGE_MANIFOLD_Z + 4.0,
            );
    }
    channels
}

fn empty_cylinder_simulator_pockets() -> Part {
    let tray = centered_cube(
        "co2_o2_empty_changeover_empty_cylinder_simulator_pocket_tray",
        EMPTY_SIM_X,
        EMPTY_SIM_Y,
        EMPTY_SIM_Z,
    )
    .translate(EMPTY_SIM_POS.0, EMPTY_SIM_POS.1, on_deck_z(EMPTY_SIM_Z));
    let mut pocket_cuts = Part::empty("co2_o2_empty_changeover_empty_simulator_pocket_cuts");
    let mut tokens = Part::empty("co2_o2_empty_changeover_empty_pressure_simulator_tokens");

    for gas in 0..GAS_CHANNELS {
        for source in 0..SOURCES_PER_GAS {
            let x = EMPTY_SIM_POS.0 + centered_index(source, SOURCES_PER_GAS, 112.0);
            let y = EMPTY_SIM_POS.1 + centered_index(gas, GAS_CHANNELS, 66.0);
            pocket_cuts = pocket_cuts
                + centered_cylinder(
                    format!(
                        "co2_o2_empty_changeover_{}_source_{}_empty_simulator_socket",
                        GAS_NAMES[gas],
                        source_label(source)
                    ),
                    SIMULATOR_POCKET_D / 2.0,
                    SIMULATOR_POCKET_DEPTH + 2.0,
                    48,
                )
                .translate(
                    x,
                    y,
                    BASE_Z + EMPTY_SIM_Z - SIMULATOR_POCKET_DEPTH / 2.0,
                );
            tokens = tokens
                + centered_cylinder(
                    format!(
                        "co2_o2_empty_changeover_{}_source_{}_empty_pressure_token",
                        GAS_NAMES[gas],
                        source_label(source)
                    ),
                    SIMULATOR_POCKET_D / 2.0 - 8.0,
                    9.0,
                    48,
                )
                .translate(x, y, BASE_Z + EMPTY_SIM_Z + 4.5)
                + label_land(
                    format!(
                        "co2_o2_empty_changeover_{}_source_{}_empty_token_barcode",
                        GAS_NAMES[gas],
                        source_label(source)
                    ),
                    58.0,
                    14.0,
                )
                .translate(x, y - 44.0, BASE_Z + EMPTY_SIM_Z + 2.0);
        }
    }

    tray - pocket_cuts + tokens + simulator_instruction_tabs()
}

fn simulator_instruction_tabs() -> Part {
    let co2_tab = centered_cube(
        "co2_o2_empty_changeover_co2_empty_simulator_tab",
        92.0,
        20.0,
        8.0,
    )
    .translate(
        EMPTY_SIM_POS.0 - 164.0,
        EMPTY_SIM_POS.1 - 38.0,
        BASE_Z + EMPTY_SIM_Z + 4.0,
    );
    let o2_tab = centered_cube(
        "co2_o2_empty_changeover_o2_empty_simulator_tab",
        92.0,
        20.0,
        8.0,
    )
    .translate(
        EMPTY_SIM_POS.0 - 164.0,
        EMPTY_SIM_POS.1 + 38.0,
        BASE_Z + EMPTY_SIM_Z + 4.0,
    );
    co2_tab + o2_tab
}

fn o2_co2_sensor_tap_block() -> Part {
    let block = centered_cube(
        "co2_o2_empty_changeover_o2_co2_sensor_tap_block",
        SENSOR_BLOCK_X,
        SENSOR_BLOCK_Y,
        SENSOR_BLOCK_Z,
    )
    .translate(
        SENSOR_BLOCK_POS.0,
        SENSOR_BLOCK_POS.1,
        on_deck_z(SENSOR_BLOCK_Z),
    );
    let mut tap_cuts = Part::empty("co2_o2_empty_changeover_sensor_tap_cuts");
    let mut wells = Part::empty("co2_o2_empty_changeover_sensor_probe_wells");

    for gas in 0..GAS_CHANNELS {
        for tap in 0..SENSOR_TAPS_PER_GAS {
            let x = SENSOR_BLOCK_POS.0 + centered_index(tap, SENSOR_TAPS_PER_GAS, 84.0);
            let y = SENSOR_BLOCK_POS.1 + centered_index(gas, GAS_CHANNELS, 66.0);
            tap_cuts = tap_cuts
                + vertical_port_cut(
                    format!(
                        "co2_o2_empty_changeover_{}_sensor_tap_{}_bore",
                        GAS_NAMES[gas], tap
                    ),
                    SENSOR_TAP_D,
                    SENSOR_BLOCK_Z,
                )
                .translate(x, y, BASE_Z + SENSOR_BLOCK_Z / 2.0);
            wells = wells
                + centered_cube(
                    format!(
                        "co2_o2_empty_changeover_{}_sensor_tap_{}_probe_well",
                        GAS_NAMES[gas], tap
                    ),
                    SENSOR_WELL_X,
                    SENSOR_WELL_Y,
                    18.0,
                )
                .translate(x, y, BASE_Z + SENSOR_BLOCK_Z + 9.0)
                + label_land(
                    format!(
                        "co2_o2_empty_changeover_{}_sensor_tap_{}_label_land",
                        GAS_NAMES[gas], tap
                    ),
                    56.0,
                    15.0,
                )
                .translate(x, y + 32.0, BASE_Z + SENSOR_BLOCK_Z + 3.0);
        }
    }

    block - tap_cuts + wells + sensor_flow_bridge()
}

fn sensor_flow_bridge() -> Part {
    let bridge = centered_cylinder(
        "co2_o2_empty_changeover_sensor_tap_common_flow_bridge",
        9.0,
        SENSOR_BLOCK_X - 72.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(SENSOR_BLOCK_POS.0, SENSOR_BLOCK_POS.1, BASE_Z + 46.0);
    let sample_drain = centered_cube(
        "co2_o2_empty_changeover_sensor_tap_condensate_drain_witness_slot",
        SENSOR_BLOCK_X - 84.0,
        8.0,
        6.0,
    )
    .translate(SENSOR_BLOCK_POS.0, SENSOR_BLOCK_POS.1 - 58.0, BASE_Z + 30.0);
    bridge + sample_drain
}

fn pressure_decay_challenge_ports() -> Part {
    let block = centered_cube(
        "co2_o2_empty_changeover_pressure_decay_challenge_port_block",
        PRESSURE_BLOCK_X,
        PRESSURE_BLOCK_Y,
        PRESSURE_BLOCK_Z,
    )
    .translate(
        PRESSURE_BLOCK_POS.0,
        PRESSURE_BLOCK_POS.1,
        on_deck_z(PRESSURE_BLOCK_Z),
    );
    let mut cuts = Part::empty("co2_o2_empty_changeover_pressure_decay_port_cuts");
    let mut tokens = Part::empty("co2_o2_empty_changeover_pressure_decay_evidence_tokens");

    for i in 0..PRESSURE_DECAY_PORTS {
        let col = i % 3;
        let row = i / 3;
        let x = PRESSURE_BLOCK_POS.0 + centered_index(col, 3, 76.0);
        let y = PRESSURE_BLOCK_POS.1 + centered_index(row, 2, 58.0);
        cuts = cuts
            + vertical_port_cut(
                format!("co2_o2_empty_changeover_pressure_decay_challenge_bore_{i}"),
                DECAY_PORT_D,
                PRESSURE_BLOCK_Z,
            )
            .translate(x, y, BASE_Z + PRESSURE_BLOCK_Z / 2.0);
        tokens = tokens
            + centered_cylinder(
                format!("co2_o2_empty_changeover_pressure_decay_result_token_{i}"),
                DECAY_TOKEN_D / 2.0,
                8.0,
                32,
            )
            .translate(x, y, BASE_Z + PRESSURE_BLOCK_Z + 4.0)
            + label_land(
                format!("co2_o2_empty_changeover_pressure_decay_barcode_land_{i}"),
                48.0,
                13.0,
            )
            .translate(x, y - 28.0, BASE_Z + PRESSURE_BLOCK_Z + 3.0);
    }

    block - cuts + tokens + decay_reference_gauge_lands()
}

fn decay_reference_gauge_lands() -> Part {
    let left = centered_cube(
        "co2_o2_empty_changeover_decay_reference_gauge_land_low",
        80.0,
        18.0,
        10.0,
    )
    .translate(
        PRESSURE_BLOCK_POS.0 - PRESSURE_BLOCK_X / 2.0 + 54.0,
        PRESSURE_BLOCK_POS.1 + PRESSURE_BLOCK_Y / 2.0 - 18.0,
        BASE_Z + PRESSURE_BLOCK_Z + 5.0,
    );
    let right = centered_cube(
        "co2_o2_empty_changeover_decay_reference_gauge_land_high",
        80.0,
        18.0,
        10.0,
    )
    .translate(
        PRESSURE_BLOCK_POS.0 + PRESSURE_BLOCK_X / 2.0 - 54.0,
        PRESSURE_BLOCK_POS.1 + PRESSURE_BLOCK_Y / 2.0 - 18.0,
        BASE_Z + PRESSURE_BLOCK_Z + 5.0,
    );
    left + right
}

fn alarm_status_tower() -> Part {
    let pedestal = centered_cube(
        "co2_o2_empty_changeover_alarm_status_tower_pedestal",
        STATUS_TOWER_X,
        STATUS_TOWER_Y,
        48.0,
    )
    .translate(STATUS_TOWER_POS.0, STATUS_TOWER_POS.1, BASE_Z + 24.0);
    let mast = centered_cube(
        "co2_o2_empty_changeover_alarm_status_tower_mast",
        34.0,
        34.0,
        STATUS_TOWER_Z - 48.0,
    )
    .translate(
        STATUS_TOWER_POS.0,
        STATUS_TOWER_POS.1,
        BASE_Z + 48.0 + (STATUS_TOWER_Z - 48.0) / 2.0,
    );
    let mut lights = Part::empty("co2_o2_empty_changeover_status_stack_light_segments");
    for i in 0..STATUS_LIGHTS {
        lights = lights
            + centered_cylinder(
                format!("co2_o2_empty_changeover_status_stack_light_segment_{i}"),
                STATUS_LIGHT_D / 2.0,
                18.0,
                36,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                STATUS_TOWER_POS.0,
                STATUS_TOWER_POS.1 - STATUS_TOWER_Y / 2.0 - 9.0,
                BASE_Z + 78.0 + i as f64 * 34.0,
            );
    }
    let buzzer = centered_cylinder(
        "co2_o2_empty_changeover_audible_alarm_buzzer_envelope",
        28.0,
        22.0,
        40,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATUS_TOWER_POS.0,
        STATUS_TOWER_POS.1 - STATUS_TOWER_Y / 2.0 - 11.0,
        BASE_Z + STATUS_TOWER_Z - 26.0,
    );
    let status_labels = alarm_status_label_lands();
    pedestal + mast + lights + buzzer + status_labels
}

fn alarm_status_label_lands() -> Part {
    let mut lands = Part::empty("co2_o2_empty_changeover_alarm_status_label_lands");
    for i in 0..STATUS_LIGHTS {
        lands = lands
            + label_land(
                format!("co2_o2_empty_changeover_alarm_status_label_land_{i}"),
                74.0,
                15.0,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                STATUS_TOWER_POS.0,
                STATUS_TOWER_POS.1 - STATUS_TOWER_Y / 2.0 - 3.0,
                BASE_Z + 78.0 + i as f64 * 34.0,
            );
    }
    lands
}

fn removable_service_cassette() -> Part {
    let frame_outer = centered_cube(
        "co2_o2_empty_changeover_removable_service_cassette_outer_frame",
        SERVICE_CASSETTE_X,
        SERVICE_CASSETTE_Y,
        SERVICE_CASSETTE_Z,
    )
    .translate(
        SERVICE_CASSETTE_POS.0,
        SERVICE_CASSETTE_POS.1,
        on_deck_z(SERVICE_CASSETTE_Z),
    );
    let frame_void = centered_cube(
        "co2_o2_empty_changeover_removable_service_cassette_center_void",
        SERVICE_CASSETTE_X - 72.0,
        SERVICE_CASSETTE_Y - 34.0,
        SERVICE_CASSETTE_Z + 4.0,
    )
    .translate(
        SERVICE_CASSETTE_POS.0,
        SERVICE_CASSETTE_POS.1,
        on_deck_z(SERVICE_CASSETTE_Z),
    );

    let mut bays = Part::empty("co2_o2_empty_changeover_service_cassette_keyed_bays");
    for gas in 0..GAS_CHANNELS {
        let x = SERVICE_CASSETTE_POS.0 + centered_index(gas, GAS_CHANNELS, 260.0);
        bays =
            bays + centered_cube(
                format!(
                    "co2_o2_empty_changeover_{}_service_cassette_drop_in_bay",
                    GAS_NAMES[gas]
                ),
                218.0,
                SERVICE_CASSETTE_Y - 20.0,
                SERVICE_CASSETTE_Z + 8.0,
            )
            .translate(
                x,
                SERVICE_CASSETTE_POS.1,
                on_deck_z(SERVICE_CASSETTE_Z) + 2.0,
            ) + centered_cylinder(
                format!(
                    "co2_o2_empty_changeover_{}_service_cassette_asymmetric_key_socket",
                    GAS_NAMES[gas]
                ),
                10.0,
                SERVICE_CASSETTE_Z + 8.0,
                24,
            )
            .translate(
                x - 86.0,
                SERVICE_CASSETTE_POS.1 - 30.0,
                on_deck_z(SERVICE_CASSETTE_Z) + 2.0,
            );
    }

    (frame_outer - frame_void - bays)
        + cassette_latches()
        + cassette_handle()
        + cassette_barcode_lands()
}

fn cassette_latches() -> Part {
    let mut latches = Part::empty("co2_o2_empty_changeover_service_cassette_latch_keepers");
    for (i, (x, y)) in [
        (
            SERVICE_CASSETTE_POS.0 - SERVICE_CASSETTE_X / 2.0 + 54.0,
            SERVICE_CASSETTE_POS.1 - SERVICE_CASSETTE_Y / 2.0 + 22.0,
        ),
        (
            SERVICE_CASSETTE_POS.0 + SERVICE_CASSETTE_X / 2.0 - 54.0,
            SERVICE_CASSETTE_POS.1 - SERVICE_CASSETTE_Y / 2.0 + 22.0,
        ),
        (
            SERVICE_CASSETTE_POS.0 - SERVICE_CASSETTE_X / 2.0 + 54.0,
            SERVICE_CASSETTE_POS.1 + SERVICE_CASSETTE_Y / 2.0 - 22.0,
        ),
        (
            SERVICE_CASSETTE_POS.0 + SERVICE_CASSETTE_X / 2.0 - 54.0,
            SERVICE_CASSETTE_POS.1 + SERVICE_CASSETTE_Y / 2.0 - 22.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        latches = latches
            + centered_cube(
                format!("co2_o2_empty_changeover_service_cassette_latch_keeper_{i}"),
                52.0,
                20.0,
                18.0,
            )
            .translate(*x, *y, BASE_Z + SERVICE_CASSETTE_Z + 9.0);
    }
    latches
}

fn cassette_handle() -> Part {
    let bridge = centered_cube(
        "co2_o2_empty_changeover_service_cassette_front_pull_bridge",
        250.0,
        16.0,
        26.0,
    )
    .translate(
        SERVICE_CASSETTE_POS.0,
        SERVICE_CASSETTE_POS.1 - SERVICE_CASSETTE_Y / 2.0 - 12.0,
        BASE_Z + SERVICE_CASSETTE_Z / 2.0 + 18.0,
    );
    let finger_void = centered_cube(
        "co2_o2_empty_changeover_service_cassette_pull_bridge_finger_void",
        178.0,
        20.0,
        13.0,
    )
    .translate(
        SERVICE_CASSETTE_POS.0,
        SERVICE_CASSETTE_POS.1 - SERVICE_CASSETTE_Y / 2.0 - 12.0,
        BASE_Z + SERVICE_CASSETTE_Z / 2.0 + 18.0,
    );
    bridge - finger_void
}

fn cassette_barcode_lands() -> Part {
    let mut lands = Part::empty("co2_o2_empty_changeover_service_cassette_barcode_lands");
    for gas in 0..GAS_CHANNELS {
        lands = lands
            + label_land(
                format!(
                    "co2_o2_empty_changeover_{}_service_cassette_barcode_land",
                    GAS_NAMES[gas]
                ),
                94.0,
                16.0,
            )
            .translate(
                SERVICE_CASSETTE_POS.0 + centered_index(gas, GAS_CHANNELS, 260.0),
                SERVICE_CASSETTE_POS.1 + 32.0,
                BASE_Z + SERVICE_CASSETTE_Z + 3.0,
            );
    }
    lands
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 11);
        for output in OUTPUTS {
            assert!(output.starts_with(OUTPUT_PREFIX));
            assert!(output.ends_with(".stl"));
        }
    }

    #[test]
    fn requested_station_features_are_explicit() {
        for feature in [
            "dual_co2_o2_supply_inlets",
            "regulator_mass_flow_controller_cartridges",
            "source_and_downstream_check_valves",
            "purge_vent_manifold",
            "empty_cylinder_simulator_pockets",
            "o2_co2_sensor_taps",
            "pressure_decay_challenge_ports",
            "alarm_status_tower",
            "removable_service_cassette",
            "closed_cabinet_fault_recovery_keepouts",
        ] {
            assert!(REQUIRED_FEATURE_GROUPS.contains(&feature));
        }
    }

    #[test]
    fn gas_changeover_counts_match_dual_source_co2_o2_layout() {
        assert_eq!(GAS_NAMES, ["co2", "o2"]);
        assert_eq!(SOURCES_PER_GAS, 2);
        assert_eq!(SOURCE_NAMES, ["a", "b"]);
        assert_eq!(SOURCE_COUNT, 4);
        assert_eq!(SUPPLY_INLET_COUNT, SOURCE_COUNT);
        assert_eq!(EMPTY_CYLINDER_SIMULATORS, SOURCE_COUNT);
        assert_eq!(source_label(0), "a");
        assert_eq!(source_label(1), "b");
    }

    #[test]
    fn recovery_hardware_envelopes_cover_fault_diagnosis_path() {
        assert_eq!(REGULATOR_MFC_CARTRIDGES, GAS_CHANNELS);
        assert_eq!(CHECK_VALVES, SOURCE_COUNT + GAS_CHANNELS);
        assert_eq!(PURGE_BRANCHES, GAS_CHANNELS);
        assert_eq!(VENT_PORTS, GAS_CHANNELS + 1);
        assert_eq!(SENSOR_TAPS, GAS_CHANNELS * SENSOR_TAPS_PER_GAS);
        assert_eq!(PRESSURE_DECAY_PORTS, SOURCE_COUNT + GAS_CHANNELS);
        assert!(REGULATOR_ENV_X > 90.0);
        assert!(MFC_ENV_X > REGULATOR_ENV_X);
        assert!(MANIFOLD_TUBE_D > CHECK_VALVE_D);
    }

    #[test]
    fn layout_fits_recovery_tray_without_module_overlap() {
        assert_design_constraints();
    }

    #[test]
    fn service_and_status_geometry_has_cabinet_clearance() {
        assert_eq!(STATUS_LIGHTS, 5);
        assert_eq!(ALARM_BUZZER_COUNT, 1);
        assert_eq!(SERVICE_CASSETTE_BAYS, GAS_CHANNELS);
        assert_eq!(SERVICE_LATCHES, 4);
        assert_eq!(KEEP_OUT_GAUGES, 5);
        assert!(SERVICE_CASSETTE_POS.1 - SERVICE_CASSETTE_Y / 2.0 > -STATION_Y / 2.0 + RIM_W);
        assert!(STATUS_TOWER_POS.0 + STATUS_TOWER_X / 2.0 < STATION_X / 2.0 - RIM_W);
        assert!(ALARM_VISIBILITY_CLEARANCE > STATUS_TOWER_Z);
    }
}
