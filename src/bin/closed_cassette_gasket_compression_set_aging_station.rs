use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed cassette gasket compression-set/aging validation station.
//
// Output STL manifest:
// - output/closed_cassette_gasket_compression_set_aging_station_base_leak_tray.stl
// - output/closed_cassette_gasket_compression_set_aging_station_gasket_sample_nests.stl
// - output/closed_cassette_gasket_compression_set_aging_station_compression_fixture_pockets.stl
// - output/closed_cassette_gasket_compression_set_aging_station_thickness_gauge_slots.stl
// - output/closed_cassette_gasket_compression_set_aging_station_humidity_temperature_exposure_coupon_placeholder.stl
// - output/closed_cassette_gasket_compression_set_aging_station_leak_witness_lane.stl
// - output/closed_cassette_gasket_compression_set_aging_station_lot_barcode_coa_lands.stl
// - output/closed_cassette_gasket_compression_set_aging_station_release_hold_reject_lanes.stl
// - output/closed_cassette_gasket_compression_set_aging_station_retain_sample_pockets.stl
// - output/closed_cassette_gasket_compression_set_aging_station_robot_service_keepouts.stl
// - output/closed_cassette_gasket_compression_set_aging_station_datum_and_spacer_tools.stl
// - output/closed_cassette_gasket_compression_set_aging_station_assembly.stl
//
// Product concept CAD only. This fixture represents station envelopes,
// traceability lands, sample handling, and keepouts for validation planning; it
// does not define acceptance criteria, elastomer material limits, or a release
// procedure.

const OUTPUTS: [&str; 12] = [
    "output/closed_cassette_gasket_compression_set_aging_station_base_leak_tray.stl",
    "output/closed_cassette_gasket_compression_set_aging_station_gasket_sample_nests.stl",
    "output/closed_cassette_gasket_compression_set_aging_station_compression_fixture_pockets.stl",
    "output/closed_cassette_gasket_compression_set_aging_station_thickness_gauge_slots.stl",
    "output/closed_cassette_gasket_compression_set_aging_station_humidity_temperature_exposure_coupon_placeholder.stl",
    "output/closed_cassette_gasket_compression_set_aging_station_leak_witness_lane.stl",
    "output/closed_cassette_gasket_compression_set_aging_station_lot_barcode_coa_lands.stl",
    "output/closed_cassette_gasket_compression_set_aging_station_release_hold_reject_lanes.stl",
    "output/closed_cassette_gasket_compression_set_aging_station_retain_sample_pockets.stl",
    "output/closed_cassette_gasket_compression_set_aging_station_robot_service_keepouts.stl",
    "output/closed_cassette_gasket_compression_set_aging_station_datum_and_spacer_tools.stl",
    "output/closed_cassette_gasket_compression_set_aging_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 10] = [
    "gasket_sample_nests",
    "compression_fixture_pockets",
    "thickness_gauge_slots",
    "humidity_temperature_exposure_coupon_placeholder",
    "leak_witness_lane",
    "lot_barcode_coa_lands",
    "release_hold_reject_lanes",
    "retain_sample_pockets",
    "robot_service_keepouts",
    "datum_and_spacer_tools",
];

const DECK_X: f64 = 1360.0;
const DECK_Y: f64 = 880.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const SOCKET_DEPTH: f64 = 6.0;
const MOUNT_HOLE_D: f64 = 6.8;

const SAMPLE_POS: (f64, f64) = (-428.0, 202.0);
const SAMPLE_X: f64 = 424.0;
const SAMPLE_Y: f64 = 286.0;
const SAMPLE_Z: f64 = 44.0;
const SAMPLE_ROWS: usize = 3;
const SAMPLE_COLS: usize = 4;
const SAMPLE_NEST_COUNT: usize = SAMPLE_ROWS * SAMPLE_COLS;
const SAMPLE_PITCH_X: f64 = 88.0;
const SAMPLE_PITCH_Y: f64 = 78.0;
const GASKET_OUTER_D: f64 = 58.0;
const GASKET_INNER_D: f64 = 36.0;

const COMPRESSION_POS: (f64, f64) = (34.0, 206.0);
const COMPRESSION_X: f64 = 386.0;
const COMPRESSION_Y: f64 = 286.0;
const COMPRESSION_Z: f64 = 58.0;
const COMPRESSION_ROWS: usize = 2;
const COMPRESSION_COLS: usize = 3;
const COMPRESSION_POCKET_COUNT: usize = COMPRESSION_ROWS * COMPRESSION_COLS;
const COMPRESSION_PITCH_X: f64 = 112.0;
const COMPRESSION_PITCH_Y: f64 = 112.0;
const COMPRESSION_PLATEN_D: f64 = 76.0;
const COMPRESSION_SPACER_COUNT: usize = 6;

const THICKNESS_POS: (f64, f64) = (452.0, 206.0);
const THICKNESS_X: f64 = 280.0;
const THICKNESS_Y: f64 = 286.0;
const THICKNESS_Z: f64 = 50.0;
const THICKNESS_GAUGE_ENV_X: f64 = 118.0;
const THICKNESS_GAUGE_ENV_Y: f64 = 84.0;
const THICKNESS_GAUGE_ENV_Z: f64 = 198.0;
const THICKNESS_SLOT_COUNT: usize = 8;
const REFERENCE_STEP_COUNT: usize = 6;

const EXPOSURE_POS: (f64, f64) = (-452.0, -148.0);
const EXPOSURE_X: f64 = 376.0;
const EXPOSURE_Y: f64 = 222.0;
const EXPOSURE_Z: f64 = 44.0;
const EXPOSURE_COUPON_ROWS: usize = 2;
const EXPOSURE_COUPON_COLS: usize = 4;
const EXPOSURE_COUPON_COUNT: usize = EXPOSURE_COUPON_ROWS * EXPOSURE_COUPON_COLS;
const EXPOSURE_COUPON_X: f64 = 54.0;
const EXPOSURE_COUPON_Y: f64 = 30.0;

const LEAK_POS: (f64, f64) = (-38.0, -150.0);
const LEAK_X: f64 = 402.0;
const LEAK_Y: f64 = 156.0;
const LEAK_Z: f64 = 34.0;
const LEAK_STRIP_COUNT: usize = 8;
const LEAK_STRIP_X: f64 = 38.0;
const LEAK_STRIP_Y: f64 = 86.0;

const TRACE_POS: (f64, f64) = (436.0, -112.0);
const TRACE_X: f64 = 342.0;
const TRACE_Y: f64 = 150.0;
const TRACE_Z: f64 = 12.0;
const BARCODE_LAND_COUNT: usize = 10;
const COA_LAND_COUNT: usize = 3;
const RFID_LAND_COUNT: usize = 4;

const STATUS_POS: (f64, f64) = (388.0, -330.0);
const STATUS_X: f64 = 394.0;
const STATUS_Y: f64 = 168.0;
const STATUS_Z: f64 = 46.0;
const STATUS_SLOTS_PER_LANE: usize = 4;
const STATUS_LANE_COUNT: usize = 3;

const RETAIN_POS: (f64, f64) = (-442.0, -342.0);
const RETAIN_X: f64 = 384.0;
const RETAIN_Y: f64 = 128.0;
const RETAIN_Z: f64 = 42.0;
const RETAIN_ROWS: usize = 2;
const RETAIN_COLS: usize = 5;
const RETAIN_POCKET_COUNT: usize = RETAIN_ROWS * RETAIN_COLS;

const FRONT_ROBOT_KEEP_OUT_Y: f64 = 212.0;
const REAR_SERVICE_KEEP_OUT_Y: f64 = 150.0;
const SIDE_SERVICE_KEEP_OUT_X: f64 = 94.0;
const ROBOT_PICK_CLEARANCE_Z: f64 = 268.0;
const GAUGE_SERVICE_CLEARANCE_Z: f64 = 292.0;

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
    fn fits_inside(self, deck_x: f64, deck_y: f64) -> bool {
        let usable_x = deck_x / 2.0 - RIM_W;
        let usable_y = deck_y / 2.0 - RIM_W;
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

    let base = base_leak_tray();
    export(OUTPUTS[0], &base);

    let sample_nests = gasket_sample_nests();
    export(OUTPUTS[1], &sample_nests);

    let compression = compression_fixture_pockets();
    export(OUTPUTS[2], &compression);

    let thickness = thickness_gauge_slots();
    export(OUTPUTS[3], &thickness);

    let exposure = humidity_temperature_exposure_coupon_placeholder();
    export(OUTPUTS[4], &exposure);

    let leak_lane = leak_witness_lane();
    export(OUTPUTS[5], &leak_lane);

    let traceability = lot_barcode_coa_lands();
    export(OUTPUTS[6], &traceability);

    let status_lanes = release_hold_reject_lanes();
    export(OUTPUTS[7], &status_lanes);

    let retain = retain_sample_pockets();
    export(OUTPUTS[8], &retain);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[9], &keepouts);

    let tools = datum_and_spacer_tools();
    export(OUTPUTS[10], &tools);

    let assembly = base
        + sample_nests.translate(SAMPLE_POS.0, SAMPLE_POS.1, deck_top_z())
        + compression.translate(COMPRESSION_POS.0, COMPRESSION_POS.1, deck_top_z())
        + thickness.translate(THICKNESS_POS.0, THICKNESS_POS.1, deck_top_z())
        + exposure.translate(EXPOSURE_POS.0, EXPOSURE_POS.1, deck_top_z())
        + leak_lane.translate(LEAK_POS.0, LEAK_POS.1, deck_top_z())
        + traceability.translate(TRACE_POS.0, TRACE_POS.1, deck_top_z())
        + status_lanes.translate(STATUS_POS.0, STATUS_POS.1, deck_top_z())
        + retain.translate(RETAIN_POS.0, RETAIN_POS.1, deck_top_z())
        + keepouts
        + tools.translate(0.0, 0.0, deck_top_z());
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed cassette gasket compression-set aging station:");
    println!("  Footprint:                  {DECK_X:.0}mm x {DECK_Y:.0}mm leak-tray deck");
    println!(
        "  Sample handling:            {SAMPLE_NEST_COUNT} gasket sample nests, {RETAIN_POCKET_COUNT} retain-sample pockets, {BARCODE_LAND_COUNT} barcode lands, {COA_LAND_COUNT} COA lands"
    );
    println!(
        "  Measurement workflow:       {COMPRESSION_POCKET_COUNT} compression fixture pockets, {COMPRESSION_SPACER_COUNT} spacer coupons, {THICKNESS_SLOT_COUNT} thickness gauge slots, {REFERENCE_STEP_COUNT} reference steps"
    );
    println!(
        "  Aging evidence:             {EXPOSURE_COUPON_COUNT} humidity/temperature exposure coupon placeholders and {LEAK_STRIP_COUNT} leak witness strips"
    );
    println!(
        "  Disposition lanes:          {STATUS_LANE_COUNT} release/hold/reject lanes with {STATUS_SLOTS_PER_LANE} cassette-token slots each"
    );
    println!(
        "  Keepouts:                   {FRONT_ROBOT_KEEP_OUT_Y:.0}mm front robot corridor, {REAR_SERVICE_KEEP_OUT_Y:.0}mm rear service band, {SIDE_SERVICE_KEEP_OUT_X:.0}mm side gauge service, {ROBOT_PICK_CLEARANCE_Z:.0}mm pick Z clearance"
    );
    println!("  Feature groups covered:     {}", REQUIRED_FEATURES.len());
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
    assert_eq!(SAMPLE_NEST_COUNT, 12);
    assert_eq!(COMPRESSION_POCKET_COUNT, 6);
    assert_eq!(EXPOSURE_COUPON_COUNT, 8);
    assert_eq!(RETAIN_POCKET_COUNT, 10);
    assert_eq!(STATUS_LANE_COUNT, DispositionLane::all().len());
    assert_eq!(STATUS_LANE_COUNT * STATUS_SLOTS_PER_LANE, 12);
    assert!(GASKET_OUTER_D > GASKET_INNER_D + 14.0);
    assert!(COMPRESSION_PLATEN_D > GASKET_OUTER_D + 12.0);
    assert!(THICKNESS_GAUGE_ENV_Z + THICKNESS_Z < GAUGE_SERVICE_CLEARANCE_Z);
    assert!(ROBOT_PICK_CLEARANCE_Z > THICKNESS_Z + DECK_Z + 160.0);

    let rects = layout_rects();
    for rect in rects {
        assert!(
            rect.fits_inside(DECK_X, DECK_Y),
            "{} exceeds station deck envelope",
            rect.name
        );
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

fn layout_rects() -> [Rect; 8] {
    [
        rect("gasket_sample_nests", SAMPLE_POS, SAMPLE_X, SAMPLE_Y),
        rect(
            "compression_fixture_pockets",
            COMPRESSION_POS,
            COMPRESSION_X,
            COMPRESSION_Y,
        ),
        rect(
            "thickness_gauge_slots",
            THICKNESS_POS,
            THICKNESS_X,
            THICKNESS_Y,
        ),
        rect(
            "humidity_temperature_exposure_coupon_placeholder",
            EXPOSURE_POS,
            EXPOSURE_X,
            EXPOSURE_Y,
        ),
        rect("leak_witness_lane", LEAK_POS, LEAK_X, LEAK_Y),
        rect("lot_barcode_coa_lands", TRACE_POS, TRACE_X, TRACE_Y),
        rect("release_hold_reject_lanes", STATUS_POS, STATUS_X, STATUS_Y),
        rect("retain_sample_pockets", RETAIN_POS, RETAIN_X, RETAIN_Y),
    ]
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn base_leak_tray() -> Part {
    let deck = centered_cube(
        "gasket_compression_aging_station_base_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    );
    let recessed_pan = centered_cube(
        "gasket_compression_aging_station_recessed_leak_pan",
        DECK_X - 2.0 * (RIM_W + 40.0),
        DECK_Y - 2.0 * (RIM_W + 42.0),
        8.0,
    )
    .translate(0.0, -8.0, DECK_Z / 2.0 - 4.0);
    let front_witness_gutter = centered_cube(
        "gasket_compression_aging_station_front_witness_gutter",
        DECK_X - 160.0,
        28.0,
        10.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 62.0, DECK_Z / 2.0 - 5.0);
    let drain = centered_cylinder(
        "gasket_compression_aging_station_closed_drain_port",
        9.0,
        50.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DECK_X / 2.0 - 88.0, -DECK_Y / 2.0 + 34.0, 0.0);

    deck - recessed_pan - front_witness_gutter - drain - insert_sockets() - deck_mount_holes()
        + perimeter_rim()
        + workflow_lane_marks()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("gasket_compression_aging_station_insert_sockets");
    for rect in layout_rects() {
        sockets = sockets
            + centered_cube(
                format!("gasket_compression_aging_station_socket_{}", rect.name),
                rect.x + 14.0,
                rect.y + 14.0,
                SOCKET_DEPTH + 0.6,
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
    let mut holes = Part::empty("gasket_compression_aging_station_deck_mount_holes");
    for (i, (x, y)) in mount_points().iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("gasket_compression_aging_station_m6_mount_{i}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 5.0,
                28,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn mount_points() -> [(f64, f64); 8] {
    [
        (-DECK_X / 2.0 + 52.0, -DECK_Y / 2.0 + 52.0),
        (DECK_X / 2.0 - 52.0, -DECK_Y / 2.0 + 52.0),
        (-DECK_X / 2.0 + 52.0, DECK_Y / 2.0 - 52.0),
        (DECK_X / 2.0 - 52.0, DECK_Y / 2.0 - 52.0),
        (0.0, -DECK_Y / 2.0 + 52.0),
        (0.0, DECK_Y / 2.0 - 52.0),
        (-DECK_X / 2.0 + 52.0, 0.0),
        (DECK_X / 2.0 - 52.0, 0.0),
    ]
}

fn perimeter_rim() -> Part {
    let front = centered_cube(
        "gasket_compression_aging_station_front_low_spill_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + RIM_W / 2.0, DECK_Z / 2.0 + RIM_Z / 2.0);
    let rear = centered_cube(
        "gasket_compression_aging_station_rear_service_spill_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, DECK_Z / 2.0 + RIM_Z / 2.0);
    let left = centered_cube(
        "gasket_compression_aging_station_left_sample_spill_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z / 2.0 + RIM_Z / 2.0);
    let right = centered_cube(
        "gasket_compression_aging_station_right_gauge_spill_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z / 2.0 + RIM_Z / 2.0);
    front + rear + left + right
}

fn workflow_lane_marks() -> Part {
    let incoming = centered_cube(
        "gasket_compression_aging_station_incoming_sample_flow_land",
        364.0,
        7.0,
        4.0,
    )
    .translate(-430.0, 32.0, DECK_Z / 2.0 + 2.0);
    let measure = centered_cube(
        "gasket_compression_aging_station_measurement_flow_land",
        538.0,
        7.0,
        4.0,
    )
    .translate(122.0, 42.0, DECK_Z / 2.0 + 2.0);
    let aging = centered_cube(
        "gasket_compression_aging_station_aging_witness_flow_land",
        536.0,
        7.0,
        4.0,
    )
    .rotate(0.0, 0.0, -12.0)
    .translate(-256.0, -236.0, DECK_Z / 2.0 + 2.0);
    let disposition = centered_cube(
        "gasket_compression_aging_station_disposition_flow_land",
        346.0,
        7.0,
        4.0,
    )
    .rotate(0.0, 0.0, 18.0)
    .translate(276.0, -236.0, DECK_Z / 2.0 + 2.0);
    incoming + measure + aging + disposition
}

fn gasket_sample_nests() -> Part {
    let tray = centered_cube(
        "gasket_compression_aging_sample_nest_tray",
        SAMPLE_X,
        SAMPLE_Y,
        SAMPLE_Z,
    )
    .translate(0.0, 0.0, SAMPLE_Z / 2.0);
    let shallow_basin = centered_cube(
        "gasket_compression_aging_sample_nest_shallow_basin",
        SAMPLE_X - 32.0,
        SAMPLE_Y - 30.0,
        10.0,
    )
    .translate(0.0, 0.0, SAMPLE_Z - 4.0);

    tray - shallow_basin - gasket_nest_cuts()
        + gasket_nest_rings()
        + sample_lot_label_lands()
        + sample_orientation_fences()
}

fn gasket_nest_cuts() -> Part {
    let mut cuts = Part::empty("gasket_compression_aging_sample_nest_cuts");
    for row in 0..SAMPLE_ROWS {
        for col in 0..SAMPLE_COLS {
            let index = sample_index(row, col);
            let (x, y) = sample_xy(row, col);
            let outer = centered_cylinder(
                format!("gasket_compression_aging_gasket_sample_recess_{index}"),
                GASKET_OUTER_D / 2.0 + 2.5,
                SAMPLE_Z + 4.0,
                52,
            )
            .translate(x, y, SAMPLE_Z / 2.0 + 2.0);
            let robot_finger = centered_cube(
                format!("gasket_compression_aging_gasket_sample_finger_relief_{index}"),
                18.0,
                GASKET_OUTER_D + 18.0,
                SAMPLE_Z + 4.0,
            )
            .translate(x + GASKET_OUTER_D / 2.0, y, SAMPLE_Z / 2.0 + 2.0);
            cuts = cuts + outer + robot_finger;
        }
    }
    cuts
}

fn gasket_nest_rings() -> Part {
    let mut rings = Part::empty("gasket_compression_aging_visible_gasket_sample_rings");
    for row in 0..SAMPLE_ROWS {
        for col in 0..SAMPLE_COLS {
            let index = sample_index(row, col);
            let (x, y) = sample_xy(row, col);
            let outer = centered_cylinder(
                format!("gasket_compression_aging_sample_ring_outer_{index}"),
                GASKET_OUTER_D / 2.0,
                3.0,
                56,
            );
            let inner = centered_cylinder(
                format!("gasket_compression_aging_sample_ring_inner_{index}"),
                GASKET_INNER_D / 2.0,
                4.0,
                56,
            );
            rings = rings + (outer - inner).translate(x, y, SAMPLE_Z + 1.5);
        }
    }
    rings
}

fn sample_lot_label_lands() -> Part {
    let mut lands = Part::empty("gasket_compression_aging_sample_lot_label_lands");
    for row in 0..SAMPLE_ROWS {
        for col in 0..SAMPLE_COLS {
            let index = sample_index(row, col);
            let (x, y) = sample_xy(row, col);
            lands = lands
                + centered_cube(
                    format!("gasket_compression_aging_sample_barcode_land_{index}"),
                    50.0,
                    16.0,
                    4.0,
                )
                .translate(x, y + 31.0, SAMPLE_Z + 2.0);
        }
    }
    lands
}

fn sample_orientation_fences() -> Part {
    let rear = centered_cube(
        "gasket_compression_aging_sample_nest_rear_datum_fence",
        SAMPLE_X - 42.0,
        8.0,
        24.0,
    )
    .translate(0.0, SAMPLE_Y / 2.0 - 24.0, SAMPLE_Z + 12.0);
    let left = centered_cube(
        "gasket_compression_aging_sample_nest_left_datum_fence",
        8.0,
        SAMPLE_Y - 56.0,
        24.0,
    )
    .translate(-SAMPLE_X / 2.0 + 26.0, 0.0, SAMPLE_Z + 12.0);
    let front_low = centered_cube(
        "gasket_compression_aging_sample_nest_front_robot_low_lip",
        SAMPLE_X - 86.0,
        8.0,
        14.0,
    )
    .translate(30.0, -SAMPLE_Y / 2.0 + 28.0, SAMPLE_Z + 7.0);
    rear + left + front_low
}

fn compression_fixture_pockets() -> Part {
    let block = centered_cube(
        "gasket_compression_aging_compression_fixture_block",
        COMPRESSION_X,
        COMPRESSION_Y,
        COMPRESSION_Z,
    )
    .translate(0.0, 0.0, COMPRESSION_Z / 2.0);
    let top_basin = centered_cube(
        "gasket_compression_aging_compression_fixture_top_basin",
        COMPRESSION_X - 34.0,
        COMPRESSION_Y - 34.0,
        14.0,
    )
    .translate(0.0, 0.0, COMPRESSION_Z - 5.0);

    block - top_basin - compression_pocket_cuts()
        + compression_platen_witnesses()
        + compression_spacer_magazine()
        + compression_force_frame()
}

fn compression_pocket_cuts() -> Part {
    let mut cuts = Part::empty("gasket_compression_aging_compression_fixture_pocket_cuts");
    for row in 0..COMPRESSION_ROWS {
        for col in 0..COMPRESSION_COLS {
            let index = compression_index(row, col);
            let (x, y) = compression_xy(row, col);
            let platen = centered_cylinder(
                format!("gasket_compression_aging_compression_platen_socket_{index}"),
                COMPRESSION_PLATEN_D / 2.0,
                COMPRESSION_Z + 4.0,
                60,
            )
            .translate(x, y, COMPRESSION_Z / 2.0 + 2.0);
            let clamp_slot = centered_cube(
                format!("gasket_compression_aging_compression_clamp_slide_slot_{index}"),
                COMPRESSION_PLATEN_D + 22.0,
                18.0,
                COMPRESSION_Z + 4.0,
            )
            .translate(x, y + 38.0, COMPRESSION_Z / 2.0 + 2.0);
            let removal_notch = centered_cube(
                format!("gasket_compression_aging_compression_sample_lift_notch_{index}"),
                18.0,
                COMPRESSION_PLATEN_D + 22.0,
                COMPRESSION_Z + 4.0,
            )
            .translate(x - 42.0, y, COMPRESSION_Z / 2.0 + 2.0);
            cuts = cuts + platen + clamp_slot + removal_notch;
        }
    }
    cuts
}

fn compression_platen_witnesses() -> Part {
    let mut witnesses = Part::empty("gasket_compression_aging_compression_platen_witnesses");
    for row in 0..COMPRESSION_ROWS {
        for col in 0..COMPRESSION_COLS {
            let index = compression_index(row, col);
            let (x, y) = compression_xy(row, col);
            witnesses = witnesses
                + centered_cylinder(
                    format!("gasket_compression_aging_upper_platen_envelope_{index}"),
                    (COMPRESSION_PLATEN_D / 2.0) - 7.0,
                    5.0,
                    52,
                )
                .translate(x, y, COMPRESSION_Z + 2.5);
        }
    }
    witnesses
}

fn compression_spacer_magazine() -> Part {
    let magazine = centered_cube(
        "gasket_compression_aging_compression_spacer_magazine",
        158.0,
        36.0,
        52.0,
    )
    .translate(
        COMPRESSION_X / 2.0 - 98.0,
        -COMPRESSION_Y / 2.0 + 34.0,
        COMPRESSION_Z + 26.0,
    );

    let mut slots = Part::empty("gasket_compression_aging_compression_spacer_slots");
    for i in 0..COMPRESSION_SPACER_COUNT {
        slots = slots
            + centered_cube(
                format!("gasket_compression_aging_compression_spacer_slot_{i}"),
                7.0,
                28.0,
                38.0,
            )
            .translate(
                COMPRESSION_X / 2.0 - 150.0 + i as f64 * 18.0,
                -COMPRESSION_Y / 2.0 + 34.0,
                COMPRESSION_Z + 25.0,
            );
    }
    magazine - slots
}

fn compression_force_frame() -> Part {
    let left_post = centered_cube(
        "gasket_compression_aging_force_frame_left_post",
        18.0,
        COMPRESSION_Y - 54.0,
        126.0,
    )
    .translate(-COMPRESSION_X / 2.0 + 32.0, 8.0, COMPRESSION_Z + 63.0);
    let right_post = centered_cube(
        "gasket_compression_aging_force_frame_right_post",
        18.0,
        COMPRESSION_Y - 54.0,
        126.0,
    )
    .translate(COMPRESSION_X / 2.0 - 32.0, 8.0, COMPRESSION_Z + 63.0);
    let bridge = centered_cube(
        "gasket_compression_aging_force_frame_crosshead_clearance_envelope",
        COMPRESSION_X - 58.0,
        22.0,
        28.0,
    )
    .translate(0.0, COMPRESSION_Y / 2.0 - 42.0, COMPRESSION_Z + 118.0);
    left_post + right_post + bridge
}

fn thickness_gauge_slots() -> Part {
    let dock = centered_cube(
        "gasket_compression_aging_thickness_gauge_dock",
        THICKNESS_X,
        THICKNESS_Y,
        THICKNESS_Z,
    )
    .translate(0.0, 0.0, THICKNESS_Z / 2.0);
    let gauge_foot_recess = centered_cube(
        "gasket_compression_aging_thickness_gauge_foot_recess",
        THICKNESS_GAUGE_ENV_X + 30.0,
        THICKNESS_GAUGE_ENV_Y + 28.0,
        13.0,
    )
    .translate(-8.0, 38.0, THICKNESS_Z - 4.5);
    let probe_sweep_slot = centered_cube(
        "gasket_compression_aging_thickness_probe_sweep_slot",
        THICKNESS_X - 74.0,
        26.0,
        THICKNESS_Z + 4.0,
    )
    .translate(-6.0, -70.0, THICKNESS_Z / 2.0 + 2.0);

    dock - gauge_foot_recess - probe_sweep_slot - thickness_coupon_slots()
        + thickness_gauge_envelope()
        + thickness_reference_step_block()
        + thickness_anvil_and_stop_comb()
}

fn thickness_coupon_slots() -> Part {
    let mut slots = Part::empty("gasket_compression_aging_thickness_coupon_slots");
    for i in 0..THICKNESS_SLOT_COUNT {
        slots = slots
            + centered_cube(
                format!("gasket_compression_aging_thickness_coupon_slot_{i}"),
                20.0,
                62.0,
                THICKNESS_Z + 4.0,
            )
            .translate(
                -98.0 + i as f64 * 28.0,
                -THICKNESS_Y / 2.0 + 46.0,
                THICKNESS_Z / 2.0 + 2.0,
            );
    }
    slots
}

fn thickness_gauge_envelope() -> Part {
    let body = centered_cube(
        "gasket_compression_aging_thickness_gauge_envelope_placeholder",
        THICKNESS_GAUGE_ENV_X,
        THICKNESS_GAUGE_ENV_Y,
        THICKNESS_GAUGE_ENV_Z,
    )
    .translate(-8.0, 38.0, THICKNESS_Z + THICKNESS_GAUGE_ENV_Z / 2.0);
    let display_relief = centered_cube(
        "gasket_compression_aging_thickness_gauge_display_witness",
        72.0,
        8.0,
        34.0,
    )
    .translate(-8.0, -8.0, THICKNESS_Z + 140.0);
    let data_port_relief = centered_cube(
        "gasket_compression_aging_thickness_gauge_data_port_witness",
        30.0,
        8.0,
        18.0,
    )
    .translate(48.0, -8.0, THICKNESS_Z + 82.0);
    body - display_relief - data_port_relief
}

fn thickness_reference_step_block() -> Part {
    let base = centered_cube(
        "gasket_compression_aging_thickness_reference_step_block",
        190.0,
        46.0,
        18.0,
    )
    .translate(10.0, THICKNESS_Y / 2.0 - 40.0, THICKNESS_Z + 9.0);
    let mut steps = Part::empty("gasket_compression_aging_thickness_reference_steps");
    for i in 0..REFERENCE_STEP_COUNT {
        steps = steps
            + centered_cube(
                format!("gasket_compression_aging_reference_step_{i}"),
                24.0,
                34.0,
                3.0 + i as f64 * 1.8,
            )
            .translate(
                -68.0 + i as f64 * 28.0,
                THICKNESS_Y / 2.0 - 40.0,
                THICKNESS_Z + 18.0 + (3.0 + i as f64 * 1.8) / 2.0,
            );
    }
    base + steps
}

fn thickness_anvil_and_stop_comb() -> Part {
    let anvil = centered_cylinder(
        "gasket_compression_aging_thickness_flat_anvil_land",
        24.0,
        5.0,
        48,
    )
    .translate(-90.0, -70.0, THICKNESS_Z + 2.5);
    let fence = centered_cube(
        "gasket_compression_aging_thickness_coupon_stop_comb",
        220.0,
        8.0,
        24.0,
    )
    .translate(-4.0, -THICKNESS_Y / 2.0 + 84.0, THICKNESS_Z + 12.0);
    let mut gaps = Part::empty("gasket_compression_aging_thickness_stop_comb_gaps");
    for i in 0..THICKNESS_SLOT_COUNT {
        gaps = gaps
            + centered_cube(
                format!("gasket_compression_aging_thickness_stop_comb_gap_{i}"),
                16.0,
                10.0,
                17.0,
            )
            .translate(
                -98.0 + i as f64 * 28.0,
                -THICKNESS_Y / 2.0 + 84.0,
                THICKNESS_Z + 12.0,
            );
    }
    anvil + (fence - gaps)
}

fn humidity_temperature_exposure_coupon_placeholder() -> Part {
    let carrier = centered_cube(
        "gasket_compression_aging_exposure_coupon_carrier",
        EXPOSURE_X,
        EXPOSURE_Y,
        EXPOSURE_Z,
    )
    .translate(0.0, 0.0, EXPOSURE_Z / 2.0);
    let sealed_cover_recess = centered_cube(
        "gasket_compression_aging_exposure_clear_cover_recess",
        EXPOSURE_X - 34.0,
        EXPOSURE_Y - 34.0,
        10.0,
    )
    .translate(0.0, 0.0, EXPOSURE_Z - 4.0);

    carrier
        - sealed_cover_recess
        - exposure_coupon_slots()
        - humidity_cup_recess()
        - logger_recess()
        + exposure_lid_frame()
        + humidity_temperature_sensor_envelopes()
        + exposure_coupon_witness_tabs()
}

fn exposure_coupon_slots() -> Part {
    let mut slots = Part::empty("gasket_compression_aging_exposure_coupon_slots");
    for row in 0..EXPOSURE_COUPON_ROWS {
        for col in 0..EXPOSURE_COUPON_COLS {
            let index = exposure_index(row, col);
            let x = -118.0 + col as f64 * 74.0;
            let y = -44.0 + row as f64 * 72.0;
            slots = slots
                + centered_cube(
                    format!("gasket_compression_aging_exposure_coupon_pocket_{index}"),
                    EXPOSURE_COUPON_X,
                    EXPOSURE_COUPON_Y,
                    EXPOSURE_Z + 4.0,
                )
                .translate(x, y, EXPOSURE_Z / 2.0 + 2.0);
        }
    }
    slots
}

fn humidity_cup_recess() -> Part {
    centered_cylinder(
        "gasket_compression_aging_humidity_source_cup_recess",
        33.0,
        EXPOSURE_Z + 4.0,
        48,
    )
    .translate(
        EXPOSURE_X / 2.0 - 62.0,
        -EXPOSURE_Y / 2.0 + 58.0,
        EXPOSURE_Z / 2.0 + 2.0,
    )
}

fn logger_recess() -> Part {
    let logger = centered_cube(
        "gasket_compression_aging_temp_rh_logger_recess",
        86.0,
        42.0,
        EXPOSURE_Z + 4.0,
    )
    .translate(
        EXPOSURE_X / 2.0 - 78.0,
        EXPOSURE_Y / 2.0 - 52.0,
        EXPOSURE_Z / 2.0 + 2.0,
    );
    let probe_slot = centered_cube(
        "gasket_compression_aging_temp_probe_cable_slot",
        118.0,
        9.0,
        EXPOSURE_Z + 4.0,
    )
    .translate(
        EXPOSURE_X / 2.0 - 120.0,
        EXPOSURE_Y / 2.0 - 86.0,
        EXPOSURE_Z / 2.0 + 2.0,
    );
    logger + probe_slot
}

fn exposure_lid_frame() -> Part {
    let outer = centered_cube(
        "gasket_compression_aging_exposure_lid_gasket_outer_land",
        EXPOSURE_X - 18.0,
        EXPOSURE_Y - 18.0,
        8.0,
    )
    .translate(0.0, 0.0, EXPOSURE_Z + 4.0);
    let inner = centered_cube(
        "gasket_compression_aging_exposure_lid_gasket_inner_cut",
        EXPOSURE_X - 54.0,
        EXPOSURE_Y - 54.0,
        10.0,
    )
    .translate(0.0, 0.0, EXPOSURE_Z + 4.0);
    outer - inner
}

fn humidity_temperature_sensor_envelopes() -> Part {
    let logger = centered_cube(
        "gasket_compression_aging_temp_rh_logger_placeholder",
        74.0,
        34.0,
        22.0,
    )
    .translate(
        EXPOSURE_X / 2.0 - 78.0,
        EXPOSURE_Y / 2.0 - 52.0,
        EXPOSURE_Z + 11.0,
    );
    let humidity_cup = centered_cylinder(
        "gasket_compression_aging_humidity_source_cup_placeholder",
        26.0,
        16.0,
        44,
    )
    .translate(
        EXPOSURE_X / 2.0 - 62.0,
        -EXPOSURE_Y / 2.0 + 58.0,
        EXPOSURE_Z + 8.0,
    );
    logger + humidity_cup
}

fn exposure_coupon_witness_tabs() -> Part {
    let mut tabs = Part::empty("gasket_compression_aging_exposure_coupon_witness_tabs");
    for i in 0..EXPOSURE_COUPON_COUNT {
        tabs = tabs
            + centered_cube(
                format!("gasket_compression_aging_exposure_coupon_id_tab_{i}"),
                42.0,
                12.0,
                4.0,
            )
            .translate(
                -138.0 + (i % 4) as f64 * 74.0,
                62.0 - (i / 4) as f64 * 72.0,
                EXPOSURE_Z + 2.0,
            );
    }
    tabs
}

fn leak_witness_lane() -> Part {
    let tray = centered_cube(
        "gasket_compression_aging_leak_witness_lane_tray",
        LEAK_X,
        LEAK_Y,
        LEAK_Z,
    )
    .translate(0.0, 0.0, LEAK_Z / 2.0);
    let trough = centered_cube(
        "gasket_compression_aging_leak_witness_main_trough",
        LEAK_X - 42.0,
        44.0,
        14.0,
    )
    .translate(0.0, -10.0, LEAK_Z - 5.0);
    let front_drain = centered_cube(
        "gasket_compression_aging_leak_witness_front_drain_slot",
        LEAK_X - 84.0,
        14.0,
        LEAK_Z + 4.0,
    )
    .translate(0.0, -LEAK_Y / 2.0 + 24.0, LEAK_Z / 2.0 + 2.0);

    tray - trough - front_drain - leak_strip_recesses()
        + leak_witness_strips()
        + leak_lane_end_caps()
}

fn leak_strip_recesses() -> Part {
    let mut recesses = Part::empty("gasket_compression_aging_leak_strip_recesses");
    for i in 0..LEAK_STRIP_COUNT {
        recesses = recesses
            + centered_cube(
                format!("gasket_compression_aging_leak_witness_strip_recess_{i}"),
                LEAK_STRIP_X + 4.0,
                LEAK_STRIP_Y + 4.0,
                LEAK_Z + 4.0,
            )
            .translate(-154.0 + i as f64 * 44.0, 20.0, LEAK_Z / 2.0 + 2.0);
    }
    recesses
}

fn leak_witness_strips() -> Part {
    let mut strips = Part::empty("gasket_compression_aging_leak_witness_indicator_strips");
    for i in 0..LEAK_STRIP_COUNT {
        strips = strips
            + centered_cube(
                format!("gasket_compression_aging_leak_indicator_strip_{i}"),
                LEAK_STRIP_X,
                LEAK_STRIP_Y,
                3.0,
            )
            .translate(-154.0 + i as f64 * 44.0, 20.0, LEAK_Z + 1.5);
    }
    strips
}

fn leak_lane_end_caps() -> Part {
    let inlet = centered_cube(
        "gasket_compression_aging_leak_lane_inlet_barb_placeholder",
        34.0,
        22.0,
        28.0,
    )
    .translate(-LEAK_X / 2.0 + 28.0, -12.0, LEAK_Z + 14.0);
    let outlet = centered_cube(
        "gasket_compression_aging_leak_lane_outlet_barb_placeholder",
        34.0,
        22.0,
        28.0,
    )
    .translate(LEAK_X / 2.0 - 28.0, -12.0, LEAK_Z + 14.0);
    let witness_lid = centered_cube(
        "gasket_compression_aging_leak_lane_clear_lid_placeholder",
        LEAK_X - 70.0,
        18.0,
        6.0,
    )
    .translate(0.0, LEAK_Y / 2.0 - 30.0, LEAK_Z + 3.0);
    inlet + outlet + witness_lid
}

fn lot_barcode_coa_lands() -> Part {
    let panel = centered_cube(
        "gasket_compression_aging_lot_traceability_panel",
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    )
    .translate(0.0, 0.0, TRACE_Z / 2.0);
    panel + barcode_lands() + coa_lands() + rfid_lands() + coa_clip_rails()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("gasket_compression_aging_lot_barcode_lands");
    for i in 0..BARCODE_LAND_COUNT {
        lands = lands
            + centered_cube(
                format!("gasket_compression_aging_lot_barcode_land_{i}"),
                76.0,
                20.0,
                3.0,
            )
            .translate(
                -120.0 + (i % 3) as f64 * 86.0,
                48.0 - (i / 3) as f64 * 30.0,
                TRACE_Z + 1.5,
            );
    }
    lands
}

fn coa_lands() -> Part {
    let mut lands = Part::empty("gasket_compression_aging_coa_document_lands");
    for i in 0..COA_LAND_COUNT {
        lands = lands
            + centered_cube(
                format!("gasket_compression_aging_coa_document_land_{i}"),
                104.0,
                32.0,
                3.0,
            )
            .translate(86.0, 46.0 - i as f64 * 44.0, TRACE_Z + 1.5);
    }
    lands
}

fn rfid_lands() -> Part {
    let mut lands = Part::empty("gasket_compression_aging_lot_rfid_lands");
    for i in 0..RFID_LAND_COUNT {
        lands = lands
            + centered_cylinder(
                format!("gasket_compression_aging_lot_rfid_disc_land_{i}"),
                12.0,
                3.0,
                28,
            )
            .translate(
                -150.0 + i as f64 * 32.0,
                -TRACE_Y / 2.0 + 22.0,
                TRACE_Z + 1.5,
            );
    }
    lands
}

fn coa_clip_rails() -> Part {
    let left = centered_cube(
        "gasket_compression_aging_coa_left_clip_rail",
        8.0,
        TRACE_Y - 34.0,
        18.0,
    )
    .translate(150.0, 0.0, TRACE_Z + 9.0);
    let right = centered_cube(
        "gasket_compression_aging_coa_right_clip_rail",
        8.0,
        TRACE_Y - 34.0,
        18.0,
    )
    .translate(TRACE_X / 2.0 - 22.0, 0.0, TRACE_Z + 9.0);
    left + right
}

fn release_hold_reject_lanes() -> Part {
    let base = centered_cube(
        "gasket_compression_aging_status_lane_bank",
        STATUS_X,
        STATUS_Y,
        STATUS_Z,
    )
    .translate(0.0, 0.0, STATUS_Z / 2.0);
    let lane_cuts = status_lane_recesses();
    base - lane_cuts + status_lane_dividers() + status_lane_flags() + status_token_parks()
}

fn status_lane_recesses() -> Part {
    let mut recesses = Part::empty("gasket_compression_aging_status_lane_recesses");
    for lane in DispositionLane::all() {
        let y = status_lane_y(lane);
        recesses = recesses
            + centered_cube(
                format!("gasket_compression_aging_{}_lane_recess", lane.label()),
                STATUS_X - 44.0,
                38.0,
                14.0,
            )
            .translate(0.0, y, STATUS_Z - 5.0);
    }
    recesses
}

fn status_lane_dividers() -> Part {
    let top = centered_cube(
        "gasket_compression_aging_status_lane_top_divider",
        STATUS_X - 36.0,
        6.0,
        28.0,
    )
    .translate(
        0.0,
        status_lane_y(DispositionLane::Release) - 29.0,
        STATUS_Z + 14.0,
    );
    let middle = centered_cube(
        "gasket_compression_aging_status_lane_middle_divider",
        STATUS_X - 36.0,
        6.0,
        28.0,
    )
    .translate(
        0.0,
        status_lane_y(DispositionLane::Hold) - 29.0,
        STATUS_Z + 14.0,
    );
    let bottom = centered_cube(
        "gasket_compression_aging_status_lane_bottom_stop",
        STATUS_X - 36.0,
        6.0,
        28.0,
    )
    .translate(0.0, -STATUS_Y / 2.0 + 22.0, STATUS_Z + 14.0);
    top + middle + bottom
}

fn status_lane_flags() -> Part {
    let mut flags = Part::empty("gasket_compression_aging_status_lane_label_flags");
    for lane in DispositionLane::all() {
        flags = flags
            + centered_cube(
                format!("gasket_compression_aging_{}_lane_label_land", lane.label()),
                82.0,
                18.0,
                5.0,
            )
            .translate(-STATUS_X / 2.0 + 62.0, status_lane_y(lane), STATUS_Z + 2.5);
    }
    flags
}

fn status_token_parks() -> Part {
    let mut parks = Part::empty("gasket_compression_aging_status_lane_token_parks");
    for lane in DispositionLane::all() {
        for slot in 0..STATUS_SLOTS_PER_LANE {
            parks = parks
                + centered_cube(
                    format!(
                        "gasket_compression_aging_{}_token_park_{slot}",
                        lane.label()
                    ),
                    44.0,
                    24.0,
                    5.0,
                )
                .translate(
                    -74.0 + slot as f64 * 58.0,
                    status_lane_y(lane),
                    STATUS_Z + 2.5,
                );
        }
    }
    parks
}

fn retain_sample_pockets() -> Part {
    let block = centered_cube(
        "gasket_compression_aging_retain_sample_archive_block",
        RETAIN_X,
        RETAIN_Y,
        RETAIN_Z,
    )
    .translate(0.0, 0.0, RETAIN_Z / 2.0);
    let lid_basin = centered_cube(
        "gasket_compression_aging_retain_sample_foil_lid_basin",
        RETAIN_X - 30.0,
        RETAIN_Y - 30.0,
        9.0,
    )
    .translate(0.0, 0.0, RETAIN_Z - 4.0);

    block - lid_basin - retain_pocket_cuts()
        + retain_archive_label_lands()
        + retain_lid_clip_rails()
}

fn retain_pocket_cuts() -> Part {
    let mut cuts = Part::empty("gasket_compression_aging_retain_sample_pocket_cuts");
    for row in 0..RETAIN_ROWS {
        for col in 0..RETAIN_COLS {
            let index = row * RETAIN_COLS + col;
            cuts = cuts
                + centered_cylinder(
                    format!("gasket_compression_aging_retain_sample_pocket_{index}"),
                    20.0,
                    RETAIN_Z + 4.0,
                    40,
                )
                .translate(
                    -128.0 + col as f64 * 64.0,
                    -26.0 + row as f64 * 52.0,
                    RETAIN_Z / 2.0 + 2.0,
                );
        }
    }
    cuts
}

fn retain_archive_label_lands() -> Part {
    let lot_label = centered_cube(
        "gasket_compression_aging_retain_lot_label_land",
        124.0,
        18.0,
        4.0,
    )
    .translate(
        -RETAIN_X / 2.0 + 82.0,
        RETAIN_Y / 2.0 - 20.0,
        RETAIN_Z + 2.0,
    );
    let expiry_label = centered_cube(
        "gasket_compression_aging_retain_expiry_label_land",
        124.0,
        18.0,
        4.0,
    )
    .translate(RETAIN_X / 2.0 - 82.0, RETAIN_Y / 2.0 - 20.0, RETAIN_Z + 2.0);
    lot_label + expiry_label
}

fn retain_lid_clip_rails() -> Part {
    let front = centered_cube(
        "gasket_compression_aging_retain_archive_front_lid_clip",
        RETAIN_X - 62.0,
        8.0,
        20.0,
    )
    .translate(0.0, -RETAIN_Y / 2.0 + 18.0, RETAIN_Z + 10.0);
    let rear = centered_cube(
        "gasket_compression_aging_retain_archive_rear_lid_clip",
        RETAIN_X - 62.0,
        8.0,
        20.0,
    )
    .translate(0.0, RETAIN_Y / 2.0 - 18.0, RETAIN_Z + 10.0);
    front + rear
}

fn robot_service_keepouts() -> Part {
    let front_robot_rail = centered_cube(
        "gasket_compression_aging_front_robot_keepout_rail",
        DECK_X - 180.0,
        10.0,
        34.0,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + FRONT_ROBOT_KEEP_OUT_Y,
        DECK_Z / 2.0 + 17.0,
    );
    let rear_service_rail = centered_cube(
        "gasket_compression_aging_rear_service_keepout_rail",
        DECK_X - 160.0,
        10.0,
        34.0,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - REAR_SERVICE_KEEP_OUT_Y,
        DECK_Z / 2.0 + 17.0,
    );
    let right_gauge_service_rail = centered_cube(
        "gasket_compression_aging_right_gauge_service_keepout_rail",
        10.0,
        DECK_Y - 260.0,
        34.0,
    )
    .translate(
        DECK_X / 2.0 - SIDE_SERVICE_KEEP_OUT_X,
        28.0,
        DECK_Z / 2.0 + 17.0,
    );
    let pick_clearance_bridge = centered_cube(
        "gasket_compression_aging_robot_pick_clearance_bridge",
        DECK_X - 220.0,
        18.0,
        24.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 150.0, ROBOT_PICK_CLEARANCE_Z);
    let gauge_clearance_bridge = centered_cube(
        "gasket_compression_aging_gauge_service_clearance_bridge",
        330.0,
        18.0,
        24.0,
    )
    .translate(
        THICKNESS_POS.0,
        THICKNESS_POS.1 + 132.0,
        GAUGE_SERVICE_CLEARANCE_Z,
    );
    let posts = keepout_corner_posts();
    front_robot_rail
        + rear_service_rail
        + right_gauge_service_rail
        + pick_clearance_bridge
        + gauge_clearance_bridge
        + posts
}

fn keepout_corner_posts() -> Part {
    let mut posts = Part::empty("gasket_compression_aging_keepout_corner_posts");
    for (i, (x, y, z)) in [
        (
            -DECK_X / 2.0 + 94.0,
            -DECK_Y / 2.0 + FRONT_ROBOT_KEEP_OUT_Y,
            ROBOT_PICK_CLEARANCE_Z,
        ),
        (
            DECK_X / 2.0 - SIDE_SERVICE_KEEP_OUT_X,
            -DECK_Y / 2.0 + FRONT_ROBOT_KEEP_OUT_Y,
            ROBOT_PICK_CLEARANCE_Z,
        ),
        (
            -DECK_X / 2.0 + 94.0,
            DECK_Y / 2.0 - REAR_SERVICE_KEEP_OUT_Y,
            GAUGE_SERVICE_CLEARANCE_Z,
        ),
        (
            DECK_X / 2.0 - SIDE_SERVICE_KEEP_OUT_X,
            DECK_Y / 2.0 - REAR_SERVICE_KEEP_OUT_Y,
            GAUGE_SERVICE_CLEARANCE_Z,
        ),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cylinder(
                format!("gasket_compression_aging_keepout_post_{i}"),
                7.0,
                *z - DECK_Z / 2.0,
                24,
            )
            .translate(*x, *y, (DECK_Z / 2.0 + *z) / 2.0);
    }
    posts
}

fn datum_and_spacer_tools() -> Part {
    datum_pin_grid() + compression_spacer_coupons() + go_no_go_gasket_rings()
}

fn datum_pin_grid() -> Part {
    let mut pins = Part::empty("gasket_compression_aging_station_datum_pin_grid");
    for (i, (x, y)) in [
        (-580.0, 360.0),
        (-210.0, 360.0),
        (-580.0, -396.0),
        (-210.0, -396.0),
        (220.0, 360.0),
        (584.0, 360.0),
        (220.0, -396.0),
        (584.0, -396.0),
    ]
    .iter()
    .enumerate()
    {
        pins = pins
            + centered_cylinder(
                format!("gasket_compression_aging_station_datum_pin_{i}"),
                6.0,
                26.0,
                28,
            )
            .translate(*x, *y, 13.0);
    }
    pins
}

fn compression_spacer_coupons() -> Part {
    let mut spacers = Part::empty("gasket_compression_aging_loose_compression_spacer_coupons");
    for i in 0..COMPRESSION_SPACER_COUNT {
        spacers = spacers
            + centered_cube(
                format!("gasket_compression_aging_loose_spacer_coupon_{i}"),
                22.0,
                34.0,
                3.0 + i as f64,
            )
            .translate(-44.0 + i as f64 * 18.0, -412.0, (3.0 + i as f64) / 2.0);
    }
    spacers
}

fn go_no_go_gasket_rings() -> Part {
    let go_outer = centered_cylinder(
        "gasket_compression_aging_go_ring_outer",
        GASKET_OUTER_D / 2.0 + 3.0,
        4.0,
        60,
    );
    let go_inner = centered_cylinder(
        "gasket_compression_aging_go_ring_inner",
        GASKET_INNER_D / 2.0,
        5.0,
        60,
    );
    let nogo_outer = centered_cylinder(
        "gasket_compression_aging_no_go_ring_outer",
        GASKET_OUTER_D / 2.0 - 2.0,
        4.0,
        60,
    );
    let nogo_inner = centered_cylinder(
        "gasket_compression_aging_no_go_ring_inner",
        GASKET_INNER_D / 2.0 + 3.0,
        5.0,
        60,
    );
    (go_outer - go_inner).translate(82.0, -412.0, 2.0)
        + (nogo_outer - nogo_inner).translate(148.0, -412.0, 2.0)
}

fn sample_xy(row: usize, col: usize) -> (f64, f64) {
    (
        (col as f64 - (SAMPLE_COLS as f64 - 1.0) / 2.0) * SAMPLE_PITCH_X,
        (row as f64 - (SAMPLE_ROWS as f64 - 1.0) / 2.0) * SAMPLE_PITCH_Y,
    )
}

fn sample_index(row: usize, col: usize) -> usize {
    row * SAMPLE_COLS + col
}

fn compression_xy(row: usize, col: usize) -> (f64, f64) {
    (
        (col as f64 - (COMPRESSION_COLS as f64 - 1.0) / 2.0) * COMPRESSION_PITCH_X,
        (row as f64 - (COMPRESSION_ROWS as f64 - 1.0) / 2.0) * COMPRESSION_PITCH_Y + 8.0,
    )
}

fn compression_index(row: usize, col: usize) -> usize {
    row * COMPRESSION_COLS + col
}

fn exposure_index(row: usize, col: usize) -> usize {
    row * EXPOSURE_COUPON_COLS + col
}

fn status_lane_y(lane: DispositionLane) -> f64 {
    STATUS_Y / 2.0 - 34.0 - lane.index() as f64 * 52.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_manifest_is_stable() {
        assert_eq!(OUTPUTS.len(), 12);
        assert!(OUTPUTS.iter().all(|path| path.starts_with("output/")));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
        assert!(OUTPUTS[11].contains("_assembly.stl"));
    }

    #[test]
    fn required_feature_list_is_complete() {
        for expected in [
            "gasket_sample_nests",
            "compression_fixture_pockets",
            "thickness_gauge_slots",
            "humidity_temperature_exposure_coupon_placeholder",
            "leak_witness_lane",
            "lot_barcode_coa_lands",
            "release_hold_reject_lanes",
            "retain_sample_pockets",
            "robot_service_keepouts",
        ] {
            assert!(REQUIRED_FEATURES.contains(&expected));
        }
    }

    #[test]
    fn layout_is_bounded_and_non_overlapping() {
        assert_design_constraints();
    }

    #[test]
    fn sample_and_status_counts_match_station_capacity() {
        assert_eq!(SAMPLE_NEST_COUNT, STATUS_LANE_COUNT * STATUS_SLOTS_PER_LANE);
        assert!(RETAIN_POCKET_COUNT < SAMPLE_NEST_COUNT);
        assert_eq!(COMPRESSION_POCKET_COUNT, COMPRESSION_SPACER_COUNT);
    }
}
