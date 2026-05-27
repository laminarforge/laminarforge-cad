use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
use vcad::{centered_cube, centered_cylinder, Part};

// Sealed culture module around the automated multi-chip cassette path.
//
// Intent:
// - Hold one cassette bench nest / 20-chip cassette as a sealed process module.
// - Keep routine culture closed: only service ports, sensors, and dock interfaces cross the boundary.
// - Reserve geometry for gas, media, waste, thermal coupling, imaging, and TEER/sensor cabling.
// - Make the module dockable inside the larger clean automation pod.
//
// This is a mechanical architecture model, not a validated sterile enclosure.

const COLS: usize = 4;
const ROWS: usize = 5;
const GUTTER: f64 = 5.0;
const CASSETTE_MARGIN_X: f64 = 28.0;
const CASSETTE_MARGIN_Y: f64 = 28.0;

const ARRAY_X: f64 = COLS as f64 * REVC_CHIP_LENGTH + (COLS as f64 - 1.0) * GUTTER;
const ARRAY_Y: f64 = ROWS as f64 * REVC_CHIP_WIDTH + (ROWS as f64 - 1.0) * GUTTER;
const CASSETTE_X: f64 = ARRAY_X + CASSETTE_MARGIN_X * 2.0;
const CASSETTE_Y: f64 = ARRAY_Y + CASSETTE_MARGIN_Y * 2.0;
const NEST_X: f64 = CASSETTE_X + 160.0;
const NEST_Y: f64 = CASSETTE_Y + 150.0;

const MODULE_X: f64 = NEST_X + 150.0;
const MODULE_Y: f64 = NEST_Y + 135.0;
const BASE_Z: f64 = 70.0;
const WALL: f64 = 8.0;
const LID_Z: f64 = 18.0;
const GASKET_W: f64 = 7.0;
const SERVICE_WALL_Z: f64 = 78.0;

fn main() {
    let base = module_base();
    base.write_stl("output/sealed_culture_module_base.stl")
        .unwrap();
    println!("Exported: output/sealed_culture_module_base.stl");

    let lid = module_lid();
    lid.write_stl("output/sealed_culture_module_lid.stl")
        .unwrap();
    println!("Exported: output/sealed_culture_module_lid.stl");

    let bulkhead = service_bulkhead();
    bulkhead
        .write_stl("output/sealed_culture_module_service_bulkhead.stl")
        .unwrap();
    println!("Exported: output/sealed_culture_module_service_bulkhead.stl");

    let thermal = thermal_interface_plate();
    thermal
        .write_stl("output/sealed_culture_module_thermal_plate.stl")
        .unwrap();
    println!("Exported: output/sealed_culture_module_thermal_plate.stl");

    let assembly = base
        + thermal.translate(0.0, 0.0, -BASE_Z / 2.0 + 8.0)
        + bulkhead.translate(0.0, MODULE_Y / 2.0 + 14.0, 8.0)
        + lid.translate(0.0, 0.0, BASE_Z / 2.0 + LID_Z / 2.0);
    assembly
        .write_stl("output/sealed_culture_module_assembly.stl")
        .unwrap();
    println!("Exported: output/sealed_culture_module_assembly.stl");

    println!(
        "Sealed culture module: {:.0}mm x {:.0}mm footprint, one automated cassette/nest, gasketed lid frame, service bulkhead, and thermal plate interface.",
        MODULE_X, MODULE_Y
    );
}

fn module_base() -> Part {
    let outer = centered_cube("sealed_module_base_outer", MODULE_X, MODULE_Y, BASE_Z);
    let inner = centered_cube(
        "sealed_module_base_inner_cavity",
        MODULE_X - WALL * 2.0,
        MODULE_Y - WALL * 2.0,
        BASE_Z,
    )
    .translate(0.0, 0.0, WALL);

    let nest_recess = centered_cube(
        "sealed_module_nest_recess",
        NEST_X + 20.0,
        NEST_Y + 18.0,
        18.0,
    )
    .translate(0.0, -18.0, -BASE_Z / 2.0 + 12.0);

    let drain_slope_channel =
        centered_cube("sealed_module_drain_channel", MODULE_X - 120.0, 18.0, 10.0).translate(
            0.0,
            -MODULE_Y / 2.0 + 58.0,
            -BASE_Z / 2.0 + 16.0,
        );
    let drain_port = centered_cylinder("sealed_module_drain_port", 5.0, 32.0, 28)
        .rotate(90.0, 0.0, 0.0)
        .translate(
            MODULE_X / 2.0 - 70.0,
            -MODULE_Y / 2.0 - 2.0,
            -BASE_Z / 2.0 + 18.0,
        );

    let front_glove_notch =
        centered_cube("sealed_module_front_service_notch", 210.0, WALL + 4.0, 42.0).translate(
            0.0,
            -MODULE_Y / 2.0,
            8.0,
        );

    let mut deck_mounts = Part::empty("sealed_module_deck_mounts");
    for (i, (x, y)) in [
        (-(MODULE_X / 2.0 - 35.0), -(MODULE_Y / 2.0 - 35.0)),
        (MODULE_X / 2.0 - 35.0, -(MODULE_Y / 2.0 - 35.0)),
        (-(MODULE_X / 2.0 - 35.0), MODULE_Y / 2.0 - 35.0),
        (MODULE_X / 2.0 - 35.0, MODULE_Y / 2.0 - 35.0),
        (0.0, -(MODULE_Y / 2.0 - 35.0)),
        (0.0, MODULE_Y / 2.0 - 35.0),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(format!("sealed_module_mount_boss_{i}"), 11.0, 14.0, 32)
            .translate(*x, *y, -BASE_Z / 2.0 + 7.0);
        let hole = centered_cylinder(format!("sealed_module_m6_hole_{i}"), 6.4 / 2.0, 18.0, 24)
            .translate(*x, *y, -BASE_Z / 2.0 + 7.0);
        deck_mounts = deck_mounts + (boss - hole);
    }

    outer - inner - nest_recess - drain_slope_channel - drain_port - front_glove_notch
        + deck_mounts
        + gasket_land()
}

fn gasket_land() -> Part {
    let outer = centered_cube(
        "sealed_module_gasket_land_outer",
        MODULE_X - 18.0,
        MODULE_Y - 18.0,
        6.0,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0 - 3.0);
    let inner = centered_cube(
        "sealed_module_gasket_land_inner",
        MODULE_X - 18.0 - GASKET_W * 2.0,
        MODULE_Y - 18.0 - GASKET_W * 2.0,
        8.0,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0 - 3.0);
    outer - inner
}

fn module_lid() -> Part {
    let frame_outer = centered_cube("sealed_module_lid_frame_outer", MODULE_X, MODULE_Y, LID_Z);
    let window_cut = centered_cube(
        "sealed_module_lid_window_cut",
        MODULE_X - 115.0,
        MODULE_Y - 130.0,
        LID_Z + 2.0,
    );
    let gasket_groove_outer = centered_cube(
        "sealed_module_lid_gasket_groove_outer",
        MODULE_X - 22.0,
        MODULE_Y - 22.0,
        5.0,
    )
    .translate(0.0, 0.0, -LID_Z / 2.0 + 2.0);
    let gasket_groove_inner = centered_cube(
        "sealed_module_lid_gasket_groove_inner",
        MODULE_X - 22.0 - GASKET_W * 2.0,
        MODULE_Y - 22.0 - GASKET_W * 2.0,
        6.0,
    )
    .translate(0.0, 0.0, -LID_Z / 2.0 + 2.0);

    let mut latch_holes = Part::empty("sealed_module_lid_latch_holes");
    for (i, (x, y)) in latch_points().iter().enumerate() {
        latch_holes = latch_holes
            + centered_cylinder(
                format!("sealed_module_lid_latch_{i}"),
                5.2 / 2.0,
                LID_Z + 2.0,
                24,
            )
            .translate(*x, *y, 0.0);
    }

    frame_outer - window_cut - (gasket_groove_outer - gasket_groove_inner) - latch_holes
        + optical_window()
}

fn optical_window() -> Part {
    centered_cube(
        "sealed_module_clear_window_placeholder",
        MODULE_X - 150.0,
        MODULE_Y - 170.0,
        3.0,
    )
    .translate(0.0, 0.0, -LID_Z / 2.0 + 1.5)
}

fn service_bulkhead() -> Part {
    let body = centered_cube(
        "sealed_module_service_bulkhead_body",
        MODULE_X - 120.0,
        34.0,
        SERVICE_WALL_Z,
    );
    let mut cuts = Part::empty("sealed_module_service_bulkhead_cuts");

    for (i, x) in [-310.0, -285.0, -260.0, -235.0].iter().enumerate() {
        cuts = cuts
            + centered_cylinder(format!("gas_port_{i}"), 8.0 / 2.0, 40.0, 24)
                .rotate(90.0, 0.0, 0.0)
                .translate(*x, 0.0, 18.0);
    }

    for (i, x) in [-145.0, -115.0, -85.0, -55.0, -25.0].iter().enumerate() {
        cuts = cuts
            + centered_cylinder(format!("media_in_port_{i}"), 6.0 / 2.0, 40.0, 24)
                .rotate(90.0, 0.0, 0.0)
                .translate(*x, 0.0, 4.0);
    }

    for (i, x) in [45.0, 75.0, 105.0, 135.0, 165.0].iter().enumerate() {
        cuts = cuts
            + centered_cylinder(format!("waste_out_port_{i}"), 6.0 / 2.0, 40.0, 24)
                .rotate(90.0, 0.0, 0.0)
                .translate(*x, 0.0, 4.0);
    }

    let electrical_slot = centered_cube("sealed_module_sensor_backplane_slot", 90.0, 40.0, 16.0)
        .translate(285.0, 0.0, 12.0);
    let water_in = centered_cylinder("thermal_loop_in", 10.0 / 2.0, 40.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(250.0, 0.0, -20.0);
    let water_out = centered_cylinder("thermal_loop_out", 10.0 / 2.0, 40.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(285.0, 0.0, -20.0);

    body - cuts - electrical_slot - water_in - water_out + port_labels()
}

fn port_labels() -> Part {
    centered_cube(
        "sealed_module_service_label_strip",
        MODULE_X - 150.0,
        2.0,
        8.0,
    )
    .translate(0.0, -18.0, 32.0)
}

fn thermal_interface_plate() -> Part {
    let plate = centered_cube(
        "sealed_module_thermal_plate",
        NEST_X + 38.0,
        NEST_Y + 34.0,
        10.0,
    );
    let mut channels = Part::empty("sealed_module_thermal_channels");

    for i in 0..7 {
        let y = -NEST_Y / 2.0 + 70.0 + i as f64 * 85.0;
        channels = channels
            + centered_cylinder(
                format!("thermal_serpentine_channel_{i}"),
                6.0 / 2.0,
                NEST_X,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, y, 0.0);
    }

    let inlet = centered_cylinder("thermal_plate_inlet", 6.0 / 2.0, 70.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(-(NEST_X / 2.0 - 50.0), -(NEST_Y / 2.0 + 8.0), 0.0);
    let outlet = centered_cylinder("thermal_plate_outlet", 6.0 / 2.0, 70.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(NEST_X / 2.0 - 50.0, -(NEST_Y / 2.0 + 8.0), 0.0);

    plate - channels - inlet - outlet
}

fn latch_points() -> [(f64, f64); 10] {
    [
        (-(MODULE_X / 2.0 - 55.0), -(MODULE_Y / 2.0 - 42.0)),
        (0.0, -(MODULE_Y / 2.0 - 42.0)),
        (MODULE_X / 2.0 - 55.0, -(MODULE_Y / 2.0 - 42.0)),
        (-(MODULE_X / 2.0 - 55.0), MODULE_Y / 2.0 - 42.0),
        (0.0, MODULE_Y / 2.0 - 42.0),
        (MODULE_X / 2.0 - 55.0, MODULE_Y / 2.0 - 42.0),
        (-(MODULE_X / 2.0 - 42.0), 0.0),
        (MODULE_X / 2.0 - 42.0, 0.0),
        (-(MODULE_X / 2.0 - 42.0), -MODULE_Y / 4.0),
        (MODULE_X / 2.0 - 42.0, MODULE_Y / 4.0),
    ]
}
