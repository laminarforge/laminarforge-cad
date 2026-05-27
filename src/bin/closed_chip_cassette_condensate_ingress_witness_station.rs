use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed chip cassette condensate ingress witness station.
//
// Product-concept CAD for validating whether condensate exposure reaches a
// sealed cassette boundary. The generator models station envelopes, witness
// coupons, traceability lands, disposition lanes, evidence capture geometry, and
// robot/service keepouts. It is not a test method, acceptance criterion, or
// sterility claim.

const OUTPUTS: [&str; 12] = [
    "output/closed_chip_cassette_condensate_ingress_witness_station_base_containment_tray.stl",
    "output/closed_chip_cassette_condensate_ingress_witness_station_sealed_cassette_nest.stl",
    "output/closed_chip_cassette_condensate_ingress_witness_station_condensate_drip_rail.stl",
    "output/closed_chip_cassette_condensate_ingress_witness_station_ingress_witness_coupon_windows.stl",
    "output/closed_chip_cassette_condensate_ingress_witness_station_humidity_logger_pockets.stl",
    "output/closed_chip_cassette_condensate_ingress_witness_station_dye_recovery_wells.stl",
    "output/closed_chip_cassette_condensate_ingress_witness_station_gasket_witness_lanes.stl",
    "output/closed_chip_cassette_condensate_ingress_witness_station_barcode_certificate_lands.stl",
    "output/closed_chip_cassette_condensate_ingress_witness_station_release_hold_reject_lanes.stl",
    "output/closed_chip_cassette_condensate_ingress_witness_station_evidence_bridge.stl",
    "output/closed_chip_cassette_condensate_ingress_witness_station_robot_service_keepouts.stl",
    "output/closed_chip_cassette_condensate_ingress_witness_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 10] = [
    "sealed_cassette_nest",
    "condensate_drip_rail",
    "ingress_witness_coupon_windows",
    "humidity_logger_pockets",
    "dye_recovery_wells",
    "gasket_witness_lanes",
    "barcode_certificate_lands",
    "release_hold_reject_lanes",
    "evidence_bridge",
    "robot_service_keepouts",
];

const DECK_X: f64 = 1540.0;
const DECK_Y: f64 = 960.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const SOCKET_DEPTH: f64 = 6.0;
const MOUNT_HOLE_D: f64 = 6.8;
const SECONDARY_BASIN_DEPTH: f64 = 7.5;
const DRAIN_D: f64 = 16.0;
const LEAK_SENSOR_WELLS: usize = 4;

const CHIP_COLS: usize = 4;
const CHIP_ROWS: usize = 5;
const CHIP_POSITIONS: usize = CHIP_COLS * CHIP_ROWS;
const CHIP_GUTTER: f64 = 13.0;
const CHIP_PITCH_X: f64 = REVC_CHIP_LENGTH + CHIP_GUTTER;
const CHIP_PITCH_Y: f64 = REVC_CHIP_WIDTH + CHIP_GUTTER;
const CHIP_ARRAY_X: f64 =
    CHIP_COLS as f64 * REVC_CHIP_LENGTH + (CHIP_COLS as f64 - 1.0) * CHIP_GUTTER;
const CHIP_ARRAY_Y: f64 =
    CHIP_ROWS as f64 * REVC_CHIP_WIDTH + (CHIP_ROWS as f64 - 1.0) * CHIP_GUTTER;

const NEST_POS: (f64, f64) = (-410.0, 82.0);
const NEST_X: f64 = 650.0;
const NEST_Y: f64 = 560.0;
const NEST_Z: f64 = 46.0;
const NEST_CAVITY_X: f64 = CHIP_ARRAY_X + 92.0;
const NEST_CAVITY_Y: f64 = CHIP_ARRAY_Y + 82.0;
const CASSETTE_SEAL_MOAT_W: f64 = 18.0;
const CASSETTE_CLAMP_COUNT: usize = 8;
const DATUM_PIN_D: f64 = 7.0;
const VACUUM_WITNESS_PORTS: usize = 4;

const DRIP_POS: (f64, f64) = (-410.0, 402.0);
const DRIP_RAIL_X: f64 = 622.0;
const DRIP_RAIL_Y: f64 = 88.0;
const DRIP_RAIL_Z: f64 = 54.0;
const DRIP_PORTS: usize = 7;
const DRIP_PORT_D: f64 = 6.0;
const DRIP_CHANNEL_W: f64 = 18.0;
const DRIP_SHIELD_Z: f64 = 76.0;
const DRIP_WITNESS_TICKS: usize = 14;

const COUPON_WINDOWS: usize = CHIP_POSITIONS;
const COUPON_WINDOW_X: f64 = 82.0;
const COUPON_WINDOW_Y: f64 = 32.0;
const COUPON_FRAME_W: f64 = 8.0;
const COUPON_Z: f64 = 9.0;
const COUPON_LABEL_X: f64 = 42.0;
const COUPON_LABEL_Y: f64 = 12.0;

const LOGGER_POS: (f64, f64) = (150.0, 250.0);
const LOGGER_X: f64 = 285.0;
const LOGGER_Y: f64 = 210.0;
const LOGGER_Z: f64 = 38.0;
const LOGGER_ROWS: usize = 2;
const LOGGER_COLS: usize = 2;
const LOGGER_POCKETS: usize = LOGGER_ROWS * LOGGER_COLS;
const LOGGER_POCKET_X: f64 = 76.0;
const LOGGER_POCKET_Y: f64 = 48.0;
const LOGGER_PITCH_X: f64 = 104.0;
const LOGGER_PITCH_Y: f64 = 82.0;
const LOGGER_VENT_SLOTS: usize = 12;

const DYE_POS: (f64, f64) = (500.0, 250.0);
const DYE_X: f64 = 330.0;
const DYE_Y: f64 = 210.0;
const DYE_Z: f64 = 40.0;
const DYE_ROWS: usize = 2;
const DYE_COLS: usize = 4;
const DYE_WELLS: usize = DYE_ROWS * DYE_COLS;
const DYE_WELL_D: f64 = 34.0;
const DYE_SPLASH_RIM_D: f64 = 48.0;
const DYE_PITCH_X: f64 = 72.0;
const DYE_PITCH_Y: f64 = 82.0;
const DYE_RECOVERY_VOLUME_ML: f64 = 240.0;
const CHALLENGE_DYE_VOLUME_ML: f64 = 120.0;

const GASKET_POS: (f64, f64) = (160.0, -110.0);
const GASKET_X: f64 = 385.0;
const GASKET_Y: f64 = 210.0;
const GASKET_Z: f64 = 34.0;
const GASKET_WITNESS_LANES: usize = 6;
const GASKET_LANE_X: f64 = 326.0;
const GASKET_LANE_Y: f64 = 18.0;
const GASKET_LANE_PITCH_Y: f64 = 28.0;
const GASKET_CONTROL_COUPONS: usize = 3;

const TRACE_POS: (f64, f64) = (555.0, -80.0);
const TRACE_X: f64 = 310.0;
const TRACE_Y: f64 = 190.0;
const TRACE_Z: f64 = 12.0;
const BARCODE_LANDS: usize = 8;
const CERTIFICATE_LANDS: usize = 3;
const FIDUCIALS: usize = 6;
const BARCODE_LAND_X: f64 = 72.0;
const BARCODE_LAND_Y: f64 = 18.0;
const CERTIFICATE_LAND_X: f64 = 118.0;
const CERTIFICATE_LAND_Y: f64 = 34.0;

const STATUS_POS: (f64, f64) = (430.0, -330.0);
const STATUS_X: f64 = 620.0;
const STATUS_Y: f64 = 150.0;
const STATUS_Z: f64 = 42.0;
const STATUS_LANE_COUNT: usize = 3;
const STATUS_SLOTS_PER_LANE: usize = 4;
const STATUS_LANE_X: f64 = 180.0;
const STATUS_LANE_PITCH_X: f64 = 205.0;

const BRIDGE_POS: (f64, f64) = (-30.0, 382.0);
const BRIDGE_X: f64 = 780.0;
const BRIDGE_Y: f64 = 72.0;
const BRIDGE_Z: f64 = 138.0;
const BRIDGE_BEAM_Z: f64 = 18.0;
const CAMERA_LANDS: usize = 4;
const EVIDENCE_SCALE_TICKS: usize = 20;

const ROBOT_APPROACH_Y: f64 = 300.0;
const REAR_SERVICE_Y: f64 = 170.0;
const SIDE_SERVICE_X: f64 = 118.0;
const KEEP_OUT_Z: f64 = 152.0;
const ROBOT_PICK_CLEARANCE_Z: f64 = 245.0;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DispositionLane {
    Release,
    Hold,
    Reject,
}

impl DispositionLane {
    fn all() -> [DispositionLane; STATUS_LANE_COUNT] {
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
    fn fits_inside(self) -> bool {
        let usable_x = DECK_X / 2.0 - RIM_W;
        let usable_y = DECK_Y / 2.0 - RIM_W;
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

    let base = base_containment_tray();
    export(OUTPUTS[0], &base);

    let nest = sealed_cassette_nest();
    export(OUTPUTS[1], &nest);

    let drip_rail = condensate_drip_rail();
    export(OUTPUTS[2], &drip_rail);

    let coupon_windows = ingress_witness_coupon_windows();
    export(OUTPUTS[3], &coupon_windows);

    let loggers = humidity_logger_pockets();
    export(OUTPUTS[4], &loggers);

    let dye_wells = dye_recovery_wells();
    export(OUTPUTS[5], &dye_wells);

    let gasket_lanes = gasket_witness_lanes();
    export(OUTPUTS[6], &gasket_lanes);

    let traceability = barcode_certificate_lands();
    export(OUTPUTS[7], &traceability);

    let status_lanes = release_hold_reject_lanes();
    export(OUTPUTS[8], &status_lanes);

    let evidence = evidence_bridge();
    export(OUTPUTS[9], &evidence);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let assembly = base
        + nest.translate(NEST_POS.0, NEST_POS.1, deck_top_z())
        + drip_rail.translate(DRIP_POS.0, DRIP_POS.1, deck_top_z())
        + coupon_windows.translate(NEST_POS.0, NEST_POS.1, deck_top_z())
        + loggers.translate(LOGGER_POS.0, LOGGER_POS.1, deck_top_z())
        + dye_wells.translate(DYE_POS.0, DYE_POS.1, deck_top_z())
        + gasket_lanes.translate(GASKET_POS.0, GASKET_POS.1, deck_top_z())
        + traceability.translate(TRACE_POS.0, TRACE_POS.1, deck_top_z())
        + status_lanes.translate(STATUS_POS.0, STATUS_POS.1, deck_top_z())
        + evidence.translate(BRIDGE_POS.0, BRIDGE_POS.1, deck_top_z())
        + keepouts;
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed chip cassette condensate ingress witness station:");
    println!("  Footprint:             {DECK_X:.0}mm x {DECK_Y:.0}mm secondary-containment tray");
    println!(
        "  Cassette nest:         {CHIP_POSITIONS} chip positions in a sealed cassette envelope, {VACUUM_WITNESS_PORTS} vacuum/pressure witness ports, {CASSETTE_CLAMP_COUNT} compression clamps"
    );
    println!(
        "  Condensate challenge:  {DRIP_PORTS} drip ports, {DRIP_WITNESS_TICKS} dose ticks, {COUPON_WINDOWS} ingress witness coupon windows, {GASKET_WITNESS_LANES} gasket witness lanes"
    );
    println!(
        "  Evidence capture:      {LOGGER_POCKETS} humidity logger pockets, {DYE_WELLS} dye recovery wells, {CAMERA_LANDS} evidence bridge camera lands, {BARCODE_LANDS} barcode lands, {CERTIFICATE_LANDS} certificate lands"
    );
    println!(
        "  Disposition/keepouts:  {} release/hold/reject lanes, {STATUS_SLOTS_PER_LANE} token slots per lane, {LEAK_SENSOR_WELLS} leak sensor wells, {ROBOT_APPROACH_Y:.0}mm robot approach band, {REAR_SERVICE_Y:.0}mm rear service band",
        DispositionLane::all().len()
    );
    println!("  Feature groups:        {}", REQUIRED_FEATURES.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn deck_top_z() -> f64 {
    DECK_Z / 2.0
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 12);
    assert_eq!(REQUIRED_FEATURES.len(), 10);
    assert_eq!(CHIP_POSITIONS, 20);
    assert_eq!(COUPON_WINDOWS, CHIP_POSITIONS);
    assert_eq!(LEAK_SENSOR_WELLS, 4);
    assert_eq!(LOGGER_POCKETS, 4);
    assert_eq!(DYE_WELLS, 8);
    assert_eq!(STATUS_LANE_COUNT, DispositionLane::all().len());
    assert_eq!(STATUS_LANE_COUNT * STATUS_SLOTS_PER_LANE, 12);
    assert!(NEST_CAVITY_X > CHIP_ARRAY_X + 70.0);
    assert!(NEST_CAVITY_Y > CHIP_ARRAY_Y + 60.0);
    assert!(COUPON_WINDOW_X < REVC_CHIP_LENGTH);
    assert!(COUPON_WINDOW_Y < REVC_CHIP_WIDTH);
    assert!(DYE_RECOVERY_VOLUME_ML >= CHALLENGE_DYE_VOLUME_ML * 1.5);
    assert!(ROBOT_PICK_CLEARANCE_Z > DECK_Z + BRIDGE_Z + 70.0);
    assert!(GASKET_WITNESS_LANES >= GASKET_CONTROL_COUPONS + 3);

    let rects = layout_rects();
    for rect in rects {
        assert!(rect.fits_inside(), "{} exceeds deck envelope", rect.name);
    }
    for left in 0..rects.len() {
        for right in (left + 1)..rects.len() {
            assert!(
                !rects[left].overlaps(rects[right]),
                "{} overlaps {}",
                rects[left].name,
                rects[right].name
            );
        }
    }
}

fn layout_rects() -> [Rect; 6] {
    [
        rect("sealed_cassette_nest", NEST_POS, NEST_X, NEST_Y),
        rect("humidity_logger_pockets", LOGGER_POS, LOGGER_X, LOGGER_Y),
        rect("dye_recovery_wells", DYE_POS, DYE_X, DYE_Y),
        rect("gasket_witness_lanes", GASKET_POS, GASKET_X, GASKET_Y),
        rect("barcode_certificate_lands", TRACE_POS, TRACE_X, TRACE_Y),
        rect("release_hold_reject_lanes", STATUS_POS, STATUS_X, STATUS_Y),
    ]
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn base_containment_tray() -> Part {
    let deck = centered_cube(
        "condensate_ingress_witness_base_containment_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    );
    let basin = centered_cube(
        "condensate_ingress_witness_base_secondary_basin_cut",
        DECK_X - 2.0 * (RIM_W + 42.0),
        DECK_Y - 2.0 * (RIM_W + 44.0),
        SECONDARY_BASIN_DEPTH + 0.6,
    )
    .translate(0.0, 2.0, DECK_Z / 2.0 - SECONDARY_BASIN_DEPTH / 2.0);
    let forward_drain = centered_cylinder(
        "condensate_ingress_witness_base_forward_drain_cut",
        DRAIN_D / 2.0,
        58.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 94.0, -DECK_Y / 2.0 + 36.0, 0.0);

    deck - basin - forward_drain - module_socket_cuts() - deck_mount_holes()
        + perimeter_rims()
        + base_leak_sensor_wells()
        + base_workflow_ribs()
        + base_datum_targets()
}

fn module_socket_cuts() -> Part {
    let mut sockets = Part::empty("condensate_ingress_witness_module_socket_cuts");
    for rect in layout_rects() {
        sockets = sockets
            + centered_cube(
                format!("condensate_ingress_witness_socket_cut_{}", rect.name),
                rect.x + 16.0,
                rect.y + 16.0,
                SOCKET_DEPTH + 0.8,
            )
            .translate(
                rect.center.0,
                rect.center.1,
                DECK_Z / 2.0 - SOCKET_DEPTH / 2.0,
            );
    }
    sockets
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("condensate_ingress_witness_deck_mount_holes");
    for (i, (x, y)) in mount_points().iter().enumerate() {
        let round = centered_cylinder(
            format!("condensate_ingress_witness_m6_mount_round_{i}"),
            MOUNT_HOLE_D / 2.0,
            DECK_Z + 6.0,
            28,
        )
        .translate(*x, *y, 0.0);
        let slot = centered_cube(
            format!("condensate_ingress_witness_m6_mount_slot_{i}"),
            24.0,
            7.0,
            DECK_Z + 6.0,
        )
        .translate(*x, *y, 0.0);
        holes = holes + round + slot;
    }
    holes
}

fn mount_points() -> [(f64, f64); 10] {
    [
        (-DECK_X / 2.0 + 56.0, -DECK_Y / 2.0 + 56.0),
        (DECK_X / 2.0 - 56.0, -DECK_Y / 2.0 + 56.0),
        (-DECK_X / 2.0 + 56.0, DECK_Y / 2.0 - 56.0),
        (DECK_X / 2.0 - 56.0, DECK_Y / 2.0 - 56.0),
        (0.0, -DECK_Y / 2.0 + 56.0),
        (0.0, DECK_Y / 2.0 - 56.0),
        (-DECK_X / 2.0 + 56.0, 0.0),
        (DECK_X / 2.0 - 56.0, 0.0),
        (-260.0, -DECK_Y / 2.0 + 56.0),
        (260.0, -DECK_Y / 2.0 + 56.0),
    ]
}

fn perimeter_rims() -> Part {
    let front = centered_cube(
        "condensate_ingress_witness_front_low_spill_rim",
        DECK_X,
        RIM_W,
        RIM_Z * 0.72,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + RIM_W / 2.0,
        DECK_Z / 2.0 + RIM_Z * 0.36,
    );
    let rear = centered_cube(
        "condensate_ingress_witness_rear_humidity_service_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, DECK_Z / 2.0 + RIM_Z / 2.0);
    let left = centered_cube(
        "condensate_ingress_witness_left_cassette_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z / 2.0 + RIM_Z / 2.0);
    let right = centered_cube(
        "condensate_ingress_witness_right_evidence_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z / 2.0 + RIM_Z / 2.0);
    front + rear + left + right
}

fn base_leak_sensor_wells() -> Part {
    let mut wells = Part::empty("condensate_ingress_witness_base_leak_sensor_wells");
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 118.0, -DECK_Y / 2.0 + 102.0),
        (DECK_X / 2.0 - 118.0, -DECK_Y / 2.0 + 102.0),
        (-DECK_X / 2.0 + 118.0, DECK_Y / 2.0 - 112.0),
        (DECK_X / 2.0 - 118.0, DECK_Y / 2.0 - 112.0),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cube(
            format!("condensate_ingress_witness_leak_sensor_boss_{i}"),
            50.0,
            36.0,
            7.0,
        )
        .translate(*x, *y, DECK_Z / 2.0 + 3.5);
        let cup = centered_cylinder(
            format!("condensate_ingress_witness_leak_sensor_recess_{i}"),
            10.0,
            9.0,
            30,
        )
        .translate(*x, *y, DECK_Z / 2.0 + 3.5);
        wells = wells + (boss - cup);
    }
    wells
}

fn base_workflow_ribs() -> Part {
    let cassette_to_wells = centered_cube(
        "condensate_ingress_witness_flow_rib_cassette_to_dye_wells",
        880.0,
        5.0,
        4.0,
    )
    .rotate(0.0, 0.0, -12.0)
    .translate(-8.0, 132.0, DECK_Z / 2.0 + 2.0);
    let wells_to_lanes = centered_cube(
        "condensate_ingress_witness_flow_rib_dye_wells_to_disposition",
        555.0,
        5.0,
        4.0,
    )
    .rotate(0.0, 0.0, -54.0)
    .translate(470.0, -110.0, DECK_Z / 2.0 + 2.0);
    let witness_to_trace = centered_cube(
        "condensate_ingress_witness_flow_rib_gasket_witness_to_certificate",
        385.0,
        5.0,
        4.0,
    )
    .rotate(0.0, 0.0, 8.0)
    .translate(360.0, -86.0, DECK_Z / 2.0 + 2.0);
    cassette_to_wells + wells_to_lanes + witness_to_trace
}

fn base_datum_targets() -> Part {
    let mut datums = Part::empty("condensate_ingress_witness_base_robot_datums");
    for (i, (x, y)) in [
        (-690.0, 406.0),
        (-112.0, 406.0),
        (-690.0, -212.0),
        (708.0, -405.0),
        (708.0, 362.0),
        (82.0, -405.0),
    ]
    .iter()
    .enumerate()
    {
        datums = datums
            + fiducial_disc(&format!("condensate_ingress_witness_base_datum_{i}")).translate(
                *x,
                *y,
                DECK_Z / 2.0 + 2.5,
            );
    }
    datums
}

fn sealed_cassette_nest() -> Part {
    let body = centered_cube(
        "condensate_ingress_witness_sealed_cassette_nest_body",
        NEST_X,
        NEST_Y,
        NEST_Z,
    )
    .translate(0.0, 0.0, NEST_Z / 2.0);
    let cassette_cavity = centered_cube(
        "condensate_ingress_witness_sealed_cassette_cavity_cut",
        NEST_CAVITY_X,
        NEST_CAVITY_Y,
        24.0,
    )
    .translate(0.0, 0.0, NEST_Z - 10.0);
    let inner_leak_moat = centered_cube(
        "condensate_ingress_witness_inner_condensate_moat_cut",
        NEST_CAVITY_X - 48.0,
        NEST_CAVITY_Y - 48.0,
        9.0,
    )
    .translate(0.0, 0.0, NEST_Z - 7.0);

    body - cassette_cavity - inner_leak_moat - vacuum_witness_port_cuts()
        + cassette_seal_moat_ribs()
        + cassette_hard_stops()
        + cassette_clamps()
        + cassette_datum_pins()
        + vacuum_witness_port_bosses()
        + nest_finger_reliefs()
}

fn cassette_seal_moat_ribs() -> Part {
    let outer_x = NEST_CAVITY_X + CASSETTE_SEAL_MOAT_W;
    let outer_y = NEST_CAVITY_Y + CASSETTE_SEAL_MOAT_W;
    let rib_z = 10.0;
    let rear = centered_cube(
        "condensate_ingress_witness_seal_moat_rear_rib",
        outer_x,
        CASSETTE_SEAL_MOAT_W,
        rib_z,
    )
    .translate(0.0, outer_y / 2.0, NEST_Z + rib_z / 2.0);
    let front = centered_cube(
        "condensate_ingress_witness_seal_moat_front_rib",
        outer_x,
        CASSETTE_SEAL_MOAT_W,
        rib_z,
    )
    .translate(0.0, -outer_y / 2.0, NEST_Z + rib_z / 2.0);
    let left = centered_cube(
        "condensate_ingress_witness_seal_moat_left_rib",
        CASSETTE_SEAL_MOAT_W,
        outer_y,
        rib_z,
    )
    .translate(-outer_x / 2.0, 0.0, NEST_Z + rib_z / 2.0);
    let right = centered_cube(
        "condensate_ingress_witness_seal_moat_right_rib",
        CASSETTE_SEAL_MOAT_W,
        outer_y,
        rib_z,
    )
    .translate(outer_x / 2.0, 0.0, NEST_Z + rib_z / 2.0);
    rear + front + left + right
}

fn cassette_hard_stops() -> Part {
    let rear_stop = centered_cube(
        "condensate_ingress_witness_cassette_rear_hard_stop",
        NEST_CAVITY_X - 26.0,
        12.0,
        34.0,
    )
    .translate(0.0, NEST_CAVITY_Y / 2.0 - 18.0, NEST_Z + 17.0);
    let left_stop = centered_cube(
        "condensate_ingress_witness_cassette_left_datum_stop",
        12.0,
        NEST_CAVITY_Y - 48.0,
        28.0,
    )
    .translate(-NEST_CAVITY_X / 2.0 + 24.0, 0.0, NEST_Z + 14.0);
    let front_low_lip = centered_cube(
        "condensate_ingress_witness_cassette_front_low_robot_lip",
        NEST_CAVITY_X - 92.0,
        10.0,
        16.0,
    )
    .translate(18.0, -NEST_CAVITY_Y / 2.0 + 22.0, NEST_Z + 8.0);
    rear_stop + left_stop + front_low_lip
}

fn cassette_clamps() -> Part {
    let mut clamps = Part::empty("condensate_ingress_witness_cassette_clamps");
    for i in 0..CASSETTE_CLAMP_COUNT {
        let along_x = i < CASSETTE_CLAMP_COUNT / 2;
        let side_index = i % (CASSETTE_CLAMP_COUNT / 2);
        let x = centered_index(side_index, CASSETTE_CLAMP_COUNT / 2, 142.0);
        let y = if along_x {
            NEST_CAVITY_Y / 2.0 + 30.0
        } else {
            -NEST_CAVITY_Y / 2.0 - 30.0
        };
        let pedestal = centered_cube(
            format!("condensate_ingress_witness_cassette_clamp_pedestal_{i}"),
            58.0,
            22.0,
            22.0,
        )
        .translate(x, y, NEST_Z + 11.0);
        let jaw = centered_cube(
            format!("condensate_ingress_witness_cassette_clamp_jaw_{i}"),
            46.0,
            16.0,
            14.0,
        )
        .translate(x, y.signum() * (NEST_CAVITY_Y / 2.0 + 8.0), NEST_Z + 30.0);
        clamps = clamps + pedestal + jaw;
    }
    clamps
}

fn cassette_datum_pins() -> Part {
    let mut pins = Part::empty("condensate_ingress_witness_cassette_datum_pins");
    for (i, (x, y)) in cassette_corner_points(36.0).iter().enumerate() {
        let pin = centered_cylinder(
            format!("condensate_ingress_witness_datum_pin_{i}"),
            DATUM_PIN_D / 2.0,
            20.0,
            28,
        )
        .translate(*x, *y, NEST_Z + 10.0);
        let guard = centered_cube(
            format!("condensate_ingress_witness_datum_pin_guard_{i}"),
            30.0,
            12.0,
            8.0,
        )
        .translate(*x, *y + y.signum() * 18.0, NEST_Z + 4.0);
        pins = pins + pin + guard;
    }
    pins
}

fn vacuum_witness_port_bosses() -> Part {
    let mut bosses = Part::empty("condensate_ingress_witness_vacuum_port_bosses");
    for (i, (x, y)) in witness_port_points().iter().enumerate() {
        let boss = centered_cylinder(
            format!("condensate_ingress_witness_vacuum_port_boss_{i}"),
            18.0,
            12.0,
            36,
        )
        .translate(*x, *y, NEST_Z + 6.0);
        let label = centered_cube(
            format!("condensate_ingress_witness_vacuum_port_label_land_{i}"),
            42.0,
            12.0,
            3.0,
        )
        .translate(*x, *y + y.signum() * 28.0, NEST_Z + 2.0);
        bosses = bosses + boss + label;
    }
    bosses
}

fn vacuum_witness_port_cuts() -> Part {
    let mut cuts = Part::empty("condensate_ingress_witness_vacuum_port_cuts");
    for (i, (x, y)) in witness_port_points().iter().enumerate() {
        cuts = cuts
            + centered_cylinder(
                format!("condensate_ingress_witness_vacuum_port_bore_{i}"),
                5.0,
                NEST_Z + 26.0,
                28,
            )
            .translate(*x, *y, NEST_Z / 2.0 + 6.0);
    }
    cuts
}

fn nest_finger_reliefs() -> Part {
    let mut reliefs = Part::empty("condensate_ingress_witness_nest_finger_reliefs");
    for (i, x) in [-236.0, 0.0, 236.0].iter().enumerate() {
        let relief = centered_cube(
            format!("condensate_ingress_witness_front_finger_relief_{i}"),
            72.0,
            18.0,
            18.0,
        )
        .translate(*x, -NEST_CAVITY_Y / 2.0 - 6.0, NEST_Z + 9.0);
        reliefs = reliefs + relief;
    }
    reliefs
}

fn condensate_drip_rail() -> Part {
    let rail = centered_cube(
        "condensate_ingress_witness_condensate_drip_rail_body",
        DRIP_RAIL_X,
        DRIP_RAIL_Y,
        DRIP_RAIL_Z,
    )
    .translate(0.0, 0.0, DRIP_RAIL_Z / 2.0);
    let channel = centered_cube(
        "condensate_ingress_witness_drip_rail_supply_channel_cut",
        DRIP_RAIL_X - 70.0,
        DRIP_CHANNEL_W,
        16.0,
    )
    .translate(0.0, 12.0, DRIP_RAIL_Z - 7.0);

    rail - channel - drip_port_cuts()
        + drip_meter_ticks()
        + drip_break_lips()
        + rail_splash_shield()
        + rail_end_caps()
}

fn drip_port_cuts() -> Part {
    let mut cuts = Part::empty("condensate_ingress_witness_drip_port_cuts");
    for i in 0..DRIP_PORTS {
        let x = centered_index(i, DRIP_PORTS, 82.0);
        cuts = cuts
            + centered_cylinder(
                format!("condensate_ingress_witness_drip_port_bore_{i}"),
                DRIP_PORT_D / 2.0,
                DRIP_RAIL_Z + 8.0,
                24,
            )
            .translate(x, -14.0, DRIP_RAIL_Z / 2.0);
    }
    cuts
}

fn drip_meter_ticks() -> Part {
    let mut ticks = Part::empty("condensate_ingress_witness_drip_meter_ticks");
    for i in 0..DRIP_WITNESS_TICKS {
        let x = centered_index(i, DRIP_WITNESS_TICKS, 40.0);
        let tall = i % 2 == 0;
        let tick = centered_cube(
            format!("condensate_ingress_witness_drip_meter_tick_{i}"),
            4.0,
            if tall { 24.0 } else { 14.0 },
            4.0,
        )
        .translate(x, DRIP_RAIL_Y / 2.0 - 18.0, DRIP_RAIL_Z + 2.0);
        ticks = ticks + tick;
    }
    ticks
}

fn drip_break_lips() -> Part {
    let rear = centered_cube(
        "condensate_ingress_witness_drip_rail_rear_source_lip",
        DRIP_RAIL_X - 48.0,
        7.0,
        18.0,
    )
    .translate(0.0, DRIP_RAIL_Y / 2.0 - 18.0, DRIP_RAIL_Z + 9.0);
    let front = centered_cube(
        "condensate_ingress_witness_drip_rail_front_drip_break_lip",
        DRIP_RAIL_X - 48.0,
        5.0,
        14.0,
    )
    .translate(0.0, -DRIP_RAIL_Y / 2.0 + 15.0, DRIP_RAIL_Z + 7.0);
    rear + front
}

fn rail_splash_shield() -> Part {
    let shield = centered_cube(
        "condensate_ingress_witness_drip_rail_clear_splash_shield_envelope",
        DRIP_RAIL_X - 30.0,
        6.0,
        DRIP_SHIELD_Z,
    )
    .translate(
        0.0,
        -DRIP_RAIL_Y / 2.0 - 12.0,
        DRIP_RAIL_Z + DRIP_SHIELD_Z / 2.0,
    );
    let witness_slot = centered_cube(
        "condensate_ingress_witness_drip_rail_splash_shield_view_slot",
        DRIP_RAIL_X - 120.0,
        8.0,
        24.0,
    )
    .translate(0.0, -DRIP_RAIL_Y / 2.0 - 12.0, DRIP_RAIL_Z + 38.0);
    shield - witness_slot
}

fn rail_end_caps() -> Part {
    let left = centered_cube(
        "condensate_ingress_witness_drip_rail_left_end_cap",
        18.0,
        DRIP_RAIL_Y + 16.0,
        DRIP_RAIL_Z + 16.0,
    )
    .translate(-DRIP_RAIL_X / 2.0 - 9.0, 0.0, DRIP_RAIL_Z / 2.0 + 8.0);
    let right = centered_cube(
        "condensate_ingress_witness_drip_rail_right_end_cap",
        18.0,
        DRIP_RAIL_Y + 16.0,
        DRIP_RAIL_Z + 16.0,
    )
    .translate(DRIP_RAIL_X / 2.0 + 9.0, 0.0, DRIP_RAIL_Z / 2.0 + 8.0);
    left + right
}

fn ingress_witness_coupon_windows() -> Part {
    let mut windows = Part::empty("condensate_ingress_witness_coupon_window_array");
    for row in 0..CHIP_ROWS {
        for col in 0..CHIP_COLS {
            let index = row * CHIP_COLS + col;
            let (x, y) = chip_center(col, row);
            windows = windows + coupon_window(index).translate(x, y, NEST_Z + COUPON_Z / 2.0);
        }
    }
    windows + coupon_window_row_labels()
}

fn coupon_window(index: usize) -> Part {
    let outer = centered_cube(
        format!("condensate_ingress_witness_coupon_window_outer_frame_{index}"),
        COUPON_WINDOW_X + 2.0 * COUPON_FRAME_W,
        COUPON_WINDOW_Y + 2.0 * COUPON_FRAME_W,
        COUPON_Z,
    );
    let inner = centered_cube(
        format!("condensate_ingress_witness_coupon_window_opening_{index}"),
        COUPON_WINDOW_X,
        COUPON_WINDOW_Y,
        COUPON_Z + 1.0,
    );
    let dye_streak = centered_cube(
        format!("condensate_ingress_witness_coupon_dye_streak_reference_{index}"),
        COUPON_WINDOW_X - 18.0,
        4.0,
        3.0,
    )
    .translate(0.0, 0.0, COUPON_Z / 2.0 + 1.5);
    let label_land = centered_cube(
        format!("condensate_ingress_witness_coupon_label_land_{index}"),
        COUPON_LABEL_X,
        COUPON_LABEL_Y,
        3.0,
    )
    .translate(0.0, COUPON_WINDOW_Y / 2.0 + 18.0, COUPON_Z / 2.0 + 1.5);
    outer - inner + dye_streak + label_land
}

fn coupon_window_row_labels() -> Part {
    let mut labels = Part::empty("condensate_ingress_witness_coupon_row_labels");
    for row in 0..CHIP_ROWS {
        let (_, y) = chip_center(0, row);
        labels = labels
            + centered_cube(
                format!("condensate_ingress_witness_coupon_row_label_land_{row}"),
                42.0,
                18.0,
                4.0,
            )
            .translate(-CHIP_ARRAY_X / 2.0 - 42.0, y, NEST_Z + COUPON_Z / 2.0 + 2.0);
    }
    labels
}

fn humidity_logger_pockets() -> Part {
    let plate = centered_cube(
        "condensate_ingress_witness_humidity_logger_pocket_plate",
        LOGGER_X,
        LOGGER_Y,
        LOGGER_Z,
    )
    .translate(0.0, 0.0, LOGGER_Z / 2.0);
    let plenum = centered_cube(
        "condensate_ingress_witness_humidity_logger_shared_air_plenum_cut",
        LOGGER_X - 48.0,
        22.0,
        16.0,
    )
    .translate(0.0, 0.0, LOGGER_Z - 7.0);

    plate - plenum - logger_pocket_cuts()
        + logger_pocket_retainers()
        + logger_air_vent_slots()
        + logger_chain_of_custody_tabs()
}

fn logger_pocket_cuts() -> Part {
    let mut cuts = Part::empty("condensate_ingress_witness_logger_pocket_cuts");
    for row in 0..LOGGER_ROWS {
        for col in 0..LOGGER_COLS {
            let index = row * LOGGER_COLS + col;
            let (x, y) = logger_xy(row, col);
            cuts = cuts
                + centered_cube(
                    format!("condensate_ingress_witness_logger_pocket_recess_{index}"),
                    LOGGER_POCKET_X,
                    LOGGER_POCKET_Y,
                    18.0,
                )
                .translate(x, y, LOGGER_Z - 8.0);
        }
    }
    cuts
}

fn logger_pocket_retainers() -> Part {
    let mut retainers = Part::empty("condensate_ingress_witness_logger_pocket_retainers");
    for row in 0..LOGGER_ROWS {
        for col in 0..LOGGER_COLS {
            let index = row * LOGGER_COLS + col;
            let (x, y) = logger_xy(row, col);
            let left_clip = centered_cube(
                format!("condensate_ingress_witness_logger_left_clip_{index}"),
                6.0,
                LOGGER_POCKET_Y + 10.0,
                12.0,
            )
            .translate(x - LOGGER_POCKET_X / 2.0 + 4.0, y, LOGGER_Z + 6.0);
            let right_clip = centered_cube(
                format!("condensate_ingress_witness_logger_right_clip_{index}"),
                6.0,
                LOGGER_POCKET_Y + 10.0,
                12.0,
            )
            .translate(x + LOGGER_POCKET_X / 2.0 - 4.0, y, LOGGER_Z + 6.0);
            retainers = retainers + left_clip + right_clip;
        }
    }
    retainers
}

fn logger_air_vent_slots() -> Part {
    let mut vents = Part::empty("condensate_ingress_witness_logger_air_vent_slots");
    for i in 0..LOGGER_VENT_SLOTS {
        let x = centered_index(i % 6, 6, 34.0);
        let y = if i < 6 { 84.0 } else { -84.0 };
        vents = vents
            + centered_cube(
                format!("condensate_ingress_witness_logger_air_vent_land_{i}"),
                22.0,
                5.0,
                4.0,
            )
            .translate(x, y, LOGGER_Z + 2.0);
    }
    vents
}

fn logger_chain_of_custody_tabs() -> Part {
    let left = centered_cube(
        "condensate_ingress_witness_logger_left_custody_tab",
        72.0,
        18.0,
        4.0,
    )
    .translate(
        -LOGGER_X / 2.0 + 58.0,
        -LOGGER_Y / 2.0 + 20.0,
        LOGGER_Z + 2.0,
    );
    let right = centered_cube(
        "condensate_ingress_witness_logger_right_custody_tab",
        72.0,
        18.0,
        4.0,
    )
    .translate(
        LOGGER_X / 2.0 - 58.0,
        -LOGGER_Y / 2.0 + 20.0,
        LOGGER_Z + 2.0,
    );
    left + right
}

fn dye_recovery_wells() -> Part {
    let plate = centered_cube(
        "condensate_ingress_witness_dye_recovery_well_plate",
        DYE_X,
        DYE_Y,
        DYE_Z,
    )
    .translate(0.0, 0.0, DYE_Z / 2.0);
    let gutter = centered_cube(
        "condensate_ingress_witness_dye_recovery_inlet_gutter_cut",
        DYE_X - 42.0,
        18.0,
        14.0,
    )
    .translate(0.0, DYE_Y / 2.0 - 32.0, DYE_Z - 6.0);

    plate - gutter - dye_well_cuts() + dye_well_rims() + dye_well_index_lands()
}

fn dye_well_cuts() -> Part {
    let mut cuts = Part::empty("condensate_ingress_witness_dye_recovery_well_cuts");
    for row in 0..DYE_ROWS {
        for col in 0..DYE_COLS {
            let index = row * DYE_COLS + col;
            let (x, y) = dye_xy(row, col);
            cuts = cuts
                + centered_cylinder(
                    format!("condensate_ingress_witness_dye_recovery_well_recess_{index}"),
                    DYE_WELL_D / 2.0,
                    DYE_Z + 8.0,
                    44,
                )
                .translate(x, y, DYE_Z / 2.0 + 4.0);
        }
    }
    cuts
}

fn dye_well_rims() -> Part {
    let mut rims = Part::empty("condensate_ingress_witness_dye_recovery_well_rims");
    for row in 0..DYE_ROWS {
        for col in 0..DYE_COLS {
            let index = row * DYE_COLS + col;
            let (x, y) = dye_xy(row, col);
            let outer = centered_cylinder(
                format!("condensate_ingress_witness_dye_splash_rim_outer_{index}"),
                DYE_SPLASH_RIM_D / 2.0,
                8.0,
                44,
            );
            let inner = centered_cylinder(
                format!("condensate_ingress_witness_dye_splash_rim_inner_{index}"),
                (DYE_WELL_D / 2.0) + 2.0,
                9.0,
                44,
            );
            rims = rims + (outer - inner).translate(x, y, DYE_Z + 4.0);
        }
    }
    rims
}

fn dye_well_index_lands() -> Part {
    let mut lands = Part::empty("condensate_ingress_witness_dye_well_index_lands");
    for row in 0..DYE_ROWS {
        for col in 0..DYE_COLS {
            let index = row * DYE_COLS + col;
            let (x, y) = dye_xy(row, col);
            lands = lands
                + centered_cube(
                    format!("condensate_ingress_witness_dye_well_barcode_land_{index}"),
                    42.0,
                    12.0,
                    3.0,
                )
                .translate(x, y - 33.0, DYE_Z + 1.5);
        }
    }
    lands
}

fn gasket_witness_lanes() -> Part {
    let body = centered_cube(
        "condensate_ingress_witness_gasket_lane_carrier",
        GASKET_X,
        GASKET_Y,
        GASKET_Z,
    )
    .translate(0.0, 0.0, GASKET_Z / 2.0);
    let shallow_basin = centered_cube(
        "condensate_ingress_witness_gasket_lane_basin_cut",
        GASKET_X - 42.0,
        GASKET_Y - 36.0,
        9.0,
    )
    .translate(0.0, 0.0, GASKET_Z - 4.0);

    body - shallow_basin - gasket_lane_groove_cuts()
        + gasket_witness_strips()
        + gasket_lane_pinches()
        + gasket_control_coupon_lands()
}

fn gasket_lane_groove_cuts() -> Part {
    let mut grooves = Part::empty("condensate_ingress_witness_gasket_lane_groove_cuts");
    for i in 0..GASKET_WITNESS_LANES {
        let y = centered_index(i, GASKET_WITNESS_LANES, GASKET_LANE_PITCH_Y);
        grooves = grooves
            + centered_cube(
                format!("condensate_ingress_witness_gasket_lane_recess_{i}"),
                GASKET_LANE_X,
                GASKET_LANE_Y,
                12.0,
            )
            .translate(0.0, y, GASKET_Z - 5.0);
    }
    grooves
}

fn gasket_witness_strips() -> Part {
    let mut strips = Part::empty("condensate_ingress_witness_gasket_witness_strips");
    for i in 0..GASKET_WITNESS_LANES {
        let y = centered_index(i, GASKET_WITNESS_LANES, GASKET_LANE_PITCH_Y);
        let strip = centered_cube(
            format!("condensate_ingress_witness_gasket_strip_visible_land_{i}"),
            GASKET_LANE_X - 34.0,
            GASKET_LANE_Y - 7.0,
            3.0,
        )
        .translate(0.0, y, GASKET_Z + 1.5);
        let start_marker = centered_cube(
            format!("condensate_ingress_witness_gasket_strip_start_marker_{i}"),
            9.0,
            GASKET_LANE_Y + 12.0,
            6.0,
        )
        .translate(-GASKET_LANE_X / 2.0 + 18.0, y, GASKET_Z + 3.0);
        strips = strips + strip + start_marker;
    }
    strips
}

fn gasket_lane_pinches() -> Part {
    let mut pinches = Part::empty("condensate_ingress_witness_gasket_lane_pinches");
    for i in 0..GASKET_WITNESS_LANES {
        let y = centered_index(i, GASKET_WITNESS_LANES, GASKET_LANE_PITCH_Y);
        for (side, x) in [-1.0, 1.0].iter().enumerate() {
            pinches = pinches
                + centered_cube(
                    format!("condensate_ingress_witness_gasket_lane_pinch_{i}_{side}"),
                    12.0,
                    GASKET_LANE_Y + 20.0,
                    12.0,
                )
                .translate(*x * (GASKET_LANE_X / 2.0 - 34.0), y, GASKET_Z + 6.0);
        }
    }
    pinches
}

fn gasket_control_coupon_lands() -> Part {
    let mut lands = Part::empty("condensate_ingress_witness_gasket_control_coupon_lands");
    for i in 0..GASKET_CONTROL_COUPONS {
        lands = lands
            + centered_cube(
                format!("condensate_ingress_witness_gasket_control_coupon_land_{i}"),
                72.0,
                20.0,
                4.0,
            )
            .translate(
                centered_index(i, GASKET_CONTROL_COUPONS, 86.0),
                -GASKET_Y / 2.0 + 24.0,
                GASKET_Z + 2.0,
            );
    }
    lands
}

fn barcode_certificate_lands() -> Part {
    let plate = centered_cube(
        "condensate_ingress_witness_barcode_certificate_plate",
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    )
    .translate(0.0, 0.0, TRACE_Z / 2.0);
    plate + barcode_lands() + certificate_lands() + trace_fiducials() + custody_seal_ears()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("condensate_ingress_witness_barcode_lands");
    for i in 0..BARCODE_LANDS {
        let x = centered_index(i % 4, 4, 70.0);
        let y = 62.0 - (i / 4) as f64 * 34.0;
        lands = lands
            + centered_cube(
                format!("condensate_ingress_witness_barcode_land_{i}"),
                BARCODE_LAND_X,
                BARCODE_LAND_Y,
                4.0,
            )
            .translate(x, y, TRACE_Z + 2.0);
    }
    lands
}

fn certificate_lands() -> Part {
    let mut lands = Part::empty("condensate_ingress_witness_certificate_lands");
    for i in 0..CERTIFICATE_LANDS {
        let x = centered_index(i, CERTIFICATE_LANDS, 98.0);
        lands = lands
            + centered_cube(
                format!("condensate_ingress_witness_certificate_land_{i}"),
                CERTIFICATE_LAND_X,
                CERTIFICATE_LAND_Y,
                4.0,
            )
            .translate(x, -56.0, TRACE_Z + 2.0);
    }
    lands
}

fn trace_fiducials() -> Part {
    let mut fiducials = Part::empty("condensate_ingress_witness_trace_fiducials");
    for i in 0..FIDUCIALS {
        let x = if i % 2 == 0 {
            -TRACE_X / 2.0 + 28.0
        } else {
            TRACE_X / 2.0 - 28.0
        };
        let y = centered_index(i / 2, 3, 62.0);
        fiducials =
            fiducials
                + fiducial_disc(&format!("condensate_ingress_witness_trace_fiducial_{i}"))
                    .translate(x, y, TRACE_Z + 2.0);
    }
    fiducials
}

fn custody_seal_ears() -> Part {
    let left = centered_cube(
        "condensate_ingress_witness_certificate_left_custody_seal_ear",
        22.0,
        46.0,
        6.0,
    )
    .translate(-TRACE_X / 2.0 - 11.0, 0.0, TRACE_Z + 3.0);
    let right = centered_cube(
        "condensate_ingress_witness_certificate_right_custody_seal_ear",
        22.0,
        46.0,
        6.0,
    )
    .translate(TRACE_X / 2.0 + 11.0, 0.0, TRACE_Z + 3.0);
    let left_hole = centered_cylinder(
        "condensate_ingress_witness_certificate_left_seal_hole",
        4.0,
        8.0,
        24,
    )
    .translate(-TRACE_X / 2.0 - 11.0, 0.0, TRACE_Z + 3.0);
    let right_hole = centered_cylinder(
        "condensate_ingress_witness_certificate_right_seal_hole",
        4.0,
        8.0,
        24,
    )
    .translate(TRACE_X / 2.0 + 11.0, 0.0, TRACE_Z + 3.0);
    (left - left_hole) + (right - right_hole)
}

fn release_hold_reject_lanes() -> Part {
    let base = centered_cube(
        "condensate_ingress_witness_disposition_lane_base",
        STATUS_X,
        STATUS_Y,
        STATUS_Z,
    )
    .translate(0.0, 0.0, STATUS_Z / 2.0);
    let trough = centered_cube(
        "condensate_ingress_witness_disposition_lane_common_trough_cut",
        STATUS_X - 42.0,
        STATUS_Y - 36.0,
        12.0,
    )
    .translate(0.0, 0.0, STATUS_Z - 5.0);

    base - trough - disposition_token_cuts() + disposition_lane_rails() + disposition_gate_posts()
}

fn disposition_token_cuts() -> Part {
    let mut cuts = Part::empty("condensate_ingress_witness_disposition_token_cuts");
    for lane in DispositionLane::all() {
        let x = disposition_lane_x(lane);
        for slot in 0..STATUS_SLOTS_PER_LANE {
            let y = centered_index(slot, STATUS_SLOTS_PER_LANE, 28.0);
            cuts = cuts
                + centered_cube(
                    format!(
                        "condensate_ingress_witness_{}_token_slot_cut_{slot}",
                        lane.label()
                    ),
                    118.0,
                    18.0,
                    14.0,
                )
                .translate(x, y, STATUS_Z - 6.0);
        }
    }
    cuts
}

fn disposition_lane_rails() -> Part {
    let mut rails = Part::empty("condensate_ingress_witness_disposition_lane_rails");
    for lane in DispositionLane::all() {
        let x = disposition_lane_x(lane);
        let label = lane.label();
        let left = centered_cube(
            format!("condensate_ingress_witness_{label}_lane_left_rail"),
            8.0,
            STATUS_Y - 28.0,
            20.0,
        )
        .translate(x - STATUS_LANE_X / 2.0, 0.0, STATUS_Z + 10.0);
        let right = centered_cube(
            format!("condensate_ingress_witness_{label}_lane_right_rail"),
            8.0,
            STATUS_Y - 28.0,
            20.0,
        )
        .translate(x + STATUS_LANE_X / 2.0, 0.0, STATUS_Z + 10.0);
        let name_land = centered_cube(
            format!("condensate_ingress_witness_{label}_lane_name_land"),
            126.0,
            18.0,
            4.0,
        )
        .translate(x, STATUS_Y / 2.0 - 20.0, STATUS_Z + 2.0);
        rails = rails + left + right + name_land;
    }
    rails
}

fn disposition_gate_posts() -> Part {
    let mut posts = Part::empty("condensate_ingress_witness_disposition_gate_posts");
    for lane in DispositionLane::all() {
        let x = disposition_lane_x(lane);
        for (i, y) in [-(STATUS_Y / 2.0 - 26.0), STATUS_Y / 2.0 - 26.0]
            .iter()
            .enumerate()
        {
            posts = posts
                + centered_cylinder(
                    format!("condensate_ingress_witness_{}_gate_post_{i}", lane.label()),
                    7.0,
                    32.0,
                    24,
                )
                .translate(x, *y, STATUS_Z + 16.0);
        }
    }
    posts
}

fn evidence_bridge() -> Part {
    let left_column = centered_cube(
        "condensate_ingress_witness_evidence_bridge_left_column",
        34.0,
        BRIDGE_Y,
        BRIDGE_Z,
    )
    .translate(-BRIDGE_X / 2.0, 0.0, BRIDGE_Z / 2.0);
    let right_column = centered_cube(
        "condensate_ingress_witness_evidence_bridge_right_column",
        34.0,
        BRIDGE_Y,
        BRIDGE_Z,
    )
    .translate(BRIDGE_X / 2.0, 0.0, BRIDGE_Z / 2.0);
    let beam = centered_cube(
        "condensate_ingress_witness_evidence_bridge_camera_beam",
        BRIDGE_X,
        BRIDGE_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, 0.0, BRIDGE_Z - BRIDGE_BEAM_Z / 2.0);
    left_column + right_column + beam + evidence_camera_lands() + evidence_scale_ticks()
}

fn evidence_camera_lands() -> Part {
    let mut lands = Part::empty("condensate_ingress_witness_evidence_camera_lands");
    for i in 0..CAMERA_LANDS {
        let x = centered_index(i, CAMERA_LANDS, 205.0);
        let pad = centered_cube(
            format!("condensate_ingress_witness_evidence_camera_pad_{i}"),
            78.0,
            42.0,
            6.0,
        )
        .translate(x, 0.0, BRIDGE_Z + 3.0);
        let standoff = centered_cylinder(
            format!("condensate_ingress_witness_evidence_camera_standoff_{i}"),
            6.0,
            16.0,
            24,
        )
        .translate(x, 0.0, BRIDGE_Z + 11.0);
        lands = lands + pad + standoff;
    }
    lands
}

fn evidence_scale_ticks() -> Part {
    let mut ticks = Part::empty("condensate_ingress_witness_evidence_scale_ticks");
    for i in 0..EVIDENCE_SCALE_TICKS {
        let x = centered_index(i, EVIDENCE_SCALE_TICKS, 34.0);
        let tick = centered_cube(
            format!("condensate_ingress_witness_evidence_bridge_scale_tick_{i}"),
            3.0,
            if i % 5 == 0 { 36.0 } else { 22.0 },
            4.0,
        )
        .translate(x, -BRIDGE_Y / 2.0 + 10.0, BRIDGE_Z + 2.0);
        ticks = ticks + tick;
    }
    ticks
}

fn robot_service_keepouts() -> Part {
    let front_robot = centered_cube(
        "condensate_ingress_witness_front_robot_approach_keepout",
        DECK_X - 170.0,
        ROBOT_APPROACH_Y,
        4.0,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + ROBOT_APPROACH_Y / 2.0,
        DECK_Z / 2.0 + 2.0,
    );
    let rear_service = centered_cube(
        "condensate_ingress_witness_rear_humidity_service_keepout",
        DECK_X - 160.0,
        REAR_SERVICE_Y,
        4.0,
    )
    .translate(0.0, DECK_Y / 2.0 - REAR_SERVICE_Y / 2.0, DECK_Z / 2.0 + 2.0);
    let left_robot = centered_cube(
        "condensate_ingress_witness_left_cassette_gripper_keepout",
        SIDE_SERVICE_X,
        DECK_Y - 150.0,
        4.0,
    )
    .translate(
        -DECK_X / 2.0 + SIDE_SERVICE_X / 2.0,
        0.0,
        DECK_Z / 2.0 + 2.0,
    );
    let right_service = centered_cube(
        "condensate_ingress_witness_right_dye_service_keepout",
        SIDE_SERVICE_X,
        DECK_Y - 150.0,
        4.0,
    )
    .translate(DECK_X / 2.0 - SIDE_SERVICE_X / 2.0, 0.0, DECK_Z / 2.0 + 2.0);
    let overhead = centered_cube(
        "condensate_ingress_witness_overhead_robot_pick_clearance_envelope",
        NEST_X + 110.0,
        NEST_Y + 150.0,
        5.0,
    )
    .translate(NEST_POS.0, NEST_POS.1, KEEP_OUT_Z);
    let service_posts = keepout_service_posts();
    front_robot + rear_service + left_robot + right_service + overhead + service_posts
}

fn keepout_service_posts() -> Part {
    let mut posts = Part::empty("condensate_ingress_witness_keepout_service_posts");
    for (i, (x, y)) in [
        (NEST_POS.0 - NEST_X / 2.0 - 36.0, NEST_POS.1),
        (NEST_POS.0 + NEST_X / 2.0 + 36.0, NEST_POS.1),
        (DYE_POS.0 + DYE_X / 2.0 + 28.0, DYE_POS.1),
        (STATUS_POS.0 + STATUS_X / 2.0 - 28.0, STATUS_POS.1),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cylinder(
                format!("condensate_ingress_witness_keepout_post_{i}"),
                6.0,
                KEEP_OUT_Z,
                20,
            )
            .translate(*x, *y, KEEP_OUT_Z / 2.0);
    }
    posts
}

fn chip_center(col: usize, row: usize) -> (f64, f64) {
    (
        -CHIP_ARRAY_X / 2.0 + REVC_CHIP_LENGTH / 2.0 + col as f64 * CHIP_PITCH_X,
        -CHIP_ARRAY_Y / 2.0 + REVC_CHIP_WIDTH / 2.0 + row as f64 * CHIP_PITCH_Y,
    )
}

fn logger_xy(row: usize, col: usize) -> (f64, f64) {
    (
        centered_index(col, LOGGER_COLS, LOGGER_PITCH_X),
        centered_index(row, LOGGER_ROWS, LOGGER_PITCH_Y),
    )
}

fn dye_xy(row: usize, col: usize) -> (f64, f64) {
    (
        centered_index(col, DYE_COLS, DYE_PITCH_X),
        centered_index(row, DYE_ROWS, DYE_PITCH_Y),
    )
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn cassette_corner_points(inset: f64) -> [(f64, f64); 4] {
    [
        (-NEST_CAVITY_X / 2.0 + inset, -NEST_CAVITY_Y / 2.0 + inset),
        (NEST_CAVITY_X / 2.0 - inset, -NEST_CAVITY_Y / 2.0 + inset),
        (-NEST_CAVITY_X / 2.0 + inset, NEST_CAVITY_Y / 2.0 - inset),
        (NEST_CAVITY_X / 2.0 - inset, NEST_CAVITY_Y / 2.0 - inset),
    ]
}

fn witness_port_points() -> [(f64, f64); VACUUM_WITNESS_PORTS] {
    [
        (-NEST_CAVITY_X / 2.0 + 92.0, -NEST_CAVITY_Y / 2.0 - 26.0),
        (NEST_CAVITY_X / 2.0 - 92.0, -NEST_CAVITY_Y / 2.0 - 26.0),
        (-NEST_CAVITY_X / 2.0 + 92.0, NEST_CAVITY_Y / 2.0 + 26.0),
        (NEST_CAVITY_X / 2.0 - 92.0, NEST_CAVITY_Y / 2.0 + 26.0),
    ]
}

fn disposition_lane_x(lane: DispositionLane) -> f64 {
    centered_index(lane.index(), STATUS_LANE_COUNT, STATUS_LANE_PITCH_X)
}

fn fiducial_disc(name: &str) -> Part {
    let outer = centered_cylinder(format!("{name}_outer_disc"), 13.0, 4.0, 40);
    let inner = centered_cylinder(format!("{name}_center_dot"), 4.0, 5.0, 28);
    let cross_x = centered_cube(format!("{name}_cross_x"), 24.0, 2.8, 5.0);
    let cross_y = centered_cube(format!("{name}_cross_y"), 2.8, 24.0, 5.0);
    outer - inner + cross_x + cross_y
}
