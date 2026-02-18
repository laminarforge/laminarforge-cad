use vcad::{centered_cube, centered_cylinder};

// ─── Waste Tray ───
//
// 3D-printed (PETG) waste collection tray for dispensing system
// priming and purging. Sits below the dispensing tips to catch
// excess reagent during prime/purge cycles.
//
// Features:
// - Tray body: 90x30x15mm with 2mm walls
// - Interior sloped toward drain hole at one end
// - Splash guard lip (3mm tall) around perimeter
// - Drain hole: 5mm dia at low end (connects to waste bottle via tubing)
// - 2x M3 mounting holes on bottom
//
// The sloped interior ensures liquid flows toward the drain,
// preventing pooling and cross-contamination between purge cycles.
//
// Material: PETG

fn main() {
    // ── Dimensions ──

    let tray_length = 90.0; // X
    let tray_width = 30.0; // Y
    let tray_height = 15.0; // Z (wall height)
    let wall = 2.0; // mm wall thickness

    let splash_lip = 3.0; // mm additional height around rim

    let drain_diameter = 5.0; // mm
    // Drain at -X end, centered in Y, through the bottom

    // Interior slope: higher at +X end, drains to -X end
    // Slope is achieved by tilting the interior floor
    let slope_rise = 3.0; // mm higher at +X than -X

    // Mounting holes
    let mount_hole_d = 3.2; // M3 clearance
    let mount_hole_spacing = 60.0; // center-to-center along X

    // Interior dimensions
    let inner_length = tray_length - wall * 2.0;
    let inner_width = tray_width - wall * 2.0;
    let _inner_height = tray_height - wall; // open top

    // Total height including splash lip
    let total_height = tray_height + splash_lip;

    // ── Build tray ──

    // Outer shell (including splash lip height)
    let outer = centered_cube("outer", tray_length, tray_width, total_height);

    // Inner cavity (hollowed out from top)
    // The cavity is deeper than the wall height to make it open on top
    let cavity = centered_cube(
        "cavity",
        inner_length,
        inner_width,
        total_height - wall + 1.0,
    )
    .translate(0.0, 0.0, wall / 2.0 + 0.5);

    // ── Sloped floor ──
    // To create a slope, add a wedge-shaped block inside the cavity
    // that raises the +X end. The wedge is a cube tilted via CSG.
    // Simpler approach: place a tapered block in the bottom of the cavity.

    // Wedge: a cube that is slope_rise tall at +X and 0 at -X
    // Approximate with a tilted slab. Since vcad doesn't have native wedge,
    // use a cube placed so it creates a ramp.
    // Actually, we'll create the slope by subtracting a tilted cut from a floor slab.

    // Floor slab that fills the bottom of the interior
    let floor_slab = centered_cube(
        "floor_slab",
        inner_length,
        inner_width,
        slope_rise,
    )
    .translate(0.0, 0.0, -(total_height / 2.0) + wall + slope_rise / 2.0);

    // Cut away the -X side to create the ramp (angled cut)
    // Use a tall cube positioned to remove material from the -X, lower portion
    // The slope goes from z=wall at -X to z=wall+slope_rise at +X
    // Approximate as stepped ramp (3 steps for simplicity)
    let num_steps = 6;
    let step_length = inner_length / num_steps as f64;
    let step_rise = slope_rise / num_steps as f64;

    let mut slope_cuts = centered_cube("sc_init", 0.01, 0.01, 0.01);
    for i in 0..num_steps {
        // Each step removes material at the -X side
        // Step i: from x_start, remove everything above step height
        let step_x = -(inner_length / 2.0) + (i as f64) * step_length + step_length / 2.0;
        let cut_height = slope_rise - (i as f64) * step_rise;
        if cut_height > 0.01 {
            let cut = centered_cube(
                &format!("slope_step_{i}"),
                step_length + 0.1,
                inner_width + 0.1,
                cut_height,
            )
            .translate(
                step_x,
                0.0,
                -(total_height / 2.0) + wall + slope_rise - cut_height / 2.0,
            );
            slope_cuts = slope_cuts + cut;
        }
    }

    // ── Drain hole ──
    // Through the bottom at -X end, where the floor is lowest

    let drain_x = -(tray_length / 2.0) + wall + drain_diameter;
    let drain = centered_cylinder(
        "drain",
        drain_diameter / 2.0,
        wall + slope_rise + 2.0,
        24,
    )
    .translate(drain_x, 0.0, -(total_height / 2.0) + (wall + slope_rise) / 2.0);

    // ── Mounting holes ──

    let mount_hole_1 = centered_cylinder(
        "mount_hole_1",
        mount_hole_d / 2.0,
        total_height + 2.0,
        24,
    )
    .translate(mount_hole_spacing / 2.0, 0.0, 0.0);

    let mount_hole_2 = centered_cylinder(
        "mount_hole_2",
        mount_hole_d / 2.0,
        total_height + 2.0,
        24,
    )
    .translate(-(mount_hole_spacing / 2.0), 0.0, 0.0);

    // ── Assemble ──

    let tray = (outer + floor_slab)
        - cavity
        - slope_cuts
        - drain
        - mount_hole_1
        - mount_hole_2;

    // ── Export ──

    tray.write_stl("output/waste_tray.stl").unwrap();

    println!("Exported: output/waste_tray.stl");
    println!();
    println!("── Waste Tray Specs ──");
    println!("  Outer:          {tray_length:.0}mm x {tray_width:.0}mm x {total_height:.0}mm (incl. splash lip)");
    println!("  Wall thickness: {wall:.0}mm");
    println!("  Splash lip:     {splash_lip:.0}mm above rim");
    println!("  Interior:       {inner_length:.0}mm x {inner_width:.0}mm");
    println!("  Floor slope:    {slope_rise:.0}mm rise ({num_steps}-step ramp, drains to -X)");
    println!("  Drain hole:     {drain_diameter:.0}mm dia at low end");
    println!("  Mounting holes: 2x M3 ({mount_hole_d:.1}mm) at {mount_hole_spacing:.0}mm spacing");
    println!("  Material:       PETG");
}
