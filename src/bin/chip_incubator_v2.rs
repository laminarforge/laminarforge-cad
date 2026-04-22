use vcad::{centered_cube, centered_cylinder, Part};

// ─── Chip Incubator v2 — Scaled Thermal Container ───
//
// Thermal-controlled container (incubator v2) sized to hold the 100-chip rack.
// The 100-chip rack footprint is 561×487×200mm + ~100mm rocker + ~250mm
// headroom for liquid-handler top-access.
//
// Interior target: 600 × 520 × 550 mm (X × Y × Z).
// Dual-wall insulated construction:
//     inner chamber (3mm PETG)
//     25mm PIR foam insulation gap
//     outer shell (3mm PETG)
//
// Operating conditions: 37°C, 5% CO2, ≥90% RH.
//
// Coordinate system: chamber centered at origin.
//   X: left/right      (−300 to +300)   interior 600mm
//   Y: front/back      (−260 to +260)   interior 520mm, front = −Y
//   Z: bottom/top      (−275 to +275)   interior 550mm
//
// Exports:
//   - chip_incubator_v2_chamber.stl    (inner PETG chamber)
//   - chip_incubator_v2_shell.stl      (outer PETG shell with insulation gap)
//   - chip_incubator_v2_door.stl       (front double-wall insulated door)
//   - chip_incubator_v2_top_hatch.stl  (liquid-handler top hatch)
//   - chip_incubator_v2_water_tray.stl (humidity water tray)
//   - chip_incubator_v2_rail.stl       (one rack slide-in rail)

fn main() {
    // ══════════════════════════════════════════════════════════════
    // 0. GLOBAL DIMENSIONS
    // ══════════════════════════════════════════════════════════════

    let inner_x: f64 = 600.0; // interior X
    let inner_y: f64 = 520.0; // interior Y
    let inner_z: f64 = 550.0; // interior Z
    let wall: f64 = 3.0; // PETG wall thickness (inner chamber)

    let chamber_outer_x = inner_x + wall * 2.0; // 606
    let chamber_outer_y = inner_y + wall * 2.0; // 526
    let chamber_outer_z = inner_z + wall * 2.0; // 556

    let insulation_gap: f64 = 25.0; // PIR foam
    let shell_wall: f64 = 3.0; // PETG wall (outer shell)

    // Shell interior = chamber_outer + insulation gap on all sides
    let shell_inner_x = chamber_outer_x + insulation_gap * 2.0; // 656 − 6 = 650... chamber_outer=606, +50 = 656, inner = 656-6=650
    let _ = shell_inner_x; // used through explicit values below
    // Per spec: outer shell interior 650×570×600, outer 656×576×606.
    let shell_outer_x = chamber_outer_x + (insulation_gap + shell_wall) * 2.0; // 656
    let shell_outer_y = chamber_outer_y + (insulation_gap + shell_wall) * 2.0; // 576
    let shell_outer_z = chamber_outer_z + (insulation_gap + shell_wall) * 2.0; // 606

    let shell_cavity_x = chamber_outer_x + insulation_gap * 2.0; // 656? no: 606+50=656 — but interior per spec is 650. Use computed.
    let shell_cavity_y = chamber_outer_y + insulation_gap * 2.0;
    let shell_cavity_z = chamber_outer_z + insulation_gap * 2.0;

    // ══════════════════════════════════════════════════════════════
    // 1. INNER CHAMBER
    // ══════════════════════════════════════════════════════════════

    let chamber_outer = centered_cube(
        "chamber_outer",
        chamber_outer_x,
        chamber_outer_y,
        chamber_outer_z,
    );
    let chamber_inner = centered_cube("chamber_inner", inner_x, inner_y, inner_z);

    // Open front face for door (front = −Y)
    let front_opening = centered_cube("front_opening", inner_x, wall + 2.0, inner_z)
        .translate(0.0, -(chamber_outer_y / 2.0), 0.0);

    // ── Heater port (8mm hole, bottom-rear center) ──
    let heater_port = centered_cylinder("heater_port", 8.0 / 2.0, wall + 2.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, chamber_outer_y / 2.0, -(inner_z / 2.0) + 15.0);

    // ── Temp+humidity sensor port (10mm hole, mid-rear) ──
    let sensor_port = centered_cylinder("sensor_port", 10.0 / 2.0, wall + 2.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(80.0, chamber_outer_y / 2.0, 0.0);

    // ── CO2 sensor pocket (25×35×10mm recess on rear wall, MH-Z19B) ──
    let co2_pocket = centered_cube("co2_pocket", 25.0, 10.0, 35.0)
        .translate(-80.0, chamber_outer_y / 2.0 - 5.0, 0.0);

    // ── Fan port (80mm hole, right side wall, 120mm fan body through insulation) ──
    let fan_port = centered_cylinder("fan_port", 80.0 / 2.0, wall + 2.0, 64)
        .rotate(0.0, 90.0, 0.0)
        .translate(chamber_outer_x / 2.0, 0.0, 50.0);

    // ── CO2 gas inlet (6mm hole, top-rear) ──
    let gas_inlet = centered_cylinder("gas_inlet", 6.0 / 2.0, wall + 2.0, 24)
        .translate(0.0, inner_y / 2.0 - 30.0, chamber_outer_z / 2.0);

    // ── Rocker wiring grommet (20mm hole, floor-rear) ──
    let rocker_grommet = centered_cylinder("rocker_grommet", 20.0 / 2.0, wall + 2.0, 48)
        .translate(120.0, inner_y / 2.0 - 40.0, -(chamber_outer_z / 2.0));

    // ── Drain hole (10mm, back corner of floor) ──
    let drain_hole = centered_cylinder("drain_hole", 10.0 / 2.0, wall + 2.0, 32)
        .translate(
            -(inner_x / 2.0) + 30.0,
            inner_y / 2.0 - 30.0,
            -(chamber_outer_z / 2.0),
        );

    // ── Liquid-handler top hatch cutout (350×250 on top face) ──
    let top_hatch_cutout = centered_cube("top_hatch_cutout", 350.0, 250.0, wall + 2.0)
        .translate(0.0, 0.0, chamber_outer_z / 2.0);

    // Gasket lip around the top hatch (3mm raised rim around cutout is implicit in
    // the hatch part; on the chamber we just cut the opening).

    // ── Water tray floor pocket (300 × 200 × 3mm deep near back) ──
    // Recessed into the floor: subtract a thin cube at floor level.
    let water_tray_pocket = centered_cube("water_tray_pocket", 300.0, 200.0, 3.0)
        .translate(
            0.0,
            inner_y / 2.0 - 100.0 - 10.0, // near back, with 10mm clear of back wall
            -(inner_z / 2.0) + 1.5,
        );

    // ── Condensation drip channel around perimeter of floor ──
    // 10mm wide × 5mm deep groove just inside the walls.
    // Build as: outer perim cube − inner perim cube (shell-like), at floor level.
    let channel_outer_x = inner_x - 20.0; // 20mm inset from walls
    let channel_outer_y = inner_y - 20.0;
    let channel_inner_x = channel_outer_x - 20.0; // 10mm wide on each side
    let channel_inner_y = channel_outer_y - 20.0;
    let channel_depth = 5.0;
    let channel_z = -(inner_z / 2.0) + channel_depth / 2.0;

    let channel_outer_box = centered_cube(
        "channel_outer_box",
        channel_outer_x,
        channel_outer_y,
        channel_depth,
    )
    .translate(0.0, 0.0, channel_z);
    let channel_inner_box = centered_cube(
        "channel_inner_box",
        channel_inner_x,
        channel_inner_y,
        channel_depth + 0.1,
    )
    .translate(0.0, 0.0, channel_z);

    let drip_channel = channel_outer_box - channel_inner_box;

    // ── Slide-in rail grooves (2 pairs) ──
    // Rails sit on the floor; the actual rails are printed separately and glued/screwed.
    // Here we just put locating pockets (5mm deep slot 12mm wide) to seat them.
    let rail_slot_depth = 3.0;
    let rail_slot_width = 12.0;
    let rail_slot_length = inner_y - 40.0; // along front/back axis
    let rail_slot_y_center = 0.0;
    let rail_slot_z = -(inner_z / 2.0) + rail_slot_depth / 2.0;
    let rail_x_offsets = [-180.0, -60.0, 60.0, 180.0]; // 2 pairs, centred
    let mut rail_slots = Part::empty("rail_slots");
    for (i, &xo) in rail_x_offsets.iter().enumerate() {
        let slot = centered_cube(
            &format!("rail_slot_{i}"),
            rail_slot_width,
            rail_slot_length,
            rail_slot_depth + 0.1,
        )
        .translate(xo, rail_slot_y_center, rail_slot_z);
        rail_slots = rail_slots + slot;
    }

    // ── Anti-condensation heater wire channel around top hatch (10mm wide, 3mm deep) ──
    let hatch_heat_outer_x = 370.0;
    let hatch_heat_outer_y = 270.0;
    let hatch_heat_inner_x = 350.0; // 10mm wide channel on each side
    let hatch_heat_inner_y = 250.0;
    let hatch_heat_depth = 3.0;
    let hatch_heat_z = chamber_outer_z / 2.0 - hatch_heat_depth / 2.0;

    let hatch_heat_outer = centered_cube(
        "hatch_heat_outer",
        hatch_heat_outer_x,
        hatch_heat_outer_y,
        hatch_heat_depth + 0.1,
    )
    .translate(0.0, 0.0, hatch_heat_z);
    let hatch_heat_inner = centered_cube(
        "hatch_heat_inner",
        hatch_heat_inner_x,
        hatch_heat_inner_y,
        hatch_heat_depth + 0.2,
    )
    .translate(0.0, 0.0, hatch_heat_z);
    let hatch_heat_channel = hatch_heat_outer - hatch_heat_inner;

    // Assemble chamber
    let chamber = (chamber_outer - chamber_inner - front_opening)
        - heater_port
        - sensor_port
        - co2_pocket
        - fan_port
        - gas_inlet
        - rocker_grommet
        - drain_hole
        - top_hatch_cutout
        - water_tray_pocket
        - drip_channel
        - rail_slots
        - hatch_heat_channel;

    chamber
        .write_stl("output/chip_incubator_v2_chamber.stl")
        .unwrap();
    println!("Exported: output/chip_incubator_v2_chamber.stl");

    // ══════════════════════════════════════════════════════════════
    // 2. OUTER SHELL
    // ══════════════════════════════════════════════════════════════

    let shell_outer = centered_cube(
        "shell_outer",
        shell_outer_x,
        shell_outer_y,
        shell_outer_z,
    );
    let shell_cavity = centered_cube(
        "shell_cavity",
        shell_cavity_x,
        shell_cavity_y,
        shell_cavity_z,
    );

    // Front door access opening
    let shell_front_opening = centered_cube(
        "shell_front_opening",
        inner_x + 20.0,
        shell_wall + insulation_gap + wall + 4.0,
        inner_z + 20.0,
    )
    .translate(0.0, -(shell_outer_y / 2.0), 0.0);

    // Matched pass-through ports (same positions, slightly larger for insulation gap)
    let shell_heater_port = centered_cylinder(
        "shell_heater_port",
        10.0 / 2.0,
        shell_wall + insulation_gap + wall + 4.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, shell_outer_y / 2.0, -(inner_z / 2.0) + 15.0);

    let shell_sensor_port = centered_cylinder(
        "shell_sensor_port",
        12.0 / 2.0,
        shell_wall + insulation_gap + wall + 4.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(80.0, shell_outer_y / 2.0, 0.0);

    let shell_co2_port = centered_cylinder(
        "shell_co2_port",
        14.0 / 2.0,
        shell_wall + insulation_gap + wall + 4.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-80.0, shell_outer_y / 2.0, 0.0);

    let shell_fan_port = centered_cylinder(
        "shell_fan_port",
        120.0 / 2.0,
        shell_wall + insulation_gap + wall + 4.0,
        64,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(shell_outer_x / 2.0, 0.0, 50.0);

    let shell_gas_inlet = centered_cylinder(
        "shell_gas_inlet",
        10.0 / 2.0,
        shell_wall + insulation_gap + wall + 4.0,
        24,
    )
    .translate(0.0, inner_y / 2.0 - 30.0, shell_outer_z / 2.0);

    let shell_rocker_grommet = centered_cylinder(
        "shell_rocker_grommet",
        25.0 / 2.0,
        shell_wall + insulation_gap + wall + 4.0,
        48,
    )
    .translate(120.0, inner_y / 2.0 - 40.0, -(shell_outer_z / 2.0));

    let shell_drain_hole = centered_cylinder(
        "shell_drain_hole",
        14.0 / 2.0,
        shell_wall + insulation_gap + wall + 4.0,
        32,
    )
    .translate(
        -(inner_x / 2.0) + 30.0,
        inner_y / 2.0 - 30.0,
        -(shell_outer_z / 2.0),
    );

    let shell_top_hatch_cutout = centered_cube(
        "shell_top_hatch_cutout",
        370.0,
        270.0,
        shell_wall + insulation_gap + wall + 4.0,
    )
    .translate(0.0, 0.0, shell_outer_z / 2.0);

    // ── Leveling feet mount pads (4× M6 through-holes in floor corners) ──
    let foot_inset = 30.0;
    let foot_positions = [
        (-(shell_outer_x / 2.0) + foot_inset, -(shell_outer_y / 2.0) + foot_inset),
        ((shell_outer_x / 2.0) - foot_inset, -(shell_outer_y / 2.0) + foot_inset),
        (-(shell_outer_x / 2.0) + foot_inset, (shell_outer_y / 2.0) - foot_inset),
        ((shell_outer_x / 2.0) - foot_inset, (shell_outer_y / 2.0) - foot_inset),
    ];
    let mut feet_holes = Part::empty("feet_holes");
    for (i, &(fx, fy)) in foot_positions.iter().enumerate() {
        let hole = centered_cylinder(&format!("foot_{i}"), 6.5 / 2.0, shell_wall + 4.0, 24)
            .translate(fx, fy, -(shell_outer_z / 2.0) + shell_wall / 2.0);
        feet_holes = feet_holes + hole;
    }

    let shell = (shell_outer - shell_cavity)
        - shell_front_opening
        - shell_heater_port
        - shell_sensor_port
        - shell_co2_port
        - shell_fan_port
        - shell_gas_inlet
        - shell_rocker_grommet
        - shell_drain_hole
        - shell_top_hatch_cutout
        - feet_holes;

    shell
        .write_stl("output/chip_incubator_v2_shell.stl")
        .unwrap();
    println!("Exported: output/chip_incubator_v2_shell.stl");

    // ══════════════════════════════════════════════════════════════
    // 3. FRONT DOOR (double-wall insulated)
    // ══════════════════════════════════════════════════════════════
    //
    // Total door: 5mm PETG inner + 25mm foam + 5mm PETG outer = 35mm thick.
    // Modeled as a single 35mm-thick body with a hollow foam cavity.

    let door_x = inner_x + 30.0; // 630mm
    let door_z = inner_z + 30.0; // 580mm
    let door_petg = 5.0;
    let door_foam = 25.0;
    let door_thickness = door_petg * 2.0 + door_foam; // 35mm

    let door_body = centered_cube("door_body", door_x, door_thickness, door_z);

    // Foam cavity (hollowed from inside, sealed on both faces by PETG skins)
    let door_foam_cavity = centered_cube(
        "door_foam_cavity",
        door_x - 20.0,
        door_foam,
        door_z - 20.0,
    );

    // Acrylic window cutout (300×200mm, through the entire door)
    let window_cutout = centered_cube("window_cutout", 300.0, door_thickness + 2.0, 200.0);

    // Anti-fog heater wire channel (1mm deep × 5mm wide groove around window, inner face)
    let window_heat_outer_x = 320.0;
    let window_heat_outer_z = 220.0;
    let window_heat_inner_x = 310.0;
    let window_heat_inner_z = 210.0;
    let window_heat_depth = 1.0;
    let window_heat_y = -(door_thickness / 2.0) + window_heat_depth / 2.0;

    let window_heat_outer = centered_cube(
        "window_heat_outer",
        window_heat_outer_x,
        window_heat_depth + 0.1,
        window_heat_outer_z,
    )
    .translate(0.0, window_heat_y, 0.0);
    let window_heat_inner = centered_cube(
        "window_heat_inner",
        window_heat_inner_x,
        window_heat_depth + 0.2,
        window_heat_inner_z,
    )
    .translate(0.0, window_heat_y, 0.0);
    let window_heat_channel = window_heat_outer - window_heat_inner;

    // Silicone gasket channel (3×3mm groove around full perimeter on inner face)
    let gasket_width = 3.0;
    let gasket_depth = 3.0;
    let gasket_inset = 10.0;
    let gasket_outer_x = door_x - gasket_inset * 2.0;
    let gasket_outer_z = door_z - gasket_inset * 2.0;
    let gasket_inner_x = gasket_outer_x - gasket_width * 2.0;
    let gasket_inner_z = gasket_outer_z - gasket_width * 2.0;
    let gasket_y = -(door_thickness / 2.0) + gasket_depth / 2.0;

    let gasket_outer_cut = centered_cube(
        "gasket_outer",
        gasket_outer_x,
        gasket_depth + 0.1,
        gasket_outer_z,
    )
    .translate(0.0, gasket_y, 0.0);
    let gasket_inner_fill = centered_cube(
        "gasket_inner",
        gasket_inner_x,
        gasket_depth + 0.2,
        gasket_inner_z,
    )
    .translate(0.0, gasket_y, 0.0);
    let gasket_channel = gasket_outer_cut - gasket_inner_fill;

    // Magnetic latch holes (2× 8mm neodymium) on right side
    let latch_x = door_x / 2.0 - 15.0;
    let latch_hole_1 = centered_cylinder("latch_1", 8.0 / 2.0, door_thickness + 2.0, 32)
        .translate(latch_x, 0.0, door_z / 2.0 - 30.0);
    let latch_hole_2 = centered_cylinder("latch_2", 8.0 / 2.0, door_thickness + 2.0, 32)
        .translate(latch_x, 0.0, -(door_z / 2.0) + 30.0);

    // Hinge mount holes on left side (2× 6mm for hinge pins)
    let hinge_x = -(door_x / 2.0) + 15.0;
    let hinge_hole_1 = centered_cylinder("hinge_1", 6.0 / 2.0, door_thickness + 2.0, 24)
        .translate(hinge_x, 0.0, door_z / 2.0 - 50.0);
    let hinge_hole_2 = centered_cylinder("hinge_2", 6.0 / 2.0, door_thickness + 2.0, 24)
        .translate(hinge_x, 0.0, -(door_z / 2.0) + 50.0);

    let door = door_body
        - door_foam_cavity
        - window_cutout
        - window_heat_channel
        - gasket_channel
        - latch_hole_1
        - latch_hole_2
        - hinge_hole_1
        - hinge_hole_2;

    door.write_stl("output/chip_incubator_v2_door.stl").unwrap();
    println!("Exported: output/chip_incubator_v2_door.stl");

    // ══════════════════════════════════════════════════════════════
    // 4. LIQUID-HANDLER TOP HATCH
    // ══════════════════════════════════════════════════════════════
    //
    // 360×260×15mm insulated hatch with silicone gasket and 2-point magnetic latch.

    let hatch_x = 360.0;
    let hatch_y = 260.0;
    let hatch_thickness = 15.0;
    let hatch_body = centered_cube("hatch_body", hatch_x, hatch_y, hatch_thickness);

    // Hatch gasket channel (3×3mm groove around perimeter on bottom face)
    let hatch_gasket_outer_x = hatch_x - 20.0;
    let hatch_gasket_outer_y = hatch_y - 20.0;
    let hatch_gasket_inner_x = hatch_gasket_outer_x - 6.0;
    let hatch_gasket_inner_y = hatch_gasket_outer_y - 6.0;
    let hatch_gasket_z = -(hatch_thickness / 2.0) + 1.5;

    let hatch_gasket_outer_cut = centered_cube(
        "hatch_gasket_outer",
        hatch_gasket_outer_x,
        hatch_gasket_outer_y,
        3.1,
    )
    .translate(0.0, 0.0, hatch_gasket_z);
    let hatch_gasket_inner_fill = centered_cube(
        "hatch_gasket_inner",
        hatch_gasket_inner_x,
        hatch_gasket_inner_y,
        3.2,
    )
    .translate(0.0, 0.0, hatch_gasket_z);
    let hatch_gasket = hatch_gasket_outer_cut - hatch_gasket_inner_fill;

    // 2-point magnetic latch holes (8mm dia, on short ends)
    let hatch_latch_1 = centered_cylinder("hatch_latch_1", 8.0 / 2.0, hatch_thickness + 2.0, 32)
        .translate(-(hatch_x / 2.0) + 20.0, 0.0, 0.0);
    let hatch_latch_2 = centered_cylinder("hatch_latch_2", 8.0 / 2.0, hatch_thickness + 2.0, 32)
        .translate(hatch_x / 2.0 - 20.0, 0.0, 0.0);

    // Finger pulls (2× 30×6mm slots on the top face)
    let pull_1 = centered_cube("pull_1", 60.0, 10.0, 2.0)
        .translate(-80.0, 0.0, hatch_thickness / 2.0 - 1.0);
    let pull_2 = centered_cube("pull_2", 60.0, 10.0, 2.0)
        .translate(80.0, 0.0, hatch_thickness / 2.0 - 1.0);

    let top_hatch = hatch_body - hatch_gasket - hatch_latch_1 - hatch_latch_2 - pull_1 - pull_2;

    top_hatch
        .write_stl("output/chip_incubator_v2_top_hatch.stl")
        .unwrap();
    println!("Exported: output/chip_incubator_v2_top_hatch.stl");

    // ══════════════════════════════════════════════════════════════
    // 5. WATER TRAY (humidity)
    // ══════════════════════════════════════════════════════════════

    let tray_x = 298.0; // slight clearance inside 300mm pocket
    let tray_y = 198.0;
    let tray_z = 25.0;
    let tray_wall = 2.5;

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
        .write_stl("output/chip_incubator_v2_water_tray.stl")
        .unwrap();
    println!("Exported: output/chip_incubator_v2_water_tray.stl");

    // ══════════════════════════════════════════════════════════════
    // 6. RACK SLIDE-IN RAIL (one part; print 4 for 2 pairs)
    // ══════════════════════════════════════════════════════════════

    let rail_length = inner_y - 40.0;
    let rail_base_w = 12.0; // fits in 12mm slot
    let rail_base_h = 3.0; // fits in 3mm slot
    let rail_top_w = 18.0; // wider roller track on top
    let rail_top_h = 8.0;

    let rail_base = centered_cube("rail_base", rail_base_w, rail_length, rail_base_h);
    let rail_top = centered_cube("rail_top", rail_top_w, rail_length, rail_top_h)
        .translate(0.0, 0.0, (rail_base_h + rail_top_h) / 2.0);

    // Roller V-groove down the middle of the top face (5mm wide × 3mm deep)
    let roller_groove = centered_cube("roller_groove", 5.0, rail_length + 2.0, 3.2)
        .translate(0.0, 0.0, rail_base_h / 2.0 + rail_top_h - 1.5);

    let rail = (rail_base + rail_top) - roller_groove;
    rail.write_stl("output/chip_incubator_v2_rail.stl")
        .unwrap();
    println!("Exported: output/chip_incubator_v2_rail.stl");

    // ══════════════════════════════════════════════════════════════
    // 7. SPECS REPORT
    // ══════════════════════════════════════════════════════════════

    let chamber_volume_l = (inner_x * inner_y * inner_z) / 1_000_000.0; // mm³ → L
    let shell_surface_area_m2 = 2.0
        * ((shell_outer_x * shell_outer_y)
            + (shell_outer_x * shell_outer_z)
            + (shell_outer_y * shell_outer_z))
        / 1_000_000.0; // mm² → m²

    println!();
    println!("══════════════════════════════════════════════════════════════");
    println!("   CHIP INCUBATOR v2 — SPECS");
    println!("══════════════════════════════════════════════════════════════");
    println!();
    println!(" DIMENSIONS");
    println!("   Inner chamber cavity:  {inner_x:.0} × {inner_y:.0} × {inner_z:.0} mm");
    println!("   Chamber outer (PETG):  {chamber_outer_x:.0} × {chamber_outer_y:.0} × {chamber_outer_z:.0} mm (3 mm walls)");
    println!("   Shell cavity:          {shell_cavity_x:.0} × {shell_cavity_y:.0} × {shell_cavity_z:.0} mm");
    println!("   Shell outer:           {shell_outer_x:.0} × {shell_outer_y:.0} × {shell_outer_z:.0} mm (3 mm walls)");
    println!("   Insulation gap:        {insulation_gap:.0} mm PIR foam on all 6 sides");
    println!();
    println!(" INTERIOR VOLUME:  {chamber_volume_l:.1} L");
    println!(" SHELL SURFACE:    {shell_surface_area_m2:.3} m²  (heat-loss area)");
    println!();
    println!(" PORTS (inner chamber, chamber-centered coords)");
    println!("   Heater         8 mm Ø    @ (   0,  +{:.0},  {:.0})   bottom-rear", chamber_outer_y / 2.0, -(inner_z / 2.0) + 15.0);
    println!("   Sensor (SHT40) 10 mm Ø   @ ( +80,  +{:.0},   0)      mid-rear", chamber_outer_y / 2.0);
    println!("   CO2 sensor     25×35×10  @ ( -80,  +{:.0},   0)      rear pocket (MH-Z19B)", chamber_outer_y / 2.0 - 5.0);
    println!("   Fan            80 mm Ø   @ (+{:.0},    0,  +50)      right side", chamber_outer_x / 2.0);
    println!("   Gas inlet      6 mm Ø    @ (   0,  +230, +{:.0})     top-rear 1/8\" fitting", chamber_outer_z / 2.0);
    println!("   Rocker grommet 20 mm Ø   @ (+120, +220, -{:.0})      floor-rear", chamber_outer_z / 2.0);
    println!("   Drain          10 mm Ø   @ (-270, +230, -{:.0})      back-left floor corner", chamber_outer_z / 2.0);
    println!("   Top hatch      350×250   @ (   0,    0, +{:.0})      top face", chamber_outer_z / 2.0);
    println!("   Water tray     300×200×3 @ (   0, +150, -{:.0})      floor pocket", inner_z / 2.0 - 1.5);
    println!("   Drip channel   10×5 mm   around perimeter of floor → drain");
    println!("   Hatch heat ch. 10×3 mm   around top hatch opening (anti-condensation)");
    println!("   Rail slots     12×3 mm   4 slots at X = -180, -60, +60, +180");
    println!();
    println!(" PORTS (outer shell)");
    println!("   All chamber ports mirrored with +2 mm clearance for insulation sleeving");
    println!("   Leveling feet  4× M6 through-holes, 30 mm inset from corners of floor");
    println!();
    println!(" DOOR (front, left-hinged)");
    println!("   Overall        {door_x:.0} × {door_z:.0} × {door_thickness:.0} mm (5 PETG + 25 foam + 5 PETG)");
    println!("   Window         300 × 200 mm acrylic, 1 mm anti-fog heater channel around it");
    println!("   Gasket         3 × 3 mm silicone channel, full perimeter inner face");
    println!("   Latch          2× 8 mm Ø neodymium magnets, right edge");
    println!("   Hinges         2× 6 mm hinge pin holes, left edge");
    println!();
    println!(" TOP HATCH (liquid-handler access)");
    println!("   Overall        360 × 260 × 15 mm");
    println!("   Gasket         3 × 3 mm silicone channel, bottom face");
    println!("   Latch          2× 8 mm Ø neodymium magnets, short ends");
    println!();
    println!(" RAIL (print 4× for 2 pairs)");
    println!("   Base           12 × {:.0} × 3 mm   (fits 12 mm slot in chamber floor)", rail_length);
    println!("   Top track      18 × {:.0} × 8 mm   with 5 × 3 mm V-groove for rack rollers", rail_length);
    println!();
    println!(" WATER TRAY");
    println!("   298 × 198 × 25 mm (2.5 mm PETG, ~1.4 L capacity for humidity)");
    println!();
    println!(" THERMAL");
    println!("   Operating:      37 °C / 5 % CO2 / ≥90 % RH");
    println!("   Insulation:     25 mm PIR foam (k ≈ 0.022 W/m·K)");
    println!("   Heat loss est:  Q = k·A·ΔT/t = 0.022 × {shell_surface_area_m2:.3} × 15 / 0.025");
    println!("                   ≈ {:.1} W steady-state @ 37 °C, 22 °C ambient", 0.022 * shell_surface_area_m2 * 15.0 / 0.025);
    println!("   Heater budget:  200 W silicone pad (>>4× margin)");
    println!();
    println!(" MATERIALS");
    println!("   Chamber:        PETG 3 mm");
    println!("   Shell:          PETG 3 mm");
    println!("   Door:           PETG 5 mm + PIR foam 25 mm + PETG 5 mm");
    println!("   Top hatch:      PETG 15 mm (single wall)");
    println!("   Water tray:     PETG 2.5 mm");
    println!("   Rails:          PETG");
    println!("   Gasket:         Silicone cord 3 mm (door + top hatch)");
    println!("   Window:         Acrylic 300×200×4 mm");
    println!();
    println!(" EXPORTED STLs");
    println!("   output/chip_incubator_v2_chamber.stl");
    println!("   output/chip_incubator_v2_shell.stl");
    println!("   output/chip_incubator_v2_door.stl");
    println!("   output/chip_incubator_v2_top_hatch.stl");
    println!("   output/chip_incubator_v2_water_tray.stl");
    println!("   output/chip_incubator_v2_rail.stl");
    println!("══════════════════════════════════════════════════════════════");
}
