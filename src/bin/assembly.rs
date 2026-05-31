use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder, Part};

// Full device assembly visualization.
//
// Shows the sealed disposable diagnostic cartridge architecture:
// enclosure, PCB, aluminum heat platen, cartridge, optical reader bridge, and
// lid/clamp. This visualization is not a print-ready merged part.

fn main() {
    let enclosure = build_enclosure();

    let pcb_z = floor_z() + PCB_THICKNESS / 2.0;
    let pcb = build_pcb().translate(0.0, 0.0, pcb_z);

    let platen_z = pcb_z + PCB_THICKNESS / 2.0 + BLOCK_HEIGHT / 2.0;
    let platen = build_heat_platen().translate(0.0, shelf_center_y(), platen_z);

    let cartridge_z = platen_z + BLOCK_HEIGHT / 2.0 + CARTRIDGE_BODY_HEIGHT / 2.0 + 0.6;
    let cartridge = build_cartridge().translate(0.0, shelf_center_y(), cartridge_z);

    let bridge_z = cartridge_z + CARTRIDGE_BODY_HEIGHT / 2.0 + OPTICAL_MOUNT_HEIGHT / 2.0 + 1.5;
    let bridge = build_reader_bridge().translate(
        0.0,
        shelf_center_y() + REACTION_CHAMBER_CENTER_Y,
        bridge_z,
    );

    let lid_z = OUTER_Z / 2.0 + LID_THICKNESS / 2.0;
    let lid = build_lid().translate(0.0, 0.0, lid_z);

    let assembly = enclosure + pcb + platen + cartridge + bridge + lid;
    assembly.write_stl("output/assembly.stl").unwrap();

    println!("Exported: output/assembly.stl");
    println!();
    println!("-- Sealed Cartridge Device Assembly --");
    println!("  Enclosure:      {OUTER_X:.0}mm x {OUTER_Y:.0}mm x {OUTER_Z:.0}mm");
    println!("  PCB:            {PCB_LENGTH:.0}mm x {PCB_WIDTH:.0}mm x {PCB_THICKNESS:.1}mm");
    println!("  Heat platen:    {BLOCK_LENGTH:.0}mm x {BLOCK_WIDTH:.0}mm x {BLOCK_HEIGHT:.0}mm");
    println!("  Cartridge:      {CARTRIDGE_LENGTH:.0}mm x {CARTRIDGE_WIDTH:.0}mm x {CARTRIDGE_BODY_HEIGHT:.0}mm");
    println!("  Reader bridge:  {OPTICAL_MOUNT_LENGTH:.0}mm x {OPTICAL_MOUNT_WIDTH:.0}mm x {OPTICAL_MOUNT_HEIGHT:.0}mm");
    println!(
        "  Reaction lanes: {NUM_SLOTS} sealed fluorescence windows at {SLOT_SPACING:.0}mm pitch"
    );
    println!("  Wet path:       disposable cartridge only");
}

fn build_enclosure() -> Part {
    let outer = centered_cube("assembly_enclosure_outer", OUTER_X, OUTER_Y, OUTER_Z);
    let inner = centered_cube("assembly_enclosure_inner", INNER_X, INNER_Y, WALL_HEIGHT).translate(
        0.0,
        0.0,
        ENCLOSURE_FLOOR / 2.0,
    );
    outer - inner
}

fn build_pcb() -> Part {
    centered_cube("assembly_pcb", PCB_LENGTH, PCB_WIDTH, PCB_THICKNESS)
}

fn build_heat_platen() -> Part {
    let body = centered_cube(
        "assembly_heat_platen",
        BLOCK_LENGTH,
        BLOCK_WIDTH,
        BLOCK_HEIGHT,
    );
    let pocket = centered_cube(
        "assembly_platen_cartridge_pocket",
        CARTRIDGE_LENGTH + CARTRIDGE_CLEARANCE_X,
        CARTRIDGE_WIDTH + CARTRIDGE_CLEARANCE_Y,
        BLOCK_CARTRIDGE_POCKET_DEPTH + 0.2,
    )
    .translate(
        0.0,
        0.0,
        BLOCK_HEIGHT / 2.0 - BLOCK_CARTRIDGE_POCKET_DEPTH / 2.0 + 0.1,
    );

    body - pocket
}

fn build_cartridge() -> Part {
    let body = centered_cube(
        "assembly_diagnostic_cartridge_body",
        CARTRIDGE_LENGTH,
        CARTRIDGE_WIDTH,
        CARTRIDGE_BODY_HEIGHT,
    );
    let mut windows = Part::empty("assembly_cartridge_windows");
    for i in 0..NUM_SLOTS {
        let window = centered_cube(
            format!("assembly_reaction_window_{i}"),
            REACTION_WINDOW_LENGTH,
            REACTION_WINDOW_WIDTH,
            CARTRIDGE_TOP_FILM_THICKNESS + 0.2,
        )
        .translate(
            reaction_lane_x(i),
            REACTION_CHAMBER_CENTER_Y,
            CARTRIDGE_BODY_HEIGHT / 2.0 + 0.2,
        );
        windows = windows + window;
    }

    let swab_port = centered_cylinder(
        "assembly_swab_dock_proxy",
        CARTRIDGE_SWAB_PORT_DIAMETER / 2.0,
        18.0,
        40,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(-CARTRIDGE_LENGTH / 2.0 + 10.0, CARTRIDGE_SWAB_CENTER_Y, 0.0);

    body + windows + swab_port
}

fn build_reader_bridge() -> Part {
    let bridge = centered_cube(
        "assembly_reader_bridge",
        OPTICAL_MOUNT_LENGTH,
        OPTICAL_MOUNT_WIDTH,
        OPTICAL_MOUNT_HEIGHT,
    );
    let mut apertures = Part::empty("assembly_reader_apertures");
    for i in 0..NUM_SLOTS {
        let aperture = centered_cylinder(
            format!("assembly_reader_aperture_{i}"),
            OPTICAL_APERTURE_DIAMETER / 2.0,
            OPTICAL_MOUNT_HEIGHT + 2.0,
            24,
        )
        .translate(reaction_lane_x(i), 0.0, 0.0);
        apertures = apertures + aperture;
    }
    bridge - apertures
}

fn build_lid() -> Part {
    centered_cube("assembly_lid", OUTER_X, OUTER_Y, LID_THICKNESS)
}
