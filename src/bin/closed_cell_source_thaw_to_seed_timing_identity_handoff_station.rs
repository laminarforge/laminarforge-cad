use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed cell-source thaw-to-seed timing and identity handoff validation station.
//
// Intent:
// - Validate the closed handoff from thawed cell source to automated multi-chip
//   seeding inside an isolator/cabinet workcell.
// - Make thaw-to-seed timing, source lot identity, temperature exposure,
//   gentle-mix completion, density/viability sample timing, bubble/dead-volume
//   inspection, aliquot custody, and release/hold/reject disposition visible to
//   automation and batch records.
// - Model mechanical envelopes, custody lands, witness pockets, and keepout
//   gauges only. Biological acceptance criteria, seeding recipes, aseptic
//   claims, and GMP release decisions remain external validation controls.

const OUTPUT_PREFIX: &str = "closed_cell_source_thaw_to_seed_timing_identity_handoff_station";

const OUTPUTS: [&str; 14] = [
    "output/closed_cell_source_thaw_to_seed_timing_identity_handoff_station_containment_deck.stl",
    "output/closed_cell_source_thaw_to_seed_timing_identity_handoff_station_source_vial_bag_nest_array.stl",
    "output/closed_cell_source_thaw_to_seed_timing_identity_handoff_station_thaw_to_seed_token_rail.stl",
    "output/closed_cell_source_thaw_to_seed_timing_identity_handoff_station_temperature_logger_pocket_bank.stl",
    "output/closed_cell_source_thaw_to_seed_timing_identity_handoff_station_identity_barcode_rfid_lands.stl",
    "output/closed_cell_source_thaw_to_seed_timing_identity_handoff_station_gentle_mix_witness_module.stl",
    "output/closed_cell_source_thaw_to_seed_timing_identity_handoff_station_density_viability_sample_loop_surrogate.stl",
    "output/closed_cell_source_thaw_to_seed_timing_identity_handoff_station_sterile_connector_handoff_bulkhead.stl",
    "output/closed_cell_source_thaw_to_seed_timing_identity_handoff_station_bubble_dead_volume_window_lanes.stl",
    "output/closed_cell_source_thaw_to_seed_timing_identity_handoff_station_first_last_aliquot_custody_wells.stl",
    "output/closed_cell_source_thaw_to_seed_timing_identity_handoff_station_release_hold_reject_disposition_gate.stl",
    "output/closed_cell_source_thaw_to_seed_timing_identity_handoff_station_camera_fiducial_bridge.stl",
    "output/closed_cell_source_thaw_to_seed_timing_identity_handoff_station_robot_service_keepouts.stl",
    "output/closed_cell_source_thaw_to_seed_timing_identity_handoff_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 14] = [
    "containment_deck",
    "source_vial_bag_nests",
    "timed_thaw_to_seed_token_rail",
    "temperature_logger_pockets",
    "identity_barcode_rfid_lands",
    "gentle_mix_witness",
    "density_viability_sample_loop_surrogate",
    "sterile_connector_handoff_bulkhead",
    "bubble_dead_volume_windows",
    "first_last_aliquot_custody_wells",
    "release_hold_reject_disposition",
    "camera_fiducials",
    "robot_service_keepouts",
    "named_stl_outputs",
];

const STATION_X: f64 = 1620.0;
const STATION_Y: f64 = 980.0;
const DECK_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const BASIN_DEPTH: f64 = 7.0;
const DRAIN_PORT_D: f64 = 16.0;
const MOUNT_BOSS_D: f64 = 30.0;
const MOUNT_HOLE_D: f64 = 6.6;

const CHIP_LANES: usize = 12;
const CASSETTES_PER_BATCH: usize = 4;
const SOURCE_SLOTS: usize = 6;
const SOURCE_ROWS: usize = 2;
const SOURCE_COLS: usize = 3;
const CHIPS_PER_SOURCE: usize = CHIP_LANES / SOURCE_SLOTS;

const SOURCE_CENTER: (f64, f64) = (-500.0, 205.0);
const SOURCE_X: f64 = 500.0;
const SOURCE_Y: f64 = 280.0;
const SOURCE_Z: f64 = 58.0;
const SOURCE_PITCH_X: f64 = 150.0;
const SOURCE_PITCH_Y: f64 = 112.0;
const VIAL_WELL_D: f64 = 35.0;
const VIAL_WELL_DEPTH: f64 = 38.0;
const BAG_SADDLE_X: f64 = 96.0;
const BAG_SADDLE_Y: f64 = 44.0;
const BAG_SADDLE_DEPTH: f64 = 12.0;
const SOURCE_COLLAR_D: f64 = 55.0;
const SOURCE_COLLAR_Z: f64 = 12.0;

const TIMING_CENTER: (f64, f64) = (220.0, 392.0);
const TIMING_X: f64 = 860.0;
const TIMING_Y: f64 = 112.0;
const TIMING_Z: f64 = 28.0;
const TIMING_EVENT_COUNT: usize = 5;
const TIMING_EVENTS: [&str; TIMING_EVENT_COUNT] = [
    "thaw_end",
    "mix_done",
    "sample_draw",
    "qc_result",
    "seed_start",
];
const MAX_THAW_TO_SEED_MIN: usize = 90;
const MAX_MIX_TO_SEED_MIN: usize = 20;
const TOKEN_SLOT_X: f64 = 98.0;
const TOKEN_SLOT_Y: f64 = 12.0;
const TOKEN_SLOT_DEPTH: f64 = 7.0;
const TIMING_EVENT_PITCH_X: f64 = 150.0;
const TIMING_SOURCE_PITCH_Y: f64 = 15.0;
const TIMING_TOKEN_COUNT: usize = SOURCE_SLOTS * TIMING_EVENT_COUNT;

const TEMP_CENTER: (f64, f64) = (520.0, 180.0);
const TEMP_X: f64 = 360.0;
const TEMP_Y: f64 = 250.0;
const TEMP_Z: f64 = 42.0;
const TEMP_LOGGER_POCKETS: usize = SOURCE_SLOTS + 2;
const TEMP_POCKET_X: f64 = 70.0;
const TEMP_POCKET_Y: f64 = 28.0;
const TEMP_POCKET_DEPTH: f64 = 12.0;
const TEMP_POCKET_PITCH_X: f64 = 84.0;
const TEMP_POCKET_PITCH_Y: f64 = 62.0;
const TEMP_CABLE_CHANNEL_D: f64 = 5.0;

const ID_CENTER: (f64, f64) = (520.0, -95.0);
const ID_X: f64 = 360.0;
const ID_Y: f64 = 170.0;
const ID_Z: f64 = 24.0;
const BARCODE_LANDS: usize = CHIP_LANES + SOURCE_SLOTS;
const RFID_LANDS: usize = SOURCE_SLOTS;
const LOT_CARD_SLOTS: usize = SOURCE_SLOTS;
const BARCODE_LAND_X: f64 = 42.0;
const BARCODE_LAND_Y: f64 = 14.0;
const RFID_LAND_D: f64 = 22.0;

const MIX_CENTER: (f64, f64) = (-520.0, -120.0);
const MIX_X: f64 = 470.0;
const MIX_Y: f64 = 210.0;
const MIX_Z: f64 = 48.0;
const MIX_WITNESS_LANES: usize = SOURCE_SLOTS;
const MIX_ROLLER_COUNT: usize = 2;
const MIX_BEAD_WINDOWS_PER_SOURCE: usize = 3;
const MIX_ENDPOINT_WITNESSES: usize = SOURCE_SLOTS * 2;
const MAX_MIX_RPM: f64 = 8.0;
const MAX_MIX_DELAY_MIN: usize = 15;

const SAMPLE_CENTER: (f64, f64) = (-35.0, -135.0);
const SAMPLE_X: f64 = 430.0;
const SAMPLE_Y: f64 = 230.0;
const SAMPLE_Z: f64 = 50.0;
const SAMPLE_LOOP_CHANNELS: usize = 4;
const SAMPLE_LOOP_UL: f64 = 80.0;
const SAMPLE_LOOP_TUBE_D: f64 = 7.2;
const SAMPLE_BRANCH_TUBE_D: f64 = 4.8;
const VIABILITY_CASSETTE_DOCKS: usize = 4;
const SAMPLE_TIMING_TOKENS: usize = SOURCE_SLOTS;

const BULKHEAD_CENTER: (f64, f64) = (320.0, -325.0);
const BULKHEAD_X: f64 = 520.0;
const BULKHEAD_Y: f64 = 110.0;
const BULKHEAD_Z: f64 = 72.0;
const HANDOFF_PORTS: usize = CHIP_LANES;
const HANDOFF_PORT_D: f64 = 22.0;
const HANDOFF_PORT_PITCH_X: f64 = 38.0;
const SOURCE_INLET_PORTS: usize = SOURCE_SLOTS;
const STERILE_CAP_PARKS: usize = HANDOFF_PORTS;

const BUBBLE_CENTER: (f64, f64) = (-350.0, -360.0);
const BUBBLE_X: f64 = 500.0;
const BUBBLE_Y: f64 = 100.0;
const BUBBLE_Z: f64 = 24.0;
const BUBBLE_WINDOWS: usize = CHIP_LANES;
const DEAD_VOLUME_WINDOWS: usize = CHIP_LANES;
const WINDOW_X: f64 = 28.0;
const WINDOW_Y: f64 = 18.0;
const WINDOW_PITCH_X: f64 = 38.0;

const ALIQUOT_CENTER: (f64, f64) = (-700.0, -365.0);
const ALIQUOT_X: f64 = 150.0;
const ALIQUOT_Y: f64 = 100.0;
const ALIQUOT_Z: f64 = 34.0;
const ALIQUOT_WELLS: usize = CHIP_LANES * 2;
const ALIQUOT_WELL_D: f64 = 7.0;
const ALIQUOT_WELL_DEPTH: f64 = 22.0;
const ALIQUOT_WELL_PITCH_X: f64 = 11.0;
const ALIQUOT_ROW_PITCH_Y: f64 = 38.0;

const DISPOSITION_CENTER: (f64, f64) = (690.0, -300.0);
const DISPOSITION_X: f64 = 170.0;
const DISPOSITION_Y: f64 = 240.0;
const DISPOSITION_Z: f64 = 44.0;
const DISPOSITION_LANES: [&str; 3] = ["release", "hold", "reject"];
const DISPOSITION_SLOTS_PER_LANE: usize = SOURCE_SLOTS;
const DISPOSITION_SLOT_X: f64 = 16.0;
const DISPOSITION_SLOT_Y: f64 = 26.0;
const DISPOSITION_SLOT_DEPTH: f64 = 8.0;
const DISPOSITION_SLOT_PITCH_X: f64 = 22.0;
const DISPOSITION_LANE_PITCH_Y: f64 = 70.0;

const BRIDGE_SPAN_X: f64 = 1480.0;
const BRIDGE_POST_X: f64 = 30.0;
const BRIDGE_POST_Y: f64 = 42.0;
const BRIDGE_CLEARANCE_Z: f64 = 238.0;
const BRIDGE_BEAM_Z: f64 = 30.0;
const CAMERA_PODS: usize = 5;
const FIDUCIAL_PADS: usize = 6;
const LIGHT_BARS: usize = 3;

const ROBOT_FRONT_CLEARANCE_Y: f64 = 360.0;
const SERVICE_REAR_CLEARANCE_Y: f64 = 230.0;
const LEFT_SERVICE_CLEARANCE_X: f64 = 210.0;
const RIGHT_SEEDER_CLEARANCE_X: f64 = 260.0;
const OVERHEAD_SERVICE_CLEARANCE_Z: f64 = 320.0;
const KEEP_OUT_GAUGES: usize = 5;
const KEEP_OUT_Z: f64 = 6.0;

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

    let deck = containment_deck();
    export(OUTPUTS[0], &deck);

    let source = source_vial_bag_nest_array();
    export(OUTPUTS[1], &source);

    let timing = thaw_to_seed_token_rail();
    export(OUTPUTS[2], &timing);

    let temp = temperature_logger_pocket_bank();
    export(OUTPUTS[3], &temp);

    let identity = identity_barcode_rfid_lands();
    export(OUTPUTS[4], &identity);

    let mix = gentle_mix_witness_module();
    export(OUTPUTS[5], &mix);

    let sample = density_viability_sample_loop_surrogate();
    export(OUTPUTS[6], &sample);

    let bulkhead = sterile_connector_handoff_bulkhead();
    export(OUTPUTS[7], &bulkhead);

    let windows = bubble_dead_volume_window_lanes();
    export(OUTPUTS[8], &windows);

    let aliquots = first_last_aliquot_custody_wells();
    export(OUTPUTS[9], &aliquots);

    let disposition = release_hold_reject_disposition_gate();
    export(OUTPUTS[10], &disposition);

    let bridge = camera_fiducial_bridge();
    export(OUTPUTS[11], &bridge);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[12], &keepouts);

    let assembly = deck
        + source
        + timing
        + temp
        + identity
        + mix
        + sample
        + bulkhead
        + windows
        + aliquots
        + disposition
        + bridge
        + keepouts;
    export(OUTPUTS[13], &assembly);

    println!();
    println!("Closed cell-source thaw-to-seed timing and identity handoff station:");
    println!(
        "  Deck:                    {STATION_X:.0}mm x {STATION_Y:.0}mm containment deck with {RIM_Z:.0}mm rim and {DRAIN_PORT_D:.0}mm drain"
    );
    println!(
        "  Multi-chip scale:        {SOURCE_SLOTS} source vial/bag nests feeding {CHIP_LANES} chip lanes ({CHIPS_PER_SOURCE} lanes per source)"
    );
    println!(
        "  Timing controls:         {TIMING_TOKEN_COUNT} thaw-to-seed event token slots across {:?}, max window {MAX_THAW_TO_SEED_MIN} min",
        TIMING_EVENTS
    );
    println!(
        "  Identity controls:       {BARCODE_LANDS} barcode lands, {RFID_LANDS} RFID lands, {LOT_CARD_SLOTS} lot-card slots, {SOURCE_INLET_PORTS} source inlet ports"
    );
    println!(
        "  Witness controls:        {TEMP_LOGGER_POCKETS} temperature logger pockets, {MIX_WITNESS_LANES} mix witness lanes, {SAMPLE_LOOP_CHANNELS} sample-loop channels, {BUBBLE_WINDOWS} bubble windows"
    );
    println!(
        "  Custody and disposition: {ALIQUOT_WELLS} first/last aliquot wells and {} release/hold/reject lanes",
        DISPOSITION_LANES.len()
    );
    println!(
        "  Evidence and keepouts:   {CAMERA_PODS} camera pods, {FIDUCIAL_PADS} fiducial pads, {KEEP_OUT_GAUGES} robot/service keepout gauges"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 14);
    assert_eq!(REQUIRED_FEATURES.len(), 14);
    assert_eq!(SOURCE_SLOTS, SOURCE_ROWS * SOURCE_COLS);
    assert_eq!(CHIP_LANES % SOURCE_SLOTS, 0);
    assert_eq!(CHIP_LANES / SOURCE_SLOTS, CHIPS_PER_SOURCE);
    assert_eq!(CHIP_LANES % CASSETTES_PER_BATCH, 0);
    assert_eq!(TIMING_EVENTS.len(), TIMING_EVENT_COUNT);
    assert_eq!(TIMING_TOKEN_COUNT, SOURCE_SLOTS * TIMING_EVENT_COUNT);
    assert!(MAX_THAW_TO_SEED_MIN <= 90);
    assert!(MAX_MIX_TO_SEED_MIN <= 20);
    assert!(MAX_MIX_DELAY_MIN <= 15);
    assert!(TEMP_LOGGER_POCKETS >= SOURCE_SLOTS + 2);
    assert_eq!(MIX_ENDPOINT_WITNESSES, SOURCE_SLOTS * 2);
    assert!(MAX_MIX_RPM <= 8.0);
    assert!(SAMPLE_LOOP_UL <= 100.0);
    assert!(BARCODE_LANDS >= CHIP_LANES + SOURCE_SLOTS);
    assert!(RFID_LANDS >= SOURCE_SLOTS);
    assert_eq!(HANDOFF_PORTS, CHIP_LANES);
    assert_eq!(SOURCE_INLET_PORTS, SOURCE_SLOTS);
    assert_eq!(BUBBLE_WINDOWS, CHIP_LANES);
    assert_eq!(DEAD_VOLUME_WINDOWS, CHIP_LANES);
    assert_eq!(ALIQUOT_WELLS, CHIP_LANES * 2);
    assert_eq!(DISPOSITION_LANES, ["release", "hold", "reject"]);
    assert_eq!(DISPOSITION_SLOTS_PER_LANE, SOURCE_SLOTS);
    assert!(BRIDGE_CLEARANCE_Z > SOURCE_Z + DECK_Z + 120.0);
    assert!(OVERHEAD_SERVICE_CLEARANCE_Z > BRIDGE_CLEARANCE_Z + BRIDGE_BEAM_Z);
    assert!(ROBOT_FRONT_CLEARANCE_Y >= 350.0);
    assert!(SERVICE_REAR_CLEARANCE_Y >= 220.0);
    assert!(LEFT_SERVICE_CLEARANCE_X >= 200.0);
    assert!(RIGHT_SEEDER_CLEARANCE_X >= 250.0);

    let rects = layout_rects();
    for rect in rects {
        assert!(
            rect.fits_inside_station(),
            "{} must fit inside the station deck",
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
}

fn layout_rects() -> [Rect; 10] {
    [
        rect(
            "source_vial_bag_nest_array",
            SOURCE_CENTER,
            SOURCE_X,
            SOURCE_Y,
        ),
        rect("thaw_to_seed_token_rail", TIMING_CENTER, TIMING_X, TIMING_Y),
        rect(
            "temperature_logger_pocket_bank",
            TEMP_CENTER,
            TEMP_X,
            TEMP_Y,
        ),
        rect("identity_barcode_rfid_lands", ID_CENTER, ID_X, ID_Y),
        rect("gentle_mix_witness_module", MIX_CENTER, MIX_X, MIX_Y),
        rect(
            "density_viability_sample_loop_surrogate",
            SAMPLE_CENTER,
            SAMPLE_X,
            SAMPLE_Y,
        ),
        rect(
            "sterile_connector_handoff_bulkhead",
            BULKHEAD_CENTER,
            BULKHEAD_X,
            BULKHEAD_Y,
        ),
        rect(
            "bubble_dead_volume_window_lanes",
            BUBBLE_CENTER,
            BUBBLE_X,
            BUBBLE_Y,
        ),
        rect(
            "first_last_aliquot_custody_wells",
            ALIQUOT_CENTER,
            ALIQUOT_X,
            ALIQUOT_Y,
        ),
        rect(
            "release_hold_reject_disposition_gate",
            DISPOSITION_CENTER,
            DISPOSITION_X,
            DISPOSITION_Y,
        ),
    ]
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        format!("{OUTPUT_PREFIX}_containment_deck_plate"),
        STATION_X,
        STATION_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);

    let basin = centered_cube(
        format!("{OUTPUT_PREFIX}_shallow_spill_basin_cut"),
        STATION_X - 150.0,
        STATION_Y - 150.0,
        BASIN_DEPTH + 1.0,
    )
    .translate(0.0, -10.0, DECK_Z - BASIN_DEPTH / 2.0 + 0.5);

    let drain = centered_cylinder(
        format!("{OUTPUT_PREFIX}_front_right_drain_port_cut"),
        DRAIN_PORT_D / 2.0,
        RIM_W + 48.0,
        40,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 125.0,
        -STATION_Y / 2.0 + 18.0,
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
            28,
        )
        .translate(*x, *y, DECK_Z + 5.0);
        bosses = bosses + (boss - hole);
    }
    bosses
}

fn zone_floor_markers() -> Part {
    let mut markers = Part::empty(format!("{OUTPUT_PREFIX}_validation_zone_floor_markers"));
    for rect in layout_rects() {
        markers = markers
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{}_floor_marker", rect.name),
                rect.x + 18.0,
                rect.y + 16.0,
                2.2,
            )
            .translate(rect.center.0, rect.center.1, DECK_Z + 1.1);
    }
    markers
}

fn source_vial_bag_nest_array() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_source_vial_bag_nest_plate"),
        SOURCE_X,
        SOURCE_Y,
        SOURCE_Z,
    );

    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_source_nest_cuts"));
    for index in 0..SOURCE_SLOTS {
        let (x, y) = local_source_xy(index);
        let vial_well = centered_cylinder(
            format!("{OUTPUT_PREFIX}_source_{index}_vial_well_cut"),
            VIAL_WELL_D / 2.0,
            VIAL_WELL_DEPTH + 2.0,
            48,
        )
        .translate(x - 28.0, y, SOURCE_Z / 2.0 - VIAL_WELL_DEPTH / 2.0 + 0.5);
        let bag_saddle = centered_cube(
            format!("{OUTPUT_PREFIX}_source_{index}_bag_saddle_relief_cut"),
            BAG_SADDLE_X,
            BAG_SADDLE_Y,
            BAG_SADDLE_DEPTH + 1.0,
        )
        .translate(x + 30.0, y, SOURCE_Z / 2.0 - BAG_SADDLE_DEPTH / 2.0 + 0.5);
        let clocking_key = centered_cube(
            format!("{OUTPUT_PREFIX}_source_{index}_vial_clocking_key_cut"),
            12.0,
            42.0,
            VIAL_WELL_DEPTH + 2.0,
        )
        .translate(
            x - 28.0 + VIAL_WELL_D / 2.0 - 3.0,
            y,
            SOURCE_Z / 2.0 - VIAL_WELL_DEPTH / 2.0 + 0.5,
        );
        cuts = cuts + vial_well + bag_saddle + clocking_key;
    }

    let plate = (body - cuts).translate(SOURCE_CENTER.0, SOURCE_CENTER.1, deck_insert_z(SOURCE_Z));

    plate + source_retainer_collars() + bag_edge_clamps() + source_lot_token_posts()
}

fn source_retainer_collars() -> Part {
    let mut collars = Part::empty(format!("{OUTPUT_PREFIX}_source_retainer_collars"));
    for index in 0..SOURCE_SLOTS {
        let (x, y) = world_source_xy(index);
        let outer = centered_cylinder(
            format!("{OUTPUT_PREFIX}_source_{index}_vial_retainer_collar"),
            SOURCE_COLLAR_D / 2.0,
            SOURCE_COLLAR_Z,
            48,
        )
        .translate(x - 28.0, y, DECK_Z + SOURCE_Z + SOURCE_COLLAR_Z / 2.0);
        let inner = centered_cylinder(
            format!("{OUTPUT_PREFIX}_source_{index}_vial_collar_clearance"),
            (VIAL_WELL_D + 5.0) / 2.0,
            SOURCE_COLLAR_Z + 2.0,
            48,
        )
        .translate(x - 28.0, y, DECK_Z + SOURCE_Z + SOURCE_COLLAR_Z / 2.0);
        let bag_datum = centered_cube(
            format!("{OUTPUT_PREFIX}_source_{index}_bag_identity_datum_tab"),
            44.0,
            8.0,
            10.0,
        )
        .translate(
            x + 56.0,
            y - BAG_SADDLE_Y / 2.0 - 9.0,
            DECK_Z + SOURCE_Z + 5.0,
        );
        collars = collars + (outer - inner) + bag_datum;
    }
    collars
}

fn bag_edge_clamps() -> Part {
    let mut clamps = Part::empty(format!("{OUTPUT_PREFIX}_closed_bag_edge_clamps"));
    for index in 0..SOURCE_SLOTS {
        let (x, y) = world_source_xy(index);
        let left = centered_cube(
            format!("{OUTPUT_PREFIX}_source_{index}_bag_left_clamp_lip"),
            8.0,
            BAG_SADDLE_Y + 24.0,
            18.0,
        )
        .translate(
            x + 30.0 - BAG_SADDLE_X / 2.0 - 8.0,
            y,
            DECK_Z + SOURCE_Z + 9.0,
        );
        let right = centered_cube(
            format!("{OUTPUT_PREFIX}_source_{index}_bag_right_clamp_lip"),
            8.0,
            BAG_SADDLE_Y + 24.0,
            18.0,
        )
        .translate(
            x + 30.0 + BAG_SADDLE_X / 2.0 + 8.0,
            y,
            DECK_Z + SOURCE_Z + 9.0,
        );
        clamps = clamps + left + right;
    }
    clamps
}

fn source_lot_token_posts() -> Part {
    let mut posts = Part::empty(format!("{OUTPUT_PREFIX}_source_lot_chain_token_posts"));
    for index in 0..SOURCE_SLOTS {
        let (x, y) = world_source_xy(index);
        posts = posts
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_source_{index}_lot_token_post"),
                5.0,
                24.0,
                20,
            )
            .translate(x - 76.0, y + 38.0, DECK_Z + SOURCE_Z + 12.0)
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_source_{index}_batch_record_token_post"),
                5.0,
                24.0,
                20,
            )
            .translate(x - 76.0, y - 38.0, DECK_Z + SOURCE_Z + 12.0);
    }
    posts
}

fn thaw_to_seed_token_rail() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_thaw_to_seed_token_rail_body"),
        TIMING_X,
        TIMING_Y,
        TIMING_Z,
    );

    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_timing_token_slot_cuts"));
    for source in 0..SOURCE_SLOTS {
        for event in 0..TIMING_EVENT_COUNT {
            let (x, y) = local_timing_slot_xy(source, event);
            let slot = centered_cube(
                format!(
                    "{OUTPUT_PREFIX}_source_{source}_{}_token_slot_cut",
                    TIMING_EVENTS[event]
                ),
                TOKEN_SLOT_X,
                TOKEN_SLOT_Y,
                TOKEN_SLOT_DEPTH + 1.0,
            )
            .translate(x, y, TIMING_Z / 2.0 - TOKEN_SLOT_DEPTH / 2.0 + 0.5);
            let finger_pull = centered_cube(
                format!(
                    "{OUTPUT_PREFIX}_source_{source}_{}_token_pull_cut",
                    TIMING_EVENTS[event]
                ),
                16.0,
                TOKEN_SLOT_Y + 16.0,
                TOKEN_SLOT_DEPTH + 2.0,
            )
            .translate(x, y, TIMING_Z / 2.0 - TOKEN_SLOT_DEPTH / 2.0 + 0.5);
            cuts = cuts + slot + finger_pull;
        }
    }

    let rail = (body - cuts).translate(TIMING_CENTER.0, TIMING_CENTER.1, deck_insert_z(TIMING_Z));

    rail + timing_event_hard_stops() + timing_drift_gauge()
}

fn timing_event_hard_stops() -> Part {
    let mut stops = Part::empty(format!("{OUTPUT_PREFIX}_timing_event_hard_stops"));
    for event in 0..TIMING_EVENT_COUNT {
        let x = TIMING_CENTER.0 + centered_index(event, TIMING_EVENT_COUNT, TIMING_EVENT_PITCH_X);
        let front_stop = centered_cube(
            format!("{OUTPUT_PREFIX}_{}_front_hard_stop", TIMING_EVENTS[event]),
            112.0,
            6.0,
            16.0,
        )
        .translate(
            x,
            TIMING_CENTER.1 - TIMING_Y / 2.0 - 7.0,
            DECK_Z + TIMING_Z + 8.0,
        );
        let rear_stop = centered_cube(
            format!("{OUTPUT_PREFIX}_{}_rear_hard_stop", TIMING_EVENTS[event]),
            112.0,
            6.0,
            16.0,
        )
        .translate(
            x,
            TIMING_CENTER.1 + TIMING_Y / 2.0 + 7.0,
            DECK_Z + TIMING_Z + 8.0,
        );
        stops = stops + front_stop + rear_stop;
    }
    stops
}

fn timing_drift_gauge() -> Part {
    let gauge = centered_cube(
        format!("{OUTPUT_PREFIX}_max_thaw_to_seed_drift_gauge_bar"),
        TIMING_X - 90.0,
        10.0,
        16.0,
    )
    .translate(
        TIMING_CENTER.0,
        TIMING_CENTER.1 + TIMING_Y / 2.0 + 28.0,
        DECK_Z + TIMING_Z + 8.0,
    );

    let mut index_posts = Part::empty(format!("{OUTPUT_PREFIX}_timing_drift_index_posts"));
    for source in 0..SOURCE_SLOTS {
        let x = TIMING_CENTER.0 - TIMING_X / 2.0 + 54.0 + source as f64 * 54.0;
        let post_height = 12.0 + source as f64 * 1.5;
        index_posts = index_posts
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_source_{source}_timing_drift_index_post"),
                5.0,
                post_height,
                20,
            )
            .translate(
                x,
                TIMING_CENTER.1 + TIMING_Y / 2.0 + 28.0,
                DECK_Z + TIMING_Z + post_height / 2.0,
            );
    }

    gauge + index_posts
}

fn temperature_logger_pocket_bank() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_temperature_logger_pocket_panel"),
        TEMP_X,
        TEMP_Y,
        TEMP_Z,
    );

    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_temperature_logger_pocket_cuts"));
    for index in 0..TEMP_LOGGER_POCKETS {
        let (x, y) = local_temp_pocket_xy(index);
        let pocket = centered_cube(
            format!("{OUTPUT_PREFIX}_temperature_logger_{index}_pocket_cut"),
            TEMP_POCKET_X,
            TEMP_POCKET_Y,
            TEMP_POCKET_DEPTH + 1.0,
        )
        .translate(x, y, TEMP_Z / 2.0 - TEMP_POCKET_DEPTH / 2.0 + 0.5);
        let lead_channel = centered_cylinder(
            format!("{OUTPUT_PREFIX}_temperature_logger_{index}_lead_channel_cut"),
            TEMP_CABLE_CHANNEL_D / 2.0,
            TEMP_Y / 2.0,
            20,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, y - 42.0, TEMP_Z / 2.0 - TEMP_POCKET_DEPTH + 2.0);
        cuts = cuts + pocket + lead_channel;
    }

    let panel = (body - cuts).translate(TEMP_CENTER.0, TEMP_CENTER.1, deck_insert_z(TEMP_Z));

    panel + temp_logger_retainer_clips() + temp_reference_blocks()
}

fn temp_logger_retainer_clips() -> Part {
    let mut clips = Part::empty(format!("{OUTPUT_PREFIX}_temperature_logger_retainer_clips"));
    for index in 0..TEMP_LOGGER_POCKETS {
        let (x, y) = world_temp_pocket_xy(index);
        let front = centered_cube(
            format!("{OUTPUT_PREFIX}_temperature_logger_{index}_front_clip"),
            TEMP_POCKET_X + 8.0,
            5.0,
            8.0,
        )
        .translate(x, y - TEMP_POCKET_Y / 2.0 - 5.0, DECK_Z + TEMP_Z + 4.0);
        let rear = centered_cube(
            format!("{OUTPUT_PREFIX}_temperature_logger_{index}_rear_clip"),
            TEMP_POCKET_X + 8.0,
            5.0,
            8.0,
        )
        .translate(x, y + TEMP_POCKET_Y / 2.0 + 5.0, DECK_Z + TEMP_Z + 4.0);
        clips = clips + front + rear;
    }
    clips
}

fn temp_reference_blocks() -> Part {
    let cold = centered_cube(
        format!("{OUTPUT_PREFIX}_cold_reference_logger_block"),
        54.0,
        38.0,
        20.0,
    )
    .translate(
        TEMP_CENTER.0 - TEMP_X / 2.0 + 46.0,
        TEMP_CENTER.1 - TEMP_Y / 2.0 + 36.0,
        DECK_Z + TEMP_Z + 10.0,
    );
    let warm = centered_cube(
        format!("{OUTPUT_PREFIX}_warm_reference_logger_block"),
        54.0,
        38.0,
        20.0,
    )
    .translate(
        TEMP_CENTER.0 + TEMP_X / 2.0 - 46.0,
        TEMP_CENTER.1 - TEMP_Y / 2.0 + 36.0,
        DECK_Z + TEMP_Z + 10.0,
    );

    cold + warm
}

fn identity_barcode_rfid_lands() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_identity_barcode_rfid_panel"),
        ID_X,
        ID_Y,
        ID_Z,
    )
    .translate(ID_CENTER.0, ID_CENTER.1, deck_insert_z(ID_Z));

    body + barcode_lands() + rfid_lands() + lot_card_slots() + chip_lane_map_tiles()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_barcode_lands"));
    for index in 0..BARCODE_LANDS {
        let row = index / 6;
        let col = index % 6;
        let x = ID_CENTER.0 + centered_index(col, 6, 52.0);
        let y = ID_CENTER.1 + 54.0 - row as f64 * 34.0;
        let land = centered_cube(
            format!("{OUTPUT_PREFIX}_barcode_land_{index}"),
            BARCODE_LAND_X,
            BARCODE_LAND_Y,
            5.0,
        )
        .translate(x, y, DECK_Z + ID_Z + 2.5);
        let scan_notch = centered_cube(
            format!("{OUTPUT_PREFIX}_barcode_land_{index}_scan_notch_cut"),
            6.0,
            BARCODE_LAND_Y + 2.0,
            6.0,
        )
        .translate(x - BARCODE_LAND_X / 2.0 + 8.0, y, DECK_Z + ID_Z + 2.5);
        lands = lands + (land - scan_notch);
    }
    lands
}

fn rfid_lands() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_rfid_lands"));
    for index in 0..RFID_LANDS {
        let x = ID_CENTER.0 + centered_index(index, RFID_LANDS, 45.0);
        let y = ID_CENTER.1 - ID_Y / 2.0 + 24.0;
        let pad = centered_cylinder(
            format!("{OUTPUT_PREFIX}_rfid_land_{index}"),
            RFID_LAND_D / 2.0,
            5.0,
            36,
        )
        .translate(x, y, DECK_Z + ID_Z + 2.5);
        let antenna_slot = centered_cube(
            format!("{OUTPUT_PREFIX}_rfid_land_{index}_orientation_notch"),
            5.0,
            RFID_LAND_D + 4.0,
            6.0,
        )
        .translate(x + RFID_LAND_D / 2.0 - 3.0, y, DECK_Z + ID_Z + 2.5);
        lands = lands + (pad - antenna_slot);
    }
    lands
}

fn lot_card_slots() -> Part {
    let mut slots = Part::empty(format!("{OUTPUT_PREFIX}_lot_card_slots"));
    for index in 0..LOT_CARD_SLOTS {
        let x = ID_CENTER.0 - ID_X / 2.0 + 38.0;
        let y = ID_CENTER.1 + centered_index(index, LOT_CARD_SLOTS, 22.0);
        slots = slots
            + centered_cube(
                format!("{OUTPUT_PREFIX}_source_{index}_lot_card_slot_backer"),
                46.0,
                14.0,
                7.0,
            )
            .translate(x, y, DECK_Z + ID_Z + 3.5);
    }
    slots
}

fn chip_lane_map_tiles() -> Part {
    let mut tiles = Part::empty(format!("{OUTPUT_PREFIX}_chip_lane_identity_map_tiles"));
    for lane in 0..CHIP_LANES {
        let x = ID_CENTER.0 + centered_index(lane % 6, 6, 38.0);
        let y = ID_CENTER.1 - 12.0 - (lane / 6) as f64 * 22.0;
        let tile = centered_cube(
            format!("{OUTPUT_PREFIX}_chip_lane_{lane}_identity_map_tile"),
            26.0,
            12.0,
            6.0,
        )
        .translate(x, y, DECK_Z + ID_Z + 3.0);
        tiles = tiles + tile;
    }
    tiles
}

fn gentle_mix_witness_module() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_gentle_mix_witness_base"),
        MIX_X,
        MIX_Y,
        MIX_Z,
    );

    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_gentle_mix_witness_cuts"));
    for lane in 0..MIX_WITNESS_LANES {
        let x = centered_index(lane, MIX_WITNESS_LANES, 66.0);
        let bag_relief = centered_cube(
            format!("{OUTPUT_PREFIX}_mix_lane_{lane}_bag_saddle_cut"),
            48.0,
            90.0,
            12.0,
        )
        .translate(x, 18.0, MIX_Z / 2.0 - 6.0);
        let endpoint_track = centered_cube(
            format!("{OUTPUT_PREFIX}_mix_lane_{lane}_endpoint_track_cut"),
            50.0,
            14.0,
            8.0,
        )
        .translate(x, -68.0, MIX_Z / 2.0 - 4.0);
        cuts = cuts + bag_relief + endpoint_track;
    }

    let module = (body - cuts).translate(MIX_CENTER.0, MIX_CENTER.1, deck_insert_z(MIX_Z));

    module + mix_roller_witnesses() + mix_bead_windows() + mix_delay_token_wells()
}

fn mix_roller_witnesses() -> Part {
    let mut rollers = Part::empty(format!("{OUTPUT_PREFIX}_gentle_mix_roller_witnesses"));
    for index in 0..MIX_ROLLER_COUNT {
        let y = MIX_CENTER.1 + if index == 0 { 46.0 } else { -16.0 };
        rollers = rollers
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_low_shear_mix_roller_{index}_envelope"),
                11.0,
                MIX_X - 84.0,
                32,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(MIX_CENTER.0, y, DECK_Z + MIX_Z + 12.0);
    }
    rollers
}

fn mix_bead_windows() -> Part {
    let mut windows = Part::empty(format!("{OUTPUT_PREFIX}_mix_bead_settling_windows"));
    for source in 0..SOURCE_SLOTS {
        for bead in 0..MIX_BEAD_WINDOWS_PER_SOURCE {
            let x = MIX_CENTER.0 + centered_index(source, SOURCE_SLOTS, 66.0);
            let y = MIX_CENTER.1 + 78.0 - bead as f64 * 16.0;
            windows = windows
                + centered_cube(
                    format!("{OUTPUT_PREFIX}_source_{source}_mix_bead_window_{bead}"),
                    30.0,
                    8.0,
                    5.0,
                )
                .translate(x, y, DECK_Z + MIX_Z + 2.5);
        }
    }
    windows
}

fn mix_delay_token_wells() -> Part {
    let mut wells = Part::empty(format!("{OUTPUT_PREFIX}_mix_delay_token_wells"));
    for source in 0..SOURCE_SLOTS {
        let x = MIX_CENTER.0 + centered_index(source, SOURCE_SLOTS, 66.0);
        let start = centered_cylinder(
            format!("{OUTPUT_PREFIX}_source_{source}_mix_start_token_well"),
            8.0,
            12.0,
            28,
        )
        .translate(x - 14.0, MIX_CENTER.1 - 68.0, DECK_Z + MIX_Z + 6.0);
        let end = centered_cylinder(
            format!("{OUTPUT_PREFIX}_source_{source}_mix_end_token_well"),
            8.0,
            12.0,
            28,
        )
        .translate(x + 14.0, MIX_CENTER.1 - 68.0, DECK_Z + MIX_Z + 6.0);
        wells = wells + start + end;
    }
    wells
}

fn density_viability_sample_loop_surrogate() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_density_viability_sample_loop_base"),
        SAMPLE_X,
        SAMPLE_Y,
        SAMPLE_Z,
    )
    .translate(SAMPLE_CENTER.0, SAMPLE_CENTER.1, deck_insert_z(SAMPLE_Z));

    body + sample_loop_routes()
        + metering_loop_bank()
        + viability_cassette_docks()
        + sample_timing_token_row()
}

fn sample_loop_routes() -> Part {
    let z = DECK_Z + SAMPLE_Z + 9.0;
    let top = centered_cylinder(
        format!("{OUTPUT_PREFIX}_sample_loop_top_return_route"),
        SAMPLE_LOOP_TUBE_D / 2.0,
        SAMPLE_X - 90.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(SAMPLE_CENTER.0, SAMPLE_CENTER.1 + 64.0, z);
    let bottom = centered_cylinder(
        format!("{OUTPUT_PREFIX}_sample_loop_bottom_draw_route"),
        SAMPLE_LOOP_TUBE_D / 2.0,
        SAMPLE_X - 90.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(SAMPLE_CENTER.0, SAMPLE_CENTER.1 - 64.0, z);
    let left = centered_cylinder(
        format!("{OUTPUT_PREFIX}_sample_loop_left_closed_return_leg"),
        SAMPLE_LOOP_TUBE_D / 2.0,
        128.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(SAMPLE_CENTER.0 - SAMPLE_X / 2.0 + 46.0, SAMPLE_CENTER.1, z);
    let right = centered_cylinder(
        format!("{OUTPUT_PREFIX}_sample_loop_right_closed_draw_leg"),
        SAMPLE_LOOP_TUBE_D / 2.0,
        128.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(SAMPLE_CENTER.0 + SAMPLE_X / 2.0 - 46.0, SAMPLE_CENTER.1, z);

    top + bottom + left + right
}

fn metering_loop_bank() -> Part {
    let mut bank = Part::empty(format!("{OUTPUT_PREFIX}_sample_metering_loop_bank"));
    for channel in 0..SAMPLE_LOOP_CHANNELS {
        let x = SAMPLE_CENTER.0 + centered_index(channel, SAMPLE_LOOP_CHANNELS, 70.0);
        let loop_pad = centered_cylinder(
            format!("{OUTPUT_PREFIX}_sample_channel_{channel}_metering_loop_surrogate"),
            22.0,
            6.0,
            48,
        )
        .translate(x, SAMPLE_CENTER.1, DECK_Z + SAMPLE_Z + 3.0);
        let branch = centered_cylinder(
            format!("{OUTPUT_PREFIX}_sample_channel_{channel}_viability_branch_route"),
            SAMPLE_BRANCH_TUBE_D / 2.0,
            62.0,
            20,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, SAMPLE_CENTER.1 + 30.0, DECK_Z + SAMPLE_Z + 8.0);
        bank = bank + loop_pad + branch;
    }
    bank
}

fn viability_cassette_docks() -> Part {
    let mut docks = Part::empty(format!("{OUTPUT_PREFIX}_viability_density_cassette_docks"));
    for index in 0..VIABILITY_CASSETTE_DOCKS {
        let x = SAMPLE_CENTER.0 + centered_index(index, VIABILITY_CASSETTE_DOCKS, 82.0);
        let dock = centered_cube(
            format!("{OUTPUT_PREFIX}_viability_cassette_dock_{index}"),
            58.0,
            28.0,
            10.0,
        )
        .translate(x, SAMPLE_CENTER.1 - 92.0, DECK_Z + SAMPLE_Z + 5.0);
        let optical_window = centered_cube(
            format!("{OUTPUT_PREFIX}_viability_cassette_dock_{index}_optical_window_cut"),
            34.0,
            12.0,
            12.0,
        )
        .translate(x, SAMPLE_CENTER.1 - 92.0, DECK_Z + SAMPLE_Z + 5.0);
        docks = docks + (dock - optical_window);
    }
    docks
}

fn sample_timing_token_row() -> Part {
    let mut row = Part::empty(format!("{OUTPUT_PREFIX}_sample_timing_token_row"));
    for source in 0..SAMPLE_TIMING_TOKENS {
        let x = SAMPLE_CENTER.0 + centered_index(source, SAMPLE_TIMING_TOKENS, 55.0);
        row = row
            + centered_cube(
                format!("{OUTPUT_PREFIX}_source_{source}_density_viability_sample_time_token_land"),
                34.0,
                14.0,
                6.0,
            )
            .translate(x, SAMPLE_CENTER.1 + 92.0, DECK_Z + SAMPLE_Z + 3.0);
    }
    row
}

fn sterile_connector_handoff_bulkhead() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_sterile_connector_handoff_bulkhead_body"),
        BULKHEAD_X,
        BULKHEAD_Y,
        BULKHEAD_Z,
    );

    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_handoff_bulkhead_port_cuts"));
    for port in 0..HANDOFF_PORTS {
        let x = centered_index(port, HANDOFF_PORTS, HANDOFF_PORT_PITCH_X);
        cuts = cuts
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_chip_lane_{port}_sterile_handoff_port_cut"),
                HANDOFF_PORT_D / 2.0,
                BULKHEAD_Y + 2.0,
                40,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, 12.0);
    }

    let bulkhead = (body - cuts).translate(
        BULKHEAD_CENTER.0,
        BULKHEAD_CENTER.1,
        deck_insert_z(BULKHEAD_Z),
    );

    bulkhead + handoff_port_collars() + source_inlet_manifold_tabs() + sterile_cap_parking_comb()
}

fn handoff_port_collars() -> Part {
    let mut collars = Part::empty(format!("{OUTPUT_PREFIX}_handoff_port_collars"));
    for port in 0..HANDOFF_PORTS {
        let x = BULKHEAD_CENTER.0 + centered_index(port, HANDOFF_PORTS, HANDOFF_PORT_PITCH_X);
        let y = BULKHEAD_CENTER.1 - BULKHEAD_Y / 2.0 - 6.0;
        let collar = centered_cylinder(
            format!("{OUTPUT_PREFIX}_chip_lane_{port}_handoff_port_collar"),
            (HANDOFF_PORT_D + 10.0) / 2.0,
            9.0,
            40,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, y, DECK_Z + BULKHEAD_Z / 2.0 + 12.0);
        let bore = centered_cylinder(
            format!("{OUTPUT_PREFIX}_chip_lane_{port}_handoff_port_collar_bore"),
            HANDOFF_PORT_D / 2.0,
            11.0,
            40,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, y, DECK_Z + BULKHEAD_Z / 2.0 + 12.0);
        collars = collars + (collar - bore);
    }
    collars
}

fn source_inlet_manifold_tabs() -> Part {
    let mut tabs = Part::empty(format!("{OUTPUT_PREFIX}_source_inlet_manifold_tabs"));
    for source in 0..SOURCE_INLET_PORTS {
        let x = BULKHEAD_CENTER.0 + centered_index(source, SOURCE_INLET_PORTS, 72.0);
        tabs = tabs
            + centered_cube(
                format!("{OUTPUT_PREFIX}_source_{source}_closed_inlet_keyed_tab"),
                48.0,
                16.0,
                18.0,
            )
            .translate(
                x,
                BULKHEAD_CENTER.1 + BULKHEAD_Y / 2.0 + 12.0,
                DECK_Z + BULKHEAD_Z - 4.0,
            );
    }
    tabs
}

fn sterile_cap_parking_comb() -> Part {
    let comb = centered_cube(
        format!("{OUTPUT_PREFIX}_sterile_cap_parking_comb"),
        BULKHEAD_X - 70.0,
        28.0,
        18.0,
    )
    .translate(
        BULKHEAD_CENTER.0,
        BULKHEAD_CENTER.1 - BULKHEAD_Y / 2.0 - 38.0,
        DECK_Z + 18.0,
    );

    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_sterile_cap_parking_comb_cuts"));
    for cap in 0..STERILE_CAP_PARKS {
        cuts = cuts
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_sterile_cap_park_{cap}_cut"),
                8.0,
                20.0,
                28,
            )
            .translate(
                BULKHEAD_CENTER.0 + centered_index(cap, STERILE_CAP_PARKS, 34.0),
                BULKHEAD_CENTER.1 - BULKHEAD_Y / 2.0 - 38.0,
                DECK_Z + 18.0,
            );
    }

    comb - cuts
}

fn bubble_dead_volume_window_lanes() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_bubble_dead_volume_window_panel"),
        BUBBLE_X,
        BUBBLE_Y,
        BUBBLE_Z,
    )
    .translate(BUBBLE_CENTER.0, BUBBLE_CENTER.1, deck_insert_z(BUBBLE_Z));

    body + bubble_window_row() + dead_volume_window_row() + lane_flush_trace_channels()
}

fn bubble_window_row() -> Part {
    let mut windows = Part::empty(format!("{OUTPUT_PREFIX}_bubble_window_row"));
    for lane in 0..BUBBLE_WINDOWS {
        let x = BUBBLE_CENTER.0 + centered_index(lane, BUBBLE_WINDOWS, WINDOW_PITCH_X);
        let window = centered_cube(
            format!("{OUTPUT_PREFIX}_chip_lane_{lane}_bubble_window_land"),
            WINDOW_X,
            WINDOW_Y,
            5.0,
        )
        .translate(x, BUBBLE_CENTER.1 + 22.0, DECK_Z + BUBBLE_Z + 2.5);
        let bubble_boss = centered_cylinder(
            format!("{OUTPUT_PREFIX}_chip_lane_{lane}_bubble_high_point_boss"),
            5.0,
            5.0,
            24,
        )
        .translate(x, BUBBLE_CENTER.1 + 22.0, DECK_Z + BUBBLE_Z + 7.5);
        windows = windows + window + bubble_boss;
    }
    windows
}

fn dead_volume_window_row() -> Part {
    let mut windows = Part::empty(format!("{OUTPUT_PREFIX}_dead_volume_window_row"));
    for lane in 0..DEAD_VOLUME_WINDOWS {
        let x = BUBBLE_CENTER.0 + centered_index(lane, DEAD_VOLUME_WINDOWS, WINDOW_PITCH_X);
        let window = centered_cube(
            format!("{OUTPUT_PREFIX}_chip_lane_{lane}_dead_volume_window_land"),
            WINDOW_X,
            WINDOW_Y,
            5.0,
        )
        .translate(x, BUBBLE_CENTER.1 - 22.0, DECK_Z + BUBBLE_Z + 2.5);
        let deadleg_gauge = centered_cube(
            format!("{OUTPUT_PREFIX}_chip_lane_{lane}_dead_volume_length_gauge"),
            6.0,
            WINDOW_Y + 12.0,
            6.0,
        )
        .translate(
            x + WINDOW_X / 2.0 - 5.0,
            BUBBLE_CENTER.1 - 22.0,
            DECK_Z + BUBBLE_Z + 6.0,
        );
        windows = windows + window + deadleg_gauge;
    }
    windows
}

fn lane_flush_trace_channels() -> Part {
    let mut channels = Part::empty(format!("{OUTPUT_PREFIX}_lane_flush_trace_channels"));
    for lane in 0..CHIP_LANES {
        let x = BUBBLE_CENTER.0 + centered_index(lane, CHIP_LANES, WINDOW_PITCH_X);
        channels = channels
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_chip_lane_{lane}_flush_trace_channel"),
                2.4,
                64.0,
                16,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, BUBBLE_CENTER.1, DECK_Z + BUBBLE_Z + 5.5);
    }
    channels
}

fn first_last_aliquot_custody_wells() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_first_last_aliquot_custody_body"),
        ALIQUOT_X,
        ALIQUOT_Y,
        ALIQUOT_Z,
    );

    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_first_last_aliquot_well_cuts"));
    for lane in 0..CHIP_LANES {
        for row in 0..2 {
            let x = centered_index(lane, CHIP_LANES, ALIQUOT_WELL_PITCH_X);
            let y = centered_index(row, 2, ALIQUOT_ROW_PITCH_Y);
            cuts = cuts
                + centered_cylinder(
                    format!("{OUTPUT_PREFIX}_chip_lane_{lane}_aliquot_row_{row}_well_cut"),
                    ALIQUOT_WELL_D / 2.0,
                    ALIQUOT_WELL_DEPTH + 2.0,
                    24,
                )
                .translate(x, y, ALIQUOT_Z / 2.0 - ALIQUOT_WELL_DEPTH / 2.0 + 0.5);
        }
    }

    let bank =
        (body - cuts).translate(ALIQUOT_CENTER.0, ALIQUOT_CENTER.1, deck_insert_z(ALIQUOT_Z));

    bank + aliquot_row_guards() + aliquot_tamper_seal_bar()
}

fn aliquot_row_guards() -> Part {
    let first = centered_cube(
        format!("{OUTPUT_PREFIX}_first_aliquot_row_guard"),
        ALIQUOT_X - 16.0,
        4.0,
        12.0,
    )
    .translate(
        ALIQUOT_CENTER.0,
        ALIQUOT_CENTER.1 + ALIQUOT_ROW_PITCH_Y / 2.0 + 9.0,
        DECK_Z + ALIQUOT_Z + 6.0,
    );
    let last = centered_cube(
        format!("{OUTPUT_PREFIX}_last_aliquot_row_guard"),
        ALIQUOT_X - 16.0,
        4.0,
        12.0,
    )
    .translate(
        ALIQUOT_CENTER.0,
        ALIQUOT_CENTER.1 - ALIQUOT_ROW_PITCH_Y / 2.0 - 9.0,
        DECK_Z + ALIQUOT_Z + 6.0,
    );

    first + last
}

fn aliquot_tamper_seal_bar() -> Part {
    let bar = centered_cube(
        format!("{OUTPUT_PREFIX}_aliquot_custody_tamper_seal_bar"),
        ALIQUOT_X - 18.0,
        8.0,
        12.0,
    )
    .translate(ALIQUOT_CENTER.0, ALIQUOT_CENTER.1, DECK_Z + ALIQUOT_Z + 6.0);

    let mut holes = Part::empty(format!("{OUTPUT_PREFIX}_aliquot_custody_tamper_seal_holes"));
    for index in 0..4 {
        holes = holes
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_aliquot_custody_tamper_seal_hole_{index}"),
                3.0,
                14.0,
                18,
            )
            .translate(
                ALIQUOT_CENTER.0 + centered_index(index, 4, 34.0),
                ALIQUOT_CENTER.1,
                DECK_Z + ALIQUOT_Z + 6.0,
            );
    }

    bar - holes
}

fn release_hold_reject_disposition_gate() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_release_hold_reject_disposition_body"),
        DISPOSITION_X,
        DISPOSITION_Y,
        DISPOSITION_Z,
    );

    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_disposition_slot_cuts"));
    for lane in 0..DISPOSITION_LANES.len() {
        for slot in 0..DISPOSITION_SLOTS_PER_LANE {
            let x = centered_index(slot, DISPOSITION_SLOTS_PER_LANE, DISPOSITION_SLOT_PITCH_X);
            let y = disposition_lane_y(lane);
            cuts = cuts
                + centered_cube(
                    format!(
                        "{OUTPUT_PREFIX}_{}_source_{slot}_disposition_slot_cut",
                        DISPOSITION_LANES[lane]
                    ),
                    DISPOSITION_SLOT_X,
                    DISPOSITION_SLOT_Y,
                    DISPOSITION_SLOT_DEPTH + 1.0,
                )
                .translate(
                    x,
                    y,
                    DISPOSITION_Z / 2.0 - DISPOSITION_SLOT_DEPTH / 2.0 + 0.5,
                );
        }
    }

    let gate = (body - cuts).translate(
        DISPOSITION_CENTER.0,
        DISPOSITION_CENTER.1,
        deck_insert_z(DISPOSITION_Z),
    );

    gate + disposition_lane_dividers() + fail_closed_latch_posts()
}

fn disposition_lane_dividers() -> Part {
    let mut dividers = Part::empty(format!("{OUTPUT_PREFIX}_disposition_lane_dividers"));
    for lane in 1..DISPOSITION_LANES.len() {
        let y =
            DISPOSITION_CENTER.1 + (disposition_lane_y(lane - 1) + disposition_lane_y(lane)) / 2.0;
        dividers = dividers
            + centered_cube(
                format!("{OUTPUT_PREFIX}_disposition_lane_divider_{lane}"),
                DISPOSITION_X - 22.0,
                5.0,
                16.0,
            )
            .translate(DISPOSITION_CENTER.0, y, DECK_Z + DISPOSITION_Z + 8.0);
    }
    dividers
}

fn fail_closed_latch_posts() -> Part {
    let mut posts = Part::empty(format!(
        "{OUTPUT_PREFIX}_fail_closed_disposition_latch_posts"
    ));
    for lane in 0..DISPOSITION_LANES.len() {
        let y = DISPOSITION_CENTER.1 + disposition_lane_y(lane);
        posts = posts
            + centered_cylinder(
                format!(
                    "{OUTPUT_PREFIX}_{}_fail_closed_latch_post",
                    DISPOSITION_LANES[lane]
                ),
                5.0,
                22.0,
                20,
            )
            .translate(
                DISPOSITION_CENTER.0 - DISPOSITION_X / 2.0 + 18.0,
                y,
                DECK_Z + DISPOSITION_Z + 11.0,
            )
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_{}_qa_key_post", DISPOSITION_LANES[lane]),
                5.0,
                22.0,
                20,
            )
            .translate(
                DISPOSITION_CENTER.0 + DISPOSITION_X / 2.0 - 18.0,
                y,
                DECK_Z + DISPOSITION_Z + 11.0,
            );
    }
    posts
}

fn camera_fiducial_bridge() -> Part {
    let beam = centered_cube(
        format!("{OUTPUT_PREFIX}_camera_evidence_bridge_beam"),
        BRIDGE_SPAN_X,
        48.0,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, 30.0, BRIDGE_CLEARANCE_Z + BRIDGE_BEAM_Z / 2.0);

    let rear_beam = centered_cube(
        format!("{OUTPUT_PREFIX}_rear_camera_fiducial_bridge_beam"),
        BRIDGE_SPAN_X,
        32.0,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, 358.0, BRIDGE_CLEARANCE_Z + BRIDGE_BEAM_Z / 2.0);

    beam + rear_beam + bridge_posts() + camera_pods() + fiducial_pads() + bridge_light_bars()
}

fn bridge_posts() -> Part {
    let mut posts = Part::empty(format!("{OUTPUT_PREFIX}_camera_bridge_posts"));
    for (index, (x, y)) in bridge_post_positions().iter().enumerate() {
        posts = posts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_camera_bridge_post_{index}"),
                BRIDGE_POST_X,
                BRIDGE_POST_Y,
                BRIDGE_CLEARANCE_Z,
            )
            .translate(*x, *y, BRIDGE_CLEARANCE_Z / 2.0);
    }
    posts
}

fn camera_pods() -> Part {
    let mut pods = Part::empty(format!("{OUTPUT_PREFIX}_overhead_camera_pods"));
    for index in 0..CAMERA_PODS {
        let x = centered_index(index, CAMERA_PODS, 250.0);
        pods = pods
            + centered_cube(
                format!("{OUTPUT_PREFIX}_camera_pod_{index}_envelope"),
                72.0,
                42.0,
                30.0,
            )
            .translate(x, 30.0, BRIDGE_CLEARANCE_Z + BRIDGE_BEAM_Z + 15.0);
    }
    pods
}

fn fiducial_pads() -> Part {
    let mut pads = Part::empty(format!("{OUTPUT_PREFIX}_camera_fiducial_pads"));
    for (index, (x, y)) in fiducial_positions().iter().enumerate() {
        let pad = centered_cube(
            format!("{OUTPUT_PREFIX}_fiducial_pad_{index}"),
            32.0,
            32.0,
            5.0,
        )
        .translate(*x, *y, DECK_Z + 2.5);
        let cross_x = centered_cube(
            format!("{OUTPUT_PREFIX}_fiducial_pad_{index}_cross_x_cut"),
            26.0,
            4.0,
            6.0,
        )
        .translate(*x, *y, DECK_Z + 2.5);
        let cross_y = centered_cube(
            format!("{OUTPUT_PREFIX}_fiducial_pad_{index}_cross_y_cut"),
            4.0,
            26.0,
            6.0,
        )
        .translate(*x, *y, DECK_Z + 2.5);
        pads = pads + (pad - cross_x - cross_y);
    }
    pads
}

fn bridge_light_bars() -> Part {
    let mut bars = Part::empty(format!("{OUTPUT_PREFIX}_camera_bridge_light_bars"));
    for index in 0..LIGHT_BARS {
        let y = 0.0 + index as f64 * 32.0;
        bars = bars
            + centered_cube(
                format!("{OUTPUT_PREFIX}_evidence_light_bar_{index}"),
                BRIDGE_SPAN_X - 180.0,
                8.0,
                12.0,
            )
            .translate(0.0, y, BRIDGE_CLEARANCE_Z - 14.0);
    }
    bars
}

fn robot_service_keepouts() -> Part {
    let front = centered_cube(
        format!("{OUTPUT_PREFIX}_front_robot_sweep_keepout_gauge"),
        STATION_X - 220.0,
        28.0,
        KEEP_OUT_Z,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 + 70.0,
        DECK_Z + RIM_Z + KEEP_OUT_Z / 2.0,
    );
    let rear = centered_cube(
        format!("{OUTPUT_PREFIX}_rear_service_keepout_gauge"),
        STATION_X - 260.0,
        24.0,
        KEEP_OUT_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - 84.0,
        DECK_Z + RIM_Z + KEEP_OUT_Z / 2.0,
    );
    let left = centered_cube(
        format!("{OUTPUT_PREFIX}_left_service_keepout_gauge"),
        24.0,
        STATION_Y - 240.0,
        KEEP_OUT_Z,
    )
    .translate(
        -STATION_X / 2.0 + 82.0,
        0.0,
        DECK_Z + RIM_Z + KEEP_OUT_Z / 2.0,
    );
    let right = centered_cube(
        format!("{OUTPUT_PREFIX}_right_automated_seeder_keepout_gauge"),
        24.0,
        STATION_Y - 240.0,
        KEEP_OUT_Z,
    )
    .translate(
        STATION_X / 2.0 - 82.0,
        0.0,
        DECK_Z + RIM_Z + KEEP_OUT_Z / 2.0,
    );
    let overhead = centered_cube(
        format!("{OUTPUT_PREFIX}_overhead_camera_service_keepout_gauge"),
        STATION_X - 340.0,
        44.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, 0.0, OVERHEAD_SERVICE_CLEARANCE_Z);

    front + rear + left + right + overhead
}

fn mount_positions() -> [(f64, f64); 8] {
    [
        (-STATION_X / 2.0 + 70.0, -STATION_Y / 2.0 + 70.0),
        (-STATION_X / 2.0 + 70.0, STATION_Y / 2.0 - 70.0),
        (STATION_X / 2.0 - 70.0, -STATION_Y / 2.0 + 70.0),
        (STATION_X / 2.0 - 70.0, STATION_Y / 2.0 - 70.0),
        (-STATION_X / 2.0 + 70.0, 0.0),
        (STATION_X / 2.0 - 70.0, 0.0),
        (0.0, -STATION_Y / 2.0 + 70.0),
        (0.0, STATION_Y / 2.0 - 70.0),
    ]
}

fn bridge_post_positions() -> [(f64, f64); 4] {
    [
        (-BRIDGE_SPAN_X / 2.0 + 42.0, 30.0),
        (BRIDGE_SPAN_X / 2.0 - 42.0, 30.0),
        (-BRIDGE_SPAN_X / 2.0 + 42.0, 358.0),
        (BRIDGE_SPAN_X / 2.0 - 42.0, 358.0),
    ]
}

fn fiducial_positions() -> [(f64, f64); FIDUCIAL_PADS] {
    [
        (-STATION_X / 2.0 + 120.0, -STATION_Y / 2.0 + 110.0),
        (STATION_X / 2.0 - 120.0, -STATION_Y / 2.0 + 110.0),
        (-STATION_X / 2.0 + 120.0, STATION_Y / 2.0 - 110.0),
        (STATION_X / 2.0 - 120.0, STATION_Y / 2.0 - 110.0),
        (0.0, STATION_Y / 2.0 - 110.0),
        (0.0, -STATION_Y / 2.0 + 110.0),
    ]
}

fn local_source_xy(index: usize) -> (f64, f64) {
    let row = index / SOURCE_COLS;
    let col = index % SOURCE_COLS;
    (
        centered_index(col, SOURCE_COLS, SOURCE_PITCH_X),
        centered_index(row, SOURCE_ROWS, SOURCE_PITCH_Y),
    )
}

fn world_source_xy(index: usize) -> (f64, f64) {
    let (x, y) = local_source_xy(index);
    (SOURCE_CENTER.0 + x, SOURCE_CENTER.1 + y)
}

fn local_timing_slot_xy(source: usize, event: usize) -> (f64, f64) {
    (
        centered_index(event, TIMING_EVENT_COUNT, TIMING_EVENT_PITCH_X),
        centered_index(source, SOURCE_SLOTS, TIMING_SOURCE_PITCH_Y),
    )
}

fn local_temp_pocket_xy(index: usize) -> (f64, f64) {
    let row = index / 4;
    let col = index % 4;
    (
        centered_index(col, 4, TEMP_POCKET_PITCH_X),
        centered_index(row, 2, TEMP_POCKET_PITCH_Y),
    )
}

fn world_temp_pocket_xy(index: usize) -> (f64, f64) {
    let (x, y) = local_temp_pocket_xy(index);
    (TEMP_CENTER.0 + x, TEMP_CENTER.1 + y)
}

fn disposition_lane_y(lane: usize) -> f64 {
    centered_index(lane, DISPOSITION_LANES.len(), DISPOSITION_LANE_PITCH_Y)
}

fn deck_insert_z(height: f64) -> f64 {
    DECK_Z + height / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 14);
        for path in OUTPUTS {
            assert!(path.starts_with(
                "output/closed_cell_source_thaw_to_seed_timing_identity_handoff_station_"
            ));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn required_features_cover_timing_identity_handoff_and_release() {
        for feature in [
            "containment_deck",
            "source_vial_bag_nests",
            "timed_thaw_to_seed_token_rail",
            "temperature_logger_pockets",
            "identity_barcode_rfid_lands",
            "gentle_mix_witness",
            "density_viability_sample_loop_surrogate",
            "sterile_connector_handoff_bulkhead",
            "bubble_dead_volume_windows",
            "first_last_aliquot_custody_wells",
            "release_hold_reject_disposition",
            "camera_fiducials",
            "robot_service_keepouts",
            "named_stl_outputs",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn primary_modules_fit_without_plan_view_collision() {
        assert_design_constraints();
        let rects = layout_rects();
        assert!(rects.iter().all(|rect| rect.fits_inside_station()));
        for (i, first) in rects.iter().enumerate() {
            for second in rects.iter().skip(i + 1) {
                assert!(!first.overlaps(*second));
            }
        }
    }

    #[test]
    fn source_identity_and_handoff_capacity_scale_to_multi_chip_batch() {
        assert_eq!(CHIP_LANES, 12);
        assert_eq!(CASSETTES_PER_BATCH, 4);
        assert_eq!(SOURCE_SLOTS, SOURCE_ROWS * SOURCE_COLS);
        assert_eq!(CHIPS_PER_SOURCE, 2);
        assert_eq!(HANDOFF_PORTS, CHIP_LANES);
        assert!(SOURCE_INLET_PORTS >= SOURCE_SLOTS);
        assert!(BARCODE_LANDS >= CHIP_LANES + SOURCE_SLOTS);
        assert!(RFID_LANDS >= SOURCE_SLOTS);
        assert!(LOT_CARD_SLOTS >= SOURCE_SLOTS);
    }

    #[test]
    fn timing_tokens_bound_thaw_mix_sample_and_seed_drift() {
        assert_eq!(TIMING_EVENTS[0], "thaw_end");
        assert_eq!(TIMING_EVENTS[1], "mix_done");
        assert_eq!(TIMING_EVENTS[2], "sample_draw");
        assert_eq!(TIMING_EVENTS[4], "seed_start");
        assert_eq!(TIMING_TOKEN_COUNT, SOURCE_SLOTS * TIMING_EVENT_COUNT);
        assert!(MAX_THAW_TO_SEED_MIN <= 90);
        assert!(MAX_MIX_TO_SEED_MIN <= 20);
        for source in 0..SOURCE_SLOTS {
            for event in 0..TIMING_EVENT_COUNT {
                let (x, y) = local_timing_slot_xy(source, event);
                assert!(x.abs() + TOKEN_SLOT_X / 2.0 < TIMING_X / 2.0 - 24.0);
                assert!(y.abs() + TOKEN_SLOT_Y / 2.0 < TIMING_Y / 2.0 - 12.0);
            }
        }
    }

    #[test]
    fn witness_counts_cover_temperature_mix_sampling_and_aliquot_custody() {
        assert!(TEMP_LOGGER_POCKETS >= SOURCE_SLOTS + 2);
        assert_eq!(MIX_WITNESS_LANES, SOURCE_SLOTS);
        assert_eq!(MIX_ENDPOINT_WITNESSES, SOURCE_SLOTS * 2);
        assert!(MIX_BEAD_WINDOWS_PER_SOURCE >= 3);
        assert!(MAX_MIX_RPM <= 8.0);
        assert!(SAMPLE_LOOP_CHANNELS >= CASSETTES_PER_BATCH);
        assert!(SAMPLE_LOOP_UL <= 100.0);
        assert_eq!(SAMPLE_TIMING_TOKENS, SOURCE_SLOTS);
        assert_eq!(ALIQUOT_WELLS, CHIP_LANES * 2);
    }

    #[test]
    fn bubble_dead_volume_and_disposition_are_lane_complete() {
        assert_eq!(BUBBLE_WINDOWS, CHIP_LANES);
        assert_eq!(DEAD_VOLUME_WINDOWS, CHIP_LANES);
        assert_eq!(DISPOSITION_LANES, ["release", "hold", "reject"]);
        assert_eq!(DISPOSITION_SLOTS_PER_LANE, SOURCE_SLOTS);
        assert!(DISPOSITION_SLOT_PITCH_X * (DISPOSITION_SLOTS_PER_LANE as f64) < DISPOSITION_X);
        assert!(disposition_lane_y(0) < disposition_lane_y(1));
        assert!(disposition_lane_y(1) < disposition_lane_y(2));
    }

    #[test]
    fn camera_and_robot_service_envelopes_are_explicit() {
        assert_eq!(CAMERA_PODS, 5);
        assert_eq!(FIDUCIAL_PADS, fiducial_positions().len());
        assert_eq!(LIGHT_BARS, 3);
        assert_eq!(KEEP_OUT_GAUGES, 5);
        assert!(ROBOT_FRONT_CLEARANCE_Y >= 350.0);
        assert!(SERVICE_REAR_CLEARANCE_Y >= 220.0);
        assert!(LEFT_SERVICE_CLEARANCE_X >= 200.0);
        assert!(RIGHT_SEEDER_CLEARANCE_X >= 250.0);
        assert!(OVERHEAD_SERVICE_CLEARANCE_Z > BRIDGE_CLEARANCE_Z + BRIDGE_BEAM_Z);
    }
}
