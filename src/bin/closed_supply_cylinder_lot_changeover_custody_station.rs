use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed supply-cylinder lot changeover and custody validation station for the
// tissue-chip clean workcell.
//
// Design intent:
// - Keep cylinder lot handoff evidence physically adjacent to the actual A/B
//   cylinder restraints and dual regulator swap panel.
// - Make purge, vent, leak-check, sensor, gas-line label, and calibration coupon
//   interfaces visible as repeatable packaging geometry for bought certified gas
//   components.
// - Treat this as CAD for station layout, custody controls, and validation
//   fixtures. It is not a pressure-vessel or regulator design.

const OUTPUTS: [&str; 11] = [
    "output/closed_supply_cylinder_lot_changeover_custody_station_base_leak_tray.stl",
    "output/closed_supply_cylinder_lot_changeover_custody_station_cylinder_restraints.stl",
    "output/closed_supply_cylinder_lot_changeover_custody_station_dual_regulator_swap_panel.stl",
    "output/closed_supply_cylinder_lot_changeover_custody_station_barcode_lot_custody_pocket.stl",
    "output/closed_supply_cylinder_lot_changeover_custody_station_purge_vent_manifold.stl",
    "output/closed_supply_cylinder_lot_changeover_custody_station_leak_check_ports.stl",
    "output/closed_supply_cylinder_lot_changeover_custody_station_gas_line_label_lands.stl",
    "output/closed_supply_cylinder_lot_changeover_custody_station_sensor_mounts.stl",
    "output/closed_supply_cylinder_lot_changeover_custody_station_calibration_coupon_fixtures.stl",
    "output/closed_supply_cylinder_lot_changeover_custody_station_service_keepouts.stl",
    "output/closed_supply_cylinder_lot_changeover_custody_station_assembly.stl",
];

const REQUIRED_FEATURE_GROUPS: [&str; 9] = [
    "cylinder_restraints",
    "dual_regulator_swap_panel",
    "barcode_lot_custody_pocket",
    "purge_vent_manifold",
    "leak_check_ports",
    "gas_line_labels",
    "sensor_mounts",
    "calibration_coupon_fixtures",
    "robot_operator_service_keepouts",
];

const GAS_CHANNELS: usize = 4;
const GAS_NAMES: [&str; GAS_CHANNELS] = ["co2", "o2", "n2", "clean_air"];
const SOURCES_PER_GAS: usize = 2;
const SOURCE_COUNT: usize = GAS_CHANNELS * SOURCES_PER_GAS;
const REGULATOR_COUNT: usize = SOURCE_COUNT;
const SWAP_SELECTOR_COUNT: usize = GAS_CHANNELS;
const BARCODE_LANDS: usize = SOURCE_COUNT;
const LOT_CARD_SLOTS: usize = SOURCE_COUNT;
const CUSTODY_WITNESS_WINDOWS: usize = GAS_CHANNELS;
const PURGE_VALVES: usize = GAS_CHANNELS;
const VENT_FILTERS: usize = GAS_CHANNELS;
const RELIEF_VENTS: usize = GAS_CHANNELS;
const LEAK_CHECK_PORTS: usize = SOURCE_COUNT + GAS_CHANNELS;
const GAS_LINE_LABELS: usize = SOURCE_COUNT + GAS_CHANNELS;
const PRESSURE_SENSOR_MOUNTS: usize = SOURCE_COUNT;
const FLOW_SENSOR_MOUNTS: usize = GAS_CHANNELS;
const COUPON_FIXTURES: usize = 6;

const DECK_X: f64 = 1500.0;
const DECK_Y: f64 = 920.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 44.0;

const PANEL_X: f64 = 1370.0;
const PANEL_Y: f64 = 30.0;
const PANEL_Z: f64 = 430.0;
const PANEL_CENTER_Y: f64 = DECK_Y / 2.0 - 58.0;

const CHANNEL_PITCH_X: f64 = 310.0;
const SOURCE_ROW_PITCH_Y: f64 = 146.0;
const SOURCE_ROW_CENTER_Y: f64 = -116.0;

const CYLINDER_CRADLE_X: f64 = 236.0;
const CYLINDER_CRADLE_Y: f64 = 102.0;
const CYLINDER_RAIL_W: f64 = 18.0;
const CYLINDER_RAIL_Z: f64 = 38.0;
const CYLINDER_FOOT_CUP_D: f64 = 88.0;
const STRAP_POST_Z: f64 = 148.0;
const STRAP_BRIDGE_Z: f64 = 22.0;

const REGULATOR_X: f64 = 92.0;
const REGULATOR_Y: f64 = 58.0;
const REGULATOR_Z: f64 = 78.0;
const REGULATOR_GAUGE_D: f64 = 38.0;
const SWAP_VALVE_X: f64 = 112.0;
const SWAP_VALVE_Z: f64 = 74.0;

const CUSTODY_PANEL_X: f64 = 360.0;
const CUSTODY_PANEL_Y: f64 = 146.0;
const CUSTODY_PANEL_Z: f64 = 34.0;
const CUSTODY_POS: (f64, f64) = (-525.0, -365.0);

const MANIFOLD_X: f64 = 1290.0;
const MANIFOLD_Y: f64 = 86.0;
const MANIFOLD_Z: f64 = 84.0;
const MANIFOLD_POS_Y: f64 = 205.0;

const LEAK_PANEL_X: f64 = 320.0;
const LEAK_PANEL_Y: f64 = 132.0;
const LEAK_PANEL_Z: f64 = 34.0;
const LEAK_POS: (f64, f64) = (-160.0, -365.0);

const LABEL_RAIL_X: f64 = 1280.0;
const LABEL_RAIL_Y: f64 = 44.0;
const LABEL_RAIL_Z: f64 = 18.0;
const LABEL_RAIL_POS_Y: f64 = 92.0;

const SENSOR_PANEL_X: f64 = 300.0;
const SENSOR_PANEL_Y: f64 = 132.0;
const SENSOR_PANEL_Z: f64 = 42.0;
const SENSOR_POS: (f64, f64) = (175.0, -365.0);

const COUPON_PANEL_X: f64 = 320.0;
const COUPON_PANEL_Y: f64 = 132.0;
const COUPON_PANEL_Z: f64 = 40.0;
const COUPON_POS: (f64, f64) = (515.0, -365.0);

const FRONT_OPERATOR_CLEARANCE: f64 = 520.0;
const REAR_CYLINDER_CLEARANCE: f64 = 420.0;
const SIDE_CART_CLEARANCE: f64 = 220.0;
const TOP_CYLINDER_LIFT_CLEARANCE: f64 = 640.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_leak_tray();
    export(OUTPUTS[0], &base);

    let restraints = cylinder_restraints();
    export(OUTPUTS[1], &restraints);

    let swap_panel = dual_regulator_swap_panel();
    export(OUTPUTS[2], &swap_panel);

    let custody = barcode_lot_custody_pocket();
    export(OUTPUTS[3], &custody);

    let purge = purge_vent_manifold();
    export(OUTPUTS[4], &purge);

    let leak = leak_check_ports();
    export(OUTPUTS[5], &leak);

    let labels = gas_line_label_lands();
    export(OUTPUTS[6], &labels);

    let sensors = sensor_mounts();
    export(OUTPUTS[7], &sensors);

    let coupons = calibration_coupon_fixtures();
    export(OUTPUTS[8], &coupons);

    let keepouts = service_keepouts();
    export(OUTPUTS[9], &keepouts);

    let assembly = base
        + restraints
        + swap_panel
        + custody
        + purge
        + leak
        + labels
        + sensors
        + coupons
        + keepouts;
    export(OUTPUTS[10], &assembly);

    println!();
    println!("Closed supply-cylinder lot changeover/custody validation station:");
    println!("  Footprint:              {DECK_X:.0}mm x {DECK_Y:.0}mm leak-tray deck");
    println!(
        "  Gas channels:           {} channels ({:?}) with A/B source restraints",
        GAS_CHANNELS, GAS_NAMES
    );
    println!(
        "  Swap controls:          {REGULATOR_COUNT} regulator envelopes, {SWAP_SELECTOR_COUNT} A/B selectors, {PURGE_VALVES} purge valves, {VENT_FILTERS} vent filters, and {RELIEF_VENTS} relief vent paths"
    );
    println!(
        "  Custody evidence:       {BARCODE_LANDS} barcode lands, {LOT_CARD_SLOTS} lot card slots, {CUSTODY_WITNESS_WINDOWS} witness windows, tamper pads, and quarantine pocket"
    );
    println!(
        "  Verification interfaces:{LEAK_CHECK_PORTS} leak-check ports, {GAS_LINE_LABELS} gas-line label lands, {PRESSURE_SENSOR_MOUNTS} pressure sensor mounts, {FLOW_SENSOR_MOUNTS} flow sensor mounts, and {COUPON_FIXTURES} calibration coupon fixtures"
    );
    println!(
        "  Service envelopes:      {FRONT_OPERATOR_CLEARANCE:.0}mm front approach, {REAR_CYLINDER_CLEARANCE:.0}mm rear cylinder access, {SIDE_CART_CLEARANCE:.0}mm side cart access, {TOP_CYLINDER_LIFT_CLEARANCE:.0}mm lift gauge"
    );
    println!(
        "  Feature groups covered: {}. Geometry is packaging/interface CAD for bought certified gas hardware.",
        REQUIRED_FEATURE_GROUPS.len()
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn deck_top_z() -> f64 {
    DECK_Z / 2.0
}

fn place_z(height: f64) -> f64 {
    deck_top_z() + height / 2.0
}

fn base_leak_tray() -> Part {
    let deck = centered_cube(
        "closed_supply_cylinder_station_base_leak_tray_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    );

    let sump = centered_cube(
        "closed_supply_cylinder_station_recessed_gas_service_spill_sump",
        DECK_X - 156.0,
        92.0,
        8.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 72.0, deck_top_z() - 4.0);

    let drain = centered_cylinder(
        "closed_supply_cylinder_station_front_sump_drain",
        7.0,
        52.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 86.0, -DECK_Y / 2.0 + 26.0, 0.0);

    let rear_panel_socket = centered_cube(
        "closed_supply_cylinder_station_rear_swap_panel_socket",
        PANEL_X + 36.0,
        18.0,
        12.0,
    )
    .translate(0.0, PANEL_CENTER_Y, deck_top_z() - 6.0);

    let left_rim = centered_cube(
        "closed_supply_cylinder_station_left_retention_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, place_z(RIM_Z));
    let right_rim = centered_cube(
        "closed_supply_cylinder_station_right_retention_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, place_z(RIM_Z));
    let rear_rim = centered_cube(
        "closed_supply_cylinder_station_rear_retention_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, place_z(RIM_Z));
    let front_low_lip = centered_cube(
        "closed_supply_cylinder_station_low_front_service_lip",
        DECK_X,
        14.0,
        18.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 7.0, place_z(18.0));

    let mut mount_holes = Part::empty("closed_supply_cylinder_station_base_mount_hole_clearances");
    for (i, (x, y)) in base_mount_points().iter().enumerate() {
        mount_holes = mount_holes
            + centered_cylinder(
                format!("closed_supply_cylinder_station_base_mount_hole_{i}"),
                4.0,
                DECK_Z + 8.0,
                24,
            )
            .translate(*x, *y, 0.0);
    }

    deck - sump - drain - rear_panel_socket - mount_holes
        + left_rim
        + right_rim
        + rear_rim
        + front_low_lip
}

fn cylinder_restraints() -> Part {
    let mut bank = Part::empty("closed_supply_cylinder_station_cylinder_restraint_bank");
    for gas in 0..GAS_CHANNELS {
        for source in 0..SOURCES_PER_GAS {
            let x = channel_x(gas);
            let y = source_y(source);
            bank = bank + cylinder_restraint(gas, source).translate(x, y, 0.0);
        }
    }
    bank
}

fn cylinder_restraint(gas: usize, source: usize) -> Part {
    let label = format!(
        "closed_supply_cylinder_station_{}_source_{}",
        GAS_NAMES[gas],
        source_label(source)
    );

    let rail_left = centered_cube(
        format!("{label}_left_v_block_rail"),
        CYLINDER_CRADLE_X,
        CYLINDER_RAIL_W,
        CYLINDER_RAIL_Z,
    )
    .translate(
        0.0,
        -CYLINDER_CRADLE_Y / 2.0 + CYLINDER_RAIL_W / 2.0,
        place_z(CYLINDER_RAIL_Z),
    );
    let rail_right = centered_cube(
        format!("{label}_right_v_block_rail"),
        CYLINDER_CRADLE_X,
        CYLINDER_RAIL_W,
        CYLINDER_RAIL_Z,
    )
    .translate(
        0.0,
        CYLINDER_CRADLE_Y / 2.0 - CYLINDER_RAIL_W / 2.0,
        place_z(CYLINDER_RAIL_Z),
    );

    let foot_cup_ring = centered_cylinder(
        format!("{label}_rear_foot_cup_outer"),
        CYLINDER_FOOT_CUP_D / 2.0,
        14.0,
        48,
    )
    .translate(-CYLINDER_CRADLE_X / 2.0 + 52.0, 0.0, deck_top_z() + 7.0)
        - centered_cylinder(
            format!("{label}_rear_foot_cup_inner_clearance"),
            CYLINDER_FOOT_CUP_D / 2.0 - 13.0,
            18.0,
            48,
        )
        .translate(-CYLINDER_CRADLE_X / 2.0 + 52.0, 0.0, deck_top_z() + 7.0);

    let front_stop = centered_cube(
        format!("{label}_front_neck_stop_block"),
        24.0,
        CYLINDER_CRADLE_Y + 22.0,
        58.0,
    )
    .translate(CYLINDER_CRADLE_X / 2.0 - 18.0, 0.0, place_z(58.0));

    let mut strap_posts = Part::empty(format!("{label}_strap_posts"));
    for (i, x) in [
        -CYLINDER_CRADLE_X / 2.0 + 78.0,
        CYLINDER_CRADLE_X / 2.0 - 78.0,
    ]
    .iter()
    .enumerate()
    {
        for (j, y) in [
            -CYLINDER_CRADLE_Y / 2.0 - 14.0,
            CYLINDER_CRADLE_Y / 2.0 + 14.0,
        ]
        .iter()
        .enumerate()
        {
            strap_posts = strap_posts
                + centered_cylinder(format!("{label}_strap_post_{i}_{j}"), 7.0, STRAP_POST_Z, 24)
                    .translate(*x, *y, deck_top_z() + STRAP_POST_Z / 2.0);
        }
    }

    let strap_bridge = centered_cube(
        format!("{label}_lot_changeover_tamper_strap_bridge"),
        CYLINDER_CRADLE_X - 72.0,
        10.0,
        STRAP_BRIDGE_Z,
    )
    .translate(0.0, 0.0, deck_top_z() + STRAP_POST_Z - 18.0);

    let lot_token_land = centered_cube(format!("{label}_cylinder_lot_token_land"), 62.0, 8.0, 24.0)
        .translate(
            CYLINDER_CRADLE_X / 2.0 - 52.0,
            -CYLINDER_CRADLE_Y / 2.0 - 16.0,
            place_z(24.0),
        );

    rail_left
        + rail_right
        + foot_cup_ring
        + front_stop
        + strap_posts
        + strap_bridge
        + lot_token_land
}

fn dual_regulator_swap_panel() -> Part {
    let panel = centered_cube(
        "closed_supply_cylinder_station_dual_regulator_swap_backplane",
        PANEL_X,
        PANEL_Y,
        PANEL_Z,
    )
    .translate(0.0, PANEL_CENTER_Y, place_z(PANEL_Z));

    let mut regulator_envelopes = Part::empty("closed_supply_cylinder_station_regulator_envelopes");
    let mut gauges = Part::empty("closed_supply_cylinder_station_regulator_gauge_windows");
    for gas in 0..GAS_CHANNELS {
        for source in 0..SOURCES_PER_GAS {
            let x = channel_x(gas) + source_side_offset(source);
            let z = deck_top_z() + 274.0;
            regulator_envelopes = regulator_envelopes
                + centered_cube(
                    format!(
                        "closed_supply_cylinder_station_{}_source_{}_regulator_envelope",
                        GAS_NAMES[gas],
                        source_label(source)
                    ),
                    REGULATOR_X,
                    REGULATOR_Y,
                    REGULATOR_Z,
                )
                .translate(x, PANEL_CENTER_Y - PANEL_Y / 2.0 - 30.0, z);

            gauges = gauges
                + centered_cylinder(
                    format!(
                        "closed_supply_cylinder_station_{}_source_{}_regulator_gauge_window",
                        GAS_NAMES[gas],
                        source_label(source)
                    ),
                    REGULATOR_GAUGE_D / 2.0,
                    PANEL_Y + 10.0,
                    40,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(x, PANEL_CENTER_Y - 1.0, z + 8.0);
        }
    }

    let mut selectors = Part::empty("closed_supply_cylinder_station_ab_swap_selectors");
    for gas in 0..GAS_CHANNELS {
        let x = channel_x(gas);
        let valve = centered_cube(
            format!(
                "closed_supply_cylinder_station_{}_dual_regulator_swap_selector",
                GAS_NAMES[gas]
            ),
            SWAP_VALVE_X,
            48.0,
            SWAP_VALVE_Z,
        )
        .translate(
            x,
            PANEL_CENTER_Y - PANEL_Y / 2.0 - 34.0,
            deck_top_z() + 154.0,
        );
        let witness = centered_cube(
            format!(
                "closed_supply_cylinder_station_{}_ab_position_witness_window",
                GAS_NAMES[gas]
            ),
            76.0,
            PANEL_Y + 8.0,
            28.0,
        )
        .translate(x, PANEL_CENTER_Y - 1.0, deck_top_z() + 74.0);
        let lock_tab = centered_cube(
            format!(
                "closed_supply_cylinder_station_{}_regulator_swap_lockout_tab",
                GAS_NAMES[gas]
            ),
            102.0,
            8.0,
            24.0,
        )
        .translate(
            x,
            PANEL_CENTER_Y - PANEL_Y / 2.0 - 62.0,
            deck_top_z() + 96.0,
        );

        selectors = selectors + valve + witness + lock_tab;
    }

    panel - gauges + regulator_envelopes + selectors
}

fn barcode_lot_custody_pocket() -> Part {
    let tray = centered_cube(
        "closed_supply_cylinder_station_barcode_lot_custody_tray_body",
        CUSTODY_PANEL_X,
        CUSTODY_PANEL_Y,
        CUSTODY_PANEL_Z,
    );

    let mut barcode_recesses = Part::empty("closed_supply_cylinder_station_barcode_scan_recesses");
    for i in 0..BARCODE_LANDS {
        let row = i / 4;
        let col = i % 4;
        barcode_recesses = barcode_recesses
            + centered_cube(
                format!("closed_supply_cylinder_station_barcode_land_{i}"),
                58.0,
                18.0,
                9.0,
            )
            .translate(centered_index(col, 4, 78.0), 38.0 - row as f64 * 38.0, 12.0);
    }

    let mut card_slots = Part::empty("closed_supply_cylinder_station_lot_card_slots");
    for i in 0..LOT_CARD_SLOTS {
        let row = i / 4;
        let col = i % 4;
        card_slots = card_slots
            + centered_cube(
                format!("closed_supply_cylinder_station_lot_card_slot_{i}"),
                46.0,
                5.0,
                CUSTODY_PANEL_Z + 8.0,
            )
            .translate(
                centered_index(col, 4, 78.0),
                -CUSTODY_PANEL_Y / 2.0 + 18.0 + row as f64 * 22.0,
                0.0,
            );
    }

    let quarantine_pocket = centered_cube(
        "closed_supply_cylinder_station_quarantine_failed_lot_pocket",
        124.0,
        24.0,
        16.0,
    )
    .translate(
        -CUSTODY_PANEL_X / 2.0 + 78.0,
        -44.0,
        CUSTODY_PANEL_Z / 2.0 + 6.0,
    );

    let release_token_rail = centered_cube(
        "closed_supply_cylinder_station_released_lot_token_rail",
        134.0,
        14.0,
        18.0,
    )
    .translate(
        CUSTODY_PANEL_X / 2.0 - 84.0,
        -44.0,
        CUSTODY_PANEL_Z / 2.0 + 9.0,
    );

    let mut witness_windows = Part::empty("closed_supply_cylinder_station_custody_witness_windows");
    for gas in 0..GAS_CHANNELS {
        witness_windows = witness_windows
            + centered_cube(
                format!(
                    "closed_supply_cylinder_station_{}_lot_witness_window",
                    GAS_NAMES[gas]
                ),
                46.0,
                8.0,
                18.0,
            )
            .translate(
                centered_index(gas, GAS_CHANNELS, 70.0),
                0.0,
                CUSTODY_PANEL_Z / 2.0 + 9.0,
            );
    }

    let mut tamper_pads = Part::empty("closed_supply_cylinder_station_tamper_seal_pads");
    for i in 0..4 {
        tamper_pads = tamper_pads
            + centered_cylinder(
                format!("closed_supply_cylinder_station_tamper_seal_pad_{i}"),
                10.0,
                4.0,
                28,
            )
            .translate(
                centered_index(i, 4, 56.0),
                CUSTODY_PANEL_Y / 2.0 - 18.0,
                CUSTODY_PANEL_Z / 2.0 + 2.0,
            );
    }

    (tray - barcode_recesses - card_slots
        + quarantine_pocket
        + release_token_rail
        + witness_windows
        + tamper_pads)
        .translate(CUSTODY_POS.0, CUSTODY_POS.1, place_z(CUSTODY_PANEL_Z))
}

fn purge_vent_manifold() -> Part {
    let rail = centered_cube(
        "closed_supply_cylinder_station_purge_vent_manifold_body",
        MANIFOLD_X,
        MANIFOLD_Y,
        MANIFOLD_Z,
    )
    .translate(0.0, MANIFOLD_POS_Y, place_z(MANIFOLD_Z));

    let vent_header = centered_cylinder(
        "closed_supply_cylinder_station_common_vent_header",
        18.0,
        MANIFOLD_X - 90.0,
        36,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, MANIFOLD_POS_Y + 16.0, deck_top_z() + 100.0);

    let purge_header = centered_cylinder(
        "closed_supply_cylinder_station_common_purge_header",
        14.0,
        MANIFOLD_X - 130.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, MANIFOLD_POS_Y - 18.0, deck_top_z() + 54.0);

    let mut valves_filters = Part::empty("closed_supply_cylinder_station_purge_valves_filters");
    for gas in 0..GAS_CHANNELS {
        let x = channel_x(gas);
        let purge_valve = centered_cube(
            format!(
                "closed_supply_cylinder_station_{}_purge_valve_envelope",
                GAS_NAMES[gas]
            ),
            74.0,
            44.0,
            52.0,
        )
        .translate(x - 54.0, MANIFOLD_POS_Y - 50.0, place_z(52.0));

        let vent_filter = centered_cylinder(
            format!(
                "closed_supply_cylinder_station_{}_sterile_vent_filter",
                GAS_NAMES[gas]
            ),
            18.0,
            72.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x + 46.0, MANIFOLD_POS_Y + 58.0, deck_top_z() + 116.0);

        let relief_path = centered_cube(
            format!(
                "closed_supply_cylinder_station_{}_relief_vent_path_land",
                GAS_NAMES[gas]
            ),
            82.0,
            8.0,
            22.0,
        )
        .translate(
            x + 42.0,
            MANIFOLD_POS_Y + MANIFOLD_Y / 2.0 + 8.0,
            deck_top_z() + 52.0,
        );

        valves_filters = valves_filters + purge_valve + vent_filter + relief_path;
    }

    let drain_cup = centered_cube(
        "closed_supply_cylinder_station_purge_condensate_capture_cup",
        120.0,
        58.0,
        38.0,
    )
    .translate(
        MANIFOLD_X / 2.0 - 96.0,
        MANIFOLD_POS_Y - 58.0,
        place_z(38.0),
    );

    rail + vent_header + purge_header + valves_filters + drain_cup
}

fn leak_check_ports() -> Part {
    let panel = centered_cube(
        "closed_supply_cylinder_station_leak_check_port_panel",
        LEAK_PANEL_X,
        LEAK_PANEL_Y,
        LEAK_PANEL_Z,
    );

    let mut ports = Part::empty("closed_supply_cylinder_station_leak_check_quick_connect_ports");
    for i in 0..LEAK_CHECK_PORTS {
        let row = i / 6;
        let col = i % 6;
        ports = ports
            + centered_cylinder(
                format!("closed_supply_cylinder_station_leak_check_port_{i}"),
                8.0,
                LEAK_PANEL_Z + 10.0,
                28,
            )
            .translate(
                centered_index(col, 6, 46.0),
                centered_index(row, 2, 42.0),
                0.0,
            );
    }

    let gauge_nest = centered_cube(
        "closed_supply_cylinder_station_leak_gauge_reference_nest",
        88.0,
        22.0,
        12.0,
    )
    .translate(
        -LEAK_PANEL_X / 2.0 + 62.0,
        -LEAK_PANEL_Y / 2.0 + 22.0,
        LEAK_PANEL_Z / 2.0 - 6.0,
    );

    let pass_fail_token_lane = centered_cube(
        "closed_supply_cylinder_station_leak_pass_fail_token_lane",
        116.0,
        18.0,
        12.0,
    )
    .translate(
        LEAK_PANEL_X / 2.0 - 82.0,
        -LEAK_PANEL_Y / 2.0 + 22.0,
        LEAK_PANEL_Z / 2.0 + 6.0,
    );

    (panel - ports - gauge_nest + pass_fail_token_lane).translate(
        LEAK_POS.0,
        LEAK_POS.1,
        place_z(LEAK_PANEL_Z),
    )
}

fn gas_line_label_lands() -> Part {
    let rail = centered_cube(
        "closed_supply_cylinder_station_gas_line_label_reference_rail",
        LABEL_RAIL_X,
        LABEL_RAIL_Y,
        LABEL_RAIL_Z,
    )
    .translate(0.0, LABEL_RAIL_POS_Y, place_z(LABEL_RAIL_Z));

    let mut labels = Part::empty("closed_supply_cylinder_station_gas_line_label_lands");
    for gas in 0..GAS_CHANNELS {
        for source in 0..SOURCES_PER_GAS {
            labels = labels
                + centered_cube(
                    format!(
                        "closed_supply_cylinder_station_{}_source_{}_line_label_land",
                        GAS_NAMES[gas],
                        source_label(source)
                    ),
                    78.0,
                    8.0,
                    18.0,
                )
                .translate(
                    channel_x(gas) + source_side_offset(source),
                    LABEL_RAIL_POS_Y - LABEL_RAIL_Y / 2.0 - 8.0,
                    place_z(18.0),
                );
        }

        labels = labels
            + centered_cube(
                format!(
                    "closed_supply_cylinder_station_{}_qualified_output_label_land",
                    GAS_NAMES[gas]
                ),
                112.0,
                8.0,
                18.0,
            )
            .translate(
                channel_x(gas),
                LABEL_RAIL_POS_Y + LABEL_RAIL_Y / 2.0 + 8.0,
                place_z(18.0),
            );
    }

    let arrow_strip = centered_cube(
        "closed_supply_cylinder_station_flow_direction_arrow_strip",
        LABEL_RAIL_X - 96.0,
        6.0,
        10.0,
    )
    .translate(0.0, LABEL_RAIL_POS_Y, deck_top_z() + LABEL_RAIL_Z + 5.0);

    rail + labels + arrow_strip
}

fn sensor_mounts() -> Part {
    let base = centered_cube(
        "closed_supply_cylinder_station_sensor_mount_base",
        SENSOR_PANEL_X,
        SENSOR_PANEL_Y,
        SENSOR_PANEL_Z,
    );

    let mut pressure_pockets =
        Part::empty("closed_supply_cylinder_station_pressure_transducer_pockets");
    for i in 0..PRESSURE_SENSOR_MOUNTS {
        let row = i / 4;
        let col = i % 4;
        pressure_pockets = pressure_pockets
            + centered_cube(
                format!("closed_supply_cylinder_station_pressure_sensor_mount_{i}"),
                42.0,
                22.0,
                18.0,
            )
            .translate(
                centered_index(col, 4, 62.0),
                28.0 - row as f64 * 48.0,
                SENSOR_PANEL_Z / 2.0 - 9.0,
            );
    }

    let mut flow_clamps = Part::empty("closed_supply_cylinder_station_flow_sensor_clamps");
    for gas in 0..FLOW_SENSOR_MOUNTS {
        flow_clamps = flow_clamps
            + centered_cube(
                format!(
                    "closed_supply_cylinder_station_{}_flow_sensor_bridge_clamp",
                    GAS_NAMES[gas]
                ),
                46.0,
                16.0,
                34.0,
            )
            .translate(
                centered_index(gas, FLOW_SENSOR_MOUNTS, 62.0),
                -SENSOR_PANEL_Y / 2.0 + 22.0,
                SENSOR_PANEL_Z / 2.0 + 17.0,
            );
    }

    let harness_strain_relief = centered_cube(
        "closed_supply_cylinder_station_sensor_harness_strain_relief_comb",
        SENSOR_PANEL_X - 46.0,
        8.0,
        18.0,
    )
    .translate(0.0, SENSOR_PANEL_Y / 2.0 - 14.0, SENSOR_PANEL_Z / 2.0 + 9.0);

    (base - pressure_pockets + flow_clamps + harness_strain_relief).translate(
        SENSOR_POS.0,
        SENSOR_POS.1,
        place_z(SENSOR_PANEL_Z),
    )
}

fn calibration_coupon_fixtures() -> Part {
    let tray = centered_cube(
        "closed_supply_cylinder_station_calibration_coupon_fixture_tray",
        COUPON_PANEL_X,
        COUPON_PANEL_Y,
        COUPON_PANEL_Z,
    );

    let mut coupon_slots = Part::empty("closed_supply_cylinder_station_calibration_coupon_slots");
    for i in 0..COUPON_FIXTURES {
        coupon_slots = coupon_slots
            + centered_cube(
                format!("closed_supply_cylinder_station_calibration_coupon_slot_{i}"),
                62.0,
                24.0,
                14.0,
            )
            .translate(
                centered_index(i % 3, 3, 88.0),
                centered_index(i / 3, 2, 54.0),
                COUPON_PANEL_Z / 2.0 - 7.0,
            );
    }

    let leak_standard_nest = centered_cylinder(
        "closed_supply_cylinder_station_leak_standard_orifice_coupon_nest",
        18.0,
        COUPON_PANEL_Z + 8.0,
        32,
    )
    .translate(-COUPON_PANEL_X / 2.0 + 34.0, 0.0, 0.0);

    let barcode_witness_clip = centered_cube(
        "closed_supply_cylinder_station_barcode_witness_coupon_clip",
        74.0,
        12.0,
        22.0,
    )
    .translate(
        COUPON_PANEL_X / 2.0 - 58.0,
        COUPON_PANEL_Y / 2.0 - 22.0,
        COUPON_PANEL_Z / 2.0 + 11.0,
    );

    let purge_flow_coupon_stop = centered_cube(
        "closed_supply_cylinder_station_purge_flow_coupon_stop",
        92.0,
        10.0,
        22.0,
    )
    .translate(
        COUPON_PANEL_X / 2.0 - 68.0,
        -COUPON_PANEL_Y / 2.0 + 22.0,
        COUPON_PANEL_Z / 2.0 + 11.0,
    );

    (tray - coupon_slots - leak_standard_nest + barcode_witness_clip + purge_flow_coupon_stop)
        .translate(COUPON_POS.0, COUPON_POS.1, place_z(COUPON_PANEL_Z))
}

fn service_keepouts() -> Part {
    let front_operator = centered_cube(
        "closed_supply_cylinder_station_front_operator_robot_approach_keepout",
        DECK_X - 240.0,
        FRONT_OPERATOR_CLEARANCE,
        14.0,
    )
    .translate(
        0.0,
        -(DECK_Y / 2.0 + FRONT_OPERATOR_CLEARANCE / 2.0),
        deck_top_z() + 7.0,
    );

    let rear_cylinder = centered_cube(
        "closed_supply_cylinder_station_rear_cylinder_exchange_keepout",
        DECK_X - 210.0,
        REAR_CYLINDER_CLEARANCE,
        16.0,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 + REAR_CYLINDER_CLEARANCE / 2.0,
        deck_top_z() + 8.0,
    );

    let left_cart = centered_cube(
        "closed_supply_cylinder_station_left_cylinder_cart_keepout",
        SIDE_CART_CLEARANCE,
        DECK_Y - 120.0,
        14.0,
    )
    .translate(
        -(DECK_X / 2.0 + SIDE_CART_CLEARANCE / 2.0),
        -22.0,
        deck_top_z() + 7.0,
    );

    let right_cart = centered_cube(
        "closed_supply_cylinder_station_right_cylinder_cart_keepout",
        SIDE_CART_CLEARANCE,
        DECK_Y - 120.0,
        14.0,
    )
    .translate(
        DECK_X / 2.0 + SIDE_CART_CLEARANCE / 2.0,
        -22.0,
        deck_top_z() + 7.0,
    );

    let lift_gauge = centered_cube(
        "closed_supply_cylinder_station_top_cylinder_lift_clearance_gauge",
        DECK_X - 300.0,
        34.0,
        TOP_CYLINDER_LIFT_CLEARANCE,
    )
    .translate(
        0.0,
        SOURCE_ROW_CENTER_Y,
        deck_top_z() + TOP_CYLINDER_LIFT_CLEARANCE / 2.0,
    );

    front_operator + rear_cylinder + left_cart + right_cart + lift_gauge
}

fn channel_x(index: usize) -> f64 {
    centered_index(index, GAS_CHANNELS, CHANNEL_PITCH_X)
}

fn source_y(index: usize) -> f64 {
    SOURCE_ROW_CENTER_Y + centered_index(index, SOURCES_PER_GAS, SOURCE_ROW_PITCH_Y)
}

fn source_side_offset(index: usize) -> f64 {
    centered_index(index, SOURCES_PER_GAS, 86.0)
}

fn source_label(index: usize) -> &'static str {
    match index {
        0 => "a",
        1 => "b",
        _ => "unknown",
    }
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
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

fn insert_specs() -> [(&'static str, (f64, f64), f64, f64); 4] {
    [
        ("custody", CUSTODY_POS, CUSTODY_PANEL_X, CUSTODY_PANEL_Y),
        ("leak", LEAK_POS, LEAK_PANEL_X, LEAK_PANEL_Y),
        ("sensor", SENSOR_POS, SENSOR_PANEL_X, SENSOR_PANEL_Y),
        ("coupon", COUPON_POS, COUPON_PANEL_X, COUPON_PANEL_Y),
    ]
}

fn fits_on_deck(pos: (f64, f64), width: f64, depth: f64) -> bool {
    pos.0.abs() + width / 2.0 < DECK_X / 2.0 - RIM_W
        && pos.1.abs() + depth / 2.0 < DECK_Y / 2.0 - RIM_W
}

fn assert_layout() {
    assert!(PANEL_X < DECK_X - 2.0 * RIM_W);
    assert!(PANEL_CENTER_Y + PANEL_Y / 2.0 < DECK_Y / 2.0 - RIM_W);
    for (_, pos, width, depth) in insert_specs() {
        assert!(fits_on_deck(pos, width, depth));
    }
    for gas in 0..GAS_CHANNELS {
        assert!(channel_x(gas).abs() + CYLINDER_CRADLE_X / 2.0 < DECK_X / 2.0 - 52.0);
    }
    assert!(source_y(0) - CYLINDER_CRADLE_Y / 2.0 > -DECK_Y / 2.0 + 118.0);
    assert!(source_y(1) + CYLINDER_CRADLE_Y / 2.0 < MANIFOLD_POS_Y - MANIFOLD_Y / 2.0);
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
            assert!(
                path.starts_with("output/closed_supply_cylinder_lot_changeover_custody_station_")
            );
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn all_requested_feature_groups_are_present() {
        for feature in [
            "cylinder_restraints",
            "dual_regulator_swap_panel",
            "barcode_lot_custody_pocket",
            "purge_vent_manifold",
            "leak_check_ports",
            "gas_line_labels",
            "sensor_mounts",
            "calibration_coupon_fixtures",
        ] {
            assert!(REQUIRED_FEATURE_GROUPS.contains(&feature));
        }
        assert_eq!(REQUIRED_FEATURE_GROUPS.len(), 9);
    }

    #[test]
    fn cylinder_sources_and_regulator_swap_are_dual_lot() {
        assert_eq!(GAS_NAMES, ["co2", "o2", "n2", "clean_air"]);
        assert_eq!(SOURCES_PER_GAS, 2);
        assert_eq!(SOURCE_COUNT, 8);
        assert_eq!(REGULATOR_COUNT, SOURCE_COUNT);
        assert_eq!(SWAP_SELECTOR_COUNT, GAS_CHANNELS);
        assert_eq!(source_label(0), "a");
        assert_eq!(source_label(1), "b");
    }

    #[test]
    fn traceability_and_validation_counts_cover_every_source() {
        assert_eq!(BARCODE_LANDS, SOURCE_COUNT);
        assert_eq!(LOT_CARD_SLOTS, SOURCE_COUNT);
        assert_eq!(CUSTODY_WITNESS_WINDOWS, GAS_CHANNELS);
        assert_eq!(LEAK_CHECK_PORTS, SOURCE_COUNT + GAS_CHANNELS);
        assert_eq!(GAS_LINE_LABELS, SOURCE_COUNT + GAS_CHANNELS);
        assert_eq!(PRESSURE_SENSOR_MOUNTS, SOURCE_COUNT);
        assert_eq!(FLOW_SENSOR_MOUNTS, GAS_CHANNELS);
        assert!(COUPON_FIXTURES >= 6);
    }

    #[test]
    fn purge_vent_and_sensor_interfaces_are_channelized() {
        assert_eq!(PURGE_VALVES, GAS_CHANNELS);
        assert_eq!(VENT_FILTERS, GAS_CHANNELS);
        assert_eq!(RELIEF_VENTS, GAS_CHANNELS);
        assert!(REGULATOR_X > 80.0);
        assert!(REGULATOR_GAUGE_D > 30.0);
    }

    #[test]
    fn layout_fits_with_service_clearances() {
        assert_layout();
        assert!(FRONT_OPERATOR_CLEARANCE >= 500.0);
        assert!(REAR_CYLINDER_CLEARANCE >= 400.0);
        assert!(SIDE_CART_CLEARANCE >= 200.0);
        assert!(TOP_CYLINDER_LIFT_CLEARANCE >= 600.0);
        assert!(CYLINDER_FOOT_CUP_D < CYLINDER_CRADLE_X / 2.0);
    }
}
