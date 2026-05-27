use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed environmental probe placement shadow and strain validation station.
//
// Research assumptions captured in geometry:
// - Chamber mapping practice uses distributed corner/center/worst-case loggers,
//   including areas near doors, pass-throughs, shelves, and airflow features.
// - Reliable CO2/RH/temperature readings depend on probe placement, response
//   time, pressure/temperature effects, condensation avoidance, and comparison
//   with independent reference loggers.
// - Sensor feedthroughs should be treated as sealed mechanical interfaces, with
//   bend-radius control, pull/strain witnesses, leak witness wells, and custody
//   records before installed probes are trusted in a closed cabinet.
//
// This is fixture CAD only. Acceptance limits, probe calibration records, leak
// test pressure, sample-rate rules, and release authority remain validation
// protocol controls outside this generator.

const OUTPUT_PREFIX: &str =
    "output/closed_environmental_probe_placement_shadow_strain_validation_station_";

const OUTPUTS: [&str; 12] = [
    "output/closed_environmental_probe_placement_shadow_strain_validation_station_containment_deck.stl",
    "output/closed_environmental_probe_placement_shadow_strain_validation_station_scaled_cassette_position_shadow_map.stl",
    "output/closed_environmental_probe_placement_shadow_strain_validation_station_probe_mast_array.stl",
    "output/closed_environmental_probe_placement_shadow_strain_validation_station_airflow_shadow_blocker_bank.stl",
    "output/closed_environmental_probe_placement_shadow_strain_validation_station_cable_feedthrough_coupon_panel.stl",
    "output/closed_environmental_probe_placement_shadow_strain_validation_station_bend_radius_strain_gauge_array.stl",
    "output/closed_environmental_probe_placement_shadow_strain_validation_station_leak_test_witness_well_bank.stl",
    "output/closed_environmental_probe_placement_shadow_strain_validation_station_reference_logger_pockets.stl",
    "output/closed_environmental_probe_placement_shadow_strain_validation_station_thermal_rh_dummy_load_coupons.stl",
    "output/closed_environmental_probe_placement_shadow_strain_validation_station_edge_center_marker_camera_fiducial_board.stl",
    "output/closed_environmental_probe_placement_shadow_strain_validation_station_traceability_disposition_lanes.stl",
    "output/closed_environmental_probe_placement_shadow_strain_validation_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "containment_deck",
    "scaled_cassette_position_shadow_map",
    "probe_mast_array",
    "airflow_shadow_blocker_bank",
    "cable_feedthrough_coupon_panel",
    "bend_radius_strain_gauge_array",
    "leak_test_witness_well_bank",
    "reference_logger_pockets",
    "thermal_rh_dummy_load_coupons",
    "edge_center_marker_camera_fiducial_board",
    "traceability_disposition_lanes",
];

const LIMITATIONS: [&str; 6] = [
    "mechanical_fixture_only",
    "no_acceptance_limits",
    "no_calibration_protocol",
    "no_leak_test_pressure_claim",
    "no_sterility_claim",
    "no_biological_performance_claim",
];

const STATION_X: f64 = 1740.0;
const STATION_Y: f64 = 980.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const SOCKET_DEPTH: f64 = 6.0;
const MOUNT_HOLE_D: f64 = 6.6;
const MOUNT_HOLE_COUNT: usize = 8;
const DATUM_TARGET_COUNT: usize = 4;

const CASSETTE_COLS: usize = 4;
const CASSETTE_ROWS: usize = 4;
const CASSETTE_POSITION_COUNT: usize = CASSETTE_COLS * CASSETTE_ROWS;
const EDGE_POSITION_COUNT: usize = 12;
const CENTER_POSITION_COUNT: usize = CASSETTE_POSITION_COUNT - EDGE_POSITION_COUNT;
const CHIP_SCALE: f64 = 0.62;
const SCALED_CHIP_X: f64 = REVC_CHIP_LENGTH * CHIP_SCALE;
const SCALED_CHIP_Y: f64 = REVC_CHIP_WIDTH * CHIP_SCALE;
const CHIP_GUTTER_X: f64 = 16.0;
const CHIP_GUTTER_Y: f64 = 14.0;
const CASSETTE_PITCH_X: f64 = SCALED_CHIP_X + CHIP_GUTTER_X;
const CASSETTE_PITCH_Y: f64 = SCALED_CHIP_Y + CHIP_GUTTER_Y;
const CASSETTE_ARRAY_X: f64 =
    CASSETTE_COLS as f64 * SCALED_CHIP_X + (CASSETTE_COLS as f64 - 1.0) * CHIP_GUTTER_X;
const CASSETTE_ARRAY_Y: f64 =
    CASSETTE_ROWS as f64 * SCALED_CHIP_Y + (CASSETTE_ROWS as f64 - 1.0) * CHIP_GUTTER_Y;
const CASSETTE_MAP_X: f64 = 500.0;
const CASSETTE_MAP_Y: f64 = 360.0;
const CASSETTE_MAP_Z: f64 = 34.0;
const CASSETTE_MAP_POS: (f64, f64) = (-440.0, 135.0);
const CHIP_RECESS_DEPTH: f64 = 12.0;
const CABINET_SLOT_RAIL_W: f64 = 14.0;

const PROBE_KIND_COUNT: usize = 4;
const PROBE_ZONE_COUNT: usize = 3;
const PROBE_MAST_COUNT: usize = PROBE_KIND_COUNT * PROBE_ZONE_COUNT;
const PROBE_PANEL_X: f64 = 540.0;
const PROBE_PANEL_Y: f64 = 300.0;
const PROBE_PANEL_Z: f64 = 26.0;
const PROBE_PANEL_POS: (f64, f64) = (355.0, 175.0);
const PROBE_KIND_PITCH_X: f64 = 112.0;
const PROBE_ZONE_PITCH_Y: f64 = 72.0;
const MAST_OUTER_D: f64 = 18.0;
const MAST_HEIGHT: f64 = 118.0;
const MAST_COLLAR_Z: f64 = 10.0;
const MAST_COLLAR_D: f64 = 30.0;
const PROBE_PLACEMENT_CLEARANCE_Z: f64 = 96.0;

const SHADOW_BANK_X: f64 = 440.0;
const SHADOW_BANK_Y: f64 = 150.0;
const SHADOW_BANK_Z: f64 = 22.0;
const SHADOW_BANK_POS: (f64, f64) = (55.0, -100.0);
const SHADOW_BLOCKER_COUNT: usize = 6;
const SHADOW_BLOCKER_PITCH_X: f64 = 62.0;
const SHADOW_BLOCKER_X: f64 = 18.0;
const SHADOW_BLOCKER_Y: f64 = 106.0;
const SHADOW_BLOCKER_Z_BASE: f64 = 36.0;
const SHADOW_BLOCKER_Z_STEP: f64 = 10.0;
const AIRFLOW_RIBBON_COUNT: usize = 5;

const FEEDTHROUGH_PANEL_X: f64 = 360.0;
const FEEDTHROUGH_PANEL_Y: f64 = 190.0;
const FEEDTHROUGH_PANEL_Z: f64 = 22.0;
const FEEDTHROUGH_WALL_Z: f64 = 82.0;
const FEEDTHROUGH_PANEL_POS: (f64, f64) = (-560.0, -290.0);
const FEEDTHROUGH_COLS: usize = 4;
const FEEDTHROUGH_ROWS: usize = 2;
const FEEDTHROUGH_COUNT: usize = FEEDTHROUGH_COLS * FEEDTHROUGH_ROWS;
const FEEDTHROUGH_PITCH_X: f64 = 70.0;
const FEEDTHROUGH_PITCH_Z: f64 = 30.0;
const FEEDTHROUGH_BORE_D: f64 = 10.2;
const FEEDTHROUGH_SEAL_COLLAR_D: f64 = 24.0;

const BEND_GAUGE_X: f64 = 370.0;
const BEND_GAUGE_Y: f64 = 140.0;
const BEND_GAUGE_Z: f64 = 22.0;
const BEND_GAUGE_POS: (f64, f64) = (-125.0, -315.0);
const BEND_LANE_COUNT: usize = 4;
const BEND_MANDREL_COUNT: usize = 3;
const BEND_LANE_PITCH_Y: f64 = 28.0;
const BEND_MANDREL_PITCH_X: f64 = 92.0;
const BEND_MANDREL_RADII: [f64; BEND_MANDREL_COUNT] = [18.0, 26.0, 35.0];
const MIN_VALIDATED_BEND_RADIUS: f64 = 35.0;
const STRAIN_GAUGE_PAD_COUNT: usize = BEND_LANE_COUNT * 2;

const LEAK_WELL_X: f64 = 320.0;
const LEAK_WELL_Y: f64 = 140.0;
const LEAK_WELL_Z: f64 = 34.0;
const LEAK_WELL_POS: (f64, f64) = (280.0, -315.0);
const LEAK_WELL_COUNT: usize = FEEDTHROUGH_COUNT;
const LEAK_WELL_COLS: usize = 4;
const LEAK_WELL_PITCH_X: f64 = 66.0;
const LEAK_WELL_PITCH_Y: f64 = 44.0;
const LEAK_WELL_D: f64 = 26.0;
const LEAK_WELL_DEPTH: f64 = 18.0;

const LOGGER_PANEL_X: f64 = 250.0;
const LOGGER_PANEL_Y: f64 = 160.0;
const LOGGER_PANEL_Z: f64 = 34.0;
const LOGGER_PANEL_POS: (f64, f64) = (635.0, -110.0);
const LOGGER_POCKET_COUNT: usize = 4;
const LOGGER_POCKET_X: f64 = 72.0;
const LOGGER_POCKET_Y: f64 = 46.0;
const LOGGER_POCKET_DEPTH: f64 = 16.0;
const LOGGER_PITCH_X: f64 = 92.0;
const LOGGER_PITCH_Y: f64 = 58.0;

const DUMMY_LOAD_X: f64 = 320.0;
const DUMMY_LOAD_Y: f64 = 120.0;
const DUMMY_LOAD_Z: f64 = 18.0;
const DUMMY_LOAD_POS: (f64, f64) = (-565.0, 385.0);
const DUMMY_COUPON_COUNT: usize = CASSETTE_POSITION_COUNT;
const DUMMY_COUPON_COLS: usize = 8;
const DUMMY_COUPON_X: f64 = 24.0;
const DUMMY_COUPON_Y: f64 = 18.0;
const DUMMY_COUPON_Z: f64 = 8.0;
const DUMMY_COUPON_PITCH_X: f64 = 34.0;
const DUMMY_COUPON_PITCH_Y: f64 = 38.0;
const RH_WICK_WELL_COUNT: usize = 8;

const MARKER_BOARD_X: f64 = 60.0;
const MARKER_BOARD_Y: f64 = 360.0;
const MARKER_BOARD_Z: f64 = 16.0;
const MARKER_BOARD_POS: (f64, f64) = (-780.0, 115.0);
const POSITION_MARKER_COUNT: usize = CASSETTE_POSITION_COUNT;
const CAMERA_FIDUCIAL_COUNT: usize = 4;
const ILLUMINATION_BALANCE_LAND_COUNT: usize = 2;

const TRACE_PANEL_X: f64 = 620.0;
const TRACE_PANEL_Y: f64 = 90.0;
const TRACE_PANEL_Z: f64 = 24.0;
const TRACE_PANEL_POS: (f64, f64) = (220.0, 405.0);
const BARCODE_LAND_COUNT: usize = 6;
const CERTIFICATE_LAND_COUNT: usize = 3;
const DISPOSITION_LANE_COUNT: usize = 3;
const DISPOSITION_LANE_NAMES: [&str; DISPOSITION_LANE_COUNT] = ["release", "hold", "reject"];
const DISPOSITION_SLOT_COUNT_PER_LANE: usize = 4;
const DISPOSITION_TOTAL_CAPACITY: usize = DISPOSITION_LANE_COUNT * DISPOSITION_SLOT_COUNT_PER_LANE;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ProbeKind {
    Temperature,
    Rh,
    Co2,
    O2,
}

impl ProbeKind {
    fn all() -> [ProbeKind; PROBE_KIND_COUNT] {
        [
            ProbeKind::Temperature,
            ProbeKind::Rh,
            ProbeKind::Co2,
            ProbeKind::O2,
        ]
    }

    fn index(self) -> usize {
        match self {
            ProbeKind::Temperature => 0,
            ProbeKind::Rh => 1,
            ProbeKind::Co2 => 2,
            ProbeKind::O2 => 3,
        }
    }

    fn label(self) -> &'static str {
        match self {
            ProbeKind::Temperature => "temperature",
            ProbeKind::Rh => "rh",
            ProbeKind::Co2 => "co2",
            ProbeKind::O2 => "o2",
        }
    }

    fn bore_d(self) -> f64 {
        match self {
            ProbeKind::Temperature => 5.4,
            ProbeKind::Rh => 8.2,
            ProbeKind::Co2 => 10.4,
            ProbeKind::O2 => 8.8,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PlacementZone {
    Edge,
    Center,
    Exhaust,
}

impl PlacementZone {
    fn all() -> [PlacementZone; PROBE_ZONE_COUNT] {
        [
            PlacementZone::Edge,
            PlacementZone::Center,
            PlacementZone::Exhaust,
        ]
    }

    fn index(self) -> usize {
        match self {
            PlacementZone::Edge => 0,
            PlacementZone::Center => 1,
            PlacementZone::Exhaust => 2,
        }
    }

    fn label(self) -> &'static str {
        match self {
            PlacementZone::Edge => "edge",
            PlacementZone::Center => "center",
            PlacementZone::Exhaust => "exhaust",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CassetteZone {
    Edge,
    Center,
}

impl CassetteZone {
    fn label(self) -> &'static str {
        match self {
            CassetteZone::Edge => "edge",
            CassetteZone::Center => "center",
        }
    }
}

#[derive(Clone, Copy)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_station(self) -> bool {
        let usable_x = STATION_X / 2.0 - RIM_W - 14.0;
        let usable_y = STATION_Y / 2.0 - RIM_W - 14.0;

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
    assert_design_constraints();

    let deck = containment_deck();
    export(OUTPUTS[0], &deck);

    let map = scaled_cassette_position_shadow_map();
    export(OUTPUTS[1], &map);

    let masts = probe_mast_array();
    export(OUTPUTS[2], &masts);

    let blockers = airflow_shadow_blocker_bank();
    export(OUTPUTS[3], &blockers);

    let feedthroughs = cable_feedthrough_coupon_panel();
    export(OUTPUTS[4], &feedthroughs);

    let bend_gauges = bend_radius_strain_gauge_array();
    export(OUTPUTS[5], &bend_gauges);

    let leak_wells = leak_test_witness_well_bank();
    export(OUTPUTS[6], &leak_wells);

    let loggers = reference_logger_pockets();
    export(OUTPUTS[7], &loggers);

    let dummy_loads = thermal_rh_dummy_load_coupons();
    export(OUTPUTS[8], &dummy_loads);

    let markers = edge_center_marker_camera_fiducial_board();
    export(OUTPUTS[9], &markers);

    let traceability = traceability_disposition_lanes();
    export(OUTPUTS[10], &traceability);

    let assembly = station_assembly();
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed environmental probe placement shadow and strain validation station:");
    println!("  Footprint:             {STATION_X:.0}mm x {STATION_Y:.0}mm contained cabinet deck");
    println!(
        "  Cassette map:          {CASSETTE_COLS}x{CASSETTE_ROWS} scaled positions, {EDGE_POSITION_COUNT} edge / {CENTER_POSITION_COUNT} center"
    );
    println!(
        "  Probe masts:           {PROBE_MAST_COUNT} mast sockets covering temperature, RH, CO2, and O2 across {} placement zones",
        PlacementZone::all().len()
    );
    println!(
        "  Airflow/loads:         {SHADOW_BLOCKER_COUNT} shadow blockers, {AIRFLOW_RIBBON_COUNT} flow ribbons, {DUMMY_COUPON_COUNT} thermal/RH dummy coupons"
    );
    println!(
        "  Cable validation:      {FEEDTHROUGH_COUNT} feedthrough coupons, {BEND_LANE_COUNT} bend lanes, {STRAIN_GAUGE_PAD_COUNT} strain pads, {LEAK_WELL_COUNT} leak witness wells"
    );
    println!(
        "  Evidence/release:      {LOGGER_POCKET_COUNT} logger pockets, {CAMERA_FIDUCIAL_COUNT} camera fiducials, {BARCODE_LAND_COUNT} barcode lands, {CERTIFICATE_LAND_COUNT} certificate lands, {DISPOSITION_TOTAL_CAPACITY} disposition token slots"
    );
    println!(
        "  Limitations:           mechanical fixture only; protocol limits, traceability, leak pressure, sterility, and biological performance are external controls"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    containment_deck()
        + scaled_cassette_position_shadow_map().translate(
            CASSETTE_MAP_POS.0,
            CASSETTE_MAP_POS.1,
            module_base_z(),
        )
        + probe_mast_array().translate(PROBE_PANEL_POS.0, PROBE_PANEL_POS.1, module_base_z())
        + airflow_shadow_blocker_bank().translate(
            SHADOW_BANK_POS.0,
            SHADOW_BANK_POS.1,
            module_base_z(),
        )
        + cable_feedthrough_coupon_panel().translate(
            FEEDTHROUGH_PANEL_POS.0,
            FEEDTHROUGH_PANEL_POS.1,
            module_base_z(),
        )
        + bend_radius_strain_gauge_array().translate(
            BEND_GAUGE_POS.0,
            BEND_GAUGE_POS.1,
            module_base_z(),
        )
        + leak_test_witness_well_bank().translate(LEAK_WELL_POS.0, LEAK_WELL_POS.1, module_base_z())
        + reference_logger_pockets().translate(
            LOGGER_PANEL_POS.0,
            LOGGER_PANEL_POS.1,
            module_base_z(),
        )
        + thermal_rh_dummy_load_coupons().translate(
            DUMMY_LOAD_POS.0,
            DUMMY_LOAD_POS.1,
            module_base_z(),
        )
        + edge_center_marker_camera_fiducial_board().translate(
            MARKER_BOARD_POS.0,
            MARKER_BOARD_POS.1,
            module_base_z(),
        )
        + traceability_disposition_lanes().translate(
            TRACE_PANEL_POS.0,
            TRACE_PANEL_POS.1,
            module_base_z(),
        )
}

fn module_base_z() -> f64 {
    BASE_Z - SOCKET_DEPTH
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn assert_design_constraints() {
    assert_eq!(CASSETTE_POSITION_COUNT, 16);
    assert_eq!(edge_position_count(), EDGE_POSITION_COUNT);
    assert_eq!(center_position_count(), CENTER_POSITION_COUNT);
    assert_eq!(ProbeKind::all().len(), PROBE_KIND_COUNT);
    assert_eq!(PlacementZone::all().len(), PROBE_ZONE_COUNT);
    assert_eq!(PROBE_MAST_COUNT, PROBE_KIND_COUNT * PROBE_ZONE_COUNT);
    assert_eq!(FEEDTHROUGH_COUNT, FEEDTHROUGH_COLS * FEEDTHROUGH_ROWS);
    assert_eq!(LEAK_WELL_COUNT, FEEDTHROUGH_COUNT);
    assert_eq!(LOGGER_POCKET_COUNT, PROBE_KIND_COUNT);
    assert_eq!(DUMMY_COUPON_COUNT, CASSETTE_POSITION_COUNT);
    assert_eq!(POSITION_MARKER_COUNT, CASSETTE_POSITION_COUNT);
    assert_eq!(MOUNT_HOLE_COUNT, 8);
    assert_eq!(DATUM_TARGET_COUNT, 4);
    assert_eq!(DISPOSITION_LANE_NAMES, ["release", "hold", "reject"]);
    assert!(OUTPUTS.iter().all(|path| path.starts_with(OUTPUT_PREFIX)));
    assert!(REVC_TOTAL_HEIGHT < CASSETTE_MAP_Z);
    assert!(CASSETTE_ARRAY_X < CASSETTE_MAP_X - 72.0);
    assert!(CASSETTE_ARRAY_Y < CASSETTE_MAP_Y - 64.0);
    assert!(PROBE_PLACEMENT_CLEARANCE_Z > MAST_HEIGHT / 2.0);
    assert_eq!(
        validated_bend_radius(),
        MIN_VALIDATED_BEND_RADIUS,
        "highest bend mandrel must represent the validated minimum radius"
    );
    assert_eq!(REQUIRED_FEATURES.len(), 11);
    assert_eq!(LIMITATIONS.len(), 6);
    assert!(module_footprints_fit_station());
    assert!(!critical_modules_overlap());
}

fn containment_deck() -> Part {
    let plate = centered_cube(
        "probe_shadow_strain_station_containment_deck_plate",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);

    plate - deck_washdown_recess() - module_socket_recesses() - mounting_holes() - drain_slot()
        + perimeter_rims()
        + cabinet_datum_rails()
        + deck_flow_direction_ribbons()
        + deck_datum_targets()
}

fn deck_washdown_recess() -> Part {
    centered_cube(
        "probe_shadow_strain_station_washdown_recess",
        STATION_X - 130.0,
        STATION_Y - 126.0,
        5.0,
    )
    .translate(0.0, -6.0, BASE_Z - 2.5 + 0.2)
}

fn module_socket_recesses() -> Part {
    let mut sockets = Part::empty("probe_shadow_strain_station_module_socket_recesses");
    for rect in module_rects() {
        sockets = sockets
            + centered_cube(
                format!("probe_shadow_strain_station_{}_socket", rect.name),
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
    sockets
}

fn mounting_holes() -> Part {
    let mut holes = Part::empty("probe_shadow_strain_station_mounting_holes");
    let points = [
        (-STATION_X / 2.0 + 54.0, -STATION_Y / 2.0 + 54.0),
        (STATION_X / 2.0 - 54.0, -STATION_Y / 2.0 + 54.0),
        (-STATION_X / 2.0 + 54.0, STATION_Y / 2.0 - 54.0),
        (STATION_X / 2.0 - 54.0, STATION_Y / 2.0 - 54.0),
        (0.0, -STATION_Y / 2.0 + 54.0),
        (0.0, STATION_Y / 2.0 - 54.0),
        (-STATION_X / 2.0 + 54.0, 0.0),
        (STATION_X / 2.0 - 54.0, 0.0),
    ];
    for (index, (x, y)) in points.into_iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("probe_shadow_strain_station_m6_mount_hole_{index}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 4.0,
                32,
            )
            .translate(x, y, BASE_Z / 2.0);
    }
    holes
}

fn drain_slot() -> Part {
    centered_cube(
        "probe_shadow_strain_station_front_leak_capture_drain_slot",
        92.0,
        12.0,
        BASE_Z + 4.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 24.0, BASE_Z / 2.0)
}

fn perimeter_rims() -> Part {
    let front = centered_cube(
        "probe_shadow_strain_station_front_low_spill_lip",
        STATION_X - 120.0,
        12.0,
        24.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 22.0, BASE_Z + 12.0);
    let rear = centered_cube(
        "probe_shadow_strain_station_rear_service_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "probe_shadow_strain_station_left_spill_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "probe_shadow_strain_station_right_spill_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    front + rear + left + right
}

fn cabinet_datum_rails() -> Part {
    let left = centered_cube(
        "probe_shadow_strain_station_clean_box_left_cassette_datum_rail",
        CABINET_SLOT_RAIL_W,
        CASSETTE_MAP_Y + 52.0,
        26.0,
    )
    .translate(
        CASSETTE_MAP_POS.0 - CASSETTE_MAP_X / 2.0 - 26.0,
        CASSETTE_MAP_POS.1,
        BASE_Z + 13.0,
    );
    let right = centered_cube(
        "probe_shadow_strain_station_clean_box_right_cassette_datum_rail",
        CABINET_SLOT_RAIL_W,
        CASSETTE_MAP_Y + 52.0,
        26.0,
    )
    .translate(
        CASSETTE_MAP_POS.0 + CASSETTE_MAP_X / 2.0 + 26.0,
        CASSETTE_MAP_POS.1,
        BASE_Z + 13.0,
    );
    let rear = centered_cube(
        "probe_shadow_strain_station_rear_fan_plane_reference_rail",
        CASSETTE_MAP_X + 140.0,
        12.0,
        24.0,
    )
    .translate(
        CASSETTE_MAP_POS.0,
        CASSETTE_MAP_POS.1 + CASSETTE_MAP_Y / 2.0 + 28.0,
        BASE_Z + 12.0,
    );
    let pass_through = centered_cube(
        "probe_shadow_strain_station_feedthrough_bulkhead_reference_rail",
        FEEDTHROUGH_PANEL_X + 80.0,
        12.0,
        22.0,
    )
    .translate(
        FEEDTHROUGH_PANEL_POS.0,
        FEEDTHROUGH_PANEL_POS.1 + FEEDTHROUGH_PANEL_Y / 2.0 + 26.0,
        BASE_Z + 11.0,
    );

    left + right + rear + pass_through
}

fn deck_flow_direction_ribbons() -> Part {
    let mut ribbons = Part::empty("probe_shadow_strain_station_deck_flow_direction_ribbons");
    for index in 0..AIRFLOW_RIBBON_COUNT {
        let y = CASSETTE_MAP_POS.1 - 118.0 + index as f64 * 56.0;
        let shaft = centered_cube(
            format!("probe_shadow_strain_station_airflow_arrow_{index}_shaft"),
            132.0,
            6.0,
            6.0,
        )
        .translate(CASSETTE_MAP_POS.0 + 430.0, y, BASE_Z + 3.0);
        let head = centered_cube(
            format!("probe_shadow_strain_station_airflow_arrow_{index}_head"),
            18.0,
            18.0,
            6.0,
        )
        .translate(CASSETTE_MAP_POS.0 + 506.0, y, BASE_Z + 3.0);
        ribbons = ribbons + shaft + head;
    }
    ribbons
}

fn deck_datum_targets() -> Part {
    let mut targets = Part::empty("probe_shadow_strain_station_deck_camera_datum_targets");
    let points = [
        (-STATION_X / 2.0 + 82.0, -STATION_Y / 2.0 + 82.0),
        (STATION_X / 2.0 - 82.0, -STATION_Y / 2.0 + 82.0),
        (-STATION_X / 2.0 + 82.0, STATION_Y / 2.0 - 82.0),
        (STATION_X / 2.0 - 82.0, STATION_Y / 2.0 - 82.0),
    ];
    for (index, (x, y)) in points.into_iter().enumerate() {
        targets = targets
            + fiducial_disc(&format!(
                "probe_shadow_strain_station_deck_datum_target_{index}"
            ))
            .translate(x, y, BASE_Z + 2.0);
    }
    targets
}

fn scaled_cassette_position_shadow_map() -> Part {
    let body = centered_cube(
        "probe_shadow_strain_station_scaled_cassette_map_base",
        CASSETTE_MAP_X,
        CASSETTE_MAP_Y,
        CASSETTE_MAP_Z,
    )
    .translate(0.0, 0.0, CASSETTE_MAP_Z / 2.0);

    let mut recesses = Part::empty("probe_shadow_strain_station_scaled_cassette_recesses");
    let mut features = Part::empty("probe_shadow_strain_station_scaled_cassette_features");

    for position in 0..CASSETTE_POSITION_COUNT {
        let (x, y) = cassette_position_xy(position);
        let zone = cassette_zone(position);
        recesses = recesses
            + centered_cube(
                format!(
                    "probe_shadow_strain_station_position_{position:02}_{}_scaled_chip_recess",
                    zone.label()
                ),
                SCALED_CHIP_X + 7.0,
                SCALED_CHIP_Y + 7.0,
                CHIP_RECESS_DEPTH + 1.0,
            )
            .translate(x, y, CASSETTE_MAP_Z - CHIP_RECESS_DEPTH / 2.0 + 0.5);

        let shadow_witness = centered_cube(
            format!(
                "probe_shadow_strain_station_position_{position:02}_{}_air_shadow_witness_bar",
                zone.label()
            ),
            SCALED_CHIP_X - 18.0,
            5.0,
            5.0,
        )
        .translate(x, y + SCALED_CHIP_Y / 2.0 + 9.0, CASSETTE_MAP_Z + 2.5);
        let gradient_pin = centered_cylinder(
            format!(
                "probe_shadow_strain_station_position_{position:02}_{}_thermal_gradient_pin",
                zone.label()
            ),
            if zone == CassetteZone::Edge { 4.5 } else { 6.5 },
            5.0,
            28,
        )
        .translate(x - SCALED_CHIP_X / 2.0 + 13.0, y, CASSETTE_MAP_Z + 2.5);
        features = features + shadow_witness + gradient_pin;
    }

    body - recesses + cassette_map_datum_rails() + features
}

fn cassette_map_datum_rails() -> Part {
    let front = centered_cube(
        "probe_shadow_strain_station_scaled_cassette_front_datum_rail",
        CASSETTE_ARRAY_X + 42.0,
        10.0,
        18.0,
    )
    .translate(0.0, -CASSETTE_ARRAY_Y / 2.0 - 24.0, CASSETTE_MAP_Z + 9.0);
    let rear = centered_cube(
        "probe_shadow_strain_station_scaled_cassette_rear_datum_rail",
        CASSETTE_ARRAY_X + 42.0,
        10.0,
        18.0,
    )
    .translate(0.0, CASSETTE_ARRAY_Y / 2.0 + 24.0, CASSETTE_MAP_Z + 9.0);
    let left = centered_cube(
        "probe_shadow_strain_station_scaled_cassette_left_slot_rail",
        10.0,
        CASSETTE_ARRAY_Y + 42.0,
        18.0,
    )
    .translate(-CASSETTE_ARRAY_X / 2.0 - 24.0, 0.0, CASSETTE_MAP_Z + 9.0);
    let right = centered_cube(
        "probe_shadow_strain_station_scaled_cassette_right_slot_rail",
        10.0,
        CASSETTE_ARRAY_Y + 42.0,
        18.0,
    )
    .translate(CASSETTE_ARRAY_X / 2.0 + 24.0, 0.0, CASSETTE_MAP_Z + 9.0);

    front + rear + left + right
}

fn probe_mast_array() -> Part {
    let panel = centered_cube(
        "probe_shadow_strain_station_probe_mast_array_panel",
        PROBE_PANEL_X,
        PROBE_PANEL_Y,
        PROBE_PANEL_Z,
    )
    .translate(0.0, 0.0, PROBE_PANEL_Z / 2.0);

    let mut cuts = Part::empty("probe_shadow_strain_station_probe_mast_panel_cuts");
    let mut masts = Part::empty("probe_shadow_strain_station_probe_masts");

    for kind in ProbeKind::all() {
        for zone in PlacementZone::all() {
            let (x, y) = probe_mast_xy(kind, zone);
            let label = format!(
                "probe_shadow_strain_station_{}_{}_probe_mast",
                kind.label(),
                zone.label()
            );

            cuts = cuts
                + centered_cylinder(
                    format!("{label}_panel_clearance_bore"),
                    kind.bore_d() / 2.0,
                    PROBE_PANEL_Z + 2.0,
                    30,
                )
                .translate(x, y, PROBE_PANEL_Z / 2.0);

            let mast_shell = centered_cylinder(
                format!("{label}_vertical_sleeve"),
                MAST_OUTER_D / 2.0,
                MAST_HEIGHT,
                36,
            )
            .translate(x, y, PROBE_PANEL_Z + MAST_HEIGHT / 2.0);
            let mast_bore = centered_cylinder(
                format!("{label}_probe_bore"),
                kind.bore_d() / 2.0,
                MAST_HEIGHT + 4.0,
                30,
            )
            .translate(x, y, PROBE_PANEL_Z + MAST_HEIGHT / 2.0);
            let collar = centered_cylinder(
                format!("{label}_height_stop_collar"),
                MAST_COLLAR_D / 2.0,
                MAST_COLLAR_Z,
                36,
            )
            .translate(x, y, PROBE_PANEL_Z + 28.0);
            let cable_flag =
                centered_cube(format!("{label}_rear_cable_exit_flag"), 24.0, 8.0, 18.0).translate(
                    x,
                    y + 24.0,
                    PROBE_PANEL_Z + 26.0,
                );
            let shadow_keepout = centered_cube(
                format!("{label}_local_air_shadow_keepout_vane"),
                7.0,
                44.0,
                42.0,
            )
            .translate(x + 18.0, y, PROBE_PANEL_Z + 44.0);

            masts = masts + (mast_shell - mast_bore) + collar + cable_flag + shadow_keepout;
        }
    }

    let front_reference_bar = centered_cube(
        "probe_shadow_strain_station_probe_array_front_probe_tip_reference_bar",
        PROBE_PANEL_X - 56.0,
        8.0,
        16.0,
    )
    .translate(0.0, -PROBE_PANEL_Y / 2.0 + 22.0, PROBE_PANEL_Z + 8.0);

    panel - cuts + masts + front_reference_bar + gripper_fiducials("probe_mast_array", 420.0)
}

fn airflow_shadow_blocker_bank() -> Part {
    let panel = centered_cube(
        "probe_shadow_strain_station_airflow_shadow_blocker_bank_panel",
        SHADOW_BANK_X,
        SHADOW_BANK_Y,
        SHADOW_BANK_Z,
    )
    .translate(0.0, 0.0, SHADOW_BANK_Z / 2.0);

    let mut blockers = Part::empty("probe_shadow_strain_station_airflow_shadow_blockers");
    for index in 0..SHADOW_BLOCKER_COUNT {
        let x = centered_index(index, SHADOW_BLOCKER_COUNT, SHADOW_BLOCKER_PITCH_X);
        let blocker_z = SHADOW_BLOCKER_Z_BASE + index as f64 * SHADOW_BLOCKER_Z_STEP;
        let blocker = centered_cube(
            format!("probe_shadow_strain_station_shadow_blocker_{index}_removable_plate"),
            SHADOW_BLOCKER_X,
            SHADOW_BLOCKER_Y,
            blocker_z,
        )
        .translate(x, 0.0, SHADOW_BANK_Z + blocker_z / 2.0);
        let top_tab = centered_cube(
            format!("probe_shadow_strain_station_shadow_blocker_{index}_height_id_tab"),
            30.0,
            12.0,
            8.0,
        )
        .translate(
            x,
            SHADOW_BLOCKER_Y / 2.0 + 7.0,
            SHADOW_BANK_Z + blocker_z + 4.0,
        );
        blockers = blockers + blocker + top_tab;
    }

    let fan_side_rail = centered_cube(
        "probe_shadow_strain_station_blocker_bank_fan_side_reference_rail",
        SHADOW_BANK_X - 28.0,
        8.0,
        22.0,
    )
    .translate(0.0, SHADOW_BANK_Y / 2.0 - 18.0, SHADOW_BANK_Z + 11.0);
    let cassette_side_rail = centered_cube(
        "probe_shadow_strain_station_blocker_bank_cassette_side_reference_rail",
        SHADOW_BANK_X - 28.0,
        8.0,
        22.0,
    )
    .translate(0.0, -SHADOW_BANK_Y / 2.0 + 18.0, SHADOW_BANK_Z + 11.0);

    panel + blockers + fan_side_rail + cassette_side_rail
}

fn cable_feedthrough_coupon_panel() -> Part {
    let base = centered_cube(
        "probe_shadow_strain_station_feedthrough_coupon_base",
        FEEDTHROUGH_PANEL_X,
        FEEDTHROUGH_PANEL_Y,
        FEEDTHROUGH_PANEL_Z,
    )
    .translate(0.0, 0.0, FEEDTHROUGH_PANEL_Z / 2.0);
    let wall = centered_cube(
        "probe_shadow_strain_station_feedthrough_coupon_bulkhead_wall",
        FEEDTHROUGH_PANEL_X,
        18.0,
        FEEDTHROUGH_WALL_Z,
    )
    .translate(
        0.0,
        FEEDTHROUGH_PANEL_Y / 2.0 - 18.0,
        FEEDTHROUGH_PANEL_Z + FEEDTHROUGH_WALL_Z / 2.0,
    );

    let mut bores = Part::empty("probe_shadow_strain_station_feedthrough_bores");
    let mut collars = Part::empty("probe_shadow_strain_station_feedthrough_seal_collars");

    for index in 0..FEEDTHROUGH_COUNT {
        let col = index % FEEDTHROUGH_COLS;
        let row = index / FEEDTHROUGH_COLS;
        let x = centered_index(col, FEEDTHROUGH_COLS, FEEDTHROUGH_PITCH_X);
        let z = FEEDTHROUGH_PANEL_Z + 28.0 + row as f64 * FEEDTHROUGH_PITCH_Z;
        let y = FEEDTHROUGH_PANEL_Y / 2.0 - 18.0;

        bores = bores
            + centered_cylinder(
                format!("probe_shadow_strain_station_feedthrough_coupon_{index}_cable_bore"),
                FEEDTHROUGH_BORE_D / 2.0,
                28.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, y, z);
        collars = collars
            + centered_cylinder(
                format!("probe_shadow_strain_station_feedthrough_coupon_{index}_seal_collar"),
                FEEDTHROUGH_SEAL_COLLAR_D / 2.0,
                5.0,
                36,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, y - 12.0, z)
            + centered_cube(
                format!("probe_shadow_strain_station_feedthrough_coupon_{index}_pull_witness_tab"),
                28.0,
                12.0,
                10.0,
            )
            .translate(x, y - 28.0, z - 18.0);
    }

    let wall_with_bores = wall - bores;
    base + wall_with_bores + collars + gripper_fiducials("feedthrough_coupon_panel", 260.0)
}

fn bend_radius_strain_gauge_array() -> Part {
    let panel = centered_cube(
        "probe_shadow_strain_station_bend_radius_gauge_panel",
        BEND_GAUGE_X,
        BEND_GAUGE_Y,
        BEND_GAUGE_Z,
    )
    .translate(0.0, 0.0, BEND_GAUGE_Z / 2.0);

    let mut cuts = Part::empty("probe_shadow_strain_station_bend_lane_reliefs");
    let mut features = Part::empty("probe_shadow_strain_station_bend_mandrels_and_strain_pads");

    for lane in 0..BEND_LANE_COUNT {
        let y = centered_index(lane, BEND_LANE_COUNT, BEND_LANE_PITCH_Y);
        cuts = cuts
            + centered_cube(
                format!("probe_shadow_strain_station_bend_lane_{lane}_cable_channel"),
                BEND_GAUGE_X - 46.0,
                9.0,
                7.0,
            )
            .translate(0.0, y, BEND_GAUGE_Z - 3.5 + 0.2);

        for (mandrel_index, radius) in BEND_MANDREL_RADII.into_iter().enumerate() {
            let x = centered_index(mandrel_index, BEND_MANDREL_COUNT, BEND_MANDREL_PITCH_X);
            let mandrel = centered_cylinder(
                format!("probe_shadow_strain_station_bend_lane_{lane}_radius_{radius:.0}_mandrel"),
                radius,
                14.0,
                48,
            )
            .translate(x, y, BEND_GAUGE_Z + 7.0);
            let cable_guard = centered_cylinder(
                format!(
                    "probe_shadow_strain_station_bend_lane_{lane}_radius_{radius:.0}_retainer_cap"
                ),
                radius + 4.0,
                4.0,
                48,
            )
            .translate(x, y, BEND_GAUGE_Z + 18.0);
            features = features + mandrel + cable_guard;
        }

        for side in 0..2 {
            let x = if side == 0 {
                -BEND_GAUGE_X / 2.0 + 30.0
            } else {
                BEND_GAUGE_X / 2.0 - 30.0
            };
            features = features
                + centered_cube(
                    format!("probe_shadow_strain_station_bend_lane_{lane}_strain_pad_{side}"),
                    34.0,
                    16.0,
                    5.0,
                )
                .translate(x, y, BEND_GAUGE_Z + 2.5);
        }
    }

    panel - cuts + features
}

fn leak_test_witness_well_bank() -> Part {
    let body = centered_cube(
        "probe_shadow_strain_station_leak_witness_well_bank_body",
        LEAK_WELL_X,
        LEAK_WELL_Y,
        LEAK_WELL_Z,
    )
    .translate(0.0, 0.0, LEAK_WELL_Z / 2.0);

    let mut well_cuts = Part::empty("probe_shadow_strain_station_leak_witness_well_cuts");
    let mut features = Part::empty("probe_shadow_strain_station_leak_witness_features");
    for index in 0..LEAK_WELL_COUNT {
        let col = index % LEAK_WELL_COLS;
        let row = index / LEAK_WELL_COLS;
        let x = centered_index(col, LEAK_WELL_COLS, LEAK_WELL_PITCH_X);
        let y = centered_index(row, FEEDTHROUGH_ROWS, LEAK_WELL_PITCH_Y);
        well_cuts = well_cuts
            + centered_cylinder(
                format!("probe_shadow_strain_station_leak_witness_well_{index}"),
                LEAK_WELL_D / 2.0,
                LEAK_WELL_DEPTH + 1.0,
                36,
            )
            .translate(x, y, LEAK_WELL_Z - LEAK_WELL_DEPTH / 2.0 + 0.5);

        features = features
            + centered_cube(
                format!("probe_shadow_strain_station_leak_witness_well_{index}_cap_land"),
                36.0,
                8.0,
                4.0,
            )
            .translate(x, y - LEAK_WELL_D / 2.0 - 9.0, LEAK_WELL_Z + 2.0);
    }

    let dye_moat = rectangular_frame(
        "probe_shadow_strain_station_leak_witness_dye_moat",
        LEAK_WELL_X - 34.0,
        LEAK_WELL_Y - 28.0,
        7.0,
        6.0,
    )
    .translate(0.0, 0.0, LEAK_WELL_Z + 3.5);

    body - well_cuts + features + dye_moat
}

fn reference_logger_pockets() -> Part {
    let body = centered_cube(
        "probe_shadow_strain_station_reference_logger_pocket_body",
        LOGGER_PANEL_X,
        LOGGER_PANEL_Y,
        LOGGER_PANEL_Z,
    )
    .translate(0.0, 0.0, LOGGER_PANEL_Z / 2.0);

    let mut cuts = Part::empty("probe_shadow_strain_station_reference_logger_pocket_cuts");
    let mut features = Part::empty("probe_shadow_strain_station_reference_logger_features");

    for index in 0..LOGGER_POCKET_COUNT {
        let col = index % 2;
        let row = index / 2;
        let x = centered_index(col, 2, LOGGER_PITCH_X);
        let y = centered_index(row, 2, LOGGER_PITCH_Y);
        let label = match index {
            0 => "temperature",
            1 => "rh",
            2 => "co2",
            _ => "o2",
        };

        cuts = cuts
            + centered_cube(
                format!("probe_shadow_strain_station_reference_logger_{label}_pocket"),
                LOGGER_POCKET_X,
                LOGGER_POCKET_Y,
                LOGGER_POCKET_DEPTH + 1.0,
            )
            .translate(x, y, LOGGER_PANEL_Z - LOGGER_POCKET_DEPTH / 2.0 + 0.5);

        features = features
            + centered_cube(
                format!("probe_shadow_strain_station_reference_logger_{label}_lock_tab"),
                42.0,
                8.0,
                8.0,
            )
            .translate(x, y + LOGGER_POCKET_Y / 2.0 + 7.0, LOGGER_PANEL_Z + 4.0)
            + fiducial_disc(&format!(
                "probe_shadow_strain_station_reference_logger_{label}_camera_dot"
            ))
            .translate(x, y - LOGGER_POCKET_Y / 2.0 - 12.0, LOGGER_PANEL_Z + 2.0);
    }

    body - cuts + features
}

fn thermal_rh_dummy_load_coupons() -> Part {
    let panel = centered_cube(
        "probe_shadow_strain_station_thermal_rh_dummy_load_panel",
        DUMMY_LOAD_X,
        DUMMY_LOAD_Y,
        DUMMY_LOAD_Z,
    )
    .translate(0.0, 0.0, DUMMY_LOAD_Z / 2.0);

    let mut coupons = Part::empty("probe_shadow_strain_station_thermal_rh_dummy_coupons");
    for index in 0..DUMMY_COUPON_COUNT {
        let col = index % DUMMY_COUPON_COLS;
        let row = index / DUMMY_COUPON_COLS;
        let x = centered_index(col, DUMMY_COUPON_COLS, DUMMY_COUPON_PITCH_X);
        let y = centered_index(
            row,
            DUMMY_COUPON_COUNT / DUMMY_COUPON_COLS,
            DUMMY_COUPON_PITCH_Y,
        );
        let zone = cassette_zone(index);
        let coupon_z = if zone == CassetteZone::Edge {
            DUMMY_COUPON_Z
        } else {
            DUMMY_COUPON_Z + 4.0
        };
        coupons = coupons
            + centered_cube(
                format!(
                    "probe_shadow_strain_station_dummy_load_coupon_{index:02}_{}",
                    zone.label()
                ),
                DUMMY_COUPON_X,
                DUMMY_COUPON_Y,
                coupon_z,
            )
            .translate(x, y, DUMMY_LOAD_Z + coupon_z / 2.0);
    }

    let mut wick_wells = Part::empty("probe_shadow_strain_station_rh_wick_witness_wells");
    for index in 0..RH_WICK_WELL_COUNT {
        let x = centered_index(index, RH_WICK_WELL_COUNT, 34.0);
        wick_wells = wick_wells
            + centered_cylinder(
                format!("probe_shadow_strain_station_rh_wick_well_{index}"),
                7.0,
                DUMMY_LOAD_Z + 2.0,
                28,
            )
            .translate(x, -DUMMY_LOAD_Y / 2.0 + 20.0, DUMMY_LOAD_Z / 2.0);
    }

    panel - wick_wells + coupons
}

fn edge_center_marker_camera_fiducial_board() -> Part {
    let board = centered_cube(
        "probe_shadow_strain_station_edge_center_marker_fiducial_board",
        MARKER_BOARD_X,
        MARKER_BOARD_Y,
        MARKER_BOARD_Z,
    )
    .translate(0.0, 0.0, MARKER_BOARD_Z / 2.0);

    let mut markers = Part::empty("probe_shadow_strain_station_position_marker_tokens");
    for position in 0..POSITION_MARKER_COUNT {
        let col = position % 2;
        let row = position / 2;
        let x = centered_index(col, 2, 22.0);
        let y = MARKER_BOARD_Y / 2.0 - 34.0 - row as f64 * 36.0;
        let zone = cassette_zone(position);
        let marker_r = if zone == CassetteZone::Edge { 5.5 } else { 8.0 };
        markers = markers
            + centered_cylinder(
                format!(
                    "probe_shadow_strain_station_position_{position:02}_{}_marker_token",
                    zone.label()
                ),
                marker_r,
                5.0,
                32,
            )
            .translate(x, y, MARKER_BOARD_Z + 2.5);
    }

    let mut fiducials = Part::empty("probe_shadow_strain_station_camera_illumination_fiducials");
    for index in 0..CAMERA_FIDUCIAL_COUNT {
        let y = if index < 2 {
            MARKER_BOARD_Y / 2.0 - 18.0
        } else {
            -MARKER_BOARD_Y / 2.0 + 18.0
        };
        let x = if index % 2 == 0 {
            -MARKER_BOARD_X / 2.0 + 16.0
        } else {
            MARKER_BOARD_X / 2.0 - 16.0
        };
        fiducials = fiducials
            + fiducial_disc(&format!(
                "probe_shadow_strain_station_camera_fiducial_{index}"
            ))
            .translate(x, y, MARKER_BOARD_Z + 2.0);
    }

    for index in 0..ILLUMINATION_BALANCE_LAND_COUNT {
        let y = centered_index(index, ILLUMINATION_BALANCE_LAND_COUNT, 74.0);
        fiducials = fiducials
            + centered_cube(
                format!("probe_shadow_strain_station_illumination_balance_land_{index}"),
                44.0,
                18.0,
                4.0,
            )
            .translate(0.0, y, MARKER_BOARD_Z + 2.0);
    }

    board + markers + fiducials
}

fn traceability_disposition_lanes() -> Part {
    let panel = centered_cube(
        "probe_shadow_strain_station_traceability_disposition_panel",
        TRACE_PANEL_X,
        TRACE_PANEL_Y,
        TRACE_PANEL_Z,
    )
    .translate(0.0, 0.0, TRACE_PANEL_Z / 2.0);

    let mut features = Part::empty("probe_shadow_strain_station_traceability_features");
    for index in 0..BARCODE_LAND_COUNT {
        let x = -TRACE_PANEL_X / 2.0 + 42.0 + index as f64 * 54.0;
        features = features
            + centered_cube(
                format!("probe_shadow_strain_station_barcode_land_{index}"),
                42.0,
                16.0,
                4.0,
            )
            .translate(x, TRACE_PANEL_Y / 2.0 - 23.0, TRACE_PANEL_Z + 2.0);
    }

    for index in 0..CERTIFICATE_LAND_COUNT {
        let x = -TRACE_PANEL_X / 2.0 + 64.0 + index as f64 * 74.0;
        features = features
            + centered_cube(
                format!("probe_shadow_strain_station_certificate_land_{index}"),
                62.0,
                20.0,
                4.0,
            )
            .translate(x, -TRACE_PANEL_Y / 2.0 + 24.0, TRACE_PANEL_Z + 2.0);
    }

    let lane_start_x = 86.0;
    for (lane_index, name) in DISPOSITION_LANE_NAMES.into_iter().enumerate() {
        let x = lane_start_x + lane_index as f64 * 154.0;
        let lane = rectangular_frame(
            &format!("probe_shadow_strain_station_{name}_disposition_lane"),
            132.0,
            58.0,
            18.0,
            7.0,
        )
        .translate(x, 0.0, TRACE_PANEL_Z + 9.0);
        features = features + lane;

        for slot in 0..DISPOSITION_SLOT_COUNT_PER_LANE {
            let sx = x - 45.0 + slot as f64 * 30.0;
            features = features
                + centered_cube(
                    format!("probe_shadow_strain_station_{name}_token_slot_{slot}"),
                    22.0,
                    18.0,
                    5.0,
                )
                .translate(sx, 0.0, TRACE_PANEL_Z + 2.5);
        }
    }

    panel + features
}

fn fiducial_disc(name: &str) -> Part {
    centered_cylinder(format!("{name}_disc"), 7.0, 4.0, 32)
        - centered_cylinder(format!("{name}_center_dot"), 1.6, 5.0, 20)
        - centered_cube(format!("{name}_cross_x"), 12.0, 1.8, 5.0)
        - centered_cube(format!("{name}_cross_y"), 1.8, 12.0, 5.0)
}

fn gripper_fiducials(name: &str, span_x: f64) -> Part {
    let left = fiducial_disc(&format!(
        "probe_shadow_strain_station_{name}_left_grip_fiducial"
    ))
    .translate(-span_x / 2.0, 0.0, 3.0);
    let right = fiducial_disc(&format!(
        "probe_shadow_strain_station_{name}_right_grip_fiducial"
    ))
    .translate(span_x / 2.0, 0.0, 3.0);
    left + right
}

fn rectangular_frame(name: &str, x: f64, y: f64, z: f64, rail: f64) -> Part {
    let front = centered_cube(format!("{name}_front"), x, rail, z).translate(0.0, -y / 2.0, 0.0);
    let rear = centered_cube(format!("{name}_rear"), x, rail, z).translate(0.0, y / 2.0, 0.0);
    let left = centered_cube(format!("{name}_left"), rail, y, z).translate(-x / 2.0, 0.0, 0.0);
    let right = centered_cube(format!("{name}_right"), rail, y, z).translate(x / 2.0, 0.0, 0.0);
    front + rear + left + right
}

fn cassette_position_xy(position: usize) -> (f64, f64) {
    let col = position % CASSETTE_COLS;
    let row = position / CASSETTE_COLS;
    let x = (col as f64 - (CASSETTE_COLS as f64 - 1.0) / 2.0) * CASSETTE_PITCH_X;
    let y = ((CASSETTE_ROWS as f64 - 1.0) / 2.0 - row as f64) * CASSETTE_PITCH_Y;
    (x, y)
}

fn cassette_zone(position: usize) -> CassetteZone {
    let col = position % CASSETTE_COLS;
    let row = position / CASSETTE_COLS;
    if col == 0 || col == CASSETTE_COLS - 1 || row == 0 || row == CASSETTE_ROWS - 1 {
        CassetteZone::Edge
    } else {
        CassetteZone::Center
    }
}

fn edge_position_count() -> usize {
    (0..CASSETTE_POSITION_COUNT)
        .filter(|position| cassette_zone(*position) == CassetteZone::Edge)
        .count()
}

fn center_position_count() -> usize {
    CASSETTE_POSITION_COUNT - edge_position_count()
}

fn probe_mast_xy(kind: ProbeKind, zone: PlacementZone) -> (f64, f64) {
    (
        centered_index(kind.index(), PROBE_KIND_COUNT, PROBE_KIND_PITCH_X),
        centered_index(zone.index(), PROBE_ZONE_COUNT, PROBE_ZONE_PITCH_Y),
    )
}

fn validated_bend_radius() -> f64 {
    BEND_MANDREL_RADII[BEND_MANDREL_COUNT - 1]
}

fn module_rects() -> [Rect; 10] {
    [
        Rect {
            name: "scaled_cassette_position_shadow_map",
            center: CASSETTE_MAP_POS,
            x: CASSETTE_MAP_X,
            y: CASSETTE_MAP_Y,
        },
        Rect {
            name: "probe_mast_array",
            center: PROBE_PANEL_POS,
            x: PROBE_PANEL_X,
            y: PROBE_PANEL_Y,
        },
        Rect {
            name: "airflow_shadow_blocker_bank",
            center: SHADOW_BANK_POS,
            x: SHADOW_BANK_X,
            y: SHADOW_BANK_Y,
        },
        Rect {
            name: "cable_feedthrough_coupon_panel",
            center: FEEDTHROUGH_PANEL_POS,
            x: FEEDTHROUGH_PANEL_X,
            y: FEEDTHROUGH_PANEL_Y,
        },
        Rect {
            name: "bend_radius_strain_gauge_array",
            center: BEND_GAUGE_POS,
            x: BEND_GAUGE_X,
            y: BEND_GAUGE_Y,
        },
        Rect {
            name: "leak_test_witness_well_bank",
            center: LEAK_WELL_POS,
            x: LEAK_WELL_X,
            y: LEAK_WELL_Y,
        },
        Rect {
            name: "reference_logger_pockets",
            center: LOGGER_PANEL_POS,
            x: LOGGER_PANEL_X,
            y: LOGGER_PANEL_Y,
        },
        Rect {
            name: "thermal_rh_dummy_load_coupons",
            center: DUMMY_LOAD_POS,
            x: DUMMY_LOAD_X,
            y: DUMMY_LOAD_Y,
        },
        Rect {
            name: "edge_center_marker_camera_fiducial_board",
            center: MARKER_BOARD_POS,
            x: MARKER_BOARD_X,
            y: MARKER_BOARD_Y,
        },
        Rect {
            name: "traceability_disposition_lanes",
            center: TRACE_PANEL_POS,
            x: TRACE_PANEL_X,
            y: TRACE_PANEL_Y,
        },
    ]
}

fn module_footprints_fit_station() -> bool {
    module_rects()
        .iter()
        .copied()
        .all(Rect::fits_inside_station)
}

fn critical_modules_overlap() -> bool {
    let rects = module_rects();
    for i in 0..rects.len() {
        for j in i + 1..rects.len() {
            if rects[i].overlaps(rects[j]) {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        for output in OUTPUTS {
            assert!(output.starts_with(OUTPUT_PREFIX), "{output}");
            assert!(output.ends_with(".stl"), "{output}");
        }
    }

    #[test]
    fn cassette_map_matches_scaled_clean_box_positions() {
        assert_eq!(CASSETTE_POSITION_COUNT, 16);
        assert_eq!(edge_position_count(), 12);
        assert_eq!(center_position_count(), 4);
        assert_eq!(cassette_zone(0), CassetteZone::Edge);
        assert_eq!(cassette_zone(5), CassetteZone::Center);
        assert_eq!(cassette_zone(10), CassetteZone::Center);
        assert!(SCALED_CHIP_X < REVC_CHIP_LENGTH);
        assert!(SCALED_CHIP_Y < REVC_CHIP_WIDTH);
        assert!(CASSETTE_ARRAY_X < CASSETTE_MAP_X);
        assert!(CASSETTE_ARRAY_Y < CASSETTE_MAP_Y);
    }

    #[test]
    fn probe_and_cable_counts_cover_required_sensors() {
        assert_eq!(ProbeKind::all().len(), PROBE_KIND_COUNT);
        assert_eq!(PlacementZone::all().len(), PROBE_ZONE_COUNT);
        assert_eq!(PROBE_MAST_COUNT, 12);
        assert!(ProbeKind::all().contains(&ProbeKind::Temperature));
        assert!(ProbeKind::all().contains(&ProbeKind::Rh));
        assert!(ProbeKind::all().contains(&ProbeKind::Co2));
        assert!(ProbeKind::all().contains(&ProbeKind::O2));
        assert_eq!(FEEDTHROUGH_COUNT, 8);
        assert_eq!(LEAK_WELL_COUNT, FEEDTHROUGH_COUNT);
    }

    #[test]
    fn module_footprints_fit_without_overlap() {
        assert_design_constraints();
        for rect in module_rects() {
            assert!(
                rect.fits_inside_station(),
                "{} footprint exceeds station envelope",
                rect.name
            );
        }
        assert!(!critical_modules_overlap());
    }

    #[test]
    fn strain_and_leak_validation_features_are_counted() {
        assert_eq!(BEND_LANE_COUNT, 4);
        assert_eq!(BEND_MANDREL_COUNT, 3);
        assert_eq!(validated_bend_radius(), MIN_VALIDATED_BEND_RADIUS);
        assert_eq!(STRAIN_GAUGE_PAD_COUNT, 8);
        assert_eq!(LEAK_WELL_COUNT, FEEDTHROUGH_COUNT);
        assert!(FEEDTHROUGH_SEAL_COLLAR_D > FEEDTHROUGH_BORE_D);
    }

    #[test]
    fn evidence_and_disposition_features_are_explicit() {
        assert_eq!(LOGGER_POCKET_COUNT, PROBE_KIND_COUNT);
        assert_eq!(DUMMY_COUPON_COUNT, CASSETTE_POSITION_COUNT);
        assert_eq!(POSITION_MARKER_COUNT, CASSETTE_POSITION_COUNT);
        assert_eq!(CAMERA_FIDUCIAL_COUNT, 4);
        assert_eq!(ILLUMINATION_BALANCE_LAND_COUNT, 2);
        assert_eq!(BARCODE_LAND_COUNT, 6);
        assert_eq!(CERTIFICATE_LAND_COUNT, 3);
        assert_eq!(DISPOSITION_LANE_NAMES, ["release", "hold", "reject"]);
        assert_eq!(DISPOSITION_TOTAL_CAPACITY, 12);
    }

    #[test]
    fn feature_registry_and_limitations_are_stable() {
        for feature in [
            "containment_deck",
            "scaled_cassette_position_shadow_map",
            "probe_mast_array",
            "airflow_shadow_blocker_bank",
            "cable_feedthrough_coupon_panel",
            "bend_radius_strain_gauge_array",
            "leak_test_witness_well_bank",
            "reference_logger_pockets",
            "thermal_rh_dummy_load_coupons",
            "edge_center_marker_camera_fiducial_board",
            "traceability_disposition_lanes",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }

        for limitation in [
            "mechanical_fixture_only",
            "no_acceptance_limits",
            "no_calibration_protocol",
            "no_leak_test_pressure_claim",
            "no_sterility_claim",
            "no_biological_performance_claim",
        ] {
            assert!(LIMITATIONS.contains(&limitation));
        }
    }
}
