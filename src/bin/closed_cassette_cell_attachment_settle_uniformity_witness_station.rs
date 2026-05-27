use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed cassette cell attachment and settle-time uniformity witness station.
//
// Source-only validation CAD for the interval after robotic seeding and before
// perfusion/media exchange. The station models no-cell witness geometry for a
// 16-slot cassette inside a clean isolator: equivalent no-flow settle timing,
// leveled/tilt-adjustable datum control, vibration/transport coupons, humidity
// and evaporation witnesses, imaging fiducials, edge/center comparisons, and
// release/hold/reject disposition paths. It is fixture/interface geometry, not
// a live-cell protocol or biological acceptance criterion.

const OUTPUT_PREFIX: &str = "closed_cassette_cell_attachment_settle_uniformity_witness_station";

const OUTPUTS: [&str; 13] = [
    "output/closed_cassette_cell_attachment_settle_uniformity_witness_station_isolator_deck.stl",
    "output/closed_cassette_cell_attachment_settle_uniformity_witness_station_sixteen_slot_cassette_witness_nest.stl",
    "output/closed_cassette_cell_attachment_settle_uniformity_witness_station_leveled_tilt_adjustable_reference_plane.stl",
    "output/closed_cassette_cell_attachment_settle_uniformity_witness_station_vibration_transport_witness_coupons.stl",
    "output/closed_cassette_cell_attachment_settle_uniformity_witness_station_timed_perfusion_start_gates.stl",
    "output/closed_cassette_cell_attachment_settle_uniformity_witness_station_imaging_fiducial_bridge.stl",
    "output/closed_cassette_cell_attachment_settle_uniformity_witness_station_humidity_evaporation_witness_wells.stl",
    "output/closed_cassette_cell_attachment_settle_uniformity_witness_station_edge_center_comparison_markers.stl",
    "output/closed_cassette_cell_attachment_settle_uniformity_witness_station_release_hold_reject_disposition_lanes.stl",
    "output/closed_cassette_cell_attachment_settle_uniformity_witness_station_barcode_custody_board.stl",
    "output/closed_cassette_cell_attachment_settle_uniformity_witness_station_closed_perfusion_enable_timing_harness.stl",
    "output/closed_cassette_cell_attachment_settle_uniformity_witness_station_robot_service_keepouts.stl",
    "output/closed_cassette_cell_attachment_settle_uniformity_witness_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 12] = [
    "isolator_deck",
    "sixteen_slot_cassette_witness_nest",
    "leveled_tilt_adjustable_reference_plane",
    "vibration_transport_witness_coupons",
    "timed_perfusion_start_gates",
    "imaging_fiducial_bridge",
    "humidity_evaporation_witness_wells",
    "edge_center_comparison_markers",
    "release_hold_reject_disposition_lanes",
    "barcode_custody_board",
    "closed_perfusion_enable_timing_harness",
    "robot_service_keepouts",
];

const DECK_X: f64 = 1580.0;
const DECK_Y: f64 = 980.0;
const DECK_Z: f64 = 22.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 42.0;
const BASIN_X: f64 = 1370.0;
const BASIN_Y: f64 = 790.0;
const BASIN_DEPTH: f64 = 7.0;
const DRAIN_CHANNEL_W: f64 = 14.0;
const MOUNT_HOLE_D: f64 = 6.8;
const ISOLATOR_LOCATOR_COUNT: usize = 4;
const CLEAN_WIPE_RIBS: usize = 6;

const SLOT_COLS: usize = 4;
const SLOT_ROWS: usize = 4;
const SLOT_COUNT: usize = SLOT_COLS * SLOT_ROWS;
const EDGE_SLOT_COUNT: usize = 12;
const CENTER_SLOT_COUNT: usize = SLOT_COUNT - EDGE_SLOT_COUNT;
const SLOT_GAP_X: f64 = 20.0;
const SLOT_GAP_Y: f64 = 18.0;
const SLOT_PITCH_X: f64 = REVC_CHIP_LENGTH + SLOT_GAP_X;
const SLOT_PITCH_Y: f64 = REVC_CHIP_WIDTH + SLOT_GAP_Y;
const SLOT_ARRAY_X: f64 =
    SLOT_COLS as f64 * REVC_CHIP_LENGTH + (SLOT_COLS as f64 - 1.0) * SLOT_GAP_X;
const SLOT_ARRAY_Y: f64 =
    SLOT_ROWS as f64 * REVC_CHIP_WIDTH + (SLOT_ROWS as f64 - 1.0) * SLOT_GAP_Y;

const NEST_CENTER: (f64, f64) = (-330.0, 80.0);
const NEST_MARGIN_X: f64 = 46.0;
const NEST_MARGIN_Y: f64 = 48.0;
const NEST_X: f64 = SLOT_ARRAY_X + 2.0 * NEST_MARGIN_X;
const NEST_Y: f64 = SLOT_ARRAY_Y + 2.0 * NEST_MARGIN_Y;
const NEST_Z: f64 = 36.0;
const SLOT_POCKET_X: f64 = REVC_CHIP_LENGTH + 8.0;
const SLOT_POCKET_Y: f64 = REVC_CHIP_WIDTH + 8.0;
const SLOT_RECESS_DEPTH: f64 = 12.0;
const SLOT_RIM_W: f64 = 7.0;
const SLOT_HARD_STOP_Z: f64 = 11.0;
const SLOT_ID_TABS_PER_SLOT: usize = 2;
const SLOT_SETTLE_WINDOW_X: f64 = REVC_CHIP_LENGTH * 0.46;
const SLOT_SETTLE_WINDOW_Y: f64 = REVC_CHIP_WIDTH * 0.36;
const NEST_LOCATOR_PIN_D: f64 = 8.0;
const NEST_CLAMP_COUNT: usize = SLOT_ROWS * 2;

const REF_PLANE_Z: f64 = 12.0;
const REF_PLANE_FRAME_W: f64 = 26.0;
const LEVELING_FEET: usize = 4;
const TILT_WEDGE_COUNT: usize = 4;
const TILT_TICK_COUNT: usize = 9;
const LEVELING_SCREW_D: f64 = 14.0;
const LEVELING_PAD_D: f64 = 30.0;
const TILT_WEDGE_X: f64 = 86.0;
const TILT_WEDGE_Y: f64 = 34.0;
const TILT_WEDGE_Z: f64 = 16.0;
const BUBBLE_LEVEL_X: f64 = 176.0;
const BUBBLE_LEVEL_Y: f64 = 34.0;
const BUBBLE_LEVEL_Z: f64 = 18.0;

const VIB_CENTER: (f64, f64) = (420.0, 30.0);
const VIB_X: f64 = 470.0;
const VIB_Y: f64 = 220.0;
const VIB_Z: f64 = 24.0;
const VIBRATION_COUPON_COUNT: usize = SLOT_COUNT;
const VIBRATION_COUPON_X: f64 = 72.0;
const VIBRATION_COUPON_Y: f64 = 28.0;
const ACCELEROMETER_PODS: usize = 4;
const ACCELEROMETER_POD_X: f64 = 66.0;
const ACCELEROMETER_POD_Y: f64 = 44.0;
const TRANSPORT_WITNESS_COMB_TICKS: usize = 7;

const GATE_CENTER: (f64, f64) = (410.0, 275.0);
const GATE_X: f64 = 460.0;
const GATE_Y: f64 = 185.0;
const GATE_Z: f64 = 28.0;
const PERFUSION_GATE_COUNT: usize = SLOT_COUNT;
const GATE_COLS: usize = 8;
const GATE_ROWS: usize = 2;
const GATE_LANE_X: f64 = 42.0;
const GATE_LANE_Y: f64 = 54.0;
const GATE_TOKEN_SLOT_X: f64 = 28.0;
const GATE_TOKEN_SLOT_Y: f64 = 17.0;
const GATE_VALVE_BOSS_D: f64 = 17.0;
const SETTLE_TOKEN_COUNT: usize = 8;
const SETTLE_MINUTES: [usize; SETTLE_TOKEN_COUNT] = [0, 10, 20, 30, 45, 60, 90, 120];
const PERFUSION_GATE_SLOT_MAP: [usize; PERFUSION_GATE_COUNT] =
    [0, 4, 8, 12, 1, 5, 9, 13, 2, 6, 10, 14, 3, 7, 11, 15];

const BRIDGE_POST_X: f64 = 30.0;
const BRIDGE_POST_Y: f64 = 42.0;
const BRIDGE_BEAM_W: f64 = 26.0;
const BRIDGE_BEAM_Z: f64 = 26.0;
const BRIDGE_UNDERSIDE_Z: f64 = 218.0;
const BRIDGE_CAMERA_PODS: usize = 4;
const CAMERA_POD_X: f64 = 86.0;
const CAMERA_POD_Y: f64 = 58.0;
const CAMERA_POD_Z: f64 = 38.0;
const FIDUCIAL_TARGETS: usize = 8;
const FIDUCIAL_D: f64 = 18.0;

const HUM_CENTER: (f64, f64) = (-560.0, -340.0);
const HUM_X: f64 = 340.0;
const HUM_Y: f64 = 180.0;
const HUM_Z: f64 = 22.0;
const SLOT_PAIRED_HUMIDITY_WELLS: usize = SLOT_COUNT;
const CORNER_HUMIDITY_REFERENCES: usize = 4;
const HUMIDITY_WELL_COUNT: usize = SLOT_PAIRED_HUMIDITY_WELLS + CORNER_HUMIDITY_REFERENCES;
const HUMIDITY_WELL_D: f64 = 19.0;
const HUMIDITY_WELL_RIM_D: f64 = 29.0;
const HUMIDITY_WELL_DEPTH: f64 = 10.0;
const EVAPORATION_BALANCE_BARS: usize = 5;

const EDGE_MARKER_D: f64 = 15.0;
const CENTER_MARKER_D: f64 = 24.0;
const MARKER_Z: f64 = 5.0;
const EDGE_CENTER_STRIP_Z: f64 = 3.0;

const DISPOSITION_CENTER: (f64, f64) = (450.0, -315.0);
const DISPOSITION_X: f64 = 360.0;
const DISPOSITION_Y: f64 = 170.0;
const DISPOSITION_Z: f64 = 26.0;
const DISPOSITION_LANES: usize = 3;
const DISPOSITION_TOKENS_PER_LANE: usize = 5;
const DISPOSITION_LANE_X: f64 = 96.0;
const DISPOSITION_LANE_Y: f64 = 126.0;
const QUARANTINE_POCKET_X: f64 = 74.0;
const QUARANTINE_POCKET_Y: f64 = 56.0;

const CUSTODY_CENTER: (f64, f64) = (0.0, -365.0);
const CUSTODY_X: f64 = 350.0;
const CUSTODY_Y: f64 = 140.0;
const CUSTODY_Z: f64 = 18.0;
const BARCODE_LANDS: usize = SLOT_COUNT;
const RFID_LANDS: usize = 4;
const TIMESTAMP_TICKS: usize = 8;
const CUSTODY_FIDUCIALS: usize = 4;

const HARNESS_Z: f64 = 9.0;
const HARNESS_TUBE_W: f64 = 6.0;
const HARNESS_MANIFOLD_X: f64 = 24.0;
const HARNESS_MANIFOLD_Y: f64 = NEST_Y + 82.0;
const DRY_BREAK_COUPLERS: usize = SLOT_ROWS;
const ENABLE_BUS_MARKERS: usize = PERFUSION_GATE_COUNT;

const KEEP_OUT_X: f64 = 1500.0;
const KEEP_OUT_Y: f64 = 905.0;
const KEEP_OUT_RAIL_Z: f64 = 6.0;
const FRONT_ROBOT_CLEARANCE: f64 = 420.0;
const REAR_SERVICE_CLEARANCE: f64 = 270.0;
const LEFT_CASSETTE_SERVICE_CLEARANCE: f64 = 250.0;
const RIGHT_GATE_SERVICE_CLEARANCE: f64 = 260.0;
const TOP_IMAGING_CLEARANCE: f64 = 260.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_deck(self) -> bool {
        let usable_x = DECK_X / 2.0 - RIM_W - 16.0;
        let usable_y = DECK_Y / 2.0 - RIM_W - 16.0;

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
    assert_design_constraints();

    let deck = isolator_deck();
    export(OUTPUTS[0], &deck);

    let nest = sixteen_slot_cassette_witness_nest();
    export(OUTPUTS[1], &nest);

    let reference = leveled_tilt_adjustable_reference_plane();
    export(OUTPUTS[2], &reference);

    let vibration = vibration_transport_witness_coupons();
    export(OUTPUTS[3], &vibration);

    let gates = timed_perfusion_start_gates();
    export(OUTPUTS[4], &gates);

    let bridge = imaging_fiducial_bridge();
    export(OUTPUTS[5], &bridge);

    let humidity = humidity_evaporation_witness_wells();
    export(OUTPUTS[6], &humidity);

    let markers = edge_center_comparison_markers();
    export(OUTPUTS[7], &markers);

    let disposition = release_hold_reject_disposition_lanes();
    export(OUTPUTS[8], &disposition);

    let custody = barcode_custody_board();
    export(OUTPUTS[9], &custody);

    let harness = closed_perfusion_enable_timing_harness();
    export(OUTPUTS[10], &harness);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[11], &keepouts);

    let assembly = station_assembly();
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed cassette cell attachment settle uniformity witness station:");
    println!(
        "  Cassette witness nest:       {SLOT_COLS}x{SLOT_ROWS} no-flow slots ({SLOT_COUNT} total) on {:.0}mm x {:.0}mm nest",
        NEST_X, NEST_Y
    );
    println!(
        "  Attachment timing controls:  {PERFUSION_GATE_COUNT} timed perfusion gates mapped one-to-one to slots, settle tokens {:?} min",
        SETTLE_MINUTES
    );
    println!(
        "  Tilt/vibration controls:     {LEVELING_FEET} leveling feet, {TILT_WEDGE_COUNT} tilt wedges, {VIBRATION_COUPON_COUNT} transport coupons, {ACCELEROMETER_PODS} accelerometer pods"
    );
    println!(
        "  Humidity witnesses:          {SLOT_PAIRED_HUMIDITY_WELLS} slot-paired wells plus {CORNER_HUMIDITY_REFERENCES} corner references"
    );
    println!(
        "  Edge/center comparison:      {EDGE_SLOT_COUNT} edge markers and {CENTER_SLOT_COUNT} center markers with imaging bridge clearance {:.0}mm",
        bridge_clearance_above_chip()
    );
    println!(
        "  Disposition/custody:         release/hold/reject lanes, {BARCODE_LANDS} barcode lands, {RFID_LANDS} RFID lands"
    );
    println!("  STL outputs:                 {} files", OUTPUTS.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    isolator_deck()
        + sixteen_slot_cassette_witness_nest()
        + leveled_tilt_adjustable_reference_plane()
        + vibration_transport_witness_coupons()
        + timed_perfusion_start_gates()
        + imaging_fiducial_bridge()
        + humidity_evaporation_witness_wells()
        + edge_center_comparison_markers()
        + release_hold_reject_disposition_lanes()
        + barcode_custody_board()
        + closed_perfusion_enable_timing_harness()
        + robot_service_keepouts()
}

fn isolator_deck() -> Part {
    let deck = centered_cube(
        format!("{OUTPUT_PREFIX}_deck_plate"),
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let basin = centered_cube(
        format!("{OUTPUT_PREFIX}_shallow_closed_station_basin_cut"),
        BASIN_X,
        BASIN_Y,
        BASIN_DEPTH + 2.0,
    )
    .translate(0.0, -8.0, DECK_Z - BASIN_DEPTH / 2.0 + 0.5);
    let drain = centered_cube(
        format!("{OUTPUT_PREFIX}_front_witness_drain_channel_cut"),
        BASIN_X - 90.0,
        DRAIN_CHANNEL_W,
        BASIN_DEPTH + 2.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 82.0, DECK_Z - BASIN_DEPTH / 2.0 + 0.5);

    deck - basin - drain - deck_mount_holes()
        + deck_perimeter_rims()
        + isolator_locator_pads()
        + clean_wipe_ribs()
        + module_floor_markers()
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty(format!("{OUTPUT_PREFIX}_deck_mount_holes"));
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
    .into_iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_deck_m6_mount_hole_{i}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 4.0,
                28,
            )
            .translate(x, y, DECK_Z / 2.0);
    }
    holes
}

fn deck_perimeter_rims() -> Part {
    let front = centered_cube(
        format!("{OUTPUT_PREFIX}_front_clean_wipe_rim"),
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        format!("{OUTPUT_PREFIX}_rear_clean_wipe_rim"),
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let left = centered_cube(
        format!("{OUTPUT_PREFIX}_left_clean_wipe_rim"),
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube(
        format!("{OUTPUT_PREFIX}_right_clean_wipe_rim"),
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);

    front + rear + left + right
}

fn isolator_locator_pads() -> Part {
    let mut pads = Part::empty(format!("{OUTPUT_PREFIX}_isolator_locator_pads"));
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 96.0, -DECK_Y / 2.0 + 96.0),
        (DECK_X / 2.0 - 96.0, -DECK_Y / 2.0 + 96.0),
        (-DECK_X / 2.0 + 96.0, DECK_Y / 2.0 - 96.0),
        (DECK_X / 2.0 - 96.0, DECK_Y / 2.0 - 96.0),
    ]
    .into_iter()
    .enumerate()
    {
        let pad = centered_cylinder(
            format!("{OUTPUT_PREFIX}_clean_isolator_locator_pad_{i}"),
            30.0,
            8.0,
            36,
        )
        .translate(x, y, DECK_Z + 4.0);
        let key = centered_cube(
            format!("{OUTPUT_PREFIX}_clean_isolator_locator_key_flat_{i}"),
            46.0,
            10.0,
            5.0,
        )
        .translate(x, y, DECK_Z + 10.5);
        pads = pads + pad + key;
    }
    pads
}

fn clean_wipe_ribs() -> Part {
    let mut ribs = Part::empty(format!("{OUTPUT_PREFIX}_clean_wipe_reference_ribs"));
    for i in 0..CLEAN_WIPE_RIBS {
        ribs = ribs
            + centered_cube(
                format!("{OUTPUT_PREFIX}_clean_wipe_rib_{i}"),
                118.0,
                6.0,
                5.0,
            )
            .translate(
                centered_index(i, CLEAN_WIPE_RIBS, 172.0),
                -DECK_Y / 2.0 + 126.0,
                DECK_Z + 2.5,
            );
    }
    ribs
}

fn module_floor_markers() -> Part {
    let mut markers = Part::empty(format!("{OUTPUT_PREFIX}_module_floor_markers"));
    for rect in primary_rects() {
        markers = markers
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{}_floor_shadow", rect.name),
                rect.x + 12.0,
                rect.y + 12.0,
                3.0,
            )
            .translate(rect.center.0, rect.center.1, DECK_Z + 1.5);
    }
    markers
}

fn sixteen_slot_cassette_witness_nest() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_cassette_nest_body"),
        NEST_X,
        NEST_Y,
        NEST_Z,
    )
    .translate(0.0, 0.0, NEST_Z / 2.0);

    (body - slot_recesses() - datum_probe_grooves()
        + slot_rims()
        + slot_hard_stops()
        + slot_settle_windows()
        + nest_locator_pins()
        + nest_edge_clamps()
        + slot_identity_tabs())
    .translate(NEST_CENTER.0, NEST_CENTER.1, DECK_Z)
}

fn slot_recesses() -> Part {
    let mut recesses = Part::empty(format!("{OUTPUT_PREFIX}_slot_recesses"));
    for slot in 0..SLOT_COUNT {
        let (x, y) = slot_local_center(slot);
        recesses = recesses
            + centered_cube(
                format!("{OUTPUT_PREFIX}_slot_{slot:02}_closed_chip_settle_recess"),
                SLOT_POCKET_X,
                SLOT_POCKET_Y,
                SLOT_RECESS_DEPTH + 1.0,
            )
            .translate(x, y, NEST_Z - SLOT_RECESS_DEPTH / 2.0 + 0.5);
    }
    recesses
}

fn slot_rims() -> Part {
    let mut rims = Part::empty(format!("{OUTPUT_PREFIX}_slot_wipeable_rims"));
    for slot in 0..SLOT_COUNT {
        let (x, y) = slot_local_center(slot);
        let front = centered_cube(
            format!("{OUTPUT_PREFIX}_slot_{slot:02}_front_settle_rim"),
            SLOT_POCKET_X + SLOT_RIM_W * 2.0,
            SLOT_RIM_W,
            7.0,
        )
        .translate(x, y - SLOT_POCKET_Y / 2.0 - SLOT_RIM_W / 2.0, NEST_Z + 3.5);
        let rear = centered_cube(
            format!("{OUTPUT_PREFIX}_slot_{slot:02}_rear_settle_rim"),
            SLOT_POCKET_X + SLOT_RIM_W * 2.0,
            SLOT_RIM_W,
            7.0,
        )
        .translate(x, y + SLOT_POCKET_Y / 2.0 + SLOT_RIM_W / 2.0, NEST_Z + 3.5);
        let left = centered_cube(
            format!("{OUTPUT_PREFIX}_slot_{slot:02}_left_settle_rim"),
            SLOT_RIM_W,
            SLOT_POCKET_Y,
            7.0,
        )
        .translate(x - SLOT_POCKET_X / 2.0 - SLOT_RIM_W / 2.0, y, NEST_Z + 3.5);
        let right = centered_cube(
            format!("{OUTPUT_PREFIX}_slot_{slot:02}_right_settle_rim"),
            SLOT_RIM_W,
            SLOT_POCKET_Y,
            7.0,
        )
        .translate(x + SLOT_POCKET_X / 2.0 + SLOT_RIM_W / 2.0, y, NEST_Z + 3.5);
        rims = rims + front + rear + left + right;
    }
    rims
}

fn slot_hard_stops() -> Part {
    let mut stops = Part::empty(format!("{OUTPUT_PREFIX}_slot_hard_stops"));
    for slot in 0..SLOT_COUNT {
        let (x, y) = slot_local_center(slot);
        let rear = centered_cube(
            format!("{OUTPUT_PREFIX}_slot_{slot:02}_datum_back_stop"),
            SLOT_POCKET_X * 0.48,
            8.0,
            SLOT_HARD_STOP_Z,
        )
        .translate(
            x,
            y + SLOT_POCKET_Y / 2.0 + 18.0,
            NEST_Z + SLOT_HARD_STOP_Z / 2.0,
        );
        let left = centered_cube(
            format!("{OUTPUT_PREFIX}_slot_{slot:02}_datum_side_stop"),
            8.0,
            SLOT_POCKET_Y * 0.38,
            SLOT_HARD_STOP_Z,
        )
        .translate(
            x - SLOT_POCKET_X / 2.0 - 18.0,
            y,
            NEST_Z + SLOT_HARD_STOP_Z / 2.0,
        );
        stops = stops + rear + left;
    }
    stops
}

fn slot_settle_windows() -> Part {
    let mut windows = Part::empty(format!("{OUTPUT_PREFIX}_slot_no_flow_settle_windows"));
    for slot in 0..SLOT_COUNT {
        let (x, y) = slot_local_center(slot);
        let window = centered_cube(
            format!("{OUTPUT_PREFIX}_slot_{slot:02}_transparent_settle_witness_land"),
            SLOT_SETTLE_WINDOW_X,
            SLOT_SETTLE_WINDOW_Y,
            4.0,
        )
        .translate(x, y, NEST_Z + 2.0);
        let centerline_x = centered_cube(
            format!("{OUTPUT_PREFIX}_slot_{slot:02}_settle_centerline_x"),
            SLOT_SETTLE_WINDOW_X,
            2.0,
            5.0,
        )
        .translate(x, y, NEST_Z + 4.5);
        let centerline_y = centered_cube(
            format!("{OUTPUT_PREFIX}_slot_{slot:02}_settle_centerline_y"),
            2.0,
            SLOT_SETTLE_WINDOW_Y,
            5.0,
        )
        .translate(x, y, NEST_Z + 4.5);
        windows = windows + window + centerline_x + centerline_y;
    }
    windows
}

fn datum_probe_grooves() -> Part {
    let x_groove = centered_cube(
        format!("{OUTPUT_PREFIX}_nest_x_level_probe_groove"),
        NEST_X - 72.0,
        5.0,
        4.0,
    )
    .translate(0.0, 0.0, NEST_Z - 1.4);
    let y_groove = centered_cube(
        format!("{OUTPUT_PREFIX}_nest_y_level_probe_groove"),
        5.0,
        NEST_Y - 72.0,
        4.0,
    )
    .translate(0.0, 0.0, NEST_Z - 1.4);

    x_groove + y_groove
}

fn nest_locator_pins() -> Part {
    let mut pins = Part::empty(format!("{OUTPUT_PREFIX}_nest_locator_pins"));
    for (i, (x, y)) in [
        (-NEST_X / 2.0 + 30.0, -NEST_Y / 2.0 + 30.0),
        (NEST_X / 2.0 - 30.0, -NEST_Y / 2.0 + 30.0),
        (-NEST_X / 2.0 + 30.0, NEST_Y / 2.0 - 30.0),
        (NEST_X / 2.0 - 30.0, NEST_Y / 2.0 - 30.0),
    ]
    .into_iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("{OUTPUT_PREFIX}_nest_locator_boss_{i}"),
            NEST_LOCATOR_PIN_D / 2.0 + 6.0,
            6.0,
            32,
        )
        .translate(x, y, NEST_Z + 3.0);
        let pin = centered_cylinder(
            format!("{OUTPUT_PREFIX}_nest_locator_pin_{i}"),
            NEST_LOCATOR_PIN_D / 2.0,
            16.0,
            32,
        )
        .translate(x, y, NEST_Z + 8.0);
        pins = pins + boss + pin;
    }
    pins
}

fn nest_edge_clamps() -> Part {
    let mut clamps = Part::empty(format!("{OUTPUT_PREFIX}_nest_edge_clamps"));
    for row in 0..SLOT_ROWS {
        let y = centered_index(row, SLOT_ROWS, SLOT_PITCH_Y);
        for (side, sx) in [("left", -1.0), ("right", 1.0)] {
            clamps = clamps
                + centered_cube(
                    format!("{OUTPUT_PREFIX}_{side}_row_{row}_soft_settle_clamp"),
                    34.0,
                    58.0,
                    18.0,
                )
                .translate(sx * (NEST_X / 2.0 - 19.0), y, NEST_Z + 9.0);
        }
    }
    clamps
}

fn slot_identity_tabs() -> Part {
    let mut tabs = Part::empty(format!("{OUTPUT_PREFIX}_slot_identity_tabs"));
    for slot in 0..SLOT_COUNT {
        let (x, y) = slot_local_center(slot);
        for tick in 0..SLOT_ID_TABS_PER_SLOT {
            tabs = tabs
                + centered_cube(
                    format!("{OUTPUT_PREFIX}_slot_{slot:02}_identity_tick_{tick}"),
                    7.0 + slot as f64 * 0.45,
                    5.0,
                    5.0,
                )
                .translate(
                    x - SLOT_POCKET_X / 2.0 + 16.0 + tick as f64 * 12.0,
                    y + SLOT_POCKET_Y / 2.0 + 17.0,
                    NEST_Z + 2.5,
                );
        }
    }
    tabs
}

fn leveled_tilt_adjustable_reference_plane() -> Part {
    let frame = centered_cube(
        format!("{OUTPUT_PREFIX}_leveled_reference_plane_frame"),
        NEST_X + 78.0,
        NEST_Y + 78.0,
        REF_PLANE_Z,
    )
    .translate(
        NEST_CENTER.0,
        NEST_CENTER.1,
        reference_plane_z() + REF_PLANE_Z / 2.0,
    );
    let window = centered_cube(
        format!("{OUTPUT_PREFIX}_leveled_reference_plane_cassette_window_cut"),
        NEST_X - REF_PLANE_FRAME_W,
        NEST_Y - REF_PLANE_FRAME_W,
        REF_PLANE_Z + 2.0,
    )
    .translate(
        NEST_CENTER.0,
        NEST_CENTER.1,
        reference_plane_z() + REF_PLANE_Z / 2.0,
    );

    frame - window
        + leveling_screw_feet()
        + tilt_reference_wedges()
        + bubble_level_carriers()
        + tilt_tick_ladders()
}

fn leveling_screw_feet() -> Part {
    let mut feet = Part::empty(format!("{OUTPUT_PREFIX}_leveling_screw_feet"));
    for (i, (x, y)) in reference_corner_points(42.0).into_iter().enumerate() {
        let pad = centered_cylinder(
            format!("{OUTPUT_PREFIX}_leveling_foot_pad_{i}"),
            LEVELING_PAD_D / 2.0,
            8.0,
            36,
        )
        .translate(x, y, DECK_Z + 4.0);
        let screw = centered_cylinder(
            format!("{OUTPUT_PREFIX}_tilt_adjust_screw_{i}"),
            LEVELING_SCREW_D / 2.0,
            reference_plane_z() - DECK_Z + 20.0,
            36,
        )
        .translate(x, y, DECK_Z + (reference_plane_z() - DECK_Z + 20.0) / 2.0);
        feet = feet + pad + screw;
    }
    feet
}

fn tilt_reference_wedges() -> Part {
    let mut wedges = Part::empty(format!("{OUTPUT_PREFIX}_tilt_reference_wedges"));
    for (i, (x, y)) in [
        (NEST_CENTER.0 - NEST_X / 2.0 - 56.0, NEST_CENTER.1),
        (NEST_CENTER.0 + NEST_X / 2.0 + 56.0, NEST_CENTER.1),
        (NEST_CENTER.0, NEST_CENTER.1 - NEST_Y / 2.0 - 54.0),
        (NEST_CENTER.0, NEST_CENTER.1 + NEST_Y / 2.0 + 54.0),
    ]
    .into_iter()
    .enumerate()
    {
        let wedge = centered_cube(
            format!("{OUTPUT_PREFIX}_known_tilt_wedge_{i}"),
            TILT_WEDGE_X,
            TILT_WEDGE_Y,
            TILT_WEDGE_Z,
        )
        .translate(x, y, DECK_Z + TILT_WEDGE_Z / 2.0);
        let high_edge = centered_cube(
            format!("{OUTPUT_PREFIX}_known_tilt_wedge_{i}_high_edge"),
            TILT_WEDGE_X,
            5.0,
            TILT_WEDGE_Z + 8.0,
        )
        .translate(
            x,
            y + TILT_WEDGE_Y / 2.0 - 2.5,
            DECK_Z + TILT_WEDGE_Z / 2.0 + 4.0,
        );
        wedges = wedges + wedge + high_edge;
    }
    wedges
}

fn bubble_level_carriers() -> Part {
    let x_level = centered_cube(
        format!("{OUTPUT_PREFIX}_x_axis_bubble_level_carrier"),
        BUBBLE_LEVEL_X,
        BUBBLE_LEVEL_Y,
        BUBBLE_LEVEL_Z,
    )
    .translate(
        NEST_CENTER.0,
        NEST_CENTER.1 + NEST_Y / 2.0 + 42.0,
        reference_plane_z() + BUBBLE_LEVEL_Z / 2.0,
    );
    let y_level = centered_cube(
        format!("{OUTPUT_PREFIX}_y_axis_bubble_level_carrier"),
        BUBBLE_LEVEL_Y,
        BUBBLE_LEVEL_X,
        BUBBLE_LEVEL_Z,
    )
    .translate(
        NEST_CENTER.0 - NEST_X / 2.0 - 42.0,
        NEST_CENTER.1,
        reference_plane_z() + BUBBLE_LEVEL_Z / 2.0,
    );

    x_level + y_level
}

fn tilt_tick_ladders() -> Part {
    let mut ticks = Part::empty(format!("{OUTPUT_PREFIX}_tilt_tick_ladders"));
    for axis in 0..2 {
        for i in 0..TILT_TICK_COUNT {
            let length = 16.0 + i as f64 * 4.0;
            let (x, y, sx, sy) = if axis == 0 {
                (
                    NEST_CENTER.0 - 160.0 + i as f64 * 40.0,
                    NEST_CENTER.1 + NEST_Y / 2.0 + 72.0,
                    5.0,
                    length,
                )
            } else {
                (
                    NEST_CENTER.0 - NEST_X / 2.0 - 72.0,
                    NEST_CENTER.1 - 160.0 + i as f64 * 40.0,
                    length,
                    5.0,
                )
            };
            ticks = ticks
                + centered_cube(
                    format!("{OUTPUT_PREFIX}_tilt_axis_{axis}_tick_{i}"),
                    sx,
                    sy,
                    5.0,
                )
                .translate(x, y, reference_plane_z() + REF_PLANE_Z + 2.5);
        }
    }
    ticks
}

fn vibration_transport_witness_coupons() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_vibration_transport_coupon_panel"),
        VIB_X,
        VIB_Y,
        VIB_Z,
    )
    .translate(VIB_CENTER.0, VIB_CENTER.1, DECK_Z + VIB_Z / 2.0);

    body - vibration_coupon_reliefs()
        + vibration_coupon_tabs()
        + accelerometer_pods()
        + transport_direction_combs()
        + transport_lock_witness_pins()
}

fn vibration_coupon_reliefs() -> Part {
    let mut reliefs = Part::empty(format!("{OUTPUT_PREFIX}_vibration_coupon_reliefs"));
    for coupon in 0..VIBRATION_COUPON_COUNT {
        let (x, y) = vibration_coupon_position(coupon);
        reliefs = reliefs
            + centered_cube(
                format!("{OUTPUT_PREFIX}_slot_{coupon:02}_vibration_coupon_recess"),
                VIBRATION_COUPON_X,
                VIBRATION_COUPON_Y,
                7.0,
            )
            .translate(x, y, DECK_Z + VIB_Z - 2.8);
    }
    reliefs
}

fn vibration_coupon_tabs() -> Part {
    let mut coupons = Part::empty(format!("{OUTPUT_PREFIX}_vibration_coupon_tabs"));
    for coupon in 0..VIBRATION_COUPON_COUNT {
        let (x, y) = vibration_coupon_position(coupon);
        let tab = centered_cube(
            format!("{OUTPUT_PREFIX}_slot_{coupon:02}_transport_witness_coupon"),
            VIBRATION_COUPON_X - 10.0,
            VIBRATION_COUPON_Y - 8.0,
            5.0,
        )
        .translate(x, y, DECK_Z + VIB_Z + 2.5);
        let shear_dot = centered_cylinder(
            format!("{OUTPUT_PREFIX}_slot_{coupon:02}_shear_dot"),
            5.0,
            5.5,
            24,
        )
        .translate(
            x + VIBRATION_COUPON_X / 2.0 - 18.0,
            y,
            DECK_Z + VIB_Z + 2.75,
        );
        coupons = coupons + tab + shear_dot;
    }
    coupons
}

fn accelerometer_pods() -> Part {
    let mut pods = Part::empty(format!("{OUTPUT_PREFIX}_accelerometer_tilt_pods"));
    for (i, (x, y)) in [
        (
            VIB_CENTER.0 - VIB_X / 2.0 + 56.0,
            VIB_CENTER.1 - VIB_Y / 2.0 + 42.0,
        ),
        (
            VIB_CENTER.0 + VIB_X / 2.0 - 56.0,
            VIB_CENTER.1 - VIB_Y / 2.0 + 42.0,
        ),
        (
            VIB_CENTER.0 - VIB_X / 2.0 + 56.0,
            VIB_CENTER.1 + VIB_Y / 2.0 - 42.0,
        ),
        (
            VIB_CENTER.0 + VIB_X / 2.0 - 56.0,
            VIB_CENTER.1 + VIB_Y / 2.0 - 42.0,
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let pod = centered_cube(
            format!("{OUTPUT_PREFIX}_accelerometer_pod_{i}"),
            ACCELEROMETER_POD_X,
            ACCELEROMETER_POD_Y,
            18.0,
        )
        .translate(x, y, DECK_Z + VIB_Z + 9.0);
        let cable = centered_cube(
            format!("{OUTPUT_PREFIX}_accelerometer_pod_{i}_cable_escape"),
            46.0,
            6.0,
            4.0,
        )
        .translate(x, y - 28.0, DECK_Z + VIB_Z + 20.0);
        pods = pods + pod + cable;
    }
    pods
}

fn transport_direction_combs() -> Part {
    let mut combs = Part::empty(format!("{OUTPUT_PREFIX}_transport_direction_combs"));
    for i in 0..TRANSPORT_WITNESS_COMB_TICKS {
        combs = combs
            + centered_cube(
                format!("{OUTPUT_PREFIX}_transport_comb_tick_{i}"),
                12.0 + i as f64 * 14.0,
                6.0,
                5.0,
            )
            .translate(
                VIB_CENTER.0 - VIB_X / 2.0 + 60.0 + i as f64 * 48.0,
                VIB_CENTER.1,
                DECK_Z + VIB_Z + 2.5,
            );
    }
    combs
}

fn transport_lock_witness_pins() -> Part {
    let mut pins = Part::empty(format!("{OUTPUT_PREFIX}_transport_lock_witness_pins"));
    for i in 0..SLOT_ROWS {
        pins = pins
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_transport_lock_witness_pin_{i}"),
                7.0,
                16.0,
                28,
            )
            .translate(
                VIB_CENTER.0 + VIB_X / 2.0 - 28.0,
                centered_index(i, SLOT_ROWS, 46.0) + VIB_CENTER.1,
                DECK_Z + VIB_Z + 8.0,
            );
    }
    pins
}

fn timed_perfusion_start_gates() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_timed_start_gate_panel"),
        GATE_X,
        GATE_Y,
        GATE_Z,
    )
    .translate(GATE_CENTER.0, GATE_CENTER.1, DECK_Z + GATE_Z / 2.0);

    body - gate_token_reliefs() + gate_lane_features() + settle_time_token_rail() + enable_bus_bar()
}

fn gate_token_reliefs() -> Part {
    let mut reliefs = Part::empty(format!("{OUTPUT_PREFIX}_gate_token_reliefs"));
    for gate in 0..PERFUSION_GATE_COUNT {
        let (x, y) = gate_position(gate);
        reliefs = reliefs
            + centered_cube(
                format!("{OUTPUT_PREFIX}_gate_{gate:02}_settle_token_recess"),
                GATE_TOKEN_SLOT_X,
                GATE_TOKEN_SLOT_Y,
                8.0,
            )
            .translate(x, y, DECK_Z + GATE_Z - 3.4);
    }
    reliefs
}

fn gate_lane_features() -> Part {
    let mut gates = Part::empty(format!("{OUTPUT_PREFIX}_gate_lane_features"));
    for gate in 0..PERFUSION_GATE_COUNT {
        let slot = PERFUSION_GATE_SLOT_MAP[gate];
        let (x, y) = gate_position(gate);
        let lane = centered_cube(
            format!("{OUTPUT_PREFIX}_gate_{gate:02}_slot_{slot:02}_no_flow_latch_lane"),
            GATE_LANE_X,
            GATE_LANE_Y,
            5.0,
        )
        .translate(x, y, DECK_Z + GATE_Z + 2.5);
        let boss = centered_cylinder(
            format!("{OUTPUT_PREFIX}_gate_{gate:02}_slot_{slot:02}_perfusion_enable_boss"),
            GATE_VALVE_BOSS_D / 2.0,
            12.0,
            28,
        )
        .translate(x, y + GATE_LANE_Y / 2.0 - 12.0, DECK_Z + GATE_Z + 6.0);
        let interlock = centered_cube(
            format!("{OUTPUT_PREFIX}_gate_{gate:02}_slot_{slot:02}_start_interlock_flag"),
            11.0,
            28.0,
            12.0,
        )
        .translate(x - GATE_LANE_X / 2.0 + 8.0, y, DECK_Z + GATE_Z + 6.0);
        gates = gates + lane + boss + interlock;
    }
    gates
}

fn settle_time_token_rail() -> Part {
    let rail = centered_cube(
        format!("{OUTPUT_PREFIX}_settle_time_token_rail"),
        GATE_X - 54.0,
        28.0,
        12.0,
    )
    .translate(
        GATE_CENTER.0,
        GATE_CENTER.1 - GATE_Y / 2.0 + 22.0,
        DECK_Z + GATE_Z + 6.0,
    );
    let mut tokens = Part::empty(format!("{OUTPUT_PREFIX}_settle_time_tokens"));
    for i in 0..SETTLE_TOKEN_COUNT {
        tokens = tokens
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_settle_time_{}min_token", SETTLE_MINUTES[i]),
                10.0,
                7.0,
                28,
            )
            .translate(
                centered_index(i, SETTLE_TOKEN_COUNT, 46.0) + GATE_CENTER.0,
                GATE_CENTER.1 - GATE_Y / 2.0 + 22.0,
                DECK_Z + GATE_Z + 15.5,
            );
    }
    rail + tokens
}

fn enable_bus_bar() -> Part {
    let bus = centered_cube(
        format!("{OUTPUT_PREFIX}_all_slots_perfusion_enable_bus"),
        GATE_X - 62.0,
        10.0,
        10.0,
    )
    .translate(
        GATE_CENTER.0,
        GATE_CENTER.1 + GATE_Y / 2.0 - 18.0,
        DECK_Z + GATE_Z + 5.0,
    );
    let mut markers = Part::empty(format!("{OUTPUT_PREFIX}_enable_bus_slot_markers"));
    for i in 0..ENABLE_BUS_MARKERS {
        markers = markers
            + centered_cube(
                format!("{OUTPUT_PREFIX}_enable_bus_slot_{i:02}_marker"),
                5.0,
                18.0,
                5.0,
            )
            .translate(
                centered_index(i, ENABLE_BUS_MARKERS, 23.0) + GATE_CENTER.0,
                GATE_CENTER.1 + GATE_Y / 2.0 - 18.0,
                DECK_Z + GATE_Z + 12.5,
            );
    }
    bus + markers
}

fn imaging_fiducial_bridge() -> Part {
    bridge_posts() + bridge_beams() + camera_pods() + imaging_fiducials()
}

fn bridge_posts() -> Part {
    let mut posts = Part::empty(format!("{OUTPUT_PREFIX}_imaging_bridge_posts"));
    for (i, (x, y)) in reference_corner_points(18.0).into_iter().enumerate() {
        let post_height = BRIDGE_UNDERSIDE_Z - DECK_Z;
        posts = posts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_imaging_bridge_post_{i}"),
                BRIDGE_POST_X,
                BRIDGE_POST_Y,
                post_height,
            )
            .translate(x, y, DECK_Z + post_height / 2.0);
    }
    posts
}

fn bridge_beams() -> Part {
    let span_x = NEST_X + 116.0;
    let span_y = NEST_Y + 116.0;
    let front = centered_cube(
        format!("{OUTPUT_PREFIX}_imaging_bridge_front_beam"),
        span_x,
        BRIDGE_BEAM_W,
        BRIDGE_BEAM_Z,
    )
    .translate(
        NEST_CENTER.0,
        NEST_CENTER.1 - span_y / 2.0,
        BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0,
    );
    let rear = centered_cube(
        format!("{OUTPUT_PREFIX}_imaging_bridge_rear_beam"),
        span_x,
        BRIDGE_BEAM_W,
        BRIDGE_BEAM_Z,
    )
    .translate(
        NEST_CENTER.0,
        NEST_CENTER.1 + span_y / 2.0,
        BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0,
    );
    let left = centered_cube(
        format!("{OUTPUT_PREFIX}_imaging_bridge_left_beam"),
        BRIDGE_BEAM_W,
        span_y,
        BRIDGE_BEAM_Z,
    )
    .translate(
        NEST_CENTER.0 - span_x / 2.0,
        NEST_CENTER.1,
        BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0,
    );
    let right = centered_cube(
        format!("{OUTPUT_PREFIX}_imaging_bridge_right_beam"),
        BRIDGE_BEAM_W,
        span_y,
        BRIDGE_BEAM_Z,
    )
    .translate(
        NEST_CENTER.0 + span_x / 2.0,
        NEST_CENTER.1,
        BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0,
    );

    front + rear + left + right
}

fn camera_pods() -> Part {
    let mut pods = Part::empty(format!("{OUTPUT_PREFIX}_imaging_camera_pods"));
    for i in 0..BRIDGE_CAMERA_PODS {
        let col = i % 2;
        let row = i / 2;
        let x = NEST_CENTER.0 + centered_index(col, 2, NEST_X * 0.48);
        let y = NEST_CENTER.1 + centered_index(row, 2, NEST_Y * 0.48);
        let pod = centered_cube(
            format!("{OUTPUT_PREFIX}_imaging_camera_pod_{i}"),
            CAMERA_POD_X,
            CAMERA_POD_Y,
            CAMERA_POD_Z,
        )
        .translate(
            x,
            y,
            BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z + CAMERA_POD_Z / 2.0,
        );
        let lens = centered_cylinder(
            format!("{OUTPUT_PREFIX}_imaging_camera_lens_clearance_{i}"),
            13.0,
            7.0,
            32,
        )
        .translate(x, y, BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z + 3.5);
        pods = pods + (pod - lens);
    }
    pods
}

fn imaging_fiducials() -> Part {
    let mut targets = Part::empty(format!("{OUTPUT_PREFIX}_imaging_fiducial_targets"));
    for i in 0..FIDUCIAL_TARGETS {
        let slot = if i < 4 { i * 5 } else { i * 3 - 11 };
        let (slot_x, slot_y) = slot_global_center(slot);
        let offset_x = if i % 2 == 0 { -48.0 } else { 48.0 };
        let offset_y = if i < 4 { -32.0 } else { 32.0 };
        let boss = centered_cylinder(
            format!("{OUTPUT_PREFIX}_fiducial_target_{i}"),
            FIDUCIAL_D / 2.0,
            5.0,
            36,
        )
        .translate(slot_x + offset_x, slot_y + offset_y, DECK_Z + NEST_Z + 7.5);
        let cross_x = centered_cube(
            format!("{OUTPUT_PREFIX}_fiducial_target_{i}_cross_x"),
            FIDUCIAL_D + 8.0,
            2.0,
            6.0,
        )
        .translate(slot_x + offset_x, slot_y + offset_y, DECK_Z + NEST_Z + 11.0);
        let cross_y = centered_cube(
            format!("{OUTPUT_PREFIX}_fiducial_target_{i}_cross_y"),
            2.0,
            FIDUCIAL_D + 8.0,
            6.0,
        )
        .translate(slot_x + offset_x, slot_y + offset_y, DECK_Z + NEST_Z + 11.0);
        targets = targets + boss + cross_x + cross_y;
    }
    targets
}

fn humidity_evaporation_witness_wells() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_humidity_evaporation_well_panel"),
        HUM_X,
        HUM_Y,
        HUM_Z,
    )
    .translate(HUM_CENTER.0, HUM_CENTER.1, DECK_Z + HUM_Z / 2.0);

    body - humidity_well_cuts()
        + humidity_well_rims()
        + evaporation_balance_bars()
        + humidity_wick_channel()
}

fn humidity_well_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_humidity_well_cuts"));
    for well in 0..HUMIDITY_WELL_COUNT {
        let (x, y) = humidity_well_position(well);
        cuts = cuts
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_humidity_well_{well:02}_cut"),
                HUMIDITY_WELL_D / 2.0,
                HUMIDITY_WELL_DEPTH + 1.0,
                28,
            )
            .translate(x, y, DECK_Z + HUM_Z - HUMIDITY_WELL_DEPTH / 2.0 + 0.5);
    }
    cuts
}

fn humidity_well_rims() -> Part {
    let mut rims = Part::empty(format!("{OUTPUT_PREFIX}_humidity_well_rims"));
    for well in 0..HUMIDITY_WELL_COUNT {
        let (x, y) = humidity_well_position(well);
        rims = rims
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_humidity_well_{well:02}_rim"),
                HUMIDITY_WELL_RIM_D / 2.0,
                4.0,
                28,
            )
            .translate(x, y, DECK_Z + HUM_Z + 2.0);
    }
    rims
}

fn evaporation_balance_bars() -> Part {
    let mut bars = Part::empty(format!("{OUTPUT_PREFIX}_evaporation_balance_bars"));
    for i in 0..EVAPORATION_BALANCE_BARS {
        bars = bars
            + centered_cube(
                format!("{OUTPUT_PREFIX}_evaporation_balance_bar_{i}"),
                34.0 + i as f64 * 24.0,
                6.0,
                5.0,
            )
            .translate(
                HUM_CENTER.0 - HUM_X / 2.0 + 62.0 + i as f64 * 52.0,
                HUM_CENTER.1 - HUM_Y / 2.0 + 24.0,
                DECK_Z + HUM_Z + 2.5,
            );
    }
    bars
}

fn humidity_wick_channel() -> Part {
    centered_cube(
        format!("{OUTPUT_PREFIX}_covered_humidity_wick_channel"),
        HUM_X - 54.0,
        10.0,
        7.0,
    )
    .translate(
        HUM_CENTER.0,
        HUM_CENTER.1 + HUM_Y / 2.0 - 24.0,
        DECK_Z + HUM_Z + 3.5,
    )
}

fn edge_center_comparison_markers() -> Part {
    let mut markers = Part::empty(format!("{OUTPUT_PREFIX}_edge_center_comparison_markers"));
    for slot in 0..SLOT_COUNT {
        let (x, y) = slot_global_center(slot);
        let d = if is_edge_slot(slot) {
            EDGE_MARKER_D
        } else {
            CENTER_MARKER_D
        };
        markers = markers
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_slot_{slot:02}_edge_center_marker"),
                d / 2.0,
                MARKER_Z,
                28,
            )
            .translate(
                x + SLOT_POCKET_X / 2.0 - 18.0,
                y - SLOT_POCKET_Y / 2.0 + 16.0,
                DECK_Z + NEST_Z + MARKER_Z / 2.0 + 7.0,
            );
    }
    markers + edge_center_reference_strips()
}

fn edge_center_reference_strips() -> Part {
    let top = centered_cube(
        format!("{OUTPUT_PREFIX}_edge_row_reference_strip"),
        SLOT_ARRAY_X,
        6.0,
        EDGE_CENTER_STRIP_Z,
    )
    .translate(
        NEST_CENTER.0,
        NEST_CENTER.1 + SLOT_ARRAY_Y / 2.0 + 32.0,
        DECK_Z + NEST_Z + 4.5,
    );
    let bottom = centered_cube(
        format!("{OUTPUT_PREFIX}_edge_row_bottom_reference_strip"),
        SLOT_ARRAY_X,
        6.0,
        EDGE_CENTER_STRIP_Z,
    )
    .translate(
        NEST_CENTER.0,
        NEST_CENTER.1 - SLOT_ARRAY_Y / 2.0 - 32.0,
        DECK_Z + NEST_Z + 4.5,
    );
    let center_cross_x = centered_cube(
        format!("{OUTPUT_PREFIX}_center_slot_comparison_cross_x"),
        SLOT_PITCH_X + REVC_CHIP_LENGTH,
        5.0,
        EDGE_CENTER_STRIP_Z,
    )
    .translate(NEST_CENTER.0, NEST_CENTER.1, DECK_Z + NEST_Z + 5.0);
    let center_cross_y = centered_cube(
        format!("{OUTPUT_PREFIX}_center_slot_comparison_cross_y"),
        5.0,
        SLOT_PITCH_Y + REVC_CHIP_WIDTH,
        EDGE_CENTER_STRIP_Z,
    )
    .translate(NEST_CENTER.0, NEST_CENTER.1, DECK_Z + NEST_Z + 5.0);

    top + bottom + center_cross_x + center_cross_y
}

fn release_hold_reject_disposition_lanes() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_disposition_lane_panel"),
        DISPOSITION_X,
        DISPOSITION_Y,
        DISPOSITION_Z,
    )
    .translate(
        DISPOSITION_CENTER.0,
        DISPOSITION_CENTER.1,
        DECK_Z + DISPOSITION_Z / 2.0,
    );

    body - disposition_lane_recesses()
        + disposition_lane_walls()
        + disposition_token_posts()
        + quarantine_pocket()
}

fn disposition_lane_recesses() -> Part {
    let mut recesses = Part::empty(format!("{OUTPUT_PREFIX}_disposition_lane_recesses"));
    for lane in 0..DISPOSITION_LANES {
        let x = disposition_lane_x(lane);
        recesses = recesses
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{}_lane_recess", disposition_label(lane)),
                DISPOSITION_LANE_X,
                DISPOSITION_LANE_Y,
                8.0,
            )
            .translate(x, DISPOSITION_CENTER.1, DECK_Z + DISPOSITION_Z - 3.5);
    }
    recesses
}

fn disposition_lane_walls() -> Part {
    let mut walls = Part::empty(format!("{OUTPUT_PREFIX}_disposition_lane_walls"));
    for lane in 0..DISPOSITION_LANES {
        let x = disposition_lane_x(lane);
        for sx in [-1.0, 1.0] {
            walls = walls
                + centered_cube(
                    format!(
                        "{OUTPUT_PREFIX}_{}_lane_side_wall_{sx}",
                        disposition_label(lane)
                    ),
                    6.0,
                    DISPOSITION_LANE_Y + 14.0,
                    16.0,
                )
                .translate(
                    x + sx * (DISPOSITION_LANE_X / 2.0 + 6.0),
                    DISPOSITION_CENTER.1,
                    DECK_Z + DISPOSITION_Z + 8.0,
                );
        }
    }
    walls
}

fn disposition_token_posts() -> Part {
    let mut posts = Part::empty(format!("{OUTPUT_PREFIX}_disposition_token_posts"));
    for lane in 0..DISPOSITION_LANES {
        let x = disposition_lane_x(lane);
        for token in 0..DISPOSITION_TOKENS_PER_LANE {
            posts = posts
                + centered_cylinder(
                    format!(
                        "{OUTPUT_PREFIX}_{}_token_post_{token}",
                        disposition_label(lane)
                    ),
                    6.5,
                    8.0,
                    24,
                )
                .translate(
                    x,
                    centered_index(token, DISPOSITION_TOKENS_PER_LANE, 22.0) + DISPOSITION_CENTER.1,
                    DECK_Z + DISPOSITION_Z + 4.0,
                );
        }
    }
    posts
}

fn quarantine_pocket() -> Part {
    let pocket = centered_cube(
        format!("{OUTPUT_PREFIX}_reject_quarantine_pocket"),
        QUARANTINE_POCKET_X,
        QUARANTINE_POCKET_Y,
        26.0,
    )
    .translate(
        DISPOSITION_CENTER.0 + DISPOSITION_X / 2.0 - 54.0,
        DISPOSITION_CENTER.1 + DISPOSITION_Y / 2.0 - 40.0,
        DECK_Z + DISPOSITION_Z + 13.0,
    );
    let open_top = centered_cube(
        format!("{OUTPUT_PREFIX}_reject_quarantine_pocket_open_top_cut"),
        QUARANTINE_POCKET_X - 16.0,
        QUARANTINE_POCKET_Y - 14.0,
        18.0,
    )
    .translate(
        DISPOSITION_CENTER.0 + DISPOSITION_X / 2.0 - 54.0,
        DISPOSITION_CENTER.1 + DISPOSITION_Y / 2.0 - 40.0,
        DECK_Z + DISPOSITION_Z + 19.0,
    );
    pocket - open_top
}

fn barcode_custody_board() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_barcode_custody_board"),
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    )
    .translate(CUSTODY_CENTER.0, CUSTODY_CENTER.1, DECK_Z + CUSTODY_Z / 2.0);

    body + barcode_lands() + rfid_lands() + timestamp_tick_strip() + custody_fiducials()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_barcode_lands"));
    for slot in 0..BARCODE_LANDS {
        let col = slot % 8;
        let row = slot / 8;
        lands = lands
            + centered_cube(
                format!("{OUTPUT_PREFIX}_slot_{slot:02}_barcode_land"),
                34.0,
                14.0,
                4.0,
            )
            .translate(
                CUSTODY_CENTER.0 + centered_index(col, 8, 38.0),
                CUSTODY_CENTER.1 + 30.0 - row as f64 * 30.0,
                DECK_Z + CUSTODY_Z + 2.0,
            );
    }
    lands
}

fn rfid_lands() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_rfid_lands"));
    for i in 0..RFID_LANDS {
        lands = lands
            + centered_cylinder(format!("{OUTPUT_PREFIX}_rfid_land_{i}"), 12.0, 4.0, 28).translate(
                CUSTODY_CENTER.0 - CUSTODY_X / 2.0 + 32.0,
                CUSTODY_CENTER.1 + centered_index(i, RFID_LANDS, 28.0),
                DECK_Z + CUSTODY_Z + 2.0,
            );
    }
    lands
}

fn timestamp_tick_strip() -> Part {
    let mut ticks = Part::empty(format!("{OUTPUT_PREFIX}_timestamp_tick_strip"));
    for i in 0..TIMESTAMP_TICKS {
        ticks = ticks
            + centered_cube(
                format!("{OUTPUT_PREFIX}_timestamp_tick_{i}"),
                5.0,
                16.0 + i as f64 * 2.0,
                4.0,
            )
            .translate(
                CUSTODY_CENTER.0 - 124.0 + i as f64 * 32.0,
                CUSTODY_CENTER.1 - CUSTODY_Y / 2.0 + 22.0,
                DECK_Z + CUSTODY_Z + 2.0,
            );
    }
    ticks
}

fn custody_fiducials() -> Part {
    let mut fiducials = Part::empty(format!("{OUTPUT_PREFIX}_custody_board_fiducials"));
    let points = [
        (
            CUSTODY_CENTER.0 - CUSTODY_X / 2.0 + 20.0,
            CUSTODY_CENTER.1 - CUSTODY_Y / 2.0 + 20.0,
        ),
        (
            CUSTODY_CENTER.0 + CUSTODY_X / 2.0 - 20.0,
            CUSTODY_CENTER.1 - CUSTODY_Y / 2.0 + 20.0,
        ),
        (
            CUSTODY_CENTER.0 - CUSTODY_X / 2.0 + 20.0,
            CUSTODY_CENTER.1 + CUSTODY_Y / 2.0 - 20.0,
        ),
        (
            CUSTODY_CENTER.0 + CUSTODY_X / 2.0 - 20.0,
            CUSTODY_CENTER.1 + CUSTODY_Y / 2.0 - 20.0,
        ),
    ];
    assert_eq!(points.len(), CUSTODY_FIDUCIALS);
    for (i, (x, y)) in points.into_iter().enumerate() {
        fiducials = fiducials
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_custody_fiducial_{i}"),
                7.0,
                4.5,
                28,
            )
            .translate(x, y, DECK_Z + CUSTODY_Z + 2.25);
    }
    fiducials
}

fn closed_perfusion_enable_timing_harness() -> Part {
    timing_manifold_bar() + slot_enable_routes() + dry_break_couplers()
}

fn timing_manifold_bar() -> Part {
    centered_cube(
        format!("{OUTPUT_PREFIX}_closed_timing_enable_manifold_bar"),
        HARNESS_MANIFOLD_X,
        HARNESS_MANIFOLD_Y,
        HARNESS_Z,
    )
    .translate(
        NEST_CENTER.0 + NEST_X / 2.0 + 46.0,
        NEST_CENTER.1,
        DECK_Z + HARNESS_Z / 2.0 + 4.0,
    )
}

fn slot_enable_routes() -> Part {
    let mut routes = Part::empty(format!("{OUTPUT_PREFIX}_slot_enable_timing_routes"));
    for gate in 0..PERFUSION_GATE_COUNT {
        let slot = PERFUSION_GATE_SLOT_MAP[gate];
        let (_, gate_y) = gate_position(gate);
        let (slot_x, slot_y) = slot_global_center(slot);
        let bus_x = NEST_CENTER.0 + NEST_X / 2.0 + 46.0;
        let route_z = DECK_Z + HARNESS_Z / 2.0 + 8.0 + (gate % 4) as f64 * 1.6;
        let gate_to_bus = centered_cube(
            format!("{OUTPUT_PREFIX}_gate_{gate:02}_to_timing_bus_route"),
            GATE_CENTER.0 - GATE_X / 2.0 - bus_x,
            HARNESS_TUBE_W,
            HARNESS_Z,
        )
        .translate(
            (GATE_CENTER.0 - GATE_X / 2.0 + bus_x) / 2.0,
            gate_y,
            route_z,
        );
        let bus_to_slot = centered_cube(
            format!("{OUTPUT_PREFIX}_slot_{slot:02}_settle_time_equalized_route"),
            HARNESS_TUBE_W,
            (slot_y - gate_y).abs() + HARNESS_TUBE_W,
            HARNESS_Z,
        )
        .translate(bus_x, (slot_y + gate_y) / 2.0, route_z);
        let slot_branch = centered_cube(
            format!("{OUTPUT_PREFIX}_slot_{slot:02}_dry_break_branch_route"),
            bus_x - (slot_x + SLOT_POCKET_X / 2.0),
            HARNESS_TUBE_W,
            HARNESS_Z,
        )
        .translate(
            (bus_x + slot_x + SLOT_POCKET_X / 2.0) / 2.0,
            slot_y,
            route_z,
        );
        routes = routes + gate_to_bus + bus_to_slot + slot_branch;
    }
    routes
}

fn dry_break_couplers() -> Part {
    let mut couplers = Part::empty(format!("{OUTPUT_PREFIX}_dry_break_couplers"));
    for row in 0..DRY_BREAK_COUPLERS {
        let y = NEST_CENTER.1 + centered_index(row, SLOT_ROWS, SLOT_PITCH_Y);
        couplers = couplers
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_row_{row}_perfusion_enable_dry_break_coupler"),
                12.0,
                16.0,
                28,
            )
            .translate(
                NEST_CENTER.0 + NEST_X / 2.0 + 22.0,
                y,
                DECK_Z + NEST_Z + 8.0,
            );
    }
    couplers
}

fn robot_service_keepouts() -> Part {
    keepout_outline() + robot_approach_pads() + service_pull_tabs() + clean_glove_reach_gauges()
}

fn keepout_outline() -> Part {
    let front = centered_cube(
        format!("{OUTPUT_PREFIX}_front_robot_keepout_rail"),
        KEEP_OUT_X,
        8.0,
        KEEP_OUT_RAIL_Z,
    )
    .translate(0.0, -KEEP_OUT_Y / 2.0, DECK_Z + KEEP_OUT_RAIL_Z / 2.0 + 1.0);
    let rear = centered_cube(
        format!("{OUTPUT_PREFIX}_rear_service_keepout_rail"),
        KEEP_OUT_X,
        8.0,
        KEEP_OUT_RAIL_Z,
    )
    .translate(0.0, KEEP_OUT_Y / 2.0, DECK_Z + KEEP_OUT_RAIL_Z / 2.0 + 1.0);
    let left = centered_cube(
        format!("{OUTPUT_PREFIX}_left_service_keepout_rail"),
        8.0,
        KEEP_OUT_Y,
        KEEP_OUT_RAIL_Z,
    )
    .translate(-KEEP_OUT_X / 2.0, 0.0, DECK_Z + KEEP_OUT_RAIL_Z / 2.0 + 1.0);
    let right = centered_cube(
        format!("{OUTPUT_PREFIX}_right_gate_service_keepout_rail"),
        8.0,
        KEEP_OUT_Y,
        KEEP_OUT_RAIL_Z,
    )
    .translate(KEEP_OUT_X / 2.0, 0.0, DECK_Z + KEEP_OUT_RAIL_Z / 2.0 + 1.0);

    front + rear + left + right
}

fn robot_approach_pads() -> Part {
    let mut pads = Part::empty(format!("{OUTPUT_PREFIX}_robot_approach_pads"));
    for i in 0..6 {
        pads = pads
            + centered_cube(
                format!("{OUTPUT_PREFIX}_front_robot_approach_pad_{i}"),
                94.0,
                28.0,
                5.0,
            )
            .translate(
                centered_index(i, 6, 172.0),
                -KEEP_OUT_Y / 2.0 + 42.0,
                DECK_Z + 2.5,
            );
    }
    pads
}

fn service_pull_tabs() -> Part {
    let mut tabs = Part::empty(format!("{OUTPUT_PREFIX}_service_pull_tabs"));
    for i in 0..4 {
        tabs = tabs
            + centered_cube(
                format!("{OUTPUT_PREFIX}_rear_service_pull_tab_{i}"),
                126.0,
                22.0,
                5.0,
            )
            .translate(
                centered_index(i, 4, 230.0),
                KEEP_OUT_Y / 2.0 - 42.0,
                DECK_Z + 2.5,
            );
    }
    tabs
}

fn clean_glove_reach_gauges() -> Part {
    let mut gauges = Part::empty(format!("{OUTPUT_PREFIX}_clean_glove_reach_gauges"));
    for (i, (x, y, label_x)) in [
        (-KEEP_OUT_X / 2.0 + 44.0, NEST_CENTER.1, 22.0),
        (KEEP_OUT_X / 2.0 - 44.0, GATE_CENTER.1, 34.0),
        (0.0, KEEP_OUT_Y / 2.0 - 72.0, 46.0),
    ]
    .into_iter()
    .enumerate()
    {
        gauges = gauges
            + centered_cube(
                format!("{OUTPUT_PREFIX}_glove_reach_clearance_gauge_{i}"),
                label_x,
                84.0,
                5.0,
            )
            .translate(x, y, DECK_Z + 2.5);
    }
    gauges
}

fn assert_design_constraints() {
    assert_eq!(SLOT_COUNT, 16);
    assert_eq!(SLOT_COUNT, SLOT_COLS * SLOT_ROWS);
    assert_eq!(EDGE_SLOT_COUNT + CENTER_SLOT_COUNT, SLOT_COUNT);
    assert_eq!(edge_slot_count(), EDGE_SLOT_COUNT);
    assert_eq!(center_slot_count(), CENTER_SLOT_COUNT);
    assert_eq!(PERFUSION_GATE_COUNT, SLOT_COUNT);
    assert!(is_gate_map_complete());
    assert_eq!(SLOT_PAIRED_HUMIDITY_WELLS, SLOT_COUNT);
    assert_eq!(
        HUMIDITY_WELL_COUNT,
        SLOT_PAIRED_HUMIDITY_WELLS + CORNER_HUMIDITY_REFERENCES
    );
    assert_eq!(VIBRATION_COUPON_COUNT, SLOT_COUNT);
    assert_eq!(NEST_CLAMP_COUNT, SLOT_ROWS * 2);
    assert_eq!(LEVELING_FEET, 4);
    assert_eq!(ISOLATOR_LOCATOR_COUNT, 4);
    assert_eq!(DRY_BREAK_COUPLERS, SLOT_ROWS);
    assert_eq!(GATE_ROWS * GATE_COLS, PERFUSION_GATE_COUNT);
    assert!(SETTLE_MINUTES.windows(2).all(|pair| pair[0] < pair[1]));
    assert!(SETTLE_MINUTES[SETTLE_TOKEN_COUNT - 1] >= 90);
    assert!(SLOT_POCKET_X > REVC_CHIP_LENGTH);
    assert!(SLOT_POCKET_Y > REVC_CHIP_WIDTH);
    assert!(NEST_X > SLOT_ARRAY_X);
    assert!(NEST_Y > SLOT_ARRAY_Y);
    assert!(reference_plane_z() > DECK_Z + NEST_Z + 2.0);
    assert!(bridge_clearance_above_chip() > REVC_TOTAL_HEIGHT + 120.0);
    assert!(FRONT_ROBOT_CLEARANCE >= 400.0);
    assert!(REAR_SERVICE_CLEARANCE >= 260.0);
    assert!(LEFT_CASSETTE_SERVICE_CLEARANCE >= 240.0);
    assert!(RIGHT_GATE_SERVICE_CLEARANCE >= 250.0);
    assert!(TOP_IMAGING_CLEARANCE >= 240.0);

    let rects = primary_rects();
    for rect in rects {
        assert!(
            rect.fits_inside_deck(),
            "{} does not fit on deck",
            rect.name
        );
    }
    for left in 0..rects.len() {
        for right in left + 1..rects.len() {
            assert!(
                !rects[left].overlaps(rects[right]),
                "{} overlaps {}",
                rects[left].name,
                rects[right].name
            );
        }
    }
}

fn primary_rects() -> [Rect; 6] {
    [
        Rect {
            name: "cassette_nest",
            center: NEST_CENTER,
            x: NEST_X,
            y: NEST_Y,
        },
        Rect {
            name: "vibration_coupons",
            center: VIB_CENTER,
            x: VIB_X,
            y: VIB_Y,
        },
        Rect {
            name: "timed_gates",
            center: GATE_CENTER,
            x: GATE_X,
            y: GATE_Y,
        },
        Rect {
            name: "humidity_witness",
            center: HUM_CENTER,
            x: HUM_X,
            y: HUM_Y,
        },
        Rect {
            name: "disposition",
            center: DISPOSITION_CENTER,
            x: DISPOSITION_X,
            y: DISPOSITION_Y,
        },
        Rect {
            name: "custody_board",
            center: CUSTODY_CENTER,
            x: CUSTODY_X,
            y: CUSTODY_Y,
        },
    ]
}

fn slot_local_center(slot: usize) -> (f64, f64) {
    let col = slot % SLOT_COLS;
    let row = slot / SLOT_COLS;
    (
        centered_index(col, SLOT_COLS, SLOT_PITCH_X),
        centered_index(row, SLOT_ROWS, SLOT_PITCH_Y),
    )
}

fn slot_global_center(slot: usize) -> (f64, f64) {
    let (x, y) = slot_local_center(slot);
    (NEST_CENTER.0 + x, NEST_CENTER.1 + y)
}

fn is_edge_slot(slot: usize) -> bool {
    let col = slot % SLOT_COLS;
    let row = slot / SLOT_COLS;
    col == 0 || col == SLOT_COLS - 1 || row == 0 || row == SLOT_ROWS - 1
}

fn edge_slot_count() -> usize {
    (0..SLOT_COUNT).filter(|slot| is_edge_slot(*slot)).count()
}

fn center_slot_count() -> usize {
    SLOT_COUNT - edge_slot_count()
}

fn is_gate_map_complete() -> bool {
    let mut seen = [false; SLOT_COUNT];
    for slot in PERFUSION_GATE_SLOT_MAP {
        if slot >= SLOT_COUNT || seen[slot] {
            return false;
        }
        seen[slot] = true;
    }
    seen.into_iter().all(|mapped| mapped)
}

fn gate_position(gate: usize) -> (f64, f64) {
    let col = gate % GATE_COLS;
    let row = gate / GATE_COLS;
    (
        GATE_CENTER.0 + centered_index(col, GATE_COLS, 48.0),
        GATE_CENTER.1 + centered_index(row, GATE_ROWS, 62.0) + 14.0,
    )
}

fn humidity_well_position(well: usize) -> (f64, f64) {
    if well < SLOT_PAIRED_HUMIDITY_WELLS {
        let col = well % SLOT_COLS;
        let row = well / SLOT_COLS;
        (
            HUM_CENTER.0 + centered_index(col, SLOT_COLS, 56.0) - 26.0,
            HUM_CENTER.1 + centered_index(row, SLOT_ROWS, 34.0) + 10.0,
        )
    } else {
        let corner = well - SLOT_PAIRED_HUMIDITY_WELLS;
        let x = if corner % 2 == 0 {
            HUM_CENTER.0 + HUM_X / 2.0 - 46.0
        } else {
            HUM_CENTER.0 - HUM_X / 2.0 + 46.0
        };
        let y = if corner < 2 {
            HUM_CENTER.1 + HUM_Y / 2.0 - 38.0
        } else {
            HUM_CENTER.1 - HUM_Y / 2.0 + 38.0
        };
        (x, y)
    }
}

fn vibration_coupon_position(coupon: usize) -> (f64, f64) {
    let col = coupon % SLOT_COLS;
    let row = coupon / SLOT_COLS;
    (
        VIB_CENTER.0 + centered_index(col, SLOT_COLS, 82.0) - 22.0,
        VIB_CENTER.1 + centered_index(row, SLOT_ROWS, 38.0),
    )
}

fn disposition_lane_x(lane: usize) -> f64 {
    DISPOSITION_CENTER.0 + centered_index(lane, DISPOSITION_LANES, 110.0) - 20.0
}

fn disposition_label(lane: usize) -> &'static str {
    match lane {
        0 => "release",
        1 => "hold",
        2 => "reject",
        _ => "unknown",
    }
}

fn reference_corner_points(inset: f64) -> [(f64, f64); 4] {
    [
        (
            NEST_CENTER.0 - (NEST_X + 78.0) / 2.0 + inset,
            NEST_CENTER.1 - (NEST_Y + 78.0) / 2.0 + inset,
        ),
        (
            NEST_CENTER.0 + (NEST_X + 78.0) / 2.0 - inset,
            NEST_CENTER.1 - (NEST_Y + 78.0) / 2.0 + inset,
        ),
        (
            NEST_CENTER.0 - (NEST_X + 78.0) / 2.0 + inset,
            NEST_CENTER.1 + (NEST_Y + 78.0) / 2.0 - inset,
        ),
        (
            NEST_CENTER.0 + (NEST_X + 78.0) / 2.0 - inset,
            NEST_CENTER.1 + (NEST_Y + 78.0) / 2.0 - inset,
        ),
    ]
}

fn reference_plane_z() -> f64 {
    DECK_Z + NEST_Z + 10.0
}

fn bridge_clearance_above_chip() -> f64 {
    BRIDGE_UNDERSIDE_Z - (DECK_Z + NEST_Z + REVC_TOTAL_HEIGHT)
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_and_station_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(OUTPUTS.len(), 13);
        assert_eq!(unique.len(), OUTPUTS.len());
        assert!(OUTPUTS.iter().all(|path| path.starts_with("output/")));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
        assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
        assert!(OUTPUTS[0].ends_with("_isolator_deck.stl"));
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }

    #[test]
    fn required_features_have_exported_parts() {
        for feature in REQUIRED_FEATURES {
            assert!(
                OUTPUTS.iter().any(|output| output.contains(feature)),
                "missing required output for {feature}"
            );
        }
    }

    #[test]
    fn cassette_slot_topology_and_edge_center_controls_are_stable() {
        assert_eq!(SLOT_COLS, 4);
        assert_eq!(SLOT_ROWS, 4);
        assert_eq!(SLOT_COUNT, 16);
        assert_eq!(edge_slot_count(), 12);
        assert_eq!(center_slot_count(), 4);
        assert!(SLOT_ARRAY_X < NEST_X - 70.0);
        assert!(SLOT_ARRAY_Y < NEST_Y - 70.0);
        assert!(SLOT_POCKET_X > REVC_CHIP_LENGTH);
        assert!(SLOT_POCKET_Y > REVC_CHIP_WIDTH);
    }

    #[test]
    fn timing_vibration_and_humidity_witnesses_cover_every_slot() {
        assert_eq!(PERFUSION_GATE_COUNT, SLOT_COUNT);
        assert!(is_gate_map_complete());
        assert_eq!(VIBRATION_COUPON_COUNT, SLOT_COUNT);
        assert_eq!(SLOT_PAIRED_HUMIDITY_WELLS, SLOT_COUNT);
        assert_eq!(HUMIDITY_WELL_COUNT, SLOT_COUNT + CORNER_HUMIDITY_REFERENCES);
        assert_eq!(DRY_BREAK_COUPLERS, SLOT_ROWS);
        assert_eq!(SETTLE_MINUTES, [0, 10, 20, 30, 45, 60, 90, 120]);
    }

    #[test]
    fn geometry_constraints_and_service_clearances_hold() {
        assert_design_constraints();
        assert!(bridge_clearance_above_chip() > REVC_TOTAL_HEIGHT + 120.0);
        assert!(reference_plane_z() > DECK_Z + NEST_Z);
        assert!(FRONT_ROBOT_CLEARANCE >= 400.0);
        assert!(REAR_SERVICE_CLEARANCE >= 260.0);
        assert!(LEFT_CASSETTE_SERVICE_CLEARANCE >= 240.0);
        assert!(RIGHT_GATE_SERVICE_CLEARANCE >= 250.0);
    }
}
