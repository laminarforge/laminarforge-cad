use vcad::{centered_cube, centered_cylinder, Part};

// ─── CO2 Incubator ───
//
// 3D-printed (PETG) DIY CO2 incubator for cell culture. Features a
// gas-tight inner chamber with shelf rails, sensor pocket, and ports
// for heater, thermistor, fan, gas inlet, and cable grommet. An outer
// shell provides a 15mm insulation gap. Includes a gasketed door with
// acrylic window, ventilated shelves, and a humidity water tray.
//
// Exports:
//   - co2_incubator_chamber.stl  (inner PETG box)
//   - co2_incubator_shell.stl    (outer insulated housing)
//   - co2_incubator_door.stl     (front door with gasket channel)
//   - co2_incubator_shelf.stl    (ventilated shelf)
//   - co2_incubator_water_tray.stl (humidity tray)

fn main() {
    // ══════════════════════════════════════════════════════════════
    // 1. INNER CHAMBER
    // ══════════════════════════════════════════════════════════════

    // Inner cavity dimensions
    let inner_x = 250.0;
    let inner_y = 200.0;
    let inner_z = 200.0;
    let wall = 3.0;

    let outer_x = inner_x + wall * 2.0;
    let outer_y = inner_y + wall * 2.0;
    let outer_z = inner_z + wall * 2.0;

    // Outer box
    let chamber_outer = centered_cube("chamber_outer", outer_x, outer_y, outer_z);

    // Inner cavity (open front face for door)
    let chamber_inner = centered_cube("chamber_inner", inner_x, inner_y, inner_z);

    // Open the front face (remove front wall for door opening)
    let front_opening = centered_cube(
        "front_opening",
        inner_x,
        wall + 2.0,
        inner_z,
    )
    .translate(0.0, -(outer_y / 2.0), 0.0);

    // ── Shelf rail grooves (2 pairs at 80mm and 160mm from bottom) ──
    let rail_width = 5.0;
    let rail_depth = 3.0;
    let shelf_heights = [80.0, 160.0];

    let mut shelf_rails = Part::empty("shelf_rails");
    for &h in &shelf_heights {
        // Z position: bottom of inner cavity is at -inner_z/2, so shelf at bottom + h
        let rail_z = -(inner_z / 2.0) + h;

        // Left rail (groove into left inner wall)
        let left_rail = centered_cube("left_rail", rail_depth, inner_y, rail_width)
            .translate(-(inner_x / 2.0) + rail_depth / 2.0, 0.0, rail_z);

        // Right rail (groove into right inner wall)
        let right_rail = centered_cube("right_rail", rail_depth, inner_y, rail_width)
            .translate(inner_x / 2.0 - rail_depth / 2.0, 0.0, rail_z);

        shelf_rails = shelf_rails + left_rail + right_rail;
    }

    // ── Sensor pocket (MH-Z19B: 33x20x7mm recess on inside back wall) ──
    let sensor_pocket = centered_cube("sensor_pocket", 33.0, 7.0, 20.0)
        .translate(0.0, inner_y / 2.0 - 7.0 / 2.0, 20.0);

    // ── Heater port (6.2mm hole, bottom back) ──
    let heater_port = centered_cylinder("heater_port", 6.2 / 2.0, wall + 2.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, outer_y / 2.0, -(inner_z / 2.0) + 10.0);

    // ── Thermistor port (3.5mm hole, mid-height back) ──
    let thermistor_port = centered_cylinder("thermistor_port", 3.5 / 2.0, wall + 2.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(30.0, outer_y / 2.0, 0.0);

    // ── Fan port (40mm hole, right side wall) ──
    let fan_port = centered_cylinder("fan_port", 40.0 / 2.0, wall + 2.0, 48)
        .rotate(0.0, 90.0, 0.0)
        .translate(outer_x / 2.0, 0.0, 20.0);

    // ── Gas inlet port (6mm hole, top wall) ──
    let gas_inlet = centered_cylinder("gas_inlet", 6.0 / 2.0, wall + 2.0, 24)
        .translate(0.0, 30.0, outer_z / 2.0);

    // ── Power/sensor cable grommet hole (10mm, bottom back) ──
    let grommet_hole = centered_cylinder("grommet_hole", 10.0 / 2.0, wall + 2.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(-50.0, outer_y / 2.0, -(inner_z / 2.0) + 10.0);

    // Assemble chamber
    let chamber = (chamber_outer - chamber_inner - front_opening)
        - shelf_rails
        - sensor_pocket
        - heater_port
        - thermistor_port
        - fan_port
        - gas_inlet
        - grommet_hole;

    chamber
        .write_stl("output/co2_incubator_chamber.stl")
        .unwrap();

    println!("Exported: output/co2_incubator_chamber.stl");

    // ══════════════════════════════════════════════════════════════
    // 2. OUTER SHELL
    // ══════════════════════════════════════════════════════════════

    let insulation_gap = 15.0;
    let shell_wall = 3.0;

    let shell_outer_x = outer_x + (insulation_gap + shell_wall) * 2.0;
    let shell_outer_y = outer_y + (insulation_gap + shell_wall) * 2.0;
    let shell_outer_z = outer_z + (insulation_gap + shell_wall) * 2.0;

    let shell_inner_x = outer_x + insulation_gap * 2.0;
    let shell_inner_y = outer_y + insulation_gap * 2.0;
    let shell_inner_z = outer_z + insulation_gap * 2.0;

    let shell_outer = centered_cube("shell_outer", shell_outer_x, shell_outer_y, shell_outer_z);
    let shell_inner = centered_cube("shell_inner", shell_inner_x, shell_inner_y, shell_inner_z);

    // Front door access opening (slightly larger than inner chamber front opening)
    let door_opening_x = inner_x + 10.0;
    let door_opening_z = inner_z + 10.0;
    let shell_front_opening = centered_cube(
        "shell_front_opening",
        door_opening_x,
        shell_wall + insulation_gap + wall + 2.0,
        door_opening_z,
    )
    .translate(0.0, -(shell_outer_y / 2.0), 0.0);

    let shell = (shell_outer - shell_inner) - shell_front_opening;

    shell
        .write_stl("output/co2_incubator_shell.stl")
        .unwrap();

    println!("Exported: output/co2_incubator_shell.stl");

    // ══════════════════════════════════════════════════════════════
    // 3. DOOR
    // ══════════════════════════════════════════════════════════════

    let door_x = inner_x + 20.0;
    let door_z = inner_z + 20.0;
    let door_thickness = 8.0;

    let door_body = centered_cube("door_body", door_x, door_thickness, door_z);

    // Gasket channel (2mm wide x 2mm deep groove around perimeter)
    let gasket_width = 2.0;
    let gasket_depth = 2.0;
    // Inset from edge for gasket channel
    let gasket_inset = 6.0;
    let gasket_outer_x = door_x - gasket_inset * 2.0;
    let gasket_outer_z = door_z - gasket_inset * 2.0;
    let gasket_inner_x = gasket_outer_x - gasket_width * 2.0;
    let gasket_inner_z = gasket_outer_z - gasket_width * 2.0;

    let gasket_outer_cut = centered_cube(
        "gasket_outer",
        gasket_outer_x,
        gasket_depth,
        gasket_outer_z,
    )
    .translate(0.0, -(door_thickness / 2.0) + gasket_depth / 2.0, 0.0);

    let gasket_inner_fill = centered_cube(
        "gasket_inner",
        gasket_inner_x,
        gasket_depth + 0.1,
        gasket_inner_z,
    )
    .translate(0.0, -(door_thickness / 2.0) + gasket_depth / 2.0, 0.0);

    // Acrylic window cutout (100x80mm, through door)
    let window_cutout = centered_cube(
        "window_cutout",
        100.0,
        door_thickness + 2.0,
        80.0,
    );

    // Magnetic latch holes (2x 6mm dia, near top corners)
    let latch_offset_x = door_x / 2.0 - 15.0;
    let latch_z = door_z / 2.0 - 15.0;

    let latch_hole_1 = centered_cylinder("latch_1", 6.0 / 2.0, door_thickness + 2.0, 24)
        .translate(-latch_offset_x, 0.0, latch_z);

    let latch_hole_2 = centered_cylinder("latch_2", 6.0 / 2.0, door_thickness + 2.0, 24)
        .translate(latch_offset_x, 0.0, latch_z);

    let door = (door_body - gasket_outer_cut + gasket_inner_fill)
        - window_cutout
        - latch_hole_1
        - latch_hole_2;

    door.write_stl("output/co2_incubator_door.stl").unwrap();

    println!("Exported: output/co2_incubator_door.stl");

    // ══════════════════════════════════════════════════════════════
    // 4. SHELF
    // ══════════════════════════════════════════════════════════════

    // Shelf fits inside rail grooves: slightly narrower than inner_x
    let shelf_x = inner_x - 2.0; // clearance
    let shelf_y = inner_y - 10.0; // slightly shorter than depth for airflow
    let shelf_thickness = 3.0;

    // Shelf body with rail tabs on sides
    let shelf_body = centered_cube("shelf_body", shelf_x, shelf_y, shelf_thickness);

    // Side tabs that slide into rail grooves (3mm deep x 5mm tall)
    let tab_depth = 3.0 - 0.3; // slightly less than groove for fit
    let tab_height = 5.0 - 0.3;
    let left_tab = centered_cube("left_tab", tab_depth, shelf_y, tab_height)
        .translate(-(shelf_x / 2.0) - tab_depth / 2.0, 0.0, 0.0);
    let right_tab = centered_cube("right_tab", tab_depth, shelf_y, tab_height)
        .translate(shelf_x / 2.0 + tab_depth / 2.0, 0.0, 0.0);

    // Ventilation holes: grid of 5mm holes
    let hole_spacing_x = 15.0;
    let hole_spacing_y = 15.0;
    let hole_d = 5.0;
    let num_holes_x = ((shelf_x - 20.0) / hole_spacing_x) as i32;
    let num_holes_y = ((shelf_y - 20.0) / hole_spacing_y) as i32;

    let mut vent_holes = Part::empty("vent_holes");
    for ix in 0..num_holes_x {
        for iy in 0..num_holes_y {
            let hx = -(num_holes_x as f64 - 1.0) * hole_spacing_x / 2.0
                + (ix as f64) * hole_spacing_x;
            let hy = -(num_holes_y as f64 - 1.0) * hole_spacing_y / 2.0
                + (iy as f64) * hole_spacing_y;
            let hole = centered_cylinder(
                &format!("vent_{ix}_{iy}"),
                hole_d / 2.0,
                shelf_thickness + 2.0,
                16,
            )
            .translate(hx, hy, 0.0);
            vent_holes = vent_holes + hole;
        }
    }

    let shelf = (shelf_body + left_tab + right_tab) - vent_holes;

    shelf
        .write_stl("output/co2_incubator_shelf.stl")
        .unwrap();

    println!("Exported: output/co2_incubator_shelf.stl");

    // ══════════════════════════════════════════════════════════════
    // 5. WATER TRAY
    // ══════════════════════════════════════════════════════════════

    let tray_x = 200.0;
    let tray_y = 150.0;
    let tray_z = 20.0;
    let tray_wall = 2.0;

    let tray_outer = centered_cube("tray_outer", tray_x, tray_y, tray_z);
    let tray_inner = centered_cube(
        "tray_inner",
        tray_x - tray_wall * 2.0,
        tray_y - tray_wall * 2.0,
        tray_z - tray_wall,
    )
    .translate(0.0, 0.0, tray_wall / 2.0);

    let water_tray = tray_outer - tray_inner;

    water_tray
        .write_stl("output/co2_incubator_water_tray.stl")
        .unwrap();

    println!("Exported: output/co2_incubator_water_tray.stl");

    // ── Print specs ──

    println!();
    println!("── CO2 Incubator Specs ──");
    println!("  Inner chamber:   {inner_x:.0}mm x {inner_y:.0}mm x {inner_z:.0}mm");
    println!("  Chamber walls:   {wall:.0}mm PETG");
    println!("  Shelf rails:     2 pairs at 80mm and 160mm height, {rail_width:.0}mm wide x {rail_depth:.0}mm deep");
    println!("  Sensor pocket:   33x20x7mm (MH-Z19B CO2 sensor)");
    println!("  Heater port:     6.2mm dia (bottom back)");
    println!("  Thermistor port: 3.5mm dia (mid-height back)");
    println!("  Fan port:        40mm dia (side wall)");
    println!("  Gas inlet:       6mm dia (top)");
    println!("  Cable grommet:   10mm dia (bottom back)");
    println!("  Insulation gap:  {insulation_gap:.0}mm");
    println!("  Shell outer:     {shell_outer_x:.0}mm x {shell_outer_y:.0}mm x {shell_outer_z:.0}mm");
    println!("  Door:            {door_x:.0}mm x {door_z:.0}mm x {door_thickness:.0}mm");
    println!("  Gasket channel:  {gasket_width:.0}mm wide x {gasket_depth:.0}mm deep");
    println!("  Window cutout:   100mm x 80mm (acrylic)");
    println!("  Latch holes:     2x 6mm dia (magnetic)");
    println!("  Shelf:           {shelf_x:.0}mm x {shelf_y:.0}mm x {shelf_thickness:.0}mm with {hole_d:.0}mm vent holes");
    println!("  Water tray:      {tray_x:.0}mm x {tray_y:.0}mm x {tray_z:.0}mm");
    println!("  Material:        PETG");
}
