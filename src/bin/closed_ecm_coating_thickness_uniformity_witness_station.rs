use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed ECM/coating thickness uniformity witness station.
//
// Intent:
// - Validate coating film thickness uniformity across a closed 20-position
//   tissue-culture cassette workflow without introducing live cells.
// - Make the coating reservoir surrogate, controlled dispense/aspirate
//   manifolds, witness coupon matrix, UV/fluorescence imaging windows,
//   edge-bead/dewetting traps, timing token rails, contamination witnesses,
//   release/hold/reject gates, and robot/service keepouts mechanically explicit.
// - This is fixture/interface CAD only. ECM chemistry, UV exposure dose,
//   fluorescence acceptance thresholds, and biological release criteria remain
//   process controls outside this geometry.

const OUTPUTS: [&str; 12] = [
    "output/closed_ecm_coating_thickness_uniformity_witness_station_leak_tray_deck.stl",
    "output/closed_ecm_coating_thickness_uniformity_witness_station_multi_chip_cassette_datum_nest.stl",
    "output/closed_ecm_coating_thickness_uniformity_witness_station_coating_reservoir_surrogate.stl",
    "output/closed_ecm_coating_thickness_uniformity_witness_station_dispense_aspirate_manifolds.stl",
    "output/closed_ecm_coating_thickness_uniformity_witness_station_film_thickness_witness_coupon_matrix.stl",
    "output/closed_ecm_coating_thickness_uniformity_witness_station_uv_fluorescence_imaging_window_frame.stl",
    "output/closed_ecm_coating_thickness_uniformity_witness_station_edge_bead_dewetting_trap_array.stl",
    "output/closed_ecm_coating_thickness_uniformity_witness_station_timing_token_rails.stl",
    "output/closed_ecm_coating_thickness_uniformity_witness_station_contamination_witness_coupon_bank.stl",
    "output/closed_ecm_coating_thickness_uniformity_witness_station_release_hold_reject_gate_cartridge.stl",
    "output/closed_ecm_coating_thickness_uniformity_witness_station_robot_service_keepout_gauges.stl",
    "output/closed_ecm_coating_thickness_uniformity_witness_station_assembly.stl",
];

const FEATURE_NAMES: [&str; 11] = [
    "leak_tray_deck",
    "multi_chip_cassette_datum_nest",
    "coating_reservoir_surrogate",
    "dispense_aspirate_manifolds",
    "film_thickness_witness_coupon_matrix",
    "uv_fluorescence_imaging_window_frame",
    "edge_bead_dewetting_trap_array",
    "timing_token_rails",
    "contamination_witness_coupon_bank",
    "release_hold_reject_gate_cartridge",
    "robot_service_keepout_gauges",
];

const CASSETTE_COLS: usize = 4;
const CASSETTE_ROWS: usize = 5;
const CASSETTE_POSITIONS: usize = CASSETTE_COLS * CASSETTE_ROWS;
const EDGE_POSITIONS: usize = 14;
const CENTER_POSITIONS: usize = CASSETTE_POSITIONS - EDGE_POSITIONS;

const COATING_RESERVOIRS: usize = 4;
const DISPENSE_LANES: usize = CASSETTE_ROWS;
const ASPIRATE_LANES: usize = CASSETTE_ROWS;
const MANIFOLD_ROW_PORTS: usize = CASSETTE_ROWS * 2;
const FILM_THICKNESS_COUPONS: usize = CASSETTE_POSITIONS;
const EDGE_BEAD_TRAPS: usize = EDGE_POSITIONS;
const DEWETTING_SENTINELS: usize = CASSETTE_POSITIONS;
const TIMING_TOKEN_RAILS: usize = 6;
const TIMING_TOKENS: usize = 12;
const CONTAMINATION_COUPONS: usize = 12;
const IMAGING_WINDOWS: usize = 2;
const FIDUCIAL_TARGETS: usize = 8;
const RELEASE_GATE_COUNT: usize = 3;

const CHIP_GAP_X: f64 = 8.0;
const CHIP_GAP_Y: f64 = 6.0;
const CASSETTE_MARGIN_X: f64 = 38.0;
const CASSETTE_MARGIN_Y: f64 = 36.0;
const ARRAY_X: f64 =
    CASSETTE_COLS as f64 * REVC_CHIP_LENGTH + (CASSETTE_COLS as f64 - 1.0) * CHIP_GAP_X;
const ARRAY_Y: f64 =
    CASSETTE_ROWS as f64 * REVC_CHIP_WIDTH + (CASSETTE_ROWS as f64 - 1.0) * CHIP_GAP_Y;
const CASSETTE_X: f64 = ARRAY_X + 2.0 * CASSETTE_MARGIN_X;
const CASSETTE_Y: f64 = ARRAY_Y + 2.0 * CASSETTE_MARGIN_Y;
const PITCH_X: f64 = REVC_CHIP_LENGTH + CHIP_GAP_X;
const PITCH_Y: f64 = REVC_CHIP_WIDTH + CHIP_GAP_Y;

const DECK_X: f64 = 1840.0;
const DECK_Y: f64 = 1220.0;
const DECK_Z: f64 = 22.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 34.0;
const SOCKET_DEPTH: f64 = 6.0;
const MOUNT_HOLE_D: f64 = 6.8;
const DRAIN_D: f64 = 18.0;

const NEST_CENTER: (f64, f64) = (-220.0, 115.0);
const NEST_X: f64 = CASSETTE_X + 72.0;
const NEST_Y: f64 = CASSETTE_Y + 72.0;
const NEST_Z: f64 = 36.0;
const NEST_SOCKET_DEPTH: f64 = 9.0;
const NEST_RAIL_W: f64 = 16.0;
const NEST_RAIL_Z: f64 = 28.0;
const CHIP_RELIEF_Z: f64 = 8.0;
const DATUM_PIN_D: f64 = 7.2;

const RESERVOIR_CENTER: (f64, f64) = (-720.0, 315.0);
const RESERVOIR_X: f64 = 230.0;
const RESERVOIR_Y: f64 = 280.0;
const RESERVOIR_Z: f64 = 64.0;
const RESERVOIR_WELL_D: f64 = 42.0;
const RESERVOIR_WELL_DEPTH: f64 = 42.0;
const RESERVOIR_PITCH_Y: f64 = 58.0;

const MANIFOLD_CENTER: (f64, f64) = (520.0, 260.0);
const MANIFOLD_X: f64 = 440.0;
const MANIFOLD_Y: f64 = 310.0;
const MANIFOLD_Z: f64 = 54.0;
const DISPENSE_BORE_D: f64 = 6.0;
const ASPIRATE_BORE_D: f64 = 7.2;
const PORT_BOSS_D: f64 = 24.0;
const PORT_BORE_D: f64 = 4.2;
const MANIFOLD_ROW_PITCH_Y: f64 = 47.0;

const FILM_CENTER: (f64, f64) = (-620.0, -350.0);
const FILM_MATRIX_X: f64 = 420.0;
const FILM_MATRIX_Y: f64 = 295.0;
const FILM_MATRIX_Z: f64 = 30.0;
const COUPON_SLOT_X: f64 = 62.0;
const COUPON_SLOT_Y: f64 = 34.0;
const COUPON_SLOT_DEPTH: f64 = 14.0;
const COUPON_PITCH_X: f64 = 88.0;
const COUPON_PITCH_Y: f64 = 55.0;
const THICKNESS_STEP_MIN_Z: f64 = 1.2;
const THICKNESS_STEP_INCREMENT_Z: f64 = 0.35;

const IMAGING_FRAME_X: f64 = NEST_X + 150.0;
const IMAGING_FRAME_Y: f64 = 188.0;
const IMAGING_FRAME_Z: f64 = 70.0;
const IMAGING_RAIL_W: f64 = 16.0;
const UV_WINDOW_X: f64 = CASSETTE_X * 0.46;
const UV_WINDOW_Y: f64 = 84.0;
const FLUORESCENCE_WINDOW_X: f64 = CASSETTE_X * 0.46;
const FLUORESCENCE_WINDOW_Y: f64 = 84.0;
const CAMERA_CLEARANCE_Z: f64 = 184.0;

const TRAP_PLATE_X: f64 = CASSETTE_X + 38.0;
const TRAP_PLATE_Y: f64 = CASSETTE_Y + 36.0;
const TRAP_PLATE_Z: f64 = 12.0;
const EDGE_TRAP_SLOT_X: f64 = REVC_CHIP_LENGTH - 18.0;
const EDGE_TRAP_SLOT_Y: f64 = 10.0;
const DEWETTING_WITNESS_D: f64 = 13.0;
const EDGE_BEAD_GUTTER_W: f64 = 12.0;

const TIMING_CENTER: (f64, f64) = (100.0, -450.0);
const TIMING_X: f64 = 420.0;
const TIMING_Y: f64 = 108.0;
const TIMING_Z: f64 = 18.0;
const TOKEN_RAIL_X: f64 = 374.0;
const TOKEN_RAIL_Y: f64 = 8.0;
const TOKEN_D: f64 = 22.0;
const TOKEN_Z: f64 = 7.0;

const CONTAM_CENTER: (f64, f64) = (585.0, -165.0);
const CONTAM_X: f64 = 330.0;
const CONTAM_Y: f64 = 260.0;
const CONTAM_Z: f64 = 28.0;
const CONTAM_SLOT_X: f64 = 48.0;
const CONTAM_SLOT_Y: f64 = 28.0;
const CONTAM_SLOT_DEPTH: f64 = 12.0;
const CONTAM_PITCH_X: f64 = 76.0;
const CONTAM_PITCH_Y: f64 = 58.0;

const GATE_CENTER: (f64, f64) = (585.0, -440.0);
const GATE_X: f64 = 390.0;
const GATE_Y: f64 = 120.0;
const GATE_Z: f64 = 26.0;
const GATE_LANE_X: f64 = 104.0;
const GATE_LANE_Y: f64 = 72.0;
const GATE_PITCH_X: f64 = 122.0;
const GATE_FLAG_Z: f64 = 34.0;

const KEEP_OUT_X: f64 = 1720.0;
const KEEP_OUT_Y: f64 = 1110.0;
const KEEP_OUT_PAD_Z: f64 = 6.0;
const ROBOT_APPROACH_CLEARANCE: f64 = 330.0;
const RESERVOIR_SERVICE_CLEARANCE: f64 = 235.0;
const MANIFOLD_TUBE_PULL_CLEARANCE: f64 = 210.0;
const CAMERA_WORKING_CLEARANCE: f64 = 148.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside(self) -> bool {
        let usable_x = DECK_X / 2.0 - RIM_W - 12.0;
        let usable_y = DECK_Y / 2.0 - RIM_W - 12.0;

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

    let deck = leak_tray_deck();
    export(OUTPUTS[0], &deck);

    let nest = multi_chip_cassette_datum_nest();
    export(OUTPUTS[1], &nest);

    let reservoir = coating_reservoir_surrogate();
    export(OUTPUTS[2], &reservoir);

    let manifolds = dispense_aspirate_manifolds();
    export(OUTPUTS[3], &manifolds);

    let film = film_thickness_witness_coupon_matrix();
    export(OUTPUTS[4], &film);

    let imaging = uv_fluorescence_imaging_window_frame();
    export(OUTPUTS[5], &imaging);

    let traps = edge_bead_dewetting_trap_array();
    export(OUTPUTS[6], &traps);

    let timing = timing_token_rails();
    export(OUTPUTS[7], &timing);

    let contam = contamination_witness_coupon_bank();
    export(OUTPUTS[8], &contam);

    let gates = release_hold_reject_gate_cartridge();
    export(OUTPUTS[9], &gates);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[10], &keepouts);

    let assembly = deck
        + nest.translate(NEST_CENTER.0, NEST_CENTER.1, DECK_Z + NEST_Z / 2.0)
        + reservoir.translate(
            RESERVOIR_CENTER.0,
            RESERVOIR_CENTER.1,
            DECK_Z + RESERVOIR_Z / 2.0,
        )
        + manifolds.translate(
            MANIFOLD_CENTER.0,
            MANIFOLD_CENTER.1,
            DECK_Z + MANIFOLD_Z / 2.0,
        )
        + film.translate(FILM_CENTER.0, FILM_CENTER.1, DECK_Z + FILM_MATRIX_Z / 2.0)
        + traps.translate(
            NEST_CENTER.0,
            NEST_CENTER.1,
            DECK_Z + NEST_Z + TRAP_PLATE_Z / 2.0 + 4.0,
        )
        + imaging.translate(
            NEST_CENTER.0,
            NEST_CENTER.1,
            DECK_Z + NEST_Z + TRAP_PLATE_Z + IMAGING_FRAME_Z / 2.0 + 18.0,
        )
        + timing.translate(TIMING_CENTER.0, TIMING_CENTER.1, DECK_Z + TIMING_Z / 2.0)
        + contam.translate(CONTAM_CENTER.0, CONTAM_CENTER.1, DECK_Z + CONTAM_Z / 2.0)
        + gates.translate(GATE_CENTER.0, GATE_CENTER.1, DECK_Z + GATE_Z / 2.0)
        + keepouts.translate(0.0, 0.0, DECK_Z + KEEP_OUT_PAD_Z / 2.0);

    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed ECM/coating thickness uniformity witness station:");
    println!(
        "  Cassette map:               {CASSETTE_ROWS} rows x {CASSETTE_COLS} columns = {CASSETTE_POSITIONS} Rev C chip positions"
    );
    println!(
        "  Reservoir surrogate:        {COATING_RESERVOIRS} indexed coating/rinse/tracer wells with closed pickup lands"
    );
    println!(
        "  Fluid control:              {DISPENSE_LANES} dispense lanes, {ASPIRATE_LANES} aspirate lanes, {MANIFOLD_ROW_PORTS} row ports"
    );
    println!(
        "  Witnessing:                 {FILM_THICKNESS_COUPONS} film-thickness coupons, {EDGE_BEAD_TRAPS} edge-bead traps, {DEWETTING_SENTINELS} dewetting sentinels"
    );
    println!(
        "  Evidence controls:          {IMAGING_WINDOWS} imaging windows, {FIDUCIAL_TARGETS} fiducials, {CONTAMINATION_COUPONS} contamination coupons, {TIMING_TOKENS} timing tokens"
    );
    println!(
        "  Release routing:            {RELEASE_GATE_COUNT} release/hold/reject gates; clearances robot {ROBOT_APPROACH_CLEARANCE:.0}mm, reservoir {RESERVOIR_SERVICE_CLEARANCE:.0}mm, tube pull {MANIFOLD_TUBE_PULL_CLEARANCE:.0}mm, camera working {CAMERA_WORKING_CLEARANCE:.0}mm"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_layout() {
    assert_eq!(OUTPUTS.len(), FEATURE_NAMES.len() + 1);
    assert_eq!(CASSETTE_POSITIONS, 20);
    assert_eq!(EDGE_POSITIONS + CENTER_POSITIONS, CASSETTE_POSITIONS);
    assert_eq!(FILM_THICKNESS_COUPONS, CASSETTE_POSITIONS);
    assert_eq!(DEWETTING_SENTINELS, CASSETTE_POSITIONS);
    assert_eq!(DISPENSE_LANES, CASSETTE_ROWS);
    assert_eq!(ASPIRATE_LANES, CASSETTE_ROWS);
    assert!(NEST_Z > REVC_TOTAL_HEIGHT + 14.0);
    assert!(COUPON_SLOT_X < COUPON_PITCH_X);
    assert!(COUPON_SLOT_Y < COUPON_PITCH_Y);
    assert!(reservoir_span_y() + RESERVOIR_WELL_D < RESERVOIR_Y);
    assert!(manifold_row_span_y() + PORT_BOSS_D < MANIFOLD_Y);
    assert!(camera_clearance_above_cassette() > 120.0);

    let rects = layout_rects();
    for rect in rects {
        assert!(rect.fits_inside(), "{} does not fit on deck", rect.name);
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

fn layout_rects() -> [Rect; 7] {
    [
        Rect {
            name: "multi_chip_cassette_datum_nest",
            center: NEST_CENTER,
            x: NEST_X,
            y: NEST_Y,
        },
        Rect {
            name: "coating_reservoir_surrogate",
            center: RESERVOIR_CENTER,
            x: RESERVOIR_X,
            y: RESERVOIR_Y,
        },
        Rect {
            name: "dispense_aspirate_manifolds",
            center: MANIFOLD_CENTER,
            x: MANIFOLD_X,
            y: MANIFOLD_Y,
        },
        Rect {
            name: "film_thickness_witness_coupon_matrix",
            center: FILM_CENTER,
            x: FILM_MATRIX_X,
            y: FILM_MATRIX_Y,
        },
        Rect {
            name: "timing_token_rails",
            center: TIMING_CENTER,
            x: TIMING_X,
            y: TIMING_Y,
        },
        Rect {
            name: "contamination_witness_coupon_bank",
            center: CONTAM_CENTER,
            x: CONTAM_X,
            y: CONTAM_Y,
        },
        Rect {
            name: "release_hold_reject_gate_cartridge",
            center: GATE_CENTER,
            x: GATE_X,
            y: GATE_Y,
        },
    ]
}

fn leak_tray_deck() -> Part {
    let deck = centered_cube(
        "ecm_thickness_uniformity_leak_tray_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);

    let nest_socket = deck_socket(
        "ecm_thickness_uniformity_nest_socket",
        NEST_CENTER,
        NEST_X + 26.0,
        NEST_Y + 26.0,
    );
    let reservoir_socket = deck_socket(
        "ecm_thickness_uniformity_reservoir_socket",
        RESERVOIR_CENTER,
        RESERVOIR_X + 22.0,
        RESERVOIR_Y + 22.0,
    );
    let manifold_socket = deck_socket(
        "ecm_thickness_uniformity_manifold_socket",
        MANIFOLD_CENTER,
        MANIFOLD_X + 22.0,
        MANIFOLD_Y + 22.0,
    );
    let film_socket = deck_socket(
        "ecm_thickness_uniformity_film_coupon_socket",
        FILM_CENTER,
        FILM_MATRIX_X + 22.0,
        FILM_MATRIX_Y + 22.0,
    );
    let wet_gutter = centered_cube(
        "ecm_thickness_uniformity_front_wet_gutter",
        DECK_X - 160.0,
        18.0,
        8.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 104.0, DECK_Z - 3.0);
    let drain = centered_cylinder(
        "ecm_thickness_uniformity_leak_tray_drain",
        DRAIN_D / 2.0,
        48.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 88.0, -DECK_Y / 2.0 + 28.0, DECK_Z / 2.0);

    deck - nest_socket
        - reservoir_socket
        - manifold_socket
        - film_socket
        - wet_gutter
        - drain
        - deck_mounting_holes()
        + perimeter_rims()
        + deck_datum_targets()
        + process_zone_label_lands()
}

fn deck_socket(name: &str, center: (f64, f64), x: f64, y: f64) -> Part {
    centered_cube(name, x, y, SOCKET_DEPTH + 0.4).translate(
        center.0,
        center.1,
        DECK_Z - SOCKET_DEPTH / 2.0 + 0.2,
    )
}

fn deck_mounting_holes() -> Part {
    let mut holes = Part::empty("ecm_thickness_uniformity_deck_mounting_holes");
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 64.0, -DECK_Y / 2.0 + 58.0),
        (DECK_X / 2.0 - 64.0, -DECK_Y / 2.0 + 58.0),
        (-DECK_X / 2.0 + 64.0, DECK_Y / 2.0 - 58.0),
        (DECK_X / 2.0 - 64.0, DECK_Y / 2.0 - 58.0),
        (0.0, -DECK_Y / 2.0 + 58.0),
        (0.0, DECK_Y / 2.0 - 58.0),
        (-DECK_X / 2.0 + 64.0, 0.0),
        (DECK_X / 2.0 - 64.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("ecm_thickness_uniformity_m6_clearance_{i}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 4.0,
                28,
            )
            .translate(*x, *y, DECK_Z / 2.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    let rear = centered_cube(
        "ecm_thickness_uniformity_rear_leak_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "ecm_thickness_uniformity_left_leak_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "ecm_thickness_uniformity_right_leak_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let front_left = centered_cube(
        "ecm_thickness_uniformity_front_left_low_lip",
        620.0,
        RIM_W,
        16.0,
    )
    .translate(
        -DECK_X / 2.0 + 340.0,
        -DECK_Y / 2.0 + RIM_W / 2.0,
        DECK_Z + 8.0,
    );
    let front_right = centered_cube(
        "ecm_thickness_uniformity_front_right_low_lip",
        620.0,
        RIM_W,
        16.0,
    )
    .translate(
        DECK_X / 2.0 - 340.0,
        -DECK_Y / 2.0 + RIM_W / 2.0,
        DECK_Z + 8.0,
    );

    rear + left + right + front_left + front_right
}

fn deck_datum_targets() -> Part {
    let mut targets = Part::empty("ecm_thickness_uniformity_deck_datum_targets");
    for (i, (x, y)) in [
        (
            NEST_CENTER.0 - NEST_X / 2.0 + 44.0,
            NEST_CENTER.1 + NEST_Y / 2.0 - 42.0,
        ),
        (
            NEST_CENTER.0 + NEST_X / 2.0 - 44.0,
            NEST_CENTER.1 + NEST_Y / 2.0 - 42.0,
        ),
        (
            RESERVOIR_CENTER.0,
            RESERVOIR_CENTER.1 - RESERVOIR_Y / 2.0 + 34.0,
        ),
        (
            MANIFOLD_CENTER.0 + MANIFOLD_X / 2.0 - 36.0,
            MANIFOLD_CENTER.1 - MANIFOLD_Y / 2.0 + 34.0,
        ),
        (
            FILM_CENTER.0 - FILM_MATRIX_X / 2.0 + 34.0,
            FILM_CENTER.1 + FILM_MATRIX_Y / 2.0 - 34.0,
        ),
        (
            CONTAM_CENTER.0 + CONTAM_X / 2.0 - 34.0,
            CONTAM_CENTER.1 + CONTAM_Y / 2.0 - 34.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        let puck = centered_cylinder(
            format!("ecm_thickness_uniformity_deck_datum_puck_{i}"),
            10.0,
            3.0,
            40,
        )
        .translate(*x, *y, DECK_Z + 1.5);
        let center = centered_cylinder(
            format!("ecm_thickness_uniformity_deck_datum_center_{i}"),
            2.0,
            4.0,
            24,
        )
        .translate(*x, *y, DECK_Z + 1.5);
        targets = targets + (puck - center);
    }
    targets
}

fn process_zone_label_lands() -> Part {
    let reservoir = centered_cube(
        "ecm_thickness_uniformity_reservoir_zone_land",
        RESERVOIR_X,
        10.0,
        3.0,
    )
    .translate(
        RESERVOIR_CENTER.0,
        RESERVOIR_CENTER.1 - RESERVOIR_Y / 2.0 - 20.0,
        DECK_Z + 1.5,
    );
    let manifold = centered_cube(
        "ecm_thickness_uniformity_manifold_zone_land",
        MANIFOLD_X,
        10.0,
        3.0,
    )
    .translate(
        MANIFOLD_CENTER.0,
        MANIFOLD_CENTER.1 - MANIFOLD_Y / 2.0 - 20.0,
        DECK_Z + 1.5,
    );
    let film = centered_cube(
        "ecm_thickness_uniformity_coupon_zone_land",
        FILM_MATRIX_X,
        10.0,
        3.0,
    )
    .translate(
        FILM_CENTER.0,
        FILM_CENTER.1 + FILM_MATRIX_Y / 2.0 + 18.0,
        DECK_Z + 1.5,
    );
    let gate = centered_cube("ecm_thickness_uniformity_gate_zone_land", GATE_X, 10.0, 3.0)
        .translate(
            GATE_CENTER.0,
            GATE_CENTER.1 + GATE_Y / 2.0 + 18.0,
            DECK_Z + 1.5,
        );

    reservoir + manifold + film + gate
}

fn multi_chip_cassette_datum_nest() -> Part {
    let base = centered_cube(
        "ecm_thickness_uniformity_cassette_datum_nest_base",
        NEST_X,
        NEST_Y,
        NEST_Z,
    );
    let cassette_socket = centered_cube(
        "ecm_thickness_uniformity_cassette_socket",
        CASSETTE_X + 12.0,
        CASSETTE_Y + 12.0,
        NEST_SOCKET_DEPTH + 1.0,
    )
    .translate(0.0, 0.0, NEST_Z / 2.0 - NEST_SOCKET_DEPTH / 2.0 + 0.5);
    let side_tube_clearance = centered_cube(
        "ecm_thickness_uniformity_cassette_side_tube_clearance",
        62.0,
        CASSETTE_Y + 76.0,
        NEST_Z + 2.0,
    )
    .translate(-(CASSETTE_X / 2.0 + 48.0), 0.0, 2.0);
    let front_aspirate_clearance = centered_cube(
        "ecm_thickness_uniformity_front_aspirate_tube_clearance",
        CASSETTE_X - 80.0,
        26.0,
        NEST_Z + 2.0,
    )
    .translate(0.0, -(CASSETTE_Y / 2.0 + 32.0), 2.0);

    base - cassette_socket
        - side_tube_clearance
        - front_aspirate_clearance
        - cassette_chip_reliefs()
        + nest_datum_rails()
        + nest_datum_pins()
        + cassette_row_latch_bosses()
        + cassette_edge_center_marks()
}

fn cassette_chip_reliefs() -> Part {
    let mut cuts = Part::empty("ecm_thickness_uniformity_cassette_chip_reliefs");
    for row in 0..CASSETTE_ROWS {
        for col in 0..CASSETTE_COLS {
            let (x, y) = cassette_position_xy(row, col, PITCH_X, PITCH_Y);
            cuts = cuts
                + centered_cube(
                    format!("ecm_thickness_uniformity_chip_relief_r{row}_c{col}"),
                    REVC_CHIP_LENGTH - 16.0,
                    REVC_CHIP_WIDTH - 14.0,
                    CHIP_RELIEF_Z,
                )
                .translate(x, y, NEST_Z / 2.0 - CHIP_RELIEF_Z / 2.0 + 0.5);
        }
    }
    cuts
}

fn nest_datum_rails() -> Part {
    let left = centered_cube(
        "ecm_thickness_uniformity_left_hard_datum_rail",
        NEST_RAIL_W,
        NEST_Y,
        NEST_RAIL_Z,
    )
    .translate(
        -(NEST_X / 2.0 - NEST_RAIL_W / 2.0),
        0.0,
        NEST_Z / 2.0 + NEST_RAIL_Z / 2.0,
    );
    let back = centered_cube(
        "ecm_thickness_uniformity_back_hard_datum_rail",
        NEST_X,
        NEST_RAIL_W,
        NEST_RAIL_Z,
    )
    .translate(
        0.0,
        NEST_Y / 2.0 - NEST_RAIL_W / 2.0,
        NEST_Z / 2.0 + NEST_RAIL_Z / 2.0,
    );
    let front_soft = centered_cube(
        "ecm_thickness_uniformity_front_soft_capture_rail",
        NEST_X * 0.72,
        NEST_RAIL_W,
        NEST_RAIL_Z * 0.62,
    )
    .translate(
        24.0,
        -(NEST_Y / 2.0 - NEST_RAIL_W / 2.0),
        NEST_Z / 2.0 + NEST_RAIL_Z * 0.31,
    );
    let right_soft = centered_cube(
        "ecm_thickness_uniformity_right_soft_capture_rail",
        NEST_RAIL_W,
        NEST_Y * 0.72,
        NEST_RAIL_Z * 0.62,
    )
    .translate(
        NEST_X / 2.0 - NEST_RAIL_W / 2.0,
        -8.0,
        NEST_Z / 2.0 + NEST_RAIL_Z * 0.31,
    );

    left + back + front_soft + right_soft
}

fn nest_datum_pins() -> Part {
    let mut pins = Part::empty("ecm_thickness_uniformity_cassette_datum_pins");
    for (i, (x, y)) in [
        (-(CASSETTE_X / 2.0 - 30.0), CASSETTE_Y / 2.0 - 32.0),
        (CASSETTE_X / 2.0 - 42.0, CASSETTE_Y / 2.0 - 32.0),
        (-(CASSETTE_X / 2.0 - 30.0), -(CASSETTE_Y / 2.0 - 36.0)),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("ecm_thickness_uniformity_datum_pin_boss_{i}"),
            DATUM_PIN_D / 2.0 + 3.0,
            10.0,
            32,
        )
        .translate(*x, *y, NEST_Z / 2.0 + 5.0);
        let pilot = centered_cylinder(
            format!("ecm_thickness_uniformity_datum_pin_pilot_{i}"),
            DATUM_PIN_D / 2.0,
            12.0,
            28,
        )
        .translate(*x, *y, NEST_Z / 2.0 + 5.0);
        pins = pins + (boss - pilot);
    }
    pins
}

fn cassette_row_latch_bosses() -> Part {
    let mut bosses = Part::empty("ecm_thickness_uniformity_row_latch_bosses");
    for row in 0..CASSETTE_ROWS {
        let y = cassette_position_y(row, PITCH_Y);
        for side in [-1.0, 1.0] {
            let x = side * (CASSETTE_X / 2.0 + 18.0);
            let boss = centered_cylinder(
                format!("ecm_thickness_uniformity_row_{row}_latch_boss_{side}"),
                9.0,
                18.0,
                32,
            )
            .translate(x, y, NEST_Z / 2.0 + 9.0);
            let screw = centered_cylinder(
                format!("ecm_thickness_uniformity_row_{row}_latch_screw_{side}"),
                2.0,
                20.0,
                20,
            )
            .translate(x, y, NEST_Z / 2.0 + 9.0);
            bosses = bosses + (boss - screw);
        }
    }
    bosses
}

fn cassette_edge_center_marks() -> Part {
    let mut marks = Part::empty("ecm_thickness_uniformity_cassette_edge_center_marks");
    for row in 0..CASSETTE_ROWS {
        for col in 0..CASSETTE_COLS {
            let (x, y) = cassette_position_xy(row, col, PITCH_X, PITCH_Y);
            let idx = row * CASSETTE_COLS + col;
            if is_edge_position(row, col) {
                marks = marks
                    + centered_cube(
                        format!("ecm_thickness_uniformity_edge_position_land_{idx:02}"),
                        22.0,
                        8.0,
                        4.0,
                    )
                    .translate(
                        x - REVC_CHIP_LENGTH / 2.0 + 24.0,
                        y + REVC_CHIP_WIDTH / 2.0 - 16.0,
                        NEST_Z / 2.0 + 2.0,
                    );
            } else {
                marks = marks
                    + centered_cylinder(
                        format!("ecm_thickness_uniformity_center_position_land_{idx:02}"),
                        8.0,
                        4.0,
                        32,
                    )
                    .translate(
                        x - REVC_CHIP_LENGTH / 2.0 + 24.0,
                        y + REVC_CHIP_WIDTH / 2.0 - 16.0,
                        NEST_Z / 2.0 + 2.0,
                    );
            }
        }
    }
    marks
}

fn coating_reservoir_surrogate() -> Part {
    let body = centered_cube(
        "ecm_thickness_uniformity_coating_reservoir_surrogate_body",
        RESERVOIR_X,
        RESERVOIR_Y,
        RESERVOIR_Z,
    );
    let mut wells = Part::empty("ecm_thickness_uniformity_reservoir_well_cuts");
    let mut collars = Part::empty("ecm_thickness_uniformity_reservoir_pickup_collars");

    for i in 0..COATING_RESERVOIRS {
        let y = reservoir_well_y(i);
        wells = wells
            + centered_cylinder(
                format!("ecm_thickness_uniformity_reservoir_well_cut_{i}"),
                RESERVOIR_WELL_D / 2.0,
                RESERVOIR_WELL_DEPTH,
                44,
            )
            .translate(
                -28.0,
                y,
                RESERVOIR_Z / 2.0 - RESERVOIR_WELL_DEPTH / 2.0 + 0.5,
            );
        let collar = centered_cylinder(
            format!("ecm_thickness_uniformity_reservoir_pickup_collar_{i}"),
            RESERVOIR_WELL_D / 2.0 + 8.0,
            8.0,
            44,
        )
        .translate(-28.0, y, RESERVOIR_Z / 2.0 + 4.0);
        let pickup = centered_cylinder(
            format!("ecm_thickness_uniformity_reservoir_pickup_bore_{i}"),
            5.0,
            10.0,
            28,
        )
        .translate(-28.0, y, RESERVOIR_Z / 2.0 + 4.0);
        collars = collars + (collar - pickup);
    }

    body - wells
        + collars
        + reservoir_overflow_weirs()
        + reservoir_label_lands()
        + reservoir_level_gauge_steps()
}

fn reservoir_overflow_weirs() -> Part {
    let mut weirs = Part::empty("ecm_thickness_uniformity_reservoir_overflow_weirs");
    for i in 0..COATING_RESERVOIRS {
        let y = reservoir_well_y(i);
        weirs = weirs
            + centered_cube(
                format!("ecm_thickness_uniformity_reservoir_overflow_weir_{i}"),
                76.0,
                8.0,
                7.0,
            )
            .translate(54.0, y, RESERVOIR_Z / 2.0 + 3.5);
    }
    weirs
}

fn reservoir_label_lands() -> Part {
    let mut lands = Part::empty("ecm_thickness_uniformity_reservoir_label_lands");
    for i in 0..COATING_RESERVOIRS {
        lands = lands
            + centered_cube(
                format!("ecm_thickness_uniformity_reservoir_barcode_land_{i}"),
                76.0,
                16.0,
                4.0,
            )
            .translate(50.0, reservoir_well_y(i) + 20.0, RESERVOIR_Z / 2.0 + 2.0);
    }
    lands
}

fn reservoir_level_gauge_steps() -> Part {
    let mut steps = Part::empty("ecm_thickness_uniformity_reservoir_level_gauge_steps");
    for i in 0..5 {
        steps = steps
            + centered_cube(
                format!("ecm_thickness_uniformity_reservoir_level_step_{i}"),
                38.0 + i as f64 * 8.0,
                6.0,
                3.0,
            )
            .translate(
                -RESERVOIR_X / 2.0 + 44.0,
                RESERVOIR_Y / 2.0 - 34.0 - i as f64 * 14.0,
                RESERVOIR_Z / 2.0 + 1.5,
            );
    }
    steps
}

fn dispense_aspirate_manifolds() -> Part {
    let body = centered_cube(
        "ecm_thickness_uniformity_dispense_aspirate_manifold_body",
        MANIFOLD_X,
        MANIFOLD_Y,
        MANIFOLD_Z,
    );
    let mut bores = Part::empty("ecm_thickness_uniformity_manifold_bores");

    for row in 0..CASSETTE_ROWS {
        let y = manifold_row_y(row);
        bores = bores
            + centered_cylinder(
                format!("ecm_thickness_uniformity_dispense_row_bore_{row}"),
                DISPENSE_BORE_D / 2.0,
                MANIFOLD_X + 8.0,
                28,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, y + 12.0, 7.0);
        bores = bores
            + centered_cylinder(
                format!("ecm_thickness_uniformity_aspirate_row_bore_{row}"),
                ASPIRATE_BORE_D / 2.0,
                MANIFOLD_X + 8.0,
                28,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, y - 12.0, -7.0);
    }

    body - bores + row_port_bosses() + aspirate_trough_lands() + manifold_pressure_lands()
}

fn row_port_bosses() -> Part {
    let mut bosses = Part::empty("ecm_thickness_uniformity_row_port_bosses");
    for row in 0..CASSETTE_ROWS {
        let y = manifold_row_y(row);
        for (kind, x, dy) in [("dispense", -142.0, 12.0), ("aspirate", 142.0, -12.0)] {
            let boss = centered_cylinder(
                format!("ecm_thickness_uniformity_{kind}_row_{row}_port_boss"),
                PORT_BOSS_D / 2.0,
                16.0,
                36,
            )
            .translate(x, y + dy, MANIFOLD_Z / 2.0 + 8.0);
            let bore = centered_cylinder(
                format!("ecm_thickness_uniformity_{kind}_row_{row}_port_bore"),
                PORT_BORE_D / 2.0,
                18.0,
                24,
            )
            .translate(x, y + dy, MANIFOLD_Z / 2.0 + 8.0);
            bosses = bosses + (boss - bore);
        }
    }
    bosses
}

fn aspirate_trough_lands() -> Part {
    let mut lands = Part::empty("ecm_thickness_uniformity_aspirate_trough_lands");
    for row in 0..CASSETTE_ROWS {
        lands = lands
            + centered_cube(
                format!("ecm_thickness_uniformity_aspirate_row_{row}_trough_land"),
                MANIFOLD_X - 70.0,
                8.0,
                5.0,
            )
            .translate(0.0, manifold_row_y(row) - 23.0, MANIFOLD_Z / 2.0 + 2.5);
    }
    lands
}

fn manifold_pressure_lands() -> Part {
    let mut lands = Part::empty("ecm_thickness_uniformity_manifold_pressure_lands");
    for i in 0..4 {
        let x = -150.0 + i as f64 * 100.0;
        lands = lands
            + centered_cylinder(
                format!("ecm_thickness_uniformity_manifold_pressure_land_{i}"),
                13.0,
                5.0,
                36,
            )
            .translate(x, 0.0, MANIFOLD_Z / 2.0 + 2.5);
    }
    lands
}

fn film_thickness_witness_coupon_matrix() -> Part {
    let base = centered_cube(
        "ecm_thickness_uniformity_film_coupon_matrix_base",
        FILM_MATRIX_X,
        FILM_MATRIX_Y,
        FILM_MATRIX_Z,
    );
    let mut slots = Part::empty("ecm_thickness_uniformity_film_coupon_slots");
    let mut lands = Part::empty("ecm_thickness_uniformity_film_thickness_step_lands");

    for row in 0..CASSETTE_ROWS {
        for col in 0..CASSETTE_COLS {
            let (x, y) = coupon_position_xy(row, col);
            let idx = row * CASSETTE_COLS + col;
            slots = slots
                + centered_cube(
                    format!("ecm_thickness_uniformity_coupon_slot_r{row}_c{col}"),
                    COUPON_SLOT_X,
                    COUPON_SLOT_Y,
                    COUPON_SLOT_DEPTH + 1.0,
                )
                .translate(
                    x,
                    y,
                    FILM_MATRIX_Z / 2.0 - COUPON_SLOT_DEPTH / 2.0 + 0.5,
                );

            let step_z = thickness_step_height(idx);
            lands = lands
                + centered_cube(
                    format!("ecm_thickness_uniformity_film_step_land_{idx:02}"),
                    34.0,
                    8.0,
                    step_z,
                )
                .translate(
                    x,
                    y - COUPON_SLOT_Y / 2.0 - 10.0,
                    FILM_MATRIX_Z / 2.0 + step_z / 2.0,
                );
        }
    }

    base - slots + lands + coupon_retainer_bars() + coupon_chain_of_custody_land()
}

fn coupon_retainer_bars() -> Part {
    let mut bars = Part::empty("ecm_thickness_uniformity_coupon_retainer_bars");
    for row in 0..CASSETTE_ROWS {
        bars = bars
            + centered_cube(
                format!("ecm_thickness_uniformity_coupon_row_{row}_retainer_bar"),
                FILM_MATRIX_X - 54.0,
                6.0,
                6.0,
            )
            .translate(
                0.0,
                coupon_position_y(row) + COUPON_SLOT_Y / 2.0 + 12.0,
                FILM_MATRIX_Z / 2.0 + 3.0,
            );
    }
    bars
}

fn coupon_chain_of_custody_land() -> Part {
    let run_land = centered_cube(
        "ecm_thickness_uniformity_coupon_chain_of_custody_land",
        FILM_MATRIX_X - 68.0,
        18.0,
        4.0,
    )
    .translate(0.0, -FILM_MATRIX_Y / 2.0 + 22.0, FILM_MATRIX_Z / 2.0 + 2.0);
    let edge_center_key = centered_cube(
        "ecm_thickness_uniformity_edge_center_coupon_key_land",
        132.0,
        20.0,
        4.0,
    )
    .translate(
        FILM_MATRIX_X / 2.0 - 92.0,
        FILM_MATRIX_Y / 2.0 - 24.0,
        FILM_MATRIX_Z / 2.0 + 2.0,
    );
    run_land + edge_center_key
}

fn uv_fluorescence_imaging_window_frame() -> Part {
    let front = centered_cube(
        "ecm_thickness_uniformity_imaging_front_light_bar",
        IMAGING_FRAME_X,
        IMAGING_RAIL_W,
        IMAGING_FRAME_Z,
    )
    .translate(0.0, -IMAGING_FRAME_Y / 2.0 + IMAGING_RAIL_W / 2.0, 0.0);
    let rear = centered_cube(
        "ecm_thickness_uniformity_imaging_rear_light_bar",
        IMAGING_FRAME_X,
        IMAGING_RAIL_W,
        IMAGING_FRAME_Z,
    )
    .translate(0.0, IMAGING_FRAME_Y / 2.0 - IMAGING_RAIL_W / 2.0, 0.0);
    let left = centered_cube(
        "ecm_thickness_uniformity_imaging_left_fiducial_rail",
        IMAGING_RAIL_W,
        IMAGING_FRAME_Y,
        IMAGING_FRAME_Z,
    )
    .translate(-IMAGING_FRAME_X / 2.0 + IMAGING_RAIL_W / 2.0, 0.0, 0.0);
    let right = centered_cube(
        "ecm_thickness_uniformity_imaging_right_fiducial_rail",
        IMAGING_RAIL_W,
        IMAGING_FRAME_Y,
        IMAGING_FRAME_Z,
    )
    .translate(IMAGING_FRAME_X / 2.0 - IMAGING_RAIL_W / 2.0, 0.0, 0.0);
    let center_bridge = centered_cube(
        "ecm_thickness_uniformity_uv_fluorescence_center_bridge",
        18.0,
        IMAGING_FRAME_Y,
        IMAGING_FRAME_Z,
    )
    .translate(0.0, 0.0, 0.0);

    front
        + rear
        + left
        + right
        + center_bridge
        + imaging_window_lands()
        + imaging_fiducials()
        + led_bar_lands()
}

fn imaging_window_lands() -> Part {
    let uv = centered_cube(
        "ecm_thickness_uniformity_uv_window_reference_land",
        UV_WINDOW_X,
        UV_WINDOW_Y,
        4.0,
    )
    .translate(-UV_WINDOW_X / 2.0 - 18.0, 0.0, IMAGING_FRAME_Z / 2.0 + 2.0);
    let fluorescence = centered_cube(
        "ecm_thickness_uniformity_fluorescence_window_reference_land",
        FLUORESCENCE_WINDOW_X,
        FLUORESCENCE_WINDOW_Y,
        4.0,
    )
    .translate(
        FLUORESCENCE_WINDOW_X / 2.0 + 18.0,
        0.0,
        IMAGING_FRAME_Z / 2.0 + 2.0,
    );
    let uv_shield = centered_cube(
        "ecm_thickness_uniformity_uv_shield_slide_land",
        UV_WINDOW_X,
        12.0,
        6.0,
    )
    .translate(
        -UV_WINDOW_X / 2.0 - 18.0,
        -UV_WINDOW_Y / 2.0 - 16.0,
        IMAGING_FRAME_Z / 2.0 + 3.0,
    );
    let fluorescence_filter = centered_cube(
        "ecm_thickness_uniformity_fluorescence_filter_slide_land",
        FLUORESCENCE_WINDOW_X,
        12.0,
        6.0,
    )
    .translate(
        FLUORESCENCE_WINDOW_X / 2.0 + 18.0,
        FLUORESCENCE_WINDOW_Y / 2.0 + 16.0,
        IMAGING_FRAME_Z / 2.0 + 3.0,
    );

    uv + fluorescence + uv_shield + fluorescence_filter
}

fn imaging_fiducials() -> Part {
    let mut fiducials = Part::empty("ecm_thickness_uniformity_imaging_fiducials");
    for (i, (x, y)) in [
        (-IMAGING_FRAME_X / 2.0 + 44.0, -IMAGING_FRAME_Y / 2.0 + 36.0),
        (IMAGING_FRAME_X / 2.0 - 44.0, -IMAGING_FRAME_Y / 2.0 + 36.0),
        (-IMAGING_FRAME_X / 2.0 + 44.0, IMAGING_FRAME_Y / 2.0 - 36.0),
        (IMAGING_FRAME_X / 2.0 - 44.0, IMAGING_FRAME_Y / 2.0 - 36.0),
        (-145.0, -IMAGING_FRAME_Y / 2.0 + 36.0),
        (145.0, -IMAGING_FRAME_Y / 2.0 + 36.0),
        (-145.0, IMAGING_FRAME_Y / 2.0 - 36.0),
        (145.0, IMAGING_FRAME_Y / 2.0 - 36.0),
    ]
    .iter()
    .enumerate()
    {
        let target = centered_cylinder(
            format!("ecm_thickness_uniformity_imaging_fiducial_target_{i}"),
            10.0,
            4.0,
            36,
        )
        .translate(*x, *y, IMAGING_FRAME_Z / 2.0 + 2.0);
        let center = centered_cylinder(
            format!("ecm_thickness_uniformity_imaging_fiducial_center_cut_{i}"),
            2.0,
            5.0,
            24,
        )
        .translate(*x, *y, IMAGING_FRAME_Z / 2.0 + 2.0);
        fiducials = fiducials + (target - center);
    }
    fiducials
}

fn led_bar_lands() -> Part {
    let mut bars = Part::empty("ecm_thickness_uniformity_imaging_led_bar_lands");
    for i in 0..4 {
        let x = -300.0 + i as f64 * 200.0;
        bars = bars
            + centered_cube(
                format!("ecm_thickness_uniformity_led_bar_land_{i}"),
                128.0,
                14.0,
                5.0,
            )
            .translate(
                x,
                -IMAGING_FRAME_Y / 2.0 + 38.0,
                IMAGING_FRAME_Z / 2.0 + 2.5,
            );
    }
    bars
}

fn edge_bead_dewetting_trap_array() -> Part {
    let plate = centered_cube(
        "ecm_thickness_uniformity_edge_bead_dewetting_trap_plate",
        TRAP_PLATE_X,
        TRAP_PLATE_Y,
        TRAP_PLATE_Z,
    );
    let mut chip_windows = Part::empty("ecm_thickness_uniformity_trap_chip_windows");
    let mut traps = Part::empty("ecm_thickness_uniformity_edge_bead_trap_lands");
    let mut sentinels = Part::empty("ecm_thickness_uniformity_dewetting_sentinels");

    for row in 0..CASSETTE_ROWS {
        for col in 0..CASSETTE_COLS {
            let (x, y) = cassette_position_xy(row, col, PITCH_X, PITCH_Y);
            let idx = row * CASSETTE_COLS + col;
            chip_windows = chip_windows
                + centered_cube(
                    format!("ecm_thickness_uniformity_trap_window_r{row}_c{col}"),
                    REVC_CHIP_LENGTH - 22.0,
                    REVC_CHIP_WIDTH - 20.0,
                    TRAP_PLATE_Z + 2.0,
                )
                .translate(x, y, 0.0);

            sentinels = sentinels
                + centered_cylinder(
                    format!("ecm_thickness_uniformity_dewetting_sentinel_{idx:02}"),
                    DEWETTING_WITNESS_D / 2.0,
                    4.0,
                    28,
                )
                .translate(
                    x + REVC_CHIP_LENGTH / 2.0 - 18.0,
                    y - REVC_CHIP_WIDTH / 2.0 + 18.0,
                    TRAP_PLATE_Z / 2.0 + 2.0,
                );

            if is_edge_position(row, col) {
                traps = traps + edge_bead_trap_for_position(row, col, x, y, idx);
            }
        }
    }

    plate - chip_windows + traps + sentinels + perimeter_dewetting_gutters()
}

fn edge_bead_trap_for_position(row: usize, col: usize, x: f64, y: f64, idx: usize) -> Part {
    let top_or_bottom = row == 0 || row == CASSETTE_ROWS - 1;
    if top_or_bottom {
        let y_offset = if row == 0 { 1.0 } else { -1.0 } * (REVC_CHIP_WIDTH / 2.0 - 9.0);
        centered_cube(
            format!("ecm_thickness_uniformity_edge_bead_trap_bar_{idx:02}"),
            EDGE_TRAP_SLOT_X,
            EDGE_TRAP_SLOT_Y,
            5.0,
        )
        .translate(x, y + y_offset, TRAP_PLATE_Z / 2.0 + 2.5)
    } else {
        let x_offset = if col == 0 { -1.0 } else { 1.0 } * (REVC_CHIP_LENGTH / 2.0 - 9.0);
        centered_cube(
            format!("ecm_thickness_uniformity_edge_bead_side_trap_bar_{idx:02}"),
            EDGE_TRAP_SLOT_Y,
            REVC_CHIP_WIDTH - 18.0,
            5.0,
        )
        .translate(x + x_offset, y, TRAP_PLATE_Z / 2.0 + 2.5)
    }
}

fn perimeter_dewetting_gutters() -> Part {
    let top = centered_cube(
        "ecm_thickness_uniformity_top_edge_dewetting_gutter",
        TRAP_PLATE_X - 48.0,
        EDGE_BEAD_GUTTER_W,
        4.0,
    )
    .translate(0.0, TRAP_PLATE_Y / 2.0 - 26.0, TRAP_PLATE_Z / 2.0 + 2.0);
    let bottom = centered_cube(
        "ecm_thickness_uniformity_bottom_edge_dewetting_gutter",
        TRAP_PLATE_X - 48.0,
        EDGE_BEAD_GUTTER_W,
        4.0,
    )
    .translate(0.0, -TRAP_PLATE_Y / 2.0 + 26.0, TRAP_PLATE_Z / 2.0 + 2.0);
    let left = centered_cube(
        "ecm_thickness_uniformity_left_edge_dewetting_gutter",
        EDGE_BEAD_GUTTER_W,
        TRAP_PLATE_Y - 48.0,
        4.0,
    )
    .translate(-TRAP_PLATE_X / 2.0 + 26.0, 0.0, TRAP_PLATE_Z / 2.0 + 2.0);
    let right = centered_cube(
        "ecm_thickness_uniformity_right_edge_dewetting_gutter",
        EDGE_BEAD_GUTTER_W,
        TRAP_PLATE_Y - 48.0,
        4.0,
    )
    .translate(TRAP_PLATE_X / 2.0 - 26.0, 0.0, TRAP_PLATE_Z / 2.0 + 2.0);

    top + bottom + left + right
}

fn timing_token_rails() -> Part {
    let base = centered_cube(
        "ecm_thickness_uniformity_timing_token_rail_base",
        TIMING_X,
        TIMING_Y,
        TIMING_Z,
    );
    let mut rails = Part::empty("ecm_thickness_uniformity_timing_token_rails");
    let mut tokens = Part::empty("ecm_thickness_uniformity_timing_tokens");

    for rail in 0..TIMING_TOKEN_RAILS {
        let y = -TIMING_Y / 2.0 + 18.0 + rail as f64 * 14.0;
        rails = rails
            + centered_cube(
                format!("ecm_thickness_uniformity_timing_rail_{rail}"),
                TOKEN_RAIL_X,
                TOKEN_RAIL_Y,
                5.0,
            )
            .translate(0.0, y, TIMING_Z / 2.0 + 2.5);
    }

    for i in 0..TIMING_TOKENS {
        let x = -TIMING_X / 2.0 + 44.0 + (i % 6) as f64 * 66.0;
        let y = if i < 6 { 26.0 } else { -26.0 };
        tokens = tokens
            + centered_cylinder(
                format!("ecm_thickness_uniformity_timing_token_{i}"),
                TOKEN_D / 2.0,
                TOKEN_Z,
                36,
            )
            .translate(x, y, TIMING_Z / 2.0 + TOKEN_Z / 2.0);
    }

    base + rails + tokens + timing_barcode_land()
}

fn timing_barcode_land() -> Part {
    centered_cube(
        "ecm_thickness_uniformity_timing_run_barcode_land",
        TIMING_X - 62.0,
        16.0,
        4.0,
    )
    .translate(0.0, -TIMING_Y / 2.0 + 14.0, TIMING_Z / 2.0 + 2.0)
}

fn contamination_witness_coupon_bank() -> Part {
    let base = centered_cube(
        "ecm_thickness_uniformity_contamination_witness_coupon_bank",
        CONTAM_X,
        CONTAM_Y,
        CONTAM_Z,
    );
    let mut slots = Part::empty("ecm_thickness_uniformity_contamination_coupon_slots");
    let mut lands = Part::empty("ecm_thickness_uniformity_contamination_coupon_lands");

    for i in 0..CONTAMINATION_COUPONS {
        let row = i / 4;
        let col = i % 4;
        let x = (col as f64 - 1.5) * CONTAM_PITCH_X;
        let y = (1.0 - row as f64) * CONTAM_PITCH_Y;
        slots = slots
            + centered_cube(
                format!("ecm_thickness_uniformity_contamination_coupon_slot_{i}"),
                CONTAM_SLOT_X,
                CONTAM_SLOT_Y,
                CONTAM_SLOT_DEPTH + 1.0,
            )
            .translate(x, y, CONTAM_Z / 2.0 - CONTAM_SLOT_DEPTH / 2.0 + 0.5);
        lands = lands
            + centered_cube(
                format!("ecm_thickness_uniformity_contamination_coupon_label_{i}"),
                42.0,
                8.0,
                3.0,
            )
            .translate(x, y - CONTAM_SLOT_Y / 2.0 - 9.0, CONTAM_Z / 2.0 + 1.5);
    }

    base - slots + lands + contamination_zone_keys()
}

fn contamination_zone_keys() -> Part {
    let reservoir_key = centered_cube(
        "ecm_thickness_uniformity_contam_reservoir_zone_key",
        CONTAM_X - 58.0,
        10.0,
        3.0,
    )
    .translate(0.0, CONTAM_Y / 2.0 - 18.0, CONTAM_Z / 2.0 + 1.5);
    let aspirate_key = centered_cube(
        "ecm_thickness_uniformity_contam_aspirate_zone_key",
        CONTAM_X - 58.0,
        10.0,
        3.0,
    )
    .translate(0.0, -CONTAM_Y / 2.0 + 18.0, CONTAM_Z / 2.0 + 1.5);
    reservoir_key + aspirate_key
}

fn release_hold_reject_gate_cartridge() -> Part {
    let base = centered_cube(
        "ecm_thickness_uniformity_release_hold_reject_gate_cartridge",
        GATE_X,
        GATE_Y,
        GATE_Z,
    );
    let mut lanes = Part::empty("ecm_thickness_uniformity_release_hold_reject_lanes");
    let mut flags = Part::empty("ecm_thickness_uniformity_release_hold_reject_flags");

    for gate in 0..RELEASE_GATE_COUNT {
        let x = (gate as f64 - 1.0) * GATE_PITCH_X;
        lanes = lanes
            + centered_cube(
                format!("ecm_thickness_uniformity_gate_lane_cut_{gate}"),
                GATE_LANE_X,
                GATE_LANE_Y,
                GATE_Z + 2.0,
            )
            .translate(x, 0.0, 1.0);
        flags = flags
            + centered_cube(
                format!("ecm_thickness_uniformity_gate_flag_{gate}"),
                38.0,
                12.0,
                GATE_FLAG_Z,
            )
            .translate(
                x,
                GATE_LANE_Y / 2.0 + 12.0,
                GATE_Z / 2.0 + GATE_FLAG_Z / 2.0,
            );
        flags = flags
            + centered_cube(
                format!("ecm_thickness_uniformity_gate_barcode_land_{gate}"),
                86.0,
                18.0,
                4.0,
            )
            .translate(x, -GATE_LANE_Y / 2.0 - 12.0, GATE_Z / 2.0 + 2.0);
    }

    base - lanes + flags + gate_hinge_pins()
}

fn gate_hinge_pins() -> Part {
    let mut pins = Part::empty("ecm_thickness_uniformity_gate_hinge_pins");
    for gate in 0..RELEASE_GATE_COUNT {
        let x = (gate as f64 - 1.0) * GATE_PITCH_X - GATE_LANE_X / 2.0 - 10.0;
        pins = pins
            + centered_cylinder(
                format!("ecm_thickness_uniformity_gate_hinge_pin_{gate}"),
                5.0,
                GATE_Y - 26.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, GATE_Z / 2.0 + 5.0);
    }
    pins
}

fn robot_service_keepout_gauges() -> Part {
    let front = centered_cube(
        "ecm_thickness_uniformity_robot_front_approach_keepout_gauge",
        KEEP_OUT_X,
        KEEP_OUT_PAD_Z,
        KEEP_OUT_PAD_Z,
    )
    .translate(0.0, -KEEP_OUT_Y / 2.0, 0.0);
    let rear = centered_cube(
        "ecm_thickness_uniformity_rear_service_keepout_gauge",
        KEEP_OUT_X,
        KEEP_OUT_PAD_Z,
        KEEP_OUT_PAD_Z,
    )
    .translate(0.0, KEEP_OUT_Y / 2.0, 0.0);
    let left = centered_cube(
        "ecm_thickness_uniformity_left_service_keepout_gauge",
        KEEP_OUT_PAD_Z,
        KEEP_OUT_Y,
        KEEP_OUT_PAD_Z,
    )
    .translate(-KEEP_OUT_X / 2.0, 0.0, 0.0);
    let right = centered_cube(
        "ecm_thickness_uniformity_right_service_keepout_gauge",
        KEEP_OUT_PAD_Z,
        KEEP_OUT_Y,
        KEEP_OUT_PAD_Z,
    )
    .translate(KEEP_OUT_X / 2.0, 0.0, 0.0);
    let camera = centered_cube(
        "ecm_thickness_uniformity_camera_working_clearance_gauge",
        42.0,
        42.0,
        CAMERA_CLEARANCE_Z,
    )
    .translate(NEST_CENTER.0, NEST_CENTER.1, CAMERA_CLEARANCE_Z / 2.0);
    let reservoir_pull = centered_cube(
        "ecm_thickness_uniformity_reservoir_cartridge_pull_gauge",
        RESERVOIR_X,
        RESERVOIR_SERVICE_CLEARANCE,
        8.0,
    )
    .translate(
        RESERVOIR_CENTER.0,
        RESERVOIR_CENTER.1 + RESERVOIR_Y / 2.0 + RESERVOIR_SERVICE_CLEARANCE / 2.0,
        4.0,
    );
    let manifold_pull = centered_cube(
        "ecm_thickness_uniformity_manifold_tube_pull_gauge",
        MANIFOLD_TUBE_PULL_CLEARANCE,
        18.0,
        42.0,
    )
    .translate(
        MANIFOLD_CENTER.0 + MANIFOLD_X / 2.0 + MANIFOLD_TUBE_PULL_CLEARANCE / 2.0,
        MANIFOLD_CENTER.1,
        21.0,
    );
    let robot_front = centered_cube(
        "ecm_thickness_uniformity_robot_approach_depth_gauge",
        72.0,
        ROBOT_APPROACH_CLEARANCE,
        8.0,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 - ROBOT_APPROACH_CLEARANCE / 2.0 + 68.0,
        4.0,
    );

    front + rear + left + right + camera + reservoir_pull + manifold_pull + robot_front
}

fn cassette_position_xy(row: usize, col: usize, pitch_x: f64, pitch_y: f64) -> (f64, f64) {
    (
        cassette_position_x(col, pitch_x),
        cassette_position_y(row, pitch_y),
    )
}

fn cassette_position_x(col: usize, pitch_x: f64) -> f64 {
    (col as f64 - (CASSETTE_COLS as f64 - 1.0) / 2.0) * pitch_x
}

fn cassette_position_y(row: usize, pitch_y: f64) -> f64 {
    ((CASSETTE_ROWS as f64 - 1.0) / 2.0 - row as f64) * pitch_y
}

fn coupon_position_xy(row: usize, col: usize) -> (f64, f64) {
    (
        (col as f64 - (CASSETTE_COLS as f64 - 1.0) / 2.0) * COUPON_PITCH_X,
        coupon_position_y(row),
    )
}

fn coupon_position_y(row: usize) -> f64 {
    ((CASSETTE_ROWS as f64 - 1.0) / 2.0 - row as f64) * COUPON_PITCH_Y
}

fn is_edge_position(row: usize, col: usize) -> bool {
    row == 0 || row == CASSETTE_ROWS - 1 || col == 0 || col == CASSETTE_COLS - 1
}

fn reservoir_well_y(index: usize) -> f64 {
    -reservoir_span_y() / 2.0 + index as f64 * RESERVOIR_PITCH_Y
}

fn reservoir_span_y() -> f64 {
    RESERVOIR_PITCH_Y * (COATING_RESERVOIRS as f64 - 1.0)
}

fn manifold_row_y(row: usize) -> f64 {
    -manifold_row_span_y() / 2.0 + row as f64 * MANIFOLD_ROW_PITCH_Y
}

fn manifold_row_span_y() -> f64 {
    MANIFOLD_ROW_PITCH_Y * (CASSETTE_ROWS as f64 - 1.0)
}

fn thickness_step_height(index: usize) -> f64 {
    THICKNESS_STEP_MIN_Z + (index % CASSETTE_COLS) as f64 * THICKNESS_STEP_INCREMENT_Z
}

fn camera_clearance_above_cassette() -> f64 {
    CAMERA_CLEARANCE_Z - (NEST_Z + TRAP_PLATE_Z + REVC_TOTAL_HEIGHT)
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
        assert_eq!(OUTPUTS.len(), 12);
        for path in OUTPUTS {
            assert!(
                path.starts_with("output/closed_ecm_coating_thickness_uniformity_witness_station_"),
                "{path} is not scoped to this generator"
            );
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn cassette_map_matches_twenty_chip_closed_workflow() {
        assert_eq!(CASSETTE_ROWS * CASSETTE_COLS, CASSETTE_POSITIONS);
        assert_eq!(CASSETTE_POSITIONS, 20);

        let mut edge_count = 0;
        let mut center_count = 0;
        for row in 0..CASSETTE_ROWS {
            for col in 0..CASSETTE_COLS {
                if is_edge_position(row, col) {
                    edge_count += 1;
                } else {
                    center_count += 1;
                }
            }
        }

        assert_eq!(edge_count, EDGE_POSITIONS);
        assert_eq!(center_count, CENTER_POSITIONS);
        assert_eq!(CENTER_POSITIONS, 6);
    }

    #[test]
    fn coating_thickness_witnesses_cover_all_cassette_positions() {
        assert_eq!(FILM_THICKNESS_COUPONS, CASSETTE_POSITIONS);
        assert_eq!(DEWETTING_SENTINELS, CASSETTE_POSITIONS);
        assert_eq!(EDGE_BEAD_TRAPS, EDGE_POSITIONS);
        assert!(COUPON_SLOT_X < COUPON_PITCH_X);
        assert!(COUPON_SLOT_Y < COUPON_PITCH_Y);
        assert!(thickness_step_height(0) < thickness_step_height(3));
        assert_eq!(
            thickness_step_height(0),
            thickness_step_height(CASSETTE_COLS)
        );
    }

    #[test]
    fn reservoir_and_manifold_counts_are_controlled() {
        assert_eq!(COATING_RESERVOIRS, 4);
        assert_eq!(DISPENSE_LANES, CASSETTE_ROWS);
        assert_eq!(ASPIRATE_LANES, CASSETTE_ROWS);
        assert_eq!(MANIFOLD_ROW_PORTS, 10);
        assert!(reservoir_span_y() + RESERVOIR_WELL_D < RESERVOIR_Y);
        assert!(manifold_row_span_y() + PORT_BOSS_D < MANIFOLD_Y);
    }

    #[test]
    fn imaging_and_process_gates_are_explicit() {
        assert_eq!(IMAGING_WINDOWS, 2);
        assert_eq!(FIDUCIAL_TARGETS, 8);
        assert_eq!(TIMING_TOKEN_RAILS, 6);
        assert_eq!(TIMING_TOKENS, 12);
        assert_eq!(CONTAMINATION_COUPONS, 12);
        assert_eq!(RELEASE_GATE_COUNT, 3);
        assert!(camera_clearance_above_cassette() > 120.0);
    }

    #[test]
    fn major_modules_fit_without_planar_collisions() {
        assert_layout();
    }

    #[test]
    fn service_keepouts_clear_closed_fixture_operations() {
        assert!(ROBOT_APPROACH_CLEARANCE >= 300.0);
        assert!(RESERVOIR_SERVICE_CLEARANCE >= 220.0);
        assert!(MANIFOLD_TUBE_PULL_CLEARANCE >= 200.0);
        assert!(CAMERA_WORKING_CLEARANCE > REVC_TOTAL_HEIGHT + 100.0);
        assert!(KEEP_OUT_X < DECK_X);
        assert!(KEEP_OUT_Y < DECK_Y);
    }
}
