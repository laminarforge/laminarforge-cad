use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed cleanroom/gowning material shedding surrogate station.
//
// Intent:
// - Challenge representative gowning/material coupons inside the closed
//   cell-culture enclosure with a bounded airflow path.
// - Hold each surrogate sample on clamp rails over a collection funnel so
//   released fibers/particles are directed to a removable collection cassette.
// - Keep removable coupons, barcode/certificate evidence, release/hold/reject
//   disposition, airflow direction markers, evidence imaging, and robot/service
//   keepouts visible as mechanical interfaces.
//
// Product-concept CAD only. This is not a cleanroom qualification protocol,
// particle acceptance criterion, gowning procedure, or sterility claim.

const OUTPUTS: [&str; 11] = [
    "output/closed_cleanroom_gowning_material_shedding_surrogate_station_base_particle_tray.stl",
    "output/closed_cleanroom_gowning_material_shedding_surrogate_station_sample_clamp_rails.stl",
    "output/closed_cleanroom_gowning_material_shedding_surrogate_station_particle_collection_funnel.stl",
    "output/closed_cleanroom_gowning_material_shedding_surrogate_station_airflow_challenge_duct.stl",
    "output/closed_cleanroom_gowning_material_shedding_surrogate_station_removable_coupon_holders.stl",
    "output/closed_cleanroom_gowning_material_shedding_surrogate_station_barcode_certificate_lands.stl",
    "output/closed_cleanroom_gowning_material_shedding_surrogate_station_release_hold_reject_lanes.stl",
    "output/closed_cleanroom_gowning_material_shedding_surrogate_station_airflow_direction_arrows.stl",
    "output/closed_cleanroom_gowning_material_shedding_surrogate_station_evidence_bridge.stl",
    "output/closed_cleanroom_gowning_material_shedding_surrogate_station_robot_service_keepouts.stl",
    "output/closed_cleanroom_gowning_material_shedding_surrogate_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 10] = [
    "sample_clamp_rails",
    "particle_collection_funnel",
    "airflow_challenge_duct",
    "removable_coupon_holders",
    "barcode_certificate_lands",
    "release_hold_reject_lanes",
    "airflow_direction_arrows",
    "evidence_bridge",
    "robot_service_keepouts",
    "base_particle_tray",
];

const DECK_X: f64 = 1500.0;
const DECK_Y: f64 = 920.0;
const DECK_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 46.0;
const SOCKET_DEPTH: f64 = 6.0;

const CLAMP_RAIL_X: f64 = 760.0;
const CLAMP_RAIL_Y: f64 = 192.0;
const CLAMP_POS: (f64, f64) = (0.0, 144.0);
const SAMPLE_LANES: usize = 6;
const CLAMPS_PER_LANE: usize = 2;
const SAMPLE_LANE_PITCH_X: f64 = 110.0;
const CLAMP_RAIL_SPAN_Y: f64 = 116.0;
const SAMPLE_CLEAR_APERTURE_X: f64 = 86.0;

const FUNNEL_X: f64 = 560.0;
const FUNNEL_Y: f64 = 290.0;
const FUNNEL_Z: f64 = 118.0;
const FUNNEL_POS: (f64, f64) = (0.0, -36.0);
const FUNNEL_MOUTH_D: f64 = 380.0;
const FUNNEL_THROAT_D: f64 = 78.0;
const COLLECTION_CASSETTES: usize = 3;

const DUCT_X: f64 = 940.0;
const DUCT_Y: f64 = 180.0;
const DUCT_Z: f64 = 150.0;
const DUCT_POS: (f64, f64) = (0.0, 318.0);
const FLOW_STRAIGHTENER_ROWS: usize = 3;
const FLOW_STRAIGHTENER_COLS: usize = 9;
const NOZZLE_SLOTS: usize = SAMPLE_LANES;
const PRESSURE_TAPS: usize = 4;

const COUPON_HOLDER_X: f64 = 346.0;
const COUPON_HOLDER_Y: f64 = 278.0;
const COUPON_HOLDER_Z: f64 = 42.0;
const COUPON_POS: (f64, f64) = (-498.0, -122.0);
const COUPON_ROWS: usize = 3;
const COUPON_COLS: usize = 4;
const COUPON_COUNT: usize = COUPON_ROWS * COUPON_COLS;
const COUPON_SLOT_X: f64 = 62.0;
const COUPON_SLOT_Y: f64 = 38.0;
const COUPON_PITCH_X: f64 = 78.0;
const COUPON_PITCH_Y: f64 = 70.0;

const TRACE_PANEL_X: f64 = 366.0;
const TRACE_PANEL_Y: f64 = 178.0;
const TRACE_PANEL_Z: f64 = 10.0;
const TRACE_POS: (f64, f64) = (460.0, -120.0);
const BARCODE_LANDS: usize = 12;
const CERTIFICATE_LANDS: usize = 4;
const CHAIN_OF_CUSTODY_CARD_LANDS: usize = 3;

const STATUS_X: f64 = 530.0;
const STATUS_Y: f64 = 168.0;
const STATUS_Z: f64 = 44.0;
const STATUS_POS: (f64, f64) = (0.0, -346.0);
const DISPOSITION_LANES: usize = 3;
const STATUS_SLOTS_PER_LANE: usize = 6;
const STATUS_SLOT_X: f64 = 68.0;
const STATUS_SLOT_Y: f64 = 34.0;
const STATUS_LANE_PITCH_Y: f64 = 48.0;

const AIRFLOW_ARROW_COUNT: usize = 9;
const BRIDGE_X: f64 = 1040.0;
const BRIDGE_Y: f64 = 250.0;
const BRIDGE_UNDERSIDE_Z: f64 = 274.0;
const BRIDGE_BEAM_Z: f64 = 30.0;
const BRIDGE_POS: (f64, f64) = (0.0, 110.0);
const CAMERA_PODS: usize = 4;
const LED_BARS: usize = 2;

const FRONT_ROBOT_KEEP_OUT: f64 = 390.0;
const REAR_DUCT_SERVICE_KEEP_OUT: f64 = 270.0;
const LEFT_COUPON_SERVICE_KEEP_OUT: f64 = 240.0;
const RIGHT_CERT_SERVICE_KEEP_OUT: f64 = 230.0;
const ROBOT_Z_CLEARANCE: f64 = 360.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside(self, deck_x: f64, deck_y: f64) -> bool {
        let half_x = deck_x / 2.0 - RIM_W - 8.0;
        let half_y = deck_y / 2.0 - RIM_W - 8.0;
        self.center.0.abs() + self.x / 2.0 <= half_x && self.center.1.abs() + self.y / 2.0 <= half_y
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_particle_tray();
    export(OUTPUTS[0], &base);

    let clamp_rails = sample_clamp_rails();
    export(OUTPUTS[1], &clamp_rails);

    let funnel = particle_collection_funnel();
    export(OUTPUTS[2], &funnel);

    let duct = airflow_challenge_duct();
    export(OUTPUTS[3], &duct);

    let coupons = removable_coupon_holders();
    export(OUTPUTS[4], &coupons);

    let traceability = barcode_certificate_lands();
    export(OUTPUTS[5], &traceability);

    let status = release_hold_reject_lanes();
    export(OUTPUTS[6], &status);

    let arrows = airflow_direction_arrows();
    export(OUTPUTS[7], &arrows);

    let bridge = evidence_bridge();
    export(OUTPUTS[8], &bridge);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[9], &keepouts);

    let insert_z = insert_z();
    let assembly = base
        + clamp_rails.translate(CLAMP_POS.0, CLAMP_POS.1, insert_z)
        + funnel.translate(FUNNEL_POS.0, FUNNEL_POS.1, insert_z)
        + duct.translate(DUCT_POS.0, DUCT_POS.1, insert_z)
        + coupons.translate(COUPON_POS.0, COUPON_POS.1, insert_z)
        + traceability.translate(TRACE_POS.0, TRACE_POS.1, insert_z)
        + status.translate(STATUS_POS.0, STATUS_POS.1, insert_z)
        + arrows
        + bridge.translate(BRIDGE_POS.0, BRIDGE_POS.1, DECK_Z)
        + keepouts;
    export(OUTPUTS[10], &assembly);

    println!();
    println!("Closed cleanroom gowning/material shedding surrogate station:");
    println!("  Footprint:                  {DECK_X:.0}mm x {DECK_Y:.0}mm contained particle tray");
    println!(
        "  Sample challenge:           {SAMPLE_LANES} sample lanes, {CLAMPS_PER_LANE} clamps per lane, {NOZZLE_SLOTS} airflow nozzle slots, {PRESSURE_TAPS} pressure taps"
    );
    println!(
        "  Particle collection:        {FUNNEL_MOUTH_D:.0}mm mouth tapering to {FUNNEL_THROAT_D:.0}mm throat with {COLLECTION_CASSETTES} removable collection cassette pockets"
    );
    println!(
        "  Coupons and traceability:   {COUPON_COUNT} removable coupon holders, {BARCODE_LANDS} barcode lands, {CERTIFICATE_LANDS} certificate lands, {CHAIN_OF_CUSTODY_CARD_LANDS} custody card lands"
    );
    println!(
        "  Disposition/evidence:       release/hold/reject lanes with {STATUS_SLOTS_PER_LANE} slots each, {AIRFLOW_ARROW_COUNT} geometric airflow arrows, {CAMERA_PODS} camera pods, {LED_BARS} LED bars"
    );
    println!(
        "  Keepouts:                   front robot {FRONT_ROBOT_KEEP_OUT:.0}mm, rear duct service {REAR_DUCT_SERVICE_KEEP_OUT:.0}mm, left coupon service {LEFT_COUPON_SERVICE_KEEP_OUT:.0}mm, right certificate service {RIGHT_CERT_SERVICE_KEEP_OUT:.0}mm, Z clearance {ROBOT_Z_CLEARANCE:.0}mm"
    );
    println!("  Feature groups covered:     {}", REQUIRED_FEATURES.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn insert_z() -> f64 {
    DECK_Z - SOCKET_DEPTH / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn layout_rects() -> [Rect; 6] {
    [
        rect("sample_clamp_rails", CLAMP_POS, CLAMP_RAIL_X, CLAMP_RAIL_Y),
        rect("particle_collection_funnel", FUNNEL_POS, FUNNEL_X, FUNNEL_Y),
        rect("airflow_challenge_duct", DUCT_POS, DUCT_X, DUCT_Y),
        rect(
            "removable_coupon_holders",
            COUPON_POS,
            COUPON_HOLDER_X,
            COUPON_HOLDER_Y,
        ),
        rect(
            "barcode_certificate_lands",
            TRACE_POS,
            TRACE_PANEL_X,
            TRACE_PANEL_Y,
        ),
        rect("release_hold_reject_lanes", STATUS_POS, STATUS_X, STATUS_Y),
    ]
}

fn assert_layout() {
    for rect in layout_rects() {
        assert!(
            rect.fits_inside(DECK_X, DECK_Y),
            "{} exceeds closed station deck envelope",
            rect.name
        );
    }

    assert_eq!(COUPON_COUNT, COUPON_ROWS * COUPON_COLS);
    assert_eq!(NOZZLE_SLOTS, SAMPLE_LANES);
    assert_eq!(DISPOSITION_LANES, 3);
    assert_eq!(FLOW_STRAIGHTENER_ROWS * FLOW_STRAIGHTENER_COLS, 27);
    assert!(FUNNEL_THROAT_D < FUNNEL_MOUTH_D);
    assert!(CLAMP_RAIL_SPAN_Y < FUNNEL_MOUTH_D);
    assert!(BRIDGE_UNDERSIDE_Z > DUCT_Z + 80.0);
    assert!(ROBOT_Z_CLEARANCE > BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z);
}

fn base_particle_tray() -> Part {
    let deck = centered_cube(
        "gowning_shedding_station_base_particle_tray_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);

    let recessed_pan = centered_cube(
        "gowning_shedding_station_wipeable_recessed_particle_pan",
        DECK_X - 150.0,
        DECK_Y - 132.0,
        8.0,
    )
    .translate(0.0, -6.0, DECK_Z - 4.0);
    let sample_to_funnel_gutter = centered_cube(
        "gowning_shedding_station_sample_to_funnel_particle_gutter",
        860.0,
        210.0,
        9.0,
    )
    .translate(0.0, 42.0, DECK_Z - 4.5);
    let disposition_sump = centered_cube(
        "gowning_shedding_station_status_lane_low_point_sump",
        670.0,
        72.0,
        8.0,
    )
    .translate(0.0, STATUS_POS.1, DECK_Z - 4.0);
    let drain_port = centered_cylinder(
        "gowning_shedding_station_closed_particle_tray_drain_port",
        9.0,
        46.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 92.0, -DECK_Y / 2.0 + 34.0, DECK_Z - 8.0);

    deck - recessed_pan
        - sample_to_funnel_gutter
        - disposition_sump
        - drain_port
        - insert_sockets()
        - deck_mount_holes()
        - datum_pin_holes()
        + perimeter_rim()
        + base_zone_lands()
        + clean_dirty_dividers()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("gowning_shedding_station_insert_registration_sockets");
    for rect in layout_rects() {
        sockets = sockets
            + centered_cube(
                format!("gowning_shedding_station_socket_{}", rect.name),
                rect.x + 14.0,
                rect.y + 14.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(
                rect.center.0,
                rect.center.1,
                DECK_Z - SOCKET_DEPTH / 2.0 + 0.2,
            );
    }
    sockets
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("gowning_shedding_station_deck_mount_holes");
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 62.0, -DECK_Y / 2.0 + 58.0),
        (DECK_X / 2.0 - 62.0, -DECK_Y / 2.0 + 58.0),
        (-DECK_X / 2.0 + 62.0, DECK_Y / 2.0 - 58.0),
        (DECK_X / 2.0 - 62.0, DECK_Y / 2.0 - 58.0),
        (-DECK_X / 6.0, -DECK_Y / 2.0 + 58.0),
        (DECK_X / 6.0, -DECK_Y / 2.0 + 58.0),
        (-DECK_X / 6.0, DECK_Y / 2.0 - 58.0),
        (DECK_X / 6.0, DECK_Y / 2.0 - 58.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("gowning_shedding_station_m6_mount_clearance_{i}"),
                3.4,
                DECK_Z + 4.0,
                28,
            )
            .translate(*x, *y, DECK_Z / 2.0);
    }
    holes
}

fn datum_pin_holes() -> Part {
    let mut holes = Part::empty("gowning_shedding_station_datum_pin_holes");
    for (i, (x, y)) in [
        (-650.0, 350.0),
        (650.0, 350.0),
        (-650.0, -350.0),
        (650.0, -350.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("gowning_shedding_station_datum_pin_clearance_{i}"),
                2.6,
                DECK_Z + 4.0,
                24,
            )
            .translate(*x, *y, DECK_Z / 2.0);
    }
    holes
}

fn perimeter_rim() -> Part {
    let front = centered_cube(
        "gowning_shedding_station_front_particle_containment_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        "gowning_shedding_station_rear_air_duct_service_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "gowning_shedding_station_left_coupon_exchange_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "gowning_shedding_station_right_traceability_service_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);

    front + rear + left + right
}

fn base_zone_lands() -> Part {
    let incoming = centered_cube(
        "gowning_shedding_station_clean_coupon_incoming_floor_land",
        420.0,
        136.0,
        4.0,
    )
    .translate(COUPON_POS.0, COUPON_POS.1 + 88.0, DECK_Z + 2.0);
    let challenge = centered_cube(
        "gowning_shedding_station_airflow_challenge_floor_land",
        900.0,
        332.0,
        4.0,
    )
    .translate(0.0, 94.0, DECK_Z + 2.0);
    let trace = centered_cube(
        "gowning_shedding_station_traceability_floor_land",
        430.0,
        138.0,
        4.0,
    )
    .translate(TRACE_POS.0, TRACE_POS.1 + 66.0, DECK_Z + 2.0);
    let disposition = centered_cube(
        "gowning_shedding_station_release_hold_reject_floor_land",
        680.0,
        116.0,
        4.0,
    )
    .translate(STATUS_POS.0, STATUS_POS.1, DECK_Z + 2.0);

    incoming + challenge + trace + disposition
}

fn clean_dirty_dividers() -> Part {
    let clean_to_challenge = centered_cube(
        "gowning_shedding_station_coupon_to_challenge_lane_divider",
        12.0,
        390.0,
        32.0,
    )
    .translate(-276.0, 18.0, DECK_Z + 16.0);
    let trace_to_challenge = centered_cube(
        "gowning_shedding_station_traceability_to_challenge_lane_divider",
        12.0,
        362.0,
        32.0,
    )
    .translate(296.0, 10.0, DECK_Z + 16.0);
    let challenge_to_status = centered_cube(
        "gowning_shedding_station_challenge_to_status_particle_barrier",
        1030.0,
        12.0,
        30.0,
    )
    .translate(0.0, -236.0, DECK_Z + 15.0);

    clean_to_challenge + trace_to_challenge + challenge_to_status
}

fn sample_clamp_rails() -> Part {
    let backer = centered_cube(
        "gowning_shedding_sample_clamp_backer_plate",
        CLAMP_RAIL_X,
        CLAMP_RAIL_Y,
        12.0,
    )
    .translate(0.0, 0.0, 6.0);
    let center_aperture = centered_cube(
        "gowning_shedding_sample_clamp_center_air_aperture",
        CLAMP_RAIL_X - 124.0,
        78.0,
        14.0,
    )
    .translate(0.0, 0.0, 6.0);

    backer - center_aperture
        + sample_rail_bars()
        + sample_lane_crossbars()
        + clamp_station_blocks()
        + rail_end_datums()
}

fn sample_rail_bars() -> Part {
    let upstream = centered_cube(
        "gowning_shedding_upstream_clamp_rail",
        CLAMP_RAIL_X - 70.0,
        18.0,
        34.0,
    )
    .translate(0.0, CLAMP_RAIL_SPAN_Y / 2.0, 29.0);
    let downstream = centered_cube(
        "gowning_shedding_downstream_clamp_rail",
        CLAMP_RAIL_X - 70.0,
        18.0,
        34.0,
    )
    .translate(0.0, -CLAMP_RAIL_SPAN_Y / 2.0, 29.0);
    let upstream_slot = centered_cube(
        "gowning_shedding_upstream_clamp_rail_undercut",
        CLAMP_RAIL_X - 126.0,
        6.0,
        18.0,
    )
    .translate(0.0, CLAMP_RAIL_SPAN_Y / 2.0, 27.0);
    let downstream_slot = centered_cube(
        "gowning_shedding_downstream_clamp_rail_undercut",
        CLAMP_RAIL_X - 126.0,
        6.0,
        18.0,
    )
    .translate(0.0, -CLAMP_RAIL_SPAN_Y / 2.0, 27.0);

    upstream + downstream - upstream_slot - downstream_slot
}

fn sample_lane_crossbars() -> Part {
    let mut bars = Part::empty("gowning_shedding_sample_lane_air_gap_crossbars");
    for lane in 0..SAMPLE_LANES {
        let x = centered_index(lane, SAMPLE_LANES, SAMPLE_LANE_PITCH_X);
        let witness_land = centered_cube(
            format!("gowning_shedding_surrogate_sample_witness_land_{lane}"),
            SAMPLE_CLEAR_APERTURE_X,
            30.0,
            5.0,
        )
        .translate(x, 0.0, 18.5);
        let upstream_crossbar = centered_cube(
            format!("gowning_shedding_upstream_sample_end_bar_{lane}"),
            SAMPLE_CLEAR_APERTURE_X + 20.0,
            10.0,
            18.0,
        )
        .translate(x, CLAMP_RAIL_SPAN_Y / 2.0 - 26.0, 31.0);
        let downstream_crossbar = centered_cube(
            format!("gowning_shedding_downstream_sample_end_bar_{lane}"),
            SAMPLE_CLEAR_APERTURE_X + 20.0,
            10.0,
            18.0,
        )
        .translate(x, -CLAMP_RAIL_SPAN_Y / 2.0 + 26.0, 31.0);
        bars = bars + witness_land + upstream_crossbar + downstream_crossbar;
    }
    bars
}

fn clamp_station_blocks() -> Part {
    let mut clamps = Part::empty("gowning_shedding_material_sample_clamp_blocks");
    for lane in 0..SAMPLE_LANES {
        let x = centered_index(lane, SAMPLE_LANES, SAMPLE_LANE_PITCH_X);
        for clamp in 0..CLAMPS_PER_LANE {
            let y = if clamp == 0 {
                CLAMP_RAIL_SPAN_Y / 2.0
            } else {
                -CLAMP_RAIL_SPAN_Y / 2.0
            };
            let pad = centered_cube(
                format!("gowning_shedding_lane_{lane}_spring_clamp_pad_{clamp}"),
                54.0,
                30.0,
                18.0,
            )
            .translate(x, y, 56.0);
            let screw = centered_cylinder(
                format!("gowning_shedding_lane_{lane}_thumb_screw_boss_{clamp}"),
                10.0,
                18.0,
                32,
            )
            .translate(x, y, 76.0);
            let screw_clearance = centered_cylinder(
                format!("gowning_shedding_lane_{lane}_thumb_screw_clearance_{clamp}"),
                3.0,
                24.0,
                20,
            )
            .translate(x, y, 76.0);
            clamps = clamps + pad + (screw - screw_clearance);
        }
    }
    clamps
}

fn rail_end_datums() -> Part {
    let mut datums = Part::empty("gowning_shedding_clamp_rail_end_datums");
    for (i, x) in [-(CLAMP_RAIL_X / 2.0 - 42.0), CLAMP_RAIL_X / 2.0 - 42.0]
        .iter()
        .enumerate()
    {
        let block = centered_cube(
            format!("gowning_shedding_clamp_rail_end_stop_{i}"),
            20.0,
            CLAMP_RAIL_Y - 28.0,
            42.0,
        )
        .translate(*x, 0.0, 33.0);
        let pin = centered_cylinder(
            format!("gowning_shedding_clamp_rail_pick_datum_pin_{i}"),
            4.0,
            20.0,
            24,
        )
        .translate(*x, 0.0, 60.0);
        datums = datums + block + pin;
    }
    datums
}

fn particle_collection_funnel() -> Part {
    let flange = centered_cube(
        "gowning_shedding_particle_funnel_top_mount_flange",
        FUNNEL_X,
        FUNNEL_Y,
        16.0,
    )
    .translate(0.0, 0.0, FUNNEL_Z - 8.0);
    let flange_cut = centered_cube(
        "gowning_shedding_particle_funnel_mouth_cut",
        FUNNEL_X - 150.0,
        FUNNEL_Y - 70.0,
        20.0,
    )
    .translate(0.0, 0.0, FUNNEL_Z - 8.0);

    let outer_taper = Part::cone(
        "gowning_shedding_particle_collection_outer_taper",
        FUNNEL_MOUTH_D / 2.0,
        FUNNEL_THROAT_D / 2.0 + 18.0,
        FUNNEL_Z - 18.0,
        96,
    )
    .translate(0.0, 0.0, (FUNNEL_Z - 18.0) / 2.0);
    let inner_taper = Part::cone(
        "gowning_shedding_particle_collection_inner_air_path",
        FUNNEL_MOUTH_D / 2.0 - 16.0,
        FUNNEL_THROAT_D / 2.0 + 6.0,
        FUNNEL_Z - 8.0,
        96,
    )
    .translate(0.0, 0.0, (FUNNEL_Z - 8.0) / 2.0 + 2.0);

    let throat_boss = centered_cylinder(
        "gowning_shedding_collection_filter_cassette_throat_boss",
        FUNNEL_THROAT_D / 2.0 + 18.0,
        24.0,
        64,
    )
    .translate(0.0, 0.0, 16.0);
    let throat_cut = centered_cylinder(
        "gowning_shedding_collection_filter_cassette_throat_cut",
        FUNNEL_THROAT_D / 2.0,
        30.0,
        64,
    )
    .translate(0.0, 0.0, 16.0);

    (flange - flange_cut)
        + (outer_taper - inner_taper)
        + (throat_boss - throat_cut)
        + collection_cassette_slide()
        + funnel_crosshair_grid()
        + funnel_alignment_pins()
}

fn collection_cassette_slide() -> Part {
    let slide_plate = centered_cube(
        "gowning_shedding_collection_cassette_slide_plate",
        330.0,
        126.0,
        18.0,
    )
    .translate(0.0, -FUNNEL_Y / 2.0 - 30.0, 24.0);
    let slide_cut = centered_cube(
        "gowning_shedding_collection_cassette_withdrawal_clearance",
        246.0,
        72.0,
        20.0,
    )
    .translate(0.0, -FUNNEL_Y / 2.0 - 30.0, 24.0);

    let mut pockets = Part::empty("gowning_shedding_collection_filter_cassette_pockets");
    for i in 0..COLLECTION_CASSETTES {
        let x = centered_index(i, COLLECTION_CASSETTES, 88.0);
        let land = centered_cube(
            format!("gowning_shedding_collection_cassette_index_land_{i}"),
            68.0,
            30.0,
            6.0,
        )
        .translate(x, -FUNNEL_Y / 2.0 - 86.0, 39.0);
        let detent = centered_cylinder(
            format!("gowning_shedding_collection_cassette_ball_detent_{i}"),
            4.0,
            10.0,
            20,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -FUNNEL_Y / 2.0 - 66.0, 38.0);
        pockets = pockets + land + detent;
    }

    (slide_plate - slide_cut) + pockets
}

fn funnel_crosshair_grid() -> Part {
    let x_bar = centered_cube(
        "gowning_shedding_funnel_mouth_crosshair_x_bar",
        FUNNEL_X - 118.0,
        7.0,
        8.0,
    )
    .translate(0.0, 0.0, FUNNEL_Z + 4.0);
    let y_bar = centered_cube(
        "gowning_shedding_funnel_mouth_crosshair_y_bar",
        7.0,
        FUNNEL_Y - 72.0,
        8.0,
    )
    .translate(0.0, 0.0, FUNNEL_Z + 4.0);
    let upstream_witness = centered_cube(
        "gowning_shedding_funnel_upstream_edge_witness_strip",
        FUNNEL_X - 180.0,
        10.0,
        6.0,
    )
    .translate(0.0, FUNNEL_Y / 2.0 - 32.0, FUNNEL_Z + 3.0);

    x_bar + y_bar + upstream_witness
}

fn funnel_alignment_pins() -> Part {
    let mut pins = Part::empty("gowning_shedding_funnel_alignment_pins");
    for (i, (x, y)) in [
        (-FUNNEL_X / 2.0 + 48.0, -FUNNEL_Y / 2.0 + 42.0),
        (FUNNEL_X / 2.0 - 48.0, -FUNNEL_Y / 2.0 + 42.0),
        (-FUNNEL_X / 2.0 + 48.0, FUNNEL_Y / 2.0 - 42.0),
        (FUNNEL_X / 2.0 - 48.0, FUNNEL_Y / 2.0 - 42.0),
    ]
    .iter()
    .enumerate()
    {
        pins = pins
            + centered_cylinder(
                format!("gowning_shedding_funnel_alignment_pin_{i}"),
                4.0,
                18.0,
                24,
            )
            .translate(*x, *y, FUNNEL_Z + 9.0);
    }
    pins
}

fn airflow_challenge_duct() -> Part {
    let plenum = centered_cube(
        "gowning_shedding_airflow_challenge_rectangular_plenum",
        DUCT_X,
        DUCT_Y,
        86.0,
    )
    .translate(0.0, 0.0, 84.0);
    let plenum_void = centered_cube(
        "gowning_shedding_airflow_challenge_plenum_void",
        DUCT_X - 70.0,
        DUCT_Y - 54.0,
        62.0,
    )
    .translate(0.0, 0.0, 84.0);
    let top_service_lid = centered_cube(
        "gowning_shedding_airflow_duct_removable_service_lid_land",
        DUCT_X - 110.0,
        DUCT_Y - 60.0,
        10.0,
    )
    .translate(0.0, 0.0, 132.0);

    (plenum - plenum_void)
        + top_service_lid
        + hepa_inlet_filter_frame()
        + flow_straightener_grid()
        + nozzle_slot_array()
        + pressure_tap_manifold()
        + duct_side_latches()
}

fn hepa_inlet_filter_frame() -> Part {
    let outer = centered_cube(
        "gowning_shedding_airflow_challenge_hepa_inlet_outer_frame",
        DUCT_X - 120.0,
        18.0,
        132.0,
    )
    .translate(0.0, DUCT_Y / 2.0 + 8.0, 76.0);
    let inner = centered_cube(
        "gowning_shedding_airflow_challenge_hepa_inlet_opening",
        DUCT_X - 190.0,
        22.0,
        86.0,
    )
    .translate(0.0, DUCT_Y / 2.0 + 8.0, 76.0);
    let gasket_land = centered_cube(
        "gowning_shedding_airflow_challenge_hepa_gasket_land",
        DUCT_X - 86.0,
        10.0,
        16.0,
    )
    .translate(0.0, DUCT_Y / 2.0 + 25.0, 142.0);

    (outer - inner) + gasket_land
}

fn flow_straightener_grid() -> Part {
    let grid_bar = centered_cube(
        "gowning_shedding_airflow_straightener_honeycomb_frame",
        DUCT_X - 172.0,
        18.0,
        72.0,
    )
    .translate(0.0, DUCT_Y / 2.0 - 24.0, 82.0);
    let mut cells = Part::empty("gowning_shedding_airflow_straightener_cell_cuts");
    for row in 0..FLOW_STRAIGHTENER_ROWS {
        let z = 58.0 + row as f64 * 24.0;
        for col in 0..FLOW_STRAIGHTENER_COLS {
            let x = centered_index(col, FLOW_STRAIGHTENER_COLS, 78.0);
            cells = cells
                + centered_cylinder(
                    format!("gowning_shedding_airflow_straightener_cell_{row}_{col}"),
                    11.0,
                    24.0,
                    24,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(x, DUCT_Y / 2.0 - 24.0, z);
        }
    }
    grid_bar - cells
}

fn nozzle_slot_array() -> Part {
    let nozzle_bar = centered_cube(
        "gowning_shedding_airflow_nozzle_slot_bar",
        DUCT_X - 132.0,
        36.0,
        34.0,
    )
    .translate(0.0, -DUCT_Y / 2.0 - 10.0, 50.0);
    let mut slots = Part::empty("gowning_shedding_airflow_nozzle_slot_cuts");
    let mut lips = Part::empty("gowning_shedding_airflow_nozzle_lip_witnesses");
    for slot in 0..NOZZLE_SLOTS {
        let x = centered_index(slot, NOZZLE_SLOTS, SAMPLE_LANE_PITCH_X);
        slots = slots
            + centered_cube(
                format!("gowning_shedding_airflow_lane_{slot}_knife_edge_nozzle_slot"),
                72.0,
                40.0,
                12.0,
            )
            .translate(x, -DUCT_Y / 2.0 - 10.0, 50.0);
        lips = lips
            + centered_cube(
                format!("gowning_shedding_airflow_lane_{slot}_nozzle_lower_lip"),
                78.0,
                8.0,
                9.0,
            )
            .translate(x, -DUCT_Y / 2.0 - 34.0, 37.0);
    }
    (nozzle_bar - slots) + lips
}

fn pressure_tap_manifold() -> Part {
    let manifold = centered_cube(
        "gowning_shedding_airflow_pressure_tap_manifold_bar",
        330.0,
        20.0,
        24.0,
    )
    .translate(DUCT_X / 2.0 - 190.0, 0.0, 138.0);
    let mut taps = Part::empty("gowning_shedding_airflow_pressure_taps");
    for i in 0..PRESSURE_TAPS {
        let x = DUCT_X / 2.0 - 300.0 + i as f64 * 72.0;
        let boss = centered_cylinder(
            format!("gowning_shedding_airflow_pressure_tap_boss_{i}"),
            8.0,
            18.0,
            24,
        )
        .translate(x, 0.0, 156.0);
        let bore = centered_cylinder(
            format!("gowning_shedding_airflow_pressure_tap_bore_{i}"),
            2.2,
            22.0,
            18,
        )
        .translate(x, 0.0, 156.0);
        taps = taps + (boss - bore);
    }
    manifold + taps
}

fn duct_side_latches() -> Part {
    let mut latches = Part::empty("gowning_shedding_airflow_challenge_duct_side_latches");
    for (i, x) in [-(DUCT_X / 2.0 - 46.0), DUCT_X / 2.0 - 46.0]
        .iter()
        .enumerate()
    {
        let latch = centered_cube(
            format!("gowning_shedding_airflow_challenge_duct_cam_latch_land_{i}"),
            24.0,
            58.0,
            22.0,
        )
        .translate(*x, -10.0, 138.0);
        let hinge = centered_cylinder(
            format!("gowning_shedding_airflow_challenge_duct_latch_hinge_{i}"),
            5.0,
            64.0,
            20,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, -10.0, 124.0);
        latches = latches + latch + hinge;
    }
    latches
}

fn removable_coupon_holders() -> Part {
    let body = centered_cube(
        "gowning_shedding_removable_coupon_holder_tray_body",
        COUPON_HOLDER_X,
        COUPON_HOLDER_Y,
        COUPON_HOLDER_Z,
    )
    .translate(0.0, 0.0, COUPON_HOLDER_Z / 2.0);
    let grip_trough = centered_cube(
        "gowning_shedding_removable_coupon_holder_front_grip_trough",
        COUPON_HOLDER_X - 54.0,
        26.0,
        COUPON_HOLDER_Z + 4.0,
    )
    .translate(0.0, -COUPON_HOLDER_Y / 2.0 + 28.0, COUPON_HOLDER_Z / 2.0);

    body - coupon_slot_cuts() - grip_trough
        + coupon_holder_latches()
        + coupon_holder_handles()
        + coupon_holder_datum_pins()
}

fn coupon_slot_cuts() -> Part {
    let mut cuts = Part::empty("gowning_shedding_removable_coupon_slot_cuts");
    for row in 0..COUPON_ROWS {
        let y = centered_index(row, COUPON_ROWS, COUPON_PITCH_Y);
        for col in 0..COUPON_COLS {
            let x = centered_index(col, COUPON_COLS, COUPON_PITCH_X);
            cuts = cuts
                + centered_cube(
                    format!("gowning_shedding_removable_coupon_slot_{row}_{col}"),
                    COUPON_SLOT_X,
                    COUPON_SLOT_Y,
                    22.0,
                )
                .translate(x, y, COUPON_HOLDER_Z - 8.0);
        }
    }
    cuts
}

fn coupon_holder_latches() -> Part {
    let mut latches = Part::empty("gowning_shedding_coupon_holder_spring_latches");
    for row in 0..COUPON_ROWS {
        let y = centered_index(row, COUPON_ROWS, COUPON_PITCH_Y);
        for col in 0..COUPON_COLS {
            let x = centered_index(col, COUPON_COLS, COUPON_PITCH_X);
            let index = row * COUPON_COLS + col;
            latches = latches
                + centered_cube(
                    format!("gowning_shedding_coupon_holder_latch_land_{index}"),
                    42.0,
                    8.0,
                    8.0,
                )
                .translate(x, y + COUPON_SLOT_Y / 2.0 + 8.0, COUPON_HOLDER_Z + 4.0)
                + centered_cube(
                    format!("gowning_shedding_coupon_holder_negative_stop_{index}"),
                    42.0,
                    8.0,
                    8.0,
                )
                .translate(x, y - COUPON_SLOT_Y / 2.0 - 8.0, COUPON_HOLDER_Z + 4.0);
        }
    }
    latches
}

fn coupon_holder_handles() -> Part {
    let mut handles = Part::empty("gowning_shedding_coupon_holder_pull_handles");
    for row in 0..COUPON_ROWS {
        let y = centered_index(row, COUPON_ROWS, COUPON_PITCH_Y);
        let handle = centered_cube(
            format!("gowning_shedding_coupon_holder_row_{row}_removal_handle"),
            COUPON_HOLDER_X - 72.0,
            12.0,
            18.0,
        )
        .translate(0.0, y + 28.0, COUPON_HOLDER_Z + 18.0);
        let undercut = centered_cube(
            format!("gowning_shedding_coupon_holder_row_{row}_handle_undercut"),
            COUPON_HOLDER_X - 112.0,
            8.0,
            10.0,
        )
        .translate(0.0, y + 28.0, COUPON_HOLDER_Z + 18.0);
        handles = handles + (handle - undercut);
    }
    handles
}

fn coupon_holder_datum_pins() -> Part {
    let mut pins = Part::empty("gowning_shedding_coupon_holder_datum_pins");
    for (i, (x, y)) in [
        (-COUPON_HOLDER_X / 2.0 + 32.0, -COUPON_HOLDER_Y / 2.0 + 30.0),
        (COUPON_HOLDER_X / 2.0 - 32.0, -COUPON_HOLDER_Y / 2.0 + 30.0),
        (-COUPON_HOLDER_X / 2.0 + 32.0, COUPON_HOLDER_Y / 2.0 - 30.0),
        (COUPON_HOLDER_X / 2.0 - 32.0, COUPON_HOLDER_Y / 2.0 - 30.0),
    ]
    .iter()
    .enumerate()
    {
        pins = pins
            + centered_cylinder(
                format!("gowning_shedding_coupon_holder_datum_pin_{i}"),
                3.5,
                16.0,
                24,
            )
            .translate(*x, *y, COUPON_HOLDER_Z + 8.0);
    }
    pins
}

fn barcode_certificate_lands() -> Part {
    let panel = centered_cube(
        "gowning_shedding_barcode_certificate_panel",
        TRACE_PANEL_X,
        TRACE_PANEL_Y,
        TRACE_PANEL_Z,
    )
    .translate(0.0, 0.0, TRACE_PANEL_Z / 2.0);
    panel + barcode_lands() + certificate_lands() + custody_card_lands() + scanner_datum_frame()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("gowning_shedding_barcode_scan_lands");
    for i in 0..BARCODE_LANDS {
        let col = i % 3;
        let row = i / 3;
        let x = -112.0 + col as f64 * 112.0;
        let y = 48.0 - row as f64 * 32.0;
        lands = lands
            + centered_cube(
                format!("gowning_shedding_coupon_barcode_land_{i}"),
                86.0,
                20.0,
                4.0,
            )
            .translate(x, y, TRACE_PANEL_Z + 2.0);
    }
    lands
}

fn certificate_lands() -> Part {
    let mut lands = Part::empty("gowning_shedding_certificate_lands");
    for i in 0..CERTIFICATE_LANDS {
        let y = -58.0 + i as f64 * 38.0;
        lands = lands
            + centered_cube(
                format!("gowning_shedding_material_certificate_card_land_{i}"),
                74.0,
                28.0,
                5.0,
            )
            .translate(TRACE_PANEL_X / 2.0 - 56.0, y, TRACE_PANEL_Z + 2.5);
    }
    lands
}

fn custody_card_lands() -> Part {
    let mut lands = Part::empty("gowning_shedding_chain_of_custody_card_lands");
    for i in 0..CHAIN_OF_CUSTODY_CARD_LANDS {
        lands = lands
            + centered_cube(
                format!("gowning_shedding_chain_of_custody_card_land_{i}"),
                72.0,
                30.0,
                5.0,
            )
            .translate(
                -TRACE_PANEL_X / 2.0 + 54.0,
                -48.0 + i as f64 * 44.0,
                TRACE_PANEL_Z + 2.5,
            );
    }
    lands
}

fn scanner_datum_frame() -> Part {
    let frame_top = centered_cube(
        "gowning_shedding_scanner_datum_frame_top",
        TRACE_PANEL_X - 36.0,
        7.0,
        9.0,
    )
    .translate(0.0, TRACE_PANEL_Y / 2.0 - 17.0, TRACE_PANEL_Z + 4.5);
    let frame_bottom = centered_cube(
        "gowning_shedding_scanner_datum_frame_bottom",
        TRACE_PANEL_X - 36.0,
        7.0,
        9.0,
    )
    .translate(0.0, -TRACE_PANEL_Y / 2.0 + 17.0, TRACE_PANEL_Z + 4.5);
    let frame_left = centered_cube(
        "gowning_shedding_scanner_datum_frame_left",
        7.0,
        TRACE_PANEL_Y - 42.0,
        9.0,
    )
    .translate(-TRACE_PANEL_X / 2.0 + 17.0, 0.0, TRACE_PANEL_Z + 4.5);
    let frame_right = centered_cube(
        "gowning_shedding_scanner_datum_frame_right",
        7.0,
        TRACE_PANEL_Y - 42.0,
        9.0,
    )
    .translate(TRACE_PANEL_X / 2.0 - 17.0, 0.0, TRACE_PANEL_Z + 4.5);

    frame_top + frame_bottom + frame_left + frame_right
}

fn release_hold_reject_lanes() -> Part {
    let panel = centered_cube(
        "gowning_shedding_release_hold_reject_lane_panel",
        STATUS_X,
        STATUS_Y,
        STATUS_Z,
    )
    .translate(0.0, 0.0, STATUS_Z / 2.0);
    let slots = status_slot_cuts();
    panel - slots + status_lane_dividers() + status_gate_tabs() + status_evidence_token_lands()
}

fn status_slot_cuts() -> Part {
    let mut slots = Part::empty("gowning_shedding_release_hold_reject_slot_cuts");
    for lane in 0..DISPOSITION_LANES {
        let y = centered_index(lane, DISPOSITION_LANES, STATUS_LANE_PITCH_Y);
        for slot in 0..STATUS_SLOTS_PER_LANE {
            let x = centered_index(slot, STATUS_SLOTS_PER_LANE, 78.0);
            slots = slots
                + centered_cube(
                    format!("gowning_shedding_status_lane_{lane}_slot_{slot}"),
                    STATUS_SLOT_X,
                    STATUS_SLOT_Y,
                    24.0,
                )
                .translate(x, y, STATUS_Z - 8.0);
        }
    }
    slots
}

fn status_lane_dividers() -> Part {
    let mut dividers = Part::empty("gowning_shedding_status_lane_dividers");
    for i in 0..(DISPOSITION_LANES + 1) {
        let y = -STATUS_LANE_PITCH_Y * 1.5 + i as f64 * STATUS_LANE_PITCH_Y;
        dividers = dividers
            + centered_cube(
                format!("gowning_shedding_status_lane_boundary_{i}"),
                STATUS_X - 44.0,
                7.0,
                16.0,
            )
            .translate(0.0, y, STATUS_Z + 8.0);
    }
    dividers
}

fn status_gate_tabs() -> Part {
    let release = centered_cube(
        "gowning_shedding_release_lane_green_gate_tab_geometry",
        52.0,
        24.0,
        20.0,
    )
    .translate(-STATUS_X / 2.0 + 38.0, STATUS_LANE_PITCH_Y, STATUS_Z + 10.0);
    let hold = centered_cube(
        "gowning_shedding_hold_lane_yellow_gate_tab_geometry",
        52.0,
        24.0,
        20.0,
    )
    .translate(-STATUS_X / 2.0 + 38.0, 0.0, STATUS_Z + 10.0);
    let reject = centered_cube(
        "gowning_shedding_reject_lane_red_gate_tab_geometry",
        52.0,
        24.0,
        20.0,
    )
    .translate(
        -STATUS_X / 2.0 + 38.0,
        -STATUS_LANE_PITCH_Y,
        STATUS_Z + 10.0,
    );

    release + hold + reject
}

fn status_evidence_token_lands() -> Part {
    let mut lands = Part::empty("gowning_shedding_status_evidence_token_lands");
    for lane in 0..DISPOSITION_LANES {
        let y = centered_index(lane, DISPOSITION_LANES, STATUS_LANE_PITCH_Y);
        lands = lands
            + centered_cube(
                format!("gowning_shedding_status_lane_{lane}_run_record_token_land"),
                62.0,
                28.0,
                5.0,
            )
            .translate(STATUS_X / 2.0 - 48.0, y, STATUS_Z + 2.5);
    }
    lands
}

fn airflow_direction_arrows() -> Part {
    let mut arrows = Part::empty("gowning_shedding_geometric_airflow_direction_arrows");

    for (i, x) in [-330.0, -220.0, -110.0, 0.0, 110.0, 220.0, 330.0]
        .iter()
        .enumerate()
    {
        arrows = arrows
            + flow_arrow(format!("gowning_shedding_top_deck_airflow_arrow_{i}"), 74.0)
                .rotate(0.0, 0.0, -90.0)
                .translate(*x, 210.0, DECK_Z + 8.0);
    }

    arrows = arrows
        + flow_arrow(
            "gowning_shedding_funnel_capture_airflow_arrow".to_string(),
            96.0,
        )
        .rotate(0.0, 0.0, -90.0)
        .translate(-FUNNEL_X / 2.0 + 78.0, -62.0, DECK_Z + 8.0)
        + flow_arrow(
            "gowning_shedding_cassette_exhaust_airflow_arrow".to_string(),
            88.0,
        )
        .translate(154.0, -196.0, DECK_Z + 8.0);

    arrows
}

fn flow_arrow(name: String, length: f64) -> Part {
    let shaft = centered_cube(format!("{name}_shaft"), length, 8.0, 7.0).translate(
        length / 2.0 - 10.0,
        0.0,
        0.0,
    );
    let head = centered_cube(format!("{name}_head"), 24.0, 24.0, 7.0)
        .rotate(0.0, 0.0, 45.0)
        .translate(length - 10.0, 0.0, 0.0);
    shaft + head
}

fn evidence_bridge() -> Part {
    bridge_posts() + bridge_beams() + camera_pods() + led_bars() + evidence_card_scan_bridge()
}

fn bridge_posts() -> Part {
    let mut posts = Part::empty("gowning_shedding_evidence_bridge_posts");
    for (i, (x, y)) in [
        (-BRIDGE_X / 2.0, -BRIDGE_Y / 2.0),
        (BRIDGE_X / 2.0, -BRIDGE_Y / 2.0),
        (-BRIDGE_X / 2.0, BRIDGE_Y / 2.0),
        (BRIDGE_X / 2.0, BRIDGE_Y / 2.0),
    ]
    .iter()
    .enumerate()
    {
        let post = centered_cube(
            format!("gowning_shedding_evidence_bridge_post_{i}"),
            28.0,
            28.0,
            BRIDGE_UNDERSIDE_Z,
        )
        .translate(*x, *y, BRIDGE_UNDERSIDE_Z / 2.0);
        let foot = centered_cube(
            format!("gowning_shedding_evidence_bridge_foot_plate_{i}"),
            70.0,
            54.0,
            10.0,
        )
        .translate(*x, *y, 5.0);
        posts = posts + post + foot;
    }
    posts
}

fn bridge_beams() -> Part {
    let front = centered_cube(
        "gowning_shedding_evidence_bridge_front_beam",
        BRIDGE_X + 58.0,
        30.0,
        BRIDGE_BEAM_Z,
    )
    .translate(
        0.0,
        -BRIDGE_Y / 2.0,
        BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0,
    );
    let rear = centered_cube(
        "gowning_shedding_evidence_bridge_rear_beam",
        BRIDGE_X + 58.0,
        30.0,
        BRIDGE_BEAM_Z,
    )
    .translate(
        0.0,
        BRIDGE_Y / 2.0,
        BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0,
    );
    let left = centered_cube(
        "gowning_shedding_evidence_bridge_left_cross_beam",
        30.0,
        BRIDGE_Y + 58.0,
        BRIDGE_BEAM_Z,
    )
    .translate(
        -BRIDGE_X / 2.0,
        0.0,
        BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0,
    );
    let right = centered_cube(
        "gowning_shedding_evidence_bridge_right_cross_beam",
        30.0,
        BRIDGE_Y + 58.0,
        BRIDGE_BEAM_Z,
    )
    .translate(
        BRIDGE_X / 2.0,
        0.0,
        BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0,
    );
    let cable_tray = centered_cube(
        "gowning_shedding_evidence_bridge_camera_cable_tray",
        BRIDGE_X - 160.0,
        22.0,
        18.0,
    )
    .translate(0.0, BRIDGE_Y / 2.0 + 34.0, BRIDGE_UNDERSIDE_Z + 20.0);

    front + rear + left + right + cable_tray
}

fn camera_pods() -> Part {
    let mut pods = Part::empty("gowning_shedding_evidence_bridge_camera_pods");
    for i in 0..CAMERA_PODS {
        let x = centered_index(i, CAMERA_PODS, 210.0);
        let pod = centered_cube(
            format!("gowning_shedding_evidence_camera_pod_{i}"),
            76.0,
            54.0,
            40.0,
        )
        .translate(x, -18.0, BRIDGE_UNDERSIDE_Z - 20.0);
        let lens = centered_cylinder(
            format!("gowning_shedding_evidence_camera_lens_clearance_{i}"),
            12.0,
            20.0,
            32,
        )
        .translate(x, -18.0, BRIDGE_UNDERSIDE_Z - 44.0);
        pods = pods + (pod - lens);
    }
    pods
}

fn led_bars() -> Part {
    let front = centered_cube(
        "gowning_shedding_evidence_bridge_front_led_bar",
        BRIDGE_X - 220.0,
        16.0,
        16.0,
    )
    .translate(0.0, -BRIDGE_Y / 2.0 + 46.0, BRIDGE_UNDERSIDE_Z - 18.0);
    let rear = centered_cube(
        "gowning_shedding_evidence_bridge_rear_led_bar",
        BRIDGE_X - 220.0,
        16.0,
        16.0,
    )
    .translate(0.0, BRIDGE_Y / 2.0 - 46.0, BRIDGE_UNDERSIDE_Z - 18.0);
    front + rear
}

fn evidence_card_scan_bridge() -> Part {
    let arm = centered_cube(
        "gowning_shedding_certificate_evidence_scan_arm",
        26.0,
        210.0,
        24.0,
    )
    .translate(BRIDGE_X / 2.0 - 122.0, -10.0, BRIDGE_UNDERSIDE_Z - 32.0);
    let scan_pod = centered_cube(
        "gowning_shedding_certificate_evidence_scan_pod",
        110.0,
        78.0,
        36.0,
    )
    .translate(BRIDGE_X / 2.0 - 122.0, -122.0, BRIDGE_UNDERSIDE_Z - 48.0);
    let sight_cut = centered_cube(
        "gowning_shedding_certificate_scan_window_cut",
        70.0,
        48.0,
        12.0,
    )
    .translate(BRIDGE_X / 2.0 - 122.0, -122.0, BRIDGE_UNDERSIDE_Z - 56.0);

    arm + (scan_pod - sight_cut)
}

fn robot_service_keepouts() -> Part {
    let front = centered_cube(
        "gowning_shedding_front_robot_sweep_keepout_gauge",
        DECK_X - 220.0,
        FRONT_ROBOT_KEEP_OUT,
        8.0,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + FRONT_ROBOT_KEEP_OUT / 2.0,
        DECK_Z + 4.0,
    );
    let rear = centered_cube(
        "gowning_shedding_rear_air_duct_filter_service_keepout_gauge",
        DECK_X - 260.0,
        REAR_DUCT_SERVICE_KEEP_OUT,
        8.0,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - REAR_DUCT_SERVICE_KEEP_OUT / 2.0,
        DECK_Z + 4.0,
    );
    let left = centered_cube(
        "gowning_shedding_left_coupon_exchange_keepout_gauge",
        LEFT_COUPON_SERVICE_KEEP_OUT,
        DECK_Y - 210.0,
        8.0,
    )
    .translate(
        -DECK_X / 2.0 + LEFT_COUPON_SERVICE_KEEP_OUT / 2.0,
        0.0,
        DECK_Z + 4.0,
    );
    let right = centered_cube(
        "gowning_shedding_right_certificate_service_keepout_gauge",
        RIGHT_CERT_SERVICE_KEEP_OUT,
        DECK_Y - 210.0,
        8.0,
    )
    .translate(
        DECK_X / 2.0 - RIGHT_CERT_SERVICE_KEEP_OUT / 2.0,
        0.0,
        DECK_Z + 4.0,
    );

    front + rear + left + right + keepout_height_gauges()
}

fn keepout_height_gauges() -> Part {
    let mut gauges = Part::empty("gowning_shedding_robot_service_keepout_height_gauges");
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 104.0, -DECK_Y / 2.0 + 104.0),
        (DECK_X / 2.0 - 104.0, -DECK_Y / 2.0 + 104.0),
        (-DECK_X / 2.0 + 104.0, DECK_Y / 2.0 - 104.0),
        (DECK_X / 2.0 - 104.0, DECK_Y / 2.0 - 104.0),
    ]
    .iter()
    .enumerate()
    {
        let post = centered_cube(
            format!("gowning_shedding_keepout_z_clearance_gauge_post_{i}"),
            14.0,
            14.0,
            ROBOT_Z_CLEARANCE,
        )
        .translate(*x, *y, DECK_Z + ROBOT_Z_CLEARANCE / 2.0);
        let flag = centered_cube(
            format!("gowning_shedding_keepout_z_clearance_flag_{i}"),
            74.0,
            12.0,
            18.0,
        )
        .translate(*x, *y, DECK_Z + ROBOT_Z_CLEARANCE);
        gauges = gauges + post + flag;
    }
    gauges
}
