use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed cassette perfusion startup shear-ramp validation station.
//
// Intent:
// - Package a sealed tissue-on-chip cassette fixture that verifies a repeatable
//   low-shear startup ramp from priming flow to operating flow.
// - Keep cassette datums, sterile in/out bulkheads, interchangeable restrictor
//   plates, bubble observation, pressure and flow instrumentation, waste split,
//   leak capture, sampling, routing relief, traceability, and witness coupons
//   physically visible on one bench/workcell fixture.
// - Model mechanical validation hardware only. Biological acceptance criteria,
//   assay release decisions, and wetted-material qualification remain separate.

const OUTPUTS: [&str; 12] = [
    "output/closed_cassette_perfusion_startup_shear_ramp_validation_station_base_leak_moat.stl",
    "output/closed_cassette_perfusion_startup_shear_ramp_validation_station_cassette_clamp_datum_nest.stl",
    "output/closed_cassette_perfusion_startup_shear_ramp_validation_station_sterile_bulkhead_panel.stl",
    "output/closed_cassette_perfusion_startup_shear_ramp_validation_station_interchangeable_restrictor_plate_bank.stl",
    "output/closed_cassette_perfusion_startup_shear_ramp_validation_station_bubble_observation_window.stl",
    "output/closed_cassette_perfusion_startup_shear_ramp_validation_station_pressure_transducer_bosses.stl",
    "output/closed_cassette_perfusion_startup_shear_ramp_validation_station_flow_sensor_saddle.stl",
    "output/closed_cassette_perfusion_startup_shear_ramp_validation_station_waste_recirculation_splitter.stl",
    "output/closed_cassette_perfusion_startup_shear_ramp_validation_station_sample_access_port.stl",
    "output/closed_cassette_perfusion_startup_shear_ramp_validation_station_cable_gas_strain_relief.stl",
    "output/closed_cassette_perfusion_startup_shear_ramp_validation_station_labels_datums_qc_coupons.stl",
    "output/closed_cassette_perfusion_startup_shear_ramp_validation_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 12] = [
    "cassette_clamp_datum",
    "sterile_fluidic_in_bulkhead",
    "sterile_fluidic_out_bulkhead",
    "interchangeable_restrictor_plate",
    "bubble_observation_window",
    "pressure_transducer_bosses",
    "flow_sensor_saddle",
    "waste_recirculation_split",
    "leak_moat",
    "sample_access_port",
    "cable_gas_routing_strain_relief",
    "labels_datums_qc_witness_coupons",
];

const DECK_X: f64 = 980.0;
const DECK_Y: f64 = 660.0;
const DECK_Z: f64 = 18.0;
const RIM_W: f64 = 16.0;
const RIM_Z: f64 = 34.0;
const MOUNT_HOLE_D: f64 = 6.6;
const LEAK_MOAT_W: f64 = 18.0;

const CASSETTE_X: f64 = REVC_CHIP_LENGTH * 2.0 + 118.0;
const CASSETTE_Y: f64 = REVC_CHIP_WIDTH * 2.0 + 92.0;
const CASSETTE_KEEP_OUT_Z: f64 = 54.0;
const NEST_X: f64 = CASSETTE_X + 94.0;
const NEST_Y: f64 = CASSETTE_Y + 86.0;
const NEST_Z: f64 = 34.0;
const NEST_POS: (f64, f64) = (-185.0, 42.0);
const DATUM_PIN_COUNT: usize = 4;
const CLAMP_COUNT: usize = 6;

const BULKHEAD_X: f64 = 210.0;
const BULKHEAD_Y: f64 = 176.0;
const BULKHEAD_Z: f64 = 58.0;
const BULKHEAD_POS: (f64, f64) = (-410.0, -216.0);
const BULKHEAD_PORTS: usize = 4;
const BULKHEAD_PORT_D: f64 = 8.2;

const RESTRICTOR_X: f64 = 340.0;
const RESTRICTOR_Y: f64 = 136.0;
const RESTRICTOR_Z: f64 = 28.0;
const RESTRICTOR_POS: (f64, f64) = (196.0, 196.0);
const RESTRICTOR_PLATES: usize = 5;
const RAMP_STEP_COUNT: usize = 5;

const WINDOW_X: f64 = 330.0;
const WINDOW_Y: f64 = 78.0;
const WINDOW_Z: f64 = 24.0;
const WINDOW_POS: (f64, f64) = (196.0, 72.0);
const BUBBLE_TICK_COUNT: usize = 11;

const PRESSURE_BAR_X: f64 = 360.0;
const PRESSURE_BAR_Y: f64 = 92.0;
const PRESSURE_BAR_Z: f64 = 42.0;
const PRESSURE_POS: (f64, f64) = (196.0, -52.0);
const PRESSURE_TRANSDUCERS: usize = 4;
const PRESSURE_BOSS_D: f64 = 28.0;
const PRESSURE_PORT_D: f64 = 4.0;

const FLOW_X: f64 = 298.0;
const FLOW_Y: f64 = 112.0;
const FLOW_Z: f64 = 48.0;
const FLOW_POS: (f64, f64) = (206.0, -186.0);
const FLOW_SENSOR_CHANNELS: usize = 2;

const SPLITTER_X: f64 = 250.0;
const SPLITTER_Y: f64 = 124.0;
const SPLITTER_Z: f64 = 38.0;
const SPLITTER_POS: (f64, f64) = (-88.0, -234.0);
const SPLIT_BRANCHES: usize = 2;

const SAMPLE_X: f64 = 150.0;
const SAMPLE_Y: f64 = 118.0;
const SAMPLE_Z: f64 = 44.0;
const SAMPLE_POS: (f64, f64) = (-302.0, -44.0);
const SAMPLE_PORTS: usize = 3;

const RELIEF_X: f64 = 300.0;
const RELIEF_Y: f64 = 92.0;
const RELIEF_Z: f64 = 36.0;
const RELIEF_POS: (f64, f64) = (-88.0, 274.0);
const STRAIN_RELIEF_LANES: usize = 6;

const LABEL_X: f64 = 318.0;
const LABEL_Y: f64 = 132.0;
const LABEL_Z: f64 = 10.0;
const LABEL_POS: (f64, f64) = (330.0, -294.0);
const QC_COUPONS: usize = 8;
const ENGRAVED_DATUMS: usize = 6;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_leak_moat();
    export(OUTPUTS[0], &base);

    let nest = cassette_clamp_datum_nest();
    export(OUTPUTS[1], &nest);

    let bulkheads = sterile_bulkhead_panel();
    export(OUTPUTS[2], &bulkheads);

    let restrictors = interchangeable_restrictor_plate_bank();
    export(OUTPUTS[3], &restrictors);

    let window = bubble_observation_window();
    export(OUTPUTS[4], &window);

    let pressure = pressure_transducer_bosses();
    export(OUTPUTS[5], &pressure);

    let flow = flow_sensor_saddle();
    export(OUTPUTS[6], &flow);

    let splitter = waste_recirculation_splitter();
    export(OUTPUTS[7], &splitter);

    let sample = sample_access_port();
    export(OUTPUTS[8], &sample);

    let relief = cable_gas_strain_relief();
    export(OUTPUTS[9], &relief);

    let labels = labels_datums_qc_coupons();
    export(OUTPUTS[10], &labels);

    let assembly = base
        + nest.translate(NEST_POS.0, NEST_POS.1, deck_mount_z(NEST_Z))
        + bulkheads.translate(BULKHEAD_POS.0, BULKHEAD_POS.1, deck_mount_z(BULKHEAD_Z))
        + restrictors.translate(
            RESTRICTOR_POS.0,
            RESTRICTOR_POS.1,
            deck_mount_z(RESTRICTOR_Z),
        )
        + window.translate(WINDOW_POS.0, WINDOW_POS.1, deck_mount_z(WINDOW_Z))
        + pressure.translate(PRESSURE_POS.0, PRESSURE_POS.1, deck_mount_z(PRESSURE_BAR_Z))
        + flow.translate(FLOW_POS.0, FLOW_POS.1, deck_mount_z(FLOW_Z))
        + splitter.translate(SPLITTER_POS.0, SPLITTER_POS.1, deck_mount_z(SPLITTER_Z))
        + sample.translate(SAMPLE_POS.0, SAMPLE_POS.1, deck_mount_z(SAMPLE_Z))
        + relief.translate(RELIEF_POS.0, RELIEF_POS.1, deck_mount_z(RELIEF_Z))
        + labels.translate(LABEL_POS.0, LABEL_POS.1, DECK_Z / 2.0 + LABEL_Z / 2.0);

    export(OUTPUTS[11], &assembly);

    println!(
        "Closed cassette perfusion startup shear-ramp station: {:.0}mm x {:.0}mm deck, {} cassette datum pins, {} clamps, {} sterile bulkhead ports, {} interchangeable restrictor plates, {} ramp witness steps, {} pressure bosses, {} flow channels, {} split branches, {} sample ports, {} strain-relief lanes, {} QC coupons, and {} required feature groups.",
        DECK_X,
        DECK_Y,
        DATUM_PIN_COUNT,
        CLAMP_COUNT,
        BULKHEAD_PORTS,
        RESTRICTOR_PLATES,
        RAMP_STEP_COUNT,
        PRESSURE_TRANSDUCERS,
        FLOW_SENSOR_CHANNELS,
        SPLIT_BRANCHES,
        SAMPLE_PORTS,
        STRAIN_RELIEF_LANES,
        QC_COUPONS,
        REQUIRED_FEATURES.len()
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn deck_mount_z(component_z: f64) -> f64 {
    DECK_Z / 2.0 + component_z / 2.0
}

fn base_leak_moat() -> Part {
    let deck = centered_cube(
        "startup_shear_ramp_validation_base_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    );

    let nest_recess = centered_cube(
        "startup_shear_ramp_validation_cassette_nest_recess",
        NEST_X + 30.0,
        NEST_Y + 28.0,
        7.0,
    )
    .translate(NEST_POS.0, NEST_POS.1, DECK_Z / 2.0 - 2.5);

    let front_moat = centered_cube(
        "startup_shear_ramp_validation_front_leak_moat",
        DECK_X - 96.0,
        LEAK_MOAT_W,
        8.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 68.0, DECK_Z / 2.0 - 2.0);
    let rear_moat = centered_cube(
        "startup_shear_ramp_validation_rear_leak_moat",
        DECK_X - 96.0,
        LEAK_MOAT_W,
        8.0,
    )
    .translate(0.0, DECK_Y / 2.0 - 68.0, DECK_Z / 2.0 - 2.0);
    let left_moat = centered_cube(
        "startup_shear_ramp_validation_left_leak_moat",
        LEAK_MOAT_W,
        DECK_Y - 156.0,
        8.0,
    )
    .translate(-DECK_X / 2.0 + 58.0, 0.0, DECK_Z / 2.0 - 2.0);
    let right_moat = centered_cube(
        "startup_shear_ramp_validation_right_leak_moat",
        LEAK_MOAT_W,
        DECK_Y - 156.0,
        8.0,
    )
    .translate(DECK_X / 2.0 - 58.0, 0.0, DECK_Z / 2.0 - 2.0);
    let drain = centered_cylinder(
        "startup_shear_ramp_validation_leak_moat_drain",
        5.0,
        54.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 86.0, -DECK_Y / 2.0 + 68.0, 0.0);

    deck - nest_recess - front_moat - rear_moat - left_moat - right_moat - drain
        + deck_rims()
        + deck_mount_holes()
        + moat_witness_ribs()
}

fn deck_rims() -> Part {
    let rear = centered_cube(
        "startup_shear_ramp_validation_rear_service_rim",
        DECK_X - 88.0,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - 34.0, DECK_Z / 2.0 + RIM_Z / 2.0);
    let left = centered_cube(
        "startup_shear_ramp_validation_left_spill_rim",
        RIM_W,
        DECK_Y - 120.0,
        RIM_Z,
    )
    .translate(-DECK_X / 2.0 + 34.0, 0.0, DECK_Z / 2.0 + RIM_Z / 2.0);
    let right = centered_cube(
        "startup_shear_ramp_validation_right_spill_rim",
        RIM_W,
        DECK_Y - 120.0,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - 34.0, 0.0, DECK_Z / 2.0 + RIM_Z / 2.0);
    let front = centered_cube(
        "startup_shear_ramp_validation_front_low_robot_lip",
        DECK_X - 190.0,
        10.0,
        14.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 32.0, DECK_Z / 2.0 + 7.0);

    rear + left + right + front
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("startup_shear_ramp_validation_deck_mount_holes");
    for (i, (x, y)) in mount_points().iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("startup_shear_ramp_validation_m6_mount_hole_{i}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 4.0,
                28,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn moat_witness_ribs() -> Part {
    let mut ribs = Part::empty("startup_shear_ramp_validation_moat_witness_ribs");
    for i in 0..10 {
        ribs = ribs
            + centered_cube(
                format!("startup_shear_ramp_validation_moat_witness_rib_{i}"),
                6.0,
                DECK_Y - 184.0,
                4.0,
            )
            .translate(centered_index(i, 10, 72.0), 0.0, DECK_Z / 2.0 + 2.0);
    }
    ribs
}

fn cassette_clamp_datum_nest() -> Part {
    let tray = centered_cube(
        "startup_shear_ramp_validation_cassette_clamp_datum_tray",
        NEST_X,
        NEST_Y,
        NEST_Z,
    );
    let cassette_relief = centered_cube(
        "startup_shear_ramp_validation_cassette_body_keepout_relief",
        CASSETTE_X + 2.0,
        CASSETTE_Y + 2.0,
        NEST_Z + 4.0,
    )
    .translate(0.0, 0.0, 7.0);

    tray - cassette_relief + datum_rails() + datum_pin_bosses() + cassette_clamps()
}

fn datum_rails() -> Part {
    let rear = centered_cube(
        "startup_shear_ramp_validation_cassette_rear_datum_rail",
        CASSETTE_X + 78.0,
        18.0,
        NEST_Z + 30.0,
    )
    .translate(0.0, CASSETTE_Y / 2.0 + 28.0, 15.0);
    let left = centered_cube(
        "startup_shear_ramp_validation_cassette_left_datum_rail",
        18.0,
        CASSETTE_Y + 70.0,
        NEST_Z + 30.0,
    )
    .translate(-(CASSETTE_X / 2.0 + 28.0), 0.0, 15.0);
    let right_spring = centered_cube(
        "startup_shear_ramp_validation_cassette_right_spring_preload_rail",
        14.0,
        CASSETTE_Y + 54.0,
        NEST_Z + 16.0,
    )
    .translate(CASSETTE_X / 2.0 + 28.0, 0.0, 8.0);
    let front = centered_cube(
        "startup_shear_ramp_validation_cassette_front_low_loading_lip",
        CASSETTE_X + 66.0,
        12.0,
        12.0,
    )
    .translate(0.0, -(CASSETTE_Y / 2.0 + 27.0), -5.0);

    rear + left + right_spring + front
}

fn datum_pin_bosses() -> Part {
    let mut bosses = Part::empty("startup_shear_ramp_validation_datum_pin_bosses");
    for (i, (x, y)) in datum_pin_points().iter().enumerate() {
        let boss = centered_cylinder(
            format!("startup_shear_ramp_validation_datum_pin_boss_{i}"),
            12.0,
            12.0,
            32,
        )
        .translate(*x, *y, NEST_Z / 2.0 + 6.0);
        let bore = centered_cylinder(
            format!("startup_shear_ramp_validation_datum_pin_bore_{i}"),
            3.2,
            15.0,
            24,
        )
        .translate(*x, *y, NEST_Z / 2.0 + 6.0);
        bosses = bosses + (boss - bore);
    }
    bosses
}

fn cassette_clamps() -> Part {
    let mut clamps = Part::empty("startup_shear_ramp_validation_cassette_toggle_clamps");
    for i in 0..CLAMP_COUNT {
        let x = centered_index(i % 3, 3, 120.0);
        let y = if i < 3 {
            CASSETTE_Y / 2.0 + 52.0
        } else {
            -(CASSETTE_Y / 2.0 + 52.0)
        };
        clamps = clamps
            + centered_cube(
                format!("startup_shear_ramp_validation_toggle_clamp_foot_{i}"),
                48.0,
                20.0,
                12.0,
            )
            .translate(x, y, NEST_Z / 2.0 + 6.0)
            + centered_cube(
                format!("startup_shear_ramp_validation_toggle_clamp_swing_arm_{i}"),
                68.0,
                10.0,
                8.0,
            )
            .translate(
                x,
                y.signum() * (CASSETTE_Y / 2.0 + 18.0),
                NEST_Z / 2.0 + 24.0,
            );
    }
    clamps
}

fn sterile_bulkhead_panel() -> Part {
    let body = centered_cube(
        "startup_shear_ramp_validation_sterile_bulkhead_panel_body",
        BULKHEAD_X,
        BULKHEAD_Y,
        BULKHEAD_Z,
    );
    let mut bores = Part::empty("startup_shear_ramp_validation_sterile_bulkhead_bores");
    let mut labels = Part::empty("startup_shear_ramp_validation_sterile_bulkhead_labels");
    for i in 0..BULKHEAD_PORTS {
        let y = centered_index(i, BULKHEAD_PORTS, 34.0);
        bores = bores
            + centered_cylinder(
                format!("startup_shear_ramp_validation_bulkhead_port_bore_{i}"),
                BULKHEAD_PORT_D / 2.0,
                BULKHEAD_X + 8.0,
                30,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, y, 8.0);
        labels = labels
            + centered_cube(
                format!("startup_shear_ramp_validation_bulkhead_port_label_land_{i}"),
                48.0,
                10.0,
                4.0,
            )
            .translate(0.0, y + 13.0, BULKHEAD_Z / 2.0 + 2.0);
    }

    let in_header = centered_cube(
        "startup_shear_ramp_validation_bulkhead_sterile_in_header",
        74.0,
        16.0,
        12.0,
    )
    .translate(-44.0, 66.0, BULKHEAD_Z / 2.0 + 6.0);
    let out_header = centered_cube(
        "startup_shear_ramp_validation_bulkhead_sterile_out_header",
        74.0,
        16.0,
        12.0,
    )
    .translate(44.0, -66.0, BULKHEAD_Z / 2.0 + 6.0);

    body - bores + labels + in_header + out_header
}

fn interchangeable_restrictor_plate_bank() -> Part {
    let base = centered_cube(
        "startup_shear_ramp_validation_restrictor_plate_bank_base",
        RESTRICTOR_X,
        RESTRICTOR_Y,
        RESTRICTOR_Z,
    );
    let mut pockets = Part::empty("startup_shear_ramp_validation_restrictor_plate_pockets");
    let mut plates = Part::empty("startup_shear_ramp_validation_restrictor_plates");
    for i in 0..RESTRICTOR_PLATES {
        let x = centered_index(i, RESTRICTOR_PLATES, 60.0);
        pockets = pockets
            + centered_cube(
                format!("startup_shear_ramp_validation_restrictor_plate_pocket_{i}"),
                48.0,
                98.0,
                RESTRICTOR_Z + 4.0,
            )
            .translate(x, 0.0, 4.0);
        plates = plates + restrictor_plate(i).translate(x, 0.0, RESTRICTOR_Z / 2.0 + 5.0);
    }
    base - pockets + plates + ramp_step_witness()
}

fn restrictor_plate(index: usize) -> Part {
    let plate = centered_cube(
        format!("startup_shear_ramp_validation_restrictor_plate_{index}"),
        42.0,
        92.0,
        10.0,
    );
    let channel = centered_cube(
        format!("startup_shear_ramp_validation_restrictor_plate_{index}_ramp_slot"),
        35.0,
        1.0 + index as f64 * 0.35,
        12.0,
    );
    let inlet = centered_cylinder(
        format!("startup_shear_ramp_validation_restrictor_plate_{index}_inlet_bore"),
        3.0,
        48.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, -30.0, 0.0);
    let outlet = centered_cylinder(
        format!("startup_shear_ramp_validation_restrictor_plate_{index}_outlet_bore"),
        3.0,
        48.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, 30.0, 0.0);
    plate - channel - inlet - outlet + restrictor_key_tabs(index)
}

fn restrictor_key_tabs(index: usize) -> Part {
    let mut tabs = Part::empty(format!(
        "startup_shear_ramp_validation_restrictor_plate_{index}_key_tabs"
    ));
    for tab in 0..=index {
        tabs = tabs
            + centered_cube(
                format!("startup_shear_ramp_validation_restrictor_plate_{index}_key_tab_{tab}"),
                6.0,
                4.0,
                4.0,
            )
            .translate(-15.0 + tab as f64 * 7.0, 42.0, 7.0);
    }
    tabs
}

fn ramp_step_witness() -> Part {
    let mut steps = Part::empty("startup_shear_ramp_validation_ramp_step_witness_gauge");
    for i in 0..RAMP_STEP_COUNT {
        steps = steps
            + centered_cube(
                format!("startup_shear_ramp_validation_ramp_step_witness_{i}"),
                34.0,
                8.0,
                4.0 + i as f64 * 2.0,
            )
            .translate(
                centered_index(i, RAMP_STEP_COUNT, 42.0),
                -RESTRICTOR_Y / 2.0 + 14.0,
                RESTRICTOR_Z / 2.0 + 2.0 + i as f64,
            );
    }
    steps
}

fn bubble_observation_window() -> Part {
    let frame = centered_cube(
        "startup_shear_ramp_validation_bubble_observation_window_frame",
        WINDOW_X,
        WINDOW_Y,
        WINDOW_Z,
    );
    let window = centered_cube(
        "startup_shear_ramp_validation_bubble_clear_sight_window",
        WINDOW_X - 54.0,
        WINDOW_Y - 34.0,
        WINDOW_Z + 4.0,
    );
    let flow_bore = centered_cylinder(
        "startup_shear_ramp_validation_bubble_window_flow_bore",
        3.6,
        WINDOW_X + 8.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0);
    frame - window - flow_bore + bubble_scale_ticks()
}

fn bubble_scale_ticks() -> Part {
    let mut ticks = Part::empty("startup_shear_ramp_validation_bubble_scale_ticks");
    for i in 0..BUBBLE_TICK_COUNT {
        let tick_h = if i % 5 == 0 { 18.0 } else { 10.0 };
        ticks = ticks
            + centered_cube(
                format!("startup_shear_ramp_validation_bubble_scale_tick_{i}"),
                2.0,
                tick_h,
                3.0,
            )
            .translate(
                centered_index(i, BUBBLE_TICK_COUNT, 24.0),
                WINDOW_Y / 2.0 - 10.0,
                WINDOW_Z / 2.0 + 1.5,
            );
    }
    ticks
}

fn pressure_transducer_bosses() -> Part {
    let rail = centered_cube(
        "startup_shear_ramp_validation_pressure_transducer_rail",
        PRESSURE_BAR_X,
        PRESSURE_BAR_Y,
        PRESSURE_BAR_Z,
    );
    let mut bosses = Part::empty("startup_shear_ramp_validation_pressure_transducer_bosses");
    for i in 0..PRESSURE_TRANSDUCERS {
        let x = centered_index(i, PRESSURE_TRANSDUCERS, 76.0);
        let boss = centered_cylinder(
            format!("startup_shear_ramp_validation_pressure_boss_{i}"),
            PRESSURE_BOSS_D / 2.0,
            22.0,
            36,
        )
        .translate(x, 0.0, PRESSURE_BAR_Z / 2.0 + 11.0);
        let port = centered_cylinder(
            format!("startup_shear_ramp_validation_pressure_port_bore_{i}"),
            PRESSURE_PORT_D / 2.0,
            PRESSURE_BAR_Z + 28.0,
            24,
        )
        .translate(x, 0.0, PRESSURE_BAR_Z / 2.0 + 10.0);
        let wrench_flat = centered_cube(
            format!("startup_shear_ramp_validation_pressure_boss_wrench_flat_{i}"),
            34.0,
            8.0,
            9.0,
        )
        .translate(x, 20.0, PRESSURE_BAR_Z / 2.0 + 20.0);
        bosses = bosses + (boss - port) + wrench_flat;
    }
    rail + bosses
}

fn flow_sensor_saddle() -> Part {
    let saddle = centered_cube(
        "startup_shear_ramp_validation_flow_sensor_saddle_body",
        FLOW_X,
        FLOW_Y,
        FLOW_Z,
    );
    let instrument_relief = centered_cube(
        "startup_shear_ramp_validation_flow_meter_instrument_relief",
        FLOW_X - 68.0,
        FLOW_Y - 40.0,
        FLOW_Z + 4.0,
    )
    .translate(0.0, 0.0, 10.0);

    let mut channels = Part::empty("startup_shear_ramp_validation_flow_sensor_channels");
    for i in 0..FLOW_SENSOR_CHANNELS {
        channels = channels
            + centered_cylinder(
                format!("startup_shear_ramp_validation_flow_channel_{i}"),
                3.2,
                FLOW_X + 6.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, centered_index(i, FLOW_SENSOR_CHANNELS, 38.0), 0.0);
    }

    saddle - instrument_relief - channels + flow_sensor_retainer_rails()
}

fn flow_sensor_retainer_rails() -> Part {
    let front = centered_cube(
        "startup_shear_ramp_validation_flow_sensor_front_retainer_rail",
        FLOW_X - 36.0,
        10.0,
        16.0,
    )
    .translate(0.0, -FLOW_Y / 2.0 + 14.0, FLOW_Z / 2.0 + 8.0);
    let rear = centered_cube(
        "startup_shear_ramp_validation_flow_sensor_rear_retainer_rail",
        FLOW_X - 36.0,
        10.0,
        16.0,
    )
    .translate(0.0, FLOW_Y / 2.0 - 14.0, FLOW_Z / 2.0 + 8.0);
    front + rear
}

fn waste_recirculation_splitter() -> Part {
    let body = centered_cube(
        "startup_shear_ramp_validation_waste_recirculation_splitter_body",
        SPLITTER_X,
        SPLITTER_Y,
        SPLITTER_Z,
    );
    let trunk = centered_cylinder(
        "startup_shear_ramp_validation_splitter_common_trunk",
        4.0,
        SPLITTER_X + 8.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0);
    let waste = centered_cylinder(
        "startup_shear_ramp_validation_splitter_waste_branch",
        3.8,
        SPLITTER_Y + 8.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(44.0, -18.0, 0.0);
    let recirc = centered_cylinder(
        "startup_shear_ramp_validation_splitter_recirculation_branch",
        3.4,
        SPLITTER_Y + 8.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-44.0, 18.0, 0.0);
    let valve_lands = centered_cube(
        "startup_shear_ramp_validation_splitter_waste_valve_land",
        70.0,
        26.0,
        10.0,
    )
    .translate(44.0, -42.0, SPLITTER_Z / 2.0 + 5.0)
        + centered_cube(
            "startup_shear_ramp_validation_splitter_recirc_valve_land",
            70.0,
            26.0,
            10.0,
        )
        .translate(-44.0, 42.0, SPLITTER_Z / 2.0 + 5.0);

    body - trunk - waste - recirc + valve_lands
}

fn sample_access_port() -> Part {
    let block = centered_cube(
        "startup_shear_ramp_validation_sample_access_port_block",
        SAMPLE_X,
        SAMPLE_Y,
        SAMPLE_Z,
    );
    let mut ports = Part::empty("startup_shear_ramp_validation_sample_access_port_bores");
    for i in 0..SAMPLE_PORTS {
        ports = ports
            + centered_cylinder(
                format!("startup_shear_ramp_validation_sample_septum_bore_{i}"),
                6.5,
                SAMPLE_Z + 6.0,
                30,
            )
            .translate(centered_index(i, SAMPLE_PORTS, 42.0), 0.0, 0.0);
    }
    let cap_chain_land = centered_cube(
        "startup_shear_ramp_validation_sample_cap_chain_land",
        SAMPLE_X - 26.0,
        12.0,
        6.0,
    )
    .translate(0.0, SAMPLE_Y / 2.0 - 18.0, SAMPLE_Z / 2.0 + 3.0);
    block - ports + cap_chain_land
}

fn cable_gas_strain_relief() -> Part {
    let base = centered_cube(
        "startup_shear_ramp_validation_cable_gas_strain_relief_base",
        RELIEF_X,
        RELIEF_Y,
        RELIEF_Z,
    );
    let mut lanes = Part::empty("startup_shear_ramp_validation_cable_gas_relief_lanes");
    for i in 0..STRAIN_RELIEF_LANES {
        let x = centered_index(i, STRAIN_RELIEF_LANES, 42.0);
        lanes = lanes
            + centered_cylinder(
                format!("startup_shear_ramp_validation_cable_gas_relief_lane_{i}"),
                if i < 2 { 5.0 } else { 3.6 },
                RELIEF_Y + 10.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, 4.0);
    }
    let clamp_bar = centered_cube(
        "startup_shear_ramp_validation_cable_gas_relief_clamp_bar",
        RELIEF_X - 42.0,
        14.0,
        12.0,
    )
    .translate(0.0, 0.0, RELIEF_Z / 2.0 + 6.0);
    base - lanes + clamp_bar
}

fn labels_datums_qc_coupons() -> Part {
    let panel = centered_cube(
        "startup_shear_ramp_validation_labels_datums_qc_panel",
        LABEL_X,
        LABEL_Y,
        LABEL_Z,
    );
    let mut features = Part::empty("startup_shear_ramp_validation_labels_datums_qc_features");
    for i in 0..QC_COUPONS {
        features = features
            + centered_cube(
                format!("startup_shear_ramp_validation_qc_witness_coupon_{i}"),
                28.0,
                22.0,
                6.0,
            )
            .translate(
                centered_index(i % 4, 4, 42.0),
                -34.0 + (i / 4) as f64 * 34.0,
                LABEL_Z / 2.0 + 3.0,
            );
    }
    for i in 0..ENGRAVED_DATUMS {
        features = features
            + centered_cube(
                format!("startup_shear_ramp_validation_engraved_datum_land_{i}"),
                34.0,
                8.0,
                3.0,
            )
            .translate(
                centered_index(i, ENGRAVED_DATUMS, 46.0),
                LABEL_Y / 2.0 - 18.0,
                LABEL_Z / 2.0 + 1.5,
            );
    }
    let barcode_land = centered_cube(
        "startup_shear_ramp_validation_run_record_barcode_land",
        LABEL_X - 54.0,
        18.0,
        3.0,
    )
    .translate(0.0, -LABEL_Y / 2.0 + 18.0, LABEL_Z / 2.0 + 1.5);

    panel + features + barcode_land
}

fn mount_points() -> [(f64, f64); 4] {
    [
        (-DECK_X / 2.0 + 64.0, -DECK_Y / 2.0 + 64.0),
        (DECK_X / 2.0 - 64.0, -DECK_Y / 2.0 + 64.0),
        (-DECK_X / 2.0 + 64.0, DECK_Y / 2.0 - 64.0),
        (DECK_X / 2.0 - 64.0, DECK_Y / 2.0 - 64.0),
    ]
}

fn datum_pin_points() -> [(f64, f64); DATUM_PIN_COUNT] {
    [
        (-(CASSETTE_X / 2.0 - 42.0), -(CASSETTE_Y / 2.0 - 36.0)),
        (CASSETTE_X / 2.0 - 42.0, -(CASSETTE_Y / 2.0 - 36.0)),
        (-(CASSETTE_X / 2.0 - 42.0), CASSETTE_Y / 2.0 - 36.0),
        (CASSETTE_X / 2.0 - 42.0, CASSETTE_Y / 2.0 - 36.0),
    ]
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn feature_count() -> usize {
    REQUIRED_FEATURES.len()
}

fn output_count() -> usize {
    OUTPUTS.len()
}

fn fluid_port_count() -> usize {
    BULKHEAD_PORTS + PRESSURE_TRANSDUCERS + FLOW_SENSOR_CHANNELS + SPLIT_BRANCHES + SAMPLE_PORTS
}

fn assert_layout() {
    assert_eq!(output_count(), 12);
    assert_eq!(feature_count(), 12);
    assert!(NEST_X + 80.0 < DECK_X);
    assert!(NEST_Y + 110.0 < DECK_Y);
    assert!(CASSETTE_KEEP_OUT_Z > NEST_Z);
    assert!(fluid_port_count() >= 15);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exports_twelve_prefixed_outputs() {
        assert_eq!(output_count(), 12);
        assert!(OUTPUTS.iter().all(|path| path.starts_with(
            "output/closed_cassette_perfusion_startup_shear_ramp_validation_station_"
        )));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
    }

    #[test]
    fn required_feature_metadata_covers_requested_hardware() {
        for feature in [
            "cassette_clamp_datum",
            "sterile_fluidic_in_bulkhead",
            "sterile_fluidic_out_bulkhead",
            "interchangeable_restrictor_plate",
            "bubble_observation_window",
            "pressure_transducer_bosses",
            "flow_sensor_saddle",
            "waste_recirculation_split",
            "leak_moat",
            "sample_access_port",
            "cable_gas_routing_strain_relief",
            "labels_datums_qc_witness_coupons",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn station_has_expected_fixture_counts() {
        assert_eq!(DATUM_PIN_COUNT, datum_pin_points().len());
        assert_eq!(CLAMP_COUNT, 6);
        assert_eq!(RESTRICTOR_PLATES, RAMP_STEP_COUNT);
        assert_eq!(QC_COUPONS, 8);
        assert_eq!(ENGRAVED_DATUMS, 6);
    }

    #[test]
    fn fluid_path_has_startup_ramp_instrumentation() {
        assert_eq!(BULKHEAD_PORTS, 4);
        assert_eq!(PRESSURE_TRANSDUCERS, 4);
        assert_eq!(FLOW_SENSOR_CHANNELS, 2);
        assert_eq!(SPLIT_BRANCHES, 2);
        assert_eq!(SAMPLE_PORTS, 3);
        assert_eq!(fluid_port_count(), 15);
    }

    #[test]
    fn centered_index_is_symmetric() {
        assert_eq!(centered_index(0, 5, 60.0), -120.0);
        assert_eq!(centered_index(2, 5, 60.0), 0.0);
        assert_eq!(centered_index(4, 5, 60.0), 120.0);
    }
}
