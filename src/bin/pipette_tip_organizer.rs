use vcad::{centered_cube, centered_cylinder, Part};

// Starter cell-culture liquid-handling organizer.
//
// This is not a sterile barrier. It keeps bought pipettes, sterile tip boxes,
// serological pipette sleeves, and conical tubes staged repeatably beside a
// certified BSC or practice enclosure so handling errors are easier to spot.

fn pipette_stand() -> Part {
    let base = centered_cube("pipette_stand_base", 260.0, 95.0, 12.0);
    let back = centered_cube("pipette_stand_back", 260.0, 10.0, 120.0).translate(0.0, 42.5, 54.0);
    let shelf = centered_cube("pipette_stand_shelf", 250.0, 18.0, 8.0).translate(0.0, 22.0, 72.0);

    let mut notches = Part::empty("pipette_notches");
    let first_x = -90.0;
    for i in 0..4 {
        let x = first_x + i as f64 * 60.0;
        let handle_relief = centered_cylinder(format!("pipette_handle_relief_{i}"), 13.0, 24.0, 32)
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 22.0, 74.0);
        let neck_slot = centered_cube(format!("pipette_neck_slot_{i}"), 15.0, 28.0, 18.0)
            .translate(x, 22.0, 79.0);
        let barrel_cup = centered_cylinder(format!("pipette_barrel_cup_{i}"), 9.0, 10.0, 32)
            .translate(x, -18.0, 6.0);
        notches = notches + handle_relief + neck_slot + barrel_cup;
    }

    let label_strip = centered_cube("label_strip", 220.0, 2.0, 22.0).translate(0.0, 36.0, 34.0);
    base + back + shelf - notches - label_strip
}

fn tip_box_tray() -> Part {
    let tray = centered_cube("tip_box_tray", 305.0, 130.0, 16.0);
    let mut pockets = Part::empty("tip_box_pockets");
    for (i, x) in [-100.0, 0.0, 100.0].iter().enumerate() {
        let pocket =
            centered_cube(format!("tip_box_pocket_{i}"), 86.0, 96.0, 10.0).translate(*x, 0.0, 6.0);
        let finger_front = centered_cylinder(format!("tip_box_finger_front_{i}"), 11.0, 18.0, 24)
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, -55.0, 12.0);
        let finger_back = centered_cylinder(format!("tip_box_finger_back_{i}"), 11.0, 18.0, 24)
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 55.0, 12.0);
        pockets = pockets + pocket + finger_front + finger_back;
    }

    let front_lip =
        centered_cube("tip_tray_front_lip", 305.0, 6.0, 20.0).translate(0.0, -62.0, 10.0);
    let back_lip = centered_cube("tip_tray_back_lip", 305.0, 6.0, 20.0).translate(0.0, 62.0, 10.0);
    let left_lip =
        centered_cube("tip_tray_left_lip", 6.0, 130.0, 20.0).translate(-149.5, 0.0, 10.0);
    let right_lip =
        centered_cube("tip_tray_right_lip", 6.0, 130.0, 20.0).translate(149.5, 0.0, 10.0);

    tray + front_lip + back_lip + left_lip + right_lip - pockets
}

fn serological_sleeve_rack() -> Part {
    let base = centered_cube("sleeve_rack_base", 170.0, 70.0, 12.0);
    let back = centered_cube("sleeve_rack_back", 170.0, 8.0, 85.0).translate(0.0, 31.0, 42.5);

    let mut channels = Part::empty("sleeve_channels");
    for (i, x) in [-54.0, -18.0, 18.0, 54.0].iter().enumerate() {
        let channel = centered_cylinder(format!("sleeve_channel_{i}"), 9.0, 160.0, 32)
            .rotate(0.0, 90.0, 0.0)
            .translate(*x, -2.0, 37.0);
        let opening = centered_cube(format!("sleeve_opening_{i}"), 24.0, 80.0, 20.0)
            .translate(*x, -3.0, 48.0);
        channels = channels + channel + opening;
    }

    base + back - channels
}

fn conical_staging_block() -> Part {
    let body = centered_cube("conical_staging_block", 195.0, 82.0, 32.0);
    let mut holes = Part::empty("conical_holes");

    for (i, x) in [-65.0, -32.5, 0.0, 32.5, 65.0].iter().enumerate() {
        let tube_hole =
            centered_cylinder(format!("conical_15ml_{i}"), 9.0, 27.0, 32).translate(*x, -20.0, 5.0);
        let chamfer = Part::cone(format!("conical_chamfer_{i}"), 11.0, 9.0, 3.0, 32)
            .translate(*x, -20.0, 14.5);
        holes = holes + tube_hole + chamfer;
    }

    for (i, x) in [-45.0, 0.0, 45.0].iter().enumerate() {
        let tube_hole =
            centered_cylinder(format!("conical_50ml_{i}"), 15.5, 27.0, 40).translate(*x, 22.0, 5.0);
        let chamfer = Part::cone(format!("conical_50ml_chamfer_{i}"), 18.0, 15.5, 3.0, 40)
            .translate(*x, 22.0, 14.5);
        holes = holes + tube_hole + chamfer;
    }

    body - holes
}

fn organizer_assembly() -> Part {
    pipette_stand().translate(-190.0, 0.0, 0.0)
        + tip_box_tray().translate(120.0, -20.0, 0.0)
        + serological_sleeve_rack().translate(-185.0, 150.0, 0.0)
        + conical_staging_block().translate(135.0, 145.0, 0.0)
}

fn main() {
    let stand = pipette_stand();
    stand
        .write_stl("output/pipette_tip_organizer_pipette_stand.stl")
        .unwrap();
    println!("Exported: output/pipette_tip_organizer_pipette_stand.stl");

    let tray = tip_box_tray();
    tray.write_stl("output/pipette_tip_organizer_tip_box_tray.stl")
        .unwrap();
    println!("Exported: output/pipette_tip_organizer_tip_box_tray.stl");

    let sleeves = serological_sleeve_rack();
    sleeves
        .write_stl("output/pipette_tip_organizer_serological_sleeve_rack.stl")
        .unwrap();
    println!("Exported: output/pipette_tip_organizer_serological_sleeve_rack.stl");

    let conicals = conical_staging_block();
    conicals
        .write_stl("output/pipette_tip_organizer_conical_staging_block.stl")
        .unwrap();
    println!("Exported: output/pipette_tip_organizer_conical_staging_block.stl");

    organizer_assembly()
        .write_stl("output/pipette_tip_organizer_assembly.stl")
        .unwrap();
    println!("Exported: output/pipette_tip_organizer_assembly.stl");

    println!("Pipette/tip organizer: P20/P200/P1000 staging, three sterile tip boxes, serological sleeves, and conical tube staging.");
}
