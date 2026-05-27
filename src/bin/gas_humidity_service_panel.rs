use vcad::{centered_cube, centered_cylinder, Part};

// External gas and humidity service panel for modular closed culture boxes.
//
// Scope:
// - Keep all gas mixing, humidification, relief, calibration, and sensor access
//   outside the sterile culture box volume.
// - Present explicit bulkhead families for CO2/O2/N2/air inputs, conditioned gas
//   outputs, sterile vent filters, calibration gas, relief/exhaust, condensate,
//   and sensor pockets.
// - Model purchased regulators/MFCs/filters as service envelopes, not DIY
//   pressure-rated components.

const OUTPUTS: &[&str] = &[
    "output/gas_humidity_service_panel_baseplate.stl",
    "output/gas_humidity_service_panel_bulkhead_panel.stl",
    "output/gas_humidity_service_panel_gas_control_bank.stl",
    "output/gas_humidity_service_panel_distribution_manifold.stl",
    "output/gas_humidity_service_panel_humidifier_cradle.stl",
    "output/gas_humidity_service_panel_condenser_trap.stl",
    "output/gas_humidity_service_panel_filter_relief_bank.stl",
    "output/gas_humidity_service_panel_sensor_calibration_bank.stl",
    "output/gas_humidity_service_panel_isolation_cover.stl",
    "output/gas_humidity_service_panel_assembly.stl",
];

const GAS_CHANNELS: usize = 4;
const GAS_NAMES: [&str; GAS_CHANNELS] = ["co2", "o2", "n2", "air"];
const REGULATOR_COUNT: usize = GAS_CHANNELS;
const MFC_COUNT: usize = GAS_CHANNELS;
const CULTURE_BOX_FEEDS: usize = 6;
const STERILE_FILTER_PORTS: usize = 4;
const SENSOR_POCKETS: usize = 4;
const CALIBRATION_PORTS: usize = 1;
const RELIEF_EXHAUST_PATHS: usize = 2;
const CONDENSATE_DRAINS: usize = 2;
const SERVICE_ISOLATION_VALVES: usize =
    GAS_CHANNELS + CULTURE_BOX_FEEDS + RELIEF_EXHAUST_PATHS + CALIBRATION_PORTS;

const BASE_X: f64 = 760.0;
const BASE_Y: f64 = 360.0;
const BASE_Z: f64 = 18.0;

const PANEL_X: f64 = 700.0;
const PANEL_Y: f64 = 26.0;
const PANEL_Z: f64 = 470.0;
const PANEL_BASE_Y: f64 = BASE_Y / 2.0 - 48.0;

const GAS_CHANNEL_PITCH_X: f64 = 136.0;
const GAS_INLET_PORT_D: f64 = 12.5;
const REGULATOR_X: f64 = 78.0;
const REGULATOR_Y: f64 = 54.0;
const REGULATOR_Z: f64 = 70.0;
const MFC_X: f64 = 92.0;
const MFC_Y: f64 = 52.0;
const MFC_Z: f64 = 84.0;

const DISTRIBUTION_X: f64 = 610.0;
const DISTRIBUTION_Y: f64 = 58.0;
const DISTRIBUTION_Z: f64 = 78.0;
const CULTURE_FEED_PITCH_X: f64 = 88.0;
const CULTURE_FEED_PORT_D: f64 = 10.4;

const HUMIDIFIER_X: f64 = 305.0;
const HUMIDIFIER_Y: f64 = 150.0;
const HUMIDIFIER_Z: f64 = 250.0;
const HUMIDIFIER_BOTTLE_D: f64 = 86.0;
const HUMIDIFIER_BOTTLE_Z: f64 = 218.0;
const WATER_JACKET_D: f64 = 112.0;
const WATER_JACKET_Z: f64 = 132.0;

const CONDENSER_X: f64 = 205.0;
const CONDENSER_Y: f64 = 126.0;
const CONDENSER_Z: f64 = 236.0;

const FILTER_BANK_X: f64 = 560.0;
const FILTER_BANK_Y: f64 = 66.0;
const FILTER_BANK_Z: f64 = 118.0;
const FILTER_PITCH_X: f64 = 112.0;
const FILTER_BODY_D: f64 = 34.0;
const FILTER_BODY_LEN: f64 = 86.0;

const SENSOR_BANK_X: f64 = 640.0;
const SENSOR_BANK_Y: f64 = 78.0;
const SENSOR_BANK_Z: f64 = 128.0;
const SENSOR_PITCH_X: f64 = 118.0;
const SENSOR_POCKET_X: f64 = 68.0;
const SENSOR_POCKET_Z: f64 = 54.0;

const ISOLATION_COVER_X: f64 = 720.0;
const ISOLATION_COVER_Y: f64 = 84.0;
const ISOLATION_COVER_Z: f64 = 128.0;
const ISOLATION_VALVE_PITCH_X: f64 = 50.0;

fn main() {
    let baseplate = baseplate();
    export(&baseplate, OUTPUTS[0]);

    let bulkhead_panel = bulkhead_panel();
    export(&bulkhead_panel, OUTPUTS[1]);

    let gas_control_bank = gas_control_bank();
    export(&gas_control_bank, OUTPUTS[2]);

    let distribution_manifold = distribution_manifold();
    export(&distribution_manifold, OUTPUTS[3]);

    let humidifier_cradle = humidifier_cradle();
    export(&humidifier_cradle, OUTPUTS[4]);

    let condenser_trap = condenser_trap();
    export(&condenser_trap, OUTPUTS[5]);

    let filter_relief_bank = filter_relief_bank();
    export(&filter_relief_bank, OUTPUTS[6]);

    let sensor_calibration_bank = sensor_calibration_bank();
    export(&sensor_calibration_bank, OUTPUTS[7]);

    let isolation_cover = isolation_cover();
    export(&isolation_cover, OUTPUTS[8]);

    let assembly = baseplate
        + bulkhead_panel.translate(0.0, PANEL_BASE_Y, BASE_Z / 2.0 + PANEL_Z / 2.0)
        + gas_control_bank.translate(0.0, PANEL_BASE_Y - 50.0, BASE_Z / 2.0 + 292.0)
        + distribution_manifold.translate(0.0, PANEL_BASE_Y - 58.0, BASE_Z / 2.0 + 112.0)
        + humidifier_cradle.translate(-205.0, -88.0, BASE_Z / 2.0)
        + condenser_trap.translate(185.0, -88.0, BASE_Z / 2.0)
        + filter_relief_bank.translate(0.0, PANEL_BASE_Y - 64.0, BASE_Z / 2.0 + 405.0)
        + sensor_calibration_bank.translate(0.0, -BASE_Y / 2.0 + 102.0, BASE_Z / 2.0 + 82.0)
        + isolation_cover.translate(0.0, -BASE_Y / 2.0 + 48.0, BASE_Z / 2.0 + 70.0);

    export(&assembly, OUTPUTS[9]);

    println!(
        "Gas/humidity service panel: {:.0}mm x {:.0}mm footprint, {:.0}mm rear panel, {} regulators, {} MFC placeholders, {} culture-box feeds, {} sterile 0.2um/HEPA vent filter ports, {} sensor pockets, {:.0}mm humidifier cradle, condenser trap, {} condensate drains, relief/exhaust path, and external isolation cover.",
        BASE_X,
        BASE_Y,
        PANEL_Z,
        REGULATOR_COUNT,
        MFC_COUNT,
        CULTURE_BOX_FEEDS,
        STERILE_FILTER_PORTS,
        SENSOR_POCKETS,
        HUMIDIFIER_Z,
        CONDENSATE_DRAINS
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn baseplate() -> Part {
    let deck = centered_cube(
        "gas_humidity_service_panel_baseplate",
        BASE_X,
        BASE_Y,
        BASE_Z,
    );

    let spill_sump = centered_cube(
        "gas_humidity_base_front_condensate_sump",
        BASE_X - 96.0,
        88.0,
        8.0,
    )
    .translate(0.0, -BASE_Y / 2.0 + 72.0, BASE_Z / 2.0 - 3.0);

    let sump_drain = centered_cylinder(
        "gas_humidity_base_condensate_floor_drain",
        8.0 / 2.0,
        40.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(BASE_X / 2.0 - 72.0, -BASE_Y / 2.0 + 20.0, 0.0);

    let rear_panel_socket = centered_cube(
        "gas_humidity_rear_bulkhead_panel_socket",
        PANEL_X + 28.0,
        18.0,
        10.0,
    )
    .translate(0.0, PANEL_BASE_Y, BASE_Z / 2.0 - 3.0);

    let mut mount_slots = Part::empty("gas_humidity_base_mount_slots");
    for (i, (x, y)) in base_mount_points().iter().enumerate() {
        mount_slots = mount_slots
            + centered_cylinder(
                format!("gas_humidity_base_m6_clearance_{i}"),
                6.6 / 2.0,
                BASE_Z + 2.0,
                24,
            )
            .translate(*x, *y, 0.0)
            + centered_cube(
                format!("gas_humidity_base_m6_service_slot_{i}"),
                24.0,
                6.8,
                BASE_Z + 2.0,
            )
            .translate(*x, *y, 0.0);
    }

    deck - spill_sump - sump_drain - rear_panel_socket - mount_slots
        + base_rails()
        + leveling_feet()
        + service_cartridge_locator_tabs()
}

fn base_rails() -> Part {
    let left = centered_cube(
        "gas_humidity_left_base_guard_rail",
        18.0,
        BASE_Y - 58.0,
        28.0,
    )
    .translate(-(BASE_X / 2.0 - 30.0), 0.0, BASE_Z / 2.0 + 14.0);
    let right = centered_cube(
        "gas_humidity_right_base_guard_rail",
        18.0,
        BASE_Y - 58.0,
        28.0,
    )
    .translate(BASE_X / 2.0 - 30.0, 0.0, BASE_Z / 2.0 + 14.0);
    let rear = centered_cube(
        "gas_humidity_rear_panel_heel_rail",
        BASE_X - 84.0,
        16.0,
        28.0,
    )
    .translate(0.0, PANEL_BASE_Y - 22.0, BASE_Z / 2.0 + 14.0);
    let front = centered_cube("gas_humidity_front_service_lip", BASE_X - 120.0, 14.0, 20.0)
        .translate(0.0, -BASE_Y / 2.0 + 26.0, BASE_Z / 2.0 + 10.0);

    left + right + rear + front
}

fn leveling_feet() -> Part {
    let mut feet = Part::empty("gas_humidity_base_leveling_feet");
    for (i, (x, y)) in [
        (-(BASE_X / 2.0 - 56.0), -(BASE_Y / 2.0 - 48.0)),
        (BASE_X / 2.0 - 56.0, -(BASE_Y / 2.0 - 48.0)),
        (-(BASE_X / 2.0 - 56.0), BASE_Y / 2.0 - 48.0),
        (BASE_X / 2.0 - 56.0, BASE_Y / 2.0 - 48.0),
        (0.0, -(BASE_Y / 2.0 - 48.0)),
        (0.0, BASE_Y / 2.0 - 48.0),
    ]
    .iter()
    .enumerate()
    {
        let pad = centered_cylinder(format!("gas_humidity_leveling_pad_{i}"), 22.0, 10.0, 36)
            .translate(*x, *y, -(BASE_Z / 2.0 + 5.0));
        let adjuster = centered_cylinder(
            format!("gas_humidity_leveler_thread_clearance_{i}"),
            8.0 / 2.0,
            16.0,
            24,
        )
        .translate(*x, *y, -(BASE_Z / 2.0 + 5.0));
        feet = feet + (pad - adjuster);
    }
    feet
}

fn service_cartridge_locator_tabs() -> Part {
    let mut tabs = Part::empty("gas_humidity_service_cartridge_locator_tabs");
    for (i, x) in [-305.0, -105.0, 105.0, 305.0].iter().enumerate() {
        let tab = centered_cube(
            format!("gas_humidity_cartridge_locator_tab_{i}"),
            32.0,
            46.0,
            16.0,
        )
        .translate(*x, -96.0, BASE_Z / 2.0 + 8.0);
        let socket = centered_cylinder(
            format!("gas_humidity_cartridge_locator_socket_{i}"),
            4.0 / 2.0,
            18.0,
            20,
        )
        .translate(*x, -96.0, BASE_Z / 2.0 + 8.0);
        tabs = tabs + (tab - socket);
    }
    tabs
}

fn bulkhead_panel() -> Part {
    let panel = centered_cube(
        "gas_humidity_rear_bulkhead_panel",
        PANEL_X,
        PANEL_Y,
        PANEL_Z,
    );
    let mut cuts = Part::empty("gas_humidity_rear_bulkhead_panel_cuts");

    for i in 0..GAS_CHANNELS {
        cuts = cuts
            + centered_cylinder(
                format!("gas_humidity_{}_gas_inlet_bulkhead", GAS_NAMES[i]),
                GAS_INLET_PORT_D / 2.0,
                PANEL_Y + 10.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(gas_channel_x(i), 0.0, 178.0);
    }

    for i in 0..CULTURE_BOX_FEEDS {
        cuts = cuts
            + centered_cylinder(
                format!("gas_humidity_box_feed_bulkhead_{i}"),
                CULTURE_FEED_PORT_D / 2.0,
                PANEL_Y + 10.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(culture_box_feed_x(i), 0.0, -172.0);
    }

    for (i, x) in [-305.0, 305.0].iter().enumerate() {
        cuts = cuts
            + centered_cylinder(
                format!("gas_humidity_relief_exhaust_bulkhead_{i}"),
                14.0 / 2.0,
                PANEL_Y + 10.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 0.0, -72.0);
    }

    cuts = cuts
        + centered_cylinder(
            "gas_humidity_calibration_gas_bulkhead",
            6.4 / 2.0,
            PANEL_Y + 10.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(-304.0, 0.0, 72.0);

    for i in 0..SENSOR_POCKETS {
        cuts = cuts
            + centered_cylinder(
                format!("gas_humidity_sensor_sample_port_{i}"),
                4.8 / 2.0,
                PANEL_Y + 10.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(sensor_pocket_x(i), 0.0, 72.0);
    }

    let cable_transit = centered_cube(
        "gas_humidity_rear_bulkhead_cable_transit",
        130.0,
        PANEL_Y + 10.0,
        58.0,
    )
    .translate(262.0, 0.0, 68.0);

    panel - cuts - cable_transit
        + panel_label_strip("gas_inputs", PANEL_X - 90.0).translate(0.0, -18.0, 214.0)
        + panel_label_strip("conditioned_outputs", PANEL_X - 90.0).translate(0.0, -18.0, -214.0)
        + gasket_land("rear_bulkhead_outer_gasket", PANEL_X - 42.0, PANEL_Z - 42.0)
}

fn gasket_land(name: &str, width: f64, height: f64) -> Part {
    let top = centered_cube(format!("gas_humidity_{name}_top"), width, 4.0, 8.0).translate(
        0.0,
        -(PANEL_Y / 2.0 + 3.0),
        height / 2.0,
    );
    let bottom = centered_cube(format!("gas_humidity_{name}_bottom"), width, 4.0, 8.0).translate(
        0.0,
        -(PANEL_Y / 2.0 + 3.0),
        -height / 2.0,
    );
    let left = centered_cube(format!("gas_humidity_{name}_left"), 8.0, 4.0, height).translate(
        -width / 2.0,
        -(PANEL_Y / 2.0 + 3.0),
        0.0,
    );
    let right = centered_cube(format!("gas_humidity_{name}_right"), 8.0, 4.0, height).translate(
        width / 2.0,
        -(PANEL_Y / 2.0 + 3.0),
        0.0,
    );
    top + bottom + left + right
}

fn gas_control_bank() -> Part {
    let backer = centered_cube(
        "gas_humidity_gas_control_backer_plate",
        PANEL_X - 84.0,
        12.0,
        332.0,
    )
    .translate(0.0, 10.0, 8.0);

    let mut controls = Part::empty("gas_humidity_gas_control_channels");
    for i in 0..GAS_CHANNELS {
        let x = gas_channel_x(i);
        controls = controls
            + regulator_placeholder(GAS_NAMES[i]).translate(x, -24.0, 104.0)
            + mfc_placeholder(GAS_NAMES[i]).translate(x, -24.0, -32.0);
    }

    let mixing_block = gas_mixing_block().translate(0.0, -28.0, -150.0);

    backer
        + controls
        + mixing_block
        + panel_label_strip("mfc_regulator_row", PANEL_X - 132.0).translate(0.0, -22.0, 168.0)
}

fn regulator_placeholder(name: &str) -> Part {
    let body = centered_cube(
        format!("gas_humidity_{name}_regulator_body"),
        REGULATOR_X,
        REGULATOR_Y,
        REGULATOR_Z,
    );
    let gauge = centered_cylinder(
        format!("gas_humidity_{name}_regulator_gauge"),
        25.0,
        12.0,
        40,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, -(REGULATOR_Y / 2.0 + 6.0), 12.0);
    let knob = centered_cylinder(
        format!("gas_humidity_{name}_regulator_knob"),
        16.0,
        20.0,
        32,
    )
    .translate(0.0, 0.0, REGULATOR_Z / 2.0 + 10.0);
    let inlet = centered_cylinder(
        format!("gas_humidity_{name}_regulator_inlet_bore"),
        7.0 / 2.0,
        REGULATOR_X + 10.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0);
    let relief_boss = centered_cylinder(
        format!("gas_humidity_{name}_regulator_relief_boss"),
        8.0,
        16.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(REGULATOR_X / 2.0 + 4.0, -8.0, -18.0);

    body + gauge + knob + relief_boss - inlet
}

fn mfc_placeholder(name: &str) -> Part {
    let body = centered_cube(format!("gas_humidity_{name}_mfc_body"), MFC_X, MFC_Y, MFC_Z);
    let display_recess = centered_cube(
        format!("gas_humidity_{name}_mfc_display_recess"),
        54.0,
        8.0,
        30.0,
    )
    .translate(0.0, -(MFC_Y / 2.0 + 1.0), 15.0);
    let flow_bore = centered_cylinder(
        format!("gas_humidity_{name}_mfc_flow_bore"),
        5.0 / 2.0,
        MFC_X + 12.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, -24.0);
    let mount_slot_left = centered_cube(
        format!("gas_humidity_{name}_mfc_left_mount_slot"),
        8.0,
        MFC_Y + 4.0,
        16.0,
    )
    .translate(-MFC_X / 2.0 + 12.0, 0.0, MFC_Z / 2.0 - 14.0);
    let mount_slot_right = centered_cube(
        format!("gas_humidity_{name}_mfc_right_mount_slot"),
        8.0,
        MFC_Y + 4.0,
        16.0,
    )
    .translate(MFC_X / 2.0 - 12.0, 0.0, MFC_Z / 2.0 - 14.0);

    body - display_recess - flow_bore - mount_slot_left - mount_slot_right
}

fn gas_mixing_block() -> Part {
    let body = centered_cube(
        "gas_humidity_mixed_gas_manifold_body",
        PANEL_X - 150.0,
        46.0,
        56.0,
    );
    let main_bore = centered_cylinder(
        "gas_humidity_mixed_gas_main_bore",
        8.0 / 2.0,
        PANEL_X - 118.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0);

    let mut inlet_bores = Part::empty("gas_humidity_mixing_block_inlet_bores");
    for i in 0..GAS_CHANNELS {
        inlet_bores = inlet_bores
            + centered_cylinder(
                format!("gas_humidity_mixing_block_channel_bore_{i}"),
                5.0 / 2.0,
                54.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(gas_channel_x(i), 0.0, 12.0);
    }

    let flush_port = centered_cylinder("gas_humidity_mixing_block_flush_port", 6.0 / 2.0, 56.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(PANEL_X / 2.0 - 116.0, 0.0, -14.0);

    body - main_bore - inlet_bores - flush_port + manifold_port_nuts(GAS_CHANNELS)
}

fn distribution_manifold() -> Part {
    let body = centered_cube(
        "gas_humidity_conditioned_distribution_manifold",
        DISTRIBUTION_X,
        DISTRIBUTION_Y,
        DISTRIBUTION_Z,
    );
    let main_bore = centered_cylinder(
        "gas_humidity_conditioned_main_bore",
        9.5 / 2.0,
        DISTRIBUTION_X + 18.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, 10.0);

    let mut outlet_bores = Part::empty("gas_humidity_culture_box_feed_bores");
    let mut outlet_lands = Part::empty("gas_humidity_culture_box_feed_lands");
    for i in 0..CULTURE_BOX_FEEDS {
        let x = culture_box_feed_x(i);
        outlet_bores = outlet_bores
            + centered_cylinder(
                format!("gas_humidity_culture_box_feed_bore_{i}"),
                CULTURE_FEED_PORT_D / 2.0,
                DISTRIBUTION_Y + 14.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, -12.0);
        outlet_lands = outlet_lands
            + centered_cylinder(
                format!("gas_humidity_culture_box_feed_bulkhead_land_{i}"),
                17.0,
                8.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -(DISTRIBUTION_Y / 2.0 + 4.0), -12.0);
    }

    let bypass_bore = centered_cylinder(
        "gas_humidity_distribution_bypass_bore",
        6.0 / 2.0,
        DISTRIBUTION_Y + 14.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DISTRIBUTION_X / 2.0 - 48.0, 0.0, 22.0);
    let relief_branch = centered_cylinder(
        "gas_humidity_distribution_relief_branch",
        6.0 / 2.0,
        72.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-(DISTRIBUTION_X / 2.0 - 48.0), 0.0, 22.0);

    body - main_bore - outlet_bores - bypass_bore - relief_branch
        + outlet_lands
        + panel_label_strip("conditioned_feed_labels", DISTRIBUTION_X - 40.0).translate(
            0.0,
            -(DISTRIBUTION_Y / 2.0 + 4.0),
            -34.0,
        )
}

fn humidifier_cradle() -> Part {
    let tray = centered_cube(
        "gas_humidity_humidifier_spill_cradle_tray",
        HUMIDIFIER_X,
        HUMIDIFIER_Y,
        24.0,
    )
    .translate(0.0, 0.0, 12.0);
    let tray_sump = centered_cube(
        "gas_humidity_humidifier_tray_sump",
        HUMIDIFIER_X - 42.0,
        HUMIDIFIER_Y - 38.0,
        9.0,
    )
    .translate(0.0, 0.0, 22.0);
    let drain = centered_cylinder(
        "gas_humidity_humidifier_tray_condensate_drain",
        6.0 / 2.0,
        36.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(HUMIDIFIER_X / 2.0 - 34.0, -(HUMIDIFIER_Y / 2.0 + 4.0), 17.0);

    let bottle_socket = centered_cylinder(
        "gas_humidity_humidifier_bottle_socket",
        HUMIDIFIER_BOTTLE_D / 2.0 + 5.0,
        28.0,
        56,
    )
    .translate(-72.0, 0.0, 24.0);
    let bottle_envelope = centered_cylinder(
        "gas_humidity_humidifier_bottle_envelope",
        HUMIDIFIER_BOTTLE_D / 2.0,
        HUMIDIFIER_BOTTLE_Z,
        56,
    )
    .translate(-72.0, 0.0, 24.0 + HUMIDIFIER_BOTTLE_Z / 2.0);

    let jacket_outer = centered_cylinder(
        "gas_humidity_heated_water_jacket_outer",
        WATER_JACKET_D / 2.0,
        WATER_JACKET_Z,
        56,
    )
    .translate(-72.0, 0.0, 86.0);
    let jacket_inner = centered_cylinder(
        "gas_humidity_heated_water_jacket_inner_clearance",
        HUMIDIFIER_BOTTLE_D / 2.0 + 3.0,
        WATER_JACKET_Z + 2.0,
        56,
    )
    .translate(-72.0, 0.0, 86.0);

    let cartridge_body = centered_cube(
        "gas_humidity_disposable_humidifier_cartridge_envelope",
        102.0,
        92.0,
        92.0,
    )
    .translate(96.0, 0.0, 74.0);
    let cartridge_window = centered_cube(
        "gas_humidity_disposable_humidifier_cartridge_window",
        62.0,
        98.0,
        44.0,
    )
    .translate(96.0, 0.0, 86.0);

    let inlet = centered_cylinder("gas_humidity_humidifier_inlet_port", 5.0 / 2.0, 126.0, 24)
        .rotate(0.0, 90.0, 0.0)
        .translate(16.0, 0.0, 128.0);
    let outlet = centered_cylinder("gas_humidity_humidifier_outlet_port", 5.0 / 2.0, 126.0, 24)
        .rotate(0.0, 90.0, 0.0)
        .translate(16.0, 0.0, 154.0);

    tray - tray_sump - drain - bottle_socket
        + bottle_envelope
        + (jacket_outer - jacket_inner)
        + (cartridge_body - cartridge_window - inlet - outlet)
        + humidifier_retainers()
}

fn humidifier_retainers() -> Part {
    let bottle_strap = centered_cube(
        "gas_humidity_humidifier_bottle_retain_strap",
        118.0,
        12.0,
        16.0,
    )
    .translate(-72.0, -(HUMIDIFIER_Y / 2.0 + 6.0), 142.0);
    let cartridge_latch_left = centered_cube(
        "gas_humidity_humidifier_cartridge_left_latch",
        12.0,
        28.0,
        72.0,
    )
    .translate(36.0, -(HUMIDIFIER_Y / 2.0 + 8.0), 78.0);
    let cartridge_latch_right = centered_cube(
        "gas_humidity_humidifier_cartridge_right_latch",
        12.0,
        28.0,
        72.0,
    )
    .translate(156.0, -(HUMIDIFIER_Y / 2.0 + 8.0), 78.0);

    bottle_strap + cartridge_latch_left + cartridge_latch_right
}

fn condenser_trap() -> Part {
    let outer = centered_cube(
        "gas_humidity_condensate_knockout_trap_outer",
        CONDENSER_X,
        CONDENSER_Y,
        CONDENSER_Z,
    )
    .translate(0.0, 0.0, CONDENSER_Z / 2.0);
    let cavity = centered_cube(
        "gas_humidity_condensate_knockout_trap_cavity",
        CONDENSER_X - 28.0,
        CONDENSER_Y - 28.0,
        CONDENSER_Z - 38.0,
    )
    .translate(0.0, 0.0, CONDENSER_Z / 2.0 + 10.0);

    let inlet = centered_cylinder("gas_humidity_condenser_wet_gas_inlet", 7.0 / 2.0, 58.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(-62.0, -(CONDENSER_Y / 2.0 + 2.0), 168.0);
    let outlet = centered_cylinder("gas_humidity_condenser_dry_gas_outlet", 7.0 / 2.0, 58.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(62.0, -(CONDENSER_Y / 2.0 + 2.0), 168.0);
    let bottom_drain = centered_cylinder(
        "gas_humidity_condenser_condensate_bottom_drain",
        7.0 / 2.0,
        34.0,
        24,
    )
    .translate(0.0, 0.0, 5.0);

    let baffle_a = centered_cube(
        "gas_humidity_condenser_baffle_a",
        14.0,
        CONDENSER_Y - 36.0,
        132.0,
    )
    .translate(-32.0, 0.0, 98.0);
    let baffle_b = centered_cube(
        "gas_humidity_condenser_baffle_b",
        14.0,
        CONDENSER_Y - 36.0,
        132.0,
    )
    .translate(32.0, 0.0, 138.0);
    let probe_port = centered_cylinder(
        "gas_humidity_condenser_dewpoint_probe_port",
        8.0 / 2.0,
        52.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, -(CONDENSER_Y / 2.0 + 2.0), 92.0);

    (outer - cavity - inlet - outlet - bottom_drain - probe_port)
        + baffle_a
        + baffle_b
        + condenser_mounting_feet()
}

fn condenser_mounting_feet() -> Part {
    centered_cube("gas_humidity_condenser_left_foot", 70.0, 22.0, 16.0).translate(
        -54.0,
        CONDENSER_Y / 2.0 + 12.0,
        8.0,
    ) + centered_cube("gas_humidity_condenser_right_foot", 70.0, 22.0, 16.0).translate(
        54.0,
        CONDENSER_Y / 2.0 + 12.0,
        8.0,
    )
}

fn filter_relief_bank() -> Part {
    let backer = centered_cube(
        "gas_humidity_filter_relief_backer",
        FILTER_BANK_X,
        FILTER_BANK_Y,
        FILTER_BANK_Z,
    );

    let mut filter_bores = Part::empty("gas_humidity_sterile_filter_port_bores");
    let mut filters = Part::empty("gas_humidity_sterile_filter_cartridges");
    for i in 0..STERILE_FILTER_PORTS {
        let x = filter_x(i);
        let cartridge = centered_cylinder(
            format!("gas_humidity_0_2um_hepa_filter_cartridge_{i}"),
            FILTER_BODY_D / 2.0,
            FILTER_BODY_LEN,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -(FILTER_BANK_Y / 2.0 + FILTER_BODY_LEN / 2.0), 22.0);
        let clamp = centered_cube(
            format!("gas_humidity_0_2um_filter_clamp_{i}"),
            54.0,
            14.0,
            22.0,
        )
        .translate(x, -(FILTER_BANK_Y / 2.0 + 8.0), 22.0);
        filters = filters + cartridge + clamp;
        filter_bores = filter_bores
            + centered_cylinder(
                format!("gas_humidity_sterile_filter_panel_port_{i}"),
                10.0 / 2.0,
                FILTER_BANK_Y + 12.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, 22.0);
    }

    let relief_valve = relief_valve_placeholder("primary").translate(-188.0, -58.0, -34.0);
    let exhaust_valve = relief_valve_placeholder("exhaust").translate(188.0, -58.0, -34.0);
    let exhaust_muffler =
        centered_cylinder("gas_humidity_exhaust_muffler_placeholder", 20.0, 88.0, 32)
            .rotate(90.0, 0.0, 0.0)
            .translate(270.0, -58.0, -34.0);
    let relief_manifold = centered_cube(
        "gas_humidity_relief_exhaust_manifold",
        FILTER_BANK_X - 110.0,
        32.0,
        38.0,
    )
    .translate(0.0, -42.0, -34.0);
    let relief_bore = centered_cylinder(
        "gas_humidity_relief_exhaust_manifold_bore",
        8.0 / 2.0,
        FILTER_BANK_X - 78.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -42.0, -34.0);

    backer - filter_bores
        + filters
        + (relief_manifold - relief_bore)
        + relief_valve
        + exhaust_valve
        + exhaust_muffler
        + panel_label_strip("sterile_filter_relief_bank", FILTER_BANK_X - 42.0).translate(
            0.0,
            -(FILTER_BANK_Y / 2.0 + 4.0),
            54.0,
        )
}

fn relief_valve_placeholder(name: &str) -> Part {
    let body = centered_cylinder(
        format!("gas_humidity_{name}_relief_valve_body"),
        18.0,
        42.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0);
    let stem = centered_cylinder(
        format!("gas_humidity_{name}_relief_valve_stem"),
        8.0,
        28.0,
        24,
    )
    .translate(0.0, 0.0, 24.0);
    let knob = centered_cylinder(
        format!("gas_humidity_{name}_relief_valve_knob"),
        17.0,
        10.0,
        28,
    )
    .translate(0.0, 0.0, 42.0);

    body + stem + knob
}

fn sensor_calibration_bank() -> Part {
    let plate = centered_cube(
        "gas_humidity_sensor_calibration_service_plate",
        SENSOR_BANK_X,
        SENSOR_BANK_Y,
        SENSOR_BANK_Z,
    );
    let mut pocket_cuts = Part::empty("gas_humidity_sensor_pocket_cuts");
    let mut sensor_lands = Part::empty("gas_humidity_sensor_mount_lands");

    for i in 0..SENSOR_POCKETS {
        let x = sensor_pocket_x(i);
        pocket_cuts = pocket_cuts
            + centered_cube(
                format!("gas_humidity_sensor_pocket_cutout_{i}"),
                SENSOR_POCKET_X,
                SENSOR_BANK_Y + 8.0,
                SENSOR_POCKET_Z,
            )
            .translate(x, 0.0, 22.0);
        sensor_lands = sensor_lands
            + centered_cube(
                format!("gas_humidity_sensor_face_land_{i}"),
                SENSOR_POCKET_X + 16.0,
                8.0,
                SENSOR_POCKET_Z + 16.0,
            )
            .translate(x, -(SENSOR_BANK_Y / 2.0 + 4.0), 22.0);
    }

    let calibration_port =
        centered_cylinder("gas_humidity_calibration_gas_quick_connect", 11.0, 36.0, 28)
            .rotate(90.0, 0.0, 0.0)
            .translate(
                -(SENSOR_BANK_X / 2.0 - 44.0),
                -(SENSOR_BANK_Y / 2.0 + 18.0),
                -34.0,
            );
    let calibration_bore = centered_cylinder(
        "gas_humidity_calibration_gas_bore",
        6.4 / 2.0,
        SENSOR_BANK_Y + 12.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-(SENSOR_BANK_X / 2.0 - 44.0), 0.0, -34.0);

    let sample_pump = centered_cube("gas_humidity_sample_pump_envelope", 72.0, 46.0, 44.0)
        .translate(SENSOR_BANK_X / 2.0 - 72.0, -18.0, -32.0);
    let sample_manifold = centered_cube("gas_humidity_sensor_sample_manifold", 360.0, 34.0, 32.0)
        .translate(24.0, -24.0, -34.0);
    let purge_drain_port = centered_cylinder(
        "gas_humidity_sensor_bank_purge_drain_port",
        5.0 / 2.0,
        52.0,
        20,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(SENSOR_BANK_X / 2.0 - 42.0, 0.0, -48.0);

    plate - pocket_cuts - calibration_bore - purge_drain_port
        + sensor_lands
        + calibration_port
        + sample_pump
        + sample_manifold
        + panel_label_strip("co2_o2_rh_pressure_sensor_row", SENSOR_BANK_X - 50.0).translate(
            0.0,
            -(SENSOR_BANK_Y / 2.0 + 5.0),
            SENSOR_BANK_Z / 2.0 - 12.0,
        )
}

fn isolation_cover() -> Part {
    let outer = centered_cube(
        "gas_humidity_external_isolation_cover_outer",
        ISOLATION_COVER_X,
        ISOLATION_COVER_Y,
        ISOLATION_COVER_Z,
    );
    let inner = centered_cube(
        "gas_humidity_external_isolation_cover_clearance",
        ISOLATION_COVER_X - 34.0,
        ISOLATION_COVER_Y - 24.0,
        ISOLATION_COVER_Z - 26.0,
    )
    .translate(0.0, -4.0, 0.0);
    let service_window = centered_cube(
        "gas_humidity_external_isolation_cover_front_window",
        ISOLATION_COVER_X - 120.0,
        18.0,
        62.0,
    )
    .translate(0.0, -(ISOLATION_COVER_Y / 2.0 + 2.0), 10.0);

    let mut valves = Part::empty("gas_humidity_external_isolation_valves");
    for i in 0..SERVICE_ISOLATION_VALVES {
        let x = isolation_valve_x(i);
        let stem = centered_cylinder(
            format!("gas_humidity_isolation_valve_stem_{i}"),
            5.0,
            24.0,
            20,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -(ISOLATION_COVER_Y / 2.0 + 12.0), -36.0);
        let handle = centered_cube(
            format!("gas_humidity_isolation_valve_lockout_handle_{i}"),
            30.0,
            10.0,
            12.0,
        )
        .translate(x, -(ISOLATION_COVER_Y / 2.0 + 26.0), -36.0);
        valves = valves + stem + handle;
    }

    let lock_left = lockout_tab("left").translate(-(ISOLATION_COVER_X / 2.0 - 34.0), -46.0, 46.0);
    let lock_right = lockout_tab("right").translate(ISOLATION_COVER_X / 2.0 - 34.0, -46.0, 46.0);
    let gasket = cover_gasket_lip();

    outer - inner - service_window + valves + lock_left + lock_right + gasket
}

fn lockout_tab(name: &str) -> Part {
    let tab = centered_cube(
        format!("gas_humidity_isolation_cover_{name}_lockout_tab"),
        32.0,
        12.0,
        34.0,
    );
    let shackle = centered_cylinder(
        format!("gas_humidity_isolation_cover_{name}_shackle_hole"),
        5.5 / 2.0,
        14.0,
        20,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, 0.0, 6.0);
    tab - shackle
}

fn cover_gasket_lip() -> Part {
    let top = centered_cube(
        "gas_humidity_isolation_cover_top_gasket_lip",
        ISOLATION_COVER_X - 70.0,
        6.0,
        8.0,
    )
    .translate(
        0.0,
        ISOLATION_COVER_Y / 2.0 - 9.0,
        ISOLATION_COVER_Z / 2.0 - 18.0,
    );
    let bottom = centered_cube(
        "gas_humidity_isolation_cover_bottom_gasket_lip",
        ISOLATION_COVER_X - 70.0,
        6.0,
        8.0,
    )
    .translate(
        0.0,
        ISOLATION_COVER_Y / 2.0 - 9.0,
        -(ISOLATION_COVER_Z / 2.0 - 18.0),
    );
    let left = centered_cube(
        "gas_humidity_isolation_cover_left_gasket_lip",
        8.0,
        6.0,
        ISOLATION_COVER_Z - 54.0,
    )
    .translate(
        -(ISOLATION_COVER_X / 2.0 - 36.0),
        ISOLATION_COVER_Y / 2.0 - 9.0,
        0.0,
    );
    let right = centered_cube(
        "gas_humidity_isolation_cover_right_gasket_lip",
        8.0,
        6.0,
        ISOLATION_COVER_Z - 54.0,
    )
    .translate(
        ISOLATION_COVER_X / 2.0 - 36.0,
        ISOLATION_COVER_Y / 2.0 - 9.0,
        0.0,
    );
    top + bottom + left + right
}

fn manifold_port_nuts(count: usize) -> Part {
    let mut nuts = Part::empty("gas_humidity_manifold_port_nuts");
    for i in 0..count {
        let nut = centered_cylinder(
            format!("gas_humidity_mixing_block_port_nut_{i}"),
            13.0,
            6.0,
            6,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(gas_channel_x(i), -28.0, 12.0);
        nuts = nuts + nut;
    }
    nuts
}

fn panel_label_strip(name: &str, width: f64) -> Part {
    centered_cube(format!("gas_humidity_label_strip_{name}"), width, 4.0, 10.0)
}

fn base_mount_points() -> [(f64, f64); 6] {
    [
        (-(BASE_X / 2.0 - 44.0), -(BASE_Y / 2.0 - 38.0)),
        (BASE_X / 2.0 - 44.0, -(BASE_Y / 2.0 - 38.0)),
        (-(BASE_X / 2.0 - 44.0), PANEL_BASE_Y + 2.0),
        (BASE_X / 2.0 - 44.0, PANEL_BASE_Y + 2.0),
        (-132.0, -(BASE_Y / 2.0 - 38.0)),
        (132.0, -(BASE_Y / 2.0 - 38.0)),
    ]
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn gas_channel_x(index: usize) -> f64 {
    centered_index(index, GAS_CHANNELS, GAS_CHANNEL_PITCH_X)
}

fn culture_box_feed_x(index: usize) -> f64 {
    centered_index(index, CULTURE_BOX_FEEDS, CULTURE_FEED_PITCH_X)
}

fn filter_x(index: usize) -> f64 {
    centered_index(index, STERILE_FILTER_PORTS, FILTER_PITCH_X)
}

fn sensor_pocket_x(index: usize) -> f64 {
    centered_index(index, SENSOR_POCKETS, SENSOR_PITCH_X)
}

fn isolation_valve_x(index: usize) -> f64 {
    centered_index(index, SERVICE_ISOLATION_VALVES, ISOLATION_VALVE_PITCH_X)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 10);
        for path in OUTPUTS {
            assert!(path.starts_with("output/gas_humidity_service_panel_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn gas_controls_cover_incubator_mix_without_extra_implicit_channels() {
        assert_eq!(GAS_NAMES, ["co2", "o2", "n2", "air"]);
        assert_eq!(REGULATOR_COUNT, GAS_CHANNELS);
        assert_eq!(MFC_COUNT, GAS_CHANNELS);
        assert_eq!(gas_channel_x(0), -gas_channel_x(GAS_CHANNELS - 1));
        assert!(gas_channel_x(0).abs() + REGULATOR_X / 2.0 < PANEL_X / 2.0 - 42.0);
        assert!(gas_channel_x(GAS_CHANNELS - 1).abs() + MFC_X / 2.0 < PANEL_X / 2.0 - 42.0);
    }

    #[test]
    fn culture_box_feeds_and_filter_ports_fit_the_panel() {
        assert_eq!(CULTURE_BOX_FEEDS, 6);
        assert_eq!(STERILE_FILTER_PORTS, 4);
        assert!(culture_box_feed_x(0).abs() + CULTURE_FEED_PORT_D < DISTRIBUTION_X / 2.0 - 28.0);
        assert!(
            culture_box_feed_x(CULTURE_BOX_FEEDS - 1).abs() + CULTURE_FEED_PORT_D
                < DISTRIBUTION_X / 2.0 - 28.0
        );
        assert!(filter_x(0).abs() + FILTER_BODY_D / 2.0 < FILTER_BANK_X / 2.0 - 32.0);
        assert!(
            filter_x(STERILE_FILTER_PORTS - 1).abs() + FILTER_BODY_D / 2.0
                < FILTER_BANK_X / 2.0 - 32.0
        );
    }

    #[test]
    fn humidity_and_condensate_service_features_have_clearance() {
        assert!(HUMIDIFIER_BOTTLE_D + 40.0 < HUMIDIFIER_X / 2.0);
        assert!(HUMIDIFIER_BOTTLE_Z < HUMIDIFIER_Z);
        assert!(WATER_JACKET_D > HUMIDIFIER_BOTTLE_D + 20.0);
        assert!(WATER_JACKET_Z < HUMIDIFIER_BOTTLE_Z);
        assert!(CONDENSER_X + HUMIDIFIER_X + 80.0 < BASE_X);
        assert_eq!(CONDENSATE_DRAINS, 2);
        assert!(CONDENSER_Z < PANEL_Z - 120.0);
    }

    #[test]
    fn service_isolation_and_sensor_counts_cover_external_paths() {
        assert_eq!(SENSOR_POCKETS, 4);
        assert_eq!(CALIBRATION_PORTS, 1);
        assert_eq!(RELIEF_EXHAUST_PATHS, 2);
        assert_eq!(
            SERVICE_ISOLATION_VALVES,
            GAS_CHANNELS + CULTURE_BOX_FEEDS + RELIEF_EXHAUST_PATHS + CALIBRATION_PORTS
        );
        assert!(
            isolation_valve_x(SERVICE_ISOLATION_VALVES - 1).abs() + 26.0 < ISOLATION_COVER_X / 2.0
        );
        assert!(sensor_pocket_x(0).abs() + SENSOR_POCKET_X / 2.0 < SENSOR_BANK_X / 2.0 - 48.0);
        assert!(
            sensor_pocket_x(SENSOR_POCKETS - 1).abs() + SENSOR_POCKET_X / 2.0
                < SENSOR_BANK_X / 2.0 - 48.0
        );
    }
}
