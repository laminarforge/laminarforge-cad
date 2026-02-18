use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder};

// ─── Horizontal Optical Mount ───
//
// 3D-printed (PETG) optics housing that sits directly on top of the
// aluminum heating block. Provides horizontal LED-to-photodiode
// optical paths through each PCR tube for real-time turbidity
// measurement during LAMP amplification.
//
// Features:
// - 8 pairs of LED/photodiode press-fit holes on opposite side faces
// - 8 rectangular light channels connecting each LED to its photodiode
// - 8 tube pass-through holes aligned with the heating block slots
// - Light-tight partition walls between adjacent optical channels
//
// The horizontal optical path passes through the clear PCR tube walls
// at OPTICAL_CENTER_Z (5mm) above the mount bottom. The tube cap is
// above the optical path and not in the measurement zone.
//
// Dimensions: 84mm x 22mm x 12mm
// Material: PETG (opaque black recommended for light isolation)

fn main() {
    // ── Mount body ──

    let body = centered_cube("body", MOUNT_LENGTH, MOUNT_WIDTH, MOUNT_HEIGHT);

    // ── Tube pass-through holes (vertical, aligned with block slots) ──

    let first_x = first_slot_x();
    let mut tube_holes = centered_cube("th_init", 0.01, 0.01, 0.01);

    for i in 0..NUM_SLOTS {
        let x = first_x + (i as f64) * SLOT_SPACING;
        let hole = centered_cylinder(
            &format!("tube_pass_{i}"),
            TUBE_PASSTHROUGH_DIAMETER / 2.0,
            MOUNT_HEIGHT + 2.0, // through entire height
            32,
        )
        .translate(x, 0.0, 0.0);
        tube_holes = tube_holes + hole;
    }

    // ── LED holes (left side face, -Y) ──

    let optical_z = -(MOUNT_HEIGHT / 2.0) + OPTICAL_CENTER_Z;
    let mut led_holes = centered_cube("lh_init", 0.01, 0.01, 0.01);

    for i in 0..NUM_SLOTS {
        let x = first_x + (i as f64) * SLOT_SPACING;
        let hole = centered_cylinder(
            &format!("led_hole_{i}"),
            COMPONENT_HOLE_DIAMETER / 2.0,
            COMPONENT_HOLE_DEPTH,
            32,
        )
        .rotate(90.0, 0.0, 0.0) // align along Y axis
        .translate(x, -(MOUNT_WIDTH / 2.0) + COMPONENT_HOLE_DEPTH / 2.0, optical_z);
        led_holes = led_holes + hole;
    }

    // ── Photodiode holes (right side face, +Y) ──

    let mut pd_holes = centered_cube("ph_init", 0.01, 0.01, 0.01);

    for i in 0..NUM_SLOTS {
        let x = first_x + (i as f64) * SLOT_SPACING;
        let hole = centered_cylinder(
            &format!("pd_hole_{i}"),
            COMPONENT_HOLE_DIAMETER / 2.0,
            COMPONENT_HOLE_DEPTH,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, MOUNT_WIDTH / 2.0 - COMPONENT_HOLE_DEPTH / 2.0, optical_z);
        pd_holes = pd_holes + hole;
    }

    // ── Light channels (rectangular, connecting LED to photodiode through body) ──
    // Each channel spans the full Y width minus the component hole depths on each side,
    // but we cut the full width for simplicity (the component holes overlap)

    let channel_length = MOUNT_WIDTH - COMPONENT_HOLE_DEPTH * 2.0 + 2.0; // overlap into holes
    let mut channels = centered_cube("ch_init", 0.01, 0.01, 0.01);

    for i in 0..NUM_SLOTS {
        let x = first_x + (i as f64) * SLOT_SPACING;
        let channel = centered_cube(
            &format!("channel_{i}"),
            OPTICAL_CHANNEL_WIDTH,
            channel_length,
            OPTICAL_CHANNEL_HEIGHT,
        )
        .translate(x, 0.0, optical_z);
        channels = channels + channel;
    }

    // ── Light-tight partition walls ──
    // Instead of adding material, we achieve light isolation by ensuring the
    // body material between adjacent channels remains solid. The narrow channel
    // width (3mm) vs slot spacing (10mm) already provides 7mm of opaque wall
    // between channels. We add explicit vertical slots between channels to
    // create full-height baffles that block any stray light bouncing inside.
    //
    // Actually, the solid body already isolates channels. But we can reinforce
    // by cutting grooves on the outside that accept thin opaque inserts.
    // For now, the 7mm wall between 3mm channels provides sufficient isolation.
    // No additional cuts needed — the body geometry handles it.

    // ── Assemble ──

    let mount = body - tube_holes - led_holes - pd_holes - channels;

    // ── Export ──

    mount
        .write_stl("output/optical_mount.stl")
        .unwrap();

    let wall_between = SLOT_SPACING - OPTICAL_CHANNEL_WIDTH;

    println!("Exported: output/optical_mount.stl");
    println!();
    println!("── Optical Mount Specs ──");
    println!("  Body:           {MOUNT_LENGTH:.1}mm x {MOUNT_WIDTH:.1}mm x {MOUNT_HEIGHT:.1}mm");
    println!("  Tube holes:     {NUM_SLOTS}x {TUBE_PASSTHROUGH_DIAMETER:.1}mm dia (pass-through)");
    println!("  LED holes:      {NUM_SLOTS}x {COMPONENT_HOLE_DIAMETER:.1}mm dia x {COMPONENT_HOLE_DEPTH:.1}mm deep (-Y face)");
    println!("  Photodiode:     {NUM_SLOTS}x {COMPONENT_HOLE_DIAMETER:.1}mm dia x {COMPONENT_HOLE_DEPTH:.1}mm deep (+Y face)");
    println!("  Light channels: {OPTICAL_CHANNEL_WIDTH:.1}mm x {OPTICAL_CHANNEL_HEIGHT:.1}mm (horizontal)");
    println!("  Optical axis:   Z = {OPTICAL_CENTER_Z:.1}mm from mount bottom");
    println!("  Partition wall: {wall_between:.1}mm solid between channels");
    println!("  Material:       PETG (opaque black)");
}
