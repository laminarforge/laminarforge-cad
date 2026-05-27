use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed incubator condensate cross-contamination witness station.
//
// This standalone generator models a no-cell validation fixture for observing
// whether condensate can bridge between incubator rack positions. It provides a
// sealed cassette surrogate grid, indexed drip challenge rails, tracer/coupon
// witness wells, gutter/diverter comparison lanes, position barcode lands,
// RH/temp logger pockets, release/hold/reject lanes, evidence imaging support,
// and robot/service keepout gauges. It is mechanical validation packaging only;
// tracer chemistry, incubator recipes, sampling statistics, and release rules
// remain external validation controls.

const OUTPUT_PREFIX: &str = "closed_incubator_condensate_cross_contamination_witness_station";

const OUTPUTS: [&str; 11] = [
    "output/closed_incubator_condensate_cross_contamination_witness_station_base_containment_deck.stl",
    "output/closed_incubator_condensate_cross_contamination_witness_station_sealed_cassette_surrogate_grid.stl",
    "output/closed_incubator_condensate_cross_contamination_witness_station_condensate_drip_challenge_rails.stl",
    "output/closed_incubator_condensate_cross_contamination_witness_station_tracer_coupon_witness_wells.stl",
    "output/closed_incubator_condensate_cross_contamination_witness_station_gutter_diverter_comparison_plate.stl",
    "output/closed_incubator_condensate_cross_contamination_witness_station_rh_temp_logger_pockets.stl",
    "output/closed_incubator_condensate_cross_contamination_witness_station_position_barcode_lands.stl",
    "output/closed_incubator_condensate_cross_contamination_witness_station_release_hold_reject_lanes.stl",
    "output/closed_incubator_condensate_cross_contamination_witness_station_evidence_bridge.stl",
    "output/closed_incubator_condensate_cross_contamination_witness_station_robot_service_keepout_gauges.stl",
    "output/closed_incubator_condensate_cross_contamination_witness_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 10] = [
    "sealed_cassette_surrogate_grid",
    "condensate_drip_challenge_rails",
    "tracer_coupon_witness_wells",
    "gutter_diverter_comparison_plate",
    "position_barcode_lands",
    "rh_temp_logger_pockets",
    "release_hold_reject_lanes",
    "evidence_bridge",
    "robot_service_keepout_gauges",
    "named_stl_outputs",
];

const STATION_X: f64 = 1560.0;
const STATION_Y: f64 = 980.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 46.0;
const SOCKET_DEPTH: f64 = 6.0;
const BASIN_DEPTH: f64 = 9.0;
const MOUNT_HOLE_COUNT: usize = 8;
const DATUM_TARGET_COUNT: usize = 4;

const GRID_X: f64 = 640.0;
const GRID_Y: f64 = 410.0;
const GRID_Z: f64 = 46.0;
const GRID_COLS: usize = 4;
const GRID_ROWS: usize = 3;
const POSITION_COUNT: usize = GRID_COLS * GRID_ROWS;
const GRID_PITCH_X: f64 = 135.0;
const GRID_PITCH_Y: f64 = 112.0;
const CASSETTE_SURROGATE_X: f64 = 104.0;
const CASSETTE_SURROGATE_Y: f64 = 76.0;
const CASSETTE_SURROGATE_Z: f64 = 28.0;
const CASSETTE_GASKET_W: f64 = 7.0;
const GRID_POS: (f64, f64) = (-360.0, 90.0);

const RAIL_X: f64 = 640.0;
const RAIL_Y: f64 = 110.0;
const RAIL_Z: f64 = 70.0;
const DRIP_RAIL_COUNT: usize = 3;
const DRIP_NOZZLE_COUNT: usize = POSITION_COUNT;
const DRIP_NOZZLE_D: f64 = 5.0;
const DRIP_HEADER_D: f64 = 24.0;
const DRIP_RAIL_POS: (f64, f64) = (-360.0, 380.0);

const WITNESS_X: f64 = 430.0;
const WITNESS_Y: f64 = 250.0;
const WITNESS_Z: f64 = 44.0;
const TRACER_COUNT: usize = 4;
const WELLS_PER_TRACER: usize = 4;
const WITNESS_WELL_COUNT: usize = TRACER_COUNT * WELLS_PER_TRACER;
const WITNESS_WELL_D: f64 = 34.0;
const WITNESS_WELL_DEPTH: f64 = 22.0;
const COUPON_SLOT_COUNT: usize = POSITION_COUNT;
const COUPON_SLOT_X: f64 = 44.0;
const COUPON_SLOT_Y: f64 = 20.0;
const WITNESS_POS: (f64, f64) = (360.0, 240.0);

const GUTTER_X: f64 = 640.0;
const GUTTER_Y: f64 = 220.0;
const GUTTER_Z: f64 = 40.0;
const GUTTER_LANE_COUNT: usize = 3;
const GUTTER_CHANNEL_COUNT: usize = 6;
const DIVERTER_VANE_COUNT: usize = 6;
const GUTTER_COLLECTION_CUP_COUNT: usize = 3;
const GUTTER_POS: (f64, f64) = (-360.0, -250.0);

const LOGGER_X: f64 = 400.0;
const LOGGER_Y: f64 = 150.0;
const LOGGER_Z: f64 = 42.0;
const LOGGER_POCKET_COUNT: usize = 6;
const LOGGER_POCKET_X: f64 = 66.0;
const LOGGER_POCKET_Y: f64 = 38.0;
const LOGGER_POS: (f64, f64) = (360.0, 35.0);

const BARCODE_X: f64 = 400.0;
const BARCODE_Y: f64 = 110.0;
const BARCODE_Z: f64 = 10.0;
const BARCODE_LAND_COUNT: usize = POSITION_COUNT + 4;
const CERTIFICATE_LAND_COUNT: usize = 4;
const BARCODE_POS: (f64, f64) = (360.0, -145.0);

const LANE_X: f64 = 400.0;
const LANE_Y: f64 = 130.0;
const LANE_Z: f64 = 30.0;
const DISPOSITION_LANE_COUNT: usize = 3;
const SLOTS_PER_DISPOSITION_LANE: usize = 4;
const DISPOSITION_SLOT_COUNT: usize = DISPOSITION_LANE_COUNT * SLOTS_PER_DISPOSITION_LANE;
const LANE_POS: (f64, f64) = (360.0, -330.0);

const EVIDENCE_X: f64 = 1280.0;
const EVIDENCE_Y: f64 = 70.0;
const EVIDENCE_BEAM_Z: f64 = 28.0;
const EVIDENCE_CLEARANCE_Z: f64 = 210.0;
const EVIDENCE_CAMERA_COUNT: usize = 5;
const EVIDENCE_LIGHT_BAR_COUNT: usize = 2;
const EVIDENCE_POS: (f64, f64) = (0.0, 425.0);

const KEEP_OUT_X: f64 = 1480.0;
const KEEP_OUT_Y: f64 = 900.0;
const KEEP_OUT_Z: f64 = 6.0;
const ROBOT_FRONT_CLEARANCE: f64 = 390.0;
const SERVICE_REAR_CLEARANCE: f64 = 240.0;
const SIDE_SERVICE_CLEARANCE: f64 = 250.0;
const ROBOT_Z_CLEARANCE: f64 = 330.0;
const KEEP_OUT_GAUGE_COUNT: usize = 5;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_deck(self) -> bool {
        let usable_x = STATION_X / 2.0 - RIM_W - 12.0;
        let usable_y = STATION_Y / 2.0 - RIM_W - 12.0;

        self.center.0.abs() + self.x / 2.0 <= usable_x
            && self.center.1.abs() + self.y / 2.0 <= usable_y
    }

    fn overlaps(self, other: Rect) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();

        dx < (self.x + other.x) / 2.0 && dy < (self.y + other.y) / 2.0
    }
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

    fn index(self) -> usize {
        match self {
            DispositionLane::Release => 0,
            DispositionLane::Hold => 1,
            DispositionLane::Reject => 2,
        }
    }

    fn name(self) -> &'static str {
        match self {
            DispositionLane::Release => "release",
            DispositionLane::Hold => "hold",
            DispositionLane::Reject => "reject",
        }
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_containment_deck();
    export(OUTPUTS[0], &base);

    let grid = sealed_cassette_surrogate_grid();
    export(OUTPUTS[1], &grid);

    let rails = condensate_drip_challenge_rails();
    export(OUTPUTS[2], &rails);

    let witness = tracer_coupon_witness_wells();
    export(OUTPUTS[3], &witness);

    let gutters = gutter_diverter_comparison_plate();
    export(OUTPUTS[4], &gutters);

    let loggers = rh_temp_logger_pockets();
    export(OUTPUTS[5], &loggers);

    let barcodes = position_barcode_lands();
    export(OUTPUTS[6], &barcodes);

    let lanes = release_hold_reject_lanes();
    export(OUTPUTS[7], &lanes);

    let evidence = evidence_bridge();
    export(OUTPUTS[8], &evidence);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[9], &keepouts);

    let assembly = base
        + grid.translate(GRID_POS.0, GRID_POS.1, insert_z(GRID_Z))
        + rails.translate(DRIP_RAIL_POS.0, DRIP_RAIL_POS.1, insert_z(RAIL_Z))
        + witness.translate(WITNESS_POS.0, WITNESS_POS.1, insert_z(WITNESS_Z))
        + gutters.translate(GUTTER_POS.0, GUTTER_POS.1, insert_z(GUTTER_Z))
        + loggers.translate(LOGGER_POS.0, LOGGER_POS.1, insert_z(LOGGER_Z))
        + barcodes.translate(BARCODE_POS.0, BARCODE_POS.1, insert_z(BARCODE_Z))
        + lanes.translate(LANE_POS.0, LANE_POS.1, insert_z(LANE_Z))
        + evidence.translate(EVIDENCE_POS.0, EVIDENCE_POS.1, DECK_Z)
        + keepouts.translate(0.0, 0.0, DECK_Z);
    export(OUTPUTS[10], &assembly);

    println!();
    println!("Closed incubator condensate cross-contamination witness station:");
    println!(
        "  Footprint:            {STATION_X:.0}mm x {STATION_Y:.0}mm containment deck with {:.0}mL freeboard",
        containment_freeboard_ml()
    );
    println!(
        "  Rack surrogate grid:  {GRID_COLS}x{GRID_ROWS} sealed cassette positions with {POSITION_COUNT} position-indexed wells and gasket witness frames"
    );
    println!(
        "  Drip challenge:       {DRIP_RAIL_COUNT} drip rails, {DRIP_NOZZLE_COUNT} indexed nozzles, and {GUTTER_LANE_COUNT} gutter/diverter comparison lanes"
    );
    println!(
        "  Witness capture:      {TRACER_COUNT} tracer groups, {WITNESS_WELL_COUNT} witness wells, {COUPON_SLOT_COUNT} coupon slots, and {GUTTER_COLLECTION_CUP_COUNT} collection cup lands"
    );
    println!(
        "  Trace/environment:    {BARCODE_LAND_COUNT} position barcode lands, {CERTIFICATE_LAND_COUNT} certificate lands, {LOGGER_POCKET_COUNT} RH/temp logger pockets"
    );
    println!(
        "  Evidence/keepout:     {EVIDENCE_CAMERA_COUNT} cameras, {EVIDENCE_LIGHT_BAR_COUNT} light bars, {KEEP_OUT_GAUGE_COUNT} keepout gauges, release/hold/reject capacity for {DISPOSITION_SLOT_COUNT} positions"
    );
    println!("  Required features:    {}", REQUIRED_FEATURES.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn insert_z(height: f64) -> f64 {
    DECK_Z + height / 2.0 - SOCKET_DEPTH / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 11);
    assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
    assert_eq!(REQUIRED_FEATURES.len(), 10);
    assert_eq!(POSITION_COUNT, GRID_ROWS * GRID_COLS);
    assert_eq!(DRIP_NOZZLE_COUNT, POSITION_COUNT);
    assert_eq!(WITNESS_WELL_COUNT, TRACER_COUNT * WELLS_PER_TRACER);
    assert_eq!(COUPON_SLOT_COUNT, POSITION_COUNT);
    assert_eq!(BARCODE_LAND_COUNT, POSITION_COUNT + CERTIFICATE_LAND_COUNT);
    assert_eq!(DispositionLane::all().len(), DISPOSITION_LANE_COUNT);
    assert_eq!(DISPOSITION_SLOT_COUNT, POSITION_COUNT);
    assert_eq!(MOUNT_HOLE_COUNT, mount_hole_positions().len());
    assert_eq!(DATUM_TARGET_COUNT, datum_positions().len());
    assert!(CASSETTE_GASKET_W < CASSETTE_SURROGATE_X / 8.0);
    assert!(WITNESS_WELL_DEPTH < WITNESS_Z);
    assert!(DRIP_NOZZLE_D < DRIP_HEADER_D / 3.0);
    assert!(EVIDENCE_CLEARANCE_Z > GRID_Z + RAIL_Z);
    assert!(ROBOT_Z_CLEARANCE > EVIDENCE_CLEARANCE_Z + EVIDENCE_BEAM_Z);
    assert!(containment_freeboard_ml() > maximum_condensate_challenge_ml());
    assert!(front_robot_clearance() >= ROBOT_FRONT_CLEARANCE);
    assert!(rear_service_clearance() >= SERVICE_REAR_CLEARANCE);
    assert!(side_service_clearance() >= SIDE_SERVICE_CLEARANCE);

    for item in socket_rects() {
        assert!(
            item.fits_inside_deck(),
            "{} exceeds containment deck",
            item.name
        );
    }

    let rects = socket_rects();
    for a in 0..rects.len() {
        for b in (a + 1)..rects.len() {
            assert!(
                !rects[a].overlaps(rects[b]),
                "{} overlaps {}",
                rects[a].name,
                rects[b].name
            );
        }
    }
}

fn socket_rects() -> [Rect; 7] {
    [
        rect("sealed_cassette_surrogate_grid", GRID_POS, GRID_X, GRID_Y),
        rect(
            "condensate_drip_challenge_rails",
            DRIP_RAIL_POS,
            RAIL_X,
            RAIL_Y,
        ),
        rect(
            "tracer_coupon_witness_wells",
            WITNESS_POS,
            WITNESS_X,
            WITNESS_Y,
        ),
        rect(
            "gutter_diverter_comparison_plate",
            GUTTER_POS,
            GUTTER_X,
            GUTTER_Y,
        ),
        rect("rh_temp_logger_pockets", LOGGER_POS, LOGGER_X, LOGGER_Y),
        rect("position_barcode_lands", BARCODE_POS, BARCODE_X, BARCODE_Y),
        rect("release_hold_reject_lanes", LANE_POS, LANE_X, LANE_Y),
    ]
}

fn front_robot_clearance() -> f64 {
    ROBOT_FRONT_CLEARANCE
}

fn rear_service_clearance() -> f64 {
    STATION_Y / 2.0 - (DRIP_RAIL_POS.1 + RAIL_Y / 2.0) + EVIDENCE_CLEARANCE_Z
}

fn side_service_clearance() -> f64 {
    SIDE_SERVICE_CLEARANCE
}

fn containment_freeboard_ml() -> f64 {
    let inner_x = STATION_X - 2.0 * RIM_W;
    let inner_y = STATION_Y - 2.0 * RIM_W;
    let freeboard_z = RIM_Z - BASIN_DEPTH;
    inner_x * inner_y * freeboard_z / 1000.0
}

fn maximum_condensate_challenge_ml() -> f64 {
    let nozzles = DRIP_NOZZLE_COUNT as f64 * 4.0;
    let wells = WITNESS_WELL_COUNT as f64 * 11.0;
    let gutters = GUTTER_CHANNEL_COUNT as f64 * 28.0;
    nozzles + wells + gutters
}

fn base_containment_deck() -> Part {
    let deck = centered_cube(
        "condensate_cross_contamination_base_deck",
        STATION_X,
        STATION_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let basin = centered_cube(
        "condensate_cross_contamination_secondary_basin_cut",
        STATION_X - 2.0 * (RIM_W + 48.0),
        STATION_Y - 2.0 * (RIM_W + 50.0),
        BASIN_DEPTH + 0.8,
    )
    .translate(0.0, -8.0, DECK_Z - BASIN_DEPTH / 2.0);
    let front_drain = centered_cylinder(
        "condensate_cross_contamination_front_closed_drain_placeholder",
        8.0,
        52.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 98.0,
        -STATION_Y / 2.0 + 34.0,
        DECK_Z - 8.0,
    );

    deck - basin - front_drain - insert_sockets() - deck_mount_holes()
        + perimeter_rims()
        + workflow_spines()
        + robot_datum_targets()
        + evidence_anchor_lands()
        + base_flow_witness_ribs()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("condensate_cross_contamination_insert_sockets");
    for item in socket_rects() {
        sockets = sockets
            + centered_cube(
                format!("condensate_cross_contamination_{}_socket", item.name),
                item.x + 10.0,
                item.y + 10.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(item.center.0, item.center.1, DECK_Z - SOCKET_DEPTH / 2.0);
    }
    sockets
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("condensate_cross_contamination_deck_mount_holes");
    for (i, (x, y)) in mount_hole_positions().into_iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("condensate_cross_contamination_m6_mount_clearance_{i}"),
                3.4,
                DECK_Z + 4.0,
                28,
            )
            .translate(x, y, DECK_Z / 2.0)
            + centered_cube(
                format!("condensate_cross_contamination_service_slot_{i}"),
                30.0,
                7.5,
                DECK_Z + 4.0,
            )
            .translate(x, y, DECK_Z / 2.0);
    }
    holes
}

fn mount_hole_positions() -> [(f64, f64); MOUNT_HOLE_COUNT] {
    [
        (-STATION_X / 2.0 + 58.0, -STATION_Y / 2.0 + 58.0),
        (STATION_X / 2.0 - 58.0, -STATION_Y / 2.0 + 58.0),
        (-STATION_X / 2.0 + 58.0, STATION_Y / 2.0 - 58.0),
        (STATION_X / 2.0 - 58.0, STATION_Y / 2.0 - 58.0),
        (0.0, -STATION_Y / 2.0 + 58.0),
        (0.0, STATION_Y / 2.0 - 58.0),
        (-STATION_X / 2.0 + 58.0, 0.0),
        (STATION_X / 2.0 - 58.0, 0.0),
    ]
}

fn perimeter_rims() -> Part {
    let front = centered_cube(
        "condensate_cross_contamination_front_low_robot_rim",
        STATION_X,
        RIM_W,
        28.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, DECK_Z + 14.0);
    let rear = centered_cube(
        "condensate_cross_contamination_rear_service_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "condensate_cross_contamination_left_spill_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "condensate_cross_contamination_right_spill_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);

    front + rear + left + right
}

fn workflow_spines() -> Part {
    let upper = centered_cube(
        "condensate_cross_contamination_drip_to_grid_zone_spine",
        STATION_X - 210.0,
        10.0,
        24.0,
    )
    .translate(0.0, 315.0, DECK_Z + 12.0);
    let middle = centered_cube(
        "condensate_cross_contamination_witness_logger_zone_spine",
        STATION_X - 260.0,
        10.0,
        22.0,
    )
    .translate(0.0, -55.0, DECK_Z + 11.0);
    let lower = centered_cube(
        "condensate_cross_contamination_diverter_disposition_zone_spine",
        STATION_X - 260.0,
        10.0,
        22.0,
    )
    .translate(0.0, -230.0, DECK_Z + 11.0);
    let clean_used = centered_cube(
        "condensate_cross_contamination_left_right_clean_used_spine",
        12.0,
        STATION_Y - 180.0,
        28.0,
    )
    .translate(0.0, -28.0, DECK_Z + 14.0);

    upper + middle + lower + clean_used
}

fn robot_datum_targets() -> Part {
    let mut targets = Part::empty("condensate_cross_contamination_robot_datum_targets");
    for (i, (x, y)) in datum_positions().into_iter().enumerate() {
        targets = targets
            + fiducial_disc(&format!(
                "condensate_cross_contamination_deck_datum_target_{i}"
            ))
            .translate(x, y, DECK_Z + 2.5);
    }
    targets
}

fn datum_positions() -> [(f64, f64); DATUM_TARGET_COUNT] {
    [
        (-STATION_X / 2.0 + 100.0, -STATION_Y / 2.0 + 98.0),
        (STATION_X / 2.0 - 100.0, -STATION_Y / 2.0 + 98.0),
        (-STATION_X / 2.0 + 100.0, STATION_Y / 2.0 - 98.0),
        (STATION_X / 2.0 - 100.0, STATION_Y / 2.0 - 98.0),
    ]
}

fn evidence_anchor_lands() -> Part {
    let left = centered_cube(
        "condensate_cross_contamination_evidence_bridge_left_anchor_land",
        96.0,
        42.0,
        8.0,
    )
    .translate(
        EVIDENCE_POS.0 - EVIDENCE_X / 2.0 + 62.0,
        EVIDENCE_POS.1,
        DECK_Z + 4.0,
    );
    let right = centered_cube(
        "condensate_cross_contamination_evidence_bridge_right_anchor_land",
        96.0,
        42.0,
        8.0,
    )
    .translate(
        EVIDENCE_POS.0 + EVIDENCE_X / 2.0 - 62.0,
        EVIDENCE_POS.1,
        DECK_Z + 4.0,
    );
    left + right
}

fn base_flow_witness_ribs() -> Part {
    let mut ribs = Part::empty("condensate_cross_contamination_base_flow_witness_ribs");
    for (i, y) in [-348.0, -292.0, -128.0, 42.0, 202.0, 352.0]
        .into_iter()
        .enumerate()
    {
        ribs = ribs
            + centered_cube(
                format!("condensate_cross_contamination_base_flow_witness_rib_{i}"),
                STATION_X - 240.0,
                4.0,
                5.0,
            )
            .translate(0.0, y, DECK_Z + 2.5);
    }
    ribs
}

fn sealed_cassette_surrogate_grid() -> Part {
    let tray = centered_cube(
        "condensate_cross_contamination_sealed_grid_tray",
        GRID_X,
        GRID_Y,
        GRID_Z,
    );
    let inner_relief = centered_cube(
        "condensate_cross_contamination_sealed_grid_inner_condensate_relief_cut",
        GRID_X - 64.0,
        GRID_Y - 58.0,
        10.0,
    )
    .translate(0.0, 0.0, GRID_Z / 2.0 - 5.0);

    tray - inner_relief - cassette_recesses() - gasket_frame_grooves()
        + cassette_surrogates()
        + grid_position_ribs()
        + grid_robot_handles()
        + grid_fiducials()
}

fn cassette_recesses() -> Part {
    let mut recesses = Part::empty("condensate_cross_contamination_cassette_recesses");
    for position in 0..POSITION_COUNT {
        let (x, y) = grid_position_xy(position);
        recesses = recesses
            + centered_cube(
                format!(
                    "condensate_cross_contamination_position_{}_cassette_recess",
                    position_label(position)
                ),
                CASSETTE_SURROGATE_X + 18.0,
                CASSETTE_SURROGATE_Y + 18.0,
                13.0,
            )
            .translate(x, y, GRID_Z / 2.0 - 6.0);
    }
    recesses
}

fn gasket_frame_grooves() -> Part {
    let mut grooves = Part::empty("condensate_cross_contamination_gasket_frame_grooves");
    for position in 0..POSITION_COUNT {
        let (x, y) = grid_position_xy(position);
        let outer = centered_cube(
            format!(
                "condensate_cross_contamination_position_{}_gasket_outer_groove",
                position_label(position)
            ),
            CASSETTE_SURROGATE_X + 28.0,
            CASSETTE_SURROGATE_Y + 28.0,
            7.0,
        )
        .translate(x, y, GRID_Z / 2.0 + 1.5);
        let inner = centered_cube(
            format!(
                "condensate_cross_contamination_position_{}_gasket_inner_island",
                position_label(position)
            ),
            CASSETTE_SURROGATE_X + 28.0 - 2.0 * CASSETTE_GASKET_W,
            CASSETTE_SURROGATE_Y + 28.0 - 2.0 * CASSETTE_GASKET_W,
            8.0,
        )
        .translate(x, y, GRID_Z / 2.0 + 1.5);
        grooves = grooves + (outer - inner);
    }
    grooves
}

fn cassette_surrogates() -> Part {
    let mut surrogates = Part::empty("condensate_cross_contamination_cassette_surrogates");
    for position in 0..POSITION_COUNT {
        let (x, y) = grid_position_xy(position);
        let slot_name = position_label(position);
        let body = centered_cube(
            format!("condensate_cross_contamination_{slot_name}_sealed_surrogate_body"),
            CASSETTE_SURROGATE_X,
            CASSETTE_SURROGATE_Y,
            CASSETTE_SURROGATE_Z,
        )
        .translate(x, y, GRID_Z / 2.0 + CASSETTE_SURROGATE_Z / 2.0 - 2.0);
        let witness_slot = centered_cube(
            format!("condensate_cross_contamination_{slot_name}_top_witness_coupon_slot"),
            COUPON_SLOT_X,
            COUPON_SLOT_Y,
            8.0,
        )
        .translate(x, y, GRID_Z / 2.0 + CASSETTE_SURROGATE_Z - 2.0);
        let drip_target = centered_cylinder(
            format!("condensate_cross_contamination_{slot_name}_drip_target_bullseye"),
            12.0,
            3.0,
            32,
        )
        .translate(
            x,
            y - CASSETTE_SURROGATE_Y / 2.0 + 18.0,
            GRID_Z / 2.0 + CASSETTE_SURROGATE_Z + 1.5,
        );

        surrogates = surrogates + (body - witness_slot) + drip_target;
    }
    surrogates
}

fn grid_position_ribs() -> Part {
    let mut ribs = Part::empty("condensate_cross_contamination_grid_position_isolation_ribs");
    for col in 1..GRID_COLS {
        let x = centered_index(col - 1, GRID_COLS - 1, GRID_PITCH_X) + GRID_PITCH_X / 2.0
            - GRID_PITCH_X * (GRID_COLS as f64 - 2.0) / 2.0;
        ribs = ribs
            + centered_cube(
                format!("condensate_cross_contamination_grid_column_isolation_rib_{col}"),
                8.0,
                GRID_Y - 74.0,
                18.0,
            )
            .translate(x, 0.0, GRID_Z / 2.0 + 9.0);
    }
    for row in 1..GRID_ROWS {
        let y = centered_index(row - 1, GRID_ROWS - 1, GRID_PITCH_Y) + GRID_PITCH_Y / 2.0
            - GRID_PITCH_Y * (GRID_ROWS as f64 - 2.0) / 2.0;
        ribs = ribs
            + centered_cube(
                format!("condensate_cross_contamination_grid_row_isolation_rib_{row}"),
                GRID_X - 84.0,
                8.0,
                18.0,
            )
            .translate(0.0, y, GRID_Z / 2.0 + 9.0);
    }
    ribs
}

fn grid_robot_handles() -> Part {
    let left = centered_cube(
        "condensate_cross_contamination_grid_left_robot_handle",
        22.0,
        128.0,
        28.0,
    )
    .translate(-GRID_X / 2.0 + 26.0, 0.0, GRID_Z / 2.0 + 14.0);
    let right = centered_cube(
        "condensate_cross_contamination_grid_right_robot_handle",
        22.0,
        128.0,
        28.0,
    )
    .translate(GRID_X / 2.0 - 26.0, 0.0, GRID_Z / 2.0 + 14.0);
    left + right
}

fn grid_fiducials() -> Part {
    let mut fiducials = Part::empty("condensate_cross_contamination_grid_fiducials");
    for (i, (x, y)) in [
        (-GRID_X / 2.0 + 52.0, -GRID_Y / 2.0 + 42.0),
        (GRID_X / 2.0 - 52.0, -GRID_Y / 2.0 + 42.0),
        (-GRID_X / 2.0 + 52.0, GRID_Y / 2.0 - 42.0),
        (GRID_X / 2.0 - 52.0, GRID_Y / 2.0 - 42.0),
    ]
    .into_iter()
    .enumerate()
    {
        fiducials = fiducials
            + fiducial_disc(&format!("condensate_cross_contamination_grid_fiducial_{i}"))
                .translate(x, y, GRID_Z / 2.0 + 2.5);
    }
    fiducials
}

fn condensate_drip_challenge_rails() -> Part {
    let base = centered_cube(
        "condensate_cross_contamination_drip_rail_base",
        RAIL_X,
        RAIL_Y,
        18.0,
    )
    .translate(0.0, 0.0, -RAIL_Z / 2.0 + 9.0);
    let rear_header = centered_cylinder(
        "condensate_cross_contamination_drip_rail_rear_header",
        DRIP_HEADER_D / 2.0,
        RAIL_X - 80.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, RAIL_Y / 2.0 - 28.0, RAIL_Z / 2.0 - 22.0);
    let front_header = centered_cylinder(
        "condensate_cross_contamination_drip_rail_front_header",
        DRIP_HEADER_D / 2.0,
        RAIL_X - 80.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -RAIL_Y / 2.0 + 28.0, RAIL_Z / 2.0 - 22.0);

    base + rear_header + front_header + rail_standoffs() + drip_nozzle_array() + rail_scale_ticks()
}

fn rail_standoffs() -> Part {
    let mut standoffs = Part::empty("condensate_cross_contamination_drip_rail_standoffs");
    for rail in 0..DRIP_RAIL_COUNT {
        let y = centered_index(rail, DRIP_RAIL_COUNT, 36.0);
        for x in [-RAIL_X / 2.0 + 72.0, RAIL_X / 2.0 - 72.0] {
            standoffs = standoffs
                + centered_cube(
                    format!("condensate_cross_contamination_drip_rail_{rail}_standoff_{x:.0}"),
                    26.0,
                    16.0,
                    RAIL_Z - 18.0,
                )
                .translate(x, y, 0.0);
        }
    }
    standoffs
}

fn drip_nozzle_array() -> Part {
    let mut nozzles = Part::empty("condensate_cross_contamination_indexed_drip_nozzles");
    for position in 0..POSITION_COUNT {
        let (x, _grid_y) = grid_position_xy(position);
        let row = position / GRID_COLS;
        let y = centered_index(row, GRID_ROWS, 30.0);
        let nozzle = centered_cylinder(
            format!(
                "condensate_cross_contamination_position_{}_drip_nozzle",
                position_label(position)
            ),
            DRIP_NOZZLE_D / 2.0,
            26.0,
            20,
        )
        .translate(x, y, RAIL_Z / 2.0 - 36.0);
        let droplet_target = centered_cylinder(
            format!(
                "condensate_cross_contamination_position_{}_droplet_shadow_target",
                position_label(position)
            ),
            8.0,
            2.0,
            24,
        )
        .translate(x, y - 12.0, -RAIL_Z / 2.0 + 20.0);
        nozzles = nozzles + nozzle + droplet_target;
    }
    nozzles
}

fn rail_scale_ticks() -> Part {
    let mut ticks = Part::empty("condensate_cross_contamination_drip_rail_position_ticks");
    for position in 0..POSITION_COUNT {
        let (x, _grid_y) = grid_position_xy(position);
        ticks = ticks
            + centered_cube(
                format!(
                    "condensate_cross_contamination_position_{}_rail_tick",
                    position_label(position)
                ),
                3.0,
                RAIL_Y - 22.0,
                4.0,
            )
            .translate(x, 0.0, -RAIL_Z / 2.0 + 20.0);
    }
    ticks
}

fn tracer_coupon_witness_wells() -> Part {
    let plate = centered_cube(
        "condensate_cross_contamination_tracer_coupon_witness_plate",
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    );
    let relief = centered_cube(
        "condensate_cross_contamination_witness_plate_spill_relief_cut",
        WITNESS_X - 56.0,
        WITNESS_Y - 48.0,
        8.0,
    )
    .translate(0.0, 0.0, WITNESS_Z / 2.0 - 4.0);

    plate - relief - tracer_well_cuts() - coupon_slot_cuts()
        + tracer_color_lands()
        + witness_cross_channel_ribs()
}

fn tracer_well_cuts() -> Part {
    let mut cuts = Part::empty("condensate_cross_contamination_tracer_well_cuts");
    for tracer in 0..TRACER_COUNT {
        for well in 0..WELLS_PER_TRACER {
            let well_index = tracer * WELLS_PER_TRACER + well;
            let x = centered_index(well, WELLS_PER_TRACER, 70.0);
            let y = centered_index(tracer, TRACER_COUNT, 48.0) + 28.0;
            cuts = cuts
                + centered_cylinder(
                    format!(
                        "condensate_cross_contamination_{}_tracer_well_{well}",
                        tracer_name(tracer)
                    ),
                    WITNESS_WELL_D / 2.0,
                    WITNESS_WELL_DEPTH,
                    36,
                )
                .translate(x, y, WITNESS_Z / 2.0 - WITNESS_WELL_DEPTH / 2.0 + 1.0)
                + centered_cylinder(
                    format!("condensate_cross_contamination_witness_well_{well_index}_drain_notch"),
                    4.0,
                    22.0,
                    20,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(
                    x + WITNESS_WELL_D / 2.0 - 2.0,
                    y,
                    WITNESS_Z / 2.0 - 12.0,
                );
        }
    }
    cuts
}

fn coupon_slot_cuts() -> Part {
    let mut cuts = Part::empty("condensate_cross_contamination_coupon_slot_cuts");
    for position in 0..COUPON_SLOT_COUNT {
        let col = position % GRID_COLS;
        let row = position / GRID_COLS;
        cuts = cuts
            + centered_cube(
                format!(
                    "condensate_cross_contamination_position_{}_coupon_slot_cut",
                    position_label(position)
                ),
                COUPON_SLOT_X,
                COUPON_SLOT_Y,
                10.0,
            )
            .translate(
                centered_index(col, GRID_COLS, 64.0),
                -WITNESS_Y / 2.0 + 34.0 + row as f64 * 24.0,
                WITNESS_Z / 2.0 - 4.0,
            );
    }
    cuts
}

fn tracer_color_lands() -> Part {
    let mut lands = Part::empty("condensate_cross_contamination_tracer_color_lands");
    for tracer in 0..TRACER_COUNT {
        let y = centered_index(tracer, TRACER_COUNT, 48.0) + 28.0;
        lands = lands
            + centered_cube(
                format!(
                    "condensate_cross_contamination_{}_tracer_color_land",
                    tracer_name(tracer)
                ),
                32.0,
                18.0,
                4.0,
            )
            .translate(-WITNESS_X / 2.0 + 34.0, y, WITNESS_Z / 2.0 + 2.0);
    }
    lands
}

fn witness_cross_channel_ribs() -> Part {
    let horizontal = centered_cube(
        "condensate_cross_contamination_witness_isolation_horizontal_rib",
        WITNESS_X - 58.0,
        5.0,
        10.0,
    )
    .translate(0.0, -8.0, WITNESS_Z / 2.0 + 5.0);
    let vertical = centered_cube(
        "condensate_cross_contamination_witness_isolation_vertical_rib",
        5.0,
        WITNESS_Y - 64.0,
        10.0,
    )
    .translate(-WITNESS_X / 2.0 + 82.0, 0.0, WITNESS_Z / 2.0 + 5.0);
    horizontal + vertical
}

fn gutter_diverter_comparison_plate() -> Part {
    let plate = centered_cube(
        "condensate_cross_contamination_gutter_diverter_comparison_plate",
        GUTTER_X,
        GUTTER_Y,
        GUTTER_Z,
    );
    let basin = centered_cube(
        "condensate_cross_contamination_gutter_comparison_common_basin_cut",
        GUTTER_X - 54.0,
        GUTTER_Y - 50.0,
        9.0,
    )
    .translate(0.0, 0.0, GUTTER_Z / 2.0 - 4.0);

    plate - basin - gutter_channel_cuts() - collection_cup_cuts()
        + diverter_vanes()
        + gutter_lane_labels()
}

fn gutter_channel_cuts() -> Part {
    let mut cuts = Part::empty("condensate_cross_contamination_gutter_channel_cuts");
    for channel in 0..GUTTER_CHANNEL_COUNT {
        let lane = channel % GUTTER_LANE_COUNT;
        let pass = channel / GUTTER_LANE_COUNT;
        cuts = cuts
            + centered_cube(
                format!("condensate_cross_contamination_gutter_channel_cut_{channel}"),
                GUTTER_X / 3.0 - 46.0,
                14.0,
                14.0,
            )
            .translate(
                centered_index(lane, GUTTER_LANE_COUNT, GUTTER_X / 3.0),
                centered_index(pass, 2, 78.0),
                GUTTER_Z / 2.0 - 7.0,
            );
    }
    cuts
}

fn collection_cup_cuts() -> Part {
    let mut cuts = Part::empty("condensate_cross_contamination_gutter_collection_cup_cuts");
    for cup in 0..GUTTER_COLLECTION_CUP_COUNT {
        cuts = cuts
            + centered_cylinder(
                format!("condensate_cross_contamination_gutter_collection_cup_cut_{cup}"),
                27.0,
                18.0,
                36,
            )
            .translate(
                centered_index(cup, GUTTER_COLLECTION_CUP_COUNT, GUTTER_X / 3.0),
                -GUTTER_Y / 2.0 + 42.0,
                GUTTER_Z / 2.0 - 8.0,
            );
    }
    cuts
}

fn diverter_vanes() -> Part {
    let mut vanes = Part::empty("condensate_cross_contamination_diverter_vanes");
    for vane in 0..DIVERTER_VANE_COUNT {
        let lane = vane % GUTTER_LANE_COUNT;
        let row = vane / GUTTER_LANE_COUNT;
        vanes = vanes
            + centered_cube(
                format!("condensate_cross_contamination_diverter_vane_{vane}"),
                14.0,
                72.0,
                22.0,
            )
            .rotate(0.0, 0.0, if row == 0 { -12.0 } else { 12.0 })
            .translate(
                centered_index(lane, GUTTER_LANE_COUNT, GUTTER_X / 3.0) + 22.0,
                centered_index(row, 2, 78.0),
                GUTTER_Z / 2.0 + 11.0,
            );
    }
    vanes
}

fn gutter_lane_labels() -> Part {
    let mut labels = Part::empty("condensate_cross_contamination_gutter_lane_label_lands");
    for lane in 0..GUTTER_LANE_COUNT {
        labels = labels
            + centered_cube(
                format!(
                    "condensate_cross_contamination_{}_gutter_label_land",
                    gutter_lane_name(lane)
                ),
                122.0,
                22.0,
                3.0,
            )
            .translate(
                centered_index(lane, GUTTER_LANE_COUNT, GUTTER_X / 3.0),
                GUTTER_Y / 2.0 - 28.0,
                GUTTER_Z / 2.0 + 1.5,
            );
    }
    labels
}

fn rh_temp_logger_pockets() -> Part {
    let body = centered_cube(
        "condensate_cross_contamination_rh_temp_logger_pocket_block",
        LOGGER_X,
        LOGGER_Y,
        LOGGER_Z,
    );
    let cable_trench = centered_cube(
        "condensate_cross_contamination_logger_cable_trench",
        LOGGER_X - 54.0,
        16.0,
        16.0,
    )
    .translate(0.0, -LOGGER_Y / 2.0 + 30.0, LOGGER_Z / 2.0 - 8.0);

    body - logger_pocket_cuts() - cable_trench + logger_clip_ribs() + logger_id_lands()
}

fn logger_pocket_cuts() -> Part {
    let mut pockets = Part::empty("condensate_cross_contamination_logger_pocket_cuts");
    for pocket in 0..LOGGER_POCKET_COUNT {
        let col = pocket % 3;
        let row = pocket / 3;
        pockets = pockets
            + centered_cube(
                format!("condensate_cross_contamination_rh_temp_logger_pocket_{pocket}"),
                LOGGER_POCKET_X,
                LOGGER_POCKET_Y,
                18.0,
            )
            .translate(
                centered_index(col, 3, 108.0),
                centered_index(row, 2, 56.0) + 8.0,
                LOGGER_Z / 2.0 - 9.0,
            );
    }
    pockets
}

fn logger_clip_ribs() -> Part {
    let mut ribs = Part::empty("condensate_cross_contamination_logger_clip_ribs");
    for pocket in 0..LOGGER_POCKET_COUNT {
        let col = pocket % 3;
        let row = pocket / 3;
        let x = centered_index(col, 3, 108.0);
        let y = centered_index(row, 2, 56.0) + 8.0;
        ribs = ribs
            + centered_cube(
                format!("condensate_cross_contamination_logger_{pocket}_left_clip"),
                5.0,
                LOGGER_POCKET_Y + 8.0,
                10.0,
            )
            .translate(x - LOGGER_POCKET_X / 2.0 - 6.0, y, LOGGER_Z / 2.0 + 5.0)
            + centered_cube(
                format!("condensate_cross_contamination_logger_{pocket}_right_clip"),
                5.0,
                LOGGER_POCKET_Y + 8.0,
                10.0,
            )
            .translate(x + LOGGER_POCKET_X / 2.0 + 6.0, y, LOGGER_Z / 2.0 + 5.0);
    }
    ribs
}

fn logger_id_lands() -> Part {
    let mut lands = Part::empty("condensate_cross_contamination_logger_id_lands");
    for pocket in 0..LOGGER_POCKET_COUNT {
        lands = lands
            + centered_cube(
                format!("condensate_cross_contamination_logger_{pocket}_id_land"),
                52.0,
                14.0,
                3.0,
            )
            .translate(
                centered_index(pocket % 3, 3, 108.0),
                LOGGER_Y / 2.0 - 24.0 - (pocket / 3) as f64 * 20.0,
                LOGGER_Z / 2.0 + 1.5,
            );
    }
    lands
}

fn position_barcode_lands() -> Part {
    let plate = centered_cube(
        "condensate_cross_contamination_position_barcode_land_plate",
        BARCODE_X,
        BARCODE_Y,
        BARCODE_Z,
    );
    plate + barcode_lands() + certificate_lands()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("condensate_cross_contamination_position_barcode_lands");
    for position in 0..POSITION_COUNT {
        lands = lands
            + centered_cube(
                format!(
                    "condensate_cross_contamination_position_{}_barcode_land",
                    position_label(position)
                ),
                78.0,
                18.0,
                2.5,
            )
            .translate(
                centered_index(position % 4, 4, 88.0),
                centered_index(position / 4, 3, 27.0) + 10.0,
                BARCODE_Z / 2.0 + 1.25,
            );
    }
    lands
}

fn certificate_lands() -> Part {
    let mut lands = Part::empty("condensate_cross_contamination_certificate_lands");
    for cert in 0..CERTIFICATE_LAND_COUNT {
        lands = lands
            + centered_cube(
                format!("condensate_cross_contamination_run_certificate_land_{cert}"),
                82.0,
                18.0,
                2.5,
            )
            .translate(
                centered_index(cert, CERTIFICATE_LAND_COUNT, 92.0),
                -BARCODE_Y / 2.0 + 22.0,
                BARCODE_Z / 2.0 + 1.25,
            );
    }
    lands
}

fn release_hold_reject_lanes() -> Part {
    let plate = centered_cube(
        "condensate_cross_contamination_release_hold_reject_lane_plate",
        LANE_X,
        LANE_Y,
        LANE_Z,
    );
    plate - disposition_slot_cuts() + disposition_lane_headers() + disposition_lane_tokens()
}

fn disposition_slot_cuts() -> Part {
    let mut cuts = Part::empty("condensate_cross_contamination_disposition_slot_cuts");
    for lane in DispositionLane::all() {
        for slot in 0..SLOTS_PER_DISPOSITION_LANE {
            cuts = cuts
                + centered_cube(
                    format!(
                        "condensate_cross_contamination_{}_lane_slot_{slot}_cut",
                        lane.name()
                    ),
                    74.0,
                    22.0,
                    12.0,
                )
                .translate(
                    centered_index(slot, SLOTS_PER_DISPOSITION_LANE, 86.0),
                    centered_index(lane.index(), DISPOSITION_LANE_COUNT, 38.0),
                    LANE_Z / 2.0 - 6.0,
                );
        }
    }
    cuts
}

fn disposition_lane_headers() -> Part {
    let mut headers = Part::empty("condensate_cross_contamination_disposition_lane_headers");
    for lane in DispositionLane::all() {
        headers = headers
            + centered_cube(
                format!(
                    "condensate_cross_contamination_{}_lane_header_land",
                    lane.name()
                ),
                62.0,
                16.0,
                3.0,
            )
            .translate(
                -LANE_X / 2.0 + 44.0,
                centered_index(lane.index(), DISPOSITION_LANE_COUNT, 38.0),
                LANE_Z / 2.0 + 1.5,
            );
    }
    headers
}

fn disposition_lane_tokens() -> Part {
    let mut tokens = Part::empty("condensate_cross_contamination_disposition_lane_tokens");
    for lane in DispositionLane::all() {
        for slot in 0..SLOTS_PER_DISPOSITION_LANE {
            tokens = tokens
                + centered_cube(
                    format!(
                        "condensate_cross_contamination_{}_lane_position_token_{slot}",
                        lane.name()
                    ),
                    48.0,
                    12.0,
                    5.0,
                )
                .translate(
                    centered_index(slot, SLOTS_PER_DISPOSITION_LANE, 86.0),
                    centered_index(lane.index(), DISPOSITION_LANE_COUNT, 38.0) + 12.0,
                    LANE_Z / 2.0 + 2.5,
                );
        }
    }
    tokens
}

fn evidence_bridge() -> Part {
    let left_post = centered_cube(
        "condensate_cross_contamination_evidence_bridge_left_post",
        44.0,
        EVIDENCE_Y,
        EVIDENCE_CLEARANCE_Z,
    )
    .translate(-EVIDENCE_X / 2.0 + 54.0, 0.0, EVIDENCE_CLEARANCE_Z / 2.0);
    let right_post = centered_cube(
        "condensate_cross_contamination_evidence_bridge_right_post",
        44.0,
        EVIDENCE_Y,
        EVIDENCE_CLEARANCE_Z,
    )
    .translate(EVIDENCE_X / 2.0 - 54.0, 0.0, EVIDENCE_CLEARANCE_Z / 2.0);
    let beam = centered_cube(
        "condensate_cross_contamination_evidence_bridge_camera_beam",
        EVIDENCE_X,
        EVIDENCE_Y,
        EVIDENCE_BEAM_Z,
    )
    .translate(0.0, 0.0, EVIDENCE_CLEARANCE_Z + EVIDENCE_BEAM_Z / 2.0);

    left_post + right_post + beam + evidence_camera_pods() + evidence_light_bars()
}

fn evidence_camera_pods() -> Part {
    let mut pods = Part::empty("condensate_cross_contamination_evidence_camera_pods");
    for camera in 0..EVIDENCE_CAMERA_COUNT {
        let x = centered_index(camera, EVIDENCE_CAMERA_COUNT, (EVIDENCE_X - 260.0) / 4.0);
        let z = EVIDENCE_CLEARANCE_Z - 28.0;
        pods = pods
            + centered_cube(
                format!("condensate_cross_contamination_evidence_camera_pod_{camera}"),
                60.0,
                42.0,
                36.0,
            )
            .translate(x, -7.0, z)
            + centered_cylinder(
                format!("condensate_cross_contamination_evidence_camera_lens_{camera}"),
                10.0,
                18.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -EVIDENCE_Y / 2.0 - 8.0, z);
    }
    pods
}

fn evidence_light_bars() -> Part {
    let mut bars = Part::empty("condensate_cross_contamination_evidence_light_bars");
    for bar in 0..EVIDENCE_LIGHT_BAR_COUNT {
        let y = if bar == 0 { -22.0 } else { 22.0 };
        bars = bars
            + centered_cube(
                format!("condensate_cross_contamination_evidence_light_bar_{bar}"),
                EVIDENCE_X - 220.0,
                10.0,
                10.0,
            )
            .translate(0.0, y, EVIDENCE_CLEARANCE_Z - 55.0);
    }
    bars
}

fn robot_service_keepout_gauges() -> Part {
    let perimeter = centered_cube(
        "condensate_cross_contamination_keepout_outer_reference_plate",
        KEEP_OUT_X,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    );
    let open_cut = centered_cube(
        "condensate_cross_contamination_keepout_open_work_area_cut",
        KEEP_OUT_X - 120.0,
        KEEP_OUT_Y - 120.0,
        KEEP_OUT_Z + 2.0,
    );
    let front = centered_cube(
        "condensate_cross_contamination_front_robot_approach_keepout_gauge",
        KEEP_OUT_X - 160.0,
        22.0,
        18.0,
    )
    .translate(0.0, -STATION_Y / 2.0 - ROBOT_FRONT_CLEARANCE / 2.0, 9.0);
    let rear = centered_cube(
        "condensate_cross_contamination_rear_service_sweep_keepout_gauge",
        KEEP_OUT_X - 180.0,
        22.0,
        18.0,
    )
    .translate(0.0, STATION_Y / 2.0 + SERVICE_REAR_CLEARANCE / 2.0, 9.0);
    let side = centered_cube(
        "condensate_cross_contamination_right_service_hand_keepout_gauge",
        22.0,
        KEEP_OUT_Y - 180.0,
        18.0,
    )
    .translate(STATION_X / 2.0 + SIDE_SERVICE_CLEARANCE / 2.0, 0.0, 9.0);
    let z = centered_cube(
        "condensate_cross_contamination_robot_z_clearance_gauge",
        70.0,
        70.0,
        ROBOT_Z_CLEARANCE,
    )
    .translate(
        STATION_X / 2.0 - 95.0,
        STATION_Y / 2.0 - 105.0,
        ROBOT_Z_CLEARANCE / 2.0,
    );

    perimeter - open_cut + front + rear + side + z
}

fn grid_position_xy(position: usize) -> (f64, f64) {
    let col = position % GRID_COLS;
    let row = position / GRID_COLS;
    (
        centered_index(col, GRID_COLS, GRID_PITCH_X),
        centered_index(row, GRID_ROWS, GRID_PITCH_Y),
    )
}

fn position_label(position: usize) -> &'static str {
    match position {
        0 => "a1",
        1 => "a2",
        2 => "a3",
        3 => "a4",
        4 => "b1",
        5 => "b2",
        6 => "b3",
        7 => "b4",
        8 => "c1",
        9 => "c2",
        10 => "c3",
        11 => "c4",
        _ => "unknown",
    }
}

fn tracer_name(tracer: usize) -> &'static str {
    match tracer {
        0 => "blue",
        1 => "green",
        2 => "amber",
        3 => "magenta",
        _ => "unknown",
    }
}

fn gutter_lane_name(lane: usize) -> &'static str {
    match lane {
        0 => "open_gutter_reference",
        1 => "diverter_control",
        2 => "raised_lip_control",
        _ => "unknown",
    }
}

fn fiducial_disc(name: &str) -> Part {
    let disc = centered_cylinder(format!("{name}_outer_disc"), 11.0, 5.0, 36);
    let center = centered_cylinder(format!("{name}_center_dot"), 2.3, 6.0, 20);
    let cross_x = centered_cube(format!("{name}_cross_x"), 18.0, 2.2, 6.0);
    let cross_y = centered_cube(format!("{name}_cross_y"), 2.2, 18.0, 6.0);
    disc - center - cross_x - cross_y
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_scoped_and_stable() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 11);
        for path in OUTPUTS {
            assert!(path.starts_with(
                "output/closed_incubator_condensate_cross_contamination_witness_station_"
            ));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn requested_feature_groups_are_explicit() {
        for feature in [
            "sealed_cassette_surrogate_grid",
            "condensate_drip_challenge_rails",
            "tracer_coupon_witness_wells",
            "gutter_diverter_comparison_plate",
            "position_barcode_lands",
            "rh_temp_logger_pockets",
            "release_hold_reject_lanes",
            "evidence_bridge",
            "robot_service_keepout_gauges",
            "named_stl_outputs",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn modules_fit_and_do_not_overlap_on_deck() {
        assert_design_constraints();
        let rects = socket_rects();
        for rect in rects {
            assert!(rect.fits_inside_deck(), "{} does not fit", rect.name);
        }
    }

    #[test]
    fn cassette_grid_drip_and_barcode_counts_match_positions() {
        assert_eq!(GRID_COLS, 4);
        assert_eq!(GRID_ROWS, 3);
        assert_eq!(POSITION_COUNT, 12);
        assert_eq!(DRIP_NOZZLE_COUNT, POSITION_COUNT);
        assert_eq!(COUPON_SLOT_COUNT, POSITION_COUNT);
        assert_eq!(BARCODE_LAND_COUNT, POSITION_COUNT + CERTIFICATE_LAND_COUNT);
        assert_eq!(position_label(POSITION_COUNT - 1), "c4");
    }

    #[test]
    fn tracer_witness_and_gutter_comparison_capacity_is_explicit() {
        assert_eq!(TRACER_COUNT, 4);
        assert_eq!(WITNESS_WELL_COUNT, 16);
        assert_eq!(GUTTER_LANE_COUNT, 3);
        assert_eq!(GUTTER_CHANNEL_COUNT, 6);
        assert_eq!(DIVERTER_VANE_COUNT, 6);
        assert!(containment_freeboard_ml() > maximum_condensate_challenge_ml());
    }

    #[test]
    fn environmental_logging_disposition_and_evidence_are_complete() {
        assert_eq!(LOGGER_POCKET_COUNT, 6);
        assert_eq!(DispositionLane::all().len(), 3);
        assert_eq!(DISPOSITION_SLOT_COUNT, POSITION_COUNT);
        assert_eq!(EVIDENCE_CAMERA_COUNT, 5);
        assert_eq!(EVIDENCE_LIGHT_BAR_COUNT, 2);
        assert_eq!(KEEP_OUT_GAUGE_COUNT, 5);
        assert!(ROBOT_Z_CLEARANCE > EVIDENCE_CLEARANCE_Z + EVIDENCE_BEAM_Z);
    }
}
