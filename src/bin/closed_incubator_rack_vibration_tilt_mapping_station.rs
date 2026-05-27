use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Incubator rack vibration and tilt mapping station.
//
// This standalone generator models a no-cell metrology station for mapping how
// a six-position incubator rack transmits vibration and static tilt into loaded
// cassette envelopes. It provides a rack datum plate, accelerometer/tilt sensor
// nests, mass surrogate cassettes, anti-slip clamps, isolation foot test pads,
// cable management, repeatable position labels, and service clearance gauges.

const OUTPUTS: [&str; 11] = [
    "output/closed_incubator_rack_vibration_tilt_mapping_station_deck.stl",
    "output/closed_incubator_rack_vibration_tilt_mapping_station_rack_datum_plate.stl",
    "output/closed_incubator_rack_vibration_tilt_mapping_station_cassette_surrogate_weights.stl",
    "output/closed_incubator_rack_vibration_tilt_mapping_station_accelerometer_tilt_sensor_fixtures.stl",
    "output/closed_incubator_rack_vibration_tilt_mapping_station_anti_slip_clamps.stl",
    "output/closed_incubator_rack_vibration_tilt_mapping_station_isolation_foot_test_pads.stl",
    "output/closed_incubator_rack_vibration_tilt_mapping_station_cable_routing.stl",
    "output/closed_incubator_rack_vibration_tilt_mapping_station_repeatable_position_labels.stl",
    "output/closed_incubator_rack_vibration_tilt_mapping_station_tilt_reference_tools.stl",
    "output/closed_incubator_rack_vibration_tilt_mapping_station_service_clearance_gauges.stl",
    "output/closed_incubator_rack_vibration_tilt_mapping_station_assembly.stl",
];

const RACK_SLOT_COUNT: usize = 6;
const SLOT_COLS: usize = 3;
const SLOT_ROWS: usize = 2;
const CORNER_REFERENCE_SENSOR_COUNT: usize = 4;
const SENSOR_FIXTURE_COUNT: usize = RACK_SLOT_COUNT + CORNER_REFERENCE_SENSOR_COUNT;
const ISOLATION_FOOT_COUNT: usize = 4;
const SIDE_CLAMP_COUNT: usize = SLOT_ROWS * 2;
const TOE_CLAMP_COUNT: usize = SLOT_COLS * 2;
const CLAMP_COUNT: usize = SIDE_CLAMP_COUNT + TOE_CLAMP_COUNT;
const POSITION_LABEL_COUNT: usize = RACK_SLOT_COUNT + ISOLATION_FOOT_COUNT;

const DECK_X: f64 = 1080.0;
const DECK_Y: f64 = 780.0;
const DECK_Z: f64 = 22.0;
const FOOT_PAD_Z: f64 = 20.0;
const DECK_BOTTOM_Z: f64 = FOOT_PAD_Z + 4.0;
const DECK_TOP_Z: f64 = DECK_BOTTOM_Z + DECK_Z;

const DATUM_PLATE_X: f64 = 780.0;
const DATUM_PLATE_Y: f64 = 444.0;
const DATUM_PLATE_Z: f64 = 24.0;
const DATUM_PLATE_CENTER_X: f64 = 0.0;
const DATUM_PLATE_CENTER_Y: f64 = 48.0;
const DATUM_PLATE_TOP_Z: f64 = DECK_TOP_Z + DATUM_PLATE_Z;
const DATUM_RAIL_W: f64 = 18.0;
const DATUM_RAIL_Z: f64 = 28.0;
const DATUM_PIN_D: f64 = 9.0;
const DATUM_GRID_HOLE_D: f64 = 6.6;
const DATUM_GRID_COLS: usize = 5;
const DATUM_GRID_ROWS: usize = 3;

const SLOT_PITCH_X: f64 = 222.0;
const SLOT_PITCH_Y: f64 = 174.0;
const CASSETTE_SURROGATE_X: f64 = REVC_CHIP_LENGTH + 68.0;
const CASSETTE_SURROGATE_Y: f64 = REVC_CHIP_WIDTH + 56.0;
const CASSETTE_SURROGATE_BASE_Z: f64 = 14.0;
const CASSETTE_BALLAST_Z: f64 = 20.0;
const CASSETTE_SURROGATE_Z: f64 = CASSETTE_SURROGATE_BASE_Z + CASSETTE_BALLAST_Z;
const BALLAST_BLOCK_X: f64 = CASSETTE_SURROGATE_X - 42.0;
const BALLAST_BLOCK_Y: f64 = CASSETTE_SURROGATE_Y - 36.0;
const BALLAST_POCKET_COUNT: usize = 4;
const BALLAST_PUCK_D: f64 = 28.0;
const CENTER_OF_MASS_TARGET_D: f64 = 26.0;

const SENSOR_POD_X: f64 = 104.0;
const SENSOR_POD_Y: f64 = 64.0;
const SENSOR_POD_Z: f64 = 18.0;
const ACCELEROMETER_POCKET_X: f64 = 32.0;
const ACCELEROMETER_POCKET_Y: f64 = 26.0;
const TILT_SENSOR_POCKET_X: f64 = 46.0;
const TILT_SENSOR_POCKET_Y: f64 = 22.0;
const SENSOR_RECESS_DEPTH: f64 = 7.0;
const SENSOR_MOUNT_BOSS_D: f64 = 7.5;
const SENSOR_CABLE_PASSAGE_D: f64 = 8.0;
const SENSOR_CABLE_BUNDLE_D: f64 = 4.2;

const CLAMP_JAW_X: f64 = 74.0;
const CLAMP_JAW_Y: f64 = 24.0;
const CLAMP_BODY_X: f64 = 96.0;
const CLAMP_BODY_Y: f64 = 34.0;
const CLAMP_BODY_Z: f64 = 24.0;
const CLAMP_TOE_OVERHANG: f64 = 12.0;
const RUBBER_PAD_Z: f64 = 3.0;
const CLAMP_SCREW_D: f64 = 10.0;

const FOOT_PAD_D: f64 = 88.0;
const FOOT_PAD_RECESS_D: f64 = 56.0;
const FOOT_PAD_SPAN_X: f64 = DECK_X - 150.0;
const FOOT_PAD_SPAN_Y: f64 = DECK_Y - 116.0;
const DUROMETER_TOKEN_D: f64 = 18.0;
const SHEAR_WITNESS_BAR_COUNT: usize = 5;

const CABLE_TROUGH_X: f64 = DATUM_PLATE_X - 70.0;
const CABLE_TROUGH_Y: f64 = 46.0;
const CABLE_TROUGH_Z: f64 = 22.0;
const CABLE_BRANCH_W: f64 = 14.0;
const CABLE_BRANCH_Z: f64 = 8.0;
const CABLE_CLAMP_COUNT: usize = SENSOR_FIXTURE_COUNT;
const CABLE_CLAMP_PITCH_X: f64 = 64.0;
const BULKHEAD_X: f64 = 156.0;
const BULKHEAD_Y: f64 = 28.0;
const BULKHEAD_Z: f64 = 46.0;

const LABEL_LAND_X: f64 = 68.0;
const LABEL_LAND_Y: f64 = 22.0;
const LABEL_LAND_Z: f64 = 2.2;
const LABEL_TICK_X: f64 = 4.0;
const LABEL_TICK_Y: f64 = 14.0;
const FIDUCIAL_D: f64 = 18.0;
const FIDUCIAL_GROOVE_W: f64 = 1.2;

const TILT_WEDGE_COUNT: usize = 4;
const TILT_WEDGE_X: f64 = 92.0;
const TILT_WEDGE_Y: f64 = 46.0;
const TILT_WEDGE_Z: f64 = 18.0;
const BUBBLE_LEVEL_X: f64 = 156.0;
const BUBBLE_LEVEL_Y: f64 = 42.0;
const BUBBLE_LEVEL_Z: f64 = 18.0;

const SERVICE_KEEP_OUT_X: f64 = 820.0;
const SERVICE_KEEP_OUT_Y: f64 = 92.0;
const SERVICE_KEEP_OUT_Z: f64 = 14.0;
const SENSOR_ACCESS_CLEARANCE_Z: f64 = 128.0;
const CABLE_SERVICE_CLEARANCE_Y: f64 = 48.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = station_deck();
    export(OUTPUTS[0], &deck);

    let datum = rack_datum_plate();
    export(OUTPUTS[1], &datum);

    let surrogates = cassette_surrogate_weights();
    export(OUTPUTS[2], &surrogates);

    let sensors = accelerometer_tilt_sensor_fixtures();
    export(OUTPUTS[3], &sensors);

    let clamps = anti_slip_clamps();
    export(OUTPUTS[4], &clamps);

    let feet = isolation_foot_test_pads();
    export(OUTPUTS[5], &feet);

    let cable = cable_routing();
    export(OUTPUTS[6], &cable);

    let labels = repeatable_position_labels();
    export(OUTPUTS[7], &labels);

    let tilt_tools = tilt_reference_tools();
    export(OUTPUTS[8], &tilt_tools);

    let keepouts = service_clearance_gauges();
    export(OUTPUTS[9], &keepouts);

    let assembly = deck
        + datum
        + surrogates
        + sensors
        + clamps
        + feet
        + cable
        + labels
        + tilt_tools
        + keepouts;
    export(OUTPUTS[10], &assembly);

    println!();
    println!("Closed incubator rack vibration/tilt mapping station:");
    println!(
        "  Datum envelope:              {DATUM_PLATE_X:.0}mm x {DATUM_PLATE_Y:.0}mm rack datum plate on {DECK_X:.0}mm x {DECK_Y:.0}mm deck"
    );
    println!(
        "  Rack map:                    {RACK_SLOT_COUNT} cassette surrogate weights in a {SLOT_COLS}x{SLOT_ROWS} slot grid, {SLOT_PITCH_X:.0}mm x {SLOT_PITCH_Y:.0}mm pitch"
    );
    println!(
        "  Sensor fixtures:             {RACK_SLOT_COUNT} slot pods plus {CORNER_REFERENCE_SENSOR_COUNT} corner reference pods, {SENSOR_CABLE_PASSAGE_D:.1}mm cable passages"
    );
    println!(
        "  Retention/isolation:          {CLAMP_COUNT} anti-slip clamps and {ISOLATION_FOOT_COUNT} isolation foot test pads with replaceable token pockets"
    );
    println!(
        "  Traceability:                {POSITION_LABEL_COUNT} repeatable position label lands plus fiducial targets and slot tick keys"
    );
    println!(
        "  Cable/service clearances:     rear {CABLE_TROUGH_X:.0}mm cable trough, {CABLE_CLAMP_COUNT} cable clamp comb slots, {:.0}mm sensor access clearance",
        sensor_access_clearance()
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_design_constraints() {
    assert_eq!(RACK_SLOT_COUNT, SLOT_COLS * SLOT_ROWS);
    assert_eq!(
        SENSOR_FIXTURE_COUNT,
        RACK_SLOT_COUNT + CORNER_REFERENCE_SENSOR_COUNT
    );
    assert_eq!(POSITION_LABEL_COUNT, RACK_SLOT_COUNT + ISOLATION_FOOT_COUNT);
    assert_eq!(BALLAST_POCKET_COUNT, 4);
    assert!(CASSETTE_SURROGATE_Z > REVC_TOTAL_HEIGHT + 16.0);
    assert!(slot_span_x() + CASSETTE_SURROGATE_X < DATUM_PLATE_X - 70.0);
    assert!(slot_span_y() + CASSETTE_SURROGATE_Y < DATUM_PLATE_Y - 72.0);
    assert!(sensor_cable_radial_clearance() >= 1.5);
    assert!(sensor_access_clearance() >= 72.0);
    assert!(cable_service_clearance() >= CABLE_SERVICE_CLEARANCE_Y);
    assert_eq!(anti_slip_clamp_count(), CLAMP_COUNT);
    assert_eq!(cable_branch_count(), SENSOR_FIXTURE_COUNT);
    assert_eq!(isolation_pad_count(), ISOLATION_FOOT_COUNT);
}

fn station_deck() -> Part {
    let deck = centered_cube(
        "closed_incubator_vibration_tilt_mapping_station_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_BOTTOM_Z + DECK_Z / 2.0);

    let datum_recess = centered_cube(
        "closed_incubator_vibration_tilt_mapping_station_datum_plate_recess",
        DATUM_PLATE_X + 18.0,
        DATUM_PLATE_Y + 18.0,
        5.0,
    )
    .translate(DATUM_PLATE_CENTER_X, DATUM_PLATE_CENTER_Y, DECK_TOP_Z - 2.2);

    let cable_bulkhead_recess = centered_cube(
        "closed_incubator_vibration_tilt_mapping_station_cable_bulkhead_recess",
        BULKHEAD_X + 20.0,
        BULKHEAD_Y + 16.0,
        5.0,
    )
    .translate(0.0, rear_cable_y(), DECK_TOP_Z - 2.2);

    deck - datum_recess - cable_bulkhead_recess - deck_mounting_holes()
        + deck_edge_bumpers()
        + deck_lift_handles()
}

fn deck_mounting_holes() -> Part {
    let mut holes = Part::empty("closed_incubator_vibration_tilt_deck_mounting_holes");
    for (i, (x, y)) in deck_mount_points().into_iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("closed_incubator_vibration_tilt_deck_m6_mount_{i}"),
                3.3,
                DECK_Z + 2.0,
                28,
            )
            .translate(x, y, DECK_BOTTOM_Z + DECK_Z / 2.0);
    }
    holes
}

fn deck_edge_bumpers() -> Part {
    let mut bumpers = Part::empty("closed_incubator_vibration_tilt_deck_edge_bumpers");
    for (i, (sx, sy)) in [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)]
        .into_iter()
        .enumerate()
    {
        bumpers = bumpers
            + centered_cube(
                format!("closed_incubator_vibration_tilt_deck_corner_bumper_{i}"),
                44.0,
                44.0,
                8.0,
            )
            .translate(
                sx * (DECK_X / 2.0 - 42.0),
                sy * (DECK_Y / 2.0 - 42.0),
                DECK_TOP_Z + 4.0,
            );
    }
    bumpers
}

fn deck_lift_handles() -> Part {
    let mut handles = Part::empty("closed_incubator_vibration_tilt_deck_lift_handles");
    for (i, y) in [-(DECK_Y / 2.0 - 64.0), DECK_Y / 2.0 - 64.0]
        .into_iter()
        .enumerate()
    {
        let bridge = centered_cube(
            format!("closed_incubator_vibration_tilt_deck_lift_handle_bridge_{i}"),
            160.0,
            16.0,
            18.0,
        )
        .translate(0.0, y, DECK_TOP_Z + 9.0);
        let left_standoff = centered_cube(
            format!("closed_incubator_vibration_tilt_deck_lift_handle_left_standoff_{i}"),
            18.0,
            22.0,
            20.0,
        )
        .translate(-72.0, y, DECK_TOP_Z + 10.0);
        let right_standoff = centered_cube(
            format!("closed_incubator_vibration_tilt_deck_lift_handle_right_standoff_{i}"),
            18.0,
            22.0,
            20.0,
        )
        .translate(72.0, y, DECK_TOP_Z + 10.0);
        handles = handles + bridge + left_standoff + right_standoff;
    }
    handles
}

fn rack_datum_plate() -> Part {
    let plate = centered_cube(
        "closed_incubator_vibration_tilt_rack_datum_plate",
        DATUM_PLATE_X,
        DATUM_PLATE_Y,
        DATUM_PLATE_Z,
    )
    .translate(
        DATUM_PLATE_CENTER_X,
        DATUM_PLATE_CENTER_Y,
        DECK_TOP_Z + DATUM_PLATE_Z / 2.0,
    );

    plate - slot_relief_pockets() - datum_grid_holes() - leveling_probe_channels()
        + datum_rails()
        + datum_pins()
        + rack_hard_stop_keys()
}

fn slot_relief_pockets() -> Part {
    let mut pockets = Part::empty("closed_incubator_vibration_tilt_slot_relief_pockets");
    for slot in 0..RACK_SLOT_COUNT {
        let (x, y) = slot_center(slot);
        pockets = pockets
            + centered_cube(
                format!(
                    "closed_incubator_vibration_tilt_slot_{}_surrogate_relief",
                    slot_label(slot)
                ),
                CASSETTE_SURROGATE_X + 10.0,
                CASSETTE_SURROGATE_Y + 10.0,
                5.0,
            )
            .translate(x, y, DATUM_PLATE_TOP_Z - 2.2);
    }
    pockets
}

fn datum_grid_holes() -> Part {
    let mut holes = Part::empty("closed_incubator_vibration_tilt_datum_grid_holes");
    for row in 0..DATUM_GRID_ROWS {
        for col in 0..DATUM_GRID_COLS {
            let x = centered_index(col, DATUM_GRID_COLS, 150.0);
            let y = DATUM_PLATE_CENTER_Y + centered_index(row, DATUM_GRID_ROWS, 130.0);
            holes = holes
                + centered_cylinder(
                    format!("closed_incubator_vibration_tilt_datum_grid_hole_{col}_{row}"),
                    DATUM_GRID_HOLE_D / 2.0,
                    DATUM_PLATE_Z + 2.0,
                    28,
                )
                .translate(x, y, DECK_TOP_Z + DATUM_PLATE_Z / 2.0);
        }
    }
    holes
}

fn leveling_probe_channels() -> Part {
    let x_channel = centered_cube(
        "closed_incubator_vibration_tilt_x_leveling_probe_channel",
        DATUM_PLATE_X - 96.0,
        5.0,
        3.2,
    )
    .translate(
        DATUM_PLATE_CENTER_X,
        DATUM_PLATE_CENTER_Y,
        DATUM_PLATE_TOP_Z - 1.0,
    );
    let y_channel = centered_cube(
        "closed_incubator_vibration_tilt_y_leveling_probe_channel",
        5.0,
        DATUM_PLATE_Y - 96.0,
        3.2,
    )
    .translate(
        DATUM_PLATE_CENTER_X,
        DATUM_PLATE_CENTER_Y,
        DATUM_PLATE_TOP_Z - 1.0,
    );

    x_channel + y_channel
}

fn datum_rails() -> Part {
    let left_x_datum = centered_cube(
        "closed_incubator_vibration_tilt_left_x_datum_rail",
        DATUM_RAIL_W,
        DATUM_PLATE_Y,
        DATUM_RAIL_Z,
    )
    .translate(
        DATUM_PLATE_CENTER_X - DATUM_PLATE_X / 2.0 + DATUM_RAIL_W / 2.0,
        DATUM_PLATE_CENTER_Y,
        DATUM_PLATE_TOP_Z + DATUM_RAIL_Z / 2.0,
    );
    let rear_y_datum = centered_cube(
        "closed_incubator_vibration_tilt_rear_y_datum_rail",
        DATUM_PLATE_X,
        DATUM_RAIL_W,
        DATUM_RAIL_Z,
    )
    .translate(
        DATUM_PLATE_CENTER_X,
        DATUM_PLATE_CENTER_Y + DATUM_PLATE_Y / 2.0 - DATUM_RAIL_W / 2.0,
        DATUM_PLATE_TOP_Z + DATUM_RAIL_Z / 2.0,
    );
    let right_soft_capture = centered_cube(
        "closed_incubator_vibration_tilt_right_soft_capture_rail",
        DATUM_RAIL_W,
        DATUM_PLATE_Y * 0.64,
        DATUM_RAIL_Z * 0.62,
    )
    .translate(
        DATUM_PLATE_CENTER_X + DATUM_PLATE_X / 2.0 - DATUM_RAIL_W / 2.0,
        DATUM_PLATE_CENTER_Y - 18.0,
        DATUM_PLATE_TOP_Z + DATUM_RAIL_Z * 0.31,
    );
    let front_soft_capture = centered_cube(
        "closed_incubator_vibration_tilt_front_soft_capture_rail",
        DATUM_PLATE_X * 0.56,
        DATUM_RAIL_W,
        DATUM_RAIL_Z * 0.62,
    )
    .translate(
        DATUM_PLATE_CENTER_X - 36.0,
        DATUM_PLATE_CENTER_Y - DATUM_PLATE_Y / 2.0 + DATUM_RAIL_W / 2.0,
        DATUM_PLATE_TOP_Z + DATUM_RAIL_Z * 0.31,
    );

    left_x_datum + rear_y_datum + right_soft_capture + front_soft_capture
}

fn datum_pins() -> Part {
    let mut pins = Part::empty("closed_incubator_vibration_tilt_datum_pins");
    for (i, (x, y)) in datum_pin_points().into_iter().enumerate() {
        let boss = centered_cylinder(
            format!("closed_incubator_vibration_tilt_datum_pin_boss_{i}"),
            DATUM_PIN_D / 2.0 + 5.0,
            6.0,
            36,
        )
        .translate(x, y, DATUM_PLATE_TOP_Z + 3.0);
        let pin = centered_cylinder(
            format!("closed_incubator_vibration_tilt_datum_pin_{i}"),
            DATUM_PIN_D / 2.0,
            14.0,
            36,
        )
        .translate(x, y, DATUM_PLATE_TOP_Z + 7.0);
        pins = pins + boss + pin;
    }
    pins
}

fn rack_hard_stop_keys() -> Part {
    let mut keys = Part::empty("closed_incubator_vibration_tilt_rack_hard_stop_keys");
    for slot in 0..RACK_SLOT_COUNT {
        let (x, y) = slot_center(slot);
        let rear = centered_cube(
            format!(
                "closed_incubator_vibration_tilt_slot_{}_rear_hard_stop",
                slot_label(slot)
            ),
            CASSETTE_SURROGATE_X * 0.46,
            9.0,
            11.0,
        )
        .translate(
            x,
            y + CASSETTE_SURROGATE_Y / 2.0 + 12.0,
            DATUM_PLATE_TOP_Z + 5.5,
        );
        let left = centered_cube(
            format!(
                "closed_incubator_vibration_tilt_slot_{}_left_hard_stop",
                slot_label(slot)
            ),
            9.0,
            CASSETTE_SURROGATE_Y * 0.34,
            11.0,
        )
        .translate(
            x - CASSETTE_SURROGATE_X / 2.0 - 12.0,
            y,
            DATUM_PLATE_TOP_Z + 5.5,
        );
        keys = keys + rear + left;
    }
    keys
}

fn cassette_surrogate_weights() -> Part {
    let mut surrogates = Part::empty("closed_incubator_vibration_tilt_cassette_surrogate_weights");
    for slot in 0..RACK_SLOT_COUNT {
        let (x, y) = slot_center(slot);
        surrogates =
            surrogates + cassette_surrogate_weight(slot).translate(x, y, DATUM_PLATE_TOP_Z);
    }
    surrogates
}

fn cassette_surrogate_weight(slot: usize) -> Part {
    let label = slot_label(slot);
    let base = centered_cube(
        format!("closed_incubator_vibration_tilt_{label}_cassette_surrogate_base"),
        CASSETTE_SURROGATE_X,
        CASSETTE_SURROGATE_Y,
        CASSETTE_SURROGATE_BASE_Z,
    )
    .translate(0.0, 0.0, CASSETTE_SURROGATE_BASE_Z / 2.0);

    let ballast = centered_cube(
        format!("closed_incubator_vibration_tilt_{label}_dense_ballast_block"),
        BALLAST_BLOCK_X,
        BALLAST_BLOCK_Y,
        CASSETTE_BALLAST_Z,
    )
    .translate(
        0.0,
        0.0,
        CASSETTE_SURROGATE_BASE_Z + CASSETTE_BALLAST_Z / 2.0,
    );

    let handle_clearance = centered_cube(
        format!("closed_incubator_vibration_tilt_{label}_surrogate_handle_clearance"),
        64.0,
        20.0,
        CASSETTE_BALLAST_Z + 2.0,
    )
    .translate(
        0.0,
        0.0,
        CASSETTE_SURROGATE_BASE_Z + CASSETTE_BALLAST_Z / 2.0,
    );
    let center_target_cut = centered_cylinder(
        format!("closed_incubator_vibration_tilt_{label}_center_of_mass_target_cut"),
        CENTER_OF_MASS_TARGET_D / 2.0,
        3.0,
        36,
    )
    .translate(0.0, 0.0, CASSETTE_SURROGATE_Z - 1.0);

    base - underside_clearance_pads(label)
        + (ballast - ballast_puck_reliefs(label) - handle_clearance - center_target_cut)
        + center_of_mass_target(label)
        + surrogate_corner_keys(label)
        + surrogate_edge_guard(label)
}

fn underside_clearance_pads(label: &str) -> Part {
    let mut pads = Part::empty(format!(
        "closed_incubator_vibration_tilt_{label}_underside_clearance_pads"
    ));
    for (i, (sx, sy)) in [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)]
        .into_iter()
        .enumerate()
    {
        pads = pads
            + centered_cube(
                format!("closed_incubator_vibration_tilt_{label}_underside_relief_{i}"),
                34.0,
                24.0,
                5.0,
            )
            .translate(
                sx * (CASSETTE_SURROGATE_X / 2.0 - 28.0),
                sy * (CASSETTE_SURROGATE_Y / 2.0 - 24.0),
                2.0,
            );
    }
    pads
}

fn ballast_puck_reliefs(label: &str) -> Part {
    let mut reliefs = Part::empty(format!(
        "closed_incubator_vibration_tilt_{label}_ballast_puck_reliefs"
    ));
    for (i, (sx, sy)) in [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)]
        .into_iter()
        .enumerate()
    {
        reliefs = reliefs
            + centered_cylinder(
                format!("closed_incubator_vibration_tilt_{label}_ballast_puck_relief_{i}"),
                BALLAST_PUCK_D / 2.0,
                CASSETTE_BALLAST_Z + 1.0,
                40,
            )
            .translate(
                sx * (BALLAST_BLOCK_X / 2.0 - 28.0),
                sy * (BALLAST_BLOCK_Y / 2.0 - 24.0),
                CASSETTE_SURROGATE_BASE_Z + CASSETTE_BALLAST_Z / 2.0,
            );
    }
    reliefs
}

fn center_of_mass_target(label: &str) -> Part {
    let ring = centered_cylinder(
        format!("closed_incubator_vibration_tilt_{label}_center_of_mass_target_ring"),
        CENTER_OF_MASS_TARGET_D / 2.0,
        2.0,
        40,
    )
    .translate(0.0, 0.0, CASSETTE_SURROGATE_Z + 1.0);
    let cross_x = centered_cube(
        format!("closed_incubator_vibration_tilt_{label}_center_of_mass_target_x_tick"),
        CENTER_OF_MASS_TARGET_D + 12.0,
        2.0,
        2.2,
    )
    .translate(0.0, 0.0, CASSETTE_SURROGATE_Z + 1.1);
    let cross_y = centered_cube(
        format!("closed_incubator_vibration_tilt_{label}_center_of_mass_target_y_tick"),
        2.0,
        CENTER_OF_MASS_TARGET_D + 12.0,
        2.2,
    )
    .translate(0.0, 0.0, CASSETTE_SURROGATE_Z + 1.1);

    ring + cross_x + cross_y
}

fn surrogate_corner_keys(label: &str) -> Part {
    let mut keys = Part::empty(format!(
        "closed_incubator_vibration_tilt_{label}_surrogate_corner_keys"
    ));
    for (i, (sx, sy)) in [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)]
        .into_iter()
        .enumerate()
    {
        let key = centered_cylinder(
            format!("closed_incubator_vibration_tilt_{label}_corner_key_{i}"),
            6.0,
            4.0,
            28,
        )
        .translate(
            sx * (CASSETTE_SURROGATE_X / 2.0 - 18.0),
            sy * (CASSETTE_SURROGATE_Y / 2.0 - 18.0),
            CASSETTE_SURROGATE_Z + 2.0,
        );
        keys = keys + key;
    }
    keys
}

fn surrogate_edge_guard(label: &str) -> Part {
    let front = centered_cube(
        format!("closed_incubator_vibration_tilt_{label}_front_low_edge_guard"),
        CASSETTE_SURROGATE_X,
        6.0,
        5.0,
    )
    .translate(
        0.0,
        -(CASSETTE_SURROGATE_Y / 2.0 - 3.0),
        CASSETTE_SURROGATE_Z + 2.5,
    );
    let rear = centered_cube(
        format!("closed_incubator_vibration_tilt_{label}_rear_low_edge_guard"),
        CASSETTE_SURROGATE_X,
        6.0,
        5.0,
    )
    .translate(
        0.0,
        CASSETTE_SURROGATE_Y / 2.0 - 3.0,
        CASSETTE_SURROGATE_Z + 2.5,
    );
    front + rear
}

fn accelerometer_tilt_sensor_fixtures() -> Part {
    let mut fixtures =
        Part::empty("closed_incubator_vibration_tilt_accelerometer_tilt_sensor_fixtures");
    for slot in 0..RACK_SLOT_COUNT {
        let (x, y) = slot_center(slot);
        fixtures = fixtures
            + slot_sensor_fixture(slot).translate(
                x,
                y,
                DATUM_PLATE_TOP_Z + CASSETTE_SURROGATE_Z + SENSOR_POD_Z / 2.0,
            );
    }
    for reference in 0..CORNER_REFERENCE_SENSOR_COUNT {
        let (x, y) = reference_sensor_point(reference);
        fixtures = fixtures
            + corner_reference_sensor_fixture(reference).translate(
                x,
                y,
                DATUM_PLATE_TOP_Z + SENSOR_POD_Z / 2.0,
            );
    }
    fixtures
}

fn slot_sensor_fixture(slot: usize) -> Part {
    let label = slot_label(slot);
    let body = centered_cube(
        format!("closed_incubator_vibration_tilt_{label}_sensor_pod_body"),
        SENSOR_POD_X,
        SENSOR_POD_Y,
        SENSOR_POD_Z,
    );
    let accelerometer = centered_cube(
        format!("closed_incubator_vibration_tilt_{label}_triax_accelerometer_recess"),
        ACCELEROMETER_POCKET_X,
        ACCELEROMETER_POCKET_Y,
        SENSOR_RECESS_DEPTH + 1.0,
    )
    .translate(
        -22.0,
        0.0,
        SENSOR_POD_Z / 2.0 - SENSOR_RECESS_DEPTH / 2.0 + 0.4,
    );
    let tilt_sensor = centered_cube(
        format!("closed_incubator_vibration_tilt_{label}_tilt_sensor_recess"),
        TILT_SENSOR_POCKET_X,
        TILT_SENSOR_POCKET_Y,
        SENSOR_RECESS_DEPTH + 1.0,
    )
    .translate(
        24.0,
        0.0,
        SENSOR_POD_Z / 2.0 - SENSOR_RECESS_DEPTH / 2.0 + 0.4,
    );
    let cable_exit = centered_cylinder(
        format!("closed_incubator_vibration_tilt_{label}_sensor_cable_exit"),
        SENSOR_CABLE_PASSAGE_D / 2.0,
        SENSOR_POD_Y + 4.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, SENSOR_POD_Y / 2.0 - 8.0, 0.0);

    body - accelerometer - tilt_sensor - cable_exit
        + sensor_mount_bosses(label)
        + sensor_axis_keys(label)
        + sensor_retainer_bridge(label)
}

fn corner_reference_sensor_fixture(reference: usize) -> Part {
    let label = reference_label(reference);
    let body = centered_cube(
        format!("closed_incubator_vibration_tilt_{label}_corner_reference_sensor_body"),
        SENSOR_POD_X * 0.82,
        SENSOR_POD_Y * 0.82,
        SENSOR_POD_Z,
    );
    let accel_recess = centered_cube(
        format!("closed_incubator_vibration_tilt_{label}_reference_accelerometer_recess"),
        ACCELEROMETER_POCKET_X,
        ACCELEROMETER_POCKET_Y,
        SENSOR_RECESS_DEPTH + 1.0,
    )
    .translate(
        0.0,
        -9.0,
        SENSOR_POD_Z / 2.0 - SENSOR_RECESS_DEPTH / 2.0 + 0.4,
    );
    let tilt_bubble = centered_cylinder(
        format!("closed_incubator_vibration_tilt_{label}_reference_tilt_bubble_recess"),
        13.0,
        SENSOR_RECESS_DEPTH + 1.0,
        36,
    )
    .translate(
        0.0,
        17.0,
        SENSOR_POD_Z / 2.0 - SENSOR_RECESS_DEPTH / 2.0 + 0.4,
    );
    let cable_exit = centered_cylinder(
        format!("closed_incubator_vibration_tilt_{label}_reference_cable_exit"),
        SENSOR_CABLE_PASSAGE_D / 2.0,
        SENSOR_POD_Y,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, SENSOR_POD_Y / 2.0 - 5.0, 0.0);

    body - accel_recess - tilt_bubble - cable_exit
        + sensor_mount_bosses(label)
        + sensor_axis_keys(label)
}

fn sensor_mount_bosses(label: &str) -> Part {
    let mut bosses = Part::empty(format!(
        "closed_incubator_vibration_tilt_{label}_sensor_mount_bosses"
    ));
    for (i, (x, y)) in [(-38.0, -22.0), (38.0, -22.0), (0.0, 24.0)]
        .into_iter()
        .enumerate()
    {
        let boss = centered_cylinder(
            format!("closed_incubator_vibration_tilt_{label}_sensor_mount_boss_{i}"),
            SENSOR_MOUNT_BOSS_D / 2.0,
            4.0,
            28,
        )
        .translate(x, y, SENSOR_POD_Z / 2.0 + 2.0);
        let pilot = centered_cylinder(
            format!("closed_incubator_vibration_tilt_{label}_sensor_mount_pilot_{i}"),
            1.4,
            4.4,
            20,
        )
        .translate(x, y, SENSOR_POD_Z / 2.0 + 2.0);
        bosses = bosses + (boss - pilot);
    }
    bosses
}

fn sensor_axis_keys(label: &str) -> Part {
    let x_axis = centered_cube(
        format!("closed_incubator_vibration_tilt_{label}_sensor_x_axis_key"),
        38.0,
        3.0,
        2.0,
    )
    .translate(-22.0, -SENSOR_POD_Y / 2.0 + 9.0, SENSOR_POD_Z / 2.0 + 1.0);
    let y_axis = centered_cube(
        format!("closed_incubator_vibration_tilt_{label}_sensor_y_axis_key"),
        3.0,
        30.0,
        2.0,
    )
    .translate(-40.0, -SENSOR_POD_Y / 2.0 + 22.0, SENSOR_POD_Z / 2.0 + 1.0);
    let tilt_zero = centered_cube(
        format!("closed_incubator_vibration_tilt_{label}_tilt_zero_key"),
        42.0,
        4.0,
        2.0,
    )
    .translate(24.0, SENSOR_POD_Y / 2.0 - 10.0, SENSOR_POD_Z / 2.0 + 1.0);

    x_axis + y_axis + tilt_zero
}

fn sensor_retainer_bridge(label: &str) -> Part {
    let left = centered_cube(
        format!("closed_incubator_vibration_tilt_{label}_accelerometer_retainer_bridge"),
        ACCELEROMETER_POCKET_X + 12.0,
        4.0,
        4.0,
    )
    .translate(-22.0, 0.0, SENSOR_POD_Z / 2.0 + 2.0);
    let right = centered_cube(
        format!("closed_incubator_vibration_tilt_{label}_tilt_sensor_retainer_bridge"),
        TILT_SENSOR_POCKET_X + 12.0,
        4.0,
        4.0,
    )
    .translate(24.0, 0.0, SENSOR_POD_Z / 2.0 + 2.0);
    left + right
}

fn anti_slip_clamps() -> Part {
    let mut clamps = Part::empty("closed_incubator_vibration_tilt_anti_slip_clamps");

    for row in 0..SLOT_ROWS {
        let y = row_center_y(row);
        clamps = clamps + side_clamp(row, false).translate(left_clamp_x(), y, DATUM_PLATE_TOP_Z);
        clamps = clamps + side_clamp(row, true).translate(right_clamp_x(), y, DATUM_PLATE_TOP_Z);
    }

    for col in 0..SLOT_COLS {
        let x = col_center_x(col);
        clamps = clamps + toe_clamp(col, false).translate(x, front_clamp_y(), DATUM_PLATE_TOP_Z);
        clamps = clamps + toe_clamp(col, true).translate(x, rear_clamp_y(), DATUM_PLATE_TOP_Z);
    }

    clamps
}

fn side_clamp(row: usize, right_side: bool) -> Part {
    let side = if right_side { "right" } else { "left" };
    let body = centered_cube(
        format!("closed_incubator_vibration_tilt_{side}_row_{row}_side_clamp_body"),
        CLAMP_BODY_X,
        CLAMP_BODY_Y,
        CLAMP_BODY_Z,
    )
    .translate(0.0, 0.0, CLAMP_BODY_Z / 2.0);
    let jaw_sign = if right_side { -1.0 } else { 1.0 };
    let jaw = centered_cube(
        format!("closed_incubator_vibration_tilt_{side}_row_{row}_rubber_jaw"),
        CLAMP_JAW_X,
        CLAMP_JAW_Y,
        RUBBER_PAD_Z,
    )
    .translate(
        jaw_sign * (CLAMP_BODY_X / 2.0 - CLAMP_TOE_OVERHANG),
        0.0,
        CLAMP_BODY_Z + RUBBER_PAD_Z / 2.0,
    );
    let pivot = centered_cylinder(
        format!("closed_incubator_vibration_tilt_{side}_row_{row}_cam_pivot"),
        CLAMP_SCREW_D / 2.0,
        CLAMP_BODY_Y + 2.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, 0.0, CLAMP_BODY_Z / 2.0);
    let screw_clearance = centered_cylinder(
        format!("closed_incubator_vibration_tilt_{side}_row_{row}_thumb_screw_clearance"),
        3.2,
        CLAMP_BODY_Z + 4.0,
        24,
    )
    .translate(-jaw_sign * 24.0, 0.0, CLAMP_BODY_Z / 2.0);

    body - screw_clearance + pivot + jaw
}

fn toe_clamp(col: usize, rear_side: bool) -> Part {
    let side = if rear_side { "rear" } else { "front" };
    let body = centered_cube(
        format!("closed_incubator_vibration_tilt_{side}_col_{col}_toe_clamp_body"),
        CLAMP_BODY_X,
        CLAMP_BODY_Y,
        CLAMP_BODY_Z,
    )
    .translate(0.0, 0.0, CLAMP_BODY_Z / 2.0);
    let jaw_sign = if rear_side { -1.0 } else { 1.0 };
    let jaw = centered_cube(
        format!("closed_incubator_vibration_tilt_{side}_col_{col}_anti_slip_toe"),
        CLAMP_JAW_X,
        CLAMP_JAW_Y,
        RUBBER_PAD_Z,
    )
    .translate(
        0.0,
        jaw_sign * (CLAMP_BODY_Y / 2.0 - CLAMP_TOE_OVERHANG),
        CLAMP_BODY_Z + RUBBER_PAD_Z / 2.0,
    );
    let cam_slot = centered_cube(
        format!("closed_incubator_vibration_tilt_{side}_col_{col}_cam_adjust_slot"),
        58.0,
        8.0,
        CLAMP_BODY_Z + 2.0,
    )
    .translate(0.0, 0.0, CLAMP_BODY_Z / 2.0);
    let screw = centered_cylinder(
        format!("closed_incubator_vibration_tilt_{side}_col_{col}_clamp_screw_washer"),
        CLAMP_SCREW_D / 2.0,
        4.0,
        32,
    )
    .translate(0.0, -jaw_sign * 13.0, CLAMP_BODY_Z + 2.0);

    body - cam_slot + screw + jaw
}

fn isolation_foot_test_pads() -> Part {
    let mut pads = Part::empty("closed_incubator_vibration_tilt_isolation_foot_test_pads");
    for (i, (x, y)) in foot_pad_points().into_iter().enumerate() {
        pads = pads + isolation_foot_pad(i).translate(x, y, FOOT_PAD_Z / 2.0);
    }
    pads
}

fn isolation_foot_pad(index: usize) -> Part {
    let pad = centered_cylinder(
        format!("closed_incubator_vibration_tilt_isolation_foot_pad_{index}"),
        FOOT_PAD_D / 2.0,
        FOOT_PAD_Z,
        56,
    );
    let elastomer_recess = centered_cylinder(
        format!("closed_incubator_vibration_tilt_isolation_foot_elastomer_recess_{index}"),
        FOOT_PAD_RECESS_D / 2.0,
        7.0,
        48,
    )
    .translate(0.0, 0.0, FOOT_PAD_Z / 2.0 - 2.5);
    let durometer_token = centered_cylinder(
        format!("closed_incubator_vibration_tilt_durometer_token_recess_{index}"),
        DUROMETER_TOKEN_D / 2.0,
        5.0,
        32,
    )
    .translate(0.0, FOOT_PAD_D / 2.0 - 20.0, FOOT_PAD_Z / 2.0 - 1.6);

    pad - elastomer_recess - durometer_token + shear_witness_bars(index) + foot_label_key(index)
}

fn shear_witness_bars(index: usize) -> Part {
    let mut bars = Part::empty(format!(
        "closed_incubator_vibration_tilt_isolation_foot_shear_witness_bars_{index}"
    ));
    for i in 0..SHEAR_WITNESS_BAR_COUNT {
        bars = bars
            + centered_cube(
                format!("closed_incubator_vibration_tilt_foot_{index}_shear_bar_{i}"),
                4.0,
                46.0,
                2.0,
            )
            .translate(
                centered_index(i, SHEAR_WITNESS_BAR_COUNT, 9.0),
                -7.0,
                FOOT_PAD_Z / 2.0 + 1.0,
            );
    }
    bars
}

fn foot_label_key(index: usize) -> Part {
    let mut ticks = Part::empty(format!(
        "closed_incubator_vibration_tilt_foot_{index}_position_label_key"
    ));
    for i in 0..=index {
        ticks = ticks
            + centered_cube(
                format!("closed_incubator_vibration_tilt_foot_{index}_label_tick_{i}"),
                4.0,
                12.0,
                2.0,
            )
            .translate(
                centered_index(i, index + 1, 7.0),
                -(FOOT_PAD_D / 2.0 - 16.0),
                FOOT_PAD_Z / 2.0 + 1.0,
            );
    }
    ticks
}

fn cable_routing() -> Part {
    let trough = centered_cube(
        "closed_incubator_vibration_tilt_rear_cable_trough",
        CABLE_TROUGH_X,
        CABLE_TROUGH_Y,
        CABLE_TROUGH_Z,
    )
    .translate(
        0.0,
        rear_cable_y(),
        DATUM_PLATE_TOP_Z + CABLE_TROUGH_Z / 2.0,
    );
    let trough_lid_land = centered_cube(
        "closed_incubator_vibration_tilt_rear_cable_trough_lid_land",
        CABLE_TROUGH_X - 42.0,
        8.0,
        4.0,
    )
    .translate(
        0.0,
        rear_cable_y() + CABLE_TROUGH_Y / 2.0 - 5.0,
        DATUM_PLATE_TOP_Z + CABLE_TROUGH_Z + 2.0,
    );
    let bulkhead = cable_bulkhead();

    trough + trough_lid_land + cable_branches() + cable_clamp_comb() + bulkhead
}

fn cable_branches() -> Part {
    let mut branches = Part::empty("closed_incubator_vibration_tilt_sensor_cable_branches");
    for slot in 0..RACK_SLOT_COUNT {
        let (x, y) = slot_center(slot);
        branches = branches + cable_branch(format!("slot_{}", slot_label(slot)), x, y);
    }
    for reference in 0..CORNER_REFERENCE_SENSOR_COUNT {
        let (x, y) = reference_sensor_point(reference);
        branches = branches + cable_branch(reference_label(reference), x, y);
    }
    branches
}

fn cable_branch(label: impl Into<String>, x: f64, y: f64) -> Part {
    let label = label.into();
    let branch_len = rear_cable_y() - y - CABLE_TROUGH_Y / 2.0 + 6.0;
    centered_cube(
        format!("closed_incubator_vibration_tilt_{label}_cable_branch_channel"),
        CABLE_BRANCH_W,
        branch_len.max(18.0),
        CABLE_BRANCH_Z,
    )
    .translate(
        x,
        y + branch_len.max(18.0) / 2.0,
        DATUM_PLATE_TOP_Z + CABLE_BRANCH_Z / 2.0 + 2.0,
    )
}

fn cable_clamp_comb() -> Part {
    let comb = centered_cube(
        "closed_incubator_vibration_tilt_cable_clamp_comb_body",
        CABLE_CLAMP_COUNT as f64 * CABLE_CLAMP_PITCH_X + 36.0,
        32.0,
        28.0,
    )
    .translate(0.0, rear_cable_y() - 34.0, DATUM_PLATE_TOP_Z + 14.0);

    comb - cable_clamp_passages() + cable_number_ticks()
}

fn cable_clamp_passages() -> Part {
    let mut passages = Part::empty("closed_incubator_vibration_tilt_cable_clamp_passages");
    for i in 0..CABLE_CLAMP_COUNT {
        let x = centered_index(i, CABLE_CLAMP_COUNT, CABLE_CLAMP_PITCH_X);
        passages = passages
            + centered_cylinder(
                format!("closed_incubator_vibration_tilt_cable_clamp_passage_{i}"),
                SENSOR_CABLE_PASSAGE_D / 2.0,
                36.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, rear_cable_y() - 34.0, DATUM_PLATE_TOP_Z + 14.0);
    }
    passages
}

fn cable_number_ticks() -> Part {
    let mut ticks = Part::empty("closed_incubator_vibration_tilt_cable_clamp_number_ticks");
    for i in 0..CABLE_CLAMP_COUNT {
        let x = centered_index(i, CABLE_CLAMP_COUNT, CABLE_CLAMP_PITCH_X);
        for tick in 0..=(i % 5) {
            ticks = ticks
                + centered_cube(
                    format!("closed_incubator_vibration_tilt_cable_{i}_number_tick_{tick}"),
                    3.0,
                    9.0,
                    2.0,
                )
                .translate(
                    x - 8.0 + tick as f64 * 4.0,
                    rear_cable_y() - 51.0,
                    DATUM_PLATE_TOP_Z + 29.0,
                );
        }
    }
    ticks
}

fn cable_bulkhead() -> Part {
    let body = centered_cube(
        "closed_incubator_vibration_tilt_cable_bulkhead_plate",
        BULKHEAD_X,
        BULKHEAD_Y,
        BULKHEAD_Z,
    )
    .translate(0.0, rear_cable_y() + 40.0, DECK_TOP_Z + BULKHEAD_Z / 2.0);
    let pass_a = centered_cube(
        "closed_incubator_vibration_tilt_cable_bulkhead_left_passage",
        48.0,
        BULKHEAD_Y + 2.0,
        18.0,
    )
    .translate(-36.0, rear_cable_y() + 40.0, DECK_TOP_Z + BULKHEAD_Z / 2.0);
    let pass_b = centered_cube(
        "closed_incubator_vibration_tilt_cable_bulkhead_right_passage",
        48.0,
        BULKHEAD_Y + 2.0,
        18.0,
    )
    .translate(36.0, rear_cable_y() + 40.0, DECK_TOP_Z + BULKHEAD_Z / 2.0);

    body - pass_a - pass_b
}

fn repeatable_position_labels() -> Part {
    slot_label_lands() + foot_label_lands() + fiducial_targets() + rack_axis_reference_marks()
}

fn slot_label_lands() -> Part {
    let mut lands = Part::empty("closed_incubator_vibration_tilt_slot_position_label_lands");
    for slot in 0..RACK_SLOT_COUNT {
        let (x, y) = slot_center(slot);
        let label_y = y - CASSETTE_SURROGATE_Y / 2.0 - 25.0;
        let land = centered_cube(
            format!(
                "closed_incubator_vibration_tilt_slot_{}_label_land",
                slot_label(slot)
            ),
            LABEL_LAND_X,
            LABEL_LAND_Y,
            LABEL_LAND_Z,
        )
        .translate(x, label_y, DATUM_PLATE_TOP_Z + LABEL_LAND_Z / 2.0 + 1.5);
        lands = lands + land + slot_label_tick_key(slot, x, label_y);
    }
    lands
}

fn slot_label_tick_key(slot: usize, x: f64, y: f64) -> Part {
    let mut ticks = Part::empty(format!(
        "closed_incubator_vibration_tilt_slot_{}_label_tick_key",
        slot_label(slot)
    ));
    for i in 0..=slot {
        ticks = ticks
            + centered_cube(
                format!(
                    "closed_incubator_vibration_tilt_slot_{}_label_tick_{i}",
                    slot_label(slot)
                ),
                LABEL_TICK_X,
                LABEL_TICK_Y,
                LABEL_LAND_Z + 1.0,
            )
            .translate(
                x + centered_index(i, slot + 1, 7.0),
                y,
                DATUM_PLATE_TOP_Z + LABEL_LAND_Z + 2.0,
            );
    }
    ticks
}

fn foot_label_lands() -> Part {
    let mut lands = Part::empty("closed_incubator_vibration_tilt_foot_position_label_lands");
    for (i, (x, y)) in foot_pad_points().into_iter().enumerate() {
        lands = lands
            + centered_cube(
                format!("closed_incubator_vibration_tilt_foot_{i}_label_land"),
                42.0,
                18.0,
                LABEL_LAND_Z,
            )
            .translate(x, y + 58.0, DECK_TOP_Z + LABEL_LAND_Z / 2.0 + 1.0);
    }
    lands
}

fn fiducial_targets() -> Part {
    let mut targets = Part::empty("closed_incubator_vibration_tilt_fiducial_targets");
    for slot in 0..RACK_SLOT_COUNT {
        let (x, y) = slot_center(slot);
        targets = targets
            + fiducial_target(format!("slot_{}", slot_label(slot))).translate(
                x + CASSETTE_SURROGATE_X / 2.0 - 20.0,
                y - CASSETTE_SURROGATE_Y / 2.0 + 20.0,
                DATUM_PLATE_TOP_Z + 2.0,
            );
    }
    for (i, (x, y)) in datum_corner_fiducials().into_iter().enumerate() {
        targets = targets
            + fiducial_target(format!("datum_corner_{i}")).translate(x, y, DATUM_PLATE_TOP_Z + 2.0);
    }
    targets
}

fn fiducial_target(label: impl Into<String>) -> Part {
    let label = label.into();
    let disk = centered_cylinder(
        format!("closed_incubator_vibration_tilt_{label}_fiducial_disk"),
        FIDUCIAL_D / 2.0,
        2.0,
        40,
    );
    let x_groove = centered_cube(
        format!("closed_incubator_vibration_tilt_{label}_fiducial_x_groove"),
        FIDUCIAL_D + 4.0,
        FIDUCIAL_GROOVE_W,
        2.4,
    );
    let y_groove = centered_cube(
        format!("closed_incubator_vibration_tilt_{label}_fiducial_y_groove"),
        FIDUCIAL_GROOVE_W,
        FIDUCIAL_D + 4.0,
        2.4,
    );
    disk - x_groove - y_groove
}

fn rack_axis_reference_marks() -> Part {
    let x_axis = centered_cube(
        "closed_incubator_vibration_tilt_rack_x_axis_reference_mark",
        DATUM_PLATE_X - 130.0,
        4.0,
        2.2,
    )
    .translate(
        DATUM_PLATE_CENTER_X,
        DATUM_PLATE_CENTER_Y - DATUM_PLATE_Y / 2.0 + 36.0,
        DATUM_PLATE_TOP_Z + 2.0,
    );
    let y_axis = centered_cube(
        "closed_incubator_vibration_tilt_rack_y_axis_reference_mark",
        4.0,
        DATUM_PLATE_Y - 130.0,
        2.2,
    )
    .translate(
        DATUM_PLATE_CENTER_X - DATUM_PLATE_X / 2.0 + 36.0,
        DATUM_PLATE_CENTER_Y,
        DATUM_PLATE_TOP_Z + 2.0,
    );

    x_axis + y_axis
}

fn tilt_reference_tools() -> Part {
    bubble_level_cradle() + tilt_wedge_ladder() + zero_reference_bar()
}

fn bubble_level_cradle() -> Part {
    let base = centered_cube(
        "closed_incubator_vibration_tilt_bubble_level_cradle_base",
        BUBBLE_LEVEL_X,
        BUBBLE_LEVEL_Y,
        BUBBLE_LEVEL_Z,
    )
    .translate(
        -DATUM_PLATE_X / 2.0 + 142.0,
        DATUM_PLATE_CENTER_Y - DATUM_PLATE_Y / 2.0 - 56.0,
        DECK_TOP_Z + BUBBLE_LEVEL_Z / 2.0,
    );
    let vial_recess = centered_cylinder(
        "closed_incubator_vibration_tilt_bubble_level_vial_recess",
        9.0,
        BUBBLE_LEVEL_X - 28.0,
        36,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(
        -DATUM_PLATE_X / 2.0 + 142.0,
        DATUM_PLATE_CENTER_Y - DATUM_PLATE_Y / 2.0 - 56.0,
        DECK_TOP_Z + BUBBLE_LEVEL_Z / 2.0,
    );

    base - vial_recess
}

fn tilt_wedge_ladder() -> Part {
    let mut wedges = Part::empty("closed_incubator_vibration_tilt_reference_wedge_ladder");
    for i in 0..TILT_WEDGE_COUNT {
        let x = centered_index(i, TILT_WEDGE_COUNT, TILT_WEDGE_X + 14.0) + 166.0;
        let y = DATUM_PLATE_CENTER_Y - DATUM_PLATE_Y / 2.0 - 56.0;
        wedges = wedges + tilt_wedge(i).translate(x, y, DECK_TOP_Z + TILT_WEDGE_Z / 2.0);
    }
    wedges
}

fn tilt_wedge(index: usize) -> Part {
    let body = centered_cube(
        format!("closed_incubator_vibration_tilt_reference_wedge_{index}_body"),
        TILT_WEDGE_X,
        TILT_WEDGE_Y,
        TILT_WEDGE_Z,
    );
    let slope_cut = centered_cube(
        format!("closed_incubator_vibration_tilt_reference_wedge_{index}_slope_relief"),
        TILT_WEDGE_X + 2.0,
        TILT_WEDGE_Y + 2.0,
        TILT_WEDGE_Z,
    )
    .rotate(0.0, 0.0, index as f64 * 1.25)
    .translate(0.0, 0.0, TILT_WEDGE_Z / 2.0 + 5.0);
    let label_ticks = wedge_ticks(index);

    body - slope_cut + label_ticks
}

fn wedge_ticks(index: usize) -> Part {
    let mut ticks = Part::empty(format!(
        "closed_incubator_vibration_tilt_reference_wedge_{index}_ticks"
    ));
    for tick in 0..=index {
        ticks = ticks
            + centered_cube(
                format!("closed_incubator_vibration_tilt_reference_wedge_{index}_tick_{tick}"),
                4.0,
                14.0,
                2.0,
            )
            .translate(
                centered_index(tick, index + 1, 8.0),
                -TILT_WEDGE_Y / 2.0 + 12.0,
                TILT_WEDGE_Z / 2.0 + 1.0,
            );
    }
    ticks
}

fn zero_reference_bar() -> Part {
    centered_cube(
        "closed_incubator_vibration_tilt_zero_reference_bar",
        DATUM_PLATE_X - 140.0,
        10.0,
        12.0,
    )
    .translate(
        DATUM_PLATE_CENTER_X,
        DATUM_PLATE_CENTER_Y - DATUM_PLATE_Y / 2.0 - 104.0,
        DECK_TOP_Z + 6.0,
    )
}

fn service_clearance_gauges() -> Part {
    let front_access = centered_cube(
        "closed_incubator_vibration_tilt_front_sensor_access_clearance_gauge",
        SERVICE_KEEP_OUT_X,
        SERVICE_KEEP_OUT_Y,
        SERVICE_KEEP_OUT_Z,
    )
    .translate(
        DATUM_PLATE_CENTER_X,
        DATUM_PLATE_CENTER_Y - DATUM_PLATE_Y / 2.0 - SERVICE_KEEP_OUT_Y / 2.0 - 20.0,
        DATUM_PLATE_TOP_Z + SENSOR_ACCESS_CLEARANCE_Z,
    );
    let rear_cable_access = centered_cube(
        "closed_incubator_vibration_tilt_rear_cable_service_clearance_gauge",
        CABLE_TROUGH_X,
        CABLE_SERVICE_CLEARANCE_Y,
        SERVICE_KEEP_OUT_Z,
    )
    .translate(
        DATUM_PLATE_CENTER_X,
        rear_cable_y() + CABLE_TROUGH_Y / 2.0 + CABLE_SERVICE_CLEARANCE_Y / 2.0,
        DATUM_PLATE_TOP_Z + SENSOR_ACCESS_CLEARANCE_Z * 0.7,
    );
    let side_service_left = centered_cube(
        "closed_incubator_vibration_tilt_left_clamp_service_clearance_gauge",
        64.0,
        DATUM_PLATE_Y,
        SERVICE_KEEP_OUT_Z,
    )
    .translate(
        DATUM_PLATE_CENTER_X - DATUM_PLATE_X / 2.0 - 48.0,
        DATUM_PLATE_CENTER_Y,
        DATUM_PLATE_TOP_Z + 58.0,
    );
    let side_service_right = centered_cube(
        "closed_incubator_vibration_tilt_right_clamp_service_clearance_gauge",
        64.0,
        DATUM_PLATE_Y,
        SERVICE_KEEP_OUT_Z,
    )
    .translate(
        DATUM_PLATE_CENTER_X + DATUM_PLATE_X / 2.0 + 48.0,
        DATUM_PLATE_CENTER_Y,
        DATUM_PLATE_TOP_Z + 58.0,
    );

    front_access + rear_cable_access + side_service_left + side_service_right
}

fn slot_center(slot: usize) -> (f64, f64) {
    let col = slot % SLOT_COLS;
    let row = slot / SLOT_COLS;
    (col_center_x(col), row_center_y(row))
}

fn col_center_x(col: usize) -> f64 {
    DATUM_PLATE_CENTER_X + centered_index(col, SLOT_COLS, SLOT_PITCH_X)
}

fn row_center_y(row: usize) -> f64 {
    DATUM_PLATE_CENTER_Y + centered_index(row, SLOT_ROWS, SLOT_PITCH_Y)
}

fn reference_sensor_point(reference: usize) -> (f64, f64) {
    let x = if reference % 2 == 0 {
        DATUM_PLATE_CENTER_X - DATUM_PLATE_X / 2.0 + 74.0
    } else {
        DATUM_PLATE_CENTER_X + DATUM_PLATE_X / 2.0 - 74.0
    };
    let y = if reference / 2 == 0 {
        DATUM_PLATE_CENTER_Y - DATUM_PLATE_Y / 2.0 + 72.0
    } else {
        DATUM_PLATE_CENTER_Y + DATUM_PLATE_Y / 2.0 - 72.0
    };
    (x, y)
}

fn datum_pin_points() -> [(f64, f64); 3] {
    [
        (
            DATUM_PLATE_CENTER_X - DATUM_PLATE_X / 2.0 + 42.0,
            DATUM_PLATE_CENTER_Y + DATUM_PLATE_Y / 2.0 - 42.0,
        ),
        (
            DATUM_PLATE_CENTER_X + DATUM_PLATE_X / 2.0 - 42.0,
            DATUM_PLATE_CENTER_Y + DATUM_PLATE_Y / 2.0 - 42.0,
        ),
        (
            DATUM_PLATE_CENTER_X - DATUM_PLATE_X / 2.0 + 42.0,
            DATUM_PLATE_CENTER_Y - DATUM_PLATE_Y / 2.0 + 42.0,
        ),
    ]
}

fn datum_corner_fiducials() -> [(f64, f64); 4] {
    [
        (
            DATUM_PLATE_CENTER_X - DATUM_PLATE_X / 2.0 + 70.0,
            DATUM_PLATE_CENTER_Y - DATUM_PLATE_Y / 2.0 + 70.0,
        ),
        (
            DATUM_PLATE_CENTER_X + DATUM_PLATE_X / 2.0 - 70.0,
            DATUM_PLATE_CENTER_Y - DATUM_PLATE_Y / 2.0 + 70.0,
        ),
        (
            DATUM_PLATE_CENTER_X - DATUM_PLATE_X / 2.0 + 70.0,
            DATUM_PLATE_CENTER_Y + DATUM_PLATE_Y / 2.0 - 70.0,
        ),
        (
            DATUM_PLATE_CENTER_X + DATUM_PLATE_X / 2.0 - 70.0,
            DATUM_PLATE_CENTER_Y + DATUM_PLATE_Y / 2.0 - 70.0,
        ),
    ]
}

fn deck_mount_points() -> [(f64, f64); 8] {
    [
        (-(DECK_X / 2.0 - 44.0), -(DECK_Y / 2.0 - 44.0)),
        (DECK_X / 2.0 - 44.0, -(DECK_Y / 2.0 - 44.0)),
        (-(DECK_X / 2.0 - 44.0), DECK_Y / 2.0 - 44.0),
        (DECK_X / 2.0 - 44.0, DECK_Y / 2.0 - 44.0),
        (0.0, -(DECK_Y / 2.0 - 44.0)),
        (0.0, DECK_Y / 2.0 - 44.0),
        (-(DECK_X / 2.0 - 44.0), 0.0),
        (DECK_X / 2.0 - 44.0, 0.0),
    ]
}

fn foot_pad_points() -> [(f64, f64); ISOLATION_FOOT_COUNT] {
    [
        (-FOOT_PAD_SPAN_X / 2.0, -FOOT_PAD_SPAN_Y / 2.0),
        (FOOT_PAD_SPAN_X / 2.0, -FOOT_PAD_SPAN_Y / 2.0),
        (-FOOT_PAD_SPAN_X / 2.0, FOOT_PAD_SPAN_Y / 2.0),
        (FOOT_PAD_SPAN_X / 2.0, FOOT_PAD_SPAN_Y / 2.0),
    ]
}

fn slot_label(slot: usize) -> &'static str {
    match slot {
        0 => "s1",
        1 => "s2",
        2 => "s3",
        3 => "s4",
        4 => "s5",
        5 => "s6",
        _ => "unknown",
    }
}

fn reference_label(reference: usize) -> &'static str {
    match reference {
        0 => "front_left_reference",
        1 => "front_right_reference",
        2 => "rear_left_reference",
        3 => "rear_right_reference",
        _ => "unknown_reference",
    }
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn slot_span_x() -> f64 {
    (SLOT_COLS as f64 - 1.0) * SLOT_PITCH_X
}

fn slot_span_y() -> f64 {
    (SLOT_ROWS as f64 - 1.0) * SLOT_PITCH_Y
}

fn rear_cable_y() -> f64 {
    DATUM_PLATE_CENTER_Y + DATUM_PLATE_Y / 2.0 + 42.0
}

fn left_clamp_x() -> f64 {
    DATUM_PLATE_CENTER_X - slot_span_x() / 2.0 - CASSETTE_SURROGATE_X / 2.0 - 46.0
}

fn right_clamp_x() -> f64 {
    DATUM_PLATE_CENTER_X + slot_span_x() / 2.0 + CASSETTE_SURROGATE_X / 2.0 + 46.0
}

fn front_clamp_y() -> f64 {
    DATUM_PLATE_CENTER_Y - slot_span_y() / 2.0 - CASSETTE_SURROGATE_Y / 2.0 - 38.0
}

fn rear_clamp_y() -> f64 {
    DATUM_PLATE_CENTER_Y + slot_span_y() / 2.0 + CASSETTE_SURROGATE_Y / 2.0 + 38.0
}

fn sensor_cable_radial_clearance() -> f64 {
    (SENSOR_CABLE_PASSAGE_D - SENSOR_CABLE_BUNDLE_D) / 2.0
}

fn sensor_access_clearance() -> f64 {
    SENSOR_ACCESS_CLEARANCE_Z - CASSETTE_SURROGATE_Z - SENSOR_POD_Z
}

fn cable_service_clearance() -> f64 {
    DECK_Y / 2.0 - (rear_cable_y() + CABLE_TROUGH_Y / 2.0)
}

fn anti_slip_clamp_count() -> usize {
    SIDE_CLAMP_COUNT + TOE_CLAMP_COUNT
}

fn cable_branch_count() -> usize {
    RACK_SLOT_COUNT + CORNER_REFERENCE_SENSOR_COUNT
}

fn isolation_pad_count() -> usize {
    foot_pad_points().len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn six_slot_layout_fits_on_datum_plate() {
        assert_eq!(RACK_SLOT_COUNT, 6);
        assert_eq!(SLOT_COLS * SLOT_ROWS, RACK_SLOT_COUNT);
        assert!(slot_span_x() + CASSETTE_SURROGATE_X < DATUM_PLATE_X - 70.0);
        assert!(slot_span_y() + CASSETTE_SURROGATE_Y < DATUM_PLATE_Y - 72.0);

        for slot in 0..RACK_SLOT_COUNT {
            let (x, y) = slot_center(slot);
            assert!(x.abs() + CASSETTE_SURROGATE_X / 2.0 < DATUM_PLATE_X / 2.0 - 24.0);
            assert!(
                (y - DATUM_PLATE_CENTER_Y).abs() + CASSETTE_SURROGATE_Y / 2.0
                    < DATUM_PLATE_Y / 2.0 - 24.0
            );
        }
    }

    #[test]
    fn sensor_and_cable_counts_match_mapping_process() {
        assert_eq!(SENSOR_FIXTURE_COUNT, 10);
        assert_eq!(cable_branch_count(), SENSOR_FIXTURE_COUNT);
        assert!(sensor_cable_radial_clearance() >= 1.5);
        assert!(sensor_access_clearance() >= 72.0);
        assert_eq!(BALLAST_POCKET_COUNT, 4);
    }

    #[test]
    fn clamps_are_outside_surrogate_slot_envelope() {
        assert_eq!(anti_slip_clamp_count(), CLAMP_COUNT);
        assert!(left_clamp_x() < col_center_x(0) - CASSETTE_SURROGATE_X / 2.0);
        assert!(right_clamp_x() > col_center_x(SLOT_COLS - 1) + CASSETTE_SURROGATE_X / 2.0);
        assert!(front_clamp_y() < row_center_y(0) - CASSETTE_SURROGATE_Y / 2.0);
        assert!(rear_clamp_y() > row_center_y(SLOT_ROWS - 1) + CASSETTE_SURROGATE_Y / 2.0);
    }

    #[test]
    fn isolation_and_labeling_features_are_complete() {
        assert_eq!(isolation_pad_count(), ISOLATION_FOOT_COUNT);
        assert_eq!(POSITION_LABEL_COUNT, RACK_SLOT_COUNT + ISOLATION_FOOT_COUNT);
        assert_eq!(datum_corner_fiducials().len(), 4);
        assert!(FOOT_PAD_SPAN_X > DATUM_PLATE_X);
        assert!(FOOT_PAD_SPAN_Y > DATUM_PLATE_Y);
        assert!(cable_service_clearance() >= CABLE_SERVICE_CLEARANCE_Y);
    }

    #[test]
    fn output_manifest_exports_parts_plus_assembly() {
        assert_eq!(OUTPUTS.len(), 11);
        assert!(OUTPUTS[0].ends_with("_deck.stl"));
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }
}
