use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed-cassette evaporation and osmolality-drift mapping station.
//
// This model is workflow geometry for incubator position studies. It presents
// weighed surrogate cassette nests on the same 4x5 cassette basis used by the
// culture hardware, adds humid-incubator temperature/RH logger pockets,
// condensate/drip control, media reservoir dummy pockets, barcode lands for
// mass-balance records, edge/center mapping tokens, disposition lanes, and
// robot/service clearance volumes. Analytical acceptance criteria and drift
// calculations remain outside CAD in the study protocol.

const OUTPUTS: [&str; 11] = [
    "output/closed_cassette_evaporation_mass_loss_mapping_station_base_pan.stl",
    "output/closed_cassette_evaporation_mass_loss_mapping_station_weighed_surrogate_cassette_nests.stl",
    "output/closed_cassette_evaporation_mass_loss_mapping_station_logger_pockets.stl",
    "output/closed_cassette_evaporation_mass_loss_mapping_station_condensate_shields.stl",
    "output/closed_cassette_evaporation_mass_loss_mapping_station_media_reservoir_dummy_pockets.stl",
    "output/closed_cassette_evaporation_mass_loss_mapping_station_mass_balance_barcode_lands.stl",
    "output/closed_cassette_evaporation_mass_loss_mapping_station_edge_center_position_tokens.stl",
    "output/closed_cassette_evaporation_mass_loss_mapping_station_drip_capture.stl",
    "output/closed_cassette_evaporation_mass_loss_mapping_station_release_hold_reject_lanes.stl",
    "output/closed_cassette_evaporation_mass_loss_mapping_station_robot_service_keepouts.stl",
    "output/closed_cassette_evaporation_mass_loss_mapping_station_assembly.stl",
];

const CASSETTE_COLS: usize = 4;
const CASSETTE_ROWS: usize = 5;
const CASSETTE_POSITION_COUNT: usize = CASSETTE_COLS * CASSETTE_ROWS;
const EDGE_POSITION_COUNT: usize = 14;
const CENTER_POSITION_COUNT: usize = CASSETTE_POSITION_COUNT - EDGE_POSITION_COUNT;
const LOGGER_POCKET_COUNT: usize = 6;
const MEDIA_RESERVOIR_DUMMY_COUNT: usize = 8;
const DISPOSITION_LANE_COUNT: usize = 3;

const CHIP_GUTTER: f64 = 5.0;
const CASSETTE_MARGIN_X: f64 = 34.0;
const CASSETTE_MARGIN_Y: f64 = 34.0;
const CASSETTE_SURROGATE_Z: f64 = 38.0;
const CASSETTE_SEAL_LID_Z: f64 = 6.0;

const ARRAY_X: f64 =
    CASSETTE_COLS as f64 * REVC_CHIP_LENGTH + (CASSETTE_COLS as f64 - 1.0) * CHIP_GUTTER;
const ARRAY_Y: f64 =
    CASSETTE_ROWS as f64 * REVC_CHIP_WIDTH + (CASSETTE_ROWS as f64 - 1.0) * CHIP_GUTTER;
const CASSETTE_X: f64 = ARRAY_X + 2.0 * CASSETTE_MARGIN_X;
const CASSETTE_Y: f64 = ARRAY_Y + 2.0 * CASSETTE_MARGIN_Y;

const TRAY_X: f64 = 1160.0;
const TRAY_Y: f64 = 760.0;
const BASE_Z: f64 = 26.0;
const PERIMETER_RIM_W: f64 = 18.0;
const PERIMETER_RIM_Z: f64 = 32.0;
const MOUNT_HOLE_D: f64 = 5.4;

const NEST_CENTER_X: f64 = -64.0;
const NEST_CENTER_Y: f64 = 52.0;
const NEST_SOCKET_DEPTH: f64 = 5.0;
const NEST_RAIL_W: f64 = 14.0;
const NEST_RAIL_Z: f64 = 22.0;
const NEST_LEDGE_W: f64 = 20.0;
const NEST_LEDGE_Z: f64 = 7.0;
const WEIGH_PAD_X: f64 = 82.0;
const WEIGH_PAD_Y: f64 = 48.0;
const WEIGH_PAD_Z: f64 = 5.0;
const WEIGH_PAD_RELIEF_DEPTH: f64 = 2.2;
const MICROBALANCE_PAD_D: f64 = 13.0;

const LOGGER_BANK_CENTER_X: f64 = -420.0;
const LOGGER_BANK_CENTER_Y: f64 = -184.0;
const LOGGER_BANK_X: f64 = 270.0;
const LOGGER_BANK_Y: f64 = 236.0;
const LOGGER_BANK_Z: f64 = 34.0;
const LOGGER_POCKET_X: f64 = 68.0;
const LOGGER_POCKET_Y: f64 = 42.0;
const LOGGER_POCKET_DEPTH: f64 = 18.0;
const LOGGER_PITCH_X: f64 = 88.0;
const LOGGER_PITCH_Y: f64 = 72.0;
const LOGGER_CABLE_SLOT_W: f64 = 9.0;
const LOGGER_CABLE_SLOT_Y: f64 = 54.0;

const MEDIA_BANK_CENTER_X: f64 = -420.0;
const MEDIA_BANK_CENTER_Y: f64 = 174.0;
const MEDIA_BANK_X: f64 = 270.0;
const MEDIA_BANK_Y: f64 = 250.0;
const MEDIA_BANK_Z: f64 = 36.0;
const RESERVOIR_POCKET_X: f64 = 76.0;
const RESERVOIR_POCKET_Y: f64 = 48.0;
const RESERVOIR_POCKET_DEPTH: f64 = 20.0;
const RESERVOIR_PITCH_X: f64 = 92.0;
const RESERVOIR_PITCH_Y: f64 = 78.0;
const DUMMY_FILL_LEVEL_Z: f64 = 4.0;

const BARCODE_STRIP_CENTER_X: f64 = 282.0;
const BARCODE_STRIP_CENTER_Y: f64 = 222.0;
const BARCODE_STRIP_X: f64 = 360.0;
const BARCODE_STRIP_Y: f64 = 170.0;
const BARCODE_STRIP_Z: f64 = 22.0;
const BARCODE_LAND_X: f64 = 54.0;
const BARCODE_LAND_Y: f64 = 18.0;
const BARCODE_LAND_Z: f64 = 3.0;
const BARCODE_LAND_COUNT: usize = CASSETTE_POSITION_COUNT + LOGGER_POCKET_COUNT + 1;

const TOKEN_D: f64 = 12.0;
const EDGE_TOKEN_Z: f64 = 3.0;
const CENTER_TOKEN_Z: f64 = 5.0;

const CONDENSATE_SHIELD_Z: f64 = 4.0;
const SHIELD_OVERHANG_X: f64 = 22.0;
const SHIELD_OVERHANG_Y: f64 = 18.0;
const SHIELD_STANDOFF_Z: f64 = 18.0;
const SHIELD_GAP_Z: f64 = SHIELD_STANDOFF_Z - CASSETTE_SEAL_LID_Z;
const DRIP_GUTTER_W: f64 = 13.0;
const DRIP_GUTTER_DEPTH: f64 = 6.0;
const DRIP_CAPTURE_BASIN_X: f64 = CASSETTE_X + 96.0;
const DRIP_CAPTURE_BASIN_Y: f64 = 78.0;
const DRIP_CAPTURE_BASIN_DEPTH: f64 = 14.0;
const DRAIN_PORT_D: f64 = 8.0;

const LANE_CENTER_X: f64 = 324.0;
const LANE_CENTER_Y: f64 = -226.0;
const LANE_BANK_X: f64 = 398.0;
const LANE_BANK_Y: f64 = 236.0;
const LANE_BANK_Z: f64 = 34.0;
const LANE_W: f64 = 104.0;
const LANE_Y: f64 = 186.0;
const LANE_RIB_W: f64 = 9.0;
const LANE_RECESS_DEPTH: f64 = 13.0;
const RELEASE_TOKEN_COUNT: usize = 8;
const HOLD_TOKEN_COUNT: usize = 8;
const REJECT_TOKEN_COUNT: usize = 4;

const ROBOT_FRONT_KEEP_OUT_Y: f64 = 250.0;
const ROBOT_SIDE_KEEP_OUT_X: f64 = 92.0;
const ROBOT_PICK_CLEARANCE_Z: f64 = 116.0;
const SERVICE_REAR_KEEP_OUT_Y: f64 = 180.0;
const SERVICE_LOGGER_PULL_X: f64 = 128.0;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum MapZone {
    Edge,
    Center,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DispositionLane {
    Release,
    Hold,
    Reject,
}

impl DispositionLane {
    fn all() -> [DispositionLane; DISPOSITION_LANE_COUNT] {
        [
            DispositionLane::Release,
            DispositionLane::Hold,
            DispositionLane::Reject,
        ]
    }

    fn label(self) -> &'static str {
        match self {
            DispositionLane::Release => "release",
            DispositionLane::Hold => "hold",
            DispositionLane::Reject => "reject",
        }
    }

    fn index(self) -> usize {
        match self {
            DispositionLane::Release => 0,
            DispositionLane::Hold => 1,
            DispositionLane::Reject => 2,
        }
    }

    fn nominal_capacity(self) -> usize {
        match self {
            DispositionLane::Release => RELEASE_TOKEN_COUNT,
            DispositionLane::Hold => HOLD_TOKEN_COUNT,
            DispositionLane::Reject => REJECT_TOKEN_COUNT,
        }
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_pan();
    export(OUTPUTS[0], &base);

    let nests = weighed_surrogate_cassette_nests();
    export(OUTPUTS[1], &nests);

    let loggers = humidity_temperature_logger_pockets();
    export(OUTPUTS[2], &loggers);

    let shields = condensate_shields();
    export(OUTPUTS[3], &shields);

    let reservoirs = media_reservoir_dummy_pockets();
    export(OUTPUTS[4], &reservoirs);

    let barcodes = mass_balance_barcode_lands();
    export(OUTPUTS[5], &barcodes);

    let tokens = edge_center_position_tokens();
    export(OUTPUTS[6], &tokens);

    let drip = drip_capture();
    export(OUTPUTS[7], &drip);

    let lanes = release_hold_reject_lanes();
    export(OUTPUTS[8], &lanes);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[9], &keepouts);

    let assembly =
        base + nests + loggers + shields + reservoirs + barcodes + tokens + drip + lanes + keepouts;
    export(OUTPUTS[10], &assembly);

    println!();
    println!("Closed cassette evaporation mass-loss mapping station:");
    println!("  Station footprint:           {TRAY_X:.0}mm x {TRAY_Y:.0}mm x {BASE_Z:.0}mm base");
    println!(
        "  Cassette map:                {CASSETTE_COLS}x{CASSETTE_ROWS} weighed surrogate cassette nests, {EDGE_POSITION_COUNT} edge / {CENTER_POSITION_COUNT} center positions"
    );
    println!(
        "  Incubator study aids:        {LOGGER_POCKET_COUNT} humidity/temperature logger pockets and {MEDIA_RESERVOIR_DUMMY_COUNT} media reservoir dummy pockets"
    );
    println!(
        "  Mass balance traceability:   {BARCODE_LAND_COUNT} barcode lands with sealed-cassette weigh pads and microbalance targets"
    );
    println!(
        "  Condensate control:          {SHIELD_GAP_Z:.1}mm shield gap, {DRIP_GUTTER_W:.1}mm gutters, {DRIP_CAPTURE_BASIN_DEPTH:.1}mm front capture basin"
    );
    println!(
        "  Disposition workflow:        release/hold/reject lanes hold {} study tokens",
        total_disposition_capacity()
    );
    println!(
        "  Robot/service keepouts:      {ROBOT_PICK_CLEARANCE_Z:.0}mm pick clearance, {ROBOT_SIDE_KEEP_OUT_X:.0}mm side corridors, {SERVICE_REAR_KEEP_OUT_Y:.0}mm rear service zone"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_design_constraints() {
    assert_eq!(CASSETTE_POSITION_COUNT, 20);
    assert_eq!(edge_position_count(), EDGE_POSITION_COUNT);
    assert_eq!(center_position_count(), CENTER_POSITION_COUNT);
    assert_eq!(LOGGER_POCKET_COUNT, logger_pocket_centers().len());
    assert_eq!(
        MEDIA_RESERVOIR_DUMMY_COUNT,
        reservoir_dummy_centers().len(),
        "media reservoir dummy map changed unexpectedly"
    );
    assert_eq!(DISPOSITION_LANE_COUNT, DispositionLane::all().len());
    assert_eq!(
        BARCODE_LAND_COUNT,
        CASSETTE_POSITION_COUNT + LOGGER_POCKET_COUNT + 1
    );
    assert!(CASSETTE_X > ARRAY_X && CASSETTE_Y > ARRAY_Y);
    assert!(CASSETTE_SURROGATE_Z >= REVC_TOTAL_HEIGHT + 16.0);
    assert!(NEST_SOCKET_DEPTH < BASE_Z / 2.0);
    assert!(SHIELD_GAP_Z >= 10.0);
    assert!(DRIP_GUTTER_W >= DRAIN_PORT_D);
    assert!(drip_basin_front_y() > tray_front_y());
    assert!(drain_port_y() < drip_basin_front_y());
    assert!(ROBOT_PICK_CLEARANCE_Z > BASE_Z + CASSETTE_SURROGATE_Z + 45.0);
    assert!(SERVICE_LOGGER_PULL_X > LOGGER_POCKET_X + 40.0);
    assert!(total_disposition_capacity() == CASSETTE_POSITION_COUNT);
}

fn base_pan() -> Part {
    let deck = centered_cube(
        "cassette_evaporation_station_base_deck",
        TRAY_X,
        TRAY_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);

    deck - cassette_socket_recess()
        - logger_bank_socket()
        - media_bank_socket()
        - barcode_strip_socket()
        - lane_bank_socket()
        - drainage_channels()
        - tray_mount_holes()
        + perimeter_rims()
        + underside_scale_datum_rails()
}

fn cassette_socket_recess() -> Part {
    centered_cube(
        "cassette_evaporation_station_cassette_socket_recess",
        CASSETTE_X + 42.0,
        CASSETTE_Y + 42.0,
        NEST_SOCKET_DEPTH + 0.4,
    )
    .translate(
        NEST_CENTER_X,
        NEST_CENTER_Y,
        BASE_Z - NEST_SOCKET_DEPTH / 2.0 + 0.2,
    )
}

fn logger_bank_socket() -> Part {
    centered_cube(
        "cassette_evaporation_station_logger_bank_socket",
        LOGGER_BANK_X + 10.0,
        LOGGER_BANK_Y + 10.0,
        NEST_SOCKET_DEPTH + 0.4,
    )
    .translate(
        LOGGER_BANK_CENTER_X,
        LOGGER_BANK_CENTER_Y,
        BASE_Z - NEST_SOCKET_DEPTH / 2.0 + 0.2,
    )
}

fn media_bank_socket() -> Part {
    centered_cube(
        "cassette_evaporation_station_media_dummy_bank_socket",
        MEDIA_BANK_X + 10.0,
        MEDIA_BANK_Y + 10.0,
        NEST_SOCKET_DEPTH + 0.4,
    )
    .translate(
        MEDIA_BANK_CENTER_X,
        MEDIA_BANK_CENTER_Y,
        BASE_Z - NEST_SOCKET_DEPTH / 2.0 + 0.2,
    )
}

fn barcode_strip_socket() -> Part {
    centered_cube(
        "cassette_evaporation_station_mass_balance_barcode_strip_socket",
        BARCODE_STRIP_X + 12.0,
        BARCODE_STRIP_Y + 12.0,
        NEST_SOCKET_DEPTH + 0.4,
    )
    .translate(
        BARCODE_STRIP_CENTER_X,
        BARCODE_STRIP_CENTER_Y,
        BASE_Z - NEST_SOCKET_DEPTH / 2.0 + 0.2,
    )
}

fn lane_bank_socket() -> Part {
    centered_cube(
        "cassette_evaporation_station_disposition_lane_bank_socket",
        LANE_BANK_X + 12.0,
        LANE_BANK_Y + 12.0,
        NEST_SOCKET_DEPTH + 0.4,
    )
    .translate(
        LANE_CENTER_X,
        LANE_CENTER_Y,
        BASE_Z - NEST_SOCKET_DEPTH / 2.0 + 0.2,
    )
}

fn drainage_channels() -> Part {
    let front_channel = centered_cube(
        "cassette_evaporation_station_front_drip_runoff_channel",
        TRAY_X - 120.0,
        DRIP_GUTTER_W,
        DRIP_GUTTER_DEPTH,
    )
    .translate(0.0, tray_front_y() + 52.0, BASE_Z - DRIP_GUTTER_DEPTH / 2.0);
    let left_channel = centered_cube(
        "cassette_evaporation_station_left_drip_runoff_channel",
        DRIP_GUTTER_W,
        CASSETTE_Y + 118.0,
        DRIP_GUTTER_DEPTH,
    )
    .translate(
        NEST_CENTER_X - CASSETTE_X / 2.0 - 34.0,
        NEST_CENTER_Y - 10.0,
        BASE_Z - DRIP_GUTTER_DEPTH / 2.0,
    );
    let right_channel = centered_cube(
        "cassette_evaporation_station_right_drip_runoff_channel",
        DRIP_GUTTER_W,
        CASSETTE_Y + 118.0,
        DRIP_GUTTER_DEPTH,
    )
    .translate(
        NEST_CENTER_X + CASSETTE_X / 2.0 + 34.0,
        NEST_CENTER_Y - 10.0,
        BASE_Z - DRIP_GUTTER_DEPTH / 2.0,
    );

    front_channel + left_channel + right_channel
}

fn tray_mount_holes() -> Part {
    let mut holes = Part::empty("cassette_evaporation_station_tray_mount_holes");
    for (i, (x, y)) in [
        (-(TRAY_X / 2.0 - 42.0), -(TRAY_Y / 2.0 - 38.0)),
        (TRAY_X / 2.0 - 42.0, -(TRAY_Y / 2.0 - 38.0)),
        (-(TRAY_X / 2.0 - 42.0), TRAY_Y / 2.0 - 38.0),
        (TRAY_X / 2.0 - 42.0, TRAY_Y / 2.0 - 38.0),
        (0.0, -(TRAY_Y / 2.0 - 38.0)),
        (0.0, TRAY_Y / 2.0 - 38.0),
        (-(TRAY_X / 2.0 - 42.0), 0.0),
        (TRAY_X / 2.0 - 42.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("cassette_evaporation_station_m5_mount_clearance_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 4.0,
                24,
            )
            .translate(*x, *y, BASE_Z / 2.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    let rear = centered_cube(
        "cassette_evaporation_station_rear_service_rim",
        TRAY_X,
        PERIMETER_RIM_W,
        PERIMETER_RIM_Z,
    )
    .translate(
        0.0,
        tray_rear_y() - PERIMETER_RIM_W / 2.0,
        BASE_Z + PERIMETER_RIM_Z / 2.0,
    );
    let left = centered_cube(
        "cassette_evaporation_station_left_service_rim",
        PERIMETER_RIM_W,
        TRAY_Y,
        PERIMETER_RIM_Z,
    )
    .translate(
        -(TRAY_X / 2.0 - PERIMETER_RIM_W / 2.0),
        0.0,
        BASE_Z + PERIMETER_RIM_Z / 2.0,
    );
    let right = centered_cube(
        "cassette_evaporation_station_right_robot_rim",
        PERIMETER_RIM_W,
        TRAY_Y - 112.0,
        PERIMETER_RIM_Z,
    )
    .translate(
        TRAY_X / 2.0 - PERIMETER_RIM_W / 2.0,
        28.0,
        BASE_Z + PERIMETER_RIM_Z / 2.0,
    );
    let front_low_lip = centered_cube(
        "cassette_evaporation_station_front_low_drip_lip",
        TRAY_X - 140.0,
        12.0,
        20.0,
    )
    .translate(0.0, tray_front_y() + 20.0, BASE_Z + 10.0);

    rear + left + right + front_low_lip
}

fn underside_scale_datum_rails() -> Part {
    let left = centered_cube(
        "cassette_evaporation_station_underside_left_scale_datum_rail",
        18.0,
        TRAY_Y - 120.0,
        10.0,
    )
    .translate(-(TRAY_X / 2.0 - 70.0), 0.0, 5.0);
    let right = centered_cube(
        "cassette_evaporation_station_underside_right_scale_datum_rail",
        18.0,
        TRAY_Y - 120.0,
        10.0,
    )
    .translate(TRAY_X / 2.0 - 70.0, 0.0, 5.0);
    let rear_stop = centered_cube(
        "cassette_evaporation_station_underside_rear_scale_stop",
        TRAY_X - 180.0,
        18.0,
        10.0,
    )
    .translate(0.0, tray_rear_y() - 70.0, 5.0);

    left + right + rear_stop
}

fn weighed_surrogate_cassette_nests() -> Part {
    cassette_datum_rails() + cassette_position_grid() + weigh_pad_grid() + nest_datum_pin_bosses()
}

fn cassette_datum_rails() -> Part {
    let left_x_datum = centered_cube(
        "cassette_evaporation_left_x_datum_rail",
        NEST_RAIL_W,
        CASSETTE_Y + 28.0,
        NEST_RAIL_Z,
    )
    .translate(
        cassette_left_edge() - NEST_RAIL_W / 2.0 - 3.0,
        NEST_CENTER_Y,
        BASE_Z + NEST_RAIL_Z / 2.0,
    );
    let rear_y_datum = centered_cube(
        "cassette_evaporation_rear_y_datum_rail",
        CASSETTE_X + 36.0,
        NEST_RAIL_W,
        NEST_RAIL_Z,
    )
    .translate(
        NEST_CENTER_X,
        cassette_rear_edge() + NEST_RAIL_W / 2.0 + 3.0,
        BASE_Z + NEST_RAIL_Z / 2.0,
    );
    let right_soft_preload = centered_cube(
        "cassette_evaporation_right_soft_preload_rail",
        NEST_RAIL_W,
        CASSETTE_Y + 18.0,
        NEST_RAIL_Z * 0.64,
    )
    .translate(
        cassette_right_edge() + NEST_RAIL_W / 2.0 + 3.0,
        NEST_CENTER_Y,
        BASE_Z + NEST_RAIL_Z * 0.32,
    );
    let front_low_lip = centered_cube(
        "cassette_evaporation_front_low_loading_lip",
        CASSETTE_X + 36.0,
        10.0,
        16.0,
    )
    .translate(NEST_CENTER_X, cassette_front_edge() - 8.0, BASE_Z + 8.0);
    let left_ledge = centered_cube(
        "cassette_evaporation_left_scale_ledge",
        NEST_LEDGE_W,
        CASSETTE_Y - 36.0,
        NEST_LEDGE_Z,
    )
    .translate(
        cassette_left_edge() + 34.0,
        NEST_CENTER_Y,
        BASE_Z + NEST_LEDGE_Z / 2.0,
    );
    let right_ledge = centered_cube(
        "cassette_evaporation_right_scale_ledge",
        NEST_LEDGE_W,
        CASSETTE_Y - 36.0,
        NEST_LEDGE_Z,
    )
    .translate(
        cassette_right_edge() - 34.0,
        NEST_CENTER_Y,
        BASE_Z + NEST_LEDGE_Z / 2.0,
    );

    left_x_datum + rear_y_datum + right_soft_preload + front_low_lip + left_ledge + right_ledge
}

fn cassette_position_grid() -> Part {
    let mut grid = Part::empty("cassette_evaporation_position_separator_grid");
    for col in 1..CASSETTE_COLS {
        let x = cassette_left_edge()
            + CASSETTE_MARGIN_X
            + col as f64 * REVC_CHIP_LENGTH
            + (col as f64 - 0.5) * CHIP_GUTTER;
        grid = grid
            + centered_cube(
                format!("cassette_evaporation_column_separator_{col}"),
                4.0,
                ARRAY_Y + 16.0,
                9.0,
            )
            .translate(x, NEST_CENTER_Y, BASE_Z + 4.5);
    }
    for row in 1..CASSETTE_ROWS {
        let y = cassette_front_edge()
            + CASSETTE_MARGIN_Y
            + row as f64 * REVC_CHIP_WIDTH
            + (row as f64 - 0.5) * CHIP_GUTTER;
        grid = grid
            + centered_cube(
                format!("cassette_evaporation_row_separator_{row}"),
                ARRAY_X + 16.0,
                4.0,
                9.0,
            )
            .translate(NEST_CENTER_X, y, BASE_Z + 4.5);
    }
    grid
}

fn weigh_pad_grid() -> Part {
    let mut pads = Part::empty("cassette_evaporation_weighed_surrogate_cassette_nests");
    for row in 0..CASSETTE_ROWS {
        for col in 0..CASSETTE_COLS {
            let index = position_index(col, row);
            let (x, y) = cassette_position_center(col, row);
            pads = pads
                + single_weigh_pad(index, col, row).translate(x, y, BASE_Z + WEIGH_PAD_Z / 2.0);
        }
    }
    pads
}

fn single_weigh_pad(index: usize, col: usize, row: usize) -> Part {
    let zone = map_zone(col, row);
    let label = match zone {
        MapZone::Edge => "edge",
        MapZone::Center => "center",
    };
    let pad = centered_cube(
        format!("cassette_evaporation_{label}_weigh_pad_{index}"),
        WEIGH_PAD_X,
        WEIGH_PAD_Y,
        WEIGH_PAD_Z,
    );
    let cassette_recess = centered_cube(
        format!("cassette_evaporation_surrogate_cassette_recess_{index}"),
        WEIGH_PAD_X - 16.0,
        WEIGH_PAD_Y - 12.0,
        WEIGH_PAD_RELIEF_DEPTH + 0.2,
    )
    .translate(
        0.0,
        0.0,
        WEIGH_PAD_Z / 2.0 - WEIGH_PAD_RELIEF_DEPTH / 2.0 + 0.1,
    );
    let balance_target = centered_cylinder(
        format!("cassette_evaporation_microbalance_target_{index}"),
        MICROBALANCE_PAD_D / 2.0,
        1.8,
        28,
    )
    .translate(
        -WEIGH_PAD_X / 2.0 + 15.0,
        WEIGH_PAD_Y / 2.0 - 12.0,
        WEIGH_PAD_Z / 2.0 + 0.9,
    );
    let sealed_lid_boss = centered_cube(
        format!("cassette_evaporation_sealed_lid_reference_boss_{index}"),
        22.0,
        6.0,
        CASSETTE_SEAL_LID_Z,
    )
    .translate(
        WEIGH_PAD_X / 2.0 - 20.0,
        -WEIGH_PAD_Y / 2.0 + 8.0,
        WEIGH_PAD_Z / 2.0,
    );

    pad - cassette_recess + balance_target + sealed_lid_boss
}

fn nest_datum_pin_bosses() -> Part {
    let mut bosses = Part::empty("cassette_evaporation_nest_datum_pin_bosses");
    for (i, (x, y)) in nest_datum_pin_points().iter().enumerate() {
        let boss = centered_cylinder(format!("cassette_evaporation_datum_boss_{i}"), 8.0, 9.0, 32)
            .translate(*x, *y, BASE_Z + 4.5);
        let pilot = centered_cylinder(
            format!("cassette_evaporation_datum_pin_pilot_{i}"),
            2.0,
            11.0,
            20,
        )
        .translate(*x, *y, BASE_Z + 4.5);
        bosses = bosses + (boss - pilot);
    }
    bosses
}

fn humidity_temperature_logger_pockets() -> Part {
    let body = centered_cube(
        "cassette_evaporation_humidity_temperature_logger_bank",
        LOGGER_BANK_X,
        LOGGER_BANK_Y,
        LOGGER_BANK_Z,
    )
    .translate(
        LOGGER_BANK_CENTER_X,
        LOGGER_BANK_CENTER_Y,
        BASE_Z + LOGGER_BANK_Z / 2.0,
    );

    let mut cuts = Part::empty("cassette_evaporation_logger_pocket_cuts");
    let mut tabs = Part::empty("cassette_evaporation_logger_retainer_tabs");
    for (i, (x, y)) in logger_pocket_centers().iter().enumerate() {
        cuts =
            cuts + centered_cube(
                format!("cassette_evaporation_temp_rh_logger_pocket_{i}"),
                LOGGER_POCKET_X,
                LOGGER_POCKET_Y,
                LOGGER_POCKET_DEPTH,
            )
            .translate(
                *x,
                *y,
                BASE_Z + LOGGER_BANK_Z - LOGGER_POCKET_DEPTH / 2.0 + 0.2,
            ) + centered_cube(
                format!("cassette_evaporation_logger_cable_slot_{i}"),
                LOGGER_CABLE_SLOT_W,
                LOGGER_CABLE_SLOT_Y,
                LOGGER_BANK_Z + 2.0,
            )
            .translate(
                *x + LOGGER_POCKET_X / 2.0 - 10.0,
                *y - LOGGER_POCKET_Y / 2.0,
                BASE_Z + LOGGER_BANK_Z / 2.0,
            );

        tabs = tabs
            + centered_cube(
                format!("cassette_evaporation_logger_retainer_tab_{i}"),
                LOGGER_POCKET_X - 16.0,
                5.0,
                7.0,
            )
            .translate(
                *x,
                *y + LOGGER_POCKET_Y / 2.0 + 5.0,
                BASE_Z + LOGGER_BANK_Z + 3.5,
            );
    }

    body - cuts + tabs + logger_bank_gripper_ears()
}

fn logger_bank_gripper_ears() -> Part {
    let left = centered_cube(
        "cassette_evaporation_logger_bank_left_robot_ear",
        24.0,
        LOGGER_BANK_Y - 42.0,
        16.0,
    )
    .translate(
        LOGGER_BANK_CENTER_X - LOGGER_BANK_X / 2.0 - 12.0,
        LOGGER_BANK_CENTER_Y,
        BASE_Z + 8.0,
    );
    let right = centered_cube(
        "cassette_evaporation_logger_bank_right_robot_ear",
        24.0,
        LOGGER_BANK_Y - 42.0,
        16.0,
    )
    .translate(
        LOGGER_BANK_CENTER_X + LOGGER_BANK_X / 2.0 + 12.0,
        LOGGER_BANK_CENTER_Y,
        BASE_Z + 8.0,
    );
    left + right
}

fn condensate_shields() -> Part {
    let cassette_shield = centered_cube(
        "cassette_evaporation_clear_condensate_diverter_over_cassette_map",
        CASSETTE_X + SHIELD_OVERHANG_X,
        CASSETTE_Y + SHIELD_OVERHANG_Y,
        CONDENSATE_SHIELD_Z,
    )
    .translate(
        NEST_CENTER_X,
        NEST_CENTER_Y + 8.0,
        BASE_Z + SHIELD_STANDOFF_Z + CONDENSATE_SHIELD_Z / 2.0,
    );
    let logger_shield = centered_cube(
        "cassette_evaporation_logger_bank_condensate_lip",
        LOGGER_BANK_X + 28.0,
        18.0,
        CONDENSATE_SHIELD_Z,
    )
    .translate(
        LOGGER_BANK_CENTER_X,
        LOGGER_BANK_CENTER_Y + LOGGER_BANK_Y / 2.0 + 18.0,
        BASE_Z + LOGGER_BANK_Z + 18.0,
    );
    let media_shield = centered_cube(
        "cassette_evaporation_media_dummy_condensate_lip",
        MEDIA_BANK_X + 28.0,
        18.0,
        CONDENSATE_SHIELD_Z,
    )
    .translate(
        MEDIA_BANK_CENTER_X,
        MEDIA_BANK_CENTER_Y + MEDIA_BANK_Y / 2.0 + 18.0,
        BASE_Z + MEDIA_BANK_Z + 18.0,
    );

    cassette_shield + shield_standoffs() + logger_shield + media_shield + condensate_gutters()
}

fn shield_standoffs() -> Part {
    let mut posts = Part::empty("cassette_evaporation_condensate_shield_standoffs");
    for (i, (x, y)) in [
        (
            NEST_CENTER_X - CASSETTE_X / 2.0 - 18.0,
            NEST_CENTER_Y - CASSETTE_Y / 2.0 - 10.0,
        ),
        (
            NEST_CENTER_X + CASSETTE_X / 2.0 + 18.0,
            NEST_CENTER_Y - CASSETTE_Y / 2.0 - 10.0,
        ),
        (
            NEST_CENTER_X - CASSETTE_X / 2.0 - 18.0,
            NEST_CENTER_Y + CASSETTE_Y / 2.0 + 16.0,
        ),
        (
            NEST_CENTER_X + CASSETTE_X / 2.0 + 18.0,
            NEST_CENTER_Y + CASSETTE_Y / 2.0 + 16.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cylinder(
                format!("cassette_evaporation_condensate_shield_standoff_{i}"),
                5.0,
                SHIELD_STANDOFF_Z,
                24,
            )
            .translate(*x, *y, BASE_Z + SHIELD_STANDOFF_Z / 2.0);
    }
    posts
}

fn condensate_gutters() -> Part {
    let front = centered_cube(
        "cassette_evaporation_front_condensate_shield_gutter",
        CASSETTE_X + 84.0,
        DRIP_GUTTER_W,
        8.0,
    )
    .translate(
        NEST_CENTER_X,
        cassette_front_edge() - 36.0,
        BASE_Z + SHIELD_STANDOFF_Z - 4.0,
    );
    let rear = centered_cube(
        "cassette_evaporation_rear_condensate_shield_gutter",
        CASSETTE_X + 84.0,
        DRIP_GUTTER_W,
        8.0,
    )
    .translate(
        NEST_CENTER_X,
        cassette_rear_edge() + 38.0,
        BASE_Z + SHIELD_STANDOFF_Z - 4.0,
    );
    let left_downspout = centered_cube(
        "cassette_evaporation_left_condensate_downspout",
        DRIP_GUTTER_W,
        18.0,
        SHIELD_STANDOFF_Z,
    )
    .translate(
        NEST_CENTER_X - CASSETTE_X / 2.0 - 44.0,
        drip_basin_rear_y() + 18.0,
        BASE_Z + SHIELD_STANDOFF_Z / 2.0,
    );
    let right_downspout = centered_cube(
        "cassette_evaporation_right_condensate_downspout",
        DRIP_GUTTER_W,
        18.0,
        SHIELD_STANDOFF_Z,
    )
    .translate(
        NEST_CENTER_X + CASSETTE_X / 2.0 + 44.0,
        drip_basin_rear_y() + 18.0,
        BASE_Z + SHIELD_STANDOFF_Z / 2.0,
    );

    front + rear + left_downspout + right_downspout
}

fn media_reservoir_dummy_pockets() -> Part {
    let body = centered_cube(
        "cassette_evaporation_media_reservoir_dummy_bank",
        MEDIA_BANK_X,
        MEDIA_BANK_Y,
        MEDIA_BANK_Z,
    )
    .translate(
        MEDIA_BANK_CENTER_X,
        MEDIA_BANK_CENTER_Y,
        BASE_Z + MEDIA_BANK_Z / 2.0,
    );

    let mut cuts = Part::empty("cassette_evaporation_media_reservoir_dummy_pocket_cuts");
    let mut fill_levels = Part::empty("cassette_evaporation_media_reservoir_fill_level_ribs");
    for (i, (x, y)) in reservoir_dummy_centers().iter().enumerate() {
        cuts =
            cuts + centered_cube(
                format!("cassette_evaporation_media_reservoir_dummy_pocket_{i}"),
                RESERVOIR_POCKET_X,
                RESERVOIR_POCKET_Y,
                RESERVOIR_POCKET_DEPTH,
            )
            .translate(
                *x,
                *y,
                BASE_Z + MEDIA_BANK_Z - RESERVOIR_POCKET_DEPTH / 2.0 + 0.2,
            ) + centered_cylinder(
                format!("cassette_evaporation_reservoir_evap_reference_well_{i}"),
                9.0,
                RESERVOIR_POCKET_DEPTH + 1.0,
                28,
            )
            .translate(
                *x - RESERVOIR_POCKET_X / 2.0 + 18.0,
                *y + RESERVOIR_POCKET_Y / 2.0 - 16.0,
                BASE_Z + MEDIA_BANK_Z - RESERVOIR_POCKET_DEPTH / 2.0 + 0.2,
            );

        fill_levels = fill_levels
            + centered_cube(
                format!("cassette_evaporation_media_dummy_fill_level_tick_{i}"),
                RESERVOIR_POCKET_X - 18.0,
                4.0,
                DUMMY_FILL_LEVEL_Z,
            )
            .translate(
                *x,
                *y - RESERVOIR_POCKET_Y / 2.0 + 10.0,
                BASE_Z + MEDIA_BANK_Z + DUMMY_FILL_LEVEL_Z / 2.0,
            );
    }

    body - cuts + fill_levels + media_bank_gripper_ears()
}

fn media_bank_gripper_ears() -> Part {
    let left = centered_cube(
        "cassette_evaporation_media_bank_left_robot_ear",
        24.0,
        MEDIA_BANK_Y - 48.0,
        16.0,
    )
    .translate(
        MEDIA_BANK_CENTER_X - MEDIA_BANK_X / 2.0 - 12.0,
        MEDIA_BANK_CENTER_Y,
        BASE_Z + 8.0,
    );
    let right = centered_cube(
        "cassette_evaporation_media_bank_right_robot_ear",
        24.0,
        MEDIA_BANK_Y - 48.0,
        16.0,
    )
    .translate(
        MEDIA_BANK_CENTER_X + MEDIA_BANK_X / 2.0 + 12.0,
        MEDIA_BANK_CENTER_Y,
        BASE_Z + 8.0,
    );
    left + right
}

fn mass_balance_barcode_lands() -> Part {
    let strip = centered_cube(
        "cassette_evaporation_mass_balance_barcode_strip",
        BARCODE_STRIP_X,
        BARCODE_STRIP_Y,
        BARCODE_STRIP_Z,
    )
    .translate(
        BARCODE_STRIP_CENTER_X,
        BARCODE_STRIP_CENTER_Y,
        BASE_Z + BARCODE_STRIP_Z / 2.0,
    );

    strip + cassette_barcode_lands() + logger_barcode_lands() + study_master_barcode_land()
}

fn cassette_barcode_lands() -> Part {
    let mut lands = Part::empty("cassette_evaporation_cassette_mass_balance_barcode_lands");
    for index in 0..CASSETTE_POSITION_COUNT {
        let col = index % 5;
        let row = index / 5;
        let x = BARCODE_STRIP_CENTER_X - 144.0 + col as f64 * 72.0;
        let y = BARCODE_STRIP_CENTER_Y + 55.0 - row as f64 * 32.0;
        lands = lands
            + barcode_land(
                format!("cassette_evaporation_position_{index}_barcode_land"),
                x,
                y,
            );
    }
    lands
}

fn logger_barcode_lands() -> Part {
    let mut lands = Part::empty("cassette_evaporation_logger_barcode_lands");
    for i in 0..LOGGER_POCKET_COUNT {
        let x = BARCODE_STRIP_CENTER_X - 110.0 + i as f64 * 44.0;
        let y = BARCODE_STRIP_CENTER_Y - 78.0;
        lands = lands
            + barcode_land(
                format!("cassette_evaporation_logger_{i}_barcode_land"),
                x,
                y,
            );
    }
    lands
}

fn study_master_barcode_land() -> Part {
    barcode_land(
        "cassette_evaporation_study_master_mass_balance_barcode_land",
        BARCODE_STRIP_CENTER_X + 144.0,
        BARCODE_STRIP_CENTER_Y - 78.0,
    )
}

fn barcode_land(name: impl Into<String>, x: f64, y: f64) -> Part {
    let land = centered_cube(name.into(), BARCODE_LAND_X, BARCODE_LAND_Y, BARCODE_LAND_Z)
        .translate(x, y, BASE_Z + BARCODE_STRIP_Z + BARCODE_LAND_Z / 2.0);
    let scan_notch = centered_cube(
        "cassette_evaporation_barcode_scan_notch",
        BARCODE_LAND_X - 12.0,
        2.0,
        BARCODE_LAND_Z + 0.4,
    )
    .translate(x, y, BASE_Z + BARCODE_STRIP_Z + BARCODE_LAND_Z / 2.0);
    land - scan_notch
}

fn edge_center_position_tokens() -> Part {
    let mut tokens = Part::empty("cassette_evaporation_edge_center_position_tokens");
    for row in 0..CASSETTE_ROWS {
        for col in 0..CASSETTE_COLS {
            let index = position_index(col, row);
            let (x, y) = cassette_position_center(col, row);
            tokens = tokens
                + position_token(index, col, row).translate(x, y, BASE_Z + WEIGH_PAD_Z + 3.0);
        }
    }
    tokens
}

fn position_token(index: usize, col: usize, row: usize) -> Part {
    match map_zone(col, row) {
        MapZone::Edge => {
            let token = centered_cube(
                format!("cassette_evaporation_edge_position_square_token_{index}"),
                TOKEN_D,
                TOKEN_D,
                EDGE_TOKEN_Z,
            );
            let center_cut = centered_cylinder(
                format!("cassette_evaporation_edge_token_center_dot_{index}"),
                2.0,
                EDGE_TOKEN_Z + 0.4,
                20,
            );
            token - center_cut
        }
        MapZone::Center => {
            let token = centered_cylinder(
                format!("cassette_evaporation_center_position_round_token_{index}"),
                TOKEN_D / 2.0,
                CENTER_TOKEN_Z,
                32,
            );
            let bar = centered_cube(
                format!("cassette_evaporation_center_token_crossbar_{index}"),
                TOKEN_D + 3.0,
                2.4,
                CENTER_TOKEN_Z + 0.6,
            );
            token + bar
        }
    }
}

fn drip_capture() -> Part {
    let basin = centered_cube(
        "cassette_evaporation_front_drip_capture_basin",
        DRIP_CAPTURE_BASIN_X,
        DRIP_CAPTURE_BASIN_Y,
        DRIP_CAPTURE_BASIN_DEPTH,
    )
    .translate(
        NEST_CENTER_X,
        drip_basin_center_y(),
        BASE_Z + DRIP_CAPTURE_BASIN_DEPTH / 2.0,
    );
    let recess = centered_cube(
        "cassette_evaporation_front_drip_capture_basin_recess",
        DRIP_CAPTURE_BASIN_X - 44.0,
        DRIP_CAPTURE_BASIN_Y - 22.0,
        DRIP_CAPTURE_BASIN_DEPTH + 0.4,
    )
    .translate(
        NEST_CENTER_X,
        drip_basin_center_y(),
        BASE_Z + DRIP_CAPTURE_BASIN_DEPTH / 2.0 + 1.8,
    );
    let sump = centered_cube("cassette_evaporation_drip_capture_sump", 82.0, 38.0, 10.0).translate(
        NEST_CENTER_X + DRIP_CAPTURE_BASIN_X / 2.0 - 68.0,
        drip_basin_center_y() - DRIP_CAPTURE_BASIN_Y / 2.0 + 24.0,
        BASE_Z + 5.0,
    );
    let drain = centered_cylinder(
        "cassette_evaporation_drip_capture_forward_drain_port",
        DRAIN_PORT_D / 2.0,
        38.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        NEST_CENTER_X + DRIP_CAPTURE_BASIN_X / 2.0 - 68.0,
        drain_port_y(),
        BASE_Z + 7.0,
    );
    let absorbent_retainers = absorbent_pad_retainers();
    let leak_sensor_land = centered_cube(
        "cassette_evaporation_drip_capture_leak_sensor_land",
        68.0,
        24.0,
        4.0,
    )
    .translate(
        NEST_CENTER_X - DRIP_CAPTURE_BASIN_X / 2.0 + 72.0,
        drip_basin_center_y() - 8.0,
        BASE_Z + DRIP_CAPTURE_BASIN_DEPTH + 2.0,
    );

    basin - recess - sump - drain + absorbent_retainers + leak_sensor_land
}

fn absorbent_pad_retainers() -> Part {
    let front = centered_cube(
        "cassette_evaporation_absorbent_pad_front_retainer",
        DRIP_CAPTURE_BASIN_X - 84.0,
        7.0,
        10.0,
    )
    .translate(
        NEST_CENTER_X,
        drip_basin_front_y() + 18.0,
        BASE_Z + DRIP_CAPTURE_BASIN_DEPTH + 5.0,
    );
    let rear = centered_cube(
        "cassette_evaporation_absorbent_pad_rear_retainer",
        DRIP_CAPTURE_BASIN_X - 84.0,
        7.0,
        10.0,
    )
    .translate(
        NEST_CENTER_X,
        drip_basin_rear_y() - 18.0,
        BASE_Z + DRIP_CAPTURE_BASIN_DEPTH + 5.0,
    );
    front + rear
}

fn release_hold_reject_lanes() -> Part {
    let body = centered_cube(
        "cassette_evaporation_release_hold_reject_lane_bank",
        LANE_BANK_X,
        LANE_BANK_Y,
        LANE_BANK_Z,
    )
    .translate(LANE_CENTER_X, LANE_CENTER_Y, BASE_Z + LANE_BANK_Z / 2.0);

    let mut recesses = Part::empty("cassette_evaporation_disposition_lane_recesses");
    let mut ribs = Part::empty("cassette_evaporation_disposition_lane_separators");
    let mut tokens = Part::empty("cassette_evaporation_disposition_capacity_tokens");
    for lane in DispositionLane::all() {
        let x = lane_center_x(lane);
        let label = lane.label();
        recesses = recesses
            + centered_cube(
                format!("cassette_evaporation_{label}_lane_recess"),
                LANE_W,
                LANE_Y,
                LANE_RECESS_DEPTH,
            )
            .translate(
                x,
                LANE_CENTER_Y,
                BASE_Z + LANE_BANK_Z - LANE_RECESS_DEPTH / 2.0 + 0.2,
            );

        for slot in 0..lane.nominal_capacity() {
            tokens = tokens
                + centered_cube(
                    format!("cassette_evaporation_{label}_lane_token_slot_{slot}"),
                    18.0,
                    12.0,
                    5.0,
                )
                .translate(
                    x,
                    lane_token_y(slot, lane.nominal_capacity()),
                    BASE_Z + LANE_BANK_Z + 2.5,
                );
        }
    }

    for i in 0..2 {
        let x = LANE_CENTER_X - LANE_BANK_X / 6.0 + i as f64 * LANE_BANK_X / 3.0;
        ribs = ribs
            + centered_cube(
                format!("cassette_evaporation_disposition_lane_separator_{i}"),
                LANE_RIB_W,
                LANE_BANK_Y - 24.0,
                16.0,
            )
            .translate(x, LANE_CENTER_Y, BASE_Z + LANE_BANK_Z + 8.0);
    }

    body - recesses + ribs + tokens + lane_bank_gripper_ears()
}

fn lane_bank_gripper_ears() -> Part {
    let left = centered_cube(
        "cassette_evaporation_disposition_lane_left_robot_ear",
        24.0,
        LANE_BANK_Y - 48.0,
        16.0,
    )
    .translate(
        LANE_CENTER_X - LANE_BANK_X / 2.0 - 12.0,
        LANE_CENTER_Y,
        BASE_Z + 8.0,
    );
    let right = centered_cube(
        "cassette_evaporation_disposition_lane_right_robot_ear",
        24.0,
        LANE_BANK_Y - 48.0,
        16.0,
    )
    .translate(
        LANE_CENTER_X + LANE_BANK_X / 2.0 + 12.0,
        LANE_CENTER_Y,
        BASE_Z + 8.0,
    );
    left + right
}

fn robot_service_keepouts() -> Part {
    let front_robot = centered_cube(
        "cassette_evaporation_front_robot_pick_keepout_volume",
        CASSETTE_X + ROBOT_SIDE_KEEP_OUT_X * 2.0,
        ROBOT_FRONT_KEEP_OUT_Y,
        ROBOT_PICK_CLEARANCE_Z,
    )
    .translate(
        NEST_CENTER_X,
        cassette_front_edge() - ROBOT_FRONT_KEEP_OUT_Y / 2.0 - 28.0,
        BASE_Z + ROBOT_PICK_CLEARANCE_Z / 2.0,
    );
    let left_service = centered_cube(
        "cassette_evaporation_left_logger_service_keepout_volume",
        SERVICE_LOGGER_PULL_X,
        LOGGER_BANK_Y + MEDIA_BANK_Y + 96.0,
        74.0,
    )
    .translate(
        LOGGER_BANK_CENTER_X - LOGGER_BANK_X / 2.0 - SERVICE_LOGGER_PULL_X / 2.0 - 30.0,
        (LOGGER_BANK_CENTER_Y + MEDIA_BANK_CENTER_Y) / 2.0,
        BASE_Z + 37.0,
    );
    let rear_service = centered_cube(
        "cassette_evaporation_rear_incubator_service_keepout_volume",
        TRAY_X - 180.0,
        SERVICE_REAR_KEEP_OUT_Y,
        92.0,
    )
    .translate(
        0.0,
        tray_rear_y() + SERVICE_REAR_KEEP_OUT_Y / 2.0 + 24.0,
        BASE_Z + 46.0,
    );
    let lane_robot = centered_cube(
        "cassette_evaporation_disposition_lane_robot_keepout_volume",
        LANE_BANK_X + 78.0,
        LANE_BANK_Y + 72.0,
        82.0,
    )
    .translate(LANE_CENTER_X, LANE_CENTER_Y, BASE_Z + 41.0);

    front_robot + left_service + rear_service + lane_robot
}

fn cassette_position_center(col: usize, row: usize) -> (f64, f64) {
    let x =
        cassette_left_edge() + CASSETTE_MARGIN_X + REVC_CHIP_LENGTH / 2.0 + col as f64 * pitch_x();
    let y =
        cassette_front_edge() + CASSETTE_MARGIN_Y + REVC_CHIP_WIDTH / 2.0 + row as f64 * pitch_y();
    (x, y)
}

fn map_zone(col: usize, row: usize) -> MapZone {
    if col == 0 || col == CASSETTE_COLS - 1 || row == 0 || row == CASSETTE_ROWS - 1 {
        MapZone::Edge
    } else {
        MapZone::Center
    }
}

fn edge_position_count() -> usize {
    let mut count = 0;
    for row in 0..CASSETTE_ROWS {
        for col in 0..CASSETTE_COLS {
            if map_zone(col, row) == MapZone::Edge {
                count += 1;
            }
        }
    }
    count
}

fn center_position_count() -> usize {
    CASSETTE_POSITION_COUNT - edge_position_count()
}

fn position_index(col: usize, row: usize) -> usize {
    row * CASSETTE_COLS + col
}

fn pitch_x() -> f64 {
    REVC_CHIP_LENGTH + CHIP_GUTTER
}

fn pitch_y() -> f64 {
    REVC_CHIP_WIDTH + CHIP_GUTTER
}

fn cassette_left_edge() -> f64 {
    NEST_CENTER_X - CASSETTE_X / 2.0
}

fn cassette_right_edge() -> f64 {
    NEST_CENTER_X + CASSETTE_X / 2.0
}

fn cassette_front_edge() -> f64 {
    NEST_CENTER_Y - CASSETTE_Y / 2.0
}

fn cassette_rear_edge() -> f64 {
    NEST_CENTER_Y + CASSETTE_Y / 2.0
}

fn tray_front_y() -> f64 {
    -TRAY_Y / 2.0
}

fn tray_rear_y() -> f64 {
    TRAY_Y / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn logger_pocket_centers() -> [(f64, f64); LOGGER_POCKET_COUNT] {
    [
        (
            LOGGER_BANK_CENTER_X + centered_index(0, 3, LOGGER_PITCH_X),
            LOGGER_BANK_CENTER_Y + centered_index(0, 2, LOGGER_PITCH_Y),
        ),
        (
            LOGGER_BANK_CENTER_X + centered_index(1, 3, LOGGER_PITCH_X),
            LOGGER_BANK_CENTER_Y + centered_index(0, 2, LOGGER_PITCH_Y),
        ),
        (
            LOGGER_BANK_CENTER_X + centered_index(2, 3, LOGGER_PITCH_X),
            LOGGER_BANK_CENTER_Y + centered_index(0, 2, LOGGER_PITCH_Y),
        ),
        (
            LOGGER_BANK_CENTER_X + centered_index(0, 3, LOGGER_PITCH_X),
            LOGGER_BANK_CENTER_Y + centered_index(1, 2, LOGGER_PITCH_Y),
        ),
        (
            LOGGER_BANK_CENTER_X + centered_index(1, 3, LOGGER_PITCH_X),
            LOGGER_BANK_CENTER_Y + centered_index(1, 2, LOGGER_PITCH_Y),
        ),
        (
            LOGGER_BANK_CENTER_X + centered_index(2, 3, LOGGER_PITCH_X),
            LOGGER_BANK_CENTER_Y + centered_index(1, 2, LOGGER_PITCH_Y),
        ),
    ]
}

fn reservoir_dummy_centers() -> [(f64, f64); MEDIA_RESERVOIR_DUMMY_COUNT] {
    [
        (
            MEDIA_BANK_CENTER_X + centered_index(0, 2, RESERVOIR_PITCH_X),
            MEDIA_BANK_CENTER_Y + centered_index(0, 4, RESERVOIR_PITCH_Y),
        ),
        (
            MEDIA_BANK_CENTER_X + centered_index(1, 2, RESERVOIR_PITCH_X),
            MEDIA_BANK_CENTER_Y + centered_index(0, 4, RESERVOIR_PITCH_Y),
        ),
        (
            MEDIA_BANK_CENTER_X + centered_index(0, 2, RESERVOIR_PITCH_X),
            MEDIA_BANK_CENTER_Y + centered_index(1, 4, RESERVOIR_PITCH_Y),
        ),
        (
            MEDIA_BANK_CENTER_X + centered_index(1, 2, RESERVOIR_PITCH_X),
            MEDIA_BANK_CENTER_Y + centered_index(1, 4, RESERVOIR_PITCH_Y),
        ),
        (
            MEDIA_BANK_CENTER_X + centered_index(0, 2, RESERVOIR_PITCH_X),
            MEDIA_BANK_CENTER_Y + centered_index(2, 4, RESERVOIR_PITCH_Y),
        ),
        (
            MEDIA_BANK_CENTER_X + centered_index(1, 2, RESERVOIR_PITCH_X),
            MEDIA_BANK_CENTER_Y + centered_index(2, 4, RESERVOIR_PITCH_Y),
        ),
        (
            MEDIA_BANK_CENTER_X + centered_index(0, 2, RESERVOIR_PITCH_X),
            MEDIA_BANK_CENTER_Y + centered_index(3, 4, RESERVOIR_PITCH_Y),
        ),
        (
            MEDIA_BANK_CENTER_X + centered_index(1, 2, RESERVOIR_PITCH_X),
            MEDIA_BANK_CENTER_Y + centered_index(3, 4, RESERVOIR_PITCH_Y),
        ),
    ]
}

fn nest_datum_pin_points() -> [(f64, f64); 4] {
    [
        (cassette_left_edge() + 24.0, cassette_front_edge() + 24.0),
        (cassette_right_edge() - 24.0, cassette_front_edge() + 24.0),
        (cassette_left_edge() + 24.0, cassette_rear_edge() - 24.0),
        (cassette_right_edge() - 24.0, cassette_rear_edge() - 24.0),
    ]
}

fn drip_basin_center_y() -> f64 {
    cassette_front_edge() - 72.0
}

fn drip_basin_front_y() -> f64 {
    drip_basin_center_y() - DRIP_CAPTURE_BASIN_Y / 2.0
}

fn drip_basin_rear_y() -> f64 {
    drip_basin_center_y() + DRIP_CAPTURE_BASIN_Y / 2.0
}

fn drain_port_y() -> f64 {
    drip_basin_front_y() - 9.0
}

fn lane_center_x(lane: DispositionLane) -> f64 {
    LANE_CENTER_X + centered_index(lane.index(), DISPOSITION_LANE_COUNT, LANE_W + 20.0)
}

fn lane_token_y(slot: usize, capacity: usize) -> f64 {
    LANE_CENTER_Y + centered_index(slot, capacity, 18.0)
}

fn total_disposition_capacity() -> usize {
    DispositionLane::all()
        .iter()
        .map(|lane| lane.nominal_capacity())
        .sum()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn cassette_map_has_twenty_weighed_positions() {
        let mut positions = BTreeSet::new();
        for row in 0..CASSETTE_ROWS {
            for col in 0..CASSETTE_COLS {
                positions.insert((col, row));
                let (x, y) = cassette_position_center(col, row);
                assert!(x > cassette_left_edge());
                assert!(x < cassette_right_edge());
                assert!(y > cassette_front_edge());
                assert!(y < cassette_rear_edge());
            }
        }

        assert_eq!(CASSETTE_COLS, 4);
        assert_eq!(CASSETTE_ROWS, 5);
        assert_eq!(positions.len(), CASSETTE_POSITION_COUNT);
        assert_eq!(CASSETTE_POSITION_COUNT, 20);
        assert!(CASSETTE_SURROGATE_Z >= REVC_TOTAL_HEIGHT + 16.0);
    }

    #[test]
    fn edge_center_tokens_match_incubator_position_study_zones() {
        let mut edge = 0;
        let mut center = 0;
        for row in 0..CASSETTE_ROWS {
            for col in 0..CASSETTE_COLS {
                match map_zone(col, row) {
                    MapZone::Edge => edge += 1,
                    MapZone::Center => center += 1,
                }
            }
        }

        assert_eq!(edge, EDGE_POSITION_COUNT);
        assert_eq!(center, CENTER_POSITION_COUNT);
        assert_eq!(edge + center, CASSETTE_POSITION_COUNT);
        assert!(CENTER_TOKEN_Z > EDGE_TOKEN_Z);
    }

    #[test]
    fn logger_and_media_dummy_counts_cover_mapping_controls() {
        assert_eq!(logger_pocket_centers().len(), LOGGER_POCKET_COUNT);
        assert_eq!(reservoir_dummy_centers().len(), MEDIA_RESERVOIR_DUMMY_COUNT);
        assert!(LOGGER_POCKET_DEPTH < LOGGER_BANK_Z);
        assert!(RESERVOIR_POCKET_DEPTH < MEDIA_BANK_Z);
        assert!(LOGGER_CABLE_SLOT_W >= 8.0);
        assert!(DUMMY_FILL_LEVEL_Z >= 4.0);
    }

    #[test]
    fn mass_balance_labels_and_disposition_lanes_are_sized_for_all_positions() {
        assert_eq!(
            BARCODE_LAND_COUNT,
            CASSETTE_POSITION_COUNT + LOGGER_POCKET_COUNT + 1
        );
        assert_eq!(DispositionLane::all().len(), DISPOSITION_LANE_COUNT);
        assert_eq!(total_disposition_capacity(), CASSETTE_POSITION_COUNT);
        assert_eq!(DispositionLane::Release.nominal_capacity(), 8);
        assert_eq!(DispositionLane::Hold.nominal_capacity(), 8);
        assert_eq!(DispositionLane::Reject.nominal_capacity(), 4);
    }

    #[test]
    fn condensate_and_drip_capture_route_forward() {
        assert!(SHIELD_GAP_Z >= 10.0);
        assert!(DRIP_GUTTER_W >= DRAIN_PORT_D);
        assert!(drip_basin_rear_y() < cassette_front_edge());
        assert!(drip_basin_front_y() > tray_front_y());
        assert!(drain_port_y() < drip_basin_front_y());
        assert!(DRIP_CAPTURE_BASIN_DEPTH >= 12.0);
    }

    #[test]
    fn robot_and_service_keepouts_clear_hardware() {
        assert!(ROBOT_PICK_CLEARANCE_Z > BASE_Z + CASSETTE_SURROGATE_Z + 45.0);
        assert!(ROBOT_SIDE_KEEP_OUT_X >= 90.0);
        assert!(ROBOT_FRONT_KEEP_OUT_Y >= 240.0);
        assert!(SERVICE_REAR_KEEP_OUT_Y >= 160.0);
        assert!(SERVICE_LOGGER_PULL_X > LOGGER_POCKET_X + 40.0);

        for (x, y) in nest_datum_pin_points() {
            assert!(x > cassette_left_edge());
            assert!(x < cassette_right_edge());
            assert!(y > cassette_front_edge());
            assert!(y < cassette_rear_edge());
        }
    }
}
