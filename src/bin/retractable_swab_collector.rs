use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder};

// ─── Retractable Swab Collector ───
//
// Bayonet twist-lock swab transport tube. Two-part design for safe
// sample transport: the swab is held inside the inner slider, which
// locks into the outer tube via L-shaped bayonet slots.
//
// Operation:
// 1. Insert slider with swab into outer tube, aligning pins with slots
// 2. Push in (axial travel) then twist (rotational lock)
// 3. Pins engage the horizontal portion of the L-slot = locked
// 4. To eject: twist back, pull straight out
//
// Outer tube: COLLECTOR_OD (12mm) OD, 10mm ID, 160mm long
//   - 2x L-shaped bayonet slots near open end
// Inner slider: 9.8mm OD, 150mm long
//   - 2x bayonet pins that engage the slots
//
// Material: PETG or resin (autoclavable if needed)

fn main() {
    // ── Outer tube dimensions ──

    let outer_od = COLLECTOR_OD; // 12.0mm
    let outer_id = 10.0; // mm
    let outer_length = 160.0; // mm
    let outer_wall = (outer_od - outer_id) / 2.0; // 1.0mm

    // Bayonet slot dimensions
    let slot_width = 3.0; // mm (circumferential width)
    let slot_axial = 10.0; // mm (axial portion of L)
    let slot_rotational = 8.0; // mm (rotational portion of L, measured as arc length)
    let slot_depth = outer_wall + 0.5; // through the wall with clearance

    // Slot positions: 180 degrees apart, near the open end (+Z)
    let slot_z_start = outer_length / 2.0 - 5.0; // 5mm from open end

    // ── Inner slider dimensions ──

    let slider_od = 9.8; // mm (snug fit in 10mm ID)
    let slider_length = 150.0; // mm
    let slider_wall = 1.0; // mm

    // Bayonet pin dimensions
    let pin_width = 3.0; // mm (matches slot width)
    let pin_height = 2.0; // mm (radial protrusion)
    let pin_thickness = 2.0; // mm (axial thickness)

    // Pin position: near the open end of the slider
    let pin_z = slider_length / 2.0 - 8.0; // matches slot entry position

    // ── Build outer tube ──

    let outer_body = centered_cylinder(
        "outer_body",
        outer_od / 2.0,
        outer_length,
        48,
    );

    let outer_bore = centered_cylinder(
        "outer_bore",
        outer_id / 2.0,
        outer_length + 2.0,
        48,
    );

    // Bayonet slots: L-shaped cuts through the tube wall
    // Slot 1 at 0 degrees, slot 2 at 180 degrees
    // Each L-slot: vertical (axial) entry + horizontal (rotational) lock

    // Approximate the L-slot with rectangular cuts through the wall
    // Axial portion: vertical slot
    let slot1_axial = centered_cube(
        "slot1_axial",
        slot_width,
        slot_depth,
        slot_axial,
    )
    .translate(0.0, outer_od / 2.0, slot_z_start - slot_axial / 2.0);

    // Rotational portion: horizontal slot perpendicular to axial
    // Rotational arc approximated as a rectangular cut rotated around Z
    let slot1_rot = centered_cube(
        "slot1_rot",
        slot_rotational,
        slot_depth,
        slot_width,
    )
    .translate(slot_rotational / 2.0, outer_od / 2.0, slot_z_start - slot_axial);

    // Slot 2: 180 degrees opposite
    let slot2_axial = centered_cube(
        "slot2_axial",
        slot_width,
        slot_depth,
        slot_axial,
    )
    .translate(0.0, -(outer_od / 2.0), slot_z_start - slot_axial / 2.0);

    let slot2_rot = centered_cube(
        "slot2_rot",
        slot_rotational,
        slot_depth,
        slot_width,
    )
    .translate(-(slot_rotational / 2.0), -(outer_od / 2.0), slot_z_start - slot_axial);

    // Closed bottom end (the tube is open at +Z, closed at -Z by default
    // since it's a hollow cylinder — leave as is)

    let outer_tube = outer_body
        - outer_bore
        - slot1_axial
        - slot1_rot
        - slot2_axial
        - slot2_rot;

    // ── Build inner slider ──

    let slider_body = centered_cylinder(
        "slider_body",
        slider_od / 2.0,
        slider_length,
        48,
    );

    let slider_bore = centered_cylinder(
        "slider_bore",
        (slider_od - slider_wall * 2.0) / 2.0,
        slider_length + 2.0,
        48,
    );

    // Bayonet pins: small protrusions on the outside
    // Pin 1 at 0 degrees (+Y face)
    let pin1 = centered_cube(
        "pin1",
        pin_width,
        pin_height,
        pin_thickness,
    )
    .translate(0.0, slider_od / 2.0 + pin_height / 2.0, pin_z);

    // Pin 2 at 180 degrees (-Y face)
    let pin2 = centered_cube(
        "pin2",
        pin_width,
        pin_height,
        pin_thickness,
    )
    .translate(0.0, -(slider_od / 2.0 + pin_height / 2.0), pin_z);

    // Swab retention: small internal lip at the bottom to prevent swab from falling out
    let retention_ring = centered_cylinder(
        "retention_ring",
        slider_od / 2.0 - slider_wall,
        1.5,
        48,
    ) - centered_cylinder(
        "retention_hole",
        1.5, // small hole for swab shaft (2mm shaft fits through 3mm hole)
        2.0,
        24,
    );

    let retention = retention_ring.translate(0.0, 0.0, -(slider_length / 2.0) + 1.0);

    let slider = slider_body - slider_bore + pin1 + pin2 + retention;

    // ── Export ──

    outer_tube
        .write_stl("output/swab_collector_outer.stl")
        .unwrap();

    slider
        .write_stl("output/swab_collector_slider.stl")
        .unwrap();

    println!("Exported: output/swab_collector_outer.stl");
    println!("Exported: output/swab_collector_slider.stl");
    println!();
    println!("── Retractable Swab Collector Specs ──");
    println!("  Outer tube:     {outer_od:.1}mm OD x {outer_id:.1}mm ID x {outer_length:.1}mm");
    println!("  Wall thickness: {outer_wall:.1}mm");
    println!("  Bayonet slots:  2x L-shaped ({slot_axial:.0}mm axial + {slot_rotational:.0}mm rotational, {slot_width:.0}mm wide)");
    println!("  Inner slider:   {slider_od:.1}mm OD x {slider_length:.1}mm");
    println!("  Bayonet pins:   2x {pin_width:.0}mm x {pin_height:.0}mm x {pin_thickness:.0}mm");
    println!("  Operation:      push + twist to lock, twist + pull to eject");
    println!("  Material:       PETG");
}
