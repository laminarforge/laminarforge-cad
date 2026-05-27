use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
use vcad::{centered_cube, centered_cylinder, Part};

// No-cell flow/pressure validation fixture for the 4x5 tissue-chip cassette.
//
// Purpose:
// - Validate row balance, pressure sensor mapping, leak capture, bubble rejection,
//   and restriction/occlusion response before live-cell use.
// - Provide repeatable dummy-chip restriction coupons and known bypass/occlusion
//   features for media-equivalent water runs.
// - Keep calibration loads on a dedicated fixture instead of burning culture
//   cassettes for every pump/manifold revision.
//
// This is a mechanical validation fixture, not a biological chip.
//
// Exports:
//   output/flow_pressure_validation_fixture_baseplate.stl
//   output/flow_pressure_validation_fixture_restrictor_coupon_carrier.stl
//   output/flow_pressure_validation_fixture_row_manifold_tree.stl
//   output/flow_pressure_validation_fixture_bubble_challenge_insert.stl
//   output/flow_pressure_validation_fixture_pressure_sensor_bar.stl
//   output/flow_pressure_validation_fixture_leak_witness_tray.stl
//   output/flow_pressure_validation_fixture_assembly.stl

const COLS: usize = 4;
const ROWS: usize = 5;
const LANES: usize = COLS * ROWS;
const GUTTER: f64 = 5.0;

const ARRAY_X: f64 = COLS as f64 * REVC_CHIP_LENGTH + (COLS as f64 - 1.0) * GUTTER;
const ARRAY_Y: f64 = ROWS as f64 * REVC_CHIP_WIDTH + (ROWS as f64 - 1.0) * GUTTER;

const BASE_X: f64 = 900.0;
const BASE_Y: f64 = 720.0;
const BASE_Z: f64 = 18.0;

const CARRIER_X: f64 = ARRAY_X + 150.0;
const CARRIER_Y: f64 = ARRAY_Y + 130.0;
const CARRIER_Z: f64 = 24.0;

const COUPON_X: f64 = REVC_CHIP_LENGTH + 4.0;
const COUPON_Y: f64 = REVC_CHIP_WIDTH + 4.0;
const COUPON_Z: f64 = 10.0;

const ROW_PITCH: f64 = REVC_CHIP_WIDTH + GUTTER;
const COL_PITCH: f64 = REVC_CHIP_LENGTH + GUTTER;
const TUBE_OD: f64 = 4.8;
const TUBE_CLEARANCE: f64 = 0.7;
const BORE_D: f64 = TUBE_OD + TUBE_CLEARANCE;

fn main() {
    fs::create_dir_all("output").unwrap();

    let base = baseplate();
    export(
        "output/flow_pressure_validation_fixture_baseplate.stl",
        &base,
    );

    let carrier = restrictor_coupon_carrier();
    export(
        "output/flow_pressure_validation_fixture_restrictor_coupon_carrier.stl",
        &carrier,
    );

    let manifold = row_manifold_tree();
    export(
        "output/flow_pressure_validation_fixture_row_manifold_tree.stl",
        &manifold,
    );

    let bubble = bubble_challenge_insert();
    export(
        "output/flow_pressure_validation_fixture_bubble_challenge_insert.stl",
        &bubble,
    );

    let pressure = pressure_sensor_bar();
    export(
        "output/flow_pressure_validation_fixture_pressure_sensor_bar.stl",
        &pressure,
    );

    let leak = leak_witness_tray();
    export(
        "output/flow_pressure_validation_fixture_leak_witness_tray.stl",
        &leak,
    );

    let assembly = base
        + leak.translate(0.0, 0.0, BASE_Z / 2.0 + 8.0)
        + carrier.translate(92.0, 8.0, BASE_Z / 2.0 + 34.0)
        + manifold.translate(-315.0, 18.0, BASE_Z / 2.0 + 56.0)
        + bubble.translate(-330.0, -248.0, BASE_Z / 2.0 + 62.0)
        + pressure.translate(112.0, -(BASE_Y / 2.0 - 84.0), BASE_Z / 2.0 + 68.0);

    export(
        "output/flow_pressure_validation_fixture_assembly.stl",
        &assembly,
    );

    println!(
        "Flow/pressure validation fixture: {:.0}mm x {:.0}mm base, {} dummy-chip coupons in a {}x{} cassette grid, five row manifolds, pressure sensor bar, bubble challenge insert, and leak witness tray.",
        BASE_X, BASE_Y, LANES, COLS, ROWS
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn baseplate() -> Part {
    let deck = centered_cube("flow_pressure_validation_baseplate", BASE_X, BASE_Y, BASE_Z);
    let pocket = centered_cube(
        "flow_pressure_validation_baseplate_recess",
        BASE_X - 96.0,
        BASE_Y - 90.0,
        8.0,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0 - 3.0);
    let drain = centered_cylinder("flow_pressure_validation_base_drain", 10.0 / 2.0, 38.0, 28)
        .rotate(90.0, 0.0, 0.0)
        .translate(BASE_X / 2.0 - 78.0, -BASE_Y / 2.0 - 2.0, -1.0);

    deck - pocket - drain + perimeter_rails() + mounting_slots() + robot_fiducials()
}

fn perimeter_rails() -> Part {
    let left = centered_cube(
        "flow_pressure_validation_left_rail",
        18.0,
        BASE_Y - 70.0,
        36.0,
    )
    .translate(-(BASE_X / 2.0 - 34.0), 0.0, BASE_Z / 2.0 + 18.0);
    let right = centered_cube(
        "flow_pressure_validation_right_rail",
        18.0,
        BASE_Y - 70.0,
        36.0,
    )
    .translate(BASE_X / 2.0 - 34.0, 0.0, BASE_Z / 2.0 + 18.0);
    let rear = centered_cube(
        "flow_pressure_validation_rear_rail",
        BASE_X - 70.0,
        18.0,
        36.0,
    )
    .translate(0.0, BASE_Y / 2.0 - 34.0, BASE_Z / 2.0 + 18.0);
    let front = centered_cube(
        "flow_pressure_validation_front_low_lip",
        BASE_X - 110.0,
        12.0,
        20.0,
    )
    .translate(0.0, -BASE_Y / 2.0 + 30.0, BASE_Z / 2.0 + 10.0);

    left + right + rear + front
}

fn mounting_slots() -> Part {
    let mut slots = Part::empty("flow_pressure_validation_mounting_slots");
    for (i, (x, y)) in [
        (-(BASE_X / 2.0 - 62.0), -(BASE_Y / 2.0 - 58.0)),
        (BASE_X / 2.0 - 62.0, -(BASE_Y / 2.0 - 58.0)),
        (-(BASE_X / 2.0 - 62.0), BASE_Y / 2.0 - 58.0),
        (BASE_X / 2.0 - 62.0, BASE_Y / 2.0 - 58.0),
        (0.0, -(BASE_Y / 2.0 - 58.0)),
        (0.0, BASE_Y / 2.0 - 58.0),
    ]
    .iter()
    .enumerate()
    {
        slots = slots
            + centered_cylinder(
                format!("flow_pressure_validation_m6_hole_{i}"),
                6.6 / 2.0,
                BASE_Z + 4.0,
                24,
            )
            .translate(*x, *y, 0.0)
            + centered_cube(
                format!("flow_pressure_validation_m6_slot_{i}"),
                22.0,
                6.8,
                BASE_Z + 4.0,
            )
            .translate(*x, *y, 0.0);
    }
    slots
}

fn robot_fiducials() -> Part {
    let mut fiducials = Part::empty("flow_pressure_validation_robot_fiducials");
    for (i, (x, y)) in [
        (-(BASE_X / 2.0 - 70.0), BASE_Y / 2.0 - 72.0),
        (BASE_X / 2.0 - 70.0, BASE_Y / 2.0 - 72.0),
        (-(BASE_X / 2.0 - 70.0), -(BASE_Y / 2.0 - 72.0)),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + fiducial_disc(&format!("flow_pressure_validation_fiducial_{i}")).translate(
                *x,
                *y,
                BASE_Z / 2.0 + 3.0,
            );
    }
    fiducials
}

fn restrictor_coupon_carrier() -> Part {
    let tray = centered_cube(
        "flow_pressure_restrictor_coupon_carrier_tray",
        CARRIER_X,
        CARRIER_Y,
        CARRIER_Z,
    );
    let window = centered_cube(
        "flow_pressure_restrictor_carrier_open_window",
        ARRAY_X + 22.0,
        ARRAY_Y + 22.0,
        CARRIER_Z + 4.0,
    );

    let mut coupons = Part::empty("flow_pressure_validation_dummy_coupon_array");
    let mut recesses = Part::empty("flow_pressure_validation_coupon_recesses");
    for row in 0..ROWS {
        for col in 0..COLS {
            let idx = row * COLS + col;
            let (x, y) = chip_center(col, row);
            recesses = recesses
                + centered_cube(
                    format!("flow_pressure_coupon_recess_{idx}"),
                    COUPON_X + 2.0,
                    COUPON_Y + 2.0,
                    CARRIER_Z + 4.0,
                )
                .translate(x, y, 0.0);
            coupons = coupons
                + dummy_restrictor_coupon(idx).translate(x, y, CARRIER_Z / 2.0 + COUPON_Z / 2.0);
        }
    }

    let carrier = tray - window - recesses + datum_rails() + row_label_tabs();
    carrier + coupons
}

fn dummy_restrictor_coupon(index: usize) -> Part {
    let body = centered_cube(
        format!("flow_pressure_dummy_chip_restrictor_coupon_{index}"),
        COUPON_X,
        COUPON_Y,
        COUPON_Z,
    );
    let inlet = centered_cylinder(
        format!("flow_pressure_dummy_coupon_{index}_inlet"),
        BORE_D / 2.0,
        COUPON_X + 6.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -COUPON_Y / 4.0, 0.0);
    let outlet = centered_cylinder(
        format!("flow_pressure_dummy_coupon_{index}_outlet"),
        BORE_D / 2.0,
        COUPON_X + 6.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, COUPON_Y / 4.0, 0.0);
    let restrictor = centered_cube(
        format!("flow_pressure_dummy_coupon_{index}_known_restriction_slot"),
        COUPON_X - 32.0,
        2.2,
        COUPON_Z + 2.0,
    );
    let bubble_window = centered_cube(
        format!("flow_pressure_dummy_coupon_{index}_bubble_observation_window"),
        34.0,
        18.0,
        COUPON_Z + 2.0,
    )
    .translate(COUPON_X / 2.0 - 32.0, 0.0, 0.0);

    body - inlet - outlet - restrictor - bubble_window
}

fn datum_rails() -> Part {
    let back = centered_cube(
        "flow_pressure_coupon_carrier_back_datum",
        ARRAY_X + 64.0,
        14.0,
        22.0,
    )
    .translate(0.0, ARRAY_Y / 2.0 + 32.0, CARRIER_Z / 2.0 + 11.0);
    let left = centered_cube(
        "flow_pressure_coupon_carrier_left_datum",
        14.0,
        ARRAY_Y + 64.0,
        22.0,
    )
    .translate(-(ARRAY_X / 2.0 + 32.0), 0.0, CARRIER_Z / 2.0 + 11.0);
    let right = centered_cube(
        "flow_pressure_coupon_carrier_right_spring_datum",
        14.0,
        ARRAY_Y + 64.0,
        16.0,
    )
    .translate(ARRAY_X / 2.0 + 32.0, 0.0, CARRIER_Z / 2.0 + 8.0);
    back + left + right
}

fn row_label_tabs() -> Part {
    let mut tabs = Part::empty("flow_pressure_validation_row_label_tabs");
    for row in 0..ROWS {
        let y = row_center(row);
        tabs = tabs
            + centered_cube(
                format!("flow_pressure_validation_row_{row}_label_land"),
                56.0,
                18.0,
                6.0,
            )
            .translate(CARRIER_X / 2.0 - 40.0, y, CARRIER_Z / 2.0 + 3.0);
    }
    tabs
}

fn row_manifold_tree() -> Part {
    let body = centered_cube(
        "flow_pressure_validation_row_manifold_body",
        190.0,
        ARRAY_Y + 130.0,
        52.0,
    );
    let common_header = centered_cylinder(
        "flow_pressure_validation_common_supply_header",
        8.0 / 2.0,
        ARRAY_Y + 90.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0);
    let waste_header = centered_cylinder(
        "flow_pressure_validation_common_return_header",
        8.0 / 2.0,
        ARRAY_Y + 90.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(52.0, 0.0, -15.0);

    let mut cuts = Part::empty("flow_pressure_validation_row_manifold_cuts");
    for row in 0..ROWS {
        let y = row_center(row);
        cuts = cuts
            + centered_cylinder(
                format!("flow_pressure_validation_row_{row}_supply_port"),
                BORE_D / 2.0,
                210.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, y, 8.0)
            + centered_cylinder(
                format!("flow_pressure_validation_row_{row}_return_port"),
                BORE_D / 2.0,
                210.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, y, -15.0);
    }

    body - common_header - waste_header - cuts + row_shutoff_placeholders() + manifold_mount_tabs()
}

fn row_shutoff_placeholders() -> Part {
    let mut valves = Part::empty("flow_pressure_validation_row_shutoff_placeholders");
    for row in 0..ROWS {
        let y = row_center(row);
        valves = valves
            + centered_cube(
                format!("flow_pressure_validation_row_{row}_pinch_valve_placeholder"),
                42.0,
                28.0,
                28.0,
            )
            .translate(-38.0, y, 34.0);
    }
    valves
}

fn manifold_mount_tabs() -> Part {
    let top = centered_cube(
        "flow_pressure_validation_manifold_top_mount_tab",
        210.0,
        34.0,
        10.0,
    )
    .translate(0.0, ARRAY_Y / 2.0 + 80.0, -31.0);
    let bottom = centered_cube(
        "flow_pressure_validation_manifold_bottom_mount_tab",
        210.0,
        34.0,
        10.0,
    )
    .translate(0.0, -(ARRAY_Y / 2.0 + 80.0), -31.0);
    top + bottom
}

fn bubble_challenge_insert() -> Part {
    let block = centered_cube(
        "flow_pressure_validation_bubble_challenge_insert_body",
        250.0,
        95.0,
        70.0,
    );
    let chamber = centered_cylinder(
        "flow_pressure_validation_bubble_challenge_observation_chamber",
        28.0,
        258.0,
        48,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, 6.0);
    let inlet = centered_cylinder(
        "flow_pressure_validation_bubble_challenge_inlet",
        BORE_D / 2.0,
        102.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-82.0, 0.0, -18.0);
    let outlet = centered_cylinder(
        "flow_pressure_validation_bubble_challenge_outlet",
        BORE_D / 2.0,
        102.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(82.0, 0.0, -18.0);
    let high_point_vent = centered_cylinder(
        "flow_pressure_validation_bubble_challenge_high_point_vent",
        3.0 / 2.0,
        80.0,
        18,
    )
    .translate(0.0, 0.0, 38.0);
    let optical_slot = centered_cube(
        "flow_pressure_validation_bubble_challenge_optical_fork_slot",
        82.0,
        110.0,
        18.0,
    )
    .translate(0.0, 0.0, 2.0);

    block - chamber - inlet - outlet - high_point_vent - optical_slot
        + bubble_metering_port_markers()
}

fn bubble_metering_port_markers() -> Part {
    let small = centered_cylinder("flow_pressure_validation_small_bubble_marker", 6.0, 5.0, 28)
        .translate(-62.0, -53.0, 36.0);
    let medium = centered_cylinder(
        "flow_pressure_validation_medium_bubble_marker",
        10.0,
        5.0,
        32,
    )
    .translate(0.0, -53.0, 36.0);
    let large = centered_cylinder(
        "flow_pressure_validation_large_bubble_marker",
        15.0,
        5.0,
        40,
    )
    .translate(62.0, -53.0, 36.0);
    small + medium + large
}

fn pressure_sensor_bar() -> Part {
    let bar = centered_cube(
        "flow_pressure_validation_pressure_sensor_bar",
        ARRAY_X + 190.0,
        66.0,
        48.0,
    );
    let mut cuts = Part::empty("flow_pressure_validation_pressure_bar_cuts");
    for row in 0..ROWS {
        let x = -((ROWS as f64 - 1.0) * 86.0) / 2.0 + row as f64 * 86.0;
        cuts = cuts
            + centered_cylinder(
                format!("flow_pressure_validation_row_{row}_pressure_tap"),
                4.2 / 2.0,
                72.0,
                22,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, 10.0)
            + centered_cube(
                format!("flow_pressure_validation_row_{row}_sensor_pocket"),
                42.0,
                70.0,
                24.0,
            )
            .translate(x, 0.0, -10.0);
    }
    let reference_port = centered_cylinder(
        "flow_pressure_validation_reference_calibrator_port",
        8.0 / 2.0,
        72.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(ARRAY_X / 2.0 + 65.0, 0.0, 8.0);

    bar - cuts - reference_port + pressure_bar_label_lands()
}

fn pressure_bar_label_lands() -> Part {
    let mut lands = Part::empty("flow_pressure_validation_pressure_label_lands");
    for row in 0..ROWS {
        let x = -((ROWS as f64 - 1.0) * 86.0) / 2.0 + row as f64 * 86.0;
        lands = lands
            + centered_cube(
                format!("flow_pressure_validation_row_{row}_pressure_label"),
                54.0,
                5.0,
                8.0,
            )
            .translate(x, -36.0, 24.0);
    }
    lands
}

fn leak_witness_tray() -> Part {
    let tray = centered_cube(
        "flow_pressure_validation_leak_witness_tray",
        CARRIER_X + 70.0,
        CARRIER_Y + 65.0,
        18.0,
    );
    let basin = centered_cube(
        "flow_pressure_validation_leak_witness_basin",
        CARRIER_X + 20.0,
        CARRIER_Y + 18.0,
        10.0,
    )
    .translate(0.0, 0.0, 5.0);
    let sensor_well = centered_cube(
        "flow_pressure_validation_leak_sensor_well",
        58.0,
        42.0,
        16.0,
    )
    .translate(CARRIER_X / 2.0 - 36.0, -(CARRIER_Y / 2.0 - 40.0), 2.0);
    let drain = centered_cylinder(
        "flow_pressure_validation_leak_witness_drain",
        9.0 / 2.0,
        36.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(CARRIER_X / 2.0 - 35.0, -(CARRIER_Y / 2.0 + 35.0), 0.0);

    tray - basin - sensor_well - drain + tray_handles()
}

fn tray_handles() -> Part {
    let left = centered_cube(
        "flow_pressure_validation_left_tray_handle",
        42.0,
        20.0,
        26.0,
    )
    .translate(-(CARRIER_X / 2.0 + 52.0), 0.0, 8.0);
    let right = centered_cube(
        "flow_pressure_validation_right_tray_handle",
        42.0,
        20.0,
        26.0,
    )
    .translate(CARRIER_X / 2.0 + 52.0, 0.0, 8.0);
    left + right
}

fn chip_center(col: usize, row: usize) -> (f64, f64) {
    (
        -ARRAY_X / 2.0 + REVC_CHIP_LENGTH / 2.0 + col as f64 * COL_PITCH,
        row_center(row),
    )
}

fn row_center(row: usize) -> f64 {
    -ARRAY_Y / 2.0 + REVC_CHIP_WIDTH / 2.0 + row as f64 * ROW_PITCH
}

fn fiducial_disc(name: &str) -> Part {
    let disk = centered_cylinder(format!("{name}_disk"), 9.0, 3.0, 40);
    let center = centered_cylinder(format!("{name}_center_bore"), 2.0, 4.0, 20);
    disk - center
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixture_covers_full_twenty_chip_grid() {
        assert_eq!(LANES, 20);
        assert!(CARRIER_X > ARRAY_X + 100.0);
        assert!(CARRIER_Y > ARRAY_Y + 100.0);
    }

    #[test]
    fn row_manifold_matches_cassette_rows() {
        assert_eq!(ROWS, 5);
        assert!((row_center(0) + row_center(ROWS - 1)).abs() < 0.001);
        assert!(ROW_PITCH > REVC_CHIP_WIDTH);
    }

    #[test]
    fn coupon_geometry_stays_inside_validation_carrier() {
        assert!(COUPON_X > REVC_CHIP_LENGTH);
        assert!(COUPON_Y > REVC_CHIP_WIDTH);
        assert!(COUPON_Z < CARRIER_Z);
    }

    #[test]
    fn fixture_fits_existing_bench_scale() {
        assert!(BASE_X <= 950.0);
        assert!(BASE_Y <= 760.0);
        assert!(ARRAY_X < BASE_X - 260.0);
    }
}
