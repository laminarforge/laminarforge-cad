use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed cell suspension settling-time, gentle remix, and hold validation station.
//
// Validation/interface CAD for automated seeding prep: source bag/vial nests,
// a gentle mixer envelope, timed settling tokens, top/middle/bottom sampling
// loop witnesses, density coupon windows, temperature pocketing, bubble and
// dead-volume witnesses, release/hold/reject disposition lanes, barcode
// custody, and robot/service keepouts. It is not a live-cell protocol,
// biological acceptance criterion, sterility claim, or mixer control system.

const OUTPUT_PREFIX: &str = "closed_cell_settling_time_mixing_hold_station";

const OUTPUTS: [&str; 13] = [
    "output/closed_cell_settling_time_mixing_hold_station_base_containment_deck.stl",
    "output/closed_cell_settling_time_mixing_hold_station_source_bag_vial_nests.stl",
    "output/closed_cell_settling_time_mixing_hold_station_gentle_mixer_envelope.stl",
    "output/closed_cell_settling_time_mixing_hold_station_timed_settling_token_rail.stl",
    "output/closed_cell_settling_time_mixing_hold_station_top_middle_bottom_sampling_loop_witnesses.stl",
    "output/closed_cell_settling_time_mixing_hold_station_cell_density_coupon_windows.stl",
    "output/closed_cell_settling_time_mixing_hold_station_temperature_pocket.stl",
    "output/closed_cell_settling_time_mixing_hold_station_bubble_dead_volume_witnesses.stl",
    "output/closed_cell_settling_time_mixing_hold_station_release_hold_reject_lanes.stl",
    "output/closed_cell_settling_time_mixing_hold_station_barcode_custody_board.stl",
    "output/closed_cell_settling_time_mixing_hold_station_robot_service_keepouts.stl",
    "output/closed_cell_settling_time_mixing_hold_station_settling_reference_tokens.stl",
    "output/closed_cell_settling_time_mixing_hold_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 10] = [
    "source_bag_vial_nests",
    "gentle_mixer_envelope",
    "timed_settling_token_rail",
    "top_middle_bottom_sampling_loop_witnesses",
    "cell_density_coupon_windows",
    "temperature_pocket",
    "bubble_dead_volume_witnesses",
    "release_hold_reject_lanes",
    "barcode_custody_board",
    "robot_service_keepouts",
];

const STATION_X: f64 = 1540.0;
const STATION_Y: f64 = 940.0;
const DECK_Z: f64 = 22.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 46.0;
const BASIN_DEPTH: f64 = 8.0;
const BASIN_X: f64 = 1320.0;
const BASIN_Y: f64 = 720.0;
const DRAIN_D: f64 = 18.0;
const MOUNT_HOLE_D: f64 = 6.8;
const LEAK_WITNESS_WELLS: usize = 10;

const SOURCE_CENTER: (f64, f64) = (-470.0, 190.0);
const SOURCE_X: f64 = 430.0;
const SOURCE_Y: f64 = 260.0;
const SOURCE_Z: f64 = 48.0;
const BAG_POCKET_X: f64 = 255.0;
const BAG_POCKET_Y: f64 = 128.0;
const BAG_POCKET_DEPTH: f64 = 17.0;
const VIAL_NESTS: usize = 4;
const VIAL_WELL_D: f64 = 32.0;
const VIAL_COLLAR_D: f64 = 52.0;
const VIAL_WELL_DEPTH: f64 = 30.0;
const SOURCE_LOCATOR_PINS: usize = 6;
const SOURCE_PORT_SADDLES: usize = 5;

const MIXER_CENTER: (f64, f64) = (10.0, 190.0);
const MIXER_X: f64 = 420.0;
const MIXER_Y: f64 = 260.0;
const MIXER_Z: f64 = 36.0;
const MIXER_SWEEP_LIMIT_DEG: f64 = 8.0;
const MIXER_ARC_SEGMENTS: usize = 17;
const MIXER_CRADLE_RIBS: usize = 7;
const MIXER_BUMPER_POSTS: usize = 6;
const MIXER_CLEARANCE_Z: f64 = 150.0;

const TOKEN_CENTER: (f64, f64) = (0.0, 388.0);
const TOKEN_RAIL_X: f64 = 1260.0;
const TOKEN_RAIL_Y: f64 = 70.0;
const TOKEN_RAIL_Z: f64 = 24.0;
const SETTLING_TOKEN_COUNT: usize = 8;
const SETTLING_MINUTES: [usize; SETTLING_TOKEN_COUNT] = [0, 5, 10, 15, 20, 30, 45, 60];
const TOKEN_SLOT_X: f64 = 72.0;
const TOKEN_SLOT_Y: f64 = 28.0;
const TOKEN_SLOT_DEPTH: f64 = 7.0;
const TOKEN_PITCH_X: f64 = 150.0;

const SAMPLE_CENTER: (f64, f64) = (480.0, 165.0);
const SAMPLE_X: f64 = 390.0;
const SAMPLE_Y: f64 = 290.0;
const SAMPLE_Z: f64 = 42.0;
const SAMPLE_LEVELS: usize = 3;
const SAMPLE_LOOPS_PER_LEVEL: usize = 4;
const SAMPLE_LOOP_D: f64 = 18.0;
const SAMPLE_LOOP_DEPTH: f64 = 20.0;
const SAMPLE_LOOP_PITCH_X: f64 = 70.0;
const SAMPLE_LEVEL_PITCH_Y: f64 = 82.0;

const DENSITY_CENTER: (f64, f64) = (-500.0, -90.0);
const DENSITY_X: f64 = 360.0;
const DENSITY_Y: f64 = 190.0;
const DENSITY_Z: f64 = 24.0;
const DENSITY_WINDOWS: usize = 12;
const DENSITY_COLS: usize = 4;
const DENSITY_WINDOW_X: f64 = 54.0;
const DENSITY_WINDOW_Y: f64 = 34.0;
const DENSITY_LADDER_STEPS: usize = 6;

const TEMP_CENTER: (f64, f64) = (-100.0, -98.0);
const TEMP_X: f64 = 320.0;
const TEMP_Y: f64 = 180.0;
const TEMP_Z: f64 = 38.0;
const TEMP_POCKETS: usize = 3;
const TEMP_POCKET_D: f64 = 42.0;
const TEMP_PROBE_WELLS: usize = 4;
const TEMP_PROBE_D: f64 = 8.0;
const TEMP_CABLE_CHANNELS: usize = 4;

const BUBBLE_CENTER: (f64, f64) = (290.0, -105.0);
const BUBBLE_X: f64 = 360.0;
const BUBBLE_Y: f64 = 190.0;
const BUBBLE_Z: f64 = 42.0;
const BUBBLE_SIGHT_CHANNELS: usize = 4;
const DEAD_VOLUME_WELLS: usize = 6;
const DEAD_VOLUME_WELL_D: f64 = 24.0;
const BUBBLE_LADDER_TICKS: usize = 10;

const DISPOSITION_CENTER: (f64, f64) = (540.0, -330.0);
const DISPOSITION_X: f64 = 330.0;
const DISPOSITION_Y: f64 = 150.0;
const DISPOSITION_Z: f64 = 24.0;
const DISPOSITION_LANES: usize = 3;
const DISPOSITION_TOKENS_PER_LANE: usize = 4;

const BARCODE_CENTER: (f64, f64) = (40.0, -330.0);
const BARCODE_X: f64 = 420.0;
const BARCODE_Y: f64 = 150.0;
const BARCODE_Z: f64 = 18.0;
const BARCODE_SLOTS: usize = 10;
const BARCODE_SLOT_X: f64 = 58.0;
const BARCODE_SLOT_Y: f64 = 24.0;
const BARCODE_FIDUCIALS: usize = 4;

const REFERENCE_CENTER: (f64, f64) = (-500.0, -330.0);
const REFERENCE_X: f64 = 350.0;
const REFERENCE_Y: f64 = 150.0;
const REFERENCE_Z: f64 = 20.0;
const REFERENCE_TOKENS: usize = 9;
const REFERENCE_TOKEN_D: f64 = 24.0;

const KEEP_OUT_X: f64 = 1460.0;
const KEEP_OUT_Y: f64 = 850.0;
const FRONT_ROBOT_CLEARANCE: f64 = 430.0;
const REAR_SERVICE_CLEARANCE: f64 = 270.0;
const LEFT_SOURCE_SERVICE_CLEARANCE: f64 = 260.0;
const RIGHT_SAMPLING_SERVICE_CLEARANCE: f64 = 260.0;
const TOP_MIXER_LIFT_CLEARANCE: f64 = 340.0;
const KEEP_OUT_RAIL_Z: f64 = 6.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_station(self) -> bool {
        let usable_x = STATION_X / 2.0 - RIM_W - 14.0;
        let usable_y = STATION_Y / 2.0 - RIM_W - 14.0;

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

    let deck = base_containment_deck();
    export(OUTPUTS[0], &deck);

    let source = source_bag_vial_nests();
    export(OUTPUTS[1], &source);

    let mixer = gentle_mixer_envelope();
    export(OUTPUTS[2], &mixer);

    let token_rail = timed_settling_token_rail();
    export(OUTPUTS[3], &token_rail);

    let sample = top_middle_bottom_sampling_loop_witnesses();
    export(OUTPUTS[4], &sample);

    let density = cell_density_coupon_windows();
    export(OUTPUTS[5], &density);

    let temperature = temperature_pocket();
    export(OUTPUTS[6], &temperature);

    let bubbles = bubble_dead_volume_witnesses();
    export(OUTPUTS[7], &bubbles);

    let disposition = release_hold_reject_lanes();
    export(OUTPUTS[8], &disposition);

    let barcode = barcode_custody_board();
    export(OUTPUTS[9], &barcode);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let references = settling_reference_tokens();
    export(OUTPUTS[11], &references);

    let assembly = station_assembly();
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed cell settling-time, gentle remix, and hold validation station:");
    println!(
        "  Source custody: {VIAL_NESTS} vial nests plus one source-bag pocket with {SOURCE_PORT_SADDLES} closed-transfer port saddles"
    );
    println!(
        "  Gentle remix:   +/-{MIXER_SWEEP_LIMIT_DEG:.0}deg mixer envelope, {MIXER_CRADLE_RIBS} low-shear cradle ribs, {MIXER_BUMPER_POSTS} bumper posts, {MIXER_CLEARANCE_Z:.0}mm local height gauge"
    );
    println!(
        "  Settling rail:  {SETTLING_TOKEN_COUNT} timed token stations for {SETTLING_MINUTES:?} minute checks"
    );
    println!(
        "  Sampling:       {SAMPLE_LEVELS} top/middle/bottom levels x {SAMPLE_LOOPS_PER_LEVEL} loop witnesses, {DENSITY_WINDOWS} density coupon windows"
    );
    println!(
        "  Hold evidence:  {TEMP_POCKETS} temperature pockets, {BUBBLE_SIGHT_CHANNELS} bubble sight channels, {DEAD_VOLUME_WELLS} dead-volume wells, release/hold/reject lanes"
    );
    println!(
        "  Keepouts:       front robot {FRONT_ROBOT_CLEARANCE:.0}mm, rear service {REAR_SERVICE_CLEARANCE:.0}mm, source side {LEFT_SOURCE_SERVICE_CLEARANCE:.0}mm, sample side {RIGHT_SAMPLING_SERVICE_CLEARANCE:.0}mm"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    base_containment_deck()
        + source_bag_vial_nests().translate(SOURCE_CENTER.0, SOURCE_CENTER.1, DECK_Z)
        + gentle_mixer_envelope().translate(MIXER_CENTER.0, MIXER_CENTER.1, DECK_Z)
        + timed_settling_token_rail().translate(TOKEN_CENTER.0, TOKEN_CENTER.1, DECK_Z)
        + top_middle_bottom_sampling_loop_witnesses().translate(
            SAMPLE_CENTER.0,
            SAMPLE_CENTER.1,
            DECK_Z,
        )
        + cell_density_coupon_windows().translate(DENSITY_CENTER.0, DENSITY_CENTER.1, DECK_Z)
        + temperature_pocket().translate(TEMP_CENTER.0, TEMP_CENTER.1, DECK_Z)
        + bubble_dead_volume_witnesses().translate(BUBBLE_CENTER.0, BUBBLE_CENTER.1, DECK_Z)
        + release_hold_reject_lanes().translate(DISPOSITION_CENTER.0, DISPOSITION_CENTER.1, DECK_Z)
        + barcode_custody_board().translate(BARCODE_CENTER.0, BARCODE_CENTER.1, DECK_Z)
        + settling_reference_tokens().translate(REFERENCE_CENTER.0, REFERENCE_CENTER.1, DECK_Z)
        + robot_service_keepouts()
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 13, "expected stable STL export count");
    assert_eq!(
        SAMPLE_LEVELS, 3,
        "sampling witnesses must stay top/middle/bottom"
    );
    assert_eq!(
        DISPOSITION_LANES, 3,
        "disposition board must retain release/hold/reject lanes"
    );
    assert!(
        SETTLING_MINUTES
            .windows(2)
            .all(|window| window[0] < window[1]),
        "settling tokens must increase monotonically"
    );
    assert!(
        SETTLING_MINUTES[0] == 0 && SETTLING_MINUTES[SETTLING_TOKEN_COUNT - 1] >= 60,
        "settling rail must span immediate through one-hour hold checks"
    );
    assert!(
        TOP_MIXER_LIFT_CLEARANCE >= 2.0 * MIXER_CLEARANCE_Z,
        "top keepout must clear the gentle mixer envelope and service lift"
    );
    assert!(
        FRONT_ROBOT_CLEARANCE >= 400.0 && REAR_SERVICE_CLEARANCE >= 260.0,
        "robot and service clearances are below station target"
    );
    assert!(
        LEFT_SOURCE_SERVICE_CLEARANCE >= 250.0 && RIGHT_SAMPLING_SERVICE_CLEARANCE >= 250.0,
        "source/sample side service clearances are below station target"
    );

    let rects = layout_rects();
    for rect in rects {
        assert!(
            rect.fits_inside_station(),
            "{} does not fit inside containment deck",
            rect.name
        );
    }
    assert!(
        keepout_rect().fits_inside_station(),
        "robot/service keepout outline does not fit on deck"
    );

    for i in 0..rects.len() {
        for other in rects.iter().skip(i + 1) {
            assert!(
                !rects[i].overlaps(*other),
                "{} overlaps {}",
                rects[i].name,
                other.name
            );
        }
    }
}

fn layout_rects() -> [Rect; 10] {
    [
        Rect {
            name: "source_bag_vial_nests",
            center: SOURCE_CENTER,
            x: SOURCE_X,
            y: SOURCE_Y,
        },
        Rect {
            name: "gentle_mixer_envelope",
            center: MIXER_CENTER,
            x: MIXER_X,
            y: MIXER_Y,
        },
        Rect {
            name: "timed_settling_token_rail",
            center: TOKEN_CENTER,
            x: TOKEN_RAIL_X,
            y: TOKEN_RAIL_Y,
        },
        Rect {
            name: "top_middle_bottom_sampling_loop_witnesses",
            center: SAMPLE_CENTER,
            x: SAMPLE_X,
            y: SAMPLE_Y,
        },
        Rect {
            name: "cell_density_coupon_windows",
            center: DENSITY_CENTER,
            x: DENSITY_X,
            y: DENSITY_Y,
        },
        Rect {
            name: "temperature_pocket",
            center: TEMP_CENTER,
            x: TEMP_X,
            y: TEMP_Y,
        },
        Rect {
            name: "bubble_dead_volume_witnesses",
            center: BUBBLE_CENTER,
            x: BUBBLE_X,
            y: BUBBLE_Y,
        },
        Rect {
            name: "release_hold_reject_lanes",
            center: DISPOSITION_CENTER,
            x: DISPOSITION_X,
            y: DISPOSITION_Y,
        },
        Rect {
            name: "barcode_custody_board",
            center: BARCODE_CENTER,
            x: BARCODE_X,
            y: BARCODE_Y,
        },
        Rect {
            name: "settling_reference_tokens",
            center: REFERENCE_CENTER,
            x: REFERENCE_X,
            y: REFERENCE_Y,
        },
    ]
}

fn keepout_rect() -> Rect {
    Rect {
        name: "robot_service_keepouts",
        center: (0.0, 0.0),
        x: KEEP_OUT_X,
        y: KEEP_OUT_Y,
    }
}

fn base_containment_deck() -> Part {
    let deck = centered_cube(
        format!("{OUTPUT_PREFIX}_deck_plate"),
        STATION_X,
        STATION_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let basin = centered_cube(
        format!("{OUTPUT_PREFIX}_recessed_spill_basin_cut"),
        BASIN_X,
        BASIN_Y,
        BASIN_DEPTH + 1.0,
    )
    .translate(0.0, -12.0, DECK_Z - BASIN_DEPTH / 2.0 + 0.5);
    let drain = centered_cylinder(
        format!("{OUTPUT_PREFIX}_front_closed_cell_spill_drain_cut"),
        DRAIN_D / 2.0,
        RIM_W + 36.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, -STATION_Y / 2.0 + 12.0, DECK_Z - 6.0);

    deck - basin - drain - deck_mount_holes()
        + containment_rims()
        + leak_witness_wells()
        + station_floor_markers()
        + deck_datum_targets()
}

fn containment_rims() -> Part {
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

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty(format!("{OUTPUT_PREFIX}_deck_mount_holes"));
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 62.0, -STATION_Y / 2.0 + 62.0),
        (0.0, -STATION_Y / 2.0 + 62.0),
        (STATION_X / 2.0 - 62.0, -STATION_Y / 2.0 + 62.0),
        (-STATION_X / 2.0 + 62.0, STATION_Y / 2.0 - 62.0),
        (0.0, STATION_Y / 2.0 - 62.0),
        (STATION_X / 2.0 - 62.0, STATION_Y / 2.0 - 62.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_m6_mount_hole_{i}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 4.0,
                28,
            )
            .translate(*x, *y, DECK_Z / 2.0);
    }
    holes
}

fn leak_witness_wells() -> Part {
    let mut wells = Part::empty(format!("{OUTPUT_PREFIX}_leak_witness_wells"));
    for i in 0..LEAK_WITNESS_WELLS {
        let x = centered_index(i % 5, 5, 92.0);
        let y = -382.0 + (i / 5) as f64 * 34.0;
        wells = wells
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_settled_cell_leak_witness_well_{i}"),
                14.0,
                7.0,
                28,
            )
            .translate(x, y, DECK_Z + 3.5);
    }
    wells
}

fn station_floor_markers() -> Part {
    let mut markers = Part::empty(format!("{OUTPUT_PREFIX}_module_floor_markers"));
    for rect in layout_rects() {
        markers = markers
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{}_floor_marker", rect.name),
                rect.x + 18.0,
                rect.y + 14.0,
                3.0,
            )
            .translate(rect.center.0, rect.center.1, DECK_Z + 1.5);
    }
    markers
}

fn deck_datum_targets() -> Part {
    let mut targets = Part::empty(format!("{OUTPUT_PREFIX}_deck_datum_targets"));
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 108.0, -STATION_Y / 2.0 + 104.0),
        (STATION_X / 2.0 - 108.0, -STATION_Y / 2.0 + 104.0),
        (-STATION_X / 2.0 + 108.0, STATION_Y / 2.0 - 104.0),
        (STATION_X / 2.0 - 108.0, STATION_Y / 2.0 - 104.0),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("{OUTPUT_PREFIX}_machine_vision_datum_boss_{i}"),
            18.0,
            7.0,
            36,
        )
        .translate(*x, *y, DECK_Z + 3.5);
        let center_cut = centered_cylinder(
            format!("{OUTPUT_PREFIX}_machine_vision_datum_center_cut_{i}"),
            4.0,
            8.0,
            24,
        )
        .translate(*x, *y, DECK_Z + 4.0);
        targets = targets + (boss - center_cut);
    }
    targets
}

fn source_bag_vial_nests() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_source_bag_vial_nest_body"),
        SOURCE_X,
        SOURCE_Y,
        SOURCE_Z,
    )
    .translate(0.0, 0.0, SOURCE_Z / 2.0);
    let bag_pocket = centered_cube(
        format!("{OUTPUT_PREFIX}_source_bag_pocket_cut"),
        BAG_POCKET_X,
        BAG_POCKET_Y,
        BAG_POCKET_DEPTH + 1.0,
    )
    .translate(-62.0, 12.0, SOURCE_Z - BAG_POCKET_DEPTH / 2.0 + 0.5);

    body - bag_pocket - vial_well_cuts()
        + bag_saddle_ribs()
        + vial_collar_bosses()
        + source_locator_pins()
        + source_port_saddles()
}

fn vial_well_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_source_vial_well_cuts"));
    for i in 0..VIAL_NESTS {
        let x = 112.0 + centered_index(i % 2, 2, 62.0);
        let y = centered_index(i / 2, 2, 74.0);
        cuts = cuts
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_source_vial_well_cut_{i}"),
                VIAL_WELL_D / 2.0,
                VIAL_WELL_DEPTH + 1.0,
                36,
            )
            .translate(x, y, SOURCE_Z - VIAL_WELL_DEPTH / 2.0 + 0.5);
    }
    cuts
}

fn vial_collar_bosses() -> Part {
    let mut collars = Part::empty(format!("{OUTPUT_PREFIX}_source_vial_collar_bosses"));
    for i in 0..VIAL_NESTS {
        let x = 112.0 + centered_index(i % 2, 2, 62.0);
        let y = centered_index(i / 2, 2, 74.0);
        let collar = centered_cylinder(
            format!("{OUTPUT_PREFIX}_source_vial_collar_{i}"),
            VIAL_COLLAR_D / 2.0,
            12.0,
            36,
        )
        .translate(x, y, SOURCE_Z + 6.0);
        let bore = centered_cylinder(
            format!("{OUTPUT_PREFIX}_source_vial_collar_opening_cut_{i}"),
            VIAL_WELL_D / 2.0,
            13.0,
            32,
        )
        .translate(x, y, SOURCE_Z + 6.5);
        collars = collars + (collar - bore);
    }
    collars
}

fn bag_saddle_ribs() -> Part {
    let mut ribs = Part::empty(format!("{OUTPUT_PREFIX}_source_bag_saddle_ribs"));
    for i in 0..5 {
        ribs = ribs
            + centered_cube(
                format!("{OUTPUT_PREFIX}_source_bag_gentle_saddle_rib_{i}"),
                BAG_POCKET_X - 28.0,
                5.0,
                9.0,
            )
            .translate(-62.0, 12.0 + centered_index(i, 5, 24.0), SOURCE_Z + 4.5);
    }
    ribs
}

fn source_locator_pins() -> Part {
    let mut pins = Part::empty(format!("{OUTPUT_PREFIX}_source_locator_pins"));
    for i in 0..SOURCE_LOCATOR_PINS {
        let x = if i % 2 == 0 { -166.0 } else { -30.0 };
        let y = centered_index(i / 2, 3, 70.0);
        pins = pins
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_source_soft_locator_pin_{i}"),
                7.0,
                15.0,
                24,
            )
            .translate(x, y, SOURCE_Z + 7.5);
    }
    pins
}

fn source_port_saddles() -> Part {
    let mut saddles = Part::empty(format!(
        "{OUTPUT_PREFIX}_source_closed_transfer_port_saddles"
    ));
    for i in 0..SOURCE_PORT_SADDLES {
        saddles = saddles
            + centered_cube(
                format!("{OUTPUT_PREFIX}_source_port_saddle_{i}"),
                38.0,
                18.0,
                12.0,
            )
            .translate(
                centered_index(i, SOURCE_PORT_SADDLES, 47.0) - 56.0,
                -SOURCE_Y / 2.0 + 25.0,
                SOURCE_Z + 6.0,
            );
    }
    saddles
}

fn gentle_mixer_envelope() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_gentle_mixer_base_envelope"),
        MIXER_X,
        MIXER_Y,
        MIXER_Z,
    )
    .translate(0.0, 0.0, MIXER_Z / 2.0);
    let cassette_sweep_relief = centered_cube(
        format!("{OUTPUT_PREFIX}_gentle_mixer_suspension_bag_sweep_relief_cut"),
        MIXER_X - 96.0,
        MIXER_Y - 84.0,
        12.0,
    )
    .translate(0.0, 0.0, MIXER_Z - 5.0);

    body - cassette_sweep_relief
        + mixer_cradle_ribs()
        + mixer_arc_witness_rails()
        + mixer_bumper_posts()
        + mixer_height_gauge_posts()
}

fn mixer_cradle_ribs() -> Part {
    let mut ribs = Part::empty(format!(
        "{OUTPUT_PREFIX}_gentle_mixer_low_shear_cradle_ribs"
    ));
    for i in 0..MIXER_CRADLE_RIBS {
        ribs = ribs
            + centered_cube(
                format!("{OUTPUT_PREFIX}_gentle_mixer_cradle_rib_{i}"),
                MIXER_X - 130.0,
                5.0,
                10.0,
            )
            .translate(
                0.0,
                centered_index(i, MIXER_CRADLE_RIBS, 24.0),
                MIXER_Z + 5.0,
            );
    }
    ribs
}

fn mixer_arc_witness_rails() -> Part {
    mixer_arc_witness_rail(-84.0, "front") + mixer_arc_witness_rail(84.0, "rear")
}

fn mixer_arc_witness_rail(y_offset: f64, side: &str) -> Part {
    let mut rail = Part::empty(format!("{OUTPUT_PREFIX}_{side}_gentle_mixer_sweep_arc"));
    for i in 0..MIXER_ARC_SEGMENTS {
        let frac = i as f64 / (MIXER_ARC_SEGMENTS - 1) as f64;
        let deg = -MIXER_SWEEP_LIMIT_DEG + frac * MIXER_SWEEP_LIMIT_DEG * 2.0;
        let rad = deg.to_radians();
        let x = rad.sin() * 230.0;
        let z = MIXER_Z + 16.0 + (1.0 - rad.cos()) * 230.0;
        rail = rail
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{side}_mixer_arc_tick_{i}"),
                18.0,
                16.0,
                8.0,
            )
            .translate(x, y_offset, z);
    }
    rail
}

fn mixer_bumper_posts() -> Part {
    let mut posts = Part::empty(format!("{OUTPUT_PREFIX}_gentle_mixer_soft_bumper_posts"));
    for i in 0..MIXER_BUMPER_POSTS {
        let x = centered_index(i % 3, 3, 96.0);
        let y = if i < 3 {
            -MIXER_Y / 2.0 + 34.0
        } else {
            MIXER_Y / 2.0 - 34.0
        };
        posts = posts
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_gentle_mixer_soft_bumper_{i}"),
                13.0,
                34.0,
                28,
            )
            .translate(x, y, MIXER_Z + 17.0);
    }
    posts
}

fn mixer_height_gauge_posts() -> Part {
    let left = centered_cube(
        format!("{OUTPUT_PREFIX}_left_mixer_clearance_height_gauge"),
        16.0,
        26.0,
        MIXER_CLEARANCE_Z,
    )
    .translate(
        -MIXER_X / 2.0 + 36.0,
        0.0,
        MIXER_Z + MIXER_CLEARANCE_Z / 2.0,
    );
    let right = centered_cube(
        format!("{OUTPUT_PREFIX}_right_mixer_clearance_height_gauge"),
        16.0,
        26.0,
        MIXER_CLEARANCE_Z,
    )
    .translate(MIXER_X / 2.0 - 36.0, 0.0, MIXER_Z + MIXER_CLEARANCE_Z / 2.0);

    left + right
}

fn timed_settling_token_rail() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_timed_settling_token_rail_body"),
        TOKEN_RAIL_X,
        TOKEN_RAIL_Y,
        TOKEN_RAIL_Z,
    )
    .translate(0.0, 0.0, TOKEN_RAIL_Z / 2.0);
    body - settling_token_slot_cuts() + settling_token_index_posts() + settling_time_label_lands()
}

fn settling_token_slot_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_settling_token_slot_cuts"));
    for i in 0..SETTLING_TOKEN_COUNT {
        cuts = cuts
            + centered_cube(
                format!(
                    "{OUTPUT_PREFIX}_settling_token_slot_{}min_cut",
                    SETTLING_MINUTES[i]
                ),
                TOKEN_SLOT_X,
                TOKEN_SLOT_Y,
                TOKEN_SLOT_DEPTH + 1.0,
            )
            .translate(
                centered_index(i, SETTLING_TOKEN_COUNT, TOKEN_PITCH_X),
                0.0,
                TOKEN_RAIL_Z - TOKEN_SLOT_DEPTH / 2.0 + 0.5,
            );
    }
    cuts
}

fn settling_token_index_posts() -> Part {
    let mut posts = Part::empty(format!("{OUTPUT_PREFIX}_settling_token_index_posts"));
    for i in 0..SETTLING_TOKEN_COUNT {
        posts = posts
            + centered_cylinder(
                format!(
                    "{OUTPUT_PREFIX}_settling_token_{}min_index_post",
                    SETTLING_MINUTES[i]
                ),
                6.0,
                14.0,
                20,
            )
            .translate(
                centered_index(i, SETTLING_TOKEN_COUNT, TOKEN_PITCH_X),
                TOKEN_RAIL_Y / 2.0 - 15.0,
                TOKEN_RAIL_Z + 7.0,
            );
    }
    posts
}

fn settling_time_label_lands() -> Part {
    let mut labels = Part::empty(format!("{OUTPUT_PREFIX}_settling_time_label_lands"));
    for i in 0..SETTLING_TOKEN_COUNT {
        labels = labels
            + centered_cube(
                format!(
                    "{OUTPUT_PREFIX}_settling_time_{}min_label_land",
                    SETTLING_MINUTES[i]
                ),
                46.0,
                10.0,
                4.0,
            )
            .translate(
                centered_index(i, SETTLING_TOKEN_COUNT, TOKEN_PITCH_X),
                -TOKEN_RAIL_Y / 2.0 + 12.0,
                TOKEN_RAIL_Z + 2.0,
            );
    }
    labels
}

fn top_middle_bottom_sampling_loop_witnesses() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_top_middle_bottom_sampling_loop_block"),
        SAMPLE_X,
        SAMPLE_Y,
        SAMPLE_Z,
    )
    .translate(0.0, 0.0, SAMPLE_Z / 2.0);

    body - sampling_loop_cuts() + sampling_loop_retainer_posts() + sampling_level_label_lands()
}

fn sampling_loop_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_sampling_loop_cuts"));
    for level in 0..SAMPLE_LEVELS {
        let y = centered_index(level, SAMPLE_LEVELS, SAMPLE_LEVEL_PITCH_Y);
        for loop_i in 0..SAMPLE_LOOPS_PER_LEVEL {
            cuts = cuts
                + centered_cylinder(
                    format!("{OUTPUT_PREFIX}_level_{level}_sample_loop_{loop_i}_pocket_cut"),
                    SAMPLE_LOOP_D / 2.0,
                    SAMPLE_LOOP_DEPTH + 1.0,
                    32,
                )
                .translate(
                    centered_index(loop_i, SAMPLE_LOOPS_PER_LEVEL, SAMPLE_LOOP_PITCH_X),
                    y,
                    SAMPLE_Z - SAMPLE_LOOP_DEPTH / 2.0 + 0.5,
                );
        }
    }
    cuts
}

fn sampling_loop_retainer_posts() -> Part {
    let mut posts = Part::empty(format!("{OUTPUT_PREFIX}_sampling_loop_retainer_posts"));
    for level in 0..SAMPLE_LEVELS {
        let y = centered_index(level, SAMPLE_LEVELS, SAMPLE_LEVEL_PITCH_Y);
        for i in 0..5 {
            posts = posts
                + centered_cylinder(
                    format!("{OUTPUT_PREFIX}_level_{level}_sample_loop_retainer_post_{i}"),
                    4.5,
                    12.0,
                    18,
                )
                .translate(centered_index(i, 5, 54.0), y - 28.0, SAMPLE_Z + 6.0);
        }
    }
    posts
}

fn sampling_level_label_lands() -> Part {
    let mut labels = Part::empty(format!("{OUTPUT_PREFIX}_top_middle_bottom_label_lands"));
    for level in 0..SAMPLE_LEVELS {
        let y = centered_index(level, SAMPLE_LEVELS, SAMPLE_LEVEL_PITCH_Y);
        labels = labels
            + centered_cube(
                format!("{OUTPUT_PREFIX}_sample_level_{level}_label_land"),
                SAMPLE_X - 48.0,
                10.0,
                5.0,
            )
            .translate(0.0, y + 30.0, SAMPLE_Z + 2.5);
    }
    labels
}

fn cell_density_coupon_windows() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_cell_density_coupon_window_plate"),
        DENSITY_X,
        DENSITY_Y,
        DENSITY_Z,
    )
    .translate(0.0, 0.0, DENSITY_Z / 2.0);
    body + density_window_frames() + density_reference_ladder()
}

fn density_window_frames() -> Part {
    let mut frames = Part::empty(format!("{OUTPUT_PREFIX}_cell_density_coupon_window_frames"));
    for i in 0..DENSITY_WINDOWS {
        let x = centered_index(i % DENSITY_COLS, DENSITY_COLS, 78.0);
        let y = centered_index(i / DENSITY_COLS, DENSITY_WINDOWS / DENSITY_COLS, 52.0);
        let frame = centered_cube(
            format!("{OUTPUT_PREFIX}_density_coupon_window_frame_{i}"),
            DENSITY_WINDOW_X,
            DENSITY_WINDOW_Y,
            10.0,
        )
        .translate(x, y, DENSITY_Z + 5.0);
        let clear = centered_cube(
            format!("{OUTPUT_PREFIX}_density_coupon_clear_aperture_cut_{i}"),
            DENSITY_WINDOW_X - 16.0,
            DENSITY_WINDOW_Y - 12.0,
            12.0,
        )
        .translate(x, y, DENSITY_Z + 6.0);
        frames = frames + (frame - clear);
    }
    frames
}

fn density_reference_ladder() -> Part {
    let mut ladder = Part::empty(format!("{OUTPUT_PREFIX}_cell_density_reference_ladder"));
    for i in 0..DENSITY_LADDER_STEPS {
        ladder = ladder
            + centered_cube(
                format!("{OUTPUT_PREFIX}_density_reference_step_{i}"),
                24.0 + i as f64 * 8.0,
                10.0,
                4.0,
            )
            .translate(
                -DENSITY_X / 2.0 + 45.0 + i as f64 * 36.0,
                -DENSITY_Y / 2.0 + 20.0,
                DENSITY_Z + 2.0,
            );
    }
    ladder
}

fn temperature_pocket() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_temperature_pocket_block"),
        TEMP_X,
        TEMP_Y,
        TEMP_Z,
    )
    .translate(0.0, 0.0, TEMP_Z / 2.0);
    body - temperature_pocket_cuts() - temperature_probe_well_cuts()
        + temperature_sensor_collars()
        + temperature_cable_comb()
}

fn temperature_pocket_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_temperature_pocket_cuts"));
    for i in 0..TEMP_POCKETS {
        cuts = cuts
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_sealed_hold_temperature_pocket_{i}_cut"),
                TEMP_POCKET_D / 2.0,
                22.0,
                36,
            )
            .translate(centered_index(i, TEMP_POCKETS, 76.0), 22.0, TEMP_Z - 11.0);
    }
    cuts
}

fn temperature_probe_well_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_temperature_probe_well_cuts"));
    for i in 0..TEMP_PROBE_WELLS {
        cuts = cuts
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_temperature_probe_well_{i}_cut"),
                TEMP_PROBE_D / 2.0,
                TEMP_Z + 2.0,
                20,
            )
            .translate(
                centered_index(i, TEMP_PROBE_WELLS, 48.0),
                -48.0,
                TEMP_Z / 2.0,
            );
    }
    cuts
}

fn temperature_sensor_collars() -> Part {
    let mut collars = Part::empty(format!("{OUTPUT_PREFIX}_temperature_sensor_collars"));
    for i in 0..TEMP_POCKETS {
        let boss = centered_cylinder(
            format!("{OUTPUT_PREFIX}_temperature_pocket_collar_{i}"),
            TEMP_POCKET_D / 2.0 + 8.0,
            9.0,
            36,
        )
        .translate(centered_index(i, TEMP_POCKETS, 76.0), 22.0, TEMP_Z + 4.5);
        let bore = centered_cylinder(
            format!("{OUTPUT_PREFIX}_temperature_pocket_collar_opening_cut_{i}"),
            TEMP_POCKET_D / 2.0,
            10.0,
            32,
        )
        .translate(centered_index(i, TEMP_POCKETS, 76.0), 22.0, TEMP_Z + 5.0);
        collars = collars + (boss - bore);
    }
    collars
}

fn temperature_cable_comb() -> Part {
    let mut comb = Part::empty(format!("{OUTPUT_PREFIX}_temperature_cable_comb"));
    for i in 0..TEMP_CABLE_CHANNELS {
        let saddle = centered_cube(
            format!("{OUTPUT_PREFIX}_temperature_cable_saddle_{i}"),
            34.0,
            14.0,
            10.0,
        )
        .translate(
            centered_index(i, TEMP_CABLE_CHANNELS, 44.0),
            -TEMP_Y / 2.0 + 18.0,
            TEMP_Z + 5.0,
        );
        let channel = centered_cylinder(
            format!("{OUTPUT_PREFIX}_temperature_cable_channel_cut_{i}"),
            4.0,
            16.0,
            18,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            centered_index(i, TEMP_CABLE_CHANNELS, 44.0),
            -TEMP_Y / 2.0 + 18.0,
            TEMP_Z + 5.0,
        );
        comb = comb + (saddle - channel);
    }
    comb
}

fn bubble_dead_volume_witnesses() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_bubble_dead_volume_witness_block"),
        BUBBLE_X,
        BUBBLE_Y,
        BUBBLE_Z,
    )
    .translate(0.0, 0.0, BUBBLE_Z / 2.0);
    body - bubble_sight_channel_cuts() - dead_volume_well_cuts()
        + bubble_ladder_ticks()
        + dead_volume_route_witness_strips()
}

fn bubble_sight_channel_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_bubble_sight_channel_cuts"));
    for i in 0..BUBBLE_SIGHT_CHANNELS {
        cuts = cuts
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_bubble_sight_tube_{i}_cut"),
                8.0,
                BUBBLE_Y + 3.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(i, BUBBLE_SIGHT_CHANNELS, 56.0),
                0.0,
                BUBBLE_Z - 14.0,
            );
    }
    cuts
}

fn dead_volume_well_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_dead_volume_well_cuts"));
    for i in 0..DEAD_VOLUME_WELLS {
        cuts = cuts
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_dead_volume_well_{i}_cut"),
                DEAD_VOLUME_WELL_D / 2.0,
                14.0,
                28,
            )
            .translate(
                centered_index(i % 3, 3, 74.0) + 42.0,
                if i < 3 { 48.0 } else { 72.0 },
                BUBBLE_Z - 7.0,
            );
    }
    cuts
}

fn bubble_ladder_ticks() -> Part {
    let mut ticks = Part::empty(format!("{OUTPUT_PREFIX}_bubble_ladder_ticks"));
    for i in 0..BUBBLE_LADDER_TICKS {
        ticks = ticks
            + centered_cube(
                format!("{OUTPUT_PREFIX}_bubble_ladder_tick_{i}"),
                if i % 5 == 0 { 34.0 } else { 20.0 },
                4.0,
                5.0,
            )
            .translate(
                -BUBBLE_X / 2.0 + 35.0,
                centered_index(i, BUBBLE_LADDER_TICKS, 12.0),
                BUBBLE_Z + 2.5,
            );
    }
    ticks
}

fn dead_volume_route_witness_strips() -> Part {
    let mut strips = Part::empty(format!("{OUTPUT_PREFIX}_dead_volume_route_witness_strips"));
    for i in 0..4 {
        strips = strips
            + centered_cube(
                format!("{OUTPUT_PREFIX}_dead_volume_route_strip_{i}"),
                126.0,
                8.0,
                5.0,
            )
            .translate(54.0, -70.0 + i as f64 * 22.0, BUBBLE_Z + 2.5);
    }
    strips
}

fn release_hold_reject_lanes() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_release_hold_reject_lane_board"),
        DISPOSITION_X,
        DISPOSITION_Y,
        DISPOSITION_Z,
    )
    .translate(0.0, 0.0, DISPOSITION_Z / 2.0);
    body - disposition_lane_recesses() + disposition_lane_tokens() + disposition_gate_posts()
}

fn disposition_lane_recesses() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_release_hold_reject_lane_recesses"));
    for lane in 0..DISPOSITION_LANES {
        cuts = cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_disposition_lane_{lane}_recess_cut"),
                DISPOSITION_X - 54.0,
                28.0,
                7.0,
            )
            .translate(
                0.0,
                centered_index(lane, DISPOSITION_LANES, 44.0),
                DISPOSITION_Z - 3.5,
            );
    }
    cuts
}

fn disposition_lane_tokens() -> Part {
    let mut tokens = Part::empty(format!("{OUTPUT_PREFIX}_release_hold_reject_lane_tokens"));
    for lane in 0..DISPOSITION_LANES {
        let y = centered_index(lane, DISPOSITION_LANES, 44.0);
        for token in 0..DISPOSITION_TOKENS_PER_LANE {
            tokens = tokens
                + centered_cube(
                    format!("{OUTPUT_PREFIX}_disposition_lane_{lane}_token_{token}"),
                    36.0,
                    16.0,
                    6.0,
                )
                .translate(
                    centered_index(token, DISPOSITION_TOKENS_PER_LANE, 54.0),
                    y,
                    DISPOSITION_Z + 3.0,
                );
        }
    }
    tokens
}

fn disposition_gate_posts() -> Part {
    let mut posts = Part::empty(format!("{OUTPUT_PREFIX}_release_hold_reject_gate_posts"));
    for lane in 0..DISPOSITION_LANES {
        let y = centered_index(lane, DISPOSITION_LANES, 44.0);
        for x in [-DISPOSITION_X / 2.0 + 26.0, DISPOSITION_X / 2.0 - 26.0] {
            posts = posts
                + centered_cylinder(
                    format!("{OUTPUT_PREFIX}_disposition_lane_{lane}_gate_post_{x:.0}"),
                    6.0,
                    20.0,
                    20,
                )
                .translate(x, y, DISPOSITION_Z + 10.0);
        }
    }
    posts
}

fn barcode_custody_board() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_barcode_custody_board_body"),
        BARCODE_X,
        BARCODE_Y,
        BARCODE_Z,
    )
    .translate(0.0, 0.0, BARCODE_Z / 2.0);
    body - barcode_slot_cuts() + barcode_fiducials() + custody_chain_ticks()
}

fn barcode_slot_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_barcode_custody_slot_cuts"));
    for i in 0..BARCODE_SLOTS {
        cuts = cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_barcode_custody_slot_{i}_cut"),
                BARCODE_SLOT_X,
                BARCODE_SLOT_Y,
                6.0,
            )
            .translate(
                centered_index(i % 5, 5, 74.0),
                if i < 5 { -30.0 } else { 34.0 },
                BARCODE_Z - 3.0,
            );
    }
    cuts
}

fn barcode_fiducials() -> Part {
    let mut fiducials = Part::empty(format!("{OUTPUT_PREFIX}_barcode_scanner_fiducials"));
    for i in 0..BARCODE_FIDUCIALS {
        let x = if i % 2 == 0 {
            -BARCODE_X / 2.0 + 28.0
        } else {
            BARCODE_X / 2.0 - 28.0
        };
        let y = if i < 2 {
            -BARCODE_Y / 2.0 + 24.0
        } else {
            BARCODE_Y / 2.0 - 24.0
        };
        fiducials = fiducials
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_barcode_fiducial_{i}"),
                9.0,
                5.0,
                28,
            )
            .translate(x, y, BARCODE_Z + 2.5);
    }
    fiducials
}

fn custody_chain_ticks() -> Part {
    let mut ticks = Part::empty(format!("{OUTPUT_PREFIX}_barcode_custody_chain_ticks"));
    for i in 0..9 {
        ticks = ticks
            + centered_cube(
                format!("{OUTPUT_PREFIX}_custody_chain_arrow_tick_{i}"),
                22.0,
                5.0,
                4.0,
            )
            .translate(centered_index(i, 9, 38.0), 0.0, BARCODE_Z + 2.0);
    }
    ticks
}

fn robot_service_keepouts() -> Part {
    let front = centered_cube(
        format!("{OUTPUT_PREFIX}_front_robot_keepout_rail_{FRONT_ROBOT_CLEARANCE:.0}mm"),
        KEEP_OUT_X,
        8.0,
        KEEP_OUT_RAIL_Z,
    )
    .translate(0.0, -KEEP_OUT_Y / 2.0, DECK_Z + KEEP_OUT_RAIL_Z / 2.0);
    let rear = centered_cube(
        format!("{OUTPUT_PREFIX}_rear_service_keepout_rail_{REAR_SERVICE_CLEARANCE:.0}mm"),
        KEEP_OUT_X,
        8.0,
        KEEP_OUT_RAIL_Z,
    )
    .translate(0.0, KEEP_OUT_Y / 2.0, DECK_Z + KEEP_OUT_RAIL_Z / 2.0);
    let left = centered_cube(
        format!(
            "{OUTPUT_PREFIX}_left_source_service_keepout_rail_{LEFT_SOURCE_SERVICE_CLEARANCE:.0}mm"
        ),
        8.0,
        KEEP_OUT_Y,
        KEEP_OUT_RAIL_Z,
    )
    .translate(-KEEP_OUT_X / 2.0, 0.0, DECK_Z + KEEP_OUT_RAIL_Z / 2.0);
    let right = centered_cube(
        format!("{OUTPUT_PREFIX}_right_sampling_service_keepout_rail_{RIGHT_SAMPLING_SERVICE_CLEARANCE:.0}mm"),
        8.0,
        KEEP_OUT_Y,
        KEEP_OUT_RAIL_Z,
    )
    .translate(KEEP_OUT_X / 2.0, 0.0, DECK_Z + KEEP_OUT_RAIL_Z / 2.0);
    let top_gauge = centered_cube(
        format!("{OUTPUT_PREFIX}_top_mixer_lift_keepout_gauge_{TOP_MIXER_LIFT_CLEARANCE:.0}mm"),
        150.0,
        18.0,
        18.0,
    )
    .translate(
        MIXER_CENTER.0,
        MIXER_CENTER.1,
        DECK_Z + TOP_MIXER_LIFT_CLEARANCE,
    );

    front + rear + left + right + top_gauge + robot_approach_pads() + service_pull_tabs()
}

fn robot_approach_pads() -> Part {
    let mut pads = Part::empty(format!("{OUTPUT_PREFIX}_robot_approach_pads"));
    for i in 0..5 {
        pads = pads
            + centered_cube(
                format!("{OUTPUT_PREFIX}_robot_front_approach_pad_{i}"),
                86.0,
                28.0,
                5.0,
            )
            .translate(
                centered_index(i, 5, 172.0),
                -KEEP_OUT_Y / 2.0 + 42.0,
                DECK_Z + 2.5,
            );
    }
    pads
}

fn service_pull_tabs() -> Part {
    let mut tabs = Part::empty(format!("{OUTPUT_PREFIX}_service_pull_clearance_tabs"));
    for i in 0..4 {
        tabs = tabs
            + centered_cube(
                format!("{OUTPUT_PREFIX}_rear_service_pull_tab_{i}"),
                118.0,
                22.0,
                5.0,
            )
            .translate(
                centered_index(i, 4, 220.0),
                KEEP_OUT_Y / 2.0 - 40.0,
                DECK_Z + 2.5,
            );
    }
    tabs
}

fn settling_reference_tokens() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_settling_reference_token_board"),
        REFERENCE_X,
        REFERENCE_Y,
        REFERENCE_Z,
    )
    .translate(0.0, 0.0, REFERENCE_Z / 2.0);
    body + reference_token_posts() + reference_gradient_bars()
}

fn reference_token_posts() -> Part {
    let mut posts = Part::empty(format!("{OUTPUT_PREFIX}_settling_reference_token_posts"));
    for i in 0..REFERENCE_TOKENS {
        posts = posts
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_settling_reference_token_{i}"),
                REFERENCE_TOKEN_D / 2.0,
                8.0,
                28,
            )
            .translate(
                centered_index(i % 3, 3, 76.0),
                centered_index(i / 3, 3, 42.0),
                REFERENCE_Z + 4.0,
            );
    }
    posts
}

fn reference_gradient_bars() -> Part {
    let mut bars = Part::empty(format!("{OUTPUT_PREFIX}_settling_density_gradient_bars"));
    for i in 0..6 {
        bars = bars
            + centered_cube(
                format!("{OUTPUT_PREFIX}_settling_gradient_reference_bar_{i}"),
                36.0 + i as f64 * 18.0,
                8.0,
                4.0,
            )
            .translate(
                -REFERENCE_X / 2.0 + 46.0 + i as f64 * 42.0,
                -REFERENCE_Y / 2.0 + 22.0,
                REFERENCE_Z + 2.0,
            );
    }
    bars
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn output_paths_are_unique_and_named_for_station() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        for output in OUTPUTS {
            assert!(output.starts_with("output/"));
            assert!(output.ends_with(".stl"));
            assert!(
                output.contains(OUTPUT_PREFIX),
                "output path must include station prefix: {output}"
            );
        }
    }

    #[test]
    fn required_design_cues_are_exported() {
        for feature in REQUIRED_FEATURES {
            assert!(
                OUTPUTS.iter().any(|output| output.contains(feature)),
                "missing output for required feature {feature}"
            );
        }
    }

    #[test]
    fn design_constraints_hold() {
        assert_design_constraints();
    }

    #[test]
    fn settling_sampling_and_disposition_counts_are_stable() {
        assert_eq!(VIAL_NESTS, 4);
        assert_eq!(SAMPLE_LEVELS * SAMPLE_LOOPS_PER_LEVEL, 12);
        assert_eq!(DENSITY_WINDOWS, 12);
        assert_eq!(TEMP_POCKETS, 3);
        assert_eq!(DISPOSITION_LANES * DISPOSITION_TOKENS_PER_LANE, 12);
        assert_eq!(SETTLING_MINUTES, [0, 5, 10, 15, 20, 30, 45, 60]);
    }
}
