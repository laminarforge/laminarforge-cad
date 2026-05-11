use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder, Part};

// ─── Arm Hole Grommet (Two-Part Snap-Fit) ───
//
// 3D-printed (PETG) two-part grommet for the 90mm arm holes in the
// still air box front panel. The grommet sandwiches the 3mm acrylic
// panel between an inner ring and an outer ring.
//
// Inner ring: Inserts from inside the SAB. Has snap-fit barbs that
//   click through the panel hole and lock against the outer face.
// Outer ring: Slides over the inner ring from outside. Captures the
//   panel edge between inner flange and outer ring face.
//
// Both rings have a smooth rolled edge (chamfer) for comfort when
// reaching through the arm holes during use.
//
// Assembly: Inner ring through hole → snap barbs click → outer ring
// slides on from other side → panel is clamped between flanges.

fn main() {
    // ── Dimensions ──

    let arm_hole_id = SAB_ARM_HOLE_DIAMETER; // 90mm — inner clear opening
    let panel_thickness = SAB_PANEL_THICKNESS; // 3mm acrylic

    let ring_od = 100.0; // outer diameter of both rings
    let ring_height = 10.0; // total height of each ring

    // Flange: wider lip that sits against the panel face
    let flange_od = 106.0; // extends 3mm beyond ring OD for panel retention
    let flange_thickness = 2.0;

    // Snap-fit barbs on inner ring
    let barb_height = 3.0; // how far barb protrudes radially
    let barb_length = 5.0; // axial length of barb
    let barb_width = 8.0; // circumferential width
    let num_barbs: usize = 4; // evenly spaced

    // Chamfer/rolled edge for comfort
    let chamfer_height = 2.0;
    let chamfer_extra = 1.5; // radius increase at chamfer lip

    // ── Inner Ring ──

    // Main ring body
    let inner_body = centered_cylinder("inner_body", ring_od / 2.0, ring_height, 64);

    // Core bore (the arm opening)
    let inner_bore = centered_cylinder("inner_bore", arm_hole_id / 2.0, ring_height + 2.0, 64);

    // Flange at one end (sits against inside face of panel)
    let inner_flange = centered_cylinder("inner_flange", flange_od / 2.0, flange_thickness, 64)
        .translate(0.0, 0.0, -(ring_height / 2.0) + flange_thickness / 2.0);

    // Snap-fit barbs: small protrusions on the outer surface near the tip
    // These click past the panel hole and lock against the outer face
    let barb_radial_pos = ring_od / 2.0 + barb_height / 2.0;
    let barb_z = ring_height / 2.0 - barb_length / 2.0; // at the tip end

    let mut barbs = Part::empty("barbs");
    for i in 0..num_barbs {
        let angle = 360.0 * (i as f64) / (num_barbs as f64);

        // Each barb is a small ramp-shaped block
        // Simplified as a cube positioned on the outer surface
        let barb = centered_cube(format!("barb_{i}"), barb_width, barb_height, barb_length)
            .translate(0.0, barb_radial_pos, barb_z)
            .rotate(0.0, 0.0, angle);

        barbs = barbs + barb;
    }

    // Comfort chamfer: cone flare at the flange end for smooth arm entry
    // Wider at entry, narrows to nominal bore diameter
    let comfort_cone = Part::cone(
        "comfort_cone",
        arm_hole_id / 2.0 + chamfer_extra, // wider at entry
        arm_hole_id / 2.0,                 // narrows to nominal
        chamfer_height,
        64,
    )
    .translate(0.0, 0.0, -(ring_height / 2.0));

    let inner_ring = (inner_body + inner_flange + barbs) - inner_bore - comfort_cone;

    // ── Outer Ring ──

    // Main ring body — slides over the inner ring from the outside
    // Bore is sized to slip over the inner ring OD with slight clearance
    let outer_bore_d = ring_od + 0.4; // 0.4mm clearance around inner ring

    let outer_body = centered_cylinder(
        "outer_body",
        flange_od / 2.0, // same OD as flanges for flush look
        ring_height,
        64,
    );

    let outer_bore = centered_cylinder("outer_bore", outer_bore_d / 2.0, ring_height + 2.0, 64);

    // Inner bore for arm clearance (same as inner ring)
    let outer_arm_bore =
        centered_cylinder("outer_arm_bore", arm_hole_id / 2.0, ring_height + 2.0, 64);

    // The outer ring needs a step/ledge to capture the panel:
    // - First portion: wider bore (outer_bore_d) to slip over inner ring
    // - Panel zone: bore matches panel hole (ring_od) with panel_thickness depth
    // The panel sits between the inner flange and this ledge.

    // Snap-fit barb recesses in the outer ring bore
    // Slots that the barbs slide through and lock into
    let mut barb_slots = Part::empty("barb_slots");
    for i in 0..num_barbs {
        let angle = 360.0 * (i as f64) / (num_barbs as f64);
        let slot = centered_cube(
            format!("slot_{i}"),
            barb_width + 1.0, // 0.5mm clearance each side
            barb_height + 1.0,
            ring_height + 2.0, // full height slot
        )
        .translate(0.0, barb_radial_pos, 0.0)
        .rotate(0.0, 0.0, angle);

        barb_slots = barb_slots + slot;
    }

    // Comfort chamfer on outer ring (outside face where arm enters)
    let outer_comfort = Part::cone(
        "outer_comfort",
        arm_hole_id / 2.0 + chamfer_extra,
        arm_hole_id / 2.0,
        chamfer_height,
        64,
    )
    .translate(0.0, 0.0, ring_height / 2.0 - chamfer_height);

    let outer_ring = outer_body - outer_bore - outer_arm_bore - barb_slots - outer_comfort;

    // ── Export ──

    inner_ring
        .write_stl("output/arm_hole_grommet_inner.stl")
        .unwrap();

    outer_ring
        .write_stl("output/arm_hole_grommet_outer.stl")
        .unwrap();

    println!("Exported: output/arm_hole_grommet_inner.stl");
    println!("Exported: output/arm_hole_grommet_outer.stl");
    println!();
    println!("── Arm Hole Grommet Specs ──");
    println!("  Arm hole ID:     {arm_hole_id:.0}mm (clear opening)");
    println!("  Ring OD:         {ring_od:.0}mm (body), {flange_od:.0}mm (flange)");
    println!("  Ring height:     {ring_height:.0}mm each");
    println!("  Panel thickness: {panel_thickness:.0}mm acrylic (sandwiched)");
    println!("  Flange:          {flange_od:.0}mm OD × {flange_thickness:.0}mm thick");
    println!("  Snap barbs:      {num_barbs}× ({barb_width:.0}mm × {barb_height:.0}mm × {barb_length:.0}mm)");
    println!("  Comfort chamfer: {chamfer_height:.0}mm × {chamfer_extra:.1}mm rolled edge");
    println!("  Clearances:      0.4mm ring fit, 1.0mm barb slots");
    println!("  Quantity:        2 sets per SAB (one per arm hole)");
    println!("  Material:        PETG");
}
