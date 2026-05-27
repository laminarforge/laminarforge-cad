use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed inline media pH/osmolality CO2-lag validation station.
//
// This generator models a contained, source-only validation fixture positioned
// inside or immediately adjacent to the isolator. It packages the mechanical
// interfaces needed to release conditioned media before it reaches scaled
// tissue-chip perfusion lanes: gas-equilibration lag surrogates, pH and
// osmolality split/retain sampling, CO2/O2 probe docks, temperature
// equilibration, residence-time tokenization, bubble/dead-volume witness
// windows, reference standard custody, waste flush capture, imaging fiducials,
// batch traceability, and release/hold/reject disposition geometry.
//
// Research rationale encoded in the layout:
// - Mammalian culture pH is coupled to CO2/bicarbonate equilibrium and can drift
//   during handling or scale-up.
// - O2, CO2, pH, temperature, sample timing, and residence time are direct
//   environmental variables for reproducible tissue-chip culture.
// - Evaporation and concentration effects can shift osmolality, especially in
//   small-volume and long-duration culture formats.
// - Inline release should preserve custody from batch media conditioning through
//   multi-chip perfusion handoff, not a loose benchtop sample.
//
// This is architecture CAD only. It is not a sterile barrier, pressure vessel,
// clinical release specification, sensor metrology package, calibration
// procedure, media recipe, or biological acceptance criterion.

const OUTPUT_PREFIX: &str = "output/closed_inline_media_ph_osmolality_co2_lag_validation_station_";

const OUTPUTS: [&str; 14] = [
    "output/closed_inline_media_ph_osmolality_co2_lag_validation_station_containment_deck.stl",
    "output/closed_inline_media_ph_osmolality_co2_lag_validation_station_scaled_multichip_perfusion_handoff_bulkhead.stl",
    "output/closed_inline_media_ph_osmolality_co2_lag_validation_station_gas_equilibration_lag_loop_surrogate.stl",
    "output/closed_inline_media_ph_osmolality_co2_lag_validation_station_co2_o2_probe_dock_bridge.stl",
    "output/closed_inline_media_ph_osmolality_co2_lag_validation_station_temperature_equilibration_block.stl",
    "output/closed_inline_media_ph_osmolality_co2_lag_validation_station_ph_osmolality_sample_split_retain_wells.stl",
    "output/closed_inline_media_ph_osmolality_co2_lag_validation_station_bubble_dead_volume_window_panel.stl",
    "output/closed_inline_media_ph_osmolality_co2_lag_validation_station_reference_standard_custody_lands.stl",
    "output/closed_inline_media_ph_osmolality_co2_lag_validation_station_timed_residence_loop_token_rail.stl",
    "output/closed_inline_media_ph_osmolality_co2_lag_validation_station_waste_flush_capture_caddy.stl",
    "output/closed_inline_media_ph_osmolality_co2_lag_validation_station_batch_traceability_lands.stl",
    "output/closed_inline_media_ph_osmolality_co2_lag_validation_station_release_hold_reject_disposition_gate.stl",
    "output/closed_inline_media_ph_osmolality_co2_lag_validation_station_camera_illumination_fiducial_bridge.stl",
    "output/closed_inline_media_ph_osmolality_co2_lag_validation_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 17] = [
    "containment_deck",
    "scaled_multi_chip_perfusion_handoff",
    "gas_equilibration_loop_surrogate",
    "ph_sample_wells",
    "osmolality_sample_wells",
    "co2_probe_docks",
    "o2_probe_docks",
    "temperature_equilibration_block",
    "timed_residence_loop_token_rail",
    "bubble_windows",
    "dead_volume_windows",
    "reference_standard_custody_lands",
    "sample_split_retain_wells",
    "waste_flush_capture",
    "camera_illumination_fiducials",
    "release_hold_reject_disposition_features",
    "batch_traceability_lands",
];

const RESEARCH_BASIS: [&str; 6] = [
    "co2_bicarbonate_equilibrium_drives_media_ph",
    "oxygen_and_carbon_dioxide_instability_affects_cell_culture_reproducibility",
    "temperature_and_sample_timing_change_measured_media_state",
    "evaporation_and_concentration_shift_osmolality",
    "perfusion_residence_time_and_sensor_lag_affect_inline_release_evidence",
    "batch_traceability_must_follow_media_to_scaled_chip_lanes",
];

const LIMITATIONS: [&str; 7] = [
    "architecture_cad_only",
    "not_a_sterile_barrier_claim",
    "not_a_pressure_rated_gas_mixer",
    "no_clinical_release_thresholds",
    "no_sensor_metrology_claim",
    "no_media_recipe_or_process_limits",
    "no_live_cell_acceptance_criterion",
];

const PARAMETRIC_REVISION: &str = "closed_inline_media_ph_osmolality_co2_lag_validation_station_v1";
const UNITS: &str = "millimeters";
const CYLINDER_SEGMENTS: u32 = 32;
const FIDUCIAL_SEGMENTS: u32 = 36;

const STATION_X: f64 = 1780.0;
const STATION_Y: f64 = 1020.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 22.0;
const RIM_Z: f64 = 46.0;
const SOCKET_DEPTH: f64 = 6.0;
const MODULE_MARGIN_MM: f64 = 16.0;
const MODULE_CLEARANCE_MM: f64 = 16.0;
const MOUNT_HOLE_D: f64 = 6.8;
const DRAIN_D: f64 = 18.0;
const SUMP_DEPTH: f64 = 7.0;

const CHIP_COLUMNS: usize = 4;
const CHIP_ROWS: usize = 6;
const CHIP_COUNT: usize = CHIP_COLUMNS * CHIP_ROWS;
const PERFUSION_GROUPS: usize = CHIP_ROWS;
const CHIPS_PER_GROUP: usize = CHIP_COLUMNS;
const SAMPLE_REPLICATES: usize = 2;

const HANDOFF_POS: (f64, f64) = (-620.0, 300.0);
const HANDOFF_X: f64 = 360.0;
const HANDOFF_Y: f64 = 220.0;
const HANDOFF_Z: f64 = 78.0;
const HANDOFF_PORTS: usize = PERFUSION_GROUPS * 2;
const HANDOFF_PORT_D: f64 = 10.0;
const CHIP_MAP_CELL_X: f64 = 30.0;
const CHIP_MAP_CELL_Y: f64 = 20.0;

const GAS_POS: (f64, f64) = (-210.0, 300.0);
const GAS_X: f64 = 390.0;
const GAS_Y: f64 = 220.0;
const GAS_Z: f64 = 62.0;
const GAS_LOOP_LANES: usize = PERFUSION_GROUPS;
const GAS_LAG_WINDOWS: usize = PERFUSION_GROUPS;
const GAS_MEMBRANE_WINDOWS: usize = PERFUSION_GROUPS;
const GAS_LOOP_PORTS: usize = GAS_LOOP_LANES * 2;
const GAS_PORT_D: f64 = 8.2;

const PROBE_POS: (f64, f64) = (220.0, 300.0);
const PROBE_X: f64 = 390.0;
const PROBE_Y: f64 = 220.0;
const PROBE_Z: f64 = 58.0;
const CO2_PROBE_DOCKS: usize = PERFUSION_GROUPS;
const O2_PROBE_DOCKS: usize = PERFUSION_GROUPS;
const TOTAL_GAS_PROBE_DOCKS: usize = CO2_PROBE_DOCKS + O2_PROBE_DOCKS;
const PROBE_DOCK_D: f64 = 18.0;
const PROBE_CABLE_CLAMPS: usize = TOTAL_GAS_PROBE_DOCKS;

const TEMP_POS: (f64, f64) = (620.0, 300.0);
const TEMP_X: f64 = 360.0;
const TEMP_Y: f64 = 220.0;
const TEMP_Z: f64 = 60.0;
const TEMPERATURE_ZONES: usize = PERFUSION_GROUPS;
const TEMP_REFERENCE_WELLS: usize = PERFUSION_GROUPS;
const THERMAL_MASS_RIBS: usize = 7;

const SAMPLE_POS: (f64, f64) = (-620.0, 15.0);
const SAMPLE_X: f64 = 360.0;
const SAMPLE_Y: f64 = 220.0;
const SAMPLE_Z: f64 = 58.0;
const PH_SAMPLE_WELLS: usize = PERFUSION_GROUPS * SAMPLE_REPLICATES;
const OSMOLALITY_SAMPLE_WELLS: usize = PERFUSION_GROUPS * SAMPLE_REPLICATES;
const SPLIT_RETAIN_WELLS: usize = PERFUSION_GROUPS * 2;
const TOTAL_SAMPLE_WELLS: usize = PH_SAMPLE_WELLS + OSMOLALITY_SAMPLE_WELLS + SPLIT_RETAIN_WELLS;
const SAMPLE_WELL_D: f64 = 13.6;
const RETAIN_WELL_D: f64 = 15.0;
const SAMPLE_SELECTOR_VALVES: usize = PERFUSION_GROUPS;

const BUBBLE_POS: (f64, f64) = (-210.0, 15.0);
const BUBBLE_X: f64 = 390.0;
const BUBBLE_Y: f64 = 220.0;
const BUBBLE_Z: f64 = 56.0;
const BUBBLE_WINDOWS: usize = PERFUSION_GROUPS;
const DEAD_VOLUME_WINDOWS: usize = PERFUSION_GROUPS;
const DEAD_VOLUME_STEPS_PER_WINDOW: usize = 4;
const BUBBLE_WINDOW_X: f64 = 34.0;
const BUBBLE_WINDOW_Y: f64 = 66.0;

const STANDARD_POS: (f64, f64) = (220.0, 15.0);
const STANDARD_X: f64 = 390.0;
const STANDARD_Y: f64 = 220.0;
const STANDARD_Z: f64 = 44.0;
const PH_STANDARD_LANDS: usize = 4;
const OSMO_STANDARD_LANDS: usize = 4;
const GAS_STANDARD_LANDS: usize = 4;
const REFERENCE_STANDARD_LANDS: usize =
    PH_STANDARD_LANDS + OSMO_STANDARD_LANDS + GAS_STANDARD_LANDS;
const STANDARD_VIAL_D: f64 = 19.0;
const STANDARD_SEAL_WELLS: usize = 6;

const WASTE_POS: (f64, f64) = (620.0, 15.0);
const WASTE_X: f64 = 360.0;
const WASTE_Y: f64 = 220.0;
const WASTE_Z: f64 = 58.0;
const WASTE_STREAMS: usize = PERFUSION_GROUPS;
const WASTE_CUPS_PER_STREAM: usize = 2;
const WASTE_CAPTURE_CUPS: usize = WASTE_STREAMS * WASTE_CUPS_PER_STREAM;
const FLUSH_MANIFOLD_CHANNELS: usize = PERFUSION_GROUPS;
const WASTE_CUP_D: f64 = 25.0;

const TOKEN_POS: (f64, f64) = (-520.0, -285.0);
const TOKEN_X: f64 = 560.0;
const TOKEN_Y: f64 = 190.0;
const TOKEN_Z: f64 = 30.0;
const RESIDENCE_LEVELS: usize = 4;
const RESIDENCE_TOKENS: usize = PERFUSION_GROUPS * RESIDENCE_LEVELS;
const RESIDENCE_LOOP_RAILS: usize = PERFUSION_GROUPS;
const RESIDENCE_TOKEN_D: f64 = 14.0;

const DISPOSITION_POS: (f64, f64) = (100.0, -285.0);
const DISPOSITION_X: f64 = 500.0;
const DISPOSITION_Y: f64 = 190.0;
const DISPOSITION_Z: f64 = 34.0;
const DISPOSITION_NAMES: [&str; 3] = ["release", "hold", "reject"];
const DISPOSITION_STATES: usize = DISPOSITION_NAMES.len();
const DISPOSITION_TOKEN_WELLS: usize = DISPOSITION_STATES * PERFUSION_GROUPS;
const DECISION_CARD_LANDS: usize = DISPOSITION_STATES;

const TRACE_POS: (f64, f64) = (600.0, -285.0);
const TRACE_X: f64 = 360.0;
const TRACE_Y: f64 = 190.0;
const TRACE_Z: f64 = 18.0;
const BATCH_BARCODE_LANDS: usize = 8;
const RUN_CARD_LANDS: usize = 4;
const SAMPLE_TIME_LANDS: usize = PERFUSION_GROUPS;
const CUSTODY_SEAL_WELLS: usize = 6;

const CAMERA_POS: (f64, f64) = (0.0, -445.0);
const CAMERA_X: f64 = 1560.0;
const CAMERA_Y: f64 = 52.0;
const CAMERA_BEAM_Z: f64 = 24.0;
const CAMERA_POST_Z: f64 = 170.0;
const CAMERA_COUNT: usize = 4;
const ILLUMINATION_BARS: usize = 8;
const CAMERA_FIDUCIALS: usize = 6;

const ROUTE_SEGMENTS: usize = 13;
const FLOW_MARKERS: usize = 18;
const DECK_DATUM_TARGETS: usize = 8;

#[derive(Clone, Copy, Debug)]
struct Footprint {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Footprint {
    fn fits_inside_station(self) -> bool {
        self.center.0.abs() + self.x / 2.0 <= STATION_X / 2.0 - RIM_W - MODULE_MARGIN_MM
            && self.center.1.abs() + self.y / 2.0 <= STATION_Y / 2.0 - RIM_W - MODULE_MARGIN_MM
    }

    fn overlaps_with_clearance(self, other: Footprint, clearance: f64) -> bool {
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

    let handoff = scaled_multichip_perfusion_handoff_bulkhead();
    export(OUTPUTS[1], &handoff);

    let gas = gas_equilibration_lag_loop_surrogate();
    export(OUTPUTS[2], &gas);

    let probes = co2_o2_probe_dock_bridge();
    export(OUTPUTS[3], &probes);

    let temperature = temperature_equilibration_block();
    export(OUTPUTS[4], &temperature);

    let samples = ph_osmolality_sample_split_retain_wells();
    export(OUTPUTS[5], &samples);

    let bubble_dead_volume = bubble_dead_volume_window_panel();
    export(OUTPUTS[6], &bubble_dead_volume);

    let standards = reference_standard_custody_lands();
    export(OUTPUTS[7], &standards);

    let tokens = timed_residence_loop_token_rail();
    export(OUTPUTS[8], &tokens);

    let waste = waste_flush_capture_caddy();
    export(OUTPUTS[9], &waste);

    let traceability = batch_traceability_lands();
    export(OUTPUTS[10], &traceability);

    let disposition = release_hold_reject_disposition_gate();
    export(OUTPUTS[11], &disposition);

    let camera = camera_illumination_fiducial_bridge();
    export(OUTPUTS[12], &camera);

    let assembly = deck
        + handoff
        + gas
        + probes
        + temperature
        + samples
        + bubble_dead_volume
        + standards
        + tokens
        + waste
        + traceability
        + disposition
        + camera
        + route_and_timing_overlay();
    export(OUTPUTS[13], &assembly);

    println!();
    println!("Closed inline media pH/osmolality CO2-lag validation station:");
    println!("  Revision/units:         {PARAMETRIC_REVISION} / {UNITS}");
    println!("  Footprint:              {STATION_X:.0}mm x {STATION_Y:.0}mm contained deck");
    println!(
        "  Scaled perfusion:       {CHIP_COUNT} chip positions in {CHIP_ROWS} media groups, {CHIPS_PER_GROUP} chips per group"
    );
    println!(
        "  Chemistry sampling:     {PH_SAMPLE_WELLS} pH wells, {OSMOLALITY_SAMPLE_WELLS} osmolality wells, {SPLIT_RETAIN_WELLS} split/retain wells"
    );
    println!(
        "  Gas and temperature:    {GAS_LOOP_LANES} gas-lag loop lanes, {CO2_PROBE_DOCKS} CO2 docks, {O2_PROBE_DOCKS} O2 docks, {TEMPERATURE_ZONES} thermal zones"
    );
    println!(
        "  Witness features:       {BUBBLE_WINDOWS} bubble windows, {DEAD_VOLUME_WINDOWS} dead-volume windows, {RESIDENCE_TOKENS} timed residence tokens"
    );
    println!(
        "  Traceability/release:   {REFERENCE_STANDARD_LANDS} reference standard lands, {WASTE_CAPTURE_CUPS} waste cups, {DISPOSITION_TOKEN_WELLS} disposition wells"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn deck_top_z() -> f64 {
    BASE_Z
}

fn place_z(height: f64) -> f64 {
    deck_top_z() + height / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn group_x(index: usize, pitch: f64) -> f64 {
    centered_index(index, PERFUSION_GROUPS, pitch)
}

fn footprint(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Footprint {
    Footprint { name, center, x, y }
}

fn module_footprints() -> [Footprint; 12] {
    [
        footprint(
            "scaled_multichip_perfusion_handoff_bulkhead",
            HANDOFF_POS,
            HANDOFF_X,
            HANDOFF_Y,
        ),
        footprint(
            "gas_equilibration_lag_loop_surrogate",
            GAS_POS,
            GAS_X,
            GAS_Y,
        ),
        footprint("co2_o2_probe_dock_bridge", PROBE_POS, PROBE_X, PROBE_Y),
        footprint("temperature_equilibration_block", TEMP_POS, TEMP_X, TEMP_Y),
        footprint(
            "ph_osmolality_sample_split_retain_wells",
            SAMPLE_POS,
            SAMPLE_X,
            SAMPLE_Y,
        ),
        footprint(
            "bubble_dead_volume_window_panel",
            BUBBLE_POS,
            BUBBLE_X,
            BUBBLE_Y,
        ),
        footprint(
            "reference_standard_custody_lands",
            STANDARD_POS,
            STANDARD_X,
            STANDARD_Y,
        ),
        footprint("waste_flush_capture_caddy", WASTE_POS, WASTE_X, WASTE_Y),
        footprint(
            "timed_residence_loop_token_rail",
            TOKEN_POS,
            TOKEN_X,
            TOKEN_Y,
        ),
        footprint(
            "release_hold_reject_disposition_gate",
            DISPOSITION_POS,
            DISPOSITION_X,
            DISPOSITION_Y,
        ),
        footprint("batch_traceability_lands", TRACE_POS, TRACE_X, TRACE_Y),
        footprint(
            "camera_illumination_fiducial_bridge",
            CAMERA_POS,
            CAMERA_X,
            CAMERA_Y,
        ),
    ]
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        "inline_ph_osmo_co2_lag_containment_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let sump = centered_cube(
        "inline_ph_osmo_co2_lag_low_sump_recess",
        STATION_X - 148.0,
        STATION_Y - 138.0,
        SUMP_DEPTH,
    )
    .translate(0.0, -8.0, deck_top_z() - SUMP_DEPTH / 2.0 + 0.2);
    let drain = centered_cylinder(
        "inline_ph_osmo_co2_lag_front_waste_flush_drain",
        DRAIN_D / 2.0,
        80.0,
        CYLINDER_SEGMENTS,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 92.0,
        -STATION_Y / 2.0 + 10.0,
        deck_top_z() - 5.0,
    );

    deck - sump - drain - module_insert_sockets() - deck_mount_holes()
        + perimeter_rims()
        + deck_datum_targets()
        + deck_zone_locator_rails()
}

fn module_insert_sockets() -> Part {
    let mut sockets = Part::empty("inline_ph_osmo_co2_lag_module_insert_sockets");
    for module in module_footprints() {
        sockets = sockets
            + centered_cube(
                format!("inline_ph_osmo_co2_lag_{}_socket", module.name),
                module.x + 12.0,
                module.y + 12.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(
                module.center.0,
                module.center.1,
                deck_top_z() - SOCKET_DEPTH / 2.0 + 0.2,
            );
    }
    sockets
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("inline_ph_osmo_co2_lag_deck_mount_holes");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 62.0, -STATION_Y / 2.0 + 58.0),
        (STATION_X / 2.0 - 62.0, -STATION_Y / 2.0 + 58.0),
        (-STATION_X / 2.0 + 62.0, STATION_Y / 2.0 - 58.0),
        (STATION_X / 2.0 - 62.0, STATION_Y / 2.0 - 58.0),
        (-390.0, 160.0),
        (390.0, 160.0),
        (-390.0, -170.0),
        (390.0, -170.0),
    ]
    .into_iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("inline_ph_osmo_co2_lag_mount_hole_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 4.0,
                24,
            )
            .translate(x, y, BASE_Z / 2.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    let left = centered_cube(
        "inline_ph_osmo_co2_lag_left_spill_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(
        -STATION_X / 2.0 + RIM_W / 2.0,
        0.0,
        deck_top_z() + RIM_Z / 2.0,
    );
    let right = centered_cube(
        "inline_ph_osmo_co2_lag_right_spill_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        deck_top_z() + RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "inline_ph_osmo_co2_lag_rear_isolator_service_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        deck_top_z() + RIM_Z / 2.0,
    );
    let front = centered_cube(
        "inline_ph_osmo_co2_lag_low_robot_access_lip",
        STATION_X - 260.0,
        12.0,
        20.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 18.0, deck_top_z() + 10.0);
    left + right + rear + front
}

fn deck_datum_targets() -> Part {
    let mut targets = Part::empty("inline_ph_osmo_co2_lag_deck_datum_targets");
    for (i, (x, y)) in [
        (-770.0, 432.0),
        (-260.0, 432.0),
        (260.0, 432.0),
        (770.0, 432.0),
        (-770.0, -430.0),
        (-260.0, -430.0),
        (260.0, -430.0),
        (770.0, -430.0),
    ]
    .into_iter()
    .enumerate()
    {
        let target = centered_cylinder(
            format!("inline_ph_osmo_co2_lag_robot_datum_outer_{i}"),
            12.0,
            4.0,
            FIDUCIAL_SEGMENTS,
        )
        .translate(x, y, deck_top_z() + 2.0)
            - centered_cylinder(
                format!("inline_ph_osmo_co2_lag_robot_datum_inner_{i}"),
                4.2,
                6.0,
                24,
            )
            .translate(x, y, deck_top_z() + 2.0);
        targets = targets + target;
    }
    targets
}

fn deck_zone_locator_rails() -> Part {
    let top = centered_cube(
        "inline_ph_osmo_co2_lag_top_media_conditioning_zone_rail",
        STATION_X - 260.0,
        9.0,
        16.0,
    )
    .translate(0.0, 160.0, deck_top_z() + 8.0);
    let middle = centered_cube(
        "inline_ph_osmo_co2_lag_sampling_witness_zone_rail",
        STATION_X - 260.0,
        9.0,
        16.0,
    )
    .translate(0.0, -106.0, deck_top_z() + 8.0);
    let trace = centered_cube(
        "inline_ph_osmo_co2_lag_traceability_release_zone_rail",
        STATION_X - 360.0,
        9.0,
        16.0,
    )
    .translate(0.0, -386.0, deck_top_z() + 8.0);
    top + middle + trace
}

fn scaled_multichip_perfusion_handoff_bulkhead() -> Part {
    let body = centered_cube(
        "inline_ph_osmo_co2_lag_scaled_perfusion_handoff_body",
        HANDOFF_X,
        HANDOFF_Y,
        HANDOFF_Z,
    )
    .translate(HANDOFF_POS.0, HANDOFF_POS.1, place_z(HANDOFF_Z));
    body - handoff_port_bores()
        + handoff_port_rims()
        + chip_batch_map_lands()
        + media_group_manifold_rails()
        + handoff_keyed_connector_tabs()
}

fn handoff_port_bores() -> Part {
    let mut bores = Part::empty("inline_ph_osmo_co2_lag_handoff_port_bores");
    for group in 0..PERFUSION_GROUPS {
        for side in 0..2 {
            bores = bores
                + centered_cylinder(
                    format!("inline_ph_osmo_co2_lag_group_{group}_handoff_port_{side}_bore"),
                    HANDOFF_PORT_D / 2.0,
                    HANDOFF_Y + 16.0,
                    24,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(
                    HANDOFF_POS.0 + group_x(group, 46.0),
                    HANDOFF_POS.1 + centered_index(side, 2, 74.0),
                    deck_top_z() + HANDOFF_Z / 2.0,
                );
        }
    }
    bores
}

fn handoff_port_rims() -> Part {
    let mut rims = Part::empty("inline_ph_osmo_co2_lag_handoff_port_rims");
    for group in 0..PERFUSION_GROUPS {
        for side in 0..2 {
            rims = rims
                + centered_cylinder(
                    format!("inline_ph_osmo_co2_lag_group_{group}_handoff_port_{side}_rim"),
                    13.0,
                    5.0,
                    CYLINDER_SEGMENTS,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(
                    HANDOFF_POS.0 + group_x(group, 46.0),
                    HANDOFF_POS.1 + centered_index(side, 2, 74.0),
                    deck_top_z() + HANDOFF_Z / 2.0,
                );
        }
    }
    rims
}

fn chip_batch_map_lands() -> Part {
    let mut map = Part::empty("inline_ph_osmo_co2_lag_24_chip_batch_map_lands");
    for row in 0..CHIP_ROWS {
        for col in 0..CHIP_COLUMNS {
            let index = row * CHIP_COLUMNS + col;
            map = map
                + centered_cube(
                    format!("inline_ph_osmo_co2_lag_chip_{index}_batch_map_land"),
                    CHIP_MAP_CELL_X,
                    CHIP_MAP_CELL_Y,
                    5.0,
                )
                .translate(
                    HANDOFF_POS.0 + centered_index(col, CHIP_COLUMNS, 38.0),
                    HANDOFF_POS.1 - 4.0 + centered_index(row, CHIP_ROWS, 25.0),
                    deck_top_z() + HANDOFF_Z + 2.5,
                );
        }
    }
    map
}

fn media_group_manifold_rails() -> Part {
    let mut rails = Part::empty("inline_ph_osmo_co2_lag_media_group_manifold_rails");
    for group in 0..PERFUSION_GROUPS {
        rails = rails
            + centered_cube(
                format!("inline_ph_osmo_co2_lag_group_{group}_media_manifold_rail"),
                34.0,
                HANDOFF_Y - 38.0,
                7.0,
            )
            .translate(
                HANDOFF_POS.0 + group_x(group, 46.0),
                HANDOFF_POS.1,
                deck_top_z() + HANDOFF_Z + 8.5,
            );
    }
    rails
}

fn handoff_keyed_connector_tabs() -> Part {
    let mut tabs = Part::empty("inline_ph_osmo_co2_lag_handoff_keyed_connector_tabs");
    for port in 0..HANDOFF_PORTS {
        let group = port / 2;
        let side = port % 2;
        tabs = tabs
            + centered_cube(
                format!("inline_ph_osmo_co2_lag_handoff_port_{port}_key_flat"),
                24.0,
                7.0,
                7.0,
            )
            .translate(
                HANDOFF_POS.0 + group_x(group, 46.0),
                HANDOFF_POS.1 + centered_index(side, 2, 74.0) - 18.0,
                deck_top_z() + HANDOFF_Z + 6.0,
            );
    }
    tabs
}

fn gas_equilibration_lag_loop_surrogate() -> Part {
    let body = centered_cube(
        "inline_ph_osmo_co2_lag_gas_equilibration_loop_body",
        GAS_X,
        GAS_Y,
        GAS_Z,
    )
    .translate(GAS_POS.0, GAS_POS.1, place_z(GAS_Z));
    body - gas_loop_port_bores() - gas_lag_window_cuts()
        + gas_loop_port_rims()
        + visible_serpentine_hold_route()
        + gas_membrane_window_frames()
        + lag_gradient_reference_steps()
}

fn gas_loop_port_bores() -> Part {
    let mut bores = Part::empty("inline_ph_osmo_co2_lag_gas_loop_port_bores");
    for lane in 0..GAS_LOOP_LANES {
        for side in 0..2 {
            bores = bores
                + centered_cylinder(
                    format!("inline_ph_osmo_co2_lag_gas_loop_lane_{lane}_port_{side}_bore"),
                    GAS_PORT_D / 2.0,
                    GAS_Y + 14.0,
                    24,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(
                    GAS_POS.0 + group_x(lane, 48.0),
                    GAS_POS.1 + centered_index(side, 2, 76.0),
                    deck_top_z() + GAS_Z / 2.0,
                );
        }
    }
    bores
}

fn gas_loop_port_rims() -> Part {
    let mut rims = Part::empty("inline_ph_osmo_co2_lag_gas_loop_port_rims");
    for lane in 0..GAS_LOOP_LANES {
        for side in 0..2 {
            rims = rims
                + centered_cylinder(
                    format!("inline_ph_osmo_co2_lag_gas_loop_lane_{lane}_port_{side}_rim"),
                    11.0,
                    5.0,
                    CYLINDER_SEGMENTS,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(
                    GAS_POS.0 + group_x(lane, 48.0),
                    GAS_POS.1 + centered_index(side, 2, 76.0),
                    deck_top_z() + GAS_Z / 2.0,
                );
        }
    }
    rims
}

fn gas_lag_window_cuts() -> Part {
    let mut cuts = Part::empty("inline_ph_osmo_co2_lag_gas_lag_window_cuts");
    for lane in 0..GAS_LAG_WINDOWS {
        cuts = cuts
            + centered_cube(
                format!("inline_ph_osmo_co2_lag_lane_{lane}_co2_lag_window_cut"),
                31.0,
                50.0,
                GAS_Z + 10.0,
            )
            .translate(GAS_POS.0 + group_x(lane, 48.0), GAS_POS.1, place_z(GAS_Z));
    }
    cuts
}

fn visible_serpentine_hold_route() -> Part {
    let mut route = Part::empty("inline_ph_osmo_co2_lag_serpentine_hold_route");
    for segment in 0..8 {
        route = route
            + centered_cube(
                format!("inline_ph_osmo_co2_lag_serpentine_route_segment_{segment}"),
                if segment % 2 == 0 {
                    GAS_X - 72.0
                } else {
                    GAS_X - 126.0
                },
                7.0,
                7.0,
            )
            .translate(
                GAS_POS.0,
                GAS_POS.1 + centered_index(segment, 8, 18.0),
                deck_top_z() + GAS_Z + 4.5,
            );
    }
    for turn in 0..7 {
        route = route
            + centered_cylinder(
                format!("inline_ph_osmo_co2_lag_serpentine_turn_marker_{turn}"),
                10.0,
                7.0,
                CYLINDER_SEGMENTS,
            )
            .translate(
                GAS_POS.0
                    + if turn % 2 == 0 {
                        GAS_X / 2.0 - 52.0
                    } else {
                        -GAS_X / 2.0 + 52.0
                    },
                GAS_POS.1 + centered_index(turn, 7, 18.0),
                deck_top_z() + GAS_Z + 4.5,
            );
    }
    route
}

fn gas_membrane_window_frames() -> Part {
    let mut frames = Part::empty("inline_ph_osmo_co2_lag_gas_membrane_window_frames");
    for lane in 0..GAS_MEMBRANE_WINDOWS {
        frames = frames
            + rectangular_frame(
                &format!("inline_ph_osmo_co2_lag_lane_{lane}_membrane_frame"),
                38.0,
                58.0,
                4.0,
                6.0,
            )
            .translate(
                GAS_POS.0 + group_x(lane, 48.0),
                GAS_POS.1,
                deck_top_z() + GAS_Z + 4.0,
            );
    }
    frames
}

fn lag_gradient_reference_steps() -> Part {
    let mut steps = Part::empty("inline_ph_osmo_co2_lag_reference_gradient_steps");
    for lane in 0..GAS_LOOP_LANES {
        for step in 0..RESIDENCE_LEVELS {
            steps = steps
                + centered_cube(
                    format!("inline_ph_osmo_co2_lag_lane_{lane}_residence_step_{step}"),
                    24.0,
                    8.0,
                    3.0 + step as f64 * 1.5,
                )
                .translate(
                    GAS_POS.0 + group_x(lane, 48.0),
                    GAS_POS.1 - GAS_Y / 2.0 + 28.0 + step as f64 * 14.0,
                    deck_top_z() + GAS_Z + 3.0,
                );
        }
    }
    steps
}

fn co2_o2_probe_dock_bridge() -> Part {
    let body = centered_cube(
        "inline_ph_osmo_co2_lag_co2_o2_probe_bridge_body",
        PROBE_X,
        PROBE_Y,
        PROBE_Z,
    )
    .translate(PROBE_POS.0, PROBE_POS.1, place_z(PROBE_Z));
    body - probe_dock_bores()
        + probe_dock_rims()
        + probe_cable_strain_reliefs()
        + co2_o2_comparator_windows()
        + probe_identity_key_lands()
}

fn probe_dock_bores() -> Part {
    let mut bores = Part::empty("inline_ph_osmo_co2_lag_probe_dock_bores");
    for gas_row in 0..2 {
        for group in 0..PERFUSION_GROUPS {
            bores = bores
                + centered_cylinder(
                    format!("inline_ph_osmo_co2_lag_probe_row_{gas_row}_group_{group}_dock_bore"),
                    PROBE_DOCK_D / 2.0,
                    PROBE_Z + 12.0,
                    CYLINDER_SEGMENTS,
                )
                .translate(
                    PROBE_POS.0 + group_x(group, 48.0),
                    PROBE_POS.1 + centered_index(gas_row, 2, 60.0),
                    place_z(PROBE_Z),
                );
        }
    }
    bores
}

fn probe_dock_rims() -> Part {
    let mut rims = Part::empty("inline_ph_osmo_co2_lag_probe_dock_rims");
    for gas_row in 0..2 {
        for group in 0..PERFUSION_GROUPS {
            let label = if gas_row == 0 { "co2" } else { "o2" };
            let x = PROBE_POS.0 + group_x(group, 48.0);
            let y = PROBE_POS.1 + centered_index(gas_row, 2, 60.0);
            let rim = centered_cylinder(
                format!("inline_ph_osmo_co2_lag_{label}_group_{group}_probe_rim_outer"),
                PROBE_DOCK_D / 2.0 + 6.0,
                5.0,
                CYLINDER_SEGMENTS,
            )
            .translate(x, y, deck_top_z() + PROBE_Z + 2.5)
                - centered_cylinder(
                    format!("inline_ph_osmo_co2_lag_{label}_group_{group}_probe_rim_inner"),
                    PROBE_DOCK_D / 2.0,
                    7.0,
                    CYLINDER_SEGMENTS,
                )
                .translate(x, y, deck_top_z() + PROBE_Z + 2.5);
            rims = rims + rim;
        }
    }
    rims
}

fn probe_cable_strain_reliefs() -> Part {
    let mut clamps = Part::empty("inline_ph_osmo_co2_lag_probe_cable_strain_reliefs");
    for clamp in 0..PROBE_CABLE_CLAMPS {
        let row = clamp / PERFUSION_GROUPS;
        let group = clamp % PERFUSION_GROUPS;
        clamps = clamps
            + centered_cube(
                format!("inline_ph_osmo_co2_lag_probe_cable_clamp_{clamp}"),
                28.0,
                9.0,
                9.0,
            )
            .translate(
                PROBE_POS.0 + group_x(group, 48.0),
                PROBE_POS.1 + centered_index(row, 2, 60.0) + 24.0,
                deck_top_z() + PROBE_Z + 8.0,
            );
    }
    clamps
}

fn co2_o2_comparator_windows() -> Part {
    let mut windows = Part::empty("inline_ph_osmo_co2_lag_co2_o2_comparator_windows");
    for group in 0..PERFUSION_GROUPS {
        windows = windows
            + rectangular_frame(
                &format!("inline_ph_osmo_co2_lag_group_{group}_co2_o2_comparator_window"),
                34.0,
                24.0,
                4.0,
                5.0,
            )
            .translate(
                PROBE_POS.0 + group_x(group, 48.0),
                PROBE_POS.1,
                deck_top_z() + PROBE_Z + 4.0,
            );
    }
    windows
}

fn probe_identity_key_lands() -> Part {
    let co2_land = centered_cube(
        "inline_ph_osmo_co2_lag_co2_probe_identity_land",
        PROBE_X - 62.0,
        8.0,
        5.0,
    )
    .translate(
        PROBE_POS.0,
        PROBE_POS.1 - 58.0,
        deck_top_z() + PROBE_Z + 7.0,
    );
    let o2_land = centered_cube(
        "inline_ph_osmo_co2_lag_o2_probe_identity_land",
        PROBE_X - 62.0,
        8.0,
        5.0,
    )
    .translate(
        PROBE_POS.0,
        PROBE_POS.1 + 58.0,
        deck_top_z() + PROBE_Z + 7.0,
    );
    co2_land + o2_land
}

fn temperature_equilibration_block() -> Part {
    let body = centered_cube(
        "inline_ph_osmo_co2_lag_temperature_equilibration_body",
        TEMP_X,
        TEMP_Y,
        TEMP_Z,
    )
    .translate(TEMP_POS.0, TEMP_POS.1, place_z(TEMP_Z));
    body - temperature_reference_well_cuts()
        + temperature_reference_well_rims()
        + thermal_mass_ribs()
        + temperature_probe_bridge()
        + equilibrated_media_lane_pads()
}

fn temperature_reference_well_cuts() -> Part {
    let mut cuts = Part::empty("inline_ph_osmo_co2_lag_temperature_reference_well_cuts");
    for well in 0..TEMP_REFERENCE_WELLS {
        cuts = cuts
            + centered_cylinder(
                format!("inline_ph_osmo_co2_lag_temperature_group_{well}_reference_well_cut"),
                9.0,
                TEMP_Z + 12.0,
                CYLINDER_SEGMENTS,
            )
            .translate(
                TEMP_POS.0 + group_x(well, 42.0),
                TEMP_POS.1 + 28.0,
                place_z(TEMP_Z),
            );
    }
    cuts
}

fn temperature_reference_well_rims() -> Part {
    let mut rims = Part::empty("inline_ph_osmo_co2_lag_temperature_reference_well_rims");
    for well in 0..TEMP_REFERENCE_WELLS {
        let x = TEMP_POS.0 + group_x(well, 42.0);
        let y = TEMP_POS.1 + 28.0;
        let rim = centered_cylinder(
            format!("inline_ph_osmo_co2_lag_temperature_group_{well}_well_rim_outer"),
            14.0,
            5.0,
            CYLINDER_SEGMENTS,
        )
        .translate(x, y, deck_top_z() + TEMP_Z + 2.5)
            - centered_cylinder(
                format!("inline_ph_osmo_co2_lag_temperature_group_{well}_well_rim_inner"),
                9.0,
                7.0,
                CYLINDER_SEGMENTS,
            )
            .translate(x, y, deck_top_z() + TEMP_Z + 2.5);
        rims = rims + rim;
    }
    rims
}

fn thermal_mass_ribs() -> Part {
    let mut ribs = Part::empty("inline_ph_osmo_co2_lag_thermal_mass_ribs");
    for rib in 0..THERMAL_MASS_RIBS {
        ribs = ribs
            + centered_cube(
                format!("inline_ph_osmo_co2_lag_thermal_mass_rib_{rib}"),
                9.0,
                TEMP_Y - 42.0,
                8.0,
            )
            .translate(
                TEMP_POS.0 + centered_index(rib, THERMAL_MASS_RIBS, 44.0),
                TEMP_POS.1,
                deck_top_z() + TEMP_Z + 6.0,
            );
    }
    ribs
}

fn temperature_probe_bridge() -> Part {
    let bridge = centered_cube(
        "inline_ph_osmo_co2_lag_temperature_probe_bridge_socket",
        TEMP_X - 68.0,
        20.0,
        14.0,
    )
    .translate(
        TEMP_POS.0,
        TEMP_POS.1 - TEMP_Y / 2.0 + 34.0,
        deck_top_z() + TEMP_Z + 7.0,
    );
    let cable = centered_cube(
        "inline_ph_osmo_co2_lag_temperature_probe_cable_channel",
        22.0,
        TEMP_Y - 64.0,
        8.0,
    )
    .translate(TEMP_POS.0, TEMP_POS.1, deck_top_z() + TEMP_Z + 6.0);
    bridge + cable
}

fn equilibrated_media_lane_pads() -> Part {
    let mut pads = Part::empty("inline_ph_osmo_co2_lag_equilibrated_media_lane_pads");
    for zone in 0..TEMPERATURE_ZONES {
        pads = pads
            + centered_cube(
                format!("inline_ph_osmo_co2_lag_temperature_zone_{zone}_media_lane_pad"),
                34.0,
                20.0,
                5.0,
            )
            .translate(
                TEMP_POS.0 + group_x(zone, 42.0),
                TEMP_POS.1 - 28.0,
                deck_top_z() + TEMP_Z + 4.0,
            );
    }
    pads
}

fn ph_osmolality_sample_split_retain_wells() -> Part {
    let body = centered_cube(
        "inline_ph_osmo_co2_lag_sample_split_retain_body",
        SAMPLE_X,
        SAMPLE_Y,
        SAMPLE_Z,
    )
    .translate(SAMPLE_POS.0, SAMPLE_POS.1, place_z(SAMPLE_Z));
    body - sample_well_cuts() - sample_selector_valve_bores()
        + sample_well_rims()
        + split_retain_well_rims()
        + sample_selector_valve_blocks()
        + analyzer_handoff_lands()
}

fn sample_well_cuts() -> Part {
    let mut cuts = Part::empty("inline_ph_osmo_co2_lag_sample_well_cuts");
    for group in 0..PERFUSION_GROUPS {
        for replicate in 0..SAMPLE_REPLICATES {
            cuts = cuts
                + centered_cylinder(
                    format!("inline_ph_osmo_co2_lag_group_{group}_ph_replicate_{replicate}_well_cut"),
                    SAMPLE_WELL_D / 2.0,
                    SAMPLE_Z + 10.0,
                    24,
                )
                .translate(
                    SAMPLE_POS.0 + group_x(group, 44.0) + centered_index(replicate, SAMPLE_REPLICATES, 13.0),
                    SAMPLE_POS.1 + 55.0,
                    place_z(SAMPLE_Z),
                )
                + centered_cylinder(
                    format!("inline_ph_osmo_co2_lag_group_{group}_osmolality_replicate_{replicate}_well_cut"),
                    SAMPLE_WELL_D / 2.0,
                    SAMPLE_Z + 10.0,
                    24,
                )
                .translate(
                    SAMPLE_POS.0 + group_x(group, 44.0) + centered_index(replicate, SAMPLE_REPLICATES, 13.0),
                    SAMPLE_POS.1 + 18.0,
                    place_z(SAMPLE_Z),
                );
        }
        for split in 0..2 {
            cuts = cuts
                + centered_cylinder(
                    format!("inline_ph_osmo_co2_lag_group_{group}_split_retain_{split}_well_cut"),
                    RETAIN_WELL_D / 2.0,
                    SAMPLE_Z + 10.0,
                    24,
                )
                .translate(
                    SAMPLE_POS.0 + group_x(group, 44.0),
                    SAMPLE_POS.1 - 46.0 + centered_index(split, 2, 30.0),
                    place_z(SAMPLE_Z),
                );
        }
    }
    cuts
}

fn sample_well_rims() -> Part {
    let mut rims = Part::empty("inline_ph_osmo_co2_lag_ph_osmolality_sample_well_rims");
    for group in 0..PERFUSION_GROUPS {
        for replicate in 0..SAMPLE_REPLICATES {
            for analyte in 0..2 {
                let analyte_name = if analyte == 0 { "ph" } else { "osmolality" };
                let x = SAMPLE_POS.0
                    + group_x(group, 44.0)
                    + centered_index(replicate, SAMPLE_REPLICATES, 13.0);
                let y = SAMPLE_POS.1 + if analyte == 0 { 55.0 } else { 18.0 };
                let rim = centered_cylinder(
                    format!("inline_ph_osmo_co2_lag_group_{group}_{analyte_name}_replicate_{replicate}_rim_outer"),
                    SAMPLE_WELL_D / 2.0 + 4.0,
                    4.0,
                    24,
                )
                .translate(x, y, deck_top_z() + SAMPLE_Z + 2.0)
                    - centered_cylinder(
                        format!("inline_ph_osmo_co2_lag_group_{group}_{analyte_name}_replicate_{replicate}_rim_inner"),
                        SAMPLE_WELL_D / 2.0,
                        6.0,
                        24,
                    )
                    .translate(x, y, deck_top_z() + SAMPLE_Z + 2.0);
                rims = rims + rim;
            }
        }
    }
    rims
}

fn split_retain_well_rims() -> Part {
    let mut rims = Part::empty("inline_ph_osmo_co2_lag_split_retain_well_rims");
    for group in 0..PERFUSION_GROUPS {
        for split in 0..2 {
            let x = SAMPLE_POS.0 + group_x(group, 44.0);
            let y = SAMPLE_POS.1 - 46.0 + centered_index(split, 2, 30.0);
            let rim = centered_cylinder(
                format!("inline_ph_osmo_co2_lag_group_{group}_split_retain_{split}_rim_outer"),
                RETAIN_WELL_D / 2.0 + 4.5,
                4.0,
                24,
            )
            .translate(x, y, deck_top_z() + SAMPLE_Z + 2.0)
                - centered_cylinder(
                    format!("inline_ph_osmo_co2_lag_group_{group}_split_retain_{split}_rim_inner"),
                    RETAIN_WELL_D / 2.0,
                    6.0,
                    24,
                )
                .translate(x, y, deck_top_z() + SAMPLE_Z + 2.0);
            rims = rims + rim;
        }
    }
    rims
}

fn sample_selector_valve_bores() -> Part {
    let mut bores = Part::empty("inline_ph_osmo_co2_lag_sample_selector_valve_bores");
    for valve in 0..SAMPLE_SELECTOR_VALVES {
        bores = bores
            + centered_cylinder(
                format!("inline_ph_osmo_co2_lag_group_{valve}_sample_selector_cross_bore"),
                3.6,
                SAMPLE_Y + 14.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                SAMPLE_POS.0 + group_x(valve, 44.0),
                SAMPLE_POS.1 - SAMPLE_Y / 2.0 + 28.0,
                deck_top_z() + 29.0,
            );
    }
    bores
}

fn sample_selector_valve_blocks() -> Part {
    let mut blocks = Part::empty("inline_ph_osmo_co2_lag_sample_selector_valve_blocks");
    for valve in 0..SAMPLE_SELECTOR_VALVES {
        blocks = blocks
            + centered_cube(
                format!("inline_ph_osmo_co2_lag_group_{valve}_sample_selector_block"),
                28.0,
                22.0,
                18.0,
            )
            .translate(
                SAMPLE_POS.0 + group_x(valve, 44.0),
                SAMPLE_POS.1 - SAMPLE_Y / 2.0 + 28.0,
                deck_top_z() + SAMPLE_Z + 9.0,
            );
    }
    blocks
}

fn analyzer_handoff_lands() -> Part {
    let ph = rectangular_frame(
        "inline_ph_osmo_co2_lag_ph_analyzer_handoff_land",
        138.0,
        28.0,
        4.0,
        5.0,
    )
    .translate(
        SAMPLE_POS.0 - 80.0,
        SAMPLE_POS.1 + SAMPLE_Y / 2.0 - 26.0,
        deck_top_z() + SAMPLE_Z + 3.5,
    );
    let osmo = rectangular_frame(
        "inline_ph_osmo_co2_lag_osmolality_analyzer_handoff_land",
        150.0,
        28.0,
        4.0,
        5.0,
    )
    .translate(
        SAMPLE_POS.0 + 88.0,
        SAMPLE_POS.1 + SAMPLE_Y / 2.0 - 26.0,
        deck_top_z() + SAMPLE_Z + 3.5,
    );
    ph + osmo
}

fn bubble_dead_volume_window_panel() -> Part {
    let body = centered_cube(
        "inline_ph_osmo_co2_lag_bubble_dead_volume_window_body",
        BUBBLE_X,
        BUBBLE_Y,
        BUBBLE_Z,
    )
    .translate(BUBBLE_POS.0, BUBBLE_POS.1, place_z(BUBBLE_Z));
    body - bubble_window_cuts() - dead_volume_window_cuts()
        + bubble_window_frames()
        + dead_volume_step_ladders()
        + high_point_bubble_capture_cups()
        + dead_leg_route_shadow_bars()
}

fn bubble_window_cuts() -> Part {
    let mut cuts = Part::empty("inline_ph_osmo_co2_lag_bubble_window_cuts");
    for window in 0..BUBBLE_WINDOWS {
        cuts = cuts
            + centered_cube(
                format!("inline_ph_osmo_co2_lag_bubble_window_{window}_cut"),
                BUBBLE_WINDOW_X,
                BUBBLE_WINDOW_Y,
                BUBBLE_Z + 12.0,
            )
            .translate(
                BUBBLE_POS.0 + group_x(window, 48.0),
                BUBBLE_POS.1 + 40.0,
                place_z(BUBBLE_Z),
            );
    }
    cuts
}

fn dead_volume_window_cuts() -> Part {
    let mut cuts = Part::empty("inline_ph_osmo_co2_lag_dead_volume_window_cuts");
    for window in 0..DEAD_VOLUME_WINDOWS {
        cuts = cuts
            + centered_cube(
                format!("inline_ph_osmo_co2_lag_dead_volume_window_{window}_cut"),
                BUBBLE_WINDOW_X,
                45.0,
                BUBBLE_Z + 12.0,
            )
            .translate(
                BUBBLE_POS.0 + group_x(window, 48.0),
                BUBBLE_POS.1 - 42.0,
                place_z(BUBBLE_Z),
            );
    }
    cuts
}

fn bubble_window_frames() -> Part {
    let mut frames = Part::empty("inline_ph_osmo_co2_lag_bubble_window_frames");
    for window in 0..BUBBLE_WINDOWS {
        frames = frames
            + rectangular_frame(
                &format!("inline_ph_osmo_co2_lag_bubble_window_{window}_frame"),
                BUBBLE_WINDOW_X + 12.0,
                BUBBLE_WINDOW_Y + 12.0,
                4.0,
                6.0,
            )
            .translate(
                BUBBLE_POS.0 + group_x(window, 48.0),
                BUBBLE_POS.1 + 40.0,
                deck_top_z() + BUBBLE_Z + 4.0,
            );
    }
    frames
}

fn dead_volume_step_ladders() -> Part {
    let mut ladders = Part::empty("inline_ph_osmo_co2_lag_dead_volume_step_ladders");
    for window in 0..DEAD_VOLUME_WINDOWS {
        for step in 0..DEAD_VOLUME_STEPS_PER_WINDOW {
            ladders = ladders
                + centered_cube(
                    format!("inline_ph_osmo_co2_lag_dead_volume_window_{window}_step_{step}"),
                    28.0,
                    5.0,
                    3.0 + step as f64,
                )
                .translate(
                    BUBBLE_POS.0 + group_x(window, 48.0),
                    BUBBLE_POS.1 - 60.0 + step as f64 * 12.0,
                    deck_top_z() + BUBBLE_Z + 3.0,
                );
        }
    }
    ladders
}

fn high_point_bubble_capture_cups() -> Part {
    let mut cups = Part::empty("inline_ph_osmo_co2_lag_high_point_bubble_capture_cups");
    for cup in 0..BUBBLE_WINDOWS {
        let x = BUBBLE_POS.0 + group_x(cup, 48.0);
        let capture = centered_cylinder(
            format!("inline_ph_osmo_co2_lag_bubble_capture_cup_{cup}_outer"),
            12.0,
            10.0,
            CYLINDER_SEGMENTS,
        )
        .translate(
            x,
            BUBBLE_POS.1 + BUBBLE_Y / 2.0 - 24.0,
            deck_top_z() + BUBBLE_Z + 10.0,
        ) - centered_cylinder(
            format!("inline_ph_osmo_co2_lag_bubble_capture_cup_{cup}_inner"),
            7.5,
            12.0,
            24,
        )
        .translate(
            x,
            BUBBLE_POS.1 + BUBBLE_Y / 2.0 - 24.0,
            deck_top_z() + BUBBLE_Z + 10.0,
        );
        cups = cups + capture;
    }
    cups
}

fn dead_leg_route_shadow_bars() -> Part {
    let upper = centered_cube(
        "inline_ph_osmo_co2_lag_bubble_path_route_shadow_bar",
        BUBBLE_X - 58.0,
        8.0,
        7.0,
    )
    .translate(
        BUBBLE_POS.0,
        BUBBLE_POS.1 + 82.0,
        deck_top_z() + BUBBLE_Z + 5.0,
    );
    let lower = centered_cube(
        "inline_ph_osmo_co2_lag_dead_volume_path_route_shadow_bar",
        BUBBLE_X - 58.0,
        8.0,
        7.0,
    )
    .translate(
        BUBBLE_POS.0,
        BUBBLE_POS.1 - 82.0,
        deck_top_z() + BUBBLE_Z + 5.0,
    );
    upper + lower
}

fn reference_standard_custody_lands() -> Part {
    let body = centered_cube(
        "inline_ph_osmo_co2_lag_reference_standard_custody_body",
        STANDARD_X,
        STANDARD_Y,
        STANDARD_Z,
    )
    .translate(STANDARD_POS.0, STANDARD_POS.1, place_z(STANDARD_Z));
    body - reference_standard_vial_cuts()
        + reference_standard_vial_rims()
        + custody_seal_rings()
        + standard_family_label_lands()
        + custody_chain_rail()
}

fn reference_standard_vial_cuts() -> Part {
    let mut cuts = Part::empty("inline_ph_osmo_co2_lag_reference_standard_vial_cuts");
    for standard in 0..REFERENCE_STANDARD_LANDS {
        let row = standard / 4;
        let col = standard % 4;
        cuts = cuts
            + centered_cylinder(
                format!("inline_ph_osmo_co2_lag_reference_standard_{standard}_vial_cut"),
                STANDARD_VIAL_D / 2.0,
                STANDARD_Z + 10.0,
                CYLINDER_SEGMENTS,
            )
            .translate(
                STANDARD_POS.0 + centered_index(col, 4, 62.0),
                STANDARD_POS.1 + centered_index(row, 3, 52.0),
                place_z(STANDARD_Z),
            );
    }
    cuts
}

fn reference_standard_vial_rims() -> Part {
    let mut rims = Part::empty("inline_ph_osmo_co2_lag_reference_standard_vial_rims");
    for standard in 0..REFERENCE_STANDARD_LANDS {
        let row = standard / 4;
        let col = standard % 4;
        let x = STANDARD_POS.0 + centered_index(col, 4, 62.0);
        let y = STANDARD_POS.1 + centered_index(row, 3, 52.0);
        let rim = centered_cylinder(
            format!("inline_ph_osmo_co2_lag_reference_standard_{standard}_rim_outer"),
            STANDARD_VIAL_D / 2.0 + 5.0,
            4.0,
            CYLINDER_SEGMENTS,
        )
        .translate(x, y, deck_top_z() + STANDARD_Z + 2.0)
            - centered_cylinder(
                format!("inline_ph_osmo_co2_lag_reference_standard_{standard}_rim_inner"),
                STANDARD_VIAL_D / 2.0,
                6.0,
                CYLINDER_SEGMENTS,
            )
            .translate(x, y, deck_top_z() + STANDARD_Z + 2.0);
        rims = rims + rim;
    }
    rims
}

fn custody_seal_rings() -> Part {
    let mut rings = Part::empty("inline_ph_osmo_co2_lag_standard_custody_seal_rings");
    for ring in 0..STANDARD_SEAL_WELLS {
        let seal = centered_cylinder(
            format!("inline_ph_osmo_co2_lag_reference_custody_seal_{ring}_outer"),
            8.0,
            4.0,
            24,
        )
        .translate(
            STANDARD_POS.0 + centered_index(ring, STANDARD_SEAL_WELLS, 36.0),
            STANDARD_POS.1 - STANDARD_Y / 2.0 + 24.0,
            deck_top_z() + STANDARD_Z + 4.0,
        ) - centered_cylinder(
            format!("inline_ph_osmo_co2_lag_reference_custody_seal_{ring}_inner"),
            4.0,
            6.0,
            24,
        )
        .translate(
            STANDARD_POS.0 + centered_index(ring, STANDARD_SEAL_WELLS, 36.0),
            STANDARD_POS.1 - STANDARD_Y / 2.0 + 24.0,
            deck_top_z() + STANDARD_Z + 4.0,
        );
        rings = rings + seal;
    }
    rings
}

fn standard_family_label_lands() -> Part {
    let mut lands = Part::empty("inline_ph_osmo_co2_lag_standard_family_label_lands");
    for family in 0..3 {
        lands = lands
            + centered_cube(
                format!("inline_ph_osmo_co2_lag_standard_family_{family}_label_land"),
                112.0,
                12.0,
                5.0,
            )
            .translate(
                STANDARD_POS.0 + centered_index(family, 3, 116.0),
                STANDARD_POS.1 + STANDARD_Y / 2.0 - 22.0,
                deck_top_z() + STANDARD_Z + 4.0,
            );
    }
    lands
}

fn custody_chain_rail() -> Part {
    centered_cube(
        "inline_ph_osmo_co2_lag_reference_standard_chain_of_custody_rail",
        STANDARD_X - 64.0,
        9.0,
        7.0,
    )
    .translate(
        STANDARD_POS.0,
        STANDARD_POS.1 - 28.0,
        deck_top_z() + STANDARD_Z + 5.0,
    )
}

fn timed_residence_loop_token_rail() -> Part {
    let body = centered_cube(
        "inline_ph_osmo_co2_lag_timed_residence_token_rail_body",
        TOKEN_X,
        TOKEN_Y,
        TOKEN_Z,
    )
    .translate(TOKEN_POS.0, TOKEN_POS.1, place_z(TOKEN_Z));
    body - residence_token_recesses()
        + residence_token_rims()
        + residence_loop_lane_rails()
        + timebase_reference_bar()
}

fn residence_token_recesses() -> Part {
    let mut cuts = Part::empty("inline_ph_osmo_co2_lag_residence_token_recesses");
    for group in 0..PERFUSION_GROUPS {
        for level in 0..RESIDENCE_LEVELS {
            cuts = cuts
                + centered_cylinder(
                    format!(
                        "inline_ph_osmo_co2_lag_group_{group}_residence_level_{level}_token_recess"
                    ),
                    RESIDENCE_TOKEN_D / 2.0,
                    TOKEN_Z + 8.0,
                    24,
                )
                .translate(
                    TOKEN_POS.0 + group_x(group, 78.0),
                    TOKEN_POS.1 + centered_index(level, RESIDENCE_LEVELS, 32.0),
                    place_z(TOKEN_Z),
                );
        }
    }
    cuts
}

fn residence_token_rims() -> Part {
    let mut rims = Part::empty("inline_ph_osmo_co2_lag_residence_token_rims");
    for group in 0..PERFUSION_GROUPS {
        for level in 0..RESIDENCE_LEVELS {
            let x = TOKEN_POS.0 + group_x(group, 78.0);
            let y = TOKEN_POS.1 + centered_index(level, RESIDENCE_LEVELS, 32.0);
            let rim = centered_cylinder(
                format!("inline_ph_osmo_co2_lag_group_{group}_residence_level_{level}_token_rim_outer"),
                RESIDENCE_TOKEN_D / 2.0 + 4.5,
                4.0,
                24,
            )
            .translate(x, y, deck_top_z() + TOKEN_Z + 2.0)
                - centered_cylinder(
                    format!("inline_ph_osmo_co2_lag_group_{group}_residence_level_{level}_token_rim_inner"),
                    RESIDENCE_TOKEN_D / 2.0,
                    6.0,
                    24,
                )
                .translate(x, y, deck_top_z() + TOKEN_Z + 2.0);
            rims = rims + rim;
        }
    }
    rims
}

fn residence_loop_lane_rails() -> Part {
    let mut rails = Part::empty("inline_ph_osmo_co2_lag_residence_loop_lane_rails");
    for lane in 0..RESIDENCE_LOOP_RAILS {
        rails = rails
            + centered_cube(
                format!("inline_ph_osmo_co2_lag_residence_loop_lane_{lane}_rail"),
                48.0,
                TOKEN_Y - 34.0,
                5.0,
            )
            .translate(
                TOKEN_POS.0 + group_x(lane, 78.0),
                TOKEN_POS.1,
                deck_top_z() + TOKEN_Z + 5.0,
            );
    }
    rails
}

fn timebase_reference_bar() -> Part {
    centered_cube(
        "inline_ph_osmo_co2_lag_timebase_reference_bar",
        TOKEN_X - 54.0,
        10.0,
        6.0,
    )
    .translate(
        TOKEN_POS.0,
        TOKEN_POS.1 + TOKEN_Y / 2.0 - 20.0,
        deck_top_z() + TOKEN_Z + 4.0,
    )
}

fn waste_flush_capture_caddy() -> Part {
    let body = centered_cube(
        "inline_ph_osmo_co2_lag_waste_flush_capture_body",
        WASTE_X,
        WASTE_Y,
        WASTE_Z,
    )
    .translate(WASTE_POS.0, WASTE_POS.1, place_z(WASTE_Z));
    body - waste_capture_cup_cuts()
        + waste_capture_cup_rims()
        + flush_manifold_channels()
        + segregated_waste_stream_labels()
}

fn waste_capture_cup_cuts() -> Part {
    let mut cuts = Part::empty("inline_ph_osmo_co2_lag_waste_capture_cup_cuts");
    for stream in 0..WASTE_STREAMS {
        for cup in 0..WASTE_CUPS_PER_STREAM {
            cuts = cuts
                + centered_cylinder(
                    format!("inline_ph_osmo_co2_lag_waste_stream_{stream}_cup_{cup}_cut"),
                    WASTE_CUP_D / 2.0,
                    WASTE_Z + 10.0,
                    CYLINDER_SEGMENTS,
                )
                .translate(
                    WASTE_POS.0 + group_x(stream, 42.0),
                    WASTE_POS.1 + centered_index(cup, WASTE_CUPS_PER_STREAM, 56.0),
                    place_z(WASTE_Z),
                );
        }
    }
    cuts
}

fn waste_capture_cup_rims() -> Part {
    let mut rims = Part::empty("inline_ph_osmo_co2_lag_waste_capture_cup_rims");
    for stream in 0..WASTE_STREAMS {
        for cup in 0..WASTE_CUPS_PER_STREAM {
            let x = WASTE_POS.0 + group_x(stream, 42.0);
            let y = WASTE_POS.1 + centered_index(cup, WASTE_CUPS_PER_STREAM, 56.0);
            let rim = centered_cylinder(
                format!("inline_ph_osmo_co2_lag_waste_stream_{stream}_cup_{cup}_rim_outer"),
                WASTE_CUP_D / 2.0 + 5.0,
                5.0,
                CYLINDER_SEGMENTS,
            )
            .translate(x, y, deck_top_z() + WASTE_Z + 2.5)
                - centered_cylinder(
                    format!("inline_ph_osmo_co2_lag_waste_stream_{stream}_cup_{cup}_rim_inner"),
                    WASTE_CUP_D / 2.0,
                    7.0,
                    CYLINDER_SEGMENTS,
                )
                .translate(x, y, deck_top_z() + WASTE_Z + 2.5);
            rims = rims + rim;
        }
    }
    rims
}

fn flush_manifold_channels() -> Part {
    let mut channels = Part::empty("inline_ph_osmo_co2_lag_flush_manifold_channels");
    for channel in 0..FLUSH_MANIFOLD_CHANNELS {
        channels = channels
            + centered_cube(
                format!("inline_ph_osmo_co2_lag_flush_channel_{channel}_raised_route"),
                30.0,
                WASTE_Y - 42.0,
                7.0,
            )
            .translate(
                WASTE_POS.0 + group_x(channel, 42.0),
                WASTE_POS.1,
                deck_top_z() + WASTE_Z + 5.0,
            );
    }
    channels
}

fn segregated_waste_stream_labels() -> Part {
    let mut labels = Part::empty("inline_ph_osmo_co2_lag_segregated_waste_stream_labels");
    for stream in 0..WASTE_STREAMS {
        labels = labels
            + centered_cube(
                format!("inline_ph_osmo_co2_lag_waste_stream_{stream}_barcode_land"),
                32.0,
                12.0,
                5.0,
            )
            .translate(
                WASTE_POS.0 + group_x(stream, 42.0),
                WASTE_POS.1 - WASTE_Y / 2.0 + 24.0,
                deck_top_z() + WASTE_Z + 4.0,
            );
    }
    labels
}

fn batch_traceability_lands() -> Part {
    let body = centered_cube(
        "inline_ph_osmo_co2_lag_batch_traceability_body",
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    )
    .translate(TRACE_POS.0, TRACE_POS.1, place_z(TRACE_Z));
    body + batch_barcode_lands() + run_card_lands() + sample_time_lands() + custody_seal_wells()
}

fn batch_barcode_lands() -> Part {
    let mut lands = Part::empty("inline_ph_osmo_co2_lag_batch_barcode_lands");
    for land in 0..BATCH_BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("inline_ph_osmo_co2_lag_batch_barcode_land_{land}"),
                34.0,
                14.0,
                4.0,
            )
            .translate(
                TRACE_POS.0 + centered_index(land, BATCH_BARCODE_LANDS, 40.0),
                TRACE_POS.1 + 55.0,
                deck_top_z() + TRACE_Z + 2.0,
            );
    }
    lands
}

fn run_card_lands() -> Part {
    let mut lands = Part::empty("inline_ph_osmo_co2_lag_run_card_lands");
    for land in 0..RUN_CARD_LANDS {
        lands = lands
            + rectangular_frame(
                &format!("inline_ph_osmo_co2_lag_run_card_land_{land}"),
                64.0,
                30.0,
                4.0,
                5.0,
            )
            .translate(
                TRACE_POS.0 + centered_index(land, RUN_CARD_LANDS, 76.0),
                TRACE_POS.1 + 8.0,
                deck_top_z() + TRACE_Z + 3.0,
            );
    }
    lands
}

fn sample_time_lands() -> Part {
    let mut lands = Part::empty("inline_ph_osmo_co2_lag_sample_time_lands");
    for land in 0..SAMPLE_TIME_LANDS {
        lands = lands
            + centered_cube(
                format!("inline_ph_osmo_co2_lag_group_{land}_sample_time_land"),
                36.0,
                12.0,
                5.0,
            )
            .translate(
                TRACE_POS.0 + group_x(land, 50.0),
                TRACE_POS.1 - 42.0,
                deck_top_z() + TRACE_Z + 4.0,
            );
    }
    lands
}

fn custody_seal_wells() -> Part {
    let mut wells = Part::empty("inline_ph_osmo_co2_lag_batch_custody_seal_wells");
    for well in 0..CUSTODY_SEAL_WELLS {
        let seal = centered_cylinder(
            format!("inline_ph_osmo_co2_lag_batch_custody_seal_{well}_outer"),
            8.0,
            4.0,
            24,
        )
        .translate(
            TRACE_POS.0 + group_x(well, 42.0),
            TRACE_POS.1 - TRACE_Y / 2.0 + 22.0,
            deck_top_z() + TRACE_Z + 3.0,
        ) - centered_cylinder(
            format!("inline_ph_osmo_co2_lag_batch_custody_seal_{well}_inner"),
            4.0,
            6.0,
            24,
        )
        .translate(
            TRACE_POS.0 + group_x(well, 42.0),
            TRACE_POS.1 - TRACE_Y / 2.0 + 22.0,
            deck_top_z() + TRACE_Z + 3.0,
        );
        wells = wells + seal;
    }
    wells
}

fn release_hold_reject_disposition_gate() -> Part {
    let body = centered_cube(
        "inline_ph_osmo_co2_lag_release_hold_reject_disposition_body",
        DISPOSITION_X,
        DISPOSITION_Y,
        DISPOSITION_Z,
    )
    .translate(DISPOSITION_POS.0, DISPOSITION_POS.1, place_z(DISPOSITION_Z));
    body - disposition_token_recesses()
        + disposition_token_rims()
        + disposition_lane_dividers()
        + decision_card_lands()
        + release_hold_reject_hard_stops()
}

fn disposition_token_recesses() -> Part {
    let mut cuts = Part::empty("inline_ph_osmo_co2_lag_disposition_token_recesses");
    for state in 0..DISPOSITION_STATES {
        let state_name = DISPOSITION_NAMES[state];
        for group in 0..PERFUSION_GROUPS {
            cuts = cuts
                + centered_cylinder(
                    format!("inline_ph_osmo_co2_lag_{state_name}_group_{group}_token_recess"),
                    7.0,
                    DISPOSITION_Z + 8.0,
                    24,
                )
                .translate(
                    DISPOSITION_POS.0 + group_x(group, 54.0),
                    DISPOSITION_POS.1 + centered_index(state, DISPOSITION_STATES, 48.0),
                    place_z(DISPOSITION_Z),
                );
        }
    }
    cuts
}

fn disposition_token_rims() -> Part {
    let mut rims = Part::empty("inline_ph_osmo_co2_lag_disposition_token_rims");
    for state in 0..DISPOSITION_STATES {
        let state_name = DISPOSITION_NAMES[state];
        for group in 0..PERFUSION_GROUPS {
            let x = DISPOSITION_POS.0 + group_x(group, 54.0);
            let y = DISPOSITION_POS.1 + centered_index(state, DISPOSITION_STATES, 48.0);
            let rim = centered_cylinder(
                format!("inline_ph_osmo_co2_lag_{state_name}_group_{group}_token_rim_outer"),
                11.0,
                4.0,
                24,
            )
            .translate(x, y, deck_top_z() + DISPOSITION_Z + 2.0)
                - centered_cylinder(
                    format!("inline_ph_osmo_co2_lag_{state_name}_group_{group}_token_rim_inner"),
                    7.0,
                    6.0,
                    24,
                )
                .translate(x, y, deck_top_z() + DISPOSITION_Z + 2.0);
            rims = rims + rim;
        }
    }
    rims
}

fn disposition_lane_dividers() -> Part {
    let mut dividers = Part::empty("inline_ph_osmo_co2_lag_disposition_lane_dividers");
    for divider in 0..4 {
        dividers = dividers
            + centered_cube(
                format!("inline_ph_osmo_co2_lag_disposition_lane_divider_{divider}"),
                DISPOSITION_X - 64.0,
                6.0,
                8.0,
            )
            .translate(
                DISPOSITION_POS.0,
                DISPOSITION_POS.1 - 72.0 + divider as f64 * 48.0,
                deck_top_z() + DISPOSITION_Z + 6.0,
            );
    }
    dividers
}

fn decision_card_lands() -> Part {
    let mut lands = Part::empty("inline_ph_osmo_co2_lag_decision_card_lands");
    for land in 0..DECISION_CARD_LANDS {
        let state_name = DISPOSITION_NAMES[land];
        lands = lands
            + rectangular_frame(
                &format!("inline_ph_osmo_co2_lag_{state_name}_decision_card_land"),
                82.0,
                28.0,
                4.0,
                5.0,
            )
            .translate(
                DISPOSITION_POS.0 - DISPOSITION_X / 2.0 + 64.0,
                DISPOSITION_POS.1 + centered_index(land, DISPOSITION_STATES, 48.0),
                deck_top_z() + DISPOSITION_Z + 4.0,
            );
    }
    lands
}

fn release_hold_reject_hard_stops() -> Part {
    let mut stops = Part::empty("inline_ph_osmo_co2_lag_release_hold_reject_hard_stops");
    for state in 0..DISPOSITION_STATES {
        let state_name = DISPOSITION_NAMES[state];
        stops = stops
            + centered_cube(
                format!("inline_ph_osmo_co2_lag_{state_name}_lane_hard_stop"),
                14.0,
                36.0,
                22.0,
            )
            .translate(
                DISPOSITION_POS.0 + DISPOSITION_X / 2.0 - 42.0,
                DISPOSITION_POS.1 + centered_index(state, DISPOSITION_STATES, 48.0),
                deck_top_z() + DISPOSITION_Z + 11.0,
            );
    }
    stops
}

fn camera_illumination_fiducial_bridge() -> Part {
    let beam = centered_cube(
        "inline_ph_osmo_co2_lag_camera_illumination_bridge_beam",
        CAMERA_X,
        CAMERA_Y,
        CAMERA_BEAM_Z,
    )
    .translate(
        CAMERA_POS.0,
        CAMERA_POS.1,
        deck_top_z() + CAMERA_POST_Z + CAMERA_BEAM_Z / 2.0,
    );
    beam + camera_mounts() + illumination_bars() + camera_fiducial_targets() + bridge_posts()
}

fn bridge_posts() -> Part {
    let mut posts = Part::empty("inline_ph_osmo_co2_lag_camera_bridge_posts");
    for post in 0..4 {
        posts = posts
            + centered_cube(
                format!("inline_ph_osmo_co2_lag_camera_bridge_post_{post}"),
                24.0,
                24.0,
                CAMERA_POST_Z,
            )
            .translate(
                centered_index(post, 4, CAMERA_X / 3.0),
                CAMERA_POS.1,
                deck_top_z() + CAMERA_POST_Z / 2.0,
            );
    }
    posts
}

fn camera_mounts() -> Part {
    let mut mounts = Part::empty("inline_ph_osmo_co2_lag_camera_mounts");
    for camera in 0..CAMERA_COUNT {
        mounts = mounts
            + rectangular_frame(
                &format!("inline_ph_osmo_co2_lag_camera_{camera}_mount_frame"),
                70.0,
                34.0,
                5.0,
                8.0,
            )
            .translate(
                CAMERA_POS.0 + centered_index(camera, CAMERA_COUNT, 300.0),
                CAMERA_POS.1,
                deck_top_z() + CAMERA_POST_Z + CAMERA_BEAM_Z + 6.0,
            );
    }
    mounts
}

fn illumination_bars() -> Part {
    let mut bars = Part::empty("inline_ph_osmo_co2_lag_illumination_bars");
    for bar in 0..ILLUMINATION_BARS {
        bars = bars
            + centered_cube(
                format!("inline_ph_osmo_co2_lag_illumination_bar_{bar}"),
                96.0,
                8.0,
                6.0,
            )
            .translate(
                CAMERA_POS.0 + centered_index(bar, ILLUMINATION_BARS, 170.0),
                CAMERA_POS.1 - CAMERA_Y / 2.0 - 12.0,
                deck_top_z() + CAMERA_POST_Z + CAMERA_BEAM_Z + 5.0,
            );
    }
    bars
}

fn camera_fiducial_targets() -> Part {
    let mut targets = Part::empty("inline_ph_osmo_co2_lag_camera_fiducial_targets");
    for fiducial in 0..CAMERA_FIDUCIALS {
        let target = centered_cylinder(
            format!("inline_ph_osmo_co2_lag_camera_fiducial_{fiducial}_outer"),
            10.0,
            4.0,
            FIDUCIAL_SEGMENTS,
        )
        .translate(
            CAMERA_POS.0 + centered_index(fiducial, CAMERA_FIDUCIALS, 220.0),
            CAMERA_POS.1 + CAMERA_Y / 2.0 + 12.0,
            deck_top_z() + CAMERA_POST_Z + CAMERA_BEAM_Z + 5.0,
        ) - centered_cylinder(
            format!("inline_ph_osmo_co2_lag_camera_fiducial_{fiducial}_inner"),
            4.0,
            6.0,
            24,
        )
        .translate(
            CAMERA_POS.0 + centered_index(fiducial, CAMERA_FIDUCIALS, 220.0),
            CAMERA_POS.1 + CAMERA_Y / 2.0 + 12.0,
            deck_top_z() + CAMERA_POST_Z + CAMERA_BEAM_Z + 5.0,
        );
        targets = targets + target;
    }
    targets
}

fn route_and_timing_overlay() -> Part {
    let mut overlay = Part::empty("inline_ph_osmo_co2_lag_route_and_timing_overlay");
    for (i, (x1, y1, x2, y2)) in route_segments().into_iter().enumerate() {
        let width = (x2 - x1).abs().max(8.0);
        let depth = (y2 - y1).abs().max(8.0);
        overlay = overlay
            + centered_cube(
                format!("inline_ph_osmo_co2_lag_route_overlay_segment_{i}"),
                width,
                depth,
                4.0,
            )
            .translate((x1 + x2) / 2.0, (y1 + y2) / 2.0, deck_top_z() + 5.0);
    }
    for marker in 0..FLOW_MARKERS {
        overlay = overlay
            + centered_cube(
                format!("inline_ph_osmo_co2_lag_flow_timing_marker_{marker}"),
                24.0,
                5.0,
                5.0,
            )
            .translate(
                -690.0 + marker as f64 * 80.0,
                -142.0 + if marker % 2 == 0 { 0.0 } else { 20.0 },
                deck_top_z() + 10.0,
            );
    }
    overlay
}

fn route_segments() -> [(f64, f64, f64, f64); ROUTE_SEGMENTS] {
    [
        (
            HANDOFF_POS.0 + HANDOFF_X / 2.0,
            HANDOFF_POS.1,
            GAS_POS.0 - GAS_X / 2.0,
            GAS_POS.1,
        ),
        (
            GAS_POS.0 + GAS_X / 2.0,
            GAS_POS.1,
            PROBE_POS.0 - PROBE_X / 2.0,
            PROBE_POS.1,
        ),
        (
            PROBE_POS.0 + PROBE_X / 2.0,
            PROBE_POS.1,
            TEMP_POS.0 - TEMP_X / 2.0,
            TEMP_POS.1,
        ),
        (
            TEMP_POS.0,
            TEMP_POS.1 - TEMP_Y / 2.0,
            WASTE_POS.0,
            WASTE_POS.1 + WASTE_Y / 2.0,
        ),
        (
            GAS_POS.0,
            GAS_POS.1 - GAS_Y / 2.0,
            BUBBLE_POS.0,
            BUBBLE_POS.1 + BUBBLE_Y / 2.0,
        ),
        (
            HANDOFF_POS.0,
            HANDOFF_POS.1 - HANDOFF_Y / 2.0,
            SAMPLE_POS.0,
            SAMPLE_POS.1 + SAMPLE_Y / 2.0,
        ),
        (
            SAMPLE_POS.0 + SAMPLE_X / 2.0,
            SAMPLE_POS.1,
            BUBBLE_POS.0 - BUBBLE_X / 2.0,
            BUBBLE_POS.1,
        ),
        (
            BUBBLE_POS.0 + BUBBLE_X / 2.0,
            BUBBLE_POS.1,
            STANDARD_POS.0 - STANDARD_X / 2.0,
            STANDARD_POS.1,
        ),
        (
            STANDARD_POS.0 + STANDARD_X / 2.0,
            STANDARD_POS.1,
            WASTE_POS.0 - WASTE_X / 2.0,
            WASTE_POS.1,
        ),
        (
            SAMPLE_POS.0,
            SAMPLE_POS.1 - SAMPLE_Y / 2.0,
            TOKEN_POS.0,
            TOKEN_POS.1 + TOKEN_Y / 2.0,
        ),
        (
            TOKEN_POS.0 + TOKEN_X / 2.0,
            TOKEN_POS.1,
            DISPOSITION_POS.0 - DISPOSITION_X / 2.0,
            DISPOSITION_POS.1,
        ),
        (
            DISPOSITION_POS.0 + DISPOSITION_X / 2.0,
            DISPOSITION_POS.1,
            TRACE_POS.0 - TRACE_X / 2.0,
            TRACE_POS.1,
        ),
        (
            WASTE_POS.0,
            WASTE_POS.1 - WASTE_Y / 2.0,
            TRACE_POS.0,
            TRACE_POS.1 + TRACE_Y / 2.0,
        ),
    ]
}

fn rectangular_frame(name: &str, outer_x: f64, outer_y: f64, rail: f64, z: f64) -> Part {
    let left = centered_cube(format!("{name}_left_rail"), rail, outer_y, z).translate(
        -outer_x / 2.0 + rail / 2.0,
        0.0,
        0.0,
    );
    let right = centered_cube(format!("{name}_right_rail"), rail, outer_y, z).translate(
        outer_x / 2.0 - rail / 2.0,
        0.0,
        0.0,
    );
    let front = centered_cube(format!("{name}_front_rail"), outer_x, rail, z).translate(
        0.0,
        -outer_y / 2.0 + rail / 2.0,
        0.0,
    );
    let rear = centered_cube(format!("{name}_rear_rail"), outer_x, rail, z).translate(
        0.0,
        outer_y / 2.0 - rail / 2.0,
        0.0,
    );
    left + right + front + rear
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 14);
    assert!(OUTPUTS.iter().all(|path| path.starts_with(OUTPUT_PREFIX)));
    assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
    assert_eq!(
        OUTPUTS[OUTPUTS.len() - 1],
        format!("{OUTPUT_PREFIX}assembly.stl")
    );
    assert_eq!(REQUIRED_FEATURES.len(), 17);
    assert_eq!(RESEARCH_BASIS.len(), 6);
    assert!(RESEARCH_BASIS.iter().all(|basis| basis.is_ascii()));
    assert_eq!(LIMITATIONS.len(), 7);
    assert_eq!(UNITS, "millimeters");
    assert_eq!(CHIP_COUNT, CHIP_COLUMNS * CHIP_ROWS);
    assert_eq!(PERFUSION_GROUPS, CHIP_ROWS);
    assert_eq!(CHIPS_PER_GROUP, CHIP_COLUMNS);
    assert_eq!(HANDOFF_PORTS, PERFUSION_GROUPS * 2);
    assert_eq!(GAS_LOOP_LANES, PERFUSION_GROUPS);
    assert_eq!(GAS_LOOP_PORTS, GAS_LOOP_LANES * 2);
    assert_eq!(GAS_LAG_WINDOWS, PERFUSION_GROUPS);
    assert_eq!(GAS_MEMBRANE_WINDOWS, PERFUSION_GROUPS);
    assert_eq!(CO2_PROBE_DOCKS, PERFUSION_GROUPS);
    assert_eq!(O2_PROBE_DOCKS, PERFUSION_GROUPS);
    assert_eq!(TOTAL_GAS_PROBE_DOCKS, CO2_PROBE_DOCKS + O2_PROBE_DOCKS);
    assert_eq!(PROBE_CABLE_CLAMPS, TOTAL_GAS_PROBE_DOCKS);
    assert_eq!(TEMPERATURE_ZONES, PERFUSION_GROUPS);
    assert_eq!(TEMP_REFERENCE_WELLS, PERFUSION_GROUPS);
    assert_eq!(PH_SAMPLE_WELLS, PERFUSION_GROUPS * SAMPLE_REPLICATES);
    assert_eq!(
        OSMOLALITY_SAMPLE_WELLS,
        PERFUSION_GROUPS * SAMPLE_REPLICATES
    );
    assert_eq!(SPLIT_RETAIN_WELLS, PERFUSION_GROUPS * 2);
    assert_eq!(
        TOTAL_SAMPLE_WELLS,
        PH_SAMPLE_WELLS + OSMOLALITY_SAMPLE_WELLS + SPLIT_RETAIN_WELLS
    );
    assert_eq!(BUBBLE_WINDOWS, PERFUSION_GROUPS);
    assert_eq!(DEAD_VOLUME_WINDOWS, PERFUSION_GROUPS);
    assert_eq!(
        REFERENCE_STANDARD_LANDS,
        PH_STANDARD_LANDS + OSMO_STANDARD_LANDS + GAS_STANDARD_LANDS
    );
    assert_eq!(WASTE_CAPTURE_CUPS, WASTE_STREAMS * WASTE_CUPS_PER_STREAM);
    assert_eq!(FLUSH_MANIFOLD_CHANNELS, PERFUSION_GROUPS);
    assert_eq!(RESIDENCE_TOKENS, CHIP_COUNT);
    assert_eq!(RESIDENCE_LOOP_RAILS, PERFUSION_GROUPS);
    assert_eq!(DISPOSITION_NAMES, ["release", "hold", "reject"]);
    assert_eq!(
        DISPOSITION_TOKEN_WELLS,
        DISPOSITION_STATES * PERFUSION_GROUPS
    );
    assert_eq!(SAMPLE_TIME_LANDS, PERFUSION_GROUPS);
    assert_eq!(DECK_DATUM_TARGETS, 8);
    assert_eq!(route_segments().len(), ROUTE_SEGMENTS);

    for required in [
        "containment_deck",
        "scaled_multi_chip_perfusion_handoff",
        "gas_equilibration_loop_surrogate",
        "ph_sample_wells",
        "osmolality_sample_wells",
        "co2_probe_docks",
        "o2_probe_docks",
        "temperature_equilibration_block",
        "timed_residence_loop_token_rail",
        "bubble_windows",
        "dead_volume_windows",
        "reference_standard_custody_lands",
        "sample_split_retain_wells",
        "waste_flush_capture",
        "camera_illumination_fiducials",
        "release_hold_reject_disposition_features",
        "batch_traceability_lands",
    ] {
        assert!(REQUIRED_FEATURES.contains(&required));
    }

    for limitation in [
        "architecture_cad_only",
        "not_a_sterile_barrier_claim",
        "not_a_pressure_rated_gas_mixer",
        "no_clinical_release_thresholds",
        "no_sensor_metrology_claim",
        "no_media_recipe_or_process_limits",
        "no_live_cell_acceptance_criterion",
    ] {
        assert!(LIMITATIONS.contains(&limitation));
    }

    for module in module_footprints() {
        assert!(
            module.fits_inside_station(),
            "{} exceeds station envelope",
            module.name
        );
    }

    let modules = module_footprints();
    for i in 0..modules.len() {
        for j in (i + 1)..modules.len() {
            assert!(
                !modules[i].overlaps_with_clearance(modules[j], MODULE_CLEARANCE_MM),
                "{} overlaps {}",
                modules[i].name,
                modules[j].name
            );
        }
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
        assert_eq!(OUTPUTS.len(), 14);
        for path in OUTPUTS {
            assert!(path.starts_with(OUTPUT_PREFIX));
            assert!(path.ends_with(".stl"));
        }
        assert_eq!(
            OUTPUTS,
            [
                "output/closed_inline_media_ph_osmolality_co2_lag_validation_station_containment_deck.stl",
                "output/closed_inline_media_ph_osmolality_co2_lag_validation_station_scaled_multichip_perfusion_handoff_bulkhead.stl",
                "output/closed_inline_media_ph_osmolality_co2_lag_validation_station_gas_equilibration_lag_loop_surrogate.stl",
                "output/closed_inline_media_ph_osmolality_co2_lag_validation_station_co2_o2_probe_dock_bridge.stl",
                "output/closed_inline_media_ph_osmolality_co2_lag_validation_station_temperature_equilibration_block.stl",
                "output/closed_inline_media_ph_osmolality_co2_lag_validation_station_ph_osmolality_sample_split_retain_wells.stl",
                "output/closed_inline_media_ph_osmolality_co2_lag_validation_station_bubble_dead_volume_window_panel.stl",
                "output/closed_inline_media_ph_osmolality_co2_lag_validation_station_reference_standard_custody_lands.stl",
                "output/closed_inline_media_ph_osmolality_co2_lag_validation_station_timed_residence_loop_token_rail.stl",
                "output/closed_inline_media_ph_osmolality_co2_lag_validation_station_waste_flush_capture_caddy.stl",
                "output/closed_inline_media_ph_osmolality_co2_lag_validation_station_batch_traceability_lands.stl",
                "output/closed_inline_media_ph_osmolality_co2_lag_validation_station_release_hold_reject_disposition_gate.stl",
                "output/closed_inline_media_ph_osmolality_co2_lag_validation_station_camera_illumination_fiducial_bridge.stl",
                "output/closed_inline_media_ph_osmolality_co2_lag_validation_station_assembly.stl",
            ]
        );
    }

    #[test]
    fn requested_validation_features_are_represented() {
        for feature in [
            "containment_deck",
            "scaled_multi_chip_perfusion_handoff",
            "gas_equilibration_loop_surrogate",
            "ph_sample_wells",
            "osmolality_sample_wells",
            "co2_probe_docks",
            "o2_probe_docks",
            "temperature_equilibration_block",
            "timed_residence_loop_token_rail",
            "bubble_windows",
            "dead_volume_windows",
            "reference_standard_custody_lands",
            "sample_split_retain_wells",
            "waste_flush_capture",
            "camera_illumination_fiducials",
            "release_hold_reject_disposition_features",
            "batch_traceability_lands",
        ] {
            assert!(
                REQUIRED_FEATURES.contains(&feature),
                "missing feature {feature}"
            );
        }
    }

    #[test]
    fn counts_cover_multi_chip_media_chemistry_and_release_traceability() {
        assert_eq!(CHIP_COLUMNS, 4);
        assert_eq!(CHIP_ROWS, 6);
        assert_eq!(CHIP_COUNT, 24);
        assert_eq!(PERFUSION_GROUPS, 6);
        assert_eq!(HANDOFF_PORTS, 12);
        assert_eq!(PH_SAMPLE_WELLS, 12);
        assert_eq!(OSMOLALITY_SAMPLE_WELLS, 12);
        assert_eq!(SPLIT_RETAIN_WELLS, 12);
        assert_eq!(TOTAL_SAMPLE_WELLS, 36);
        assert_eq!(CO2_PROBE_DOCKS, 6);
        assert_eq!(O2_PROBE_DOCKS, 6);
        assert_eq!(TEMPERATURE_ZONES, 6);
        assert_eq!(BUBBLE_WINDOWS, 6);
        assert_eq!(DEAD_VOLUME_WINDOWS, 6);
        assert_eq!(REFERENCE_STANDARD_LANDS, 12);
        assert_eq!(WASTE_CAPTURE_CUPS, 12);
        assert_eq!(RESIDENCE_TOKENS, CHIP_COUNT);
        assert_eq!(DISPOSITION_NAMES, ["release", "hold", "reject"]);
        assert_eq!(DISPOSITION_TOKEN_WELLS, 18);
    }

    #[test]
    fn station_modules_fit_inside_bounds_without_major_overlap() {
        assert_design_constraints();
        for module in module_footprints() {
            assert!(module.fits_inside_station(), "{module:?} outside deck");
        }
        let modules = module_footprints();
        for (i, left) in modules.iter().enumerate() {
            for right in modules.iter().skip(i + 1) {
                assert!(
                    !left.overlaps_with_clearance(*right, MODULE_CLEARANCE_MM),
                    "{} overlaps {}",
                    left.name,
                    right.name
                );
            }
        }
    }

    #[test]
    fn research_basis_and_limitations_are_explicit() {
        for basis in [
            "co2_bicarbonate_equilibrium_drives_media_ph",
            "oxygen_and_carbon_dioxide_instability_affects_cell_culture_reproducibility",
            "temperature_and_sample_timing_change_measured_media_state",
            "evaporation_and_concentration_shift_osmolality",
            "perfusion_residence_time_and_sensor_lag_affect_inline_release_evidence",
            "batch_traceability_must_follow_media_to_scaled_chip_lanes",
        ] {
            assert!(RESEARCH_BASIS.contains(&basis));
        }
        assert!(LIMITATIONS.contains(&"no_clinical_release_thresholds"));
        assert!(LIMITATIONS.contains(&"no_sensor_metrology_claim"));
        assert!(LIMITATIONS.contains(&"no_live_cell_acceptance_criterion"));
    }

    #[test]
    fn workflow_routes_from_conditioning_to_sampling_release_and_traceability() {
        assert!(HANDOFF_POS.0 < GAS_POS.0);
        assert!(GAS_POS.0 < PROBE_POS.0);
        assert!(PROBE_POS.0 < TEMP_POS.0);
        assert!(SAMPLE_POS.1 < HANDOFF_POS.1);
        assert!(TOKEN_POS.1 < SAMPLE_POS.1);
        assert!(DISPOSITION_POS.0 > TOKEN_POS.0);
        assert!(TRACE_POS.0 > DISPOSITION_POS.0);
        assert_eq!(route_segments().len(), ROUTE_SEGMENTS);
    }
}
