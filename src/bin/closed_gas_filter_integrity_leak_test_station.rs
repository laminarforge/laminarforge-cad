use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed gas filter integrity/leak-test station for sterile CO2/O2/N2/air
// feeds into incubator and culture modules.
//
// Intent:
// - Fixture sterile inline gas filter cartridges before they are released to a
//   closed incubator/culture module gas feed.
// - Make check-valve and filter orientation visible, with pressure-decay ports,
//   vent/relief routing placeholders, barcode evidence lands, and release/hold/
//   reject handling lanes in one bench-scale module.
// - Model purchased valves, regulators, certified filters, relief hardware, and
//   transducers as service envelopes only; the printed CAD is not pressure-rated
//   containment or sterile barrier validation hardware.
//
// Exports:
//   output/closed_gas_filter_integrity_leak_test_station_base_containment_tray.stl
//   output/closed_gas_filter_integrity_leak_test_station_bulkhead_panel.stl
//   output/closed_gas_filter_integrity_leak_test_station_filter_cartridge_nests.stl
//   output/closed_gas_filter_integrity_leak_test_station_orientation_gauge_bank.stl
//   output/closed_gas_filter_integrity_leak_test_station_pressure_decay_port_manifold.stl
//   output/closed_gas_filter_integrity_leak_test_station_vent_relief_route_bank.stl
//   output/closed_gas_filter_integrity_leak_test_station_calibration_source_barcode_lands.stl
//   output/closed_gas_filter_integrity_leak_test_station_release_hold_reject_lanes.stl
//   output/closed_gas_filter_integrity_leak_test_station_robot_service_keepouts.stl
//   output/closed_gas_filter_integrity_leak_test_station_assembly.stl

const OUTPUTS: [&str; 10] = [
    "output/closed_gas_filter_integrity_leak_test_station_base_containment_tray.stl",
    "output/closed_gas_filter_integrity_leak_test_station_bulkhead_panel.stl",
    "output/closed_gas_filter_integrity_leak_test_station_filter_cartridge_nests.stl",
    "output/closed_gas_filter_integrity_leak_test_station_orientation_gauge_bank.stl",
    "output/closed_gas_filter_integrity_leak_test_station_pressure_decay_port_manifold.stl",
    "output/closed_gas_filter_integrity_leak_test_station_vent_relief_route_bank.stl",
    "output/closed_gas_filter_integrity_leak_test_station_calibration_source_barcode_lands.stl",
    "output/closed_gas_filter_integrity_leak_test_station_release_hold_reject_lanes.stl",
    "output/closed_gas_filter_integrity_leak_test_station_robot_service_keepouts.stl",
    "output/closed_gas_filter_integrity_leak_test_station_assembly.stl",
];

const REQUIRED_FEATURE_GROUPS: [&str; 9] = [
    "sterile_gas_filter_cartridge_nests",
    "check_valve_filter_orientation_gauges",
    "pressure_decay_ports",
    "sterile_vent_relief_route_placeholders",
    "calibration_gas_source_barcode_lands",
    "release_hold_reject_lanes",
    "gas_safe_leak_containment_tray",
    "robot_service_keepouts",
    "incubator_module_bulkhead_panel",
];

const GAS_CHANNELS: usize = 4;
const GAS_NAMES: [&str; GAS_CHANNELS] = ["co2", "o2", "n2", "air"];
const FILTERS_PER_CHANNEL: usize = 2;
const FILTER_CARTRIDGE_NESTS: usize = GAS_CHANNELS * FILTERS_PER_CHANNEL;
const CHECK_VALVES_PER_CHANNEL: usize = 2;
const CHECK_VALVE_PLACEHOLDERS: usize = GAS_CHANNELS * CHECK_VALVES_PER_CHANNEL;
const ORIENTATION_GAUGES: usize = FILTER_CARTRIDGE_NESTS + CHECK_VALVE_PLACEHOLDERS;
const PRESSURE_DECAY_PORTS_PER_CHANNEL: usize = 2;
const PRESSURE_DECAY_PORTS: usize = GAS_CHANNELS * PRESSURE_DECAY_PORTS_PER_CHANNEL;
const PRESSURE_SENSOR_POCKETS: usize = GAS_CHANNELS;
const VENT_FILTERS: usize = GAS_CHANNELS;
const RELIEF_ROUTE_PLACEHOLDERS: usize = GAS_CHANNELS + 1;
const SOURCE_BARCODE_LANDS: usize = GAS_CHANNELS;
const CALIBRATION_BARCODE_LANDS: usize = 2;
const TOTAL_BARCODE_LANDS: usize = SOURCE_BARCODE_LANDS + CALIBRATION_BARCODE_LANDS;
const SORT_LANES: usize = 3;
const SORT_LANE_NAMES: [&str; SORT_LANES] = ["release", "hold", "reject"];

const BASE_X: f64 = 1120.0;
const BASE_Y: f64 = 690.0;
const BASE_Z: f64 = 22.0;
const CONTAINMENT_WALL_Z: f64 = 46.0;
const CONTAINMENT_WALL_T: f64 = 18.0;
const SUMP_DRAIN_D: f64 = 12.0;
const MOUNT_HOLE_D: f64 = 6.6;

const BULKHEAD_X: f64 = 1040.0;
const BULKHEAD_Y: f64 = 28.0;
const BULKHEAD_Z: f64 = 360.0;
const BULKHEAD_CENTER_Y: f64 = BASE_Y / 2.0 - 52.0;
const BULKHEAD_CENTER_Z: f64 = BASE_Z / 2.0 + BULKHEAD_Z / 2.0;
const BULKHEAD_PORT_D: f64 = 11.0;
const BULKHEAD_OUTPUT_D: f64 = 10.0;
const CALIBRATION_PORT_D: f64 = 6.4;
const RELIEF_PORT_D: f64 = 14.0;

const CHANNEL_PITCH_X: f64 = 232.0;
const FILTER_STAGE_PITCH_Y: f64 = 92.0;
const FILTER_BANK_CENTER_Y: f64 = 36.0;
const FILTER_NEST_BLOCK_X: f64 = 136.0;
const FILTER_NEST_BLOCK_Y: f64 = 54.0;
const FILTER_NEST_BLOCK_Z: f64 = 38.0;
const FILTER_BODY_D: f64 = 34.0;
const FILTER_BODY_LEN: f64 = 122.0;
const FILTER_CLAMP_X: f64 = 48.0;
const FILTER_CLAMP_Y: f64 = 14.0;
const FILTER_CLAMP_Z: f64 = 24.0;

const ORIENTATION_BANK_X: f64 = 1010.0;
const ORIENTATION_BANK_Y: f64 = 76.0;
const ORIENTATION_BANK_Z: f64 = 68.0;
const CHECK_VALVE_D: f64 = 22.0;
const CHECK_VALVE_LEN: f64 = 64.0;

const DECAY_MANIFOLD_X: f64 = 1010.0;
const DECAY_MANIFOLD_Y: f64 = 76.0;
const DECAY_MANIFOLD_Z: f64 = 86.0;
const DECAY_PORT_D: f64 = 8.0;
const SENSOR_POCKET_X: f64 = 58.0;
const SENSOR_POCKET_Y: f64 = 18.0;
const SENSOR_POCKET_Z: f64 = 40.0;

const VENT_BANK_X: f64 = 1010.0;
const VENT_BANK_Y: f64 = 72.0;
const VENT_BANK_Z: f64 = 92.0;
const VENT_FILTER_D: f64 = 30.0;
const VENT_FILTER_LEN: f64 = 78.0;

const BARCODE_PANEL_X: f64 = 900.0;
const BARCODE_PANEL_Y: f64 = 104.0;
const BARCODE_PANEL_Z: f64 = 10.0;
const BARCODE_LAND_X: f64 = 92.0;
const BARCODE_LAND_Y: f64 = 56.0;
const BARCODE_LAND_Z: f64 = 6.0;

const LANE_PANEL_X: f64 = 900.0;
const LANE_PANEL_Y: f64 = 156.0;
const LANE_PANEL_Z: f64 = 16.0;
const LANE_PITCH_X: f64 = 260.0;
const LANE_WIDTH_X: f64 = 210.0;
const LANE_RAIL_Y: f64 = 126.0;
const LANE_RAIL_Z: f64 = 28.0;
const LANE_GATE_Z: f64 = 58.0;

const FRONT_ROBOT_KEEP_OUT_Y: f64 = 460.0;
const REAR_BULKHEAD_SERVICE_KEEP_OUT_Y: f64 = 330.0;
const RIGHT_FILTER_SERVICE_KEEP_OUT_X: f64 = 240.0;
const TOP_FILTER_LIFT_KEEP_OUT_Z: f64 = 260.0;

fn main() {
    fs::create_dir_all("output").unwrap();

    let base = base_containment_tray();
    export(OUTPUTS[0], &base);

    let bulkhead = bulkhead_panel();
    export(OUTPUTS[1], &bulkhead);

    let nests = filter_cartridge_nests();
    export(OUTPUTS[2], &nests);

    let orientation = orientation_gauge_bank();
    export(OUTPUTS[3], &orientation);

    let decay = pressure_decay_port_manifold();
    export(OUTPUTS[4], &decay);

    let vent = vent_relief_route_bank();
    export(OUTPUTS[5], &vent);

    let barcode = calibration_source_barcode_lands();
    export(OUTPUTS[6], &barcode);

    let lanes = release_hold_reject_lanes();
    export(OUTPUTS[7], &lanes);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[8], &keepouts);

    let assembly =
        base + bulkhead + nests + orientation + decay + vent + barcode + lanes + keepouts;
    export(OUTPUTS[9], &assembly);

    println!(
        "Closed gas filter integrity/leak-test station: {:.0} x {:.0}mm deck, {:.0}mm bulkhead panel, {} gas channels ({:?}), {} filter cartridge nests, {} check-valve placeholders, {} orientation gauges, {} pressure-decay ports, {} sensor pockets, {} sterile vent filters, {} relief route placeholders, {} barcode lands, and {} release/hold/reject lanes.",
        BASE_X,
        BASE_Y,
        BULKHEAD_Z,
        GAS_CHANNELS,
        GAS_NAMES,
        FILTER_CARTRIDGE_NESTS,
        CHECK_VALVE_PLACEHOLDERS,
        ORIENTATION_GAUGES,
        PRESSURE_DECAY_PORTS,
        PRESSURE_SENSOR_POCKETS,
        VENT_FILTERS,
        RELIEF_ROUTE_PLACEHOLDERS,
        TOTAL_BARCODE_LANDS,
        SORT_LANES
    );
    println!(
        "Service geometry: {:.0}mm front robot keepout, {:.0}mm rear bulkhead access, {:.0}mm right-side filter service bay, {:.0}mm top cartridge lift envelope, and {} required feature groups. CAD is packaging/interface geometry for bought certified gas and sterile filter hardware.",
        FRONT_ROBOT_KEEP_OUT_Y,
        REAR_BULKHEAD_SERVICE_KEEP_OUT_Y,
        RIGHT_FILTER_SERVICE_KEEP_OUT_X,
        TOP_FILTER_LIFT_KEEP_OUT_Z,
        REQUIRED_FEATURE_GROUPS.len()
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn base_containment_tray() -> Part {
    let deck = centered_cube(
        "closed_gas_filter_integrity_base_containment_deck",
        BASE_X,
        BASE_Y,
        BASE_Z,
    );
    let sump_recess = centered_cube(
        "closed_gas_filter_integrity_low_pressure_sump_recess",
        BASE_X - 126.0,
        BASE_Y - 148.0,
        8.0,
    )
    .translate(0.0, -22.0, BASE_Z / 2.0 - 3.0);
    let front_drain = centered_cylinder(
        "closed_gas_filter_integrity_front_sump_drain",
        SUMP_DRAIN_D / 2.0,
        44.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(BASE_X / 2.0 - 76.0, -(BASE_Y / 2.0 - 18.0), 0.0);
    let rear_socket = centered_cube(
        "closed_gas_filter_integrity_bulkhead_panel_socket",
        BULKHEAD_X + 36.0,
        18.0,
        8.0,
    )
    .translate(0.0, BULKHEAD_CENTER_Y, BASE_Z / 2.0 - 2.0);

    let mut mount_clearances = Part::empty("closed_gas_filter_integrity_mount_clearances");
    for (i, (x, y)) in base_mount_points().iter().enumerate() {
        let hole = centered_cylinder(
            format!("closed_gas_filter_integrity_m6_mount_clearance_{i}"),
            MOUNT_HOLE_D / 2.0,
            BASE_Z + 6.0,
            24,
        )
        .translate(*x, *y, 0.0);
        let slot = centered_cube(
            format!("closed_gas_filter_integrity_m6_mount_slot_{i}"),
            26.0,
            MOUNT_HOLE_D + 1.0,
            BASE_Z + 6.0,
        )
        .translate(*x, *y, 0.0);
        mount_clearances = mount_clearances + hole + slot;
    }

    deck - sump_recess - front_drain - rear_socket - mount_clearances
        + containment_walls()
        + datum_rails()
        + leveling_feet()
}

fn containment_walls() -> Part {
    let left = centered_cube(
        "closed_gas_filter_integrity_left_gas_safe_containment_wall",
        CONTAINMENT_WALL_T,
        BASE_Y - 92.0,
        CONTAINMENT_WALL_Z,
    )
    .translate(
        -(BASE_X / 2.0 - 34.0),
        -20.0,
        BASE_Z / 2.0 + CONTAINMENT_WALL_Z / 2.0,
    );
    let right = centered_cube(
        "closed_gas_filter_integrity_right_gas_safe_containment_wall",
        CONTAINMENT_WALL_T,
        BASE_Y - 92.0,
        CONTAINMENT_WALL_Z,
    )
    .translate(
        BASE_X / 2.0 - 34.0,
        -20.0,
        BASE_Z / 2.0 + CONTAINMENT_WALL_Z / 2.0,
    );
    let front = centered_cube(
        "closed_gas_filter_integrity_front_low_wall_with_robot_notch",
        BASE_X - 112.0,
        CONTAINMENT_WALL_T,
        30.0,
    )
    .translate(0.0, -(BASE_Y / 2.0 - 30.0), BASE_Z / 2.0 + 15.0);
    let rear = centered_cube(
        "closed_gas_filter_integrity_rear_bulkhead_splash_wall",
        BASE_X - 126.0,
        CONTAINMENT_WALL_T,
        CONTAINMENT_WALL_Z,
    )
    .translate(
        0.0,
        BULKHEAD_CENTER_Y - 26.0,
        BASE_Z / 2.0 + CONTAINMENT_WALL_Z / 2.0,
    );

    left + right + front + rear
}

fn datum_rails() -> Part {
    let left = centered_cube(
        "closed_gas_filter_integrity_left_fixture_datum_rail",
        18.0,
        BASE_Y - 230.0,
        22.0,
    )
    .translate(-BASE_X / 2.0 + 92.0, -28.0, BASE_Z / 2.0 + 11.0);
    let right = centered_cube(
        "closed_gas_filter_integrity_right_fixture_datum_rail",
        18.0,
        BASE_Y - 230.0,
        22.0,
    )
    .translate(BASE_X / 2.0 - 92.0, -28.0, BASE_Z / 2.0 + 11.0);
    let center = centered_cube(
        "closed_gas_filter_integrity_center_cartridge_datum_rail",
        BASE_X - 260.0,
        16.0,
        20.0,
    )
    .translate(0.0, FILTER_BANK_CENTER_Y, BASE_Z / 2.0 + 10.0);
    let bulkhead_heel = centered_cube(
        "closed_gas_filter_integrity_bulkhead_heel_rail",
        BULKHEAD_X + 42.0,
        16.0,
        20.0,
    )
    .translate(0.0, BULKHEAD_CENTER_Y - 28.0, BASE_Z / 2.0 + 10.0);

    left + right + center + bulkhead_heel
}

fn leveling_feet() -> Part {
    let mut feet = Part::empty("closed_gas_filter_integrity_leveling_feet");
    for (i, (x, y)) in [
        (-(BASE_X / 2.0 - 62.0), -(BASE_Y / 2.0 - 58.0)),
        (BASE_X / 2.0 - 62.0, -(BASE_Y / 2.0 - 58.0)),
        (-(BASE_X / 2.0 - 62.0), BASE_Y / 2.0 - 78.0),
        (BASE_X / 2.0 - 62.0, BASE_Y / 2.0 - 78.0),
        (0.0, -(BASE_Y / 2.0 - 58.0)),
        (0.0, BASE_Y / 2.0 - 78.0),
    ]
    .iter()
    .enumerate()
    {
        let pad = centered_cylinder(
            format!("closed_gas_filter_integrity_leveling_pad_{i}"),
            24.0,
            10.0,
            32,
        )
        .translate(*x, *y, -(BASE_Z / 2.0 + 5.0));
        let adjuster = centered_cylinder(
            format!("closed_gas_filter_integrity_leveler_screw_clearance_{i}"),
            4.0,
            14.0,
            20,
        )
        .translate(*x, *y, -(BASE_Z / 2.0 + 5.0));
        feet = feet + (pad - adjuster);
    }
    feet
}

fn bulkhead_panel() -> Part {
    let panel = centered_cube(
        "closed_gas_filter_integrity_incubator_feed_bulkhead_panel",
        BULKHEAD_X,
        BULKHEAD_Y,
        BULKHEAD_Z,
    )
    .translate(0.0, BULKHEAD_CENTER_Y, BULKHEAD_CENTER_Z);

    let mut cuts = Part::empty("closed_gas_filter_integrity_bulkhead_panel_cuts");
    let mut lands = Part::empty("closed_gas_filter_integrity_bulkhead_panel_lands");

    for i in 0..GAS_CHANNELS {
        let x = channel_x(i);
        let inlet = bulkhead_cut(
            format!(
                "closed_gas_filter_integrity_{}_source_inlet_cut",
                GAS_NAMES[i]
            ),
            BULKHEAD_PORT_D,
            x,
            78.0,
        );
        let output = bulkhead_cut(
            format!(
                "closed_gas_filter_integrity_{}_qualified_output_cut",
                GAS_NAMES[i]
            ),
            BULKHEAD_OUTPUT_D,
            x,
            -78.0,
        );
        let inlet_land = bulkhead_land(
            format!(
                "closed_gas_filter_integrity_{}_source_inlet_land",
                GAS_NAMES[i]
            ),
            19.0,
            x,
            78.0,
        );
        let output_land = bulkhead_land(
            format!(
                "closed_gas_filter_integrity_{}_qualified_output_land",
                GAS_NAMES[i]
            ),
            18.0,
            x,
            -78.0,
        );
        cuts = cuts + inlet + output;
        lands = lands + inlet_land + output_land;
    }

    cuts =
        cuts + bulkhead_cut(
            "closed_gas_filter_integrity_calibration_gas_inlet_cut",
            CALIBRATION_PORT_D,
            -(BULKHEAD_X / 2.0 - 96.0),
            0.0,
        ) + bulkhead_cut(
            "closed_gas_filter_integrity_sterile_relief_exhaust_cut",
            RELIEF_PORT_D,
            BULKHEAD_X / 2.0 - 96.0,
            0.0,
        ) + centered_cube(
            "closed_gas_filter_integrity_bulkhead_cable_transit_cut",
            96.0,
            BULKHEAD_Y + 8.0,
            42.0,
        )
        .translate(
            BULKHEAD_X / 2.0 - 102.0,
            BULKHEAD_CENTER_Y,
            BULKHEAD_CENTER_Z - 138.0,
        );

    lands = lands
        + bulkhead_land(
            "closed_gas_filter_integrity_calibration_gas_inlet_land",
            15.0,
            -(BULKHEAD_X / 2.0 - 96.0),
            0.0,
        )
        + bulkhead_land(
            "closed_gas_filter_integrity_sterile_relief_exhaust_land",
            24.0,
            BULKHEAD_X / 2.0 - 96.0,
            0.0,
        )
        + bulkhead_gasket_land()
        + label_strip("source_gas_inputs", BULKHEAD_X - 112.0).translate(
            0.0,
            BULKHEAD_CENTER_Y - BULKHEAD_Y / 2.0 - 5.0,
            BULKHEAD_CENTER_Z + 134.0,
        )
        + label_strip("qualified_outputs_to_incubator_modules", BULKHEAD_X - 112.0).translate(
            0.0,
            BULKHEAD_CENTER_Y - BULKHEAD_Y / 2.0 - 5.0,
            BULKHEAD_CENTER_Z - 134.0,
        );

    panel - cuts + lands
}

fn filter_cartridge_nests() -> Part {
    let mut nests = Part::empty("closed_gas_filter_integrity_filter_cartridge_nests");
    for channel in 0..GAS_CHANNELS {
        for stage in 0..FILTERS_PER_CHANNEL {
            let x = channel_x(channel);
            let y = filter_stage_y(stage);
            let name = format!(
                "closed_gas_filter_integrity_{}_stage_{}_filter",
                GAS_NAMES[channel],
                stage + 1
            );
            nests = nests + filter_nest(&name).translate(x, y, BASE_Z / 2.0 + 28.0);
        }
    }
    nests
}

fn filter_nest(name: &str) -> Part {
    let saddle = centered_cube(
        format!("{name}_sterile_cartridge_saddle_block"),
        FILTER_NEST_BLOCK_X,
        FILTER_NEST_BLOCK_Y,
        FILTER_NEST_BLOCK_Z,
    );
    let cartridge_clearance = centered_cylinder(
        format!("{name}_cartridge_half_round_clearance"),
        FILTER_BODY_D / 2.0 + 2.0,
        FILTER_NEST_BLOCK_X + 12.0,
        36,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, FILTER_NEST_BLOCK_Z / 2.0 - 4.0);
    let cartridge_envelope = centered_cylinder(
        format!("{name}_0_2um_filter_cartridge_envelope"),
        FILTER_BODY_D / 2.0,
        FILTER_BODY_LEN,
        36,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, FILTER_NEST_BLOCK_Z / 2.0 + 8.0);
    let left_clamp = centered_cube(
        format!("{name}_left_swing_clamp"),
        FILTER_CLAMP_X,
        FILTER_CLAMP_Y,
        FILTER_CLAMP_Z,
    )
    .translate(
        -FILTER_NEST_BLOCK_X / 2.0 + 30.0,
        -FILTER_NEST_BLOCK_Y / 2.0 - 6.0,
        16.0,
    );
    let right_clamp = centered_cube(
        format!("{name}_right_swing_clamp"),
        FILTER_CLAMP_X,
        FILTER_CLAMP_Y,
        FILTER_CLAMP_Z,
    )
    .translate(
        FILTER_NEST_BLOCK_X / 2.0 - 30.0,
        -FILTER_NEST_BLOCK_Y / 2.0 - 6.0,
        16.0,
    );
    let upstream_tube =
        centered_cylinder(format!("{name}_upstream_tube_axis_gauge"), 4.0, 38.0, 20)
            .rotate(0.0, 90.0, 0.0)
            .translate(-(FILTER_NEST_BLOCK_X / 2.0 + 19.0), 0.0, 24.0);
    let downstream_tube =
        centered_cylinder(format!("{name}_downstream_tube_axis_gauge"), 4.0, 38.0, 20)
            .rotate(0.0, 90.0, 0.0)
            .translate(FILTER_NEST_BLOCK_X / 2.0 + 19.0, 0.0, 24.0);
    let keyed_stop = centered_cube(
        format!("{name}_single_direction_keyed_stop"),
        14.0,
        FILTER_NEST_BLOCK_Y + 18.0,
        22.0,
    )
    .translate(FILTER_NEST_BLOCK_X / 2.0 - 12.0, 0.0, 28.0);

    (saddle - cartridge_clearance)
        + cartridge_envelope
        + left_clamp
        + right_clamp
        + upstream_tube
        + downstream_tube
        + keyed_stop
}

fn orientation_gauge_bank() -> Part {
    let plate = centered_cube(
        "closed_gas_filter_integrity_orientation_gauge_backer_plate",
        ORIENTATION_BANK_X,
        ORIENTATION_BANK_Y,
        ORIENTATION_BANK_Z,
    )
    .translate(
        0.0,
        -BASE_Y / 2.0 + 190.0,
        BASE_Z / 2.0 + ORIENTATION_BANK_Z / 2.0,
    );

    let mut gauges = Part::empty("closed_gas_filter_integrity_orientation_gauges");
    for channel in 0..GAS_CHANNELS {
        let x = channel_x(channel);
        gauges = gauges
            + channel_orientation_gauge(GAS_NAMES[channel], 0).translate(
                x - 42.0,
                -BASE_Y / 2.0 + 144.0,
                BASE_Z / 2.0 + 72.0,
            )
            + channel_orientation_gauge(GAS_NAMES[channel], 1).translate(
                x + 42.0,
                -BASE_Y / 2.0 + 144.0,
                BASE_Z / 2.0 + 72.0,
            )
            + filter_arrow_gauge(GAS_NAMES[channel]).translate(
                x,
                -BASE_Y / 2.0 + 222.0,
                BASE_Z / 2.0 + 58.0,
            );
    }

    plate + gauges
}

fn channel_orientation_gauge(gas: &str, index: usize) -> Part {
    let valve = centered_cylinder(
        format!("closed_gas_filter_integrity_{gas}_check_valve_{index}_body"),
        CHECK_VALVE_D / 2.0,
        CHECK_VALVE_LEN,
        28,
    )
    .rotate(0.0, 90.0, 0.0);
    let seat = centered_cube(
        format!("closed_gas_filter_integrity_{gas}_check_valve_{index}_seat"),
        CHECK_VALVE_LEN + 24.0,
        24.0,
        12.0,
    )
    .translate(0.0, 0.0, -17.0);
    let arrow = flow_arrow(format!(
        "closed_gas_filter_integrity_{gas}_check_valve_{index}_flow_arrow"
    ))
    .translate(0.0, -21.0, 14.0);
    let upstream_pin = centered_cylinder(
        format!("closed_gas_filter_integrity_{gas}_check_valve_{index}_upstream_pin"),
        4.0,
        16.0,
        18,
    )
    .translate(-(CHECK_VALVE_LEN / 2.0 + 8.0), 0.0, 10.0);
    let downstream_pin = centered_cylinder(
        format!("closed_gas_filter_integrity_{gas}_check_valve_{index}_downstream_pin"),
        6.0,
        16.0,
        20,
    )
    .translate(CHECK_VALVE_LEN / 2.0 + 8.0, 0.0, 10.0);

    seat + valve + arrow + upstream_pin + downstream_pin
}

fn filter_arrow_gauge(gas: &str) -> Part {
    let strip = centered_cube(
        format!("closed_gas_filter_integrity_{gas}_filter_orientation_strip"),
        156.0,
        10.0,
        8.0,
    );
    let stage_1_arrow = flow_arrow(format!(
        "closed_gas_filter_integrity_{gas}_stage_1_filter_arrow"
    ))
    .translate(-42.0, -14.0, 8.0);
    let stage_2_arrow = flow_arrow(format!(
        "closed_gas_filter_integrity_{gas}_stage_2_filter_arrow"
    ))
    .translate(42.0, -14.0, 8.0);
    let keyed_notch = centered_cube(
        format!("closed_gas_filter_integrity_{gas}_filter_key_notch_gauge"),
        18.0,
        14.0,
        12.0,
    )
    .translate(0.0, 0.0, 12.0);

    strip + stage_1_arrow + stage_2_arrow + keyed_notch
}

fn flow_arrow(name: String) -> Part {
    let shaft = centered_cube(format!("{name}_shaft"), 34.0, 5.0, 5.0);
    let head = centered_cube(format!("{name}_head"), 14.0, 14.0, 5.0)
        .rotate(0.0, 0.0, 45.0)
        .translate(21.0, 0.0, 0.0);
    shaft + head
}

fn pressure_decay_port_manifold() -> Part {
    let body = centered_cube(
        "closed_gas_filter_integrity_pressure_decay_manifold_body",
        DECAY_MANIFOLD_X,
        DECAY_MANIFOLD_Y,
        DECAY_MANIFOLD_Z,
    )
    .translate(
        0.0,
        -BASE_Y / 2.0 + 298.0,
        BASE_Z / 2.0 + DECAY_MANIFOLD_Z / 2.0,
    );
    let main_bore = centered_cylinder(
        "closed_gas_filter_integrity_pressure_decay_common_bore",
        5.0,
        DECAY_MANIFOLD_X - 66.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -BASE_Y / 2.0 + 298.0, BASE_Z / 2.0 + 66.0);

    let mut cuts = Part::empty("closed_gas_filter_integrity_pressure_decay_port_cuts");
    let mut ports = Part::empty("closed_gas_filter_integrity_pressure_decay_quick_connects");
    let mut sensors = Part::empty("closed_gas_filter_integrity_pressure_sensor_pockets");
    for channel in 0..GAS_CHANNELS {
        let x = channel_x(channel);
        for side in 0..PRESSURE_DECAY_PORTS_PER_CHANNEL {
            let px = x + pressure_port_offset(side);
            cuts = cuts
                + centered_cylinder(
                    format!(
                        "closed_gas_filter_integrity_{}_pressure_decay_port_{}_cut",
                        GAS_NAMES[channel], side
                    ),
                    DECAY_PORT_D / 2.0,
                    DECAY_MANIFOLD_Y + 16.0,
                    24,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(px, -BASE_Y / 2.0 + 298.0, BASE_Z / 2.0 + 30.0);
            ports = ports
                + centered_cylinder(
                    format!(
                        "closed_gas_filter_integrity_{}_pressure_decay_port_{}_qc_land",
                        GAS_NAMES[channel], side
                    ),
                    13.0,
                    16.0,
                    28,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(px, -BASE_Y / 2.0 + 252.0, BASE_Z / 2.0 + 30.0);
        }
        let pocket = centered_cube(
            format!(
                "closed_gas_filter_integrity_{}_decay_transducer_pocket",
                GAS_NAMES[channel]
            ),
            SENSOR_POCKET_X,
            SENSOR_POCKET_Y,
            SENSOR_POCKET_Z,
        )
        .translate(x, -BASE_Y / 2.0 + 250.0, BASE_Z / 2.0 + 68.0);
        let face = centered_cube(
            format!(
                "closed_gas_filter_integrity_{}_transducer_face_land",
                GAS_NAMES[channel]
            ),
            SENSOR_POCKET_X + 14.0,
            8.0,
            SENSOR_POCKET_Z + 12.0,
        )
        .translate(x, -BASE_Y / 2.0 + 240.0, BASE_Z / 2.0 + 68.0);
        sensors = sensors + face - pocket;
    }

    (body - main_bore - cuts)
        + ports
        + sensors
        + label_strip(
            "pressure_decay_upstream_downstream_ports",
            DECAY_MANIFOLD_X - 72.0,
        )
        .translate(0.0, -BASE_Y / 2.0 + 252.0, BASE_Z / 2.0 + 102.0)
}

fn vent_relief_route_bank() -> Part {
    let backer = centered_cube(
        "closed_gas_filter_integrity_sterile_vent_relief_route_backer",
        VENT_BANK_X,
        VENT_BANK_Y,
        VENT_BANK_Z,
    )
    .translate(0.0, BULKHEAD_CENTER_Y - 70.0, BASE_Z / 2.0 + 158.0);
    let route_manifold = centered_cube(
        "closed_gas_filter_integrity_relief_route_manifold_placeholder",
        VENT_BANK_X - 120.0,
        34.0,
        34.0,
    )
    .translate(0.0, BULKHEAD_CENTER_Y - 104.0, BASE_Z / 2.0 + 114.0);
    let route_bore = centered_cylinder(
        "closed_gas_filter_integrity_relief_route_common_bore",
        5.5,
        VENT_BANK_X - 92.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, BULKHEAD_CENTER_Y - 104.0, BASE_Z / 2.0 + 114.0);

    let mut vents = Part::empty("closed_gas_filter_integrity_sterile_vent_filter_placeholders");
    let mut cuts = Part::empty("closed_gas_filter_integrity_vent_relief_port_cuts");
    for channel in 0..GAS_CHANNELS {
        let x = channel_x(channel);
        vents = vents
            + centered_cylinder(
                format!(
                    "closed_gas_filter_integrity_{}_sterile_vent_filter_placeholder",
                    GAS_NAMES[channel]
                ),
                VENT_FILTER_D / 2.0,
                VENT_FILTER_LEN,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, BULKHEAD_CENTER_Y - 124.0, BASE_Z / 2.0 + 166.0)
            + relief_valve_placeholder(GAS_NAMES[channel]).translate(
                x + 50.0,
                BULKHEAD_CENTER_Y - 102.0,
                BASE_Z / 2.0 + 124.0,
            );
        cuts = cuts
            + centered_cylinder(
                format!(
                    "closed_gas_filter_integrity_{}_vent_relief_panel_bore",
                    GAS_NAMES[channel]
                ),
                5.5,
                VENT_BANK_Y + 16.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, BULKHEAD_CENTER_Y - 70.0, BASE_Z / 2.0 + 166.0);
    }

    let shared_relief = relief_valve_placeholder("shared_route").translate(
        -(VENT_BANK_X / 2.0 - 76.0),
        BULKHEAD_CENTER_Y - 104.0,
        BASE_Z / 2.0 + 124.0,
    );
    let exhaust_muffler = centered_cylinder(
        "closed_gas_filter_integrity_sterile_exhaust_muffler_placeholder",
        20.0,
        96.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        VENT_BANK_X / 2.0 - 82.0,
        BULKHEAD_CENTER_Y - 124.0,
        BASE_Z / 2.0 + 116.0,
    );

    backer - cuts
        + (route_manifold - route_bore)
        + vents
        + shared_relief
        + exhaust_muffler
        + label_strip(
            "sterile_vent_relief_routes_to_safe_exhaust",
            VENT_BANK_X - 90.0,
        )
        .translate(0.0, BULKHEAD_CENTER_Y - 110.0, BASE_Z / 2.0 + 212.0)
}

fn relief_valve_placeholder(name: &str) -> Part {
    let body = centered_cylinder(
        format!("closed_gas_filter_integrity_{name}_relief_valve_body"),
        15.0,
        34.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0);
    let stem = centered_cylinder(
        format!("closed_gas_filter_integrity_{name}_relief_valve_stem"),
        5.0,
        22.0,
        20,
    )
    .translate(0.0, 0.0, 22.0);
    let knob = centered_cylinder(
        format!("closed_gas_filter_integrity_{name}_relief_valve_knob"),
        12.0,
        9.0,
        24,
    )
    .translate(0.0, 0.0, 36.0);
    body + stem + knob
}

fn calibration_source_barcode_lands() -> Part {
    let panel = centered_cube(
        "closed_gas_filter_integrity_calibration_source_barcode_panel",
        BARCODE_PANEL_X,
        BARCODE_PANEL_Y,
        BARCODE_PANEL_Z,
    )
    .translate(
        0.0,
        -BASE_Y / 2.0 + 74.0,
        BASE_Z / 2.0 + BARCODE_PANEL_Z / 2.0 + 2.0,
    );

    let mut lands = Part::empty("closed_gas_filter_integrity_source_barcode_lands");
    for channel in 0..GAS_CHANNELS {
        let x = channel_x(channel);
        lands = lands
            + barcode_land(format!(
                "closed_gas_filter_integrity_{}_source_filter_lot_barcode_land",
                GAS_NAMES[channel]
            ))
            .translate(x, -BASE_Y / 2.0 + 52.0, BASE_Z / 2.0 + 16.0);
    }

    for i in 0..CALIBRATION_BARCODE_LANDS {
        lands = lands
            + barcode_land(format!(
                "closed_gas_filter_integrity_calibration_gas_source_barcode_land_{i}"
            ))
            .translate(
                calibration_barcode_x(i),
                -BASE_Y / 2.0 + 104.0,
                BASE_Z / 2.0 + 16.0,
            );
    }

    let calibration_bottle_socket = centered_cylinder(
        "closed_gas_filter_integrity_calibration_gas_bottle_socket",
        34.0,
        18.0,
        40,
    )
    .translate(
        -(BARCODE_PANEL_X / 2.0 - 74.0),
        -BASE_Y / 2.0 + 104.0,
        BASE_Z / 2.0 + 22.0,
    );
    let calibration_qc_port = centered_cylinder(
        "closed_gas_filter_integrity_calibration_gas_qc_port_placeholder",
        11.0,
        34.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        -(BARCODE_PANEL_X / 2.0 - 142.0),
        -BASE_Y / 2.0 + 118.0,
        BASE_Z / 2.0 + 26.0,
    );

    panel + lands + calibration_bottle_socket + calibration_qc_port
}

fn barcode_land(name: String) -> Part {
    let land = centered_cube(name.clone(), BARCODE_LAND_X, BARCODE_LAND_Y, BARCODE_LAND_Z);
    let witness = centered_cube(format!("{name}_scan_window_recess"), 70.0, 38.0, 3.0).translate(
        0.0,
        0.0,
        BARCODE_LAND_Z / 2.0 - 1.0,
    );
    land - witness
}

fn release_hold_reject_lanes() -> Part {
    let panel = centered_cube(
        "closed_gas_filter_integrity_release_hold_reject_lane_panel",
        LANE_PANEL_X,
        LANE_PANEL_Y,
        LANE_PANEL_Z,
    )
    .translate(
        0.0,
        -BASE_Y / 2.0 + 94.0,
        BASE_Z / 2.0 + LANE_PANEL_Z / 2.0 + 32.0,
    );

    let mut lanes = Part::empty("closed_gas_filter_integrity_release_hold_reject_lanes");
    for index in 0..SORT_LANES {
        lanes = lanes
            + sort_lane(index).translate(
                sort_lane_x(index),
                -BASE_Y / 2.0 + 94.0,
                BASE_Z / 2.0 + 54.0,
            );
    }

    panel + lanes
}

fn sort_lane(index: usize) -> Part {
    let name = SORT_LANE_NAMES[index];
    let floor = centered_cube(
        format!("closed_gas_filter_integrity_{name}_lane_floor"),
        LANE_WIDTH_X,
        LANE_RAIL_Y,
        8.0,
    );
    let left_rail = centered_cube(
        format!("closed_gas_filter_integrity_{name}_lane_left_rail"),
        10.0,
        LANE_RAIL_Y,
        LANE_RAIL_Z,
    )
    .translate(-(LANE_WIDTH_X / 2.0 - 10.0), 0.0, LANE_RAIL_Z / 2.0);
    let right_rail = centered_cube(
        format!("closed_gas_filter_integrity_{name}_lane_right_rail"),
        10.0,
        LANE_RAIL_Y,
        LANE_RAIL_Z,
    )
    .translate(LANE_WIDTH_X / 2.0 - 10.0, 0.0, LANE_RAIL_Z / 2.0);
    let gate = centered_cube(
        format!("closed_gas_filter_integrity_{name}_lane_status_gate"),
        LANE_WIDTH_X - 36.0,
        12.0,
        LANE_GATE_Z,
    )
    .translate(0.0, LANE_RAIL_Y / 2.0 - 16.0, LANE_GATE_Z / 2.0);
    let puck_stop = centered_cylinder(
        format!("closed_gas_filter_integrity_{name}_filter_puck_stop"),
        22.0,
        8.0,
        32,
    )
    .translate(0.0, -(LANE_RAIL_Y / 2.0 - 28.0), 12.0);
    floor + left_rail + right_rail + gate + puck_stop
}

fn robot_service_keepouts() -> Part {
    let front_robot = centered_cube(
        "closed_gas_filter_integrity_front_robot_loading_keepout",
        BASE_X - 180.0,
        FRONT_ROBOT_KEEP_OUT_Y,
        14.0,
    )
    .translate(
        0.0,
        -(BASE_Y / 2.0 + FRONT_ROBOT_KEEP_OUT_Y / 2.0),
        BASE_Z / 2.0 + 7.0,
    );
    let rear_service = centered_cube(
        "closed_gas_filter_integrity_rear_bulkhead_service_keepout",
        BULKHEAD_X,
        REAR_BULKHEAD_SERVICE_KEEP_OUT_Y,
        16.0,
    )
    .translate(
        0.0,
        BASE_Y / 2.0 + REAR_BULKHEAD_SERVICE_KEEP_OUT_Y / 2.0,
        BASE_Z / 2.0 + 8.0,
    );
    let right_filter_swap = centered_cube(
        "closed_gas_filter_integrity_right_filter_swap_service_keepout",
        RIGHT_FILTER_SERVICE_KEEP_OUT_X,
        BASE_Y - 190.0,
        16.0,
    )
    .translate(
        BASE_X / 2.0 + RIGHT_FILTER_SERVICE_KEEP_OUT_X / 2.0,
        -32.0,
        BASE_Z / 2.0 + 8.0,
    );
    let top_lift = centered_cube(
        "closed_gas_filter_integrity_top_filter_lift_keepout",
        BASE_X - 260.0,
        FILTER_STAGE_PITCH_Y * 2.0 + 90.0,
        TOP_FILTER_LIFT_KEEP_OUT_Z,
    )
    .translate(
        0.0,
        FILTER_BANK_CENTER_Y,
        BASE_Z / 2.0 + TOP_FILTER_LIFT_KEEP_OUT_Z / 2.0,
    );

    front_robot + rear_service + right_filter_swap + top_lift
}

fn bulkhead_cut(name: impl Into<String>, diameter: f64, x: f64, z_offset: f64) -> Part {
    centered_cylinder(name.into(), diameter / 2.0, BULKHEAD_Y + 12.0, 28)
        .rotate(90.0, 0.0, 0.0)
        .translate(x, BULKHEAD_CENTER_Y, BULKHEAD_CENTER_Z + z_offset)
}

fn bulkhead_land(name: impl Into<String>, radius: f64, x: f64, z_offset: f64) -> Part {
    centered_cylinder(name.into(), radius, 8.0, 28)
        .rotate(90.0, 0.0, 0.0)
        .translate(
            x,
            BULKHEAD_CENTER_Y - BULKHEAD_Y / 2.0 - 4.0,
            BULKHEAD_CENTER_Z + z_offset,
        )
}

fn bulkhead_gasket_land() -> Part {
    let top = centered_cube(
        "closed_gas_filter_integrity_bulkhead_gasket_top_land",
        BULKHEAD_X - 42.0,
        5.0,
        8.0,
    )
    .translate(
        0.0,
        BULKHEAD_CENTER_Y - BULKHEAD_Y / 2.0 - 4.0,
        BULKHEAD_CENTER_Z + BULKHEAD_Z / 2.0 - 24.0,
    );
    let bottom = centered_cube(
        "closed_gas_filter_integrity_bulkhead_gasket_bottom_land",
        BULKHEAD_X - 42.0,
        5.0,
        8.0,
    )
    .translate(
        0.0,
        BULKHEAD_CENTER_Y - BULKHEAD_Y / 2.0 - 4.0,
        BULKHEAD_CENTER_Z - BULKHEAD_Z / 2.0 + 24.0,
    );
    let left = centered_cube(
        "closed_gas_filter_integrity_bulkhead_gasket_left_land",
        8.0,
        5.0,
        BULKHEAD_Z - 54.0,
    )
    .translate(
        -(BULKHEAD_X / 2.0 - 24.0),
        BULKHEAD_CENTER_Y - BULKHEAD_Y / 2.0 - 4.0,
        BULKHEAD_CENTER_Z,
    );
    let right = centered_cube(
        "closed_gas_filter_integrity_bulkhead_gasket_right_land",
        8.0,
        5.0,
        BULKHEAD_Z - 54.0,
    )
    .translate(
        BULKHEAD_X / 2.0 - 24.0,
        BULKHEAD_CENTER_Y - BULKHEAD_Y / 2.0 - 4.0,
        BULKHEAD_CENTER_Z,
    );
    top + bottom + left + right
}

fn label_strip(name: &str, width: f64) -> Part {
    let strip = centered_cube(
        format!("closed_gas_filter_integrity_label_strip_{name}"),
        width,
        4.0,
        13.0,
    );
    let tick_count = 12;
    let mut ticks = Part::empty(format!(
        "closed_gas_filter_integrity_label_strip_{name}_ticks"
    ));
    for i in 0..tick_count {
        let x = (i as f64 - (tick_count as f64 - 1.0) / 2.0) * width / tick_count as f64;
        ticks = ticks
            + centered_cube(
                format!("closed_gas_filter_integrity_label_strip_{name}_tick_{i}"),
                4.0,
                6.0,
                18.0,
            )
            .translate(x, -1.0, 0.0);
    }
    strip + ticks
}

fn channel_x(index: usize) -> f64 {
    (index as f64 - (GAS_CHANNELS as f64 - 1.0) / 2.0) * CHANNEL_PITCH_X
}

fn filter_stage_y(stage: usize) -> f64 {
    FILTER_BANK_CENTER_Y
        + (stage as f64 - (FILTERS_PER_CHANNEL as f64 - 1.0) / 2.0) * FILTER_STAGE_PITCH_Y
}

fn pressure_port_offset(side: usize) -> f64 {
    (side as f64 - 0.5) * 66.0
}

fn calibration_barcode_x(index: usize) -> f64 {
    match index {
        0 => -(BARCODE_PANEL_X / 2.0 - 210.0),
        1 => BARCODE_PANEL_X / 2.0 - 82.0,
        _ => 0.0,
    }
}

fn sort_lane_x(index: usize) -> f64 {
    (index as f64 - (SORT_LANES as f64 - 1.0) / 2.0) * LANE_PITCH_X
}

fn base_mount_points() -> [(f64, f64); 8] {
    [
        (-(BASE_X / 2.0 - 74.0), -(BASE_Y / 2.0 - 70.0)),
        (BASE_X / 2.0 - 74.0, -(BASE_Y / 2.0 - 70.0)),
        (-(BASE_X / 2.0 - 74.0), BASE_Y / 2.0 - 82.0),
        (BASE_X / 2.0 - 74.0, BASE_Y / 2.0 - 82.0),
        (-260.0, -(BASE_Y / 2.0 - 70.0)),
        (260.0, -(BASE_Y / 2.0 - 70.0)),
        (-260.0, BASE_Y / 2.0 - 82.0),
        (260.0, BASE_Y / 2.0 - 82.0),
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
        assert_eq!(OUTPUTS.len(), 10);
        for path in OUTPUTS {
            assert!(path.starts_with("output/closed_gas_filter_integrity_leak_test_station_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn all_incubator_gas_channels_are_present() {
        assert_eq!(GAS_NAMES, ["co2", "o2", "n2", "air"]);
        assert_eq!(FILTERS_PER_CHANNEL, 2);
        assert_eq!(FILTER_CARTRIDGE_NESTS, GAS_CHANNELS * 2);
        assert_eq!(CHECK_VALVE_PLACEHOLDERS, GAS_CHANNELS * 2);
        assert_eq!(
            ORIENTATION_GAUGES,
            FILTER_CARTRIDGE_NESTS + CHECK_VALVE_PLACEHOLDERS
        );
    }

    #[test]
    fn leak_test_and_safety_routes_are_counted() {
        assert_eq!(PRESSURE_DECAY_PORTS, GAS_CHANNELS * 2);
        assert_eq!(PRESSURE_SENSOR_POCKETS, GAS_CHANNELS);
        assert_eq!(VENT_FILTERS, GAS_CHANNELS);
        assert_eq!(RELIEF_ROUTE_PLACEHOLDERS, GAS_CHANNELS + 1);
        assert!(DECAY_PORT_D > 6.0);
        assert!(VENT_FILTER_D > CHECK_VALVE_D);
    }

    #[test]
    fn barcode_and_lane_evidence_is_explicit() {
        assert_eq!(SOURCE_BARCODE_LANDS, GAS_CHANNELS);
        assert_eq!(CALIBRATION_BARCODE_LANDS, 2);
        assert_eq!(TOTAL_BARCODE_LANDS, 6);
        assert_eq!(SORT_LANE_NAMES, ["release", "hold", "reject"]);
        assert_eq!(SORT_LANES, 3);
    }

    #[test]
    fn cartridge_nests_and_lanes_fit_on_the_deck() {
        assert!(channel_x(0).abs() + FILTER_NEST_BLOCK_X / 2.0 < BASE_X / 2.0 - 86.0);
        assert!(
            channel_x(GAS_CHANNELS - 1).abs() + FILTER_NEST_BLOCK_X / 2.0 < BASE_X / 2.0 - 86.0
        );
        assert!(filter_stage_y(0) - FILTER_NEST_BLOCK_Y / 2.0 > -BASE_Y / 2.0 + 170.0);
        assert!(filter_stage_y(1) + FILTER_NEST_BLOCK_Y / 2.0 < BULKHEAD_CENTER_Y - 105.0);
        assert!(sort_lane_x(0).abs() + LANE_WIDTH_X / 2.0 < LANE_PANEL_X / 2.0);
        assert!(sort_lane_x(SORT_LANES - 1).abs() + LANE_WIDTH_X / 2.0 < LANE_PANEL_X / 2.0);
    }

    #[test]
    fn bulkhead_and_keepouts_are_serviceable() {
        assert!(BULKHEAD_X < BASE_X);
        assert!(BULKHEAD_Z >= 340.0);
        assert!(BULKHEAD_CENTER_Y > 0.0);
        assert!(FRONT_ROBOT_KEEP_OUT_Y >= 440.0);
        assert!(REAR_BULKHEAD_SERVICE_KEEP_OUT_Y >= 300.0);
        assert!(RIGHT_FILTER_SERVICE_KEEP_OUT_X >= 220.0);
        assert!(TOP_FILTER_LIFT_KEEP_OUT_Z >= 240.0);
        assert_eq!(REQUIRED_FEATURE_GROUPS.len(), 9);
    }
}
