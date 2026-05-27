use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed chip-cassette thermal-contact preload uniformity station.
//
// Intent:
// - Verify that a sealed chip cassette receives repeatable thermal-pad preload
//   across edge and center positions before the cassette is released to closed
//   culture work.
// - Keep the cassette datum nest, thermal coupon grid, preload force/height
//   witness pockets, torque/preload token handling, dummy thermal loads,
//   temperature probe docks, contact-resistance references, compression witness
//   film, release/hold/reject disposition, evidence capture, and robot/service
//   keepouts physically segregated.
// - Model station-level fixtures and evidence locations only. Force limits,
//   contact-resistance acceptance criteria, probe calibration, and lot-release
//   decisions remain controlled by external quality protocols.

const OUTPUT_PREFIX: &str = "closed_chip_cassette_thermal_contact_preload_uniformity_station";

const OUTPUTS: [&str; 13] = [
    "output/closed_chip_cassette_thermal_contact_preload_uniformity_station_base_containment_deck.stl",
    "output/closed_chip_cassette_thermal_contact_preload_uniformity_station_cassette_datum_nest.stl",
    "output/closed_chip_cassette_thermal_contact_preload_uniformity_station_thermal_pad_coupon_grid.stl",
    "output/closed_chip_cassette_thermal_contact_preload_uniformity_station_preload_force_height_witness_pockets.stl",
    "output/closed_chip_cassette_thermal_contact_preload_uniformity_station_torque_preload_token_rails.stl",
    "output/closed_chip_cassette_thermal_contact_preload_uniformity_station_edge_center_thermal_dummy_loads.stl",
    "output/closed_chip_cassette_thermal_contact_preload_uniformity_station_temperature_probe_docks.stl",
    "output/closed_chip_cassette_thermal_contact_preload_uniformity_station_contact_resistance_reference_blocks.stl",
    "output/closed_chip_cassette_thermal_contact_preload_uniformity_station_compression_witness_film_pockets.stl",
    "output/closed_chip_cassette_thermal_contact_preload_uniformity_station_release_hold_reject_lanes.stl",
    "output/closed_chip_cassette_thermal_contact_preload_uniformity_station_evidence_bridge.stl",
    "output/closed_chip_cassette_thermal_contact_preload_uniformity_station_robot_service_keepouts.stl",
    "output/closed_chip_cassette_thermal_contact_preload_uniformity_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "cassette_datum_nest",
    "thermal_pad_coupon_grid",
    "preload_force_height_witness_pockets",
    "torque_preload_token_rails",
    "edge_center_thermal_dummy_loads",
    "temperature_probe_docks",
    "contact_resistance_reference_blocks",
    "compression_witness_film_pockets",
    "release_hold_reject_lanes",
    "evidence_bridge",
    "robot_service_keepouts",
];

const CHIP_COLS: usize = 4;
const CHIP_ROWS: usize = 4;
const CHIP_POSITION_COUNT: usize = CHIP_COLS * CHIP_ROWS;
const EDGE_POSITION_COUNT: usize = 12;
const CENTER_POSITION_COUNT: usize = CHIP_POSITION_COUNT - EDGE_POSITION_COUNT;

const DECK_X: f64 = 1980.0;
const DECK_Y: f64 = 1120.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const SOCKET_DEPTH: f64 = 6.0;
const MOUNT_HOLE_D: f64 = 6.8;

const CHIP_GUTTER_X: f64 = 9.0;
const CHIP_GUTTER_Y: f64 = 9.0;
const CHIP_PITCH_X: f64 = REVC_CHIP_LENGTH + CHIP_GUTTER_X;
const CHIP_PITCH_Y: f64 = REVC_CHIP_WIDTH + CHIP_GUTTER_Y;
const CHIP_ARRAY_X: f64 =
    CHIP_COLS as f64 * REVC_CHIP_LENGTH + (CHIP_COLS as f64 - 1.0) * CHIP_GUTTER_X;
const CHIP_ARRAY_Y: f64 =
    CHIP_ROWS as f64 * REVC_CHIP_WIDTH + (CHIP_ROWS as f64 - 1.0) * CHIP_GUTTER_Y;
const CARRIER_MARGIN_X: f64 = 56.0;
const CARRIER_MARGIN_Y: f64 = 50.0;
const CARRIER_X: f64 = CHIP_ARRAY_X + 2.0 * CARRIER_MARGIN_X;
const CARRIER_Y: f64 = CHIP_ARRAY_Y + 2.0 * CARRIER_MARGIN_Y;
const CARRIER_Z: f64 = REVC_TOTAL_HEIGHT + 28.0;

const NEST_POS: (f64, f64) = (-430.0, 150.0);
const NEST_X: f64 = 720.0;
const NEST_Y: f64 = 530.0;
const NEST_Z: f64 = 42.0;
const NEST_RAIL_W: f64 = 16.0;
const NEST_RAIL_Z: f64 = 30.0;
const DATUM_PIN_COUNT: usize = 4;
const DATUM_PIN_D: f64 = 7.0;
const LATCH_PAD_COUNT: usize = 6;

const COUPON_POS: (f64, f64) = (385.0, 365.0);
const COUPON_PANEL_X: f64 = 560.0;
const COUPON_PANEL_Y: f64 = 220.0;
const COUPON_PANEL_Z: f64 = 28.0;
const COUPON_COLS: usize = 4;
const COUPON_ROWS: usize = 3;
const COUPON_COUNT: usize = COUPON_COLS * COUPON_ROWS;
const PAD_COUPON_X: f64 = 82.0;
const PAD_COUPON_Y: f64 = 42.0;
const PAD_COUPON_DEPTH: f64 = 8.0;
const COUPON_PITCH_X: f64 = 118.0;
const COUPON_PITCH_Y: f64 = 62.0;
const GAP_SHIM_SLOTS_PER_COUPON: usize = 3;

const PRELOAD_POS: (f64, f64) = (435.0, 130.0);
const PRELOAD_PANEL_X: f64 = 520.0;
const PRELOAD_PANEL_Y: f64 = 200.0;
const PRELOAD_PANEL_Z: f64 = 38.0;
const PRELOAD_STATION_COLS: usize = 4;
const PRELOAD_STATION_ROWS: usize = 2;
const PRELOAD_STATION_COUNT: usize = PRELOAD_STATION_COLS * PRELOAD_STATION_ROWS;
const PRELOAD_PITCH_X: f64 = 104.0;
const PRELOAD_PITCH_Y: f64 = 82.0;
const FORCE_PUCK_D: f64 = 30.0;
const HEIGHT_PROBE_SLOT_X: f64 = 34.0;
const HEIGHT_PROBE_SLOT_Y: f64 = 14.0;
const NOMINAL_PAD_THICKNESS: f64 = 2.0;
const NOMINAL_COMPRESSION: f64 = 0.18;
const LOW_COMPRESSION: f64 = 0.12;
const HIGH_COMPRESSION: f64 = 0.24;

const TOKEN_POS: (f64, f64) = (-465.0, -290.0);
const TOKEN_PANEL_X: f64 = 650.0;
const TOKEN_PANEL_Y: f64 = 190.0;
const TOKEN_PANEL_Z: f64 = 18.0;
const TOKEN_RAIL_COUNT: usize = 3;
const TOKENS_PER_RAIL: usize = 8;
const TOKEN_SLOT_X: f64 = 52.0;
const TOKEN_SLOT_Y: f64 = 28.0;
const TOKEN_RAIL_PITCH_Y: f64 = 50.0;

const DUMMY_POS: (f64, f64) = (420.0, -125.0);
const DUMMY_PANEL_X: f64 = 520.0;
const DUMMY_PANEL_Y: f64 = 220.0;
const DUMMY_PANEL_Z: f64 = 28.0;
const EDGE_DUMMY_D: f64 = 23.0;
const CENTER_DUMMY_D: f64 = 31.0;
const DUMMY_LOAD_Z: f64 = 18.0;
const DUMMY_POSITION_PITCH_X: f64 = 70.0;
const DUMMY_POSITION_PITCH_Y: f64 = 46.0;

const PROBE_POS: (f64, f64) = (420.0, -365.0);
const PROBE_PANEL_X: f64 = 520.0;
const PROBE_PANEL_Y: f64 = 170.0;
const PROBE_PANEL_Z: f64 = 32.0;
const PROBE_DOCK_COUNT: usize = 10;
const PROBE_COLS: usize = 5;
const PROBE_ROWS: usize = 2;
const PROBE_SOCKET_D: f64 = 6.0;
const PROBE_DOCK_PITCH_X: f64 = 82.0;
const PROBE_DOCK_PITCH_Y: f64 = 76.0;

const REFERENCE_POS: (f64, f64) = (-870.0, -55.0);
const REFERENCE_PANEL_X: f64 = 150.0;
const REFERENCE_PANEL_Y: f64 = 260.0;
const REFERENCE_PANEL_Z: f64 = 34.0;
const REFERENCE_BLOCK_COUNT: usize = 5;
const REFERENCE_BLOCK_X: f64 = 94.0;
const REFERENCE_BLOCK_Y: f64 = 34.0;
const REFERENCE_BLOCK_Z: f64 = 20.0;
const REFERENCE_BLOCK_PITCH_Y: f64 = 44.0;

const FILM_POS: (f64, f64) = (-50.0, -255.0);
const FILM_PANEL_X: f64 = 160.0;
const FILM_PANEL_Y: f64 = 260.0;
const FILM_PANEL_Z: f64 = 18.0;
const FILM_POCKET_COUNT: usize = 6;
const FILM_POCKET_X: f64 = 112.0;
const FILM_POCKET_Y: f64 = 28.0;
const FILM_POCKET_PITCH_Y: f64 = 36.0;

const LANE_POS: (f64, f64) = (-465.0, -470.0);
const LANE_PANEL_X: f64 = 650.0;
const LANE_PANEL_Y: f64 = 100.0;
const LANE_PANEL_Z: f64 = 28.0;
const LANE_COUNT: usize = 3;
const LANE_SLOT_X: f64 = 148.0;
const LANE_SLOT_Y: f64 = 62.0;
const LANE_PITCH_X: f64 = 190.0;
const RELEASE_CAPACITY: usize = 6;
const HOLD_CAPACITY: usize = 4;
const REJECT_CAPACITY: usize = 6;

const EVIDENCE_POS: (f64, f64) = (0.0, 505.0);
const EVIDENCE_SPAN_X: f64 = 1680.0;
const EVIDENCE_BRIDGE_Y: f64 = 70.0;
const EVIDENCE_POST_W: f64 = 28.0;
const EVIDENCE_POST_Z: f64 = 230.0;
const EVIDENCE_BEAM_Z: f64 = 26.0;
const CAMERA_COUNT: usize = 4;
const LIGHT_BAR_COUNT: usize = 2;
const EVIDENCE_CLEARANCE_Z: f64 = 190.0;

const ROBOT_FRONT_CLEARANCE_Y: f64 = 136.0;
const SERVICE_REAR_CLEARANCE_Y: f64 = 108.0;
const SIDE_SERVICE_CLEARANCE_X: f64 = 118.0;
const ROBOT_PICK_CLEARANCE_Z: f64 = 180.0;
const SERVICE_CLEARANCE_Z: f64 = 285.0;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ChipZone {
    Edge,
    Center,
}

impl ChipZone {
    fn label(self) -> &'static str {
        match self {
            ChipZone::Edge => "edge",
            ChipZone::Center => "center",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TokenRail {
    Torque,
    Preload,
    Witness,
}

impl TokenRail {
    fn all() -> [TokenRail; TOKEN_RAIL_COUNT] {
        [TokenRail::Torque, TokenRail::Preload, TokenRail::Witness]
    }

    fn index(self) -> usize {
        match self {
            TokenRail::Torque => 0,
            TokenRail::Preload => 1,
            TokenRail::Witness => 2,
        }
    }

    fn label(self) -> &'static str {
        match self {
            TokenRail::Torque => "torque",
            TokenRail::Preload => "preload",
            TokenRail::Witness => "witness",
        }
    }
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

    fn label(self) -> &'static str {
        match self {
            DispositionLane::Release => "release",
            DispositionLane::Hold => "hold",
            DispositionLane::Reject => "reject",
        }
    }

    fn capacity(self) -> usize {
        match self {
            DispositionLane::Release => RELEASE_CAPACITY,
            DispositionLane::Hold => HOLD_CAPACITY,
            DispositionLane::Reject => REJECT_CAPACITY,
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
        let usable_x = DECK_X / 2.0 - RIM_W - 20.0;
        let usable_y = DECK_Y / 2.0 - RIM_W - 20.0;

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

    let base = base_containment_deck();
    export(OUTPUTS[0], &base);

    let nest = cassette_datum_nest();
    export(OUTPUTS[1], &nest);

    let coupons = thermal_pad_coupon_grid();
    export(OUTPUTS[2], &coupons);

    let preload = preload_force_height_witness_pockets();
    export(OUTPUTS[3], &preload);

    let token_rails = torque_preload_token_rails();
    export(OUTPUTS[4], &token_rails);

    let dummy_loads = edge_center_thermal_dummy_loads();
    export(OUTPUTS[5], &dummy_loads);

    let probes = temperature_probe_docks();
    export(OUTPUTS[6], &probes);

    let references = contact_resistance_reference_blocks();
    export(OUTPUTS[7], &references);

    let film = compression_witness_film_pockets();
    export(OUTPUTS[8], &film);

    let lanes = release_hold_reject_lanes();
    export(OUTPUTS[9], &lanes);

    let bridge = evidence_bridge();
    export(OUTPUTS[10], &bridge);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[11], &keepouts);

    let assembly = station_assembly();
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed chip-cassette thermal-contact preload uniformity station:");
    println!(
        "  Chip cassette:          {CHIP_COLS}x{CHIP_ROWS} positions, {EDGE_POSITION_COUNT} edge / {CENTER_POSITION_COUNT} center"
    );
    println!(
        "  Thermal coupons:        {COUPON_COUNT} pad coupons with {GAP_SHIM_SLOTS_PER_COUPON} shim references each"
    );
    println!(
        "  Preload witnesses:      {PRELOAD_STATION_COUNT} force/height pockets across {:.0}% nominal compression",
        NOMINAL_COMPRESSION * 100.0
    );
    println!(
        "  Token handling:         {TOKEN_RAIL_COUNT} rails x {TOKENS_PER_RAIL} tokens for torque, preload, and witness states"
    );
    println!(
        "  Thermal evidence:       {PROBE_DOCK_COUNT} temperature probe docks, {REFERENCE_BLOCK_COUNT} contact-resistance references, {FILM_POCKET_COUNT} film pockets"
    );
    println!(
        "  Disposition/evidence:   release/hold/reject lanes for {} cassette tokens, {CAMERA_COUNT} cameras, {LIGHT_BAR_COUNT} light bars",
        disposition_capacity()
    );
    println!("  STL outputs:            {} files", OUTPUTS.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    base_containment_deck()
        + cassette_datum_nest().translate(NEST_POS.0, NEST_POS.1, deck_top_z())
        + thermal_pad_coupon_grid().translate(COUPON_POS.0, COUPON_POS.1, deck_top_z())
        + preload_force_height_witness_pockets().translate(
            PRELOAD_POS.0,
            PRELOAD_POS.1,
            deck_top_z(),
        )
        + torque_preload_token_rails().translate(TOKEN_POS.0, TOKEN_POS.1, deck_top_z())
        + edge_center_thermal_dummy_loads().translate(DUMMY_POS.0, DUMMY_POS.1, deck_top_z())
        + temperature_probe_docks().translate(PROBE_POS.0, PROBE_POS.1, deck_top_z())
        + contact_resistance_reference_blocks().translate(
            REFERENCE_POS.0,
            REFERENCE_POS.1,
            deck_top_z(),
        )
        + compression_witness_film_pockets().translate(FILM_POS.0, FILM_POS.1, deck_top_z())
        + release_hold_reject_lanes().translate(LANE_POS.0, LANE_POS.1, deck_top_z())
        + evidence_bridge().translate(EVIDENCE_POS.0, EVIDENCE_POS.1, deck_top_z())
        + robot_service_keepouts()
}

fn deck_top_z() -> f64 {
    DECK_Z
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 13);
    assert_eq!(REQUIRED_FEATURES.len(), 11);
    assert_eq!(CHIP_POSITION_COUNT, 16);
    assert_eq!(edge_position_count(), EDGE_POSITION_COUNT);
    assert_eq!(center_position_count(), CENTER_POSITION_COUNT);
    assert_eq!(COUPON_COUNT, COUPON_COLS * COUPON_ROWS);
    assert_eq!(
        PRELOAD_STATION_COUNT,
        PRELOAD_STATION_ROWS * PRELOAD_STATION_COLS
    );
    assert_eq!(TokenRail::all().len(), TOKEN_RAIL_COUNT);
    assert_eq!(DispositionLane::all().len(), LANE_COUNT);
    assert_eq!(disposition_capacity(), CHIP_POSITION_COUNT);
    assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
    assert!(OUTPUTS.iter().all(|path| path.starts_with("output/")));
    assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
    assert!(CARRIER_X > CHIP_ARRAY_X + 100.0);
    assert!(CARRIER_Y > CHIP_ARRAY_Y + 90.0);
    assert!(CARRIER_Z > REVC_TOTAL_HEIGHT + 25.0);
    assert!(LOW_COMPRESSION < NOMINAL_COMPRESSION);
    assert!(NOMINAL_COMPRESSION < HIGH_COMPRESSION);
    assert!(EVIDENCE_CLEARANCE_Z > NEST_Z + CARRIER_Z + 90.0);
    assert!(ROBOT_PICK_CLEARANCE_Z > NEST_Z + CARRIER_Z + 80.0);
    assert!(SERVICE_CLEARANCE_Z > ROBOT_PICK_CLEARANCE_Z);

    for rect in module_footprints() {
        assert!(
            rect.fits_inside_deck(),
            "{} exceeds deck footprint",
            rect.name
        );
    }
    assert!(!critical_footprints_overlap());
}

fn base_containment_deck() -> Part {
    let deck = centered_cube(
        format!("{OUTPUT_PREFIX}_base_plate"),
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let leak_pan = centered_cube(
        format!("{OUTPUT_PREFIX}_recessed_thermal_evidence_pan"),
        DECK_X - 2.0 * (RIM_W + 56.0),
        DECK_Y - 2.0 * (RIM_W + 56.0),
        8.0,
    )
    .translate(0.0, -4.0, DECK_Z - 4.0);
    let front_drain = centered_cube(
        format!("{OUTPUT_PREFIX}_front_condensate_drain_groove"),
        DECK_X - 260.0,
        24.0,
        10.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 56.0, DECK_Z - 5.0);
    let drain_port = centered_cylinder(
        format!("{OUTPUT_PREFIX}_closed_drain_port"),
        9.0 / 2.0,
        60.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 90.0, -DECK_Y / 2.0 + 28.0, DECK_Z / 2.0);

    deck - leak_pan - front_drain - drain_port - mount_holes() - module_socket_recesses()
        + deck_perimeter_rims()
        + deck_flow_landmarks()
}

fn mount_holes() -> Part {
    let mut holes = Part::empty(format!("{OUTPUT_PREFIX}_mount_holes"));
    for (index, (x, y)) in mount_points().into_iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_m6_mount_hole_{index}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 4.0,
                32,
            )
            .translate(x, y, DECK_Z / 2.0);
    }
    holes
}

fn mount_points() -> [(f64, f64); 10] {
    [
        (-DECK_X / 2.0 + 56.0, -DECK_Y / 2.0 + 56.0),
        (DECK_X / 2.0 - 56.0, -DECK_Y / 2.0 + 56.0),
        (-DECK_X / 2.0 + 56.0, DECK_Y / 2.0 - 56.0),
        (DECK_X / 2.0 - 56.0, DECK_Y / 2.0 - 56.0),
        (0.0, -DECK_Y / 2.0 + 56.0),
        (0.0, DECK_Y / 2.0 - 56.0),
        (-DECK_X / 2.0 + 56.0, 0.0),
        (DECK_X / 2.0 - 56.0, 0.0),
        (-DECK_X / 2.0 + 56.0, -260.0),
        (DECK_X / 2.0 - 56.0, -260.0),
    ]
}

fn module_socket_recesses() -> Part {
    let mut sockets = Part::empty(format!("{OUTPUT_PREFIX}_module_socket_recesses"));
    for rect in module_footprints() {
        sockets = sockets
            + centered_cube(
                format!("{}_{}_socket_recess", OUTPUT_PREFIX, rect.name),
                rect.x + 18.0,
                rect.y + 18.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(
                rect.center.0,
                rect.center.1,
                DECK_Z - SOCKET_DEPTH / 2.0 + 0.2,
            );
    }
    sockets
}

fn deck_perimeter_rims() -> Part {
    let front = centered_cube(
        format!("{OUTPUT_PREFIX}_front_low_spill_rim"),
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        format!("{OUTPUT_PREFIX}_rear_service_spill_rim"),
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let left = centered_cube(
        format!("{OUTPUT_PREFIX}_left_robot_spill_rim"),
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube(
        format!("{OUTPUT_PREFIX}_right_service_spill_rim"),
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);

    front + rear + left + right
}

fn deck_flow_landmarks() -> Part {
    let nest_to_preload = centered_cube(
        format!("{OUTPUT_PREFIX}_nest_to_preload_flow_land"),
        390.0,
        7.0,
        6.0,
    )
    .translate(-20.0, 86.0, DECK_Z + 3.0);
    let preload_to_dummy = centered_cube(
        format!("{OUTPUT_PREFIX}_preload_to_dummy_flow_land"),
        225.0,
        7.0,
        6.0,
    )
    .rotate(0.0, 0.0, -90.0)
    .translate(620.0, -5.0, DECK_Z + 3.0);
    let film_to_lanes = centered_cube(
        format!("{OUTPUT_PREFIX}_film_to_disposition_flow_land"),
        210.0,
        7.0,
        6.0,
    )
    .rotate(0.0, 0.0, -35.0)
    .translate(-200.0, -365.0, DECK_Z + 3.0);
    let reference_to_coupons = centered_cube(
        format!("{OUTPUT_PREFIX}_reference_to_coupon_flow_land"),
        430.0,
        7.0,
        6.0,
    )
    .rotate(0.0, 0.0, 33.0)
    .translate(-290.0, 190.0, DECK_Z + 3.0);

    nest_to_preload + preload_to_dummy + film_to_lanes + reference_to_coupons
}

fn cassette_datum_nest() -> Part {
    let base = centered_cube(
        format!("{OUTPUT_PREFIX}_cassette_datum_nest_body"),
        NEST_X,
        NEST_Y,
        NEST_Z,
    )
    .translate(0.0, 0.0, NEST_Z / 2.0);
    let carrier_recess = centered_cube(
        format!("{OUTPUT_PREFIX}_cassette_carrier_recess"),
        CARRIER_X + 8.0,
        CARRIER_Y + 8.0,
        CARRIER_Z + 6.0,
    )
    .translate(0.0, 0.0, NEST_Z - (CARRIER_Z + 6.0) / 2.0 + 3.0);

    base - carrier_recess - chip_position_pocket_cuts()
        + nest_datum_rails()
        + nest_datum_pins()
        + cassette_latch_pads()
        + chip_contact_window_marks()
        + preload_platen_stop_lands()
}

fn chip_position_pocket_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_chip_position_pocket_cuts"));
    for position in 0..CHIP_POSITION_COUNT {
        let (x, y) = chip_position_xy(position);
        cuts = cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_cassette_position_{position:02}_pocket_cut"),
                REVC_CHIP_LENGTH + 4.0,
                REVC_CHIP_WIDTH + 4.0,
                REVC_TOTAL_HEIGHT + 12.0,
            )
            .translate(x, y, NEST_Z - (REVC_TOTAL_HEIGHT + 12.0) / 2.0 + 2.0);
    }
    cuts
}

fn nest_datum_rails() -> Part {
    let left = centered_cube(
        format!("{OUTPUT_PREFIX}_nest_left_hard_datum_rail"),
        NEST_RAIL_W,
        CARRIER_Y + 44.0,
        NEST_RAIL_Z,
    )
    .translate(-CARRIER_X / 2.0 - 18.0, 0.0, NEST_Z + NEST_RAIL_Z / 2.0);
    let rear = centered_cube(
        format!("{OUTPUT_PREFIX}_nest_rear_hard_datum_stop"),
        CARRIER_X + 48.0,
        NEST_RAIL_W,
        NEST_RAIL_Z,
    )
    .translate(0.0, CARRIER_Y / 2.0 + 18.0, NEST_Z + NEST_RAIL_Z / 2.0);
    let front = centered_cube(
        format!("{OUTPUT_PREFIX}_nest_front_robot_load_low_rail"),
        CARRIER_X + 44.0,
        12.0,
        16.0,
    )
    .translate(0.0, -CARRIER_Y / 2.0 - 20.0, NEST_Z + 8.0);
    let right_spring = centered_cube(
        format!("{OUTPUT_PREFIX}_nest_right_preload_spring_datum_strip"),
        12.0,
        CARRIER_Y + 22.0,
        22.0,
    )
    .translate(CARRIER_X / 2.0 + 20.0, 0.0, NEST_Z + 11.0);

    left + rear + front + right_spring
}

fn nest_datum_pins() -> Part {
    let mut pins = Part::empty(format!("{OUTPUT_PREFIX}_nest_datum_pins"));
    for (index, (x, y)) in datum_pin_points().into_iter().enumerate() {
        let boss = centered_cylinder(
            format!("{OUTPUT_PREFIX}_datum_pin_boss_{index}"),
            13.0,
            8.0,
            36,
        )
        .translate(x, y, NEST_Z + 4.0);
        let pin = centered_cylinder(
            format!("{OUTPUT_PREFIX}_datum_pin_{index}"),
            DATUM_PIN_D / 2.0,
            20.0,
            32,
        )
        .translate(x, y, NEST_Z + 16.0);
        pins = pins + boss + pin;
    }
    pins
}

fn cassette_latch_pads() -> Part {
    let mut pads = Part::empty(format!("{OUTPUT_PREFIX}_cassette_latch_pads"));
    for index in 0..LATCH_PAD_COUNT {
        let x = centered_index(index, LATCH_PAD_COUNT, 88.0);
        let pad = centered_cube(
            format!("{OUTPUT_PREFIX}_latch_pad_{index}"),
            54.0,
            28.0,
            12.0,
        )
        .translate(x, -CARRIER_Y / 2.0 - 56.0, NEST_Z + 6.0);
        let screw = centered_cylinder(
            format!("{OUTPUT_PREFIX}_latch_pad_screw_clearance_{index}"),
            3.4 / 2.0,
            18.0,
            24,
        )
        .translate(x, -CARRIER_Y / 2.0 - 56.0, NEST_Z + 6.0);
        pads = pads + (pad - screw);
    }
    pads
}

fn chip_contact_window_marks() -> Part {
    let mut marks = Part::empty(format!("{OUTPUT_PREFIX}_chip_contact_window_marks"));
    for position in 0..CHIP_POSITION_COUNT {
        let (x, y) = chip_position_xy(position);
        let zone = chip_zone(position);
        let marker_d = match zone {
            ChipZone::Edge => 14.0,
            ChipZone::Center => 20.0,
        };
        marks = marks
            + centered_cube(
                format!(
                    "{OUTPUT_PREFIX}_position_{position:02}_{}_thermal_window_x",
                    zone.label()
                ),
                REVC_CHIP_LENGTH - 24.0,
                4.0,
                5.0,
            )
            .translate(x, y, NEST_Z + 2.5)
            + centered_cube(
                format!(
                    "{OUTPUT_PREFIX}_position_{position:02}_{}_thermal_window_y",
                    zone.label()
                ),
                4.0,
                REVC_CHIP_WIDTH - 20.0,
                5.0,
            )
            .translate(x, y, NEST_Z + 2.5)
            + centered_cylinder(
                format!(
                    "{OUTPUT_PREFIX}_position_{position:02}_{}_zone_marker",
                    zone.label()
                ),
                marker_d / 2.0,
                4.0,
                32,
            )
            .translate(
                x - REVC_CHIP_LENGTH / 2.0 + 20.0,
                y + REVC_CHIP_WIDTH / 2.0 - 18.0,
                NEST_Z + 2.0,
            );
    }
    marks
}

fn preload_platen_stop_lands() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_preload_platen_stop_lands"));
    for (index, (x, y)) in carrier_screw_points().into_iter().enumerate() {
        lands = lands
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_preload_platen_stop_land_{index}"),
                14.0,
                5.0,
                36,
            )
            .translate(x, y, NEST_Z + 2.5);
    }
    lands
}

fn thermal_pad_coupon_grid() -> Part {
    let base = centered_cube(
        format!("{OUTPUT_PREFIX}_thermal_pad_coupon_grid_base"),
        COUPON_PANEL_X,
        COUPON_PANEL_Y,
        COUPON_PANEL_Z,
    )
    .translate(0.0, 0.0, COUPON_PANEL_Z / 2.0);
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_thermal_pad_coupon_pocket_cuts"));
    let mut features = Part::empty(format!("{OUTPUT_PREFIX}_thermal_pad_coupon_features"));

    for coupon in 0..COUPON_COUNT {
        let (x, y) = coupon_xy(coupon);
        cuts = cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_pad_coupon_{coupon:02}_pocket_cut"),
                PAD_COUPON_X,
                PAD_COUPON_Y,
                PAD_COUPON_DEPTH + 0.6,
            )
            .translate(x, y, COUPON_PANEL_Z - PAD_COUPON_DEPTH / 2.0 + 0.3);

        for slot in 0..GAP_SHIM_SLOTS_PER_COUPON {
            cuts = cuts
                + centered_cube(
                    format!("{OUTPUT_PREFIX}_pad_coupon_{coupon:02}_shim_slot_{slot}"),
                    6.0,
                    PAD_COUPON_Y + 20.0,
                    COUPON_PANEL_Z + 2.0,
                )
                .translate(x - 28.0 + slot as f64 * 28.0, y, COUPON_PANEL_Z / 2.0);
        }

        features = features
            + centered_cube(
                format!("{OUTPUT_PREFIX}_pad_coupon_{coupon:02}_left_retainer"),
                7.0,
                PAD_COUPON_Y + 12.0,
                8.0,
            )
            .translate(x - PAD_COUPON_X / 2.0 - 8.0, y, COUPON_PANEL_Z + 4.0)
            + centered_cube(
                format!("{OUTPUT_PREFIX}_pad_coupon_{coupon:02}_right_retainer"),
                7.0,
                PAD_COUPON_Y + 12.0,
                8.0,
            )
            .translate(x + PAD_COUPON_X / 2.0 + 8.0, y, COUPON_PANEL_Z + 4.0)
            + centered_cube(
                format!("{OUTPUT_PREFIX}_pad_coupon_{coupon:02}_barcode_land"),
                PAD_COUPON_X * 0.70,
                7.0,
                4.0,
            )
            .translate(x, y - PAD_COUPON_Y / 2.0 - 12.0, COUPON_PANEL_Z + 2.0);
    }

    base - cuts + features + coupon_grid_datum_pins()
}

fn coupon_grid_datum_pins() -> Part {
    let mut pins = Part::empty(format!("{OUTPUT_PREFIX}_coupon_grid_datum_pins"));
    for (index, (x, y)) in [
        (-COUPON_PANEL_X / 2.0 + 34.0, COUPON_PANEL_Y / 2.0 - 26.0),
        (COUPON_PANEL_X / 2.0 - 34.0, COUPON_PANEL_Y / 2.0 - 26.0),
    ]
    .into_iter()
    .enumerate()
    {
        pins = pins
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_coupon_grid_datum_pin_{index}"),
                5.0,
                12.0,
                28,
            )
            .translate(x, y, COUPON_PANEL_Z + 6.0);
    }
    pins
}

fn preload_force_height_witness_pockets() -> Part {
    let base = centered_cube(
        format!("{OUTPUT_PREFIX}_preload_force_height_witness_base"),
        PRELOAD_PANEL_X,
        PRELOAD_PANEL_Y,
        PRELOAD_PANEL_Z,
    )
    .translate(0.0, 0.0, PRELOAD_PANEL_Z / 2.0);
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_preload_force_height_pocket_cuts"));
    let mut features = Part::empty(format!("{OUTPUT_PREFIX}_preload_force_height_features"));

    for station in 0..PRELOAD_STATION_COUNT {
        let (x, y) = preload_station_xy(station);
        cuts = cuts
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_preload_station_{station:02}_force_puck_pocket"),
                FORCE_PUCK_D / 2.0,
                15.0,
                36,
            )
            .translate(x - 22.0, y, PRELOAD_PANEL_Z - 7.5)
            + centered_cube(
                format!("{OUTPUT_PREFIX}_preload_station_{station:02}_height_probe_slot"),
                HEIGHT_PROBE_SLOT_X,
                HEIGHT_PROBE_SLOT_Y,
                PRELOAD_PANEL_Z + 2.0,
            )
            .translate(x + 24.0, y, PRELOAD_PANEL_Z / 2.0);

        features = features
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_preload_station_{station:02}_spring_post"),
                6.0,
                22.0,
                28,
            )
            .translate(x - 22.0, y, PRELOAD_PANEL_Z + 11.0)
            + centered_cube(
                format!("{OUTPUT_PREFIX}_preload_station_{station:02}_height_witness_land"),
                58.0,
                8.0,
                5.0,
            )
            .translate(x + 4.0, y - 26.0, PRELOAD_PANEL_Z + 2.5);
    }

    base - cuts + features + compression_guard_step_bank()
}

fn compression_guard_step_bank() -> Part {
    let mut bank = Part::empty(format!("{OUTPUT_PREFIX}_compression_guard_step_bank"));
    for (index, compression) in [LOW_COMPRESSION, NOMINAL_COMPRESSION, HIGH_COMPRESSION]
        .into_iter()
        .enumerate()
    {
        let height = compressed_pad_height(compression);
        bank = bank
            + centered_cube(
                format!(
                    "{OUTPUT_PREFIX}_compression_step_{index}_{:.0}_percent",
                    compression * 100.0
                ),
                72.0,
                18.0,
                height * 4.0,
            )
            .translate(
                -PRELOAD_PANEL_X / 2.0 + 58.0,
                -44.0 + index as f64 * 42.0,
                PRELOAD_PANEL_Z + height * 2.0,
            );
    }
    bank
}

fn torque_preload_token_rails() -> Part {
    let base = centered_cube(
        format!("{OUTPUT_PREFIX}_torque_preload_token_rail_base"),
        TOKEN_PANEL_X,
        TOKEN_PANEL_Y,
        TOKEN_PANEL_Z,
    )
    .translate(0.0, 0.0, TOKEN_PANEL_Z / 2.0);
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_token_slot_cuts"));
    let mut rails = Part::empty(format!("{OUTPUT_PREFIX}_token_rail_features"));

    for rail in TokenRail::all() {
        let y = token_rail_y(rail);
        rails = rails
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{}_token_left_rail", rail.label()),
                TOKEN_PANEL_X - 44.0,
                6.0,
                13.0,
            )
            .translate(0.0, y - TOKEN_SLOT_Y / 2.0 - 8.0, TOKEN_PANEL_Z + 6.5)
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{}_token_right_rail", rail.label()),
                TOKEN_PANEL_X - 44.0,
                6.0,
                13.0,
            )
            .translate(0.0, y + TOKEN_SLOT_Y / 2.0 + 8.0, TOKEN_PANEL_Z + 6.5)
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{}_rail_scan_land", rail.label()),
                92.0,
                14.0,
                4.0,
            )
            .translate(-TOKEN_PANEL_X / 2.0 + 70.0, y, TOKEN_PANEL_Z + 2.0);

        for token in 0..TOKENS_PER_RAIL {
            let x = centered_index(token, TOKENS_PER_RAIL, 66.0) + 36.0;
            cuts = cuts
                + centered_cube(
                    format!(
                        "{OUTPUT_PREFIX}_{}_token_{token:02}_pocket_cut",
                        rail.label()
                    ),
                    TOKEN_SLOT_X,
                    TOKEN_SLOT_Y,
                    TOKEN_PANEL_Z + 2.0,
                )
                .translate(x, y, TOKEN_PANEL_Z / 2.0);
        }
    }

    base - cuts + rails
}

fn edge_center_thermal_dummy_loads() -> Part {
    let base = centered_cube(
        format!("{OUTPUT_PREFIX}_edge_center_dummy_load_base"),
        DUMMY_PANEL_X,
        DUMMY_PANEL_Y,
        DUMMY_PANEL_Z,
    )
    .translate(0.0, 0.0, DUMMY_PANEL_Z / 2.0);
    let mut features = Part::empty(format!("{OUTPUT_PREFIX}_edge_center_dummy_load_features"));

    for position in 0..CHIP_POSITION_COUNT {
        let (x, y) = dummy_position_xy(position);
        let zone = chip_zone(position);
        let diameter = match zone {
            ChipZone::Edge => EDGE_DUMMY_D,
            ChipZone::Center => CENTER_DUMMY_D,
        };
        features = features
            + centered_cylinder(
                format!(
                    "{OUTPUT_PREFIX}_position_{position:02}_{}_thermal_dummy_load",
                    zone.label()
                ),
                diameter / 2.0,
                DUMMY_LOAD_Z,
                40,
            )
            .translate(x, y, DUMMY_PANEL_Z + DUMMY_LOAD_Z / 2.0)
            + centered_cube(
                format!(
                    "{OUTPUT_PREFIX}_position_{position:02}_{}_load_id_land",
                    zone.label()
                ),
                38.0,
                7.0,
                4.0,
            )
            .translate(x, y - diameter / 2.0 - 12.0, DUMMY_PANEL_Z + 2.0);
    }

    base + features + edge_center_balance_bar()
}

fn edge_center_balance_bar() -> Part {
    let edge_bar = centered_cube(
        format!("{OUTPUT_PREFIX}_edge_dummy_load_balance_bar"),
        212.0,
        10.0,
        8.0,
    )
    .translate(-122.0, -DUMMY_PANEL_Y / 2.0 + 20.0, DUMMY_PANEL_Z + 4.0);
    let center_bar = centered_cube(
        format!("{OUTPUT_PREFIX}_center_dummy_load_balance_bar"),
        120.0,
        14.0,
        8.0,
    )
    .translate(156.0, -DUMMY_PANEL_Y / 2.0 + 20.0, DUMMY_PANEL_Z + 4.0);

    edge_bar + center_bar
}

fn temperature_probe_docks() -> Part {
    let base = centered_cube(
        format!("{OUTPUT_PREFIX}_temperature_probe_dock_base"),
        PROBE_PANEL_X,
        PROBE_PANEL_Y,
        PROBE_PANEL_Z,
    )
    .translate(0.0, 0.0, PROBE_PANEL_Z / 2.0);
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_temperature_probe_socket_cuts"));
    let mut features = Part::empty(format!("{OUTPUT_PREFIX}_temperature_probe_dock_features"));

    for dock in 0..PROBE_DOCK_COUNT {
        let (x, y) = probe_dock_xy(dock);
        cuts = cuts
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_temperature_probe_dock_{dock:02}_socket_cut"),
                PROBE_SOCKET_D / 2.0,
                PROBE_PANEL_Z + 2.0,
                24,
            )
            .translate(x, y, PROBE_PANEL_Z / 2.0)
            + centered_cube(
                format!("{OUTPUT_PREFIX}_temperature_probe_dock_{dock:02}_cable_groove"),
                68.0,
                7.0,
                7.0,
            )
            .translate(x + 34.0, y, PROBE_PANEL_Z - 3.5);

        features = features
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_temperature_probe_dock_{dock:02}_collar"),
                11.0,
                8.0,
                32,
            )
            .translate(x, y, PROBE_PANEL_Z + 4.0)
            + centered_cube(
                format!("{OUTPUT_PREFIX}_temperature_probe_dock_{dock:02}_id_land"),
                36.0,
                8.0,
                4.0,
            )
            .translate(x, y - 22.0, PROBE_PANEL_Z + 2.0);
    }

    base - cuts + features + probe_strain_relief_comb()
}

fn probe_strain_relief_comb() -> Part {
    let mut comb = Part::empty(format!("{OUTPUT_PREFIX}_probe_strain_relief_comb"));
    for index in 0..PROBE_COLS {
        comb = comb
            + centered_cube(
                format!("{OUTPUT_PREFIX}_probe_strain_relief_finger_{index}"),
                8.0,
                34.0,
                16.0,
            )
            .translate(
                centered_index(index, PROBE_COLS, PROBE_DOCK_PITCH_X),
                PROBE_PANEL_Y / 2.0 - 22.0,
                PROBE_PANEL_Z + 8.0,
            );
    }
    comb
}

fn contact_resistance_reference_blocks() -> Part {
    let base = centered_cube(
        format!("{OUTPUT_PREFIX}_contact_resistance_reference_base"),
        REFERENCE_PANEL_X,
        REFERENCE_PANEL_Y,
        REFERENCE_PANEL_Z,
    )
    .translate(0.0, 0.0, REFERENCE_PANEL_Z / 2.0);
    let mut blocks = Part::empty(format!(
        "{OUTPUT_PREFIX}_contact_resistance_reference_blocks"
    ));

    for reference in 0..REFERENCE_BLOCK_COUNT {
        let y = centered_index(reference, REFERENCE_BLOCK_COUNT, REFERENCE_BLOCK_PITCH_Y);
        let step_height = REFERENCE_BLOCK_Z + reference as f64 * 2.0;
        blocks = blocks
            + centered_cube(
                format!("{OUTPUT_PREFIX}_contact_reference_block_{reference:02}"),
                REFERENCE_BLOCK_X,
                REFERENCE_BLOCK_Y,
                step_height,
            )
            .translate(0.0, y, REFERENCE_PANEL_Z + step_height / 2.0)
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_contact_reference_block_{reference:02}_probe_well"),
                5.0,
                9.0,
                24,
            )
            .translate(
                REFERENCE_BLOCK_X / 2.0 - 18.0,
                y,
                REFERENCE_PANEL_Z + step_height + 4.5,
            );
    }

    base + blocks + reference_shim_retainer()
}

fn reference_shim_retainer() -> Part {
    let left = centered_cube(
        format!("{OUTPUT_PREFIX}_contact_reference_left_shim_retainer"),
        8.0,
        REFERENCE_PANEL_Y - 34.0,
        18.0,
    )
    .translate(
        -REFERENCE_PANEL_X / 2.0 + 18.0,
        0.0,
        REFERENCE_PANEL_Z + 9.0,
    );
    let right = centered_cube(
        format!("{OUTPUT_PREFIX}_contact_reference_right_shim_retainer"),
        8.0,
        REFERENCE_PANEL_Y - 34.0,
        18.0,
    )
    .translate(REFERENCE_PANEL_X / 2.0 - 18.0, 0.0, REFERENCE_PANEL_Z + 9.0);

    left + right
}

fn compression_witness_film_pockets() -> Part {
    let base = centered_cube(
        format!("{OUTPUT_PREFIX}_compression_witness_film_base"),
        FILM_PANEL_X,
        FILM_PANEL_Y,
        FILM_PANEL_Z,
    )
    .translate(0.0, 0.0, FILM_PANEL_Z / 2.0);
    let mut cuts = Part::empty(format!(
        "{OUTPUT_PREFIX}_compression_witness_film_pocket_cuts"
    ));
    let mut clips = Part::empty(format!("{OUTPUT_PREFIX}_compression_witness_film_clips"));

    for pocket in 0..FILM_POCKET_COUNT {
        let y = centered_index(pocket, FILM_POCKET_COUNT, FILM_POCKET_PITCH_Y);
        cuts = cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_compression_film_pocket_{pocket:02}_cut"),
                FILM_POCKET_X,
                FILM_POCKET_Y,
                7.0,
            )
            .translate(0.0, y, FILM_PANEL_Z - 3.5);
        clips = clips
            + centered_cube(
                format!("{OUTPUT_PREFIX}_compression_film_pocket_{pocket:02}_left_clip"),
                7.0,
                FILM_POCKET_Y + 10.0,
                9.0,
            )
            .translate(-FILM_POCKET_X / 2.0 - 8.0, y, FILM_PANEL_Z + 4.5)
            + centered_cube(
                format!("{OUTPUT_PREFIX}_compression_film_pocket_{pocket:02}_right_clip"),
                7.0,
                FILM_POCKET_Y + 10.0,
                9.0,
            )
            .translate(FILM_POCKET_X / 2.0 + 8.0, y, FILM_PANEL_Z + 4.5);
    }

    base - cuts + clips + film_lot_retain_land()
}

fn film_lot_retain_land() -> Part {
    centered_cube(
        format!("{OUTPUT_PREFIX}_compression_film_lot_retain_scan_land"),
        FILM_PANEL_X - 36.0,
        12.0,
        4.0,
    )
    .translate(0.0, -FILM_PANEL_Y / 2.0 + 18.0, FILM_PANEL_Z + 2.0)
}

fn release_hold_reject_lanes() -> Part {
    let base = centered_cube(
        format!("{OUTPUT_PREFIX}_release_hold_reject_lane_base"),
        LANE_PANEL_X,
        LANE_PANEL_Y,
        LANE_PANEL_Z,
    )
    .translate(0.0, 0.0, LANE_PANEL_Z / 2.0);
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_release_hold_reject_lane_cuts"));
    let mut features = Part::empty(format!("{OUTPUT_PREFIX}_release_hold_reject_lane_features"));

    for lane in DispositionLane::all() {
        let x = lane_x(lane);
        cuts = cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{}_lane_token_pocket", lane.label()),
                LANE_SLOT_X,
                LANE_SLOT_Y,
                LANE_PANEL_Z + 2.0,
            )
            .translate(x, 4.0, LANE_PANEL_Z / 2.0);
        features = features
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{}_lane_front_lip", lane.label()),
                LANE_SLOT_X + 24.0,
                8.0,
                12.0,
            )
            .translate(x, -LANE_SLOT_Y / 2.0 - 8.0, LANE_PANEL_Z + 6.0)
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{}_lane_backstop", lane.label()),
                LANE_SLOT_X + 24.0,
                8.0,
                20.0,
            )
            .translate(x, LANE_SLOT_Y / 2.0 + 8.0, LANE_PANEL_Z + 10.0)
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{}_lane_capacity_land", lane.label()),
                74.0,
                9.0,
                4.0,
            )
            .translate(x, -LANE_PANEL_Y / 2.0 + 14.0, LANE_PANEL_Z + 2.0);
    }

    base - cuts + features
}

fn evidence_bridge() -> Part {
    let mut parts = Part::empty(format!("{OUTPUT_PREFIX}_evidence_bridge"));
    for (index, x) in [
        -EVIDENCE_SPAN_X / 2.0 + 44.0,
        -EVIDENCE_SPAN_X / 6.0,
        EVIDENCE_SPAN_X / 6.0,
        EVIDENCE_SPAN_X / 2.0 - 44.0,
    ]
    .into_iter()
    .enumerate()
    {
        parts = parts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_evidence_bridge_post_{index}"),
                EVIDENCE_POST_W,
                EVIDENCE_POST_W,
                EVIDENCE_POST_Z,
            )
            .translate(x, 0.0, EVIDENCE_POST_Z / 2.0);
    }

    let beam = centered_cube(
        format!("{OUTPUT_PREFIX}_evidence_bridge_crossbeam"),
        EVIDENCE_SPAN_X,
        EVIDENCE_BRIDGE_Y,
        EVIDENCE_BEAM_Z,
    )
    .translate(0.0, 0.0, EVIDENCE_POST_Z + EVIDENCE_BEAM_Z / 2.0);
    let camera_bar = evidence_camera_carriages();
    let lights = evidence_light_bars();

    parts + beam + camera_bar + lights
}

fn evidence_camera_carriages() -> Part {
    let mut cameras = Part::empty(format!("{OUTPUT_PREFIX}_evidence_camera_carriages"));
    for index in 0..CAMERA_COUNT {
        let x = centered_index(index, CAMERA_COUNT, EVIDENCE_SPAN_X / 4.8);
        cameras = cameras
            + centered_cube(
                format!("{OUTPUT_PREFIX}_evidence_camera_carriage_{index}"),
                108.0,
                54.0,
                18.0,
            )
            .translate(x, -7.0, EVIDENCE_POST_Z - 18.0)
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_evidence_camera_lens_clearance_{index}"),
                14.0,
                20.0,
                36,
            )
            .translate(x, -7.0, EVIDENCE_POST_Z - 34.0);
    }
    cameras
}

fn evidence_light_bars() -> Part {
    let mut lights = Part::empty(format!("{OUTPUT_PREFIX}_evidence_light_bars"));
    for index in 0..LIGHT_BAR_COUNT {
        let y = if index == 0 {
            -EVIDENCE_BRIDGE_Y / 2.0 - 12.0
        } else {
            EVIDENCE_BRIDGE_Y / 2.0 + 12.0
        };
        lights = lights
            + centered_cube(
                format!("{OUTPUT_PREFIX}_evidence_light_bar_{index}"),
                EVIDENCE_SPAN_X - 180.0,
                10.0,
                12.0,
            )
            .translate(0.0, y, EVIDENCE_POST_Z - 46.0);
    }
    lights
}

fn robot_service_keepouts() -> Part {
    let front = centered_cube(
        format!("{OUTPUT_PREFIX}_front_robot_sweep_keepout_gauge"),
        DECK_X - 180.0,
        ROBOT_FRONT_CLEARANCE_Y,
        8.0,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + ROBOT_FRONT_CLEARANCE_Y / 2.0 + 16.0,
        DECK_Z + 4.0,
    );
    let rear = centered_cube(
        format!("{OUTPUT_PREFIX}_rear_service_keepout_gauge"),
        DECK_X - 180.0,
        SERVICE_REAR_CLEARANCE_Y,
        8.0,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - SERVICE_REAR_CLEARANCE_Y / 2.0 - 16.0,
        DECK_Z + 4.0,
    );
    let left = centered_cube(
        format!("{OUTPUT_PREFIX}_left_robot_gripper_keepout_gauge"),
        SIDE_SERVICE_CLEARANCE_X,
        DECK_Y - 250.0,
        8.0,
    )
    .translate(
        -DECK_X / 2.0 + SIDE_SERVICE_CLEARANCE_X / 2.0 + 18.0,
        -20.0,
        DECK_Z + 4.0,
    );
    let right = centered_cube(
        format!("{OUTPUT_PREFIX}_right_thermal_service_keepout_gauge"),
        SIDE_SERVICE_CLEARANCE_X,
        DECK_Y - 250.0,
        8.0,
    )
    .translate(
        DECK_X / 2.0 - SIDE_SERVICE_CLEARANCE_X / 2.0 - 18.0,
        -20.0,
        DECK_Z + 4.0,
    );

    front + rear + left + right + keepout_height_gauges()
}

fn keepout_height_gauges() -> Part {
    let mut gauges = Part::empty(format!("{OUTPUT_PREFIX}_keepout_height_gauges"));
    for (index, (x, y, height)) in [
        (
            NEST_POS.0 + NEST_X / 2.0 + 38.0,
            NEST_POS.1,
            ROBOT_PICK_CLEARANCE_Z,
        ),
        (
            PRELOAD_POS.0 + PRELOAD_PANEL_X / 2.0 + 34.0,
            PRELOAD_POS.1,
            SERVICE_CLEARANCE_Z,
        ),
        (
            PROBE_POS.0 + PROBE_PANEL_X / 2.0 + 34.0,
            PROBE_POS.1,
            SERVICE_CLEARANCE_Z,
        ),
        (
            LANE_POS.0 - LANE_PANEL_X / 2.0 - 34.0,
            LANE_POS.1,
            ROBOT_PICK_CLEARANCE_Z,
        ),
    ]
    .into_iter()
    .enumerate()
    {
        gauges = gauges
            + centered_cube(
                format!("{OUTPUT_PREFIX}_keepout_height_gauge_{index}"),
                18.0,
                18.0,
                height,
            )
            .translate(x, y, DECK_Z + height / 2.0);
    }
    gauges
}

fn module_footprints() -> [Rect; 9] {
    [
        Rect {
            name: "cassette_datum_nest",
            center: NEST_POS,
            x: NEST_X,
            y: NEST_Y,
        },
        Rect {
            name: "thermal_pad_coupon_grid",
            center: COUPON_POS,
            x: COUPON_PANEL_X,
            y: COUPON_PANEL_Y,
        },
        Rect {
            name: "preload_force_height_witness_pockets",
            center: PRELOAD_POS,
            x: PRELOAD_PANEL_X,
            y: PRELOAD_PANEL_Y,
        },
        Rect {
            name: "torque_preload_token_rails",
            center: TOKEN_POS,
            x: TOKEN_PANEL_X,
            y: TOKEN_PANEL_Y,
        },
        Rect {
            name: "edge_center_thermal_dummy_loads",
            center: DUMMY_POS,
            x: DUMMY_PANEL_X,
            y: DUMMY_PANEL_Y,
        },
        Rect {
            name: "temperature_probe_docks",
            center: PROBE_POS,
            x: PROBE_PANEL_X,
            y: PROBE_PANEL_Y,
        },
        Rect {
            name: "contact_resistance_reference_blocks",
            center: REFERENCE_POS,
            x: REFERENCE_PANEL_X,
            y: REFERENCE_PANEL_Y,
        },
        Rect {
            name: "compression_witness_film_pockets",
            center: FILM_POS,
            x: FILM_PANEL_X,
            y: FILM_PANEL_Y,
        },
        Rect {
            name: "release_hold_reject_lanes",
            center: LANE_POS,
            x: LANE_PANEL_X,
            y: LANE_PANEL_Y,
        },
    ]
}

fn critical_footprints_overlap() -> bool {
    let footprints = module_footprints();
    for left in 0..footprints.len() {
        for right in (left + 1)..footprints.len() {
            if footprints[left].overlaps(footprints[right]) {
                return true;
            }
        }
    }
    false
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn chip_position_xy(position: usize) -> (f64, f64) {
    let col = position % CHIP_COLS;
    let row = position / CHIP_COLS;
    (
        centered_index(col, CHIP_COLS, CHIP_PITCH_X),
        centered_index(row, CHIP_ROWS, CHIP_PITCH_Y),
    )
}

fn chip_zone(position: usize) -> ChipZone {
    let col = position % CHIP_COLS;
    let row = position / CHIP_COLS;
    if col == 0 || col == CHIP_COLS - 1 || row == 0 || row == CHIP_ROWS - 1 {
        ChipZone::Edge
    } else {
        ChipZone::Center
    }
}

fn edge_position_count() -> usize {
    (0..CHIP_POSITION_COUNT)
        .filter(|position| chip_zone(*position) == ChipZone::Edge)
        .count()
}

fn center_position_count() -> usize {
    CHIP_POSITION_COUNT - edge_position_count()
}

fn datum_pin_points() -> [(f64, f64); DATUM_PIN_COUNT] {
    [
        (-CARRIER_X / 2.0 - 26.0, -CARRIER_Y / 2.0 - 22.0),
        (CARRIER_X / 2.0 + 26.0, -CARRIER_Y / 2.0 - 22.0),
        (-CARRIER_X / 2.0 - 26.0, CARRIER_Y / 2.0 + 22.0),
        (CARRIER_X / 2.0 + 26.0, CARRIER_Y / 2.0 + 22.0),
    ]
}

fn carrier_screw_points() -> [(f64, f64); PRELOAD_STATION_COUNT] {
    let x = CARRIER_X / 2.0 + 34.0;
    let y = CARRIER_Y / 2.0 + 32.0;
    [
        (-x, -y),
        (0.0, -y),
        (x, -y),
        (x, 0.0),
        (x, y),
        (0.0, y),
        (-x, y),
        (-x, 0.0),
    ]
}

fn coupon_xy(index: usize) -> (f64, f64) {
    (
        centered_index(index % COUPON_COLS, COUPON_COLS, COUPON_PITCH_X),
        centered_index(index / COUPON_COLS, COUPON_ROWS, COUPON_PITCH_Y),
    )
}

fn preload_station_xy(index: usize) -> (f64, f64) {
    (
        centered_index(
            index % PRELOAD_STATION_COLS,
            PRELOAD_STATION_COLS,
            PRELOAD_PITCH_X,
        ),
        centered_index(
            index / PRELOAD_STATION_COLS,
            PRELOAD_STATION_ROWS,
            PRELOAD_PITCH_Y,
        ),
    )
}

fn dummy_position_xy(position: usize) -> (f64, f64) {
    let col = position % CHIP_COLS;
    let row = position / CHIP_COLS;
    (
        centered_index(col, CHIP_COLS, DUMMY_POSITION_PITCH_X),
        centered_index(row, CHIP_ROWS, DUMMY_POSITION_PITCH_Y),
    )
}

fn probe_dock_xy(index: usize) -> (f64, f64) {
    (
        centered_index(index % PROBE_COLS, PROBE_COLS, PROBE_DOCK_PITCH_X),
        centered_index(index / PROBE_COLS, PROBE_ROWS, PROBE_DOCK_PITCH_Y),
    )
}

fn token_rail_y(rail: TokenRail) -> f64 {
    centered_index(rail.index(), TOKEN_RAIL_COUNT, TOKEN_RAIL_PITCH_Y)
}

fn lane_x(lane: DispositionLane) -> f64 {
    centered_index(lane.index(), LANE_COUNT, LANE_PITCH_X)
}

fn compressed_pad_height(compression: f64) -> f64 {
    NOMINAL_PAD_THICKNESS * (1.0 - compression)
}

fn disposition_capacity() -> usize {
    DispositionLane::all()
        .into_iter()
        .map(DispositionLane::capacity)
        .sum()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_scoped_and_stable() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 13);
        for output in OUTPUTS {
            assert!(output.starts_with(&format!("output/{OUTPUT_PREFIX}_")));
            assert!(output.ends_with(".stl"));
        }
        assert_eq!(
            OUTPUTS[OUTPUTS.len() - 1],
            format!("output/{OUTPUT_PREFIX}_assembly.stl")
        );
    }

    #[test]
    fn required_features_cover_requested_station_elements() {
        for feature in [
            "cassette_datum_nest",
            "thermal_pad_coupon_grid",
            "preload_force_height_witness_pockets",
            "torque_preload_token_rails",
            "edge_center_thermal_dummy_loads",
            "temperature_probe_docks",
            "contact_resistance_reference_blocks",
            "compression_witness_film_pockets",
            "release_hold_reject_lanes",
            "evidence_bridge",
            "robot_service_keepouts",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn chip_cassette_edge_center_split_is_deterministic() {
        assert_eq!(CHIP_POSITION_COUNT, 16);
        assert_eq!(edge_position_count(), 12);
        assert_eq!(center_position_count(), 4);
        assert_eq!(chip_zone(0), ChipZone::Edge);
        assert_eq!(chip_zone(5), ChipZone::Center);
        assert_eq!(chip_zone(10), ChipZone::Center);
        assert_eq!(chip_zone(15), ChipZone::Edge);
    }

    #[test]
    fn thermal_coupon_preload_and_probe_counts_are_explicit() {
        assert_eq!(COUPON_COUNT, 12);
        assert_eq!(GAP_SHIM_SLOTS_PER_COUPON, 3);
        assert_eq!(PRELOAD_STATION_COUNT, 8);
        assert_eq!(PROBE_DOCK_COUNT, 10);
        assert_eq!(REFERENCE_BLOCK_COUNT, 5);
        assert_eq!(FILM_POCKET_COUNT, 6);
    }

    #[test]
    fn compression_witness_steps_encode_guard_band() {
        assert!(LOW_COMPRESSION < NOMINAL_COMPRESSION);
        assert!(NOMINAL_COMPRESSION < HIGH_COMPRESSION);
        assert!(
            compressed_pad_height(LOW_COMPRESSION) > compressed_pad_height(NOMINAL_COMPRESSION)
        );
        assert!(
            compressed_pad_height(NOMINAL_COMPRESSION) > compressed_pad_height(HIGH_COMPRESSION)
        );
        assert!((compressed_pad_height(NOMINAL_COMPRESSION) - 1.64).abs() < 0.001);
    }

    #[test]
    fn token_and_disposition_lanes_cover_preload_workflow() {
        assert_eq!(TokenRail::all().len(), 3);
        assert_eq!(TOKENS_PER_RAIL, PRELOAD_STATION_COUNT);
        assert_eq!(DispositionLane::all().len(), 3);
        assert_eq!(disposition_capacity(), CHIP_POSITION_COUNT);
    }

    #[test]
    fn layout_fits_deck_without_major_module_overlap() {
        assert_design_constraints();
        assert!(module_footprints()
            .iter()
            .all(|rect| rect.fits_inside_deck()));
        assert!(!critical_footprints_overlap());
    }
}
