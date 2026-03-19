use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder};

// ─── PETG Enclosure Box ───
//
// 3D-printed (PETG) enclosure that houses the PCB trace heater +
// copper spreader + tube holder + optical mount assembly in the front
// shelf area, with an electronics bay in the rear for the ESP32-S3,
// driver board, and wiring.
//
// The PCB sits on the enclosure floor. The serpentine copper trace on
// the PCB bottom layer provides heating. A thin copper spreader plate
// sits on the PCB over the heater zone, and the tube holder sits on
// top of that.
//
// Features:
// - Hollow box with ENCLOSURE_WALL walls and ENCLOSURE_FLOOR floor
// - Front shelf/pocket sized for spreader + tube holder + optical mount
//   with POCKET_CLEARANCE on each side
// - Rear electronics bay (42mm deep) for ESP32-S3 and control board
// - 4x M3 mounting holes in floor for PCB attachment
// - USB port cutout in rear wall (12mm x 8mm)
// - Barrel jack cutout in rear wall (12mm dia)
// - 2x LED indicator holes in front wall (3mm dia)
// - Ventilation slot pattern on both side walls
//
// Material: PETG

fn main() {
    // ── Outer shell ──

    let outer = centered_cube("outer", OUTER_X, OUTER_Y, OUTER_Z);

    // Inner cavity (open top)
    let inner = centered_cube("inner", INNER_X, INNER_Y, WALL_HEIGHT)
        .translate(0.0, 0.0, ENCLOSURE_FLOOR / 2.0);

    let shell = outer - inner;

    // ── Component pocket (recessed shelf in front portion) ──
    // The shelf sits in the front half of the enclosure.
    // Pocket is sized for spreader + tube holder + mount with clearance.
    // The PCB sits on the enclosure floor (not in the pocket).

    let pocket_x = HOLDER_LENGTH + POCKET_CLEARANCE * 2.0;
    let pocket_y = HOLDER_WIDTH + POCKET_CLEARANCE * 2.0;
    let pocket_z = SPREADER_THICKNESS + HOLDER_HEIGHT + 1.0; // spreader + holder + clearance

    let shelf_y = shelf_center_y();
    let pocket_floor_z = floor_z();

    let pocket = centered_cube("pocket", pocket_x, pocket_y, pocket_z)
        .translate(0.0, shelf_y, pocket_floor_z + pocket_z / 2.0);

    // ── M3 PCB mounting holes in floor (4x, rectangular pattern) ──

    let mount_hole_d = 3.2; // M3 clearance
    let mount_spacing_x = PCB_LENGTH - 10.0; // near PCB edges
    let mount_spacing_y = PCB_WIDTH - 10.0; // near PCB edges
    let mount_depth = ENCLOSURE_FLOOR + 2.0; // through floor

    let mut mount_holes = centered_cube("mh_init", 0.01, 0.01, 0.01);

    for &dx in &[-mount_spacing_x / 2.0, mount_spacing_x / 2.0] {
        for &dy in &[-mount_spacing_y / 2.0, mount_spacing_y / 2.0] {
            let hole = centered_cylinder(
                &format!("mount_{}_{}", if dx < 0.0 { "l" } else { "r" }, if dy < 0.0 { "f" } else { "b" }),
                mount_hole_d / 2.0,
                mount_depth,
                24,
            )
            .translate(dx, dy, -(OUTER_Z / 2.0) + mount_depth / 2.0 - 1.0);
            mount_holes = mount_holes + hole;
        }
    }

    // ── USB port cutout in rear wall (+Y face) ──

    let usb_width = 12.0;
    let usb_height = 8.0;
    let usb_z = floor_z() + 10.0; // above floor level

    let usb_cutout = centered_cube("usb_cutout", usb_width, ENCLOSURE_WALL + 2.0, usb_height)
        .translate(0.0, OUTER_Y / 2.0, usb_z);

    // ── Barrel jack cutout in rear wall (+Y face) ──

    let barrel_d = 12.0;
    let barrel_z = floor_z() + 10.0;

    let barrel_cutout = centered_cylinder("barrel_jack", barrel_d / 2.0, ENCLOSURE_WALL + 2.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(-(OUTER_X / 4.0), OUTER_Y / 2.0, barrel_z);

    // ── LED indicator holes in front wall (-Y face) ──

    let led_d = 3.0;
    let led_spacing = 15.0;
    let led_z = floor_z() + 20.0;

    let led_hole_1 = centered_cylinder("led_1", led_d / 2.0, ENCLOSURE_WALL + 2.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(-(led_spacing / 2.0), -(OUTER_Y / 2.0), led_z);

    let led_hole_2 = centered_cylinder("led_2", led_d / 2.0, ENCLOSURE_WALL + 2.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(led_spacing / 2.0, -(OUTER_Y / 2.0), led_z);

    // ── Side wall ventilation slots ──
    // Slot pattern: 5 horizontal slots on each side wall for airflow

    let vent_width = 20.0;
    let vent_height = 2.0;
    let vent_spacing = 4.0;
    let num_vents = 5;

    let vent_start_z = floor_z() + 8.0;

    let mut vents = centered_cube("v_init", 0.01, 0.01, 0.01);

    for side in &[-1.0_f64, 1.0] {
        let vx = side * (OUTER_X / 2.0);
        for j in 0..num_vents {
            let vz = vent_start_z + (j as f64) * vent_spacing;
            let slot = centered_cube(
                &format!("vent_{}_{j}", if *side < 0.0 { "l" } else { "r" }),
                ENCLOSURE_WALL + 2.0,
                vent_width,
                vent_height,
            )
            .translate(vx, INNER_Y / 4.0, vz); // centered on electronics bay area
            vents = vents + slot;
        }
    }

    // ── Print brim (thin flange around base for bed adhesion) ──

    let brim_width = 5.0; // extends 5mm beyond outer walls
    let brim_thickness = 0.4; // ~2 layers at 0.2mm
    let brim = centered_cube(
        "brim",
        OUTER_X + brim_width * 2.0,
        OUTER_Y + brim_width * 2.0,
        brim_thickness,
    )
    .translate(0.0, 0.0, -(OUTER_Z / 2.0) + brim_thickness / 2.0);

    // ── Assemble ──

    let enclosure = shell
        + brim
        - pocket
        - mount_holes
        - usb_cutout
        - barrel_cutout
        - led_hole_1
        - led_hole_2
        - vents;

    // ── Export ──

    enclosure
        .write_stl("output/enclosure.stl")
        .unwrap();

    println!("Exported: output/enclosure.stl");
    println!();
    println!("── Enclosure Specs ──");
    println!("  Outer:          {OUTER_X:.1}mm x {OUTER_Y:.1}mm x {OUTER_Z:.1}mm");
    println!("  Wall:           {ENCLOSURE_WALL:.1}mm (floor: {ENCLOSURE_FLOOR:.1}mm)");
    println!("  Inner cavity:   {INNER_X:.1}mm x {INNER_Y:.1}mm x {WALL_HEIGHT:.1}mm");
    println!("  Component pocket: {pocket_x:.1}mm x {pocket_y:.1}mm x {pocket_z:.1}mm");
    println!("  Pocket clear:   {POCKET_CLEARANCE:.1}mm per side");
    println!("  Shelf depth:    {SHELF_DEPTH:.1}mm (front)");
    println!("  Electronics:    {ELECTRONICS_DEPTH:.1}mm (rear)");
    println!("  PCB mount holes: 4x M3 ({mount_spacing_x:.0}mm x {mount_spacing_y:.0}mm pattern)");
    println!("  USB cutout:     {usb_width:.0}mm x {usb_height:.0}mm (rear wall)");
    println!("  Barrel jack:    {barrel_d:.0}mm dia (rear wall)");
    println!("  LED holes:      2x {led_d:.0}mm dia (front wall, {led_spacing:.0}mm apart)");
    println!("  Vents:          {num_vents}x per side ({vent_width:.0}mm x {vent_height:.0}mm slots)");
    println!("  Material:       PETG");
}
