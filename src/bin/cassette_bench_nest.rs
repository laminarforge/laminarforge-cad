use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
use vcad::{centered_cube, centered_cylinder, Part};

// Reusable bench/deck nest for the automated 20-chip media exchange cassette.
//
// Intent:
// - Give the cassette a repeatable datum on a bench, robot deck, or incubator shelf.
// - Keep tubing and the sterile harness clear of pinch points.
// - Catch small leaks during dry/no-cell validation and route them to one drain corner.
// - Provide hard stops, latch posts, fiducials, and gripper/gantry approach clearance.
//
// This is a mechanical validation fixture. It is not a sterile culture chamber.

const COLS: usize = 4;
const ROWS: usize = 5;
const GUTTER: f64 = 5.0;
const CASSETTE_MARGIN_X: f64 = 28.0;
const CASSETTE_MARGIN_Y: f64 = 28.0;

const ARRAY_X: f64 = COLS as f64 * REVC_CHIP_LENGTH + (COLS as f64 - 1.0) * GUTTER;
const ARRAY_Y: f64 = ROWS as f64 * REVC_CHIP_WIDTH + (ROWS as f64 - 1.0) * GUTTER;
const CASSETTE_X: f64 = ARRAY_X + CASSETTE_MARGIN_X * 2.0;
const CASSETTE_Y: f64 = ARRAY_Y + CASSETTE_MARGIN_Y * 2.0;

const NEST_X: f64 = CASSETTE_X + 160.0;
const NEST_Y: f64 = CASSETTE_Y + 150.0;
const TRAY_Z: f64 = 18.0;
const RAIL_Z: f64 = 20.0;
const DATUM_RAIL_W: f64 = 16.0;
const TUBE_TRENCH_W: f64 = 34.0;

fn main() {
    let tray = leak_tray();
    tray.write_stl("output/cassette_bench_nest_leak_tray.stl")
        .unwrap();
    println!("Exported: output/cassette_bench_nest_leak_tray.stl");

    let datum = datum_rails();
    datum
        .write_stl("output/cassette_bench_nest_datum_rails.stl")
        .unwrap();
    println!("Exported: output/cassette_bench_nest_datum_rails.stl");

    let service = service_bridges();
    service
        .write_stl("output/cassette_bench_nest_service_bridges.stl")
        .unwrap();
    println!("Exported: output/cassette_bench_nest_service_bridges.stl");

    let assembly = tray
        + datum.translate(0.0, 0.0, TRAY_Z / 2.0 + RAIL_Z / 2.0)
        + service.translate(0.0, 0.0, TRAY_Z / 2.0 + 16.0);
    assembly
        .write_stl("output/cassette_bench_nest_assembly.stl")
        .unwrap();
    println!("Exported: output/cassette_bench_nest_assembly.stl");

    println!(
        "Cassette bench nest: {:.0}mm x {:.0}mm deck module for a 4x5 cassette, with datum rails, leak tray, tube trenches, latch posts, and robot fiducials.",
        NEST_X, NEST_Y
    );
}

fn leak_tray() -> Part {
    let tray = centered_cube("cassette_nest_tray_body", NEST_X, NEST_Y, TRAY_Z);
    let basin = centered_cube(
        "cassette_nest_recessed_basin",
        CASSETTE_X + 70.0,
        CASSETTE_Y + 58.0,
        TRAY_Z + 2.0,
    )
    .translate(0.0, 0.0, 5.0);

    let left_tube_trench = centered_cube(
        "cassette_nest_left_tube_trench",
        TUBE_TRENCH_W,
        NEST_Y + 2.0,
        TRAY_Z + 3.0,
    )
    .translate(-(CASSETTE_X / 2.0 + 38.0), 0.0, 3.5);
    let right_tube_trench = centered_cube(
        "cassette_nest_right_tube_trench",
        TUBE_TRENCH_W,
        NEST_Y + 2.0,
        TRAY_Z + 3.0,
    )
    .translate(CASSETTE_X / 2.0 + 38.0, 0.0, 3.5);
    let rear_harness_clearance = centered_cube(
        "cassette_nest_rear_harness_clearance",
        CASSETTE_X + 120.0,
        56.0,
        TRAY_Z + 3.0,
    )
    .translate(0.0, -(CASSETTE_Y / 2.0 + 52.0), 3.5);

    let drain_sump = centered_cube("cassette_nest_drain_sump", 70.0, 42.0, TRAY_Z + 2.0).translate(
        NEST_X / 2.0 - 50.0,
        -NEST_Y / 2.0 + 38.0,
        2.0,
    );
    let drain_port = centered_cylinder("cassette_nest_drain_port", 5.0, 26.0, 28)
        .rotate(90.0, 0.0, 0.0)
        .translate(NEST_X / 2.0 - 50.0, -NEST_Y / 2.0 + 14.0, 0.0);

    let mut mount_holes = Part::empty("cassette_nest_mount_holes");
    for (i, (x, y)) in [
        (-(NEST_X / 2.0 - 28.0), -(NEST_Y / 2.0 - 28.0)),
        (NEST_X / 2.0 - 28.0, -(NEST_Y / 2.0 - 28.0)),
        (-(NEST_X / 2.0 - 28.0), NEST_Y / 2.0 - 28.0),
        (NEST_X / 2.0 - 28.0, NEST_Y / 2.0 - 28.0),
        (0.0, -(NEST_Y / 2.0 - 28.0)),
        (0.0, NEST_Y / 2.0 - 28.0),
    ]
    .iter()
    .enumerate()
    {
        mount_holes = mount_holes
            + centered_cylinder(
                format!("cassette_nest_m5_mount_{i}"),
                5.3 / 2.0,
                TRAY_Z + 2.0,
                24,
            )
            .translate(*x, *y, 0.0);
    }

    tray - basin
        - left_tube_trench
        - right_tube_trench
        - rear_harness_clearance
        - drain_sump
        - drain_port
        - mount_holes
}

fn datum_rails() -> Part {
    let back_stop = centered_cube(
        "cassette_nest_back_y_datum",
        CASSETTE_X + 36.0,
        DATUM_RAIL_W,
        RAIL_Z,
    )
    .translate(0.0, CASSETTE_Y / 2.0 + DATUM_RAIL_W / 2.0 + 2.0, 0.0);
    let left_stop = centered_cube(
        "cassette_nest_left_x_datum",
        DATUM_RAIL_W,
        CASSETTE_Y + 28.0,
        RAIL_Z,
    )
    .translate(-(CASSETTE_X / 2.0 + DATUM_RAIL_W / 2.0 + 2.0), 0.0, 0.0);
    let right_soft_rail = centered_cube(
        "cassette_nest_right_soft_rail",
        DATUM_RAIL_W,
        CASSETTE_Y + 28.0,
        RAIL_Z * 0.65,
    )
    .translate(
        CASSETTE_X / 2.0 + DATUM_RAIL_W / 2.0 + 2.0,
        0.0,
        -RAIL_Z * 0.175,
    );

    let front_lip = centered_cube(
        "cassette_nest_front_low_lip",
        CASSETTE_X + 36.0,
        10.0,
        RAIL_Z * 0.45,
    )
    .translate(0.0, -(CASSETTE_Y / 2.0 + 7.0), -RAIL_Z * 0.275);

    let mut latch_posts = Part::empty("cassette_nest_latch_posts");
    for (i, (x, y)) in [
        (-(CASSETTE_X / 2.0 - 38.0), -(CASSETTE_Y / 2.0 - 34.0)),
        (CASSETTE_X / 2.0 - 38.0, -(CASSETTE_Y / 2.0 - 34.0)),
        (-(CASSETTE_X / 2.0 - 38.0), CASSETTE_Y / 2.0 - 34.0),
        (CASSETTE_X / 2.0 - 38.0, CASSETTE_Y / 2.0 - 34.0),
    ]
    .iter()
    .enumerate()
    {
        let post = centered_cylinder(format!("cassette_nest_latch_post_{i}"), 8.0, RAIL_Z, 32)
            .translate(*x, *y, 0.0);
        let screw = centered_cylinder(
            format!("cassette_nest_latch_screw_{i}"),
            3.2 / 2.0,
            RAIL_Z + 2.0,
            20,
        )
        .translate(*x, *y, 0.0);
        latch_posts = latch_posts + (post - screw);
    }

    back_stop + left_stop + right_soft_rail + front_lip + latch_posts + fiducial_targets()
}

fn service_bridges() -> Part {
    let rear_bridge = centered_cube(
        "cassette_nest_rear_harness_bridge",
        CASSETTE_X + 110.0,
        34.0,
        16.0,
    )
    .translate(0.0, -(CASSETTE_Y / 2.0 + 72.0), 0.0);

    let left_strain_relief = side_strain_relief("left").translate(
        -(CASSETTE_X / 2.0 + 72.0),
        -(CASSETTE_Y / 2.0 + 72.0),
        0.0,
    );
    let right_strain_relief = side_strain_relief("right").translate(
        CASSETTE_X / 2.0 + 72.0,
        -(CASSETTE_Y / 2.0 + 72.0),
        0.0,
    );

    rear_bridge + left_strain_relief + right_strain_relief
}

fn side_strain_relief(name: &str) -> Part {
    let body = centered_cube(format!("{name}_service_bridge_body"), 58.0, 38.0, 18.0);
    let tube_cut = centered_cylinder(format!("{name}_service_bridge_tube_cut"), 7.0, 62.0, 28)
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, 0.0, 0.0);
    let top_slot = centered_cube(format!("{name}_service_bridge_top_slot"), 62.0, 16.0, 20.0)
        .translate(0.0, 0.0, 7.0);
    let zip_tie_slot = centered_cube(
        format!("{name}_service_bridge_zip_tie_slot"),
        8.0,
        42.0,
        6.0,
    )
    .translate(0.0, 0.0, -4.0);
    body - tube_cut - top_slot - zip_tie_slot
}

fn fiducial_targets() -> Part {
    let mut targets = Part::empty("cassette_nest_fiducials");
    for (i, (x, y)) in [
        (-(NEST_X / 2.0 - 40.0), NEST_Y / 2.0 - 40.0),
        (NEST_X / 2.0 - 40.0, NEST_Y / 2.0 - 40.0),
        (-(NEST_X / 2.0 - 40.0), -(NEST_Y / 2.0 - 40.0)),
    ]
    .iter()
    .enumerate()
    {
        let disc = centered_cylinder(format!("cassette_nest_fiducial_disc_{i}"), 8.0, 2.5, 40)
            .translate(*x, *y, RAIL_Z / 2.0 + 1.25);
        let hole = centered_cylinder(format!("cassette_nest_fiducial_hole_{i}"), 1.5, 3.0, 20)
            .translate(*x, *y, RAIL_Z / 2.0 + 1.25);
        targets = targets + (disc - hole);
    }
    targets
}
