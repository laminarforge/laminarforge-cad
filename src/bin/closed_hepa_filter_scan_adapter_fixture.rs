use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed clean-cabinet HEPA/filter leak-scan adapter fixture.
//
// Intent:
// - Datum a service-removable HEPA/filter cassette without opening the closed
//   cabinet process volume.
// - Present repeatable scan-wand rails, aerosol challenge placeholders,
//   upstream/downstream pressure taps, gasket witness lands, and evidence
//   capture features for leak-scan validation.
// - Keep pass/fail tokens, barcode/camera evidence capture, robot approach,
//   aerosol cart, filter pull, and service hand clearances visible as CAD
//   geometry. Purchased filters, probes, aerosol generators, sensors, and
//   validation procedures are external to this fixture.

const OUTPUTS: [&str; 10] = [
    "output/closed_hepa_filter_scan_adapter_fixture_cleanable_base_deck.stl",
    "output/closed_hepa_filter_scan_adapter_fixture_filter_cassette_datum.stl",
    "output/closed_hepa_filter_scan_adapter_fixture_scan_wand_rails.stl",
    "output/closed_hepa_filter_scan_adapter_fixture_aerosol_challenge_ports.stl",
    "output/closed_hepa_filter_scan_adapter_fixture_pressure_tap_bulkheads.stl",
    "output/closed_hepa_filter_scan_adapter_fixture_gasket_witness_lands.stl",
    "output/closed_hepa_filter_scan_adapter_fixture_pass_fail_tag_pockets.stl",
    "output/closed_hepa_filter_scan_adapter_fixture_barcode_evidence_camera_bridge.stl",
    "output/closed_hepa_filter_scan_adapter_fixture_robot_service_keepouts.stl",
    "output/closed_hepa_filter_scan_adapter_fixture_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 8] = [
    "filter_cassette_datum",
    "scan_wand_rails",
    "aerosol_challenge_port_placeholders",
    "upstream_downstream_pressure_taps",
    "gasket_witness_lands",
    "pass_fail_tag_pockets",
    "barcode_evidence_camera_bridge",
    "robot_service_keepouts",
];

const DECK_X: f64 = 1280.0;
const DECK_Y: f64 = 780.0;
const DECK_Z: f64 = 22.0;
const DECK_RIM_W: f64 = 16.0;
const DECK_RIM_Z: f64 = 24.0;
const MOUNT_HOLE_D: f64 = 6.6;

const FILTER_X: f64 = 610.0;
const FILTER_Y: f64 = 305.0;
const FILTER_Z: f64 = 96.0;
const FILTER_CENTER: (f64, f64) = (-218.0, 28.0);
const FILTER_CLEARANCE: f64 = 8.0;

const DATUM_X: f64 = FILTER_X + 128.0;
const DATUM_Y: f64 = FILTER_Y + 116.0;
const DATUM_Z: f64 = 34.0;
const DATUM_RAIL_W: f64 = 24.0;
const DATUM_RAIL_Z: f64 = 38.0;
const LOCATOR_COUNT: usize = 6;
const CLAMP_COUNT: usize = 8;

const SCAN_RAIL_X: f64 = FILTER_X + 184.0;
const SCAN_RAIL_SPACING_Y: f64 = FILTER_Y + 132.0;
const SCAN_RAIL_W: f64 = 18.0;
const SCAN_RAIL_Z: f64 = 24.0;
const SCAN_RAIL_POST_Z: f64 = 108.0;
const WAND_STANDOFF_Z: f64 = 25.0;
const SCAN_PASS_COUNT: usize = 9;

const PORT_PANEL_CENTER: (f64, f64) = (352.0, 244.0);
const PORT_PANEL_X: f64 = 430.0;
const PORT_PANEL_Y: f64 = 34.0;
const PORT_PANEL_Z: f64 = 174.0;
const AEROSOL_PORTS: usize = 4;
const AEROSOL_PORT_PITCH: f64 = 78.0;
const AEROSOL_PORT_D: f64 = 19.0;

const PRESSURE_PANEL_CENTER: (f64, f64) = (348.0, -182.0);
const PRESSURE_PANEL_X: f64 = 438.0;
const PRESSURE_PANEL_Y: f64 = 36.0;
const PRESSURE_PANEL_Z: f64 = 154.0;
const PRESSURE_TAPS_PER_SIDE: usize = 5;
const PRESSURE_TAP_PITCH: f64 = 66.0;
const PRESSURE_TAP_D: f64 = 7.2;

const WITNESS_LAND_Z: f64 = 5.0;
const WITNESS_STRIP_W: f64 = 18.0;
const WITNESS_DISC_COUNT: usize = 12;
const GASKET_COMPRESSION_MIN: f64 = 3.0;
const GASKET_COMPRESSION_MAX: f64 = 5.0;

const TAG_PANEL_CENTER: (f64, f64) = (396.0, 34.0);
const TAG_PANEL_X: f64 = 332.0;
const TAG_PANEL_Y: f64 = 214.0;
const TAG_PANEL_Z: f64 = 28.0;
const TAG_POCKET_COUNT: usize = 6;
const TAG_POCKET_X: f64 = 84.0;
const TAG_POCKET_Y: f64 = 42.0;
const TAG_POCKET_DEPTH: f64 = 8.0;
const PASS_FAIL_SEGREGATION_MIN: f64 = 44.0;

const CAMERA_BRIDGE_SPAN_X: f64 = FILTER_X + 256.0;
const CAMERA_BRIDGE_POST_X: f64 = 30.0;
const CAMERA_BRIDGE_POST_Y: f64 = 56.0;
const CAMERA_BRIDGE_UNDERSIDE_Z: f64 = 214.0;
const CAMERA_BRIDGE_BEAM_Z: f64 = 28.0;
const CAMERA_COUNT: usize = 3;
const LED_SEGMENTS: usize = 8;

const FRONT_ROBOT_KEEP_OUT_Y: f64 = 186.0;
const REAR_SERVICE_KEEP_OUT_Y: f64 = 264.0;
const RIGHT_AEROSOL_CART_KEEP_OUT_X: f64 = 278.0;
const FILTER_PULL_KEEP_OUT_Z: f64 = 360.0;
const ROBOT_Z_CLEARANCE: f64 = 330.0;
const KEEP_OUT_RAIL: f64 = 10.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let deck = cleanable_base_deck();
    export(OUTPUTS[0], &deck);

    let datum = filter_cassette_datum();
    export(OUTPUTS[1], &datum);

    let rails = scan_wand_rails();
    export(OUTPUTS[2], &rails);

    let challenge_ports = aerosol_challenge_ports();
    export(OUTPUTS[3], &challenge_ports);

    let pressure_taps = pressure_tap_bulkheads();
    export(OUTPUTS[4], &pressure_taps);

    let witness_lands = gasket_witness_lands();
    export(OUTPUTS[5], &witness_lands);

    let tag_pockets = pass_fail_tag_pockets();
    export(OUTPUTS[6], &tag_pockets);

    let evidence_bridge = barcode_evidence_camera_bridge();
    export(OUTPUTS[7], &evidence_bridge);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[8], &keepouts);

    let assembly = deck
        + datum
        + rails
        + challenge_ports
        + pressure_taps
        + witness_lands
        + tag_pockets
        + evidence_bridge
        + keepouts;
    export(OUTPUTS[9], &assembly);

    println!();
    println!("Closed HEPA/filter leak-scan adapter fixture:");
    println!("  Cleanable deck:              {DECK_X:.0}mm x {DECK_Y:.0}mm x {DECK_Z:.0}mm");
    println!(
        "  Filter cassette datum:       {FILTER_X:.0}mm x {FILTER_Y:.0}mm x {FILTER_Z:.0}mm cassette envelope on {DATUM_X:.0}mm x {DATUM_Y:.0}mm nest"
    );
    println!(
        "  Scan wand rails:             {SCAN_RAIL_X:.0}mm travel, {SCAN_RAIL_SPACING_Y:.0}mm rail spacing, {WAND_STANDOFF_Z:.0}mm nominal probe standoff, {SCAN_PASS_COUNT} indexed scan passes"
    );
    println!(
        "  Challenge/tap placeholders:  {AEROSOL_PORTS} aerosol challenge ports plus {} upstream/downstream pressure taps",
        PRESSURE_TAPS_PER_SIDE * 2
    );
    println!(
        "  Gasket controls:             {WITNESS_DISC_COUNT} witness discs and {:.0}-{:.0}mm compression gauge lands",
        GASKET_COMPRESSION_MIN, GASKET_COMPRESSION_MAX
    );
    println!(
        "  Evidence controls:           {TAG_POCKET_COUNT} pass/fail tag pockets, {CAMERA_COUNT} camera placeholders, {LED_SEGMENTS} LED evidence segments"
    );
    println!("  Required feature groups:     {}", REQUIRED_FEATURES.len());
    println!(
        "  Keepouts:                    {:.0}mm front robot, {:.0}mm rear service, {:.0}mm right aerosol cart, {:.0}mm Z filter-pull gauge",
        FRONT_ROBOT_KEEP_OUT_Y,
        REAR_SERVICE_KEEP_OUT_Y,
        RIGHT_AEROSOL_CART_KEEP_OUT_X,
        FILTER_PULL_KEEP_OUT_Z
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn cleanable_base_deck() -> Part {
    let deck = centered_cube(
        "closed_hepa_filter_scan_adapter_cleanable_base_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    );
    let datum_socket = top_recess(
        "closed_hepa_filter_scan_adapter_filter_datum_socket",
        FILTER_CENTER,
        DATUM_X + 30.0,
        DATUM_Y + 28.0,
        6.0,
    );
    let tag_socket = top_recess(
        "closed_hepa_filter_scan_adapter_pass_fail_tag_socket",
        TAG_PANEL_CENTER,
        TAG_PANEL_X + 22.0,
        TAG_PANEL_Y + 22.0,
        5.0,
    );
    let pressure_socket = top_recess(
        "closed_hepa_filter_scan_adapter_pressure_panel_socket",
        PRESSURE_PANEL_CENTER,
        PRESSURE_PANEL_X + 26.0,
        52.0,
        5.0,
    );
    let port_socket = top_recess(
        "closed_hepa_filter_scan_adapter_challenge_port_panel_socket",
        PORT_PANEL_CENTER,
        PORT_PANEL_X + 28.0,
        52.0,
        5.0,
    );
    let wipe_gutters = wipe_gutters();

    deck - datum_socket
        - tag_socket
        - pressure_socket
        - port_socket
        - wipe_gutters
        - mounting_slots()
        + perimeter_lips()
        + rail_mount_pads()
        + base_robot_fiducials()
}

fn top_recess(name: &str, center: (f64, f64), x: f64, y: f64, depth: f64) -> Part {
    centered_cube(name, x, y, depth + 0.2).translate(
        center.0,
        center.1,
        DECK_Z / 2.0 - depth / 2.0 + 0.1,
    )
}

fn wipe_gutters() -> Part {
    let left = centered_cube(
        "closed_hepa_filter_scan_adapter_left_wipe_gutter",
        14.0,
        DECK_Y - 130.0,
        6.0,
    )
    .translate(-DECK_X / 2.0 + 78.0, 0.0, DECK_Z / 2.0 - 2.4);
    let front = centered_cube(
        "closed_hepa_filter_scan_adapter_front_wipe_gutter",
        DECK_X - 190.0,
        14.0,
        6.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 78.0, DECK_Z / 2.0 - 2.4);
    let port_sump = centered_cube(
        "closed_hepa_filter_scan_adapter_right_port_sump",
        22.0,
        DECK_Y - 210.0,
        7.0,
    )
    .translate(188.0, 6.0, DECK_Z / 2.0 - 2.7);
    let drain = centered_cylinder(
        "closed_hepa_filter_scan_adapter_wipe_gutter_drain",
        6.0,
        DECK_Z + 4.0,
        28,
    )
    .translate(DECK_X / 2.0 - 78.0, -DECK_Y / 2.0 + 78.0, 0.0);

    left + front + port_sump + drain
}

fn perimeter_lips() -> Part {
    let rear = centered_cube(
        "closed_hepa_filter_scan_adapter_rear_cleanable_lip",
        DECK_X - 122.0,
        DECK_RIM_W,
        DECK_RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - 38.0, deck_insert_z(DECK_RIM_Z));
    let left = centered_cube(
        "closed_hepa_filter_scan_adapter_left_cleanable_lip",
        DECK_RIM_W,
        DECK_Y - 132.0,
        DECK_RIM_Z,
    )
    .translate(-DECK_X / 2.0 + 38.0, 0.0, deck_insert_z(DECK_RIM_Z));
    let front_low = centered_cube(
        "closed_hepa_filter_scan_adapter_front_low_retaining_lip",
        DECK_X - 250.0,
        12.0,
        14.0,
    )
    .translate(18.0, -DECK_Y / 2.0 + 38.0, deck_insert_z(14.0));
    let right_short = centered_cube(
        "closed_hepa_filter_scan_adapter_right_service_stop_lip",
        DECK_RIM_W,
        DECK_Y - 290.0,
        18.0,
    )
    .translate(DECK_X / 2.0 - 38.0, -44.0, deck_insert_z(18.0));

    rear + left + front_low + right_short
}

fn rail_mount_pads() -> Part {
    let mut pads = Part::empty("closed_hepa_filter_scan_adapter_scan_rail_mount_pads");
    for (i, (x, y)) in scan_post_points().iter().enumerate() {
        let pad = centered_cube(
            format!("closed_hepa_filter_scan_adapter_scan_rail_mount_pad_{i}"),
            72.0,
            50.0,
            8.0,
        )
        .translate(*x, *y, DECK_Z / 2.0 + 4.0);
        let bolt = centered_cylinder(
            format!("closed_hepa_filter_scan_adapter_scan_rail_pad_bolt_clearance_{i}"),
            3.3,
            12.0,
            22,
        )
        .translate(*x, *y, DECK_Z / 2.0 + 4.0);
        pads = pads + (pad - bolt);
    }
    pads
}

fn mounting_slots() -> Part {
    let mut slots = Part::empty("closed_hepa_filter_scan_adapter_mounting_slots");
    for (i, (x, y)) in deck_mount_points().iter().enumerate() {
        slots = slots
            + centered_cylinder(
                format!("closed_hepa_filter_scan_adapter_m6_mount_round_{i}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 6.0,
                24,
            )
            .translate(*x, *y, 0.0)
            + centered_cube(
                format!("closed_hepa_filter_scan_adapter_m6_mount_slot_{i}"),
                24.0,
                MOUNT_HOLE_D + 0.7,
                DECK_Z + 6.0,
            )
            .translate(*x, *y, 0.0);
    }
    slots
}

fn deck_mount_points() -> [(f64, f64); 8] {
    [
        (-(DECK_X / 2.0 - 68.0), -(DECK_Y / 2.0 - 66.0)),
        (DECK_X / 2.0 - 68.0, -(DECK_Y / 2.0 - 66.0)),
        (-(DECK_X / 2.0 - 68.0), DECK_Y / 2.0 - 66.0),
        (DECK_X / 2.0 - 68.0, DECK_Y / 2.0 - 66.0),
        (FILTER_CENTER.0 - DATUM_X / 2.0 + 72.0, FILTER_CENTER.1),
        (FILTER_CENTER.0 + DATUM_X / 2.0 - 72.0, FILTER_CENTER.1),
        (PRESSURE_PANEL_CENTER.0, PRESSURE_PANEL_CENTER.1 - 58.0),
        (PORT_PANEL_CENTER.0, PORT_PANEL_CENTER.1 + 58.0),
    ]
}

fn base_robot_fiducials() -> Part {
    let mut fiducials = Part::empty("closed_hepa_filter_scan_adapter_base_robot_fiducials");
    for (i, (x, y)) in [
        (-(DECK_X / 2.0 - 90.0), -(DECK_Y / 2.0 - 92.0)),
        (DECK_X / 2.0 - 90.0, -(DECK_Y / 2.0 - 92.0)),
        (-(DECK_X / 2.0 - 90.0), DECK_Y / 2.0 - 92.0),
        (DECK_X / 2.0 - 90.0, DECK_Y / 2.0 - 92.0),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + fiducial_target(&format!(
                "closed_hepa_filter_scan_adapter_deck_robot_fiducial_{i}"
            ))
            .translate(*x, *y, DECK_Z / 2.0 + 2.0);
    }
    fiducials
}

fn filter_cassette_datum() -> Part {
    let tray = centered_cube(
        "closed_hepa_filter_scan_adapter_filter_cassette_outer_datum_tray",
        DATUM_X,
        DATUM_Y,
        DATUM_Z,
    );
    let cassette_recess = centered_cube(
        "closed_hepa_filter_scan_adapter_filter_cassette_clearance_recess",
        FILTER_X + FILTER_CLEARANCE,
        FILTER_Y + FILTER_CLEARANCE,
        DATUM_Z + 4.0,
    )
    .translate(0.0, 0.0, 7.0);
    let gasket_shadow = centered_cube(
        "closed_hepa_filter_scan_adapter_filter_gasket_shadow_channel",
        FILTER_X + 44.0,
        FILTER_Y + 44.0,
        7.0,
    )
    .translate(0.0, 0.0, DATUM_Z / 2.0 - 2.0);

    let body = tray - cassette_recess - gasket_shadow;
    let rails = cassette_datum_rails();
    let locators = cassette_locator_bosses();
    let clamps = cassette_clamps();
    let envelope = clearance_frame(
        "closed_hepa_filter_scan_adapter_hepa_filter_cassette_envelope",
        FILTER_X,
        FILTER_Y,
        FILTER_Z,
    )
    .translate(0.0, 0.0, DATUM_Z / 2.0 + FILTER_Z / 2.0 + 10.0);

    (body + rails + locators + clamps + envelope + cassette_pull_handles()).translate(
        FILTER_CENTER.0,
        FILTER_CENTER.1,
        deck_insert_z(DATUM_Z) + 2.0,
    )
}

fn cassette_datum_rails() -> Part {
    let left = centered_cube(
        "closed_hepa_filter_scan_adapter_left_filter_hard_datum_rail",
        DATUM_RAIL_W,
        FILTER_Y + 92.0,
        DATUM_RAIL_Z,
    )
    .translate(
        -(FILTER_X / 2.0 + 43.0),
        0.0,
        DATUM_Z / 2.0 + DATUM_RAIL_Z / 2.0,
    );
    let right = centered_cube(
        "closed_hepa_filter_scan_adapter_right_filter_spring_datum_rail",
        DATUM_RAIL_W,
        FILTER_Y + 92.0,
        DATUM_RAIL_Z,
    )
    .translate(
        FILTER_X / 2.0 + 43.0,
        0.0,
        DATUM_Z / 2.0 + DATUM_RAIL_Z / 2.0,
    );
    let rear = centered_cube(
        "closed_hepa_filter_scan_adapter_rear_filter_a_datum_stop",
        FILTER_X + 94.0,
        24.0,
        42.0,
    )
    .translate(0.0, FILTER_Y / 2.0 + 43.0, DATUM_Z / 2.0 + 21.0);
    let front_left = centered_cube(
        "closed_hepa_filter_scan_adapter_front_left_filter_pull_stop",
        FILTER_X / 2.0 - 36.0,
        18.0,
        30.0,
    )
    .translate(
        -(FILTER_X / 4.0 + 24.0),
        -(FILTER_Y / 2.0 + 34.0),
        DATUM_Z / 2.0 + 15.0,
    );
    let front_right = centered_cube(
        "closed_hepa_filter_scan_adapter_front_right_filter_pull_stop",
        FILTER_X / 2.0 - 36.0,
        18.0,
        30.0,
    )
    .translate(
        FILTER_X / 4.0 + 24.0,
        -(FILTER_Y / 2.0 + 34.0),
        DATUM_Z / 2.0 + 15.0,
    );

    left + right + rear + front_left + front_right
}

fn cassette_locator_bosses() -> Part {
    let mut locators = Part::empty("closed_hepa_filter_scan_adapter_cassette_locator_bosses");
    for (i, (x, y)) in cassette_locator_points().iter().enumerate() {
        let boss = centered_cylinder(
            format!("closed_hepa_filter_scan_adapter_cassette_locator_boss_{i}"),
            15.0,
            12.0,
            36,
        )
        .translate(*x, *y, DATUM_Z / 2.0 + 6.0);
        let dowel = centered_cylinder(
            format!("closed_hepa_filter_scan_adapter_cassette_dowel_socket_{i}"),
            3.2,
            14.0,
            24,
        )
        .translate(*x, *y, DATUM_Z / 2.0 + 6.0);
        locators = locators + (boss - dowel);
    }
    locators
}

fn cassette_locator_points() -> [(f64, f64); LOCATOR_COUNT] {
    [
        (-(FILTER_X / 2.0 - 48.0), -(FILTER_Y / 2.0 - 42.0)),
        (FILTER_X / 2.0 - 48.0, -(FILTER_Y / 2.0 - 42.0)),
        (-(FILTER_X / 2.0 - 48.0), FILTER_Y / 2.0 - 42.0),
        (FILTER_X / 2.0 - 48.0, FILTER_Y / 2.0 - 42.0),
        (0.0, -(FILTER_Y / 2.0 - 42.0)),
        (0.0, FILTER_Y / 2.0 - 42.0),
    ]
}

fn cassette_clamps() -> Part {
    let mut clamps = Part::empty("closed_hepa_filter_scan_adapter_cassette_toggle_clamps");
    for (i, (x, y, rot)) in cassette_clamp_points().iter().enumerate() {
        let pad = centered_cube(
            format!("closed_hepa_filter_scan_adapter_cassette_clamp_pad_{i}"),
            62.0,
            26.0,
            18.0,
        )
        .rotate(0.0, 0.0, *rot)
        .translate(*x, *y, DATUM_Z / 2.0 + 30.0);
        let hinge = centered_cylinder(
            format!("closed_hepa_filter_scan_adapter_cassette_clamp_hinge_{i}"),
            5.5,
            42.0,
            24,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(*x, *y, DATUM_Z / 2.0 + 17.0);
        clamps = clamps + pad + hinge;
    }
    clamps
}

fn cassette_clamp_points() -> [(f64, f64, f64); CLAMP_COUNT] {
    [
        (-(FILTER_X / 2.0 + 55.0), -112.0, 0.0),
        (-(FILTER_X / 2.0 + 55.0), 0.0, 0.0),
        (-(FILTER_X / 2.0 + 55.0), 112.0, 0.0),
        (FILTER_X / 2.0 + 55.0, -112.0, 0.0),
        (FILTER_X / 2.0 + 55.0, 0.0, 0.0),
        (FILTER_X / 2.0 + 55.0, 112.0, 0.0),
        (-180.0, FILTER_Y / 2.0 + 58.0, 90.0),
        (180.0, FILTER_Y / 2.0 + 58.0, 90.0),
    ]
}

fn cassette_pull_handles() -> Part {
    let left = centered_cube(
        "closed_hepa_filter_scan_adapter_left_filter_pull_handle_shadow",
        78.0,
        18.0,
        28.0,
    )
    .translate(-178.0, -(FILTER_Y / 2.0 + 72.0), DATUM_Z / 2.0 + 34.0);
    let right = centered_cube(
        "closed_hepa_filter_scan_adapter_right_filter_pull_handle_shadow",
        78.0,
        18.0,
        28.0,
    )
    .translate(178.0, -(FILTER_Y / 2.0 + 72.0), DATUM_Z / 2.0 + 34.0);
    left + right
}

fn scan_wand_rails() -> Part {
    let left_rail = centered_cube(
        "closed_hepa_filter_scan_adapter_left_scan_wand_rail",
        SCAN_RAIL_X,
        SCAN_RAIL_W,
        SCAN_RAIL_Z,
    )
    .translate(
        FILTER_CENTER.0,
        FILTER_CENTER.1 - SCAN_RAIL_SPACING_Y / 2.0,
        deck_insert_z(SCAN_RAIL_Z) + SCAN_RAIL_POST_Z,
    );
    let right_rail = centered_cube(
        "closed_hepa_filter_scan_adapter_right_scan_wand_rail",
        SCAN_RAIL_X,
        SCAN_RAIL_W,
        SCAN_RAIL_Z,
    )
    .translate(
        FILTER_CENTER.0,
        FILTER_CENTER.1 + SCAN_RAIL_SPACING_Y / 2.0,
        deck_insert_z(SCAN_RAIL_Z) + SCAN_RAIL_POST_Z,
    );
    let travel_beam = centered_cube(
        "closed_hepa_filter_scan_adapter_scan_wand_cross_sled",
        86.0,
        SCAN_RAIL_SPACING_Y + 74.0,
        20.0,
    )
    .translate(
        FILTER_CENTER.0 - 44.0,
        FILTER_CENTER.1,
        deck_insert_z(20.0) + SCAN_RAIL_POST_Z + 16.0,
    );
    let wand_socket = centered_cylinder(
        "closed_hepa_filter_scan_adapter_scan_wand_probe_socket",
        12.0,
        100.0,
        36,
    )
    .translate(
        FILTER_CENTER.0 - 44.0,
        FILTER_CENTER.1,
        deck_insert_z(100.0) + WAND_STANDOFF_Z + FILTER_Z,
    );
    let standoff_gauge = centered_cube(
        "closed_hepa_filter_scan_adapter_scan_wand_25mm_standoff_gauge",
        FILTER_X + 22.0,
        18.0,
        6.0,
    )
    .translate(
        FILTER_CENTER.0,
        FILTER_CENTER.1,
        deck_insert_z(6.0) + DATUM_Z + FILTER_Z + WAND_STANDOFF_Z,
    );

    left_rail + right_rail + scan_rail_posts() + travel_beam + wand_socket + standoff_gauge
        - scan_index_notches()
        + scan_pass_index_flags()
}

fn scan_rail_posts() -> Part {
    let mut posts = Part::empty("closed_hepa_filter_scan_adapter_scan_rail_posts");
    for (i, (x, y)) in scan_post_points().iter().enumerate() {
        posts = posts
            + centered_cube(
                format!("closed_hepa_filter_scan_adapter_scan_rail_post_{i}"),
                32.0,
                28.0,
                SCAN_RAIL_POST_Z,
            )
            .translate(*x, *y, DECK_Z / 2.0 + SCAN_RAIL_POST_Z / 2.0 + 8.0);
    }
    posts
}

fn scan_post_points() -> [(f64, f64); 8] {
    [
        (
            FILTER_CENTER.0 - SCAN_RAIL_X / 2.0 + 46.0,
            FILTER_CENTER.1 - SCAN_RAIL_SPACING_Y / 2.0,
        ),
        (FILTER_CENTER.0, FILTER_CENTER.1 - SCAN_RAIL_SPACING_Y / 2.0),
        (
            FILTER_CENTER.0 + SCAN_RAIL_X / 2.0 - 46.0,
            FILTER_CENTER.1 - SCAN_RAIL_SPACING_Y / 2.0,
        ),
        (
            FILTER_CENTER.0 - SCAN_RAIL_X / 2.0 + 46.0,
            FILTER_CENTER.1 + SCAN_RAIL_SPACING_Y / 2.0,
        ),
        (FILTER_CENTER.0, FILTER_CENTER.1 + SCAN_RAIL_SPACING_Y / 2.0),
        (
            FILTER_CENTER.0 + SCAN_RAIL_X / 2.0 - 46.0,
            FILTER_CENTER.1 + SCAN_RAIL_SPACING_Y / 2.0,
        ),
        (FILTER_CENTER.0 - SCAN_RAIL_X / 2.0 + 46.0, FILTER_CENTER.1),
        (FILTER_CENTER.0 + SCAN_RAIL_X / 2.0 - 46.0, FILTER_CENTER.1),
    ]
}

fn scan_index_notches() -> Part {
    let mut notches = Part::empty("closed_hepa_filter_scan_adapter_scan_index_notches");
    for pass in 0..SCAN_PASS_COUNT {
        let y = FILTER_CENTER.1 + centered_index(pass, SCAN_PASS_COUNT, FILTER_Y / 8.0);
        notches = notches
            + centered_cube(
                format!("closed_hepa_filter_scan_adapter_scan_pass_notch_{pass}"),
                28.0,
                7.0,
                SCAN_RAIL_Z + 4.0,
            )
            .translate(
                FILTER_CENTER.0 - SCAN_RAIL_X / 2.0 + 96.0,
                y,
                deck_insert_z(SCAN_RAIL_Z) + SCAN_RAIL_POST_Z,
            );
    }
    notches
}

fn scan_pass_index_flags() -> Part {
    let mut flags = Part::empty("closed_hepa_filter_scan_adapter_scan_pass_index_flags");
    for pass in 0..SCAN_PASS_COUNT {
        let y = FILTER_CENTER.1 + centered_index(pass, SCAN_PASS_COUNT, FILTER_Y / 8.0);
        flags = flags
            + centered_cube(
                format!("closed_hepa_filter_scan_adapter_scan_pass_label_land_{pass}"),
                42.0,
                6.0,
                16.0,
            )
            .translate(
                FILTER_CENTER.0 + SCAN_RAIL_X / 2.0 - 90.0,
                y,
                deck_insert_z(16.0) + SCAN_RAIL_POST_Z + 28.0,
            );
    }
    flags
}

fn aerosol_challenge_ports() -> Part {
    let panel = centered_cube(
        "closed_hepa_filter_scan_adapter_aerosol_challenge_port_panel",
        PORT_PANEL_X,
        PORT_PANEL_Y,
        PORT_PANEL_Z,
    )
    .translate(
        PORT_PANEL_CENTER.0,
        PORT_PANEL_CENTER.1,
        deck_insert_z(PORT_PANEL_Z),
    );
    let plenum_shadow = centered_cube(
        "closed_hepa_filter_scan_adapter_upstream_plenum_shadow",
        PORT_PANEL_X - 84.0,
        34.0,
        52.0,
    )
    .translate(
        PORT_PANEL_CENTER.0,
        PORT_PANEL_CENTER.1 - 40.0,
        deck_insert_z(52.0) + 52.0,
    );
    let challenge_holes = port_holes(
        "aerosol_challenge_port",
        AEROSOL_PORTS,
        AEROSOL_PORT_PITCH,
        AEROSOL_PORT_D,
        PORT_PANEL_CENTER,
        deck_insert_z(PORT_PANEL_Z) + 26.0,
        PORT_PANEL_Y,
    );
    let collars = port_collars(
        "aerosol_challenge_tri_clamp_placeholder",
        AEROSOL_PORTS,
        AEROSOL_PORT_PITCH,
        44.0,
        AEROSOL_PORT_D,
        PORT_PANEL_CENTER,
        deck_insert_z(PORT_PANEL_Z) + 26.0,
        PORT_PANEL_Y,
    );
    let caps = aerosol_port_caps();
    let label_strip = centered_cube(
        "closed_hepa_filter_scan_adapter_aerosol_lot_label_strip",
        PORT_PANEL_X - 76.0,
        8.0,
        18.0,
    )
    .translate(
        PORT_PANEL_CENTER.0,
        PORT_PANEL_CENTER.1 - PORT_PANEL_Y / 2.0 - 11.0,
        deck_insert_z(PORT_PANEL_Z) + 72.0,
    );

    panel - challenge_holes + collars + caps + plenum_shadow + label_strip
}

fn aerosol_port_caps() -> Part {
    let mut caps = Part::empty("closed_hepa_filter_scan_adapter_aerosol_port_blank_caps");
    for i in 0..AEROSOL_PORTS {
        let x = PORT_PANEL_CENTER.0 + centered_index(i, AEROSOL_PORTS, AEROSOL_PORT_PITCH);
        caps = caps
            + centered_cylinder(
                format!("closed_hepa_filter_scan_adapter_aerosol_port_blank_cap_{i}"),
                18.0,
                14.0,
                36,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                x,
                PORT_PANEL_CENTER.1 - PORT_PANEL_Y / 2.0 - 14.0,
                deck_insert_z(PORT_PANEL_Z) + 26.0,
            );
    }
    caps
}

fn pressure_tap_bulkheads() -> Part {
    let panel = centered_cube(
        "closed_hepa_filter_scan_adapter_pressure_tap_bulkhead_panel",
        PRESSURE_PANEL_X,
        PRESSURE_PANEL_Y,
        PRESSURE_PANEL_Z,
    )
    .translate(
        PRESSURE_PANEL_CENTER.0,
        PRESSURE_PANEL_CENTER.1,
        deck_insert_z(PRESSURE_PANEL_Z),
    );
    let upstream_holes = port_holes(
        "upstream_pressure_tap",
        PRESSURE_TAPS_PER_SIDE,
        PRESSURE_TAP_PITCH,
        PRESSURE_TAP_D,
        PRESSURE_PANEL_CENTER,
        deck_insert_z(PRESSURE_PANEL_Z) + 32.0,
        PRESSURE_PANEL_Y,
    );
    let downstream_holes = port_holes(
        "downstream_pressure_tap",
        PRESSURE_TAPS_PER_SIDE,
        PRESSURE_TAP_PITCH,
        PRESSURE_TAP_D,
        PRESSURE_PANEL_CENTER,
        deck_insert_z(PRESSURE_PANEL_Z) - 30.0,
        PRESSURE_PANEL_Y,
    );
    let upstream_collars = port_collars(
        "upstream_pressure_tap_luer_land",
        PRESSURE_TAPS_PER_SIDE,
        PRESSURE_TAP_PITCH,
        28.0,
        PRESSURE_TAP_D,
        PRESSURE_PANEL_CENTER,
        deck_insert_z(PRESSURE_PANEL_Z) + 32.0,
        PRESSURE_PANEL_Y,
    );
    let downstream_collars = port_collars(
        "downstream_pressure_tap_luer_land",
        PRESSURE_TAPS_PER_SIDE,
        PRESSURE_TAP_PITCH,
        28.0,
        PRESSURE_TAP_D,
        PRESSURE_PANEL_CENTER,
        deck_insert_z(PRESSURE_PANEL_Z) - 30.0,
        PRESSURE_PANEL_Y,
    );
    let differential_manometer_shadow = centered_cube(
        "closed_hepa_filter_scan_adapter_differential_pressure_manometer_shadow",
        PRESSURE_PANEL_X - 92.0,
        24.0,
        30.0,
    )
    .translate(
        PRESSURE_PANEL_CENTER.0,
        PRESSURE_PANEL_CENTER.1 - 34.0,
        deck_insert_z(PRESSURE_PANEL_Z) + 78.0,
    );
    let baseline_coupon_pocket = centered_cube(
        "closed_hepa_filter_scan_adapter_pressure_baseline_coupon_pocket",
        96.0,
        18.0,
        22.0,
    )
    .translate(
        PRESSURE_PANEL_CENTER.0 + PRESSURE_PANEL_X / 2.0 - 76.0,
        PRESSURE_PANEL_CENTER.1 - 33.0,
        deck_insert_z(PRESSURE_PANEL_Z) - 66.0,
    );

    panel - upstream_holes - downstream_holes
        + upstream_collars
        + downstream_collars
        + differential_manometer_shadow
        + baseline_coupon_pocket
        + pressure_tap_label_lands()
}

fn pressure_tap_label_lands() -> Part {
    let mut lands = Part::empty("closed_hepa_filter_scan_adapter_pressure_tap_label_lands");
    for i in 0..PRESSURE_TAPS_PER_SIDE {
        let x =
            PRESSURE_PANEL_CENTER.0 + centered_index(i, PRESSURE_TAPS_PER_SIDE, PRESSURE_TAP_PITCH);
        lands = lands
            + centered_cube(
                format!("closed_hepa_filter_scan_adapter_upstream_pressure_label_land_{i}"),
                42.0,
                6.0,
                12.0,
            )
            .translate(
                x,
                PRESSURE_PANEL_CENTER.1 - PRESSURE_PANEL_Y / 2.0 - 9.0,
                deck_insert_z(PRESSURE_PANEL_Z) + 52.0,
            )
            + centered_cube(
                format!("closed_hepa_filter_scan_adapter_downstream_pressure_label_land_{i}"),
                42.0,
                6.0,
                12.0,
            )
            .translate(
                x,
                PRESSURE_PANEL_CENTER.1 - PRESSURE_PANEL_Y / 2.0 - 9.0,
                deck_insert_z(PRESSURE_PANEL_Z) - 10.0,
            );
    }
    lands
}

fn port_holes(
    prefix: &str,
    count: usize,
    pitch: f64,
    bore_d: f64,
    center: (f64, f64),
    z: f64,
    panel_y: f64,
) -> Part {
    let mut holes = Part::empty(format!("closed_hepa_filter_scan_adapter_{prefix}_holes"));
    for i in 0..count {
        let x = center.0 + centered_index(i, count, pitch);
        holes = holes
            + centered_cylinder(
                format!("closed_hepa_filter_scan_adapter_{prefix}_hole_{i}"),
                bore_d / 2.0,
                panel_y + 8.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, center.1, z);
    }
    holes
}

fn port_collars(
    prefix: &str,
    count: usize,
    pitch: f64,
    outside_d: f64,
    inside_d: f64,
    center: (f64, f64),
    z: f64,
    panel_y: f64,
) -> Part {
    let mut collars = Part::empty(format!("closed_hepa_filter_scan_adapter_{prefix}_collars"));
    for i in 0..count {
        let x = center.0 + centered_index(i, count, pitch);
        collars = collars
            + port_collar(
                &format!("closed_hepa_filter_scan_adapter_{prefix}_{i}"),
                outside_d,
                inside_d,
                panel_y,
            )
            .translate(x, center.1 - panel_y / 2.0 - 8.0, z);
    }
    collars
}

fn gasket_witness_lands() -> Part {
    let top = centered_cube(
        "closed_hepa_filter_scan_adapter_rear_gasket_witness_land",
        FILTER_X + 58.0,
        WITNESS_STRIP_W,
        WITNESS_LAND_Z,
    )
    .translate(
        FILTER_CENTER.0,
        FILTER_CENTER.1 + FILTER_Y / 2.0 + 28.0,
        deck_insert_z(WITNESS_LAND_Z) + DATUM_Z + 2.0,
    );
    let bottom = centered_cube(
        "closed_hepa_filter_scan_adapter_front_gasket_witness_land",
        FILTER_X + 58.0,
        WITNESS_STRIP_W,
        WITNESS_LAND_Z,
    )
    .translate(
        FILTER_CENTER.0,
        FILTER_CENTER.1 - FILTER_Y / 2.0 - 28.0,
        deck_insert_z(WITNESS_LAND_Z) + DATUM_Z + 2.0,
    );
    let left = centered_cube(
        "closed_hepa_filter_scan_adapter_left_gasket_witness_land",
        WITNESS_STRIP_W,
        FILTER_Y + 46.0,
        WITNESS_LAND_Z,
    )
    .translate(
        FILTER_CENTER.0 - FILTER_X / 2.0 - 28.0,
        FILTER_CENTER.1,
        deck_insert_z(WITNESS_LAND_Z) + DATUM_Z + 2.0,
    );
    let right = centered_cube(
        "closed_hepa_filter_scan_adapter_right_gasket_witness_land",
        WITNESS_STRIP_W,
        FILTER_Y + 46.0,
        WITNESS_LAND_Z,
    )
    .translate(
        FILTER_CENTER.0 + FILTER_X / 2.0 + 28.0,
        FILTER_CENTER.1,
        deck_insert_z(WITNESS_LAND_Z) + DATUM_Z + 2.0,
    );

    top + bottom + left + right + witness_disc_pockets() + compression_gauge_lands()
}

fn witness_disc_pockets() -> Part {
    let mut discs = Part::empty("closed_hepa_filter_scan_adapter_gasket_witness_disc_pockets");
    for i in 0..WITNESS_DISC_COUNT {
        let (x, y) = witness_disc_position(i);
        let boss = centered_cylinder(
            format!("closed_hepa_filter_scan_adapter_witness_disc_land_{i}"),
            11.0,
            4.0,
            36,
        )
        .translate(x, y, deck_insert_z(4.0) + DATUM_Z + 6.0);
        let recess = centered_cylinder(
            format!("closed_hepa_filter_scan_adapter_witness_disc_recess_{i}"),
            7.0,
            5.0,
            30,
        )
        .translate(x, y, deck_insert_z(4.0) + DATUM_Z + 6.0);
        discs = discs + (boss - recess);
    }
    discs
}

fn witness_disc_position(index: usize) -> (f64, f64) {
    let side = index / 3;
    let slot = index % 3;
    let offset = centered_index(slot, 3, 175.0);
    match side {
        0 => (
            FILTER_CENTER.0 + offset,
            FILTER_CENTER.1 + FILTER_Y / 2.0 + 58.0,
        ),
        1 => (
            FILTER_CENTER.0 + offset,
            FILTER_CENTER.1 - FILTER_Y / 2.0 - 58.0,
        ),
        2 => (
            FILTER_CENTER.0 - FILTER_X / 2.0 - 58.0,
            FILTER_CENTER.1 + offset / 1.8,
        ),
        _ => (
            FILTER_CENTER.0 + FILTER_X / 2.0 + 58.0,
            FILTER_CENTER.1 + offset / 1.8,
        ),
    }
}

fn compression_gauge_lands() -> Part {
    let nominal = centered_cube(
        "closed_hepa_filter_scan_adapter_nominal_gasket_compression_gauge_land",
        128.0,
        16.0,
        GASKET_COMPRESSION_MIN,
    )
    .translate(
        FILTER_CENTER.0 - 214.0,
        FILTER_CENTER.1 + FILTER_Y / 2.0 + 86.0,
        DECK_Z / 2.0 + DATUM_Z + GASKET_COMPRESSION_MIN / 2.0 + 8.0,
    );
    let high = centered_cube(
        "closed_hepa_filter_scan_adapter_high_gasket_compression_gauge_land",
        128.0,
        16.0,
        GASKET_COMPRESSION_MAX,
    )
    .translate(
        FILTER_CENTER.0 + 214.0,
        FILTER_CENTER.1 + FILTER_Y / 2.0 + 86.0,
        DECK_Z / 2.0 + DATUM_Z + GASKET_COMPRESSION_MAX / 2.0 + 8.0,
    );
    nominal + high
}

fn pass_fail_tag_pockets() -> Part {
    let panel = centered_cube(
        "closed_hepa_filter_scan_adapter_pass_fail_tag_panel",
        TAG_PANEL_X,
        TAG_PANEL_Y,
        TAG_PANEL_Z,
    )
    .translate(
        TAG_PANEL_CENTER.0,
        TAG_PANEL_CENTER.1,
        deck_insert_z(TAG_PANEL_Z),
    );
    let wash_basin = centered_cube(
        "closed_hepa_filter_scan_adapter_pass_fail_tag_wash_basin",
        TAG_PANEL_X - 36.0,
        TAG_PANEL_Y - 36.0,
        6.0,
    )
    .translate(
        TAG_PANEL_CENTER.0,
        TAG_PANEL_CENTER.1,
        deck_insert_z(TAG_PANEL_Z) - TAG_PANEL_Z / 2.0 + 3.0,
    );
    let divider = centered_cube(
        "closed_hepa_filter_scan_adapter_pass_fail_hard_divider",
        18.0,
        TAG_PANEL_Y - 42.0,
        36.0,
    )
    .translate(
        TAG_PANEL_CENTER.0,
        TAG_PANEL_CENTER.1,
        deck_insert_z(36.0) + TAG_PANEL_Z,
    );
    let pockets = tag_pocket_cuts();
    let label_lands = tag_label_lands();
    let evidence_token_lane = centered_cube(
        "closed_hepa_filter_scan_adapter_evidence_token_chain_lane",
        TAG_PANEL_X - 52.0,
        18.0,
        5.0,
    )
    .translate(
        TAG_PANEL_CENTER.0,
        TAG_PANEL_CENTER.1 - TAG_PANEL_Y / 2.0 + 30.0,
        deck_insert_z(TAG_PANEL_Z) + TAG_PANEL_Z / 2.0 + 2.5,
    );

    panel - wash_basin - pockets + divider + label_lands + evidence_token_lane
}

fn tag_pocket_cuts() -> Part {
    let mut pockets = Part::empty("closed_hepa_filter_scan_adapter_pass_fail_tag_pocket_cuts");
    for i in 0..TAG_POCKET_COUNT {
        let is_pass = i < TAG_POCKET_COUNT / 2;
        let local_index = i % (TAG_POCKET_COUNT / 2);
        let x = if is_pass {
            TAG_PANEL_CENTER.0 - TAG_POCKET_X / 2.0 - PASS_FAIL_SEGREGATION_MIN / 2.0
        } else {
            TAG_PANEL_CENTER.0 + TAG_POCKET_X / 2.0 + PASS_FAIL_SEGREGATION_MIN / 2.0
        };
        let y = TAG_PANEL_CENTER.1 + centered_index(local_index, TAG_POCKET_COUNT / 2, 56.0) + 22.0;
        pockets = pockets
            + centered_cube(
                format!("closed_hepa_filter_scan_adapter_tag_pocket_cut_{i}"),
                TAG_POCKET_X,
                TAG_POCKET_Y,
                TAG_POCKET_DEPTH + 0.2,
            )
            .translate(
                x,
                y,
                deck_insert_z(TAG_PANEL_Z) + TAG_PANEL_Z / 2.0 - TAG_POCKET_DEPTH / 2.0,
            );
    }
    pockets
}

fn tag_label_lands() -> Part {
    let pass = centered_cube(
        "closed_hepa_filter_scan_adapter_pass_tag_barcode_land",
        112.0,
        14.0,
        4.0,
    )
    .translate(
        TAG_PANEL_CENTER.0 - 72.0,
        TAG_PANEL_CENTER.1 + TAG_PANEL_Y / 2.0 - 28.0,
        deck_insert_z(TAG_PANEL_Z) + TAG_PANEL_Z / 2.0 + 2.0,
    );
    let fail = centered_cube(
        "closed_hepa_filter_scan_adapter_fail_tag_barcode_land",
        112.0,
        14.0,
        4.0,
    )
    .translate(
        TAG_PANEL_CENTER.0 + 72.0,
        TAG_PANEL_CENTER.1 + TAG_PANEL_Y / 2.0 - 28.0,
        deck_insert_z(TAG_PANEL_Z) + TAG_PANEL_Z / 2.0 + 2.0,
    );
    let quarantine_flag = centered_cube(
        "closed_hepa_filter_scan_adapter_failed_filter_quarantine_flag",
        112.0,
        10.0,
        26.0,
    )
    .translate(
        TAG_PANEL_CENTER.0 + 72.0,
        TAG_PANEL_CENTER.1 - TAG_PANEL_Y / 2.0 + 52.0,
        deck_insert_z(TAG_PANEL_Z) + 42.0,
    );

    pass + fail + quarantine_flag
}

fn barcode_evidence_camera_bridge() -> Part {
    let left_post = camera_bridge_post("left").translate(
        FILTER_CENTER.0 - CAMERA_BRIDGE_SPAN_X / 2.0,
        FILTER_CENTER.1 - FILTER_Y / 2.0 - 76.0,
        DECK_Z / 2.0 + (CAMERA_BRIDGE_UNDERSIDE_Z + CAMERA_BRIDGE_BEAM_Z) / 2.0,
    );
    let right_post = camera_bridge_post("right").translate(
        FILTER_CENTER.0 + CAMERA_BRIDGE_SPAN_X / 2.0,
        FILTER_CENTER.1 - FILTER_Y / 2.0 - 76.0,
        DECK_Z / 2.0 + (CAMERA_BRIDGE_UNDERSIDE_Z + CAMERA_BRIDGE_BEAM_Z) / 2.0,
    );
    let beam = centered_cube(
        "closed_hepa_filter_scan_adapter_evidence_camera_bridge_beam",
        CAMERA_BRIDGE_SPAN_X + CAMERA_BRIDGE_POST_X,
        CAMERA_BRIDGE_POST_Y,
        CAMERA_BRIDGE_BEAM_Z,
    )
    .translate(
        FILTER_CENTER.0,
        FILTER_CENTER.1 - FILTER_Y / 2.0 - 76.0,
        DECK_Z / 2.0 + CAMERA_BRIDGE_UNDERSIDE_Z + CAMERA_BRIDGE_BEAM_Z / 2.0,
    );
    let barcode_plate = centered_cube(
        "closed_hepa_filter_scan_adapter_filter_cassette_barcode_read_land",
        196.0,
        14.0,
        42.0,
    )
    .translate(
        FILTER_CENTER.0 - FILTER_X / 2.0 + 132.0,
        FILTER_CENTER.1 - FILTER_Y / 2.0 - 102.0,
        DECK_Z / 2.0 + 116.0,
    );

    left_post + right_post + beam + camera_pods() + led_evidence_segments() + barcode_plate
        - camera_cable_passages()
}

fn camera_bridge_post(side: &str) -> Part {
    let post = centered_cube(
        format!("closed_hepa_filter_scan_adapter_{side}_camera_bridge_post"),
        CAMERA_BRIDGE_POST_X,
        CAMERA_BRIDGE_POST_Y,
        CAMERA_BRIDGE_UNDERSIDE_Z + CAMERA_BRIDGE_BEAM_Z,
    );
    let service_slot = centered_cube(
        format!("closed_hepa_filter_scan_adapter_{side}_camera_bridge_service_slot"),
        CAMERA_BRIDGE_POST_X + 4.0,
        14.0,
        110.0,
    )
    .translate(0.0, 0.0, -28.0);
    post - service_slot
}

fn camera_pods() -> Part {
    let mut pods = Part::empty("closed_hepa_filter_scan_adapter_evidence_camera_pods");
    for i in 0..CAMERA_COUNT {
        let x = FILTER_CENTER.0 + centered_index(i, CAMERA_COUNT, 236.0);
        let pod = centered_cube(
            format!("closed_hepa_filter_scan_adapter_camera_pod_{i}"),
            88.0,
            38.0,
            34.0,
        )
        .translate(
            x,
            FILTER_CENTER.1 - FILTER_Y / 2.0 - 110.0,
            DECK_Z / 2.0 + CAMERA_BRIDGE_UNDERSIDE_Z - 24.0,
        );
        let lens = centered_cylinder(
            format!("closed_hepa_filter_scan_adapter_camera_lens_shadow_{i}"),
            14.0,
            12.0,
            36,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            x,
            FILTER_CENTER.1 - FILTER_Y / 2.0 - 133.0,
            DECK_Z / 2.0 + CAMERA_BRIDGE_UNDERSIDE_Z - 24.0,
        );
        pods = pods + pod + lens;
    }
    pods
}

fn led_evidence_segments() -> Part {
    let mut leds = Part::empty("closed_hepa_filter_scan_adapter_led_evidence_segments");
    for i in 0..LED_SEGMENTS {
        let x = FILTER_CENTER.0 + centered_index(i, LED_SEGMENTS, 82.0);
        leds = leds
            + centered_cube(
                format!("closed_hepa_filter_scan_adapter_evidence_led_segment_{i}"),
                48.0,
                10.0,
                12.0,
            )
            .translate(
                x,
                FILTER_CENTER.1 - FILTER_Y / 2.0 - 122.0,
                DECK_Z / 2.0 + CAMERA_BRIDGE_UNDERSIDE_Z - 58.0,
            );
    }
    leds
}

fn camera_cable_passages() -> Part {
    let left = centered_cylinder(
        "closed_hepa_filter_scan_adapter_left_camera_cable_passage",
        6.0,
        CAMERA_BRIDGE_POST_X + 10.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(
        FILTER_CENTER.0 - CAMERA_BRIDGE_SPAN_X / 2.0,
        FILTER_CENTER.1 - FILTER_Y / 2.0 - 76.0,
        DECK_Z / 2.0 + CAMERA_BRIDGE_UNDERSIDE_Z - 46.0,
    );
    let right = centered_cylinder(
        "closed_hepa_filter_scan_adapter_right_camera_cable_passage",
        6.0,
        CAMERA_BRIDGE_POST_X + 10.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(
        FILTER_CENTER.0 + CAMERA_BRIDGE_SPAN_X / 2.0,
        FILTER_CENTER.1 - FILTER_Y / 2.0 - 76.0,
        DECK_Z / 2.0 + CAMERA_BRIDGE_UNDERSIDE_Z - 46.0,
    );
    left + right
}

fn robot_service_keepouts() -> Part {
    let front_robot = clearance_frame(
        "closed_hepa_filter_scan_adapter_front_robot_scan_approach_keepout",
        FILTER_X + 320.0,
        FRONT_ROBOT_KEEP_OUT_Y,
        ROBOT_Z_CLEARANCE,
    )
    .translate(
        FILTER_CENTER.0,
        -DECK_Y / 2.0 - FRONT_ROBOT_KEEP_OUT_Y / 2.0 + 30.0,
        DECK_Z / 2.0 + ROBOT_Z_CLEARANCE / 2.0,
    );
    let rear_filter_pull = clearance_frame(
        "closed_hepa_filter_scan_adapter_rear_filter_pull_service_keepout",
        FILTER_X + 220.0,
        REAR_SERVICE_KEEP_OUT_Y,
        FILTER_PULL_KEEP_OUT_Z,
    )
    .translate(
        FILTER_CENTER.0,
        DECK_Y / 2.0 + REAR_SERVICE_KEEP_OUT_Y / 2.0 - 36.0,
        DECK_Z / 2.0 + FILTER_PULL_KEEP_OUT_Z / 2.0,
    );
    let aerosol_cart = clearance_frame(
        "closed_hepa_filter_scan_adapter_right_aerosol_generator_cart_keepout",
        RIGHT_AEROSOL_CART_KEEP_OUT_X,
        DECK_Y - 230.0,
        260.0,
    )
    .translate(
        DECK_X / 2.0 + RIGHT_AEROSOL_CART_KEEP_OUT_X / 2.0 - 42.0,
        0.0,
        DECK_Z / 2.0 + 130.0,
    );
    let wand_sweep = clearance_frame(
        "closed_hepa_filter_scan_adapter_scan_wand_sweep_envelope",
        SCAN_RAIL_X,
        SCAN_RAIL_SPACING_Y,
        92.0,
    )
    .translate(
        FILTER_CENTER.0,
        FILTER_CENTER.1,
        DECK_Z / 2.0 + FILTER_Z + WAND_STANDOFF_Z + 50.0,
    );

    front_robot + rear_filter_pull + aerosol_cart + wand_sweep + keepout_datum_targets()
}

fn keepout_datum_targets() -> Part {
    let mut targets = Part::empty("closed_hepa_filter_scan_adapter_keepout_datum_targets");
    for (i, (x, y)) in [
        (
            FILTER_CENTER.0 - FILTER_X / 2.0,
            FILTER_CENTER.1 - FILTER_Y / 2.0,
        ),
        (
            FILTER_CENTER.0 + FILTER_X / 2.0,
            FILTER_CENTER.1 - FILTER_Y / 2.0,
        ),
        (
            FILTER_CENTER.0 - FILTER_X / 2.0,
            FILTER_CENTER.1 + FILTER_Y / 2.0,
        ),
        (
            FILTER_CENTER.0 + FILTER_X / 2.0,
            FILTER_CENTER.1 + FILTER_Y / 2.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        targets = targets
            + fiducial_target(&format!(
                "closed_hepa_filter_scan_adapter_keepout_corner_target_{i}"
            ))
            .translate(*x, *y, DECK_Z / 2.0 + DATUM_Z + FILTER_Z + 72.0);
    }
    targets
}

fn port_collar(name: &str, outside_d: f64, inside_d: f64, panel_y: f64) -> Part {
    let collar = centered_cylinder(format!("{name}_outer"), outside_d / 2.0, 12.0, 40)
        .rotate(90.0, 0.0, 0.0);
    let bore = centered_cylinder(
        format!("{name}_inner_bore"),
        inside_d / 2.0,
        panel_y + 18.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0);
    collar - bore
}

fn fiducial_target(name: &str) -> Part {
    let plate = centered_cube(format!("{name}_plate"), 34.0, 34.0, 4.0);
    let outer = centered_cylinder(format!("{name}_outer_ring"), 11.0, 5.0, 36);
    let inner = centered_cylinder(format!("{name}_center_mark"), 4.0, 6.0, 28);
    plate + outer - inner
}

fn clearance_frame(name: &str, x: f64, y: f64, z: f64) -> Part {
    let mut frame = Part::empty(format!("{name}_frame"));

    for (i, sx) in [-1.0, 1.0].iter().enumerate() {
        for (j, sy) in [-1.0, 1.0].iter().enumerate() {
            frame = frame
                + centered_cube(
                    format!("{name}_vertical_post_{i}_{j}"),
                    KEEP_OUT_RAIL,
                    KEEP_OUT_RAIL,
                    z,
                )
                .translate(
                    sx * (x / 2.0 - KEEP_OUT_RAIL / 2.0),
                    sy * (y / 2.0 - KEEP_OUT_RAIL / 2.0),
                    0.0,
                );
        }
    }

    for (k, zsign) in [-1.0, 1.0].iter().enumerate() {
        let zpos = zsign * (z / 2.0 - KEEP_OUT_RAIL / 2.0);
        frame = frame
            + centered_cube(
                format!("{name}_front_x_rail_{k}"),
                x,
                KEEP_OUT_RAIL,
                KEEP_OUT_RAIL,
            )
            .translate(0.0, -(y / 2.0 - KEEP_OUT_RAIL / 2.0), zpos)
            + centered_cube(
                format!("{name}_rear_x_rail_{k}"),
                x,
                KEEP_OUT_RAIL,
                KEEP_OUT_RAIL,
            )
            .translate(0.0, y / 2.0 - KEEP_OUT_RAIL / 2.0, zpos)
            + centered_cube(
                format!("{name}_left_y_rail_{k}"),
                KEEP_OUT_RAIL,
                y,
                KEEP_OUT_RAIL,
            )
            .translate(-(x / 2.0 - KEEP_OUT_RAIL / 2.0), 0.0, zpos)
            + centered_cube(
                format!("{name}_right_y_rail_{k}"),
                KEEP_OUT_RAIL,
                y,
                KEEP_OUT_RAIL,
            )
            .translate(x / 2.0 - KEEP_OUT_RAIL / 2.0, 0.0, zpos);
    }

    frame
}

fn deck_insert_z(height: f64) -> f64 {
    DECK_Z / 2.0 + height / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn assert_layout() {
    assert!(
        FILTER_CENTER.0.abs() + DATUM_X / 2.0 < DECK_X / 2.0 - 28.0,
        "filter datum exceeds deck X boundary"
    );
    assert!(
        FILTER_CENTER.1.abs() + DATUM_Y / 2.0 < DECK_Y / 2.0 - 38.0,
        "filter datum exceeds deck Y boundary"
    );
    assert!(
        TAG_PANEL_CENTER.0 + TAG_PANEL_X / 2.0 < DECK_X / 2.0 - 30.0,
        "tag panel exceeds deck right boundary"
    );
    assert!(
        PRESSURE_PANEL_CENTER.0 + PRESSURE_PANEL_X / 2.0 < DECK_X / 2.0 - 22.0,
        "pressure panel exceeds deck right boundary"
    );
    assert!(
        scan_rail_span_margin() >= 80.0,
        "scan rails need at least 80mm travel beyond filter envelope"
    );
    assert!(
        pass_fail_tag_gap() >= PASS_FAIL_SEGREGATION_MIN,
        "pass and fail tag pockets are not segregated enough"
    );
}

fn scan_rail_span_margin() -> f64 {
    (SCAN_RAIL_X - FILTER_X) / 2.0
}

fn pass_fail_tag_gap() -> f64 {
    let pass_right_edge = TAG_PANEL_CENTER.0 - TAG_POCKET_X / 2.0 - PASS_FAIL_SEGREGATION_MIN / 2.0
        + TAG_POCKET_X / 2.0;
    let fail_left_edge = TAG_PANEL_CENTER.0 + TAG_POCKET_X / 2.0 + PASS_FAIL_SEGREGATION_MIN / 2.0
        - TAG_POCKET_X / 2.0;
    fail_left_edge - pass_right_edge
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 10);
        for path in OUTPUTS {
            assert!(path.starts_with("output/closed_hepa_filter_scan_adapter_fixture_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn feature_list_covers_requested_controls() {
        assert_eq!(REQUIRED_FEATURES.len(), 8);
        assert!(REQUIRED_FEATURES.contains(&"filter_cassette_datum"));
        assert!(REQUIRED_FEATURES.contains(&"scan_wand_rails"));
        assert!(REQUIRED_FEATURES.contains(&"aerosol_challenge_port_placeholders"));
        assert!(REQUIRED_FEATURES.contains(&"upstream_downstream_pressure_taps"));
        assert!(REQUIRED_FEATURES.contains(&"gasket_witness_lands"));
        assert!(REQUIRED_FEATURES.contains(&"pass_fail_tag_pockets"));
        assert!(REQUIRED_FEATURES.contains(&"barcode_evidence_camera_bridge"));
        assert!(REQUIRED_FEATURES.contains(&"robot_service_keepouts"));
    }

    #[test]
    fn filter_cassette_datum_fits_deck_and_service_envelope() {
        assert!(FILTER_X >= 600.0);
        assert!(FILTER_Y >= 300.0);
        assert!(DATUM_X > FILTER_X + 120.0);
        assert!(DATUM_Y > FILTER_Y + 110.0);
        assert!(FILTER_CENTER.0.abs() + DATUM_X / 2.0 < DECK_X / 2.0 - 28.0);
        assert!(FILTER_CENTER.1.abs() + DATUM_Y / 2.0 < DECK_Y / 2.0 - 38.0);
        assert_eq!(LOCATOR_COUNT, cassette_locator_points().len());
        assert_eq!(CLAMP_COUNT, cassette_clamp_points().len());
    }

    #[test]
    fn scan_rails_cover_filter_face_with_standoff() {
        assert!(SCAN_RAIL_X > FILTER_X + 160.0);
        assert!(SCAN_RAIL_SPACING_Y > FILTER_Y + 120.0);
        assert_eq!(SCAN_PASS_COUNT, 9);
        assert!(scan_rail_span_margin() >= 80.0);
        assert!((WAND_STANDOFF_Z - 25.0).abs() < 0.001);
        assert!(CAMERA_BRIDGE_UNDERSIDE_Z > FILTER_Z + WAND_STANDOFF_Z + 60.0);
    }

    #[test]
    fn challenge_and_pressure_ports_are_explicit() {
        assert_eq!(AEROSOL_PORTS, 4);
        assert_eq!(PRESSURE_TAPS_PER_SIDE * 2, 10);
        assert!(AEROSOL_PORT_D >= 19.0);
        assert!(PRESSURE_TAP_D >= 7.0);
        let aerosol_edge =
            centered_index(AEROSOL_PORTS - 1, AEROSOL_PORTS, AEROSOL_PORT_PITCH).abs() + 32.0;
        let pressure_edge = centered_index(
            PRESSURE_TAPS_PER_SIDE - 1,
            PRESSURE_TAPS_PER_SIDE,
            PRESSURE_TAP_PITCH,
        )
        .abs()
            + 26.0;
        assert!(aerosol_edge < PORT_PANEL_X / 2.0);
        assert!(pressure_edge < PRESSURE_PANEL_X / 2.0);
    }

    #[test]
    fn gasket_witness_and_evidence_controls_are_counted() {
        assert_eq!(WITNESS_DISC_COUNT, 12);
        assert!(GASKET_COMPRESSION_MIN >= 3.0);
        assert!(GASKET_COMPRESSION_MAX <= 5.0);
        assert!(pass_fail_tag_gap() >= PASS_FAIL_SEGREGATION_MIN);
        assert_eq!(TAG_POCKET_COUNT, 6);
        assert_eq!(CAMERA_COUNT, 3);
        assert_eq!(LED_SEGMENTS, 8);
    }

    #[test]
    fn robot_and_service_keepouts_are_large_enough() {
        assert!(FRONT_ROBOT_KEEP_OUT_Y >= 180.0);
        assert!(REAR_SERVICE_KEEP_OUT_Y >= 250.0);
        assert!(RIGHT_AEROSOL_CART_KEEP_OUT_X >= 260.0);
        assert!(FILTER_PULL_KEEP_OUT_Z >= 350.0);
        assert!(ROBOT_Z_CLEARANCE >= 320.0);
    }
}
