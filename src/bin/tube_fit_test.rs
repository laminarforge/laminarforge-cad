use laminarforge_cad::*;
use vcad::{centered_cube, centered_cylinder, Part};

// ─── Tube Fit Test Jig ───
//
// Small block with 6 holes at different diameters to find the optimal
// friction fit for 0.2mL PCR tubes (6.096mm OD) on the Bambu Lab A1.
//
// Each hole has raised tally marks on the front face:
//   1 bar  = 6.1mm CAD (tightest)
//   2 bars = 6.2mm CAD
//   3 bars = 6.3mm CAD (current HOLDER_TUBE_DIAMETER)
//   4 bars = 6.4mm CAD
//   5 bars = 6.5mm CAD (original value)
//   6 bars = 6.6mm CAD (loosest)
//
// After printing, test each hole with a tube and pick the best fit.
// Then update HOLDER_TUBE_DIAMETER in lib.rs with the winner.
//
// Export: output/tube_fit_test.stl

const NUM_TEST_HOLES: usize = 6;
const HOLE_SPACING: f64 = 12.0;
const START_DIAMETER: f64 = 6.1;
const DIAMETER_STEP: f64 = 0.1;

// Block dimensions
const BLOCK_HEIGHT: f64 = 10.0; // tall enough to test friction
const BLOCK_WIDTH: f64 = 18.0; // front-to-back
const BLOCK_LENGTH: f64 = (NUM_TEST_HOLES as f64) * HOLE_SPACING + 8.0; // left-right

// Tally mark dimensions
const MARK_WIDTH: f64 = 0.8;
const MARK_HEIGHT: f64 = 4.0;
const MARK_DEPTH: f64 = 0.6; // raised above surface
const MARK_GAP: f64 = 1.4; // between bars

fn main() {
    let body = centered_cube("body", BLOCK_LENGTH, BLOCK_WIDTH, BLOCK_HEIGHT);

    let first_x = -((NUM_TEST_HOLES as f64 - 1.0) * HOLE_SPACING) / 2.0;

    let mut holes = Part::empty("holes");
    let mut marks = Part::empty("marks");

    for i in 0..NUM_TEST_HOLES {
        let diameter = START_DIAMETER + (i as f64) * DIAMETER_STEP;
        let x = first_x + (i as f64) * HOLE_SPACING;

        // Through-hole
        let hole = centered_cylinder(&format!("hole_{i}"), diameter / 2.0, BLOCK_HEIGHT + 2.0, 32)
            .translate(x, 0.0, 0.0);
        holes = holes + hole;

        // Tally marks on front face (-Y side)
        let num_bars = i + 1;
        let total_marks_width = (num_bars as f64) * MARK_WIDTH + ((num_bars - 1) as f64) * MARK_GAP;
        let mark_start_x = x - total_marks_width / 2.0 + MARK_WIDTH / 2.0;

        for b in 0..num_bars {
            let mx = mark_start_x + (b as f64) * (MARK_WIDTH + MARK_GAP);
            let bar = centered_cube(
                &format!("mark_{i}_{b}"),
                MARK_WIDTH,
                MARK_DEPTH,
                MARK_HEIGHT,
            )
            .translate(
                mx,
                -(BLOCK_WIDTH / 2.0) - MARK_DEPTH / 2.0 + 0.01, // flush with front face, raised outward
                -(BLOCK_HEIGHT / 2.0) + MARK_HEIGHT / 2.0 + 1.0, // near bottom of front face
            );
            marks = marks + bar;
        }
    }

    let part = (body - holes) + marks;

    part.write_stl("output/tube_fit_test.stl").unwrap();

    println!("Exported: output/tube_fit_test.stl");
    println!();
    println!("── Tube Fit Test Jig ──");
    println!("  Tube OD:    {TUBE_OD:.3}mm (Novas Bio 0.2mL PCR)");
    println!("  Block:      {BLOCK_LENGTH:.0}mm x {BLOCK_WIDTH:.0}mm x {BLOCK_HEIGHT:.0}mm");
    println!("  FDM shrink: ~0.2mm expected (holes print smaller than CAD)");
    println!();
    println!("  Tally  CAD dia   Est. printed   Clearance vs tube");
    println!("  ─────  ───────   ────────────   ─────────────────");
    for i in 0..NUM_TEST_HOLES {
        let d = START_DIAMETER + (i as f64) * DIAMETER_STEP;
        let printed = d - 0.2;
        let clearance = printed - TUBE_OD;
        let bars: String = "|".repeat(i + 1);
        let fit = if clearance < -0.05 {
            "interference"
        } else if clearance < 0.05 {
            "friction fit"
        } else if clearance < 0.15 {
            "snug"
        } else {
            "loose"
        };
        println!("  {bars:<6} {d:.1}mm     ~{printed:.1}mm          {clearance:+.3}mm ({fit})");
    }
    println!();
    println!("  Test each hole with a tube. Pick the best fit,");
    println!("  then update HOLDER_TUBE_DIAMETER in lib.rs.");
}
