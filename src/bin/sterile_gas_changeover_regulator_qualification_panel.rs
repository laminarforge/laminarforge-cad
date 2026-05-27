use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Sterile gas changeover and regulator qualification panel for the closed
// culture utility skid.
//
// Intent:
// - Package bought CO2/O2/N2/air cylinders or bulk gas inputs, regulators,
//   MFCs, sterile filters, check valves, relief/exhaust, calibration gas
//   injection, and pressure sensing as serviceable envelopes.
// - Make dual-source changeover and cylinder lot evidence visible without
//   treating the printed/machined CAD as safety-critical pressure hardware.
// - Keep service isolation, leak test, calibration, and robot/operator
//   clearances explicit for reproducible closed-culture operation.

const OUTPUTS: [&str; 12] = [
    "output/sterile_gas_changeover_regulator_qualification_panel_base_leak_tray.stl",
    "output/sterile_gas_changeover_regulator_qualification_panel_cylinder_restraint_datum_rails.stl",
    "output/sterile_gas_changeover_regulator_qualification_panel_bulkhead_input_output_panel.stl",
    "output/sterile_gas_changeover_regulator_qualification_panel_regulator_mfc_envelopes.stl",
    "output/sterile_gas_changeover_regulator_qualification_panel_dual_source_changeover_bank.stl",
    "output/sterile_gas_changeover_regulator_qualification_panel_sterile_filter_check_valve_bank.stl",
    "output/sterile_gas_changeover_regulator_qualification_panel_relief_exhaust_calibration_bank.stl",
    "output/sterile_gas_changeover_regulator_qualification_panel_pressure_transducer_calibration_lands.stl",
    "output/sterile_gas_changeover_regulator_qualification_panel_barcode_cylinder_lot_scan_lands.stl",
    "output/sterile_gas_changeover_regulator_qualification_panel_leak_test_isolation_tag_panel.stl",
    "output/sterile_gas_changeover_regulator_qualification_panel_robot_operator_service_keepouts.stl",
    "output/sterile_gas_changeover_regulator_qualification_panel_assembly.stl",
];

const REQUIRED_FEATURE_GROUPS: [&str; 10] = [
    "cylinder_or_bulk_source_restraints",
    "purchased_regulator_envelopes",
    "mass_flow_controller_envelopes",
    "dual_source_changeover_placeholders",
    "sterile_filter_and_check_valve_bank",
    "relief_exhaust_route",
    "calibration_standard_gas_port",
    "pressure_transducer_calibration_lands",
    "barcode_cylinder_lot_scan_lands",
    "leak_test_and_service_isolation_tags",
];

const GAS_CHANNELS: usize = 4;
const GAS_NAMES: [&str; GAS_CHANNELS] = ["co2", "o2", "n2", "air"];
const SOURCES_PER_GAS: usize = 2;
const SOURCE_COUNT: usize = GAS_CHANNELS * SOURCES_PER_GAS;
const REGULATOR_ENVELOPES: usize = SOURCE_COUNT;
const MFC_COUNT: usize = GAS_CHANNELS;
const CHANGEOVER_VALVES: usize = GAS_CHANNELS;
const STERILE_FILTERS: usize = GAS_CHANNELS * 2;
const CHECK_VALVES: usize = SOURCE_COUNT;
const RELIEF_VALVES: usize = GAS_CHANNELS + 1;
const PRESSURE_TRANSDUCERS: usize = GAS_CHANNELS * 2;
const LEAK_TEST_PORTS: usize = SOURCE_COUNT;
const SERVICE_ISOLATION_TAGS: usize = SOURCE_COUNT + GAS_CHANNELS + 2;
const CYLINDER_BARCODE_LANDS: usize = SOURCE_COUNT;
const CHANGEOVER_EVIDENCE_WINDOWS: usize = GAS_CHANNELS;
const CALIBRATION_GAS_PORTS: usize = 1;
const BULK_GAS_INPUT_PORTS: usize = GAS_CHANNELS;
const QUALIFIED_OUTPUT_PORTS: usize = GAS_CHANNELS;

const DECK_X: f64 = 1680.0;
const DECK_Y: f64 = 940.0;
const DECK_Z: f64 = 24.0;
const PANEL_X: f64 = 1540.0;
const PANEL_Y: f64 = 34.0;
const PANEL_Z: f64 = 520.0;
const PANEL_CENTER_Y: f64 = DECK_Y / 2.0 - 68.0;
const PANEL_CENTER_Z: f64 = DECK_Z / 2.0 + PANEL_Z / 2.0;

const CHANNEL_PITCH_X: f64 = 340.0;
const SOURCE_ROW_PITCH_Y: f64 = 182.0;
const SOURCE_ROW_CENTER_Y: f64 = -238.0;
const CYLINDER_CRADLE_X: f64 = 212.0;
const CYLINDER_CRADLE_Y: f64 = 132.0;
const CYLINDER_RAIL_W: f64 = 18.0;
const CYLINDER_RAIL_Z: f64 = 42.0;
const CYLINDER_FOOT_CUP_D: f64 = 108.0;
const CYLINDER_STRAP_POST_Z: f64 = 188.0;

const REGULATOR_X: f64 = 104.0;
const REGULATOR_Y: f64 = 72.0;
const REGULATOR_Z: f64 = 96.0;
const MFC_X: f64 = 148.0;
const MFC_Y: f64 = 68.0;
const MFC_Z: f64 = 108.0;
const CHANGEOVER_X: f64 = 142.0;
const CHANGEOVER_Y: f64 = 76.0;
const CHANGEOVER_Z: f64 = 92.0;

const FILTER_BODY_D: f64 = 34.0;
const FILTER_BODY_LEN: f64 = 116.0;
const CHECK_VALVE_D: f64 = 22.0;
const CHECK_VALVE_LEN: f64 = 54.0;
const FILTER_BANK_X: f64 = 1390.0;
const FILTER_BANK_Y: f64 = 84.0;
const FILTER_BANK_Z: f64 = 112.0;

const SENSOR_BANK_X: f64 = 1350.0;
const SENSOR_BANK_Y: f64 = 82.0;
const SENSOR_BANK_Z: f64 = 104.0;
const SENSOR_POCKET_X: f64 = 70.0;
const SENSOR_POCKET_Z: f64 = 56.0;
const SENSOR_PAIR_PITCH_X: f64 = 78.0;

const LEAK_PANEL_X: f64 = 1450.0;
const LEAK_PANEL_Y: f64 = 72.0;
const LEAK_PANEL_Z: f64 = 94.0;
const TAG_LAND_X: f64 = 52.0;
const TAG_LAND_Z: f64 = 30.0;

const FRONT_ROBOT_CLEARANCE: f64 = 540.0;
const REAR_CYLINDER_SERVICE_CLEARANCE: f64 = 430.0;
const SIDE_REGULATOR_SERVICE_CLEARANCE: f64 = 260.0;
const TOP_CYLINDER_LIFT_CLEARANCE: f64 = 740.0;

fn main() {
    fs::create_dir_all("output").unwrap();

    let base = base_leak_tray();
    export(OUTPUTS[0], &base);

    let restraints = cylinder_restraint_datum_rails();
    export(OUTPUTS[1], &restraints);

    let bulkhead = bulkhead_input_output_panel();
    export(OUTPUTS[2], &bulkhead);

    let controls = regulator_mfc_envelopes();
    export(OUTPUTS[3], &controls);

    let changeover = dual_source_changeover_bank();
    export(OUTPUTS[4], &changeover);

    let filters = sterile_filter_check_valve_bank();
    export(OUTPUTS[5], &filters);

    let relief = relief_exhaust_calibration_bank();
    export(OUTPUTS[6], &relief);

    let sensors = pressure_transducer_calibration_lands();
    export(OUTPUTS[7], &sensors);

    let barcode = barcode_cylinder_lot_scan_lands();
    export(OUTPUTS[8], &barcode);

    let leak_tags = leak_test_isolation_tag_panel();
    export(OUTPUTS[9], &leak_tags);

    let keepouts = robot_operator_service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let assembly = base
        + restraints
        + bulkhead
        + controls
        + changeover
        + filters
        + relief
        + sensors
        + barcode
        + leak_tags
        + keepouts;
    export(OUTPUTS[11], &assembly);

    println!(
        "Sterile gas changeover/regulator qualification panel: {:.0} x {:.0}mm deck, {:.0}mm rear panel, {} gas channels ({:?}), {} A/B source restraints, {} purchased regulator envelopes, {} MFC envelopes, {} changeover valves, {} changeover evidence windows, {} sterile filter cartridges, {} check valves, {} relief valves, {} pressure transducer lands, {} leak-test ports, {} bulk gas input ports, {} qualified output ports, {} calibration gas port, {} cylinder barcode lands, and {} service isolation tags.",
        DECK_X,
        DECK_Y,
        PANEL_Z,
        GAS_CHANNELS,
        GAS_NAMES,
        SOURCE_COUNT,
        REGULATOR_ENVELOPES,
        MFC_COUNT,
        CHANGEOVER_VALVES,
        CHANGEOVER_EVIDENCE_WINDOWS,
        STERILE_FILTERS,
        CHECK_VALVES,
        RELIEF_VALVES,
        PRESSURE_TRANSDUCERS,
        LEAK_TEST_PORTS,
        BULK_GAS_INPUT_PORTS,
        QUALIFIED_OUTPUT_PORTS,
        CALIBRATION_GAS_PORTS,
        CYLINDER_BARCODE_LANDS,
        SERVICE_ISOLATION_TAGS
    );
    println!(
        "Service envelopes: {:.0}mm front robot/operator approach, {:.0}mm rear cylinder/bulk-gas access, {:.0}mm side regulator access, {:.0}mm top cylinder lift clearance, and {} named feature groups. CAD is packaging/interface geometry for bought certified gas hardware, not a pressure-vessel design.",
        FRONT_ROBOT_CLEARANCE,
        REAR_CYLINDER_SERVICE_CLEARANCE,
        SIDE_REGULATOR_SERVICE_CLEARANCE,
        TOP_CYLINDER_LIFT_CLEARANCE,
        REQUIRED_FEATURE_GROUPS.len()
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn base_leak_tray() -> Part {
    let deck = centered_cube(
        "sterile_gas_changeover_base_leak_tray_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    );
    let recessed_spill_pan = centered_cube(
        "sterile_gas_changeover_recessed_spill_pan",
        DECK_X - 132.0,
        DECK_Y - 162.0,
        9.0,
    )
    .translate(0.0, -28.0, DECK_Z / 2.0 - 4.0);
    let front_drain = centered_cylinder(
        "sterile_gas_changeover_front_low_point_drain",
        12.0 / 2.0,
        58.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 92.0, -(DECK_Y / 2.0 - 22.0), 0.0);
    let rear_panel_socket = centered_cube(
        "sterile_gas_changeover_rear_panel_socket",
        PANEL_X + 46.0,
        22.0,
        10.0,
    )
    .translate(0.0, PANEL_CENTER_Y, DECK_Z / 2.0 - 3.0);

    let mut mount_holes = Part::empty("sterile_gas_changeover_base_mount_holes");
    for (i, (x, y)) in base_mount_points().iter().enumerate() {
        let hole = centered_cylinder(
            format!("sterile_gas_changeover_m8_mount_clearance_{i}"),
            8.4 / 2.0,
            DECK_Z + 4.0,
            28,
        )
        .translate(*x, *y, 0.0);
        let slot = centered_cube(
            format!("sterile_gas_changeover_m8_mount_service_slot_{i}"),
            34.0,
            8.8,
            DECK_Z + 4.0,
        )
        .translate(*x, *y, 0.0);
        mount_holes = mount_holes + hole + slot;
    }

    let gas_passage_channel = centered_cube(
        "sterile_gas_changeover_front_to_rear_hose_passage_channel",
        DECK_X - 260.0,
        42.0,
        8.0,
    )
    .translate(0.0, -40.0, DECK_Z / 2.0 - 4.0);

    deck - recessed_spill_pan - front_drain - rear_panel_socket - mount_holes - gas_passage_channel
        + perimeter_lip()
        + leveler_feet()
        + cylinder_row_datums()
}

fn perimeter_lip() -> Part {
    let front = centered_cube(
        "sterile_gas_changeover_front_spill_lip",
        DECK_X - 92.0,
        18.0,
        32.0,
    )
    .translate(0.0, -(DECK_Y / 2.0 - 26.0), DECK_Z / 2.0 + 16.0);
    let rear = centered_cube(
        "sterile_gas_changeover_rear_panel_heel_lip",
        DECK_X - 92.0,
        18.0,
        36.0,
    )
    .translate(0.0, PANEL_CENTER_Y - 30.0, DECK_Z / 2.0 + 18.0);
    let left = centered_cube(
        "sterile_gas_changeover_left_spill_lip",
        20.0,
        DECK_Y - 120.0,
        34.0,
    )
    .translate(-(DECK_X / 2.0 - 36.0), -24.0, DECK_Z / 2.0 + 17.0);
    let right = centered_cube(
        "sterile_gas_changeover_right_spill_lip",
        20.0,
        DECK_Y - 120.0,
        34.0,
    )
    .translate(DECK_X / 2.0 - 36.0, -24.0, DECK_Z / 2.0 + 17.0);

    front + rear + left + right
}

fn leveler_feet() -> Part {
    let mut feet = Part::empty("sterile_gas_changeover_leveler_feet");
    for (i, (x, y)) in [
        (-(DECK_X / 2.0 - 70.0), -(DECK_Y / 2.0 - 62.0)),
        (DECK_X / 2.0 - 70.0, -(DECK_Y / 2.0 - 62.0)),
        (-(DECK_X / 2.0 - 70.0), DECK_Y / 2.0 - 62.0),
        (DECK_X / 2.0 - 70.0, DECK_Y / 2.0 - 62.0),
        (-280.0, -(DECK_Y / 2.0 - 62.0)),
        (280.0, -(DECK_Y / 2.0 - 62.0)),
        (-280.0, DECK_Y / 2.0 - 62.0),
        (280.0, DECK_Y / 2.0 - 62.0),
    ]
    .iter()
    .enumerate()
    {
        let pad = centered_cylinder(
            format!("sterile_gas_changeover_leveling_pad_{i}"),
            24.0,
            12.0,
            36,
        )
        .translate(*x, *y, -(DECK_Z / 2.0 + 6.0));
        let screw_clearance = centered_cylinder(
            format!("sterile_gas_changeover_leveling_screw_clearance_{i}"),
            9.0 / 2.0,
            18.0,
            24,
        )
        .translate(*x, *y, -(DECK_Z / 2.0 + 6.0));
        feet = feet + (pad - screw_clearance);
    }
    feet
}

fn cylinder_row_datums() -> Part {
    let mut datums = Part::empty("sterile_gas_changeover_source_row_datums");
    for source in 0..SOURCES_PER_GAS {
        let y = source_row_y(source);
        datums = datums
            + centered_cube(
                format!(
                    "sterile_gas_changeover_source_{}_front_datum_rail",
                    source_label(source)
                ),
                DECK_X - 238.0,
                14.0,
                18.0,
            )
            .translate(0.0, y - CYLINDER_CRADLE_Y / 2.0 - 18.0, DECK_Z / 2.0 + 9.0)
            + centered_cube(
                format!(
                    "sterile_gas_changeover_source_{}_rear_datum_rail",
                    source_label(source)
                ),
                DECK_X - 238.0,
                14.0,
                18.0,
            )
            .translate(0.0, y + CYLINDER_CRADLE_Y / 2.0 + 18.0, DECK_Z / 2.0 + 9.0);
    }
    datums
}

fn cylinder_restraint_datum_rails() -> Part {
    let mut assembly = Part::empty("sterile_gas_changeover_cylinder_restraint_datum_rails");

    for gas in 0..GAS_CHANNELS {
        let x = gas_channel_x(gas);
        for source in 0..SOURCES_PER_GAS {
            let y = source_row_y(source);
            let label = source_label(source);
            let rail_left = centered_cube(
                format!(
                    "sterile_gas_changeover_{}_source_{}_left_cylinder_rail",
                    GAS_NAMES[gas], label
                ),
                CYLINDER_RAIL_W,
                CYLINDER_CRADLE_Y,
                CYLINDER_RAIL_Z,
            )
            .translate(
                x - CYLINDER_CRADLE_X / 2.0 + CYLINDER_RAIL_W / 2.0,
                y,
                DECK_Z / 2.0 + CYLINDER_RAIL_Z / 2.0,
            );
            let rail_right = centered_cube(
                format!(
                    "sterile_gas_changeover_{}_source_{}_right_cylinder_rail",
                    GAS_NAMES[gas], label
                ),
                CYLINDER_RAIL_W,
                CYLINDER_CRADLE_Y,
                CYLINDER_RAIL_Z,
            )
            .translate(
                x + CYLINDER_CRADLE_X / 2.0 - CYLINDER_RAIL_W / 2.0,
                y,
                DECK_Z / 2.0 + CYLINDER_RAIL_Z / 2.0,
            );
            let front_stop = centered_cube(
                format!(
                    "sterile_gas_changeover_{}_source_{}_front_cylinder_stop",
                    GAS_NAMES[gas], label
                ),
                CYLINDER_CRADLE_X,
                18.0,
                48.0,
            )
            .translate(x, y - CYLINDER_CRADLE_Y / 2.0 + 9.0, DECK_Z / 2.0 + 24.0);
            let rear_stop = centered_cube(
                format!(
                    "sterile_gas_changeover_{}_source_{}_rear_cylinder_stop",
                    GAS_NAMES[gas], label
                ),
                CYLINDER_CRADLE_X,
                18.0,
                74.0,
            )
            .translate(x, y + CYLINDER_CRADLE_Y / 2.0 - 9.0, DECK_Z / 2.0 + 37.0);
            let foot_cup = centered_cylinder(
                format!(
                    "sterile_gas_changeover_{}_source_{}_cylinder_foot_cup",
                    GAS_NAMES[gas], label
                ),
                CYLINDER_FOOT_CUP_D / 2.0,
                20.0,
                56,
            )
            .translate(x, y - 18.0, DECK_Z / 2.0 + 10.0);
            let foot_clearance = centered_cylinder(
                format!(
                    "sterile_gas_changeover_{}_source_{}_cylinder_foot_clearance",
                    GAS_NAMES[gas], label
                ),
                CYLINDER_FOOT_CUP_D / 2.0 - 12.0,
                22.0,
                56,
            )
            .translate(x, y - 18.0, DECK_Z / 2.0 + 12.0);
            let strap_post_left = cylinder_strap_post(gas, source, "left").translate(
                x - CYLINDER_CRADLE_X / 2.0 + 18.0,
                y + CYLINDER_CRADLE_Y / 2.0 - 10.0,
                DECK_Z / 2.0 + CYLINDER_STRAP_POST_Z / 2.0,
            );
            let strap_post_right = cylinder_strap_post(gas, source, "right").translate(
                x + CYLINDER_CRADLE_X / 2.0 - 18.0,
                y + CYLINDER_CRADLE_Y / 2.0 - 10.0,
                DECK_Z / 2.0 + CYLINDER_STRAP_POST_Z / 2.0,
            );
            let chain = centered_cube(
                format!(
                    "sterile_gas_changeover_{}_source_{}_restraint_chain_envelope",
                    GAS_NAMES[gas], label
                ),
                CYLINDER_CRADLE_X - 54.0,
                12.0,
                18.0,
            )
            .translate(x, y + CYLINDER_CRADLE_Y / 2.0 - 20.0, DECK_Z / 2.0 + 128.0);

            assembly = assembly
                + rail_left
                + rail_right
                + front_stop
                + rear_stop
                + (foot_cup - foot_clearance)
                + strap_post_left
                + strap_post_right
                + chain;
        }
    }

    assembly
}

fn cylinder_strap_post(gas: usize, source: usize, side: &str) -> Part {
    let post = centered_cube(
        format!(
            "sterile_gas_changeover_{}_source_{}_{}_strap_post",
            GAS_NAMES[gas],
            source_label(source),
            side
        ),
        22.0,
        18.0,
        CYLINDER_STRAP_POST_Z,
    );
    let pin_hole = centered_cylinder(
        format!(
            "sterile_gas_changeover_{}_source_{}_{}_strap_pin_hole",
            GAS_NAMES[gas],
            source_label(source),
            side
        ),
        5.0 / 2.0,
        30.0,
        20,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, 0.0, 46.0);
    post - pin_hole
}

fn bulkhead_input_output_panel() -> Part {
    let panel = centered_cube(
        "sterile_gas_changeover_bulkhead_input_output_panel",
        PANEL_X,
        PANEL_Y,
        PANEL_Z,
    )
    .translate(0.0, PANEL_CENTER_Y, PANEL_CENTER_Z);

    let mut cuts = Part::empty("sterile_gas_changeover_bulkhead_cuts");
    let mut lands = Part::empty("sterile_gas_changeover_bulkhead_label_lands");

    for gas in 0..GAS_CHANNELS {
        let x = gas_channel_x(gas);
        for source in 0..SOURCES_PER_GAS {
            let source_x = x + source_offset_x(source);
            let z = PANEL_CENTER_Z + 170.0;
            cuts = cuts
                + panel_port_cut(
                    format!(
                        "sterile_gas_changeover_{}_source_{}_cylinder_input_bulkhead",
                        GAS_NAMES[gas],
                        source_label(source)
                    ),
                    14.0,
                )
                .translate(source_x, PANEL_CENTER_Y, z);
            lands = lands
                + panel_label_land(
                    format!(
                        "sterile_gas_changeover_{}_source_{}_input_label_land",
                        GAS_NAMES[gas],
                        source_label(source)
                    ),
                    78.0,
                    26.0,
                )
                .translate(
                    source_x,
                    PANEL_CENTER_Y - PANEL_Y / 2.0 - 5.0,
                    z + 36.0,
                );
        }

        cuts = cuts
            + panel_port_cut(
                format!(
                    "sterile_gas_changeover_{}_bulk_gas_input_port",
                    GAS_NAMES[gas]
                ),
                12.7,
            )
            .translate(x, PANEL_CENTER_Y, PANEL_CENTER_Z + 78.0)
            + panel_port_cut(
                format!(
                    "sterile_gas_changeover_{}_qualified_output_port",
                    GAS_NAMES[gas]
                ),
                10.4,
            )
            .translate(x, PANEL_CENTER_Y, PANEL_CENTER_Z - 186.0);
        lands = lands
            + panel_label_land(
                format!(
                    "sterile_gas_changeover_{}_bulk_input_label_land",
                    GAS_NAMES[gas]
                ),
                96.0,
                26.0,
            )
            .translate(
                x,
                PANEL_CENTER_Y - PANEL_Y / 2.0 - 5.0,
                PANEL_CENTER_Z + 112.0,
            )
            + panel_label_land(
                format!(
                    "sterile_gas_changeover_{}_qualified_output_label_land",
                    GAS_NAMES[gas]
                ),
                114.0,
                26.0,
            )
            .translate(
                x,
                PANEL_CENTER_Y - PANEL_Y / 2.0 - 5.0,
                PANEL_CENTER_Z - 222.0,
            );
    }

    let calibration_cut = panel_port_cut(
        "sterile_gas_changeover_calibration_standard_gas_bulkhead",
        6.4,
    )
    .translate(
        -(PANEL_X / 2.0 - 96.0),
        PANEL_CENTER_Y,
        PANEL_CENTER_Z - 38.0,
    );
    let relief_cut = panel_port_cut("sterile_gas_changeover_relief_exhaust_bulkhead", 18.0)
        .translate(PANEL_X / 2.0 - 98.0, PANEL_CENTER_Y, PANEL_CENTER_Z - 38.0);
    let data_pass = centered_cube(
        "sterile_gas_changeover_signal_cable_pass_through",
        126.0,
        PANEL_Y + 10.0,
        54.0,
    )
    .translate(PANEL_X / 2.0 - 118.0, PANEL_CENTER_Y, PANEL_CENTER_Z + 86.0);

    let gasket = rectangular_gasket_land(
        "sterile_gas_changeover_bulkhead_panel_outer_gasket",
        PANEL_X - 54.0,
        PANEL_Z - 54.0,
    )
    .translate(0.0, PANEL_CENTER_Y - PANEL_Y / 2.0 - 4.0, PANEL_CENTER_Z);

    panel - cuts - calibration_cut - relief_cut - data_pass
        + lands
        + gasket
        + panel_section_labels()
}

fn panel_port_cut(name: impl Into<String>, diameter: f64) -> Part {
    centered_cylinder(name, diameter / 2.0, PANEL_Y + 12.0, 28).rotate(90.0, 0.0, 0.0)
}

fn panel_label_land(name: impl Into<String>, x: f64, z: f64) -> Part {
    centered_cube(name, x, 4.0, z)
}

fn rectangular_gasket_land(name: &str, x: f64, z: f64) -> Part {
    let top = centered_cube(format!("{name}_top"), x, 5.0, 9.0).translate(0.0, 0.0, z / 2.0);
    let bottom = centered_cube(format!("{name}_bottom"), x, 5.0, 9.0).translate(0.0, 0.0, -z / 2.0);
    let left = centered_cube(format!("{name}_left"), 9.0, 5.0, z).translate(-x / 2.0, 0.0, 0.0);
    let right = centered_cube(format!("{name}_right"), 9.0, 5.0, z).translate(x / 2.0, 0.0, 0.0);
    top + bottom + left + right
}

fn panel_section_labels() -> Part {
    centered_cube(
        "sterile_gas_changeover_panel_source_input_label_strip",
        PANEL_X - 142.0,
        6.0,
        18.0,
    )
    .translate(
        0.0,
        PANEL_CENTER_Y - PANEL_Y / 2.0 - 9.0,
        PANEL_CENTER_Z + 232.0,
    ) + centered_cube(
        "sterile_gas_changeover_panel_qualified_output_label_strip",
        PANEL_X - 142.0,
        6.0,
        18.0,
    )
    .translate(
        0.0,
        PANEL_CENTER_Y - PANEL_Y / 2.0 - 9.0,
        PANEL_CENTER_Z - 248.0,
    )
}

fn regulator_mfc_envelopes() -> Part {
    let backer = centered_cube(
        "sterile_gas_changeover_regulator_mfc_backer_plate",
        PANEL_X - 138.0,
        14.0,
        300.0,
    )
    .translate(0.0, PANEL_CENTER_Y - 64.0, PANEL_CENTER_Z + 48.0);
    let mut controls = Part::empty("sterile_gas_changeover_regulator_mfc_envelopes");

    for gas in 0..GAS_CHANNELS {
        let x = gas_channel_x(gas);
        for source in 0..SOURCES_PER_GAS {
            controls = controls
                + regulator_placeholder(gas, source).translate(
                    x + source_offset_x(source),
                    PANEL_CENTER_Y - 104.0,
                    PANEL_CENTER_Z + 154.0,
                );
        }
        controls = controls
            + mfc_placeholder(gas).translate(x, PANEL_CENTER_Y - 108.0, PANEL_CENTER_Z + 2.0)
            + outlet_pressure_regulator_trim(gas).translate(
                x,
                PANEL_CENTER_Y - 104.0,
                PANEL_CENTER_Z - 120.0,
            );
    }

    backer + controls
}

fn regulator_placeholder(gas: usize, source: usize) -> Part {
    let name = format!(
        "sterile_gas_changeover_{}_source_{}_purchased_regulator",
        GAS_NAMES[gas],
        source_label(source)
    );
    let body = centered_cube(
        format!("{name}_body_envelope"),
        REGULATOR_X,
        REGULATOR_Y,
        REGULATOR_Z,
    );
    let gauge = centered_cylinder(format!("{name}_face_gauge_envelope"), 28.0, 14.0, 40)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, -(REGULATOR_Y / 2.0 + 8.0), 16.0);
    let knob = centered_cylinder(format!("{name}_adjustment_knob_envelope"), 18.0, 24.0, 32)
        .translate(0.0, 0.0, REGULATOR_Z / 2.0 + 12.0);
    let inlet_bore = centered_cylinder(
        format!("{name}_inlet_axis_clearance"),
        5.0,
        REGULATOR_X + 16.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0);
    let relief_boss = centered_cylinder(
        format!("{name}_built_in_relief_boss_envelope"),
        9.0,
        18.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(REGULATOR_X / 2.0 + 8.0, -12.0, -22.0);
    body + gauge + knob + relief_boss - inlet_bore
}

fn mfc_placeholder(gas: usize) -> Part {
    let name = format!(
        "sterile_gas_changeover_{}_mass_flow_controller",
        GAS_NAMES[gas]
    );
    let body = centered_cube(format!("{name}_body_envelope"), MFC_X, MFC_Y, MFC_Z);
    let display_recess = centered_cube(format!("{name}_display_recess"), 78.0, 10.0, 34.0)
        .translate(0.0, -(MFC_Y / 2.0 + 1.0), 20.0);
    let flow_bore = centered_cylinder(
        format!("{name}_flow_axis_clearance"),
        5.0 / 2.0,
        MFC_X + 16.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, -28.0);
    let left_mount = centered_cube(format!("{name}_left_mount_slot"), 9.0, MFC_Y + 6.0, 18.0)
        .translate(-MFC_X / 2.0 + 17.0, 0.0, MFC_Z / 2.0 - 18.0);
    let right_mount = centered_cube(format!("{name}_right_mount_slot"), 9.0, MFC_Y + 6.0, 18.0)
        .translate(MFC_X / 2.0 - 17.0, 0.0, MFC_Z / 2.0 - 18.0);
    body - display_recess - flow_bore - left_mount - right_mount
}

fn outlet_pressure_regulator_trim(gas: usize) -> Part {
    let name = format!(
        "sterile_gas_changeover_{}_low_pressure_trim_regulator",
        GAS_NAMES[gas]
    );
    let body = centered_cube(format!("{name}_envelope"), 118.0, 58.0, 54.0);
    let gauge = centered_cylinder(format!("{name}_gauge_envelope"), 20.0, 12.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, -35.0, 4.0);
    let bore = centered_cylinder(format!("{name}_flow_axis_clearance"), 4.0, 134.0, 24)
        .rotate(0.0, 90.0, 0.0);
    body + gauge - bore
}

fn dual_source_changeover_bank() -> Part {
    let mut bank = Part::empty("sterile_gas_changeover_dual_source_changeover_bank");
    let manifold = centered_cube(
        "sterile_gas_changeover_changeover_manifold_spine",
        PANEL_X - 180.0,
        56.0,
        66.0,
    )
    .translate(0.0, PANEL_CENTER_Y - 132.0, PANEL_CENTER_Z + 66.0);

    for gas in 0..GAS_CHANNELS {
        let x = gas_channel_x(gas);
        let body = centered_cube(
            format!(
                "sterile_gas_changeover_{}_ab_changeover_valve_envelope",
                GAS_NAMES[gas]
            ),
            CHANGEOVER_X,
            CHANGEOVER_Y,
            CHANGEOVER_Z,
        )
        .translate(x, PANEL_CENTER_Y - 146.0, PANEL_CENTER_Z + 76.0);
        let handle = centered_cube(
            format!(
                "sterile_gas_changeover_{}_manual_override_handle_envelope",
                GAS_NAMES[gas]
            ),
            26.0,
            72.0,
            18.0,
        )
        .translate(x, PANEL_CENTER_Y - 196.0, PANEL_CENTER_Z + 126.0);
        let evidence_window = centered_cube(
            format!(
                "sterile_gas_changeover_{}_changeover_evidence_window",
                GAS_NAMES[gas]
            ),
            86.0,
            8.0,
            26.0,
        )
        .translate(x, PANEL_CENTER_Y - 188.0, PANEL_CENTER_Z + 32.0);
        let left_inlet = centered_cylinder(
            format!(
                "sterile_gas_changeover_{}_source_a_changeover_bore",
                GAS_NAMES[gas]
            ),
            4.0,
            CHANGEOVER_X + 16.0,
            24,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(x - 34.0, PANEL_CENTER_Y - 146.0, PANEL_CENTER_Z + 82.0);
        let right_inlet = centered_cylinder(
            format!(
                "sterile_gas_changeover_{}_source_b_changeover_bore",
                GAS_NAMES[gas]
            ),
            4.0,
            CHANGEOVER_X + 16.0,
            24,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(x + 34.0, PANEL_CENTER_Y - 146.0, PANEL_CENTER_Z + 58.0);
        bank = bank + (body - left_inlet - right_inlet) + handle + evidence_window;
    }

    manifold + bank
}

fn sterile_filter_check_valve_bank() -> Part {
    let backer = centered_cube(
        "sterile_gas_changeover_filter_check_valve_backer",
        FILTER_BANK_X,
        FILTER_BANK_Y,
        FILTER_BANK_Z,
    )
    .translate(0.0, PANEL_CENTER_Y - 142.0, PANEL_CENTER_Z - 36.0);
    let main_bore = centered_cylinder(
        "sterile_gas_changeover_filter_bank_common_bore",
        8.0 / 2.0,
        FILTER_BANK_X - 82.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, PANEL_CENTER_Y - 142.0, PANEL_CENTER_Z - 28.0);

    let mut parts = Part::empty("sterile_gas_changeover_filter_check_valve_parts");
    let mut branch_bores = Part::empty("sterile_gas_changeover_filter_bank_branch_bores");

    for gas in 0..GAS_CHANNELS {
        let x = gas_channel_x(gas);
        for filter_index in 0..2 {
            let local_x = x + if filter_index == 0 { -56.0 } else { 56.0 };
            let z = PANEL_CENTER_Z - 14.0 - filter_index as f64 * 46.0;
            let filter = centered_cylinder(
                format!(
                    "sterile_gas_changeover_{}_sterile_filter_{}_cartridge_envelope",
                    GAS_NAMES[gas], filter_index
                ),
                FILTER_BODY_D / 2.0,
                FILTER_BODY_LEN,
                36,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(local_x, PANEL_CENTER_Y - 206.0, z);
            let clamp = centered_cube(
                format!(
                    "sterile_gas_changeover_{}_sterile_filter_{}_clamp",
                    GAS_NAMES[gas], filter_index
                ),
                58.0,
                16.0,
                22.0,
            )
            .translate(local_x, PANEL_CENTER_Y - 150.0, z);
            parts = parts + filter + clamp;
            branch_bores = branch_bores
                + centered_cylinder(
                    format!(
                        "sterile_gas_changeover_{}_sterile_filter_{}_panel_bore",
                        GAS_NAMES[gas], filter_index
                    ),
                    5.0,
                    FILTER_BANK_Y + 16.0,
                    24,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(local_x, PANEL_CENTER_Y - 142.0, z);
        }

        for source in 0..SOURCES_PER_GAS {
            let check = centered_cylinder(
                format!(
                    "sterile_gas_changeover_{}_source_{}_check_valve_envelope",
                    GAS_NAMES[gas],
                    source_label(source)
                ),
                CHECK_VALVE_D / 2.0,
                CHECK_VALVE_LEN,
                28,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(
                x + source_offset_x(source),
                PANEL_CENTER_Y - 190.0,
                PANEL_CENTER_Z - 102.0,
            );
            let arrow = centered_cube(
                format!(
                    "sterile_gas_changeover_{}_source_{}_flow_direction_land",
                    GAS_NAMES[gas],
                    source_label(source)
                ),
                36.0,
                8.0,
                10.0,
            )
            .translate(
                x + source_offset_x(source),
                PANEL_CENTER_Y - 224.0,
                PANEL_CENTER_Z - 102.0,
            );
            parts = parts + check + arrow;
        }
    }

    backer - main_bore - branch_bores + parts
}

fn relief_exhaust_calibration_bank() -> Part {
    let manifold = centered_cube(
        "sterile_gas_changeover_relief_exhaust_manifold_body",
        PANEL_X - 260.0,
        56.0,
        74.0,
    )
    .translate(0.0, PANEL_CENTER_Y - 128.0, PANEL_CENTER_Z - 214.0);
    let manifold_bore = centered_cylinder(
        "sterile_gas_changeover_relief_exhaust_common_bore",
        9.5 / 2.0,
        PANEL_X - 224.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, PANEL_CENTER_Y - 128.0, PANEL_CENTER_Z - 214.0);

    let mut parts = Part::empty("sterile_gas_changeover_relief_exhaust_calibration_parts");
    for gas in 0..GAS_CHANNELS {
        let relief = relief_valve_placeholder(GAS_NAMES[gas]).translate(
            gas_channel_x(gas),
            PANEL_CENTER_Y - 174.0,
            PANEL_CENTER_Z - 172.0,
        );
        let exhaust_branch = centered_cylinder(
            format!(
                "sterile_gas_changeover_{}_relief_branch_bore",
                GAS_NAMES[gas]
            ),
            4.0,
            68.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            gas_channel_x(gas),
            PANEL_CENTER_Y - 128.0,
            PANEL_CENTER_Z - 190.0,
        );
        parts = parts + relief - exhaust_branch;
    }

    let overpressure_relief = relief_valve_placeholder("shared_overpressure").translate(
        PANEL_X / 2.0 - 174.0,
        PANEL_CENTER_Y - 174.0,
        PANEL_CENTER_Z - 214.0,
    );
    let exhaust_muffler = centered_cylinder(
        "sterile_gas_changeover_exhaust_muffler_envelope",
        26.0,
        128.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        PANEL_X / 2.0 - 90.0,
        PANEL_CENTER_Y - 210.0,
        PANEL_CENTER_Z - 214.0,
    );
    let calibration_port = centered_cylinder(
        "sterile_gas_changeover_calibration_standard_quick_connect",
        11.0,
        42.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        -(PANEL_X / 2.0 - 122.0),
        PANEL_CENTER_Y - 168.0,
        PANEL_CENTER_Z - 214.0,
    );
    let calibration_check = centered_cylinder(
        "sterile_gas_changeover_calibration_injection_check_valve",
        10.0,
        46.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(
        -(PANEL_X / 2.0 - 188.0),
        PANEL_CENTER_Y - 168.0,
        PANEL_CENTER_Z - 214.0,
    );
    let purge_capture = centered_cube(
        "sterile_gas_changeover_calibration_purge_capture_land",
        118.0,
        12.0,
        38.0,
    )
    .translate(
        -(PANEL_X / 2.0 - 154.0),
        PANEL_CENTER_Y - 208.0,
        PANEL_CENTER_Z - 254.0,
    );

    (manifold - manifold_bore)
        + parts
        + overpressure_relief
        + exhaust_muffler
        + calibration_port
        + calibration_check
        + purge_capture
}

fn relief_valve_placeholder(name: &str) -> Part {
    let body = centered_cylinder(
        format!("sterile_gas_changeover_{name}_relief_valve_body"),
        17.0,
        48.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0);
    let stem = centered_cylinder(
        format!("sterile_gas_changeover_{name}_relief_valve_stem"),
        8.0,
        28.0,
        24,
    )
    .translate(0.0, 0.0, 24.0);
    let cap = centered_cylinder(
        format!("sterile_gas_changeover_{name}_relief_valve_cap"),
        15.0,
        12.0,
        28,
    )
    .translate(0.0, 0.0, 44.0);
    body + stem + cap
}

fn pressure_transducer_calibration_lands() -> Part {
    let plate = centered_cube(
        "sterile_gas_changeover_pressure_transducer_calibration_plate",
        SENSOR_BANK_X,
        SENSOR_BANK_Y,
        SENSOR_BANK_Z,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + 110.0,
        DECK_Z / 2.0 + SENSOR_BANK_Z / 2.0,
    );

    let mut cuts = Part::empty("sterile_gas_changeover_pressure_sensor_pocket_cuts");
    let mut lands = Part::empty("sterile_gas_changeover_pressure_sensor_lands");
    let sensor_y = -DECK_Y / 2.0 + 110.0;
    let sensor_z = DECK_Z / 2.0 + SENSOR_BANK_Z / 2.0;

    for gas in 0..GAS_CHANNELS {
        let x = gas_channel_x(gas);
        for (stage, z_offset) in [22.0, -34.0].iter().enumerate() {
            let sensor_x = x + (stage as f64 - 0.5) * SENSOR_PAIR_PITCH_X;
            cuts = cuts
                + centered_cube(
                    format!(
                        "sterile_gas_changeover_{}_pressure_transducer_stage_{stage}_pocket",
                        GAS_NAMES[gas]
                    ),
                    SENSOR_POCKET_X,
                    SENSOR_BANK_Y + 8.0,
                    SENSOR_POCKET_Z,
                )
                .translate(sensor_x, sensor_y, sensor_z + *z_offset);
            lands = lands
                + centered_cube(
                    format!(
                        "sterile_gas_changeover_{}_pressure_transducer_stage_{stage}_face_land",
                        GAS_NAMES[gas]
                    ),
                    SENSOR_POCKET_X + 16.0,
                    8.0,
                    SENSOR_POCKET_Z + 14.0,
                )
                .translate(
                    sensor_x,
                    sensor_y - SENSOR_BANK_Y / 2.0 - 5.0,
                    sensor_z + *z_offset,
                );
        }
        lands = lands
            + centered_cube(
                format!(
                    "sterile_gas_changeover_{}_calibration_gauge_land",
                    GAS_NAMES[gas]
                ),
                98.0,
                8.0,
                32.0,
            )
            .translate(x, sensor_y - SENSOR_BANK_Y / 2.0 - 6.0, sensor_z + 58.0);
    }

    let reference_sensor_socket = centered_cube(
        "sterile_gas_changeover_reference_pressure_meter_socket",
        142.0,
        SENSOR_BANK_Y + 8.0,
        64.0,
    )
    .translate(PANEL_X / 2.0 - 174.0, sensor_y, sensor_z - 2.0);
    let calibration_gas_token_slot = centered_cube(
        "sterile_gas_changeover_calibration_gas_certificate_token_slot",
        164.0,
        8.0,
        34.0,
    )
    .translate(
        -(PANEL_X / 2.0 - 174.0),
        sensor_y - SENSOR_BANK_Y / 2.0 - 6.0,
        sensor_z - 2.0,
    );

    plate - cuts - reference_sensor_socket + lands + calibration_gas_token_slot
}

fn barcode_cylinder_lot_scan_lands() -> Part {
    let mut lands = Part::empty("sterile_gas_changeover_barcode_cylinder_lot_scan_lands");
    for gas in 0..GAS_CHANNELS {
        let x = gas_channel_x(gas);
        for source in 0..SOURCES_PER_GAS {
            let y = source_row_y(source) - CYLINDER_CRADLE_Y / 2.0 - 48.0;
            lands = lands
                + centered_cube(
                    format!(
                        "sterile_gas_changeover_{}_source_{}_cylinder_barcode_scan_land",
                        GAS_NAMES[gas],
                        source_label(source)
                    ),
                    118.0,
                    6.0,
                    42.0,
                )
                .translate(x, y, DECK_Z / 2.0 + 44.0)
                + centered_cube(
                    format!(
                        "sterile_gas_changeover_{}_source_{}_rfid_lot_card_slot",
                        GAS_NAMES[gas],
                        source_label(source)
                    ),
                    82.0,
                    6.0,
                    28.0,
                )
                .translate(x, y - 14.0, DECK_Z / 2.0 + 18.0);
        }

        lands = lands
            + centered_cube(
                format!(
                    "sterile_gas_changeover_{}_changeover_evidence_label_land",
                    GAS_NAMES[gas]
                ),
                148.0,
                6.0,
                36.0,
            )
            .translate(
                gas_channel_x(gas),
                PANEL_CENTER_Y - 216.0,
                PANEL_CENTER_Z + 118.0,
            );
    }

    let run_record_strip = centered_cube(
        "sterile_gas_changeover_run_record_scan_strip",
        DECK_X - 260.0,
        8.0,
        34.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 34.0, DECK_Z / 2.0 + 42.0);
    lands + run_record_strip
}

fn leak_test_isolation_tag_panel() -> Part {
    let panel_y = -DECK_Y / 2.0 + 222.0;
    let panel = centered_cube(
        "sterile_gas_changeover_leak_test_isolation_tag_panel",
        LEAK_PANEL_X,
        LEAK_PANEL_Y,
        LEAK_PANEL_Z,
    )
    .translate(0.0, panel_y, DECK_Z / 2.0 + LEAK_PANEL_Z / 2.0);
    let mut cuts = Part::empty("sterile_gas_changeover_leak_test_port_cuts");
    let mut ports = Part::empty("sterile_gas_changeover_leak_test_port_lands");
    let mut tags = Part::empty("sterile_gas_changeover_service_isolation_tag_lands");

    for gas in 0..GAS_CHANNELS {
        for source in 0..SOURCES_PER_GAS {
            let x = gas_channel_x(gas) + source_offset_x(source);
            let port_z = DECK_Z / 2.0 + 64.0;
            cuts = cuts
                + centered_cylinder(
                    format!(
                        "sterile_gas_changeover_{}_source_{}_leak_test_bore",
                        GAS_NAMES[gas],
                        source_label(source)
                    ),
                    5.0,
                    LEAK_PANEL_Y + 12.0,
                    24,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(x, panel_y, port_z);
            ports = ports
                + centered_cylinder(
                    format!(
                        "sterile_gas_changeover_{}_source_{}_leak_test_quick_connect_land",
                        GAS_NAMES[gas],
                        source_label(source)
                    ),
                    14.0,
                    10.0,
                    28,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(x, panel_y - LEAK_PANEL_Y / 2.0 - 5.0, port_z);
            tags = tags
                + centered_cube(
                    format!(
                        "sterile_gas_changeover_{}_source_{}_service_isolation_tag_land",
                        GAS_NAMES[gas],
                        source_label(source)
                    ),
                    TAG_LAND_X,
                    6.0,
                    TAG_LAND_Z,
                )
                .translate(
                    x,
                    panel_y - LEAK_PANEL_Y / 2.0 - 8.0,
                    DECK_Z / 2.0 + 104.0,
                );
        }

        tags = tags
            + centered_cube(
                format!(
                    "sterile_gas_changeover_{}_qualified_output_isolation_tag",
                    GAS_NAMES[gas]
                ),
                TAG_LAND_X + 22.0,
                6.0,
                TAG_LAND_Z,
            )
            .translate(
                gas_channel_x(gas),
                panel_y - LEAK_PANEL_Y / 2.0 - 8.0,
                DECK_Z / 2.0 + 28.0,
            );
    }

    let relief_tag = centered_cube(
        "sterile_gas_changeover_relief_exhaust_service_tag_land",
        116.0,
        6.0,
        30.0,
    )
    .translate(
        PANEL_X / 2.0 - 150.0,
        panel_y - LEAK_PANEL_Y / 2.0 - 8.0,
        DECK_Z / 2.0 + 28.0,
    );
    let calibration_tag = centered_cube(
        "sterile_gas_changeover_calibration_port_service_tag_land",
        116.0,
        6.0,
        30.0,
    )
    .translate(
        -(PANEL_X / 2.0 - 150.0),
        panel_y - LEAK_PANEL_Y / 2.0 - 8.0,
        DECK_Z / 2.0 + 28.0,
    );

    panel - cuts + ports + tags + relief_tag + calibration_tag
}

fn robot_operator_service_keepouts() -> Part {
    let front_robot = centered_cube(
        "sterile_gas_changeover_front_robot_operator_approach_keepout",
        DECK_X - 260.0,
        FRONT_ROBOT_CLEARANCE,
        18.0,
    )
    .translate(
        0.0,
        -(DECK_Y / 2.0 + FRONT_ROBOT_CLEARANCE / 2.0),
        DECK_Z / 2.0 + 9.0,
    );
    let rear_cylinder = centered_cube(
        "sterile_gas_changeover_rear_cylinder_bulk_service_keepout",
        DECK_X - 220.0,
        REAR_CYLINDER_SERVICE_CLEARANCE,
        20.0,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 + REAR_CYLINDER_SERVICE_CLEARANCE / 2.0,
        DECK_Z / 2.0 + 10.0,
    );
    let left_service = centered_cube(
        "sterile_gas_changeover_left_regulator_service_keepout",
        SIDE_REGULATOR_SERVICE_CLEARANCE,
        DECK_Y - 150.0,
        16.0,
    )
    .translate(
        -(DECK_X / 2.0 + SIDE_REGULATOR_SERVICE_CLEARANCE / 2.0),
        -20.0,
        DECK_Z / 2.0 + 8.0,
    );
    let right_service = centered_cube(
        "sterile_gas_changeover_right_regulator_service_keepout",
        SIDE_REGULATOR_SERVICE_CLEARANCE,
        DECK_Y - 150.0,
        16.0,
    )
    .translate(
        DECK_X / 2.0 + SIDE_REGULATOR_SERVICE_CLEARANCE / 2.0,
        -20.0,
        DECK_Z / 2.0 + 8.0,
    );
    let top_lift = centered_cube(
        "sterile_gas_changeover_top_cylinder_lift_keepout_gauge",
        DECK_X - 280.0,
        42.0,
        TOP_CYLINDER_LIFT_CLEARANCE,
    )
    .translate(
        0.0,
        SOURCE_ROW_CENTER_Y,
        DECK_Z / 2.0 + TOP_CYLINDER_LIFT_CLEARANCE / 2.0,
    );

    front_robot + rear_cylinder + left_service + right_service + top_lift
}

fn gas_channel_x(index: usize) -> f64 {
    (index as f64 - (GAS_CHANNELS as f64 - 1.0) / 2.0) * CHANNEL_PITCH_X
}

fn source_row_y(source: usize) -> f64 {
    SOURCE_ROW_CENTER_Y + (source as f64 - 0.5) * SOURCE_ROW_PITCH_Y
}

fn source_offset_x(source: usize) -> f64 {
    (source as f64 - 0.5) * 112.0
}

fn source_label(source: usize) -> &'static str {
    match source {
        0 => "a",
        1 => "b",
        _ => "unknown",
    }
}

fn base_mount_points() -> [(f64, f64); 10] {
    [
        (-(DECK_X / 2.0 - 78.0), -(DECK_Y / 2.0 - 74.0)),
        (DECK_X / 2.0 - 78.0, -(DECK_Y / 2.0 - 74.0)),
        (-(DECK_X / 2.0 - 78.0), DECK_Y / 2.0 - 74.0),
        (DECK_X / 2.0 - 78.0, DECK_Y / 2.0 - 74.0),
        (-420.0, -(DECK_Y / 2.0 - 74.0)),
        (0.0, -(DECK_Y / 2.0 - 74.0)),
        (420.0, -(DECK_Y / 2.0 - 74.0)),
        (-420.0, DECK_Y / 2.0 - 74.0),
        (0.0, DECK_Y / 2.0 - 74.0),
        (420.0, DECK_Y / 2.0 - 74.0),
    ]
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        for path in OUTPUTS {
            assert!(
                path.starts_with("output/sterile_gas_changeover_regulator_qualification_panel_")
            );
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn gas_sources_are_explicit_dual_source_channels() {
        assert_eq!(GAS_NAMES, ["co2", "o2", "n2", "air"]);
        assert_eq!(SOURCES_PER_GAS, 2);
        assert_eq!(SOURCE_COUNT, 8);
        assert_eq!(REGULATOR_ENVELOPES, SOURCE_COUNT);
        assert_eq!(MFC_COUNT, GAS_CHANNELS);
        assert_eq!(CHANGEOVER_VALVES, GAS_CHANNELS);
        assert_eq!(source_label(0), "a");
        assert_eq!(source_label(1), "b");
    }

    #[test]
    fn source_restraints_fit_on_deck_with_service_margin() {
        assert!(gas_channel_x(0).abs() + CYLINDER_CRADLE_X / 2.0 < DECK_X / 2.0 - 96.0);
        assert!(
            gas_channel_x(GAS_CHANNELS - 1).abs() + CYLINDER_CRADLE_X / 2.0 < DECK_X / 2.0 - 96.0
        );
        assert!(source_row_y(0) - CYLINDER_CRADLE_Y / 2.0 > -DECK_Y / 2.0 + 70.0);
        assert!(source_row_y(1) + CYLINDER_CRADLE_Y / 2.0 < PANEL_CENTER_Y - 124.0);
        assert!(CYLINDER_FOOT_CUP_D < CYLINDER_CRADLE_X);
    }

    #[test]
    fn bought_gas_hardware_envelopes_are_counted_not_pressure_rated() {
        assert_eq!(BULK_GAS_INPUT_PORTS, GAS_CHANNELS);
        assert_eq!(QUALIFIED_OUTPUT_PORTS, GAS_CHANNELS);
        assert_eq!(STERILE_FILTERS, GAS_CHANNELS * 2);
        assert_eq!(CHECK_VALVES, SOURCE_COUNT);
        assert_eq!(RELIEF_VALVES, GAS_CHANNELS + 1);
        assert_eq!(CALIBRATION_GAS_PORTS, 1);
        assert!(REGULATOR_X > 90.0);
        assert!(MFC_X > REGULATOR_X);
    }

    #[test]
    fn verification_and_evidence_features_cover_each_channel() {
        assert_eq!(PRESSURE_TRANSDUCERS, GAS_CHANNELS * 2);
        assert_eq!(LEAK_TEST_PORTS, SOURCE_COUNT);
        assert_eq!(CYLINDER_BARCODE_LANDS, SOURCE_COUNT);
        assert_eq!(CHANGEOVER_EVIDENCE_WINDOWS, GAS_CHANNELS);
        assert_eq!(SERVICE_ISOLATION_TAGS, SOURCE_COUNT + GAS_CHANNELS + 2);
        assert!(TAG_LAND_X > 45.0);
    }

    #[test]
    fn service_keepouts_are_visible_for_robot_and_operator_access() {
        assert!(FRONT_ROBOT_CLEARANCE >= 500.0);
        assert!(REAR_CYLINDER_SERVICE_CLEARANCE >= 400.0);
        assert!(SIDE_REGULATOR_SERVICE_CLEARANCE >= 240.0);
        assert!(TOP_CYLINDER_LIFT_CLEARANCE >= 700.0);
        assert_eq!(REQUIRED_FEATURE_GROUPS.len(), 10);
    }
}
