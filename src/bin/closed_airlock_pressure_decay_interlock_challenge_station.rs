use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed cabinet airlock validation fixture for pressure-decay and interlock
// challenge testing.
//
// Intent:
// - Exercise inner/outer door surrogate plates as a closed cabinet airlock
//   validation article, without treating printed parts as pressure-rated
//   containment.
// - Reserve repeatable seal compression coupons, differential pressure ports,
//   HEPA purge duct interface geometry, lockout flag blocks, traceability lands,
//   release/hold/reject handling lanes, evidence capture, and robot/service
//   keepouts.
// - Keep purchased seals, pressure transducers, HEPA hardware, lockout switches,
//   barcode readers, and acceptance procedures external to this CAD generator.

const OUTPUTS: [&str; 11] = [
    "output/closed_airlock_pressure_decay_interlock_challenge_station_base_validation_deck.stl",
    "output/closed_airlock_pressure_decay_interlock_challenge_station_door_surrogate_plates.stl",
    "output/closed_airlock_pressure_decay_interlock_challenge_station_seal_compression_coupons.stl",
    "output/closed_airlock_pressure_decay_interlock_challenge_station_differential_pressure_ports.stl",
    "output/closed_airlock_pressure_decay_interlock_challenge_station_hepa_purge_duct_interface.stl",
    "output/closed_airlock_pressure_decay_interlock_challenge_station_lockout_flag_blocks.stl",
    "output/closed_airlock_pressure_decay_interlock_challenge_station_barcode_certificate_lands.stl",
    "output/closed_airlock_pressure_decay_interlock_challenge_station_release_hold_reject_lanes.stl",
    "output/closed_airlock_pressure_decay_interlock_challenge_station_evidence_bridge.stl",
    "output/closed_airlock_pressure_decay_interlock_challenge_station_robot_service_keepouts.stl",
    "output/closed_airlock_pressure_decay_interlock_challenge_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 9] = [
    "inner_outer_door_surrogate_plates",
    "seal_compression_coupons",
    "differential_pressure_ports",
    "hepa_purge_duct_interface",
    "lockout_flag_blocks",
    "barcode_certificate_lands",
    "release_hold_reject_lanes",
    "evidence_bridge",
    "robot_service_keepouts",
];

const DECK_X: f64 = 1480.0;
const DECK_Y: f64 = 940.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 38.0;
const SOCKET_DEPTH: f64 = 6.0;
const MOUNT_HOLE_D: f64 = 6.8;

const DOOR_ZONE_POS: (f64, f64) = (-330.0, 140.0);
const DOOR_ZONE_X: f64 = 690.0;
const DOOR_ZONE_Y: f64 = 330.0;
const DOOR_PLATE_X: f64 = 292.0;
const DOOR_PLATE_Y: f64 = 258.0;
const DOOR_PLATE_Z: f64 = 28.0;
const DOOR_PITCH_X: f64 = 342.0;
const DOOR_GASKET_W: f64 = 14.0;
const DOOR_GASKET_Z: f64 = 9.0;
const DOOR_HINGE_D: f64 = 18.0;
const DOOR_LATCH_BLOCKS_PER_PLATE: usize = 3;
const INTERLOCK_PIN_COUNT: usize = 4;

const COUPON_POS: (f64, f64) = (330.0, 170.0);
const COUPON_X: f64 = 360.0;
const COUPON_Y: f64 = 260.0;
const COUPON_Z: f64 = 44.0;
const COUPON_ROWS: usize = 3;
const COUPON_COLS: usize = 4;
const COUPON_COUNT: usize = COUPON_ROWS * COUPON_COLS;
const COUPON_PITCH_X: f64 = 76.0;
const COUPON_PITCH_Y: f64 = 68.0;
const SEAL_COUPON_OUTER_D: f64 = 48.0;
const SEAL_COUPON_INNER_D: f64 = 28.0;
const COMPRESSION_STEP_COUNT: usize = 5;
const COMPRESSION_MIN_MM: f64 = 2.0;
const COMPRESSION_MAX_MM: f64 = 6.0;

const PRESSURE_POS: (f64, f64) = (-330.0, -180.0);
const PRESSURE_X: f64 = 620.0;
const PRESSURE_Y: f64 = 170.0;
const PRESSURE_Z: f64 = 54.0;
const PRESSURE_PAIR_COUNT: usize = 4;
const PRESSURE_PORT_COUNT: usize = PRESSURE_PAIR_COUNT * 2;
const PRESSURE_PAIR_PITCH_X: f64 = 132.0;
const PRESSURE_PORT_PITCH_Y: f64 = 56.0;
const PRESSURE_PORT_D: f64 = 8.0;
const PRESSURE_SENSOR_POCKETS: usize = 4;
const DECAY_REFERENCE_VOLUME_COUNT: usize = 3;

const HEPA_POS: (f64, f64) = (325.0, -115.0);
const HEPA_X: f64 = 400.0;
const HEPA_Y: f64 = 230.0;
const HEPA_Z: f64 = 64.0;
const HEPA_DUCT_ID: f64 = 152.0;
const HEPA_DUCT_OD: f64 = 196.0;
const HEPA_FLANGE_BOLT_COUNT: usize = 8;
const HEPA_DIFFUSER_SLOT_COUNT: usize = 7;
const PURGE_DAMPER_FLAG_COUNT: usize = 3;

const LOCKOUT_POS: (f64, f64) = (-520.0, -355.0);
const LOCKOUT_X: f64 = 300.0;
const LOCKOUT_Y: f64 = 150.0;
const LOCKOUT_Z: f64 = 34.0;
const LOCKOUT_FLAG_COUNT: usize = 6;
const LOCKOUT_FLAG_PITCH_X: f64 = 43.0;
const LOCKOUT_FLAG_THROW_Z: f64 = 76.0;

const TRACE_POS: (f64, f64) = (-125.0, -360.0);
const TRACE_X: f64 = 390.0;
const TRACE_Y: f64 = 140.0;
const TRACE_Z: f64 = 14.0;
const BARCODE_LAND_COUNT: usize = 6;
const CERTIFICATE_LAND_COUNT: usize = 3;
const WITNESS_TOKEN_COUNT: usize = 6;

const LANES_POS: (f64, f64) = (365.0, -355.0);
const LANES_X: f64 = 390.0;
const LANES_Y: f64 = 155.0;
const LANES_Z: f64 = 18.0;
const LANE_COUNT: usize = 3;
const LANE_PITCH_X: f64 = 122.0;
const LANE_WIDTH_X: f64 = 94.0;
const LANE_RAIL_Y: f64 = 124.0;
const LANE_GATE_Z: f64 = 48.0;
const LANE_TOKEN_SLOTS: usize = 4;

const EVIDENCE_POS: (f64, f64) = (-60.0, 355.0);
const EVIDENCE_SPAN_X: f64 = 1160.0;
const EVIDENCE_POST_Y: f64 = 52.0;
const EVIDENCE_POST_X: f64 = 30.0;
const EVIDENCE_UNDERSIDE_Z: f64 = 228.0;
const EVIDENCE_BEAM_Z: f64 = 30.0;
const EVIDENCE_CAMERA_COUNT: usize = 4;
const EVIDENCE_LIGHT_SEGMENTS: usize = 10;

const FRONT_ROBOT_KEEP_OUT_Y: f64 = 260.0;
const REAR_SERVICE_KEEP_OUT_Y: f64 = 210.0;
const LEFT_DOOR_SERVICE_KEEP_OUT_X: f64 = 160.0;
const RIGHT_HEPA_SERVICE_KEEP_OUT_X: f64 = 260.0;
const TOP_PURGE_DUCT_KEEP_OUT_Z: f64 = 350.0;
const ROBOT_PICK_CLEARANCE_Z: f64 = 320.0;
const KEEP_OUT_RAIL: f64 = 10.0;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DispositionLane {
    Release,
    Hold,
    Reject,
}

impl DispositionLane {
    fn all() -> [DispositionLane; LANE_COUNT] {
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

    fn label(self) -> &'static str {
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
    fn fits_inside_deck(self) -> bool {
        let usable_x = DECK_X / 2.0 - RIM_W - 12.0;
        let usable_y = DECK_Y / 2.0 - RIM_W - 12.0;
        self.center.0 - self.x / 2.0 >= -usable_x
            && self.center.0 + self.x / 2.0 <= usable_x
            && self.center.1 - self.y / 2.0 >= -usable_y
            && self.center.1 + self.y / 2.0 <= usable_y
    }

    fn overlaps(self, other: Rect) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();
        dx < (self.x + other.x) / 2.0 && dy < (self.y + other.y) / 2.0
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_validation_deck();
    export(OUTPUTS[0], &base);

    let doors = door_surrogate_plates();
    export(OUTPUTS[1], &doors);

    let coupons = seal_compression_coupons();
    export(OUTPUTS[2], &coupons);

    let pressure_ports = differential_pressure_ports();
    export(OUTPUTS[3], &pressure_ports);

    let hepa = hepa_purge_duct_interface();
    export(OUTPUTS[4], &hepa);

    let lockouts = lockout_flag_blocks();
    export(OUTPUTS[5], &lockouts);

    let traceability = barcode_certificate_lands();
    export(OUTPUTS[6], &traceability);

    let lanes = release_hold_reject_lanes();
    export(OUTPUTS[7], &lanes);

    let evidence = evidence_bridge();
    export(OUTPUTS[8], &evidence);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[9], &keepouts);

    let assembly = base
        + doors
        + coupons
        + pressure_ports
        + hepa
        + lockouts
        + traceability
        + lanes
        + evidence
        + keepouts;
    export(OUTPUTS[10], &assembly);

    println!();
    println!("Closed airlock pressure-decay interlock challenge station:");
    println!("  Footprint:                  {DECK_X:.0}mm x {DECK_Y:.0}mm validation deck");
    println!(
        "  Door surrogate plates:      inner and outer plates, {INTERLOCK_PIN_COUNT} interlock challenge pins, {} latch strike blocks",
        DOOR_LATCH_BLOCKS_PER_PLATE * 2
    );
    println!(
        "  Seal coupons:               {COUPON_COUNT} compression coupons, {COMPRESSION_STEP_COUNT} gauge steps from {COMPRESSION_MIN_MM:.0}-{COMPRESSION_MAX_MM:.0}mm"
    );
    println!(
        "  Pressure-decay controls:    {PRESSURE_PORT_COUNT} differential ports, {PRESSURE_SENSOR_POCKETS} sensor pockets, {DECAY_REFERENCE_VOLUME_COUNT} reference volume nests"
    );
    println!(
        "  HEPA purge interface:       {HEPA_DUCT_ID:.0}mm duct ID, {HEPA_FLANGE_BOLT_COUNT} bolt lands, {HEPA_DIFFUSER_SLOT_COUNT} diffuser slots, {PURGE_DAMPER_FLAG_COUNT} damper flags"
    );
    println!(
        "  Evidence workflow:          {LOCKOUT_FLAG_COUNT} lockout flag blocks, {BARCODE_LAND_COUNT} barcode lands, {CERTIFICATE_LAND_COUNT} certificate lands, {WITNESS_TOKEN_COUNT} witness token pockets, {LANE_COUNT} release/hold/reject lanes"
    );
    println!(
        "  Keepouts:                   {FRONT_ROBOT_KEEP_OUT_Y:.0}mm front robot approach, {REAR_SERVICE_KEEP_OUT_Y:.0}mm rear service, {RIGHT_HEPA_SERVICE_KEEP_OUT_X:.0}mm HEPA side service, {TOP_PURGE_DUCT_KEEP_OUT_Z:.0}mm purge duct lift"
    );
    println!("  Required feature groups:    {}", REQUIRED_FEATURES.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn base_validation_deck() -> Part {
    let deck = centered_cube(
        "airlock_pressure_decay_interlock_base_validation_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    );

    let sockets = zone_socket(
        "door_surrogate_plate_socket",
        DOOR_ZONE_POS,
        DOOR_ZONE_X,
        DOOR_ZONE_Y,
    ) + zone_socket("seal_coupon_socket", COUPON_POS, COUPON_X, COUPON_Y)
        + zone_socket("pressure_port_socket", PRESSURE_POS, PRESSURE_X, PRESSURE_Y)
        + zone_socket("hepa_purge_socket", HEPA_POS, HEPA_X, HEPA_Y)
        + zone_socket("lockout_flag_socket", LOCKOUT_POS, LOCKOUT_X, LOCKOUT_Y)
        + zone_socket("traceability_socket", TRACE_POS, TRACE_X, TRACE_Y)
        + zone_socket("disposition_lane_socket", LANES_POS, LANES_X, LANES_Y);

    deck - sockets - mounting_slots()
        + perimeter_lips()
        + datum_rails()
        + wipe_gutters()
        + robot_fiducials()
}

fn zone_socket(name: &str, center: (f64, f64), x: f64, y: f64) -> Part {
    centered_cube(
        format!("airlock_pressure_decay_interlock_{name}"),
        x + 20.0,
        y + 20.0,
        SOCKET_DEPTH + 0.2,
    )
    .translate(center.0, center.1, DECK_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.1)
}

fn perimeter_lips() -> Part {
    let rear = centered_cube(
        "airlock_pressure_decay_interlock_rear_cleanable_lip",
        DECK_X - 112.0,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - 38.0, deck_insert_z(RIM_Z));
    let left = centered_cube(
        "airlock_pressure_decay_interlock_left_cleanable_lip",
        RIM_W,
        DECK_Y - 138.0,
        RIM_Z,
    )
    .translate(-DECK_X / 2.0 + 38.0, 0.0, deck_insert_z(RIM_Z));
    let right = centered_cube(
        "airlock_pressure_decay_interlock_right_low_service_lip",
        RIM_W,
        DECK_Y - 236.0,
        26.0,
    )
    .translate(DECK_X / 2.0 - 38.0, -34.0, deck_insert_z(26.0));
    let front = centered_cube(
        "airlock_pressure_decay_interlock_front_robot_low_lip",
        DECK_X - 254.0,
        12.0,
        16.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 36.0, deck_insert_z(16.0));

    rear + left + right + front
}

fn datum_rails() -> Part {
    let door_heel = centered_cube(
        "airlock_pressure_decay_interlock_door_station_datum_rail",
        DOOR_ZONE_X - 54.0,
        16.0,
        20.0,
    )
    .translate(
        DOOR_ZONE_POS.0,
        DOOR_ZONE_POS.1 - DOOR_ZONE_Y / 2.0 + 30.0,
        deck_insert_z(20.0),
    );
    let coupon_rail = centered_cube(
        "airlock_pressure_decay_interlock_coupon_bank_datum_rail",
        COUPON_X - 44.0,
        14.0,
        18.0,
    )
    .translate(
        COUPON_POS.0,
        COUPON_POS.1 - COUPON_Y / 2.0 + 26.0,
        deck_insert_z(18.0),
    );
    let pressure_rail = centered_cube(
        "airlock_pressure_decay_interlock_pressure_manifold_datum_rail",
        PRESSURE_X - 52.0,
        14.0,
        18.0,
    )
    .translate(
        PRESSURE_POS.0,
        PRESSURE_POS.1 + PRESSURE_Y / 2.0 - 22.0,
        deck_insert_z(18.0),
    );
    let lane_rail = centered_cube(
        "airlock_pressure_decay_interlock_disposition_lane_datum_rail",
        LANES_X - 32.0,
        14.0,
        18.0,
    )
    .translate(
        LANES_POS.0,
        LANES_POS.1 + LANES_Y / 2.0 - 22.0,
        deck_insert_z(18.0),
    );

    door_heel + coupon_rail + pressure_rail + lane_rail
}

fn wipe_gutters() -> Part {
    let front_gutter = centered_cube(
        "airlock_pressure_decay_interlock_front_wipe_gutter",
        DECK_X - 232.0,
        12.0,
        6.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 74.0, DECK_Z / 2.0 - 2.0);
    let side_gutter = centered_cube(
        "airlock_pressure_decay_interlock_left_wipe_gutter",
        12.0,
        DECK_Y - 176.0,
        6.0,
    )
    .translate(-DECK_X / 2.0 + 78.0, 0.0, DECK_Z / 2.0 - 2.0);
    let hepa_sump = centered_cube(
        "airlock_pressure_decay_interlock_hepa_purge_condensate_sump",
        42.0,
        138.0,
        8.0,
    )
    .translate(
        HEPA_POS.0 + HEPA_X / 2.0 - 34.0,
        HEPA_POS.1,
        DECK_Z / 2.0 - 2.6,
    );
    let drain = centered_cylinder(
        "airlock_pressure_decay_interlock_wipe_gutter_drain",
        6.0,
        DECK_Z + 6.0,
        28,
    )
    .translate(DECK_X / 2.0 - 74.0, -DECK_Y / 2.0 + 74.0, 0.0);

    front_gutter + side_gutter + hepa_sump + drain
}

fn mounting_slots() -> Part {
    let mut slots = Part::empty("airlock_pressure_decay_interlock_mounting_slots");
    for (i, (x, y)) in deck_mount_points().iter().enumerate() {
        let round = centered_cylinder(
            format!("airlock_pressure_decay_interlock_m6_mount_round_{i}"),
            MOUNT_HOLE_D / 2.0,
            DECK_Z + 6.0,
            24,
        )
        .translate(*x, *y, 0.0);
        let slot = centered_cube(
            format!("airlock_pressure_decay_interlock_m6_mount_slot_{i}"),
            26.0,
            MOUNT_HOLE_D + 0.8,
            DECK_Z + 6.0,
        )
        .translate(*x, *y, 0.0);
        slots = slots + round + slot;
    }
    slots
}

fn deck_mount_points() -> [(f64, f64); 10] {
    [
        (-(DECK_X / 2.0 - 72.0), -(DECK_Y / 2.0 - 70.0)),
        (DECK_X / 2.0 - 72.0, -(DECK_Y / 2.0 - 70.0)),
        (-(DECK_X / 2.0 - 72.0), DECK_Y / 2.0 - 72.0),
        (DECK_X / 2.0 - 72.0, DECK_Y / 2.0 - 72.0),
        (DOOR_ZONE_POS.0 - DOOR_ZONE_X / 2.0 + 64.0, DOOR_ZONE_POS.1),
        (DOOR_ZONE_POS.0 + DOOR_ZONE_X / 2.0 - 64.0, DOOR_ZONE_POS.1),
        (PRESSURE_POS.0 - PRESSURE_X / 2.0 + 64.0, PRESSURE_POS.1),
        (PRESSURE_POS.0 + PRESSURE_X / 2.0 - 64.0, PRESSURE_POS.1),
        (HEPA_POS.0, HEPA_POS.1 - HEPA_Y / 2.0 + 36.0),
        (LANES_POS.0, LANES_POS.1),
    ]
}

fn robot_fiducials() -> Part {
    let mut fiducials = Part::empty("airlock_pressure_decay_interlock_robot_fiducials");
    for (i, (x, y)) in [
        (-(DECK_X / 2.0 - 102.0), -(DECK_Y / 2.0 - 104.0)),
        (DECK_X / 2.0 - 102.0, -(DECK_Y / 2.0 - 104.0)),
        (-(DECK_X / 2.0 - 102.0), DECK_Y / 2.0 - 104.0),
        (DECK_X / 2.0 - 102.0, DECK_Y / 2.0 - 104.0),
    ]
    .iter()
    .enumerate()
    {
        fiducials =
            fiducials
                + fiducial_target(&format!("airlock_pressure_decay_interlock_fiducial_{i}"))
                    .translate(*x, *y, DECK_Z / 2.0 + 2.0);
    }
    fiducials
}

fn fiducial_target(name: &str) -> Part {
    let outer = centered_cylinder(format!("{name}_outer_ring"), 18.0, 2.0, 44);
    let inner = centered_cylinder(format!("{name}_center_clearance"), 8.0, 3.0, 36);
    let cross_x = centered_cube(format!("{name}_cross_x"), 36.0, 4.0, 2.6);
    let cross_y = centered_cube(format!("{name}_cross_y"), 4.0, 36.0, 2.6);
    outer - inner + cross_x + cross_y
}

fn door_surrogate_plates() -> Part {
    let inner = door_plate("inner", -(DOOR_PITCH_X / 2.0));
    let outer = door_plate("outer", DOOR_PITCH_X / 2.0);
    let gap_reference = centered_cube(
        "airlock_pressure_decay_interlock_door_gap_reference_bar",
        30.0,
        DOOR_PLATE_Y + 54.0,
        18.0,
    )
    .translate(0.0, 0.0, DOOR_PLATE_Z / 2.0 + 9.0);
    let interlock_pins = interlock_challenge_pin_bank();

    (inner + outer + gap_reference + interlock_pins).translate(
        DOOR_ZONE_POS.0,
        DOOR_ZONE_POS.1,
        deck_top_z(),
    )
}

fn door_plate(label: &str, x_offset: f64) -> Part {
    let plate = centered_cube(
        format!("airlock_pressure_decay_interlock_{label}_door_surrogate_plate"),
        DOOR_PLATE_X,
        DOOR_PLATE_Y,
        DOOR_PLATE_Z,
    );
    let gasket_groove = rect_frame(
        &format!("airlock_pressure_decay_interlock_{label}_door_gasket_land"),
        DOOR_PLATE_X - 42.0,
        DOOR_PLATE_Y - 42.0,
        DOOR_GASKET_W,
        DOOR_GASKET_Z,
    )
    .translate(0.0, 0.0, DOOR_PLATE_Z / 2.0 + DOOR_GASKET_Z / 2.0);
    let challenge_leak_path = centered_cube(
        format!("airlock_pressure_decay_interlock_{label}_calibrated_leak_path_shadow"),
        DOOR_PLATE_X - 92.0,
        10.0,
        4.0,
    )
    .translate(0.0, -(DOOR_PLATE_Y / 2.0 - 78.0), DOOR_PLATE_Z / 2.0 + 3.0);
    let hinge = hinge_barrel(label);
    let latch = latch_strike_blocks(label);
    let handle = centered_cube(
        format!("airlock_pressure_decay_interlock_{label}_door_handle_surrogate"),
        22.0,
        92.0,
        22.0,
    )
    .translate(DOOR_PLATE_X / 2.0 - 52.0, 0.0, DOOR_PLATE_Z / 2.0 + 18.0);
    let sight_land = centered_cube(
        format!("airlock_pressure_decay_interlock_{label}_view_window_evidence_land"),
        96.0,
        56.0,
        5.0,
    )
    .translate(-24.0, 0.0, DOOR_PLATE_Z / 2.0 + 2.5);

    (plate + gasket_groove + challenge_leak_path + hinge + latch + handle + sight_land).translate(
        x_offset,
        0.0,
        DOOR_PLATE_Z / 2.0,
    )
}

fn hinge_barrel(label: &str) -> Part {
    let mut hinge = Part::empty(format!(
        "airlock_pressure_decay_interlock_{label}_hinge_barrels"
    ));
    for (i, y) in [-82.0, 0.0, 82.0].iter().enumerate() {
        let barrel = centered_cylinder(
            format!("airlock_pressure_decay_interlock_{label}_hinge_barrel_{i}"),
            DOOR_HINGE_D / 2.0,
            54.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(-(DOOR_PLATE_X / 2.0 + 10.0), *y, DOOR_PLATE_Z / 2.0 + 16.0);
        let pin = centered_cylinder(
            format!("airlock_pressure_decay_interlock_{label}_hinge_pin_clearance_{i}"),
            3.2,
            60.0,
            20,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(-(DOOR_PLATE_X / 2.0 + 10.0), *y, DOOR_PLATE_Z / 2.0 + 16.0);
        hinge = hinge + (barrel - pin);
    }
    hinge
}

fn latch_strike_blocks(label: &str) -> Part {
    let mut blocks = Part::empty(format!(
        "airlock_pressure_decay_interlock_{label}_latch_strike_blocks"
    ));
    for i in 0..DOOR_LATCH_BLOCKS_PER_PLATE {
        let y = centered_index(i, DOOR_LATCH_BLOCKS_PER_PLATE, 72.0);
        let block = centered_cube(
            format!("airlock_pressure_decay_interlock_{label}_latch_strike_block_{i}"),
            38.0,
            26.0,
            28.0,
        )
        .translate(DOOR_PLATE_X / 2.0 - 22.0, y, DOOR_PLATE_Z / 2.0 + 14.0);
        let keeper = centered_cube(
            format!("airlock_pressure_decay_interlock_{label}_latch_keeper_clearance_{i}"),
            18.0,
            9.0,
            30.0,
        )
        .translate(DOOR_PLATE_X / 2.0 - 22.0, y, DOOR_PLATE_Z / 2.0 + 14.0);
        blocks = blocks + (block - keeper);
    }
    blocks
}

fn interlock_challenge_pin_bank() -> Part {
    let rail = centered_cube(
        "airlock_pressure_decay_interlock_interlock_pin_reference_rail",
        52.0,
        DOOR_PLATE_Y - 32.0,
        26.0,
    )
    .translate(0.0, 0.0, DOOR_PLATE_Z / 2.0 + 13.0);
    let mut pins = Part::empty("airlock_pressure_decay_interlock_challenge_pin_bank");
    for i in 0..INTERLOCK_PIN_COUNT {
        let y = centered_index(i, INTERLOCK_PIN_COUNT, 54.0);
        let pin = centered_cylinder(
            format!("airlock_pressure_decay_interlock_go_no_go_pin_{i}"),
            6.0 + i as f64,
            34.0,
            28,
        )
        .translate(0.0, y, DOOR_PLATE_Z / 2.0 + 17.0);
        let sleeve = centered_cylinder(
            format!("airlock_pressure_decay_interlock_pin_socket_{i}"),
            10.5 + i as f64,
            12.0,
            28,
        )
        .translate(0.0, y, DOOR_PLATE_Z / 2.0 + 6.0);
        pins = pins + pin + sleeve;
    }
    rail + pins
}

fn seal_compression_coupons() -> Part {
    let base = centered_cube(
        "airlock_pressure_decay_interlock_seal_compression_coupon_panel",
        COUPON_X,
        COUPON_Y,
        COUPON_Z,
    );
    let recess = centered_cube(
        "airlock_pressure_decay_interlock_seal_coupon_panel_cleanout_recess",
        COUPON_X - 46.0,
        COUPON_Y - 44.0,
        12.0,
    )
    .translate(0.0, 0.0, COUPON_Z / 2.0 - 5.0);
    let mut coupons = Part::empty("airlock_pressure_decay_interlock_seal_coupon_nests");
    for row in 0..COUPON_ROWS {
        for col in 0..COUPON_COLS {
            let idx = coupon_index(row, col);
            let (x, y) = coupon_xy(row, col);
            coupons = coupons + seal_coupon_nest(idx).translate(x, y, COUPON_Z / 2.0 + 8.0);
        }
    }
    let steps = compression_gauge_steps();
    let witness_slots = coupon_witness_slots();

    (base - recess + coupons + steps + witness_slots).translate(
        COUPON_POS.0,
        COUPON_POS.1,
        deck_top_z() + COUPON_Z / 2.0,
    )
}

fn seal_coupon_nest(index: usize) -> Part {
    let pocket = centered_cylinder(
        format!("airlock_pressure_decay_interlock_seal_coupon_outer_nest_{index}"),
        SEAL_COUPON_OUTER_D / 2.0 + 5.0,
        12.0,
        48,
    );
    let coupon = centered_cylinder(
        format!("airlock_pressure_decay_interlock_seal_coupon_ring_{index}"),
        SEAL_COUPON_OUTER_D / 2.0,
        7.0,
        48,
    );
    let coupon_inner = centered_cylinder(
        format!("airlock_pressure_decay_interlock_seal_coupon_inner_clearance_{index}"),
        SEAL_COUPON_INNER_D / 2.0,
        8.0,
        44,
    );
    let witness_tick = centered_cube(
        format!("airlock_pressure_decay_interlock_seal_coupon_clocking_tick_{index}"),
        6.0,
        18.0,
        8.0,
    )
    .translate(SEAL_COUPON_OUTER_D / 2.0 + 8.0, 0.0, 1.0);

    pocket + (coupon - coupon_inner) + witness_tick
}

fn compression_gauge_steps() -> Part {
    let mut steps = Part::empty("airlock_pressure_decay_interlock_compression_gauge_steps");
    for i in 0..COMPRESSION_STEP_COUNT {
        let height = COMPRESSION_MIN_MM
            + (COMPRESSION_MAX_MM - COMPRESSION_MIN_MM) * i as f64
                / (COMPRESSION_STEP_COUNT as f64 - 1.0);
        steps = steps
            + centered_cube(
                format!("airlock_pressure_decay_interlock_compression_step_{i}"),
                34.0,
                38.0,
                height,
            )
            .translate(
                -COUPON_X / 2.0 + 44.0 + i as f64 * 42.0,
                -(COUPON_Y / 2.0 - 28.0),
                COUPON_Z / 2.0 + height / 2.0,
            );
    }
    steps
}

fn coupon_witness_slots() -> Part {
    let mut slots = Part::empty("airlock_pressure_decay_interlock_coupon_witness_slots");
    for i in 0..COUPON_COLS {
        slots = slots
            + centered_cube(
                format!("airlock_pressure_decay_interlock_coupon_witness_slot_{i}"),
                54.0,
                8.0,
                8.0,
            )
            .translate(
                centered_index(i, COUPON_COLS, COUPON_PITCH_X),
                COUPON_Y / 2.0 - 28.0,
                COUPON_Z / 2.0 + 4.0,
            );
    }
    slots
}

fn differential_pressure_ports() -> Part {
    let block = centered_cube(
        "airlock_pressure_decay_interlock_differential_pressure_manifold_block",
        PRESSURE_X,
        PRESSURE_Y,
        PRESSURE_Z,
    );
    let service_recess = centered_cube(
        "airlock_pressure_decay_interlock_pressure_sensor_service_recess",
        PRESSURE_X - 72.0,
        PRESSURE_Y - 64.0,
        10.0,
    )
    .translate(0.0, 0.0, PRESSURE_Z / 2.0 - 4.0);

    let mut ports = Part::empty("airlock_pressure_decay_interlock_differential_port_lands");
    for pair in 0..PRESSURE_PAIR_COUNT {
        let x = centered_index(pair, PRESSURE_PAIR_COUNT, PRESSURE_PAIR_PITCH_X);
        ports = ports
            + pressure_port_pair(pair, x)
            + sensor_pocket(pair, x)
            + reference_volume_nest(pair, x);
    }

    let bridge_channel = centered_cylinder(
        "airlock_pressure_decay_interlock_equalization_channel_shadow",
        6.0,
        PRESSURE_X - 106.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, PRESSURE_Z / 2.0 + 16.0);

    (block - service_recess + ports + bridge_channel).translate(
        PRESSURE_POS.0,
        PRESSURE_POS.1,
        deck_top_z() + PRESSURE_Z / 2.0,
    )
}

fn pressure_port_pair(pair: usize, x: f64) -> Part {
    let inner = pressure_port_land(
        &format!("airlock_pressure_decay_interlock_pair_{pair}_inner_airlock_port"),
        x,
        -PRESSURE_PORT_PITCH_Y / 2.0,
    );
    let outer = pressure_port_land(
        &format!("airlock_pressure_decay_interlock_pair_{pair}_outer_airlock_port"),
        x,
        PRESSURE_PORT_PITCH_Y / 2.0,
    );
    let pair_bridge = centered_cube(
        format!("airlock_pressure_decay_interlock_pair_{pair}_delta_p_bridge_land"),
        28.0,
        PRESSURE_PORT_PITCH_Y + 16.0,
        6.0,
    )
    .translate(x, 0.0, PRESSURE_Z / 2.0 + 3.0);
    inner + outer + pair_bridge
}

fn pressure_port_land(name: &str, x: f64, y: f64) -> Part {
    let boss = centered_cylinder(format!("{name}_boss"), 18.0, 12.0, 36).translate(
        x,
        y,
        PRESSURE_Z / 2.0 + 6.0,
    );
    let bore = centered_cylinder(format!("{name}_tube_bore"), PRESSURE_PORT_D / 2.0, 14.0, 28)
        .translate(x, y, PRESSURE_Z / 2.0 + 6.0);
    boss - bore
}

fn sensor_pocket(pair: usize, x: f64) -> Part {
    centered_cube(
        format!("airlock_pressure_decay_interlock_delta_p_sensor_pocket_{pair}"),
        58.0,
        22.0,
        30.0,
    )
    .translate(x, 0.0, PRESSURE_Z / 2.0 + 17.0)
}

fn reference_volume_nest(pair: usize, x: f64) -> Part {
    if pair >= DECAY_REFERENCE_VOLUME_COUNT {
        return Part::empty(format!(
            "airlock_pressure_decay_interlock_reference_volume_absent_{pair}"
        ));
    }

    let nest = centered_cylinder(
        format!("airlock_pressure_decay_interlock_reference_volume_nest_{pair}"),
        20.0 + pair as f64 * 3.0,
        40.0,
        36,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(x, -(PRESSURE_Y / 2.0 - 24.0), PRESSURE_Z / 2.0 + 24.0);
    let end_stop = centered_cube(
        format!("airlock_pressure_decay_interlock_reference_volume_stop_{pair}"),
        8.0,
        38.0,
        40.0,
    )
    .translate(
        x + 28.0,
        -(PRESSURE_Y / 2.0 - 24.0),
        PRESSURE_Z / 2.0 + 24.0,
    );

    nest + end_stop
}

fn hepa_purge_duct_interface() -> Part {
    let base = centered_cube(
        "airlock_pressure_decay_interlock_hepa_purge_plenum_base",
        HEPA_X,
        HEPA_Y,
        HEPA_Z,
    );
    let plenum_recess = centered_cube(
        "airlock_pressure_decay_interlock_hepa_purge_plenum_recess",
        HEPA_X - 84.0,
        HEPA_Y - 74.0,
        14.0,
    )
    .translate(0.0, 0.0, HEPA_Z / 2.0 - 6.0);
    let flange = hepa_round_flange();
    let diffuser = purge_diffuser_slots();
    let damper = purge_damper_flags();
    let clamps = hepa_quick_clamp_lugs();

    (base - plenum_recess + flange + diffuser + damper + clamps).translate(
        HEPA_POS.0,
        HEPA_POS.1,
        deck_top_z() + HEPA_Z / 2.0,
    )
}

fn hepa_round_flange() -> Part {
    let flange = centered_cylinder(
        "airlock_pressure_decay_interlock_hepa_purge_round_flange_outer",
        HEPA_DUCT_OD / 2.0,
        18.0,
        72,
    )
    .translate(0.0, 0.0, HEPA_Z / 2.0 + 9.0);
    let bore = centered_cylinder(
        "airlock_pressure_decay_interlock_hepa_purge_round_flange_bore",
        HEPA_DUCT_ID / 2.0,
        22.0,
        72,
    )
    .translate(0.0, 0.0, HEPA_Z / 2.0 + 9.0);

    let mut bolts = Part::empty("airlock_pressure_decay_interlock_hepa_flange_bolt_lands");
    for i in 0..HEPA_FLANGE_BOLT_COUNT {
        let theta = i as f64 * 360.0 / HEPA_FLANGE_BOLT_COUNT as f64;
        let (x, y) = polar_xy(theta, HEPA_DUCT_OD / 2.0 - 18.0);
        let boss = centered_cylinder(
            format!("airlock_pressure_decay_interlock_hepa_flange_bolt_boss_{i}"),
            9.0,
            22.0,
            28,
        )
        .translate(x, y, HEPA_Z / 2.0 + 11.0);
        let hole = centered_cylinder(
            format!("airlock_pressure_decay_interlock_hepa_flange_bolt_clearance_{i}"),
            3.3,
            24.0,
            20,
        )
        .translate(x, y, HEPA_Z / 2.0 + 11.0);
        bolts = bolts + (boss - hole);
    }

    flange - bore + bolts
}

fn purge_diffuser_slots() -> Part {
    let mut diffuser = Part::empty("airlock_pressure_decay_interlock_hepa_diffuser_slots");
    for i in 0..HEPA_DIFFUSER_SLOT_COUNT {
        let x = centered_index(i, HEPA_DIFFUSER_SLOT_COUNT, 42.0);
        let slot = centered_cube(
            format!("airlock_pressure_decay_interlock_hepa_diffuser_slot_{i}"),
            24.0,
            HEPA_Y - 82.0,
            12.0,
        )
        .translate(x, 0.0, HEPA_Z / 2.0 + 21.0);
        let slot_lip = centered_cube(
            format!("airlock_pressure_decay_interlock_hepa_diffuser_slot_lip_{i}"),
            30.0,
            8.0,
            18.0,
        )
        .translate(x, HEPA_Y / 2.0 - 52.0, HEPA_Z / 2.0 + 24.0);
        diffuser = diffuser + slot + slot_lip;
    }
    diffuser
}

fn purge_damper_flags() -> Part {
    let mut flags = Part::empty("airlock_pressure_decay_interlock_purge_damper_flags");
    for i in 0..PURGE_DAMPER_FLAG_COUNT {
        let y = centered_index(i, PURGE_DAMPER_FLAG_COUNT, 54.0);
        let pivot = centered_cylinder(
            format!("airlock_pressure_decay_interlock_purge_damper_pivot_{i}"),
            8.0,
            32.0,
            28,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(-(HEPA_X / 2.0 - 38.0), y, HEPA_Z / 2.0 + 34.0);
        let blade = centered_cube(
            format!("airlock_pressure_decay_interlock_purge_damper_visible_blade_{i}"),
            54.0,
            10.0,
            22.0,
        )
        .translate(-(HEPA_X / 2.0 - 72.0), y, HEPA_Z / 2.0 + 34.0);
        flags = flags + pivot + blade;
    }
    flags
}

fn hepa_quick_clamp_lugs() -> Part {
    let mut lugs = Part::empty("airlock_pressure_decay_interlock_hepa_quick_clamp_lugs");
    for (i, x) in [-(HEPA_X / 2.0 - 46.0), HEPA_X / 2.0 - 46.0]
        .iter()
        .enumerate()
    {
        for (j, y) in [-(HEPA_Y / 2.0 - 42.0), HEPA_Y / 2.0 - 42.0]
            .iter()
            .enumerate()
        {
            let idx = i * 2 + j;
            let lug = centered_cube(
                format!("airlock_pressure_decay_interlock_hepa_quick_clamp_lug_{idx}"),
                38.0,
                28.0,
                26.0,
            )
            .translate(*x, *y, HEPA_Z / 2.0 + 13.0);
            let pin = centered_cylinder(
                format!("airlock_pressure_decay_interlock_hepa_quick_clamp_pin_{idx}"),
                4.0,
                44.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, *y, HEPA_Z / 2.0 + 13.0);
            lugs = lugs + (lug - pin);
        }
    }
    lugs
}

fn lockout_flag_blocks() -> Part {
    let base = centered_cube(
        "airlock_pressure_decay_interlock_lockout_flag_block_base",
        LOCKOUT_X,
        LOCKOUT_Y,
        LOCKOUT_Z,
    );
    let mut flags = Part::empty("airlock_pressure_decay_interlock_lockout_flag_blocks");
    for i in 0..LOCKOUT_FLAG_COUNT {
        let x = centered_index(i, LOCKOUT_FLAG_COUNT, LOCKOUT_FLAG_PITCH_X);
        let pocket = centered_cube(
            format!("airlock_pressure_decay_interlock_lockout_flag_pocket_{i}"),
            31.0,
            54.0,
            14.0,
        )
        .translate(x, -18.0, LOCKOUT_Z / 2.0 + 7.0);
        let pivot = centered_cylinder(
            format!("airlock_pressure_decay_interlock_lockout_flag_pivot_{i}"),
            5.0,
            38.0,
            22,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -38.0, LOCKOUT_Z / 2.0 + 18.0);
        let flag = centered_cube(
            format!("airlock_pressure_decay_interlock_lockout_flag_throw_{i}"),
            22.0,
            7.0,
            LOCKOUT_FLAG_THROW_Z,
        )
        .translate(x, -58.0, LOCKOUT_Z / 2.0 + LOCKOUT_FLAG_THROW_Z / 2.0);
        let challenge_key = centered_cube(
            format!("airlock_pressure_decay_interlock_lockout_challenge_key_{i}"),
            26.0,
            20.0,
            9.0,
        )
        .translate(x, 42.0, LOCKOUT_Z / 2.0 + 4.5);
        flags = flags + pocket + pivot + flag + challenge_key;
    }
    let cable_trough = centered_cube(
        "airlock_pressure_decay_interlock_lockout_switch_cable_trough",
        LOCKOUT_X - 44.0,
        12.0,
        8.0,
    )
    .translate(0.0, LOCKOUT_Y / 2.0 - 22.0, LOCKOUT_Z / 2.0 + 4.0);

    (base + flags + cable_trough).translate(
        LOCKOUT_POS.0,
        LOCKOUT_POS.1,
        deck_top_z() + LOCKOUT_Z / 2.0,
    )
}

fn barcode_certificate_lands() -> Part {
    let panel = centered_cube(
        "airlock_pressure_decay_interlock_barcode_certificate_panel",
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    );
    let mut lands = Part::empty("airlock_pressure_decay_interlock_traceability_lands");
    for i in 0..BARCODE_LAND_COUNT {
        let x = centered_index(i % 3, 3, 98.0);
        let y = if i < 3 { 36.0 } else { -12.0 };
        lands = lands
            + centered_cube(
                format!("airlock_pressure_decay_interlock_barcode_land_{i}"),
                78.0,
                28.0,
                5.0,
            )
            .translate(x - 62.0, y, TRACE_Z / 2.0 + 2.5);
    }
    for i in 0..CERTIFICATE_LAND_COUNT {
        lands = lands
            + centered_cube(
                format!("airlock_pressure_decay_interlock_certificate_land_{i}"),
                78.0,
                42.0,
                5.0,
            )
            .translate(
                126.0,
                centered_index(i, CERTIFICATE_LAND_COUNT, 42.0),
                TRACE_Z / 2.0 + 2.5,
            );
    }
    for i in 0..WITNESS_TOKEN_COUNT {
        lands = lands
            + centered_cylinder(
                format!("airlock_pressure_decay_interlock_evidence_witness_token_pocket_{i}"),
                10.0,
                6.0,
                28,
            )
            .translate(
                -TRACE_X / 2.0 + 26.0 + i as f64 * 22.0,
                -(TRACE_Y / 2.0 - 20.0),
                TRACE_Z / 2.0 + 3.0,
            );
    }

    (panel + lands).translate(TRACE_POS.0, TRACE_POS.1, deck_top_z() + TRACE_Z / 2.0)
}

fn release_hold_reject_lanes() -> Part {
    let base = centered_cube(
        "airlock_pressure_decay_interlock_release_hold_reject_lane_panel",
        LANES_X,
        LANES_Y,
        LANES_Z,
    );
    let mut lanes = Part::empty("airlock_pressure_decay_interlock_disposition_lanes");
    for lane in DispositionLane::all() {
        lanes = lanes + disposition_lane(lane);
    }
    (base + lanes).translate(LANES_POS.0, LANES_POS.1, deck_top_z() + LANES_Z / 2.0)
}

fn disposition_lane(lane: DispositionLane) -> Part {
    let x = lane_x(lane);
    let label = lane.label();
    let floor = centered_cube(
        format!("airlock_pressure_decay_interlock_{label}_lane_floor_land"),
        LANE_WIDTH_X,
        LANE_RAIL_Y,
        5.0,
    )
    .translate(x, 0.0, LANES_Z / 2.0 + 2.5);
    let left_rail = centered_cube(
        format!("airlock_pressure_decay_interlock_{label}_lane_left_rail"),
        8.0,
        LANE_RAIL_Y,
        26.0,
    )
    .translate(x - LANE_WIDTH_X / 2.0 + 6.0, 0.0, LANES_Z / 2.0 + 13.0);
    let right_rail = centered_cube(
        format!("airlock_pressure_decay_interlock_{label}_lane_right_rail"),
        8.0,
        LANE_RAIL_Y,
        26.0,
    )
    .translate(x + LANE_WIDTH_X / 2.0 - 6.0, 0.0, LANES_Z / 2.0 + 13.0);
    let gate = centered_cube(
        format!("airlock_pressure_decay_interlock_{label}_lane_gate"),
        LANE_WIDTH_X - 20.0,
        10.0,
        LANE_GATE_Z,
    )
    .translate(
        x,
        LANE_RAIL_Y / 2.0 - 10.0,
        LANES_Z / 2.0 + LANE_GATE_Z / 2.0,
    );
    let mut token_slots = Part::empty(format!(
        "airlock_pressure_decay_interlock_{label}_lane_token_slots"
    ));
    for i in 0..LANE_TOKEN_SLOTS {
        token_slots = token_slots
            + centered_cube(
                format!("airlock_pressure_decay_interlock_{label}_lane_token_slot_{i}"),
                18.0,
                26.0,
                7.0,
            )
            .translate(
                x,
                -LANE_RAIL_Y / 2.0 + 22.0 + i as f64 * 24.0,
                LANES_Z / 2.0 + 3.5,
            );
    }

    floor + left_rail + right_rail + gate + token_slots
}

fn evidence_bridge() -> Part {
    let left_post = centered_cube(
        "airlock_pressure_decay_interlock_evidence_bridge_left_post",
        EVIDENCE_POST_X,
        EVIDENCE_POST_Y,
        EVIDENCE_UNDERSIDE_Z,
    )
    .translate(-EVIDENCE_SPAN_X / 2.0, 0.0, EVIDENCE_UNDERSIDE_Z / 2.0);
    let right_post = centered_cube(
        "airlock_pressure_decay_interlock_evidence_bridge_right_post",
        EVIDENCE_POST_X,
        EVIDENCE_POST_Y,
        EVIDENCE_UNDERSIDE_Z,
    )
    .translate(EVIDENCE_SPAN_X / 2.0, 0.0, EVIDENCE_UNDERSIDE_Z / 2.0);
    let beam = centered_cube(
        "airlock_pressure_decay_interlock_evidence_bridge_camera_beam",
        EVIDENCE_SPAN_X + EVIDENCE_POST_X,
        EVIDENCE_POST_Y,
        EVIDENCE_BEAM_Z,
    )
    .translate(0.0, 0.0, EVIDENCE_UNDERSIDE_Z + EVIDENCE_BEAM_Z / 2.0);
    let cameras = evidence_camera_bank();
    let lights = evidence_light_segments();
    let certificate_sight_bar = centered_cube(
        "airlock_pressure_decay_interlock_certificate_sight_bar",
        TRACE_X + LANES_X + 80.0,
        10.0,
        12.0,
    )
    .translate(250.0, -32.0, EVIDENCE_UNDERSIDE_Z - 18.0);

    (left_post + right_post + beam + cameras + lights + certificate_sight_bar).translate(
        EVIDENCE_POS.0,
        EVIDENCE_POS.1,
        deck_top_z(),
    )
}

fn evidence_camera_bank() -> Part {
    let mut cameras = Part::empty("airlock_pressure_decay_interlock_evidence_camera_bank");
    for i in 0..EVIDENCE_CAMERA_COUNT {
        let x = centered_index(i, EVIDENCE_CAMERA_COUNT, EVIDENCE_SPAN_X / 4.6);
        let mount = centered_cube(
            format!("airlock_pressure_decay_interlock_evidence_camera_mount_{i}"),
            46.0,
            36.0,
            18.0,
        )
        .translate(
            x,
            -EVIDENCE_POST_Y / 2.0 - 10.0,
            EVIDENCE_UNDERSIDE_Z - 24.0,
        );
        let lens = centered_cylinder(
            format!("airlock_pressure_decay_interlock_evidence_camera_lens_{i}"),
            12.0,
            18.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            x,
            -EVIDENCE_POST_Y / 2.0 - 22.0,
            EVIDENCE_UNDERSIDE_Z - 24.0,
        );
        cameras = cameras + mount + lens;
    }
    cameras
}

fn evidence_light_segments() -> Part {
    let mut lights = Part::empty("airlock_pressure_decay_interlock_evidence_light_segments");
    for i in 0..EVIDENCE_LIGHT_SEGMENTS {
        lights = lights
            + centered_cube(
                format!("airlock_pressure_decay_interlock_evidence_light_segment_{i}"),
                72.0,
                8.0,
                8.0,
            )
            .translate(
                centered_index(i, EVIDENCE_LIGHT_SEGMENTS, 92.0),
                EVIDENCE_POST_Y / 2.0 + 6.0,
                EVIDENCE_UNDERSIDE_Z - 22.0,
            );
    }
    lights
}

fn robot_service_keepouts() -> Part {
    let front_robot = wire_box(
        "airlock_pressure_decay_interlock_front_robot_pick_keepout",
        DECK_X - 210.0,
        FRONT_ROBOT_KEEP_OUT_Y,
        ROBOT_PICK_CLEARANCE_Z,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + FRONT_ROBOT_KEEP_OUT_Y / 2.0 + 24.0,
        deck_top_z(),
    );
    let rear_service = wire_box(
        "airlock_pressure_decay_interlock_rear_service_keepout",
        DECK_X - 250.0,
        REAR_SERVICE_KEEP_OUT_Y,
        250.0,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - REAR_SERVICE_KEEP_OUT_Y / 2.0 - 28.0,
        deck_top_z(),
    );
    let left_door_service = wire_box(
        "airlock_pressure_decay_interlock_left_door_service_swing_keepout",
        LEFT_DOOR_SERVICE_KEEP_OUT_X,
        DOOR_ZONE_Y + 140.0,
        260.0,
    )
    .translate(
        DOOR_ZONE_POS.0 - DOOR_ZONE_X / 2.0 - LEFT_DOOR_SERVICE_KEEP_OUT_X / 2.0 + 48.0,
        DOOR_ZONE_POS.1,
        deck_top_z(),
    );
    let right_hepa_service = wire_box(
        "airlock_pressure_decay_interlock_right_hepa_filter_service_keepout",
        RIGHT_HEPA_SERVICE_KEEP_OUT_X,
        HEPA_Y + 120.0,
        TOP_PURGE_DUCT_KEEP_OUT_Z,
    )
    .translate(
        HEPA_POS.0 + HEPA_X / 2.0 + RIGHT_HEPA_SERVICE_KEEP_OUT_X / 2.0 - 42.0,
        HEPA_POS.1,
        deck_top_z(),
    );
    let duct_lift = wire_box(
        "airlock_pressure_decay_interlock_top_purge_duct_lift_keepout",
        HEPA_X + 120.0,
        HEPA_Y + 112.0,
        TOP_PURGE_DUCT_KEEP_OUT_Z,
    )
    .translate(HEPA_POS.0, HEPA_POS.1, deck_top_z());

    front_robot + rear_service + left_door_service + right_hepa_service + duct_lift
}

fn rect_frame(name: &str, x: f64, y: f64, rail_w: f64, z: f64) -> Part {
    let top = centered_cube(format!("{name}_top"), x, rail_w, z).translate(0.0, y / 2.0, 0.0);
    let bottom =
        centered_cube(format!("{name}_bottom"), x, rail_w, z).translate(0.0, -y / 2.0, 0.0);
    let left =
        centered_cube(format!("{name}_left"), rail_w, y + rail_w, z).translate(-x / 2.0, 0.0, 0.0);
    let right =
        centered_cube(format!("{name}_right"), rail_w, y + rail_w, z).translate(x / 2.0, 0.0, 0.0);
    top + bottom + left + right
}

fn wire_box(name: &str, x: f64, y: f64, z: f64) -> Part {
    let mut frame = Part::empty(format!("{name}_wireframe"));

    for (i, zpos) in [KEEP_OUT_RAIL / 2.0, z - KEEP_OUT_RAIL / 2.0]
        .iter()
        .enumerate()
    {
        frame = frame
            + centered_cube(
                format!("{name}_front_x_rail_{i}"),
                x,
                KEEP_OUT_RAIL,
                KEEP_OUT_RAIL,
            )
            .translate(0.0, -(y / 2.0 - KEEP_OUT_RAIL / 2.0), *zpos)
            + centered_cube(
                format!("{name}_rear_x_rail_{i}"),
                x,
                KEEP_OUT_RAIL,
                KEEP_OUT_RAIL,
            )
            .translate(0.0, y / 2.0 - KEEP_OUT_RAIL / 2.0, *zpos)
            + centered_cube(
                format!("{name}_left_y_rail_{i}"),
                KEEP_OUT_RAIL,
                y,
                KEEP_OUT_RAIL,
            )
            .translate(-(x / 2.0 - KEEP_OUT_RAIL / 2.0), 0.0, *zpos)
            + centered_cube(
                format!("{name}_right_y_rail_{i}"),
                KEEP_OUT_RAIL,
                y,
                KEEP_OUT_RAIL,
            )
            .translate(x / 2.0 - KEEP_OUT_RAIL / 2.0, 0.0, *zpos);
    }

    for (i, (xsign, ysign)) in [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)]
        .iter()
        .enumerate()
    {
        frame = frame
            + centered_cube(
                format!("{name}_vertical_post_{i}"),
                KEEP_OUT_RAIL,
                KEEP_OUT_RAIL,
                z,
            )
            .translate(
                xsign * (x / 2.0 - KEEP_OUT_RAIL / 2.0),
                ysign * (y / 2.0 - KEEP_OUT_RAIL / 2.0),
                z / 2.0,
            );
    }

    frame
}

fn coupon_xy(row: usize, col: usize) -> (f64, f64) {
    (
        centered_index(col, COUPON_COLS, COUPON_PITCH_X),
        centered_index(row, COUPON_ROWS, COUPON_PITCH_Y) + 12.0,
    )
}

fn coupon_index(row: usize, col: usize) -> usize {
    row * COUPON_COLS + col
}

fn lane_x(lane: DispositionLane) -> f64 {
    centered_index(lane.index(), LANE_COUNT, LANE_PITCH_X)
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn deck_top_z() -> f64 {
    DECK_Z / 2.0
}

fn deck_insert_z(height: f64) -> f64 {
    DECK_Z / 2.0 + height / 2.0
}

fn polar_xy(theta_degrees: f64, radius: f64) -> (f64, f64) {
    let radians = theta_degrees.to_radians();
    (radius * radians.cos(), radius * radians.sin())
}

fn layout_rects() -> [Rect; 7] {
    [
        Rect {
            name: "door_zone",
            center: DOOR_ZONE_POS,
            x: DOOR_ZONE_X,
            y: DOOR_ZONE_Y,
        },
        Rect {
            name: "coupon_zone",
            center: COUPON_POS,
            x: COUPON_X,
            y: COUPON_Y,
        },
        Rect {
            name: "pressure_zone",
            center: PRESSURE_POS,
            x: PRESSURE_X,
            y: PRESSURE_Y,
        },
        Rect {
            name: "hepa_zone",
            center: HEPA_POS,
            x: HEPA_X,
            y: HEPA_Y,
        },
        Rect {
            name: "lockout_zone",
            center: LOCKOUT_POS,
            x: LOCKOUT_X,
            y: LOCKOUT_Y,
        },
        Rect {
            name: "traceability_zone",
            center: TRACE_POS,
            x: TRACE_X,
            y: TRACE_Y,
        },
        Rect {
            name: "disposition_lane_zone",
            center: LANES_POS,
            x: LANES_X,
            y: LANES_Y,
        },
    ]
}

fn assert_design_constraints() {
    for rect in layout_rects() {
        assert!(
            rect.fits_inside_deck(),
            "{} exceeds usable validation deck area",
            rect.name
        );
    }

    let rects = layout_rects();
    for i in 0..rects.len() {
        for j in (i + 1)..rects.len() {
            assert!(
                !rects[i].overlaps(rects[j]),
                "{} overlaps {}",
                rects[i].name,
                rects[j].name
            );
        }
    }

    assert!(DOOR_PITCH_X > DOOR_PLATE_X + 38.0);
    assert_eq!(PRESSURE_PORT_COUNT, PRESSURE_PAIR_COUNT * 2);
    assert!(HEPA_DUCT_ID >= 150.0);
    assert!(FRONT_ROBOT_KEEP_OUT_Y >= 240.0);
    assert!(TOP_PURGE_DUCT_KEEP_OUT_Z >= 340.0);
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 11);
        for path in OUTPUTS {
            assert!(
                path.starts_with(
                    "output/closed_airlock_pressure_decay_interlock_challenge_station_"
                ),
                "{path}"
            );
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn requested_feature_groups_are_explicit() {
        assert_eq!(REQUIRED_FEATURES.len(), 9);
        for expected in [
            "inner_outer_door_surrogate_plates",
            "seal_compression_coupons",
            "differential_pressure_ports",
            "hepa_purge_duct_interface",
            "lockout_flag_blocks",
            "barcode_certificate_lands",
            "release_hold_reject_lanes",
            "evidence_bridge",
            "robot_service_keepouts",
        ] {
            assert!(REQUIRED_FEATURES.contains(&expected));
        }
    }

    #[test]
    fn layout_zones_are_bounded_and_non_overlapping() {
        assert_design_constraints();
    }

    #[test]
    fn pressure_decay_and_purge_counts_match_validation_channels() {
        assert_eq!(PRESSURE_PAIR_COUNT, 4);
        assert_eq!(PRESSURE_PORT_COUNT, 8);
        assert_eq!(PRESSURE_SENSOR_POCKETS, PRESSURE_PAIR_COUNT);
        assert_eq!(DECAY_REFERENCE_VOLUME_COUNT, 3);
        assert_eq!(HEPA_FLANGE_BOLT_COUNT, 8);
        assert!(HEPA_DUCT_OD - HEPA_DUCT_ID >= 40.0);
        assert_eq!(HEPA_DIFFUSER_SLOT_COUNT, 7);
    }

    #[test]
    fn door_interlock_and_seal_coupon_capacity_is_complete() {
        assert_eq!(INTERLOCK_PIN_COUNT, 4);
        assert_eq!(DOOR_LATCH_BLOCKS_PER_PLATE * 2, 6);
        assert_eq!(COUPON_COUNT, 12);
        assert_eq!(COMPRESSION_STEP_COUNT, 5);
        assert!(COMPRESSION_MIN_MM < COMPRESSION_MAX_MM);
        assert!(SEAL_COUPON_OUTER_D > SEAL_COUPON_INNER_D);
    }

    #[test]
    fn evidence_and_disposition_controls_are_counted() {
        assert_eq!(LOCKOUT_FLAG_COUNT, 6);
        assert_eq!(BARCODE_LAND_COUNT, 6);
        assert_eq!(CERTIFICATE_LAND_COUNT, 3);
        assert_eq!(WITNESS_TOKEN_COUNT, 6);
        assert_eq!(DispositionLane::all().len(), LANE_COUNT);
        assert_eq!(lane_x(DispositionLane::Release), -LANE_PITCH_X);
        assert_eq!(lane_x(DispositionLane::Hold), 0.0);
        assert_eq!(lane_x(DispositionLane::Reject), LANE_PITCH_X);
    }
}
