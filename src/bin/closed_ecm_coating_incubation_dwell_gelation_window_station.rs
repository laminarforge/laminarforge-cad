use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed ECM coating incubation dwell and gelation-window validation station.
//
// Intent:
// - Validate dwell timing, gelation/viscosity drift, surface wetness, humidity
//   exposure, and edge/center variability before automated cell seeding.
// - Keep a closed isolator deck, 16-position coating witness array, edge/center
//   humidity and temperature witnesses, dwell-token rail, gelation ladder,
//   meniscus windows, residual capture, evidence fiducials, custody lands, and
//   release/hold/reject gates mechanically explicit.
// - This is source-only fixture/interface CAD. It does not define ECM chemistry,
//   acceptance thresholds, sterility claims, or cell-seeding protocol limits.

const OUTPUTS: [&str; 13] = [
    "output/closed_ecm_coating_incubation_dwell_gelation_window_station_base_leak_tray_deck.stl",
    "output/closed_ecm_coating_incubation_dwell_gelation_window_station_sixteen_position_witness_coupon_array.stl",
    "output/closed_ecm_coating_incubation_dwell_gelation_window_station_edge_center_humidity_temperature_witness_wells.stl",
    "output/closed_ecm_coating_incubation_dwell_gelation_window_station_timed_dwell_token_rail.stl",
    "output/closed_ecm_coating_incubation_dwell_gelation_window_station_viscosity_gelation_ladder_coupons.stl",
    "output/closed_ecm_coating_incubation_dwell_gelation_window_station_wetness_meniscus_inspection_windows.stl",
    "output/closed_ecm_coating_incubation_dwell_gelation_window_station_rinse_drain_residual_capture_trough.stl",
    "output/closed_ecm_coating_incubation_dwell_gelation_window_station_camera_illumination_fiducials.stl",
    "output/closed_ecm_coating_incubation_dwell_gelation_window_station_barcode_run_custody_lands.stl",
    "output/closed_ecm_coating_incubation_dwell_gelation_window_station_release_hold_reject_disposition_gate.stl",
    "output/closed_ecm_coating_incubation_dwell_gelation_window_station_closed_isolator_lid_bulkhead_envelope.stl",
    "output/closed_ecm_coating_incubation_dwell_gelation_window_station_robot_service_keepouts.stl",
    "output/closed_ecm_coating_incubation_dwell_gelation_window_station_assembly.stl",
];

const FEATURE_NAMES: [&str; 12] = [
    "base_leak_tray_deck",
    "sixteen_position_witness_coupon_array",
    "edge_center_humidity_temperature_witness_wells",
    "timed_dwell_token_rail",
    "viscosity_gelation_ladder_coupons",
    "wetness_meniscus_inspection_windows",
    "rinse_drain_residual_capture_trough",
    "camera_illumination_fiducials",
    "barcode_run_custody_lands",
    "release_hold_reject_disposition_gate",
    "closed_isolator_lid_bulkhead_envelope",
    "robot_service_keepouts",
];

const STATION_X: f64 = 1520.0;
const STATION_Y: f64 = 1060.0;
const BASE_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const SOCKET_DEPTH: f64 = 6.0;
const MOUNT_HOLE_D: f64 = 6.8;
const DRAIN_PORT_D: f64 = 16.0;

const ARRAY_ROWS: usize = 4;
const ARRAY_COLS: usize = 4;
const ARRAY_POSITIONS: usize = ARRAY_ROWS * ARRAY_COLS;
const EDGE_POSITIONS: usize = 12;
const CENTER_POSITIONS: usize = ARRAY_POSITIONS - EDGE_POSITIONS;
const CHIP_GAP_X: f64 = 12.0;
const CHIP_GAP_Y: f64 = 10.0;
const ARRAY_PITCH_X: f64 = REVC_CHIP_LENGTH + CHIP_GAP_X;
const ARRAY_PITCH_Y: f64 = REVC_CHIP_WIDTH + CHIP_GAP_Y;
const CHIP_ARRAY_X: f64 =
    ARRAY_COLS as f64 * REVC_CHIP_LENGTH + (ARRAY_COLS as f64 - 1.0) * CHIP_GAP_X;
const CHIP_ARRAY_Y: f64 =
    ARRAY_ROWS as f64 * REVC_CHIP_WIDTH + (ARRAY_ROWS as f64 - 1.0) * CHIP_GAP_Y;

const ARRAY_CENTER: (f64, f64) = (-315.0, 145.0);
const ARRAY_BANK_X: f64 = CHIP_ARRAY_X + 90.0;
const ARRAY_BANK_Y: f64 = CHIP_ARRAY_Y + 82.0;
const ARRAY_BANK_Z: f64 = 36.0;
const COUPON_SLOT_X: f64 = REVC_CHIP_LENGTH * 0.66;
const COUPON_SLOT_Y: f64 = REVC_CHIP_WIDTH * 0.52;
const COUPON_SLOT_DEPTH: f64 = 18.0;
const MENISCUS_INDEX_D: f64 = 11.0;

const ENV_CENTER: (f64, f64) = (385.0, 255.0);
const ENV_BLOCK_X: f64 = 330.0;
const ENV_BLOCK_Y: f64 = 235.0;
const ENV_BLOCK_Z: f64 = 46.0;
const EDGE_HUMIDITY_WELLS: usize = 4;
const EDGE_TEMPERATURE_WELLS: usize = 4;
const CENTER_HUMIDITY_WELLS: usize = 2;
const CENTER_TEMPERATURE_WELLS: usize = 2;
const ENV_WITNESS_WELLS: usize =
    EDGE_HUMIDITY_WELLS + EDGE_TEMPERATURE_WELLS + CENTER_HUMIDITY_WELLS + CENTER_TEMPERATURE_WELLS;
const ENV_WELL_D: f64 = 18.0;
const ENV_WELL_DEPTH: f64 = 30.0;
const ENV_EDGE_PITCH_X: f64 = 52.0;
const ENV_CENTER_PITCH_X: f64 = 62.0;

const TOKEN_CENTER: (f64, f64) = (430.0, -35.0);
const TOKEN_RAIL_X: f64 = 320.0;
const TOKEN_RAIL_Y: f64 = 220.0;
const TOKEN_RAIL_Z: f64 = 30.0;
const DWELL_RAILS: usize = 4;
const TOKENS_PER_RAIL: usize = 8;
const DWELL_TOKEN_SLOTS: usize = DWELL_RAILS * TOKENS_PER_RAIL;
const TOKEN_SLOT_X: f64 = 28.0;
const TOKEN_SLOT_Y: f64 = 18.0;
const TOKEN_RAIL_PITCH_Y: f64 = 43.0;

const LADDER_CENTER: (f64, f64) = (-440.0, -315.0);
const LADDER_X: f64 = 400.0;
const LADDER_Y: f64 = 210.0;
const LADDER_Z: f64 = 34.0;
const VISCOSITY_LADDER_COUPONS: usize = 8;
const GELATION_LADDER_COUPONS: usize = 8;
const LADDER_COUPONS: usize = VISCOSITY_LADDER_COUPONS + GELATION_LADDER_COUPONS;
const LADDER_SLOT_X: f64 = 32.0;
const LADDER_SLOT_Y: f64 = 64.0;
const LADDER_PITCH_X: f64 = 41.0;

const WINDOW_CENTER: (f64, f64) = (-15.0, -315.0);
const WINDOW_FRAME_X: f64 = 360.0;
const WINDOW_FRAME_Y: f64 = 210.0;
const WINDOW_FRAME_Z: f64 = 28.0;
const MENISCUS_WINDOW_ROWS: usize = 4;
const MENISCUS_WINDOW_COLS: usize = 4;
const MENISCUS_WINDOWS: usize = MENISCUS_WINDOW_ROWS * MENISCUS_WINDOW_COLS;
const WINDOW_OPENING_X: f64 = 48.0;
const WINDOW_OPENING_Y: f64 = 28.0;
const WINDOW_PITCH_X: f64 = 70.0;
const WINDOW_PITCH_Y: f64 = 42.0;

const TROUGH_CENTER: (f64, f64) = (385.0, -315.0);
const TROUGH_X: f64 = 330.0;
const TROUGH_Y: f64 = 210.0;
const TROUGH_Z: f64 = 44.0;
const RESIDUAL_CAPTURE_WELLS: usize = ARRAY_POSITIONS;
const DRAIN_TROUGHS: usize = 4;
const RESIDUAL_WELL_D: f64 = 19.0;
const RESIDUAL_WELL_DEPTH: f64 = 30.0;
const TROUGH_LANE_PITCH_Y: f64 = 38.0;

const CUSTODY_CENTER: (f64, f64) = (-95.0, 460.0);
const CUSTODY_X: f64 = 640.0;
const CUSTODY_Y: f64 = 86.0;
const CUSTODY_Z: f64 = 10.0;
const BARCODE_LANDS: usize = 8;
const RUN_CUSTODY_LANDS: usize = 4;
const CUSTODY_POSITION_TABS: usize = ARRAY_POSITIONS;

const DISPOSITION_CENTER: (f64, f64) = (540.0, 460.0);
const DISPOSITION_X: f64 = 310.0;
const DISPOSITION_Y: f64 = 86.0;
const DISPOSITION_Z: f64 = 34.0;
const DISPOSITION_LANES: usize = 3;
const DISPOSITION_SLOTS_PER_LANE: usize = 4;
const DISPOSITION_SLOT_X: f64 = 70.0;
const DISPOSITION_SLOT_Y: f64 = 18.0;
const DISPOSITION_LANE_PITCH_Y: f64 = 25.0;

const EVIDENCE_CENTER: (f64, f64) = (-40.0, 30.0);
const CAMERA_BRIDGE_X: f64 = 1190.0;
const CAMERA_BRIDGE_Y: f64 = 64.0;
const CAMERA_BRIDGE_Z: f64 = 225.0;
const CAMERA_POST_X: f64 = 32.0;
const CAMERA_POST_Y: f64 = 48.0;
const CAMERA_BEAM_Z: f64 = 26.0;
const CAMERA_FIDUCIALS: usize = 6;
const ILLUMINATION_BARS: usize = 4;
const CAMERA_BOSSES: usize = 3;

const LID_CENTER: (f64, f64) = (-40.0, 30.0);
const LID_X: f64 = 1310.0;
const LID_Y: f64 = 840.0;
const LID_Z: f64 = 178.0;
const LID_POST_X: f64 = 26.0;
const LID_POST_Y: f64 = 26.0;
const LID_BEAM_Z: f64 = 18.0;
const HUMIDITY_PURGE_PORTS: usize = 4;
const BULKHEAD_PORTS: usize = 6;
const BULKHEAD_PORT_D: f64 = 22.0;
const BULKHEAD_PORT_PITCH_X: f64 = 52.0;
const LID_INSPECTION_WINDOWS: usize = 4;

const ROBOT_SWEEP_X: f64 = 1260.0;
const ROBOT_SWEEP_Y: f64 = 160.0;
const ROBOT_SWEEP_Z: f64 = 150.0;
const FRONT_ROBOT_APPROACH: f64 = 410.0;
const REAR_BULKHEAD_SERVICE: f64 = 260.0;
const SIDE_SERVICE_CLEARANCE: f64 = 230.0;
const CAMERA_UNDERSIDE_CLEARANCE: f64 = 196.0;
const LID_LIFT_CLEARANCE: f64 = 315.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_on_deck(self) -> bool {
        let usable_x = STATION_X / 2.0 - RIM_W - 8.0;
        let usable_y = STATION_Y / 2.0 - RIM_W - 8.0;
        self.center.0.abs() + self.x / 2.0 <= usable_x
            && self.center.1.abs() + self.y / 2.0 <= usable_y
    }

    fn overlaps(self, other: Rect) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();
        dx < (self.x + other.x) / 2.0 && dy < (self.y + other.y) / 2.0
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_leak_tray_deck();
    export(OUTPUTS[0], &base);

    let array = sixteen_position_witness_coupon_array();
    export(OUTPUTS[1], &array);

    let environment = edge_center_humidity_temperature_witness_wells();
    export(OUTPUTS[2], &environment);

    let tokens = timed_dwell_token_rail();
    export(OUTPUTS[3], &tokens);

    let ladder = viscosity_gelation_ladder_coupons();
    export(OUTPUTS[4], &ladder);

    let windows = wetness_meniscus_inspection_windows();
    export(OUTPUTS[5], &windows);

    let trough = rinse_drain_residual_capture_trough();
    export(OUTPUTS[6], &trough);

    let evidence = camera_illumination_fiducials();
    export(OUTPUTS[7], &evidence);

    let custody = barcode_run_custody_lands();
    export(OUTPUTS[8], &custody);

    let disposition = release_hold_reject_disposition_gate();
    export(OUTPUTS[9], &disposition);

    let lid = closed_isolator_lid_bulkhead_envelope();
    export(OUTPUTS[10], &lid);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[11], &keepouts);

    let assembly = base
        + array
        + environment
        + tokens
        + ladder
        + windows
        + trough
        + evidence
        + custody
        + disposition
        + lid
        + keepouts;
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed ECM coating incubation dwell gelation-window station:");
    println!(
        "  Multi-chip map:              {ARRAY_ROWS} x {ARRAY_COLS} = {ARRAY_POSITIONS} coating witness positions ({EDGE_POSITIONS} edge, {CENTER_POSITIONS} center)"
    );
    println!(
        "  Environmental witnesses:     {ENV_WITNESS_WELLS} wells covering edge/center humidity and temperature before seeding"
    );
    println!(
        "  Timing controls:             {DWELL_RAILS} dwell rails with {DWELL_TOKEN_SLOTS} start/release token slots"
    );
    println!(
        "  Gelation checks:             {VISCOSITY_LADDER_COUPONS} viscosity and {GELATION_LADDER_COUPONS} gelation ladder coupons plus {MENISCUS_WINDOWS} wetness inspection windows"
    );
    println!(
        "  Residual capture:            {DRAIN_TROUGHS} drain lanes and {RESIDUAL_CAPTURE_WELLS} indexed residual wells"
    );
    println!(
        "  Evidence/custody:            {CAMERA_FIDUCIALS} fiducials, {ILLUMINATION_BARS} light bars, {BARCODE_LANDS} barcode lands, {RUN_CUSTODY_LANDS} run custody lands, release/hold/reject disposition lanes"
    );
    println!(
        "  Closed isolator envelope:    {HUMIDITY_PURGE_PORTS} humidity purge ports, {BULKHEAD_PORTS} bulkhead ports, camera clearance {CAMERA_UNDERSIDE_CLEARANCE:.0}mm, lid lift {LID_LIFT_CLEARANCE:.0}mm"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_layout() {
    assert_eq!(OUTPUTS.len(), FEATURE_NAMES.len() + 1);
    assert_eq!(ARRAY_ROWS * ARRAY_COLS, ARRAY_POSITIONS);
    assert_eq!(ARRAY_POSITIONS, 16);
    assert_eq!(EDGE_POSITIONS + CENTER_POSITIONS, ARRAY_POSITIONS);
    assert_eq!(CENTER_POSITIONS, 4);
    assert_eq!(DWELL_RAILS * TOKENS_PER_RAIL, DWELL_TOKEN_SLOTS);
    assert_eq!(DWELL_TOKEN_SLOTS, ARRAY_POSITIONS * 2);
    assert_eq!(RESIDUAL_CAPTURE_WELLS, ARRAY_POSITIONS);
    assert_eq!(MENISCUS_WINDOWS, ARRAY_POSITIONS);
    assert_eq!(LADDER_COUPONS, ARRAY_POSITIONS);
    assert_eq!(
        EDGE_HUMIDITY_WELLS + EDGE_TEMPERATURE_WELLS,
        EDGE_POSITIONS - CENTER_POSITIONS
    );
    assert_eq!(
        CENTER_HUMIDITY_WELLS + CENTER_TEMPERATURE_WELLS,
        CENTER_POSITIONS
    );
    assert!(ARRAY_BANK_Z > REVC_TOTAL_HEIGHT + 14.0);
    assert!(COUPON_SLOT_X < ARRAY_PITCH_X);
    assert!(COUPON_SLOT_Y < ARRAY_PITCH_Y);
    assert!(TOKEN_SLOT_X * (TOKENS_PER_RAIL as f64) < TOKEN_RAIL_X);
    assert!(residual_span_x() + RESIDUAL_WELL_D < TROUGH_X);
    assert!(bulkhead_port_span_x() + BULKHEAD_PORT_D < LID_X / 2.0);
    assert!(camera_bridge_clearance_above_coupon_array() >= 140.0);

    let rects = deck_module_rects();
    for rect in rects {
        assert!(rect.fits_on_deck(), "{} does not fit on deck", rect.name);
    }

    for (i, left) in rects.iter().enumerate() {
        for right in rects.iter().skip(i + 1) {
            assert!(
                !left.overlaps(*right),
                "{} overlaps {}",
                left.name,
                right.name
            );
        }
    }
}

fn deck_module_rects() -> [Rect; 8] {
    [
        Rect {
            name: "sixteen_position_witness_coupon_array",
            center: ARRAY_CENTER,
            x: ARRAY_BANK_X,
            y: ARRAY_BANK_Y,
        },
        Rect {
            name: "edge_center_humidity_temperature_witness_wells",
            center: ENV_CENTER,
            x: ENV_BLOCK_X,
            y: ENV_BLOCK_Y,
        },
        Rect {
            name: "timed_dwell_token_rail",
            center: TOKEN_CENTER,
            x: TOKEN_RAIL_X,
            y: TOKEN_RAIL_Y,
        },
        Rect {
            name: "viscosity_gelation_ladder_coupons",
            center: LADDER_CENTER,
            x: LADDER_X,
            y: LADDER_Y,
        },
        Rect {
            name: "wetness_meniscus_inspection_windows",
            center: WINDOW_CENTER,
            x: WINDOW_FRAME_X,
            y: WINDOW_FRAME_Y,
        },
        Rect {
            name: "rinse_drain_residual_capture_trough",
            center: TROUGH_CENTER,
            x: TROUGH_X,
            y: TROUGH_Y,
        },
        Rect {
            name: "barcode_run_custody_lands",
            center: CUSTODY_CENTER,
            x: CUSTODY_X,
            y: CUSTODY_Y,
        },
        Rect {
            name: "release_hold_reject_disposition_gate",
            center: DISPOSITION_CENTER,
            x: DISPOSITION_X,
            y: DISPOSITION_Y,
        },
    ]
}

fn base_leak_tray_deck() -> Part {
    let deck = centered_cube(
        "ecm_dwell_gelation_base_leak_tray_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let wet_basin = centered_cube(
        "ecm_dwell_gelation_wet_process_basin",
        STATION_X - 130.0,
        STATION_Y - 150.0,
        7.0,
    )
    .translate(0.0, -24.0, BASE_Z - 3.0);
    let front_drain = centered_cylinder(
        "ecm_dwell_gelation_front_leak_tray_drain",
        DRAIN_PORT_D / 2.0,
        52.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 94.0,
        -STATION_Y / 2.0 + 20.0,
        BASE_Z - 7.0,
    );

    deck - wet_basin - front_drain - deck_module_recesses() - deck_mount_holes()
        + perimeter_rims()
        + deck_zone_dividers()
        + deck_datum_targets()
        + humidity_shadow_reference_ribs()
}

fn deck_module_recesses() -> Part {
    let mut recesses = Part::empty("ecm_dwell_gelation_deck_module_recesses");
    for rect in deck_module_rects() {
        recesses = recesses
            + centered_cube(
                format!("ecm_dwell_gelation_{}_socket_recess", rect.name),
                rect.x + 18.0,
                rect.y + 18.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(
                rect.center.0,
                rect.center.1,
                BASE_Z - SOCKET_DEPTH / 2.0 + 0.2,
            );
    }
    recesses
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("ecm_dwell_gelation_deck_mount_holes");
    for (index, (x, y)) in [
        (-STATION_X / 2.0 + 64.0, -STATION_Y / 2.0 + 62.0),
        (STATION_X / 2.0 - 64.0, -STATION_Y / 2.0 + 62.0),
        (-STATION_X / 2.0 + 64.0, STATION_Y / 2.0 - 62.0),
        (STATION_X / 2.0 - 64.0, STATION_Y / 2.0 - 62.0),
        (0.0, -STATION_Y / 2.0 + 62.0),
        (0.0, STATION_Y / 2.0 - 62.0),
        (-STATION_X / 2.0 + 64.0, 0.0),
        (STATION_X / 2.0 - 64.0, 0.0),
    ]
    .into_iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("ecm_dwell_gelation_deck_m6_clearance_{index}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 4.0,
                28,
            )
            .translate(x, y, BASE_Z / 2.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    let rear = centered_cube("ecm_dwell_gelation_rear_leak_rim", STATION_X, RIM_W, RIM_Z)
        .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let left = centered_cube("ecm_dwell_gelation_left_leak_rim", RIM_W, STATION_Y, RIM_Z)
        .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let right = centered_cube("ecm_dwell_gelation_right_leak_rim", RIM_W, STATION_Y, RIM_Z)
        .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let front_left = centered_cube(
        "ecm_dwell_gelation_front_left_low_load_lip",
        530.0,
        RIM_W,
        16.0,
    )
    .translate(
        -STATION_X / 2.0 + 285.0,
        -STATION_Y / 2.0 + RIM_W / 2.0,
        BASE_Z + 8.0,
    );
    let front_right = centered_cube(
        "ecm_dwell_gelation_front_right_low_load_lip",
        530.0,
        RIM_W,
        16.0,
    )
    .translate(
        STATION_X / 2.0 - 285.0,
        -STATION_Y / 2.0 + RIM_W / 2.0,
        BASE_Z + 8.0,
    );
    rear + left + right + front_left + front_right
}

fn deck_zone_dividers() -> Part {
    let array_env_split = centered_cube(
        "ecm_dwell_gelation_array_environment_zone_split",
        8.0,
        440.0,
        22.0,
    )
    .translate(92.0, 155.0, BASE_Z + 11.0);
    let lower_row_split = centered_cube(
        "ecm_dwell_gelation_ladder_window_trough_row_split",
        STATION_X - 250.0,
        8.0,
        22.0,
    )
    .translate(-20.0, -185.0, BASE_Z + 11.0);
    let custody_split = centered_cube(
        "ecm_dwell_gelation_custody_disposition_split",
        8.0,
        CUSTODY_Y,
        20.0,
    )
    .translate(335.0, CUSTODY_CENTER.1, BASE_Z + 10.0);
    array_env_split + lower_row_split + custody_split
}

fn deck_datum_targets() -> Part {
    let mut targets = Part::empty("ecm_dwell_gelation_robot_datum_targets");
    for (index, (x, y)) in [
        (-STATION_X / 2.0 + 84.0, STATION_Y / 2.0 - 84.0),
        (STATION_X / 2.0 - 84.0, STATION_Y / 2.0 - 84.0),
        (-STATION_X / 2.0 + 84.0, -STATION_Y / 2.0 + 84.0),
        (STATION_X / 2.0 - 84.0, -STATION_Y / 2.0 + 84.0),
        (
            ARRAY_CENTER.0 - ARRAY_BANK_X / 2.0 + 38.0,
            ARRAY_CENTER.1 + ARRAY_BANK_Y / 2.0 - 38.0,
        ),
        (
            TROUGH_CENTER.0 + TROUGH_X / 2.0 - 34.0,
            TROUGH_CENTER.1 - TROUGH_Y / 2.0 + 34.0,
        ),
    ]
    .into_iter()
    .enumerate()
    {
        targets = targets
            + fiducial_disc(&format!("ecm_dwell_gelation_deck_datum_{index}")).translate(
                x,
                y,
                BASE_Z + 2.0,
            );
    }
    targets
}

fn humidity_shadow_reference_ribs() -> Part {
    let mut ribs = Part::empty("ecm_dwell_gelation_humidity_shadow_reference_ribs");
    for (index, x) in [-540.0, -360.0, -180.0, 0.0, 180.0, 360.0, 540.0]
        .into_iter()
        .enumerate()
    {
        ribs = ribs
            + centered_cube(
                format!("ecm_dwell_gelation_humidity_shadow_rib_{index}"),
                5.0,
                STATION_Y - 210.0,
                4.0,
            )
            .translate(x, -10.0, BASE_Z + 2.0);
    }
    ribs
}

fn sixteen_position_witness_coupon_array() -> Part {
    let tray = centered_cube(
        "ecm_dwell_gelation_16_position_coupon_tray",
        ARRAY_BANK_X,
        ARRAY_BANK_Y,
        ARRAY_BANK_Z,
    );
    let spill_pan = centered_cube(
        "ecm_dwell_gelation_16_position_spill_pan",
        ARRAY_BANK_X - 42.0,
        ARRAY_BANK_Y - 42.0,
        9.0,
    )
    .translate(0.0, 0.0, ARRAY_BANK_Z / 2.0 - 4.0);
    let part = tray - spill_pan - coupon_slot_cuts()
        + coupon_edge_center_rims()
        + coupon_position_index_dots()
        + array_quadrant_identity_lands()
        + coupon_retainer_tabs();
    place_on_deck(part, ARRAY_CENTER, ARRAY_BANK_Z)
}

fn coupon_slot_cuts() -> Part {
    let mut slots = Part::empty("ecm_dwell_gelation_coupon_slot_cuts");
    for row in 0..ARRAY_ROWS {
        for col in 0..ARRAY_COLS {
            let index = position_index(row, col);
            let (x, y) = array_position_xy(row, col);
            slots = slots
                + centered_cube(
                    format!("ecm_dwell_gelation_coupon_slot_{index}"),
                    COUPON_SLOT_X,
                    COUPON_SLOT_Y,
                    COUPON_SLOT_DEPTH + 0.6,
                )
                .translate(
                    x,
                    y,
                    ARRAY_BANK_Z / 2.0 - COUPON_SLOT_DEPTH / 2.0 + 0.3,
                );
        }
    }
    slots
}

fn coupon_edge_center_rims() -> Part {
    let mut rims = Part::empty("ecm_dwell_gelation_coupon_edge_center_rims");
    for row in 0..ARRAY_ROWS {
        for col in 0..ARRAY_COLS {
            let index = position_index(row, col);
            let (x, y) = array_position_xy(row, col);
            let tag_y = y + COUPON_SLOT_Y / 2.0 + 8.0;
            let rim = centered_cube(
                format!("ecm_dwell_gelation_coupon_slot_rim_{index}"),
                COUPON_SLOT_X + 10.0,
                COUPON_SLOT_Y + 10.0,
                5.0,
            )
            .translate(x, y, ARRAY_BANK_Z / 2.0 + 2.5);
            let open = centered_cube(
                format!("ecm_dwell_gelation_coupon_slot_rim_open_{index}"),
                COUPON_SLOT_X + 2.0,
                COUPON_SLOT_Y + 2.0,
                5.6,
            )
            .translate(x, y, ARRAY_BANK_Z / 2.0 + 2.5);
            let edge_center_tag = centered_cube(
                format!(
                    "ecm_dwell_gelation_{}_coupon_marker_{index}",
                    if is_edge_position(row, col) {
                        "edge"
                    } else {
                        "center"
                    }
                ),
                26.0,
                6.0,
                4.0,
            )
            .translate(x, tag_y, ARRAY_BANK_Z / 2.0 + 4.0);
            rims = rims + (rim - open) + edge_center_tag;
        }
    }
    rims
}

fn coupon_position_index_dots() -> Part {
    let mut dots = Part::empty("ecm_dwell_gelation_coupon_position_index_dots");
    for row in 0..ARRAY_ROWS {
        for col in 0..ARRAY_COLS {
            let index = position_index(row, col);
            let (x, y) = array_position_xy(row, col);
            dots = dots
                + centered_cylinder(
                    format!("ecm_dwell_gelation_position_{index}_meniscus_index_dot"),
                    MENISCUS_INDEX_D / 2.0,
                    3.0,
                    24,
                )
                .translate(
                    x + COUPON_SLOT_X / 2.0 - 11.0,
                    y - COUPON_SLOT_Y / 2.0 + 10.0,
                    ARRAY_BANK_Z / 2.0 + 1.5,
                );
        }
    }
    dots
}

fn array_quadrant_identity_lands() -> Part {
    let mut lands = Part::empty("ecm_dwell_gelation_array_quadrant_identity_lands");
    for (index, (x, y)) in [
        (-ARRAY_BANK_X / 2.0 + 62.0, ARRAY_BANK_Y / 2.0 - 20.0),
        (ARRAY_BANK_X / 2.0 - 62.0, ARRAY_BANK_Y / 2.0 - 20.0),
        (-ARRAY_BANK_X / 2.0 + 62.0, -ARRAY_BANK_Y / 2.0 + 20.0),
        (ARRAY_BANK_X / 2.0 - 62.0, -ARRAY_BANK_Y / 2.0 + 20.0),
    ]
    .into_iter()
    .enumerate()
    {
        lands = lands
            + centered_cube(
                format!("ecm_dwell_gelation_array_quadrant_{index}_barcode_land"),
                92.0,
                18.0,
                4.0,
            )
            .translate(x, y, ARRAY_BANK_Z / 2.0 + 2.0);
    }
    lands
}

fn coupon_retainer_tabs() -> Part {
    let mut tabs = Part::empty("ecm_dwell_gelation_coupon_retainer_tabs");
    for row in 0..ARRAY_ROWS {
        for col in 0..ARRAY_COLS {
            let index = position_index(row, col);
            let (x, y) = array_position_xy(row, col);
            for side in [-1.0, 1.0] {
                tabs = tabs
                    + centered_cube(
                        format!("ecm_dwell_gelation_coupon_retainer_{index}_{side}"),
                        8.0,
                        18.0,
                        10.0,
                    )
                    .translate(
                        x + side * (COUPON_SLOT_X / 2.0 + 7.0),
                        y,
                        ARRAY_BANK_Z / 2.0 + 5.0,
                    );
            }
        }
    }
    tabs
}

fn edge_center_humidity_temperature_witness_wells() -> Part {
    let block = centered_cube(
        "ecm_dwell_gelation_edge_center_environment_witness_block",
        ENV_BLOCK_X,
        ENV_BLOCK_Y,
        ENV_BLOCK_Z,
    );
    let trough = centered_cube(
        "ecm_dwell_gelation_environment_well_spill_trough",
        ENV_BLOCK_X - 42.0,
        ENV_BLOCK_Y - 42.0,
        8.0,
    )
    .translate(0.0, 0.0, ENV_BLOCK_Z / 2.0 - 4.0);
    let part = block - trough - environment_well_cuts()
        + environment_sensor_clip_lands()
        + edge_center_separator_wall()
        + environment_well_rims();
    place_on_deck(part, ENV_CENTER, ENV_BLOCK_Z)
}

fn environment_well_cuts() -> Part {
    let mut cuts = Part::empty("ecm_dwell_gelation_environment_well_cuts");
    for index in 0..ENV_WITNESS_WELLS {
        let (x, y) = environment_well_xy(index);
        cuts = cuts
            + centered_cylinder(
                format!("ecm_dwell_gelation_environment_witness_well_cut_{index}"),
                ENV_WELL_D / 2.0,
                ENV_WELL_DEPTH + 0.6,
                36,
            )
            .translate(x, y, ENV_BLOCK_Z / 2.0 - ENV_WELL_DEPTH / 2.0 + 0.3);
    }
    cuts
}

fn environment_sensor_clip_lands() -> Part {
    let mut clips = Part::empty("ecm_dwell_gelation_environment_sensor_clip_lands");
    for index in 0..ENV_WITNESS_WELLS {
        let (x, y) = environment_well_xy(index);
        clips = clips
            + centered_cube(
                format!("ecm_dwell_gelation_environment_probe_clip_{index}"),
                28.0,
                9.0,
                7.0,
            )
            .translate(x, y - ENV_WELL_D / 2.0 - 12.0, ENV_BLOCK_Z / 2.0 + 3.5);
    }
    clips
}

fn edge_center_separator_wall() -> Part {
    let edge_label = centered_cube(
        "ecm_dwell_gelation_environment_edge_witness_label_land",
        ENV_BLOCK_X - 46.0,
        12.0,
        4.0,
    )
    .translate(0.0, 54.0, ENV_BLOCK_Z / 2.0 + 2.0);
    let center_label = centered_cube(
        "ecm_dwell_gelation_environment_center_witness_label_land",
        ENV_BLOCK_X - 86.0,
        12.0,
        4.0,
    )
    .translate(0.0, -54.0, ENV_BLOCK_Z / 2.0 + 2.0);
    let separator = centered_cube(
        "ecm_dwell_gelation_environment_edge_center_separator",
        ENV_BLOCK_X - 50.0,
        6.0,
        18.0,
    )
    .translate(0.0, 0.0, ENV_BLOCK_Z / 2.0 + 9.0);
    edge_label + center_label + separator
}

fn environment_well_rims() -> Part {
    let mut rims = Part::empty("ecm_dwell_gelation_environment_well_rims");
    for index in 0..ENV_WITNESS_WELLS {
        let (x, y) = environment_well_xy(index);
        let outer = centered_cylinder(
            format!("ecm_dwell_gelation_environment_well_rim_{index}"),
            ENV_WELL_D / 2.0 + 3.0,
            4.0,
            36,
        )
        .translate(x, y, ENV_BLOCK_Z / 2.0 + 2.0);
        let inner = centered_cylinder(
            format!("ecm_dwell_gelation_environment_well_rim_open_{index}"),
            ENV_WELL_D / 2.0 + 0.5,
            4.4,
            36,
        )
        .translate(x, y, ENV_BLOCK_Z / 2.0 + 2.0);
        rims = rims + (outer - inner);
    }
    rims
}

fn timed_dwell_token_rail() -> Part {
    let base = centered_cube(
        "ecm_dwell_gelation_timed_dwell_token_rail_base",
        TOKEN_RAIL_X,
        TOKEN_RAIL_Y,
        TOKEN_RAIL_Z,
    );
    let part = base - dwell_token_slot_cuts()
        + dwell_token_time_lands()
        + dwell_start_release_gates()
        + overdue_token_quarantine_cup();
    place_on_deck(part, TOKEN_CENTER, TOKEN_RAIL_Z)
}

fn dwell_token_slot_cuts() -> Part {
    let mut slots = Part::empty("ecm_dwell_gelation_dwell_token_slot_cuts");
    for rail in 0..DWELL_RAILS {
        let y = token_rail_y(rail);
        for token in 0..TOKENS_PER_RAIL {
            let x = centered_index(token, TOKENS_PER_RAIL, 35.0);
            slots = slots
                + centered_cube(
                    format!("ecm_dwell_gelation_token_slot_r{rail}_t{token}"),
                    TOKEN_SLOT_X,
                    TOKEN_SLOT_Y,
                    TOKEN_RAIL_Z + 2.0,
                )
                .translate(x, y, 2.0);
        }
    }
    slots
}

fn dwell_token_time_lands() -> Part {
    let mut lands = Part::empty("ecm_dwell_gelation_dwell_token_time_lands");
    for rail in 0..DWELL_RAILS {
        lands = lands
            + centered_cube(
                format!("ecm_dwell_gelation_dwell_rail_{rail}_time_label_land"),
                52.0,
                20.0,
                4.0,
            )
            .translate(
                TOKEN_RAIL_X / 2.0 - 35.0,
                token_rail_y(rail),
                TOKEN_RAIL_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn dwell_start_release_gates() -> Part {
    let start = centered_cube(
        "ecm_dwell_gelation_dwell_start_token_gate",
        TOKEN_RAIL_X - 48.0,
        10.0,
        28.0,
    )
    .translate(0.0, TOKEN_RAIL_Y / 2.0 - 26.0, TOKEN_RAIL_Z / 2.0 + 14.0);
    let release = centered_cube(
        "ecm_dwell_gelation_dwell_release_token_gate",
        TOKEN_RAIL_X - 48.0,
        10.0,
        28.0,
    )
    .translate(0.0, -TOKEN_RAIL_Y / 2.0 + 26.0, TOKEN_RAIL_Z / 2.0 + 14.0);
    start + release
}

fn overdue_token_quarantine_cup() -> Part {
    let cup = centered_cube(
        "ecm_dwell_gelation_overdue_token_quarantine_cup_body",
        74.0,
        44.0,
        30.0,
    )
    .translate(-TOKEN_RAIL_X / 2.0 + 47.0, 0.0, TOKEN_RAIL_Z / 2.0 + 15.0);
    let cut = centered_cube(
        "ecm_dwell_gelation_overdue_token_quarantine_cup_opening",
        54.0,
        26.0,
        26.0,
    )
    .translate(-TOKEN_RAIL_X / 2.0 + 47.0, 0.0, TOKEN_RAIL_Z / 2.0 + 19.0);
    cup - cut
}

fn viscosity_gelation_ladder_coupons() -> Part {
    let block = centered_cube(
        "ecm_dwell_gelation_viscosity_gelation_ladder_block",
        LADDER_X,
        LADDER_Y,
        LADDER_Z,
    );
    let basin = centered_cube(
        "ecm_dwell_gelation_ladder_spill_basin",
        LADDER_X - 38.0,
        LADDER_Y - 44.0,
        8.0,
    )
    .translate(0.0, 0.0, LADDER_Z / 2.0 - 4.0);
    let part = block - basin - ladder_coupon_slot_cuts()
        + ladder_step_height_witnesses()
        + ladder_row_labels()
        + ladder_coupon_clamp_tabs();
    place_on_deck(part, LADDER_CENTER, LADDER_Z)
}

fn ladder_coupon_slot_cuts() -> Part {
    let mut slots = Part::empty("ecm_dwell_gelation_ladder_coupon_slot_cuts");
    for index in 0..LADDER_COUPONS {
        let (x, y) = ladder_coupon_xy(index);
        slots = slots
            + centered_cube(
                format!("ecm_dwell_gelation_ladder_coupon_slot_{index}"),
                LADDER_SLOT_X,
                LADDER_SLOT_Y,
                LADDER_Z + 2.0,
            )
            .translate(x, y, 2.0);
    }
    slots
}

fn ladder_step_height_witnesses() -> Part {
    let mut steps = Part::empty("ecm_dwell_gelation_ladder_step_height_witnesses");
    for index in 0..LADDER_COUPONS {
        let (x, y) = ladder_coupon_xy(index);
        let step_height = 4.0 + (index % VISCOSITY_LADDER_COUPONS) as f64 * 1.3;
        steps = steps
            + centered_cube(
                format!("ecm_dwell_gelation_ladder_viscosity_step_{index}"),
                18.0,
                18.0,
                step_height,
            )
            .translate(
                x,
                y + LADDER_SLOT_Y / 2.0 + 15.0,
                LADDER_Z / 2.0 + step_height / 2.0,
            );
    }
    steps
}

fn ladder_row_labels() -> Part {
    let viscosity = centered_cube(
        "ecm_dwell_gelation_viscosity_ladder_label_land",
        LADDER_X - 54.0,
        14.0,
        4.0,
    )
    .translate(0.0, 54.0, LADDER_Z / 2.0 + 2.0);
    let gelation = centered_cube(
        "ecm_dwell_gelation_gelation_ladder_label_land",
        LADDER_X - 54.0,
        14.0,
        4.0,
    )
    .translate(0.0, -54.0, LADDER_Z / 2.0 + 2.0);
    viscosity + gelation
}

fn ladder_coupon_clamp_tabs() -> Part {
    let mut tabs = Part::empty("ecm_dwell_gelation_ladder_coupon_clamp_tabs");
    for index in 0..LADDER_COUPONS {
        let (x, y) = ladder_coupon_xy(index);
        tabs = tabs
            + centered_cube(
                format!("ecm_dwell_gelation_ladder_coupon_clamp_{index}"),
                LADDER_SLOT_X + 8.0,
                6.0,
                8.0,
            )
            .translate(x, y - LADDER_SLOT_Y / 2.0 - 6.0, LADDER_Z / 2.0 + 4.0);
    }
    tabs
}

fn wetness_meniscus_inspection_windows() -> Part {
    let frame = centered_cube(
        "ecm_dwell_gelation_wetness_meniscus_window_frame",
        WINDOW_FRAME_X,
        WINDOW_FRAME_Y,
        WINDOW_FRAME_Z,
    );
    let part = frame - meniscus_window_cuts()
        + meniscus_window_index_lands()
        + window_backlight_baffles()
        + meniscus_reference_sill();
    place_on_deck(part, WINDOW_CENTER, WINDOW_FRAME_Z)
}

fn meniscus_window_cuts() -> Part {
    let mut cuts = Part::empty("ecm_dwell_gelation_meniscus_window_cuts");
    for row in 0..MENISCUS_WINDOW_ROWS {
        for col in 0..MENISCUS_WINDOW_COLS {
            let index = position_index(row, col);
            let (x, y) = window_xy(row, col);
            cuts = cuts
                + centered_cube(
                    format!("ecm_dwell_gelation_meniscus_window_opening_{index}"),
                    WINDOW_OPENING_X,
                    WINDOW_OPENING_Y,
                    WINDOW_FRAME_Z + 2.0,
                )
                .translate(x, y, 0.0);
        }
    }
    cuts
}

fn meniscus_window_index_lands() -> Part {
    let mut lands = Part::empty("ecm_dwell_gelation_meniscus_window_index_lands");
    for row in 0..MENISCUS_WINDOW_ROWS {
        for col in 0..MENISCUS_WINDOW_COLS {
            let index = position_index(row, col);
            let (x, y) = window_xy(row, col);
            lands = lands
                + centered_cube(
                    format!("ecm_dwell_gelation_meniscus_window_index_land_{index}"),
                    20.0,
                    5.0,
                    4.0,
                )
                .translate(
                    x,
                    y + WINDOW_OPENING_Y / 2.0 + 7.0,
                    WINDOW_FRAME_Z / 2.0 + 2.0,
                );
        }
    }
    lands
}

fn window_backlight_baffles() -> Part {
    let mut baffles = Part::empty("ecm_dwell_gelation_window_backlight_baffles");
    for col in 0..=MENISCUS_WINDOW_COLS {
        baffles = baffles
            + centered_cube(
                format!("ecm_dwell_gelation_window_backlight_baffle_{col}"),
                4.0,
                WINDOW_FRAME_Y - 34.0,
                18.0,
            )
            .translate(
                centered_index(col, MENISCUS_WINDOW_COLS + 1, WINDOW_PITCH_X),
                0.0,
                WINDOW_FRAME_Z / 2.0 + 9.0,
            );
    }
    baffles
}

fn meniscus_reference_sill() -> Part {
    centered_cube(
        "ecm_dwell_gelation_meniscus_reference_sill",
        WINDOW_FRAME_X - 44.0,
        10.0,
        12.0,
    )
    .translate(
        0.0,
        -WINDOW_FRAME_Y / 2.0 + 22.0,
        WINDOW_FRAME_Z / 2.0 + 6.0,
    )
}

fn rinse_drain_residual_capture_trough() -> Part {
    let block = centered_cube(
        "ecm_dwell_gelation_residual_capture_trough_block",
        TROUGH_X,
        TROUGH_Y,
        TROUGH_Z,
    );
    let part = block - drain_lane_cuts() - residual_well_cuts() - residual_front_drain_cut()
        + residual_well_rims()
        + trough_lane_labels()
        + residual_sample_pull_tabs();
    place_on_deck(part, TROUGH_CENTER, TROUGH_Z)
}

fn drain_lane_cuts() -> Part {
    let mut lanes = Part::empty("ecm_dwell_gelation_drain_lane_cuts");
    for lane in 0..DRAIN_TROUGHS {
        lanes = lanes
            + centered_cube(
                format!("ecm_dwell_gelation_drain_lane_cut_{lane}"),
                TROUGH_X - 56.0,
                16.0,
                20.0,
            )
            .translate(0.0, drain_lane_y(lane), TROUGH_Z / 2.0 - 8.0);
    }
    lanes
}

fn residual_well_cuts() -> Part {
    let mut wells = Part::empty("ecm_dwell_gelation_residual_well_cuts");
    for index in 0..RESIDUAL_CAPTURE_WELLS {
        let lane = index / ARRAY_COLS;
        let col = index % ARRAY_COLS;
        let x = centered_index(col, ARRAY_COLS, 64.0);
        let y = drain_lane_y(lane);
        wells = wells
            + centered_cylinder(
                format!("ecm_dwell_gelation_residual_capture_well_{index}"),
                RESIDUAL_WELL_D / 2.0,
                RESIDUAL_WELL_DEPTH + 0.6,
                32,
            )
            .translate(x, y, TROUGH_Z / 2.0 - RESIDUAL_WELL_DEPTH / 2.0 + 0.3);
    }
    wells
}

fn residual_front_drain_cut() -> Part {
    centered_cylinder(
        "ecm_dwell_gelation_residual_trough_front_drain_cut",
        8.0,
        44.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(TROUGH_X / 2.0 - 38.0, -TROUGH_Y / 2.0 + 9.0, TROUGH_Z / 2.0)
}

fn residual_well_rims() -> Part {
    let mut rims = Part::empty("ecm_dwell_gelation_residual_well_rims");
    for index in 0..RESIDUAL_CAPTURE_WELLS {
        let lane = index / ARRAY_COLS;
        let col = index % ARRAY_COLS;
        let x = centered_index(col, ARRAY_COLS, 64.0);
        let y = drain_lane_y(lane);
        let outer = centered_cylinder(
            format!("ecm_dwell_gelation_residual_well_rim_{index}"),
            RESIDUAL_WELL_D / 2.0 + 3.0,
            4.0,
            32,
        )
        .translate(x, y, TROUGH_Z / 2.0 + 2.0);
        let inner = centered_cylinder(
            format!("ecm_dwell_gelation_residual_well_rim_open_{index}"),
            RESIDUAL_WELL_D / 2.0 + 0.5,
            4.4,
            32,
        )
        .translate(x, y, TROUGH_Z / 2.0 + 2.0);
        rims = rims + (outer - inner);
    }
    rims
}

fn trough_lane_labels() -> Part {
    let mut labels = Part::empty("ecm_dwell_gelation_trough_lane_labels");
    for lane in 0..DRAIN_TROUGHS {
        labels = labels
            + centered_cube(
                format!("ecm_dwell_gelation_trough_lane_{lane}_label_land"),
                42.0,
                14.0,
                4.0,
            )
            .translate(
                -TROUGH_X / 2.0 + 34.0,
                drain_lane_y(lane),
                TROUGH_Z / 2.0 + 2.0,
            );
    }
    labels
}

fn residual_sample_pull_tabs() -> Part {
    let rear = centered_cube(
        "ecm_dwell_gelation_residual_capture_rear_pull_tab",
        TROUGH_X - 70.0,
        9.0,
        18.0,
    )
    .translate(0.0, TROUGH_Y / 2.0 - 20.0, TROUGH_Z / 2.0 + 9.0);
    let front = centered_cube(
        "ecm_dwell_gelation_residual_capture_front_pull_tab",
        TROUGH_X - 70.0,
        9.0,
        18.0,
    )
    .translate(0.0, -TROUGH_Y / 2.0 + 20.0, TROUGH_Z / 2.0 + 9.0);
    rear + front
}

fn camera_illumination_fiducials() -> Part {
    let left_post = centered_cube(
        "ecm_dwell_gelation_camera_bridge_left_post",
        CAMERA_POST_X,
        CAMERA_POST_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(
        EVIDENCE_CENTER.0 - CAMERA_BRIDGE_X / 2.0,
        EVIDENCE_CENTER.1,
        BASE_Z + CAMERA_BRIDGE_Z / 2.0,
    );
    let right_post = centered_cube(
        "ecm_dwell_gelation_camera_bridge_right_post",
        CAMERA_POST_X,
        CAMERA_POST_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(
        EVIDENCE_CENTER.0 + CAMERA_BRIDGE_X / 2.0,
        EVIDENCE_CENTER.1,
        BASE_Z + CAMERA_BRIDGE_Z / 2.0,
    );
    let beam = centered_cube(
        "ecm_dwell_gelation_camera_bridge_overhead_beam",
        CAMERA_BRIDGE_X,
        CAMERA_BRIDGE_Y,
        CAMERA_BEAM_Z,
    )
    .translate(
        EVIDENCE_CENTER.0,
        EVIDENCE_CENTER.1,
        BASE_Z + CAMERA_BRIDGE_Z - CAMERA_BEAM_Z / 2.0,
    );

    left_post + right_post + beam + camera_bosses() + illumination_bars() + evidence_fiducials()
}

fn camera_bosses() -> Part {
    let mut bosses = Part::empty("ecm_dwell_gelation_camera_bosses");
    for index in 0..CAMERA_BOSSES {
        let x = EVIDENCE_CENTER.0 + centered_index(index, CAMERA_BOSSES, 235.0);
        bosses = bosses
            + centered_cylinder(
                format!("ecm_dwell_gelation_camera_lens_boss_{index}"),
                22.0,
                12.0,
                44,
            )
            .translate(
                x,
                EVIDENCE_CENTER.1,
                BASE_Z + CAMERA_BRIDGE_Z - CAMERA_BEAM_Z - 6.0,
            )
            - centered_cylinder(
                format!("ecm_dwell_gelation_camera_lens_opening_{index}"),
                10.0,
                13.0,
                32,
            )
            .translate(
                x,
                EVIDENCE_CENTER.1,
                BASE_Z + CAMERA_BRIDGE_Z - CAMERA_BEAM_Z - 6.0,
            );
    }
    bosses
}

fn illumination_bars() -> Part {
    let mut bars = Part::empty("ecm_dwell_gelation_illumination_bars");
    for index in 0..ILLUMINATION_BARS {
        let x = EVIDENCE_CENTER.0 + centered_index(index, ILLUMINATION_BARS, 210.0);
        bars = bars
            + centered_cube(
                format!("ecm_dwell_gelation_low_angle_light_bar_{index}"),
                150.0,
                14.0,
                10.0,
            )
            .translate(
                x,
                EVIDENCE_CENTER.1 + CAMERA_BRIDGE_Y / 2.0 + 18.0,
                BASE_Z + CAMERA_BRIDGE_Z - CAMERA_BEAM_Z - 14.0,
            );
    }
    bars
}

fn evidence_fiducials() -> Part {
    let mut fiducials = Part::empty("ecm_dwell_gelation_evidence_fiducials");
    for (index, x) in [-520.0, -310.0, -100.0, 110.0, 320.0, 530.0]
        .into_iter()
        .enumerate()
    {
        fiducials = fiducials
            + fiducial_disc(&format!("ecm_dwell_gelation_camera_fiducial_{index}")).translate(
                EVIDENCE_CENTER.0 + x,
                EVIDENCE_CENTER.1 - CAMERA_BRIDGE_Y / 2.0 - 18.0,
                BASE_Z + CAMERA_BRIDGE_Z - CAMERA_BEAM_Z - 5.0,
            );
    }
    fiducials
}

fn barcode_run_custody_lands() -> Part {
    let panel = centered_cube(
        "ecm_dwell_gelation_barcode_run_custody_panel",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    let part = panel + barcode_lands() + run_custody_lands() + position_custody_tabs();
    place_on_deck(part, CUSTODY_CENTER, CUSTODY_Z)
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("ecm_dwell_gelation_barcode_lands");
    for index in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("ecm_dwell_gelation_barcode_land_{index}"),
                60.0,
                18.0,
                4.0,
            )
            .translate(
                -CUSTODY_X / 2.0 + 45.0 + index as f64 * 72.0,
                CUSTODY_Y / 2.0 - 23.0,
                CUSTODY_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn run_custody_lands() -> Part {
    let mut lands = Part::empty("ecm_dwell_gelation_run_custody_lands");
    for index in 0..RUN_CUSTODY_LANDS {
        lands = lands
            + centered_cube(
                format!("ecm_dwell_gelation_run_custody_land_{index}"),
                118.0,
                18.0,
                4.0,
            )
            .translate(
                -CUSTODY_X / 2.0 + 78.0 + index as f64 * 148.0,
                -CUSTODY_Y / 2.0 + 23.0,
                CUSTODY_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn position_custody_tabs() -> Part {
    let mut tabs = Part::empty("ecm_dwell_gelation_position_custody_tabs");
    for index in 0..CUSTODY_POSITION_TABS {
        tabs = tabs
            + centered_cube(
                format!("ecm_dwell_gelation_position_custody_tab_{index}"),
                22.0,
                12.0,
                6.0,
            )
            .translate(
                centered_index(index, CUSTODY_POSITION_TABS, 34.0),
                0.0,
                CUSTODY_Z / 2.0 + 3.0,
            );
    }
    tabs
}

fn release_hold_reject_disposition_gate() -> Part {
    let panel = centered_cube(
        "ecm_dwell_gelation_release_hold_reject_panel",
        DISPOSITION_X,
        DISPOSITION_Y,
        DISPOSITION_Z,
    );
    let part = panel - disposition_slot_cuts()
        + disposition_lane_labels()
        + disposition_gate_stops()
        + disposition_status_flags();
    place_on_deck(part, DISPOSITION_CENTER, DISPOSITION_Z)
}

fn disposition_slot_cuts() -> Part {
    let mut cuts = Part::empty("ecm_dwell_gelation_disposition_slot_cuts");
    for lane in 0..DISPOSITION_LANES {
        for slot in 0..DISPOSITION_SLOTS_PER_LANE {
            cuts = cuts
                + centered_cube(
                    format!(
                        "ecm_dwell_gelation_disposition_{}_slot_{slot}",
                        lane_name(lane)
                    ),
                    DISPOSITION_SLOT_X,
                    DISPOSITION_SLOT_Y,
                    DISPOSITION_Z + 2.0,
                )
                .translate(
                    centered_index(slot, DISPOSITION_SLOTS_PER_LANE, 76.0),
                    disposition_lane_y(lane),
                    2.0,
                );
        }
    }
    cuts
}

fn disposition_lane_labels() -> Part {
    let mut labels = Part::empty("ecm_dwell_gelation_disposition_lane_labels");
    for lane in 0..DISPOSITION_LANES {
        labels = labels
            + centered_cube(
                format!("ecm_dwell_gelation_{}_lane_label_land", lane_name(lane)),
                52.0,
                8.0,
                4.0,
            )
            .translate(
                -DISPOSITION_X / 2.0 + 32.0,
                disposition_lane_y(lane),
                DISPOSITION_Z / 2.0 + 2.0,
            );
    }
    labels
}

fn disposition_gate_stops() -> Part {
    let release = centered_cube(
        "ecm_dwell_gelation_release_gate_stop",
        12.0,
        DISPOSITION_Y - 18.0,
        36.0,
    )
    .translate(-DISPOSITION_X / 2.0 + 84.0, 0.0, DISPOSITION_Z / 2.0 + 18.0);
    let hold = centered_cube(
        "ecm_dwell_gelation_hold_gate_stop",
        12.0,
        DISPOSITION_Y - 18.0,
        36.0,
    )
    .translate(0.0, 0.0, DISPOSITION_Z / 2.0 + 18.0);
    let reject = centered_cube(
        "ecm_dwell_gelation_reject_gate_stop",
        12.0,
        DISPOSITION_Y - 18.0,
        36.0,
    )
    .translate(DISPOSITION_X / 2.0 - 84.0, 0.0, DISPOSITION_Z / 2.0 + 18.0);
    release + hold + reject
}

fn disposition_status_flags() -> Part {
    let mut flags = Part::empty("ecm_dwell_gelation_disposition_status_flags");
    for lane in 0..DISPOSITION_LANES {
        flags = flags
            + centered_cube(
                format!("ecm_dwell_gelation_{}_status_flag", lane_name(lane)),
                24.0,
                12.0,
                42.0,
            )
            .translate(
                DISPOSITION_X / 2.0 - 28.0,
                disposition_lane_y(lane),
                DISPOSITION_Z / 2.0 + 21.0,
            );
    }
    flags
}

fn closed_isolator_lid_bulkhead_envelope() -> Part {
    lid_posts()
        + lid_top_rails()
        + lid_inspection_window_frames()
        + humidity_purge_ports()
        + closed_bulkhead_ports()
        + gasket_compression_lands()
}

fn lid_posts() -> Part {
    let mut posts = Part::empty("ecm_dwell_gelation_closed_lid_posts");
    for (index, (x, y)) in [
        (-LID_X / 2.0, -LID_Y / 2.0),
        (LID_X / 2.0, -LID_Y / 2.0),
        (-LID_X / 2.0, LID_Y / 2.0),
        (LID_X / 2.0, LID_Y / 2.0),
    ]
    .into_iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(
                format!("ecm_dwell_gelation_closed_lid_corner_post_{index}"),
                LID_POST_X,
                LID_POST_Y,
                LID_Z,
            )
            .translate(LID_CENTER.0 + x, LID_CENTER.1 + y, BASE_Z + LID_Z / 2.0);
    }
    posts
}

fn lid_top_rails() -> Part {
    let rear = centered_cube(
        "ecm_dwell_gelation_closed_lid_rear_top_rail",
        LID_X,
        LID_POST_Y,
        LID_BEAM_Z,
    )
    .translate(
        LID_CENTER.0,
        LID_CENTER.1 + LID_Y / 2.0,
        BASE_Z + LID_Z - LID_BEAM_Z / 2.0,
    );
    let front = centered_cube(
        "ecm_dwell_gelation_closed_lid_front_top_rail",
        LID_X,
        LID_POST_Y,
        LID_BEAM_Z,
    )
    .translate(
        LID_CENTER.0,
        LID_CENTER.1 - LID_Y / 2.0,
        BASE_Z + LID_Z - LID_BEAM_Z / 2.0,
    );
    let left = centered_cube(
        "ecm_dwell_gelation_closed_lid_left_top_rail",
        LID_POST_X,
        LID_Y,
        LID_BEAM_Z,
    )
    .translate(
        LID_CENTER.0 - LID_X / 2.0,
        LID_CENTER.1,
        BASE_Z + LID_Z - LID_BEAM_Z / 2.0,
    );
    let right = centered_cube(
        "ecm_dwell_gelation_closed_lid_right_top_rail",
        LID_POST_X,
        LID_Y,
        LID_BEAM_Z,
    )
    .translate(
        LID_CENTER.0 + LID_X / 2.0,
        LID_CENTER.1,
        BASE_Z + LID_Z - LID_BEAM_Z / 2.0,
    );
    rear + front + left + right
}

fn lid_inspection_window_frames() -> Part {
    let mut frames = Part::empty("ecm_dwell_gelation_lid_inspection_window_frames");
    for index in 0..LID_INSPECTION_WINDOWS {
        let x = LID_CENTER.0 + centered_index(index, LID_INSPECTION_WINDOWS, 260.0);
        let frame = centered_cube(
            format!("ecm_dwell_gelation_lid_inspection_window_frame_{index}"),
            180.0,
            84.0,
            8.0,
        )
        .translate(x, LID_CENTER.1 + 120.0, BASE_Z + LID_Z - 42.0);
        let open = centered_cube(
            format!("ecm_dwell_gelation_lid_inspection_window_open_{index}"),
            144.0,
            52.0,
            8.6,
        )
        .translate(x, LID_CENTER.1 + 120.0, BASE_Z + LID_Z - 42.0);
        frames = frames + (frame - open);
    }
    frames
}

fn humidity_purge_ports() -> Part {
    let mut ports = Part::empty("ecm_dwell_gelation_humidity_purge_ports");
    for index in 0..HUMIDITY_PURGE_PORTS {
        let x = LID_CENTER.0 + centered_index(index, HUMIDITY_PURGE_PORTS, 160.0);
        let collar = centered_cylinder(
            format!("ecm_dwell_gelation_humidity_purge_port_collar_{index}"),
            18.0,
            12.0,
            36,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, LID_CENTER.1 + LID_Y / 2.0 + 10.0, BASE_Z + LID_Z - 74.0);
        let bore = centered_cylinder(
            format!("ecm_dwell_gelation_humidity_purge_port_bore_{index}"),
            8.0,
            13.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, LID_CENTER.1 + LID_Y / 2.0 + 10.0, BASE_Z + LID_Z - 74.0);
        ports = ports + (collar - bore);
    }
    ports
}

fn closed_bulkhead_ports() -> Part {
    let bulkhead = centered_cube(
        "ecm_dwell_gelation_closed_reagent_residual_bulkhead",
        420.0,
        24.0,
        98.0,
    )
    .translate(
        LID_CENTER.0 + 280.0,
        LID_CENTER.1 + LID_Y / 2.0 + 12.0,
        BASE_Z + 72.0,
    );
    let mut collars = Part::empty("ecm_dwell_gelation_bulkhead_port_collars");
    for index in 0..BULKHEAD_PORTS {
        let x = LID_CENTER.0 + 280.0 + centered_index(index, BULKHEAD_PORTS, BULKHEAD_PORT_PITCH_X);
        let collar = centered_cylinder(
            format!("ecm_dwell_gelation_bulkhead_port_collar_{index}"),
            BULKHEAD_PORT_D / 2.0 + 8.0,
            16.0,
            36,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, LID_CENTER.1 + LID_Y / 2.0 + 25.0, BASE_Z + 72.0);
        let bore = centered_cylinder(
            format!("ecm_dwell_gelation_bulkhead_port_bore_{index}"),
            BULKHEAD_PORT_D / 2.0,
            18.0,
            36,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, LID_CENTER.1 + LID_Y / 2.0 + 25.0, BASE_Z + 72.0);
        collars = collars + (collar - bore);
    }
    bulkhead + collars
}

fn gasket_compression_lands() -> Part {
    let front = centered_cube(
        "ecm_dwell_gelation_closed_lid_front_gasket_land",
        LID_X - 80.0,
        10.0,
        5.0,
    )
    .translate(
        LID_CENTER.0,
        LID_CENTER.1 - LID_Y / 2.0 + 26.0,
        BASE_Z + 2.5,
    );
    let rear = centered_cube(
        "ecm_dwell_gelation_closed_lid_rear_gasket_land",
        LID_X - 80.0,
        10.0,
        5.0,
    )
    .translate(
        LID_CENTER.0,
        LID_CENTER.1 + LID_Y / 2.0 - 26.0,
        BASE_Z + 2.5,
    );
    let left = centered_cube(
        "ecm_dwell_gelation_closed_lid_left_gasket_land",
        10.0,
        LID_Y - 80.0,
        5.0,
    )
    .translate(
        LID_CENTER.0 - LID_X / 2.0 + 26.0,
        LID_CENTER.1,
        BASE_Z + 2.5,
    );
    let right = centered_cube(
        "ecm_dwell_gelation_closed_lid_right_gasket_land",
        10.0,
        LID_Y - 80.0,
        5.0,
    )
    .translate(
        LID_CENTER.0 + LID_X / 2.0 - 26.0,
        LID_CENTER.1,
        BASE_Z + 2.5,
    );
    front + rear + left + right
}

fn robot_service_keepouts() -> Part {
    let robot = centered_cube(
        "ecm_dwell_gelation_robot_wrist_front_sweep_keepout",
        ROBOT_SWEEP_X,
        ROBOT_SWEEP_Y,
        ROBOT_SWEEP_Z,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 + FRONT_ROBOT_APPROACH,
        BASE_Z + ROBOT_SWEEP_Z / 2.0,
    );
    let rear = centered_cube(
        "ecm_dwell_gelation_rear_bulkhead_service_keepout",
        STATION_X - 210.0,
        8.0,
        96.0,
    )
    .translate(0.0, STATION_Y / 2.0 - REAR_BULKHEAD_SERVICE, BASE_Z + 48.0);
    let side = centered_cube(
        "ecm_dwell_gelation_side_service_keepout",
        8.0,
        STATION_Y - 280.0,
        92.0,
    )
    .translate(
        STATION_X / 2.0 - SIDE_SERVICE_CLEARANCE,
        -10.0,
        BASE_Z + 46.0,
    );
    let camera = centered_cube(
        "ecm_dwell_gelation_camera_underside_clearance_gauge",
        CAMERA_BRIDGE_X - 120.0,
        10.0,
        8.0,
    )
    .translate(
        EVIDENCE_CENTER.0,
        EVIDENCE_CENTER.1,
        BASE_Z + CAMERA_UNDERSIDE_CLEARANCE,
    );
    let lid_lift = centered_cube(
        "ecm_dwell_gelation_lid_lift_clearance_gauge",
        LID_X,
        LID_Y,
        8.0,
    )
    .translate(LID_CENTER.0, LID_CENTER.1, BASE_Z + LID_LIFT_CLEARANCE);
    robot + rear + side + camera + lid_lift
}

fn place_on_deck(part: Part, center: (f64, f64), z: f64) -> Part {
    part.translate(center.0, center.1, BASE_Z + z / 2.0)
}

fn fiducial_disc(name: &str) -> Part {
    let puck = centered_cylinder(format!("{name}_puck"), 12.0, 4.0, 40);
    let center = centered_cylinder(format!("{name}_center_bore"), 2.2, 4.6, 24);
    let cross_x = centered_cube(format!("{name}_cross_x"), 20.0, 2.2, 4.6);
    let cross_y = centered_cube(format!("{name}_cross_y"), 2.2, 20.0, 4.6);
    puck - center - cross_x - cross_y
}

fn array_position_xy(row: usize, col: usize) -> (f64, f64) {
    (
        centered_index(col, ARRAY_COLS, ARRAY_PITCH_X),
        centered_index_rev(row, ARRAY_ROWS, ARRAY_PITCH_Y),
    )
}

fn position_index(row: usize, col: usize) -> usize {
    row * ARRAY_COLS + col
}

fn is_edge_position(row: usize, col: usize) -> bool {
    row == 0 || row == ARRAY_ROWS - 1 || col == 0 || col == ARRAY_COLS - 1
}

fn environment_well_xy(index: usize) -> (f64, f64) {
    match index {
        0..=3 => (
            centered_index(index, EDGE_HUMIDITY_WELLS, ENV_EDGE_PITCH_X),
            72.0,
        ),
        4..=7 => (
            centered_index(index - 4, EDGE_TEMPERATURE_WELLS, ENV_EDGE_PITCH_X),
            28.0,
        ),
        8..=9 => (
            centered_index(index - 8, CENTER_HUMIDITY_WELLS, ENV_CENTER_PITCH_X),
            -36.0,
        ),
        _ => (
            centered_index(index - 10, CENTER_TEMPERATURE_WELLS, ENV_CENTER_PITCH_X),
            -80.0,
        ),
    }
}

fn token_rail_y(rail: usize) -> f64 {
    centered_index_rev(rail, DWELL_RAILS, TOKEN_RAIL_PITCH_Y)
}

fn ladder_coupon_xy(index: usize) -> (f64, f64) {
    let row = index / VISCOSITY_LADDER_COUPONS;
    let col = index % VISCOSITY_LADDER_COUPONS;
    (
        centered_index(col, VISCOSITY_LADDER_COUPONS, LADDER_PITCH_X),
        if row == 0 { 44.0 } else { -44.0 },
    )
}

fn window_xy(row: usize, col: usize) -> (f64, f64) {
    (
        centered_index(col, MENISCUS_WINDOW_COLS, WINDOW_PITCH_X),
        centered_index_rev(row, MENISCUS_WINDOW_ROWS, WINDOW_PITCH_Y),
    )
}

fn drain_lane_y(lane: usize) -> f64 {
    centered_index_rev(lane, DRAIN_TROUGHS, TROUGH_LANE_PITCH_Y)
}

fn residual_span_x() -> f64 {
    64.0 * (ARRAY_COLS as f64 - 1.0)
}

fn bulkhead_port_span_x() -> f64 {
    BULKHEAD_PORT_PITCH_X * (BULKHEAD_PORTS as f64 - 1.0)
}

fn disposition_lane_y(lane: usize) -> f64 {
    centered_index_rev(lane, DISPOSITION_LANES, DISPOSITION_LANE_PITCH_Y)
}

fn lane_name(lane: usize) -> &'static str {
    match lane {
        0 => "release",
        1 => "hold",
        _ => "reject",
    }
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn centered_index_rev(index: usize, count: usize, pitch: f64) -> f64 {
    ((count as f64 - 1.0) / 2.0 - index as f64) * pitch
}

fn camera_bridge_clearance_above_coupon_array() -> f64 {
    CAMERA_UNDERSIDE_CLEARANCE - ARRAY_BANK_Z - REVC_TOTAL_HEIGHT
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), FEATURE_NAMES.len() + 1);
        for path in OUTPUTS {
            assert!(path.starts_with(
                "output/closed_ecm_coating_incubation_dwell_gelation_window_station_"
            ));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn sixteen_position_array_has_edge_center_coverage() {
        assert_eq!(ARRAY_ROWS * ARRAY_COLS, ARRAY_POSITIONS);
        assert_eq!(ARRAY_POSITIONS, 16);

        let mut edge = 0;
        let mut center = 0;
        for row in 0..ARRAY_ROWS {
            for col in 0..ARRAY_COLS {
                if is_edge_position(row, col) {
                    edge += 1;
                } else {
                    center += 1;
                }
            }
        }

        assert_eq!(edge, EDGE_POSITIONS);
        assert_eq!(center, CENTER_POSITIONS);
        assert_eq!(CENTER_POSITIONS, 4);
    }

    #[test]
    fn dwell_tokens_cover_start_and_release_for_all_positions() {
        assert_eq!(DWELL_RAILS * TOKENS_PER_RAIL, DWELL_TOKEN_SLOTS);
        assert_eq!(DWELL_TOKEN_SLOTS, ARRAY_POSITIONS * 2);
        assert!(TOKEN_SLOT_X * (TOKENS_PER_RAIL as f64) < TOKEN_RAIL_X);
        assert!(TOKEN_RAIL_PITCH_Y * (DWELL_RAILS as f64) < TOKEN_RAIL_Y);
    }

    #[test]
    fn witness_counts_match_validation_scope() {
        assert_eq!(ENV_WITNESS_WELLS, 12);
        assert_eq!(EDGE_HUMIDITY_WELLS, EDGE_TEMPERATURE_WELLS);
        assert_eq!(CENTER_HUMIDITY_WELLS, CENTER_TEMPERATURE_WELLS);
        assert_eq!(MENISCUS_WINDOWS, ARRAY_POSITIONS);
        assert_eq!(RESIDUAL_CAPTURE_WELLS, ARRAY_POSITIONS);
        assert_eq!(LADDER_COUPONS, ARRAY_POSITIONS);
    }

    #[test]
    fn disposition_and_custody_features_are_explicit() {
        assert_eq!(BARCODE_LANDS, 8);
        assert_eq!(RUN_CUSTODY_LANDS, 4);
        assert_eq!(CUSTODY_POSITION_TABS, ARRAY_POSITIONS);
        assert_eq!(DISPOSITION_LANES, 3);
        assert_eq!(lane_name(0), "release");
        assert_eq!(lane_name(1), "hold");
        assert_eq!(lane_name(2), "reject");
        assert_eq!(DISPOSITION_LANES * DISPOSITION_SLOTS_PER_LANE, 12);
    }

    #[test]
    fn station_layout_and_closed_isolator_clearances_are_valid() {
        assert_layout();
        assert_eq!(HUMIDITY_PURGE_PORTS, 4);
        assert_eq!(BULKHEAD_PORTS, 6);
        assert_eq!(CAMERA_FIDUCIALS, 6);
        assert_eq!(ILLUMINATION_BARS, 4);
        assert_eq!(CAMERA_BOSSES, 3);
        assert!(FRONT_ROBOT_APPROACH >= 400.0);
        assert!(REAR_BULKHEAD_SERVICE >= 250.0);
        assert!(SIDE_SERVICE_CLEARANCE >= 220.0);
        assert!(camera_bridge_clearance_above_coupon_array() >= 140.0);
        assert!(LID_LIFT_CLEARANCE > LID_Z + BASE_Z);
    }
}
