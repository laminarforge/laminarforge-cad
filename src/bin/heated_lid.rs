use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder, Part};

// Heated cartridge clamp lid.
//
// This is a reusable dry clamp for a sealed diagnostic cartridge. It replaces
// the PCR tube-cap heated lid with a flat compression frame, a small heated
// anti-condensation plate, and open optical windows for fluorescence reading.

fn main() {
    build_heated_clamp_plate();
    build_hinge_frame();
}

fn build_heated_clamp_plate() {
    let plate_length = CARTRIDGE_BAY_LENGTH;
    let plate_width = CARTRIDGE_BAY_WIDTH + 6.0;
    let plate_height = 6.0;

    let body = centered_cube(
        "heated_cartridge_clamp_plate_body",
        plate_length,
        plate_width,
        plate_height,
    );

    let mut cutouts = Part::empty("heated_cartridge_clamp_plate_cutouts");

    let optical_window = centered_cube(
        "fluorescence_window_clearance",
        HEATER_ZONE_LENGTH + 6.0,
        REACTION_WINDOW_WIDTH + 8.0,
        plate_height + 2.0,
    )
    .translate(0.0, REACTION_CHAMBER_CENTER_Y, 0.0);
    cutouts = cutouts + optical_window;

    let heater_recess = centered_cube("kapton_heater_recess", 70.0, 16.0, 0.5).translate(
        0.0,
        -CARTRIDGE_WIDTH / 2.0 + 9.0,
        plate_height / 2.0 - 0.2,
    );
    cutouts = cutouts + heater_recess;

    let thermistor_pocket = centered_cylinder("clamp_lid_thermistor_pocket", 1.5, 12.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, -plate_width / 2.0 + 6.0, 0.0);
    cutouts = cutouts + thermistor_pocket;

    for &mx in &[-42.0, 42.0] {
        let hinge_hole = centered_cylinder(
            format!(
                "clamp_lid_hinge_hole_{}",
                if mx < 0.0 { "left" } else { "right" }
            ),
            3.2 / 2.0,
            plate_height + 2.0,
            24,
        )
        .translate(mx, plate_width / 2.0 - 5.0, 0.0);
        cutouts = cutouts + hinge_hole;
    }

    for &mx in &[-34.0, 34.0] {
        let magnet = centered_cylinder(
            format!(
                "clamp_lid_magnet_recess_{}",
                if mx < 0.0 { "left" } else { "right" }
            ),
            6.2 / 2.0,
            3.2,
            32,
        )
        .translate(mx, -plate_width / 2.0 + 5.0, -plate_height / 2.0 + 1.5);
        cutouts = cutouts + magnet;
    }

    let clamp = body - cutouts + compression_ribs(plate_length, plate_width, plate_height);

    clamp.write_stl("output/heated_lid_plate.stl").unwrap();

    println!("Exported: output/heated_lid_plate.stl");
    println!();
    println!("-- Heated Cartridge Clamp Plate --");
    println!("  Body:             {plate_length:.0}mm x {plate_width:.0}mm x {plate_height:.0}mm");
    println!("  Material:         6061-T6 aluminum");
    println!(
        "  Optical opening:  {:.0}mm x {:.0}mm over cartridge reaction windows",
        HEATER_ZONE_LENGTH + 6.0,
        REACTION_WINDOW_WIDTH + 8.0
    );
    println!("  Clamp target:     sealed cartridge film stack, not tube caps");
    println!("  Wet path:         disposable cartridge only");
}

fn compression_ribs(plate_length: f64, plate_width: f64, plate_height: f64) -> Part {
    let rib_z = -plate_height / 2.0 - 0.8;
    let rear = centered_cube("rear_film_compression_rib", plate_length - 10.0, 2.0, 1.6).translate(
        0.0,
        CARTRIDGE_WIDTH / 2.0 - 3.0,
        rib_z,
    );
    let front = centered_cube("front_film_compression_rib", plate_length - 10.0, 2.0, 1.6)
        .translate(0.0, -CARTRIDGE_WIDTH / 2.0 + 3.0, rib_z);
    let swab_end = centered_cube("swab_port_compression_pad", 30.0, 8.0, 1.6).translate(
        -plate_length / 2.0 + 18.0,
        -plate_width / 2.0 + 14.0,
        rib_z,
    );
    rear + front + swab_end
}

fn build_hinge_frame() {
    let frame_length = 94.0;
    let frame_width = 20.0;
    let frame_height = 16.0;
    let wall_t = 3.0;

    let outer = centered_cube(
        "cartridge_clamp_hinge_outer",
        frame_length,
        frame_width,
        frame_height,
    );
    let inner = centered_cube(
        "cartridge_clamp_hinge_inner",
        frame_length - wall_t * 2.0,
        frame_width + 2.0,
        frame_height - wall_t,
    )
    .translate(0.0, 0.0, -wall_t / 2.0);

    let mut holes = Part::empty("cartridge_clamp_hinge_holes");
    for &mx in &[-42.0, 42.0] {
        let pin = centered_cylinder(
            format!(
                "cartridge_clamp_hinge_pin_{}",
                if mx < 0.0 { "left" } else { "right" }
            ),
            3.2 / 2.0,
            frame_width + 2.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(mx, 0.0, frame_height / 2.0 - 4.0);
        holes = holes + pin;
    }

    for &mx in &[-35.0, 35.0] {
        let mount = centered_cylinder(
            format!(
                "cartridge_clamp_hinge_mount_{}",
                if mx < 0.0 { "left" } else { "right" }
            ),
            3.2 / 2.0,
            frame_width + 2.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(mx, 0.0, -frame_height / 2.0 + 5.0);
        holes = holes + mount;
    }

    let frame = outer - inner - holes;
    frame.write_stl("output/heated_lid_frame.stl").unwrap();

    println!("Exported: output/heated_lid_frame.stl");
    println!(
        "  Hinge frame:      {frame_length:.0}mm x {frame_width:.0}mm x {frame_height:.0}mm PETG"
    );
}
