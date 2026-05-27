use vcad::{centered_cube, centered_cylinder, Part};

// ─── CO2 Incubator ───
//
// 3D-printed (PETG) starter CO2 incubator cabinet prototype.
// Features a gas-tight inner chamber with shelf rails, rear service
// manifold, external electronics bay, heater diffuser, and ports for
// heater, redundant temperature sensing, fan, CO2 inlet, sampling,
// passive relief, and cable glands. An outer shell provides a 25mm
// insulation gap. Includes a gasketed door with acrylic window,
// ventilated shelves, and a humidity water tray.
//
// Research status: internal prototype only. Do not publish as a public
// build until the 0-20% CO2 sensor, gas relief path, sealing approach,
// and commissioning data are locked down.
//
// Exports:
//   - co2_incubator_chamber.stl  (inner PETG box)
//   - co2_incubator_shell.stl    (outer insulated housing)
//   - co2_incubator_door.stl     (front door with gasket channel)
//   - co2_incubator_shelf.stl    (ventilated shelf)
//   - co2_incubator_water_tray.stl (humidity tray)
//   - co2_incubator_service_bay.stl (external electronics enclosure)
//   - co2_incubator_service_manifold.stl (rear bulkhead for gas/sensors)
//   - co2_incubator_heater_diffuser.stl (warm-air diffuser over heater)

fn main() {
    // ══════════════════════════════════════════════════════════════
    // 1. INNER CHAMBER
    // ══════════════════════════════════════════════════════════════

    // Inner cavity dimensions
    let inner_x = 300.0;
    let inner_y = 250.0;
    let inner_z = 250.0;
    let wall = 3.0;

    let outer_x = inner_x + wall * 2.0;
    let outer_y = inner_y + wall * 2.0;
    let outer_z = inner_z + wall * 2.0;

    // Outer box
    let chamber_outer = centered_cube("chamber_outer", outer_x, outer_y, outer_z);

    // Inner cavity (open front face for door)
    let chamber_inner = centered_cube("chamber_inner", inner_x, inner_y, inner_z);

    // Open the front face (remove front wall for door opening)
    let front_opening = centered_cube("front_opening", inner_x, wall + 2.0, inner_z).translate(
        0.0,
        -(outer_y / 2.0),
        0.0,
    );

    // ── Shelf rail grooves (3 pairs for flasks/plates/chip fixtures) ──
    let rail_width = 5.0;
    let rail_depth = 3.0;
    let shelf_heights = [70.0, 135.0, 200.0];

    let mut shelf_rails = Part::empty("shelf_rails");
    for &h in &shelf_heights {
        // Z position: bottom of inner cavity is at -inner_z/2, so shelf at bottom + h
        let rail_z = -(inner_z / 2.0) + h;

        // Left rail (groove into left inner wall)
        let left_rail = centered_cube("left_rail", rail_depth, inner_y, rail_width).translate(
            -(inner_x / 2.0) + rail_depth / 2.0,
            0.0,
            rail_z,
        );

        // Right rail (groove into right inner wall)
        let right_rail = centered_cube("right_rail", rail_depth, inner_y, rail_width).translate(
            inner_x / 2.0 - rail_depth / 2.0,
            0.0,
            rail_z,
        );

        shelf_rails = shelf_rails + left_rail + right_rail;
    }

    // ── Temp/RH sensor pocket on rear wall ──
    // CO2 is sampled through a rear bulkhead instead of mounting the CO2
    // sensor inside the humid chamber. Final CO2 sensor geometry belongs in
    // the external service bay after selecting a 0-20% incubator-range module.
    let temp_rh_pocket = centered_cube("temp_rh_pocket", 20.0, 8.0, 20.0).translate(
        55.0,
        inner_y / 2.0 - 8.0 / 2.0,
        30.0,
    );

    // ── Heater port (8mm hole, bottom back) ──
    let heater_port = centered_cylinder("heater_port", 8.0 / 2.0, wall + 2.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, outer_y / 2.0, -(inner_z / 2.0) + 10.0);

    // ── Redundant temperature probe port (5mm hole, mid-height back) ──
    let thermistor_port = centered_cylinder("thermistor_port", 5.0 / 2.0, wall + 2.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(75.0, outer_y / 2.0, 0.0);

    // ── Fan port (60mm hole, right side wall) ──
    let fan_port = centered_cylinder("fan_port", 60.0 / 2.0, wall + 2.0, 48)
        .rotate(0.0, 90.0, 0.0)
        .translate(outer_x / 2.0, 0.0, 20.0);

    // ── CO2 inlet port (6mm hole, rear upper wall) ──
    let gas_inlet = centered_cylinder("gas_inlet", 6.0 / 2.0, wall + 2.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(-75.0, outer_y / 2.0, inner_z / 2.0 - 35.0);

    // ── CO2 sample outlet and passive relief port (rear upper wall) ──
    let sample_outlet = centered_cylinder("sample_outlet", 6.0 / 2.0, wall + 2.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, outer_y / 2.0, inner_z / 2.0 - 35.0);
    let relief_port = centered_cylinder("relief_port", 6.0 / 2.0, wall + 2.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(75.0, outer_y / 2.0, inner_z / 2.0 - 35.0);

    // ── Power/sensor cable gland hole (16mm, bottom back) ──
    let grommet_hole = centered_cylinder("grommet_hole", 16.0 / 2.0, wall + 2.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(-75.0, outer_y / 2.0, -(inner_z / 2.0) + 20.0);

    // ── Door latch insert pockets (front side walls, 2 per side) ──
    let latch_pocket_w = 8.0;
    let latch_pocket_h = 18.0;
    let latch_pocket_d = 4.0;
    let mut latch_pockets = Part::empty("latch_pockets");
    for &x_sign in &[-1.0, 1.0] {
        for &z in &[-70.0, 70.0] {
            let pocket = centered_cube(
                "latch_pocket",
                latch_pocket_d,
                latch_pocket_w,
                latch_pocket_h,
            )
            .translate(
                x_sign * (inner_x / 2.0 - latch_pocket_d / 2.0),
                -(outer_y / 2.0) + latch_pocket_w / 2.0,
                z,
            );
            latch_pockets = latch_pockets + pocket;
        }
    }

    // Assemble chamber
    let chamber = (chamber_outer - chamber_inner - front_opening)
        - shelf_rails
        - temp_rh_pocket
        - heater_port
        - thermistor_port
        - fan_port
        - gas_inlet
        - sample_outlet
        - relief_port
        - grommet_hole
        - latch_pockets;

    chamber
        .write_stl("output/co2_incubator_chamber.stl")
        .unwrap();

    println!("Exported: output/co2_incubator_chamber.stl");

    // ══════════════════════════════════════════════════════════════
    // 2. OUTER SHELL
    // ══════════════════════════════════════════════════════════════

    let insulation_gap = 25.0;
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

    shell.write_stl("output/co2_incubator_shell.stl").unwrap();

    println!("Exported: output/co2_incubator_shell.stl");

    // ══════════════════════════════════════════════════════════════
    // 3. DOOR
    // ══════════════════════════════════════════════════════════════

    let door_x = inner_x + 20.0;
    let door_z = inner_z + 20.0;
    let door_thickness = 12.0;

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

    let gasket_outer_cut =
        centered_cube("gasket_outer", gasket_outer_x, gasket_depth, gasket_outer_z).translate(
            0.0,
            -(door_thickness / 2.0) + gasket_depth / 2.0,
            0.0,
        );

    let gasket_inner_fill = centered_cube(
        "gasket_inner",
        gasket_inner_x,
        gasket_depth + 0.1,
        gasket_inner_z,
    )
    .translate(0.0, -(door_thickness / 2.0) + gasket_depth / 2.0, 0.0);

    // Acrylic window cutout (140x100mm, through door)
    let window_cutout = centered_cube("window_cutout", 140.0, door_thickness + 2.0, 100.0);

    // Magnetic latch holes (2x 6mm dia, near top corners)
    let latch_offset_x = door_x / 2.0 - 15.0;
    let latch_z = door_z / 2.0 - 15.0;

    let latch_hole_1 = centered_cylinder("latch_1", 6.0 / 2.0, door_thickness + 2.0, 24).translate(
        -latch_offset_x,
        0.0,
        latch_z,
    );

    let latch_hole_2 = centered_cylinder("latch_2", 6.0 / 2.0, door_thickness + 2.0, 24).translate(
        latch_offset_x,
        0.0,
        latch_z,
    );

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
    let left_tab = centered_cube("left_tab", tab_depth, shelf_y, tab_height).translate(
        -(shelf_x / 2.0) - tab_depth / 2.0,
        0.0,
        0.0,
    );
    let right_tab = centered_cube("right_tab", tab_depth, shelf_y, tab_height).translate(
        shelf_x / 2.0 + tab_depth / 2.0,
        0.0,
        0.0,
    );

    // Ventilation holes: grid of 5mm holes
    let hole_spacing_x = 15.0;
    let hole_spacing_y = 15.0;
    let hole_d = 5.0;
    let num_holes_x = ((shelf_x - 20.0) / hole_spacing_x) as i32;
    let num_holes_y = ((shelf_y - 20.0) / hole_spacing_y) as i32;

    let mut vent_holes = Part::empty("vent_holes");
    for ix in 0..num_holes_x {
        for iy in 0..num_holes_y {
            let hx =
                -(num_holes_x as f64 - 1.0) * hole_spacing_x / 2.0 + (ix as f64) * hole_spacing_x;
            let hy =
                -(num_holes_y as f64 - 1.0) * hole_spacing_y / 2.0 + (iy as f64) * hole_spacing_y;
            let hole = centered_cylinder(
                format!("vent_{ix}_{iy}"),
                hole_d / 2.0,
                shelf_thickness + 2.0,
                16,
            )
            .translate(hx, hy, 0.0);
            vent_holes = vent_holes + hole;
        }
    }

    let shelf = (shelf_body + left_tab + right_tab) - vent_holes;

    shelf.write_stl("output/co2_incubator_shelf.stl").unwrap();

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

    // ══════════════════════════════════════════════════════════════
    // 6. SERVICE BAY
    // ══════════════════════════════════════════════════════════════

    let bay_x = 160.0;
    let bay_y = 55.0;
    let bay_z = 180.0;
    let bay_wall = 3.0;

    let bay_outer = centered_cube("service_bay_outer", bay_x, bay_y, bay_z);
    let bay_inner = centered_cube(
        "service_bay_inner",
        bay_x - bay_wall * 2.0,
        bay_y - bay_wall,
        bay_z - bay_wall * 2.0,
    )
    .translate(0.0, bay_wall / 2.0, 0.0);

    let cable_slot = centered_cube("bay_cable_slot", 40.0, bay_wall + 2.0, 16.0).translate(
        0.0,
        -(bay_y / 2.0),
        -(bay_z / 2.0) + 28.0,
    );

    let vent_slot = centered_cube("bay_vent_slot", 110.0, bay_wall + 2.0, 6.0).translate(
        0.0,
        bay_y / 2.0,
        bay_z / 2.0 - 25.0,
    );

    let mut mount_posts = Part::empty("bay_mount_posts");
    for (px, pz) in [(-55.0, -55.0), (55.0, -55.0), (-55.0, 55.0), (55.0, 55.0)] {
        let post = centered_cylinder("bay_post", 4.0, 10.0, 24)
            .rotate(90.0, 0.0, 0.0)
            .translate(px, -(bay_y / 2.0) + 8.0, pz);
        mount_posts = mount_posts + post;
    }

    let service_bay = (bay_outer - bay_inner - cable_slot - vent_slot) + mount_posts;
    service_bay
        .write_stl("output/co2_incubator_service_bay.stl")
        .unwrap();

    println!("Exported: output/co2_incubator_service_bay.stl");

    // ══════════════════════════════════════════════════════════════
    // 7. REAR SERVICE MANIFOLD
    // ══════════════════════════════════════════════════════════════

    let manifold_x = 190.0;
    let manifold_y = 12.0;
    let manifold_z = 90.0;
    let manifold_plate = centered_cube("manifold_plate", manifold_x, manifold_y, manifold_z);

    let co2_bulkhead = centered_cylinder("co2_bulkhead", 6.0 / 2.0, manifold_y + 2.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(-65.0, 0.0, 25.0);
    let sample_bulkhead = centered_cylinder("sample_bulkhead", 6.0 / 2.0, manifold_y + 2.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, 0.0, 25.0);
    let relief_bulkhead = centered_cylinder("relief_bulkhead", 6.0 / 2.0, manifold_y + 2.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(65.0, 0.0, 25.0);
    let probe_bulkhead = centered_cylinder("probe_bulkhead", 5.0 / 2.0, manifold_y + 2.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(65.0, 0.0, 0.0);
    let heater_bulkhead = centered_cylinder("heater_bulkhead", 8.0 / 2.0, manifold_y + 2.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, 0.0, -25.0);
    let gland_bulkhead = centered_cylinder("gland_bulkhead", 16.0 / 2.0, manifold_y + 2.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(-65.0, 0.0, -25.0);

    let service_manifold = manifold_plate
        - co2_bulkhead
        - sample_bulkhead
        - relief_bulkhead
        - probe_bulkhead
        - heater_bulkhead
        - gland_bulkhead;
    service_manifold
        .write_stl("output/co2_incubator_service_manifold.stl")
        .unwrap();

    println!("Exported: output/co2_incubator_service_manifold.stl");

    // ══════════════════════════════════════════════════════════════
    // 8. HEATER DIFFUSER
    // ══════════════════════════════════════════════════════════════

    let diffuser_x = 220.0;
    let diffuser_y = 35.0;
    let diffuser_z = 18.0;
    let diffuser_body = centered_cube("diffuser_body", diffuser_x, diffuser_y, diffuser_z);
    let mut diffuser_slots = Part::empty("diffuser_slots");
    for ix in 0..9 {
        let slot_x = -80.0 + ix as f64 * 20.0;
        let slot = centered_cube("diffuser_slot", 8.0, diffuser_y + 2.0, diffuser_z + 2.0)
            .translate(slot_x, 0.0, 0.0);
        diffuser_slots = diffuser_slots + slot;
    }
    let diffuser = diffuser_body - diffuser_slots;
    diffuser
        .write_stl("output/co2_incubator_heater_diffuser.stl")
        .unwrap();

    println!("Exported: output/co2_incubator_heater_diffuser.stl");

    println!();
    println!("── CO2 Incubator Cabinet Specs ──");
    println!("  Inner chamber:   {inner_x:.0}mm x {inner_y:.0}mm x {inner_z:.0}mm");
    println!("  Chamber walls:   {wall:.0}mm PETG");
    println!("  Shelf rails:     3 pairs at 70/135/200mm height, {rail_width:.0}mm wide x {rail_depth:.0}mm deep");
    println!("  Temp/RH pocket:  20x20x8mm rear recess");
    println!("  Heater port:     8mm dia (bottom back)");
    println!("  Probe port:      5mm dia (mid-height back)");
    println!("  Fan port:        60mm dia (side wall)");
    println!("  CO2 inlet:       6mm dia (upper rear)");
    println!("  Sample outlet:   6mm dia (upper rear)");
    println!("  Relief port:     6mm dia (upper rear)");
    println!("  Cable gland:     16mm dia (bottom back)");
    println!("  Insulation gap:  {insulation_gap:.0}mm");
    println!(
        "  Shell outer:     {shell_outer_x:.0}mm x {shell_outer_y:.0}mm x {shell_outer_z:.0}mm"
    );
    println!("  Door:            {door_x:.0}mm x {door_z:.0}mm x {door_thickness:.0}mm");
    println!("  Gasket channel:  {gasket_width:.0}mm wide x {gasket_depth:.0}mm deep");
    println!("  Window cutout:   140mm x 100mm (acrylic)");
    println!("  Latch holes:     2x 6mm dia (magnetic)");
    println!("  Shelf:           {shelf_x:.0}mm x {shelf_y:.0}mm x {shelf_thickness:.0}mm with {hole_d:.0}mm vent holes");
    println!("  Water tray:      {tray_x:.0}mm x {tray_y:.0}mm x {tray_z:.0}mm");
    println!("  Service bay:     {bay_x:.0}mm x {bay_y:.0}mm x {bay_z:.0}mm");
    println!("  Manifold plate:  {manifold_x:.0}mm x {manifold_y:.0}mm x {manifold_z:.0}mm");
    println!("  Diffuser:        {diffuser_x:.0}mm x {diffuser_y:.0}mm x {diffuser_z:.0}mm");
    println!("  Material:        PETG");
}
