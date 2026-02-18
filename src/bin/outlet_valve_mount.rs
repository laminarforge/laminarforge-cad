use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder};

// ─── Outlet Pinch Valve Mount ───
//
// 3D-printed (PETG) mount that holds a short silicone tubing section
// and a solenoid or micro servo to pinch it open/closed.
//
// The silicone section sits in a groove. A plunger (driven by solenoid
// or servo cam) presses down onto the tube to close it.
// Default state: CLOSED (normally closed for fail-safe).
//
// Two mounting options:
// A) 12V solenoid pinch: spring holds pinch closed, power opens
// B) SG90 servo + cam: servo rotates cam that presses on tube

fn main() {
    // ── Dimensions ──

    let silicone_od = SILICONE_OD; // 3.2mm
    let silicone_section_len = 10.0; // short section for pinch

    // Mount body
    let body_length = 30.0; // along tube axis
    let body_width = 20.0;
    let body_height = 15.0;

    // Tube groove: channel for the silicone section
    let groove_d = silicone_od + 0.4; // 3.6mm with clearance
    let groove_depth = silicone_od / 2.0 + 0.5; // tube sits halfway in

    // Barbed adapter clearance holes
    let adapter_hole_d = 5.0; // clearance for press-fit barbed adapter

    // Pinch plunger slot: vertical slot above the tube for plunger to descend
    let plunger_slot_width = 6.0;
    let plunger_slot_length = silicone_section_len + 2.0;
    let plunger_slot_height = body_height; // through to top

    // Solenoid mounting holes (Option A)
    let solenoid_hole_spacing = 16.0;
    let solenoid_hole_d = NEMA17_HOLE_DIAMETER; // M3

    // Frame mounting
    let mount_hole_spacing = 22.0;
    let mount_hole_d = NEMA17_HOLE_DIAMETER; // M3

    // ── Build body ──

    let body = centered_cube("body", body_length, body_width, body_height);

    // ── Tube groove (horizontal channel along X-axis at bottom) ──

    let groove = centered_cylinder(
        "tube_groove",
        groove_d / 2.0,
        body_length + 2.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0) // along X axis
    .translate(0.0, 0.0, -(body_height / 2.0) + groove_depth);

    // Flatten the bottom of the groove so tube sits stable
    let groove_flat = centered_cube(
        "groove_flat",
        body_length + 2.0,
        groove_d,
        groove_depth,
    )
    .translate(0.0, 0.0, -(body_height / 2.0) + groove_depth / 2.0);

    // ── Tube entry/exit holes at each end ──

    let entry_hole = centered_cylinder(
        "entry_hole",
        adapter_hole_d / 2.0,
        8.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(-(body_length / 2.0), 0.0, -(body_height / 2.0) + groove_depth);

    let exit_hole = centered_cylinder(
        "exit_hole",
        adapter_hole_d / 2.0,
        8.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(body_length / 2.0, 0.0, -(body_height / 2.0) + groove_depth);

    // ── Plunger slot (vertical channel above pinch point) ──

    let plunger_slot = centered_cube(
        "plunger_slot",
        plunger_slot_length,
        plunger_slot_width,
        plunger_slot_height,
    )
    .translate(0.0, 0.0, groove_depth / 2.0);

    // ── Mounting holes (bottom face, for frame attachment) ──

    let mount_hole_1 = centered_cylinder(
        "mount_hole_0",
        mount_hole_d / 2.0,
        body_height + 2.0,
        24,
    )
    .translate(-(mount_hole_spacing / 2.0), 0.0, 0.0);

    let mount_hole_2 = centered_cylinder(
        "mount_hole_1",
        mount_hole_d / 2.0,
        body_height + 2.0,
        24,
    )
    .translate(mount_hole_spacing / 2.0, 0.0, 0.0);

    // ── Solenoid/servo mounting holes on top face ──
    // These are positioned to either side of the plunger slot

    let top_hole_1 = centered_cylinder(
        "top_hole_0",
        solenoid_hole_d / 2.0,
        5.0,
        24,
    )
    .translate(-(solenoid_hole_spacing / 2.0), 0.0, body_height / 2.0 - 2.0);

    let top_hole_2 = centered_cylinder(
        "top_hole_1",
        solenoid_hole_d / 2.0,
        5.0,
        24,
    )
    .translate(solenoid_hole_spacing / 2.0, 0.0, body_height / 2.0 - 2.0);

    // ── Assemble ──

    let valve_mount = body
        - groove
        - groove_flat
        - entry_hole
        - exit_hole
        - plunger_slot
        - mount_hole_1
        - mount_hole_2
        - top_hole_1
        - top_hole_2;

    // ── Export ──

    valve_mount
        .write_stl("output/outlet_valve_mount.stl")
        .unwrap();

    println!("Exported: output/outlet_valve_mount.stl");
    println!();
    println!("── Outlet Pinch Valve Mount Specs ──");
    println!("  Body:            {body_length:.1}mm × {body_width:.1}mm × {body_height:.1}mm");
    println!("  Tube groove:     {groove_d:.1}mm dia, {groove_depth:.1}mm deep");
    println!("  Silicone section: {silicone_section_len:.0}mm (pinch zone)");
    println!("  Plunger slot:    {plunger_slot_length:.1}mm × {plunger_slot_width:.1}mm");
    println!("  Adapter holes:   {adapter_hole_d:.1}mm dia (entry/exit)");
    println!("  Mounting:        2× M3 through-holes at {mount_hole_spacing:.0}mm");
    println!("  Actuator mount:  2× M3 at {solenoid_hole_spacing:.0}mm (solenoid or servo adapter)");
    println!("  Default state:   CLOSED (fail-safe)");
    println!("  Material:        PETG");
}
