use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed cell-suspension clump/filter and viability-loss sentinel station.
//
// Intent:
// - Challenge a closed cell-suspension stream immediately before scaled
//   tissue-chip seeding so clumps, filter loading, pressure drop, trapped
//   bubbles, and viability-loss retain samples are visible before release.
// - Keep the inlet loop sealed while exposing a gentle sieve coupon ladder,
//   optical clump witness windows, pressure-drop taps, backflush recovery,
//   live/dead retain pockets, bubble/dead-volume windows, waste/reject split,
//   barcode custody lands, disposition gates, camera evidence, and robot
//   service datums as deterministic CAD features.
// - Model envelopes and interfaces only. Biological acceptance thresholds,
//   sieve media, imaging algorithms, and release logic remain quality-system
//   data outside this architecture CAD.

const OUTPUT_PREFIX: &str = "closed_cell_suspension_clump_filter_viability_loss_sentinel_station";

const OUTPUTS: [&str; 12] = [
    "output/closed_cell_suspension_clump_filter_viability_loss_sentinel_station_leak_tray_deck.stl",
    "output/closed_cell_suspension_clump_filter_viability_loss_sentinel_station_sealed_sample_loop_inlet_manifold.stl",
    "output/closed_cell_suspension_clump_filter_viability_loss_sentinel_station_gentle_sieve_filter_coupon_ladder.stl",
    "output/closed_cell_suspension_clump_filter_viability_loss_sentinel_station_optical_clump_witness_windows.stl",
    "output/closed_cell_suspension_clump_filter_viability_loss_sentinel_station_pressure_drop_tap_ladder.stl",
    "output/closed_cell_suspension_clump_filter_viability_loss_sentinel_station_backflush_recovery_route.stl",
    "output/closed_cell_suspension_clump_filter_viability_loss_sentinel_station_live_dead_retain_sample_pockets.stl",
    "output/closed_cell_suspension_clump_filter_viability_loss_sentinel_station_bubble_dead_volume_window_bank.stl",
    "output/closed_cell_suspension_clump_filter_viability_loss_sentinel_station_waste_reject_split_manifold.stl",
    "output/closed_cell_suspension_clump_filter_viability_loss_sentinel_station_barcode_custody_release_gate_panel.stl",
    "output/closed_cell_suspension_clump_filter_viability_loss_sentinel_station_camera_evidence_bridge_robot_datums.stl",
    "output/closed_cell_suspension_clump_filter_viability_loss_sentinel_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 16] = [
    "sealed_sample_loop_inlet",
    "gentle_inline_sieve_filter_coupon_ladder",
    "optical_clump_witness_windows",
    "pressure_drop_tap_ladder",
    "backflush_recovery_route",
    "live_dead_retain_sample_pockets",
    "bubble_dead_volume_windows",
    "waste_reject_split",
    "barcode_custody_lands",
    "release_hold_reject_gates",
    "camera_evidence_bridge",
    "robotic_service_datums",
    "low_shear_cell_suspension_route",
    "recoverable_clump_retain_pockets",
    "closed_loop_interlock_valves",
    "pre_seed_disposition_evidence",
];

const DISPOSITION_LANES: [&str; 3] = ["release", "hold", "reject"];
const FILTER_STAGE_LABELS: [&str; 5] = ["coarse", "guard", "medium", "fine", "sentinel"];

const DECK_X: f64 = 1460.0;
const DECK_Y: f64 = 860.0;
const DECK_Z: f64 = 22.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 40.0;
const SOCKET_DEPTH: f64 = 5.0;
const SUMP_DEPTH: f64 = 7.0;
const MOUNT_HOLE_D: f64 = 6.8;
const LEAK_WITNESS_WELLS: usize = 14;
const DECK_DATUM_TARGETS: usize = 4;

const LOOP_CENTER: (f64, f64) = (-470.0, 230.0);
const LOOP_X: f64 = 410.0;
const LOOP_Y: f64 = 230.0;
const LOOP_Z: f64 = 64.0;
const SAMPLE_LOOP_INLET_PORTS: usize = 2;
const SAMPLE_LOOP_RETURN_PORTS: usize = 2;
const SAMPLE_BRANCH_PORTS: usize = 4;
const LOOP_INTERLOCK_VALVES: usize = 8;
const LOOP_PRESSURE_SEAL_TAPS: usize = 4;
const STERILE_CAP_LANDS: usize = 4;
const LOOP_TUBE_D: f64 = 7.2;
const LOOP_BORE_D: f64 = 5.0;

const FILTER_CENTER: (f64, f64) = (0.0, 230.0);
const FILTER_X: f64 = 440.0;
const FILTER_Y: f64 = 230.0;
const FILTER_Z: f64 = 56.0;
const SIEVE_STAGES: usize = FILTER_STAGE_LABELS.len();
const COUPONS_PER_STAGE: usize = 2;
const FILTER_COUPONS: usize = SIEVE_STAGES * COUPONS_PER_STAGE;
const SIEVE_COUPON_X: f64 = 58.0;
const SIEVE_COUPON_Y: f64 = 68.0;
const SIEVE_STAGE_PITCH_X: f64 = 74.0;
const SIEVE_REPLICATE_PITCH_Y: f64 = 74.0;
const SIEVE_MESH_RIBS_PER_COUPON: usize = 4;
const LOW_SHEAR_BYPASS_CHANNELS: usize = 2;

const OPTICAL_CENTER: (f64, f64) = (470.0, 230.0);
const OPTICAL_X: f64 = 360.0;
const OPTICAL_Y: f64 = 230.0;
const OPTICAL_Z: f64 = 60.0;
const OPTICAL_CLUMP_WINDOWS: usize = 6;
const CLUMP_SIZE_BANDS: usize = 5;
const OPTICAL_LIGHT_BARS: usize = 2;
const DARK_REFERENCE_SHUTTERS: usize = 2;
const WINDOW_PATH_LENGTH_MM: f64 = 8.0;

const PRESSURE_CENTER: (f64, f64) = (-470.0, -70.0);
const PRESSURE_X: f64 = 410.0;
const PRESSURE_Y: f64 = 220.0;
const PRESSURE_Z: f64 = 58.0;
const PRESSURE_TAP_POINTS: usize = SIEVE_STAGES + 1;
const PRESSURE_TAP_PAIRS: usize = PRESSURE_TAP_POINTS;
const DP_REFERENCE_RESTRICTORS: usize = SIEVE_STAGES;
const PRESSURE_SENSOR_PADS: usize = PRESSURE_TAP_POINTS;
const PRESSURE_TUBE_D: f64 = 5.6;

const BACKFLUSH_CENTER: (f64, f64) = (0.0, -70.0);
const BACKFLUSH_X: f64 = 420.0;
const BACKFLUSH_Y: f64 = 220.0;
const BACKFLUSH_Z: f64 = 60.0;
const BACKFLUSH_BRANCHES: usize = SIEVE_STAGES;
const RECOVERY_VIAL_NESTS: usize = SIEVE_STAGES;
const BACKFLUSH_SELECTOR_VALVES: usize = SIEVE_STAGES + 1;
const RECOVERY_CHECK_VALVES: usize = SIEVE_STAGES;
const RECOVERY_ROUTE_SEGMENTS: usize = 8;

const RETAIN_CENTER: (f64, f64) = (470.0, -70.0);
const RETAIN_X: f64 = 360.0;
const RETAIN_Y: f64 = 220.0;
const RETAIN_Z: f64 = 54.0;
const LIVE_DEAD_RETAIN_PAIRS: usize = 4;
const RETAIN_SAMPLE_POCKETS: usize = LIVE_DEAD_RETAIN_PAIRS * 2;
const LIVE_DEAD_REAGENT_GUARDS: usize = 2;
const RETAIN_SEPTUM_CAPS: usize = RETAIN_SAMPLE_POCKETS;
const RETAIN_CHILLED_LANDS: usize = 4;

const BUBBLE_CENTER: (f64, f64) = (-470.0, -330.0);
const BUBBLE_X: f64 = 410.0;
const BUBBLE_Y: f64 = 120.0;
const BUBBLE_Z: f64 = 44.0;
const BUBBLE_WINDOWS: usize = 4;
const DEAD_VOLUME_WINDOWS: usize = 4;
const AIR_GAP_SENTINELS: usize = 3;
const DEAD_VOLUME_TICK_MARKS: usize = 9;

const WASTE_CENTER: (f64, f64) = (0.0, -330.0);
const WASTE_X: f64 = 420.0;
const WASTE_Y: f64 = 120.0;
const WASTE_Z: f64 = 64.0;
const WASTE_REJECT_LANES: usize = 2;
const SPLIT_DIVERTER_VALVES: usize = 4;
const WASTE_BOTTLE_NESTS: usize = 2;
const REJECT_BOTTLE_NESTS: usize = 2;
const QUARANTINE_LOCKS: usize = 3;

const CUSTODY_CENTER: (f64, f64) = (470.0, -330.0);
const CUSTODY_X: f64 = 360.0;
const CUSTODY_Y: f64 = 120.0;
const CUSTODY_Z: f64 = 58.0;
const BARCODE_LANDS: usize = 12;
const RFID_LANDS: usize = 4;
const CUSTODY_SEAL_LANDS: usize = 6;
const GATE_SLOTS_PER_LANE: usize = 4;
const GATE_SOLENOIDS: usize = DISPOSITION_LANES.len() * 2;

const CAMERA_BRIDGE_X: f64 = 1370.0;
const CAMERA_BRIDGE_Y: f64 = 60.0;
const CAMERA_BRIDGE_Z: f64 = 150.0;
const CAMERA_PODS: usize = 3;
const EVIDENCE_LIGHT_BARS: usize = 2;
const ROBOT_SERVICE_DATUMS: usize = 6;
const ROBOT_SERVICE_KEEP_OUTS: usize = 5;
const CAMERA_CLEARANCE_Z: f64 = 180.0;
const FRONT_ROBOT_CLEARANCE: f64 = 320.0;
const REAR_TUBE_SERVICE_CLEARANCE: f64 = 210.0;

const ROUTE_Z: f64 = DECK_Z + 84.0;
const ROUTE_TUBE_D: f64 = 7.2;
const ROUTE_SEGMENTS: usize = 12;
const ROUTE_DIRECTION_MARKERS: usize = 10;

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
        self.center.0.abs() + self.x / 2.0 <= DECK_X / 2.0 - RIM_W - 12.0
            && self.center.1.abs() + self.y / 2.0 <= DECK_Y / 2.0 - RIM_W - 12.0
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

    let inlet = sealed_sample_loop_inlet_manifold();
    write_part(OUTPUTS[1], &inlet);

    let sieve = gentle_sieve_filter_coupon_ladder();
    write_part(OUTPUTS[2], &sieve);

    let optical = optical_clump_witness_windows();
    write_part(OUTPUTS[3], &optical);

    let pressure = pressure_drop_tap_ladder();
    write_part(OUTPUTS[4], &pressure);

    let backflush = backflush_recovery_route();
    write_part(OUTPUTS[5], &backflush);

    let retain = live_dead_retain_sample_pockets();
    write_part(OUTPUTS[6], &retain);

    let bubbles = bubble_dead_volume_window_bank();
    write_part(OUTPUTS[7], &bubbles);

    let waste = waste_reject_split_manifold();
    write_part(OUTPUTS[8], &waste);

    let custody = barcode_custody_release_gate_panel();
    write_part(OUTPUTS[9], &custody);

    let bridge = camera_evidence_bridge_robot_datums();
    write_part(OUTPUTS[10], &bridge);

    let assembly = station_assembly();
    write_part(OUTPUTS[11], &assembly);

    println!();
    println!("Closed cell-suspension clump/filter viability-loss sentinel station:");
    println!(
        "  Footprint: {DECK_X:.0}mm x {DECK_Y:.0}mm contained deck with {LEAK_WITNESS_WELLS} leak witness wells and {DECK_DATUM_TARGETS} deck datums"
    );
    println!(
        "  Closed inlet loop: {SAMPLE_LOOP_INLET_PORTS} sealed inlets, {SAMPLE_LOOP_RETURN_PORTS} returns, {LOOP_INTERLOCK_VALVES} interlock valves, {SAMPLE_BRANCH_PORTS} sample branches"
    );
    println!(
        "  Filter challenge: {FILTER_COUPONS} coupons across {SIEVE_STAGES} gentle stages, {PRESSURE_TAP_PAIRS} pressure tap pairs, {BACKFLUSH_BRANCHES} backflush branches"
    );
    println!(
        "  Sentinel evidence: {OPTICAL_CLUMP_WINDOWS} clump windows, {BUBBLE_WINDOWS} bubble windows, {DEAD_VOLUME_WINDOWS} dead-volume windows, {RETAIN_SAMPLE_POCKETS} live/dead retain pockets"
    );
    println!(
        "  Custody/disposition: {BARCODE_LANDS} barcode lands, {RFID_LANDS} RFID lands, {} gates ({}) and {WASTE_REJECT_LANES} waste/reject split lanes",
        DISPOSITION_LANES.len(),
        DISPOSITION_LANES.join(", ")
    );
    println!(
        "  Evidence bridge: {CAMERA_PODS} camera pods, {EVIDENCE_LIGHT_BARS} light bars, {ROBOT_SERVICE_DATUMS} robot service datums, {ROBOT_SERVICE_KEEP_OUTS} keepout rails"
    );
}

fn write_part(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    leak_tray_deck()
        + sealed_sample_loop_inlet_manifold().translate(
            LOOP_CENTER.0,
            LOOP_CENTER.1,
            on_deck_z(LOOP_Z),
        )
        + gentle_sieve_filter_coupon_ladder().translate(
            FILTER_CENTER.0,
            FILTER_CENTER.1,
            on_deck_z(FILTER_Z),
        )
        + optical_clump_witness_windows().translate(
            OPTICAL_CENTER.0,
            OPTICAL_CENTER.1,
            on_deck_z(OPTICAL_Z),
        )
        + pressure_drop_tap_ladder().translate(
            PRESSURE_CENTER.0,
            PRESSURE_CENTER.1,
            on_deck_z(PRESSURE_Z),
        )
        + backflush_recovery_route().translate(
            BACKFLUSH_CENTER.0,
            BACKFLUSH_CENTER.1,
            on_deck_z(BACKFLUSH_Z),
        )
        + live_dead_retain_sample_pockets().translate(
            RETAIN_CENTER.0,
            RETAIN_CENTER.1,
            on_deck_z(RETAIN_Z),
        )
        + bubble_dead_volume_window_bank().translate(
            BUBBLE_CENTER.0,
            BUBBLE_CENTER.1,
            on_deck_z(BUBBLE_Z),
        )
        + waste_reject_split_manifold().translate(
            WASTE_CENTER.0,
            WASTE_CENTER.1,
            on_deck_z(WASTE_Z),
        )
        + barcode_custody_release_gate_panel().translate(
            CUSTODY_CENTER.0,
            CUSTODY_CENTER.1,
            on_deck_z(CUSTODY_Z),
        )
        + sample_flow_route_harness()
        + camera_evidence_bridge_robot_datums().translate(0.0, 0.0, DECK_Z + CAMERA_BRIDGE_Z / 2.0)
}

fn on_deck_z(height: f64) -> f64 {
    DECK_Z + height / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn module_specs() -> [ModuleSpec; 9] {
    [
        ModuleSpec {
            name: "sealed_sample_loop_inlet_manifold",
            center: LOOP_CENTER,
            x: LOOP_X,
            y: LOOP_Y,
            z: LOOP_Z,
        },
        ModuleSpec {
            name: "gentle_sieve_filter_coupon_ladder",
            center: FILTER_CENTER,
            x: FILTER_X,
            y: FILTER_Y,
            z: FILTER_Z,
        },
        ModuleSpec {
            name: "optical_clump_witness_windows",
            center: OPTICAL_CENTER,
            x: OPTICAL_X,
            y: OPTICAL_Y,
            z: OPTICAL_Z,
        },
        ModuleSpec {
            name: "pressure_drop_tap_ladder",
            center: PRESSURE_CENTER,
            x: PRESSURE_X,
            y: PRESSURE_Y,
            z: PRESSURE_Z,
        },
        ModuleSpec {
            name: "backflush_recovery_route",
            center: BACKFLUSH_CENTER,
            x: BACKFLUSH_X,
            y: BACKFLUSH_Y,
            z: BACKFLUSH_Z,
        },
        ModuleSpec {
            name: "live_dead_retain_sample_pockets",
            center: RETAIN_CENTER,
            x: RETAIN_X,
            y: RETAIN_Y,
            z: RETAIN_Z,
        },
        ModuleSpec {
            name: "bubble_dead_volume_window_bank",
            center: BUBBLE_CENTER,
            x: BUBBLE_X,
            y: BUBBLE_Y,
            z: BUBBLE_Z,
        },
        ModuleSpec {
            name: "waste_reject_split_manifold",
            center: WASTE_CENTER,
            x: WASTE_X,
            y: WASTE_Y,
            z: WASTE_Z,
        },
        ModuleSpec {
            name: "barcode_custody_release_gate_panel",
            center: CUSTODY_CENTER,
            x: CUSTODY_X,
            y: CUSTODY_Y,
            z: CUSTODY_Z,
        },
    ]
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 12);
    assert_eq!(
        STERILE_CAP_LANDS,
        SAMPLE_LOOP_INLET_PORTS + SAMPLE_LOOP_RETURN_PORTS
    );
    assert_eq!(FILTER_STAGE_LABELS.len(), SIEVE_STAGES);
    assert_eq!(FILTER_COUPONS, SIEVE_STAGES * COUPONS_PER_STAGE);
    assert_eq!(PRESSURE_TAP_POINTS, SIEVE_STAGES + 1);
    assert_eq!(PRESSURE_TAP_PAIRS, PRESSURE_TAP_POINTS);
    assert_eq!(PRESSURE_SENSOR_PADS, PRESSURE_TAP_POINTS);
    assert_eq!(BACKFLUSH_BRANCHES, SIEVE_STAGES);
    assert_eq!(RECOVERY_VIAL_NESTS, SIEVE_STAGES);
    assert_eq!(RECOVERY_CHECK_VALVES, SIEVE_STAGES);
    assert_eq!(RECOVERY_ROUTE_SEGMENTS, 8);
    assert_eq!(RETAIN_SAMPLE_POCKETS, LIVE_DEAD_RETAIN_PAIRS * 2);
    assert_eq!(RETAIN_SEPTUM_CAPS, RETAIN_SAMPLE_POCKETS);
    assert_eq!(WASTE_REJECT_LANES, 2);
    assert_eq!(GATE_SOLENOIDS, DISPOSITION_LANES.len() * 2);
    assert!(WINDOW_PATH_LENGTH_MM >= 8.0);
    assert!(LOOP_TUBE_D > LOOP_BORE_D);
    assert!(ROUTE_SEGMENTS >= 12);
    assert!(CAMERA_CLEARANCE_Z >= 175.0);
    assert!(FRONT_ROBOT_CLEARANCE >= 300.0);
    assert!(REAR_TUBE_SERVICE_CLEARANCE >= 200.0);

    let modules = module_specs();
    for module in modules {
        assert!(
            module.fits_on_deck(),
            "{} exceeds deck envelope",
            module.name
        );
        assert!(module.z > 0.0, "{} has non-positive height", module.name);
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

fn leak_tray_deck() -> Part {
    let floor = centered_cube(
        format!("{OUTPUT_PREFIX}_deck_floor"),
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let sump = centered_cube(
        format!("{OUTPUT_PREFIX}_shallow_sentinel_sump_cut"),
        DECK_X - 150.0,
        DECK_Y - 148.0,
        SUMP_DEPTH + 0.8,
    )
    .translate(0.0, -6.0, DECK_Z - SUMP_DEPTH / 2.0 + 0.4);
    let front_drain = centered_cube(
        format!("{OUTPUT_PREFIX}_front_waste_low_point_slot_cut"),
        DECK_X - 260.0,
        14.0,
        SUMP_DEPTH + 1.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 70.0, DECK_Z - SUMP_DEPTH / 2.0);

    floor - sump - front_drain - module_socket_cuts() - deck_mount_slots()
        + perimeter_rims()
        + deck_zone_dividers()
        + module_floor_markers()
        + deck_fiducial_targets()
        + leak_witness_wells()
}

fn module_socket_cuts() -> Part {
    let mut sockets = Part::empty(format!("{OUTPUT_PREFIX}_module_socket_cuts"));
    for module in module_specs() {
        sockets = sockets
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{}_registration_socket_cut", module.name),
                module.x + 8.0,
                module.y + 8.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(
                module.center.0,
                module.center.1,
                DECK_Z - SOCKET_DEPTH / 2.0 + 0.2,
            );
    }
    sockets
}

fn deck_mount_slots() -> Part {
    let mut slots = Part::empty(format!("{OUTPUT_PREFIX}_deck_mount_slots"));
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
        let hole = centered_cylinder(
            format!("{OUTPUT_PREFIX}_m6_mount_clearance_{i}"),
            MOUNT_HOLE_D / 2.0,
            DECK_Z + 4.0,
            28,
        )
        .translate(*x, *y, DECK_Z / 2.0);
        let slot = centered_cube(
            format!("{OUTPUT_PREFIX}_m6_mount_slot_relief_{i}"),
            28.0,
            7.2,
            DECK_Z + 4.0,
        )
        .translate(*x, *y, DECK_Z / 2.0);
        slots = slots + hole + slot;
    }
    slots
}

fn perimeter_rims() -> Part {
    let left = centered_cube(
        format!("{OUTPUT_PREFIX}_left_containment_rim"),
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube(
        format!("{OUTPUT_PREFIX}_right_containment_rim"),
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        format!("{OUTPUT_PREFIX}_rear_containment_rim"),
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let front_low_lip = centered_cube(
        format!("{OUTPUT_PREFIX}_front_low_waste_capture_lip"),
        DECK_X - 180.0,
        14.0,
        24.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 28.0, DECK_Z + 12.0);

    left + right + rear + front_low_lip
}

fn deck_zone_dividers() -> Part {
    let inlet_to_test = centered_cube(
        format!("{OUTPUT_PREFIX}_inlet_filter_optical_zone_divider"),
        DECK_X - 170.0,
        10.0,
        28.0,
    )
    .translate(0.0, 82.0, DECK_Z + 14.0);
    let test_to_disposition = centered_cube(
        format!("{OUTPUT_PREFIX}_pressure_backflush_retain_zone_divider"),
        DECK_X - 198.0,
        10.0,
        28.0,
    )
    .translate(0.0, -224.0, DECK_Z + 14.0);
    let inlet_filter_baffle = centered_cube(
        format!("{OUTPUT_PREFIX}_inlet_to_sieve_splash_baffle"),
        10.0,
        238.0,
        28.0,
    )
    .translate(-235.0, 230.0, DECK_Z + 14.0);
    let filter_optical_baffle = centered_cube(
        format!("{OUTPUT_PREFIX}_sieve_to_optical_light_baffle"),
        10.0,
        238.0,
        30.0,
    )
    .translate(235.0, 230.0, DECK_Z + 15.0);
    let pressure_backflush_baffle = centered_cube(
        format!("{OUTPUT_PREFIX}_pressure_to_backflush_wet_baffle"),
        10.0,
        226.0,
        28.0,
    )
    .translate(-235.0, -70.0, DECK_Z + 14.0);
    let backflush_retain_baffle = centered_cube(
        format!("{OUTPUT_PREFIX}_backflush_to_retain_reagent_baffle"),
        10.0,
        226.0,
        28.0,
    )
    .translate(235.0, -70.0, DECK_Z + 14.0);

    inlet_to_test
        + test_to_disposition
        + inlet_filter_baffle
        + filter_optical_baffle
        + pressure_backflush_baffle
        + backflush_retain_baffle
}

fn module_floor_markers() -> Part {
    let mut markers = Part::empty(format!("{OUTPUT_PREFIX}_module_floor_markers"));
    for module in module_specs() {
        markers = markers
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{}_floor_marker", module.name),
                module.x + 14.0,
                module.y + 14.0,
                2.4,
            )
            .translate(module.center.0, module.center.1, DECK_Z + 1.2);
    }
    markers
}

fn deck_fiducial_targets() -> Part {
    let mut datums = Part::empty(format!("{OUTPUT_PREFIX}_deck_fiducial_targets"));
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 98.0, -DECK_Y / 2.0 + 98.0),
        (DECK_X / 2.0 - 98.0, -DECK_Y / 2.0 + 98.0),
        (-DECK_X / 2.0 + 98.0, DECK_Y / 2.0 - 98.0),
        (DECK_X / 2.0 - 98.0, DECK_Y / 2.0 - 98.0),
    ]
    .iter()
    .enumerate()
    {
        datums = datums
            + fiducial_disc(&format!("{OUTPUT_PREFIX}_deck_datum_{i}")).translate(
                *x,
                *y,
                DECK_Z + 2.0,
            );
    }
    datums
}

fn leak_witness_wells() -> Part {
    let mut wells = Part::empty(format!("{OUTPUT_PREFIX}_leak_witness_wells"));
    for i in 0..LEAK_WITNESS_WELLS {
        let x = centered_index(i % 7, 7, 72.0);
        let y = -DECK_Y / 2.0 + 96.0 + (i / 7) as f64 * 34.0;
        wells = wells
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_closed_path_leak_witness_well_{i}"),
                10.0,
                6.0,
                28,
            )
            .translate(x, y, DECK_Z + 3.0);
    }
    wells
}

fn sealed_sample_loop_inlet_manifold() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_sealed_inlet_loop_body"),
        LOOP_X,
        LOOP_Y,
        LOOP_Z,
    );
    let service_recess = centered_cube(
        format!("{OUTPUT_PREFIX}_sealed_inlet_loop_service_recess_cut"),
        LOOP_X - 86.0,
        LOOP_Y - 80.0,
        14.0,
    )
    .translate(0.0, 0.0, LOOP_Z / 2.0 - 7.0);

    body - service_recess
        + sealed_inlet_ports()
        + loop_return_ports()
        + loop_tube_race()
        + loop_interlock_valves()
        + sample_branch_ports()
        + loop_pressure_seal_taps()
        + inlet_flow_direction_markers()
}

fn sealed_inlet_ports() -> Part {
    let mut ports = Part::empty(format!("{OUTPUT_PREFIX}_sealed_sample_loop_inlet_ports"));
    for i in 0..SAMPLE_LOOP_INLET_PORTS {
        let x = centered_index(i, SAMPLE_LOOP_INLET_PORTS, 92.0);
        let boss = centered_cylinder(
            format!("{OUTPUT_PREFIX}_sealed_inlet_port_boss_{i}"),
            18.0,
            18.0,
            36,
        )
        .translate(x, LOOP_Y / 2.0 - 32.0, LOOP_Z / 2.0 + 9.0);
        let bore = centered_cylinder(
            format!("{OUTPUT_PREFIX}_sealed_inlet_port_bore_{i}"),
            LOOP_BORE_D / 2.0,
            22.0,
            22,
        )
        .translate(x, LOOP_Y / 2.0 - 32.0, LOOP_Z / 2.0 + 10.0);
        let cap_land = centered_cube(
            format!("{OUTPUT_PREFIX}_sealed_inlet_cap_land_{i}"),
            42.0,
            20.0,
            7.0,
        )
        .translate(x, LOOP_Y / 2.0 - 70.0, LOOP_Z / 2.0 + 5.0);
        ports = ports + (boss - bore) + cap_land;
    }
    ports
}

fn loop_return_ports() -> Part {
    let mut ports = Part::empty(format!("{OUTPUT_PREFIX}_sample_loop_return_ports"));
    for i in 0..SAMPLE_LOOP_RETURN_PORTS {
        let x = centered_index(i, SAMPLE_LOOP_RETURN_PORTS, 92.0);
        let boss = centered_cylinder(
            format!("{OUTPUT_PREFIX}_return_port_boss_{i}"),
            15.0,
            16.0,
            32,
        )
        .translate(x, -LOOP_Y / 2.0 + 32.0, LOOP_Z / 2.0 + 8.0);
        let bore = centered_cylinder(
            format!("{OUTPUT_PREFIX}_return_port_bore_{i}"),
            LOOP_BORE_D / 2.0,
            18.0,
            20,
        )
        .translate(x, -LOOP_Y / 2.0 + 32.0, LOOP_Z / 2.0 + 9.0);
        ports = ports + (boss - bore);
    }
    ports
}

fn loop_tube_race() -> Part {
    let left = tube_y(
        &format!("{OUTPUT_PREFIX}_sealed_loop_left_low_shear_race"),
        LOOP_Y - 86.0,
        LOOP_TUBE_D,
    )
    .translate(-LOOP_X / 2.0 + 62.0, 0.0, LOOP_Z / 2.0 + 17.0);
    let right = tube_y(
        &format!("{OUTPUT_PREFIX}_sealed_loop_right_low_shear_race"),
        LOOP_Y - 86.0,
        LOOP_TUBE_D,
    )
    .translate(LOOP_X / 2.0 - 62.0, 0.0, LOOP_Z / 2.0 + 17.0);
    let rear = tube_x(
        &format!("{OUTPUT_PREFIX}_sealed_loop_rear_low_shear_race"),
        LOOP_X - 124.0,
        LOOP_TUBE_D,
    )
    .translate(0.0, LOOP_Y / 2.0 - 56.0, LOOP_Z / 2.0 + 17.0);
    let front = tube_x(
        &format!("{OUTPUT_PREFIX}_sealed_loop_front_low_shear_race"),
        LOOP_X - 124.0,
        LOOP_TUBE_D,
    )
    .translate(0.0, -LOOP_Y / 2.0 + 56.0, LOOP_Z / 2.0 + 17.0);

    left + right + rear + front
}

fn loop_interlock_valves() -> Part {
    let mut valves = Part::empty(format!("{OUTPUT_PREFIX}_closed_loop_interlock_valves"));
    for i in 0..LOOP_INTERLOCK_VALVES {
        let x = if i < 4 {
            centered_index(i, 4, 70.0)
        } else if i % 2 == 0 {
            -LOOP_X / 2.0 + 94.0
        } else {
            LOOP_X / 2.0 - 94.0
        };
        let y = if i < 4 {
            0.0
        } else {
            centered_index((i - 4) / 2, 2, 64.0)
        };
        let valve = centered_cube(
            format!("{OUTPUT_PREFIX}_pinch_interlock_valve_{i}"),
            42.0,
            24.0,
            24.0,
        )
        .translate(x, y, LOOP_Z / 2.0 + 12.0);
        let tube_clearance = tube_x(
            &format!("{OUTPUT_PREFIX}_pinch_interlock_valve_tube_clearance_{i}"),
            46.0,
            LOOP_TUBE_D,
        )
        .translate(x, y, LOOP_Z / 2.0 + 12.0);
        valves = valves + (valve - tube_clearance);
    }
    valves
}

fn sample_branch_ports() -> Part {
    let mut branches = Part::empty(format!("{OUTPUT_PREFIX}_sample_branch_ports"));
    for i in 0..SAMPLE_BRANCH_PORTS {
        let y = centered_index(i, SAMPLE_BRANCH_PORTS, 38.0);
        let block = centered_cube(
            format!("{OUTPUT_PREFIX}_sealed_sample_branch_block_{i}"),
            34.0,
            18.0,
            16.0,
        )
        .translate(0.0, y, LOOP_Z / 2.0 + 8.0);
        let bore = tube_x(
            &format!("{OUTPUT_PREFIX}_sealed_sample_branch_bore_{i}"),
            40.0,
            LOOP_BORE_D,
        )
        .translate(0.0, y, LOOP_Z / 2.0 + 8.0);
        branches = branches + (block - bore);
    }
    branches
}

fn loop_pressure_seal_taps() -> Part {
    let mut taps = Part::empty(format!("{OUTPUT_PREFIX}_loop_pressure_seal_taps"));
    for i in 0..LOOP_PRESSURE_SEAL_TAPS {
        taps = taps
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_loop_pressure_seal_tap_{i}"),
                8.0,
                16.0,
                28,
            )
            .translate(
                centered_index(i, LOOP_PRESSURE_SEAL_TAPS, 68.0),
                -26.0,
                LOOP_Z / 2.0 + 8.0,
            );
    }
    taps
}

fn inlet_flow_direction_markers() -> Part {
    flow_arrow_marker(
        format!("{OUTPUT_PREFIX}_inlet_forward_flow_arrow"),
        1.0,
        0.0,
    )
    .translate(-94.0, LOOP_Y / 2.0 - 58.0, LOOP_Z / 2.0 + 4.0)
        + flow_arrow_marker(format!("{OUTPUT_PREFIX}_return_flow_arrow"), -1.0, 0.0).translate(
            94.0,
            -LOOP_Y / 2.0 + 58.0,
            LOOP_Z / 2.0 + 4.0,
        )
}

fn gentle_sieve_filter_coupon_ladder() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_gentle_sieve_ladder_body"),
        FILTER_X,
        FILTER_Y,
        FILTER_Z,
    );
    let rinse_basin = centered_cube(
        format!("{OUTPUT_PREFIX}_gentle_sieve_ladder_rinse_basin_cut"),
        FILTER_X - 54.0,
        FILTER_Y - 50.0,
        11.0,
    )
    .translate(0.0, 0.0, FILTER_Z / 2.0 - 5.5);

    body - rinse_basin - sieve_coupon_pocket_cuts()
        + sieve_coupon_frames()
        + sieve_mesh_surrogate_ribs()
        + gentle_bypass_channels()
        + filter_stage_label_lands()
        + sieve_ladder_handle_tabs()
}

fn sieve_coupon_pocket_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_sieve_coupon_pocket_cuts"));
    for stage in 0..SIEVE_STAGES {
        for replicate in 0..COUPONS_PER_STAGE {
            let index = stage * COUPONS_PER_STAGE + replicate;
            cuts = cuts
                + centered_cube(
                    format!("{OUTPUT_PREFIX}_sieve_coupon_pocket_cut_{index}"),
                    SIEVE_COUPON_X,
                    SIEVE_COUPON_Y,
                    15.0,
                )
                .translate(
                    centered_index(stage, SIEVE_STAGES, SIEVE_STAGE_PITCH_X),
                    centered_index(replicate, COUPONS_PER_STAGE, SIEVE_REPLICATE_PITCH_Y),
                    FILTER_Z / 2.0 - 6.0,
                );
        }
    }
    cuts
}

fn sieve_coupon_frames() -> Part {
    let mut frames = Part::empty(format!("{OUTPUT_PREFIX}_sieve_coupon_frames"));
    for stage in 0..SIEVE_STAGES {
        for replicate in 0..COUPONS_PER_STAGE {
            let index = stage * COUPONS_PER_STAGE + replicate;
            let x = centered_index(stage, SIEVE_STAGES, SIEVE_STAGE_PITCH_X);
            let y = centered_index(replicate, COUPONS_PER_STAGE, SIEVE_REPLICATE_PITCH_Y);
            let frame = centered_cube(
                format!("{OUTPUT_PREFIX}_sieve_coupon_frame_{index}"),
                SIEVE_COUPON_X + 16.0,
                SIEVE_COUPON_Y + 16.0,
                8.0,
            )
            .translate(x, y, FILTER_Z / 2.0 + 4.0);
            let opening = centered_cube(
                format!("{OUTPUT_PREFIX}_sieve_coupon_frame_opening_{index}"),
                SIEVE_COUPON_X - 8.0,
                SIEVE_COUPON_Y - 8.0,
                10.0,
            )
            .translate(x, y, FILTER_Z / 2.0 + 5.0);
            frames = frames + (frame - opening);
        }
    }
    frames
}

fn sieve_mesh_surrogate_ribs() -> Part {
    let mut ribs = Part::empty(format!("{OUTPUT_PREFIX}_sieve_mesh_surrogate_ribs"));
    for stage in 0..SIEVE_STAGES {
        for replicate in 0..COUPONS_PER_STAGE {
            let index = stage * COUPONS_PER_STAGE + replicate;
            let base_x = centered_index(stage, SIEVE_STAGES, SIEVE_STAGE_PITCH_X);
            let y = centered_index(replicate, COUPONS_PER_STAGE, SIEVE_REPLICATE_PITCH_Y);
            for rib in 0..SIEVE_MESH_RIBS_PER_COUPON {
                ribs = ribs
                    + centered_cube(
                        format!("{OUTPUT_PREFIX}_sieve_coupon_{index}_mesh_rib_{rib}"),
                        4.0,
                        SIEVE_COUPON_Y - 14.0,
                        4.0,
                    )
                    .translate(
                        base_x + centered_index(rib, SIEVE_MESH_RIBS_PER_COUPON, 12.0),
                        y,
                        FILTER_Z / 2.0 + 9.0,
                    );
            }
        }
    }
    ribs
}

fn gentle_bypass_channels() -> Part {
    let mut channels = Part::empty(format!("{OUTPUT_PREFIX}_low_shear_bypass_channels"));
    for i in 0..LOW_SHEAR_BYPASS_CHANNELS {
        let y = centered_index(i, LOW_SHEAR_BYPASS_CHANNELS, 156.0);
        channels = channels
            + tube_x(
                &format!("{OUTPUT_PREFIX}_low_shear_bypass_channel_{i}"),
                FILTER_X - 72.0,
                7.0,
            )
            .translate(0.0, y, FILTER_Z / 2.0 + 16.0);
    }
    channels
}

fn filter_stage_label_lands() -> Part {
    let mut labels = Part::empty(format!("{OUTPUT_PREFIX}_filter_stage_label_lands"));
    for (stage, label) in FILTER_STAGE_LABELS.iter().enumerate() {
        labels = labels
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{label}_filter_stage_label_land"),
                58.0,
                10.0,
                4.0,
            )
            .translate(
                centered_index(stage, SIEVE_STAGES, SIEVE_STAGE_PITCH_X),
                -FILTER_Y / 2.0 + 22.0,
                FILTER_Z / 2.0 + 4.0,
            );
    }
    labels
}

fn sieve_ladder_handle_tabs() -> Part {
    centered_cube(
        format!("{OUTPUT_PREFIX}_sieve_ladder_left_aseptic_pull_tab"),
        18.0,
        FILTER_Y - 36.0,
        16.0,
    )
    .translate(-FILTER_X / 2.0 - 9.0, 0.0, 6.0)
        + centered_cube(
            format!("{OUTPUT_PREFIX}_sieve_ladder_right_aseptic_pull_tab"),
            18.0,
            FILTER_Y - 36.0,
            16.0,
        )
        .translate(FILTER_X / 2.0 + 9.0, 0.0, 6.0)
}

fn optical_clump_witness_windows() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_optical_clump_window_body"),
        OPTICAL_X,
        OPTICAL_Y,
        OPTICAL_Z,
    );
    let light_tunnel = centered_cube(
        format!("{OUTPUT_PREFIX}_optical_light_tunnel_cut"),
        OPTICAL_X - 74.0,
        64.0,
        18.0,
    )
    .translate(0.0, 0.0, OPTICAL_Z / 2.0 - 9.0);

    body - light_tunnel - optical_window_cuts()
        + optical_window_frames()
        + clump_size_band_ticks()
        + optical_light_bars()
        + dark_reference_shutters()
        + optical_camera_mount_lands()
}

fn optical_window_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_optical_clump_window_cuts"));
    for i in 0..OPTICAL_CLUMP_WINDOWS {
        cuts = cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_optical_clump_witness_window_cut_{i}"),
                34.0,
                78.0,
                20.0,
            )
            .translate(
                centered_index(i, OPTICAL_CLUMP_WINDOWS, 48.0),
                0.0,
                OPTICAL_Z / 2.0 - 10.0,
            );
    }
    cuts
}

fn optical_window_frames() -> Part {
    let mut frames = Part::empty(format!("{OUTPUT_PREFIX}_optical_clump_window_frames"));
    for i in 0..OPTICAL_CLUMP_WINDOWS {
        let x = centered_index(i, OPTICAL_CLUMP_WINDOWS, 48.0);
        let frame = centered_cube(
            format!("{OUTPUT_PREFIX}_optical_clump_witness_frame_{i}"),
            46.0,
            90.0,
            8.0,
        )
        .translate(x, 0.0, OPTICAL_Z / 2.0 + 4.0);
        let opening = centered_cube(
            format!("{OUTPUT_PREFIX}_optical_clump_witness_frame_opening_{i}"),
            30.0,
            72.0,
            10.0,
        )
        .translate(x, 0.0, OPTICAL_Z / 2.0 + 5.0);
        frames = frames + (frame - opening);
    }
    frames
}

fn clump_size_band_ticks() -> Part {
    let mut ticks = Part::empty(format!("{OUTPUT_PREFIX}_clump_size_band_ticks"));
    for band in 0..CLUMP_SIZE_BANDS {
        ticks = ticks
            + centered_cube(
                format!("{OUTPUT_PREFIX}_clump_size_band_tick_{band}"),
                5.0,
                OPTICAL_Y - 48.0,
                5.0,
            )
            .translate(
                centered_index(band, CLUMP_SIZE_BANDS, 58.0),
                0.0,
                OPTICAL_Z / 2.0 + 8.0,
            );
    }
    ticks
}

fn optical_light_bars() -> Part {
    let mut bars = Part::empty(format!("{OUTPUT_PREFIX}_optical_light_bars"));
    for i in 0..OPTICAL_LIGHT_BARS {
        bars = bars
            + centered_cube(
                format!("{OUTPUT_PREFIX}_optical_grazing_light_bar_{i}"),
                OPTICAL_X - 80.0,
                10.0,
                12.0,
            )
            .translate(
                0.0,
                centered_index(i, OPTICAL_LIGHT_BARS, 126.0),
                OPTICAL_Z / 2.0 + 10.0,
            );
    }
    bars
}

fn dark_reference_shutters() -> Part {
    let mut shutters = Part::empty(format!("{OUTPUT_PREFIX}_dark_reference_shutters"));
    for i in 0..DARK_REFERENCE_SHUTTERS {
        shutters = shutters
            + centered_cube(
                format!("{OUTPUT_PREFIX}_dark_reference_shutter_{i}"),
                38.0,
                82.0,
                10.0,
            )
            .translate(
                centered_index(i, DARK_REFERENCE_SHUTTERS, OPTICAL_X - 78.0),
                0.0,
                OPTICAL_Z / 2.0 + 11.0,
            );
    }
    shutters
}

fn optical_camera_mount_lands() -> Part {
    let left = centered_cube(
        format!("{OUTPUT_PREFIX}_optical_left_camera_mount_land"),
        48.0,
        28.0,
        18.0,
    )
    .translate(
        -OPTICAL_X / 2.0 + 38.0,
        OPTICAL_Y / 2.0 - 28.0,
        OPTICAL_Z / 2.0 + 9.0,
    );
    let right = centered_cube(
        format!("{OUTPUT_PREFIX}_optical_right_camera_mount_land"),
        48.0,
        28.0,
        18.0,
    )
    .translate(
        OPTICAL_X / 2.0 - 38.0,
        OPTICAL_Y / 2.0 - 28.0,
        OPTICAL_Z / 2.0 + 9.0,
    );
    left + right
}

fn pressure_drop_tap_ladder() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_pressure_drop_tap_ladder_body"),
        PRESSURE_X,
        PRESSURE_Y,
        PRESSURE_Z,
    );
    let channel_relief = centered_cube(
        format!("{OUTPUT_PREFIX}_pressure_drop_channel_relief_cut"),
        PRESSURE_X - 70.0,
        44.0,
        13.0,
    )
    .translate(0.0, 0.0, PRESSURE_Z / 2.0 - 6.5);

    body - channel_relief
        + pressure_tap_pair_bosses()
        + pressure_reference_restrictors()
        + pressure_sensor_pad_ladder()
        + equal_length_pressure_trace_markers()
}

fn pressure_tap_pair_bosses() -> Part {
    let mut taps = Part::empty(format!("{OUTPUT_PREFIX}_pressure_tap_pair_bosses"));
    for i in 0..PRESSURE_TAP_PAIRS {
        let x = centered_index(i, PRESSURE_TAP_PAIRS, 58.0);
        for side in 0..2 {
            let y = centered_index(side, 2, 58.0);
            let boss = centered_cylinder(
                format!("{OUTPUT_PREFIX}_pressure_drop_tap_{i}_{side}_boss"),
                10.0,
                16.0,
                28,
            )
            .translate(x, y, PRESSURE_Z / 2.0 + 8.0);
            let bore = centered_cylinder(
                format!("{OUTPUT_PREFIX}_pressure_drop_tap_{i}_{side}_bore"),
                PRESSURE_TUBE_D / 2.0,
                18.0,
                20,
            )
            .translate(x, y, PRESSURE_Z / 2.0 + 9.0);
            taps = taps + (boss - bore);
        }
    }
    taps
}

fn pressure_reference_restrictors() -> Part {
    let mut restrictors = Part::empty(format!("{OUTPUT_PREFIX}_pressure_reference_restrictors"));
    for i in 0..DP_REFERENCE_RESTRICTORS {
        restrictors = restrictors
            + tube_x(
                &format!("{OUTPUT_PREFIX}_dp_reference_restrictor_{i}"),
                44.0,
                4.2,
            )
            .translate(
                centered_index(i, DP_REFERENCE_RESTRICTORS, 68.0),
                -PRESSURE_Y / 2.0 + 34.0,
                PRESSURE_Z / 2.0 + 12.0,
            );
    }
    restrictors
}

fn pressure_sensor_pad_ladder() -> Part {
    let mut pads = Part::empty(format!("{OUTPUT_PREFIX}_pressure_sensor_pad_ladder"));
    for i in 0..PRESSURE_SENSOR_PADS {
        pads = pads
            + centered_cube(
                format!("{OUTPUT_PREFIX}_pressure_sensor_pad_{i}"),
                42.0,
                28.0,
                10.0,
            )
            .translate(
                centered_index(i, PRESSURE_SENSOR_PADS, 58.0),
                PRESSURE_Y / 2.0 - 34.0,
                PRESSURE_Z / 2.0 + 5.0,
            );
    }
    pads
}

fn equal_length_pressure_trace_markers() -> Part {
    let mut markers = Part::empty(format!(
        "{OUTPUT_PREFIX}_equal_length_pressure_trace_markers"
    ));
    for i in 0..PRESSURE_TAP_POINTS {
        markers = markers
            + centered_cube(
                format!("{OUTPUT_PREFIX}_pressure_trace_length_marker_{i}"),
                4.0,
                PRESSURE_Y - 76.0,
                4.0,
            )
            .translate(
                centered_index(i, PRESSURE_TAP_POINTS, 58.0),
                0.0,
                PRESSURE_Z / 2.0 + 5.0,
            );
    }
    markers
}

fn backflush_recovery_route() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_backflush_recovery_route_body"),
        BACKFLUSH_X,
        BACKFLUSH_Y,
        BACKFLUSH_Z,
    );
    let recovery_sump = centered_cube(
        format!("{OUTPUT_PREFIX}_backflush_recovery_sump_cut"),
        BACKFLUSH_X - 64.0,
        BACKFLUSH_Y - 58.0,
        12.0,
    )
    .translate(0.0, 0.0, BACKFLUSH_Z / 2.0 - 6.0);

    body - recovery_sump
        + backflush_branch_tubes()
        + recovery_vial_nests()
        + backflush_selector_valves()
        + recovery_check_valves()
        + backflush_pump_envelope()
        + backflush_flow_arrows()
}

fn backflush_branch_tubes() -> Part {
    let mut branches = Part::empty(format!("{OUTPUT_PREFIX}_backflush_branch_tubes"));
    for i in 0..BACKFLUSH_BRANCHES {
        branches = branches
            + tube_y(
                &format!("{OUTPUT_PREFIX}_backflush_stage_branch_{i}"),
                BACKFLUSH_Y - 70.0,
                6.4,
            )
            .translate(
                centered_index(i, BACKFLUSH_BRANCHES, 62.0),
                0.0,
                BACKFLUSH_Z / 2.0 + 18.0,
            );
    }
    branches
}

fn recovery_vial_nests() -> Part {
    let mut nests = Part::empty(format!("{OUTPUT_PREFIX}_recovered_clump_vial_nests"));
    for i in 0..RECOVERY_VIAL_NESTS {
        let x = centered_index(i, RECOVERY_VIAL_NESTS, 62.0);
        let cup = centered_cylinder(
            format!("{OUTPUT_PREFIX}_recovered_clump_retain_vial_nest_{i}"),
            18.0,
            22.0,
            32,
        )
        .translate(x, -BACKFLUSH_Y / 2.0 + 38.0, BACKFLUSH_Z / 2.0 + 11.0);
        let bore = centered_cylinder(
            format!("{OUTPUT_PREFIX}_recovered_clump_retain_vial_clearance_{i}"),
            11.0,
            24.0,
            28,
        )
        .translate(x, -BACKFLUSH_Y / 2.0 + 38.0, BACKFLUSH_Z / 2.0 + 12.0);
        nests = nests + (cup - bore);
    }
    nests
}

fn backflush_selector_valves() -> Part {
    let mut valves = Part::empty(format!("{OUTPUT_PREFIX}_backflush_selector_valves"));
    for i in 0..BACKFLUSH_SELECTOR_VALVES {
        valves = valves
            + centered_cube(
                format!("{OUTPUT_PREFIX}_backflush_selector_valve_{i}"),
                34.0,
                28.0,
                24.0,
            )
            .translate(
                centered_index(i, BACKFLUSH_SELECTOR_VALVES, 54.0),
                0.0,
                BACKFLUSH_Z / 2.0 + 12.0,
            );
    }
    valves
}

fn recovery_check_valves() -> Part {
    let mut valves = Part::empty(format!("{OUTPUT_PREFIX}_recovery_check_valves"));
    for i in 0..RECOVERY_CHECK_VALVES {
        valves = valves
            + centered_cube(
                format!("{OUTPUT_PREFIX}_recovery_check_valve_{i}"),
                24.0,
                18.0,
                16.0,
            )
            .translate(
                centered_index(i, RECOVERY_CHECK_VALVES, 62.0),
                BACKFLUSH_Y / 2.0 - 44.0,
                BACKFLUSH_Z / 2.0 + 8.0,
            );
    }
    valves
}

fn backflush_pump_envelope() -> Part {
    centered_cube(
        format!("{OUTPUT_PREFIX}_gentle_backflush_pump_envelope"),
        72.0,
        52.0,
        34.0,
    )
    .translate(-BACKFLUSH_X / 2.0 + 66.0, 0.0, BACKFLUSH_Z / 2.0 + 17.0)
}

fn backflush_flow_arrows() -> Part {
    let mut arrows = Part::empty(format!("{OUTPUT_PREFIX}_backflush_flow_arrows"));
    for i in 0..BACKFLUSH_BRANCHES {
        arrows = arrows
            + flow_arrow_marker(
                format!("{OUTPUT_PREFIX}_backflush_recovery_arrow_{i}"),
                0.0,
                -1.0,
            )
            .translate(
                centered_index(i, BACKFLUSH_BRANCHES, 62.0),
                -14.0,
                BACKFLUSH_Z / 2.0 + 5.0,
            );
    }
    arrows
}

fn live_dead_retain_sample_pockets() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_live_dead_retain_sample_body"),
        RETAIN_X,
        RETAIN_Y,
        RETAIN_Z,
    );
    let chilled_recess = centered_cube(
        format!("{OUTPUT_PREFIX}_live_dead_chilled_retain_shelf_cut"),
        RETAIN_X - 58.0,
        RETAIN_Y - 52.0,
        10.0,
    )
    .translate(0.0, 0.0, RETAIN_Z / 2.0 - 5.0);

    body - chilled_recess - retain_sample_pocket_cuts()
        + retain_sample_pocket_frames()
        + live_dead_reagent_guard_lands()
        + retain_septum_caps()
        + chilled_retain_lands()
}

fn retain_sample_pocket_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_retain_sample_pocket_cuts"));
    for pair in 0..LIVE_DEAD_RETAIN_PAIRS {
        for side in 0..2 {
            let index = pair * 2 + side;
            cuts = cuts
                + centered_cylinder(
                    format!("{OUTPUT_PREFIX}_retain_sample_pocket_cut_{index}"),
                    14.0,
                    18.0,
                    28,
                )
                .translate(
                    centered_index(pair, LIVE_DEAD_RETAIN_PAIRS, 58.0),
                    centered_index(side, 2, 72.0),
                    RETAIN_Z / 2.0 - 8.0,
                );
        }
    }
    cuts
}

fn retain_sample_pocket_frames() -> Part {
    let mut frames = Part::empty(format!("{OUTPUT_PREFIX}_retain_sample_pocket_frames"));
    for pair in 0..LIVE_DEAD_RETAIN_PAIRS {
        for side in 0..2 {
            let index = pair * 2 + side;
            frames = frames
                + centered_cylinder(
                    format!(
                        "{OUTPUT_PREFIX}_{}_retain_sample_frame_{pair}",
                        if side == 0 { "live" } else { "dead" }
                    ),
                    20.0,
                    8.0,
                    32,
                )
                .translate(
                    centered_index(pair, LIVE_DEAD_RETAIN_PAIRS, 58.0),
                    centered_index(side, 2, 72.0),
                    RETAIN_Z / 2.0 + 4.0,
                )
                - centered_cylinder(
                    format!("{OUTPUT_PREFIX}_retain_sample_frame_opening_{index}"),
                    12.0,
                    10.0,
                    24,
                )
                .translate(
                    centered_index(pair, LIVE_DEAD_RETAIN_PAIRS, 58.0),
                    centered_index(side, 2, 72.0),
                    RETAIN_Z / 2.0 + 5.0,
                );
        }
    }
    frames
}

fn live_dead_reagent_guard_lands() -> Part {
    let mut guards = Part::empty(format!("{OUTPUT_PREFIX}_live_dead_reagent_guard_lands"));
    for i in 0..LIVE_DEAD_REAGENT_GUARDS {
        guards = guards
            + centered_cube(
                format!("{OUTPUT_PREFIX}_reagent_guard_land_{i}"),
                RETAIN_X - 78.0,
                8.0,
                8.0,
            )
            .translate(
                0.0,
                centered_index(i, LIVE_DEAD_REAGENT_GUARDS, 92.0),
                RETAIN_Z / 2.0 + 4.0,
            );
    }
    guards
}

fn retain_septum_caps() -> Part {
    let mut caps = Part::empty(format!("{OUTPUT_PREFIX}_retain_septum_caps"));
    for pair in 0..LIVE_DEAD_RETAIN_PAIRS {
        for side in 0..2 {
            let index = pair * 2 + side;
            caps = caps
                + centered_cube(
                    format!("{OUTPUT_PREFIX}_retain_sample_septum_cap_land_{index}"),
                    28.0,
                    10.0,
                    5.0,
                )
                .translate(
                    centered_index(pair, LIVE_DEAD_RETAIN_PAIRS, 58.0),
                    centered_index(side, 2, 72.0) + if side == 0 { 28.0 } else { -28.0 },
                    RETAIN_Z / 2.0 + 5.0,
                );
        }
    }
    caps
}

fn chilled_retain_lands() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_chilled_retain_lands"));
    for i in 0..RETAIN_CHILLED_LANDS {
        lands = lands
            + centered_cube(
                format!("{OUTPUT_PREFIX}_chilled_retain_thermal_land_{i}"),
                52.0,
                14.0,
                6.0,
            )
            .translate(
                centered_index(i, RETAIN_CHILLED_LANDS, 66.0),
                -RETAIN_Y / 2.0 + 22.0,
                RETAIN_Z / 2.0 + 3.0,
            );
    }
    lands
}

fn bubble_dead_volume_window_bank() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_bubble_dead_volume_window_body"),
        BUBBLE_X,
        BUBBLE_Y,
        BUBBLE_Z,
    );
    let trough = centered_cube(
        format!("{OUTPUT_PREFIX}_bubble_dead_volume_low_point_trough_cut"),
        BUBBLE_X - 58.0,
        24.0,
        10.0,
    )
    .translate(0.0, -BUBBLE_Y / 2.0 + 28.0, BUBBLE_Z / 2.0 - 5.0);

    body - trough - bubble_dead_volume_window_cuts()
        + bubble_window_frames()
        + dead_volume_tick_marks()
        + air_gap_sentinel_wells()
}

fn bubble_dead_volume_window_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_bubble_dead_volume_window_cuts"));
    for i in 0..BUBBLE_WINDOWS {
        cuts = cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_bubble_witness_window_cut_{i}"),
                46.0,
                30.0,
                12.0,
            )
            .translate(
                centered_index(i, BUBBLE_WINDOWS, 66.0),
                28.0,
                BUBBLE_Z / 2.0 - 6.0,
            );
    }
    for i in 0..DEAD_VOLUME_WINDOWS {
        cuts = cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_dead_volume_witness_window_cut_{i}"),
                46.0,
                30.0,
                12.0,
            )
            .translate(
                centered_index(i, DEAD_VOLUME_WINDOWS, 66.0),
                -26.0,
                BUBBLE_Z / 2.0 - 6.0,
            );
    }
    cuts
}

fn bubble_window_frames() -> Part {
    let mut frames = Part::empty(format!("{OUTPUT_PREFIX}_bubble_dead_volume_window_frames"));
    for row in 0..2 {
        let count = if row == 0 {
            BUBBLE_WINDOWS
        } else {
            DEAD_VOLUME_WINDOWS
        };
        for i in 0..count {
            let frame = centered_cube(
                format!("{OUTPUT_PREFIX}_window_frame_{row}_{i}"),
                58.0,
                42.0,
                7.0,
            )
            .translate(
                centered_index(i, count, 66.0),
                if row == 0 { 28.0 } else { -26.0 },
                BUBBLE_Z / 2.0 + 3.5,
            );
            let opening = centered_cube(
                format!("{OUTPUT_PREFIX}_window_frame_opening_{row}_{i}"),
                40.0,
                24.0,
                9.0,
            )
            .translate(
                centered_index(i, count, 66.0),
                if row == 0 { 28.0 } else { -26.0 },
                BUBBLE_Z / 2.0 + 4.5,
            );
            frames = frames + (frame - opening);
        }
    }
    frames
}

fn dead_volume_tick_marks() -> Part {
    let mut ticks = Part::empty(format!("{OUTPUT_PREFIX}_dead_volume_tick_marks"));
    for i in 0..DEAD_VOLUME_TICK_MARKS {
        ticks = ticks
            + centered_cube(
                format!("{OUTPUT_PREFIX}_dead_volume_tick_{i}"),
                4.0,
                46.0,
                4.0,
            )
            .translate(
                centered_index(i, DEAD_VOLUME_TICK_MARKS, 36.0),
                -BUBBLE_Y / 2.0 + 30.0,
                BUBBLE_Z / 2.0 + 4.0,
            );
    }
    ticks
}

fn air_gap_sentinel_wells() -> Part {
    let mut wells = Part::empty(format!("{OUTPUT_PREFIX}_air_gap_sentinel_wells"));
    for i in 0..AIR_GAP_SENTINELS {
        wells = wells
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_air_gap_sentinel_well_{i}"),
                10.0,
                8.0,
                28,
            )
            .translate(
                BUBBLE_X / 2.0 - 54.0,
                centered_index(i, AIR_GAP_SENTINELS, 32.0),
                BUBBLE_Z / 2.0 + 4.0,
            );
    }
    wells
}

fn waste_reject_split_manifold() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_waste_reject_split_body"),
        WASTE_X,
        WASTE_Y,
        WASTE_Z,
    );
    let moat = centered_cube(
        format!("{OUTPUT_PREFIX}_waste_reject_secondary_moat_cut"),
        WASTE_X - 58.0,
        WASTE_Y - 42.0,
        10.0,
    )
    .translate(0.0, 0.0, WASTE_Z / 2.0 - 5.0);

    body - moat
        + waste_reject_lane_bars()
        + split_diverter_valves()
        + waste_and_reject_bottle_nests()
        + quarantine_lock_lands()
}

fn waste_reject_lane_bars() -> Part {
    let mut lanes = Part::empty(format!("{OUTPUT_PREFIX}_waste_reject_lane_bars"));
    for i in 0..WASTE_REJECT_LANES {
        lanes = lanes
            + tube_x(
                &format!(
                    "{OUTPUT_PREFIX}_{}_split_lane",
                    if i == 0 { "waste" } else { "reject" }
                ),
                WASTE_X - 84.0,
                7.0,
            )
            .translate(
                0.0,
                centered_index(i, WASTE_REJECT_LANES, 52.0),
                WASTE_Z / 2.0 + 16.0,
            );
    }
    lanes
}

fn split_diverter_valves() -> Part {
    let mut valves = Part::empty(format!("{OUTPUT_PREFIX}_split_diverter_valves"));
    for i in 0..SPLIT_DIVERTER_VALVES {
        valves = valves
            + centered_cube(
                format!("{OUTPUT_PREFIX}_waste_reject_diverter_valve_{i}"),
                34.0,
                24.0,
                22.0,
            )
            .translate(
                centered_index(i, SPLIT_DIVERTER_VALVES, 64.0),
                0.0,
                WASTE_Z / 2.0 + 11.0,
            );
    }
    valves
}

fn waste_and_reject_bottle_nests() -> Part {
    let mut nests = Part::empty(format!("{OUTPUT_PREFIX}_waste_reject_bottle_nests"));
    for i in 0..WASTE_BOTTLE_NESTS {
        nests = nests
            + bottle_nest(&format!("{OUTPUT_PREFIX}_waste_bottle_nest_{i}")).translate(
                centered_index(i, WASTE_BOTTLE_NESTS, 54.0) - 106.0,
                WASTE_Y / 2.0 - 28.0,
                WASTE_Z / 2.0 + 8.0,
            );
    }
    for i in 0..REJECT_BOTTLE_NESTS {
        nests = nests
            + bottle_nest(&format!("{OUTPUT_PREFIX}_reject_bottle_nest_{i}")).translate(
                centered_index(i, REJECT_BOTTLE_NESTS, 54.0) + 106.0,
                -WASTE_Y / 2.0 + 28.0,
                WASTE_Z / 2.0 + 8.0,
            );
    }
    nests
}

fn quarantine_lock_lands() -> Part {
    let mut locks = Part::empty(format!("{OUTPUT_PREFIX}_quarantine_lock_lands"));
    for i in 0..QUARANTINE_LOCKS {
        locks = locks
            + centered_cube(
                format!("{OUTPUT_PREFIX}_quarantine_lock_land_{i}"),
                46.0,
                18.0,
                8.0,
            )
            .translate(
                centered_index(i, QUARANTINE_LOCKS, 58.0),
                -4.0,
                WASTE_Z / 2.0 + 4.0,
            );
    }
    locks
}

fn barcode_custody_release_gate_panel() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_barcode_custody_release_gate_panel_body"),
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    let card_recess = centered_cube(
        format!("{OUTPUT_PREFIX}_custody_card_recess_cut"),
        CUSTODY_X - 50.0,
        CUSTODY_Y - 36.0,
        9.0,
    )
    .translate(0.0, 0.0, CUSTODY_Z / 2.0 - 4.5);

    body - card_recess
        + barcode_custody_lands()
        + rfid_custody_lands()
        + custody_seal_lands()
        + release_hold_reject_gate_array()
        + gate_decision_input_lands()
}

fn barcode_custody_lands() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_barcode_custody_lands"));
    for i in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("{OUTPUT_PREFIX}_barcode_custody_land_{i}"),
                34.0,
                12.0,
                4.0,
            )
            .translate(
                centered_index(i % 6, 6, 48.0),
                CUSTODY_Y / 2.0 - 20.0 - (i / 6) as f64 * 22.0,
                CUSTODY_Z / 2.0 + 4.0,
            );
    }
    lands
}

fn rfid_custody_lands() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_rfid_custody_lands"));
    for i in 0..RFID_LANDS {
        lands = lands
            + centered_cube(
                format!("{OUTPUT_PREFIX}_rfid_custody_land_{i}"),
                42.0,
                18.0,
                4.0,
            )
            .translate(
                centered_index(i, RFID_LANDS, 62.0),
                -CUSTODY_Y / 2.0 + 20.0,
                CUSTODY_Z / 2.0 + 4.0,
            );
    }
    lands
}

fn custody_seal_lands() -> Part {
    let mut seals = Part::empty(format!("{OUTPUT_PREFIX}_custody_seal_lands"));
    for i in 0..CUSTODY_SEAL_LANDS {
        seals = seals
            + centered_cube(
                format!("{OUTPUT_PREFIX}_tamper_custody_seal_land_{i}"),
                32.0,
                8.0,
                5.0,
            )
            .translate(
                centered_index(i, CUSTODY_SEAL_LANDS, 46.0),
                0.0,
                CUSTODY_Z / 2.0 + 5.0,
            );
    }
    seals
}

fn release_hold_reject_gate_array() -> Part {
    let mut gates = Part::empty(format!("{OUTPUT_PREFIX}_release_hold_reject_gate_array"));
    for (lane, name) in DISPOSITION_LANES.iter().enumerate() {
        let x = centered_index(lane, DISPOSITION_LANES.len(), 98.0);
        let rail = centered_cube(
            format!("{OUTPUT_PREFIX}_{name}_gate_rail"),
            78.0,
            16.0,
            18.0,
        )
        .translate(x, -10.0, CUSTODY_Z / 2.0 + 9.0);
        gates = gates + rail;
        for slot in 0..GATE_SLOTS_PER_LANE {
            gates = gates
                + centered_cube(
                    format!("{OUTPUT_PREFIX}_{name}_gate_slot_token_{slot}"),
                    12.0,
                    10.0,
                    8.0,
                )
                .translate(
                    x + centered_index(slot, GATE_SLOTS_PER_LANE, 16.0),
                    -28.0,
                    CUSTODY_Z / 2.0 + 8.0,
                );
        }
    }
    for i in 0..GATE_SOLENOIDS {
        gates = gates
            + centered_cube(
                format!("{OUTPUT_PREFIX}_disposition_gate_solenoid_envelope_{i}"),
                22.0,
                20.0,
                20.0,
            )
            .translate(
                centered_index(i, GATE_SOLENOIDS, 44.0),
                -CUSTODY_Y / 2.0 + 38.0,
                CUSTODY_Z / 2.0 + 10.0,
            );
    }
    gates
}

fn gate_decision_input_lands() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_gate_decision_input_lands"));
    for i in 0..REQUIRED_FEATURES.len().min(8) {
        lands = lands
            + centered_cube(
                format!("{OUTPUT_PREFIX}_gate_decision_input_land_{i}"),
                26.0,
                8.0,
                4.0,
            )
            .translate(
                centered_index(i, 8, 32.0),
                CUSTODY_Y / 2.0 - 54.0,
                CUSTODY_Z / 2.0 + 4.0,
            );
    }
    lands
}

fn camera_evidence_bridge_robot_datums() -> Part {
    let rear_beam = centered_cube(
        format!("{OUTPUT_PREFIX}_rear_camera_evidence_bridge_beam"),
        CAMERA_BRIDGE_X,
        CAMERA_BRIDGE_Y,
        24.0,
    )
    .translate(0.0, DECK_Y / 2.0 - 86.0, CAMERA_BRIDGE_Z / 2.0);
    let front_beam = centered_cube(
        format!("{OUTPUT_PREFIX}_front_robot_datum_bridge_beam"),
        CAMERA_BRIDGE_X,
        28.0,
        20.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 86.0, CAMERA_BRIDGE_Z / 2.0 - 36.0);
    let posts = bridge_posts();

    rear_beam
        + front_beam
        + posts
        + camera_pods()
        + evidence_light_bars()
        + robot_service_datums()
        + robot_service_keepout_rails()
}

fn bridge_posts() -> Part {
    let mut posts = Part::empty(format!("{OUTPUT_PREFIX}_camera_bridge_posts"));
    for (i, (x, y)) in [
        (-CAMERA_BRIDGE_X / 2.0 + 34.0, DECK_Y / 2.0 - 86.0),
        (CAMERA_BRIDGE_X / 2.0 - 34.0, DECK_Y / 2.0 - 86.0),
        (-CAMERA_BRIDGE_X / 2.0 + 34.0, -DECK_Y / 2.0 + 86.0),
        (CAMERA_BRIDGE_X / 2.0 - 34.0, -DECK_Y / 2.0 + 86.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_evidence_bridge_post_{i}"),
                24.0,
                24.0,
                CAMERA_BRIDGE_Z,
            )
            .translate(*x, *y, 0.0);
    }
    posts
}

fn camera_pods() -> Part {
    let mut pods = Part::empty(format!("{OUTPUT_PREFIX}_camera_pods"));
    for i in 0..CAMERA_PODS {
        let x = centered_index(i, CAMERA_PODS, 390.0);
        let pod = centered_cube(
            format!("{OUTPUT_PREFIX}_evidence_camera_pod_{i}"),
            72.0,
            48.0,
            42.0,
        )
        .translate(x, DECK_Y / 2.0 - 128.0, CAMERA_BRIDGE_Z / 2.0 - 44.0);
        let lens = centered_cylinder(
            format!("{OUTPUT_PREFIX}_evidence_camera_lens_clearance_{i}"),
            13.0,
            18.0,
            28,
        )
        .translate(x, DECK_Y / 2.0 - 128.0, CAMERA_BRIDGE_Z / 2.0 - 68.0);
        pods = pods + (pod - lens);
    }
    pods
}

fn evidence_light_bars() -> Part {
    let mut bars = Part::empty(format!("{OUTPUT_PREFIX}_evidence_light_bars"));
    for i in 0..EVIDENCE_LIGHT_BARS {
        bars = bars
            + centered_cube(
                format!("{OUTPUT_PREFIX}_evidence_light_bar_{i}"),
                CAMERA_BRIDGE_X - 250.0,
                12.0,
                12.0,
            )
            .translate(
                0.0,
                DECK_Y / 2.0 - 156.0 - i as f64 * 22.0,
                CAMERA_BRIDGE_Z / 2.0 - 58.0,
            );
    }
    bars
}

fn robot_service_datums() -> Part {
    let mut datums = Part::empty(format!("{OUTPUT_PREFIX}_robot_service_datums"));
    for i in 0..ROBOT_SERVICE_DATUMS {
        datums = datums
            + fiducial_disc(&format!("{OUTPUT_PREFIX}_robot_service_datum_{i}")).translate(
                centered_index(i, ROBOT_SERVICE_DATUMS, 210.0),
                -DECK_Y / 2.0 + 86.0,
                CAMERA_BRIDGE_Z / 2.0 - 58.0,
            );
    }
    datums
}

fn robot_service_keepout_rails() -> Part {
    let mut rails = Part::empty(format!("{OUTPUT_PREFIX}_robot_service_keepout_rails"));
    for (i, (name, x, y, width, depth)) in [
        (
            "front_robot_sweep",
            0.0,
            -DECK_Y / 2.0 + 48.0,
            DECK_X - 160.0,
            8.0,
        ),
        (
            "rear_tube_service",
            0.0,
            DECK_Y / 2.0 - 48.0,
            DECK_X - 160.0,
            8.0,
        ),
        (
            "left_filter_coupon_pull",
            -DECK_X / 2.0 + 58.0,
            0.0,
            8.0,
            DECK_Y - 170.0,
        ),
        (
            "right_camera_service",
            DECK_X / 2.0 - 58.0,
            0.0,
            8.0,
            DECK_Y - 170.0,
        ),
        ("center_bridge_lift", 0.0, 0.0, CAMERA_BRIDGE_X - 180.0, 8.0),
    ]
    .iter()
    .enumerate()
    {
        rails = rails
            + centered_cube(
                format!("{OUTPUT_PREFIX}_keepout_{i}_{name}"),
                *width,
                *depth,
                6.0,
            )
            .translate(*x, *y, CAMERA_BRIDGE_Z / 2.0 - 72.0);
    }
    rails
}

fn sample_flow_route_harness() -> Part {
    let mut routes = Part::empty(format!("{OUTPUT_PREFIX}_closed_sample_flow_route_harness"));
    let segments = [
        (LOOP_CENTER.0 + 160.0, LOOP_CENTER.1, 250.0, 0.0),
        (FILTER_CENTER.0 + 166.0, FILTER_CENTER.1, 250.0, 0.0),
        (OPTICAL_CENTER.0, 80.0, 210.0, 90.0),
        (PRESSURE_CENTER.0, 80.0, 210.0, 90.0),
        (PRESSURE_CENTER.0 + 160.0, PRESSURE_CENTER.1, 250.0, 0.0),
        (BACKFLUSH_CENTER.0 + 160.0, BACKFLUSH_CENTER.1, 250.0, 0.0),
        (RETAIN_CENTER.0, -190.0, 130.0, 90.0),
        (BUBBLE_CENTER.0 + 160.0, BUBBLE_CENTER.1, 250.0, 0.0),
        (WASTE_CENTER.0 + 160.0, WASTE_CENTER.1, 250.0, 0.0),
        (CUSTODY_CENTER.0 - 150.0, CUSTODY_CENTER.1, 110.0, 0.0),
        (0.0, -200.0, 260.0, 90.0),
        (-250.0, 70.0, 330.0, 90.0),
    ];

    for (i, (x, y, length, rotation)) in segments.iter().enumerate() {
        let tube = if *rotation == 0.0 {
            tube_x(
                &format!("{OUTPUT_PREFIX}_route_segment_{i}"),
                *length,
                ROUTE_TUBE_D,
            )
        } else {
            tube_y(
                &format!("{OUTPUT_PREFIX}_route_segment_{i}"),
                *length,
                ROUTE_TUBE_D,
            )
        };
        routes = routes + tube.translate(*x, *y, ROUTE_Z);
    }

    for i in 0..ROUTE_DIRECTION_MARKERS {
        routes = routes
            + flow_arrow_marker(
                format!("{OUTPUT_PREFIX}_route_direction_marker_{i}"),
                1.0,
                0.0,
            )
            .translate(
                centered_index(i, ROUTE_DIRECTION_MARKERS, 110.0),
                -12.0,
                ROUTE_Z + 7.0,
            );
    }
    routes
}

fn bottle_nest(name: &str) -> Part {
    let cup = centered_cylinder(format!("{name}_cup"), 18.0, 16.0, 32);
    let clearance = centered_cylinder(format!("{name}_clearance"), 11.0, 18.0, 28);
    cup - clearance
}

fn tube_x(name: &str, length: f64, diameter: f64) -> Part {
    centered_cylinder(name, diameter / 2.0, length, 24).rotate(0.0, 90.0, 0.0)
}

fn tube_y(name: &str, length: f64, diameter: f64) -> Part {
    centered_cylinder(name, diameter / 2.0, length, 24).rotate(90.0, 0.0, 0.0)
}

fn flow_arrow_marker(name: String, dir_x: f64, dir_y: f64) -> Part {
    let shaft = if dir_y.abs() > dir_x.abs() {
        centered_cube(format!("{name}_shaft"), 5.0, 22.0, 4.0)
    } else {
        centered_cube(format!("{name}_shaft"), 22.0, 5.0, 4.0)
    };
    let head = centered_cube(format!("{name}_head"), 10.0, 10.0, 4.0).translate(
        dir_x * 14.0,
        dir_y * 14.0,
        0.0,
    );
    shaft + head
}

fn fiducial_disc(name: &str) -> Part {
    centered_cylinder(format!("{name}_outer_disc"), 7.0, 3.0, 32)
        - centered_cylinder(format!("{name}_center_clearance"), 2.0, 4.0, 20)
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
        assert!(OUTPUTS.iter().all(|path| path.starts_with(
            "output/closed_cell_suspension_clump_filter_viability_loss_sentinel_station_"
        )));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
        assert!(OUTPUTS[0].ends_with("_leak_tray_deck.stl"));
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }

    #[test]
    fn required_features_cover_user_requested_station_elements() {
        for feature in [
            "sealed_sample_loop_inlet",
            "gentle_inline_sieve_filter_coupon_ladder",
            "optical_clump_witness_windows",
            "pressure_drop_tap_ladder",
            "backflush_recovery_route",
            "live_dead_retain_sample_pockets",
            "bubble_dead_volume_windows",
            "waste_reject_split",
            "barcode_custody_lands",
            "release_hold_reject_gates",
            "camera_evidence_bridge",
            "robotic_service_datums",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature), "{feature}");
        }
        assert_eq!(REQUIRED_FEATURES.len(), 16);
    }

    #[test]
    fn station_dimensions_and_modules_fit_without_overlap() {
        assert!(DECK_X <= 1500.0);
        assert!(DECK_Y <= 900.0);
        assert!(RIM_Z >= 40.0);
        assert_eq!(module_specs().len(), 9);
        assert_design_constraints();
    }

    #[test]
    fn sieve_filter_pressure_and_backflush_counts_are_pinned() {
        assert_eq!(SIEVE_STAGES, 5);
        assert_eq!(COUPONS_PER_STAGE, 2);
        assert_eq!(FILTER_COUPONS, 10);
        assert_eq!(
            FILTER_STAGE_LABELS,
            ["coarse", "guard", "medium", "fine", "sentinel"]
        );
        assert_eq!(PRESSURE_TAP_POINTS, SIEVE_STAGES + 1);
        assert_eq!(PRESSURE_TAP_PAIRS, 6);
        assert_eq!(DP_REFERENCE_RESTRICTORS, SIEVE_STAGES);
        assert_eq!(BACKFLUSH_BRANCHES, SIEVE_STAGES);
        assert_eq!(RECOVERY_VIAL_NESTS, SIEVE_STAGES);
        assert_eq!(RECOVERY_ROUTE_SEGMENTS, 8);
    }

    #[test]
    fn optical_viability_and_dead_volume_evidence_counts_are_explicit() {
        assert_eq!(OPTICAL_CLUMP_WINDOWS, 6);
        assert_eq!(CLUMP_SIZE_BANDS, 5);
        assert_eq!(OPTICAL_LIGHT_BARS, 2);
        assert_eq!(WINDOW_PATH_LENGTH_MM, 8.0);
        assert_eq!(LIVE_DEAD_RETAIN_PAIRS, 4);
        assert_eq!(RETAIN_SAMPLE_POCKETS, 8);
        assert_eq!(BUBBLE_WINDOWS, 4);
        assert_eq!(DEAD_VOLUME_WINDOWS, 4);
        assert_eq!(AIR_GAP_SENTINELS, 3);
        assert_eq!(DEAD_VOLUME_TICK_MARKS, 9);
    }

    #[test]
    fn custody_release_and_waste_routes_match_disposition_model() {
        assert_eq!(DISPOSITION_LANES, ["release", "hold", "reject"]);
        assert_eq!(GATE_SLOTS_PER_LANE, 4);
        assert_eq!(GATE_SOLENOIDS, 6);
        assert_eq!(WASTE_REJECT_LANES, 2);
        assert_eq!(WASTE_BOTTLE_NESTS, 2);
        assert_eq!(REJECT_BOTTLE_NESTS, 2);
        assert_eq!(QUARANTINE_LOCKS, 3);
        assert_eq!(BARCODE_LANDS, 12);
        assert_eq!(RFID_LANDS, 4);
        assert_eq!(CUSTODY_SEAL_LANDS, 6);
    }

    #[test]
    fn closed_loop_and_robot_evidence_interfaces_are_sized() {
        assert_eq!(SAMPLE_LOOP_INLET_PORTS, 2);
        assert_eq!(SAMPLE_LOOP_RETURN_PORTS, 2);
        assert_eq!(SAMPLE_BRANCH_PORTS, 4);
        assert_eq!(LOOP_INTERLOCK_VALVES, 8);
        assert_eq!(LOOP_PRESSURE_SEAL_TAPS, 4);
        assert_eq!(STERILE_CAP_LANDS, 4);
        assert_eq!(ROUTE_SEGMENTS, 12);
        assert_eq!(ROUTE_DIRECTION_MARKERS, 10);
        assert_eq!(CAMERA_PODS, 3);
        assert_eq!(EVIDENCE_LIGHT_BARS, 2);
        assert_eq!(ROBOT_SERVICE_DATUMS, 6);
        assert_eq!(ROBOT_SERVICE_KEEP_OUTS, 5);
        assert!(CAMERA_CLEARANCE_Z >= 175.0);
        assert!(FRONT_ROBOT_CLEARANCE >= 300.0);
    }
}
