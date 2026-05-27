use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed cassette evaporation edge-guard/media-overlay validation station.
//
// Mechanical validation fixture only: models a cassette edge/center surrogate,
// overlay dam coupons, evaporation witness wells, osmolality sample pockets,
// RH/logger docks, fill-level reference ladder, condensate drip witnesses,
// custody lands, robot/service gauges, and release/hold/reject gates. It does
// not encode media recipes, biological acceptance criteria, or study limits.

const OUTPUT_PREFIX: &str = "closed_cassette_evaporation_edge_guard_media_overlay_station";

const OUTPUTS: [&str; 12] = [
    "output/closed_cassette_evaporation_edge_guard_media_overlay_station_containment_deck.stl",
    "output/closed_cassette_evaporation_edge_guard_media_overlay_station_edge_center_surrogate_grid.stl",
    "output/closed_cassette_evaporation_edge_guard_media_overlay_station_overlay_dam_coupon_bank.stl",
    "output/closed_cassette_evaporation_edge_guard_media_overlay_station_evaporation_witness_wells.stl",
    "output/closed_cassette_evaporation_edge_guard_media_overlay_station_osmolality_sample_pockets.stl",
    "output/closed_cassette_evaporation_edge_guard_media_overlay_station_rh_temperature_logger_docks.stl",
    "output/closed_cassette_evaporation_edge_guard_media_overlay_station_fill_level_reference_ladder.stl",
    "output/closed_cassette_evaporation_edge_guard_media_overlay_station_condensate_drip_witness.stl",
    "output/closed_cassette_evaporation_edge_guard_media_overlay_station_release_hold_reject_gate_bank.stl",
    "output/closed_cassette_evaporation_edge_guard_media_overlay_station_barcode_custody_evidence_lands.stl",
    "output/closed_cassette_evaporation_edge_guard_media_overlay_station_robot_service_keepout_gauges.stl",
    "output/closed_cassette_evaporation_edge_guard_media_overlay_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 18] = [
    "cassette_edge_center_surrogate",
    "edge_lane_map",
    "center_position_map",
    "media_overlay_dams",
    "edge_guard_coupon_bank",
    "evaporation_witness_wells",
    "osmolality_sample_pockets",
    "rh_logger_docks",
    "temperature_logger_docks",
    "fill_level_reference_ladder",
    "condensate_drip_witness",
    "release_gate",
    "hold_gate",
    "reject_gate",
    "barcode_custody_lands",
    "evidence_photo_lands",
    "robot_service_keepout_gauges",
    "named_stl_outputs",
];

const DECK_X: f64 = 1520.0;
const DECK_Y: f64 = 980.0;
const DECK_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const SOCKET_DEPTH: f64 = 5.0;
const BASIN_DEPTH: f64 = 7.0;
const MOUNT_HOLE_D: f64 = 6.0;
const MOUNT_HOLE_COUNT: usize = 8;
const DATUM_COUNT: usize = 4;

const CASSETTE_COLS: usize = 4;
const CASSETTE_ROWS: usize = 5;
const POSITION_COUNT: usize = CASSETTE_COLS * CASSETTE_ROWS;
const EDGE_POSITION_COUNT: usize = 14;
const CENTER_POSITION_COUNT: usize = POSITION_COUNT - EDGE_POSITION_COUNT;
const CHIP_GAP: f64 = 8.0;
const GRID_MARGIN_X: f64 = 34.0;
const GRID_MARGIN_Y: f64 = 34.0;
const GRID_X: f64 = CASSETTE_COLS as f64 * REVC_CHIP_LENGTH
    + (CASSETTE_COLS as f64 - 1.0) * CHIP_GAP
    + 2.0 * GRID_MARGIN_X;
const GRID_Y: f64 = CASSETTE_ROWS as f64 * REVC_CHIP_WIDTH
    + (CASSETTE_ROWS as f64 - 1.0) * CHIP_GAP
    + 2.0 * GRID_MARGIN_Y;
const GRID_Z: f64 = 40.0;
const CHIP_RECESS_DEPTH: f64 = 12.0;
const CHIP_SURROGATE_Z: f64 = 18.0;
const EDGE_GUARD_RAIL_W: f64 = 7.0;
const EDGE_GUARD_RAIL_Z: f64 = 11.0;
const CENTER_MARKER_D: f64 = 10.0;
const EDGE_MARKER_D: f64 = 14.0;
const GRID_POS: (f64, f64) = (-220.0, 130.0);

const COUPON_X: f64 = 390.0;
const COUPON_Y: f64 = 220.0;
const COUPON_Z: f64 = 28.0;
const COUPON_POS: (f64, f64) = (470.0, 260.0);
const OVERLAY_DAM_COUPONS: usize = 12;
const COUPON_COLS: usize = 4;
const COUPON_ROWS: usize = 3;
const COUPON_SLOT_X: f64 = 64.0;
const COUPON_SLOT_Y: f64 = 34.0;
const COUPON_SLOT_DEPTH: f64 = 10.0;
const COUPON_PITCH_X: f64 = 84.0;
const COUPON_PITCH_Y: f64 = 54.0;
const DAM_HEIGHT_LEVELS_MM: [f64; 3] = [0.8, 1.6, 2.4];

const EVAP_X: f64 = 390.0;
const EVAP_Y: f64 = 190.0;
const EVAP_Z: f64 = 34.0;
const EVAP_POS: (f64, f64) = (470.0, 20.0);
const EVAP_WELL_COUNT: usize = POSITION_COUNT;
const EVAP_WELL_D: f64 = 17.0;
const EVAP_WELL_DEPTH: f64 = 15.0;
const EVAP_COLS: usize = 5;
const EVAP_ROWS: usize = 4;
const EVAP_PITCH_X: f64 = 66.0;
const EVAP_PITCH_Y: f64 = 42.0;

const OSMO_X: f64 = 390.0;
const OSMO_Y: f64 = 190.0;
const OSMO_Z: f64 = 36.0;
const OSMO_POS: (f64, f64) = (470.0, -190.0);
const OSMO_SAMPLE_POCKETS: usize = 24;
const OSMO_COLS: usize = 6;
const OSMO_ROWS: usize = 4;
const OSMO_POCKET_D: f64 = 16.0;
const OSMO_POCKET_DEPTH: f64 = 16.0;
const OSMO_PITCH_X: f64 = 55.0;
const OSMO_PITCH_Y: f64 = 42.0;

const LOGGER_X: f64 = 200.0;
const LOGGER_Y: f64 = 210.0;
const LOGGER_Z: f64 = 34.0;
const LOGGER_POS: (f64, f64) = (-630.0, -260.0);
const LOGGER_DOCKS: usize = 6;
const LOGGER_COLS: usize = 2;
const LOGGER_ROWS: usize = 3;
const LOGGER_POCKET_X: f64 = 72.0;
const LOGGER_POCKET_Y: f64 = 38.0;
const LOGGER_POCKET_DEPTH: f64 = 14.0;
const LOGGER_PITCH_X: f64 = 86.0;
const LOGGER_PITCH_Y: f64 = 56.0;
const LOGGER_CABLE_COMBS: usize = LOGGER_DOCKS;

const LADDER_X: f64 = 200.0;
const LADDER_Y: f64 = 250.0;
const LADDER_Z: f64 = 28.0;
const LADDER_POS: (f64, f64) = (-630.0, 55.0);
const LEVEL_STEPS: usize = 8;
const LEVEL_STEP_HEIGHT_MM: f64 = 0.75;
const LADDER_RUNG_X: f64 = 118.0;
const LADDER_RUNG_Y: f64 = 11.0;
const LADDER_PITCH_Y: f64 = 24.0;

const DRIP_X: f64 = 600.0;
const DRIP_Y: f64 = 150.0;
const DRIP_Z: f64 = 34.0;
const DRIP_POS: (f64, f64) = (-170.0, -350.0);
const DRIP_LANES: usize = 5;
const DRIP_CUPS: usize = 10;
const DRIP_VANES: usize = 8;
const DRIP_CHANNEL_W: f64 = 22.0;
const DRIP_CHANNEL_DEPTH: f64 = 13.0;
const DRIP_CUP_D: f64 = 18.0;

const GATE_X: f64 = 390.0;
const GATE_Y: f64 = 120.0;
const GATE_Z: f64 = 32.0;
const GATE_POS: (f64, f64) = (470.0, -398.0);
const GATE_COUNT: usize = 3;
const GATE_TOKEN_SLOTS: usize = 6;
const GATE_LANE_X: f64 = 112.0;
const GATE_LANE_Y: f64 = 78.0;
const GATE_PITCH_X: f64 = 130.0;

const CUSTODY_X: f64 = 190.0;
const CUSTODY_Y: f64 = 150.0;
const CUSTODY_Z: f64 = 16.0;
const CUSTODY_POS: (f64, f64) = (-630.0, 330.0);
const BARCODE_LANDS: usize = 10;
const EVIDENCE_LANDS: usize = 4;
const BARCODE_LAND_X: f64 = 58.0;
const BARCODE_LAND_Y: f64 = 15.0;

const KEEP_OUT_Z: f64 = 7.0;
const FRONT_ROBOT_CLEARANCE_Y: f64 = 122.0;
const REAR_SERVICE_CLEARANCE_Y: f64 = 108.0;
const LEFT_LOGGER_SERVICE_X: f64 = 118.0;
const RIGHT_SAMPLE_SERVICE_X: f64 = 138.0;
const TOP_PICK_CLEARANCE_Z: f64 = 160.0;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MapZone {
    Edge,
    Center,
}

impl MapZone {
    fn label(self) -> &'static str {
        match self {
            MapZone::Edge => "edge",
            MapZone::Center => "center",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DispositionGate {
    Release,
    Hold,
    Reject,
}

impl DispositionGate {
    fn all() -> [DispositionGate; GATE_COUNT] {
        [
            DispositionGate::Release,
            DispositionGate::Hold,
            DispositionGate::Reject,
        ]
    }

    fn label(self) -> &'static str {
        match self {
            DispositionGate::Release => "release",
            DispositionGate::Hold => "hold",
            DispositionGate::Reject => "reject",
        }
    }

    fn index(self) -> usize {
        match self {
            DispositionGate::Release => 0,
            DispositionGate::Hold => 1,
            DispositionGate::Reject => 2,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Footprint {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Footprint {
    fn fits_inside_deck(self, margin: f64) -> bool {
        self.center.0 - self.x / 2.0 >= -DECK_X / 2.0 + margin
            && self.center.0 + self.x / 2.0 <= DECK_X / 2.0 - margin
            && self.center.1 - self.y / 2.0 >= -DECK_Y / 2.0 + margin
            && self.center.1 + self.y / 2.0 <= DECK_Y / 2.0 - margin
    }

    fn overlaps(self, other: Footprint) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();
        dx < (self.x + other.x) / 2.0 && dy < (self.y + other.y) / 2.0
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = containment_deck();
    export(OUTPUTS[0], &deck);

    let grid = edge_center_surrogate_grid();
    export(OUTPUTS[1], &grid);

    let coupons = overlay_dam_coupon_bank();
    export(OUTPUTS[2], &coupons);

    let evaporation = evaporation_witness_wells();
    export(OUTPUTS[3], &evaporation);

    let osmolality = osmolality_sample_pockets();
    export(OUTPUTS[4], &osmolality);

    let loggers = rh_temperature_logger_docks();
    export(OUTPUTS[5], &loggers);

    let ladder = fill_level_reference_ladder();
    export(OUTPUTS[6], &ladder);

    let drip = condensate_drip_witness();
    export(OUTPUTS[7], &drip);

    let gates = release_hold_reject_gate_bank();
    export(OUTPUTS[8], &gates);

    let custody = barcode_custody_evidence_lands();
    export(OUTPUTS[9], &custody);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[10], &keepouts);

    let assembly = deck
        + grid.translate(GRID_POS.0, GRID_POS.1, insert_z(GRID_Z))
        + coupons.translate(COUPON_POS.0, COUPON_POS.1, insert_z(COUPON_Z))
        + evaporation.translate(EVAP_POS.0, EVAP_POS.1, insert_z(EVAP_Z))
        + osmolality.translate(OSMO_POS.0, OSMO_POS.1, insert_z(OSMO_Z))
        + loggers.translate(LOGGER_POS.0, LOGGER_POS.1, insert_z(LOGGER_Z))
        + ladder.translate(LADDER_POS.0, LADDER_POS.1, insert_z(LADDER_Z))
        + drip.translate(DRIP_POS.0, DRIP_POS.1, insert_z(DRIP_Z))
        + gates.translate(GATE_POS.0, GATE_POS.1, insert_z(GATE_Z))
        + custody.translate(CUSTODY_POS.0, CUSTODY_POS.1, insert_z(CUSTODY_Z))
        + keepouts.translate(0.0, 0.0, DECK_Z);
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed cassette evaporation edge-guard/media-overlay station:");
    println!(
        "  Deck:                 {DECK_X:.0}mm x {DECK_Y:.0}mm contained cassette validation deck with {:.0}mL witness containment",
        containment_volume_ml()
    );
    println!(
        "  Cassette surrogate:   {CASSETTE_COLS}x{CASSETTE_ROWS} positions, {EDGE_POSITION_COUNT} edge and {CENTER_POSITION_COUNT} center comparison homes"
    );
    println!(
        "  Overlay challenge:    {OVERLAY_DAM_COUPONS} removable dam coupons across {} dam heights",
        DAM_HEIGHT_LEVELS_MM.len()
    );
    println!(
        "  Drift evidence:       {EVAP_WELL_COUNT} evaporation witness wells, {OSMO_SAMPLE_POCKETS} osmolality sample pockets, {DRIP_CUPS} condensate witness cups"
    );
    println!(
        "  Environment:          {LOGGER_DOCKS} RH/temperature logger docks plus {LEVEL_STEPS} fill-level reference ladder steps"
    );
    println!(
        "  Disposition:          release/hold/reject gates with {} total token slots",
        total_gate_token_slots()
    );
    println!(
        "  Clearances:           {FRONT_ROBOT_CLEARANCE_Y:.0}mm front robot gauge, {LEFT_LOGGER_SERVICE_X:.0}mm logger service gauge, {TOP_PICK_CLEARANCE_Z:.0}mm Z-pick gauge"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 12);
    assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
    assert_eq!(REQUIRED_FEATURES.len(), 18);
    assert!(REQUIRED_FEATURES.contains(&"media_overlay_dams"));
    assert!(REQUIRED_FEATURES.contains(&"edge_guard_coupon_bank"));
    assert_eq!(POSITION_COUNT, CASSETTE_COLS * CASSETTE_ROWS);
    assert_eq!(EDGE_POSITION_COUNT + CENTER_POSITION_COUNT, POSITION_COUNT);
    assert_eq!(edge_position_count(), EDGE_POSITION_COUNT);
    assert_eq!(center_position_count(), CENTER_POSITION_COUNT);
    assert_eq!(OVERLAY_DAM_COUPONS, COUPON_COLS * COUPON_ROWS);
    assert_eq!(EVAP_WELL_COUNT, EVAP_COLS * EVAP_ROWS);
    assert_eq!(OSMO_SAMPLE_POCKETS, OSMO_COLS * OSMO_ROWS);
    assert_eq!(LOGGER_DOCKS, LOGGER_COLS * LOGGER_ROWS);
    assert_eq!(LOGGER_CABLE_COMBS, LOGGER_DOCKS);
    assert_eq!(DispositionGate::all().len(), GATE_COUNT);
    assert_eq!(total_gate_token_slots(), GATE_COUNT * GATE_TOKEN_SLOTS);
    assert_eq!(mount_points().len(), MOUNT_HOLE_COUNT);
    assert_eq!(datum_points().len(), DATUM_COUNT);
    assert!(REVC_TOTAL_HEIGHT < CHIP_SURROGATE_Z + CHIP_RECESS_DEPTH);
    assert!(CHIP_RECESS_DEPTH < GRID_Z);
    assert!(EVAP_WELL_DEPTH < EVAP_Z);
    assert!(OSMO_POCKET_DEPTH < OSMO_Z);
    assert!(LOGGER_POCKET_DEPTH < LOGGER_Z);
    assert!(COUPON_SLOT_DEPTH < COUPON_Z);
    assert!(DAM_HEIGHT_LEVELS_MM.windows(2).all(|w| w[0] < w[1]));
    assert!(TOP_PICK_CLEARANCE_Z > GRID_Z + CHIP_SURROGATE_Z + 80.0);

    let footprints = module_footprints();
    for item in footprints {
        assert!(
            item.fits_inside_deck(30.0),
            "{} exceeds deck envelope",
            item.name
        );
    }
    for (i, a) in footprints.iter().enumerate() {
        for b in footprints.iter().skip(i + 1) {
            assert!(!a.overlaps(*b), "{} overlaps {}", a.name, b.name);
        }
    }
}

fn module_footprints() -> [Footprint; 9] {
    [
        footprint("edge_center_surrogate_grid", GRID_POS, GRID_X, GRID_Y),
        footprint("overlay_dam_coupon_bank", COUPON_POS, COUPON_X, COUPON_Y),
        footprint("evaporation_witness_wells", EVAP_POS, EVAP_X, EVAP_Y),
        footprint("osmolality_sample_pockets", OSMO_POS, OSMO_X, OSMO_Y),
        footprint(
            "rh_temperature_logger_docks",
            LOGGER_POS,
            LOGGER_X,
            LOGGER_Y,
        ),
        footprint(
            "fill_level_reference_ladder",
            LADDER_POS,
            LADDER_X,
            LADDER_Y,
        ),
        footprint("condensate_drip_witness", DRIP_POS, DRIP_X, DRIP_Y),
        footprint("release_hold_reject_gate_bank", GATE_POS, GATE_X, GATE_Y),
        footprint(
            "barcode_custody_evidence_lands",
            CUSTODY_POS,
            CUSTODY_X,
            CUSTODY_Y,
        ),
    ]
}

fn footprint(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Footprint {
    Footprint { name, center, x, y }
}

fn insert_z(height: f64) -> f64 {
    DECK_Z + height / 2.0 - SOCKET_DEPTH / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn chip_position_xy(position: usize) -> (f64, f64) {
    let col = position % CASSETTE_COLS;
    let row = position / CASSETTE_COLS;
    (
        centered_index(col, CASSETTE_COLS, REVC_CHIP_LENGTH + CHIP_GAP),
        centered_index(row, CASSETTE_ROWS, REVC_CHIP_WIDTH + CHIP_GAP),
    )
}

fn position_zone(position: usize) -> MapZone {
    let col = position % CASSETTE_COLS;
    let row = position / CASSETTE_COLS;
    if col == 0 || col == CASSETTE_COLS - 1 || row == 0 || row == CASSETTE_ROWS - 1 {
        MapZone::Edge
    } else {
        MapZone::Center
    }
}

fn edge_position_count() -> usize {
    (0..POSITION_COUNT)
        .filter(|position| position_zone(*position) == MapZone::Edge)
        .count()
}

fn center_position_count() -> usize {
    POSITION_COUNT - edge_position_count()
}

fn position_label(position: usize) -> String {
    let row = position / CASSETTE_COLS;
    let col = position % CASSETTE_COLS;
    format!("r{}_c{}", row + 1, col + 1)
}

fn total_gate_token_slots() -> usize {
    DispositionGate::all()
        .into_iter()
        .map(|_| GATE_TOKEN_SLOTS)
        .sum()
}

fn containment_volume_ml() -> f64 {
    let inner_x = DECK_X - 2.0 * RIM_W;
    let inner_y = DECK_Y - 2.0 * RIM_W;
    inner_x * inner_y * BASIN_DEPTH / 1000.0
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        "edge_guard_overlay_containment_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let basin = centered_cube(
        "edge_guard_overlay_shallow_secondary_basin_cut",
        DECK_X - 2.0 * (RIM_W + 42.0),
        DECK_Y - 2.0 * (RIM_W + 42.0),
        BASIN_DEPTH + 0.6,
    )
    .translate(0.0, -8.0, DECK_Z - BASIN_DEPTH / 2.0);
    let drain_plug = centered_cylinder(
        "edge_guard_overlay_closed_drain_plug_cut",
        10.0,
        DECK_Z + 4.0,
        32,
    )
    .translate(DECK_X / 2.0 - 86.0, -DECK_Y / 2.0 + 56.0, DECK_Z / 2.0);

    deck - basin - drain_plug - module_sockets() - mount_holes()
        + perimeter_rims()
        + workflow_dividers()
        + datum_targets()
        + containment_volume_badge()
}

fn module_sockets() -> Part {
    let mut sockets = Part::empty("edge_guard_overlay_module_registration_sockets");
    for item in module_footprints() {
        sockets = sockets
            + centered_cube(
                format!("edge_guard_overlay_{}_socket", item.name),
                item.x + 12.0,
                item.y + 12.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(item.center.0, item.center.1, DECK_Z - SOCKET_DEPTH / 2.0);
    }
    sockets
}

fn mount_holes() -> Part {
    let mut holes = Part::empty("edge_guard_overlay_mount_holes");
    for (i, (x, y)) in mount_points().into_iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("edge_guard_overlay_m5_mount_clearance_{i}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 4.0,
                28,
            )
            .translate(x, y, DECK_Z / 2.0);
    }
    holes
}

fn mount_points() -> [(f64, f64); MOUNT_HOLE_COUNT] {
    [
        (-DECK_X / 2.0 + 58.0, -DECK_Y / 2.0 + 58.0),
        (DECK_X / 2.0 - 58.0, -DECK_Y / 2.0 + 58.0),
        (-DECK_X / 2.0 + 58.0, DECK_Y / 2.0 - 58.0),
        (DECK_X / 2.0 - 58.0, DECK_Y / 2.0 - 58.0),
        (0.0, -DECK_Y / 2.0 + 58.0),
        (0.0, DECK_Y / 2.0 - 58.0),
        (-DECK_X / 2.0 + 58.0, 0.0),
        (DECK_X / 2.0 - 58.0, 0.0),
    ]
}

fn perimeter_rims() -> Part {
    let front = centered_cube(
        "edge_guard_overlay_front_robot_low_rim",
        DECK_X - 110.0,
        RIM_W,
        24.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + RIM_W / 2.0, DECK_Z + 12.0);
    let rear = centered_cube(
        "edge_guard_overlay_rear_service_high_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "edge_guard_overlay_left_logger_service_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "edge_guard_overlay_right_sample_service_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    front + rear + left + right
}

fn workflow_dividers() -> Part {
    let horizontal = centered_cube(
        "edge_guard_overlay_edge_center_to_sample_workflow_divider",
        DECK_X - 260.0,
        9.0,
        19.0,
    )
    .translate(0.0, -95.0, DECK_Z + 9.5);
    let lower = centered_cube(
        "edge_guard_overlay_condensate_gate_workflow_divider",
        DECK_X - 280.0,
        9.0,
        18.0,
    )
    .translate(0.0, -307.0, DECK_Z + 9.0);
    let vertical = centered_cube(
        "edge_guard_overlay_logger_sample_workflow_spine",
        9.0,
        DECK_Y - 220.0,
        18.0,
    )
    .translate(160.0, -20.0, DECK_Z + 9.0);
    horizontal + lower + vertical
}

fn datum_targets() -> Part {
    let mut datums = Part::empty("edge_guard_overlay_datum_targets");
    for (i, (x, y)) in datum_points().into_iter().enumerate() {
        datums = datums
            + fiducial_disc(&format!("edge_guard_overlay_deck_datum_{i}")).translate(
                x,
                y,
                DECK_Z + 2.0,
            );
    }
    datums
}

fn datum_points() -> [(f64, f64); DATUM_COUNT] {
    [
        (-DECK_X / 2.0 + 116.0, -DECK_Y / 2.0 + 104.0),
        (DECK_X / 2.0 - 116.0, -DECK_Y / 2.0 + 104.0),
        (-DECK_X / 2.0 + 116.0, DECK_Y / 2.0 - 104.0),
        (DECK_X / 2.0 - 116.0, DECK_Y / 2.0 - 104.0),
    ]
}

fn containment_volume_badge() -> Part {
    let badge = centered_cube(
        "edge_guard_overlay_containment_volume_label_land",
        168.0,
        26.0,
        3.0,
    )
    .translate(-DECK_X / 2.0 + 176.0, -DECK_Y / 2.0 + 92.0, DECK_Z + 1.5);
    let ticks = (0..5).fold(
        Part::empty("edge_guard_overlay_containment_volume_tick_marks"),
        |acc, i| {
            acc + centered_cube(
                format!("edge_guard_overlay_containment_volume_tick_{i}"),
                6.0,
                18.0,
                3.5,
            )
            .translate(
                -DECK_X / 2.0 + 118.0 + i as f64 * 28.0,
                -DECK_Y / 2.0 + 92.0,
                DECK_Z + 1.75,
            )
        },
    );
    badge + ticks
}

fn edge_center_surrogate_grid() -> Part {
    let tray = centered_cube(
        "edge_guard_overlay_surrogate_grid_tray",
        GRID_X,
        GRID_Y,
        GRID_Z,
    );
    let relief = centered_cube(
        "edge_guard_overlay_grid_moisture_relief_cut",
        GRID_X - 64.0,
        GRID_Y - 64.0,
        8.0,
    )
    .translate(0.0, 0.0, GRID_Z / 2.0 - 4.0);

    tray - relief - chip_recesses()
        + chip_surrogate_inserts()
        + grid_isolation_ribs()
        + edge_guard_lane_rails()
        + edge_center_map_tokens()
        + grid_pick_handles()
}

fn chip_recesses() -> Part {
    let mut recesses = Part::empty("edge_guard_overlay_chip_recesses");
    for position in 0..POSITION_COUNT {
        let (x, y) = chip_position_xy(position);
        recesses = recesses
            + centered_cube(
                format!(
                    "edge_guard_overlay_{}_{}_chip_recess",
                    position_zone(position).label(),
                    position_label(position)
                ),
                REVC_CHIP_LENGTH + 10.0,
                REVC_CHIP_WIDTH + 10.0,
                CHIP_RECESS_DEPTH,
            )
            .translate(x, y, GRID_Z / 2.0 - CHIP_RECESS_DEPTH / 2.0);
    }
    recesses
}

fn chip_surrogate_inserts() -> Part {
    let mut inserts = Part::empty("edge_guard_overlay_chip_surrogate_inserts");
    for position in 0..POSITION_COUNT {
        let (x, y) = chip_position_xy(position);
        let zone = position_zone(position);
        let label = position_label(position);
        let body = centered_cube(
            format!(
                "edge_guard_overlay_{}_{}_sealed_chip_surrogate",
                zone.label(),
                label
            ),
            REVC_CHIP_LENGTH - 12.0,
            REVC_CHIP_WIDTH - 12.0,
            CHIP_SURROGATE_Z,
        )
        .translate(
            x,
            y,
            GRID_Z / 2.0 + CHIP_SURROGATE_Z / 2.0 - CHIP_RECESS_DEPTH / 2.0,
        );
        let overlay_land = centered_cube(
            format!(
                "edge_guard_overlay_{}_{}_media_overlay_land",
                zone.label(),
                label
            ),
            REVC_CHIP_LENGTH - 36.0,
            14.0,
            3.0,
        )
        .translate(
            x,
            y + REVC_CHIP_WIDTH / 2.0 - 22.0,
            GRID_Z / 2.0 + CHIP_SURROGATE_Z + 1.5 - CHIP_RECESS_DEPTH / 2.0,
        );
        inserts = inserts + body + overlay_land;
    }
    inserts
}

fn grid_isolation_ribs() -> Part {
    let mut ribs = Part::empty("edge_guard_overlay_grid_isolation_ribs");
    for col in 1..CASSETTE_COLS {
        ribs = ribs
            + centered_cube(
                format!("edge_guard_overlay_column_isolation_rib_{col}"),
                7.0,
                GRID_Y - 76.0,
                16.0,
            )
            .translate(
                centered_index(col, CASSETTE_COLS + 1, REVC_CHIP_LENGTH + CHIP_GAP),
                0.0,
                GRID_Z / 2.0 + 8.0,
            );
    }
    for row in 1..CASSETTE_ROWS {
        ribs = ribs
            + centered_cube(
                format!("edge_guard_overlay_row_isolation_rib_{row}"),
                GRID_X - 82.0,
                7.0,
                16.0,
            )
            .translate(
                0.0,
                centered_index(row, CASSETTE_ROWS + 1, REVC_CHIP_WIDTH + CHIP_GAP),
                GRID_Z / 2.0 + 8.0,
            );
    }
    ribs
}

fn edge_guard_lane_rails() -> Part {
    let mut rails = Part::empty("edge_guard_overlay_edge_guard_lane_rails");
    for position in 0..POSITION_COUNT {
        if position_zone(position) == MapZone::Edge {
            let (x, y) = chip_position_xy(position);
            let label = position_label(position);
            rails = rails
                + centered_cube(
                    format!("edge_guard_overlay_{label}_north_media_overlay_dam"),
                    REVC_CHIP_LENGTH - 22.0,
                    EDGE_GUARD_RAIL_W,
                    EDGE_GUARD_RAIL_Z,
                )
                .translate(
                    x,
                    y + REVC_CHIP_WIDTH / 2.0 - 11.0,
                    GRID_Z / 2.0 + EDGE_GUARD_RAIL_Z / 2.0,
                )
                + centered_cube(
                    format!("edge_guard_overlay_{label}_south_media_overlay_dam"),
                    REVC_CHIP_LENGTH - 22.0,
                    EDGE_GUARD_RAIL_W,
                    EDGE_GUARD_RAIL_Z,
                )
                .translate(
                    x,
                    y - REVC_CHIP_WIDTH / 2.0 + 11.0,
                    GRID_Z / 2.0 + EDGE_GUARD_RAIL_Z / 2.0,
                );
        }
    }
    rails
}

fn edge_center_map_tokens() -> Part {
    let mut tokens = Part::empty("edge_guard_overlay_edge_center_map_tokens");
    for position in 0..POSITION_COUNT {
        let (x, y) = chip_position_xy(position);
        let (diameter, z) = match position_zone(position) {
            MapZone::Edge => (EDGE_MARKER_D, 4.0),
            MapZone::Center => (CENTER_MARKER_D, 6.0),
        };
        tokens = tokens
            + centered_cylinder(
                format!(
                    "edge_guard_overlay_{}_{}_map_token",
                    position_zone(position).label(),
                    position_label(position)
                ),
                diameter / 2.0,
                z,
                28,
            )
            .translate(
                x - REVC_CHIP_LENGTH / 2.0 + 22.0,
                y - REVC_CHIP_WIDTH / 2.0 + 18.0,
                GRID_Z / 2.0 + z / 2.0,
            );
    }
    tokens
}

fn grid_pick_handles() -> Part {
    let left = centered_cube(
        "edge_guard_overlay_grid_left_robot_pick_handle",
        24.0,
        132.0,
        26.0,
    )
    .translate(-GRID_X / 2.0 + 28.0, 0.0, GRID_Z / 2.0 + 13.0);
    let right = centered_cube(
        "edge_guard_overlay_grid_right_robot_pick_handle",
        24.0,
        132.0,
        26.0,
    )
    .translate(GRID_X / 2.0 - 28.0, 0.0, GRID_Z / 2.0 + 13.0);
    left + right
}

fn overlay_dam_coupon_bank() -> Part {
    let plate = centered_cube(
        "edge_guard_overlay_dam_coupon_bank_plate",
        COUPON_X,
        COUPON_Y,
        COUPON_Z,
    );
    let relief = centered_cube(
        "edge_guard_overlay_dam_coupon_spill_relief_cut",
        COUPON_X - 56.0,
        COUPON_Y - 52.0,
        7.0,
    )
    .translate(0.0, 0.0, COUPON_Z / 2.0 - 3.5);

    plate - relief - coupon_slot_cuts() + removable_overlay_dam_coupons() + coupon_height_keys()
}

fn coupon_slot_cuts() -> Part {
    let mut slots = Part::empty("edge_guard_overlay_coupon_slot_cuts");
    for coupon in 0..OVERLAY_DAM_COUPONS {
        let (x, y) = coupon_xy(coupon);
        slots = slots
            + centered_cube(
                format!("edge_guard_overlay_dam_coupon_{coupon}_slot_cut"),
                COUPON_SLOT_X + 8.0,
                COUPON_SLOT_Y + 8.0,
                COUPON_SLOT_DEPTH,
            )
            .translate(x, y, COUPON_Z / 2.0 - COUPON_SLOT_DEPTH / 2.0);
    }
    slots
}

fn removable_overlay_dam_coupons() -> Part {
    let mut coupons = Part::empty("edge_guard_overlay_removable_dam_coupons");
    for coupon in 0..OVERLAY_DAM_COUPONS {
        let (x, y) = coupon_xy(coupon);
        let height = DAM_HEIGHT_LEVELS_MM[coupon / COUPON_COLS];
        let land_z = 5.0 + height;
        let base = centered_cube(
            format!("edge_guard_overlay_dam_coupon_{coupon}_base"),
            COUPON_SLOT_X,
            COUPON_SLOT_Y,
            5.0,
        )
        .translate(x, y, COUPON_Z / 2.0 + 2.5);
        let dam = centered_cube(
            format!("edge_guard_overlay_dam_coupon_{coupon}_{height:.1}mm_media_dam"),
            COUPON_SLOT_X - 12.0,
            5.0,
            land_z,
        )
        .translate(
            x,
            y + COUPON_SLOT_Y / 2.0 - 8.0,
            COUPON_Z / 2.0 + land_z / 2.0,
        );
        let witness_groove = centered_cube(
            format!("edge_guard_overlay_dam_coupon_{coupon}_meniscus_witness_land"),
            COUPON_SLOT_X - 18.0,
            7.0,
            3.0,
        )
        .translate(x, y - 6.0, COUPON_Z / 2.0 + 6.5);
        coupons = coupons + base + dam + witness_groove;
    }
    coupons
}

fn coupon_height_keys() -> Part {
    let mut keys = Part::empty("edge_guard_overlay_coupon_height_keys");
    for row in 0..COUPON_ROWS {
        keys = keys
            + centered_cube(
                format!("edge_guard_overlay_coupon_height_key_row_{row}"),
                22.0,
                8.0 + row as f64 * 5.0,
                6.0,
            )
            .translate(
                -COUPON_X / 2.0 + 25.0,
                centered_index(row, COUPON_ROWS, COUPON_PITCH_Y),
                COUPON_Z / 2.0 + 3.0,
            );
    }
    keys
}

fn coupon_xy(coupon: usize) -> (f64, f64) {
    (
        centered_index(coupon % COUPON_COLS, COUPON_COLS, COUPON_PITCH_X),
        centered_index(coupon / COUPON_COLS, COUPON_ROWS, COUPON_PITCH_Y),
    )
}

fn evaporation_witness_wells() -> Part {
    let plate = centered_cube(
        "edge_guard_overlay_evaporation_witness_well_plate",
        EVAP_X,
        EVAP_Y,
        EVAP_Z,
    );
    let relief = centered_cube(
        "edge_guard_overlay_evaporation_well_spill_relief_cut",
        EVAP_X - 54.0,
        EVAP_Y - 48.0,
        8.0,
    )
    .translate(0.0, 0.0, EVAP_Z / 2.0 - 4.0);

    plate - relief - evaporation_well_cuts() + evaporation_well_rings() + evaporation_edge_tags()
}

fn evaporation_well_cuts() -> Part {
    let mut cuts = Part::empty("edge_guard_overlay_evaporation_well_cuts");
    for well in 0..EVAP_WELL_COUNT {
        let (x, y) = evaporation_well_xy(well);
        cuts = cuts
            + centered_cylinder(
                format!("edge_guard_overlay_evaporation_witness_well_{well}_cut"),
                EVAP_WELL_D / 2.0,
                EVAP_WELL_DEPTH,
                36,
            )
            .translate(x, y, EVAP_Z / 2.0 - EVAP_WELL_DEPTH / 2.0);
    }
    cuts
}

fn evaporation_well_rings() -> Part {
    let mut rings = Part::empty("edge_guard_overlay_evaporation_well_edge_center_rings");
    for well in 0..EVAP_WELL_COUNT {
        let (x, y) = evaporation_well_xy(well);
        let zone = position_zone(well);
        let outer_d = match zone {
            MapZone::Edge => EVAP_WELL_D + 11.0,
            MapZone::Center => EVAP_WELL_D + 7.0,
        };
        rings = rings
            + centered_cylinder(
                format!(
                    "edge_guard_overlay_{}_evaporation_well_{well}_rim",
                    zone.label()
                ),
                outer_d / 2.0,
                3.0,
                36,
            )
            .translate(x, y, EVAP_Z / 2.0 + 1.5);
    }
    rings
}

fn evaporation_edge_tags() -> Part {
    let mut tags = Part::empty("edge_guard_overlay_evaporation_edge_center_tags");
    for well in 0..EVAP_WELL_COUNT {
        let (x, y) = evaporation_well_xy(well);
        tags = tags
            + centered_cube(
                format!(
                    "edge_guard_overlay_{}_evaporation_well_{well}_barcode_tag",
                    position_zone(well).label()
                ),
                28.0,
                7.0,
                3.0,
            )
            .translate(x, y + 21.0, EVAP_Z / 2.0 + 1.5);
    }
    tags
}

fn evaporation_well_xy(well: usize) -> (f64, f64) {
    (
        centered_index(well % EVAP_COLS, EVAP_COLS, EVAP_PITCH_X),
        centered_index(well / EVAP_COLS, EVAP_ROWS, EVAP_PITCH_Y),
    )
}

fn osmolality_sample_pockets() -> Part {
    let plate = centered_cube(
        "edge_guard_overlay_osmolality_sample_pocket_plate",
        OSMO_X,
        OSMO_Y,
        OSMO_Z,
    );
    let trough = centered_cube(
        "edge_guard_overlay_osmolality_spill_trough_cut",
        OSMO_X - 54.0,
        18.0,
        10.0,
    )
    .translate(0.0, -OSMO_Y / 2.0 + 30.0, OSMO_Z / 2.0 - 5.0);

    plate - trough - osmolality_pocket_cuts() + osmolality_pocket_labels() + paired_sample_bridges()
}

fn osmolality_pocket_cuts() -> Part {
    let mut cuts = Part::empty("edge_guard_overlay_osmolality_pocket_cuts");
    for pocket in 0..OSMO_SAMPLE_POCKETS {
        let (x, y) = osmolality_pocket_xy(pocket);
        cuts = cuts
            + centered_cylinder(
                format!("edge_guard_overlay_osmolality_sample_pocket_{pocket}_cut"),
                OSMO_POCKET_D / 2.0,
                OSMO_POCKET_DEPTH,
                34,
            )
            .translate(x, y, OSMO_Z / 2.0 - OSMO_POCKET_DEPTH / 2.0);
    }
    cuts
}

fn osmolality_pocket_labels() -> Part {
    let mut labels = Part::empty("edge_guard_overlay_osmolality_sample_pocket_labels");
    for pocket in 0..OSMO_SAMPLE_POCKETS {
        let (x, y) = osmolality_pocket_xy(pocket);
        labels = labels
            + centered_cube(
                format!("edge_guard_overlay_osmolality_sample_pocket_{pocket}_label_land"),
                30.0,
                7.0,
                3.0,
            )
            .translate(x, y + 21.0, OSMO_Z / 2.0 + 1.5);
    }
    labels
}

fn paired_sample_bridges() -> Part {
    let mut bridges = Part::empty("edge_guard_overlay_osmolality_edge_center_pair_bridges");
    for pair in 0..(OSMO_SAMPLE_POCKETS / 2) {
        let (x0, y0) = osmolality_pocket_xy(pair * 2);
        let (x1, _) = osmolality_pocket_xy(pair * 2 + 1);
        bridges = bridges
            + centered_cube(
                format!("edge_guard_overlay_osmolality_pair_{pair}_edge_center_bridge"),
                (x1 - x0).abs() - OSMO_POCKET_D,
                4.0,
                4.0,
            )
            .translate((x0 + x1) / 2.0, y0, OSMO_Z / 2.0 + 2.0);
    }
    bridges
}

fn osmolality_pocket_xy(pocket: usize) -> (f64, f64) {
    (
        centered_index(pocket % OSMO_COLS, OSMO_COLS, OSMO_PITCH_X),
        centered_index(pocket / OSMO_COLS, OSMO_ROWS, OSMO_PITCH_Y),
    )
}

fn rh_temperature_logger_docks() -> Part {
    let plate = centered_cube(
        "edge_guard_overlay_rh_temperature_logger_dock_plate",
        LOGGER_X,
        LOGGER_Y,
        LOGGER_Z,
    );
    let cable_spine = centered_cube(
        "edge_guard_overlay_logger_cable_spine_cut",
        15.0,
        LOGGER_Y - 42.0,
        13.0,
    )
    .translate(0.0, 0.0, LOGGER_Z / 2.0 - 6.5);

    plate - cable_spine - logger_pocket_cuts() + logger_retainer_clips() + logger_cable_combs()
}

fn logger_pocket_cuts() -> Part {
    let mut cuts = Part::empty("edge_guard_overlay_logger_pocket_cuts");
    for dock in 0..LOGGER_DOCKS {
        let (x, y) = logger_xy(dock);
        cuts = cuts
            + centered_cube(
                format!("edge_guard_overlay_rh_temperature_logger_dock_{dock}_cut"),
                LOGGER_POCKET_X,
                LOGGER_POCKET_Y,
                LOGGER_POCKET_DEPTH,
            )
            .translate(x, y, LOGGER_Z / 2.0 - LOGGER_POCKET_DEPTH / 2.0);
    }
    cuts
}

fn logger_retainer_clips() -> Part {
    let mut clips = Part::empty("edge_guard_overlay_logger_retainer_clips");
    for dock in 0..LOGGER_DOCKS {
        let (x, y) = logger_xy(dock);
        clips = clips
            + centered_cube(
                format!("edge_guard_overlay_logger_{dock}_left_retainer_clip"),
                5.0,
                LOGGER_POCKET_Y + 6.0,
                9.0,
            )
            .translate(x - LOGGER_POCKET_X / 2.0 - 5.0, y, LOGGER_Z / 2.0 + 4.5)
            + centered_cube(
                format!("edge_guard_overlay_logger_{dock}_right_retainer_clip"),
                5.0,
                LOGGER_POCKET_Y + 6.0,
                9.0,
            )
            .translate(x + LOGGER_POCKET_X / 2.0 + 5.0, y, LOGGER_Z / 2.0 + 4.5);
    }
    clips
}

fn logger_cable_combs() -> Part {
    let mut combs = Part::empty("edge_guard_overlay_logger_cable_combs");
    for dock in 0..LOGGER_DOCKS {
        let (_, y) = logger_xy(dock);
        combs = combs
            + centered_cube(
                format!("edge_guard_overlay_logger_{dock}_cable_comb"),
                9.0,
                32.0,
                8.0,
            )
            .translate(LOGGER_X / 2.0 - 26.0, y, LOGGER_Z / 2.0 + 4.0);
    }
    combs
}

fn logger_xy(dock: usize) -> (f64, f64) {
    (
        centered_index(dock % LOGGER_COLS, LOGGER_COLS, LOGGER_PITCH_X),
        centered_index(dock / LOGGER_COLS, LOGGER_ROWS, LOGGER_PITCH_Y),
    )
}

fn fill_level_reference_ladder() -> Part {
    let plate = centered_cube(
        "edge_guard_overlay_fill_level_reference_ladder_plate",
        LADDER_X,
        LADDER_Y,
        LADDER_Z,
    );
    let sight_channel = centered_cube(
        "edge_guard_overlay_ladder_clear_sight_channel_cut",
        42.0,
        LADDER_Y - 44.0,
        10.0,
    )
    .translate(34.0, 0.0, LADDER_Z / 2.0 - 5.0);

    plate - sight_channel + ladder_rungs() + meniscus_tick_posts() + ladder_zero_datum()
}

fn ladder_rungs() -> Part {
    let mut rungs = Part::empty("edge_guard_overlay_fill_level_ladder_rungs");
    for step in 0..LEVEL_STEPS {
        let y = centered_index(step, LEVEL_STEPS, LADDER_PITCH_Y);
        let height = 3.0 + step as f64 * LEVEL_STEP_HEIGHT_MM;
        rungs = rungs
            + centered_cube(
                format!("edge_guard_overlay_fill_level_step_{step}_{height:.2}mm"),
                LADDER_RUNG_X - step as f64 * 5.0,
                LADDER_RUNG_Y,
                height,
            )
            .translate(-20.0, y, LADDER_Z / 2.0 + height / 2.0);
    }
    rungs
}

fn meniscus_tick_posts() -> Part {
    let mut ticks = Part::empty("edge_guard_overlay_meniscus_tick_posts");
    for step in 0..LEVEL_STEPS {
        let y = centered_index(step, LEVEL_STEPS, LADDER_PITCH_Y);
        ticks = ticks
            + centered_cylinder(
                format!("edge_guard_overlay_fill_level_meniscus_tick_{step}"),
                3.0,
                8.0,
                18,
            )
            .translate(76.0, y, LADDER_Z / 2.0 + 4.0);
    }
    ticks
}

fn ladder_zero_datum() -> Part {
    centered_cube(
        "edge_guard_overlay_fill_level_zero_reference_datum_land",
        LADDER_X - 46.0,
        12.0,
        4.0,
    )
    .translate(0.0, -LADDER_Y / 2.0 + 24.0, LADDER_Z / 2.0 + 2.0)
}

fn condensate_drip_witness() -> Part {
    let plate = centered_cube(
        "edge_guard_overlay_condensate_drip_witness_plate",
        DRIP_X,
        DRIP_Y,
        DRIP_Z,
    );
    let sump = centered_cube(
        "edge_guard_overlay_condensate_drip_common_sump_cut",
        DRIP_X - 58.0,
        28.0,
        DRIP_CHANNEL_DEPTH,
    )
    .translate(
        0.0,
        -DRIP_Y / 2.0 + 38.0,
        DRIP_Z / 2.0 - DRIP_CHANNEL_DEPTH / 2.0,
    );

    plate - sump - drip_channel_cuts() - drip_cup_cuts() + drip_vanes() + drip_barcode_tabs()
}

fn drip_channel_cuts() -> Part {
    let mut cuts = Part::empty("edge_guard_overlay_condensate_drip_channel_cuts");
    for lane in 0..DRIP_LANES {
        cuts = cuts
            + centered_cube(
                format!("edge_guard_overlay_condensate_lane_{lane}_cut"),
                DRIP_X - 82.0,
                DRIP_CHANNEL_W,
                DRIP_CHANNEL_DEPTH,
            )
            .translate(
                0.0,
                centered_index(lane, DRIP_LANES, 23.0),
                DRIP_Z / 2.0 - DRIP_CHANNEL_DEPTH / 2.0,
            );
    }
    cuts
}

fn drip_cup_cuts() -> Part {
    let mut cups = Part::empty("edge_guard_overlay_condensate_drip_cup_cuts");
    for cup in 0..DRIP_CUPS {
        cups = cups
            + centered_cylinder(
                format!("edge_guard_overlay_condensate_drip_witness_cup_{cup}_cut"),
                DRIP_CUP_D / 2.0,
                14.0,
                30,
            )
            .translate(
                centered_index(cup % 5, 5, 92.0),
                if cup < 5 { 42.0 } else { -42.0 },
                DRIP_Z / 2.0 - 7.0,
            );
    }
    cups
}

fn drip_vanes() -> Part {
    let mut vanes = Part::empty("edge_guard_overlay_condensate_drip_vanes");
    for vane in 0..DRIP_VANES {
        vanes = vanes
            + centered_cube(
                format!("edge_guard_overlay_condensate_directional_vane_{vane}"),
                8.0,
                42.0,
                12.0,
            )
            .translate(
                centered_index(vane, DRIP_VANES, 62.0),
                0.0,
                DRIP_Z / 2.0 + 6.0,
            );
    }
    vanes
}

fn drip_barcode_tabs() -> Part {
    let mut tabs = Part::empty("edge_guard_overlay_condensate_barcode_tabs");
    for lane in 0..DRIP_LANES {
        tabs = tabs
            + centered_cube(
                format!("edge_guard_overlay_condensate_lane_{lane}_barcode_tab"),
                42.0,
                9.0,
                3.0,
            )
            .translate(
                -DRIP_X / 2.0 + 50.0 + lane as f64 * 54.0,
                DRIP_Y / 2.0 - 22.0,
                DRIP_Z / 2.0 + 1.5,
            );
    }
    tabs
}

fn release_hold_reject_gate_bank() -> Part {
    let plate = centered_cube(
        "edge_guard_overlay_release_hold_reject_gate_bank",
        GATE_X,
        GATE_Y,
        GATE_Z,
    );
    let common_relief = centered_cube(
        "edge_guard_overlay_gate_bank_spill_relief_cut",
        GATE_X - 54.0,
        GATE_Y - 36.0,
        7.0,
    )
    .translate(0.0, 0.0, GATE_Z / 2.0 - 3.5);

    plate - common_relief - gate_lane_cuts() + gate_lane_walls() + gate_token_slots()
}

fn gate_lane_cuts() -> Part {
    let mut cuts = Part::empty("edge_guard_overlay_release_hold_reject_gate_lane_cuts");
    for gate in DispositionGate::all() {
        cuts = cuts
            + centered_cube(
                format!("edge_guard_overlay_{}_gate_lane_recess_cut", gate.label()),
                GATE_LANE_X,
                GATE_LANE_Y,
                12.0,
            )
            .translate(gate_x(gate), 0.0, GATE_Z / 2.0 - 6.0);
    }
    cuts
}

fn gate_lane_walls() -> Part {
    let mut walls = Part::empty("edge_guard_overlay_release_hold_reject_gate_walls");
    for gate in DispositionGate::all() {
        let x = gate_x(gate);
        walls = walls
            + centered_cube(
                format!("edge_guard_overlay_{}_gate_front_wall", gate.label()),
                GATE_LANE_X,
                6.0,
                13.0,
            )
            .translate(x, -GATE_LANE_Y / 2.0 - 7.0, GATE_Z / 2.0 + 6.5)
            + centered_cube(
                format!("edge_guard_overlay_{}_gate_back_wall", gate.label()),
                GATE_LANE_X,
                6.0,
                13.0,
            )
            .translate(x, GATE_LANE_Y / 2.0 + 7.0, GATE_Z / 2.0 + 6.5);
    }
    walls
}

fn gate_token_slots() -> Part {
    let mut slots = Part::empty("edge_guard_overlay_release_hold_reject_token_slots");
    for gate in DispositionGate::all() {
        let x0 = gate_x(gate);
        for slot in 0..GATE_TOKEN_SLOTS {
            slots = slots
                + centered_cylinder(
                    format!("edge_guard_overlay_{}_gate_token_slot_{slot}", gate.label()),
                    7.0,
                    4.0,
                    24,
                )
                .translate(
                    x0 + centered_index(slot % 3, 3, 26.0),
                    centered_index(slot / 3, 2, 25.0),
                    GATE_Z / 2.0 + 2.0,
                );
        }
    }
    slots
}

fn gate_x(gate: DispositionGate) -> f64 {
    centered_index(gate.index(), GATE_COUNT, GATE_PITCH_X)
}

fn barcode_custody_evidence_lands() -> Part {
    let plate = centered_cube(
        "edge_guard_overlay_barcode_custody_evidence_plate",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    plate + barcode_lands() + evidence_lands() + custody_seal_bosses()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("edge_guard_overlay_barcode_custody_lands");
    for land in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("edge_guard_overlay_barcode_land_{land}"),
                BARCODE_LAND_X,
                BARCODE_LAND_Y,
                3.0,
            )
            .translate(
                centered_index(land % 2, 2, 78.0),
                CUSTODY_Y / 2.0 - 24.0 - (land / 2) as f64 * 20.0,
                CUSTODY_Z / 2.0 + 1.5,
            );
    }
    lands
}

fn evidence_lands() -> Part {
    let mut lands = Part::empty("edge_guard_overlay_evidence_photo_lands");
    for land in 0..EVIDENCE_LANDS {
        lands = lands
            + centered_cube(
                format!("edge_guard_overlay_evidence_photo_land_{land}"),
                34.0,
                24.0,
                3.5,
            )
            .translate(
                centered_index(land, EVIDENCE_LANDS, 38.0),
                -CUSTODY_Y / 2.0 + 24.0,
                CUSTODY_Z / 2.0 + 1.75,
            );
    }
    lands
}

fn custody_seal_bosses() -> Part {
    let mut bosses = Part::empty("edge_guard_overlay_custody_seal_bosses");
    for (i, (x, y)) in [
        (-CUSTODY_X / 2.0 + 18.0, -CUSTODY_Y / 2.0 + 18.0),
        (CUSTODY_X / 2.0 - 18.0, -CUSTODY_Y / 2.0 + 18.0),
        (-CUSTODY_X / 2.0 + 18.0, CUSTODY_Y / 2.0 - 18.0),
        (CUSTODY_X / 2.0 - 18.0, CUSTODY_Y / 2.0 - 18.0),
    ]
    .into_iter()
    .enumerate()
    {
        bosses = bosses
            + centered_cylinder(
                format!("edge_guard_overlay_custody_seal_boss_{i}"),
                5.5,
                4.0,
                20,
            )
            .translate(x, y, CUSTODY_Z / 2.0 + 2.0);
    }
    bosses
}

fn robot_service_keepout_gauges() -> Part {
    let front = centered_cube(
        "edge_guard_overlay_front_robot_approach_keepout_gauge",
        DECK_X - 160.0,
        7.0,
        KEEP_OUT_Z,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + FRONT_ROBOT_CLEARANCE_Y,
        KEEP_OUT_Z / 2.0,
    );
    let rear = centered_cube(
        "edge_guard_overlay_rear_service_pull_keepout_gauge",
        DECK_X - 180.0,
        7.0,
        KEEP_OUT_Z,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - REAR_SERVICE_CLEARANCE_Y,
        KEEP_OUT_Z / 2.0,
    );
    let left = centered_cube(
        "edge_guard_overlay_left_logger_service_keepout_gauge",
        7.0,
        DECK_Y - 210.0,
        KEEP_OUT_Z,
    )
    .translate(-DECK_X / 2.0 + LEFT_LOGGER_SERVICE_X, 0.0, KEEP_OUT_Z / 2.0);
    let right = centered_cube(
        "edge_guard_overlay_right_sample_service_keepout_gauge",
        7.0,
        DECK_Y - 230.0,
        KEEP_OUT_Z,
    )
    .translate(DECK_X / 2.0 - RIGHT_SAMPLE_SERVICE_X, 0.0, KEEP_OUT_Z / 2.0);
    let z_post = centered_cylinder(
        "edge_guard_overlay_top_pick_clearance_z_gauge",
        10.0,
        TOP_PICK_CLEARANCE_Z,
        28,
    )
    .translate(
        DECK_X / 2.0 - 92.0,
        -DECK_Y / 2.0 + 100.0,
        TOP_PICK_CLEARANCE_Z / 2.0,
    );
    front + rear + left + right + z_post
}

fn fiducial_disc(name: &str) -> Part {
    centered_cylinder(format!("{name}_outer_disc"), 10.0, 3.0, 32)
        - centered_cylinder(format!("{name}_center_dimple"), 3.5, 3.4, 24)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_scoped_and_unique() {
        let unique = OUTPUTS.iter().collect::<BTreeSet<_>>();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert!(OUTPUTS
            .iter()
            .all(|path| path.starts_with("output/") && path.ends_with(".stl")));
        assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
        assert!(OUTPUTS.last().unwrap().ends_with("_assembly.stl"));
    }

    #[test]
    fn requested_feature_set_is_present() {
        for required in [
            "cassette_edge_center_surrogate",
            "edge_lane_map",
            "center_position_map",
            "media_overlay_dams",
            "edge_guard_coupon_bank",
            "evaporation_witness_wells",
            "osmolality_sample_pockets",
            "rh_logger_docks",
            "fill_level_reference_ladder",
            "condensate_drip_witness",
            "release_gate",
            "hold_gate",
            "reject_gate",
        ] {
            assert!(
                REQUIRED_FEATURES.contains(&required),
                "missing feature marker {required}"
            );
        }
    }

    #[test]
    fn cassette_grid_counts_edge_and_center_positions() {
        assert_eq!(POSITION_COUNT, 20);
        assert_eq!(edge_position_count(), 14);
        assert_eq!(center_position_count(), 6);
        assert_eq!(position_zone(0), MapZone::Edge);
        assert_eq!(position_zone(6), MapZone::Center);
        assert_eq!(position_zone(10), MapZone::Center);
        assert_eq!(position_zone(19), MapZone::Edge);
    }

    #[test]
    fn module_footprints_fit_and_do_not_overlap() {
        let footprints = module_footprints();
        for item in footprints {
            assert!(item.fits_inside_deck(30.0), "{} is off deck", item.name);
        }
        for (i, a) in footprints.iter().enumerate() {
            for b in footprints.iter().skip(i + 1) {
                assert!(!a.overlaps(*b), "{} overlaps {}", a.name, b.name);
            }
        }
    }

    #[test]
    fn witness_and_gate_counts_match_study_layout() {
        assert_eq!(OVERLAY_DAM_COUPONS, 12);
        assert_eq!(DAM_HEIGHT_LEVELS_MM.len(), COUPON_ROWS);
        assert_eq!(EVAP_WELL_COUNT, POSITION_COUNT);
        assert_eq!(OSMO_SAMPLE_POCKETS, 24);
        assert_eq!(LOGGER_DOCKS, 6);
        assert_eq!(DRIP_CUPS, 10);
        assert_eq!(total_gate_token_slots(), 18);
    }

    #[test]
    fn dimensions_leave_clearance_and_containment_volume() {
        assert!(GRID_X < DECK_X * 0.45);
        assert!(GRID_Y < DECK_Y * 0.60);
        assert!(COUPON_SLOT_X < COUPON_PITCH_X);
        assert!(OSMO_POCKET_D + 12.0 < OSMO_PITCH_X);
        assert!(EVAP_WELL_D + 12.0 < EVAP_PITCH_X);
        assert!(containment_volume_ml() > 9_000.0);
        assert!(TOP_PICK_CLEARANCE_Z > GRID_Z + CHIP_SURROGATE_Z + 80.0);
    }
}
