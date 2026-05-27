use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
use vcad::{centered_cube, centered_cylinder, Part};

// Inline media conditioning and QC module for the last sterile media path before
// chip cassette handoff.
//
// Intent:
// - Equilibrate incoming media thermally in a water-jacket block before it
//   reaches cassette feeds.
// - Package pH, dissolved oxygen, conductivity, pressure, and flow sensing in
//   an inline serviceable cartridge path.
// - Include sterile sampling, calibration fluids, filtering, membrane degassing,
//   bypass/waste routing, and a positive cassette handoff datum.
//
// This is packaging and architecture CAD. Wetted materials, sterile connector
// selection, sensor chemistry, membrane sizing, calibration acceptance criteria,
// and biological process validation remain separate gates.

const OUTPUTS: &[&str] = &[
    "output/inline_media_conditioning_qc_module_baseplate.stl",
    "output/inline_media_conditioning_qc_module_water_jacket_block.stl",
    "output/inline_media_conditioning_qc_module_filter_holder.stl",
    "output/inline_media_conditioning_qc_module_degassing_membrane.stl",
    "output/inline_media_conditioning_qc_module_sensor_qc_manifold.stl",
    "output/inline_media_conditioning_qc_module_pressure_flow_block.stl",
    "output/inline_media_conditioning_qc_module_sampling_loop.stl",
    "output/inline_media_conditioning_qc_module_calibration_port_bank.stl",
    "output/inline_media_conditioning_qc_module_bypass_waste_manifold.stl",
    "output/inline_media_conditioning_qc_module_cassette_handoff_datum.stl",
    "output/inline_media_conditioning_qc_module_assembly.stl",
];

const CASSETTE_COLS: usize = 4;
const CASSETTE_ROWS: usize = 5;
const CASSETTE_GUTTER: f64 = 5.0;
const CASSETTE_ARRAY_X: f64 =
    CASSETTE_COLS as f64 * REVC_CHIP_LENGTH + (CASSETTE_COLS as f64 - 1.0) * CASSETTE_GUTTER;
const CASSETTE_ARRAY_Y: f64 =
    CASSETTE_ROWS as f64 * REVC_CHIP_WIDTH + (CASSETTE_ROWS as f64 - 1.0) * CASSETTE_GUTTER;

const MODULE_X: f64 = 860.0;
const MODULE_Y: f64 = 520.0;
const DECK_Z: f64 = 18.0;
const SERVICE_CLEARANCE_Y: f64 = 86.0;
const TUBE_OD: f64 = 4.8;
const TUBE_CLEARANCE: f64 = 0.7;
const FLUID_BORE_D: f64 = TUBE_OD + TUBE_CLEARANCE;
const WATER_LINE_D: f64 = 8.0;
const SAMPLE_LOOP_D: f64 = 6.2;
const WASTE_BORE_D: f64 = 7.0;

const WATER_BLOCK_X: f64 = 320.0;
const WATER_BLOCK_Y: f64 = 112.0;
const WATER_BLOCK_Z: f64 = 44.0;
const WATER_CHANNELS: usize = 4;
const WATER_CHANNEL_PITCH_Y: f64 = 24.0;

const FILTER_HOLDER_X: f64 = 238.0;
const FILTER_HOLDER_Y: f64 = 118.0;
const FILTER_HOLDER_Z: f64 = 68.0;
const FILTER_BODY_D: f64 = 36.0;

const DEGASSER_X: f64 = 258.0;
const DEGASSER_Y: f64 = 108.0;
const DEGASSER_Z: f64 = 54.0;
const DEGASSER_MEMBRANE_WINDOWS: usize = 3;

const QC_BLOCK_X: f64 = 282.0;
const QC_BLOCK_Y: f64 = 94.0;
const QC_BLOCK_Z: f64 = 44.0;
const CHEM_SENSOR_COUNT: usize = 3; // pH, optical DO, conductivity.
const CHEM_SENSOR_PITCH_X: f64 = 70.0;

const PRESSURE_FLOW_X: f64 = 250.0;
const PRESSURE_FLOW_Y: f64 = 86.0;
const PRESSURE_FLOW_Z: f64 = 40.0;
const PRESSURE_SENSOR_COUNT: usize = 2;
const FLOW_WINDOW_X: f64 = 76.0;

const SAMPLE_LOOP_X: f64 = 236.0;
const SAMPLE_LOOP_Y: f64 = 132.0;
const SAMPLE_LOOP_Z: f64 = 42.0;
const SAMPLE_VALVE_COUNT: usize = 3;

const CAL_BANK_X: f64 = 238.0;
const CAL_BANK_Y: f64 = 92.0;
const CAL_BANK_Z: f64 = 48.0;
const CALIBRATION_PORTS: usize = 4; // low pH, neutral pH, conductivity, DO zero/span.
const CAL_PORT_PITCH_X: f64 = 50.0;

const BYPASS_X: f64 = 310.0;
const BYPASS_Y: f64 = 94.0;
const BYPASS_Z: f64 = 38.0;
const BYPASS_VALVES: usize = 4; // inlet isolate, bypass, sample, waste.

const HANDOFF_X: f64 = 224.0;
const HANDOFF_Y: f64 = 164.0;
const HANDOFF_Z: f64 = 30.0;
const ROW_PORT_D: f64 = 5.8;
const HANDOFF_PORT_SPAN_Y: f64 = 108.0;

const FILTER_POS: (f64, f64) = (-302.0, 150.0);
const WATER_POS: (f64, f64) = (-168.0, 40.0);
const DEGASSER_POS: (f64, f64) = (92.0, 132.0);
const QC_POS: (f64, f64) = (264.0, 54.0);
const PRESSURE_FLOW_POS: (f64, f64) = (232.0, -72.0);
const SAMPLE_POS: (f64, f64) = (16.0, -152.0);
const CAL_POS: (f64, f64) = (-218.0, -150.0);
const BYPASS_POS: (f64, f64) = (4.0, -34.0);
const HANDOFF_POS: (f64, f64) = (310.0, -156.0);

fn main() {
    let baseplate = baseplate();
    export(&baseplate, OUTPUTS[0]);

    let water_jacket = water_jacket_block();
    export(&water_jacket, OUTPUTS[1]);

    let filter = sterile_filter_holder();
    export(&filter, OUTPUTS[2]);

    let degasser = degassing_membrane_placeholder();
    export(&degasser, OUTPUTS[3]);

    let qc_manifold = sensor_qc_manifold();
    export(&qc_manifold, OUTPUTS[4]);

    let pressure_flow = pressure_flow_sensor_block();
    export(&pressure_flow, OUTPUTS[5]);

    let sampling_loop = sterile_sampling_loop();
    export(&sampling_loop, OUTPUTS[6]);

    let calibration = calibration_port_bank();
    export(&calibration, OUTPUTS[7]);

    let bypass_waste = bypass_waste_manifold();
    export(&bypass_waste, OUTPUTS[8]);

    let handoff = cassette_handoff_datum();
    export(&handoff, OUTPUTS[9]);

    let assembly = baseplate
        + filter.translate(
            FILTER_POS.0,
            FILTER_POS.1,
            DECK_Z / 2.0 + FILTER_HOLDER_Z / 2.0,
        )
        + water_jacket.translate(WATER_POS.0, WATER_POS.1, DECK_Z / 2.0 + WATER_BLOCK_Z / 2.0)
        + degasser.translate(
            DEGASSER_POS.0,
            DEGASSER_POS.1,
            DECK_Z / 2.0 + DEGASSER_Z / 2.0,
        )
        + qc_manifold.translate(QC_POS.0, QC_POS.1, DECK_Z / 2.0 + QC_BLOCK_Z / 2.0)
        + pressure_flow.translate(
            PRESSURE_FLOW_POS.0,
            PRESSURE_FLOW_POS.1,
            DECK_Z / 2.0 + PRESSURE_FLOW_Z / 2.0,
        )
        + sampling_loop.translate(
            SAMPLE_POS.0,
            SAMPLE_POS.1,
            DECK_Z / 2.0 + SAMPLE_LOOP_Z / 2.0,
        )
        + calibration.translate(CAL_POS.0, CAL_POS.1, DECK_Z / 2.0 + CAL_BANK_Z / 2.0)
        + bypass_waste.translate(BYPASS_POS.0, BYPASS_POS.1, DECK_Z / 2.0 + BYPASS_Z / 2.0)
        + handoff.translate(HANDOFF_POS.0, HANDOFF_POS.1, DECK_Z / 2.0 + HANDOFF_Z / 2.0)
        + routed_tube_placeholders();

    export(&assembly, OUTPUTS[10]);

    println!(
        "Inline media conditioning/QC module: {:.0}mm x {:.0}mm deck, {:.0}mm water-jacket block, {} chemical sensor pockets, {} pressure taps, flow pocket, sterile sampling loop, {} calibration ports, membrane degasser placeholder, filter holder, bypass/waste manifold, and {} row cassette handoff datum ports for a {}x{} Rev C cassette array ({:.0}mm x {:.0}mm).",
        MODULE_X,
        MODULE_Y,
        WATER_BLOCK_X,
        CHEM_SENSOR_COUNT,
        PRESSURE_SENSOR_COUNT,
        CALIBRATION_PORTS,
        CASSETTE_ROWS,
        CASSETTE_COLS,
        CASSETTE_ROWS,
        CASSETTE_ARRAY_X,
        CASSETTE_ARRAY_Y
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn baseplate() -> Part {
    let deck = centered_cube("inline_media_qc_baseplate", MODULE_X, MODULE_Y, DECK_Z);

    let wet_path_sump = centered_cube("inline_media_qc_wet_path_sump", MODULE_X - 124.0, 92.0, 8.0)
        .translate(12.0, -58.0, DECK_Z / 2.0 - 3.5);
    let calibration_sump = centered_cube(
        "inline_media_qc_calibration_sump",
        CAL_BANK_X + 64.0,
        CAL_BANK_Y + 34.0,
        8.0,
    )
    .translate(CAL_POS.0, CAL_POS.1, DECK_Z / 2.0 - 3.5);
    let filter_drip_pocket = centered_cube(
        "inline_media_qc_filter_drip_pocket",
        FILTER_HOLDER_X + 26.0,
        42.0,
        8.0,
    )
    .translate(FILTER_POS.0, FILTER_POS.1 - 58.0, DECK_Z / 2.0 - 3.5);
    let front_service_relief = centered_cube(
        "inline_media_qc_front_service_relief",
        MODULE_X - 112.0,
        SERVICE_CLEARANCE_Y,
        6.0,
    )
    .translate(
        0.0,
        -MODULE_Y / 2.0 + SERVICE_CLEARANCE_Y / 2.0,
        DECK_Z / 2.0 - 2.5,
    );
    let waste_drain = centered_cylinder("inline_media_qc_waste_sump_drain", 8.0 / 2.0, 34.0, 28)
        .rotate(90.0, 0.0, 0.0)
        .translate(MODULE_X / 2.0 - 76.0, -MODULE_Y / 2.0 + 30.0, 0.0);

    deck - wet_path_sump
        - calibration_sump
        - filter_drip_pocket
        - front_service_relief
        - waste_drain
        - deck_mount_slots()
        - deck_route_trenches()
        + deck_perimeter_rails()
        + skid_locator_bosses()
        + component_locator_tabs()
        + tube_bridge_standoffs()
}

fn deck_mount_slots() -> Part {
    let mut slots = Part::empty("inline_media_qc_deck_mount_slots");
    for (i, (x, y)) in deck_mount_points().iter().enumerate() {
        let hole = centered_cylinder(
            format!("inline_media_qc_deck_m6_clearance_{i}"),
            6.6 / 2.0,
            DECK_Z + 2.0,
            24,
        )
        .translate(*x, *y, 0.0);
        let slot = centered_cube(
            format!("inline_media_qc_deck_m6_slot_relief_{i}"),
            26.0,
            6.8,
            DECK_Z + 2.0,
        )
        .translate(*x, *y, 0.0);
        slots = slots + hole + slot;
    }
    slots
}

fn deck_route_trenches() -> Part {
    let filter_to_water = centered_cube(
        "inline_media_qc_filter_to_water_tube_trench",
        118.0,
        26.0,
        8.0,
    )
    .translate(-242.0, 92.0, DECK_Z / 2.0 - 3.5);
    let water_to_degasser = centered_cube(
        "inline_media_qc_water_to_degasser_tube_trench",
        214.0,
        26.0,
        8.0,
    )
    .translate(-42.0, 86.0, DECK_Z / 2.0 - 3.5);
    let degasser_to_qc = centered_cube(
        "inline_media_qc_degasser_to_sensor_tube_trench",
        132.0,
        26.0,
        8.0,
    )
    .translate(178.0, 90.0, DECK_Z / 2.0 - 3.5);
    let qc_to_pressure = centered_cube(
        "inline_media_qc_sensor_to_pressure_tube_trench",
        26.0,
        128.0,
        8.0,
    )
    .translate(248.0, -4.0, DECK_Z / 2.0 - 3.5);
    let pressure_to_handoff = centered_cube(
        "inline_media_qc_pressure_to_handoff_tube_trench",
        132.0,
        26.0,
        8.0,
    )
    .translate(278.0, -120.0, DECK_Z / 2.0 - 3.5);
    let bypass_to_waste = centered_cube(
        "inline_media_qc_bypass_to_waste_tube_trench",
        32.0,
        118.0,
        8.0,
    )
    .translate(BYPASS_POS.0 - 100.0, -98.0, DECK_Z / 2.0 - 3.5);

    filter_to_water
        + water_to_degasser
        + degasser_to_qc
        + qc_to_pressure
        + pressure_to_handoff
        + bypass_to_waste
}

fn deck_perimeter_rails() -> Part {
    let left = centered_cube(
        "inline_media_qc_left_deck_rail",
        18.0,
        MODULE_Y - 58.0,
        28.0,
    )
    .translate(-(MODULE_X / 2.0 - 30.0), 0.0, DECK_Z / 2.0 + 14.0);
    let right = centered_cube(
        "inline_media_qc_right_deck_rail",
        18.0,
        MODULE_Y - 58.0,
        28.0,
    )
    .translate(MODULE_X / 2.0 - 30.0, 0.0, DECK_Z / 2.0 + 14.0);
    let rear = centered_cube(
        "inline_media_qc_rear_deck_rail",
        MODULE_X - 72.0,
        18.0,
        30.0,
    )
    .translate(0.0, MODULE_Y / 2.0 - 30.0, DECK_Z / 2.0 + 15.0);
    let front_lip = centered_cube(
        "inline_media_qc_front_spill_lip",
        MODULE_X - 136.0,
        14.0,
        18.0,
    )
    .translate(0.0, -MODULE_Y / 2.0 + 24.0, DECK_Z / 2.0 + 9.0);

    left + right + rear + front_lip
}

fn skid_locator_bosses() -> Part {
    let mut bosses = Part::empty("inline_media_qc_skid_locator_bosses");
    for (i, (x, y)) in skid_locator_points().iter().enumerate() {
        let boss = centered_cylinder(
            format!("inline_media_qc_skid_locator_boss_{i}"),
            12.0,
            9.0,
            32,
        )
        .translate(*x, *y, DECK_Z / 2.0 + 4.5);
        let socket = centered_cylinder(
            format!("inline_media_qc_skid_locator_socket_{i}"),
            4.1 / 2.0,
            11.0,
            24,
        )
        .translate(*x, *y, DECK_Z / 2.0 + 4.5);
        bosses = bosses + (boss - socket);
    }
    bosses
}

fn component_locator_tabs() -> Part {
    let mut tabs = Part::empty("inline_media_qc_component_locator_tabs");
    for (i, (x, y, span)) in [
        (WATER_POS.0, WATER_POS.1, WATER_BLOCK_X),
        (DEGASSER_POS.0, DEGASSER_POS.1, DEGASSER_X),
        (QC_POS.0, QC_POS.1, QC_BLOCK_X),
        (PRESSURE_FLOW_POS.0, PRESSURE_FLOW_POS.1, PRESSURE_FLOW_X),
        (HANDOFF_POS.0, HANDOFF_POS.1, HANDOFF_X),
    ]
    .iter()
    .enumerate()
    {
        tabs = tabs
            + centered_cube(
                format!("inline_media_qc_component_key_left_{i}"),
                18.0,
                12.0,
                10.0,
            )
            .translate(*x - span / 2.0 + 20.0, *y, DECK_Z / 2.0 + 5.0)
            + centered_cube(
                format!("inline_media_qc_component_key_right_{i}"),
                18.0,
                12.0,
                10.0,
            )
            .translate(*x + span / 2.0 - 20.0, *y, DECK_Z / 2.0 + 5.0);
    }
    tabs
}

fn tube_bridge_standoffs() -> Part {
    let mut standoffs = Part::empty("inline_media_qc_tube_bridge_standoffs");
    for (i, (x, y)) in [
        (-342.0, 84.0),
        (-118.0, 90.0),
        (44.0, 86.0),
        (196.0, 88.0),
        (266.0, -8.0),
        (272.0, -124.0),
        (-100.0, -96.0),
    ]
    .iter()
    .enumerate()
    {
        let post = centered_cylinder(
            format!("inline_media_qc_tube_bridge_post_{i}"),
            8.0,
            20.0,
            28,
        )
        .translate(*x, *y, DECK_Z / 2.0 + 10.0);
        let tube_slot = centered_cylinder(
            format!("inline_media_qc_tube_bridge_slot_{i}"),
            FLUID_BORE_D / 2.0,
            18.0,
            24,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(*x, *y, DECK_Z / 2.0 + 16.0);
        standoffs = standoffs + (post - tube_slot);
    }
    standoffs
}

fn water_jacket_block() -> Part {
    let body = centered_cube(
        "inline_media_qc_water_jacket_body",
        WATER_BLOCK_X,
        WATER_BLOCK_Y,
        WATER_BLOCK_Z,
    );
    let media_bore = centered_cylinder(
        "inline_media_qc_water_jacket_media_bore",
        FLUID_BORE_D / 2.0,
        WATER_BLOCK_X + 18.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0);

    let mut jacket_cuts = Part::empty("inline_media_qc_water_jacket_channel_cuts");
    for i in 0..WATER_CHANNELS {
        let y = water_channel_y(i);
        let channel = centered_cylinder(
            format!("inline_media_qc_water_jacket_parallel_channel_{i}"),
            WATER_LINE_D / 2.0,
            WATER_BLOCK_X - 54.0,
            32,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, y, 8.0);
        jacket_cuts = jacket_cuts + channel;
    }

    let left_cross = centered_cylinder(
        "inline_media_qc_water_jacket_left_cross_channel",
        WATER_LINE_D / 2.0,
        (WATER_CHANNELS as f64 - 1.0) * WATER_CHANNEL_PITCH_Y + 10.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-(WATER_BLOCK_X / 2.0 - 34.0), 0.0, 8.0);
    let right_cross = centered_cylinder(
        "inline_media_qc_water_jacket_right_cross_channel",
        WATER_LINE_D / 2.0,
        (WATER_CHANNELS as f64 - 1.0) * WATER_CHANNEL_PITCH_Y + 10.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(WATER_BLOCK_X / 2.0 - 34.0, 0.0, 8.0);
    let water_in = centered_cylinder(
        "inline_media_qc_water_jacket_inlet_port",
        9.0 / 2.0,
        WATER_BLOCK_Y + 14.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-(WATER_BLOCK_X / 2.0 - 52.0), -WATER_BLOCK_Y / 2.0, 8.0);
    let water_out = centered_cylinder(
        "inline_media_qc_water_jacket_outlet_port",
        9.0 / 2.0,
        WATER_BLOCK_Y + 14.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(WATER_BLOCK_X / 2.0 - 52.0, -WATER_BLOCK_Y / 2.0, 8.0);

    let thermistor_pocket = centered_cylinder(
        "inline_media_qc_water_jacket_thermistor_pocket",
        3.2 / 2.0,
        44.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, WATER_BLOCK_Y / 2.0 - 8.0, -10.0);
    let probe_well = centered_cylinder(
        "inline_media_qc_water_jacket_reference_probe_well",
        5.0 / 2.0,
        WATER_BLOCK_Z + 4.0,
        24,
    )
    .translate(WATER_BLOCK_X / 2.0 - 76.0, 0.0, 0.0);

    let cover_lip = centered_cube(
        "inline_media_qc_water_jacket_cover_lip",
        WATER_BLOCK_X - 34.0,
        10.0,
        7.0,
    )
    .translate(0.0, WATER_BLOCK_Y / 2.0 + 5.0, WATER_BLOCK_Z / 2.0 - 7.0);
    let mount_bosses =
        block_mount_bosses("water_jacket", WATER_BLOCK_X, WATER_BLOCK_Y, WATER_BLOCK_Z);

    body + cover_lip + mount_bosses
        - media_bore
        - jacket_cuts
        - left_cross
        - right_cross
        - water_in
        - water_out
        - thermistor_pocket
        - probe_well
}

fn sterile_filter_holder() -> Part {
    let saddle = centered_cube(
        "inline_media_qc_filter_holder_saddle",
        FILTER_HOLDER_X,
        FILTER_HOLDER_Y,
        24.0,
    )
    .translate(0.0, 0.0, -(FILTER_HOLDER_Z / 2.0 - 12.0));
    let filter_body = centered_cylinder(
        "inline_media_qc_filter_capsule_envelope",
        FILTER_BODY_D / 2.0,
        FILTER_HOLDER_X - 52.0,
        48,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, 2.0);
    let body_clearance = centered_cylinder(
        "inline_media_qc_filter_capsule_clearance",
        FILTER_BODY_D / 2.0 + 1.4,
        FILTER_HOLDER_X - 26.0,
        48,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, 2.0);

    let mut clamps = Part::empty("inline_media_qc_filter_retention_clamps");
    for (i, x) in [-74.0, 0.0, 74.0].iter().enumerate() {
        let clamp = centered_cube(
            format!("inline_media_qc_filter_clamp_bridge_{i}"),
            22.0,
            FILTER_HOLDER_Y,
            18.0,
        )
        .translate(*x, 0.0, 22.0);
        let clamp_bore = centered_cylinder(
            format!("inline_media_qc_filter_clamp_bore_{i}"),
            FILTER_BODY_D / 2.0 + 2.0,
            FILTER_HOLDER_Y + 8.0,
            40,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, 0.0, 12.0);
        let latch = centered_cylinder(
            format!("inline_media_qc_filter_clamp_m4_latch_{i}"),
            4.3 / 2.0,
            FILTER_HOLDER_Y + 10.0,
            20,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, 0.0, 32.0);
        clamps = clamps + (clamp - clamp_bore - latch);
    }

    let inlet_tab =
        filter_bulkhead_tab("inlet").translate(-(FILTER_HOLDER_X / 2.0 - 18.0), 0.0, 2.0);
    let outlet_tab =
        filter_bulkhead_tab("outlet").translate(FILTER_HOLDER_X / 2.0 - 18.0, 0.0, 2.0);
    let directional_key = centered_cube("inline_media_qc_filter_directional_key", 34.0, 14.0, 12.0)
        .translate(
            -(FILTER_HOLDER_X / 2.0 - 56.0),
            -(FILTER_HOLDER_Y / 2.0 - 10.0),
            30.0,
        );

    saddle + filter_body + clamps + inlet_tab + outlet_tab + directional_key - body_clearance
}

fn filter_bulkhead_tab(name: &str) -> Part {
    let tab = centered_cube(
        format!("inline_media_qc_filter_{name}_bulkhead_tab"),
        34.0,
        52.0,
        34.0,
    );
    let tube = centered_cylinder(
        format!("inline_media_qc_filter_{name}_tube_clearance"),
        FLUID_BORE_D / 2.0,
        58.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0);
    let screw = centered_cylinder(
        format!("inline_media_qc_filter_{name}_m4_mount"),
        4.4 / 2.0,
        38.0,
        20,
    )
    .translate(0.0, 14.0, 0.0);
    tab - tube - screw
}

fn degassing_membrane_placeholder() -> Part {
    let body = centered_cube(
        "inline_media_qc_degassing_membrane_body",
        DEGASSER_X,
        DEGASSER_Y,
        DEGASSER_Z,
    );
    let media_channel = centered_cylinder(
        "inline_media_qc_degassing_media_channel",
        FLUID_BORE_D / 2.0,
        DEGASSER_X + 18.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0);
    let gas_lumen = centered_cylinder(
        "inline_media_qc_degassing_gas_lumen",
        6.0 / 2.0,
        DEGASSER_X - 34.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 24.0, 12.0);

    let mut membrane_windows = Part::empty("inline_media_qc_degassing_membrane_windows");
    for i in 0..DEGASSER_MEMBRANE_WINDOWS {
        membrane_windows = membrane_windows
            + centered_cube(
                format!("inline_media_qc_degassing_membrane_window_{i}"),
                58.0,
                12.0,
                DEGASSER_Z + 4.0,
            )
            .translate(degasser_window_x(i), -(DEGASSER_Y / 2.0 - 17.0), 0.0);
    }

    let vacuum_in = centered_cylinder(
        "inline_media_qc_degassing_vacuum_inlet",
        7.0 / 2.0,
        DEGASSER_Y + 16.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-(DEGASSER_X / 2.0 - 42.0), 0.0, 12.0);
    let vacuum_out = centered_cylinder(
        "inline_media_qc_degassing_vacuum_outlet",
        7.0 / 2.0,
        DEGASSER_Y + 16.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DEGASSER_X / 2.0 - 42.0, 0.0, 12.0);
    let membrane_land = centered_cube(
        "inline_media_qc_degassing_membrane_clamp_land",
        DEGASSER_X - 28.0,
        10.0,
        8.0,
    )
    .translate(0.0, -(DEGASSER_Y / 2.0 + 5.0), 0.0);
    let clamp_ears = side_latch_ears("inline_media_qc_degasser", DEGASSER_X, DEGASSER_Y, 0.0);

    body + membrane_land + clamp_ears
        - media_channel
        - gas_lumen
        - membrane_windows
        - vacuum_in
        - vacuum_out
}

fn sensor_qc_manifold() -> Part {
    let body = centered_cube(
        "inline_media_qc_sensor_manifold_body",
        QC_BLOCK_X,
        QC_BLOCK_Y,
        QC_BLOCK_Z,
    );
    let main_bore = centered_cylinder(
        "inline_media_qc_sensor_manifold_media_bore",
        FLUID_BORE_D / 2.0,
        QC_BLOCK_X + 18.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0);

    let mut sensor_pockets = Part::empty("inline_media_qc_chemical_sensor_pockets");
    for sensor in 0..CHEM_SENSOR_COUNT {
        let x = chemical_sensor_x(sensor);
        let pocket = centered_cylinder(
            format!("inline_media_qc_chemical_sensor_pocket_{sensor}"),
            chemical_sensor_radius(sensor),
            28.0,
            40,
        )
        .translate(x, -8.0, QC_BLOCK_Z / 2.0 - 8.0);
        let reader_window = centered_cube(
            format!("inline_media_qc_chemical_sensor_reader_window_{sensor}"),
            34.0,
            26.0,
            16.0,
        )
        .translate(x, -(QC_BLOCK_Y / 2.0 - 16.0), QC_BLOCK_Z / 2.0 - 10.0);
        let cable_slot = centered_cube(
            format!("inline_media_qc_chemical_sensor_cable_slot_{sensor}"),
            8.0,
            42.0,
            8.0,
        )
        .translate(x, QC_BLOCK_Y / 2.0 - 18.0, QC_BLOCK_Z / 2.0 - 12.0);
        sensor_pockets = sensor_pockets + pocket + reader_window + cable_slot;
    }

    let top_label_land = centered_cube(
        "inline_media_qc_sensor_manifold_top_service_land",
        QC_BLOCK_X - 34.0,
        6.0,
        6.0,
    )
    .translate(0.0, -(QC_BLOCK_Y / 2.0 + 3.0), QC_BLOCK_Z / 2.0 - 6.0);
    let optical_dark_cover_rail = centered_cube(
        "inline_media_qc_sensor_optical_dark_cover_rail",
        QC_BLOCK_X - 56.0,
        10.0,
        9.0,
    )
    .translate(0.0, QC_BLOCK_Y / 2.0 - 10.0, QC_BLOCK_Z / 2.0 + 2.0);
    let mount_bosses = block_mount_bosses("sensor_manifold", QC_BLOCK_X, QC_BLOCK_Y, QC_BLOCK_Z);

    body + top_label_land + optical_dark_cover_rail + mount_bosses - main_bore - sensor_pockets
}

fn pressure_flow_sensor_block() -> Part {
    let body = centered_cube(
        "inline_media_qc_pressure_flow_block_body",
        PRESSURE_FLOW_X,
        PRESSURE_FLOW_Y,
        PRESSURE_FLOW_Z,
    );
    let main_bore = centered_cylinder(
        "inline_media_qc_pressure_flow_media_bore",
        FLUID_BORE_D / 2.0,
        PRESSURE_FLOW_X + 18.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0);
    let flow_meter_pocket = centered_cube(
        "inline_media_qc_flow_meter_pocket",
        FLOW_WINDOW_X,
        48.0,
        18.0,
    )
    .translate(0.0, 0.0, PRESSURE_FLOW_Z / 2.0 - 10.0);
    let flow_view_window = centered_cube(
        "inline_media_qc_flow_meter_view_window",
        FLOW_WINDOW_X - 16.0,
        PRESSURE_FLOW_Y + 8.0,
        12.0,
    )
    .translate(0.0, 0.0, PRESSURE_FLOW_Z / 2.0 - 8.0);

    let mut pressure_cuts = Part::empty("inline_media_qc_pressure_sensor_cuts");
    let mut pressure_bosses = Part::empty("inline_media_qc_pressure_sensor_bosses");
    for sensor in 0..PRESSURE_SENSOR_COUNT {
        let x = pressure_sensor_x(sensor);
        let tap = centered_cylinder(
            format!("inline_media_qc_pressure_tap_bore_{sensor}"),
            2.0,
            PRESSURE_FLOW_Z + 8.0,
            20,
        )
        .translate(x, 0.0, 0.0);
        let pocket = centered_cube(
            format!("inline_media_qc_pressure_sensor_pocket_{sensor}"),
            42.0,
            32.0,
            16.0,
        )
        .translate(
            x,
            -(PRESSURE_FLOW_Y / 2.0 - 18.0),
            PRESSURE_FLOW_Z / 2.0 - 8.0,
        );
        let boss = centered_cylinder(
            format!("inline_media_qc_pressure_sensor_boss_{sensor}"),
            15.0,
            8.0,
            32,
        )
        .translate(x, 0.0, PRESSURE_FLOW_Z / 2.0 + 4.0);
        pressure_cuts = pressure_cuts + tap + pocket;
        pressure_bosses = pressure_bosses + boss;
    }

    let cable_gutter = centered_cube(
        "inline_media_qc_pressure_flow_cable_gutter",
        PRESSURE_FLOW_X - 34.0,
        9.0,
        9.0,
    )
    .translate(
        0.0,
        PRESSURE_FLOW_Y / 2.0 - 10.0,
        PRESSURE_FLOW_Z / 2.0 - 10.0,
    );
    let latch_ears = side_latch_ears(
        "inline_media_qc_pressure_flow",
        PRESSURE_FLOW_X,
        PRESSURE_FLOW_Y,
        0.0,
    );

    body + pressure_bosses + latch_ears
        - main_bore
        - flow_meter_pocket
        - flow_view_window
        - pressure_cuts
        - cable_gutter
}

fn sterile_sampling_loop() -> Part {
    let deck = centered_cube(
        "inline_media_qc_sampling_loop_cartridge_deck",
        SAMPLE_LOOP_X,
        SAMPLE_LOOP_Y,
        20.0,
    )
    .translate(0.0, 0.0, -(SAMPLE_LOOP_Z / 2.0 - 10.0));

    let upstream = centered_cylinder(
        "inline_media_qc_sampling_loop_upstream_leg",
        SAMPLE_LOOP_D / 2.0,
        SAMPLE_LOOP_X - 54.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 26.0, 4.0);
    let downstream = centered_cylinder(
        "inline_media_qc_sampling_loop_downstream_leg",
        SAMPLE_LOOP_D / 2.0,
        SAMPLE_LOOP_X - 54.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -26.0, 4.0);
    let loop_return = centered_cylinder(
        "inline_media_qc_sampling_loop_u_return",
        SAMPLE_LOOP_D / 2.0,
        52.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(SAMPLE_LOOP_X / 2.0 - 34.0, 0.0, 4.0);
    let sample_takeoff = centered_cylinder(
        "inline_media_qc_sampling_loop_takeoff",
        SAMPLE_LOOP_D / 2.0,
        76.0,
        24,
    )
    .translate(-24.0, 0.0, 18.0);
    let septum_cup = centered_cylinder("inline_media_qc_sampling_loop_septum_cup", 18.0, 16.0, 40)
        .translate(-24.0, 0.0, SAMPLE_LOOP_Z / 2.0 - 8.0);
    let vial_clearance = centered_cylinder(
        "inline_media_qc_sampling_loop_vial_clearance",
        13.0,
        SAMPLE_LOOP_Z + 10.0,
        40,
    )
    .translate(-24.0, 0.0, 10.0);

    let mut valves = Part::empty("inline_media_qc_sampling_loop_valve_features");
    for i in 0..SAMPLE_VALVE_COUNT {
        let x = sample_valve_x(i);
        let saddle = centered_cube(
            format!("inline_media_qc_sampling_pinch_valve_saddle_{i}"),
            30.0,
            26.0,
            16.0,
        )
        .translate(x, if i == 1 { 0.0 } else { 26.0 }, 18.0);
        let stem = centered_cylinder(
            format!("inline_media_qc_sampling_valve_stem_{i}"),
            4.0 / 2.0,
            34.0,
            20,
        )
        .translate(x, if i == 1 { 0.0 } else { 26.0 }, 14.0);
        valves = valves + (saddle - stem);
    }

    let sterile_connector = centered_cube(
        "inline_media_qc_sampling_loop_aseptic_connector_land",
        58.0,
        34.0,
        20.0,
    )
    .translate(-(SAMPLE_LOOP_X / 2.0 - 42.0), -28.0, 14.0);
    let connector_bore = centered_cylinder(
        "inline_media_qc_sampling_loop_connector_bore",
        FLUID_BORE_D / 2.0,
        64.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(-(SAMPLE_LOOP_X / 2.0 - 42.0), -28.0, 14.0);

    deck + upstream
        + downstream
        + loop_return
        + sample_takeoff
        + septum_cup
        + valves
        + (sterile_connector - connector_bore)
        - vial_clearance
}

fn calibration_port_bank() -> Part {
    let body = centered_cube(
        "inline_media_qc_calibration_port_bank_body",
        CAL_BANK_X,
        CAL_BANK_Y,
        CAL_BANK_Z,
    );
    let calibration_manifold = centered_cylinder(
        "inline_media_qc_calibration_common_manifold",
        FLUID_BORE_D / 2.0,
        CAL_BANK_X + 16.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -16.0, 0.0);

    let mut port_cuts = Part::empty("inline_media_qc_calibration_port_cuts");
    let mut caps = Part::empty("inline_media_qc_calibration_port_caps");
    for port in 0..CALIBRATION_PORTS {
        let x = calibration_port_x(port);
        let vertical = centered_cylinder(
            format!("inline_media_qc_calibration_vertical_bore_{port}"),
            6.2 / 2.0,
            CAL_BANK_Z + 8.0,
            28,
        )
        .translate(x, -16.0, 0.0);
        let cup = centered_cylinder(
            format!("inline_media_qc_calibration_luer_cup_{port}"),
            12.0 / 2.0,
            16.0,
            32,
        )
        .translate(x, -16.0, CAL_BANK_Z / 2.0 - 8.0);
        let check_pocket = centered_cube(
            format!("inline_media_qc_calibration_check_valve_pocket_{port}"),
            26.0,
            22.0,
            12.0,
        )
        .translate(x, 12.0, 6.0);
        let cap = centered_cylinder(
            format!("inline_media_qc_calibration_cap_standoff_{port}"),
            13.0,
            7.0,
            32,
        )
        .translate(x, -16.0, CAL_BANK_Z / 2.0 + 3.5);
        port_cuts = port_cuts + vertical + cup + check_pocket;
        caps = caps + cap;
    }

    let waste_flush = centered_cylinder(
        "inline_media_qc_calibration_waste_flush_bore",
        WASTE_BORE_D / 2.0,
        CAL_BANK_Y + 14.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(CAL_BANK_X / 2.0 - 28.0, 0.0, -8.0);
    let cover_lip = centered_cube(
        "inline_media_qc_calibration_cover_lip",
        CAL_BANK_X - 30.0,
        8.0,
        7.0,
    )
    .translate(0.0, -(CAL_BANK_Y / 2.0 + 4.0), CAL_BANK_Z / 2.0 - 7.0);

    body + caps + cover_lip - calibration_manifold - port_cuts - waste_flush
}

fn bypass_waste_manifold() -> Part {
    let body = centered_cube(
        "inline_media_qc_bypass_waste_manifold_body",
        BYPASS_X,
        BYPASS_Y,
        BYPASS_Z,
    );
    let main_bore = centered_cylinder(
        "inline_media_qc_bypass_waste_main_bore",
        FLUID_BORE_D / 2.0,
        BYPASS_X + 20.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0);
    let bypass_bore = centered_cylinder(
        "inline_media_qc_bypass_waste_bypass_bore",
        FLUID_BORE_D / 2.0,
        BYPASS_X - 62.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(20.0, -24.0, 0.0);
    let waste_branch = centered_cylinder(
        "inline_media_qc_bypass_waste_branch_bore",
        WASTE_BORE_D / 2.0,
        BYPASS_Y + 18.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-86.0, -12.0, -4.0);
    let sample_branch = centered_cylinder(
        "inline_media_qc_bypass_sample_branch_bore",
        FLUID_BORE_D / 2.0,
        BYPASS_Y + 18.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(46.0, -12.0, -4.0);

    let mut valve_cuts = Part::empty("inline_media_qc_bypass_valve_cuts");
    let mut valve_bosses = Part::empty("inline_media_qc_bypass_valve_bosses");
    for valve in 0..BYPASS_VALVES {
        let (x, y) = bypass_valve_xy(valve);
        let stem = centered_cylinder(
            format!("inline_media_qc_bypass_valve_stem_{valve}"),
            4.2 / 2.0,
            BYPASS_Z + 8.0,
            20,
        )
        .translate(x, y, 4.0);
        let pocket = centered_cube(
            format!("inline_media_qc_bypass_valve_actuator_pocket_{valve}"),
            26.0,
            24.0,
            12.0,
        )
        .translate(x, y, BYPASS_Z / 2.0 - 7.0);
        let boss = centered_cylinder(
            format!("inline_media_qc_bypass_valve_boss_{valve}"),
            11.0,
            7.0,
            28,
        )
        .translate(x, y, BYPASS_Z / 2.0 + 3.5);
        valve_cuts = valve_cuts + stem + pocket;
        valve_bosses = valve_bosses + boss;
    }

    let waste_cup = centered_cube("inline_media_qc_bypass_waste_cup_recess", 56.0, 32.0, 16.0)
        .translate(-118.0, -(BYPASS_Y / 2.0 - 16.0), -6.0);
    let front_drain_lip = centered_cube(
        "inline_media_qc_bypass_waste_front_drain_lip",
        BYPASS_X - 44.0,
        8.0,
        7.0,
    )
    .translate(0.0, -(BYPASS_Y / 2.0 + 4.0), -(BYPASS_Z / 2.0 - 7.0));

    body + valve_bosses + front_drain_lip
        - main_bore
        - bypass_bore
        - waste_branch
        - sample_branch
        - valve_cuts
        - waste_cup
}

fn cassette_handoff_datum() -> Part {
    let plate = centered_cube(
        "inline_media_qc_cassette_handoff_datum_plate",
        HANDOFF_X,
        HANDOFF_Y,
        HANDOFF_Z,
    );
    let cassette_outline = centered_cube(
        "inline_media_qc_cassette_handoff_cassette_outline_relief",
        REVC_CHIP_LENGTH + 16.0,
        REVC_CHIP_WIDTH + 16.0,
        7.0,
    )
    .translate(-16.0, 0.0, HANDOFF_Z / 2.0 - 3.0);

    let hard_stop_x = centered_cube(
        "inline_media_qc_cassette_handoff_x_hard_stop",
        14.0,
        HANDOFF_Y - 32.0,
        28.0,
    )
    .translate(-(HANDOFF_X / 2.0 - 18.0), 0.0, HANDOFF_Z / 2.0 + 4.0);
    let hard_stop_y = centered_cube(
        "inline_media_qc_cassette_handoff_y_hard_stop",
        HANDOFF_X - 46.0,
        12.0,
        28.0,
    )
    .translate(10.0, HANDOFF_Y / 2.0 - 20.0, HANDOFF_Z / 2.0 + 4.0);

    let mut datum_pins = Part::empty("inline_media_qc_cassette_handoff_datum_pins");
    for (i, (x, y)) in [(-62.0, -48.0), (-62.0, 48.0), (58.0, -48.0)]
        .iter()
        .enumerate()
    {
        let pin = centered_cylinder(
            format!("inline_media_qc_cassette_handoff_datum_pin_{i}"),
            4.0,
            18.0,
            28,
        )
        .translate(*x, *y, HANDOFF_Z / 2.0 + 9.0);
        let pin_hole = centered_cylinder(
            format!("inline_media_qc_cassette_handoff_datum_pin_fastener_{i}"),
            2.2 / 2.0,
            20.0,
            18,
        )
        .translate(*x, *y, HANDOFF_Z / 2.0 + 9.0);
        datum_pins = datum_pins + (pin - pin_hole);
    }

    let mut row_ports = Part::empty("inline_media_qc_cassette_handoff_row_ports");
    let mut row_labels = Part::empty("inline_media_qc_cassette_handoff_row_port_lands");
    for row in 0..CASSETTE_ROWS {
        let y = row_handoff_y(row);
        row_ports = row_ports
            + centered_cylinder(
                format!("inline_media_qc_cassette_row_{row}_feed_port"),
                ROW_PORT_D / 2.0,
                HANDOFF_X + 12.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(22.0, y, 0.0);
        row_labels = row_labels
            + centered_cube(
                format!("inline_media_qc_cassette_row_{row}_port_land"),
                46.0,
                12.0,
                7.0,
            )
            .translate(72.0, y, HANDOFF_Z / 2.0 + 3.5);
    }

    let waste_return_port = centered_cylinder(
        "inline_media_qc_cassette_handoff_waste_return_port",
        WASTE_BORE_D / 2.0,
        HANDOFF_X + 12.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(22.0, -(HANDOFF_Y / 2.0 - 18.0), -5.0);
    let datum_arrow = centered_cube(
        "inline_media_qc_cassette_handoff_asymmetric_key_arrow",
        34.0,
        10.0,
        8.0,
    )
    .translate(
        HANDOFF_X / 2.0 - 38.0,
        HANDOFF_Y / 2.0 - 38.0,
        HANDOFF_Z / 2.0 + 4.0,
    );

    plate + hard_stop_x + hard_stop_y + datum_pins + row_labels + datum_arrow
        - cassette_outline
        - row_ports
        - waste_return_port
        - handoff_mount_holes()
}

fn routed_tube_placeholders() -> Part {
    let filter_to_water = tube_run_x(
        "inline_media_qc_route_filter_to_water",
        FILTER_POS.0 + FILTER_HOLDER_X / 2.0,
        WATER_POS.0 - WATER_BLOCK_X / 2.0,
        116.0,
        DECK_Z + 32.0,
        FLUID_BORE_D,
    );
    let water_to_degasser = tube_run_x(
        "inline_media_qc_route_water_to_degasser",
        WATER_POS.0 + WATER_BLOCK_X / 2.0,
        DEGASSER_POS.0 - DEGASSER_X / 2.0,
        102.0,
        DECK_Z + 36.0,
        FLUID_BORE_D,
    );
    let degasser_to_qc = tube_run_x(
        "inline_media_qc_route_degasser_to_qc",
        DEGASSER_POS.0 + DEGASSER_X / 2.0,
        QC_POS.0 - QC_BLOCK_X / 2.0,
        96.0,
        DECK_Z + 36.0,
        FLUID_BORE_D,
    );
    let qc_to_pressure = tube_run_y(
        "inline_media_qc_route_qc_to_pressure",
        246.0,
        QC_POS.1 - QC_BLOCK_Y / 2.0,
        PRESSURE_FLOW_POS.1 + PRESSURE_FLOW_Y / 2.0,
        DECK_Z + 36.0,
        FLUID_BORE_D,
    );
    let pressure_to_handoff = tube_run_x(
        "inline_media_qc_route_pressure_to_handoff",
        PRESSURE_FLOW_POS.0 + PRESSURE_FLOW_X / 2.0,
        HANDOFF_POS.0 - HANDOFF_X / 2.0,
        -118.0,
        DECK_Z + 34.0,
        FLUID_BORE_D,
    );
    let bypass_to_sample = tube_run_y(
        "inline_media_qc_route_bypass_to_sample",
        46.0,
        BYPASS_POS.1 - BYPASS_Y / 2.0,
        SAMPLE_POS.1 + SAMPLE_LOOP_Y / 2.0,
        DECK_Z + 32.0,
        SAMPLE_LOOP_D,
    );
    let calibration_to_bypass = tube_run_x(
        "inline_media_qc_route_calibration_to_bypass",
        CAL_POS.0 + CAL_BANK_X / 2.0,
        BYPASS_POS.0 - BYPASS_X / 2.0,
        -118.0,
        DECK_Z + 30.0,
        FLUID_BORE_D,
    );
    let waste_to_front = tube_run_y(
        "inline_media_qc_route_waste_to_front",
        -118.0,
        BYPASS_POS.1 - BYPASS_Y / 2.0,
        -MODULE_Y / 2.0 + 38.0,
        DECK_Z + 28.0,
        WASTE_BORE_D,
    );

    filter_to_water
        + water_to_degasser
        + degasser_to_qc
        + qc_to_pressure
        + pressure_to_handoff
        + bypass_to_sample
        + calibration_to_bypass
        + waste_to_front
}

fn block_mount_bosses(name: &str, width: f64, depth: f64, height: f64) -> Part {
    let mut bosses = Part::empty(format!("inline_media_qc_{name}_mount_bosses"));
    for (i, (x, y)) in [
        (-(width / 2.0 - 22.0), -(depth / 2.0 - 18.0)),
        (width / 2.0 - 22.0, -(depth / 2.0 - 18.0)),
        (-(width / 2.0 - 22.0), depth / 2.0 - 18.0),
        (width / 2.0 - 22.0, depth / 2.0 - 18.0),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("inline_media_qc_{name}_mount_boss_{i}"),
            8.0,
            7.0,
            28,
        )
        .translate(*x, *y, height / 2.0 + 3.5);
        let hole = centered_cylinder(
            format!("inline_media_qc_{name}_mount_hole_{i}"),
            3.4 / 2.0,
            9.0,
            20,
        )
        .translate(*x, *y, height / 2.0 + 3.5);
        bosses = bosses + (boss - hole);
    }
    bosses
}

fn side_latch_ears(name: &str, width: f64, depth: f64, z: f64) -> Part {
    let mut ears = Part::empty(format!("{name}_side_latch_ears"));
    for (i, x) in [-(width / 2.0 - 22.0), width / 2.0 - 22.0]
        .iter()
        .enumerate()
    {
        let ear = centered_cube(format!("{name}_latch_ear_{i}"), 32.0, 24.0, 12.0).translate(
            *x,
            -(depth / 2.0 + 12.0),
            z,
        );
        let screw = centered_cylinder(format!("{name}_latch_screw_{i}"), 3.4 / 2.0, 14.0, 20)
            .translate(*x, -(depth / 2.0 + 12.0), z);
        ears = ears + (ear - screw);
    }
    ears
}

fn handoff_mount_holes() -> Part {
    let mut holes = Part::empty("inline_media_qc_cassette_handoff_mount_holes");
    for (i, (x, y)) in [
        (-(HANDOFF_X / 2.0 - 20.0), -(HANDOFF_Y / 2.0 - 18.0)),
        (HANDOFF_X / 2.0 - 20.0, -(HANDOFF_Y / 2.0 - 18.0)),
        (-(HANDOFF_X / 2.0 - 20.0), HANDOFF_Y / 2.0 - 18.0),
        (HANDOFF_X / 2.0 - 20.0, HANDOFF_Y / 2.0 - 18.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("inline_media_qc_cassette_handoff_m5_mount_{i}"),
                5.3 / 2.0,
                HANDOFF_Z + 4.0,
                24,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn tube_run_x(name: &str, x_a: f64, x_b: f64, y: f64, z: f64, diameter: f64) -> Part {
    let len = (x_b - x_a).abs();
    centered_cylinder(format!("{name}_tube_placeholder"), diameter / 2.0, len, 24)
        .rotate(0.0, 90.0, 0.0)
        .translate((x_a + x_b) / 2.0, y, z)
}

fn tube_run_y(name: &str, x: f64, y_a: f64, y_b: f64, z: f64, diameter: f64) -> Part {
    let len = (y_b - y_a).abs();
    centered_cylinder(format!("{name}_tube_placeholder"), diameter / 2.0, len, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(x, (y_a + y_b) / 2.0, z)
}

fn water_channel_y(index: usize) -> f64 {
    -((WATER_CHANNELS as f64 - 1.0) * WATER_CHANNEL_PITCH_Y) / 2.0
        + index as f64 * WATER_CHANNEL_PITCH_Y
}

fn degasser_window_x(index: usize) -> f64 {
    let pitch = 72.0;
    -((DEGASSER_MEMBRANE_WINDOWS as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn chemical_sensor_x(sensor: usize) -> f64 {
    -((CHEM_SENSOR_COUNT as f64 - 1.0) * CHEM_SENSOR_PITCH_X) / 2.0
        + sensor as f64 * CHEM_SENSOR_PITCH_X
}

fn chemical_sensor_radius(sensor: usize) -> f64 {
    match sensor {
        0 => 10.0, // pH puck or micro-probe pocket.
        1 => 9.0,  // optical DO spot/read head pocket.
        2 => 8.0,  // conductivity electrode cartridge pocket.
        _ => unreachable!("chemical sensor index is constrained by CHEM_SENSOR_COUNT"),
    }
}

fn pressure_sensor_x(sensor: usize) -> f64 {
    match sensor {
        0 => -(PRESSURE_FLOW_X / 2.0 - 48.0),
        1 => PRESSURE_FLOW_X / 2.0 - 48.0,
        _ => unreachable!("pressure sensor index is constrained by PRESSURE_SENSOR_COUNT"),
    }
}

fn sample_valve_x(index: usize) -> f64 {
    let pitch = 54.0;
    -((SAMPLE_VALVE_COUNT as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn calibration_port_x(port: usize) -> f64 {
    -((CALIBRATION_PORTS as f64 - 1.0) * CAL_PORT_PITCH_X) / 2.0 + port as f64 * CAL_PORT_PITCH_X
}

fn bypass_valve_xy(valve: usize) -> (f64, f64) {
    match valve {
        0 => (-112.0, 0.0),
        1 => (-36.0, -24.0),
        2 => (42.0, 0.0),
        3 => (112.0, -24.0),
        _ => unreachable!("bypass valve index is constrained by BYPASS_VALVES"),
    }
}

fn deck_mount_points() -> [(f64, f64); 8] {
    [
        (-(MODULE_X / 2.0 - 44.0), -(MODULE_Y / 2.0 - 42.0)),
        (MODULE_X / 2.0 - 44.0, -(MODULE_Y / 2.0 - 42.0)),
        (-(MODULE_X / 2.0 - 44.0), MODULE_Y / 2.0 - 42.0),
        (MODULE_X / 2.0 - 44.0, MODULE_Y / 2.0 - 42.0),
        (0.0, -(MODULE_Y / 2.0 - 42.0)),
        (0.0, MODULE_Y / 2.0 - 42.0),
        (-(MODULE_X / 2.0 - 44.0), 0.0),
        (MODULE_X / 2.0 - 44.0, 0.0),
    ]
}

fn skid_locator_points() -> [(f64, f64); 4] {
    [
        (-(MODULE_X / 2.0 - 94.0), -(MODULE_Y / 2.0 - 92.0)),
        (MODULE_X / 2.0 - 94.0, -(MODULE_Y / 2.0 - 92.0)),
        (-(MODULE_X / 2.0 - 94.0), MODULE_Y / 2.0 - 92.0),
        (MODULE_X / 2.0 - 94.0, MODULE_Y / 2.0 - 92.0),
    ]
}

fn cassette_row_y(row: usize) -> f64 {
    -((CASSETTE_ROWS as f64 - 1.0) * (REVC_CHIP_WIDTH + CASSETTE_GUTTER)) / 2.0
        + row as f64 * (REVC_CHIP_WIDTH + CASSETTE_GUTTER)
}

fn row_handoff_y(row: usize) -> f64 {
    cassette_row_y(row) * HANDOFF_PORT_SPAN_Y / CASSETTE_ARRAY_Y
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 11);
        for path in OUTPUTS {
            assert!(path.starts_with("output/inline_media_conditioning_qc_module_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn required_inline_qc_features_have_sane_counts() {
        assert_eq!(CHEM_SENSOR_COUNT, 3);
        assert_eq!(PRESSURE_SENSOR_COUNT, 2);
        assert_eq!(CALIBRATION_PORTS, 4);
        assert_eq!(BYPASS_VALVES, 4);
        assert_eq!(SAMPLE_VALVE_COUNT, 3);
        assert_eq!(DEGASSER_MEMBRANE_WINDOWS, 3);
        assert_eq!(CASSETTE_ROWS, 5);
        assert_eq!(CASSETTE_COLS, 4);
    }

    #[test]
    fn major_components_fit_on_service_deck() {
        assert!(FILTER_POS.0 - FILTER_HOLDER_X / 2.0 > -MODULE_X / 2.0);
        assert!(HANDOFF_POS.0 + HANDOFF_X / 2.0 < MODULE_X / 2.0);
        assert!(DEGASSER_POS.1 + DEGASSER_Y / 2.0 < MODULE_Y / 2.0);
        assert!(SAMPLE_POS.1 - SAMPLE_LOOP_Y / 2.0 > -MODULE_Y / 2.0 + 20.0);
        assert!(CAL_POS.1 - CAL_BANK_Y / 2.0 > -MODULE_Y / 2.0 + 20.0);
        assert!(SERVICE_CLEARANCE_Y >= 80.0);
    }

    #[test]
    fn sensor_and_calibration_arrays_fit_their_blocks() {
        assert_eq!(
            chemical_sensor_x(0),
            -chemical_sensor_x(CHEM_SENSOR_COUNT - 1)
        );
        assert!(chemical_sensor_x(0).abs() < QC_BLOCK_X / 2.0 - 40.0);
        assert!(calibration_port_x(0).abs() < CAL_BANK_X / 2.0 - 28.0);
        assert!(calibration_port_x(CALIBRATION_PORTS - 1).abs() < CAL_BANK_X / 2.0 - 28.0);
        assert!(pressure_sensor_x(0) < 0.0);
        assert!(pressure_sensor_x(1) > 0.0);
    }

    #[test]
    fn cassette_handoff_preserves_row_order_and_datum_scale() {
        assert_eq!(
            CASSETTE_ARRAY_X,
            CASSETTE_COLS as f64 * REVC_CHIP_LENGTH + 15.0
        );
        assert_eq!(
            CASSETTE_ARRAY_Y,
            CASSETTE_ROWS as f64 * REVC_CHIP_WIDTH + 20.0
        );
        assert!(row_handoff_y(0) < row_handoff_y(CASSETTE_ROWS - 1));
        assert!(row_handoff_y(0) > -HANDOFF_Y / 2.0 + 18.0);
        assert!(row_handoff_y(CASSETTE_ROWS - 1) < HANDOFF_Y / 2.0 - 18.0);
        assert!(ROW_PORT_D > TUBE_OD);
    }

    #[test]
    fn water_jacket_and_flow_path_clear_tubing() {
        assert!(FLUID_BORE_D > TUBE_OD);
        assert!(WATER_LINE_D > FLUID_BORE_D);
        assert!(water_channel_y(0) < water_channel_y(WATER_CHANNELS - 1));
        assert!(water_channel_y(0).abs() < WATER_BLOCK_Y / 2.0 - 18.0);
        assert!(FILTER_BODY_D > 5.0 * FLUID_BORE_D);
        assert!(WASTE_BORE_D > FLUID_BORE_D);
    }
}
