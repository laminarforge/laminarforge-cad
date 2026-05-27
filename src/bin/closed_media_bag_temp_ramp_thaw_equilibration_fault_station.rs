use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed media-bag temperature ramp/thaw/equilibration fault station.
//
// This generator models a closed-system validation fixture for pre-feed media
// bag thermal handling. Geometry captures containment, cold/warm nests,
// dry-block surrogates, probe and sample-loop witnesses, logger custody,
// wetness evidence, challenge tokens, connector handoff, release routing,
// camera evidence, and keepout gauges. It intentionally encodes only fixture
// interfaces and traceability hardware, not culture recipes, release criteria,
// or performance claims.

const OUTPUT_PREFIX: &str = "closed_media_bag_temp_ramp_thaw_equilibration_fault_station";

const OUTPUTS: [&str; 13] = [
    "output/closed_media_bag_temp_ramp_thaw_equilibration_fault_station_leak_tray_deck.stl",
    "output/closed_media_bag_temp_ramp_thaw_equilibration_fault_station_chilled_and_warm_bag_nests.stl",
    "output/closed_media_bag_temp_ramp_thaw_equilibration_fault_station_dry_thaw_equilibration_block_surrogates.stl",
    "output/closed_media_bag_temp_ramp_thaw_equilibration_fault_station_inline_temperature_probe_pockets.stl",
    "output/closed_media_bag_temp_ramp_thaw_equilibration_fault_station_stratification_sample_loop_witness.stl",
    "output/closed_media_bag_temp_ramp_thaw_equilibration_fault_station_thermal_logger_custody_lands.stl",
    "output/closed_media_bag_temp_ramp_thaw_equilibration_fault_station_condensation_wetness_wells.stl",
    "output/closed_media_bag_temp_ramp_thaw_equilibration_fault_station_overtemp_undertemp_challenge_tokens.stl",
    "output/closed_media_bag_temp_ramp_thaw_equilibration_fault_station_sterile_connector_handoff_bulkhead.stl",
    "output/closed_media_bag_temp_ramp_thaw_equilibration_fault_station_release_hold_reject_lanes.stl",
    "output/closed_media_bag_temp_ramp_thaw_equilibration_fault_station_evidence_camera_bridge.stl",
    "output/closed_media_bag_temp_ramp_thaw_equilibration_fault_station_robot_service_keepouts.stl",
    "output/closed_media_bag_temp_ramp_thaw_equilibration_fault_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 12] = [
    "base_leak_tray_deck",
    "chilled_bag_nest",
    "warm_bag_nest",
    "dry_thaw_equilibration_block_surrogate",
    "inline_temperature_probe_pockets",
    "stratification_sample_loop_witness",
    "thermal_logger_custody_lands",
    "condensation_wetness_wells",
    "overtemp_undertemp_challenge_tokens",
    "sterile_connector_handoff_bulkhead",
    "release_hold_reject_lanes",
    "robot_service_keepouts",
];

#[cfg(test)]
const CLAIM_TEXTS: [&str; 6] = [
    "validation fixture only",
    "closed media bag interface surrogate",
    "dry thermal block challenge hardware",
    "temperature evidence and custody geometry",
    "release hold reject sorting aid",
    "no culture recipe or acceptance criteria",
];

const STATION_X: f64 = 1560.0;
const STATION_Y: f64 = 980.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 22.0;
const RIM_Z: f64 = 54.0;
const SUMP_DEPTH: f64 = 7.0;
const SOCKET_DEPTH: f64 = 5.0;
const MOUNT_HOLE_D: f64 = 6.8;
const DATUM_TARGETS: usize = 6;

const BAG_NEST_COUNT: usize = 2;
const BAG_NEST_X: f64 = 360.0;
const BAG_NEST_Y: f64 = 250.0;
const BAG_NEST_Z: f64 = 58.0;
const BAG_POCKET_X: f64 = 276.0;
const BAG_POCKET_Y: f64 = 170.0;
const BAG_POCKET_DEPTH: f64 = 24.0;
const BAG_RIBS_PER_NEST: usize = 6;
const BAG_LOCATOR_PINS_PER_NEST: usize = 4;
const BAG_PORT_KEYS_PER_NEST: usize = 4;

const CHILLED_NEST_CENTER: (f64, f64) = (-470.0, 190.0);
const WARM_NEST_CENTER: (f64, f64) = (-80.0, 190.0);

const BLOCK_CENTER: (f64, f64) = (390.0, 190.0);
const BLOCK_X: f64 = 470.0;
const BLOCK_Y: f64 = 250.0;
const BLOCK_Z: f64 = 64.0;
const DRY_BLOCK_COUNT: usize = 3;
const THAW_CHANNELS_PER_BLOCK: usize = 4;
const THERMAL_RAMP_STEPS: usize = 7;
const BLOCK_CHANNEL_X: f64 = 86.0;
const BLOCK_CHANNEL_Y: f64 = 30.0;

const PROBE_CENTER: (f64, f64) = (-475.0, -80.0);
const PROBE_X: f64 = 340.0;
const PROBE_Y: f64 = 220.0;
const PROBE_Z: f64 = 46.0;
const INLINE_PROBE_POCKETS: usize = 12;
const PROBE_POCKET_D: f64 = 11.0;
const PROBE_REFERENCE_WELLS: usize = 4;

const STRAT_CENTER: (f64, f64) = (-75.0, -80.0);
const STRAT_X: f64 = 390.0;
const STRAT_Y: f64 = 220.0;
const STRAT_Z: f64 = 48.0;
const STRATIFICATION_LEVELS: usize = 3;
const LOOPS_PER_LEVEL: usize = 4;
const SAMPLE_LOOP_WITNESSES: usize = STRATIFICATION_LEVELS * LOOPS_PER_LEVEL;
const SAMPLE_LOOP_D: f64 = 17.0;

const LOGGER_CENTER: (f64, f64) = (365.0, -80.0);
const LOGGER_X: f64 = 410.0;
const LOGGER_Y: f64 = 220.0;
const LOGGER_Z: f64 = 42.0;
const THERMAL_LOGGER_LANDS: usize = 8;
const LOGGER_SEAL_WITNESSES: usize = 8;
const LOGGER_CARD_LANDS: usize = 4;

const WETNESS_CENTER: (f64, f64) = (-535.0, -326.0);
const WETNESS_X: f64 = 310.0;
const WETNESS_Y: f64 = 132.0;
const WETNESS_Z: f64 = 24.0;
const WETNESS_WELLS: usize = 10;
const CONDENSATION_GUTTERS: usize = 4;
const WETNESS_WELL_D: f64 = 24.0;

const TOKEN_CENTER: (f64, f64) = (-185.0, -326.0);
const TOKEN_X: f64 = 300.0;
const TOKEN_Y: f64 = 132.0;
const TOKEN_Z: f64 = 26.0;
const OVERTEMP_TOKENS: usize = 4;
const UNDERTEMP_TOKENS: usize = 4;
const TOKEN_DIAMETER: f64 = 24.0;

const BULKHEAD_CENTER: (f64, f64) = (155.0, -326.0);
const BULKHEAD_X: f64 = 300.0;
const BULKHEAD_Y: f64 = 132.0;
const BULKHEAD_Z: f64 = 70.0;
const STERILE_CONNECTORS: usize = 6;
const CONNECTOR_D: f64 = 19.0;
const CAP_PARK_WELLS: usize = 6;

const DECISION_CENTER: (f64, f64) = (500.0, -326.0);
const DECISION_X: f64 = 280.0;
const DECISION_Y: f64 = 132.0;
const DECISION_Z: f64 = 24.0;
const DECISION_LANES: usize = 3;
const DECISION_TOKENS_PER_LANE: usize = 3;
const DECISION_TOKENS: usize = DECISION_LANES * DECISION_TOKENS_PER_LANE;

const BRIDGE_CENTER: (f64, f64) = (0.0, 405.0);
const BRIDGE_SPAN_X: f64 = 1310.0;
const BRIDGE_POST_X: f64 = 34.0;
const BRIDGE_POST_Y: f64 = 48.0;
const BRIDGE_CLEARANCE_Z: f64 = 226.0;
const BRIDGE_BEAM_Z: f64 = 32.0;
const CAMERA_PODS: usize = 4;
const LIGHT_BARS: usize = 2;

const FRONT_ROBOT_CLEARANCE: f64 = 430.0;
const REAR_SERVICE_CLEARANCE: f64 = 270.0;
const LEFT_BAG_SERVICE_CLEARANCE: f64 = 240.0;
const RIGHT_CONNECTOR_SERVICE_CLEARANCE: f64 = 250.0;
const TOP_BAG_LIFT_CLEARANCE: f64 = 360.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_station(self) -> bool {
        self.center.0.abs() + self.x / 2.0 <= STATION_X / 2.0 - RIM_W - 14.0
            && self.center.1.abs() + self.y / 2.0 <= STATION_Y / 2.0 - RIM_W - 14.0
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

    let deck = leak_tray_deck();
    export(OUTPUTS[0], &deck);

    let nests = chilled_and_warm_bag_nests();
    export(OUTPUTS[1], &nests);

    let blocks = dry_thaw_equilibration_block_surrogates();
    export(OUTPUTS[2], &blocks);

    let probes = inline_temperature_probe_pockets();
    export(OUTPUTS[3], &probes);

    let stratification = stratification_sample_loop_witness();
    export(OUTPUTS[4], &stratification);

    let loggers = thermal_logger_custody_lands();
    export(OUTPUTS[5], &loggers);

    let wetness = condensation_wetness_wells();
    export(OUTPUTS[6], &wetness);

    let tokens = overtemp_undertemp_challenge_tokens();
    export(OUTPUTS[7], &tokens);

    let bulkhead = sterile_connector_handoff_bulkhead();
    export(OUTPUTS[8], &bulkhead);

    let decision = release_hold_reject_lanes();
    export(OUTPUTS[9], &decision);

    let bridge = evidence_camera_bridge();
    export(OUTPUTS[10], &bridge);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[11], &keepouts);

    let assembly = deck
        + nests
        + blocks
        + probes
        + stratification
        + loggers
        + wetness
        + tokens
        + bulkhead
        + decision
        + bridge
        + keepouts;
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed media-bag temperature ramp/thaw/equilibration fault station:");
    println!("  Footprint:                  {STATION_X:.0}mm x {STATION_Y:.0}mm leak tray with raised rim and dry sockets");
    println!(
        "  Bag nests:                  {BAG_NEST_COUNT} nests, chilled and warm, each with {BAG_RIBS_PER_NEST} ribs and {BAG_PORT_KEYS_PER_NEST} port-key lands"
    );
    println!(
        "  Dry thermal surrogates:     {DRY_BLOCK_COUNT} blocks, {THAW_CHANNELS_PER_BLOCK} channels each, {THERMAL_RAMP_STEPS} ramp-step witnesses"
    );
    println!(
        "  Temperature evidence:       {INLINE_PROBE_POCKETS} inline probe pockets, {PROBE_REFERENCE_WELLS} reference wells, {THERMAL_LOGGER_LANDS} logger custody lands"
    );
    println!(
        "  Fault challenges:           {OVERTEMP_TOKENS} overtemp tokens, {UNDERTEMP_TOKENS} undertemp tokens, {WETNESS_WELLS} wetness wells"
    );
    println!(
        "  Handoff and disposition:    {STERILE_CONNECTORS} connector positions, release/hold/reject lanes with {DECISION_TOKENS} custody tokens"
    );
    println!(
        "  Robot/service keepouts:     front {FRONT_ROBOT_CLEARANCE:.0}mm, rear {REAR_SERVICE_CLEARANCE:.0}mm, left {LEFT_BAG_SERVICE_CLEARANCE:.0}mm, right {RIGHT_CONNECTOR_SERVICE_CLEARANCE:.0}mm, top {TOP_BAG_LIFT_CLEARANCE:.0}mm"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_design_constraints() {
    assert_eq!(BAG_NEST_COUNT, 2);
    assert_eq!(OUTPUTS.len(), 13);
    assert_eq!(REQUIRED_FEATURES.len(), 12);
    assert_eq!(THERMAL_LOGGER_LANDS, LOGGER_SEAL_WITNESSES);
    assert_eq!(
        SAMPLE_LOOP_WITNESSES,
        STRATIFICATION_LEVELS * LOOPS_PER_LEVEL
    );
    assert_eq!(DECISION_TOKENS, DECISION_LANES * DECISION_TOKENS_PER_LANE);
    assert!(INLINE_PROBE_POCKETS >= BAG_NEST_COUNT * 4 + DRY_BLOCK_COUNT);
    assert!(WETNESS_WELLS >= CONDENSATION_GUTTERS * 2);
    assert!(STERILE_CONNECTORS >= BAG_NEST_COUNT + DRY_BLOCK_COUNT);
    assert!(BRIDGE_CLEARANCE_Z > BASE_Z + BAG_NEST_Z + 90.0);
    assert!(TOP_BAG_LIFT_CLEARANCE > BRIDGE_CLEARANCE_Z + BRIDGE_BEAM_Z);

    let rects = module_rects();
    for rect in rects {
        assert!(
            rect.fits_inside_station(),
            "{} must fit within the leak-tray station bounds",
            rect.name
        );
    }

    for (index, first) in rects.iter().enumerate() {
        for second in rects.iter().skip(index + 1) {
            assert!(
                !first.overlaps_with_clearance(*second, 12.0),
                "{} overlaps {}",
                first.name,
                second.name
            );
        }
    }
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

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn module_rects() -> [Rect; 10] {
    [
        rect(
            "chilled_bag_nest",
            CHILLED_NEST_CENTER,
            BAG_NEST_X,
            BAG_NEST_Y,
        ),
        rect("warm_bag_nest", WARM_NEST_CENTER, BAG_NEST_X, BAG_NEST_Y),
        rect(
            "dry_thaw_equilibration_block_surrogates",
            BLOCK_CENTER,
            BLOCK_X,
            BLOCK_Y,
        ),
        rect(
            "inline_temperature_probe_pockets",
            PROBE_CENTER,
            PROBE_X,
            PROBE_Y,
        ),
        rect(
            "stratification_sample_loop_witness",
            STRAT_CENTER,
            STRAT_X,
            STRAT_Y,
        ),
        rect(
            "thermal_logger_custody_lands",
            LOGGER_CENTER,
            LOGGER_X,
            LOGGER_Y,
        ),
        rect(
            "condensation_wetness_wells",
            WETNESS_CENTER,
            WETNESS_X,
            WETNESS_Y,
        ),
        rect(
            "overtemp_undertemp_challenge_tokens",
            TOKEN_CENTER,
            TOKEN_X,
            TOKEN_Y,
        ),
        rect(
            "sterile_connector_handoff_bulkhead",
            BULKHEAD_CENTER,
            BULKHEAD_X,
            BULKHEAD_Y,
        ),
        rect(
            "release_hold_reject_lanes",
            DECISION_CENTER,
            DECISION_X,
            DECISION_Y,
        ),
    ]
}

fn leak_tray_deck() -> Part {
    let deck = centered_cube(
        format!("{OUTPUT_PREFIX}_base_leak_tray_deck"),
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let sump = centered_cube(
        format!("{OUTPUT_PREFIX}_shallow_leak_sump_recess"),
        STATION_X - 142.0,
        STATION_Y - 136.0,
        SUMP_DEPTH + 0.6,
    )
    .translate(0.0, -8.0, deck_top_z() - SUMP_DEPTH / 2.0 + 0.3);
    let drain = centered_cylinder(
        format!("{OUTPUT_PREFIX}_front_leak_tray_drain_cut"),
        9.0,
        RIM_W + 44.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(620.0, -STATION_Y / 2.0 + 12.0, deck_top_z() - 5.0);

    deck - sump - drain - module_sockets() - deck_mount_holes()
        + perimeter_rims()
        + datum_targets()
        + floor_zone_markers()
}

fn module_sockets() -> Part {
    let mut sockets = Part::empty(format!("{OUTPUT_PREFIX}_module_sockets"));
    for module in module_rects() {
        sockets = sockets
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{}_socket_cut", module.name),
                module.x + 14.0,
                module.y + 14.0,
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

fn perimeter_rims() -> Part {
    let front = centered_cube(
        format!("{OUTPUT_PREFIX}_front_low_leak_tray_lip"),
        STATION_X - 220.0,
        14.0,
        24.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 16.0, deck_top_z() + 12.0);
    let rear = centered_cube(
        format!("{OUTPUT_PREFIX}_rear_service_leak_tray_rim"),
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        deck_top_z() + RIM_Z / 2.0,
    );
    let left = centered_cube(
        format!("{OUTPUT_PREFIX}_left_bag_service_leak_tray_rim"),
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
        format!("{OUTPUT_PREFIX}_right_connector_service_leak_tray_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        deck_top_z() + RIM_Z / 2.0,
    );

    front + rear + left + right
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty(format!("{OUTPUT_PREFIX}_deck_mount_holes"));
    for (index, (x, y)) in [
        (-STATION_X / 2.0 + 64.0, -STATION_Y / 2.0 + 60.0),
        (STATION_X / 2.0 - 64.0, -STATION_Y / 2.0 + 60.0),
        (-STATION_X / 2.0 + 64.0, STATION_Y / 2.0 - 60.0),
        (STATION_X / 2.0 - 64.0, STATION_Y / 2.0 - 60.0),
        (-260.0, STATION_Y / 2.0 - 60.0),
        (260.0, STATION_Y / 2.0 - 60.0),
        (-260.0, -STATION_Y / 2.0 + 60.0),
        (260.0, -STATION_Y / 2.0 + 60.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_m6_mount_clearance_{index}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 4.0,
                24,
            )
            .translate(*x, *y, BASE_Z / 2.0);
    }
    holes
}

fn datum_targets() -> Part {
    let mut targets = Part::empty(format!("{OUTPUT_PREFIX}_robot_datum_targets"));
    for index in 0..DATUM_TARGETS {
        let x = centered_index(index % 3, 3, 555.0);
        let y = if index < 3 { -410.0 } else { 410.0 };
        let target = centered_cylinder(
            format!("{OUTPUT_PREFIX}_datum_target_outer_{index}"),
            18.0,
            5.0,
            36,
        )
        .translate(x, y, deck_top_z() + 2.5)
            - centered_cylinder(
                format!("{OUTPUT_PREFIX}_datum_target_inner_{index}"),
                5.0,
                7.0,
                24,
            )
            .translate(x, y, deck_top_z() + 2.5);
        targets = targets + target;
    }
    targets
}

fn floor_zone_markers() -> Part {
    let mut markers = Part::empty(format!("{OUTPUT_PREFIX}_floor_zone_markers"));
    for module in module_rects() {
        markers = markers
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{}_floor_marker", module.name),
                module.x + 22.0,
                module.y + 18.0,
                3.0,
            )
            .translate(module.center.0, module.center.1, deck_top_z() + 1.5);
    }
    markers
}

fn chilled_and_warm_bag_nests() -> Part {
    bag_nest("chilled", CHILLED_NEST_CENTER, 0) + bag_nest("warm", WARM_NEST_CENTER, 1)
}

fn bag_nest(label: &str, center: (f64, f64), nest_index: usize) -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_{label}_closed_media_bag_nest_body"),
        BAG_NEST_X,
        BAG_NEST_Y,
        BAG_NEST_Z,
    )
    .translate(center.0, center.1, place_z(BAG_NEST_Z));
    let pocket = centered_cube(
        format!("{OUTPUT_PREFIX}_{label}_soft_bag_pocket_cut"),
        BAG_POCKET_X,
        BAG_POCKET_Y,
        BAG_POCKET_DEPTH + 1.0,
    )
    .translate(
        center.0,
        center.1 - 8.0,
        deck_top_z() + BAG_NEST_Z - BAG_POCKET_DEPTH / 2.0 + 0.5,
    );

    body - pocket
        + bag_saddle_ribs(label, center)
        + bag_locator_pins(label, center)
        + bag_port_key_lands(label, center, nest_index)
        + bag_nest_thermal_boundary_tabs(label, center)
}

fn bag_saddle_ribs(label: &str, center: (f64, f64)) -> Part {
    let mut ribs = Part::empty(format!("{OUTPUT_PREFIX}_{label}_saddle_ribs"));
    for rib in 0..BAG_RIBS_PER_NEST {
        ribs = ribs
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{label}_bag_saddle_rib_{rib}"),
                BAG_POCKET_X - 34.0,
                7.0,
                12.0,
            )
            .translate(
                center.0,
                center.1 + centered_index(rib, BAG_RIBS_PER_NEST, 25.0) - 8.0,
                deck_top_z() + BAG_NEST_Z + 6.0,
            );
    }
    ribs
}

fn bag_locator_pins(label: &str, center: (f64, f64)) -> Part {
    let mut pins = Part::empty(format!("{OUTPUT_PREFIX}_{label}_bag_locator_pins"));
    for pin in 0..BAG_LOCATOR_PINS_PER_NEST {
        let x = center.0 + centered_index(pin % 2, 2, BAG_POCKET_X - 54.0);
        let y = center.1 + if pin < 2 { -86.0 } else { 74.0 };
        pins = pins
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_{label}_soft_bag_locator_pin_{pin}"),
                9.0,
                17.0,
                28,
            )
            .translate(x, y, deck_top_z() + BAG_NEST_Z + 8.5);
    }
    pins
}

fn bag_port_key_lands(label: &str, center: (f64, f64), nest_index: usize) -> Part {
    let mut keys = Part::empty(format!("{OUTPUT_PREFIX}_{label}_bag_port_key_lands"));
    for key in 0..BAG_PORT_KEYS_PER_NEST {
        keys = keys
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{label}_nest_{nest_index}_port_key_land_{key}"),
                44.0,
                24.0,
                12.0,
            )
            .translate(
                center.0 + centered_index(key, BAG_PORT_KEYS_PER_NEST, 58.0),
                center.1 - BAG_NEST_Y / 2.0 + 26.0,
                deck_top_z() + BAG_NEST_Z + 6.0,
            );
    }
    keys
}

fn bag_nest_thermal_boundary_tabs(label: &str, center: (f64, f64)) -> Part {
    let inlet = centered_cube(
        format!("{OUTPUT_PREFIX}_{label}_inlet_temperature_boundary_tab"),
        112.0,
        12.0,
        8.0,
    )
    .translate(
        center.0 - 72.0,
        center.1 + BAG_NEST_Y / 2.0 - 28.0,
        deck_top_z() + BAG_NEST_Z + 4.0,
    );
    let outlet = centered_cube(
        format!("{OUTPUT_PREFIX}_{label}_outlet_temperature_boundary_tab"),
        112.0,
        12.0,
        8.0,
    )
    .translate(
        center.0 + 72.0,
        center.1 + BAG_NEST_Y / 2.0 - 28.0,
        deck_top_z() + BAG_NEST_Z + 4.0,
    );
    inlet + outlet
}

fn dry_thaw_equilibration_block_surrogates() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_dry_thaw_equilibration_block_body"),
        BLOCK_X,
        BLOCK_Y,
        BLOCK_Z,
    )
    .translate(BLOCK_CENTER.0, BLOCK_CENTER.1, place_z(BLOCK_Z));

    body - dry_block_channel_cuts()
        + dry_block_channel_rims()
        + ramp_step_witness_ticks()
        + block_contact_coupon_lands()
}

fn dry_block_channel_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_dry_block_channel_cuts"));
    for block in 0..DRY_BLOCK_COUNT {
        let block_x = BLOCK_CENTER.0 + centered_index(block, DRY_BLOCK_COUNT, 138.0);
        for channel in 0..THAW_CHANNELS_PER_BLOCK {
            cuts = cuts
                + centered_cube(
                    format!("{OUTPUT_PREFIX}_dry_block_{block}_bag_channel_{channel}_cut"),
                    BLOCK_CHANNEL_X,
                    BLOCK_CHANNEL_Y,
                    BLOCK_Z + 2.0,
                )
                .translate(
                    block_x,
                    BLOCK_CENTER.1 + centered_index(channel, THAW_CHANNELS_PER_BLOCK, 42.0),
                    place_z(BLOCK_Z),
                );
        }
    }
    cuts
}

fn dry_block_channel_rims() -> Part {
    let mut rims = Part::empty(format!("{OUTPUT_PREFIX}_dry_block_channel_rims"));
    for block in 0..DRY_BLOCK_COUNT {
        let block_x = BLOCK_CENTER.0 + centered_index(block, DRY_BLOCK_COUNT, 138.0);
        for channel in 0..THAW_CHANNELS_PER_BLOCK {
            rims = rims
                + rectangular_frame(
                    &format!("{OUTPUT_PREFIX}_dry_block_{block}_channel_{channel}_rim"),
                    BLOCK_CHANNEL_X + 16.0,
                    BLOCK_CHANNEL_Y + 14.0,
                    5.0,
                    7.0,
                )
                .translate(
                    block_x,
                    BLOCK_CENTER.1 + centered_index(channel, THAW_CHANNELS_PER_BLOCK, 42.0),
                    deck_top_z() + BLOCK_Z + 3.5,
                );
        }
    }
    rims
}

fn ramp_step_witness_ticks() -> Part {
    let mut ticks = Part::empty(format!("{OUTPUT_PREFIX}_thermal_ramp_step_witness_ticks"));
    for step in 0..THERMAL_RAMP_STEPS {
        ticks = ticks
            + centered_cube(
                format!("{OUTPUT_PREFIX}_thermal_ramp_step_{step}_witness_tick"),
                42.0,
                8.0,
                9.0,
            )
            .translate(
                BLOCK_CENTER.0 + centered_index(step, THERMAL_RAMP_STEPS, 54.0),
                BLOCK_CENTER.1 + BLOCK_Y / 2.0 - 24.0,
                deck_top_z() + BLOCK_Z + 4.5,
            );
    }
    ticks
}

fn block_contact_coupon_lands() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_dry_block_contact_coupon_lands"));
    for block in 0..DRY_BLOCK_COUNT {
        lands = lands
            + centered_cube(
                format!("{OUTPUT_PREFIX}_dry_block_{block}_flatness_contact_coupon_land"),
                96.0,
                18.0,
                7.0,
            )
            .translate(
                BLOCK_CENTER.0 + centered_index(block, DRY_BLOCK_COUNT, 138.0),
                BLOCK_CENTER.1 - BLOCK_Y / 2.0 + 24.0,
                deck_top_z() + BLOCK_Z + 3.5,
            );
    }
    lands
}

fn inline_temperature_probe_pockets() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_inline_temperature_probe_pocket_body"),
        PROBE_X,
        PROBE_Y,
        PROBE_Z,
    )
    .translate(PROBE_CENTER.0, PROBE_CENTER.1, place_z(PROBE_Z));

    body - probe_pocket_cuts() - probe_cable_cross_channels()
        + probe_pocket_rims()
        + reference_well_lands()
        + probe_lead_strain_relief_combs()
}

fn probe_pocket_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_inline_probe_pocket_cuts"));
    for probe in 0..INLINE_PROBE_POCKETS {
        let col = probe % 4;
        let row = probe / 4;
        cuts = cuts
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_inline_probe_{probe}_pocket_cut"),
                PROBE_POCKET_D / 2.0,
                PROBE_Z + 10.0,
                24,
            )
            .translate(
                PROBE_CENTER.0 + centered_index(col, 4, 66.0),
                PROBE_CENTER.1 + centered_index(row, 3, 54.0),
                place_z(PROBE_Z),
            );
    }
    cuts
}

fn probe_cable_cross_channels() -> Part {
    let mut channels = Part::empty(format!("{OUTPUT_PREFIX}_probe_cable_cross_channels"));
    for row in 0..3 {
        channels = channels
            + centered_cube(
                format!("{OUTPUT_PREFIX}_probe_row_{row}_cable_channel_cut"),
                PROBE_X - 62.0,
                8.0,
                12.0,
            )
            .translate(
                PROBE_CENTER.0,
                PROBE_CENTER.1 + centered_index(row, 3, 54.0),
                deck_top_z() + PROBE_Z - 6.0,
            );
    }
    channels
}

fn probe_pocket_rims() -> Part {
    let mut rims = Part::empty(format!("{OUTPUT_PREFIX}_inline_probe_pocket_rims"));
    for probe in 0..INLINE_PROBE_POCKETS {
        let col = probe % 4;
        let row = probe / 4;
        let x = PROBE_CENTER.0 + centered_index(col, 4, 66.0);
        let y = PROBE_CENTER.1 + centered_index(row, 3, 54.0);
        let rim = centered_cylinder(
            format!("{OUTPUT_PREFIX}_inline_probe_{probe}_rim_outer"),
            PROBE_POCKET_D / 2.0 + 5.5,
            5.0,
            28,
        )
        .translate(x, y, deck_top_z() + PROBE_Z + 2.5)
            - centered_cylinder(
                format!("{OUTPUT_PREFIX}_inline_probe_{probe}_rim_inner"),
                PROBE_POCKET_D / 2.0,
                7.0,
                24,
            )
            .translate(x, y, deck_top_z() + PROBE_Z + 2.5);
        rims = rims + rim;
    }
    rims
}

fn reference_well_lands() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_probe_reference_well_lands"));
    for well in 0..PROBE_REFERENCE_WELLS {
        let land = centered_cylinder(
            format!("{OUTPUT_PREFIX}_probe_reference_well_{well}_outer_land"),
            17.0,
            9.0,
            32,
        )
        .translate(
            PROBE_CENTER.0 + centered_index(well, PROBE_REFERENCE_WELLS, 58.0),
            PROBE_CENTER.1 - PROBE_Y / 2.0 + 30.0,
            deck_top_z() + PROBE_Z + 4.5,
        ) - centered_cylinder(
            format!("{OUTPUT_PREFIX}_probe_reference_well_{well}_inner_cut"),
            8.0,
            11.0,
            24,
        )
        .translate(
            PROBE_CENTER.0 + centered_index(well, PROBE_REFERENCE_WELLS, 58.0),
            PROBE_CENTER.1 - PROBE_Y / 2.0 + 30.0,
            deck_top_z() + PROBE_Z + 4.5,
        );
        lands = lands + land;
    }
    lands
}

fn probe_lead_strain_relief_combs() -> Part {
    let rear = centered_cube(
        format!("{OUTPUT_PREFIX}_probe_rear_lead_strain_relief_comb"),
        PROBE_X - 64.0,
        16.0,
        16.0,
    )
    .translate(
        PROBE_CENTER.0,
        PROBE_CENTER.1 + PROBE_Y / 2.0 - 24.0,
        deck_top_z() + PROBE_Z + 8.0,
    );
    let front = centered_cube(
        format!("{OUTPUT_PREFIX}_probe_front_lead_strain_relief_comb"),
        PROBE_X - 64.0,
        16.0,
        16.0,
    )
    .translate(
        PROBE_CENTER.0,
        PROBE_CENTER.1 - PROBE_Y / 2.0 + 24.0,
        deck_top_z() + PROBE_Z + 8.0,
    );
    rear + front
}

fn stratification_sample_loop_witness() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_stratification_sample_loop_body"),
        STRAT_X,
        STRAT_Y,
        STRAT_Z,
    )
    .translate(STRAT_CENTER.0, STRAT_CENTER.1, place_z(STRAT_Z));

    body - sample_loop_cuts()
        + sample_loop_rims()
        + top_middle_bottom_level_lands()
        + recirculation_witness_route()
}

fn sample_loop_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_stratification_loop_cuts"));
    for level in 0..STRATIFICATION_LEVELS {
        for loop_index in 0..LOOPS_PER_LEVEL {
            cuts = cuts
                + centered_cylinder(
                    format!("{OUTPUT_PREFIX}_level_{level}_sample_loop_{loop_index}_cut"),
                    SAMPLE_LOOP_D / 2.0,
                    STRAT_Z + 8.0,
                    28,
                )
                .translate(
                    STRAT_CENTER.0 + centered_index(loop_index, LOOPS_PER_LEVEL, 68.0),
                    STRAT_CENTER.1 + centered_index(level, STRATIFICATION_LEVELS, 58.0),
                    place_z(STRAT_Z),
                );
        }
    }
    cuts
}

fn sample_loop_rims() -> Part {
    let mut rims = Part::empty(format!("{OUTPUT_PREFIX}_stratification_sample_loop_rims"));
    for level in 0..STRATIFICATION_LEVELS {
        for loop_index in 0..LOOPS_PER_LEVEL {
            let x = STRAT_CENTER.0 + centered_index(loop_index, LOOPS_PER_LEVEL, 68.0);
            let y = STRAT_CENTER.1 + centered_index(level, STRATIFICATION_LEVELS, 58.0);
            let rim = centered_cylinder(
                format!("{OUTPUT_PREFIX}_level_{level}_sample_loop_{loop_index}_outer_rim"),
                SAMPLE_LOOP_D / 2.0 + 5.0,
                5.0,
                28,
            )
            .translate(x, y, deck_top_z() + STRAT_Z + 2.5)
                - centered_cylinder(
                    format!(
                        "{OUTPUT_PREFIX}_level_{level}_sample_loop_{loop_index}_inner_clearance"
                    ),
                    SAMPLE_LOOP_D / 2.0,
                    7.0,
                    24,
                )
                .translate(x, y, deck_top_z() + STRAT_Z + 2.5);
            rims = rims + rim;
        }
    }
    rims
}

fn top_middle_bottom_level_lands() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_top_middle_bottom_level_lands"));
    for level in 0..STRATIFICATION_LEVELS {
        lands = lands
            + centered_cube(
                format!("{OUTPUT_PREFIX}_stratification_level_{level}_identity_land"),
                STRAT_X - 58.0,
                8.0,
                6.0,
            )
            .translate(
                STRAT_CENTER.0,
                STRAT_CENTER.1 + centered_index(level, STRATIFICATION_LEVELS, 58.0) + 24.0,
                deck_top_z() + STRAT_Z + 5.0,
            );
    }
    lands
}

fn recirculation_witness_route() -> Part {
    let inlet = centered_cube(
        format!("{OUTPUT_PREFIX}_stratification_inlet_witness_route"),
        28.0,
        STRAT_Y - 52.0,
        8.0,
    )
    .translate(
        STRAT_CENTER.0 - STRAT_X / 2.0 + 36.0,
        STRAT_CENTER.1,
        deck_top_z() + STRAT_Z + 4.0,
    );
    let outlet = centered_cube(
        format!("{OUTPUT_PREFIX}_stratification_outlet_witness_route"),
        28.0,
        STRAT_Y - 52.0,
        8.0,
    )
    .translate(
        STRAT_CENTER.0 + STRAT_X / 2.0 - 36.0,
        STRAT_CENTER.1,
        deck_top_z() + STRAT_Z + 4.0,
    );
    inlet + outlet
}

fn thermal_logger_custody_lands() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_thermal_logger_custody_body"),
        LOGGER_X,
        LOGGER_Y,
        LOGGER_Z,
    )
    .translate(LOGGER_CENTER.0, LOGGER_CENTER.1, place_z(LOGGER_Z));

    body - logger_slot_cuts()
        + logger_seal_witnesses()
        + custody_card_lands()
        + logger_chain_bridge_tabs()
}

fn logger_slot_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_thermal_logger_slot_cuts"));
    for logger in 0..THERMAL_LOGGER_LANDS {
        cuts = cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_thermal_logger_{logger}_custody_slot_cut"),
                54.0,
                28.0,
                10.0,
            )
            .translate(
                LOGGER_CENTER.0 + centered_index(logger % 4, 4, 78.0),
                LOGGER_CENTER.1 + centered_index(logger / 4, 2, 72.0),
                deck_top_z() + LOGGER_Z - 5.0,
            );
    }
    cuts
}

fn logger_seal_witnesses() -> Part {
    let mut seals = Part::empty(format!("{OUTPUT_PREFIX}_logger_tamper_seal_witnesses"));
    for seal in 0..LOGGER_SEAL_WITNESSES {
        seals = seals
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_thermal_logger_{seal}_seal_witness_puck"),
                10.0,
                6.0,
                28,
            )
            .translate(
                LOGGER_CENTER.0 + centered_index(seal % 4, 4, 78.0) + 32.0,
                LOGGER_CENTER.1 + centered_index(seal / 4, 2, 72.0),
                deck_top_z() + LOGGER_Z + 3.0,
            );
    }
    seals
}

fn custody_card_lands() -> Part {
    let mut cards = Part::empty(format!("{OUTPUT_PREFIX}_logger_custody_card_lands"));
    for card in 0..LOGGER_CARD_LANDS {
        cards = cards
            + centered_cube(
                format!("{OUTPUT_PREFIX}_custody_card_land_{card}"),
                74.0,
                22.0,
                6.0,
            )
            .translate(
                LOGGER_CENTER.0 + centered_index(card, LOGGER_CARD_LANDS, 88.0),
                LOGGER_CENTER.1 - LOGGER_Y / 2.0 + 26.0,
                deck_top_z() + LOGGER_Z + 3.0,
            );
    }
    cards
}

fn logger_chain_bridge_tabs() -> Part {
    let upper = centered_cube(
        format!("{OUTPUT_PREFIX}_logger_upper_chain_of_custody_bridge"),
        LOGGER_X - 62.0,
        9.0,
        8.0,
    )
    .translate(
        LOGGER_CENTER.0,
        LOGGER_CENTER.1 + LOGGER_Y / 2.0 - 26.0,
        deck_top_z() + LOGGER_Z + 4.0,
    );
    let lower = centered_cube(
        format!("{OUTPUT_PREFIX}_logger_lower_chain_of_custody_bridge"),
        LOGGER_X - 62.0,
        9.0,
        8.0,
    )
    .translate(
        LOGGER_CENTER.0,
        LOGGER_CENTER.1 - LOGGER_Y / 2.0 + 60.0,
        deck_top_z() + LOGGER_Z + 4.0,
    );
    upper + lower
}

fn condensation_wetness_wells() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_condensation_wetness_well_body"),
        WETNESS_X,
        WETNESS_Y,
        WETNESS_Z,
    )
    .translate(WETNESS_CENTER.0, WETNESS_CENTER.1, place_z(WETNESS_Z));

    body - wetness_well_cuts()
        + wetness_well_rims()
        + condensation_gutter_rails()
        + wetness_indicator_coupon_lands()
}

fn wetness_well_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_wetness_well_cuts"));
    for well in 0..WETNESS_WELLS {
        cuts = cuts
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_condensation_wetness_well_{well}_cut"),
                WETNESS_WELL_D / 2.0,
                WETNESS_Z + 8.0,
                28,
            )
            .translate(
                WETNESS_CENTER.0 + centered_index(well % 5, 5, 48.0),
                WETNESS_CENTER.1 + centered_index(well / 5, 2, 48.0),
                place_z(WETNESS_Z),
            );
    }
    cuts
}

fn wetness_well_rims() -> Part {
    let mut rims = Part::empty(format!("{OUTPUT_PREFIX}_wetness_well_rims"));
    for well in 0..WETNESS_WELLS {
        let x = WETNESS_CENTER.0 + centered_index(well % 5, 5, 48.0);
        let y = WETNESS_CENTER.1 + centered_index(well / 5, 2, 48.0);
        let rim = centered_cylinder(
            format!("{OUTPUT_PREFIX}_wetness_well_{well}_rim_outer"),
            WETNESS_WELL_D / 2.0 + 5.0,
            5.0,
            28,
        )
        .translate(x, y, deck_top_z() + WETNESS_Z + 2.5)
            - centered_cylinder(
                format!("{OUTPUT_PREFIX}_wetness_well_{well}_rim_inner"),
                WETNESS_WELL_D / 2.0,
                7.0,
                24,
            )
            .translate(x, y, deck_top_z() + WETNESS_Z + 2.5);
        rims = rims + rim;
    }
    rims
}

fn condensation_gutter_rails() -> Part {
    let mut gutters = Part::empty(format!("{OUTPUT_PREFIX}_condensation_gutter_rails"));
    for gutter in 0..CONDENSATION_GUTTERS {
        gutters = gutters
            + centered_cube(
                format!("{OUTPUT_PREFIX}_condensation_gutter_{gutter}_rail"),
                WETNESS_X - 44.0,
                8.0,
                8.0,
            )
            .translate(
                WETNESS_CENTER.0,
                WETNESS_CENTER.1 + centered_index(gutter, CONDENSATION_GUTTERS, 28.0),
                deck_top_z() + WETNESS_Z + 4.0,
            );
    }
    gutters
}

fn wetness_indicator_coupon_lands() -> Part {
    let left = centered_cube(
        format!("{OUTPUT_PREFIX}_left_wetness_indicator_coupon_land"),
        34.0,
        WETNESS_Y - 24.0,
        6.0,
    )
    .translate(
        WETNESS_CENTER.0 - WETNESS_X / 2.0 + 24.0,
        WETNESS_CENTER.1,
        deck_top_z() + WETNESS_Z + 3.0,
    );
    let right = centered_cube(
        format!("{OUTPUT_PREFIX}_right_wetness_indicator_coupon_land"),
        34.0,
        WETNESS_Y - 24.0,
        6.0,
    )
    .translate(
        WETNESS_CENTER.0 + WETNESS_X / 2.0 - 24.0,
        WETNESS_CENTER.1,
        deck_top_z() + WETNESS_Z + 3.0,
    );
    left + right
}

fn overtemp_undertemp_challenge_tokens() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_overtemp_undertemp_token_tray_body"),
        TOKEN_X,
        TOKEN_Y,
        TOKEN_Z,
    )
    .translate(TOKEN_CENTER.0, TOKEN_CENTER.1, place_z(TOKEN_Z));

    body - challenge_token_socket_cuts()
        + challenge_token_pucks("overtemp", OVERTEMP_TOKENS, 34.0)
        + challenge_token_pucks("undertemp", UNDERTEMP_TOKENS, -34.0)
        + token_custody_index_bar()
}

fn challenge_token_socket_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_challenge_token_socket_cuts"));
    for token in 0..OVERTEMP_TOKENS {
        cuts = cuts
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_overtemp_token_{token}_socket_cut"),
                TOKEN_DIAMETER / 2.0,
                TOKEN_Z + 6.0,
                28,
            )
            .translate(
                TOKEN_CENTER.0 + centered_index(token, OVERTEMP_TOKENS, 54.0),
                TOKEN_CENTER.1 + 34.0,
                place_z(TOKEN_Z),
            );
    }
    for token in 0..UNDERTEMP_TOKENS {
        cuts = cuts
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_undertemp_token_{token}_socket_cut"),
                TOKEN_DIAMETER / 2.0,
                TOKEN_Z + 6.0,
                28,
            )
            .translate(
                TOKEN_CENTER.0 + centered_index(token, UNDERTEMP_TOKENS, 54.0),
                TOKEN_CENTER.1 - 34.0,
                place_z(TOKEN_Z),
            );
    }
    cuts
}

fn challenge_token_pucks(kind: &str, count: usize, y_offset: f64) -> Part {
    let mut pucks = Part::empty(format!("{OUTPUT_PREFIX}_{kind}_challenge_token_pucks"));
    for token in 0..count {
        pucks = pucks
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_{kind}_challenge_token_{token}_puck"),
                TOKEN_DIAMETER / 2.0 - 2.0,
                7.0,
                28,
            )
            .translate(
                TOKEN_CENTER.0 + centered_index(token, count, 54.0),
                TOKEN_CENTER.1 + y_offset,
                deck_top_z() + TOKEN_Z + 3.5,
            )
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{kind}_challenge_token_{token}_orientation_key"),
                18.0,
                5.0,
                9.0,
            )
            .translate(
                TOKEN_CENTER.0 + centered_index(token, count, 54.0),
                TOKEN_CENTER.1 + y_offset + 14.0,
                deck_top_z() + TOKEN_Z + 4.5,
            );
    }
    pucks
}

fn token_custody_index_bar() -> Part {
    centered_cube(
        format!("{OUTPUT_PREFIX}_overtemp_undertemp_custody_index_bar"),
        TOKEN_X - 42.0,
        8.0,
        8.0,
    )
    .translate(TOKEN_CENTER.0, TOKEN_CENTER.1, deck_top_z() + TOKEN_Z + 4.0)
}

fn sterile_connector_handoff_bulkhead() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_sterile_connector_handoff_bulkhead_body"),
        BULKHEAD_X,
        BULKHEAD_Y,
        BULKHEAD_Z,
    )
    .translate(BULKHEAD_CENTER.0, BULKHEAD_CENTER.1, place_z(BULKHEAD_Z));

    body - connector_bores()
        + connector_gasket_lands()
        + cap_park_well_lands()
        + handoff_alignment_rails()
}

fn connector_bores() -> Part {
    let mut bores = Part::empty(format!("{OUTPUT_PREFIX}_connector_bulkhead_bores"));
    for connector in 0..STERILE_CONNECTORS {
        bores = bores
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_connector_{connector}_bulkhead_bore"),
                CONNECTOR_D / 2.0,
                BULKHEAD_Y + 18.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                BULKHEAD_CENTER.0 + centered_index(connector, STERILE_CONNECTORS, 42.0),
                BULKHEAD_CENTER.1,
                deck_top_z() + 36.0,
            );
    }
    bores
}

fn connector_gasket_lands() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_connector_gasket_lands"));
    for connector in 0..STERILE_CONNECTORS {
        lands = lands
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_connector_{connector}_front_gasket_land"),
                CONNECTOR_D / 2.0 + 8.0,
                6.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                BULKHEAD_CENTER.0 + centered_index(connector, STERILE_CONNECTORS, 42.0),
                BULKHEAD_CENTER.1 - BULKHEAD_Y / 2.0 - 3.0,
                deck_top_z() + 36.0,
            )
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_connector_{connector}_rear_gasket_land"),
                CONNECTOR_D / 2.0 + 8.0,
                6.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                BULKHEAD_CENTER.0 + centered_index(connector, STERILE_CONNECTORS, 42.0),
                BULKHEAD_CENTER.1 + BULKHEAD_Y / 2.0 + 3.0,
                deck_top_z() + 36.0,
            );
    }
    lands
}

fn cap_park_well_lands() -> Part {
    let mut wells = Part::empty(format!("{OUTPUT_PREFIX}_cap_park_well_lands"));
    for well in 0..CAP_PARK_WELLS {
        let cup = centered_cylinder(
            format!("{OUTPUT_PREFIX}_connector_cap_park_well_{well}_outer"),
            12.0,
            9.0,
            28,
        )
        .translate(
            BULKHEAD_CENTER.0 + centered_index(well, CAP_PARK_WELLS, 42.0),
            BULKHEAD_CENTER.1 + BULKHEAD_Y / 2.0 - 22.0,
            deck_top_z() + BULKHEAD_Z + 4.5,
        ) - centered_cylinder(
            format!("{OUTPUT_PREFIX}_connector_cap_park_well_{well}_inner"),
            7.0,
            11.0,
            24,
        )
        .translate(
            BULKHEAD_CENTER.0 + centered_index(well, CAP_PARK_WELLS, 42.0),
            BULKHEAD_CENTER.1 + BULKHEAD_Y / 2.0 - 22.0,
            deck_top_z() + BULKHEAD_Z + 4.5,
        );
        wells = wells + cup;
    }
    wells
}

fn handoff_alignment_rails() -> Part {
    let lower = centered_cube(
        format!("{OUTPUT_PREFIX}_bulkhead_lower_handoff_alignment_rail"),
        BULKHEAD_X - 42.0,
        8.0,
        12.0,
    )
    .translate(
        BULKHEAD_CENTER.0,
        BULKHEAD_CENTER.1 - BULKHEAD_Y / 2.0 + 18.0,
        deck_top_z() + BULKHEAD_Z + 6.0,
    );
    let upper = centered_cube(
        format!("{OUTPUT_PREFIX}_bulkhead_upper_handoff_alignment_rail"),
        BULKHEAD_X - 42.0,
        8.0,
        12.0,
    )
    .translate(
        BULKHEAD_CENTER.0,
        BULKHEAD_CENTER.1 + BULKHEAD_Y / 2.0 - 48.0,
        deck_top_z() + BULKHEAD_Z + 6.0,
    );
    lower + upper
}

fn release_hold_reject_lanes() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_release_hold_reject_lane_body"),
        DECISION_X,
        DECISION_Y,
        DECISION_Z,
    )
    .translate(DECISION_CENTER.0, DECISION_CENTER.1, place_z(DECISION_Z));

    body - decision_token_socket_cuts() + decision_lane_rails() + decision_status_pucks()
}

fn decision_token_socket_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_decision_token_socket_cuts"));
    for lane in 0..DECISION_LANES {
        for token in 0..DECISION_TOKENS_PER_LANE {
            cuts = cuts
                + centered_cube(
                    format!("{OUTPUT_PREFIX}_decision_lane_{lane}_token_{token}_socket_cut"),
                    34.0,
                    20.0,
                    DECISION_Z + 4.0,
                )
                .translate(
                    DECISION_CENTER.0 + centered_index(token, DECISION_TOKENS_PER_LANE, 62.0),
                    DECISION_CENTER.1 + centered_index(lane, DECISION_LANES, 38.0),
                    place_z(DECISION_Z),
                );
        }
    }
    cuts
}

fn decision_lane_rails() -> Part {
    let mut rails = Part::empty(format!("{OUTPUT_PREFIX}_decision_lane_rails"));
    for lane in 0..DECISION_LANES {
        rails = rails
            + centered_cube(
                format!("{OUTPUT_PREFIX}_decision_lane_{lane}_left_route_rail"),
                DECISION_X - 38.0,
                6.0,
                8.0,
            )
            .translate(
                DECISION_CENTER.0,
                DECISION_CENTER.1 + centered_index(lane, DECISION_LANES, 38.0) - 16.0,
                deck_top_z() + DECISION_Z + 4.0,
            )
            + centered_cube(
                format!("{OUTPUT_PREFIX}_decision_lane_{lane}_right_route_rail"),
                DECISION_X - 38.0,
                6.0,
                8.0,
            )
            .translate(
                DECISION_CENTER.0,
                DECISION_CENTER.1 + centered_index(lane, DECISION_LANES, 38.0) + 16.0,
                deck_top_z() + DECISION_Z + 4.0,
            );
    }
    rails
}

fn decision_status_pucks() -> Part {
    let mut pucks = Part::empty(format!("{OUTPUT_PREFIX}_release_hold_reject_status_pucks"));
    for lane in 0..DECISION_LANES {
        pucks = pucks
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_decision_lane_{lane}_status_puck"),
                11.0,
                7.0,
                28,
            )
            .translate(
                DECISION_CENTER.0 - DECISION_X / 2.0 + 28.0,
                DECISION_CENTER.1 + centered_index(lane, DECISION_LANES, 38.0),
                deck_top_z() + DECISION_Z + 3.5,
            );
    }
    pucks
}

fn evidence_camera_bridge() -> Part {
    let left_post = centered_cube(
        format!("{OUTPUT_PREFIX}_evidence_camera_bridge_left_post"),
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_CLEARANCE_Z,
    )
    .translate(
        -BRIDGE_SPAN_X / 2.0,
        BRIDGE_CENTER.1,
        deck_top_z() + BRIDGE_CLEARANCE_Z / 2.0,
    );
    let right_post = centered_cube(
        format!("{OUTPUT_PREFIX}_evidence_camera_bridge_right_post"),
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_CLEARANCE_Z,
    )
    .translate(
        BRIDGE_SPAN_X / 2.0,
        BRIDGE_CENTER.1,
        deck_top_z() + BRIDGE_CLEARANCE_Z / 2.0,
    );
    let beam = centered_cube(
        format!("{OUTPUT_PREFIX}_evidence_camera_bridge_beam"),
        BRIDGE_SPAN_X + BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(
        BRIDGE_CENTER.0,
        BRIDGE_CENTER.1,
        deck_top_z() + BRIDGE_CLEARANCE_Z + BRIDGE_BEAM_Z / 2.0,
    );

    left_post + right_post + beam + camera_pods() + bridge_light_bars()
}

fn camera_pods() -> Part {
    let mut pods = Part::empty(format!("{OUTPUT_PREFIX}_evidence_camera_pods"));
    for pod in 0..CAMERA_PODS {
        pods =
            pods + centered_cube(
                format!("{OUTPUT_PREFIX}_evidence_camera_pod_{pod}"),
                74.0,
                42.0,
                28.0,
            )
            .translate(
                BRIDGE_CENTER.0 + centered_index(pod, CAMERA_PODS, 260.0),
                BRIDGE_CENTER.1 - 6.0,
                deck_top_z() + BRIDGE_CLEARANCE_Z + BRIDGE_BEAM_Z + 14.0,
            ) + centered_cylinder(
                format!("{OUTPUT_PREFIX}_evidence_camera_lens_shadow_{pod}"),
                12.0,
                8.0,
                28,
            )
            .translate(
                BRIDGE_CENTER.0 + centered_index(pod, CAMERA_PODS, 260.0),
                BRIDGE_CENTER.1 - 32.0,
                deck_top_z() + BRIDGE_CLEARANCE_Z + BRIDGE_BEAM_Z + 14.0,
            );
    }
    pods
}

fn bridge_light_bars() -> Part {
    let mut bars = Part::empty(format!("{OUTPUT_PREFIX}_evidence_bridge_light_bars"));
    for bar in 0..LIGHT_BARS {
        bars = bars
            + centered_cube(
                format!("{OUTPUT_PREFIX}_evidence_light_bar_{bar}"),
                390.0,
                12.0,
                14.0,
            )
            .translate(
                BRIDGE_CENTER.0 + centered_index(bar, LIGHT_BARS, 500.0),
                BRIDGE_CENTER.1 + 28.0,
                deck_top_z() + BRIDGE_CLEARANCE_Z + BRIDGE_BEAM_Z + 7.0,
            );
    }
    bars
}

fn robot_service_keepouts() -> Part {
    let front = keepout_bar(
        "front_robot_keepout_gauge",
        STATION_X - 260.0,
        10.0,
        7.0,
        (0.0, -STATION_Y / 2.0 - FRONT_ROBOT_CLEARANCE / 2.0, 3.5),
    );
    let rear = keepout_bar(
        "rear_service_keepout_gauge",
        STATION_X - 260.0,
        10.0,
        7.0,
        (0.0, STATION_Y / 2.0 + REAR_SERVICE_CLEARANCE / 2.0, 3.5),
    );
    let left = keepout_bar(
        "left_bag_service_keepout_gauge",
        10.0,
        STATION_Y - 170.0,
        7.0,
        (
            -STATION_X / 2.0 - LEFT_BAG_SERVICE_CLEARANCE / 2.0,
            0.0,
            3.5,
        ),
    );
    let right = keepout_bar(
        "right_connector_service_keepout_gauge",
        10.0,
        STATION_Y - 170.0,
        7.0,
        (
            STATION_X / 2.0 + RIGHT_CONNECTOR_SERVICE_CLEARANCE / 2.0,
            0.0,
            3.5,
        ),
    );
    let top = centered_cube(
        format!("{OUTPUT_PREFIX}_top_bag_lift_keepout_gauge"),
        260.0,
        22.0,
        12.0,
    )
    .translate(0.0, 0.0, deck_top_z() + TOP_BAG_LIFT_CLEARANCE);

    front + rear + left + right + top
}

fn keepout_bar(name: &str, x: f64, y: f64, z: f64, position: (f64, f64, f64)) -> Part {
    centered_cube(format!("{OUTPUT_PREFIX}_{name}"), x, y, z)
        .translate(position.0, position.1, position.2)
}

fn rectangular_frame(name: &str, x: f64, y: f64, z: f64, rail: f64) -> Part {
    centered_cube(format!("{name}_outer"), x, y, z)
        - centered_cube(
            format!("{name}_inner_cut"),
            x - rail * 2.0,
            y - rail * 2.0,
            z + 1.0,
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bag_temp_and_custody_counts_are_explicit() {
        assert_eq!(BAG_NEST_COUNT, 2);
        assert_eq!(DRY_BLOCK_COUNT, 3);
        assert_eq!(INLINE_PROBE_POCKETS, 12);
        assert_eq!(THERMAL_LOGGER_LANDS, 8);
        assert_eq!(LOGGER_SEAL_WITNESSES, THERMAL_LOGGER_LANDS);
        assert_eq!(STERILE_CONNECTORS, 6);
        assert_eq!(DECISION_LANES, 3);
        assert_eq!(DECISION_TOKENS, 9);
    }

    #[test]
    fn output_manifest_is_stable_and_specific() {
        assert_eq!(OUTPUTS.len(), 13);
        assert!(OUTPUTS.iter().all(|path| path.starts_with("output/")));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
        assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
        assert_eq!(
            OUTPUTS[12],
            "output/closed_media_bag_temp_ramp_thaw_equilibration_fault_station_assembly.stl"
        );

        let mut sorted = OUTPUTS.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), OUTPUTS.len());
    }

    #[test]
    fn station_bounds_and_keepouts_are_coherent() {
        assert_design_constraints();
        assert!(STATION_X <= 1600.0);
        assert!(STATION_Y <= 1000.0);
        assert!(FRONT_ROBOT_CLEARANCE >= 420.0);
        assert!(REAR_SERVICE_CLEARANCE >= 260.0);
        assert!(TOP_BAG_LIFT_CLEARANCE > BRIDGE_CLEARANCE_Z + BRIDGE_BEAM_Z);
    }

    #[test]
    fn feature_coverage_matches_requested_station() {
        for feature in [
            "base_leak_tray_deck",
            "chilled_bag_nest",
            "warm_bag_nest",
            "dry_thaw_equilibration_block_surrogate",
            "inline_temperature_probe_pockets",
            "stratification_sample_loop_witness",
            "thermal_logger_custody_lands",
            "condensation_wetness_wells",
            "overtemp_undertemp_challenge_tokens",
            "sterile_connector_handoff_bulkhead",
            "release_hold_reject_lanes",
            "robot_service_keepouts",
        ] {
            assert!(
                REQUIRED_FEATURES.contains(&feature),
                "missing required feature marker {feature}"
            );
        }
    }

    #[test]
    fn no_process_or_health_claims_are_encoded() {
        let forbidden = [
            "diagnose",
            "therapy",
            "therapeutic",
            "patient",
            "treat",
            "cure",
            "guarantee",
            "certified sterile",
            "validated sterile",
        ];
        for claim in CLAIM_TEXTS {
            let lower = claim.to_ascii_lowercase();
            for term in forbidden {
                assert!(
                    !lower.contains(term),
                    "claim text must stay fixture-only: {claim}"
                );
            }
        }
    }
}
