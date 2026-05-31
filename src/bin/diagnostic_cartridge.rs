use std::fs;

use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder, Part};

// Printable prototype diagnostic cartridge.
//
// This is the disposable wet-path object for the cartridge-based LAMP/CRISPR
// fluorescence architecture. It is a macrofluidic prototype, not a final
// injection-molded COC/COP part. The final optical window and seals are separate
// qualified film/glass/COC components; the printed body proves fit and handling.

const OUTPUTS: &[&str] = &[
    "output/diagnostic_cartridge_body.stl",
    "output/diagnostic_cartridge_window_frame.stl",
    "output/diagnostic_cartridge_assembly.stl",
    "output/diagnostic_cartridge_design_note.txt",
];

fn main() {
    fs::create_dir_all("output").unwrap();

    let body = cartridge_body();
    export(&body, OUTPUTS[0]);

    let frame = optical_window_frame();
    export(&frame, OUTPUTS[1]);

    let assembly = body + frame.translate(0.0, 0.0, CARTRIDGE_BODY_HEIGHT / 2.0 + 0.8);
    export(&assembly, OUTPUTS[2]);

    fs::write(OUTPUTS[3], design_note()).unwrap();
    println!("Wrote: {}", OUTPUTS[3]);
    println!();
    println!("-- Diagnostic Cartridge Prototype --");
    println!("  Body:             {CARTRIDGE_LENGTH:.0}mm x {CARTRIDGE_WIDTH:.0}mm x {CARTRIDGE_BODY_HEIGHT:.0}mm");
    println!("  Reaction lanes:   {NUM_SLOTS} sealed fluorescence windows");
    println!("  Swab dock:        {CARTRIDGE_SWAB_PORT_DIAMETER:.1}mm bore into internal elution chamber");
    println!("  Material target:  printed prototype body now; COC/COP final cartridge later");
    println!("  Wet path:         disposable only");
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn cartridge_body() -> Part {
    let body = centered_cube(
        "diagnostic_cartridge_body",
        CARTRIDGE_LENGTH,
        CARTRIDGE_WIDTH,
        CARTRIDGE_BODY_HEIGHT,
    );

    let cutouts = swab_dock_cutouts()
        + reaction_chamber_cutouts()
        + macrofluidic_channel_cutouts()
        + waste_chamber_cutout()
        + alignment_holes()
        + gasket_groove();

    body - cutouts + cartridge_latch_tabs() + handling_ribs() + film_lands()
}

fn swab_dock_cutouts() -> Part {
    let bore_len = CARTRIDGE_SWAB_CHAMBER_LENGTH + 6.0;
    let bore_center_x = -CARTRIDGE_LENGTH / 2.0 + bore_len / 2.0 - 1.0;
    let swab_bore = centered_cylinder(
        "swab_insert_bore",
        CARTRIDGE_SWAB_PORT_DIAMETER / 2.0,
        bore_len,
        48,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(bore_center_x, CARTRIDGE_SWAB_CENTER_Y, 0.0);

    let elution_chamber = centered_cube(
        "swab_elution_chamber",
        CARTRIDGE_SWAB_CHAMBER_LENGTH,
        CARTRIDGE_SWAB_CHAMBER_WIDTH,
        CARTRIDGE_SWAB_CHAMBER_DEPTH,
    )
    .translate(
        -CARTRIDGE_LENGTH / 2.0 + CARTRIDGE_SWAB_CHAMBER_LENGTH / 2.0 + 4.0,
        CARTRIDGE_SWAB_CENTER_Y,
        0.6,
    );

    let inlet_mouth = centered_cube(
        "swab_port_keyed_mouth",
        6.0,
        CARTRIDGE_SWAB_PORT_DIAMETER + 4.0,
        CARTRIDGE_SWAB_PORT_DIAMETER + 2.0,
    )
    .translate(-CARTRIDGE_LENGTH / 2.0 - 0.5, CARTRIDGE_SWAB_CENTER_Y, 0.0);

    swab_bore + elution_chamber + inlet_mouth
}

fn reaction_chamber_cutouts() -> Part {
    let mut cutouts = Part::empty("reaction_chamber_cutouts");
    let top_z = CARTRIDGE_BODY_HEIGHT / 2.0;

    for i in 0..NUM_SLOTS {
        let x = reaction_lane_x(i);
        let chamber = centered_cube(
            format!("reaction_chamber_{i}"),
            REACTION_CHAMBER_LENGTH,
            REACTION_CHAMBER_WIDTH,
            REACTION_CHAMBER_DEPTH + 0.2,
        )
        .translate(
            x,
            REACTION_CHAMBER_CENTER_Y,
            top_z - REACTION_CHAMBER_DEPTH / 2.0 + 0.1,
        );

        let optical_window_recess = centered_cube(
            format!("reaction_window_recess_{i}"),
            REACTION_WINDOW_LENGTH,
            REACTION_WINDOW_WIDTH,
            CARTRIDGE_TOP_FILM_THICKNESS + 0.2,
        )
        .translate(
            x,
            REACTION_CHAMBER_CENTER_Y,
            top_z - CARTRIDGE_TOP_FILM_THICKNESS / 2.0 + 0.1,
        );

        cutouts = cutouts + chamber + optical_window_recess;
    }

    cutouts
}

fn macrofluidic_channel_cutouts() -> Part {
    let mut cuts = Part::empty("macrofluidic_channel_cutouts");
    let top_z = CARTRIDGE_BODY_HEIGHT / 2.0;
    let channel_z = top_z - CARTRIDGE_CHANNEL_DEPTH / 2.0 + 0.1;

    let common_bus = centered_cube(
        "sample_distribution_bus",
        HEATER_ZONE_LENGTH,
        CARTRIDGE_CHANNEL_WIDTH,
        CARTRIDGE_CHANNEL_DEPTH + 0.2,
    )
    .translate(0.0, 0.5, channel_z);
    cuts = cuts + common_bus;

    let swab_to_bus = centered_cube(
        "swab_to_distribution_bus",
        34.0,
        CARTRIDGE_CHANNEL_WIDTH,
        CARTRIDGE_CHANNEL_DEPTH + 0.2,
    )
    .rotate(0.0, 0.0, 25.0)
    .translate(-CARTRIDGE_LENGTH / 2.0 + 35.0, -5.5, channel_z);
    cuts = cuts + swab_to_bus;

    for i in 0..NUM_SLOTS {
        let x = reaction_lane_x(i);
        let inlet = centered_cube(
            format!("lane_{i}_reaction_inlet"),
            CARTRIDGE_CHANNEL_WIDTH,
            REACTION_CHAMBER_CENTER_Y - 0.5,
            CARTRIDGE_CHANNEL_DEPTH + 0.2,
        )
        .translate(x - REACTION_CHAMBER_LENGTH / 4.0, 4.2, channel_z);
        let outlet = centered_cube(
            format!("lane_{i}_reaction_to_waste"),
            CARTRIDGE_CHANNEL_WIDTH,
            CARTRIDGE_WASTE_CENTER_Y - REACTION_CHAMBER_CENTER_Y,
            CARTRIDGE_CHANNEL_DEPTH + 0.2,
        )
        .translate(x + REACTION_CHAMBER_LENGTH / 4.0, 13.0, channel_z);
        cuts = cuts + inlet + outlet;
    }

    cuts
}

fn waste_chamber_cutout() -> Part {
    centered_cube(
        "sealed_waste_chamber",
        CARTRIDGE_WASTE_CHAMBER_LENGTH,
        CARTRIDGE_WASTE_CHAMBER_WIDTH,
        REACTION_CHAMBER_DEPTH + 0.4,
    )
    .translate(
        CARTRIDGE_LENGTH / 2.0 - CARTRIDGE_WASTE_CHAMBER_LENGTH / 2.0 - 8.0,
        CARTRIDGE_WASTE_CENTER_Y,
        CARTRIDGE_BODY_HEIGHT / 2.0 - REACTION_CHAMBER_DEPTH / 2.0,
    )
}

fn alignment_holes() -> Part {
    let mut holes = Part::empty("cartridge_alignment_holes");
    for (i, (x, y)) in cartridge_alignment_positions().iter().enumerate() {
        let hole = centered_cylinder(
            format!("cartridge_alignment_hole_{i}"),
            CARTRIDGE_ALIGNMENT_HOLE_DIAMETER / 2.0,
            CARTRIDGE_BODY_HEIGHT + 2.0,
            24,
        )
        .translate(*x, *y, 0.0);
        holes = holes + hole;
    }
    holes
}

fn gasket_groove() -> Part {
    let top_z = CARTRIDGE_BODY_HEIGHT / 2.0;
    let groove_z = top_z - 0.25;
    let upper = centered_cube(
        "top_film_gasket_groove_rear",
        CARTRIDGE_LENGTH - 12.0,
        0.9,
        0.4,
    )
    .translate(0.0, CARTRIDGE_WIDTH / 2.0 - 5.0, groove_z);
    let lower = centered_cube(
        "top_film_gasket_groove_front",
        CARTRIDGE_LENGTH - 12.0,
        0.9,
        0.4,
    )
    .translate(0.0, -CARTRIDGE_WIDTH / 2.0 + 5.0, groove_z);
    let left = centered_cube(
        "top_film_gasket_groove_left",
        0.9,
        CARTRIDGE_WIDTH - 10.0,
        0.4,
    )
    .translate(-CARTRIDGE_LENGTH / 2.0 + 6.0, 0.0, groove_z);
    let right = centered_cube(
        "top_film_gasket_groove_right",
        0.9,
        CARTRIDGE_WIDTH - 10.0,
        0.4,
    )
    .translate(CARTRIDGE_LENGTH / 2.0 - 6.0, 0.0, groove_z);
    upper + lower + left + right
}

fn cartridge_latch_tabs() -> Part {
    let tab_z = 0.0;
    let left = centered_cube("left_side_latch_tab", 10.0, 3.0, 3.0).translate(
        -CARTRIDGE_LENGTH / 2.0 + 22.0,
        -CARTRIDGE_WIDTH / 2.0 - 1.5,
        tab_z,
    );
    let right = centered_cube("right_side_latch_tab", 10.0, 3.0, 3.0).translate(
        -CARTRIDGE_LENGTH / 2.0 + 22.0,
        CARTRIDGE_WIDTH / 2.0 + 1.5,
        tab_z,
    );
    left + right
}

fn handling_ribs() -> Part {
    let top_z = CARTRIDGE_BODY_HEIGHT / 2.0 + 0.45;
    let mut ribs = Part::empty("cartridge_handling_ribs");
    for (i, y) in [-15.0, 15.0].iter().enumerate() {
        let rib = centered_cube(format!("cartridge_handling_rib_{i}"), 72.0, 1.2, 0.9)
            .translate(8.0, *y, top_z);
        ribs = ribs + rib;
    }
    ribs
}

fn film_lands() -> Part {
    let top_z = CARTRIDGE_BODY_HEIGHT / 2.0 + 0.25;
    let mut lands = Part::empty("reaction_window_film_lands");
    for i in 0..NUM_SLOTS {
        let land = centered_cube(
            format!("reaction_window_film_land_{i}"),
            REACTION_WINDOW_LENGTH + 2.0,
            REACTION_WINDOW_WIDTH + 2.0,
            0.5,
        )
        .translate(reaction_lane_x(i), REACTION_CHAMBER_CENTER_Y, top_z);
        let opening = centered_cube(
            format!("reaction_window_film_opening_{i}"),
            REACTION_WINDOW_LENGTH,
            REACTION_WINDOW_WIDTH,
            0.8,
        )
        .translate(reaction_lane_x(i), REACTION_CHAMBER_CENTER_Y, top_z);
        lands = lands + (land - opening);
    }
    lands
}

fn optical_window_frame() -> Part {
    let frame = centered_cube(
        "diagnostic_cartridge_optical_film_frame",
        HEATER_ZONE_LENGTH + 8.0,
        REACTION_WINDOW_WIDTH + 8.0,
        1.0,
    );

    let mut windows = Part::empty("diagnostic_cartridge_film_frame_windows");
    for i in 0..NUM_SLOTS {
        let window = centered_cube(
            format!("film_frame_window_{i}"),
            REACTION_WINDOW_LENGTH,
            REACTION_WINDOW_WIDTH,
            1.4,
        )
        .translate(reaction_lane_x(i), 0.0, 0.0);
        windows = windows + window;
    }

    frame - windows
}

fn cartridge_alignment_positions() -> [(f64, f64); 4] {
    [
        (-CARTRIDGE_LENGTH / 2.0 + 8.0, -CARTRIDGE_WIDTH / 2.0 + 7.0),
        (CARTRIDGE_LENGTH / 2.0 - 8.0, -CARTRIDGE_WIDTH / 2.0 + 7.0),
        (-CARTRIDGE_LENGTH / 2.0 + 8.0, CARTRIDGE_WIDTH / 2.0 - 7.0),
        (CARTRIDGE_LENGTH / 2.0 - 8.0, CARTRIDGE_WIDTH / 2.0 - 7.0),
    ]
}

fn design_note() -> String {
    [
        "Diagnostic cartridge prototype",
        "",
        "Architecture:",
        "- Disposable cartridge owns swab dock, elution chamber, reaction cavities, optical windows, and waste containment.",
        "- Reusable instrument owns heat, optics, clamping, sensors, and electronics.",
        "- No reusable wet plumbing and no loose PCR tube workflow.",
        "",
        "Prototype material stack:",
        "- 3D printed cartridge body for fit/function iteration.",
        "- Separate optical film/window over reaction lane row.",
        "- Gasket or diagnostic adhesive film for sealing trials.",
        "",
        "Final material direction:",
        "- Injection-molded or bonded COC/COP body and optical window.",
        "- Qualified medical/diagnostic adhesive, film, or gasket seal.",
        "- Printed resin remains a prototype material until inhibition, leaching, autofluorescence, sealing, and sterilization/clean-handling risks are tested.",
        "",
        "Design intent:",
        "- Keep geometry macrofluidic at first: larger chambers, short channels, and no digital partitioning dependency.",
        "- Preserve eight optical lanes from the Rev A electronics so the cartridge can support target, controls, and future multiplexing.",
        "",
    ]
    .join("\n")
}
