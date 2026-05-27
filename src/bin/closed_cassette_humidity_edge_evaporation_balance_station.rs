use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed-cassette humidity edge evaporation balance station.
//
// This model is a closed-system incubator validation fixture. It maps whether
// cassette edge lanes evaporate or drift differently than center lanes during
// humid operation by co-locating a 4x5 cassette surrogate grid, humidity logger
// pockets, weighed micro-reservoir pads, condensate controls, cap-seal witness
// pockets, dye/osmolality sampling wells, custody surfaces, disposition lanes,
// an evidence bridge, and robot/service keepout gauges on one manufacturable
// station.

const OUTPUTS: [&str; 12] = [
    "output/closed_cassette_humidity_edge_evaporation_balance_station_base_tray.stl",
    "output/closed_cassette_humidity_edge_evaporation_balance_station_edge_center_surrogate_grid.stl",
    "output/closed_cassette_humidity_edge_evaporation_balance_station_humidity_logger_pockets.stl",
    "output/closed_cassette_humidity_edge_evaporation_balance_station_micro_reservoir_mass_pads.stl",
    "output/closed_cassette_humidity_edge_evaporation_balance_station_condensate_drip_shields.stl",
    "output/closed_cassette_humidity_edge_evaporation_balance_station_cap_seal_witness_pockets.stl",
    "output/closed_cassette_humidity_edge_evaporation_balance_station_dye_osmolality_sample_wells.stl",
    "output/closed_cassette_humidity_edge_evaporation_balance_station_barcode_custody_lands.stl",
    "output/closed_cassette_humidity_edge_evaporation_balance_station_release_hold_reject_lanes.stl",
    "output/closed_cassette_humidity_edge_evaporation_balance_station_evidence_bridge.stl",
    "output/closed_cassette_humidity_edge_evaporation_balance_station_robot_service_keepout_gauges.stl",
    "output/closed_cassette_humidity_edge_evaporation_balance_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 15] = [
    "closed_cassette_surrogate_grid",
    "edge_lane_map",
    "center_lane_map",
    "humidity_logger_pockets",
    "micro_reservoir_mass_pads",
    "condensate_drip_shields",
    "cap_seal_witness_pockets",
    "dye_sample_wells",
    "osmolality_sample_wells",
    "barcode_lands",
    "custody_lands",
    "release_lane",
    "hold_lane",
    "reject_lane",
    "evidence_bridge",
];

const COLS: usize = 4;
const ROWS: usize = 5;
const POSITION_COUNT: usize = COLS * ROWS;
const EDGE_POSITION_COUNT: usize = 14;
const CENTER_POSITION_COUNT: usize = POSITION_COUNT - EDGE_POSITION_COUNT;

const TRAY_X: f64 = 1280.0;
const TRAY_Y: f64 = 860.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 38.0;
const SOCKET_DEPTH: f64 = 5.2;
const MOUNT_HOLE_D: f64 = 5.4;

const CHIP_GUTTER: f64 = 7.0;
const GRID_CENTER_X: f64 = -128.0;
const GRID_CENTER_Y: f64 = 92.0;
const ARRAY_X: f64 = COLS as f64 * REVC_CHIP_LENGTH + (COLS as f64 - 1.0) * CHIP_GUTTER;
const ARRAY_Y: f64 = ROWS as f64 * REVC_CHIP_WIDTH + (ROWS as f64 - 1.0) * CHIP_GUTTER;
const GRID_FRAME_MARGIN_X: f64 = 34.0;
const GRID_FRAME_MARGIN_Y: f64 = 32.0;
const GRID_FRAME_X: f64 = ARRAY_X + 2.0 * GRID_FRAME_MARGIN_X;
const GRID_FRAME_Y: f64 = ARRAY_Y + 2.0 * GRID_FRAME_MARGIN_Y;
const GRID_FRAME_Z: f64 = 12.0;
const GRID_RIB_W: f64 = 8.0;
const GRID_RIB_Z: f64 = 8.0;
const POSITION_PAD_X: f64 = REVC_CHIP_LENGTH - 14.0;
const POSITION_PAD_Y: f64 = REVC_CHIP_WIDTH - 13.0;
const POSITION_PAD_Z: f64 = 9.0;
const POSITION_RECESS_DEPTH: f64 = 3.0;
const EDGE_LANE_RAIL_Z: f64 = 15.0;
const CENTER_LANE_RAIL_Z: f64 = 10.0;
const EDGE_TOKEN_D: f64 = 13.0;
const CENTER_TOKEN_D: f64 = 10.0;

const LOGGER_BANK_CENTER_X: f64 = -532.0;
const LOGGER_BANK_CENTER_Y: f64 = 72.0;
const LOGGER_BANK_X: f64 = 168.0;
const LOGGER_BANK_Y: f64 = 500.0;
const LOGGER_BANK_Z: f64 = 34.0;
const LOGGER_COUNT: usize = 8;
const LOGGER_POCKET_X: f64 = 64.0;
const LOGGER_POCKET_Y: f64 = 44.0;
const LOGGER_POCKET_DEPTH: f64 = 16.0;
const LOGGER_PITCH_X: f64 = 72.0;
const LOGGER_PITCH_Y: f64 = 108.0;
const LOGGER_LANYARD_SLOT_W: f64 = 8.0;
const LOGGER_DIFFUSION_SLOT_COUNT: usize = 5;

const MASS_PAD_X: f64 = 84.0;
const MASS_PAD_Y: f64 = 36.0;
const MASS_PAD_Z: f64 = 5.0;
const MASS_PAD_RELIEF_DEPTH: f64 = 2.0;
const MICRO_RESERVOIRS_PER_POSITION: usize = 3;
const MICRO_RESERVOIR_D: f64 = 11.0;
const MICRO_RESERVOIR_PITCH: f64 = 21.0;
const EVAPORATION_BALANCE_STEP_MG: f64 = 25.0;

const SHIELD_Z: f64 = 4.0;
const SHIELD_STANDOFF_Z: f64 = 28.0;
const SHIELD_OVERHANG_X: f64 = 24.0;
const SHIELD_OVERHANG_Y: f64 = 20.0;
const DRIP_GUTTER_W: f64 = 12.0;
const DRIP_GUTTER_DEPTH: f64 = 7.0;
const DRIP_WITNESS_COUNT: usize = 8;
const DRIP_WITNESS_D: f64 = 12.0;

const CAP_WITNESS_X: f64 = 40.0;
const CAP_WITNESS_Y: f64 = 24.0;
const CAP_WITNESS_Z: f64 = 7.0;
const CAP_WITNESS_WELL_D: f64 = 12.0;
const CAP_WITNESS_RECESS_DEPTH: f64 = 5.5;
const CAP_SEAL_WITNESS_COUNT: usize = POSITION_COUNT;

const SAMPLE_PANEL_CENTER_X: f64 = 420.0;
const SAMPLE_PANEL_CENTER_Y: f64 = 188.0;
const SAMPLE_PANEL_X: f64 = 318.0;
const SAMPLE_PANEL_Y: f64 = 284.0;
const SAMPLE_PANEL_Z: f64 = 34.0;
const SAMPLE_WELL_COLS: usize = 6;
const SAMPLE_WELL_ROWS: usize = 4;
const SAMPLE_WELL_COUNT: usize = SAMPLE_WELL_COLS * SAMPLE_WELL_ROWS;
const SAMPLE_WELL_D: f64 = 18.0;
const SAMPLE_WELL_DEPTH: f64 = 17.0;
const SAMPLE_PITCH_X: f64 = 42.0;
const SAMPLE_PITCH_Y: f64 = 56.0;
const DYE_WELL_COUNT: usize = 12;
const OSMOLALITY_WELL_COUNT: usize = SAMPLE_WELL_COUNT - DYE_WELL_COUNT;

const CUSTODY_PANEL_CENTER_X: f64 = 418.0;
const CUSTODY_PANEL_CENTER_Y: f64 = -86.0;
const CUSTODY_PANEL_X: f64 = 342.0;
const CUSTODY_PANEL_Y: f64 = 188.0;
const CUSTODY_PANEL_Z: f64 = 18.0;
const POSITION_BARCODE_COUNT: usize = POSITION_COUNT;
const LOGGER_BARCODE_COUNT: usize = LOGGER_COUNT;
const CUSTODY_CARD_COUNT: usize = 4;
const BARCODE_LAND_X: f64 = 48.0;
const BARCODE_LAND_Y: f64 = 15.0;
const BARCODE_LAND_Z: f64 = 3.0;
const CUSTODY_SEAL_D: f64 = 15.0;

const LANE_BANK_CENTER_X: f64 = 40.0;
const LANE_BANK_CENTER_Y: f64 = -330.0;
const LANE_BANK_X: f64 = 720.0;
const LANE_BANK_Y: f64 = 108.0;
const LANE_BANK_Z: f64 = 30.0;
const DISPOSITION_LANE_COUNT: usize = 3;
const LANE_GAP: f64 = 24.0;
const LANE_RIB_W: f64 = 9.0;
const RELEASE_TOKEN_COUNT: usize = 8;
const HOLD_TOKEN_COUNT: usize = 8;
const REJECT_TOKEN_COUNT: usize = 4;
const STATUS_TOKEN_D: f64 = 16.0;

const BRIDGE_CENTER_X: f64 = 244.0;
const BRIDGE_CENTER_Y: f64 = -204.0;
const BRIDGE_X: f64 = 650.0;
const BRIDGE_Y: f64 = 42.0;
const BRIDGE_Z: f64 = 112.0;
const BRIDGE_POST_W: f64 = 22.0;
const BRIDGE_SHUTTER_COUNT: usize = DISPOSITION_LANE_COUNT;
const EVIDENCE_CARD_SLOTS: usize = 6;

const FRONT_ROBOT_KEEP_OUT_Y: f64 = 198.0;
const SIDE_SERVICE_KEEP_OUT_X: f64 = 110.0;
const LOGGER_SERVICE_PULL_X: f64 = 122.0;
const SAMPLE_SERVICE_PULL_Y: f64 = 156.0;
const VERTICAL_PICK_CLEARANCE_Z: f64 = 156.0;
const KEEP_OUT_GAUGE_Z: f64 = 16.0;

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

    fn capacity(self) -> usize {
        match self {
            DispositionLane::Release => RELEASE_TOKEN_COUNT,
            DispositionLane::Hold => HOLD_TOKEN_COUNT,
            DispositionLane::Reject => REJECT_TOKEN_COUNT,
        }
    }
}

#[derive(Clone, Copy)]
struct Footprint {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_tray();
    export(OUTPUTS[0], &base);

    let grid = edge_center_surrogate_grid();
    export(OUTPUTS[1], &grid);

    let loggers = humidity_logger_pockets();
    export(OUTPUTS[2], &loggers);

    let mass = micro_reservoir_mass_pads();
    export(OUTPUTS[3], &mass);

    let shields = condensate_drip_shields();
    export(OUTPUTS[4], &shields);

    let cap_witness = cap_seal_witness_pockets();
    export(OUTPUTS[5], &cap_witness);

    let samples = dye_osmolality_sample_wells();
    export(OUTPUTS[6], &samples);

    let custody = barcode_custody_lands();
    export(OUTPUTS[7], &custody);

    let lanes = release_hold_reject_lanes();
    export(OUTPUTS[8], &lanes);

    let bridge = evidence_bridge();
    export(OUTPUTS[9], &bridge);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[10], &keepouts);

    let assembly = base
        + grid
        + loggers
        + mass
        + shields
        + cap_witness
        + samples
        + custody
        + lanes
        + bridge
        + keepouts;
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed cassette humidity edge evaporation balance station:");
    println!(
        "  Footprint:                  {TRAY_X:.0}mm x {TRAY_Y:.0}mm, {BASE_Z:.0}mm base with closed-tray rims"
    );
    println!(
        "  Cassette map:               {COLS}x{ROWS} surrogate positions, {EDGE_POSITION_COUNT} edge lanes and {CENTER_POSITION_COUNT} center lanes"
    );
    println!(
        "  Humidity evidence:          {LOGGER_COUNT} logger pockets, {DRIP_WITNESS_COUNT} drip witnesses, {CAP_SEAL_WITNESS_COUNT} cap-seal witness pockets"
    );
    println!(
        "  Mass balance:               {POSITION_COUNT} micro-reservoir mass pads, {MICRO_RESERVOIRS_PER_POSITION} reservoirs per pad, {EVAPORATION_BALANCE_STEP_MG:.0}mg witness step"
    );
    println!(
        "  Sampling:                   {DYE_WELL_COUNT} dye wells and {OSMOLALITY_WELL_COUNT} osmolality wells for edge/center drift checks"
    );
    println!(
        "  Traceability:               {POSITION_BARCODE_COUNT} position barcodes, {LOGGER_BARCODE_COUNT} logger barcodes, {CUSTODY_CARD_COUNT} custody cards"
    );
    println!(
        "  Disposition bridge:         release/hold/reject lanes carry {} total study tokens with {BRIDGE_SHUTTER_COUNT} bridge shutters",
        total_disposition_capacity()
    );
    println!(
        "  Robot/service gauges:       {VERTICAL_PICK_CLEARANCE_Z:.0}mm pick height, {SIDE_SERVICE_KEEP_OUT_X:.0}mm side service gauge, {LOGGER_SERVICE_PULL_X:.0}mm logger pull gauge"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_design_constraints() {
    assert_eq!(POSITION_COUNT, 20);
    assert_eq!(REQUIRED_FEATURES.len(), 15);
    assert_eq!(edge_position_count(), EDGE_POSITION_COUNT);
    assert_eq!(center_position_count(), CENTER_POSITION_COUNT);
    assert_eq!(EDGE_POSITION_COUNT + CENTER_POSITION_COUNT, POSITION_COUNT);
    assert_eq!(LOGGER_COUNT, logger_centers().len());
    assert_eq!(SAMPLE_WELL_COUNT, SAMPLE_WELL_COLS * SAMPLE_WELL_ROWS);
    assert_eq!(SAMPLE_WELL_COUNT, DYE_WELL_COUNT + OSMOLALITY_WELL_COUNT);
    assert_eq!(DISPOSITION_LANE_COUNT, DispositionLane::all().len());
    assert_eq!(total_disposition_capacity(), POSITION_COUNT);
    assert_eq!(CAP_SEAL_WITNESS_COUNT, POSITION_COUNT);
    assert!(REVC_TOTAL_HEIGHT < POSITION_PAD_Z + GRID_FRAME_Z + 4.0);
    assert!(MASS_PAD_X < POSITION_PAD_X && MASS_PAD_Y < POSITION_PAD_Y);
    assert!(MICRO_RESERVOIR_D + 6.0 < MICRO_RESERVOIR_PITCH);
    assert!(LOGGER_POCKET_DEPTH < LOGGER_BANK_Z);
    assert!(SAMPLE_WELL_DEPTH < SAMPLE_PANEL_Z);
    assert!(SHIELD_STANDOFF_Z > GRID_FRAME_Z + POSITION_PAD_Z);
    assert!(VERTICAL_PICK_CLEARANCE_Z > BASE_Z + SHIELD_STANDOFF_Z + 50.0);
    assert!(LOGGER_SERVICE_PULL_X > LOGGER_POCKET_X + 40.0);
    assert!(SAMPLE_SERVICE_PULL_Y > SAMPLE_PANEL_Y / 2.0);
    for footprint in footprints() {
        assert!(
            fits_on_tray(footprint.center, footprint.x, footprint.y, 18.0),
            "{} footprint exceeds tray bounds",
            footprint.name
        );
    }
    assert!(all_footprints_fit());
}

fn base_tray() -> Part {
    let deck = centered_cube(
        "humidity_edge_balance_station_base_deck",
        TRAY_X,
        TRAY_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);

    deck - station_sockets() - drainage_reliefs() - mount_holes()
        + perimeter_rims()
        + underside_scale_rails()
        + closed_system_reference_datum()
}

fn station_sockets() -> Part {
    let grid_socket = centered_cube(
        "humidity_edge_balance_grid_frame_socket",
        GRID_FRAME_X + 28.0,
        GRID_FRAME_Y + 28.0,
        SOCKET_DEPTH + 0.4,
    )
    .translate(
        GRID_CENTER_X,
        GRID_CENTER_Y,
        BASE_Z - SOCKET_DEPTH / 2.0 + 0.2,
    );
    let logger_socket = centered_cube(
        "humidity_edge_balance_logger_bank_socket",
        LOGGER_BANK_X + 12.0,
        LOGGER_BANK_Y + 12.0,
        SOCKET_DEPTH + 0.4,
    )
    .translate(
        LOGGER_BANK_CENTER_X,
        LOGGER_BANK_CENTER_Y,
        BASE_Z - SOCKET_DEPTH / 2.0 + 0.2,
    );
    let sample_socket = centered_cube(
        "humidity_edge_balance_sample_panel_socket",
        SAMPLE_PANEL_X + 12.0,
        SAMPLE_PANEL_Y + 12.0,
        SOCKET_DEPTH + 0.4,
    )
    .translate(
        SAMPLE_PANEL_CENTER_X,
        SAMPLE_PANEL_CENTER_Y,
        BASE_Z - SOCKET_DEPTH / 2.0 + 0.2,
    );
    let custody_socket = centered_cube(
        "humidity_edge_balance_custody_panel_socket",
        CUSTODY_PANEL_X + 12.0,
        CUSTODY_PANEL_Y + 12.0,
        SOCKET_DEPTH + 0.4,
    )
    .translate(
        CUSTODY_PANEL_CENTER_X,
        CUSTODY_PANEL_CENTER_Y,
        BASE_Z - SOCKET_DEPTH / 2.0 + 0.2,
    );
    let lane_socket = centered_cube(
        "humidity_edge_balance_disposition_lane_socket",
        LANE_BANK_X + 12.0,
        LANE_BANK_Y + 12.0,
        SOCKET_DEPTH + 0.4,
    )
    .translate(
        LANE_BANK_CENTER_X,
        LANE_BANK_CENTER_Y,
        BASE_Z - SOCKET_DEPTH / 2.0 + 0.2,
    );
    let bridge_socket = centered_cube(
        "humidity_edge_balance_evidence_bridge_socket",
        BRIDGE_X + 20.0,
        BRIDGE_Y + 18.0,
        SOCKET_DEPTH + 0.4,
    )
    .translate(
        BRIDGE_CENTER_X,
        BRIDGE_CENTER_Y,
        BASE_Z - SOCKET_DEPTH / 2.0 + 0.2,
    );

    grid_socket + logger_socket + sample_socket + custody_socket + lane_socket + bridge_socket
}

fn drainage_reliefs() -> Part {
    let front = centered_cube(
        "humidity_edge_balance_front_condensate_runoff_relief",
        TRAY_X - 134.0,
        DRIP_GUTTER_W,
        DRIP_GUTTER_DEPTH,
    )
    .translate(0.0, tray_front_y() + 54.0, BASE_Z - DRIP_GUTTER_DEPTH / 2.0);
    let grid_left = centered_cube(
        "humidity_edge_balance_left_edge_lane_runoff_relief",
        DRIP_GUTTER_W,
        GRID_FRAME_Y + 52.0,
        DRIP_GUTTER_DEPTH,
    )
    .translate(
        grid_left_edge() - 26.0,
        GRID_CENTER_Y,
        BASE_Z - DRIP_GUTTER_DEPTH / 2.0,
    );
    let grid_right = centered_cube(
        "humidity_edge_balance_right_edge_lane_runoff_relief",
        DRIP_GUTTER_W,
        GRID_FRAME_Y + 52.0,
        DRIP_GUTTER_DEPTH,
    )
    .translate(
        grid_right_edge() + 26.0,
        GRID_CENTER_Y,
        BASE_Z - DRIP_GUTTER_DEPTH / 2.0,
    );
    let sample_spill = centered_cube(
        "humidity_edge_balance_sample_panel_spill_relief",
        SAMPLE_PANEL_X + 42.0,
        DRIP_GUTTER_W,
        DRIP_GUTTER_DEPTH,
    )
    .translate(
        SAMPLE_PANEL_CENTER_X,
        SAMPLE_PANEL_CENTER_Y - SAMPLE_PANEL_Y / 2.0 - 22.0,
        BASE_Z - DRIP_GUTTER_DEPTH / 2.0,
    );

    front + grid_left + grid_right + sample_spill
}

fn mount_holes() -> Part {
    let mut holes = Part::empty("humidity_edge_balance_station_mount_holes");
    for (i, (x, y)) in mount_points().iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("humidity_edge_balance_m5_mount_hole_{i}"),
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
        "humidity_edge_balance_rear_closed_system_rim",
        TRAY_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, tray_rear_y() - RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "humidity_edge_balance_left_logger_service_rim",
        RIM_W,
        TRAY_Y,
        RIM_Z,
    )
    .translate(tray_left_x() + RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "humidity_edge_balance_right_sample_service_rim",
        RIM_W,
        TRAY_Y,
        RIM_Z,
    )
    .translate(tray_right_x() - RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let front = centered_cube(
        "humidity_edge_balance_front_robot_low_lip",
        TRAY_X - 150.0,
        13.0,
        20.0,
    )
    .translate(0.0, tray_front_y() + 18.0, BASE_Z + 10.0);

    rear + left + right + front
}

fn underside_scale_rails() -> Part {
    let left = centered_cube(
        "humidity_edge_balance_underside_left_scale_rail",
        18.0,
        TRAY_Y - 134.0,
        9.0,
    )
    .translate(tray_left_x() + 78.0, 0.0, 4.5);
    let right = centered_cube(
        "humidity_edge_balance_underside_right_scale_rail",
        18.0,
        TRAY_Y - 134.0,
        9.0,
    )
    .translate(tray_right_x() - 78.0, 0.0, 4.5);
    let rear_stop = centered_cube(
        "humidity_edge_balance_underside_rear_scale_stop",
        TRAY_X - 198.0,
        18.0,
        9.0,
    )
    .translate(0.0, tray_rear_y() - 78.0, 4.5);

    left + right + rear_stop
}

fn closed_system_reference_datum() -> Part {
    let datum_bar = centered_cube(
        "humidity_edge_balance_closed_cassette_reference_datum_bar",
        GRID_FRAME_X + 48.0,
        10.0,
        12.0,
    )
    .translate(
        GRID_CENTER_X,
        grid_rear_edge() + 26.0,
        BASE_Z + GRID_FRAME_Z + 6.0,
    );
    let mut pins = Part::empty("humidity_edge_balance_closed_cassette_reference_datum_pins");
    for (i, x) in [
        grid_left_edge() + 44.0,
        GRID_CENTER_X,
        grid_right_edge() - 44.0,
    ]
    .iter()
    .enumerate()
    {
        pins = pins
            + centered_cylinder(
                format!("humidity_edge_balance_reference_datum_pin_{i}"),
                5.0,
                12.0,
                28,
            )
            .translate(*x, grid_rear_edge() + 26.0, BASE_Z + GRID_FRAME_Z + 6.0);
    }
    datum_bar + pins
}

fn edge_center_surrogate_grid() -> Part {
    let frame = centered_cube(
        "humidity_edge_balance_cassette_surrogate_grid_frame",
        GRID_FRAME_X,
        GRID_FRAME_Y,
        GRID_FRAME_Z,
    )
    .translate(GRID_CENTER_X, GRID_CENTER_Y, BASE_Z + GRID_FRAME_Z / 2.0);

    frame - grid_position_recesses()
        + grid_ribs()
        + cassette_position_cells()
        + edge_center_zone_tokens()
        + sealed_cassette_datum_rails()
}

fn grid_position_recesses() -> Part {
    let mut cuts = Part::empty("humidity_edge_balance_position_recesses");
    for row in 0..ROWS {
        for col in 0..COLS {
            let (x, y) = position_center(col, row);
            cuts = cuts
                + centered_cube(
                    format!("humidity_edge_balance_position_socket_{col}_{row}"),
                    POSITION_PAD_X + 10.0,
                    POSITION_PAD_Y + 10.0,
                    POSITION_RECESS_DEPTH + 0.4,
                )
                .translate(
                    x,
                    y,
                    BASE_Z + GRID_FRAME_Z - POSITION_RECESS_DEPTH / 2.0 + 0.2,
                );
        }
    }
    cuts
}

fn grid_ribs() -> Part {
    let mut ribs = Part::empty("humidity_edge_balance_grid_ribs");
    for col in 0..COLS - 1 {
        let (left_x, _) = position_center(col, 0);
        let (right_x, _) = position_center(col + 1, 0);
        let x = (left_x + right_x) / 2.0;
        ribs = ribs
            + centered_cube(
                format!("humidity_edge_balance_column_evaporation_baffle_{col}"),
                GRID_RIB_W,
                ARRAY_Y + 18.0,
                GRID_RIB_Z,
            )
            .translate(x, GRID_CENTER_Y, BASE_Z + GRID_FRAME_Z + GRID_RIB_Z / 2.0);
    }
    for row in 0..ROWS - 1 {
        let (_, lower_y) = position_center(0, row);
        let (_, upper_y) = position_center(0, row + 1);
        let y = (lower_y + upper_y) / 2.0;
        ribs = ribs
            + centered_cube(
                format!("humidity_edge_balance_row_evaporation_baffle_{row}"),
                ARRAY_X + 18.0,
                GRID_RIB_W,
                GRID_RIB_Z,
            )
            .translate(GRID_CENTER_X, y, BASE_Z + GRID_FRAME_Z + GRID_RIB_Z / 2.0);
    }
    ribs
}

fn cassette_position_cells() -> Part {
    let mut cells = Part::empty("humidity_edge_balance_cassette_position_cells");
    for row in 0..ROWS {
        for col in 0..COLS {
            let (x, y) = position_center(col, row);
            cells = cells
                + cassette_position_cell(col, row).translate(
                    x,
                    y,
                    BASE_Z + GRID_FRAME_Z + POSITION_PAD_Z / 2.0,
                );
        }
    }
    cells
}

fn cassette_position_cell(col: usize, row: usize) -> Part {
    let zone = position_zone(col, row);
    let label = match zone {
        MapZone::Edge => "edge",
        MapZone::Center => "center",
    };
    let body = centered_cube(
        format!("humidity_edge_balance_{label}_surrogate_cell_{col}_{row}"),
        POSITION_PAD_X,
        POSITION_PAD_Y,
        POSITION_PAD_Z,
    );
    let humidity_lane = centered_cube(
        format!("humidity_edge_balance_{label}_lane_evaporation_channel_{col}_{row}"),
        POSITION_PAD_X - 26.0,
        8.0,
        POSITION_RECESS_DEPTH + 0.8,
    )
    .translate(0.0, -POSITION_PAD_Y / 2.0 + 18.0, POSITION_PAD_Z / 2.0);
    let finger_relief = centered_cube(
        format!("humidity_edge_balance_{label}_robot_finger_relief_{col}_{row}"),
        17.0,
        POSITION_PAD_Y + 2.0,
        POSITION_RECESS_DEPTH + 0.8,
    )
    .translate(POSITION_PAD_X / 2.0 - 18.0, 0.0, POSITION_PAD_Z / 2.0);
    let rail_z = match zone {
        MapZone::Edge => EDGE_LANE_RAIL_Z,
        MapZone::Center => CENTER_LANE_RAIL_Z,
    };
    let left_rail = centered_cube(
        format!("humidity_edge_balance_{label}_lane_left_rail_{col}_{row}"),
        6.0,
        POSITION_PAD_Y - 10.0,
        rail_z,
    )
    .translate(
        -POSITION_PAD_X / 2.0 + 9.0,
        0.0,
        POSITION_PAD_Z / 2.0 + rail_z / 2.0,
    );
    let right_rail = centered_cube(
        format!("humidity_edge_balance_{label}_lane_right_rail_{col}_{row}"),
        6.0,
        POSITION_PAD_Y - 10.0,
        rail_z,
    )
    .translate(
        POSITION_PAD_X / 2.0 - 9.0,
        0.0,
        POSITION_PAD_Z / 2.0 + rail_z / 2.0,
    );

    body - humidity_lane - finger_relief + left_rail + right_rail
}

fn edge_center_zone_tokens() -> Part {
    let mut tokens = Part::empty("humidity_edge_balance_edge_center_zone_tokens");
    for row in 0..ROWS {
        for col in 0..COLS {
            let (x, y) = position_center(col, row);
            let (d, z, y_offset, label) = match position_zone(col, row) {
                MapZone::Edge => (EDGE_TOKEN_D, 3.0, POSITION_PAD_Y / 2.0 - 13.0, "edge"),
                MapZone::Center => (CENTER_TOKEN_D, 5.0, POSITION_PAD_Y / 2.0 - 13.0, "center"),
            };
            tokens = tokens
                + centered_cylinder(
                    format!("humidity_edge_balance_{label}_zone_token_{col}_{row}"),
                    d / 2.0,
                    z,
                    28,
                )
                .translate(
                    x - POSITION_PAD_X / 2.0 + 18.0,
                    y + y_offset,
                    BASE_Z + 29.0,
                );
        }
    }
    tokens
}

fn sealed_cassette_datum_rails() -> Part {
    let left = centered_cube(
        "humidity_edge_balance_left_closed_cassette_datum_rail",
        12.0,
        GRID_FRAME_Y + 18.0,
        20.0,
    )
    .translate(
        grid_left_edge() + 15.0,
        GRID_CENTER_Y,
        BASE_Z + GRID_FRAME_Z + 10.0,
    );
    let front = centered_cube(
        "humidity_edge_balance_front_closed_cassette_datum_rail",
        GRID_FRAME_X - 28.0,
        12.0,
        16.0,
    )
    .translate(
        GRID_CENTER_X,
        grid_front_edge() + 15.0,
        BASE_Z + GRID_FRAME_Z + 8.0,
    );
    let soft_right_stop = centered_cube(
        "humidity_edge_balance_right_soft_cassette_stop_rail",
        9.0,
        GRID_FRAME_Y - 42.0,
        13.0,
    )
    .translate(
        grid_right_edge() - 15.0,
        GRID_CENTER_Y,
        BASE_Z + GRID_FRAME_Z + 6.5,
    );

    left + front + soft_right_stop
}

fn humidity_logger_pockets() -> Part {
    let bank = centered_cube(
        "humidity_edge_balance_humidity_logger_bank",
        LOGGER_BANK_X,
        LOGGER_BANK_Y,
        LOGGER_BANK_Z,
    )
    .translate(
        LOGGER_BANK_CENTER_X,
        LOGGER_BANK_CENTER_Y,
        BASE_Z + LOGGER_BANK_Z / 2.0,
    );

    bank - logger_pocket_cuts() - logger_lanyard_slots()
        + logger_retention_lips()
        + logger_diffusion_louver_ribs()
        + logger_edge_center_reference_tabs()
}

fn logger_pocket_cuts() -> Part {
    let mut pockets = Part::empty("humidity_edge_balance_logger_pocket_cuts");
    for (i, (x, y)) in logger_centers().iter().enumerate() {
        pockets = pockets
            + centered_cube(
                format!("humidity_edge_balance_logger_recess_{i}"),
                LOGGER_POCKET_X,
                LOGGER_POCKET_Y,
                LOGGER_POCKET_DEPTH + 0.8,
            )
            .translate(
                *x,
                *y,
                BASE_Z + LOGGER_BANK_Z - LOGGER_POCKET_DEPTH / 2.0 + 0.2,
            );
    }
    pockets
}

fn logger_lanyard_slots() -> Part {
    let mut slots = Part::empty("humidity_edge_balance_logger_lanyard_slots");
    for (i, (x, y)) in logger_centers().iter().enumerate() {
        slots = slots
            + centered_cube(
                format!("humidity_edge_balance_logger_lanyard_slot_{i}"),
                LOGGER_LANYARD_SLOT_W,
                LOGGER_POCKET_Y + 26.0,
                LOGGER_POCKET_DEPTH + 1.0,
            )
            .translate(
                *x + LOGGER_POCKET_X / 2.0 - 10.0,
                *y,
                BASE_Z + LOGGER_BANK_Z - LOGGER_POCKET_DEPTH / 2.0,
            );
    }
    slots
}

fn logger_retention_lips() -> Part {
    let mut lips = Part::empty("humidity_edge_balance_logger_retention_lips");
    for (i, (x, y)) in logger_centers().iter().enumerate() {
        let front = centered_cube(
            format!("humidity_edge_balance_logger_front_retention_lip_{i}"),
            LOGGER_POCKET_X + 12.0,
            5.0,
            5.0,
        )
        .translate(
            *x,
            *y - LOGGER_POCKET_Y / 2.0 - 4.0,
            BASE_Z + LOGGER_BANK_Z + 2.5,
        );
        let rear = centered_cube(
            format!("humidity_edge_balance_logger_rear_retention_lip_{i}"),
            LOGGER_POCKET_X + 12.0,
            5.0,
            5.0,
        )
        .translate(
            *x,
            *y + LOGGER_POCKET_Y / 2.0 + 4.0,
            BASE_Z + LOGGER_BANK_Z + 2.5,
        );
        lips = lips + front + rear;
    }
    lips
}

fn logger_diffusion_louver_ribs() -> Part {
    let mut ribs = Part::empty("humidity_edge_balance_logger_diffusion_louver_ribs");
    for (i, (x, y)) in logger_centers().iter().enumerate() {
        for slot in 0..LOGGER_DIFFUSION_SLOT_COUNT {
            ribs = ribs
                + centered_cube(
                    format!("humidity_edge_balance_logger_louver_rib_{i}_{slot}"),
                    LOGGER_POCKET_X - 18.0,
                    1.8,
                    3.0,
                )
                .translate(
                    *x,
                    *y - 14.0 + slot as f64 * 7.0,
                    BASE_Z + LOGGER_BANK_Z + 1.5,
                );
        }
    }
    ribs
}

fn logger_edge_center_reference_tabs() -> Part {
    let edge_tab = centered_cube(
        "humidity_edge_balance_logger_edge_reference_tab",
        LOGGER_BANK_X - 28.0,
        12.0,
        6.0,
    )
    .translate(
        LOGGER_BANK_CENTER_X,
        LOGGER_BANK_CENTER_Y + LOGGER_BANK_Y / 2.0 - 24.0,
        BASE_Z + LOGGER_BANK_Z + 3.0,
    );
    let center_tab = centered_cube(
        "humidity_edge_balance_logger_center_reference_tab",
        LOGGER_BANK_X - 28.0,
        12.0,
        9.0,
    )
    .translate(
        LOGGER_BANK_CENTER_X,
        LOGGER_BANK_CENTER_Y - LOGGER_BANK_Y / 2.0 + 24.0,
        BASE_Z + LOGGER_BANK_Z + 4.5,
    );

    edge_tab + center_tab
}

fn micro_reservoir_mass_pads() -> Part {
    let mut pads = Part::empty("humidity_edge_balance_micro_reservoir_mass_pads");
    for row in 0..ROWS {
        for col in 0..COLS {
            let (x, y) = position_center(col, row);
            pads = pads
                + micro_reservoir_mass_pad(col, row).translate(
                    x,
                    y - 10.0,
                    BASE_Z + GRID_FRAME_Z + POSITION_PAD_Z + MASS_PAD_Z / 2.0,
                );
        }
    }
    pads
}

fn micro_reservoir_mass_pad(col: usize, row: usize) -> Part {
    let zone = position_zone(col, row);
    let label = match zone {
        MapZone::Edge => "edge",
        MapZone::Center => "center",
    };
    let pad = centered_cube(
        format!("humidity_edge_balance_{label}_micro_reservoir_weigh_pad_{col}_{row}"),
        MASS_PAD_X,
        MASS_PAD_Y,
        MASS_PAD_Z,
    );
    let mut cuts = Part::empty(format!(
        "humidity_edge_balance_{label}_micro_reservoir_recesses_{col}_{row}"
    ));
    let mut bosses = Part::empty(format!(
        "humidity_edge_balance_{label}_micro_reservoir_mass_step_bosses_{col}_{row}"
    ));
    for r in 0..MICRO_RESERVOIRS_PER_POSITION {
        let x = centered_index(r, MICRO_RESERVOIRS_PER_POSITION, MICRO_RESERVOIR_PITCH);
        cuts = cuts
            + centered_cylinder(
                format!("humidity_edge_balance_{label}_micro_reservoir_recess_{col}_{row}_{r}"),
                MICRO_RESERVOIR_D / 2.0,
                MASS_PAD_RELIEF_DEPTH + 0.8,
                28,
            )
            .translate(x, -4.0, MASS_PAD_Z / 2.0);
        bosses = bosses
            + centered_cylinder(
                format!("humidity_edge_balance_{label}_mass_reference_dot_{col}_{row}_{r}"),
                2.0 + r as f64 * 0.35,
                1.2,
                20,
            )
            .translate(x, MASS_PAD_Y / 2.0 - 7.0, MASS_PAD_Z / 2.0 + 0.6);
    }
    let side_key = centered_cube(
        format!("humidity_edge_balance_{label}_mass_pad_zone_key_{col}_{row}"),
        9.0,
        MASS_PAD_Y - 8.0,
        if zone == MapZone::Edge { 2.4 } else { 1.4 },
    )
    .translate(
        -MASS_PAD_X / 2.0 + 8.0,
        0.0,
        MASS_PAD_Z / 2.0 + if zone == MapZone::Edge { 1.2 } else { 0.7 },
    );

    pad - cuts + bosses + side_key
}

fn condensate_drip_shields() -> Part {
    edge_lane_shields() + drip_gutters() + drip_witness_cups()
}

fn edge_lane_shields() -> Part {
    let front = shield_plate(
        "humidity_edge_balance_front_edge_lane_condensate_shield",
        GRID_FRAME_X + SHIELD_OVERHANG_X,
        SHIELD_OVERHANG_Y,
        GRID_CENTER_X,
        grid_front_edge() - SHIELD_OVERHANG_Y / 2.0 + 4.0,
    );
    let rear = shield_plate(
        "humidity_edge_balance_rear_edge_lane_condensate_shield",
        GRID_FRAME_X + SHIELD_OVERHANG_X,
        SHIELD_OVERHANG_Y,
        GRID_CENTER_X,
        grid_rear_edge() + SHIELD_OVERHANG_Y / 2.0 - 4.0,
    );
    let left = shield_plate(
        "humidity_edge_balance_left_edge_lane_condensate_shield",
        SHIELD_OVERHANG_X,
        GRID_FRAME_Y,
        grid_left_edge() - SHIELD_OVERHANG_X / 2.0 + 4.0,
        GRID_CENTER_Y,
    );
    let right = shield_plate(
        "humidity_edge_balance_right_edge_lane_condensate_shield",
        SHIELD_OVERHANG_X,
        GRID_FRAME_Y,
        grid_right_edge() + SHIELD_OVERHANG_X / 2.0 - 4.0,
        GRID_CENTER_Y,
    );

    front + rear + left + right + shield_standoffs()
}

fn shield_plate(name: &str, x: f64, y: f64, cx: f64, cy: f64) -> Part {
    centered_cube(name, x, y, SHIELD_Z).translate(cx, cy, BASE_Z + SHIELD_STANDOFF_Z)
}

fn shield_standoffs() -> Part {
    let mut standoffs = Part::empty("humidity_edge_balance_condensate_shield_standoffs");
    for (i, (x, y)) in [
        (grid_left_edge() - 8.0, grid_front_edge() - 8.0),
        (grid_right_edge() + 8.0, grid_front_edge() - 8.0),
        (grid_left_edge() - 8.0, grid_rear_edge() + 8.0),
        (grid_right_edge() + 8.0, grid_rear_edge() + 8.0),
        (GRID_CENTER_X, grid_front_edge() - 8.0),
        (GRID_CENTER_X, grid_rear_edge() + 8.0),
    ]
    .iter()
    .enumerate()
    {
        standoffs = standoffs
            + centered_cylinder(
                format!("humidity_edge_balance_condensate_shield_standoff_{i}"),
                4.0,
                SHIELD_STANDOFF_Z - GRID_FRAME_Z,
                24,
            )
            .translate(
                *x,
                *y,
                BASE_Z + GRID_FRAME_Z + (SHIELD_STANDOFF_Z - GRID_FRAME_Z) / 2.0,
            );
    }
    standoffs
}

fn drip_gutters() -> Part {
    let front = centered_cube(
        "humidity_edge_balance_front_drip_gutter",
        GRID_FRAME_X + 88.0,
        DRIP_GUTTER_W,
        DRIP_GUTTER_DEPTH,
    )
    .translate(
        GRID_CENTER_X,
        grid_front_edge() - 34.0,
        BASE_Z + DRIP_GUTTER_DEPTH / 2.0,
    );
    let rear = centered_cube(
        "humidity_edge_balance_rear_drip_gutter",
        GRID_FRAME_X + 88.0,
        DRIP_GUTTER_W,
        DRIP_GUTTER_DEPTH,
    )
    .translate(
        GRID_CENTER_X,
        grid_rear_edge() + 34.0,
        BASE_Z + DRIP_GUTTER_DEPTH / 2.0,
    );
    let left = centered_cube(
        "humidity_edge_balance_left_drip_gutter",
        DRIP_GUTTER_W,
        GRID_FRAME_Y + 64.0,
        DRIP_GUTTER_DEPTH,
    )
    .translate(
        grid_left_edge() - 34.0,
        GRID_CENTER_Y,
        BASE_Z + DRIP_GUTTER_DEPTH / 2.0,
    );
    let right = centered_cube(
        "humidity_edge_balance_right_drip_gutter",
        DRIP_GUTTER_W,
        GRID_FRAME_Y + 64.0,
        DRIP_GUTTER_DEPTH,
    )
    .translate(
        grid_right_edge() + 34.0,
        GRID_CENTER_Y,
        BASE_Z + DRIP_GUTTER_DEPTH / 2.0,
    );

    front + rear + left + right
}

fn drip_witness_cups() -> Part {
    let mut cups = Part::empty("humidity_edge_balance_condensate_drip_witness_cups");
    for i in 0..DRIP_WITNESS_COUNT {
        let x = GRID_CENTER_X + centered_index(i % 4, 4, (GRID_FRAME_X - 120.0) / 3.0);
        let y = if i < 4 {
            grid_front_edge() - 54.0
        } else {
            grid_rear_edge() + 54.0
        };
        let cup = centered_cylinder(
            format!("humidity_edge_balance_drip_witness_cup_body_{i}"),
            DRIP_WITNESS_D / 2.0 + 3.0,
            5.0,
            28,
        );
        let cut = centered_cylinder(
            format!("humidity_edge_balance_drip_witness_cup_recess_{i}"),
            DRIP_WITNESS_D / 2.0,
            4.0,
            28,
        )
        .translate(0.0, 0.0, 1.2);
        cups = cups + (cup - cut).translate(x, y, BASE_Z + 2.5);
    }
    cups
}

fn cap_seal_witness_pockets() -> Part {
    let mut pockets = Part::empty("humidity_edge_balance_cap_seal_witness_pockets");
    for row in 0..ROWS {
        for col in 0..COLS {
            let (x, y) = position_center(col, row);
            let label = match position_zone(col, row) {
                MapZone::Edge => "edge",
                MapZone::Center => "center",
            };
            let body = centered_cube(
                format!("humidity_edge_balance_{label}_cap_seal_witness_body_{col}_{row}"),
                CAP_WITNESS_X,
                CAP_WITNESS_Y,
                CAP_WITNESS_Z,
            );
            let cap_cup = centered_cylinder(
                format!("humidity_edge_balance_{label}_cap_seal_witness_recess_{col}_{row}"),
                CAP_WITNESS_WELL_D / 2.0,
                CAP_WITNESS_RECESS_DEPTH + 0.8,
                28,
            )
            .translate(-8.0, 0.0, CAP_WITNESS_Z / 2.0);
            let seal_tail = centered_cube(
                format!("humidity_edge_balance_{label}_cap_seal_tail_slot_{col}_{row}"),
                18.0,
                4.0,
                CAP_WITNESS_RECESS_DEPTH + 0.8,
            )
            .translate(9.0, 0.0, CAP_WITNESS_Z / 2.0);
            let witness = (body - cap_cup - seal_tail).translate(
                x + POSITION_PAD_X / 2.0 - 28.0,
                y + POSITION_PAD_Y / 2.0 - 18.0,
                BASE_Z + GRID_FRAME_Z + POSITION_PAD_Z + CAP_WITNESS_Z / 2.0,
            );
            pockets = pockets + witness;
        }
    }
    pockets
}

fn dye_osmolality_sample_wells() -> Part {
    let panel = centered_cube(
        "humidity_edge_balance_dye_osmolality_sample_panel",
        SAMPLE_PANEL_X,
        SAMPLE_PANEL_Y,
        SAMPLE_PANEL_Z,
    )
    .translate(
        SAMPLE_PANEL_CENTER_X,
        SAMPLE_PANEL_CENTER_Y,
        BASE_Z + SAMPLE_PANEL_Z / 2.0,
    );

    panel - sample_well_cuts()
        + sample_well_rims()
        + sample_lane_dividers()
        + edge_center_sample_reference_keys()
}

fn sample_well_cuts() -> Part {
    let mut cuts = Part::empty("humidity_edge_balance_sample_well_cuts");
    for row in 0..SAMPLE_WELL_ROWS {
        for col in 0..SAMPLE_WELL_COLS {
            let i = row * SAMPLE_WELL_COLS + col;
            let (x, y) = sample_well_center(col, row);
            cuts = cuts
                + centered_cylinder(
                    format!("humidity_edge_balance_sample_well_cut_{i}_{col}_{row}"),
                    SAMPLE_WELL_D / 2.0,
                    SAMPLE_WELL_DEPTH + 1.0,
                    32,
                )
                .translate(
                    x,
                    y,
                    BASE_Z + SAMPLE_PANEL_Z - SAMPLE_WELL_DEPTH / 2.0 + 0.3,
                );
        }
    }
    cuts
}

fn sample_well_rims() -> Part {
    let mut rims = Part::empty("humidity_edge_balance_sample_well_rims");
    for row in 0..SAMPLE_WELL_ROWS {
        for col in 0..SAMPLE_WELL_COLS {
            let i = row * SAMPLE_WELL_COLS + col;
            let (x, y) = sample_well_center(col, row);
            let label = if i < DYE_WELL_COUNT { "dye" } else { "osmo" };
            rims = rims
                + centered_cylinder(
                    format!("humidity_edge_balance_{label}_sample_well_rim_{i}"),
                    SAMPLE_WELL_D / 2.0 + 3.2,
                    3.0,
                    32,
                )
                .translate(x, y, BASE_Z + SAMPLE_PANEL_Z + 1.5);
        }
    }
    rims
}

fn sample_lane_dividers() -> Part {
    let dye_osmo_split = centered_cube(
        "humidity_edge_balance_dye_osmolality_split_rib",
        SAMPLE_PANEL_X - 40.0,
        8.0,
        8.0,
    )
    .translate(
        SAMPLE_PANEL_CENTER_X,
        SAMPLE_PANEL_CENTER_Y,
        BASE_Z + SAMPLE_PANEL_Z + 4.0,
    );
    let edge_center_split = centered_cube(
        "humidity_edge_balance_edge_center_sample_split_rib",
        8.0,
        SAMPLE_PANEL_Y - 38.0,
        8.0,
    )
    .translate(
        SAMPLE_PANEL_CENTER_X,
        SAMPLE_PANEL_CENTER_Y,
        BASE_Z + SAMPLE_PANEL_Z + 4.0,
    );
    dye_osmo_split + edge_center_split
}

fn edge_center_sample_reference_keys() -> Part {
    let edge_key = centered_cube(
        "humidity_edge_balance_edge_sample_reference_key",
        92.0,
        16.0,
        5.0,
    )
    .translate(
        SAMPLE_PANEL_CENTER_X - 72.0,
        SAMPLE_PANEL_CENTER_Y + SAMPLE_PANEL_Y / 2.0 - 24.0,
        BASE_Z + SAMPLE_PANEL_Z + 2.5,
    );
    let center_key = centered_cube(
        "humidity_edge_balance_center_sample_reference_key",
        92.0,
        16.0,
        8.0,
    )
    .translate(
        SAMPLE_PANEL_CENTER_X + 72.0,
        SAMPLE_PANEL_CENTER_Y + SAMPLE_PANEL_Y / 2.0 - 24.0,
        BASE_Z + SAMPLE_PANEL_Z + 4.0,
    );
    edge_key + center_key
}

fn barcode_custody_lands() -> Part {
    let panel = centered_cube(
        "humidity_edge_balance_barcode_custody_panel",
        CUSTODY_PANEL_X,
        CUSTODY_PANEL_Y,
        CUSTODY_PANEL_Z,
    )
    .translate(
        CUSTODY_PANEL_CENTER_X,
        CUSTODY_PANEL_CENTER_Y,
        BASE_Z + CUSTODY_PANEL_Z / 2.0,
    );

    panel - custody_card_slots()
        + position_barcode_lands()
        + logger_barcode_lands()
        + custody_seal_lands()
        + run_card_lands()
}

fn position_barcode_lands() -> Part {
    let mut lands = Part::empty("humidity_edge_balance_position_barcode_lands");
    for i in 0..POSITION_BARCODE_COUNT {
        let col = i % 5;
        let row = i / 5;
        let x = CUSTODY_PANEL_CENTER_X - 120.0 + col as f64 * 54.0;
        let y = CUSTODY_PANEL_CENTER_Y + 54.0 - row as f64 * 30.0;
        lands = lands
            + centered_cube(
                format!("humidity_edge_balance_position_barcode_land_{i}"),
                BARCODE_LAND_X,
                BARCODE_LAND_Y,
                BARCODE_LAND_Z,
            )
            .translate(x, y, BASE_Z + CUSTODY_PANEL_Z + BARCODE_LAND_Z / 2.0);
    }
    lands
}

fn logger_barcode_lands() -> Part {
    let mut lands = Part::empty("humidity_edge_balance_logger_barcode_lands");
    for i in 0..LOGGER_BARCODE_COUNT {
        let x = CUSTODY_PANEL_CENTER_X - 136.0 + i as f64 * 38.0;
        let y = CUSTODY_PANEL_CENTER_Y - 72.0;
        lands = lands
            + centered_cube(
                format!("humidity_edge_balance_logger_barcode_land_{i}"),
                32.0,
                BARCODE_LAND_Y,
                BARCODE_LAND_Z,
            )
            .translate(x, y, BASE_Z + CUSTODY_PANEL_Z + BARCODE_LAND_Z / 2.0);
    }
    lands
}

fn custody_seal_lands() -> Part {
    let mut lands = Part::empty("humidity_edge_balance_custody_seal_lands");
    for i in 0..CUSTODY_CARD_COUNT {
        let x = CUSTODY_PANEL_CENTER_X + 122.0;
        let y = CUSTODY_PANEL_CENTER_Y + centered_index(i, CUSTODY_CARD_COUNT, 38.0);
        let land = centered_cylinder(
            format!("humidity_edge_balance_custody_seal_land_{i}"),
            CUSTODY_SEAL_D / 2.0,
            4.0,
            30,
        )
        .translate(x, y, BASE_Z + CUSTODY_PANEL_Z + 2.0);
        let tether = centered_cube(
            format!("humidity_edge_balance_custody_seal_tether_land_{i}"),
            34.0,
            4.0,
            3.0,
        )
        .translate(x - 24.0, y, BASE_Z + CUSTODY_PANEL_Z + 1.5);
        lands = lands + land + tether;
    }
    lands
}

fn run_card_lands() -> Part {
    let mut cards = Part::empty("humidity_edge_balance_run_card_lands");
    for i in 0..CUSTODY_CARD_COUNT {
        let x = CUSTODY_PANEL_CENTER_X - 132.0 + i as f64 * 78.0;
        let y = CUSTODY_PANEL_CENTER_Y - CUSTODY_PANEL_Y / 2.0 + 18.0;
        cards = cards
            + centered_cube(
                format!("humidity_edge_balance_run_card_land_{i}"),
                64.0,
                16.0,
                3.0,
            )
            .translate(x, y, BASE_Z + CUSTODY_PANEL_Z + 1.5);
    }
    cards
}

fn custody_card_slots() -> Part {
    let mut slots = Part::empty("humidity_edge_balance_custody_card_slots");
    for i in 0..CUSTODY_CARD_COUNT {
        let x = CUSTODY_PANEL_CENTER_X - 132.0 + i as f64 * 78.0;
        let y = CUSTODY_PANEL_CENTER_Y - CUSTODY_PANEL_Y / 2.0 + 18.0;
        slots = slots
            + centered_cube(
                format!("humidity_edge_balance_custody_card_slot_{i}"),
                58.0,
                3.2,
                9.0,
            )
            .translate(x, y + 12.0, BASE_Z + CUSTODY_PANEL_Z - 4.0);
    }
    slots
}

fn release_hold_reject_lanes() -> Part {
    let bank = centered_cube(
        "humidity_edge_balance_release_hold_reject_lane_bank",
        LANE_BANK_X,
        LANE_BANK_Y,
        LANE_BANK_Z,
    )
    .translate(
        LANE_BANK_CENTER_X,
        LANE_BANK_CENTER_Y,
        BASE_Z + LANE_BANK_Z / 2.0,
    );

    bank - lane_recesses() + lane_side_walls() + disposition_tokens() + lane_name_keys()
}

fn lane_recesses() -> Part {
    let mut cuts = Part::empty("humidity_edge_balance_lane_recesses");
    for lane in DispositionLane::all() {
        cuts = cuts
            + centered_cube(
                format!("humidity_edge_balance_{}_lane_recess", lane.label()),
                lane_width() - 18.0,
                LANE_BANK_Y - 30.0,
                13.0,
            )
            .translate(
                lane_center_x(lane),
                LANE_BANK_CENTER_Y,
                BASE_Z + LANE_BANK_Z - 6.5,
            );
    }
    cuts
}

fn lane_side_walls() -> Part {
    let mut walls = Part::empty("humidity_edge_balance_lane_side_walls");
    for lane in DispositionLane::all() {
        let x = lane_center_x(lane);
        let left = centered_cube(
            format!("humidity_edge_balance_{}_lane_left_wall", lane.label()),
            LANE_RIB_W,
            LANE_BANK_Y - 16.0,
            16.0,
        )
        .translate(
            x - lane_width() / 2.0 + LANE_RIB_W / 2.0,
            LANE_BANK_CENTER_Y,
            BASE_Z + LANE_BANK_Z + 8.0,
        );
        let right = centered_cube(
            format!("humidity_edge_balance_{}_lane_right_wall", lane.label()),
            LANE_RIB_W,
            LANE_BANK_Y - 16.0,
            16.0,
        )
        .translate(
            x + lane_width() / 2.0 - LANE_RIB_W / 2.0,
            LANE_BANK_CENTER_Y,
            BASE_Z + LANE_BANK_Z + 8.0,
        );
        walls = walls + left + right;
    }
    walls
}

fn disposition_tokens() -> Part {
    let mut tokens = Part::empty("humidity_edge_balance_disposition_tokens");
    for lane in DispositionLane::all() {
        let capacity = lane.capacity();
        for slot in 0..capacity {
            let x = lane_center_x(lane) + centered_index(slot % 4, 4, STATUS_TOKEN_D + 12.0);
            let y = LANE_BANK_CENTER_Y
                + centered_index(slot / 4, (capacity + 3) / 4, STATUS_TOKEN_D + 16.0);
            tokens = tokens
                + centered_cylinder(
                    format!(
                        "humidity_edge_balance_{}_lane_status_token_{slot}",
                        lane.label()
                    ),
                    STATUS_TOKEN_D / 2.0,
                    5.0,
                    28,
                )
                .translate(x, y, BASE_Z + LANE_BANK_Z + 2.5);
        }
    }
    tokens
}

fn lane_name_keys() -> Part {
    let mut keys = Part::empty("humidity_edge_balance_lane_name_keys");
    for lane in DispositionLane::all() {
        let height = match lane {
            DispositionLane::Release => 2.0,
            DispositionLane::Hold => 4.0,
            DispositionLane::Reject => 6.0,
        };
        keys = keys
            + centered_cube(
                format!("humidity_edge_balance_{}_lane_name_key", lane.label()),
                lane_width() - 38.0,
                11.0,
                height,
            )
            .translate(
                lane_center_x(lane),
                LANE_BANK_CENTER_Y + LANE_BANK_Y / 2.0 - 16.0,
                BASE_Z + LANE_BANK_Z + height / 2.0,
            );
    }
    keys
}

fn evidence_bridge() -> Part {
    let left_post = centered_cube(
        "humidity_edge_balance_evidence_bridge_left_post",
        BRIDGE_POST_W,
        BRIDGE_Y,
        BRIDGE_Z,
    )
    .translate(
        BRIDGE_CENTER_X - BRIDGE_X / 2.0 + BRIDGE_POST_W / 2.0,
        BRIDGE_CENTER_Y,
        BASE_Z + BRIDGE_Z / 2.0,
    );
    let right_post = centered_cube(
        "humidity_edge_balance_evidence_bridge_right_post",
        BRIDGE_POST_W,
        BRIDGE_Y,
        BRIDGE_Z,
    )
    .translate(
        BRIDGE_CENTER_X + BRIDGE_X / 2.0 - BRIDGE_POST_W / 2.0,
        BRIDGE_CENTER_Y,
        BASE_Z + BRIDGE_Z / 2.0,
    );
    let crossbeam = centered_cube(
        "humidity_edge_balance_evidence_bridge_crossbeam",
        BRIDGE_X,
        BRIDGE_Y,
        20.0,
    )
    .translate(BRIDGE_CENTER_X, BRIDGE_CENTER_Y, BASE_Z + BRIDGE_Z - 10.0);
    let lower_rail = centered_cube(
        "humidity_edge_balance_evidence_bridge_lower_custody_rail",
        BRIDGE_X - 74.0,
        10.0,
        10.0,
    )
    .translate(BRIDGE_CENTER_X, BRIDGE_CENTER_Y, BASE_Z + 28.0);

    left_post + right_post + crossbeam + lower_rail + bridge_shutters() + evidence_card_slots()
}

fn bridge_shutters() -> Part {
    let mut shutters = Part::empty("humidity_edge_balance_evidence_bridge_shutters");
    for i in 0..BRIDGE_SHUTTER_COUNT {
        shutters = shutters
            + centered_cube(
                format!("humidity_edge_balance_evidence_bridge_shutter_{i}"),
                82.0,
                8.0,
                38.0,
            )
            .translate(
                BRIDGE_CENTER_X + centered_index(i, BRIDGE_SHUTTER_COUNT, 116.0),
                BRIDGE_CENTER_Y - BRIDGE_Y / 2.0 - 4.0,
                BASE_Z + 56.0,
            );
    }
    shutters
}

fn evidence_card_slots() -> Part {
    let mut slots = Part::empty("humidity_edge_balance_evidence_card_slots");
    for i in 0..EVIDENCE_CARD_SLOTS {
        slots = slots
            + centered_cube(
                format!("humidity_edge_balance_evidence_card_slot_land_{i}"),
                54.0,
                7.0,
                4.0,
            )
            .translate(
                BRIDGE_CENTER_X + centered_index(i, EVIDENCE_CARD_SLOTS, 70.0),
                BRIDGE_CENTER_Y + BRIDGE_Y / 2.0 + 5.0,
                BASE_Z + BRIDGE_Z - 24.0,
            );
    }
    slots
}

fn robot_service_keepout_gauges() -> Part {
    front_robot_sweep_gauge()
        + side_service_pull_gauges()
        + vertical_pick_clearance_gauge()
        + sample_probe_keepout_gauge()
}

fn front_robot_sweep_gauge() -> Part {
    let rail = centered_cube(
        "humidity_edge_balance_front_robot_sweep_keepout_gauge",
        TRAY_X - 190.0,
        16.0,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        0.0,
        tray_front_y() + FRONT_ROBOT_KEEP_OUT_Y,
        BASE_Z + KEEP_OUT_GAUGE_Z / 2.0,
    );
    rail + gauge_ticks(
        "humidity_edge_balance_front_robot_sweep_tick",
        -460.0,
        tray_front_y() + FRONT_ROBOT_KEEP_OUT_Y + 16.0,
        10,
        102.0,
        true,
    )
}

fn side_service_pull_gauges() -> Part {
    let left = centered_cube(
        "humidity_edge_balance_logger_side_service_pull_gauge",
        SIDE_SERVICE_KEEP_OUT_X,
        14.0,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        LOGGER_BANK_CENTER_X + LOGGER_BANK_X / 2.0 + SIDE_SERVICE_KEEP_OUT_X / 2.0,
        LOGGER_BANK_CENTER_Y,
        BASE_Z + KEEP_OUT_GAUGE_Z / 2.0,
    );
    let right = centered_cube(
        "humidity_edge_balance_sample_side_service_pull_gauge",
        SIDE_SERVICE_KEEP_OUT_X,
        14.0,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        SAMPLE_PANEL_CENTER_X - SAMPLE_PANEL_X / 2.0 - SIDE_SERVICE_KEEP_OUT_X / 2.0,
        SAMPLE_PANEL_CENTER_Y,
        BASE_Z + KEEP_OUT_GAUGE_Z / 2.0,
    );
    left + right
}

fn vertical_pick_clearance_gauge() -> Part {
    let left_mast = centered_cube(
        "humidity_edge_balance_vertical_pick_clearance_left_mast",
        14.0,
        14.0,
        VERTICAL_PICK_CLEARANCE_Z,
    )
    .translate(
        grid_left_edge() - 58.0,
        grid_front_edge() - 70.0,
        BASE_Z + VERTICAL_PICK_CLEARANCE_Z / 2.0,
    );
    let right_mast = centered_cube(
        "humidity_edge_balance_vertical_pick_clearance_right_mast",
        14.0,
        14.0,
        VERTICAL_PICK_CLEARANCE_Z,
    )
    .translate(
        grid_right_edge() + 58.0,
        grid_front_edge() - 70.0,
        BASE_Z + VERTICAL_PICK_CLEARANCE_Z / 2.0,
    );
    let top_bar = centered_cube(
        "humidity_edge_balance_vertical_pick_clearance_top_bar",
        GRID_FRAME_X + 130.0,
        12.0,
        12.0,
    )
    .translate(
        GRID_CENTER_X,
        grid_front_edge() - 70.0,
        BASE_Z + VERTICAL_PICK_CLEARANCE_Z,
    );

    left_mast + right_mast + top_bar
}

fn sample_probe_keepout_gauge() -> Part {
    let gauge = centered_cube(
        "humidity_edge_balance_sample_probe_service_keepout_gauge",
        SAMPLE_PANEL_X - 34.0,
        12.0,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        SAMPLE_PANEL_CENTER_X,
        SAMPLE_PANEL_CENTER_Y - SAMPLE_PANEL_Y / 2.0 - SAMPLE_SERVICE_PULL_Y / 2.0,
        BASE_Z + KEEP_OUT_GAUGE_Z / 2.0,
    );
    gauge
        + gauge_ticks(
            "humidity_edge_balance_sample_probe_keepout_tick",
            SAMPLE_PANEL_CENTER_X - 126.0,
            SAMPLE_PANEL_CENTER_Y - SAMPLE_PANEL_Y / 2.0 - SAMPLE_SERVICE_PULL_Y / 2.0 - 14.0,
            7,
            42.0,
            false,
        )
}

fn gauge_ticks(
    prefix: &str,
    start_x: f64,
    y: f64,
    count: usize,
    pitch: f64,
    tall_mid: bool,
) -> Part {
    let mut ticks = Part::empty(format!("{prefix}_ticks"));
    for i in 0..count {
        let height = if tall_mid && i == count / 2 {
            18.0
        } else {
            11.0
        };
        ticks = ticks
            + centered_cube(format!("{prefix}_{i}"), 4.0, 10.0, height).translate(
                start_x + i as f64 * pitch,
                y,
                BASE_Z + height / 2.0,
            );
    }
    ticks
}

fn position_zone(col: usize, row: usize) -> MapZone {
    if col == 0 || col == COLS - 1 || row == 0 || row == ROWS - 1 {
        MapZone::Edge
    } else {
        MapZone::Center
    }
}

fn edge_position_count() -> usize {
    let mut count = 0;
    for row in 0..ROWS {
        for col in 0..COLS {
            if position_zone(col, row) == MapZone::Edge {
                count += 1;
            }
        }
    }
    count
}

fn center_position_count() -> usize {
    POSITION_COUNT - edge_position_count()
}

fn position_center(col: usize, row: usize) -> (f64, f64) {
    (
        GRID_CENTER_X + centered_index(col, COLS, REVC_CHIP_LENGTH + CHIP_GUTTER),
        GRID_CENTER_Y + centered_index(row, ROWS, REVC_CHIP_WIDTH + CHIP_GUTTER),
    )
}

fn sample_well_center(col: usize, row: usize) -> (f64, f64) {
    (
        SAMPLE_PANEL_CENTER_X + centered_index(col, SAMPLE_WELL_COLS, SAMPLE_PITCH_X),
        SAMPLE_PANEL_CENTER_Y + centered_index(row, SAMPLE_WELL_ROWS, SAMPLE_PITCH_Y),
    )
}

fn logger_centers() -> [(f64, f64); LOGGER_COUNT] {
    [
        (
            LOGGER_BANK_CENTER_X - LOGGER_PITCH_X / 2.0,
            LOGGER_BANK_CENTER_Y + centered_index(0, 4, LOGGER_PITCH_Y),
        ),
        (
            LOGGER_BANK_CENTER_X + LOGGER_PITCH_X / 2.0,
            LOGGER_BANK_CENTER_Y + centered_index(0, 4, LOGGER_PITCH_Y),
        ),
        (
            LOGGER_BANK_CENTER_X - LOGGER_PITCH_X / 2.0,
            LOGGER_BANK_CENTER_Y + centered_index(1, 4, LOGGER_PITCH_Y),
        ),
        (
            LOGGER_BANK_CENTER_X + LOGGER_PITCH_X / 2.0,
            LOGGER_BANK_CENTER_Y + centered_index(1, 4, LOGGER_PITCH_Y),
        ),
        (
            LOGGER_BANK_CENTER_X - LOGGER_PITCH_X / 2.0,
            LOGGER_BANK_CENTER_Y + centered_index(2, 4, LOGGER_PITCH_Y),
        ),
        (
            LOGGER_BANK_CENTER_X + LOGGER_PITCH_X / 2.0,
            LOGGER_BANK_CENTER_Y + centered_index(2, 4, LOGGER_PITCH_Y),
        ),
        (
            LOGGER_BANK_CENTER_X - LOGGER_PITCH_X / 2.0,
            LOGGER_BANK_CENTER_Y + centered_index(3, 4, LOGGER_PITCH_Y),
        ),
        (
            LOGGER_BANK_CENTER_X + LOGGER_PITCH_X / 2.0,
            LOGGER_BANK_CENTER_Y + centered_index(3, 4, LOGGER_PITCH_Y),
        ),
    ]
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn lane_width() -> f64 {
    (LANE_BANK_X - (DISPOSITION_LANE_COUNT as f64 + 1.0) * LANE_GAP) / DISPOSITION_LANE_COUNT as f64
}

fn lane_center_x(lane: DispositionLane) -> f64 {
    LANE_BANK_CENTER_X
        + centered_index(
            lane.index(),
            DISPOSITION_LANE_COUNT,
            lane_width() + LANE_GAP,
        )
}

fn total_disposition_capacity() -> usize {
    DispositionLane::all()
        .iter()
        .map(|lane| lane.capacity())
        .sum()
}

fn tray_left_x() -> f64 {
    -TRAY_X / 2.0
}

fn tray_right_x() -> f64 {
    TRAY_X / 2.0
}

fn tray_front_y() -> f64 {
    -TRAY_Y / 2.0
}

fn tray_rear_y() -> f64 {
    TRAY_Y / 2.0
}

fn grid_left_edge() -> f64 {
    GRID_CENTER_X - GRID_FRAME_X / 2.0
}

fn grid_right_edge() -> f64 {
    GRID_CENTER_X + GRID_FRAME_X / 2.0
}

fn grid_front_edge() -> f64 {
    GRID_CENTER_Y - GRID_FRAME_Y / 2.0
}

fn grid_rear_edge() -> f64 {
    GRID_CENTER_Y + GRID_FRAME_Y / 2.0
}

fn mount_points() -> [(f64, f64); 8] {
    [
        (tray_left_x() + 46.0, tray_front_y() + 42.0),
        (tray_right_x() - 46.0, tray_front_y() + 42.0),
        (tray_left_x() + 46.0, tray_rear_y() - 42.0),
        (tray_right_x() - 46.0, tray_rear_y() - 42.0),
        (0.0, tray_front_y() + 42.0),
        (0.0, tray_rear_y() - 42.0),
        (tray_left_x() + 46.0, 0.0),
        (tray_right_x() - 46.0, 0.0),
    ]
}

fn all_footprints_fit() -> bool {
    footprints()
        .iter()
        .all(|f| fits_on_tray(f.center, f.x, f.y, 18.0))
}

fn footprints() -> [Footprint; 6] {
    [
        Footprint {
            name: "cassette_grid",
            center: (GRID_CENTER_X, GRID_CENTER_Y),
            x: GRID_FRAME_X,
            y: GRID_FRAME_Y,
        },
        Footprint {
            name: "logger_bank",
            center: (LOGGER_BANK_CENTER_X, LOGGER_BANK_CENTER_Y),
            x: LOGGER_BANK_X,
            y: LOGGER_BANK_Y,
        },
        Footprint {
            name: "sample_panel",
            center: (SAMPLE_PANEL_CENTER_X, SAMPLE_PANEL_CENTER_Y),
            x: SAMPLE_PANEL_X,
            y: SAMPLE_PANEL_Y,
        },
        Footprint {
            name: "custody_panel",
            center: (CUSTODY_PANEL_CENTER_X, CUSTODY_PANEL_CENTER_Y),
            x: CUSTODY_PANEL_X,
            y: CUSTODY_PANEL_Y,
        },
        Footprint {
            name: "disposition_lanes",
            center: (LANE_BANK_CENTER_X, LANE_BANK_CENTER_Y),
            x: LANE_BANK_X,
            y: LANE_BANK_Y,
        },
        Footprint {
            name: "evidence_bridge",
            center: (BRIDGE_CENTER_X, BRIDGE_CENTER_Y),
            x: BRIDGE_X,
            y: BRIDGE_Y,
        },
    ]
}

fn fits_on_tray(center: (f64, f64), x: f64, y: f64, margin: f64) -> bool {
    center.0.abs() + x / 2.0 <= TRAY_X / 2.0 - margin
        && center.1.abs() + y / 2.0 <= TRAY_Y / 2.0 - margin
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_contract_is_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(OUTPUTS.len(), 12);
        assert_eq!(unique.len(), OUTPUTS.len());
        assert!(OUTPUTS.iter().all(|path| path
            .starts_with("output/closed_cassette_humidity_edge_evaporation_balance_station_")));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }

    #[test]
    fn required_feature_list_covers_design_intent() {
        for feature in [
            "closed_cassette_surrogate_grid",
            "edge_lane_map",
            "center_lane_map",
            "humidity_logger_pockets",
            "micro_reservoir_mass_pads",
            "condensate_drip_shields",
            "cap_seal_witness_pockets",
            "dye_sample_wells",
            "osmolality_sample_wells",
            "barcode_lands",
            "custody_lands",
            "release_lane",
            "hold_lane",
            "reject_lane",
            "evidence_bridge",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn cassette_grid_maps_edge_and_center_lanes() {
        let mut positions = BTreeSet::new();
        let mut edge = 0;
        let mut center = 0;
        for row in 0..ROWS {
            for col in 0..COLS {
                positions.insert((col, row));
                let (x, y) = position_center(col, row);
                assert!(x > grid_left_edge());
                assert!(x < grid_right_edge());
                assert!(y > grid_front_edge());
                assert!(y < grid_rear_edge());
                match position_zone(col, row) {
                    MapZone::Edge => edge += 1,
                    MapZone::Center => center += 1,
                }
            }
        }

        assert_eq!(positions.len(), POSITION_COUNT);
        assert_eq!(edge, EDGE_POSITION_COUNT);
        assert_eq!(center, CENTER_POSITION_COUNT);
        assert_eq!(edge + center, 20);
    }

    #[test]
    fn logger_mass_and_cap_witness_counts_cover_all_mapping_positions() {
        assert_eq!(logger_centers().len(), LOGGER_COUNT);
        assert_eq!(CAP_SEAL_WITNESS_COUNT, POSITION_COUNT);
        assert_eq!(MICRO_RESERVOIRS_PER_POSITION * POSITION_COUNT, 60);
        assert!(LOGGER_POCKET_DEPTH < LOGGER_BANK_Z);
        assert!(MASS_PAD_X < POSITION_PAD_X);
        assert!(MASS_PAD_Y < POSITION_PAD_Y);
        assert!(MICRO_RESERVOIR_D + 6.0 < MICRO_RESERVOIR_PITCH);
    }

    #[test]
    fn dye_osmolality_and_custody_surfaces_are_batch_sized() {
        assert_eq!(SAMPLE_WELL_COUNT, 24);
        assert_eq!(DYE_WELL_COUNT, 12);
        assert_eq!(OSMOLALITY_WELL_COUNT, 12);
        assert_eq!(POSITION_BARCODE_COUNT, POSITION_COUNT);
        assert_eq!(LOGGER_BARCODE_COUNT, LOGGER_COUNT);
        assert_eq!(CUSTODY_CARD_COUNT, 4);
        assert!(SAMPLE_WELL_DEPTH < SAMPLE_PANEL_Z);
        assert!(BARCODE_LAND_X <= 54.0);
    }

    #[test]
    fn disposition_lanes_balance_all_twenty_positions() {
        assert_eq!(DispositionLane::all().len(), DISPOSITION_LANE_COUNT);
        assert_eq!(DispositionLane::Release.capacity(), 8);
        assert_eq!(DispositionLane::Hold.capacity(), 8);
        assert_eq!(DispositionLane::Reject.capacity(), 4);
        assert_eq!(total_disposition_capacity(), POSITION_COUNT);
        assert!(lane_center_x(DispositionLane::Release) < lane_center_x(DispositionLane::Hold));
        assert!(lane_center_x(DispositionLane::Hold) < lane_center_x(DispositionLane::Reject));
        assert!(lane_width() > 180.0);
    }

    #[test]
    fn condensate_and_keepout_geometry_has_clearance() {
        assert!(SHIELD_STANDOFF_Z > GRID_FRAME_Z + POSITION_PAD_Z);
        assert!(DRIP_GUTTER_W >= DRIP_WITNESS_D - 2.0);
        assert!(VERTICAL_PICK_CLEARANCE_Z > BASE_Z + SHIELD_STANDOFF_Z + 50.0);
        assert!(LOGGER_SERVICE_PULL_X > LOGGER_POCKET_X + 40.0);
        assert!(FRONT_ROBOT_KEEP_OUT_Y > LANE_BANK_Y);
    }

    #[test]
    fn major_subassemblies_fit_on_the_station_deck() {
        for footprint in footprints() {
            assert!(
                fits_on_tray(footprint.center, footprint.x, footprint.y, 18.0),
                "{} footprint exceeds tray bounds",
                footprint.name
            );
        }
        assert!(all_footprints_fit());
    }
}
