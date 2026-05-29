use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed module gasket compression and seal validation fixture.
//
// Intent:
// - Verify sealed culture module gasket compression before a module enters the
//   isolator/incubator workflow.
// - Check latch engagement, clamp force witness points, pressure/vacuum decay
//   ports, dye/wetness leak evidence, traceability, and disposition status in
//   one closed-process bench fixture.
// - Keep this as packaging and datum CAD for purchased force sensors, pressure
//   transducers, camera/illumination, valves, and leak-test instrumentation.
//
// This is architecture/fit CAD only. It is not a validated leak-test protocol,
// elastomer material specification, pressure boundary rating, or sterile-barrier
// certification.

const OUTPUTS: [&str; 12] = [
    "output/closed_module_gasket_seal_validation_fixture_base_leak_tray.stl",
    "output/closed_module_gasket_seal_validation_fixture_module_datum_nest.stl",
    "output/closed_module_gasket_seal_validation_fixture_gasket_witness_compression_gauge_lands.stl",
    "output/closed_module_gasket_seal_validation_fixture_clamp_latch_force_check_pockets.stl",
    "output/closed_module_gasket_seal_validation_fixture_pressure_vacuum_decay_port_manifold.stl",
    "output/closed_module_gasket_seal_validation_fixture_dye_wetness_leak_witness_tray.stl",
    "output/closed_module_gasket_seal_validation_fixture_barcode_run_record_scan_lands.stl",
    "output/closed_module_gasket_seal_validation_fixture_pass_hold_reject_lanes.stl",
    "output/closed_module_gasket_seal_validation_fixture_calibration_shim_used_gasket_storage.stl",
    "output/closed_module_gasket_seal_validation_fixture_transparent_inspection_window_envelope.stl",
    "output/closed_module_gasket_seal_validation_fixture_robot_service_keepouts.stl",
    "output/closed_module_gasket_seal_validation_fixture_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 12] = [
    "module_datum_nest",
    "gasket_witness_compression_gauge_lands",
    "clamp_latch_force_check_pockets",
    "pressure_vacuum_decay_port_manifold",
    "dye_wetness_leak_witness_tray",
    "barcode_run_record_scan_lands",
    "pass_hold_reject_lanes",
    "calibration_shim_pockets",
    "used_gasket_quarantine_wells",
    "transparent_inspection_window_envelope",
    "robot_service_keepouts",
    "drainable_leak_containment",
];

const DECK_X: f64 = 1280.0;
const DECK_Y: f64 = 860.0;
const DECK_Z: f64 = 22.0;

const MODULE_X: f64 = 720.0;
const MODULE_Y: f64 = 520.0;
const MODULE_Z: f64 = 96.0;
const MODULE_CENTER_X: f64 = -150.0;
const MODULE_CENTER_Y: f64 = -20.0;

const NEST_X: f64 = 850.0;
const NEST_Y: f64 = 650.0;
const NEST_Z: f64 = 34.0;
const NEST_CENTER_X: f64 = MODULE_CENTER_X;
const NEST_CENTER_Y: f64 = MODULE_CENTER_Y;
const DATUM_PIN_D: f64 = 12.0;
const DATUM_PIN_Z: f64 = 20.0;
const MODULE_CLEARANCE: f64 = 1.0;

const GASKET_OUTER_X: f64 = 660.0;
const GASKET_OUTER_Y: f64 = 456.0;
const GASKET_TRACE_W: f64 = 16.0;
const GASKET_LAND_Z: f64 = 12.0;
const COMPRESSION_GAUGE_LANDS: usize = 16;
const COMPRESSION_STEP_COUNT: usize = 5;
const COMPRESSION_STEP_PITCH_X: f64 = 18.0;

const CLAMP_FORCE_POCKETS: usize = 8;
const LATCH_ENGAGEMENT_POCKETS: usize = 4;
const CLAMP_BLOCK_X: f64 = 76.0;
const CLAMP_BLOCK_Y: f64 = 52.0;
const CLAMP_BLOCK_Z: f64 = 48.0;
const LATCH_BLOCK_X: f64 = 96.0;
const LATCH_BLOCK_Y: f64 = 46.0;
const LATCH_BLOCK_Z: f64 = 46.0;

const MANIFOLD_X: f64 = 920.0;
const MANIFOLD_Y: f64 = 86.0;
const MANIFOLD_Z: f64 = 118.0;
const MANIFOLD_CENTER_X: f64 = -80.0;
const MANIFOLD_CENTER_Y: f64 = 332.0;
const DECAY_PORTS: usize = 12;
const DECAY_PORT_D: f64 = 8.0;
const SENSOR_POCKETS: usize = 6;
const VALVE_BLOCKS: usize = 8;

const WITNESS_TRAY_X: f64 = 900.0;
const WITNESS_TRAY_Y: f64 = 670.0;
const WITNESS_TRAY_Z: f64 = 28.0;
const WETNESS_SENSOR_WELLS: usize = 8;
const DYE_CHANNELS: usize = 4;
const DRAIN_D: f64 = 12.0;

const BARCODE_PANEL_X: f64 = 470.0;
const BARCODE_PANEL_Y: f64 = 118.0;
const BARCODE_PANEL_Z: f64 = 14.0;
const BARCODE_CENTER_X: f64 = -390.0;
const BARCODE_CENTER_Y: f64 = -356.0;
const BARCODE_LANDS: usize = 10;

const DISPOSITION_CENTER_X: f64 = 420.0;
const DISPOSITION_CENTER_Y: f64 = -250.0;
const DISPOSITION_LANE_X: f64 = 360.0;
const DISPOSITION_LANE_Y: f64 = 82.0;
const DISPOSITION_LANE_Z: f64 = 28.0;
const DISPOSITION_LANES: usize = 3;
const DISPOSITION_SLOTS_PER_LANE: usize = 4;

const STORAGE_CENTER_X: f64 = 420.0;
const STORAGE_CENTER_Y: f64 = 32.0;
const STORAGE_X: f64 = 390.0;
const STORAGE_Y: f64 = 284.0;
const STORAGE_Z: f64 = 30.0;
const CALIBRATION_SHIM_POCKETS: usize = 12;
const USED_GASKET_WELLS: usize = 12;
const SHIM_POCKET_X: f64 = 38.0;
const SHIM_POCKET_Y: f64 = 18.0;
const USED_GASKET_WELL_D: f64 = 24.0;

const WINDOW_X: f64 = 840.0;
const WINDOW_Y: f64 = 610.0;
const WINDOW_Z: f64 = 220.0;
const WINDOW_FRAME_W: f64 = 22.0;
const WINDOW_CENTER_X: f64 = MODULE_CENTER_X;
const WINDOW_CENTER_Y: f64 = MODULE_CENTER_Y;

const ROBOT_KEEP_OUT_ZONES: usize = 5;
const FRONT_ROBOT_APPROACH_Y: f64 = 150.0;
const REAR_SERVICE_CLEARANCE_Y: f64 = 170.0;
const SIDE_SERVICE_CLEARANCE_X: f64 = 150.0;
const TOP_CAMERA_CLEARANCE_Z: f64 = 150.0;
const GASKET_CHANGE_CLEARANCE_X: f64 = 220.0;

const MOUNT_HOLE_D: f64 = 6.6;
const FIDUCIAL_D: f64 = 18.0;

fn main() {
    fs::create_dir_all("output").unwrap();

    let base = base_leak_tray();
    export(&base, OUTPUTS[0]);

    let nest = module_datum_nest();
    export(&nest, OUTPUTS[1]);

    let gasket = gasket_witness_compression_gauge_lands();
    export(&gasket, OUTPUTS[2]);

    let clamp_latch = clamp_latch_force_check_pockets();
    export(&clamp_latch, OUTPUTS[3]);

    let manifold = pressure_vacuum_decay_port_manifold();
    export(&manifold, OUTPUTS[4]);

    let witness_tray = dye_wetness_leak_witness_tray();
    export(&witness_tray, OUTPUTS[5]);

    let barcode = barcode_run_record_scan_lands();
    export(&barcode, OUTPUTS[6]);

    let disposition = pass_hold_reject_lanes();
    export(&disposition, OUTPUTS[7]);

    let storage = calibration_shim_used_gasket_storage();
    export(&storage, OUTPUTS[8]);

    let inspection_window = transparent_inspection_window_envelope();
    export(&inspection_window, OUTPUTS[9]);

    let keepouts = robot_service_keepouts();
    export(&keepouts, OUTPUTS[10]);

    let assembly = base
        + witness_tray
        + nest
        + gasket
        + clamp_latch
        + manifold
        + barcode
        + disposition
        + storage
        + inspection_window
        + keepouts;
    export(&assembly, OUTPUTS[11]);

    println!(
        "Closed module gasket seal validation fixture: {:.0}mm x {:.0}mm deck, {:.0}mm x {:.0}mm sealed module datum, {} compression gauge lands, {} clamp-force pockets, {} latch engagement pockets, {} pressure/vacuum decay ports, {} wetness wells, {} shim pockets, {} used-gasket quarantine wells, and {} pass/hold/reject slots.",
        DECK_X,
        DECK_Y,
        MODULE_X,
        MODULE_Y,
        COMPRESSION_GAUGE_LANDS,
        CLAMP_FORCE_POCKETS,
        LATCH_ENGAGEMENT_POCKETS,
        DECAY_PORTS,
        WETNESS_SENSOR_WELLS,
        CALIBRATION_SHIM_POCKETS,
        USED_GASKET_WELLS,
        DISPOSITION_LANES * DISPOSITION_SLOTS_PER_LANE
    );
    println!(
        "Modeled {} validation control groups including module datum nest, gasket compression witness, clamp/latch force checks, pressure/vacuum decay manifold, dye/wetness tray, barcode/run-record lands, calibration shim storage, used-gasket quarantine, transparent inspection window envelope, and {} robot/service keepout envelopes.",
        REQUIRED_FEATURES.len(),
        ROBOT_KEEP_OUT_ZONES
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn base_leak_tray() -> Part {
    let deck = centered_cube("gasket_seal_fixture_deck", DECK_X, DECK_Y, DECK_Z);
    let recessed_field = centered_cube(
        "gasket_seal_fixture_recessed_component_field",
        DECK_X - 112.0,
        DECK_Y - 110.0,
        7.0,
    )
    .translate(0.0, -10.0, DECK_Z / 2.0 - 3.0);
    let witness_tray_socket = centered_cube(
        "gasket_seal_fixture_witness_tray_socket",
        WITNESS_TRAY_X + 30.0,
        WITNESS_TRAY_Y + 28.0,
        8.0,
    )
    .translate(MODULE_CENTER_X, MODULE_CENTER_Y, DECK_Z / 2.0 - 3.0);
    let manifold_socket = centered_cube(
        "gasket_seal_fixture_manifold_socket",
        MANIFOLD_X + 26.0,
        MANIFOLD_Y + 24.0,
        8.0,
    )
    .translate(MANIFOLD_CENTER_X, MANIFOLD_CENTER_Y, DECK_Z / 2.0 - 3.0);
    let drain = centered_cylinder(
        "gasket_seal_fixture_base_low_point_drain",
        DRAIN_D / 2.0,
        42.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        MODULE_CENTER_X + WITNESS_TRAY_X / 2.0 - 46.0,
        -(DECK_Y / 2.0 + 1.0),
        0.0,
    );

    deck - recessed_field - witness_tray_socket - manifold_socket - drain - deck_mount_holes()
        + base_perimeter_lips()
        + base_mount_bosses()
        + robot_fiducials()
        + base_drain_guard()
}

fn base_perimeter_lips() -> Part {
    let rear = centered_cube(
        "gasket_seal_fixture_rear_wipe_lip",
        DECK_X - 76.0,
        18.0,
        36.0,
    )
    .translate(0.0, DECK_Y / 2.0 - 34.0, DECK_Z / 2.0 + 18.0);
    let front = centered_cube(
        "gasket_seal_fixture_front_witness_lip",
        DECK_X - 128.0,
        14.0,
        24.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 32.0, DECK_Z / 2.0 + 12.0);
    let left = centered_cube(
        "gasket_seal_fixture_left_retaining_lip",
        18.0,
        DECK_Y - 92.0,
        32.0,
    )
    .translate(-DECK_X / 2.0 + 34.0, 0.0, DECK_Z / 2.0 + 16.0);
    let right = centered_cube(
        "gasket_seal_fixture_right_retaining_lip",
        18.0,
        DECK_Y - 92.0,
        32.0,
    )
    .translate(DECK_X / 2.0 - 34.0, 0.0, DECK_Z / 2.0 + 16.0);

    rear + front + left + right
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("gasket_seal_fixture_deck_mount_holes");
    for (i, (x, y)) in deck_mount_points().iter().enumerate() {
        let round = centered_cylinder(
            format!("gasket_seal_fixture_m6_clearance_{i}"),
            MOUNT_HOLE_D / 2.0,
            DECK_Z + 4.0,
            24,
        )
        .translate(*x, *y, 0.0);
        let slot = centered_cube(
            format!("gasket_seal_fixture_m6_slot_relief_{i}"),
            28.0,
            MOUNT_HOLE_D + 0.5,
            DECK_Z + 4.0,
        )
        .translate(*x, *y, 0.0);
        holes = holes + round + slot;
    }
    holes
}

fn base_mount_bosses() -> Part {
    let mut bosses = Part::empty("gasket_seal_fixture_mount_bosses");
    for (i, (x, y)) in deck_mount_points().iter().enumerate() {
        let boss = centered_cylinder(
            format!("gasket_seal_fixture_mount_boss_{i}"),
            16.0,
            10.0,
            32,
        )
        .translate(*x, *y, -DECK_Z / 2.0 + 7.0);
        let hole = centered_cylinder(
            format!("gasket_seal_fixture_mount_boss_clearance_{i}"),
            MOUNT_HOLE_D / 2.0,
            12.0,
            24,
        )
        .translate(*x, *y, -DECK_Z / 2.0 + 7.0);
        bosses = bosses + (boss - hole);
    }
    bosses
}

fn robot_fiducials() -> Part {
    let mut fiducials = Part::empty("gasket_seal_fixture_robot_fiducials");
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 86.0, -DECK_Y / 2.0 + 86.0),
        (DECK_X / 2.0 - 86.0, -DECK_Y / 2.0 + 86.0),
        (-DECK_X / 2.0 + 86.0, DECK_Y / 2.0 - 86.0),
        (DECK_X / 2.0 - 86.0, DECK_Y / 2.0 - 86.0),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + fiducial_target(&format!("gasket_seal_fixture_base_fiducial_{i}")).translate(
                *x,
                *y,
                DECK_Z / 2.0 + 2.0,
            );
    }
    fiducials
}

fn base_drain_guard() -> Part {
    let guard = centered_cube(
        "gasket_seal_fixture_drain_guard_wetness_sensor_cover",
        132.0,
        52.0,
        28.0,
    )
    .translate(
        MODULE_CENTER_X + WITNESS_TRAY_X / 2.0 - 46.0,
        -DECK_Y / 2.0 + 45.0,
        DECK_Z / 2.0 + 14.0,
    );
    let access = centered_cylinder(
        "gasket_seal_fixture_drain_guard_access_bore",
        20.0,
        56.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        MODULE_CENTER_X + WITNESS_TRAY_X / 2.0 - 46.0,
        -DECK_Y / 2.0 + 45.0,
        DECK_Z / 2.0 + 14.0,
    );
    guard - access
}

fn module_datum_nest() -> Part {
    let plate = centered_cube(
        "gasket_seal_fixture_module_nest_plate",
        NEST_X,
        NEST_Y,
        NEST_Z,
    )
    .translate(
        NEST_CENTER_X,
        NEST_CENTER_Y,
        DECK_Z / 2.0 + WITNESS_TRAY_Z + NEST_Z / 2.0 + 5.0,
    );
    let module_pocket = centered_cube(
        "gasket_seal_fixture_module_clearance_pocket",
        MODULE_X + MODULE_CLEARANCE * 2.0,
        MODULE_Y + MODULE_CLEARANCE * 2.0,
        NEST_Z + 6.0,
    )
    .translate(
        MODULE_CENTER_X,
        MODULE_CENTER_Y,
        DECK_Z / 2.0 + WITNESS_TRAY_Z + NEST_Z / 2.0 + 12.0,
    );
    let gasket_shadow = centered_cube(
        "gasket_seal_fixture_module_gasket_shadow_relief",
        GASKET_OUTER_X + 36.0,
        GASKET_OUTER_Y + 36.0,
        9.0,
    )
    .translate(
        MODULE_CENTER_X,
        MODULE_CENTER_Y,
        DECK_Z / 2.0 + WITNESS_TRAY_Z + NEST_Z + 4.0,
    );

    plate - module_pocket - gasket_shadow
        + datum_rails()
        + datum_pins_and_hard_stops()
        + module_present_sensor_flags()
}

fn datum_rails() -> Part {
    let rail_z = 38.0;
    let rail_y = MODULE_Y + 78.0;
    let rail_offset_x = MODULE_X / 2.0 + 36.0;
    let z = DECK_Z / 2.0 + WITNESS_TRAY_Z + NEST_Z + rail_z / 2.0 + 5.0;
    let left = datum_rail("left", rail_y, rail_z).translate(
        MODULE_CENTER_X - rail_offset_x,
        MODULE_CENTER_Y,
        z,
    );
    let right = datum_rail("right", rail_y, rail_z).translate(
        MODULE_CENTER_X + rail_offset_x,
        MODULE_CENTER_Y,
        z,
    );
    let rear = centered_cube(
        "gasket_seal_fixture_rear_module_hard_stop",
        MODULE_X + 90.0,
        26.0,
        rail_z,
    )
    .translate(MODULE_CENTER_X, MODULE_CENTER_Y + rail_y / 2.0 - 13.0, z);
    let front_safety_lip = centered_cube(
        "gasket_seal_fixture_front_module_safety_lip",
        MODULE_X + 70.0,
        18.0,
        24.0,
    )
    .translate(
        MODULE_CENTER_X,
        MODULE_CENTER_Y - rail_y / 2.0 + 18.0,
        z - 5.0,
    );
    left + right + rear + front_safety_lip
}

fn datum_rail(name: &str, rail_y: f64, rail_z: f64) -> Part {
    let body = centered_cube(
        format!("gasket_seal_fixture_{name}_datum_rail"),
        36.0,
        rail_y,
        rail_z,
    );
    let relief = centered_cube(
        format!("gasket_seal_fixture_{name}_datum_rail_top_relief"),
        18.0,
        rail_y - 48.0,
        rail_z + 2.0,
    )
    .translate(0.0, 0.0, 12.0);
    let mut latch_windows = Part::empty(format!("gasket_seal_fixture_{name}_rail_latch_windows"));
    for (i, y) in [-210.0, 0.0, 210.0].iter().enumerate() {
        latch_windows = latch_windows
            + centered_cube(
                format!("gasket_seal_fixture_{name}_rail_latch_window_{i}"),
                40.0,
                34.0,
                rail_z + 4.0,
            )
            .translate(0.0, *y, 0.0);
    }
    body - relief - latch_windows
}

fn datum_pins_and_hard_stops() -> Part {
    let mut features = Part::empty("gasket_seal_fixture_module_datum_pins_hard_stops");
    for (i, (x, y)) in module_datum_points().iter().enumerate() {
        let pin = centered_cylinder(
            format!("gasket_seal_fixture_datum_pin_{i}"),
            DATUM_PIN_D / 2.0,
            DATUM_PIN_Z,
            32,
        )
        .translate(
            MODULE_CENTER_X + *x,
            MODULE_CENTER_Y + *y,
            DECK_Z / 2.0 + WITNESS_TRAY_Z + NEST_Z + DATUM_PIN_Z / 2.0 + 7.0,
        );
        let lead_in = centered_cylinder(
            format!("gasket_seal_fixture_datum_pin_lead_in_{i}"),
            9.0,
            4.0,
            32,
        )
        .translate(
            MODULE_CENTER_X + *x,
            MODULE_CENTER_Y + *y,
            DECK_Z / 2.0 + WITNESS_TRAY_Z + NEST_Z + DATUM_PIN_Z + 11.0,
        );
        features = features + pin + lead_in;
    }

    for (i, x) in [-(MODULE_X / 2.0 - 72.0), MODULE_X / 2.0 - 72.0]
        .iter()
        .enumerate()
    {
        features = features
            + centered_cube(
                format!("gasket_seal_fixture_rear_kinematic_stop_{i}"),
                72.0,
                30.0,
                34.0,
            )
            .translate(
                MODULE_CENTER_X + *x,
                MODULE_CENTER_Y + MODULE_Y / 2.0 + 30.0,
                DECK_Z / 2.0 + WITNESS_TRAY_Z + NEST_Z + 22.0,
            );
    }

    features
}

fn module_present_sensor_flags() -> Part {
    let mut flags = Part::empty("gasket_seal_fixture_module_present_sensor_flags");
    for (i, (x, y)) in [
        (-(MODULE_X / 2.0 - 44.0), -(MODULE_Y / 2.0 - 38.0)),
        (MODULE_X / 2.0 - 44.0, -(MODULE_Y / 2.0 - 38.0)),
        (0.0, MODULE_Y / 2.0 - 40.0),
    ]
    .iter()
    .enumerate()
    {
        flags = flags
            + centered_cube(
                format!("gasket_seal_fixture_module_present_optical_flag_{i}"),
                34.0,
                22.0,
                24.0,
            )
            .translate(
                MODULE_CENTER_X + *x,
                MODULE_CENTER_Y + *y,
                DECK_Z / 2.0 + WITNESS_TRAY_Z + NEST_Z + 18.0,
            );
    }
    flags
}

fn gasket_witness_compression_gauge_lands() -> Part {
    let z = DECK_Z / 2.0 + WITNESS_TRAY_Z + NEST_Z + 12.0;
    let top = centered_cube(
        "gasket_seal_fixture_gasket_witness_top_rail",
        GASKET_OUTER_X,
        GASKET_TRACE_W,
        GASKET_LAND_Z,
    )
    .translate(MODULE_CENTER_X, MODULE_CENTER_Y + GASKET_OUTER_Y / 2.0, z);
    let bottom = centered_cube(
        "gasket_seal_fixture_gasket_witness_bottom_rail",
        GASKET_OUTER_X,
        GASKET_TRACE_W,
        GASKET_LAND_Z,
    )
    .translate(MODULE_CENTER_X, MODULE_CENTER_Y - GASKET_OUTER_Y / 2.0, z);
    let left = centered_cube(
        "gasket_seal_fixture_gasket_witness_left_rail",
        GASKET_TRACE_W,
        GASKET_OUTER_Y,
        GASKET_LAND_Z,
    )
    .translate(MODULE_CENTER_X - GASKET_OUTER_X / 2.0, MODULE_CENTER_Y, z);
    let right = centered_cube(
        "gasket_seal_fixture_gasket_witness_right_rail",
        GASKET_TRACE_W,
        GASKET_OUTER_Y,
        GASKET_LAND_Z,
    )
    .translate(MODULE_CENTER_X + GASKET_OUTER_X / 2.0, MODULE_CENTER_Y, z);

    top + bottom + left + right + compression_gauge_land_array() + stepped_feeler_gauges()
}

fn compression_gauge_land_array() -> Part {
    let mut lands = Part::empty("gasket_seal_fixture_compression_gauge_lands");
    for i in 0..COMPRESSION_GAUGE_LANDS {
        let (x, y) = compression_land_xy(i);
        let land = centered_cube(
            format!("gasket_seal_fixture_compression_witness_land_{i}"),
            42.0,
            24.0,
            8.0,
        )
        .translate(
            MODULE_CENTER_X + x,
            MODULE_CENTER_Y + y,
            DECK_Z / 2.0 + WITNESS_TRAY_Z + NEST_Z + 28.0,
        );
        let witness_pin = centered_cylinder(
            format!("gasket_seal_fixture_compression_witness_probe_relief_{i}"),
            4.0,
            12.0,
            24,
        )
        .translate(
            MODULE_CENTER_X + x,
            MODULE_CENTER_Y + y,
            DECK_Z / 2.0 + WITNESS_TRAY_Z + NEST_Z + 29.0,
        );
        lands = lands + (land - witness_pin);
    }
    lands
}

fn stepped_feeler_gauges() -> Part {
    let mut gauges = Part::empty("gasket_seal_fixture_stepped_compression_feeler_gauges");
    let start_x =
        MODULE_CENTER_X - (COMPRESSION_STEP_COUNT as f64 - 1.0) * COMPRESSION_STEP_PITCH_X / 2.0;
    for i in 0..COMPRESSION_STEP_COUNT {
        let height = 2.0 + i as f64 * 0.6;
        gauges = gauges
            + centered_cube(
                format!("gasket_seal_fixture_compression_step_{i}_{height:.1}mm"),
                14.0,
                64.0,
                height,
            )
            .translate(
                start_x + i as f64 * COMPRESSION_STEP_PITCH_X,
                MODULE_CENTER_Y - GASKET_OUTER_Y / 2.0 - 46.0,
                DECK_Z / 2.0 + WITNESS_TRAY_Z + NEST_Z + height / 2.0 + 8.0,
            );
    }
    gauges
}

fn clamp_latch_force_check_pockets() -> Part {
    clamp_force_sensor_pockets() + latch_engagement_gauges() + clamp_force_reference_bar()
}

fn clamp_force_sensor_pockets() -> Part {
    let mut pockets = Part::empty("gasket_seal_fixture_clamp_force_sensor_pockets");
    for i in 0..CLAMP_FORCE_POCKETS {
        let (x, y, rot) = clamp_pocket_xy_rot(i);
        let block = centered_cube(
            format!("gasket_seal_fixture_clamp_force_block_{i}"),
            CLAMP_BLOCK_X,
            CLAMP_BLOCK_Y,
            CLAMP_BLOCK_Z,
        )
        .translate(x, y, DECK_Z / 2.0 + CLAMP_BLOCK_Z / 2.0 + 12.0);
        let load_cell_pocket = centered_cube(
            format!("gasket_seal_fixture_clamp_load_cell_pocket_{i}"),
            42.0,
            24.0,
            CLAMP_BLOCK_Z + 4.0,
        )
        .translate(x, y, DECK_Z / 2.0 + CLAMP_BLOCK_Z / 2.0 + 16.0);
        let cam_bore = centered_cylinder(
            format!("gasket_seal_fixture_clamp_cam_bore_{i}"),
            8.0,
            CLAMP_BLOCK_X + 8.0,
            28,
        )
        .rotate(0.0, 90.0, rot)
        .translate(x, y, DECK_Z / 2.0 + CLAMP_BLOCK_Z / 2.0 + 12.0);
        pockets = pockets + (block - load_cell_pocket - cam_bore);
    }
    pockets
}

fn latch_engagement_gauges() -> Part {
    let mut gauges = Part::empty("gasket_seal_fixture_latch_engagement_gauges");
    for i in 0..LATCH_ENGAGEMENT_POCKETS {
        let x = MODULE_CENTER_X
            + match i {
                0 => -(MODULE_X / 2.0 - 118.0),
                1 => MODULE_X / 2.0 - 118.0,
                2 => -(MODULE_X / 2.0 - 118.0),
                _ => MODULE_X / 2.0 - 118.0,
            };
        let y = MODULE_CENTER_Y
            + if i < 2 {
                -(MODULE_Y / 2.0 + 62.0)
            } else {
                MODULE_Y / 2.0 + 62.0
            };
        let block = centered_cube(
            format!("gasket_seal_fixture_latch_engagement_block_{i}"),
            LATCH_BLOCK_X,
            LATCH_BLOCK_Y,
            LATCH_BLOCK_Z,
        )
        .translate(x, y, DECK_Z / 2.0 + LATCH_BLOCK_Z / 2.0 + 14.0);
        let depth_window = centered_cube(
            format!("gasket_seal_fixture_latch_depth_window_{i}"),
            58.0,
            12.0,
            LATCH_BLOCK_Z + 6.0,
        )
        .translate(x, y, DECK_Z / 2.0 + LATCH_BLOCK_Z / 2.0 + 16.0);
        let witness_flag = centered_cube(
            format!("gasket_seal_fixture_latch_green_flag_land_{i}"),
            34.0,
            8.0,
            14.0,
        )
        .translate(x, y + if i < 2 { -28.0 } else { 28.0 }, DECK_Z / 2.0 + 46.0);
        gauges = gauges + (block - depth_window) + witness_flag;
    }
    gauges
}

fn clamp_force_reference_bar() -> Part {
    let bar = centered_cube(
        "gasket_seal_fixture_clamp_force_reference_bar",
        520.0,
        32.0,
        18.0,
    )
    .translate(
        MODULE_CENTER_X,
        MODULE_CENTER_Y + MODULE_Y / 2.0 + 110.0,
        DECK_Z / 2.0 + 40.0,
    );
    let mut marks = Part::empty("gasket_seal_fixture_clamp_force_reference_marks");
    for i in 0..9 {
        marks = marks
            + centered_cube(
                format!("gasket_seal_fixture_clamp_force_tick_{i}"),
                4.0,
                40.0,
                20.0,
            )
            .translate(
                MODULE_CENTER_X - 220.0 + i as f64 * 55.0,
                MODULE_CENTER_Y + MODULE_Y / 2.0 + 110.0,
                DECK_Z / 2.0 + 41.0,
            );
    }
    bar + marks
}

fn pressure_vacuum_decay_port_manifold() -> Part {
    let body = centered_cube(
        "gasket_seal_fixture_pressure_vacuum_manifold_body",
        MANIFOLD_X,
        MANIFOLD_Y,
        MANIFOLD_Z,
    )
    .translate(
        MANIFOLD_CENTER_X,
        MANIFOLD_CENTER_Y,
        DECK_Z / 2.0 + MANIFOLD_Z / 2.0 + 10.0,
    );
    let mut port_cuts = Part::empty("gasket_seal_fixture_decay_port_bores");
    for i in 0..DECAY_PORTS {
        let x = decay_port_x(i);
        let z = if i % 2 == 0 { 78.0 } else { 44.0 };
        port_cuts = port_cuts
            + centered_cylinder(
                format!("gasket_seal_fixture_decay_port_bore_{i}"),
                DECAY_PORT_D / 2.0,
                MANIFOLD_Y + 10.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(MANIFOLD_CENTER_X + x, MANIFOLD_CENTER_Y, DECK_Z / 2.0 + z);
    }

    body - port_cuts
        + manifold_valve_blocks()
        + manifold_sensor_pockets()
        + manifold_tube_strain_reliefs()
}

fn manifold_valve_blocks() -> Part {
    let mut valves = Part::empty("gasket_seal_fixture_manifold_valve_blocks");
    for i in 0..VALVE_BLOCKS {
        valves = valves
            + centered_cube(
                format!("gasket_seal_fixture_decay_isolation_valve_block_{i}"),
                48.0,
                34.0,
                28.0,
            )
            .translate(
                MANIFOLD_CENTER_X - 350.0 + i as f64 * 100.0,
                MANIFOLD_CENTER_Y - MANIFOLD_Y / 2.0 - 28.0,
                DECK_Z / 2.0 + MANIFOLD_Z - 10.0,
            );
    }
    valves
}

fn manifold_sensor_pockets() -> Part {
    let mut sensors = Part::empty("gasket_seal_fixture_manifold_sensor_pockets");
    for i in 0..SENSOR_POCKETS {
        let pocket = centered_cube(
            format!("gasket_seal_fixture_pressure_sensor_pocket_{i}"),
            58.0,
            30.0,
            24.0,
        )
        .translate(
            MANIFOLD_CENTER_X - 300.0 + i as f64 * 120.0,
            MANIFOLD_CENTER_Y + MANIFOLD_Y / 2.0 + 22.0,
            DECK_Z / 2.0 + MANIFOLD_Z - 22.0,
        );
        let sensor_tap = centered_cylinder(
            format!("gasket_seal_fixture_pressure_sensor_tap_{i}"),
            3.2,
            38.0,
            20,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            MANIFOLD_CENTER_X - 300.0 + i as f64 * 120.0,
            MANIFOLD_CENTER_Y + MANIFOLD_Y / 2.0 + 22.0,
            DECK_Z / 2.0 + MANIFOLD_Z - 22.0,
        );
        sensors = sensors + (pocket - sensor_tap);
    }
    sensors
}

fn manifold_tube_strain_reliefs() -> Part {
    let mut combs = Part::empty("gasket_seal_fixture_manifold_tube_strain_relief_combs");
    for side in 0..2 {
        let y = MANIFOLD_CENTER_Y + if side == 0 { -94.0 } else { 94.0 };
        let mut comb = centered_cube(
            format!("gasket_seal_fixture_manifold_tube_comb_{side}"),
            MANIFOLD_X - 120.0,
            24.0,
            24.0,
        )
        .translate(MANIFOLD_CENTER_X, y, DECK_Z / 2.0 + 38.0);
        for i in 0..DECAY_PORTS {
            comb = comb
                - centered_cylinder(
                    format!("gasket_seal_fixture_manifold_tube_comb_slot_{side}_{i}"),
                    4.2,
                    30.0,
                    20,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(
                    MANIFOLD_CENTER_X + decay_port_x(i),
                    y,
                    DECK_Z / 2.0 + 38.0,
                );
        }
        combs = combs + comb;
    }
    combs
}

fn dye_wetness_leak_witness_tray() -> Part {
    let tray = centered_cube(
        "gasket_seal_fixture_dye_wetness_witness_tray_outer",
        WITNESS_TRAY_X,
        WITNESS_TRAY_Y,
        WITNESS_TRAY_Z,
    )
    .translate(
        MODULE_CENTER_X,
        MODULE_CENTER_Y,
        DECK_Z / 2.0 + WITNESS_TRAY_Z / 2.0 + 3.0,
    );
    let basin = centered_cube(
        "gasket_seal_fixture_dye_wetness_shallow_basin",
        WITNESS_TRAY_X - 54.0,
        WITNESS_TRAY_Y - 54.0,
        14.0,
    )
    .translate(
        MODULE_CENTER_X,
        MODULE_CENTER_Y,
        DECK_Z / 2.0 + WITNESS_TRAY_Z / 2.0 + 8.0,
    );
    let module_shadow = centered_cube(
        "gasket_seal_fixture_dye_wetness_module_shadow",
        MODULE_X + 60.0,
        MODULE_Y + 58.0,
        8.0,
    )
    .translate(
        MODULE_CENTER_X,
        MODULE_CENTER_Y,
        DECK_Z / 2.0 + WITNESS_TRAY_Z + 4.0,
    );
    let drain = centered_cylinder(
        "gasket_seal_fixture_dye_wetness_tray_low_point_drain",
        DRAIN_D / 2.0,
        44.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        MODULE_CENTER_X + WITNESS_TRAY_X / 2.0 - 46.0,
        MODULE_CENTER_Y - WITNESS_TRAY_Y / 2.0 - 3.0,
        DECK_Z / 2.0 + WITNESS_TRAY_Z / 2.0 + 4.0,
    );

    tray - basin - module_shadow - drain
        + dye_channel_ribs()
        + wetness_sensor_wells()
        + tray_lift_handles()
}

fn dye_channel_ribs() -> Part {
    let mut ribs = Part::empty("gasket_seal_fixture_dye_channel_ribs");
    for i in 0..DYE_CHANNELS {
        let y = MODULE_CENTER_Y - 180.0 + i as f64 * 120.0;
        ribs = ribs
            + centered_cube(
                format!("gasket_seal_fixture_dye_flow_channel_rib_{i}"),
                WITNESS_TRAY_X - 120.0,
                5.0,
                8.0,
            )
            .translate(MODULE_CENTER_X, y, DECK_Z / 2.0 + WITNESS_TRAY_Z + 5.0);
    }
    ribs
}

fn wetness_sensor_wells() -> Part {
    let mut wells = Part::empty("gasket_seal_fixture_wetness_sensor_wells");
    for i in 0..WETNESS_SENSOR_WELLS {
        let (x, y) = wetness_sensor_xy(i);
        let rim = centered_cube(
            format!("gasket_seal_fixture_wetness_sensor_well_rim_{i}"),
            54.0,
            34.0,
            10.0,
        )
        .translate(x, y, DECK_Z / 2.0 + WITNESS_TRAY_Z + 7.0);
        let pocket = centered_cube(
            format!("gasket_seal_fixture_wetness_sensor_well_pocket_{i}"),
            36.0,
            20.0,
            12.0,
        )
        .translate(x, y, DECK_Z / 2.0 + WITNESS_TRAY_Z + 7.0);
        wells = wells + (rim - pocket);
    }
    wells
}

fn tray_lift_handles() -> Part {
    let mut handles = Part::empty("gasket_seal_fixture_witness_tray_lift_handles");
    for (i, x) in [
        MODULE_CENTER_X - WITNESS_TRAY_X / 2.0 + 58.0,
        MODULE_CENTER_X + WITNESS_TRAY_X / 2.0 - 58.0,
    ]
    .iter()
    .enumerate()
    {
        let handle = centered_cube(
            format!("gasket_seal_fixture_witness_tray_handle_{i}"),
            28.0,
            142.0,
            20.0,
        )
        .translate(*x, MODULE_CENTER_Y, DECK_Z / 2.0 + WITNESS_TRAY_Z + 10.0);
        let grip = centered_cube(
            format!("gasket_seal_fixture_witness_tray_handle_grip_{i}"),
            32.0,
            96.0,
            22.0,
        )
        .translate(*x, MODULE_CENTER_Y, DECK_Z / 2.0 + WITNESS_TRAY_Z + 11.0);
        handles = handles + (handle - grip);
    }
    handles
}

fn barcode_run_record_scan_lands() -> Part {
    let panel = centered_cube(
        "gasket_seal_fixture_barcode_run_record_panel",
        BARCODE_PANEL_X,
        BARCODE_PANEL_Y,
        BARCODE_PANEL_Z,
    )
    .translate(
        BARCODE_CENTER_X,
        BARCODE_CENTER_Y,
        DECK_Z / 2.0 + BARCODE_PANEL_Z / 2.0 + 8.0,
    );
    let mut lands = Part::empty("gasket_seal_fixture_barcode_run_record_lands");
    for i in 0..BARCODE_LANDS {
        let col = i % 5;
        let row = i / 5;
        lands = lands
            + centered_cube(
                format!("gasket_seal_fixture_barcode_rfid_scan_land_{i}"),
                72.0,
                26.0,
                4.0,
            )
            .translate(
                BARCODE_CENTER_X - 170.0 + col as f64 * 85.0,
                BARCODE_CENTER_Y - 28.0 + row as f64 * 56.0,
                DECK_Z / 2.0 + BARCODE_PANEL_Z + 12.0,
            );
    }
    let run_record_card =
        centered_cube("gasket_seal_fixture_run_record_card_land", 185.0, 54.0, 5.0).translate(
            BARCODE_CENTER_X + 120.0,
            BARCODE_CENTER_Y,
            DECK_Z / 2.0 + BARCODE_PANEL_Z + 13.0,
        );
    panel + lands + run_record_card
}

fn pass_hold_reject_lanes() -> Part {
    let mut lanes = Part::empty("gasket_seal_fixture_pass_hold_reject_lanes");
    for lane in 0..DISPOSITION_LANES {
        let y = DISPOSITION_CENTER_Y + (lane as f64 - 1.0) * 96.0;
        let lane_name = match lane {
            0 => "pass",
            1 => "hold",
            _ => "reject",
        };
        let tray = centered_cube(
            format!("gasket_seal_fixture_{lane_name}_disposition_lane"),
            DISPOSITION_LANE_X,
            DISPOSITION_LANE_Y,
            DISPOSITION_LANE_Z,
        )
        .translate(
            DISPOSITION_CENTER_X,
            y,
            DECK_Z / 2.0 + DISPOSITION_LANE_Z / 2.0 + 8.0,
        );
        let mut slots = Part::empty(format!("gasket_seal_fixture_{lane_name}_disposition_slots"));
        for slot in 0..DISPOSITION_SLOTS_PER_LANE {
            slots = slots
                + centered_cube(
                    format!("gasket_seal_fixture_{lane_name}_slot_{slot}"),
                    58.0,
                    48.0,
                    DISPOSITION_LANE_Z + 4.0,
                )
                .translate(
                    DISPOSITION_CENTER_X - 126.0 + slot as f64 * 84.0,
                    y,
                    DECK_Z / 2.0 + DISPOSITION_LANE_Z / 2.0 + 10.0,
                );
        }
        let label_land = centered_cube(
            format!("gasket_seal_fixture_{lane_name}_label_land"),
            82.0,
            12.0,
            5.0,
        )
        .translate(
            DISPOSITION_CENTER_X,
            y + 42.0,
            DECK_Z / 2.0 + DISPOSITION_LANE_Z + 13.0,
        );
        lanes = lanes + (tray - slots) + label_land;
    }
    lanes
}

fn calibration_shim_used_gasket_storage() -> Part {
    let base = centered_cube(
        "gasket_seal_fixture_calibration_and_quarantine_storage_base",
        STORAGE_X,
        STORAGE_Y,
        STORAGE_Z,
    )
    .translate(
        STORAGE_CENTER_X,
        STORAGE_CENTER_Y,
        DECK_Z / 2.0 + STORAGE_Z / 2.0 + 8.0,
    );
    let mut cuts = Part::empty("gasket_seal_fixture_storage_pocket_cuts");

    for i in 0..CALIBRATION_SHIM_POCKETS {
        let col = i % 6;
        let row = i / 6;
        cuts = cuts
            + centered_cube(
                format!("gasket_seal_fixture_calibration_shim_pocket_{i}"),
                SHIM_POCKET_X,
                SHIM_POCKET_Y,
                STORAGE_Z + 5.0,
            )
            .translate(
                STORAGE_CENTER_X - 140.0 + col as f64 * 56.0,
                STORAGE_CENTER_Y - 94.0 + row as f64 * 34.0,
                DECK_Z / 2.0 + STORAGE_Z / 2.0 + 10.0,
            );
    }

    for i in 0..USED_GASKET_WELLS {
        let col = i % 6;
        let row = i / 6;
        cuts = cuts
            + centered_cylinder(
                format!("gasket_seal_fixture_used_gasket_quarantine_well_{i}"),
                USED_GASKET_WELL_D / 2.0,
                STORAGE_Z + 6.0,
                28,
            )
            .translate(
                STORAGE_CENTER_X - 140.0 + col as f64 * 56.0,
                STORAGE_CENTER_Y + 28.0 + row as f64 * 52.0,
                DECK_Z / 2.0 + STORAGE_Z / 2.0 + 10.0,
            );
    }

    let quarantine_lid_land = centered_cube(
        "gasket_seal_fixture_used_gasket_quarantine_lid_land",
        340.0,
        34.0,
        6.0,
    )
    .translate(
        STORAGE_CENTER_X,
        STORAGE_CENTER_Y + STORAGE_Y / 2.0 - 28.0,
        DECK_Z / 2.0 + STORAGE_Z + 13.0,
    );
    base - cuts + quarantine_lid_land
}

fn transparent_inspection_window_envelope() -> Part {
    let z_base = DECK_Z / 2.0 + WITNESS_TRAY_Z + NEST_Z + 30.0;
    let mut frame = Part::empty("gasket_seal_fixture_transparent_inspection_window_frame");

    for (i, (x, y)) in [
        (
            WINDOW_CENTER_X - WINDOW_X / 2.0 + WINDOW_FRAME_W / 2.0,
            WINDOW_CENTER_Y - WINDOW_Y / 2.0 + WINDOW_FRAME_W / 2.0,
        ),
        (
            WINDOW_CENTER_X + WINDOW_X / 2.0 - WINDOW_FRAME_W / 2.0,
            WINDOW_CENTER_Y - WINDOW_Y / 2.0 + WINDOW_FRAME_W / 2.0,
        ),
        (
            WINDOW_CENTER_X - WINDOW_X / 2.0 + WINDOW_FRAME_W / 2.0,
            WINDOW_CENTER_Y + WINDOW_Y / 2.0 - WINDOW_FRAME_W / 2.0,
        ),
        (
            WINDOW_CENTER_X + WINDOW_X / 2.0 - WINDOW_FRAME_W / 2.0,
            WINDOW_CENTER_Y + WINDOW_Y / 2.0 - WINDOW_FRAME_W / 2.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        frame = frame
            + centered_cube(
                format!("gasket_seal_fixture_inspection_window_post_{i}"),
                WINDOW_FRAME_W,
                WINDOW_FRAME_W,
                WINDOW_Z,
            )
            .translate(*x, *y, z_base + WINDOW_Z / 2.0);
    }

    let front_top = centered_cube(
        "gasket_seal_fixture_inspection_window_front_top_rail",
        WINDOW_X,
        WINDOW_FRAME_W,
        WINDOW_FRAME_W,
    )
    .translate(
        WINDOW_CENTER_X,
        WINDOW_CENTER_Y - WINDOW_Y / 2.0 + WINDOW_FRAME_W / 2.0,
        z_base + WINDOW_Z - WINDOW_FRAME_W / 2.0,
    );
    let rear_top = centered_cube(
        "gasket_seal_fixture_inspection_window_rear_top_rail",
        WINDOW_X,
        WINDOW_FRAME_W,
        WINDOW_FRAME_W,
    )
    .translate(
        WINDOW_CENTER_X,
        WINDOW_CENTER_Y + WINDOW_Y / 2.0 - WINDOW_FRAME_W / 2.0,
        z_base + WINDOW_Z - WINDOW_FRAME_W / 2.0,
    );
    let left_top = centered_cube(
        "gasket_seal_fixture_inspection_window_left_top_rail",
        WINDOW_FRAME_W,
        WINDOW_Y,
        WINDOW_FRAME_W,
    )
    .translate(
        WINDOW_CENTER_X - WINDOW_X / 2.0 + WINDOW_FRAME_W / 2.0,
        WINDOW_CENTER_Y,
        z_base + WINDOW_Z - WINDOW_FRAME_W / 2.0,
    );
    let right_top = centered_cube(
        "gasket_seal_fixture_inspection_window_right_top_rail",
        WINDOW_FRAME_W,
        WINDOW_Y,
        WINDOW_FRAME_W,
    )
    .translate(
        WINDOW_CENTER_X + WINDOW_X / 2.0 - WINDOW_FRAME_W / 2.0,
        WINDOW_CENTER_Y,
        z_base + WINDOW_Z - WINDOW_FRAME_W / 2.0,
    );
    let transparent_panel = centered_cube(
        "gasket_seal_fixture_clear_inspection_window_placeholder",
        WINDOW_X - 88.0,
        WINDOW_Y - 88.0,
        6.0,
    )
    .translate(WINDOW_CENTER_X, WINDOW_CENTER_Y, z_base + WINDOW_Z - 42.0);
    let camera_bridge = centered_cube(
        "gasket_seal_fixture_camera_light_bridge",
        WINDOW_X - 160.0,
        54.0,
        42.0,
    )
    .translate(WINDOW_CENTER_X, WINDOW_CENTER_Y, z_base + WINDOW_Z + 18.0);

    frame + front_top + rear_top + left_top + right_top + transparent_panel + camera_bridge
}

fn robot_service_keepouts() -> Part {
    let front_robot = centered_cube(
        "gasket_seal_fixture_front_robot_module_load_keepout",
        NEST_X + 120.0,
        FRONT_ROBOT_APPROACH_Y,
        MODULE_Z + 120.0,
    )
    .translate(
        MODULE_CENTER_X,
        -DECK_Y / 2.0 + FRONT_ROBOT_APPROACH_Y / 2.0 + 8.0,
        DECK_Z / 2.0 + (MODULE_Z + 120.0) / 2.0,
    );
    let rear_service = centered_cube(
        "gasket_seal_fixture_rear_pressure_service_keepout",
        MANIFOLD_X + 80.0,
        REAR_SERVICE_CLEARANCE_Y,
        MANIFOLD_Z + 90.0,
    )
    .translate(
        MANIFOLD_CENTER_X,
        DECK_Y / 2.0 - REAR_SERVICE_CLEARANCE_Y / 2.0,
        DECK_Z / 2.0 + (MANIFOLD_Z + 90.0) / 2.0,
    );
    let right_gasket_change = centered_cube(
        "gasket_seal_fixture_right_gasket_change_service_keepout",
        GASKET_CHANGE_CLEARANCE_X,
        MODULE_Y + 180.0,
        MODULE_Z + 90.0,
    )
    .translate(
        DECK_X / 2.0 - GASKET_CHANGE_CLEARANCE_X / 2.0 - 8.0,
        MODULE_CENTER_Y,
        DECK_Z / 2.0 + (MODULE_Z + 90.0) / 2.0,
    );
    let left_window_wipe = centered_cube(
        "gasket_seal_fixture_left_window_wipe_service_keepout",
        SIDE_SERVICE_CLEARANCE_X,
        WINDOW_Y + 120.0,
        WINDOW_Z,
    )
    .translate(
        -DECK_X / 2.0 + SIDE_SERVICE_CLEARANCE_X / 2.0 + 8.0,
        MODULE_CENTER_Y,
        DECK_Z / 2.0 + WINDOW_Z / 2.0 + 55.0,
    );
    let top_camera = centered_cube(
        "gasket_seal_fixture_top_camera_focus_keepout",
        WINDOW_X - 160.0,
        WINDOW_Y - 160.0,
        TOP_CAMERA_CLEARANCE_Z,
    )
    .translate(
        WINDOW_CENTER_X,
        WINDOW_CENTER_Y,
        DECK_Z / 2.0 + WITNESS_TRAY_Z + NEST_Z + WINDOW_Z + TOP_CAMERA_CLEARANCE_Z / 2.0 + 42.0,
    );

    front_robot + rear_service + right_gasket_change + left_window_wipe + top_camera
}

fn fiducial_target(name: &str) -> Part {
    let disk = centered_cylinder(format!("{name}_disk"), FIDUCIAL_D / 2.0, 2.0, 36);
    let center = centered_cylinder(format!("{name}_center_mark"), 2.0, 3.0, 20);
    disk - center
}

fn deck_mount_points() -> [(f64, f64); 10] {
    [
        (-DECK_X / 2.0 + 58.0, -DECK_Y / 2.0 + 58.0),
        (DECK_X / 2.0 - 58.0, -DECK_Y / 2.0 + 58.0),
        (-DECK_X / 2.0 + 58.0, DECK_Y / 2.0 - 58.0),
        (DECK_X / 2.0 - 58.0, DECK_Y / 2.0 - 58.0),
        (0.0, -DECK_Y / 2.0 + 58.0),
        (0.0, DECK_Y / 2.0 - 58.0),
        (-DECK_X / 2.0 + 58.0, 0.0),
        (DECK_X / 2.0 - 58.0, 0.0),
        (MODULE_CENTER_X - 330.0, MODULE_CENTER_Y),
        (MODULE_CENTER_X + 330.0, MODULE_CENTER_Y),
    ]
}

fn module_datum_points() -> [(f64, f64); 6] {
    [
        (-(MODULE_X / 2.0 - 64.0), -(MODULE_Y / 2.0 - 58.0)),
        (MODULE_X / 2.0 - 64.0, -(MODULE_Y / 2.0 - 58.0)),
        (-(MODULE_X / 2.0 - 64.0), MODULE_Y / 2.0 - 58.0),
        (MODULE_X / 2.0 - 64.0, MODULE_Y / 2.0 - 58.0),
        (0.0, -(MODULE_Y / 2.0 - 58.0)),
        (0.0, MODULE_Y / 2.0 - 58.0),
    ]
}

fn compression_land_xy(index: usize) -> (f64, f64) {
    let per_side = COMPRESSION_GAUGE_LANDS / 4;
    let side = index / per_side;
    let offset = index % per_side;
    let t = -0.375 + offset as f64 * 0.25;
    match side {
        0 => (t * GASKET_OUTER_X, GASKET_OUTER_Y / 2.0 + 28.0),
        1 => (GASKET_OUTER_X / 2.0 + 28.0, t * GASKET_OUTER_Y),
        2 => (-t * GASKET_OUTER_X, -GASKET_OUTER_Y / 2.0 - 28.0),
        _ => (-GASKET_OUTER_X / 2.0 - 28.0, -t * GASKET_OUTER_Y),
    }
}

fn clamp_pocket_xy_rot(index: usize) -> (f64, f64, f64) {
    let per_side = CLAMP_FORCE_POCKETS / 4;
    let side = index / per_side;
    let offset = index % per_side;
    let side_t = if offset == 0 { -0.27 } else { 0.27 };
    match side {
        0 => (
            MODULE_CENTER_X + side_t * MODULE_X,
            MODULE_CENTER_Y - MODULE_Y / 2.0 - 62.0,
            0.0,
        ),
        1 => (
            MODULE_CENTER_X + MODULE_X / 2.0 + 62.0,
            MODULE_CENTER_Y + side_t * MODULE_Y,
            90.0,
        ),
        2 => (
            MODULE_CENTER_X - side_t * MODULE_X,
            MODULE_CENTER_Y + MODULE_Y / 2.0 + 62.0,
            0.0,
        ),
        _ => (
            MODULE_CENTER_X - MODULE_X / 2.0 - 62.0,
            MODULE_CENTER_Y - side_t * MODULE_Y,
            90.0,
        ),
    }
}

fn decay_port_x(index: usize) -> f64 {
    -MANIFOLD_X / 2.0 + 92.0 + index as f64 * ((MANIFOLD_X - 184.0) / (DECAY_PORTS as f64 - 1.0))
}

fn wetness_sensor_xy(index: usize) -> (f64, f64) {
    let side = index / 2;
    let offset = index % 2;
    let t = if offset == 0 { -0.25 } else { 0.25 };
    match side {
        0 => (
            MODULE_CENTER_X + t * WITNESS_TRAY_X,
            MODULE_CENTER_Y - WITNESS_TRAY_Y / 2.0 + 58.0,
        ),
        1 => (
            MODULE_CENTER_X + WITNESS_TRAY_X / 2.0 - 58.0,
            MODULE_CENTER_Y + t * WITNESS_TRAY_Y,
        ),
        2 => (
            MODULE_CENTER_X - t * WITNESS_TRAY_X,
            MODULE_CENTER_Y + WITNESS_TRAY_Y / 2.0 - 58.0,
        ),
        _ => (
            MODULE_CENTER_X - WITNESS_TRAY_X / 2.0 + 58.0,
            MODULE_CENTER_Y - t * WITNESS_TRAY_Y,
        ),
    }
}

#[cfg(test)]
fn rect_fits_deck(cx: f64, cy: f64, sx: f64, sy: f64, margin: f64) -> bool {
    cx.abs() + sx / 2.0 + margin <= DECK_X / 2.0 && cy.abs() + sy / 2.0 + margin <= DECK_Y / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn output_paths_are_unique_and_scoped() {
        let unique: HashSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        for path in OUTPUTS {
            assert!(path.starts_with("output/closed_module_gasket_seal_validation_fixture_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn required_feature_groups_cover_validation_workflow() {
        assert_eq!(REQUIRED_FEATURES.len(), 12);
        assert!(REQUIRED_FEATURES.contains(&"module_datum_nest"));
        assert!(REQUIRED_FEATURES.contains(&"gasket_witness_compression_gauge_lands"));
        assert!(REQUIRED_FEATURES.contains(&"clamp_latch_force_check_pockets"));
        assert!(REQUIRED_FEATURES.contains(&"pressure_vacuum_decay_port_manifold"));
        assert!(REQUIRED_FEATURES.contains(&"dye_wetness_leak_witness_tray"));
        assert!(REQUIRED_FEATURES.contains(&"pass_hold_reject_lanes"));
        assert!(REQUIRED_FEATURES.contains(&"calibration_shim_pockets"));
        assert!(REQUIRED_FEATURES.contains(&"used_gasket_quarantine_wells"));
        assert!(REQUIRED_FEATURES.contains(&"transparent_inspection_window_envelope"));
        assert!(REQUIRED_FEATURES.contains(&"robot_service_keepouts"));
    }

    #[test]
    fn module_gasket_and_nest_geometry_are_plausible() {
        assert!(MODULE_X >= 650.0);
        assert!(MODULE_Y >= 480.0);
        assert!(NEST_X > MODULE_X + 100.0);
        assert!(NEST_Y > MODULE_Y + 100.0);
        assert!(GASKET_OUTER_X < MODULE_X);
        assert!(GASKET_OUTER_Y < MODULE_Y);
        assert!(GASKET_TRACE_W >= 12.0);
        assert!(MODULE_CLEARANCE >= 0.5);
        assert_eq!(module_datum_points().len(), 6);
    }

    #[test]
    fn repeated_validation_counts_are_sane() {
        assert_eq!(COMPRESSION_GAUGE_LANDS, 16);
        assert_eq!(CLAMP_FORCE_POCKETS, 8);
        assert_eq!(LATCH_ENGAGEMENT_POCKETS, 4);
        assert_eq!(DECAY_PORTS, 12);
        assert_eq!(SENSOR_POCKETS, 6);
        assert_eq!(VALVE_BLOCKS, 8);
        assert_eq!(WETNESS_SENSOR_WELLS, 8);
        assert_eq!(CALIBRATION_SHIM_POCKETS, 12);
        assert_eq!(USED_GASKET_WELLS, 12);
        assert_eq!(DISPOSITION_LANES * DISPOSITION_SLOTS_PER_LANE, 12);
        assert_eq!(ROBOT_KEEP_OUT_ZONES, 5);
    }

    #[test]
    fn major_envelopes_fit_on_deck() {
        assert!(rect_fits_deck(
            MODULE_CENTER_X,
            MODULE_CENTER_Y,
            WITNESS_TRAY_X,
            WITNESS_TRAY_Y,
            18.0
        ));
        assert!(rect_fits_deck(
            NEST_CENTER_X,
            NEST_CENTER_Y,
            NEST_X,
            NEST_Y,
            14.0
        ));
        assert!(rect_fits_deck(
            MANIFOLD_CENTER_X,
            MANIFOLD_CENTER_Y,
            MANIFOLD_X,
            MANIFOLD_Y,
            14.0
        ));
        assert!(rect_fits_deck(
            BARCODE_CENTER_X,
            BARCODE_CENTER_Y,
            BARCODE_PANEL_X,
            BARCODE_PANEL_Y,
            12.0
        ));
        assert!(rect_fits_deck(
            DISPOSITION_CENTER_X,
            DISPOSITION_CENTER_Y,
            DISPOSITION_LANE_X,
            DISPOSITION_LANE_Y * DISPOSITION_LANES as f64 + 38.0,
            12.0
        ));
        assert!(rect_fits_deck(
            STORAGE_CENTER_X,
            STORAGE_CENTER_Y,
            STORAGE_X,
            STORAGE_Y,
            12.0
        ));
    }

    #[test]
    fn repeated_feature_positions_stay_inside_local_envelopes() {
        for i in 0..COMPRESSION_GAUGE_LANDS {
            let (x, y) = compression_land_xy(i);
            assert!(x.abs() <= GASKET_OUTER_X / 2.0 + 45.0);
            assert!(y.abs() <= GASKET_OUTER_Y / 2.0 + 45.0);
        }
        for i in 0..DECAY_PORTS {
            assert!(decay_port_x(i).abs() < MANIFOLD_X / 2.0 - 60.0);
        }
        for i in 0..WETNESS_SENSOR_WELLS {
            let (x, y) = wetness_sensor_xy(i);
            assert!(rect_fits_deck(x, y, 54.0, 34.0, 4.0));
        }
    }
}
