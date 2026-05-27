use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed-system chip preflush/debubble fixture for tissue-chip arrays before
// seeding or perfusion.
//
// Intent:
// - Hold a small Rev C tissue-chip array in a sealed pre-use nest.
// - Route independent priming channels through sterile connector bulkheads,
//   vertical bubble traps, pressure-limited flush bypasses, optical bubble
//   inspection forks, sensor witness points, and sealed waste capture.
// - Encode mechanical datums and safety-critical intent only. Wetted
//   materials, sterility validation, pressure controller firmware, and
//   biological release criteria remain separate gates.

const OUTPUTS: [&str; 11] = [
    "output/closed_microfluidic_chip_preflush_debubble_station_base_leak_tray.stl",
    "output/closed_microfluidic_chip_preflush_debubble_station_chip_array_clamping_nest.stl",
    "output/closed_microfluidic_chip_preflush_debubble_station_dual_priming_channel_manifold.stl",
    "output/closed_microfluidic_chip_preflush_debubble_station_vertical_bubble_trap_tower.stl",
    "output/closed_microfluidic_chip_preflush_debubble_station_pressure_limited_flush_panel.stl",
    "output/closed_microfluidic_chip_preflush_debubble_station_sterile_connector_bulkhead.stl",
    "output/closed_microfluidic_chip_preflush_debubble_station_optical_bubble_inspection_bridge.stl",
    "output/closed_microfluidic_chip_preflush_debubble_station_sealed_waste_capture_cassette.stl",
    "output/closed_microfluidic_chip_preflush_debubble_station_sensor_witness_points.stl",
    "output/closed_microfluidic_chip_preflush_debubble_station_robot_service_keepouts.stl",
    "output/closed_microfluidic_chip_preflush_debubble_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "closed_chip_array_nest",
    "dual_priming_channels",
    "vertical_bubble_trap_orientation",
    "pressure_limited_flush_bypass",
    "sterile_connector_bulkhead",
    "optical_bubble_inspection",
    "sealed_waste_capture",
    "sensor_witness_points",
    "closed_fluid_path_routing",
    "leak_containment",
    "robot_service_keepouts",
];

const STATION_X: f64 = 1040.0;
const STATION_Y: f64 = 720.0;
const DECK_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 40.0;
const WASHDOWN_GUTTER_W: f64 = 12.0;
const MOUNT_HOLE_D: f64 = 6.6;
const DATUM_PIN_D: f64 = 7.0;

const CHIP_COLS: usize = 3;
const CHIP_ROWS: usize = 2;
const CHIP_COUNT: usize = CHIP_COLS * CHIP_ROWS;
const CHIP_GUTTER: f64 = 12.0;
const CHIP_PITCH_X: f64 = REVC_CHIP_LENGTH + CHIP_GUTTER;
const CHIP_PITCH_Y: f64 = REVC_CHIP_WIDTH + CHIP_GUTTER;
const CHIP_ARRAY_X: f64 =
    CHIP_COLS as f64 * REVC_CHIP_LENGTH + (CHIP_COLS as f64 - 1.0) * CHIP_GUTTER;
const CHIP_ARRAY_Y: f64 =
    CHIP_ROWS as f64 * REVC_CHIP_WIDTH + (CHIP_ROWS as f64 - 1.0) * CHIP_GUTTER;
const CHIP_NEST_X: f64 = CHIP_ARRAY_X + 72.0;
const CHIP_NEST_Y: f64 = CHIP_ARRAY_Y + 70.0;
const CHIP_NEST_Z: f64 = 34.0;
const CHIP_NEST_CENTER: (f64, f64) = (-180.0, 70.0);
const CHIP_POCKET_CLEARANCE: f64 = 0.55;
const CHIP_POCKET_DEPTH: f64 = 9.0;
const GASKET_LAND_W: f64 = 9.0;

const FLUID_TUBE_OD: f64 = 4.8;
const TUBE_CLEARANCE: f64 = 0.8;
const FLUID_BORE_D: f64 = FLUID_TUBE_OD + TUBE_CLEARANCE;
const PRIMING_CHANNELS_PER_CHIP: usize = 2;
const PRIMING_CHANNEL_COUNT: usize = CHIP_COUNT * PRIMING_CHANNELS_PER_CHIP;
const PRIMING_CHANNEL_PITCH: f64 = 38.0;
const PRIMING_MANIFOLD_X: f64 = CHIP_NEST_X + 42.0;
const PRIMING_MANIFOLD_Y: f64 = 112.0;
const PRIMING_MANIFOLD_Z: f64 = 46.0;
const PRIMING_CENTER: (f64, f64) = (-180.0, -190.0);
const MAX_PRIMING_DEADLEG_MM: f64 = 12.0;

const BUBBLE_TRAP_COUNT: usize = CHIP_COUNT;
const BUBBLE_TRAP_CENTER: (f64, f64) = (-180.0, -54.0);
const BUBBLE_TRAP_PITCH_X: f64 = 72.0;
const BUBBLE_TRAP_D: f64 = 34.0;
const BUBBLE_TRAP_Z: f64 = 96.0;
const BUBBLE_TRAP_INTERNAL_D: f64 = 22.0;
const STERILE_FILTER_D: f64 = 20.0;
const BUBBLE_TRAP_UP_MARKERS: usize = BUBBLE_TRAP_COUNT;
const BUBBLE_TRAP_UPHILL_OFFSET_Y: f64 = 26.0;

const PRESSURE_PANEL_CENTER: (f64, f64) = (280.0, -76.0);
const PRESSURE_PANEL_X: f64 = 360.0;
const PRESSURE_PANEL_Y: f64 = 150.0;
const PRESSURE_PANEL_Z: f64 = 50.0;
const PRESSURE_RELIEF_COUNT: usize = CHIP_COUNT;
const PRESSURE_RELIEF_PITCH_X: f64 = 46.0;
const PRESSURE_LIMIT_KPA: f64 = 18.0;
const MAX_PRESEED_FLUSH_PRESSURE_KPA: f64 = 20.0;
const BYPASS_LOOP_POST_D: f64 = 13.0;

const STERILE_BULKHEAD_CENTER: (f64, f64) = (-180.0, 274.0);
const STERILE_BULKHEAD_X: f64 = 590.0;
const STERILE_BULKHEAD_Y: f64 = 74.0;
const STERILE_BULKHEAD_Z: f64 = 62.0;
const STERILE_CONNECTOR_PORTS: usize = PRIMING_CHANNEL_COUNT;
const STERILE_CONNECTOR_PITCH_X: f64 = 42.0;
const ASEPTIC_CAP_CLEARANCE_D: f64 = 14.0;
const BULKHEAD_GASKET_D: f64 = 22.0;

const OPTICAL_BRIDGE_CENTER: (f64, f64) = PRIMING_CENTER;
const OPTICAL_BRIDGE_X: f64 = 620.0;
const OPTICAL_BRIDGE_Y: f64 = 96.0;
const OPTICAL_UNDERSIDE_Z: f64 = 104.0;
const OPTICAL_BEAM_Z: f64 = 18.0;
const OPTICAL_FORK_COUNT: usize = PRIMING_CHANNEL_COUNT;
const OPTICAL_FORK_PITCH_X: f64 = PRIMING_CHANNEL_PITCH;
const OPTICAL_FORK_X: f64 = 22.0;
const OPTICAL_FORK_ARM_Y: f64 = 12.0;
const OPTICAL_FORK_Z: f64 = 30.0;

const WASTE_CENTER: (f64, f64) = (330.0, -230.0);
const WASTE_X: f64 = 280.0;
const WASTE_Y: f64 = 176.0;
const WASTE_Z: f64 = 68.0;
const WASTE_INLETS: usize = CHIP_COUNT;
const WASTE_INLET_PITCH_X: f64 = 36.0;
const WASTE_CAPTURE_VOLUME_ML: f64 = 350.0;
const WASTE_SUMP_X: f64 = 220.0;
const WASTE_SUMP_Y: f64 = 118.0;

const SENSOR_PANEL_CENTER: (f64, f64) = (250.0, 168.0);
const SENSOR_PANEL_X: f64 = 360.0;
const SENSOR_PANEL_Y: f64 = 160.0;
const SENSOR_PANEL_Z: f64 = 34.0;
const SENSOR_WITNESS_TYPES: usize = 3;
const SENSOR_WITNESS_POINTS: usize = CHIP_COUNT * SENSOR_WITNESS_TYPES;
const SENSOR_WITNESS_PITCH_X: f64 = 46.0;
const SENSOR_WITNESS_PITCH_Y: f64 = 38.0;

const ROBOT_KEEP_OUT_X: f64 = 940.0;
const ROBOT_KEEP_OUT_Y: f64 = 640.0;
const ROBOT_KEEP_OUT_Z: f64 = 160.0;
const ROBOT_KEEP_OUT_WINDOWS: usize = 5;
const FRONT_SERVICE_CLEARANCE: f64 = 260.0;
const TOP_LIFT_CLEARANCE_Z: f64 = 188.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_leak_tray();
    export(OUTPUTS[0], &base);

    let chip_nest = chip_array_clamping_nest();
    export(OUTPUTS[1], &chip_nest);

    let priming = dual_priming_channel_manifold();
    export(OUTPUTS[2], &priming);

    let traps = vertical_bubble_trap_tower();
    export(OUTPUTS[3], &traps);

    let pressure = pressure_limited_flush_panel();
    export(OUTPUTS[4], &pressure);

    let connectors = sterile_connector_bulkhead();
    export(OUTPUTS[5], &connectors);

    let optical = optical_bubble_inspection_bridge();
    export(OUTPUTS[6], &optical);

    let waste = sealed_waste_capture_cassette();
    export(OUTPUTS[7], &waste);

    let witnesses = sensor_witness_points();
    export(OUTPUTS[8], &witnesses);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[9], &keepouts);

    let assembly = base
        + chip_nest
        + priming
        + traps
        + pressure
        + connectors
        + optical
        + waste
        + witnesses
        + keepouts;
    export(OUTPUTS[10], &assembly);

    println!(
        "Closed microfluidic chip preflush/debubble station: {:.0}mm x {:.0}mm leak-tray deck, {} Rev C chips, {} sterile priming channels, {} vertical bubble traps, {:.0}kPa pressure-limit intent, {} optical inspection forks, {} sensor witness points, and {:.0}mL sealed waste capture.",
        STATION_X,
        STATION_Y,
        CHIP_COUNT,
        PRIMING_CHANNEL_COUNT,
        BUBBLE_TRAP_COUNT,
        PRESSURE_LIMIT_KPA,
        OPTICAL_FORK_COUNT,
        SENSOR_WITNESS_POINTS,
        WASTE_CAPTURE_VOLUME_ML
    );
    println!(
        "Design intent coverage: {} required feature groups, {} robot keepout windows, max priming dead-leg {:.0}mm, front service clearance {:.0}mm, top lift clearance {:.0}mm.",
        REQUIRED_FEATURES.len(),
        ROBOT_KEEP_OUT_WINDOWS,
        MAX_PRIMING_DEADLEG_MM,
        FRONT_SERVICE_CLEARANCE,
        TOP_LIFT_CLEARANCE_Z
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn base_leak_tray() -> Part {
    let deck = centered_cube(
        "closed_preflush_debubble_station_cleanable_deck",
        STATION_X,
        STATION_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let basin = centered_cube(
        "closed_preflush_debubble_station_shallow_leak_basin",
        STATION_X - RIM_W * 2.0 - WASHDOWN_GUTTER_W * 2.0,
        STATION_Y - RIM_W * 2.0 - WASHDOWN_GUTTER_W * 2.0,
        8.0,
    )
    .translate(0.0, 0.0, DECK_Z - 3.0);
    let drain = centered_cylinder(
        "closed_preflush_debubble_station_front_leak_tray_drain",
        12.0 / 2.0,
        70.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 76.0,
        -(STATION_Y / 2.0 - 18.0),
        DECK_Z - 6.0,
    );

    deck - basin - drain - mounting_holes()
        + tray_rims()
        + deck_datum_pins()
        + flow_direction_rails()
}

fn tray_rims() -> Part {
    let left = centered_cube(
        "closed_preflush_debubble_station_left_containment_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-(STATION_X / 2.0 - RIM_W / 2.0), 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "closed_preflush_debubble_station_right_containment_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        "closed_preflush_debubble_station_rear_containment_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let front = centered_cube(
        "closed_preflush_debubble_station_front_low_service_lip",
        STATION_X - 132.0,
        12.0,
        24.0,
    )
    .translate(0.0, -(STATION_Y / 2.0 - 6.0), DECK_Z + 12.0);

    left + right + rear + front
}

fn mounting_holes() -> Part {
    let mut holes = Part::empty("closed_preflush_debubble_station_mount_holes");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 44.0), -(STATION_Y / 2.0 - 44.0)),
        (STATION_X / 2.0 - 44.0, -(STATION_Y / 2.0 - 44.0)),
        (-(STATION_X / 2.0 - 44.0), STATION_Y / 2.0 - 44.0),
        (STATION_X / 2.0 - 44.0, STATION_Y / 2.0 - 44.0),
        (0.0, -(STATION_Y / 2.0 - 44.0)),
        (0.0, STATION_Y / 2.0 - 44.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("closed_preflush_debubble_station_m6_mount_hole_{i}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 6.0,
                28,
            )
            .translate(*x, *y, DECK_Z / 2.0);
    }
    holes
}

fn deck_datum_pins() -> Part {
    let mut pins = Part::empty("closed_preflush_debubble_station_deck_datum_pins");
    for (i, (x, y)) in [
        (
            CHIP_NEST_CENTER.0 - CHIP_NEST_X / 2.0 + 24.0,
            CHIP_NEST_CENTER.1 - CHIP_NEST_Y / 2.0 + 24.0,
        ),
        (
            CHIP_NEST_CENTER.0 - CHIP_NEST_X / 2.0 + 24.0,
            CHIP_NEST_CENTER.1 + CHIP_NEST_Y / 2.0 - 24.0,
        ),
        (
            CHIP_NEST_CENTER.0 + CHIP_NEST_X / 2.0 - 24.0,
            CHIP_NEST_CENTER.1 - CHIP_NEST_Y / 2.0 + 24.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        pins = pins
            + centered_cylinder(
                format!("closed_preflush_debubble_station_datum_pin_{i}"),
                DATUM_PIN_D / 2.0,
                10.0,
                28,
            )
            .translate(*x, *y, DECK_Z + 5.0);
    }
    pins
}

fn flow_direction_rails() -> Part {
    let inlet_rail = centered_cube(
        "closed_preflush_debubble_station_closed_path_inlet_direction_rail",
        520.0,
        7.0,
        8.0,
    )
    .translate(-180.0, 232.0, DECK_Z + 4.0);
    let trap_rail = centered_cube(
        "closed_preflush_debubble_station_trap_to_chip_direction_rail",
        520.0,
        7.0,
        8.0,
    )
    .translate(-180.0, -116.0, DECK_Z + 4.0);
    let waste_rail = centered_cube(
        "closed_preflush_debubble_station_chip_to_waste_direction_rail",
        380.0,
        7.0,
        8.0,
    )
    .translate(160.0, -276.0, DECK_Z + 4.0);

    inlet_rail + trap_rail + waste_rail
}

fn chip_array_clamping_nest() -> Part {
    let tray = centered_cube(
        "closed_preflush_debubble_chip_array_clamping_nest_body",
        CHIP_NEST_X,
        CHIP_NEST_Y,
        CHIP_NEST_Z,
    )
    .translate(
        CHIP_NEST_CENTER.0,
        CHIP_NEST_CENTER.1,
        DECK_Z + CHIP_NEST_Z / 2.0,
    );
    let mut cuts = Part::empty("closed_preflush_debubble_chip_array_nest_cuts");
    for row in 0..CHIP_ROWS {
        for col in 0..CHIP_COLS {
            let chip = row * CHIP_COLS + col;
            let (x, y) = chip_center(col, row);
            cuts = cuts
                + chip_pocket_cut(chip).translate(x, y, 0.0)
                + chip_optical_window_cut(chip).translate(x, y, 0.0)
                + chip_inlet_outlet_clearances(chip).translate(x, y, 0.0);
        }
    }

    tray - cuts + gasket_lands() + chip_hold_down_clamps() + closed_cassette_handle_tabs()
}

fn chip_pocket_cut(chip: usize) -> Part {
    centered_cube(
        format!("closed_preflush_debubble_chip_{chip}_sealed_pocket_recess"),
        REVC_CHIP_LENGTH + CHIP_POCKET_CLEARANCE * 2.0,
        REVC_CHIP_WIDTH + CHIP_POCKET_CLEARANCE * 2.0,
        CHIP_POCKET_DEPTH + 0.3,
    )
    .translate(
        0.0,
        0.0,
        DECK_Z + CHIP_NEST_Z - CHIP_POCKET_DEPTH / 2.0 + 0.15,
    )
}

fn chip_optical_window_cut(chip: usize) -> Part {
    centered_cube(
        format!("closed_preflush_debubble_chip_{chip}_underside_bubble_view_window"),
        REVC_CHIP_LENGTH - 34.0,
        REVC_CHIP_WIDTH - 30.0,
        CHIP_NEST_Z + 4.0,
    )
    .translate(0.0, 0.0, DECK_Z + CHIP_NEST_Z / 2.0)
}

fn chip_inlet_outlet_clearances(chip: usize) -> Part {
    let inlet = centered_cylinder(
        format!("closed_preflush_debubble_chip_{chip}_inlet_tube_clearance"),
        FLUID_BORE_D / 2.0,
        42.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        -(REVC_CHIP_LENGTH / 2.0 - 20.0),
        REVC_CHIP_WIDTH / 2.0 + 8.0,
        DECK_Z + CHIP_NEST_Z - 12.0,
    );
    let outlet = centered_cylinder(
        format!("closed_preflush_debubble_chip_{chip}_outlet_tube_clearance"),
        FLUID_BORE_D / 2.0,
        42.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        REVC_CHIP_LENGTH / 2.0 - 20.0,
        -(REVC_CHIP_WIDTH / 2.0 + 8.0),
        DECK_Z + CHIP_NEST_Z - 12.0,
    );
    inlet + outlet
}

fn gasket_lands() -> Part {
    let mut lands = Part::empty("closed_preflush_debubble_chip_gasket_compression_lands");
    for row in 0..CHIP_ROWS {
        for col in 0..CHIP_COLS {
            let chip = row * CHIP_COLS + col;
            let (x, y) = chip_center(col, row);
            let front = centered_cube(
                format!("closed_preflush_debubble_chip_{chip}_front_gasket_land"),
                REVC_CHIP_LENGTH + 8.0,
                GASKET_LAND_W,
                5.0,
            )
            .translate(
                x,
                y - REVC_CHIP_WIDTH / 2.0 - 8.0,
                DECK_Z + CHIP_NEST_Z + 2.5,
            );
            let rear = centered_cube(
                format!("closed_preflush_debubble_chip_{chip}_rear_gasket_land"),
                REVC_CHIP_LENGTH + 8.0,
                GASKET_LAND_W,
                5.0,
            )
            .translate(
                x,
                y + REVC_CHIP_WIDTH / 2.0 + 8.0,
                DECK_Z + CHIP_NEST_Z + 2.5,
            );
            lands = lands + front + rear;
        }
    }
    lands
}

fn chip_hold_down_clamps() -> Part {
    let mut clamps = Part::empty("closed_preflush_debubble_chip_array_hold_down_clamps");
    for row in 0..CHIP_ROWS {
        for col in 0..CHIP_COLS {
            let chip = row * CHIP_COLS + col;
            let (x, y) = chip_center(col, row);
            for (corner, sx, sy) in [
                ("front_left", -1.0, -1.0),
                ("front_right", 1.0, -1.0),
                ("rear_left", -1.0, 1.0),
                ("rear_right", 1.0, 1.0),
            ] {
                let ear = centered_cube(
                    format!("closed_preflush_debubble_chip_{chip}_{corner}_clamp_ear"),
                    28.0,
                    18.0,
                    8.0,
                )
                .translate(
                    x + sx * (REVC_CHIP_LENGTH / 2.0 - 14.0),
                    y + sy * (REVC_CHIP_WIDTH / 2.0 + 3.0),
                    DECK_Z + CHIP_NEST_Z + 4.0,
                );
                let screw = centered_cylinder(
                    format!("closed_preflush_debubble_chip_{chip}_{corner}_clamp_screw"),
                    3.4 / 2.0,
                    10.0,
                    20,
                )
                .translate(
                    x + sx * (REVC_CHIP_LENGTH / 2.0 - 14.0),
                    y + sy * (REVC_CHIP_WIDTH / 2.0 + 3.0),
                    DECK_Z + CHIP_NEST_Z + 4.0,
                );
                clamps = clamps + (ear - screw);
            }
        }
    }
    clamps
}

fn closed_cassette_handle_tabs() -> Part {
    let left = centered_cube(
        "closed_preflush_debubble_chip_array_left_robot_handle_tab",
        24.0,
        92.0,
        18.0,
    )
    .translate(
        CHIP_NEST_CENTER.0 - CHIP_NEST_X / 2.0 - 12.0,
        CHIP_NEST_CENTER.1,
        DECK_Z + CHIP_NEST_Z / 2.0,
    );
    let right = centered_cube(
        "closed_preflush_debubble_chip_array_right_robot_handle_tab",
        24.0,
        92.0,
        18.0,
    )
    .translate(
        CHIP_NEST_CENTER.0 + CHIP_NEST_X / 2.0 + 12.0,
        CHIP_NEST_CENTER.1,
        DECK_Z + CHIP_NEST_Z / 2.0,
    );

    left + right
}

fn dual_priming_channel_manifold() -> Part {
    let body = centered_cube(
        "closed_preflush_debubble_dual_priming_channel_manifold_body",
        PRIMING_MANIFOLD_X,
        PRIMING_MANIFOLD_Y,
        PRIMING_MANIFOLD_Z,
    )
    .translate(
        PRIMING_CENTER.0,
        PRIMING_CENTER.1,
        DECK_Z + PRIMING_MANIFOLD_Z / 2.0,
    );
    let mut cuts = Part::empty("closed_preflush_debubble_priming_channel_cuts");
    for channel in 0..PRIMING_CHANNEL_COUNT {
        let x = priming_channel_x(channel);
        let bore = centered_cylinder(
            format!("closed_preflush_debubble_priming_channel_{channel}_bore"),
            FLUID_BORE_D / 2.0,
            PRIMING_MANIFOLD_Y + 12.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, PRIMING_CENTER.1, DECK_Z + PRIMING_MANIFOLD_Z / 2.0);
        let open_view_slot = centered_cube(
            format!("closed_preflush_debubble_priming_channel_{channel}_bubble_view_slot"),
            10.0,
            PRIMING_MANIFOLD_Y + 10.0,
            12.0,
        )
        .translate(x, PRIMING_CENTER.1, DECK_Z + PRIMING_MANIFOLD_Z - 6.0);
        cuts = cuts + bore + open_view_slot;
    }

    body - cuts + priming_check_valve_lands() + low_deadleg_direction_ribs()
}

fn priming_check_valve_lands() -> Part {
    let mut lands = Part::empty("closed_preflush_debubble_priming_check_valve_lands");
    for channel in 0..PRIMING_CHANNEL_COUNT {
        let x = priming_channel_x(channel);
        let inlet_land = centered_cube(
            format!("closed_preflush_debubble_priming_channel_{channel}_inlet_check_valve_land"),
            18.0,
            20.0,
            6.0,
        )
        .translate(
            x,
            PRIMING_CENTER.1 + PRIMING_MANIFOLD_Y / 2.0 - 18.0,
            DECK_Z + PRIMING_MANIFOLD_Z + 3.0,
        );
        let outlet_land = centered_cube(
            format!("closed_preflush_debubble_priming_channel_{channel}_outlet_check_valve_land"),
            18.0,
            20.0,
            6.0,
        )
        .translate(
            x,
            PRIMING_CENTER.1 - PRIMING_MANIFOLD_Y / 2.0 + 18.0,
            DECK_Z + PRIMING_MANIFOLD_Z + 3.0,
        );
        lands = lands + inlet_land + outlet_land;
    }
    lands
}

fn low_deadleg_direction_ribs() -> Part {
    let mut ribs = Part::empty("closed_preflush_debubble_low_deadleg_direction_ribs");
    for chip in 0..CHIP_COUNT {
        let x = bubble_trap_x(chip);
        let rib = centered_cube(
            format!("closed_preflush_debubble_chip_{chip}_prime_flow_direction_rib"),
            46.0,
            7.0,
            7.0,
        )
        .translate(x, PRIMING_CENTER.1, DECK_Z + PRIMING_MANIFOLD_Z + 3.5);
        ribs = ribs + rib;
    }
    ribs
}

fn vertical_bubble_trap_tower() -> Part {
    let base = centered_cube(
        "closed_preflush_debubble_vertical_bubble_trap_base_bar",
        CHIP_NEST_X + 24.0,
        58.0,
        18.0,
    )
    .translate(BUBBLE_TRAP_CENTER.0, BUBBLE_TRAP_CENTER.1, DECK_Z + 9.0);
    let mut traps = Part::empty("closed_preflush_debubble_vertical_bubble_traps");
    for trap_index in 0..BUBBLE_TRAP_COUNT {
        let x = bubble_trap_x(trap_index);
        traps = traps
            + vertical_bubble_trap(trap_index).translate(x, BUBBLE_TRAP_CENTER.1, 0.0)
            + bubble_trap_up_marker(trap_index).translate(
                x,
                BUBBLE_TRAP_CENTER.1 + BUBBLE_TRAP_UPHILL_OFFSET_Y,
                0.0,
            );
    }

    base + traps
}

fn vertical_bubble_trap(trap_index: usize) -> Part {
    let outer = centered_cylinder(
        format!("closed_preflush_debubble_vertical_trap_{trap_index}_outer_shell"),
        BUBBLE_TRAP_D / 2.0,
        BUBBLE_TRAP_Z,
        48,
    )
    .translate(0.0, 0.0, DECK_Z + BUBBLE_TRAP_Z / 2.0);
    let chamber = centered_cylinder(
        format!("closed_preflush_debubble_vertical_trap_{trap_index}_air_capture_chamber"),
        BUBBLE_TRAP_INTERNAL_D / 2.0,
        BUBBLE_TRAP_Z - 16.0,
        40,
    )
    .translate(0.0, 0.0, DECK_Z + BUBBLE_TRAP_Z / 2.0 + 2.0);
    let inlet = centered_cylinder(
        format!("closed_preflush_debubble_vertical_trap_{trap_index}_low_inlet_bore"),
        FLUID_BORE_D / 2.0,
        46.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, -10.0, DECK_Z + 30.0);
    let outlet = centered_cylinder(
        format!("closed_preflush_debubble_vertical_trap_{trap_index}_high_outlet_bore"),
        FLUID_BORE_D / 2.0,
        46.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, 10.0, DECK_Z + 58.0);
    let sight_slot = centered_cube(
        format!("closed_preflush_debubble_vertical_trap_{trap_index}_front_sight_slot"),
        12.0,
        8.0,
        BUBBLE_TRAP_Z - 28.0,
    )
    .translate(0.0, -(BUBBLE_TRAP_D / 2.0), DECK_Z + BUBBLE_TRAP_Z / 2.0);
    let sterile_filter = centered_cylinder(
        format!("closed_preflush_debubble_vertical_trap_{trap_index}_sterile_filter_cap"),
        STERILE_FILTER_D / 2.0,
        8.0,
        32,
    )
    .translate(0.0, 0.0, DECK_Z + BUBBLE_TRAP_Z + 4.0);

    outer - chamber - inlet - outlet - sight_slot + sterile_filter
}

fn bubble_trap_up_marker(trap_index: usize) -> Part {
    let stem = centered_cube(
        format!("closed_preflush_debubble_trap_{trap_index}_up_orientation_stem"),
        5.0,
        5.0,
        42.0,
    )
    .translate(0.0, 0.0, DECK_Z + BUBBLE_TRAP_Z / 2.0);
    let top = centered_cube(
        format!("closed_preflush_debubble_trap_{trap_index}_up_orientation_cap"),
        18.0,
        7.0,
        7.0,
    )
    .translate(0.0, 0.0, DECK_Z + BUBBLE_TRAP_Z + 3.5);

    stem + top
}

fn pressure_limited_flush_panel() -> Part {
    let panel = centered_cube(
        "closed_preflush_debubble_pressure_limited_flush_panel_body",
        PRESSURE_PANEL_X,
        PRESSURE_PANEL_Y,
        PRESSURE_PANEL_Z,
    )
    .translate(
        PRESSURE_PANEL_CENTER.0,
        PRESSURE_PANEL_CENTER.1,
        DECK_Z + PRESSURE_PANEL_Z / 2.0,
    );
    let mut cuts = Part::empty("closed_preflush_debubble_pressure_limit_cuts");
    for valve in 0..PRESSURE_RELIEF_COUNT {
        let x = pressure_valve_x(valve);
        let regulator_socket = centered_cylinder(
            format!("closed_preflush_debubble_pressure_regulator_{valve}_socket"),
            22.0 / 2.0,
            PRESSURE_PANEL_Z + 4.0,
            32,
        )
        .translate(
            x,
            PRESSURE_PANEL_CENTER.1 + 20.0,
            DECK_Z + PRESSURE_PANEL_Z / 2.0,
        );
        let relief_bore = centered_cylinder(
            format!("closed_preflush_debubble_pressure_relief_{valve}_bypass_bore"),
            FLUID_BORE_D / 2.0,
            PRESSURE_PANEL_Y + 10.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, PRESSURE_PANEL_CENTER.1, DECK_Z + 24.0);
        cuts = cuts + regulator_socket + relief_bore;
    }

    panel - cuts + bypass_loop_posts() + pressure_setpoint_witness_flags()
}

fn bypass_loop_posts() -> Part {
    let mut posts = Part::empty("closed_preflush_debubble_pressure_bypass_loop_posts");
    for valve in 0..PRESSURE_RELIEF_COUNT {
        let x = pressure_valve_x(valve);
        for (side, y) in [
            ("upstream", PRESSURE_PANEL_CENTER.1 + 54.0),
            ("waste_side", PRESSURE_PANEL_CENTER.1 - 54.0),
        ] {
            posts = posts
                + centered_cylinder(
                    format!("closed_preflush_debubble_pressure_valve_{valve}_{side}_loop_post"),
                    BYPASS_LOOP_POST_D / 2.0,
                    24.0,
                    28,
                )
                .translate(x, y, DECK_Z + PRESSURE_PANEL_Z + 12.0);
        }
    }
    posts
}

fn pressure_setpoint_witness_flags() -> Part {
    let mut flags = Part::empty("closed_preflush_debubble_pressure_setpoint_witness_flags");
    for valve in 0..PRESSURE_RELIEF_COUNT {
        let x = pressure_valve_x(valve);
        flags = flags
            + centered_cube(
                format!(
                    "closed_preflush_debubble_pressure_valve_{valve}_{:.0}_kpa_setpoint_flag",
                    PRESSURE_LIMIT_KPA
                ),
                28.0,
                8.0,
                18.0,
            )
            .translate(
                x,
                PRESSURE_PANEL_CENTER.1 - 18.0,
                DECK_Z + PRESSURE_PANEL_Z + 9.0,
            );
    }
    flags
}

fn sterile_connector_bulkhead() -> Part {
    let bulkhead = centered_cube(
        "closed_preflush_debubble_sterile_connector_bulkhead_body",
        STERILE_BULKHEAD_X,
        STERILE_BULKHEAD_Y,
        STERILE_BULKHEAD_Z,
    )
    .translate(
        STERILE_BULKHEAD_CENTER.0,
        STERILE_BULKHEAD_CENTER.1,
        DECK_Z + STERILE_BULKHEAD_Z / 2.0,
    );
    let mut bores = Part::empty("closed_preflush_debubble_sterile_connector_bores");
    let mut collars = Part::empty("closed_preflush_debubble_sterile_connector_collars");
    for port in 0..STERILE_CONNECTOR_PORTS {
        let x = sterile_connector_x(port);
        bores = bores
            + centered_cylinder(
                format!("closed_preflush_debubble_sterile_connector_{port}_tube_bore"),
                FLUID_BORE_D / 2.0,
                STERILE_BULKHEAD_Y + 8.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                x,
                STERILE_BULKHEAD_CENTER.1,
                DECK_Z + STERILE_BULKHEAD_Z / 2.0,
            )
            + centered_cylinder(
                format!("closed_preflush_debubble_sterile_connector_{port}_aseptic_cap_pocket"),
                ASEPTIC_CAP_CLEARANCE_D / 2.0,
                16.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                x,
                STERILE_BULKHEAD_CENTER.1 + STERILE_BULKHEAD_Y / 2.0 - 7.0,
                DECK_Z + STERILE_BULKHEAD_Z / 2.0,
            );
        collars = collars
            + centered_cylinder(
                format!("closed_preflush_debubble_sterile_connector_{port}_gasket_collar"),
                BULKHEAD_GASKET_D / 2.0,
                8.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                x,
                STERILE_BULKHEAD_CENTER.1 + STERILE_BULKHEAD_Y / 2.0 + 4.0,
                DECK_Z + STERILE_BULKHEAD_Z / 2.0,
            );
    }

    bulkhead - bores + collars + bulkhead_screw_lugs()
}

fn bulkhead_screw_lugs() -> Part {
    let mut lugs = Part::empty("closed_preflush_debubble_sterile_bulkhead_screw_lugs");
    for (i, x) in [
        STERILE_BULKHEAD_CENTER.0 - STERILE_BULKHEAD_X / 2.0 + 34.0,
        STERILE_BULKHEAD_CENTER.0 + STERILE_BULKHEAD_X / 2.0 - 34.0,
    ]
    .iter()
    .enumerate()
    {
        let lug = centered_cube(
            format!("closed_preflush_debubble_sterile_bulkhead_screw_lug_{i}"),
            34.0,
            22.0,
            16.0,
        )
        .translate(
            *x,
            STERILE_BULKHEAD_CENTER.1,
            DECK_Z + STERILE_BULKHEAD_Z + 8.0,
        );
        let screw = centered_cylinder(
            format!("closed_preflush_debubble_sterile_bulkhead_screw_{i}"),
            4.3 / 2.0,
            18.0,
            22,
        )
        .translate(
            *x,
            STERILE_BULKHEAD_CENTER.1,
            DECK_Z + STERILE_BULKHEAD_Z + 8.0,
        );
        lugs = lugs + (lug - screw);
    }
    lugs
}

fn optical_bubble_inspection_bridge() -> Part {
    let rear_post = centered_cube(
        "closed_preflush_debubble_optical_bridge_rear_post",
        18.0,
        OPTICAL_BRIDGE_Y,
        OPTICAL_UNDERSIDE_Z,
    )
    .translate(
        OPTICAL_BRIDGE_CENTER.0 - OPTICAL_BRIDGE_X / 2.0,
        OPTICAL_BRIDGE_CENTER.1,
        DECK_Z + OPTICAL_UNDERSIDE_Z / 2.0,
    );
    let front_post = centered_cube(
        "closed_preflush_debubble_optical_bridge_front_post",
        18.0,
        OPTICAL_BRIDGE_Y,
        OPTICAL_UNDERSIDE_Z,
    )
    .translate(
        OPTICAL_BRIDGE_CENTER.0 + OPTICAL_BRIDGE_X / 2.0,
        OPTICAL_BRIDGE_CENTER.1,
        DECK_Z + OPTICAL_UNDERSIDE_Z / 2.0,
    );
    let beam = centered_cube(
        "closed_preflush_debubble_optical_bubble_inspection_top_beam",
        OPTICAL_BRIDGE_X,
        20.0,
        OPTICAL_BEAM_Z,
    )
    .translate(
        OPTICAL_BRIDGE_CENTER.0,
        OPTICAL_BRIDGE_CENTER.1,
        DECK_Z + OPTICAL_UNDERSIDE_Z + OPTICAL_BEAM_Z / 2.0,
    );
    let side_rail = centered_cube(
        "closed_preflush_debubble_optical_bridge_front_cable_rail",
        OPTICAL_BRIDGE_X - 34.0,
        8.0,
        10.0,
    )
    .translate(
        OPTICAL_BRIDGE_CENTER.0,
        OPTICAL_BRIDGE_CENTER.1 - OPTICAL_BRIDGE_Y / 2.0,
        DECK_Z + OPTICAL_UNDERSIDE_Z - 10.0,
    ) + centered_cube(
        "closed_preflush_debubble_optical_bridge_rear_cable_rail",
        OPTICAL_BRIDGE_X - 34.0,
        8.0,
        10.0,
    )
    .translate(
        OPTICAL_BRIDGE_CENTER.0,
        OPTICAL_BRIDGE_CENTER.1 + OPTICAL_BRIDGE_Y / 2.0,
        DECK_Z + OPTICAL_UNDERSIDE_Z - 10.0,
    );

    rear_post + front_post + beam + side_rail + optical_forks()
}

fn optical_forks() -> Part {
    let mut forks = Part::empty("closed_preflush_debubble_optical_bubble_inspection_forks");
    for fork in 0..OPTICAL_FORK_COUNT {
        let x = optical_fork_x(fork);
        let y = OPTICAL_BRIDGE_CENTER.1;
        let z = DECK_Z + 58.0;
        let fork_body = centered_cube(
            format!("closed_preflush_debubble_optical_fork_{fork}_base"),
            OPTICAL_FORK_X,
            54.0,
            OPTICAL_FORK_Z,
        )
        .translate(x, y, z);
        let led_arm = centered_cube(
            format!("closed_preflush_debubble_optical_fork_{fork}_led_arm"),
            OPTICAL_FORK_X,
            OPTICAL_FORK_ARM_Y,
            OPTICAL_FORK_Z + 16.0,
        )
        .translate(x, y - 27.0, z + 8.0);
        let detector_arm = centered_cube(
            format!("closed_preflush_debubble_optical_fork_{fork}_detector_arm"),
            OPTICAL_FORK_X,
            OPTICAL_FORK_ARM_Y,
            OPTICAL_FORK_Z + 16.0,
        )
        .translate(x, y + 27.0, z + 8.0);
        let tube_gap = centered_cylinder(
            format!("closed_preflush_debubble_optical_fork_{fork}_tube_gap"),
            (FLUID_BORE_D + 2.0) / 2.0,
            66.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, y, z + 10.0);
        let light_path = centered_cylinder(
            format!("closed_preflush_debubble_optical_fork_{fork}_light_path"),
            2.4 / 2.0,
            66.0,
            16,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, y, z + 18.0);

        forks = forks + (fork_body + led_arm + detector_arm - tube_gap - light_path);
    }
    forks
}

fn sealed_waste_capture_cassette() -> Part {
    let cassette = centered_cube(
        "closed_preflush_debubble_sealed_waste_capture_cassette_body",
        WASTE_X,
        WASTE_Y,
        WASTE_Z,
    )
    .translate(WASTE_CENTER.0, WASTE_CENTER.1, DECK_Z + WASTE_Z / 2.0);
    let sump = centered_cube(
        "closed_preflush_debubble_sealed_waste_capture_sump",
        WASTE_SUMP_X,
        WASTE_SUMP_Y,
        WASTE_Z - 22.0,
    )
    .translate(WASTE_CENTER.0, WASTE_CENTER.1, DECK_Z + WASTE_Z / 2.0 + 8.0);
    let fill_window = centered_cube(
        "closed_preflush_debubble_sealed_waste_capture_fill_level_window",
        12.0,
        74.0,
        26.0,
    )
    .translate(
        WASTE_CENTER.0 + WASTE_X / 2.0 - 8.0,
        WASTE_CENTER.1,
        DECK_Z + WASTE_Z / 2.0,
    );
    let mut inlets = Part::empty("closed_preflush_debubble_waste_capture_inlet_bores");
    let mut collars = Part::empty("closed_preflush_debubble_waste_capture_inlet_collars");
    for inlet in 0..WASTE_INLETS {
        let x = waste_inlet_x(inlet);
        inlets = inlets
            + centered_cylinder(
                format!("closed_preflush_debubble_waste_capture_inlet_{inlet}_bore"),
                FLUID_BORE_D / 2.0,
                38.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                x,
                WASTE_CENTER.1 + WASTE_Y / 2.0 - 10.0,
                DECK_Z + WASTE_Z - 22.0,
            );
        collars = collars
            + centered_cylinder(
                format!("closed_preflush_debubble_waste_capture_inlet_{inlet}_sealed_collar"),
                13.0 / 2.0,
                8.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                x,
                WASTE_CENTER.1 + WASTE_Y / 2.0 + 4.0,
                DECK_Z + WASTE_Z - 22.0,
            );
    }
    let hydrophobic_filter_land = centered_cylinder(
        "closed_preflush_debubble_waste_capture_hydrophobic_filter_land",
        24.0 / 2.0,
        8.0,
        32,
    )
    .translate(
        WASTE_CENTER.0 - WASTE_X / 2.0 + 44.0,
        WASTE_CENTER.1 - WASTE_Y / 2.0 + 36.0,
        DECK_Z + WASTE_Z + 4.0,
    );
    let pull_tabs = centered_cube(
        "closed_preflush_debubble_waste_capture_left_pull_tab",
        46.0,
        14.0,
        16.0,
    )
    .translate(
        WASTE_CENTER.0 - WASTE_X / 2.0 + 50.0,
        WASTE_CENTER.1 - WASTE_Y / 2.0 - 7.0,
        DECK_Z + WASTE_Z / 2.0,
    ) + centered_cube(
        "closed_preflush_debubble_waste_capture_right_pull_tab",
        46.0,
        14.0,
        16.0,
    )
    .translate(
        WASTE_CENTER.0 + WASTE_X / 2.0 - 50.0,
        WASTE_CENTER.1 - WASTE_Y / 2.0 - 7.0,
        DECK_Z + WASTE_Z / 2.0,
    );

    cassette - sump - fill_window - inlets + collars + hydrophobic_filter_land + pull_tabs
}

fn sensor_witness_points() -> Part {
    let panel = centered_cube(
        "closed_preflush_debubble_sensor_witness_points_panel",
        SENSOR_PANEL_X,
        SENSOR_PANEL_Y,
        SENSOR_PANEL_Z,
    )
    .translate(
        SENSOR_PANEL_CENTER.0,
        SENSOR_PANEL_CENTER.1,
        DECK_Z + SENSOR_PANEL_Z / 2.0,
    );
    let mut pads = Part::empty("closed_preflush_debubble_sensor_witness_point_pads");
    for row in 0..SENSOR_WITNESS_TYPES {
        for chip in 0..CHIP_COUNT {
            let witness = row * CHIP_COUNT + chip;
            let x =
                SENSOR_PANEL_CENTER.0 + centered_index(chip, CHIP_COUNT, SENSOR_WITNESS_PITCH_X);
            let y = SENSOR_PANEL_CENTER.1
                + centered_index(row, SENSOR_WITNESS_TYPES, SENSOR_WITNESS_PITCH_Y);
            let pad = centered_cylinder(
                format!("closed_preflush_debubble_sensor_witness_{witness}_pad"),
                7.0,
                3.0,
                28,
            )
            .translate(x, y, DECK_Z + SENSOR_PANEL_Z + 1.5);
            let center_dimple = centered_cylinder(
                format!("closed_preflush_debubble_sensor_witness_{witness}_center_dimple"),
                1.7,
                4.0,
                16,
            )
            .translate(x, y, DECK_Z + SENSOR_PANEL_Z + 1.5);
            pads = pads + (pad - center_dimple);
        }
    }

    panel + pads + sensor_cable_tie_lands()
}

fn sensor_cable_tie_lands() -> Part {
    let mut lands = Part::empty("closed_preflush_debubble_sensor_cable_tie_lands");
    for i in 0..4 {
        let x = SENSOR_PANEL_CENTER.0 + centered_index(i, 4, 82.0);
        let land = centered_cube(
            format!("closed_preflush_debubble_sensor_cable_tie_land_{i}"),
            38.0,
            12.0,
            8.0,
        )
        .translate(
            x,
            SENSOR_PANEL_CENTER.1 - SENSOR_PANEL_Y / 2.0 + 18.0,
            DECK_Z + SENSOR_PANEL_Z + 4.0,
        );
        let slot = centered_cube(
            format!("closed_preflush_debubble_sensor_cable_tie_slot_{i}"),
            26.0,
            4.0,
            10.0,
        )
        .translate(
            x,
            SENSOR_PANEL_CENTER.1 - SENSOR_PANEL_Y / 2.0 + 18.0,
            DECK_Z + SENSOR_PANEL_Z + 4.0,
        );
        lands = lands + (land - slot);
    }
    lands
}

fn robot_service_keepouts() -> Part {
    let front = centered_cube(
        "closed_preflush_debubble_front_robot_service_keepout_edge",
        ROBOT_KEEP_OUT_X,
        8.0,
        8.0,
    )
    .translate(0.0, -ROBOT_KEEP_OUT_Y / 2.0, DECK_Z + 4.0);
    let rear = centered_cube(
        "closed_preflush_debubble_rear_sterile_connector_service_keepout_edge",
        ROBOT_KEEP_OUT_X,
        8.0,
        8.0,
    )
    .translate(0.0, ROBOT_KEEP_OUT_Y / 2.0, DECK_Z + 4.0);
    let left = centered_cube(
        "closed_preflush_debubble_left_tubing_sweep_keepout_edge",
        8.0,
        ROBOT_KEEP_OUT_Y,
        8.0,
    )
    .translate(-ROBOT_KEEP_OUT_X / 2.0, 0.0, DECK_Z + 4.0);
    let right = centered_cube(
        "closed_preflush_debubble_right_waste_service_keepout_edge",
        8.0,
        ROBOT_KEEP_OUT_Y,
        8.0,
    )
    .translate(ROBOT_KEEP_OUT_X / 2.0, 0.0, DECK_Z + 4.0);
    let top_lift = keepout_window_frame(
        "closed_preflush_debubble_top_chip_lift_keepout",
        CHIP_NEST_X + 110.0,
        CHIP_NEST_Y + 110.0,
        8.0,
    )
    .translate(
        CHIP_NEST_CENTER.0,
        CHIP_NEST_CENTER.1,
        DECK_Z + TOP_LIFT_CLEARANCE_Z,
    );
    let pressure_panel_service = centered_cube(
        "closed_preflush_debubble_pressure_panel_service_height_gauge",
        PRESSURE_PANEL_X + 44.0,
        PRESSURE_PANEL_Y + 44.0,
        8.0,
    )
    .translate(
        PRESSURE_PANEL_CENTER.0,
        PRESSURE_PANEL_CENTER.1,
        DECK_Z + ROBOT_KEEP_OUT_Z,
    );

    front + rear + left + right + top_lift + pressure_panel_service
}

fn keepout_window_frame(name: &str, x: f64, y: f64, z: f64) -> Part {
    let front =
        centered_cube(format!("{name}_front_rail"), x, 8.0, z).translate(0.0, -y / 2.0, 0.0);
    let rear = centered_cube(format!("{name}_rear_rail"), x, 8.0, z).translate(0.0, y / 2.0, 0.0);
    let left = centered_cube(format!("{name}_left_rail"), 8.0, y, z).translate(-x / 2.0, 0.0, 0.0);
    let right = centered_cube(format!("{name}_right_rail"), 8.0, y, z).translate(x / 2.0, 0.0, 0.0);

    front + rear + left + right
}

fn assert_layout() {
    assert_eq!(CHIP_COUNT, CHIP_COLS * CHIP_ROWS);
    assert_eq!(
        PRIMING_CHANNEL_COUNT,
        CHIP_COUNT * PRIMING_CHANNELS_PER_CHIP
    );
    assert_eq!(STERILE_CONNECTOR_PORTS, PRIMING_CHANNEL_COUNT);
    assert_eq!(OPTICAL_FORK_COUNT, PRIMING_CHANNEL_COUNT);
    assert_eq!(PRESSURE_RELIEF_COUNT, CHIP_COUNT);
    assert_eq!(WASTE_INLETS, CHIP_COUNT);
    assert_eq!(SENSOR_WITNESS_POINTS, CHIP_COUNT * SENSOR_WITNESS_TYPES);
    assert_eq!(BUBBLE_TRAP_UP_MARKERS, BUBBLE_TRAP_COUNT);
    assert_eq!(ROBOT_KEEP_OUT_WINDOWS, 5);
    assert!(PRESSURE_LIMIT_KPA <= MAX_PRESEED_FLUSH_PRESSURE_KPA);
    assert!(BUBBLE_TRAP_Z > BUBBLE_TRAP_D * 2.0);
    assert!(MAX_PRIMING_DEADLEG_MM <= 12.0);

    for (center, width, depth) in module_footprints() {
        assert!(fits_on_station(center, width, depth));
    }
}

fn module_footprints() -> [((f64, f64), f64, f64); 8] {
    [
        (CHIP_NEST_CENTER, CHIP_NEST_X, CHIP_NEST_Y),
        (PRIMING_CENTER, PRIMING_MANIFOLD_X, PRIMING_MANIFOLD_Y),
        (BUBBLE_TRAP_CENTER, CHIP_NEST_X + 24.0, 58.0),
        (PRESSURE_PANEL_CENTER, PRESSURE_PANEL_X, PRESSURE_PANEL_Y),
        (
            STERILE_BULKHEAD_CENTER,
            STERILE_BULKHEAD_X,
            STERILE_BULKHEAD_Y,
        ),
        (OPTICAL_BRIDGE_CENTER, OPTICAL_BRIDGE_X, OPTICAL_BRIDGE_Y),
        (WASTE_CENTER, WASTE_X, WASTE_Y),
        (SENSOR_PANEL_CENTER, SENSOR_PANEL_X, SENSOR_PANEL_Y),
    ]
}

fn fits_on_station(center: (f64, f64), width: f64, depth: f64) -> bool {
    let usable_x = STATION_X / 2.0 - RIM_W - 8.0;
    let usable_y = STATION_Y / 2.0 - RIM_W - 8.0;
    center.0.abs() + width / 2.0 <= usable_x && center.1.abs() + depth / 2.0 <= usable_y
}

fn chip_center(col: usize, row: usize) -> (f64, f64) {
    (
        CHIP_NEST_CENTER.0 + centered_index(col, CHIP_COLS, CHIP_PITCH_X),
        CHIP_NEST_CENTER.1 + centered_index(row, CHIP_ROWS, CHIP_PITCH_Y),
    )
}

fn priming_channel_x(channel: usize) -> f64 {
    PRIMING_CENTER.0 + centered_index(channel, PRIMING_CHANNEL_COUNT, PRIMING_CHANNEL_PITCH)
}

fn bubble_trap_x(trap_index: usize) -> f64 {
    BUBBLE_TRAP_CENTER.0 + centered_index(trap_index, BUBBLE_TRAP_COUNT, BUBBLE_TRAP_PITCH_X)
}

fn pressure_valve_x(valve: usize) -> f64 {
    PRESSURE_PANEL_CENTER.0 + centered_index(valve, PRESSURE_RELIEF_COUNT, PRESSURE_RELIEF_PITCH_X)
}

fn sterile_connector_x(port: usize) -> f64 {
    STERILE_BULKHEAD_CENTER.0
        + centered_index(port, STERILE_CONNECTOR_PORTS, STERILE_CONNECTOR_PITCH_X)
}

fn optical_fork_x(fork: usize) -> f64 {
    OPTICAL_BRIDGE_CENTER.0 + centered_index(fork, OPTICAL_FORK_COUNT, OPTICAL_FORK_PITCH_X)
}

fn waste_inlet_x(inlet: usize) -> f64 {
    WASTE_CENTER.0 + centered_index(inlet, WASTE_INLETS, WASTE_INLET_PITCH_X)
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 11);
        for path in OUTPUTS {
            assert!(path.starts_with("output/closed_microfluidic_chip_preflush_debubble_station_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn required_feature_coverage_matches_requested_design_intent() {
        assert_eq!(REQUIRED_FEATURES.len(), 11);
        assert!(REQUIRED_FEATURES.contains(&"dual_priming_channels"));
        assert!(REQUIRED_FEATURES.contains(&"vertical_bubble_trap_orientation"));
        assert!(REQUIRED_FEATURES.contains(&"pressure_limited_flush_bypass"));
        assert!(REQUIRED_FEATURES.contains(&"sterile_connector_bulkhead"));
        assert!(REQUIRED_FEATURES.contains(&"optical_bubble_inspection"));
        assert!(REQUIRED_FEATURES.contains(&"sealed_waste_capture"));
        assert!(REQUIRED_FEATURES.contains(&"sensor_witness_points"));
    }

    #[test]
    fn chip_array_matches_preflush_batch_scope() {
        assert_eq!(CHIP_COLS, 3);
        assert_eq!(CHIP_ROWS, 2);
        assert_eq!(CHIP_COUNT, 6);
        assert_eq!(CHIP_PITCH_X, REVC_CHIP_LENGTH + CHIP_GUTTER);
        assert_eq!(CHIP_PITCH_Y, REVC_CHIP_WIDTH + CHIP_GUTTER);
        assert!(CHIP_NEST_X > CHIP_ARRAY_X);
        assert!(CHIP_NEST_Y > CHIP_ARRAY_Y);
    }

    #[test]
    fn closed_fluid_path_counts_are_lane_consistent() {
        assert_eq!(PRIMING_CHANNEL_COUNT, CHIP_COUNT * 2);
        assert_eq!(STERILE_CONNECTOR_PORTS, PRIMING_CHANNEL_COUNT);
        assert_eq!(OPTICAL_FORK_COUNT, PRIMING_CHANNEL_COUNT);
        assert_eq!(PRESSURE_RELIEF_COUNT, CHIP_COUNT);
        assert_eq!(WASTE_INLETS, CHIP_COUNT);
        assert_eq!(SENSOR_WITNESS_POINTS, CHIP_COUNT * SENSOR_WITNESS_TYPES);
    }

    #[test]
    fn pressure_limit_and_bubble_trap_orientation_are_explicit() {
        assert!(PRESSURE_LIMIT_KPA <= MAX_PRESEED_FLUSH_PRESSURE_KPA);
        assert!(BUBBLE_TRAP_Z > BUBBLE_TRAP_D * 2.0);
        assert_eq!(BUBBLE_TRAP_UP_MARKERS, BUBBLE_TRAP_COUNT);
        assert!(BUBBLE_TRAP_UPHILL_OFFSET_Y > BUBBLE_TRAP_D / 2.0);
    }

    #[test]
    fn all_modules_fit_inside_cleanable_leak_tray() {
        assert_layout();
        for (center, width, depth) in module_footprints() {
            assert!(fits_on_station(center, width, depth));
        }
    }

    #[test]
    fn sterile_input_and_waste_capture_are_physically_separated() {
        assert!(STERILE_BULKHEAD_CENTER.1 > CHIP_NEST_CENTER.1);
        assert!(WASTE_CENTER.1 < PRIMING_CENTER.1);
        assert!(WASTE_CAPTURE_VOLUME_ML >= 300.0);
        assert!(STERILE_BULKHEAD_CENTER.1 - WASTE_CENTER.1 > CHIP_NEST_Y);
    }

    #[test]
    fn optical_and_sensor_witness_arrays_preserve_symmetry() {
        assert_eq!(
            optical_fork_x(0),
            -((OPTICAL_FORK_COUNT as f64 - 1.0) * OPTICAL_FORK_PITCH_X) / 2.0
                + OPTICAL_BRIDGE_CENTER.0
        );
        assert_eq!(priming_channel_x(0), optical_fork_x(0));
        assert_eq!(
            priming_channel_x(PRIMING_CHANNEL_COUNT - 1) - PRIMING_CENTER.0,
            -(priming_channel_x(0) - PRIMING_CENTER.0)
        );
        assert!(SENSOR_WITNESS_PITCH_X >= PRESSURE_RELIEF_PITCH_X);
    }
}
