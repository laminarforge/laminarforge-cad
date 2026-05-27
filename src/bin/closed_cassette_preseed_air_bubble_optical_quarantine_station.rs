use std::collections::BTreeSet;
use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed pre-seed cassette air-bubble optical quarantine station.
//
// Source-only validation CAD for a sealed cassette station that quarantines
// pre-seed fluid paths when inlet/outlet prime loops, transmitted-light bubble
// windows, low-point purge pockets, pressure/flow taps, dye/air slug challenge
// tokens, waste/retain routing, custody lands, and release/hold/reject gates
// disagree. This models fixture architecture only; purchased optics, sterile
// wetted components, and process acceptance thresholds remain external.

const OUTPUT_PREFIX: &str = "output/closed_cassette_preseed_air_bubble_optical_quarantine_station";
const OUTPUTS: [&str; 12] = [
    "output/closed_cassette_preseed_air_bubble_optical_quarantine_station_sealed_cassette_datum_nest.stl",
    "output/closed_cassette_preseed_air_bubble_optical_quarantine_station_inlet_outlet_prime_loop_witness.stl",
    "output/closed_cassette_preseed_air_bubble_optical_quarantine_station_transmitted_light_bubble_window_array.stl",
    "output/closed_cassette_preseed_air_bubble_optical_quarantine_station_low_point_purge_pocket_ladder.stl",
    "output/closed_cassette_preseed_air_bubble_optical_quarantine_station_pressure_flow_tap_bosses.stl",
    "output/closed_cassette_preseed_air_bubble_optical_quarantine_station_dye_air_slug_challenge_token_rail.stl",
    "output/closed_cassette_preseed_air_bubble_optical_quarantine_station_waste_retain_split_manifold.stl",
    "output/closed_cassette_preseed_air_bubble_optical_quarantine_station_barcode_custody_lands.stl",
    "output/closed_cassette_preseed_air_bubble_optical_quarantine_station_release_hold_reject_gate_bank.stl",
    "output/closed_cassette_preseed_air_bubble_optical_quarantine_station_camera_evidence_bridge.stl",
    "output/closed_cassette_preseed_air_bubble_optical_quarantine_station_robotic_service_datums.stl",
    "output/closed_cassette_preseed_air_bubble_optical_quarantine_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "sealed_cassette_datum_nest",
    "inlet_outlet_prime_loop_witness",
    "transmitted_light_bubble_window_array",
    "low_point_purge_pocket_ladder",
    "pressure_flow_tap_bosses",
    "dye_air_slug_challenge_token_rail",
    "waste_retain_split_manifold",
    "barcode_custody_lands",
    "release_hold_reject_gate_bank",
    "camera_evidence_bridge",
    "robotic_service_datums",
];

const LIMITATIONS: [&str; 5] = [
    "mechanical_quarantine_fixture_only",
    "no_cell_processing_protocol",
    "no_release_acceptance_thresholds",
    "not_pressure_rated_hardware",
    "purchased_wetted_components_external",
];

const STATION_X: f64 = 1460.0;
const STATION_Y: f64 = 920.0;
const BASE_Z: f64 = 24.0;
const CURB_W: f64 = 20.0;
const CURB_Z: f64 = 44.0;
const SUMP_X: f64 = 1290.0;
const SUMP_Y: f64 = 755.0;
const SUMP_DEPTH: f64 = 7.0;
const DRAIN_D: f64 = 18.0;
const MOUNT_HOLE_D: f64 = 6.8;
const DECK_DATUM_TARGETS: usize = 6;

const NEST_POS: (f64, f64) = (-325.0, 130.0);
const CASSETTE_ROWS: usize = 2;
const CASSETTE_COLS: usize = 4;
const CASSETTE_SLOTS: usize = CASSETTE_ROWS * CASSETTE_COLS;
const CASSETTE_SLOT_X: f64 = REVC_CHIP_LENGTH + 18.0;
const CASSETTE_SLOT_Y: f64 = REVC_CHIP_WIDTH + 16.0;
const CASSETTE_SLOT_DEPTH: f64 = REVC_TOTAL_HEIGHT + 11.0;
const CASSETTE_SLOT_PITCH_X: f64 = CASSETTE_SLOT_X + 20.0;
const CASSETTE_SLOT_PITCH_Y: f64 = CASSETTE_SLOT_Y + 22.0;
const CASSETTE_MARGIN_X: f64 = 34.0;
const CASSETTE_MARGIN_Y: f64 = 32.0;
const NEST_X: f64 = (CASSETTE_COLS as f64 - 1.0) * CASSETTE_SLOT_PITCH_X
    + CASSETTE_SLOT_X
    + 2.0 * CASSETTE_MARGIN_X;
const NEST_Y: f64 = (CASSETTE_ROWS as f64 - 1.0) * CASSETTE_SLOT_PITCH_Y
    + CASSETTE_SLOT_Y
    + 2.0 * CASSETTE_MARGIN_Y;
const NEST_Z: f64 = 46.0;
const NEST_SEAL_LIP_W: f64 = 10.0;
const NEST_DATUM_PINS: usize = 6;
const CASSETTE_HOLD_DOWNS: usize = 8;

const PRIME_POS: (f64, f64) = (112.0, 342.0);
const PRIME_PANEL_X: f64 = 610.0;
const PRIME_PANEL_Y: f64 = 126.0;
const PRIME_PANEL_Z: f64 = 30.0;
const PRIME_LOOPS: usize = 2;
const PRIME_WINDOWS_PER_LOOP: usize = 4;
const PRIME_WITNESS_WINDOWS: usize = PRIME_LOOPS * PRIME_WINDOWS_PER_LOOP;
const PRIME_LOOP_PITCH_Y: f64 = 44.0;
const PRIME_WINDOW_X: f64 = 46.0;
const PRIME_WINDOW_Y: f64 = 16.0;
const PRIME_BORE_D: f64 = 8.0;

const BUBBLE_POS: (f64, f64) = (360.0, 82.0);
const BUBBLE_PANEL_X: f64 = 610.0;
const BUBBLE_PANEL_Y: f64 = 212.0;
const BUBBLE_PANEL_Z: f64 = 28.0;
const BUBBLE_UPRIGHT_Z: f64 = 118.0;
const BUBBLE_LANES: usize = CASSETTE_SLOTS;
const BUBBLE_WINDOWS: usize = BUBBLE_LANES;
const BUBBLE_WINDOW_X: f64 = 44.0;
const BUBBLE_WINDOW_Y: f64 = 20.0;
const BUBBLE_WINDOW_Z: f64 = 38.0;
const BUBBLE_LANE_PITCH_X: f64 = 68.0;
const TRANSMITTED_LIGHT_PAIRS: usize = BUBBLE_WINDOWS;
const BUBBLE_CALIBRATION_WELLS: usize = 4;

const PURGE_POS: (f64, f64) = (528.0, -150.0);
const PURGE_LADDER_X: f64 = 264.0;
const PURGE_LADDER_Y: f64 = 230.0;
const PURGE_LADDER_Z: f64 = 42.0;
const PURGE_POCKETS: usize = 6;
const PURGE_POCKET_D: f64 = 26.0;
const PURGE_POCKET_DEPTH: f64 = 18.0;
const PURGE_STEP_DROP_MM: f64 = 4.0;
const PURGE_WITNESS_RIBS: usize = PURGE_POCKETS + 1;

const TAP_POS: (f64, f64) = (-98.0, -126.0);
const TAP_PANEL_X: f64 = 430.0;
const TAP_PANEL_Y: f64 = 210.0;
const TAP_PANEL_Z: f64 = 42.0;
const PRESSURE_TAPS: usize = BUBBLE_LANES;
const FLOW_TAPS: usize = BUBBLE_LANES;
const TAP_BOSSES: usize = PRESSURE_TAPS + FLOW_TAPS;
const TAP_BOSS_D: f64 = 26.0;
const TAP_BORE_D: f64 = 7.0;
const TAP_LANE_PITCH_X: f64 = 48.0;

const TOKEN_POS: (f64, f64) = (-480.0, -190.0);
const TOKEN_RAIL_X: f64 = 382.0;
const TOKEN_RAIL_Y: f64 = 118.0;
const TOKEN_RAIL_Z: f64 = 30.0;
const DYE_TOKEN_SLOTS: usize = 6;
const AIR_TOKEN_SLOTS: usize = 4;
const CHALLENGE_TOKEN_SLOTS: usize = DYE_TOKEN_SLOTS + AIR_TOKEN_SLOTS;
const TOKEN_SLOT_X: f64 = 28.0;
const TOKEN_SLOT_Y: f64 = 48.0;
const CHALLENGE_INDEX_TICKS: usize = CHALLENGE_TOKEN_SLOTS + 1;

const SPLIT_POS: (f64, f64) = (115.0, -306.0);
const SPLIT_PANEL_X: f64 = 482.0;
const SPLIT_PANEL_Y: f64 = 132.0;
const SPLIT_PANEL_Z: f64 = 36.0;
const SPLIT_BRANCHES: usize = 2;
const SPLIT_LANE_NAMES: [&str; SPLIT_BRANCHES] = ["retain", "waste"];
const SPLIT_CAPTURE_WELLS_PER_BRANCH: usize = 4;
const SPLIT_CAPTURE_WELLS: usize = SPLIT_BRANCHES * SPLIT_CAPTURE_WELLS_PER_BRANCH;
const SPLIT_WELL_D: f64 = 22.0;
const SPLIT_VALVE_BOSSES: usize = 3;

const CUSTODY_POS: (f64, f64) = (-454.0, -338.0);
const CUSTODY_PLATE_X: f64 = 360.0;
const CUSTODY_PLATE_Y: f64 = 124.0;
const CUSTODY_PLATE_Z: f64 = 14.0;
const BARCODE_LANDS: usize = CASSETTE_SLOTS;
const CUSTODY_CARD_LANDS: usize = 4;
const CUSTODY_SEAL_POSTS: usize = 4;

const GATE_POS: (f64, f64) = (518.0, -342.0);
const GATE_BANK_X: f64 = 300.0;
const GATE_BANK_Y: f64 = 126.0;
const GATE_BANK_Z: f64 = 32.0;
const STATUS_LANES: usize = 3;
const STATUS_LANE_NAMES: [&str; STATUS_LANES] = ["release", "hold", "reject"];
const GATE_SLOTS_PER_LANE: usize = 4;
const GATE_SLOTS: usize = STATUS_LANES * GATE_SLOTS_PER_LANE;
const GATE_SLOT_X: f64 = 46.0;
const GATE_SLOT_Y: f64 = 24.0;

const BRIDGE_POS: (f64, f64) = (18.0, 366.0);
const BRIDGE_SPAN_X: f64 = 1120.0;
const BRIDGE_POST_X: f64 = 30.0;
const BRIDGE_POST_Y: f64 = 46.0;
const BRIDGE_POST_Z: f64 = 178.0;
const BRIDGE_BEAM_Y: f64 = 52.0;
const BRIDGE_BEAM_Z: f64 = 24.0;
const EVIDENCE_CAMERA_PODS: usize = 3;
const EVIDENCE_LED_SEGMENTS: usize = 8;
const EVIDENCE_CARD_LANDS: usize = 4;
const CAMERA_CLEARANCE_Z: f64 = 142.0;

const ROBOT_DATUM_PINS: usize = 4;
const ROBOT_SERVICE_FIDUCIALS: usize = 8;
const ROBOT_TOOL_DOCK_DATUMS: usize = 3;
const ROBOT_SWEEP_X: f64 = 1190.0;
const ROBOT_SWEEP_Y: f64 = 738.0;
const ROBOT_SWEEP_Z: f64 = 160.0;
const KEEP_OUT_RAIL: f64 = 7.0;
const FRONT_ROBOT_SERVICE: f64 = 380.0;
const REAR_OPTICAL_SERVICE: f64 = 250.0;
const LEFT_CASSETTE_SERVICE: f64 = 220.0;
const RIGHT_PURGE_SERVICE: f64 = 205.0;

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

    let nest = sealed_cassette_datum_nest();
    export(&nest, OUTPUTS[0]);

    let prime = inlet_outlet_prime_loop_witness();
    export(&prime, OUTPUTS[1]);

    let bubbles = transmitted_light_bubble_window_array();
    export(&bubbles, OUTPUTS[2]);

    let purge = low_point_purge_pocket_ladder();
    export(&purge, OUTPUTS[3]);

    let taps = pressure_flow_tap_bosses();
    export(&taps, OUTPUTS[4]);

    let tokens = dye_air_slug_challenge_token_rail();
    export(&tokens, OUTPUTS[5]);

    let split = waste_retain_split_manifold();
    export(&split, OUTPUTS[6]);

    let custody = barcode_custody_lands();
    export(&custody, OUTPUTS[7]);

    let gates = release_hold_reject_gate_bank();
    export(&gates, OUTPUTS[8]);

    let bridge = camera_evidence_bridge();
    export(&bridge, OUTPUTS[9]);

    let robot = robotic_service_datums();
    export(&robot, OUTPUTS[10]);

    let assembly = station_assembly();
    export(&assembly, OUTPUTS[11]);

    println!();
    println!("Closed cassette pre-seed air-bubble optical quarantine station:");
    println!(
        "  Sealed cassette nest:        {CASSETTE_ROWS} x {CASSETTE_COLS} cassette slots ({CASSETTE_SLOTS} total), {NEST_DATUM_PINS} datum pins, {CASSETTE_HOLD_DOWNS} hold-downs"
    );
    println!(
        "  Prime loop witness:          {PRIME_LOOPS} inlet/outlet loops, {PRIME_WITNESS_WINDOWS} loop witness windows, {PRIME_BORE_D:.1}mm nominal bore"
    );
    println!(
        "  Bubble optics:               {BUBBLE_WINDOWS} transmitted-light windows with {TRANSMITTED_LIGHT_PAIRS} camera/illuminator pairs and {BUBBLE_CALIBRATION_WELLS} calibration wells"
    );
    println!(
        "  Low-point purge ladder:      {PURGE_POCKETS} purge pockets with {PURGE_STEP_DROP_MM:.1}mm indexed elevation drop per pocket"
    );
    println!(
        "  Pressure/flow taps:          {PRESSURE_TAPS} pressure taps + {FLOW_TAPS} flow taps ({TAP_BOSSES} bosses)"
    );
    println!(
        "  Challenge/custody handling:  {DYE_TOKEN_SLOTS} dye slug tokens, {AIR_TOKEN_SLOTS} air slug tokens, {BARCODE_LANDS} barcode lands"
    );
    println!(
        "  Disposition routing:         retain/waste split with {SPLIT_CAPTURE_WELLS} capture wells and {STATUS_LANES} release/hold/reject gate lanes"
    );
    println!(
        "  Evidence/robotics:           {EVIDENCE_CAMERA_PODS} camera pods, {EVIDENCE_LED_SEGMENTS} LED segments, {ROBOT_DATUM_PINS} robot datum pins, {ROBOT_SERVICE_FIDUCIALS} service fiducials"
    );
    println!("  Labeled STL outputs:         {} files", OUTPUTS.len());
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    sealed_cassette_datum_nest()
        + inlet_outlet_prime_loop_witness().translate(PRIME_POS.0, PRIME_POS.1, BASE_Z)
        + transmitted_light_bubble_window_array().translate(BUBBLE_POS.0, BUBBLE_POS.1, BASE_Z)
        + low_point_purge_pocket_ladder().translate(PURGE_POS.0, PURGE_POS.1, BASE_Z)
        + pressure_flow_tap_bosses().translate(TAP_POS.0, TAP_POS.1, BASE_Z)
        + dye_air_slug_challenge_token_rail().translate(TOKEN_POS.0, TOKEN_POS.1, BASE_Z)
        + waste_retain_split_manifold().translate(SPLIT_POS.0, SPLIT_POS.1, BASE_Z)
        + barcode_custody_lands().translate(CUSTODY_POS.0, CUSTODY_POS.1, BASE_Z)
        + release_hold_reject_gate_bank().translate(GATE_POS.0, GATE_POS.1, BASE_Z)
        + camera_evidence_bridge()
        + robotic_service_datums()
}

fn sealed_cassette_datum_nest() -> Part {
    containment_deck() + cassette_datum_nest().translate(NEST_POS.0, NEST_POS.1, BASE_Z)
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        "preseed_air_bubble_quarantine_containment_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let sump = centered_cube(
        "preseed_air_bubble_quarantine_sump_recess",
        SUMP_X,
        SUMP_Y,
        SUMP_DEPTH + 1.0,
    )
    .translate(0.0, -10.0, BASE_Z - SUMP_DEPTH / 2.0 + 0.5);
    let drain = centered_cylinder(
        "preseed_air_bubble_quarantine_front_right_drain_cut",
        DRAIN_D / 2.0,
        CURB_W + 42.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 92.0,
        -STATION_Y / 2.0 + 12.0,
        BASE_Z - 5.0,
    );

    deck - sump - drain - deck_mount_holes() - module_socket_recesses()
        + containment_curbs()
        + deck_datum_targets()
        + deck_flow_witness_ribs()
        + deck_zone_markers()
}

fn containment_curbs() -> Part {
    let z = BASE_Z + CURB_Z / 2.0;
    let front = centered_cube(
        "preseed_air_bubble_quarantine_front_curb",
        STATION_X,
        CURB_W,
        CURB_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + CURB_W / 2.0, z);
    let rear = centered_cube(
        "preseed_air_bubble_quarantine_rear_curb",
        STATION_X,
        CURB_W,
        CURB_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - CURB_W / 2.0, z);
    let left = centered_cube(
        "preseed_air_bubble_quarantine_left_curb",
        CURB_W,
        STATION_Y,
        CURB_Z,
    )
    .translate(-STATION_X / 2.0 + CURB_W / 2.0, 0.0, z);
    let right = centered_cube(
        "preseed_air_bubble_quarantine_right_curb",
        CURB_W,
        STATION_Y,
        CURB_Z,
    )
    .translate(STATION_X / 2.0 - CURB_W / 2.0, 0.0, z);
    front + rear + left + right
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("preseed_air_bubble_quarantine_mount_holes");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 78.0, -STATION_Y / 2.0 + 72.0),
        (STATION_X / 2.0 - 78.0, -STATION_Y / 2.0 + 72.0),
        (-STATION_X / 2.0 + 78.0, STATION_Y / 2.0 - 72.0),
        (STATION_X / 2.0 - 78.0, STATION_Y / 2.0 - 72.0),
        (0.0, -STATION_Y / 2.0 + 72.0),
        (0.0, STATION_Y / 2.0 - 72.0),
    ]
    .iter()
    .copied()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("preseed_air_bubble_quarantine_m6_mount_hole_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 6.0,
                28,
            )
            .translate(x, y, BASE_Z / 2.0);
    }
    holes
}

fn module_socket_recesses() -> Part {
    let mut recesses = Part::empty("preseed_air_bubble_quarantine_module_socket_recesses");
    for footprint in module_footprints() {
        recesses = recesses
            + centered_cube(
                format!(
                    "preseed_air_bubble_quarantine_{}_socket_recess",
                    footprint.name
                ),
                footprint.x + 20.0,
                footprint.y + 18.0,
                SUMP_DEPTH + 1.0,
            )
            .translate(
                footprint.center.0,
                footprint.center.1,
                BASE_Z - SUMP_DEPTH / 2.0 + 0.5,
            );
    }
    recesses
}

fn deck_datum_targets() -> Part {
    let mut targets = Part::empty("preseed_air_bubble_quarantine_deck_datum_targets");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 118.0, -STATION_Y / 2.0 + 112.0),
        (STATION_X / 2.0 - 118.0, -STATION_Y / 2.0 + 112.0),
        (-STATION_X / 2.0 + 118.0, STATION_Y / 2.0 - 112.0),
        (STATION_X / 2.0 - 118.0, STATION_Y / 2.0 - 112.0),
        (-128.0, STATION_Y / 2.0 - 112.0),
        (128.0, STATION_Y / 2.0 - 112.0),
    ]
    .iter()
    .copied()
    .enumerate()
    {
        let ring = centered_cylinder(
            format!("preseed_air_bubble_quarantine_deck_datum_ring_{i}"),
            17.0,
            5.0,
            36,
        )
        .translate(x, y, BASE_Z + 2.5);
        let bore = centered_cylinder(
            format!("preseed_air_bubble_quarantine_deck_datum_center_bore_{i}"),
            4.0,
            6.0,
            24,
        )
        .translate(x, y, BASE_Z + 3.0);
        targets = targets + (ring - bore);
    }
    targets
}

fn deck_flow_witness_ribs() -> Part {
    let mut ribs = Part::empty("preseed_air_bubble_quarantine_deck_flow_witness_ribs");
    for rib in 0..9 {
        ribs = ribs
            + centered_cube(
                format!("preseed_air_bubble_quarantine_sump_slope_witness_rib_{rib}"),
                SUMP_X - 142.0,
                5.0,
                4.0,
            )
            .translate(0.0, centered_index(rib, 9, 68.0) - 8.0, BASE_Z + 2.0);
    }
    ribs
}

fn deck_zone_markers() -> Part {
    let mut markers = Part::empty("preseed_air_bubble_quarantine_deck_zone_markers");
    for footprint in module_footprints() {
        markers = markers
            + centered_cube(
                format!(
                    "preseed_air_bubble_quarantine_{}_zone_marker",
                    footprint.name
                ),
                footprint.x + 32.0,
                footprint.y + 26.0,
                3.0,
            )
            .translate(footprint.center.0, footprint.center.1, BASE_Z + 1.5);
    }
    markers
}

fn cassette_datum_nest() -> Part {
    let body = centered_cube(
        "preseed_air_bubble_quarantine_cassette_nest_body",
        NEST_X,
        NEST_Y,
        NEST_Z,
    )
    .translate(0.0, 0.0, NEST_Z / 2.0);

    body - cassette_slot_cuts()
        + cassette_seal_lip()
        + cassette_datum_pins()
        + cassette_hold_downs()
        + cassette_prime_port_lands()
        + cassette_wetness_gutters()
}

fn cassette_slot_cuts() -> Part {
    let mut cuts = Part::empty("preseed_air_bubble_quarantine_cassette_slot_cuts");
    for row in 0..CASSETTE_ROWS {
        for col in 0..CASSETTE_COLS {
            let index = row * CASSETTE_COLS + col;
            cuts = cuts
                + centered_cube(
                    format!("preseed_air_bubble_quarantine_cassette_slot_{index}_sealed_recess"),
                    CASSETTE_SLOT_X,
                    CASSETTE_SLOT_Y,
                    CASSETTE_SLOT_DEPTH + 1.0,
                )
                .translate(
                    cassette_slot_x(col),
                    cassette_slot_y(row),
                    NEST_Z - CASSETTE_SLOT_DEPTH / 2.0 + 0.5,
                );
        }
    }
    cuts
}

fn cassette_seal_lip() -> Part {
    let front = centered_cube(
        "preseed_air_bubble_quarantine_cassette_front_seal_lip",
        NEST_X,
        NEST_SEAL_LIP_W,
        18.0,
    )
    .translate(0.0, -NEST_Y / 2.0 + NEST_SEAL_LIP_W / 2.0, NEST_Z + 9.0);
    let rear = centered_cube(
        "preseed_air_bubble_quarantine_cassette_rear_seal_lip",
        NEST_X,
        NEST_SEAL_LIP_W,
        18.0,
    )
    .translate(0.0, NEST_Y / 2.0 - NEST_SEAL_LIP_W / 2.0, NEST_Z + 9.0);
    let left = centered_cube(
        "preseed_air_bubble_quarantine_cassette_left_seal_lip",
        NEST_SEAL_LIP_W,
        NEST_Y,
        18.0,
    )
    .translate(-NEST_X / 2.0 + NEST_SEAL_LIP_W / 2.0, 0.0, NEST_Z + 9.0);
    let right = centered_cube(
        "preseed_air_bubble_quarantine_cassette_right_seal_lip",
        NEST_SEAL_LIP_W,
        NEST_Y,
        18.0,
    )
    .translate(NEST_X / 2.0 - NEST_SEAL_LIP_W / 2.0, 0.0, NEST_Z + 9.0);
    front + rear + left + right
}

fn cassette_datum_pins() -> Part {
    let mut pins = Part::empty("preseed_air_bubble_quarantine_cassette_datum_pins");
    for (i, (x, y)) in [
        (-NEST_X / 2.0 + 38.0, -NEST_Y / 2.0 + 34.0),
        (NEST_X / 2.0 - 38.0, -NEST_Y / 2.0 + 34.0),
        (-NEST_X / 2.0 + 38.0, NEST_Y / 2.0 - 34.0),
        (NEST_X / 2.0 - 38.0, NEST_Y / 2.0 - 34.0),
        (-NEST_X / 2.0 + 38.0, 0.0),
        (NEST_X / 2.0 - 38.0, 0.0),
    ]
    .iter()
    .copied()
    .enumerate()
    {
        pins = pins
            + centered_cylinder(
                format!("preseed_air_bubble_quarantine_cassette_datum_pin_{i}"),
                5.0,
                16.0,
                32,
            )
            .translate(x, y, NEST_Z + 8.0)
            + centered_cylinder(
                format!("preseed_air_bubble_quarantine_cassette_datum_pin_{i}_seal_land"),
                13.0,
                4.0,
                36,
            )
            .translate(x, y, NEST_Z + 2.0);
    }
    pins
}

fn cassette_hold_downs() -> Part {
    let mut clamps = Part::empty("preseed_air_bubble_quarantine_cassette_hold_downs");
    for slot in 0..CASSETTE_SLOTS {
        let row = slot / CASSETTE_COLS;
        let col = slot % CASSETTE_COLS;
        clamps = clamps
            + centered_cube(
                format!("preseed_air_bubble_quarantine_slot_{slot}_front_hold_down_tab"),
                42.0,
                10.0,
                16.0,
            )
            .translate(
                cassette_slot_x(col),
                cassette_slot_y(row) - CASSETTE_SLOT_Y / 2.0 - 8.0,
                NEST_Z + 8.0,
            );
    }
    clamps
}

fn cassette_prime_port_lands() -> Part {
    let mut lands = Part::empty("preseed_air_bubble_quarantine_cassette_prime_port_lands");
    for col in 0..CASSETTE_COLS {
        for side in 0..2 {
            let y = if side == 0 {
                -NEST_Y / 2.0 + 24.0
            } else {
                NEST_Y / 2.0 - 24.0
            };
            lands = lands
                + centered_cylinder(
                    format!(
                        "preseed_air_bubble_quarantine_cassette_col_{col}_prime_port_land_{side}"
                    ),
                    12.0,
                    5.0,
                    32,
                )
                .translate(cassette_slot_x(col), y, NEST_Z + 2.5);
        }
    }
    lands
}

fn cassette_wetness_gutters() -> Part {
    let mut gutters = Part::empty("preseed_air_bubble_quarantine_cassette_wetness_gutters");
    for row in 0..CASSETTE_ROWS {
        gutters = gutters
            + centered_cube(
                format!("preseed_air_bubble_quarantine_cassette_row_{row}_wetness_gutter"),
                NEST_X - 62.0,
                6.0,
                4.0,
            )
            .translate(0.0, cassette_slot_y(row), NEST_Z + 2.0);
    }
    gutters
}

fn inlet_outlet_prime_loop_witness() -> Part {
    let panel = centered_cube(
        "preseed_air_bubble_quarantine_prime_loop_panel",
        PRIME_PANEL_X,
        PRIME_PANEL_Y,
        PRIME_PANEL_Z,
    )
    .translate(0.0, 0.0, PRIME_PANEL_Z / 2.0);

    panel - prime_loop_bore_cuts() - prime_window_cuts()
        + prime_loop_trace_rails()
        + prime_witness_lens_lands()
        + prime_loop_labels()
}

fn prime_loop_bore_cuts() -> Part {
    let mut cuts = Part::empty("preseed_air_bubble_quarantine_prime_loop_bore_cuts");
    for loop_index in 0..PRIME_LOOPS {
        let y = prime_loop_y(loop_index);
        cuts = cuts
            + centered_cylinder(
                format!("preseed_air_bubble_quarantine_prime_loop_{loop_index}_straight_bore"),
                PRIME_BORE_D / 2.0,
                PRIME_PANEL_X + 16.0,
                28,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, y, PRIME_PANEL_Z * 0.58)
            + centered_cylinder(
                format!("preseed_air_bubble_quarantine_prime_loop_{loop_index}_left_turn_bore"),
                PRIME_BORE_D / 2.0,
                PRIME_LOOP_PITCH_Y + 18.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(-PRIME_PANEL_X / 2.0 + 56.0, 0.0, PRIME_PANEL_Z * 0.58)
            + centered_cylinder(
                format!("preseed_air_bubble_quarantine_prime_loop_{loop_index}_right_turn_bore"),
                PRIME_BORE_D / 2.0,
                PRIME_LOOP_PITCH_Y + 18.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(PRIME_PANEL_X / 2.0 - 56.0, 0.0, PRIME_PANEL_Z * 0.58);
    }
    cuts
}

fn prime_window_cuts() -> Part {
    let mut cuts = Part::empty("preseed_air_bubble_quarantine_prime_loop_window_cuts");
    for loop_index in 0..PRIME_LOOPS {
        for window in 0..PRIME_WINDOWS_PER_LOOP {
            let index = loop_index * PRIME_WINDOWS_PER_LOOP + window;
            cuts = cuts
                + centered_cube(
                    format!("preseed_air_bubble_quarantine_prime_witness_window_{index}_cut"),
                    PRIME_WINDOW_X,
                    PRIME_WINDOW_Y,
                    PRIME_PANEL_Z + 2.0,
                )
                .translate(
                    centered_index(window, PRIME_WINDOWS_PER_LOOP, 116.0),
                    prime_loop_y(loop_index),
                    PRIME_PANEL_Z / 2.0,
                );
        }
    }
    cuts
}

fn prime_loop_trace_rails() -> Part {
    let mut rails = Part::empty("preseed_air_bubble_quarantine_prime_loop_trace_rails");
    for loop_index in 0..PRIME_LOOPS {
        let y = prime_loop_y(loop_index);
        rails = rails
            + centered_cube(
                format!("preseed_air_bubble_quarantine_prime_loop_{loop_index}_raised_trace"),
                PRIME_PANEL_X - 70.0,
                7.0,
                5.0,
            )
            .translate(0.0, y, PRIME_PANEL_Z + 2.5);
    }
    rails
}

fn prime_witness_lens_lands() -> Part {
    let mut lands = Part::empty("preseed_air_bubble_quarantine_prime_witness_lens_lands");
    for loop_index in 0..PRIME_LOOPS {
        for window in 0..PRIME_WINDOWS_PER_LOOP {
            let index = loop_index * PRIME_WINDOWS_PER_LOOP + window;
            let land = centered_cube(
                format!("preseed_air_bubble_quarantine_prime_witness_window_{index}_lens_land"),
                PRIME_WINDOW_X + 12.0,
                PRIME_WINDOW_Y + 10.0,
                4.0,
            )
            .translate(
                centered_index(window, PRIME_WINDOWS_PER_LOOP, 116.0),
                prime_loop_y(loop_index),
                PRIME_PANEL_Z + 2.0,
            );
            let clear = centered_cube(
                format!(
                    "preseed_air_bubble_quarantine_prime_witness_window_{index}_clear_aperture"
                ),
                PRIME_WINDOW_X,
                PRIME_WINDOW_Y,
                5.0,
            )
            .translate(
                centered_index(window, PRIME_WINDOWS_PER_LOOP, 116.0),
                prime_loop_y(loop_index),
                PRIME_PANEL_Z + 2.0,
            );
            lands = lands + (land - clear);
        }
    }
    lands
}

fn prime_loop_labels() -> Part {
    let inlet = centered_cube(
        "preseed_air_bubble_quarantine_inlet_prime_loop_label_land",
        92.0,
        18.0,
        3.0,
    )
    .translate(
        -PRIME_PANEL_X / 2.0 + 72.0,
        prime_loop_y(0),
        PRIME_PANEL_Z + 1.5,
    );
    let outlet = centered_cube(
        "preseed_air_bubble_quarantine_outlet_prime_loop_label_land",
        92.0,
        18.0,
        3.0,
    )
    .translate(
        -PRIME_PANEL_X / 2.0 + 72.0,
        prime_loop_y(1),
        PRIME_PANEL_Z + 1.5,
    );
    inlet + outlet
}

fn transmitted_light_bubble_window_array() -> Part {
    let base = centered_cube(
        "preseed_air_bubble_quarantine_bubble_window_base",
        BUBBLE_PANEL_X,
        BUBBLE_PANEL_Y,
        BUBBLE_PANEL_Z,
    )
    .translate(0.0, 0.0, BUBBLE_PANEL_Z / 2.0);
    let rear_upright = centered_cube(
        "preseed_air_bubble_quarantine_bubble_window_rear_upright",
        BUBBLE_PANEL_X,
        18.0,
        BUBBLE_UPRIGHT_Z,
    )
    .translate(
        0.0,
        BUBBLE_PANEL_Y / 2.0 - 16.0,
        BUBBLE_PANEL_Z + BUBBLE_UPRIGHT_Z / 2.0,
    );
    let front_upright = centered_cube(
        "preseed_air_bubble_quarantine_bubble_window_front_illuminator_upright",
        BUBBLE_PANEL_X,
        18.0,
        BUBBLE_UPRIGHT_Z,
    )
    .translate(
        0.0,
        -BUBBLE_PANEL_Y / 2.0 + 16.0,
        BUBBLE_PANEL_Z + BUBBLE_UPRIGHT_Z / 2.0,
    );

    base + (rear_upright - bubble_rear_window_cuts())
        + (front_upright - bubble_front_window_cuts())
        + bubble_lane_tube_troughs()
        + transmitted_light_source_lands()
        + bubble_calibration_wells()
        + bubble_index_tick_rail()
}

fn bubble_rear_window_cuts() -> Part {
    let mut cuts = Part::empty("preseed_air_bubble_quarantine_rear_bubble_window_cuts");
    for lane in 0..BUBBLE_WINDOWS {
        cuts = cuts
            + centered_cube(
                format!("preseed_air_bubble_quarantine_lane_{lane}_camera_window_cut"),
                BUBBLE_WINDOW_X,
                BUBBLE_WINDOW_Y + 6.0,
                BUBBLE_WINDOW_Z,
            )
            .translate(
                bubble_lane_x(lane),
                BUBBLE_PANEL_Y / 2.0 - 16.0,
                BUBBLE_PANEL_Z + 72.0,
            );
    }
    cuts
}

fn bubble_front_window_cuts() -> Part {
    let mut cuts = Part::empty("preseed_air_bubble_quarantine_front_bubble_window_cuts");
    for lane in 0..BUBBLE_WINDOWS {
        cuts = cuts
            + centered_cube(
                format!("preseed_air_bubble_quarantine_lane_{lane}_transmitted_light_window_cut"),
                BUBBLE_WINDOW_X,
                BUBBLE_WINDOW_Y + 6.0,
                BUBBLE_WINDOW_Z,
            )
            .translate(
                bubble_lane_x(lane),
                -BUBBLE_PANEL_Y / 2.0 + 16.0,
                BUBBLE_PANEL_Z + 72.0,
            );
    }
    cuts
}

fn bubble_lane_tube_troughs() -> Part {
    let mut troughs = Part::empty("preseed_air_bubble_quarantine_bubble_lane_tube_troughs");
    for lane in 0..BUBBLE_LANES {
        troughs = troughs
            + centered_cylinder(
                format!("preseed_air_bubble_quarantine_bubble_lane_{lane}_tube_trough"),
                5.5,
                BUBBLE_PANEL_Y + 24.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(bubble_lane_x(lane), 0.0, BUBBLE_PANEL_Z + 7.0);
    }
    troughs
}

fn transmitted_light_source_lands() -> Part {
    let mut lands = Part::empty("preseed_air_bubble_quarantine_transmitted_light_lands");
    for lane in 0..BUBBLE_WINDOWS {
        lands = lands
            + centered_cube(
                format!("preseed_air_bubble_quarantine_lane_{lane}_camera_land"),
                54.0,
                20.0,
                5.0,
            )
            .translate(
                bubble_lane_x(lane),
                BUBBLE_PANEL_Y / 2.0 + 20.0,
                BUBBLE_PANEL_Z + 98.0,
            )
            + centered_cube(
                format!("preseed_air_bubble_quarantine_lane_{lane}_transmitted_led_land"),
                54.0,
                20.0,
                5.0,
            )
            .translate(
                bubble_lane_x(lane),
                -BUBBLE_PANEL_Y / 2.0 - 20.0,
                BUBBLE_PANEL_Z + 98.0,
            );
    }
    lands
}

fn bubble_calibration_wells() -> Part {
    let mut wells = Part::empty("preseed_air_bubble_quarantine_bubble_calibration_wells");
    for well in 0..BUBBLE_CALIBRATION_WELLS {
        wells = wells
            + centered_cylinder(
                format!("preseed_air_bubble_quarantine_bubble_reference_well_{well}"),
                12.0,
                5.0,
                28,
            )
            .translate(
                centered_index(well, BUBBLE_CALIBRATION_WELLS, 52.0),
                0.0,
                BUBBLE_PANEL_Z + 2.5,
            );
    }
    wells
}

fn bubble_index_tick_rail() -> Part {
    let mut ticks = Part::empty("preseed_air_bubble_quarantine_bubble_lane_index_ticks");
    for lane in 0..BUBBLE_LANES {
        ticks = ticks
            + centered_cube(
                format!("preseed_air_bubble_quarantine_bubble_lane_{lane}_index_tick"),
                4.0,
                18.0,
                6.0,
            )
            .translate(
                bubble_lane_x(lane),
                -BUBBLE_PANEL_Y / 2.0 + 38.0,
                BUBBLE_PANEL_Z + 3.0,
            );
    }
    ticks
}

fn low_point_purge_pocket_ladder() -> Part {
    let body = centered_cube(
        "preseed_air_bubble_quarantine_low_point_purge_ladder_body",
        PURGE_LADDER_X,
        PURGE_LADDER_Y,
        PURGE_LADDER_Z,
    )
    .translate(0.0, 0.0, PURGE_LADDER_Z / 2.0);

    body - purge_pocket_cuts()
        + purge_pocket_collars()
        + purge_step_index_ribs()
        + purge_drain_witness_gutter()
}

fn purge_pocket_cuts() -> Part {
    let mut cuts = Part::empty("preseed_air_bubble_quarantine_purge_pocket_cuts");
    for pocket in 0..PURGE_POCKETS {
        cuts = cuts
            + centered_cylinder(
                format!("preseed_air_bubble_quarantine_low_point_purge_pocket_{pocket}_cut"),
                PURGE_POCKET_D / 2.0,
                PURGE_POCKET_DEPTH + 1.0,
                36,
            )
            .translate(
                centered_index(pocket % 2, 2, 82.0),
                purge_pocket_y(pocket),
                PURGE_LADDER_Z - PURGE_POCKET_DEPTH / 2.0 + 0.5 - pocket as f64 * 0.45,
            );
    }
    cuts
}

fn purge_pocket_collars() -> Part {
    let mut collars = Part::empty("preseed_air_bubble_quarantine_purge_pocket_collars");
    for pocket in 0..PURGE_POCKETS {
        let x = centered_index(pocket % 2, 2, 82.0);
        let y = purge_pocket_y(pocket);
        let collar = centered_cylinder(
            format!("preseed_air_bubble_quarantine_low_point_purge_pocket_{pocket}_collar"),
            20.0,
            6.0,
            36,
        )
        .translate(x, y, PURGE_LADDER_Z + 3.0 - pocket as f64 * 0.35);
        let bore = centered_cylinder(
            format!("preseed_air_bubble_quarantine_low_point_purge_pocket_{pocket}_collar_bore"),
            PURGE_POCKET_D / 2.0,
            7.0,
            36,
        )
        .translate(x, y, PURGE_LADDER_Z + 3.0 - pocket as f64 * 0.35);
        collars = collars + (collar - bore);
    }
    collars
}

fn purge_step_index_ribs() -> Part {
    let mut ribs = Part::empty("preseed_air_bubble_quarantine_purge_step_index_ribs");
    for rib in 0..PURGE_WITNESS_RIBS {
        ribs = ribs
            + centered_cube(
                format!("preseed_air_bubble_quarantine_purge_ladder_step_index_rib_{rib}"),
                PURGE_LADDER_X - 44.0,
                4.0,
                5.0,
            )
            .translate(
                0.0,
                centered_index(rib, PURGE_WITNESS_RIBS, 34.0),
                PURGE_LADDER_Z + 2.5,
            );
    }
    ribs
}

fn purge_drain_witness_gutter() -> Part {
    centered_cube(
        "preseed_air_bubble_quarantine_purge_low_point_drain_witness_gutter",
        12.0,
        PURGE_LADDER_Y - 36.0,
        6.0,
    )
    .translate(PURGE_LADDER_X / 2.0 - 42.0, 0.0, PURGE_LADDER_Z + 3.0)
}

fn pressure_flow_tap_bosses() -> Part {
    let panel = centered_cube(
        "preseed_air_bubble_quarantine_pressure_flow_tap_panel",
        TAP_PANEL_X,
        TAP_PANEL_Y,
        TAP_PANEL_Z,
    )
    .translate(0.0, 0.0, TAP_PANEL_Z / 2.0);

    panel - tap_bore_cuts()
        + tap_boss_collars()
        + tap_lane_labels()
        + pressure_flow_reference_blinds()
}

fn tap_bore_cuts() -> Part {
    let mut cuts = Part::empty("preseed_air_bubble_quarantine_tap_bore_cuts");
    for lane in 0..BUBBLE_LANES {
        let x = tap_lane_x(lane);
        cuts = cuts
            + centered_cylinder(
                format!("preseed_air_bubble_quarantine_lane_{lane}_pressure_tap_bore"),
                TAP_BORE_D / 2.0,
                TAP_PANEL_Y + 12.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, TAP_PANEL_Z * 0.65)
            + centered_cylinder(
                format!("preseed_air_bubble_quarantine_lane_{lane}_flow_tap_bore"),
                TAP_BORE_D / 2.0,
                TAP_PANEL_Y + 12.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, TAP_PANEL_Z * 0.34);
    }
    cuts
}

fn tap_boss_collars() -> Part {
    let mut collars = Part::empty("preseed_air_bubble_quarantine_tap_boss_collars");
    for lane in 0..BUBBLE_LANES {
        for (kind, z) in [
            ("pressure", TAP_PANEL_Z * 0.65),
            ("flow", TAP_PANEL_Z * 0.34),
        ] {
            let x = tap_lane_x(lane);
            let boss = centered_cylinder(
                format!("preseed_air_bubble_quarantine_lane_{lane}_{kind}_tap_boss"),
                TAP_BOSS_D / 2.0,
                8.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -TAP_PANEL_Y / 2.0 - 4.0, z);
            let bore = centered_cylinder(
                format!("preseed_air_bubble_quarantine_lane_{lane}_{kind}_tap_boss_bore"),
                TAP_BORE_D / 2.0,
                10.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -TAP_PANEL_Y / 2.0 - 4.0, z);
            collars = collars + (boss - bore);
        }
    }
    collars
}

fn tap_lane_labels() -> Part {
    let mut labels = Part::empty("preseed_air_bubble_quarantine_tap_lane_labels");
    for lane in 0..BUBBLE_LANES {
        labels = labels
            + centered_cube(
                format!("preseed_air_bubble_quarantine_lane_{lane}_tap_label_land"),
                34.0,
                16.0,
                3.0,
            )
            .translate(
                tap_lane_x(lane),
                TAP_PANEL_Y / 2.0 - 18.0,
                TAP_PANEL_Z + 1.5,
            );
    }
    labels
}

fn pressure_flow_reference_blinds() -> Part {
    let mut blinds = Part::empty("preseed_air_bubble_quarantine_pressure_flow_reference_blinds");
    for blind in 0..4 {
        blinds = blinds
            + centered_cylinder(
                format!("preseed_air_bubble_quarantine_reference_blind_tap_{blind}"),
                13.0,
                7.0,
                32,
            )
            .translate(centered_index(blind, 4, 42.0), 0.0, TAP_PANEL_Z + 3.5);
    }
    blinds
}

fn dye_air_slug_challenge_token_rail() -> Part {
    let rail = centered_cube(
        "preseed_air_bubble_quarantine_dye_air_slug_token_rail",
        TOKEN_RAIL_X,
        TOKEN_RAIL_Y,
        TOKEN_RAIL_Z,
    )
    .translate(0.0, 0.0, TOKEN_RAIL_Z / 2.0);

    rail - challenge_token_slot_cuts()
        + challenge_token_lands()
        + challenge_index_ticks()
        + dye_air_separator_key()
}

fn challenge_token_slot_cuts() -> Part {
    let mut cuts = Part::empty("preseed_air_bubble_quarantine_challenge_token_slot_cuts");
    for slot in 0..CHALLENGE_TOKEN_SLOTS {
        cuts = cuts
            + centered_cube(
                format!("preseed_air_bubble_quarantine_challenge_token_slot_{slot}_cut"),
                TOKEN_SLOT_X,
                TOKEN_SLOT_Y,
                16.0,
            )
            .translate(
                centered_index(slot, CHALLENGE_TOKEN_SLOTS, 34.0),
                0.0,
                TOKEN_RAIL_Z - 7.5,
            );
    }
    cuts
}

fn challenge_token_lands() -> Part {
    let mut lands = Part::empty("preseed_air_bubble_quarantine_challenge_token_lands");
    for slot in 0..CHALLENGE_TOKEN_SLOTS {
        let token_kind = if slot < DYE_TOKEN_SLOTS { "dye" } else { "air" };
        lands = lands
            + centered_cube(
                format!("preseed_air_bubble_quarantine_{token_kind}_slug_token_{slot}_label_land"),
                30.0,
                16.0,
                3.0,
            )
            .translate(
                centered_index(slot, CHALLENGE_TOKEN_SLOTS, 34.0),
                -TOKEN_RAIL_Y / 2.0 + 14.0,
                TOKEN_RAIL_Z + 1.5,
            );
    }
    lands
}

fn challenge_index_ticks() -> Part {
    let mut ticks = Part::empty("preseed_air_bubble_quarantine_challenge_index_ticks");
    for tick in 0..CHALLENGE_INDEX_TICKS {
        ticks = ticks
            + centered_cube(
                format!("preseed_air_bubble_quarantine_challenge_index_tick_{tick}"),
                3.0,
                18.0,
                6.0,
            )
            .translate(
                centered_index(tick, CHALLENGE_INDEX_TICKS, 34.0),
                TOKEN_RAIL_Y / 2.0 - 13.0,
                TOKEN_RAIL_Z + 3.0,
            );
    }
    ticks
}

fn dye_air_separator_key() -> Part {
    centered_cube(
        "preseed_air_bubble_quarantine_dye_air_slug_separator_key",
        7.0,
        TOKEN_RAIL_Y - 18.0,
        12.0,
    )
    .translate(
        (centered_index(DYE_TOKEN_SLOTS - 1, CHALLENGE_TOKEN_SLOTS, 34.0)
            + centered_index(DYE_TOKEN_SLOTS, CHALLENGE_TOKEN_SLOTS, 34.0))
            / 2.0,
        0.0,
        TOKEN_RAIL_Z + 6.0,
    )
}

fn waste_retain_split_manifold() -> Part {
    let panel = centered_cube(
        "preseed_air_bubble_quarantine_waste_retain_split_panel",
        SPLIT_PANEL_X,
        SPLIT_PANEL_Y,
        SPLIT_PANEL_Z,
    )
    .translate(0.0, 0.0, SPLIT_PANEL_Z / 2.0);

    panel - split_capture_well_cuts()
        + split_manifold_raised_paths()
        + split_capture_collars()
        + split_valve_bosses()
        + split_branch_label_lands()
}

fn split_capture_well_cuts() -> Part {
    let mut cuts = Part::empty("preseed_air_bubble_quarantine_split_capture_well_cuts");
    for branch in 0..SPLIT_BRANCHES {
        for well in 0..SPLIT_CAPTURE_WELLS_PER_BRANCH {
            cuts = cuts
                + centered_cylinder(
                    format!(
                        "preseed_air_bubble_quarantine_{}_capture_well_{well}_cut",
                        SPLIT_LANE_NAMES[branch]
                    ),
                    SPLIT_WELL_D / 2.0,
                    16.0,
                    32,
                )
                .translate(
                    centered_index(well, SPLIT_CAPTURE_WELLS_PER_BRANCH, 46.0),
                    split_branch_y(branch),
                    SPLIT_PANEL_Z - 7.5,
                );
        }
    }
    cuts
}

fn split_manifold_raised_paths() -> Part {
    let trunk = centered_cube(
        "preseed_air_bubble_quarantine_split_manifold_inlet_trunk",
        SPLIT_PANEL_X - 92.0,
        8.0,
        6.0,
    )
    .translate(0.0, 0.0, SPLIT_PANEL_Z + 3.0);
    let retain = centered_cube(
        "preseed_air_bubble_quarantine_retain_branch_raised_path",
        174.0,
        8.0,
        6.0,
    )
    .rotate(0.0, 0.0, -18.0)
    .translate(82.0, split_branch_y(0) / 2.0, SPLIT_PANEL_Z + 3.0);
    let waste = centered_cube(
        "preseed_air_bubble_quarantine_waste_branch_raised_path",
        174.0,
        8.0,
        6.0,
    )
    .rotate(0.0, 0.0, 18.0)
    .translate(82.0, split_branch_y(1) / 2.0, SPLIT_PANEL_Z + 3.0);
    trunk + retain + waste
}

fn split_capture_collars() -> Part {
    let mut collars = Part::empty("preseed_air_bubble_quarantine_split_capture_collars");
    for branch in 0..SPLIT_BRANCHES {
        for well in 0..SPLIT_CAPTURE_WELLS_PER_BRANCH {
            collars = collars
                + centered_cylinder(
                    format!(
                        "preseed_air_bubble_quarantine_{}_capture_well_{well}_collar",
                        SPLIT_LANE_NAMES[branch]
                    ),
                    16.0,
                    5.0,
                    32,
                )
                .translate(
                    centered_index(well, SPLIT_CAPTURE_WELLS_PER_BRANCH, 46.0),
                    split_branch_y(branch),
                    SPLIT_PANEL_Z + 2.5,
                );
        }
    }
    collars
}

fn split_valve_bosses() -> Part {
    let mut bosses = Part::empty("preseed_air_bubble_quarantine_split_valve_bosses");
    for boss in 0..SPLIT_VALVE_BOSSES {
        bosses = bosses
            + centered_cylinder(
                format!("preseed_air_bubble_quarantine_split_selector_valve_boss_{boss}"),
                17.0,
                8.0,
                36,
            )
            .translate(
                -SPLIT_PANEL_X / 2.0 + 82.0 + boss as f64 * 48.0,
                0.0,
                SPLIT_PANEL_Z + 4.0,
            );
    }
    bosses
}

fn split_branch_label_lands() -> Part {
    let mut lands = Part::empty("preseed_air_bubble_quarantine_split_branch_label_lands");
    for branch in 0..SPLIT_BRANCHES {
        lands = lands
            + centered_cube(
                format!(
                    "preseed_air_bubble_quarantine_{}_branch_label_land",
                    SPLIT_LANE_NAMES[branch]
                ),
                84.0,
                18.0,
                3.0,
            )
            .translate(
                SPLIT_PANEL_X / 2.0 - 64.0,
                split_branch_y(branch),
                SPLIT_PANEL_Z + 1.5,
            );
    }
    lands
}

fn barcode_custody_lands() -> Part {
    let plate = centered_cube(
        "preseed_air_bubble_quarantine_barcode_custody_plate",
        CUSTODY_PLATE_X,
        CUSTODY_PLATE_Y,
        CUSTODY_PLATE_Z,
    )
    .translate(0.0, 0.0, CUSTODY_PLATE_Z / 2.0);

    plate + barcode_land_blocks() + custody_card_land_blocks() + custody_seal_posts()
}

fn barcode_land_blocks() -> Part {
    let mut lands = Part::empty("preseed_air_bubble_quarantine_barcode_lands");
    for land in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("preseed_air_bubble_quarantine_cassette_{land}_barcode_land"),
                54.0,
                16.0,
                3.0,
            )
            .translate(
                centered_index(land % 4, 4, 74.0),
                CUSTODY_PLATE_Y / 2.0 - 24.0 - (land / 4) as f64 * 26.0,
                CUSTODY_PLATE_Z + 1.5,
            );
    }
    lands
}

fn custody_card_land_blocks() -> Part {
    let mut lands = Part::empty("preseed_air_bubble_quarantine_custody_card_lands");
    for land in 0..CUSTODY_CARD_LANDS {
        lands = lands
            + centered_cube(
                format!("preseed_air_bubble_quarantine_custody_card_land_{land}"),
                72.0,
                20.0,
                3.0,
            )
            .translate(
                centered_index(land, CUSTODY_CARD_LANDS, 82.0),
                -CUSTODY_PLATE_Y / 2.0 + 26.0,
                CUSTODY_PLATE_Z + 1.5,
            );
    }
    lands
}

fn custody_seal_posts() -> Part {
    let mut posts = Part::empty("preseed_air_bubble_quarantine_custody_seal_posts");
    for post in 0..CUSTODY_SEAL_POSTS {
        posts = posts
            + centered_cylinder(
                format!("preseed_air_bubble_quarantine_custody_tamper_seal_post_{post}"),
                8.0,
                14.0,
                28,
            )
            .translate(
                centered_index(post, CUSTODY_SEAL_POSTS, 72.0),
                0.0,
                CUSTODY_PLATE_Z + 7.0,
            );
    }
    posts
}

fn release_hold_reject_gate_bank() -> Part {
    let bank = centered_cube(
        "preseed_air_bubble_quarantine_release_hold_reject_gate_bank",
        GATE_BANK_X,
        GATE_BANK_Y,
        GATE_BANK_Z,
    )
    .translate(0.0, 0.0, GATE_BANK_Z / 2.0);

    bank - gate_slot_cuts() + gate_dividers() + gate_selector_paddles() + gate_lane_label_lands()
}

fn gate_slot_cuts() -> Part {
    let mut cuts = Part::empty("preseed_air_bubble_quarantine_gate_slot_cuts");
    for lane in 0..STATUS_LANES {
        for slot in 0..GATE_SLOTS_PER_LANE {
            cuts = cuts
                + centered_cube(
                    format!(
                        "preseed_air_bubble_quarantine_{}_gate_slot_{slot}_cut",
                        STATUS_LANE_NAMES[lane]
                    ),
                    GATE_SLOT_X,
                    GATE_SLOT_Y,
                    16.0,
                )
                .translate(
                    centered_index(slot, GATE_SLOTS_PER_LANE, 58.0),
                    gate_lane_y(lane),
                    GATE_BANK_Z - 7.5,
                );
        }
    }
    cuts
}

fn gate_dividers() -> Part {
    let mut dividers = Part::empty("preseed_air_bubble_quarantine_gate_dividers");
    for lane in 0..STATUS_LANES - 1 {
        dividers = dividers
            + centered_cube(
                format!("preseed_air_bubble_quarantine_gate_lane_divider_{lane}"),
                GATE_BANK_X - 28.0,
                5.0,
                18.0,
            )
            .translate(
                0.0,
                (gate_lane_y(lane) + gate_lane_y(lane + 1)) / 2.0,
                GATE_BANK_Z + 9.0,
            );
    }
    dividers
}

fn gate_selector_paddles() -> Part {
    let mut paddles = Part::empty("preseed_air_bubble_quarantine_gate_selector_paddles");
    for lane in 0..STATUS_LANES {
        paddles = paddles
            + centered_cube(
                format!(
                    "preseed_air_bubble_quarantine_{}_gate_selector_paddle",
                    STATUS_LANE_NAMES[lane]
                ),
                42.0,
                18.0,
                10.0,
            )
            .translate(
                -GATE_BANK_X / 2.0 + 38.0,
                gate_lane_y(lane),
                GATE_BANK_Z + 5.0,
            );
    }
    paddles
}

fn gate_lane_label_lands() -> Part {
    let mut lands = Part::empty("preseed_air_bubble_quarantine_gate_lane_label_lands");
    for lane in 0..STATUS_LANES {
        lands = lands
            + centered_cube(
                format!(
                    "preseed_air_bubble_quarantine_{}_gate_label_land",
                    STATUS_LANE_NAMES[lane]
                ),
                62.0,
                16.0,
                3.0,
            )
            .translate(
                GATE_BANK_X / 2.0 - 44.0,
                gate_lane_y(lane),
                GATE_BANK_Z + 1.5,
            );
    }
    lands
}

fn camera_evidence_bridge() -> Part {
    let left_post = centered_cube(
        "preseed_air_bubble_quarantine_camera_evidence_bridge_left_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_POST_Z,
    )
    .translate(
        BRIDGE_POS.0 - BRIDGE_SPAN_X / 2.0,
        BRIDGE_POS.1,
        BASE_Z + BRIDGE_POST_Z / 2.0,
    );
    let right_post = centered_cube(
        "preseed_air_bubble_quarantine_camera_evidence_bridge_right_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_POST_Z,
    )
    .translate(
        BRIDGE_POS.0 + BRIDGE_SPAN_X / 2.0,
        BRIDGE_POS.1,
        BASE_Z + BRIDGE_POST_Z / 2.0,
    );
    let beam = centered_cube(
        "preseed_air_bubble_quarantine_camera_evidence_bridge_beam",
        BRIDGE_SPAN_X + BRIDGE_POST_X,
        BRIDGE_BEAM_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(
        BRIDGE_POS.0,
        BRIDGE_POS.1,
        BASE_Z + BRIDGE_POST_Z + BRIDGE_BEAM_Z / 2.0,
    );

    left_post
        + right_post
        + beam
        + evidence_camera_pods()
        + evidence_led_segments()
        + evidence_card_lands()
}

fn evidence_camera_pods() -> Part {
    let mut pods = Part::empty("preseed_air_bubble_quarantine_evidence_camera_pods");
    for pod in 0..EVIDENCE_CAMERA_PODS {
        pods = pods
            + centered_cube(
                format!("preseed_air_bubble_quarantine_evidence_camera_pod_{pod}"),
                86.0,
                42.0,
                30.0,
            )
            .translate(
                BRIDGE_POS.0 + centered_index(pod, EVIDENCE_CAMERA_PODS, 245.0),
                BRIDGE_POS.1 - 8.0,
                BASE_Z + BRIDGE_POST_Z - 18.0,
            );
    }
    pods
}

fn evidence_led_segments() -> Part {
    let mut leds = Part::empty("preseed_air_bubble_quarantine_evidence_led_segments");
    for segment in 0..EVIDENCE_LED_SEGMENTS {
        leds = leds
            + centered_cube(
                format!(
                "preseed_air_bubble_quarantine_transmitted_light_evidence_led_segment_{segment}"
            ),
                54.0,
                8.0,
                6.0,
            )
            .translate(
                BRIDGE_POS.0 + centered_index(segment, EVIDENCE_LED_SEGMENTS, 64.0),
                BRIDGE_POS.1 - BRIDGE_BEAM_Y / 2.0 - 8.0,
                BASE_Z + BRIDGE_POST_Z - 42.0,
            );
    }
    leds
}

fn evidence_card_lands() -> Part {
    let mut lands = Part::empty("preseed_air_bubble_quarantine_evidence_card_lands");
    for land in 0..EVIDENCE_CARD_LANDS {
        lands = lands
            + centered_cube(
                format!("preseed_air_bubble_quarantine_evidence_card_land_{land}"),
                118.0,
                22.0,
                5.0,
            )
            .translate(
                BRIDGE_POS.0 + centered_index(land, EVIDENCE_CARD_LANDS, 136.0),
                BRIDGE_POS.1 + BRIDGE_BEAM_Y / 2.0 + 16.0,
                BASE_Z + BRIDGE_POST_Z - 38.0,
            );
    }
    lands
}

fn robotic_service_datums() -> Part {
    robot_datum_pins()
        + robot_service_fiducials()
        + robot_tool_dock_datums()
        + robot_sweep_keepout()
        + service_clearance_gauges()
}

fn robot_datum_pins() -> Part {
    let mut pins = Part::empty("preseed_air_bubble_quarantine_robot_datum_pins");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 162.0, -STATION_Y / 2.0 + 148.0),
        (STATION_X / 2.0 - 162.0, -STATION_Y / 2.0 + 148.0),
        (-STATION_X / 2.0 + 162.0, STATION_Y / 2.0 - 148.0),
        (STATION_X / 2.0 - 162.0, STATION_Y / 2.0 - 148.0),
    ]
    .iter()
    .copied()
    .enumerate()
    {
        pins = pins
            + centered_cylinder(
                format!("preseed_air_bubble_quarantine_robot_datum_pin_{i}"),
                9.0,
                28.0,
                36,
            )
            .translate(x, y, BASE_Z + 14.0)
            + centered_cube(
                format!("preseed_air_bubble_quarantine_robot_datum_pin_{i}_approach_land"),
                46.0,
                24.0,
                4.0,
            )
            .translate(x, y - 34.0, BASE_Z + 2.0);
    }
    pins
}

fn robot_service_fiducials() -> Part {
    let mut fiducials = Part::empty("preseed_air_bubble_quarantine_robot_service_fiducials");
    for fiducial in 0..ROBOT_SERVICE_FIDUCIALS {
        let x = centered_index(fiducial % 4, 4, 190.0);
        let y = if fiducial < 4 {
            -STATION_Y / 2.0 + 116.0
        } else {
            STATION_Y / 2.0 - 116.0
        };
        let target = centered_cylinder(
            format!("preseed_air_bubble_quarantine_robot_service_fiducial_{fiducial}"),
            12.0,
            5.0,
            32,
        )
        .translate(x, y, BASE_Z + 2.5);
        let bore = centered_cylinder(
            format!("preseed_air_bubble_quarantine_robot_service_fiducial_{fiducial}_center_bore"),
            3.2,
            6.0,
            24,
        )
        .translate(x, y, BASE_Z + 3.0);
        fiducials = fiducials + (target - bore);
    }
    fiducials
}

fn robot_tool_dock_datums() -> Part {
    let mut datums = Part::empty("preseed_air_bubble_quarantine_robot_tool_dock_datums");
    for datum in 0..ROBOT_TOOL_DOCK_DATUMS {
        datums = datums
            + centered_cube(
                format!("preseed_air_bubble_quarantine_robot_tool_dock_datum_block_{datum}"),
                68.0,
                34.0,
                24.0,
            )
            .translate(
                STATION_X / 2.0 - 112.0,
                centered_index(datum, ROBOT_TOOL_DOCK_DATUMS, 72.0),
                BASE_Z + 12.0,
            )
            - centered_cylinder(
                format!("preseed_air_bubble_quarantine_robot_tool_dock_datum_{datum}_pin_bore"),
                5.0,
                30.0,
                24,
            )
            .translate(
                STATION_X / 2.0 - 112.0,
                centered_index(datum, ROBOT_TOOL_DOCK_DATUMS, 72.0),
                BASE_Z + 12.0,
            );
    }
    datums
}

fn robot_sweep_keepout() -> Part {
    wireframe_box(
        "preseed_air_bubble_quarantine_robot_sweep_keepout",
        ROBOT_SWEEP_X,
        ROBOT_SWEEP_Y,
        ROBOT_SWEEP_Z,
        KEEP_OUT_RAIL,
    )
    .translate(0.0, -4.0, BASE_Z + ROBOT_SWEEP_Z / 2.0)
}

fn service_clearance_gauges() -> Part {
    let front = centered_cube(
        "preseed_air_bubble_quarantine_front_robot_service_clearance_gauge",
        590.0,
        KEEP_OUT_RAIL,
        38.0,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 - FRONT_ROBOT_SERVICE / 2.0,
        BASE_Z + 19.0,
    );
    let rear = centered_cube(
        "preseed_air_bubble_quarantine_rear_optical_service_clearance_gauge",
        660.0,
        KEEP_OUT_RAIL,
        38.0,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 + REAR_OPTICAL_SERVICE / 2.0,
        BASE_Z + 19.0,
    );
    let left = centered_cube(
        "preseed_air_bubble_quarantine_left_cassette_service_clearance_gauge",
        KEEP_OUT_RAIL,
        356.0,
        38.0,
    )
    .translate(
        -STATION_X / 2.0 - LEFT_CASSETTE_SERVICE / 2.0,
        60.0,
        BASE_Z + 19.0,
    );
    let right = centered_cube(
        "preseed_air_bubble_quarantine_right_purge_service_clearance_gauge",
        KEEP_OUT_RAIL,
        356.0,
        38.0,
    )
    .translate(
        STATION_X / 2.0 + RIGHT_PURGE_SERVICE / 2.0,
        -90.0,
        BASE_Z + 19.0,
    );
    front + rear + left + right
}

fn wireframe_box(name: &str, x: f64, y: f64, z: f64, rail: f64) -> Part {
    let mut part = Part::empty(name);
    for (zi, dz) in [-1.0, 1.0].iter().copied().enumerate() {
        for (yi, dy) in [-1.0, 1.0].iter().copied().enumerate() {
            part = part
                + centered_cube(format!("{name}_x_rail_{zi}_{yi}"), x, rail, rail).translate(
                    0.0,
                    dy * y / 2.0,
                    dz * z / 2.0,
                );
        }
        for (xi, dx) in [-1.0, 1.0].iter().copied().enumerate() {
            part = part
                + centered_cube(format!("{name}_y_rail_{zi}_{xi}"), rail, y, rail).translate(
                    dx * x / 2.0,
                    0.0,
                    dz * z / 2.0,
                );
        }
    }
    part
}

fn module_footprints() -> [Footprint; 9] {
    [
        Footprint {
            name: "sealed_cassette_datum_nest",
            center: NEST_POS,
            x: NEST_X,
            y: NEST_Y,
        },
        Footprint {
            name: "prime_loop_witness",
            center: PRIME_POS,
            x: PRIME_PANEL_X,
            y: PRIME_PANEL_Y,
        },
        Footprint {
            name: "bubble_window_array",
            center: BUBBLE_POS,
            x: BUBBLE_PANEL_X,
            y: BUBBLE_PANEL_Y,
        },
        Footprint {
            name: "purge_pocket_ladder",
            center: PURGE_POS,
            x: PURGE_LADDER_X,
            y: PURGE_LADDER_Y,
        },
        Footprint {
            name: "pressure_flow_taps",
            center: TAP_POS,
            x: TAP_PANEL_X,
            y: TAP_PANEL_Y,
        },
        Footprint {
            name: "slug_challenge_tokens",
            center: TOKEN_POS,
            x: TOKEN_RAIL_X,
            y: TOKEN_RAIL_Y,
        },
        Footprint {
            name: "waste_retain_split",
            center: SPLIT_POS,
            x: SPLIT_PANEL_X,
            y: SPLIT_PANEL_Y,
        },
        Footprint {
            name: "barcode_custody_lands",
            center: CUSTODY_POS,
            x: CUSTODY_PLATE_X,
            y: CUSTODY_PLATE_Y,
        },
        Footprint {
            name: "release_hold_reject_gates",
            center: GATE_POS,
            x: GATE_BANK_X,
            y: GATE_BANK_Y,
        },
    ]
}

fn fits_inside_station(footprint: Footprint) -> bool {
    let usable_x = STATION_X / 2.0 - CURB_W - 24.0;
    let usable_y = STATION_Y / 2.0 - CURB_W - 24.0;
    footprint.center.0 - footprint.x / 2.0 >= -usable_x
        && footprint.center.0 + footprint.x / 2.0 <= usable_x
        && footprint.center.1 - footprint.y / 2.0 >= -usable_y
        && footprint.center.1 + footprint.y / 2.0 <= usable_y
}

fn cassette_slot_x(col: usize) -> f64 {
    centered_index(col, CASSETTE_COLS, CASSETTE_SLOT_PITCH_X)
}

fn cassette_slot_y(row: usize) -> f64 {
    centered_index(row, CASSETTE_ROWS, CASSETTE_SLOT_PITCH_Y)
}

fn prime_loop_y(loop_index: usize) -> f64 {
    centered_index(loop_index, PRIME_LOOPS, PRIME_LOOP_PITCH_Y)
}

fn bubble_lane_x(lane: usize) -> f64 {
    centered_index(lane, BUBBLE_LANES, BUBBLE_LANE_PITCH_X)
}

fn purge_pocket_y(pocket: usize) -> f64 {
    centered_index(pocket, PURGE_POCKETS, 34.0)
}

fn tap_lane_x(lane: usize) -> f64 {
    centered_index(lane, BUBBLE_LANES, TAP_LANE_PITCH_X)
}

fn split_branch_y(branch: usize) -> f64 {
    centered_index(branch, SPLIT_BRANCHES, 48.0)
}

fn gate_lane_y(lane: usize) -> f64 {
    centered_index(lane, STATUS_LANES, 34.0)
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 12);
    assert_eq!(REQUIRED_FEATURES.len(), 11);
    assert_eq!(LIMITATIONS.len(), 5);
    assert_eq!(DECK_DATUM_TARGETS, 6);
    assert_eq!(CASSETTE_SLOTS, 8);
    assert_eq!(CASSETTE_SLOTS, CASSETTE_ROWS * CASSETTE_COLS);
    assert_eq!(BUBBLE_LANES, CASSETTE_SLOTS);
    assert_eq!(BUBBLE_WINDOWS, BUBBLE_LANES);
    assert_eq!(TRANSMITTED_LIGHT_PAIRS, BUBBLE_WINDOWS);
    assert_eq!(PRIME_WITNESS_WINDOWS, PRIME_LOOPS * PRIME_WINDOWS_PER_LOOP);
    assert_eq!(PRESSURE_TAPS, FLOW_TAPS);
    assert_eq!(TAP_BOSSES, PRESSURE_TAPS + FLOW_TAPS);
    assert_eq!(CHALLENGE_TOKEN_SLOTS, DYE_TOKEN_SLOTS + AIR_TOKEN_SLOTS);
    assert_eq!(CHALLENGE_INDEX_TICKS, CHALLENGE_TOKEN_SLOTS + 1);
    assert_eq!(SPLIT_LANE_NAMES, ["retain", "waste"]);
    assert_eq!(
        SPLIT_CAPTURE_WELLS,
        SPLIT_BRANCHES * SPLIT_CAPTURE_WELLS_PER_BRANCH
    );
    assert_eq!(STATUS_LANE_NAMES, ["release", "hold", "reject"]);
    assert_eq!(GATE_SLOTS, STATUS_LANES * GATE_SLOTS_PER_LANE);
    assert!(GATE_SLOTS >= CASSETTE_SLOTS);
    assert!(BRIDGE_POST_Z > CAMERA_CLEARANCE_Z);
    assert!(ROBOT_SWEEP_X < STATION_X);
    assert!(ROBOT_SWEEP_Y < STATION_Y);

    let unique_outputs: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
    assert_eq!(unique_outputs.len(), OUTPUTS.len());
    for (path, feature) in OUTPUTS
        .iter()
        .take(REQUIRED_FEATURES.len())
        .zip(REQUIRED_FEATURES)
    {
        assert!(path.starts_with(OUTPUT_PREFIX), "{path}");
        assert!(path.contains(feature), "{path} missing {feature}");
        assert!(path.ends_with(".stl"), "{path}");
    }
    assert!(OUTPUTS[11].ends_with("_assembly.stl"));

    for footprint in module_footprints() {
        assert!(
            fits_inside_station(footprint),
            "{} exceeds deck envelope",
            footprint.name
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_names_are_unique_scoped_and_complete() {
        assert_design_constraints();
        assert!(OUTPUTS[0].contains("sealed_cassette_datum_nest"));
        assert!(OUTPUTS[1].contains("inlet_outlet_prime_loop_witness"));
        assert!(OUTPUTS[2].contains("transmitted_light_bubble_window_array"));
        assert!(OUTPUTS[10].contains("robotic_service_datums"));
        assert!(OUTPUTS[11].ends_with("_assembly.stl"));
    }

    #[test]
    fn required_feature_coverage_matches_quarantine_scope() {
        for feature in [
            "sealed_cassette_datum_nest",
            "inlet_outlet_prime_loop_witness",
            "transmitted_light_bubble_window_array",
            "low_point_purge_pocket_ladder",
            "pressure_flow_tap_bosses",
            "dye_air_slug_challenge_token_rail",
            "waste_retain_split_manifold",
            "barcode_custody_lands",
            "release_hold_reject_gate_bank",
            "camera_evidence_bridge",
            "robotic_service_datums",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn cassette_prime_and_bubble_lane_counts_align() {
        assert_eq!(CASSETTE_ROWS, 2);
        assert_eq!(CASSETTE_COLS, 4);
        assert_eq!(CASSETTE_SLOTS, 8);
        assert_eq!(NEST_DATUM_PINS, 6);
        assert_eq!(CASSETTE_HOLD_DOWNS, CASSETTE_SLOTS);
        assert_eq!(PRIME_LOOPS, 2);
        assert_eq!(PRIME_WITNESS_WINDOWS, 8);
        assert_eq!(BUBBLE_LANES, CASSETTE_SLOTS);
        assert_eq!(BUBBLE_WINDOWS, CASSETTE_SLOTS);
        assert_eq!(TRANSMITTED_LIGHT_PAIRS, BUBBLE_WINDOWS);
    }

    #[test]
    fn purge_tap_split_and_gate_counts_are_explicit() {
        assert_eq!(PURGE_POCKETS, 6);
        assert_eq!(PURGE_WITNESS_RIBS, PURGE_POCKETS + 1);
        assert!(PURGE_STEP_DROP_MM >= 4.0);
        assert_eq!(PRESSURE_TAPS, CASSETTE_SLOTS);
        assert_eq!(FLOW_TAPS, CASSETTE_SLOTS);
        assert_eq!(TAP_BOSSES, 16);
        assert_eq!(SPLIT_BRANCHES, 2);
        assert_eq!(SPLIT_LANE_NAMES, ["retain", "waste"]);
        assert_eq!(SPLIT_CAPTURE_WELLS, 8);
        assert_eq!(STATUS_LANE_NAMES, ["release", "hold", "reject"]);
        assert!(GATE_SLOTS >= CASSETTE_SLOTS);
    }

    #[test]
    fn custody_challenge_evidence_and_robotics_are_dimensioned() {
        assert_eq!(DYE_TOKEN_SLOTS, 6);
        assert_eq!(AIR_TOKEN_SLOTS, 4);
        assert_eq!(CHALLENGE_TOKEN_SLOTS, 10);
        assert_eq!(CHALLENGE_INDEX_TICKS, 11);
        assert_eq!(BARCODE_LANDS, CASSETTE_SLOTS);
        assert_eq!(CUSTODY_CARD_LANDS, 4);
        assert_eq!(EVIDENCE_CAMERA_PODS, 3);
        assert_eq!(EVIDENCE_LED_SEGMENTS, CASSETTE_SLOTS);
        assert_eq!(ROBOT_DATUM_PINS, 4);
        assert_eq!(ROBOT_SERVICE_FIDUCIALS, CASSETTE_SLOTS);
        assert_eq!(ROBOT_TOOL_DOCK_DATUMS, 3);
        assert!(BRIDGE_SPAN_X > BUBBLE_PANEL_X);
        assert!(BRIDGE_POST_Z > CAMERA_CLEARANCE_Z);
    }

    #[test]
    fn station_modules_fit_on_deck() {
        for footprint in module_footprints() {
            assert!(fits_inside_station(footprint));
        }
        assert!(PRIME_POS.1 > BUBBLE_POS.1);
        assert!(CUSTODY_POS.0 < SPLIT_POS.0);
        assert!(GATE_POS.0 > SPLIT_POS.0);
        assert!(PURGE_POS.0 > TAP_POS.0);
    }

    #[test]
    fn limitation_markers_prevent_protocol_scope_creep() {
        assert!(LIMITATIONS.contains(&"mechanical_quarantine_fixture_only"));
        assert!(LIMITATIONS.contains(&"no_cell_processing_protocol"));
        assert!(LIMITATIONS.contains(&"no_release_acceptance_thresholds"));
        assert!(LIMITATIONS.contains(&"not_pressure_rated_hardware"));
        assert!(LIMITATIONS.contains(&"purchased_wetted_components_external"));
    }
}
