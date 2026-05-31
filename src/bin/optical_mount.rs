use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder, Part};

// Fluorescence reader bridge for the sealed diagnostic cartridge.
//
// The bridge is reusable and dry. It aligns excitation LEDs and detectors over
// the disposable cartridge reaction-window row and provides light baffles, wire
// routing, and clamp clearance. No PCR tubes pass through this part.

fn main() {
    let body = centered_cube(
        "fluorescence_reader_bridge_body",
        OPTICAL_MOUNT_LENGTH,
        OPTICAL_MOUNT_WIDTH,
        OPTICAL_MOUNT_HEIGHT,
    );

    let mut cutouts = Part::empty("fluorescence_reader_bridge_cutouts");

    for i in 0..NUM_SLOTS {
        let x = reaction_lane_x(i);

        let window_aperture = centered_cube(
            format!("reaction_window_aperture_{i}"),
            OPTICAL_WINDOW_APERTURE_LENGTH,
            OPTICAL_WINDOW_APERTURE_WIDTH,
            OPTICAL_MOUNT_HEIGHT + 2.0,
        )
        .translate(x, 0.0, 0.0);

        let led_y = -OPTICAL_MOUNT_WIDTH / 2.0 + OPTICAL_LED_HOLE_DEPTH / 2.0;
        let led = centered_cylinder(
            format!("blue_excitation_led_pocket_{i}"),
            OPTICAL_LED_HOLE_DIAMETER / 2.0,
            OPTICAL_LED_HOLE_DEPTH + 1.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, led_y, 1.8);

        let opt_y = OPTICAL_MOUNT_WIDTH / 2.0 - OPTICAL_OPT101_RECESS_DEPTH / 2.0;
        let opt_recess = centered_cube(
            format!("detector_recess_{i}"),
            OPTICAL_OPT101_RECESS_X,
            OPTICAL_OPT101_RECESS_DEPTH + 1.0,
            OPTICAL_OPT101_RECESS_Z,
        )
        .translate(x, opt_y, 1.8);

        let cross_aperture = centered_cylinder(
            format!("window_cross_aperture_{i}"),
            OPTICAL_APERTURE_DIAMETER / 2.0,
            OPTICAL_MOUNT_WIDTH + 2.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 0.0, 1.8);

        cutouts = cutouts + window_aperture + led + opt_recess + cross_aperture;
    }

    let wire_y = OPTICAL_MOUNT_WIDTH / 2.0 - OPTICAL_WIRE_CHANNEL_DEPTH / 2.0;
    let wire_z = -OPTICAL_MOUNT_HEIGHT / 2.0 + OPTICAL_WIRE_CHANNEL_WIDTH / 2.0;
    let wire_channel = centered_cube(
        "detector_wire_channel",
        OPTICAL_MOUNT_LENGTH - 8.0,
        OPTICAL_WIRE_CHANNEL_DEPTH + 1.0,
        OPTICAL_WIRE_CHANNEL_WIDTH,
    )
    .translate(0.0, wire_y, wire_z);
    cutouts = cutouts + wire_channel;

    for &mx in &[-BLOCK_MOUNT_HOLE_X, BLOCK_MOUNT_HOLE_X] {
        let mount_hole = centered_cylinder(
            format!(
                "reader_bridge_m3_{}",
                if mx < 0.0 { "left" } else { "right" }
            ),
            BLOCK_MOUNT_HOLE_DIAMETER / 2.0,
            OPTICAL_MOUNT_HEIGHT + 2.0,
            24,
        )
        .translate(mx, 0.0, 0.0);
        cutouts = cutouts + mount_hole;
    }

    for &ax in &[-42.0, 42.0] {
        let align_hole = centered_cylinder(
            format!(
                "reader_bridge_alignment_{}",
                if ax < 0.0 { "left" } else { "right" }
            ),
            CARTRIDGE_ALIGNMENT_HOLE_DIAMETER / 2.0,
            OPTICAL_MOUNT_HEIGHT + 2.0,
            24,
        )
        .translate(ax, -OPTICAL_MOUNT_WIDTH / 2.0 + 3.0, 0.0);
        cutouts = cutouts + align_hole;
    }

    let bridge = body - cutouts + baffle_ribs();

    bridge.write_stl("output/optical_mount.stl").unwrap();

    let detector_divider = SLOT_SPACING - OPTICAL_OPT101_RECESS_X;
    println!("Exported: output/optical_mount.stl");
    println!();
    println!("-- Cartridge Fluorescence Reader Bridge --");
    println!("  Body:              {OPTICAL_MOUNT_LENGTH:.0}mm x {OPTICAL_MOUNT_WIDTH:.0}mm x {OPTICAL_MOUNT_HEIGHT:.0}mm");
    println!("  Material:          opaque black PETG or resin prototype");
    println!("  Reaction windows:  {NUM_SLOTS}x {REACTION_WINDOW_LENGTH:.0}mm x {REACTION_WINDOW_WIDTH:.0}mm cartridge windows");
    println!("  Lane pitch:        {SLOT_SPACING:.0}mm");
    println!("  LED pockets:       {NUM_SLOTS}x {OPTICAL_LED_HOLE_DIAMETER:.1}mm blue excitation");
    println!("  Detector recesses: {NUM_SLOTS}x {OPTICAL_OPT101_RECESS_X:.1}mm x {OPTICAL_OPT101_RECESS_Z:.1}mm x {OPTICAL_OPT101_RECESS_DEPTH:.1}mm");
    println!("  Detector divider:  {detector_divider:.1}mm");
    println!("  Wet path:          sealed disposable cartridge below the bridge");
}

fn baffle_ribs() -> Part {
    let mut ribs = Part::empty("reader_bridge_light_baffle_ribs");
    let rib_z = -OPTICAL_MOUNT_HEIGHT / 2.0 + 2.0;

    for i in 0..=NUM_SLOTS {
        let x = first_slot_x() - SLOT_SPACING / 2.0 + i as f64 * SLOT_SPACING;
        let rib = centered_cube(
            format!("reader_bridge_baffle_rib_{i}"),
            1.0,
            OPTICAL_MOUNT_WIDTH - 8.0,
            4.0,
        )
        .translate(x, 0.0, rib_z);
        ribs = ribs + rib;
    }

    ribs
}
