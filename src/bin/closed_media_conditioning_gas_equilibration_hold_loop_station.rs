use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed media conditioning gas equilibration hold-loop validation station.
//
// This generator models validation-fixture packaging for pre-perfusion media
// conditioning: CO2/O2 blend interface surfaces, environmental witness blocks,
// a recirculating media hold loop, pH/osmolality sampling interface,
// bubble-trap/degas surrogate, custody/status surfaces, and closed connector
// handoff geometry for a reservoir/perfusion rack. It intentionally avoids
// process limits, biological release criteria, and clinical acceptance
// thresholds.

const OUTPUT_PREFIX: &str = "output/closed_media_conditioning_gas_equilibration_hold_loop_station_";

const OUTPUTS: [&str; 12] = [
    "output/closed_media_conditioning_gas_equilibration_hold_loop_station_containment_deck.stl",
    "output/closed_media_conditioning_gas_equilibration_hold_loop_station_co2_o2_blend_interface_panel.stl",
    "output/closed_media_conditioning_gas_equilibration_hold_loop_station_temperature_humidity_dewpoint_witness_blocks.stl",
    "output/closed_media_conditioning_gas_equilibration_hold_loop_station_recirculating_media_hold_loop_deck.stl",
    "output/closed_media_conditioning_gas_equilibration_hold_loop_station_bubble_trap_degas_surrogate_bank.stl",
    "output/closed_media_conditioning_gas_equilibration_hold_loop_station_ph_osmolality_sample_interface.stl",
    "output/closed_media_conditioning_gas_equilibration_hold_loop_station_reservoir_perfusion_handoff_bulkhead.stl",
    "output/closed_media_conditioning_gas_equilibration_hold_loop_station_barcode_custody_status_surfaces.stl",
    "output/closed_media_conditioning_gas_equilibration_hold_loop_station_closed_connector_cap_retention_bridge.stl",
    "output/closed_media_conditioning_gas_equilibration_hold_loop_station_route_and_witness_overlay.stl",
    "output/closed_media_conditioning_gas_equilibration_hold_loop_station_robot_service_keepout_gauges.stl",
    "output/closed_media_conditioning_gas_equilibration_hold_loop_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 12] = [
    "co2_o2_blend_interface",
    "gas_equilibration_hold_loop",
    "recirculating_media_loop",
    "temperature_witness_block",
    "humidity_witness_block",
    "dewpoint_witness_block",
    "ph_sample_interface",
    "osmolality_sample_interface",
    "bubble_trap_degas_surrogate",
    "barcode_custody_surfaces",
    "status_surfaces",
    "reservoir_perfusion_closed_handoff",
];

const LIMITATIONS: [&str; 6] = [
    "validation_fixture_only",
    "not_a_pressure_rated_gas_mixer",
    "not_a_sterile_wetted_product_design",
    "not_a_biological_release_protocol",
    "no_clinical_acceptance_thresholds",
    "purchased_connectors_sensors_and_membranes_are_surrogates",
];

const REPRODUCIBILITY_CONTROLS: [&str; 6] = [
    "fixed_output_manifest",
    "millimeter_units",
    "no_random_inputs",
    "named_deterministic_geometry",
    "static_feature_counts",
    "stable_layout_rectangles",
];

const PARAMETRIC_REVISION: &str =
    "closed_media_conditioning_gas_equilibration_hold_loop_station_v1";
const UNITS: &str = "millimeters";
const GRID_STEP_MM: f64 = 2.0;
const DEFAULT_TESSELLATION_SEGMENTS: u32 = 32;

const STATION_X: f64 = 1540.0;
const STATION_Y: f64 = 940.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 44.0;
const SOCKET_DEPTH: f64 = 6.0;
const SUMP_RECESS_Z: f64 = 7.0;
const MOUNT_HOLE_D: f64 = 6.8;
const DATUM_RING_D: f64 = 18.0;
const LAYOUT_CLEARANCE: f64 = 14.0;

const GAS_CENTER: (f64, f64) = (-520.0, 250.0);
const GAS_X: f64 = 360.0;
const GAS_Y: f64 = 230.0;
const GAS_Z: f64 = 74.0;
const GAS_CHANNELS: usize = 2;
const GAS_NAMES: [&str; GAS_CHANNELS] = ["co2", "o2"];
const GAS_PORTS_PER_CHANNEL: usize = 4;
const GAS_INTERFACE_PORTS: usize = GAS_CHANNELS * GAS_PORTS_PER_CHANNEL;
const BLEND_SELECTOR_COUNT: usize = 4;
const BLEND_WITNESS_WINDOWS: usize = 6;
const GAS_PORT_D: f64 = 10.0;

const ENV_CENTER: (f64, f64) = (0.0, 250.0);
const ENV_X: f64 = 420.0;
const ENV_Y: f64 = 230.0;
const ENV_Z: f64 = 62.0;
const ENV_WITNESS_BLOCKS: usize = 3;
const TEMP_WITNESS_WELLS: usize = 6;
const HUMIDITY_WITNESS_WELLS: usize = 4;
const DEWPOINT_WITNESS_WELLS: usize = 4;
const ENV_REFERENCE_COUPONS: usize = 6;
const ENV_WELL_D: f64 = 16.0;

const LOOP_CENTER: (f64, f64) = (455.0, 250.0);
const LOOP_X: f64 = 420.0;
const LOOP_Y: f64 = 230.0;
const LOOP_Z: f64 = 64.0;
const LOOP_LANES: usize = 6;
const MEDIA_LOOP_PORTS: usize = LOOP_LANES * 2;
const HOLD_LOOP_COIL_SEGMENTS: usize = 8;
const RECIRC_PUMP_SURROGATES: usize = 2;
const LOOP_LANE_PITCH_X: f64 = 52.0;
const MEDIA_BORE_D: f64 = 7.2;

const TRAP_CENTER: (f64, f64) = (-520.0, -85.0);
const TRAP_X: f64 = 360.0;
const TRAP_Y: f64 = 260.0;
const TRAP_Z: f64 = 92.0;
const BUBBLE_TRAP_COLUMNS: usize = LOOP_LANES;
const DEGAS_MEMBRANE_WINDOWS: usize = LOOP_LANES;
const VENT_CAPTURE_CUPS: usize = LOOP_LANES;
const TRAP_COLUMN_D: f64 = 30.0;
const MEMBRANE_WINDOW_X: f64 = 34.0;
const MEMBRANE_WINDOW_Y: f64 = 76.0;

const SAMPLE_CENTER: (f64, f64) = (0.0, -85.0);
const SAMPLE_X: f64 = 420.0;
const SAMPLE_Y: f64 = 260.0;
const SAMPLE_Z: f64 = 58.0;
const SAMPLE_ANALYTES: usize = 2;
const SAMPLE_REPLICATES: usize = 6;
const SAMPLE_WELLS: usize = SAMPLE_ANALYTES * SAMPLE_REPLICATES;
const SAMPLE_VALVES: usize = SAMPLE_ANALYTES * 3;
const SAMPLE_WELL_D: f64 = 17.0;
const SAMPLE_PITCH_X: f64 = 54.0;
const SAMPLE_PITCH_Y: f64 = 56.0;

const HANDOFF_CENTER: (f64, f64) = (455.0, -85.0);
const HANDOFF_X: f64 = 420.0;
const HANDOFF_Y: f64 = 260.0;
const HANDOFF_Z: f64 = 72.0;
const CLOSED_HANDOFF_CONNECTORS: usize = 8;
const RESERVOIR_DOCKS: usize = 4;
const PERFUSION_RACK_DOCKS: usize = 4;
const CONNECTOR_KEY_COUNT: usize = CLOSED_HANDOFF_CONNECTORS;
const CONNECTOR_D: f64 = 18.0;

const TRACE_CENTER: (f64, f64) = (0.0, -375.0);
const TRACE_X: f64 = 980.0;
const TRACE_Y: f64 = 90.0;
const TRACE_Z: f64 = 18.0;
const BARCODE_LANDS: usize = 12;
const CUSTODY_TOKEN_SLOTS: usize = 8;
const STATUS_LANES: usize = 4;
const STATUS_TOKENS_PER_LANE: usize = 3;
const STATUS_TOKENS: usize = STATUS_LANES * STATUS_TOKENS_PER_LANE;

const CAP_BRIDGE_SEGMENTS: usize = 8;
const GASKET_WITNESS_PUCKS: usize = 8;
const ROUTE_SEGMENTS: usize = 10;
const FLOW_DIRECTION_MARKERS: usize = 12;

const SERVICE_KEEP_OUT_GAUGES: usize = 5;
const FRONT_ROBOT_CLEARANCE: f64 = 420.0;
const REAR_GAS_SERVICE_CLEARANCE: f64 = 280.0;
const LEFT_TRAP_SERVICE_CLEARANCE: f64 = 210.0;
const RIGHT_HANDOFF_SERVICE_CLEARANCE: f64 = 230.0;
const TOP_CONNECTOR_LIFT_CLEARANCE: f64 = 250.0;

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

    let gas = co2_o2_blend_interface_panel();
    export(OUTPUTS[1], &gas);

    let environment = temperature_humidity_dewpoint_witness_blocks();
    export(OUTPUTS[2], &environment);

    let loop_deck = recirculating_media_hold_loop_deck();
    export(OUTPUTS[3], &loop_deck);

    let traps = bubble_trap_degas_surrogate_bank();
    export(OUTPUTS[4], &traps);

    let samples = ph_osmolality_sample_interface();
    export(OUTPUTS[5], &samples);

    let handoff = reservoir_perfusion_handoff_bulkhead();
    export(OUTPUTS[6], &handoff);

    let custody = barcode_custody_status_surfaces();
    export(OUTPUTS[7], &custody);

    let caps = closed_connector_cap_retention_bridge();
    export(OUTPUTS[8], &caps);

    let overlay = route_and_witness_overlay();
    export(OUTPUTS[9], &overlay);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[10], &keepouts);

    let assembly = deck
        + gas
        + environment
        + loop_deck
        + traps
        + samples
        + handoff
        + custody
        + caps
        + overlay
        + keepouts;
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed media conditioning gas equilibration hold-loop station:");
    println!("  Revision/units:              {PARAMETRIC_REVISION} / {UNITS}");
    println!("  Footprint:                   {STATION_X:.0}mm x {STATION_Y:.0}mm containment deck");
    println!(
        "  Gas interface:               {:?} channels, {GAS_INTERFACE_PORTS} ports, {BLEND_SELECTOR_COUNT} selector blocks, {BLEND_WITNESS_WINDOWS} witness windows",
        GAS_NAMES
    );
    println!(
        "  Environment witnesses:       {ENV_WITNESS_BLOCKS} blocks with temperature, humidity, and dewpoint witness wells"
    );
    println!(
        "  Hold loop:                   {LOOP_LANES} recirculating lanes, {MEDIA_LOOP_PORTS} media ports, {HOLD_LOOP_COIL_SEGMENTS} coil route segments"
    );
    println!(
        "  Sample/degas coverage:       {SAMPLE_WELLS} pH/osmolality sample wells, {BUBBLE_TRAP_COLUMNS} bubble trap columns, {DEGAS_MEMBRANE_WINDOWS} degas membrane windows"
    );
    println!(
        "  Handoff/evidence:            {CLOSED_HANDOFF_CONNECTORS} closed connectors, {BARCODE_LANDS} barcode lands, {CUSTODY_TOKEN_SLOTS} custody slots, {STATUS_TOKENS} status tokens"
    );
    println!(
        "  Reproducibility controls:    {} explicit controls; no clinical acceptance thresholds",
        REPRODUCIBILITY_CONTROLS.len()
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
    centered_index(index, LOOP_LANES, LOOP_LANE_PITCH_X)
}

fn sample_x(index: usize) -> f64 {
    centered_index(index, SAMPLE_REPLICATES, SAMPLE_PITCH_X)
}

fn status_lane_x(index: usize) -> f64 {
    centered_index(index, STATUS_LANES, 164.0)
}

fn module_rects() -> [Rect; 7] {
    [
        rect("co2_o2_blend_interface_panel", GAS_CENTER, GAS_X, GAS_Y),
        rect(
            "temperature_humidity_dewpoint_witness_blocks",
            ENV_CENTER,
            ENV_X,
            ENV_Y,
        ),
        rect(
            "recirculating_media_hold_loop_deck",
            LOOP_CENTER,
            LOOP_X,
            LOOP_Y,
        ),
        rect(
            "bubble_trap_degas_surrogate_bank",
            TRAP_CENTER,
            TRAP_X,
            TRAP_Y,
        ),
        rect(
            "ph_osmolality_sample_interface",
            SAMPLE_CENTER,
            SAMPLE_X,
            SAMPLE_Y,
        ),
        rect(
            "reservoir_perfusion_handoff_bulkhead",
            HANDOFF_CENTER,
            HANDOFF_X,
            HANDOFF_Y,
        ),
        rect(
            "barcode_custody_status_surfaces",
            TRACE_CENTER,
            TRACE_X,
            TRACE_Y,
        ),
    ]
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        "media_gas_hold_loop_containment_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let sump = centered_cube(
        "media_gas_hold_loop_deck_low_sump_recess",
        STATION_X - 148.0,
        STATION_Y - 136.0,
        SUMP_RECESS_Z,
    )
    .translate(0.0, -8.0, deck_top_z() - SUMP_RECESS_Z / 2.0 + 0.2);
    let front_drain = centered_cylinder(
        "media_gas_hold_loop_front_containment_drain",
        6.0,
        68.0,
        DEFAULT_TESSELLATION_SEGMENTS,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 86.0,
        -STATION_Y / 2.0 + 8.0,
        deck_top_z() - 5.0,
    );

    deck - sump - front_drain - module_insert_sockets() - mounting_holes()
        + perimeter_rims()
        + datum_targets()
        + zone_locator_rails()
}

fn module_insert_sockets() -> Part {
    let mut sockets = Part::empty("media_gas_hold_loop_module_insert_sockets");
    for module in module_rects() {
        sockets = sockets
            + centered_cube(
                format!("media_gas_hold_loop_{}_socket", module.name),
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
    let mut holes = Part::empty("media_gas_hold_loop_mounting_holes");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 60.0, -STATION_Y / 2.0 + 54.0),
        (STATION_X / 2.0 - 60.0, -STATION_Y / 2.0 + 54.0),
        (-STATION_X / 2.0 + 60.0, STATION_Y / 2.0 - 54.0),
        (STATION_X / 2.0 - 60.0, STATION_Y / 2.0 - 54.0),
        (-265.0, 68.0),
        (265.0, 68.0),
        (-265.0, -260.0),
        (265.0, -260.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("media_gas_hold_loop_mount_clearance_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 4.0,
                24,
            )
            .translate(*x, *y, BASE_Z / 2.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    let left = centered_cube(
        "media_gas_hold_loop_left_spill_retention_rim",
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
        "media_gas_hold_loop_right_spill_retention_rim",
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
        "media_gas_hold_loop_rear_gas_service_rim",
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
        "media_gas_hold_loop_low_front_robot_service_lip",
        STATION_X - 220.0,
        12.0,
        20.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 16.0, deck_top_z() + 10.0);
    left + right + rear + front
}

fn datum_targets() -> Part {
    let mut targets = Part::empty("media_gas_hold_loop_robot_datum_targets");
    for (i, (x, y)) in [
        (-650.0, -386.0),
        (650.0, -386.0),
        (-650.0, 372.0),
        (650.0, 372.0),
        (-240.0, 372.0),
        (240.0, 372.0),
    ]
    .iter()
    .enumerate()
    {
        let ring = centered_cylinder(
            format!("media_gas_hold_loop_datum_target_outer_{i}"),
            DATUM_RING_D / 2.0,
            4.0,
            DEFAULT_TESSELLATION_SEGMENTS,
        )
        .translate(*x, *y, deck_top_z() + 2.0)
            - centered_cylinder(
                format!("media_gas_hold_loop_datum_target_inner_{i}"),
                4.0,
                6.0,
                24,
            )
            .translate(*x, *y, deck_top_z() + 2.0);
        targets = targets + ring;
    }
    targets
}

fn zone_locator_rails() -> Part {
    let rear = centered_cube(
        "media_gas_hold_loop_rear_gas_environment_loop_locator_rail",
        STATION_X - 260.0,
        10.0,
        18.0,
    )
    .translate(0.0, 82.0, deck_top_z() + 9.0);
    let front = centered_cube(
        "media_gas_hold_loop_front_trap_sample_handoff_locator_rail",
        STATION_X - 300.0,
        10.0,
        18.0,
    )
    .translate(0.0, -235.0, deck_top_z() + 9.0);
    let trace = centered_cube(
        "media_gas_hold_loop_traceability_zone_locator_rail",
        TRACE_X + 50.0,
        8.0,
        16.0,
    )
    .translate(
        TRACE_CENTER.0,
        TRACE_CENTER.1 + TRACE_Y / 2.0 + 18.0,
        deck_top_z() + 8.0,
    );
    rear + front + trace
}

fn co2_o2_blend_interface_panel() -> Part {
    let body = centered_cube(
        "media_gas_hold_loop_co2_o2_blend_panel_body",
        GAS_X,
        GAS_Y,
        GAS_Z,
    )
    .translate(GAS_CENTER.0, GAS_CENTER.1, place_z(GAS_Z));
    body - gas_port_bores()
        + gas_port_gasket_lands()
        + blend_selector_blocks()
        + gas_witness_window_frames()
        + gas_route_plenum_rails()
}

fn gas_port_bores() -> Part {
    let mut bores = Part::empty("media_gas_hold_loop_gas_interface_port_bores");
    for gas in 0..GAS_CHANNELS {
        for port in 0..GAS_PORTS_PER_CHANNEL {
            let x = GAS_CENTER.0 + centered_index(gas, GAS_CHANNELS, 138.0);
            let y = GAS_CENTER.1 + centered_index(port, GAS_PORTS_PER_CHANNEL, 42.0);
            bores = bores
                + centered_cylinder(
                    format!(
                        "media_gas_hold_loop_{}_interface_port_{port}_bore",
                        GAS_NAMES[gas]
                    ),
                    GAS_PORT_D / 2.0,
                    GAS_Z + 12.0,
                    24,
                )
                .translate(x, y, place_z(GAS_Z));
        }
    }
    bores
}

fn gas_port_gasket_lands() -> Part {
    let mut lands = Part::empty("media_gas_hold_loop_gas_port_gasket_lands");
    for gas in 0..GAS_CHANNELS {
        for port in 0..GAS_PORTS_PER_CHANNEL {
            let x = GAS_CENTER.0 + centered_index(gas, GAS_CHANNELS, 138.0);
            let y = GAS_CENTER.1 + centered_index(port, GAS_PORTS_PER_CHANNEL, 42.0);
            let outer = centered_cylinder(
                format!(
                    "media_gas_hold_loop_{}_interface_port_{port}_gasket_outer",
                    GAS_NAMES[gas]
                ),
                16.0,
                5.0,
                DEFAULT_TESSELLATION_SEGMENTS,
            )
            .translate(x, y, deck_top_z() + GAS_Z + 2.5);
            let inner = centered_cylinder(
                format!(
                    "media_gas_hold_loop_{}_interface_port_{port}_gasket_clearance",
                    GAS_NAMES[gas]
                ),
                GAS_PORT_D / 2.0 + 1.0,
                7.0,
                24,
            )
            .translate(x, y, deck_top_z() + GAS_Z + 2.5);
            lands = lands + (outer - inner);
        }
    }
    lands
}

fn blend_selector_blocks() -> Part {
    let mut selectors = Part::empty("media_gas_hold_loop_blend_selector_blocks");
    for i in 0..BLEND_SELECTOR_COUNT {
        let x = GAS_CENTER.0 + centered_index(i, BLEND_SELECTOR_COUNT, 62.0);
        let y = GAS_CENTER.1 - GAS_Y / 2.0 + 42.0;
        selectors = selectors
            + centered_cube(
                format!("media_gas_hold_loop_blend_selector_block_{i}"),
                44.0,
                34.0,
                28.0,
            )
            .translate(x, y, deck_top_z() + GAS_Z + 14.0)
            + centered_cylinder(
                format!("media_gas_hold_loop_blend_selector_handle_{i}"),
                9.0,
                22.0,
                24,
            )
            .translate(x, y, deck_top_z() + GAS_Z + 39.0);
    }
    selectors
}

fn gas_witness_window_frames() -> Part {
    let mut windows = Part::empty("media_gas_hold_loop_gas_witness_window_frames");
    for i in 0..BLEND_WITNESS_WINDOWS {
        let x = GAS_CENTER.0 + centered_index(i, BLEND_WITNESS_WINDOWS, 48.0);
        let y = GAS_CENTER.1 + GAS_Y / 2.0 - 36.0;
        windows = windows
            + rectangular_frame(
                &format!("media_gas_hold_loop_blend_witness_window_frame_{i}"),
                34.0,
                18.0,
                4.0,
                5.0,
            )
            .translate(x, y, deck_top_z() + GAS_Z + 4.0);
    }
    windows
}

fn gas_route_plenum_rails() -> Part {
    let upper = centered_cube(
        "media_gas_hold_loop_co2_o2_upper_route_plenum_rail",
        GAS_X - 58.0,
        10.0,
        12.0,
    )
    .translate(
        GAS_CENTER.0,
        GAS_CENTER.1 + 18.0,
        deck_top_z() + GAS_Z + 6.0,
    );
    let lower = centered_cube(
        "media_gas_hold_loop_co2_o2_lower_route_plenum_rail",
        GAS_X - 58.0,
        10.0,
        12.0,
    )
    .translate(
        GAS_CENTER.0,
        GAS_CENTER.1 - 18.0,
        deck_top_z() + GAS_Z + 6.0,
    );
    upper + lower
}

fn temperature_humidity_dewpoint_witness_blocks() -> Part {
    let body = centered_cube(
        "media_gas_hold_loop_environment_witness_body",
        ENV_X,
        ENV_Y,
        ENV_Z,
    )
    .translate(ENV_CENTER.0, ENV_CENTER.1, place_z(ENV_Z));
    body - environment_well_cuts()
        + environment_well_rims()
        + probe_bridge_sockets()
        + environmental_reference_coupon_lands()
        + witness_block_title_tabs()
}

fn environment_well_cuts() -> Part {
    let mut cuts = Part::empty("media_gas_hold_loop_environment_witness_well_cuts");
    for block in 0..ENV_WITNESS_BLOCKS {
        let x = ENV_CENTER.0 + centered_index(block, ENV_WITNESS_BLOCKS, 130.0);
        let well_count = match block {
            0 => TEMP_WITNESS_WELLS,
            1 => HUMIDITY_WITNESS_WELLS,
            _ => DEWPOINT_WITNESS_WELLS,
        };
        for well in 0..well_count {
            let col = well % 2;
            let row = well / 2;
            let y = ENV_CENTER.1 - 38.0 + centered_index(row, 3, 34.0);
            cuts = cuts
                + centered_cylinder(
                    format!("media_gas_hold_loop_environment_block_{block}_well_{well}_cut"),
                    ENV_WELL_D / 2.0,
                    ENV_Z + 10.0,
                    24,
                )
                .translate(x + centered_index(col, 2, 36.0), y, place_z(ENV_Z));
        }
    }
    cuts
}

fn environment_well_rims() -> Part {
    let mut rims = Part::empty("media_gas_hold_loop_environment_witness_well_rims");
    for block in 0..ENV_WITNESS_BLOCKS {
        let x = ENV_CENTER.0 + centered_index(block, ENV_WITNESS_BLOCKS, 130.0);
        let well_count = match block {
            0 => TEMP_WITNESS_WELLS,
            1 => HUMIDITY_WITNESS_WELLS,
            _ => DEWPOINT_WITNESS_WELLS,
        };
        for well in 0..well_count {
            let col = well % 2;
            let row = well / 2;
            let y = ENV_CENTER.1 - 38.0 + centered_index(row, 3, 34.0);
            let rim = centered_cylinder(
                format!("media_gas_hold_loop_environment_block_{block}_well_{well}_rim_outer"),
                ENV_WELL_D / 2.0 + 5.0,
                5.0,
                28,
            )
            .translate(
                x + centered_index(col, 2, 36.0),
                y,
                deck_top_z() + ENV_Z + 2.5,
            ) - centered_cylinder(
                format!("media_gas_hold_loop_environment_block_{block}_well_{well}_rim_inner"),
                ENV_WELL_D / 2.0,
                7.0,
                24,
            )
            .translate(
                x + centered_index(col, 2, 36.0),
                y,
                deck_top_z() + ENV_Z + 2.5,
            );
            rims = rims + rim;
        }
    }
    rims
}

fn probe_bridge_sockets() -> Part {
    let mut sockets = Part::empty("media_gas_hold_loop_environment_probe_bridge_sockets");
    for block in 0..ENV_WITNESS_BLOCKS {
        let x = ENV_CENTER.0 + centered_index(block, ENV_WITNESS_BLOCKS, 130.0);
        sockets = sockets
            + centered_cube(
                format!("media_gas_hold_loop_environment_block_{block}_probe_bridge_socket"),
                92.0,
                18.0,
                16.0,
            )
            .translate(
                x,
                ENV_CENTER.1 + ENV_Y / 2.0 - 38.0,
                deck_top_z() + ENV_Z + 8.0,
            )
            + centered_cube(
                format!("media_gas_hold_loop_environment_block_{block}_lead_strain_relief"),
                20.0,
                58.0,
                12.0,
            )
            .translate(
                x,
                ENV_CENTER.1 + ENV_Y / 2.0 - 72.0,
                deck_top_z() + ENV_Z + 6.0,
            );
    }
    sockets
}

fn environmental_reference_coupon_lands() -> Part {
    let mut coupons = Part::empty("media_gas_hold_loop_environment_reference_coupon_lands");
    for i in 0..ENV_REFERENCE_COUPONS {
        coupons = coupons
            + centered_cube(
                format!("media_gas_hold_loop_environment_reference_coupon_land_{i}"),
                42.0,
                16.0,
                6.0,
            )
            .translate(
                ENV_CENTER.0 + centered_index(i, ENV_REFERENCE_COUPONS, 54.0),
                ENV_CENTER.1 - ENV_Y / 2.0 + 24.0,
                deck_top_z() + ENV_Z + 3.0,
            );
    }
    coupons
}

fn witness_block_title_tabs() -> Part {
    let mut tabs = Part::empty("media_gas_hold_loop_environment_witness_title_tabs");
    for (i, name) in ["temperature", "humidity", "dewpoint"].iter().enumerate() {
        tabs = tabs
            + centered_cube(
                format!("media_gas_hold_loop_{name}_witness_block_tab"),
                92.0,
                10.0,
                7.0,
            )
            .translate(
                ENV_CENTER.0 + centered_index(i, ENV_WITNESS_BLOCKS, 130.0),
                ENV_CENTER.1,
                deck_top_z() + ENV_Z + 10.0,
            );
    }
    tabs
}

fn recirculating_media_hold_loop_deck() -> Part {
    let body = centered_cube(
        "media_gas_hold_loop_recirculating_media_loop_body",
        LOOP_X,
        LOOP_Y,
        LOOP_Z,
    )
    .translate(LOOP_CENTER.0, LOOP_CENTER.1, place_z(LOOP_Z));
    body - media_loop_port_bores()
        + media_loop_port_rims()
        + hold_loop_coil_route()
        + pump_surrogate_cradles()
        + loop_direction_markers()
}

fn media_loop_port_bores() -> Part {
    let mut bores = Part::empty("media_gas_hold_loop_media_loop_port_bores");
    for lane in 0..LOOP_LANES {
        for (side, y_offset) in [-72.0, 72.0].iter().enumerate() {
            bores = bores
                + centered_cylinder(
                    format!("media_gas_hold_loop_lane_{lane}_port_{side}_bore"),
                    MEDIA_BORE_D / 2.0,
                    LOOP_Y + 16.0,
                    24,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(
                    LOOP_CENTER.0 + lane_x(lane),
                    LOOP_CENTER.1 + y_offset,
                    place_z(LOOP_Z),
                );
        }
    }
    bores
}

fn media_loop_port_rims() -> Part {
    let mut rims = Part::empty("media_gas_hold_loop_media_loop_port_rims");
    for lane in 0..LOOP_LANES {
        for (side, y_offset) in [-72.0, 72.0].iter().enumerate() {
            rims = rims
                + centered_cylinder(
                    format!("media_gas_hold_loop_lane_{lane}_port_{side}_outer_rim"),
                    13.0,
                    5.0,
                    DEFAULT_TESSELLATION_SEGMENTS,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(
                    LOOP_CENTER.0 + lane_x(lane),
                    LOOP_CENTER.1 + y_offset,
                    deck_top_z() + LOOP_Z / 2.0,
                );
        }
    }
    rims
}

fn hold_loop_coil_route() -> Part {
    let mut route = Part::empty("media_gas_hold_loop_visible_hold_loop_coil_route");
    for segment in 0..HOLD_LOOP_COIL_SEGMENTS {
        let y = LOOP_CENTER.1 + centered_index(segment, HOLD_LOOP_COIL_SEGMENTS, 18.0);
        let length = if segment % 2 == 0 {
            LOOP_X - 88.0
        } else {
            LOOP_X - 142.0
        };
        route = route
            + centered_cube(
                format!("media_gas_hold_loop_hold_coil_route_segment_{segment}"),
                length,
                8.0,
                8.0,
            )
            .translate(LOOP_CENTER.0, y, deck_top_z() + LOOP_Z + 4.0);
    }
    for turn in 0..(HOLD_LOOP_COIL_SEGMENTS - 1) {
        route = route
            + centered_cylinder(
                format!("media_gas_hold_loop_hold_coil_u_turn_marker_{turn}"),
                12.0,
                8.0,
                DEFAULT_TESSELLATION_SEGMENTS,
            )
            .translate(
                LOOP_CENTER.0
                    + if turn % 2 == 0 {
                        LOOP_X / 2.0 - 54.0
                    } else {
                        -LOOP_X / 2.0 + 54.0
                    },
                LOOP_CENTER.1 + centered_index(turn, HOLD_LOOP_COIL_SEGMENTS - 1, 18.0),
                deck_top_z() + LOOP_Z + 4.0,
            );
    }
    route
}

fn pump_surrogate_cradles() -> Part {
    let mut cradles = Part::empty("media_gas_hold_loop_recirc_pump_surrogate_cradles");
    for pump in 0..RECIRC_PUMP_SURROGATES {
        cradles = cradles
            + centered_cube(
                format!("media_gas_hold_loop_recirc_pump_surrogate_cradle_{pump}"),
                118.0,
                38.0,
                24.0,
            )
            .translate(
                LOOP_CENTER.0 + centered_index(pump, RECIRC_PUMP_SURROGATES, 230.0),
                LOOP_CENTER.1 - LOOP_Y / 2.0 + 34.0,
                deck_top_z() + LOOP_Z + 12.0,
            )
            + centered_cylinder(
                format!("media_gas_hold_loop_recirc_pump_head_shadow_{pump}"),
                22.0,
                16.0,
                DEFAULT_TESSELLATION_SEGMENTS,
            )
            .translate(
                LOOP_CENTER.0 + centered_index(pump, RECIRC_PUMP_SURROGATES, 230.0),
                LOOP_CENTER.1 - LOOP_Y / 2.0 + 34.0,
                deck_top_z() + LOOP_Z + 34.0,
            );
    }
    cradles
}

fn loop_direction_markers() -> Part {
    let mut markers = Part::empty("media_gas_hold_loop_recirculation_direction_markers");
    for marker in 0..LOOP_LANES {
        markers = markers
            + centered_cube(
                format!("media_gas_hold_loop_lane_{marker}_recirc_direction_bar"),
                30.0,
                6.0,
                5.0,
            )
            .translate(
                LOOP_CENTER.0 + lane_x(marker),
                LOOP_CENTER.1,
                deck_top_z() + LOOP_Z + 12.0,
            )
            + centered_cube(
                format!("media_gas_hold_loop_lane_{marker}_recirc_direction_head"),
                10.0,
                14.0,
                5.0,
            )
            .rotate(0.0, 0.0, 45.0)
            .translate(
                LOOP_CENTER.0 + lane_x(marker) + 18.0,
                LOOP_CENTER.1,
                deck_top_z() + LOOP_Z + 12.0,
            );
    }
    markers
}

fn bubble_trap_degas_surrogate_bank() -> Part {
    let body = centered_cube(
        "media_gas_hold_loop_bubble_trap_degas_surrogate_body",
        TRAP_X,
        TRAP_Y,
        TRAP_Z,
    )
    .translate(TRAP_CENTER.0, TRAP_CENTER.1, place_z(TRAP_Z));
    body - bubble_trap_column_cuts() - degas_membrane_window_cuts()
        + bubble_trap_column_rims()
        + degas_membrane_window_frames()
        + vent_capture_cups()
        + trap_route_manifold_bars()
}

fn bubble_trap_column_cuts() -> Part {
    let mut cuts = Part::empty("media_gas_hold_loop_bubble_trap_column_cuts");
    for column in 0..BUBBLE_TRAP_COLUMNS {
        cuts = cuts
            + centered_cylinder(
                format!("media_gas_hold_loop_bubble_trap_column_{column}_cut"),
                TRAP_COLUMN_D / 2.0,
                TRAP_Z + 18.0,
                DEFAULT_TESSELLATION_SEGMENTS,
            )
            .translate(
                TRAP_CENTER.0 + lane_x(column),
                TRAP_CENTER.1 + 36.0,
                place_z(TRAP_Z),
            );
    }
    cuts
}

fn degas_membrane_window_cuts() -> Part {
    let mut cuts = Part::empty("media_gas_hold_loop_degas_membrane_window_cuts");
    for window in 0..DEGAS_MEMBRANE_WINDOWS {
        cuts = cuts
            + centered_cube(
                format!("media_gas_hold_loop_degas_membrane_window_{window}_cut"),
                MEMBRANE_WINDOW_X,
                MEMBRANE_WINDOW_Y,
                TRAP_Z + 12.0,
            )
            .translate(
                TRAP_CENTER.0 + lane_x(window),
                TRAP_CENTER.1 - 58.0,
                place_z(TRAP_Z),
            );
    }
    cuts
}

fn bubble_trap_column_rims() -> Part {
    let mut rims = Part::empty("media_gas_hold_loop_bubble_trap_column_rims");
    for column in 0..BUBBLE_TRAP_COLUMNS {
        let x = TRAP_CENTER.0 + lane_x(column);
        let rim = centered_cylinder(
            format!("media_gas_hold_loop_bubble_trap_column_{column}_rim_outer"),
            TRAP_COLUMN_D / 2.0 + 7.0,
            6.0,
            DEFAULT_TESSELLATION_SEGMENTS,
        )
        .translate(x, TRAP_CENTER.1 + 36.0, deck_top_z() + TRAP_Z + 3.0)
            - centered_cylinder(
                format!("media_gas_hold_loop_bubble_trap_column_{column}_rim_inner"),
                TRAP_COLUMN_D / 2.0,
                8.0,
                DEFAULT_TESSELLATION_SEGMENTS,
            )
            .translate(x, TRAP_CENTER.1 + 36.0, deck_top_z() + TRAP_Z + 3.0);
        rims = rims + rim;
    }
    rims
}

fn degas_membrane_window_frames() -> Part {
    let mut frames = Part::empty("media_gas_hold_loop_degas_membrane_window_frames");
    for window in 0..DEGAS_MEMBRANE_WINDOWS {
        frames = frames
            + rectangular_frame(
                &format!("media_gas_hold_loop_degas_membrane_window_{window}_frame"),
                MEMBRANE_WINDOW_X + 12.0,
                MEMBRANE_WINDOW_Y + 12.0,
                4.0,
                7.0,
            )
            .translate(
                TRAP_CENTER.0 + lane_x(window),
                TRAP_CENTER.1 - 58.0,
                deck_top_z() + TRAP_Z + 4.0,
            );
    }
    frames
}

fn vent_capture_cups() -> Part {
    let mut cups = Part::empty("media_gas_hold_loop_trap_vent_capture_cups");
    for cup in 0..VENT_CAPTURE_CUPS {
        let x = TRAP_CENTER.0 + lane_x(cup);
        let capture = centered_cylinder(
            format!("media_gas_hold_loop_high_point_vent_capture_cup_{cup}_outer"),
            13.0,
            12.0,
            28,
        )
        .translate(
            x,
            TRAP_CENTER.1 + TRAP_Y / 2.0 - 28.0,
            deck_top_z() + TRAP_Z + 14.0,
        ) - centered_cylinder(
            format!("media_gas_hold_loop_high_point_vent_capture_cup_{cup}_inner"),
            8.0,
            14.0,
            24,
        )
        .translate(
            x,
            TRAP_CENTER.1 + TRAP_Y / 2.0 - 28.0,
            deck_top_z() + TRAP_Z + 14.0,
        );
        cups = cups + capture;
    }
    cups
}

fn trap_route_manifold_bars() -> Part {
    let inlet = centered_cube(
        "media_gas_hold_loop_bubble_trap_inlet_manifold_bar",
        TRAP_X - 58.0,
        10.0,
        12.0,
    )
    .translate(
        TRAP_CENTER.0,
        TRAP_CENTER.1 - 118.0,
        deck_top_z() + TRAP_Z + 6.0,
    );
    let outlet = centered_cube(
        "media_gas_hold_loop_bubble_trap_outlet_manifold_bar",
        TRAP_X - 58.0,
        10.0,
        12.0,
    )
    .translate(
        TRAP_CENTER.0,
        TRAP_CENTER.1 + 112.0,
        deck_top_z() + TRAP_Z + 6.0,
    );
    inlet + outlet
}

fn ph_osmolality_sample_interface() -> Part {
    let body = centered_cube(
        "media_gas_hold_loop_ph_osmolality_sample_interface_body",
        SAMPLE_X,
        SAMPLE_Y,
        SAMPLE_Z,
    )
    .translate(SAMPLE_CENTER.0, SAMPLE_CENTER.1, place_z(SAMPLE_Z));
    body - sample_well_cuts() - sample_valve_bores()
        + sample_well_rims()
        + sample_valve_blocks()
        + analyzer_handoff_pockets()
        + sample_custody_lane_tabs()
}

fn sample_well_cuts() -> Part {
    let mut cuts = Part::empty("media_gas_hold_loop_sample_well_cuts");
    for analyte in 0..SAMPLE_ANALYTES {
        for replicate in 0..SAMPLE_REPLICATES {
            cuts = cuts
                + centered_cylinder(
                    format!(
                    "media_gas_hold_loop_sample_analyte_{analyte}_replicate_{replicate}_well_cut"
                ),
                    SAMPLE_WELL_D / 2.0,
                    SAMPLE_Z + 10.0,
                    24,
                )
                .translate(
                    SAMPLE_CENTER.0 + sample_x(replicate),
                    SAMPLE_CENTER.1 + centered_index(analyte, SAMPLE_ANALYTES, SAMPLE_PITCH_Y),
                    place_z(SAMPLE_Z),
                );
        }
    }
    cuts
}

fn sample_well_rims() -> Part {
    let mut rims = Part::empty("media_gas_hold_loop_sample_well_rims");
    for analyte in 0..SAMPLE_ANALYTES {
        for replicate in 0..SAMPLE_REPLICATES {
            let x = SAMPLE_CENTER.0 + sample_x(replicate);
            let y = SAMPLE_CENTER.1 + centered_index(analyte, SAMPLE_ANALYTES, SAMPLE_PITCH_Y);
            let rim = centered_cylinder(
                format!("media_gas_hold_loop_sample_analyte_{analyte}_replicate_{replicate}_septum_rim"),
                SAMPLE_WELL_D / 2.0 + 5.0,
                5.0,
                28,
            )
            .translate(x, y, deck_top_z() + SAMPLE_Z + 2.5)
                - centered_cylinder(
                    format!("media_gas_hold_loop_sample_analyte_{analyte}_replicate_{replicate}_septum_clearance"),
                    SAMPLE_WELL_D / 2.0,
                    7.0,
                    24,
                )
                .translate(x, y, deck_top_z() + SAMPLE_Z + 2.5);
            rims = rims + rim;
        }
    }
    rims
}

fn sample_valve_bores() -> Part {
    let mut bores = Part::empty("media_gas_hold_loop_sample_valve_bores");
    for valve in 0..SAMPLE_VALVES {
        bores = bores
            + centered_cylinder(
                format!("media_gas_hold_loop_sample_valve_{valve}_cross_bore"),
                MEDIA_BORE_D / 2.0,
                SAMPLE_Y + 16.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                SAMPLE_CENTER.0 + centered_index(valve, SAMPLE_VALVES, 52.0),
                SAMPLE_CENTER.1 + SAMPLE_Y / 2.0 - 38.0,
                deck_top_z() + 30.0,
            );
    }
    bores
}

fn sample_valve_blocks() -> Part {
    let mut valves = Part::empty("media_gas_hold_loop_sample_isolation_valve_blocks");
    for valve in 0..SAMPLE_VALVES {
        valves = valves
            + centered_cube(
                format!("media_gas_hold_loop_sample_isolation_valve_block_{valve}"),
                36.0,
                24.0,
                20.0,
            )
            .translate(
                SAMPLE_CENTER.0 + centered_index(valve, SAMPLE_VALVES, 52.0),
                SAMPLE_CENTER.1 + SAMPLE_Y / 2.0 - 38.0,
                deck_top_z() + SAMPLE_Z + 10.0,
            );
    }
    valves
}

fn analyzer_handoff_pockets() -> Part {
    let ph = rectangular_frame(
        "media_gas_hold_loop_ph_analyzer_handoff_pocket",
        126.0,
        42.0,
        6.0,
        8.0,
    )
    .translate(
        SAMPLE_CENTER.0 - 84.0,
        SAMPLE_CENTER.1 - SAMPLE_Y / 2.0 + 38.0,
        deck_top_z() + SAMPLE_Z + 4.0,
    );
    let osmo = rectangular_frame(
        "media_gas_hold_loop_osmolality_analyzer_handoff_pocket",
        146.0,
        42.0,
        6.0,
        8.0,
    )
    .translate(
        SAMPLE_CENTER.0 + 92.0,
        SAMPLE_CENTER.1 - SAMPLE_Y / 2.0 + 38.0,
        deck_top_z() + SAMPLE_Z + 4.0,
    );
    ph + osmo
}

fn sample_custody_lane_tabs() -> Part {
    let ph = centered_cube(
        "media_gas_hold_loop_ph_sample_custody_lane_tab",
        SAMPLE_X - 74.0,
        8.0,
        6.0,
    )
    .translate(
        SAMPLE_CENTER.0,
        SAMPLE_CENTER.1 + centered_index(0, SAMPLE_ANALYTES, SAMPLE_PITCH_Y) - 26.0,
        deck_top_z() + SAMPLE_Z + 8.0,
    );
    let osmo = centered_cube(
        "media_gas_hold_loop_osmolality_sample_custody_lane_tab",
        SAMPLE_X - 74.0,
        8.0,
        6.0,
    )
    .translate(
        SAMPLE_CENTER.0,
        SAMPLE_CENTER.1 + centered_index(1, SAMPLE_ANALYTES, SAMPLE_PITCH_Y) + 26.0,
        deck_top_z() + SAMPLE_Z + 8.0,
    );
    ph + osmo
}

fn reservoir_perfusion_handoff_bulkhead() -> Part {
    let body = centered_cube(
        "media_gas_hold_loop_reservoir_perfusion_handoff_bulkhead_body",
        HANDOFF_X,
        HANDOFF_Y,
        HANDOFF_Z,
    )
    .translate(HANDOFF_CENTER.0, HANDOFF_CENTER.1, place_z(HANDOFF_Z));
    body - handoff_connector_bores()
        + handoff_connector_keys()
        + reservoir_dock_saddles()
        + perfusion_rack_dock_saddles()
        + handoff_route_guard_rails()
}

fn handoff_connector_bores() -> Part {
    let mut bores = Part::empty("media_gas_hold_loop_handoff_connector_bores");
    for connector in 0..CLOSED_HANDOFF_CONNECTORS {
        let row = connector / 4;
        let col = connector % 4;
        bores = bores
            + centered_cylinder(
                format!("media_gas_hold_loop_closed_handoff_connector_{connector}_bore"),
                CONNECTOR_D / 2.0,
                HANDOFF_Y + 18.0,
                DEFAULT_TESSELLATION_SEGMENTS,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                HANDOFF_CENTER.0 + centered_index(col, 4, 70.0),
                HANDOFF_CENTER.1 + centered_index(row, 2, 82.0),
                deck_top_z() + HANDOFF_Z / 2.0,
            );
    }
    bores
}

fn handoff_connector_keys() -> Part {
    let mut keys = Part::empty("media_gas_hold_loop_handoff_connector_keys");
    for connector in 0..CONNECTOR_KEY_COUNT {
        let row = connector / 4;
        let col = connector % 4;
        let x = HANDOFF_CENTER.0 + centered_index(col, 4, 70.0);
        let y = HANDOFF_CENTER.1 + centered_index(row, 2, 82.0);
        keys = keys
            + centered_cube(
                format!("media_gas_hold_loop_closed_handoff_connector_{connector}_key_flat"),
                34.0,
                8.0,
                8.0,
            )
            .translate(x, y - 20.0, deck_top_z() + HANDOFF_Z + 4.0)
            + centered_cube(
                format!("media_gas_hold_loop_closed_handoff_connector_{connector}_orientation_tab"),
                9.0,
                24.0,
                8.0,
            )
            .translate(x + 22.0, y, deck_top_z() + HANDOFF_Z + 4.0);
    }
    keys
}

fn reservoir_dock_saddles() -> Part {
    let mut saddles = Part::empty("media_gas_hold_loop_reservoir_dock_saddles");
    for dock in 0..RESERVOIR_DOCKS {
        saddles = saddles
            + rectangular_frame(
                &format!("media_gas_hold_loop_reservoir_dock_{dock}_saddle"),
                58.0,
                34.0,
                5.0,
                8.0,
            )
            .translate(
                HANDOFF_CENTER.0 + centered_index(dock, RESERVOIR_DOCKS, 82.0),
                HANDOFF_CENTER.1 + HANDOFF_Y / 2.0 - 38.0,
                deck_top_z() + HANDOFF_Z + 4.0,
            );
    }
    saddles
}

fn perfusion_rack_dock_saddles() -> Part {
    let mut saddles = Part::empty("media_gas_hold_loop_perfusion_rack_dock_saddles");
    for dock in 0..PERFUSION_RACK_DOCKS {
        saddles = saddles
            + rectangular_frame(
                &format!("media_gas_hold_loop_perfusion_rack_dock_{dock}_saddle"),
                58.0,
                34.0,
                5.0,
                8.0,
            )
            .translate(
                HANDOFF_CENTER.0 + centered_index(dock, PERFUSION_RACK_DOCKS, 82.0),
                HANDOFF_CENTER.1 - HANDOFF_Y / 2.0 + 38.0,
                deck_top_z() + HANDOFF_Z + 4.0,
            );
    }
    saddles
}

fn handoff_route_guard_rails() -> Part {
    let reservoir_side = centered_cube(
        "media_gas_hold_loop_reservoir_side_handoff_guard_rail",
        HANDOFF_X - 54.0,
        10.0,
        14.0,
    )
    .translate(
        HANDOFF_CENTER.0,
        HANDOFF_CENTER.1 + HANDOFF_Y / 2.0 - 78.0,
        deck_top_z() + HANDOFF_Z + 7.0,
    );
    let perfusion_side = centered_cube(
        "media_gas_hold_loop_perfusion_rack_side_handoff_guard_rail",
        HANDOFF_X - 54.0,
        10.0,
        14.0,
    )
    .translate(
        HANDOFF_CENTER.0,
        HANDOFF_CENTER.1 - HANDOFF_Y / 2.0 + 78.0,
        deck_top_z() + HANDOFF_Z + 7.0,
    );
    reservoir_side + perfusion_side
}

fn barcode_custody_status_surfaces() -> Part {
    let body = centered_cube(
        "media_gas_hold_loop_barcode_custody_status_body",
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    )
    .translate(TRACE_CENTER.0, TRACE_CENTER.1, place_z(TRACE_Z));
    body + barcode_lands() + custody_token_slots() + status_token_lanes()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("media_gas_hold_loop_barcode_lands");
    for land in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("media_gas_hold_loop_barcode_land_{land}"),
                56.0,
                18.0,
                4.0,
            )
            .translate(
                TRACE_CENTER.0 + centered_index(land, BARCODE_LANDS, 74.0),
                TRACE_CENTER.1 + 24.0,
                deck_top_z() + TRACE_Z + 2.0,
            );
    }
    lands
}

fn custody_token_slots() -> Part {
    let mut slots = Part::empty("media_gas_hold_loop_custody_token_slots");
    for slot in 0..CUSTODY_TOKEN_SLOTS {
        slots = slots
            + rectangular_frame(
                &format!("media_gas_hold_loop_custody_token_slot_{slot}"),
                42.0,
                24.0,
                4.0,
                5.0,
            )
            .translate(
                TRACE_CENTER.0 + centered_index(slot, CUSTODY_TOKEN_SLOTS, 74.0),
                TRACE_CENTER.1 - 8.0,
                deck_top_z() + TRACE_Z + 2.5,
            );
    }
    slots
}

fn status_token_lanes() -> Part {
    let mut lanes = Part::empty("media_gas_hold_loop_status_token_lanes");
    for lane in 0..STATUS_LANES {
        lanes = lanes
            + centered_cube(
                format!("media_gas_hold_loop_status_lane_{lane}_base"),
                118.0,
                16.0,
                5.0,
            )
            .translate(
                TRACE_CENTER.0 + status_lane_x(lane),
                TRACE_CENTER.1 - TRACE_Y / 2.0 + 16.0,
                deck_top_z() + TRACE_Z + 2.5,
            );
        for token in 0..STATUS_TOKENS_PER_LANE {
            lanes = lanes
                + centered_cube(
                    format!("media_gas_hold_loop_status_lane_{lane}_token_{token}"),
                    22.0,
                    12.0,
                    9.0,
                )
                .translate(
                    TRACE_CENTER.0
                        + status_lane_x(lane)
                        + centered_index(token, STATUS_TOKENS_PER_LANE, 32.0),
                    TRACE_CENTER.1 - TRACE_Y / 2.0 + 16.0,
                    deck_top_z() + TRACE_Z + 9.5,
                );
        }
    }
    lanes
}

fn closed_connector_cap_retention_bridge() -> Part {
    let mut bridge = Part::empty("media_gas_hold_loop_closed_connector_cap_retention_bridge");
    for segment in 0..CAP_BRIDGE_SEGMENTS {
        let x = HANDOFF_CENTER.0 + centered_index(segment, CAP_BRIDGE_SEGMENTS, 42.0);
        bridge = bridge
            + centered_cube(
                format!("media_gas_hold_loop_cap_retention_tether_segment_{segment}"),
                34.0,
                7.0,
                8.0,
            )
            .translate(
                x,
                HANDOFF_CENTER.1 - HANDOFF_Y / 2.0 - 28.0,
                deck_top_z() + 48.0,
            )
            + centered_cylinder(
                format!("media_gas_hold_loop_connector_cap_park_ring_{segment}"),
                12.0,
                5.0,
                28,
            )
            .translate(
                x,
                HANDOFF_CENTER.1 - HANDOFF_Y / 2.0 - 52.0,
                deck_top_z() + 48.0,
            );
    }
    for puck in 0..GASKET_WITNESS_PUCKS {
        bridge = bridge
            + centered_cylinder(
                format!("media_gas_hold_loop_gasket_witness_puck_{puck}"),
                10.0,
                6.0,
                28,
            )
            .translate(
                SAMPLE_CENTER.0 + centered_index(puck, GASKET_WITNESS_PUCKS, 38.0),
                SAMPLE_CENTER.1 - SAMPLE_Y / 2.0 - 34.0,
                deck_top_z() + 42.0,
            );
    }
    bridge
}

fn route_and_witness_overlay() -> Part {
    let mut overlay = Part::empty("media_gas_hold_loop_route_and_witness_overlay");
    for (i, (x1, y1, x2, y2)) in route_segments().into_iter().enumerate() {
        let width = (x2 - x1).abs().max(8.0);
        let depth = (y2 - y1).abs().max(8.0);
        overlay = overlay
            + centered_cube(
                format!("media_gas_hold_loop_route_overlay_segment_{i}"),
                width,
                depth,
                5.0,
            )
            .translate((x1 + x2) / 2.0, (y1 + y2) / 2.0, deck_top_z() + 6.0);
    }
    for marker in 0..FLOW_DIRECTION_MARKERS {
        overlay = overlay
            + centered_cube(
                format!("media_gas_hold_loop_flow_direction_marker_{marker}"),
                28.0,
                6.0,
                6.0,
            )
            .translate(
                -430.0 + marker as f64 * 78.0,
                -236.0 + if marker % 2 == 0 { 0.0 } else { 26.0 },
                deck_top_z() + 12.0,
            );
    }
    overlay
}

fn route_segments() -> [(f64, f64, f64, f64); ROUTE_SEGMENTS] {
    [
        (
            GAS_CENTER.0 + GAS_X / 2.0,
            GAS_CENTER.1,
            ENV_CENTER.0 - ENV_X / 2.0,
            ENV_CENTER.1,
        ),
        (
            ENV_CENTER.0 + ENV_X / 2.0,
            ENV_CENTER.1,
            LOOP_CENTER.0 - LOOP_X / 2.0,
            LOOP_CENTER.1,
        ),
        (
            LOOP_CENTER.0,
            LOOP_CENTER.1 - LOOP_Y / 2.0,
            HANDOFF_CENTER.0,
            HANDOFF_CENTER.1 + HANDOFF_Y / 2.0,
        ),
        (
            LOOP_CENTER.0 - LOOP_X / 2.0,
            LOOP_CENTER.1 - 28.0,
            TRAP_CENTER.0 + TRAP_X / 2.0,
            TRAP_CENTER.1 + 64.0,
        ),
        (
            TRAP_CENTER.0 + TRAP_X / 2.0,
            TRAP_CENTER.1,
            SAMPLE_CENTER.0 - SAMPLE_X / 2.0,
            SAMPLE_CENTER.1,
        ),
        (
            SAMPLE_CENTER.0 + SAMPLE_X / 2.0,
            SAMPLE_CENTER.1,
            HANDOFF_CENTER.0 - HANDOFF_X / 2.0,
            HANDOFF_CENTER.1,
        ),
        (
            SAMPLE_CENTER.0,
            SAMPLE_CENTER.1 - SAMPLE_Y / 2.0,
            TRACE_CENTER.0,
            TRACE_CENTER.1 + TRACE_Y / 2.0,
        ),
        (
            HANDOFF_CENTER.0,
            HANDOFF_CENTER.1 - HANDOFF_Y / 2.0,
            TRACE_CENTER.0 + TRACE_X / 2.0 - 90.0,
            TRACE_CENTER.1 + TRACE_Y / 2.0,
        ),
        (
            TRAP_CENTER.0,
            TRAP_CENTER.1 - TRAP_Y / 2.0,
            TRACE_CENTER.0 - TRACE_X / 2.0 + 90.0,
            TRACE_CENTER.1 + TRACE_Y / 2.0,
        ),
        (
            GAS_CENTER.0,
            GAS_CENTER.1 - GAS_Y / 2.0,
            TRAP_CENTER.0,
            TRAP_CENTER.1 + TRAP_Y / 2.0,
        ),
    ]
}

fn robot_service_keepout_gauges() -> Part {
    let front = centered_cube(
        "media_gas_hold_loop_front_robot_approach_keepout_gauge",
        STATION_X - 170.0,
        18.0,
        160.0,
    )
    .translate(0.0, -STATION_Y / 2.0 - FRONT_ROBOT_CLEARANCE / 2.0, 80.0);
    let rear = centered_cube(
        "media_gas_hold_loop_rear_gas_service_keepout_gauge",
        STATION_X - 260.0,
        16.0,
        170.0,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 + REAR_GAS_SERVICE_CLEARANCE / 2.0,
        85.0,
    );
    let left = centered_cube(
        "media_gas_hold_loop_left_bubble_trap_service_keepout_gauge",
        16.0,
        STATION_Y - 230.0,
        180.0,
    )
    .translate(
        -STATION_X / 2.0 - LEFT_TRAP_SERVICE_CLEARANCE / 2.0,
        0.0,
        90.0,
    );
    let right = centered_cube(
        "media_gas_hold_loop_right_handoff_service_keepout_gauge",
        16.0,
        STATION_Y - 230.0,
        180.0,
    )
    .translate(
        STATION_X / 2.0 + RIGHT_HANDOFF_SERVICE_CLEARANCE / 2.0,
        0.0,
        90.0,
    );
    let top = centered_cube(
        "media_gas_hold_loop_top_connector_lift_keepout_gauge",
        HANDOFF_X,
        HANDOFF_Y,
        16.0,
    )
    .translate(
        HANDOFF_CENTER.0,
        HANDOFF_CENTER.1,
        deck_top_z() + HANDOFF_Z + TOP_CONNECTOR_LIFT_CLEARANCE,
    );
    front + rear + left + right + top
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
    assert_eq!(OUTPUTS.len(), 12);
    assert!(OUTPUTS.iter().all(|path| path.starts_with(OUTPUT_PREFIX)));
    assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
    assert_eq!(
        OUTPUTS[OUTPUTS.len() - 1],
        format!("{OUTPUT_PREFIX}assembly.stl")
    );
    assert_eq!(REQUIRED_FEATURES.len(), 12);
    assert_eq!(LIMITATIONS.len(), 6);
    assert_eq!(REPRODUCIBILITY_CONTROLS.len(), 6);
    assert_eq!(UNITS, "millimeters");
    assert_eq!(GRID_STEP_MM, 2.0);
    assert_eq!(GAS_INTERFACE_PORTS, GAS_CHANNELS * GAS_PORTS_PER_CHANNEL);
    assert_eq!(ENV_WITNESS_BLOCKS, 3);
    assert_eq!(MEDIA_LOOP_PORTS, LOOP_LANES * 2);
    assert_eq!(BUBBLE_TRAP_COLUMNS, LOOP_LANES);
    assert_eq!(DEGAS_MEMBRANE_WINDOWS, LOOP_LANES);
    assert_eq!(VENT_CAPTURE_CUPS, LOOP_LANES);
    assert_eq!(SAMPLE_WELLS, SAMPLE_ANALYTES * SAMPLE_REPLICATES);
    assert_eq!(SAMPLE_VALVES, SAMPLE_ANALYTES * 3);
    assert_eq!(
        CLOSED_HANDOFF_CONNECTORS,
        RESERVOIR_DOCKS + PERFUSION_RACK_DOCKS
    );
    assert_eq!(CONNECTOR_KEY_COUNT, CLOSED_HANDOFF_CONNECTORS);
    assert_eq!(STATUS_TOKENS, STATUS_LANES * STATUS_TOKENS_PER_LANE);
    assert_eq!(CAP_BRIDGE_SEGMENTS, CLOSED_HANDOFF_CONNECTORS);
    assert_eq!(GASKET_WITNESS_PUCKS, CLOSED_HANDOFF_CONNECTORS);
    assert_eq!(route_segments().len(), ROUTE_SEGMENTS);
    assert_eq!(SERVICE_KEEP_OUT_GAUGES, 5);
    assert!(FRONT_ROBOT_CLEARANCE >= 400.0);
    assert!(REAR_GAS_SERVICE_CLEARANCE >= 260.0);
    assert!(LEFT_TRAP_SERVICE_CLEARANCE >= 200.0);
    assert!(RIGHT_HANDOFF_SERVICE_CLEARANCE >= 220.0);
    assert!(TOP_CONNECTOR_LIFT_CLEARANCE > HANDOFF_Z + 120.0);

    for required in [
        "co2_o2_blend_interface",
        "gas_equilibration_hold_loop",
        "recirculating_media_loop",
        "temperature_witness_block",
        "humidity_witness_block",
        "dewpoint_witness_block",
        "ph_sample_interface",
        "osmolality_sample_interface",
        "bubble_trap_degas_surrogate",
        "barcode_custody_surfaces",
        "status_surfaces",
        "reservoir_perfusion_closed_handoff",
    ] {
        assert!(REQUIRED_FEATURES.contains(&required));
    }

    for rect in module_rects() {
        assert!(
            rect.fits_inside_station(),
            "{} exceeds station envelope",
            rect.name
        );
    }

    let rects = module_rects();
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
    fn output_manifest_is_stable_unique_and_scoped() {
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
                "output/closed_media_conditioning_gas_equilibration_hold_loop_station_containment_deck.stl",
                "output/closed_media_conditioning_gas_equilibration_hold_loop_station_co2_o2_blend_interface_panel.stl",
                "output/closed_media_conditioning_gas_equilibration_hold_loop_station_temperature_humidity_dewpoint_witness_blocks.stl",
                "output/closed_media_conditioning_gas_equilibration_hold_loop_station_recirculating_media_hold_loop_deck.stl",
                "output/closed_media_conditioning_gas_equilibration_hold_loop_station_bubble_trap_degas_surrogate_bank.stl",
                "output/closed_media_conditioning_gas_equilibration_hold_loop_station_ph_osmolality_sample_interface.stl",
                "output/closed_media_conditioning_gas_equilibration_hold_loop_station_reservoir_perfusion_handoff_bulkhead.stl",
                "output/closed_media_conditioning_gas_equilibration_hold_loop_station_barcode_custody_status_surfaces.stl",
                "output/closed_media_conditioning_gas_equilibration_hold_loop_station_closed_connector_cap_retention_bridge.stl",
                "output/closed_media_conditioning_gas_equilibration_hold_loop_station_route_and_witness_overlay.stl",
                "output/closed_media_conditioning_gas_equilibration_hold_loop_station_robot_service_keepout_gauges.stl",
                "output/closed_media_conditioning_gas_equilibration_hold_loop_station_assembly.stl",
            ]
        );
    }

    #[test]
    fn requested_validation_features_are_represented() {
        for feature in [
            "co2_o2_blend_interface",
            "gas_equilibration_hold_loop",
            "recirculating_media_loop",
            "temperature_witness_block",
            "humidity_witness_block",
            "dewpoint_witness_block",
            "ph_sample_interface",
            "osmolality_sample_interface",
            "bubble_trap_degas_surrogate",
            "barcode_custody_surfaces",
            "status_surfaces",
            "reservoir_perfusion_closed_handoff",
        ] {
            assert!(
                REQUIRED_FEATURES.contains(&feature),
                "missing feature {feature}"
            );
        }
    }

    #[test]
    fn counts_cover_gas_media_sample_degas_and_handoff_interfaces() {
        assert_eq!(GAS_CHANNELS, 2);
        assert_eq!(GAS_NAMES, ["co2", "o2"]);
        assert_eq!(GAS_INTERFACE_PORTS, 8);
        assert_eq!(ENV_WITNESS_BLOCKS, 3);
        assert_eq!(
            TEMP_WITNESS_WELLS + HUMIDITY_WITNESS_WELLS + DEWPOINT_WITNESS_WELLS,
            14
        );
        assert_eq!(LOOP_LANES, 6);
        assert_eq!(MEDIA_LOOP_PORTS, 12);
        assert_eq!(BUBBLE_TRAP_COLUMNS, LOOP_LANES);
        assert_eq!(DEGAS_MEMBRANE_WINDOWS, LOOP_LANES);
        assert_eq!(SAMPLE_ANALYTES, 2);
        assert_eq!(SAMPLE_WELLS, 12);
        assert_eq!(CLOSED_HANDOFF_CONNECTORS, 8);
        assert_eq!(BARCODE_LANDS, 12);
        assert_eq!(STATUS_TOKENS, 12);
    }

    #[test]
    fn station_modules_fit_inside_bounds_without_major_overlap() {
        assert_design_constraints();
        let rects = module_rects();
        for rect in rects {
            assert!(rect.fits_inside_station(), "{rect:?} outside station");
        }
        for (i, left) in rects.iter().enumerate() {
            for right in rects.iter().skip(i + 1) {
                assert!(
                    !left.overlaps_with_clearance(*right, LAYOUT_CLEARANCE),
                    "{} overlaps {}",
                    left.name,
                    right.name
                );
            }
        }
    }

    #[test]
    fn reproducibility_controls_are_explicit() {
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
            "closed_media_conditioning_gas_equilibration_hold_loop_station_v1"
        );
        assert_eq!(UNITS, "millimeters");
        assert_eq!(GRID_STEP_MM, 2.0);
        assert_eq!(DEFAULT_TESSELLATION_SEGMENTS, 32);
    }

    #[test]
    fn fixture_limitations_exclude_clinical_thresholds() {
        assert!(LIMITATIONS.contains(&"validation_fixture_only"));
        assert!(LIMITATIONS.contains(&"no_clinical_acceptance_thresholds"));
        assert!(LIMITATIONS.contains(&"not_a_biological_release_protocol"));
        assert!(LIMITATIONS.contains(&"not_a_pressure_rated_gas_mixer"));
    }

    #[test]
    fn workflow_moves_from_gas_to_environment_loop_trap_sample_and_handoff() {
        assert!(GAS_CENTER.0 < ENV_CENTER.0);
        assert!(ENV_CENTER.0 < LOOP_CENTER.0);
        assert!(TRAP_CENTER.0 < SAMPLE_CENTER.0);
        assert!(SAMPLE_CENTER.0 < HANDOFF_CENTER.0);
        assert!(TRACE_CENTER.1 < SAMPLE_CENTER.1);
        assert_eq!(route_segments().len(), ROUTE_SEGMENTS);
    }
}
