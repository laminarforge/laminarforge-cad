use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder};

// ─── Syringe Pump Cradle ───
//
// 3D-printed (PETG) cradle that holds a 1mL disposable syringe
// in a vertical orientation. The barrel is clamped in a half-round
// channel with the flange resting on a shelf. A NEMA17 stepper
// with T8 lead screw drives the plunger pusher above.
//
// The cradle has:
// - Half-round channel sized for syringe barrel OD
// - Flange shelf (syringe rests on its finger flanges)
// - Open top for plunger pusher access
// - Luer tip exits through bottom hole
// - Mounting tabs for frame attachment (bolts go front-to-back into back wall)
// - Guide rod bore (press-fit, holds bottom of anti-rotation rod)

fn main() {
    // ── Dimensions ──

    let barrel_od = SYRINGE_BARREL_OD; // 6.5mm
    let barrel_len = SYRINGE_BARREL_LENGTH; // 65mm
    let flange_w = SYRINGE_FLANGE_WIDTH; // 14mm
    let flange_t = SYRINGE_FLANGE_THICKNESS; // 1.5mm

    let clearance = 0.3; // fit clearance

    // Cradle body
    let body_width = flange_w + 6.0; // 20mm — wide enough for flange + walls
    let body_depth = barrel_od + 6.0; // 12.5mm — barrel + walls
    let body_height = barrel_len + flange_t + 5.0; // ~71.5mm

    let wall = 3.0;

    // Barrel channel (semicircular + rectangular)
    let channel_d = barrel_od + clearance; // 6.8mm
    let channel_depth = body_height - 5.0; // almost full height, open at top

    // Flange shelf: a ledge that the syringe flanges rest on
    // Located near the bottom of the cradle
    let shelf_z = -(body_height / 2.0) + wall + flange_t / 2.0;

    // Luer tip exit hole at bottom
    let tip_hole_d = 5.0; // clearance for Luer tip

    // Mounting tabs (bolts go front-to-back through Y-axis into frame back wall)
    let tab_width = 8.0;
    let tab_height = body_height;
    let tab_thickness = 4.0;
    let tab_hole_d = NEMA17_HOLE_DIAMETER; // M3
    let tab_hole_half_spacing_z = 20.0; // ±20mm from center = 40mm apart

    // Guide rod bore (press-fit, at top of cradle body)
    let guide_rod_d = SYRINGE_GUIDE_ROD_DIAMETER; // 4mm
    let guide_bore_d = guide_rod_d + 0.1; // tight press-fit
    let guide_offset = SYRINGE_GUIDE_ROD_OFFSET; // 12mm from lead screw center
    let guide_bore_depth = 10.0; // press-fit depth into cradle top

    // ── Build cradle body ──

    let body = centered_cube("body", body_width, body_depth, body_height);

    // Barrel channel: vertical cylinder cut
    let barrel_channel = centered_cylinder(
        "barrel_channel",
        channel_d / 2.0,
        channel_depth + 1.0,
        32,
    )
    .translate(0.0, 0.0, (body_height - channel_depth) / 2.0);

    // Open the front face for syringe insertion (half-round → clip-in)
    let front_opening = centered_cube(
        "front_opening",
        channel_d - 1.0, // slightly narrower than barrel for snap-fit retention
        body_depth / 2.0 + 1.0,
        channel_depth + 1.0,
    )
    .translate(0.0, -(body_depth / 4.0 + 0.5), (body_height - channel_depth) / 2.0);

    // Flange slot: wider rectangular cut at shelf height for flanges to rest
    let flange_slot = centered_cube(
        "flange_slot",
        flange_w + clearance,
        body_depth + 2.0,
        flange_t + clearance,
    )
    .translate(0.0, 0.0, shelf_z);

    // Luer tip exit hole through bottom
    let tip_hole = centered_cylinder(
        "tip_hole",
        tip_hole_d / 2.0,
        wall + 2.0,
        24,
    )
    .translate(0.0, 0.0, -(body_height / 2.0));

    // Guide rod bore (from top, press-fit)
    let guide_bore = centered_cylinder(
        "guide_bore",
        guide_bore_d / 2.0,
        guide_bore_depth,
        24,
    )
    .translate(guide_offset, 0.0, body_height / 2.0 - guide_bore_depth / 2.0);

    // ── Mounting tabs ──

    let left_tab = centered_cube(
        "left_tab",
        tab_thickness,
        tab_width,
        tab_height,
    )
    .translate(-(body_width / 2.0 + tab_thickness / 2.0), 0.0, 0.0);

    // Tab holes go front-to-back (Y-axis) to bolt into frame back wall
    let left_tab_hole_top = centered_cylinder(
        "left_tab_hole_top",
        tab_hole_d / 2.0,
        tab_thickness + 2.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-(body_width / 2.0 + tab_thickness / 2.0), 0.0, tab_hole_half_spacing_z);

    let left_tab_hole_bot = centered_cylinder(
        "left_tab_hole_bot",
        tab_hole_d / 2.0,
        tab_thickness + 2.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-(body_width / 2.0 + tab_thickness / 2.0), 0.0, -tab_hole_half_spacing_z);

    let right_tab = centered_cube(
        "right_tab",
        tab_thickness,
        tab_width,
        tab_height,
    )
    .translate(body_width / 2.0 + tab_thickness / 2.0, 0.0, 0.0);

    let right_tab_hole_top = centered_cylinder(
        "right_tab_hole_top",
        tab_hole_d / 2.0,
        tab_thickness + 2.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(body_width / 2.0 + tab_thickness / 2.0, 0.0, tab_hole_half_spacing_z);

    let right_tab_hole_bot = centered_cylinder(
        "right_tab_hole_bot",
        tab_hole_d / 2.0,
        tab_thickness + 2.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(body_width / 2.0 + tab_thickness / 2.0, 0.0, -tab_hole_half_spacing_z);

    // ── Assemble ──

    let cradle = (body + left_tab + right_tab)
        - barrel_channel
        - front_opening
        - flange_slot
        - tip_hole
        - guide_bore
        - left_tab_hole_top
        - left_tab_hole_bot
        - right_tab_hole_top
        - right_tab_hole_bot;

    // ── Export ──

    cradle
        .write_stl("output/syringe_pump_cradle.stl")
        .unwrap();

    println!("Exported: output/syringe_pump_cradle.stl");
    println!();
    println!("── Syringe Pump Cradle Specs ──");
    println!("  Body:            {body_width:.1}mm × {body_depth:.1}mm × {body_height:.1}mm");
    println!("  Barrel channel:  {channel_d:.1}mm dia (for {barrel_od:.1}mm barrel)");
    println!("  Flange shelf:    {flange_w:.1}mm wide × {flange_t:.1}mm slot");
    println!("  Tip exit hole:   {tip_hole_d:.1}mm dia");
    println!("  Guide rod bore:  {guide_bore_d:.1}mm dia × {guide_bore_depth:.0}mm deep (press-fit for {guide_rod_d:.0}mm rod)");
    println!("  Mounting:        4× M3 holes on side tabs, {:.0}mm apart (front-to-back into back wall)", tab_hole_half_spacing_z * 2.0);
    println!("  Fit:             snap-in from front, flange rests on shelf");
    println!("  Material:        PETG");
}
