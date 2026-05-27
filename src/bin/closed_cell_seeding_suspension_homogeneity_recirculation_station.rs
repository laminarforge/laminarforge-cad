use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed-system cell-seeding suspension homogeneity and recirculation
// validation station.
//
// Intent:
// - Keep a cell suspension represented as a closed sterile loop from source
//   bag/vessel nest through gentle recirculation, bubble/settling traps,
//   inline density/viability sampling surrogate, and a closed connector
//   handoff to the downstream seeding/distribution manifold.
// - Make time-since-mix witnesses, custody/status surfaces, reference control
//   coupons, robot access, and service keepouts visible in deterministic CAD.
// - Model validation fixture interfaces only. This file does not encode
//   clinical acceptance thresholds, live-cell process limits, or release
//   criteria.

const OUTPUT_PREFIX: &str = "closed_cell_seeding_suspension_homogeneity_recirculation_station";

const OUTPUTS: [&str; 13] = [
    "output/closed_cell_seeding_suspension_homogeneity_recirculation_station_leak_tray_deck.stl",
    "output/closed_cell_seeding_suspension_homogeneity_recirculation_station_source_bag_vessel_nest.stl",
    "output/closed_cell_seeding_suspension_homogeneity_recirculation_station_gentle_recirculation_mixing_path.stl",
    "output/closed_cell_seeding_suspension_homogeneity_recirculation_station_low_shear_pump_bypass_guard.stl",
    "output/closed_cell_seeding_suspension_homogeneity_recirculation_station_inline_density_viability_sampling_surrogate.stl",
    "output/closed_cell_seeding_suspension_homogeneity_recirculation_station_time_since_mix_witness_lanes.stl",
    "output/closed_cell_seeding_suspension_homogeneity_recirculation_station_bubble_settling_trap_bank.stl",
    "output/closed_cell_seeding_suspension_homogeneity_recirculation_station_closed_connector_handoff_manifold.stl",
    "output/closed_cell_seeding_suspension_homogeneity_recirculation_station_barcode_custody_status_surfaces.stl",
    "output/closed_cell_seeding_suspension_homogeneity_recirculation_station_robot_service_keepouts.stl",
    "output/closed_cell_seeding_suspension_homogeneity_recirculation_station_closed_loop_route_tube_harness.stl",
    "output/closed_cell_seeding_suspension_homogeneity_recirculation_station_reference_control_coupon_rack.stl",
    "output/closed_cell_seeding_suspension_homogeneity_recirculation_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 14] = [
    "source_bag_vessel_nest",
    "gentle_recirculation_mixing_path",
    "low_shear_pump_bypass_guard",
    "inline_density_viability_sampling_surrogate",
    "time_since_mix_witness_lanes",
    "bubble_settling_trap_bank",
    "closed_connector_handoff_manifold",
    "barcode_custody_status_surfaces",
    "robot_service_keepouts",
    "closed_loop_route_tube_harness",
    "reference_control_coupon_rack",
    "closed_transfer_port_saddles",
    "mix_age_token_slots",
    "distribution_manifold_handoff_key",
];

const REPRODUCIBILITY_CONTROLS: [&str; 9] = [
    "time_since_mix_witness_lanes",
    "mix_age_token_slots",
    "inline_density_viability_sampling_surrogate",
    "bubble_settling_trap_bank",
    "barcode_custody_status_surfaces",
    "reference_control_coupon_rack",
    "closed_connector_handoff_manifold",
    "route_direction_markers",
    "robot_service_keepouts",
];

const DECK_X: f64 = 1580.0;
const DECK_Y: f64 = 960.0;
const DECK_Z: f64 = 22.0;
const TRAY_RIM_W: f64 = 20.0;
const TRAY_RIM_Z: f64 = 38.0;
const SUMP_DEPTH: f64 = 7.0;
const DRAIN_CHANNEL_W: f64 = 14.0;
const MOUNT_HOLE_D: f64 = 6.8;
const LEAK_WITNESS_WELLS: usize = 12;

const SOURCE_CENTER: (f64, f64) = (-515.0, 200.0);
const SOURCE_X: f64 = 430.0;
const SOURCE_Y: f64 = 250.0;
const SOURCE_Z: f64 = 48.0;
const SOURCE_BAG_POCKET_X: f64 = 250.0;
const SOURCE_BAG_POCKET_Y: f64 = 132.0;
const SOURCE_BAG_POCKET_DEPTH: f64 = 18.0;
const SOURCE_VESSEL_NESTS: usize = 4;
const SOURCE_VESSEL_WELL_D: f64 = 34.0;
const SOURCE_VESSEL_COLLAR_D: f64 = 54.0;
const SOURCE_PORT_SADDLES: usize = 6;
const SOURCE_LOCATOR_PINS: usize = 6;

const MIXER_CENTER: (f64, f64) = (-60.0, 205.0);
const MIXER_X: f64 = 390.0;
const MIXER_Y: f64 = 240.0;
const MIXER_Z: f64 = 42.0;
const MIXER_LOOP_WINDOW_X: f64 = 252.0;
const MIXER_LOOP_WINDOW_Y: f64 = 112.0;
const MIXER_TUBE_D: f64 = 7.2;
const MIXER_ROLLER_COUNT: usize = 3;
const MIXER_BAFFLE_RIBS: usize = 6;
const MIXER_SWEEP_ARC_TICKS: usize = 13;
const MIXER_SWEEP_LIMIT_DEG: f64 = 7.0;

const PUMP_CENTER: (f64, f64) = (-60.0, -105.0);
const PUMP_X: f64 = 350.0;
const PUMP_Y: f64 = 220.0;
const PUMP_Z: f64 = 54.0;
const PUMP_HEADS: usize = 2;
const BYPASS_VALVES: usize = 3;
const PRIMING_SIGHT_PORTS: usize = 4;
const PUMP_SHEAR_GUARD_RIBS: usize = 5;

const SAMPLING_CENTER: (f64, f64) = (410.0, 205.0);
const SAMPLING_X: f64 = 360.0;
const SAMPLING_Y: f64 = 240.0;
const SAMPLING_Z: f64 = 44.0;
const DENSITY_WINDOWS: usize = 5;
const VIABILITY_SURROGATE_SLOTS: usize = 4;
const SAMPLE_BRANCH_VALVES: usize = 6;
const SAMPLE_RETURN_CHANNELS: usize = 2;
const SAMPLE_CASSETTE_KEY_PINS: usize = 4;

const WITNESS_CENTER: (f64, f64) = (0.0, 405.0);
const WITNESS_X: f64 = 1220.0;
const WITNESS_Y: f64 = 70.0;
const WITNESS_Z: f64 = 18.0;
const MIX_AGE_TOKEN_COUNT: usize = 8;
const MIX_AGE_MINUTES: [usize; MIX_AGE_TOKEN_COUNT] = [0, 2, 5, 10, 15, 20, 30, 45];
const MIX_AGE_TOKEN_X: f64 = 82.0;
const MIX_AGE_TOKEN_Y: f64 = 30.0;
const WITNESS_LANE_COUNT: usize = 3;
const WITNESS_LANE_PITCH_Y: f64 = 18.0;

const BUBBLE_CENTER: (f64, f64) = (-515.0, -95.0);
const BUBBLE_X: f64 = 390.0;
const BUBBLE_Y: f64 = 230.0;
const BUBBLE_Z: f64 = 58.0;
const BUBBLE_TRAPS: usize = 4;
const SETTLING_WELLS: usize = 6;
const BUBBLE_LADDER_TICKS: usize = 11;
const TRAP_SIGHT_WINDOW_X: f64 = 52.0;
const TRAP_SIGHT_WINDOW_Y: f64 = 128.0;
const SETTLING_WELL_D: f64 = 28.0;

const HANDOFF_CENTER: (f64, f64) = (410.0, -105.0);
const HANDOFF_X: f64 = 360.0;
const HANDOFF_Y: f64 = 220.0;
const HANDOFF_Z: f64 = 50.0;
const DRY_BREAK_CONNECTORS: usize = 6;
const MANIFOLD_LANES: usize = 8;
const CONNECTOR_CAP_PARKS: usize = 6;
const HANDOFF_KEY_PINS: usize = 4;
const HANDOFF_LATCH_EARS: usize = 2;

const CUSTODY_CENTER: (f64, f64) = (240.0, -360.0);
const CUSTODY_X: f64 = 580.0;
const CUSTODY_Y: f64 = 150.0;
const CUSTODY_Z: f64 = 22.0;
const BARCODE_LANDS: usize = 10;
const RFID_LANDS: usize = 4;
const STATUS_LANES: usize = 4;
const LOT_CARD_SLOTS: usize = 6;

const CONTROL_CENTER: (f64, f64) = (-500.0, -360.0);
const CONTROL_X: f64 = 330.0;
const CONTROL_Y: f64 = 150.0;
const CONTROL_Z: f64 = 22.0;
const CONTROL_COUPONS: usize = 9;
const CONTROL_COUPON_D: f64 = 24.0;
const CONTROL_ROWS: usize = 3;
const CONTROL_COLS: usize = 3;

const ROUTE_TUBE_D: f64 = 7.0;
const ROUTE_Z: f64 = DECK_Z + 86.0;
const ROUTE_SEGMENTS: usize = 10;
const ROUTE_ELBOWS: usize = 9;
const ROUTE_DIRECTION_MARKERS: usize = 8;

const KEEP_OUT_X: f64 = 1500.0;
const KEEP_OUT_Y: f64 = 890.0;
const FRONT_ROBOT_CLEARANCE: f64 = 360.0;
const REAR_SOURCE_SERVICE_CLEARANCE: f64 = 250.0;
const LEFT_BAG_SERVICE_CLEARANCE: f64 = 260.0;
const RIGHT_MANIFOLD_SERVICE_CLEARANCE: f64 = 250.0;
const TOP_BAG_LIFT_CLEARANCE_Z: f64 = 330.0;
const SAMPLE_SENSOR_SERVICE_CLEARANCE_Z: f64 = 210.0;
const KEEP_OUT_ZONES: usize = 5;

#[derive(Clone, Copy, Debug)]
struct Footprint {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Footprint {
    fn fits_on_deck(self) -> bool {
        let usable_x = DECK_X / 2.0 - TRAY_RIM_W - 12.0;
        let usable_y = DECK_Y / 2.0 - TRAY_RIM_W - 12.0;

        self.center.0.abs() + self.x / 2.0 <= usable_x
            && self.center.1.abs() + self.y / 2.0 <= usable_y
    }

    fn overlaps(self, other: Footprint) -> bool {
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

    let source = source_bag_vessel_nest();
    write_part(OUTPUTS[1], &source);

    let mixer = gentle_recirculation_mixing_path();
    write_part(OUTPUTS[2], &mixer);

    let pump = low_shear_pump_bypass_guard();
    write_part(OUTPUTS[3], &pump);

    let sampling = inline_density_viability_sampling_surrogate();
    write_part(OUTPUTS[4], &sampling);

    let witnesses = time_since_mix_witness_lanes();
    write_part(OUTPUTS[5], &witnesses);

    let traps = bubble_settling_trap_bank();
    write_part(OUTPUTS[6], &traps);

    let handoff = closed_connector_handoff_manifold();
    write_part(OUTPUTS[7], &handoff);

    let custody = barcode_custody_status_surfaces();
    write_part(OUTPUTS[8], &custody);

    let keepouts = robot_service_keepouts();
    write_part(OUTPUTS[9], &keepouts);

    let routes = closed_loop_route_tube_harness();
    write_part(OUTPUTS[10], &routes);

    let controls = reference_control_coupon_rack();
    write_part(OUTPUTS[11], &controls);

    let assembly = station_assembly();
    write_part(OUTPUTS[12], &assembly);

    println!();
    println!("Closed cell seeding suspension homogeneity recirculation station:");
    println!(
        "  Closed source: {SOURCE_VESSEL_NESTS} vessel nests, one source-bag pocket, {SOURCE_PORT_SADDLES} closed-transfer port saddles"
    );
    println!(
        "  Recirculation: {MIXER_ROLLER_COUNT} gentle rollers, {MIXER_BAFFLE_RIBS} baffle ribs, {PUMP_HEADS} pump heads, {BYPASS_VALVES} bypass valves, {ROUTE_SEGMENTS} closed-loop route segments"
    );
    println!(
        "  Sampling evidence: {DENSITY_WINDOWS} density windows, {VIABILITY_SURROGATE_SLOTS} viability surrogate slots, {BUBBLE_TRAPS} bubble traps, {SETTLING_WELLS} settling wells"
    );
    println!(
        "  Reproducibility controls: {MIX_AGE_TOKEN_COUNT} mix-age tokens {MIX_AGE_MINUTES:?}, {CONTROL_COUPONS} reference coupons, {BARCODE_LANDS} barcode lands, {RFID_LANDS} RFID lands"
    );
    println!(
        "  Handoff: {DRY_BREAK_CONNECTORS} dry-break connectors into {MANIFOLD_LANES} distribution-manifold lanes with {CONNECTOR_CAP_PARKS} cap parks"
    );
}

fn write_part(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    leak_tray_deck()
        + source_bag_vessel_nest().translate(
            SOURCE_CENTER.0,
            SOURCE_CENTER.1,
            DECK_Z + SOURCE_Z / 2.0,
        )
        + gentle_recirculation_mixing_path().translate(
            MIXER_CENTER.0,
            MIXER_CENTER.1,
            DECK_Z + MIXER_Z / 2.0,
        )
        + low_shear_pump_bypass_guard().translate(
            PUMP_CENTER.0,
            PUMP_CENTER.1,
            DECK_Z + PUMP_Z / 2.0,
        )
        + inline_density_viability_sampling_surrogate().translate(
            SAMPLING_CENTER.0,
            SAMPLING_CENTER.1,
            DECK_Z + SAMPLING_Z / 2.0,
        )
        + time_since_mix_witness_lanes().translate(
            WITNESS_CENTER.0,
            WITNESS_CENTER.1,
            DECK_Z + WITNESS_Z / 2.0,
        )
        + bubble_settling_trap_bank().translate(
            BUBBLE_CENTER.0,
            BUBBLE_CENTER.1,
            DECK_Z + BUBBLE_Z / 2.0,
        )
        + closed_connector_handoff_manifold().translate(
            HANDOFF_CENTER.0,
            HANDOFF_CENTER.1,
            DECK_Z + HANDOFF_Z / 2.0,
        )
        + barcode_custody_status_surfaces().translate(
            CUSTODY_CENTER.0,
            CUSTODY_CENTER.1,
            DECK_Z + CUSTODY_Z / 2.0,
        )
        + reference_control_coupon_rack().translate(
            CONTROL_CENTER.0,
            CONTROL_CENTER.1,
            DECK_Z + CONTROL_Z / 2.0,
        )
        + closed_loop_route_tube_harness()
        + robot_service_keepouts()
}

fn leak_tray_deck() -> Part {
    let deck = centered_cube(
        format!("{OUTPUT_PREFIX}_deck_plate"),
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let sump = centered_cube(
        format!("{OUTPUT_PREFIX}_shallow_spill_sump_cut"),
        DECK_X - 140.0,
        DECK_Y - 150.0,
        SUMP_DEPTH + 2.0,
    )
    .translate(0.0, -8.0, DECK_Z - SUMP_DEPTH / 2.0 + 0.5);
    let front_drain = centered_cube(
        format!("{OUTPUT_PREFIX}_front_drain_channel_cut"),
        DECK_X - 180.0,
        DRAIN_CHANNEL_W,
        SUMP_DEPTH + 2.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 74.0, DECK_Z - SUMP_DEPTH / 2.0 + 0.5);

    deck - sump - front_drain - mount_holes()
        + tray_rims()
        + leak_witness_wells()
        + deck_floor_markers()
        + machine_vision_datum_targets()
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

fn mount_holes() -> Part {
    let mut holes = Part::empty(format!("{OUTPUT_PREFIX}_mount_holes"));
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 54.0, -DECK_Y / 2.0 + 54.0),
        (0.0, -DECK_Y / 2.0 + 54.0),
        (DECK_X / 2.0 - 54.0, -DECK_Y / 2.0 + 54.0),
        (-DECK_X / 2.0 + 54.0, DECK_Y / 2.0 - 54.0),
        (0.0, DECK_Y / 2.0 - 54.0),
        (DECK_X / 2.0 - 54.0, DECK_Y / 2.0 - 54.0),
        (-DECK_X / 2.0 + 54.0, 0.0),
        (DECK_X / 2.0 - 54.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_m6_mount_hole_{i}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 4.0,
                28,
            )
            .translate(*x, *y, DECK_Z / 2.0);
    }
    holes
}

fn leak_witness_wells() -> Part {
    let mut wells = Part::empty(format!("{OUTPUT_PREFIX}_closed_loop_leak_witness_wells"));
    for i in 0..LEAK_WITNESS_WELLS {
        let x = centered_index(i % 6, 6, 78.0);
        let y = -DECK_Y / 2.0 + 112.0 + (i / 6) as f64 * 36.0;
        wells = wells
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_leak_witness_well_{i}"),
                11.0,
                6.0,
                28,
            )
            .translate(x, y, DECK_Z + 3.0);
    }
    wells
}

fn deck_floor_markers() -> Part {
    let mut markers = Part::empty(format!("{OUTPUT_PREFIX}_module_floor_markers"));
    for footprint in primary_footprints() {
        markers = markers
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{}_floor_marker", footprint.name),
                footprint.x + 12.0,
                footprint.y + 12.0,
                3.0,
            )
            .translate(footprint.center.0, footprint.center.1, DECK_Z + 1.5);
    }
    markers
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
        let boss = centered_cylinder(
            format!("{OUTPUT_PREFIX}_datum_target_boss_{i}"),
            18.0,
            7.0,
            36,
        )
        .translate(*x, *y, DECK_Z + 3.5);
        let center_cut = centered_cylinder(
            format!("{OUTPUT_PREFIX}_datum_target_center_cut_{i}"),
            4.0,
            8.0,
            24,
        )
        .translate(*x, *y, DECK_Z + 4.0);
        targets = targets + (boss - center_cut);
    }
    targets
}

fn source_bag_vessel_nest() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_source_nest_body"),
        SOURCE_X,
        SOURCE_Y,
        SOURCE_Z,
    );
    let bag_pocket = centered_cube(
        format!("{OUTPUT_PREFIX}_source_bag_pocket_cut"),
        SOURCE_BAG_POCKET_X,
        SOURCE_BAG_POCKET_Y,
        SOURCE_BAG_POCKET_DEPTH + 1.0,
    )
    .translate(
        -62.0,
        14.0,
        SOURCE_Z / 2.0 - SOURCE_BAG_POCKET_DEPTH / 2.0 + 0.5,
    );

    body - bag_pocket - source_vessel_well_cuts()
        + source_bag_saddle_ribs()
        + source_vessel_collars()
        + source_locator_pins()
        + source_closed_transfer_port_saddles()
        + source_tube_strain_relief_comb()
}

fn source_vessel_well_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_source_vessel_well_cuts"));
    for i in 0..SOURCE_VESSEL_NESTS {
        let x = 114.0 + centered_index(i % 2, 2, 62.0);
        let y = centered_index(i / 2, 2, 70.0);
        cuts = cuts
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_source_vessel_well_cut_{i}"),
                SOURCE_VESSEL_WELL_D / 2.0,
                34.0,
                36,
            )
            .translate(x, y, SOURCE_Z / 2.0 - 16.0);
    }
    cuts
}

fn source_vessel_collars() -> Part {
    let mut collars = Part::empty(format!("{OUTPUT_PREFIX}_source_vessel_collar_bosses"));
    for i in 0..SOURCE_VESSEL_NESTS {
        let x = 114.0 + centered_index(i % 2, 2, 62.0);
        let y = centered_index(i / 2, 2, 70.0);
        let boss = centered_cylinder(
            format!("{OUTPUT_PREFIX}_source_vessel_collar_{i}"),
            SOURCE_VESSEL_COLLAR_D / 2.0,
            12.0,
            36,
        )
        .translate(x, y, SOURCE_Z / 2.0 + 6.0);
        let bore = centered_cylinder(
            format!("{OUTPUT_PREFIX}_source_vessel_collar_opening_cut_{i}"),
            SOURCE_VESSEL_WELL_D / 2.0,
            13.0,
            32,
        )
        .translate(x, y, SOURCE_Z / 2.0 + 6.5);
        collars = collars + (boss - bore);
    }
    collars
}

fn source_bag_saddle_ribs() -> Part {
    let mut ribs = Part::empty(format!("{OUTPUT_PREFIX}_source_bag_saddle_ribs"));
    for i in 0..6 {
        ribs = ribs
            + centered_cube(
                format!("{OUTPUT_PREFIX}_source_bag_gentle_saddle_rib_{i}"),
                SOURCE_BAG_POCKET_X - 26.0,
                5.0,
                9.0,
            )
            .translate(
                -62.0,
                14.0 + centered_index(i, 6, 20.0),
                SOURCE_Z / 2.0 + 4.5,
            );
    }
    ribs
}

fn source_locator_pins() -> Part {
    let mut pins = Part::empty(format!("{OUTPUT_PREFIX}_source_locator_pins"));
    for i in 0..SOURCE_LOCATOR_PINS {
        pins = pins
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_source_soft_locator_pin_{i}"),
                6.0,
                14.0,
                24,
            )
            .translate(
                -150.0 + (i % 3) as f64 * 76.0,
                if i < 3 { -92.0 } else { 94.0 },
                SOURCE_Z / 2.0 + 7.0,
            );
    }
    pins
}

fn source_closed_transfer_port_saddles() -> Part {
    let mut saddles = Part::empty(format!(
        "{OUTPUT_PREFIX}_source_closed_transfer_port_saddles"
    ));
    for i in 0..SOURCE_PORT_SADDLES {
        let saddle = centered_cube(
            format!("{OUTPUT_PREFIX}_source_closed_port_saddle_{i}"),
            36.0,
            18.0,
            12.0,
        )
        .translate(
            centered_index(i, SOURCE_PORT_SADDLES, 48.0) - 50.0,
            -SOURCE_Y / 2.0 + 24.0,
            SOURCE_Z / 2.0 + 6.0,
        );
        let tube_cut = centered_cylinder(
            format!("{OUTPUT_PREFIX}_source_closed_port_saddle_tube_cut_{i}"),
            5.0,
            20.0,
            18,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            centered_index(i, SOURCE_PORT_SADDLES, 48.0) - 50.0,
            -SOURCE_Y / 2.0 + 24.0,
            SOURCE_Z / 2.0 + 8.0,
        );
        saddles = saddles + (saddle - tube_cut);
    }
    saddles
}

fn source_tube_strain_relief_comb() -> Part {
    let mut comb = Part::empty(format!("{OUTPUT_PREFIX}_source_tube_strain_relief_comb"));
    for i in 0..SOURCE_PORT_SADDLES {
        comb = comb
            + centered_cube(
                format!("{OUTPUT_PREFIX}_source_tube_strain_relief_finger_{i}"),
                8.0,
                42.0,
                16.0,
            )
            .translate(
                centered_index(i, SOURCE_PORT_SADDLES, 46.0) - 50.0,
                -SOURCE_Y / 2.0 - 6.0,
                SOURCE_Z / 2.0 + 8.0,
            );
    }
    comb
}

fn gentle_recirculation_mixing_path() -> Part {
    let base = centered_cube(
        format!("{OUTPUT_PREFIX}_gentle_mixing_loop_plate"),
        MIXER_X,
        MIXER_Y,
        MIXER_Z,
    );
    let witness_window = centered_cube(
        format!("{OUTPUT_PREFIX}_recirculation_loop_witness_window_cut"),
        MIXER_LOOP_WINDOW_X,
        MIXER_LOOP_WINDOW_Y,
        MIXER_Z + 3.0,
    )
    .translate(0.0, 0.0, 3.0);

    base - witness_window
        + recirculation_loop_tube_window()
        + mixer_rollers()
        + mixer_baffle_ribs()
        + mixer_sweep_arc_ticks()
        + mixer_direction_markers()
}

fn recirculation_loop_tube_window() -> Part {
    let left = centered_cylinder(
        format!("{OUTPUT_PREFIX}_recirculation_loop_left_sight_tube"),
        MIXER_TUBE_D / 2.0,
        MIXER_LOOP_WINDOW_Y + 24.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-MIXER_LOOP_WINDOW_X / 2.0, 0.0, MIXER_Z / 2.0 + 12.0);
    let right = centered_cylinder(
        format!("{OUTPUT_PREFIX}_recirculation_loop_right_sight_tube"),
        MIXER_TUBE_D / 2.0,
        MIXER_LOOP_WINDOW_Y + 24.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(MIXER_LOOP_WINDOW_X / 2.0, 0.0, MIXER_Z / 2.0 + 12.0);
    let top = centered_cylinder(
        format!("{OUTPUT_PREFIX}_recirculation_loop_top_span"),
        MIXER_TUBE_D / 2.0,
        MIXER_LOOP_WINDOW_X,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, MIXER_LOOP_WINDOW_Y / 2.0, MIXER_Z / 2.0 + 12.0);
    let bottom = centered_cylinder(
        format!("{OUTPUT_PREFIX}_recirculation_loop_bottom_span"),
        MIXER_TUBE_D / 2.0,
        MIXER_LOOP_WINDOW_X,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -MIXER_LOOP_WINDOW_Y / 2.0, MIXER_Z / 2.0 + 12.0);

    left + right + top + bottom
}

fn mixer_rollers() -> Part {
    let mut rollers = Part::empty(format!("{OUTPUT_PREFIX}_gentle_recirculation_rollers"));
    for i in 0..MIXER_ROLLER_COUNT {
        rollers = rollers
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_gentle_mixer_roller_{i}"),
                12.0,
                MIXER_LOOP_WINDOW_X - 40.0,
                32,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(
                0.0,
                centered_index(i, MIXER_ROLLER_COUNT, 42.0),
                MIXER_Z / 2.0 + 26.0,
            );
    }
    rollers
}

fn mixer_baffle_ribs() -> Part {
    let mut ribs = Part::empty(format!("{OUTPUT_PREFIX}_gentle_mixer_baffle_ribs"));
    for i in 0..MIXER_BAFFLE_RIBS {
        ribs = ribs
            + centered_cube(
                format!("{OUTPUT_PREFIX}_gentle_mixer_baffle_rib_{i}"),
                8.0,
                MIXER_LOOP_WINDOW_Y + 18.0,
                13.0,
            )
            .translate(
                centered_index(i, MIXER_BAFFLE_RIBS, 42.0),
                0.0,
                MIXER_Z / 2.0 + 6.5,
            );
    }
    ribs
}

fn mixer_sweep_arc_ticks() -> Part {
    let mut ticks = Part::empty(format!("{OUTPUT_PREFIX}_mixer_sweep_arc_ticks"));
    for i in 0..MIXER_SWEEP_ARC_TICKS {
        let frac = i as f64 / (MIXER_SWEEP_ARC_TICKS - 1) as f64;
        let deg = -MIXER_SWEEP_LIMIT_DEG + frac * MIXER_SWEEP_LIMIT_DEG * 2.0;
        let rad = deg.to_radians();
        let x = rad.sin() * 190.0;
        let z = MIXER_Z / 2.0 + 48.0 + (1.0 - rad.cos()) * 190.0;
        ticks = ticks
            + centered_cube(
                format!("{OUTPUT_PREFIX}_gentle_mixer_sweep_arc_tick_{i}"),
                16.0,
                12.0,
                7.0,
            )
            .translate(x, MIXER_Y / 2.0 - 22.0, z);
    }
    ticks
}

fn mixer_direction_markers() -> Part {
    let inlet = flow_arrow_marker(
        format!("{OUTPUT_PREFIX}_mixer_inlet_direction_marker"),
        1.0,
        0.0,
    )
    .translate(
        -MIXER_X / 2.0 + 70.0,
        -MIXER_Y / 2.0 + 32.0,
        MIXER_Z / 2.0 + 8.0,
    );
    let return_marker = flow_arrow_marker(
        format!("{OUTPUT_PREFIX}_mixer_return_direction_marker"),
        -1.0,
        0.0,
    )
    .translate(
        MIXER_X / 2.0 - 70.0,
        MIXER_Y / 2.0 - 32.0,
        MIXER_Z / 2.0 + 8.0,
    );

    inlet + return_marker
}

fn low_shear_pump_bypass_guard() -> Part {
    let base = centered_cube(
        format!("{OUTPUT_PREFIX}_pump_bypass_guard_base"),
        PUMP_X,
        PUMP_Y,
        PUMP_Z,
    );
    let access_relief = centered_cube(
        format!("{OUTPUT_PREFIX}_pump_head_access_window_cut"),
        PUMP_X - 86.0,
        PUMP_Y - 86.0,
        PUMP_Z + 2.0,
    )
    .translate(0.0, -8.0, 4.0);

    base - access_relief
        + pump_head_envelopes()
        + bypass_valve_ladder()
        + priming_sight_ports()
        + pump_shear_guard_ribs()
}

fn pump_head_envelopes() -> Part {
    let mut heads = Part::empty(format!("{OUTPUT_PREFIX}_low_shear_pump_heads"));
    for i in 0..PUMP_HEADS {
        let y = centered_index(i, PUMP_HEADS, 72.0);
        let head = centered_cylinder(
            format!("{OUTPUT_PREFIX}_low_shear_pump_head_{i}"),
            34.0,
            22.0,
            40,
        )
        .translate(-74.0, y, PUMP_Z / 2.0 + 11.0);
        let guard = centered_cylinder(
            format!("{OUTPUT_PREFIX}_low_shear_pump_head_guard_ring_{i}"),
            43.0,
            8.0,
            40,
        )
        .translate(-74.0, y, PUMP_Z / 2.0 + 28.0);
        let guard_cut = centered_cylinder(
            format!("{OUTPUT_PREFIX}_low_shear_pump_head_guard_opening_cut_{i}"),
            34.0,
            9.0,
            36,
        )
        .translate(-74.0, y, PUMP_Z / 2.0 + 28.5);
        heads = heads + head + (guard - guard_cut);
    }
    heads
}

fn bypass_valve_ladder() -> Part {
    let mut valves = Part::empty(format!("{OUTPUT_PREFIX}_closed_loop_bypass_valve_ladder"));
    for i in 0..BYPASS_VALVES {
        let y = centered_index(i, BYPASS_VALVES, 48.0);
        let body = centered_cube(
            format!("{OUTPUT_PREFIX}_bypass_valve_body_{i}"),
            58.0,
            22.0,
            22.0,
        )
        .translate(82.0, y, PUMP_Z / 2.0 + 11.0);
        let knob = centered_cylinder(
            format!("{OUTPUT_PREFIX}_bypass_valve_position_witness_knob_{i}"),
            9.0,
            9.0,
            28,
        )
        .translate(82.0, y, PUMP_Z / 2.0 + 26.5);
        valves = valves + body + knob;
    }
    let bypass_bridge = centered_cylinder(
        format!("{OUTPUT_PREFIX}_bypass_return_bridge_tube"),
        ROUTE_TUBE_D / 2.0,
        156.0,
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(126.0, 0.0, PUMP_Z / 2.0 + 20.0);

    valves + bypass_bridge
}

fn priming_sight_ports() -> Part {
    let mut ports = Part::empty(format!("{OUTPUT_PREFIX}_priming_sight_ports"));
    for i in 0..PRIMING_SIGHT_PORTS {
        ports = ports
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_priming_sight_port_{i}"),
                10.0,
                8.0,
                28,
            )
            .translate(
                centered_index(i, PRIMING_SIGHT_PORTS, 42.0),
                -PUMP_Y / 2.0 + 34.0,
                PUMP_Z / 2.0 + 4.0,
            );
    }
    ports
}

fn pump_shear_guard_ribs() -> Part {
    let mut ribs = Part::empty(format!("{OUTPUT_PREFIX}_pump_shear_guard_ribs"));
    for i in 0..PUMP_SHEAR_GUARD_RIBS {
        ribs = ribs
            + centered_cube(
                format!("{OUTPUT_PREFIX}_pump_shear_guard_rib_{i}"),
                PUMP_X - 80.0,
                5.0,
                12.0,
            )
            .translate(
                0.0,
                centered_index(i, PUMP_SHEAR_GUARD_RIBS, 32.0),
                PUMP_Z / 2.0 + 6.0,
            );
    }
    ribs
}

fn inline_density_viability_sampling_surrogate() -> Part {
    let base = centered_cube(
        format!("{OUTPUT_PREFIX}_inline_sampling_surrogate_base"),
        SAMPLING_X,
        SAMPLING_Y,
        SAMPLING_Z,
    );
    base - density_window_cuts() - viability_slot_cuts()
        + density_window_frames()
        + sample_branch_valves()
        + sample_return_channels()
        + sample_cassette_key_pins()
}

fn density_window_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_density_window_cuts"));
    for i in 0..DENSITY_WINDOWS {
        cuts = cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_density_optical_window_{i}_cut"),
                42.0,
                24.0,
                SAMPLING_Z + 2.0,
            )
            .translate(centered_index(i, DENSITY_WINDOWS, 56.0), 48.0, 2.0);
    }
    cuts
}

fn density_window_frames() -> Part {
    let mut frames = Part::empty(format!("{OUTPUT_PREFIX}_density_window_frames"));
    for i in 0..DENSITY_WINDOWS {
        let frame = centered_cube(
            format!("{OUTPUT_PREFIX}_density_optical_window_frame_{i}"),
            56.0,
            36.0,
            8.0,
        )
        .translate(
            centered_index(i, DENSITY_WINDOWS, 56.0),
            48.0,
            SAMPLING_Z / 2.0 + 4.0,
        );
        let clear = centered_cube(
            format!("{OUTPUT_PREFIX}_density_optical_window_frame_clear_cut_{i}"),
            42.0,
            24.0,
            9.0,
        )
        .translate(
            centered_index(i, DENSITY_WINDOWS, 56.0),
            48.0,
            SAMPLING_Z / 2.0 + 4.5,
        );
        frames = frames + (frame - clear);
    }
    frames
}

fn viability_slot_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_viability_surrogate_slot_cuts"));
    for i in 0..VIABILITY_SURROGATE_SLOTS {
        cuts = cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_viability_surrogate_coupon_slot_{i}_cut"),
                72.0,
                22.0,
                12.0,
            )
            .translate(
                centered_index(i, VIABILITY_SURROGATE_SLOTS, 78.0),
                -42.0,
                SAMPLING_Z / 2.0 - 5.0,
            );
    }
    cuts
}

fn sample_branch_valves() -> Part {
    let mut valves = Part::empty(format!("{OUTPUT_PREFIX}_sample_branch_valves"));
    for i in 0..SAMPLE_BRANCH_VALVES {
        valves = valves
            + centered_cube(
                format!("{OUTPUT_PREFIX}_sample_branch_valve_{i}"),
                34.0,
                16.0,
                18.0,
            )
            .translate(
                centered_index(i, SAMPLE_BRANCH_VALVES, 48.0),
                -SAMPLING_Y / 2.0 + 24.0,
                SAMPLING_Z / 2.0 + 9.0,
            );
    }
    valves
}

fn sample_return_channels() -> Part {
    let mut channels = Part::empty(format!("{OUTPUT_PREFIX}_sample_return_channels"));
    for i in 0..SAMPLE_RETURN_CHANNELS {
        channels = channels
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_sample_return_channel_{i}"),
                ROUTE_TUBE_D / 2.0,
                SAMPLING_X - 52.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, -12.0 + i as f64 * 24.0, SAMPLING_Z / 2.0 + 16.0);
    }
    channels
}

fn sample_cassette_key_pins() -> Part {
    let mut pins = Part::empty(format!("{OUTPUT_PREFIX}_sample_cassette_key_pins"));
    for i in 0..SAMPLE_CASSETTE_KEY_PINS {
        let sx = if i % 2 == 0 { -1.0 } else { 1.0 };
        let sy = if i < 2 { -1.0 } else { 1.0 };
        pins = pins
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_sampling_cassette_key_pin_{i}"),
                5.0,
                12.0,
                24,
            )
            .translate(
                sx * (SAMPLING_X / 2.0 - 34.0),
                sy * (SAMPLING_Y / 2.0 - 34.0),
                SAMPLING_Z / 2.0 + 6.0,
            );
    }
    pins
}

fn time_since_mix_witness_lanes() -> Part {
    let base = centered_cube(
        format!("{OUTPUT_PREFIX}_time_since_mix_witness_lane_base"),
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    );

    base - mix_age_token_slot_cuts() + witness_lane_rails() + mix_age_label_lands()
}

fn mix_age_token_slot_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_mix_age_token_slot_cuts"));
    for lane in 0..WITNESS_LANE_COUNT {
        let y = centered_index(lane, WITNESS_LANE_COUNT, WITNESS_LANE_PITCH_Y);
        for i in 0..MIX_AGE_TOKEN_COUNT {
            cuts = cuts
                + centered_cube(
                    format!(
                        "{OUTPUT_PREFIX}_mix_age_lane_{lane}_{}min_token_slot_cut",
                        MIX_AGE_MINUTES[i]
                    ),
                    MIX_AGE_TOKEN_X,
                    MIX_AGE_TOKEN_Y / 2.0,
                    8.0,
                )
                .translate(
                    centered_index(i, MIX_AGE_TOKEN_COUNT, 142.0),
                    y,
                    WITNESS_Z / 2.0 - 3.5,
                );
        }
    }
    cuts
}

fn witness_lane_rails() -> Part {
    let mut rails = Part::empty(format!("{OUTPUT_PREFIX}_time_since_mix_lane_rails"));
    for lane in 0..WITNESS_LANE_COUNT {
        rails = rails
            + centered_cube(
                format!("{OUTPUT_PREFIX}_time_since_mix_lane_{lane}_front_rail"),
                WITNESS_X - 70.0,
                4.0,
                8.0,
            )
            .translate(
                0.0,
                centered_index(lane, WITNESS_LANE_COUNT, WITNESS_LANE_PITCH_Y) - 9.0,
                WITNESS_Z / 2.0 + 4.0,
            )
            + centered_cube(
                format!("{OUTPUT_PREFIX}_time_since_mix_lane_{lane}_rear_rail"),
                WITNESS_X - 70.0,
                4.0,
                8.0,
            )
            .translate(
                0.0,
                centered_index(lane, WITNESS_LANE_COUNT, WITNESS_LANE_PITCH_Y) + 9.0,
                WITNESS_Z / 2.0 + 4.0,
            );
    }
    rails
}

fn mix_age_label_lands() -> Part {
    let mut labels = Part::empty(format!("{OUTPUT_PREFIX}_mix_age_label_lands"));
    for i in 0..MIX_AGE_TOKEN_COUNT {
        labels = labels
            + centered_cube(
                format!(
                    "{OUTPUT_PREFIX}_mix_age_{}min_label_land",
                    MIX_AGE_MINUTES[i]
                ),
                50.0,
                9.0,
                4.0,
            )
            .translate(
                centered_index(i, MIX_AGE_TOKEN_COUNT, 142.0),
                -WITNESS_Y / 2.0 + 8.0,
                WITNESS_Z / 2.0 + 2.0,
            );
    }
    labels
}

fn bubble_settling_trap_bank() -> Part {
    let base = centered_cube(
        format!("{OUTPUT_PREFIX}_bubble_settling_trap_bank_base"),
        BUBBLE_X,
        BUBBLE_Y,
        BUBBLE_Z,
    );

    base - trap_sight_window_cuts() - settling_well_cuts()
        + trap_sight_tubes()
        + bubble_ladder_ticks()
        + settling_trap_overflow_channels()
}

fn trap_sight_window_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_trap_sight_window_cuts"));
    for i in 0..BUBBLE_TRAPS {
        cuts = cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_bubble_trap_sight_window_{i}_cut"),
                TRAP_SIGHT_WINDOW_X,
                TRAP_SIGHT_WINDOW_Y,
                BUBBLE_Z + 2.0,
            )
            .translate(centered_index(i, BUBBLE_TRAPS, 78.0), 18.0, 2.0);
    }
    cuts
}

fn trap_sight_tubes() -> Part {
    let mut tubes = Part::empty(format!("{OUTPUT_PREFIX}_bubble_trap_sight_tubes"));
    for i in 0..BUBBLE_TRAPS {
        tubes = tubes
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_bubble_trap_vertical_sight_tube_{i}"),
                ROUTE_TUBE_D / 2.0,
                TRAP_SIGHT_WINDOW_Y + 18.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(i, BUBBLE_TRAPS, 78.0),
                18.0,
                BUBBLE_Z / 2.0 + 10.0,
            );
    }
    tubes
}

fn settling_well_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_settling_well_cuts"));
    for i in 0..SETTLING_WELLS {
        cuts = cuts
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_settling_well_{i}_cut"),
                SETTLING_WELL_D / 2.0,
                22.0,
                32,
            )
            .translate(
                centered_index(i % 3, 3, 72.0) + 32.0,
                -BUBBLE_Y / 2.0 + 44.0 + (i / 3) as f64 * 42.0,
                BUBBLE_Z / 2.0 - 8.0,
            );
    }
    cuts
}

fn bubble_ladder_ticks() -> Part {
    let mut ticks = Part::empty(format!("{OUTPUT_PREFIX}_bubble_settling_ladder_ticks"));
    for i in 0..BUBBLE_LADDER_TICKS {
        ticks = ticks
            + centered_cube(
                format!("{OUTPUT_PREFIX}_bubble_trap_ladder_tick_{i}"),
                if i % 5 == 0 { 32.0 } else { 20.0 },
                4.0,
                5.0,
            )
            .translate(
                -BUBBLE_X / 2.0 + 34.0,
                centered_index(i, BUBBLE_LADDER_TICKS, 12.0),
                BUBBLE_Z / 2.0 + 2.5,
            );
    }
    ticks
}

fn settling_trap_overflow_channels() -> Part {
    let upper = centered_cylinder(
        format!("{OUTPUT_PREFIX}_bubble_trap_upper_overflow_channel"),
        ROUTE_TUBE_D / 2.0,
        BUBBLE_X - 80.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 80.0, BUBBLE_Z / 2.0 + 18.0);
    let lower = centered_cylinder(
        format!("{OUTPUT_PREFIX}_settling_trap_lower_return_channel"),
        ROUTE_TUBE_D / 2.0,
        BUBBLE_X - 80.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -80.0, BUBBLE_Z / 2.0 + 12.0);

    upper + lower
}

fn closed_connector_handoff_manifold() -> Part {
    let base = centered_cube(
        format!("{OUTPUT_PREFIX}_closed_connector_handoff_manifold_base"),
        HANDOFF_X,
        HANDOFF_Y,
        HANDOFF_Z,
    );

    base - manifold_lane_cuts()
        + dry_break_connector_bosses()
        + connector_cap_parks()
        + handoff_key_pins()
        + handoff_latch_ears()
}

fn dry_break_connector_bosses() -> Part {
    let mut bosses = Part::empty(format!("{OUTPUT_PREFIX}_dry_break_connector_bosses"));
    for i in 0..DRY_BREAK_CONNECTORS {
        let x = centered_index(i, DRY_BREAK_CONNECTORS, 46.0);
        let boss = centered_cylinder(
            format!("{OUTPUT_PREFIX}_dry_break_connector_boss_{i}"),
            16.0,
            20.0,
            36,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -HANDOFF_Y / 2.0 - 2.0, HANDOFF_Z / 2.0 + 4.0);
        let bore = centered_cylinder(
            format!("{OUTPUT_PREFIX}_dry_break_connector_bore_cut_{i}"),
            6.0,
            22.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -HANDOFF_Y / 2.0 - 2.0, HANDOFF_Z / 2.0 + 4.0);
        bosses = bosses + (boss - bore);
    }
    bosses
}

fn manifold_lane_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_distribution_manifold_lane_cuts"));
    for i in 0..MANIFOLD_LANES {
        cuts = cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_distribution_manifold_lane_{i}_relief_cut"),
                HANDOFF_X - 72.0,
                9.0,
                10.0,
            )
            .translate(
                0.0,
                centered_index(i, MANIFOLD_LANES, 20.0) + 14.0,
                HANDOFF_Z / 2.0 - 4.0,
            );
    }
    cuts
}

fn connector_cap_parks() -> Part {
    let mut parks = Part::empty(format!("{OUTPUT_PREFIX}_connector_cap_parks"));
    for i in 0..CONNECTOR_CAP_PARKS {
        parks = parks
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_sterile_connector_cap_park_{i}"),
                11.0,
                9.0,
                28,
            )
            .translate(
                centered_index(i, CONNECTOR_CAP_PARKS, 44.0),
                HANDOFF_Y / 2.0 - 26.0,
                HANDOFF_Z / 2.0 + 4.5,
            );
    }
    parks
}

fn handoff_key_pins() -> Part {
    let mut pins = Part::empty(format!(
        "{OUTPUT_PREFIX}_distribution_manifold_handoff_key_pins"
    ));
    for i in 0..HANDOFF_KEY_PINS {
        let sx = if i % 2 == 0 { -1.0 } else { 1.0 };
        let sy = if i < 2 { -1.0 } else { 1.0 };
        pins = pins
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_distribution_manifold_handoff_key_pin_{i}"),
                5.0,
                14.0,
                24,
            )
            .translate(
                sx * (HANDOFF_X / 2.0 - 34.0),
                sy * (HANDOFF_Y / 2.0 - 34.0),
                HANDOFF_Z / 2.0 + 7.0,
            );
    }
    pins
}

fn handoff_latch_ears() -> Part {
    let mut ears = Part::empty(format!("{OUTPUT_PREFIX}_handoff_latch_ears"));
    for i in 0..HANDOFF_LATCH_EARS {
        let x = if i == 0 {
            -HANDOFF_X / 2.0 - 13.0
        } else {
            HANDOFF_X / 2.0 + 13.0
        };
        ears = ears
            + centered_cube(
                format!("{OUTPUT_PREFIX}_handoff_latch_ear_{i}"),
                26.0,
                82.0,
                28.0,
            )
            .translate(x, 0.0, HANDOFF_Z / 2.0);
    }
    ears
}

fn barcode_custody_status_surfaces() -> Part {
    let base = centered_cube(
        format!("{OUTPUT_PREFIX}_barcode_custody_status_panel"),
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );

    base + barcode_lands() + rfid_lands() + status_lane_surfaces() + lot_card_slots()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_barcode_lands"));
    for i in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(format!("{OUTPUT_PREFIX}_barcode_land_{i}"), 82.0, 12.0, 4.0)
                .translate(
                    centered_index(i % 5, 5, 96.0) - 54.0,
                    -48.0 + (i / 5) as f64 * 26.0,
                    CUSTODY_Z / 2.0 + 2.0,
                );
    }
    lands
}

fn rfid_lands() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_rfid_lands"));
    for i in 0..RFID_LANDS {
        lands = lands
            + centered_cylinder(format!("{OUTPUT_PREFIX}_rfid_land_{i}"), 17.0, 4.0, 36).translate(
                CUSTODY_X / 2.0 - 70.0,
                centered_index(i, RFID_LANDS, 34.0),
                CUSTODY_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn status_lane_surfaces() -> Part {
    let mut lanes = Part::empty(format!("{OUTPUT_PREFIX}_status_lane_surfaces"));
    for i in 0..STATUS_LANES {
        lanes = lanes
            + centered_cube(format!("{OUTPUT_PREFIX}_status_lane_{i}"), 62.0, 22.0, 5.0).translate(
                -CUSTODY_X / 2.0 + 54.0,
                centered_index(i, STATUS_LANES, 32.0),
                CUSTODY_Z / 2.0 + 2.5,
            );
    }
    lanes
}

fn lot_card_slots() -> Part {
    let mut slots = Part::empty(format!("{OUTPUT_PREFIX}_lot_custody_card_slots"));
    for i in 0..LOT_CARD_SLOTS {
        let slot = centered_cube(
            format!("{OUTPUT_PREFIX}_lot_custody_card_slot_frame_{i}"),
            48.0,
            24.0,
            8.0,
        )
        .translate(
            centered_index(i, LOT_CARD_SLOTS, 56.0) + 28.0,
            CUSTODY_Y / 2.0 - 30.0,
            CUSTODY_Z / 2.0 + 4.0,
        );
        let relief = centered_cube(
            format!("{OUTPUT_PREFIX}_lot_custody_card_slot_relief_cut_{i}"),
            36.0,
            16.0,
            9.0,
        )
        .translate(
            centered_index(i, LOT_CARD_SLOTS, 56.0) + 28.0,
            CUSTODY_Y / 2.0 - 30.0,
            CUSTODY_Z / 2.0 + 4.5,
        );
        slots = slots + (slot - relief);
    }
    slots
}

fn robot_service_keepouts() -> Part {
    let front_robot = keepout_box(
        format!("{OUTPUT_PREFIX}_front_robot_distribution_clearance"),
        KEEP_OUT_X,
        94.0,
        FRONT_ROBOT_CLEARANCE,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + 74.0,
        DECK_Z + FRONT_ROBOT_CLEARANCE / 2.0,
    );
    let rear_source = keepout_box(
        format!("{OUTPUT_PREFIX}_rear_source_bag_service_clearance"),
        KEEP_OUT_X,
        88.0,
        REAR_SOURCE_SERVICE_CLEARANCE,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - 72.0,
        DECK_Z + REAR_SOURCE_SERVICE_CLEARANCE / 2.0,
    );
    let left_service = keepout_box(
        format!("{OUTPUT_PREFIX}_left_source_bag_service_clearance"),
        118.0,
        KEEP_OUT_Y,
        LEFT_BAG_SERVICE_CLEARANCE,
    )
    .translate(
        -DECK_X / 2.0 + 74.0,
        0.0,
        DECK_Z + LEFT_BAG_SERVICE_CLEARANCE / 2.0,
    );
    let right_service = keepout_box(
        format!("{OUTPUT_PREFIX}_right_manifold_service_clearance"),
        118.0,
        KEEP_OUT_Y,
        RIGHT_MANIFOLD_SERVICE_CLEARANCE,
    )
    .translate(
        DECK_X / 2.0 - 74.0,
        0.0,
        DECK_Z + RIGHT_MANIFOLD_SERVICE_CLEARANCE / 2.0,
    );
    let top_lift = keepout_box(
        format!("{OUTPUT_PREFIX}_top_source_bag_lift_clearance"),
        SOURCE_X + 80.0,
        SOURCE_Y + 80.0,
        TOP_BAG_LIFT_CLEARANCE_Z,
    )
    .translate(
        SOURCE_CENTER.0,
        SOURCE_CENTER.1,
        DECK_Z + TOP_BAG_LIFT_CLEARANCE_Z / 2.0,
    );
    let sample_sensor_service = keepout_box(
        format!("{OUTPUT_PREFIX}_sample_sensor_service_clearance"),
        SAMPLING_X + 60.0,
        SAMPLING_Y + 60.0,
        SAMPLE_SENSOR_SERVICE_CLEARANCE_Z,
    )
    .translate(
        SAMPLING_CENTER.0,
        SAMPLING_CENTER.1,
        DECK_Z + SAMPLE_SENSOR_SERVICE_CLEARANCE_Z / 2.0,
    );

    front_robot + rear_source + left_service + right_service + top_lift + sample_sensor_service
}

fn closed_loop_route_tube_harness() -> Part {
    route_segment_x(
        "source_to_mixer_front",
        SOURCE_CENTER.0 + SOURCE_X / 2.0 - 16.0,
        MIXER_CENTER.0 - MIXER_X / 2.0 + 18.0,
        SOURCE_CENTER.1 - 60.0,
    ) + route_segment_y(
        "mixer_down_to_pump",
        MIXER_CENTER.0,
        MIXER_CENTER.1 - MIXER_Y / 2.0 + 22.0,
        PUMP_CENTER.1 + PUMP_Y / 2.0 - 22.0,
    ) + route_segment_x(
        "pump_to_sampling_branch",
        PUMP_CENTER.0 + PUMP_X / 2.0 - 26.0,
        SAMPLING_CENTER.0 - SAMPLING_X / 2.0 + 24.0,
        PUMP_CENTER.1 + 72.0,
    ) + route_segment_y(
        "sampling_branch_rise",
        SAMPLING_CENTER.0 - 72.0,
        PUMP_CENTER.1 + 72.0,
        SAMPLING_CENTER.1 - SAMPLING_Y / 2.0 + 20.0,
    ) + route_segment_y(
        "sampling_to_handoff_drop",
        SAMPLING_CENTER.0 + 88.0,
        SAMPLING_CENTER.1 - SAMPLING_Y / 2.0 + 20.0,
        HANDOFF_CENTER.1 + HANDOFF_Y / 2.0 - 18.0,
    ) + route_segment_x(
        "handoff_to_bubble_return",
        HANDOFF_CENTER.0 - HANDOFF_X / 2.0 + 24.0,
        BUBBLE_CENTER.0 + BUBBLE_X / 2.0 - 18.0,
        HANDOFF_CENTER.1 - 38.0,
    ) + route_segment_y(
        "bubble_return_rise",
        BUBBLE_CENTER.0 + 64.0,
        BUBBLE_CENTER.1 + BUBBLE_Y / 2.0 - 18.0,
        SOURCE_CENTER.1 - SOURCE_Y / 2.0 + 28.0,
    ) + route_segment_x(
        "bubble_to_source_return",
        BUBBLE_CENTER.0 + 64.0,
        SOURCE_CENTER.0 - 48.0,
        SOURCE_CENTER.1 - SOURCE_Y / 2.0 + 28.0,
    ) + route_segment_x(
        "sampling_bypass_upper_span",
        MIXER_CENTER.0 + MIXER_X / 2.0 - 18.0,
        SAMPLING_CENTER.0 - SAMPLING_X / 2.0 + 22.0,
        MIXER_CENTER.1 + MIXER_Y / 2.0 - 30.0,
    ) + route_segment_y(
        "manifold_closed_handoff_stub",
        HANDOFF_CENTER.0 + HANDOFF_X / 2.0 - 32.0,
        HANDOFF_CENTER.1 - 48.0,
        HANDOFF_CENTER.1 + 74.0,
    ) + route_elbows()
        + route_direction_markers()
}

fn route_segment_x(name: &str, x0: f64, x1: f64, y: f64) -> Part {
    centered_cylinder(
        format!("{OUTPUT_PREFIX}_route_{name}"),
        ROUTE_TUBE_D / 2.0,
        (x1 - x0).abs(),
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate((x0 + x1) / 2.0, y, ROUTE_Z)
}

fn route_segment_y(name: &str, x: f64, y0: f64, y1: f64) -> Part {
    centered_cylinder(
        format!("{OUTPUT_PREFIX}_route_{name}"),
        ROUTE_TUBE_D / 2.0,
        (y1 - y0).abs(),
        24,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(x, (y0 + y1) / 2.0, ROUTE_Z)
}

fn route_elbows() -> Part {
    let mut elbows = Part::empty(format!("{OUTPUT_PREFIX}_closed_loop_route_elbows"));
    for (i, (x, y)) in [
        (
            SOURCE_CENTER.0 - 48.0,
            SOURCE_CENTER.1 - SOURCE_Y / 2.0 + 28.0,
        ),
        (MIXER_CENTER.0, MIXER_CENTER.1 - MIXER_Y / 2.0 + 22.0),
        (PUMP_CENTER.0 + PUMP_X / 2.0 - 26.0, PUMP_CENTER.1 + 72.0),
        (SAMPLING_CENTER.0 - 72.0, PUMP_CENTER.1 + 72.0),
        (
            SAMPLING_CENTER.0 + 88.0,
            SAMPLING_CENTER.1 - SAMPLING_Y / 2.0 + 20.0,
        ),
        (
            HANDOFF_CENTER.0 - HANDOFF_X / 2.0 + 24.0,
            HANDOFF_CENTER.1 - 38.0,
        ),
        (
            BUBBLE_CENTER.0 + 64.0,
            BUBBLE_CENTER.1 + BUBBLE_Y / 2.0 - 18.0,
        ),
        (
            BUBBLE_CENTER.0 + 64.0,
            SOURCE_CENTER.1 - SOURCE_Y / 2.0 + 28.0,
        ),
        (
            HANDOFF_CENTER.0 + HANDOFF_X / 2.0 - 32.0,
            HANDOFF_CENTER.1 + 74.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        elbows = elbows
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_closed_loop_route_elbow_{i}"),
                ROUTE_TUBE_D / 2.0 + 2.0,
                12.0,
                24,
            )
            .translate(*x, *y, ROUTE_Z);
    }
    elbows
}

fn route_direction_markers() -> Part {
    let mut markers = Part::empty(format!("{OUTPUT_PREFIX}_route_direction_markers"));
    for (i, (x, y, dx, dy)) in [
        (-330.0, 140.0, 1.0, 0.0),
        (-60.0, 70.0, 0.0, -1.0),
        (150.0, -32.0, 1.0, 0.0),
        (338.0, 76.0, 0.0, 1.0),
        (498.0, 28.0, 0.0, -1.0),
        (-40.0, -143.0, -1.0, 0.0),
        (-451.0, 30.0, 0.0, 1.0),
        (-520.0, 103.0, -1.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        markers = markers
            + flow_arrow_marker(
                format!("{OUTPUT_PREFIX}_route_direction_marker_{i}"),
                *dx,
                *dy,
            )
            .translate(*x, *y, ROUTE_Z + 10.0);
    }
    markers
}

fn reference_control_coupon_rack() -> Part {
    let base = centered_cube(
        format!("{OUTPUT_PREFIX}_reference_control_coupon_rack_base"),
        CONTROL_X,
        CONTROL_Y,
        CONTROL_Z,
    );

    base - control_coupon_pocket_cuts() + control_coupon_label_lands() + control_datum_posts()
}

fn control_coupon_pocket_cuts() -> Part {
    let mut cuts = Part::empty(format!(
        "{OUTPUT_PREFIX}_reference_control_coupon_pocket_cuts"
    ));
    for i in 0..CONTROL_COUPONS {
        cuts = cuts
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_reference_control_coupon_{}_pocket_cut", i),
                CONTROL_COUPON_D / 2.0,
                10.0,
                28,
            )
            .translate(
                centered_index(i % CONTROL_COLS, CONTROL_COLS, 70.0),
                centered_index(i / CONTROL_COLS, CONTROL_ROWS, 40.0),
                CONTROL_Z / 2.0 - 4.5,
            );
    }
    cuts
}

fn control_coupon_label_lands() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_reference_control_label_lands"));
    for i in 0..CONTROL_COUPONS {
        lands = lands
            + centered_cube(
                format!("{OUTPUT_PREFIX}_reference_control_coupon_{i}_label_land"),
                36.0,
                10.0,
                4.0,
            )
            .translate(
                centered_index(i % CONTROL_COLS, CONTROL_COLS, 70.0),
                centered_index(i / CONTROL_COLS, CONTROL_ROWS, 40.0) + 20.0,
                CONTROL_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn control_datum_posts() -> Part {
    let mut posts = Part::empty(format!("{OUTPUT_PREFIX}_reference_control_datum_posts"));
    for (i, (sx, sy)) in [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)]
        .iter()
        .enumerate()
    {
        posts = posts
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_reference_control_datum_post_{i}"),
                5.0,
                12.0,
                24,
            )
            .translate(
                sx * (CONTROL_X / 2.0 - 28.0),
                sy * (CONTROL_Y / 2.0 - 24.0),
                CONTROL_Z / 2.0 + 6.0,
            );
    }
    posts
}

fn keepout_box(name: String, x: f64, y: f64, z: f64) -> Part {
    let envelope = centered_cube(format!("{name}_envelope"), x, y, z);
    let hollow = centered_cube(format!("{name}_hollow"), x - 18.0, y - 18.0, z - 18.0);
    envelope - hollow
}

fn flow_arrow_marker(name: String, dx: f64, dy: f64) -> Part {
    let shaft = if dx.abs() > dy.abs() {
        centered_cube(format!("{name}_shaft"), 26.0, 6.0, 4.0).translate(
            dx.signum() * -3.0,
            0.0,
            0.0,
        )
    } else {
        centered_cube(format!("{name}_shaft"), 6.0, 26.0, 4.0).translate(
            0.0,
            dy.signum() * -3.0,
            0.0,
        )
    };
    let head = if dx.abs() > dy.abs() {
        centered_cube(format!("{name}_head"), 9.0, 18.0, 4.0).translate(
            dx.signum() * 16.0,
            0.0,
            0.0,
        )
    } else {
        centered_cube(format!("{name}_head"), 18.0, 9.0, 4.0).translate(
            0.0,
            dy.signum() * 16.0,
            0.0,
        )
    };

    shaft + head
}

fn primary_footprints() -> [Footprint; 9] {
    [
        Footprint {
            name: "source_bag_vessel_nest",
            center: SOURCE_CENTER,
            x: SOURCE_X,
            y: SOURCE_Y,
        },
        Footprint {
            name: "gentle_recirculation_mixing_path",
            center: MIXER_CENTER,
            x: MIXER_X,
            y: MIXER_Y,
        },
        Footprint {
            name: "inline_density_viability_sampling_surrogate",
            center: SAMPLING_CENTER,
            x: SAMPLING_X,
            y: SAMPLING_Y,
        },
        Footprint {
            name: "low_shear_pump_bypass_guard",
            center: PUMP_CENTER,
            x: PUMP_X,
            y: PUMP_Y,
        },
        Footprint {
            name: "bubble_settling_trap_bank",
            center: BUBBLE_CENTER,
            x: BUBBLE_X,
            y: BUBBLE_Y,
        },
        Footprint {
            name: "closed_connector_handoff_manifold",
            center: HANDOFF_CENTER,
            x: HANDOFF_X,
            y: HANDOFF_Y,
        },
        Footprint {
            name: "time_since_mix_witness_lanes",
            center: WITNESS_CENTER,
            x: WITNESS_X,
            y: WITNESS_Y,
        },
        Footprint {
            name: "barcode_custody_status_surfaces",
            center: CUSTODY_CENTER,
            x: CUSTODY_X,
            y: CUSTODY_Y,
        },
        Footprint {
            name: "reference_control_coupon_rack",
            center: CONTROL_CENTER,
            x: CONTROL_X,
            y: CONTROL_Y,
        },
    ]
}

fn keepout_outline() -> Footprint {
    Footprint {
        name: "robot_service_keepout_outline",
        center: (0.0, 0.0),
        x: KEEP_OUT_X,
        y: KEEP_OUT_Y,
    }
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 13, "expected stable STL export count");
    assert_eq!(REQUIRED_FEATURES.len(), 14, "required feature list changed");
    assert_eq!(
        REPRODUCIBILITY_CONTROLS.len(),
        9,
        "reproducibility controls list changed"
    );
    assert_eq!(
        WITNESS_LANE_COUNT, 3,
        "time-since-mix witnesses must keep three lanes"
    );
    assert_eq!(
        CONTROL_ROWS * CONTROL_COLS,
        CONTROL_COUPONS,
        "reference controls must remain a complete grid"
    );
    assert_eq!(
        DRY_BREAK_CONNECTORS, CONNECTOR_CAP_PARKS,
        "every closed connector must have a cap park"
    );
    assert_eq!(ROUTE_ELBOWS, 9, "closed-loop elbow count changed");
    assert_eq!(
        ROUTE_DIRECTION_MARKERS, 8,
        "route direction marker count changed"
    );
    assert_eq!(KEEP_OUT_ZONES, 5, "keepout zone count changed");
    assert!(
        MIX_AGE_MINUTES
            .windows(2)
            .all(|window| window[0] < window[1]),
        "mix-age witness tokens must increase monotonically"
    );
    assert!(
        FRONT_ROBOT_CLEARANCE >= 340.0
            && REAR_SOURCE_SERVICE_CLEARANCE >= 240.0
            && RIGHT_MANIFOLD_SERVICE_CLEARANCE >= 240.0,
        "robot/service clearances below station target"
    );

    let footprints = primary_footprints();
    for footprint in footprints {
        assert!(
            footprint.fits_on_deck(),
            "{} does not fit inside the deck",
            footprint.name
        );
    }
    assert!(
        keepout_outline().fits_on_deck(),
        "keepout outline must fit inside the deck"
    );
    for left in 0..footprints.len() {
        for right in left + 1..footprints.len() {
            assert!(
                !footprints[left].overlaps(footprints[right]),
                "{} overlaps {}",
                footprints[left].name,
                footprints[right].name
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

        assert_eq!(OUTPUTS.len(), 13);
        assert_eq!(unique.len(), OUTPUTS.len());
        assert!(OUTPUTS.iter().all(|path| path.starts_with(
            "output/closed_cell_seeding_suspension_homogeneity_recirculation_station_"
        )));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
        assert!(OUTPUTS[0].ends_with("_leak_tray_deck.stl"));
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }

    #[test]
    fn required_features_cover_closed_homogeneity_fixture_intent() {
        for feature in [
            "source_bag_vessel_nest",
            "gentle_recirculation_mixing_path",
            "inline_density_viability_sampling_surrogate",
            "time_since_mix_witness_lanes",
            "bubble_settling_trap_bank",
            "closed_connector_handoff_manifold",
            "barcode_custody_status_surfaces",
            "robot_service_keepouts",
            "closed_loop_route_tube_harness",
            "reference_control_coupon_rack",
            "distribution_manifold_handoff_key",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
        assert_eq!(REQUIRED_FEATURES.len(), 14);
    }

    #[test]
    fn station_bounds_and_primary_modules_do_not_overlap() {
        assert!(DECK_X <= 1600.0);
        assert!(DECK_Y <= 980.0);
        assert!(TRAY_RIM_Z >= 34.0);
        assert_eq!(primary_footprints().len(), 9);
        assert_design_constraints();
    }

    #[test]
    fn closed_loop_counts_and_handoff_interfaces_are_explicit() {
        assert_eq!(ROUTE_SEGMENTS, 10);
        assert_eq!(ROUTE_ELBOWS, 9);
        assert_eq!(ROUTE_DIRECTION_MARKERS, 8);
        assert_eq!(SOURCE_PORT_SADDLES, DRY_BREAK_CONNECTORS);
        assert_eq!(DRY_BREAK_CONNECTORS, CONNECTOR_CAP_PARKS);
        assert_eq!(MANIFOLD_LANES, 8);
        assert_eq!(HANDOFF_KEY_PINS, 4);
        assert_eq!(HANDOFF_LATCH_EARS, 2);
    }

    #[test]
    fn reproducibility_controls_are_modeled_without_acceptance_thresholds() {
        for control in [
            "time_since_mix_witness_lanes",
            "mix_age_token_slots",
            "inline_density_viability_sampling_surrogate",
            "bubble_settling_trap_bank",
            "barcode_custody_status_surfaces",
            "reference_control_coupon_rack",
            "closed_connector_handoff_manifold",
            "route_direction_markers",
            "robot_service_keepouts",
        ] {
            assert!(REPRODUCIBILITY_CONTROLS.contains(&control));
        }
        assert_eq!(MIX_AGE_TOKEN_COUNT, MIX_AGE_MINUTES.len());
        assert_eq!(MIX_AGE_MINUTES[0], 0);
        assert!(MIX_AGE_MINUTES[MIX_AGE_TOKEN_COUNT - 1] >= 45);
        assert_eq!(WITNESS_LANE_COUNT, 3);
        assert_eq!(CONTROL_COUPONS, 9);
    }

    #[test]
    fn sampling_bubble_and_custody_feature_counts_stay_complete() {
        assert_eq!(DENSITY_WINDOWS, 5);
        assert_eq!(VIABILITY_SURROGATE_SLOTS, 4);
        assert_eq!(SAMPLE_BRANCH_VALVES, 6);
        assert_eq!(BUBBLE_TRAPS, 4);
        assert_eq!(SETTLING_WELLS, 6);
        assert_eq!(BARCODE_LANDS, 10);
        assert_eq!(RFID_LANDS, 4);
        assert_eq!(STATUS_LANES, 4);
        assert_eq!(LOT_CARD_SLOTS, 6);
    }

    #[test]
    fn robot_and_service_clearances_cover_source_sampling_and_handoff() {
        assert_eq!(KEEP_OUT_ZONES, 5);
        assert!(FRONT_ROBOT_CLEARANCE >= 360.0);
        assert!(REAR_SOURCE_SERVICE_CLEARANCE >= 250.0);
        assert!(LEFT_BAG_SERVICE_CLEARANCE >= 260.0);
        assert!(RIGHT_MANIFOLD_SERVICE_CLEARANCE >= 250.0);
        assert!(TOP_BAG_LIFT_CLEARANCE_Z > SOURCE_Z + 250.0);
        assert!(SAMPLE_SENSOR_SERVICE_CLEARANCE_Z > SAMPLING_Z + 150.0);
    }
}
