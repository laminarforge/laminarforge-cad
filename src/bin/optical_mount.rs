use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder, Part};

// ─── Optical Mount for LAMP Device v1 ───
//
// 3D-printed (PETG) bar that provides the optical detection path for
// real-time LAMP turbidity measurement. Sits on top of the aluminum
// heating block, straddling the PCR tube array.
//
// The mount holds 8x blue LEDs on the front face and 8x OPT101P
// photodiode/transimpedance amplifier ICs on the back face. PCR tubes
// pass through vertical channels between the LED and detector sides.
//
// Light path (per slot):
//   LED (5mm, 470nm) → 3mm aperture → PCR tube → 3mm aperture → OPT101P window
//
// The OPT101P is a DIP-8 package (TI) with integrated photodiode and
// transimpedance amplifier. Package dimensions: 9.81mm × 6.35mm × 5.08mm.
// Oriented with pin rows along X axis (slot array direction), so the
// 9.81mm body length consumes the X spacing between slots.
// At 12mm slot spacing: 12 - 10.4mm(recess) = 1.6mm divider wall.
//
// Cross-section (Y-Z plane through one slot):
//
//     -Y (front)                          +Y (back)
//     ┌──────────┐  ┌──────────┐  ┌──────────────┐
//     │          │  │  (tube)  │  │              │
//     │  ○ LED   │──│  channel │──│  ▢ OPT101   │
//     │  5.2mm   │  │  7mm     │  │  10.4×10mm  │
//     │          │  │          │  │              │
//     └──────────┘  └──────────┘  └──────────────┘
//     LED wall 10mm  gap 8mm     detector wall 8mm
//           └───────── 26mm total ─────────┘
//
//     Bottom: wire routing channel (3mm × 2.5mm groove, full length)
//
// Features:
//   - 8× 5.2mm round press-fit holes (front face) for 470nm blue LEDs
//   - 8× rectangular recesses (back face) for OPT101P DIP-8 packages
//   - 8× vertical tube channels (open top) for PCR tube pass-through
//   - 8× 3mm horizontal apertures (light path from LED to OPT101)
//   - Light-tight divider walls between adjacent slots (1.6mm min)
//   - Wire routing channel along bottom of back face for OPT101 leads
//   - 2× M3 mounting holes at ends (align with heating block pattern)
//
// Material: PETG (opaque black recommended for light-tightness)
// Export: output/optical_mount.stl

fn main() {
    // ── Mount body ──
    let mount_length = HEATER_ZONE_LENGTH; // 98mm, matches heating block
    let body = centered_cube(
        "body",
        mount_length,
        OPTICAL_MOUNT_WIDTH,
        OPTICAL_MOUNT_HEIGHT,
    );

    let first_x = first_slot_x(); // -42mm at 12mm spacing

    let mut cutouts = Part::empty("cutouts");

    for i in 0..NUM_SLOTS {
        let slot_x = first_x + (i as f64) * SLOT_SPACING;

        // ── LED press-fit hole (front face, -Y) ──
        // 5.2mm diameter, 8mm deep from front face
        // Cylinder along Y axis (rotated 90° around X)
        let led_y = -OPTICAL_MOUNT_WIDTH / 2.0 + OPTICAL_LED_HOLE_DEPTH / 2.0;
        let led = centered_cylinder(
            format!("led_{i}"),
            OPTICAL_LED_HOLE_DIAMETER / 2.0,
            OPTICAL_LED_HOLE_DEPTH + 2.0, // +2mm to cut cleanly through front face
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(slot_x, led_y, 0.0);

        // ── OPT101P rectangular recess (back face, +Y) ──
        // 10.4mm × 10.0mm × 5mm deep from back face
        let opt_y = OPTICAL_MOUNT_WIDTH / 2.0 - OPTICAL_OPT101_RECESS_DEPTH / 2.0;
        let opt_recess = centered_cube(
            format!("opt_{i}"),
            OPTICAL_OPT101_RECESS_X,
            OPTICAL_OPT101_RECESS_DEPTH + 1.0, // +1mm to cut through back face
            OPTICAL_OPT101_RECESS_Z,
        )
        .translate(slot_x, opt_y, 0.0);

        // ── Vertical tube channel (open top) ──
        // 7mm wide gap for PCR tube insertion from above
        // Extends from 3mm above bottom (structural floor) through the top
        let tube_gap_y = -OPTICAL_MOUNT_WIDTH / 2.0 + OPTICAL_LED_WALL + OPTICAL_TUBE_GAP / 2.0;
        let channel_bottom = -OPTICAL_MOUNT_HEIGHT / 2.0 + 3.0; // 3mm bottom wall
        let channel_height = OPTICAL_MOUNT_HEIGHT - 3.0 + 1.0; // extends through top
        let channel_z = channel_bottom + channel_height / 2.0;
        let tube_channel = centered_cube(
            format!("tube_ch_{i}"),
            OPTICAL_TUBE_CHANNEL_WIDTH,
            OPTICAL_TUBE_GAP,
            channel_height,
        )
        .translate(slot_x, tube_gap_y, channel_z);

        // ── Optical aperture (horizontal through-hole) ──
        // 3mm diameter hole connecting LED pocket to OPT101 recess
        // At the tube gap, this intersects the tube channel — light
        // passes through the PCR tube here.
        let aperture = centered_cylinder(
            format!("aperture_{i}"),
            OPTICAL_APERTURE_DIAMETER / 2.0,
            OPTICAL_MOUNT_WIDTH + 2.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(slot_x, 0.0, 0.0);

        cutouts = cutouts + led + opt_recess + tube_channel + aperture;
    }

    // ── Wire routing channel ──
    // Groove along the bottom of the back face for OPT101 lead wires
    // Runs the full length of the mount (minus 4mm inset from ends)
    let wire_y = OPTICAL_MOUNT_WIDTH / 2.0 - OPTICAL_WIRE_CHANNEL_DEPTH / 2.0;
    let wire_z = -OPTICAL_MOUNT_HEIGHT / 2.0 + OPTICAL_WIRE_CHANNEL_WIDTH / 2.0;
    let wire_channel = centered_cube(
        "wire_channel",
        mount_length - 4.0,               // inset from ends
        OPTICAL_WIRE_CHANNEL_DEPTH + 1.0, // +1mm to cut through back face
        OPTICAL_WIRE_CHANNEL_WIDTH,
    )
    .translate(0.0, wire_y, wire_z);
    cutouts = cutouts + wire_channel;

    // ── Per-slot wire exit slots ──
    // Vertical slots from each OPT101 recess down to the wire channel
    // Allows DIP-8 leads to route to the common wire channel
    for i in 0..NUM_SLOTS {
        let slot_x = first_x + (i as f64) * SLOT_SPACING;
        let exit_y = OPTICAL_MOUNT_WIDTH / 2.0 - OPTICAL_WIRE_CHANNEL_DEPTH / 2.0;
        // Vertical slot from bottom of OPT101 recess to top of wire channel
        let recess_bottom_z = -OPTICAL_OPT101_RECESS_Z / 2.0;
        let channel_top_z = wire_z + OPTICAL_WIRE_CHANNEL_WIDTH / 2.0;
        let exit_height = recess_bottom_z - channel_top_z;
        if exit_height.abs() > 0.5 {
            let exit_z = (recess_bottom_z + channel_top_z) / 2.0;
            let wire_exit = centered_cube(
                format!("wire_exit_{i}"),
                2.5, // narrow slot for lead wires
                OPTICAL_WIRE_CHANNEL_DEPTH + 1.0,
                exit_height.abs() + 0.5,
            )
            .translate(slot_x, exit_y, exit_z);
            cutouts = cutouts + wire_exit;
        }
    }

    // ── M3 mounting holes ──
    // Through-holes at the ends, aligned with heating block mount pattern
    // Uses the X positions from BLOCK_MOUNT_HOLE_X
    for &mx in &[-BLOCK_MOUNT_HOLE_X, BLOCK_MOUNT_HOLE_X] {
        let mount_hole = centered_cylinder(
            format!("mount_{}", if mx < 0.0 { "l" } else { "r" }),
            BLOCK_MOUNT_HOLE_DIAMETER / 2.0,
            OPTICAL_MOUNT_HEIGHT + 2.0,
            24,
        )
        .translate(mx, 0.0, 0.0);
        cutouts = cutouts + mount_hole;
    }

    // ── Alignment pin holes ──
    // 2x 2mm holes near center for alignment dowels to heating block
    let align_pin_d = 2.2; // 2mm pin + clearance
    for &ax in &[-20.0, 20.0] {
        let align_y = -OPTICAL_MOUNT_WIDTH / 2.0 + 2.0; // near front face
        let align_hole = centered_cylinder(
            format!("align_{}", if ax < 0.0 { "l" } else { "r" }),
            align_pin_d / 2.0,
            OPTICAL_MOUNT_HEIGHT + 2.0,
            24,
        )
        .translate(ax, align_y, 0.0);
        cutouts = cutouts + align_hole;
    }

    // ── Assemble ──
    let part = body - cutouts;

    // ── Export ──
    part.write_stl("output/optical_mount.stl").unwrap();

    // ── Print specs ──
    let divider_wall = SLOT_SPACING - OPTICAL_OPT101_RECESS_X;
    let led_divider = SLOT_SPACING - OPTICAL_LED_HOLE_DIAMETER;

    println!("Exported: output/optical_mount.stl");
    println!();
    println!("── Optical Mount Specs ──");
    println!("  Body:              {mount_length:.0}mm x {OPTICAL_MOUNT_WIDTH:.0}mm x {OPTICAL_MOUNT_HEIGHT:.0}mm");
    println!("  Material:          PETG (opaque black recommended)");
    println!("  Slot spacing:      {SLOT_SPACING:.0}mm center-to-center");
    println!();
    println!("  LED holes:         {NUM_SLOTS}x {OPTICAL_LED_HOLE_DIAMETER:.1}mm dia, {OPTICAL_LED_HOLE_DEPTH:.0}mm deep (front face)");
    println!("  LED divider wall:  {led_divider:.1}mm between adjacent LEDs");
    println!();
    println!("  OPT101 recesses:   {NUM_SLOTS}x {OPTICAL_OPT101_RECESS_X:.1}mm x {OPTICAL_OPT101_RECESS_Z:.0}mm x {OPTICAL_OPT101_RECESS_DEPTH:.0}mm deep (back face)");
    println!("  OPT101 divider:    {divider_wall:.1}mm between adjacent recesses");
    println!();
    println!("  Tube channels:     {NUM_SLOTS}x {OPTICAL_TUBE_CHANNEL_WIDTH:.0}mm wide (open top)");
    println!(
        "  Optical aperture:  {OPTICAL_APERTURE_DIAMETER:.0}mm dia (LED to OPT101 through tube)"
    );
    println!("  Wire channel:      {OPTICAL_WIRE_CHANNEL_WIDTH:.0}mm x {OPTICAL_WIRE_CHANNEL_DEPTH:.1}mm groove (bottom back face)");
    println!("  Mounting holes:    2x M3 ({BLOCK_MOUNT_HOLE_DIAMETER:.1}mm) at +/-{BLOCK_MOUNT_HOLE_X:.0}mm");
    println!("  Alignment pins:    2x {align_pin_d:.1}mm holes at +/-20mm");
    println!();
    println!("── Optical Path ──");
    println!("  LED (470nm) -> {OPTICAL_APERTURE_DIAMETER:.0}mm aperture -> PCR tube -> {OPTICAL_APERTURE_DIAMETER:.0}mm aperture -> OPT101P window");
    println!(
        "  Front wall:  {OPTICAL_LED_WALL:.0}mm (LED {OPTICAL_LED_HOLE_DEPTH:.0}mm + wall 2mm)"
    );
    println!("  Tube gap:    {OPTICAL_TUBE_GAP:.0}mm (tube OD {TUBE_OD:.1}mm + clearance)");
    println!("  Detector wall: {OPTICAL_DETECTOR_WALL:.0}mm (OPT101 {OPTICAL_OPT101_RECESS_DEPTH:.0}mm + wall 3mm)");
    println!();
    println!("── Design Notes ──");
    println!("  OPT101P DIP-8 oriented with pin rows along X axis.");
    println!("  Slot spacing increased from 10mm to 12mm to accommodate 9.81mm DIP-8 body.");
    println!("  Divider walls between OPT101 recesses: {divider_wall:.1}mm (printable in PETG).");
    println!("  Tube channels open from top for PCR tube insertion.");
    println!("  Wire channel routes DIP-8 leads along bottom of back face.");
    println!("  Use opaque black PETG for light-tightness.");
}
