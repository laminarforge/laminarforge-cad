use vcad::{centered_cube, centered_cylinder, Part};

// Stable holder for a manual cell-culture aspiration waste train.
// This supports a primary disinfectant waste bottle, secondary overflow trap,
// inline hydrophobic/HEPA filter, tubing routing, and shallow spill containment.
//
// Exports:
//   output/aspirator_waste_trap_base_tray.stl
//   output/aspirator_waste_trap_bottle_cradle.stl
//   output/aspirator_waste_trap_filter_clip.stl
//   output/aspirator_waste_trap_tubing_comb.stl
//   output/aspirator_waste_trap_assembly.stl

fn main() {
    let tray_x = 360.0;
    let tray_y = 180.0;
    let tray_floor = 4.0;
    let tray_wall = 12.0;
    let tray_height = tray_floor + tray_wall;

    let tray_outer = centered_cube("asp_tray_outer", tray_x, tray_y, tray_height);
    let tray_cavity = centered_cube(
        "asp_tray_cavity",
        tray_x - 2.0 * tray_wall,
        tray_y - 2.0 * tray_wall,
        tray_height,
    )
    .translate(0.0, 0.0, tray_floor);
    let drain_recess = centered_cylinder("asp_tray_drain_recess", 12.0 / 2.0, 2.0, 32).translate(
        tray_x / 2.0 - 30.0,
        -tray_y / 2.0 + 30.0,
        tray_height / 2.0,
    );
    let base_tray = tray_outer - tray_cavity - drain_recess;
    base_tray
        .write_stl("output/aspirator_waste_trap_base_tray.stl")
        .unwrap();

    let primary_cradle = bottle_cradle("primary", 96.0, 98.0, 80.0).translate(-80.0, 0.0, 22.0);
    let secondary_cradle = bottle_cradle("secondary", 60.0, 62.0, 58.0).translate(90.0, 15.0, 18.0);
    let cradle = primary_cradle + secondary_cradle;
    cradle
        .write_stl("output/aspirator_waste_trap_bottle_cradle.stl")
        .unwrap();

    let filter_clip = filter_clip().translate(0.0, -58.0, 35.0);
    filter_clip
        .write_stl("output/aspirator_waste_trap_filter_clip.stl")
        .unwrap();

    let tubing_comb = tubing_comb().translate(0.0, 70.0, 26.0);
    tubing_comb
        .write_stl("output/aspirator_waste_trap_tubing_comb.stl")
        .unwrap();

    let assembly = base_tray + cradle + filter_clip + tubing_comb;
    assembly
        .write_stl("output/aspirator_waste_trap_assembly.stl")
        .unwrap();

    println!("Exported: output/aspirator_waste_trap_base_tray.stl");
    println!("Exported: output/aspirator_waste_trap_bottle_cradle.stl");
    println!("Exported: output/aspirator_waste_trap_filter_clip.stl");
    println!("Exported: output/aspirator_waste_trap_tubing_comb.stl");
    println!("Exported: output/aspirator_waste_trap_assembly.stl");
    println!("Aspiration support: primary waste bottle, secondary trap, inline filter, tubing routing, and spill tray.");
}

fn bottle_cradle(name: &str, bottle_dia: f64, ring_od: f64, height: f64) -> Part {
    let base = centered_cube(format!("{name}_base"), ring_od + 24.0, ring_od + 24.0, 8.0)
        .translate(0.0, 0.0, -height / 2.0 + 4.0);

    let back_support = centered_cube(format!("{name}_back_support"), ring_od, 8.0, height)
        .translate(0.0, ring_od / 2.0 - 4.0, 0.0);
    let left_support = centered_cube(format!("{name}_left_support"), 8.0, ring_od * 0.70, height)
        .translate(-(bottle_dia / 2.0 + 8.0), 4.0, 0.0);
    let right_support = centered_cube(format!("{name}_right_support"), 8.0, ring_od * 0.70, height)
        .translate(bottle_dia / 2.0 + 8.0, 4.0, 0.0);
    let bottom_socket =
        centered_cylinder(format!("{name}_bottom_socket"), bottle_dia / 2.0, 4.0, 80).translate(
            0.0,
            0.0,
            -height / 2.0 + 10.0,
        );

    let mut strap_slots = Part::empty(format!("{name}_strap_slots"));
    for z in [-height * 0.20, height * 0.20] {
        let slot = centered_cube(format!("{name}_strap_slot"), ring_od + 4.0, 4.0, 8.0).translate(
            0.0,
            ring_od / 2.0 - 5.0,
            z,
        );
        strap_slots = strap_slots + slot;
    }

    base + back_support + left_support + right_support + bottom_socket - strap_slots
}

fn filter_clip() -> Part {
    let clip_body = centered_cube("asp_filter_clip_body", 78.0, 22.0, 24.0);
    let filter_channel =
        centered_cylinder("asp_filter_channel", 8.0 / 2.0, 82.0, 32).rotate(0.0, 90.0, 0.0);
    let snap_slot = centered_cube("asp_filter_snap_slot", 82.0, 8.0, 16.0).translate(0.0, 8.0, 0.0);
    let mount_hole_l =
        centered_cylinder("asp_filter_mount_l", 3.2 / 2.0, 26.0, 18).translate(-30.0, 0.0, 0.0);
    let mount_hole_r =
        centered_cylinder("asp_filter_mount_r", 3.2 / 2.0, 26.0, 18).translate(30.0, 0.0, 0.0);
    clip_body - filter_channel - snap_slot - mount_hole_l - mount_hole_r
}

fn tubing_comb() -> Part {
    let body = centered_cube("asp_tubing_comb_body", 120.0, 18.0, 16.0);
    let mut channels = Part::empty("asp_tubing_comb_channels");
    for (i, x) in [-45.0, -15.0, 15.0, 45.0].iter().enumerate() {
        let channel = centered_cylinder(format!("asp_tube_channel_{i}"), 4.8 / 2.0, 20.0, 24)
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 0.0, 1.5);
        let top_slot =
            centered_cube(format!("asp_tube_slot_{i}"), 6.0, 20.0, 12.0).translate(*x, 0.0, 7.0);
        channels = channels + channel + top_slot;
    }
    body - channels
}
