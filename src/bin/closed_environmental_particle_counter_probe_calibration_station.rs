use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed environmental particle counter / scan-probe calibration station.
//
// Research assumptions captured in geometry:
// - A walk-in sterile support pod or isolator support cell should keep the
//   particle counter inlet, HEPA/ULPA scan probe, challenge coupons, and used
//   calibration materials in a closed, wipeable station rather than loose
//   benchtop staging.
// - Reference particles are represented as sealed challenge coupons, surrogate
//   aerosol target lands, and zero/filter caps. Purchased PSL aerosols,
//   traceable particle counters, zero filters, and calibration certificates
//   remain external process-controlled items.
// - HEPA/ULPA scan geometry needs repeatable probe axis alignment, indexed
//   scan lanes, standoff gauges, and service bulkheads for aerosol/return
//   tubing without opening the clean work volume.
//
// Product concept CAD only. This does not define particle acceptance limits,
// aerosol generation protocols, metrological traceability, or release criteria.

const OUTPUTS: [&str; 13] = [
    "output/closed_environmental_particle_counter_probe_calibration_station_base_leak_tray.stl",
    "output/closed_environmental_particle_counter_probe_calibration_station_clean_probe_parking_cradle.stl",
    "output/closed_environmental_particle_counter_probe_calibration_station_aerosol_challenge_coupon_holder.stl",
    "output/closed_environmental_particle_counter_probe_calibration_station_surrogate_target_lands.stl",
    "output/closed_environmental_particle_counter_probe_calibration_station_hepa_ulpa_scan_alignment_frame.stl",
    "output/closed_environmental_particle_counter_probe_calibration_station_particle_counter_inlet_bulkhead_panel.stl",
    "output/closed_environmental_particle_counter_probe_calibration_station_barcode_certificate_lands.stl",
    "output/closed_environmental_particle_counter_probe_calibration_station_released_hold_reject_lanes.stl",
    "output/closed_environmental_particle_counter_probe_calibration_station_clean_used_coupon_segregation_tray.stl",
    "output/closed_environmental_particle_counter_probe_calibration_station_waste_used_coupon_bin.stl",
    "output/closed_environmental_particle_counter_probe_calibration_station_service_bulkhead.stl",
    "output/closed_environmental_particle_counter_probe_calibration_station_robot_keepout_gauge.stl",
    "output/closed_environmental_particle_counter_probe_calibration_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "clean_probe_parking_cradle",
    "reference_particle_aerosol_challenge_coupon_holder",
    "surrogate_target_lands",
    "hepa_ulpa_scan_probe_alignment_geometry",
    "particle_counter_inlet_bulkhead",
    "barcode_certificate_lands",
    "released_hold_reject_lanes",
    "clean_used_coupon_segregation",
    "waste_used_coupon_bin",
    "robot_keepouts",
    "service_bulkhead",
];

const DECK_X: f64 = 1320.0;
const DECK_Y: f64 = 820.0;
const DECK_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const SOCKET_DEPTH: f64 = 6.0;

const PROBE_CRADLE_X: f64 = 330.0;
const PROBE_CRADLE_Y: f64 = 160.0;
const PROBE_CRADLE_Z: f64 = 44.0;
const PROBE_POS: (f64, f64) = (-440.0, 225.0);
const PROBE_DOCK_COUNT: usize = 4;
const PROBE_DOCK_PITCH: f64 = 68.0;
const PROBE_SLEEVE_D: f64 = 18.0;

const COUPON_HOLDER_X: f64 = 390.0;
const COUPON_HOLDER_Y: f64 = 170.0;
const COUPON_HOLDER_Z: f64 = 38.0;
const COUPON_POS: (f64, f64) = (-45.0, 225.0);
const CHALLENGE_COUPON_COUNT: usize = 6;
const CHALLENGE_COUPON_PITCH: f64 = 55.0;
const CHALLENGE_COUPON_X: f64 = 40.0;
const CHALLENGE_COUPON_Y: f64 = 72.0;
const AEROSOL_PUCK_COUNT: usize = 4;

const TARGET_LAND_X: f64 = 330.0;
const TARGET_LAND_Y: f64 = 150.0;
const TARGET_LAND_Z: f64 = 26.0;
const TARGET_POS: (f64, f64) = (385.0, 225.0);
const SURROGATE_TARGET_COUNT: usize = 5;
const SURROGATE_TARGET_PITCH: f64 = 58.0;

const SCAN_FRAME_X: f64 = 430.0;
const SCAN_FRAME_Y: f64 = 250.0;
const SCAN_FRAME_Z: f64 = 122.0;
const SCAN_POS: (f64, f64) = (-385.0, -20.0);
const SCAN_LANE_COUNT: usize = 7;
const SCAN_LANE_PITCH: f64 = 48.0;
const SCAN_RAIL_Z: f64 = 24.0;
const PROBE_STANDOFF_MM: f64 = 25.0;
const SCAN_PROBE_SOCKET_D: f64 = 24.0;

const INLET_PANEL_X: f64 = 360.0;
const INLET_PANEL_Y: f64 = 210.0;
const INLET_PANEL_Z: f64 = 72.0;
const INLET_POS: (f64, f64) = (70.0, -20.0);
const PARTICLE_PORT_COUNT: usize = 5;
const PARTICLE_PORT_PITCH: f64 = 58.0;
const SAMPLE_TUBE_OD: f64 = 6.35;
const SAMPLE_TUBE_CLEARANCE_D: f64 = SAMPLE_TUBE_OD + 1.2;

const TRACE_PANEL_X: f64 = 270.0;
const TRACE_PANEL_Y: f64 = 210.0;
const TRACE_PANEL_Z: f64 = 12.0;
const TRACE_POS: (f64, f64) = (430.0, -20.0);
const BARCODE_LANDS: usize = 8;
const CERTIFICATE_LANDS: usize = 3;

const STATUS_X: f64 = 390.0;
const STATUS_Y: f64 = 220.0;
const STATUS_Z: f64 = 44.0;
const STATUS_POS: (f64, f64) = (360.0, -250.0);
const STATUS_LANES: usize = 3;
const STATUS_SLOTS_PER_LANE: usize = 4;
const STATUS_SLOT_X: f64 = 88.0;
const STATUS_SLOT_Y: f64 = 34.0;

const SEG_TRAY_X: f64 = 300.0;
const SEG_TRAY_Y: f64 = 190.0;
const SEG_TRAY_Z: f64 = 40.0;
const SEG_POS: (f64, f64) = (-105.0, -250.0);
const SEG_WELLS_PER_SIDE: usize = 5;
const CLEAN_USED_DIVIDER_Z: f64 = 72.0;

const WASTE_BIN_X: f64 = 300.0;
const WASTE_BIN_Y: f64 = 190.0;
const WASTE_BIN_Z: f64 = 108.0;
const WASTE_POS: (f64, f64) = (-445.0, -250.0);
const WASTE_COUPON_SLOTS: usize = 8;
const WASTE_CHUTE_X: f64 = 116.0;
const WASTE_CHUTE_Y: f64 = 34.0;

const SERVICE_BULKHEAD_X: f64 = 1120.0;
const SERVICE_BULKHEAD_Y: f64 = 54.0;
const SERVICE_BULKHEAD_Z: f64 = 138.0;
const SERVICE_POS: (f64, f64) = (0.0, 352.0);
const SERVICE_PORTS: usize = 10;
const SERVICE_PORT_PITCH: f64 = 96.0;

const KEEP_OUT_X: f64 = 1210.0;
const KEEP_OUT_Y: f64 = 700.0;
const KEEP_OUT_Z: f64 = 116.0;
const ROBOT_FRONT_APPROACH_MM: f64 = 430.0;
const SERVICE_REAR_CLEARANCE_MM: f64 = 280.0;
const SIDE_COUPON_LOAD_CLEARANCE_MM: f64 = 240.0;
const ROBOT_Z_CLEARANCE_MM: f64 = 340.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside(self, deck_x: f64, deck_y: f64) -> bool {
        let usable_x = deck_x / 2.0 - RIM_W - 12.0;
        let usable_y = deck_y / 2.0 - RIM_W - 12.0;

        self.center.0.abs() + self.x / 2.0 <= usable_x
            && self.center.1.abs() + self.y / 2.0 <= usable_y
    }

    fn overlaps(self, other: Rect) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();

        dx < (self.x + other.x) / 2.0 && dy < (self.y + other.y) / 2.0
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout_constraints();

    let base = base_leak_tray();
    export(OUTPUTS[0], &base);

    let probe_cradle = clean_probe_parking_cradle();
    export(OUTPUTS[1], &probe_cradle);

    let coupon_holder = aerosol_challenge_coupon_holder();
    export(OUTPUTS[2], &coupon_holder);

    let target_lands = surrogate_target_lands();
    export(OUTPUTS[3], &target_lands);

    let scan_frame = hepa_ulpa_scan_alignment_frame();
    export(OUTPUTS[4], &scan_frame);

    let inlet_panel = particle_counter_inlet_bulkhead_panel();
    export(OUTPUTS[5], &inlet_panel);

    let traceability = barcode_certificate_lands();
    export(OUTPUTS[6], &traceability);

    let status_lanes = released_hold_reject_lanes();
    export(OUTPUTS[7], &status_lanes);

    let segregation = clean_used_coupon_segregation_tray();
    export(OUTPUTS[8], &segregation);

    let waste = waste_used_coupon_bin();
    export(OUTPUTS[9], &waste);

    let service = service_bulkhead();
    export(OUTPUTS[10], &service);

    let keepouts = robot_keepout_gauge();
    export(OUTPUTS[11], &keepouts);

    let assembly = base
        + probe_cradle.translate(PROBE_POS.0, PROBE_POS.1, insert_z(PROBE_CRADLE_Z))
        + coupon_holder.translate(COUPON_POS.0, COUPON_POS.1, insert_z(COUPON_HOLDER_Z))
        + target_lands.translate(TARGET_POS.0, TARGET_POS.1, insert_z(TARGET_LAND_Z))
        + scan_frame.translate(SCAN_POS.0, SCAN_POS.1, insert_z(SCAN_FRAME_Z))
        + inlet_panel.translate(INLET_POS.0, INLET_POS.1, insert_z(INLET_PANEL_Z))
        + traceability.translate(TRACE_POS.0, TRACE_POS.1, insert_z(TRACE_PANEL_Z))
        + status_lanes.translate(STATUS_POS.0, STATUS_POS.1, insert_z(STATUS_Z))
        + segregation.translate(SEG_POS.0, SEG_POS.1, insert_z(SEG_TRAY_Z))
        + waste.translate(WASTE_POS.0, WASTE_POS.1, insert_z(WASTE_BIN_Z))
        + service.translate(SERVICE_POS.0, SERVICE_POS.1, insert_z(SERVICE_BULKHEAD_Z))
        + keepouts;
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed environmental particle counter / probe calibration station:");
    println!("  Footprint:                  {DECK_X:.0}mm x {DECK_Y:.0}mm closed leak-tray deck");
    println!(
        "  Probe handling:             {PROBE_DOCK_COUNT} clean parking sleeves plus indexed HEPA/ULPA scan frame with {SCAN_LANE_COUNT} lanes and {PROBE_STANDOFF_MM:.0}mm standoff gauge"
    );
    println!(
        "  Reference challenge:        {CHALLENGE_COUPON_COUNT} sealed particle/aerosol challenge coupon pockets, {AEROSOL_PUCK_COUNT} aerosol puck wells, and {SURROGATE_TARGET_COUNT} surrogate target lands"
    );
    println!(
        "  Closed interfaces:          {PARTICLE_PORT_COUNT} particle counter inlet/zero/return ports, {SERVICE_PORTS} rear service bulkhead ports, and {SAMPLE_TUBE_OD:.2}mm OD sample tube assumption"
    );
    println!(
        "  Traceability/disposition:   {BARCODE_LANDS} barcode lands, {CERTIFICATE_LANDS} certificate lands, {STATUS_LANES} released/hold/reject lanes with {STATUS_SLOTS_PER_LANE} slots each"
    );
    println!(
        "  Segregation/keepouts:       clean-used coupon tray, {WASTE_COUPON_SLOTS} waste coupon slots, {ROBOT_FRONT_APPROACH_MM:.0}mm robot approach, {SERVICE_REAR_CLEARANCE_MM:.0}mm rear service, {SIDE_COUPON_LOAD_CLEARANCE_MM:.0}mm side load, {ROBOT_Z_CLEARANCE_MM:.0}mm Z clearance"
    );
    println!("  Feature groups covered:     {}", REQUIRED_FEATURES.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn insert_z(height: f64) -> f64 {
    DECK_Z / 2.0 + height / 2.0 - SOCKET_DEPTH / 2.0
}

fn assert_layout_constraints() {
    let rects = layout_rects();
    for rect in rects {
        assert!(
            rect.fits_inside(DECK_X, DECK_Y),
            "{} exceeds station envelope",
            rect.name
        );
    }

    for (a, b) in non_overlap_pairs() {
        assert!(!a.overlaps(b), "{} overlaps {}", a.name, b.name);
    }

    assert!(PROBE_STANDOFF_MM >= 25.0);
    assert!(SAMPLE_TUBE_CLEARANCE_D > SAMPLE_TUBE_OD);
    assert_eq!(STATUS_LANES, 3);
    assert!(ROBOT_Z_CLEARANCE_MM > SERVICE_BULKHEAD_Z + DECK_Z);
    assert!(CLEAN_USED_DIVIDER_Z > SEG_TRAY_Z);
}

fn layout_rects() -> [Rect; 10] {
    [
        rect(
            "clean_probe_parking_cradle",
            PROBE_POS,
            PROBE_CRADLE_X,
            PROBE_CRADLE_Y,
        ),
        rect(
            "aerosol_challenge_coupon_holder",
            COUPON_POS,
            COUPON_HOLDER_X,
            COUPON_HOLDER_Y,
        ),
        rect(
            "surrogate_target_lands",
            TARGET_POS,
            TARGET_LAND_X,
            TARGET_LAND_Y,
        ),
        rect(
            "hepa_ulpa_scan_alignment_frame",
            SCAN_POS,
            SCAN_FRAME_X,
            SCAN_FRAME_Y,
        ),
        rect(
            "particle_counter_inlet_bulkhead_panel",
            INLET_POS,
            INLET_PANEL_X,
            INLET_PANEL_Y,
        ),
        rect(
            "barcode_certificate_lands",
            TRACE_POS,
            TRACE_PANEL_X,
            TRACE_PANEL_Y,
        ),
        rect("released_hold_reject_lanes", STATUS_POS, STATUS_X, STATUS_Y),
        rect(
            "clean_used_coupon_segregation_tray",
            SEG_POS,
            SEG_TRAY_X,
            SEG_TRAY_Y,
        ),
        rect("waste_used_coupon_bin", WASTE_POS, WASTE_BIN_X, WASTE_BIN_Y),
        rect(
            "service_bulkhead",
            SERVICE_POS,
            SERVICE_BULKHEAD_X,
            SERVICE_BULKHEAD_Y,
        ),
    ]
}

fn non_overlap_pairs() -> [(Rect, Rect); 11] {
    let rects = layout_rects();
    [
        (rects[0], rects[1]),
        (rects[1], rects[2]),
        (rects[0], rects[3]),
        (rects[1], rects[4]),
        (rects[2], rects[5]),
        (rects[3], rects[4]),
        (rects[4], rects[5]),
        (rects[6], rects[7]),
        (rects[7], rects[8]),
        (rects[0], rects[9]),
        (rects[2], rects[9]),
    ]
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn base_leak_tray() -> Part {
    let deck = centered_cube(
        "particle_counter_probe_station_base_closed_leak_tray_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let leak_pan = centered_cube(
        "particle_counter_probe_station_base_recessed_wipe_leak_pan",
        DECK_X - 2.0 * (RIM_W + 46.0),
        DECK_Y - 2.0 * (RIM_W + 54.0),
        8.0,
    )
    .translate(0.0, -16.0, DECK_Z - 4.0);
    let front_drain = centered_cube(
        "particle_counter_probe_station_base_front_waste_drain_channel",
        DECK_X - 190.0,
        22.0,
        8.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 62.0, DECK_Z - 4.0);
    let drain_port = centered_cylinder(
        "particle_counter_probe_station_base_closed_drain_port",
        8.0,
        44.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 86.0, -DECK_Y / 2.0 + 34.0, DECK_Z - 8.0);

    deck - leak_pan - front_drain - drain_port - insert_sockets() - deck_mount_holes()
        + perimeter_rim()
        + zone_spines()
        + robot_datum_targets()
        + closed_transfer_lips()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("particle_counter_probe_station_insert_sockets");
    for rect in layout_rects().iter().take(9) {
        sockets = sockets
            + centered_cube(
                format!("particle_counter_probe_station_socket_{}", rect.name),
                rect.x + 12.0,
                rect.y + 12.0,
                SOCKET_DEPTH,
            )
            .translate(rect.center.0, rect.center.1, DECK_Z - SOCKET_DEPTH / 2.0);
    }
    sockets
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("particle_counter_probe_station_m6_mount_holes");
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 56.0, -DECK_Y / 2.0 + 56.0),
        (DECK_X / 2.0 - 56.0, -DECK_Y / 2.0 + 56.0),
        (-DECK_X / 2.0 + 56.0, DECK_Y / 2.0 - 56.0),
        (DECK_X / 2.0 - 56.0, DECK_Y / 2.0 - 56.0),
        (0.0, -DECK_Y / 2.0 + 56.0),
        (0.0, DECK_Y / 2.0 - 56.0),
        (-DECK_X / 2.0 + 56.0, 0.0),
        (DECK_X / 2.0 - 56.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("particle_counter_probe_station_m6_mount_clearance_{i}"),
                3.4,
                DECK_Z + 4.0,
                28,
            )
            .translate(*x, *y, DECK_Z / 2.0);
    }
    holes
}

fn perimeter_rim() -> Part {
    let front = centered_cube(
        "particle_counter_probe_station_front_spill_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        "particle_counter_probe_station_rear_service_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "particle_counter_probe_station_left_spill_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "particle_counter_probe_station_right_spill_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);

    front + rear + left + right
}

fn zone_spines() -> Part {
    let clean_dirty_spine = centered_cube(
        "particle_counter_probe_station_clean_dirty_center_spine",
        DECK_X - 160.0,
        12.0,
        28.0,
    )
    .translate(0.0, 108.0, DECK_Z + 14.0);
    let process_status_spine = centered_cube(
        "particle_counter_probe_station_process_status_spine",
        DECK_X - 180.0,
        10.0,
        24.0,
    )
    .translate(0.0, -136.0, DECK_Z + 12.0);
    let coupon_waste_spine = centered_cube(
        "particle_counter_probe_station_coupon_waste_vertical_spine",
        12.0,
        226.0,
        28.0,
    )
    .translate(-278.0, -250.0, DECK_Z + 14.0);

    clean_dirty_spine + process_status_spine + coupon_waste_spine
}

fn robot_datum_targets() -> Part {
    let mut targets = Part::empty("particle_counter_probe_station_robot_datum_targets");
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 92.0, -DECK_Y / 2.0 + 92.0),
        (DECK_X / 2.0 - 92.0, -DECK_Y / 2.0 + 92.0),
        (-DECK_X / 2.0 + 92.0, DECK_Y / 2.0 - 92.0),
        (DECK_X / 2.0 - 92.0, DECK_Y / 2.0 - 92.0),
    ]
    .iter()
    .enumerate()
    {
        targets = targets
            + fiducial_disc(&format!("particle_counter_probe_station_base_fiducial_{i}"))
                .translate(*x, *y, DECK_Z + 2.0);
    }
    targets
}

fn closed_transfer_lips() -> Part {
    let clean_lip = centered_cube(
        "particle_counter_probe_station_clean_probe_transfer_lip",
        280.0,
        12.0,
        18.0,
    )
    .translate(PROBE_POS.0, PROBE_POS.1 - 102.0, DECK_Z + 9.0);
    let coupon_lip = centered_cube(
        "particle_counter_probe_station_coupon_transfer_lip",
        348.0,
        12.0,
        18.0,
    )
    .translate(COUPON_POS.0, COUPON_POS.1 - 108.0, DECK_Z + 9.0);
    let waste_lip = centered_cube(
        "particle_counter_probe_station_used_coupon_transfer_lip",
        270.0,
        12.0,
        20.0,
    )
    .translate(WASTE_POS.0, WASTE_POS.1 + 112.0, DECK_Z + 10.0);

    clean_lip + coupon_lip + waste_lip
}

fn clean_probe_parking_cradle() -> Part {
    let body = centered_cube(
        "particle_counter_probe_clean_parking_cradle_body",
        PROBE_CRADLE_X,
        PROBE_CRADLE_Y,
        PROBE_CRADLE_Z,
    );
    let rear_fence = centered_cube(
        "particle_counter_probe_clean_parking_cradle_rear_fence",
        PROBE_CRADLE_X,
        14.0,
        PROBE_CRADLE_Z + 38.0,
    )
    .translate(0.0, PROBE_CRADLE_Y / 2.0 - 7.0, 19.0);
    let front_soft_stop = centered_cube(
        "particle_counter_probe_clean_parking_cradle_front_robot_soft_stop",
        PROBE_CRADLE_X - 38.0,
        10.0,
        24.0,
    )
    .translate(
        0.0,
        -PROBE_CRADLE_Y / 2.0 + 12.0,
        PROBE_CRADLE_Z / 2.0 + 12.0,
    );
    let sleeve_cuts = probe_sleeve_cuts();
    let nose_stops = probe_nose_stops();
    let cap_pockets = clean_zero_cap_pockets();
    let cable_comb = probe_cable_comb();

    body + rear_fence + front_soft_stop + nose_stops + cap_pockets + cable_comb - sleeve_cuts
}

fn probe_sleeve_cuts() -> Part {
    let mut cuts = Part::empty("particle_counter_probe_clean_parking_sleeve_cuts");
    for i in 0..PROBE_DOCK_COUNT {
        let x = centered_index(i, PROBE_DOCK_COUNT, PROBE_DOCK_PITCH);
        let radius = match i {
            0 => SCAN_PROBE_SOCKET_D / 2.0,
            1 => 11.0,
            2 => PROBE_SLEEVE_D / 2.0,
            _ => 8.0,
        };
        let sleeve = centered_cylinder(
            format!("particle_counter_probe_clean_parking_sleeve_{i}"),
            radius,
            PROBE_CRADLE_Y + 12.0,
            36,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -5.0, 5.0);
        let top_access = centered_cube(
            format!("particle_counter_probe_clean_parking_top_access_slot_{i}"),
            18.0,
            PROBE_CRADLE_Y - 34.0,
            18.0,
        )
        .translate(x, -10.0, PROBE_CRADLE_Z / 2.0 - 5.0);
        cuts = cuts + sleeve + top_access;
    }
    cuts
}

fn probe_nose_stops() -> Part {
    let mut stops = Part::empty("particle_counter_probe_clean_parking_nose_stops");
    for i in 0..PROBE_DOCK_COUNT {
        let x = centered_index(i, PROBE_DOCK_COUNT, PROBE_DOCK_PITCH);
        stops = stops
            + centered_cube(
                format!("particle_counter_probe_clean_parking_nose_stop_{i}"),
                30.0,
                7.0,
                20.0,
            )
            .translate(x, PROBE_CRADLE_Y / 2.0 - 28.0, PROBE_CRADLE_Z / 2.0 + 10.0)
            + fiducial_disc(&format!("particle_counter_probe_parking_grip_fiducial_{i}"))
                .translate(x, -PROBE_CRADLE_Y / 2.0 + 28.0, PROBE_CRADLE_Z / 2.0 + 2.0);
    }
    stops
}

fn clean_zero_cap_pockets() -> Part {
    let mut pockets = Part::empty("particle_counter_probe_clean_zero_cap_pockets");
    for i in 0..3 {
        let x = -112.0 + i as f64 * 112.0;
        let pocket = centered_cylinder(
            format!("particle_counter_probe_zero_filter_cap_pocket_{i}"),
            18.0,
            10.0,
            36,
        )
        .translate(x, -PROBE_CRADLE_Y / 2.0 + 34.0, PROBE_CRADLE_Z / 2.0 + 5.0);
        let retainer = centered_cube(
            format!("particle_counter_probe_zero_filter_cap_retainer_tab_{i}"),
            46.0,
            6.0,
            12.0,
        )
        .translate(x, -PROBE_CRADLE_Y / 2.0 + 12.0, PROBE_CRADLE_Z / 2.0 + 12.0);
        pockets = pockets + pocket + retainer;
    }
    pockets
}

fn probe_cable_comb() -> Part {
    let comb = centered_cube(
        "particle_counter_probe_clean_parking_cable_comb_body",
        PROBE_CRADLE_X - 52.0,
        14.0,
        18.0,
    )
    .translate(0.0, -8.0, PROBE_CRADLE_Z / 2.0 + 22.0);
    let mut notches = Part::empty("particle_counter_probe_clean_parking_cable_comb_notches");
    for i in 0..PROBE_DOCK_COUNT {
        let x = centered_index(i, PROBE_DOCK_COUNT, PROBE_DOCK_PITCH);
        notches = notches
            + centered_cube(
                format!("particle_counter_probe_clean_parking_cable_notch_{i}"),
                16.0,
                18.0,
                20.0,
            )
            .translate(x, -8.0, PROBE_CRADLE_Z / 2.0 + 23.0);
    }
    comb - notches
}

fn aerosol_challenge_coupon_holder() -> Part {
    let body = centered_cube(
        "particle_counter_aerosol_challenge_coupon_holder_body",
        COUPON_HOLDER_X,
        COUPON_HOLDER_Y,
        COUPON_HOLDER_Z,
    );
    let inner_recess = centered_cube(
        "particle_counter_aerosol_challenge_coupon_holder_wipe_recess",
        COUPON_HOLDER_X - 40.0,
        COUPON_HOLDER_Y - 38.0,
        10.0,
    )
    .translate(0.0, 0.0, COUPON_HOLDER_Z / 2.0 - 4.0);
    let gasket_land = gasket_frame(
        "particle_counter_aerosol_challenge_coupon_holder_sealed_lid_gasket",
        COUPON_HOLDER_X - 24.0,
        COUPON_HOLDER_Y - 24.0,
        8.0,
        8.0,
    )
    .translate(0.0, 0.0, COUPON_HOLDER_Z / 2.0 + 5.0);
    let handle_shadow = centered_cube(
        "particle_counter_aerosol_challenge_coupon_holder_lift_handle_shadow",
        112.0,
        18.0,
        22.0,
    )
    .translate(
        0.0,
        -COUPON_HOLDER_Y / 2.0 - 10.0,
        COUPON_HOLDER_Z / 2.0 + 18.0,
    );

    body - inner_recess - challenge_coupon_slots() - aerosol_puck_wells()
        + gasket_land
        + challenge_coupon_clips()
        + challenge_flow_witness_bridge()
        + handle_shadow
}

fn challenge_coupon_slots() -> Part {
    let mut slots = Part::empty("particle_counter_aerosol_challenge_coupon_slots");
    for i in 0..CHALLENGE_COUPON_COUNT {
        let x = centered_index(i, CHALLENGE_COUPON_COUNT, CHALLENGE_COUPON_PITCH);
        slots = slots
            + centered_cube(
                format!("particle_counter_reference_particle_coupon_slot_{i}"),
                CHALLENGE_COUPON_X,
                CHALLENGE_COUPON_Y,
                COUPON_HOLDER_Z + 6.0,
            )
            .translate(x, 22.0, 0.0);
    }
    slots
}

fn aerosol_puck_wells() -> Part {
    let mut wells = Part::empty("particle_counter_aerosol_surrogate_puck_wells");
    for i in 0..AEROSOL_PUCK_COUNT {
        let x = centered_index(i, AEROSOL_PUCK_COUNT, 74.0);
        wells = wells
            + centered_cylinder(
                format!("particle_counter_aerosol_puck_well_{i}"),
                22.0,
                COUPON_HOLDER_Z + 4.0,
                44,
            )
            .translate(x, -50.0, 0.0);
    }
    wells
}

fn challenge_coupon_clips() -> Part {
    let mut clips = Part::empty("particle_counter_challenge_coupon_spring_clips");
    for i in 0..CHALLENGE_COUPON_COUNT {
        let x = centered_index(i, CHALLENGE_COUPON_COUNT, CHALLENGE_COUPON_PITCH);
        let left = centered_cube(
            format!("particle_counter_reference_particle_coupon_left_clip_{i}"),
            5.0,
            CHALLENGE_COUPON_Y + 18.0,
            14.0,
        )
        .translate(
            x - CHALLENGE_COUPON_X / 2.0 - 6.0,
            22.0,
            COUPON_HOLDER_Z / 2.0 + 7.0,
        );
        let right = centered_cube(
            format!("particle_counter_reference_particle_coupon_right_clip_{i}"),
            5.0,
            CHALLENGE_COUPON_Y + 18.0,
            14.0,
        )
        .translate(
            x + CHALLENGE_COUPON_X / 2.0 + 6.0,
            22.0,
            COUPON_HOLDER_Z / 2.0 + 7.0,
        );
        clips = clips + left + right;
    }
    clips
}

fn challenge_flow_witness_bridge() -> Part {
    let bridge = centered_cube(
        "particle_counter_challenge_flow_witness_bridge",
        COUPON_HOLDER_X - 62.0,
        20.0,
        24.0,
    )
    .translate(
        0.0,
        COUPON_HOLDER_Y / 2.0 - 34.0,
        COUPON_HOLDER_Z / 2.0 + 12.0,
    );
    let mut ports = Part::empty("particle_counter_challenge_flow_witness_bridge_ports");
    for i in 0..4 {
        let x = centered_index(i, 4, 78.0);
        ports = ports
            + centered_cylinder(
                format!("particle_counter_challenge_flow_witness_port_{i}"),
                5.0,
                26.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                x,
                COUPON_HOLDER_Y / 2.0 - 34.0,
                COUPON_HOLDER_Z / 2.0 + 12.0,
            );
    }
    bridge - ports
}

fn surrogate_target_lands() -> Part {
    let plate = centered_cube(
        "particle_counter_surrogate_target_land_plate",
        TARGET_LAND_X,
        TARGET_LAND_Y,
        TARGET_LAND_Z,
    );
    let recessed_field = centered_cube(
        "particle_counter_surrogate_target_clean_recess_field",
        TARGET_LAND_X - 36.0,
        TARGET_LAND_Y - 34.0,
        8.0,
    )
    .translate(0.0, 0.0, TARGET_LAND_Z / 2.0 - 3.0);

    plate - recessed_field
        + surrogate_target_discs()
        + surrogate_target_labels()
        + surrogate_target_retainers()
        + target_grip_fiducials()
}

fn surrogate_target_discs() -> Part {
    let mut targets = Part::empty("particle_counter_surrogate_target_reference_discs");
    for i in 0..SURROGATE_TARGET_COUNT {
        let x = centered_index(i, SURROGATE_TARGET_COUNT, SURROGATE_TARGET_PITCH);
        let radius = match i {
            0 => 14.0,
            1 => 18.0,
            2 => 12.0,
            3 => 16.0,
            _ => 20.0,
        };
        let disc = centered_cylinder(
            format!("particle_counter_surrogate_target_land_{i}"),
            radius,
            6.0,
            40,
        )
        .translate(x, 18.0, TARGET_LAND_Z / 2.0 + 3.0);
        let witness_ring = centered_cylinder(
            format!("particle_counter_surrogate_target_witness_ring_{i}"),
            radius + 5.0,
            3.0,
            40,
        )
        .translate(x, 18.0, TARGET_LAND_Z / 2.0 + 1.5);
        targets = targets + witness_ring + disc;
    }
    targets
}

fn surrogate_target_labels() -> Part {
    let mut labels = Part::empty("particle_counter_surrogate_target_label_lands");
    for i in 0..SURROGATE_TARGET_COUNT {
        let x = centered_index(i, SURROGATE_TARGET_COUNT, SURROGATE_TARGET_PITCH);
        labels = labels
            + centered_cube(
                format!("particle_counter_surrogate_target_label_land_{i}"),
                44.0,
                16.0,
                3.0,
            )
            .translate(x, -TARGET_LAND_Y / 2.0 + 32.0, TARGET_LAND_Z / 2.0 + 2.0);
    }
    labels
}

fn surrogate_target_retainers() -> Part {
    let front = centered_cube(
        "particle_counter_surrogate_target_front_retainer_bar",
        TARGET_LAND_X - 44.0,
        8.0,
        14.0,
    )
    .translate(0.0, -TARGET_LAND_Y / 2.0 + 12.0, TARGET_LAND_Z / 2.0 + 7.0);
    let rear = centered_cube(
        "particle_counter_surrogate_target_rear_retainer_bar",
        TARGET_LAND_X - 44.0,
        8.0,
        14.0,
    )
    .translate(0.0, TARGET_LAND_Y / 2.0 - 12.0, TARGET_LAND_Z / 2.0 + 7.0);

    front + rear
}

fn target_grip_fiducials() -> Part {
    fiducial_disc("particle_counter_surrogate_target_left_fiducial").translate(
        -TARGET_LAND_X / 2.0 + 30.0,
        -TARGET_LAND_Y / 2.0 + 28.0,
        TARGET_LAND_Z / 2.0 + 2.0,
    ) + fiducial_disc("particle_counter_surrogate_target_right_fiducial").translate(
        TARGET_LAND_X / 2.0 - 30.0,
        -TARGET_LAND_Y / 2.0 + 28.0,
        TARGET_LAND_Z / 2.0 + 2.0,
    )
}

fn hepa_ulpa_scan_alignment_frame() -> Part {
    let left_rail = centered_cube(
        "particle_counter_hepa_ulpa_alignment_left_scan_rail",
        SCAN_FRAME_X,
        18.0,
        SCAN_RAIL_Z,
    )
    .translate(0.0, -SCAN_FRAME_Y / 2.0 + 30.0, SCAN_FRAME_Z / 2.0);
    let right_rail = centered_cube(
        "particle_counter_hepa_ulpa_alignment_right_scan_rail",
        SCAN_FRAME_X,
        18.0,
        SCAN_RAIL_Z,
    )
    .translate(0.0, SCAN_FRAME_Y / 2.0 - 30.0, SCAN_FRAME_Z / 2.0);
    let front_cross = centered_cube(
        "particle_counter_hepa_ulpa_alignment_front_cross_stop",
        24.0,
        SCAN_FRAME_Y - 44.0,
        SCAN_RAIL_Z,
    )
    .translate(-SCAN_FRAME_X / 2.0 + 28.0, 0.0, SCAN_FRAME_Z / 2.0);
    let rear_cross = centered_cube(
        "particle_counter_hepa_ulpa_alignment_rear_cross_stop",
        24.0,
        SCAN_FRAME_Y - 44.0,
        SCAN_RAIL_Z,
    )
    .translate(SCAN_FRAME_X / 2.0 - 28.0, 0.0, SCAN_FRAME_Z / 2.0);
    let sled = scan_probe_sled();
    let lane_index = scan_lane_index_teeth();
    let standoff = scan_standoff_gauge();
    let filter_face = filter_face_surrogate_frame();

    left_rail + right_rail + front_cross + rear_cross + sled + lane_index + standoff + filter_face
}

fn scan_probe_sled() -> Part {
    let sled = centered_cube(
        "particle_counter_hepa_ulpa_scan_probe_cross_sled",
        82.0,
        SCAN_FRAME_Y - 34.0,
        28.0,
    )
    .translate(-70.0, 0.0, SCAN_FRAME_Z / 2.0 + 24.0);
    let probe_socket = centered_cylinder(
        "particle_counter_hepa_ulpa_scan_probe_socket",
        SCAN_PROBE_SOCKET_D / 2.0,
        90.0,
        42,
    )
    .translate(-70.0, 0.0, SCAN_FRAME_Z / 2.0 + 42.0);
    let bore = centered_cylinder(
        "particle_counter_hepa_ulpa_scan_probe_socket_bore",
        (SCAN_PROBE_SOCKET_D / 2.0) - 3.0,
        96.0,
        42,
    )
    .translate(-70.0, 0.0, SCAN_FRAME_Z / 2.0 + 42.0);
    let cable_tunnel = centered_cube(
        "particle_counter_hepa_ulpa_scan_probe_cable_tunnel",
        26.0,
        34.0,
        18.0,
    )
    .translate(-70.0, -SCAN_FRAME_Y / 2.0 + 28.0, SCAN_FRAME_Z / 2.0 + 48.0);

    sled + (probe_socket - bore) - cable_tunnel
}

fn scan_lane_index_teeth() -> Part {
    let mut teeth = Part::empty("particle_counter_hepa_ulpa_scan_lane_index_teeth");
    for i in 0..SCAN_LANE_COUNT {
        let x = centered_index(i, SCAN_LANE_COUNT, SCAN_LANE_PITCH);
        let tooth = centered_cube(
            format!("particle_counter_hepa_ulpa_scan_lane_index_tooth_{i}"),
            5.0,
            SCAN_FRAME_Y - 56.0,
            18.0,
        )
        .translate(x, 0.0, SCAN_FRAME_Z / 2.0 + 9.0);
        let label = centered_cube(
            format!("particle_counter_hepa_ulpa_scan_lane_label_land_{i}"),
            30.0,
            16.0,
            3.0,
        )
        .translate(x, -SCAN_FRAME_Y / 2.0 + 52.0, SCAN_FRAME_Z / 2.0 + 16.0);
        teeth = teeth + tooth + label;
    }
    teeth
}

fn scan_standoff_gauge() -> Part {
    let gauge_bar = centered_cube(
        "particle_counter_hepa_ulpa_scan_probe_standoff_gauge_bar",
        SCAN_FRAME_X - 66.0,
        10.0,
        16.0,
    )
    .translate(0.0, 0.0, SCAN_FRAME_Z / 2.0 + PROBE_STANDOFF_MM);
    let low_stop = centered_cube(
        "particle_counter_hepa_ulpa_scan_probe_low_standoff_stop",
        80.0,
        8.0,
        12.0,
    )
    .translate(SCAN_FRAME_X / 2.0 - 80.0, -42.0, SCAN_FRAME_Z / 2.0 + 10.0);
    let high_stop = centered_cube(
        "particle_counter_hepa_ulpa_scan_probe_high_standoff_stop",
        80.0,
        8.0,
        18.0,
    )
    .translate(SCAN_FRAME_X / 2.0 - 80.0, 42.0, SCAN_FRAME_Z / 2.0 + 16.0);

    gauge_bar + low_stop + high_stop
}

fn filter_face_surrogate_frame() -> Part {
    let outer = centered_cube(
        "particle_counter_hepa_ulpa_filter_face_surrogate_outer_frame",
        SCAN_FRAME_X - 88.0,
        SCAN_FRAME_Y - 92.0,
        16.0,
    )
    .translate(0.0, 0.0, SCAN_FRAME_Z / 2.0 - 18.0);
    let inner = centered_cube(
        "particle_counter_hepa_ulpa_filter_face_surrogate_inner_opening",
        SCAN_FRAME_X - 140.0,
        SCAN_FRAME_Y - 144.0,
        20.0,
    )
    .translate(0.0, 0.0, SCAN_FRAME_Z / 2.0 - 18.0);
    let challenge_strip = centered_cube(
        "particle_counter_hepa_ulpa_filter_face_aerosol_challenge_strip",
        SCAN_FRAME_X - 160.0,
        12.0,
        10.0,
    )
    .translate(0.0, SCAN_FRAME_Y / 2.0 - 72.0, SCAN_FRAME_Z / 2.0 - 3.0);

    (outer - inner) + challenge_strip
}

fn particle_counter_inlet_bulkhead_panel() -> Part {
    let panel = centered_cube(
        "particle_counter_inlet_bulkhead_panel_body",
        INLET_PANEL_X,
        INLET_PANEL_Y,
        INLET_PANEL_Z,
    );
    let rear_backer = centered_cube(
        "particle_counter_inlet_bulkhead_rear_backer",
        INLET_PANEL_X,
        16.0,
        INLET_PANEL_Z + 40.0,
    )
    .translate(0.0, INLET_PANEL_Y / 2.0 - 8.0, 20.0);
    let gasket = gasket_frame(
        "particle_counter_inlet_bulkhead_closed_transfer_gasket",
        INLET_PANEL_X - 34.0,
        INLET_PANEL_Y - 38.0,
        7.0,
        8.0,
    )
    .translate(0.0, 0.0, INLET_PANEL_Z / 2.0 + 5.0);

    panel + rear_backer + gasket - particle_inlet_port_bores() + port_collars() + filter_cap_parks()
}

fn port_collars() -> Part {
    let mut collars = Part::empty("particle_counter_inlet_bulkhead_port_collars");
    for i in 0..PARTICLE_PORT_COUNT {
        let x = centered_index(i, PARTICLE_PORT_COUNT, PARTICLE_PORT_PITCH);
        let collar = centered_cylinder(
            format!("particle_counter_inlet_bulkhead_port_collar_{i}"),
            17.0,
            10.0,
            36,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -INLET_PANEL_Y / 2.0 - 4.0, 8.0);
        collars = collars + collar;
    }
    collars
}

fn particle_inlet_port_bores() -> Part {
    let mut bores = Part::empty("particle_counter_inlet_bulkhead_port_bores");
    for i in 0..PARTICLE_PORT_COUNT {
        let x = centered_index(i, PARTICLE_PORT_COUNT, PARTICLE_PORT_PITCH);
        let primary = centered_cylinder(
            format!("particle_counter_inlet_bulkhead_primary_bore_{i}"),
            9.0,
            INLET_PANEL_Y + 28.0,
            36,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 0.0, 8.0);
        let tube_bore = centered_cylinder(
            format!("particle_counter_inlet_bulkhead_sample_tube_bore_{i}"),
            SAMPLE_TUBE_CLEARANCE_D / 2.0,
            INLET_PANEL_X + 12.0,
            28,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(x, 0.0, -12.0);
        bores = bores + primary + tube_bore;
    }
    bores
}

fn filter_cap_parks() -> Part {
    let mut parks = Part::empty("particle_counter_inlet_bulkhead_filter_cap_parks");
    for i in 0..3 {
        let x = centered_index(i, 3, 84.0);
        parks = parks
            + centered_cube(
                format!("particle_counter_inlet_bulkhead_zero_filter_label_land_{i}"),
                62.0,
                18.0,
                4.0,
            )
            .translate(x, INLET_PANEL_Y / 2.0 - 38.0, INLET_PANEL_Z / 2.0 + 2.0)
            + centered_cylinder(
                format!("particle_counter_inlet_bulkhead_zero_filter_cap_park_{i}"),
                13.0,
                8.0,
                32,
            )
            .translate(x, INLET_PANEL_Y / 2.0 - 70.0, INLET_PANEL_Z / 2.0 + 4.0);
    }
    parks
}

fn barcode_certificate_lands() -> Part {
    let plate = centered_cube(
        "particle_counter_traceability_barcode_certificate_plate",
        TRACE_PANEL_X,
        TRACE_PANEL_Y,
        TRACE_PANEL_Z,
    );
    let barcode_lands = barcode_land_grid();
    let certificate_lands = certificate_card_lands();
    let camera_alignment = centered_cube(
        "particle_counter_traceability_camera_alignment_land",
        TRACE_PANEL_X - 52.0,
        18.0,
        5.0,
    )
    .translate(0.0, TRACE_PANEL_Y / 2.0 - 28.0, TRACE_PANEL_Z / 2.0 + 3.0);
    let record_notch = centered_cube(
        "particle_counter_traceability_certificate_thumb_notch",
        68.0,
        22.0,
        TRACE_PANEL_Z + 4.0,
    )
    .translate(0.0, -TRACE_PANEL_Y / 2.0 + 12.0, 0.0);

    plate - record_notch + barcode_lands + certificate_lands + camera_alignment
}

fn barcode_land_grid() -> Part {
    let mut lands = Part::empty("particle_counter_traceability_barcode_lands");
    for i in 0..BARCODE_LANDS {
        let col = i % 2;
        let row = i / 2;
        let x = -58.0 + col as f64 * 116.0;
        let y = -56.0 + row as f64 * 36.0;
        lands = lands
            + centered_cube(
                format!("particle_counter_traceability_barcode_land_{i}"),
                88.0,
                22.0,
                4.0,
            )
            .translate(x, y, TRACE_PANEL_Z / 2.0 + 2.0);
    }
    lands
}

fn certificate_card_lands() -> Part {
    let mut lands = Part::empty("particle_counter_traceability_certificate_lands");
    for i in 0..CERTIFICATE_LANDS {
        let y = -72.0 + i as f64 * 54.0;
        let land = centered_cube(
            format!("particle_counter_traceability_certificate_card_land_{i}"),
            66.0,
            36.0,
            4.0,
        )
        .translate(TRACE_PANEL_X / 2.0 - 48.0, y, TRACE_PANEL_Z / 2.0 + 2.0);
        let clip = centered_cube(
            format!("particle_counter_traceability_certificate_clip_{i}"),
            54.0,
            5.0,
            10.0,
        )
        .translate(
            TRACE_PANEL_X / 2.0 - 48.0,
            y + 20.0,
            TRACE_PANEL_Z / 2.0 + 7.0,
        );
        lands = lands + land + clip;
    }
    lands
}

fn released_hold_reject_lanes() -> Part {
    let tray = centered_cube(
        "particle_counter_released_hold_reject_lane_tray",
        STATUS_X,
        STATUS_Y,
        STATUS_Z,
    );
    let basin = centered_cube(
        "particle_counter_released_hold_reject_lane_basin",
        STATUS_X - 36.0,
        STATUS_Y - 34.0,
        12.0,
    )
    .translate(0.0, 0.0, STATUS_Z / 2.0 - 4.0);
    let front_access = centered_cube(
        "particle_counter_released_hold_reject_front_robot_access",
        STATUS_X - 58.0,
        24.0,
        STATUS_Z + 4.0,
    )
    .translate(0.0, -STATUS_Y / 2.0 + 14.0, 0.0);

    tray - basin - front_access - status_slot_cuts()
        + status_lane_dividers()
        + status_label_lands()
        + status_lane_hard_stops()
}

fn status_slot_cuts() -> Part {
    let mut cuts = Part::empty("particle_counter_status_lane_slot_cuts");
    for lane in 0..STATUS_LANES {
        let x = status_lane_x(lane);
        for slot in 0..STATUS_SLOTS_PER_LANE {
            let y = -48.0 + slot as f64 * 34.0;
            cuts = cuts
                + centered_cube(
                    format!("particle_counter_status_lane_{lane}_coupon_carrier_slot_{slot}"),
                    STATUS_SLOT_X,
                    STATUS_SLOT_Y,
                    STATUS_Z + 5.0,
                )
                .translate(x, y, 0.0);
        }
    }
    cuts
}

fn status_lane_dividers() -> Part {
    let release_hold = centered_cube(
        "particle_counter_status_released_hold_divider",
        8.0,
        STATUS_Y - 44.0,
        30.0,
    )
    .translate(
        (status_lane_x(0) + status_lane_x(1)) / 2.0,
        0.0,
        STATUS_Z / 2.0 + 15.0,
    );
    let hold_reject = centered_cube(
        "particle_counter_status_hold_reject_divider",
        8.0,
        STATUS_Y - 44.0,
        38.0,
    )
    .translate(
        (status_lane_x(1) + status_lane_x(2)) / 2.0,
        0.0,
        STATUS_Z / 2.0 + 19.0,
    );
    let reject_wall = centered_cube(
        "particle_counter_status_reject_tall_outer_wall",
        10.0,
        STATUS_Y - 36.0,
        54.0,
    )
    .translate(status_lane_x(2) + 55.0, 0.0, STATUS_Z / 2.0 + 27.0);

    release_hold + hold_reject + reject_wall
}

fn status_label_lands() -> Part {
    let released = centered_cube(
        "particle_counter_status_released_label_land",
        78.0,
        18.0,
        5.0,
    )
    .translate(
        status_lane_x(0),
        STATUS_Y / 2.0 - 24.0,
        STATUS_Z / 2.0 + 3.0,
    );
    let hold = centered_cube("particle_counter_status_hold_label_land", 78.0, 18.0, 5.0).translate(
        status_lane_x(1),
        STATUS_Y / 2.0 - 24.0,
        STATUS_Z / 2.0 + 3.0,
    );
    let reject = centered_cube("particle_counter_status_reject_label_land", 78.0, 18.0, 5.0)
        .translate(
            status_lane_x(2),
            STATUS_Y / 2.0 - 24.0,
            STATUS_Z / 2.0 + 3.0,
        );

    released + hold + reject
}

fn status_lane_hard_stops() -> Part {
    let mut stops = Part::empty("particle_counter_status_lane_hard_stops");
    for lane in 0..STATUS_LANES {
        let x = status_lane_x(lane);
        stops = stops
            + centered_cube(
                format!("particle_counter_status_lane_{lane}_rear_hard_stop"),
                STATUS_SLOT_X + 10.0,
                8.0,
                26.0,
            )
            .translate(x, 78.0, STATUS_Z / 2.0 + 13.0)
            + centered_cube(
                format!("particle_counter_status_lane_{lane}_front_low_stop"),
                STATUS_SLOT_X + 10.0,
                6.0,
                16.0,
            )
            .translate(x, -78.0, STATUS_Z / 2.0 + 8.0);
    }
    stops
}

fn status_lane_x(lane: usize) -> f64 {
    centered_index(lane, STATUS_LANES, 118.0)
}

fn clean_used_coupon_segregation_tray() -> Part {
    let tray = centered_cube(
        "particle_counter_clean_used_coupon_segregation_tray_body",
        SEG_TRAY_X,
        SEG_TRAY_Y,
        SEG_TRAY_Z,
    );
    let clean_basin = centered_cube(
        "particle_counter_clean_coupon_basin",
        SEG_TRAY_X / 2.0 - 24.0,
        SEG_TRAY_Y - 42.0,
        10.0,
    )
    .translate(-SEG_TRAY_X / 4.0, 0.0, SEG_TRAY_Z / 2.0 - 4.0);
    let used_basin = centered_cube(
        "particle_counter_used_coupon_basin",
        SEG_TRAY_X / 2.0 - 24.0,
        SEG_TRAY_Y - 42.0,
        10.0,
    )
    .translate(SEG_TRAY_X / 4.0, 0.0, SEG_TRAY_Z / 2.0 - 4.0);
    let divider = centered_cube(
        "particle_counter_clean_used_coupon_tall_divider",
        12.0,
        SEG_TRAY_Y - 24.0,
        CLEAN_USED_DIVIDER_Z,
    )
    .translate(0.0, 0.0, SEG_TRAY_Z / 2.0 + CLEAN_USED_DIVIDER_Z / 2.0);

    tray - clean_basin - used_basin - clean_used_well_cuts()
        + divider
        + clean_used_label_lands()
        + coupon_transfer_bridge()
}

fn clean_used_well_cuts() -> Part {
    let mut cuts = Part::empty("particle_counter_clean_used_coupon_well_cuts");
    for side in 0..2 {
        let base_x = if side == 0 {
            -SEG_TRAY_X / 4.0
        } else {
            SEG_TRAY_X / 4.0
        };
        for i in 0..SEG_WELLS_PER_SIDE {
            let y = centered_index(i, SEG_WELLS_PER_SIDE, 30.0);
            cuts = cuts
                + centered_cube(
                    format!("particle_counter_coupon_segregation_side_{side}_well_{i}"),
                    76.0,
                    20.0,
                    SEG_TRAY_Z + 4.0,
                )
                .translate(base_x, y, 0.0);
        }
    }
    cuts
}

fn clean_used_label_lands() -> Part {
    let clean = centered_cube("particle_counter_clean_coupon_label_land", 96.0, 18.0, 4.0)
        .translate(
            -SEG_TRAY_X / 4.0,
            SEG_TRAY_Y / 2.0 - 24.0,
            SEG_TRAY_Z / 2.0 + 2.0,
        );
    let used = centered_cube("particle_counter_used_coupon_label_land", 96.0, 18.0, 4.0).translate(
        SEG_TRAY_X / 4.0,
        SEG_TRAY_Y / 2.0 - 24.0,
        SEG_TRAY_Z / 2.0 + 2.0,
    );
    clean + used
}

fn coupon_transfer_bridge() -> Part {
    let bridge = centered_cube(
        "particle_counter_clean_used_coupon_one_way_transfer_bridge",
        70.0,
        28.0,
        22.0,
    )
    .translate(0.0, -SEG_TRAY_Y / 2.0 + 36.0, SEG_TRAY_Z / 2.0 + 11.0);
    let slot = centered_cube(
        "particle_counter_clean_used_coupon_one_way_transfer_slot",
        52.0,
        12.0,
        24.0,
    )
    .translate(0.0, -SEG_TRAY_Y / 2.0 + 36.0, SEG_TRAY_Z / 2.0 + 11.0);

    bridge - slot
}

fn waste_used_coupon_bin() -> Part {
    let shell = centered_cube(
        "particle_counter_waste_used_coupon_bin_outer_shell",
        WASTE_BIN_X,
        WASTE_BIN_Y,
        WASTE_BIN_Z,
    );
    let cavity = centered_cube(
        "particle_counter_waste_used_coupon_bin_liner_cavity",
        WASTE_BIN_X - 38.0,
        WASTE_BIN_Y - 38.0,
        WASTE_BIN_Z - 24.0,
    )
    .translate(0.0, 0.0, 12.0);
    let lid = centered_cube(
        "particle_counter_waste_used_coupon_bin_latching_lid",
        WASTE_BIN_X - 20.0,
        WASTE_BIN_Y - 20.0,
        16.0,
    )
    .translate(0.0, 0.0, WASTE_BIN_Z / 2.0 + 8.0);
    let chute_cut = centered_cube(
        "particle_counter_waste_used_coupon_bin_one_way_chute_cut",
        WASTE_CHUTE_X,
        WASTE_CHUTE_Y,
        20.0,
    )
    .translate(0.0, -WASTE_BIN_Y / 2.0 + 42.0, WASTE_BIN_Z / 2.0 + 10.0);
    let liner_ring = gasket_frame(
        "particle_counter_waste_used_coupon_bin_liner_retainer_ring",
        WASTE_BIN_X - 34.0,
        WASTE_BIN_Y - 34.0,
        10.0,
        10.0,
    )
    .translate(0.0, 0.0, WASTE_BIN_Z / 2.0 - 5.0);

    shell - cavity + lid - chute_cut
        + liner_ring
        + waste_coupon_slot_markers()
        + waste_handle_tabs()
}

fn waste_coupon_slot_markers() -> Part {
    let mut markers = Part::empty("particle_counter_waste_coupon_slot_markers");
    for i in 0..WASTE_COUPON_SLOTS {
        let x = centered_index(i % 4, 4, 56.0);
        let y = -12.0 + (i / 4) as f64 * 44.0;
        markers = markers
            + centered_cube(
                format!("particle_counter_waste_coupon_slot_marker_{i}"),
                38.0,
                18.0,
                4.0,
            )
            .translate(x, y, WASTE_BIN_Z / 2.0 + 18.0);
    }
    markers
}

fn waste_handle_tabs() -> Part {
    let left = centered_cube(
        "particle_counter_waste_coupon_bin_left_service_handle",
        18.0,
        74.0,
        28.0,
    )
    .translate(-WASTE_BIN_X / 2.0 - 9.0, 0.0, WASTE_BIN_Z / 2.0);
    let right = centered_cube(
        "particle_counter_waste_coupon_bin_right_service_handle",
        18.0,
        74.0,
        28.0,
    )
    .translate(WASTE_BIN_X / 2.0 + 9.0, 0.0, WASTE_BIN_Z / 2.0);
    left + right
}

fn service_bulkhead() -> Part {
    let panel = centered_cube(
        "particle_counter_rear_service_bulkhead_panel",
        SERVICE_BULKHEAD_X,
        SERVICE_BULKHEAD_Y,
        SERVICE_BULKHEAD_Z,
    );
    let gasket = gasket_frame(
        "particle_counter_rear_service_bulkhead_isolator_gasket_land",
        SERVICE_BULKHEAD_X - 60.0,
        SERVICE_BULKHEAD_Y + 18.0,
        8.0,
        9.0,
    )
    .translate(0.0, -SERVICE_BULKHEAD_Y / 2.0 - 3.0, 0.0);
    let cable_tray = centered_cube(
        "particle_counter_rear_service_bulkhead_cable_tray",
        SERVICE_BULKHEAD_X - 120.0,
        24.0,
        26.0,
    )
    .translate(
        0.0,
        SERVICE_BULKHEAD_Y / 2.0 + 18.0,
        -SERVICE_BULKHEAD_Z / 2.0 + 40.0,
    );

    panel + gasket + cable_tray - service_port_bores()
        + service_port_collars()
        + service_label_lands()
}

fn service_port_bores() -> Part {
    let mut bores = Part::empty("particle_counter_rear_service_bulkhead_port_bores");
    for i in 0..SERVICE_PORTS {
        let x = centered_index(i, SERVICE_PORTS, SERVICE_PORT_PITCH);
        let radius = match i {
            0 | 1 => 11.0,
            2 | 3 => 8.0,
            4 | 5 => 13.0,
            6 | 7 => 6.0,
            _ => 9.0,
        };
        bores = bores
            + centered_cylinder(
                format!("particle_counter_rear_service_bulkhead_port_bore_{i}"),
                radius,
                SERVICE_BULKHEAD_Y + 20.0,
                36,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, 10.0);
    }
    bores
}

fn service_port_collars() -> Part {
    let mut collars = Part::empty("particle_counter_rear_service_bulkhead_port_collars");
    for i in 0..SERVICE_PORTS {
        let x = centered_index(i, SERVICE_PORTS, SERVICE_PORT_PITCH);
        collars = collars
            + centered_cylinder(
                format!("particle_counter_rear_service_bulkhead_port_collar_{i}"),
                20.0,
                9.0,
                36,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -SERVICE_BULKHEAD_Y / 2.0 - 4.0, 10.0);
    }
    collars
}

fn service_label_lands() -> Part {
    let mut labels = Part::empty("particle_counter_rear_service_bulkhead_label_lands");
    for i in 0..SERVICE_PORTS {
        let x = centered_index(i, SERVICE_PORTS, SERVICE_PORT_PITCH);
        labels = labels
            + centered_cube(
                format!("particle_counter_rear_service_bulkhead_label_land_{i}"),
                68.0,
                18.0,
                4.0,
            )
            .translate(
                x,
                -SERVICE_BULKHEAD_Y / 2.0 - 8.0,
                -SERVICE_BULKHEAD_Z / 2.0 + 18.0,
            );
    }
    labels
}

fn robot_keepout_gauge() -> Part {
    let front_approach = centered_cube(
        "particle_counter_robot_front_approach_keepout_bar",
        KEEP_OUT_X,
        10.0,
        18.0,
    )
    .translate(0.0, -KEEP_OUT_Y / 2.0, DECK_Z + 9.0);
    let rear_service = centered_cube(
        "particle_counter_rear_service_keepout_bar",
        KEEP_OUT_X,
        10.0,
        22.0,
    )
    .translate(0.0, KEEP_OUT_Y / 2.0, DECK_Z + 11.0);
    let left_side = centered_cube(
        "particle_counter_left_coupon_load_keepout_bar",
        10.0,
        KEEP_OUT_Y,
        18.0,
    )
    .translate(-KEEP_OUT_X / 2.0, 0.0, DECK_Z + 9.0);
    let right_side = centered_cube(
        "particle_counter_right_robot_sweep_keepout_bar",
        10.0,
        KEEP_OUT_Y,
        18.0,
    )
    .translate(KEEP_OUT_X / 2.0, 0.0, DECK_Z + 9.0);

    front_approach
        + rear_service
        + left_side
        + right_side
        + vertical_keepout_masts()
        + robot_sweep_window_markers()
}

fn vertical_keepout_masts() -> Part {
    let mut masts = Part::empty("particle_counter_vertical_robot_keepout_masts");
    for (i, (x, y)) in [
        (-KEEP_OUT_X / 2.0, -KEEP_OUT_Y / 2.0),
        (KEEP_OUT_X / 2.0, -KEEP_OUT_Y / 2.0),
        (-KEEP_OUT_X / 2.0, KEEP_OUT_Y / 2.0),
        (KEEP_OUT_X / 2.0, KEEP_OUT_Y / 2.0),
    ]
    .iter()
    .enumerate()
    {
        masts = masts
            + centered_cube(
                format!("particle_counter_robot_keepout_vertical_mast_{i}"),
                18.0,
                18.0,
                KEEP_OUT_Z,
            )
            .translate(*x, *y, DECK_Z + KEEP_OUT_Z / 2.0);
    }
    masts
}

fn robot_sweep_window_markers() -> Part {
    let front_window = centered_cube(
        "particle_counter_robot_front_sweep_window_marker",
        360.0,
        8.0,
        34.0,
    )
    .translate(0.0, -KEEP_OUT_Y / 2.0 + 54.0, DECK_Z + 17.0);
    let waste_window = centered_cube(
        "particle_counter_robot_waste_drop_sweep_window_marker",
        260.0,
        8.0,
        34.0,
    )
    .translate(WASTE_POS.0, -KEEP_OUT_Y / 2.0 + 92.0, DECK_Z + 17.0);
    let service_window = centered_cube(
        "particle_counter_rear_bulkhead_service_window_marker",
        420.0,
        8.0,
        38.0,
    )
    .translate(0.0, KEEP_OUT_Y / 2.0 - 54.0, DECK_Z + 19.0);

    front_window + waste_window + service_window
}

fn fiducial_disc(name: &str) -> Part {
    let disc = centered_cylinder(format!("{name}_disc"), 10.0, 3.0, 32);
    let slot_a = centered_cube(format!("{name}_slot_a"), 18.0, 3.0, 4.0);
    let slot_b = centered_cube(format!("{name}_slot_b"), 3.0, 18.0, 4.0);
    disc - slot_a - slot_b
}

fn gasket_frame(name: &str, x: f64, y: f64, z: f64, wall: f64) -> Part {
    let outer = centered_cube(format!("{name}_outer"), x, y, z);
    let inner = centered_cube(
        format!("{name}_inner_cut"),
        x - 2.0 * wall,
        y - 2.0 * wall,
        z + 2.0,
    );
    outer - inner
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn output_names_are_stable_and_unique() {
        assert_eq!(OUTPUTS.len(), 13);
        let unique: HashSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert!(OUTPUTS.iter().all(|path| path.starts_with(
            "output/closed_environmental_particle_counter_probe_calibration_station_"
        )));
    }

    #[test]
    fn required_feature_list_covers_user_requested_surfaces() {
        for feature in [
            "clean_probe_parking_cradle",
            "reference_particle_aerosol_challenge_coupon_holder",
            "hepa_ulpa_scan_probe_alignment_geometry",
            "barcode_certificate_lands",
            "released_hold_reject_lanes",
            "clean_used_coupon_segregation",
            "waste_used_coupon_bin",
            "robot_keepouts",
            "service_bulkhead",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn modules_fit_and_do_not_overlap() {
        assert_layout_constraints();
    }

    #[test]
    fn disposition_and_challenge_capacity_is_balanced() {
        assert_eq!(STATUS_LANES * STATUS_SLOTS_PER_LANE, 12);
        assert!(CHALLENGE_COUPON_COUNT >= 6);
        assert!(WASTE_COUPON_SLOTS >= CHALLENGE_COUPON_COUNT);
        assert_eq!(SEG_WELLS_PER_SIDE * 2, 10);
    }

    #[test]
    fn closed_scan_geometry_has_serviceable_clearances() {
        assert!(PROBE_STANDOFF_MM >= 25.0);
        assert!(SCAN_FRAME_Z > INLET_PANEL_Z);
        assert!(ROBOT_Z_CLEARANCE_MM > KEEP_OUT_Z + DECK_Z);
        assert!(SERVICE_REAR_CLEARANCE_MM > SERVICE_BULKHEAD_Y + 200.0);
    }
}
