use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
use vcad::{centered_cube, centered_cylinder, Part};

// Disposable tubing harness for the 20-chip automated media exchange cassette.
//
// Intent:
// - Keep culture media in a replaceable sterile fluid path.
// - Present keyed, robot-loadable inserts instead of routine manual pipetting.
// - Align five row trunks and twenty chip branches to the 4x5 cassette layout.
// - Provide visible bubble traps, strain relief, and pump-side service routing.
//
// This is a CAD interface model for automation planning. Material, sterilization,
// connector selection, and biological validation remain separate gates.

const COLS: usize = 4;
const ROWS: usize = 5;
const GUTTER: f64 = 5.0;
const CASSETTE_MARGIN_X: f64 = 28.0;
const CASSETTE_MARGIN_Y: f64 = 28.0;

const TUBE_OD: f64 = 4.8;
const TUBE_CLEARANCE: f64 = 0.5;
const TUBE_CHANNEL_D: f64 = TUBE_OD + TUBE_CLEARANCE;
const ROW_TRUNK_D: f64 = 6.0;
const LATCH_HOLE_D: f64 = 3.2;

const PITCH_X: f64 = REVC_CHIP_LENGTH + GUTTER;
const PITCH_Y: f64 = REVC_CHIP_WIDTH + GUTTER;
const ARRAY_X: f64 = COLS as f64 * REVC_CHIP_LENGTH + (COLS as f64 - 1.0) * GUTTER;
const ARRAY_Y: f64 = ROWS as f64 * REVC_CHIP_WIDTH + (ROWS as f64 - 1.0) * GUTTER;
const CASSETTE_X: f64 = ARRAY_X + CASSETTE_MARGIN_X * 2.0;
const CASSETTE_Y: f64 = ARRAY_Y + CASSETTE_MARGIN_Y * 2.0;

const INSERT_X: f64 = CASSETTE_X + 76.0;
const INSERT_Y: f64 = 96.0;
const INSERT_Z: f64 = 18.0;
const COMB_X: f64 = CASSETTE_X + 24.0;
const COMB_Y: f64 = CASSETTE_Y + 20.0;
const COMB_Z: f64 = 12.0;

fn main() {
    let manifold_insert = manifold_insert();
    manifold_insert
        .write_stl("output/sterile_tubing_harness_manifold_insert.stl")
        .unwrap();
    println!("Exported: output/sterile_tubing_harness_manifold_insert.stl");

    let branch_comb = chip_branch_comb();
    branch_comb
        .write_stl("output/sterile_tubing_harness_branch_comb.stl")
        .unwrap();
    println!("Exported: output/sterile_tubing_harness_branch_comb.stl");

    let pump_coupler = pump_interface_coupler();
    pump_coupler
        .write_stl("output/sterile_tubing_harness_pump_coupler.stl")
        .unwrap();
    println!("Exported: output/sterile_tubing_harness_pump_coupler.stl");

    let assembly = manifold_insert.translate(0.0, -(CASSETTE_Y / 2.0 + 48.0), 14.0)
        + branch_comb.translate(0.0, 0.0, 18.0)
        + pump_coupler.translate(-(INSERT_X / 2.0 + 42.0), -(CASSETTE_Y / 2.0 + 48.0), 18.0)
        + pump_coupler.rotate(0.0, 0.0, 180.0).translate(
            INSERT_X / 2.0 + 42.0,
            -(CASSETTE_Y / 2.0 + 48.0),
            18.0,
        );

    assembly
        .write_stl("output/sterile_tubing_harness_assembly.stl")
        .unwrap();
    println!("Exported: output/sterile_tubing_harness_assembly.stl");

    println!(
        "Sterile tubing harness: disposable row manifold insert, 20 chip branch strain-relief comb, keyed pump couplers, and 4x5 cassette alignment."
    );
}

fn manifold_insert() -> Part {
    let body = centered_cube("harness_manifold_insert_body", INSERT_X, INSERT_Y, INSERT_Z);
    let mut cuts = Part::empty("harness_manifold_insert_cuts");

    for row in 0..ROWS {
        let y = row_y(row) * (INSERT_Y - 28.0) / ARRAY_Y;

        let row_trunk = centered_cylinder(
            format!("row_{row}_trunk_channel"),
            ROW_TRUNK_D / 2.0,
            INSERT_X + 6.0,
            32,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, y, 1.5);
        let top_slot = centered_cube(
            format!("row_{row}_trunk_top_slot"),
            INSERT_X + 8.0,
            ROW_TRUNK_D + 1.2,
            INSERT_Z,
        )
        .translate(0.0, y, 7.0);

        cuts = cuts + row_trunk + top_slot;

        for col in 0..COLS {
            let x = chip_x(col);
            let branch_socket = centered_cylinder(
                format!("row_{row}_col_{col}_branch_socket"),
                TUBE_CHANNEL_D / 2.0,
                INSERT_Y + 8.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, y, 1.5);

            let branch_slot = centered_cube(
                format!("row_{row}_col_{col}_branch_top_slot"),
                TUBE_CHANNEL_D + 1.2,
                INSERT_Y + 8.0,
                INSERT_Z,
            )
            .translate(x, y, 7.0);

            let bubble_window = centered_cube(
                format!("row_{row}_col_{col}_bubble_window"),
                18.0,
                10.0,
                INSERT_Z + 2.0,
            )
            .translate(x, y + 18.0, 0.0);

            cuts = cuts + branch_socket + branch_slot + bubble_window;
        }
    }

    let mut latch_holes = Part::empty("harness_manifold_latch_holes");
    for (i, (x, y)) in [
        (-(INSERT_X / 2.0 - 18.0), -(INSERT_Y / 2.0 - 14.0)),
        (INSERT_X / 2.0 - 18.0, -(INSERT_Y / 2.0 - 14.0)),
        (-(INSERT_X / 2.0 - 18.0), INSERT_Y / 2.0 - 14.0),
        (INSERT_X / 2.0 - 18.0, INSERT_Y / 2.0 - 14.0),
        (0.0, INSERT_Y / 2.0 - 14.0),
    ]
    .iter()
    .enumerate()
    {
        latch_holes = latch_holes
            + centered_cylinder(
                format!("harness_latch_hole_{i}"),
                LATCH_HOLE_D / 2.0,
                24.0,
                24,
            )
            .translate(*x, *y, 0.0);
    }

    let key_notch = centered_cube("harness_asymmetric_key_notch", 42.0, 18.0, INSERT_Z + 2.0)
        .translate(-(INSERT_X / 2.0 - 44.0), INSERT_Y / 2.0 - 9.0, 0.0);

    body - cuts - latch_holes - key_notch
}

fn chip_branch_comb() -> Part {
    let long_rail_left = centered_cube("harness_left_branch_rail", 18.0, COMB_Y, COMB_Z).translate(
        -(CASSETTE_X / 2.0 + 18.0),
        0.0,
        0.0,
    );
    let long_rail_right = centered_cube("harness_right_branch_rail", 18.0, COMB_Y, COMB_Z)
        .translate(CASSETTE_X / 2.0 + 18.0, 0.0, 0.0);

    let mut row_bridges = Part::empty("harness_row_branch_bridges");
    let mut channel_cuts = Part::empty("harness_branch_comb_cuts");

    for row in 0..ROWS {
        let y = row_y(row);
        row_bridges = row_bridges
            + centered_cube(format!("harness_row_{row}_bridge"), COMB_X, 16.0, COMB_Z)
                .translate(0.0, y, 0.0);

        for col in 0..COLS {
            let x = chip_x(col);
            let channel = centered_cylinder(
                format!("harness_row_{row}_col_{col}_drop_channel"),
                TUBE_CHANNEL_D / 2.0,
                34.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, y, 0.0);
            let open_slot = centered_cube(
                format!("harness_row_{row}_col_{col}_drop_slot"),
                TUBE_CHANNEL_D + 1.0,
                36.0,
                COMB_Z,
            )
            .translate(x, y, 5.0);
            channel_cuts = channel_cuts + channel + open_slot;
        }
    }

    let mut fiducials = Part::empty("harness_comb_fiducials");
    for (i, (x, y)) in [
        (-(COMB_X / 2.0 - 26.0), COMB_Y / 2.0 - 26.0),
        (COMB_X / 2.0 - 26.0, COMB_Y / 2.0 - 26.0),
        (-(COMB_X / 2.0 - 26.0), -(COMB_Y / 2.0 - 26.0)),
    ]
    .iter()
    .enumerate()
    {
        let target = centered_cylinder(format!("harness_comb_fiducial_{i}"), 7.0, 2.0, 40)
            .translate(*x, *y, COMB_Z / 2.0 + 1.0);
        let center = centered_cylinder(format!("harness_comb_fiducial_hole_{i}"), 1.5, 3.0, 20)
            .translate(*x, *y, COMB_Z / 2.0 + 1.0);
        fiducials = fiducials + (target - center);
    }

    long_rail_left + long_rail_right + (row_bridges - channel_cuts) + fiducials
}

fn pump_interface_coupler() -> Part {
    let body = centered_cube("harness_pump_coupler_body", 68.0, 118.0, 26.0);
    let mut channels = Part::empty("harness_pump_coupler_channels");

    for row in 0..ROWS {
        let y = row_y(row) * 82.0 / ARRAY_Y;
        let tube = centered_cylinder(
            format!("harness_pump_coupler_channel_{row}"),
            ROW_TRUNK_D / 2.0,
            72.0,
            32,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, y, 0.0);
        let top_slot = centered_cube(
            format!("harness_pump_coupler_slot_{row}"),
            72.0,
            ROW_TRUNK_D + 1.2,
            26.0,
        )
        .translate(0.0, y, 9.0);
        channels = channels + tube + top_slot;
    }

    let clamp_window = centered_cube("harness_pump_coupler_clamp_window", 38.0, 96.0, 12.0)
        .translate(0.0, 0.0, 5.0);
    let keyed_flat = centered_cube("harness_pump_coupler_keyed_flat", 18.0, 28.0, 28.0)
        .translate(-25.0, 45.0, 0.0);

    body - channels - clamp_window - keyed_flat
}

fn chip_x(col: usize) -> f64 {
    -((COLS as f64 - 1.0) * PITCH_X) / 2.0 + col as f64 * PITCH_X
}

fn row_y(row: usize) -> f64 {
    -((ROWS as f64 - 1.0) * PITCH_Y) / 2.0 + row as f64 * PITCH_Y
}
