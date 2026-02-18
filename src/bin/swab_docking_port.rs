use laminarforge_cad::*;
use vcad::centered_cylinder;

// ─── Swab Docking Port ───
//
// 3D-printed (PETG) docking port that mounts in the still air box (SAB)
// front panel. Allows swab collectors to be inserted from outside the
// SAB without breaking the still-air seal.
//
// Features:
// - Outer flange: 25mm OD, 5mm thick ring with 2x M3 mounting holes
// - Port tube: SAB_DOCK_PORT_ID (12.3mm) ID, 15mm OD, 25mm long
// - O-ring groove on inner wall for sealing against collector tube
// - Panel cutout adapter for SAB_PANEL_THICKNESS (3mm) acrylic
//
// The collector tube (12mm OD) slides into the port (12.3mm ID) with
// the O-ring providing a light seal. This is not a pressure seal —
// just enough friction to prevent air currents.
//
// Material: PETG
// Quantity: SAB_NUM_PORTS (4) per still air box

fn main() {
    // ── Dimensions ──

    let port_id = SAB_DOCK_PORT_ID; // 12.3mm
    let port_od = 15.0; // mm
    let port_depth = SAB_DOCK_PORT_DEPTH; // 25.0mm

    let flange_od = 25.0; // mm
    let flange_thickness = 5.0; // mm

    let panel_thickness = SAB_PANEL_THICKNESS; // 3.0mm

    // O-ring groove (standard AS568 -010: 1/16" cross-section ≈ 1.6mm)
    let oring_width = 1.5; // mm (groove width)
    let oring_depth = 1.0; // mm (groove depth, radial)
    let oring_z_from_flange = 10.0; // mm (distance from flange face)

    // Mounting holes
    let mount_hole_d = 3.2; // M3 clearance
    let mount_hole_spacing = 20.0; // center-to-center, along X

    // Panel adapter: stepped shoulder so the port sits flush in the panel cutout
    let adapter_od = port_od + 2.0; // 17mm — slightly wider than port
    let adapter_depth = panel_thickness; // flush with panel

    // ── Build port tube ──

    let port_body = centered_cylinder(
        "port_body",
        port_od / 2.0,
        port_depth,
        48,
    );

    let port_bore = centered_cylinder(
        "port_bore",
        port_id / 2.0,
        port_depth + 2.0,
        48,
    );

    // ── O-ring groove ──
    // Annular groove on the inner wall of the port tube
    // Groove is at oring_z_from_flange from the flange end
    // The flange is at +Z end of the port tube

    let oring_z = port_depth / 2.0 - oring_z_from_flange;

    // The groove is cut into the inner wall: widen the bore locally
    let oring_groove = centered_cylinder(
        "oring_groove",
        (port_id / 2.0) + oring_depth,
        oring_width,
        48,
    )
    .translate(0.0, 0.0, oring_z);

    // ── Flange ──

    let flange = centered_cylinder(
        "flange",
        flange_od / 2.0,
        flange_thickness,
        48,
    )
    .translate(0.0, 0.0, port_depth / 2.0 + flange_thickness / 2.0);

    let flange_bore = centered_cylinder(
        "flange_bore",
        port_id / 2.0,
        flange_thickness + 2.0,
        48,
    )
    .translate(0.0, 0.0, port_depth / 2.0 + flange_thickness / 2.0);

    // Mounting holes through flange
    let mount_hole_1 = centered_cylinder(
        "mount_hole_1",
        mount_hole_d / 2.0,
        flange_thickness + 2.0,
        24,
    )
    .translate(
        mount_hole_spacing / 2.0,
        0.0,
        port_depth / 2.0 + flange_thickness / 2.0,
    );

    let mount_hole_2 = centered_cylinder(
        "mount_hole_2",
        mount_hole_d / 2.0,
        flange_thickness + 2.0,
        24,
    )
    .translate(
        -(mount_hole_spacing / 2.0),
        0.0,
        port_depth / 2.0 + flange_thickness / 2.0,
    );

    // ── Panel adapter shoulder ──
    // A ring that sits inside the panel cutout, preventing the port from
    // being pushed through. Located just behind the flange.

    let adapter = centered_cylinder(
        "adapter",
        adapter_od / 2.0,
        adapter_depth,
        48,
    )
    .translate(0.0, 0.0, port_depth / 2.0 - adapter_depth / 2.0);

    let adapter_bore = centered_cylinder(
        "adapter_bore",
        port_id / 2.0,
        adapter_depth + 2.0,
        48,
    )
    .translate(0.0, 0.0, port_depth / 2.0 - adapter_depth / 2.0);

    // ── Assemble ──

    let docking_port = (port_body + flange + adapter)
        - port_bore
        - flange_bore
        - adapter_bore
        - oring_groove
        - mount_hole_1
        - mount_hole_2;

    // ── Export ──

    docking_port
        .write_stl("output/swab_docking_port.stl")
        .unwrap();

    println!("Exported: output/swab_docking_port.stl");
    println!();
    println!("── Swab Docking Port Specs ──");
    println!("  Port bore:      {port_id:.1}mm ID x {port_od:.1}mm OD x {port_depth:.1}mm deep");
    println!("  Flange:         {flange_od:.1}mm OD x {flange_thickness:.1}mm thick");
    println!("  Mounting holes: 2x M3 ({mount_hole_d:.1}mm) at {mount_hole_spacing:.0}mm spacing");
    println!("  O-ring groove:  {oring_width:.1}mm wide x {oring_depth:.1}mm deep at {oring_z_from_flange:.0}mm from flange");
    println!("  Panel adapter:  {adapter_od:.1}mm OD x {adapter_depth:.1}mm (for {panel_thickness:.0}mm acrylic)");
    println!("  Collector fit:  {:.1}mm OD collector into {port_id:.1}mm bore ({:.1}mm clearance)", COLLECTOR_OD, port_id - COLLECTOR_OD);
    println!("  Quantity:       {} per SAB", SAB_NUM_PORTS);
    println!("  Material:       PETG");
}
