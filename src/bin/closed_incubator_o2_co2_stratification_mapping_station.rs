use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed incubator O2/CO2 stratification mapping station.
//
// This is validation fixture CAD for mapping gas stratification across rack
// edge/center positions before live tissue-chip runs. It provides no-cell rack
// surrogates, vertical sample mast positions, O2/CO2 probe docks, reference-gas
// puck lands, challenge inlet placeholders, traceability lands, disposition
// lanes, clean/used segregation, evidence camera support, cable/tubing strain
// relief, and robot/service keepout gauges. Gas standards, analyzer accuracy,
// acceptance criteria, incubation recipes, and study statistics remain protocol
// controls outside this model.

const OUTPUTS: [&str; 14] = [
    "output/closed_incubator_o2_co2_stratification_mapping_station_base_containment_deck.stl",
    "output/closed_incubator_o2_co2_stratification_mapping_station_rack_slot_surrogate_fixture.stl",
    "output/closed_incubator_o2_co2_stratification_mapping_station_vertical_sampling_mast_positions.stl",
    "output/closed_incubator_o2_co2_stratification_mapping_station_o2_co2_probe_docks.stl",
    "output/closed_incubator_o2_co2_stratification_mapping_station_reference_gas_calibration_puck_lands.stl",
    "output/closed_incubator_o2_co2_stratification_mapping_station_edge_center_position_tokens.stl",
    "output/closed_incubator_o2_co2_stratification_mapping_station_gas_mixing_challenge_inlet_placeholders.stl",
    "output/closed_incubator_o2_co2_stratification_mapping_station_barcode_certificate_lands.stl",
    "output/closed_incubator_o2_co2_stratification_mapping_station_release_hold_reject_lanes.stl",
    "output/closed_incubator_o2_co2_stratification_mapping_station_clean_used_segregation.stl",
    "output/closed_incubator_o2_co2_stratification_mapping_station_evidence_camera_bridge.stl",
    "output/closed_incubator_o2_co2_stratification_mapping_station_cable_tubing_strain_relief.stl",
    "output/closed_incubator_o2_co2_stratification_mapping_station_robot_service_keepout_gauges.stl",
    "output/closed_incubator_o2_co2_stratification_mapping_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 13] = [
    "base_containment_deck",
    "rack_slot_surrogate_fixture",
    "vertical_sampling_mast_positions",
    "o2_co2_probe_docks",
    "reference_gas_calibration_puck_lands",
    "edge_center_position_tokens",
    "gas_mixing_challenge_inlet_placeholders",
    "barcode_certificate_lands",
    "release_hold_reject_lanes",
    "clean_used_segregation",
    "evidence_camera_bridge",
    "cable_tubing_strain_relief",
    "robot_service_keepout_gauges",
];

const DECK_X: f64 = 1620.0;
const DECK_Y: f64 = 1000.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const SOCKET_DEPTH: f64 = 6.0;
const MOUNT_HOLE_COUNT: usize = 8;
const DATUM_TARGET_COUNT: usize = 4;

const RACK_COLS: usize = 4;
const RACK_ROWS: usize = 3;
const SLOT_COUNT: usize = RACK_COLS * RACK_ROWS;
const EDGE_POSITION_COUNT: usize = 10;
const CENTER_POSITION_COUNT: usize = SLOT_COUNT - EDGE_POSITION_COUNT;
const SLOT_PITCH_X: f64 = 118.0;
const SLOT_PITCH_Y: f64 = 130.0;
const SLOT_SURROGATE_X: f64 = REVC_CHIP_LENGTH + 44.0;
const SLOT_SURROGATE_Y: f64 = REVC_CHIP_WIDTH + 38.0;
const SLOT_RELIEF_DEPTH: f64 = 9.0;
const RACK_FIXTURE_X: f64 = 560.0;
const RACK_FIXTURE_Y: f64 = 430.0;
const RACK_FIXTURE_Z: f64 = 36.0;
const RACK_POS: (f64, f64) = (-330.0, 120.0);

const MAST_HEIGHT: f64 = 178.0;
const MAST_BASE_D: f64 = 20.0;
const MAST_SHAFT_D: f64 = 8.0;
const MAST_TIER_COUNT: usize = 3;
const LOW_SAMPLE_Z: f64 = 52.0;
const MID_SAMPLE_Z: f64 = 104.0;
const HIGH_SAMPLE_Z: f64 = 156.0;

const PROBE_DOCK_COUNT: usize = 6;
const O2_PROBE_DOCK_COUNT: usize = 3;
const CO2_PROBE_DOCK_COUNT: usize = 3;
const PROBE_BANK_X: f64 = 340.0;
const PROBE_BANK_Y: f64 = 160.0;
const PROBE_BANK_Z: f64 = 46.0;
const PROBE_POS: (f64, f64) = (360.0, 240.0);
const PROBE_DOCK_PITCH_X: f64 = 52.0;
const O2_PROBE_SLEEVE_D: f64 = 12.5;
const CO2_PROBE_SLEEVE_D: f64 = 15.0;

const PUCK_LAND_COUNT: usize = 6;
const PUCK_BANK_X: f64 = 340.0;
const PUCK_BANK_Y: f64 = 150.0;
const PUCK_BANK_Z: f64 = 34.0;
const PUCK_POS: (f64, f64) = (360.0, 40.0);
const PUCK_D: f64 = 38.0;
const PUCK_PITCH_X: f64 = 78.0;
const PUCK_PITCH_Y: f64 = 58.0;

const TOKEN_PLATE_X: f64 = 170.0;
const TOKEN_PLATE_Y: f64 = 260.0;
const TOKEN_PLATE_Z: f64 = 12.0;
const TOKEN_POS: (f64, f64) = (65.0, 110.0);
const EDGE_TOKEN_D: f64 = 20.0;
const CENTER_TOKEN_D: f64 = 15.0;

const CHALLENGE_PORT_COUNT: usize = 6;
const CHALLENGE_PANEL_X: f64 = 520.0;
const CHALLENGE_PANEL_Y: f64 = 130.0;
const CHALLENGE_PANEL_Z: f64 = 52.0;
const CHALLENGE_POS: (f64, f64) = (-305.0, -180.0);
const CHALLENGE_PORT_PITCH_X: f64 = 74.0;
const CHALLENGE_PORT_D: f64 = 13.0;
const MIXING_STATIC_CHANNEL_W: f64 = 10.0;

const TRACE_PLATE_X: f64 = 330.0;
const TRACE_PLATE_Y: f64 = 120.0;
const TRACE_PLATE_Z: f64 = 12.0;
const TRACE_POS: (f64, f64) = (370.0, -140.0);
const BARCODE_LAND_COUNT: usize = SLOT_COUNT + 2;
const CERTIFICATE_LAND_COUNT: usize = 4;

const LANE_COUNT: usize = 3;
const LANE_BANK_X: f64 = 410.0;
const LANE_BANK_Y: f64 = 150.0;
const LANE_BANK_Z: f64 = 32.0;
const LANE_POS: (f64, f64) = (370.0, -330.0);
const LANE_SLOT_X: f64 = 88.0;
const LANE_SLOT_Y: f64 = 28.0;
const RELEASE_LANE_CAPACITY: usize = 6;
const HOLD_LANE_CAPACITY: usize = 4;
const REJECT_LANE_CAPACITY: usize = 2;

const SEG_TRAY_X: f64 = 110.0;
const SEG_TRAY_Y: f64 = 250.0;
const SEG_TRAY_Z: f64 = 42.0;
const SEG_POS: (f64, f64) = (-700.0, -310.0);
const CLEAN_WELL_COUNT: usize = 6;
const USED_WELL_COUNT: usize = 6;
const SEG_DIVIDER_Z: f64 = 86.0;
const CLEAN_USED_GAP: f64 = 18.0;

const CAMERA_BRIDGE_X: f64 = 1100.0;
const CAMERA_BRIDGE_Y: f64 = 72.0;
const CAMERA_BRIDGE_Z: f64 = 220.0;
const CAMERA_POS: (f64, f64) = (-10.0, 345.0);
const CAMERA_TARGET_COUNT: usize = 5;
const CAMERA_CLEARANCE_Z: f64 = 182.0;

const STRAIN_RELIEF_X: f64 = 560.0;
const STRAIN_RELIEF_Y: f64 = 100.0;
const STRAIN_RELIEF_Z: f64 = 36.0;
const STRAIN_POS: (f64, f64) = (-310.0, -345.0);
const SAMPLE_TUBE_COUNT: usize = SLOT_COUNT;
const SERVICE_LINE_COUNT: usize = 6;
const SAMPLE_TUBE_OD: f64 = 3.2;
const SERVICE_TUBE_OD: f64 = 6.35;

const KEEP_OUT_X: f64 = 1500.0;
const KEEP_OUT_Y: f64 = 900.0;
const KEEP_OUT_Z: f64 = 160.0;
const ROBOT_FRONT_CLEARANCE: f64 = 405.0;
const SERVICE_REAR_CLEARANCE: f64 = 240.0;
const ROBOT_Z_CLEARANCE: f64 = 330.0;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    fn all() -> [DispositionLane; LANE_COUNT] {
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

    fn capacity(self) -> usize {
        match self {
            DispositionLane::Release => RELEASE_LANE_CAPACITY,
            DispositionLane::Hold => HOLD_LANE_CAPACITY,
            DispositionLane::Reject => REJECT_LANE_CAPACITY,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_deck(self) -> bool {
        let usable_x = DECK_X / 2.0 - RIM_W - 16.0;
        let usable_y = DECK_Y / 2.0 - RIM_W - 16.0;

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

    let deck = base_containment_deck();
    export(OUTPUTS[0], &deck);

    let rack = rack_slot_surrogate_fixture();
    export(OUTPUTS[1], &rack);

    let masts = vertical_sampling_mast_positions();
    export(OUTPUTS[2], &masts);

    let probes = o2_co2_probe_docks();
    export(OUTPUTS[3], &probes);

    let pucks = reference_gas_calibration_puck_lands();
    export(OUTPUTS[4], &pucks);

    let tokens = edge_center_position_tokens();
    export(OUTPUTS[5], &tokens);

    let challenge = gas_mixing_challenge_inlet_placeholders();
    export(OUTPUTS[6], &challenge);

    let trace = barcode_certificate_lands();
    export(OUTPUTS[7], &trace);

    let lanes = release_hold_reject_lanes();
    export(OUTPUTS[8], &lanes);

    let segregation = clean_used_segregation();
    export(OUTPUTS[9], &segregation);

    let camera = evidence_camera_bridge();
    export(OUTPUTS[10], &camera);

    let strain = cable_tubing_strain_relief();
    export(OUTPUTS[11], &strain);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[12], &keepouts);

    let assembly = deck
        + rack.translate(RACK_POS.0, RACK_POS.1, seated_z())
        + masts.translate(RACK_POS.0, RACK_POS.1, DECK_Z)
        + probes.translate(PROBE_POS.0, PROBE_POS.1, seated_z())
        + pucks.translate(PUCK_POS.0, PUCK_POS.1, seated_z())
        + tokens.translate(TOKEN_POS.0, TOKEN_POS.1, seated_z())
        + challenge.translate(CHALLENGE_POS.0, CHALLENGE_POS.1, seated_z())
        + trace.translate(TRACE_POS.0, TRACE_POS.1, seated_z())
        + lanes.translate(LANE_POS.0, LANE_POS.1, seated_z())
        + segregation.translate(SEG_POS.0, SEG_POS.1, seated_z())
        + camera.translate(CAMERA_POS.0, CAMERA_POS.1, DECK_Z)
        + strain.translate(STRAIN_POS.0, STRAIN_POS.1, seated_z())
        + keepouts.translate(0.0, 0.0, DECK_Z);
    export(OUTPUTS[13], &assembly);

    println!();
    println!("Closed incubator O2/CO2 stratification mapping station:");
    println!(
        "  Mapping fixture:       {RACK_COLS}x{RACK_ROWS} rack surrogate with {EDGE_POSITION_COUNT} edge and {CENTER_POSITION_COUNT} center positions"
    );
    println!(
        "  Sampling geometry:     {SLOT_COUNT} mast positions with {MAST_TIER_COUNT} low/mid/high elevation markers"
    );
    println!(
        "  Probe/calibration:     {O2_PROBE_DOCK_COUNT} O2 docks, {CO2_PROBE_DOCK_COUNT} CO2 docks, and {PUCK_LAND_COUNT} reference gas puck lands"
    );
    println!(
        "  Challenge interface:   {CHALLENGE_PORT_COUNT} gas mixing inlet placeholders, {SAMPLE_TUBE_COUNT} sample strain-relief comb slots, and {SERVICE_LINE_COUNT} service line saddles"
    );
    println!(
        "  Evidence/control:      {BARCODE_LAND_COUNT} barcode lands, {CERTIFICATE_LAND_COUNT} certificate lands, camera bridge with {CAMERA_TARGET_COUNT} view targets, and release/hold/reject capacity for {} positions",
        total_lane_capacity()
    );
    println!(
        "  Keepouts:              {ROBOT_FRONT_CLEARANCE:.0}mm robot approach, {SERVICE_REAR_CLEARANCE:.0}mm rear service, {ROBOT_Z_CLEARANCE:.0}mm Z clearance"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn seated_z() -> f64 {
    DECK_Z - SOCKET_DEPTH / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 14);
    assert_eq!(REQUIRED_FEATURES.len(), 13);
    assert_eq!(SLOT_COUNT, 12);
    assert_eq!(edge_position_count(), EDGE_POSITION_COUNT);
    assert_eq!(center_position_count(), CENTER_POSITION_COUNT);
    assert_eq!(O2_PROBE_DOCK_COUNT + CO2_PROBE_DOCK_COUNT, PROBE_DOCK_COUNT);
    assert_eq!(DispositionLane::all().len(), LANE_COUNT);
    assert_eq!(total_lane_capacity(), SLOT_COUNT);
    assert_eq!(CLEAN_WELL_COUNT, USED_WELL_COUNT);
    assert_eq!(PUCK_LAND_COUNT, reference_gas_names().len());
    assert_eq!(CHALLENGE_PORT_COUNT, challenge_port_names().len());
    assert!(SLOT_SURROGATE_X > REVC_CHIP_LENGTH);
    assert!(SLOT_SURROGATE_Y > REVC_CHIP_WIDTH);
    assert!(SLOT_RELIEF_DEPTH < RACK_FIXTURE_Z - REVC_TOTAL_HEIGHT);
    assert!(MAST_HEIGHT > HIGH_SAMPLE_Z + 12.0);
    assert!(CAMERA_CLEARANCE_Z > MAST_HEIGHT);
    assert!(ROBOT_Z_CLEARANCE > DECK_Z + CAMERA_BRIDGE_Z + 70.0);
    assert!(O2_PROBE_SLEEVE_D > SAMPLE_TUBE_OD);
    assert!(CO2_PROBE_SLEEVE_D > SAMPLE_TUBE_OD);
    assert!(SERVICE_TUBE_OD + 1.0 < CHALLENGE_PORT_D);
    assert!(SEG_DIVIDER_Z > SEG_TRAY_Z);
    assert!(clean_used_gap() >= CLEAN_USED_GAP);

    for item in socket_rects() {
        assert!(item.fits_inside_deck(), "{} exceeds deck rim", item.name);
    }

    for (a, b) in non_overlap_pairs() {
        assert!(!a.overlaps(b), "{} overlaps {}", a.name, b.name);
    }
}

fn socket_rects() -> [Rect; 9] {
    [
        rect(
            "rack_slot_surrogate_fixture",
            RACK_POS,
            RACK_FIXTURE_X,
            RACK_FIXTURE_Y,
        ),
        rect("o2_co2_probe_docks", PROBE_POS, PROBE_BANK_X, PROBE_BANK_Y),
        rect(
            "reference_gas_calibration_puck_lands",
            PUCK_POS,
            PUCK_BANK_X,
            PUCK_BANK_Y,
        ),
        rect(
            "edge_center_position_tokens",
            TOKEN_POS,
            TOKEN_PLATE_X,
            TOKEN_PLATE_Y,
        ),
        rect(
            "gas_mixing_challenge_inlet_placeholders",
            CHALLENGE_POS,
            CHALLENGE_PANEL_X,
            CHALLENGE_PANEL_Y,
        ),
        rect(
            "barcode_certificate_lands",
            TRACE_POS,
            TRACE_PLATE_X,
            TRACE_PLATE_Y,
        ),
        rect(
            "release_hold_reject_lanes",
            LANE_POS,
            LANE_BANK_X,
            LANE_BANK_Y,
        ),
        rect("clean_used_segregation", SEG_POS, SEG_TRAY_X, SEG_TRAY_Y),
        rect(
            "cable_tubing_strain_relief",
            STRAIN_POS,
            STRAIN_RELIEF_X,
            STRAIN_RELIEF_Y,
        ),
    ]
}

fn non_overlap_pairs() -> [(Rect, Rect); 11] {
    let rects = socket_rects();
    [
        (rects[0], rects[1]),
        (rects[0], rects[2]),
        (rects[0], rects[3]),
        (rects[0], rects[4]),
        (rects[1], rects[2]),
        (rects[2], rects[5]),
        (rects[3], rects[5]),
        (rects[4], rects[8]),
        (rects[5], rects[6]),
        (rects[6], rects[8]),
        (rects[7], rects[8]),
    ]
}

fn base_containment_deck() -> Part {
    let deck = centered_cube(
        "incubator_stratification_station_base_containment_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let wipe_basin = centered_cube(
        "incubator_stratification_station_wipeable_secondary_basin_cut",
        DECK_X - 2.0 * (RIM_W + 54.0),
        DECK_Y - 2.0 * (RIM_W + 54.0),
        8.0,
    )
    .translate(0.0, -8.0, DECK_Z - 4.0);
    let front_condensate_channel = centered_cube(
        "incubator_stratification_station_front_condensate_channel",
        DECK_X - 180.0,
        18.0,
        8.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 58.0, DECK_Z - 4.0);
    let drain_port = centered_cylinder(
        "incubator_stratification_station_closed_deck_drain_placeholder",
        8.0,
        48.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 92.0, -DECK_Y / 2.0 + 32.0, DECK_Z - 8.0);

    deck - wipe_basin - front_condensate_channel - drain_port - insert_sockets() - mounting_holes()
        + perimeter_rims()
        + zone_spines()
        + robot_datum_targets()
        + camera_anchor_lands()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("incubator_stratification_station_insert_sockets");
    for item in socket_rects() {
        sockets = sockets
            + centered_cube(
                format!("incubator_stratification_station_{}_socket", item.name),
                item.x + 10.0,
                item.y + 10.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(item.center.0, item.center.1, DECK_Z - SOCKET_DEPTH / 2.0);
    }
    sockets
}

fn mounting_holes() -> Part {
    let mut holes = Part::empty("incubator_stratification_station_deck_mount_holes");
    for (i, (x, y)) in mount_hole_positions().iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("incubator_stratification_station_m6_mount_clearance_{i}"),
                3.4,
                DECK_Z + 4.0,
                28,
            )
            .translate(*x, *y, DECK_Z / 2.0);
    }
    holes
}

fn mount_hole_positions() -> [(f64, f64); MOUNT_HOLE_COUNT] {
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
        "incubator_stratification_station_front_low_containment_rim",
        DECK_X,
        RIM_W,
        28.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + RIM_W / 2.0, DECK_Z + 14.0);
    let rear = centered_cube(
        "incubator_stratification_station_rear_service_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "incubator_stratification_station_left_clean_used_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "incubator_stratification_station_right_analyzer_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);

    front + rear + left + right
}

fn zone_spines() -> Part {
    let rack_gas_divider = centered_cube(
        "incubator_stratification_station_rack_gas_workflow_spine",
        DECK_X - 220.0,
        10.0,
        26.0,
    )
    .translate(-60.0, -112.0, DECK_Z + 13.0);
    let disposition_divider = centered_cube(
        "incubator_stratification_station_traceability_disposition_spine",
        10.0,
        330.0,
        24.0,
    )
    .translate(132.0, -250.0, DECK_Z + 12.0);
    let clean_used_divider = centered_cube(
        "incubator_stratification_station_clean_used_zone_spine",
        12.0,
        290.0,
        32.0,
    )
    .translate(-620.0, -310.0, DECK_Z + 16.0);

    rack_gas_divider + disposition_divider + clean_used_divider
}

fn robot_datum_targets() -> Part {
    let mut targets = Part::empty("incubator_stratification_station_robot_datum_targets");
    for (i, (x, y)) in datum_target_positions().iter().enumerate() {
        targets = targets
            + fiducial_disc(&format!(
                "incubator_stratification_station_robot_datum_target_{i}"
            ))
            .translate(*x, *y, DECK_Z + 2.0);
    }
    targets
}

fn datum_target_positions() -> [(f64, f64); DATUM_TARGET_COUNT] {
    [
        (-DECK_X / 2.0 + 94.0, -DECK_Y / 2.0 + 94.0),
        (DECK_X / 2.0 - 94.0, -DECK_Y / 2.0 + 94.0),
        (-DECK_X / 2.0 + 94.0, DECK_Y / 2.0 - 94.0),
        (DECK_X / 2.0 - 94.0, DECK_Y / 2.0 - 94.0),
    ]
}

fn camera_anchor_lands() -> Part {
    let left = centered_cube(
        "incubator_stratification_station_camera_bridge_left_anchor_land",
        96.0,
        42.0,
        8.0,
    )
    .translate(
        CAMERA_POS.0 - CAMERA_BRIDGE_X / 2.0 + 56.0,
        CAMERA_POS.1,
        DECK_Z + 4.0,
    );
    let right = centered_cube(
        "incubator_stratification_station_camera_bridge_right_anchor_land",
        96.0,
        42.0,
        8.0,
    )
    .translate(
        CAMERA_POS.0 + CAMERA_BRIDGE_X / 2.0 - 56.0,
        CAMERA_POS.1,
        DECK_Z + 4.0,
    );
    left + right
}

fn rack_slot_surrogate_fixture() -> Part {
    let body = centered_cube(
        "incubator_stratification_rack_slot_surrogate_fixture_body",
        RACK_FIXTURE_X,
        RACK_FIXTURE_Y,
        RACK_FIXTURE_Z,
    )
    .translate(0.0, 0.0, RACK_FIXTURE_Z / 2.0);
    let perimeter_gasket = gasket_frame_xy(
        "incubator_stratification_rack_fixture_wipeable_gasket_land",
        RACK_FIXTURE_X - 26.0,
        RACK_FIXTURE_Y - 26.0,
        8.0,
        6.0,
    )
    .translate(0.0, 0.0, RACK_FIXTURE_Z + 3.0);

    body - rack_slot_reliefs() + rack_slot_lips() + rack_datum_features() + perimeter_gasket
}

fn rack_slot_reliefs() -> Part {
    let mut reliefs = Part::empty("incubator_stratification_rack_slot_reliefs");
    for row in 0..RACK_ROWS {
        for col in 0..RACK_COLS {
            let index = position_index(col, row);
            let (x, y) = rack_slot_center(col, row);
            reliefs = reliefs
                + centered_cube(
                    format!("incubator_stratification_slot_{index}_module_relief"),
                    SLOT_SURROGATE_X,
                    SLOT_SURROGATE_Y,
                    SLOT_RELIEF_DEPTH + 0.6,
                )
                .translate(x, y, RACK_FIXTURE_Z - SLOT_RELIEF_DEPTH / 2.0);
        }
    }
    reliefs
}

fn rack_slot_lips() -> Part {
    let mut lips = Part::empty("incubator_stratification_rack_slot_lips");
    for row in 0..RACK_ROWS {
        for col in 0..RACK_COLS {
            let index = position_index(col, row);
            let (x, y) = rack_slot_center(col, row);
            let zone = map_zone(col, row);
            let zone_label = zone.label();
            let front = centered_cube(
                format!("incubator_stratification_slot_{index}_{zone_label}_front_lip"),
                SLOT_SURROGATE_X + 12.0,
                6.0,
                13.0,
            )
            .translate(x, y - SLOT_SURROGATE_Y / 2.0 - 6.0, RACK_FIXTURE_Z + 6.5);
            let rear = centered_cube(
                format!("incubator_stratification_slot_{index}_{zone_label}_rear_lip"),
                SLOT_SURROGATE_X + 12.0,
                6.0,
                13.0,
            )
            .translate(x, y + SLOT_SURROGATE_Y / 2.0 + 6.0, RACK_FIXTURE_Z + 6.5);
            let side_key = centered_cube(
                format!("incubator_stratification_slot_{index}_{zone_label}_datum_key"),
                8.0,
                SLOT_SURROGATE_Y * 0.66,
                16.0,
            )
            .translate(x - SLOT_SURROGATE_X / 2.0 - 8.0, y, RACK_FIXTURE_Z + 8.0);
            lips = lips + front + rear + side_key;
        }
    }
    lips
}

fn rack_datum_features() -> Part {
    let mut datums = Part::empty("incubator_stratification_rack_datum_features");
    for (i, (x, y)) in [
        (-RACK_FIXTURE_X / 2.0 + 32.0, -RACK_FIXTURE_Y / 2.0 + 32.0),
        (RACK_FIXTURE_X / 2.0 - 32.0, -RACK_FIXTURE_Y / 2.0 + 32.0),
        (-RACK_FIXTURE_X / 2.0 + 32.0, RACK_FIXTURE_Y / 2.0 - 32.0),
        (RACK_FIXTURE_X / 2.0 - 32.0, RACK_FIXTURE_Y / 2.0 - 32.0),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("incubator_stratification_rack_datum_boss_{i}"),
            9.0,
            8.0,
            32,
        )
        .translate(*x, *y, RACK_FIXTURE_Z + 4.0);
        let pilot = centered_cylinder(
            format!("incubator_stratification_rack_datum_pin_pilot_{i}"),
            2.6,
            10.0,
            24,
        )
        .translate(*x, *y, RACK_FIXTURE_Z + 5.0);
        datums = datums + (boss - pilot);
    }
    datums
}

fn vertical_sampling_mast_positions() -> Part {
    let mut masts = Part::empty("incubator_stratification_vertical_sampling_mast_positions");
    for row in 0..RACK_ROWS {
        for col in 0..RACK_COLS {
            let index = position_index(col, row);
            let (x, y) = rack_slot_center(col, row);
            let zone = map_zone(col, row);
            let base = centered_cylinder(
                format!(
                    "incubator_stratification_slot_{index}_{}_mast_base_socket",
                    zone.label()
                ),
                MAST_BASE_D / 2.0,
                8.0,
                32,
            )
            .translate(x, y, 4.0);
            let shaft = centered_cylinder(
                format!("incubator_stratification_slot_{index}_sample_mast_axis"),
                MAST_SHAFT_D / 2.0,
                MAST_HEIGHT,
                28,
            )
            .translate(x, y, MAST_HEIGHT / 2.0);
            masts = masts + base + shaft + mast_elevation_markers(index, x, y);
        }
    }
    masts
}

fn mast_elevation_markers(index: usize, x: f64, y: f64) -> Part {
    let mut markers = Part::empty("incubator_stratification_mast_elevation_markers");
    for (tier, z) in [LOW_SAMPLE_Z, MID_SAMPLE_Z, HIGH_SAMPLE_Z]
        .iter()
        .enumerate()
    {
        markers = markers
            + centered_cylinder(
                format!("incubator_stratification_slot_{index}_sample_tier_{tier}_collar"),
                7.8,
                3.0,
                28,
            )
            .translate(x, y, *z);
    }
    markers
}

fn o2_co2_probe_docks() -> Part {
    let body = centered_cube(
        "incubator_stratification_o2_co2_probe_dock_bank_body",
        PROBE_BANK_X,
        PROBE_BANK_Y,
        PROBE_BANK_Z,
    )
    .translate(0.0, 0.0, PROBE_BANK_Z / 2.0);
    let rear_fence = centered_cube(
        "incubator_stratification_probe_dock_rear_cable_fence",
        PROBE_BANK_X,
        12.0,
        50.0,
    )
    .translate(0.0, PROBE_BANK_Y / 2.0 - 6.0, PROBE_BANK_Z + 25.0);
    let front_stop = centered_cube(
        "incubator_stratification_probe_dock_front_positive_stop",
        PROBE_BANK_X - 42.0,
        9.0,
        24.0,
    )
    .translate(0.0, -PROBE_BANK_Y / 2.0 + 16.0, PROBE_BANK_Z + 12.0);

    body + rear_fence
        + front_stop
        + probe_nose_stops()
        + probe_label_lands()
        + probe_cable_saddles()
        - probe_sleeve_cuts()
}

fn probe_sleeve_cuts() -> Part {
    let mut cuts = Part::empty("incubator_stratification_probe_sleeve_cuts");
    for i in 0..PROBE_DOCK_COUNT {
        let x = centered_index(i, PROBE_DOCK_COUNT, PROBE_DOCK_PITCH_X);
        let diameter = if i < O2_PROBE_DOCK_COUNT {
            O2_PROBE_SLEEVE_D
        } else {
            CO2_PROBE_SLEEVE_D
        };
        let sleeve = centered_cylinder(
            format!("incubator_stratification_probe_dock_{i}_sleeve_bore"),
            diameter / 2.0,
            PROBE_BANK_Y + 14.0,
            36,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -4.0, PROBE_BANK_Z / 2.0);
        let top_access = centered_cube(
            format!("incubator_stratification_probe_dock_{i}_top_access_slot"),
            diameter + 8.0,
            PROBE_BANK_Y - 36.0,
            18.0,
        )
        .translate(x, -6.0, PROBE_BANK_Z - 8.0);
        cuts = cuts + sleeve + top_access;
    }
    cuts
}

fn probe_nose_stops() -> Part {
    let mut stops = Part::empty("incubator_stratification_probe_nose_stops");
    for i in 0..PROBE_DOCK_COUNT {
        let x = centered_index(i, PROBE_DOCK_COUNT, PROBE_DOCK_PITCH_X);
        stops = stops
            + centered_cube(
                format!("incubator_stratification_probe_dock_{i}_nose_stop"),
                30.0,
                7.0,
                18.0,
            )
            .translate(x, PROBE_BANK_Y / 2.0 - 28.0, PROBE_BANK_Z + 9.0);
    }
    stops
}

fn probe_label_lands() -> Part {
    let mut labels = Part::empty("incubator_stratification_probe_label_lands");
    for i in 0..PROBE_DOCK_COUNT {
        let x = centered_index(i, PROBE_DOCK_COUNT, PROBE_DOCK_PITCH_X);
        let label = if i < O2_PROBE_DOCK_COUNT { "o2" } else { "co2" };
        labels = labels
            + centered_cube(
                format!("incubator_stratification_{label}_probe_dock_{i}_label_land"),
                38.0,
                16.0,
                3.0,
            )
            .translate(x, -PROBE_BANK_Y / 2.0 + 34.0, PROBE_BANK_Z + 1.5);
    }
    labels
}

fn probe_cable_saddles() -> Part {
    let mut saddles = Part::empty("incubator_stratification_probe_cable_saddles");
    for i in 0..PROBE_DOCK_COUNT {
        let x = centered_index(i, PROBE_DOCK_COUNT, PROBE_DOCK_PITCH_X);
        let saddle = centered_cube(
            format!("incubator_stratification_probe_dock_{i}_cable_saddle"),
            24.0,
            18.0,
            12.0,
        )
        .translate(x, -PROBE_BANK_Y / 2.0 + 12.0, PROBE_BANK_Z + 6.0);
        saddles = saddles + saddle;
    }
    saddles
}

fn reference_gas_calibration_puck_lands() -> Part {
    let body = centered_cube(
        "incubator_stratification_reference_gas_puck_land_body",
        PUCK_BANK_X,
        PUCK_BANK_Y,
        PUCK_BANK_Z,
    )
    .translate(0.0, 0.0, PUCK_BANK_Z / 2.0);
    let gasket = gasket_frame_xy(
        "incubator_stratification_reference_gas_puck_bank_gasket",
        PUCK_BANK_X - 28.0,
        PUCK_BANK_Y - 28.0,
        7.0,
        7.0,
    )
    .translate(0.0, 0.0, PUCK_BANK_Z + 3.5);

    body - calibration_puck_well_cuts()
        + calibration_puck_rims()
        + gas_reference_label_lands()
        + gasket
}

fn calibration_puck_well_cuts() -> Part {
    let mut cuts = Part::empty("incubator_stratification_calibration_puck_well_cuts");
    for i in 0..PUCK_LAND_COUNT {
        let (x, y) = puck_position(i);
        cuts = cuts
            + centered_cylinder(
                format!(
                    "incubator_stratification_{}_puck_well_cut",
                    reference_gas_names()[i]
                ),
                PUCK_D / 2.0,
                18.0,
                42,
            )
            .translate(x, y, PUCK_BANK_Z - 8.5);
    }
    cuts
}

fn calibration_puck_rims() -> Part {
    let mut rims = Part::empty("incubator_stratification_calibration_puck_rims");
    for i in 0..PUCK_LAND_COUNT {
        let (x, y) = puck_position(i);
        rims = rims
            + annular_disc(
                &format!(
                    "incubator_stratification_{}_puck_guard_rim",
                    reference_gas_names()[i]
                ),
                PUCK_D + 11.0,
                PUCK_D + 1.5,
                5.0,
            )
            .translate(x, y, PUCK_BANK_Z + 2.5);
    }
    rims
}

fn gas_reference_label_lands() -> Part {
    let mut labels = Part::empty("incubator_stratification_reference_gas_label_lands");
    for i in 0..PUCK_LAND_COUNT {
        let (x, y) = puck_position(i);
        labels = labels
            + centered_cube(
                format!(
                    "incubator_stratification_{}_reference_label_land",
                    reference_gas_names()[i]
                ),
                52.0,
                12.0,
                3.0,
            )
            .translate(x, y - 36.0, PUCK_BANK_Z + 1.5);
    }
    labels
}

fn reference_gas_names() -> [&'static str; PUCK_LAND_COUNT] {
    [
        "ambient",
        "low_o2",
        "nominal_co2",
        "high_co2",
        "zero",
        "span",
    ]
}

fn puck_position(index: usize) -> (f64, f64) {
    let col = index % 3;
    let row = index / 3;
    (
        centered_index(col, 3, PUCK_PITCH_X),
        centered_index(row, 2, PUCK_PITCH_Y),
    )
}

fn edge_center_position_tokens() -> Part {
    let plate = centered_cube(
        "incubator_stratification_edge_center_token_plate",
        TOKEN_PLATE_X,
        TOKEN_PLATE_Y,
        TOKEN_PLATE_Z,
    )
    .translate(0.0, 0.0, TOKEN_PLATE_Z / 2.0);
    let mut tokens = Part::empty("incubator_stratification_edge_center_position_tokens");
    for row in 0..RACK_ROWS {
        for col in 0..RACK_COLS {
            let index = position_index(col, row);
            let zone = map_zone(col, row);
            let (x, y) = token_position(col, row);
            let token_d = match zone {
                MapZone::Edge => EDGE_TOKEN_D,
                MapZone::Center => CENTER_TOKEN_D,
            };
            let height = match zone {
                MapZone::Edge => 6.0,
                MapZone::Center => 9.0,
            };
            let token = centered_cylinder(
                format!(
                    "incubator_stratification_position_{index}_{}_token",
                    zone.label()
                ),
                token_d / 2.0,
                height,
                32,
            )
            .translate(x, y, TOKEN_PLATE_Z + height / 2.0);
            let scan_dot = fiducial_disc(&format!(
                "incubator_stratification_position_{index}_{}_token_scan",
                zone.label()
            ))
            .translate(x, y - 22.0, TOKEN_PLATE_Z + 2.0);
            tokens = tokens + token + scan_dot;
        }
    }
    plate + tokens
}

fn gas_mixing_challenge_inlet_placeholders() -> Part {
    let body = centered_cube(
        "incubator_stratification_gas_mixing_challenge_panel_body",
        CHALLENGE_PANEL_X,
        CHALLENGE_PANEL_Y,
        CHALLENGE_PANEL_Z,
    )
    .translate(0.0, 0.0, CHALLENGE_PANEL_Z / 2.0);
    let rear_fence = centered_cube(
        "incubator_stratification_gas_challenge_rear_tubing_fence",
        CHALLENGE_PANEL_X,
        12.0,
        42.0,
    )
    .translate(0.0, CHALLENGE_PANEL_Y / 2.0 - 6.0, CHALLENGE_PANEL_Z + 21.0);
    let static_mixer_shadow = centered_cube(
        "incubator_stratification_static_mixer_placeholder_shadow",
        CHALLENGE_PANEL_X - 80.0,
        MIXING_STATIC_CHANNEL_W,
        10.0,
    )
    .translate(0.0, 5.0, CHALLENGE_PANEL_Z + 5.0);

    body + rear_fence
        + static_mixer_shadow
        + challenge_port_label_lands()
        + challenge_sample_splitter_tabs()
        - challenge_port_bores()
}

fn challenge_port_bores() -> Part {
    let mut bores = Part::empty("incubator_stratification_gas_challenge_port_bores");
    for i in 0..CHALLENGE_PORT_COUNT {
        let x = centered_index(i, CHALLENGE_PORT_COUNT, CHALLENGE_PORT_PITCH_X);
        bores = bores
            + centered_cylinder(
                format!(
                    "incubator_stratification_{}_challenge_port_bore",
                    challenge_port_names()[i]
                ),
                CHALLENGE_PORT_D / 2.0,
                CHALLENGE_PANEL_Y + 14.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, CHALLENGE_PANEL_Z / 2.0);
    }
    bores
}

fn challenge_port_label_lands() -> Part {
    let mut labels = Part::empty("incubator_stratification_gas_challenge_port_labels");
    for i in 0..CHALLENGE_PORT_COUNT {
        let x = centered_index(i, CHALLENGE_PORT_COUNT, CHALLENGE_PORT_PITCH_X);
        labels = labels
            + centered_cube(
                format!(
                    "incubator_stratification_{}_challenge_port_label_land",
                    challenge_port_names()[i]
                ),
                54.0,
                14.0,
                3.0,
            )
            .translate(x, -CHALLENGE_PANEL_Y / 2.0 + 22.0, CHALLENGE_PANEL_Z + 1.5);
    }
    labels
}

fn challenge_sample_splitter_tabs() -> Part {
    let mut tabs = Part::empty("incubator_stratification_gas_challenge_splitter_tabs");
    for i in 0..4 {
        let x = centered_index(i, 4, 96.0);
        tabs = tabs
            + centered_cube(
                format!("incubator_stratification_challenge_mixer_alignment_tab_{i}"),
                40.0,
                10.0,
                16.0,
            )
            .translate(x, 28.0, CHALLENGE_PANEL_Z + 8.0);
    }
    tabs
}

fn challenge_port_names() -> [&'static str; CHALLENGE_PORT_COUNT] {
    [
        "dry_air",
        "co2_span",
        "n2_low_o2",
        "mixed_supply",
        "sample_return",
        "exhaust",
    ]
}

fn barcode_certificate_lands() -> Part {
    let plate = centered_cube(
        "incubator_stratification_barcode_certificate_plate",
        TRACE_PLATE_X,
        TRACE_PLATE_Y,
        TRACE_PLATE_Z,
    )
    .translate(0.0, 0.0, TRACE_PLATE_Z / 2.0);
    plate + barcode_lands() + certificate_lands() + traceability_fiducials()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("incubator_stratification_barcode_lands");
    for i in 0..BARCODE_LAND_COUNT {
        let col = i % 7;
        let row = i / 7;
        lands = lands
            + centered_cube(
                format!("incubator_stratification_barcode_land_{i}"),
                38.0,
                14.0,
                3.0,
            )
            .translate(
                centered_index(col, 7, 42.0),
                centered_index(row, 2, 36.0) + 18.0,
                TRACE_PLATE_Z + 1.5,
            );
    }
    lands
}

fn certificate_lands() -> Part {
    let mut lands = Part::empty("incubator_stratification_certificate_lands");
    for i in 0..CERTIFICATE_LAND_COUNT {
        lands = lands
            + centered_cube(
                format!("incubator_stratification_certificate_land_{i}"),
                64.0,
                18.0,
                3.0,
            )
            .translate(
                centered_index(i, CERTIFICATE_LAND_COUNT, 74.0),
                -TRACE_PLATE_Y / 2.0 + 24.0,
                TRACE_PLATE_Z + 1.5,
            );
    }
    lands
}

fn traceability_fiducials() -> Part {
    let left = fiducial_disc("incubator_stratification_trace_plate_left_fiducial").translate(
        -TRACE_PLATE_X / 2.0 + 28.0,
        TRACE_PLATE_Y / 2.0 - 28.0,
        TRACE_PLATE_Z + 2.0,
    );
    let right = fiducial_disc("incubator_stratification_trace_plate_right_fiducial").translate(
        TRACE_PLATE_X / 2.0 - 28.0,
        TRACE_PLATE_Y / 2.0 - 28.0,
        TRACE_PLATE_Z + 2.0,
    );
    left + right
}

fn release_hold_reject_lanes() -> Part {
    let body = centered_cube(
        "incubator_stratification_release_hold_reject_lane_body",
        LANE_BANK_X,
        LANE_BANK_Y,
        LANE_BANK_Z,
    )
    .translate(0.0, 0.0, LANE_BANK_Z / 2.0);
    body - lane_recesses() + lane_sidewalls() + lane_status_tokens()
}

fn lane_recesses() -> Part {
    let mut recesses = Part::empty("incubator_stratification_disposition_lane_recesses");
    for lane in DispositionLane::all() {
        let x = lane_center_x(lane);
        recesses = recesses
            + centered_cube(
                format!("incubator_stratification_{}_lane_recess", lane.name()),
                LANE_SLOT_X,
                LANE_BANK_Y - 34.0,
                15.0,
            )
            .translate(x, 0.0, LANE_BANK_Z - 7.0);
    }
    recesses
}

fn lane_sidewalls() -> Part {
    let mut walls = Part::empty("incubator_stratification_disposition_lane_sidewalls");
    for lane in DispositionLane::all() {
        let x = lane_center_x(lane);
        walls = walls
            + centered_cube(
                format!("incubator_stratification_{}_lane_front_stop", lane.name()),
                LANE_SLOT_X + 18.0,
                8.0,
                24.0,
            )
            .translate(x, -LANE_BANK_Y / 2.0 + 16.0, LANE_BANK_Z + 12.0)
            + centered_cube(
                format!("incubator_stratification_{}_lane_rear_stop", lane.name()),
                LANE_SLOT_X + 18.0,
                8.0,
                24.0,
            )
            .translate(x, LANE_BANK_Y / 2.0 - 16.0, LANE_BANK_Z + 12.0);
    }
    walls
}

fn lane_status_tokens() -> Part {
    let mut tokens = Part::empty("incubator_stratification_disposition_lane_status_tokens");
    for lane in DispositionLane::all() {
        for slot in 0..lane.capacity() {
            let x = lane_center_x(lane);
            let y = centered_index(slot, lane.capacity(), LANE_SLOT_Y);
            tokens = tokens
                + centered_cube(
                    format!(
                        "incubator_stratification_{}_lane_token_slot_{slot}",
                        lane.name()
                    ),
                    34.0,
                    14.0,
                    4.0,
                )
                .translate(x, y, LANE_BANK_Z + 2.0);
        }
    }
    tokens
}

fn clean_used_segregation() -> Part {
    let body = centered_cube(
        "incubator_stratification_clean_used_segregation_tray_body",
        SEG_TRAY_X,
        SEG_TRAY_Y,
        SEG_TRAY_Z,
    )
    .translate(0.0, 0.0, SEG_TRAY_Z / 2.0);
    let divider = centered_cube(
        "incubator_stratification_clean_used_high_divider",
        10.0,
        SEG_TRAY_Y - 26.0,
        SEG_DIVIDER_Z,
    )
    .translate(0.0, 0.0, SEG_DIVIDER_Z / 2.0);
    let clean_label = centered_cube(
        "incubator_stratification_clean_side_label_land",
        SEG_TRAY_X / 2.0 - 22.0,
        16.0,
        3.0,
    )
    .translate(
        -SEG_TRAY_X / 4.0,
        -SEG_TRAY_Y / 2.0 + 22.0,
        SEG_TRAY_Z + 1.5,
    );
    let used_label = centered_cube(
        "incubator_stratification_used_side_label_land",
        SEG_TRAY_X / 2.0 - 22.0,
        16.0,
        3.0,
    )
    .translate(SEG_TRAY_X / 4.0, -SEG_TRAY_Y / 2.0 + 22.0, SEG_TRAY_Z + 1.5);

    body - clean_used_well_cuts() + divider + clean_label + used_label + used_chute_shadow()
}

fn clean_used_well_cuts() -> Part {
    let mut cuts = Part::empty("incubator_stratification_clean_used_well_cuts");
    for side in 0..2 {
        for i in 0..CLEAN_WELL_COUNT {
            let x = if side == 0 {
                -SEG_TRAY_X / 4.0
            } else {
                SEG_TRAY_X / 4.0
            };
            let y = centered_index(i, CLEAN_WELL_COUNT, 31.0) + 18.0;
            cuts = cuts
                + centered_cylinder(
                    format!("incubator_stratification_clean_used_side_{side}_well_{i}"),
                    9.0,
                    20.0,
                    28,
                )
                .translate(x, y, SEG_TRAY_Z - 9.0);
        }
    }
    cuts
}

fn used_chute_shadow() -> Part {
    centered_cube(
        "incubator_stratification_used_probe_chute_shadow",
        SEG_TRAY_X / 2.0 - 20.0,
        28.0,
        18.0,
    )
    .translate(SEG_TRAY_X / 4.0, SEG_TRAY_Y / 2.0 - 28.0, SEG_TRAY_Z + 9.0)
}

fn evidence_camera_bridge() -> Part {
    let left_leg = centered_cube(
        "incubator_stratification_camera_bridge_left_upright",
        28.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(-CAMERA_BRIDGE_X / 2.0 + 42.0, 0.0, CAMERA_BRIDGE_Z / 2.0);
    let right_leg = centered_cube(
        "incubator_stratification_camera_bridge_right_upright",
        28.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(CAMERA_BRIDGE_X / 2.0 - 42.0, 0.0, CAMERA_BRIDGE_Z / 2.0);
    let top_beam = centered_cube(
        "incubator_stratification_camera_bridge_top_beam",
        CAMERA_BRIDGE_X,
        26.0,
        28.0,
    )
    .translate(0.0, 0.0, CAMERA_BRIDGE_Z - 14.0);
    let camera_sled = centered_cube(
        "incubator_stratification_camera_bridge_center_camera_sled",
        128.0,
        54.0,
        26.0,
    )
    .translate(0.0, 0.0, CAMERA_BRIDGE_Z - 52.0);
    let fov_gauge = centered_cube(
        "incubator_stratification_camera_bridge_field_of_view_gauge",
        CAMERA_BRIDGE_X - 160.0,
        4.0,
        6.0,
    )
    .translate(0.0, -CAMERA_BRIDGE_Y / 2.0 + 10.0, CAMERA_CLEARANCE_Z);

    left_leg + right_leg + top_beam + camera_sled + fov_gauge + camera_view_targets()
}

fn camera_view_targets() -> Part {
    let mut targets = Part::empty("incubator_stratification_camera_view_targets");
    for i in 0..CAMERA_TARGET_COUNT {
        let x = centered_index(i, CAMERA_TARGET_COUNT, 205.0);
        targets =
            targets
                + fiducial_disc(&format!("incubator_stratification_camera_view_target_{i}"))
                    .translate(x, -CAMERA_BRIDGE_Y / 2.0 + 18.0, CAMERA_CLEARANCE_Z - 8.0);
    }
    targets
}

fn cable_tubing_strain_relief() -> Part {
    let body = centered_cube(
        "incubator_stratification_cable_tubing_strain_relief_body",
        STRAIN_RELIEF_X,
        STRAIN_RELIEF_Y,
        STRAIN_RELIEF_Z,
    )
    .translate(0.0, 0.0, STRAIN_RELIEF_Z / 2.0);
    let rear_keeper = centered_cube(
        "incubator_stratification_strain_relief_rear_keeper_bar",
        STRAIN_RELIEF_X - 40.0,
        12.0,
        34.0,
    )
    .translate(0.0, STRAIN_RELIEF_Y / 2.0 - 10.0, STRAIN_RELIEF_Z + 17.0);

    body + rear_keeper + sample_tube_comb_labels()
        - sample_tube_comb_notches()
        - service_line_saddle_notches()
}

fn sample_tube_comb_notches() -> Part {
    let mut notches = Part::empty("incubator_stratification_sample_tube_comb_notches");
    for i in 0..SAMPLE_TUBE_COUNT {
        let x = centered_index(i, SAMPLE_TUBE_COUNT, 38.0);
        notches = notches
            + centered_cube(
                format!("incubator_stratification_sample_line_{i}_comb_notch"),
                SAMPLE_TUBE_OD + 3.0,
                STRAIN_RELIEF_Y + 8.0,
                20.0,
            )
            .translate(x, 0.0, STRAIN_RELIEF_Z - 8.0);
    }
    notches
}

fn service_line_saddle_notches() -> Part {
    let mut notches = Part::empty("incubator_stratification_service_line_saddle_notches");
    for i in 0..SERVICE_LINE_COUNT {
        let x = centered_index(i, SERVICE_LINE_COUNT, 64.0);
        notches = notches
            + centered_cylinder(
                format!("incubator_stratification_service_line_{i}_saddle_notch"),
                (SERVICE_TUBE_OD + 1.6) / 2.0,
                STRAIN_RELIEF_Y + 8.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -8.0, STRAIN_RELIEF_Z / 2.0);
    }
    notches
}

fn sample_tube_comb_labels() -> Part {
    let mut labels = Part::empty("incubator_stratification_sample_tube_comb_labels");
    for i in 0..SAMPLE_TUBE_COUNT {
        let x = centered_index(i, SAMPLE_TUBE_COUNT, 38.0);
        labels = labels
            + centered_cube(
                format!("incubator_stratification_sample_line_{i}_barcode_tick_land"),
                18.0,
                10.0,
                3.0,
            )
            .translate(x, -STRAIN_RELIEF_Y / 2.0 + 18.0, STRAIN_RELIEF_Z + 1.5);
    }
    labels
}

fn robot_service_keepout_gauges() -> Part {
    let front = centered_cube(
        "incubator_stratification_robot_front_approach_keepout_bar",
        KEEP_OUT_X,
        10.0,
        20.0,
    )
    .translate(0.0, -KEEP_OUT_Y / 2.0, 10.0);
    let rear = centered_cube(
        "incubator_stratification_rear_service_keepout_bar",
        KEEP_OUT_X,
        10.0,
        24.0,
    )
    .translate(0.0, KEEP_OUT_Y / 2.0, 12.0);
    let left = centered_cube(
        "incubator_stratification_left_service_keepout_bar",
        10.0,
        KEEP_OUT_Y,
        20.0,
    )
    .translate(-KEEP_OUT_X / 2.0, 0.0, 10.0);
    let right = centered_cube(
        "incubator_stratification_right_robot_sweep_keepout_bar",
        10.0,
        KEEP_OUT_Y,
        20.0,
    )
    .translate(KEEP_OUT_X / 2.0, 0.0, 10.0);

    front + rear + left + right + keepout_vertical_masts() + service_sweep_markers()
}

fn keepout_vertical_masts() -> Part {
    let mut masts = Part::empty("incubator_stratification_keepout_vertical_masts");
    for (i, (x, y)) in [
        (-KEEP_OUT_X / 2.0, -KEEP_OUT_Y / 2.0),
        (KEEP_OUT_X / 2.0, -KEEP_OUT_Y / 2.0),
        (-KEEP_OUT_X / 2.0, KEEP_OUT_Y / 2.0),
        (KEEP_OUT_X / 2.0, KEEP_OUT_Y / 2.0),
    ]
    .iter()
    .enumerate()
    {
        masts = masts
            + centered_cube(
                format!("incubator_stratification_keepout_vertical_mast_{i}"),
                18.0,
                18.0,
                KEEP_OUT_Z,
            )
            .translate(*x, *y, KEEP_OUT_Z / 2.0);
    }
    masts
}

fn service_sweep_markers() -> Part {
    let rear_window = centered_cube(
        "incubator_stratification_rear_analyzer_service_window_marker",
        520.0,
        8.0,
        38.0,
    )
    .translate(260.0, KEEP_OUT_Y / 2.0 - SERVICE_REAR_CLEARANCE / 2.0, 38.0);
    let front_robot_window = centered_cube(
        "incubator_stratification_front_robot_pick_window_marker",
        520.0,
        8.0,
        34.0,
    )
    .translate(90.0, -KEEP_OUT_Y / 2.0 + ROBOT_FRONT_CLEARANCE / 2.0, 34.0);
    let left_clean_load_window = centered_cube(
        "incubator_stratification_left_clean_used_service_window_marker",
        8.0,
        260.0,
        34.0,
    )
    .translate(-KEEP_OUT_X / 2.0 + 72.0, -306.0, 34.0);

    rear_window + front_robot_window + left_clean_load_window
}

impl MapZone {
    fn label(self) -> &'static str {
        match self {
            MapZone::Edge => "edge",
            MapZone::Center => "center",
        }
    }
}

fn map_zone(col: usize, row: usize) -> MapZone {
    if col == 0 || row == 0 || col == RACK_COLS - 1 || row == RACK_ROWS - 1 {
        MapZone::Edge
    } else {
        MapZone::Center
    }
}

fn edge_position_count() -> usize {
    let mut count = 0;
    for row in 0..RACK_ROWS {
        for col in 0..RACK_COLS {
            if map_zone(col, row) == MapZone::Edge {
                count += 1;
            }
        }
    }
    count
}

fn center_position_count() -> usize {
    SLOT_COUNT - edge_position_count()
}

fn position_index(col: usize, row: usize) -> usize {
    row * RACK_COLS + col
}

fn rack_slot_center(col: usize, row: usize) -> (f64, f64) {
    (
        centered_index(col, RACK_COLS, SLOT_PITCH_X),
        centered_index(row, RACK_ROWS, SLOT_PITCH_Y),
    )
}

fn token_position(col: usize, row: usize) -> (f64, f64) {
    (
        centered_index(col, RACK_COLS, 38.0),
        centered_index(row, RACK_ROWS, 74.0),
    )
}

fn lane_center_x(lane: DispositionLane) -> f64 {
    centered_index(lane.index(), LANE_COUNT, 126.0)
}

fn total_lane_capacity() -> usize {
    DispositionLane::all()
        .iter()
        .map(|lane| lane.capacity())
        .sum()
}

fn clean_rect() -> Rect {
    rect(
        "clean_seg_side",
        (-SEG_TRAY_X / 4.0, 0.0),
        SEG_TRAY_X / 2.0 - CLEAN_USED_GAP,
        SEG_TRAY_Y,
    )
}

fn used_rect() -> Rect {
    rect(
        "used_seg_side",
        (SEG_TRAY_X / 4.0, 0.0),
        SEG_TRAY_X / 2.0 - CLEAN_USED_GAP,
        SEG_TRAY_Y,
    )
}

fn clean_used_gap() -> f64 {
    let clean = clean_rect();
    let used = used_rect();
    let clean_right = clean.center.0 + clean.x / 2.0;
    let used_left = used.center.0 - used.x / 2.0;
    used_left - clean_right
}

fn gasket_frame_xy(name: &str, x: f64, y: f64, wall: f64, z: f64) -> Part {
    let outer = centered_cube(format!("{name}_outer"), x, y, z);
    let inner = centered_cube(
        format!("{name}_inner_cut"),
        x - 2.0 * wall,
        y - 2.0 * wall,
        z + 2.0,
    );
    outer - inner
}

fn annular_disc(name: &str, outer_d: f64, inner_d: f64, z: f64) -> Part {
    centered_cylinder(format!("{name}_outer"), outer_d / 2.0, z, 42)
        - centered_cylinder(format!("{name}_inner"), inner_d / 2.0, z + 2.0, 42)
}

fn fiducial_disc(name: &str) -> Part {
    let disc = centered_cylinder(format!("{name}_disc"), 10.0, 4.0, 32);
    let slot_x = centered_cube(format!("{name}_slot_x"), 18.0, 2.5, 5.0);
    let slot_y = centered_cube(format!("{name}_slot_y"), 2.5, 18.0, 5.0);
    disc - slot_x - slot_y
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn output_names_are_stable_and_unique() {
        let unique: HashSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 14);
        assert!(OUTPUTS.iter().all(|path| path
            .starts_with("output/closed_incubator_o2_co2_stratification_mapping_station_")));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
        assert!(OUTPUTS[0].ends_with("_base_containment_deck.stl"));
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }

    #[test]
    fn feature_list_covers_requested_validation_surfaces() {
        for feature in [
            "base_containment_deck",
            "rack_slot_surrogate_fixture",
            "vertical_sampling_mast_positions",
            "o2_co2_probe_docks",
            "reference_gas_calibration_puck_lands",
            "edge_center_position_tokens",
            "gas_mixing_challenge_inlet_placeholders",
            "barcode_certificate_lands",
            "release_hold_reject_lanes",
            "clean_used_segregation",
            "evidence_camera_bridge",
            "cable_tubing_strain_relief",
            "robot_service_keepout_gauges",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn rack_map_covers_edge_center_positions_and_mast_tiers() {
        let mut edge = 0;
        let mut center = 0;
        for row in 0..RACK_ROWS {
            for col in 0..RACK_COLS {
                let (x, y) = rack_slot_center(col, row);
                assert!(x.abs() < RACK_FIXTURE_X / 2.0 - SLOT_SURROGATE_X / 2.0);
                assert!(y.abs() < RACK_FIXTURE_Y / 2.0 - SLOT_SURROGATE_Y / 2.0);
                match map_zone(col, row) {
                    MapZone::Edge => edge += 1,
                    MapZone::Center => center += 1,
                }
            }
        }

        assert_eq!(SLOT_COUNT, 12);
        assert_eq!(edge, EDGE_POSITION_COUNT);
        assert_eq!(center, CENTER_POSITION_COUNT);
        assert_eq!(MAST_TIER_COUNT, 3);
        assert!(LOW_SAMPLE_Z < MID_SAMPLE_Z && MID_SAMPLE_Z < HIGH_SAMPLE_Z);
    }

    #[test]
    fn socketed_modules_fit_without_unplanned_overlap() {
        for item in socket_rects() {
            assert!(item.fits_inside_deck(), "{} outside deck", item.name);
        }

        for (a, b) in non_overlap_pairs() {
            assert!(!a.overlaps(b), "{} overlaps {}", a.name, b.name);
        }
    }

    #[test]
    fn probe_calibration_and_challenge_counts_match_station_plan() {
        assert_eq!(O2_PROBE_DOCK_COUNT, 3);
        assert_eq!(CO2_PROBE_DOCK_COUNT, 3);
        assert_eq!(PROBE_DOCK_COUNT, 6);
        assert_eq!(PUCK_LAND_COUNT, 6);
        assert_eq!(CHALLENGE_PORT_COUNT, 6);
        assert_eq!(SAMPLE_TUBE_COUNT, SLOT_COUNT);
        assert!(CHALLENGE_PORT_D > SERVICE_TUBE_OD + 1.0);
    }

    #[test]
    fn disposition_and_segregation_capacity_cover_all_positions() {
        assert_eq!(DispositionLane::all().len(), LANE_COUNT);
        assert_eq!(total_lane_capacity(), SLOT_COUNT);
        assert_eq!(
            RELEASE_LANE_CAPACITY + HOLD_LANE_CAPACITY + REJECT_LANE_CAPACITY,
            SLOT_COUNT
        );
        assert_eq!(CLEAN_WELL_COUNT, SLOT_COUNT / 2);
        assert_eq!(USED_WELL_COUNT, SLOT_COUNT / 2);
        assert!(clean_used_gap() >= CLEAN_USED_GAP);
        assert!(SEG_DIVIDER_Z > SEG_TRAY_Z);
    }

    #[test]
    fn full_design_constraints_pass() {
        assert_design_constraints();
    }

    #[test]
    fn evidence_and_keepout_clearances_are_serviceable() {
        assert!(CAMERA_CLEARANCE_Z > MAST_HEIGHT);
        assert!(ROBOT_Z_CLEARANCE > CAMERA_BRIDGE_Z + DECK_Z);
        assert!(ROBOT_FRONT_CLEARANCE >= 400.0);
        assert!(SERVICE_REAR_CLEARANCE >= 240.0);
        assert!(KEEP_OUT_X < DECK_X && KEEP_OUT_Y < DECK_Y);
    }
}
