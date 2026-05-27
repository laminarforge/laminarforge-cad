use vcad::{centered_cube, centered_cylinder, Part};

// Pressure-driven perfusion controller panel for multiplexed tissue-chip lanes.
//
// Intent:
// - Package purchased/commercial pressure controllers as serviceable envelopes.
// - Route sterile filtered pressure, vacuum reference, reservoir headspace ports,
//   relief valves, pressure sensors, calibration ports, and bulkhead fittings.
// - Provide lane-by-lane pneumatic/cable strain relief so parallel chips can run
//   at stable pressure-driven flow and shear without tugging on small tubing.
//
// This is mechanical packaging for bought pressure-control instruments. The
// pressure controllers themselves are not DIY devices and are not modeled
// internally here.

const LANES: usize = 8;
const PRESSURE_CONTROLLERS: usize = 4;
const LANES_PER_CONTROLLER: usize = LANES / PRESSURE_CONTROLLERS;
const RESERVOIR_PORTS: usize = LANES;
const PRESSURE_SENSORS: usize = LANES;
const CALIBRATION_PORTS: usize = LANES;
const STERILE_FILTERS: usize = PRESSURE_CONTROLLERS;
const RELIEF_VALVES: usize = PRESSURE_CONTROLLERS;

const BASE_X: f64 = 860.0;
const BASE_Y: f64 = 380.0;
const BASE_Z: f64 = 16.0;
const PANEL_X: f64 = 790.0;
const PANEL_Y: f64 = 26.0;
const PANEL_Z: f64 = 430.0;
const PANEL_BASE_Y: f64 = BASE_Y / 2.0 - 54.0;

const CONTROLLER_ENV_X: f64 = 126.0;
const CONTROLLER_ENV_Y: f64 = 92.0;
const CONTROLLER_ENV_Z: f64 = 146.0;
const CONTROLLER_FACE_X: f64 = 112.0;
const CONTROLLER_FACE_Z: f64 = 108.0;
const CONTROLLER_PITCH_X: f64 = 168.0;
const CONTROLLER_ROW_Z: f64 = 118.0;

const MANIFOLD_X: f64 = 690.0;
const MANIFOLD_Y: f64 = 54.0;
const MANIFOLD_Z: f64 = 72.0;
const LANE_PITCH_X: f64 = 78.0;
const BULKHEAD_PORT_D: f64 = 10.4;
const SENSOR_PORT_D: f64 = 4.4;
const CAL_PORT_D: f64 = 6.2;

const FILTER_LEN: f64 = 86.0;
const FILTER_D: f64 = 26.0;
const FILTER_PITCH_X: f64 = 156.0;
const RELIEF_D: f64 = 18.0;
const SENSOR_X: f64 = 38.0;
const SENSOR_Y: f64 = 28.0;
const SENSOR_Z: f64 = 24.0;

fn main() {
    let baseplate = baseplate();
    baseplate
        .write_stl("output/pressure_driven_perfusion_panel_baseplate.stl")
        .unwrap();
    println!("Exported: output/pressure_driven_perfusion_panel_baseplate.stl");

    let panel = instrument_panel();
    panel
        .write_stl("output/pressure_driven_perfusion_panel_instrument_panel.stl")
        .unwrap();
    println!("Exported: output/pressure_driven_perfusion_panel_instrument_panel.stl");

    let controllers = commercial_controller_carriers();
    controllers
        .write_stl("output/pressure_driven_perfusion_panel_commercial_controller_carriers.stl")
        .unwrap();
    println!("Exported: output/pressure_driven_perfusion_panel_commercial_controller_carriers.stl");

    let reservoir = reservoir_pressure_bulkhead_manifold();
    reservoir
        .write_stl("output/pressure_driven_perfusion_panel_reservoir_bulkhead_manifold.stl")
        .unwrap();
    println!("Exported: output/pressure_driven_perfusion_panel_reservoir_bulkhead_manifold.stl");

    let vacuum = vacuum_reference_manifold();
    vacuum
        .write_stl("output/pressure_driven_perfusion_panel_vacuum_reference_manifold.stl")
        .unwrap();
    println!("Exported: output/pressure_driven_perfusion_panel_vacuum_reference_manifold.stl");

    let filters = filter_relief_sensor_bank();
    filters
        .write_stl("output/pressure_driven_perfusion_panel_filter_relief_sensor_bank.stl")
        .unwrap();
    println!("Exported: output/pressure_driven_perfusion_panel_filter_relief_sensor_bank.stl");

    let calibration = calibration_service_bar();
    calibration
        .write_stl("output/pressure_driven_perfusion_panel_calibration_service_bar.stl")
        .unwrap();
    println!("Exported: output/pressure_driven_perfusion_panel_calibration_service_bar.stl");

    let strain_relief = cable_pneumatic_strain_relief();
    strain_relief
        .write_stl("output/pressure_driven_perfusion_panel_strain_relief.stl")
        .unwrap();
    println!("Exported: output/pressure_driven_perfusion_panel_strain_relief.stl");

    let assembly = baseplate
        + panel.translate(0.0, PANEL_BASE_Y, BASE_Z / 2.0 + PANEL_Z / 2.0)
        + controllers.translate(0.0, PANEL_BASE_Y + 68.0, BASE_Z / 2.0 + PANEL_Z / 2.0)
        + reservoir.translate(0.0, PANEL_BASE_Y - 56.0, BASE_Z / 2.0 + 100.0)
        + vacuum.translate(0.0, PANEL_BASE_Y - 64.0, BASE_Z / 2.0 + 34.0)
        + filters.translate(0.0, PANEL_BASE_Y - 66.0, BASE_Z / 2.0 + 252.0)
        + calibration.translate(0.0, PANEL_BASE_Y - 62.0, BASE_Z / 2.0 + 174.0)
        + strain_relief.translate(0.0, -BASE_Y / 2.0 + 54.0, BASE_Z / 2.0 + 34.0);

    assembly
        .write_stl("output/pressure_driven_perfusion_panel_assembly.stl")
        .unwrap();
    println!("Exported: output/pressure_driven_perfusion_panel_assembly.stl");

    println!(
        "Pressure-driven perfusion panel: {:.0}mm x {:.0}mm benchtop footprint, {:.0}mm tall service panel, {} lanes, {} purchased pressure-controller envelopes, {} reservoir headspace ports, {} pressure sensors, {} calibration ports, {} relief valves, sterile filters, and cable/pneumatic strain relief.",
        BASE_X,
        BASE_Y,
        PANEL_Z,
        LANES,
        PRESSURE_CONTROLLERS,
        RESERVOIR_PORTS,
        PRESSURE_SENSORS,
        CALIBRATION_PORTS,
        RELIEF_VALVES
    );
}

fn baseplate() -> Part {
    let deck = centered_cube("pressure_perfusion_panel_baseplate", BASE_X, BASE_Y, BASE_Z);

    let spill_sump = centered_cube(
        "pressure_perfusion_panel_front_service_spill_sump",
        BASE_X - 96.0,
        82.0,
        8.0,
    )
    .translate(0.0, -BASE_Y / 2.0 + 74.0, BASE_Z / 2.0 - 3.0);
    let drain = centered_cylinder("pressure_perfusion_panel_sump_drain", 7.0 / 2.0, 32.0, 28)
        .rotate(90.0, 0.0, 0.0)
        .translate(BASE_X / 2.0 - 82.0, -BASE_Y / 2.0 + 28.0, 0.0);

    let rear_panel_slot = centered_cube(
        "pressure_perfusion_panel_rear_panel_socket",
        PANEL_X + 22.0,
        18.0,
        8.0,
    )
    .translate(0.0, PANEL_BASE_Y, BASE_Z / 2.0 - 2.0);

    let mut mount_slots = Part::empty("pressure_perfusion_panel_base_mount_slots");
    for (i, (x, y)) in base_mount_points().iter().enumerate() {
        let hole = centered_cylinder(
            format!("pressure_perfusion_panel_m6_clearance_{i}"),
            6.6 / 2.0,
            BASE_Z + 2.0,
            24,
        )
        .translate(*x, *y, 0.0);
        let slot = centered_cube(
            format!("pressure_perfusion_panel_m6_slot_{i}"),
            24.0,
            6.8,
            BASE_Z + 2.0,
        )
        .translate(*x, *y, 0.0);
        mount_slots = mount_slots + hole + slot;
    }

    deck - spill_sump - drain - rear_panel_slot - mount_slots
        + base_perimeter_rails()
        + panel_gussets()
        + leveling_feet()
}

fn base_perimeter_rails() -> Part {
    let left = centered_cube(
        "pressure_perfusion_panel_left_base_rail",
        18.0,
        BASE_Y - 54.0,
        26.0,
    )
    .translate(-(BASE_X / 2.0 - 30.0), 0.0, BASE_Z / 2.0 + 13.0);
    let right = centered_cube(
        "pressure_perfusion_panel_right_base_rail",
        18.0,
        BASE_Y - 54.0,
        26.0,
    )
    .translate(BASE_X / 2.0 - 30.0, 0.0, BASE_Z / 2.0 + 13.0);
    let rear = centered_cube(
        "pressure_perfusion_panel_rear_base_rail",
        BASE_X - 72.0,
        18.0,
        28.0,
    )
    .translate(0.0, BASE_Y / 2.0 - 30.0, BASE_Z / 2.0 + 14.0);
    let front = centered_cube(
        "pressure_perfusion_panel_front_service_lip",
        BASE_X - 108.0,
        14.0,
        18.0,
    )
    .translate(0.0, -BASE_Y / 2.0 + 24.0, BASE_Z / 2.0 + 9.0);

    left + right + rear + front
}

fn panel_gussets() -> Part {
    let mut gussets = Part::empty("pressure_perfusion_panel_rear_gussets");
    for (i, x) in [-330.0, -198.0, -66.0, 66.0, 198.0, 330.0]
        .iter()
        .enumerate()
    {
        let web = centered_cube(
            format!("pressure_perfusion_panel_gusset_web_{i}"),
            12.0,
            82.0,
            92.0,
        )
        .translate(*x, PANEL_BASE_Y - 32.0, BASE_Z / 2.0 + 46.0);
        let foot = centered_cube(
            format!("pressure_perfusion_panel_gusset_foot_{i}"),
            46.0,
            72.0,
            10.0,
        )
        .translate(*x, PANEL_BASE_Y - 32.0, BASE_Z / 2.0 + 5.0);
        let screw = centered_cylinder(
            format!("pressure_perfusion_panel_gusset_m5_clearance_{i}"),
            5.4 / 2.0,
            14.0,
            24,
        )
        .translate(*x, PANEL_BASE_Y - 50.0, BASE_Z / 2.0 + 5.0);
        gussets = gussets + (web + foot - screw);
    }
    gussets
}

fn leveling_feet() -> Part {
    let mut feet = Part::empty("pressure_perfusion_panel_leveling_feet");
    for (i, (x, y)) in [
        (-(BASE_X / 2.0 - 58.0), -(BASE_Y / 2.0 - 50.0)),
        (BASE_X / 2.0 - 58.0, -(BASE_Y / 2.0 - 50.0)),
        (-(BASE_X / 2.0 - 58.0), BASE_Y / 2.0 - 50.0),
        (BASE_X / 2.0 - 58.0, BASE_Y / 2.0 - 50.0),
        (0.0, -(BASE_Y / 2.0 - 50.0)),
        (0.0, BASE_Y / 2.0 - 50.0),
    ]
    .iter()
    .enumerate()
    {
        let pad = centered_cylinder(
            format!("pressure_perfusion_panel_leveling_pad_{i}"),
            22.0,
            10.0,
            40,
        )
        .translate(*x, *y, -(BASE_Z / 2.0 + 5.0));
        let adjuster = centered_cylinder(
            format!("pressure_perfusion_panel_leveler_thread_clearance_{i}"),
            8.0 / 2.0,
            18.0,
            24,
        )
        .translate(*x, *y, -(BASE_Z / 2.0 + 5.0));
        feet = feet + (pad - adjuster);
    }
    feet
}

fn instrument_panel() -> Part {
    let panel = centered_cube(
        "pressure_perfusion_panel_instrument_plate",
        PANEL_X,
        PANEL_Y,
        PANEL_Z,
    );

    let mut cuts = Part::empty("pressure_perfusion_panel_instrument_plate_cuts");
    for controller in 0..PRESSURE_CONTROLLERS {
        let x = controller_x(controller);
        let face = centered_cube(
            format!("pressure_perfusion_controller_{controller}_purchased_face_cutout"),
            CONTROLLER_FACE_X,
            PANEL_Y + 4.0,
            CONTROLLER_FACE_Z,
        )
        .translate(x, 0.0, CONTROLLER_ROW_Z);
        let display = centered_cube(
            format!("pressure_perfusion_controller_{controller}_display_window"),
            78.0,
            PANEL_Y + 6.0,
            26.0,
        )
        .translate(x, 0.0, CONTROLLER_ROW_Z + 24.0);
        cuts = cuts + face + display;

        for (j, (dx, dz)) in [
            (
                -CONTROLLER_FACE_X / 2.0 - 11.0,
                -CONTROLLER_FACE_Z / 2.0 + 11.0,
            ),
            (
                CONTROLLER_FACE_X / 2.0 + 11.0,
                -CONTROLLER_FACE_Z / 2.0 + 11.0,
            ),
            (
                -CONTROLLER_FACE_X / 2.0 - 11.0,
                CONTROLLER_FACE_Z / 2.0 - 11.0,
            ),
            (
                CONTROLLER_FACE_X / 2.0 + 11.0,
                CONTROLLER_FACE_Z / 2.0 - 11.0,
            ),
        ]
        .iter()
        .enumerate()
        {
            cuts = cuts
                + centered_cylinder(
                    format!("pressure_perfusion_controller_{controller}_m4_face_screw_{j}"),
                    4.4 / 2.0,
                    PANEL_Y + 6.0,
                    20,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(x + *dx, 0.0, CONTROLLER_ROW_Z + *dz);
        }
    }

    for lane in 0..LANES {
        let x = lane_x(lane);
        cuts = cuts
            + centered_cylinder(
                format!("pressure_perfusion_lane_{lane}_reservoir_bulkhead_panel_cutout"),
                BULKHEAD_PORT_D / 2.0,
                PANEL_Y + 6.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, -54.0)
            + centered_cylinder(
                format!("pressure_perfusion_lane_{lane}_pressure_sensor_panel_cutout"),
                SENSOR_PORT_D / 2.0,
                PANEL_Y + 6.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, -116.0)
            + centered_cylinder(
                format!("pressure_perfusion_lane_{lane}_calibration_panel_cutout"),
                CAL_PORT_D / 2.0,
                PANEL_Y + 6.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, -176.0);
    }

    let vacuum_source = centered_cylinder(
        "pressure_perfusion_panel_vacuum_source_bulkhead_cutout",
        12.6 / 2.0,
        PANEL_Y + 6.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-(PANEL_X / 2.0 - 64.0), 0.0, -174.0);
    let compressed_air_source = centered_cylinder(
        "pressure_perfusion_panel_clean_gas_inlet_bulkhead_cutout",
        12.6 / 2.0,
        PANEL_Y + 6.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(PANEL_X / 2.0 - 64.0, 0.0, -174.0);
    cuts = cuts + vacuum_source + compressed_air_source;

    panel - cuts + panel_stiffeners() + panel_label_lands()
}

fn panel_stiffeners() -> Part {
    let top = centered_cube(
        "pressure_perfusion_panel_top_stiffener",
        PANEL_X - 42.0,
        14.0,
        18.0,
    )
    .translate(0.0, PANEL_Y / 2.0 + 7.0, PANEL_Z / 2.0 - 24.0);
    let lower = centered_cube(
        "pressure_perfusion_panel_lower_stiffener",
        PANEL_X - 42.0,
        14.0,
        18.0,
    )
    .translate(0.0, PANEL_Y / 2.0 + 7.0, -PANEL_Z / 2.0 + 24.0);

    let mut ribs = Part::empty("pressure_perfusion_panel_vertical_stiffeners");
    for (i, x) in [-360.0, -180.0, 0.0, 180.0, 360.0].iter().enumerate() {
        ribs = ribs
            + centered_cube(
                format!("pressure_perfusion_panel_vertical_stiffener_{i}"),
                12.0,
                14.0,
                PANEL_Z - 84.0,
            )
            .translate(*x, PANEL_Y / 2.0 + 7.0, 0.0);
    }

    top + lower + ribs
}

fn panel_label_lands() -> Part {
    let controller_strip = centered_cube(
        "pressure_perfusion_panel_purchased_controller_label_land",
        PANEL_X - 84.0,
        4.0,
        14.0,
    )
    .translate(0.0, -(PANEL_Y / 2.0 + 2.0), CONTROLLER_ROW_Z - 78.0);
    let reservoir_strip = centered_cube(
        "pressure_perfusion_panel_reservoir_port_label_land",
        MANIFOLD_X,
        4.0,
        12.0,
    )
    .translate(0.0, -(PANEL_Y / 2.0 + 2.0), -24.0);
    let sensor_strip = centered_cube(
        "pressure_perfusion_panel_sensor_label_land",
        MANIFOLD_X,
        4.0,
        12.0,
    )
    .translate(0.0, -(PANEL_Y / 2.0 + 2.0), -88.0);
    let calibration_strip = centered_cube(
        "pressure_perfusion_panel_calibration_label_land",
        MANIFOLD_X,
        4.0,
        12.0,
    )
    .translate(0.0, -(PANEL_Y / 2.0 + 2.0), -148.0);

    controller_strip + reservoir_strip + sensor_strip + calibration_strip
}

fn commercial_controller_carriers() -> Part {
    let mut carriers = Part::empty("pressure_perfusion_commercial_controller_carriers");

    for controller in 0..PRESSURE_CONTROLLERS {
        let x = controller_x(controller);
        let envelope =
            purchased_controller_envelope(controller).translate(x, 0.0, CONTROLLER_ROW_Z);
        let cradle = purchased_controller_cradle(controller).translate(x, 0.0, CONTROLLER_ROW_Z);
        let service_loop = controller_service_loop(controller).translate(
            x,
            CONTROLLER_ENV_Y / 2.0 + 18.0,
            CONTROLLER_ROW_Z - 62.0,
        );
        carriers = carriers + envelope + cradle + service_loop;
    }

    let din_rail = centered_cube(
        "pressure_perfusion_commercial_controller_common_din_rail",
        PANEL_X - 110.0,
        16.0,
        22.0,
    )
    .translate(0.0, CONTROLLER_ENV_Y / 2.0 + 24.0, CONTROLLER_ROW_Z - 86.0);

    carriers + din_rail
}

fn purchased_controller_envelope(controller: usize) -> Part {
    let body = centered_cube(
        format!("pressure_perfusion_purchased_pressure_controller_{controller}_envelope"),
        CONTROLLER_ENV_X,
        CONTROLLER_ENV_Y,
        CONTROLLER_ENV_Z,
    );
    let screen_recess = centered_cube(
        format!("pressure_perfusion_purchased_pressure_controller_{controller}_screen_recess"),
        76.0,
        8.0,
        24.0,
    )
    .translate(0.0, -(CONTROLLER_ENV_Y / 2.0 + 1.0), 28.0);
    let knob_clearance = centered_cylinder(
        format!("pressure_perfusion_purchased_pressure_controller_{controller}_knob_clearance"),
        13.0,
        10.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, -(CONTROLLER_ENV_Y / 2.0 + 1.0), -20.0);
    let rear_connector_clearance = centered_cube(
        format!("pressure_perfusion_purchased_pressure_controller_{controller}_rear_connector_clearance"),
        72.0,
        10.0,
        34.0,
    )
    .translate(0.0, CONTROLLER_ENV_Y / 2.0 + 1.0, -44.0);

    body - screen_recess - knob_clearance - rear_connector_clearance
}

fn purchased_controller_cradle(controller: usize) -> Part {
    let base = centered_cube(
        format!("pressure_perfusion_purchased_controller_{controller}_cradle_base"),
        CONTROLLER_ENV_X + 28.0,
        14.0,
        14.0,
    )
    .translate(
        0.0,
        CONTROLLER_ENV_Y / 2.0 + 6.0,
        -(CONTROLLER_ENV_Z / 2.0 + 7.0),
    );
    let left = centered_cube(
        format!("pressure_perfusion_purchased_controller_{controller}_left_cradle_ear"),
        12.0,
        24.0,
        CONTROLLER_ENV_Z + 10.0,
    )
    .translate(-(CONTROLLER_ENV_X / 2.0 + 9.0), 2.0, 0.0);
    let right = centered_cube(
        format!("pressure_perfusion_purchased_controller_{controller}_right_cradle_ear"),
        12.0,
        24.0,
        CONTROLLER_ENV_Z + 10.0,
    )
    .translate(CONTROLLER_ENV_X / 2.0 + 9.0, 2.0, 0.0);

    let mut screws = Part::empty(format!(
        "pressure_perfusion_purchased_controller_{controller}_cradle_screws"
    ));
    for (i, dx) in [
        -(CONTROLLER_ENV_X / 2.0 + 9.0),
        CONTROLLER_ENV_X / 2.0 + 9.0,
    ]
    .iter()
    .enumerate()
    {
        screws = screws
            + centered_cylinder(
                format!("pressure_perfusion_purchased_controller_{controller}_ear_screw_{i}"),
                4.4 / 2.0,
                28.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*dx, 0.0, 46.0)
            + centered_cylinder(
                format!("pressure_perfusion_purchased_controller_{controller}_lower_ear_screw_{i}"),
                4.4 / 2.0,
                28.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*dx, 0.0, -46.0);
    }

    base + left + right - screws
}

fn controller_service_loop(controller: usize) -> Part {
    let cable_loop = centered_cube(
        format!("pressure_perfusion_controller_{controller}_cable_service_loop_keepout"),
        92.0,
        26.0,
        16.0,
    );
    let pneumatic_loop = centered_cube(
        format!("pressure_perfusion_controller_{controller}_pneumatic_service_loop_keepout"),
        92.0,
        26.0,
        16.0,
    )
    .translate(0.0, 0.0, 26.0);
    let bend_radius = centered_cylinder(
        format!("pressure_perfusion_controller_{controller}_min_bend_radius_marker"),
        16.0,
        8.0,
        40,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, 16.0, 13.0);

    cable_loop + pneumatic_loop + bend_radius
}

fn reservoir_pressure_bulkhead_manifold() -> Part {
    let body = centered_cube(
        "pressure_perfusion_reservoir_bulkhead_manifold_body",
        MANIFOLD_X,
        MANIFOLD_Y,
        MANIFOLD_Z,
    );

    let mut cuts = Part::empty("pressure_perfusion_reservoir_bulkhead_manifold_cuts");
    for lane in 0..LANES {
        let x = lane_x(lane);
        let bulkhead = centered_cylinder(
            format!("pressure_perfusion_lane_{lane}_reservoir_pressure_bulkhead"),
            BULKHEAD_PORT_D / 2.0,
            MANIFOLD_Y + 8.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 0.0, 14.0);
        let o_ring_recess = centered_cylinder(
            format!("pressure_perfusion_lane_{lane}_reservoir_bulkhead_or_ring_face"),
            15.0 / 2.0,
            4.0,
            36,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -(MANIFOLD_Y / 2.0 + 1.0), 14.0);
        let pressure_tap = centered_cylinder(
            format!("pressure_perfusion_lane_{lane}_reservoir_pressure_tap"),
            SENSOR_PORT_D / 2.0,
            MANIFOLD_Z + 8.0,
            22,
        )
        .translate(x, 0.0, 0.0);
        cuts = cuts + bulkhead + o_ring_recess + pressure_tap;
    }

    let left_supply = centered_cylinder(
        "pressure_perfusion_reservoir_manifold_left_controller_supply",
        8.0 / 2.0,
        MANIFOLD_X + 10.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, -16.0);
    let right_equalization = centered_cylinder(
        "pressure_perfusion_reservoir_manifold_equalization_reference_bore",
        4.0 / 2.0,
        MANIFOLD_X + 10.0,
        22,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 14.0, -22.0);

    let mut mount_holes = Part::empty("pressure_perfusion_reservoir_manifold_mount_holes");
    for (i, x) in [-315.0, -105.0, 105.0, 315.0].iter().enumerate() {
        mount_holes = mount_holes
            + centered_cylinder(
                format!("pressure_perfusion_reservoir_manifold_m5_mount_{i}"),
                5.4 / 2.0,
                MANIFOLD_Y + 8.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 0.0, -22.0);
    }

    body - cuts - left_supply - right_equalization - mount_holes
        + lane_equal_length_guides("reservoir")
        + grouped_controller_headers()
}

fn lane_equal_length_guides(prefix: &str) -> Part {
    let mut guides = Part::empty(format!("pressure_perfusion_{prefix}_equal_length_guides"));

    for lane in 0..LANES {
        let x = lane_x(lane);
        let serpentine = centered_cube(
            format!("pressure_perfusion_{prefix}_lane_{lane}_equal_length_routing_land"),
            34.0,
            8.0,
            9.0,
        )
        .translate(x, -(MANIFOLD_Y / 2.0 + 7.0), -26.0);
        let branch = centered_cube(
            format!("pressure_perfusion_{prefix}_lane_{lane}_branch_land"),
            8.0,
            30.0,
            9.0,
        )
        .translate(x, -(MANIFOLD_Y / 2.0 + 18.0), -10.0);
        guides = guides + serpentine + branch;
    }

    guides
}

fn grouped_controller_headers() -> Part {
    let mut headers = Part::empty("pressure_perfusion_grouped_controller_headers");
    for controller in 0..PRESSURE_CONTROLLERS {
        let first_lane = controller * LANES_PER_CONTROLLER;
        let second_lane = first_lane + LANES_PER_CONTROLLER - 1;
        let x0 = lane_x(first_lane);
        let x1 = lane_x(second_lane);
        let x_mid = (x0 + x1) / 2.0;
        let header = centered_cube(
            format!("pressure_perfusion_controller_{controller}_two_lane_header_land"),
            (x1 - x0).abs() + 46.0,
            14.0,
            16.0,
        )
        .translate(x_mid, MANIFOLD_Y / 2.0 + 7.0, -12.0);
        let port = centered_cylinder(
            format!("pressure_perfusion_controller_{controller}_supply_port_land"),
            11.0,
            8.0,
            36,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x_mid, MANIFOLD_Y / 2.0 + 16.0, -12.0);
        headers = headers + header + port;
    }
    headers
}

fn vacuum_reference_manifold() -> Part {
    let body = centered_cube(
        "pressure_perfusion_vacuum_reference_manifold_body",
        MANIFOLD_X - 120.0,
        46.0,
        54.0,
    );

    let main_bore = centered_cylinder(
        "pressure_perfusion_vacuum_reference_main_bore",
        7.0 / 2.0,
        MANIFOLD_X - 96.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, 4.0);
    let vacuum_source = centered_cylinder(
        "pressure_perfusion_vacuum_reference_source_port",
        12.0 / 2.0,
        52.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-(MANIFOLD_X / 2.0 - 92.0), 0.0, 4.0);

    let mut reference_ports = Part::empty("pressure_perfusion_vacuum_reference_lane_ports");
    for lane in 0..LANES {
        reference_ports = reference_ports
            + centered_cylinder(
                format!("pressure_perfusion_lane_{lane}_vacuum_reference_port"),
                5.0 / 2.0,
                50.0,
                22,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(lane_x(lane), 0.0, -12.0);
    }

    let water_trap = centered_cylinder(
        "pressure_perfusion_vacuum_reference_condensate_trap_envelope",
        30.0,
        72.0,
        48,
    )
    .translate(MANIFOLD_X / 2.0 - 126.0, 0.0, 60.0);
    let trap_socket = centered_cylinder(
        "pressure_perfusion_vacuum_reference_trap_socket",
        18.0,
        26.0,
        40,
    )
    .translate(MANIFOLD_X / 2.0 - 126.0, 0.0, 28.0);
    let low_range_sensor_pocket = centered_cube(
        "pressure_perfusion_vacuum_reference_low_range_sensor_pocket",
        58.0,
        22.0,
        26.0,
    )
    .translate(0.0, 36.0, 18.0);
    let reference_regulator_relief =
        relief_valve("vacuum_reference_relief").translate(-(MANIFOLD_X / 2.0 - 188.0), 8.0, 42.0);

    body - main_bore - vacuum_source - reference_ports
        + water_trap
        + trap_socket
        + low_range_sensor_pocket
        + reference_regulator_relief
}

fn filter_relief_sensor_bank() -> Part {
    let backer = centered_cube(
        "pressure_perfusion_filter_relief_sensor_bank_backer",
        MANIFOLD_X,
        16.0,
        178.0,
    );
    let mut features = Part::empty("pressure_perfusion_filter_relief_sensor_bank_features");

    for controller in 0..PRESSURE_CONTROLLERS {
        let x = filter_x(controller);
        features = features
            + sterile_filter_clip(controller).translate(x, -24.0, 48.0)
            + relief_valve(format!("controller_{controller}_relief")).translate(x, 24.0, -12.0)
            + controller_pressure_reference_port(controller).translate(x, -28.0, -66.0);
    }

    for lane in 0..LANES {
        let x = lane_x(lane);
        features = features + lane_pressure_sensor_pocket(lane).translate(x, 22.0, -72.0);
    }

    backer + features
}

fn sterile_filter_clip(controller: usize) -> Part {
    let filter = centered_cylinder(
        format!("pressure_perfusion_controller_{controller}_sterile_gas_filter_envelope"),
        FILTER_D / 2.0,
        FILTER_LEN,
        40,
    )
    .rotate(0.0, 90.0, 0.0);
    let left_clip = centered_cube(
        format!("pressure_perfusion_controller_{controller}_left_filter_clip"),
        10.0,
        24.0,
        FILTER_D + 10.0,
    )
    .translate(-(FILTER_LEN / 2.0 + 7.0), 0.0, 0.0);
    let right_clip = centered_cube(
        format!("pressure_perfusion_controller_{controller}_right_filter_clip"),
        10.0,
        24.0,
        FILTER_D + 10.0,
    )
    .translate(FILTER_LEN / 2.0 + 7.0, 0.0, 0.0);
    let filter_clearance = centered_cylinder(
        format!("pressure_perfusion_controller_{controller}_filter_clip_clearance"),
        FILTER_D / 2.0 + 1.0,
        FILTER_LEN + 26.0,
        40,
    )
    .rotate(0.0, 90.0, 0.0);

    filter + (left_clip + right_clip - filter_clearance)
}

fn relief_valve(name: impl Into<String>) -> Part {
    let name = name.into();
    let body = centered_cylinder(
        format!("pressure_perfusion_{name}_relief_valve_body"),
        RELIEF_D / 2.0,
        34.0,
        36,
    );
    let hex = centered_cube(
        format!("pressure_perfusion_{name}_relief_valve_hex_land"),
        22.0,
        22.0,
        10.0,
    )
    .translate(0.0, 0.0, -20.0);
    let vent = centered_cylinder(
        format!("pressure_perfusion_{name}_relief_valve_vent_hole"),
        3.0 / 2.0,
        38.0,
        18,
    );

    body + hex - vent
}

fn controller_pressure_reference_port(controller: usize) -> Part {
    let boss = centered_cylinder(
        format!("pressure_perfusion_controller_{controller}_reference_port_boss"),
        11.0,
        12.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0);
    let port = centered_cylinder(
        format!("pressure_perfusion_controller_{controller}_reference_port_clearance"),
        5.2 / 2.0,
        16.0,
        22,
    )
    .rotate(90.0, 0.0, 0.0);

    boss - port
}

fn lane_pressure_sensor_pocket(lane: usize) -> Part {
    let sensor = centered_cube(
        format!("pressure_perfusion_lane_{lane}_commercial_pressure_sensor_pocket"),
        SENSOR_X,
        SENSOR_Y,
        SENSOR_Z,
    );
    let port = centered_cylinder(
        format!("pressure_perfusion_lane_{lane}_sensor_pneumatic_port"),
        SENSOR_PORT_D / 2.0,
        SENSOR_Y + 4.0,
        22,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, 0.0, -2.0);
    let cable = centered_cube(
        format!("pressure_perfusion_lane_{lane}_sensor_cable_exit"),
        14.0,
        SENSOR_Y + 4.0,
        8.0,
    )
    .translate(0.0, 0.0, 8.0);

    sensor - port - cable
}

fn calibration_service_bar() -> Part {
    let bar = centered_cube(
        "pressure_perfusion_calibration_service_bar_body",
        MANIFOLD_X,
        44.0,
        58.0,
    );

    let mut cuts = Part::empty("pressure_perfusion_calibration_service_bar_cuts");
    for lane in 0..LANES {
        let x = lane_x(lane);
        cuts = cuts
            + centered_cylinder(
                format!("pressure_perfusion_lane_{lane}_calibration_quick_connect"),
                CAL_PORT_D / 2.0,
                50.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, 10.0)
            + centered_cylinder(
                format!("pressure_perfusion_lane_{lane}_zero_reference_pin_valve"),
                3.8 / 2.0,
                50.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, -14.0);
    }

    let inlet_calibrator_port = centered_cylinder(
        "pressure_perfusion_traceable_calibrator_inlet_port",
        12.0 / 2.0,
        52.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-(MANIFOLD_X / 2.0 - 34.0), 0.0, 0.0);
    let capped_exhaust_port = centered_cylinder(
        "pressure_perfusion_calibration_capped_exhaust_port",
        12.0 / 2.0,
        52.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(MANIFOLD_X / 2.0 - 34.0, 0.0, 0.0);
    cuts = cuts + inlet_calibrator_port + capped_exhaust_port;

    bar - cuts + calibration_cover_lip() + lane_equal_length_guides("calibration")
}

fn calibration_cover_lip() -> Part {
    let upper = centered_cube(
        "pressure_perfusion_calibration_bar_upper_cover_lip",
        MANIFOLD_X - 48.0,
        10.0,
        8.0,
    )
    .translate(0.0, -27.0, 34.0);
    let lower = centered_cube(
        "pressure_perfusion_calibration_bar_lower_cover_lip",
        MANIFOLD_X - 48.0,
        10.0,
        8.0,
    )
    .translate(0.0, -27.0, -34.0);

    upper + lower
}

fn cable_pneumatic_strain_relief() -> Part {
    let rail = centered_cube(
        "pressure_perfusion_cable_pneumatic_strain_relief_rail",
        PANEL_X - 80.0,
        46.0,
        36.0,
    );
    let mut cuts = Part::empty("pressure_perfusion_cable_pneumatic_strain_relief_cuts");

    for lane in 0..LANES {
        let x = lane_x(lane);
        cuts = cuts
            + centered_cylinder(
                format!("pressure_perfusion_lane_{lane}_pneumatic_tube_strain_relief_groove"),
                5.4 / 2.0,
                52.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, 6.0)
            + centered_cube(
                format!("pressure_perfusion_lane_{lane}_tube_zip_tie_slot"),
                16.0,
                52.0,
                5.0,
            )
            .translate(x, 0.0, -10.0);
    }

    for (i, x) in [-260.0, -86.0, 86.0, 260.0].iter().enumerate() {
        cuts = cuts
            + centered_cube(
                format!("pressure_perfusion_controller_cable_bundle_slot_{i}"),
                46.0,
                52.0,
                12.0,
            )
            .translate(*x, 0.0, -1.0);
    }

    let left_ear = tie_down_ear("left").translate(-(PANEL_X / 2.0 - 76.0), 0.0, 0.0);
    let right_ear = tie_down_ear("right").translate(PANEL_X / 2.0 - 76.0, 0.0, 0.0);
    let pneumatic_comb = pneumatic_comb_teeth();

    rail - cuts + left_ear + right_ear + pneumatic_comb
}

fn pneumatic_comb_teeth() -> Part {
    let mut teeth = Part::empty("pressure_perfusion_pneumatic_comb_teeth");
    for lane in 0..LANES {
        let x = lane_x(lane);
        teeth = teeth
            + centered_cube(
                format!("pressure_perfusion_lane_{lane}_front_tube_comb_tooth"),
                6.0,
                20.0,
                30.0,
            )
            .translate(x - 9.0, -30.0, 3.0)
            + centered_cube(
                format!("pressure_perfusion_lane_{lane}_rear_tube_comb_tooth"),
                6.0,
                20.0,
                30.0,
            )
            .translate(x + 9.0, -30.0, 3.0);
    }
    teeth
}

fn tie_down_ear(side: &str) -> Part {
    let ear = centered_cube(
        format!("pressure_perfusion_strain_relief_{side}_tie_down_ear"),
        34.0,
        56.0,
        14.0,
    );
    let screw = centered_cylinder(
        format!("pressure_perfusion_strain_relief_{side}_m5_clearance"),
        5.4 / 2.0,
        18.0,
        24,
    )
    .translate(0.0, 0.0, 0.0);

    ear - screw
}

fn base_mount_points() -> [(f64, f64); 8] {
    [
        (-(BASE_X / 2.0 - 44.0), -(BASE_Y / 2.0 - 42.0)),
        (BASE_X / 2.0 - 44.0, -(BASE_Y / 2.0 - 42.0)),
        (-(BASE_X / 2.0 - 44.0), BASE_Y / 2.0 - 42.0),
        (BASE_X / 2.0 - 44.0, BASE_Y / 2.0 - 42.0),
        (0.0, -(BASE_Y / 2.0 - 42.0)),
        (0.0, BASE_Y / 2.0 - 42.0),
        (-(BASE_X / 2.0 - 44.0), 0.0),
        (BASE_X / 2.0 - 44.0, 0.0),
    ]
}

fn controller_x(controller: usize) -> f64 {
    -((PRESSURE_CONTROLLERS as f64 - 1.0) * CONTROLLER_PITCH_X) / 2.0
        + controller as f64 * CONTROLLER_PITCH_X
}

fn filter_x(filter: usize) -> f64 {
    -((STERILE_FILTERS as f64 - 1.0) * FILTER_PITCH_X) / 2.0 + filter as f64 * FILTER_PITCH_X
}

fn lane_x(lane: usize) -> f64 {
    -((LANES as f64 - 1.0) * LANE_PITCH_X) / 2.0 + lane as f64 * LANE_PITCH_X
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lane_counts_match_parallel_pressure_driven_layout() {
        assert_eq!(LANES, 8);
        assert_eq!(RESERVOIR_PORTS, LANES);
        assert_eq!(PRESSURE_SENSORS, LANES);
        assert_eq!(CALIBRATION_PORTS, LANES);
        assert_eq!(PRESSURE_CONTROLLERS * LANES_PER_CONTROLLER, LANES);
    }

    #[test]
    fn purchased_controller_envelopes_fit_on_panel_without_implying_diy_controls() {
        let controller_span =
            (PRESSURE_CONTROLLERS as f64 - 1.0) * CONTROLLER_PITCH_X + CONTROLLER_ENV_X;
        assert!(controller_span < PANEL_X - 80.0);
        assert!(CONTROLLER_ENV_Z < PANEL_Z / 2.0);
        assert!(CONTROLLER_FACE_X < CONTROLLER_ENV_X);
        assert!(CONTROLLER_FACE_Z < CONTROLLER_ENV_Z);
    }

    #[test]
    fn lane_pitch_clears_bulkheads_sensors_and_tube_comb() {
        assert!(LANE_PITCH_X > BULKHEAD_PORT_D + SENSOR_X + 18.0);
        assert!(LANE_PITCH_X > CAL_PORT_D + 50.0);
        assert!(lane_x(0) > -MANIFOLD_X / 2.0 + 34.0);
        assert!(lane_x(LANES - 1) < MANIFOLD_X / 2.0 - 34.0);
    }

    #[test]
    fn safety_and_calibration_features_cover_each_control_channel() {
        assert_eq!(STERILE_FILTERS, PRESSURE_CONTROLLERS);
        assert_eq!(RELIEF_VALVES, PRESSURE_CONTROLLERS);
        assert!(FILTER_PITCH_X > FILTER_LEN + 36.0);
        assert!(MANIFOLD_Z >= 60.0);
    }
}
