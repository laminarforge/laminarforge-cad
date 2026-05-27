use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed cell-source bag agitation shear-threshold validation station.
//
// Intent:
// - Validate that incoming closed cell-source bags can be gently mixed before
//   seeding without exceeding the documented low-shear rocking envelope.
// - Make bag support, rocking limits, accelerometer/tilt logging, sampling
//   handoff, bubble/foam inspection, identity custody, and release/hold/reject
//   evidence gates visible in the CAD fixture.
// - This is validation/interface CAD only. Cell viability criteria, acceptance
//   limits, sterility claims, and seeding recipes remain external controls.

const OUTPUT_PREFIX: &str = "closed_cell_source_bag_agitation_shear_threshold_station";

const OUTPUTS: [&str; 13] = [
    "output/closed_cell_source_bag_agitation_shear_threshold_station_containment_deck.stl",
    "output/closed_cell_source_bag_agitation_shear_threshold_station_bag_cradle.stl",
    "output/closed_cell_source_bag_agitation_shear_threshold_station_low_shear_rocking_envelope.stl",
    "output/closed_cell_source_bag_agitation_shear_threshold_station_accelerometer_tilt_witness_blocks.stl",
    "output/closed_cell_source_bag_agitation_shear_threshold_station_sampling_handoff_manifold.stl",
    "output/closed_cell_source_bag_agitation_shear_threshold_station_bubble_foam_observation_windows.stl",
    "output/closed_cell_source_bag_agitation_shear_threshold_station_foam_height_coupon_array.stl",
    "output/closed_cell_source_bag_agitation_shear_threshold_station_tubing_strain_relief_clamp_comb.stl",
    "output/closed_cell_source_bag_agitation_shear_threshold_station_identity_custody_barcode_lands.stl",
    "output/closed_cell_source_bag_agitation_shear_threshold_station_release_hold_reject_evidence_gate.stl",
    "output/closed_cell_source_bag_agitation_shear_threshold_station_evidence_camera_bridge.stl",
    "output/closed_cell_source_bag_agitation_shear_threshold_station_robot_service_keepouts.stl",
    "output/closed_cell_source_bag_agitation_shear_threshold_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 13] = [
    "containment_deck",
    "bag_cradle",
    "low_shear_rocking_envelope",
    "accelerometer_tilt_witness_blocks",
    "sampling_handoff_manifold",
    "bubble_foam_observation_windows",
    "foam_height_coupon_array",
    "tubing_strain_relief_clamp_comb",
    "identity_custody_barcode_lands",
    "release_hold_reject_evidence_gate",
    "evidence_camera_bridge",
    "robot_service_keepouts",
    "assembly",
];

const STATION_X: f64 = 1560.0;
const STATION_Y: f64 = 940.0;
const DECK_Z: f64 = 22.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 52.0;
const BASIN_X: f64 = 1360.0;
const BASIN_Y: f64 = 760.0;
const BASIN_DEPTH: f64 = 7.0;
const DRAIN_PORT_D: f64 = 16.0;
const MOUNT_HOLE_D: f64 = 6.8;
const MOUNT_BOSSES: usize = 8;
const LEAK_SENSOR_WELLS: usize = 8;

const SOURCE_BAGS: usize = 4;
const BAG_CRADLE_CENTER: (f64, f64) = (-440.0, 180.0);
const BAG_CRADLE_X: f64 = 520.0;
const BAG_CRADLE_Y: f64 = 300.0;
const BAG_CRADLE_Z: f64 = 58.0;
const BAG_POCKET_X: f64 = 430.0;
const BAG_POCKET_Y: f64 = 210.0;
const BAG_POCKET_DEPTH: f64 = 18.0;
const SADDLE_RIBS: usize = 9;
const SOFT_LOCATOR_PINS: usize = 8;
const PORT_CAPTURE_CLIPS: usize = SOURCE_BAGS * 2;

const ROCKER_CENTER: (f64, f64) = BAG_CRADLE_CENTER;
const ROCKER_RADIUS: f64 = 270.0;
const ROCKER_SEGMENTS: usize = 17;
const MAX_ROCK_ANGLE_DEG: f64 = 8.0;
const ROCKER_LIMIT_STOPS: usize = 4;
const ROCKER_PIVOT_D: f64 = 34.0;
const ACCEL_LIMIT_G: f64 = 0.15;
const SHEAR_WARNING_TOKENS: usize = 5;

const LOGGER_CENTER: (f64, f64) = (160.0, 255.0);
const LOGGER_X: f64 = 410.0;
const LOGGER_Y: f64 = 245.0;
const LOGGER_Z: f64 = 44.0;
const ACCELEROMETER_NESTS: usize = SOURCE_BAGS + 1;
const TILT_WITNESS_BLOCKS: usize = 3;
const TILT_SCALE_TICKS: usize = 17;
const LOGGER_CABLE_CLIPS: usize = 6;

const SAMPLE_CENTER: (f64, f64) = (520.0, -70.0);
const SAMPLE_X: f64 = 320.0;
const SAMPLE_Y: f64 = 240.0;
const SAMPLE_Z: f64 = 48.0;
const SAMPLE_PORTS: usize = SOURCE_BAGS;
const SAMPLE_LOOP_LEVELS: usize = 3;
const SAMPLE_LOOP_UL: f64 = 60.0;
const SAMPLE_LOOP_D: f64 = 15.0;
const SAMPLE_LOOP_PITCH_X: f64 = 58.0;
const VIABILITY_DOCKS: usize = SOURCE_BAGS;
const STERILE_HANDOFF_PORTS: usize = SOURCE_BAGS * 2;

const BUBBLE_CENTER: (f64, f64) = (-455.0, -190.0);
const BUBBLE_X: f64 = 520.0;
const BUBBLE_Y: f64 = 190.0;
const BUBBLE_Z: f64 = 36.0;
const OBSERVATION_WINDOWS: usize = SOURCE_BAGS * 2;
const BUBBLE_SCALE_TICKS: usize = 12;
const FOAM_TRAP_WITNESSES: usize = SOURCE_BAGS;

const FOAM_CENTER: (f64, f64) = (-35.0, -210.0);
const FOAM_X: f64 = 300.0;
const FOAM_Y: f64 = 190.0;
const FOAM_Z: f64 = 28.0;
const FOAM_COUPONS: usize = SOURCE_BAGS;
const FOAM_HEIGHT_LEVELS: usize = 5;

const TUBING_CENTER: (f64, f64) = (440.0, -300.0);
const TUBING_X: f64 = 420.0;
const TUBING_Y: f64 = 110.0;
const TUBING_Z: f64 = 38.0;
const TUBING_CHANNELS: usize = STERILE_HANDOFF_PORTS;
const TUBING_CHANNEL_D: f64 = 10.0;
const TUBING_CHANNEL_PITCH: f64 = 42.0;

const IDENTITY_CENTER: (f64, f64) = (550.0, 310.0);
const IDENTITY_X: f64 = 300.0;
const IDENTITY_Y: f64 = 150.0;
const IDENTITY_Z: f64 = 22.0;
const BARCODE_LANDS: usize = SOURCE_BAGS * 3;
const RFID_LANDS: usize = SOURCE_BAGS;
const BAG_LOT_CARD_SLOTS: usize = SOURCE_BAGS;

const GATE_CENTER: (f64, f64) = (120.0, 30.0);
const GATE_X: f64 = 360.0;
const GATE_Y: f64 = 200.0;
const GATE_Z: f64 = 36.0;
const DISPOSITION_LANES: [&str; 3] = ["release", "hold", "reject"];
const EVIDENCE_SLOTS_PER_LANE: usize = SOURCE_BAGS;
const EVIDENCE_SLOT_X: f64 = 58.0;
const EVIDENCE_SLOT_Y: f64 = 26.0;
const GATE_INTERLOCK_PINS: usize = 6;

const BRIDGE_SPAN_X: f64 = 1320.0;
const BRIDGE_POST_Z: f64 = 260.0;
const BRIDGE_BEAM_Z: f64 = 34.0;
const CAMERA_PODS: usize = 5;
const LIGHT_BARS: usize = 4;
const FIDUCIAL_PADS: usize = 6;

const FRONT_ROBOT_CLEARANCE: f64 = 390.0;
const REAR_SERVICE_CLEARANCE: f64 = 245.0;
const LEFT_BAG_LOAD_CLEARANCE: f64 = 230.0;
const RIGHT_ANALYZER_CLEARANCE: f64 = 260.0;
const OVERHEAD_ROCKER_CLEARANCE_Z: f64 = 370.0;
const KEEP_OUT_RAIL: f64 = 8.0;

#[derive(Clone, Copy)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_station(self) -> bool {
        let usable_x = STATION_X / 2.0 - RIM_W - 30.0;
        let usable_y = STATION_Y / 2.0 - RIM_W - 30.0;
        self.center.0 - self.x / 2.0 >= -usable_x
            && self.center.0 + self.x / 2.0 <= usable_x
            && self.center.1 - self.y / 2.0 >= -usable_y
            && self.center.1 + self.y / 2.0 <= usable_y
    }

    #[cfg(test)]
    fn overlaps(self, other: Rect) -> bool {
        let x_overlap = (self.center.0 - other.center.0).abs() < (self.x + other.x) / 2.0;
        let y_overlap = (self.center.1 - other.center.1).abs() < (self.y + other.y) / 2.0;
        x_overlap && y_overlap
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = containment_deck();
    export(OUTPUTS[0], &deck);

    let cradle = bag_cradle();
    export(OUTPUTS[1], &cradle);

    let rocker = low_shear_rocking_envelope();
    export(OUTPUTS[2], &rocker);

    let loggers = accelerometer_tilt_witness_blocks();
    export(OUTPUTS[3], &loggers);

    let sampling = sampling_handoff_manifold();
    export(OUTPUTS[4], &sampling);

    let bubbles = bubble_foam_observation_windows();
    export(OUTPUTS[5], &bubbles);

    let foam = foam_height_coupon_array();
    export(OUTPUTS[6], &foam);

    let tubing = tubing_strain_relief_clamp_comb();
    export(OUTPUTS[7], &tubing);

    let identity = identity_custody_barcode_lands();
    export(OUTPUTS[8], &identity);

    let gate = release_hold_reject_evidence_gate();
    export(OUTPUTS[9], &gate);

    let bridge = evidence_camera_bridge();
    export(OUTPUTS[10], &bridge);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[11], &keepouts);

    let assembly = station_assembly();
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed cell-source bag agitation shear-threshold validation station:");
    println!(
        "  Containment deck: {STATION_X:.0}mm x {STATION_Y:.0}mm with raised rim, recessed basin, {LEAK_SENSOR_WELLS} leak wells, and drain witness"
    );
    println!(
        "  Bag cradle:       {SOURCE_BAGS} incoming bags, {SADDLE_RIBS} soft saddle ribs, {SOFT_LOCATOR_PINS} locator pins, {PORT_CAPTURE_CLIPS} port capture clips"
    );
    println!(
        "  Rocking envelope: +/-{MAX_ROCK_ANGLE_DEG:.1}deg low-shear arc, {ROCKER_SEGMENTS} witness segments, {ROCKER_LIMIT_STOPS} hard stops, {ACCEL_LIMIT_G:.2}g acceleration evidence limit"
    );
    println!(
        "  Witness blocks:   {ACCELEROMETER_NESTS} accelerometer nests, {TILT_WITNESS_BLOCKS} tilt blocks, {TILT_SCALE_TICKS} tilt ticks, {LOGGER_CABLE_CLIPS} cable clips"
    );
    println!(
        "  Sampling handoff: {SAMPLE_PORTS} bag sample ports, {SAMPLE_LOOP_LEVELS} level loops at {SAMPLE_LOOP_UL:.0}uL nominal, {VIABILITY_DOCKS} viability docks, {STERILE_HANDOFF_PORTS} sterile handoff ports"
    );
    println!(
        "  Bubble/foam view: {OBSERVATION_WINDOWS} observation windows, {FOAM_TRAP_WITNESSES} foam trap witnesses, {FOAM_COUPONS} removable foam-height coupons"
    );
    println!(
        "  Evidence gates:   {:?} lanes, {EVIDENCE_SLOTS_PER_LANE} evidence slots per lane, {CAMERA_PODS} camera pods, {FIDUCIAL_PADS} fiducials",
        DISPOSITION_LANES
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    containment_deck()
        + bag_cradle().translate(BAG_CRADLE_CENTER.0, BAG_CRADLE_CENTER.1, DECK_Z)
        + low_shear_rocking_envelope().translate(ROCKER_CENTER.0, ROCKER_CENTER.1, DECK_Z + 4.0)
        + accelerometer_tilt_witness_blocks().translate(LOGGER_CENTER.0, LOGGER_CENTER.1, DECK_Z)
        + sampling_handoff_manifold().translate(SAMPLE_CENTER.0, SAMPLE_CENTER.1, DECK_Z)
        + bubble_foam_observation_windows().translate(BUBBLE_CENTER.0, BUBBLE_CENTER.1, DECK_Z)
        + foam_height_coupon_array().translate(FOAM_CENTER.0, FOAM_CENTER.1, DECK_Z)
        + tubing_strain_relief_clamp_comb().translate(TUBING_CENTER.0, TUBING_CENTER.1, DECK_Z)
        + identity_custody_barcode_lands().translate(IDENTITY_CENTER.0, IDENTITY_CENTER.1, DECK_Z)
        + release_hold_reject_evidence_gate().translate(GATE_CENTER.0, GATE_CENTER.1, DECK_Z)
        + evidence_camera_bridge()
        + robot_service_keepouts()
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
        format!("{OUTPUT_PREFIX}_recessed_secondary_containment_basin_cut"),
        BASIN_X,
        BASIN_Y,
        BASIN_DEPTH + 1.0,
    )
    .translate(0.0, 0.0, DECK_Z - BASIN_DEPTH / 2.0 + 0.5);
    let drain = centered_cylinder(
        format!("{OUTPUT_PREFIX}_front_drain_port_cut"),
        DRAIN_PORT_D / 2.0,
        RIM_W + 36.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, -STATION_Y / 2.0 + 12.0, DECK_Z - 5.0);

    deck - basin - drain
        + containment_rims()
        + deck_mount_bosses()
        + leak_sensor_wells()
        + module_floor_markers()
        + deck_datum_targets()
}

fn containment_rims() -> Part {
    let front = centered_cube(
        format!("{OUTPUT_PREFIX}_front_secondary_containment_rim"),
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        format!("{OUTPUT_PREFIX}_rear_secondary_containment_rim"),
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let left = centered_cube(
        format!("{OUTPUT_PREFIX}_left_secondary_containment_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube(
        format!("{OUTPUT_PREFIX}_right_secondary_containment_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    front + rear + left + right
}

fn deck_mount_bosses() -> Part {
    let mut bosses = Part::empty(format!("{OUTPUT_PREFIX}_deck_mount_bosses"));
    for (index, (x, y)) in mount_positions().iter().enumerate() {
        let boss = centered_cylinder(
            format!("{OUTPUT_PREFIX}_deck_mount_boss_{index}"),
            18.0,
            7.0,
            32,
        )
        .translate(*x, *y, DECK_Z + 3.5);
        let hole = centered_cylinder(
            format!("{OUTPUT_PREFIX}_deck_m6_mount_hole_cut_{index}"),
            MOUNT_HOLE_D / 2.0,
            9.0,
            24,
        )
        .translate(*x, *y, DECK_Z + 4.5);
        bosses = bosses + (boss - hole);
    }
    bosses
}

fn leak_sensor_wells() -> Part {
    let mut wells = Part::empty(format!("{OUTPUT_PREFIX}_deck_leak_sensor_wells"));
    for index in 0..LEAK_SENSOR_WELLS {
        let x = centered_index(index % 4, 4, 165.0);
        let y = -385.0 + (index / 4) as f64 * 42.0;
        wells = wells
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_leak_sensor_well_{index}"),
                16.0,
                7.0,
                28,
            )
            .translate(x, y, DECK_Z + 3.5);
    }
    wells
}

fn module_floor_markers() -> Part {
    let mut markers = Part::empty(format!("{OUTPUT_PREFIX}_module_floor_markers"));
    for rect in layout_rects() {
        markers = markers
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{}_floor_marker", rect.name),
                rect.x + 18.0,
                rect.y + 18.0,
                3.0,
            )
            .translate(rect.center.0, rect.center.1, DECK_Z + 1.5);
    }
    markers
}

fn deck_datum_targets() -> Part {
    let mut targets = Part::empty(format!("{OUTPUT_PREFIX}_deck_robot_datum_targets"));
    for (index, (x, y)) in datum_positions().iter().enumerate() {
        let boss = centered_cylinder(
            format!("{OUTPUT_PREFIX}_deck_datum_boss_{index}"),
            17.0,
            6.0,
            32,
        )
        .translate(*x, *y, DECK_Z + 3.0);
        let center = centered_cylinder(
            format!("{OUTPUT_PREFIX}_deck_datum_center_cut_{index}"),
            4.0,
            8.0,
            20,
        )
        .translate(*x, *y, DECK_Z + 4.0);
        targets = targets + (boss - center);
    }
    targets
}

fn bag_cradle() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_bag_cradle_body"),
        BAG_CRADLE_X,
        BAG_CRADLE_Y,
        BAG_CRADLE_Z,
    )
    .translate(0.0, 0.0, BAG_CRADLE_Z / 2.0);
    let pocket = centered_cube(
        format!("{OUTPUT_PREFIX}_soft_cell_source_bag_pocket_cut"),
        BAG_POCKET_X,
        BAG_POCKET_Y,
        BAG_POCKET_DEPTH + 1.0,
    )
    .translate(0.0, -4.0, BAG_CRADLE_Z - BAG_POCKET_DEPTH / 2.0 + 0.5);

    body - pocket
        + cradle_saddle_ribs()
        + soft_locator_pins()
        + bag_port_capture_clips()
        + bag_edge_compression_witness_lips()
}

fn cradle_saddle_ribs() -> Part {
    let mut ribs = Part::empty(format!("{OUTPUT_PREFIX}_low_shear_saddle_ribs"));
    for index in 0..SADDLE_RIBS {
        ribs = ribs
            + centered_cube(
                format!("{OUTPUT_PREFIX}_bag_cradle_soft_saddle_rib_{index}"),
                BAG_POCKET_X - 38.0,
                6.0,
                12.0,
            )
            .translate(
                0.0,
                centered_index(index, SADDLE_RIBS, 22.0) - 4.0,
                BAG_CRADLE_Z + 6.0,
            );
    }
    ribs
}

fn soft_locator_pins() -> Part {
    let mut pins = Part::empty(format!("{OUTPUT_PREFIX}_soft_bag_locator_pins"));
    for index in 0..SOFT_LOCATOR_PINS {
        let row = index / 4;
        let col = index % 4;
        pins = pins
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_bag_locator_pin_{index}"),
                8.0,
                16.0,
                24,
            )
            .translate(
                centered_index(col, 4, 122.0),
                if row == 0 { -122.0 } else { 114.0 },
                BAG_CRADLE_Z + 8.0,
            );
    }
    pins
}

fn bag_port_capture_clips() -> Part {
    let mut clips = Part::empty(format!("{OUTPUT_PREFIX}_bag_port_capture_clips"));
    for index in 0..PORT_CAPTURE_CLIPS {
        let side = if index % 2 == 0 { -1.0 } else { 1.0 };
        let bag = index / 2;
        clips = clips
            + centered_cube(
                format!("{OUTPUT_PREFIX}_bag_{bag}_port_capture_clip_{index}"),
                34.0,
                18.0,
                14.0,
            )
            .translate(
                centered_index(bag, SOURCE_BAGS, 92.0),
                side * (BAG_CRADLE_Y / 2.0 - 20.0),
                BAG_CRADLE_Z + 7.0,
            );
    }
    clips
}

fn bag_edge_compression_witness_lips() -> Part {
    let front = centered_cube(
        format!("{OUTPUT_PREFIX}_bag_front_edge_compression_witness_lip"),
        BAG_POCKET_X + 22.0,
        9.0,
        9.0,
    )
    .translate(0.0, -BAG_POCKET_Y / 2.0 - 9.0, BAG_CRADLE_Z + 4.5);
    let rear = centered_cube(
        format!("{OUTPUT_PREFIX}_bag_rear_edge_compression_witness_lip"),
        BAG_POCKET_X + 22.0,
        9.0,
        9.0,
    )
    .translate(0.0, BAG_POCKET_Y / 2.0 + 1.0, BAG_CRADLE_Z + 4.5);
    front + rear
}

fn low_shear_rocking_envelope() -> Part {
    rocker_arc_witness_rail(-86.0, "left")
        + rocker_arc_witness_rail(86.0, "right")
        + rocker_pivot_bosses()
        + rocker_limit_stop_flags()
        + acceleration_limit_token_strip()
}

fn rocker_arc_witness_rail(y_offset: f64, side: &str) -> Part {
    let mut rail = Part::empty(format!("{OUTPUT_PREFIX}_{side}_low_shear_rocking_arc_rail"));
    for index in 0..ROCKER_SEGMENTS {
        let frac = index as f64 / (ROCKER_SEGMENTS - 1) as f64;
        let deg = -MAX_ROCK_ANGLE_DEG + frac * MAX_ROCK_ANGLE_DEG * 2.0;
        let rad = deg.to_radians();
        let x = rad.sin() * ROCKER_RADIUS;
        let z = 34.0 + (1.0 - rad.cos()) * ROCKER_RADIUS;
        rail = rail
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{side}_rocker_arc_witness_segment_{index}"),
                20.0,
                18.0,
                10.0,
            )
            .translate(x, y_offset, z);
    }
    rail
}

fn rocker_pivot_bosses() -> Part {
    let left = centered_cylinder(
        format!("{OUTPUT_PREFIX}_left_rocker_pivot_axis_boss"),
        ROCKER_PIVOT_D / 2.0,
        28.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, -116.0, 36.0);
    let right = centered_cylinder(
        format!("{OUTPUT_PREFIX}_right_rocker_pivot_axis_boss"),
        ROCKER_PIVOT_D / 2.0,
        28.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, 116.0, 36.0);
    left + right
}

fn rocker_limit_stop_flags() -> Part {
    let mut stops = Part::empty(format!("{OUTPUT_PREFIX}_low_shear_limit_stop_flags"));
    for index in 0..ROCKER_LIMIT_STOPS {
        let x = if index % 2 == 0 { -78.0 } else { 78.0 };
        let y = if index < 2 { -132.0 } else { 132.0 };
        stops = stops
            + centered_cube(
                format!("{OUTPUT_PREFIX}_shear_threshold_hard_stop_flag_{index}"),
                30.0,
                18.0,
                56.0,
            )
            .translate(x, y, 28.0);
    }
    stops
}

fn acceleration_limit_token_strip() -> Part {
    let strip = centered_cube(
        format!("{OUTPUT_PREFIX}_acceleration_limit_token_strip"),
        310.0,
        22.0,
        8.0,
    )
    .translate(0.0, 0.0, 10.0);
    let mut tokens = Part::empty(format!("{OUTPUT_PREFIX}_shear_warning_acceleration_tokens"));
    for index in 0..SHEAR_WARNING_TOKENS {
        tokens = tokens
            + centered_cube(
                format!("{OUTPUT_PREFIX}_accel_limit_{index}_token_land"),
                42.0,
                15.0,
                5.0,
            )
            .translate(centered_index(index, SHEAR_WARNING_TOKENS, 54.0), 0.0, 16.5);
    }
    strip + tokens
}

fn accelerometer_tilt_witness_blocks() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_accelerometer_tilt_witness_block"),
        LOGGER_X,
        LOGGER_Y,
        LOGGER_Z,
    )
    .translate(0.0, 0.0, LOGGER_Z / 2.0);

    body - accelerometer_pocket_cuts() - tilt_block_pocket_cuts()
        + tilt_scale_tick_lands()
        + logger_cable_clip_lands()
        + logger_serial_plate_lands()
}

fn accelerometer_pocket_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_accelerometer_pocket_cuts"));
    for index in 0..ACCELEROMETER_NESTS {
        cuts = cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_accelerometer_nest_cut_{index}"),
                58.0,
                42.0,
                22.0,
            )
            .translate(
                centered_index(index, ACCELEROMETER_NESTS, 72.0),
                -48.0,
                LOGGER_Z - 10.0,
            );
    }
    cuts
}

fn tilt_block_pocket_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_tilt_witness_block_pocket_cuts"));
    for index in 0..TILT_WITNESS_BLOCKS {
        cuts = cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_tilt_witness_block_cut_{index}"),
                78.0,
                40.0,
                20.0,
            )
            .translate(
                centered_index(index, TILT_WITNESS_BLOCKS, 108.0),
                42.0,
                LOGGER_Z - 9.0,
            );
    }
    cuts
}

fn tilt_scale_tick_lands() -> Part {
    let mut ticks = Part::empty(format!("{OUTPUT_PREFIX}_tilt_scale_tick_lands"));
    for index in 0..TILT_SCALE_TICKS {
        ticks = ticks
            + centered_cube(
                format!("{OUTPUT_PREFIX}_tilt_scale_tick_{index}"),
                4.0,
                if index == TILT_SCALE_TICKS / 2 {
                    34.0
                } else {
                    20.0
                },
                5.0,
            )
            .translate(
                centered_index(index, TILT_SCALE_TICKS, 18.0),
                -LOGGER_Y / 2.0 + 18.0,
                LOGGER_Z + 2.5,
            );
    }
    ticks
}

fn logger_cable_clip_lands() -> Part {
    let mut clips = Part::empty(format!("{OUTPUT_PREFIX}_logger_cable_clip_lands"));
    for index in 0..LOGGER_CABLE_CLIPS {
        clips = clips
            + centered_cube(
                format!("{OUTPUT_PREFIX}_logger_cable_clip_land_{index}"),
                30.0,
                12.0,
                7.0,
            )
            .translate(
                centered_index(index, LOGGER_CABLE_CLIPS, 48.0),
                LOGGER_Y / 2.0 - 18.0,
                LOGGER_Z + 3.5,
            );
    }
    clips
}

fn logger_serial_plate_lands() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_logger_serial_plate_lands"));
    for index in 0..SOURCE_BAGS {
        lands = lands
            + centered_cube(
                format!("{OUTPUT_PREFIX}_bag_{index}_logger_serial_plate_land"),
                56.0,
                14.0,
                4.0,
            )
            .translate(
                centered_index(index, SOURCE_BAGS, 80.0),
                0.0,
                LOGGER_Z + 2.0,
            );
    }
    lands
}

fn sampling_handoff_manifold() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_sampling_handoff_manifold_body"),
        SAMPLE_X,
        SAMPLE_Y,
        SAMPLE_Z,
    )
    .translate(0.0, 0.0, SAMPLE_Z / 2.0);

    body - sample_loop_pocket_cuts() - sterile_handoff_port_cuts()
        + sample_port_collars()
        + viability_dock_lands()
        + sample_timing_token_lands()
}

fn sample_loop_pocket_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_sample_loop_pocket_cuts"));
    for level in 0..SAMPLE_LOOP_LEVELS {
        for port in 0..SAMPLE_PORTS {
            cuts = cuts
                + centered_cylinder(
                    format!("{OUTPUT_PREFIX}_sample_level_{level}_bag_{port}_loop_pocket_cut"),
                    SAMPLE_LOOP_D / 2.0,
                    SAMPLE_Z + 3.0,
                    28,
                )
                .translate(
                    centered_index(port, SAMPLE_PORTS, SAMPLE_LOOP_PITCH_X),
                    centered_index(level, SAMPLE_LOOP_LEVELS, 54.0) - 24.0,
                    SAMPLE_Z / 2.0 + 1.5,
                );
        }
    }
    cuts
}

fn sterile_handoff_port_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_sterile_handoff_port_cuts"));
    for port in 0..STERILE_HANDOFF_PORTS {
        cuts = cuts
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_sterile_handoff_port_cut_{port}"),
                8.5,
                SAMPLE_Y + 4.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(port, STERILE_HANDOFF_PORTS, 34.0),
                0.0,
                SAMPLE_Z - 14.0,
            );
    }
    cuts
}

fn sample_port_collars() -> Part {
    let mut collars = Part::empty(format!("{OUTPUT_PREFIX}_sample_port_collars"));
    for port in 0..SAMPLE_PORTS {
        let collar = centered_cylinder(
            format!("{OUTPUT_PREFIX}_bag_{port}_sample_port_collar"),
            16.0,
            10.0,
            32,
        )
        .translate(
            centered_index(port, SAMPLE_PORTS, 64.0),
            SAMPLE_Y / 2.0 - 30.0,
            SAMPLE_Z + 5.0,
        );
        let bore = centered_cylinder(
            format!("{OUTPUT_PREFIX}_bag_{port}_sample_port_bore_cut"),
            6.0,
            12.0,
            22,
        )
        .translate(
            centered_index(port, SAMPLE_PORTS, 64.0),
            SAMPLE_Y / 2.0 - 30.0,
            SAMPLE_Z + 6.0,
        );
        collars = collars + (collar - bore);
    }
    collars
}

fn viability_dock_lands() -> Part {
    let mut docks = Part::empty(format!("{OUTPUT_PREFIX}_viability_counter_dock_lands"));
    for dock in 0..VIABILITY_DOCKS {
        docks = docks
            + centered_cube(
                format!("{OUTPUT_PREFIX}_viability_counter_dock_land_{dock}"),
                54.0,
                26.0,
                6.0,
            )
            .translate(
                centered_index(dock, VIABILITY_DOCKS, 68.0),
                -SAMPLE_Y / 2.0 + 24.0,
                SAMPLE_Z + 3.0,
            );
    }
    docks
}

fn sample_timing_token_lands() -> Part {
    let mut tokens = Part::empty(format!("{OUTPUT_PREFIX}_sample_timing_token_lands"));
    for bag in 0..SOURCE_BAGS {
        tokens = tokens
            + centered_cube(
                format!("{OUTPUT_PREFIX}_bag_{bag}_sample_draw_timing_token_land"),
                42.0,
                14.0,
                5.0,
            )
            .translate(centered_index(bag, SOURCE_BAGS, 58.0), 24.0, SAMPLE_Z + 2.5);
    }
    tokens
}

fn bubble_foam_observation_windows() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_bubble_foam_observation_window_block"),
        BUBBLE_X,
        BUBBLE_Y,
        BUBBLE_Z,
    )
    .translate(0.0, 0.0, BUBBLE_Z / 2.0);

    body + observation_window_frames() + bubble_tick_ladders() + foam_trap_witness_chambers()
}

fn observation_window_frames() -> Part {
    let mut frames = Part::empty(format!(
        "{OUTPUT_PREFIX}_bubble_foam_observation_window_frames"
    ));
    for index in 0..OBSERVATION_WINDOWS {
        let x = centered_index(index % SOURCE_BAGS, SOURCE_BAGS, 118.0);
        let y = if index < SOURCE_BAGS { -46.0 } else { 48.0 };
        let frame = centered_cube(
            format!("{OUTPUT_PREFIX}_observation_window_frame_{index}"),
            74.0,
            36.0,
            10.0,
        )
        .translate(x, y, BUBBLE_Z + 5.0);
        let pane = centered_cube(
            format!("{OUTPUT_PREFIX}_observation_clear_pane_cut_{index}"),
            56.0,
            22.0,
            12.0,
        )
        .translate(x, y, BUBBLE_Z + 6.0);
        frames = frames + (frame - pane);
    }
    frames
}

fn bubble_tick_ladders() -> Part {
    let mut ticks = Part::empty(format!("{OUTPUT_PREFIX}_bubble_tick_ladders"));
    for bag in 0..SOURCE_BAGS {
        for tick in 0..BUBBLE_SCALE_TICKS {
            ticks = ticks
                + centered_cube(
                    format!("{OUTPUT_PREFIX}_bag_{bag}_bubble_tick_{tick}"),
                    if tick % 4 == 0 { 24.0 } else { 14.0 },
                    3.0,
                    4.0,
                )
                .translate(
                    centered_index(bag, SOURCE_BAGS, 118.0) - 38.0,
                    centered_index(tick, BUBBLE_SCALE_TICKS, 8.0),
                    BUBBLE_Z + 2.0,
                );
        }
    }
    ticks
}

fn foam_trap_witness_chambers() -> Part {
    let mut chambers = Part::empty(format!("{OUTPUT_PREFIX}_foam_trap_witness_chambers"));
    for bag in 0..FOAM_TRAP_WITNESSES {
        let boss = centered_cylinder(
            format!("{OUTPUT_PREFIX}_bag_{bag}_foam_trap_witness_chamber"),
            19.0,
            16.0,
            32,
        )
        .translate(
            centered_index(bag, FOAM_TRAP_WITNESSES, 104.0),
            BUBBLE_Y / 2.0 - 24.0,
            BUBBLE_Z + 8.0,
        );
        let vent = centered_cylinder(
            format!("{OUTPUT_PREFIX}_bag_{bag}_foam_trap_vent_cut"),
            6.5,
            18.0,
            20,
        )
        .translate(
            centered_index(bag, FOAM_TRAP_WITNESSES, 104.0),
            BUBBLE_Y / 2.0 - 24.0,
            BUBBLE_Z + 9.0,
        );
        chambers = chambers + (boss - vent);
    }
    chambers
}

fn foam_height_coupon_array() -> Part {
    let base = centered_cube(
        format!("{OUTPUT_PREFIX}_foam_height_coupon_array_base"),
        FOAM_X,
        FOAM_Y,
        FOAM_Z,
    )
    .translate(0.0, 0.0, FOAM_Z / 2.0);

    base - foam_coupon_slot_cuts() + foam_height_level_tabs() + removable_coupon_pull_tabs()
}

fn foam_coupon_slot_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_foam_coupon_slot_cuts"));
    for coupon in 0..FOAM_COUPONS {
        cuts = cuts
            + centered_cube(
                format!("{OUTPUT_PREFIX}_bag_{coupon}_foam_coupon_slot_cut"),
                50.0,
                118.0,
                18.0,
            )
            .translate(
                centered_index(coupon, FOAM_COUPONS, 66.0),
                0.0,
                FOAM_Z - 8.0,
            );
    }
    cuts
}

fn foam_height_level_tabs() -> Part {
    let mut tabs = Part::empty(format!("{OUTPUT_PREFIX}_foam_height_level_tabs"));
    for coupon in 0..FOAM_COUPONS {
        let x = centered_index(coupon, FOAM_COUPONS, 66.0) + 34.0;
        for level in 0..FOAM_HEIGHT_LEVELS {
            tabs = tabs
                + centered_cube(
                    format!("{OUTPUT_PREFIX}_foam_coupon_{coupon}_height_level_{level}"),
                    22.0,
                    4.0,
                    4.0,
                )
                .translate(
                    x,
                    centered_index(level, FOAM_HEIGHT_LEVELS, 22.0),
                    FOAM_Z + 2.0,
                );
        }
    }
    tabs
}

fn removable_coupon_pull_tabs() -> Part {
    let mut tabs = Part::empty(format!("{OUTPUT_PREFIX}_removable_foam_coupon_pull_tabs"));
    for coupon in 0..FOAM_COUPONS {
        tabs = tabs
            + centered_cube(
                format!("{OUTPUT_PREFIX}_foam_coupon_{coupon}_pull_tab"),
                42.0,
                16.0,
                6.0,
            )
            .translate(
                centered_index(coupon, FOAM_COUPONS, 66.0),
                FOAM_Y / 2.0 - 18.0,
                FOAM_Z + 3.0,
            );
    }
    tabs
}

fn tubing_strain_relief_clamp_comb() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_tubing_strain_relief_clamp_comb_body"),
        TUBING_X,
        TUBING_Y,
        TUBING_Z,
    )
    .translate(0.0, 0.0, TUBING_Z / 2.0);

    body - tubing_channel_cuts() + tubing_clamp_latches() + route_number_tabs()
}

fn tubing_channel_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_tubing_channel_cuts"));
    for channel in 0..TUBING_CHANNELS {
        cuts = cuts
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_closed_tubing_channel_cut_{channel}"),
                TUBING_CHANNEL_D / 2.0,
                TUBING_Y + 4.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(channel, TUBING_CHANNELS, TUBING_CHANNEL_PITCH),
                0.0,
                TUBING_Z - 10.0,
            );
    }
    cuts
}

fn tubing_clamp_latches() -> Part {
    let mut latches = Part::empty(format!("{OUTPUT_PREFIX}_tubing_clamp_latches"));
    for channel in 0..TUBING_CHANNELS {
        latches = latches
            + centered_cube(
                format!("{OUTPUT_PREFIX}_tubing_channel_{channel}_snap_latch_land"),
                24.0,
                10.0,
                7.0,
            )
            .translate(
                centered_index(channel, TUBING_CHANNELS, TUBING_CHANNEL_PITCH),
                -TUBING_Y / 2.0 + 14.0,
                TUBING_Z + 3.5,
            );
    }
    latches
}

fn route_number_tabs() -> Part {
    let mut tabs = Part::empty(format!("{OUTPUT_PREFIX}_tubing_route_number_tabs"));
    for channel in 0..TUBING_CHANNELS {
        tabs = tabs
            + centered_cube(
                format!("{OUTPUT_PREFIX}_tubing_route_number_tab_{channel}"),
                22.0,
                12.0,
                4.0,
            )
            .translate(
                centered_index(channel, TUBING_CHANNELS, TUBING_CHANNEL_PITCH),
                TUBING_Y / 2.0 + 7.0,
                TUBING_Z + 2.0,
            );
    }
    tabs
}

fn identity_custody_barcode_lands() -> Part {
    let base = centered_cube(
        format!("{OUTPUT_PREFIX}_identity_custody_barcode_land_plate"),
        IDENTITY_X,
        IDENTITY_Y,
        IDENTITY_Z,
    )
    .translate(0.0, 0.0, IDENTITY_Z / 2.0);

    base + barcode_land_array() + rfid_land_array() + bag_lot_card_slots()
}

fn barcode_land_array() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_barcode_land_array"));
    for index in 0..BARCODE_LANDS {
        let row = index / 4;
        let col = index % 4;
        lands = lands
            + centered_cube(
                format!("{OUTPUT_PREFIX}_barcode_land_{index}"),
                44.0,
                14.0,
                4.0,
            )
            .translate(
                centered_index(col, 4, 60.0),
                -IDENTITY_Y / 2.0 + 22.0 + row as f64 * 28.0,
                IDENTITY_Z + 2.0,
            );
    }
    lands
}

fn rfid_land_array() -> Part {
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_rfid_land_array"));
    for index in 0..RFID_LANDS {
        lands = lands
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_bag_{index}_rfid_token_land"),
                13.0,
                4.0,
                28,
            )
            .translate(
                centered_index(index, RFID_LANDS, 62.0),
                46.0,
                IDENTITY_Z + 2.0,
            );
    }
    lands
}

fn bag_lot_card_slots() -> Part {
    let mut slots = Part::empty(format!("{OUTPUT_PREFIX}_bag_lot_card_slots"));
    for index in 0..BAG_LOT_CARD_SLOTS {
        slots = slots
            + centered_cube(
                format!("{OUTPUT_PREFIX}_bag_{index}_lot_card_slot_land"),
                58.0,
                12.0,
                5.0,
            )
            .translate(
                centered_index(index, BAG_LOT_CARD_SLOTS, 66.0),
                -46.0,
                IDENTITY_Z + 2.5,
            );
    }
    slots
}

fn release_hold_reject_evidence_gate() -> Part {
    let body = centered_cube(
        format!("{OUTPUT_PREFIX}_release_hold_reject_evidence_gate_body"),
        GATE_X,
        GATE_Y,
        GATE_Z,
    )
    .translate(0.0, 0.0, GATE_Z / 2.0);

    body - evidence_slot_cuts()
        + disposition_lane_dividers()
        + interlock_pin_lands()
        + evidence_token_labels()
}

fn evidence_slot_cuts() -> Part {
    let mut cuts = Part::empty(format!("{OUTPUT_PREFIX}_disposition_evidence_slot_cuts"));
    for (lane_index, lane_name) in DISPOSITION_LANES.iter().enumerate() {
        for slot in 0..EVIDENCE_SLOTS_PER_LANE {
            cuts = cuts
                + centered_cube(
                    format!("{OUTPUT_PREFIX}_{lane_name}_bag_{slot}_evidence_slot_cut"),
                    EVIDENCE_SLOT_X,
                    EVIDENCE_SLOT_Y,
                    18.0,
                )
                .translate(
                    centered_index(slot, EVIDENCE_SLOTS_PER_LANE, 72.0),
                    disposition_lane_y(lane_index),
                    GATE_Z - 8.0,
                );
        }
    }
    cuts
}

fn disposition_lane_dividers() -> Part {
    let mut dividers = Part::empty(format!("{OUTPUT_PREFIX}_disposition_lane_dividers"));
    for index in 0..=DISPOSITION_LANES.len() {
        dividers = dividers
            + centered_cube(
                format!("{OUTPUT_PREFIX}_disposition_lane_divider_{index}"),
                GATE_X - 38.0,
                5.0,
                7.0,
            )
            .translate(
                0.0,
                -GATE_Y / 2.0 + 32.0 + index as f64 * 46.0,
                GATE_Z + 3.5,
            );
    }
    dividers
}

fn interlock_pin_lands() -> Part {
    let mut pins = Part::empty(format!("{OUTPUT_PREFIX}_gate_interlock_pin_lands"));
    for index in 0..GATE_INTERLOCK_PINS {
        pins = pins
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_gate_interlock_pin_land_{index}"),
                9.0,
                7.0,
                24,
            )
            .translate(
                centered_index(index % 3, 3, 110.0),
                if index < 3 {
                    -GATE_Y / 2.0 + 18.0
                } else {
                    GATE_Y / 2.0 - 18.0
                },
                GATE_Z + 3.5,
            );
    }
    pins
}

fn evidence_token_labels() -> Part {
    let mut labels = Part::empty(format!("{OUTPUT_PREFIX}_evidence_token_label_lands"));
    for (lane_index, lane_name) in DISPOSITION_LANES.iter().enumerate() {
        labels = labels
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{lane_name}_lane_label_land"),
                66.0,
                16.0,
                4.0,
            )
            .translate(
                -GATE_X / 2.0 + 44.0,
                disposition_lane_y(lane_index),
                GATE_Z + 2.0,
            );
    }
    labels
}

fn evidence_camera_bridge() -> Part {
    let left_front = bridge_post(-BRIDGE_SPAN_X / 2.0, -310.0, "left_front");
    let right_front = bridge_post(BRIDGE_SPAN_X / 2.0, -310.0, "right_front");
    let left_rear = bridge_post(-BRIDGE_SPAN_X / 2.0, 365.0, "left_rear");
    let right_rear = bridge_post(BRIDGE_SPAN_X / 2.0, 365.0, "right_rear");
    let beam = centered_cube(
        format!("{OUTPUT_PREFIX}_evidence_camera_bridge_cross_beam"),
        BRIDGE_SPAN_X + 90.0,
        36.0,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, 28.0, BRIDGE_POST_Z + BRIDGE_BEAM_Z / 2.0);

    left_front
        + right_front
        + left_rear
        + right_rear
        + beam
        + camera_pod_array()
        + bridge_light_bars()
        + bridge_fiducial_pads()
}

fn bridge_post(x: f64, y: f64, name: &str) -> Part {
    centered_cube(
        format!("{OUTPUT_PREFIX}_camera_bridge_{name}_post"),
        34.0,
        34.0,
        BRIDGE_POST_Z,
    )
    .translate(x, y, BRIDGE_POST_Z / 2.0)
}

fn camera_pod_array() -> Part {
    let mut pods = Part::empty(format!("{OUTPUT_PREFIX}_camera_pod_array"));
    for index in 0..CAMERA_PODS {
        let x = centered_index(index, CAMERA_PODS, 235.0);
        let pod = centered_cube(
            format!("{OUTPUT_PREFIX}_camera_pod_{index}_mount"),
            78.0,
            44.0,
            28.0,
        )
        .translate(x, 28.0, BRIDGE_POST_Z - 22.0);
        let lens = centered_cylinder(
            format!("{OUTPUT_PREFIX}_camera_pod_{index}_lens_axis_cut"),
            12.0,
            30.0,
            28,
        )
        .translate(x, 28.0, BRIDGE_POST_Z - 21.0);
        pods = pods + (pod - lens);
    }
    pods
}

fn bridge_light_bars() -> Part {
    let mut bars = Part::empty(format!("{OUTPUT_PREFIX}_evidence_light_bars"));
    for index in 0..LIGHT_BARS {
        bars = bars
            + centered_cube(
                format!("{OUTPUT_PREFIX}_evidence_light_bar_{index}"),
                180.0,
                12.0,
                10.0,
            )
            .translate(
                centered_index(index, LIGHT_BARS, 250.0),
                -6.0,
                BRIDGE_POST_Z - 48.0,
            );
    }
    bars
}

fn bridge_fiducial_pads() -> Part {
    let mut pads = Part::empty(format!("{OUTPUT_PREFIX}_bridge_fiducial_pads"));
    for (index, (x, y)) in fiducial_positions().iter().enumerate() {
        let boss = centered_cylinder(
            format!("{OUTPUT_PREFIX}_bridge_fiducial_pad_{index}"),
            15.0,
            5.0,
            28,
        )
        .translate(*x, *y, DECK_Z + 66.0);
        let dot = centered_cylinder(
            format!("{OUTPUT_PREFIX}_bridge_fiducial_center_cut_{index}"),
            4.0,
            6.0,
            18,
        )
        .translate(*x, *y, DECK_Z + 67.0);
        pads = pads + (boss - dot);
    }
    pads
}

fn robot_service_keepouts() -> Part {
    let front = centered_cube(
        format!("{OUTPUT_PREFIX}_front_robot_pick_keepout_gauge"),
        STATION_X - 120.0,
        KEEP_OUT_RAIL,
        90.0,
    )
    .translate(0.0, -STATION_Y / 2.0 - FRONT_ROBOT_CLEARANCE, 45.0);
    let rear = centered_cube(
        format!("{OUTPUT_PREFIX}_rear_service_keepout_gauge"),
        STATION_X - 120.0,
        KEEP_OUT_RAIL,
        90.0,
    )
    .translate(0.0, STATION_Y / 2.0 + REAR_SERVICE_CLEARANCE, 45.0);
    let left = centered_cube(
        format!("{OUTPUT_PREFIX}_left_bag_load_keepout_gauge"),
        KEEP_OUT_RAIL,
        STATION_Y - 110.0,
        90.0,
    )
    .translate(-STATION_X / 2.0 - LEFT_BAG_LOAD_CLEARANCE, 0.0, 45.0);
    let right = centered_cube(
        format!("{OUTPUT_PREFIX}_right_analyzer_handoff_keepout_gauge"),
        KEEP_OUT_RAIL,
        STATION_Y - 110.0,
        90.0,
    )
    .translate(STATION_X / 2.0 + RIGHT_ANALYZER_CLEARANCE, 0.0, 45.0);
    let overhead = centered_cube(
        format!("{OUTPUT_PREFIX}_overhead_rocker_motion_keepout_gauge"),
        BAG_CRADLE_X + 70.0,
        28.0,
        KEEP_OUT_RAIL,
    )
    .translate(
        BAG_CRADLE_CENTER.0,
        BAG_CRADLE_CENTER.1,
        DECK_Z + OVERHEAD_ROCKER_CLEARANCE_Z,
    );
    front + rear + left + right + overhead
}

fn mount_positions() -> [(f64, f64); MOUNT_BOSSES] {
    [
        (-STATION_X / 2.0 + 66.0, -STATION_Y / 2.0 + 64.0),
        (-STATION_X / 2.0 + 66.0, STATION_Y / 2.0 - 64.0),
        (STATION_X / 2.0 - 66.0, -STATION_Y / 2.0 + 64.0),
        (STATION_X / 2.0 - 66.0, STATION_Y / 2.0 - 64.0),
        (-STATION_X / 2.0 + 66.0, 0.0),
        (STATION_X / 2.0 - 66.0, 0.0),
        (0.0, -STATION_Y / 2.0 + 64.0),
        (0.0, STATION_Y / 2.0 - 64.0),
    ]
}

fn datum_positions() -> [(f64, f64); 4] {
    [
        (-STATION_X / 2.0 + 114.0, -STATION_Y / 2.0 + 104.0),
        (STATION_X / 2.0 - 114.0, -STATION_Y / 2.0 + 104.0),
        (-STATION_X / 2.0 + 114.0, STATION_Y / 2.0 - 104.0),
        (STATION_X / 2.0 - 114.0, STATION_Y / 2.0 - 104.0),
    ]
}

fn fiducial_positions() -> [(f64, f64); FIDUCIAL_PADS] {
    [
        (-620.0, -305.0),
        (-245.0, -305.0),
        (125.0, -305.0),
        (495.0, -305.0),
        (-620.0, 360.0),
        (495.0, 360.0),
    ]
}

fn layout_rects() -> [Rect; 8] {
    [
        Rect {
            name: "bag_cradle",
            center: BAG_CRADLE_CENTER,
            x: BAG_CRADLE_X,
            y: BAG_CRADLE_Y,
        },
        Rect {
            name: "logger_witnesses",
            center: LOGGER_CENTER,
            x: LOGGER_X,
            y: LOGGER_Y,
        },
        Rect {
            name: "sampling_handoff",
            center: SAMPLE_CENTER,
            x: SAMPLE_X,
            y: SAMPLE_Y,
        },
        Rect {
            name: "bubble_observation",
            center: BUBBLE_CENTER,
            x: BUBBLE_X,
            y: BUBBLE_Y,
        },
        Rect {
            name: "foam_coupon_array",
            center: FOAM_CENTER,
            x: FOAM_X,
            y: FOAM_Y,
        },
        Rect {
            name: "tubing_comb",
            center: TUBING_CENTER,
            x: TUBING_X,
            y: TUBING_Y,
        },
        Rect {
            name: "identity_custody",
            center: IDENTITY_CENTER,
            x: IDENTITY_X,
            y: IDENTITY_Y,
        },
        Rect {
            name: "disposition_gate",
            center: GATE_CENTER,
            x: GATE_X,
            y: GATE_Y,
        },
    ]
}

fn disposition_lane_y(lane_index: usize) -> f64 {
    centered_index(lane_index, DISPOSITION_LANES.len(), 48.0)
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn assert_design_constraints() {
    assert!(STATION_X >= 1500.0);
    assert!(STATION_Y >= 900.0);
    assert_eq!(SOURCE_BAGS, 4);
    assert!(BAG_POCKET_X >= 400.0);
    assert!(MAX_ROCK_ANGLE_DEG <= 8.0);
    assert!(ACCEL_LIMIT_G <= 0.15);
    assert_eq!(SAMPLE_LOOP_LEVELS, 3);
    assert!(OBSERVATION_WINDOWS >= SOURCE_BAGS * 2);
    assert_eq!(DISPOSITION_LANES, ["release", "hold", "reject"]);
    assert!(FRONT_ROBOT_CLEARANCE >= 350.0);
    assert!(OVERHEAD_ROCKER_CLEARANCE_Z > BRIDGE_BEAM_Z + BRIDGE_POST_Z / 2.0);
    for rect in layout_rects() {
        assert!(rect.fits_inside_station(), "{} outside station", rect.name);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 13);
        for path in OUTPUTS {
            assert!(path
                .starts_with("output/closed_cell_source_bag_agitation_shear_threshold_station_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn required_feature_outputs_are_present() {
        for feature in REQUIRED_FEATURES {
            assert!(
                OUTPUTS.iter().any(|path| path.contains(feature)),
                "missing output for {feature}"
            );
        }
    }

    #[test]
    fn dimensions_and_layout_keep_modules_inside_containment() {
        assert_design_constraints();
        let rects = layout_rects();
        assert!(rects.iter().all(|rect| rect.fits_inside_station()));
        for (index, first) in rects.iter().enumerate() {
            for second in rects.iter().skip(index + 1) {
                assert!(
                    !first.overlaps(*second),
                    "{} overlaps {}",
                    first.name,
                    second.name
                );
            }
        }
    }

    #[test]
    fn rocking_envelope_is_low_shear_and_instrumented() {
        assert!(MAX_ROCK_ANGLE_DEG <= 8.0);
        assert!(ACCEL_LIMIT_G <= 0.15);
        assert!(ROCKER_SEGMENTS >= 17);
        assert_eq!(ROCKER_LIMIT_STOPS, 4);
        assert_eq!(SHEAR_WARNING_TOKENS, 5);
        assert!(ROCKER_RADIUS > BAG_POCKET_Y);
    }

    #[test]
    fn sampling_bubble_foam_and_disposition_capacity_cover_each_bag() {
        assert_eq!(SOURCE_BAGS, 4);
        assert_eq!(SAMPLE_PORTS, SOURCE_BAGS);
        assert_eq!(VIABILITY_DOCKS, SOURCE_BAGS);
        assert_eq!(STERILE_HANDOFF_PORTS, SOURCE_BAGS * 2);
        assert_eq!(OBSERVATION_WINDOWS, SOURCE_BAGS * 2);
        assert_eq!(FOAM_COUPONS, SOURCE_BAGS);
        assert_eq!(EVIDENCE_SLOTS_PER_LANE, SOURCE_BAGS);
        assert_eq!(DISPOSITION_LANES, ["release", "hold", "reject"]);
    }

    #[test]
    fn witness_blocks_and_camera_bridge_support_evidence_capture() {
        assert!(ACCELEROMETER_NESTS >= SOURCE_BAGS + 1);
        assert_eq!(TILT_WITNESS_BLOCKS, 3);
        assert!(TILT_SCALE_TICKS % 2 == 1);
        assert_eq!(CAMERA_PODS, 5);
        assert_eq!(LIGHT_BARS, 4);
        assert_eq!(FIDUCIAL_PADS, fiducial_positions().len());
        assert!(OVERHEAD_ROCKER_CLEARANCE_Z > BRIDGE_POST_Z);
    }
}
