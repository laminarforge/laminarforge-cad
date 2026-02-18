use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder, Part};

// ─── Full Device Assembly — LAMP Device v1 + Dispensing System ───
//
// All parts positioned in their assembled configuration:
//   1. Enclosure (base tray)
//   2. Aluminum heating block (in pocket)
//   3. Optical sensor mount (on block/shelf)
//   4. Lid (on top)
//   5. PCR tubes (in slots, for visualization)
//   6. Dispensing frame (above enclosure)
//   7. Syringe cradle (simplified, on back wall)
//   8. Linear rail (X-axis, simplified)
//   9. Dispensing carriage (on rail)
//
// This is a visualization model — not for printing.
// Print individual parts from their own binaries.

fn build_enclosure() -> Part {
    let floor_z = floor_z();
    let shelf_cy = shelf_center_y();

    let outer_box = centered_cube("enc_outer", OUTER_X, OUTER_Y, OUTER_Z);
    let inner_cavity = centered_cube("enc_inner", INNER_X, INNER_Y, WALL_HEIGHT + 1.0)
        .translate(0.0, 0.0, (ENCLOSURE_FLOOR + 1.0) / 2.0);
    let shell = outer_box - inner_cavity;

    let shelf = centered_cube("enc_shelf", INNER_X, SHELF_DEPTH, BLOCK_HEIGHT)
        .translate(0.0, shelf_cy, floor_z + BLOCK_HEIGHT / 2.0);

    let pocket = centered_cube(
        "enc_pocket",
        BLOCK_LENGTH + POCKET_CLEARANCE * 2.0,
        BLOCK_WIDTH + POCKET_CLEARANCE * 2.0,
        BLOCK_HEIGHT + 1.0,
    )
    .translate(0.0, shelf_cy, floor_z + BLOCK_HEIGHT / 2.0);

    let wire_channel = centered_cube("enc_wire", 70.0, 8.0, 10.0).translate(
        0.0,
        shelf_cy + SHELF_DEPTH / 2.0,
        floor_z + 5.0,
    );

    let usb_port = centered_cube("enc_usb", 12.0, ENCLOSURE_WALL * 3.0, 7.0).translate(
        15.0,
        OUTER_Y / 2.0 - ENCLOSURE_WALL / 2.0,
        floor_z + 8.0,
    );

    let barrel_jack =
        centered_cylinder("enc_barrel", 6.5, ENCLOSURE_WALL * 3.0, 32)
            .rotate(90.0, 0.0, 0.0)
            .translate(-15.0, OUTER_Y / 2.0 - ENCLOSURE_WALL / 2.0, floor_z + 8.0);

    let mut vents = Part::empty("enc_vents");
    for i in 0..3_usize {
        let z = floor_z + 8.0 + (i as f64 * 8.0);
        let lv = centered_cube(&format!("enc_lv_{i}"), ENCLOSURE_WALL * 3.0, 25.0, 3.0)
            .translate(-(OUTER_X / 2.0) + ENCLOSURE_WALL / 2.0, shelf_cy, z);
        let rv = centered_cube(&format!("enc_rv_{i}"), ENCLOSURE_WALL * 3.0, 25.0, 3.0)
            .translate(OUTER_X / 2.0 - ENCLOSURE_WALL / 2.0, shelf_cy, z);
        vents = vents + lv + rv;
    }

    (shell + shelf) - pocket - wire_channel - usb_port - barrel_jack - vents
}

fn build_heating_block() -> Part {
    let block = centered_cube("blk_body", BLOCK_LENGTH, BLOCK_WIDTH, BLOCK_HEIGHT);

    let first_x = first_slot_x();
    let bore_z = -(BLOCK_HEIGHT / 2.0) + BORE_Z_OFFSET_FROM_BOTTOM;

    let mut holes = Part::empty("blk_holes");
    for i in 0..NUM_SLOTS {
        let x = first_x + (i as f64 * SLOT_SPACING);
        let hole = centered_cylinder(
            &format!("blk_tube_{i}"),
            TUBE_HOLE_DIAMETER / 2.0,
            TUBE_HOLE_DEPTH + 1.0,
            32,
        )
        .translate(
            x,
            0.0,
            (BLOCK_HEIGHT - TUBE_HOLE_DEPTH) / 2.0 + 0.5,
        );
        holes = holes + hole;
    }

    let heater = centered_cylinder("blk_heater", HEATER_BORE_DIAMETER / 2.0, HEATER_BORE_DEPTH, 32)
        .rotate(0.0, 90.0, 0.0)
        .translate(
            -BLOCK_LENGTH / 2.0 + HEATER_BORE_DEPTH / 2.0,
            HEATER_Y_OFFSET,
            bore_z,
        );

    let thermistor = centered_cylinder(
        "blk_therm",
        THERMISTOR_BORE_DIAMETER / 2.0,
        THERMISTOR_BORE_DEPTH,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(
        BLOCK_LENGTH / 2.0 - THERMISTOR_BORE_DEPTH / 2.0,
        THERMISTOR_Y_OFFSET,
        bore_z,
    );

    block - holes - heater - thermistor
}

fn build_optical_mount() -> Part {
    let optical_z_offset = -(MOUNT_HEIGHT / 2.0) + OPTICAL_CENTER_Z;
    let front_face_y = -MOUNT_WIDTH / 2.0;
    let back_face_y = MOUNT_WIDTH / 2.0;
    let first_x = first_slot_x();

    let body = centered_cube("mnt_body", MOUNT_LENGTH, MOUNT_WIDTH, MOUNT_HEIGHT);
    let mut cuts = Part::empty("mnt_cuts");

    for i in 0..NUM_SLOTS {
        let x = first_x + (i as f64 * SLOT_SPACING);

        let tube = centered_cylinder(
            &format!("mnt_tube_{i}"),
            TUBE_PASSTHROUGH_DIAMETER / 2.0,
            MOUNT_HEIGHT + 2.0,
            32,
        )
        .translate(x, 0.0, 0.0);

        let led = centered_cylinder(
            &format!("mnt_led_{i}"),
            COMPONENT_HOLE_DIAMETER / 2.0,
            COMPONENT_HOLE_DEPTH + 1.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            x,
            front_face_y + COMPONENT_HOLE_DEPTH / 2.0,
            optical_z_offset,
        );

        let sensor = centered_cylinder(
            &format!("mnt_sensor_{i}"),
            COMPONENT_HOLE_DIAMETER / 2.0,
            COMPONENT_HOLE_DEPTH + 1.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            x,
            back_face_y - COMPONENT_HOLE_DEPTH / 2.0,
            optical_z_offset,
        );

        let channel_y_len = (MOUNT_WIDTH / 2.0) - (TUBE_PASSTHROUGH_DIAMETER / 2.0);

        let front_ch = centered_cube(
            &format!("mnt_fch_{i}"),
            OPTICAL_CHANNEL_WIDTH,
            channel_y_len,
            OPTICAL_CHANNEL_HEIGHT,
        )
        .translate(
            x,
            -(TUBE_PASSTHROUGH_DIAMETER / 2.0) - channel_y_len / 2.0,
            optical_z_offset,
        );

        let back_ch = centered_cube(
            &format!("mnt_bch_{i}"),
            OPTICAL_CHANNEL_WIDTH,
            channel_y_len,
            OPTICAL_CHANNEL_HEIGHT,
        )
        .translate(
            x,
            (TUBE_PASSTHROUGH_DIAMETER / 2.0) + channel_y_len / 2.0,
            optical_z_offset,
        );

        cuts = cuts + tube + led + sensor + front_ch + back_ch;
    }

    body - cuts
}

fn build_lid() -> Part {
    let lip_x = INNER_X - LID_LIP_CLEARANCE * 2.0;
    let lip_y = INNER_Y - LID_LIP_CLEARANCE * 2.0;
    let shelf_cy = shelf_center_y();
    let first_x = first_slot_x();

    let top_plate = centered_cube("lid_plate", OUTER_X, OUTER_Y, LID_THICKNESS);
    let lip = centered_cube("lid_lip", lip_x, lip_y, LID_LIP_DEPTH)
        .translate(0.0, 0.0, -(LID_THICKNESS + LID_LIP_DEPTH) / 2.0);

    let lid_body = top_plate + lip;

    let mut holes = Part::empty("lid_holes");
    for i in 0..NUM_SLOTS {
        let x = first_x + (i as f64 * SLOT_SPACING);
        let hole = centered_cylinder(
            &format!("lid_tube_{i}"),
            LID_TUBE_HOLE_DIAMETER / 2.0,
            LID_THICKNESS + LID_LIP_DEPTH + 2.0,
            32,
        )
        .translate(x, shelf_cy, 0.0);
        holes = holes + hole;
    }

    let front_grip = centered_cube("lid_fg", 30.0, 8.0, LID_THICKNESS + 2.0)
        .translate(0.0, -(OUTER_Y / 2.0), 0.0);
    let back_grip = centered_cube("lid_bg", 30.0, 8.0, LID_THICKNESS + 2.0)
        .translate(0.0, OUTER_Y / 2.0, 0.0);

    lid_body - holes - front_grip - back_grip
}

fn build_tubes() -> Part {
    let tube_length = 22.0;
    let tube_wall = 0.5;
    let first_x = first_slot_x();
    let shelf_cy = shelf_center_y();

    let mut tubes = Part::empty("tubes");
    for i in 0..NUM_SLOTS {
        let x = first_x + (i as f64 * SLOT_SPACING);

        let outer = centered_cylinder(&format!("tube_out_{i}"), TUBE_OD / 2.0, tube_length, 32);
        let inner = centered_cylinder(
            &format!("tube_in_{i}"),
            (TUBE_OD / 2.0) - tube_wall,
            tube_length - 1.0,
            32,
        )
        .translate(0.0, 0.0, 0.5);
        let cap = centered_cylinder(&format!("tube_cap_{i}"), TUBE_OD / 2.0 + 0.5, 2.0, 32)
            .translate(0.0, 0.0, tube_length / 2.0 + 1.0);

        let tube = ((outer - inner) + cap).translate(x, shelf_cy, 0.0);
        tubes = tubes + tube;
    }

    tubes
}

fn build_dispensing_frame() -> Part {
    // Simplified dispensing frame (box shell above enclosure)
    let frame_w = OUTER_X + 20.0; // 120mm
    let frame_d = OUTER_Y + 4.0; // 80mm
    let frame_h = 110.0;
    let wall = 4.0;
    let back_wall = 6.0;

    let outer = centered_cube("df_outer", frame_w, frame_d, frame_h);
    let inner_w = frame_w - wall * 2.0;
    let inner_d = frame_d - wall - back_wall;
    let inner = centered_cube("df_inner", inner_w, inner_d, frame_h + 2.0)
        .translate(0.0, -(back_wall - wall) / 2.0, 0.0);

    // Open front (remove front wall)
    let front_cut = centered_cube("df_front", inner_w + 2.0, wall + 2.0, frame_h - 10.0)
        .translate(0.0, -(frame_d / 2.0), 0.0);

    // Open top
    let top_cut = centered_cube("df_top", inner_w + 2.0, inner_d + 2.0, wall + 2.0)
        .translate(0.0, -(back_wall - wall) / 2.0, frame_h / 2.0);

    outer - inner - front_cut - top_cut
}

fn build_syringe_cradle_simplified() -> Part {
    // Simplified syringe cradle (small box)
    let w = 20.0;
    let d = SYRINGE_BARREL_LENGTH * 0.6; // show partial length
    let h = 12.0;
    centered_cube("sc_body", w, d, h)
}

fn build_linear_rail_simplified() -> Part {
    // Simplified linear rail (thin rod)
    let len = OUTER_X + 10.0;
    centered_cylinder("rail", LINEAR_ROD_DIAMETER / 2.0, len, 16)
        .rotate(0.0, 90.0, 0.0)
}

fn build_carriage_simplified() -> Part {
    // Simplified dispensing carriage (small block)
    let w = 15.0;
    let d = OUTER_Y * 0.4;
    let h = 20.0;
    centered_cube("carriage", w, d, h)
}

fn main() {
    let fz = floor_z();
    let shelf_cy = shelf_center_y();

    // ── Position each part in assembly coordinates ──

    // Enclosure: at origin (reference frame)
    let enclosure = build_enclosure();

    // Heating block: sits in the pocket, bottom on enclosure floor
    let block_z = fz + BLOCK_HEIGHT / 2.0;
    let heating_block = build_heating_block().translate(0.0, shelf_cy, block_z);

    // Optical mount: sits on top of the block
    let mount_z = fz + BLOCK_HEIGHT + MOUNT_HEIGHT / 2.0;
    let optical_mount = build_optical_mount().translate(0.0, shelf_cy, mount_z);

    // Lid: sits on top of the enclosure walls
    let lid_z = OUTER_Z / 2.0 + LID_THICKNESS / 2.0;
    let lid = build_lid().translate(0.0, 0.0, lid_z);

    // Tubes: sitting in block holes
    let tube_bottom_z = fz;
    let tube_length = 22.0;
    let tube_center_z = tube_bottom_z + BLOCK_HEIGHT - TUBE_HOLE_DEPTH + tube_length / 2.0;
    let tubes = build_tubes().translate(0.0, 0.0, tube_center_z);

    // ── Dispensing System (above enclosure) ──

    // Dispensing frame: sits on top of enclosure
    let frame_z = OUTER_Z / 2.0 + 110.0 / 2.0;
    let dispensing_frame = build_dispensing_frame().translate(0.0, 0.0, frame_z);

    // Syringe cradles (2×, on back wall of dispensing frame)
    let syringe_back_y = OUTER_Y / 2.0 + 2.0; // near back wall
    let syringe_z = frame_z + 110.0 / 2.0 - 35.0;
    let syr1 = build_syringe_cradle_simplified()
        .translate(-25.0, syringe_back_y - 10.0, syringe_z);
    let syr2 = build_syringe_cradle_simplified()
        .translate(25.0, syringe_back_y - 10.0, syringe_z);

    // Linear rail (X-axis, near top of frame)
    let rail_z = frame_z + 110.0 / 2.0 - 15.0;
    let linear_rail = build_linear_rail_simplified()
        .translate(0.0, -(OUTER_Y / 2.0 - 15.0), rail_z);

    // Dispensing carriage (on rail)
    let carriage = build_carriage_simplified()
        .translate(0.0, -(OUTER_Y / 2.0 - 15.0), rail_z - 15.0);

    // ── Combine all parts ──

    let assembly = enclosure + heating_block + optical_mount + lid + tubes
        + dispensing_frame + syr1 + syr2
        + linear_rail + carriage;

    // ── Export ──

    assembly.write_stl("output/assembly.stl").unwrap();

    println!("Exported: output/assembly.stl");
    println!();
    println!("── Full Device Assembly v2 ──");
    println!("  Base unit: enclosure, heating block, optical mount, lid, 8 tubes");
    println!("  Dispensing: frame, 2× syringe cradles, rail, carriage");
    println!();
    println!(
        "  Base size:    {:.0}mm x {:.0}mm x {:.0}mm",
        OUTER_X, OUTER_Y, OUTER_Z
    );
    println!(
        "  With frame:   {:.0}mm x {:.0}mm x {:.0}mm",
        OUTER_X + 20.0,
        OUTER_Y + 4.0,
        OUTER_Z + 110.0 + LID_THICKNESS
    );
    println!();
    println!("  This is a visualization model — print individual parts from:");
    println!("    cargo run --release --bin heating_block");
    println!("    cargo run --release --bin optical_mount");
    println!("    cargo run --release --bin enclosure");
    println!("    cargo run --release --bin lid");
    println!("    cargo run --release --bin dispensing_frame");
    println!("    cargo run --release --bin syringe_pump_cradle");
    println!("    cargo run --release --bin dispensing_carriage");
    println!("    cargo run --release --bin capping_arm");
    println!("    cargo run --release --bin rail_mounts");
}
