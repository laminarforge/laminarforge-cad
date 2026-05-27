use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed-module residual rinse sampling station.
//
// Intent:
// - Collect validated post-cleaning rinse samples from sealed culture modules
//   without opening the sterile interior to the cabinet atmosphere.
// - Keep sterile rinse feed, returned rinse, drain isolation, sample capture,
//   TOC/conductivity/protein witness points, barcode traceability, and leak
//   containment visible in one bench-scale module.
// - Provide keyed module alignment and robot keepout envelopes so automated
//   loading, scanning, and vial handling are constrained by the CAD itself.
//
// This is product-architecture CAD for mechanical packaging and verification
// planning. It is not a cleaning validation protocol, sterile connector
// specification, or analytical method validation.

const OUTPUTS: &[&str] = &[
    "output/closed_module_residual_rinse_sampling_station_baseplate.stl",
    "output/closed_module_residual_rinse_sampling_station_leak_tray.stl",
    "output/closed_module_residual_rinse_sampling_station_sealed_module_dock.stl",
    "output/closed_module_residual_rinse_sampling_station_sterile_rinse_inlet_manifold.stl",
    "output/closed_module_residual_rinse_sampling_station_residual_return_manifold.stl",
    "output/closed_module_residual_rinse_sampling_station_sample_vial_carousel.stl",
    "output/closed_module_residual_rinse_sampling_station_analytical_test_pockets.stl",
    "output/closed_module_residual_rinse_sampling_station_drain_isolation_bank.stl",
    "output/closed_module_residual_rinse_sampling_station_barcode_run_record_land.stl",
    "output/closed_module_residual_rinse_sampling_station_robot_service_keepouts.stl",
    "output/closed_module_residual_rinse_sampling_station_assembly.stl",
];

const STATION_X: f64 = 1120.0;
const STATION_Y: f64 = 780.0;
const BASE_Z: f64 = 22.0;
const RAIL_Z: f64 = 32.0;
const MOUNT_HOLE_D: f64 = 6.6;

const LEAK_TRAY_X: f64 = 950.0;
const LEAK_TRAY_Y: f64 = 610.0;
const LEAK_TRAY_Z: f64 = 34.0;
const LEAK_SENSOR_WELLS: usize = 4;

const DOCK_X: f64 = 620.0;
const DOCK_Y: f64 = 382.0;
const DOCK_Z: f64 = 118.0;
const MODULE_ENVELOPE_X: f64 = 512.0;
const MODULE_ENVELOPE_Y: f64 = 278.0;
const MODULE_ENVELOPE_Z: f64 = 72.0;
const DOCK_CLAMP_COUNT: usize = 6;
const KEY_PIN_COUNT: usize = 4;

const INLET_MANIFOLD_X: f64 = 760.0;
const INLET_MANIFOLD_Y: f64 = 92.0;
const INLET_MANIFOLD_Z: f64 = 56.0;
const RINSE_INLET_PORTS: usize = 8;
const RINSE_INLET_PORT_D: f64 = 6.0;
const RINSE_PORT_PITCH_X: f64 = 82.0;

const RETURN_MANIFOLD_X: f64 = 760.0;
const RETURN_MANIFOLD_Y: f64 = 104.0;
const RETURN_MANIFOLD_Z: f64 = 60.0;
const RETURN_PORTS: usize = 8;
const RETURN_PORT_D: f64 = 7.0;

const SAMPLE_CAROUSEL_D: f64 = 318.0;
const SAMPLE_CAROUSEL_Z: f64 = 42.0;
const SAMPLE_VIALS: usize = 16;
const SAMPLE_VIAL_D: f64 = 17.8;
const SAMPLE_VIAL_PITCH_RADIUS: f64 = 126.0;
const SAMPLE_SPLIT_PORTS: usize = 4;

const TEST_POCKET_X: f64 = 430.0;
const TEST_POCKET_Y: f64 = 168.0;
const TEST_POCKET_Z: f64 = 38.0;
const CONDUCTIVITY_CELL_COUNT: usize = 2;
const TOC_VIAL_COUNT: usize = 4;
const PROTEIN_SWAB_COUNT: usize = 8;

const DRAIN_BANK_X: f64 = 370.0;
const DRAIN_BANK_Y: f64 = 132.0;
const DRAIN_BANK_Z: f64 = 58.0;
const DRAIN_VALVES: usize = 5;
const DRAIN_BORE_D: f64 = 9.5;

const SCAN_LAND_X: f64 = 380.0;
const SCAN_LAND_Y: f64 = 122.0;
const SCAN_LAND_Z: f64 = 12.0;
const BARCODE_LANDS: usize = 6;
const RUN_RECORD_TAGS: usize = 4;

const ROBOT_KEEP_OUT_Z: f64 = 245.0;
const ROBOT_KEEP_OUT_ZONES: usize = 4;
const FRONT_SERVICE_CLEARANCE: f64 = 360.0;
const REAR_TUBE_SERVICE_CLEARANCE: f64 = 210.0;
const LEFT_OPERATOR_CLEARANCE: f64 = 160.0;
const RIGHT_VIAL_SERVICE_CLEARANCE: f64 = 180.0;

const DOCK_POS: (f64, f64) = (-156.0, 26.0);
const INLET_POS: (f64, f64) = (-82.0, STATION_Y / 2.0 - 86.0);
const RETURN_POS: (f64, f64) = (-82.0, -STATION_Y / 2.0 + 106.0);
const CAROUSEL_POS: (f64, f64) = (382.0, -94.0);
const TEST_POCKET_POS: (f64, f64) = (344.0, 176.0);
const DRAIN_POS: (f64, f64) = (-414.0, -214.0);
const SCAN_POS: (f64, f64) = (-390.0, 240.0);

fn main() {
    fs::create_dir_all("output").unwrap();

    let baseplate = baseplate();
    export(&baseplate, OUTPUTS[0]);

    let leak_tray = leak_tray();
    export(&leak_tray, OUTPUTS[1]);

    let dock = sealed_module_dock();
    export(&dock, OUTPUTS[2]);

    let inlet = sterile_rinse_inlet_manifold();
    export(&inlet, OUTPUTS[3]);

    let return_manifold = residual_return_manifold();
    export(&return_manifold, OUTPUTS[4]);

    let carousel = sample_vial_carousel();
    export(&carousel, OUTPUTS[5]);

    let tests = analytical_test_pockets();
    export(&tests, OUTPUTS[6]);

    let drain = drain_isolation_bank();
    export(&drain, OUTPUTS[7]);

    let scan = barcode_run_record_land();
    export(&scan, OUTPUTS[8]);

    let keepouts = robot_service_keepouts();
    export(&keepouts, OUTPUTS[9]);

    let assembly = baseplate
        + leak_tray.translate(0.0, -18.0, BASE_Z / 2.0 + LEAK_TRAY_Z / 2.0 + 2.0)
        + dock.translate(
            DOCK_POS.0,
            DOCK_POS.1,
            BASE_Z / 2.0 + LEAK_TRAY_Z + DOCK_Z / 2.0 + 8.0,
        )
        + inlet.translate(
            INLET_POS.0,
            INLET_POS.1,
            BASE_Z / 2.0 + INLET_MANIFOLD_Z / 2.0 + 34.0,
        )
        + return_manifold.translate(
            RETURN_POS.0,
            RETURN_POS.1,
            BASE_Z / 2.0 + RETURN_MANIFOLD_Z / 2.0 + 34.0,
        )
        + carousel.translate(
            CAROUSEL_POS.0,
            CAROUSEL_POS.1,
            BASE_Z / 2.0 + LEAK_TRAY_Z + SAMPLE_CAROUSEL_Z / 2.0 + 10.0,
        )
        + tests.translate(
            TEST_POCKET_POS.0,
            TEST_POCKET_POS.1,
            BASE_Z / 2.0 + TEST_POCKET_Z / 2.0 + 18.0,
        )
        + drain.translate(
            DRAIN_POS.0,
            DRAIN_POS.1,
            BASE_Z / 2.0 + DRAIN_BANK_Z / 2.0 + 24.0,
        )
        + scan.translate(
            SCAN_POS.0,
            SCAN_POS.1,
            BASE_Z / 2.0 + SCAN_LAND_Z / 2.0 + 4.0,
        )
        + closed_tube_route_placeholders()
        + keepouts.translate(0.0, 0.0, BASE_Z / 2.0 + ROBOT_KEEP_OUT_Z / 2.0 + 24.0);
    export(&assembly, OUTPUTS[10]);

    println!(
        "Closed-module residual rinse sampling station: {:.0}mm x {:.0}mm bench deck with {:.0}mm x {:.0}mm sealed dock, {} sterile rinse inlet ports, {} return ports, {}-position validated vial carousel, {} conductivity cells, {} TOC vial pockets, {} protein/swab pockets, {} leak sensor wells, {} drain isolation valves, {} barcode/run-record lands, and {} robot/service keepout envelopes.",
        STATION_X,
        STATION_Y,
        DOCK_X,
        DOCK_Y,
        RINSE_INLET_PORTS,
        RETURN_PORTS,
        SAMPLE_VIALS,
        CONDUCTIVITY_CELL_COUNT,
        TOC_VIAL_COUNT,
        PROTEIN_SWAB_COUNT,
        LEAK_SENSOR_WELLS,
        DRAIN_VALVES,
        BARCODE_LANDS + RUN_RECORD_TAGS,
        ROBOT_KEEP_OUT_ZONES
    );
    println!(
        "Service clearances modeled: {:.0}mm front vial/service, {:.0}mm rear sterile tubing, {:.0}mm left operator, {:.0}mm right vial handoff; keyed module alignment uses {} datum pins and {} clamp bosses.",
        FRONT_SERVICE_CLEARANCE,
        REAR_TUBE_SERVICE_CLEARANCE,
        LEFT_OPERATOR_CLEARANCE,
        RIGHT_VIAL_SERVICE_CLEARANCE,
        KEY_PIN_COUNT,
        DOCK_CLAMP_COUNT
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn baseplate() -> Part {
    let deck = centered_cube(
        "residual_rinse_sampling_station_baseplate",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );

    let leak_tray_socket = centered_cube(
        "residual_rinse_sampling_station_leak_tray_socket",
        LEAK_TRAY_X + 28.0,
        LEAK_TRAY_Y + 28.0,
        8.0,
    )
    .translate(0.0, -18.0, BASE_Z / 2.0 - 3.0);
    let dock_pocket = centered_cube(
        "residual_rinse_sampling_station_dock_registration_pocket",
        DOCK_X + 34.0,
        DOCK_Y + 32.0,
        7.0,
    )
    .translate(DOCK_POS.0, DOCK_POS.1, BASE_Z / 2.0 - 2.5);
    let wet_side_sump = centered_cube(
        "residual_rinse_sampling_station_wet_side_sump",
        STATION_X - 118.0,
        72.0,
        8.0,
    )
    .translate(0.0, RETURN_POS.1 - 18.0, BASE_Z / 2.0 - 3.0);
    let drain_through = centered_cylinder(
        "residual_rinse_sampling_station_base_drain_through",
        DRAIN_BORE_D / 2.0,
        42.0,
        30,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 88.0, -STATION_Y / 2.0 + 36.0, 0.0);

    deck - leak_tray_socket - dock_pocket - wet_side_sump - drain_through - base_mount_slots()
        + perimeter_rails()
        + keyed_locator_bosses()
        + robot_fiducials()
        + service_clearance_lands()
}

fn perimeter_rails() -> Part {
    let left = centered_cube(
        "residual_rinse_sampling_station_left_rail",
        22.0,
        STATION_Y - 74.0,
        RAIL_Z,
    )
    .translate(-(STATION_X / 2.0 - 34.0), 0.0, BASE_Z / 2.0 + RAIL_Z / 2.0);
    let right = centered_cube(
        "residual_rinse_sampling_station_right_rail",
        22.0,
        STATION_Y - 74.0,
        RAIL_Z,
    )
    .translate(STATION_X / 2.0 - 34.0, 0.0, BASE_Z / 2.0 + RAIL_Z / 2.0);
    let rear = centered_cube(
        "residual_rinse_sampling_station_rear_tube_guard_rail",
        STATION_X - 78.0,
        22.0,
        46.0,
    )
    .translate(0.0, STATION_Y / 2.0 - 34.0, BASE_Z / 2.0 + 23.0);
    let front = centered_cube(
        "residual_rinse_sampling_station_front_low_spill_lip",
        STATION_X - 104.0,
        18.0,
        28.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 33.0, BASE_Z / 2.0 + 14.0);

    left + right + rear + front
}

fn base_mount_slots() -> Part {
    let mut slots = Part::empty("residual_rinse_sampling_station_base_mount_slots");
    for (i, (x, y)) in base_mount_points().iter().enumerate() {
        let hole = centered_cylinder(
            format!("residual_rinse_sampling_station_m6_clearance_{i}"),
            MOUNT_HOLE_D / 2.0,
            BASE_Z + 4.0,
            24,
        )
        .translate(*x, *y, 0.0);
        let slot = centered_cube(
            format!("residual_rinse_sampling_station_m6_slot_{i}"),
            30.0,
            MOUNT_HOLE_D + 0.5,
            BASE_Z + 4.0,
        )
        .translate(*x, *y, 0.0);
        slots = slots + hole + slot;
    }
    slots
}

fn base_mount_points() -> [(f64, f64); 10] {
    [
        (-(STATION_X / 2.0 - 64.0), -(STATION_Y / 2.0 - 58.0)),
        (STATION_X / 2.0 - 64.0, -(STATION_Y / 2.0 - 58.0)),
        (-(STATION_X / 2.0 - 64.0), STATION_Y / 2.0 - 58.0),
        (STATION_X / 2.0 - 64.0, STATION_Y / 2.0 - 58.0),
        (0.0, -(STATION_Y / 2.0 - 58.0)),
        (0.0, STATION_Y / 2.0 - 58.0),
        (-(STATION_X / 2.0 - 64.0), 0.0),
        (STATION_X / 2.0 - 64.0, 0.0),
        (
            DOCK_POS.0 - DOCK_X / 2.0 + 68.0,
            DOCK_POS.1 + DOCK_Y / 2.0 - 48.0,
        ),
        (
            DOCK_POS.0 + DOCK_X / 2.0 - 68.0,
            DOCK_POS.1 - DOCK_Y / 2.0 + 48.0,
        ),
    ]
}

fn keyed_locator_bosses() -> Part {
    let mut bosses = Part::empty("residual_rinse_sampling_station_keyed_locator_bosses");
    for (i, (x, y, d)) in [
        (DOCK_POS.0 - 246.0, DOCK_POS.1 - 134.0, 18.0),
        (DOCK_POS.0 + 246.0, DOCK_POS.1 - 134.0, 14.0),
        (DOCK_POS.0 - 246.0, DOCK_POS.1 + 134.0, 14.0),
        (DOCK_POS.0 + 246.0, DOCK_POS.1 + 134.0, 18.0),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("residual_rinse_sampling_station_keyed_datum_boss_{i}"),
            *d / 2.0,
            16.0,
            30,
        )
        .translate(*x, *y, BASE_Z / 2.0 + 8.0);
        let pilot = centered_cylinder(
            format!("residual_rinse_sampling_station_keyed_datum_pilot_{i}"),
            (*d - 6.0) / 2.0,
            20.0,
            24,
        )
        .translate(*x, *y, BASE_Z / 2.0 + 8.0);
        bosses = bosses + (boss - pilot);
    }
    bosses
}

fn robot_fiducials() -> Part {
    let mut fiducials = Part::empty("residual_rinse_sampling_station_robot_fiducials");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 78.0), STATION_Y / 2.0 - 76.0),
        (STATION_X / 2.0 - 78.0, STATION_Y / 2.0 - 76.0),
        (-(STATION_X / 2.0 - 78.0), -(STATION_Y / 2.0 - 76.0)),
        (STATION_X / 2.0 - 78.0, -(STATION_Y / 2.0 - 76.0)),
    ]
    .iter()
    .enumerate()
    {
        fiducials =
            fiducials
                + fiducial_target(&format!("residual_rinse_sampling_station_fiducial_{i}"))
                    .translate(*x, *y, BASE_Z / 2.0 + 2.0);
    }
    fiducials
}

fn service_clearance_lands() -> Part {
    let front = centered_cube(
        "residual_rinse_sampling_station_front_service_clearance_land",
        STATION_X - 180.0,
        10.0,
        7.0,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 + FRONT_SERVICE_CLEARANCE,
        BASE_Z / 2.0 + 3.5,
    );
    let rear = centered_cube(
        "residual_rinse_sampling_station_rear_tube_service_clearance_land",
        STATION_X - 170.0,
        10.0,
        7.0,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - REAR_TUBE_SERVICE_CLEARANCE,
        BASE_Z / 2.0 + 3.5,
    );
    let left = centered_cube(
        "residual_rinse_sampling_station_left_operator_clearance_land",
        10.0,
        STATION_Y - 190.0,
        7.0,
    )
    .translate(
        -(STATION_X / 2.0 - LEFT_OPERATOR_CLEARANCE),
        0.0,
        BASE_Z / 2.0 + 3.5,
    );
    let right = centered_cube(
        "residual_rinse_sampling_station_right_vial_service_clearance_land",
        10.0,
        STATION_Y - 190.0,
        7.0,
    )
    .translate(
        STATION_X / 2.0 - RIGHT_VIAL_SERVICE_CLEARANCE,
        0.0,
        BASE_Z / 2.0 + 3.5,
    );

    front + rear + left + right
}

fn leak_tray() -> Part {
    let tray = centered_cube(
        "residual_rinse_sampling_station_leak_tray_outer",
        LEAK_TRAY_X,
        LEAK_TRAY_Y,
        LEAK_TRAY_Z,
    );
    let shallow_basin = centered_cube(
        "residual_rinse_sampling_station_leak_tray_shallow_basin",
        LEAK_TRAY_X - 52.0,
        LEAK_TRAY_Y - 52.0,
        18.0,
    )
    .translate(0.0, 0.0, LEAK_TRAY_Z / 2.0 - 7.0);
    let dock_shadow = centered_cube(
        "residual_rinse_sampling_station_leak_tray_module_shadow",
        DOCK_X + 28.0,
        DOCK_Y + 30.0,
        8.0,
    )
    .translate(DOCK_POS.0, DOCK_POS.1 + 18.0, LEAK_TRAY_Z / 2.0 - 3.0);
    let low_point_drain = centered_cylinder(
        "residual_rinse_sampling_station_leak_tray_low_point_drain",
        DRAIN_BORE_D / 2.0,
        46.0,
        30,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(LEAK_TRAY_X / 2.0 - 48.0, -(LEAK_TRAY_Y / 2.0 + 12.0), 0.0);

    tray - shallow_basin - dock_shadow - low_point_drain
        + leak_tray_flow_ribs()
        + leak_tray_sensor_wells()
        + leak_tray_lift_handles()
}

fn leak_tray_flow_ribs() -> Part {
    let mut ribs = Part::empty("residual_rinse_sampling_station_leak_tray_flow_ribs");
    for (i, y) in [-226.0, -136.0, -46.0, 44.0, 134.0, 224.0]
        .iter()
        .enumerate()
    {
        ribs = ribs
            + centered_cube(
                format!("residual_rinse_sampling_station_leak_tray_flow_rib_{i}"),
                LEAK_TRAY_X - 92.0,
                4.0,
                8.0,
            )
            .translate(0.0, *y, LEAK_TRAY_Z / 2.0 - 3.0);
    }
    for (i, x) in [-338.0, -202.0, -66.0, 70.0, 206.0, 342.0]
        .iter()
        .enumerate()
    {
        ribs = ribs
            + centered_cube(
                format!("residual_rinse_sampling_station_leak_tray_cross_rib_{i}"),
                4.0,
                LEAK_TRAY_Y - 104.0,
                7.0,
            )
            .translate(*x, 0.0, LEAK_TRAY_Z / 2.0 - 3.5);
    }
    ribs
}

fn leak_tray_sensor_wells() -> Part {
    let mut wells = Part::empty("residual_rinse_sampling_station_leak_sensor_wells");
    for (i, (x, y)) in [
        (-(LEAK_TRAY_X / 2.0 - 60.0), -(LEAK_TRAY_Y / 2.0 - 54.0)),
        (LEAK_TRAY_X / 2.0 - 60.0, -(LEAK_TRAY_Y / 2.0 - 54.0)),
        (-(LEAK_TRAY_X / 2.0 - 60.0), LEAK_TRAY_Y / 2.0 - 54.0),
        (LEAK_TRAY_X / 2.0 - 60.0, LEAK_TRAY_Y / 2.0 - 54.0),
    ]
    .iter()
    .enumerate()
    {
        let rim = centered_cube(
            format!("residual_rinse_sampling_station_leak_sensor_well_rim_{i}"),
            54.0,
            34.0,
            10.0,
        )
        .translate(*x, *y, LEAK_TRAY_Z / 2.0 + 5.0);
        let pocket = centered_cube(
            format!("residual_rinse_sampling_station_leak_sensor_well_pocket_{i}"),
            38.0,
            18.0,
            12.0,
        )
        .translate(*x, *y, LEAK_TRAY_Z / 2.0 + 5.0);
        wells = wells + (rim - pocket);
    }
    wells
}

fn leak_tray_lift_handles() -> Part {
    let left = lift_handle("residual_rinse_sampling_station_leak_tray_left_handle").translate(
        -(LEAK_TRAY_X / 2.0 + 8.0),
        0.0,
        LEAK_TRAY_Z / 2.0 + 12.0,
    );
    let right = lift_handle("residual_rinse_sampling_station_leak_tray_right_handle").translate(
        LEAK_TRAY_X / 2.0 + 8.0,
        0.0,
        LEAK_TRAY_Z / 2.0 + 12.0,
    );
    left + right
}

fn sealed_module_dock() -> Part {
    let lower_cradle = centered_cube(
        "residual_rinse_sampling_station_sealed_dock_lower_cradle",
        DOCK_X,
        DOCK_Y,
        DOCK_Z,
    );
    let module_clearance = centered_cube(
        "residual_rinse_sampling_station_sealed_dock_module_clearance",
        MODULE_ENVELOPE_X,
        MODULE_ENVELOPE_Y,
        MODULE_ENVELOPE_Z,
    )
    .translate(0.0, 0.0, DOCK_Z / 2.0 - MODULE_ENVELOPE_Z / 2.0 + 10.0);
    let o_ring_groove = centered_cube(
        "residual_rinse_sampling_station_sealed_dock_oring_groove",
        MODULE_ENVELOPE_X + 36.0,
        MODULE_ENVELOPE_Y + 34.0,
        8.0,
    )
    .translate(0.0, 0.0, DOCK_Z / 2.0 - 10.0);
    let access_window = centered_cube(
        "residual_rinse_sampling_station_sealed_dock_access_window",
        MODULE_ENVELOPE_X - 86.0,
        MODULE_ENVELOPE_Y - 68.0,
        16.0,
    )
    .translate(0.0, 0.0, DOCK_Z / 2.0 + 3.0);

    lower_cradle - module_clearance - o_ring_groove - access_window
        + dock_clamp_bosses()
        + dock_alignment_features()
        + dock_connector_towers()
        + dock_cover_frame()
}

fn dock_clamp_bosses() -> Part {
    let mut bosses = Part::empty("residual_rinse_sampling_station_dock_clamp_bosses");
    for (i, (x, y)) in [
        (-250.0, -164.0),
        (0.0, -164.0),
        (250.0, -164.0),
        (-250.0, 164.0),
        (0.0, 164.0),
        (250.0, 164.0),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("residual_rinse_sampling_station_dock_clamp_boss_{i}"),
            19.0,
            22.0,
            32,
        )
        .translate(*x, *y, DOCK_Z / 2.0 + 11.0);
        let screw = centered_cylinder(
            format!("residual_rinse_sampling_station_dock_clamp_screw_clearance_{i}"),
            4.6,
            26.0,
            24,
        )
        .translate(*x, *y, DOCK_Z / 2.0 + 11.0);
        bosses = bosses + (boss - screw);
    }
    bosses
}

fn dock_alignment_features() -> Part {
    let left_key = centered_cube(
        "residual_rinse_sampling_station_dock_left_asymmetric_key_rail",
        22.0,
        MODULE_ENVELOPE_Y + 58.0,
        34.0,
    )
    .translate(-(MODULE_ENVELOPE_X / 2.0 + 28.0), 0.0, DOCK_Z / 2.0 + 17.0);
    let right_key = centered_cube(
        "residual_rinse_sampling_station_dock_right_narrow_key_rail",
        14.0,
        MODULE_ENVELOPE_Y + 22.0,
        28.0,
    )
    .translate(MODULE_ENVELOPE_X / 2.0 + 30.0, 0.0, DOCK_Z / 2.0 + 14.0);

    let mut pins = Part::empty("residual_rinse_sampling_station_dock_key_pins");
    for (i, (x, y, d)) in [
        (
            -(MODULE_ENVELOPE_X / 2.0 - 34.0),
            -(MODULE_ENVELOPE_Y / 2.0 - 28.0),
            17.0,
        ),
        (
            MODULE_ENVELOPE_X / 2.0 - 34.0,
            -(MODULE_ENVELOPE_Y / 2.0 - 28.0),
            13.0,
        ),
        (
            -(MODULE_ENVELOPE_X / 2.0 - 34.0),
            MODULE_ENVELOPE_Y / 2.0 - 28.0,
            13.0,
        ),
        (
            MODULE_ENVELOPE_X / 2.0 - 34.0,
            MODULE_ENVELOPE_Y / 2.0 - 28.0,
            17.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        pins = pins
            + centered_cylinder(
                format!("residual_rinse_sampling_station_dock_key_pin_{i}"),
                *d / 2.0,
                28.0,
                30,
            )
            .translate(*x, *y, DOCK_Z / 2.0 + 14.0);
    }

    left_key + right_key + pins
}

fn dock_connector_towers() -> Part {
    let mut towers = Part::empty("residual_rinse_sampling_station_dock_connector_towers");
    for (i, y) in [-126.0, -84.0, -42.0, 0.0, 42.0, 84.0, 126.0]
        .iter()
        .enumerate()
    {
        let tower = centered_cube(
            format!("residual_rinse_sampling_station_dock_connector_tower_{i}"),
            44.0,
            24.0,
            58.0,
        )
        .translate(-(DOCK_X / 2.0 - 36.0), *y, DOCK_Z / 2.0 + 29.0);
        let bore = centered_cylinder(
            format!("residual_rinse_sampling_station_dock_connector_bore_{i}"),
            RINSE_INLET_PORT_D / 2.0,
            52.0,
            24,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(-(DOCK_X / 2.0 - 36.0), *y, DOCK_Z / 2.0 + 29.0);
        towers = towers + (tower - bore);
    }
    for (i, y) in [-126.0, -84.0, -42.0, 0.0, 42.0, 84.0, 126.0]
        .iter()
        .enumerate()
    {
        let tower = centered_cube(
            format!("residual_rinse_sampling_station_dock_return_tower_{i}"),
            44.0,
            24.0,
            58.0,
        )
        .translate(DOCK_X / 2.0 - 36.0, *y, DOCK_Z / 2.0 + 29.0);
        let bore = centered_cylinder(
            format!("residual_rinse_sampling_station_dock_return_bore_{i}"),
            RETURN_PORT_D / 2.0,
            52.0,
            24,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(DOCK_X / 2.0 - 36.0, *y, DOCK_Z / 2.0 + 29.0);
        towers = towers + (tower - bore);
    }
    towers
}

fn dock_cover_frame() -> Part {
    let outer = centered_cube(
        "residual_rinse_sampling_station_dock_sealed_cover_frame",
        DOCK_X + 42.0,
        DOCK_Y + 42.0,
        22.0,
    )
    .translate(0.0, 0.0, DOCK_Z / 2.0 + 36.0);
    let inner = centered_cube(
        "residual_rinse_sampling_station_dock_cover_frame_window",
        DOCK_X - 64.0,
        DOCK_Y - 64.0,
        26.0,
    )
    .translate(0.0, 0.0, DOCK_Z / 2.0 + 36.0);
    let gasket_witness = centered_cube(
        "residual_rinse_sampling_station_dock_cover_gasket_witness_line",
        DOCK_X - 20.0,
        DOCK_Y - 20.0,
        5.0,
    )
    .translate(0.0, 0.0, DOCK_Z / 2.0 + 48.0);
    outer - inner + gasket_witness
}

fn sterile_rinse_inlet_manifold() -> Part {
    let block = centered_cube(
        "residual_rinse_sampling_station_sterile_rinse_inlet_manifold_block",
        INLET_MANIFOLD_X,
        INLET_MANIFOLD_Y,
        INLET_MANIFOLD_Z,
    );
    let header = centered_cylinder(
        "residual_rinse_sampling_station_sterile_rinse_inlet_header_bore",
        8.0 / 2.0,
        INLET_MANIFOLD_X + 28.0,
        36,
    )
    .rotate(0.0, 90.0, 0.0);
    let feed_through = centered_cylinder(
        "residual_rinse_sampling_station_sterile_rinse_feed_through",
        12.0 / 2.0,
        INLET_MANIFOLD_Y + 22.0,
        34,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-(INLET_MANIFOLD_X / 2.0 - 46.0), 0.0, 0.0);

    block - header - feed_through - manifold_port_bores("inlet", RINSE_INLET_PORT_D)
        + inlet_filter_housings()
        + inlet_valve_actuator_bosses()
        + manifold_label_tabs("inlet")
}

fn residual_return_manifold() -> Part {
    let block = centered_cube(
        "residual_rinse_sampling_station_residual_return_manifold_block",
        RETURN_MANIFOLD_X,
        RETURN_MANIFOLD_Y,
        RETURN_MANIFOLD_Z,
    );
    let header = centered_cylinder(
        "residual_rinse_sampling_station_residual_return_header_bore",
        9.5 / 2.0,
        RETURN_MANIFOLD_X + 28.0,
        36,
    )
    .rotate(0.0, 90.0, 0.0);
    let outlet = centered_cylinder(
        "residual_rinse_sampling_station_residual_return_outlet_bore",
        13.0 / 2.0,
        RETURN_MANIFOLD_Y + 26.0,
        34,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(RETURN_MANIFOLD_X / 2.0 - 48.0, 0.0, 0.0);

    block - header - outlet - manifold_port_bores("return", RETURN_PORT_D)
        + return_check_valves()
        + sample_splitter_bosses()
        + manifold_label_tabs("return")
}

fn manifold_port_bores(kind: &str, diameter: f64) -> Part {
    let mut bores = Part::empty(format!("residual_rinse_sampling_station_{kind}_port_bores"));
    for i in 0..RINSE_INLET_PORTS {
        let x = port_x(i, RINSE_INLET_PORTS, RINSE_PORT_PITCH_X);
        bores = bores
            + centered_cylinder(
                format!("residual_rinse_sampling_station_{kind}_port_bore_{i}"),
                diameter / 2.0,
                86.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, 0.0);
    }
    bores
}

fn inlet_filter_housings() -> Part {
    let mut housings = Part::empty("residual_rinse_sampling_station_inlet_filter_housings");
    for (i, x) in [-302.0, -174.0, -46.0, 82.0, 210.0, 338.0]
        .iter()
        .enumerate()
    {
        let shell = centered_cylinder(
            format!("residual_rinse_sampling_station_inlet_filter_shell_{i}"),
            17.0,
            78.0,
            36,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, -58.0, INLET_MANIFOLD_Z / 2.0 + 16.0);
        let bore = centered_cylinder(
            format!("residual_rinse_sampling_station_inlet_filter_bore_{i}"),
            9.0,
            84.0,
            30,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, -58.0, INLET_MANIFOLD_Z / 2.0 + 16.0);
        housings = housings + (shell - bore);
    }
    housings
}

fn inlet_valve_actuator_bosses() -> Part {
    let mut bosses = Part::empty("residual_rinse_sampling_station_inlet_valve_actuator_bosses");
    for i in 0..RINSE_INLET_PORTS {
        let x = port_x(i, RINSE_INLET_PORTS, RINSE_PORT_PITCH_X);
        let boss = centered_cube(
            format!("residual_rinse_sampling_station_inlet_valve_actuator_pad_{i}"),
            38.0,
            22.0,
            18.0,
        )
        .translate(
            x,
            INLET_MANIFOLD_Y / 2.0 + 14.0,
            INLET_MANIFOLD_Z / 2.0 + 9.0,
        );
        let slot = centered_cube(
            format!("residual_rinse_sampling_station_inlet_valve_actuator_slot_{i}"),
            22.0,
            8.0,
            20.0,
        )
        .translate(
            x,
            INLET_MANIFOLD_Y / 2.0 + 14.0,
            INLET_MANIFOLD_Z / 2.0 + 9.0,
        );
        bosses = bosses + (boss - slot);
    }
    bosses
}

fn return_check_valves() -> Part {
    let mut valves = Part::empty("residual_rinse_sampling_station_return_check_valves");
    for i in 0..RETURN_PORTS {
        let x = port_x(i, RETURN_PORTS, RINSE_PORT_PITCH_X);
        let shell = centered_cylinder(
            format!("residual_rinse_sampling_station_return_check_valve_shell_{i}"),
            15.0,
            56.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            x,
            RETURN_MANIFOLD_Y / 2.0 + 30.0,
            RETURN_MANIFOLD_Z / 2.0 + 12.0,
        );
        let bore = centered_cylinder(
            format!("residual_rinse_sampling_station_return_check_valve_bore_{i}"),
            6.0,
            60.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            x,
            RETURN_MANIFOLD_Y / 2.0 + 30.0,
            RETURN_MANIFOLD_Z / 2.0 + 12.0,
        );
        valves = valves + (shell - bore);
    }
    valves
}

fn sample_splitter_bosses() -> Part {
    let mut splitters = Part::empty("residual_rinse_sampling_station_sample_splitter_bosses");
    for i in 0..SAMPLE_SPLIT_PORTS {
        let x = port_x(i, SAMPLE_SPLIT_PORTS, 118.0);
        let boss = centered_cube(
            format!("residual_rinse_sampling_station_sample_splitter_boss_{i}"),
            52.0,
            34.0,
            24.0,
        )
        .translate(
            x,
            -(RETURN_MANIFOLD_Y / 2.0 + 20.0),
            RETURN_MANIFOLD_Z / 2.0 + 12.0,
        );
        let bore = centered_cylinder(
            format!("residual_rinse_sampling_station_sample_splitter_bore_{i}"),
            5.2 / 2.0,
            44.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            x,
            -(RETURN_MANIFOLD_Y / 2.0 + 20.0),
            RETURN_MANIFOLD_Z / 2.0 + 12.0,
        );
        splitters = splitters + (boss - bore);
    }
    splitters
}

fn manifold_label_tabs(kind: &str) -> Part {
    let mut tabs = Part::empty(format!("residual_rinse_sampling_station_{kind}_label_tabs"));
    for i in 0..4 {
        let x = port_x(i, 4, 142.0);
        tabs = tabs
            + centered_cube(
                format!("residual_rinse_sampling_station_{kind}_label_tab_{i}"),
                86.0,
                3.0,
                22.0,
            )
            .translate(x, -58.0, 0.0);
    }
    tabs
}

fn sample_vial_carousel() -> Part {
    let disk = centered_cylinder(
        "residual_rinse_sampling_station_sample_vial_carousel_disk",
        SAMPLE_CAROUSEL_D / 2.0,
        SAMPLE_CAROUSEL_Z,
        96,
    );
    let center_hub = centered_cylinder(
        "residual_rinse_sampling_station_sample_vial_carousel_center_hub",
        54.0,
        SAMPLE_CAROUSEL_Z + 26.0,
        64,
    )
    .translate(0.0, 0.0, 13.0);
    let drive_bore = centered_cylinder(
        "residual_rinse_sampling_station_sample_vial_carousel_drive_bore",
        18.0,
        SAMPLE_CAROUSEL_Z + 32.0,
        48,
    )
    .translate(0.0, 0.0, 13.0);

    disk - sample_vial_pocket_bores() - drive_bore
        + center_hub
        + carousel_index_ring()
        + carousel_vial_presence_windows()
}

fn sample_vial_pocket_bores() -> Part {
    let mut bores = Part::empty("residual_rinse_sampling_station_sample_vial_pocket_bores");
    for i in 0..SAMPLE_VIALS {
        let angle = std::f64::consts::TAU * i as f64 / SAMPLE_VIALS as f64;
        let x = SAMPLE_VIAL_PITCH_RADIUS * angle.cos();
        let y = SAMPLE_VIAL_PITCH_RADIUS * angle.sin();
        bores = bores
            + centered_cylinder(
                format!("residual_rinse_sampling_station_vial_pocket_bore_{i}"),
                SAMPLE_VIAL_D / 2.0,
                SAMPLE_CAROUSEL_Z + 8.0,
                30,
            )
            .translate(x, y, 0.0);
    }
    bores
}

fn carousel_index_ring() -> Part {
    let ring_outer = centered_cylinder(
        "residual_rinse_sampling_station_carousel_index_ring_outer",
        SAMPLE_CAROUSEL_D / 2.0 + 18.0,
        12.0,
        96,
    )
    .translate(0.0, 0.0, SAMPLE_CAROUSEL_Z / 2.0 + 6.0);
    let ring_inner = centered_cylinder(
        "residual_rinse_sampling_station_carousel_index_ring_inner",
        SAMPLE_CAROUSEL_D / 2.0 - 7.0,
        14.0,
        96,
    )
    .translate(0.0, 0.0, SAMPLE_CAROUSEL_Z / 2.0 + 6.0);
    ring_outer - ring_inner + carousel_index_teeth()
}

fn carousel_index_teeth() -> Part {
    let mut teeth = Part::empty("residual_rinse_sampling_station_carousel_index_teeth");
    for i in 0..SAMPLE_VIALS {
        let angle = std::f64::consts::TAU * i as f64 / SAMPLE_VIALS as f64;
        let x = (SAMPLE_CAROUSEL_D / 2.0 + 8.0) * angle.cos();
        let y = (SAMPLE_CAROUSEL_D / 2.0 + 8.0) * angle.sin();
        teeth = teeth
            + centered_cube(
                format!("residual_rinse_sampling_station_carousel_index_tooth_{i}"),
                10.0,
                20.0,
                18.0,
            )
            .rotate(0.0, 0.0, angle.to_degrees())
            .translate(x, y, SAMPLE_CAROUSEL_Z / 2.0 + 9.0);
    }
    teeth
}

fn carousel_vial_presence_windows() -> Part {
    let mut windows = Part::empty("residual_rinse_sampling_station_carousel_presence_windows");
    for i in 0..SAMPLE_VIALS {
        let angle = std::f64::consts::TAU * (i as f64 + 0.5) / SAMPLE_VIALS as f64;
        let x = (SAMPLE_VIAL_PITCH_RADIUS - 30.0) * angle.cos();
        let y = (SAMPLE_VIAL_PITCH_RADIUS - 30.0) * angle.sin();
        windows = windows
            + centered_cube(
                format!("residual_rinse_sampling_station_carousel_presence_window_{i}"),
                18.0,
                5.0,
                16.0,
            )
            .rotate(0.0, 0.0, angle.to_degrees())
            .translate(x, y, SAMPLE_CAROUSEL_Z / 2.0 + 8.0);
    }
    windows
}

fn analytical_test_pockets() -> Part {
    let tray = centered_cube(
        "residual_rinse_sampling_station_analytical_test_pocket_tray",
        TEST_POCKET_X,
        TEST_POCKET_Y,
        TEST_POCKET_Z,
    );
    let sump = centered_cube(
        "residual_rinse_sampling_station_analytical_test_pocket_sump",
        TEST_POCKET_X - 36.0,
        TEST_POCKET_Y - 30.0,
        14.0,
    )
    .translate(0.0, 0.0, TEST_POCKET_Z / 2.0 - 6.0);

    tray - sump - conductivity_cell_bores() - toc_vial_bores() - protein_swab_slot_bores()
        + conductivity_cell_labels()
        + toc_vial_retainer_rails()
        + protein_swab_retainer_comb()
}

fn conductivity_cell_bores() -> Part {
    let mut bores = Part::empty("residual_rinse_sampling_station_conductivity_cell_bores");
    for (i, x) in [-164.0, -104.0].iter().enumerate() {
        bores = bores
            + centered_cube(
                format!("residual_rinse_sampling_station_conductivity_cell_pocket_{i}"),
                44.0,
                86.0,
                TEST_POCKET_Z + 8.0,
            )
            .translate(*x, 0.0, 0.0);
    }
    bores
}

fn toc_vial_bores() -> Part {
    let mut bores = Part::empty("residual_rinse_sampling_station_toc_vial_bores");
    for (i, x) in [-22.0, 34.0, 90.0, 146.0].iter().enumerate() {
        bores = bores
            + centered_cylinder(
                format!("residual_rinse_sampling_station_toc_vial_pocket_{i}"),
                13.5,
                TEST_POCKET_Z + 8.0,
                32,
            )
            .translate(*x, 38.0, 0.0);
    }
    bores
}

fn protein_swab_slot_bores() -> Part {
    let mut bores = Part::empty("residual_rinse_sampling_station_protein_swab_slot_bores");
    for i in 0..PROTEIN_SWAB_COUNT {
        let x = -22.0 + i as f64 * 42.0;
        bores = bores
            + centered_cube(
                format!("residual_rinse_sampling_station_protein_swab_slot_{i}"),
                28.0,
                58.0,
                TEST_POCKET_Z + 8.0,
            )
            .translate(x, -42.0, 0.0);
    }
    bores
}

fn conductivity_cell_labels() -> Part {
    let mut labels = Part::empty("residual_rinse_sampling_station_conductivity_labels");
    for (i, x) in [-164.0, -104.0].iter().enumerate() {
        labels = labels
            + centered_cube(
                format!("residual_rinse_sampling_station_conductivity_label_land_{i}"),
                46.0,
                4.0,
                9.0,
            )
            .translate(*x, TEST_POCKET_Y / 2.0 - 14.0, TEST_POCKET_Z / 2.0 + 4.5);
    }
    labels
}

fn toc_vial_retainer_rails() -> Part {
    let front = centered_cube(
        "residual_rinse_sampling_station_toc_vial_front_retainer_rail",
        210.0,
        6.0,
        16.0,
    )
    .translate(62.0, 12.0, TEST_POCKET_Z / 2.0 + 8.0);
    let rear = centered_cube(
        "residual_rinse_sampling_station_toc_vial_rear_retainer_rail",
        210.0,
        6.0,
        16.0,
    )
    .translate(62.0, 68.0, TEST_POCKET_Z / 2.0 + 8.0);
    front + rear
}

fn protein_swab_retainer_comb() -> Part {
    let mut comb = Part::empty("residual_rinse_sampling_station_protein_swab_retainer_comb");
    for i in 0..=PROTEIN_SWAB_COUNT {
        let x = -43.0 + i as f64 * 42.0;
        comb = comb
            + centered_cube(
                format!("residual_rinse_sampling_station_protein_swab_comb_tooth_{i}"),
                4.0,
                70.0,
                18.0,
            )
            .translate(x, -42.0, TEST_POCKET_Z / 2.0 + 9.0);
    }
    comb
}

fn drain_isolation_bank() -> Part {
    let block = centered_cube(
        "residual_rinse_sampling_station_inline_drain_isolation_block",
        DRAIN_BANK_X,
        DRAIN_BANK_Y,
        DRAIN_BANK_Z,
    );
    let waste_header = centered_cylinder(
        "residual_rinse_sampling_station_inline_drain_header_bore",
        DRAIN_BORE_D / 2.0,
        DRAIN_BANK_X + 28.0,
        36,
    )
    .rotate(0.0, 90.0, 0.0);
    let decon_outlet = centered_cylinder(
        "residual_rinse_sampling_station_inline_drain_decon_outlet_bore",
        13.0 / 2.0,
        DRAIN_BANK_Y + 22.0,
        34,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DRAIN_BANK_X / 2.0 - 40.0, 0.0, 0.0);

    block - waste_header - decon_outlet - drain_valve_bores()
        + drain_valve_actuator_pads()
        + drain_check_witness_windows()
}

fn drain_valve_bores() -> Part {
    let mut bores = Part::empty("residual_rinse_sampling_station_drain_valve_bores");
    for i in 0..DRAIN_VALVES {
        let x = port_x(i, DRAIN_VALVES, 64.0);
        bores = bores
            + centered_cylinder(
                format!("residual_rinse_sampling_station_drain_valve_bore_{i}"),
                DRAIN_BORE_D / 2.0,
                DRAIN_BANK_Y + 20.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, 0.0);
    }
    bores
}

fn drain_valve_actuator_pads() -> Part {
    let mut pads = Part::empty("residual_rinse_sampling_station_drain_valve_actuator_pads");
    for i in 0..DRAIN_VALVES {
        let x = port_x(i, DRAIN_VALVES, 64.0);
        let pad = centered_cube(
            format!("residual_rinse_sampling_station_drain_valve_actuator_pad_{i}"),
            42.0,
            34.0,
            22.0,
        )
        .translate(x, -DRAIN_BANK_Y / 2.0 - 17.0, DRAIN_BANK_Z / 2.0 + 11.0);
        let slot = centered_cube(
            format!("residual_rinse_sampling_station_drain_valve_actuator_slot_{i}"),
            24.0,
            10.0,
            24.0,
        )
        .translate(x, -DRAIN_BANK_Y / 2.0 - 17.0, DRAIN_BANK_Z / 2.0 + 11.0);
        pads = pads + (pad - slot);
    }
    pads
}

fn drain_check_witness_windows() -> Part {
    let mut windows = Part::empty("residual_rinse_sampling_station_drain_witness_windows");
    for i in 0..DRAIN_VALVES {
        let x = port_x(i, DRAIN_VALVES, 64.0);
        windows = windows
            + centered_cube(
                format!("residual_rinse_sampling_station_drain_witness_window_{i}"),
                34.0,
                4.0,
                18.0,
            )
            .translate(x, DRAIN_BANK_Y / 2.0 + 4.0, DRAIN_BANK_Z / 2.0 + 9.0);
    }
    windows
}

fn barcode_run_record_land() -> Part {
    let land = centered_cube(
        "residual_rinse_sampling_station_barcode_run_record_land",
        SCAN_LAND_X,
        SCAN_LAND_Y,
        SCAN_LAND_Z,
    );
    let recessed_scan_field = centered_cube(
        "residual_rinse_sampling_station_recessed_scan_field",
        SCAN_LAND_X - 30.0,
        SCAN_LAND_Y - 26.0,
        6.0,
    )
    .translate(0.0, 0.0, SCAN_LAND_Z / 2.0 - 2.5);

    land - recessed_scan_field + barcode_lands() + run_record_tag_slots() + scan_landing_fiducials()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("residual_rinse_sampling_station_barcode_lands");
    for i in 0..BARCODE_LANDS {
        let x = port_x(i, BARCODE_LANDS, 58.0);
        lands = lands
            + centered_cube(
                format!("residual_rinse_sampling_station_barcode_land_{i}"),
                46.0,
                18.0,
                4.0,
            )
            .translate(x, 22.0, SCAN_LAND_Z / 2.0 + 2.0);
    }
    lands
}

fn run_record_tag_slots() -> Part {
    let mut tags = Part::empty("residual_rinse_sampling_station_run_record_tag_slots");
    for i in 0..RUN_RECORD_TAGS {
        let x = port_x(i, RUN_RECORD_TAGS, 68.0);
        let frame = centered_cube(
            format!("residual_rinse_sampling_station_run_record_tag_frame_{i}"),
            52.0,
            28.0,
            8.0,
        )
        .translate(x, -30.0, SCAN_LAND_Z / 2.0 + 4.0);
        let pocket = centered_cube(
            format!("residual_rinse_sampling_station_run_record_tag_pocket_{i}"),
            42.0,
            18.0,
            10.0,
        )
        .translate(x, -30.0, SCAN_LAND_Z / 2.0 + 4.0);
        tags = tags + (frame - pocket);
    }
    tags
}

fn scan_landing_fiducials() -> Part {
    fiducial_target("residual_rinse_sampling_station_scan_fiducial_left").translate(
        -(SCAN_LAND_X / 2.0 - 28.0),
        SCAN_LAND_Y / 2.0 - 26.0,
        SCAN_LAND_Z / 2.0 + 2.0,
    ) + fiducial_target("residual_rinse_sampling_station_scan_fiducial_right").translate(
        SCAN_LAND_X / 2.0 - 28.0,
        SCAN_LAND_Y / 2.0 - 26.0,
        SCAN_LAND_Z / 2.0 + 2.0,
    )
}

fn robot_service_keepouts() -> Part {
    let dock_load = keepout_box(
        "residual_rinse_sampling_station_robot_keepout_module_load",
        DOCK_X + 160.0,
        DOCK_Y + 120.0,
        ROBOT_KEEP_OUT_Z,
    )
    .translate(DOCK_POS.0, DOCK_POS.1, 0.0);
    let vial_pick = keepout_box(
        "residual_rinse_sampling_station_robot_keepout_vial_pick",
        SAMPLE_CAROUSEL_D + 150.0,
        SAMPLE_CAROUSEL_D + 116.0,
        ROBOT_KEEP_OUT_Z - 24.0,
    )
    .translate(CAROUSEL_POS.0, CAROUSEL_POS.1, 0.0);
    let scan = keepout_box(
        "residual_rinse_sampling_station_robot_keepout_scan_head",
        SCAN_LAND_X + 124.0,
        SCAN_LAND_Y + 100.0,
        ROBOT_KEEP_OUT_Z - 70.0,
    )
    .translate(SCAN_POS.0, SCAN_POS.1, 0.0);
    let tube_service = keepout_box(
        "residual_rinse_sampling_station_robot_keepout_tube_service",
        INLET_MANIFOLD_X + 120.0,
        186.0,
        ROBOT_KEEP_OUT_Z - 44.0,
    )
    .translate(INLET_POS.0, INLET_POS.1, 0.0);

    dock_load + vial_pick + scan + tube_service
}

fn closed_tube_route_placeholders() -> Part {
    let inlet_to_dock = tube_span_x(
        "residual_rinse_sampling_station_tube_route_inlet_to_dock",
        INLET_MANIFOLD_X - 120.0,
    )
    .translate(-88.0, 234.0, BASE_Z / 2.0 + LEAK_TRAY_Z + DOCK_Z + 18.0);
    let dock_to_return = tube_span_y(
        "residual_rinse_sampling_station_tube_route_dock_to_return",
        430.0,
    )
    .translate(DOCK_POS.0 + DOCK_X / 2.0 + 54.0, -74.0, BASE_Z / 2.0 + 86.0);
    let return_to_carousel = tube_span_x(
        "residual_rinse_sampling_station_tube_route_return_to_carousel",
        460.0,
    )
    .translate(154.0, -266.0, BASE_Z / 2.0 + 82.0);
    let return_to_drain = tube_span_x(
        "residual_rinse_sampling_station_tube_route_return_to_drain",
        342.0,
    )
    .translate(-286.0, -306.0, BASE_Z / 2.0 + 80.0);
    inlet_to_dock + dock_to_return + return_to_carousel + return_to_drain
}

fn tube_span_x(name: &str, length: f64) -> Part {
    centered_cylinder(name, 3.6, length, 24).rotate(0.0, 90.0, 0.0)
}

fn tube_span_y(name: &str, length: f64) -> Part {
    centered_cylinder(name, 3.6, length, 24).rotate(90.0, 0.0, 0.0)
}

fn fiducial_target(name: &str) -> Part {
    let outer = centered_cylinder(format!("{name}_outer"), 14.0, 3.0, 40);
    let inner = centered_cylinder(format!("{name}_inner"), 6.0, 4.0, 32);
    let cross_x = centered_cube(format!("{name}_cross_x"), 34.0, 2.6, 4.0);
    let cross_y = centered_cube(format!("{name}_cross_y"), 2.6, 34.0, 4.0);
    outer - inner + cross_x + cross_y
}

fn lift_handle(name: &str) -> Part {
    let body = centered_cube(format!("{name}_body"), 24.0, 128.0, 34.0);
    let grip = centered_cube(format!("{name}_grip_cutout"), 28.0, 78.0, 18.0);
    body - grip
}

fn keepout_box(name: &str, x: f64, y: f64, z: f64) -> Part {
    let envelope = centered_cube(format!("{name}_envelope"), x, y, z);
    let interior = centered_cube(
        format!("{name}_interior_relief"),
        x - 18.0,
        y - 18.0,
        z + 4.0,
    );
    envelope - interior
}

fn port_x(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}
