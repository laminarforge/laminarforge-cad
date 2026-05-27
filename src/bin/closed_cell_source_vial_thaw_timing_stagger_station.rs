use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed-system cell-source vial thaw timing/stagger reproducibility station.
//
// The generator models mechanical validation hardware for repeatable pre-seeding
// thaw timing: indexed vial nests, stagger timer rails, probe wells, barcode
// custody slots, thaw start/end witness indicators, sterile closed-transfer
// staging, condensate containment, and raised block-letter labels. It is product
// concept CAD only; thaw recipes, biological acceptance criteria, aseptic
// processing claims, and release decisions remain external validation controls.

const OUTPUT_PREFIX: &str = "closed_cell_source_vial_thaw_timing_stagger_station";

const OUTPUTS: [&str; 12] = [
    "output/closed_cell_source_vial_thaw_timing_stagger_station_base_containment_deck.stl",
    "output/closed_cell_source_vial_thaw_timing_stagger_station_timed_vial_nest_ladder.stl",
    "output/closed_cell_source_vial_thaw_timing_stagger_station_stagger_timing_rail.stl",
    "output/closed_cell_source_vial_thaw_timing_stagger_station_temperature_probe_well_bank.stl",
    "output/closed_cell_source_vial_thaw_timing_stagger_station_barcode_custody_slots.stl",
    "output/closed_cell_source_vial_thaw_timing_stagger_station_thaw_start_end_witness_indicators.stl",
    "output/closed_cell_source_vial_thaw_timing_stagger_station_sterile_transfer_staging.stl",
    "output/closed_cell_source_vial_thaw_timing_stagger_station_drain_condensate_containment.stl",
    "output/closed_cell_source_vial_thaw_timing_stagger_station_evidence_camera_bridge.stl",
    "output/closed_cell_source_vial_thaw_timing_stagger_station_robot_service_keepouts.stl",
    "output/closed_cell_source_vial_thaw_timing_stagger_station_human_readable_labels.stl",
    "output/closed_cell_source_vial_thaw_timing_stagger_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 9] = [
    "timed_vial_nests",
    "temperature_probe_wells",
    "barcode_slots",
    "thaw_start_witness_indicators",
    "thaw_end_witness_indicators",
    "sterile_transfer_staging",
    "drain_condensate_containment",
    "human_readable_csg_labels",
    "named_stl_outputs",
];

const STATION_X: f64 = 1480.0;
const STATION_Y: f64 = 920.0;
const DECK_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const BASIN_DEPTH: f64 = 8.0;
const BASIN_MARGIN_X: f64 = 108.0;
const BASIN_MARGIN_Y: f64 = 122.0;
const DRAIN_PORT_D: f64 = 18.0;
const MOUNT_BOSS_COUNT: usize = 8;
const MOUNT_BOSS_D: f64 = 32.0;
const MOUNT_HOLE_D: f64 = 6.4;

const VIAL_COUNT: usize = 6;
const VIAL_COLS: usize = 3;
const VIAL_ROWS: usize = 2;
const STAGGER_MINUTES: [usize; VIAL_COUNT] = [0, 3, 6, 9, 12, 15];
const NEST_CENTER: (f64, f64) = (-350.0, 95.0);
const NEST_PANEL_X: f64 = 620.0;
const NEST_PANEL_Y: f64 = 420.0;
const NEST_PANEL_Z: f64 = 54.0;
const VIAL_PITCH_X: f64 = 170.0;
const VIAL_PITCH_Y: f64 = 160.0;
const VIAL_WELL_D: f64 = 33.0;
const VIAL_WELL_DEPTH: f64 = 36.0;
const VIAL_COLLAR_D: f64 = 54.0;
const VIAL_COLLAR_Z: f64 = 14.0;
const TIMER_TOKEN_SLOT_X: f64 = 68.0;
const TIMER_TOKEN_SLOT_Y: f64 = 22.0;
const TIMER_TOKEN_SLOT_DEPTH: f64 = 7.0;

const TIMING_RAIL_CENTER: (f64, f64) = (0.0, 390.0);
const TIMING_RAIL_X: f64 = 1220.0;
const TIMING_RAIL_Y: f64 = 76.0;
const TIMING_RAIL_Z: f64 = 26.0;
const TIMING_SLOT_X: f64 = 112.0;
const TIMING_SLOT_Y: f64 = 34.0;
const TIMING_SLOT_DEPTH: f64 = 8.0;
const TIMING_PITCH_X: f64 = 180.0;
const TIMER_FLAG_COUNT: usize = VIAL_COUNT;
const TIMING_INDEX_POST_D: f64 = 15.0;

const PROBE_CENTER: (f64, f64) = (320.0, 210.0);
const PROBE_PANEL_X: f64 = 420.0;
const PROBE_PANEL_Y: f64 = 240.0;
const PROBE_PANEL_Z: f64 = 42.0;
const PROBE_WELL_COUNT: usize = VIAL_COUNT + 2;
const PROBE_WELL_D: f64 = 8.0;
const PROBE_WELL_DEPTH: f64 = 34.0;
const PROBE_COLLAR_D: f64 = 20.0;
const PROBE_CHANNEL_D: f64 = 6.0;
const REFERENCE_WELL_D: f64 = 24.0;

const BARCODE_CENTER: (f64, f64) = (305.0, -300.0);
const BARCODE_PANEL_X: f64 = 430.0;
const BARCODE_PANEL_Y: f64 = 130.0;
const BARCODE_PANEL_Z: f64 = 18.0;
const BARCODE_SLOT_COUNT: usize = VIAL_COUNT + 2;
const BARCODE_SLOT_X: f64 = 64.0;
const BARCODE_SLOT_Y: f64 = 28.0;
const BARCODE_SLOT_DEPTH: f64 = 5.0;
const BARCODE_PITCH_X: f64 = 92.0;

const WITNESS_CENTER: (f64, f64) = (-360.0, -260.0);
const WITNESS_PANEL_X: f64 = 620.0;
const WITNESS_PANEL_Y: f64 = 170.0;
const WITNESS_PANEL_Z: f64 = 32.0;
const WITNESS_LANE_PITCH_X: f64 = 96.0;
const WITNESS_WELL_D: f64 = 20.0;
const WITNESS_WELL_DEPTH: f64 = 14.0;
const WITNESS_TRACK_X: f64 = 56.0;
const WITNESS_TRACK_Y: f64 = 18.0;
const WITNESS_TRACK_DEPTH: f64 = 6.0;

const TRANSFER_CENTER: (f64, f64) = (470.0, -80.0);
const TRANSFER_PANEL_X: f64 = 360.0;
const TRANSFER_PANEL_Y: f64 = 230.0;
const TRANSFER_PANEL_Z: f64 = 38.0;
const TRANSFER_PORTS: usize = 6;
const TRANSFER_PORT_D: f64 = 24.0;
const TRANSFER_COLLAR_D: f64 = 42.0;
const TRANSFER_PORT_PITCH_X: f64 = 76.0;
const CAP_PARK_WELLS: usize = TRANSFER_PORTS;
const TUBE_COMB_CHANNELS: usize = TRANSFER_PORTS;
const SEEDING_DOCKS: usize = 3;

const CONDENSATE_CENTER: (f64, f64) = (0.0, -405.0);
const CONDENSATE_PAN_X: f64 = 1310.0;
const CONDENSATE_PAN_Y: f64 = 44.0;
const CONDENSATE_PAN_Z: f64 = 18.0;
const GUTTER_CHANNELS: usize = 4;
const CONDENSATE_WITNESS_WELLS: usize = 6;
const CONDENSATE_WELL_D: f64 = 26.0;

const BRIDGE_CENTER: (f64, f64) = (0.0, 408.0);
const BRIDGE_SPAN_X: f64 = 1280.0;
const BRIDGE_POST_X: f64 = 32.0;
const BRIDGE_POST_Y: f64 = 46.0;
const BRIDGE_CLEARANCE_Z: f64 = 214.0;
const BRIDGE_BEAM_Z: f64 = 30.0;
const CAMERA_PODS: usize = 4;
const LIGHT_BARS: usize = 2;

const FRONT_ROBOT_CLEARANCE: f64 = 420.0;
const REAR_SERVICE_CLEARANCE: f64 = 260.0;
const LEFT_SERVICE_CLEARANCE: f64 = 260.0;
const RIGHT_TRANSFER_CLEARANCE: f64 = 300.0;
const TOP_ACCESS_CLEARANCE: f64 = 340.0;
const KEEP_OUT_GAUGE_Z: f64 = 6.0;

const TEXT_THICKNESS: f64 = 2.8;

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

        dx < (self.x + other.x) / 2.0 && dy < (self.y + other.y) / 2.0
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_containment_deck();
    export(OUTPUTS[0], &base);

    let nests = timed_vial_nest_ladder();
    export(OUTPUTS[1], &nests);

    let timing = stagger_timing_rail();
    export(OUTPUTS[2], &timing);

    let probes = temperature_probe_well_bank();
    export(OUTPUTS[3], &probes);

    let barcodes = barcode_custody_slots();
    export(OUTPUTS[4], &barcodes);

    let witnesses = thaw_start_end_witness_indicators();
    export(OUTPUTS[5], &witnesses);

    let transfer = sterile_transfer_staging();
    export(OUTPUTS[6], &transfer);

    let condensate = drain_condensate_containment();
    export(OUTPUTS[7], &condensate);

    let bridge = evidence_camera_bridge();
    export(OUTPUTS[8], &bridge);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[9], &keepouts);

    let labels = human_readable_labels();
    export(OUTPUTS[10], &labels);

    let assembly = base
        + nests
        + timing
        + probes
        + barcodes
        + witnesses
        + transfer
        + condensate
        + bridge
        + keepouts
        + labels;
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed cell-source vial thaw timing/stagger station:");
    println!(
        "  Deck and containment:       {STATION_X:.0}mm x {STATION_Y:.0}mm deck, {RIM_Z:.0}mm raised rim, {DRAIN_PORT_D:.0}mm drain port"
    );
    println!(
        "  Timed vial nests:           {VIAL_COUNT} closed vial nests in {VIAL_ROWS} rows x {VIAL_COLS} columns, staggered at {:?} minutes",
        STAGGER_MINUTES
    );
    println!(
        "  Timing reproducibility:     {TIMER_FLAG_COUNT} timer flag pockets, start/end witness wells per vial, and indexed timing rail"
    );
    println!(
        "  Temperature monitoring:     {PROBE_WELL_COUNT} probe/reference wells with {PROBE_WELL_D:.1}mm probe bores and cable strain relief"
    );
    println!(
        "  Identity/custody:           {BARCODE_SLOT_COUNT} barcode/custody slots plus raised CSG labels for vial, time, probe, transfer, and drain zones"
    );
    println!(
        "  Closed transfer staging:    {TRANSFER_PORTS} sterile connector ports, {CAP_PARK_WELLS} cap wells, {TUBE_COMB_CHANNELS} tube-comb lanes, {SEEDING_DOCKS} seeding handoff docks"
    );
    println!(
        "  Condensate management:      {GUTTER_CHANNELS} gutter channels and {CONDENSATE_WITNESS_WELLS} witness wells draining to the front pan"
    );
    println!(
        "  Robot/service keepouts:     front {FRONT_ROBOT_CLEARANCE:.0}mm, rear {REAR_SERVICE_CLEARANCE:.0}mm, left {LEFT_SERVICE_CLEARANCE:.0}mm, right {RIGHT_TRANSFER_CLEARANCE:.0}mm, top {TOP_ACCESS_CLEARANCE:.0}mm"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_design_constraints() {
    assert_eq!(VIAL_COUNT, VIAL_ROWS * VIAL_COLS);
    assert_eq!(STAGGER_MINUTES.len(), VIAL_COUNT);
    assert_eq!(REQUIRED_FEATURES.len(), 9);
    assert_eq!(OUTPUTS.len(), 12);
    assert!(STAGGER_MINUTES.windows(2).all(|pair| pair[1] > pair[0]));
    assert!(PROBE_WELL_COUNT >= VIAL_COUNT + 2);
    assert!(BARCODE_SLOT_COUNT >= VIAL_COUNT + 2);
    assert!(TRANSFER_PORTS >= VIAL_COUNT);
    assert!(CONDENSATE_WITNESS_WELLS >= VIAL_COUNT);
    assert!(BRIDGE_CLEARANCE_Z > DECK_Z + NEST_PANEL_Z + 70.0);
    assert!(TOP_ACCESS_CLEARANCE > BRIDGE_CLEARANCE_Z + BRIDGE_BEAM_Z);

    let rects = layout_rects();
    for rect in rects {
        assert!(
            rect.fits_inside_station(),
            "{} must fit inside station deck",
            rect.name
        );
    }

    for (i, first) in rects.iter().enumerate() {
        for second in rects.iter().skip(i + 1) {
            assert!(
                !first.overlaps(*second),
                "{} overlaps {}",
                first.name,
                second.name
            );
        }
    }

    for label in required_label_texts() {
        assert!(
            label.chars().all(supported_label_char),
            "label text contains unsupported glyph: {label}"
        );
    }
}

fn layout_rects() -> [Rect; 7] {
    [
        rect(
            "timed_vial_nest_ladder",
            NEST_CENTER,
            NEST_PANEL_X,
            NEST_PANEL_Y,
        ),
        rect(
            "stagger_timing_rail",
            TIMING_RAIL_CENTER,
            TIMING_RAIL_X,
            TIMING_RAIL_Y,
        ),
        rect(
            "temperature_probe_well_bank",
            PROBE_CENTER,
            PROBE_PANEL_X,
            PROBE_PANEL_Y,
        ),
        rect(
            "barcode_custody_slots",
            BARCODE_CENTER,
            BARCODE_PANEL_X,
            BARCODE_PANEL_Y,
        ),
        rect(
            "thaw_start_end_witness_indicators",
            WITNESS_CENTER,
            WITNESS_PANEL_X,
            WITNESS_PANEL_Y,
        ),
        rect(
            "sterile_transfer_staging",
            TRANSFER_CENTER,
            TRANSFER_PANEL_X,
            TRANSFER_PANEL_Y,
        ),
        rect(
            "drain_condensate_containment",
            CONDENSATE_CENTER,
            CONDENSATE_PAN_X,
            CONDENSATE_PAN_Y,
        ),
    ]
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn base_containment_deck() -> Part {
    let deck = centered_cube(
        format!("{OUTPUT_PREFIX}_base_deck"),
        STATION_X,
        STATION_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);

    let basin = centered_cube(
        format!("{OUTPUT_PREFIX}_shallow_condensate_basin_cut"),
        STATION_X - BASIN_MARGIN_X,
        STATION_Y - BASIN_MARGIN_Y,
        BASIN_DEPTH + 1.0,
    )
    .translate(0.0, -8.0, DECK_Z - BASIN_DEPTH / 2.0 + 0.5);

    let drain = centered_cylinder(
        format!("{OUTPUT_PREFIX}_front_drain_port_cut"),
        DRAIN_PORT_D / 2.0,
        RIM_W + 40.0,
        40,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 130.0,
        -STATION_Y / 2.0 + 16.0,
        DECK_Z - 4.0,
    );

    deck - basin - drain + containment_rim() + mount_bosses() + zone_floor_markers()
}

fn containment_rim() -> Part {
    let front = centered_cube(
        format!("{OUTPUT_PREFIX}_front_containment_rim"),
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        format!("{OUTPUT_PREFIX}_rear_containment_rim"),
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let left = centered_cube(
        format!("{OUTPUT_PREFIX}_left_containment_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube(
        format!("{OUTPUT_PREFIX}_right_containment_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);

    front + rear + left + right
}

fn mount_bosses() -> Part {
    let mut bosses = Part::empty(format!("{OUTPUT_PREFIX}_deck_mount_bosses"));
    for (index, (x, y)) in mount_positions().iter().enumerate() {
        let boss = centered_cylinder(
            format!("{OUTPUT_PREFIX}_deck_mount_boss_{index}"),
            MOUNT_BOSS_D / 2.0,
            10.0,
            40,
        )
        .translate(*x, *y, DECK_Z + 5.0);
        let hole = centered_cylinder(
            format!("{OUTPUT_PREFIX}_deck_mount_hole_{index}"),
            MOUNT_HOLE_D / 2.0,
            12.0,
            24,
        )
        .translate(*x, *y, DECK_Z + 5.0);
        bosses = bosses + (boss - hole);
    }
    bosses
}

fn zone_floor_markers() -> Part {
    let nests = centered_cube(
        format!("{OUTPUT_PREFIX}_vial_nest_zone_floor_marker"),
        NEST_PANEL_X + 40.0,
        NEST_PANEL_Y + 34.0,
        3.0,
    )
    .translate(NEST_CENTER.0, NEST_CENTER.1, DECK_Z + 1.5);
    let probe = centered_cube(
        format!("{OUTPUT_PREFIX}_probe_zone_floor_marker"),
        PROBE_PANEL_X + 34.0,
        PROBE_PANEL_Y + 32.0,
        3.0,
    )
    .translate(PROBE_CENTER.0, PROBE_CENTER.1, DECK_Z + 1.5);
    let transfer = centered_cube(
        format!("{OUTPUT_PREFIX}_transfer_zone_floor_marker"),
        TRANSFER_PANEL_X + 34.0,
        TRANSFER_PANEL_Y + 32.0,
        3.0,
    )
    .translate(TRANSFER_CENTER.0, TRANSFER_CENTER.1, DECK_Z + 1.5);
    let witness = centered_cube(
        format!("{OUTPUT_PREFIX}_witness_zone_floor_marker"),
        WITNESS_PANEL_X + 34.0,
        WITNESS_PANEL_Y + 30.0,
        3.0,
    )
    .translate(WITNESS_CENTER.0, WITNESS_CENTER.1, DECK_Z + 1.5);

    nests + probe + transfer + witness
}

fn timed_vial_nest_ladder() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_timed_vial_nest_plate"),
        NEST_PANEL_X,
        NEST_PANEL_Y,
        NEST_PANEL_Z,
    );

    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_timed_vial_nest_cuts"));
    for index in 0..VIAL_COUNT {
        let (x, y) = local_vial_xy(index);
        let vial_well = centered_cylinder(
            format!("{OUTPUT_PREFIX}_vial_{index}_closed_source_well"),
            VIAL_WELL_D / 2.0,
            VIAL_WELL_DEPTH + 2.0,
            56,
        )
        .translate(x, y, NEST_PANEL_Z / 2.0 - VIAL_WELL_DEPTH / 2.0 + 0.5);
        let anti_rotation_key = centered_cube(
            format!("{OUTPUT_PREFIX}_vial_{index}_anti_rotation_key_cut"),
            16.0,
            44.0,
            VIAL_WELL_DEPTH + 2.0,
        )
        .translate(
            x + VIAL_WELL_D / 2.0 - 3.0,
            y,
            NEST_PANEL_Z / 2.0 - VIAL_WELL_DEPTH / 2.0 + 0.5,
        );
        let token_slot = centered_cube(
            format!("{OUTPUT_PREFIX}_vial_{index}_timer_token_slot_cut"),
            TIMER_TOKEN_SLOT_X,
            TIMER_TOKEN_SLOT_Y,
            TIMER_TOKEN_SLOT_DEPTH + 1.0,
        )
        .translate(
            x,
            y + 58.0,
            NEST_PANEL_Z / 2.0 - TIMER_TOKEN_SLOT_DEPTH / 2.0 + 0.5,
        );
        cuts = cuts + vial_well + anti_rotation_key + token_slot;
    }

    let plate = (body - cuts).translate(NEST_CENTER.0, NEST_CENTER.1, DECK_Z + NEST_PANEL_Z / 2.0);

    plate + vial_retainer_collars() + stagger_lane_hard_stops() + thaw_bag_edge_clamps()
}

fn vial_retainer_collars() -> Part {
    let mut collars = Part::empty(format!("{OUTPUT_PREFIX}_timed_vial_retainer_collars"));
    for index in 0..VIAL_COUNT {
        let (x, y) = world_vial_xy(index);
        let outer = centered_cylinder(
            format!("{OUTPUT_PREFIX}_vial_{index}_raised_retainer_collar"),
            VIAL_COLLAR_D / 2.0,
            VIAL_COLLAR_Z,
            56,
        )
        .translate(x, y, DECK_Z + NEST_PANEL_Z + VIAL_COLLAR_Z / 2.0);
        let inner = centered_cylinder(
            format!("{OUTPUT_PREFIX}_vial_{index}_collar_inner_clearance"),
            (VIAL_WELL_D + 5.0) / 2.0,
            VIAL_COLLAR_Z + 2.0,
            56,
        )
        .translate(x, y, DECK_Z + NEST_PANEL_Z + VIAL_COLLAR_Z / 2.0);
        let clocking_tab = centered_cube(
            format!("{OUTPUT_PREFIX}_vial_{index}_clocking_tab_land"),
            18.0,
            8.0,
            9.0,
        )
        .translate(
            x + VIAL_COLLAR_D / 2.0 + 8.0,
            y,
            DECK_Z + NEST_PANEL_Z + 4.5,
        );
        collars = collars + (outer - inner) + clocking_tab;
    }
    collars
}

fn stagger_lane_hard_stops() -> Part {
    let mut stops = Part::empty(format!("{OUTPUT_PREFIX}_stagger_lane_hard_stops"));
    for index in 0..VIAL_COUNT {
        let (x, y) = world_vial_xy(index);
        let stop = centered_cube(
            format!(
                "{OUTPUT_PREFIX}_vial_{index}_stagger_hard_stop_{:02}_minute",
                STAGGER_MINUTES[index]
            ),
            72.0,
            10.0,
            18.0,
        )
        .translate(x, y - 62.0, DECK_Z + NEST_PANEL_Z + 9.0);
        let tether = centered_cylinder(
            format!("{OUTPUT_PREFIX}_vial_{index}_timer_token_tether_boss"),
            7.0,
            10.0,
            24,
        )
        .translate(x + 46.0, y + 58.0, DECK_Z + NEST_PANEL_Z + 5.0);
        stops = stops + stop + tether;
    }
    stops
}

fn thaw_bag_edge_clamps() -> Part {
    let mut clamps = Part::empty(format!("{OUTPUT_PREFIX}_closed_bag_edge_clamp_lips"));
    for row in 0..VIAL_ROWS {
        let y = NEST_CENTER.1 + centered_index(row, VIAL_ROWS, VIAL_PITCH_Y);
        let left = centered_cube(
            format!("{OUTPUT_PREFIX}_row_{row}_left_bag_edge_clamp"),
            26.0,
            122.0,
            22.0,
        )
        .translate(
            NEST_CENTER.0 - NEST_PANEL_X / 2.0 + 38.0,
            y,
            DECK_Z + NEST_PANEL_Z + 11.0,
        );
        let right = centered_cube(
            format!("{OUTPUT_PREFIX}_row_{row}_right_bag_edge_clamp"),
            26.0,
            122.0,
            22.0,
        )
        .translate(
            NEST_CENTER.0 + NEST_PANEL_X / 2.0 - 38.0,
            y,
            DECK_Z + NEST_PANEL_Z + 11.0,
        );
        clamps = clamps + left + right;
    }
    clamps
}

fn stagger_timing_rail() -> Part {
    let rail = centered_cube(
        format!("{OUTPUT_PREFIX}_stagger_timing_rail_body"),
        TIMING_RAIL_X,
        TIMING_RAIL_Y,
        TIMING_RAIL_Z,
    );

    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_stagger_timing_rail_slot_cuts"));
    for index in 0..VIAL_COUNT {
        let x = timing_slot_x(index);
        let slot = centered_cube(
            format!("{OUTPUT_PREFIX}_timer_flag_slot_{index}"),
            TIMING_SLOT_X,
            TIMING_SLOT_Y,
            TIMING_SLOT_DEPTH + 1.0,
        )
        .translate(x, 0.0, TIMING_RAIL_Z / 2.0 - TIMING_SLOT_DEPTH / 2.0 + 0.5);
        let pull = centered_cube(
            format!("{OUTPUT_PREFIX}_timer_flag_finger_pull_{index}"),
            26.0,
            TIMING_RAIL_Y + 4.0,
            TIMING_SLOT_DEPTH + 2.0,
        )
        .translate(x, 0.0, TIMING_RAIL_Z / 2.0 - TIMING_SLOT_DEPTH / 2.0 + 0.5);
        cuts = cuts + slot + pull;
    }

    let rail = (rail - cuts).translate(
        TIMING_RAIL_CENTER.0,
        TIMING_RAIL_CENTER.1,
        DECK_Z + TIMING_RAIL_Z / 2.0,
    );

    rail + timer_index_posts() + start_end_latch_row()
}

fn timer_index_posts() -> Part {
    let mut posts = Part::empty(format!("{OUTPUT_PREFIX}_timer_index_posts"));
    for index in 0..VIAL_COUNT {
        let x = TIMING_RAIL_CENTER.0 + timing_slot_x(index) - TIMING_SLOT_X / 2.0 - 18.0;
        let post_height = 16.0 + index as f64 * 2.2;
        let post = centered_cylinder(
            format!("{OUTPUT_PREFIX}_timer_index_post_{index}"),
            TIMING_INDEX_POST_D / 2.0,
            post_height,
            24,
        )
        .translate(
            x,
            TIMING_RAIL_CENTER.1,
            DECK_Z + TIMING_RAIL_Z + post_height / 2.0,
        );
        let detent = centered_cube(
            format!("{OUTPUT_PREFIX}_timer_detent_tooth_{index}"),
            18.0,
            10.0,
            10.0,
        )
        .translate(
            x + 24.0,
            TIMING_RAIL_CENTER.1 - TIMING_RAIL_Y / 2.0 - 5.0,
            DECK_Z + TIMING_RAIL_Z + 5.0,
        );
        posts = posts + post + detent;
    }
    posts
}

fn start_end_latch_row() -> Part {
    let start_bar = centered_cube(
        format!("{OUTPUT_PREFIX}_timing_rail_start_latch_bar"),
        TIMING_RAIL_X - 78.0,
        8.0,
        18.0,
    )
    .translate(
        TIMING_RAIL_CENTER.0,
        TIMING_RAIL_CENTER.1 - TIMING_RAIL_Y / 2.0 - 9.0,
        DECK_Z + TIMING_RAIL_Z + 9.0,
    );
    let end_bar = centered_cube(
        format!("{OUTPUT_PREFIX}_timing_rail_end_latch_bar"),
        TIMING_RAIL_X - 78.0,
        8.0,
        18.0,
    )
    .translate(
        TIMING_RAIL_CENTER.0,
        TIMING_RAIL_CENTER.1 + TIMING_RAIL_Y / 2.0 + 9.0,
        DECK_Z + TIMING_RAIL_Z + 9.0,
    );

    start_bar + end_bar
}

fn temperature_probe_well_bank() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_temperature_probe_well_bank_body"),
        PROBE_PANEL_X,
        PROBE_PANEL_Y,
        PROBE_PANEL_Z,
    );

    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_temperature_probe_well_cuts"));
    for index in 0..PROBE_WELL_COUNT {
        let (x, y) = local_probe_xy(index);
        let diameter = if index >= VIAL_COUNT {
            REFERENCE_WELL_D
        } else {
            PROBE_WELL_D
        };
        cuts = cuts
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_probe_well_{index}_cut"),
                diameter / 2.0,
                PROBE_WELL_DEPTH + 2.0,
                36,
            )
            .translate(x, y, PROBE_PANEL_Z / 2.0 - PROBE_WELL_DEPTH / 2.0 + 0.5);
    }

    let lead_channel = centered_cylinder(
        format!("{OUTPUT_PREFIX}_probe_lead_trunk_channel_cut"),
        PROBE_CHANNEL_D / 2.0,
        PROBE_PANEL_X - 70.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -78.0, PROBE_PANEL_Z / 2.0 - 15.0);

    let panel = (body - cuts - lead_channel).translate(
        PROBE_CENTER.0,
        PROBE_CENTER.1,
        DECK_Z + PROBE_PANEL_Z / 2.0,
    );

    panel + probe_collars() + probe_strain_relief_comb()
}

fn probe_collars() -> Part {
    let mut collars = Part::empty(format!("{OUTPUT_PREFIX}_probe_well_collars"));
    for index in 0..PROBE_WELL_COUNT {
        let (x, y) = world_probe_xy(index);
        let diameter = if index >= VIAL_COUNT {
            REFERENCE_WELL_D + 10.0
        } else {
            PROBE_COLLAR_D
        };
        let outer = centered_cylinder(
            format!("{OUTPUT_PREFIX}_probe_well_{index}_raised_collar"),
            diameter / 2.0,
            8.0,
            36,
        )
        .translate(x, y, DECK_Z + PROBE_PANEL_Z + 4.0);
        let inner = centered_cylinder(
            format!("{OUTPUT_PREFIX}_probe_well_{index}_collar_clearance"),
            if index >= VIAL_COUNT {
                REFERENCE_WELL_D / 2.0
            } else {
                PROBE_WELL_D / 2.0
            },
            10.0,
            32,
        )
        .translate(x, y, DECK_Z + PROBE_PANEL_Z + 4.0);
        collars = collars + (outer - inner);
    }
    collars
}

fn probe_strain_relief_comb() -> Part {
    let comb = centered_cube(
        format!("{OUTPUT_PREFIX}_probe_strain_relief_comb"),
        PROBE_PANEL_X - 66.0,
        28.0,
        22.0,
    )
    .translate(
        PROBE_CENTER.0,
        PROBE_CENTER.1 - PROBE_PANEL_Y / 2.0 - 20.0,
        DECK_Z + PROBE_PANEL_Z - 4.0,
    );

    let mut channels = Part::empty(format!("{OUTPUT_PREFIX}_probe_strain_relief_channel_cuts"));
    for index in 0..PROBE_WELL_COUNT {
        channels = channels
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_probe_strain_relief_channel_{index}"),
                3.2,
                PROBE_PANEL_Y / 2.0,
                18,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                PROBE_CENTER.0 + centered_index(index, PROBE_WELL_COUNT, 42.0),
                PROBE_CENTER.1 - PROBE_PANEL_Y / 2.0 - 20.0,
                DECK_Z + PROBE_PANEL_Z - 4.0,
            );
    }

    comb - channels
}

fn barcode_custody_slots() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_barcode_custody_panel"),
        BARCODE_PANEL_X,
        BARCODE_PANEL_Y,
        BARCODE_PANEL_Z,
    );

    let mut slot_cuts = Part::empty(format!("{OUTPUT_PREFIX}_barcode_slot_cuts"));
    for index in 0..BARCODE_SLOT_COUNT {
        let row = index / 4;
        let col = index % 4;
        let x = centered_index(col, 4, BARCODE_PITCH_X);
        let y = if row == 0 { 28.0 } else { -30.0 };
        slot_cuts = slot_cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_barcode_custody_slot_{index}_cut"),
                BARCODE_SLOT_X,
                BARCODE_SLOT_Y,
                BARCODE_SLOT_DEPTH + 1.0,
            )
            .translate(x, y, BARCODE_PANEL_Z / 2.0 - BARCODE_SLOT_DEPTH / 2.0 + 0.5);
    }

    let panel = (body - slot_cuts).translate(
        BARCODE_CENTER.0,
        BARCODE_CENTER.1,
        DECK_Z + BARCODE_PANEL_Z / 2.0,
    );

    panel + barcode_slot_clips() + custody_seal_wells()
}

fn barcode_slot_clips() -> Part {
    let mut clips = Part::empty(format!("{OUTPUT_PREFIX}_barcode_slot_retaining_clips"));
    for index in 0..BARCODE_SLOT_COUNT {
        let row = index / 4;
        let col = index % 4;
        let x = BARCODE_CENTER.0 + centered_index(col, 4, BARCODE_PITCH_X);
        let y = BARCODE_CENTER.1 + if row == 0 { 28.0 } else { -30.0 };
        let front = centered_cube(
            format!("{OUTPUT_PREFIX}_barcode_slot_{index}_front_clip"),
            BARCODE_SLOT_X + 10.0,
            5.0,
            8.0,
        )
        .translate(
            x,
            y - BARCODE_SLOT_Y / 2.0 - 3.0,
            DECK_Z + BARCODE_PANEL_Z + 4.0,
        );
        let rear = centered_cube(
            format!("{OUTPUT_PREFIX}_barcode_slot_{index}_rear_clip"),
            BARCODE_SLOT_X + 10.0,
            5.0,
            8.0,
        )
        .translate(
            x,
            y + BARCODE_SLOT_Y / 2.0 + 3.0,
            DECK_Z + BARCODE_PANEL_Z + 4.0,
        );
        clips = clips + front + rear;
    }
    clips
}

fn custody_seal_wells() -> Part {
    let tray = centered_cube(
        format!("{OUTPUT_PREFIX}_custody_seal_token_tray"),
        132.0,
        34.0,
        12.0,
    )
    .translate(
        BARCODE_CENTER.0 + BARCODE_PANEL_X / 2.0 - 86.0,
        BARCODE_CENTER.1 - BARCODE_PANEL_Y / 2.0 - 22.0,
        DECK_Z + 10.0,
    );

    let mut wells = Part::empty(format!("{OUTPUT_PREFIX}_custody_seal_well_cuts"));
    for index in 0..4 {
        wells = wells
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_custody_seal_well_{index}"),
                9.0,
                14.0,
                28,
            )
            .translate(
                BARCODE_CENTER.0 + BARCODE_PANEL_X / 2.0 - 132.0 + index as f64 * 30.0,
                BARCODE_CENTER.1 - BARCODE_PANEL_Y / 2.0 - 22.0,
                DECK_Z + 12.0,
            );
    }

    tray - wells
}

fn thaw_start_end_witness_indicators() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_start_end_witness_panel"),
        WITNESS_PANEL_X,
        WITNESS_PANEL_Y,
        WITNESS_PANEL_Z,
    );

    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_start_end_witness_cuts"));
    for index in 0..VIAL_COUNT {
        let x = centered_index(index, VIAL_COUNT, WITNESS_LANE_PITCH_X);
        let start_well = centered_cylinder(
            format!("{OUTPUT_PREFIX}_vial_{index}_thaw_start_witness_well"),
            WITNESS_WELL_D / 2.0,
            WITNESS_WELL_DEPTH + 1.0,
            32,
        )
        .translate(
            x,
            42.0,
            WITNESS_PANEL_Z / 2.0 - WITNESS_WELL_DEPTH / 2.0 + 0.5,
        );
        let end_well = centered_cylinder(
            format!("{OUTPUT_PREFIX}_vial_{index}_thaw_end_witness_well"),
            WITNESS_WELL_D / 2.0,
            WITNESS_WELL_DEPTH + 1.0,
            32,
        )
        .translate(
            x,
            -42.0,
            WITNESS_PANEL_Z / 2.0 - WITNESS_WELL_DEPTH / 2.0 + 0.5,
        );
        let slider_track = centered_cube(
            format!("{OUTPUT_PREFIX}_vial_{index}_witness_slider_track"),
            WITNESS_TRACK_X,
            WITNESS_TRACK_Y,
            WITNESS_TRACK_DEPTH + 1.0,
        )
        .translate(
            x,
            0.0,
            WITNESS_PANEL_Z / 2.0 - WITNESS_TRACK_DEPTH / 2.0 + 0.5,
        );
        cuts = cuts + start_well + end_well + slider_track;
    }

    let panel = (body - cuts).translate(
        WITNESS_CENTER.0,
        WITNESS_CENTER.1,
        DECK_Z + WITNESS_PANEL_Z / 2.0,
    );

    panel + witness_flag_posts() + witness_lane_dividers()
}

fn witness_flag_posts() -> Part {
    let mut posts = Part::empty(format!("{OUTPUT_PREFIX}_start_end_witness_flag_posts"));
    for index in 0..VIAL_COUNT {
        let x = WITNESS_CENTER.0 + centered_index(index, VIAL_COUNT, WITNESS_LANE_PITCH_X);
        let start_post = centered_cylinder(
            format!("{OUTPUT_PREFIX}_vial_{index}_start_flag_post"),
            5.0,
            24.0,
            20,
        )
        .translate(
            x - 22.0,
            WITNESS_CENTER.1 + 42.0,
            DECK_Z + WITNESS_PANEL_Z + 12.0,
        );
        let end_post = centered_cylinder(
            format!("{OUTPUT_PREFIX}_vial_{index}_end_flag_post"),
            5.0,
            24.0,
            20,
        )
        .translate(
            x + 22.0,
            WITNESS_CENTER.1 - 42.0,
            DECK_Z + WITNESS_PANEL_Z + 12.0,
        );
        let tamper_bridge = centered_cube(
            format!("{OUTPUT_PREFIX}_vial_{index}_witness_tamper_bridge"),
            40.0,
            8.0,
            14.0,
        )
        .translate(x, WITNESS_CENTER.1, DECK_Z + WITNESS_PANEL_Z + 7.0);
        posts = posts + start_post + end_post + tamper_bridge;
    }
    posts
}

fn witness_lane_dividers() -> Part {
    let mut dividers = Part::empty(format!("{OUTPUT_PREFIX}_witness_lane_dividers"));
    for index in 1..VIAL_COUNT {
        let x = WITNESS_CENTER.0 + centered_index(index, VIAL_COUNT, WITNESS_LANE_PITCH_X)
            - WITNESS_LANE_PITCH_X / 2.0;
        dividers = dividers
            + centered_cube(
                format!("{OUTPUT_PREFIX}_witness_lane_divider_{index}"),
                6.0,
                WITNESS_PANEL_Y - 30.0,
                18.0,
            )
            .translate(x, WITNESS_CENTER.1, DECK_Z + WITNESS_PANEL_Z + 9.0);
    }
    dividers
}

fn sterile_transfer_staging() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_sterile_transfer_staging_panel"),
        TRANSFER_PANEL_X,
        TRANSFER_PANEL_Y,
        TRANSFER_PANEL_Z,
    );

    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_sterile_transfer_port_cuts"));
    for index in 0..TRANSFER_PORTS {
        let x = centered_index(index, TRANSFER_PORTS, TRANSFER_PORT_PITCH_X);
        cuts = cuts
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_sterile_transfer_port_{index}_cut"),
                TRANSFER_PORT_D / 2.0,
                TRANSFER_PANEL_Z + 2.0,
                40,
            )
            .translate(x, 48.0, 0.0);
        cuts = cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_transfer_tube_channel_{index}_cut"),
                12.0,
                86.0,
                12.0,
            )
            .translate(x, -28.0, TRANSFER_PANEL_Z / 2.0 - 5.0);
    }

    let panel = (body - cuts).translate(
        TRANSFER_CENTER.0,
        TRANSFER_CENTER.1,
        DECK_Z + TRANSFER_PANEL_Z / 2.0,
    );

    panel
        + transfer_port_collars()
        + cap_parking_wells()
        + seeding_handoff_docks()
        + tube_comb_gate()
}

fn transfer_port_collars() -> Part {
    let mut collars = Part::empty(format!("{OUTPUT_PREFIX}_sterile_transfer_port_collars"));
    for index in 0..TRANSFER_PORTS {
        let x = TRANSFER_CENTER.0 + centered_index(index, TRANSFER_PORTS, TRANSFER_PORT_PITCH_X);
        let y = TRANSFER_CENTER.1 + 48.0;
        let outer = centered_cylinder(
            format!("{OUTPUT_PREFIX}_sterile_transfer_port_{index}_outer_collar"),
            TRANSFER_COLLAR_D / 2.0,
            12.0,
            44,
        )
        .translate(x, y, DECK_Z + TRANSFER_PANEL_Z + 6.0);
        let inner = centered_cylinder(
            format!("{OUTPUT_PREFIX}_sterile_transfer_port_{index}_inner_clearance"),
            TRANSFER_PORT_D / 2.0,
            14.0,
            40,
        )
        .translate(x, y, DECK_Z + TRANSFER_PANEL_Z + 6.0);
        collars = collars + (outer - inner);
    }
    collars
}

fn cap_parking_wells() -> Part {
    let tray = centered_cube(
        format!("{OUTPUT_PREFIX}_sterile_cap_parking_tray"),
        TRANSFER_PANEL_X - 64.0,
        40.0,
        16.0,
    )
    .translate(
        TRANSFER_CENTER.0,
        TRANSFER_CENTER.1 - TRANSFER_PANEL_Y / 2.0 - 26.0,
        DECK_Z + 16.0,
    );

    let mut wells = Part::empty(format!("{OUTPUT_PREFIX}_sterile_cap_parking_well_cuts"));
    for index in 0..CAP_PARK_WELLS {
        wells = wells
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_sterile_cap_parking_well_{index}"),
                12.0,
                18.0,
                32,
            )
            .translate(
                TRANSFER_CENTER.0 + centered_index(index, CAP_PARK_WELLS, 42.0),
                TRANSFER_CENTER.1 - TRANSFER_PANEL_Y / 2.0 - 26.0,
                DECK_Z + 18.0,
            );
    }

    tray - wells
}

fn seeding_handoff_docks() -> Part {
    let mut docks = Part::empty(format!("{OUTPUT_PREFIX}_automated_seeding_handoff_docks"));
    for index in 0..SEEDING_DOCKS {
        let x = TRANSFER_CENTER.0 + centered_index(index, SEEDING_DOCKS, 94.0);
        let dock = centered_cube(
            format!("{OUTPUT_PREFIX}_seeding_handoff_dock_{index}"),
            72.0,
            42.0,
            28.0,
        )
        .translate(
            x,
            TRANSFER_CENTER.1 - 34.0,
            DECK_Z + TRANSFER_PANEL_Z + 14.0,
        );
        let saddle = centered_cylinder(
            format!("{OUTPUT_PREFIX}_seeding_handoff_dock_{index}_saddle_cut"),
            11.0,
            78.0,
            32,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(
            x,
            TRANSFER_CENTER.1 - 34.0,
            DECK_Z + TRANSFER_PANEL_Z + 16.0,
        );
        docks = docks + (dock - saddle);
    }
    docks
}

fn tube_comb_gate() -> Part {
    let comb = centered_cube(
        format!("{OUTPUT_PREFIX}_sterile_transfer_tube_comb_gate"),
        TRANSFER_PANEL_X - 72.0,
        26.0,
        24.0,
    )
    .translate(
        TRANSFER_CENTER.0,
        TRANSFER_CENTER.1 + TRANSFER_PANEL_Y / 2.0 + 20.0,
        DECK_Z + TRANSFER_PANEL_Z - 2.0,
    );

    let mut channels = Part::empty(format!("{OUTPUT_PREFIX}_sterile_transfer_tube_comb_cuts"));
    for index in 0..TUBE_COMB_CHANNELS {
        channels = channels
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_tube_comb_channel_{index}"),
                5.0,
                34.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                TRANSFER_CENTER.0 + centered_index(index, TUBE_COMB_CHANNELS, 42.0),
                TRANSFER_CENTER.1 + TRANSFER_PANEL_Y / 2.0 + 20.0,
                DECK_Z + TRANSFER_PANEL_Z - 2.0,
            );
    }

    comb - channels
}

fn drain_condensate_containment() -> Part {
    let pan = centered_cube(
        format!("{OUTPUT_PREFIX}_front_drain_condensate_pan"),
        CONDENSATE_PAN_X,
        CONDENSATE_PAN_Y,
        CONDENSATE_PAN_Z,
    );

    let mut channel_cuts = Part::empty(format!("{OUTPUT_PREFIX}_front_condensate_channel_cuts"));
    for index in 0..GUTTER_CHANNELS {
        channel_cuts = channel_cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_condensate_gutter_channel_{index}"),
                CONDENSATE_PAN_X - 120.0,
                8.0,
                8.0,
            )
            .translate(
                0.0,
                centered_index(index, GUTTER_CHANNELS, 14.0),
                CONDENSATE_PAN_Z / 2.0 - 3.0,
            );
    }

    let drain_cup_cut = centered_cylinder(
        format!("{OUTPUT_PREFIX}_removable_drain_cup_socket_cut"),
        24.0,
        CONDENSATE_PAN_Z + 2.0,
        40,
    )
    .translate(CONDENSATE_PAN_X / 2.0 - 118.0, 0.0, 0.0);

    let pan = (pan - channel_cuts - drain_cup_cut).translate(
        CONDENSATE_CENTER.0,
        CONDENSATE_CENTER.1,
        DECK_Z + CONDENSATE_PAN_Z / 2.0,
    );

    pan + condensate_witness_wells() + side_gutter_spines() + drain_cup_retainer()
}

fn condensate_witness_wells() -> Part {
    let tray = centered_cube(
        format!("{OUTPUT_PREFIX}_condensate_witness_well_tray"),
        360.0,
        46.0,
        16.0,
    )
    .translate(
        CONDENSATE_CENTER.0 - CONDENSATE_PAN_X / 2.0 + 238.0,
        CONDENSATE_CENTER.1 + 54.0,
        DECK_Z + 16.0,
    );

    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_condensate_witness_well_cuts"));
    for index in 0..CONDENSATE_WITNESS_WELLS {
        cuts = cuts
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_condensate_witness_well_{index}"),
                CONDENSATE_WELL_D / 2.0,
                18.0,
                32,
            )
            .translate(
                CONDENSATE_CENTER.0 - CONDENSATE_PAN_X / 2.0 + 106.0 + index as f64 * 48.0,
                CONDENSATE_CENTER.1 + 54.0,
                DECK_Z + 18.0,
            );
    }

    tray - cuts
}

fn side_gutter_spines() -> Part {
    let left = centered_cube(
        format!("{OUTPUT_PREFIX}_left_panel_condensate_gutter_spine"),
        24.0,
        STATION_Y - 210.0,
        16.0,
    )
    .translate(-STATION_X / 2.0 + 62.0, -8.0, DECK_Z + 8.0);
    let right = centered_cube(
        format!("{OUTPUT_PREFIX}_right_panel_condensate_gutter_spine"),
        24.0,
        STATION_Y - 210.0,
        16.0,
    )
    .translate(STATION_X / 2.0 - 62.0, -8.0, DECK_Z + 8.0);
    let rear = centered_cube(
        format!("{OUTPUT_PREFIX}_rear_panel_condensate_gutter_spine"),
        STATION_X - 180.0,
        24.0,
        16.0,
    )
    .translate(0.0, STATION_Y / 2.0 - 72.0, DECK_Z + 8.0);

    left + right + rear
}

fn drain_cup_retainer() -> Part {
    let outer = centered_cylinder(
        format!("{OUTPUT_PREFIX}_front_drain_cup_retainer_ring"),
        36.0,
        12.0,
        48,
    )
    .translate(
        CONDENSATE_CENTER.0 + CONDENSATE_PAN_X / 2.0 - 118.0,
        CONDENSATE_CENTER.1,
        DECK_Z + CONDENSATE_PAN_Z + 6.0,
    );
    let inner = centered_cylinder(
        format!("{OUTPUT_PREFIX}_front_drain_cup_retainer_clearance"),
        24.0,
        14.0,
        48,
    )
    .translate(
        CONDENSATE_CENTER.0 + CONDENSATE_PAN_X / 2.0 - 118.0,
        CONDENSATE_CENTER.1,
        DECK_Z + CONDENSATE_PAN_Z + 6.0,
    );

    outer - inner
}

fn evidence_camera_bridge() -> Part {
    let post_z = BRIDGE_CLEARANCE_Z;
    let left_post = centered_cube(
        format!("{OUTPUT_PREFIX}_evidence_bridge_left_post"),
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        post_z,
    )
    .translate(
        BRIDGE_CENTER.0 - BRIDGE_SPAN_X / 2.0,
        BRIDGE_CENTER.1,
        DECK_Z + post_z / 2.0,
    );
    let right_post = centered_cube(
        format!("{OUTPUT_PREFIX}_evidence_bridge_right_post"),
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        post_z,
    )
    .translate(
        BRIDGE_CENTER.0 + BRIDGE_SPAN_X / 2.0,
        BRIDGE_CENTER.1,
        DECK_Z + post_z / 2.0,
    );
    let beam = centered_cube(
        format!("{OUTPUT_PREFIX}_evidence_bridge_camera_beam"),
        BRIDGE_SPAN_X + BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(
        BRIDGE_CENTER.0,
        BRIDGE_CENTER.1,
        DECK_Z + post_z + BRIDGE_BEAM_Z / 2.0,
    );

    left_post + right_post + beam + camera_pods() + evidence_light_bars()
}

fn camera_pods() -> Part {
    let mut pods = Part::empty(format!("{OUTPUT_PREFIX}_evidence_camera_pods"));
    for index in 0..CAMERA_PODS {
        let pod = centered_cube(
            format!("{OUTPUT_PREFIX}_evidence_camera_pod_{index}"),
            54.0,
            34.0,
            24.0,
        )
        .translate(
            BRIDGE_CENTER.0 + centered_index(index, CAMERA_PODS, 290.0),
            BRIDGE_CENTER.1 - BRIDGE_POST_Y / 2.0 - 18.0,
            DECK_Z + BRIDGE_CLEARANCE_Z - 12.0,
        );
        let lens = centered_cylinder(
            format!("{OUTPUT_PREFIX}_evidence_camera_pod_{index}_lens_cut"),
            8.0,
            36.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            BRIDGE_CENTER.0 + centered_index(index, CAMERA_PODS, 290.0),
            BRIDGE_CENTER.1 - BRIDGE_POST_Y / 2.0 - 18.0,
            DECK_Z + BRIDGE_CLEARANCE_Z - 12.0,
        );
        pods = pods + (pod - lens);
    }
    pods
}

fn evidence_light_bars() -> Part {
    let mut bars = Part::empty(format!("{OUTPUT_PREFIX}_evidence_light_bars"));
    for index in 0..LIGHT_BARS {
        let y = BRIDGE_CENTER.1 + if index == 0 { -36.0 } else { 36.0 };
        bars = bars
            + centered_cube(
                format!("{OUTPUT_PREFIX}_evidence_light_bar_{index}"),
                BRIDGE_SPAN_X - 150.0,
                12.0,
                12.0,
            )
            .translate(BRIDGE_CENTER.0, y, DECK_Z + BRIDGE_CLEARANCE_Z - 24.0);
    }
    bars
}

fn robot_service_keepouts() -> Part {
    let front = centered_cube(
        format!("{OUTPUT_PREFIX}_front_robot_approach_keepout_gauge"),
        STATION_X - 180.0,
        FRONT_ROBOT_CLEARANCE,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 - FRONT_ROBOT_CLEARANCE / 2.0,
        KEEP_OUT_GAUGE_Z / 2.0,
    );
    let rear = centered_cube(
        format!("{OUTPUT_PREFIX}_rear_service_keepout_gauge"),
        STATION_X - 180.0,
        REAR_SERVICE_CLEARANCE,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 + REAR_SERVICE_CLEARANCE / 2.0,
        KEEP_OUT_GAUGE_Z / 2.0,
    );
    let left = centered_cube(
        format!("{OUTPUT_PREFIX}_left_probe_service_keepout_gauge"),
        LEFT_SERVICE_CLEARANCE,
        STATION_Y - 170.0,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        -STATION_X / 2.0 - LEFT_SERVICE_CLEARANCE / 2.0,
        0.0,
        KEEP_OUT_GAUGE_Z / 2.0,
    );
    let right = centered_cube(
        format!("{OUTPUT_PREFIX}_right_transfer_service_keepout_gauge"),
        RIGHT_TRANSFER_CLEARANCE,
        STATION_Y - 170.0,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        STATION_X / 2.0 + RIGHT_TRANSFER_CLEARANCE / 2.0,
        0.0,
        KEEP_OUT_GAUGE_Z / 2.0,
    );
    let top = centered_cube(
        format!("{OUTPUT_PREFIX}_top_lid_and_probe_access_clearance_gauge"),
        STATION_X - 220.0,
        STATION_Y - 210.0,
        4.0,
    )
    .translate(0.0, 0.0, DECK_Z + TOP_ACCESS_CLEARANCE);

    front + rear + left + right + top
}

fn human_readable_labels() -> Part {
    let mut labels = Part::empty(format!("{OUTPUT_PREFIX}_raised_human_readable_labels"));

    labels = labels
        + raised_label(
            "title",
            "VIAL THAW STAGGER",
            0.0,
            390.0,
            DECK_Z + TIMING_RAIL_Z + 5.0,
            5.0,
        )
        + raised_label(
            "nest_panel",
            "TIMED VIAL NESTS",
            NEST_CENTER.0,
            NEST_CENTER.1 + NEST_PANEL_Y / 2.0 - 28.0,
            DECK_Z + NEST_PANEL_Z + 5.0,
            4.4,
        )
        + raised_label(
            "probe_panel",
            "TEMP PROBES",
            PROBE_CENTER.0,
            PROBE_CENTER.1 + PROBE_PANEL_Y / 2.0 - 28.0,
            DECK_Z + PROBE_PANEL_Z + 5.0,
            4.1,
        )
        + raised_label(
            "barcode_panel",
            "BARCODES",
            BARCODE_CENTER.0 - 98.0,
            BARCODE_CENTER.1 + BARCODE_PANEL_Y / 2.0 - 22.0,
            DECK_Z + BARCODE_PANEL_Z + 5.0,
            4.0,
        )
        + raised_label(
            "start_witness",
            "START",
            WITNESS_CENTER.0 - WITNESS_PANEL_X / 2.0 + 66.0,
            WITNESS_CENTER.1 + 42.0,
            DECK_Z + WITNESS_PANEL_Z + 5.0,
            3.8,
        )
        + raised_label(
            "end_witness",
            "END",
            WITNESS_CENTER.0 - WITNESS_PANEL_X / 2.0 + 54.0,
            WITNESS_CENTER.1 - 42.0,
            DECK_Z + WITNESS_PANEL_Z + 5.0,
            3.8,
        )
        + raised_label(
            "transfer_panel",
            "STERILE TRANSFER",
            TRANSFER_CENTER.0,
            TRANSFER_CENTER.1 - TRANSFER_PANEL_Y / 2.0 + 24.0,
            DECK_Z + TRANSFER_PANEL_Z + 5.0,
            3.6,
        )
        + raised_label(
            "drain_pan",
            "DRAIN",
            CONDENSATE_CENTER.0 + CONDENSATE_PAN_X / 2.0 - 118.0,
            CONDENSATE_CENTER.1,
            DECK_Z + CONDENSATE_PAN_Z + 6.0,
            3.6,
        );

    for index in 0..VIAL_COUNT {
        let (x, y) = world_vial_xy(index);
        labels = labels
            + raised_label(
                format!("vial_{}_time_label", index),
                time_label(index),
                x,
                y + 92.0,
                DECK_Z + NEST_PANEL_Z + 5.0,
                3.5,
            );
    }

    labels
}

fn raised_label(
    name: impl Into<String>,
    text: &str,
    x: f64,
    y: f64,
    base_z: f64,
    cell: f64,
) -> Part {
    block_text(format!("{OUTPUT_PREFIX}_label_{}", name.into()), text, cell).translate(
        x,
        y,
        base_z + TEXT_THICKNESS / 2.0,
    )
}

fn block_text(name: impl Into<String>, text: &str, cell: f64) -> Part {
    let name = name.into();
    let chars: Vec<char> = text.chars().collect();
    let char_count = chars.len();
    let total_width = if char_count == 0 {
        0.0
    } else {
        ((char_count as f64 * 5.0) + (char_count.saturating_sub(1) as f64)) * cell
    };
    let mut part = Part::empty(name.clone());

    for (char_index, ch) in chars.into_iter().enumerate() {
        let glyph = glyph_rows(ch);
        let x0 = -total_width / 2.0 + char_index as f64 * 6.0 * cell;
        for (row, row_bits) in glyph.iter().enumerate() {
            for (col, bit) in row_bits.chars().enumerate() {
                if bit == '1' {
                    let x = x0 + col as f64 * cell + cell / 2.0;
                    let y = (3.0 - row as f64) * cell;
                    part = part
                        + centered_cube(
                            format!("{name}_glyph_{char_index}_{row}_{col}"),
                            cell * 0.82,
                            cell * 0.82,
                            TEXT_THICKNESS,
                        )
                        .translate(x, y, 0.0);
                }
            }
        }
    }

    part
}

fn required_label_texts() -> [&'static str; 14] {
    [
        "VIAL THAW STAGGER",
        "TIMED VIAL NESTS",
        "TEMP PROBES",
        "BARCODES",
        "START",
        "END",
        "STERILE TRANSFER",
        "DRAIN",
        "T+00",
        "T+03",
        "T+06",
        "T+09",
        "T+12",
        "T+15",
    ]
}

fn supported_label_char(ch: char) -> bool {
    matches!(ch, 'A'..='Z' | '0'..='9' | ' ' | '+' | '-')
}

fn glyph_rows(ch: char) -> [&'static str; 7] {
    match ch {
        'A' => [
            "01110", "10001", "10001", "11111", "10001", "10001", "10001",
        ],
        'B' => [
            "11110", "10001", "10001", "11110", "10001", "10001", "11110",
        ],
        'C' => [
            "01111", "10000", "10000", "10000", "10000", "10000", "01111",
        ],
        'D' => [
            "11110", "10001", "10001", "10001", "10001", "10001", "11110",
        ],
        'E' => [
            "11111", "10000", "10000", "11110", "10000", "10000", "11111",
        ],
        'F' => [
            "11111", "10000", "10000", "11110", "10000", "10000", "10000",
        ],
        'G' => [
            "01111", "10000", "10000", "10111", "10001", "10001", "01111",
        ],
        'H' => [
            "10001", "10001", "10001", "11111", "10001", "10001", "10001",
        ],
        'I' => [
            "11111", "00100", "00100", "00100", "00100", "00100", "11111",
        ],
        'J' => [
            "00111", "00010", "00010", "00010", "00010", "10010", "01100",
        ],
        'K' => [
            "10001", "10010", "10100", "11000", "10100", "10010", "10001",
        ],
        'L' => [
            "10000", "10000", "10000", "10000", "10000", "10000", "11111",
        ],
        'M' => [
            "10001", "11011", "10101", "10101", "10001", "10001", "10001",
        ],
        'N' => [
            "10001", "11001", "10101", "10011", "10001", "10001", "10001",
        ],
        'O' => [
            "01110", "10001", "10001", "10001", "10001", "10001", "01110",
        ],
        'P' => [
            "11110", "10001", "10001", "11110", "10000", "10000", "10000",
        ],
        'Q' => [
            "01110", "10001", "10001", "10001", "10101", "10010", "01101",
        ],
        'R' => [
            "11110", "10001", "10001", "11110", "10100", "10010", "10001",
        ],
        'S' => [
            "01111", "10000", "10000", "01110", "00001", "00001", "11110",
        ],
        'T' => [
            "11111", "00100", "00100", "00100", "00100", "00100", "00100",
        ],
        'U' => [
            "10001", "10001", "10001", "10001", "10001", "10001", "01110",
        ],
        'V' => [
            "10001", "10001", "10001", "10001", "10001", "01010", "00100",
        ],
        'W' => [
            "10001", "10001", "10001", "10101", "10101", "10101", "01010",
        ],
        'X' => [
            "10001", "10001", "01010", "00100", "01010", "10001", "10001",
        ],
        'Y' => [
            "10001", "10001", "01010", "00100", "00100", "00100", "00100",
        ],
        'Z' => [
            "11111", "00001", "00010", "00100", "01000", "10000", "11111",
        ],
        '0' => [
            "01110", "10001", "10011", "10101", "11001", "10001", "01110",
        ],
        '1' => [
            "00100", "01100", "00100", "00100", "00100", "00100", "01110",
        ],
        '2' => [
            "01110", "10001", "00001", "00010", "00100", "01000", "11111",
        ],
        '3' => [
            "11110", "00001", "00001", "01110", "00001", "00001", "11110",
        ],
        '4' => [
            "00010", "00110", "01010", "10010", "11111", "00010", "00010",
        ],
        '5' => [
            "11111", "10000", "10000", "11110", "00001", "00001", "11110",
        ],
        '6' => [
            "01110", "10000", "10000", "11110", "10001", "10001", "01110",
        ],
        '7' => [
            "11111", "00001", "00010", "00100", "01000", "01000", "01000",
        ],
        '8' => [
            "01110", "10001", "10001", "01110", "10001", "10001", "01110",
        ],
        '9' => [
            "01110", "10001", "10001", "01111", "00001", "00001", "01110",
        ],
        '+' => [
            "00000", "00100", "00100", "11111", "00100", "00100", "00000",
        ],
        '-' => [
            "00000", "00000", "00000", "11111", "00000", "00000", "00000",
        ],
        ' ' => [
            "00000", "00000", "00000", "00000", "00000", "00000", "00000",
        ],
        _ => panic!("unsupported label glyph: {ch}"),
    }
}

fn mount_positions() -> [(f64, f64); MOUNT_BOSS_COUNT] {
    [
        (-STATION_X / 2.0 + 74.0, -STATION_Y / 2.0 + 72.0),
        (-STATION_X / 2.0 + 74.0, STATION_Y / 2.0 - 72.0),
        (STATION_X / 2.0 - 74.0, -STATION_Y / 2.0 + 72.0),
        (STATION_X / 2.0 - 74.0, STATION_Y / 2.0 - 72.0),
        (-STATION_X / 2.0 + 74.0, 0.0),
        (STATION_X / 2.0 - 74.0, 0.0),
        (0.0, -STATION_Y / 2.0 + 72.0),
        (0.0, STATION_Y / 2.0 - 72.0),
    ]
}

fn local_vial_xy(index: usize) -> (f64, f64) {
    let row = index / VIAL_COLS;
    let col = index % VIAL_COLS;
    (
        centered_index(col, VIAL_COLS, VIAL_PITCH_X),
        centered_index(row, VIAL_ROWS, VIAL_PITCH_Y),
    )
}

fn world_vial_xy(index: usize) -> (f64, f64) {
    let (x, y) = local_vial_xy(index);
    (NEST_CENTER.0 + x, NEST_CENTER.1 + y)
}

fn timing_slot_x(index: usize) -> f64 {
    centered_index(index, VIAL_COUNT, TIMING_PITCH_X)
}

fn local_probe_xy(index: usize) -> (f64, f64) {
    if index < VIAL_COUNT {
        (
            centered_index(index % 3, 3, 92.0),
            if index < 3 { 42.0 } else { -22.0 },
        )
    } else {
        (centered_index(index - VIAL_COUNT, 2, 92.0), -86.0)
    }
}

fn world_probe_xy(index: usize) -> (f64, f64) {
    let (x, y) = local_probe_xy(index);
    (PROBE_CENTER.0 + x, PROBE_CENTER.1 + y)
}

fn time_label(index: usize) -> &'static str {
    match STAGGER_MINUTES[index] {
        0 => "T+00",
        3 => "T+03",
        6 => "T+06",
        9 => "T+09",
        12 => "T+12",
        15 => "T+15",
        _ => "T+XX",
    }
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn exported_paths_are_unique_and_named() {
        let unique = OUTPUTS.iter().copied().collect::<HashSet<_>>();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert!(OUTPUTS.iter().all(|path| path.starts_with("output/")));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
        assert!(OUTPUTS
            .iter()
            .all(|path| path.contains("closed_cell_source_vial_thaw_timing_stagger_station")));
    }

    #[test]
    fn vial_and_timing_features_cover_all_stagger_positions() {
        assert_eq!(STAGGER_MINUTES, [0, 3, 6, 9, 12, 15]);
        for index in 0..VIAL_COUNT {
            let (x, y) = world_vial_xy(index);
            assert!(x.abs() + VIAL_COLLAR_D / 2.0 < STATION_X / 2.0 - RIM_W);
            assert!(y.abs() + VIAL_COLLAR_D / 2.0 < STATION_Y / 2.0 - RIM_W);
            assert_eq!(time_label(index), required_label_texts()[8 + index]);
        }
    }

    #[test]
    fn required_features_and_labels_are_present() {
        assert!(REQUIRED_FEATURES.contains(&"timed_vial_nests"));
        assert!(REQUIRED_FEATURES.contains(&"temperature_probe_wells"));
        assert!(REQUIRED_FEATURES.contains(&"barcode_slots"));
        assert!(REQUIRED_FEATURES.contains(&"thaw_start_witness_indicators"));
        assert!(REQUIRED_FEATURES.contains(&"thaw_end_witness_indicators"));
        assert!(REQUIRED_FEATURES.contains(&"sterile_transfer_staging"));
        assert!(REQUIRED_FEATURES.contains(&"drain_condensate_containment"));
        assert!(REQUIRED_FEATURES.contains(&"human_readable_csg_labels"));
        for label in required_label_texts() {
            assert!(label.chars().all(supported_label_char));
        }
    }
}
