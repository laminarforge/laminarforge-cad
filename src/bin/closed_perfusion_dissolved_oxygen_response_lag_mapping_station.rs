use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed perfusion dissolved-oxygen response-lag mapping station.
//
// Intent:
// - Package a closed-system validation fixture for proving that dissolved
//   oxygen or optical O2 sensors in perfusion loops respond fast enough to
//   visible gas-equilibrated media step changes.
// - Keep the step-change cartridges, sensor pocket ladder, residence-time
//   witness loop, flow-rate tokens, bubble/degas window, reference sample
//   split, release/hold/reject gates, and evidence capture geometry explicit.
// - Represent bought sensors, tubing, gas-equilibrated media cartridges, and
//   optical windows as deterministic placeholder geometry only.
//
// This is mechanical validation packaging only. It is not a dissolved oxygen
// calibration standard, pressure-rated wetted design, process release method,
// or biological acceptance protocol.

const PREFIX: &str = "closed_perfusion_dissolved_oxygen_response_lag_mapping_station";
const OUTPUT_PREFIX: &str = "output/closed_perfusion_dissolved_oxygen_response_lag_mapping_station";

const OUTPUTS: [&str; 12] = [
    "output/closed_perfusion_dissolved_oxygen_response_lag_mapping_station_containment_deck.stl",
    "output/closed_perfusion_dissolved_oxygen_response_lag_mapping_station_step_change_gas_equilibrated_media_cartridge_bank.stl",
    "output/closed_perfusion_dissolved_oxygen_response_lag_mapping_station_sensor_pocket_ladder.stl",
    "output/closed_perfusion_dissolved_oxygen_response_lag_mapping_station_residence_time_witness_loop.stl",
    "output/closed_perfusion_dissolved_oxygen_response_lag_mapping_station_flow_rate_token_rail.stl",
    "output/closed_perfusion_dissolved_oxygen_response_lag_mapping_station_bubble_degas_window_panel.stl",
    "output/closed_perfusion_dissolved_oxygen_response_lag_mapping_station_reference_sample_split_manifold.stl",
    "output/closed_perfusion_dissolved_oxygen_response_lag_mapping_station_release_hold_reject_evidence_gates.stl",
    "output/closed_perfusion_dissolved_oxygen_response_lag_mapping_station_timestamp_hypoxia_hyperoxia_event_token_strip.stl",
    "output/closed_perfusion_dissolved_oxygen_response_lag_mapping_station_barcode_coa_custody_plate.stl",
    "output/closed_perfusion_dissolved_oxygen_response_lag_mapping_station_evidence_camera_robot_keepout_bridge.stl",
    "output/closed_perfusion_dissolved_oxygen_response_lag_mapping_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "step_change_gas_equilibrated_media_cartridges",
    "sensor_pocket_ladder",
    "residence_time_witness_loop",
    "flow_rate_token_rail",
    "bubble_degas_window",
    "reference_sample_split",
    "release_hold_reject_evidence_gates",
    "timestamp_hypoxia_hyperoxia_event_tokens",
    "barcode_coa_custody_plate",
    "evidence_camera_robot_keepout_bridge",
    "closed_loop_route_identity",
];

const LIMITATIONS: [&str; 6] = [
    "mechanical_validation_packaging_only",
    "not_a_dissolved_oxygen_calibration_standard",
    "not_pressure_rated_wetted_design",
    "not_process_release_method",
    "not_biological_acceptance_protocol",
    "external_sensors_tubing_media_and_optics",
];

const REPRODUCIBILITY_CONTROLS: [&str; 6] = [
    "fixed_output_manifest",
    "millimeter_units",
    "no_random_inputs",
    "named_deterministic_geometry",
    "static_feature_counts",
    "stable_layout_rectangles",
];

const FORBIDDEN_CLAIM_TERMS: [&str; 6] = [
    "patient",
    "therapy",
    "therapeutic",
    "diagnosis",
    "clinical",
    "sterility assurance",
];

const PARAMETRIC_REVISION: &str =
    "closed_perfusion_dissolved_oxygen_response_lag_mapping_station_v1";
const UNITS: &str = "millimeters";
const GRID_STEP_MM: f64 = 2.0;
const DEFAULT_TESSELLATION_SEGMENTS: u32 = 32;

const STATION_X: f64 = 1540.0;
const STATION_Y: f64 = 980.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 46.0;
const BASIN_X: f64 = STATION_X - 124.0;
const BASIN_Y: f64 = STATION_Y - 124.0;
const BASIN_RECESS_Z: f64 = 7.0;
const DESIGN_CLEARANCE: f64 = 12.0;
const LAYOUT_CLEARANCE: f64 = 10.0;
const MOUNT_SLOTS: usize = 10;
const DATUM_TARGETS: usize = 6;

const LOOP_LANES: usize = 8;
const SENSOR_STATIONS_PER_LANE: usize = 5;
const SENSOR_POCKETS: usize = LOOP_LANES * SENSOR_STATIONS_PER_LANE;
const STEP_CARTRIDGES: usize = 6;
const STEP_STATE_LABELS: [&str; STEP_CARTRIDGES] = [
    "low_o2",
    "low_mid_o2",
    "normoxic",
    "high_mid_o2",
    "hyperoxic",
    "blank",
];
const FLOW_RATES_UL_MIN: [usize; 8] = [25, 50, 100, 150, 250, 400, 650, 900];
const RESIDENCE_SEGMENTS: usize = LOOP_LANES;
const RESIDENCE_TAPS_PER_SEGMENT: usize = 4;
const RESIDENCE_TAPS: usize = RESIDENCE_SEGMENTS * RESIDENCE_TAPS_PER_SEGMENT;
const BUBBLE_WINDOWS: usize = LOOP_LANES;
const DEGAS_WINDOWS: usize = LOOP_LANES;
const REFERENCE_SAMPLE_BRANCHES: usize = LOOP_LANES;
const REFERENCE_SPLIT_RATIO_TOKENS: usize = 3;
const EVENT_TOKENS: usize = 12;
const DISPOSITION_GATES: usize = 3;
const TOKENS_PER_DISPOSITION: usize = LOOP_LANES;
const BARCODE_LANDS: usize = 12;
const COA_LANDS: usize = 4;
const CAMERA_TARGETS: usize = 5;
const KEEP_OUT_GAUGES: usize = 6;
const ROUTE_IDENTITY_TICKS: usize = LOOP_LANES + STEP_CARTRIDGES;

const CARTRIDGE_CENTER: (f64, f64) = (-520.0, 315.0);
const CARTRIDGE_X: f64 = 360.0;
const CARTRIDGE_Y: f64 = 210.0;
const CARTRIDGE_Z: f64 = 58.0;
const CARTRIDGE_PITCH_X: f64 = 54.0;
const CARTRIDGE_SLOT_X: f64 = 34.0;
const CARTRIDGE_SLOT_Y: f64 = 140.0;

const SENSOR_CENTER: (f64, f64) = (-520.0, -130.0);
const SENSOR_X: f64 = 365.0;
const SENSOR_Y: f64 = 420.0;
const SENSOR_Z: f64 = 48.0;
const SENSOR_PITCH_X: f64 = 58.0;
const SENSOR_PITCH_Y: f64 = 44.0;
const SENSOR_POCKET_D: f64 = 22.0;
const SENSOR_LADDER_RISE_STEP: f64 = 2.8;

const LOOP_CENTER: (f64, f64) = (5.0, 92.0);
const LOOP_X: f64 = 620.0;
const LOOP_Y: f64 = 522.0;
const LOOP_Z: f64 = 24.0;
const LOOP_TRACE_W: f64 = 7.0;
const LOOP_TRACE_Z: f64 = 7.0;
const LOOP_RUN_X: f64 = 512.0;
const LOOP_PITCH_Y: f64 = 54.0;
const WITNESS_TAP_D: f64 = 14.0;

const FLOW_TOKEN_CENTER: (f64, f64) = (535.0, 343.0);
const FLOW_TOKEN_X: f64 = 350.0;
const FLOW_TOKEN_Y: f64 = 160.0;
const FLOW_TOKEN_Z: f64 = 32.0;
const FLOW_TOKEN_D: f64 = 24.0;
const FLOW_TOKEN_PITCH_X: f64 = 38.0;

const BUBBLE_CENTER: (f64, f64) = (505.0, 65.0);
const BUBBLE_X: f64 = 320.0;
const BUBBLE_Y: f64 = 286.0;
const BUBBLE_Z: f64 = 46.0;
const WINDOW_X: f64 = 86.0;
const WINDOW_Y: f64 = 20.0;
const WINDOW_PITCH_Y: f64 = 30.0;

const SAMPLE_CENTER: (f64, f64) = (498.0, -214.0);
const SAMPLE_X: f64 = 334.0;
const SAMPLE_Y: f64 = 220.0;
const SAMPLE_Z: f64 = 48.0;
const SAMPLE_WELL_D: f64 = 28.0;
const SAMPLE_PITCH_X: f64 = 70.0;
const SAMPLE_PITCH_Y: f64 = 42.0;

const GATE_CENTER: (f64, f64) = (-452.0, -405.0);
const GATE_X: f64 = 430.0;
const GATE_Y: f64 = 70.0;
const GATE_Z: f64 = 36.0;
const GATE_NAMES: [&str; DISPOSITION_GATES] = ["release", "hold", "reject"];

const EVENT_CENTER: (f64, f64) = (-20.0, -405.0);
const EVENT_X: f64 = 385.0;
const EVENT_Y: f64 = 70.0;
const EVENT_Z: f64 = 24.0;
const EVENT_TOKEN_D: f64 = 22.0;

const CUSTODY_CENTER: (f64, f64) = (378.0, -405.0);
const CUSTODY_X: f64 = 380.0;
const CUSTODY_Y: f64 = 70.0;
const CUSTODY_Z: f64 = 16.0;

const BRIDGE_CENTER: (f64, f64) = (8.0, -18.0);
const BRIDGE_X: f64 = 1410.0;
const BRIDGE_Y: f64 = 58.0;
const BRIDGE_Z: f64 = 230.0;
const KEEP_OUT_X: f64 = 1450.0;
const KEEP_OUT_Y: f64 = 880.0;
const KEEP_OUT_Z: f64 = 170.0;
const FRONT_ROBOT_CLEARANCE: f64 = 320.0;
const REAR_CARTRIDGE_SERVICE_CLEARANCE: f64 = 250.0;
const SENSOR_LIFT_CLEARANCE_Z: f64 = 165.0;
const CARTRIDGE_LIFT_CLEARANCE_Z: f64 = 210.0;
const SAMPLE_ACCESS_CLEARANCE: f64 = 190.0;

const LABEL_BAR_COUNT: usize = 8;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_station(self) -> bool {
        let usable_x = STATION_X / 2.0 - RIM_W - DESIGN_CLEARANCE;
        let usable_y = STATION_Y / 2.0 - RIM_W - DESIGN_CLEARANCE;
        self.center.0.abs() + self.x / 2.0 <= usable_x
            && self.center.1.abs() + self.y / 2.0 <= usable_y
    }

    fn overlaps_with_clearance(self, other: Rect, clearance: f64) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();
        dx < (self.x + other.x) / 2.0 + clearance && dy < (self.y + other.y) / 2.0 + clearance
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = containment_deck();
    export(OUTPUTS[0], &deck);

    let cartridges = step_change_gas_equilibrated_media_cartridge_bank();
    export(OUTPUTS[1], &cartridges);

    let sensors = sensor_pocket_ladder();
    export(OUTPUTS[2], &sensors);

    let loop_bed = residence_time_witness_loop();
    export(OUTPUTS[3], &loop_bed);

    let flow_tokens = flow_rate_token_rail();
    export(OUTPUTS[4], &flow_tokens);

    let bubble = bubble_degas_window_panel();
    export(OUTPUTS[5], &bubble);

    let samples = reference_sample_split_manifold();
    export(OUTPUTS[6], &samples);

    let gates = release_hold_reject_evidence_gates();
    export(OUTPUTS[7], &gates);

    let events = timestamp_hypoxia_hyperoxia_event_token_strip();
    export(OUTPUTS[8], &events);

    let custody = barcode_coa_custody_plate();
    export(OUTPUTS[9], &custody);

    let bridge = evidence_camera_robot_keepout_bridge();
    export(OUTPUTS[10], &bridge);

    let assembly = deck
        + cartridges
        + sensors
        + loop_bed
        + flow_tokens
        + bubble
        + samples
        + gates
        + events
        + custody
        + bridge;
    export(OUTPUTS[11], &assembly);

    println!(
        "Closed perfusion dissolved-oxygen response-lag mapping station: {STATION_X:.0}mm x {STATION_Y:.0}mm contained deck, {STEP_CARTRIDGES} gas-equilibrated media step cartridges, {LOOP_LANES} closed loop lanes, {SENSOR_POCKETS} sensor ladder pockets, {RESIDENCE_TAPS} residence-time taps, and {EVENT_TOKENS} hypoxia/hyperoxia event tokens."
    );
    println!(
        "Evidence coverage: {BUBBLE_WINDOWS} bubble windows, {DEGAS_WINDOWS} degas windows, {REFERENCE_SAMPLE_BRANCHES} reference split branches, {BARCODE_LANDS} barcode lands, {COA_LANDS} COA lands, {DISPOSITION_GATES} disposition gates, {CAMERA_TARGETS} camera targets, {KEEP_OUT_GAUGES} keepout gauges, {} limitations, and {} outputs.",
        LIMITATIONS.len(),
        OUTPUTS.len()
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        format!("{PREFIX}_containment_deck_base"),
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let basin = centered_cube(
        format!("{PREFIX}_sumped_response_lag_basin_recess"),
        BASIN_X,
        BASIN_Y,
        BASIN_RECESS_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0 - BASIN_RECESS_Z / 2.0 + 0.8);
    let drain = centered_cylinder(
        format!("{PREFIX}_front_right_low_point_drain_bore"),
        9.0,
        68.0,
        DEFAULT_TESSELLATION_SEGMENTS,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 80.0, -(STATION_Y / 2.0 - 20.0), 0.0);

    deck - basin - drain
        + containment_rims()
        + mounting_slot_bosses()
        + datum_targets()
        + module_socket_rails()
        + closed_loop_route_identity_ticks()
}

fn containment_rims() -> Part {
    let z = BASE_Z / 2.0 + RIM_Z / 2.0;
    let left = centered_cube(
        format!("{PREFIX}_left_containment_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-(STATION_X / 2.0 - RIM_W / 2.0), 0.0, z);
    let right = centered_cube(
        format!("{PREFIX}_right_containment_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, z);
    let rear = centered_cube(
        format!("{PREFIX}_rear_containment_rim"),
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, z);
    let front = centered_cube(
        format!("{PREFIX}_front_low_profile_robot_access_lip"),
        STATION_X,
        RIM_W,
        RIM_Z * 0.58,
    )
    .translate(
        0.0,
        -(STATION_Y / 2.0 - RIM_W / 2.0),
        BASE_Z / 2.0 + RIM_Z * 0.29,
    );
    left + right + rear + front
}

fn mounting_slot_bosses() -> Part {
    let mut slots = Part::empty(format!("{PREFIX}_mounting_slot_bosses"));
    for (i, (x, y)) in mount_points().iter().enumerate() {
        let boss = centered_cube(format!("{PREFIX}_m6_slot_boss_{i}"), 50.0, 24.0, 8.0).translate(
            *x,
            *y,
            BASE_Z / 2.0 + 4.0,
        );
        let bore = centered_cylinder(format!("{PREFIX}_m6_slot_bore_{i}"), 3.5, 18.0, 24)
            .translate(*x, *y, BASE_Z / 2.0 + 4.0);
        slots = slots + (boss - bore);
    }
    slots
}

fn datum_targets() -> Part {
    let mut datums = Part::empty(format!("{PREFIX}_station_datum_targets"));
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 74.0), -(STATION_Y / 2.0 - 66.0)),
        (STATION_X / 2.0 - 74.0, -(STATION_Y / 2.0 - 66.0)),
        (-(STATION_X / 2.0 - 74.0), STATION_Y / 2.0 - 66.0),
        (STATION_X / 2.0 - 74.0, STATION_Y / 2.0 - 66.0),
        (0.0, STATION_Y / 2.0 - 66.0),
        (0.0, -(STATION_Y / 2.0 - 66.0)),
    ]
    .iter()
    .enumerate()
    {
        datums = datums
            + fiducial_target(format!("{PREFIX}_datum_target_{i}")).translate(
                *x,
                *y,
                BASE_Z / 2.0 + 2.0,
            );
    }
    datums
}

fn module_socket_rails() -> Part {
    let mut rails = Part::empty(format!("{PREFIX}_module_socket_rails"));
    for (i, rect) in non_overlay_rects().iter().enumerate() {
        rails = rails
            + centered_cube(
                format!("{PREFIX}_module_{i}_front_socket_rail"),
                rect.x + 22.0,
                5.0,
                8.0,
            )
            .translate(
                rect.center.0,
                rect.center.1 - rect.y / 2.0 - 5.0,
                BASE_Z / 2.0 + 4.0,
            )
            + centered_cube(
                format!("{PREFIX}_module_{i}_rear_socket_rail"),
                rect.x + 22.0,
                5.0,
                8.0,
            )
            .translate(
                rect.center.0,
                rect.center.1 + rect.y / 2.0 + 5.0,
                BASE_Z / 2.0 + 4.0,
            );
    }
    rails
}

fn closed_loop_route_identity_ticks() -> Part {
    let rail = centered_cube(
        format!("{PREFIX}_closed_loop_route_identity_rail"),
        LOOP_X - 72.0,
        9.0,
        6.0,
    )
    .translate(
        LOOP_CENTER.0,
        LOOP_CENTER.1 + LOOP_Y / 2.0 - 26.0,
        BASE_Z / 2.0 + 3.0,
    );
    let mut ticks = Part::empty(format!("{PREFIX}_closed_loop_route_identity_ticks"));
    for tick in 0..ROUTE_IDENTITY_TICKS {
        ticks = ticks
            + centered_cube(
                format!("{PREFIX}_route_identity_tick_{tick:02}"),
                18.0,
                15.0,
                7.0,
            )
            .translate(
                LOOP_CENTER.0 + centered_index(tick, ROUTE_IDENTITY_TICKS, 34.0),
                LOOP_CENTER.1 + LOOP_Y / 2.0 - 26.0,
                BASE_Z / 2.0 + 6.5,
            );
    }
    rail + ticks
}

fn step_change_gas_equilibrated_media_cartridge_bank() -> Part {
    let bank = centered_cube(
        format!("{PREFIX}_step_change_cartridge_bank_body"),
        CARTRIDGE_X,
        CARTRIDGE_Y,
        CARTRIDGE_Z,
    )
    .translate(
        CARTRIDGE_CENTER.0,
        CARTRIDGE_CENTER.1,
        BASE_Z / 2.0 + CARTRIDGE_Z / 2.0,
    );
    bank - cartridge_slot_cuts() - cartridge_tube_bores()
        + cartridge_retainers()
        + gas_equilibration_state_plaque_row()
        + step_transition_key_ridge()
}

fn cartridge_slot_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_step_cartridge_slot_cuts"));
    for cartridge in 0..STEP_CARTRIDGES {
        cuts = cuts
            + centered_cube(
                format!(
                    "{PREFIX}_{}_step_media_cartridge_slot_cut",
                    STEP_STATE_LABELS[cartridge]
                ),
                CARTRIDGE_SLOT_X,
                CARTRIDGE_SLOT_Y,
                CARTRIDGE_Z + 10.0,
            )
            .translate(
                cartridge_x(cartridge),
                CARTRIDGE_CENTER.1 - 4.0,
                BASE_Z / 2.0 + CARTRIDGE_Z / 2.0 - 4.0,
            );
    }
    cuts
}

fn cartridge_tube_bores() -> Part {
    let mut bores = Part::empty(format!("{PREFIX}_cartridge_tube_bores"));
    for cartridge in 0..STEP_CARTRIDGES {
        let x = cartridge_x(cartridge);
        for side in 0..2 {
            bores = bores
                + centered_cylinder(
                    format!(
                        "{PREFIX}_{}_step_cartridge_tube_bore_{side}",
                        STEP_STATE_LABELS[cartridge]
                    ),
                    3.2,
                    52.0,
                    20,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(
                    x,
                    CARTRIDGE_CENTER.1 - CARTRIDGE_Y / 2.0 + 26.0 + side as f64 * 112.0,
                    BASE_Z / 2.0 + 23.0,
                );
        }
    }
    bores
}

fn cartridge_retainers() -> Part {
    let mut retainers = Part::empty(format!("{PREFIX}_step_cartridge_retainers"));
    for cartridge in 0..STEP_CARTRIDGES {
        let x = cartridge_x(cartridge);
        retainers = retainers
            + centered_cube(
                format!(
                    "{PREFIX}_{}_step_cartridge_front_latch",
                    STEP_STATE_LABELS[cartridge]
                ),
                CARTRIDGE_SLOT_X + 14.0,
                6.0,
                9.0,
            )
            .translate(
                x,
                CARTRIDGE_CENTER.1 - CARTRIDGE_Y / 2.0 + 28.0,
                BASE_Z / 2.0 + CARTRIDGE_Z + 4.5,
            )
            + centered_cube(
                format!(
                    "{PREFIX}_{}_step_cartridge_rear_latch",
                    STEP_STATE_LABELS[cartridge]
                ),
                CARTRIDGE_SLOT_X + 14.0,
                6.0,
                9.0,
            )
            .translate(
                x,
                CARTRIDGE_CENTER.1 + CARTRIDGE_Y / 2.0 - 28.0,
                BASE_Z / 2.0 + CARTRIDGE_Z + 4.5,
            );
    }
    retainers
}

fn gas_equilibration_state_plaque_row() -> Part {
    let mut plaques = Part::empty(format!("{PREFIX}_gas_equilibration_state_plaque_row"));
    for cartridge in 0..STEP_CARTRIDGES {
        plaques = plaques
            + csg_label_plaque(
                format!(
                    "{PREFIX}_{}_gas_equilibration_state_plaque",
                    STEP_STATE_LABELS[cartridge]
                ),
                40.0,
                15.0,
                4.0,
                110 + cartridge,
            )
            .translate(
                cartridge_x(cartridge),
                CARTRIDGE_CENTER.1 + CARTRIDGE_Y / 2.0 - 14.0,
                BASE_Z / 2.0 + CARTRIDGE_Z + 2.0,
            );
    }
    plaques
}

fn step_transition_key_ridge() -> Part {
    let ridge = centered_cube(
        format!("{PREFIX}_low_normoxic_high_o2_step_transition_key_ridge"),
        CARTRIDGE_X - 44.0,
        8.0,
        8.0,
    )
    .translate(
        CARTRIDGE_CENTER.0,
        CARTRIDGE_CENTER.1 - CARTRIDGE_Y / 2.0 + 18.0,
        BASE_Z / 2.0 + CARTRIDGE_Z + 4.0,
    );
    let mut ticks = Part::empty(format!("{PREFIX}_step_transition_direction_ticks"));
    for tick in 0..STEP_CARTRIDGES {
        ticks = ticks
            + centered_cube(
                format!("{PREFIX}_step_transition_tick_{tick}"),
                5.0 + tick as f64 * 1.2,
                18.0,
                9.0,
            )
            .translate(
                cartridge_x(tick),
                CARTRIDGE_CENTER.1 - CARTRIDGE_Y / 2.0 + 18.0,
                BASE_Z / 2.0 + CARTRIDGE_Z + 8.5,
            );
    }
    ridge + ticks
}

fn sensor_pocket_ladder() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_sensor_pocket_ladder_body"),
        SENSOR_X,
        SENSOR_Y,
        SENSOR_Z,
    )
    .translate(
        SENSOR_CENTER.0,
        SENSOR_CENTER.1,
        BASE_Z / 2.0 + SENSOR_Z / 2.0,
    );
    body - sensor_pocket_cuts()
        + sensor_ladder_step_risers()
        + sensor_retainer_tabs()
        + sensor_baseline_blank_pockets()
}

fn sensor_pocket_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_sensor_pocket_ladder_cuts"));
    for lane in 0..LOOP_LANES {
        for station in 0..SENSOR_STATIONS_PER_LANE {
            let (x, y) = sensor_pocket_position(lane, station);
            cuts = cuts
                + centered_cylinder(
                    format!("{PREFIX}_lane_{lane:02}_sensor_station_{station}_pocket_cut"),
                    SENSOR_POCKET_D / 2.0,
                    SENSOR_Z + 12.0,
                    DEFAULT_TESSELLATION_SEGMENTS,
                )
                .translate(x, y, BASE_Z / 2.0 + SENSOR_Z / 2.0 - 4.0);
        }
    }
    cuts
}

fn sensor_ladder_step_risers() -> Part {
    let mut risers = Part::empty(format!("{PREFIX}_sensor_pocket_ladder_step_risers"));
    for station in 0..SENSOR_STATIONS_PER_LANE {
        risers = risers
            + centered_cube(
                format!("{PREFIX}_sensor_response_lag_step_{station}_riser"),
                SENSOR_X - 48.0,
                5.0,
                4.0 + station as f64 * SENSOR_LADDER_RISE_STEP,
            )
            .translate(
                SENSOR_CENTER.0,
                SENSOR_CENTER.1 + centered_index(station, SENSOR_STATIONS_PER_LANE, SENSOR_PITCH_Y),
                BASE_Z / 2.0 + SENSOR_Z + 2.0 + station as f64 * SENSOR_LADDER_RISE_STEP / 2.0,
            );
    }
    risers
}

fn sensor_retainer_tabs() -> Part {
    let mut tabs = Part::empty(format!("{PREFIX}_sensor_retainer_tabs"));
    for lane in 0..LOOP_LANES {
        for station in 0..SENSOR_STATIONS_PER_LANE {
            let (x, y) = sensor_pocket_position(lane, station);
            tabs = tabs
                + centered_cube(
                    format!("{PREFIX}_lane_{lane:02}_sensor_station_{station}_retainer_tab"),
                    18.0,
                    4.0,
                    5.0,
                )
                .translate(
                    x,
                    y + SENSOR_POCKET_D / 2.0 + 6.0,
                    BASE_Z / 2.0 + SENSOR_Z + 2.5,
                );
        }
    }
    tabs
}

fn sensor_baseline_blank_pockets() -> Part {
    let mut blanks = Part::empty(format!("{PREFIX}_sensor_baseline_blank_pockets"));
    for lane in 0..LOOP_LANES {
        blanks = blanks
            + port_ring(
                format!("{PREFIX}_lane_{lane:02}_baseline_blank_reference_ring"),
                24.0,
                12.0,
                6.0,
            )
            .translate(
                SENSOR_CENTER.0 + centered_index(lane, LOOP_LANES, SENSOR_PITCH_X),
                SENSOR_CENTER.1 - SENSOR_Y / 2.0 + 22.0,
                BASE_Z / 2.0 + SENSOR_Z + 3.0,
            );
    }
    blanks
}

fn residence_time_witness_loop() -> Part {
    let plate = centered_cube(
        format!("{PREFIX}_residence_time_witness_loop_bed"),
        LOOP_X,
        LOOP_Y,
        LOOP_Z,
    )
    .translate(LOOP_CENTER.0, LOOP_CENTER.1, BASE_Z / 2.0 + LOOP_Z / 2.0);
    let recess = centered_cube(
        format!("{PREFIX}_residence_time_witness_loop_recess"),
        LOOP_X - 52.0,
        LOOP_Y - 42.0,
        5.0,
    )
    .translate(LOOP_CENTER.0, LOOP_CENTER.1, BASE_Z / 2.0 + LOOP_Z - 2.0);
    plate - recess + residence_loop_traces() + residence_time_tap_rings() + step_front_markers()
}

fn residence_loop_traces() -> Part {
    let mut traces = Part::empty(format!("{PREFIX}_residence_time_loop_traces"));
    let z = BASE_Z / 2.0 + LOOP_Z + LOOP_TRACE_Z / 2.0;
    for lane in 0..LOOP_LANES {
        let y = loop_lane_y(lane);
        let x0 = LOOP_CENTER.0 - LOOP_RUN_X / 2.0;
        let x1 = LOOP_CENTER.0 + LOOP_RUN_X / 2.0;
        traces = traces
            + centered_cube(
                format!("{PREFIX}_lane_{lane:02}_forward_residence_trace"),
                LOOP_RUN_X,
                LOOP_TRACE_W,
                LOOP_TRACE_Z,
            )
            .translate(LOOP_CENTER.0, y + 9.0, z)
            + centered_cube(
                format!("{PREFIX}_lane_{lane:02}_return_residence_trace"),
                LOOP_RUN_X - 78.0,
                LOOP_TRACE_W,
                LOOP_TRACE_Z,
            )
            .translate(LOOP_CENTER.0, y - 9.0, z)
            + port_ring(
                format!("{PREFIX}_lane_{lane:02}_left_residence_loop_turn"),
                30.0,
                14.0,
                LOOP_TRACE_Z,
            )
            .translate(x0, y, z)
            + port_ring(
                format!("{PREFIX}_lane_{lane:02}_right_residence_loop_turn"),
                30.0,
                14.0,
                LOOP_TRACE_Z,
            )
            .translate(x1, y, z);
    }
    traces
}

fn residence_time_tap_rings() -> Part {
    let mut taps = Part::empty(format!("{PREFIX}_residence_time_tap_rings"));
    for lane in 0..LOOP_LANES {
        let y = loop_lane_y(lane);
        for tap in 0..RESIDENCE_TAPS_PER_SEGMENT {
            taps = taps
                + port_ring(
                    format!("{PREFIX}_lane_{lane:02}_residence_time_tap_{tap}"),
                    WITNESS_TAP_D,
                    5.5,
                    5.0,
                )
                .translate(
                    LOOP_CENTER.0 + centered_index(tap, RESIDENCE_TAPS_PER_SEGMENT, 132.0),
                    y + 9.0,
                    BASE_Z / 2.0 + LOOP_Z + LOOP_TRACE_Z + 2.5,
                );
        }
    }
    taps
}

fn step_front_markers() -> Part {
    let mut markers = Part::empty(format!("{PREFIX}_step_front_arrival_markers"));
    for lane in 0..LOOP_LANES {
        markers = markers
            + centered_cube(
                format!("{PREFIX}_lane_{lane:02}_step_front_arrival_marker"),
                34.0,
                5.0,
                7.0,
            )
            .translate(
                LOOP_CENTER.0 + LOOP_RUN_X / 2.0 - 66.0,
                loop_lane_y(lane) + 9.0,
                BASE_Z / 2.0 + LOOP_Z + LOOP_TRACE_Z + 3.5,
            );
    }
    markers
}

fn flow_rate_token_rail() -> Part {
    let rail = centered_cube(
        format!("{PREFIX}_flow_rate_token_rail_body"),
        FLOW_TOKEN_X,
        FLOW_TOKEN_Y,
        FLOW_TOKEN_Z,
    )
    .translate(
        FLOW_TOKEN_CENTER.0,
        FLOW_TOKEN_CENTER.1,
        BASE_Z / 2.0 + FLOW_TOKEN_Z / 2.0,
    );
    rail + flow_rate_tokens() + pump_setpoint_check_lands() + token_parking_lips()
}

fn flow_rate_tokens() -> Part {
    let mut tokens = Part::empty(format!("{PREFIX}_flow_rate_tokens"));
    for (token, rate) in FLOW_RATES_UL_MIN.iter().enumerate() {
        tokens = tokens
            + port_ring(
                format!("{PREFIX}_{rate}_ul_min_flow_rate_token"),
                FLOW_TOKEN_D,
                FLOW_TOKEN_D - 8.0,
                6.0,
            )
            .translate(
                FLOW_TOKEN_CENTER.0
                    + centered_index(token, FLOW_RATES_UL_MIN.len(), FLOW_TOKEN_PITCH_X),
                FLOW_TOKEN_CENTER.1 + 25.0,
                BASE_Z / 2.0 + FLOW_TOKEN_Z + 3.0,
            )
            + csg_label_plaque(
                format!("{PREFIX}_{rate}_ul_min_flow_rate_label"),
                30.0,
                12.0,
                3.0,
                *rate,
            )
            .translate(
                FLOW_TOKEN_CENTER.0
                    + centered_index(token, FLOW_RATES_UL_MIN.len(), FLOW_TOKEN_PITCH_X),
                FLOW_TOKEN_CENTER.1 - 26.0,
                BASE_Z / 2.0 + FLOW_TOKEN_Z + 1.5,
            );
    }
    tokens
}

fn pump_setpoint_check_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_pump_setpoint_check_lands"));
    for lane in 0..LOOP_LANES {
        lands = lands
            + centered_cube(
                format!("{PREFIX}_lane_{lane:02}_pump_setpoint_check_land"),
                24.0,
                12.0,
                5.0,
            )
            .translate(
                FLOW_TOKEN_CENTER.0 + centered_index(lane, LOOP_LANES, 38.0),
                FLOW_TOKEN_CENTER.1 - FLOW_TOKEN_Y / 2.0 + 18.0,
                BASE_Z / 2.0 + FLOW_TOKEN_Z + 2.5,
            );
    }
    lands
}

fn token_parking_lips() -> Part {
    let front = centered_cube(
        format!("{PREFIX}_flow_rate_token_front_parking_lip"),
        FLOW_TOKEN_X - 38.0,
        6.0,
        8.0,
    )
    .translate(
        FLOW_TOKEN_CENTER.0,
        FLOW_TOKEN_CENTER.1 - FLOW_TOKEN_Y / 2.0 + 8.0,
        BASE_Z / 2.0 + FLOW_TOKEN_Z + 4.0,
    );
    let rear = centered_cube(
        format!("{PREFIX}_flow_rate_token_rear_parking_lip"),
        FLOW_TOKEN_X - 38.0,
        6.0,
        8.0,
    )
    .translate(
        FLOW_TOKEN_CENTER.0,
        FLOW_TOKEN_CENTER.1 + FLOW_TOKEN_Y / 2.0 - 8.0,
        BASE_Z / 2.0 + FLOW_TOKEN_Z + 4.0,
    );
    front + rear
}

fn bubble_degas_window_panel() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_bubble_degas_window_panel_body"),
        BUBBLE_X,
        BUBBLE_Y,
        BUBBLE_Z,
    )
    .translate(
        BUBBLE_CENTER.0,
        BUBBLE_CENTER.1,
        BASE_Z / 2.0 + BUBBLE_Z / 2.0,
    );
    body - bubble_window_cuts() - degas_window_cuts()
        + bubble_window_frames()
        + degas_membrane_frames()
        + wet_dry_reference_tabs()
}

fn bubble_window_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_bubble_window_cuts"));
    for lane in 0..BUBBLE_WINDOWS {
        cuts = cuts
            + centered_cube(
                format!("{PREFIX}_lane_{lane:02}_bubble_witness_window_cut"),
                WINDOW_X,
                WINDOW_Y,
                BUBBLE_Z + 8.0,
            )
            .translate(
                BUBBLE_CENTER.0 - 54.0,
                bubble_lane_y(lane),
                BASE_Z / 2.0 + BUBBLE_Z / 2.0,
            );
    }
    cuts
}

fn degas_window_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_degas_window_cuts"));
    for lane in 0..DEGAS_WINDOWS {
        cuts = cuts
            + centered_cube(
                format!("{PREFIX}_lane_{lane:02}_degas_membrane_window_cut"),
                WINDOW_X - 16.0,
                WINDOW_Y,
                BUBBLE_Z + 8.0,
            )
            .translate(
                BUBBLE_CENTER.0 + 68.0,
                bubble_lane_y(lane),
                BASE_Z / 2.0 + BUBBLE_Z / 2.0,
            );
    }
    cuts
}

fn bubble_window_frames() -> Part {
    let mut frames = Part::empty(format!("{PREFIX}_bubble_window_frames"));
    for lane in 0..BUBBLE_WINDOWS {
        frames = frames
            + rectangular_frame_xy(
                format!("{PREFIX}_lane_{lane:02}_bubble_window_frame"),
                WINDOW_X + 16.0,
                WINDOW_Y + 12.0,
                5.0,
                6.0,
            )
            .translate(
                BUBBLE_CENTER.0 - 54.0,
                bubble_lane_y(lane),
                BASE_Z / 2.0 + BUBBLE_Z + 3.0,
            );
    }
    frames
}

fn degas_membrane_frames() -> Part {
    let mut frames = Part::empty(format!("{PREFIX}_degas_membrane_window_frames"));
    for lane in 0..DEGAS_WINDOWS {
        frames = frames
            + rectangular_frame_xy(
                format!("{PREFIX}_lane_{lane:02}_degas_membrane_frame"),
                WINDOW_X,
                WINDOW_Y + 12.0,
                5.0,
                6.0,
            )
            .translate(
                BUBBLE_CENTER.0 + 68.0,
                bubble_lane_y(lane),
                BASE_Z / 2.0 + BUBBLE_Z + 3.0,
            );
    }
    frames
}

fn wet_dry_reference_tabs() -> Part {
    let mut tabs = Part::empty(format!("{PREFIX}_bubble_degas_wet_dry_reference_tabs"));
    for tab in 0..4 {
        tabs = tabs
            + csg_label_plaque(
                format!("{PREFIX}_bubble_degas_wet_dry_reference_tab_{tab}"),
                45.0,
                14.0,
                4.0,
                220 + tab,
            )
            .translate(
                BUBBLE_CENTER.0 - BUBBLE_X / 2.0 + 28.0,
                BUBBLE_CENTER.1 + centered_index(tab, 4, 56.0),
                BASE_Z / 2.0 + BUBBLE_Z + 2.0,
            );
    }
    tabs
}

fn reference_sample_split_manifold() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_reference_sample_split_manifold_body"),
        SAMPLE_X,
        SAMPLE_Y,
        SAMPLE_Z,
    )
    .translate(
        SAMPLE_CENTER.0,
        SAMPLE_CENTER.1,
        BASE_Z / 2.0 + SAMPLE_Z / 2.0,
    );
    body - sample_well_cuts() - sample_branch_bores()
        + sample_well_rings()
        + split_ratio_tokens()
        + retain_reference_capture_rail()
}

fn sample_well_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_reference_sample_well_cuts"));
    for branch in 0..REFERENCE_SAMPLE_BRANCHES {
        let (x, y) = sample_branch_position(branch);
        cuts = cuts
            + centered_cylinder(
                format!("{PREFIX}_lane_{branch:02}_reference_sample_well_cut"),
                SAMPLE_WELL_D / 2.0,
                SAMPLE_Z + 10.0,
                DEFAULT_TESSELLATION_SEGMENTS,
            )
            .translate(x, y, BASE_Z / 2.0 + SAMPLE_Z / 2.0 - 4.0);
    }
    cuts
}

fn sample_branch_bores() -> Part {
    let mut bores = Part::empty(format!("{PREFIX}_reference_sample_branch_bores"));
    for branch in 0..REFERENCE_SAMPLE_BRANCHES {
        let (x, y) = sample_branch_position(branch);
        bores = bores
            + centered_cylinder(
                format!("{PREFIX}_lane_{branch:02}_reference_sample_split_bore"),
                2.8,
                68.0,
                18,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, y - 24.0, BASE_Z / 2.0 + 20.0);
    }
    bores
}

fn sample_well_rings() -> Part {
    let mut rings = Part::empty(format!("{PREFIX}_reference_sample_well_rings"));
    for branch in 0..REFERENCE_SAMPLE_BRANCHES {
        let (x, y) = sample_branch_position(branch);
        rings = rings
            + port_ring(
                format!("{PREFIX}_lane_{branch:02}_reference_sample_well_ring"),
                SAMPLE_WELL_D + 13.0,
                SAMPLE_WELL_D,
                6.0,
            )
            .translate(x, y, BASE_Z / 2.0 + SAMPLE_Z + 3.0);
    }
    rings
}

fn split_ratio_tokens() -> Part {
    let mut tokens = Part::empty(format!("{PREFIX}_reference_split_ratio_tokens"));
    for token in 0..REFERENCE_SPLIT_RATIO_TOKENS {
        tokens = tokens
            + csg_label_plaque(
                format!("{PREFIX}_reference_split_ratio_token_{token}"),
                52.0,
                15.0,
                4.0,
                310 + token,
            )
            .translate(
                SAMPLE_CENTER.0 + centered_index(token, REFERENCE_SPLIT_RATIO_TOKENS, 70.0),
                SAMPLE_CENTER.1 + SAMPLE_Y / 2.0 - 22.0,
                BASE_Z / 2.0 + SAMPLE_Z + 2.0,
            );
    }
    tokens
}

fn retain_reference_capture_rail() -> Part {
    centered_cube(
        format!("{PREFIX}_retain_reference_sample_capture_rail"),
        SAMPLE_X - 56.0,
        8.0,
        8.0,
    )
    .translate(
        SAMPLE_CENTER.0,
        SAMPLE_CENTER.1 - SAMPLE_Y / 2.0 + 24.0,
        BASE_Z / 2.0 + SAMPLE_Z + 4.0,
    )
}

fn release_hold_reject_evidence_gates() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_release_hold_reject_gate_bank_body"),
        GATE_X,
        GATE_Y,
        GATE_Z,
    )
    .translate(GATE_CENTER.0, GATE_CENTER.1, BASE_Z / 2.0 + GATE_Z / 2.0);
    body + disposition_gate_posts() + disposition_token_slots() + gate_label_plaques()
}

fn disposition_gate_posts() -> Part {
    let mut posts = Part::empty(format!("{PREFIX}_disposition_gate_posts"));
    for gate in 0..DISPOSITION_GATES {
        let x = disposition_gate_x(gate);
        posts = posts
            + centered_cube(
                format!("{PREFIX}_{}_gate_post_left", GATE_NAMES[gate]),
                8.0,
                GATE_Y + 10.0,
                gate_height(gate),
            )
            .translate(
                x - 50.0,
                GATE_CENTER.1,
                BASE_Z / 2.0 + GATE_Z + gate_height(gate) / 2.0,
            )
            + centered_cube(
                format!("{PREFIX}_{}_gate_post_right", GATE_NAMES[gate]),
                8.0,
                GATE_Y + 10.0,
                gate_height(gate),
            )
            .translate(
                x + 50.0,
                GATE_CENTER.1,
                BASE_Z / 2.0 + GATE_Z + gate_height(gate) / 2.0,
            )
            + centered_cube(
                format!("{PREFIX}_{}_gate_crossbar", GATE_NAMES[gate]),
                108.0,
                8.0,
                8.0,
            )
            .translate(x, GATE_CENTER.1, BASE_Z / 2.0 + GATE_Z + gate_height(gate));
    }
    posts
}

fn disposition_token_slots() -> Part {
    let mut slots = Part::empty(format!("{PREFIX}_disposition_token_slots"));
    for gate in 0..DISPOSITION_GATES {
        let x0 = disposition_gate_x(gate);
        for token in 0..TOKENS_PER_DISPOSITION {
            slots = slots
                + centered_cube(
                    format!(
                        "{PREFIX}_{}_lane_{token:02}_evidence_token_slot",
                        GATE_NAMES[gate]
                    ),
                    10.0,
                    22.0,
                    5.0,
                )
                .translate(
                    x0 + centered_index(token, TOKENS_PER_DISPOSITION, 12.0),
                    GATE_CENTER.1 - 22.0,
                    BASE_Z / 2.0 + GATE_Z + 2.5,
                );
        }
    }
    slots
}

fn gate_label_plaques() -> Part {
    let mut plaques = Part::empty(format!("{PREFIX}_release_hold_reject_gate_label_plaques"));
    for gate in 0..DISPOSITION_GATES {
        plaques = plaques
            + csg_label_plaque(
                format!("{PREFIX}_{}_gate_label_plaque", GATE_NAMES[gate]),
                70.0,
                16.0,
                4.0,
                410 + gate,
            )
            .translate(
                disposition_gate_x(gate),
                GATE_CENTER.1 + 21.0,
                BASE_Z / 2.0 + GATE_Z + 2.0,
            );
    }
    plaques
}

fn timestamp_hypoxia_hyperoxia_event_token_strip() -> Part {
    let strip = centered_cube(
        format!("{PREFIX}_timestamp_hypoxia_hyperoxia_event_token_strip_body"),
        EVENT_X,
        EVENT_Y,
        EVENT_Z,
    )
    .translate(EVENT_CENTER.0, EVENT_CENTER.1, BASE_Z / 2.0 + EVENT_Z / 2.0);
    strip + event_token_rings() + response_lag_scale_ticks() + reference_clock_beacons()
}

fn event_token_rings() -> Part {
    let mut rings = Part::empty(format!("{PREFIX}_hypoxia_hyperoxia_event_token_rings"));
    for token in 0..EVENT_TOKENS {
        let row = token / 6;
        let col = token % 6;
        rings = rings
            + port_ring(
                format!("{PREFIX}_event_token_{token:02}_ring"),
                EVENT_TOKEN_D,
                EVENT_TOKEN_D - 8.0,
                5.0,
            )
            .translate(
                EVENT_CENTER.0 + centered_index(col, 6, 44.0),
                EVENT_CENTER.1 - 16.0 + row as f64 * 32.0,
                BASE_Z / 2.0 + EVENT_Z + 2.5,
            );
    }
    rings
}

fn response_lag_scale_ticks() -> Part {
    let mut ticks = Part::empty(format!("{PREFIX}_response_lag_scale_ticks"));
    for tick in 0..9 {
        ticks = ticks
            + centered_cube(
                format!("{PREFIX}_response_lag_scale_tick_{tick}"),
                4.0,
                12.0 + tick as f64 * 2.0,
                5.0,
            )
            .translate(
                EVENT_CENTER.0 + centered_index(tick, 9, 34.0),
                EVENT_CENTER.1 + EVENT_Y / 2.0 - 12.0,
                BASE_Z / 2.0 + EVENT_Z + 2.5,
            );
    }
    ticks
}

fn reference_clock_beacons() -> Part {
    let mut beacons = Part::empty(format!("{PREFIX}_reference_clock_beacons"));
    for beacon in 0..4 {
        beacons = beacons
            + centered_cylinder(
                format!("{PREFIX}_reference_clock_beacon_{beacon}"),
                6.0,
                8.0,
                20,
            )
            .translate(
                EVENT_CENTER.0 - EVENT_X / 2.0 + 26.0 + beacon as f64 * 22.0,
                EVENT_CENTER.1 - EVENT_Y / 2.0 + 15.0,
                BASE_Z / 2.0 + EVENT_Z + 4.0,
            );
    }
    beacons
}

fn barcode_coa_custody_plate() -> Part {
    let plate = centered_cube(
        format!("{PREFIX}_barcode_coa_custody_plate_body"),
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    )
    .translate(
        CUSTODY_CENTER.0,
        CUSTODY_CENTER.1,
        BASE_Z / 2.0 + CUSTODY_Z / 2.0,
    );
    plate + barcode_lands() + coa_lands() + tamper_seal_tabs()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_barcode_lands"));
    for land in 0..BARCODE_LANDS {
        let row = land / 6;
        let col = land % 6;
        lands = lands
            + csg_label_plaque(
                format!("{PREFIX}_barcode_land_{land:02}"),
                42.0,
                13.0,
                3.0,
                510 + land,
            )
            .translate(
                CUSTODY_CENTER.0 - 116.0 + col as f64 * 45.0,
                CUSTODY_CENTER.1 - 16.0 + row as f64 * 30.0,
                BASE_Z / 2.0 + CUSTODY_Z + 1.5,
            );
    }
    lands
}

fn coa_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_coa_certificate_lands"));
    for land in 0..COA_LANDS {
        lands = lands
            + centered_cube(
                format!("{PREFIX}_coa_certificate_land_{land}"),
                46.0,
                18.0,
                4.0,
            )
            .translate(
                CUSTODY_CENTER.0 + CUSTODY_X / 2.0 - 52.0,
                CUSTODY_CENTER.1 + centered_index(land, COA_LANDS, 15.0),
                BASE_Z / 2.0 + CUSTODY_Z + 2.0,
            );
    }
    lands
}

fn tamper_seal_tabs() -> Part {
    let mut tabs = Part::empty(format!("{PREFIX}_custody_tamper_seal_tabs"));
    for tab in 0..4 {
        tabs = tabs
            + centered_cube(format!("{PREFIX}_tamper_seal_tab_{tab}"), 18.0, 8.0, 5.0).translate(
                CUSTODY_CENTER.0 - CUSTODY_X / 2.0 + 22.0,
                CUSTODY_CENTER.1 + centered_index(tab, 4, 17.0),
                BASE_Z / 2.0 + CUSTODY_Z + 2.5,
            );
    }
    tabs
}

fn evidence_camera_robot_keepout_bridge() -> Part {
    let bridge = centered_cube(
        format!("{PREFIX}_camera_evidence_bridge_crossbar"),
        BRIDGE_X,
        BRIDGE_Y,
        18.0,
    )
    .translate(BRIDGE_CENTER.0, BRIDGE_CENTER.1, BASE_Z / 2.0 + BRIDGE_Z);
    bridge
        + camera_mounts()
        + vertical_bridge_posts()
        + robot_service_keepout_frame()
        + clearance_gauge_bars()
}

fn camera_mounts() -> Part {
    let mut mounts = Part::empty(format!("{PREFIX}_camera_mounts"));
    for camera in 0..CAMERA_TARGETS {
        mounts = mounts
            + rectangular_frame_xy(
                format!("{PREFIX}_camera_target_{camera}_field_of_view_frame"),
                110.0,
                32.0,
                5.0,
                6.0,
            )
            .translate(
                BRIDGE_CENTER.0 + centered_index(camera, CAMERA_TARGETS, 265.0),
                BRIDGE_CENTER.1,
                BASE_Z / 2.0 + BRIDGE_Z + 11.0,
            )
            + centered_cylinder(
                format!("{PREFIX}_camera_target_{camera}_mount_boss"),
                8.0,
                10.0,
                24,
            )
            .translate(
                BRIDGE_CENTER.0 + centered_index(camera, CAMERA_TARGETS, 265.0),
                BRIDGE_CENTER.1,
                BASE_Z / 2.0 + BRIDGE_Z + 20.0,
            );
    }
    mounts
}

fn vertical_bridge_posts() -> Part {
    let mut posts = Part::empty(format!("{PREFIX}_vertical_camera_bridge_posts"));
    for (i, x) in [
        -(BRIDGE_X / 2.0 - 38.0),
        -BRIDGE_X / 4.0,
        BRIDGE_X / 4.0,
        BRIDGE_X / 2.0 - 38.0,
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(
                format!("{PREFIX}_camera_bridge_post_{i}"),
                16.0,
                26.0,
                BRIDGE_Z,
            )
            .translate(
                BRIDGE_CENTER.0 + *x,
                BRIDGE_CENTER.1,
                BASE_Z / 2.0 + BRIDGE_Z / 2.0,
            );
    }
    posts
}

fn robot_service_keepout_frame() -> Part {
    keepout_frame(
        format!("{PREFIX}_robot_service_keepout_frame"),
        KEEP_OUT_X,
        KEEP_OUT_Y,
        8.0,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0 + KEEP_OUT_Z)
}

fn clearance_gauge_bars() -> Part {
    let mut gauges = Part::empty(format!("{PREFIX}_clearance_gauge_bars"));
    let gauge_specs = [
        (
            "front_robot_clearance",
            FRONT_ROBOT_CLEARANCE,
            -(STATION_Y / 2.0 - 92.0),
        ),
        (
            "rear_cartridge_service_clearance",
            REAR_CARTRIDGE_SERVICE_CLEARANCE,
            STATION_Y / 2.0 - 92.0,
        ),
        (
            "sensor_lift_clearance",
            SENSOR_LIFT_CLEARANCE_Z,
            SENSOR_CENTER.1,
        ),
        (
            "cartridge_lift_clearance",
            CARTRIDGE_LIFT_CLEARANCE_Z,
            CARTRIDGE_CENTER.1,
        ),
        (
            "sample_access_clearance",
            SAMPLE_ACCESS_CLEARANCE,
            SAMPLE_CENTER.1,
        ),
        ("bridge_keepout_height", KEEP_OUT_Z, BRIDGE_CENTER.1),
    ];
    for (i, (name, span, y)) in gauge_specs.iter().enumerate() {
        gauges = gauges
            + gauge_bar(format!("{PREFIX}_{name}_gauge_{i}"), *span, 7.0, 7.0).translate(
                centered_index(i, KEEP_OUT_GAUGES, 220.0),
                *y,
                BASE_Z / 2.0 + KEEP_OUT_Z + 16.0,
            );
    }
    gauges
}

fn mount_points() -> [(f64, f64); MOUNT_SLOTS] {
    [
        (-(STATION_X / 2.0 - 66.0), -(STATION_Y / 2.0 - 60.0)),
        (STATION_X / 2.0 - 66.0, -(STATION_Y / 2.0 - 60.0)),
        (-(STATION_X / 2.0 - 66.0), STATION_Y / 2.0 - 60.0),
        (STATION_X / 2.0 - 66.0, STATION_Y / 2.0 - 60.0),
        (0.0, -(STATION_Y / 2.0 - 60.0)),
        (0.0, STATION_Y / 2.0 - 60.0),
        (-(STATION_X / 2.0 - 66.0), 0.0),
        (STATION_X / 2.0 - 66.0, 0.0),
        (-(STATION_X / 4.0), STATION_Y / 2.0 - 60.0),
        (STATION_X / 4.0, STATION_Y / 2.0 - 60.0),
    ]
}

fn module_rects() -> [Rect; 10] {
    [
        rect(
            "step_change_gas_equilibrated_media_cartridge_bank",
            CARTRIDGE_CENTER,
            CARTRIDGE_X,
            CARTRIDGE_Y,
        ),
        rect("sensor_pocket_ladder", SENSOR_CENTER, SENSOR_X, SENSOR_Y),
        rect("residence_time_witness_loop", LOOP_CENTER, LOOP_X, LOOP_Y),
        rect(
            "flow_rate_token_rail",
            FLOW_TOKEN_CENTER,
            FLOW_TOKEN_X,
            FLOW_TOKEN_Y,
        ),
        rect(
            "bubble_degas_window_panel",
            BUBBLE_CENTER,
            BUBBLE_X,
            BUBBLE_Y,
        ),
        rect(
            "reference_sample_split_manifold",
            SAMPLE_CENTER,
            SAMPLE_X,
            SAMPLE_Y,
        ),
        rect(
            "release_hold_reject_evidence_gates",
            GATE_CENTER,
            GATE_X,
            GATE_Y,
        ),
        rect(
            "timestamp_hypoxia_hyperoxia_event_token_strip",
            EVENT_CENTER,
            EVENT_X,
            EVENT_Y,
        ),
        rect(
            "barcode_coa_custody_plate",
            CUSTODY_CENTER,
            CUSTODY_X,
            CUSTODY_Y,
        ),
        rect(
            "evidence_camera_robot_keepout_bridge",
            BRIDGE_CENTER,
            BRIDGE_X,
            BRIDGE_Y,
        ),
    ]
}

fn non_overlay_rects() -> [Rect; 9] {
    [
        rect(
            "step_change_gas_equilibrated_media_cartridge_bank",
            CARTRIDGE_CENTER,
            CARTRIDGE_X,
            CARTRIDGE_Y,
        ),
        rect("sensor_pocket_ladder", SENSOR_CENTER, SENSOR_X, SENSOR_Y),
        rect("residence_time_witness_loop", LOOP_CENTER, LOOP_X, LOOP_Y),
        rect(
            "flow_rate_token_rail",
            FLOW_TOKEN_CENTER,
            FLOW_TOKEN_X,
            FLOW_TOKEN_Y,
        ),
        rect(
            "bubble_degas_window_panel",
            BUBBLE_CENTER,
            BUBBLE_X,
            BUBBLE_Y,
        ),
        rect(
            "reference_sample_split_manifold",
            SAMPLE_CENTER,
            SAMPLE_X,
            SAMPLE_Y,
        ),
        rect(
            "release_hold_reject_evidence_gates",
            GATE_CENTER,
            GATE_X,
            GATE_Y,
        ),
        rect(
            "timestamp_hypoxia_hyperoxia_event_token_strip",
            EVENT_CENTER,
            EVENT_X,
            EVENT_Y,
        ),
        rect(
            "barcode_coa_custody_plate",
            CUSTODY_CENTER,
            CUSTODY_X,
            CUSTODY_Y,
        ),
    ]
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn cartridge_x(index: usize) -> f64 {
    CARTRIDGE_CENTER.0 + centered_index(index, STEP_CARTRIDGES, CARTRIDGE_PITCH_X)
}

fn sensor_pocket_position(lane: usize, station: usize) -> (f64, f64) {
    (
        SENSOR_CENTER.0 + centered_index(lane, LOOP_LANES, SENSOR_PITCH_X),
        SENSOR_CENTER.1 + centered_index(station, SENSOR_STATIONS_PER_LANE, SENSOR_PITCH_Y),
    )
}

fn loop_lane_y(index: usize) -> f64 {
    LOOP_CENTER.1 + centered_index(index, LOOP_LANES, LOOP_PITCH_Y)
}

fn bubble_lane_y(index: usize) -> f64 {
    BUBBLE_CENTER.1 + centered_index(index, BUBBLE_WINDOWS, WINDOW_PITCH_Y)
}

fn sample_branch_position(branch: usize) -> (f64, f64) {
    let row = branch / 4;
    let col = branch % 4;
    (
        SAMPLE_CENTER.0 + centered_index(col, 4, SAMPLE_PITCH_X),
        SAMPLE_CENTER.1 + centered_index(row, 2, SAMPLE_PITCH_Y),
    )
}

fn disposition_gate_x(gate: usize) -> f64 {
    GATE_CENTER.0 + centered_index(gate, DISPOSITION_GATES, 136.0)
}

fn gate_height(gate: usize) -> f64 {
    match gate {
        0 => 24.0,
        1 => 38.0,
        _ => 54.0,
    }
}

fn rectangular_frame_xy(
    name: impl Into<String>,
    outer_x: f64,
    outer_y: f64,
    rail: f64,
    z: f64,
) -> Part {
    let name = name.into();
    centered_cube(format!("{name}_outer"), outer_x, outer_y, z)
        - centered_cube(
            format!("{name}_inner_clearance"),
            outer_x - 2.0 * rail,
            outer_y - 2.0 * rail,
            z + 1.0,
        )
}

fn keepout_frame(name: impl Into<String>, x: f64, y: f64, z: f64) -> Part {
    let name = name.into();
    let front =
        centered_cube(format!("{name}_front_rail"), x, 8.0, z).translate(0.0, -y / 2.0, 0.0);
    let rear = centered_cube(format!("{name}_rear_rail"), x, 8.0, z).translate(0.0, y / 2.0, 0.0);
    let left = centered_cube(format!("{name}_left_rail"), 8.0, y, z).translate(-x / 2.0, 0.0, 0.0);
    let right = centered_cube(format!("{name}_right_rail"), 8.0, y, z).translate(x / 2.0, 0.0, 0.0);
    front + rear + left + right
}

fn port_ring(name: impl Into<String>, outer_d: f64, inner_d: f64, z: f64) -> Part {
    let name = name.into();
    centered_cylinder(
        format!("{name}_outer"),
        outer_d / 2.0,
        z,
        DEFAULT_TESSELLATION_SEGMENTS,
    ) - centered_cylinder(format!("{name}_inner"), inner_d / 2.0, z + 1.0, 24)
}

fn fiducial_target(name: impl Into<String>) -> Part {
    let name = name.into();
    let ring = port_ring(format!("{name}_ring"), 22.0, 8.0, 4.0);
    let crosshair = centered_cube(format!("{name}_crosshair_x"), 26.0, 2.0, 3.0)
        + centered_cube(format!("{name}_crosshair_y"), 2.0, 26.0, 3.0);
    ring + crosshair
}

fn gauge_bar(name: impl Into<String>, x: f64, y: f64, z: f64) -> Part {
    let name = name.into();
    let bar = centered_cube(format!("{name}_bar"), x, y, z);
    let tick_a = centered_cube(format!("{name}_tick_a"), 14.0, 14.0, z + 6.0).translate(
        -x / 2.0,
        -y / 2.0,
        0.0,
    );
    let tick_b = centered_cube(format!("{name}_tick_b"), 14.0, 14.0, z + 6.0).translate(
        x / 2.0,
        y / 2.0,
        0.0,
    );
    bar + tick_a + tick_b
}

fn csg_label_plaque(name: impl Into<String>, x: f64, y: f64, z: f64, seed: usize) -> Part {
    let name = name.into();
    let sheet = centered_cube(format!("{name}_sheet"), x, y, z);
    let mut bars = Part::empty(format!("{name}_raised_barcode_bars"));
    for i in 0..LABEL_BAR_COUNT {
        let width = 1.8 + ((seed + i) % 4) as f64 * 1.1;
        let height = (y - 5.0 - (i % 3) as f64).max(3.0);
        let x_offset = -x / 2.0 + 7.0 + i as f64 * ((x - 16.0) / LABEL_BAR_COUNT as f64);
        bars = bars
            + centered_cube(format!("{name}_bar_{i}"), width, height, z + 1.2).translate(
                x_offset,
                0.0,
                z / 2.0 + 0.6,
            );
    }
    let corner = centered_cube(format!("{name}_orientation_corner"), 7.0, 3.0, z + 1.4).translate(
        x / 2.0 - 6.0,
        y / 2.0 - 4.0,
        z / 2.0 + 0.7,
    );
    sheet + bars + corner
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 12);
    assert!(OUTPUTS.iter().all(|path| path.starts_with(OUTPUT_PREFIX)));
    assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
    assert_eq!(
        OUTPUTS[OUTPUTS.len() - 1],
        format!("{OUTPUT_PREFIX}_assembly.stl")
    );
    assert_eq!(REQUIRED_FEATURES.len(), 11);
    assert_eq!(LIMITATIONS.len(), 6);
    assert_eq!(REPRODUCIBILITY_CONTROLS.len(), 6);
    assert_eq!(PARAMETRIC_REVISION, format!("{PREFIX}_v1"));
    assert_eq!(UNITS, "millimeters");
    assert_eq!(GRID_STEP_MM, 2.0);
    assert_eq!(DEFAULT_TESSELLATION_SEGMENTS, 32);
    assert_eq!(STEP_CARTRIDGES, STEP_STATE_LABELS.len());
    assert_eq!(SENSOR_POCKETS, LOOP_LANES * SENSOR_STATIONS_PER_LANE);
    assert_eq!(
        RESIDENCE_TAPS,
        RESIDENCE_SEGMENTS * RESIDENCE_TAPS_PER_SEGMENT
    );
    assert_eq!(RESIDENCE_SEGMENTS, LOOP_LANES);
    assert_eq!(FLOW_RATES_UL_MIN.len(), LOOP_LANES);
    assert_eq!(BUBBLE_WINDOWS, LOOP_LANES);
    assert_eq!(DEGAS_WINDOWS, LOOP_LANES);
    assert_eq!(REFERENCE_SAMPLE_BRANCHES, LOOP_LANES);
    assert_eq!(DISPOSITION_GATES, GATE_NAMES.len());
    assert_eq!(TOKENS_PER_DISPOSITION, LOOP_LANES);
    assert_eq!(ROUTE_IDENTITY_TICKS, LOOP_LANES + STEP_CARTRIDGES);
    assert_eq!(MOUNT_SLOTS, mount_points().len());
    assert_eq!(DATUM_TARGETS, 6);
    assert_eq!(KEEP_OUT_GAUGES, 6);
    assert!(FLOW_RATES_UL_MIN.windows(2).all(|pair| pair[0] < pair[1]));
    assert!(FRONT_ROBOT_CLEARANCE >= 300.0);
    assert!(REAR_CARTRIDGE_SERVICE_CLEARANCE >= 240.0);
    assert!(SENSOR_LIFT_CLEARANCE_Z > SENSOR_Z + BASE_Z);
    assert!(CARTRIDGE_LIFT_CLEARANCE_Z > CARTRIDGE_Z + BASE_Z);
    assert!(SAMPLE_ACCESS_CLEARANCE > SAMPLE_Z + BASE_Z);
    assert_no_scope_claim_terms();

    for required in [
        "step_change_gas_equilibrated_media_cartridges",
        "sensor_pocket_ladder",
        "residence_time_witness_loop",
        "flow_rate_token_rail",
        "bubble_degas_window",
        "reference_sample_split",
        "release_hold_reject_evidence_gates",
        "timestamp_hypoxia_hyperoxia_event_tokens",
        "barcode_coa_custody_plate",
        "evidence_camera_robot_keepout_bridge",
        "closed_loop_route_identity",
    ] {
        assert!(REQUIRED_FEATURES.contains(&required));
    }

    for rect in module_rects() {
        assert!(rect.fits_inside_station(), "{} exceeds station", rect.name);
    }

    let rects = non_overlay_rects();
    for i in 0..rects.len() {
        for j in (i + 1)..rects.len() {
            assert!(
                !rects[i].overlaps_with_clearance(rects[j], LAYOUT_CLEARANCE),
                "{} overlaps {}",
                rects[i].name,
                rects[j].name
            );
        }
    }
}

fn assert_no_scope_claim_terms() {
    let searchable = format!(
        "{} {} {} {} {}",
        REQUIRED_FEATURES.join(" "),
        LIMITATIONS.join(" "),
        OUTPUTS.join(" "),
        PARAMETRIC_REVISION,
        PREFIX
    )
    .to_lowercase();
    for term in FORBIDDEN_CLAIM_TERMS {
        assert!(
            !searchable.contains(term),
            "claim term should not be present: {term}"
        );
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_stable_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        for output in OUTPUTS {
            assert!(output.starts_with(&format!("output/{PREFIX}_")), "{output}");
            assert!(output.ends_with(".stl"), "{output}");
        }
        assert_eq!(
            OUTPUTS,
            [
                "output/closed_perfusion_dissolved_oxygen_response_lag_mapping_station_containment_deck.stl",
                "output/closed_perfusion_dissolved_oxygen_response_lag_mapping_station_step_change_gas_equilibrated_media_cartridge_bank.stl",
                "output/closed_perfusion_dissolved_oxygen_response_lag_mapping_station_sensor_pocket_ladder.stl",
                "output/closed_perfusion_dissolved_oxygen_response_lag_mapping_station_residence_time_witness_loop.stl",
                "output/closed_perfusion_dissolved_oxygen_response_lag_mapping_station_flow_rate_token_rail.stl",
                "output/closed_perfusion_dissolved_oxygen_response_lag_mapping_station_bubble_degas_window_panel.stl",
                "output/closed_perfusion_dissolved_oxygen_response_lag_mapping_station_reference_sample_split_manifold.stl",
                "output/closed_perfusion_dissolved_oxygen_response_lag_mapping_station_release_hold_reject_evidence_gates.stl",
                "output/closed_perfusion_dissolved_oxygen_response_lag_mapping_station_timestamp_hypoxia_hyperoxia_event_token_strip.stl",
                "output/closed_perfusion_dissolved_oxygen_response_lag_mapping_station_barcode_coa_custody_plate.stl",
                "output/closed_perfusion_dissolved_oxygen_response_lag_mapping_station_evidence_camera_robot_keepout_bridge.stl",
                "output/closed_perfusion_dissolved_oxygen_response_lag_mapping_station_assembly.stl",
            ]
        );
    }

    #[test]
    fn requested_validation_features_are_represented() {
        for feature in [
            "step_change_gas_equilibrated_media_cartridges",
            "sensor_pocket_ladder",
            "residence_time_witness_loop",
            "flow_rate_token_rail",
            "bubble_degas_window",
            "reference_sample_split",
            "release_hold_reject_evidence_gates",
            "timestamp_hypoxia_hyperoxia_event_tokens",
            "barcode_coa_custody_plate",
            "evidence_camera_robot_keepout_bridge",
            "closed_loop_route_identity",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn dimensions_and_counts_pin_response_lag_mapping_capacity() {
        assert_eq!(LOOP_LANES, 8);
        assert_eq!(STEP_CARTRIDGES, 6);
        assert_eq!(STEP_STATE_LABELS[0], "low_o2");
        assert_eq!(STEP_STATE_LABELS[4], "hyperoxic");
        assert_eq!(SENSOR_STATIONS_PER_LANE, 5);
        assert_eq!(SENSOR_POCKETS, 40);
        assert_eq!(RESIDENCE_TAPS, 32);
        assert_eq!(BUBBLE_WINDOWS + DEGAS_WINDOWS, 16);
        assert_eq!(REFERENCE_SAMPLE_BRANCHES, LOOP_LANES);
        assert_eq!(EVENT_TOKENS, 12);
        assert_eq!(BARCODE_LANDS, 12);
        assert_eq!(COA_LANDS, 4);
    }

    #[test]
    fn flow_rate_and_disposition_evidence_are_pinned() {
        assert_eq!(FLOW_RATES_UL_MIN, [25, 50, 100, 150, 250, 400, 650, 900]);
        assert_eq!(FLOW_RATES_UL_MIN.len(), LOOP_LANES);
        assert_eq!(GATE_NAMES, ["release", "hold", "reject"]);
        assert_eq!(DISPOSITION_GATES, 3);
        assert_eq!(TOKENS_PER_DISPOSITION, LOOP_LANES);
        assert!(gate_height(0) < gate_height(1));
        assert!(gate_height(1) < gate_height(2));
    }

    #[test]
    fn layout_fits_station_and_preserves_ladder_symmetry() {
        assert_design_constraints();
        for rect in module_rects() {
            assert!(rect.fits_inside_station(), "{rect:?} outside station");
        }
        assert_eq!(
            cartridge_x(0) + cartridge_x(STEP_CARTRIDGES - 1),
            2.0 * CARTRIDGE_CENTER.0
        );
        assert_eq!(
            loop_lane_y(0) + loop_lane_y(LOOP_LANES - 1),
            2.0 * LOOP_CENTER.1
        );
        assert_eq!(
            sensor_pocket_position(0, 0).0 + sensor_pocket_position(LOOP_LANES - 1, 0).0,
            2.0 * SENSOR_CENTER.0
        );
        assert_eq!(
            sensor_pocket_position(0, 0).1
                + sensor_pocket_position(0, SENSOR_STATIONS_PER_LANE - 1).1,
            2.0 * SENSOR_CENTER.1
        );
    }

    #[test]
    fn reproducibility_controls_and_limitations_are_explicit() {
        for control in [
            "fixed_output_manifest",
            "millimeter_units",
            "no_random_inputs",
            "named_deterministic_geometry",
            "static_feature_counts",
            "stable_layout_rectangles",
        ] {
            assert!(REPRODUCIBILITY_CONTROLS.contains(&control));
        }
        assert_eq!(
            PARAMETRIC_REVISION,
            "closed_perfusion_dissolved_oxygen_response_lag_mapping_station_v1"
        );
        assert!(LIMITATIONS.contains(&"mechanical_validation_packaging_only"));
        assert!(LIMITATIONS.contains(&"not_a_dissolved_oxygen_calibration_standard"));
        assert!(LIMITATIONS.contains(&"not_process_release_method"));
        assert_no_scope_claim_terms();
    }
}
