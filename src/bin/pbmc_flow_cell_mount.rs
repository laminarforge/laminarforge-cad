use vcad::{centered_cube, centered_cylinder};

// ─── PBMC Isolation Flow Cell Mount ───
//
// 3D-printed (PETG) mount that holds a borosilicate glass flow
// cell tube between an LED and photodetector (OPT101P) for
// automated density gradient layer detection.
//
// The glass tube (30-50mm long, 2-3mm ID, ~6mm OD) sits
// horizontally in a light-tight channel. LED on one side,
// OPT101P on the other — same optical sensing pattern as
// the LAMP device turbidity detection.
//
// Fluid flows through the glass tube via silicone tubing
// connected with barb fittings at each end.
//
// Used in automated PBMC isolation: reads optical density
// as blood layers drain through after Ficoll centrifugation.
// Detects: RBC (opaque) → Ficoll (clear) → buffy coat (cloudy)
// → plasma (clear/yellow).

fn main() {
    // ── Dimensions ──

    // Glass flow cell tube
    let glass_od = 6.0; // borosilicate tube outer diameter
    let glass_clearance = 0.3;
    let glass_bore_d = glass_od + glass_clearance; // 6.3mm
    let glass_length = 40.0; // usable glass tube length
    let _optical_window = 10.0; // clear section for LED/detector

    // Mount body — wraps around glass tube with LED and detector pockets
    let mount_length = glass_length + 20.0; // 60mm (glass + barb space)
    let mount_width = 30.0; // Y — LED on one side, detector on other
    let mount_height = 20.0; // Z

    // LED pocket (5mm blue LED, same as LAMP device)
    let led_diameter = 5.2; // press-fit
    let led_depth = 8.0; // LED body sits in pocket

    // OPT101P detector pocket (DIP-8 package: ~5mm wide, ~4mm tall, ~6mm deep body)
    let opt101_width = 5.5; // X
    let opt101_height = 4.5; // Z
    let opt101_depth = 7.0; // into Y face
    let opt101_aperture_d = 3.0; // light entry hole

    // Optical channel between LED and detector (through glass tube area)
    let optical_channel_w = 3.0;
    let optical_channel_h = 3.0;

    // Silicone tubing barb ports at each end
    let barb_port_d = 4.0; // fits standard 3.2mm OD silicone with barb
    let barb_port_depth = 10.0;

    // Mounting tabs
    let tab_width = 10.0;
    let tab_thickness = 3.0;
    let tab_hole_d = 3.2; // M3 clearance

    // ── Build mount body ──

    let body = centered_cube("body", mount_length, mount_width, mount_height);

    // ── Glass tube bore (horizontal, along X) ──

    let glass_bore = centered_cylinder(
        "glass_bore",
        glass_bore_d / 2.0,
        mount_length + 2.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0); // along X axis

    // ── LED pocket (on -Y face, centered at optical window) ──

    let led_pocket = centered_cylinder(
        "led_pocket",
        led_diameter / 2.0,
        led_depth + 0.1,
        24,
    )
    .rotate(90.0, 0.0, 0.0) // along Y axis
    .translate(0.0, -(mount_width / 2.0) + led_depth / 2.0, 0.0);

    // ── OPT101P detector pocket (on +Y face) ──

    let detector_pocket = centered_cube(
        "detector_pocket",
        opt101_width,
        opt101_depth + 0.1,
        opt101_height,
    )
    .translate(0.0, mount_width / 2.0 - opt101_depth / 2.0, 0.0);

    // Detector aperture (light entry hole from glass tube to detector)
    let detector_aperture = centered_cylinder(
        "detector_aperture",
        opt101_aperture_d / 2.0,
        mount_width,
        24,
    )
    .rotate(90.0, 0.0, 0.0); // along Y

    // ── Optical channel (connects LED to glass tube to detector) ──

    let optical_channel = centered_cube(
        "optical_channel",
        optical_channel_w,
        mount_width + 2.0,
        optical_channel_h,
    );

    // ── Barb ports at each end (for silicone tubing connection) ──

    let barb_left = centered_cylinder(
        "barb_left",
        barb_port_d / 2.0,
        barb_port_depth + 1.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(-(mount_length / 2.0) + barb_port_depth / 2.0, 0.0, 0.0);

    let barb_right = centered_cylinder(
        "barb_right",
        barb_port_d / 2.0,
        barb_port_depth + 1.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(mount_length / 2.0 - barb_port_depth / 2.0, 0.0, 0.0);

    // ── Mounting tabs (extending from bottom, 2x with M3 holes) ──

    let tab_offset_x = mount_length / 2.0 - 10.0;
    let tab_z = -(mount_height / 2.0) - tab_thickness / 2.0;

    let tab_1 = centered_cube("tab_1", tab_width, mount_width, tab_thickness)
        .translate(tab_offset_x, 0.0, tab_z);
    let tab_hole_1 = centered_cylinder(
        "tab_hole_1",
        tab_hole_d / 2.0,
        tab_thickness + 2.0,
        24,
    )
    .translate(tab_offset_x, 0.0, tab_z);

    let tab_2 = centered_cube("tab_2", tab_width, mount_width, tab_thickness)
        .translate(-tab_offset_x, 0.0, tab_z);
    let tab_hole_2 = centered_cylinder(
        "tab_hole_2",
        tab_hole_d / 2.0,
        tab_thickness + 2.0,
        24,
    )
    .translate(-tab_offset_x, 0.0, tab_z);

    // ── Light-tight walls (block ambient light from optical path) ──
    // The body itself is solid around the glass tube except for the
    // optical channel — this inherently blocks ambient light.
    // Split line for assembly: add a horizontal split so the glass
    // tube can be inserted from above.

    let split_slot = centered_cube(
        "split_slot",
        mount_length + 2.0,
        mount_width + 2.0,
        0.5, // thin slot at glass tube centerline
    );

    // ── Assemble ──

    let mount = (body + tab_1 + tab_2)
        - glass_bore
        - led_pocket
        - detector_pocket
        - detector_aperture
        - optical_channel
        - barb_left
        - barb_right
        - tab_hole_1
        - tab_hole_2
        - split_slot;

    // ── Export ──

    mount
        .write_stl("output/pbmc_flow_cell_mount.stl")
        .unwrap();

    println!("Exported: output/pbmc_flow_cell_mount.stl");
    println!();
    println!("── PBMC Flow Cell Mount Specs ──");
    println!("  Body:           {mount_length:.0}mm x {mount_width:.0}mm x {mount_height:.0}mm");
    println!("  Glass bore:     {glass_bore_d:.1}mm dia (fits {glass_od:.0}mm OD borosilicate)");
    println!("  Glass length:   {glass_length:.0}mm");
    println!("  LED pocket:     {led_diameter:.1}mm dia x {led_depth:.0}mm deep (5mm blue LED)");
    println!("  Detector:       OPT101P pocket {opt101_width:.1}mm x {opt101_height:.1}mm x {opt101_depth:.0}mm");
    println!("  Barb ports:     2x {barb_port_d:.0}mm dia x {barb_port_depth:.0}mm deep");
    println!("  Mounting:       2x M3 tabs (bottom)");
    println!("  Split line:     horizontal at glass centerline (2-piece for tube insertion)");
    println!("  Material:       PETG (opaque, light-tight)");
}
