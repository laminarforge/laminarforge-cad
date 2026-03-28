use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder, Part};

// ─── XY Carriage with Syringe Mount ───
//
// MULA-style carriage assembly for the V-slot gantry liquid handler.
// Rides on the X-axis 2040 V-slot rail via V-wheels, holds the Z+I axis
// assembly for vertical syringe motion and plunger actuation.
//
// Ticket: T-AD4764AD
// Spec: V-slot wheels for X-axis rail, holds Z+I axis assembly.
//       Z uses 3:1 geared belt. I-axis: NEMA17 linear actuator drives
//       syringe plunger. Syringe mounted vertically, needle down.
//
// Exports:
//   output/xy_carriage_plate.stl — Main carriage plate (rides on X rail)
//   output/z_carriage.stl — Z-axis carriage (holds syringe + I-axis)
//   output/syringe_clamp.stl — Hamilton 1710RN syringe holder
//
// Material: PETG
// Reference: MULA liquid handler (HardwareX 2024)

fn main() {
    build_xy_carriage_plate();
    build_z_carriage();
    build_syringe_clamp();
}

/// XY carriage plate — rides on X-axis 2040 V-slot
///
/// V-wheels in triangular arrangement grip the V-slot rail:
///   - 2 fixed wheels on one side
///   - 2 eccentric-spacer wheels on the other side (for preload adjustment)
/// GT2 belt attaches via clamping blocks on the plate.
/// Vertical mounting holes accept the Z-axis 2040 V-slot extrusion.
fn build_xy_carriage_plate() {
    // ── Plate dimensions ──
    let plate_w = 70.0; // X (along rail travel direction)
    let plate_h = 80.0; // Y (spans across the V-slot + Z-axis mounting)
    let plate_t = 6.0; // Z thickness

    // ── V-wheel positions ──
    // 4 wheels in two pairs, gripping the 20mm-wide V-slot rail
    // Fixed pair on one side, eccentric pair on the other
    let wheel_bore_d = VWHEEL_BORE + 0.2; // 5.2mm clearance for M5 shoulder bolt
    let eccentric_bore_d = ECCENTRIC_OD + 0.2; // 7.32mm for eccentric spacer

    // Wheel spacing along X axis
    let wheel_x_spacing = 40.0; // center-to-center between wheel pairs
    // Wheel offset from plate center in Y (half the V-slot width + wheel contact offset)
    let wheel_y_fixed = -(plate_h / 2.0) + 12.0; // fixed side
    let wheel_y_eccentric = -(plate_h / 2.0) + 12.0 + VSLOT_2040_W + 2.0; // eccentric side

    // Fixed wheel bores (standard M5)
    let fw1 = centered_cylinder("fw1", wheel_bore_d / 2.0, plate_t + 2.0, 24)
        .translate(-wheel_x_spacing / 2.0, wheel_y_fixed, 0.0);
    let fw2 = centered_cylinder("fw2", wheel_bore_d / 2.0, plate_t + 2.0, 24)
        .translate(wheel_x_spacing / 2.0, wheel_y_fixed, 0.0);

    // Eccentric wheel bores (larger bore for eccentric spacer)
    let ew1 = centered_cylinder("ew1", eccentric_bore_d / 2.0, plate_t + 2.0, 24)
        .translate(-wheel_x_spacing / 2.0, wheel_y_eccentric, 0.0);
    let ew2 = centered_cylinder("ew2", eccentric_bore_d / 2.0, plate_t + 2.0, 24)
        .translate(wheel_x_spacing / 2.0, wheel_y_eccentric, 0.0);

    // ── GT2 belt attachment ──
    // Belt runs along X axis. Two clamping slots where belt folds back on itself.
    let belt_slot_w = GT2_BELT_WIDTH + 1.0; // 7mm
    let belt_slot_l = 12.0; // mm length of clamping area
    let belt_slot_d = plate_t / 2.0; // partial depth slot
    let belt_y = wheel_y_fixed + VSLOT_2040_W / 2.0; // centered on V-slot

    let belt_slot_1 = centered_cube("belt_1", belt_slot_l, belt_slot_w, belt_slot_d + 0.1)
        .translate(
            -(plate_w / 2.0) + belt_slot_l / 2.0 + 3.0,
            belt_y,
            plate_t / 2.0 - belt_slot_d / 2.0,
        );
    let belt_slot_2 = centered_cube("belt_2", belt_slot_l, belt_slot_w, belt_slot_d + 0.1)
        .translate(
            plate_w / 2.0 - belt_slot_l / 2.0 - 3.0,
            belt_y,
            plate_t / 2.0 - belt_slot_d / 2.0,
        );

    // Belt clamping screw holes (M3, through plate next to belt slots)
    let belt_screw_1 = centered_cylinder("bs1", 1.6, plate_t + 2.0, 24)
        .translate(-(plate_w / 2.0) + belt_slot_l + 5.0, belt_y, 0.0);
    let belt_screw_2 = centered_cylinder("bs2", 1.6, plate_t + 2.0, 24)
        .translate(plate_w / 2.0 - belt_slot_l - 5.0, belt_y, 0.0);

    // ── Z-axis 2040 V-slot mounting holes ──
    // The Z-axis extrusion mounts vertically to the +Y side of the plate
    // 4× M5 holes in a rectangle matching 2040 T-nut positions
    let z_mount_y = plate_h / 2.0 - 15.0; // toward +Y edge
    let z_mount_x_spacing = 30.0; // along X
    let z_mount_z_offset = 0.0;

    let mut z_mount_holes = Part::empty("z_mounts");
    let z_positions = [
        (-z_mount_x_spacing / 2.0, z_mount_y - 8.0),
        (z_mount_x_spacing / 2.0, z_mount_y - 8.0),
        (-z_mount_x_spacing / 2.0, z_mount_y + 8.0),
        (z_mount_x_spacing / 2.0, z_mount_y + 8.0),
    ];

    for (i, (zx, zy)) in z_positions.iter().enumerate() {
        let hole = centered_cylinder(
            &format!("z_mount_{i}"),
            VSLOT_M5_HOLE / 2.0,
            plate_t + 2.0,
            24,
        )
        .translate(*zx, *zy, z_mount_z_offset);
        z_mount_holes = z_mount_holes + hole;
    }

    // ── Build plate ──

    let plate = centered_cube("plate", plate_w, plate_h, plate_t);

    let carriage = plate
        - fw1
        - fw2
        - ew1
        - ew2
        - belt_slot_1
        - belt_slot_2
        - belt_screw_1
        - belt_screw_2
        - z_mount_holes;

    carriage
        .write_stl("output/xy_carriage_plate.stl")
        .unwrap();

    println!("Exported: output/xy_carriage_plate.stl");
    println!();
    println!("── XY Carriage Plate ──");
    println!("  Plate:          {plate_w:.0}mm x {plate_h:.0}mm x {plate_t:.0}mm");
    println!("  V-wheels:       4x (2 fixed @ {wheel_bore_d:.1}mm bore, 2 eccentric @ {eccentric_bore_d:.1}mm)");
    println!("  Wheel spacing:  {wheel_x_spacing:.0}mm along X");
    println!("  Belt slots:     2x {belt_slot_l:.0}mm x {belt_slot_w:.0}mm (GT2 clamp)");
    println!("  Z-mount:        4x M5 for vertical 2040 V-slot");
}

/// Z-axis carriage — rides vertically on Z 2040 V-slot, holds syringe
///
/// This is the moving part of the Z axis. It uses V-wheels to ride on
/// the vertical 2040 V-slot, and provides mounting for:
///   - Syringe clamp (holds Hamilton 1710RN barrel)
///   - I-axis motor (NEMA17 + T8 lead screw for plunger drive)
///   - GT2 belt attachment for Z-axis drive
fn build_z_carriage() {
    let body_w = 50.0; // X
    let body_h = 65.0; // Y (tall for Z-axis wheel spacing)
    let body_t = 8.0; // Z thickness (thicker for rigidity)

    // ── V-wheel bores (rides on Z-axis V-slot) ──
    let wheel_bore = VWHEEL_BORE + 0.2;
    let ecc_bore = ECCENTRIC_OD + 0.2;
    let wheel_y_spacing = 40.0; // vertical spacing on Z-axis
    let wheel_x_fixed = -(body_w / 2.0) + 10.0;
    let wheel_x_ecc = -(body_w / 2.0) + 10.0 + VSLOT_2040_W + 2.0;

    let vw1 = centered_cylinder("vw1", wheel_bore / 2.0, body_t + 2.0, 24)
        .translate(wheel_x_fixed, -wheel_y_spacing / 2.0, 0.0);
    let vw2 = centered_cylinder("vw2", wheel_bore / 2.0, body_t + 2.0, 24)
        .translate(wheel_x_fixed, wheel_y_spacing / 2.0, 0.0);
    let vw3 = centered_cylinder("vw3", ecc_bore / 2.0, body_t + 2.0, 24)
        .translate(wheel_x_ecc, 0.0, 0.0);

    // ── Z-axis GT2 belt attachment ──
    let z_belt_slot = centered_cube("z_belt", 12.0, GT2_BELT_WIDTH + 1.0, body_t / 2.0 + 0.1)
        .translate(
            wheel_x_fixed + VSLOT_2040_W / 2.0,
            body_h / 2.0 - 8.0,
            body_t / 2.0 - body_t / 4.0,
        );

    // ── I-axis motor mount (NEMA17 on top face) ──
    // Motor hangs off the +X side, shaft pointing down
    let motor_plate_w = NEMA17_BODY + 6.0;
    let motor_plate_h = NEMA17_BODY + 6.0;
    let motor_plate_t = 5.0;

    let motor_plate = centered_cube(
        "motor_plate",
        motor_plate_w,
        motor_plate_h,
        motor_plate_t,
    )
    .translate(
        body_w / 2.0 + motor_plate_w / 2.0 - 5.0,
        0.0,
        body_t / 2.0 + motor_plate_t / 2.0,
    );

    // NEMA17 holes on motor plate
    let half_bolt = NEMA17_HOLE_SPACING / 2.0;
    let motor_center_x = body_w / 2.0 + motor_plate_w / 2.0 - 5.0;
    let motor_center_z = body_t / 2.0 + motor_plate_t / 2.0;

    let mut motor_holes = Part::empty("motor_holes");
    for (i, (bx, by)) in [
        (-half_bolt, -half_bolt),
        (-half_bolt, half_bolt),
        (half_bolt, -half_bolt),
        (half_bolt, half_bolt),
    ]
    .iter()
    .enumerate()
    {
        let hole = centered_cylinder(
            &format!("motor_{i}"),
            NEMA17_HOLE_DIAMETER / 2.0,
            motor_plate_t + 2.0,
            24,
        )
        .translate(motor_center_x + bx, *by, motor_center_z);
        motor_holes = motor_holes + hole;
    }

    // Boss recess
    let boss = centered_cylinder(
        "boss",
        (NEMA17_BOSS_DIAMETER + 0.5) / 2.0,
        2.5 + 0.1,
        48,
    )
    .translate(
        motor_center_x,
        0.0,
        body_t / 2.0 + 2.5 / 2.0 - 0.05,
    );

    // Shaft + lead screw through-hole
    let shaft_through = centered_cylinder("shaft", 12.0 / 2.0, motor_plate_t + body_t + 4.0, 32)
        .translate(motor_center_x, 0.0, motor_center_z);

    // ── Syringe clamp mounting holes ──
    // 2× M3 holes on the +X face for the syringe clamp
    let clamp_hole_spacing = 20.0;
    let clamp_holes = {
        let h1 = centered_cylinder("clamp_1", 1.6, body_t + 2.0, 24)
            .translate(body_w / 2.0 - 5.0, -clamp_hole_spacing / 2.0, 0.0);
        let h2 = centered_cylinder("clamp_2", 1.6, body_t + 2.0, 24)
            .translate(body_w / 2.0 - 5.0, clamp_hole_spacing / 2.0, 0.0);
        h1 + h2
    };

    // ── Guide rod bore ──
    // 4mm guide rod prevents syringe plunger pusher from rotating
    let guide_bore = centered_cylinder(
        "guide_bore",
        (SYRINGE_GUIDE_ROD_DIAMETER + 0.1) / 2.0,
        motor_plate_t + body_t + 4.0,
        32,
    )
    .translate(
        motor_center_x + SYRINGE_GUIDE_ROD_OFFSET,
        0.0,
        motor_center_z,
    );

    // ── Build Z carriage ──

    let body = centered_cube("body", body_w, body_h, body_t);

    let z_carriage = (body + motor_plate)
        - vw1
        - vw2
        - vw3
        - z_belt_slot
        - motor_holes
        - boss
        - shaft_through
        - clamp_holes
        - guide_bore;

    z_carriage.write_stl("output/z_carriage.stl").unwrap();

    println!();
    println!("Exported: output/z_carriage.stl");
    println!();
    println!("── Z Carriage ──");
    println!("  Body:           {body_w:.0}mm x {body_h:.0}mm x {body_t:.0}mm");
    println!("  V-wheels:       3x (2 fixed, 1 eccentric) on Z V-slot");
    println!("  I-axis motor:   NEMA17 plate ({motor_plate_w:.0}mm x {motor_plate_h:.0}mm)");
    println!("  Syringe clamp:  2x M3 mounting holes");
    println!("  Guide rod bore: {:.1}mm", SYRINGE_GUIDE_ROD_DIAMETER + 0.1);
}

/// Syringe clamp — holds Hamilton 1710RN barrel vertically
///
/// Two-piece clamshell clamp that grips the syringe barrel.
/// The flange rests on a ledge to prevent vertical slip.
/// Needle points down through a clearance hole.
/// Bolts to the Z carriage.
fn build_syringe_clamp() {
    // ── Syringe dimensions ──
    let barrel_od = H1710_BARREL_OD; // 6.60mm
    let barrel_clearance = 0.3; // snug fit
    let barrel_bore = barrel_od + barrel_clearance;

    let flange_w = H1710_FLANGE_WIDTH; // 14mm
    let flange_t = H1710_FLANGE_THICKNESS; // 2mm

    // ── Clamp body (single half — print 2, bolt together) ──
    let clamp_w = 20.0; // X (half-width of full clamp)
    let clamp_d = 25.0; // Y
    let clamp_h = 30.0; // Z (barrel grip length)

    // ── Barrel bore (semicircle + full bore in the split plane) ──
    let barrel_channel = centered_cylinder("barrel", barrel_bore / 2.0, clamp_h + 2.0, 32);

    // ── Flange ledge ──
    // A notch near the top that catches the syringe flange
    let flange_slot_w = flange_w + 0.5; // 14.5mm clearance
    let flange_slot = centered_cube("flange_slot", flange_slot_w, clamp_d + 0.2, flange_t + 0.5)
        .translate(0.0, 0.0, clamp_h / 2.0 - flange_t / 2.0 - 2.0);

    // ── Needle clearance hole (through bottom) ──
    let needle_hole = centered_cylinder("needle", 2.0 / 2.0, 10.0, 24)
        .translate(0.0, 0.0, -(clamp_h / 2.0) - 2.0);

    // ── Bolt holes (M3, 2× for clamping halves together) ──
    let bolt_spacing = 18.0;
    let bolt1 = centered_cylinder("bolt_1", 1.6, clamp_d + 2.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, 0.0, -bolt_spacing / 2.0 + clamp_h / 2.0 - 5.0);
    let bolt2 = centered_cylinder("bolt_2", 1.6, clamp_d + 2.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, 0.0, bolt_spacing / 2.0 + clamp_h / 2.0 - 5.0 - bolt_spacing);

    // ── Mounting holes (M3, for attaching to Z carriage) ──
    let mount_spacing = 20.0;
    let mount1 = centered_cylinder("mount_1", 1.6, clamp_d + 2.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, 0.0, -mount_spacing / 2.0);
    let mount2 = centered_cylinder("mount_2", 1.6, clamp_d + 2.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, 0.0, mount_spacing / 2.0);

    // ── Build ──

    let body = centered_cube("body", clamp_w, clamp_d, clamp_h);

    // Cut the barrel bore from the split face (-X side, so two halves make a full bore)
    let clamp = body - barrel_channel - flange_slot - needle_hole - bolt1 - bolt2 - mount1 - mount2;

    clamp.write_stl("output/syringe_clamp.stl").unwrap();

    println!();
    println!("Exported: output/syringe_clamp.stl");
    println!();
    println!("── Syringe Clamp (half, print 2×) ──");
    println!("  Body:           {clamp_w:.0}mm x {clamp_d:.0}mm x {clamp_h:.0}mm");
    println!("  Barrel bore:    {barrel_bore:.1}mm (Hamilton 1710RN, {barrel_od:.1}mm OD + {barrel_clearance:.1}mm clearance)");
    println!("  Flange slot:    {flange_slot_w:.1}mm x {:.1}mm", flange_t + 0.5);
    println!("  Clamp bolts:    2x M3 (clamshell closure)");
    println!("  Mount holes:    2x M3 (Z carriage attachment)");
    println!("  Needle hole:    2mm dia (through bottom)");
    println!("  Material:       PETG");
}
