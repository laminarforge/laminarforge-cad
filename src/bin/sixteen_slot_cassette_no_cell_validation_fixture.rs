use std::fs;
use std::path::Path;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
use vcad::{centered_cube, centered_cylinder, Part};

// A10 first-pass CAD for the 16-slot cassette no-cell validation fixture.
//
// This is a dry bench fixture package for A7 gates: fit/datum, harness map,
// prime visibility, bubble challenge, pressure/leak witness, flow collection,
// restriction detection, dye-recovery bookkeeping, waste/backflow checks, and
// run-record traceability. It deliberately makes no sterile, live-cell, AAV,
// clinical, containment, or biological-release claim.

const OUTPUT_DIR: &str = "output/no_cell_fixture";
const OUTPUTS: [&str; 9] = [
    "output/no_cell_fixture/sixteen_slot_no_cell_fixture_cassette_nest.stl",
    "output/no_cell_fixture/sixteen_slot_no_cell_fixture_surrogate_chip_set.stl",
    "output/no_cell_fixture/sixteen_slot_no_cell_fixture_pressure_sensor_bar.stl",
    "output/no_cell_fixture/sixteen_slot_no_cell_fixture_flow_collection_deck.stl",
    "output/no_cell_fixture/sixteen_slot_no_cell_fixture_bubble_challenge_station.stl",
    "output/no_cell_fixture/sixteen_slot_no_cell_fixture_leak_witness_tray.stl",
    "output/no_cell_fixture/sixteen_slot_no_cell_fixture_waste_backflow_station.stl",
    "output/no_cell_fixture/sixteen_slot_no_cell_fixture_run_record_plate.stl",
    "output/no_cell_fixture/sixteen_slot_no_cell_validation_fixture_assembly.stl",
];

const COLS: usize = 4;
const ROWS: usize = 4;
const SLOT_COUNT: usize = COLS * ROWS;

const CHIP_GUTTER_X: f64 = 7.0;
const CHIP_GUTTER_Y: f64 = 7.0;
const SLOT_PITCH_X: f64 = REVC_CHIP_LENGTH + CHIP_GUTTER_X;
const SLOT_PITCH_Y: f64 = REVC_CHIP_WIDTH + CHIP_GUTTER_Y;
const SLOT_ARRAY_X: f64 = COLS as f64 * REVC_CHIP_LENGTH + (COLS as f64 - 1.0) * CHIP_GUTTER_X;
const SLOT_ARRAY_Y: f64 = ROWS as f64 * REVC_CHIP_WIDTH + (ROWS as f64 - 1.0) * CHIP_GUTTER_Y;

const CARRIER_MARGIN_X: f64 = 58.0;
const CARRIER_MARGIN_Y: f64 = 52.0;
const CARRIER_X: f64 = SLOT_ARRAY_X + CARRIER_MARGIN_X * 2.0;
const CARRIER_Y: f64 = SLOT_ARRAY_Y + CARRIER_MARGIN_Y * 2.0;
const CARRIER_Z: f64 = 24.0;
const LID_Z: f64 = 10.0;
const CASSETTE_STACK_Z: f64 = CARRIER_Z + LID_Z + 5.0;

const DECK_X: f64 = CARRIER_X + 360.0;
const DECK_Y: f64 = CARRIER_Y + 430.0;
const DECK_Z: f64 = 18.0;
const NEST_RAIL_W: f64 = 16.0;
const NEST_RAIL_Z: f64 = 18.0;
const FRONT_RETENTION_LIP_Z: f64 = 8.0;
const COLLECTION_VIAL_DIA: f64 = 18.0;
const COLLECTION_VIAL_DEPTH: f64 = 12.0;
const SENSOR_POCKET_DIA: f64 = 12.0;
const SENSOR_POCKET_DEPTH: f64 = 5.0;
const RUN_RECORD_BARCODE_LANDS: usize = 6;
const PRESSURE_SENSOR_COUNT: usize = 6;
const RESTRICTION_TYPES: usize = 5;
const WASTE_LEVELS: usize = 3;
const BUBBLE_WINDOWS: usize = 5;

fn main() {
    assert_fixture_contract();
    fs::create_dir_all(OUTPUT_DIR).expect("failed to create output/no_cell_fixture");

    let nest = cassette_nest();
    export(OUTPUTS[0], &nest);

    let surrogate = surrogate_chip_set();
    export(OUTPUTS[1], &surrogate);

    let pressure = pressure_sensor_bar();
    export(OUTPUTS[2], &pressure);

    let collection = flow_collection_deck();
    export(OUTPUTS[3], &collection);

    let bubble = bubble_challenge_station();
    export(OUTPUTS[4], &bubble);

    let leak = leak_witness_tray();
    export(OUTPUTS[5], &leak);

    let waste = waste_backflow_station();
    export(OUTPUTS[6], &waste);

    let record = run_record_plate();
    export(OUTPUTS[7], &record);

    let assembly = fixture_assembly(
        nest, surrogate, pressure, collection, bubble, leak, waste, record,
    );
    export(OUTPUTS[8], &assembly);

    for path in OUTPUTS {
        assert!(
            Path::new(path).exists(),
            "no-cell fixture export did not create required output: {path}"
        );
    }

    println!();
    println!("16-slot cassette no-cell validation fixture:");
    println!("  Export directory:       {OUTPUT_DIR}");
    println!("  Output count:           {}", OUTPUTS.len());
    println!("  Slot map:               {COLS} x {ROWS} ({SLOT_COUNT} slots)");
    println!("  Deck envelope:          {DECK_X:.1} x {DECK_Y:.1} x {DECK_Z:.1} mm");
    println!("  Cassette envelope ref:  {CARRIER_X:.1} x {CARRIER_Y:.1} x {CASSETTE_STACK_Z:.1} mm stack");
    println!("  Pressure points:        upstream, R1-R4 rows, and waste/backpressure");
    println!("  Collection positions:   {SLOT_COUNT} labeled S01-S16 vial nests");
    println!("  Restriction coupons:    nominal, low, high, blocked, bypass");
    println!("  Waste levels:           {WASTE_LEVELS} high/level/low head witness positions");
    println!("  Run record lands:       {RUN_RECORD_BARCODE_LANDS} barcode/status lands");
    println!(
        "  Status:                 dry no-cell fixture only; blocks cells/AAV until A7 gates pass"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap_or_else(|error| {
        panic!("failed to write no-cell fixture STL {path}: {error:?}");
    });
    println!("Exported: {path}");
}

fn cassette_nest() -> Part {
    let deck = centered_cube(
        "sixteen_slot_no_cell_fixture_cassette_nest_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    );
    let cassette_shadow = centered_cube(
        "sixteen_slot_no_cell_fixture_cassette_shadow_relief",
        CARRIER_X + 12.0,
        CARRIER_Y + 12.0,
        DECK_Z + 2.0,
    )
    .translate(0.0, 0.0, 2.0);

    deck - cassette_shadow - fixture_mount_holes("cassette_nest", DECK_X, DECK_Y, DECK_Z + 2.0)
        + nest_reference_rails()
        + cassette_outline_tokens()
        + slot_map_tokens()
        + harness_keepout_bridges()
        + drain_visibility_bridge()
}

fn surrogate_chip_set() -> Part {
    let tray_x = SLOT_ARRAY_X + 96.0;
    let tray_y = SLOT_ARRAY_Y + 122.0;
    let tray_z = 10.0;
    let tray = centered_cube(
        "sixteen_slot_surrogate_chip_set_tray",
        tray_x,
        tray_y,
        tray_z,
    );
    let mut coupons = Part::empty("sixteen_slot_surrogate_chip_set_coupons");
    let mut pockets = Part::empty("sixteen_slot_surrogate_chip_set_storage_pockets");

    for row in 0..ROWS {
        for col in 0..COLS {
            let slot = slot_number(row, col);
            let (x, y) = slot_center(row, col);
            let coupon = surrogate_chip_coupon(slot).translate(x, y, tray_z / 2.0 + 2.0);
            let pocket = centered_cube(
                format!("sixteen_slot_surrogate_chip_set_slot_{slot:02}_pocket"),
                REVC_CHIP_LENGTH + 3.0,
                REVC_CHIP_WIDTH + 3.0,
                tray_z + 2.0,
            )
            .translate(x, y, 1.0);
            coupons = coupons + coupon;
            pockets = pockets + pocket;
        }
    }

    tray - pockets + coupons + restriction_coupon_lane()
}

fn pressure_sensor_bar() -> Part {
    let bar_x = CARRIER_X + 132.0;
    let bar_y = 64.0;
    let bar_z = 22.0;
    let body = centered_cube("sixteen_slot_pressure_sensor_bar_body", bar_x, bar_y, bar_z);
    let mut sensor_cuts = Part::empty("sixteen_slot_pressure_sensor_bar_pocket_cuts");
    let mut label_lands = Part::empty("sixteen_slot_pressure_sensor_bar_label_lands");

    for i in 0..PRESSURE_SENSOR_COUNT {
        let x = centered_index(i, PRESSURE_SENSOR_COUNT, 92.0);
        sensor_cuts = sensor_cuts
            + centered_cylinder(
                format!("sixteen_slot_pressure_sensor_bar_sensor_{i}_pocket"),
                SENSOR_POCKET_DIA / 2.0,
                SENSOR_POCKET_DEPTH + 1.0,
                32,
            )
            .translate(x, 0.0, bar_z / 2.0 - SENSOR_POCKET_DEPTH / 2.0 + 0.2);
        label_lands = label_lands
            + centered_cube(
                format!("sixteen_slot_pressure_sensor_bar_sensor_{i}_label_land"),
                58.0,
                14.0,
                2.0,
            )
            .translate(x, -bar_y / 2.0 + 9.0, bar_z / 2.0 + 1.0);
    }

    body - sensor_cuts - row_tubing_reliefs(bar_x, bar_y, bar_z)
        + label_lands
        + pressure_port_guard_rails(bar_x)
}

fn flow_collection_deck() -> Part {
    let deck_x = SLOT_ARRAY_X + 126.0;
    let deck_y = SLOT_ARRAY_Y + 128.0;
    let deck_z = 16.0;
    let deck = centered_cube(
        "sixteen_slot_flow_collection_deck_body",
        deck_x,
        deck_y,
        deck_z,
    );
    let mut vial_cuts = Part::empty("sixteen_slot_flow_collection_vial_nest_cuts");
    let mut labels = Part::empty("sixteen_slot_flow_collection_slot_label_lands");
    let mut row_gutters = Part::empty("sixteen_slot_flow_collection_row_gutters");

    for row in 0..ROWS {
        let row_y = centered_index(row, ROWS, SLOT_PITCH_Y);
        row_gutters = row_gutters
            + centered_cube(
                format!("sixteen_slot_flow_collection_row_{}_spill_gutter", row + 1),
                deck_x - 48.0,
                8.0,
                deck_z + 2.0,
            )
            .translate(0.0, row_y - REVC_CHIP_WIDTH / 2.0 - 13.0, 1.0);
        for col in 0..COLS {
            let slot = slot_number(row, col);
            let (x, y) = slot_center(row, col);
            vial_cuts = vial_cuts
                + centered_cylinder(
                    format!("sixteen_slot_flow_collection_slot_{slot:02}_vial_nest"),
                    COLLECTION_VIAL_DIA / 2.0,
                    COLLECTION_VIAL_DEPTH + 1.0,
                    36,
                )
                .translate(x, y, deck_z / 2.0 - COLLECTION_VIAL_DEPTH / 2.0 + 0.2);
            labels = labels
                + centered_cube(
                    format!("sixteen_slot_flow_collection_slot_{slot:02}_label_land"),
                    32.0,
                    12.0,
                    2.0,
                )
                .translate(x, y + 22.0, deck_z / 2.0 + 1.0);
        }
    }

    deck - vial_cuts - row_gutters + labels + scale_datum_pads(deck_x, deck_y, deck_z)
}

fn bubble_challenge_station() -> Part {
    let base_x = 286.0;
    let base_y = 132.0;
    let base_z = 16.0;
    let base = centered_cube(
        "sixteen_slot_bubble_challenge_station_base",
        base_x,
        base_y,
        base_z,
    );
    let challenge_port = centered_cylinder(
        "sixteen_slot_bubble_challenge_known_slug_inlet",
        5.0,
        28.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-base_x / 2.0 + 36.0, 0.0, base_z / 2.0 + 20.0);
    let bypass_waste_port = centered_cylinder(
        "sixteen_slot_bubble_challenge_w1_w3_waste_outlet",
        5.0,
        28.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(base_x / 2.0 - 36.0, 0.0, base_z / 2.0 + 20.0);

    base + challenge_port
        + bypass_waste_port
        + bubble_window_ladder(base_z)
        + bubble_route_token_lands(base_z)
}

fn leak_witness_tray() -> Part {
    let tray_x = CARRIER_X + 84.0;
    let tray_y = 94.0;
    let tray_z = 12.0;
    let tray = centered_cube(
        "sixteen_slot_leak_witness_tray_body",
        tray_x,
        tray_y,
        tray_z,
    );
    let gutter = rectangular_frame(
        "sixteen_slot_leak_witness_tray_visible_dye_moat",
        tray_x - 28.0,
        tray_y - 24.0,
        tray_z + 2.0,
        8.0,
    );
    let wick_paper_recess = centered_cube(
        "sixteen_slot_leak_witness_wicking_paper_recess",
        tray_x - 84.0,
        tray_y - 48.0,
        tray_z + 2.0,
    )
    .translate(0.0, 0.0, 2.0);
    let photo_scale_land = centered_cube(
        "sixteen_slot_leak_witness_photo_scale_land",
        120.0,
        16.0,
        2.0,
    )
    .translate(0.0, -tray_y / 2.0 + 14.0, tray_z / 2.0 + 1.0);

    tray - gutter - wick_paper_recess + photo_scale_land + leak_status_token_pockets(tray_x, tray_z)
}

fn waste_backflow_station() -> Part {
    let base_x = 302.0;
    let base_y = 176.0;
    let base_z = 16.0;
    let base = centered_cube(
        "sixteen_slot_waste_backflow_station_base",
        base_x,
        base_y,
        base_z,
    );
    let mut stands = Part::empty("sixteen_slot_waste_backflow_head_height_stands");
    let mut labels = Part::empty("sixteen_slot_waste_backflow_status_label_lands");

    for i in 0..WASTE_LEVELS {
        let x = centered_index(i, WASTE_LEVELS, 92.0);
        let stand_h = 34.0 + i as f64 * 24.0;
        stands = stands
            + centered_cube(
                format!("sixteen_slot_waste_backflow_level_{i}_waste_bottle_stand"),
                64.0,
                74.0,
                stand_h,
            )
            .translate(x, 0.0, base_z / 2.0 + stand_h / 2.0);
        labels = labels
            + centered_cube(
                format!("sixteen_slot_waste_backflow_level_{i}_label_land"),
                58.0,
                14.0,
                2.0,
            )
            .translate(x, -base_y / 2.0 + 14.0, base_z / 2.0 + 1.0);
    }

    base + stands
        + labels
        + waste_check_valve_witness(base_z)
        + overflow_capture_lip(base_x, base_y, base_z)
}

fn run_record_plate() -> Part {
    let plate_x = 430.0;
    let plate_y = 154.0;
    let plate_z = 8.0;
    let plate = centered_cube(
        "sixteen_slot_run_record_plate_body",
        plate_x,
        plate_y,
        plate_z,
    );
    let mut lands = Part::empty("sixteen_slot_run_record_barcode_status_lands");

    for i in 0..RUN_RECORD_BARCODE_LANDS {
        let row = i / 3;
        let col = i % 3;
        lands = lands
            + centered_cube(
                format!("sixteen_slot_run_record_barcode_land_{i}"),
                104.0,
                26.0,
                2.0,
            )
            .translate(
                centered_index(col, 3, 132.0),
                centered_index(row, 2, 58.0),
                plate_z / 2.0 + 1.0,
            );
    }

    plate
        + lands
        + pass_fail_token_pockets(plate_z)
        + photo_evidence_ruler(plate_x, plate_y, plate_z)
}

fn fixture_assembly(
    nest: Part,
    surrogate: Part,
    pressure: Part,
    collection: Part,
    bubble: Part,
    leak: Part,
    waste: Part,
    record: Part,
) -> Part {
    nest + surrogate.translate(-(DECK_X / 2.0 + SLOT_ARRAY_X / 2.0 + 90.0), -130.0, 18.0)
        + pressure.translate(0.0, DECK_Y / 2.0 + 70.0, 16.0)
        + collection.translate(DECK_X / 2.0 + SLOT_ARRAY_X / 2.0 + 98.0, -112.0, 14.0)
        + bubble.translate(-(DECK_X / 2.0 + 176.0), DECK_Y / 2.0 + 208.0, 12.0)
        + leak.translate(0.0, -(DECK_Y / 2.0 + 74.0), 10.0)
        + waste.translate(DECK_X / 2.0 + 216.0, DECK_Y / 2.0 + 188.0, 10.0)
        + record.translate(0.0, -(DECK_Y / 2.0 + 184.0), 9.0)
}

fn nest_reference_rails() -> Part {
    let rear = centered_cube(
        "sixteen_slot_no_cell_fixture_rear_datum_rail",
        CARRIER_X + 30.0,
        NEST_RAIL_W,
        NEST_RAIL_Z,
    )
    .translate(
        0.0,
        CARRIER_Y / 2.0 + NEST_RAIL_W / 2.0 + 8.0,
        DECK_Z / 2.0 + NEST_RAIL_Z / 2.0,
    );
    let left = centered_cube(
        "sixteen_slot_no_cell_fixture_left_datum_rail",
        NEST_RAIL_W,
        CARRIER_Y + 30.0,
        NEST_RAIL_Z,
    )
    .translate(
        -(CARRIER_X / 2.0 + NEST_RAIL_W / 2.0 + 8.0),
        0.0,
        DECK_Z / 2.0 + NEST_RAIL_Z / 2.0,
    );
    let front_lip = centered_cube(
        "sixteen_slot_no_cell_fixture_front_low_retention_lip",
        CARRIER_X + 30.0,
        10.0,
        FRONT_RETENTION_LIP_Z,
    )
    .translate(
        0.0,
        -(CARRIER_Y / 2.0 + 20.0),
        DECK_Z / 2.0 + FRONT_RETENTION_LIP_Z / 2.0,
    );
    rear + left + front_lip
}

fn cassette_outline_tokens() -> Part {
    let front = centered_cube(
        "sixteen_slot_no_cell_fixture_cassette_front_outline_token",
        CARRIER_X,
        5.0,
        3.0,
    )
    .translate(0.0, -CARRIER_Y / 2.0, DECK_Z / 2.0 + 1.5);
    let rear = centered_cube(
        "sixteen_slot_no_cell_fixture_cassette_rear_outline_token",
        CARRIER_X,
        5.0,
        3.0,
    )
    .translate(0.0, CARRIER_Y / 2.0, DECK_Z / 2.0 + 1.5);
    let left = centered_cube(
        "sixteen_slot_no_cell_fixture_cassette_left_outline_token",
        5.0,
        CARRIER_Y,
        3.0,
    )
    .translate(-CARRIER_X / 2.0, 0.0, DECK_Z / 2.0 + 1.5);
    let right = centered_cube(
        "sixteen_slot_no_cell_fixture_cassette_right_outline_token",
        5.0,
        CARRIER_Y,
        3.0,
    )
    .translate(CARRIER_X / 2.0, 0.0, DECK_Z / 2.0 + 1.5);
    front + rear + left + right
}

fn slot_map_tokens() -> Part {
    let mut tokens = Part::empty("sixteen_slot_no_cell_fixture_s01_s16_slot_tokens");
    for row in 0..ROWS {
        for col in 0..COLS {
            let slot = slot_number(row, col);
            let (x, y) = slot_center(row, col);
            tokens = tokens
                + centered_cube(
                    format!("sixteen_slot_no_cell_fixture_slot_{slot:02}_token_land"),
                    28.0,
                    12.0,
                    2.0,
                )
                .translate(
                    x - REVC_CHIP_LENGTH / 2.0 + 20.0,
                    y + REVC_CHIP_WIDTH / 2.0 - 12.0,
                    DECK_Z / 2.0 + 1.0,
                );
        }
    }
    tokens
}

fn harness_keepout_bridges() -> Part {
    let left = centered_cube(
        "sixteen_slot_no_cell_fixture_left_harness_keepout_bridge",
        20.0,
        SLOT_ARRAY_Y + 36.0,
        8.0,
    )
    .translate(-(CARRIER_X / 2.0 + 48.0), 0.0, DECK_Z / 2.0 + 4.0);
    let right = centered_cube(
        "sixteen_slot_no_cell_fixture_right_harness_keepout_bridge",
        20.0,
        SLOT_ARRAY_Y + 36.0,
        8.0,
    )
    .translate(CARRIER_X / 2.0 + 48.0, 0.0, DECK_Z / 2.0 + 4.0);
    left + right
}

fn drain_visibility_bridge() -> Part {
    centered_cube(
        "sixteen_slot_no_cell_fixture_visible_drain_bridge",
        168.0,
        14.0,
        8.0,
    )
    .translate(
        CARRIER_X / 2.0 - 116.0,
        -CARRIER_Y / 2.0 - 46.0,
        DECK_Z / 2.0 + 4.0,
    )
}

fn surrogate_chip_coupon(slot: usize) -> Part {
    let body = centered_cube(
        format!("sixteen_slot_surrogate_chip_slot_{slot:02}_body"),
        REVC_CHIP_LENGTH,
        REVC_CHIP_WIDTH,
        4.0,
    );
    let inlet_witness = centered_cube(
        format!("sixteen_slot_surrogate_chip_slot_{slot:02}_inlet_witness_window"),
        12.0,
        10.0,
        1.5,
    )
    .translate(-REVC_CHIP_LENGTH / 2.0 + 16.0, 0.0, 2.75);
    let outlet_witness = centered_cube(
        format!("sixteen_slot_surrogate_chip_slot_{slot:02}_outlet_witness_window"),
        12.0,
        10.0,
        1.5,
    )
    .translate(REVC_CHIP_LENGTH / 2.0 - 16.0, 0.0, 2.75);
    let slot_token = centered_cube(
        format!("sixteen_slot_surrogate_chip_slot_{slot:02}_identity_token"),
        24.0,
        8.0,
        1.5,
    )
    .translate(0.0, REVC_CHIP_WIDTH / 2.0 - 9.0, 2.75);
    body + inlet_witness + outlet_witness + slot_token
}

fn restriction_coupon_lane() -> Part {
    let lane_y = -(SLOT_ARRAY_Y / 2.0 + 46.0);
    let mut lane = Part::empty("sixteen_slot_restriction_coupon_ordered_lane");
    for i in 0..RESTRICTION_TYPES {
        let x = centered_index(i, RESTRICTION_TYPES, 58.0);
        let height = 3.0 + i as f64;
        lane = lane
            + centered_cube(
                format!("sixteen_slot_restriction_coupon_type_{i}_surrogate"),
                46.0,
                22.0,
                height,
            )
            .translate(x, lane_y, 10.0 / 2.0 + height / 2.0);
    }
    lane
}

fn row_tubing_reliefs(bar_x: f64, _bar_y: f64, bar_z: f64) -> Part {
    let mut reliefs = Part::empty("sixteen_slot_pressure_sensor_bar_row_tubing_reliefs");
    for row in 0..ROWS {
        reliefs = reliefs
            + centered_cube(
                format!(
                    "sixteen_slot_pressure_sensor_bar_row_{}_tube_relief",
                    row + 1
                ),
                bar_x - 48.0,
                4.0,
                bar_z + 2.0,
            )
            .translate(0.0, centered_index(row, ROWS, 10.0), 0.0);
    }
    reliefs
}

fn pressure_port_guard_rails(bar_x: f64) -> Part {
    let front = centered_cube(
        "sixteen_slot_pressure_sensor_bar_front_guard_rail",
        bar_x - 34.0,
        5.0,
        7.0,
    )
    .translate(0.0, -34.5, 22.0 / 2.0 + 3.5);
    let rear = centered_cube(
        "sixteen_slot_pressure_sensor_bar_rear_guard_rail",
        bar_x - 34.0,
        5.0,
        7.0,
    )
    .translate(0.0, 34.5, 22.0 / 2.0 + 3.5);
    front + rear
}

fn scale_datum_pads(deck_x: f64, deck_y: f64, deck_z: f64) -> Part {
    let mut pads = Part::empty("sixteen_slot_flow_collection_scale_datum_pads");
    for (i, (sx, sy)) in [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)]
        .into_iter()
        .enumerate()
    {
        pads = pads
            + centered_cube(
                format!("sixteen_slot_flow_collection_scale_datum_pad_{i}"),
                42.0,
                24.0,
                2.0,
            )
            .translate(
                sx * (deck_x / 2.0 - 38.0),
                sy * (deck_y / 2.0 - 30.0),
                deck_z / 2.0 + 1.0,
            );
    }
    pads
}

fn bubble_window_ladder(base_z: f64) -> Part {
    let mut windows = Part::empty("sixteen_slot_bubble_challenge_window_ladder");
    for i in 0..BUBBLE_WINDOWS {
        windows = windows
            + centered_cube(
                format!("sixteen_slot_bubble_challenge_window_{i}"),
                38.0,
                12.0,
                6.0,
            )
            .translate(
                centered_index(i, BUBBLE_WINDOWS, 46.0),
                0.0,
                base_z / 2.0 + 10.0,
            );
    }
    windows
}

fn bubble_route_token_lands(base_z: f64) -> Part {
    let mut lands = Part::empty("sixteen_slot_bubble_challenge_route_token_lands");
    for i in 0..4 {
        lands = lands
            + centered_cube(
                format!("sixteen_slot_bubble_challenge_row_{}_route_token", i + 1),
                40.0,
                14.0,
                2.0,
            )
            .translate(centered_index(i, 4, 58.0), 46.0, base_z / 2.0 + 1.0);
    }
    lands
}

fn leak_status_token_pockets(tray_x: f64, tray_z: f64) -> Part {
    let mut tokens = Part::empty("sixteen_slot_leak_witness_status_token_pockets");
    for i in 0..5 {
        tokens = tokens
            + centered_cube(
                format!("sixteen_slot_leak_witness_status_token_{i}"),
                30.0,
                18.0,
                2.0,
            )
            .translate(
                -tray_x / 2.0 + 32.0 + i as f64 * 38.0,
                34.0,
                tray_z / 2.0 + 1.0,
            );
    }
    tokens
}

fn waste_check_valve_witness(base_z: f64) -> Part {
    centered_cube(
        "sixteen_slot_waste_backflow_check_valve_witness_land",
        92.0,
        28.0,
        3.0,
    )
    .translate(0.0, 72.0, base_z / 2.0 + 1.5)
}

fn overflow_capture_lip(base_x: f64, base_y: f64, base_z: f64) -> Part {
    let front = centered_cube(
        "sixteen_slot_waste_backflow_overflow_front_lip",
        base_x - 24.0,
        8.0,
        12.0,
    )
    .translate(0.0, -base_y / 2.0 + 8.0, base_z / 2.0 + 6.0);
    let rear = centered_cube(
        "sixteen_slot_waste_backflow_overflow_rear_lip",
        base_x - 24.0,
        8.0,
        12.0,
    )
    .translate(0.0, base_y / 2.0 - 8.0, base_z / 2.0 + 6.0);
    front + rear
}

fn pass_fail_token_pockets(plate_z: f64) -> Part {
    let mut pockets = Part::empty("sixteen_slot_run_record_pass_fail_token_pockets");
    for i in 0..11 {
        pockets = pockets
            + centered_cube(
                format!("sixteen_slot_run_record_gate_{i:02}_pass_fail_token_land"),
                30.0,
                18.0,
                2.0,
            )
            .translate(centered_index(i, 11, 36.0), -62.0, plate_z / 2.0 + 1.0);
    }
    pockets
}

fn photo_evidence_ruler(plate_x: f64, plate_y: f64, plate_z: f64) -> Part {
    let mut ticks = Part::empty("sixteen_slot_run_record_photo_evidence_ruler");
    for i in 0..17 {
        let h = if i % 4 == 0 { 14.0 } else { 8.0 };
        ticks = ticks
            + centered_cube(
                format!("sixteen_slot_run_record_ruler_tick_{i}"),
                2.0,
                h,
                2.0,
            )
            .translate(
                -plate_x / 2.0 + 26.0 + i as f64 * 22.0,
                plate_y / 2.0 - 18.0,
                plate_z / 2.0 + 1.0,
            );
    }
    ticks
}

fn fixture_mount_holes(prefix: &str, x: f64, y: f64, z: f64) -> Part {
    let mut holes = Part::empty(format!("{prefix}_mount_holes"));
    for (i, (sx, sy)) in [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)]
        .into_iter()
        .enumerate()
    {
        holes = holes
            + centered_cylinder(format!("{prefix}_m6_mount_hole_{i}"), 3.3, z, 28).translate(
                sx * (x / 2.0 - 36.0),
                sy * (y / 2.0 - 36.0),
                0.0,
            );
    }
    holes
}

fn rectangular_frame(name: &str, x: f64, y: f64, z: f64, wall: f64) -> Part {
    let outer = centered_cube(format!("{name}_outer"), x, y, z);
    let inner = centered_cube(
        format!("{name}_inner"),
        x - wall * 2.0,
        y - wall * 2.0,
        z + 2.0,
    );
    outer - inner
}

fn slot_center(row: usize, col: usize) -> (f64, f64) {
    (
        centered_index(col, COLS, SLOT_PITCH_X),
        centered_index(row, ROWS, SLOT_PITCH_Y),
    )
}

fn slot_number(row: usize, col: usize) -> usize {
    row * COLS + col + 1
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn assert_fixture_contract() {
    assert_eq!(SLOT_COUNT, 16);
    assert_eq!(OUTPUTS.len(), 9);
    assert_eq!(PRESSURE_SENSOR_COUNT, 6);
    assert_eq!(RESTRICTION_TYPES, 5);
    assert_eq!(WASTE_LEVELS, 3);
    assert!(DECK_X > CARRIER_X + 300.0);
    assert!(DECK_Y > CARRIER_Y + 360.0);
    assert!(OUTPUTS.iter().all(|path| path.starts_with(OUTPUT_DIR)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixture_outputs_are_stable() {
        assert_eq!(OUTPUTS.len(), 9);
        assert!(OUTPUTS
            .contains(&"output/no_cell_fixture/sixteen_slot_no_cell_fixture_cassette_nest.stl"));
        assert!(OUTPUTS.contains(
            &"output/no_cell_fixture/sixteen_slot_no_cell_fixture_flow_collection_deck.stl"
        ));
        assert!(OUTPUTS.contains(
            &"output/no_cell_fixture/sixteen_slot_no_cell_validation_fixture_assembly.stl"
        ));
    }

    #[test]
    fn fixture_preserves_a7_measurement_contract() {
        assert_eq!(SLOT_COUNT, 16);
        assert_eq!(PRESSURE_SENSOR_COUNT, 6);
        assert_eq!(RESTRICTION_TYPES, 5);
        assert_eq!(BUBBLE_WINDOWS, 5);
        assert_eq!(WASTE_LEVELS, 3);
        assert_eq!(RUN_RECORD_BARCODE_LANDS, 6);
    }

    #[test]
    fn slot_numbering_is_row_major_s01_to_s16() {
        assert_eq!(slot_number(0, 0), 1);
        assert_eq!(slot_number(0, 3), 4);
        assert_eq!(slot_number(3, 0), 13);
        assert_eq!(slot_number(3, 3), 16);
    }
}
