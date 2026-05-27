use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed cassette seed-density gradient and settle-time witness station.
//
// Research basis: cell-seeding uniformity is sensitive to suspension flow,
// movement before attachment, local edge/center position, and chip geometry.
// This no-cell validation fixture makes those variables visible with closed
// source handoff, row/column witnesses, edge/center coupons, settle timing,
// gentle-mix evidence, dead-volume/bubble windows, custody, camera evidence,
// and explicit release/hold/reject disposition gates.

const OUTPUT_PREFIX: &str = "closed_cassette_seed_density_gradient_settle_witness_station";

const OUTPUTS: [&str; 14] = [
    "output/closed_cassette_seed_density_gradient_settle_witness_station_leak_tray_deck.stl",
    "output/closed_cassette_seed_density_gradient_settle_witness_station_source_suspension_handoff.stl",
    "output/closed_cassette_seed_density_gradient_settle_witness_station_cassette_chip_position_grid.stl",
    "output/closed_cassette_seed_density_gradient_settle_witness_station_row_column_witness_wells.stl",
    "output/closed_cassette_seed_density_gradient_settle_witness_station_edge_center_coupon_grid.stl",
    "output/closed_cassette_seed_density_gradient_settle_witness_station_timed_settle_token_rail.stl",
    "output/closed_cassette_seed_density_gradient_settle_witness_station_gentle_mix_evidence_panel.stl",
    "output/closed_cassette_seed_density_gradient_settle_witness_station_bubble_dead_volume_windows.stl",
    "output/closed_cassette_seed_density_gradient_settle_witness_station_barcode_coa_custody_board.stl",
    "output/closed_cassette_seed_density_gradient_settle_witness_station_release_hold_reject_gates.stl",
    "output/closed_cassette_seed_density_gradient_settle_witness_station_camera_evidence_bridge.stl",
    "output/closed_cassette_seed_density_gradient_settle_witness_station_robot_service_keepouts.stl",
    "output/closed_cassette_seed_density_gradient_settle_witness_station_closed_route_harness.stl",
    "output/closed_cassette_seed_density_gradient_settle_witness_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 12] = [
    "source_suspension_handoff",
    "cassette_chip_position_grid",
    "row_column_witness_wells",
    "edge_center_coupon_grid",
    "timed_settle_token_rail",
    "gentle_mix_evidence_panel",
    "bubble_dead_volume_windows",
    "barcode_coa_custody_board",
    "release_hold_reject_gates",
    "camera_evidence_bridge",
    "robot_service_keepouts",
    "closed_route_harness",
];

#[cfg(test)]
const VALIDATION_CONTROLS: [&str; 10] = [
    "closed_source_handoff",
    "row_column_witness_wells",
    "edge_center_coupon_grid",
    "settle_time_tokens",
    "gentle_mix_evidence",
    "bubble_dead_volume_windows",
    "barcode_coa_custody",
    "release_hold_reject_gates",
    "camera_evidence_bridge",
    "robot_service_keepouts",
];

const DECK_X: f64 = 1600.0;
const DECK_Y: f64 = 1040.0;
const DECK_Z: f64 = 22.0;
const TRAY_RIM_W: f64 = 20.0;
const TRAY_RIM_Z: f64 = 38.0;
const BASIN_DEPTH: f64 = 7.0;
const DRAIN_CHANNEL_W: f64 = 14.0;
const MOUNT_HOLE_D: f64 = 6.8;
const FLOOR_SHADOW_Z: f64 = 2.4;

const CASSETTE_COLS: usize = 4;
const CASSETTE_ROWS: usize = 5;
const POSITION_COUNT: usize = CASSETTE_COLS * CASSETTE_ROWS;
const EDGE_POSITION_COUNT: usize = 14;
const CENTER_POSITION_COUNT: usize = POSITION_COUNT - EDGE_POSITION_COUNT;
const DENSITY_GRADIENT_LEVELS: usize = 5;
const CHIP_GUTTER_X: f64 = 8.0;
const CHIP_GUTTER_Y: f64 = 7.0;
const CHIP_PITCH_X: f64 = REVC_CHIP_LENGTH + CHIP_GUTTER_X;
const CHIP_PITCH_Y: f64 = REVC_CHIP_WIDTH + CHIP_GUTTER_Y;
const CHIP_ARRAY_X: f64 =
    CASSETTE_COLS as f64 * REVC_CHIP_LENGTH + (CASSETTE_COLS as f64 - 1.0) * CHIP_GUTTER_X;
const CHIP_ARRAY_Y: f64 =
    CASSETTE_ROWS as f64 * REVC_CHIP_WIDTH + (CASSETTE_ROWS as f64 - 1.0) * CHIP_GUTTER_Y;

const GRID_CENTER: (f64, f64) = (-120.0, 100.0);
const GRID_MARGIN_X: f64 = 40.0;
const GRID_MARGIN_Y: f64 = 43.0;
const GRID_X: f64 = CHIP_ARRAY_X + 2.0 * GRID_MARGIN_X;
const GRID_Y: f64 = CHIP_ARRAY_Y + 2.0 * GRID_MARGIN_Y;
const GRID_Z: f64 = 42.0;
const CHIP_SOCKET_X: f64 = REVC_CHIP_LENGTH + 7.0;
const CHIP_SOCKET_Y: f64 = REVC_CHIP_WIDTH + 7.0;
const CHIP_SOCKET_DEPTH: f64 = 16.0;
const CHIP_RIM_W: f64 = 6.0;
const GRADIENT_STRIP_Z: f64 = 4.0;
const EDGE_MARKER_D: f64 = 13.0;
const CENTER_MARKER_D: f64 = 19.0;

const SOURCE_CENTER: (f64, f64) = (-630.0, 235.0);
const SOURCE_X: f64 = 250.0;
const SOURCE_Y: f64 = 220.0;
const SOURCE_Z: f64 = 46.0;
const SOURCE_BAG_POCKET_X: f64 = 138.0;
const SOURCE_BAG_POCKET_Y: f64 = 86.0;
const SOURCE_BAG_POCKET_DEPTH: f64 = 18.0;
const SOURCE_DRY_BREAKS: usize = 4;
const SOURCE_DENSITY_DRAW_PORTS: usize = 5;
const SOURCE_LOCATOR_PINS: usize = 4;

const WELL_CENTER: (f64, f64) = (520.0, 270.0);
const WELL_PANEL_X: f64 = 330.0;
const WELL_PANEL_Y: f64 = 250.0;
const WELL_PANEL_Z: f64 = 30.0;
const ROW_WITNESS_WELLS: usize = CASSETTE_ROWS;
const COLUMN_WITNESS_WELLS: usize = CASSETTE_COLS;
const WITNESS_WELL_COUNT: usize = ROW_WITNESS_WELLS + COLUMN_WITNESS_WELLS;
const WITNESS_WELL_D: f64 = 26.0;
const WITNESS_WELL_RIM_D: f64 = 36.0;
const WITNESS_WELL_DEPTH: f64 = 14.0;
const WITNESS_CHANNEL_W: f64 = 7.0;

const COUPON_CENTER: (f64, f64) = (-630.0, -235.0);
const COUPON_PANEL_X: f64 = 250.0;
const COUPON_PANEL_Y: f64 = 260.0;
const COUPON_PANEL_Z: f64 = 22.0;
const COUPON_EDGE_D: f64 = 17.0;
const COUPON_CENTER_D: f64 = 23.0;
const COUPON_Z: f64 = 5.0;
const COUPON_PITCH_X: f64 = 44.0;
const COUPON_PITCH_Y: f64 = 36.0;

const SETTLE_CENTER: (f64, f64) = (520.0, 20.0);
const SETTLE_PANEL_X: f64 = 330.0;
const SETTLE_PANEL_Y: f64 = 145.0;
const SETTLE_PANEL_Z: f64 = 18.0;
const SETTLE_TOKEN_COUNT: usize = 8;
const SETTLE_MINUTES: [usize; SETTLE_TOKEN_COUNT] = [0, 10, 20, 30, 45, 60, 90, 120];
const SETTLE_TOKEN_SLOT_X: f64 = 28.0;
const SETTLE_TOKEN_SLOT_Y: f64 = 20.0;
const SETTLE_TOKEN_PITCH_X: f64 = 38.0;
const SETTLE_LANE_COUNT: usize = 3;

const MIX_CENTER: (f64, f64) = (520.0, -195.0);
const MIX_PANEL_X: f64 = 350.0;
const MIX_PANEL_Y: f64 = 210.0;
const MIX_PANEL_Z: f64 = 28.0;
const MIX_ROLLERS: usize = 4;
const MIX_BAFFLE_RIBS: usize = 6;
const MIX_SWEEP_TICKS: usize = 9;
const MIX_EVIDENCE_WINDOWS: usize = 5;
const MIX_TOKEN_LANDS: usize = 6;

const BUBBLE_CENTER: (f64, f64) = (-110.0, -375.0);
const BUBBLE_PANEL_X: f64 = 570.0;
const BUBBLE_PANEL_Y: f64 = 150.0;
const BUBBLE_PANEL_Z: f64 = 24.0;
const BUBBLE_WINDOWS: usize = POSITION_COUNT;
const BUBBLE_WINDOW_COLS: usize = 10;
const BUBBLE_WINDOW_X: f64 = 30.0;
const BUBBLE_WINDOW_Y: f64 = 18.0;
const DEAD_VOLUME_WINDOWS: usize = 8;
const DEAD_VOLUME_WINDOW_D: f64 = 16.0;
const BUBBLE_LADDER_TICKS: usize = 11;

const CUSTODY_CENTER: (f64, f64) = (530.0, -420.0);
const CUSTODY_X: f64 = 320.0;
const CUSTODY_Y: f64 = 105.0;
const CUSTODY_Z: f64 = 18.0;
const POSITION_BARCODE_LANDS: usize = POSITION_COUNT;
const COA_CARD_SLOTS: usize = 3;
const RFID_LANDS: usize = 4;
const SEAL_WITNESS_TABS: usize = 4;

const GATE_CENTER: (f64, f64) = (90.0, 435.0);
const GATE_PANEL_X: f64 = 440.0;
const GATE_PANEL_Y: f64 = 70.0;
const GATE_PANEL_Z: f64 = 24.0;
const GATE_LANES: usize = 3;
const GATE_TOKENS_PER_LANE: usize = 4;
const GATE_LANE_X: f64 = 122.0;
const GATE_LANE_Y: f64 = 44.0;
const GATE_LANE_PITCH_X: f64 = 142.0;

const CAMERA_UNDERSIDE_Z: f64 = 228.0;
const CAMERA_POST_X: f64 = 28.0;
const CAMERA_POST_Y: f64 = 34.0;
const CAMERA_BEAM_W: f64 = 22.0;
const CAMERA_BEAM_Z: f64 = 24.0;
const CAMERA_PODS: usize = 4;
const CAMERA_POD_X: f64 = 72.0;
const CAMERA_POD_Y: f64 = 48.0;
const CAMERA_POD_Z: f64 = 34.0;
const CAMERA_FIDUCIALS: usize = 8;
const CAMERA_CLEARANCE_Z: f64 = 260.0;

const ROUTE_Z: f64 = DECK_Z + 92.0;
const ROUTE_TUBE_W: f64 = 7.0;
const ROUTE_SEGMENTS: usize = 10;
const ROUTE_ELBOWS: usize = 8;
const ROUTE_DIRECTION_MARKERS: usize = 8;

const KEEP_OUT_X: f64 = 1510.0;
const KEEP_OUT_Y: f64 = 955.0;
const KEEP_OUT_Z: f64 = 7.0;
const FRONT_ROBOT_CLEARANCE: f64 = 390.0;
const REAR_SERVICE_CLEARANCE: f64 = 280.0;
const LEFT_SOURCE_SERVICE_CLEARANCE: f64 = 260.0;
const RIGHT_WITNESS_SERVICE_CLEARANCE: f64 = 250.0;
const TOP_CAMERA_SERVICE_CLEARANCE_Z: f64 = 300.0;
const KEEP_OUT_ZONES: usize = 6;

#[derive(Clone, Copy, Debug)]
struct Footprint {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Footprint {
    fn fits_on_deck(self) -> bool {
        let usable_x = DECK_X / 2.0 - TRAY_RIM_W - 12.0;
        let usable_y = DECK_Y / 2.0 - TRAY_RIM_W - 12.0;

        self.center.0.abs() + self.x / 2.0 <= usable_x
            && self.center.1.abs() + self.y / 2.0 <= usable_y
    }

    fn overlaps(self, other: Footprint) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();

        dx < (self.x + other.x) / 2.0 && dy < (self.y + other.y) / 2.0
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = leak_tray_deck();
    export(OUTPUTS[0], &deck);

    let source = source_suspension_handoff();
    export(OUTPUTS[1], &source);

    let grid = cassette_chip_position_grid();
    export(OUTPUTS[2], &grid);

    let wells = row_column_witness_wells();
    export(OUTPUTS[3], &wells);

    let coupons = edge_center_coupon_grid();
    export(OUTPUTS[4], &coupons);

    let settle = timed_settle_token_rail();
    export(OUTPUTS[5], &settle);

    let mix = gentle_mix_evidence_panel();
    export(OUTPUTS[6], &mix);

    let bubble = bubble_dead_volume_windows();
    export(OUTPUTS[7], &bubble);

    let custody = barcode_coa_custody_board();
    export(OUTPUTS[8], &custody);

    let gates = release_hold_reject_gates();
    export(OUTPUTS[9], &gates);

    let bridge = camera_evidence_bridge();
    export(OUTPUTS[10], &bridge);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[11], &keepouts);

    let route = closed_route_harness();
    export(OUTPUTS[12], &route);

    let assembly = station_assembly();
    export(OUTPUTS[13], &assembly);

    println!();
    println!("Closed cassette seed-density gradient settle witness station:");
    println!(
        "  Cassette/chip map:       {CASSETTE_ROWS} rows x {CASSETTE_COLS} columns = {POSITION_COUNT} positions ({EDGE_POSITION_COUNT} edge / {CENTER_POSITION_COUNT} center)"
    );
    println!(
        "  Density witnesses:       {WITNESS_WELL_COUNT} row/column wells, {DENSITY_GRADIENT_LEVELS} gradient levels, {POSITION_COUNT} edge/center coupons"
    );
    println!(
        "  Settle timing:           {SETTLE_TOKEN_COUNT} tokens at {:?} minutes with {SETTLE_LANE_COUNT} evidence lanes",
        SETTLE_MINUTES
    );
    println!(
        "  Closed-source evidence:  {SOURCE_DRY_BREAKS} dry-breaks, {SOURCE_DENSITY_DRAW_PORTS} density draw ports, {MIX_ROLLERS} gentle-mix rollers"
    );
    println!(
        "  Bubble/dead-volume:      {BUBBLE_WINDOWS} position windows and {DEAD_VOLUME_WINDOWS} dead-volume windows"
    );
    println!(
        "  Custody/disposition:     {POSITION_BARCODE_LANDS} barcode lands, {COA_CARD_SLOTS} COA slots, release/hold/reject gates"
    );
    println!(
        "  Evidence bridge:         {CAMERA_PODS} camera pods, {CAMERA_FIDUCIALS} fiducials, {:.0}mm chip clearance",
        camera_bridge_clearance_above_chip()
    );
    println!("  STL outputs:             {} files", OUTPUTS.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    leak_tray_deck()
        + source_suspension_handoff()
        + cassette_chip_position_grid()
        + row_column_witness_wells()
        + edge_center_coupon_grid()
        + timed_settle_token_rail()
        + gentle_mix_evidence_panel()
        + bubble_dead_volume_windows()
        + barcode_coa_custody_board()
        + release_hold_reject_gates()
        + camera_evidence_bridge()
        + robot_service_keepouts()
        + closed_route_harness()
}

fn leak_tray_deck() -> Part {
    let deck = centered_cube(
        format!("{OUTPUT_PREFIX}_deck_plate"),
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let basin = centered_cube(
        format!("{OUTPUT_PREFIX}_closed_station_shallow_basin_cut"),
        DECK_X - 150.0,
        DECK_Y - 150.0,
        BASIN_DEPTH + 2.0,
    )
    .translate(0.0, 0.0, DECK_Z - BASIN_DEPTH / 2.0 + 0.5);
    let drain = centered_cube(
        format!("{OUTPUT_PREFIX}_front_drain_witness_channel_cut"),
        DECK_X - 170.0,
        DRAIN_CHANNEL_W,
        BASIN_DEPTH + 2.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 80.0, DECK_Z - BASIN_DEPTH / 2.0 + 0.5);

    deck - basin - drain - deck_mount_holes()
        + deck_perimeter_rims()
        + deck_floor_shadows()
        + deck_flow_arrows()
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty(format!("{OUTPUT_PREFIX}_mounting_holes"));
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
                format!("{OUTPUT_PREFIX}_m6_mount_hole_{i}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 4.0,
                32,
            )
            .translate(x, y, DECK_Z / 2.0);
    }
    holes
}

fn deck_perimeter_rims() -> Part {
    let front = centered_cube(
        format!("{OUTPUT_PREFIX}_front_clean_wipe_rim"),
        DECK_X,
        TRAY_RIM_W,
        TRAY_RIM_Z,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + TRAY_RIM_W / 2.0,
        DECK_Z + TRAY_RIM_Z / 2.0,
    );
    let rear = centered_cube(
        format!("{OUTPUT_PREFIX}_rear_clean_wipe_rim"),
        DECK_X,
        TRAY_RIM_W,
        TRAY_RIM_Z,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - TRAY_RIM_W / 2.0,
        DECK_Z + TRAY_RIM_Z / 2.0,
    );
    let left = centered_cube(
        format!("{OUTPUT_PREFIX}_left_clean_wipe_rim"),
        TRAY_RIM_W,
        DECK_Y,
        TRAY_RIM_Z,
    )
    .translate(
        -DECK_X / 2.0 + TRAY_RIM_W / 2.0,
        0.0,
        DECK_Z + TRAY_RIM_Z / 2.0,
    );
    let right = centered_cube(
        format!("{OUTPUT_PREFIX}_right_clean_wipe_rim"),
        TRAY_RIM_W,
        DECK_Y,
        TRAY_RIM_Z,
    )
    .translate(
        DECK_X / 2.0 - TRAY_RIM_W / 2.0,
        0.0,
        DECK_Z + TRAY_RIM_Z / 2.0,
    );

    front + rear + left + right
}

fn deck_floor_shadows() -> Part {
    let mut shadows = Part::empty(format!("{OUTPUT_PREFIX}_process_module_floor_shadows"));
    for footprint in primary_footprints() {
        shadows = shadows
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{}_floor_shadow", footprint.name),
                footprint.x + 14.0,
                footprint.y + 14.0,
                FLOOR_SHADOW_Z,
            )
            .translate(
                footprint.center.0,
                footprint.center.1,
                DECK_Z + FLOOR_SHADOW_Z / 2.0,
            );
    }
    shadows
}

fn deck_flow_arrows() -> Part {
    let mut arrows = Part::empty(format!("{OUTPUT_PREFIX}_closed_process_flow_arrows"));
    for (i, (x, y, length)) in [
        (-510.0, 130.0, 82.0),
        (160.0, 198.0, 96.0),
        (300.0, -80.0, 74.0),
        (200.0, -348.0, 88.0),
    ]
    .into_iter()
    .enumerate()
    {
        let shaft = centered_cube(
            format!("{OUTPUT_PREFIX}_flow_arrow_{i}_shaft"),
            length,
            6.0,
            6.0,
        )
        .translate(x, y, DECK_Z + 5.0);
        let head = centered_cube(
            format!("{OUTPUT_PREFIX}_flow_arrow_{i}_head"),
            16.0,
            18.0,
            6.0,
        )
        .translate(x + length / 2.0 + 8.0, y, DECK_Z + 5.0);
        arrows = arrows + shaft + head;
    }
    arrows
}

fn source_suspension_handoff() -> Part {
    let base = centered_cube(
        format!("{OUTPUT_PREFIX}_source_suspension_handoff_base"),
        SOURCE_X,
        SOURCE_Y,
        SOURCE_Z,
    );
    let source_pocket = centered_cube(
        format!("{OUTPUT_PREFIX}_source_bag_pocket_relief"),
        SOURCE_BAG_POCKET_X,
        SOURCE_BAG_POCKET_Y,
        SOURCE_BAG_POCKET_DEPTH + 1.0,
    )
    .translate(
        -32.0,
        24.0,
        SOURCE_Z / 2.0 - SOURCE_BAG_POCKET_DEPTH / 2.0 + 0.5,
    );
    let mut bores = Part::empty(format!("{OUTPUT_PREFIX}_source_closed_port_bores"));
    let mut features = Part::empty(format!("{OUTPUT_PREFIX}_source_handoff_features"));

    for i in 0..SOURCE_DRY_BREAKS {
        let x = centered_index(i, SOURCE_DRY_BREAKS, 44.0) + 12.0;
        let y = -SOURCE_Y / 2.0 + 42.0;
        bores = bores
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_source_dry_break_bore_{i}"),
                7.0,
                SOURCE_Z + 2.0,
                28,
            )
            .translate(x, y, 0.0);
        features = features
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_source_dry_break_collar_{i}"),
                15.0,
                7.0,
                32,
            )
            .translate(x, y, SOURCE_Z / 2.0 + 3.5)
            + centered_cube(
                format!("{OUTPUT_PREFIX}_source_dry_break_cap_park_{i}"),
                26.0,
                10.0,
                5.0,
            )
            .translate(x, y - 28.0, SOURCE_Z / 2.0 + 2.5);
    }

    for i in 0..SOURCE_DENSITY_DRAW_PORTS {
        let x = centered_index(i, SOURCE_DENSITY_DRAW_PORTS, 34.0) - 10.0;
        let y = SOURCE_Y / 2.0 - 34.0;
        features = features
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_source_density_draw_port_{i}"),
                9.0,
                6.0,
                28,
            )
            .translate(x, y, SOURCE_Z / 2.0 + 3.0)
            + centered_cube(
                format!("{OUTPUT_PREFIX}_source_draw_port_identity_land_{i}"),
                24.0,
                6.0,
                4.0,
            )
            .translate(x, y + 22.0, SOURCE_Z / 2.0 + 2.0);
    }

    for (i, (x, y)) in [
        (-SOURCE_X / 2.0 + 22.0, -SOURCE_Y / 2.0 + 22.0),
        (SOURCE_X / 2.0 - 22.0, -SOURCE_Y / 2.0 + 22.0),
        (-SOURCE_X / 2.0 + 22.0, SOURCE_Y / 2.0 - 22.0),
        (SOURCE_X / 2.0 - 22.0, SOURCE_Y / 2.0 - 22.0),
    ]
    .into_iter()
    .enumerate()
    {
        features = features
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_source_locator_pin_{i}"),
                5.0,
                14.0,
                28,
            )
            .translate(x, y, SOURCE_Z / 2.0 + 7.0);
    }

    let handoff_key = centered_cube(
        format!("{OUTPUT_PREFIX}_closed_handoff_keyed_distribution_face"),
        SOURCE_X - 36.0,
        8.0,
        12.0,
    )
    .translate(0.0, -SOURCE_Y / 2.0 - 6.0, SOURCE_Z / 2.0 + 6.0);

    (base - source_pocket - bores + features + handoff_key).translate(
        SOURCE_CENTER.0,
        SOURCE_CENTER.1,
        DECK_Z + SOURCE_Z / 2.0,
    )
}

fn cassette_chip_position_grid() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_cassette_chip_position_grid_body"),
        GRID_X,
        GRID_Y,
        GRID_Z,
    );
    let mut recesses = Part::empty(format!("{OUTPUT_PREFIX}_chip_position_socket_reliefs"));
    let mut features = Part::empty(format!("{OUTPUT_PREFIX}_chip_position_grid_features"));

    for position in 0..POSITION_COUNT {
        let (x, y) = chip_local_center(position);
        let zone = position_zone_label(position);
        let gradient = position_gradient_level(position);

        recesses = recesses
            + centered_cube(
                format!("{OUTPUT_PREFIX}_position_{position:02}_{zone}_gradient_{gradient}_socket"),
                CHIP_SOCKET_X,
                CHIP_SOCKET_Y,
                CHIP_SOCKET_DEPTH + 1.0,
            )
            .translate(x, y, GRID_Z / 2.0 - CHIP_SOCKET_DEPTH / 2.0 + 0.5);

        let rim_front = centered_cube(
            format!("{OUTPUT_PREFIX}_position_{position:02}_front_wipe_rim"),
            CHIP_SOCKET_X + CHIP_RIM_W * 2.0,
            CHIP_RIM_W,
            6.0,
        )
        .translate(
            x,
            y - CHIP_SOCKET_Y / 2.0 - CHIP_RIM_W / 2.0,
            GRID_Z / 2.0 + 3.0,
        );
        let rim_rear = centered_cube(
            format!("{OUTPUT_PREFIX}_position_{position:02}_rear_wipe_rim"),
            CHIP_SOCKET_X + CHIP_RIM_W * 2.0,
            CHIP_RIM_W,
            6.0,
        )
        .translate(
            x,
            y + CHIP_SOCKET_Y / 2.0 + CHIP_RIM_W / 2.0,
            GRID_Z / 2.0 + 3.0,
        );
        let rim_left = centered_cube(
            format!("{OUTPUT_PREFIX}_position_{position:02}_left_wipe_rim"),
            CHIP_RIM_W,
            CHIP_SOCKET_Y,
            6.0,
        )
        .translate(
            x - CHIP_SOCKET_X / 2.0 - CHIP_RIM_W / 2.0,
            y,
            GRID_Z / 2.0 + 3.0,
        );
        let rim_right = centered_cube(
            format!("{OUTPUT_PREFIX}_position_{position:02}_right_wipe_rim"),
            CHIP_RIM_W,
            CHIP_SOCKET_Y,
            6.0,
        )
        .translate(
            x + CHIP_SOCKET_X / 2.0 + CHIP_RIM_W / 2.0,
            y,
            GRID_Z / 2.0 + 3.0,
        );
        let marker_d = if is_edge_position(position) {
            EDGE_MARKER_D
        } else {
            CENTER_MARKER_D
        };
        let marker = centered_cylinder(
            format!("{OUTPUT_PREFIX}_position_{position:02}_{zone}_zone_marker"),
            marker_d / 2.0,
            5.0,
            32,
        )
        .translate(
            x - CHIP_SOCKET_X / 2.0 + 17.0,
            y + CHIP_SOCKET_Y / 2.0 - 16.0,
            GRID_Z / 2.0 + 2.5,
        );
        let gradient_strip = centered_cube(
            format!(
                "{OUTPUT_PREFIX}_position_{position:02}_density_gradient_level_{gradient}_strip"
            ),
            CHIP_SOCKET_X * (0.24 + gradient as f64 * 0.08),
            5.0,
            GRADIENT_STRIP_Z,
        )
        .translate(x, y - CHIP_SOCKET_Y / 2.0 + 15.0, GRID_Z / 2.0 + 2.0);
        let settle_cross_x = centered_cube(
            format!("{OUTPUT_PREFIX}_position_{position:02}_settle_centerline_x"),
            CHIP_SOCKET_X * 0.52,
            2.0,
            4.0,
        )
        .translate(x, y, GRID_Z / 2.0 + 2.0);
        let settle_cross_y = centered_cube(
            format!("{OUTPUT_PREFIX}_position_{position:02}_settle_centerline_y"),
            2.0,
            CHIP_SOCKET_Y * 0.46,
            4.0,
        )
        .translate(x, y, GRID_Z / 2.0 + 2.0);

        features = features
            + rim_front
            + rim_rear
            + rim_left
            + rim_right
            + marker
            + gradient_strip
            + settle_cross_x
            + settle_cross_y;
    }

    let left_datum = centered_cube(
        format!("{OUTPUT_PREFIX}_grid_left_hard_datum_rail"),
        12.0,
        GRID_Y,
        20.0,
    )
    .translate(-GRID_X / 2.0 + 6.0, 0.0, GRID_Z / 2.0 + 10.0);
    let rear_datum = centered_cube(
        format!("{OUTPUT_PREFIX}_grid_rear_hard_datum_rail"),
        GRID_X,
        12.0,
        20.0,
    )
    .translate(0.0, GRID_Y / 2.0 - 6.0, GRID_Z / 2.0 + 10.0);

    (body - recesses + features + left_datum + rear_datum).translate(
        GRID_CENTER.0,
        GRID_CENTER.1,
        DECK_Z + GRID_Z / 2.0,
    )
}

fn row_column_witness_wells() -> Part {
    let board = centered_cube(
        format!("{OUTPUT_PREFIX}_row_column_witness_well_board"),
        WELL_PANEL_X,
        WELL_PANEL_Y,
        WELL_PANEL_Z,
    );
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_row_column_witness_well_cuts"));
    let mut features = Part::empty(format!("{OUTPUT_PREFIX}_row_column_witness_well_features"));

    for row in 0..ROW_WITNESS_WELLS {
        let (x, y) = row_witness_well_local(row);
        cuts = cuts
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_row_{row}_witness_well_relief"),
                WITNESS_WELL_D / 2.0,
                WITNESS_WELL_DEPTH + 1.0,
                36,
            )
            .translate(x, y, WELL_PANEL_Z / 2.0 - WITNESS_WELL_DEPTH / 2.0 + 0.5);
        features = features
            + witness_well_rim(format!("{OUTPUT_PREFIX}_row_{row}_witness_well_rim"), x, y)
            + centered_cube(
                format!("{OUTPUT_PREFIX}_row_{row}_gradient_receipt_land"),
                26.0 + row as f64 * 7.0,
                7.0,
                4.0,
            )
            .translate(x, y + 34.0, WELL_PANEL_Z / 2.0 + 2.0);
    }

    for col in 0..COLUMN_WITNESS_WELLS {
        let (x, y) = column_witness_well_local(col);
        cuts = cuts
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_column_{col}_witness_well_relief"),
                WITNESS_WELL_D / 2.0,
                WITNESS_WELL_DEPTH + 1.0,
                36,
            )
            .translate(x, y, WELL_PANEL_Z / 2.0 - WITNESS_WELL_DEPTH / 2.0 + 0.5);
        features = features
            + witness_well_rim(
                format!("{OUTPUT_PREFIX}_column_{col}_witness_well_rim"),
                x,
                y,
            )
            + centered_cube(
                format!("{OUTPUT_PREFIX}_column_{col}_gradient_receipt_land"),
                7.0,
                28.0 + col as f64 * 7.0,
                4.0,
            )
            .translate(x + 32.0, y, WELL_PANEL_Z / 2.0 + 2.0);
    }

    let row_bus = centered_cube(
        format!("{OUTPUT_PREFIX}_row_witness_closed_fluid_bus"),
        WELL_PANEL_X - 56.0,
        WITNESS_CHANNEL_W,
        7.0,
    )
    .translate(-10.0, 72.0, WELL_PANEL_Z / 2.0 + 3.5);
    let column_bus = centered_cube(
        format!("{OUTPUT_PREFIX}_column_witness_closed_fluid_bus"),
        WELL_PANEL_X - 80.0,
        WITNESS_CHANNEL_W,
        7.0,
    )
    .translate(2.0, -70.0, WELL_PANEL_Z / 2.0 + 3.5);

    (board - cuts + features + row_bus + column_bus).translate(
        WELL_CENTER.0,
        WELL_CENTER.1,
        DECK_Z + WELL_PANEL_Z / 2.0,
    )
}

fn witness_well_rim(name: String, x: f64, y: f64) -> Part {
    centered_cylinder(name, WITNESS_WELL_RIM_D / 2.0, 5.0, 36).translate(
        x,
        y,
        WELL_PANEL_Z / 2.0 + 2.5,
    )
}

fn edge_center_coupon_grid() -> Part {
    let board = centered_cube(
        format!("{OUTPUT_PREFIX}_edge_center_coupon_grid_board"),
        COUPON_PANEL_X,
        COUPON_PANEL_Y,
        COUPON_PANEL_Z,
    );
    let mut coupons = Part::empty(format!("{OUTPUT_PREFIX}_edge_center_position_coupons"));

    for position in 0..POSITION_COUNT {
        let (x, y) = coupon_local_position(position);
        let zone = position_zone_label(position);
        let d = if is_edge_position(position) {
            COUPON_EDGE_D
        } else {
            COUPON_CENTER_D
        };
        let puck = centered_cylinder(
            format!("{OUTPUT_PREFIX}_position_{position:02}_{zone}_density_coupon"),
            d / 2.0,
            COUPON_Z,
            36,
        )
        .translate(x, y, COUPON_PANEL_Z / 2.0 + COUPON_Z / 2.0);
        let orientation = centered_cube(
            format!("{OUTPUT_PREFIX}_position_{position:02}_{zone}_coupon_orientation_tick"),
            4.0,
            d * 0.9,
            2.0,
        )
        .translate(x + d * 0.22, y, COUPON_PANEL_Z / 2.0 + COUPON_Z + 1.0);
        coupons = coupons + puck + orientation;
    }

    let divider = centered_cube(
        format!("{OUTPUT_PREFIX}_coupon_edge_center_divider"),
        COUPON_PANEL_X - 34.0,
        6.0,
        8.0,
    )
    .translate(0.0, 0.0, COUPON_PANEL_Z / 2.0 + 4.0);
    let gradient_rake = density_gradient_rake();

    (board + coupons + divider + gradient_rake).translate(
        COUPON_CENTER.0,
        COUPON_CENTER.1,
        DECK_Z + COUPON_PANEL_Z / 2.0,
    )
}

fn density_gradient_rake() -> Part {
    let mut rake = Part::empty(format!("{OUTPUT_PREFIX}_coupon_density_gradient_rake"));
    for level in 0..DENSITY_GRADIENT_LEVELS {
        rake = rake
            + centered_cube(
                format!("{OUTPUT_PREFIX}_coupon_density_gradient_level_{level}_rake_bar"),
                42.0 + level as f64 * 20.0,
                5.0,
                5.0,
            )
            .translate(
                0.0,
                -COUPON_PANEL_Y / 2.0 + 26.0 + level as f64 * 13.0,
                COUPON_PANEL_Z / 2.0 + 2.5,
            );
    }
    rake
}

fn timed_settle_token_rail() -> Part {
    let board = centered_cube(
        format!("{OUTPUT_PREFIX}_timed_settle_token_rail_board"),
        SETTLE_PANEL_X,
        SETTLE_PANEL_Y,
        SETTLE_PANEL_Z,
    );
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_timed_settle_token_slot_reliefs"));
    let mut features = Part::empty(format!("{OUTPUT_PREFIX}_timed_settle_token_rail_features"));

    for (i, minutes) in SETTLE_MINUTES.into_iter().enumerate() {
        let x = centered_index(i, SETTLE_TOKEN_COUNT, SETTLE_TOKEN_PITCH_X);
        cuts = cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_settle_{minutes:03}_minute_token_slot"),
                SETTLE_TOKEN_SLOT_X,
                SETTLE_TOKEN_SLOT_Y,
                9.0,
            )
            .translate(x, 22.0, SETTLE_PANEL_Z / 2.0 - 3.0);
        features = features
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_settle_{minutes:03}_minute_token_stop"),
                4.0,
                5.0,
                24,
            )
            .translate(
                x + SETTLE_TOKEN_SLOT_X / 2.0 + 7.0,
                22.0,
                SETTLE_PANEL_Z / 2.0 + 2.5,
            )
            + centered_cube(
                format!("{OUTPUT_PREFIX}_settle_{minutes:03}_minute_barcode_land"),
                24.0,
                7.0,
                4.0,
            )
            .translate(x, -20.0, SETTLE_PANEL_Z / 2.0 + 2.0);
    }

    for lane in 0..SETTLE_LANE_COUNT {
        features = features
            + centered_cube(
                format!("{OUTPUT_PREFIX}_settle_lane_{lane}_evidence_rail"),
                SETTLE_PANEL_X - 44.0,
                5.0,
                5.0,
            )
            .translate(
                0.0,
                -SETTLE_PANEL_Y / 2.0 + 24.0 + lane as f64 * 15.0,
                SETTLE_PANEL_Z / 2.0 + 2.5,
            );
    }

    (board - cuts + features).translate(
        SETTLE_CENTER.0,
        SETTLE_CENTER.1,
        DECK_Z + SETTLE_PANEL_Z / 2.0,
    )
}

fn gentle_mix_evidence_panel() -> Part {
    let board = centered_cube(
        format!("{OUTPUT_PREFIX}_gentle_mix_evidence_board"),
        MIX_PANEL_X,
        MIX_PANEL_Y,
        MIX_PANEL_Z,
    );
    let mut features = Part::empty(format!("{OUTPUT_PREFIX}_gentle_mix_evidence_features"));

    for i in 0..MIX_ROLLERS {
        let x = centered_index(i, MIX_ROLLERS, 70.0);
        features = features
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_gentle_mix_roller_witness_{i}"),
                16.0,
                8.0,
                36,
            )
            .translate(x, 52.0, MIX_PANEL_Z / 2.0 + 4.0)
            + centered_cube(
                format!("{OUTPUT_PREFIX}_gentle_mix_roller_contact_patch_{i}"),
                42.0,
                8.0,
                4.0,
            )
            .translate(x, 29.0, MIX_PANEL_Z / 2.0 + 2.0);
    }

    for i in 0..MIX_BAFFLE_RIBS {
        features = features
            + centered_cube(
                format!("{OUTPUT_PREFIX}_gentle_mix_low_shear_baffle_rib_{i}"),
                6.0,
                80.0,
                7.0,
            )
            .translate(
                centered_index(i, MIX_BAFFLE_RIBS, 34.0),
                -20.0,
                MIX_PANEL_Z / 2.0 + 3.5,
            );
    }

    for i in 0..MIX_SWEEP_TICKS {
        let x = centered_index(i, MIX_SWEEP_TICKS, 28.0);
        let y_offset = if i % 2 == 0 { -72.0 } else { -64.0 };
        features = features
            + centered_cube(
                format!("{OUTPUT_PREFIX}_gentle_mix_sweep_arc_tick_{i}"),
                4.0,
                22.0,
                5.0,
            )
            .translate(x, y_offset, MIX_PANEL_Z / 2.0 + 2.5);
    }

    for i in 0..MIX_EVIDENCE_WINDOWS {
        features = features
            + centered_cube(
                format!("{OUTPUT_PREFIX}_mix_evidence_window_{i}"),
                42.0,
                20.0,
                4.0,
            )
            .translate(
                centered_index(i, MIX_EVIDENCE_WINDOWS, 54.0),
                -94.0,
                MIX_PANEL_Z / 2.0 + 2.0,
            );
    }

    for i in 0..MIX_TOKEN_LANDS {
        features = features
            + centered_cube(
                format!("{OUTPUT_PREFIX}_mix_lot_and_time_token_land_{i}"),
                34.0,
                11.0,
                4.0,
            )
            .translate(
                -MIX_PANEL_X / 2.0 + 34.0 + i as f64 * 48.0,
                MIX_PANEL_Y / 2.0 - 18.0,
                MIX_PANEL_Z / 2.0 + 2.0,
            );
    }

    (board + features).translate(MIX_CENTER.0, MIX_CENTER.1, DECK_Z + MIX_PANEL_Z / 2.0)
}

fn bubble_dead_volume_windows() -> Part {
    let board = centered_cube(
        format!("{OUTPUT_PREFIX}_bubble_dead_volume_window_board"),
        BUBBLE_PANEL_X,
        BUBBLE_PANEL_Y,
        BUBBLE_PANEL_Z,
    );
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_bubble_dead_volume_window_cuts"));
    let mut features = Part::empty(format!(
        "{OUTPUT_PREFIX}_bubble_dead_volume_window_features"
    ));

    for position in 0..BUBBLE_WINDOWS {
        let col = position % BUBBLE_WINDOW_COLS;
        let row = position / BUBBLE_WINDOW_COLS;
        let x = centered_index(col, BUBBLE_WINDOW_COLS, 48.0);
        let y = centered_index(row, 2, 42.0) + 28.0;
        cuts = cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_position_{position:02}_bubble_sight_window_cut"),
                BUBBLE_WINDOW_X,
                BUBBLE_WINDOW_Y,
                8.0,
            )
            .translate(x, y, BUBBLE_PANEL_Z / 2.0 - 3.0);
        features = features
            + centered_cube(
                format!("{OUTPUT_PREFIX}_position_{position:02}_bubble_window_frame"),
                BUBBLE_WINDOW_X + 8.0,
                4.0,
                5.0,
            )
            .translate(
                x,
                y + BUBBLE_WINDOW_Y / 2.0 + 4.0,
                BUBBLE_PANEL_Z / 2.0 + 2.5,
            )
            + centered_cube(
                format!("{OUTPUT_PREFIX}_position_{position:02}_dead_zone_tick"),
                4.0 + (position % DENSITY_GRADIENT_LEVELS) as f64 * 2.0,
                16.0,
                4.0,
            )
            .translate(x + 18.0, y, BUBBLE_PANEL_Z / 2.0 + 2.0);
    }

    for i in 0..DEAD_VOLUME_WINDOWS {
        let x = centered_index(i, DEAD_VOLUME_WINDOWS, 50.0);
        cuts = cuts
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_dead_volume_window_{i}_relief"),
                DEAD_VOLUME_WINDOW_D / 2.0,
                8.0,
                28,
            )
            .translate(x, -50.0, BUBBLE_PANEL_Z / 2.0 - 3.0);
        features = features
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_dead_volume_window_{i}_rim"),
                DEAD_VOLUME_WINDOW_D / 2.0 + 5.0,
                4.0,
                28,
            )
            .translate(x, -50.0, BUBBLE_PANEL_Z / 2.0 + 2.0);
    }

    for i in 0..BUBBLE_LADDER_TICKS {
        features = features
            + centered_cube(
                format!("{OUTPUT_PREFIX}_bubble_ladder_tick_{i}"),
                20.0 + i as f64 * 4.0,
                4.0,
                4.0,
            )
            .translate(
                -BUBBLE_PANEL_X / 2.0 + 44.0,
                -BUBBLE_PANEL_Y / 2.0 + 18.0 + i as f64 * 10.0,
                BUBBLE_PANEL_Z / 2.0 + 2.0,
            );
    }

    (board - cuts + features).translate(
        BUBBLE_CENTER.0,
        BUBBLE_CENTER.1,
        DECK_Z + BUBBLE_PANEL_Z / 2.0,
    )
}

fn barcode_coa_custody_board() -> Part {
    let board = centered_cube(
        format!("{OUTPUT_PREFIX}_barcode_coa_custody_board"),
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    let mut features = Part::empty(format!("{OUTPUT_PREFIX}_barcode_coa_custody_features"));

    for i in 0..POSITION_BARCODE_LANDS {
        let col = i % 10;
        let row = i / 10;
        features = features
            + centered_cube(
                format!("{OUTPUT_PREFIX}_position_{i:02}_barcode_land"),
                22.0,
                8.0,
                4.0,
            )
            .translate(
                -CUSTODY_X / 2.0 + 34.0 + col as f64 * 27.0,
                18.0 - row as f64 * 22.0,
                CUSTODY_Z / 2.0 + 2.0,
            );
    }

    for i in 0..COA_CARD_SLOTS {
        features = features
            + centered_cube(
                format!("{OUTPUT_PREFIX}_coa_card_slot_{i}"),
                64.0,
                14.0,
                5.0,
            )
            .translate(
                -CUSTODY_X / 2.0 + 54.0 + i as f64 * 80.0,
                -CUSTODY_Y / 2.0 + 16.0,
                CUSTODY_Z / 2.0 + 2.5,
            );
    }

    for i in 0..RFID_LANDS {
        features = features
            + centered_cube(
                format!("{OUTPUT_PREFIX}_rfid_custody_land_{i}"),
                24.0,
                24.0,
                4.0,
            )
            .translate(
                CUSTODY_X / 2.0 - 26.0,
                centered_index(i, RFID_LANDS, 24.0),
                CUSTODY_Z / 2.0 + 2.0,
            );
    }

    for i in 0..SEAL_WITNESS_TABS {
        features = features
            + centered_cube(
                format!("{OUTPUT_PREFIX}_tamper_seal_witness_tab_{i}"),
                34.0,
                6.0,
                4.0,
            )
            .translate(
                -CUSTODY_X / 2.0 + 36.0 + i as f64 * 58.0,
                CUSTODY_Y / 2.0 - 14.0,
                CUSTODY_Z / 2.0 + 2.0,
            );
    }

    (board + features).translate(CUSTODY_CENTER.0, CUSTODY_CENTER.1, DECK_Z + CUSTODY_Z / 2.0)
}

fn release_hold_reject_gates() -> Part {
    let board = centered_cube(
        format!("{OUTPUT_PREFIX}_release_hold_reject_gate_board"),
        GATE_PANEL_X,
        GATE_PANEL_Y,
        GATE_PANEL_Z,
    );
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_release_hold_reject_gate_cuts"));
    let mut features = Part::empty(format!("{OUTPUT_PREFIX}_release_hold_reject_gate_features"));

    for lane in 0..GATE_LANES {
        let label = disposition_label(lane);
        let x = centered_index(lane, GATE_LANES, GATE_LANE_PITCH_X);
        cuts = cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{label}_gate_lane_recess"),
                GATE_LANE_X,
                GATE_LANE_Y,
                10.0,
            )
            .translate(x, 0.0, GATE_PANEL_Z / 2.0 - 4.0);

        features = features
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{label}_gate_header_land"),
                GATE_LANE_X - 24.0,
                8.0,
                5.0,
            )
            .translate(x, GATE_LANE_Y / 2.0 - 8.0, GATE_PANEL_Z / 2.0 + 2.5)
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{label}_physical_gate_bar"),
                12.0,
                GATE_LANE_Y + 8.0,
                9.0,
            )
            .translate(x + GATE_LANE_X / 2.0 - 18.0, 0.0, GATE_PANEL_Z / 2.0 + 4.5);

        for token in 0..GATE_TOKENS_PER_LANE {
            features = features
                + centered_cylinder(
                    format!("{OUTPUT_PREFIX}_{label}_decision_token_stop_{token}"),
                    4.0,
                    5.0,
                    20,
                )
                .translate(
                    x - GATE_LANE_X / 2.0 + 24.0 + token as f64 * 24.0,
                    -GATE_LANE_Y / 2.0 + 13.0,
                    GATE_PANEL_Z / 2.0 + 2.5,
                );
        }
    }

    (board - cuts + features).translate(GATE_CENTER.0, GATE_CENTER.1, DECK_Z + GATE_PANEL_Z / 2.0)
}

fn camera_evidence_bridge() -> Part {
    let post_height = CAMERA_UNDERSIDE_Z - DECK_Z;
    let x_span = GRID_X + 110.0;
    let y_span = GRID_Y + 96.0;
    let mut bridge = Part::empty(format!("{OUTPUT_PREFIX}_camera_evidence_bridge"));

    for (i, (sx, sy)) in [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)]
        .into_iter()
        .enumerate()
    {
        bridge = bridge
            + centered_cube(
                format!("{OUTPUT_PREFIX}_camera_bridge_post_{i}"),
                CAMERA_POST_X,
                CAMERA_POST_Y,
                post_height,
            )
            .translate(
                GRID_CENTER.0 + sx * x_span / 2.0,
                GRID_CENTER.1 + sy * y_span / 2.0,
                DECK_Z + post_height / 2.0,
            );
    }

    let front_beam = centered_cube(
        format!("{OUTPUT_PREFIX}_camera_bridge_front_beam"),
        x_span + CAMERA_POST_X,
        CAMERA_BEAM_W,
        CAMERA_BEAM_Z,
    )
    .translate(
        GRID_CENTER.0,
        GRID_CENTER.1 - y_span / 2.0,
        CAMERA_UNDERSIDE_Z + CAMERA_BEAM_Z / 2.0,
    );
    let rear_beam = centered_cube(
        format!("{OUTPUT_PREFIX}_camera_bridge_rear_beam"),
        x_span + CAMERA_POST_X,
        CAMERA_BEAM_W,
        CAMERA_BEAM_Z,
    )
    .translate(
        GRID_CENTER.0,
        GRID_CENTER.1 + y_span / 2.0,
        CAMERA_UNDERSIDE_Z + CAMERA_BEAM_Z / 2.0,
    );
    let left_beam = centered_cube(
        format!("{OUTPUT_PREFIX}_camera_bridge_left_beam"),
        CAMERA_BEAM_W,
        y_span,
        CAMERA_BEAM_Z,
    )
    .translate(
        GRID_CENTER.0 - x_span / 2.0,
        GRID_CENTER.1,
        CAMERA_UNDERSIDE_Z + CAMERA_BEAM_Z / 2.0,
    );
    let right_beam = centered_cube(
        format!("{OUTPUT_PREFIX}_camera_bridge_right_beam"),
        CAMERA_BEAM_W,
        y_span,
        CAMERA_BEAM_Z,
    )
    .translate(
        GRID_CENTER.0 + x_span / 2.0,
        GRID_CENTER.1,
        CAMERA_UNDERSIDE_Z + CAMERA_BEAM_Z / 2.0,
    );

    bridge = bridge + front_beam + rear_beam + left_beam + right_beam;

    for i in 0..CAMERA_PODS {
        let col = i % 2;
        let row = i / 2;
        bridge = bridge
            + centered_cube(
                format!("{OUTPUT_PREFIX}_camera_pod_{i}"),
                CAMERA_POD_X,
                CAMERA_POD_Y,
                CAMERA_POD_Z,
            )
            .translate(
                GRID_CENTER.0 + centered_index(col, 2, x_span * 0.42),
                GRID_CENTER.1 + centered_index(row, 2, y_span * 0.34),
                CAMERA_UNDERSIDE_Z + CAMERA_BEAM_Z + CAMERA_POD_Z / 2.0,
            );
    }

    for i in 0..CAMERA_FIDUCIALS {
        let (x, y) = camera_fiducial_position(i, x_span, y_span);
        bridge = bridge
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_camera_fiducial_target_{i}"),
                8.0,
                4.0,
                32,
            )
            .translate(x, y, CAMERA_UNDERSIDE_Z - 6.0);
    }

    bridge
}

fn robot_service_keepouts() -> Part {
    let front = centered_cube(
        format!("{OUTPUT_PREFIX}_front_robot_keepout_gauge"),
        KEEP_OUT_X,
        KEEP_OUT_Z,
        KEEP_OUT_Z,
    )
    .translate(0.0, -KEEP_OUT_Y / 2.0, DECK_Z + KEEP_OUT_Z / 2.0);
    let rear = centered_cube(
        format!("{OUTPUT_PREFIX}_rear_service_keepout_gauge"),
        KEEP_OUT_X,
        KEEP_OUT_Z,
        KEEP_OUT_Z,
    )
    .translate(0.0, KEEP_OUT_Y / 2.0, DECK_Z + KEEP_OUT_Z / 2.0);
    let left = centered_cube(
        format!("{OUTPUT_PREFIX}_left_source_service_keepout_gauge"),
        KEEP_OUT_Z,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(-KEEP_OUT_X / 2.0, 0.0, DECK_Z + KEEP_OUT_Z / 2.0);
    let right = centered_cube(
        format!("{OUTPUT_PREFIX}_right_witness_service_keepout_gauge"),
        KEEP_OUT_Z,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(KEEP_OUT_X / 2.0, 0.0, DECK_Z + KEEP_OUT_Z / 2.0);
    let camera_height = centered_cube(
        format!("{OUTPUT_PREFIX}_top_camera_clearance_gauge"),
        34.0,
        34.0,
        CAMERA_CLEARANCE_Z,
    )
    .translate(
        GRID_CENTER.0,
        GRID_CENTER.1,
        DECK_Z + CAMERA_CLEARANCE_Z / 2.0,
    );
    let source_service = centered_cube(
        format!("{OUTPUT_PREFIX}_source_bag_service_pull_keepout"),
        LEFT_SOURCE_SERVICE_CLEARANCE,
        18.0,
        34.0,
    )
    .translate(
        SOURCE_CENTER.0,
        SOURCE_CENTER.1 + SOURCE_Y / 2.0 + 28.0,
        DECK_Z + 17.0,
    );
    let witness_service = centered_cube(
        format!("{OUTPUT_PREFIX}_row_column_witness_service_keepout"),
        RIGHT_WITNESS_SERVICE_CLEARANCE,
        18.0,
        34.0,
    )
    .translate(
        WELL_CENTER.0,
        WELL_CENTER.1 - WELL_PANEL_Y / 2.0 - 24.0,
        DECK_Z + 17.0,
    );

    front + rear + left + right + camera_height + source_service + witness_service
}

fn closed_route_harness() -> Part {
    let mut harness = Part::empty(format!("{OUTPUT_PREFIX}_closed_route_harness"));
    for (i, (x, y, length, horizontal)) in [
        (-515.0, 190.0, 150.0, true),
        (-435.0, 145.0, 90.0, false),
        (-300.0, 145.0, 260.0, true),
        (160.0, 205.0, 155.0, true),
        (245.0, 112.0, 185.0, false),
        (365.0, 20.0, 210.0, true),
        (342.0, -150.0, 250.0, true),
        (210.0, -276.0, 245.0, false),
        (40.0, -338.0, 300.0, true),
        (360.0, -360.0, 270.0, true),
    ]
    .into_iter()
    .enumerate()
    {
        let (sx, sy) = if horizontal {
            (length, ROUTE_TUBE_W)
        } else {
            (ROUTE_TUBE_W, length)
        };
        harness = harness
            + centered_cube(
                format!("{OUTPUT_PREFIX}_closed_route_segment_{i}"),
                sx,
                sy,
                ROUTE_TUBE_W,
            )
            .translate(x, y, ROUTE_Z);
    }

    for i in 0..ROUTE_ELBOWS {
        harness = harness
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_closed_route_elbow_{i}"),
                ROUTE_TUBE_W,
                ROUTE_TUBE_W,
                24,
            )
            .translate(
                -430.0 + i as f64 * 120.0,
                160.0 - (i % 4) as f64 * 118.0,
                ROUTE_Z,
            );
    }

    for i in 0..ROUTE_DIRECTION_MARKERS {
        harness = harness
            + centered_cube(
                format!("{OUTPUT_PREFIX}_closed_route_direction_marker_{i}"),
                16.0,
                6.0,
                5.0,
            )
            .translate(
                -475.0 + i as f64 * 118.0,
                176.0 - (i % 3) as f64 * 170.0,
                ROUTE_Z + 8.0,
            );
    }

    harness
}

fn row_witness_well_local(row: usize) -> (f64, f64) {
    (centered_index(row, ROW_WITNESS_WELLS, 48.0) - 30.0, 48.0)
}

fn column_witness_well_local(col: usize) -> (f64, f64) {
    (
        centered_index(col, COLUMN_WITNESS_WELLS, 58.0) + 22.0,
        -52.0,
    )
}

fn chip_local_center(position: usize) -> (f64, f64) {
    let col = position % CASSETTE_COLS;
    let row = position / CASSETTE_COLS;
    (
        centered_index(col, CASSETTE_COLS, CHIP_PITCH_X),
        centered_index(row, CASSETTE_ROWS, CHIP_PITCH_Y),
    )
}

fn coupon_local_position(position: usize) -> (f64, f64) {
    let col = position % CASSETTE_COLS;
    let row = position / CASSETTE_COLS;
    (
        centered_index(col, CASSETTE_COLS, COUPON_PITCH_X),
        centered_index(row, CASSETTE_ROWS, COUPON_PITCH_Y),
    )
}

fn camera_fiducial_position(index: usize, x_span: f64, y_span: f64) -> (f64, f64) {
    let side = index / 2;
    let offset = if index % 2 == 0 { -0.28 } else { 0.28 };
    match side {
        0 => (
            GRID_CENTER.0 + offset * x_span,
            GRID_CENTER.1 - y_span / 2.0 + 22.0,
        ),
        1 => (
            GRID_CENTER.0 + offset * x_span,
            GRID_CENTER.1 + y_span / 2.0 - 22.0,
        ),
        2 => (
            GRID_CENTER.0 - x_span / 2.0 + 22.0,
            GRID_CENTER.1 + offset * y_span,
        ),
        _ => (
            GRID_CENTER.0 + x_span / 2.0 - 22.0,
            GRID_CENTER.1 + offset * y_span,
        ),
    }
}

fn is_edge_position(position: usize) -> bool {
    let col = position % CASSETTE_COLS;
    let row = position / CASSETTE_COLS;
    row == 0 || row == CASSETTE_ROWS - 1 || col == 0 || col == CASSETTE_COLS - 1
}

fn edge_position_count() -> usize {
    (0..POSITION_COUNT)
        .filter(|position| is_edge_position(*position))
        .count()
}

fn center_position_count() -> usize {
    POSITION_COUNT - edge_position_count()
}

fn position_zone_label(position: usize) -> &'static str {
    if is_edge_position(position) {
        "edge"
    } else {
        "center"
    }
}

fn position_gradient_level(position: usize) -> usize {
    let col = position % CASSETTE_COLS;
    let row = position / CASSETTE_COLS;
    (row + col) % DENSITY_GRADIENT_LEVELS
}

fn disposition_label(lane: usize) -> &'static str {
    match lane {
        0 => "release",
        1 => "hold",
        2 => "reject",
        _ => "unknown",
    }
}

fn camera_bridge_clearance_above_chip() -> f64 {
    CAMERA_UNDERSIDE_Z - (DECK_Z + GRID_Z + REVC_TOTAL_HEIGHT)
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn primary_footprints() -> [Footprint; 9] {
    [
        Footprint {
            name: "source_suspension_handoff",
            center: SOURCE_CENTER,
            x: SOURCE_X,
            y: SOURCE_Y,
        },
        Footprint {
            name: "cassette_chip_position_grid",
            center: GRID_CENTER,
            x: GRID_X,
            y: GRID_Y,
        },
        Footprint {
            name: "row_column_witness_wells",
            center: WELL_CENTER,
            x: WELL_PANEL_X,
            y: WELL_PANEL_Y,
        },
        Footprint {
            name: "edge_center_coupon_grid",
            center: COUPON_CENTER,
            x: COUPON_PANEL_X,
            y: COUPON_PANEL_Y,
        },
        Footprint {
            name: "timed_settle_token_rail",
            center: SETTLE_CENTER,
            x: SETTLE_PANEL_X,
            y: SETTLE_PANEL_Y,
        },
        Footprint {
            name: "gentle_mix_evidence_panel",
            center: MIX_CENTER,
            x: MIX_PANEL_X,
            y: MIX_PANEL_Y,
        },
        Footprint {
            name: "bubble_dead_volume_windows",
            center: BUBBLE_CENTER,
            x: BUBBLE_PANEL_X,
            y: BUBBLE_PANEL_Y,
        },
        Footprint {
            name: "barcode_coa_custody_board",
            center: CUSTODY_CENTER,
            x: CUSTODY_X,
            y: CUSTODY_Y,
        },
        Footprint {
            name: "release_hold_reject_gates",
            center: GATE_CENTER,
            x: GATE_PANEL_X,
            y: GATE_PANEL_Y,
        },
    ]
}

fn camera_bridge_footprint() -> Footprint {
    Footprint {
        name: "camera_evidence_bridge",
        center: GRID_CENTER,
        x: GRID_X + 110.0,
        y: GRID_Y + 96.0,
    }
}

fn keepout_footprint() -> Footprint {
    Footprint {
        name: "robot_service_keepout_outline",
        center: (0.0, 0.0),
        x: KEEP_OUT_X,
        y: KEEP_OUT_Y,
    }
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 14, "expected stable STL export count");
    assert_eq!(POSITION_COUNT, 20, "cassette position count changed");
    assert_eq!(
        edge_position_count(),
        EDGE_POSITION_COUNT,
        "edge position count changed"
    );
    assert_eq!(
        center_position_count(),
        CENTER_POSITION_COUNT,
        "center position count changed"
    );
    assert_eq!(
        WITNESS_WELL_COUNT, 9,
        "row/column witness wells must cover all rows and columns"
    );
    assert_eq!(
        BUBBLE_WINDOWS, POSITION_COUNT,
        "bubble windows must cover every cassette position"
    );
    assert_eq!(
        POSITION_BARCODE_LANDS, POSITION_COUNT,
        "barcode custody lands must cover every cassette position"
    );
    assert_eq!(
        SOURCE_LOCATOR_PINS, 4,
        "source suspension handoff locator pin count changed"
    );
    assert_eq!(GATE_LANES, 3, "release/hold/reject gate count changed");
    assert_eq!(ROUTE_SEGMENTS, 10, "closed route segment count changed");
    assert_eq!(ROUTE_ELBOWS, 8, "closed route elbow count changed");
    assert!(
        SETTLE_MINUTES
            .windows(2)
            .all(|window| window[0] < window[1]),
        "settle-time token rail must increase monotonically"
    );
    assert!(
        GRID_Z > REVC_TOTAL_HEIGHT + 20.0,
        "cassette grid must clear Rev C chip height"
    );
    assert!(
        camera_bridge_clearance_above_chip() > 120.0,
        "camera bridge lacks chip clearance"
    );
    assert!(
        FRONT_ROBOT_CLEARANCE >= 380.0
            && REAR_SERVICE_CLEARANCE >= 270.0
            && LEFT_SOURCE_SERVICE_CLEARANCE >= 250.0
            && RIGHT_WITNESS_SERVICE_CLEARANCE >= 240.0,
        "robot/service clearances below station target"
    );
    assert!(
        TOP_CAMERA_SERVICE_CLEARANCE_Z >= CAMERA_CLEARANCE_Z,
        "camera service clearance gauge is undersized"
    );
    assert_eq!(KEEP_OUT_ZONES, 6, "keepout zone count changed");

    let footprints = primary_footprints();
    for footprint in footprints {
        assert!(
            footprint.fits_on_deck(),
            "{} does not fit inside the deck",
            footprint.name
        );
    }
    assert!(
        camera_bridge_footprint().fits_on_deck(),
        "camera bridge footprint does not fit inside the deck"
    );
    assert!(
        keepout_footprint().fits_on_deck(),
        "robot/service keepout outline does not fit inside the deck"
    );
    for left in 0..footprints.len() {
        for right in left + 1..footprints.len() {
            assert!(
                !footprints[left].overlaps(footprints[right]),
                "{} overlaps {}",
                footprints[left].name,
                footprints[right].name
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();

        assert_eq!(OUTPUTS.len(), 14);
        assert_eq!(unique.len(), OUTPUTS.len());
        assert!(OUTPUTS.iter().all(|path| path.starts_with("output/")));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
        assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
        assert!(OUTPUTS[0].ends_with("_leak_tray_deck.stl"));
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));

        for feature in REQUIRED_FEATURES {
            assert!(
                OUTPUTS.iter().any(|path| path.contains(feature)),
                "missing output for required feature {feature}"
            );
        }
    }

    #[test]
    fn cassette_grid_covers_edge_center_and_density_gradients() {
        assert_eq!(CASSETTE_ROWS, 5);
        assert_eq!(CASSETTE_COLS, 4);
        assert_eq!(POSITION_COUNT, 20);
        assert_eq!(edge_position_count(), 14);
        assert_eq!(center_position_count(), 6);
        assert_eq!(EDGE_POSITION_COUNT + CENTER_POSITION_COUNT, POSITION_COUNT);
        assert!(CHIP_SOCKET_X > REVC_CHIP_LENGTH);
        assert!(CHIP_SOCKET_Y > REVC_CHIP_WIDTH);

        let mut levels_seen = [false; DENSITY_GRADIENT_LEVELS];
        for position in 0..POSITION_COUNT {
            levels_seen[position_gradient_level(position)] = true;
        }
        assert!(levels_seen.into_iter().all(|seen| seen));
    }

    #[test]
    fn witness_timing_mix_and_bubble_controls_are_explicit() {
        assert_eq!(ROW_WITNESS_WELLS, CASSETTE_ROWS);
        assert_eq!(COLUMN_WITNESS_WELLS, CASSETTE_COLS);
        assert_eq!(WITNESS_WELL_COUNT, ROW_WITNESS_WELLS + COLUMN_WITNESS_WELLS);
        assert_eq!(SETTLE_TOKEN_COUNT, SETTLE_MINUTES.len());
        assert_eq!(SETTLE_MINUTES[0], 0);
        assert!(SETTLE_MINUTES[SETTLE_TOKEN_COUNT - 1] >= 120);
        assert_eq!(MIX_ROLLERS, 4);
        assert_eq!(MIX_BAFFLE_RIBS, 6);
        assert_eq!(BUBBLE_WINDOWS, POSITION_COUNT);
        assert_eq!(DEAD_VOLUME_WINDOWS, 8);
    }

    #[test]
    fn custody_disposition_camera_and_keepouts_are_complete() {
        for control in VALIDATION_CONTROLS {
            assert!(!control.is_empty());
        }
        assert_eq!(POSITION_BARCODE_LANDS, POSITION_COUNT);
        assert_eq!(COA_CARD_SLOTS, 3);
        assert_eq!(RFID_LANDS, 4);
        assert_eq!(GATE_LANES, 3);
        assert_eq!(GATE_TOKENS_PER_LANE * GATE_LANES, 12);
        assert_eq!(CAMERA_PODS, 4);
        assert_eq!(CAMERA_FIDUCIALS, 8);
        assert_eq!(KEEP_OUT_ZONES, 6);
        assert!(camera_bridge_clearance_above_chip() > 120.0);
    }

    #[test]
    fn station_layout_bounds_and_primary_modules_do_not_overlap() {
        assert_design_constraints();
        assert_eq!(primary_footprints().len(), 9);
        assert!(camera_bridge_footprint().fits_on_deck());
        assert!(keepout_footprint().fits_on_deck());
    }

    #[test]
    fn closed_route_and_handoff_interfaces_cover_the_process_chain() {
        assert_eq!(SOURCE_DRY_BREAKS, 4);
        assert_eq!(SOURCE_DENSITY_DRAW_PORTS, DENSITY_GRADIENT_LEVELS);
        assert_eq!(SOURCE_LOCATOR_PINS, 4);
        assert_eq!(ROUTE_SEGMENTS, 10);
        assert_eq!(ROUTE_ELBOWS, 8);
        assert_eq!(ROUTE_DIRECTION_MARKERS, 8);
        assert!(SOURCE_Z > REVC_TOTAL_HEIGHT);
        assert!(ROUTE_Z > DECK_Z + GRID_Z);
    }
}
