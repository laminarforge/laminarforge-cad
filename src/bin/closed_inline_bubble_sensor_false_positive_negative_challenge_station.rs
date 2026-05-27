use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed inline bubble-sensor false-positive / false-negative challenge station.
//
// Intent:
// - Present traceable transparent surrogate channel nests for inline optical
//   bubble sensors without making the printed parts a wetted-process design.
// - Hold known air-slug coupons, refractive-index-only media pockets, fouled
//   window / clean-window challenges, pressure and flow witness taps, and
//   disposition lanes so false-positive and false-negative behavior can be
//   challenged mechanically and documented repeatably.
// - Reserve evidence capture, barcode/certificate land, clean/used segregation,
//   and robot/service keepout geometry. Sensor thresholds, media formulation,
//   sterile barrier details, and release criteria remain validation procedures.

const OUTPUTS: [&str; 12] = [
    "output/closed_inline_bubble_sensor_false_positive_negative_challenge_station_base_validation_deck.stl",
    "output/closed_inline_bubble_sensor_false_positive_negative_challenge_station_transparent_surrogate_channel_nests.stl",
    "output/closed_inline_bubble_sensor_false_positive_negative_challenge_station_air_slug_coupon_bank.stl",
    "output/closed_inline_bubble_sensor_false_positive_negative_challenge_station_refractive_index_media_pockets.stl",
    "output/closed_inline_bubble_sensor_false_positive_negative_challenge_station_optical_sensor_mounts.stl",
    "output/closed_inline_bubble_sensor_false_positive_negative_challenge_station_pressure_flow_witness_ports.stl",
    "output/closed_inline_bubble_sensor_false_positive_negative_challenge_station_barcode_certificate_lands.stl",
    "output/closed_inline_bubble_sensor_false_positive_negative_challenge_station_release_hold_reject_lanes.stl",
    "output/closed_inline_bubble_sensor_false_positive_negative_challenge_station_evidence_bridge.stl",
    "output/closed_inline_bubble_sensor_false_positive_negative_challenge_station_clean_used_segregation.stl",
    "output/closed_inline_bubble_sensor_false_positive_negative_challenge_station_robot_service_keepouts.stl",
    "output/closed_inline_bubble_sensor_false_positive_negative_challenge_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "transparent_surrogate_channel_nests",
    "air_slug_coupon_bank",
    "refractive_index_media_pockets",
    "optical_sensor_mounts",
    "pressure_flow_witness_ports",
    "barcode_certificate_lands",
    "release_hold_reject_lanes",
    "evidence_bridge",
    "clean_used_segregation",
    "robot_service_keepouts",
    "contained_validation_deck",
];

const STATION_X: f64 = 1380.0;
const STATION_Y: f64 = 880.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const SOCKET_DEPTH: f64 = 6.0;
const MOUNT_HOLE_D: f64 = 6.8;
const TUBE_BORE_D: f64 = 6.2;

const CHANNEL_POS: (f64, f64) = (-390.0, 185.0);
const CHANNEL_NEST_X: f64 = 500.0;
const CHANNEL_NEST_Y: f64 = 250.0;
const CHANNEL_NEST_Z: f64 = 52.0;
const SURROGATE_CHANNELS: usize = 6;
const CHANNEL_PITCH_Y: f64 = 34.0;
const CHANNEL_WINDOW_X: f64 = 384.0;
const CHANNEL_WINDOW_Y: f64 = 18.0;
const CHANNEL_GROOVE_D: f64 = 5.6;
const CHANNEL_CLAMP_COUNT: usize = SURROGATE_CHANNELS * 2;

const AIR_COUPON_POS: (f64, f64) = (120.0, 190.0);
const AIR_BANK_X: f64 = 390.0;
const AIR_BANK_Y: f64 = 245.0;
const AIR_BANK_Z: f64 = 54.0;
const AIR_COUPON_ROWS: usize = 2;
const AIR_COUPON_COLS: usize = 4;
const AIR_SLUG_COUPONS: usize = AIR_COUPON_ROWS * AIR_COUPON_COLS;
const AIR_COUPON_PITCH_X: f64 = 82.0;
const AIR_COUPON_PITCH_Y: f64 = 82.0;
const AIR_SLUG_LENGTHS_MM: [f64; AIR_SLUG_COUPONS] = [0.0, 2.0, 5.0, 10.0, 20.0, 40.0, 80.0, 120.0];
const MIN_DETECTABLE_SLUG_MM: f64 = 5.0;

const RI_POS: (f64, f64) = (505.0, 190.0);
const RI_BANK_X: f64 = 270.0;
const RI_BANK_Y: f64 = 245.0;
const RI_BANK_Z: f64 = 58.0;
const RI_ROWS: usize = 2;
const RI_COLS: usize = 4;
const RI_MEDIA_POCKETS: usize = RI_ROWS * RI_COLS;
const RI_POCKET_PITCH_X: f64 = 58.0;
const RI_POCKET_PITCH_Y: f64 = 76.0;
const RI_POCKET_D: f64 = 28.0;

const OPTICAL_POS: (f64, f64) = (-300.0, -75.0);
const OPTICAL_X: f64 = 610.0;
const OPTICAL_Y: f64 = 210.0;
const OPTICAL_Z: f64 = 92.0;
const OPTICAL_SENSOR_LANES: usize = SURROGATE_CHANNELS;
const OPTICAL_LANE_PITCH_X: f64 = 82.0;
const SENSOR_FORK_Y: f64 = 118.0;
const SENSOR_BORE_D: f64 = 13.0;
const ADJUSTMENT_SLOT_COUNT: usize = OPTICAL_SENSOR_LANES * 2;

const WITNESS_POS: (f64, f64) = (325.0, -75.0);
const WITNESS_X: f64 = 460.0;
const WITNESS_Y: f64 = 210.0;
const WITNESS_Z: f64 = 56.0;
const WITNESS_LANES: usize = SURROGATE_CHANNELS;
const PRESSURE_TAPS_PER_LANE: usize = 2;
const PRESSURE_TAP_COUNT: usize = WITNESS_LANES * PRESSURE_TAPS_PER_LANE;
const FLOW_WITNESS_COUNT: usize = WITNESS_LANES;
const PRESSURE_TAP_D: f64 = 8.0;
const FLOW_WINDOW_X: f64 = 48.0;

const TRACE_POS: (f64, f64) = (-470.0, -312.0);
const TRACE_X: f64 = 330.0;
const TRACE_Y: f64 = 150.0;
const TRACE_Z: f64 = 14.0;
const BARCODE_LANDS: usize = 8;
const CERTIFICATE_LANDS: usize = 4;
const WITNESS_TOKEN_LANDS: usize = 6;

const LANES_POS: (f64, f64) = (-65.0, -312.0);
const LANES_X: f64 = 420.0;
const LANES_Y: f64 = 155.0;
const LANES_Z: f64 = 24.0;
const LANE_COUNT: usize = 3;
const LANE_PITCH_X: f64 = 130.0;
const LANE_SLOT_COUNT: usize = 5;

const SEGREGATION_POS: (f64, f64) = (360.0, -312.0);
const SEGREGATION_X: f64 = 320.0;
const SEGREGATION_Y: f64 = 155.0;
const SEGREGATION_Z: f64 = 48.0;
const CLEAN_USED_MIN_GAP: f64 = 42.0;
const CLEAN_COUPON_WELLS: usize = 6;
const USED_COUPON_CUPS: usize = 6;

const EVIDENCE_POS: (f64, f64) = (0.0, 365.0);
const EVIDENCE_SPAN_X: f64 = 1180.0;
const EVIDENCE_POST_X: f64 = 28.0;
const EVIDENCE_POST_Y: f64 = 46.0;
const EVIDENCE_UNDERSIDE_Z: f64 = 228.0;
const EVIDENCE_BEAM_Z: f64 = 30.0;
const EVIDENCE_CAMERA_COUNT: usize = 4;
const EVIDENCE_LIGHT_SEGMENTS: usize = 10;

const KEEP_OUT_ZONE_COUNT: usize = 5;
const FRONT_ROBOT_SWEEP_Y: f64 = 150.0;
const REAR_SERVICE_CLEARANCE_Y: f64 = 105.0;
const LEFT_NEST_SERVICE_X: f64 = 170.0;
const RIGHT_MEDIA_SERVICE_X: f64 = 190.0;
const TOP_SENSOR_LIFT_CLEARANCE_Z: f64 = 340.0;

#[derive(Clone, Copy, Debug)]
struct ChallengeLane {
    name: &'static str,
    expected_bubble: bool,
    false_positive_probe: bool,
    slug_length_mm: f64,
    channel_width_mm: f64,
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
        self.center.0 - self.x / 2.0 >= -usable_x
            && self.center.0 + self.x / 2.0 <= usable_x
            && self.center.1 - self.y / 2.0 >= -usable_y
            && self.center.1 + self.y / 2.0 <= usable_y
    }

    fn overlaps_with_clearance(self, other: Rect, clearance: f64) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();
        dx < (self.x + other.x) / 2.0 + clearance && dy < (self.y + other.y) / 2.0 + clearance
    }
}

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

    fn gate_height(self) -> f64 {
        match self {
            DispositionLane::Release => 20.0,
            DispositionLane::Hold => 34.0,
            DispositionLane::Reject => 48.0,
        }
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_validation_deck();
    export(OUTPUTS[0], &base);

    let channels = transparent_surrogate_channel_nests();
    export(OUTPUTS[1], &channels);

    let air_slug_bank = air_slug_coupon_bank();
    export(OUTPUTS[2], &air_slug_bank);

    let ri_pockets = refractive_index_media_pockets();
    export(OUTPUTS[3], &ri_pockets);

    let optical_mounts = optical_sensor_mounts();
    export(OUTPUTS[4], &optical_mounts);

    let witness_ports = pressure_flow_witness_ports();
    export(OUTPUTS[5], &witness_ports);

    let traceability = barcode_certificate_lands();
    export(OUTPUTS[6], &traceability);

    let lanes = release_hold_reject_lanes();
    export(OUTPUTS[7], &lanes);

    let evidence = evidence_bridge();
    export(OUTPUTS[8], &evidence);

    let segregation = clean_used_segregation();
    export(OUTPUTS[9], &segregation);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let assembly = base
        + channels.translate(CHANNEL_POS.0, CHANNEL_POS.1, insert_z(CHANNEL_NEST_Z))
        + air_slug_bank.translate(AIR_COUPON_POS.0, AIR_COUPON_POS.1, insert_z(AIR_BANK_Z))
        + ri_pockets.translate(RI_POS.0, RI_POS.1, insert_z(RI_BANK_Z))
        + optical_mounts.translate(OPTICAL_POS.0, OPTICAL_POS.1, insert_z(OPTICAL_Z))
        + witness_ports.translate(WITNESS_POS.0, WITNESS_POS.1, insert_z(WITNESS_Z))
        + traceability.translate(TRACE_POS.0, TRACE_POS.1, insert_z(TRACE_Z))
        + lanes.translate(LANES_POS.0, LANES_POS.1, insert_z(LANES_Z))
        + evidence
        + segregation.translate(
            SEGREGATION_POS.0,
            SEGREGATION_POS.1,
            insert_z(SEGREGATION_Z),
        )
        + keepouts;
    export(OUTPUTS[11], &assembly);

    let lanes = challenge_lanes();
    let positive_challenges = lanes.iter().filter(|lane| lane.expected_bubble).count();
    let false_positive_challenges = lanes
        .iter()
        .filter(|lane| lane.false_positive_probe)
        .count();

    println!();
    println!("Closed inline bubble sensor false-positive/false-negative challenge station:");
    println!(
        "  Footprint:                 {STATION_X:.0}mm x {STATION_Y:.0}mm contained validation deck"
    );
    println!(
        "  Challenge lanes:           {SURROGATE_CHANNELS} transparent surrogate channels, {positive_challenges} known-bubble lanes, {false_positive_challenges} false-positive probes"
    );
    println!(
        "  Air-slug coupon bank:      {AIR_SLUG_COUPONS} coupons spanning {:.0}-{:.0}mm, minimum detection intent {MIN_DETECTABLE_SLUG_MM:.0}mm",
        AIR_SLUG_LENGTHS_MM[0],
        AIR_SLUG_LENGTHS_MM[AIR_SLUG_LENGTHS_MM.len() - 1]
    );
    println!(
        "  Media / optics:            {RI_MEDIA_POCKETS} refractive-index pockets, {OPTICAL_SENSOR_LANES} optical sensor mounts, {ADJUSTMENT_SLOT_COUNT} adjustment slots"
    );
    println!(
        "  Witnessing:                {PRESSURE_TAP_COUNT} pressure taps, {FLOW_WITNESS_COUNT} flow witness windows, {EVIDENCE_CAMERA_COUNT} evidence camera brackets"
    );
    println!(
        "  Disposition and custody:   release/hold/reject lanes, {BARCODE_LANDS} barcode lands, {CERTIFICATE_LANDS} certificate lands, clean/used gap {CLEAN_USED_MIN_GAP:.0}mm"
    );
    println!(
        "  Required feature groups:   {}, robot/service keepout groups: {KEEP_OUT_ZONE_COUNT}",
        REQUIRED_FEATURES.len()
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn insert_z(height: f64) -> f64 {
    BASE_Z / 2.0 + height / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn challenge_lanes() -> [ChallengeLane; SURROGATE_CHANNELS] {
    [
        ChallengeLane {
            name: "negative_clear_media",
            expected_bubble: false,
            false_positive_probe: false,
            slug_length_mm: 0.0,
            channel_width_mm: 4.8,
        },
        ChallengeLane {
            name: "microbubble_threshold",
            expected_bubble: true,
            false_positive_probe: false,
            slug_length_mm: 5.0,
            channel_width_mm: 4.8,
        },
        ChallengeLane {
            name: "large_air_slug",
            expected_bubble: true,
            false_positive_probe: false,
            slug_length_mm: 40.0,
            channel_width_mm: CHANNEL_GROOVE_D,
        },
        ChallengeLane {
            name: "refractive_index_step",
            expected_bubble: false,
            false_positive_probe: true,
            slug_length_mm: 0.0,
            channel_width_mm: 4.8,
        },
        ChallengeLane {
            name: "window_fouling_probe",
            expected_bubble: false,
            false_positive_probe: true,
            slug_length_mm: 0.0,
            channel_width_mm: 4.8,
        },
        ChallengeLane {
            name: "bypass_false_negative",
            expected_bubble: true,
            false_positive_probe: false,
            slug_length_mm: 120.0,
            channel_width_mm: 6.4,
        },
    ]
}

fn insert_specs() -> [Rect; 8] {
    [
        Rect {
            name: "transparent_surrogate_channel_nests",
            center: CHANNEL_POS,
            x: CHANNEL_NEST_X,
            y: CHANNEL_NEST_Y,
        },
        Rect {
            name: "air_slug_coupon_bank",
            center: AIR_COUPON_POS,
            x: AIR_BANK_X,
            y: AIR_BANK_Y,
        },
        Rect {
            name: "refractive_index_media_pockets",
            center: RI_POS,
            x: RI_BANK_X,
            y: RI_BANK_Y,
        },
        Rect {
            name: "optical_sensor_mounts",
            center: OPTICAL_POS,
            x: OPTICAL_X,
            y: OPTICAL_Y,
        },
        Rect {
            name: "pressure_flow_witness_ports",
            center: WITNESS_POS,
            x: WITNESS_X,
            y: WITNESS_Y,
        },
        Rect {
            name: "barcode_certificate_lands",
            center: TRACE_POS,
            x: TRACE_X,
            y: TRACE_Y,
        },
        Rect {
            name: "release_hold_reject_lanes",
            center: LANES_POS,
            x: LANES_X,
            y: LANES_Y,
        },
        Rect {
            name: "clean_used_segregation",
            center: SEGREGATION_POS,
            x: SEGREGATION_X,
            y: SEGREGATION_Y,
        },
    ]
}

fn assert_design_constraints() {
    let inserts = insert_specs();
    for insert in inserts {
        assert!(
            insert.fits_inside_station(),
            "{} exceeds station envelope",
            insert.name
        );
    }
    for i in 0..inserts.len() {
        for j in (i + 1)..inserts.len() {
            assert!(
                !inserts[i].overlaps_with_clearance(inserts[j], 12.0),
                "{} overlaps {}",
                inserts[i].name,
                inserts[j].name
            );
        }
    }

    let lanes = challenge_lanes();
    assert_eq!(AIR_SLUG_COUPONS, AIR_COUPON_ROWS * AIR_COUPON_COLS);
    assert_eq!(RI_MEDIA_POCKETS, RI_ROWS * RI_COLS);
    assert_eq!(OPTICAL_SENSOR_LANES, SURROGATE_CHANNELS);
    assert_eq!(WITNESS_LANES, SURROGATE_CHANNELS);
    assert_eq!(PRESSURE_TAP_COUNT, WITNESS_LANES * PRESSURE_TAPS_PER_LANE);
    assert_eq!(CHANNEL_CLAMP_COUNT, SURROGATE_CHANNELS * 2);
    assert_eq!(DispositionLane::all().len(), LANE_COUNT);
    assert_eq!(CLEAN_COUPON_WELLS, USED_COUPON_CUPS);
    assert!(CLEAN_USED_MIN_GAP >= 40.0);
    assert!(lanes
        .iter()
        .any(|lane| lane.expected_bubble && lane.slug_length_mm >= MIN_DETECTABLE_SLUG_MM));
    assert!(lanes.iter().any(|lane| lane.false_positive_probe));
}

fn base_validation_deck() -> Part {
    let deck = centered_cube(
        "closed_inline_bubble_sensor_fpfn_station_base_floor",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let washdown_recess = centered_cube(
        "closed_inline_bubble_sensor_fpfn_station_washdown_recess",
        STATION_X - 112.0,
        STATION_Y - 112.0,
        7.0,
    )
    .translate(0.0, -6.0, BASE_Z / 2.0 - 3.5);
    let wet_path_sump = centered_cube(
        "closed_inline_bubble_sensor_fpfn_station_wet_path_sump",
        1090.0,
        256.0,
        8.0,
    )
    .translate(40.0, 52.0, BASE_Z / 2.0 - 4.0);
    let disposition_sump = centered_cube(
        "closed_inline_bubble_sensor_fpfn_station_disposition_sump",
        STATION_X - 190.0,
        105.0,
        8.0,
    )
    .translate(0.0, -312.0, BASE_Z / 2.0 - 4.0);
    let front_drain = centered_cylinder(
        "closed_inline_bubble_sensor_fpfn_station_front_validation_tray_drain",
        9.0 / 2.0,
        48.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 72.0, -STATION_Y / 2.0 - 2.0, -1.0);

    deck - washdown_recess
        - wet_path_sump
        - disposition_sump
        - front_drain
        - insert_sockets()
        - mounting_slots()
        + perimeter_rims()
        + zone_dividers()
        + rear_service_bulkheads()
        + flow_direction_ribs()
        + robot_fiducials()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("closed_inline_bubble_sensor_fpfn_station_insert_sockets");
    for insert in insert_specs() {
        sockets = sockets
            + centered_cube(
                format!(
                    "closed_inline_bubble_sensor_fpfn_station_{}_socket",
                    insert.name
                ),
                insert.x + 8.0,
                insert.y + 8.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(
                insert.center.0,
                insert.center.1,
                BASE_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.2,
            );
    }
    sockets
}

fn mounting_slots() -> Part {
    let mut slots = Part::empty("closed_inline_bubble_sensor_fpfn_station_mounting_slots");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 54.0), -(STATION_Y / 2.0 - 50.0)),
        (STATION_X / 2.0 - 54.0, -(STATION_Y / 2.0 - 50.0)),
        (-(STATION_X / 2.0 - 54.0), STATION_Y / 2.0 - 50.0),
        (STATION_X / 2.0 - 54.0, STATION_Y / 2.0 - 50.0),
        (0.0, STATION_Y / 2.0 - 50.0),
        (0.0, -(STATION_Y / 2.0 - 50.0)),
        (-(STATION_X / 2.0 - 54.0), 0.0),
        (STATION_X / 2.0 - 54.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        slots = slots
            + centered_cylinder(
                format!("closed_inline_bubble_sensor_fpfn_station_m6_mount_clearance_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 4.0,
                24,
            )
            .translate(*x, *y, 0.0)
            + centered_cube(
                format!("closed_inline_bubble_sensor_fpfn_station_mount_slot_relief_{i}"),
                28.0,
                7.0,
                BASE_Z + 4.0,
            )
            .translate(*x, *y, 0.0);
    }
    slots
}

fn perimeter_rims() -> Part {
    let left = centered_cube(
        "closed_inline_bubble_sensor_fpfn_station_left_containment_rim",
        RIM_W,
        STATION_Y - 54.0,
        RIM_Z,
    )
    .translate(
        -(STATION_X / 2.0 - RIM_W / 2.0),
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(
        "closed_inline_bubble_sensor_fpfn_station_right_containment_rim",
        RIM_W,
        STATION_Y - 54.0,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "closed_inline_bubble_sensor_fpfn_station_rear_containment_rim",
        STATION_X - 36.0,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let front_low_lip = centered_cube(
        "closed_inline_bubble_sensor_fpfn_station_front_low_service_lip",
        STATION_X - 180.0,
        14.0,
        22.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 24.0, BASE_Z / 2.0 + 11.0);

    left + right + rear + front_low_lip
}

fn zone_dividers() -> Part {
    let top_to_sensor_row = centered_cube(
        "closed_inline_bubble_sensor_fpfn_station_challenge_to_sensor_row_divider",
        STATION_X - 158.0,
        10.0,
        28.0,
    )
    .translate(0.0, 48.0, BASE_Z / 2.0 + 14.0);
    let sensor_to_disposition_row = centered_cube(
        "closed_inline_bubble_sensor_fpfn_station_sensor_to_disposition_row_divider",
        STATION_X - 170.0,
        10.0,
        26.0,
    )
    .translate(0.0, -205.0, BASE_Z / 2.0 + 13.0);
    let channel_to_coupon_barrier = centered_cube(
        "closed_inline_bubble_sensor_fpfn_station_channel_to_air_coupon_barrier",
        10.0,
        260.0,
        28.0,
    )
    .translate(-126.0, 188.0, BASE_Z / 2.0 + 14.0);
    let coupon_to_media_barrier = centered_cube(
        "closed_inline_bubble_sensor_fpfn_station_air_coupon_to_ri_media_barrier",
        10.0,
        260.0,
        28.0,
    )
    .translate(330.0, 188.0, BASE_Z / 2.0 + 14.0);
    let optical_to_witness_barrier = centered_cube(
        "closed_inline_bubble_sensor_fpfn_station_optical_to_pressure_flow_barrier",
        10.0,
        222.0,
        26.0,
    )
    .translate(40.0, -75.0, BASE_Z / 2.0 + 13.0);

    top_to_sensor_row
        + sensor_to_disposition_row
        + channel_to_coupon_barrier
        + coupon_to_media_barrier
        + optical_to_witness_barrier
}

fn rear_service_bulkheads() -> Part {
    let mut bulkheads =
        Part::empty("closed_inline_bubble_sensor_fpfn_station_rear_service_bulkheads");
    for (i, x) in [-525.0, -350.0, -175.0, 0.0, 175.0, 350.0, 525.0]
        .iter()
        .enumerate()
    {
        let block = centered_cube(
            format!("closed_inline_bubble_sensor_fpfn_station_rear_tube_bulkhead_{i}"),
            62.0,
            22.0,
            30.0,
        )
        .translate(*x, STATION_Y / 2.0 - 50.0, BASE_Z / 2.0 + 15.0);
        let bore = centered_cylinder(
            format!("closed_inline_bubble_sensor_fpfn_station_rear_tube_bore_{i}"),
            TUBE_BORE_D / 2.0,
            30.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, STATION_Y / 2.0 - 50.0, BASE_Z / 2.0 + 15.0);
        bulkheads = bulkheads + (block - bore);
    }
    bulkheads
}

fn flow_direction_ribs() -> Part {
    let top_path = centered_cube(
        "closed_inline_bubble_sensor_fpfn_station_challenge_sample_flow_rib",
        1040.0,
        6.0,
        7.0,
    )
    .translate(20.0, 54.0, BASE_Z / 2.0 + 3.5);
    let optical_path = centered_cube(
        "closed_inline_bubble_sensor_fpfn_station_optical_to_witness_flow_rib",
        820.0,
        6.0,
        7.0,
    )
    .translate(0.0, -190.0, BASE_Z / 2.0 + 3.5);
    let evidence_path = centered_cube(
        "closed_inline_bubble_sensor_fpfn_station_evidence_to_disposition_rib",
        780.0,
        6.0,
        7.0,
    )
    .translate(40.0, -238.0, BASE_Z / 2.0 + 3.5);

    top_path + optical_path + evidence_path
}

fn robot_fiducials() -> Part {
    let mut fiducials = Part::empty("closed_inline_bubble_sensor_fpfn_station_robot_fiducials");
    for (i, (x, y)) in [(-625.0, 388.0), (625.0, 388.0), (-625.0, -388.0)]
        .iter()
        .enumerate()
    {
        fiducials = fiducials
            + fiducial_disc(&format!(
                "closed_inline_bubble_sensor_fpfn_station_robot_fiducial_{i}"
            ))
            .translate(*x, *y, BASE_Z / 2.0 + 3.0);
    }
    fiducials
}

fn transparent_surrogate_channel_nests() -> Part {
    let body = centered_cube(
        "closed_inline_bubble_sensor_fpfn_transparent_surrogate_channel_nest_body",
        CHANNEL_NEST_X,
        CHANNEL_NEST_Y,
        CHANNEL_NEST_Z,
    );
    let cover_land = centered_cube(
        "closed_inline_bubble_sensor_fpfn_surrogate_channel_clear_cover_land",
        CHANNEL_NEST_X - 24.0,
        CHANNEL_NEST_Y - 24.0,
        8.0,
    )
    .translate(0.0, 0.0, CHANNEL_NEST_Z / 2.0 + 4.0);

    body - surrogate_channel_cuts() + cover_land + surrogate_channel_lanes()
}

fn surrogate_channel_cuts() -> Part {
    let mut cuts = Part::empty("closed_inline_bubble_sensor_fpfn_surrogate_channel_cuts");
    let lanes = challenge_lanes();
    for (i, lane) in lanes.iter().enumerate() {
        let y = centered_index(i, SURROGATE_CHANNELS, CHANNEL_PITCH_Y);
        let channel_bore = centered_cylinder(
            format!(
                "closed_inline_bubble_sensor_fpfn_{}_transparent_channel_bore",
                lane.name
            ),
            lane.channel_width_mm / 2.0,
            CHANNEL_WINDOW_X + 36.0,
            24,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(0.0, y, 4.0);
        let window = centered_cube(
            format!(
                "closed_inline_bubble_sensor_fpfn_{}_top_optical_window_cut",
                lane.name
            ),
            CHANNEL_WINDOW_X,
            CHANNEL_WINDOW_Y,
            CHANNEL_NEST_Z + 12.0,
        )
        .translate(0.0, y, CHANNEL_NEST_Z / 2.0 - 6.0);
        let coupon_pocket = centered_cube(
            format!(
                "closed_inline_bubble_sensor_fpfn_{}_removable_surrogate_coupon_pocket",
                lane.name
            ),
            CHANNEL_WINDOW_X + 58.0,
            26.0,
            9.0,
        )
        .translate(0.0, y, CHANNEL_NEST_Z / 2.0 - 4.5);

        cuts = cuts + channel_bore + window + coupon_pocket;
    }
    cuts
}

fn surrogate_channel_lanes() -> Part {
    let mut lanes_part =
        Part::empty("closed_inline_bubble_sensor_fpfn_surrogate_channel_lane_features");
    let lanes = challenge_lanes();
    for (i, lane) in lanes.iter().enumerate() {
        let y = centered_index(i, SURROGATE_CHANNELS, CHANNEL_PITCH_Y);
        let gasket_left = centered_cube(
            format!(
                "closed_inline_bubble_sensor_fpfn_{}_left_gasket_land",
                lane.name
            ),
            CHANNEL_WINDOW_X + 44.0,
            4.0,
            5.0,
        )
        .translate(0.0, y - 15.0, CHANNEL_NEST_Z / 2.0 + 2.5);
        let gasket_right = centered_cube(
            format!(
                "closed_inline_bubble_sensor_fpfn_{}_right_gasket_land",
                lane.name
            ),
            CHANNEL_WINDOW_X + 44.0,
            4.0,
            5.0,
        )
        .translate(0.0, y + 15.0, CHANNEL_NEST_Z / 2.0 + 2.5);
        let inlet_stop = centered_cube(
            format!(
                "closed_inline_bubble_sensor_fpfn_{}_inlet_stop_block",
                lane.name
            ),
            18.0,
            30.0,
            16.0,
        )
        .translate(-(CHANNEL_WINDOW_X / 2.0 + 38.0), y, 4.0);
        let outlet_stop = centered_cube(
            format!(
                "closed_inline_bubble_sensor_fpfn_{}_outlet_stop_block",
                lane.name
            ),
            18.0,
            30.0,
            16.0,
        )
        .translate(CHANNEL_WINDOW_X / 2.0 + 38.0, y, 4.0);
        let challenge_marker_len = if lane.expected_bubble {
            24.0 + lane.slug_length_mm * 0.45
        } else if lane.false_positive_probe {
            64.0
        } else {
            28.0
        };
        let challenge_marker = centered_cube(
            format!(
                "closed_inline_bubble_sensor_fpfn_{}_challenge_length_marker",
                lane.name
            ),
            challenge_marker_len.min(110.0),
            7.0,
            8.0,
        )
        .translate(-118.0, y, CHANNEL_NEST_Z / 2.0 + 9.0);

        lanes_part =
            lanes_part + gasket_left + gasket_right + inlet_stop + outlet_stop + challenge_marker;
    }
    lanes_part + channel_hold_down_clamps()
}

fn channel_hold_down_clamps() -> Part {
    let mut clamps = Part::empty("closed_inline_bubble_sensor_fpfn_channel_hold_down_clamps");
    for i in 0..SURROGATE_CHANNELS {
        let y = centered_index(i, SURROGATE_CHANNELS, CHANNEL_PITCH_Y);
        for (side, x) in [("inlet", -212.0), ("outlet", 212.0)] {
            let ear = centered_cube(
                format!("closed_inline_bubble_sensor_fpfn_channel_{i}_{side}_clamp_ear"),
                30.0,
                18.0,
                8.0,
            )
            .translate(x, y, CHANNEL_NEST_Z / 2.0 + 10.0);
            let screw = centered_cylinder(
                format!("closed_inline_bubble_sensor_fpfn_channel_{i}_{side}_clamp_screw"),
                3.4 / 2.0,
                12.0,
                20,
            )
            .translate(x, y, CHANNEL_NEST_Z / 2.0 + 10.0);
            clamps = clamps + (ear - screw);
        }
    }
    clamps
}

fn air_slug_coupon_bank() -> Part {
    let body = centered_cube(
        "closed_inline_bubble_sensor_fpfn_air_slug_coupon_bank_body",
        AIR_BANK_X,
        AIR_BANK_Y,
        AIR_BANK_Z,
    );
    let rear_gauge_rail = centered_cube(
        "closed_inline_bubble_sensor_fpfn_air_slug_coupon_rear_gauge_rail",
        AIR_BANK_X - 28.0,
        12.0,
        22.0,
    )
    .translate(0.0, AIR_BANK_Y / 2.0 - 18.0, AIR_BANK_Z / 2.0 + 11.0);
    let front_seal_bar = centered_cube(
        "closed_inline_bubble_sensor_fpfn_air_slug_coupon_front_seal_bar",
        AIR_BANK_X - 44.0,
        12.0,
        18.0,
    )
    .translate(0.0, -AIR_BANK_Y / 2.0 + 18.0, AIR_BANK_Z / 2.0 + 9.0);

    body - air_slug_coupon_cuts() + rear_gauge_rail + front_seal_bar + air_slug_gauge_marks()
}

fn air_slug_coupon_cuts() -> Part {
    let mut cuts = Part::empty("closed_inline_bubble_sensor_fpfn_air_slug_coupon_cuts");
    for row in 0..AIR_COUPON_ROWS {
        for col in 0..AIR_COUPON_COLS {
            let i = row * AIR_COUPON_COLS + col;
            let x = centered_index(col, AIR_COUPON_COLS, AIR_COUPON_PITCH_X);
            let y = centered_index(row, AIR_COUPON_ROWS, AIR_COUPON_PITCH_Y);
            let coupon_well = centered_cylinder(
                format!("closed_inline_bubble_sensor_fpfn_air_slug_coupon_{i}_vertical_well"),
                20.0 / 2.0,
                AIR_BANK_Z + 6.0,
                28,
            )
            .translate(x - 22.0, y, 0.0);
            let channel_window = centered_cube(
                format!("closed_inline_bubble_sensor_fpfn_air_slug_coupon_{i}_viewing_slot"),
                50.0,
                18.0,
                10.0,
            )
            .translate(x + 18.0, y, AIR_BANK_Z / 2.0 - 5.0);
            let luer_relief = centered_cylinder(
                format!("closed_inline_bubble_sensor_fpfn_air_slug_coupon_{i}_luer_relief"),
                8.4 / 2.0,
                32.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x - 22.0, y + 18.0, 5.0);
            cuts = cuts + coupon_well + channel_window + luer_relief;
        }
    }
    cuts
}

fn air_slug_gauge_marks() -> Part {
    let mut marks = Part::empty("closed_inline_bubble_sensor_fpfn_air_slug_coupon_gauge_marks");
    for (i, length_mm) in AIR_SLUG_LENGTHS_MM.iter().enumerate() {
        let row = i / AIR_COUPON_COLS;
        let col = i % AIR_COUPON_COLS;
        let x = centered_index(col, AIR_COUPON_COLS, AIR_COUPON_PITCH_X) + 18.0;
        let y = centered_index(row, AIR_COUPON_ROWS, AIR_COUPON_PITCH_Y) - 24.0;
        let gauge_len = (14.0 + *length_mm * 0.55).min(86.0);
        let gauge = centered_cube(
            format!(
                "closed_inline_bubble_sensor_fpfn_air_slug_coupon_{i}_{length_mm:.0}mm_gauge_land"
            ),
            gauge_len,
            5.0,
            7.0,
        )
        .translate(x, y, AIR_BANK_Z / 2.0 + 3.5);
        marks = marks + gauge;
    }
    marks
}

fn refractive_index_media_pockets() -> Part {
    let body = centered_cube(
        "closed_inline_bubble_sensor_fpfn_refractive_index_media_pocket_bank_body",
        RI_BANK_X,
        RI_BANK_Y,
        RI_BANK_Z,
    );
    let cold_edge_stop = centered_cube(
        "closed_inline_bubble_sensor_fpfn_ri_media_rear_cold_edge_stop",
        RI_BANK_X - 34.0,
        12.0,
        24.0,
    )
    .translate(0.0, RI_BANK_Y / 2.0 - 18.0, RI_BANK_Z / 2.0 + 12.0);
    let certificate_clip = centered_cube(
        "closed_inline_bubble_sensor_fpfn_ri_media_certificate_clip_rail",
        RI_BANK_X - 54.0,
        10.0,
        16.0,
    )
    .translate(0.0, -RI_BANK_Y / 2.0 + 18.0, RI_BANK_Z / 2.0 + 8.0);

    body - refractive_index_pocket_cuts()
        + refractive_index_pocket_collars()
        + refractive_index_ladder_tabs()
        + cold_edge_stop
        + certificate_clip
}

fn refractive_index_pocket_cuts() -> Part {
    let mut cuts =
        Part::empty("closed_inline_bubble_sensor_fpfn_refractive_index_media_pocket_cuts");
    for row in 0..RI_ROWS {
        for col in 0..RI_COLS {
            let i = row * RI_COLS + col;
            let x = centered_index(col, RI_COLS, RI_POCKET_PITCH_X);
            let y = centered_index(row, RI_ROWS, RI_POCKET_PITCH_Y);
            cuts = cuts
                + centered_cylinder(
                    format!("closed_inline_bubble_sensor_fpfn_ri_media_pocket_{i}_well"),
                    RI_POCKET_D / 2.0,
                    RI_BANK_Z + 6.0,
                    32,
                )
                .translate(x, y, 0.0)
                + centered_cube(
                    format!(
                        "closed_inline_bubble_sensor_fpfn_ri_media_pocket_{i}_meniscus_view_window"
                    ),
                    38.0,
                    10.0,
                    12.0,
                )
                .translate(x, y - 22.0, RI_BANK_Z / 2.0 - 6.0);
        }
    }
    cuts
}

fn refractive_index_pocket_collars() -> Part {
    let mut collars =
        Part::empty("closed_inline_bubble_sensor_fpfn_refractive_index_media_pocket_collars");
    for row in 0..RI_ROWS {
        for col in 0..RI_COLS {
            let i = row * RI_COLS + col;
            let x = centered_index(col, RI_COLS, RI_POCKET_PITCH_X);
            let y = centered_index(row, RI_ROWS, RI_POCKET_PITCH_Y);
            let collar = centered_cylinder(
                format!("closed_inline_bubble_sensor_fpfn_ri_media_pocket_{i}_collar_outer"),
                21.0,
                6.0,
                32,
            ) - centered_cylinder(
                format!("closed_inline_bubble_sensor_fpfn_ri_media_pocket_{i}_collar_inner"),
                RI_POCKET_D / 2.0,
                8.0,
                32,
            );
            collars = collars + collar.translate(x, y, RI_BANK_Z / 2.0 + 3.0);
        }
    }
    collars
}

fn refractive_index_ladder_tabs() -> Part {
    let mut tabs = Part::empty("closed_inline_bubble_sensor_fpfn_refractive_index_ladder_tabs");
    for i in 0..RI_MEDIA_POCKETS {
        let x = centered_index(i, RI_MEDIA_POCKETS, 27.0);
        let tab = centered_cube(
            format!("closed_inline_bubble_sensor_fpfn_ri_index_ladder_step_{i}"),
            18.0 + i as f64 * 2.2,
            8.0,
            5.0,
        )
        .translate(x, -RI_BANK_Y / 2.0 + 44.0, RI_BANK_Z / 2.0 + 2.5);
        tabs = tabs + tab;
    }
    tabs
}

fn optical_sensor_mounts() -> Part {
    let rail = centered_cube(
        "closed_inline_bubble_sensor_fpfn_optical_sensor_mount_base_rail",
        OPTICAL_X,
        OPTICAL_Y,
        20.0,
    )
    .translate(0.0, 0.0, -OPTICAL_Z / 2.0 + 10.0);
    let rear_backbone = centered_cube(
        "closed_inline_bubble_sensor_fpfn_optical_sensor_mount_rear_backbone",
        OPTICAL_X - 36.0,
        16.0,
        54.0,
    )
    .translate(0.0, OPTICAL_Y / 2.0 - 18.0, -6.0);
    let front_backbone = centered_cube(
        "closed_inline_bubble_sensor_fpfn_optical_sensor_mount_front_backbone",
        OPTICAL_X - 36.0,
        16.0,
        54.0,
    )
    .translate(0.0, -OPTICAL_Y / 2.0 + 18.0, -6.0);

    rail + rear_backbone + front_backbone + optical_sensor_forks() - optical_mount_cuts()
}

fn optical_sensor_forks() -> Part {
    let mut forks = Part::empty("closed_inline_bubble_sensor_fpfn_optical_sensor_forks");
    for i in 0..OPTICAL_SENSOR_LANES {
        let x = centered_index(i, OPTICAL_SENSOR_LANES, OPTICAL_LANE_PITCH_X);
        let emitter = centered_cube(
            format!("closed_inline_bubble_sensor_fpfn_lane_{i}_emitter_mount_block"),
            30.0,
            42.0,
            72.0,
        )
        .translate(x, -SENSOR_FORK_Y / 2.0, -2.0);
        let receiver = centered_cube(
            format!("closed_inline_bubble_sensor_fpfn_lane_{i}_receiver_mount_block"),
            30.0,
            42.0,
            72.0,
        )
        .translate(x, SENSOR_FORK_Y / 2.0, -2.0);
        let bridge = centered_cube(
            format!("closed_inline_bubble_sensor_fpfn_lane_{i}_fork_top_bridge"),
            34.0,
            SENSOR_FORK_Y + 10.0,
            10.0,
        )
        .translate(x, 0.0, OPTICAL_Z / 2.0 - 5.0);
        let datum_pin = centered_cylinder(
            format!("closed_inline_bubble_sensor_fpfn_lane_{i}_optical_datum_pin"),
            4.0 / 2.0,
            16.0,
            20,
        )
        .translate(x, 0.0, OPTICAL_Z / 2.0 + 8.0);
        forks = forks + emitter + receiver + bridge + datum_pin;
    }
    forks
}

fn optical_mount_cuts() -> Part {
    let mut cuts = Part::empty("closed_inline_bubble_sensor_fpfn_optical_sensor_mount_cuts");
    for i in 0..OPTICAL_SENSOR_LANES {
        let x = centered_index(i, OPTICAL_SENSOR_LANES, OPTICAL_LANE_PITCH_X);
        let beam_tunnel = centered_cube(
            format!("closed_inline_bubble_sensor_fpfn_lane_{i}_clear_beam_tunnel"),
            22.0,
            SENSOR_FORK_Y + 18.0,
            20.0,
        )
        .translate(x, 0.0, 2.0);
        let emitter_bore = centered_cylinder(
            format!("closed_inline_bubble_sensor_fpfn_lane_{i}_emitter_bore"),
            SENSOR_BORE_D / 2.0,
            58.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -SENSOR_FORK_Y / 2.0, 4.0);
        let receiver_bore = centered_cylinder(
            format!("closed_inline_bubble_sensor_fpfn_lane_{i}_receiver_bore"),
            SENSOR_BORE_D / 2.0,
            58.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, SENSOR_FORK_Y / 2.0, 4.0);
        let adjust_slot_a = centered_cube(
            format!("closed_inline_bubble_sensor_fpfn_lane_{i}_emitter_adjustment_slot"),
            44.0,
            8.0,
            24.0,
        )
        .translate(x, -SENSOR_FORK_Y / 2.0, -18.0);
        let adjust_slot_b = centered_cube(
            format!("closed_inline_bubble_sensor_fpfn_lane_{i}_receiver_adjustment_slot"),
            44.0,
            8.0,
            24.0,
        )
        .translate(x, SENSOR_FORK_Y / 2.0, -18.0);
        cuts = cuts + beam_tunnel + emitter_bore + receiver_bore + adjust_slot_a + adjust_slot_b;
    }
    cuts
}

fn pressure_flow_witness_ports() -> Part {
    let body = centered_cube(
        "closed_inline_bubble_sensor_fpfn_pressure_flow_witness_panel_body",
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    );
    let rear_sensor_rail = centered_cube(
        "closed_inline_bubble_sensor_fpfn_pressure_flow_rear_sensor_rail",
        WITNESS_X - 32.0,
        14.0,
        26.0,
    )
    .translate(0.0, WITNESS_Y / 2.0 - 18.0, WITNESS_Z / 2.0 + 13.0);
    let front_flow_rail = centered_cube(
        "closed_inline_bubble_sensor_fpfn_pressure_flow_front_flow_rail",
        WITNESS_X - 32.0,
        14.0,
        24.0,
    )
    .translate(0.0, -WITNESS_Y / 2.0 + 18.0, WITNESS_Z / 2.0 + 12.0);

    body - pressure_flow_panel_cuts()
        + pressure_tap_bosses()
        + flow_witness_targets()
        + rear_sensor_rail
        + front_flow_rail
}

fn pressure_flow_panel_cuts() -> Part {
    let mut cuts = Part::empty("closed_inline_bubble_sensor_fpfn_pressure_flow_panel_cuts");
    for i in 0..WITNESS_LANES {
        let x = centered_index(i, WITNESS_LANES, 66.0);
        let flow_window = centered_cube(
            format!("closed_inline_bubble_sensor_fpfn_lane_{i}_flow_witness_window_cut"),
            FLOW_WINDOW_X,
            30.0,
            WITNESS_Z + 8.0,
        )
        .translate(x, -32.0, 0.0);
        let flow_bore = centered_cylinder(
            format!("closed_inline_bubble_sensor_fpfn_lane_{i}_flow_tube_bore"),
            TUBE_BORE_D / 2.0,
            92.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -34.0, 0.0);
        cuts = cuts + flow_window + flow_bore;
    }
    for lane in 0..WITNESS_LANES {
        let x = centered_index(lane, WITNESS_LANES, 66.0);
        for (port, y) in [32.0, 70.0].iter().enumerate() {
            cuts = cuts
                + centered_cylinder(
                    format!(
                        "closed_inline_bubble_sensor_fpfn_lane_{lane}_pressure_tap_{port}_bore"
                    ),
                    PRESSURE_TAP_D / 2.0,
                    WITNESS_Z + 18.0,
                    24,
                )
                .translate(x, *y, 0.0);
        }
    }
    cuts
}

fn pressure_tap_bosses() -> Part {
    let mut bosses = Part::empty("closed_inline_bubble_sensor_fpfn_pressure_tap_bosses");
    for lane in 0..WITNESS_LANES {
        let x = centered_index(lane, WITNESS_LANES, 66.0);
        for (port, y) in [32.0, 70.0].iter().enumerate() {
            let boss = centered_cylinder(
                format!("closed_inline_bubble_sensor_fpfn_lane_{lane}_pressure_tap_{port}_boss"),
                16.0 / 2.0,
                12.0,
                28,
            ) - centered_cylinder(
                format!(
                    "closed_inline_bubble_sensor_fpfn_lane_{lane}_pressure_tap_{port}_boss_inner"
                ),
                PRESSURE_TAP_D / 2.0,
                14.0,
                24,
            );
            bosses = bosses + boss.translate(x, *y, WITNESS_Z / 2.0 + 6.0);
        }
    }
    bosses
}

fn flow_witness_targets() -> Part {
    let mut targets = Part::empty("closed_inline_bubble_sensor_fpfn_flow_witness_targets");
    for i in 0..FLOW_WITNESS_COUNT {
        let x = centered_index(i, FLOW_WITNESS_COUNT, 66.0);
        let target = centered_cube(
            format!("closed_inline_bubble_sensor_fpfn_lane_{i}_flow_witness_clear_target_land"),
            36.0,
            8.0,
            6.0,
        )
        .translate(x, -70.0, WITNESS_Z / 2.0 + 3.0);
        let rotor_disc = centered_cylinder(
            format!("closed_inline_bubble_sensor_fpfn_lane_{i}_flow_witness_rotor_surrogate"),
            18.0 / 2.0,
            5.0,
            28,
        )
        .translate(x, -32.0, WITNESS_Z / 2.0 + 2.5);
        targets = targets + target + rotor_disc;
    }
    targets
}

fn barcode_certificate_lands() -> Part {
    let plate = centered_cube(
        "closed_inline_bubble_sensor_fpfn_barcode_certificate_land_plate",
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    );
    plate + barcode_lands() + certificate_lands() + witness_token_lands()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("closed_inline_bubble_sensor_fpfn_barcode_lands");
    for i in 0..BARCODE_LANDS {
        let row = i / 4;
        let col = i % 4;
        let x = centered_index(col, 4, 72.0);
        let y = 34.0 - row as f64 * 36.0;
        let land = centered_cube(
            format!("closed_inline_bubble_sensor_fpfn_barcode_land_{i}"),
            56.0,
            18.0,
            3.0,
        )
        .translate(x, y, TRACE_Z / 2.0 + 1.5);
        lands = lands + land;
    }
    lands
}

fn certificate_lands() -> Part {
    let mut lands = Part::empty("closed_inline_bubble_sensor_fpfn_certificate_lands");
    for i in 0..CERTIFICATE_LANDS {
        let x = centered_index(i, CERTIFICATE_LANDS, 72.0);
        let land = centered_cube(
            format!("closed_inline_bubble_sensor_fpfn_certificate_clip_land_{i}"),
            60.0,
            28.0,
            3.0,
        )
        .translate(x, -50.0, TRACE_Z / 2.0 + 1.5);
        let clip = centered_cube(
            format!("closed_inline_bubble_sensor_fpfn_certificate_clip_rail_{i}"),
            60.0,
            4.0,
            8.0,
        )
        .translate(x, -67.0, TRACE_Z / 2.0 + 4.0);
        lands = lands + land + clip;
    }
    lands
}

fn witness_token_lands() -> Part {
    let mut tokens = Part::empty("closed_inline_bubble_sensor_fpfn_witness_token_lands");
    for i in 0..WITNESS_TOKEN_LANDS {
        let x = centered_index(i, WITNESS_TOKEN_LANDS, 42.0);
        tokens = tokens
            + centered_cylinder(
                format!("closed_inline_bubble_sensor_fpfn_witness_token_land_{i}"),
                12.0,
                3.0,
                28,
            )
            .translate(x, 66.0, TRACE_Z / 2.0 + 1.5);
    }
    tokens
}

fn release_hold_reject_lanes() -> Part {
    let base = centered_cube(
        "closed_inline_bubble_sensor_fpfn_release_hold_reject_lane_plate",
        LANES_X,
        LANES_Y,
        LANES_Z,
    );
    base - disposition_slot_cuts() + disposition_lane_features()
}

fn disposition_slot_cuts() -> Part {
    let mut cuts = Part::empty("closed_inline_bubble_sensor_fpfn_disposition_slot_cuts");
    for lane in DispositionLane::all() {
        let x = centered_index(lane.index(), LANE_COUNT, LANE_PITCH_X);
        for slot in 0..LANE_SLOT_COUNT {
            let y = centered_index(slot, LANE_SLOT_COUNT, 24.0);
            cuts = cuts
                + centered_cube(
                    format!(
                        "closed_inline_bubble_sensor_fpfn_{}_lane_token_slot_{slot}",
                        lane.label()
                    ),
                    48.0,
                    12.0,
                    LANES_Z + 6.0,
                )
                .translate(x, y, 0.0);
        }
    }
    cuts
}

fn disposition_lane_features() -> Part {
    let mut features = Part::empty("closed_inline_bubble_sensor_fpfn_disposition_lane_features");
    for lane in DispositionLane::all() {
        let x = centered_index(lane.index(), LANE_COUNT, LANE_PITCH_X);
        let left_rail = centered_cube(
            format!(
                "closed_inline_bubble_sensor_fpfn_{}_lane_left_rail",
                lane.label()
            ),
            7.0,
            LANES_Y - 28.0,
            16.0,
        )
        .translate(x - 34.0, 0.0, LANES_Z / 2.0 + 8.0);
        let right_rail = centered_cube(
            format!(
                "closed_inline_bubble_sensor_fpfn_{}_lane_right_rail",
                lane.label()
            ),
            7.0,
            LANES_Y - 28.0,
            16.0,
        )
        .translate(x + 34.0, 0.0, LANES_Z / 2.0 + 8.0);
        let gate = centered_cube(
            format!(
                "closed_inline_bubble_sensor_fpfn_{}_lane_gate_block",
                lane.label()
            ),
            76.0,
            12.0,
            lane.gate_height(),
        )
        .translate(
            x,
            LANES_Y / 2.0 - 18.0,
            LANES_Z / 2.0 + lane.gate_height() / 2.0,
        );
        features = features + left_rail + right_rail + gate;
    }
    features
}

fn evidence_bridge() -> Part {
    let post_height = EVIDENCE_UNDERSIDE_Z - BASE_Z / 2.0;
    let left_post = centered_cube(
        "closed_inline_bubble_sensor_fpfn_evidence_bridge_left_post",
        EVIDENCE_POST_X,
        EVIDENCE_POST_Y,
        post_height,
    )
    .translate(
        EVIDENCE_POS.0 - EVIDENCE_SPAN_X / 2.0,
        EVIDENCE_POS.1,
        BASE_Z / 2.0 + post_height / 2.0,
    );
    let right_post = centered_cube(
        "closed_inline_bubble_sensor_fpfn_evidence_bridge_right_post",
        EVIDENCE_POST_X,
        EVIDENCE_POST_Y,
        post_height,
    )
    .translate(
        EVIDENCE_POS.0 + EVIDENCE_SPAN_X / 2.0,
        EVIDENCE_POS.1,
        BASE_Z / 2.0 + post_height / 2.0,
    );
    let cross_beam = centered_cube(
        "closed_inline_bubble_sensor_fpfn_evidence_bridge_cross_beam",
        EVIDENCE_SPAN_X + EVIDENCE_POST_X,
        EVIDENCE_POST_Y,
        EVIDENCE_BEAM_Z,
    )
    .translate(
        EVIDENCE_POS.0,
        EVIDENCE_POS.1,
        EVIDENCE_UNDERSIDE_Z + EVIDENCE_BEAM_Z / 2.0,
    );
    let camera_mounts = evidence_camera_mounts();
    let lights = evidence_light_segments();

    left_post + right_post + cross_beam + camera_mounts + lights
}

fn evidence_camera_mounts() -> Part {
    let mut cameras = Part::empty("closed_inline_bubble_sensor_fpfn_evidence_camera_mounts");
    for i in 0..EVIDENCE_CAMERA_COUNT {
        let x = centered_index(i, EVIDENCE_CAMERA_COUNT, 260.0);
        let bracket = centered_cube(
            format!("closed_inline_bubble_sensor_fpfn_evidence_camera_{i}_bracket"),
            70.0,
            42.0,
            18.0,
        )
        .translate(
            EVIDENCE_POS.0 + x,
            EVIDENCE_POS.1 - 10.0,
            EVIDENCE_UNDERSIDE_Z - 9.0,
        );
        let lens_bore = centered_cylinder(
            format!("closed_inline_bubble_sensor_fpfn_evidence_camera_{i}_lens_bore"),
            18.0 / 2.0,
            24.0,
            28,
        )
        .translate(
            EVIDENCE_POS.0 + x,
            EVIDENCE_POS.1 - 10.0,
            EVIDENCE_UNDERSIDE_Z - 12.0,
        );
        cameras = cameras + (bracket - lens_bore);
    }
    cameras
}

fn evidence_light_segments() -> Part {
    let mut lights = Part::empty("closed_inline_bubble_sensor_fpfn_evidence_light_segments");
    for i in 0..EVIDENCE_LIGHT_SEGMENTS {
        let x = centered_index(i, EVIDENCE_LIGHT_SEGMENTS, 104.0);
        let light = centered_cube(
            format!("closed_inline_bubble_sensor_fpfn_evidence_light_segment_{i}"),
            72.0,
            8.0,
            8.0,
        )
        .translate(
            EVIDENCE_POS.0 + x,
            EVIDENCE_POS.1 - EVIDENCE_POST_Y / 2.0 - 8.0,
            EVIDENCE_UNDERSIDE_Z - 10.0,
        );
        lights = lights + light;
    }
    lights
}

fn clean_used_segregation() -> Part {
    let body = centered_cube(
        "closed_inline_bubble_sensor_fpfn_clean_used_segregation_body",
        SEGREGATION_X,
        SEGREGATION_Y,
        SEGREGATION_Z,
    );
    let divider = centered_cube(
        "closed_inline_bubble_sensor_fpfn_clean_used_center_divider",
        12.0,
        SEGREGATION_Y - 20.0,
        SEGREGATION_Z + 28.0,
    )
    .translate(0.0, 0.0, 14.0);
    let one_way_gate = centered_cube(
        "closed_inline_bubble_sensor_fpfn_clean_to_used_one_way_gate",
        82.0,
        12.0,
        38.0,
    )
    .translate(0.0, 0.0, SEGREGATION_Z / 2.0 + 19.0);
    body - clean_used_segregation_cuts() + divider + one_way_gate + clean_used_caps()
}

fn clean_used_segregation_cuts() -> Part {
    let mut cuts = Part::empty("closed_inline_bubble_sensor_fpfn_clean_used_segregation_cuts");
    for i in 0..CLEAN_COUPON_WELLS {
        let y = centered_index(i, CLEAN_COUPON_WELLS, 22.0);
        cuts = cuts
            + centered_cylinder(
                format!("closed_inline_bubble_sensor_fpfn_clean_coupon_well_{i}"),
                11.0,
                SEGREGATION_Z + 8.0,
                24,
            )
            .translate(-86.0, y, 0.0);
    }
    for i in 0..USED_COUPON_CUPS {
        let y = centered_index(i, USED_COUPON_CUPS, 22.0);
        cuts = cuts
            + centered_cylinder(
                format!("closed_inline_bubble_sensor_fpfn_used_coupon_cup_{i}"),
                13.0,
                SEGREGATION_Z + 8.0,
                24,
            )
            .translate(86.0, y, 0.0);
    }
    let decon_trough = centered_cube(
        "closed_inline_bubble_sensor_fpfn_used_side_decon_trough",
        110.0,
        SEGREGATION_Y - 34.0,
        12.0,
    )
    .translate(86.0, 0.0, SEGREGATION_Z / 2.0 - 6.0);
    cuts + decon_trough
}

fn clean_used_caps() -> Part {
    let mut caps = Part::empty("closed_inline_bubble_sensor_fpfn_clean_used_cap_parking_posts");
    for i in 0..CLEAN_COUPON_WELLS {
        let y = centered_index(i, CLEAN_COUPON_WELLS, 22.0);
        caps = caps
            + centered_cylinder(
                format!("closed_inline_bubble_sensor_fpfn_clean_cap_parking_post_{i}"),
                4.0,
                16.0,
                20,
            )
            .translate(-132.0, y, SEGREGATION_Z / 2.0 + 8.0);
    }
    caps
}

fn robot_service_keepouts() -> Part {
    let front = keepout_rect(
        "closed_inline_bubble_sensor_fpfn_front_robot_sweep_keepout",
        (0.0, -STATION_Y / 2.0 + FRONT_ROBOT_SWEEP_Y / 2.0 + 26.0),
        STATION_X - 160.0,
        FRONT_ROBOT_SWEEP_Y,
        BASE_Z / 2.0 + 8.0,
    );
    let rear = keepout_rect(
        "closed_inline_bubble_sensor_fpfn_rear_service_keepout",
        (0.0, STATION_Y / 2.0 - REAR_SERVICE_CLEARANCE_Y / 2.0 - 26.0),
        STATION_X - 160.0,
        REAR_SERVICE_CLEARANCE_Y,
        BASE_Z / 2.0 + 8.0,
    );
    let left = keepout_rect(
        "closed_inline_bubble_sensor_fpfn_left_channel_nest_service_keepout",
        (-STATION_X / 2.0 + LEFT_NEST_SERVICE_X / 2.0 + 36.0, 25.0),
        LEFT_NEST_SERVICE_X,
        560.0,
        BASE_Z / 2.0 + 8.0,
    );
    let right = keepout_rect(
        "closed_inline_bubble_sensor_fpfn_right_media_service_keepout",
        (STATION_X / 2.0 - RIGHT_MEDIA_SERVICE_X / 2.0 - 36.0, 25.0),
        RIGHT_MEDIA_SERVICE_X,
        560.0,
        BASE_Z / 2.0 + 8.0,
    );
    let lift = keepout_rect(
        "closed_inline_bubble_sensor_fpfn_top_sensor_lift_keepout",
        OPTICAL_POS,
        OPTICAL_X + 80.0,
        OPTICAL_Y + 80.0,
        TOP_SENSOR_LIFT_CLEARANCE_Z,
    ) + vertical_keepout_posts();

    front + rear + left + right + lift
}

fn keepout_rect(name: &str, center: (f64, f64), x: f64, y: f64, z: f64) -> Part {
    let rail = 8.0;
    let left = centered_cube(format!("{name}_left_rail"), rail, y, rail).translate(
        center.0 - x / 2.0,
        center.1,
        z,
    );
    let right = centered_cube(format!("{name}_right_rail"), rail, y, rail).translate(
        center.0 + x / 2.0,
        center.1,
        z,
    );
    let front = centered_cube(format!("{name}_front_rail"), x, rail, rail).translate(
        center.0,
        center.1 - y / 2.0,
        z,
    );
    let rear = centered_cube(format!("{name}_rear_rail"), x, rail, rail).translate(
        center.0,
        center.1 + y / 2.0,
        z,
    );
    left + right + front + rear
}

fn vertical_keepout_posts() -> Part {
    let mut posts = Part::empty("closed_inline_bubble_sensor_fpfn_vertical_keepout_posts");
    for (i, (x, y)) in [
        (
            OPTICAL_POS.0 - (OPTICAL_X + 80.0) / 2.0,
            OPTICAL_POS.1 - (OPTICAL_Y + 80.0) / 2.0,
        ),
        (
            OPTICAL_POS.0 + (OPTICAL_X + 80.0) / 2.0,
            OPTICAL_POS.1 - (OPTICAL_Y + 80.0) / 2.0,
        ),
        (
            OPTICAL_POS.0 - (OPTICAL_X + 80.0) / 2.0,
            OPTICAL_POS.1 + (OPTICAL_Y + 80.0) / 2.0,
        ),
        (
            OPTICAL_POS.0 + (OPTICAL_X + 80.0) / 2.0,
            OPTICAL_POS.1 + (OPTICAL_Y + 80.0) / 2.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(
                format!("closed_inline_bubble_sensor_fpfn_sensor_lift_keepout_post_{i}"),
                8.0,
                8.0,
                TOP_SENSOR_LIFT_CLEARANCE_Z,
            )
            .translate(*x, *y, TOP_SENSOR_LIFT_CLEARANCE_Z / 2.0);
    }
    posts
}

fn fiducial_disc(name: &str) -> Part {
    centered_cylinder(format!("{name}_outer_ring"), 15.0, 4.0, 32)
        - centered_cylinder(format!("{name}_inner_dot"), 5.5, 6.0, 24)
}
