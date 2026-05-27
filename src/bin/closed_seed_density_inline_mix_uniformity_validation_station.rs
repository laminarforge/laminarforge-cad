use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed seed-density inline mix-uniformity validation station.
//
// Intent:
// - Validate a closed cell-suspension stream immediately before a multi-slot
//   tissue-chip cassette load by representing a recirculating sample loop,
//   inline optical density/counting windows, gentle mixer witness coupons,
//   dead-volume and bubble traps, custody surfaces, disposition gates, and
//   robotic service datums.
// - Model only deterministic fixture geometry and interfaces. Cell-density
//   acceptance limits, process recipes, and disposition logic remain external
//   quality-system data.

const OUTPUT_PREFIX: &str = "closed_seed_density_inline_mix_uniformity_validation_station";

const OUTPUTS: [&str; 12] = [
    "output/closed_seed_density_inline_mix_uniformity_validation_station_leak_tray_deck.stl",
    "output/closed_seed_density_inline_mix_uniformity_validation_station_multi_slot_tissue_chip_cassette_datum_nest.stl",
    "output/closed_seed_density_inline_mix_uniformity_validation_station_closed_recirculating_sample_loop_surrogate.stl",
    "output/closed_seed_density_inline_mix_uniformity_validation_station_inline_optical_density_window_bank.stl",
    "output/closed_seed_density_inline_mix_uniformity_validation_station_cell_counting_window_bank.stl",
    "output/closed_seed_density_inline_mix_uniformity_validation_station_gentle_mixer_witness_coupon_rack.stl",
    "output/closed_seed_density_inline_mix_uniformity_validation_station_dead_volume_bubble_trap_bank.stl",
    "output/closed_seed_density_inline_mix_uniformity_validation_station_barcode_custody_surface_panel.stl",
    "output/closed_seed_density_inline_mix_uniformity_validation_station_reject_hold_release_gate_array.stl",
    "output/closed_seed_density_inline_mix_uniformity_validation_station_robotic_service_datum_bridge.stl",
    "output/closed_seed_density_inline_mix_uniformity_validation_station_closed_loop_route_harness.stl",
    "output/closed_seed_density_inline_mix_uniformity_validation_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 15] = [
    "multi_slot_tissue_chip_cassette_datum_nest",
    "cassette_slot_loading_lanes",
    "closed_recirculating_sample_loop_surrogate",
    "sample_loop_pinch_valve_ladder",
    "inline_optical_density_window_bank",
    "cell_counting_window_bank",
    "gentle_mixer_witness_coupon_rack",
    "mixer_residence_time_token_lands",
    "dead_volume_bubble_trap_bank",
    "barcode_custody_surface_panel",
    "reject_hold_release_gate_array",
    "robotic_service_datum_bridge",
    "closed_loop_route_harness",
    "cassette_load_handoff_manifold",
    "bubble_escape_level_ticks",
];

const DISPOSITION_LANES: [&str; 3] = ["reject", "hold", "release"];

const DECK_X: f64 = 1540.0;
const DECK_Y: f64 = 930.0;
const DECK_Z: f64 = 22.0;
const TRAY_RIM_W: f64 = 20.0;
const TRAY_RIM_Z: f64 = 38.0;
const SUMP_DEPTH: f64 = 7.0;
const DRAIN_CHANNEL_W: f64 = 14.0;
const MOUNT_HOLE_D: f64 = 6.8;
const LEAK_WITNESS_WELLS: usize = 12;
const MACHINE_DATUM_TARGETS: usize = 4;

const LOOP_CENTER: (f64, f64) = (-485.0, 190.0);
const LOOP_X: f64 = 410.0;
const LOOP_Y: f64 = 230.0;
const LOOP_Z: f64 = 58.0;
const SAMPLE_LOOP_PORTS: usize = 8;
const SAMPLE_LOOP_PINCH_VALVES: usize = 8;
const SAMPLE_LOOP_PRESSURE_TAPS: usize = 4;
const SAMPLE_LOOP_TUBE_D: f64 = 7.2;
const SAMPLE_LOOP_BORE_D: f64 = 5.0;
const SAMPLE_LOOP_BRANCHES: usize = 4;

const OD_CENTER: (f64, f64) = (-60.0, 190.0);
const OD_X: f64 = 330.0;
const OD_Y: f64 = 230.0;
const OD_Z: f64 = 58.0;
const OD_WINDOWS: usize = 6;
const OD_REFERENCE_LANDS: usize = 4;
const OD_EMITTER_RAILS: usize = 2;
const OD_PATH_LENGTH_MM: f64 = 10.0;

const CASSETTE_CENTER: (f64, f64) = (445.0, 190.0);
const CASSETTE_X: f64 = 420.0;
const CASSETTE_Y: f64 = 250.0;
const CASSETTE_Z: f64 = 48.0;
const CASSETTE_ROWS: usize = 3;
const CASSETTE_COLS: usize = 4;
const CASSETTE_SLOTS: usize = CASSETTE_ROWS * CASSETTE_COLS;
const CASSETTE_SLOT_X: f64 = 62.0;
const CASSETTE_SLOT_Y: f64 = 42.0;
const CASSETTE_SLOT_PITCH_X: f64 = 78.0;
const CASSETTE_SLOT_PITCH_Y: f64 = 62.0;
const CASSETTE_DATUM_PINS: usize = 4;
const CASSETTE_LOAD_LATCHES: usize = 6;
const CASSETTE_LANE_PORTS: usize = CASSETTE_SLOTS;

const MIXER_CENTER: (f64, f64) = (-485.0, -105.0);
const MIXER_X: f64 = 410.0;
const MIXER_Y: f64 = 230.0;
const MIXER_Z: f64 = 42.0;
const MIXER_COUPONS: usize = 8;
const MIXER_COUPON_X: f64 = 70.0;
const MIXER_COUPON_Y: f64 = 32.0;
const MIXER_WITNESS_ROWS: usize = 2;
const MIXER_WITNESS_COLS: usize = 4;
const MIXER_RESIDENCE_TOKENS: [usize; MIXER_COUPONS] = [0, 1, 2, 4, 6, 8, 12, 16];
const MIXER_SWEEP_ARC_TICKS: usize = 13;
const MIXER_BAFFLE_RIBS: usize = 7;

const TRAP_CENTER: (f64, f64) = (-60.0, -105.0);
const TRAP_X: f64 = 330.0;
const TRAP_Y: f64 = 230.0;
const TRAP_Z: f64 = 62.0;
const BUBBLE_TRAPS: usize = 5;
const DEAD_VOLUME_WELLS: usize = 6;
const TRAP_LEVEL_TICKS: usize = 9;
const TRAP_PURGE_PORTS: usize = 5;

const COUNT_CENTER: (f64, f64) = (390.0, -105.0);
const COUNT_X: f64 = 410.0;
const COUNT_Y: f64 = 230.0;
const COUNT_Z: f64 = 66.0;
const COUNT_WINDOWS: usize = 4;
const COUNT_ELECTRODE_PAIRS: usize = 8;
const COUNT_CAMERA_DATUMS: usize = 4;
const COUNT_CAPILLARY_LANES: usize = 4;

const CUSTODY_CENTER: (f64, f64) = (-310.0, -355.0);
const CUSTODY_X: f64 = 540.0;
const CUSTODY_Y: f64 = 118.0;
const CUSTODY_Z: f64 = 24.0;
const BARCODE_LANDS: usize = 12;
const RFID_LANDS: usize = 4;
const LOT_CARD_SLOTS: usize = 6;
const CUSTODY_SEAL_LANDS: usize = 6;

const GATE_CENTER: (f64, f64) = (300.0, -355.0);
const GATE_X: f64 = 500.0;
const GATE_Y: f64 = 118.0;
const GATE_Z: f64 = 50.0;
const GATE_SLOTS_PER_LANE: usize = 4;
const GATE_SOLENOIDS: usize = DISPOSITION_LANES.len() * 2;
const GATE_LANE_PITCH_X: f64 = 150.0;
const GATE_DECISION_INPUTS: usize = DISPOSITION_LANES.len() * 4;

const ROUTE_Z: f64 = DECK_Z + 88.0;
const ROUTE_TUBE_D: f64 = 7.0;
const ROUTE_SEGMENTS: usize = 12;
const ROUTE_ELBOWS: usize = 10;
const ROUTE_DIRECTION_MARKERS: usize = 10;

const SERVICE_BRIDGE_Z: f64 = DECK_Z + 14.0;
const SERVICE_DATUM_TOWERS: usize = 6;
const ROBOT_DOCKING_DATUMS: usize = 4;
const SERVICE_CLEARANCE_GAUGES: usize = 5;
const FRONT_ROBOT_CLEARANCE: f64 = 360.0;
const REAR_CASSETTE_CLEARANCE: f64 = 245.0;
const LEFT_LOOP_SERVICE_CLEARANCE: f64 = 255.0;
const RIGHT_CASSETTE_SERVICE_CLEARANCE: f64 = 250.0;
const TOP_SENSOR_LIFT_CLEARANCE_Z: f64 = 300.0;

#[derive(Clone, Copy, Debug)]
struct ModuleSpec {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
    z: f64,
}

impl ModuleSpec {
    fn fits_on_deck(self) -> bool {
        let usable_x = DECK_X / 2.0 - TRAY_RIM_W - 12.0;
        let usable_y = DECK_Y / 2.0 - TRAY_RIM_W - 12.0;

        self.center.0.abs() + self.x / 2.0 <= usable_x
            && self.center.1.abs() + self.y / 2.0 <= usable_y
    }

    fn overlaps(self, other: ModuleSpec) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();

        dx < (self.x + other.x) / 2.0 && dy < (self.y + other.y) / 2.0
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = leak_tray_deck();
    write_part(OUTPUTS[0], &deck);

    let cassette = multi_slot_tissue_chip_cassette_datum_nest();
    write_part(OUTPUTS[1], &cassette);

    let sample_loop = closed_recirculating_sample_loop_surrogate();
    write_part(OUTPUTS[2], &sample_loop);

    let od_bank = inline_optical_density_window_bank();
    write_part(OUTPUTS[3], &od_bank);

    let count_bank = cell_counting_window_bank();
    write_part(OUTPUTS[4], &count_bank);

    let mixer = gentle_mixer_witness_coupon_rack();
    write_part(OUTPUTS[5], &mixer);

    let traps = dead_volume_bubble_trap_bank();
    write_part(OUTPUTS[6], &traps);

    let custody = barcode_custody_surface_panel();
    write_part(OUTPUTS[7], &custody);

    let gates = reject_hold_release_gate_array();
    write_part(OUTPUTS[8], &gates);

    let datums = robotic_service_datum_bridge();
    write_part(OUTPUTS[9], &datums);

    let routes = closed_loop_route_harness();
    write_part(OUTPUTS[10], &routes);

    let assembly = station_assembly();
    write_part(OUTPUTS[11], &assembly);

    println!();
    println!("Closed seed-density inline mix-uniformity validation station:");
    println!(
        "  Cassette loading: {CASSETTE_SLOTS} tissue-chip slots in {CASSETTE_ROWS}x{CASSETTE_COLS} lanes with {CASSETTE_DATUM_PINS} datum pins"
    );
    println!(
        "  Closed loop: {SAMPLE_LOOP_PORTS} sterile ports, {SAMPLE_LOOP_PINCH_VALVES} pinch valves, {SAMPLE_LOOP_PRESSURE_TAPS} pressure taps, {ROUTE_SEGMENTS} routed segments"
    );
    println!(
        "  Inline readings: {OD_WINDOWS} OD windows at {OD_PATH_LENGTH_MM:.0}mm path, {COUNT_WINDOWS} counting windows, {COUNT_ELECTRODE_PAIRS} electrode pairs"
    );
    println!(
        "  Mixing evidence: {MIXER_COUPONS} witness coupons, residence tokens {:?}, {BUBBLE_TRAPS} bubble traps, {DEAD_VOLUME_WELLS} dead-volume wells",
        MIXER_RESIDENCE_TOKENS
    );
    println!(
        "  Custody/disposition: {BARCODE_LANDS} barcode lands, {RFID_LANDS} RFID lands, lanes {:?}, {GATE_SOLENOIDS} solenoids",
        DISPOSITION_LANES
    );
}

fn write_part(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    leak_tray_deck()
        + closed_recirculating_sample_loop_surrogate().translate(
            LOOP_CENTER.0,
            LOOP_CENTER.1,
            on_deck_z(LOOP_Z),
        )
        + inline_optical_density_window_bank().translate(OD_CENTER.0, OD_CENTER.1, on_deck_z(OD_Z))
        + multi_slot_tissue_chip_cassette_datum_nest().translate(
            CASSETTE_CENTER.0,
            CASSETTE_CENTER.1,
            on_deck_z(CASSETTE_Z),
        )
        + gentle_mixer_witness_coupon_rack().translate(
            MIXER_CENTER.0,
            MIXER_CENTER.1,
            on_deck_z(MIXER_Z),
        )
        + dead_volume_bubble_trap_bank().translate(TRAP_CENTER.0, TRAP_CENTER.1, on_deck_z(TRAP_Z))
        + cell_counting_window_bank().translate(COUNT_CENTER.0, COUNT_CENTER.1, on_deck_z(COUNT_Z))
        + barcode_custody_surface_panel().translate(
            CUSTODY_CENTER.0,
            CUSTODY_CENTER.1,
            on_deck_z(CUSTODY_Z),
        )
        + reject_hold_release_gate_array().translate(
            GATE_CENTER.0,
            GATE_CENTER.1,
            on_deck_z(GATE_Z),
        )
        + closed_loop_route_harness()
        + robotic_service_datum_bridge()
}

fn on_deck_z(height: f64) -> f64 {
    DECK_Z + height / 2.0
}

fn leak_tray_deck() -> Part {
    let deck = centered_cube(
        format!("{OUTPUT_PREFIX}_deck_floor"),
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let shallow_sump = centered_cube(
        format!("{OUTPUT_PREFIX}_closed_loop_spill_sump_cut"),
        DECK_X - 150.0,
        DECK_Y - 154.0,
        SUMP_DEPTH + 1.0,
    )
    .translate(0.0, -10.0, DECK_Z - SUMP_DEPTH / 2.0 + 0.4);
    let drain_channel = centered_cube(
        format!("{OUTPUT_PREFIX}_front_drain_channel_cut"),
        DECK_X - 230.0,
        DRAIN_CHANNEL_W,
        SUMP_DEPTH + 1.2,
    )
    .translate(0.0, -DECK_Y / 2.0 + 74.0, DECK_Z - SUMP_DEPTH / 2.0 + 0.5);

    deck - shallow_sump - drain_channel - deck_mount_holes()
        + tray_rims()
        + module_floor_markers()
        + leak_witness_well_bank()
        + machine_vision_datum_targets()
        + zone_divider_ribs()
}

fn tray_rims() -> Part {
    let front = centered_cube(
        format!("{OUTPUT_PREFIX}_front_containment_rim"),
        DECK_X,
        TRAY_RIM_W,
        TRAY_RIM_Z,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + TRAY_RIM_W / 2.0,
        DECK_Z + TRAY_RIM_Z / 2.0,
    );
    let rear = centered_cube(
        format!("{OUTPUT_PREFIX}_rear_containment_rim"),
        DECK_X,
        TRAY_RIM_W,
        TRAY_RIM_Z,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - TRAY_RIM_W / 2.0,
        DECK_Z + TRAY_RIM_Z / 2.0,
    );
    let left = centered_cube(
        format!("{OUTPUT_PREFIX}_left_containment_rim"),
        TRAY_RIM_W,
        DECK_Y,
        TRAY_RIM_Z,
    )
    .translate(
        -DECK_X / 2.0 + TRAY_RIM_W / 2.0,
        0.0,
        DECK_Z + TRAY_RIM_Z / 2.0,
    );
    let right = centered_cube(
        format!("{OUTPUT_PREFIX}_right_containment_rim"),
        TRAY_RIM_W,
        DECK_Y,
        TRAY_RIM_Z,
    )
    .translate(
        DECK_X / 2.0 - TRAY_RIM_W / 2.0,
        0.0,
        DECK_Z + TRAY_RIM_Z / 2.0,
    );

    front + rear + left + right
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty(format!("{OUTPUT_PREFIX}_deck_mount_holes"));
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 58.0, -DECK_Y / 2.0 + 58.0),
        (0.0, -DECK_Y / 2.0 + 58.0),
        (DECK_X / 2.0 - 58.0, -DECK_Y / 2.0 + 58.0),
        (-DECK_X / 2.0 + 58.0, DECK_Y / 2.0 - 58.0),
        (0.0, DECK_Y / 2.0 - 58.0),
        (DECK_X / 2.0 - 58.0, DECK_Y / 2.0 - 58.0),
        (-DECK_X / 2.0 + 58.0, 0.0),
        (DECK_X / 2.0 - 58.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_m6_mount_clearance_{i}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 4.0,
                28,
            )
            .translate(*x, *y, DECK_Z / 2.0);
    }
    holes
}

fn module_floor_markers() -> Part {
    let mut markers = Part::empty(format!("{OUTPUT_PREFIX}_module_floor_markers"));
    for module in module_specs() {
        markers = markers
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{}_floor_marker", module.name),
                module.x + 12.0,
                module.y + 12.0,
                3.0,
            )
            .translate(module.center.0, module.center.1, DECK_Z + 1.5);
    }
    markers
}

fn leak_witness_well_bank() -> Part {
    let mut wells = Part::empty(format!("{OUTPUT_PREFIX}_leak_witness_well_bank"));
    for i in 0..LEAK_WITNESS_WELLS {
        wells = wells
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_closed_loop_leak_witness_well_{i}"),
                10.5,
                6.0,
                28,
            )
            .translate(
                centered_index(i % 6, 6, 76.0),
                -DECK_Y / 2.0 + 110.0 + (i / 6) as f64 * 38.0,
                DECK_Z + 3.0,
            );
    }
    wells
}

fn machine_vision_datum_targets() -> Part {
    let mut targets = Part::empty(format!("{OUTPUT_PREFIX}_machine_vision_datum_targets"));
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 108.0, -DECK_Y / 2.0 + 104.0),
        (DECK_X / 2.0 - 108.0, -DECK_Y / 2.0 + 104.0),
        (-DECK_X / 2.0 + 108.0, DECK_Y / 2.0 - 104.0),
        (DECK_X / 2.0 - 108.0, DECK_Y / 2.0 - 104.0),
    ]
    .iter()
    .enumerate()
    {
        targets = targets
            + datum_target(&format!("{OUTPUT_PREFIX}_deck_datum_target_{i}")).translate(
                *x,
                *y,
                DECK_Z + 2.5,
            );
    }
    targets
}

fn zone_divider_ribs() -> Part {
    let recirc_to_measure = centered_cube(
        format!("{OUTPUT_PREFIX}_recirculation_to_measurement_divider"),
        10.0,
        530.0,
        26.0,
    )
    .translate(-270.0, 42.0, DECK_Z + 13.0);
    let measure_to_cassette = centered_cube(
        format!("{OUTPUT_PREFIX}_measurement_to_cassette_divider"),
        10.0,
        530.0,
        26.0,
    )
    .translate(170.0, 42.0, DECK_Z + 13.0);
    let disposition_divider = centered_cube(
        format!("{OUTPUT_PREFIX}_disposition_custody_divider"),
        DECK_X - 220.0,
        10.0,
        24.0,
    )
    .translate(0.0, -285.0, DECK_Z + 12.0);

    recirc_to_measure + measure_to_cassette + disposition_divider
}

fn multi_slot_tissue_chip_cassette_datum_nest() -> Part {
    let base = centered_cube(
        format!("{OUTPUT_PREFIX}_cassette_nest_body"),
        CASSETTE_X,
        CASSETTE_Y,
        CASSETTE_Z,
    );
    let cassette_pocket = centered_cube(
        format!("{OUTPUT_PREFIX}_cassette_socket_pocket_cut"),
        CASSETTE_X - 84.0,
        CASSETTE_Y - 70.0,
        12.0,
    )
    .translate(0.0, 0.0, CASSETTE_Z / 2.0 - 6.0);

    base - cassette_pocket - cassette_slot_pocket_cuts()
        + cassette_slot_loading_lanes()
        + cassette_datum_pins()
        + cassette_load_latches()
        + cassette_lane_port_saddles()
        + cassette_load_handoff_manifold()
}

fn cassette_slot_pocket_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_cassette_slot_pocket_cuts"));
    for slot in 0..CASSETTE_SLOTS {
        let row = slot / CASSETTE_COLS;
        let col = slot % CASSETTE_COLS;
        cuts = cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_tissue_chip_slot_pocket_cut_{slot}"),
                CASSETTE_SLOT_X,
                CASSETTE_SLOT_Y,
                18.0,
            )
            .translate(
                centered_index(col, CASSETTE_COLS, CASSETTE_SLOT_PITCH_X),
                centered_index(row, CASSETTE_ROWS, CASSETTE_SLOT_PITCH_Y),
                CASSETTE_Z / 2.0 - 6.0,
            );
    }
    cuts
}

fn cassette_slot_loading_lanes() -> Part {
    let mut lanes = Part::empty(format!("{OUTPUT_PREFIX}_cassette_slot_loading_lanes"));
    for slot in 0..CASSETTE_SLOTS {
        let row = slot / CASSETTE_COLS;
        let col = slot % CASSETTE_COLS;
        let x = centered_index(col, CASSETTE_COLS, CASSETTE_SLOT_PITCH_X);
        let y = centered_index(row, CASSETTE_ROWS, CASSETTE_SLOT_PITCH_Y);
        let frame = centered_cube(
            format!("{OUTPUT_PREFIX}_cassette_slot_frame_{slot}"),
            CASSETTE_SLOT_X + 18.0,
            CASSETTE_SLOT_Y + 16.0,
            8.0,
        )
        .translate(x, y, CASSETTE_Z / 2.0 + 4.0);
        let opening = centered_cube(
            format!("{OUTPUT_PREFIX}_cassette_slot_frame_opening_cut_{slot}"),
            CASSETTE_SLOT_X,
            CASSETTE_SLOT_Y,
            9.0,
        )
        .translate(x, y, CASSETTE_Z / 2.0 + 4.5);
        let lane_key = centered_cube(
            format!("{OUTPUT_PREFIX}_cassette_slot_lane_key_{slot}"),
            42.0,
            6.0,
            6.0,
        )
        .translate(x, y - CASSETTE_SLOT_Y / 2.0 - 9.0, CASSETTE_Z / 2.0 + 3.0);
        lanes = lanes + (frame - opening) + lane_key;
    }
    lanes
}

fn cassette_datum_pins() -> Part {
    let mut pins = Part::empty(format!("{OUTPUT_PREFIX}_cassette_datum_pins"));
    for i in 0..CASSETTE_DATUM_PINS {
        let x = if i % 2 == 0 { -1.0 } else { 1.0 } * (CASSETTE_X / 2.0 - 46.0);
        let y = if i < 2 { -1.0 } else { 1.0 } * (CASSETTE_Y / 2.0 - 42.0);
        pins = pins
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_cassette_kinematic_datum_pin_{i}"),
                5.0,
                16.0,
                28,
            )
            .translate(x, y, CASSETTE_Z / 2.0 + 8.0);
    }
    pins
}

fn cassette_load_latches() -> Part {
    let mut latches = Part::empty(format!("{OUTPUT_PREFIX}_cassette_load_latches"));
    for i in 0..CASSETTE_LOAD_LATCHES {
        let x = centered_index(i % 3, 3, 122.0);
        let y = if i < 3 {
            -CASSETTE_Y / 2.0 + 24.0
        } else {
            CASSETTE_Y / 2.0 - 24.0
        };
        latches = latches
            + centered_cube(
                format!("{OUTPUT_PREFIX}_cassette_load_latch_{i}"),
                56.0,
                20.0,
                18.0,
            )
            .translate(x, y, CASSETTE_Z / 2.0 + 9.0);
    }
    latches
}

fn cassette_lane_port_saddles() -> Part {
    let mut saddles = Part::empty(format!("{OUTPUT_PREFIX}_cassette_lane_port_saddles"));
    for slot in 0..CASSETTE_LANE_PORTS {
        let col = slot % CASSETTE_COLS;
        let row = slot / CASSETTE_COLS;
        let boss = centered_cube(
            format!("{OUTPUT_PREFIX}_cassette_lane_port_saddle_{slot}"),
            30.0,
            16.0,
            12.0,
        )
        .translate(
            centered_index(col, CASSETTE_COLS, CASSETTE_SLOT_PITCH_X),
            -CASSETTE_Y / 2.0 + 34.0 + row as f64 * 14.0,
            CASSETTE_Z / 2.0 + 6.0,
        );
        let bore = centered_cylinder(
            format!("{OUTPUT_PREFIX}_cassette_lane_port_saddle_bore_{slot}"),
            SAMPLE_LOOP_BORE_D / 2.0,
            18.0,
            18,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            centered_index(col, CASSETTE_COLS, CASSETTE_SLOT_PITCH_X),
            -CASSETTE_Y / 2.0 + 34.0 + row as f64 * 14.0,
            CASSETTE_Z / 2.0 + 6.0,
        );
        saddles = saddles + (boss - bore);
    }
    saddles
}

fn cassette_load_handoff_manifold() -> Part {
    let rail = x_tube(
        format!("{OUTPUT_PREFIX}_cassette_load_handoff_common_rail"),
        -150.0,
        150.0,
        CASSETTE_Y / 2.0 - 54.0,
        CASSETTE_Z / 2.0 + 18.0,
        ROUTE_TUBE_D / 2.0,
    );
    let mut droppers = Part::empty(format!("{OUTPUT_PREFIX}_cassette_load_handoff_droppers"));
    for i in 0..CASSETTE_COLS {
        droppers = droppers
            + y_tube(
                format!("{OUTPUT_PREFIX}_cassette_load_handoff_dropper_{i}"),
                centered_index(i, CASSETTE_COLS, CASSETTE_SLOT_PITCH_X),
                CASSETTE_Y / 2.0 - 54.0,
                CASSETTE_Y / 2.0 - 92.0,
                CASSETTE_Z / 2.0 + 18.0,
                ROUTE_TUBE_D / 2.0,
            );
    }
    rail + droppers
}

fn closed_recirculating_sample_loop_surrogate() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_sample_loop_body"),
        LOOP_X,
        LOOP_Y,
        LOOP_Z,
    );
    let loop_recess = centered_cube(
        format!("{OUTPUT_PREFIX}_sample_loop_clear_recirculation_window_cut"),
        LOOP_X - 90.0,
        LOOP_Y - 78.0,
        16.0,
    )
    .translate(0.0, 0.0, LOOP_Z / 2.0 - 8.0);

    body - loop_recess
        + sample_loop_perimeter_tubes()
        + sample_loop_sterile_ports()
        + sample_loop_pinch_valve_ladder()
        + sample_loop_pressure_tap_row()
        + sample_loop_branch_takeoffs()
        + sample_loop_direction_markers()
}

fn sample_loop_perimeter_tubes() -> Part {
    let left = y_tube(
        format!("{OUTPUT_PREFIX}_sample_loop_left_recirc_tube"),
        -LOOP_X / 2.0 + 68.0,
        -LOOP_Y / 2.0 + 50.0,
        LOOP_Y / 2.0 - 50.0,
        LOOP_Z / 2.0 + 18.0,
        SAMPLE_LOOP_TUBE_D / 2.0,
    );
    let right = y_tube(
        format!("{OUTPUT_PREFIX}_sample_loop_right_recirc_tube"),
        LOOP_X / 2.0 - 68.0,
        -LOOP_Y / 2.0 + 50.0,
        LOOP_Y / 2.0 - 50.0,
        LOOP_Z / 2.0 + 18.0,
        SAMPLE_LOOP_TUBE_D / 2.0,
    );
    let rear = x_tube(
        format!("{OUTPUT_PREFIX}_sample_loop_rear_recirc_tube"),
        -LOOP_X / 2.0 + 68.0,
        LOOP_X / 2.0 - 68.0,
        LOOP_Y / 2.0 - 50.0,
        LOOP_Z / 2.0 + 18.0,
        SAMPLE_LOOP_TUBE_D / 2.0,
    );
    let front = x_tube(
        format!("{OUTPUT_PREFIX}_sample_loop_front_recirc_tube"),
        -LOOP_X / 2.0 + 68.0,
        LOOP_X / 2.0 - 68.0,
        -LOOP_Y / 2.0 + 50.0,
        LOOP_Z / 2.0 + 18.0,
        SAMPLE_LOOP_TUBE_D / 2.0,
    );

    left + right + rear + front
}

fn sample_loop_sterile_ports() -> Part {
    let mut ports = Part::empty(format!("{OUTPUT_PREFIX}_sample_loop_sterile_ports"));
    for i in 0..SAMPLE_LOOP_PORTS {
        let top_side = i < SAMPLE_LOOP_PORTS / 2;
        let x = centered_index(i % (SAMPLE_LOOP_PORTS / 2), SAMPLE_LOOP_PORTS / 2, 72.0);
        let y = if top_side {
            LOOP_Y / 2.0 - 28.0
        } else {
            -LOOP_Y / 2.0 + 28.0
        };
        let boss = centered_cylinder(
            format!("{OUTPUT_PREFIX}_sample_loop_sterile_port_boss_{i}"),
            15.0,
            16.0,
            32,
        )
        .translate(x, y, LOOP_Z / 2.0 + 8.0);
        let bore = centered_cylinder(
            format!("{OUTPUT_PREFIX}_sample_loop_sterile_port_bore_{i}"),
            SAMPLE_LOOP_BORE_D / 2.0,
            18.0,
            20,
        )
        .translate(x, y, LOOP_Z / 2.0 + 9.0);
        ports = ports + (boss - bore);
    }
    ports
}

fn sample_loop_pinch_valve_ladder() -> Part {
    let mut valves = Part::empty(format!("{OUTPUT_PREFIX}_sample_loop_pinch_valve_ladder"));
    for i in 0..SAMPLE_LOOP_PINCH_VALVES {
        let x = if i < 4 {
            centered_index(i, 4, 72.0)
        } else if i % 2 == 0 {
            -LOOP_X / 2.0 + 92.0
        } else {
            LOOP_X / 2.0 - 92.0
        };
        let y = if i < 4 {
            0.0
        } else {
            centered_index((i - 4) / 2, 2, 58.0)
        };
        let valve = centered_cube(
            format!("{OUTPUT_PREFIX}_sample_loop_pinch_valve_body_{i}"),
            42.0,
            24.0,
            22.0,
        )
        .translate(x, y, LOOP_Z / 2.0 + 11.0);
        let clearance = x_tube(
            format!("{OUTPUT_PREFIX}_sample_loop_pinch_valve_tube_clearance_{i}"),
            x - 23.0,
            x + 23.0,
            y,
            LOOP_Z / 2.0 + 11.0,
            SAMPLE_LOOP_TUBE_D / 2.0,
        );
        valves = valves + (valve - clearance);
    }
    valves
}

fn sample_loop_pressure_tap_row() -> Part {
    let mut taps = Part::empty(format!("{OUTPUT_PREFIX}_sample_loop_pressure_taps"));
    for i in 0..SAMPLE_LOOP_PRESSURE_TAPS {
        taps = taps
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_sample_loop_pressure_tap_{i}"),
                7.5,
                18.0,
                28,
            )
            .translate(
                centered_index(i, SAMPLE_LOOP_PRESSURE_TAPS, 72.0),
                -26.0,
                LOOP_Z / 2.0 + 9.0,
            );
    }
    taps
}

fn sample_loop_branch_takeoffs() -> Part {
    let mut branches = Part::empty(format!("{OUTPUT_PREFIX}_sample_loop_branch_takeoffs"));
    for i in 0..SAMPLE_LOOP_BRANCHES {
        let y = centered_index(i, SAMPLE_LOOP_BRANCHES, 34.0);
        let block = centered_cube(
            format!("{OUTPUT_PREFIX}_sample_loop_branch_takeoff_block_{i}"),
            34.0,
            17.0,
            16.0,
        )
        .translate(0.0, y, LOOP_Z / 2.0 + 8.0);
        let bore = x_tube(
            format!("{OUTPUT_PREFIX}_sample_loop_branch_takeoff_bore_{i}"),
            -20.0,
            20.0,
            y,
            LOOP_Z / 2.0 + 8.0,
            SAMPLE_LOOP_BORE_D / 2.0,
        );
        branches = branches + (block - bore);
    }
    branches
}

fn sample_loop_direction_markers() -> Part {
    direction_marker(
        format!("{OUTPUT_PREFIX}_sample_loop_forward_direction_marker"),
        1.0,
        0.0,
    )
    .translate(-96.0, LOOP_Y / 2.0 - 54.0, LOOP_Z / 2.0 + 6.0)
        + direction_marker(
            format!("{OUTPUT_PREFIX}_sample_loop_return_direction_marker"),
            -1.0,
            0.0,
        )
        .translate(96.0, -LOOP_Y / 2.0 + 54.0, LOOP_Z / 2.0 + 6.0)
}

fn inline_optical_density_window_bank() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_od_window_bank_body"),
        OD_X,
        OD_Y,
        OD_Z,
    );
    let dark_tunnel = centered_cube(
        format!("{OUTPUT_PREFIX}_od_dark_tunnel_cut"),
        OD_X - 72.0,
        62.0,
        24.0,
    )
    .translate(0.0, -6.0, OD_Z / 2.0 - 7.0);

    body - dark_tunnel - od_window_cuts()
        + od_window_frames()
        + od_emitter_detector_rails()
        + od_reference_lands()
        + od_path_length_gauges()
        + od_light_baffle_fingers()
}

fn od_window_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_od_window_cuts"));
    for i in 0..OD_WINDOWS {
        cuts = cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_od_window_cut_{i}"),
                28.0,
                42.0,
                OD_Z + 3.0,
            )
            .translate(centered_index(i, OD_WINDOWS, 43.0), -10.0, 0.0);
    }
    cuts
}

fn od_window_frames() -> Part {
    let mut frames = Part::empty(format!("{OUTPUT_PREFIX}_od_window_frames"));
    for i in 0..OD_WINDOWS {
        let x = centered_index(i, OD_WINDOWS, 43.0);
        let frame = centered_cube(
            format!("{OUTPUT_PREFIX}_od_window_frame_{i}"),
            42.0,
            58.0,
            8.0,
        )
        .translate(x, -10.0, OD_Z / 2.0 + 4.0);
        let opening = centered_cube(
            format!("{OUTPUT_PREFIX}_od_window_frame_opening_cut_{i}"),
            28.0,
            42.0,
            9.0,
        )
        .translate(x, -10.0, OD_Z / 2.0 + 4.5);
        frames = frames + (frame - opening);
    }
    frames
}

fn od_emitter_detector_rails() -> Part {
    let mut rails = Part::empty(format!("{OUTPUT_PREFIX}_od_emitter_detector_rails"));
    for i in 0..OD_EMITTER_RAILS {
        rails = rails
            + centered_cube(
                format!("{OUTPUT_PREFIX}_od_emitter_detector_rail_{i}"),
                OD_X - 80.0,
                14.0,
                18.0,
            )
            .translate(0.0, if i == 0 { -70.0 } else { 48.0 }, OD_Z / 2.0 + 9.0);
    }
    rails
}

fn od_reference_lands() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_od_reference_lands"));
    for i in 0..OD_REFERENCE_LANDS {
        let puck = centered_cylinder(
            format!("{OUTPUT_PREFIX}_od_neutral_density_reference_puck_{i}"),
            13.0,
            8.0,
            30,
        )
        .translate(
            centered_index(i, OD_REFERENCE_LANDS, 52.0),
            OD_Y / 2.0 - 36.0,
            OD_Z / 2.0 + 4.0,
        );
        lands = lands + puck;
    }
    lands
}

fn od_path_length_gauges() -> Part {
    let mut gauges = Part::empty(format!("{OUTPUT_PREFIX}_od_path_length_gauges"));
    for i in 0..OD_WINDOWS {
        gauges = gauges
            + centered_cube(
                format!("{OUTPUT_PREFIX}_od_{OD_PATH_LENGTH_MM:.0}mm_path_length_gauge_{i}"),
                18.0,
                7.0,
                5.0,
            )
            .translate(
                centered_index(i, OD_WINDOWS, 43.0),
                -OD_Y / 2.0 + 25.0,
                OD_Z / 2.0 + 2.5,
            );
    }
    gauges
}

fn od_light_baffle_fingers() -> Part {
    let mut fingers = Part::empty(format!("{OUTPUT_PREFIX}_od_light_baffle_fingers"));
    for i in 0..OD_WINDOWS + 1 {
        fingers = fingers
            + centered_cube(
                format!("{OUTPUT_PREFIX}_od_light_baffle_finger_{i}"),
                6.0,
                96.0,
                16.0,
            )
            .translate(
                centered_index(i, OD_WINDOWS + 1, 43.0) - 21.5,
                -10.0,
                OD_Z / 2.0 + 8.0,
            );
    }
    fingers
}

fn cell_counting_window_bank() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_counting_window_bank_body"),
        COUNT_X,
        COUNT_Y,
        COUNT_Z,
    );
    let camera_well = centered_cube(
        format!("{OUTPUT_PREFIX}_counting_camera_well_cut"),
        COUNT_X - 92.0,
        72.0,
        22.0,
    )
    .translate(0.0, 12.0, COUNT_Z / 2.0 - 8.0);

    body - camera_well - counting_window_cuts()
        + counting_window_frames()
        + counting_electrode_pairs()
        + counting_capillary_lanes()
        + counting_camera_datums()
        + counting_reference_tick_strip()
}

fn counting_window_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_counting_window_cuts"));
    for i in 0..COUNT_WINDOWS {
        cuts = cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_counting_window_cut_{i}"),
                46.0,
                36.0,
                COUNT_Z + 3.0,
            )
            .translate(centered_index(i, COUNT_WINDOWS, 74.0), 10.0, 0.0);
    }
    cuts
}

fn counting_window_frames() -> Part {
    let mut frames = Part::empty(format!("{OUTPUT_PREFIX}_counting_window_frames"));
    for i in 0..COUNT_WINDOWS {
        let x = centered_index(i, COUNT_WINDOWS, 74.0);
        let frame = centered_cube(
            format!("{OUTPUT_PREFIX}_counting_window_frame_{i}"),
            62.0,
            52.0,
            9.0,
        )
        .translate(x, 10.0, COUNT_Z / 2.0 + 4.5);
        let opening = centered_cube(
            format!("{OUTPUT_PREFIX}_counting_window_opening_cut_{i}"),
            46.0,
            36.0,
            10.0,
        )
        .translate(x, 10.0, COUNT_Z / 2.0 + 5.0);
        frames = frames + (frame - opening);
    }
    frames
}

fn counting_electrode_pairs() -> Part {
    let mut electrodes = Part::empty(format!("{OUTPUT_PREFIX}_counting_electrode_pairs"));
    for i in 0..COUNT_ELECTRODE_PAIRS {
        electrodes = electrodes
            + centered_cube(
                format!("{OUTPUT_PREFIX}_counting_electrode_pair_land_{i}"),
                18.0,
                8.0,
                5.0,
            )
            .translate(
                centered_index(i % 4, 4, 74.0),
                if i < 4 { -42.0 } else { 60.0 },
                COUNT_Z / 2.0 + 2.5,
            );
    }
    electrodes
}

fn counting_capillary_lanes() -> Part {
    let mut lanes = Part::empty(format!("{OUTPUT_PREFIX}_counting_capillary_lanes"));
    for i in 0..COUNT_CAPILLARY_LANES {
        lanes = lanes
            + x_tube(
                format!("{OUTPUT_PREFIX}_counting_capillary_lane_{i}"),
                -COUNT_X / 2.0 + 48.0,
                COUNT_X / 2.0 - 48.0,
                centered_index(i, COUNT_CAPILLARY_LANES, 28.0) - 44.0,
                COUNT_Z / 2.0 + 20.0,
                ROUTE_TUBE_D / 2.0,
            );
    }
    lanes
}

fn counting_camera_datums() -> Part {
    let mut datums = Part::empty(format!("{OUTPUT_PREFIX}_counting_camera_datums"));
    for i in 0..COUNT_CAMERA_DATUMS {
        let x = if i % 2 == 0 { -1.0 } else { 1.0 } * (COUNT_X / 2.0 - 54.0);
        let y = if i < 2 { -1.0 } else { 1.0 } * (COUNT_Y / 2.0 - 44.0);
        datums = datums
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_counting_camera_datum_pin_{i}"),
                5.0,
                15.0,
                28,
            )
            .translate(x, y, COUNT_Z / 2.0 + 7.5);
    }
    datums
}

fn counting_reference_tick_strip() -> Part {
    let mut strip = Part::empty(format!("{OUTPUT_PREFIX}_counting_reference_tick_strip"));
    for i in 0..11 {
        strip = strip
            + centered_cube(
                format!("{OUTPUT_PREFIX}_counting_reference_tick_{i}"),
                if i % 5 == 0 { 5.0 } else { 3.0 },
                18.0,
                4.0,
            )
            .translate(
                -140.0 + i as f64 * 28.0,
                -COUNT_Y / 2.0 + 24.0,
                COUNT_Z / 2.0 + 2.0,
            );
    }
    strip
}

fn gentle_mixer_witness_coupon_rack() -> Part {
    let tray = centered_cube(
        format!("{OUTPUT_PREFIX}_gentle_mixer_coupon_tray"),
        MIXER_X,
        MIXER_Y,
        MIXER_Z,
    );
    let basin = centered_cube(
        format!("{OUTPUT_PREFIX}_gentle_mixer_coupon_basin_cut"),
        MIXER_X - 64.0,
        MIXER_Y - 58.0,
        12.0,
    )
    .translate(0.0, 0.0, MIXER_Z / 2.0 - 6.0);

    tray - basin - mixer_coupon_slot_cuts()
        + mixer_coupon_frames()
        + mixer_residence_time_token_lands()
        + mixer_sweep_arc_ticks()
        + mixer_baffle_witness_ribs()
        + mixer_closed_lid_latch_bar()
}

fn mixer_coupon_slot_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_mixer_coupon_slot_cuts"));
    for i in 0..MIXER_COUPONS {
        let row = i / MIXER_WITNESS_COLS;
        let col = i % MIXER_WITNESS_COLS;
        cuts = cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_mixer_witness_coupon_slot_cut_{i}"),
                MIXER_COUPON_X,
                MIXER_COUPON_Y,
                14.0,
            )
            .translate(
                centered_index(col, MIXER_WITNESS_COLS, 84.0),
                centered_index(row, MIXER_WITNESS_ROWS, 62.0),
                MIXER_Z / 2.0 - 5.0,
            );
    }
    cuts
}

fn mixer_coupon_frames() -> Part {
    let mut frames = Part::empty(format!("{OUTPUT_PREFIX}_mixer_coupon_frames"));
    for i in 0..MIXER_COUPONS {
        let row = i / MIXER_WITNESS_COLS;
        let col = i % MIXER_WITNESS_COLS;
        let x = centered_index(col, MIXER_WITNESS_COLS, 84.0);
        let y = centered_index(row, MIXER_WITNESS_ROWS, 62.0);
        let frame = centered_cube(
            format!("{OUTPUT_PREFIX}_mixer_witness_coupon_frame_{i}"),
            MIXER_COUPON_X + 14.0,
            MIXER_COUPON_Y + 14.0,
            7.0,
        )
        .translate(x, y, MIXER_Z / 2.0 + 3.5);
        let opening = centered_cube(
            format!("{OUTPUT_PREFIX}_mixer_witness_coupon_opening_cut_{i}"),
            MIXER_COUPON_X,
            MIXER_COUPON_Y,
            8.0,
        )
        .translate(x, y, MIXER_Z / 2.0 + 4.0);
        frames = frames + (frame - opening);
    }
    frames
}

fn mixer_residence_time_token_lands() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_mixer_residence_time_token_lands"));
    for (i, minutes) in MIXER_RESIDENCE_TOKENS.iter().enumerate() {
        lands = lands
            + centered_cube(
                format!("{OUTPUT_PREFIX}_mixer_residence_{minutes}_min_token_land"),
                34.0,
                12.0,
                5.0,
            )
            .translate(
                centered_index(i, MIXER_RESIDENCE_TOKENS.len(), 42.0),
                -MIXER_Y / 2.0 + 24.0,
                MIXER_Z / 2.0 + 2.5,
            );
    }
    lands
}

fn mixer_sweep_arc_ticks() -> Part {
    let mut ticks = Part::empty(format!("{OUTPUT_PREFIX}_mixer_sweep_arc_ticks"));
    for i in 0..MIXER_SWEEP_ARC_TICKS {
        let frac = i as f64 / (MIXER_SWEEP_ARC_TICKS - 1) as f64;
        let deg = -8.0 + frac * 16.0;
        let rad = deg.to_radians();
        ticks = ticks
            + centered_cube(
                format!("{OUTPUT_PREFIX}_gentle_mixer_sweep_arc_tick_{i}"),
                14.0,
                10.0,
                6.0,
            )
            .translate(
                rad.sin() * 174.0,
                MIXER_Y / 2.0 - 24.0,
                MIXER_Z / 2.0 + 42.0 + (1.0 - rad.cos()) * 174.0,
            );
    }
    ticks
}

fn mixer_baffle_witness_ribs() -> Part {
    let mut ribs = Part::empty(format!("{OUTPUT_PREFIX}_mixer_baffle_witness_ribs"));
    for i in 0..MIXER_BAFFLE_RIBS {
        ribs = ribs
            + centered_cube(
                format!("{OUTPUT_PREFIX}_gentle_mixer_baffle_witness_rib_{i}"),
                6.0,
                MIXER_Y - 94.0,
                12.0,
            )
            .translate(
                centered_index(i, MIXER_BAFFLE_RIBS, 43.0),
                0.0,
                MIXER_Z / 2.0 + 6.0,
            );
    }
    ribs
}

fn mixer_closed_lid_latch_bar() -> Part {
    centered_cube(
        format!("{OUTPUT_PREFIX}_mixer_closed_lid_latch_bar"),
        MIXER_X - 80.0,
        16.0,
        18.0,
    )
    .translate(0.0, MIXER_Y / 2.0 - 42.0, MIXER_Z / 2.0 + 9.0)
}

fn dead_volume_bubble_trap_bank() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_dead_volume_bubble_trap_body"),
        TRAP_X,
        TRAP_Y,
        TRAP_Z,
    );
    let service_recess = centered_cube(
        format!("{OUTPUT_PREFIX}_trap_bank_service_recess_cut"),
        TRAP_X - 58.0,
        TRAP_Y - 62.0,
        14.0,
    )
    .translate(0.0, 0.0, TRAP_Z / 2.0 - 7.0);

    body - service_recess - dead_volume_well_cuts()
        + bubble_trap_chambers()
        + dead_volume_well_labels()
        + bubble_escape_level_ticks()
        + trap_purge_ports()
        + trap_bypass_tube()
}

fn bubble_trap_chambers() -> Part {
    let mut chambers = Part::empty(format!("{OUTPUT_PREFIX}_bubble_trap_chambers"));
    for i in 0..BUBBLE_TRAPS {
        let x = centered_index(i, BUBBLE_TRAPS, 54.0);
        let chamber = centered_cylinder(
            format!("{OUTPUT_PREFIX}_bubble_trap_chamber_{i}"),
            18.0,
            34.0,
            36,
        )
        .translate(x, 44.0, TRAP_Z / 2.0 + 17.0);
        let sight = centered_cube(
            format!("{OUTPUT_PREFIX}_bubble_trap_sight_window_{i}"),
            24.0,
            9.0,
            28.0,
        )
        .translate(x, 22.0, TRAP_Z / 2.0 + 17.0);
        chambers = chambers + chamber + sight;
    }
    chambers
}

fn dead_volume_well_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_dead_volume_well_cuts"));
    for i in 0..DEAD_VOLUME_WELLS {
        cuts = cuts
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_dead_volume_well_cut_{i}"),
                13.0,
                20.0,
                32,
            )
            .translate(
                centered_index(i, DEAD_VOLUME_WELLS, 42.0),
                -48.0,
                TRAP_Z / 2.0 - 8.0,
            );
    }
    cuts
}

fn dead_volume_well_labels() -> Part {
    let mut labels = Part::empty(format!("{OUTPUT_PREFIX}_dead_volume_well_labels"));
    for i in 0..DEAD_VOLUME_WELLS {
        labels = labels
            + centered_cube(
                format!("{OUTPUT_PREFIX}_dead_volume_well_label_land_{i}"),
                32.0,
                10.0,
                4.0,
            )
            .translate(
                centered_index(i, DEAD_VOLUME_WELLS, 42.0),
                -82.0,
                TRAP_Z / 2.0 + 2.0,
            );
    }
    labels
}

fn bubble_escape_level_ticks() -> Part {
    let mut ticks = Part::empty(format!("{OUTPUT_PREFIX}_bubble_escape_level_ticks"));
    for trap in 0..BUBBLE_TRAPS {
        for tick in 0..TRAP_LEVEL_TICKS {
            ticks = ticks
                + centered_cube(
                    format!("{OUTPUT_PREFIX}_bubble_trap_{trap}_level_tick_{tick}"),
                    if tick % 4 == 0 { 14.0 } else { 8.0 },
                    3.0,
                    3.0,
                )
                .translate(
                    centered_index(trap, BUBBLE_TRAPS, 54.0) + 22.0,
                    21.0,
                    TRAP_Z / 2.0 + 4.0 + tick as f64 * 4.0,
                );
        }
    }
    ticks
}

fn trap_purge_ports() -> Part {
    let mut ports = Part::empty(format!("{OUTPUT_PREFIX}_trap_purge_ports"));
    for i in 0..TRAP_PURGE_PORTS {
        let port = centered_cylinder(
            format!("{OUTPUT_PREFIX}_trap_purge_port_boss_{i}"),
            8.0,
            12.0,
            28,
        )
        .translate(
            centered_index(i, TRAP_PURGE_PORTS, 54.0),
            TRAP_Y / 2.0 - 28.0,
            TRAP_Z / 2.0 + 6.0,
        );
        ports = ports + port;
    }
    ports
}

fn trap_bypass_tube() -> Part {
    x_tube(
        format!("{OUTPUT_PREFIX}_trap_bank_low_dead_volume_bypass_tube"),
        -TRAP_X / 2.0 + 40.0,
        TRAP_X / 2.0 - 40.0,
        -8.0,
        TRAP_Z / 2.0 + 18.0,
        ROUTE_TUBE_D / 2.0,
    )
}

fn barcode_custody_surface_panel() -> Part {
    let panel = centered_cube(
        format!("{OUTPUT_PREFIX}_barcode_custody_panel_body"),
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    panel
        + barcode_lands()
        + rfid_lands()
        + lot_card_slots()
        + custody_seal_lands()
        + custody_route_status_lane()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_barcode_custody_lands"));
    for i in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("{OUTPUT_PREFIX}_barcode_custody_land_{i}"),
                78.0,
                17.0,
                4.0,
            )
            .translate(
                centered_index(i % 6, 6, 82.0),
                if i < 6 { -26.0 } else { 24.0 },
                CUSTODY_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn rfid_lands() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_rfid_custody_lands"));
    for i in 0..RFID_LANDS {
        lands = lands
            + centered_cube(
                format!("{OUTPUT_PREFIX}_rfid_custody_land_{i}"),
                48.0,
                28.0,
                4.0,
            )
            .translate(
                -CUSTODY_X / 2.0 + 46.0,
                centered_index(i, RFID_LANDS, 28.0),
                CUSTODY_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn lot_card_slots() -> Part {
    let mut slots = Part::empty(format!("{OUTPUT_PREFIX}_lot_card_slots"));
    for i in 0..LOT_CARD_SLOTS {
        let slot = centered_cube(
            format!("{OUTPUT_PREFIX}_lot_card_slot_frame_{i}"),
            42.0,
            22.0,
            7.0,
        )
        .translate(
            CUSTODY_X / 2.0 - 86.0,
            centered_index(i, LOT_CARD_SLOTS, 18.0),
            CUSTODY_Z / 2.0 + 3.5,
        );
        let cut = centered_cube(
            format!("{OUTPUT_PREFIX}_lot_card_slot_opening_cut_{i}"),
            32.0,
            12.0,
            8.0,
        )
        .translate(
            CUSTODY_X / 2.0 - 86.0,
            centered_index(i, LOT_CARD_SLOTS, 18.0),
            CUSTODY_Z / 2.0 + 4.0,
        );
        slots = slots + (slot - cut);
    }
    slots
}

fn custody_seal_lands() -> Part {
    let mut seals = Part::empty(format!("{OUTPUT_PREFIX}_custody_seal_lands"));
    for i in 0..CUSTODY_SEAL_LANDS {
        seals = seals
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_custody_tamper_seal_land_{i}"),
                9.0,
                4.0,
                28,
            )
            .translate(
                centered_index(i, CUSTODY_SEAL_LANDS, 54.0),
                CUSTODY_Y / 2.0 - 18.0,
                CUSTODY_Z / 2.0 + 2.0,
            );
    }
    seals
}

fn custody_route_status_lane() -> Part {
    centered_cube(
        format!("{OUTPUT_PREFIX}_custody_route_status_lane"),
        CUSTODY_X - 118.0,
        8.0,
        6.0,
    )
    .translate(32.0, -CUSTODY_Y / 2.0 + 18.0, CUSTODY_Z / 2.0 + 3.0)
}

fn reject_hold_release_gate_array() -> Part {
    let base = centered_cube(
        format!("{OUTPUT_PREFIX}_gate_array_body"),
        GATE_X,
        GATE_Y,
        GATE_Z,
    );
    base + disposition_gate_lanes()
        + gate_solenoid_bodies()
        + gate_decision_input_lands()
        + gate_status_flag_towers()
        + gate_common_tube_headers()
}

fn disposition_gate_lanes() -> Part {
    let mut lanes = Part::empty(format!("{OUTPUT_PREFIX}_disposition_gate_lanes"));
    for (lane, name) in DISPOSITION_LANES.iter().enumerate() {
        let x = centered_index(lane, DISPOSITION_LANES.len(), GATE_LANE_PITCH_X);
        let lane_body = centered_cube(
            format!("{OUTPUT_PREFIX}_{name}_gate_lane_body"),
            122.0,
            72.0,
            16.0,
        )
        .translate(x, 2.0, GATE_Z / 2.0 + 8.0);
        let mut slots = Part::empty(format!("{OUTPUT_PREFIX}_{name}_gate_lane_slots"));
        for slot in 0..GATE_SLOTS_PER_LANE {
            slots = slots
                + centered_cube(
                    format!("{OUTPUT_PREFIX}_{name}_gate_slot_{slot}"),
                    20.0,
                    48.0,
                    17.0,
                )
                .translate(
                    x + centered_index(slot, GATE_SLOTS_PER_LANE, 26.0),
                    2.0,
                    GATE_Z / 2.0 + 8.5,
                );
        }
        lanes = lanes + (lane_body - slots);
    }
    lanes
}

fn gate_solenoid_bodies() -> Part {
    let mut solenoids = Part::empty(format!("{OUTPUT_PREFIX}_gate_solenoid_bodies"));
    for i in 0..GATE_SOLENOIDS {
        let lane = i / 2;
        let side = if i % 2 == 0 { -1.0 } else { 1.0 };
        solenoids = solenoids
            + centered_cube(
                format!("{OUTPUT_PREFIX}_gate_solenoid_body_{i}"),
                38.0,
                22.0,
                24.0,
            )
            .translate(
                centered_index(lane, DISPOSITION_LANES.len(), GATE_LANE_PITCH_X),
                side * 42.0,
                GATE_Z / 2.0 + 12.0,
            );
    }
    solenoids
}

fn gate_decision_input_lands() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_gate_decision_input_lands"));
    for i in 0..GATE_DECISION_INPUTS {
        lands = lands
            + centered_cube(
                format!("{OUTPUT_PREFIX}_gate_decision_input_land_{i}"),
                26.0,
                9.0,
                4.0,
            )
            .translate(
                centered_index(i % 6, 6, 34.0),
                -GATE_Y / 2.0 + 17.0 + (i / 6) as f64 * 15.0,
                GATE_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn gate_status_flag_towers() -> Part {
    let mut flags = Part::empty(format!("{OUTPUT_PREFIX}_gate_status_flag_towers"));
    for (lane, name) in DISPOSITION_LANES.iter().enumerate() {
        flags = flags
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{name}_status_flag_tower"),
                28.0,
                18.0,
                46.0,
            )
            .translate(
                centered_index(lane, DISPOSITION_LANES.len(), GATE_LANE_PITCH_X),
                GATE_Y / 2.0 - 23.0,
                GATE_Z / 2.0 + 23.0,
            );
    }
    flags
}

fn gate_common_tube_headers() -> Part {
    let inlet = x_tube(
        format!("{OUTPUT_PREFIX}_gate_common_inlet_header"),
        -GATE_X / 2.0 + 48.0,
        GATE_X / 2.0 - 48.0,
        -16.0,
        GATE_Z / 2.0 + 16.0,
        ROUTE_TUBE_D / 2.0,
    );
    let outlet = x_tube(
        format!("{OUTPUT_PREFIX}_gate_common_disposition_header"),
        -GATE_X / 2.0 + 48.0,
        GATE_X / 2.0 - 48.0,
        22.0,
        GATE_Z / 2.0 + 16.0,
        ROUTE_TUBE_D / 2.0,
    );
    inlet + outlet
}

fn robotic_service_datum_bridge() -> Part {
    service_datum_towers()
        + robot_docking_datums()
        + service_clearance_gauges()
        + robot_front_access_rail()
        + overhead_sensor_lift_gauge()
}

fn service_datum_towers() -> Part {
    let mut towers = Part::empty(format!("{OUTPUT_PREFIX}_service_datum_towers"));
    for i in 0..SERVICE_DATUM_TOWERS {
        let x = match i {
            0 => LOOP_CENTER.0,
            1 => OD_CENTER.0,
            2 => CASSETTE_CENTER.0,
            3 => MIXER_CENTER.0,
            4 => TRAP_CENTER.0,
            _ => COUNT_CENTER.0,
        };
        let y = match i {
            0 => LOOP_CENTER.1,
            1 => OD_CENTER.1,
            2 => CASSETTE_CENTER.1,
            3 => MIXER_CENTER.1,
            4 => TRAP_CENTER.1,
            _ => COUNT_CENTER.1,
        };
        towers = towers
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_robot_service_datum_tower_{i}"),
                10.0,
                28.0,
                32,
            )
            .translate(x, y + 96.0, SERVICE_BRIDGE_Z + 14.0);
    }
    towers
}

fn robot_docking_datums() -> Part {
    let mut datums = Part::empty(format!("{OUTPUT_PREFIX}_robot_docking_datums"));
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 154.0, -DECK_Y / 2.0 + 152.0),
        (DECK_X / 2.0 - 154.0, -DECK_Y / 2.0 + 152.0),
        (-DECK_X / 2.0 + 154.0, DECK_Y / 2.0 - 152.0),
        (DECK_X / 2.0 - 154.0, DECK_Y / 2.0 - 152.0),
    ]
    .iter()
    .enumerate()
    {
        datums = datums
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_robot_docking_spherical_datum_{i}"),
                12.0,
                12.0,
                32,
            )
            .translate(*x, *y, DECK_Z + 6.0);
    }
    datums
}

fn service_clearance_gauges() -> Part {
    let mut gauges = Part::empty(format!("{OUTPUT_PREFIX}_service_clearance_gauges"));
    for (i, (x, y, len, label_w)) in [
        (0.0, -DECK_Y / 2.0 + 54.0, FRONT_ROBOT_CLEARANCE, 90.0),
        (
            CASSETTE_CENTER.0,
            DECK_Y / 2.0 - 58.0,
            REAR_CASSETTE_CLEARANCE,
            76.0,
        ),
        (
            -DECK_X / 2.0 + 54.0,
            LOOP_CENTER.1,
            LEFT_LOOP_SERVICE_CLEARANCE,
            76.0,
        ),
        (
            DECK_X / 2.0 - 54.0,
            CASSETTE_CENTER.1,
            RIGHT_CASSETTE_SERVICE_CLEARANCE,
            76.0,
        ),
        (COUNT_CENTER.0, -DECK_Y / 2.0 + 54.0, 210.0, 70.0),
    ]
    .iter()
    .enumerate()
    {
        let gauge = centered_cube(
            format!("{OUTPUT_PREFIX}_service_clearance_gauge_{i}"),
            *label_w,
            10.0,
            6.0,
        )
        .translate(*x, *y, DECK_Z + 3.0);
        let tick = centered_cube(
            format!("{OUTPUT_PREFIX}_service_clearance_{len:.0}mm_tick_{i}"),
            10.0,
            28.0,
            6.0,
        )
        .translate(*x, *y, DECK_Z + 3.0);
        gauges = gauges + gauge + tick;
    }
    gauges
}

fn robot_front_access_rail() -> Part {
    centered_cube(
        format!("{OUTPUT_PREFIX}_robot_front_access_datum_rail"),
        DECK_X - 220.0,
        12.0,
        16.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 96.0, DECK_Z + 8.0)
}

fn overhead_sensor_lift_gauge() -> Part {
    let mast = centered_cube(
        format!("{OUTPUT_PREFIX}_overhead_sensor_lift_clearance_mast"),
        16.0,
        16.0,
        TOP_SENSOR_LIFT_CLEARANCE_Z,
    )
    .translate(
        COUNT_CENTER.0 + COUNT_X / 2.0 - 34.0,
        COUNT_CENTER.1 + COUNT_Y / 2.0 - 34.0,
        DECK_Z + TOP_SENSOR_LIFT_CLEARANCE_Z / 2.0,
    );
    let cap = centered_cube(
        format!("{OUTPUT_PREFIX}_overhead_sensor_lift_clearance_cap"),
        78.0,
        12.0,
        8.0,
    )
    .translate(
        COUNT_CENTER.0 + COUNT_X / 2.0 - 34.0,
        COUNT_CENTER.1 + COUNT_Y / 2.0 - 34.0,
        DECK_Z + TOP_SENSOR_LIFT_CLEARANCE_Z + 4.0,
    );
    mast + cap
}

fn closed_loop_route_harness() -> Part {
    let mut harness = Part::empty(format!("{OUTPUT_PREFIX}_closed_loop_route_harness"));
    for (i, (x1, y1, x2, y2)) in route_segments().iter().enumerate() {
        let part = if (x1 - x2).abs() >= (y1 - y2).abs() {
            x_tube(
                format!("{OUTPUT_PREFIX}_closed_loop_route_segment_{i}"),
                *x1,
                *x2,
                *y1,
                ROUTE_Z,
                ROUTE_TUBE_D / 2.0,
            )
        } else {
            y_tube(
                format!("{OUTPUT_PREFIX}_closed_loop_route_segment_{i}"),
                *x1,
                *y1,
                *y2,
                ROUTE_Z,
                ROUTE_TUBE_D / 2.0,
            )
        };
        harness = harness + part;
    }
    harness + route_elbows() + route_direction_markers()
}

fn route_elbows() -> Part {
    let mut elbows = Part::empty(format!("{OUTPUT_PREFIX}_closed_loop_route_elbows"));
    for (i, (x, y)) in [
        (-280.0, 190.0),
        (-250.0, 190.0),
        (105.0, 190.0),
        (150.0, 190.0),
        (445.0, 55.0),
        (-485.0, 10.0),
        (-250.0, -105.0),
        (105.0, -105.0),
        (170.0, -105.0),
        (390.0, -230.0),
    ]
    .iter()
    .enumerate()
    {
        elbows = elbows
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_closed_loop_elbow_boss_{i}"),
                10.0,
                10.0,
                28,
            )
            .translate(*x, *y, ROUTE_Z);
    }
    elbows
}

fn route_direction_markers() -> Part {
    let mut markers = Part::empty(format!("{OUTPUT_PREFIX}_route_direction_markers"));
    for (i, (x, y, dx, dy)) in [
        (-380.0, 190.0, 1.0, 0.0),
        (-115.0, 190.0, 1.0, 0.0),
        (275.0, 190.0, 1.0, 0.0),
        (445.0, 112.0, 0.0, -1.0),
        (-485.0, 72.0, 0.0, -1.0),
        (-330.0, -105.0, 1.0, 0.0),
        (20.0, -105.0, 1.0, 0.0),
        (270.0, -105.0, 1.0, 0.0),
        (390.0, -175.0, 0.0, -1.0),
        (120.0, -355.0, -1.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        markers = markers
            + direction_marker(
                format!("{OUTPUT_PREFIX}_route_direction_marker_{i}"),
                *dx,
                *dy,
            )
            .translate(*x, *y, ROUTE_Z + 6.0);
    }
    markers
}

fn route_segments() -> [(f64, f64, f64, f64); ROUTE_SEGMENTS] {
    [
        (
            LOOP_CENTER.0 + LOOP_X / 2.0 - 42.0,
            LOOP_CENTER.1,
            OD_CENTER.0 - OD_X / 2.0 + 44.0,
            LOOP_CENTER.1,
        ),
        (
            OD_CENTER.0 + OD_X / 2.0 - 38.0,
            OD_CENTER.1,
            CASSETTE_CENTER.0 - CASSETTE_X / 2.0 + 44.0,
            OD_CENTER.1,
        ),
        (
            CASSETTE_CENTER.0,
            CASSETTE_CENTER.1 - CASSETTE_Y / 2.0 + 42.0,
            CASSETTE_CENTER.0,
            COUNT_CENTER.1 + COUNT_Y / 2.0 - 42.0,
        ),
        (
            LOOP_CENTER.0,
            LOOP_CENTER.1 - LOOP_Y / 2.0 + 38.0,
            LOOP_CENTER.0,
            MIXER_CENTER.1 + MIXER_Y / 2.0 - 38.0,
        ),
        (
            MIXER_CENTER.0 + MIXER_X / 2.0 - 42.0,
            MIXER_CENTER.1,
            TRAP_CENTER.0 - TRAP_X / 2.0 + 42.0,
            MIXER_CENTER.1,
        ),
        (
            TRAP_CENTER.0 + TRAP_X / 2.0 - 42.0,
            TRAP_CENTER.1,
            COUNT_CENTER.0 - COUNT_X / 2.0 + 42.0,
            TRAP_CENTER.1,
        ),
        (
            COUNT_CENTER.0,
            COUNT_CENTER.1 - COUNT_Y / 2.0 + 40.0,
            COUNT_CENTER.0,
            GATE_CENTER.1 + GATE_Y / 2.0 - 24.0,
        ),
        (
            GATE_CENTER.0 + GATE_X / 2.0 - 52.0,
            GATE_CENTER.1,
            CASSETTE_CENTER.0 + CASSETTE_X / 2.0 - 52.0,
            GATE_CENTER.1,
        ),
        (
            CUSTODY_CENTER.0 + CUSTODY_X / 2.0 - 42.0,
            CUSTODY_CENTER.1,
            GATE_CENTER.0 - GATE_X / 2.0 + 42.0,
            CUSTODY_CENTER.1,
        ),
        (
            TRAP_CENTER.0,
            TRAP_CENTER.1 + TRAP_Y / 2.0 - 40.0,
            OD_CENTER.0,
            OD_CENTER.1 - OD_Y / 2.0 + 40.0,
        ),
        (
            COUNT_CENTER.0 - 120.0,
            COUNT_CENTER.1 + COUNT_Y / 2.0 - 42.0,
            OD_CENTER.0 + OD_X / 2.0 - 42.0,
            OD_CENTER.1 - OD_Y / 2.0 + 42.0,
        ),
        (
            CASSETTE_CENTER.0 - 130.0,
            CASSETTE_CENTER.1 - CASSETTE_Y / 2.0 + 46.0,
            COUNT_CENTER.0 + COUNT_X / 2.0 - 52.0,
            COUNT_CENTER.1 + COUNT_Y / 2.0 - 46.0,
        ),
    ]
}

fn module_specs() -> [ModuleSpec; 8] {
    [
        ModuleSpec {
            name: "closed_recirculating_sample_loop_surrogate",
            center: LOOP_CENTER,
            x: LOOP_X,
            y: LOOP_Y,
            z: LOOP_Z,
        },
        ModuleSpec {
            name: "inline_optical_density_window_bank",
            center: OD_CENTER,
            x: OD_X,
            y: OD_Y,
            z: OD_Z,
        },
        ModuleSpec {
            name: "multi_slot_tissue_chip_cassette_datum_nest",
            center: CASSETTE_CENTER,
            x: CASSETTE_X,
            y: CASSETTE_Y,
            z: CASSETTE_Z,
        },
        ModuleSpec {
            name: "gentle_mixer_witness_coupon_rack",
            center: MIXER_CENTER,
            x: MIXER_X,
            y: MIXER_Y,
            z: MIXER_Z,
        },
        ModuleSpec {
            name: "dead_volume_bubble_trap_bank",
            center: TRAP_CENTER,
            x: TRAP_X,
            y: TRAP_Y,
            z: TRAP_Z,
        },
        ModuleSpec {
            name: "cell_counting_window_bank",
            center: COUNT_CENTER,
            x: COUNT_X,
            y: COUNT_Y,
            z: COUNT_Z,
        },
        ModuleSpec {
            name: "barcode_custody_surface_panel",
            center: CUSTODY_CENTER,
            x: CUSTODY_X,
            y: CUSTODY_Y,
            z: CUSTODY_Z,
        },
        ModuleSpec {
            name: "reject_hold_release_gate_array",
            center: GATE_CENTER,
            x: GATE_X,
            y: GATE_Y,
            z: GATE_Z,
        },
    ]
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn x_tube(name: String, x1: f64, x2: f64, y: f64, z: f64, radius: f64) -> Part {
    centered_cylinder(name, radius, (x2 - x1).abs(), 24)
        .rotate(0.0, 90.0, 0.0)
        .translate((x1 + x2) / 2.0, y, z)
}

fn y_tube(name: String, x: f64, y1: f64, y2: f64, z: f64, radius: f64) -> Part {
    centered_cylinder(name, radius, (y2 - y1).abs(), 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(x, (y1 + y2) / 2.0, z)
}

fn datum_target(name: &str) -> Part {
    let boss = centered_cylinder(format!("{name}_boss"), 16.0, 7.0, 36);
    let center = centered_cylinder(format!("{name}_center_cut"), 4.0, 8.0, 24);
    let cross_x = centered_cube(format!("{name}_cross_x_cut"), 26.0, 3.0, 8.0);
    let cross_y = centered_cube(format!("{name}_cross_y_cut"), 3.0, 26.0, 8.0);

    boss - center - cross_x - cross_y
}

fn direction_marker(name: String, dx: f64, dy: f64) -> Part {
    let angle = dy.atan2(dx).to_degrees();
    let shaft = centered_cube(format!("{name}_shaft"), 28.0, 6.0, 5.0).translate(-4.0, 0.0, 0.0);
    let head = centered_cube(format!("{name}_head"), 13.0, 13.0, 5.0).translate(13.0, 0.0, 0.0);
    let notch = centered_cube(format!("{name}_head_notch_cut"), 7.0, 18.0, 6.0)
        .rotate(0.0, 0.0, 45.0)
        .translate(6.0, 0.0, 0.0);

    (shaft + (head - notch)).rotate(0.0, 0.0, angle)
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 12, "expected stable STL export count");
    assert_eq!(REQUIRED_FEATURES.len(), 15, "required feature list changed");
    assert_eq!(CASSETTE_SLOTS, CASSETTE_ROWS * CASSETTE_COLS);
    assert_eq!(CASSETTE_LANE_PORTS, CASSETTE_SLOTS);
    assert_eq!(MIXER_COUPONS, MIXER_WITNESS_ROWS * MIXER_WITNESS_COLS);
    assert_eq!(MIXER_COUPONS, MIXER_RESIDENCE_TOKENS.len());
    assert!(
        MIXER_RESIDENCE_TOKENS
            .windows(2)
            .all(|window| window[0] < window[1]),
        "mixer residence tokens must increase monotonically"
    );
    assert_eq!(
        GATE_SOLENOIDS,
        DISPOSITION_LANES.len() * 2,
        "each lane needs two physical gate solenoids"
    );
    assert_eq!(
        GATE_DECISION_INPUTS,
        DISPOSITION_LANES.len() * GATE_SLOTS_PER_LANE,
        "decision input lands must map every lane and gate slot"
    );
    assert_eq!(
        ROUTE_SEGMENTS,
        route_segments().len(),
        "closed-loop route segment count changed"
    );
    assert_eq!(ROUTE_ELBOWS, 10, "route elbow count changed");
    assert_eq!(
        ROUTE_DIRECTION_MARKERS, 10,
        "route direction marker count changed"
    );
    assert_eq!(
        MACHINE_DATUM_TARGETS, 4,
        "deck machine vision datum count changed"
    );
    assert_eq!(ROBOT_DOCKING_DATUMS, 4, "robot docking datum count changed");
    assert_eq!(
        SERVICE_CLEARANCE_GAUGES, 5,
        "service clearance gauge count changed"
    );
    assert!(
        FRONT_ROBOT_CLEARANCE >= 350.0
            && REAR_CASSETTE_CLEARANCE >= 240.0
            && LEFT_LOOP_SERVICE_CLEARANCE >= 250.0
            && RIGHT_CASSETTE_SERVICE_CLEARANCE >= 245.0,
        "robot/service clearances below station target"
    );

    let modules = module_specs();
    for module in modules {
        assert!(
            module.fits_on_deck(),
            "{} exceeds deck envelope",
            module.name
        );
        assert!(module.z > 0.0, "{} must have positive height", module.name);
    }
    for left in 0..modules.len() {
        for right in left + 1..modules.len() {
            assert!(
                !modules[left].overlaps(modules[right]),
                "{} overlaps {}",
                modules[left].name,
                modules[right].name
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

        assert_eq!(OUTPUTS.len(), 12);
        assert_eq!(unique.len(), OUTPUTS.len());
        assert!(OUTPUTS.iter().all(|path| path
            .starts_with("output/closed_seed_density_inline_mix_uniformity_validation_station_")));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
        assert!(OUTPUTS[0].ends_with("_leak_tray_deck.stl"));
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }

    #[test]
    fn required_features_cover_inline_mix_uniformity_station_intent() {
        for feature in [
            "closed_recirculating_sample_loop_surrogate",
            "inline_optical_density_window_bank",
            "cell_counting_window_bank",
            "gentle_mixer_witness_coupon_rack",
            "dead_volume_bubble_trap_bank",
            "barcode_custody_surface_panel",
            "reject_hold_release_gate_array",
            "robotic_service_datum_bridge",
            "cassette_load_handoff_manifold",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature), "{feature}");
        }
        assert_eq!(REQUIRED_FEATURES.len(), 15);
    }

    #[test]
    fn station_dimensions_and_modules_fit_without_overlap() {
        assert!(DECK_X <= 1540.0);
        assert!(DECK_Y <= 930.0);
        assert!(TRAY_RIM_Z >= 36.0);
        assert_eq!(module_specs().len(), 8);
        assert_design_constraints();
    }

    #[test]
    fn cassette_loop_and_measurement_counts_are_pinned() {
        assert_eq!(CASSETTE_ROWS, 3);
        assert_eq!(CASSETTE_COLS, 4);
        assert_eq!(CASSETTE_SLOTS, 12);
        assert_eq!(CASSETTE_LANE_PORTS, CASSETTE_SLOTS);
        assert_eq!(SAMPLE_LOOP_PORTS, 8);
        assert_eq!(SAMPLE_LOOP_PINCH_VALVES, 8);
        assert_eq!(SAMPLE_LOOP_PRESSURE_TAPS, 4);
        assert_eq!(OD_WINDOWS, 6);
        assert_eq!(COUNT_WINDOWS, 4);
        assert_eq!(COUNT_ELECTRODE_PAIRS, 8);
    }

    #[test]
    fn mixer_trap_and_bubble_evidence_counts_are_explicit() {
        assert_eq!(MIXER_COUPONS, MIXER_WITNESS_ROWS * MIXER_WITNESS_COLS);
        assert_eq!(MIXER_RESIDENCE_TOKENS[0], 0);
        assert_eq!(MIXER_RESIDENCE_TOKENS[MIXER_COUPONS - 1], 16);
        assert_eq!(MIXER_SWEEP_ARC_TICKS, 13);
        assert_eq!(BUBBLE_TRAPS, 5);
        assert_eq!(DEAD_VOLUME_WELLS, 6);
        assert_eq!(TRAP_LEVEL_TICKS, 9);
    }

    #[test]
    fn custody_gates_routes_and_service_datums_are_physical() {
        assert_eq!(DISPOSITION_LANES, ["reject", "hold", "release"]);
        assert_eq!(GATE_SLOTS_PER_LANE, 4);
        assert_eq!(GATE_SOLENOIDS, 6);
        assert_eq!(GATE_DECISION_INPUTS, 12);
        assert_eq!(BARCODE_LANDS, 12);
        assert_eq!(RFID_LANDS, 4);
        assert_eq!(ROUTE_SEGMENTS, route_segments().len());
        assert_eq!(ROUTE_ELBOWS, 10);
        assert_eq!(ROUTE_DIRECTION_MARKERS, 10);
        assert_eq!(SERVICE_DATUM_TOWERS, 6);
        assert_eq!(ROBOT_DOCKING_DATUMS, 4);
        assert!(TOP_SENSOR_LIFT_CLEARANCE_Z >= 300.0);
    }
}
