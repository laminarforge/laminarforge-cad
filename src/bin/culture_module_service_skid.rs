use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
use vcad::{centered_cube, centered_cylinder, Part};

// Service skid for the sealed culture module.
//
// Intent:
// - Give the sealed culture module a dockable bench/service footprint.
// - Mirror the service-bulkhead port families from sealed_culture_module.rs.
// - Keep utilities outside the sealed process volume: gas, media, waste,
//   thermal loop, electrical backplane, and removable pump/reservoir tray.
// - Provide drain containment, locator bosses, tie-downs, and cart/skid feet.
//
// This is a layout and fit model for module support hardware, not a validated
// sterile barrier or load-rated cart.

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

const SKID_X: f64 = MODULE_X + 260.0;
const SKID_Y: f64 = MODULE_Y + 190.0;
const DECK_Z: f64 = 18.0;
const MODULE_CLEARANCE: f64 = 8.0;
const RAIL_W: f64 = 24.0;
const RAIL_Z: f64 = 32.0;
const PANEL_Y: f64 = 18.0;
const PANEL_Z: f64 = 132.0;
const TRAY_X: f64 = MODULE_X - 110.0;
const TRAY_Y: f64 = 150.0;
const TRAY_Z: f64 = 24.0;
const SERVICE_Y: f64 = SKID_Y / 2.0 - 44.0;

fn main() {
    let deck = skid_deck();
    deck.write_stl("output/culture_module_service_skid_deck.stl")
        .unwrap();
    println!("Exported: output/culture_module_service_skid_deck.stl");

    let utility_panel = utility_panel();
    utility_panel
        .write_stl("output/culture_module_service_skid_utility_panel.stl")
        .unwrap();
    println!("Exported: output/culture_module_service_skid_utility_panel.stl");

    let cartridge_tray = service_cartridge_tray();
    cartridge_tray
        .write_stl("output/culture_module_service_skid_cartridge_tray.stl")
        .unwrap();
    println!("Exported: output/culture_module_service_skid_cartridge_tray.stl");

    let assembly = deck
        + utility_panel.translate(0.0, SERVICE_Y, DECK_Z / 2.0 + PANEL_Z / 2.0)
        + cartridge_tray.translate(0.0, -(SKID_Y / 2.0 - 92.0), DECK_Z / 2.0 + TRAY_Z / 2.0);

    assembly
        .write_stl("output/culture_module_service_skid_assembly.stl")
        .unwrap();
    println!("Exported: output/culture_module_service_skid_assembly.stl");

    println!(
        "Culture module service skid: {:.0}mm x {:.0}mm deck, accepts {:.0}mm x {:.0}mm sealed module, rear utility panel, removable {:.0}mm x {:.0}mm cartridge tray.",
        SKID_X, SKID_Y, MODULE_X, MODULE_Y, TRAY_X, TRAY_Y
    );
}

fn skid_deck() -> Part {
    let deck = centered_cube("culture_service_skid_deck", SKID_X, SKID_Y, DECK_Z);

    let module_recess = centered_cube(
        "culture_service_skid_module_recess",
        MODULE_X + MODULE_CLEARANCE,
        MODULE_Y + MODULE_CLEARANCE,
        7.0,
    )
    .translate(0.0, 10.0, DECK_Z / 2.0 - 3.5);

    let drain_sump = centered_cube(
        "culture_service_skid_drain_sump",
        MODULE_X - 90.0,
        20.0,
        10.0,
    )
    .translate(0.0, -(MODULE_Y / 2.0 - 35.0), DECK_Z / 2.0 - 5.0);
    let drain_port = centered_cylinder("culture_service_skid_drain_port", 7.0 / 2.0, 34.0, 28)
        .rotate(90.0, 0.0, 0.0)
        .translate(MODULE_X / 2.0 - 72.0, -(MODULE_Y / 2.0 + 18.0), 0.0);

    let lift_slot_left = centered_cube(
        "culture_service_skid_lift_slot_left",
        SKID_X - 150.0,
        30.0,
        DECK_Z + 2.0,
    )
    .translate(0.0, -(SKID_Y / 2.0 - 46.0), 0.0);
    let lift_slot_right = centered_cube(
        "culture_service_skid_lift_slot_right",
        SKID_X - 150.0,
        30.0,
        DECK_Z + 2.0,
    )
    .translate(0.0, SKID_Y / 2.0 - 46.0, 0.0);

    let tray_slide = centered_cube(
        "culture_service_skid_front_tray_slide_clearance",
        TRAY_X + 30.0,
        TRAY_Y + 28.0,
        DECK_Z + 2.0,
    )
    .translate(0.0, -(SKID_Y / 2.0 - 92.0), 0.0);

    let mut bolt_holes = Part::empty("culture_service_skid_bolt_holes");
    for (i, (x, y)) in deck_mount_points().iter().enumerate() {
        bolt_holes = bolt_holes
            + centered_cylinder(
                format!("culture_service_skid_m6_tie_down_{i}"),
                6.6 / 2.0,
                DECK_Z + 2.0,
                24,
            )
            .translate(*x, *y, 0.0);
    }

    let mut locating_features = Part::empty("culture_service_skid_locators");
    for (i, (x, y)) in module_locator_points().iter().enumerate() {
        let boss = centered_cylinder(
            format!("culture_service_skid_locator_boss_{i}"),
            11.0,
            9.0,
            32,
        )
        .translate(*x, *y + 10.0, DECK_Z / 2.0 + 4.5);
        let dowel_socket = centered_cylinder(
            format!("culture_service_skid_locator_socket_{i}"),
            4.0 / 2.0,
            11.0,
            24,
        )
        .translate(*x, *y + 10.0, DECK_Z / 2.0 + 4.5);
        locating_features = locating_features + (boss - dowel_socket);
    }

    deck - module_recess
        - drain_sump
        - drain_port
        - lift_slot_left
        - lift_slot_right
        - tray_slide
        - bolt_holes
        + side_rails()
        + corner_feet()
        + locating_features
        + tray_slide_rails()
}

fn side_rails() -> Part {
    let left = centered_cube(
        "culture_service_skid_left_guard_rail",
        RAIL_W,
        MODULE_Y + 78.0,
        RAIL_Z,
    )
    .translate(
        -(MODULE_X / 2.0 + RAIL_W / 2.0 + 18.0),
        10.0,
        DECK_Z / 2.0 + RAIL_Z / 2.0,
    );
    let right = centered_cube(
        "culture_service_skid_right_guard_rail",
        RAIL_W,
        MODULE_Y + 78.0,
        RAIL_Z,
    )
    .translate(
        MODULE_X / 2.0 + RAIL_W / 2.0 + 18.0,
        10.0,
        DECK_Z / 2.0 + RAIL_Z / 2.0,
    );
    let rear = centered_cube(
        "culture_service_skid_rear_dock_rail",
        MODULE_X + 84.0,
        RAIL_W,
        RAIL_Z,
    )
    .translate(
        0.0,
        MODULE_Y / 2.0 + RAIL_W / 2.0 + 28.0,
        DECK_Z / 2.0 + RAIL_Z / 2.0,
    );
    let front_stops = centered_cube(
        "culture_service_skid_front_left_stop",
        MODULE_X / 2.0 - 62.0,
        RAIL_W,
        RAIL_Z,
    )
    .translate(
        -(MODULE_X / 4.0 + 24.0),
        -(MODULE_Y / 2.0 + RAIL_W / 2.0 + 8.0),
        DECK_Z / 2.0 + RAIL_Z / 2.0,
    ) + centered_cube(
        "culture_service_skid_front_right_stop",
        MODULE_X / 2.0 - 62.0,
        RAIL_W,
        RAIL_Z,
    )
    .translate(
        MODULE_X / 4.0 + 24.0,
        -(MODULE_Y / 2.0 + RAIL_W / 2.0 + 8.0),
        DECK_Z / 2.0 + RAIL_Z / 2.0,
    );

    left + right + rear + front_stops
}

fn corner_feet() -> Part {
    let mut feet = Part::empty("culture_service_skid_leveling_feet");
    for (i, (x, y)) in [
        (-(SKID_X / 2.0 - 52.0), -(SKID_Y / 2.0 - 52.0)),
        (SKID_X / 2.0 - 52.0, -(SKID_Y / 2.0 - 52.0)),
        (-(SKID_X / 2.0 - 52.0), SKID_Y / 2.0 - 52.0),
        (SKID_X / 2.0 - 52.0, SKID_Y / 2.0 - 52.0),
        (0.0, -(SKID_Y / 2.0 - 52.0)),
        (0.0, SKID_Y / 2.0 - 52.0),
    ]
    .iter()
    .enumerate()
    {
        let pad = centered_cylinder(
            format!("culture_service_skid_leveling_pad_{i}"),
            23.0,
            10.0,
            40,
        )
        .translate(*x, *y, -(DECK_Z / 2.0 + 5.0));
        let adjuster = centered_cylinder(
            format!("culture_service_skid_leveler_thread_clearance_{i}"),
            8.0 / 2.0,
            20.0,
            24,
        )
        .translate(*x, *y, -(DECK_Z / 2.0 + 5.0));
        feet = feet + (pad - adjuster);
    }
    feet
}

fn tray_slide_rails() -> Part {
    let rail_z = 10.0;
    let left = centered_cube(
        "culture_service_skid_tray_slide_left",
        18.0,
        TRAY_Y + 38.0,
        rail_z,
    )
    .translate(
        -(TRAY_X / 2.0 + 18.0),
        -(SKID_Y / 2.0 - 92.0),
        DECK_Z / 2.0 + rail_z / 2.0,
    );
    let right = centered_cube(
        "culture_service_skid_tray_slide_right",
        18.0,
        TRAY_Y + 38.0,
        rail_z,
    )
    .translate(
        TRAY_X / 2.0 + 18.0,
        -(SKID_Y / 2.0 - 92.0),
        DECK_Z / 2.0 + rail_z / 2.0,
    );
    let rear_stop = centered_cube(
        "culture_service_skid_tray_slide_rear_stop",
        TRAY_X + 54.0,
        16.0,
        rail_z,
    )
    .translate(
        0.0,
        -(SKID_Y / 2.0 - 92.0) + TRAY_Y / 2.0 + 16.0,
        DECK_Z / 2.0 + rail_z / 2.0,
    );

    left + right + rear_stop
}

fn utility_panel() -> Part {
    let body = centered_cube(
        "culture_service_skid_utility_panel_body",
        MODULE_X + 120.0,
        PANEL_Y,
        PANEL_Z,
    );

    let mut cuts = Part::empty("culture_service_skid_utility_panel_cuts");

    for (i, x) in [-310.0, -285.0, -260.0, -235.0].iter().enumerate() {
        cuts = cuts
            + centered_cylinder(
                format!("culture_service_skid_gas_bulkhead_{i}"),
                8.0 / 2.0,
                PANEL_Y + 8.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 0.0, 32.0);
    }

    for (i, x) in [-145.0, -115.0, -85.0, -55.0, -25.0].iter().enumerate() {
        cuts = cuts
            + centered_cylinder(
                format!("culture_service_skid_media_bulkhead_{i}"),
                6.0 / 2.0,
                PANEL_Y + 8.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 0.0, 10.0);
    }

    for (i, x) in [45.0, 75.0, 105.0, 135.0, 165.0].iter().enumerate() {
        cuts = cuts
            + centered_cylinder(
                format!("culture_service_skid_waste_bulkhead_{i}"),
                6.0 / 2.0,
                PANEL_Y + 8.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 0.0, 10.0);
    }

    for (i, x) in [250.0, 285.0].iter().enumerate() {
        cuts = cuts
            + centered_cylinder(
                format!("culture_service_skid_thermal_loop_{i}"),
                10.0 / 2.0,
                PANEL_Y + 8.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 0.0, -26.0);
    }

    let backplane_slot = centered_cube(
        "culture_service_skid_backplane_slot",
        92.0,
        PANEL_Y + 8.0,
        20.0,
    )
    .translate(295.0, 0.0, 18.0);
    let relief_port = centered_cylinder(
        "culture_service_skid_relief_port",
        16.0 / 2.0,
        PANEL_Y + 8.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-340.0, 0.0, -28.0);
    let cable_tray_window = centered_cube(
        "culture_service_skid_cable_tray_window",
        MODULE_X - 150.0,
        PANEL_Y + 8.0,
        16.0,
    )
    .translate(0.0, 0.0, -48.0);

    let panel = body - cuts - backplane_slot - relief_port - cable_tray_window
        + panel_label_rails()
        + panel_mount_tabs();

    panel + tubing_strain_relief_comb().translate(0.0, -(PANEL_Y / 2.0 + 14.0), -52.0)
}

fn panel_label_rails() -> Part {
    let upper = centered_cube(
        "culture_service_skid_utility_label_strip_upper",
        MODULE_X - 125.0,
        3.0,
        7.0,
    )
    .translate(0.0, -(PANEL_Y / 2.0 + 1.5), 54.0);
    let lower = centered_cube(
        "culture_service_skid_utility_label_strip_lower",
        MODULE_X - 125.0,
        3.0,
        7.0,
    )
    .translate(0.0, -(PANEL_Y / 2.0 + 1.5), -4.0);
    upper + lower
}

fn panel_mount_tabs() -> Part {
    let mut tabs = Part::empty("culture_service_skid_utility_panel_mount_tabs");
    for (i, x) in [
        -(MODULE_X / 2.0 + 34.0),
        -(MODULE_X / 4.0),
        MODULE_X / 4.0,
        MODULE_X / 2.0 + 34.0,
    ]
    .iter()
    .enumerate()
    {
        let tab = centered_cube(
            format!("culture_service_skid_panel_mount_tab_{i}"),
            38.0,
            30.0,
            16.0,
        )
        .translate(*x, 5.0, -(PANEL_Z / 2.0 + 8.0));
        let hole = centered_cylinder(
            format!("culture_service_skid_panel_mount_hole_{i}"),
            5.4 / 2.0,
            34.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, 5.0, -(PANEL_Z / 2.0 + 8.0));
        tabs = tabs + (tab - hole);
    }
    tabs
}

fn tubing_strain_relief_comb() -> Part {
    let body = centered_cube(
        "culture_service_skid_tubing_strain_relief_comb",
        MODULE_X - 170.0,
        18.0,
        18.0,
    );
    let mut cuts = Part::empty("culture_service_skid_tubing_comb_cuts");

    for (i, x) in [-310.0, -285.0, -260.0, -235.0].iter().enumerate() {
        cuts = cuts
            + centered_cylinder(format!("culture_service_skid_gas_clip_{i}"), 5.0, 22.0, 24)
                .rotate(90.0, 0.0, 0.0)
                .translate(*x, 0.0, 0.0)
            + centered_cube(
                format!("culture_service_skid_gas_clip_slot_{i}"),
                10.0,
                22.0,
                12.0,
            )
            .translate(*x, 0.0, 7.0);
    }

    for (i, x) in [
        -145.0, -115.0, -85.0, -55.0, -25.0, 45.0, 75.0, 105.0, 135.0, 165.0,
    ]
    .iter()
    .enumerate()
    {
        cuts = cuts
            + centered_cylinder(
                format!("culture_service_skid_fluid_clip_{i}"),
                4.0,
                22.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 0.0, 0.0)
            + centered_cube(
                format!("culture_service_skid_fluid_clip_slot_{i}"),
                8.0,
                22.0,
                12.0,
            )
            .translate(*x, 0.0, 7.0);
    }

    body - cuts
}

fn service_cartridge_tray() -> Part {
    let base = centered_cube(
        "culture_service_skid_cartridge_tray_base",
        TRAY_X,
        TRAY_Y,
        TRAY_Z,
    );
    let tray_floor_pocket = centered_cube(
        "culture_service_skid_cartridge_tray_floor_pocket",
        TRAY_X - 34.0,
        TRAY_Y - 30.0,
        TRAY_Z - 7.0,
    )
    .translate(0.0, 0.0, 5.0);

    let handle_cut = centered_cube(
        "culture_service_skid_cartridge_tray_pull_handle",
        96.0,
        16.0,
        TRAY_Z + 2.0,
    )
    .translate(0.0, -(TRAY_Y / 2.0 - 8.0), 0.0);

    let mut bottle_wells = Part::empty("culture_service_skid_cartridge_tray_bottle_wells");
    for (i, x) in [-220.0, -150.0, -80.0, -10.0, 60.0].iter().enumerate() {
        bottle_wells = bottle_wells
            + centered_cylinder(
                format!("culture_service_skid_media_bottle_well_{i}"),
                24.0,
                TRAY_Z + 2.0,
                40,
            )
            .translate(*x, 20.0, 4.0);
    }

    let mut waste_wells = Part::empty("culture_service_skid_cartridge_tray_waste_wells");
    for (i, x) in [150.0, 220.0].iter().enumerate() {
        waste_wells = waste_wells
            + centered_cylinder(
                format!("culture_service_skid_waste_bottle_well_{i}"),
                30.0,
                TRAY_Z + 2.0,
                40,
            )
            .translate(*x, 20.0, 4.0);
    }

    let pump_bay = centered_cube(
        "culture_service_skid_cartridge_pump_bay",
        220.0,
        44.0,
        TRAY_Z + 2.0,
    )
    .translate(-40.0, -(TRAY_Y / 2.0 - 36.0), 4.0);

    let drip_channel = centered_cube(
        "culture_service_skid_cartridge_drip_channel",
        TRAY_X - 80.0,
        12.0,
        TRAY_Z + 2.0,
    )
    .translate(0.0, -8.0, 4.0);

    let key = centered_cube(
        "culture_service_skid_cartridge_asymmetric_key",
        44.0,
        18.0,
        10.0,
    )
    .translate(-(TRAY_X / 2.0 - 48.0), TRAY_Y / 2.0 + 9.0, -2.0);

    let latch_bosses = cartridge_latch_bosses();

    base - tray_floor_pocket - handle_cut - bottle_wells - waste_wells - pump_bay - drip_channel
        + key
        + latch_bosses
}

fn cartridge_latch_bosses() -> Part {
    let mut bosses = Part::empty("culture_service_skid_cartridge_latch_bosses");
    for (i, x) in [-(TRAY_X / 2.0 - 28.0), TRAY_X / 2.0 - 28.0]
        .iter()
        .enumerate()
    {
        let boss = centered_cylinder(
            format!("culture_service_skid_cartridge_latch_boss_{i}"),
            10.0,
            8.0,
            24,
        )
        .translate(*x, -(TRAY_Y / 2.0 - 28.0), TRAY_Z / 2.0 + 4.0);
        let hole = centered_cylinder(
            format!("culture_service_skid_cartridge_latch_hole_{i}"),
            3.3 / 2.0,
            10.0,
            20,
        )
        .translate(*x, -(TRAY_Y / 2.0 - 28.0), TRAY_Z / 2.0 + 4.0);
        bosses = bosses + (boss - hole);
    }
    bosses
}

fn deck_mount_points() -> [(f64, f64); 8] {
    [
        (-(MODULE_X / 2.0 - 38.0), -(MODULE_Y / 2.0 - 34.0)),
        (0.0, -(MODULE_Y / 2.0 - 34.0)),
        (MODULE_X / 2.0 - 38.0, -(MODULE_Y / 2.0 - 34.0)),
        (-(MODULE_X / 2.0 - 38.0), MODULE_Y / 2.0 - 34.0),
        (0.0, MODULE_Y / 2.0 - 34.0),
        (MODULE_X / 2.0 - 38.0, MODULE_Y / 2.0 - 34.0),
        (-(MODULE_X / 2.0 - 38.0), 0.0),
        (MODULE_X / 2.0 - 38.0, 0.0),
    ]
}

fn module_locator_points() -> [(f64, f64); 4] {
    [
        (-(MODULE_X / 2.0 - 64.0), -(MODULE_Y / 2.0 - 58.0)),
        (MODULE_X / 2.0 - 64.0, -(MODULE_Y / 2.0 - 58.0)),
        (-(MODULE_X / 2.0 - 64.0), MODULE_Y / 2.0 - 58.0),
        (MODULE_X / 2.0 - 64.0, MODULE_Y / 2.0 - 58.0),
    ]
}
