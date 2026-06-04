use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Robot-accessible cassette for automated multi-chip media exchange.
//
// Intent:
// - Hold one 20-chip shelf module: 4 columns x 5 rows of Rev C culture chips.
// - Treat the cassette as one AAV capsid/promoter/payload/dose condition for screening.
// - Keep media exchange on a constrained pump/reservoir/tubing path.
// - Give a robot arm or deck actuator obvious gripper/fiducial features.
// - Keep bubble inspection and leak containment visible during validation.
// - Scale AAV candidate count by adding matched cassettes, not by mixing candidates here.
//
// This is hardware architecture, not a biological readiness claim.

const COLS: usize = 4;
const ROWS: usize = 5;
const GUTTER: f64 = 5.0;
const MARGIN_X: f64 = 28.0;
const MARGIN_Y: f64 = 28.0;
const BASE_Z: f64 = 14.0;
const CHIP_CLEARANCE: f64 = 0.45;
const POCKET_DEPTH: f64 = 5.0;
const TUBE_OD: f64 = 4.8;

const PITCH_X: f64 = REVC_CHIP_LENGTH + GUTTER;
const PITCH_Y: f64 = REVC_CHIP_WIDTH + GUTTER;
const ARRAY_X: f64 = COLS as f64 * REVC_CHIP_LENGTH + (COLS as f64 - 1.0) * GUTTER;
const ARRAY_Y: f64 = ROWS as f64 * REVC_CHIP_WIDTH + (ROWS as f64 - 1.0) * GUTTER;
const BASE_X: f64 = ARRAY_X + MARGIN_X * 2.0;
const BASE_Y: f64 = ARRAY_Y + MARGIN_Y * 2.0;

fn main() {
    let base = cassette_base();
    base.write_stl("output/automated_media_exchange_cassette_base.stl")
        .unwrap();
    println!("Exported: output/automated_media_exchange_cassette_base.stl");

    let clamp = clamp_grid();
    clamp
        .write_stl("output/automated_media_exchange_cassette_clamp_frame.stl")
        .unwrap();
    println!("Exported: output/automated_media_exchange_cassette_clamp_frame.stl");

    let manifold = media_manifold();
    manifold
        .write_stl("output/automated_media_exchange_cassette_manifold.stl")
        .unwrap();
    println!("Exported: output/automated_media_exchange_cassette_manifold.stl");

    let robot_features = robot_interface_features();
    robot_features
        .write_stl("output/automated_media_exchange_cassette_robot_interface.stl")
        .unwrap();
    println!("Exported: output/automated_media_exchange_cassette_robot_interface.stl");

    let assembly =
        base + clamp.translate(
            0.0,
            0.0,
            BASE_Z / 2.0 + REVC_TOTAL_HEIGHT - POCKET_DEPTH + 2.0,
        ) + manifold.translate(0.0, 0.0, BASE_Z / 2.0 + 18.0)
            + robot_features.translate(0.0, 0.0, BASE_Z / 2.0 + 8.0);
    assembly
        .write_stl("output/automated_media_exchange_cassette_assembly.stl")
        .unwrap();
    println!("Exported: output/automated_media_exchange_cassette_assembly.stl");

    println!(
        "Automated media cassette: {COLS}x{ROWS} Rev C chips for one cassette-level AAV condition, row media rails, robot fiducials/gripper ears, bubble-view gutters, and leak tray."
    );
}

fn cassette_base() -> Part {
    let body = centered_cube("cassette_base_body", BASE_X, BASE_Y, BASE_Z);

    let mut cuts = Part::empty("cassette_base_cuts");
    let mut dowel_holes = Part::empty("chip_dowel_holes");

    for row in 0..ROWS {
        for col in 0..COLS {
            let (x, y) = chip_center(col, row);
            let pocket = centered_cube(
                format!("chip_pocket_{col}_{row}"),
                REVC_CHIP_LENGTH + CHIP_CLEARANCE * 2.0,
                REVC_CHIP_WIDTH + CHIP_CLEARANCE * 2.0,
                POCKET_DEPTH + 0.2,
            )
            .translate(x, y, BASE_Z / 2.0 - POCKET_DEPTH / 2.0 + 0.1);

            let optical_window = centered_cube(
                format!("optical_window_{col}_{row}"),
                REVC_CHIP_LENGTH - 22.0,
                REVC_CHIP_WIDTH - 22.0,
                BASE_Z + 2.0,
            )
            .translate(x, y, 0.0);

            cuts = cuts + pocket + optical_window;

            for (idx, (dx, dy)) in [
                (
                    -(REVC_CHIP_LENGTH / 2.0 + 2.2),
                    -(REVC_CHIP_WIDTH / 2.0 + 2.2),
                ),
                (-(REVC_CHIP_LENGTH / 2.0 + 2.2), REVC_CHIP_WIDTH / 2.0 + 2.2),
                (REVC_CHIP_LENGTH / 2.0 + 2.2, -(REVC_CHIP_WIDTH / 2.0 + 2.2)),
            ]
            .iter()
            .enumerate()
            {
                dowel_holes = dowel_holes
                    + centered_cylinder(
                        format!("chip_dowel_{col}_{row}_{idx}"),
                        2.9 / 2.0,
                        9.0,
                        20,
                    )
                    .translate(x + dx, y + dy, -1.5);
            }
        }
    }

    let leak_perimeter_x = centered_cube("leak_perimeter_x", BASE_X - 22.0, 8.0, 4.0).translate(
        0.0,
        -(BASE_Y / 2.0 - 14.0),
        BASE_Z / 2.0 - 2.0,
    ) + centered_cube("leak_perimeter_x2", BASE_X - 22.0, 8.0, 4.0)
        .translate(0.0, BASE_Y / 2.0 - 14.0, BASE_Z / 2.0 - 2.0);
    let leak_perimeter_y = centered_cube("leak_perimeter_y", 8.0, BASE_Y - 22.0, 4.0).translate(
        -(BASE_X / 2.0 - 14.0),
        0.0,
        BASE_Z / 2.0 - 2.0,
    ) + centered_cube("leak_perimeter_y2", 8.0, BASE_Y - 22.0, 4.0)
        .translate(BASE_X / 2.0 - 14.0, 0.0, BASE_Z / 2.0 - 2.0);

    let mut bubble_gutters = Part::empty("bubble_gutters");
    for row in 0..ROWS {
        let (_, y) = chip_center(0, row);
        bubble_gutters = bubble_gutters
            + centered_cube(
                format!("bubble_gutter_{row}"),
                ARRAY_X + 18.0,
                8.0,
                BASE_Z + 2.0,
            )
            .translate(0.0, y - REVC_CHIP_WIDTH / 2.0 - GUTTER / 2.0, 0.0);
    }

    let drain_notch = centered_cylinder("drain_notch", 5.0, 20.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(BASE_X / 2.0 - 18.0, -BASE_Y / 2.0 + 10.0, 0.0);

    let mut mount_holes = Part::empty("cassette_mount_holes");
    for (i, (x, y)) in [
        (-(BASE_X / 2.0 - 14.0), -(BASE_Y / 2.0 - 14.0)),
        (BASE_X / 2.0 - 14.0, -(BASE_Y / 2.0 - 14.0)),
        (-(BASE_X / 2.0 - 14.0), BASE_Y / 2.0 - 14.0),
        (BASE_X / 2.0 - 14.0, BASE_Y / 2.0 - 14.0),
        (0.0, -(BASE_Y / 2.0 - 14.0)),
        (0.0, BASE_Y / 2.0 - 14.0),
    ]
    .iter()
    .enumerate()
    {
        mount_holes = mount_holes
            + centered_cylinder(format!("m4_mount_{i}"), 4.3 / 2.0, BASE_Z + 2.0, 24)
                .translate(*x, *y, 0.0);
    }

    body - cuts
        - dowel_holes
        - leak_perimeter_x
        - leak_perimeter_y
        - bubble_gutters
        - drain_notch
        - mount_holes
}

fn clamp_grid() -> Part {
    let outer = centered_cube("clamp_grid_outer", ARRAY_X + 24.0, ARRAY_Y + 24.0, 6.0);
    let mut openings = Part::empty("clamp_grid_openings");
    let mut screw_holes = Part::empty("clamp_grid_screw_holes");

    for row in 0..ROWS {
        for col in 0..COLS {
            let (x, y) = chip_center(col, row);
            openings = openings
                + centered_cube(
                    format!("clamp_opening_{col}_{row}"),
                    REVC_CHIP_LENGTH - 10.0,
                    REVC_CHIP_WIDTH - 10.0,
                    7.0,
                )
                .translate(x, y, 0.0);

            for (idx, (dx, dy)) in [
                (-(REVC_CHIP_LENGTH / 2.0 + 7.0), 0.0),
                (REVC_CHIP_LENGTH / 2.0 + 7.0, 0.0),
                (0.0, -(REVC_CHIP_WIDTH / 2.0 + 7.0)),
                (0.0, REVC_CHIP_WIDTH / 2.0 + 7.0),
            ]
            .iter()
            .enumerate()
            {
                screw_holes = screw_holes
                    + centered_cylinder(format!("clamp_m3_{col}_{row}_{idx}"), 3.2 / 2.0, 8.0, 20)
                        .translate(x + dx, y + dy, 0.0);
            }
        }
    }

    outer - openings - screw_holes
}

fn media_manifold() -> Part {
    let left_bus = side_tubing_bus("inlet").translate(-(BASE_X / 2.0 + 18.0), 0.0, 0.0);
    let right_bus = side_tubing_bus("outlet").translate(BASE_X / 2.0 + 18.0, 0.0, 0.0);
    let mut row_rails = Part::empty("row_rails");

    for row in 0..ROWS {
        let (_, y) = chip_center(0, row);
        row_rails = row_rails
            + centered_cube(format!("row_media_rail_{row}"), ARRAY_X + 20.0, 12.0, 10.0)
                .translate(0.0, y, 0.0);
    }

    left_bus + right_bus + row_rails
}

fn side_tubing_bus(name: &str) -> Part {
    let body = centered_cube(format!("{name}_bus_body"), 32.0, BASE_Y - 30.0, 20.0);
    let mut channels = Part::empty(format!("{name}_bus_channels"));

    for row in 0..ROWS {
        let (_, y) = chip_center(0, row);
        let channel = centered_cylinder(format!("{name}_row_tube_{row}"), TUBE_OD / 2.0, 36.0, 24)
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, y, 0.0);
        let top_slot =
            centered_cube(format!("{name}_row_slot_{row}"), 34.0, 7.0, 14.0).translate(0.0, y, 5.0);
        channels = channels + channel + top_slot;
    }

    let label_recess = centered_cube(format!("{name}_label_recess"), 2.0, BASE_Y - 70.0, 8.0)
        .translate(12.0, 0.0, 2.0);

    body - channels - label_recess
}

fn robot_interface_features() -> Part {
    let left_grip = gripper_ear("left").translate(-BASE_X / 2.0 - 13.0, 0.0, 0.0);
    let right_grip = gripper_ear("right").translate(BASE_X / 2.0 + 13.0, 0.0, 0.0);
    let fiducials = fiducial_targets();
    left_grip + right_grip + fiducials
}

fn gripper_ear(name: &str) -> Part {
    let body = centered_cube(format!("{name}_gripper_ear"), 26.0, 220.0, 18.0);
    let grip_slot =
        centered_cube(format!("{name}_gripper_slot"), 12.0, 180.0, 10.0).translate(0.0, 0.0, 2.0);
    let lead_chamfer =
        centered_cube(format!("{name}_lead_chamfer"), 28.0, 18.0, 20.0).translate(0.0, 101.0, 0.0);
    body - grip_slot - lead_chamfer
}

fn fiducial_targets() -> Part {
    let mut targets = Part::empty("fiducials");
    for (i, (x, y)) in [
        (-(BASE_X / 2.0 - 30.0), BASE_Y / 2.0 - 24.0),
        (BASE_X / 2.0 - 30.0, BASE_Y / 2.0 - 24.0),
        (BASE_X / 2.0 - 30.0, -(BASE_Y / 2.0 - 24.0)),
    ]
    .iter()
    .enumerate()
    {
        let target =
            centered_cylinder(format!("fiducial_disc_{i}"), 7.0, 2.0, 40).translate(*x, *y, 8.0);
        let center_hole =
            centered_cylinder(format!("fiducial_center_{i}"), 1.5, 3.0, 20).translate(*x, *y, 8.0);
        targets = targets + (target - center_hole);
    }
    targets
}

fn chip_center(col: usize, row: usize) -> (f64, f64) {
    let x = -((COLS as f64 - 1.0) * PITCH_X) / 2.0 + col as f64 * PITCH_X;
    let y = -((ROWS as f64 - 1.0) * PITCH_Y) / 2.0 + row as f64 * PITCH_Y;
    (x, y)
}
