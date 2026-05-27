use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed incubator door/opening recovery challenge station.
//
// This is validation-support CAD for a no-cell incubator challenge fixture. It
// models a sealed cassette surrogate rack, door opening event gauge, logger
// pockets, baffle witness targets, condensate capture, barcode run tokens, and
// physical release/hold/reject evidence lanes. It is not a cell handling
// protocol, incubator operating limit, or product release criterion.

const OUTPUTS: [&str; 10] = [
    "output/closed_incubator_door_recovery_challenge_station_deck.stl",
    "output/closed_incubator_door_recovery_challenge_station_sealed_cassette_surrogate_rack.stl",
    "output/closed_incubator_door_recovery_challenge_station_quick_open_door_event_gauge.stl",
    "output/closed_incubator_door_recovery_challenge_station_temp_co2_rh_logger_pockets.stl",
    "output/closed_incubator_door_recovery_challenge_station_airflow_baffle_witness_targets.stl",
    "output/closed_incubator_door_recovery_challenge_station_condensate_drip_capture.stl",
    "output/closed_incubator_door_recovery_challenge_station_barcode_run_tokens.stl",
    "output/closed_incubator_door_recovery_challenge_station_release_hold_reject_evidence_lanes.stl",
    "output/closed_incubator_door_recovery_challenge_station_robot_service_keepout_gauges.stl",
    "output/closed_incubator_door_recovery_challenge_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 7] = [
    "sealed_cassette_surrogate_rack",
    "quick_open_door_event_gauge",
    "temp_co2_rh_logger_pockets",
    "airflow_baffle_witness_targets",
    "condensate_drip_capture",
    "barcode_run_tokens",
    "release_hold_reject_evidence_lanes",
];

const DECK_X: f64 = 1160.0;
const DECK_Y: f64 = 820.0;
const DECK_Z: f64 = 18.0;
const DECK_RIM_W: f64 = 16.0;
const DECK_RIM_Z: f64 = 28.0;
const DECK_BASIN_DEPTH: f64 = 5.0;
const MOUNT_SLOT_COUNT: usize = 8;
const DATUM_TARGET_COUNT: usize = 4;

const RACK_COLS: usize = 3;
const RACK_ROWS: usize = 3;
const RACK_SLOT_COUNT: usize = RACK_COLS * RACK_ROWS;
const RACK_SLOT_X: f64 = 82.0;
const RACK_SLOT_Y: f64 = 118.0;
const RACK_SLOT_RECESS_Z: f64 = 16.0;
const RACK_SLOT_PITCH_X: f64 = 108.0;
const RACK_SLOT_PITCH_Y: f64 = 136.0;
const RACK_MARGIN_X: f64 = 64.0;
const RACK_MARGIN_Y: f64 = 64.0;
const RACK_X: f64 = (RACK_COLS as f64 - 1.0) * RACK_SLOT_PITCH_X + RACK_SLOT_X + RACK_MARGIN_X;
const RACK_Y: f64 = (RACK_ROWS as f64 - 1.0) * RACK_SLOT_PITCH_Y + RACK_SLOT_Y + RACK_MARGIN_Y;
const RACK_Z: f64 = 34.0;
const RACK_GASKET_W: f64 = 8.0;
const RACK_GASKET_Z: f64 = 7.0;
const RACK_LATCH_COUNT: usize = 6;
const RACK_DATUM_PIN_D: f64 = 7.5;

const DOOR_GAUGE_X: f64 = 430.0;
const DOOR_GAUGE_Y: f64 = 150.0;
const DOOR_GAUGE_Z: f64 = 16.0;
const DOOR_EVENT_TICKS: usize = 7;
const DOOR_TICK_PITCH_X: f64 = 52.0;
const DOOR_TICK_W: f64 = 6.0;
const DOOR_TICK_MAX_Z: f64 = 40.0;
const DOOR_WINDOW_X: f64 = 166.0;
const DOOR_WINDOW_Y: f64 = 46.0;
const DOOR_HINGE_BAR_D: f64 = 18.0;
const DOOR_LATCH_STOPS: usize = 2;
const DOOR_SWING_WITNESS_COUNT: usize = 5;

const LOGGER_POCKET_TYPES: usize = 3;
const LOGGER_REFERENCE_POCKETS: usize = 1;
const LOGGER_POCKET_COUNT: usize = LOGGER_POCKET_TYPES + LOGGER_REFERENCE_POCKETS;
const LOGGER_PANEL_X: f64 = 352.0;
const LOGGER_PANEL_Y: f64 = 156.0;
const LOGGER_PANEL_Z: f64 = 32.0;
const LOGGER_POCKET_X: f64 = 68.0;
const LOGGER_POCKET_Y: f64 = 82.0;
const LOGGER_POCKET_DEPTH: f64 = 12.0;
const LOGGER_POCKET_PITCH_X: f64 = 82.0;
const LOGGER_CABLE_NOTCH_W: f64 = 12.0;
const LOGGER_SEAL_LANDS: usize = 8;

const BAFFLE_PANEL_X: f64 = 392.0;
const BAFFLE_PANEL_Y: f64 = 260.0;
const BAFFLE_PANEL_Z: f64 = 20.0;
const BAFFLE_ROWS: usize = 3;
const BAFFLE_COLS: usize = 4;
const BAFFLE_WITNESS_TARGETS: usize = BAFFLE_ROWS * BAFFLE_COLS;
const BAFFLE_TARGET_D: f64 = 24.0;
const BAFFLE_TARGET_RIM_D: f64 = 34.0;
const BAFFLE_PITCH_X: f64 = 76.0;
const BAFFLE_PITCH_Y: f64 = 62.0;
const BAFFLE_VANE_COUNT: usize = 5;
const BAFFLE_VANE_X: f64 = 298.0;
const BAFFLE_VANE_Y: f64 = 8.0;
const BAFFLE_VANE_Z: f64 = 34.0;

const CONDENSATE_TRAY_X: f64 = 410.0;
const CONDENSATE_TRAY_Y: f64 = 184.0;
const CONDENSATE_TRAY_Z: f64 = 24.0;
const CONDENSATE_BASIN_DEPTH: f64 = 12.0;
const CONDENSATE_RIM_W: f64 = 10.0;
const CONDENSATE_RIM_Z: f64 = 24.0;
const DRIP_CAPTURE_LANES: usize = 5;
const DRIP_LANE_PITCH_X: f64 = 68.0;
const DRIP_LANE_W: f64 = 44.0;
const DRIP_CUP_COUNT: usize = 3;
const DRIP_CUP_D: f64 = 58.0;
const DRIP_CHALLENGE_ML: f64 = 95.0;

const TOKEN_PLATE_X: f64 = 260.0;
const TOKEN_PLATE_Y: f64 = 150.0;
const TOKEN_PLATE_Z: f64 = 14.0;
const RUN_TOKEN_COUNT: usize = 8;
const TOKEN_ROWS: usize = 2;
const TOKEN_COLS: usize = 4;
const TOKEN_D: f64 = 20.0;
const TOKEN_RIM_D: f64 = 30.0;
const TOKEN_PITCH_X: f64 = 58.0;
const TOKEN_PITCH_Y: f64 = 62.0;
const BARCODE_LAND_X: f64 = 50.0;
const BARCODE_LAND_Y: f64 = 18.0;

const EVIDENCE_LANES: usize = 3;
const EVIDENCE_SLOTS_PER_LANE: usize = 4;
const EVIDENCE_PANEL_X: f64 = 350.0;
const EVIDENCE_PANEL_Y: f64 = 210.0;
const EVIDENCE_PANEL_Z: f64 = 26.0;
const EVIDENCE_LANE_PITCH_X: f64 = 108.0;
const EVIDENCE_SLOT_PITCH_Y: f64 = 42.0;
const EVIDENCE_SLOT_X: f64 = 78.0;
const EVIDENCE_SLOT_Y: f64 = 30.0;
const EVIDENCE_SLOT_DEPTH: f64 = 8.0;
const EVIDENCE_WALL_W: f64 = 7.0;
const EVIDENCE_MIN_SEGREGATION_GAP: f64 = 22.0;

const ROBOT_KEEP_OUT_X: f64 = 940.0;
const ROBOT_KEEP_OUT_Y: f64 = 86.0;
const ROBOT_KEEP_OUT_Z: f64 = 9.0;
const SERVICE_KEEP_OUT_X: f64 = 128.0;
const SERVICE_KEEP_OUT_Y: f64 = 560.0;
const SERVICE_KEEP_OUT_Z: f64 = 38.0;
const DOOR_SWING_KEEP_OUT_X: f64 = 480.0;
const DOOR_SWING_KEEP_OUT_Y: f64 = 92.0;
const DOOR_SWING_KEEP_OUT_Z: f64 = 42.0;

const RACK_CENTER: (f64, f64) = (-335.0, 55.0);
const DOOR_GAUGE_CENTER: (f64, f64) = (80.0, 302.0);
const LOGGER_CENTER: (f64, f64) = (-382.0, -295.0);
const BAFFLE_CENTER: (f64, f64) = (228.0, 70.0);
const CONDENSATE_CENTER: (f64, f64) = (0.0, -295.0);
const TOKEN_CENTER: (f64, f64) = (430.0, 300.0);
const EVIDENCE_CENTER: (f64, f64) = (380.0, -185.0);

#[derive(Clone, Copy)]
struct Rect {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = station_deck();
    export(OUTPUTS[0], &deck);

    let rack = sealed_cassette_surrogate_rack();
    export(OUTPUTS[1], &rack);

    let door = quick_open_door_event_gauge();
    export(OUTPUTS[2], &door);

    let loggers = temp_co2_rh_logger_pockets();
    export(OUTPUTS[3], &loggers);

    let baffles = airflow_baffle_witness_targets();
    export(OUTPUTS[4], &baffles);

    let condensate = condensate_drip_capture();
    export(OUTPUTS[5], &condensate);

    let tokens = barcode_run_tokens();
    export(OUTPUTS[6], &tokens);

    let lanes = release_hold_reject_evidence_lanes();
    export(OUTPUTS[7], &lanes);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[8], &keepouts);

    let assembly = deck
        + rack.translate(RACK_CENTER.0, RACK_CENTER.1, deck_mount_z(RACK_Z))
        + door.translate(
            DOOR_GAUGE_CENTER.0,
            DOOR_GAUGE_CENTER.1,
            deck_mount_z(DOOR_GAUGE_Z),
        )
        + loggers.translate(
            LOGGER_CENTER.0,
            LOGGER_CENTER.1,
            deck_mount_z(LOGGER_PANEL_Z),
        )
        + baffles.translate(
            BAFFLE_CENTER.0,
            BAFFLE_CENTER.1,
            deck_mount_z(BAFFLE_PANEL_Z),
        )
        + condensate.translate(
            CONDENSATE_CENTER.0,
            CONDENSATE_CENTER.1,
            deck_mount_z(CONDENSATE_TRAY_Z),
        )
        + tokens.translate(TOKEN_CENTER.0, TOKEN_CENTER.1, deck_mount_z(TOKEN_PLATE_Z))
        + lanes.translate(
            EVIDENCE_CENTER.0,
            EVIDENCE_CENTER.1,
            deck_mount_z(EVIDENCE_PANEL_Z),
        )
        + keepouts.translate(0.0, 0.0, DECK_Z / 2.0 + ROBOT_KEEP_OUT_Z / 2.0);
    export(OUTPUTS[9], &assembly);

    println!();
    println!("Closed incubator door recovery challenge station:");
    println!(
        "  Deck:                       {DECK_X:.0}mm x {DECK_Y:.0}mm evidence tray with {DATUM_TARGET_COUNT} datum targets and {MOUNT_SLOT_COUNT} mount slots"
    );
    println!(
        "  Cassette surrogate rack:    {RACK_COLS} x {RACK_ROWS} sealed positions with gasket witness ribs, latch pads, and datum pins"
    );
    println!(
        "  Door event gauge:           {DOOR_EVENT_TICKS} quick-open tick marks, {DOOR_SWING_WITNESS_COUNT} swing witnesses, and {DOOR_LATCH_STOPS} latch stops"
    );
    println!(
        "  Environmental evidence:     {LOGGER_POCKET_COUNT} logger pockets covering temp/CO2/RH/reference with {LOGGER_SEAL_LANDS} seal lands"
    );
    println!(
        "  Airflow and condensate:     {BAFFLE_WITNESS_TARGETS} baffle witness targets, {BAFFLE_VANE_COUNT} vanes, {DRIP_CAPTURE_LANES} drip lanes, {DRIP_CUP_COUNT} capture cups"
    );
    println!(
        "  Run record:                 {RUN_TOKEN_COUNT} barcode run tokens and {EVIDENCE_LANES} release/hold/reject lanes with {EVIDENCE_SLOTS_PER_LANE} evidence slots each"
    );
    println!("  Required feature groups:    {}", REQUIRED_FEATURES.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn deck_mount_z(part_z: f64) -> f64 {
    DECK_Z / 2.0 + part_z / 2.0
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 10, "unexpected STL output count");
    assert_eq!(REQUIRED_FEATURES.len(), 7);
    assert_eq!(
        RACK_SLOT_COUNT, 9,
        "rack must hold nine sealed cassette surrogates"
    );
    assert_eq!(
        LOGGER_POCKET_TYPES, 3,
        "logger pocket group must include temp, CO2, and RH pockets"
    );
    assert_eq!(EVIDENCE_LANES, 3, "release/hold/reject lane count changed");
    assert_eq!(RUN_TOKEN_COUNT, TOKEN_ROWS * TOKEN_COLS);
    assert!(door_event_span_mm() >= 300.0);
    assert!(max_door_tick_height_mm() <= DOOR_TICK_MAX_Z);
    assert!(door_recovery_window_area_mm2() >= 7_000.0);
    assert!(condensate_capture_volume_ml() > DRIP_CHALLENGE_ML);
    assert!(evidence_lane_gap() >= EVIDENCE_MIN_SEGREGATION_GAP);

    for module in module_rects() {
        assert!(
            fits_on_deck(module, 20.0),
            "{} module exceeds deck envelope",
            module_name(module)
        );
    }

    assert!(!rects_overlap(rack_rect(), door_rect()));
    assert!(!rects_overlap(door_rect(), token_rect()));
    assert!(!rects_overlap(logger_rect(), condensate_rect()));
    assert!(!rects_overlap(condensate_rect(), evidence_rect()));
    assert!(!rects_overlap(baffle_rect(), evidence_rect()));
}

fn station_deck() -> Part {
    let deck = centered_cube("closed_door_recovery_station_deck", DECK_X, DECK_Y, DECK_Z);
    let shallow_basin = centered_cube(
        "closed_door_recovery_station_secondary_basin_cut",
        DECK_X - 112.0,
        DECK_Y - 104.0,
        DECK_BASIN_DEPTH + 0.6,
    )
    .translate(0.0, -8.0, DECK_Z / 2.0 - DECK_BASIN_DEPTH / 2.0 + 0.3);

    deck - shallow_basin - deck_mount_slots()
        + deck_perimeter_rims()
        + deck_datum_targets()
        + deck_recovery_timeline_rails()
}

fn deck_mount_slots() -> Part {
    let mut slots = Part::empty("closed_door_recovery_deck_mount_slots");
    for (i, (x, y)) in [
        (-500.0, -350.0),
        (-250.0, -350.0),
        (0.0, -350.0),
        (250.0, -350.0),
        (500.0, -350.0),
        (-500.0, 350.0),
        (0.0, 350.0),
        (500.0, 350.0),
    ]
    .iter()
    .enumerate()
    {
        let hole = centered_cylinder(
            format!("closed_door_recovery_deck_mount_hole_{i}"),
            3.4,
            DECK_Z + 3.0,
            28,
        )
        .translate(*x, *y, 0.0);
        let slot = centered_cube(
            format!("closed_door_recovery_deck_mount_slot_{i}"),
            24.0,
            7.0,
            DECK_Z + 3.0,
        )
        .translate(*x, *y, 0.0);
        slots = slots + hole + slot;
    }
    slots
}

fn deck_perimeter_rims() -> Part {
    let front = centered_cube(
        "closed_door_recovery_front_condensate_rim",
        DECK_X,
        DECK_RIM_W,
        DECK_RIM_Z,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + DECK_RIM_W / 2.0,
        DECK_Z / 2.0 + DECK_RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "closed_door_recovery_rear_condensate_rim",
        DECK_X,
        DECK_RIM_W,
        DECK_RIM_Z,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - DECK_RIM_W / 2.0,
        DECK_Z / 2.0 + DECK_RIM_Z / 2.0,
    );
    let left = centered_cube(
        "closed_door_recovery_left_condensate_rim",
        DECK_RIM_W,
        DECK_Y,
        DECK_RIM_Z,
    )
    .translate(
        -DECK_X / 2.0 + DECK_RIM_W / 2.0,
        0.0,
        DECK_Z / 2.0 + DECK_RIM_Z / 2.0,
    );
    let right = centered_cube(
        "closed_door_recovery_right_condensate_rim",
        DECK_RIM_W,
        DECK_Y,
        DECK_RIM_Z,
    )
    .translate(
        DECK_X / 2.0 - DECK_RIM_W / 2.0,
        0.0,
        DECK_Z / 2.0 + DECK_RIM_Z / 2.0,
    );

    front + rear + left + right
}

fn deck_datum_targets() -> Part {
    let mut targets = Part::empty("closed_door_recovery_robot_datum_targets");
    for (i, (x, y)) in [
        (-520.0, 362.0),
        (520.0, 362.0),
        (-520.0, -362.0),
        (520.0, -362.0),
    ]
    .iter()
    .enumerate()
    {
        targets = targets
            + fiducial_disc(&format!("closed_door_recovery_robot_datum_{i}")).translate(
                *x,
                *y,
                DECK_Z / 2.0 + 1.8,
            );
    }
    targets
}

fn deck_recovery_timeline_rails() -> Part {
    let mut rails = Part::empty("closed_door_recovery_timeline_reference_rails");
    for (i, y) in [-118.0, -56.0, 6.0, 68.0, 130.0].iter().enumerate() {
        rails = rails
            + centered_cube(
                format!("closed_door_recovery_air_recovery_timeline_rail_{i}"),
                238.0,
                4.0,
                4.0,
            )
            .translate(220.0, *y, DECK_Z / 2.0 + 2.0);
    }
    rails
}

fn sealed_cassette_surrogate_rack() -> Part {
    let base = centered_cube(
        "closed_door_recovery_sealed_cassette_surrogate_rack_base",
        RACK_X,
        RACK_Y,
        RACK_Z,
    );
    let slot_reliefs = rack_slot_reliefs();

    base - slot_reliefs
        + rack_perimeter_gasket()
        + rack_slot_gasket_witnesses()
        + rack_latch_pads()
        + rack_datum_pins()
        + rack_pressure_equalization_witnesses()
}

fn rack_slot_reliefs() -> Part {
    let mut reliefs = Part::empty("closed_door_recovery_rack_slot_reliefs");
    for row in 0..RACK_ROWS {
        for col in 0..RACK_COLS {
            let slot = row * RACK_COLS + col;
            let (x, y) = rack_slot_center(row, col);
            let relief = centered_cube(
                format!("closed_door_recovery_slot_{slot}_sealed_cassette_relief"),
                RACK_SLOT_X,
                RACK_SLOT_Y,
                RACK_SLOT_RECESS_Z + 1.0,
            )
            .translate(x, y, RACK_Z / 2.0 - RACK_SLOT_RECESS_Z / 2.0 + 0.5);
            let barcode_window = centered_cube(
                format!("closed_door_recovery_slot_{slot}_cassette_barcode_window"),
                52.0,
                12.0,
                RACK_SLOT_RECESS_Z + 2.0,
            )
            .translate(
                x,
                y - RACK_SLOT_Y / 2.0 + 18.0,
                RACK_Z / 2.0 - RACK_SLOT_RECESS_Z / 2.0 + 0.6,
            );
            reliefs = reliefs + relief + barcode_window;
        }
    }
    reliefs
}

fn rack_perimeter_gasket() -> Part {
    let front = centered_cube(
        "closed_door_recovery_rack_front_gasket_witness",
        RACK_X,
        RACK_GASKET_W,
        RACK_GASKET_Z,
    )
    .translate(
        0.0,
        -RACK_Y / 2.0 + RACK_GASKET_W / 2.0,
        RACK_Z / 2.0 + RACK_GASKET_Z / 2.0,
    );
    let rear = centered_cube(
        "closed_door_recovery_rack_rear_gasket_witness",
        RACK_X,
        RACK_GASKET_W,
        RACK_GASKET_Z,
    )
    .translate(
        0.0,
        RACK_Y / 2.0 - RACK_GASKET_W / 2.0,
        RACK_Z / 2.0 + RACK_GASKET_Z / 2.0,
    );
    let left = centered_cube(
        "closed_door_recovery_rack_left_gasket_witness",
        RACK_GASKET_W,
        RACK_Y,
        RACK_GASKET_Z,
    )
    .translate(
        -RACK_X / 2.0 + RACK_GASKET_W / 2.0,
        0.0,
        RACK_Z / 2.0 + RACK_GASKET_Z / 2.0,
    );
    let right = centered_cube(
        "closed_door_recovery_rack_right_gasket_witness",
        RACK_GASKET_W,
        RACK_Y,
        RACK_GASKET_Z,
    )
    .translate(
        RACK_X / 2.0 - RACK_GASKET_W / 2.0,
        0.0,
        RACK_Z / 2.0 + RACK_GASKET_Z / 2.0,
    );

    front + rear + left + right
}

fn rack_slot_gasket_witnesses() -> Part {
    let mut witnesses = Part::empty("closed_door_recovery_slot_gasket_witnesses");
    for row in 0..RACK_ROWS {
        for col in 0..RACK_COLS {
            let slot = row * RACK_COLS + col;
            let (x, y) = rack_slot_center(row, col);
            let front = centered_cube(
                format!("closed_door_recovery_slot_{slot}_front_seal_witness"),
                RACK_SLOT_X + 18.0,
                4.0,
                6.0,
            )
            .translate(x, y - RACK_SLOT_Y / 2.0 - 8.0, RACK_Z / 2.0 + 3.0);
            let rear = centered_cube(
                format!("closed_door_recovery_slot_{slot}_rear_seal_witness"),
                RACK_SLOT_X + 18.0,
                4.0,
                6.0,
            )
            .translate(x, y + RACK_SLOT_Y / 2.0 + 8.0, RACK_Z / 2.0 + 3.0);
            witnesses = witnesses + front + rear;
        }
    }
    witnesses
}

fn rack_latch_pads() -> Part {
    let mut pads = Part::empty("closed_door_recovery_rack_latch_pads");
    for i in 0..RACK_LATCH_COUNT {
        let x = centered_index(i % 3, 3, 116.0);
        let y = if i < 3 {
            -RACK_Y / 2.0 + 24.0
        } else {
            RACK_Y / 2.0 - 24.0
        };
        pads = pads
            + centered_cube(
                format!("closed_door_recovery_rack_latch_pad_{i}"),
                44.0,
                22.0,
                8.0,
            )
            .translate(x, y, RACK_Z / 2.0 + 4.0);
    }
    pads
}

fn rack_datum_pins() -> Part {
    let mut pins = Part::empty("closed_door_recovery_rack_datum_pins");
    for (i, (x, y)) in [
        (-RACK_X / 2.0 + 26.0, RACK_Y / 2.0 - 26.0),
        (RACK_X / 2.0 - 26.0, RACK_Y / 2.0 - 26.0),
        (-RACK_X / 2.0 + 26.0, -RACK_Y / 2.0 + 26.0),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("closed_door_recovery_rack_datum_boss_{i}"),
            RACK_DATUM_PIN_D,
            8.0,
            32,
        )
        .translate(*x, *y, RACK_Z / 2.0 + 4.0);
        let pilot = centered_cylinder(
            format!("closed_door_recovery_rack_datum_pin_{i}"),
            RACK_DATUM_PIN_D * 0.42,
            14.0,
            28,
        )
        .translate(*x, *y, RACK_Z / 2.0 + 7.0);
        pins = pins + boss + pilot;
    }
    pins
}

fn rack_pressure_equalization_witnesses() -> Part {
    let mut witnesses = Part::empty("closed_door_recovery_pressure_equalization_witnesses");
    for row in 0..RACK_ROWS {
        let y = centered_index(row, RACK_ROWS, RACK_SLOT_PITCH_Y);
        witnesses = witnesses
            + centered_cube(
                format!("closed_door_recovery_row_{row}_pressure_witness_strip"),
                RACK_X - 78.0,
                5.0,
                5.0,
            )
            .translate(0.0, y, RACK_Z / 2.0 + 2.5);
    }
    witnesses
}

fn quick_open_door_event_gauge() -> Part {
    let plate = centered_cube(
        "closed_door_recovery_quick_open_event_gauge_plate",
        DOOR_GAUGE_X,
        DOOR_GAUGE_Y,
        DOOR_GAUGE_Z,
    );
    let window = centered_cube(
        "closed_door_recovery_quick_open_visible_door_window",
        DOOR_WINDOW_X,
        DOOR_WINDOW_Y,
        DOOR_GAUGE_Z + 2.0,
    )
    .translate(102.0, 0.0, 0.0);

    plate - window
        + door_hinge_bar()
        + door_event_ticks()
        + door_swing_witnesses()
        + door_latch_stop_pads()
        + door_open_timer_puck_lands()
}

fn door_hinge_bar() -> Part {
    let hinge = centered_cylinder(
        "closed_door_recovery_quick_open_hinge_axis_bar",
        DOOR_HINGE_BAR_D / 2.0,
        DOOR_GAUGE_Y + 28.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        -DOOR_GAUGE_X / 2.0 + 34.0,
        0.0,
        DOOR_GAUGE_Z / 2.0 + DOOR_HINGE_BAR_D / 2.0,
    );
    let hinge_base = centered_cube(
        "closed_door_recovery_quick_open_hinge_mount_land",
        44.0,
        DOOR_GAUGE_Y,
        8.0,
    )
    .translate(-DOOR_GAUGE_X / 2.0 + 34.0, 0.0, DOOR_GAUGE_Z / 2.0 + 4.0);
    hinge + hinge_base
}

fn door_event_ticks() -> Part {
    let mut ticks = Part::empty("closed_door_recovery_quick_open_event_ticks");
    for tick in 0..DOOR_EVENT_TICKS {
        let x = door_tick_x(tick);
        let height = 16.0 + tick as f64 * 4.0;
        let mark = centered_cube(
            format!("closed_door_recovery_event_tick_{tick}"),
            DOOR_TICK_W,
            52.0,
            height,
        )
        .translate(x, -42.0, DOOR_GAUGE_Z / 2.0 + height / 2.0);
        let base_land = centered_cube(
            format!("closed_door_recovery_event_tick_{tick}_barcode_time_land"),
            34.0,
            14.0,
            4.0,
        )
        .translate(x, -DOOR_GAUGE_Y / 2.0 + 18.0, DOOR_GAUGE_Z / 2.0 + 2.0);
        ticks = ticks + mark + base_land;
    }
    ticks
}

fn door_swing_witnesses() -> Part {
    let mut witnesses = Part::empty("closed_door_recovery_door_swing_witnesses");
    for i in 0..DOOR_SWING_WITNESS_COUNT {
        let x = -88.0 + i as f64 * 48.0;
        let y = 38.0 + i as f64 * 5.0;
        let bar = centered_cube(
            format!("closed_door_recovery_swing_arc_witness_bar_{i}"),
            42.0,
            5.0,
            10.0,
        )
        .rotate(0.0, 0.0, swing_tick_angle_deg(i))
        .translate(x, y, DOOR_GAUGE_Z / 2.0 + 5.0);
        let target = fiducial_disc(&format!("closed_door_recovery_swing_angle_target_{i}"))
            .translate(x + 22.0, y + 14.0, DOOR_GAUGE_Z / 2.0 + 2.0);
        witnesses = witnesses + bar + target;
    }
    witnesses
}

fn door_latch_stop_pads() -> Part {
    let mut stops = Part::empty("closed_door_recovery_latch_stop_pads");
    for i in 0..DOOR_LATCH_STOPS {
        let y = if i == 0 { -42.0 } else { 42.0 };
        let pad = centered_cube(
            format!("closed_door_recovery_quick_open_latch_stop_pad_{i}"),
            58.0,
            22.0,
            16.0,
        )
        .translate(DOOR_GAUGE_X / 2.0 - 46.0, y, DOOR_GAUGE_Z / 2.0 + 8.0);
        let strike = centered_cylinder(
            format!("closed_door_recovery_quick_open_latch_stop_pin_{i}"),
            6.0,
            18.0,
            28,
        )
        .translate(DOOR_GAUGE_X / 2.0 - 46.0, y, DOOR_GAUGE_Z / 2.0 + 17.0);
        stops = stops + pad + strike;
    }
    stops
}

fn door_open_timer_puck_lands() -> Part {
    let mut lands = Part::empty("closed_door_recovery_timer_puck_lands");
    for (i, x) in [-144.0, -104.0, -64.0].iter().enumerate() {
        lands = lands
            + centered_cylinder(
                format!("closed_door_recovery_timer_puck_land_{i}"),
                13.0,
                5.0,
                36,
            )
            .translate(*x, DOOR_GAUGE_Y / 2.0 - 28.0, DOOR_GAUGE_Z / 2.0 + 2.5);
    }
    lands
}

fn temp_co2_rh_logger_pockets() -> Part {
    let panel = centered_cube(
        "closed_door_recovery_temp_co2_rh_logger_panel",
        LOGGER_PANEL_X,
        LOGGER_PANEL_Y,
        LOGGER_PANEL_Z,
    );
    panel - logger_pocket_recesses()
        + logger_retainer_clips()
        + logger_seal_lands()
        + logger_cable_strain_reliefs()
}

fn logger_pocket_recesses() -> Part {
    let mut recesses = Part::empty("closed_door_recovery_logger_pocket_recesses");
    for pocket in 0..LOGGER_POCKET_COUNT {
        let x = logger_pocket_x(pocket);
        let recess = centered_cube(
            format!(
                "closed_door_recovery_{}_logger_recess",
                logger_label(pocket)
            ),
            LOGGER_POCKET_X,
            LOGGER_POCKET_Y,
            LOGGER_POCKET_DEPTH + 1.0,
        )
        .translate(
            x,
            0.0,
            LOGGER_PANEL_Z / 2.0 - LOGGER_POCKET_DEPTH / 2.0 + 0.5,
        );
        let cable = centered_cube(
            format!(
                "closed_door_recovery_{}_logger_cable_notch",
                logger_label(pocket)
            ),
            LOGGER_CABLE_NOTCH_W,
            LOGGER_PANEL_Y + 2.0,
            LOGGER_POCKET_DEPTH + 2.0,
        )
        .translate(
            x + LOGGER_POCKET_X / 2.0 - 12.0,
            0.0,
            LOGGER_PANEL_Z / 2.0 - LOGGER_POCKET_DEPTH / 2.0 + 0.6,
        );
        recesses = recesses + recess + cable;
    }
    recesses
}

fn logger_retainer_clips() -> Part {
    let mut clips = Part::empty("closed_door_recovery_logger_retainer_clips");
    for pocket in 0..LOGGER_POCKET_COUNT {
        let x = logger_pocket_x(pocket);
        for (side, y) in [("front", -1.0), ("rear", 1.0)] {
            clips = clips
                + centered_cube(
                    format!(
                        "closed_door_recovery_{}_logger_{}_retainer_clip",
                        logger_label(pocket),
                        side
                    ),
                    LOGGER_POCKET_X - 14.0,
                    5.0,
                    8.0,
                )
                .translate(
                    x,
                    y * (LOGGER_POCKET_Y / 2.0 + 8.0),
                    LOGGER_PANEL_Z / 2.0 + 4.0,
                );
        }
    }
    clips
}

fn logger_seal_lands() -> Part {
    let mut lands = Part::empty("closed_door_recovery_logger_evidence_seal_lands");
    for i in 0..LOGGER_SEAL_LANDS {
        let x = centered_index(i % 4, 4, 76.0);
        let y = if i < 4 {
            -LOGGER_PANEL_Y / 2.0 + 18.0
        } else {
            LOGGER_PANEL_Y / 2.0 - 18.0
        };
        lands = lands
            + centered_cube(
                format!("closed_door_recovery_logger_evidence_seal_land_{i}"),
                42.0,
                13.0,
                4.0,
            )
            .translate(x, y, LOGGER_PANEL_Z / 2.0 + 2.0);
    }
    lands
}

fn logger_cable_strain_reliefs() -> Part {
    let mut reliefs = Part::empty("closed_door_recovery_logger_cable_strain_reliefs");
    for pocket in 0..LOGGER_POCKET_COUNT {
        let x = logger_pocket_x(pocket) + LOGGER_POCKET_X / 2.0 - 10.0;
        reliefs = reliefs
            + centered_cube(
                format!(
                    "closed_door_recovery_{}_logger_cable_strain_bridge",
                    logger_label(pocket)
                ),
                20.0,
                18.0,
                10.0,
            )
            .translate(x, LOGGER_PANEL_Y / 2.0 - 32.0, LOGGER_PANEL_Z / 2.0 + 5.0);
    }
    reliefs
}

fn airflow_baffle_witness_targets() -> Part {
    let panel = centered_cube(
        "closed_door_recovery_airflow_baffle_witness_panel",
        BAFFLE_PANEL_X,
        BAFFLE_PANEL_Y,
        BAFFLE_PANEL_Z,
    );
    panel - baffle_flow_slots()
        + baffle_vanes()
        + baffle_witness_targets()
        + baffle_upstream_downstream_datums()
}

fn baffle_flow_slots() -> Part {
    let mut slots = Part::empty("closed_door_recovery_baffle_flow_slots");
    for i in 0..BAFFLE_VANE_COUNT {
        let y = centered_index(i, BAFFLE_VANE_COUNT, 34.0);
        slots = slots
            + centered_cube(
                format!("closed_door_recovery_baffle_flow_slot_{i}"),
                BAFFLE_VANE_X,
                10.0,
                BAFFLE_PANEL_Z + 2.0,
            )
            .translate(0.0, y, 0.0);
    }
    slots
}

fn baffle_vanes() -> Part {
    let mut vanes = Part::empty("closed_door_recovery_airflow_baffle_vanes");
    for i in 0..BAFFLE_VANE_COUNT {
        let y = centered_index(i, BAFFLE_VANE_COUNT, 34.0) + 17.0;
        vanes = vanes
            + centered_cube(
                format!("closed_door_recovery_airflow_baffle_vane_{i}"),
                BAFFLE_VANE_X,
                BAFFLE_VANE_Y,
                BAFFLE_VANE_Z,
            )
            .rotate(8.0, 0.0, 0.0)
            .translate(0.0, y, BAFFLE_PANEL_Z / 2.0 + BAFFLE_VANE_Z / 2.0);
    }
    vanes
}

fn baffle_witness_targets() -> Part {
    let mut targets = Part::empty("closed_door_recovery_airflow_baffle_witness_targets");
    for row in 0..BAFFLE_ROWS {
        for col in 0..BAFFLE_COLS {
            let target = row * BAFFLE_COLS + col;
            let x = centered_index(col, BAFFLE_COLS, BAFFLE_PITCH_X);
            let y = centered_index(row, BAFFLE_ROWS, BAFFLE_PITCH_Y);
            let rim = centered_cylinder(
                format!("closed_door_recovery_airflow_target_{target}_rim"),
                BAFFLE_TARGET_RIM_D / 2.0,
                4.0,
                36,
            )
            .translate(x, y, BAFFLE_PANEL_Z / 2.0 + 2.0);
            let witness = centered_cylinder(
                format!("closed_door_recovery_airflow_target_{target}_witness_disc"),
                BAFFLE_TARGET_D / 2.0,
                7.0,
                36,
            )
            .translate(x, y, BAFFLE_PANEL_Z / 2.0 + 3.5);
            targets = targets + rim + witness;
        }
    }
    targets
}

fn baffle_upstream_downstream_datums() -> Part {
    let upstream = centered_cube(
        "closed_door_recovery_baffle_upstream_reference_bar",
        BAFFLE_PANEL_X - 46.0,
        5.0,
        6.0,
    )
    .translate(0.0, BAFFLE_PANEL_Y / 2.0 - 24.0, BAFFLE_PANEL_Z / 2.0 + 3.0);
    let downstream = centered_cube(
        "closed_door_recovery_baffle_downstream_reference_bar",
        BAFFLE_PANEL_X - 46.0,
        5.0,
        6.0,
    )
    .translate(
        0.0,
        -BAFFLE_PANEL_Y / 2.0 + 24.0,
        BAFFLE_PANEL_Z / 2.0 + 3.0,
    );
    upstream + downstream
}

fn condensate_drip_capture() -> Part {
    let tray = centered_cube(
        "closed_door_recovery_condensate_drip_capture_tray",
        CONDENSATE_TRAY_X,
        CONDENSATE_TRAY_Y,
        CONDENSATE_TRAY_Z,
    );
    let basin = centered_cube(
        "closed_door_recovery_condensate_capture_basin_cut",
        CONDENSATE_TRAY_X - 42.0,
        CONDENSATE_TRAY_Y - 42.0,
        CONDENSATE_BASIN_DEPTH + 1.0,
    )
    .translate(
        0.0,
        0.0,
        CONDENSATE_TRAY_Z / 2.0 - CONDENSATE_BASIN_DEPTH / 2.0 + 0.5,
    );

    tray - basin - condensate_drain_slots()
        + condensate_tray_rims()
        + condensate_drip_lane_lips()
        + condensate_cup_lands()
        + condensate_level_witness_steps()
}

fn condensate_tray_rims() -> Part {
    let front = centered_cube(
        "closed_door_recovery_condensate_front_rim",
        CONDENSATE_TRAY_X,
        CONDENSATE_RIM_W,
        CONDENSATE_RIM_Z,
    )
    .translate(
        0.0,
        -CONDENSATE_TRAY_Y / 2.0 + CONDENSATE_RIM_W / 2.0,
        CONDENSATE_TRAY_Z / 2.0 + CONDENSATE_RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "closed_door_recovery_condensate_rear_drip_break_rim",
        CONDENSATE_TRAY_X,
        CONDENSATE_RIM_W,
        CONDENSATE_RIM_Z,
    )
    .translate(
        0.0,
        CONDENSATE_TRAY_Y / 2.0 - CONDENSATE_RIM_W / 2.0,
        CONDENSATE_TRAY_Z / 2.0 + CONDENSATE_RIM_Z / 2.0,
    );
    let left = centered_cube(
        "closed_door_recovery_condensate_left_rim",
        CONDENSATE_RIM_W,
        CONDENSATE_TRAY_Y,
        CONDENSATE_RIM_Z,
    )
    .translate(
        -CONDENSATE_TRAY_X / 2.0 + CONDENSATE_RIM_W / 2.0,
        0.0,
        CONDENSATE_TRAY_Z / 2.0 + CONDENSATE_RIM_Z / 2.0,
    );
    let right = centered_cube(
        "closed_door_recovery_condensate_right_rim",
        CONDENSATE_RIM_W,
        CONDENSATE_TRAY_Y,
        CONDENSATE_RIM_Z,
    )
    .translate(
        CONDENSATE_TRAY_X / 2.0 - CONDENSATE_RIM_W / 2.0,
        0.0,
        CONDENSATE_TRAY_Z / 2.0 + CONDENSATE_RIM_Z / 2.0,
    );
    front + rear + left + right
}

fn condensate_drain_slots() -> Part {
    let mut drains = Part::empty("closed_door_recovery_condensate_drain_slots");
    for lane in 0..DRIP_CAPTURE_LANES {
        let x = centered_index(lane, DRIP_CAPTURE_LANES, DRIP_LANE_PITCH_X);
        drains = drains
            + centered_cube(
                format!("closed_door_recovery_condensate_lane_{lane}_drain_gutter"),
                DRIP_LANE_W,
                12.0,
                CONDENSATE_BASIN_DEPTH + 2.0,
            )
            .translate(
                x,
                -CONDENSATE_TRAY_Y / 2.0 + 46.0,
                CONDENSATE_TRAY_Z / 2.0 - CONDENSATE_BASIN_DEPTH / 2.0 + 0.4,
            );
    }
    drains
}

fn condensate_drip_lane_lips() -> Part {
    let mut lips = Part::empty("closed_door_recovery_condensate_drip_lane_lips");
    for lane in 0..DRIP_CAPTURE_LANES {
        let x = centered_index(lane, DRIP_CAPTURE_LANES, DRIP_LANE_PITCH_X);
        lips = lips
            + centered_cube(
                format!("closed_door_recovery_condensate_lane_{lane}_drip_lip"),
                DRIP_LANE_W + 18.0,
                5.0,
                10.0,
            )
            .translate(x, 28.0, CONDENSATE_TRAY_Z / 2.0 + 5.0);
    }
    lips
}

fn condensate_cup_lands() -> Part {
    let mut lands = Part::empty("closed_door_recovery_condensate_capture_cup_lands");
    for cup in 0..DRIP_CUP_COUNT {
        let x = centered_index(cup, DRIP_CUP_COUNT, 92.0);
        let land = centered_cylinder(
            format!("closed_door_recovery_condensate_capture_cup_{cup}_land"),
            DRIP_CUP_D / 2.0,
            7.0,
            48,
        )
        .translate(
            x,
            -CONDENSATE_TRAY_Y / 2.0 + 34.0,
            CONDENSATE_TRAY_Z / 2.0 + 3.5,
        );
        let bore = centered_cylinder(
            format!("closed_door_recovery_condensate_capture_cup_{cup}_center_bore"),
            8.0,
            8.0,
            32,
        )
        .translate(
            x,
            -CONDENSATE_TRAY_Y / 2.0 + 34.0,
            CONDENSATE_TRAY_Z / 2.0 + 4.0,
        );
        lands = lands + (land - bore);
    }
    lands
}

fn condensate_level_witness_steps() -> Part {
    let mut steps = Part::empty("closed_door_recovery_condensate_level_witness_steps");
    for step in 0..4 {
        steps = steps
            + centered_cube(
                format!("closed_door_recovery_condensate_level_step_{step}"),
                34.0,
                10.0,
                4.0 + step as f64 * 2.0,
            )
            .translate(
                CONDENSATE_TRAY_X / 2.0 - 42.0,
                -48.0 + step as f64 * 22.0,
                CONDENSATE_TRAY_Z / 2.0 + 2.0 + step as f64,
            );
    }
    steps
}

fn barcode_run_tokens() -> Part {
    let plate = centered_cube(
        "closed_door_recovery_barcode_run_token_plate",
        TOKEN_PLATE_X,
        TOKEN_PLATE_Y,
        TOKEN_PLATE_Z,
    );
    plate + run_token_discs() + barcode_lands() + token_chain_custody_rail()
}

fn run_token_discs() -> Part {
    let mut tokens = Part::empty("closed_door_recovery_barcode_run_tokens");
    for token in 0..RUN_TOKEN_COUNT {
        let (x, y) = token_center(token);
        let rim = centered_cylinder(
            format!("closed_door_recovery_run_token_{token}_rim"),
            TOKEN_RIM_D / 2.0,
            4.0,
            36,
        )
        .translate(x, y, TOKEN_PLATE_Z / 2.0 + 2.0);
        let disc = centered_cylinder(
            format!("closed_door_recovery_run_token_{token}_removable_disc"),
            TOKEN_D / 2.0,
            8.0,
            36,
        )
        .translate(x, y, TOKEN_PLATE_Z / 2.0 + 4.0);
        tokens = tokens + rim + disc;
    }
    tokens
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("closed_door_recovery_barcode_lands");
    for token in 0..RUN_TOKEN_COUNT {
        let (x, y) = token_center(token);
        lands = lands
            + centered_cube(
                format!("closed_door_recovery_run_token_{token}_barcode_land"),
                BARCODE_LAND_X,
                BARCODE_LAND_Y,
                4.0,
            )
            .translate(x, y - 25.0, TOKEN_PLATE_Z / 2.0 + 2.0);
    }
    lands
}

fn token_chain_custody_rail() -> Part {
    centered_cube(
        "closed_door_recovery_token_chain_of_custody_rail",
        TOKEN_PLATE_X - 38.0,
        5.0,
        8.0,
    )
    .translate(0.0, TOKEN_PLATE_Y / 2.0 - 16.0, TOKEN_PLATE_Z / 2.0 + 4.0)
}

fn release_hold_reject_evidence_lanes() -> Part {
    let panel = centered_cube(
        "closed_door_recovery_release_hold_reject_evidence_panel",
        EVIDENCE_PANEL_X,
        EVIDENCE_PANEL_Y,
        EVIDENCE_PANEL_Z,
    );
    panel - evidence_slot_recesses()
        + evidence_lane_walls()
        + evidence_lane_token_stops()
        + evidence_lane_status_pucks()
}

fn evidence_slot_recesses() -> Part {
    let mut recesses = Part::empty("closed_door_recovery_evidence_slot_recesses");
    for lane in 0..EVIDENCE_LANES {
        let x = evidence_lane_x(lane);
        for slot in 0..EVIDENCE_SLOTS_PER_LANE {
            let y = evidence_slot_y(slot);
            recesses = recesses
                + centered_cube(
                    format!(
                        "closed_door_recovery_{}_lane_evidence_slot_{slot}",
                        evidence_lane_label(lane)
                    ),
                    EVIDENCE_SLOT_X,
                    EVIDENCE_SLOT_Y,
                    EVIDENCE_SLOT_DEPTH + 1.0,
                )
                .translate(
                    x,
                    y,
                    EVIDENCE_PANEL_Z / 2.0 - EVIDENCE_SLOT_DEPTH / 2.0 + 0.5,
                );
        }
    }
    recesses
}

fn evidence_lane_walls() -> Part {
    let mut walls = Part::empty("closed_door_recovery_evidence_lane_walls");
    for lane in 0..EVIDENCE_LANES {
        let x = evidence_lane_x(lane);
        let left = centered_cube(
            format!(
                "closed_door_recovery_{}_lane_left_wall",
                evidence_lane_label(lane)
            ),
            EVIDENCE_WALL_W,
            EVIDENCE_PANEL_Y - 28.0,
            18.0,
        )
        .translate(
            x - EVIDENCE_SLOT_X / 2.0 - 13.0,
            0.0,
            EVIDENCE_PANEL_Z / 2.0 + 9.0,
        );
        let right = centered_cube(
            format!(
                "closed_door_recovery_{}_lane_right_wall",
                evidence_lane_label(lane)
            ),
            EVIDENCE_WALL_W,
            EVIDENCE_PANEL_Y - 28.0,
            18.0,
        )
        .translate(
            x + EVIDENCE_SLOT_X / 2.0 + 13.0,
            0.0,
            EVIDENCE_PANEL_Z / 2.0 + 9.0,
        );
        walls = walls + left + right;
    }
    walls
}

fn evidence_lane_token_stops() -> Part {
    let mut stops = Part::empty("closed_door_recovery_evidence_lane_token_stops");
    for lane in 0..EVIDENCE_LANES {
        let x = evidence_lane_x(lane);
        stops = stops
            + centered_cube(
                format!(
                    "closed_door_recovery_{}_lane_end_stop",
                    evidence_lane_label(lane)
                ),
                EVIDENCE_SLOT_X + 34.0,
                8.0,
                16.0,
            )
            .translate(
                x,
                EVIDENCE_PANEL_Y / 2.0 - 22.0,
                EVIDENCE_PANEL_Z / 2.0 + 8.0,
            );
    }
    stops
}

fn evidence_lane_status_pucks() -> Part {
    let mut pucks = Part::empty("closed_door_recovery_evidence_lane_status_pucks");
    for lane in 0..EVIDENCE_LANES {
        let x = evidence_lane_x(lane);
        pucks = pucks
            + centered_cylinder(
                format!(
                    "closed_door_recovery_{}_lane_status_puck",
                    evidence_lane_label(lane)
                ),
                12.0,
                7.0,
                36,
            )
            .translate(
                x,
                -EVIDENCE_PANEL_Y / 2.0 + 24.0,
                EVIDENCE_PANEL_Z / 2.0 + 3.5,
            );
    }
    pucks
}

fn robot_service_keepout_gauges() -> Part {
    let robot = centered_cube(
        "closed_door_recovery_front_robot_access_keepout_gauge",
        ROBOT_KEEP_OUT_X,
        ROBOT_KEEP_OUT_Y,
        ROBOT_KEEP_OUT_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + ROBOT_KEEP_OUT_Y / 2.0 + 24.0, 0.0);
    let service_left = centered_cube(
        "closed_door_recovery_left_service_keepout_gauge",
        SERVICE_KEEP_OUT_X,
        SERVICE_KEEP_OUT_Y,
        SERVICE_KEEP_OUT_Z,
    )
    .translate(
        -DECK_X / 2.0 + SERVICE_KEEP_OUT_X / 2.0 + 24.0,
        0.0,
        SERVICE_KEEP_OUT_Z / 2.0,
    );
    let service_right = centered_cube(
        "closed_door_recovery_right_service_keepout_gauge",
        SERVICE_KEEP_OUT_X,
        SERVICE_KEEP_OUT_Y,
        SERVICE_KEEP_OUT_Z,
    )
    .translate(
        DECK_X / 2.0 - SERVICE_KEEP_OUT_X / 2.0 - 24.0,
        0.0,
        SERVICE_KEEP_OUT_Z / 2.0,
    );
    let door_swing = centered_cube(
        "closed_door_recovery_door_swing_keepout_gauge",
        DOOR_SWING_KEEP_OUT_X,
        DOOR_SWING_KEEP_OUT_Y,
        DOOR_SWING_KEEP_OUT_Z,
    )
    .translate(
        DOOR_GAUGE_CENTER.0,
        DOOR_GAUGE_CENTER.1 - 28.0,
        DOOR_SWING_KEEP_OUT_Z / 2.0,
    );

    robot + service_left + service_right + door_swing
}

fn fiducial_disc(name: &str) -> Part {
    let outer = centered_cylinder(format!("{name}_outer_disc"), 12.0, 3.0, 36);
    let inner = centered_cylinder(format!("{name}_center_bore"), 4.0, 4.0, 28);
    outer - inner
}

fn rack_slot_center(row: usize, col: usize) -> (f64, f64) {
    (
        centered_index(col, RACK_COLS, RACK_SLOT_PITCH_X),
        centered_index(row, RACK_ROWS, RACK_SLOT_PITCH_Y),
    )
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn door_tick_x(tick: usize) -> f64 {
    -124.0 + tick as f64 * DOOR_TICK_PITCH_X
}

fn swing_tick_angle_deg(index: usize) -> f64 {
    -18.0 + index as f64 * 9.0
}

fn logger_pocket_x(pocket: usize) -> f64 {
    centered_index(pocket, LOGGER_POCKET_COUNT, LOGGER_POCKET_PITCH_X)
}

fn logger_label(pocket: usize) -> &'static str {
    match pocket {
        0 => "temperature",
        1 => "co2",
        2 => "rh",
        3 => "reference",
        _ => "unknown",
    }
}

fn token_center(token: usize) -> (f64, f64) {
    let row = token / TOKEN_COLS;
    let col = token % TOKEN_COLS;
    (
        centered_index(col, TOKEN_COLS, TOKEN_PITCH_X),
        centered_index(row, TOKEN_ROWS, TOKEN_PITCH_Y),
    )
}

fn evidence_lane_x(lane: usize) -> f64 {
    centered_index(lane, EVIDENCE_LANES, EVIDENCE_LANE_PITCH_X)
}

fn evidence_slot_y(slot: usize) -> f64 {
    centered_index(slot, EVIDENCE_SLOTS_PER_LANE, EVIDENCE_SLOT_PITCH_Y)
}

fn evidence_lane_label(lane: usize) -> &'static str {
    match lane {
        0 => "release",
        1 => "hold",
        2 => "reject",
        _ => "unknown",
    }
}

fn door_event_span_mm() -> f64 {
    door_tick_x(DOOR_EVENT_TICKS - 1) - door_tick_x(0)
}

fn max_door_tick_height_mm() -> f64 {
    16.0 + (DOOR_EVENT_TICKS - 1) as f64 * 4.0
}

fn door_recovery_window_area_mm2() -> f64 {
    DOOR_WINDOW_X * DOOR_WINDOW_Y
}

fn condensate_capture_volume_ml() -> f64 {
    (CONDENSATE_TRAY_X - 42.0) * (CONDENSATE_TRAY_Y - 42.0) * CONDENSATE_BASIN_DEPTH / 1000.0
}

fn evidence_lane_gap() -> f64 {
    EVIDENCE_LANE_PITCH_X - EVIDENCE_SLOT_X
}

fn module_rects() -> [Rect; 7] {
    [
        rack_rect(),
        door_rect(),
        logger_rect(),
        baffle_rect(),
        condensate_rect(),
        token_rect(),
        evidence_rect(),
    ]
}

fn rack_rect() -> Rect {
    Rect {
        x: RACK_CENTER.0,
        y: RACK_CENTER.1,
        w: RACK_X,
        h: RACK_Y,
    }
}

fn door_rect() -> Rect {
    Rect {
        x: DOOR_GAUGE_CENTER.0,
        y: DOOR_GAUGE_CENTER.1,
        w: DOOR_GAUGE_X,
        h: DOOR_GAUGE_Y,
    }
}

fn logger_rect() -> Rect {
    Rect {
        x: LOGGER_CENTER.0,
        y: LOGGER_CENTER.1,
        w: LOGGER_PANEL_X,
        h: LOGGER_PANEL_Y,
    }
}

fn baffle_rect() -> Rect {
    Rect {
        x: BAFFLE_CENTER.0,
        y: BAFFLE_CENTER.1,
        w: BAFFLE_PANEL_X,
        h: BAFFLE_PANEL_Y,
    }
}

fn condensate_rect() -> Rect {
    Rect {
        x: CONDENSATE_CENTER.0,
        y: CONDENSATE_CENTER.1,
        w: CONDENSATE_TRAY_X,
        h: CONDENSATE_TRAY_Y,
    }
}

fn token_rect() -> Rect {
    Rect {
        x: TOKEN_CENTER.0,
        y: TOKEN_CENTER.1,
        w: TOKEN_PLATE_X,
        h: TOKEN_PLATE_Y,
    }
}

fn evidence_rect() -> Rect {
    Rect {
        x: EVIDENCE_CENTER.0,
        y: EVIDENCE_CENTER.1,
        w: EVIDENCE_PANEL_X,
        h: EVIDENCE_PANEL_Y,
    }
}

fn module_name(module: Rect) -> &'static str {
    if same_rect(module, rack_rect()) {
        "rack"
    } else if same_rect(module, door_rect()) {
        "door gauge"
    } else if same_rect(module, logger_rect()) {
        "logger"
    } else if same_rect(module, baffle_rect()) {
        "baffle"
    } else if same_rect(module, condensate_rect()) {
        "condensate"
    } else if same_rect(module, token_rect()) {
        "token"
    } else if same_rect(module, evidence_rect()) {
        "evidence"
    } else {
        "unknown"
    }
}

fn fits_on_deck(rect: Rect, margin: f64) -> bool {
    rect.x.abs() + rect.w / 2.0 <= DECK_X / 2.0 - margin
        && rect.y.abs() + rect.h / 2.0 <= DECK_Y / 2.0 - margin
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

fn same_rect(left: Rect, right: Rect) -> bool {
    left.x == right.x && left.y == right.y && left.w == right.w && left.h == right.h
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requested_feature_manifest_is_complete() {
        assert!(REQUIRED_FEATURES.contains(&"sealed_cassette_surrogate_rack"));
        assert!(REQUIRED_FEATURES.contains(&"quick_open_door_event_gauge"));
        assert!(REQUIRED_FEATURES.contains(&"temp_co2_rh_logger_pockets"));
        assert!(REQUIRED_FEATURES.contains(&"airflow_baffle_witness_targets"));
        assert!(REQUIRED_FEATURES.contains(&"condensate_drip_capture"));
        assert!(REQUIRED_FEATURES.contains(&"barcode_run_tokens"));
        assert!(REQUIRED_FEATURES.contains(&"release_hold_reject_evidence_lanes"));
    }

    #[test]
    fn rack_and_layout_clearances_are_valid() {
        assert_eq!(RACK_SLOT_COUNT, RACK_COLS * RACK_ROWS);
        assert_eq!(RACK_SLOT_COUNT, 9);
        assert!(RACK_SLOT_PITCH_X > RACK_SLOT_X);
        assert!(RACK_SLOT_PITCH_Y > RACK_SLOT_Y);
        assert!(module_rects().iter().all(|rect| fits_on_deck(*rect, 20.0)));
        assert!(!rects_overlap(rack_rect(), door_rect()));
        assert!(!rects_overlap(condensate_rect(), evidence_rect()));
    }

    #[test]
    fn door_event_gauge_has_measurable_opening_timeline() {
        assert_eq!(DOOR_EVENT_TICKS, 7);
        assert!(door_event_span_mm() >= 300.0);
        assert!(door_recovery_window_area_mm2() >= 7_000.0);
        assert!(max_door_tick_height_mm() <= DOOR_TICK_MAX_Z);
        assert_eq!(DOOR_SWING_WITNESS_COUNT, 5);
    }

    #[test]
    fn environmental_evidence_features_cover_required_sensors() {
        assert_eq!(LOGGER_POCKET_TYPES, 3);
        assert_eq!(LOGGER_POCKET_COUNT, 4);
        assert_eq!(logger_label(0), "temperature");
        assert_eq!(logger_label(1), "co2");
        assert_eq!(logger_label(2), "rh");
        assert_eq!(BAFFLE_WITNESS_TARGETS, BAFFLE_ROWS * BAFFLE_COLS);
        assert!(BAFFLE_WITNESS_TARGETS >= RACK_SLOT_COUNT);
    }

    #[test]
    fn condensate_tokens_and_evidence_lanes_have_capacity() {
        assert!(condensate_capture_volume_ml() > DRIP_CHALLENGE_ML);
        assert_eq!(RUN_TOKEN_COUNT, TOKEN_ROWS * TOKEN_COLS);
        assert_eq!(EVIDENCE_LANES, 3);
        assert_eq!(EVIDENCE_SLOTS_PER_LANE, 4);
        assert!(evidence_lane_gap() >= EVIDENCE_MIN_SEGREGATION_GAP);
    }

    #[test]
    fn output_manifest_exports_parts_plus_assembly() {
        assert_eq!(OUTPUTS.len(), 10);
        assert!(OUTPUTS[0].ends_with("_deck.stl"));
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }
}
