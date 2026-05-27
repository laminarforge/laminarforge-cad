use vcad::{centered_cube, centered_cylinder, Part};

// Safety and workflow accessories for the starter low-voltage water bath.
//
// Research-driven design rules:
// - keep heating low-voltage and externally fused
// - provide independent probe placement
// - retain bottles/tubes so they do not tip into the heater area
// - route cables above the wet plane with strain relief
// - provide low-water float switch mounting
// - add a spill/splash tray around the bath footprint
//
// Exports:
//   output/water_bath_safety_bottle_rack.stl
//   output/water_bath_safety_probe_clamp.stl
//   output/water_bath_safety_cable_guard.stl
//   output/water_bath_safety_float_bracket.stl
//   output/water_bath_safety_spill_tray.stl
//   output/water_bath_safety_assembly.stl

fn main() {
    let rack = bottle_rack();
    rack.write_stl("output/water_bath_safety_bottle_rack.stl")
        .unwrap();

    let probe_clamp = probe_clamp();
    probe_clamp
        .write_stl("output/water_bath_safety_probe_clamp.stl")
        .unwrap();

    let cable_guard = cable_guard();
    cable_guard
        .write_stl("output/water_bath_safety_cable_guard.stl")
        .unwrap();

    let float_bracket = float_bracket();
    float_bracket
        .write_stl("output/water_bath_safety_float_bracket.stl")
        .unwrap();

    let spill_tray = spill_tray();
    spill_tray
        .write_stl("output/water_bath_safety_spill_tray.stl")
        .unwrap();

    let assembly = spill_tray
        + rack.translate(0.0, 0.0, 22.0)
        + probe_clamp.translate(-86.0, 0.0, 38.0)
        + cable_guard.translate(0.0, 86.0, 30.0)
        + float_bracket.translate(86.0, 0.0, 38.0);
    assembly
        .write_stl("output/water_bath_safety_assembly.stl")
        .unwrap();

    println!("Exported: output/water_bath_safety_bottle_rack.stl");
    println!("Exported: output/water_bath_safety_probe_clamp.stl");
    println!("Exported: output/water_bath_safety_cable_guard.stl");
    println!("Exported: output/water_bath_safety_float_bracket.stl");
    println!("Exported: output/water_bath_safety_spill_tray.stl");
    println!("Exported: output/water_bath_safety_assembly.stl");
    println!("Safety kit: bottle retention, independent probe clamp, cable strain relief, low-water float bracket, and spill tray.");
}

fn bottle_rack() -> Part {
    let rack_x = 130.0;
    let rack_y = 78.0;
    let rack_z = 52.0;
    let base =
        centered_cube("wb_rack_base", rack_x, rack_y, 5.0).translate(0.0, 0.0, -rack_z / 2.0 + 2.5);
    let top =
        centered_cube("wb_rack_top", rack_x, rack_y, 5.0).translate(0.0, 0.0, rack_z / 2.0 - 2.5);
    let side_l = centered_cube("wb_rack_side_l", 4.0, rack_y, rack_z).translate(
        -rack_x / 2.0 + 2.0,
        0.0,
        0.0,
    );
    let side_r = centered_cube("wb_rack_side_r", 4.0, rack_y, rack_z).translate(
        rack_x / 2.0 - 2.0,
        0.0,
        0.0,
    );
    let back =
        centered_cube("wb_rack_back", rack_x, 4.0, rack_z).translate(0.0, rack_y / 2.0 - 2.0, 0.0);

    let mut holes = Part::empty("wb_rack_holes");
    for (i, x) in [-42.0, 0.0, 42.0].iter().enumerate() {
        holes = holes
            + centered_cylinder(format!("wb_bottle_hole_{i}"), 17.0 / 2.0, 7.0, 32).translate(
                *x,
                -12.0,
                rack_z / 2.0 - 2.5,
            );
    }

    base + top + side_l + side_r + back - holes
}

fn probe_clamp() -> Part {
    let body = centered_cube("wb_probe_clamp_body", 30.0, 28.0, 34.0);
    let probe_bore = centered_cylinder("wb_probe_bore", 4.2 / 2.0, 34.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, 0.0, 4.0);
    let rim_slot = centered_cube("wb_probe_rim_slot", 34.0, 8.0, 16.0).translate(0.0, -12.0, -7.0);
    let thumb_screw = centered_cylinder("wb_probe_thumb_screw", 3.2 / 2.0, 32.0, 20)
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, 0.0, 4.0);
    body - probe_bore - rim_slot - thumb_screw
}

fn cable_guard() -> Part {
    let body = centered_cube("wb_cable_guard_body", 130.0, 22.0, 26.0);
    let mut channels = Part::empty("wb_cable_channels");
    for (i, x) in [-42.0, 0.0, 42.0].iter().enumerate() {
        let channel = centered_cylinder(format!("wb_cable_channel_{i}"), 7.0 / 2.0, 24.0, 24)
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 0.0, 2.0);
        let top_slot =
            centered_cube(format!("wb_cable_slot_{i}"), 10.0, 24.0, 16.0).translate(*x, 0.0, 9.0);
        channels = channels + channel + top_slot;
    }
    body - channels
}

fn float_bracket() -> Part {
    let plate = centered_cube("wb_float_plate", 34.0, 28.0, 42.0);
    let float_bore = centered_cylinder("wb_float_bore", 12.5 / 2.0, 30.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, 0.0, 6.0);
    let rim_slot = centered_cube("wb_float_rim_slot", 38.0, 8.0, 18.0).translate(0.0, -12.0, -8.0);
    let cable_bore = centered_cylinder("wb_float_cable_bore", 4.0 / 2.0, 30.0, 20)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, 0.0, 17.0);
    plate - float_bore - rim_slot - cable_bore
}

fn spill_tray() -> Part {
    let tray_x = 230.0;
    let tray_y = 175.0;
    let tray_z = 14.0;
    let wall = 8.0;
    let outer = centered_cube("wb_spill_tray_outer", tray_x, tray_y, tray_z);
    let inner = centered_cube(
        "wb_spill_tray_inner",
        tray_x - 2.0 * wall,
        tray_y - 2.0 * wall,
        tray_z,
    )
    .translate(0.0, 0.0, 4.0);
    let drain_recess = centered_cylinder("wb_spill_tray_drain", 10.0 / 2.0, 2.0, 24).translate(
        tray_x / 2.0 - 24.0,
        -tray_y / 2.0 + 24.0,
        tray_z / 2.0,
    );
    outer - inner - drain_recess
}
