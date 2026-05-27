use vcad::{centered_cube, centered_cylinder, Part};

// Environmental utility skid for the closed isolator / sealed culture module stack.
//
// Purpose:
// - Keep gas cylinders, mixing, humidification, pressure controls, and thermal loop
//   serviceable outside the Grade A isolator volume.
// - Present validated bulkhead families to the isolator/module bays instead of ad hoc routing.
// - Make gas, water, condensate, electrical, sensor, and drain interfaces explicit for BOM/CAD.
//
// This is an architecture CAD model for service packaging, not a certified gas skid.

const SKID_X: f64 = 1250.0;
const SKID_Y: f64 = 540.0;
const SKID_Z: f64 = 1680.0;
const FRAME_W: f64 = 40.0;
const BASE_Z: f64 = 90.0;

const PANEL_X: f64 = 520.0;
const PANEL_Y: f64 = 36.0;
const GAS_PANEL_Z: f64 = 620.0;
const THERMAL_PANEL_Z: f64 = 470.0;
const SENSOR_PANEL_Z: f64 = 360.0;

fn main() {
    let frame = skid_frame();
    frame
        .write_stl("output/environmental_utility_skid_frame.stl")
        .unwrap();
    println!("Exported: output/environmental_utility_skid_frame.stl");

    let gas = gas_mixing_panel();
    gas.write_stl("output/environmental_utility_skid_gas_panel.stl")
        .unwrap();
    println!("Exported: output/environmental_utility_skid_gas_panel.stl");

    let humidifier = humidification_module();
    humidifier
        .write_stl("output/environmental_utility_skid_humidification_module.stl")
        .unwrap();
    println!("Exported: output/environmental_utility_skid_humidification_module.stl");

    let thermal = thermal_loop_module();
    thermal
        .write_stl("output/environmental_utility_skid_thermal_loop.stl")
        .unwrap();
    println!("Exported: output/environmental_utility_skid_thermal_loop.stl");

    let pressure = pressure_sensor_panel();
    pressure
        .write_stl("output/environmental_utility_skid_pressure_panel.stl")
        .unwrap();
    println!("Exported: output/environmental_utility_skid_pressure_panel.stl");

    let assembly = frame
        + gas.translate(-285.0, -SKID_Y / 2.0 + 48.0, 820.0)
        + humidifier.translate(330.0, -SKID_Y / 2.0 + 54.0, 620.0)
        + thermal.translate(320.0, -SKID_Y / 2.0 + 54.0, 1160.0)
        + pressure.translate(-285.0, -SKID_Y / 2.0 + 52.0, 1410.0)
        + cylinder_bay().translate(-410.0, 120.0, 380.0)
        + utility_bulkhead().translate(0.0, SKID_Y / 2.0 + 30.0, 970.0);

    assembly
        .write_stl("output/environmental_utility_skid_assembly.stl")
        .unwrap();
    println!("Exported: output/environmental_utility_skid_assembly.stl");

    println!(
        "Environmental utility skid: {:.0}mm x {:.0}mm x {:.0}mm frame with gas mixing, humidification, pressure cascade, thermal loop, cylinder restraints, drains, and rear service bulkhead.",
        SKID_X, SKID_Y, SKID_Z
    );
}

fn skid_frame() -> Part {
    let base = centered_cube("env_skid_base_pan", SKID_X, SKID_Y, BASE_Z);
    let sump = centered_cube("env_skid_base_sump", SKID_X - 90.0, SKID_Y - 90.0, 24.0).translate(
        0.0,
        0.0,
        BASE_Z / 2.0 - 12.0,
    );
    let drain = centered_cylinder("env_skid_floor_drain", 16.0 / 2.0, 42.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(SKID_X / 2.0 - 88.0, -SKID_Y / 2.0 - 4.0, -18.0);

    let mut posts = Part::empty("env_skid_frame_posts");
    for (i, (x, y)) in [
        (
            -(SKID_X / 2.0 - FRAME_W / 2.0),
            -(SKID_Y / 2.0 - FRAME_W / 2.0),
        ),
        (
            SKID_X / 2.0 - FRAME_W / 2.0,
            -(SKID_Y / 2.0 - FRAME_W / 2.0),
        ),
        (
            -(SKID_X / 2.0 - FRAME_W / 2.0),
            SKID_Y / 2.0 - FRAME_W / 2.0,
        ),
        (SKID_X / 2.0 - FRAME_W / 2.0, SKID_Y / 2.0 - FRAME_W / 2.0),
        (0.0, -(SKID_Y / 2.0 - FRAME_W / 2.0)),
        (0.0, SKID_Y / 2.0 - FRAME_W / 2.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(format!("env_skid_post_{i}"), FRAME_W, FRAME_W, SKID_Z).translate(
                *x,
                *y,
                SKID_Z / 2.0,
            );
    }

    let mut rails = Part::empty("env_skid_frame_rails");
    for (i, z) in [BASE_Z + 280.0, 850.0, 1330.0, SKID_Z - FRAME_W / 2.0]
        .iter()
        .enumerate()
    {
        rails =
            rails
                + centered_cube(format!("env_skid_front_rail_{i}"), SKID_X, FRAME_W, FRAME_W)
                    .translate(0.0, -SKID_Y / 2.0 + FRAME_W / 2.0, *z)
                + centered_cube(format!("env_skid_rear_rail_{i}"), SKID_X, FRAME_W, FRAME_W)
                    .translate(0.0, SKID_Y / 2.0 - FRAME_W / 2.0, *z);
    }

    base - sump - drain + posts + rails + caster_plates() + service_trench()
}

fn caster_plates() -> Part {
    let mut plates = Part::empty("env_skid_caster_plates");
    for (i, (x, y)) in [
        (-(SKID_X / 2.0 - 75.0), -(SKID_Y / 2.0 - 75.0)),
        (SKID_X / 2.0 - 75.0, -(SKID_Y / 2.0 - 75.0)),
        (-(SKID_X / 2.0 - 75.0), SKID_Y / 2.0 - 75.0),
        (SKID_X / 2.0 - 75.0, SKID_Y / 2.0 - 75.0),
    ]
    .iter()
    .enumerate()
    {
        let plate = centered_cube(format!("env_skid_caster_plate_{i}"), 96.0, 96.0, 10.0)
            .translate(*x, *y, -BASE_Z / 2.0 - 5.0);
        let stem = centered_cylinder(
            format!("env_skid_caster_stem_clearance_{i}"),
            13.0 / 2.0,
            12.0,
            28,
        )
        .translate(*x, *y, -BASE_Z / 2.0 - 5.0);
        plates = plates + (plate - stem);
    }
    plates
}

fn service_trench() -> Part {
    let tray = centered_cube("env_skid_rear_service_trench", SKID_X - 130.0, 58.0, 38.0).translate(
        0.0,
        SKID_Y / 2.0 - 82.0,
        BASE_Z / 2.0 + 19.0,
    );
    let cover_lip = centered_cube(
        "env_skid_service_trench_cover_lip",
        SKID_X - 160.0,
        10.0,
        8.0,
    )
    .translate(0.0, SKID_Y / 2.0 - 112.0, BASE_Z / 2.0 + 42.0);
    tray + cover_lip
}

fn gas_mixing_panel() -> Part {
    let panel = centered_cube("env_skid_gas_panel_body", PANEL_X, PANEL_Y, GAS_PANEL_Z);
    let mut cuts = Part::empty("env_skid_gas_panel_cuts");

    for (i, x) in [-195.0, -65.0, 65.0, 195.0].iter().enumerate() {
        cuts = cuts
            + centered_cylinder(
                format!("env_skid_regulator_cutout_{i}"),
                58.0 / 2.0,
                PANEL_Y + 8.0,
                48,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 0.0, 195.0);
    }

    for (i, (x, z)) in [
        (-210.0, 70.0),
        (-150.0, 70.0),
        (-90.0, 70.0),
        (-30.0, 70.0),
        (30.0, 70.0),
        (90.0, 70.0),
        (150.0, 70.0),
        (210.0, 70.0),
        (-150.0, -70.0),
        (-90.0, -70.0),
        (-30.0, -70.0),
        (30.0, -70.0),
        (90.0, -70.0),
        (150.0, -70.0),
    ]
    .iter()
    .enumerate()
    {
        cuts = cuts
            + centered_cylinder(
                format!("env_skid_gas_bulkhead_{i}"),
                10.0 / 2.0,
                PANEL_Y + 8.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 0.0, *z);
    }

    let mixing_block = centered_cube("env_skid_gas_mixing_block", 350.0, 44.0, 70.0).translate(
        0.0,
        -(PANEL_Y / 2.0 + 18.0),
        -185.0,
    );
    let sterile_filter_bank = filter_bank("gas_filter_bank", 5).translate(0.0, -42.0, -15.0);
    let label_lands = panel_label_strip("gas_upper", PANEL_X - 70.0).translate(0.0, -22.0, 288.0)
        + panel_label_strip("gas_lower", PANEL_X - 70.0).translate(0.0, -22.0, -292.0);

    panel - cuts + mixing_block + sterile_filter_bank + label_lands + gas_flowmeter_bank()
}

fn gas_flowmeter_bank() -> Part {
    let mut meters = Part::empty("env_skid_gas_flowmeter_bank");
    for (i, x) in [-180.0, -90.0, 0.0, 90.0, 180.0].iter().enumerate() {
        meters = meters
            + centered_cube(format!("env_skid_mfc_body_{i}"), 58.0, 48.0, 96.0)
                .translate(*x, -42.0, -120.0)
            - centered_cube(format!("env_skid_mfc_display_recess_{i}"), 42.0, 12.0, 34.0)
                .translate(*x, -68.0, -120.0);
    }
    meters
}

fn humidification_module() -> Part {
    let reservoir = centered_cylinder("env_skid_humidifier_reservoir", 86.0, 250.0, 56)
        .translate(-120.0, 0.0, 0.0);
    let reservoir_cut = centered_cylinder("env_skid_humidifier_reservoir_cavity", 74.0, 252.0, 56)
        .translate(-120.0, 0.0, 14.0);
    let lid =
        centered_cylinder("env_skid_humidifier_lid", 94.0, 18.0, 56).translate(-120.0, 0.0, 134.0);
    let condenser = centered_cube("env_skid_condensate_knockout_box", 150.0, 128.0, 210.0)
        .translate(100.0, 0.0, 10.0);
    let condenser_cavity =
        centered_cube("env_skid_condensate_knockout_cavity", 126.0, 104.0, 180.0)
            .translate(100.0, 0.0, 22.0);
    let drip_return = centered_cylinder("env_skid_condensate_return_port", 8.0 / 2.0, 150.0, 24)
        .rotate(0.0, 90.0, 0.0)
        .translate(-10.0, 0.0, -85.0);
    let dewpoint_probe = centered_cylinder("env_skid_dewpoint_probe_port", 10.0 / 2.0, 150.0, 24)
        .rotate(0.0, 90.0, 0.0)
        .translate(10.0, 0.0, 88.0);

    (reservoir - reservoir_cut) + lid + (condenser - condenser_cavity)
        - drip_return
        - dewpoint_probe
        + humidifier_mounting_saddle()
}

fn humidifier_mounting_saddle() -> Part {
    centered_cube("env_skid_humidifier_saddle_left", 190.0, 34.0, 28.0)
        .translate(-120.0, -108.0, -142.0)
        + centered_cube("env_skid_humidifier_saddle_right", 190.0, 34.0, 28.0)
            .translate(-120.0, 108.0, -142.0)
        + centered_cube("env_skid_humidifier_condensate_mount", 188.0, 150.0, 22.0)
            .translate(100.0, 0.0, -112.0)
}

fn thermal_loop_module() -> Part {
    let plate = centered_cube(
        "env_skid_thermal_loop_panel",
        PANEL_X,
        PANEL_Y,
        THERMAL_PANEL_Z,
    );
    let pump_a = pump_placeholder("a").translate(-150.0, -42.0, 95.0);
    let pump_b = pump_placeholder("b").translate(150.0, -42.0, 95.0);
    let heat_exchanger = centered_cube("env_skid_plate_heat_exchanger", 270.0, 72.0, 170.0)
        .translate(0.0, -48.0, -82.0);
    let manifold = centered_cube("env_skid_thermal_manifold_bar", 420.0, 46.0, 42.0)
        .translate(0.0, -42.0, 2.0);
    let ports = thermal_loop_ports();
    plate + pump_a + pump_b + heat_exchanger + manifold - ports
        + panel_label_strip("thermal", PANEL_X - 80.0).translate(0.0, -24.0, -215.0)
}

fn pump_placeholder(name: &str) -> Part {
    let body = centered_cylinder(format!("env_skid_circulation_pump_{name}"), 54.0, 78.0, 48)
        .rotate(90.0, 0.0, 0.0);
    let motor = centered_cube(format!("env_skid_pump_motor_{name}"), 82.0, 76.0, 72.0)
        .translate(0.0, -42.0, 0.0);
    let inlet = centered_cylinder(format!("env_skid_pump_inlet_{name}"), 14.0 / 2.0, 76.0, 24)
        .rotate(0.0, 90.0, 0.0)
        .translate(-64.0, 0.0, 0.0);
    let outlet = centered_cylinder(format!("env_skid_pump_outlet_{name}"), 14.0 / 2.0, 76.0, 24)
        .rotate(0.0, 90.0, 0.0)
        .translate(64.0, 0.0, 0.0);
    body + motor - inlet - outlet
}

fn thermal_loop_ports() -> Part {
    let mut ports = Part::empty("env_skid_thermal_loop_port_cuts");
    for (i, (x, z)) in [
        (-210.0, -5.0),
        (-150.0, -5.0),
        (-90.0, -5.0),
        (-30.0, -5.0),
        (30.0, -5.0),
        (90.0, -5.0),
        (150.0, -5.0),
        (210.0, -5.0),
    ]
    .iter()
    .enumerate()
    {
        ports = ports
            + centered_cylinder(
                format!("env_skid_thermal_bulkhead_{i}"),
                12.0 / 2.0,
                PANEL_Y + 110.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 0.0, *z);
    }
    ports
}

fn pressure_sensor_panel() -> Part {
    let panel = centered_cube(
        "env_skid_pressure_panel_body",
        PANEL_X,
        PANEL_Y,
        SENSOR_PANEL_Z,
    );
    let mut cuts = Part::empty("env_skid_pressure_sensor_cuts");

    for (i, x) in [-210.0, -140.0, -70.0, 0.0, 70.0, 140.0, 210.0]
        .iter()
        .enumerate()
    {
        cuts = cuts
            + centered_cube(
                format!("env_skid_dp_transmitter_pocket_{i}"),
                46.0,
                PANEL_Y + 8.0,
                70.0,
            )
            .translate(*x, 0.0, 80.0)
            + centered_cylinder(
                format!("env_skid_dp_tube_port_high_{i}"),
                4.0 / 2.0,
                PANEL_Y + 8.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x - 10.0, 0.0, 20.0)
            + centered_cylinder(
                format!("env_skid_dp_tube_port_low_{i}"),
                4.0 / 2.0,
                PANEL_Y + 8.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x + 10.0, 0.0, 20.0);
    }

    let alarm_stack = centered_cube("env_skid_pressure_alarm_stack", 72.0, 54.0, 118.0).translate(
        PANEL_X / 2.0 - 65.0,
        -48.0,
        -96.0,
    );
    let sample_manifold = centered_cube("env_skid_pressure_sample_manifold", 390.0, 42.0, 34.0)
        .translate(-55.0, -44.0, -112.0);

    panel - cuts
        + alarm_stack
        + sample_manifold
        + panel_label_strip("pressure", PANEL_X - 90.0).translate(0.0, -24.0, 166.0)
}

fn cylinder_bay() -> Part {
    let floor = centered_cube("env_skid_cylinder_bay_floor", 430.0, 260.0, 28.0);
    let rear = centered_cube("env_skid_cylinder_bay_rear_guard", 430.0, 24.0, 460.0)
        .translate(0.0, 118.0, 230.0);
    let mut saddles = Part::empty("env_skid_cylinder_saddles");
    for (i, x) in [-135.0, 0.0, 135.0].iter().enumerate() {
        let lower = centered_cylinder(
            format!("env_skid_cylinder_lower_saddle_{i}"),
            46.0,
            38.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, -65.0, 72.0);
        let strap = centered_cube(
            format!("env_skid_cylinder_strap_band_{i}"),
            96.0,
            14.0,
            18.0,
        )
        .translate(*x, -112.0, 270.0);
        let chain_anchor =
            centered_cylinder(format!("env_skid_cylinder_chain_anchor_{i}"), 6.0, 14.0, 24)
                .rotate(90.0, 0.0, 0.0)
                .translate(*x, -122.0, 350.0);
        saddles = saddles + lower + strap + chain_anchor;
    }
    floor + rear + saddles + cylinder_keepouts()
}

fn cylinder_keepouts() -> Part {
    let mut keepouts = Part::empty("env_skid_cylinder_keepout_rings");
    for (i, x) in [-135.0, 0.0, 135.0].iter().enumerate() {
        let ring = centered_cylinder(format!("env_skid_cylinder_keepout_ring_{i}"), 52.0, 8.0, 48)
            .translate(*x, -35.0, 155.0);
        let core = centered_cylinder(
            format!("env_skid_cylinder_keepout_core_{i}"),
            42.0,
            10.0,
            48,
        )
        .translate(*x, -35.0, 155.0);
        keepouts = keepouts + (ring - core);
    }
    keepouts
}

fn utility_bulkhead() -> Part {
    let body = centered_cube(
        "env_skid_rear_utility_bulkhead",
        SKID_X - 180.0,
        48.0,
        520.0,
    );
    let mut cuts = Part::empty("env_skid_rear_bulkhead_cuts");

    for (i, x) in [-450.0, -390.0, -330.0, -270.0, -210.0, -150.0]
        .iter()
        .enumerate()
    {
        cuts = cuts
            + centered_cylinder(format!("env_skid_rear_gas_out_{i}"), 10.0 / 2.0, 58.0, 24)
                .rotate(90.0, 0.0, 0.0)
                .translate(*x, 0.0, 120.0);
    }
    for (i, x) in [-70.0, -20.0, 30.0, 80.0, 130.0, 180.0].iter().enumerate() {
        cuts = cuts
            + centered_cylinder(
                format!("env_skid_rear_thermal_out_{i}"),
                12.0 / 2.0,
                58.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 0.0, 25.0);
    }
    for (i, x) in [260.0, 310.0, 360.0, 410.0].iter().enumerate() {
        cuts = cuts
            + centered_cylinder(
                format!("env_skid_rear_condensate_drain_{i}"),
                10.0 / 2.0,
                58.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, 0.0, -85.0);
    }

    let cable_transit = centered_cube("env_skid_rear_cable_transit", 160.0, 58.0, 70.0)
        .translate(330.0, 0.0, 120.0);
    body - cuts - cable_transit
        + panel_label_strip("rear_bulkhead_upper", SKID_X - 260.0).translate(0.0, -28.0, 235.0)
        + panel_label_strip("rear_bulkhead_lower", SKID_X - 260.0).translate(0.0, -28.0, -235.0)
}

fn filter_bank(name: &str, count: usize) -> Part {
    let mut bank = Part::empty(format!("env_skid_{name}"));
    for i in 0..count {
        let x = -160.0 + i as f64 * 80.0;
        let filter = centered_cylinder(format!("env_skid_{name}_filter_{i}"), 22.0, 78.0, 32)
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, 0.0);
        let clamp = centered_cube(format!("env_skid_{name}_clamp_{i}"), 54.0, 12.0, 20.0)
            .translate(x, -42.0, 0.0);
        bank = bank + filter + clamp;
    }
    bank
}

fn panel_label_strip(name: &str, width: f64) -> Part {
    centered_cube(format!("env_skid_label_strip_{name}"), width, 4.0, 10.0)
}
