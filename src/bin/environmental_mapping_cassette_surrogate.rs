use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Environmental mapping cassette surrogate for a 20-position tissue-chip cassette.
//
// This is a mechanical surrogate for incubator/workcell mapping runs. It keeps
// the 4x5 cassette footprint and robot datum behavior while replacing culture
// articles with thermistor, CO2, RH, O2, and logger pockets, dummy flow traces,
// drip protection, label lands, and cable strain relief.

const OUTPUTS: [&str; 9] = [
    "output/environmental_mapping_cassette_surrogate_body_frame.stl",
    "output/environmental_mapping_cassette_surrogate_sensor_pockets.stl",
    "output/environmental_mapping_cassette_surrogate_flow_dummy_channels.stl",
    "output/environmental_mapping_cassette_surrogate_cable_strain_relief.stl",
    "output/environmental_mapping_cassette_surrogate_calibration_label_lands.stl",
    "output/environmental_mapping_cassette_surrogate_humidity_drip_shields.stl",
    "output/environmental_mapping_cassette_surrogate_robot_datum_features.stl",
    "output/environmental_mapping_cassette_surrogate_edge_center_markers.stl",
    "output/environmental_mapping_cassette_surrogate_assembly.stl",
];

const COLS: usize = 4;
const ROWS: usize = 5;
const POSITION_COUNT: usize = COLS * ROWS;
const SENSOR_KIND_COUNT: usize = 5;

const GUTTER: f64 = 5.0;
const CASSETTE_MARGIN_X: f64 = 34.0;
const CASSETTE_MARGIN_Y: f64 = 34.0;
const PITCH_X: f64 = REVC_CHIP_LENGTH + GUTTER;
const PITCH_Y: f64 = REVC_CHIP_WIDTH + GUTTER;
const ARRAY_X: f64 = COLS as f64 * REVC_CHIP_LENGTH + (COLS as f64 - 1.0) * GUTTER;
const ARRAY_Y: f64 = ROWS as f64 * REVC_CHIP_WIDTH + (ROWS as f64 - 1.0) * GUTTER;
const CASSETTE_X: f64 = ARRAY_X + 2.0 * CASSETTE_MARGIN_X;
const CASSETTE_Y: f64 = ARRAY_Y + 2.0 * CASSETTE_MARGIN_Y;

const BASE_Z: f64 = 16.0;
const PERIMETER_RIM_W: f64 = 12.0;
const PERIMETER_RIM_Z: f64 = 18.0;
const GRID_RIB_W: f64 = 7.0;
const GRID_RIB_Z: f64 = 6.0;

const POCKET_PAD_X: f64 = 104.0;
const POCKET_PAD_Y: f64 = 64.0;
const POCKET_PAD_Z: f64 = 7.0;
const POCKET_RECESS_X: f64 = POCKET_PAD_X + 7.0;
const POCKET_RECESS_Y: f64 = POCKET_PAD_Y + 7.0;
const POCKET_RECESS_DEPTH: f64 = 5.0;
const SENSOR_POCKET_DEPTH: f64 = 4.2;

const FLOW_CHANNEL_W: f64 = 5.5;
const FLOW_CHANNEL_Z: f64 = 3.0;
const FLOW_MANIFOLD_W: f64 = 11.0;

const CABLE_RELIEF_Y: f64 = 32.0;
const CABLE_RELIEF_Z: f64 = 22.0;
const CABLE_CLAMP_X: f64 = 48.0;
const CABLE_PASSAGE_D: f64 = 10.0;
const SENSOR_CABLE_BUNDLE_D: f64 = 4.2;
const CABLE_CLAMP_COUNT: usize = SENSOR_KIND_COUNT;

const LABEL_LAND_X: f64 = 48.0;
const LABEL_LAND_Y: f64 = 15.0;
const LABEL_LAND_Z: f64 = 1.6;
const LABEL_STRIP_Y: f64 = 24.0;

const DRIP_SHIELD_X: f64 = 82.0;
const DRIP_SHIELD_Y: f64 = 17.0;
const DRIP_SHIELD_Z: f64 = 3.0;
const DRIP_GUTTER_W: f64 = 4.0;

const DATUM_RAIL_W: f64 = 10.0;
const DATUM_RAIL_Z: f64 = 12.0;
const DATUM_INSET: f64 = 8.0;
const DATUM_PIN_D: f64 = 6.0;
const ROBOT_FIDUCIAL_D: f64 = 12.0;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum SensorKind {
    Thermistor,
    Co2,
    Rh,
    O2,
    Logger,
}

impl SensorKind {
    fn all() -> [SensorKind; SENSOR_KIND_COUNT] {
        [
            SensorKind::Thermistor,
            SensorKind::Co2,
            SensorKind::Rh,
            SensorKind::O2,
            SensorKind::Logger,
        ]
    }

    fn label(self) -> &'static str {
        match self {
            SensorKind::Thermistor => "thermistor",
            SensorKind::Co2 => "co2",
            SensorKind::Rh => "rh",
            SensorKind::O2 => "o2",
            SensorKind::Logger => "logger",
        }
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let body = cassette_body_frame();
    export(OUTPUTS[0], &body);

    let pockets = sensor_pocket_grid();
    export(OUTPUTS[1], &pockets);

    let flow = flow_dummy_channels();
    export(OUTPUTS[2], &flow);

    let cable = cable_strain_relief();
    export(OUTPUTS[3], &cable);

    let labels = calibration_label_lands();
    export(OUTPUTS[4], &labels);

    let shields = humidity_drip_shields();
    export(OUTPUTS[5], &shields);

    let datum = robot_cassette_datum_features();
    export(OUTPUTS[6], &datum);

    let markers = edge_center_position_markers();
    export(OUTPUTS[7], &markers);

    let assembly = body + pockets + flow + cable + labels + shields + datum + markers;
    export(OUTPUTS[8], &assembly);

    println!();
    println!("Environmental mapping cassette surrogate:");
    println!(
        "  Cassette footprint:          {CASSETTE_X:.1}mm x {CASSETTE_Y:.1}mm, 4x5 positions on {PITCH_X:.1}mm x {PITCH_Y:.1}mm pitch"
    );
    println!(
        "  Sensor pocket coverage:      {POSITION_COUNT} positions, thermistor/CO2/RH/O2/logger repeated across edge and center zones"
    );
    println!(
        "  Surrogate stack height:      {:.1}mm base plus {:.1}mm sensor pads, {:.1}mm datum/drip features",
        BASE_Z,
        POCKET_PAD_Z,
        PERIMETER_RIM_Z.max(DATUM_RAIL_Z).max(DRIP_SHIELD_Z)
    );
    println!(
        "  Cable strain relief:         {CABLE_CLAMP_COUNT} clamp passages, {CABLE_PASSAGE_D:.1}mm passage for {SENSOR_CABLE_BUNDLE_D:.1}mm bundles"
    );
    println!(
        "  Datum behavior:              {:.1}mm x-datum margin, {:.1}mm y-datum margin, {:.1}mm datum pins",
        datum_margin_x(),
        datum_margin_y(),
        DATUM_PIN_D
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_design_constraints() {
    assert_eq!(
        POSITION_COUNT, 20,
        "cassette must expose 20 mapping positions"
    );
    assert_eq!(edge_position_count(), 14, "unexpected edge position count");
    assert_eq!(
        center_position_count(),
        6,
        "unexpected center position count"
    );
    assert!(
        POCKET_RECESS_X < REVC_CHIP_LENGTH - 8.0,
        "sensor pocket recess exceeds cassette position width"
    );
    assert!(
        POCKET_RECESS_Y < REVC_CHIP_WIDTH - 8.0,
        "sensor pocket recess exceeds cassette position depth"
    );
    assert!(
        REVC_TOTAL_HEIGHT <= BASE_Z,
        "surrogate base is thinner than the cassette article height"
    );
    assert!(
        cable_radial_clearance() >= 2.5,
        "strain relief cable passage is too tight"
    );
    assert!(
        datum_margin_x() >= 12.0 && datum_margin_y() >= 12.0,
        "datum rails do not fit inside cassette margins"
    );
}

fn cassette_body_frame() -> Part {
    let deck = centered_cube(
        "environmental_mapping_cassette_surrogate_base_deck",
        CASSETTE_X,
        CASSETTE_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);

    deck - pocket_recesses() - underside_lightening_pockets() - mounting_holes()
        + perimeter_rims()
        + grid_ribs()
        + cassette_corner_bumpers()
}

fn pocket_recesses() -> Part {
    let mut cuts = Part::empty("environmental_mapping_cassette_surrogate_pocket_recesses");
    for row in 0..ROWS {
        for col in 0..COLS {
            let (x, y) = chip_center(col, row);
            cuts = cuts
                + centered_cube(
                    format!("environmental_mapping_position_recess_{col}_{row}"),
                    POCKET_RECESS_X,
                    POCKET_RECESS_Y,
                    POCKET_RECESS_DEPTH,
                )
                .translate(x, y, BASE_Z - POCKET_RECESS_DEPTH / 2.0 + 0.2);
        }
    }
    cuts
}

fn underside_lightening_pockets() -> Part {
    let mut pockets = Part::empty("environmental_mapping_cassette_underside_lightening_pockets");
    for row in 0..ROWS {
        for col in 0..COLS {
            let (x, y) = chip_center(col, row);
            pockets = pockets
                + centered_cube(
                    format!("environmental_mapping_underside_weight_relief_{col}_{row}"),
                    72.0,
                    42.0,
                    6.0,
                )
                .translate(x, y, 2.5);
        }
    }
    pockets
}

fn mounting_holes() -> Part {
    let mut holes = Part::empty("environmental_mapping_cassette_mounting_holes");
    for (i, (x, y)) in mount_points().iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("environmental_mapping_cassette_m5_mount_{i}"),
                5.3 / 2.0,
                BASE_Z + 2.0,
                28,
            )
            .translate(*x, *y, BASE_Z / 2.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    let front = centered_cube(
        "environmental_mapping_cassette_front_low_rim",
        CASSETTE_X,
        PERIMETER_RIM_W,
        PERIMETER_RIM_Z,
    )
    .translate(
        0.0,
        -(CASSETTE_Y / 2.0 - PERIMETER_RIM_W / 2.0),
        BASE_Z + PERIMETER_RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "environmental_mapping_cassette_rear_cable_rim",
        CASSETTE_X,
        PERIMETER_RIM_W,
        PERIMETER_RIM_Z,
    )
    .translate(
        0.0,
        CASSETTE_Y / 2.0 - PERIMETER_RIM_W / 2.0,
        BASE_Z + PERIMETER_RIM_Z / 2.0,
    );
    let left = centered_cube(
        "environmental_mapping_cassette_left_datum_rim",
        PERIMETER_RIM_W,
        CASSETTE_Y,
        PERIMETER_RIM_Z,
    )
    .translate(
        -(CASSETTE_X / 2.0 - PERIMETER_RIM_W / 2.0),
        0.0,
        BASE_Z + PERIMETER_RIM_Z / 2.0,
    );
    let right = centered_cube(
        "environmental_mapping_cassette_right_soft_rim",
        PERIMETER_RIM_W,
        CASSETTE_Y,
        PERIMETER_RIM_Z * 0.7,
    )
    .translate(
        CASSETTE_X / 2.0 - PERIMETER_RIM_W / 2.0,
        0.0,
        BASE_Z + PERIMETER_RIM_Z * 0.35,
    );

    front + rear + left + right
}

fn grid_ribs() -> Part {
    let mut ribs = Part::empty("environmental_mapping_cassette_grid_ribs");
    for col in 0..COLS - 1 {
        let (left_x, _) = chip_center(col, 0);
        let (right_x, _) = chip_center(col + 1, 0);
        let x = (left_x + right_x) / 2.0;
        ribs = ribs
            + centered_cube(
                format!("environmental_mapping_cassette_column_rib_{col}"),
                GRID_RIB_W,
                ARRAY_Y + 18.0,
                GRID_RIB_Z,
            )
            .translate(x, 0.0, BASE_Z + GRID_RIB_Z / 2.0);
    }
    for row in 0..ROWS - 1 {
        let (_, lower_y) = chip_center(0, row);
        let (_, upper_y) = chip_center(0, row + 1);
        let y = (lower_y + upper_y) / 2.0;
        ribs = ribs
            + centered_cube(
                format!("environmental_mapping_cassette_row_rib_{row}"),
                ARRAY_X + 18.0,
                GRID_RIB_W,
                GRID_RIB_Z,
            )
            .translate(0.0, y, BASE_Z + GRID_RIB_Z / 2.0);
    }
    ribs
}

fn cassette_corner_bumpers() -> Part {
    let mut bumpers = Part::empty("environmental_mapping_cassette_corner_bumpers");
    for (i, (sx, sy)) in [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)]
        .iter()
        .enumerate()
    {
        let bumper = centered_cube(
            format!("environmental_mapping_cassette_corner_bumper_{i}"),
            30.0,
            30.0,
            8.0,
        )
        .translate(
            sx * (CASSETTE_X / 2.0 - 24.0),
            sy * (CASSETTE_Y / 2.0 - 24.0),
            BASE_Z + 4.0,
        );
        let screw_relief = centered_cylinder(
            format!("environmental_mapping_cassette_corner_bumper_screw_relief_{i}"),
            3.2 / 2.0,
            10.0,
            22,
        )
        .translate(
            sx * (CASSETTE_X / 2.0 - 24.0),
            sy * (CASSETTE_Y / 2.0 - 24.0),
            BASE_Z + 4.0,
        );
        bumpers = bumpers + (bumper - screw_relief);
    }
    bumpers
}

fn sensor_pocket_grid() -> Part {
    let mut grid = Part::empty("environmental_mapping_cassette_sensor_pocket_grid");
    for row in 0..ROWS {
        for col in 0..COLS {
            let (x, y) = chip_center(col, row);
            let pocket = sensor_pocket(col, row).translate(x, y, BASE_Z + POCKET_PAD_Z / 2.0);
            grid = grid + pocket;
        }
    }
    grid
}

fn sensor_pocket(col: usize, row: usize) -> Part {
    let kind = sensor_kind(col, row);
    let label = kind.label();
    let body = centered_cube(
        format!("environmental_mapping_{label}_pocket_pad_{col}_{row}"),
        POCKET_PAD_X,
        POCKET_PAD_Y,
        POCKET_PAD_Z,
    );
    let cable_leadout = centered_cube(
        format!("environmental_mapping_{label}_cable_leadout_{col}_{row}"),
        12.0,
        36.0,
        SENSOR_POCKET_DEPTH + 0.4,
    )
    .translate(0.0, POCKET_PAD_Y / 2.0 - 12.0, POCKET_PAD_Z / 2.0);

    body - sensor_pocket_cut(kind, col, row) - cable_leadout
        + pocket_mount_bosses(label, col, row)
        + sensor_id_key(kind, col, row)
}

fn sensor_pocket_cut(kind: SensorKind, col: usize, row: usize) -> Part {
    match kind {
        SensorKind::Thermistor => {
            let bead = centered_cylinder(
                format!("environmental_mapping_thermistor_bead_cup_{col}_{row}"),
                4.0,
                SENSOR_POCKET_DEPTH + 0.8,
                28,
            )
            .translate(-20.0, 0.0, POCKET_PAD_Z / 2.0);
            let probe = centered_cylinder(
                format!("environmental_mapping_thermistor_probe_trough_{col}_{row}"),
                1.8,
                58.0,
                20,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(8.0, 0.0, POCKET_PAD_Z / 2.0);
            bead + probe
        }
        SensorKind::Co2 => {
            let module = centered_cube(
                format!("environmental_mapping_co2_ndir_module_pocket_{col}_{row}"),
                50.0,
                34.0,
                SENSOR_POCKET_DEPTH + 0.8,
            )
            .translate(0.0, 0.0, POCKET_PAD_Z / 2.0);
            let gas_window_a = centered_cylinder(
                format!("environmental_mapping_co2_diffusion_window_a_{col}_{row}"),
                4.0,
                SENSOR_POCKET_DEPTH + 1.0,
                28,
            )
            .translate(-18.0, -12.0, POCKET_PAD_Z / 2.0);
            let gas_window_b = centered_cylinder(
                format!("environmental_mapping_co2_diffusion_window_b_{col}_{row}"),
                4.0,
                SENSOR_POCKET_DEPTH + 1.0,
                28,
            )
            .translate(18.0, -12.0, POCKET_PAD_Z / 2.0);
            module + gas_window_a + gas_window_b
        }
        SensorKind::Rh => {
            let chip = centered_cube(
                format!("environmental_mapping_rh_sensor_pocket_{col}_{row}"),
                34.0,
                25.0,
                SENSOR_POCKET_DEPTH + 0.8,
            )
            .translate(0.0, 0.0, POCKET_PAD_Z / 2.0);
            let mut louvers = Part::empty(format!(
                "environmental_mapping_rh_louver_reliefs_{col}_{row}"
            ));
            for i in 0..4 {
                louvers = louvers
                    + centered_cube(
                        format!("environmental_mapping_rh_louver_slot_{col}_{row}_{i}"),
                        44.0,
                        2.2,
                        SENSOR_POCKET_DEPTH + 1.0,
                    )
                    .translate(0.0, -15.0 + i as f64 * 8.0, POCKET_PAD_Z / 2.0);
            }
            chip + louvers
        }
        SensorKind::O2 => {
            let membrane = centered_cylinder(
                format!("environmental_mapping_o2_membrane_cup_{col}_{row}"),
                12.0,
                SENSOR_POCKET_DEPTH + 0.8,
                36,
            )
            .translate(-12.0, 0.0, POCKET_PAD_Z / 2.0);
            let board = centered_cube(
                format!("environmental_mapping_o2_board_pocket_{col}_{row}"),
                38.0,
                26.0,
                SENSOR_POCKET_DEPTH + 0.8,
            )
            .translate(16.0, 0.0, POCKET_PAD_Z / 2.0);
            membrane + board
        }
        SensorKind::Logger => {
            let logger = centered_cube(
                format!("environmental_mapping_logger_body_pocket_{col}_{row}"),
                56.0,
                38.0,
                SENSOR_POCKET_DEPTH + 0.8,
            )
            .translate(0.0, 0.0, POCKET_PAD_Z / 2.0);
            let coin_cell = centered_cylinder(
                format!("environmental_mapping_logger_coin_cell_relief_{col}_{row}"),
                10.5,
                SENSOR_POCKET_DEPTH + 1.0,
                36,
            )
            .translate(-16.0, 0.0, POCKET_PAD_Z / 2.0);
            let usb_slot = centered_cube(
                format!("environmental_mapping_logger_usb_slot_{col}_{row}"),
                10.0,
                8.0,
                SENSOR_POCKET_DEPTH + 1.0,
            )
            .translate(34.0, 0.0, POCKET_PAD_Z / 2.0);
            logger + coin_cell + usb_slot
        }
    }
}

fn pocket_mount_bosses(label: &str, col: usize, row: usize) -> Part {
    let mut bosses = Part::empty(format!(
        "environmental_mapping_{label}_pocket_mount_bosses_{col}_{row}"
    ));
    for (i, x) in [-38.0, 38.0].iter().enumerate() {
        let boss = centered_cylinder(
            format!("environmental_mapping_{label}_mount_boss_{col}_{row}_{i}"),
            4.2,
            2.4,
            24,
        )
        .translate(*x, -24.0, POCKET_PAD_Z / 2.0 + 1.2);
        let pilot = centered_cylinder(
            format!("environmental_mapping_{label}_mount_pilot_{col}_{row}_{i}"),
            1.3,
            2.8,
            18,
        )
        .translate(*x, -24.0, POCKET_PAD_Z / 2.0 + 1.2);
        bosses = bosses + (boss - pilot);
    }
    bosses
}

fn sensor_id_key(kind: SensorKind, col: usize, row: usize) -> Part {
    let label = kind.label();
    let key_count = match kind {
        SensorKind::Thermistor => 1,
        SensorKind::Co2 => 2,
        SensorKind::Rh => 3,
        SensorKind::O2 => 4,
        SensorKind::Logger => 5,
    };
    let mut keys = Part::empty(format!(
        "environmental_mapping_{label}_id_key_ticks_{col}_{row}"
    ));
    for i in 0..key_count {
        keys = keys
            + centered_cube(
                format!("environmental_mapping_{label}_id_key_tick_{col}_{row}_{i}"),
                4.0,
                9.0,
                1.2,
            )
            .translate(
                -10.0 + i as f64 * 5.0,
                POCKET_PAD_Y / 2.0 - 5.5,
                POCKET_PAD_Z / 2.0 + 0.6,
            );
    }
    keys
}

fn flow_dummy_channels() -> Part {
    let mut channels = Part::empty("environmental_mapping_cassette_flow_dummy_channels");
    for row in 0..ROWS {
        let (_, y) = chip_center(0, row);
        channels = channels
            + centered_cube(
                format!("environmental_mapping_dummy_row_flow_channel_{row}"),
                ARRAY_X + 44.0,
                FLOW_CHANNEL_W,
                FLOW_CHANNEL_Z,
            )
            .translate(0.0, y - POCKET_PAD_Y / 2.0 + 9.0, BASE_Z + 1.5);
    }
    for col in 0..COLS {
        let (x, _) = chip_center(col, 0);
        channels = channels
            + centered_cube(
                format!("environmental_mapping_dummy_column_balance_channel_{col}"),
                FLOW_CHANNEL_W,
                ARRAY_Y + 18.0,
                FLOW_CHANNEL_Z,
            )
            .translate(x + POCKET_PAD_X / 2.0 - 11.0, 0.0, BASE_Z + 1.5);
    }

    let left_manifold = centered_cube(
        "environmental_mapping_dummy_left_side_manifold",
        FLOW_MANIFOLD_W,
        ARRAY_Y + 42.0,
        FLOW_CHANNEL_Z,
    )
    .translate(-(ARRAY_X / 2.0 + 17.0), 0.0, BASE_Z + 1.5);
    let right_manifold = centered_cube(
        "environmental_mapping_dummy_right_side_manifold",
        FLOW_MANIFOLD_W,
        ARRAY_Y + 42.0,
        FLOW_CHANNEL_Z,
    )
    .translate(ARRAY_X / 2.0 + 17.0, 0.0, BASE_Z + 1.5);

    channels + left_manifold + right_manifold + dummy_flow_ports()
}

fn dummy_flow_ports() -> Part {
    let mut ports = Part::empty("environmental_mapping_dummy_flow_ports");
    for row in 0..ROWS {
        let (_, y) = chip_center(0, row);
        for (side, x) in [
            ("left", -(ARRAY_X / 2.0 + 17.0)),
            ("right", ARRAY_X / 2.0 + 17.0),
        ] {
            ports = ports
                + centered_cylinder(
                    format!("environmental_mapping_dummy_{side}_flow_port_{row}"),
                    5.5,
                    2.2,
                    28,
                )
                .translate(
                    x,
                    y - POCKET_PAD_Y / 2.0 + 9.0,
                    BASE_Z + FLOW_CHANNEL_Z + 1.1,
                );
        }
    }
    ports
}

fn cable_strain_relief() -> Part {
    let spine = centered_cube(
        "environmental_mapping_cassette_rear_cable_relief_spine",
        CASSETTE_X - 86.0,
        CABLE_RELIEF_Y,
        CABLE_RELIEF_Z,
    )
    .translate(
        0.0,
        CASSETTE_Y / 2.0 - CABLE_RELIEF_Y / 2.0 - PERIMETER_RIM_W,
        BASE_Z + CABLE_RELIEF_Z / 2.0,
    );

    let mut clamps = Part::empty("environmental_mapping_cassette_sensor_cable_clamps");
    for i in 0..CABLE_CLAMP_COUNT {
        let kind = SensorKind::all()[i];
        let x = centered_index(i, CABLE_CLAMP_COUNT, 82.0);
        clamps = clamps
            + cable_clamp(kind, i).translate(
                x,
                CASSETTE_Y / 2.0 - CABLE_RELIEF_Y / 2.0 - PERIMETER_RIM_W,
                BASE_Z + CABLE_RELIEF_Z / 2.0,
            );
    }

    spine + clamps + cable_anchor_tabs()
}

fn cable_clamp(kind: SensorKind, index: usize) -> Part {
    let label = kind.label();
    let body = centered_cube(
        format!("environmental_mapping_{label}_cable_clamp_body_{index}"),
        CABLE_CLAMP_X,
        CABLE_RELIEF_Y + 10.0,
        CABLE_RELIEF_Z,
    );
    let cable_passage = centered_cylinder(
        format!("environmental_mapping_{label}_cable_passage_{index}"),
        CABLE_PASSAGE_D / 2.0,
        CABLE_RELIEF_Y + 12.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0);
    let top_snap_slot = centered_cube(
        format!("environmental_mapping_{label}_cable_snap_slot_{index}"),
        CABLE_CLAMP_X + 2.0,
        12.0,
        CABLE_RELIEF_Z + 2.0,
    )
    .translate(0.0, 0.0, CABLE_RELIEF_Z / 2.0 - 4.0);
    let zip_tie_slot = centered_cube(
        format!("environmental_mapping_{label}_zip_tie_slot_{index}"),
        6.0,
        CABLE_RELIEF_Y + 14.0,
        4.5,
    )
    .translate(0.0, 0.0, -CABLE_RELIEF_Z / 2.0 + 5.0);

    body - cable_passage - top_snap_slot - zip_tie_slot
}

fn cable_anchor_tabs() -> Part {
    let mut tabs = Part::empty("environmental_mapping_cassette_cable_anchor_tabs");
    for (i, x) in [-(CASSETTE_X / 2.0 - 58.0), CASSETTE_X / 2.0 - 58.0]
        .iter()
        .enumerate()
    {
        let tab = centered_cube(
            format!("environmental_mapping_cable_anchor_tab_{i}"),
            42.0,
            26.0,
            8.0,
        )
        .translate(*x, CASSETTE_Y / 2.0 - 22.0, BASE_Z + CABLE_RELIEF_Z + 4.0);
        let strap = centered_cube(
            format!("environmental_mapping_cable_anchor_strap_slot_{i}"),
            28.0,
            5.0,
            9.0,
        )
        .translate(*x, CASSETTE_Y / 2.0 - 22.0, BASE_Z + CABLE_RELIEF_Z + 4.0);
        tabs = tabs + (tab - strap);
    }
    tabs
}

fn calibration_label_lands() -> Part {
    let strip = centered_cube(
        "environmental_mapping_cassette_front_calibration_label_strip",
        CASSETTE_X - 86.0,
        LABEL_STRIP_Y,
        LABEL_LAND_Z,
    )
    .translate(
        0.0,
        -(CASSETTE_Y / 2.0 - LABEL_STRIP_Y / 2.0 - PERIMETER_RIM_W),
        BASE_Z + LABEL_LAND_Z / 2.0 + 1.0,
    );

    let mut lands = Part::empty("environmental_mapping_cassette_position_label_lands");
    for row in 0..ROWS {
        for col in 0..COLS {
            let (x, y) = chip_center(col, row);
            let land = centered_cube(
                format!("environmental_mapping_label_land_p{col}_{row}"),
                LABEL_LAND_X,
                LABEL_LAND_Y,
                LABEL_LAND_Z,
            )
            .translate(
                x,
                y - POCKET_PAD_Y / 2.0 + 10.0,
                BASE_Z + POCKET_PAD_Z + LABEL_LAND_Z / 2.0,
            );
            let scan_line = centered_cube(
                format!("environmental_mapping_label_scan_relief_p{col}_{row}"),
                LABEL_LAND_X - 8.0,
                1.4,
                LABEL_LAND_Z + 0.2,
            )
            .translate(
                x,
                y - POCKET_PAD_Y / 2.0 + 4.0,
                BASE_Z + POCKET_PAD_Z + LABEL_LAND_Z / 2.0,
            );
            lands = lands + (land - scan_line);
        }
    }

    strip + lands + sensor_family_label_lands()
}

fn sensor_family_label_lands() -> Part {
    let mut lands = Part::empty("environmental_mapping_cassette_sensor_family_label_lands");
    for (i, kind) in SensorKind::all().iter().enumerate() {
        let x = centered_index(i, SENSOR_KIND_COUNT, 82.0);
        let land = centered_cube(
            format!("environmental_mapping_{}_family_label_land", kind.label()),
            62.0,
            16.0,
            LABEL_LAND_Z,
        )
        .translate(
            x,
            -(CASSETTE_Y / 2.0 - LABEL_STRIP_Y / 2.0 - PERIMETER_RIM_W),
            BASE_Z + LABEL_LAND_Z + 2.0,
        );
        lands = lands + land;
    }
    lands
}

fn humidity_drip_shields() -> Part {
    let mut shields = Part::empty("environmental_mapping_cassette_humidity_drip_shields");
    for row in 0..ROWS {
        for col in 0..COLS {
            let (x, y) = chip_center(col, row);
            shields = shields
                + drip_shield(col, row).translate(
                    x,
                    y + POCKET_PAD_Y / 2.0 - DRIP_SHIELD_Y / 2.0 - 5.0,
                    BASE_Z + POCKET_PAD_Z + DRIP_SHIELD_Z / 2.0 + 2.0,
                );
        }
    }
    shields
}

fn drip_shield(col: usize, row: usize) -> Part {
    let roof = centered_cube(
        format!("environmental_mapping_drip_shield_roof_{col}_{row}"),
        DRIP_SHIELD_X,
        DRIP_SHIELD_Y,
        DRIP_SHIELD_Z,
    );
    let front_lip = centered_cube(
        format!("environmental_mapping_drip_shield_front_lip_{col}_{row}"),
        DRIP_SHIELD_X,
        DRIP_GUTTER_W,
        DRIP_SHIELD_Z + 2.6,
    )
    .translate(0.0, -DRIP_SHIELD_Y / 2.0 + DRIP_GUTTER_W / 2.0, -1.3);
    let side_lip_left = centered_cube(
        format!("environmental_mapping_drip_shield_left_lip_{col}_{row}"),
        DRIP_GUTTER_W,
        DRIP_SHIELD_Y,
        DRIP_SHIELD_Z + 2.0,
    )
    .translate(-DRIP_SHIELD_X / 2.0 + DRIP_GUTTER_W / 2.0, 0.0, -1.0);
    let side_lip_right = centered_cube(
        format!("environmental_mapping_drip_shield_right_lip_{col}_{row}"),
        DRIP_GUTTER_W,
        DRIP_SHIELD_Y,
        DRIP_SHIELD_Z + 2.0,
    )
    .translate(DRIP_SHIELD_X / 2.0 - DRIP_GUTTER_W / 2.0, 0.0, -1.0);

    roof + front_lip + side_lip_left + side_lip_right
}

fn robot_cassette_datum_features() -> Part {
    let rear_y_datum = centered_cube(
        "environmental_mapping_cassette_rear_y_datum_rail",
        CASSETTE_X - 2.0 * (DATUM_INSET + DATUM_RAIL_W),
        DATUM_RAIL_W,
        DATUM_RAIL_Z,
    )
    .translate(
        0.0,
        CASSETTE_Y / 2.0 - DATUM_INSET - DATUM_RAIL_W / 2.0,
        BASE_Z + DATUM_RAIL_Z / 2.0,
    );
    let left_x_datum = centered_cube(
        "environmental_mapping_cassette_left_x_datum_rail",
        DATUM_RAIL_W,
        CASSETTE_Y - 2.0 * (DATUM_INSET + DATUM_RAIL_W),
        DATUM_RAIL_Z,
    )
    .translate(
        -(CASSETTE_X / 2.0 - DATUM_INSET - DATUM_RAIL_W / 2.0),
        0.0,
        BASE_Z + DATUM_RAIL_Z / 2.0,
    );
    let front_soft_stop = centered_cube(
        "environmental_mapping_cassette_front_soft_stop_rail",
        CASSETTE_X * 0.38,
        7.0,
        DATUM_RAIL_Z * 0.58,
    )
    .translate(
        -(CASSETTE_X * 0.12),
        -(CASSETTE_Y / 2.0 - DATUM_INSET - 3.5),
        BASE_Z + DATUM_RAIL_Z * 0.29,
    );
    let asymmetric_key = centered_cube(
        "environmental_mapping_cassette_asymmetric_robot_key",
        32.0,
        9.0,
        DATUM_RAIL_Z,
    )
    .translate(
        CASSETTE_X / 2.0 - DATUM_INSET - 50.0,
        -(CASSETTE_Y / 2.0 - DATUM_INSET - 4.5),
        BASE_Z + DATUM_RAIL_Z / 2.0,
    );

    rear_y_datum
        + left_x_datum
        + front_soft_stop
        + asymmetric_key
        + datum_pin_receiver_rings()
        + robot_fiducials()
}

fn datum_pin_receiver_rings() -> Part {
    let mut rings = Part::empty("environmental_mapping_cassette_datum_pin_receiver_rings");
    for (i, (x, y)) in datum_pin_points().iter().enumerate() {
        let ring = centered_cylinder(
            format!("environmental_mapping_datum_pin_receiver_ring_{i}"),
            8.0,
            3.0,
            36,
        )
        .translate(*x, *y, BASE_Z + 1.5);
        let hole = centered_cylinder(
            format!("environmental_mapping_datum_pin_receiver_cut_{i}"),
            DATUM_PIN_D / 2.0,
            3.4,
            28,
        )
        .translate(*x, *y, BASE_Z + 1.5);
        rings = rings + (ring - hole);
    }
    rings
}

fn robot_fiducials() -> Part {
    let mut fiducials = Part::empty("environmental_mapping_cassette_robot_fiducials");
    for (i, (x, y)) in robot_fiducial_points().iter().enumerate() {
        fiducials = fiducials + fiducial_target(i).translate(*x, *y, BASE_Z + 2.0);
    }
    fiducials
}

fn fiducial_target(index: usize) -> Part {
    let disk = centered_cylinder(
        format!("environmental_mapping_robot_fiducial_disk_{index}"),
        ROBOT_FIDUCIAL_D / 2.0,
        2.0,
        40,
    );
    let center = centered_cylinder(
        format!("environmental_mapping_robot_fiducial_center_{index}"),
        1.4,
        2.4,
        24,
    );
    let x_groove = centered_cube(
        format!("environmental_mapping_robot_fiducial_x_groove_{index}"),
        ROBOT_FIDUCIAL_D + 3.0,
        1.0,
        2.4,
    );
    let y_groove = centered_cube(
        format!("environmental_mapping_robot_fiducial_y_groove_{index}"),
        1.0,
        ROBOT_FIDUCIAL_D + 3.0,
        2.4,
    );
    disk - center - x_groove - y_groove
}

fn edge_center_position_markers() -> Part {
    let mut markers = Part::empty("environmental_mapping_cassette_edge_center_markers");
    for row in 0..ROWS {
        for col in 0..COLS {
            let (x, y) = chip_center(col, row);
            let marker = if is_edge_position(col, row) {
                edge_position_marker(col, row)
            } else {
                center_position_marker(col, row)
            };
            markers = markers + marker.translate(x, y, BASE_Z + POCKET_PAD_Z + 3.0);
        }
    }
    markers
}

fn edge_position_marker(col: usize, row: usize) -> Part {
    centered_cube(
        format!("environmental_mapping_edge_position_marker_{col}_{row}"),
        18.0,
        6.0,
        2.0,
    )
    .translate(-POCKET_PAD_X / 2.0 + 14.0, POCKET_PAD_Y / 2.0 - 9.0, 0.0)
}

fn center_position_marker(col: usize, row: usize) -> Part {
    let ring = centered_cylinder(
        format!("environmental_mapping_center_position_marker_ring_{col}_{row}"),
        6.0,
        2.0,
        32,
    )
    .translate(-POCKET_PAD_X / 2.0 + 14.0, POCKET_PAD_Y / 2.0 - 9.0, 0.0);
    let cut = centered_cylinder(
        format!("environmental_mapping_center_position_marker_cut_{col}_{row}"),
        2.4,
        2.2,
        24,
    )
    .translate(-POCKET_PAD_X / 2.0 + 14.0, POCKET_PAD_Y / 2.0 - 9.0, 0.0);
    ring - cut
}

fn chip_center(col: usize, row: usize) -> (f64, f64) {
    let x = -ARRAY_X / 2.0 + REVC_CHIP_LENGTH / 2.0 + col as f64 * PITCH_X;
    let y = -ARRAY_Y / 2.0 + REVC_CHIP_WIDTH / 2.0 + row as f64 * PITCH_Y;
    (x, y)
}

fn sensor_kind(col: usize, row: usize) -> SensorKind {
    match (row + 2 * col) % SENSOR_KIND_COUNT {
        0 => SensorKind::Thermistor,
        1 => SensorKind::Co2,
        2 => SensorKind::Rh,
        3 => SensorKind::O2,
        _ => SensorKind::Logger,
    }
}

fn is_edge_position(col: usize, row: usize) -> bool {
    col == 0 || col == COLS - 1 || row == 0 || row == ROWS - 1
}

fn edge_position_count() -> usize {
    let mut count = 0;
    for row in 0..ROWS {
        for col in 0..COLS {
            if is_edge_position(col, row) {
                count += 1;
            }
        }
    }
    count
}

fn center_position_count() -> usize {
    POSITION_COUNT - edge_position_count()
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn cable_radial_clearance() -> f64 {
    (CABLE_PASSAGE_D - SENSOR_CABLE_BUNDLE_D) / 2.0
}

fn datum_margin_x() -> f64 {
    CASSETTE_MARGIN_X - DATUM_INSET - DATUM_RAIL_W
}

fn datum_margin_y() -> f64 {
    CASSETTE_MARGIN_Y - DATUM_INSET - DATUM_RAIL_W
}

fn datum_pin_points() -> [(f64, f64); 3] {
    [
        (
            -(CASSETTE_X / 2.0 - DATUM_INSET - 28.0),
            CASSETTE_Y / 2.0 - DATUM_INSET - 28.0,
        ),
        (
            CASSETTE_X / 2.0 - DATUM_INSET - 28.0,
            CASSETTE_Y / 2.0 - DATUM_INSET - 28.0,
        ),
        (
            -(CASSETTE_X / 2.0 - DATUM_INSET - 28.0),
            -(CASSETTE_Y / 2.0 - DATUM_INSET - 28.0),
        ),
    ]
}

fn robot_fiducial_points() -> [(f64, f64); 3] {
    [
        (
            -(CASSETTE_X / 2.0 - DATUM_INSET - 58.0),
            CASSETTE_Y / 2.0 - DATUM_INSET - 58.0,
        ),
        (
            CASSETTE_X / 2.0 - DATUM_INSET - 58.0,
            CASSETTE_Y / 2.0 - DATUM_INSET - 58.0,
        ),
        (
            -(CASSETTE_X / 2.0 - DATUM_INSET - 58.0),
            -(CASSETTE_Y / 2.0 - DATUM_INSET - 58.0),
        ),
    ]
}

fn mount_points() -> [(f64, f64); 8] {
    [
        (-(CASSETTE_X / 2.0 - 26.0), -(CASSETTE_Y / 2.0 - 26.0)),
        (CASSETTE_X / 2.0 - 26.0, -(CASSETTE_Y / 2.0 - 26.0)),
        (-(CASSETTE_X / 2.0 - 26.0), CASSETTE_Y / 2.0 - 26.0),
        (CASSETTE_X / 2.0 - 26.0, CASSETTE_Y / 2.0 - 26.0),
        (0.0, -(CASSETTE_Y / 2.0 - 26.0)),
        (0.0, CASSETTE_Y / 2.0 - 26.0),
        (-(CASSETTE_X / 2.0 - 26.0), 0.0),
        (CASSETTE_X / 2.0 - 26.0, 0.0),
    ]
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn cassette_layout_has_twenty_positions() {
        let mut positions = BTreeSet::new();
        for row in 0..ROWS {
            for col in 0..COLS {
                positions.insert((col, row));
            }
        }

        assert_eq!(COLS, 4);
        assert_eq!(ROWS, 5);
        assert_eq!(positions.len(), 20);
        assert_eq!(POSITION_COUNT, 20);
        assert_eq!(edge_position_count() + center_position_count(), 20);
    }

    #[test]
    fn edge_and_center_positions_cover_all_sensor_families() {
        let required: BTreeSet<SensorKind> = SensorKind::all().into_iter().collect();
        let mut edge = BTreeSet::new();
        let mut center = BTreeSet::new();

        for row in 0..ROWS {
            for col in 0..COLS {
                if is_edge_position(col, row) {
                    edge.insert(sensor_kind(col, row));
                } else {
                    center.insert(sensor_kind(col, row));
                }
            }
        }

        assert_eq!(edge_position_count(), 14);
        assert_eq!(center_position_count(), 6);
        assert_eq!(edge, required);
        assert_eq!(center, required);
    }

    #[test]
    fn cable_strain_relief_leaves_clearance_for_sensor_bundles() {
        assert_eq!(CABLE_CLAMP_COUNT, SENSOR_KIND_COUNT);
        assert!(CABLE_PASSAGE_D > SENSOR_CABLE_BUNDLE_D);
        assert!(cable_radial_clearance() >= 2.5);
        assert!(CABLE_CLAMP_X >= CABLE_PASSAGE_D + 30.0);
    }

    #[test]
    fn cassette_datum_features_fit_existing_footprint() {
        assert!(datum_margin_x() >= 12.0);
        assert!(datum_margin_y() >= 12.0);
        assert!(CASSETTE_X > ARRAY_X);
        assert!(CASSETTE_Y > ARRAY_Y);
        assert!(REVC_TOTAL_HEIGHT <= BASE_Z);

        for (x, y) in datum_pin_points() {
            assert!(x.abs() + DATUM_PIN_D / 2.0 <= CASSETTE_X / 2.0 - DATUM_INSET);
            assert!(y.abs() + DATUM_PIN_D / 2.0 <= CASSETTE_Y / 2.0 - DATUM_INSET);
        }
    }
}
