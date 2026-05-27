use vcad::{centered_cube, centered_cylinder, Part};

// Independent environmental logger housing for incubator validation.
// Keeps validation electronics separate from the incubator controller.
//
// Exports:
//   output/cell_culture_logger_enclosure_body.stl
//   output/cell_culture_logger_enclosure_lid.stl
//   output/cell_culture_logger_probe_clamp.stl
//   output/cell_culture_logger_assembly.stl

fn main() {
    let outer_x = 140.0;
    let outer_y = 90.0;
    let outer_z = 45.0;
    let wall = 3.0;

    let body_outer = centered_cube("logger_body_outer", outer_x, outer_y, outer_z);
    let body_inner = centered_cube(
        "logger_body_inner",
        outer_x - 2.0 * wall,
        outer_y - 2.0 * wall,
        outer_z - wall,
    )
    .translate(0.0, 0.0, wall / 2.0);
    let top_opening = centered_cube(
        "logger_top_opening",
        outer_x - 8.0,
        outer_y - 8.0,
        wall + 2.0,
    )
    .translate(0.0, 0.0, outer_z / 2.0);

    let mut probe_glands = Part::empty("probe_glands");
    for (i, x) in [-42.0, -14.0, 14.0, 42.0].iter().enumerate() {
        let gland = centered_cylinder(format!("probe_gland_{i}"), 6.0 / 2.0, wall + 2.0, 24)
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, outer_y / 2.0, 8.0);
        probe_glands = probe_glands + gland;
    }

    let usb_gland = centered_cube("usb_gland", 16.0, wall + 2.0, 8.0).translate(
        -outer_x / 2.0 + 22.0,
        outer_y / 2.0,
        -8.0,
    );

    let mut standoffs = Part::empty("logger_standoffs");
    for x in [-45.0, 45.0] {
        for y in [-22.0, 22.0] {
            let boss = centered_cylinder("logger_boss", 5.5 / 2.0, 8.0, 24).translate(
                x,
                y,
                -outer_z / 2.0 + wall + 4.0,
            );
            let hole = centered_cylinder("logger_boss_hole", 2.7 / 2.0, 10.0, 18).translate(
                x,
                y,
                -outer_z / 2.0 + wall + 4.0,
            );
            standoffs = standoffs + (boss - hole);
        }
    }

    let body = body_outer - body_inner - top_opening - probe_glands - usb_gland + standoffs;
    body.write_stl("output/cell_culture_logger_enclosure_body.stl")
        .unwrap();

    let lid_plate = centered_cube("logger_lid", outer_x - 4.0, outer_y - 4.0, 4.0);
    let display_window =
        centered_cube("logger_display_window", 46.0, 28.0, 5.0).translate(-22.0, 0.0, 0.0);
    let button_hole =
        centered_cylinder("logger_button", 6.0 / 2.0, 5.0, 24).translate(42.0, -20.0, 0.0);
    let status_hole =
        centered_cylinder("logger_led", 5.0 / 2.0, 5.0, 24).translate(42.0, 15.0, 0.0);
    let mut screw_holes = Part::empty("logger_lid_screws");
    for x in [-58.0, 58.0] {
        for y in [-33.0, 33.0] {
            screw_holes = screw_holes
                + centered_cylinder("logger_lid_screw", 3.2 / 2.0, 6.0, 18).translate(x, y, 0.0);
        }
    }
    let lid = lid_plate - display_window - button_hole - status_hole - screw_holes;
    lid.write_stl("output/cell_culture_logger_enclosure_lid.stl")
        .unwrap();

    let clamp_base = centered_cube("probe_clamp_base", 90.0, 18.0, 8.0);
    let mut probe_channels = Part::empty("probe_channels");
    for (i, x) in [-36.0, -12.0, 12.0, 36.0].iter().enumerate() {
        let channel = centered_cylinder(format!("probe_channel_{i}"), 3.2 / 2.0, 95.0, 18)
            .rotate(0.0, 90.0, 0.0)
            .translate(*x, 0.0, 1.0);
        probe_channels = probe_channels + channel;
    }
    let clamp = clamp_base - probe_channels;
    clamp
        .write_stl("output/cell_culture_logger_probe_clamp.stl")
        .unwrap();

    let assembly =
        body + lid.translate(0.0, 0.0, outer_z / 2.0 + 6.0) + clamp.translate(0.0, 70.0, 0.0);
    assembly
        .write_stl("output/cell_culture_logger_assembly.stl")
        .unwrap();

    println!("Exported: output/cell_culture_logger_enclosure_body.stl");
    println!("Exported: output/cell_culture_logger_enclosure_lid.stl");
    println!("Exported: output/cell_culture_logger_probe_clamp.stl");
    println!("Exported: output/cell_culture_logger_assembly.stl");
    println!(
        "Independent logger housing: temp/RH/CO2/power logger, not incubator control electronics."
    );
}
