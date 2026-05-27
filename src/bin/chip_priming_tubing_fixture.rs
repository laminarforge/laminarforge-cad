use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
use vcad::{centered_cube, centered_cylinder, Part};

// Chip priming and tubing fixture for early microfluidic workflow validation.
//
// Holds one Rev C LaminarForge chip in an ANSI/SLAS footprint pocket, routes
// inlet/outlet tubing through strain-relief combs, provides a bubble-view slot,
// and catches priming overflow in a shallow trough.
//
// Exports:
//   output/chip_priming_fixture_base.stl
//   output/chip_priming_fixture_tubing_comb.stl
//   output/chip_priming_fixture_luer_clip.stl
//   output/chip_priming_fixture_assembly.stl

fn main() {
    let margin_x = 24.0;
    let margin_y = 34.0;
    let base_x = REVC_CHIP_LENGTH + margin_x * 2.0;
    let base_y = REVC_CHIP_WIDTH + margin_y * 2.0;
    let base_z = 12.0;
    let pocket_clearance = 0.25;
    let pocket_depth = 4.0;

    let base_body = centered_cube("priming_base", base_x, base_y, base_z);
    let chip_pocket = centered_cube(
        "chip_pocket",
        REVC_CHIP_LENGTH + pocket_clearance * 2.0,
        REVC_CHIP_WIDTH + pocket_clearance * 2.0,
        pocket_depth + 0.2,
    )
    .translate(0.0, 0.0, base_z / 2.0 - pocket_depth / 2.0 + 0.1);

    let bubble_window = centered_cube("bubble_window", REVC_CHIP_LENGTH - 24.0, 16.0, base_z + 2.0)
        .translate(0.0, 0.0, 0.0);
    let prime_trough = centered_cube("prime_trough", base_x - 28.0, 12.0, 5.0).translate(
        0.0,
        -(base_y / 2.0) + 18.0,
        base_z / 2.0 - 2.5,
    );

    let mut mount_holes = Part::empty("priming_mount_holes");
    for x in [-(base_x / 2.0 - 10.0), base_x / 2.0 - 10.0] {
        for y in [-(base_y / 2.0 - 10.0), base_y / 2.0 - 10.0] {
            mount_holes = mount_holes
                + centered_cylinder("priming_mount", 3.2 / 2.0, base_z + 2.0, 24)
                    .translate(x, y, 0.0);
        }
    }

    let mut dowel_holes = Part::empty("priming_dowel_holes");
    for (i, (x, y)) in [
        (
            -(REVC_CHIP_LENGTH / 2.0 + 2.0),
            -(REVC_CHIP_WIDTH / 2.0 + 2.0),
        ),
        (-(REVC_CHIP_LENGTH / 2.0 + 2.0), REVC_CHIP_WIDTH / 2.0 + 2.0),
        (REVC_CHIP_LENGTH / 2.0 + 2.0, -(REVC_CHIP_WIDTH / 2.0 + 2.0)),
    ]
    .iter()
    .enumerate()
    {
        dowel_holes = dowel_holes
            + centered_cylinder(format!("priming_dowel_{i}"), 2.9 / 2.0, 8.0, 24)
                .translate(*x, *y, 1.0);
    }

    let base = base_body - chip_pocket - bubble_window - prime_trough - mount_holes - dowel_holes;
    base.write_stl("output/chip_priming_fixture_base.stl")
        .unwrap();

    let inlet_comb = tubing_comb("inlet").translate(0.0, base_y / 2.0 + 12.0, base_z / 2.0 + 6.0);
    let outlet_comb =
        tubing_comb("outlet").translate(0.0, -(base_y / 2.0 + 12.0), base_z / 2.0 + 6.0);
    let combs = inlet_comb + outlet_comb;
    combs
        .write_stl("output/chip_priming_fixture_tubing_comb.stl")
        .unwrap();

    let luer_clip = luer_clip();
    luer_clip
        .write_stl("output/chip_priming_fixture_luer_clip.stl")
        .unwrap();

    let assembly = base
        + combs
        + luer_clip.translate(
            -(base_x / 2.0 + 24.0),
            base_y / 2.0 + 12.0,
            base_z / 2.0 + 8.0,
        )
        + luer_clip.translate(
            base_x / 2.0 + 24.0,
            -(base_y / 2.0 + 12.0),
            base_z / 2.0 + 8.0,
        );
    assembly
        .write_stl("output/chip_priming_fixture_assembly.stl")
        .unwrap();

    println!("Exported: output/chip_priming_fixture_base.stl");
    println!("Exported: output/chip_priming_fixture_tubing_comb.stl");
    println!("Exported: output/chip_priming_fixture_luer_clip.stl");
    println!("Exported: output/chip_priming_fixture_assembly.stl");
    println!("Chip priming fixture: Rev C pocket, bubble-view window, tubing combs, luer clips, and overflow trough.");
}

fn tubing_comb(name: &str) -> Part {
    let body = centered_cube(format!("{name}_comb_body"), 120.0, 18.0, 14.0);
    let mut channels = Part::empty(format!("{name}_comb_channels"));
    for (i, x) in [-45.0, -15.0, 15.0, 45.0].iter().enumerate() {
        let channel = centered_cylinder(format!("{name}_tube_channel_{i}"), 4.8 / 2.0, 20.0, 24)
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 0.0, 0.0);
        let top_slot =
            centered_cube(format!("{name}_tube_slot_{i}"), 6.0, 20.0, 12.0).translate(*x, 0.0, 5.0);
        channels = channels + channel + top_slot;
    }
    body - channels
}

fn luer_clip() -> Part {
    let body = centered_cube("luer_clip_body", 42.0, 28.0, 18.0);
    let luer_channel =
        centered_cylinder("luer_channel", 8.0 / 2.0, 44.0, 32).rotate(0.0, 90.0, 0.0);
    let snap_slot = centered_cube("luer_snap_slot", 44.0, 8.0, 14.0).translate(0.0, 7.0, 0.0);
    let mount_hole = centered_cylinder("luer_mount_hole", 3.2 / 2.0, 30.0, 20)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, -7.0, 0.0);
    body - luer_channel - snap_slot - mount_hole
}
