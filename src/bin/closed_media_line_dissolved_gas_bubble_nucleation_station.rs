use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed media-line dissolved-gas bubble nucleation validation station.
//
// Intent:
// - Challenge conditioned media across temperature steps, pressure drops, and
//   gas-equilibration changes before it is released to closed culture hardware.
// - Make bubble witness, bypass, reference, flow-token, split, and disposition
//   gate geometry explicit without defining biological release criteria.
// - Keep this as a deterministic standalone CAD worker bin for parent rollout
//   integration.

const OUTPUT_PREFIX: &str = "output/closed_media_line_dissolved_gas_bubble_nucleation_station_";

const OUTPUTS: [&str; 12] = [
    "output/closed_media_line_dissolved_gas_bubble_nucleation_station_containment_deck.stl",
    "output/closed_media_line_dissolved_gas_bubble_nucleation_station_temperature_step_media_cartridge_bank.stl",
    "output/closed_media_line_dissolved_gas_bubble_nucleation_station_pressure_drop_restrictor_ladder.stl",
    "output/closed_media_line_dissolved_gas_bubble_nucleation_station_transparent_bubble_witness_window_array.stl",
    "output/closed_media_line_dissolved_gas_bubble_nucleation_station_degas_bypass_coupon_dock.stl",
    "output/closed_media_line_dissolved_gas_bubble_nucleation_station_gas_equilibrated_reference_pocket_block.stl",
    "output/closed_media_line_dissolved_gas_bubble_nucleation_station_waste_retain_split_manifold.stl",
    "output/closed_media_line_dissolved_gas_bubble_nucleation_station_release_hold_reject_gate_bank.stl",
    "output/closed_media_line_dissolved_gas_bubble_nucleation_station_flow_token_rail.stl",
    "output/closed_media_line_dissolved_gas_bubble_nucleation_station_route_trace_overlay.stl",
    "output/closed_media_line_dissolved_gas_bubble_nucleation_station_robot_service_keepout_gauges.stl",
    "output/closed_media_line_dissolved_gas_bubble_nucleation_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "temperature_step_media_cartridges",
    "pressure_drop_restrictor_ladder",
    "transparent_bubble_witness_windows",
    "degas_bypass_coupon",
    "gas_equilibrated_reference_pockets",
    "flow_token_rail",
    "waste_retain_split",
    "release_hold_reject_gates",
    "closed_media_line_route_trace",
    "bubble_nucleation_challenge_tokens",
    "robot_service_keepouts",
];

const LIMITATIONS: [&str; 6] = [
    "validation_fixture_only",
    "not_a_pressure_rated_wetted_design",
    "not_a_sterility_protocol",
    "no_biological_release_acceptance_thresholds",
    "purchased_connectors_windows_and_sensors_are_surrogates",
    "temperature_pressure_and_gas_setpoints_require_process_validation",
];

const REPRODUCIBILITY_CONTROLS: [&str; 6] = [
    "fixed_output_manifest",
    "millimeter_units",
    "deterministic_named_geometry",
    "static_feature_counts",
    "stable_module_rectangles",
    "no_random_inputs",
];

const PARAMETRIC_REVISION: &str = "closed_media_line_dissolved_gas_bubble_nucleation_station_v1";
const UNITS: &str = "millimeters";
const DEFAULT_SEGMENTS: u32 = 32;

const STATION_X: f64 = 1580.0;
const STATION_Y: f64 = 980.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 46.0;
const BASIN_RECESS_Z: f64 = 7.0;
const SOCKET_DEPTH: f64 = 5.0;
const MOUNT_HOLE_D: f64 = 6.8;
const DATUM_RING_D: f64 = 18.0;
const LAYOUT_CLEARANCE: f64 = 14.0;

const LANES: usize = 6;
const TUBE_OD_MAX: f64 = 4.8;
const TUBE_CLEARANCE: f64 = 1.0;
const FLUID_BORE_D: f64 = TUBE_OD_MAX + TUBE_CLEARANCE;
const LANE_PITCH_X: f64 = 54.0;
const LANE_PITCH_Y: f64 = 30.0;

const TEMP_CENTER: (f64, f64) = (-480.0, 250.0);
const TEMP_X: f64 = 400.0;
const TEMP_Y: f64 = 220.0;
const TEMP_Z: f64 = 68.0;
const TEMP_STEPS: usize = 5;
const TEMP_CARTRIDGE_D: f64 = 42.0;
const TEMP_CARTRIDGE_Z: f64 = 58.0;
const TEMP_STEP_LABELS: [&str; TEMP_STEPS] = ["cold", "ambient", "prewarm", "culture", "return"];
const TEMP_PROBE_WELLS: usize = TEMP_STEPS * 2;
const TEMP_STEP_DELTA_C: f64 = 33.0;

const PRESSURE_CENTER: (f64, f64) = (0.0, 250.0);
const PRESSURE_X: f64 = 430.0;
const PRESSURE_Y: f64 = 220.0;
const PRESSURE_Z: f64 = 70.0;
const RESTRICTOR_STEPS: usize = LANES;
const RESTRICTOR_PRESSURE_TAPS: usize = RESTRICTOR_STEPS + 1;
const RESTRICTOR_MIN_BORE_D: f64 = 1.2;
const RESTRICTOR_MAX_BORE_D: f64 = 3.8;
const PRESSURE_DROP_LABEL_KPA: f64 = 55.0;

const WITNESS_CENTER: (f64, f64) = (480.0, 250.0);
const WITNESS_X: f64 = 400.0;
const WITNESS_Y: f64 = 220.0;
const WITNESS_Z: f64 = 58.0;
const WITNESS_WINDOWS: usize = LANES;
const WINDOW_X: f64 = 42.0;
const WINDOW_Y: f64 = 112.0;
const WINDOW_FRAME_RAIL: f64 = 5.0;
const BUBBLE_COMPARATOR_DOTS: usize = 4;
const BACKLIGHT_SLOTS: usize = WITNESS_WINDOWS;

const BYPASS_CENTER: (f64, f64) = (-520.0, -55.0);
const BYPASS_X: f64 = 360.0;
const BYPASS_Y: f64 = 230.0;
const BYPASS_Z: f64 = 64.0;
const DEGAS_COUPONS: usize = 3;
const BYPASS_COUPON_SOCKETS: usize = DEGAS_COUPONS;
const BYPASS_SELECTOR_VALVES: usize = 2;
const COUPON_SOCKET_X: f64 = 76.0;
const COUPON_SOCKET_Y: f64 = 110.0;

const REF_CENTER: (f64, f64) = (-115.0, -55.0);
const REF_X: f64 = 330.0;
const REF_Y: f64 = 230.0;
const REF_Z: f64 = 62.0;
const GAS_REFERENCE_POCKETS: usize = 4;
const REFERENCE_REPLICATES: usize = 2;
const REFERENCE_POCKET_COUNT: usize = GAS_REFERENCE_POCKETS * REFERENCE_REPLICATES;
const REFERENCE_POCKET_D: f64 = 32.0;
const REFERENCE_SEPTUM_D: f64 = 18.0;

const SPLIT_CENTER: (f64, f64) = (280.0, -55.0);
const SPLIT_X: f64 = 340.0;
const SPLIT_Y: f64 = 230.0;
const SPLIT_Z: f64 = 62.0;
const SPLIT_LANES: usize = LANES;
const WASTE_PORTS: usize = SPLIT_LANES;
const RETAIN_PORTS: usize = SPLIT_LANES;
const SPLIT_RATIO_MARKERS: usize = 5;
const SPLIT_CAPTURE_VOLUME_ML: f64 = 110.0;

const GATE_CENTER: (f64, f64) = (595.0, -55.0);
const GATE_X: f64 = 250.0;
const GATE_Y: f64 = 230.0;
const GATE_Z: f64 = 64.0;
const GATE_MODES: usize = 3;
const GATE_MODE_NAMES: [&str; GATE_MODES] = ["release", "hold", "reject"];
const GATE_SLOTS_PER_LANE: usize = GATE_MODES;
const GATE_SLOT_COUNT: usize = LANES * GATE_MODES;
const GATE_INTERLOCK_PINS: usize = LANES;

const TOKEN_CENTER: (f64, f64) = (-280.0, -350.0);
const TOKEN_X: f64 = 620.0;
const TOKEN_Y: f64 = 82.0;
const TOKEN_Z: f64 = 18.0;
const FLOW_TOKENS: usize = 12;
const TOKEN_SLOT_X: f64 = 34.0;
const TOKEN_SLOT_Y: f64 = 40.0;

const TRACE_CENTER: (f64, f64) = (305.0, -350.0);
const TRACE_X: f64 = 510.0;
const TRACE_Y: f64 = 82.0;
const TRACE_Z: f64 = 16.0;
const ROUTE_TRACE_SEGMENTS: usize = 10;
const BARCODE_LANDS: usize = 8;
const BUBBLE_EVENT_TOKEN_SLOTS: usize = 6;

const SERVICE_KEEP_OUT_GAUGES: usize = 6;
const FRONT_ROBOT_CLEARANCE: f64 = 360.0;
const REAR_CARTRIDGE_LIFT_CLEARANCE: f64 = 250.0;
const PRESSURE_PANEL_SERVICE_CLEARANCE: f64 = 210.0;
const WINDOW_CAMERA_CLEARANCE_Z: f64 = 245.0;
const GATE_ACCESS_CLEARANCE_X: f64 = 170.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_station(self) -> bool {
        self.center.0.abs() + self.x / 2.0 <= STATION_X / 2.0 - RIM_W - LAYOUT_CLEARANCE
            && self.center.1.abs() + self.y / 2.0 <= STATION_Y / 2.0 - RIM_W - LAYOUT_CLEARANCE
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

    let temp = temperature_step_media_cartridge_bank();
    export(OUTPUTS[1], &temp);

    let pressure = pressure_drop_restrictor_ladder();
    export(OUTPUTS[2], &pressure);

    let windows = transparent_bubble_witness_window_array();
    export(OUTPUTS[3], &windows);

    let bypass = degas_bypass_coupon_dock();
    export(OUTPUTS[4], &bypass);

    let references = gas_equilibrated_reference_pocket_block();
    export(OUTPUTS[5], &references);

    let split = waste_retain_split_manifold();
    export(OUTPUTS[6], &split);

    let gates = release_hold_reject_gate_bank();
    export(OUTPUTS[7], &gates);

    let tokens = flow_token_rail();
    export(OUTPUTS[8], &tokens);

    let trace = route_trace_overlay();
    export(OUTPUTS[9], &trace);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[10], &keepouts);

    let assembly = deck
        + temp
        + pressure
        + windows
        + bypass
        + references
        + split
        + gates
        + tokens
        + trace
        + keepouts;
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed media-line dissolved-gas bubble nucleation station:");
    println!("  Revision/units:              {PARAMETRIC_REVISION} / {UNITS}");
    println!("  Footprint:                   {STATION_X:.0}mm x {STATION_Y:.0}mm containment deck");
    println!(
        "  Challenge chain:             {TEMP_STEPS} temperature-step cartridges, {RESTRICTOR_STEPS} pressure restrictor steps, {WITNESS_WINDOWS} transparent bubble witness windows"
    );
    println!(
        "  Gas controls:                {DEGAS_COUPONS} degas/bypass coupon sockets and {REFERENCE_POCKET_COUNT} gas-equilibrated reference pockets"
    );
    println!(
        "  Disposition controls:        {FLOW_TOKENS} flow tokens, {WASTE_PORTS} waste ports, {RETAIN_PORTS} retain ports, {GATE_SLOT_COUNT} release/hold/reject gate slots"
    );
    println!(
        "  Safety boundary:             {} limitations and {} deterministic STL outputs",
        LIMITATIONS.len(),
        OUTPUTS.len()
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

fn lane_x(index: usize) -> f64 {
    centered_index(index, LANES, LANE_PITCH_X)
}

fn lane_y(index: usize) -> f64 {
    centered_index(index, LANES, LANE_PITCH_Y)
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn module_rects() -> [Rect; 9] {
    [
        rect(
            "temperature_step_media_cartridge_bank",
            TEMP_CENTER,
            TEMP_X,
            TEMP_Y,
        ),
        rect(
            "pressure_drop_restrictor_ladder",
            PRESSURE_CENTER,
            PRESSURE_X,
            PRESSURE_Y,
        ),
        rect(
            "transparent_bubble_witness_window_array",
            WITNESS_CENTER,
            WITNESS_X,
            WITNESS_Y,
        ),
        rect(
            "degas_bypass_coupon_dock",
            BYPASS_CENTER,
            BYPASS_X,
            BYPASS_Y,
        ),
        rect(
            "gas_equilibrated_reference_pocket_block",
            REF_CENTER,
            REF_X,
            REF_Y,
        ),
        rect(
            "waste_retain_split_manifold",
            SPLIT_CENTER,
            SPLIT_X,
            SPLIT_Y,
        ),
        rect("release_hold_reject_gate_bank", GATE_CENTER, GATE_X, GATE_Y),
        rect("flow_token_rail", TOKEN_CENTER, TOKEN_X, TOKEN_Y),
        rect("route_trace_overlay", TRACE_CENTER, TRACE_X, TRACE_Y),
    ]
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        "dissolved_gas_bubble_nucleation_containment_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let basin = centered_cube(
        "dissolved_gas_bubble_nucleation_shallow_basin_recess",
        STATION_X - RIM_W * 2.0 - 54.0,
        STATION_Y - RIM_W * 2.0 - 54.0,
        BASIN_RECESS_Z,
    )
    .translate(0.0, -4.0, deck_top_z() - BASIN_RECESS_Z / 2.0 + 0.2);
    let drain = centered_cylinder(
        "dissolved_gas_bubble_nucleation_front_basin_drain",
        8.0,
        72.0,
        DEFAULT_SEGMENTS,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 90.0,
        -STATION_Y / 2.0 + 18.0,
        deck_top_z() - 5.0,
    );

    deck - basin - drain - module_insert_sockets() - mounting_holes()
        + perimeter_rims()
        + datum_targets()
        + deck_route_rails()
}

fn module_insert_sockets() -> Part {
    let mut sockets = Part::empty("dissolved_gas_bubble_nucleation_module_insert_sockets");
    for module in module_rects() {
        sockets = sockets
            + centered_cube(
                format!("dissolved_gas_bubble_nucleation_{}_socket", module.name),
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

fn mounting_holes() -> Part {
    let mut holes = Part::empty("dissolved_gas_bubble_nucleation_mounting_holes");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 62.0, -STATION_Y / 2.0 + 56.0),
        (STATION_X / 2.0 - 62.0, -STATION_Y / 2.0 + 56.0),
        (-STATION_X / 2.0 + 62.0, STATION_Y / 2.0 - 56.0),
        (STATION_X / 2.0 - 62.0, STATION_Y / 2.0 - 56.0),
        (-250.0, 72.0),
        (250.0, 72.0),
        (-250.0, -230.0),
        (250.0, -230.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("dissolved_gas_bubble_nucleation_mount_clearance_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 6.0,
                24,
            )
            .translate(*x, *y, BASE_Z / 2.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    let left = centered_cube(
        "dissolved_gas_bubble_nucleation_left_spill_rim",
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
        "dissolved_gas_bubble_nucleation_right_spill_rim",
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
        "dissolved_gas_bubble_nucleation_rear_service_rim",
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
        "dissolved_gas_bubble_nucleation_low_front_robot_lip",
        STATION_X - 220.0,
        12.0,
        22.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 16.0, deck_top_z() + 11.0);
    left + right + rear + front
}

fn datum_targets() -> Part {
    let mut targets = Part::empty("dissolved_gas_bubble_nucleation_datum_targets");
    for (i, (x, y)) in [
        (-690.0, -408.0),
        (690.0, -408.0),
        (-690.0, 408.0),
        (690.0, 408.0),
        (TEMP_CENTER.0, TEMP_CENTER.1),
        (WITNESS_CENTER.0, WITNESS_CENTER.1),
    ]
    .iter()
    .enumerate()
    {
        let ring = centered_cylinder(
            format!("dissolved_gas_bubble_nucleation_datum_ring_{i}"),
            DATUM_RING_D / 2.0,
            4.0,
            DEFAULT_SEGMENTS,
        )
        .translate(*x, *y, deck_top_z() + 2.0)
            - centered_cylinder(
                format!("dissolved_gas_bubble_nucleation_datum_dot_{i}"),
                4.0,
                6.0,
                24,
            )
            .translate(*x, *y, deck_top_z() + 2.0);
        targets = targets + ring;
    }
    targets
}

fn deck_route_rails() -> Part {
    let challenge_rail = centered_cube(
        "dissolved_gas_bubble_nucleation_temperature_pressure_window_route_rail",
        1120.0,
        8.0,
        9.0,
    )
    .translate(0.0, 110.0, deck_top_z() + 4.5);
    let bypass_rail = centered_cube(
        "dissolved_gas_bubble_nucleation_bypass_reference_split_route_rail",
        1120.0,
        8.0,
        9.0,
    )
    .translate(40.0, -205.0, deck_top_z() + 4.5);
    let trace_rail = centered_cube(
        "dissolved_gas_bubble_nucleation_token_trace_locator_rail",
        1120.0,
        8.0,
        9.0,
    )
    .translate(15.0, -292.0, deck_top_z() + 4.5);
    challenge_rail + bypass_rail + trace_rail + deck_flow_arrows()
}

fn deck_flow_arrows() -> Part {
    let mut arrows = Part::empty("dissolved_gas_bubble_nucleation_deck_flow_arrows");
    for i in 0..LANES {
        let x = lane_x(i);
        let top_arrow = centered_cube(
            format!("dissolved_gas_bubble_nucleation_top_lane_{i}_flow_arrow_tail"),
            34.0,
            7.0,
            4.0,
        )
        .translate(x, 112.0, deck_top_z() + 8.0)
            + centered_cube(
                format!("dissolved_gas_bubble_nucleation_top_lane_{i}_flow_arrow_head"),
                9.0,
                20.0,
                4.0,
            )
            .rotate(0.0, 0.0, 45.0)
            .translate(x + 20.0, 112.0, deck_top_z() + 8.0);
        let lower_arrow = centered_cube(
            format!("dissolved_gas_bubble_nucleation_lower_lane_{i}_flow_arrow_tail"),
            34.0,
            7.0,
            4.0,
        )
        .translate(x + 42.0, -206.0, deck_top_z() + 8.0)
            + centered_cube(
                format!("dissolved_gas_bubble_nucleation_lower_lane_{i}_flow_arrow_head"),
                9.0,
                20.0,
                4.0,
            )
            .rotate(0.0, 0.0, -45.0)
            .translate(x + 62.0, -206.0, deck_top_z() + 8.0);
        arrows = arrows + top_arrow + lower_arrow;
    }
    arrows
}

fn temperature_step_media_cartridge_bank() -> Part {
    let body = centered_cube(
        "dissolved_gas_bubble_nucleation_temperature_step_cartridge_bank_body",
        TEMP_X,
        TEMP_Y,
        TEMP_Z,
    )
    .translate(TEMP_CENTER.0, TEMP_CENTER.1, place_z(TEMP_Z));
    body - temperature_cartridge_socket_cuts() - temperature_probe_well_cuts()
        + temperature_cartridge_rims()
        + temperature_step_heat_sink_fins()
        + temperature_port_bosses()
        + temperature_step_token_tabs()
}

fn temperature_cartridge_socket_cuts() -> Part {
    let mut cuts = Part::empty("dissolved_gas_bubble_nucleation_temp_cartridge_socket_cuts");
    for step in 0..TEMP_STEPS {
        let x = TEMP_CENTER.0 + centered_index(step, TEMP_STEPS, 66.0);
        cuts = cuts
            + centered_cylinder(
                format!(
                    "dissolved_gas_bubble_nucleation_{}_media_cartridge_socket",
                    TEMP_STEP_LABELS[step]
                ),
                TEMP_CARTRIDGE_D / 2.0,
                TEMP_CARTRIDGE_Z,
                DEFAULT_SEGMENTS,
            )
            .translate(x, TEMP_CENTER.1, deck_top_z() + TEMP_Z / 2.0 + 6.0)
            + centered_cylinder(
                format!(
                    "dissolved_gas_bubble_nucleation_{}_cartridge_media_bore",
                    TEMP_STEP_LABELS[step]
                ),
                FLUID_BORE_D / 2.0,
                TEMP_Y + 16.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, TEMP_CENTER.1, deck_top_z() + 30.0);
    }
    cuts
}

fn temperature_probe_well_cuts() -> Part {
    let mut cuts = Part::empty("dissolved_gas_bubble_nucleation_temperature_probe_well_cuts");
    for probe in 0..TEMP_PROBE_WELLS {
        let step = probe / 2;
        let side = probe % 2;
        let x = TEMP_CENTER.0 + centered_index(step, TEMP_STEPS, 66.0);
        let y = TEMP_CENTER.1 + if side == 0 { -48.0 } else { 48.0 };
        cuts = cuts
            + centered_cylinder(
                format!("dissolved_gas_bubble_nucleation_temperature_probe_well_{probe}"),
                5.0,
                TEMP_Z + 8.0,
                20,
            )
            .translate(x, y, place_z(TEMP_Z));
    }
    cuts
}

fn temperature_cartridge_rims() -> Part {
    let mut rims = Part::empty("dissolved_gas_bubble_nucleation_temperature_cartridge_rims");
    for step in 0..TEMP_STEPS {
        let x = TEMP_CENTER.0 + centered_index(step, TEMP_STEPS, 66.0);
        let rim = centered_cylinder(
            format!(
                "dissolved_gas_bubble_nucleation_{}_cartridge_retention_rim",
                TEMP_STEP_LABELS[step]
            ),
            TEMP_CARTRIDGE_D / 2.0 + 7.0,
            6.0,
            DEFAULT_SEGMENTS,
        )
        .translate(x, TEMP_CENTER.1, deck_top_z() + TEMP_Z + 3.0)
            - centered_cylinder(
                format!(
                    "dissolved_gas_bubble_nucleation_{}_cartridge_rim_clearance",
                    TEMP_STEP_LABELS[step]
                ),
                TEMP_CARTRIDGE_D / 2.0,
                8.0,
                DEFAULT_SEGMENTS,
            )
            .translate(x, TEMP_CENTER.1, deck_top_z() + TEMP_Z + 3.0);
        rims = rims + rim;
    }
    rims
}

fn temperature_step_heat_sink_fins() -> Part {
    let mut fins = Part::empty("dissolved_gas_bubble_nucleation_temperature_heat_sink_fins");
    for step in 0..TEMP_STEPS {
        let x = TEMP_CENTER.0 + centered_index(step, TEMP_STEPS, 66.0);
        let fin_height = 10.0 + step as f64 * 2.0;
        fins =
            fins + centered_cube(
                format!("dissolved_gas_bubble_nucleation_temperature_step_{step}_fin_a"),
                48.0,
                5.0,
                fin_height,
            )
            .translate(
                x,
                TEMP_CENTER.1 - 78.0,
                deck_top_z() + TEMP_Z + fin_height / 2.0,
            ) + centered_cube(
                format!("dissolved_gas_bubble_nucleation_temperature_step_{step}_fin_b"),
                48.0,
                5.0,
                fin_height,
            )
            .translate(
                x,
                TEMP_CENTER.1 + 78.0,
                deck_top_z() + TEMP_Z + fin_height / 2.0,
            );
    }
    fins
}

fn temperature_port_bosses() -> Part {
    let inlet = centered_cylinder(
        "dissolved_gas_bubble_nucleation_temperature_bank_conditioned_media_inlet",
        13.0,
        24.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(
        TEMP_CENTER.0 - TEMP_X / 2.0 - 12.0,
        TEMP_CENTER.1,
        deck_top_z() + TEMP_Z / 2.0,
    );
    let outlet = centered_cylinder(
        "dissolved_gas_bubble_nucleation_temperature_bank_conditioned_media_outlet",
        13.0,
        24.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(
        TEMP_CENTER.0 + TEMP_X / 2.0 + 12.0,
        TEMP_CENTER.1,
        deck_top_z() + TEMP_Z / 2.0,
    );
    inlet + outlet
}

fn temperature_step_token_tabs() -> Part {
    let mut tabs = Part::empty("dissolved_gas_bubble_nucleation_temperature_step_token_tabs");
    for step in 0..TEMP_STEPS {
        let x = TEMP_CENTER.0 + centered_index(step, TEMP_STEPS, 66.0);
        tabs = tabs
            + centered_cube(
                format!(
                    "dissolved_gas_bubble_nucleation_{}_temperature_step_token_tab",
                    TEMP_STEP_LABELS[step]
                ),
                42.0,
                20.0,
                6.0,
            )
            .translate(
                x,
                TEMP_CENTER.1 + TEMP_Y / 2.0 - 22.0,
                deck_top_z() + TEMP_Z + 3.0,
            );
    }
    tabs
}

fn pressure_drop_restrictor_ladder() -> Part {
    let body = centered_cube(
        "dissolved_gas_bubble_nucleation_pressure_drop_restrictor_ladder_body",
        PRESSURE_X,
        PRESSURE_Y,
        PRESSURE_Z,
    )
    .translate(PRESSURE_CENTER.0, PRESSURE_CENTER.1, place_z(PRESSURE_Z));
    body - restrictor_bores() - pressure_tap_bores()
        + restrictor_stage_blocks()
        + pressure_tap_gasket_rings()
        + differential_pressure_bridge_rails()
        + pressure_drop_limit_markers()
}

fn restrictor_bores() -> Part {
    let mut bores = Part::empty("dissolved_gas_bubble_nucleation_restrictor_bores");
    for step in 0..RESTRICTOR_STEPS {
        let x = PRESSURE_CENTER.0 + centered_index(step, RESTRICTOR_STEPS, 56.0);
        let bore_d = RESTRICTOR_MIN_BORE_D
            + (RESTRICTOR_MAX_BORE_D - RESTRICTOR_MIN_BORE_D) * step as f64
                / (RESTRICTOR_STEPS - 1) as f64;
        bores = bores
            + centered_cylinder(
                format!("dissolved_gas_bubble_nucleation_restrictor_step_{step}_bore"),
                bore_d / 2.0,
                PRESSURE_Y + 18.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, PRESSURE_CENTER.1, deck_top_z() + 34.0);
    }
    bores
}

fn pressure_tap_bores() -> Part {
    let mut bores = Part::empty("dissolved_gas_bubble_nucleation_pressure_tap_bores");
    for tap in 0..RESTRICTOR_PRESSURE_TAPS {
        let x = PRESSURE_CENTER.0 + centered_index(tap, RESTRICTOR_PRESSURE_TAPS, 47.0);
        bores = bores
            + centered_cylinder(
                format!("dissolved_gas_bubble_nucleation_pressure_tap_{tap}_bore"),
                4.0,
                PRESSURE_Z + 10.0,
                20,
            )
            .translate(x, PRESSURE_CENTER.1 + 70.0, place_z(PRESSURE_Z));
    }
    bores
}

fn restrictor_stage_blocks() -> Part {
    let mut blocks = Part::empty("dissolved_gas_bubble_nucleation_restrictor_stage_blocks");
    for step in 0..RESTRICTOR_STEPS {
        let x = PRESSURE_CENTER.0 + centered_index(step, RESTRICTOR_STEPS, 56.0);
        blocks = blocks
            + centered_cube(
                format!("dissolved_gas_bubble_nucleation_restrictor_step_{step}_ladder_plinth"),
                38.0,
                42.0,
                8.0 + step as f64 * 2.0,
            )
            .translate(
                x,
                PRESSURE_CENTER.1 - 58.0,
                deck_top_z() + PRESSURE_Z + 4.0 + step as f64,
            );
    }
    blocks
}

fn pressure_tap_gasket_rings() -> Part {
    let mut rings = Part::empty("dissolved_gas_bubble_nucleation_pressure_tap_gasket_rings");
    for tap in 0..RESTRICTOR_PRESSURE_TAPS {
        let x = PRESSURE_CENTER.0 + centered_index(tap, RESTRICTOR_PRESSURE_TAPS, 47.0);
        let ring = centered_cylinder(
            format!("dissolved_gas_bubble_nucleation_pressure_tap_{tap}_gasket_outer"),
            12.0,
            5.0,
            28,
        )
        .translate(x, PRESSURE_CENTER.1 + 70.0, deck_top_z() + PRESSURE_Z + 2.5)
            - centered_cylinder(
                format!("dissolved_gas_bubble_nucleation_pressure_tap_{tap}_gasket_inner"),
                4.5,
                7.0,
                20,
            )
            .translate(x, PRESSURE_CENTER.1 + 70.0, deck_top_z() + PRESSURE_Z + 2.5);
        rings = rings + ring;
    }
    rings
}

fn differential_pressure_bridge_rails() -> Part {
    let upper = centered_cube(
        "dissolved_gas_bubble_nucleation_differential_pressure_upper_bridge_rail",
        PRESSURE_X - 62.0,
        8.0,
        10.0,
    )
    .translate(
        PRESSURE_CENTER.0,
        PRESSURE_CENTER.1 + 100.0,
        deck_top_z() + PRESSURE_Z + 5.0,
    );
    let lower = centered_cube(
        "dissolved_gas_bubble_nucleation_differential_pressure_lower_bridge_rail",
        PRESSURE_X - 62.0,
        8.0,
        10.0,
    )
    .translate(
        PRESSURE_CENTER.0,
        PRESSURE_CENTER.1 - 100.0,
        deck_top_z() + PRESSURE_Z + 5.0,
    );
    upper + lower
}

fn pressure_drop_limit_markers() -> Part {
    let mut markers = Part::empty("dissolved_gas_bubble_nucleation_pressure_limit_markers");
    for marker in 0..3 {
        markers = markers
            + centered_cube(
                format!("dissolved_gas_bubble_nucleation_pressure_limit_marker_{marker}"),
                24.0,
                16.0,
                8.0 + marker as f64 * 4.0,
            )
            .translate(
                PRESSURE_CENTER.0 + centered_index(marker, 3, 34.0),
                PRESSURE_CENTER.1,
                deck_top_z() + PRESSURE_Z + 4.0 + marker as f64 * 2.0,
            );
    }
    markers
}

fn transparent_bubble_witness_window_array() -> Part {
    let body = centered_cube(
        "dissolved_gas_bubble_nucleation_transparent_witness_window_array_body",
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    )
    .translate(WITNESS_CENTER.0, WITNESS_CENTER.1, place_z(WITNESS_Z));
    body - bubble_witness_window_cuts() - backlight_slot_cuts()
        + bubble_witness_window_frames()
        + bubble_size_comparator_dots()
        + camera_datum_bridge()
}

fn bubble_witness_window_cuts() -> Part {
    let mut cuts = Part::empty("dissolved_gas_bubble_nucleation_witness_window_cuts");
    for lane in 0..WITNESS_WINDOWS {
        let x = WITNESS_CENTER.0 + centered_index(lane, WITNESS_WINDOWS, 54.0);
        cuts = cuts
            + centered_cube(
                format!("dissolved_gas_bubble_nucleation_lane_{lane}_transparent_window_cut"),
                WINDOW_X,
                WINDOW_Y,
                WITNESS_Z + 8.0,
            )
            .translate(x, WITNESS_CENTER.1, place_z(WITNESS_Z));
    }
    cuts
}

fn backlight_slot_cuts() -> Part {
    let mut slots = Part::empty("dissolved_gas_bubble_nucleation_backlight_slot_cuts");
    for lane in 0..BACKLIGHT_SLOTS {
        let x = WITNESS_CENTER.0 + centered_index(lane, BACKLIGHT_SLOTS, 54.0);
        slots = slots
            + centered_cube(
                format!("dissolved_gas_bubble_nucleation_lane_{lane}_backlight_slot"),
                24.0,
                16.0,
                WITNESS_Z + 10.0,
            )
            .translate(
                x,
                WITNESS_CENTER.1 - WITNESS_Y / 2.0 + 24.0,
                place_z(WITNESS_Z),
            );
    }
    slots
}

fn bubble_witness_window_frames() -> Part {
    let mut frames = Part::empty("dissolved_gas_bubble_nucleation_witness_window_frames");
    for lane in 0..WITNESS_WINDOWS {
        let x = WITNESS_CENTER.0 + centered_index(lane, WITNESS_WINDOWS, 54.0);
        frames = frames
            + rectangular_frame(
                &format!("dissolved_gas_bubble_nucleation_lane_{lane}_window_frame"),
                WINDOW_X + 14.0,
                WINDOW_Y + 14.0,
                WINDOW_FRAME_RAIL,
                7.0,
            )
            .translate(x, WITNESS_CENTER.1, deck_top_z() + WITNESS_Z + 3.5);
    }
    frames
}

fn bubble_size_comparator_dots() -> Part {
    let mut dots = Part::empty("dissolved_gas_bubble_nucleation_bubble_size_comparator_dots");
    for lane in 0..WITNESS_WINDOWS {
        let x = WITNESS_CENTER.0 + centered_index(lane, WITNESS_WINDOWS, 54.0);
        for dot in 0..BUBBLE_COMPARATOR_DOTS {
            dots = dots
                + centered_cylinder(
                    format!(
                        "dissolved_gas_bubble_nucleation_lane_{lane}_bubble_comparator_dot_{dot}"
                    ),
                    2.0 + dot as f64 * 1.3,
                    2.4,
                    20,
                )
                .translate(
                    x,
                    WITNESS_CENTER.1 - 46.0 + dot as f64 * 18.0,
                    deck_top_z() + WITNESS_Z + 8.2,
                );
        }
    }
    dots
}

fn camera_datum_bridge() -> Part {
    let bridge = centered_cube(
        "dissolved_gas_bubble_nucleation_window_camera_datum_bridge",
        WITNESS_X - 52.0,
        12.0,
        12.0,
    )
    .translate(
        WITNESS_CENTER.0,
        WITNESS_CENTER.1 + WITNESS_Y / 2.0 - 24.0,
        deck_top_z() + WITNESS_Z + 6.0,
    );
    let mut targets = Part::empty("dissolved_gas_bubble_nucleation_window_camera_targets");
    for i in 0..4 {
        targets = targets
            + centered_cylinder(
                format!("dissolved_gas_bubble_nucleation_window_camera_target_{i}"),
                8.0,
                4.0,
                24,
            )
            .translate(
                WITNESS_CENTER.0 + centered_index(i, 4, 92.0),
                WITNESS_CENTER.1 + WITNESS_Y / 2.0 - 24.0,
                deck_top_z() + WITNESS_Z + 14.0,
            );
    }
    bridge + targets
}

fn degas_bypass_coupon_dock() -> Part {
    let body = centered_cube(
        "dissolved_gas_bubble_nucleation_degas_bypass_coupon_dock_body",
        BYPASS_X,
        BYPASS_Y,
        BYPASS_Z,
    )
    .translate(BYPASS_CENTER.0, BYPASS_CENTER.1, place_z(BYPASS_Z));
    body - bypass_coupon_socket_cuts() - bypass_line_bores()
        + bypass_coupon_retention_frames()
        + bypass_selector_valve_markers()
        + bypass_pressure_equalization_bridge()
}

fn bypass_coupon_socket_cuts() -> Part {
    let mut cuts = Part::empty("dissolved_gas_bubble_nucleation_bypass_coupon_socket_cuts");
    for coupon in 0..BYPASS_COUPON_SOCKETS {
        let x = BYPASS_CENTER.0 + centered_index(coupon, BYPASS_COUPON_SOCKETS, 95.0);
        cuts = cuts
            + centered_cube(
                format!("dissolved_gas_bubble_nucleation_degas_bypass_coupon_{coupon}_socket"),
                COUPON_SOCKET_X,
                COUPON_SOCKET_Y,
                BYPASS_Z - 16.0,
            )
            .translate(x, BYPASS_CENTER.1, deck_top_z() + BYPASS_Z / 2.0 + 7.0);
    }
    cuts
}

fn bypass_line_bores() -> Part {
    let main = centered_cylinder(
        "dissolved_gas_bubble_nucleation_degas_bypass_main_line_bore",
        FLUID_BORE_D / 2.0,
        BYPASS_X + 24.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(BYPASS_CENTER.0, BYPASS_CENTER.1 - 72.0, deck_top_z() + 28.0);
    let bypass = centered_cylinder(
        "dissolved_gas_bubble_nucleation_degas_bypass_parallel_line_bore",
        FLUID_BORE_D / 2.0,
        BYPASS_X + 24.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(BYPASS_CENTER.0, BYPASS_CENTER.1 + 72.0, deck_top_z() + 28.0);
    main + bypass
}

fn bypass_coupon_retention_frames() -> Part {
    let mut frames = Part::empty("dissolved_gas_bubble_nucleation_bypass_coupon_frames");
    for coupon in 0..DEGAS_COUPONS {
        let x = BYPASS_CENTER.0 + centered_index(coupon, DEGAS_COUPONS, 95.0);
        frames = frames
            + rectangular_frame(
                &format!("dissolved_gas_bubble_nucleation_degas_coupon_{coupon}_retention_frame"),
                COUPON_SOCKET_X + 18.0,
                COUPON_SOCKET_Y + 18.0,
                6.0,
                8.0,
            )
            .translate(x, BYPASS_CENTER.1, deck_top_z() + BYPASS_Z + 4.0);
    }
    frames
}

fn bypass_selector_valve_markers() -> Part {
    let mut valves = Part::empty("dissolved_gas_bubble_nucleation_bypass_selector_valves");
    for valve in 0..BYPASS_SELECTOR_VALVES {
        let x = BYPASS_CENTER.0 + centered_index(valve, BYPASS_SELECTOR_VALVES, 250.0);
        valves = valves
            + centered_cylinder(
                format!("dissolved_gas_bubble_nucleation_bypass_selector_valve_{valve}_knob"),
                18.0,
                12.0,
                32,
            )
            .translate(x, BYPASS_CENTER.1, deck_top_z() + BYPASS_Z + 6.0)
            + centered_cube(
                format!("dissolved_gas_bubble_nucleation_bypass_selector_valve_{valve}_pointer"),
                34.0,
                7.0,
                7.0,
            )
            .rotate(0.0, 0.0, if valve == 0 { 35.0 } else { -35.0 })
            .translate(x, BYPASS_CENTER.1, deck_top_z() + BYPASS_Z + 18.0);
    }
    valves
}

fn bypass_pressure_equalization_bridge() -> Part {
    centered_cube(
        "dissolved_gas_bubble_nucleation_bypass_pressure_equalization_bridge",
        BYPASS_X - 62.0,
        10.0,
        10.0,
    )
    .translate(
        BYPASS_CENTER.0,
        BYPASS_CENTER.1,
        deck_top_z() + BYPASS_Z + 5.0,
    )
}

fn gas_equilibrated_reference_pocket_block() -> Part {
    let body = centered_cube(
        "dissolved_gas_bubble_nucleation_gas_equilibrated_reference_block_body",
        REF_X,
        REF_Y,
        REF_Z,
    )
    .translate(REF_CENTER.0, REF_CENTER.1, place_z(REF_Z));
    body - reference_pocket_cuts()
        + reference_pocket_rings()
        + reference_septum_caps()
        + gas_reference_identity_tabs()
}

fn reference_pocket_cuts() -> Part {
    let mut cuts = Part::empty("dissolved_gas_bubble_nucleation_reference_pocket_cuts");
    for pocket in 0..REFERENCE_POCKET_COUNT {
        let gas = pocket / REFERENCE_REPLICATES;
        let replicate = pocket % REFERENCE_REPLICATES;
        let x = REF_CENTER.0 + centered_index(gas, GAS_REFERENCE_POCKETS, 68.0);
        let y = REF_CENTER.1 + centered_index(replicate, REFERENCE_REPLICATES, 62.0);
        cuts = cuts
            + centered_cylinder(
                format!("dissolved_gas_bubble_nucleation_reference_pocket_{pocket}_cut"),
                REFERENCE_POCKET_D / 2.0,
                REF_Z + 10.0,
                DEFAULT_SEGMENTS,
            )
            .translate(x, y, place_z(REF_Z));
    }
    cuts
}

fn reference_pocket_rings() -> Part {
    let mut rings = Part::empty("dissolved_gas_bubble_nucleation_reference_pocket_rings");
    for pocket in 0..REFERENCE_POCKET_COUNT {
        let gas = pocket / REFERENCE_REPLICATES;
        let replicate = pocket % REFERENCE_REPLICATES;
        let x = REF_CENTER.0 + centered_index(gas, GAS_REFERENCE_POCKETS, 68.0);
        let y = REF_CENTER.1 + centered_index(replicate, REFERENCE_REPLICATES, 62.0);
        let ring = centered_cylinder(
            format!("dissolved_gas_bubble_nucleation_reference_pocket_{pocket}_outer_ring"),
            REFERENCE_POCKET_D / 2.0 + 6.0,
            6.0,
            DEFAULT_SEGMENTS,
        )
        .translate(x, y, deck_top_z() + REF_Z + 3.0)
            - centered_cylinder(
                format!("dissolved_gas_bubble_nucleation_reference_pocket_{pocket}_inner_clear"),
                REFERENCE_POCKET_D / 2.0,
                8.0,
                DEFAULT_SEGMENTS,
            )
            .translate(x, y, deck_top_z() + REF_Z + 3.0);
        rings = rings + ring;
    }
    rings
}

fn reference_septum_caps() -> Part {
    let mut caps = Part::empty("dissolved_gas_bubble_nucleation_reference_septum_caps");
    for gas in 0..GAS_REFERENCE_POCKETS {
        let x = REF_CENTER.0 + centered_index(gas, GAS_REFERENCE_POCKETS, 68.0);
        caps = caps
            + centered_cylinder(
                format!("dissolved_gas_bubble_nucleation_reference_gas_{gas}_septum_cap"),
                REFERENCE_SEPTUM_D / 2.0,
                8.0,
                24,
            )
            .translate(
                x,
                REF_CENTER.1 + REF_Y / 2.0 - 30.0,
                deck_top_z() + REF_Z + 4.0,
            );
    }
    caps
}

fn gas_reference_identity_tabs() -> Part {
    let mut tabs = Part::empty("dissolved_gas_bubble_nucleation_gas_reference_identity_tabs");
    for gas in 0..GAS_REFERENCE_POCKETS {
        let x = REF_CENTER.0 + centered_index(gas, GAS_REFERENCE_POCKETS, 68.0);
        tabs = tabs
            + centered_cube(
                format!("dissolved_gas_bubble_nucleation_reference_gas_{gas}_identity_tab"),
                42.0,
                16.0,
                6.0,
            )
            .translate(
                x,
                REF_CENTER.1 - REF_Y / 2.0 + 25.0,
                deck_top_z() + REF_Z + 3.0,
            );
    }
    tabs
}

fn waste_retain_split_manifold() -> Part {
    let body = centered_cube(
        "dissolved_gas_bubble_nucleation_waste_retain_split_manifold_body",
        SPLIT_X,
        SPLIT_Y,
        SPLIT_Z,
    )
    .translate(SPLIT_CENTER.0, SPLIT_CENTER.1, place_z(SPLIT_Z));
    body - split_manifold_bores() - waste_retain_port_cuts()
        + waste_retain_gasket_lands()
        + split_ratio_marker_blocks()
        + split_capture_cup_lands()
}

fn split_manifold_bores() -> Part {
    let mut bores = Part::empty("dissolved_gas_bubble_nucleation_split_manifold_bores");
    for lane in 0..SPLIT_LANES {
        let y = SPLIT_CENTER.1 + lane_y(lane);
        let feed = centered_cylinder(
            format!("dissolved_gas_bubble_nucleation_split_lane_{lane}_feed_bore"),
            FLUID_BORE_D / 2.0,
            SPLIT_X + 16.0,
            24,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(SPLIT_CENTER.0, y, deck_top_z() + 30.0);
        let waste = centered_cylinder(
            format!("dissolved_gas_bubble_nucleation_split_lane_{lane}_waste_branch_bore"),
            FLUID_BORE_D / 2.0,
            95.0,
            24,
        )
        .rotate(35.0, 0.0, 0.0)
        .translate(SPLIT_CENTER.0 + 48.0, y + 16.0, deck_top_z() + 30.0);
        let retain = centered_cylinder(
            format!("dissolved_gas_bubble_nucleation_split_lane_{lane}_retain_branch_bore"),
            FLUID_BORE_D / 2.0,
            95.0,
            24,
        )
        .rotate(-35.0, 0.0, 0.0)
        .translate(SPLIT_CENTER.0 + 48.0, y - 16.0, deck_top_z() + 30.0);
        bores = bores + feed + waste + retain;
    }
    bores
}

fn waste_retain_port_cuts() -> Part {
    let mut ports = Part::empty("dissolved_gas_bubble_nucleation_waste_retain_port_cuts");
    for lane in 0..SPLIT_LANES {
        let y = SPLIT_CENTER.1 + lane_y(lane);
        ports = ports
            + centered_cylinder(
                format!("dissolved_gas_bubble_nucleation_split_lane_{lane}_waste_port_cut"),
                5.0,
                SPLIT_Z + 8.0,
                20,
            )
            .translate(SPLIT_CENTER.0 + 118.0, y + 14.0, place_z(SPLIT_Z))
            + centered_cylinder(
                format!("dissolved_gas_bubble_nucleation_split_lane_{lane}_retain_port_cut"),
                5.0,
                SPLIT_Z + 8.0,
                20,
            )
            .translate(SPLIT_CENTER.0 + 118.0, y - 14.0, place_z(SPLIT_Z));
    }
    ports
}

fn waste_retain_gasket_lands() -> Part {
    let mut lands = Part::empty("dissolved_gas_bubble_nucleation_waste_retain_gasket_lands");
    for lane in 0..SPLIT_LANES {
        let y = SPLIT_CENTER.1 + lane_y(lane);
        for (branch, y_offset) in [("waste", 14.0), ("retain", -14.0)] {
            let land = centered_cylinder(
                format!("dissolved_gas_bubble_nucleation_split_lane_{lane}_{branch}_gasket_land"),
                11.0,
                5.0,
                24,
            )
            .translate(
                SPLIT_CENTER.0 + 118.0,
                y + y_offset,
                deck_top_z() + SPLIT_Z + 2.5,
            ) - centered_cylinder(
                format!("dissolved_gas_bubble_nucleation_split_lane_{lane}_{branch}_gasket_clear"),
                5.2,
                7.0,
                20,
            )
            .translate(
                SPLIT_CENTER.0 + 118.0,
                y + y_offset,
                deck_top_z() + SPLIT_Z + 2.5,
            );
            lands = lands + land;
        }
    }
    lands
}

fn split_ratio_marker_blocks() -> Part {
    let mut markers = Part::empty("dissolved_gas_bubble_nucleation_split_ratio_markers");
    for marker in 0..SPLIT_RATIO_MARKERS {
        markers = markers
            + centered_cube(
                format!("dissolved_gas_bubble_nucleation_split_ratio_marker_{marker}"),
                22.0,
                14.0,
                8.0 + marker as f64 * 2.0,
            )
            .translate(
                SPLIT_CENTER.0 + centered_index(marker, SPLIT_RATIO_MARKERS, 30.0) - 58.0,
                SPLIT_CENTER.1 + SPLIT_Y / 2.0 - 28.0,
                deck_top_z() + SPLIT_Z + 4.0 + marker as f64,
            );
    }
    markers
}

fn split_capture_cup_lands() -> Part {
    let waste_cup = centered_cylinder(
        "dissolved_gas_bubble_nucleation_waste_split_capture_cup_land",
        24.0,
        8.0,
        DEFAULT_SEGMENTS,
    )
    .translate(
        SPLIT_CENTER.0 - SPLIT_X / 2.0 + 42.0,
        SPLIT_CENTER.1 + 52.0,
        deck_top_z() + SPLIT_Z + 4.0,
    );
    let retain_cup = centered_cylinder(
        "dissolved_gas_bubble_nucleation_retain_split_capture_cup_land",
        24.0,
        8.0,
        DEFAULT_SEGMENTS,
    )
    .translate(
        SPLIT_CENTER.0 - SPLIT_X / 2.0 + 42.0,
        SPLIT_CENTER.1 - 52.0,
        deck_top_z() + SPLIT_Z + 4.0,
    );
    waste_cup + retain_cup
}

fn release_hold_reject_gate_bank() -> Part {
    let body = centered_cube(
        "dissolved_gas_bubble_nucleation_release_hold_reject_gate_bank_body",
        GATE_X,
        GATE_Y,
        GATE_Z,
    )
    .translate(GATE_CENTER.0, GATE_CENTER.1, place_z(GATE_Z));
    body - gate_slot_cuts() - gate_interlock_pin_cuts()
        + gate_mode_paddles()
        + gate_interlock_pin_bosses()
        + disposition_status_rail()
}

fn gate_slot_cuts() -> Part {
    let mut cuts = Part::empty("dissolved_gas_bubble_nucleation_gate_slot_cuts");
    for lane in 0..LANES {
        let y = GATE_CENTER.1 + lane_y(lane);
        for mode in 0..GATE_MODES {
            let x = GATE_CENTER.0 + centered_index(mode, GATE_MODES, 54.0);
            cuts = cuts
                + centered_cube(
                    format!(
                        "dissolved_gas_bubble_nucleation_lane_{lane}_{}_gate_slot_cut",
                        GATE_MODE_NAMES[mode]
                    ),
                    20.0,
                    16.0,
                    GATE_Z + 8.0,
                )
                .translate(x, y, place_z(GATE_Z));
        }
    }
    cuts
}

fn gate_interlock_pin_cuts() -> Part {
    let mut cuts = Part::empty("dissolved_gas_bubble_nucleation_gate_interlock_pin_cuts");
    for lane in 0..GATE_INTERLOCK_PINS {
        let y = GATE_CENTER.1 + lane_y(lane);
        cuts = cuts
            + centered_cylinder(
                format!("dissolved_gas_bubble_nucleation_lane_{lane}_gate_interlock_pin_cut"),
                4.0,
                GATE_Z + 8.0,
                20,
            )
            .translate(GATE_CENTER.0 + GATE_X / 2.0 - 32.0, y, place_z(GATE_Z));
    }
    cuts
}

fn gate_mode_paddles() -> Part {
    let mut paddles = Part::empty("dissolved_gas_bubble_nucleation_gate_mode_paddles");
    for mode in 0..GATE_MODES {
        let x = GATE_CENTER.0 + centered_index(mode, GATE_MODES, 54.0);
        paddles = paddles
            + centered_cube(
                format!(
                    "dissolved_gas_bubble_nucleation_{}_gate_mode_header_paddle",
                    GATE_MODE_NAMES[mode]
                ),
                42.0,
                22.0,
                9.0,
            )
            .translate(
                x,
                GATE_CENTER.1 + GATE_Y / 2.0 - 28.0,
                deck_top_z() + GATE_Z + 4.5,
            );
    }
    paddles
}

fn gate_interlock_pin_bosses() -> Part {
    let mut bosses = Part::empty("dissolved_gas_bubble_nucleation_gate_interlock_pin_bosses");
    for lane in 0..GATE_INTERLOCK_PINS {
        let y = GATE_CENTER.1 + lane_y(lane);
        bosses = bosses
            + centered_cylinder(
                format!("dissolved_gas_bubble_nucleation_lane_{lane}_gate_interlock_pin_boss"),
                10.0,
                7.0,
                24,
            )
            .translate(
                GATE_CENTER.0 + GATE_X / 2.0 - 32.0,
                y,
                deck_top_z() + GATE_Z + 3.5,
            );
    }
    bosses
}

fn disposition_status_rail() -> Part {
    centered_cube(
        "dissolved_gas_bubble_nucleation_release_hold_reject_disposition_status_rail",
        GATE_X - 40.0,
        10.0,
        10.0,
    )
    .translate(
        GATE_CENTER.0,
        GATE_CENTER.1 - GATE_Y / 2.0 + 28.0,
        deck_top_z() + GATE_Z + 5.0,
    )
}

fn flow_token_rail() -> Part {
    let body = centered_cube(
        "dissolved_gas_bubble_nucleation_flow_token_rail_body",
        TOKEN_X,
        TOKEN_Y,
        TOKEN_Z,
    )
    .translate(TOKEN_CENTER.0, TOKEN_CENTER.1, place_z(TOKEN_Z));
    body - flow_token_slot_cuts() + flow_token_status_tabs() + flow_direction_index_pips()
}

fn flow_token_slot_cuts() -> Part {
    let mut cuts = Part::empty("dissolved_gas_bubble_nucleation_flow_token_slot_cuts");
    for token in 0..FLOW_TOKENS {
        let x = TOKEN_CENTER.0 + centered_index(token, FLOW_TOKENS, 46.0);
        cuts = cuts
            + centered_cube(
                format!("dissolved_gas_bubble_nucleation_flow_token_{token}_slot_cut"),
                TOKEN_SLOT_X,
                TOKEN_SLOT_Y,
                TOKEN_Z + 6.0,
            )
            .translate(x, TOKEN_CENTER.1, place_z(TOKEN_Z));
    }
    cuts
}

fn flow_token_status_tabs() -> Part {
    let mut tabs = Part::empty("dissolved_gas_bubble_nucleation_flow_token_status_tabs");
    for token in 0..FLOW_TOKENS {
        let x = TOKEN_CENTER.0 + centered_index(token, FLOW_TOKENS, 46.0);
        tabs = tabs
            + centered_cube(
                format!("dissolved_gas_bubble_nucleation_flow_token_{token}_status_tab"),
                28.0,
                10.0,
                6.0,
            )
            .translate(
                x,
                TOKEN_CENTER.1 + TOKEN_Y / 2.0 - 12.0,
                deck_top_z() + TOKEN_Z + 3.0,
            );
    }
    tabs
}

fn flow_direction_index_pips() -> Part {
    let mut pips = Part::empty("dissolved_gas_bubble_nucleation_flow_direction_index_pips");
    for token in 0..FLOW_TOKENS {
        let x = TOKEN_CENTER.0 + centered_index(token, FLOW_TOKENS, 46.0);
        pips = pips
            + centered_cylinder(
                format!("dissolved_gas_bubble_nucleation_flow_token_{token}_index_pip"),
                3.0 + (token % 3) as f64,
                3.0,
                18,
            )
            .translate(
                x,
                TOKEN_CENTER.1 - TOKEN_Y / 2.0 + 13.0,
                deck_top_z() + TOKEN_Z + 4.5,
            );
    }
    pips
}

fn route_trace_overlay() -> Part {
    let body = centered_cube(
        "dissolved_gas_bubble_nucleation_route_trace_overlay_body",
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    )
    .translate(TRACE_CENTER.0, TRACE_CENTER.1, place_z(TRACE_Z));
    body + route_trace_segments() + barcode_lands() + bubble_event_token_slots()
}

fn route_trace_segments() -> Part {
    let mut segments = Part::empty("dissolved_gas_bubble_nucleation_route_trace_segments");
    for segment in 0..ROUTE_TRACE_SEGMENTS {
        let x = TRACE_CENTER.0 + centered_index(segment, ROUTE_TRACE_SEGMENTS, 46.0);
        let y = TRACE_CENTER.1 + if segment % 2 == 0 { -18.0 } else { 18.0 };
        let rail = centered_cube(
            format!("dissolved_gas_bubble_nucleation_route_trace_segment_{segment}"),
            34.0,
            6.0,
            7.0,
        )
        .rotate(0.0, 0.0, if segment % 2 == 0 { 12.0 } else { -12.0 })
        .translate(x, y, deck_top_z() + TRACE_Z + 3.5);
        segments = segments + rail;
    }
    segments
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("dissolved_gas_bubble_nucleation_route_trace_barcode_lands");
    for land in 0..BARCODE_LANDS {
        let x = TRACE_CENTER.0 + centered_index(land, BARCODE_LANDS, 52.0);
        lands = lands
            + centered_cube(
                format!("dissolved_gas_bubble_nucleation_trace_barcode_land_{land}"),
                38.0,
                16.0,
                5.0,
            )
            .translate(
                x,
                TRACE_CENTER.1 + TRACE_Y / 2.0 - 14.0,
                deck_top_z() + TRACE_Z + 2.5,
            );
    }
    lands
}

fn bubble_event_token_slots() -> Part {
    let mut slots = Part::empty("dissolved_gas_bubble_nucleation_event_token_slots");
    for slot in 0..BUBBLE_EVENT_TOKEN_SLOTS {
        let x = TRACE_CENTER.0 + centered_index(slot, BUBBLE_EVENT_TOKEN_SLOTS, 58.0);
        let socket = centered_cube(
            format!("dissolved_gas_bubble_nucleation_bubble_event_token_slot_{slot}_rim"),
            34.0,
            24.0,
            6.0,
        )
        .translate(
            x,
            TRACE_CENTER.1 - TRACE_Y / 2.0 + 18.0,
            deck_top_z() + TRACE_Z + 3.0,
        ) - centered_cube(
            format!("dissolved_gas_bubble_nucleation_bubble_event_token_slot_{slot}_clear"),
            24.0,
            14.0,
            8.0,
        )
        .translate(
            x,
            TRACE_CENTER.1 - TRACE_Y / 2.0 + 18.0,
            deck_top_z() + TRACE_Z + 3.0,
        );
        slots = slots + socket;
    }
    slots
}

fn robot_service_keepout_gauges() -> Part {
    let front = keepout_frame(
        "dissolved_gas_bubble_nucleation_front_robot_service_keepout",
        STATION_X - 180.0,
        FRONT_ROBOT_CLEARANCE,
        8.0,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 + FRONT_ROBOT_CLEARANCE / 2.0 + 26.0,
        deck_top_z() + 4.0,
    );
    let rear = keepout_frame(
        "dissolved_gas_bubble_nucleation_rear_cartridge_lift_keepout",
        TEMP_X + PRESSURE_X + WITNESS_X + 120.0,
        120.0,
        8.0,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - 96.0,
        deck_top_z() + REAR_CARTRIDGE_LIFT_CLEARANCE,
    );
    let pressure = keepout_frame(
        "dissolved_gas_bubble_nucleation_pressure_panel_service_keepout",
        PRESSURE_X + 44.0,
        PRESSURE_PANEL_SERVICE_CLEARANCE,
        8.0,
    )
    .translate(
        PRESSURE_CENTER.0,
        PRESSURE_CENTER.1,
        deck_top_z() + PRESSURE_Z + PRESSURE_PANEL_SERVICE_CLEARANCE / 2.0,
    );
    let camera = centered_cube(
        "dissolved_gas_bubble_nucleation_window_camera_vertical_keepout_gauge",
        WITNESS_X - 58.0,
        8.0,
        WINDOW_CAMERA_CLEARANCE_Z,
    )
    .translate(
        WITNESS_CENTER.0,
        WITNESS_CENTER.1 + WITNESS_Y / 2.0 + 22.0,
        deck_top_z() + WINDOW_CAMERA_CLEARANCE_Z / 2.0,
    );
    let gate = keepout_frame(
        "dissolved_gas_bubble_nucleation_gate_access_keepout",
        GATE_ACCESS_CLEARANCE_X,
        GATE_Y + 46.0,
        8.0,
    )
    .translate(
        GATE_CENTER.0 + GATE_X / 2.0 + GATE_ACCESS_CLEARANCE_X / 2.0 - 16.0,
        GATE_CENTER.1,
        deck_top_z() + GATE_Z + 4.0,
    );
    let token = keepout_frame(
        "dissolved_gas_bubble_nucleation_flow_token_operator_keepout",
        TOKEN_X + TRACE_X,
        82.0,
        8.0,
    )
    .translate(20.0, TOKEN_CENTER.1, deck_top_z() + TOKEN_Z + 4.0);
    front + rear + pressure + camera + gate + token
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

fn keepout_frame(name: &str, x: f64, y: f64, z: f64) -> Part {
    let front =
        centered_cube(format!("{name}_front_rail"), x, 8.0, z).translate(0.0, -y / 2.0, 0.0);
    let rear = centered_cube(format!("{name}_rear_rail"), x, 8.0, z).translate(0.0, y / 2.0, 0.0);
    let left = centered_cube(format!("{name}_left_rail"), 8.0, y, z).translate(-x / 2.0, 0.0, 0.0);
    let right = centered_cube(format!("{name}_right_rail"), 8.0, y, z).translate(x / 2.0, 0.0, 0.0);
    front + rear + left + right
}

fn split_capture_volume_ml() -> f64 {
    let cup_mm3 = std::f64::consts::PI * 24.0_f64.powi(2) * 46.0;
    cup_mm3 * 2.0 / 1000.0
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 12);
    assert!(OUTPUTS.iter().all(|path| path.starts_with(OUTPUT_PREFIX)));
    assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
    assert_eq!(
        OUTPUTS[OUTPUTS.len() - 1],
        format!("{OUTPUT_PREFIX}assembly.stl")
    );
    assert_eq!(REQUIRED_FEATURES.len(), 11);
    assert_eq!(LIMITATIONS.len(), 6);
    assert_eq!(REPRODUCIBILITY_CONTROLS.len(), 6);
    assert_eq!(UNITS, "millimeters");
    assert_eq!(TEMP_STEP_LABELS.len(), TEMP_STEPS);
    assert_eq!(TEMP_PROBE_WELLS, TEMP_STEPS * 2);
    assert_eq!(RESTRICTOR_STEPS, LANES);
    assert_eq!(RESTRICTOR_PRESSURE_TAPS, RESTRICTOR_STEPS + 1);
    assert_eq!(WITNESS_WINDOWS, LANES);
    assert_eq!(BACKLIGHT_SLOTS, WITNESS_WINDOWS);
    assert_eq!(BYPASS_COUPON_SOCKETS, DEGAS_COUPONS);
    assert_eq!(
        REFERENCE_POCKET_COUNT,
        GAS_REFERENCE_POCKETS * REFERENCE_REPLICATES
    );
    assert_eq!(SPLIT_LANES, LANES);
    assert_eq!(WASTE_PORTS, SPLIT_LANES);
    assert_eq!(RETAIN_PORTS, SPLIT_LANES);
    assert_eq!(GATE_SLOT_COUNT, LANES * GATE_MODES);
    assert_eq!(GATE_MODE_NAMES, ["release", "hold", "reject"]);
    assert_eq!(GATE_SLOTS_PER_LANE, GATE_MODES);
    assert_eq!(GATE_INTERLOCK_PINS, LANES);
    assert_eq!(SERVICE_KEEP_OUT_GAUGES, 6);
    assert_eq!(ROUTE_TRACE_SEGMENTS, 10);
    assert!(FLUID_BORE_D > TUBE_OD_MAX);
    assert!(TEMP_CARTRIDGE_D < TEMP_X / TEMP_STEPS as f64);
    assert!(TEMP_CARTRIDGE_Z < TEMP_Z);
    assert!(TEMP_STEP_DELTA_C > 30.0);
    assert!(RESTRICTOR_MIN_BORE_D < RESTRICTOR_MAX_BORE_D);
    assert!(PRESSURE_DROP_LABEL_KPA > 40.0);
    assert!(WINDOW_X < WITNESS_X / WITNESS_WINDOWS as f64);
    assert!(WINDOW_Y < WITNESS_Y);
    assert!(COUPON_SOCKET_X * (DEGAS_COUPONS as f64) < BYPASS_X);
    assert!(REFERENCE_POCKET_D * (GAS_REFERENCE_POCKETS as f64) < REF_X);
    assert!(split_capture_volume_ml() > SPLIT_CAPTURE_VOLUME_ML);
    assert!(WINDOW_CAMERA_CLEARANCE_Z > BASE_Z + WITNESS_Z);
    assert!(REAR_CARTRIDGE_LIFT_CLEARANCE > BASE_Z + TEMP_Z);

    for feature in [
        "temperature_step_media_cartridges",
        "pressure_drop_restrictor_ladder",
        "transparent_bubble_witness_windows",
        "degas_bypass_coupon",
        "gas_equilibrated_reference_pockets",
        "flow_token_rail",
        "waste_retain_split",
        "release_hold_reject_gates",
    ] {
        assert!(REQUIRED_FEATURES.contains(&feature));
    }

    let rects = module_rects();
    for rect in rects {
        assert!(
            rect.fits_inside_station(),
            "{} exceeds station envelope",
            rect.name
        );
    }
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

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        for path in OUTPUTS {
            assert!(path.starts_with(OUTPUT_PREFIX));
            assert!(path.ends_with(".stl"));
        }
        assert_eq!(
            OUTPUTS,
            [
                "output/closed_media_line_dissolved_gas_bubble_nucleation_station_containment_deck.stl",
                "output/closed_media_line_dissolved_gas_bubble_nucleation_station_temperature_step_media_cartridge_bank.stl",
                "output/closed_media_line_dissolved_gas_bubble_nucleation_station_pressure_drop_restrictor_ladder.stl",
                "output/closed_media_line_dissolved_gas_bubble_nucleation_station_transparent_bubble_witness_window_array.stl",
                "output/closed_media_line_dissolved_gas_bubble_nucleation_station_degas_bypass_coupon_dock.stl",
                "output/closed_media_line_dissolved_gas_bubble_nucleation_station_gas_equilibrated_reference_pocket_block.stl",
                "output/closed_media_line_dissolved_gas_bubble_nucleation_station_waste_retain_split_manifold.stl",
                "output/closed_media_line_dissolved_gas_bubble_nucleation_station_release_hold_reject_gate_bank.stl",
                "output/closed_media_line_dissolved_gas_bubble_nucleation_station_flow_token_rail.stl",
                "output/closed_media_line_dissolved_gas_bubble_nucleation_station_route_trace_overlay.stl",
                "output/closed_media_line_dissolved_gas_bubble_nucleation_station_robot_service_keepout_gauges.stl",
                "output/closed_media_line_dissolved_gas_bubble_nucleation_station_assembly.stl",
            ]
        );
    }

    #[test]
    fn requested_validation_features_are_represented() {
        for feature in [
            "temperature_step_media_cartridges",
            "pressure_drop_restrictor_ladder",
            "transparent_bubble_witness_windows",
            "degas_bypass_coupon",
            "gas_equilibrated_reference_pockets",
            "flow_token_rail",
            "waste_retain_split",
            "release_hold_reject_gates",
        ] {
            assert!(
                REQUIRED_FEATURES.contains(&feature),
                "missing feature {feature}"
            );
        }
    }

    #[test]
    fn dimensions_keep_the_fixture_inside_the_target_envelope() {
        assert_eq!(UNITS, "millimeters");
        assert!(STATION_X <= 1600.0);
        assert!(STATION_Y <= 1000.0);
        assert!(TEMP_X + PRESSURE_X + WITNESS_X < STATION_X);
        assert!(BYPASS_X + REF_X + SPLIT_X + GATE_X < STATION_X);
        assert!(TOKEN_X + TRACE_X < STATION_X);
        assert!(BASE_Z >= 20.0);
        assert!(RIM_Z > BASE_Z);
        assert_design_constraints();
    }

    #[test]
    fn challenge_counts_cover_temperature_pressure_gas_and_disposition() {
        assert_eq!(TEMP_STEPS, 5);
        assert_eq!(TEMP_PROBE_WELLS, 10);
        assert_eq!(RESTRICTOR_STEPS, LANES);
        assert_eq!(RESTRICTOR_PRESSURE_TAPS, 7);
        assert_eq!(WITNESS_WINDOWS, LANES);
        assert_eq!(DEGAS_COUPONS, 3);
        assert_eq!(REFERENCE_POCKET_COUNT, 8);
        assert_eq!(FLOW_TOKENS, 12);
        assert_eq!(WASTE_PORTS, 6);
        assert_eq!(RETAIN_PORTS, 6);
        assert_eq!(GATE_SLOT_COUNT, 18);
    }

    #[test]
    fn workflow_order_matches_temperature_pressure_gas_split_and_gate_logic() {
        assert!(TEMP_CENTER.0 < PRESSURE_CENTER.0);
        assert!(PRESSURE_CENTER.0 < WITNESS_CENTER.0);
        assert!(BYPASS_CENTER.0 < REF_CENTER.0);
        assert!(REF_CENTER.0 < SPLIT_CENTER.0);
        assert!(SPLIT_CENTER.0 < GATE_CENTER.0);
        assert!(TOKEN_CENTER.1 < SPLIT_CENTER.1);
        assert!(TRACE_CENTER.1 == TOKEN_CENTER.1);
    }

    #[test]
    fn margins_are_ordered_for_nucleation_risk_challenge() {
        assert!(TEMP_STEP_DELTA_C > 30.0);
        assert!(RESTRICTOR_MIN_BORE_D < RESTRICTOR_MAX_BORE_D);
        assert!(PRESSURE_DROP_LABEL_KPA > 40.0);
        assert!(FLUID_BORE_D > TUBE_OD_MAX);
        assert!(split_capture_volume_ml() > SPLIT_CAPTURE_VOLUME_ML);
        assert!(WINDOW_CAMERA_CLEARANCE_Z > BASE_Z + WITNESS_Z);
    }

    #[test]
    fn coordinate_helpers_are_symmetric() {
        assert_eq!(lane_x(0), -lane_x(LANES - 1));
        assert_eq!(lane_y(0), -lane_y(LANES - 1));
        assert_eq!(
            centered_index(0, FLOW_TOKENS, 46.0),
            -centered_index(FLOW_TOKENS - 1, FLOW_TOKENS, 46.0)
        );
    }
}
