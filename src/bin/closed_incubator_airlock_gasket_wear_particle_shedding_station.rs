use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed incubator door/airlock gasket wear and particle-shedding validation
// station.
//
// This standalone CAD generator packages a contained boundary station for
// repeated tissue-chip cassette transfers across a closed incubator/airlock
// gasket interface. The fixture keeps gasket coupon cassettes, compression
// cycle witness rails, pressure-decay ports, particle collection coupons,
// wipe/contact sampling pockets, latch-force witnesses, transfer-tongue datum
// checks, clean/used segregation, traceability, imaging fiducials, and
// release/hold/reject evidence lanes in one source-only module. It models
// validation geometry only; leak limits, particle specifications, cleaning
// agents, sampling recipes, and disposition criteria remain external controls.

const OUTPUT_PREFIX: &str = "closed_incubator_airlock_gasket_wear_particle_shedding_station";

const OUTPUTS: [&str; 13] = [
    "output/closed_incubator_airlock_gasket_wear_particle_shedding_station_containment_deck.stl",
    "output/closed_incubator_airlock_gasket_wear_particle_shedding_station_door_gasket_coupon_cassette.stl",
    "output/closed_incubator_airlock_gasket_wear_particle_shedding_station_compression_cycle_witness_rail.stl",
    "output/closed_incubator_airlock_gasket_wear_particle_shedding_station_pressure_decay_leak_port_manifold.stl",
    "output/closed_incubator_airlock_gasket_wear_particle_shedding_station_particle_collection_coupon_lanes.stl",
    "output/closed_incubator_airlock_gasket_wear_particle_shedding_station_wipe_contact_sampling_pockets.stl",
    "output/closed_incubator_airlock_gasket_wear_particle_shedding_station_cassette_transfer_tongue_datum.stl",
    "output/closed_incubator_airlock_gasket_wear_particle_shedding_station_latch_force_witness_pockets.stl",
    "output/closed_incubator_airlock_gasket_wear_particle_shedding_station_barcode_cycle_count_lands.stl",
    "output/closed_incubator_airlock_gasket_wear_particle_shedding_station_clean_used_segregation.stl",
    "output/closed_incubator_airlock_gasket_wear_particle_shedding_station_release_hold_reject_disposition_lanes.stl",
    "output/closed_incubator_airlock_gasket_wear_particle_shedding_station_camera_illumination_fiducials.stl",
    "output/closed_incubator_airlock_gasket_wear_particle_shedding_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 12] = [
    "containment_deck",
    "door_gasket_coupon_cassette",
    "compression_cycle_witness_rail",
    "pressure_decay_leak_ports",
    "particle_collection_coupon_lanes",
    "wipe_contact_sampling_pockets",
    "cassette_transfer_tongue_datum",
    "latch_force_witness_pockets",
    "barcode_cycle_count_lands",
    "camera_illumination_fiducials",
    "clean_used_segregation",
    "release_hold_reject_disposition",
];

const STATION_X: f64 = 1500.0;
const STATION_Y: f64 = 980.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 52.0;
const SOCKET_DEPTH: f64 = 6.0;
const BASIN_RECESS_Z: f64 = 7.0;
const MODULE_GAP: f64 = 22.0;
const MOUNT_SLOT_COUNT: usize = 10;
const DECK_DATUM_TARGET_COUNT: usize = 6;
const DRAIN_PORT_D: f64 = 12.0;

const COUPON_CASSETTE_X: f64 = 400.0;
const COUPON_CASSETTE_Y: f64 = 230.0;
const COUPON_CASSETTE_Z: f64 = 48.0;
const COUPON_CASSETTE_POS: (f64, f64) = (-485.0, 270.0);
const GASKET_COUPON_ROWS: usize = 3;
const GASKET_COUPON_COLS: usize = 4;
const GASKET_COUPON_COUNT: usize = GASKET_COUPON_ROWS * GASKET_COUPON_COLS;
const GASKET_COUPON_X: f64 = 62.0;
const GASKET_COUPON_Y: f64 = 38.0;
const GASKET_COUPON_PITCH_X: f64 = 82.0;
const GASKET_COUPON_PITCH_Y: f64 = 56.0;
const GASKET_FRAME_W: f64 = 7.5;
const COUPON_ORIENTATION_KEY_D: f64 = 7.0;

const CYCLE_RAIL_X: f64 = 440.0;
const CYCLE_RAIL_Y: f64 = 230.0;
const CYCLE_RAIL_Z: f64 = 42.0;
const CYCLE_RAIL_POS: (f64, f64) = (-20.0, 270.0);
const CYCLE_WITNESS_COUNT: usize = 16;
const CYCLE_TOKEN_PITCH_X: f64 = 23.5;
const COMPRESSION_TRACK_COUNT: usize = 4;
const COMPRESSION_STEP_COUNT: usize = 5;
const COMPRESSION_STEP_DELTA: f64 = 0.55;
const TRANSFER_CYCLES_PER_TOKEN: usize = 50;

const PRESSURE_MANIFOLD_X: f64 = 390.0;
const PRESSURE_MANIFOLD_Y: f64 = 230.0;
const PRESSURE_MANIFOLD_Z: f64 = 54.0;
const PRESSURE_MANIFOLD_POS: (f64, f64) = (455.0, 270.0);
const PRESSURE_PORT_ROWS: usize = GASKET_COUPON_ROWS;
const PRESSURE_PORT_COLS: usize = GASKET_COUPON_COLS;
const PRESSURE_DECAY_PORT_COUNT: usize = PRESSURE_PORT_ROWS * PRESSURE_PORT_COLS;
const PRESSURE_PORT_PITCH_X: f64 = 74.0;
const PRESSURE_PORT_PITCH_Y: f64 = 46.0;
const PRESSURE_PORT_D: f64 = 7.2;
const PRESSURE_HEADER_D: f64 = 17.0;

const PARTICLE_LANE_X: f64 = 400.0;
const PARTICLE_LANE_Y: f64 = 190.0;
const PARTICLE_LANE_Z: f64 = 44.0;
const PARTICLE_LANE_POS: (f64, f64) = (-485.0, 20.0);
const PARTICLE_LANE_COUNT: usize = 6;
const PARTICLE_COUPONS_PER_LANE: usize = 2;
const PARTICLE_COLLECTION_COUPON_COUNT: usize = PARTICLE_LANE_COUNT * PARTICLE_COUPONS_PER_LANE;
const PARTICLE_LANE_PITCH_Y: f64 = 26.0;
const PARTICLE_COUPON_X: f64 = 64.0;
const PARTICLE_COUPON_Y: f64 = 18.0;
const PARTICLE_GRID_TICK_COUNT: usize = 7;

const SAMPLING_PANEL_X: f64 = 440.0;
const SAMPLING_PANEL_Y: f64 = 190.0;
const SAMPLING_PANEL_Z: f64 = 38.0;
const SAMPLING_PANEL_POS: (f64, f64) = (-20.0, 20.0);
const WIPE_POCKET_COUNT: usize = 4;
const CONTACT_POCKET_COUNT: usize = 4;
const SAMPLING_POCKET_COUNT: usize = WIPE_POCKET_COUNT + CONTACT_POCKET_COUNT;
const WIPE_POCKET_X: f64 = 70.0;
const WIPE_POCKET_Y: f64 = 34.0;
const CONTACT_POCKET_D: f64 = 42.0;

const TRANSFER_DATUM_X: f64 = 390.0;
const TRANSFER_DATUM_Y: f64 = 190.0;
const TRANSFER_DATUM_Z: f64 = 46.0;
const TRANSFER_DATUM_POS: (f64, f64) = (455.0, 20.0);
const TRANSFER_TONGUE_X: f64 = 236.0;
const TRANSFER_TONGUE_Y: f64 = 66.0;
const TRANSFER_TONGUE_DEPTH: f64 = 16.0;
const KINEMATIC_DATUM_COUNT: usize = 3;
const TISSUE_CHIP_CASSETTES_PER_TRANSFER: usize = 4;
const TISSUE_CHIPS_PER_CASSETTE: usize = 6;
const TISSUE_CHIP_SURROGATE_COUNT: usize =
    TISSUE_CHIP_CASSETTES_PER_TRANSFER * TISSUE_CHIPS_PER_CASSETTE;

const LATCH_PANEL_X: f64 = 400.0;
const LATCH_PANEL_Y: f64 = 190.0;
const LATCH_PANEL_Z: f64 = 46.0;
const LATCH_PANEL_POS: (f64, f64) = (-485.0, -225.0);
const LATCH_FORCE_POCKET_COUNT: usize = 6;
const LATCH_FORCE_ROWS: usize = 2;
const LATCH_FORCE_COLS: usize = 3;
const LATCH_FORCE_PITCH_X: f64 = 92.0;
const LATCH_FORCE_PITCH_Y: f64 = 64.0;
const FORCE_FILM_SLOT_COUNT: usize = 6;

const TRACE_PANEL_X: f64 = 440.0;
const TRACE_PANEL_Y: f64 = 190.0;
const TRACE_PANEL_Z: f64 = 18.0;
const TRACE_PANEL_POS: (f64, f64) = (-20.0, -225.0);
const BARCODE_LAND_COUNT: usize = GASKET_COUPON_COUNT + 4;
const CYCLE_COUNT_LAND_COUNT: usize = CYCLE_WITNESS_COUNT;
const RUN_CARD_LAND_COUNT: usize = 3;

const SEGREGATION_X: f64 = 390.0;
const SEGREGATION_Y: f64 = 190.0;
const SEGREGATION_Z: f64 = 64.0;
const SEGREGATION_POS: (f64, f64) = (455.0, -225.0);
const CLEAN_SLOT_COUNT: usize = 6;
const USED_SLOT_COUNT: usize = 6;
const SEGREGATION_MIN_GAP: f64 = 44.0;

const DISPOSITION_X: f64 = 620.0;
const DISPOSITION_Y: f64 = 86.0;
const DISPOSITION_Z: f64 = 38.0;
const DISPOSITION_POS: (f64, f64) = (0.0, -402.0);
const DISPOSITION_LANES: usize = 3;
const DISPOSITION_SLOTS_PER_LANE: usize = 5;

const CAMERA_BRIDGE_X: f64 = 1280.0;
const CAMERA_BRIDGE_Y: f64 = 62.0;
const CAMERA_BRIDGE_Z: f64 = 28.0;
const CAMERA_BRIDGE_POS: (f64, f64) = (0.0, 426.0);
const CAMERA_CLEARANCE_Z: f64 = 226.0;
const CAMERA_COUNT: usize = 4;
const LIGHT_BAR_COUNT: usize = 2;
const CAMERA_FIDUCIAL_COUNT: usize = 10;
const CAMERA_POST_COUNT: usize = 4;

const FRONT_TRANSFER_CLEARANCE: f64 = 36.0;
const REAR_CAMERA_SERVICE_CLEARANCE: f64 = 33.0;
const SIDE_SERVICE_CLEARANCE: f64 = 100.0;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DispositionLane {
    Release,
    Hold,
    Reject,
}

impl DispositionLane {
    fn all() -> [DispositionLane; DISPOSITION_LANES] {
        [
            DispositionLane::Release,
            DispositionLane::Hold,
            DispositionLane::Reject,
        ]
    }

    fn index(self) -> usize {
        match self {
            DispositionLane::Release => 0,
            DispositionLane::Hold => 1,
            DispositionLane::Reject => 2,
        }
    }

    fn name(self) -> &'static str {
        match self {
            DispositionLane::Release => "release",
            DispositionLane::Hold => "hold",
            DispositionLane::Reject => "reject",
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_station(self) -> bool {
        let usable_x = STATION_X / 2.0 - RIM_W - 12.0;
        let usable_y = STATION_Y / 2.0 - RIM_W - 12.0;

        self.center.0.abs() + self.x / 2.0 <= usable_x
            && self.center.1.abs() + self.y / 2.0 <= usable_y
    }

    fn overlaps(self, other: Rect) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();

        dx < (self.x + other.x) / 2.0 + MODULE_GAP && dy < (self.y + other.y) / 2.0 + MODULE_GAP
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = containment_deck();
    export(OUTPUTS[0], &deck);

    let coupon_cassette = door_gasket_coupon_cassette();
    export(OUTPUTS[1], &coupon_cassette);

    let cycle_rail = compression_cycle_witness_rail();
    export(OUTPUTS[2], &cycle_rail);

    let pressure_manifold = pressure_decay_leak_port_manifold();
    export(OUTPUTS[3], &pressure_manifold);

    let particle_lanes = particle_collection_coupon_lanes();
    export(OUTPUTS[4], &particle_lanes);

    let sampling = wipe_contact_sampling_pockets();
    export(OUTPUTS[5], &sampling);

    let transfer_datum = cassette_transfer_tongue_datum();
    export(OUTPUTS[6], &transfer_datum);

    let latch_force = latch_force_witness_pockets();
    export(OUTPUTS[7], &latch_force);

    let traceability = barcode_cycle_count_lands();
    export(OUTPUTS[8], &traceability);

    let segregation = clean_used_segregation();
    export(OUTPUTS[9], &segregation);

    let disposition = release_hold_reject_disposition_lanes();
    export(OUTPUTS[10], &disposition);

    let camera = camera_illumination_fiducials();
    export(OUTPUTS[11], &camera);

    let assembly =
        deck + coupon_cassette.translate(
            COUPON_CASSETTE_POS.0,
            COUPON_CASSETTE_POS.1,
            insert_z(COUPON_CASSETTE_Z),
        ) + cycle_rail.translate(CYCLE_RAIL_POS.0, CYCLE_RAIL_POS.1, insert_z(CYCLE_RAIL_Z))
            + pressure_manifold.translate(
                PRESSURE_MANIFOLD_POS.0,
                PRESSURE_MANIFOLD_POS.1,
                insert_z(PRESSURE_MANIFOLD_Z),
            )
            + particle_lanes.translate(
                PARTICLE_LANE_POS.0,
                PARTICLE_LANE_POS.1,
                insert_z(PARTICLE_LANE_Z),
            )
            + sampling.translate(
                SAMPLING_PANEL_POS.0,
                SAMPLING_PANEL_POS.1,
                insert_z(SAMPLING_PANEL_Z),
            )
            + transfer_datum.translate(
                TRANSFER_DATUM_POS.0,
                TRANSFER_DATUM_POS.1,
                insert_z(TRANSFER_DATUM_Z),
            )
            + latch_force.translate(
                LATCH_PANEL_POS.0,
                LATCH_PANEL_POS.1,
                insert_z(LATCH_PANEL_Z),
            )
            + traceability.translate(
                TRACE_PANEL_POS.0,
                TRACE_PANEL_POS.1,
                insert_z(TRACE_PANEL_Z),
            )
            + segregation.translate(
                SEGREGATION_POS.0,
                SEGREGATION_POS.1,
                insert_z(SEGREGATION_Z),
            )
            + disposition.translate(
                DISPOSITION_POS.0,
                DISPOSITION_POS.1,
                insert_z(DISPOSITION_Z),
            )
            + camera.translate(CAMERA_BRIDGE_POS.0, CAMERA_BRIDGE_POS.1, BASE_Z);
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed incubator airlock gasket wear / particle-shedding station:");
    println!(
        "  Footprint:                  {STATION_X:.0}mm x {STATION_Y:.0}mm contained incubator-boundary deck"
    );
    println!(
        "  Gasket coupons:             {GASKET_COUPON_ROWS}x{GASKET_COUPON_COLS} door/airlock gasket coupon cassette, {GASKET_COUPON_COUNT} coupons"
    );
    println!(
        "  Transfer scale:             {TISSUE_CHIP_CASSETTES_PER_TRANSFER} cassette transfer datum lanes, {TISSUE_CHIP_SURROGATE_COUNT} tissue-chip surrogate positions"
    );
    println!(
        "  Wear cycle evidence:        {CYCLE_WITNESS_COUNT} cycle-count witness lands at {TRANSFER_CYCLES_PER_TOKEN} transfers/token, {COMPRESSION_TRACK_COUNT} compression tracks"
    );
    println!(
        "  Leak and particles:         {PRESSURE_DECAY_PORT_COUNT} pressure-decay ports, {PARTICLE_COLLECTION_COUPON_COUNT} particle collection coupons, {SAMPLING_POCKET_COUNT} wipe/contact pockets"
    );
    println!(
        "  Latch/trace/disposition:    {LATCH_FORCE_POCKET_COUNT} latch-force pockets, {BARCODE_LAND_COUNT} barcode lands, {DISPOSITION_LANES} release/hold/reject lanes"
    );
    println!(
        "  Evidence capture:           {CAMERA_COUNT} camera fiducial bosses, {LIGHT_BAR_COUNT} illumination rails, {CAMERA_FIDUCIAL_COUNT} field fiducials"
    );
    println!("  Required feature groups:    {}", REQUIRED_FEATURES.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn insert_z(part_z: f64) -> f64 {
    BASE_Z - SOCKET_DEPTH + part_z / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 13);
    assert_eq!(REQUIRED_FEATURES.len(), 12);
    assert_eq!(GASKET_COUPON_COUNT, GASKET_COUPON_ROWS * GASKET_COUPON_COLS);
    assert_eq!(PRESSURE_DECAY_PORT_COUNT, GASKET_COUPON_COUNT);
    assert_eq!(
        PARTICLE_COLLECTION_COUPON_COUNT,
        PARTICLE_LANE_COUNT * PARTICLE_COUPONS_PER_LANE
    );
    assert_eq!(
        TISSUE_CHIP_SURROGATE_COUNT,
        TISSUE_CHIP_CASSETTES_PER_TRANSFER * TISSUE_CHIPS_PER_CASSETTE
    );
    assert_eq!(
        LATCH_FORCE_POCKET_COUNT,
        LATCH_FORCE_ROWS * LATCH_FORCE_COLS
    );
    assert_eq!(DISPOSITION_LANES, DispositionLane::all().len());
    assert_eq!(KINEMATIC_DATUM_COUNT, 3);
    assert_eq!(DECK_DATUM_TARGET_COUNT, 6);
    assert_eq!(CAMERA_POST_COUNT, 4);
    assert_eq!(CLEAN_SLOT_COUNT, USED_SLOT_COUNT);
    assert!(OUTPUTS.iter().all(|path| path.starts_with("output/")));
    assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
    assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
    assert!(cycle_witness_transfer_capacity() >= 800);
    assert!(particle_collection_area_mm2() >= 9000.0);
    assert!(clean_used_gap_mm() >= SEGREGATION_MIN_GAP);
    assert!(compression_step_range_mm() >= 2.0);
    assert!(front_transfer_clearance_mm() >= FRONT_TRANSFER_CLEARANCE);
    assert!(rear_camera_service_clearance_mm() >= REAR_CAMERA_SERVICE_CLEARANCE);
    assert!(side_service_clearance_mm() >= SIDE_SERVICE_CLEARANCE);
    assert!(CAMERA_CLEARANCE_Z > SEGREGATION_Z + BASE_Z + 120.0);

    let modules = module_rects();
    for module in modules {
        assert!(
            module.fits_inside_station(),
            "{} exceeds station usable deck",
            module.name
        );
    }

    for (left_index, left) in modules.iter().enumerate() {
        for right in modules.iter().skip(left_index + 1) {
            if left.name == "camera_illumination_fiducials"
                || right.name == "camera_illumination_fiducials"
            {
                continue;
            }
            assert!(
                !left.overlaps(*right),
                "{} overlaps {}",
                left.name,
                right.name
            );
        }
    }
}

fn module_rects() -> [Rect; 11] {
    [
        rect(
            "door_gasket_coupon_cassette",
            COUPON_CASSETTE_POS,
            COUPON_CASSETTE_X,
            COUPON_CASSETTE_Y,
        ),
        rect(
            "compression_cycle_witness_rail",
            CYCLE_RAIL_POS,
            CYCLE_RAIL_X,
            CYCLE_RAIL_Y,
        ),
        rect(
            "pressure_decay_leak_port_manifold",
            PRESSURE_MANIFOLD_POS,
            PRESSURE_MANIFOLD_X,
            PRESSURE_MANIFOLD_Y,
        ),
        rect(
            "particle_collection_coupon_lanes",
            PARTICLE_LANE_POS,
            PARTICLE_LANE_X,
            PARTICLE_LANE_Y,
        ),
        rect(
            "wipe_contact_sampling_pockets",
            SAMPLING_PANEL_POS,
            SAMPLING_PANEL_X,
            SAMPLING_PANEL_Y,
        ),
        rect(
            "cassette_transfer_tongue_datum",
            TRANSFER_DATUM_POS,
            TRANSFER_DATUM_X,
            TRANSFER_DATUM_Y,
        ),
        rect(
            "latch_force_witness_pockets",
            LATCH_PANEL_POS,
            LATCH_PANEL_X,
            LATCH_PANEL_Y,
        ),
        rect(
            "barcode_cycle_count_lands",
            TRACE_PANEL_POS,
            TRACE_PANEL_X,
            TRACE_PANEL_Y,
        ),
        rect(
            "clean_used_segregation",
            SEGREGATION_POS,
            SEGREGATION_X,
            SEGREGATION_Y,
        ),
        rect(
            "release_hold_reject_disposition",
            DISPOSITION_POS,
            DISPOSITION_X,
            DISPOSITION_Y,
        ),
        rect(
            "camera_illumination_fiducials",
            CAMERA_BRIDGE_POS,
            CAMERA_BRIDGE_X,
            CAMERA_BRIDGE_Y,
        ),
    ]
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn cycle_witness_transfer_capacity() -> usize {
    CYCLE_WITNESS_COUNT * TRANSFER_CYCLES_PER_TOKEN
}

fn particle_collection_area_mm2() -> f64 {
    PARTICLE_COLLECTION_COUPON_COUNT as f64 * PARTICLE_COUPON_X * PARTICLE_COUPON_Y
}

fn clean_used_gap_mm() -> f64 {
    74.0
}

fn compression_step_range_mm() -> f64 {
    (COMPRESSION_STEP_COUNT as f64 - 1.0) * COMPRESSION_STEP_DELTA
}

fn front_transfer_clearance_mm() -> f64 {
    STATION_Y / 2.0 - (DISPOSITION_POS.1.abs() + DISPOSITION_Y / 2.0)
}

fn rear_camera_service_clearance_mm() -> f64 {
    STATION_Y / 2.0 - (CAMERA_BRIDGE_POS.1 + CAMERA_BRIDGE_Y / 2.0)
}

fn side_service_clearance_mm() -> f64 {
    STATION_X / 2.0 - (PRESSURE_MANIFOLD_POS.0 + PRESSURE_MANIFOLD_X / 2.0)
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        "airlock_gasket_shedding_containment_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let shallow_basin = centered_cube(
        "airlock_gasket_shedding_wipeable_secondary_basin",
        STATION_X - 140.0,
        STATION_Y - 126.0,
        BASIN_RECESS_Z + 1.0,
    )
    .translate(0.0, -10.0, BASE_Z - BASIN_RECESS_Z / 2.0);
    let particle_sump = centered_cube(
        "airlock_gasket_shedding_front_particle_sump_gutter",
        STATION_X - 210.0,
        34.0,
        9.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 60.0, BASE_Z - 4.5);
    let drain = centered_cylinder(
        "airlock_gasket_shedding_closed_drain_witness_port",
        DRAIN_PORT_D / 2.0,
        54.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 80.0,
        -STATION_Y / 2.0 + 40.0,
        BASE_Z - 7.0,
    );

    deck - shallow_basin - particle_sump - drain - module_locator_sockets() - mounting_slots()
        + containment_rims()
        + workflow_zone_dividers()
        + deck_datum_targets()
        + incubator_boundary_reference()
        + cassette_transfer_flow_arrows()
}

fn module_locator_sockets() -> Part {
    let mut sockets = Part::empty("airlock_gasket_shedding_module_locator_sockets");
    for module in module_rects()
        .into_iter()
        .filter(|module| module.name != "camera_illumination_fiducials")
    {
        sockets = sockets
            + centered_cube(
                format!("airlock_gasket_shedding_{}_socket", module.name),
                module.x + 10.0,
                module.y + 10.0,
                SOCKET_DEPTH + 0.6,
            )
            .translate(
                module.center.0,
                module.center.1,
                BASE_Z - SOCKET_DEPTH / 2.0 + 0.3,
            );
    }
    sockets
}

fn mounting_slots() -> Part {
    let mut slots = Part::empty("airlock_gasket_shedding_mounting_slots");
    for (i, (x, y)) in mount_slot_positions().into_iter().enumerate() {
        let round = centered_cylinder(
            format!("airlock_gasket_shedding_m6_mount_round_{i}"),
            3.6,
            BASE_Z + 5.0,
            28,
        )
        .translate(x, y, BASE_Z / 2.0);
        let slot = centered_cube(
            format!("airlock_gasket_shedding_m6_mount_slot_{i}"),
            30.0,
            7.2,
            BASE_Z + 5.0,
        )
        .translate(x, y, BASE_Z / 2.0);
        slots = slots + round + slot;
    }
    slots
}

fn mount_slot_positions() -> [(f64, f64); MOUNT_SLOT_COUNT] {
    [
        (-690.0, -430.0),
        (-345.0, -430.0),
        (0.0, -430.0),
        (345.0, -430.0),
        (690.0, -430.0),
        (-690.0, 430.0),
        (-345.0, 430.0),
        (0.0, 430.0),
        (345.0, 430.0),
        (690.0, 430.0),
    ]
}

fn containment_rims() -> Part {
    let front = centered_cube(
        "airlock_gasket_shedding_front_containment_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        "airlock_gasket_shedding_rear_incubator_boundary_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "airlock_gasket_shedding_left_clean_entry_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "airlock_gasket_shedding_right_used_exit_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);

    front + rear + left + right
}

fn workflow_zone_dividers() -> Part {
    let top_boundary = centered_cube(
        "airlock_gasket_shedding_coupon_to_instrument_zone_divider",
        STATION_X - 170.0,
        10.0,
        25.0,
    )
    .translate(0.0, 145.0, BASE_Z + 12.5);
    let middle_boundary = centered_cube(
        "airlock_gasket_shedding_instrument_to_sampling_zone_divider",
        STATION_X - 180.0,
        10.0,
        25.0,
    )
    .translate(0.0, -100.0, BASE_Z + 12.5);
    let lower_boundary = centered_cube(
        "airlock_gasket_shedding_sampling_to_disposition_zone_divider",
        STATION_X - 280.0,
        10.0,
        25.0,
    )
    .translate(0.0, -342.0, BASE_Z + 12.5);
    let clean_used_boundary = centered_cube(
        "airlock_gasket_shedding_clean_used_flow_boundary",
        10.0,
        210.0,
        32.0,
    )
    .translate(248.0, -225.0, BASE_Z + 16.0);

    top_boundary + middle_boundary + lower_boundary + clean_used_boundary
}

fn deck_datum_targets() -> Part {
    let mut targets = Part::empty("airlock_gasket_shedding_deck_datum_targets");
    for (i, (x, y)) in [
        (-660.0, -408.0),
        (0.0, -408.0),
        (660.0, -408.0),
        (-660.0, 408.0),
        (0.0, 408.0),
        (660.0, 408.0),
    ]
    .into_iter()
    .enumerate()
    {
        let outer = centered_cylinder(
            format!("airlock_gasket_shedding_robot_datum_outer_{i}"),
            13.0,
            5.0,
            36,
        )
        .translate(x, y, BASE_Z + 2.5);
        let center = centered_cylinder(
            format!("airlock_gasket_shedding_robot_datum_center_{i}"),
            2.1,
            6.0,
            20,
        )
        .translate(x, y, BASE_Z + 3.0);
        targets = targets + (outer - center);
    }
    targets
}

fn incubator_boundary_reference() -> Part {
    let boundary_bar = centered_cube(
        "airlock_gasket_shedding_incubator_airlock_boundary_reference_bar",
        STATION_X - 220.0,
        12.0,
        34.0,
    )
    .translate(0.0, 390.0, BASE_Z + 17.0);
    let seal_shadow = centered_cube(
        "airlock_gasket_shedding_projected_closed_door_seal_shadow",
        STATION_X - 320.0,
        6.0,
        8.0,
    )
    .translate(0.0, 360.0, BASE_Z + 4.0);
    let hinge_side = centered_cube(
        "airlock_gasket_shedding_hinge_side_boundary_marker",
        9.0,
        128.0,
        28.0,
    )
    .translate(-650.0, 326.0, BASE_Z + 14.0);
    let latch_side = centered_cube(
        "airlock_gasket_shedding_latch_side_boundary_marker",
        9.0,
        128.0,
        28.0,
    )
    .translate(650.0, 326.0, BASE_Z + 14.0);

    boundary_bar + seal_shadow + hinge_side + latch_side
}

fn cassette_transfer_flow_arrows() -> Part {
    let mut arrows = Part::empty("airlock_gasket_shedding_cassette_transfer_flow_arrows");
    for (i, y) in [322.0, 76.0, -168.0, -383.0].into_iter().enumerate() {
        let shaft = centered_cube(
            format!("airlock_gasket_shedding_transfer_flow_arrow_shaft_{i}"),
            94.0,
            4.0,
            4.0,
        )
        .translate(-710.0 + i as f64 * 46.0, y, BASE_Z + 2.0);
        let head_a = centered_cube(
            format!("airlock_gasket_shedding_transfer_flow_arrow_head_a_{i}"),
            18.0,
            4.0,
            4.0,
        )
        .rotate(0.0, 0.0, 32.0)
        .translate(-652.0 + i as f64 * 46.0, y + 5.0, BASE_Z + 2.0);
        let head_b = centered_cube(
            format!("airlock_gasket_shedding_transfer_flow_arrow_head_b_{i}"),
            18.0,
            4.0,
            4.0,
        )
        .rotate(0.0, 0.0, -32.0)
        .translate(-652.0 + i as f64 * 46.0, y - 5.0, BASE_Z + 2.0);
        arrows = arrows + shaft + head_a + head_b;
    }
    arrows
}

fn door_gasket_coupon_cassette() -> Part {
    let panel = module_panel(
        "airlock_gasket_coupon_cassette_panel",
        COUPON_CASSETTE_X,
        COUPON_CASSETTE_Y,
        COUPON_CASSETTE_Z,
    );
    let shallow_pan = centered_cube(
        "airlock_gasket_coupon_cassette_wipeable_recess",
        COUPON_CASSETTE_X - 28.0,
        COUPON_CASSETTE_Y - 28.0,
        9.0,
    )
    .translate(0.0, 0.0, COUPON_CASSETTE_Z - 4.5);

    panel - shallow_pan - gasket_coupon_recesses()
        + gasket_coupon_frames()
        + coupon_cassette_side_rails()
        + coupon_cassette_transfer_handles()
        + coupon_cassette_tissue_chip_scale_marks()
        + module_corner_fiducials(
            "airlock_gasket_coupon_cassette",
            COUPON_CASSETTE_X,
            COUPON_CASSETTE_Y,
            COUPON_CASSETTE_Z,
        )
}

fn gasket_coupon_recesses() -> Part {
    let mut recesses = Part::empty("airlock_gasket_coupon_cassette_recesses");
    for row in 0..GASKET_COUPON_ROWS {
        for col in 0..GASKET_COUPON_COLS {
            let x = centered_index(col, GASKET_COUPON_COLS, GASKET_COUPON_PITCH_X);
            let y = centered_index(row, GASKET_COUPON_ROWS, GASKET_COUPON_PITCH_Y) + 4.0;
            let index = row * GASKET_COUPON_COLS + col;
            let recess = centered_cube(
                format!("airlock_gasket_coupon_cassette_coupon_recess_{index}"),
                GASKET_COUPON_X + 10.0,
                GASKET_COUPON_Y + 10.0,
                16.0,
            )
            .translate(x, y, COUPON_CASSETTE_Z - 8.0);
            let key = centered_cylinder(
                format!("airlock_gasket_coupon_cassette_orientation_key_recess_{index}"),
                COUPON_ORIENTATION_KEY_D / 2.0,
                18.0,
                20,
            )
            .translate(
                x + GASKET_COUPON_X / 2.0 - 8.0,
                y + GASKET_COUPON_Y / 2.0 - 7.0,
                COUPON_CASSETTE_Z - 8.0,
            );
            recesses = recesses + recess + key;
        }
    }
    recesses
}

fn gasket_coupon_frames() -> Part {
    let mut frames = Part::empty("airlock_gasket_coupon_cassette_frames");
    for row in 0..GASKET_COUPON_ROWS {
        for col in 0..GASKET_COUPON_COLS {
            let x = centered_index(col, GASKET_COUPON_COLS, GASKET_COUPON_PITCH_X);
            let y = centered_index(row, GASKET_COUPON_ROWS, GASKET_COUPON_PITCH_Y) + 4.0;
            let index = row * GASKET_COUPON_COLS + col;
            let frame = rectangular_frame(
                &format!("airlock_gasket_coupon_frame_{index}"),
                GASKET_COUPON_X,
                GASKET_COUPON_Y,
                8.0,
                GASKET_FRAME_W,
            )
            .translate(x, y, COUPON_CASSETTE_Z - 2.0);
            let compression_lip = centered_cube(
                format!("airlock_gasket_coupon_compression_witness_lip_{index}"),
                GASKET_COUPON_X - 8.0,
                3.2,
                5.0,
            )
            .translate(x, y - GASKET_COUPON_Y / 2.0 - 5.0, COUPON_CASSETTE_Z + 2.5);
            let airlock_side_mark = centered_cube(
                format!("airlock_gasket_coupon_airlock_side_tick_{index}"),
                4.0,
                GASKET_COUPON_Y + 14.0,
                4.0,
            )
            .translate(x - GASKET_COUPON_X / 2.0 - 7.0, y, COUPON_CASSETTE_Z + 2.0);
            frames = frames + frame + compression_lip + airlock_side_mark;
        }
    }
    frames
}

fn coupon_cassette_side_rails() -> Part {
    let left = centered_cube(
        "airlock_gasket_coupon_cassette_left_retention_rail",
        12.0,
        COUPON_CASSETTE_Y - 36.0,
        24.0,
    )
    .translate(
        -COUPON_CASSETTE_X / 2.0 + 26.0,
        0.0,
        COUPON_CASSETTE_Z + 12.0,
    );
    let right = centered_cube(
        "airlock_gasket_coupon_cassette_right_retention_rail",
        12.0,
        COUPON_CASSETTE_Y - 36.0,
        24.0,
    )
    .translate(
        COUPON_CASSETTE_X / 2.0 - 26.0,
        0.0,
        COUPON_CASSETTE_Z + 12.0,
    );
    let hinge_edge = centered_cube(
        "airlock_gasket_coupon_cassette_hinge_edge_reference_bar",
        COUPON_CASSETTE_X - 74.0,
        10.0,
        18.0,
    )
    .translate(0.0, COUPON_CASSETTE_Y / 2.0 - 28.0, COUPON_CASSETTE_Z + 9.0);
    let latch_edge = centered_cube(
        "airlock_gasket_coupon_cassette_latch_edge_reference_bar",
        COUPON_CASSETTE_X - 74.0,
        10.0,
        18.0,
    )
    .translate(
        0.0,
        -COUPON_CASSETTE_Y / 2.0 + 28.0,
        COUPON_CASSETTE_Z + 9.0,
    );

    left + right + hinge_edge + latch_edge
}

fn coupon_cassette_transfer_handles() -> Part {
    let mut handles = Part::empty("airlock_gasket_coupon_cassette_transfer_handles");
    for (i, x) in [-135.0, 135.0].into_iter().enumerate() {
        let base = centered_cube(
            format!("airlock_gasket_coupon_cassette_handle_base_{i}"),
            84.0,
            14.0,
            16.0,
        )
        .translate(x, -COUPON_CASSETTE_Y / 2.0 + 12.0, COUPON_CASSETTE_Z + 8.0);
        let arch = centered_cylinder(
            format!("airlock_gasket_coupon_cassette_handle_arch_{i}"),
            13.0,
            84.0,
            28,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(x, -COUPON_CASSETTE_Y / 2.0 + 14.0, COUPON_CASSETTE_Z + 23.0);
        let bore = centered_cylinder(
            format!("airlock_gasket_coupon_cassette_handle_opening_{i}"),
            7.0,
            86.0,
            28,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(x, -COUPON_CASSETTE_Y / 2.0 + 14.0, COUPON_CASSETTE_Z + 23.0);
        handles = handles + base + (arch - bore);
    }
    handles
}

fn coupon_cassette_tissue_chip_scale_marks() -> Part {
    let mut marks = Part::empty("airlock_gasket_coupon_cassette_tissue_chip_scale_marks");
    for cassette in 0..TISSUE_CHIP_CASSETTES_PER_TRANSFER {
        let x = centered_index(cassette, TISSUE_CHIP_CASSETTES_PER_TRANSFER, 80.0);
        let cassette_land = centered_cube(
            format!("airlock_gasket_coupon_cassette_transfer_scale_land_{cassette}"),
            58.0,
            8.0,
            3.0,
        )
        .translate(x, COUPON_CASSETTE_Y / 2.0 - 48.0, COUPON_CASSETTE_Z + 1.5);
        marks = marks + cassette_land;
        for chip in 0..TISSUE_CHIPS_PER_CASSETTE {
            let chip_x = x - 25.0 + chip as f64 * 10.0;
            marks = marks
                + centered_cube(
                    format!("airlock_gasket_coupon_cassette_tissue_chip_tick_{cassette}_{chip}"),
                    5.0,
                    12.0,
                    3.0,
                )
                .translate(
                    chip_x,
                    COUPON_CASSETTE_Y / 2.0 - 34.0,
                    COUPON_CASSETTE_Z + 1.5,
                );
        }
    }
    marks
}

fn compression_cycle_witness_rail() -> Part {
    let panel = module_panel(
        "airlock_gasket_cycle_witness_panel",
        CYCLE_RAIL_X,
        CYCLE_RAIL_Y,
        CYCLE_RAIL_Z,
    );
    let rail_channel = centered_cube(
        "airlock_gasket_cycle_witness_repeated_compression_channel",
        CYCLE_RAIL_X - 54.0,
        54.0,
        13.0,
    )
    .translate(0.0, 38.0, CYCLE_RAIL_Z - 6.5);

    panel - rail_channel - cycle_token_pockets() - compression_track_reliefs()
        + cycle_witness_tokens()
        + compression_step_gauges()
        + transfer_count_ruler()
        + cycle_witness_guard_rails()
        + module_corner_fiducials(
            "airlock_gasket_cycle_witness",
            CYCLE_RAIL_X,
            CYCLE_RAIL_Y,
            CYCLE_RAIL_Z,
        )
}

fn cycle_token_pockets() -> Part {
    let mut pockets = Part::empty("airlock_gasket_cycle_token_pockets");
    for i in 0..CYCLE_WITNESS_COUNT {
        pockets = pockets
            + centered_cylinder(
                format!("airlock_gasket_cycle_token_pocket_{i}"),
                8.5,
                13.0,
                28,
            )
            .translate(
                centered_index(i, CYCLE_WITNESS_COUNT, CYCLE_TOKEN_PITCH_X),
                -66.0,
                CYCLE_RAIL_Z - 6.5,
            );
    }
    pockets
}

fn compression_track_reliefs() -> Part {
    let mut reliefs = Part::empty("airlock_gasket_compression_track_reliefs");
    for track in 0..COMPRESSION_TRACK_COUNT {
        reliefs = reliefs
            + centered_cube(
                format!("airlock_gasket_compression_track_relief_{track}"),
                CYCLE_RAIL_X - 96.0,
                10.0,
                9.0,
            )
            .translate(
                0.0,
                50.0 + centered_index(track, COMPRESSION_TRACK_COUNT, 20.0),
                CYCLE_RAIL_Z - 4.5,
            );
    }
    reliefs
}

fn cycle_witness_tokens() -> Part {
    let mut tokens = Part::empty("airlock_gasket_cycle_witness_tokens");
    for i in 0..CYCLE_WITNESS_COUNT {
        let x = centered_index(i, CYCLE_WITNESS_COUNT, CYCLE_TOKEN_PITCH_X);
        let token = centered_cylinder(
            format!("airlock_gasket_cycle_witness_token_ring_{i}"),
            7.5,
            4.0,
            28,
        )
        .translate(x, -66.0, CYCLE_RAIL_Z + 2.0);
        let center = centered_cylinder(
            format!("airlock_gasket_cycle_witness_token_center_{i}"),
            3.2,
            5.0,
            20,
        )
        .translate(x, -66.0, CYCLE_RAIL_Z + 2.5);
        let tick = centered_cube(
            format!("airlock_gasket_cycle_witness_increment_tick_{i}"),
            2.0,
            22.0,
            4.0,
        )
        .translate(x, -42.0, CYCLE_RAIL_Z + 2.0);
        tokens = tokens + (token - center) + tick;
    }
    tokens
}

fn compression_step_gauges() -> Part {
    let mut gauges = Part::empty("airlock_gasket_compression_step_gauges");
    for track in 0..COMPRESSION_TRACK_COUNT {
        let y = 50.0 + centered_index(track, COMPRESSION_TRACK_COUNT, 20.0);
        for step in 0..COMPRESSION_STEP_COUNT {
            let height = 3.0 + step as f64 * COMPRESSION_STEP_DELTA;
            gauges = gauges
                + centered_cube(
                    format!("airlock_gasket_compression_track_{track}_height_step_{step}"),
                    34.0,
                    8.0,
                    height,
                )
                .translate(
                    -172.0 + step as f64 * 42.0,
                    y,
                    CYCLE_RAIL_Z + height / 2.0,
                );
        }
        let full_width_pad = centered_cube(
            format!("airlock_gasket_compression_track_{track}_wear_polish_witness_pad"),
            138.0,
            8.0,
            4.0,
        )
        .translate(106.0, y, CYCLE_RAIL_Z + 2.0);
        gauges = gauges + full_width_pad;
    }
    gauges
}

fn transfer_count_ruler() -> Part {
    let mut ruler = Part::empty("airlock_gasket_transfer_count_ruler");
    for i in 0..=8 {
        let tick_h = if i % 4 == 0 { 18.0 } else { 10.0 };
        ruler = ruler
            + centered_cube(
                format!("airlock_gasket_transfer_count_ruler_tick_{i}"),
                2.4,
                tick_h,
                4.0,
            )
            .translate(-190.0 + i as f64 * 47.5, -103.0, CYCLE_RAIL_Z + 2.0);
    }
    ruler
}

fn cycle_witness_guard_rails() -> Part {
    let front = centered_cube(
        "airlock_gasket_cycle_witness_front_guard_rail",
        CYCLE_RAIL_X - 46.0,
        8.0,
        16.0,
    )
    .translate(0.0, -96.0, CYCLE_RAIL_Z + 8.0);
    let rear = centered_cube(
        "airlock_gasket_cycle_witness_rear_guard_rail",
        CYCLE_RAIL_X - 46.0,
        8.0,
        16.0,
    )
    .translate(0.0, 104.0, CYCLE_RAIL_Z + 8.0);
    front + rear
}

fn pressure_decay_leak_port_manifold() -> Part {
    let panel = module_panel(
        "airlock_gasket_pressure_decay_manifold_panel",
        PRESSURE_MANIFOLD_X,
        PRESSURE_MANIFOLD_Y,
        PRESSURE_MANIFOLD_Z,
    );
    let service_recess = centered_cube(
        "airlock_gasket_pressure_decay_tubing_service_recess",
        PRESSURE_MANIFOLD_X - 44.0,
        62.0,
        13.0,
    )
    .translate(0.0, -70.0, PRESSURE_MANIFOLD_Z - 6.5);

    panel - service_recess - pressure_port_bores() - pressure_header_bores()
        + pressure_port_bosses()
        + pressure_channel_witness_ribs()
        + pressure_reference_plugs()
        + module_corner_fiducials(
            "airlock_gasket_pressure_decay",
            PRESSURE_MANIFOLD_X,
            PRESSURE_MANIFOLD_Y,
            PRESSURE_MANIFOLD_Z,
        )
}

fn pressure_port_bores() -> Part {
    let mut bores = Part::empty("airlock_gasket_pressure_decay_port_bores");
    for row in 0..PRESSURE_PORT_ROWS {
        for col in 0..PRESSURE_PORT_COLS {
            let index = row * PRESSURE_PORT_COLS + col;
            let x = centered_index(col, PRESSURE_PORT_COLS, PRESSURE_PORT_PITCH_X);
            let y = centered_index(row, PRESSURE_PORT_ROWS, PRESSURE_PORT_PITCH_Y) + 20.0;
            bores = bores
                + centered_cylinder(
                    format!("airlock_gasket_pressure_decay_port_bore_{index}"),
                    PRESSURE_PORT_D / 2.0,
                    PRESSURE_MANIFOLD_Z + 8.0,
                    30,
                )
                .translate(x, y, PRESSURE_MANIFOLD_Z / 2.0);
        }
    }
    bores
}

fn pressure_header_bores() -> Part {
    let horizontal = centered_cylinder(
        "airlock_gasket_pressure_decay_main_header_bore",
        PRESSURE_HEADER_D / 2.0,
        PRESSURE_MANIFOLD_X - 82.0,
        36,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -72.0, PRESSURE_MANIFOLD_Z - 20.0);
    let outlet = centered_cylinder(
        "airlock_gasket_pressure_decay_quick_connect_outlet_bore",
        6.0,
        72.0,
        30,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        PRESSURE_MANIFOLD_X / 2.0 - 42.0,
        -PRESSURE_MANIFOLD_Y / 2.0 + 14.0,
        PRESSURE_MANIFOLD_Z - 20.0,
    );
    horizontal + outlet
}

fn pressure_port_bosses() -> Part {
    let mut bosses = Part::empty("airlock_gasket_pressure_decay_port_bosses");
    for row in 0..PRESSURE_PORT_ROWS {
        for col in 0..PRESSURE_PORT_COLS {
            let index = row * PRESSURE_PORT_COLS + col;
            let x = centered_index(col, PRESSURE_PORT_COLS, PRESSURE_PORT_PITCH_X);
            let y = centered_index(row, PRESSURE_PORT_ROWS, PRESSURE_PORT_PITCH_Y) + 20.0;
            let boss = centered_cylinder(
                format!("airlock_gasket_pressure_decay_port_boss_{index}"),
                13.0,
                8.0,
                36,
            )
            .translate(x, y, PRESSURE_MANIFOLD_Z + 4.0);
            let bore = centered_cylinder(
                format!("airlock_gasket_pressure_decay_port_boss_center_{index}"),
                PRESSURE_PORT_D / 2.0,
                9.0,
                28,
            )
            .translate(x, y, PRESSURE_MANIFOLD_Z + 4.5);
            bosses = bosses + (boss - bore);
        }
    }
    bosses
}

fn pressure_channel_witness_ribs() -> Part {
    let mut ribs = Part::empty("airlock_gasket_pressure_decay_channel_witness_ribs");
    for row in 0..PRESSURE_PORT_ROWS {
        let y = centered_index(row, PRESSURE_PORT_ROWS, PRESSURE_PORT_PITCH_Y) + 20.0;
        ribs = ribs
            + centered_cube(
                format!("airlock_gasket_pressure_decay_row_channel_witness_{row}"),
                PRESSURE_MANIFOLD_X - 108.0,
                4.0,
                5.0,
            )
            .translate(0.0, y, PRESSURE_MANIFOLD_Z + 2.5);
    }
    for col in 0..PRESSURE_PORT_COLS {
        let x = centered_index(col, PRESSURE_PORT_COLS, PRESSURE_PORT_PITCH_X);
        ribs = ribs
            + centered_cube(
                format!("airlock_gasket_pressure_decay_column_drop_witness_{col}"),
                4.0,
                126.0,
                5.0,
            )
            .translate(x, -18.0, PRESSURE_MANIFOLD_Z + 2.5);
    }
    ribs
}

fn pressure_reference_plugs() -> Part {
    let zero = centered_cylinder(
        "airlock_gasket_pressure_decay_zero_volume_reference_plug",
        16.0,
        10.0,
        36,
    )
    .translate(
        -PRESSURE_MANIFOLD_X / 2.0 + 46.0,
        -PRESSURE_MANIFOLD_Y / 2.0 + 42.0,
        PRESSURE_MANIFOLD_Z + 5.0,
    );
    let leak_orifice = centered_cylinder(
        "airlock_gasket_pressure_decay_known_leak_orifice_plug",
        16.0,
        10.0,
        36,
    )
    .translate(
        PRESSURE_MANIFOLD_X / 2.0 - 46.0,
        -PRESSURE_MANIFOLD_Y / 2.0 + 42.0,
        PRESSURE_MANIFOLD_Z + 5.0,
    );
    let zero_center = centered_cylinder(
        "airlock_gasket_pressure_decay_zero_reference_center_mark",
        3.0,
        11.0,
        20,
    )
    .translate(
        -PRESSURE_MANIFOLD_X / 2.0 + 46.0,
        -PRESSURE_MANIFOLD_Y / 2.0 + 42.0,
        PRESSURE_MANIFOLD_Z + 5.5,
    );
    let leak_center = centered_cylinder(
        "airlock_gasket_pressure_decay_known_leak_center_mark",
        1.4,
        11.0,
        18,
    )
    .translate(
        PRESSURE_MANIFOLD_X / 2.0 - 46.0,
        -PRESSURE_MANIFOLD_Y / 2.0 + 42.0,
        PRESSURE_MANIFOLD_Z + 5.5,
    );

    (zero - zero_center) + (leak_orifice - leak_center)
}

fn particle_collection_coupon_lanes() -> Part {
    let panel = module_panel(
        "airlock_gasket_particle_collection_panel",
        PARTICLE_LANE_X,
        PARTICLE_LANE_Y,
        PARTICLE_LANE_Z,
    );
    let sump = centered_cube(
        "airlock_gasket_particle_collection_lane_sump",
        PARTICLE_LANE_X - 42.0,
        18.0,
        10.0,
    )
    .translate(0.0, -PARTICLE_LANE_Y / 2.0 + 24.0, PARTICLE_LANE_Z - 5.0);

    panel - sump - particle_lane_recesses()
        + particle_coupon_liners()
        + particle_lane_dividers()
        + particle_grid_ticks()
        + particle_lot_retainer_rails()
        + module_corner_fiducials(
            "airlock_gasket_particle_collection",
            PARTICLE_LANE_X,
            PARTICLE_LANE_Y,
            PARTICLE_LANE_Z,
        )
}

fn particle_lane_recesses() -> Part {
    let mut recesses = Part::empty("airlock_gasket_particle_collection_lane_recesses");
    for lane in 0..PARTICLE_LANE_COUNT {
        let y = centered_index(lane, PARTICLE_LANE_COUNT, PARTICLE_LANE_PITCH_Y) + 10.0;
        recesses = recesses
            + centered_cube(
                format!("airlock_gasket_particle_collection_lane_recess_{lane}"),
                PARTICLE_LANE_X - 72.0,
                PARTICLE_COUPON_Y + 8.0,
                13.0,
            )
            .translate(0.0, y, PARTICLE_LANE_Z - 6.5);
    }
    recesses
}

fn particle_coupon_liners() -> Part {
    let mut liners = Part::empty("airlock_gasket_particle_collection_coupon_liners");
    for lane in 0..PARTICLE_LANE_COUNT {
        let y = centered_index(lane, PARTICLE_LANE_COUNT, PARTICLE_LANE_PITCH_Y) + 10.0;
        for coupon in 0..PARTICLE_COUPONS_PER_LANE {
            let index = lane * PARTICLE_COUPONS_PER_LANE + coupon;
            let x = centered_index(coupon, PARTICLE_COUPONS_PER_LANE, 172.0);
            liners = liners
                + centered_cube(
                    format!("airlock_gasket_particle_collection_removable_coupon_{index}"),
                    PARTICLE_COUPON_X,
                    PARTICLE_COUPON_Y,
                    3.2,
                )
                .translate(x, y, PARTICLE_LANE_Z + 1.6)
                + centered_cube(
                    format!("airlock_gasket_particle_collection_coupon_pull_tab_{index}"),
                    14.0,
                    8.0,
                    5.0,
                )
                .translate(
                    x + PARTICLE_COUPON_X / 2.0 + 10.0,
                    y,
                    PARTICLE_LANE_Z + 2.5,
                );
        }
    }
    liners
}

fn particle_lane_dividers() -> Part {
    let mut dividers = Part::empty("airlock_gasket_particle_collection_lane_dividers");
    for lane in 0..=PARTICLE_LANE_COUNT {
        let y = -((PARTICLE_LANE_COUNT as f64) * PARTICLE_LANE_PITCH_Y) / 2.0
            + lane as f64 * PARTICLE_LANE_PITCH_Y
            + 10.0
            - PARTICLE_LANE_PITCH_Y / 2.0;
        dividers = dividers
            + centered_cube(
                format!("airlock_gasket_particle_collection_lane_divider_{lane}"),
                PARTICLE_LANE_X - 54.0,
                3.5,
                10.0,
            )
            .translate(0.0, y, PARTICLE_LANE_Z + 5.0);
    }
    dividers
}

fn particle_grid_ticks() -> Part {
    let mut ticks = Part::empty("airlock_gasket_particle_collection_grid_ticks");
    for lane in 0..PARTICLE_LANE_COUNT {
        let y = centered_index(lane, PARTICLE_LANE_COUNT, PARTICLE_LANE_PITCH_Y) + 10.0;
        for tick in 0..PARTICLE_GRID_TICK_COUNT {
            ticks = ticks
                + centered_cube(
                    format!("airlock_gasket_particle_collection_lane_{lane}_grid_tick_{tick}"),
                    1.8,
                    PARTICLE_COUPON_Y + 12.0,
                    3.5,
                )
                .translate(
                    centered_index(tick, PARTICLE_GRID_TICK_COUNT, 44.0),
                    y,
                    PARTICLE_LANE_Z + 1.75,
                );
        }
    }
    ticks
}

fn particle_lot_retainer_rails() -> Part {
    let upper = centered_cube(
        "airlock_gasket_particle_collection_upper_retainer_rail",
        PARTICLE_LANE_X - 42.0,
        7.0,
        15.0,
    )
    .translate(0.0, PARTICLE_LANE_Y / 2.0 - 22.0, PARTICLE_LANE_Z + 7.5);
    let lower = centered_cube(
        "airlock_gasket_particle_collection_lower_retainer_rail",
        PARTICLE_LANE_X - 42.0,
        7.0,
        15.0,
    )
    .translate(0.0, -PARTICLE_LANE_Y / 2.0 + 22.0, PARTICLE_LANE_Z + 7.5);
    upper + lower
}

fn wipe_contact_sampling_pockets() -> Part {
    let panel = module_panel(
        "airlock_gasket_wipe_contact_sampling_panel",
        SAMPLING_PANEL_X,
        SAMPLING_PANEL_Y,
        SAMPLING_PANEL_Z,
    );
    let sample_drain_groove = centered_cube(
        "airlock_gasket_sampling_panel_closed_sample_drain_groove",
        SAMPLING_PANEL_X - 64.0,
        10.0,
        8.0,
    )
    .translate(0.0, -SAMPLING_PANEL_Y / 2.0 + 24.0, SAMPLING_PANEL_Z - 4.0);

    panel - sample_drain_groove - wipe_pocket_cutouts() - contact_pocket_cutouts()
        + wipe_pocket_rims()
        + contact_pocket_rims()
        + swab_handle_parking_slots()
        + sampling_sequence_ladder()
        + module_corner_fiducials(
            "airlock_gasket_sampling",
            SAMPLING_PANEL_X,
            SAMPLING_PANEL_Y,
            SAMPLING_PANEL_Z,
        )
}

fn wipe_pocket_cutouts() -> Part {
    let mut cutouts = Part::empty("airlock_gasket_wipe_sampling_pocket_cutouts");
    for i in 0..WIPE_POCKET_COUNT {
        cutouts = cutouts
            + centered_cube(
                format!("airlock_gasket_wipe_sampling_pocket_cutout_{i}"),
                WIPE_POCKET_X,
                WIPE_POCKET_Y,
                11.0,
            )
            .translate(
                centered_index(i, WIPE_POCKET_COUNT, 86.0),
                44.0,
                SAMPLING_PANEL_Z - 5.5,
            );
    }
    cutouts
}

fn contact_pocket_cutouts() -> Part {
    let mut cutouts = Part::empty("airlock_gasket_contact_sampling_pocket_cutouts");
    for i in 0..CONTACT_POCKET_COUNT {
        cutouts = cutouts
            + centered_cylinder(
                format!("airlock_gasket_contact_sampling_pocket_cutout_{i}"),
                CONTACT_POCKET_D / 2.0,
                11.0,
                36,
            )
            .translate(
                centered_index(i, CONTACT_POCKET_COUNT, 86.0),
                -38.0,
                SAMPLING_PANEL_Z - 5.5,
            );
    }
    cutouts
}

fn wipe_pocket_rims() -> Part {
    let mut rims = Part::empty("airlock_gasket_wipe_sampling_pocket_rims");
    for i in 0..WIPE_POCKET_COUNT {
        rims = rims
            + rectangular_frame(
                &format!("airlock_gasket_wipe_sampling_pocket_rim_{i}"),
                WIPE_POCKET_X + 11.0,
                WIPE_POCKET_Y + 11.0,
                6.0,
                5.5,
            )
            .translate(
                centered_index(i, WIPE_POCKET_COUNT, 86.0),
                44.0,
                SAMPLING_PANEL_Z + 3.0,
            );
    }
    rims
}

fn contact_pocket_rims() -> Part {
    let mut rims = Part::empty("airlock_gasket_contact_sampling_pocket_rims");
    for i in 0..CONTACT_POCKET_COUNT {
        let x = centered_index(i, CONTACT_POCKET_COUNT, 86.0);
        let outer = centered_cylinder(
            format!("airlock_gasket_contact_sampling_pocket_outer_rim_{i}"),
            CONTACT_POCKET_D / 2.0 + 6.0,
            6.0,
            40,
        )
        .translate(x, -38.0, SAMPLING_PANEL_Z + 3.0);
        let inner = centered_cylinder(
            format!("airlock_gasket_contact_sampling_pocket_inner_opening_{i}"),
            CONTACT_POCKET_D / 2.0,
            7.0,
            40,
        )
        .translate(x, -38.0, SAMPLING_PANEL_Z + 3.5);
        rims = rims + (outer - inner);
    }
    rims
}

fn swab_handle_parking_slots() -> Part {
    let mut slots = Part::empty("airlock_gasket_sampling_swab_handle_parking_slots");
    for i in 0..SAMPLING_POCKET_COUNT {
        slots = slots
            + centered_cube(
                format!("airlock_gasket_sampling_swab_handle_park_{i}"),
                36.0,
                5.0,
                5.0,
            )
            .translate(
                centered_index(i, SAMPLING_POCKET_COUNT, 45.0),
                -78.0,
                SAMPLING_PANEL_Z + 2.5,
            );
    }
    slots
}

fn sampling_sequence_ladder() -> Part {
    let mut ladder = Part::empty("airlock_gasket_sampling_sequence_ladder");
    for i in 0..=SAMPLING_POCKET_COUNT {
        ladder = ladder
            + centered_cube(
                format!("airlock_gasket_sampling_sequence_ladder_tick_{i}"),
                2.0,
                16.0,
                3.0,
            )
            .translate(
                centered_index(i, SAMPLING_POCKET_COUNT + 1, 38.0),
                0.0,
                SAMPLING_PANEL_Z + 1.5,
            );
    }
    ladder
}

fn cassette_transfer_tongue_datum() -> Part {
    let panel = module_panel(
        "airlock_gasket_transfer_tongue_datum_panel",
        TRANSFER_DATUM_X,
        TRANSFER_DATUM_Y,
        TRANSFER_DATUM_Z,
    );
    let tongue_recess = centered_cube(
        "airlock_gasket_transfer_tongue_reference_recess",
        TRANSFER_TONGUE_X,
        TRANSFER_TONGUE_Y,
        TRANSFER_TONGUE_DEPTH,
    )
    .translate(0.0, 16.0, TRANSFER_DATUM_Z - TRANSFER_TONGUE_DEPTH / 2.0);

    panel - tongue_recess - transfer_tongue_relief_slots()
        + transfer_tongue_reference_edges()
        + kinematic_datum_bosses()
        + tissue_chip_transfer_scale()
        + airlock_door_tongue_shadow()
        + module_corner_fiducials(
            "airlock_gasket_transfer_tongue",
            TRANSFER_DATUM_X,
            TRANSFER_DATUM_Y,
            TRANSFER_DATUM_Z,
        )
}

fn transfer_tongue_relief_slots() -> Part {
    let mut slots = Part::empty("airlock_gasket_transfer_tongue_relief_slots");
    for i in 0..4 {
        slots = slots
            + centered_cube(
                format!("airlock_gasket_transfer_tongue_side_relief_slot_{i}"),
                18.0,
                10.0,
                TRANSFER_TONGUE_DEPTH + 2.0,
            )
            .translate(
                centered_index(i, 4, 62.0),
                -TRANSFER_TONGUE_Y / 2.0 + 16.0,
                TRANSFER_DATUM_Z - TRANSFER_TONGUE_DEPTH / 2.0,
            );
    }
    slots
}

fn transfer_tongue_reference_edges() -> Part {
    let left = centered_cube(
        "airlock_gasket_transfer_tongue_left_datum_edge",
        8.0,
        TRANSFER_TONGUE_Y + 28.0,
        18.0,
    )
    .translate(
        -TRANSFER_TONGUE_X / 2.0 - 10.0,
        16.0,
        TRANSFER_DATUM_Z + 9.0,
    );
    let right = centered_cube(
        "airlock_gasket_transfer_tongue_right_datum_edge",
        8.0,
        TRANSFER_TONGUE_Y + 28.0,
        18.0,
    )
    .translate(TRANSFER_TONGUE_X / 2.0 + 10.0, 16.0, TRANSFER_DATUM_Z + 9.0);
    let nose = centered_cube(
        "airlock_gasket_transfer_tongue_nose_stop_edge",
        TRANSFER_TONGUE_X + 36.0,
        8.0,
        18.0,
    )
    .translate(0.0, TRANSFER_TONGUE_Y / 2.0 + 30.0, TRANSFER_DATUM_Z + 9.0);
    let entry_funnel = centered_cube(
        "airlock_gasket_transfer_tongue_entry_funnel_witness",
        TRANSFER_TONGUE_X + 72.0,
        8.0,
        12.0,
    )
    .translate(0.0, -TRANSFER_TONGUE_Y / 2.0 - 32.0, TRANSFER_DATUM_Z + 6.0);

    left + right + nose + entry_funnel
}

fn kinematic_datum_bosses() -> Part {
    let mut bosses = Part::empty("airlock_gasket_transfer_tongue_kinematic_datum_bosses");
    for (i, (x, y)) in [(-148.0, -54.0), (148.0, -54.0), (0.0, 78.0)]
        .into_iter()
        .enumerate()
    {
        let boss = centered_cylinder(
            format!("airlock_gasket_transfer_tongue_kinematic_boss_{i}"),
            14.0,
            9.0,
            36,
        )
        .translate(x, y, TRANSFER_DATUM_Z + 4.5);
        let center = centered_cylinder(
            format!("airlock_gasket_transfer_tongue_kinematic_center_{i}"),
            3.2,
            10.0,
            22,
        )
        .translate(x, y, TRANSFER_DATUM_Z + 5.0);
        bosses = bosses + (boss - center);
    }
    bosses
}

fn tissue_chip_transfer_scale() -> Part {
    let mut scale = Part::empty("airlock_gasket_transfer_tongue_tissue_chip_scale");
    for cassette in 0..TISSUE_CHIP_CASSETTES_PER_TRANSFER {
        let y = centered_index(cassette, TISSUE_CHIP_CASSETTES_PER_TRANSFER, 28.0) + 16.0;
        let lane = centered_cube(
            format!("airlock_gasket_transfer_tongue_cassette_lane_{cassette}"),
            TRANSFER_TONGUE_X - 40.0,
            5.0,
            4.0,
        )
        .translate(0.0, y, TRANSFER_DATUM_Z + 2.0);
        scale = scale + lane;
        for chip in 0..TISSUE_CHIPS_PER_CASSETTE {
            scale = scale
                + centered_cube(
                    format!(
                        "airlock_gasket_transfer_tongue_tissue_chip_position_{cassette}_{chip}"
                    ),
                    14.0,
                    8.0,
                    4.0,
                )
                .translate(
                    centered_index(chip, TISSUE_CHIPS_PER_CASSETTE, 30.0),
                    y,
                    TRANSFER_DATUM_Z + 2.0,
                );
        }
    }
    scale
}

fn airlock_door_tongue_shadow() -> Part {
    let sweep_arc = centered_cylinder(
        "airlock_gasket_transfer_tongue_airlock_door_sweep_shadow_outer",
        96.0,
        4.0,
        60,
    )
    .translate(-110.0, 14.0, TRANSFER_DATUM_Z + 2.0);
    let inner = centered_cylinder(
        "airlock_gasket_transfer_tongue_airlock_door_sweep_shadow_inner",
        84.0,
        5.0,
        60,
    )
    .translate(-110.0, 14.0, TRANSFER_DATUM_Z + 2.5);
    let latch_stop = centered_cube(
        "airlock_gasket_transfer_tongue_latch_stop_shadow_land",
        34.0,
        20.0,
        5.0,
    )
    .translate(128.0, 72.0, TRANSFER_DATUM_Z + 2.5);

    (sweep_arc - inner) + latch_stop
}

fn latch_force_witness_pockets() -> Part {
    let panel = module_panel(
        "airlock_gasket_latch_force_witness_panel",
        LATCH_PANEL_X,
        LATCH_PANEL_Y,
        LATCH_PANEL_Z,
    );
    let force_basin = centered_cube(
        "airlock_gasket_latch_force_witness_basin",
        LATCH_PANEL_X - 44.0,
        LATCH_PANEL_Y - 38.0,
        9.0,
    )
    .translate(0.0, 0.0, LATCH_PANEL_Z - 4.5);

    panel - force_basin - latch_force_pocket_cutouts()
        + latch_force_pocket_rims()
        + force_film_slots()
        + latch_nose_witness_blocks()
        + latch_force_ruler()
        + module_corner_fiducials(
            "airlock_gasket_latch_force",
            LATCH_PANEL_X,
            LATCH_PANEL_Y,
            LATCH_PANEL_Z,
        )
}

fn latch_force_pocket_cutouts() -> Part {
    let mut cutouts = Part::empty("airlock_gasket_latch_force_pocket_cutouts");
    for row in 0..LATCH_FORCE_ROWS {
        for col in 0..LATCH_FORCE_COLS {
            let index = row * LATCH_FORCE_COLS + col;
            cutouts = cutouts
                + centered_cylinder(
                    format!("airlock_gasket_latch_force_pocket_cutout_{index}"),
                    20.0,
                    13.0,
                    36,
                )
                .translate(
                    centered_index(col, LATCH_FORCE_COLS, LATCH_FORCE_PITCH_X),
                    centered_index(row, LATCH_FORCE_ROWS, LATCH_FORCE_PITCH_Y),
                    LATCH_PANEL_Z - 6.5,
                );
        }
    }
    cutouts
}

fn latch_force_pocket_rims() -> Part {
    let mut rims = Part::empty("airlock_gasket_latch_force_pocket_rims");
    for row in 0..LATCH_FORCE_ROWS {
        for col in 0..LATCH_FORCE_COLS {
            let index = row * LATCH_FORCE_COLS + col;
            let x = centered_index(col, LATCH_FORCE_COLS, LATCH_FORCE_PITCH_X);
            let y = centered_index(row, LATCH_FORCE_ROWS, LATCH_FORCE_PITCH_Y);
            let outer = centered_cylinder(
                format!("airlock_gasket_latch_force_pocket_outer_rim_{index}"),
                26.0,
                7.0,
                36,
            )
            .translate(x, y, LATCH_PANEL_Z + 3.5);
            let inner = centered_cylinder(
                format!("airlock_gasket_latch_force_pocket_inner_clear_{index}"),
                20.0,
                8.0,
                36,
            )
            .translate(x, y, LATCH_PANEL_Z + 4.0);
            rims = rims + (outer - inner);
        }
    }
    rims
}

fn force_film_slots() -> Part {
    let mut slots = Part::empty("airlock_gasket_latch_force_film_slots");
    for i in 0..FORCE_FILM_SLOT_COUNT {
        slots = slots
            + centered_cube(
                format!("airlock_gasket_latch_force_film_slot_{i}"),
                54.0,
                7.0,
                5.0,
            )
            .translate(
                centered_index(i, FORCE_FILM_SLOT_COUNT, 58.0),
                -78.0,
                LATCH_PANEL_Z + 2.5,
            );
    }
    slots
}

fn latch_nose_witness_blocks() -> Part {
    let mut blocks = Part::empty("airlock_gasket_latch_nose_witness_blocks");
    for i in 0..LATCH_FORCE_COLS {
        blocks = blocks
            + centered_cube(
                format!("airlock_gasket_latch_nose_polish_witness_block_{i}"),
                62.0,
                16.0,
                8.0,
            )
            .translate(
                centered_index(i, LATCH_FORCE_COLS, LATCH_FORCE_PITCH_X),
                76.0,
                LATCH_PANEL_Z + 4.0,
            );
    }
    blocks
}

fn latch_force_ruler() -> Part {
    let mut ruler = Part::empty("airlock_gasket_latch_force_ruler");
    for i in 0..=8 {
        let h = if i % 2 == 0 { 16.0 } else { 9.0 };
        ruler = ruler
            + centered_cube(
                format!("airlock_gasket_latch_force_ruler_tick_{i}"),
                2.0,
                h,
                3.5,
            )
            .translate(-174.0 + i as f64 * 43.5, -54.0, LATCH_PANEL_Z + 1.75);
    }
    ruler
}

fn barcode_cycle_count_lands() -> Part {
    let panel = module_panel(
        "airlock_gasket_barcode_cycle_count_panel",
        TRACE_PANEL_X,
        TRACE_PANEL_Y,
        TRACE_PANEL_Z,
    );
    let card_recess = centered_cube(
        "airlock_gasket_barcode_cycle_count_run_card_recess",
        TRACE_PANEL_X - 52.0,
        54.0,
        8.0,
    )
    .translate(0.0, -TRACE_PANEL_Y / 2.0 + 38.0, TRACE_PANEL_Z - 4.0);

    panel - card_recess
        + barcode_lands()
        + cycle_count_lands()
        + run_card_lands()
        + traceability_rail_marks()
        + module_corner_fiducials(
            "airlock_gasket_traceability",
            TRACE_PANEL_X,
            TRACE_PANEL_Y,
            TRACE_PANEL_Z,
        )
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("airlock_gasket_barcode_lands");
    for i in 0..BARCODE_LAND_COUNT {
        let row = i / 8;
        let col = i % 8;
        let x = centered_index(col, 8, 48.0);
        let y = 58.0 - row as f64 * 34.0;
        lands = lands
            + centered_cube(
                format!("airlock_gasket_barcode_land_plate_{i}"),
                40.0,
                20.0,
                3.5,
            )
            .translate(x, y, TRACE_PANEL_Z + 1.75)
            + barcode_stripes("airlock_gasket_barcode_land", i, x, y, TRACE_PANEL_Z + 4.0);
    }
    lands
}

fn cycle_count_lands() -> Part {
    let mut lands = Part::empty("airlock_gasket_cycle_count_lands");
    for i in 0..CYCLE_COUNT_LAND_COUNT {
        let row = i / 8;
        let col = i % 8;
        lands = lands
            + centered_cube(
                format!("airlock_gasket_cycle_count_land_{i}"),
                28.0,
                18.0,
                3.5,
            )
            .translate(
                centered_index(col, 8, 42.0),
                -8.0 - row as f64 * 30.0,
                TRACE_PANEL_Z + 1.75,
            );
    }
    lands
}

fn run_card_lands() -> Part {
    let mut lands = Part::empty("airlock_gasket_run_card_lands");
    for i in 0..RUN_CARD_LAND_COUNT {
        lands = lands
            + centered_cube(
                format!("airlock_gasket_run_card_certificate_land_{i}"),
                104.0,
                22.0,
                4.0,
            )
            .translate(
                centered_index(i, RUN_CARD_LAND_COUNT, 126.0),
                -TRACE_PANEL_Y / 2.0 + 38.0,
                TRACE_PANEL_Z + 2.0,
            );
    }
    lands
}

fn traceability_rail_marks() -> Part {
    let upper = centered_cube(
        "airlock_gasket_traceability_clean_record_rail",
        TRACE_PANEL_X - 42.0,
        5.0,
        5.0,
    )
    .translate(0.0, TRACE_PANEL_Y / 2.0 - 18.0, TRACE_PANEL_Z + 2.5);
    let lower = centered_cube(
        "airlock_gasket_traceability_used_record_rail",
        TRACE_PANEL_X - 42.0,
        5.0,
        5.0,
    )
    .translate(0.0, -TRACE_PANEL_Y / 2.0 + 16.0, TRACE_PANEL_Z + 2.5);
    upper + lower
}

fn clean_used_segregation() -> Part {
    let panel = module_panel(
        "airlock_gasket_clean_used_segregation_panel",
        SEGREGATION_X,
        SEGREGATION_Y,
        SEGREGATION_Z,
    );
    let clean_basin = centered_cube(
        "airlock_gasket_clean_bin_wipeable_basin",
        SEGREGATION_X / 2.0 - 44.0,
        SEGREGATION_Y - 42.0,
        12.0,
    )
    .translate(-SEGREGATION_X / 4.0, 0.0, SEGREGATION_Z - 6.0);
    let used_basin = centered_cube(
        "airlock_gasket_used_bin_wipeable_basin",
        SEGREGATION_X / 2.0 - 44.0,
        SEGREGATION_Y - 42.0,
        12.0,
    )
    .translate(SEGREGATION_X / 4.0, 0.0, SEGREGATION_Z - 6.0);

    panel - clean_basin - used_basin - segregation_slot_recesses()
        + segregation_divider()
        + clean_used_slot_lips()
        + one_way_used_chute()
        + module_corner_fiducials(
            "airlock_gasket_clean_used",
            SEGREGATION_X,
            SEGREGATION_Y,
            SEGREGATION_Z,
        )
}

fn segregation_slot_recesses() -> Part {
    let mut recesses = Part::empty("airlock_gasket_segregation_slot_recesses");
    for i in 0..CLEAN_SLOT_COUNT {
        let y = centered_index(i, CLEAN_SLOT_COUNT, 24.0);
        recesses = recesses
            + centered_cube(
                format!("airlock_gasket_clean_slot_recess_{i}"),
                74.0,
                16.0,
                15.0,
            )
            .translate(-94.0, y, SEGREGATION_Z - 7.5);
    }
    for i in 0..USED_SLOT_COUNT {
        let y = centered_index(i, USED_SLOT_COUNT, 24.0);
        recesses = recesses
            + centered_cube(
                format!("airlock_gasket_used_slot_recess_{i}"),
                74.0,
                16.0,
                15.0,
            )
            .translate(94.0, y, SEGREGATION_Z - 7.5);
    }
    recesses
}

fn segregation_divider() -> Part {
    let divider = centered_cube(
        "airlock_gasket_clean_used_high_segregation_divider",
        12.0,
        SEGREGATION_Y - 18.0,
        SEGREGATION_Z + 32.0,
    )
    .translate(0.0, 0.0, (SEGREGATION_Z + 32.0) / 2.0);
    let front_bridge = centered_cube(
        "airlock_gasket_clean_used_front_handling_stop",
        SEGREGATION_X - 38.0,
        8.0,
        28.0,
    )
    .translate(0.0, -SEGREGATION_Y / 2.0 + 18.0, SEGREGATION_Z + 14.0);
    divider + front_bridge
}

fn clean_used_slot_lips() -> Part {
    let mut lips = Part::empty("airlock_gasket_clean_used_slot_lips");
    for side in 0..2 {
        let x = if side == 0 { -94.0 } else { 94.0 };
        for i in 0..CLEAN_SLOT_COUNT {
            lips = lips
                + centered_cube(
                    format!("airlock_gasket_clean_used_slot_lip_{side}_{i}"),
                    82.0,
                    4.0,
                    8.0,
                )
                .translate(
                    x,
                    centered_index(i, CLEAN_SLOT_COUNT, 24.0) - 11.0,
                    SEGREGATION_Z + 4.0,
                );
        }
    }
    lips
}

fn one_way_used_chute() -> Part {
    let chute = centered_cube(
        "airlock_gasket_used_one_way_deposition_chute",
        98.0,
        20.0,
        22.0,
    )
    .rotate(0.0, 0.0, -8.0)
    .translate(96.0, -SEGREGATION_Y / 2.0 + 36.0, SEGREGATION_Z + 11.0);
    let clean_gate = centered_cube(
        "airlock_gasket_clean_side_loaded_gate_land",
        98.0,
        10.0,
        14.0,
    )
    .translate(-96.0, SEGREGATION_Y / 2.0 - 36.0, SEGREGATION_Z + 7.0);
    chute + clean_gate
}

fn release_hold_reject_disposition_lanes() -> Part {
    let panel = module_panel(
        "airlock_gasket_disposition_lane_panel",
        DISPOSITION_X,
        DISPOSITION_Y,
        DISPOSITION_Z,
    );
    let evidence_recess = centered_cube(
        "airlock_gasket_disposition_evidence_card_recess",
        DISPOSITION_X - 54.0,
        DISPOSITION_Y - 28.0,
        9.0,
    )
    .translate(0.0, 0.0, DISPOSITION_Z - 4.5);

    panel - evidence_recess - disposition_slot_cutouts()
        + disposition_lane_walls()
        + disposition_slot_tokens()
        + disposition_lane_stops()
        + module_corner_fiducials(
            "airlock_gasket_disposition",
            DISPOSITION_X,
            DISPOSITION_Y,
            DISPOSITION_Z,
        )
}

fn disposition_slot_cutouts() -> Part {
    let mut cutouts = Part::empty("airlock_gasket_disposition_slot_cutouts");
    for lane in DispositionLane::all() {
        let x = centered_index(lane.index(), DISPOSITION_LANES, 190.0);
        for slot in 0..DISPOSITION_SLOTS_PER_LANE {
            cutouts = cutouts
                + centered_cube(
                    format!(
                        "airlock_gasket_{}_disposition_slot_cutout_{slot}",
                        lane.name()
                    ),
                    72.0,
                    12.0,
                    10.0,
                )
                .translate(x - 76.0 + slot as f64 * 38.0, 0.0, DISPOSITION_Z - 5.0);
        }
    }
    cutouts
}

fn disposition_lane_walls() -> Part {
    let mut walls = Part::empty("airlock_gasket_disposition_lane_walls");
    for wall in 0..=DISPOSITION_LANES {
        let x = -DISPOSITION_X / 2.0 + 24.0 + wall as f64 * 190.0;
        walls = walls
            + centered_cube(
                format!("airlock_gasket_disposition_lane_separator_{wall}"),
                8.0,
                DISPOSITION_Y - 18.0,
                20.0,
            )
            .translate(x, 0.0, DISPOSITION_Z + 10.0);
    }
    walls
}

fn disposition_slot_tokens() -> Part {
    let mut tokens = Part::empty("airlock_gasket_disposition_slot_tokens");
    for lane in DispositionLane::all() {
        let x = centered_index(lane.index(), DISPOSITION_LANES, 190.0);
        for slot in 0..DISPOSITION_SLOTS_PER_LANE {
            tokens = tokens
                + centered_cube(
                    format!("airlock_gasket_{}_evidence_token_{slot}", lane.name()),
                    26.0,
                    18.0,
                    5.0,
                )
                .translate(
                    x - 76.0 + slot as f64 * 38.0,
                    26.0,
                    DISPOSITION_Z + 2.5,
                );
        }
    }
    tokens
}

fn disposition_lane_stops() -> Part {
    let release_stop = centered_cube("airlock_gasket_release_lane_open_stop", 160.0, 8.0, 17.0)
        .translate(
            centered_index(0, DISPOSITION_LANES, 190.0),
            DISPOSITION_Y / 2.0 - 14.0,
            DISPOSITION_Z + 8.5,
        );
    let hold_stop = centered_cube("airlock_gasket_hold_lane_review_stop", 160.0, 8.0, 28.0)
        .translate(
            centered_index(1, DISPOSITION_LANES, 190.0),
            DISPOSITION_Y / 2.0 - 14.0,
            DISPOSITION_Z + 14.0,
        );
    let reject_stop = centered_cube(
        "airlock_gasket_reject_lane_quarantine_stop",
        160.0,
        8.0,
        38.0,
    )
    .translate(
        centered_index(2, DISPOSITION_LANES, 190.0),
        DISPOSITION_Y / 2.0 - 14.0,
        DISPOSITION_Z + 19.0,
    );
    release_stop + hold_stop + reject_stop
}

fn camera_illumination_fiducials() -> Part {
    let rear_beam = centered_cube(
        "airlock_gasket_camera_bridge_rear_beam",
        CAMERA_BRIDGE_X,
        14.0,
        CAMERA_BRIDGE_Z,
    )
    .translate(0.0, CAMERA_BRIDGE_Y / 2.0 - 10.0, CAMERA_CLEARANCE_Z);
    let front_beam = centered_cube(
        "airlock_gasket_camera_bridge_front_light_beam",
        CAMERA_BRIDGE_X,
        14.0,
        CAMERA_BRIDGE_Z,
    )
    .translate(0.0, -CAMERA_BRIDGE_Y / 2.0 + 10.0, CAMERA_CLEARANCE_Z);

    rear_beam
        + front_beam
        + camera_bridge_posts()
        + camera_mount_bosses()
        + illumination_bars()
        + field_fiducial_targets()
}

fn camera_bridge_posts() -> Part {
    let mut posts = Part::empty("airlock_gasket_camera_bridge_posts");
    for (i, (x, y)) in [
        (-CAMERA_BRIDGE_X / 2.0 + 42.0, -CAMERA_BRIDGE_Y / 2.0 + 10.0),
        (CAMERA_BRIDGE_X / 2.0 - 42.0, -CAMERA_BRIDGE_Y / 2.0 + 10.0),
        (-CAMERA_BRIDGE_X / 2.0 + 42.0, CAMERA_BRIDGE_Y / 2.0 - 10.0),
        (CAMERA_BRIDGE_X / 2.0 - 42.0, CAMERA_BRIDGE_Y / 2.0 - 10.0),
    ]
    .into_iter()
    .enumerate()
    {
        let post = centered_cube(
            format!("airlock_gasket_camera_bridge_post_{i}"),
            24.0,
            24.0,
            CAMERA_CLEARANCE_Z,
        )
        .translate(x, y, CAMERA_CLEARANCE_Z / 2.0);
        let foot = centered_cube(
            format!("airlock_gasket_camera_bridge_foot_{i}"),
            58.0,
            34.0,
            8.0,
        )
        .translate(x, y, 4.0);
        posts = posts + post + foot;
    }
    posts
}

fn camera_mount_bosses() -> Part {
    let mut mounts = Part::empty("airlock_gasket_camera_mount_bosses");
    for i in 0..CAMERA_COUNT {
        let x = centered_index(i, CAMERA_COUNT, 260.0);
        let boss = centered_cylinder(
            format!("airlock_gasket_camera_mount_boss_{i}"),
            22.0,
            10.0,
            40,
        )
        .translate(x, 0.0, CAMERA_CLEARANCE_Z - CAMERA_BRIDGE_Z / 2.0 - 5.0);
        let lens = centered_cylinder(
            format!("airlock_gasket_camera_lens_clearance_{i}"),
            10.0,
            12.0,
            36,
        )
        .translate(x, 0.0, CAMERA_CLEARANCE_Z - CAMERA_BRIDGE_Z / 2.0 - 5.0);
        mounts = mounts + (boss - lens);
    }
    mounts
}

fn illumination_bars() -> Part {
    let mut bars = Part::empty("airlock_gasket_illumination_bars");
    for (i, y) in [-20.0, 20.0].into_iter().enumerate() {
        bars = bars
            + centered_cube(
                format!("airlock_gasket_illumination_bar_{i}"),
                CAMERA_BRIDGE_X - 180.0,
                8.0,
                8.0,
            )
            .translate(0.0, y, CAMERA_CLEARANCE_Z - CAMERA_BRIDGE_Z / 2.0 - 17.0);
    }
    bars
}

fn field_fiducial_targets() -> Part {
    let mut targets = Part::empty("airlock_gasket_camera_field_fiducial_targets");
    for (i, (x, y)) in [
        (-560.0, -18.0),
        (-420.0, 18.0),
        (-280.0, -18.0),
        (-140.0, 18.0),
        (0.0, -18.0),
        (140.0, 18.0),
        (280.0, -18.0),
        (420.0, 18.0),
        (560.0, -18.0),
        (0.0, 0.0),
    ]
    .into_iter()
    .enumerate()
    {
        targets = targets
            + fiducial_disc(&format!("airlock_gasket_camera_field_fiducial_{i}")).translate(
                x,
                y,
                CAMERA_CLEARANCE_Z - CAMERA_BRIDGE_Z / 2.0 - 22.0,
            );
    }
    targets
}

fn module_panel(name: &str, x: f64, y: f64, z: f64) -> Part {
    centered_cube(name, x, y, z).translate(0.0, 0.0, z / 2.0)
}

fn rectangular_frame(name: &str, x: f64, y: f64, z: f64, wall: f64) -> Part {
    let outer = centered_cube(format!("{name}_outer"), x, y, z);
    let inner = centered_cube(
        format!("{name}_inner_clearance"),
        x - 2.0 * wall,
        y - 2.0 * wall,
        z + 0.4,
    );
    (outer - inner).translate(0.0, 0.0, z / 2.0)
}

fn module_corner_fiducials(prefix: &str, x: f64, y: f64, z: f64) -> Part {
    fiducial_disc(&format!("{prefix}_fiducial_front_left")).translate(
        -x / 2.0 + 24.0,
        -y / 2.0 + 22.0,
        z + 1.5,
    ) + fiducial_disc(&format!("{prefix}_fiducial_front_right")).translate(
        x / 2.0 - 24.0,
        -y / 2.0 + 22.0,
        z + 1.5,
    )
}

fn fiducial_disc(name: &str) -> Part {
    let outer = centered_cylinder(format!("{name}_outer"), 8.0, 3.0, 32);
    let cross_x = centered_cube(format!("{name}_cross_x"), 13.0, 1.6, 3.4);
    let cross_y = centered_cube(format!("{name}_cross_y"), 1.6, 13.0, 3.4);
    (outer - cross_x - cross_y).translate(0.0, 0.0, 1.5)
}

fn barcode_stripes(prefix: &str, index: usize, x: f64, y: f64, z: f64) -> Part {
    let mut stripes = Part::empty(format!("{prefix}_{index}_stripes"));
    for (bar, dx) in [-15.0, -10.0, -5.0, 1.0, 8.0, 14.0].into_iter().enumerate() {
        let width = if bar % 2 == 0 { 2.4 } else { 1.2 };
        stripes =
            stripes
                + centered_cube(format!("{prefix}_{index}_stripe_{bar}"), width, 16.0, 1.2)
                    .translate(x + dx, y, z);
    }
    stripes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_manifest_is_complete_and_scoped() {
        assert_eq!(OUTPUTS.len(), 13);
        assert!(OUTPUTS.iter().all(|path| path.starts_with("output/")));
        assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
        assert_eq!(
            OUTPUTS[OUTPUTS.len() - 1],
            format!("output/{OUTPUT_PREFIX}_assembly.stl")
        );
    }

    #[test]
    fn requested_feature_manifest_covers_validation_station() {
        for feature in [
            "containment_deck",
            "door_gasket_coupon_cassette",
            "compression_cycle_witness_rail",
            "pressure_decay_leak_ports",
            "particle_collection_coupon_lanes",
            "wipe_contact_sampling_pockets",
            "cassette_transfer_tongue_datum",
            "latch_force_witness_pockets",
            "barcode_cycle_count_lands",
            "camera_illumination_fiducials",
            "clean_used_segregation",
            "release_hold_reject_disposition",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn coupon_cycle_and_transfer_counts_are_deterministic() {
        assert_eq!(GASKET_COUPON_COUNT, 12);
        assert_eq!(PRESSURE_DECAY_PORT_COUNT, GASKET_COUPON_COUNT);
        assert_eq!(PARTICLE_COLLECTION_COUPON_COUNT, 12);
        assert_eq!(CYCLE_WITNESS_COUNT, 16);
        assert_eq!(cycle_witness_transfer_capacity(), 800);
        assert_eq!(TISSUE_CHIP_SURROGATE_COUNT, 24);
        assert_eq!(LATCH_FORCE_POCKET_COUNT, 6);
        assert_eq!(BARCODE_LAND_COUNT, 16);
        assert_eq!(KINEMATIC_DATUM_COUNT, 3);
    }

    #[test]
    fn module_footprints_fit_and_do_not_collide() {
        let modules = module_rects();
        for module in modules {
            assert!(
                module.fits_inside_station(),
                "{} exceeds station usable deck",
                module.name
            );
        }

        for (left_index, left) in modules.iter().enumerate() {
            for right in modules.iter().skip(left_index + 1) {
                if left.name == "camera_illumination_fiducials"
                    || right.name == "camera_illumination_fiducials"
                {
                    continue;
                }
                assert!(
                    !left.overlaps(*right),
                    "{} overlaps {}",
                    left.name,
                    right.name
                );
            }
        }
    }

    #[test]
    fn validation_geometry_has_minimum_capacity_and_clearance() {
        assert!(particle_collection_area_mm2() >= 9000.0);
        assert!(clean_used_gap_mm() >= SEGREGATION_MIN_GAP);
        assert!(compression_step_range_mm() >= 2.0);
        assert!(front_transfer_clearance_mm() >= FRONT_TRANSFER_CLEARANCE);
        assert!(rear_camera_service_clearance_mm() >= REAR_CAMERA_SERVICE_CLEARANCE);
        assert!(side_service_clearance_mm() >= SIDE_SERVICE_CLEARANCE);
        assert_eq!(DECK_DATUM_TARGET_COUNT, 6);
        assert_eq!(CAMERA_POST_COUNT, 4);
        assert_eq!(CAMERA_FIDUCIAL_COUNT, 10);
    }
}
