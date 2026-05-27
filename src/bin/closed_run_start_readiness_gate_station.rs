use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed run-start readiness gate and recipe interlock station.
//
// Intent:
// - Force a physical "all prerequisites present" check before a tissue-chip
//   batch can move from setup into execution.
// - Represent only mechanical datums, custody token locations, scan lands,
//   purchased sensor/camera envelopes, and robot/service keepouts.
// - Keep recipe logic, eBR signatures, calibration acceptance criteria, and
//   biological disposition decisions outside this product-concept CAD model.

const OUTPUTS: [&str; 16] = [
    "output/closed_run_start_readiness_gate_station_cleanable_interlock_deck.stl",
    "output/closed_run_start_readiness_gate_station_released_material_token_slots.stl",
    "output/closed_run_start_readiness_gate_station_cassette_id_dock.stl",
    "output/closed_run_start_readiness_gate_station_connector_topology_token_dock.stl",
    "output/closed_run_start_readiness_gate_station_calibration_certificate_lands.stl",
    "output/closed_run_start_readiness_gate_station_sensor_cartridge_release_pockets.stl",
    "output/closed_run_start_readiness_gate_station_media_reagent_release_status_lanes.stl",
    "output/closed_run_start_readiness_gate_station_operator_robot_scan_lands.stl",
    "output/closed_run_start_readiness_gate_station_hold_reject_lockout_pocket.stl",
    "output/closed_run_start_readiness_gate_station_recipe_badge_slot.stl",
    "output/closed_run_start_readiness_gate_station_sample_archive_readiness_slots.stl",
    "output/closed_run_start_readiness_gate_station_pressure_leak_pass_token_row.stl",
    "output/closed_run_start_readiness_gate_station_environmental_map_token_row.stl",
    "output/closed_run_start_readiness_gate_station_camera_evidence_bridge.stl",
    "output/closed_run_start_readiness_gate_station_robot_service_keepouts.stl",
    "output/closed_run_start_readiness_gate_station_assembly.stl",
];

const REQUIRED_INTERLOCK_FEATURES: [&str; 14] = [
    "released_material_token_slots",
    "cassette_id_dock",
    "connector_topology_token_dock",
    "calibration_certificate_lands",
    "sensor_cartridge_release_pockets",
    "media_reagent_release_status_lanes",
    "operator_robot_scan_lands",
    "hold_reject_lockout_pocket",
    "recipe_badge_slot",
    "sample_archive_readiness_slots",
    "pressure_leak_pass_token_row",
    "environmental_map_token_row",
    "camera_evidence_bridge",
    "robot_service_keepouts",
];

const DECK_X: f64 = 1780.0;
const DECK_Y: f64 = 1240.0;
const DECK_Z: f64 = 22.0;
const DECK_RIM_W: f64 = 18.0;
const DECK_RIM_Z: f64 = 34.0;
const WASHDOWN_GUTTER_W: f64 = 14.0;
const MOUNT_HOLE_R: f64 = 3.3;

const MATERIAL_CENTER: (f64, f64) = (-725.0, 245.0);
const MATERIAL_PANEL_X: f64 = 250.0;
const MATERIAL_PANEL_Y: f64 = 520.0;
const MATERIAL_PANEL_Z: f64 = 34.0;
const MATERIAL_TOKEN_COUNT: usize = 24;
const MATERIAL_TOKEN_COLS: usize = 3;
const MATERIAL_TOKEN_X: f64 = 58.0;
const MATERIAL_TOKEN_Y: f64 = 38.0;
const MATERIAL_TOKEN_PITCH_X: f64 = 72.0;
const MATERIAL_TOKEN_PITCH_Y: f64 = 56.0;
const MATERIAL_TOKEN_DEPTH: f64 = 9.0;

const CHIP_COLS: usize = 4;
const CHIP_ROWS: usize = 5;
const CHIP_COUNT: usize = CHIP_COLS * CHIP_ROWS;
const CHIP_GUTTER: f64 = 8.0;
const CHIP_ARRAY_X: f64 =
    CHIP_COLS as f64 * REVC_CHIP_LENGTH + (CHIP_COLS as f64 - 1.0) * CHIP_GUTTER;
const CHIP_ARRAY_Y: f64 =
    CHIP_ROWS as f64 * REVC_CHIP_WIDTH + (CHIP_ROWS as f64 - 1.0) * CHIP_GUTTER;
const CASSETTE_MARGIN_X: f64 = 32.0;
const CASSETTE_MARGIN_Y: f64 = 32.0;
const CASSETTE_X: f64 = CHIP_ARRAY_X + CASSETTE_MARGIN_X * 2.0;
const CASSETTE_Y: f64 = CHIP_ARRAY_Y + CASSETTE_MARGIN_Y * 2.0;
const CASSETTE_CENTER: (f64, f64) = (-260.0, 210.0);
const CASSETTE_DOCK_X: f64 = 670.0;
const CASSETTE_DOCK_Y: f64 = 600.0;
const CASSETTE_DOCK_Z: f64 = 38.0;
const CASSETTE_RECESS_DEPTH: f64 = 9.0;
const CASSETTE_BARCODE_LANDS: usize = 6;

const TOPOLOGY_CENTER: (f64, f64) = (225.0, 270.0);
const TOPOLOGY_X: f64 = 250.0;
const TOPOLOGY_Y: f64 = 420.0;
const TOPOLOGY_Z: f64 = 34.0;
const TOPOLOGY_TOKEN_COUNT: usize = 24;
const TOPOLOGY_TOKEN_COLS: usize = 4;
const TOPOLOGY_TOKEN_D: f64 = 18.0;
const TOPOLOGY_TOKEN_PITCH_X: f64 = 48.0;
const TOPOLOGY_TOKEN_PITCH_Y: f64 = 48.0;
const TOPOLOGY_MAP_CARDS: usize = 4;

const CAL_CENTER: (f64, f64) = (620.0, 270.0);
const CAL_X: f64 = 410.0;
const CAL_Y: f64 = 420.0;
const CAL_Z: f64 = 34.0;
const CERT_CARD_COUNT: usize = 10;
const CERT_CARD_COLS: usize = 2;
const CERT_CARD_X: f64 = 144.0;
const CERT_CARD_Y: f64 = 34.0;
const CERT_CARD_PITCH_X: f64 = 178.0;
const CERT_CARD_PITCH_Y: f64 = 50.0;
const CAL_STANDARD_COUNT: usize = 6;

const RELEASE_STATUS_SLOTS: usize = MATERIAL_TOKEN_COUNT;

const SCAN_CENTER: (f64, f64) = (-725.0, -550.0);
const SCAN_X: f64 = 250.0;
const SCAN_Y: f64 = 110.0;
const SCAN_Z: f64 = 30.0;
const OPERATOR_SCAN_LANDS: usize = 6;
const ROBOT_SCAN_LANDS: usize = 6;

const SENSOR_CENTER: (f64, f64) = (-295.0, -360.0);
const SENSOR_X: f64 = 560.0;
const SENSOR_Y: f64 = 220.0;
const SENSOR_Z: f64 = 34.0;
const SENSOR_CARTRIDGE_COUNT: usize = 12;
const SENSOR_CARTRIDGE_COLS: usize = 6;
const SENSOR_CART_X: f64 = 58.0;
const SENSOR_CART_Y: f64 = 36.0;
const SENSOR_CART_PITCH_X: f64 = 78.0;
const SENSOR_CART_PITCH_Y: f64 = 58.0;

const MEDIA_CENTER: (f64, f64) = (225.0, -360.0);
const MEDIA_X: f64 = 440.0;
const MEDIA_Y: f64 = 220.0;
const MEDIA_Z: f64 = 34.0;
const MEDIA_STATUS_LANES: usize = 4;
const MEDIA_STATUS_POSITIONS_PER_LANE: usize = 3;
const MEDIA_STATUS_POSITIONS: usize = MEDIA_STATUS_LANES * MEDIA_STATUS_POSITIONS_PER_LANE;

const LOCKOUT_CENTER: (f64, f64) = (675.0, -360.0);
const LOCKOUT_X: f64 = 300.0;
const LOCKOUT_Y: f64 = 220.0;
const LOCKOUT_Z: f64 = 46.0;
const LOCKOUT_WALL: f64 = 10.0;
const LOCKOUT_TOKEN_COUNT: usize = 4;
const LOCKOUT_SEGREGATION_MIN: f64 = 58.0;

const RECIPE_CENTER: (f64, f64) = (405.0, -15.0);
const RECIPE_X: f64 = 360.0;
const RECIPE_Y: f64 = 90.0;
const RECIPE_Z: f64 = 32.0;
const RECIPE_BADGE_X: f64 = 176.0;
const RECIPE_BADGE_Y: f64 = 52.0;
const RECIPE_CHECKSUM_TOKENS: usize = 4;

const ARCHIVE_CENTER: (f64, f64) = (-295.0, -525.0);
const ARCHIVE_X: f64 = 560.0;
const ARCHIVE_Y: f64 = 100.0;
const ARCHIVE_Z: f64 = 30.0;
const ARCHIVE_READY_SLOTS: usize = 16;
const ARCHIVE_READY_COLS: usize = 8;

const PRESSURE_CENTER: (f64, f64) = (225.0, -525.0);
const PRESSURE_X: f64 = 440.0;
const PRESSURE_Y: f64 = 100.0;
const PRESSURE_Z: f64 = 30.0;
const PRESSURE_LEAK_TOKENS: usize = 12;
const PRESSURE_LEAK_COLS: usize = 6;

const ENV_CENTER: (f64, f64) = (675.0, -525.0);
const ENV_X: f64 = 300.0;
const ENV_Y: f64 = 100.0;
const ENV_Z: f64 = 30.0;
const ENV_MAP_TOKENS: usize = 12;
const ENV_MAP_COLS: usize = 6;

const BRIDGE_CENTER: (f64, f64) = (-80.0, 150.0);
const BRIDGE_SPAN_X: f64 = 1430.0;
const BRIDGE_Y: f64 = 86.0;
const BRIDGE_POST_X: f64 = 34.0;
const BRIDGE_POST_Y: f64 = 62.0;
const BRIDGE_BEAM_Z: f64 = 28.0;
const BRIDGE_UNDERSIDE_Z: f64 = 184.0;
const BRIDGE_POST_Z: f64 = BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z;
const CAMERA_COUNT: usize = 5;
const LED_SEGMENTS: usize = 10;

const FRONT_ROBOT_KEEP_OUT_Y: f64 = 132.0;
const REAR_SERVICE_KEEP_OUT_Y: f64 = 118.0;
const LEFT_CART_KEEP_OUT_X: f64 = 104.0;
const RIGHT_SERVICE_KEEP_OUT_X: f64 = 126.0;
const TOP_SERVICE_CLEARANCE_Z: f64 = 260.0;
const KEEP_OUT_GAUGE_Z: f64 = 8.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let deck = cleanable_interlock_deck();
    export(OUTPUTS[0], &deck);

    let materials = released_material_token_slots();
    export(OUTPUTS[1], &materials);

    let cassette = cassette_id_dock();
    export(OUTPUTS[2], &cassette);

    let topology = connector_topology_token_dock();
    export(OUTPUTS[3], &topology);

    let calibration = calibration_certificate_lands();
    export(OUTPUTS[4], &calibration);

    let sensors = sensor_cartridge_release_pockets();
    export(OUTPUTS[5], &sensors);

    let media = media_reagent_release_status_lanes();
    export(OUTPUTS[6], &media);

    let scans = operator_robot_scan_lands();
    export(OUTPUTS[7], &scans);

    let lockout = hold_reject_lockout_pocket();
    export(OUTPUTS[8], &lockout);

    let recipe = recipe_badge_slot();
    export(OUTPUTS[9], &recipe);

    let archive = sample_archive_readiness_slots();
    export(OUTPUTS[10], &archive);

    let pressure = pressure_leak_pass_token_row();
    export(OUTPUTS[11], &pressure);

    let environment = environmental_map_token_row();
    export(OUTPUTS[12], &environment);

    let bridge = camera_evidence_bridge();
    export(OUTPUTS[13], &bridge);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[14], &keepouts);

    let assembly =
        deck + materials.translate(
            MATERIAL_CENTER.0,
            MATERIAL_CENTER.1,
            deck_insert_z(MATERIAL_PANEL_Z),
        ) + cassette.translate(
            CASSETTE_CENTER.0,
            CASSETTE_CENTER.1,
            deck_insert_z(CASSETTE_DOCK_Z),
        ) + topology.translate(
            TOPOLOGY_CENTER.0,
            TOPOLOGY_CENTER.1,
            deck_insert_z(TOPOLOGY_Z),
        ) + calibration.translate(CAL_CENTER.0, CAL_CENTER.1, deck_insert_z(CAL_Z))
            + sensors.translate(SENSOR_CENTER.0, SENSOR_CENTER.1, deck_insert_z(SENSOR_Z))
            + media.translate(MEDIA_CENTER.0, MEDIA_CENTER.1, deck_insert_z(MEDIA_Z))
            + scans.translate(SCAN_CENTER.0, SCAN_CENTER.1, deck_insert_z(SCAN_Z))
            + lockout.translate(LOCKOUT_CENTER.0, LOCKOUT_CENTER.1, deck_insert_z(LOCKOUT_Z))
            + recipe.translate(RECIPE_CENTER.0, RECIPE_CENTER.1, deck_insert_z(RECIPE_Z))
            + archive.translate(ARCHIVE_CENTER.0, ARCHIVE_CENTER.1, deck_insert_z(ARCHIVE_Z))
            + pressure.translate(
                PRESSURE_CENTER.0,
                PRESSURE_CENTER.1,
                deck_insert_z(PRESSURE_Z),
            )
            + environment.translate(ENV_CENTER.0, ENV_CENTER.1, deck_insert_z(ENV_Z))
            + bridge.translate(BRIDGE_CENTER.0, BRIDGE_CENTER.1, DECK_Z / 2.0)
            + keepouts.translate(0.0, 0.0, DECK_Z / 2.0 + KEEP_OUT_GAUGE_Z / 2.0);
    export(OUTPUTS[15], &assembly);

    println!();
    println!("Closed run-start readiness gate and recipe interlock station:");
    println!("  Deck:                         {DECK_X:.0} x {DECK_Y:.0} x {DECK_Z:.0} mm");
    println!(
        "  Cassette ID dock:             {CASSETTE_X:.1} x {CASSETTE_Y:.1} mm cassette datum covering {CHIP_COUNT} Rev C chip positions"
    );
    println!(
        "  Released material tokens:     {MATERIAL_TOKEN_COUNT} slots for cell lot, chip lot, ECM/coating, media, reagent, consumable, and connector release evidence"
    );
    println!(
        "  Topology and recipe gates:    {TOPOLOGY_TOKEN_COUNT} connector topology tokens, {TOPOLOGY_MAP_CARDS} map cards, {RECIPE_CHECKSUM_TOKENS} checksum tokens, and one recipe badge slot"
    );
    println!(
        "  Release evidence:             {CERT_CARD_COUNT} calibration certificate lands, {CAL_STANDARD_COUNT} calibration standard tokens, {SENSOR_CARTRIDGE_COUNT} sensor cartridge pockets"
    );
    println!(
        "  Run-start status rows:        {RELEASE_STATUS_SLOTS} material release slots, {MEDIA_STATUS_POSITIONS} media/reagent positions, {ARCHIVE_READY_SLOTS} archive-ready slots, {PRESSURE_LEAK_TOKENS} pressure/leak tokens, {ENV_MAP_TOKENS} environmental map tokens"
    );
    println!(
        "  Evidence capture:             {CAMERA_COUNT} camera pods, {LED_SEGMENTS} LED segments, {:.0} mm bridge underside clearance above deck",
        bridge_clearance_above_deck()
    );
    println!(
        "  Interlock feature groups:     {} physical prerequisite groups modeled",
        REQUIRED_INTERLOCK_FEATURES.len()
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn deck_insert_z(height: f64) -> f64 {
    DECK_Z / 2.0 + height / 2.0
}

fn assert_layout() {
    for (center, x, y, name) in main_footprints() {
        assert!(
            fits_on_deck(center, x, y, 12.0),
            "{name} footprint does not fit on readiness gate deck"
        );
    }

    let footprints = main_footprints();
    for left in 0..footprints.len() {
        for right in left + 1..footprints.len() {
            let a = footprints[left];
            let b = footprints[right];
            assert!(
                !rects_overlap(rect(a.0, a.1, a.2), rect(b.0, b.1, b.2)),
                "{} overlaps {}",
                a.3,
                b.3
            );
        }
    }

    assert!(
        horizontal_gap(
            rect(LOCKOUT_CENTER, LOCKOUT_X, LOCKOUT_Y),
            rect(MEDIA_CENTER, MEDIA_X, MEDIA_Y)
        ) >= LOCKOUT_SEGREGATION_MIN,
        "hold/reject lockout is too close to released media/reagent lanes"
    );
    assert!(CASSETTE_X < CASSETTE_DOCK_X - 42.0);
    assert!(CASSETTE_Y < CASSETTE_DOCK_Y - 42.0);
    assert!(fits_on_deck(
        BRIDGE_CENTER,
        BRIDGE_SPAN_X,
        BRIDGE_Y + 160.0,
        12.0
    ));
    assert!(bridge_clearance_above_deck() >= 170.0);
}

fn cleanable_interlock_deck() -> Part {
    let deck = centered_cube(
        "closed_run_start_readiness_gate_station_cleanable_deck_plate",
        DECK_X,
        DECK_Y,
        DECK_Z,
    );

    let mut recesses = Part::empty("closed_run_start_readiness_gate_station_module_recesses");
    for (center, x, y, name) in main_footprints() {
        recesses = recesses
            + top_recess(
                format!("closed_run_start_readiness_gate_station_{name}_recess"),
                center,
                x + 18.0,
                y + 18.0,
                5.0,
            );
    }

    deck - recesses - washdown_gutters() - deck_mount_and_drain_holes()
        + deck_perimeter_lips()
        + rear_cable_gland_rail()
        + recipe_interlock_index_pins()
}

fn top_recess(name: impl Into<String>, center: (f64, f64), x: f64, y: f64, depth: f64) -> Part {
    centered_cube(name, x, y, depth + 0.2).translate(
        center.0,
        center.1,
        DECK_Z / 2.0 - depth / 2.0 + 0.1,
    )
}

fn washdown_gutters() -> Part {
    let front_sump = centered_cube(
        "closed_run_start_readiness_gate_station_front_washdown_sump",
        DECK_X - 220.0,
        18.0,
        8.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 38.0, DECK_Z / 2.0 - 3.0);

    let center_gutter = centered_cube(
        "closed_run_start_readiness_gate_station_center_wipe_gutter",
        DECK_X - 310.0,
        WASHDOWN_GUTTER_W,
        7.0,
    )
    .translate(24.0, -118.0, DECK_Z / 2.0 - 2.8);

    let left_gutter = centered_cube(
        "closed_run_start_readiness_gate_station_left_wipe_gutter",
        WASHDOWN_GUTTER_W,
        DECK_Y - 210.0,
        7.0,
    )
    .translate(-585.0, -26.0, DECK_Z / 2.0 - 2.8);

    let right_gutter = centered_cube(
        "closed_run_start_readiness_gate_station_right_wipe_gutter",
        WASHDOWN_GUTTER_W,
        DECK_Y - 300.0,
        7.0,
    )
    .translate(468.0, -94.0, DECK_Z / 2.0 - 2.8);

    front_sump + center_gutter + left_gutter + right_gutter
}

fn deck_mount_and_drain_holes() -> Part {
    let drain = centered_cylinder(
        "closed_run_start_readiness_gate_station_low_point_drain",
        8.0,
        DECK_Z + 4.0,
        32,
    )
    .translate(DECK_X / 2.0 - 78.0, -DECK_Y / 2.0 + 38.0, 0.0);

    let mut mounts = Part::empty("closed_run_start_readiness_gate_station_mount_holes");
    for (index, (x, y)) in deck_mount_points().iter().enumerate() {
        mounts = mounts
            + centered_cylinder(
                format!("closed_run_start_readiness_gate_station_m6_mount_hole_{index}"),
                MOUNT_HOLE_R,
                DECK_Z + 4.0,
                28,
            )
            .translate(*x, *y, 0.0);
    }

    drain + mounts
}

fn deck_perimeter_lips() -> Part {
    let rear = centered_cube(
        "closed_run_start_readiness_gate_station_rear_cleanable_lip",
        DECK_X - 110.0,
        DECK_RIM_W,
        DECK_RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - 38.0, DECK_Z / 2.0 + DECK_RIM_Z / 2.0);

    let left = centered_cube(
        "closed_run_start_readiness_gate_station_left_cleanable_lip",
        DECK_RIM_W,
        DECK_Y - 150.0,
        DECK_RIM_Z,
    )
    .translate(-DECK_X / 2.0 + 38.0, 0.0, DECK_Z / 2.0 + DECK_RIM_Z / 2.0);

    let right = centered_cube(
        "closed_run_start_readiness_gate_station_right_cleanable_lip",
        DECK_RIM_W,
        DECK_Y - 250.0,
        DECK_RIM_Z,
    )
    .translate(DECK_X / 2.0 - 38.0, -40.0, DECK_Z / 2.0 + DECK_RIM_Z / 2.0);

    let front_low = centered_cube(
        "closed_run_start_readiness_gate_station_front_low_retaining_lip",
        DECK_X - 230.0,
        12.0,
        14.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 22.0, DECK_Z / 2.0 + 7.0);

    rear + left + right + front_low
}

fn rear_cable_gland_rail() -> Part {
    let rail = centered_cube(
        "closed_run_start_readiness_gate_station_rear_cable_gland_rail",
        DECK_X - 260.0,
        18.0,
        26.0,
    )
    .translate(20.0, DECK_Y / 2.0 - 82.0, DECK_Z / 2.0 + 13.0);

    let mut glands = Part::empty("closed_run_start_readiness_gate_station_cable_gland_holes");
    for index in 0..8 {
        glands = glands
            + centered_cylinder(
                format!("closed_run_start_readiness_gate_station_cable_gland_{index}"),
                7.0,
                22.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                -520.0 + index as f64 * 150.0,
                DECK_Y / 2.0 - 82.0,
                DECK_Z / 2.0 + 13.0,
            );
    }

    rail - glands
}

fn recipe_interlock_index_pins() -> Part {
    let mut pins = Part::empty("closed_run_start_readiness_gate_station_interlock_index_pins");
    for (index, (x, y)) in [
        (RECIPE_CENTER.0 - RECIPE_X / 2.0 - 16.0, RECIPE_CENTER.1),
        (RECIPE_CENTER.0 + RECIPE_X / 2.0 + 16.0, RECIPE_CENTER.1),
        (LOCKOUT_CENTER.0 - LOCKOUT_X / 2.0 - 16.0, LOCKOUT_CENTER.1),
        (LOCKOUT_CENTER.0 + LOCKOUT_X / 2.0 + 16.0, LOCKOUT_CENTER.1),
    ]
    .iter()
    .enumerate()
    {
        pins = pins
            + centered_cylinder(
                format!("closed_run_start_readiness_gate_station_interlock_index_pin_{index}"),
                4.0,
                16.0,
                24,
            )
            .translate(*x, *y, DECK_Z / 2.0 + 8.0);
    }
    pins
}

fn released_material_token_slots() -> Part {
    let panel = centered_cube(
        "closed_run_start_readiness_gate_station_released_material_token_panel",
        MATERIAL_PANEL_X,
        MATERIAL_PANEL_Y,
        MATERIAL_PANEL_Z,
    );

    let slots = rectangular_slot_grid(
        "released_material_token",
        MATERIAL_TOKEN_COUNT,
        MATERIAL_TOKEN_COLS,
        MATERIAL_TOKEN_X,
        MATERIAL_TOKEN_Y,
        MATERIAL_TOKEN_PITCH_X,
        MATERIAL_TOKEN_PITCH_Y,
        (0.0, -14.0),
        MATERIAL_TOKEN_DEPTH,
        MATERIAL_PANEL_Z,
    );

    let group_dividers = material_token_dividers();
    let labels = material_token_label_lands();
    let sensors = token_present_sensor_pips(
        "released_material",
        MATERIAL_TOKEN_COUNT,
        MATERIAL_TOKEN_COLS,
        MATERIAL_TOKEN_PITCH_X,
        MATERIAL_TOKEN_PITCH_Y,
        (0.0, -14.0),
        MATERIAL_PANEL_Z,
    );

    panel - slots + group_dividers + labels + sensors
}

fn material_token_dividers() -> Part {
    let horizontal_1 = centered_cube(
        "closed_run_start_readiness_gate_station_cell_media_token_divider",
        MATERIAL_PANEL_X - 28.0,
        8.0,
        14.0,
    )
    .translate(0.0, 90.0, MATERIAL_PANEL_Z / 2.0 + 7.0);
    let horizontal_2 = centered_cube(
        "closed_run_start_readiness_gate_station_consumable_connector_token_divider",
        MATERIAL_PANEL_X - 28.0,
        8.0,
        14.0,
    )
    .translate(0.0, -112.0, MATERIAL_PANEL_Z / 2.0 + 7.0);
    let vertical = centered_cube(
        "closed_run_start_readiness_gate_station_released_material_center_divider",
        8.0,
        MATERIAL_PANEL_Y - 62.0,
        12.0,
    )
    .translate(0.0, -8.0, MATERIAL_PANEL_Z / 2.0 + 6.0);

    horizontal_1 + horizontal_2 + vertical
}

fn material_token_label_lands() -> Part {
    let mut lands = Part::empty("closed_run_start_readiness_gate_station_material_label_lands");
    for (index, y) in [205.0, 132.0, 28.0, -56.0, -160.0, -224.0]
        .iter()
        .enumerate()
    {
        lands = lands
            + centered_cube(
                format!("closed_run_start_readiness_gate_station_material_label_land_{index}"),
                196.0,
                18.0,
                4.0,
            )
            .translate(0.0, *y, MATERIAL_PANEL_Z / 2.0 + 2.0);
    }
    lands
}

fn cassette_id_dock() -> Part {
    let panel = centered_cube(
        "closed_run_start_readiness_gate_station_cassette_id_dock_panel",
        CASSETTE_DOCK_X,
        CASSETTE_DOCK_Y,
        CASSETTE_DOCK_Z,
    );

    let cassette_recess = centered_cube(
        "closed_run_start_readiness_gate_station_cassette_revc_recess",
        CASSETTE_X + 14.0,
        CASSETTE_Y + 14.0,
        CASSETTE_RECESS_DEPTH + 0.2,
    )
    .translate(
        0.0,
        0.0,
        CASSETTE_DOCK_Z / 2.0 - CASSETTE_RECESS_DEPTH / 2.0 + 0.1,
    );

    let perimeter_rail = rect_frame_xy(
        "closed_run_start_readiness_gate_station_cassette_datum_rail",
        CASSETTE_X + 50.0,
        CASSETTE_Y + 50.0,
        13.0,
        26.0,
    )
    .translate(0.0, 0.0, CASSETTE_DOCK_Z / 2.0 + 13.0);

    let chip_id_lands = cassette_chip_identity_lands();
    let corner_datums = cassette_corner_datums();
    let barcode_lands = cassette_barcode_lands();
    let latch_proof_lugs = cassette_latch_proof_lugs();

    panel - cassette_recess
        + perimeter_rail
        + chip_id_lands
        + corner_datums
        + barcode_lands
        + latch_proof_lugs
}

fn cassette_chip_identity_lands() -> Part {
    let mut lands = Part::empty("closed_run_start_readiness_gate_station_cassette_chip_id_lands");
    for row in 0..CHIP_ROWS {
        for col in 0..CHIP_COLS {
            let index = row * CHIP_COLS + col;
            let x = -CHIP_ARRAY_X / 2.0
                + REVC_CHIP_LENGTH / 2.0
                + col as f64 * (REVC_CHIP_LENGTH + CHIP_GUTTER);
            let y = -CHIP_ARRAY_Y / 2.0
                + REVC_CHIP_WIDTH / 2.0
                + row as f64 * (REVC_CHIP_WIDTH + CHIP_GUTTER);
            lands = lands
                + centered_cube(
                    format!("closed_run_start_readiness_gate_station_chip_identity_land_{index}"),
                    REVC_CHIP_LENGTH - 34.0,
                    13.0,
                    4.0,
                )
                .translate(
                    x,
                    y + REVC_CHIP_WIDTH / 2.0 - 14.0,
                    CASSETTE_DOCK_Z / 2.0 + 2.0,
                );
        }
    }
    lands
}

fn cassette_corner_datums() -> Part {
    let mut datums = Part::empty("closed_run_start_readiness_gate_station_cassette_corner_datums");
    for (index, (x, y)) in [
        (-(CASSETTE_X / 2.0 + 12.0), -(CASSETTE_Y / 2.0 + 12.0)),
        (-(CASSETTE_X / 2.0 + 12.0), CASSETTE_Y / 2.0 + 12.0),
        (CASSETTE_X / 2.0 + 12.0, -(CASSETTE_Y / 2.0 + 12.0)),
        (CASSETTE_X / 2.0 + 12.0, CASSETTE_Y / 2.0 + 12.0),
    ]
    .iter()
    .enumerate()
    {
        datums = datums
            + centered_cylinder(
                format!("closed_run_start_readiness_gate_station_cassette_datum_pin_{index}"),
                4.0,
                18.0,
                32,
            )
            .translate(*x, *y, CASSETTE_DOCK_Z / 2.0 + 9.0)
            + centered_cylinder(
                format!("closed_run_start_readiness_gate_station_cassette_datum_washer_{index}"),
                10.0,
                4.0,
                32,
            )
            .translate(*x, *y, CASSETTE_DOCK_Z / 2.0 + 19.0);
    }
    datums
}

fn cassette_barcode_lands() -> Part {
    let mut lands = Part::empty("closed_run_start_readiness_gate_station_cassette_barcode_lands");
    for index in 0..CASSETTE_BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_run_start_readiness_gate_station_cassette_barcode_land_{index}"),
                82.0,
                18.0,
                4.0,
            )
            .translate(
                -250.0 + index as f64 * 100.0,
                -(CASSETTE_Y / 2.0 + 30.0),
                CASSETTE_DOCK_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn cassette_latch_proof_lugs() -> Part {
    let left = centered_cube(
        "closed_run_start_readiness_gate_station_cassette_left_latch_proof_lug",
        18.0,
        120.0,
        18.0,
    )
    .translate(-(CASSETTE_X / 2.0 + 26.0), 0.0, CASSETTE_DOCK_Z / 2.0 + 9.0);
    let right = centered_cube(
        "closed_run_start_readiness_gate_station_cassette_right_latch_proof_lug",
        18.0,
        120.0,
        18.0,
    )
    .translate(CASSETTE_X / 2.0 + 26.0, 0.0, CASSETTE_DOCK_Z / 2.0 + 9.0);
    let rear_stop = centered_cube(
        "closed_run_start_readiness_gate_station_cassette_rear_nose_stop",
        CASSETTE_X + 28.0,
        16.0,
        22.0,
    )
    .translate(0.0, CASSETTE_Y / 2.0 + 24.0, CASSETTE_DOCK_Z / 2.0 + 11.0);

    left + right + rear_stop
}

fn connector_topology_token_dock() -> Part {
    let panel = centered_cube(
        "closed_run_start_readiness_gate_station_connector_topology_token_panel",
        TOPOLOGY_X,
        TOPOLOGY_Y,
        TOPOLOGY_Z,
    );

    let token_cuts = circular_token_grid(
        "connector_topology",
        TOPOLOGY_TOKEN_COUNT,
        TOPOLOGY_TOKEN_COLS,
        TOPOLOGY_TOKEN_D / 2.0,
        TOPOLOGY_TOKEN_PITCH_X,
        TOPOLOGY_TOKEN_PITCH_Y,
        (0.0, 22.0),
        10.0,
        TOPOLOGY_Z,
    );

    let mut map_cards = Part::empty("closed_run_start_readiness_gate_station_topology_map_cards");
    for index in 0..TOPOLOGY_MAP_CARDS {
        map_cards = map_cards
            + centered_cube(
                format!("closed_run_start_readiness_gate_station_topology_map_card_slot_{index}"),
                82.0,
                32.0,
                10.0,
            )
            .translate(
                -60.0 + (index % 2) as f64 * 120.0,
                -164.0 + (index / 2) as f64 * 48.0,
                TOPOLOGY_Z / 2.0 - 4.9,
            );
    }

    panel - token_cuts - map_cards
        + topology_token_family_rails()
        + token_present_sensor_pips(
            "connector_topology",
            TOPOLOGY_TOKEN_COUNT,
            TOPOLOGY_TOKEN_COLS,
            TOPOLOGY_TOKEN_PITCH_X,
            TOPOLOGY_TOKEN_PITCH_Y,
            (0.0, 22.0),
            TOPOLOGY_Z,
        )
}

fn topology_token_family_rails() -> Part {
    let vertical = centered_cube(
        "closed_run_start_readiness_gate_station_topology_family_vertical_rail",
        8.0,
        TOPOLOGY_Y - 58.0,
        12.0,
    )
    .translate(0.0, 20.0, TOPOLOGY_Z / 2.0 + 6.0);
    let horizontal = centered_cube(
        "closed_run_start_readiness_gate_station_topology_family_horizontal_rail",
        TOPOLOGY_X - 36.0,
        8.0,
        12.0,
    )
    .translate(0.0, -70.0, TOPOLOGY_Z / 2.0 + 6.0);

    vertical + horizontal
}

fn calibration_certificate_lands() -> Part {
    let panel = centered_cube(
        "closed_run_start_readiness_gate_station_calibration_certificate_panel",
        CAL_X,
        CAL_Y,
        CAL_Z,
    );

    let cert_cuts = rectangular_slot_grid(
        "calibration_certificate_card",
        CERT_CARD_COUNT,
        CERT_CARD_COLS,
        CERT_CARD_X,
        CERT_CARD_Y,
        CERT_CARD_PITCH_X,
        CERT_CARD_PITCH_Y,
        (0.0, 72.0),
        8.0,
        CAL_Z,
    );

    let mut standard_cuts =
        Part::empty("closed_run_start_readiness_gate_station_cal_standard_cuts");
    for index in 0..CAL_STANDARD_COUNT {
        standard_cuts = standard_cuts
            + centered_cylinder(
                format!(
                    "closed_run_start_readiness_gate_station_calibration_standard_token_{index}"
                ),
                13.0,
                9.0,
                32,
            )
            .translate(-150.0 + index as f64 * 60.0, -168.0, CAL_Z / 2.0 - 4.4);
    }

    panel - cert_cuts - standard_cuts
        + calibration_label_lands()
        + calibration_certificate_fiducials()
}

fn calibration_label_lands() -> Part {
    let mut lands = Part::empty("closed_run_start_readiness_gate_station_calibration_label_lands");
    for (index, y) in [182.0, 120.0, 58.0, -4.0, -66.0, -128.0].iter().enumerate() {
        lands = lands
            + centered_cube(
                format!("closed_run_start_readiness_gate_station_calibration_label_land_{index}"),
                CAL_X - 48.0,
                14.0,
                4.0,
            )
            .translate(0.0, *y, CAL_Z / 2.0 + 2.0);
    }
    lands
}

fn calibration_certificate_fiducials() -> Part {
    let mut fiducials =
        Part::empty("closed_run_start_readiness_gate_station_calibration_fiducials");
    for (index, (x, y)) in [
        (-CAL_X / 2.0 + 24.0, -CAL_Y / 2.0 + 24.0),
        (CAL_X / 2.0 - 24.0, -CAL_Y / 2.0 + 24.0),
        (-CAL_X / 2.0 + 24.0, CAL_Y / 2.0 - 24.0),
        (CAL_X / 2.0 - 24.0, CAL_Y / 2.0 - 24.0),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + centered_cylinder(
                format!("closed_run_start_readiness_gate_station_calibration_fiducial_{index}"),
                3.0,
                5.0,
                24,
            )
            .translate(*x, *y, CAL_Z / 2.0 + 2.5);
    }
    fiducials
}

fn sensor_cartridge_release_pockets() -> Part {
    let panel = centered_cube(
        "closed_run_start_readiness_gate_station_sensor_cartridge_release_panel",
        SENSOR_X,
        SENSOR_Y,
        SENSOR_Z,
    );

    let cartridge_cuts = rectangular_slot_grid(
        "sensor_cartridge_release",
        SENSOR_CARTRIDGE_COUNT,
        SENSOR_CARTRIDGE_COLS,
        SENSOR_CART_X,
        SENSOR_CART_Y,
        SENSOR_CART_PITCH_X,
        SENSOR_CART_PITCH_Y,
        (-10.0, 24.0),
        10.0,
        SENSOR_Z,
    );

    panel - cartridge_cuts + sensor_family_lands() + sensor_release_barcode_lands()
}

fn sensor_family_lands() -> Part {
    let mut lands = Part::empty("closed_run_start_readiness_gate_station_sensor_family_lands");
    for index in 0..6 {
        lands = lands
            + centered_cube(
                format!("closed_run_start_readiness_gate_station_sensor_family_land_{index}"),
                66.0,
                16.0,
                4.0,
            )
            .translate(-205.0 + index as f64 * 82.0, 88.0, SENSOR_Z / 2.0 + 2.0);
    }
    lands
}

fn sensor_release_barcode_lands() -> Part {
    let mut lands =
        Part::empty("closed_run_start_readiness_gate_station_sensor_release_barcode_lands");
    for index in 0..4 {
        lands = lands
            + centered_cube(
                format!("closed_run_start_readiness_gate_station_sensor_barcode_land_{index}"),
                108.0,
                18.0,
                4.0,
            )
            .translate(-190.0 + index as f64 * 126.0, -80.0, SENSOR_Z / 2.0 + 2.0);
    }
    lands
}

fn media_reagent_release_status_lanes() -> Part {
    let panel = centered_cube(
        "closed_run_start_readiness_gate_station_media_reagent_release_panel",
        MEDIA_X,
        MEDIA_Y,
        MEDIA_Z,
    );

    let mut cuts = Part::empty("closed_run_start_readiness_gate_station_media_release_slot_cuts");
    for lane in 0..MEDIA_STATUS_LANES {
        for position in 0..MEDIA_STATUS_POSITIONS_PER_LANE {
            let index = lane * MEDIA_STATUS_POSITIONS_PER_LANE + position;
            cuts = cuts
                + centered_cube(
                    format!("closed_run_start_readiness_gate_station_media_status_slot_{index}"),
                    84.0,
                    30.0,
                    9.0,
                )
                .translate(
                    -132.0 + position as f64 * 132.0,
                    70.0 - lane as f64 * 48.0,
                    MEDIA_Z / 2.0 - 4.4,
                );
        }
    }

    panel - cuts + media_lane_separation_rails() + media_release_scan_lands()
}

fn media_lane_separation_rails() -> Part {
    let mut rails = Part::empty("closed_run_start_readiness_gate_station_media_lane_rails");
    for index in 0..3 {
        rails = rails
            + centered_cube(
                format!("closed_run_start_readiness_gate_station_media_lane_rail_{index}"),
                MEDIA_X - 48.0,
                6.0,
                12.0,
            )
            .translate(0.0, 46.0 - index as f64 * 48.0, MEDIA_Z / 2.0 + 6.0);
    }
    rails
}

fn media_release_scan_lands() -> Part {
    let left = centered_cube(
        "closed_run_start_readiness_gate_station_media_release_coa_scan_land",
        150.0,
        20.0,
        4.0,
    )
    .translate(-102.0, -94.0, MEDIA_Z / 2.0 + 2.0);
    let right = centered_cube(
        "closed_run_start_readiness_gate_station_reagent_release_coa_scan_land",
        150.0,
        20.0,
        4.0,
    )
    .translate(102.0, -94.0, MEDIA_Z / 2.0 + 2.0);
    left + right
}

fn operator_robot_scan_lands() -> Part {
    let panel = centered_cube(
        "closed_run_start_readiness_gate_station_operator_robot_scan_panel",
        SCAN_X,
        SCAN_Y,
        SCAN_Z,
    );

    let mut cuts = Part::empty("closed_run_start_readiness_gate_station_operator_robot_scan_cuts");
    for index in 0..OPERATOR_SCAN_LANDS {
        cuts = cuts
            + centered_cube(
                format!("closed_run_start_readiness_gate_station_operator_badge_scan_slot_{index}"),
                54.0,
                20.0,
                7.0,
            )
            .translate(
                -82.0 + (index % 3) as f64 * 82.0,
                28.0 - (index / 3) as f64 * 36.0,
                SCAN_Z / 2.0 - 3.4,
            );
    }
    for index in 0..ROBOT_SCAN_LANDS {
        cuts = cuts
            + centered_cube(
                format!("closed_run_start_readiness_gate_station_robot_tool_scan_slot_{index}"),
                54.0,
                18.0,
                7.0,
            )
            .translate(
                -82.0 + (index % 3) as f64 * 82.0,
                -10.0 - (index / 3) as f64 * 34.0,
                SCAN_Z / 2.0 - 3.4,
            );
    }

    panel - cuts + scan_status_beacon_lands()
}

fn scan_status_beacon_lands() -> Part {
    let left = centered_cylinder(
        "closed_run_start_readiness_gate_station_operator_scan_beacon_land",
        10.0,
        5.0,
        32,
    )
    .translate(-106.0, 42.0, SCAN_Z / 2.0 + 2.5);
    let right = centered_cylinder(
        "closed_run_start_readiness_gate_station_robot_scan_beacon_land",
        10.0,
        5.0,
        32,
    )
    .translate(106.0, 42.0, SCAN_Z / 2.0 + 2.5);
    left + right
}

fn hold_reject_lockout_pocket() -> Part {
    let body = centered_cube(
        "closed_run_start_readiness_gate_station_hold_reject_lockout_body",
        LOCKOUT_X,
        LOCKOUT_Y,
        LOCKOUT_Z,
    );
    let pocket_cut = centered_cube(
        "closed_run_start_readiness_gate_station_hold_reject_lockout_deep_pocket",
        LOCKOUT_X - LOCKOUT_WALL * 2.0,
        LOCKOUT_Y - LOCKOUT_WALL * 2.0,
        28.0,
    )
    .translate(0.0, 0.0, LOCKOUT_Z / 2.0 - 13.9);

    let mut token_cuts = Part::empty("closed_run_start_readiness_gate_station_lockout_token_cuts");
    for index in 0..LOCKOUT_TOKEN_COUNT {
        token_cuts = token_cuts
            + centered_cube(
                format!("closed_run_start_readiness_gate_station_lockout_cause_token_{index}"),
                52.0,
                26.0,
                8.0,
            )
            .translate(
                -84.0 + (index % 2) as f64 * 168.0,
                -48.0 + (index / 2) as f64 * 96.0,
                LOCKOUT_Z / 2.0 - 3.9,
            );
    }

    body - pocket_cut - token_cuts + lockout_gate_bar() + lockout_label_lands()
}

fn lockout_gate_bar() -> Part {
    let bar = centered_cube(
        "closed_run_start_readiness_gate_station_recipe_start_lockout_bar",
        LOCKOUT_X - 44.0,
        16.0,
        26.0,
    )
    .translate(0.0, LOCKOUT_Y / 2.0 - 32.0, LOCKOUT_Z / 2.0 + 13.0);
    let hinge = centered_cylinder(
        "closed_run_start_readiness_gate_station_lockout_hinge_placeholder",
        8.0,
        LOCKOUT_X - 62.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, LOCKOUT_Y / 2.0 - 54.0, LOCKOUT_Z / 2.0 + 18.0);
    bar + hinge
}

fn lockout_label_lands() -> Part {
    let hold = centered_cube(
        "closed_run_start_readiness_gate_station_hold_status_label_land",
        116.0,
        18.0,
        4.0,
    )
    .translate(-68.0, -LOCKOUT_Y / 2.0 + 28.0, LOCKOUT_Z / 2.0 + 2.0);
    let reject = centered_cube(
        "closed_run_start_readiness_gate_station_reject_status_label_land",
        116.0,
        18.0,
        4.0,
    )
    .translate(68.0, -LOCKOUT_Y / 2.0 + 28.0, LOCKOUT_Z / 2.0 + 2.0);
    hold + reject
}

fn recipe_badge_slot() -> Part {
    let panel = centered_cube(
        "closed_run_start_readiness_gate_station_recipe_badge_panel",
        RECIPE_X,
        RECIPE_Y,
        RECIPE_Z,
    );
    let badge_cut = centered_cube(
        "closed_run_start_readiness_gate_station_recipe_badge_card_slot",
        RECIPE_BADGE_X,
        RECIPE_BADGE_Y,
        10.0,
    )
    .translate(-64.0, 0.0, RECIPE_Z / 2.0 - 4.9);

    let mut checksum_cuts =
        Part::empty("closed_run_start_readiness_gate_station_recipe_checksum_token_cuts");
    for index in 0..RECIPE_CHECKSUM_TOKENS {
        checksum_cuts = checksum_cuts
            + centered_cylinder(
                format!("closed_run_start_readiness_gate_station_recipe_checksum_token_{index}"),
                10.0,
                9.0,
                32,
            )
            .translate(
                72.0 + (index % 2) as f64 * 62.0,
                -18.0 + (index / 2) as f64 * 36.0,
                RECIPE_Z / 2.0 - 4.4,
            );
    }

    panel - badge_cut - checksum_cuts + recipe_gate_contact_lands()
}

fn recipe_gate_contact_lands() -> Part {
    let mut lands =
        Part::empty("closed_run_start_readiness_gate_station_recipe_gate_contact_lands");
    for index in 0..6 {
        lands = lands
            + centered_cube(
                format!("closed_run_start_readiness_gate_station_recipe_contact_land_{index}"),
                18.0,
                10.0,
                4.0,
            )
            .translate(
                -146.0 + index as f64 * 28.0,
                RECIPE_Y / 2.0 - 18.0,
                RECIPE_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn sample_archive_readiness_slots() -> Part {
    let panel = centered_cube(
        "closed_run_start_readiness_gate_station_sample_archive_readiness_panel",
        ARCHIVE_X,
        ARCHIVE_Y,
        ARCHIVE_Z,
    );
    let cuts = circular_token_grid(
        "sample_archive_ready",
        ARCHIVE_READY_SLOTS,
        ARCHIVE_READY_COLS,
        9.0,
        58.0,
        38.0,
        (0.0, 0.0),
        8.0,
        ARCHIVE_Z,
    );
    panel - cuts + front_row_label_strip("archive", ARCHIVE_X, ARCHIVE_Z)
}

fn pressure_leak_pass_token_row() -> Part {
    let panel = centered_cube(
        "closed_run_start_readiness_gate_station_pressure_leak_pass_panel",
        PRESSURE_X,
        PRESSURE_Y,
        PRESSURE_Z,
    );
    let cuts = circular_token_grid(
        "pressure_leak_pass",
        PRESSURE_LEAK_TOKENS,
        PRESSURE_LEAK_COLS,
        9.5,
        60.0,
        38.0,
        (0.0, 0.0),
        8.0,
        PRESSURE_Z,
    );
    panel - cuts + front_row_label_strip("pressure_leak", PRESSURE_X, PRESSURE_Z)
}

fn environmental_map_token_row() -> Part {
    let panel = centered_cube(
        "closed_run_start_readiness_gate_station_environmental_map_panel",
        ENV_X,
        ENV_Y,
        ENV_Z,
    );
    let cuts = circular_token_grid(
        "environmental_map",
        ENV_MAP_TOKENS,
        ENV_MAP_COLS,
        8.5,
        44.0,
        38.0,
        (0.0, 0.0),
        8.0,
        ENV_Z,
    );
    panel - cuts + front_row_label_strip("environmental_map", ENV_X, ENV_Z)
}

fn front_row_label_strip(name: &str, panel_x: f64, panel_z: f64) -> Part {
    centered_cube(
        format!("closed_run_start_readiness_gate_station_{name}_label_strip"),
        panel_x - 42.0,
        16.0,
        4.0,
    )
    .translate(0.0, 38.0, panel_z / 2.0 + 2.0)
}

fn camera_evidence_bridge() -> Part {
    let left_post = bridge_post("left", -BRIDGE_SPAN_X / 2.0 + 28.0);
    let center_post = bridge_post("center", 0.0);
    let right_post = bridge_post("right", BRIDGE_SPAN_X / 2.0 - 28.0);
    let beam = centered_cube(
        "closed_run_start_readiness_gate_station_camera_bridge_beam",
        BRIDGE_SPAN_X,
        BRIDGE_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, 0.0, BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0);

    let mut cameras = Part::empty("closed_run_start_readiness_gate_station_camera_pods");
    for index in 0..CAMERA_COUNT {
        let x = -480.0 + index as f64 * 240.0;
        let pod_y = -BRIDGE_Y / 2.0 - 20.0;
        let pod_z = BRIDGE_UNDERSIDE_Z - 14.0;
        cameras = cameras
            + centered_cube(
                format!("closed_run_start_readiness_gate_station_evidence_camera_pod_{index}"),
                48.0,
                36.0,
                28.0,
            )
            .translate(x, pod_y, pod_z)
            + centered_cylinder(
                format!("closed_run_start_readiness_gate_station_evidence_camera_lens_{index}"),
                10.0,
                14.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, pod_y - 24.0, pod_z)
            + centered_cylinder(
                format!(
                    "closed_run_start_readiness_gate_station_evidence_camera_focus_ring_{index}"
                ),
                16.0,
                5.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, pod_y - 33.0, pod_z);
    }

    let mut leds = Part::empty("closed_run_start_readiness_gate_station_evidence_led_segments");
    for index in 0..LED_SEGMENTS {
        leds = leds
            + centered_cube(
                format!("closed_run_start_readiness_gate_station_evidence_led_segment_{index}"),
                92.0,
                12.0,
                8.0,
            )
            .translate(
                -540.0 + index as f64 * 120.0,
                BRIDGE_Y / 2.0 + 12.0,
                BRIDGE_UNDERSIDE_Z - 4.0,
            );
    }

    let evidence_card_rail = centered_cube(
        "closed_run_start_readiness_gate_station_evidence_card_rail",
        BRIDGE_SPAN_X - 120.0,
        12.0,
        18.0,
    )
    .translate(0.0, -BRIDGE_Y / 2.0 - 44.0, BRIDGE_UNDERSIDE_Z - 42.0);

    left_post + center_post + right_post + beam + cameras + leds + evidence_card_rail
}

fn bridge_post(name: &str, x: f64) -> Part {
    centered_cube(
        format!("closed_run_start_readiness_gate_station_camera_bridge_{name}_post"),
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_POST_Z,
    )
    .translate(x, 0.0, BRIDGE_POST_Z / 2.0)
}

fn robot_service_keepouts() -> Part {
    let front_robot = centered_cube(
        "closed_run_start_readiness_gate_station_front_robot_keepout_gauge",
        DECK_X - 220.0,
        FRONT_ROBOT_KEEP_OUT_Y,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + FRONT_ROBOT_KEEP_OUT_Y / 2.0 + 12.0,
        0.0,
    );
    let rear_service = centered_cube(
        "closed_run_start_readiness_gate_station_rear_service_keepout_gauge",
        DECK_X - 260.0,
        REAR_SERVICE_KEEP_OUT_Y,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        20.0,
        DECK_Y / 2.0 - REAR_SERVICE_KEEP_OUT_Y / 2.0 - 12.0,
        0.0,
    );
    let left_cart = centered_cube(
        "closed_run_start_readiness_gate_station_left_cart_keepout_gauge",
        LEFT_CART_KEEP_OUT_X,
        DECK_Y - 220.0,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        -DECK_X / 2.0 + LEFT_CART_KEEP_OUT_X / 2.0 + 10.0,
        -20.0,
        0.0,
    );
    let right_service = centered_cube(
        "closed_run_start_readiness_gate_station_right_service_keepout_gauge",
        RIGHT_SERVICE_KEEP_OUT_X,
        DECK_Y - 260.0,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        DECK_X / 2.0 - RIGHT_SERVICE_KEEP_OUT_X / 2.0 - 10.0,
        -48.0,
        0.0,
    );
    let top_clearance = centered_cube(
        "closed_run_start_readiness_gate_station_top_camera_service_clearance_gauge",
        BRIDGE_SPAN_X - 120.0,
        BRIDGE_Y + 150.0,
        10.0,
    )
    .translate(BRIDGE_CENTER.0, BRIDGE_CENTER.1, TOP_SERVICE_CLEARANCE_Z);

    front_robot + rear_service + left_cart + right_service + top_clearance
}

fn rectangular_slot_grid(
    name: &str,
    count: usize,
    cols: usize,
    slot_x: f64,
    slot_y: f64,
    pitch_x: f64,
    pitch_y: f64,
    origin: (f64, f64),
    depth: f64,
    panel_z: f64,
) -> Part {
    let mut slots = Part::empty(format!(
        "closed_run_start_readiness_gate_station_{name}_slot_grid"
    ));
    for index in 0..count {
        let (x, y) = grid_position(index, count, cols, pitch_x, pitch_y);
        slots = slots
            + centered_cube(
                format!("closed_run_start_readiness_gate_station_{name}_slot_{index}"),
                slot_x,
                slot_y,
                depth + 0.2,
            )
            .translate(
                origin.0 + x,
                origin.1 + y,
                panel_z / 2.0 - depth / 2.0 + 0.1,
            );
    }
    slots
}

fn circular_token_grid(
    name: &str,
    count: usize,
    cols: usize,
    token_r: f64,
    pitch_x: f64,
    pitch_y: f64,
    origin: (f64, f64),
    depth: f64,
    panel_z: f64,
) -> Part {
    let mut slots = Part::empty(format!(
        "closed_run_start_readiness_gate_station_{name}_token_grid"
    ));
    for index in 0..count {
        let (x, y) = grid_position(index, count, cols, pitch_x, pitch_y);
        slots = slots
            + centered_cylinder(
                format!("closed_run_start_readiness_gate_station_{name}_token_{index}"),
                token_r,
                depth + 0.2,
                32,
            )
            .translate(
                origin.0 + x,
                origin.1 + y,
                panel_z / 2.0 - depth / 2.0 + 0.1,
            );
    }
    slots
}

fn token_present_sensor_pips(
    name: &str,
    count: usize,
    cols: usize,
    pitch_x: f64,
    pitch_y: f64,
    origin: (f64, f64),
    panel_z: f64,
) -> Part {
    let mut pips = Part::empty(format!(
        "closed_run_start_readiness_gate_station_{name}_token_present_sensor_pips"
    ));
    for index in 0..count {
        let (x, y) = grid_position(index, count, cols, pitch_x, pitch_y);
        pips = pips
            + centered_cylinder(
                format!("closed_run_start_readiness_gate_station_{name}_sensor_pip_{index}"),
                2.2,
                4.0,
                16,
            )
            .translate(
                origin.0 + x + pitch_x * 0.34,
                origin.1 + y - pitch_y * 0.30,
                panel_z / 2.0 + 2.0,
            );
    }
    pips
}

fn rect_frame_xy(name: &str, outer_x: f64, outer_y: f64, rail: f64, z: f64) -> Part {
    let outer = centered_cube(format!("{name}_outer"), outer_x, outer_y, z);
    let inner = centered_cube(
        format!("{name}_inner_clearance"),
        outer_x - rail * 2.0,
        outer_y - rail * 2.0,
        z + 0.2,
    );
    outer - inner
}

fn grid_position(
    index: usize,
    count: usize,
    cols: usize,
    pitch_x: f64,
    pitch_y: f64,
) -> (f64, f64) {
    let rows = (count + cols - 1) / cols;
    let row = index / cols;
    let col = index % cols;
    let x = -((cols as f64 - 1.0) * pitch_x) / 2.0 + col as f64 * pitch_x;
    let y = ((rows as f64 - 1.0) * pitch_y) / 2.0 - row as f64 * pitch_y;
    (x, y)
}

fn main_footprints() -> [((f64, f64), f64, f64, &'static str); 12] {
    [
        (
            MATERIAL_CENTER,
            MATERIAL_PANEL_X,
            MATERIAL_PANEL_Y,
            "released_material_token_slots",
        ),
        (
            CASSETTE_CENTER,
            CASSETTE_DOCK_X,
            CASSETTE_DOCK_Y,
            "cassette_id_dock",
        ),
        (
            TOPOLOGY_CENTER,
            TOPOLOGY_X,
            TOPOLOGY_Y,
            "connector_topology_token_dock",
        ),
        (CAL_CENTER, CAL_X, CAL_Y, "calibration_certificate_lands"),
        (
            SENSOR_CENTER,
            SENSOR_X,
            SENSOR_Y,
            "sensor_cartridge_release_pockets",
        ),
        (
            MEDIA_CENTER,
            MEDIA_X,
            MEDIA_Y,
            "media_reagent_release_status_lanes",
        ),
        (SCAN_CENTER, SCAN_X, SCAN_Y, "operator_robot_scan_lands"),
        (
            LOCKOUT_CENTER,
            LOCKOUT_X,
            LOCKOUT_Y,
            "hold_reject_lockout_pocket",
        ),
        (RECIPE_CENTER, RECIPE_X, RECIPE_Y, "recipe_badge_slot"),
        (
            ARCHIVE_CENTER,
            ARCHIVE_X,
            ARCHIVE_Y,
            "sample_archive_readiness_slots",
        ),
        (
            PRESSURE_CENTER,
            PRESSURE_X,
            PRESSURE_Y,
            "pressure_leak_pass_token_row",
        ),
        (ENV_CENTER, ENV_X, ENV_Y, "environmental_map_token_row"),
    ]
}

fn deck_mount_points() -> [(f64, f64); 10] {
    [
        (-(DECK_X / 2.0 - 42.0), -(DECK_Y / 2.0 - 42.0)),
        (DECK_X / 2.0 - 42.0, -(DECK_Y / 2.0 - 42.0)),
        (-(DECK_X / 2.0 - 42.0), DECK_Y / 2.0 - 42.0),
        (DECK_X / 2.0 - 42.0, DECK_Y / 2.0 - 42.0),
        (0.0, -(DECK_Y / 2.0 - 42.0)),
        (0.0, DECK_Y / 2.0 - 42.0),
        (-(DECK_X / 2.0 - 42.0), 0.0),
        (DECK_X / 2.0 - 42.0, 0.0),
        (-360.0, DECK_Y / 2.0 - 42.0),
        (360.0, DECK_Y / 2.0 - 42.0),
    ]
}

fn bridge_clearance_above_deck() -> f64 {
    BRIDGE_UNDERSIDE_Z
}

#[derive(Clone, Copy)]
struct Rect {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

fn rect(center: (f64, f64), w: f64, h: f64) -> Rect {
    Rect {
        x: center.0,
        y: center.1,
        w,
        h,
    }
}

fn fits_on_deck(center: (f64, f64), x: f64, y: f64, margin: f64) -> bool {
    center.0.abs() + x / 2.0 <= DECK_X / 2.0 - margin
        && center.1.abs() + y / 2.0 <= DECK_Y / 2.0 - margin
}

fn rects_overlap(a: Rect, b: Rect) -> bool {
    let ax0 = a.x - a.w / 2.0;
    let ax1 = a.x + a.w / 2.0;
    let ay0 = a.y - a.h / 2.0;
    let ay1 = a.y + a.h / 2.0;
    let bx0 = b.x - b.w / 2.0;
    let bx1 = b.x + b.w / 2.0;
    let by0 = b.y - b.h / 2.0;
    let by1 = b.y + b.h / 2.0;

    ax0 < bx1 && ax1 > bx0 && ay0 < by1 && ay1 > by0
}

fn horizontal_gap(a: Rect, b: Rect) -> f64 {
    let ax0 = a.x - a.w / 2.0;
    let ax1 = a.x + a.w / 2.0;
    let bx0 = b.x - b.w / 2.0;
    let bx1 = b.x + b.w / 2.0;

    if ax1 < bx0 {
        bx0 - ax1
    } else if bx1 < ax0 {
        ax0 - bx1
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 16);
        for path in OUTPUTS {
            assert!(path.starts_with("output/closed_run_start_readiness_gate_station_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn interlock_feature_list_covers_required_prerequisites() {
        assert_eq!(REQUIRED_INTERLOCK_FEATURES.len(), 14);
        assert!(REQUIRED_INTERLOCK_FEATURES.contains(&"released_material_token_slots"));
        assert!(REQUIRED_INTERLOCK_FEATURES.contains(&"cassette_id_dock"));
        assert!(REQUIRED_INTERLOCK_FEATURES.contains(&"connector_topology_token_dock"));
        assert!(REQUIRED_INTERLOCK_FEATURES.contains(&"calibration_certificate_lands"));
        assert!(REQUIRED_INTERLOCK_FEATURES.contains(&"sensor_cartridge_release_pockets"));
        assert!(REQUIRED_INTERLOCK_FEATURES.contains(&"hold_reject_lockout_pocket"));
        assert!(REQUIRED_INTERLOCK_FEATURES.contains(&"recipe_badge_slot"));
        assert!(REQUIRED_INTERLOCK_FEATURES.contains(&"camera_evidence_bridge"));
    }

    #[test]
    fn physical_readiness_counts_are_batch_scale() {
        assert_eq!(CHIP_COUNT, 20);
        assert_eq!(MATERIAL_TOKEN_COUNT, 24);
        assert_eq!(TOPOLOGY_TOKEN_COUNT, 24);
        assert_eq!(CERT_CARD_COUNT, 10);
        assert_eq!(SENSOR_CARTRIDGE_COUNT, 12);
        assert_eq!(ARCHIVE_READY_SLOTS, 16);
        assert_eq!(PRESSURE_LEAK_TOKENS, 12);
        assert_eq!(ENV_MAP_TOKENS, 12);
        assert!(MATERIAL_TOKEN_COUNT >= CHIP_COUNT);
    }

    #[test]
    fn panel_footprints_fit_without_overlap() {
        let footprints = main_footprints();
        for (center, x, y, name) in footprints {
            assert!(
                fits_on_deck(center, x, y, 12.0),
                "{name} should fit on the deck"
            );
        }

        for left in 0..footprints.len() {
            for right in left + 1..footprints.len() {
                let a = footprints[left];
                let b = footprints[right];
                assert!(
                    !rects_overlap(rect(a.0, a.1, a.2), rect(b.0, b.1, b.2)),
                    "{} should not overlap {}",
                    a.3,
                    b.3
                );
            }
        }
    }

    #[test]
    fn cassette_dock_covers_full_rev_c_cassette_envelope() {
        assert!(CASSETTE_X > CHIP_ARRAY_X + CASSETTE_MARGIN_X);
        assert!(CASSETTE_Y > CHIP_ARRAY_Y + CASSETTE_MARGIN_Y);
        assert!(CASSETTE_X < CASSETTE_DOCK_X - 42.0);
        assert!(CASSETTE_Y < CASSETTE_DOCK_Y - 42.0);
        assert_eq!(CASSETTE_BARCODE_LANDS, 6);
    }

    #[test]
    fn lockout_and_evidence_bridge_have_clearances() {
        assert!(LOCKOUT_WALL >= 10.0);
        assert!(
            horizontal_gap(
                rect(LOCKOUT_CENTER, LOCKOUT_X, LOCKOUT_Y),
                rect(MEDIA_CENTER, MEDIA_X, MEDIA_Y)
            ) >= LOCKOUT_SEGREGATION_MIN
        );
        assert!(bridge_clearance_above_deck() >= 170.0);
        assert!(fits_on_deck(
            BRIDGE_CENTER,
            BRIDGE_SPAN_X,
            BRIDGE_Y + 160.0,
            12.0
        ));
        assert_eq!(CAMERA_COUNT, 5);
        assert_eq!(LED_SEGMENTS, 10);
    }

    #[test]
    fn robot_and_service_keepouts_reserve_access() {
        assert!(FRONT_ROBOT_KEEP_OUT_Y >= 120.0);
        assert!(REAR_SERVICE_KEEP_OUT_Y >= 110.0);
        assert!(LEFT_CART_KEEP_OUT_X >= 100.0);
        assert!(RIGHT_SERVICE_KEEP_OUT_X >= 120.0);
        assert!(TOP_SERVICE_CLEARANCE_Z >= 250.0);
    }
}
