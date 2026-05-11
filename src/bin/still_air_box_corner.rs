use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder};

// ─── Still Air Box Corner Connector ───
//
// 3D-printed (PETG) corner connector for the still air box frame.
// 8 identical pieces form the vertices of a rectangular box frame.
//
// Each corner is a 3-way joint where three perpendicular rails meet.
// The body is a cube (SAB_RAIL_WIDTH × SAB_RAIL_WIDTH × SAB_RAIL_WIDTH).
// Three perpendicular square sockets accept rail ends (slide-in fit).
// M3 set screw holes through each socket face lock rails in place.
//
// Orientation: X = width rail, Y = depth rail, Z = vertical rail.
// Print orientation: Z-up, no supports needed.

fn main() {
    // ── Dimensions ──

    let body_size = SAB_RAIL_WIDTH; // 20mm cube

    // Socket dimensions: slightly undersized for snug fit on rail ends
    let socket_size = SAB_RAIL_WIDTH - 2.0; // 18mm square
    let socket_depth = 15.0; // how far rail slides in

    // M3 set screw holes (one per socket face, perpendicular to rail axis)
    let set_screw_d = 3.2; // M3 clearance
    let set_screw_depth = body_size; // through the body wall

    // ── Build body ──

    let body = centered_cube("body", body_size, body_size, body_size);

    // ── Socket cuts (3 perpendicular square channels) ──

    // +X socket: rail slides in from +X face
    let socket_x = centered_cube("socket_x", socket_depth + 0.1, socket_size, socket_size)
        .translate(body_size / 2.0 - socket_depth / 2.0 + 0.05, 0.0, 0.0);

    // +Y socket: rail slides in from +Y face
    let socket_y = centered_cube("socket_y", socket_size, socket_depth + 0.1, socket_size)
        .translate(0.0, body_size / 2.0 - socket_depth / 2.0 + 0.05, 0.0);

    // +Z socket: rail slides in from +Z face
    let socket_z = centered_cube("socket_z", socket_size, socket_size, socket_depth + 0.1)
        .translate(0.0, 0.0, body_size / 2.0 - socket_depth / 2.0 + 0.05);

    // ── Set screw holes ──
    // Each hole enters perpendicular to the rail axis, centered on the socket face.
    // Positioned at mid-depth of the socket so the screw contacts the rail.

    let screw_center = body_size / 2.0 - socket_depth / 2.0; // center of socket along its axis

    // Set screw for X socket: enters from +Y face, drills through Y-axis
    let screw_x = centered_cylinder("screw_x", set_screw_d / 2.0, set_screw_depth + 2.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(screw_center, 0.0, 0.0);

    // Set screw for Y socket: enters from +X face, drills through X-axis
    let screw_y = centered_cylinder("screw_y", set_screw_d / 2.0, set_screw_depth + 2.0, 24)
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, screw_center, 0.0);

    // Set screw for Z socket: enters from +X face, drills through X-axis
    let screw_z = centered_cylinder("screw_z", set_screw_d / 2.0, set_screw_depth + 2.0, 24)
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, 0.0, screw_center);

    // ── Assemble ──

    let corner = body - socket_x - socket_y - socket_z - screw_x - screw_y - screw_z;

    // ── Export ──

    corner.write_stl("output/still_air_box_corner.stl").unwrap();

    println!("Exported: output/still_air_box_corner.stl");
    println!();
    println!("── Still Air Box Corner Connector Specs ──");
    println!("  Body:          {body_size:.0}mm × {body_size:.0}mm × {body_size:.0}mm cube");
    println!("  Sockets:       3× {socket_size:.0}mm × {socket_size:.0}mm square, {socket_depth:.0}mm deep");
    println!("  Set screws:    3× M3 ({set_screw_d:.1}mm dia) through each face");
    println!("  Quantity:      8 per SAB frame");
    println!("  Rail fit:      {:.0}mm rail slides into {socket_size:.0}mm socket (1mm clearance per side)", SAB_RAIL_WIDTH);
    println!("  Material:      PETG");
}
