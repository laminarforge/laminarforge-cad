use laminarforge_cad::*;
use vcad::{centered_cube, Part};

// Cartridge bay fit-test coupon.
//
// Prints three short bay sections with different XY clearances so the actual
// resin/FDM prototype cartridge can be checked before committing the enclosure
// rail dimensions.

const CLEARANCES: [f64; 3] = [0.4, 0.6, 0.8];
const SECTION_LENGTH: f64 = 36.0;
const SECTION_GAP: f64 = 8.0;
const BASE_Z: f64 = 8.0;

fn main() {
    let mut coupon = Part::empty("diagnostic_cartridge_fit_test_coupon");

    for (i, clearance) in CLEARANCES.iter().enumerate() {
        let x = section_x(i);
        coupon = coupon + fit_section(i, *clearance).translate(x, 0.0, 0.0);
    }

    coupon
        .write_stl("output/diagnostic_cartridge_fit_test.stl")
        .unwrap();

    println!("Exported: output/diagnostic_cartridge_fit_test.stl");
    println!();
    println!("-- Diagnostic Cartridge Bay Fit Test --");
    println!("  Sections:       {} bay clearances", CLEARANCES.len());
    println!("  Cartridge width:{CARTRIDGE_WIDTH:.1}mm nominal");
    for (i, clearance) in CLEARANCES.iter().enumerate() {
        println!("  Section {}:      +{:.1}mm XY clearance", i + 1, clearance);
    }
    println!("  Test:           slide printed cartridge body through each section");
}

fn fit_section(index: usize, clearance: f64) -> Part {
    let section_x = SECTION_LENGTH;
    let section_y = CARTRIDGE_WIDTH + 16.0;
    let section_z = BASE_Z + CARTRIDGE_BODY_HEIGHT + CARTRIDGE_RAIL_HEIGHT + 4.0;
    let base = centered_cube(
        format!("fit_section_{}_base", index + 1),
        section_x,
        section_y,
        section_z,
    );

    let pocket = centered_cube(
        format!("fit_section_{}_cartridge_clearance", index + 1),
        section_x + 2.0,
        CARTRIDGE_WIDTH + clearance,
        CARTRIDGE_BODY_HEIGHT + CARTRIDGE_CLEARANCE_Z,
    )
    .translate(
        0.0,
        0.0,
        -section_z / 2.0 + BASE_Z + (CARTRIDGE_BODY_HEIGHT + CARTRIDGE_CLEARANCE_Z) / 2.0,
    );

    let rail_relief_left = centered_cube(
        format!("fit_section_{}_left_latch_relief", index + 1),
        section_x + 2.0,
        4.0,
        4.0,
    )
    .translate(
        0.0,
        -(CARTRIDGE_WIDTH / 2.0 + clearance / 2.0 + 2.0),
        -section_z / 2.0 + BASE_Z + 2.0,
    );
    let rail_relief_right = centered_cube(
        format!("fit_section_{}_right_latch_relief", index + 1),
        section_x + 2.0,
        4.0,
        4.0,
    )
    .translate(
        0.0,
        CARTRIDGE_WIDTH / 2.0 + clearance / 2.0 + 2.0,
        -section_z / 2.0 + BASE_Z + 2.0,
    );

    let label_bars = clearance_label_bars(index, section_x, section_y, section_z);

    base - pocket - rail_relief_left - rail_relief_right + label_bars
}

fn clearance_label_bars(index: usize, section_x: f64, section_y: f64, section_z: f64) -> Part {
    let mut bars = Part::empty(format!("fit_section_{}_clearance_label", index + 1));
    let count = index + 1;
    let total_width = count as f64 * 2.0 + (count.saturating_sub(1)) as f64 * 1.5;
    let start_x = -total_width / 2.0 + 1.0;

    for bar in 0..count {
        let x = start_x + bar as f64 * 3.5;
        let marker = centered_cube(
            format!("fit_section_{}_bar_{}", index + 1, bar + 1),
            2.0,
            0.8,
            5.0,
        )
        .translate(x, -section_y / 2.0 - 0.4, section_z / 2.0 - 5.0);
        bars = bars + marker;
    }

    let end_stop = centered_cube(
        format!("fit_section_{}_rear_stop", index + 1),
        2.0,
        section_y,
        section_z,
    )
    .translate(section_x / 2.0 - 1.0, 0.0, 0.0);

    bars + end_stop
}

fn section_x(index: usize) -> f64 {
    let pitch = SECTION_LENGTH + SECTION_GAP;
    (index as f64 - (CLEARANCES.len() as f64 - 1.0) / 2.0) * pitch
}
