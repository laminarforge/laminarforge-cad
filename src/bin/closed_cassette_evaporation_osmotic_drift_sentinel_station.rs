use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed cassette long-duration evaporation and osmotic drift sentinel station.
//
// Engineering validation fixture only: models a sealed cassette nest grid,
// environmental logger docks, mass-reference pads, cap/seal witness features,
// condensate gutters, edge/center comparison markers, custody barcode lands,
// disposition lanes, and robot/service clearance gauges. It intentionally does
// not encode biological protocols, media recipes, acceptance thresholds, or
// release criteria.

const OUTPUT_PREFIX: &str = "closed_cassette_evaporation_osmotic_drift_sentinel_station";

const OUTPUTS: [&str; 11] = [
    "output/closed_cassette_evaporation_osmotic_drift_sentinel_station_base_deck.stl",
    "output/closed_cassette_evaporation_osmotic_drift_sentinel_station_sealed_cassette_nest_grid.stl",
    "output/closed_cassette_evaporation_osmotic_drift_sentinel_station_humidity_temperature_logger_docks.stl",
    "output/closed_cassette_evaporation_osmotic_drift_sentinel_station_mass_reference_pads.stl",
    "output/closed_cassette_evaporation_osmotic_drift_sentinel_station_reservoir_cap_seal_witness.stl",
    "output/closed_cassette_evaporation_osmotic_drift_sentinel_station_condensate_gutter.stl",
    "output/closed_cassette_evaporation_osmotic_drift_sentinel_station_edge_center_comparison_markers.stl",
    "output/closed_cassette_evaporation_osmotic_drift_sentinel_station_barcode_custody_plate.stl",
    "output/closed_cassette_evaporation_osmotic_drift_sentinel_station_release_hold_reject_lanes.stl",
    "output/closed_cassette_evaporation_osmotic_drift_sentinel_station_robotic_clearance_gauges.stl",
    "output/closed_cassette_evaporation_osmotic_drift_sentinel_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 10] = [
    "sealed_cassette_nest_grid",
    "humidity_temperature_logger_docks",
    "mass_reference_pads",
    "reservoir_cap_seal_witness",
    "condensate_gutter",
    "edge_center_comparison_markers",
    "barcode_custody_plate",
    "release_hold_reject_lanes",
    "robotic_clearance_gauges",
    "named_stl_outputs",
];

const STATION_X: f64 = 1500.0;
const STATION_Y: f64 = 920.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const SOCKET_DEPTH: f64 = 6.0;
const BASIN_DEPTH: f64 = 8.0;
const MOUNT_HOLE_COUNT: usize = 8;
const DATUM_COUNT: usize = 4;

const GRID_X: f64 = 620.0;
const GRID_Y: f64 = 430.0;
const GRID_Z: f64 = 44.0;
const GRID_COLS: usize = 4;
const GRID_ROWS: usize = 3;
const POSITION_COUNT: usize = GRID_COLS * GRID_ROWS;
const GRID_PITCH_X: f64 = 134.0;
const GRID_PITCH_Y: f64 = 116.0;
const CASSETTE_X: f64 = 102.0;
const CASSETTE_Y: f64 = 78.0;
const CASSETTE_Z: f64 = 26.0;
const SEAL_WITNESS_W: f64 = 7.0;
const GRID_POS: (f64, f64) = (-350.0, 80.0);

const LOGGER_X: f64 = 420.0;
const LOGGER_Y: f64 = 180.0;
const LOGGER_Z: f64 = 42.0;
const LOGGER_DOCK_COUNT: usize = 6;
const LOGGER_DOCK_X: f64 = 72.0;
const LOGGER_DOCK_Y: f64 = 42.0;
const LOGGER_POS: (f64, f64) = (360.0, 260.0);

const MASS_X: f64 = 420.0;
const MASS_Y: f64 = 190.0;
const MASS_Z: f64 = 34.0;
const MASS_PAD_COLS: usize = 4;
const MASS_PAD_ROWS: usize = 3;
const MASS_PAD_COUNT: usize = MASS_PAD_COLS * MASS_PAD_ROWS;
const MASS_PAD_D: f64 = 46.0;
const MASS_POS: (f64, f64) = (360.0, 40.0);

const SEAL_X: f64 = 420.0;
const SEAL_Y: f64 = 200.0;
const SEAL_Z: f64 = 40.0;
const RESERVOIR_WELL_COUNT: usize = 4;
const CAP_WITNESS_COUNT: usize = POSITION_COUNT;
const SEAL_POS: (f64, f64) = (360.0, -160.0);

const GUTTER_X: f64 = 620.0;
const GUTTER_Y: f64 = 170.0;
const GUTTER_Z: f64 = 38.0;
const GUTTER_CHANNEL_COUNT: usize = 4;
const GUTTER_CUP_COUNT: usize = 4;
const DRIP_VANE_COUNT: usize = 8;
const GUTTER_POS: (f64, f64) = (-350.0, -325.0);

const MARKER_X: f64 = 620.0;
const MARKER_Y: f64 = 118.0;
const MARKER_Z: f64 = 28.0;
const EDGE_MARKER_COUNT: usize = 8;
const CENTER_MARKER_COUNT: usize = 4;
const MARKER_POS: (f64, f64) = (-350.0, 365.0);

const BARCODE_X: f64 = 420.0;
const BARCODE_Y: f64 = 70.0;
const BARCODE_Z: f64 = 12.0;
const BARCODE_LAND_COUNT: usize = POSITION_COUNT + 5;
const BARCODE_POS: (f64, f64) = (360.0, -395.0);

const LANE_X: f64 = 420.0;
const LANE_Y: f64 = 84.0;
const LANE_Z: f64 = 30.0;
const DISPOSITION_LANE_COUNT: usize = 3;
const SLOTS_PER_LANE: usize = 4;
const DISPOSITION_SLOT_COUNT: usize = DISPOSITION_LANE_COUNT * SLOTS_PER_LANE;
const LANE_POS: (f64, f64) = (360.0, -312.0);

const KEEP_OUT_X: f64 = 1430.0;
const KEEP_OUT_Y: f64 = 860.0;
const KEEP_OUT_Z: f64 = 6.0;
const ROBOT_FRONT_CLEARANCE: f64 = 360.0;
const SERVICE_REAR_CLEARANCE: f64 = 220.0;
const SIDE_CLEARANCE: f64 = 240.0;
const ROBOT_Z_CLEARANCE: f64 = 310.0;
const CLEARANCE_GAUGE_COUNT: usize = 5;

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

    let base = base_deck();
    export(OUTPUTS[0], &base);

    let grid = sealed_cassette_nest_grid();
    export(OUTPUTS[1], &grid);

    let loggers = humidity_temperature_logger_docks();
    export(OUTPUTS[2], &loggers);

    let mass = mass_reference_pads();
    export(OUTPUTS[3], &mass);

    let seal = reservoir_cap_seal_witness();
    export(OUTPUTS[4], &seal);

    let gutter = condensate_gutter();
    export(OUTPUTS[5], &gutter);

    let markers = edge_center_comparison_markers();
    export(OUTPUTS[6], &markers);

    let custody = barcode_custody_plate();
    export(OUTPUTS[7], &custody);

    let lanes = release_hold_reject_lanes();
    export(OUTPUTS[8], &lanes);

    let clearances = robotic_clearance_gauges();
    export(OUTPUTS[9], &clearances);

    let assembly = base
        + grid.translate(GRID_POS.0, GRID_POS.1, insert_z(GRID_Z))
        + loggers.translate(LOGGER_POS.0, LOGGER_POS.1, insert_z(LOGGER_Z))
        + mass.translate(MASS_POS.0, MASS_POS.1, insert_z(MASS_Z))
        + seal.translate(SEAL_POS.0, SEAL_POS.1, insert_z(SEAL_Z))
        + gutter.translate(GUTTER_POS.0, GUTTER_POS.1, insert_z(GUTTER_Z))
        + markers.translate(MARKER_POS.0, MARKER_POS.1, insert_z(MARKER_Z))
        + custody.translate(BARCODE_POS.0, BARCODE_POS.1, insert_z(BARCODE_Z))
        + lanes.translate(LANE_POS.0, LANE_POS.1, insert_z(LANE_Z))
        + clearances.translate(0.0, 0.0, DECK_Z);
    export(OUTPUTS[10], &assembly);

    println!();
    println!("Closed cassette evaporation and osmotic drift sentinel station:");
    println!(
        "  Footprint:          {STATION_X:.0}mm x {STATION_Y:.0}mm validation deck with {:.0}mL secondary containment volume",
        containment_volume_ml()
    );
    println!(
        "  Cassette grid:      {GRID_COLS}x{GRID_ROWS} sealed nests, {POSITION_COUNT} cap-seal witness frames, edge/center comparison markers"
    );
    println!(
        "  Environment:        {LOGGER_DOCK_COUNT} humidity/temperature logger docks and {GUTTER_CHANNEL_COUNT} condensate gutter lanes"
    );
    println!(
        "  Mass controls:      {MASS_PAD_COUNT} mass-reference pads plus {RESERVOIR_WELL_COUNT} reservoir witness wells"
    );
    println!(
        "  Traceability:       {BARCODE_LAND_COUNT} custody lands and {DISPOSITION_SLOT_COUNT} release/hold/reject slots"
    );
    println!(
        "  Robot clearance:    {CLEARANCE_GAUGE_COUNT} gauges, {ROBOT_FRONT_CLEARANCE:.0}mm front approach, {SERVICE_REAR_CLEARANCE:.0}mm rear service, {ROBOT_Z_CLEARANCE:.0}mm Z gauge"
    );
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

fn module_rects() -> [Rect; 8] {
    [
        rect("sealed_cassette_nest_grid", GRID_POS, GRID_X, GRID_Y),
        rect(
            "humidity_temperature_logger_docks",
            LOGGER_POS,
            LOGGER_X,
            LOGGER_Y,
        ),
        rect("mass_reference_pads", MASS_POS, MASS_X, MASS_Y),
        rect("reservoir_cap_seal_witness", SEAL_POS, SEAL_X, SEAL_Y),
        rect("condensate_gutter", GUTTER_POS, GUTTER_X, GUTTER_Y),
        rect(
            "edge_center_comparison_markers",
            MARKER_POS,
            MARKER_X,
            MARKER_Y,
        ),
        rect("barcode_custody_plate", BARCODE_POS, BARCODE_X, BARCODE_Y),
        rect("release_hold_reject_lanes", LANE_POS, LANE_X, LANE_Y),
    ]
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 11);
    assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
    assert_eq!(REQUIRED_FEATURES.len(), 10);
    assert_eq!(POSITION_COUNT, GRID_COLS * GRID_ROWS);
    assert_eq!(MASS_PAD_COUNT, MASS_PAD_COLS * MASS_PAD_ROWS);
    assert_eq!(CAP_WITNESS_COUNT, POSITION_COUNT);
    assert_eq!(DISPOSITION_SLOT_COUNT, POSITION_COUNT);
    assert_eq!(DispositionLane::all().len(), DISPOSITION_LANE_COUNT);
    assert_eq!(deck_mount_positions().len(), MOUNT_HOLE_COUNT);
    assert_eq!(datum_positions().len(), DATUM_COUNT);
    assert_eq!(EDGE_MARKER_COUNT + CENTER_MARKER_COUNT, POSITION_COUNT);
    assert!(SEAL_WITNESS_W < CASSETTE_X / 10.0);
    assert!(LOGGER_DOCK_X * 3.0 < LOGGER_X);
    assert!(MASS_PAD_D * 4.0 < MASS_X);
    assert!(containment_volume_ml() > witness_hold_volume_ml());
    assert!(ROBOT_Z_CLEARANCE > GRID_Z + LOGGER_Z + 150.0);

    for item in module_rects() {
        assert!(item.fits_inside_deck(), "{} exceeds deck", item.name);
    }

    let rects = module_rects();
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

fn containment_volume_ml() -> f64 {
    let inner_x = STATION_X - 2.0 * RIM_W;
    let inner_y = STATION_Y - 2.0 * RIM_W;
    let freeboard_z = RIM_Z - BASIN_DEPTH;
    inner_x * inner_y * freeboard_z / 1000.0
}

fn witness_hold_volume_ml() -> f64 {
    let reservoir = RESERVOIR_WELL_COUNT as f64 * 45.0;
    let gutter = GUTTER_CUP_COUNT as f64 * 55.0;
    let cap_trace = CAP_WITNESS_COUNT as f64 * 5.0;
    reservoir + gutter + cap_trace
}

fn base_deck() -> Part {
    let deck = centered_cube(
        "evap_osmotic_sentinel_base_deck",
        STATION_X,
        STATION_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let basin = centered_cube(
        "evap_osmotic_sentinel_secondary_basin_cut",
        STATION_X - 2.0 * (RIM_W + 48.0),
        STATION_Y - 2.0 * (RIM_W + 48.0),
        BASIN_DEPTH + 0.8,
    )
    .translate(0.0, -6.0, DECK_Z - BASIN_DEPTH / 2.0);
    let drain_plug =
        centered_cylinder("evap_osmotic_sentinel_closed_drain_plug_cut", 5.0, 54.0, 28)
            .rotate(90.0, 0.0, 0.0)
            .translate(
                STATION_X / 2.0 - 96.0,
                -STATION_Y / 2.0 + 35.0,
                DECK_Z - 8.0,
            );

    deck - basin - drain_plug - insert_sockets() - deck_mount_holes()
        + perimeter_rims()
        + workflow_dividers()
        + robot_datums()
        + balance_rail_lands()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("evap_osmotic_sentinel_insert_sockets");
    for item in module_rects() {
        sockets = sockets
            + centered_cube(
                format!("evap_osmotic_sentinel_{}_socket", item.name),
                item.x + 10.0,
                item.y + 10.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(item.center.0, item.center.1, DECK_Z - SOCKET_DEPTH / 2.0);
    }
    sockets
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("evap_osmotic_sentinel_deck_mount_holes");
    for (i, (x, y)) in deck_mount_positions().into_iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("evap_osmotic_sentinel_m6_mount_clearance_{i}"),
                3.4,
                DECK_Z + 4.0,
                28,
            )
            .translate(x, y, DECK_Z / 2.0)
            + centered_cube(
                format!("evap_osmotic_sentinel_alignment_slot_{i}"),
                28.0,
                7.0,
                DECK_Z + 4.0,
            )
            .translate(x, y, DECK_Z / 2.0);
    }
    holes
}

fn deck_mount_positions() -> [(f64, f64); MOUNT_HOLE_COUNT] {
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
        "evap_osmotic_sentinel_front_robot_low_rim",
        STATION_X,
        RIM_W,
        28.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, DECK_Z + 14.0);
    let rear = centered_cube(
        "evap_osmotic_sentinel_rear_service_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "evap_osmotic_sentinel_left_spill_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "evap_osmotic_sentinel_right_spill_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);

    front + rear + left + right
}

fn workflow_dividers() -> Part {
    let center_spine = centered_cube(
        "evap_osmotic_sentinel_left_right_workflow_spine",
        12.0,
        STATION_Y - 180.0,
        26.0,
    )
    .translate(0.0, -10.0, DECK_Z + 13.0);
    let upper_spine = centered_cube(
        "evap_osmotic_sentinel_edge_center_marker_spine",
        STATION_X - 230.0,
        10.0,
        22.0,
    )
    .translate(0.0, 315.0, DECK_Z + 11.0);
    let lower_spine = centered_cube(
        "evap_osmotic_sentinel_gutter_custody_spine",
        STATION_X - 260.0,
        10.0,
        22.0,
    )
    .translate(0.0, -246.0, DECK_Z + 11.0);

    center_spine + upper_spine + lower_spine
}

fn robot_datums() -> Part {
    let mut datums = Part::empty("evap_osmotic_sentinel_robot_datums");
    for (i, (x, y)) in datum_positions().into_iter().enumerate() {
        datums = datums
            + fiducial_disc(&format!("evap_osmotic_sentinel_deck_datum_{i}")).translate(
                x,
                y,
                DECK_Z + 2.5,
            );
    }
    datums
}

fn datum_positions() -> [(f64, f64); DATUM_COUNT] {
    [
        (-STATION_X / 2.0 + 104.0, -STATION_Y / 2.0 + 96.0),
        (STATION_X / 2.0 - 104.0, -STATION_Y / 2.0 + 96.0),
        (-STATION_X / 2.0 + 104.0, STATION_Y / 2.0 - 96.0),
        (STATION_X / 2.0 - 104.0, STATION_Y / 2.0 - 96.0),
    ]
}

fn balance_rail_lands() -> Part {
    let left = centered_cube(
        "evap_osmotic_sentinel_balance_robot_left_rail_land",
        280.0,
        16.0,
        8.0,
    )
    .translate(250.0, -90.0, DECK_Z + 4.0);
    let right = centered_cube(
        "evap_osmotic_sentinel_balance_robot_right_rail_land",
        280.0,
        16.0,
        8.0,
    )
    .translate(470.0, -90.0, DECK_Z + 4.0);
    left + right
}

fn sealed_cassette_nest_grid() -> Part {
    let tray = centered_cube(
        "evap_osmotic_sentinel_sealed_nest_grid_tray",
        GRID_X,
        GRID_Y,
        GRID_Z,
    );
    let relief = centered_cube(
        "evap_osmotic_sentinel_nest_grid_moisture_relief_cut",
        GRID_X - 66.0,
        GRID_Y - 62.0,
        9.0,
    )
    .translate(0.0, 0.0, GRID_Z / 2.0 - 4.5);

    tray - cassette_recesses() - seal_witness_grooves() - relief
        + cassette_surrogate_caps()
        + nest_isolation_ribs()
        + grid_handles()
        + grid_fiducials()
}

fn cassette_recesses() -> Part {
    let mut recesses = Part::empty("evap_osmotic_sentinel_cassette_recesses");
    for position in 0..POSITION_COUNT {
        let (x, y) = grid_position_xy(position);
        recesses = recesses
            + centered_cube(
                format!(
                    "evap_osmotic_sentinel_position_{}_cassette_recess",
                    position_label(position)
                ),
                CASSETTE_X + 18.0,
                CASSETTE_Y + 18.0,
                12.0,
            )
            .translate(x, y, GRID_Z / 2.0 - 6.0);
    }
    recesses
}

fn seal_witness_grooves() -> Part {
    let mut grooves = Part::empty("evap_osmotic_sentinel_seal_witness_grooves");
    for position in 0..POSITION_COUNT {
        let (x, y) = grid_position_xy(position);
        let outer = centered_cube(
            format!(
                "evap_osmotic_sentinel_position_{}_seal_witness_outer_groove",
                position_label(position)
            ),
            CASSETTE_X + 30.0,
            CASSETTE_Y + 30.0,
            7.0,
        )
        .translate(x, y, GRID_Z / 2.0 + 1.5);
        let inner = centered_cube(
            format!(
                "evap_osmotic_sentinel_position_{}_seal_witness_inner_island",
                position_label(position)
            ),
            CASSETTE_X + 30.0 - 2.0 * SEAL_WITNESS_W,
            CASSETTE_Y + 30.0 - 2.0 * SEAL_WITNESS_W,
            8.0,
        )
        .translate(x, y, GRID_Z / 2.0 + 1.5);
        grooves = grooves + (outer - inner);
    }
    grooves
}

fn cassette_surrogate_caps() -> Part {
    let mut caps = Part::empty("evap_osmotic_sentinel_cassette_surrogate_caps");
    for position in 0..POSITION_COUNT {
        let (x, y) = grid_position_xy(position);
        let label = position_label(position);
        let body = centered_cube(
            format!("evap_osmotic_sentinel_{label}_sealed_cassette_surrogate"),
            CASSETTE_X,
            CASSETTE_Y,
            CASSETTE_Z,
        )
        .translate(x, y, GRID_Z / 2.0 + CASSETTE_Z / 2.0 - 2.0);
        let cap_land = centered_cube(
            format!("evap_osmotic_sentinel_{label}_cap_seal_witness_land"),
            CASSETTE_X - 22.0,
            14.0,
            4.0,
        )
        .translate(
            x,
            y + CASSETTE_Y / 2.0 - 18.0,
            GRID_Z / 2.0 + CASSETTE_Z + 2.0,
        );
        let evaporation_target = centered_cylinder(
            format!("evap_osmotic_sentinel_{label}_evaporation_mass_target"),
            12.0,
            3.0,
            30,
        )
        .translate(
            x,
            y - CASSETTE_Y / 2.0 + 18.0,
            GRID_Z / 2.0 + CASSETTE_Z + 1.5,
        );
        caps = caps + body + cap_land + evaporation_target;
    }
    caps
}

fn nest_isolation_ribs() -> Part {
    let mut ribs = Part::empty("evap_osmotic_sentinel_nest_isolation_ribs");
    for col in 1..GRID_COLS {
        ribs = ribs
            + centered_cube(
                format!("evap_osmotic_sentinel_column_isolation_rib_{col}"),
                7.0,
                GRID_Y - 74.0,
                18.0,
            )
            .translate(
                centered_index(col, GRID_COLS + 1, GRID_PITCH_X),
                0.0,
                GRID_Z / 2.0 + 9.0,
            );
    }
    for row in 1..GRID_ROWS {
        ribs = ribs
            + centered_cube(
                format!("evap_osmotic_sentinel_row_isolation_rib_{row}"),
                GRID_X - 84.0,
                7.0,
                18.0,
            )
            .translate(
                0.0,
                centered_index(row, GRID_ROWS + 1, GRID_PITCH_Y),
                GRID_Z / 2.0 + 9.0,
            );
    }
    ribs
}

fn grid_handles() -> Part {
    let left = centered_cube(
        "evap_osmotic_sentinel_grid_left_robot_handle",
        22.0,
        126.0,
        28.0,
    )
    .translate(-GRID_X / 2.0 + 26.0, 0.0, GRID_Z / 2.0 + 14.0);
    let right = centered_cube(
        "evap_osmotic_sentinel_grid_right_robot_handle",
        22.0,
        126.0,
        28.0,
    )
    .translate(GRID_X / 2.0 - 26.0, 0.0, GRID_Z / 2.0 + 14.0);
    left + right
}

fn grid_fiducials() -> Part {
    let mut fiducials = Part::empty("evap_osmotic_sentinel_grid_fiducials");
    for (i, (x, y)) in [
        (-GRID_X / 2.0 + 48.0, -GRID_Y / 2.0 + 42.0),
        (GRID_X / 2.0 - 48.0, -GRID_Y / 2.0 + 42.0),
        (-GRID_X / 2.0 + 48.0, GRID_Y / 2.0 - 42.0),
        (GRID_X / 2.0 - 48.0, GRID_Y / 2.0 - 42.0),
    ]
    .into_iter()
    .enumerate()
    {
        fiducials = fiducials
            + fiducial_disc(&format!("evap_osmotic_sentinel_grid_fiducial_{i}")).translate(
                x,
                y,
                GRID_Z / 2.0 + 2.5,
            );
    }
    fiducials
}

fn humidity_temperature_logger_docks() -> Part {
    let body = centered_cube(
        "evap_osmotic_sentinel_humidity_temperature_logger_dock_body",
        LOGGER_X,
        LOGGER_Y,
        LOGGER_Z,
    );
    let cable_trench = centered_cube(
        "evap_osmotic_sentinel_logger_cable_trench",
        LOGGER_X - 56.0,
        16.0,
        16.0,
    )
    .translate(0.0, -LOGGER_Y / 2.0 + 30.0, LOGGER_Z / 2.0 - 8.0);

    body - logger_dock_cuts() - cable_trench + logger_clip_ribs() + logger_id_lands()
}

fn logger_dock_cuts() -> Part {
    let mut docks = Part::empty("evap_osmotic_sentinel_logger_dock_cuts");
    for dock in 0..LOGGER_DOCK_COUNT {
        docks = docks
            + centered_cube(
                format!("evap_osmotic_sentinel_logger_dock_cut_{dock}"),
                LOGGER_DOCK_X,
                LOGGER_DOCK_Y,
                18.0,
            )
            .translate(
                centered_index(dock % 3, 3, 112.0),
                centered_index(dock / 3, 2, 58.0) + 8.0,
                LOGGER_Z / 2.0 - 9.0,
            );
    }
    docks
}

fn logger_clip_ribs() -> Part {
    let mut ribs = Part::empty("evap_osmotic_sentinel_logger_clip_ribs");
    for dock in 0..LOGGER_DOCK_COUNT {
        let x = centered_index(dock % 3, 3, 112.0);
        let y = centered_index(dock / 3, 2, 58.0) + 8.0;
        ribs = ribs
            + centered_cube(
                format!("evap_osmotic_sentinel_logger_{dock}_left_clip"),
                5.0,
                LOGGER_DOCK_Y + 8.0,
                10.0,
            )
            .translate(x - LOGGER_DOCK_X / 2.0 - 6.0, y, LOGGER_Z / 2.0 + 5.0)
            + centered_cube(
                format!("evap_osmotic_sentinel_logger_{dock}_right_clip"),
                5.0,
                LOGGER_DOCK_Y + 8.0,
                10.0,
            )
            .translate(x + LOGGER_DOCK_X / 2.0 + 6.0, y, LOGGER_Z / 2.0 + 5.0);
    }
    ribs
}

fn logger_id_lands() -> Part {
    let mut lands = Part::empty("evap_osmotic_sentinel_logger_id_lands");
    for dock in 0..LOGGER_DOCK_COUNT {
        lands = lands
            + centered_cube(
                format!("evap_osmotic_sentinel_logger_{dock}_id_land"),
                54.0,
                14.0,
                3.0,
            )
            .translate(
                centered_index(dock % 3, 3, 112.0),
                LOGGER_Y / 2.0 - 24.0 - (dock / 3) as f64 * 22.0,
                LOGGER_Z / 2.0 + 1.5,
            );
    }
    lands
}

fn mass_reference_pads() -> Part {
    let plate = centered_cube(
        "evap_osmotic_sentinel_mass_reference_plate",
        MASS_X,
        MASS_Y,
        MASS_Z,
    );
    let recess_relief = centered_cube(
        "evap_osmotic_sentinel_mass_reference_common_recess_relief",
        MASS_X - 58.0,
        MASS_Y - 52.0,
        8.0,
    )
    .translate(0.0, 0.0, MASS_Z / 2.0 - 4.0);

    plate - recess_relief - mass_pad_recesses() + mass_pad_lands() + balance_datum_tabs()
}

fn mass_pad_recesses() -> Part {
    let mut recesses = Part::empty("evap_osmotic_sentinel_mass_pad_recesses");
    for pad in 0..MASS_PAD_COUNT {
        recesses = recesses
            + centered_cylinder(
                format!("evap_osmotic_sentinel_mass_reference_pad_{pad}_recess"),
                MASS_PAD_D / 2.0,
                12.0,
                36,
            )
            .translate(
                centered_index(pad % MASS_PAD_COLS, MASS_PAD_COLS, 86.0),
                centered_index(pad / MASS_PAD_COLS, MASS_PAD_ROWS, 50.0),
                MASS_Z / 2.0 - 6.0,
            );
    }
    recesses
}

fn mass_pad_lands() -> Part {
    let mut lands = Part::empty("evap_osmotic_sentinel_mass_pad_lands");
    for pad in 0..MASS_PAD_COUNT {
        lands = lands
            + centered_cube(
                format!("evap_osmotic_sentinel_mass_reference_pad_{pad}_label_land"),
                48.0,
                12.0,
                3.0,
            )
            .translate(
                centered_index(pad % MASS_PAD_COLS, MASS_PAD_COLS, 86.0),
                centered_index(pad / MASS_PAD_COLS, MASS_PAD_ROWS, 50.0) + 29.0,
                MASS_Z / 2.0 + 1.5,
            );
    }
    lands
}

fn balance_datum_tabs() -> Part {
    let left = centered_cube(
        "evap_osmotic_sentinel_balance_left_datum_tab",
        56.0,
        24.0,
        8.0,
    )
    .translate(
        -MASS_X / 2.0 + 42.0,
        -MASS_Y / 2.0 + 26.0,
        MASS_Z / 2.0 + 4.0,
    );
    let right = centered_cube(
        "evap_osmotic_sentinel_balance_right_datum_tab",
        56.0,
        24.0,
        8.0,
    )
    .translate(
        MASS_X / 2.0 - 42.0,
        -MASS_Y / 2.0 + 26.0,
        MASS_Z / 2.0 + 4.0,
    );
    left + right
}

fn reservoir_cap_seal_witness() -> Part {
    let plate = centered_cube(
        "evap_osmotic_sentinel_reservoir_cap_seal_witness_plate",
        SEAL_X,
        SEAL_Y,
        SEAL_Z,
    );
    let spill_relief = centered_cube(
        "evap_osmotic_sentinel_seal_witness_spill_relief_cut",
        SEAL_X - 56.0,
        SEAL_Y - 52.0,
        9.0,
    )
    .translate(0.0, 0.0, SEAL_Z / 2.0 - 4.5);

    plate - spill_relief - reservoir_well_cuts() - cap_witness_slots()
        + seal_color_lands()
        + reservoir_index_tabs()
}

fn reservoir_well_cuts() -> Part {
    let mut cuts = Part::empty("evap_osmotic_sentinel_reservoir_well_cuts");
    for well in 0..RESERVOIR_WELL_COUNT {
        cuts = cuts
            + centered_cylinder(
                format!("evap_osmotic_sentinel_reservoir_well_{well}_cut"),
                28.0,
                18.0,
                36,
            )
            .translate(
                centered_index(well, RESERVOIR_WELL_COUNT, 82.0),
                SEAL_Y / 2.0 - 52.0,
                SEAL_Z / 2.0 - 8.0,
            );
    }
    cuts
}

fn cap_witness_slots() -> Part {
    let mut slots = Part::empty("evap_osmotic_sentinel_cap_witness_slots");
    for position in 0..CAP_WITNESS_COUNT {
        slots = slots
            + centered_cube(
                format!(
                    "evap_osmotic_sentinel_position_{}_cap_witness_slot",
                    position_label(position)
                ),
                58.0,
                18.0,
                11.0,
            )
            .translate(
                centered_index(position % 4, 4, 78.0),
                -SEAL_Y / 2.0 + 34.0 + (position / 4) as f64 * 28.0,
                SEAL_Z / 2.0 - 5.0,
            );
    }
    slots
}

fn seal_color_lands() -> Part {
    let mut lands = Part::empty("evap_osmotic_sentinel_seal_color_lands");
    for position in 0..CAP_WITNESS_COUNT {
        lands = lands
            + centered_cube(
                format!(
                    "evap_osmotic_sentinel_position_{}_seal_trace_land",
                    position_label(position)
                ),
                44.0,
                10.0,
                3.0,
            )
            .translate(
                centered_index(position % 4, 4, 78.0),
                -SEAL_Y / 2.0 + 49.0 + (position / 4) as f64 * 28.0,
                SEAL_Z / 2.0 + 1.5,
            );
    }
    lands
}

fn reservoir_index_tabs() -> Part {
    let mut tabs = Part::empty("evap_osmotic_sentinel_reservoir_index_tabs");
    for well in 0..RESERVOIR_WELL_COUNT {
        tabs = tabs
            + centered_cube(
                format!("evap_osmotic_sentinel_reservoir_index_tab_{well}"),
                46.0,
                12.0,
                4.0,
            )
            .translate(
                centered_index(well, RESERVOIR_WELL_COUNT, 82.0),
                SEAL_Y / 2.0 - 20.0,
                SEAL_Z / 2.0 + 2.0,
            );
    }
    tabs
}

fn condensate_gutter() -> Part {
    let plate = centered_cube(
        "evap_osmotic_sentinel_condensate_gutter_plate",
        GUTTER_X,
        GUTTER_Y,
        GUTTER_Z,
    );
    let basin = centered_cube(
        "evap_osmotic_sentinel_gutter_common_basin_cut",
        GUTTER_X - 54.0,
        GUTTER_Y - 44.0,
        9.0,
    )
    .translate(0.0, 0.0, GUTTER_Z / 2.0 - 4.0);

    plate - basin - gutter_channel_cuts() - gutter_collection_cups()
        + drip_diverter_vanes()
        + gutter_lane_lands()
}

fn gutter_channel_cuts() -> Part {
    let mut cuts = Part::empty("evap_osmotic_sentinel_gutter_channel_cuts");
    for channel in 0..GUTTER_CHANNEL_COUNT {
        cuts = cuts
            + centered_cube(
                format!("evap_osmotic_sentinel_condensate_channel_{channel}_cut"),
                GUTTER_X / GUTTER_CHANNEL_COUNT as f64 - 42.0,
                14.0,
                14.0,
            )
            .translate(
                centered_index(
                    channel,
                    GUTTER_CHANNEL_COUNT,
                    GUTTER_X / GUTTER_CHANNEL_COUNT as f64,
                ),
                16.0,
                GUTTER_Z / 2.0 - 7.0,
            );
    }
    cuts
}

fn gutter_collection_cups() -> Part {
    let mut cups = Part::empty("evap_osmotic_sentinel_gutter_collection_cups");
    for cup in 0..GUTTER_CUP_COUNT {
        cups = cups
            + centered_cylinder(
                format!("evap_osmotic_sentinel_condensate_collection_cup_{cup}_cut"),
                25.0,
                17.0,
                36,
            )
            .translate(
                centered_index(cup, GUTTER_CUP_COUNT, GUTTER_X / GUTTER_CUP_COUNT as f64),
                -GUTTER_Y / 2.0 + 38.0,
                GUTTER_Z / 2.0 - 8.0,
            );
    }
    cups
}

fn drip_diverter_vanes() -> Part {
    let mut vanes = Part::empty("evap_osmotic_sentinel_drip_diverter_vanes");
    for vane in 0..DRIP_VANE_COUNT {
        vanes = vanes
            + centered_cube(
                format!("evap_osmotic_sentinel_drip_diverter_vane_{vane}"),
                12.0,
                56.0,
                20.0,
            )
            .rotate(0.0, 0.0, if vane % 2 == 0 { -10.0 } else { 10.0 })
            .translate(
                centered_index(vane % GUTTER_CHANNEL_COUNT, GUTTER_CHANNEL_COUNT, 140.0)
                    + if vane / GUTTER_CHANNEL_COUNT == 0 {
                        -22.0
                    } else {
                        22.0
                    },
                centered_index(vane / GUTTER_CHANNEL_COUNT, 2, 52.0) + 14.0,
                GUTTER_Z / 2.0 + 10.0,
            );
    }
    vanes
}

fn gutter_lane_lands() -> Part {
    let mut lands = Part::empty("evap_osmotic_sentinel_gutter_lane_lands");
    for channel in 0..GUTTER_CHANNEL_COUNT {
        lands = lands
            + centered_cube(
                format!("evap_osmotic_sentinel_condensate_channel_{channel}_label_land"),
                108.0,
                18.0,
                3.0,
            )
            .translate(
                centered_index(
                    channel,
                    GUTTER_CHANNEL_COUNT,
                    GUTTER_X / GUTTER_CHANNEL_COUNT as f64,
                ),
                GUTTER_Y / 2.0 - 26.0,
                GUTTER_Z / 2.0 + 1.5,
            );
    }
    lands
}

fn edge_center_comparison_markers() -> Part {
    let plate = centered_cube(
        "evap_osmotic_sentinel_edge_center_comparison_marker_plate",
        MARKER_X,
        MARKER_Y,
        MARKER_Z,
    );
    plate + edge_position_markers() + center_position_markers() + comparison_axis_ticks()
}

fn edge_position_markers() -> Part {
    let mut markers = Part::empty("evap_osmotic_sentinel_edge_position_markers");
    for marker in 0..EDGE_MARKER_COUNT {
        let x = centered_index(marker % 4, 4, 132.0);
        let y = if marker < 4 {
            -MARKER_Y / 2.0 + 30.0
        } else {
            MARKER_Y / 2.0 - 30.0
        };
        markers = markers
            + centered_cube(
                format!("evap_osmotic_sentinel_edge_comparison_marker_{marker}"),
                64.0,
                18.0,
                5.0,
            )
            .translate(x, y, MARKER_Z / 2.0 + 2.5);
    }
    markers
}

fn center_position_markers() -> Part {
    let mut markers = Part::empty("evap_osmotic_sentinel_center_position_markers");
    for marker in 0..CENTER_MARKER_COUNT {
        markers = markers
            + centered_cylinder(
                format!("evap_osmotic_sentinel_center_comparison_marker_{marker}"),
                15.0,
                5.0,
                32,
            )
            .translate(
                centered_index(marker, CENTER_MARKER_COUNT, 78.0),
                0.0,
                MARKER_Z / 2.0 + 2.5,
            );
    }
    markers
}

fn comparison_axis_ticks() -> Part {
    let mut ticks = Part::empty("evap_osmotic_sentinel_comparison_axis_ticks");
    for tick in 0..POSITION_COUNT {
        ticks = ticks
            + centered_cube(
                format!(
                    "evap_osmotic_sentinel_position_{}_axis_tick",
                    position_label(tick)
                ),
                3.0,
                MARKER_Y - 24.0,
                4.0,
            )
            .translate(
                centered_index(tick % GRID_COLS, GRID_COLS, GRID_PITCH_X),
                0.0,
                MARKER_Z / 2.0 + 2.0,
            );
    }
    ticks
}

fn barcode_custody_plate() -> Part {
    let plate = centered_cube(
        "evap_osmotic_sentinel_barcode_custody_plate",
        BARCODE_X,
        BARCODE_Y,
        BARCODE_Z,
    );
    plate + position_barcode_lands() + run_custody_lands()
}

fn position_barcode_lands() -> Part {
    let mut lands = Part::empty("evap_osmotic_sentinel_position_barcode_lands");
    for position in 0..POSITION_COUNT {
        lands = lands
            + centered_cube(
                format!(
                    "evap_osmotic_sentinel_position_{}_barcode_land",
                    position_label(position)
                ),
                78.0,
                16.0,
                2.5,
            )
            .translate(
                centered_index(position % 4, 4, 88.0),
                centered_index(position / 4, 3, 20.0) + 4.0,
                BARCODE_Z / 2.0 + 1.25,
            );
    }
    lands
}

fn run_custody_lands() -> Part {
    let mut lands = Part::empty("evap_osmotic_sentinel_run_custody_lands");
    for land in 0..5 {
        lands = lands
            + centered_cube(
                format!("evap_osmotic_sentinel_run_custody_land_{land}"),
                66.0,
                14.0,
                2.5,
            )
            .translate(
                centered_index(land, 5, 76.0),
                -BARCODE_Y / 2.0 + 17.0,
                BARCODE_Z / 2.0 + 1.25,
            );
    }
    lands
}

fn release_hold_reject_lanes() -> Part {
    let plate = centered_cube(
        "evap_osmotic_sentinel_disposition_lane_plate",
        LANE_X,
        LANE_Y,
        LANE_Z,
    );
    plate - disposition_slot_cuts() + disposition_headers() + disposition_tokens()
}

fn disposition_slot_cuts() -> Part {
    let mut slots = Part::empty("evap_osmotic_sentinel_disposition_slot_cuts");
    for lane in DispositionLane::all() {
        for slot in 0..SLOTS_PER_LANE {
            slots = slots
                + centered_cube(
                    format!("evap_osmotic_sentinel_{}_lane_slot_{slot}_cut", lane.name()),
                    74.0,
                    22.0,
                    12.0,
                )
                .translate(
                    centered_index(slot, SLOTS_PER_LANE, 86.0),
                    centered_index(lane.index(), DISPOSITION_LANE_COUNT, 28.0),
                    LANE_Z / 2.0 - 6.0,
                );
        }
    }
    slots
}

fn disposition_headers() -> Part {
    let mut headers = Part::empty("evap_osmotic_sentinel_disposition_headers");
    for lane in DispositionLane::all() {
        headers = headers
            + centered_cube(
                format!("evap_osmotic_sentinel_{}_lane_header_land", lane.name()),
                64.0,
                16.0,
                3.0,
            )
            .translate(
                -LANE_X / 2.0 + 44.0,
                centered_index(lane.index(), DISPOSITION_LANE_COUNT, 28.0),
                LANE_Z / 2.0 + 1.5,
            );
    }
    headers
}

fn disposition_tokens() -> Part {
    let mut tokens = Part::empty("evap_osmotic_sentinel_disposition_tokens");
    for lane in DispositionLane::all() {
        for slot in 0..SLOTS_PER_LANE {
            tokens = tokens
                + centered_cube(
                    format!(
                        "evap_osmotic_sentinel_{}_lane_position_token_{slot}",
                        lane.name()
                    ),
                    46.0,
                    12.0,
                    5.0,
                )
                .translate(
                    centered_index(slot, SLOTS_PER_LANE, 86.0),
                    centered_index(lane.index(), DISPOSITION_LANE_COUNT, 28.0) + 9.0,
                    LANE_Z / 2.0 + 2.5,
                );
        }
    }
    tokens
}

fn robotic_clearance_gauges() -> Part {
    let perimeter = centered_cube(
        "evap_osmotic_sentinel_keepout_outer_reference_plate",
        KEEP_OUT_X,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    );
    let open_cut = centered_cube(
        "evap_osmotic_sentinel_keepout_open_work_area_cut",
        KEEP_OUT_X - 120.0,
        KEEP_OUT_Y - 120.0,
        KEEP_OUT_Z + 2.0,
    );
    let front = centered_cube(
        "evap_osmotic_sentinel_front_robot_approach_clearance_gauge",
        KEEP_OUT_X - 160.0,
        22.0,
        18.0,
    )
    .translate(0.0, -STATION_Y / 2.0 - ROBOT_FRONT_CLEARANCE / 2.0, 9.0);
    let rear = centered_cube(
        "evap_osmotic_sentinel_rear_service_clearance_gauge",
        KEEP_OUT_X - 180.0,
        22.0,
        18.0,
    )
    .translate(0.0, STATION_Y / 2.0 + SERVICE_REAR_CLEARANCE / 2.0, 9.0);
    let side = centered_cube(
        "evap_osmotic_sentinel_side_service_clearance_gauge",
        22.0,
        KEEP_OUT_Y - 180.0,
        18.0,
    )
    .translate(STATION_X / 2.0 + SIDE_CLEARANCE / 2.0, 0.0, 9.0);
    let z = centered_cube(
        "evap_osmotic_sentinel_robot_z_clearance_gauge",
        68.0,
        68.0,
        ROBOT_Z_CLEARANCE,
    )
    .translate(
        STATION_X / 2.0 - 94.0,
        STATION_Y / 2.0 - 104.0,
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
            assert!(path
                .starts_with("output/closed_cassette_evaporation_osmotic_drift_sentinel_station_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn requested_feature_groups_are_explicit() {
        for feature in [
            "sealed_cassette_nest_grid",
            "humidity_temperature_logger_docks",
            "mass_reference_pads",
            "reservoir_cap_seal_witness",
            "condensate_gutter",
            "edge_center_comparison_markers",
            "barcode_custody_plate",
            "release_hold_reject_lanes",
            "robotic_clearance_gauges",
            "named_stl_outputs",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn module_bounds_fit_without_overlap() {
        assert_design_constraints();
        let rects = module_rects();
        for rect in rects {
            assert!(rect.fits_inside_deck(), "{} does not fit deck", rect.name);
        }
    }

    #[test]
    fn cassette_grid_and_edge_center_markers_match_positions() {
        assert_eq!(GRID_COLS, 4);
        assert_eq!(GRID_ROWS, 3);
        assert_eq!(POSITION_COUNT, 12);
        assert_eq!(CAP_WITNESS_COUNT, POSITION_COUNT);
        assert_eq!(EDGE_MARKER_COUNT, 8);
        assert_eq!(CENTER_MARKER_COUNT, 4);
        assert_eq!(EDGE_MARKER_COUNT + CENTER_MARKER_COUNT, POSITION_COUNT);
        assert_eq!(position_label(POSITION_COUNT - 1), "c4");
    }

    #[test]
    fn environment_mass_and_seal_witness_counts_are_locked() {
        assert_eq!(LOGGER_DOCK_COUNT, 6);
        assert_eq!(MASS_PAD_COUNT, 12);
        assert_eq!(RESERVOIR_WELL_COUNT, 4);
        assert_eq!(GUTTER_CHANNEL_COUNT, 4);
        assert_eq!(GUTTER_CUP_COUNT, 4);
        assert_eq!(DRIP_VANE_COUNT, 8);
        assert!(containment_volume_ml() > witness_hold_volume_ml());
    }

    #[test]
    fn custody_disposition_and_robotic_clearances_are_complete() {
        assert_eq!(BARCODE_LAND_COUNT, POSITION_COUNT + 5);
        assert_eq!(DispositionLane::all().len(), 3);
        assert_eq!(DISPOSITION_SLOT_COUNT, POSITION_COUNT);
        assert_eq!(CLEARANCE_GAUGE_COUNT, 5);
        assert!(ROBOT_FRONT_CLEARANCE >= 360.0);
        assert!(SERVICE_REAR_CLEARANCE >= 220.0);
        assert!(SIDE_CLEARANCE >= 240.0);
        assert!(ROBOT_Z_CLEARANCE > GRID_Z + LOGGER_Z + 150.0);
    }
}
