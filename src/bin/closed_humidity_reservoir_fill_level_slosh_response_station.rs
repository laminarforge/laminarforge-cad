use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed humidity reservoir fill-level and slosh response station.
//
// This standalone CAD generator models a no-cell validation fixture for an
// incubator humidity reservoir under rack motion, door recovery, and condensate
// return challenges. It reserves a reservoir surrogate cradle, fill-level
// witness ladder, slosh baffle coupons, RH/dewpoint logger pockets,
// overflow/underfill token rail, condensate return witness, contamination
// coupon pockets, and release/hold/reject gates. It is mechanical validation
// packaging only; fill recipes, biological controls, and acceptance criteria
// remain external validation records.

const OUTPUT_PREFIX: &str = "closed_humidity_reservoir_fill_level_slosh_response_station";

const OUTPUTS: [&str; 12] = [
    "output/closed_humidity_reservoir_fill_level_slosh_response_station_base_motion_recovery_deck.stl",
    "output/closed_humidity_reservoir_fill_level_slosh_response_station_reservoir_surrogate_cradle.stl",
    "output/closed_humidity_reservoir_fill_level_slosh_response_station_fill_level_witness_ladder.stl",
    "output/closed_humidity_reservoir_fill_level_slosh_response_station_slosh_baffle_coupon_cartridge.stl",
    "output/closed_humidity_reservoir_fill_level_slosh_response_station_rh_dewpoint_logger_pockets.stl",
    "output/closed_humidity_reservoir_fill_level_slosh_response_station_overflow_underfill_token_rail.stl",
    "output/closed_humidity_reservoir_fill_level_slosh_response_station_condensate_return_witness.stl",
    "output/closed_humidity_reservoir_fill_level_slosh_response_station_contamination_coupon_pockets.stl",
    "output/closed_humidity_reservoir_fill_level_slosh_response_station_rack_motion_door_recovery_witnesses.stl",
    "output/closed_humidity_reservoir_fill_level_slosh_response_station_release_hold_reject_gates.stl",
    "output/closed_humidity_reservoir_fill_level_slosh_response_station_robot_service_keepout_gauges.stl",
    "output/closed_humidity_reservoir_fill_level_slosh_response_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "reservoir_surrogate_cradle",
    "fill_level_witness_ladder",
    "slosh_baffle_coupons",
    "rh_dewpoint_logger_pockets",
    "overflow_underfill_token_rail",
    "condensate_return_witness",
    "contamination_coupon_pockets",
    "rack_motion_door_recovery_witnesses",
    "release_hold_reject_gates",
    "robot_service_keepout_gauges",
    "named_stl_outputs",
];

const STATION_X: f64 = 1420.0;
const STATION_Y: f64 = 860.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 46.0;
const BASIN_DEPTH: f64 = 8.0;
const SOCKET_DEPTH: f64 = 6.0;
const MOUNT_SLOT_COUNT: usize = 8;
const DATUM_TARGET_COUNT: usize = 4;

const RESERVOIR_X: f64 = 540.0;
const RESERVOIR_Y: f64 = 290.0;
const RESERVOIR_Z: f64 = 66.0;
const RESERVOIR_POS: (f64, f64) = (-390.0, 175.0);
const RESERVOIR_CAVITY_X: f64 = 435.0;
const RESERVOIR_CAVITY_Y: f64 = 210.0;
const RESERVOIR_CAVITY_DEPTH: f64 = 42.0;
const RESERVOIR_WORKING_DEPTH: f64 = 28.0;
const RESERVOIR_DRAIN_PORTS: usize = 2;
const RESERVOIR_GRIP_HANDLES: usize = 4;
const RESERVOIR_TILT_INDEX_COUNT: usize = 5;

const LADDER_X: f64 = 480.0;
const LADDER_Y: f64 = 130.0;
const LADDER_Z: f64 = 54.0;
const LADDER_POS: (f64, f64) = (-405.0, -190.0);
const FILL_LEVEL_RUNG_COUNT: usize = 8;
const FILL_LEVEL_RUNG_PITCH: f64 = 44.0;
const WITNESS_FLAG_COUNT: usize = 4;
const LADDER_SIGHT_SLOT_COUNT: usize = 6;

const BAFFLE_X: f64 = 390.0;
const BAFFLE_Y: f64 = 245.0;
const BAFFLE_Z: f64 = 58.0;
const BAFFLE_POS: (f64, f64) = (230.0, 180.0);
const SLOSH_BAFFLE_COUNT: usize = 7;
const BAFFLE_COUPONS_PER_RAIL: usize = 2;
const SLOSH_COUPON_COUNT: usize = SLOSH_BAFFLE_COUNT * BAFFLE_COUPONS_PER_RAIL;
const BAFFLE_DRAIN_SLOT_COUNT: usize = SLOSH_BAFFLE_COUNT * 2;
const BAFFLE_LOCK_TAB_COUNT: usize = 4;

const LOGGER_X: f64 = 285.0;
const LOGGER_Y: f64 = 160.0;
const LOGGER_Z: f64 = 48.0;
const LOGGER_POS: (f64, f64) = (505.0, -48.0);
const LOGGER_POCKET_COUNT: usize = 5;
const LOGGER_CABLE_CLIP_COUNT: usize = 6;
const DEWPOINT_REFERENCE_WELL_COUNT: usize = 2;
const LOGGER_POCKET_X: f64 = 42.0;
const LOGGER_POCKET_Y: f64 = 32.0;

const TOKEN_X: f64 = 440.0;
const TOKEN_Y: f64 = 100.0;
const TOKEN_Z: f64 = 26.0;
const TOKEN_POS: (f64, f64) = (440.0, -245.0);
const TOKEN_LANE_COUNT: usize = 3;
const TOKENS_PER_LANE: usize = 4;
const TOKEN_COUNT: usize = TOKEN_LANE_COUNT * TOKENS_PER_LANE;
const TOKEN_D: f64 = 24.0;

const CONDENSATE_X: f64 = 260.0;
const CONDENSATE_Y: f64 = 140.0;
const CONDENSATE_Z: f64 = 42.0;
const CONDENSATE_POS: (f64, f64) = (-5.0, -60.0);
const RETURN_CHANNEL_COUNT: usize = 5;
const RETURN_CUP_COUNT: usize = 3;
const RETURN_DYE_WITNESS_COUNT: usize = 4;
const RETURN_CUP_D: f64 = 42.0;
const CONDENSATE_BASIN_DEPTH: f64 = 22.0;

const CONTAM_X: f64 = 270.0;
const CONTAM_Y: f64 = 104.0;
const CONTAM_Z: f64 = 34.0;
const CONTAM_POS: (f64, f64) = (-20.0, -248.0);
const CONTAMINATION_COUPON_ROWS: usize = 3;
const CONTAMINATION_COUPON_COLS: usize = 4;
const CONTAMINATION_COUPON_COUNT: usize = CONTAMINATION_COUPON_ROWS * CONTAMINATION_COUPON_COLS;
const CONTAM_COUPON_SLOT_X: f64 = 44.0;
const CONTAM_COUPON_SLOT_Y: f64 = 18.0;

const RECOVERY_X: f64 = 420.0;
const RECOVERY_Y: f64 = 70.0;
const RECOVERY_Z: f64 = 32.0;
const RECOVERY_POS: (f64, f64) = (425.0, 350.0);
const RACK_MOTION_STEP_COUNT: usize = 6;
const DOOR_RECOVERY_EVENT_COUNT: usize = 5;
const RECOVERY_DATUM_PUCK_COUNT: usize = 4;

const GATE_X: f64 = 360.0;
const GATE_Y: f64 = 72.0;
const GATE_Z: f64 = 34.0;
const GATE_POS: (f64, f64) = (150.0, -356.0);
const GATE_COUNT: usize = 3;
const GATE_TOKEN_SLOTS_PER_GATE: usize = 3;
const GATE_TOKEN_SLOT_COUNT: usize = GATE_COUNT * GATE_TOKEN_SLOTS_PER_GATE;

const KEEP_OUT_X: f64 = 1340.0;
const KEEP_OUT_Y: f64 = 810.0;
const KEEP_OUT_Z: f64 = 6.0;
const FRONT_ROBOT_CLEARANCE_Y: f64 = 260.0;
const REAR_SERVICE_CLEARANCE_Y: f64 = 205.0;
const RESERVOIR_LIFT_CLEARANCE_Z: f64 = 310.0;
const LOGGER_CABLE_SERVICE_X: f64 = 178.0;
const KEEP_OUT_GAUGE_COUNT: usize = 5;

const LABEL_Z: f64 = 2.4;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_deck(self) -> bool {
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

    let base = base_motion_recovery_deck();
    export(OUTPUTS[0], &base);

    let cradle = reservoir_surrogate_cradle();
    export(OUTPUTS[1], &cradle);

    let ladder = fill_level_witness_ladder();
    export(OUTPUTS[2], &ladder);

    let baffles = slosh_baffle_coupon_cartridge();
    export(OUTPUTS[3], &baffles);

    let loggers = rh_dewpoint_logger_pockets();
    export(OUTPUTS[4], &loggers);

    let tokens = overflow_underfill_token_rail();
    export(OUTPUTS[5], &tokens);

    let condensate = condensate_return_witness();
    export(OUTPUTS[6], &condensate);

    let contamination = contamination_coupon_pockets();
    export(OUTPUTS[7], &contamination);

    let recovery = rack_motion_door_recovery_witnesses();
    export(OUTPUTS[8], &recovery);

    let gates = release_hold_reject_gates();
    export(OUTPUTS[9], &gates);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[10], &keepouts);

    let assembly =
        base + cradle.translate(
            RESERVOIR_POS.0,
            RESERVOIR_POS.1,
            module_on_deck_z(RESERVOIR_Z),
        ) + ladder.translate(LADDER_POS.0, LADDER_POS.1, module_on_deck_z(LADDER_Z))
            + baffles.translate(BAFFLE_POS.0, BAFFLE_POS.1, module_on_deck_z(BAFFLE_Z))
            + loggers.translate(LOGGER_POS.0, LOGGER_POS.1, module_on_deck_z(LOGGER_Z))
            + tokens.translate(TOKEN_POS.0, TOKEN_POS.1, module_on_deck_z(TOKEN_Z))
            + condensate.translate(
                CONDENSATE_POS.0,
                CONDENSATE_POS.1,
                module_on_deck_z(CONDENSATE_Z),
            )
            + contamination.translate(CONTAM_POS.0, CONTAM_POS.1, module_on_deck_z(CONTAM_Z))
            + recovery.translate(RECOVERY_POS.0, RECOVERY_POS.1, module_on_deck_z(RECOVERY_Z))
            + gates.translate(GATE_POS.0, GATE_POS.1, module_on_deck_z(GATE_Z))
            + keepouts.translate(0.0, 0.0, DECK_Z / 2.0 + KEEP_OUT_Z / 2.0);
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed humidity reservoir fill-level/slosh response station:");
    println!("  Footprint:             {STATION_X:.0}mm x {STATION_Y:.0}mm containment deck");
    println!(
        "  Reservoir surrogate:   {:.0}mL working volume, {RESERVOIR_DRAIN_PORTS} drain witnesses, {RESERVOIR_TILT_INDEX_COUNT} rack-motion tilt indices",
        reservoir_working_volume_ml()
    );
    println!(
        "  Fill-level ladder:     {FILL_LEVEL_RUNG_COUNT} witness rungs, {WITNESS_FLAG_COUNT} removable flags, and {LADDER_SIGHT_SLOT_COUNT} sight slots"
    );
    println!(
        "  Slosh response:        {SLOSH_BAFFLE_COUNT} baffle coupons, {SLOSH_COUPON_COUNT} wetted coupon lands, {BAFFLE_DRAIN_SLOT_COUNT} drain slots"
    );
    println!(
        "  Recovery measurement:  {LOGGER_POCKET_COUNT} RH/dewpoint logger pockets, {TOKEN_COUNT} overflow/underfill tokens, {DOOR_RECOVERY_EVENT_COUNT} door recovery events"
    );
    println!(
        "  Condensate/control:    {RETURN_CHANNEL_COUNT} return channels, {RETURN_CUP_COUNT} witness cups, {CONTAMINATION_COUPON_COUNT} contamination coupon pockets, {GATE_COUNT} disposition gates"
    );
    println!("  Required features:     {}", REQUIRED_FEATURES.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn module_on_deck_z(module_z: f64) -> f64 {
    DECK_Z / 2.0 + module_z / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 12);
    assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
    assert_eq!(REQUIRED_FEATURES.len(), 11);
    assert_eq!(MOUNT_SLOT_COUNT, mount_slot_positions().len());
    assert_eq!(DATUM_TARGET_COUNT, datum_positions().len());
    assert_eq!(RESERVOIR_GRIP_HANDLES, 4);
    assert_eq!(
        SLOSH_COUPON_COUNT,
        SLOSH_BAFFLE_COUNT * BAFFLE_COUPONS_PER_RAIL
    );
    assert_eq!(BAFFLE_DRAIN_SLOT_COUNT, SLOSH_BAFFLE_COUNT * 2);
    assert_eq!(BAFFLE_LOCK_TAB_COUNT, 4);
    assert_eq!(TOKEN_COUNT, TOKEN_LANE_COUNT * TOKENS_PER_LANE);
    assert_eq!(
        CONTAMINATION_COUPON_COUNT,
        CONTAMINATION_COUPON_ROWS * CONTAMINATION_COUPON_COLS
    );
    assert_eq!(
        GATE_TOKEN_SLOT_COUNT,
        GATE_COUNT * GATE_TOKEN_SLOTS_PER_GATE
    );
    assert_eq!(KEEP_OUT_GAUGE_COUNT, 5);
    assert!(RESERVOIR_WORKING_DEPTH < RESERVOIR_CAVITY_DEPTH);
    assert!(reservoir_working_volume_ml() > underfill_trip_volume_ml());
    assert!(reservoir_working_volume_ml() > worst_case_slosh_surge_ml());
    assert!(containment_freeboard_ml() > worst_case_slosh_surge_ml());
    assert!(condensate_return_capacity_ml() > door_recovery_condensate_challenge_ml());
    assert!(front_robot_clearance() >= FRONT_ROBOT_CLEARANCE_Y);
    assert!(rear_service_clearance() >= REAR_SERVICE_CLEARANCE_Y);
    assert!(logger_cable_service_clearance() >= LOGGER_CABLE_SERVICE_X);

    for rect in module_rects() {
        assert!(rect.fits_inside_deck(), "{} exceeds deck", rect.name);
    }

    let rects = module_rects();
    for i in 0..rects.len() {
        for j in (i + 1)..rects.len() {
            assert!(
                !rects[i].overlaps(rects[j]),
                "{} overlaps {}",
                rects[i].name,
                rects[j].name
            );
        }
    }
}

fn module_rects() -> [Rect; 9] {
    [
        Rect {
            name: "reservoir_surrogate_cradle",
            center: RESERVOIR_POS,
            x: RESERVOIR_X,
            y: RESERVOIR_Y,
        },
        Rect {
            name: "fill_level_witness_ladder",
            center: LADDER_POS,
            x: LADDER_X,
            y: LADDER_Y,
        },
        Rect {
            name: "slosh_baffle_coupon_cartridge",
            center: BAFFLE_POS,
            x: BAFFLE_X,
            y: BAFFLE_Y,
        },
        Rect {
            name: "rh_dewpoint_logger_pockets",
            center: LOGGER_POS,
            x: LOGGER_X,
            y: LOGGER_Y,
        },
        Rect {
            name: "overflow_underfill_token_rail",
            center: TOKEN_POS,
            x: TOKEN_X,
            y: TOKEN_Y,
        },
        Rect {
            name: "condensate_return_witness",
            center: CONDENSATE_POS,
            x: CONDENSATE_X,
            y: CONDENSATE_Y,
        },
        Rect {
            name: "contamination_coupon_pockets",
            center: CONTAM_POS,
            x: CONTAM_X,
            y: CONTAM_Y,
        },
        Rect {
            name: "rack_motion_door_recovery_witnesses",
            center: RECOVERY_POS,
            x: RECOVERY_X,
            y: RECOVERY_Y,
        },
        Rect {
            name: "release_hold_reject_gates",
            center: GATE_POS,
            x: GATE_X,
            y: GATE_Y,
        },
    ]
}

fn base_motion_recovery_deck() -> Part {
    let deck = centered_cube(
        "humidity_slosh_base_motion_recovery_deck",
        STATION_X,
        STATION_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let basin = centered_cube(
        "humidity_slosh_secondary_containment_basin_cut",
        STATION_X - 2.0 * (RIM_W + 48.0),
        STATION_Y - 2.0 * (RIM_W + 42.0),
        BASIN_DEPTH + 0.8,
    )
    .translate(0.0, -8.0, DECK_Z - BASIN_DEPTH / 2.0);
    let front_drain = centered_cylinder(
        "humidity_slosh_base_front_drain_placeholder_cut",
        8.5,
        64.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        -STATION_X / 2.0 + 116.0,
        -STATION_Y / 2.0 + 28.0,
        DECK_Z - 7.0,
    );

    deck - basin - front_drain - module_locator_sockets() - deck_mount_slots()
        + perimeter_rims()
        + motion_axis_witness_ribs()
        + deck_datum_targets()
        + condensate_grade_bars()
}

fn module_locator_sockets() -> Part {
    let mut sockets = Part::empty("humidity_slosh_module_locator_sockets");
    for rect in module_rects() {
        sockets = sockets
            + centered_cube(
                format!("humidity_slosh_{}_socket", rect.name),
                rect.x + 10.0,
                rect.y + 10.0,
                SOCKET_DEPTH + 0.5,
            )
            .translate(rect.center.0, rect.center.1, DECK_Z - SOCKET_DEPTH / 2.0);
    }
    sockets
}

fn deck_mount_slots() -> Part {
    let mut slots = Part::empty("humidity_slosh_deck_mount_slots");
    for (i, (x, y)) in mount_slot_positions().into_iter().enumerate() {
        slots = slots
            + centered_cylinder(
                format!("humidity_slosh_m6_mount_clearance_{i}"),
                3.4,
                DECK_Z + 4.0,
                28,
            )
            .translate(x, y, DECK_Z / 2.0)
            + centered_cube(
                format!("humidity_slosh_mount_slot_relief_{i}"),
                30.0,
                8.0,
                DECK_Z + 4.0,
            )
            .translate(x, y, DECK_Z / 2.0);
    }
    slots
}

fn mount_slot_positions() -> [(f64, f64); MOUNT_SLOT_COUNT] {
    [
        (-STATION_X / 2.0 + 62.0, -STATION_Y / 2.0 + 58.0),
        (STATION_X / 2.0 - 62.0, -STATION_Y / 2.0 + 58.0),
        (-STATION_X / 2.0 + 62.0, STATION_Y / 2.0 - 58.0),
        (STATION_X / 2.0 - 62.0, STATION_Y / 2.0 - 58.0),
        (0.0, -STATION_Y / 2.0 + 58.0),
        (0.0, STATION_Y / 2.0 - 58.0),
        (-STATION_X / 2.0 + 62.0, 0.0),
        (STATION_X / 2.0 - 62.0, 0.0),
    ]
}

fn perimeter_rims() -> Part {
    let front = centered_cube("humidity_slosh_front_low_robot_rim", STATION_X, RIM_W, 30.0)
        .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, DECK_Z + 15.0);
    let rear = centered_cube(
        "humidity_slosh_rear_door_recovery_service_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "humidity_slosh_left_condensate_spill_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "humidity_slosh_right_logger_cable_spill_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);

    front + rear + left + right
}

fn motion_axis_witness_ribs() -> Part {
    let x_axis = centered_cube(
        "humidity_slosh_rack_motion_x_axis_witness_rib",
        STATION_X - 210.0,
        6.0,
        16.0,
    )
    .translate(0.0, 305.0, DECK_Z + 8.0);
    let y_axis = centered_cube(
        "humidity_slosh_rack_motion_y_axis_witness_rib",
        6.0,
        STATION_Y - 205.0,
        16.0,
    )
    .translate(130.0, 0.0, DECK_Z + 8.0);
    let door_line = centered_cube(
        "humidity_slosh_door_recovery_event_reference_rib",
        STATION_X - 250.0,
        6.0,
        14.0,
    )
    .translate(0.0, -330.0, DECK_Z + 7.0);

    x_axis + y_axis + door_line
}

fn deck_datum_targets() -> Part {
    let mut targets = Part::empty("humidity_slosh_deck_robot_datum_targets");
    for (i, (x, y)) in datum_positions().into_iter().enumerate() {
        targets = targets
            + fiducial_disc(&format!("humidity_slosh_deck_datum_target_{i}")).translate(
                x,
                y,
                DECK_Z + 2.5,
            );
    }
    targets
}

fn datum_positions() -> [(f64, f64); DATUM_TARGET_COUNT] {
    [
        (-STATION_X / 2.0 + 104.0, -STATION_Y / 2.0 + 96.0),
        (STATION_X / 2.0 - 104.0, -STATION_Y / 2.0 + 96.0),
        (-STATION_X / 2.0 + 104.0, STATION_Y / 2.0 - 96.0),
        (STATION_X / 2.0 - 104.0, STATION_Y / 2.0 - 96.0),
    ]
}

fn condensate_grade_bars() -> Part {
    let mut bars = Part::empty("humidity_slosh_condensate_grade_witness_bars");
    for (i, y) in [-278.0, -224.0, -160.0, -92.0, -22.0, 48.0, 118.0]
        .into_iter()
        .enumerate()
    {
        bars = bars
            + centered_cube(
                format!("humidity_slosh_condensate_grade_bar_{i}"),
                STATION_X - 260.0,
                3.0,
                4.0,
            )
            .translate(0.0, y, DECK_Z + 2.0);
    }
    bars
}

fn reservoir_surrogate_cradle() -> Part {
    let shell = centered_cube(
        "humidity_slosh_reservoir_surrogate_cradle_shell",
        RESERVOIR_X,
        RESERVOIR_Y,
        RESERVOIR_Z,
    );
    let reservoir_cavity = centered_cube(
        "humidity_slosh_reservoir_working_volume_cavity_cut",
        RESERVOIR_CAVITY_X,
        RESERVOIR_CAVITY_Y,
        RESERVOIR_CAVITY_DEPTH + 0.8,
    )
    .translate(
        0.0,
        8.0,
        RESERVOIR_Z / 2.0 - RESERVOIR_CAVITY_DEPTH / 2.0 + 0.4,
    );
    let overflow_notch = centered_cube(
        "humidity_slosh_reservoir_overflow_notch_cut",
        92.0,
        14.0,
        26.0,
    )
    .translate(
        RESERVOIR_X / 2.0 - 34.0,
        RESERVOIR_Y / 2.0 - 48.0,
        RESERVOIR_Z / 2.0 - 13.0,
    );

    shell - reservoir_cavity - overflow_notch - reservoir_drain_port_cuts()
        + reservoir_tilt_index_saddles()
        + reservoir_grip_handles()
        + reservoir_level_shadow_strips()
        + cradle_barcode_lands()
}

fn reservoir_drain_port_cuts() -> Part {
    let mut cuts = Part::empty("humidity_slosh_reservoir_drain_port_cuts");
    for port in 0..RESERVOIR_DRAIN_PORTS {
        cuts = cuts
            + centered_cylinder(
                format!("humidity_slosh_reservoir_drain_port_cut_{port}"),
                7.0,
                58.0,
                28,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(
                RESERVOIR_X / 2.0 - 22.0,
                centered_index(port, RESERVOIR_DRAIN_PORTS, 58.0) - 72.0,
                RESERVOIR_Z / 2.0 - 19.0,
            );
    }
    cuts
}

fn reservoir_tilt_index_saddles() -> Part {
    let mut saddles = Part::empty("humidity_slosh_reservoir_tilt_index_saddles");
    for index in 0..RESERVOIR_TILT_INDEX_COUNT {
        saddles = saddles
            + centered_cube(
                format!("humidity_slosh_reservoir_tilt_index_saddle_{index}"),
                52.0,
                18.0,
                9.0 + index as f64 * 1.4,
            )
            .translate(
                centered_index(index, RESERVOIR_TILT_INDEX_COUNT, 72.0),
                -RESERVOIR_Y / 2.0 + 38.0,
                -RESERVOIR_Z / 2.0 + 4.5 + index as f64 * 0.7,
            );
    }
    saddles
}

fn reservoir_grip_handles() -> Part {
    let mut handles = Part::empty("humidity_slosh_reservoir_grip_handles");
    for (i, (x, y)) in [
        (-RESERVOIR_X / 2.0 + 36.0, -RESERVOIR_Y / 2.0 + 36.0),
        (RESERVOIR_X / 2.0 - 36.0, -RESERVOIR_Y / 2.0 + 36.0),
        (-RESERVOIR_X / 2.0 + 36.0, RESERVOIR_Y / 2.0 - 36.0),
        (RESERVOIR_X / 2.0 - 36.0, RESERVOIR_Y / 2.0 - 36.0),
    ]
    .into_iter()
    .enumerate()
    {
        handles = handles
            + centered_cube(
                format!("humidity_slosh_reservoir_robot_handle_land_{i}"),
                58.0,
                18.0,
                12.0,
            )
            .translate(x, y, RESERVOIR_Z / 2.0 + 6.0);
    }
    handles
}

fn reservoir_level_shadow_strips() -> Part {
    let mut strips = Part::empty("humidity_slosh_reservoir_level_shadow_strips");
    for rung in 0..FILL_LEVEL_RUNG_COUNT {
        strips = strips
            + centered_cube(
                format!("humidity_slosh_reservoir_internal_level_shadow_strip_{rung}"),
                62.0,
                3.0,
                3.0,
            )
            .translate(
                -RESERVOIR_CAVITY_X / 2.0 + 42.0,
                -RESERVOIR_CAVITY_Y / 2.0 + 32.0 + rung as f64 * 22.0,
                RESERVOIR_Z / 2.0 - RESERVOIR_CAVITY_DEPTH + 7.0 + rung as f64 * 2.8,
            );
    }
    strips
}

fn cradle_barcode_lands() -> Part {
    let left = code_bar_label(
        "humidity_slosh_reservoir_cradle_barcode_left",
        92.0,
        20.0,
        0,
    )
    .translate(
        -RESERVOIR_X / 2.0 + 78.0,
        RESERVOIR_Y / 2.0 - 28.0,
        RESERVOIR_Z / 2.0 + LABEL_Z,
    );
    let right = code_bar_label(
        "humidity_slosh_reservoir_cradle_barcode_right",
        92.0,
        20.0,
        1,
    )
    .translate(
        RESERVOIR_X / 2.0 - 78.0,
        RESERVOIR_Y / 2.0 - 28.0,
        RESERVOIR_Z / 2.0 + LABEL_Z,
    );
    left + right
}

fn fill_level_witness_ladder() -> Part {
    let frame = centered_cube(
        "humidity_slosh_fill_level_witness_ladder_frame",
        LADDER_X,
        LADDER_Y,
        LADDER_Z,
    );
    let center_clearance = centered_cube(
        "humidity_slosh_fill_level_ladder_center_window_cut",
        LADDER_X - 86.0,
        LADDER_Y - 56.0,
        LADDER_Z + 1.0,
    );

    frame - center_clearance - ladder_sight_slots()
        + ladder_vertical_rails()
        + ladder_rungs()
        + ladder_witness_flags()
        + ladder_reference_stops()
}

fn ladder_sight_slots() -> Part {
    let mut slots = Part::empty("humidity_slosh_ladder_sight_slots");
    for slot in 0..LADDER_SIGHT_SLOT_COUNT {
        slots = slots
            + centered_cube(
                format!("humidity_slosh_ladder_sight_slot_cut_{slot}"),
                34.0,
                18.0,
                LADDER_Z + 2.0,
            )
            .translate(
                centered_index(slot, LADDER_SIGHT_SLOT_COUNT, 58.0),
                0.0,
                0.0,
            );
    }
    slots
}

fn ladder_vertical_rails() -> Part {
    let left = centered_cube(
        "humidity_slosh_fill_level_ladder_left_rail",
        16.0,
        LADDER_Y - 20.0,
        18.0,
    )
    .translate(-LADDER_X / 2.0 + 44.0, 0.0, LADDER_Z / 2.0 + 9.0);
    let right = centered_cube(
        "humidity_slosh_fill_level_ladder_right_rail",
        16.0,
        LADDER_Y - 20.0,
        18.0,
    )
    .translate(LADDER_X / 2.0 - 44.0, 0.0, LADDER_Z / 2.0 + 9.0);

    left + right
}

fn ladder_rungs() -> Part {
    let mut rungs = Part::empty("humidity_slosh_fill_level_witness_rungs");
    for rung in 0..FILL_LEVEL_RUNG_COUNT {
        rungs = rungs
            + centered_cube(
                format!("humidity_slosh_fill_level_witness_rung_{rung}"),
                LADDER_X - 114.0,
                5.0,
                8.0,
            )
            .translate(
                0.0,
                centered_index(rung, FILL_LEVEL_RUNG_COUNT, FILL_LEVEL_RUNG_PITCH / 4.0),
                LADDER_Z / 2.0 + 4.0 + rung as f64 * 1.4,
            );
    }
    rungs
}

fn ladder_witness_flags() -> Part {
    let mut flags = Part::empty("humidity_slosh_fill_level_witness_flags");
    for flag in 0..WITNESS_FLAG_COUNT {
        flags = flags
            + centered_cube(
                format!("humidity_slosh_fill_level_removable_flag_{flag}"),
                36.0,
                18.0,
                18.0,
            )
            .translate(
                centered_index(flag, WITNESS_FLAG_COUNT, 92.0),
                -LADDER_Y / 2.0 + 18.0,
                LADDER_Z / 2.0 + 9.0,
            );
    }
    flags
}

fn ladder_reference_stops() -> Part {
    let low = code_bar_label("humidity_slosh_ladder_underfill_stop", 74.0, 18.0, 2).translate(
        -LADDER_X / 2.0 + 74.0,
        LADDER_Y / 2.0 - 20.0,
        LADDER_Z / 2.0 + LABEL_Z,
    );
    let high = code_bar_label("humidity_slosh_ladder_overflow_stop", 74.0, 18.0, 3).translate(
        LADDER_X / 2.0 - 74.0,
        LADDER_Y / 2.0 - 20.0,
        LADDER_Z / 2.0 + LABEL_Z,
    );
    low + high
}

fn slosh_baffle_coupon_cartridge() -> Part {
    let tray = centered_cube(
        "humidity_slosh_baffle_coupon_cartridge_tray",
        BAFFLE_X,
        BAFFLE_Y,
        BAFFLE_Z,
    );
    let pocket = centered_cube(
        "humidity_slosh_baffle_coupon_cartridge_insert_pocket_cut",
        BAFFLE_X - 58.0,
        BAFFLE_Y - 54.0,
        24.0,
    )
    .translate(0.0, 0.0, BAFFLE_Z / 2.0 - 12.0);

    tray - pocket - baffle_drain_slot_cuts()
        + baffle_coupon_fins()
        + baffle_coupon_lands()
        + baffle_lock_tabs()
        + baffle_flow_direction_arrows()
}

fn baffle_drain_slot_cuts() -> Part {
    let mut slots = Part::empty("humidity_slosh_baffle_drain_slot_cuts");
    for slot in 0..BAFFLE_DRAIN_SLOT_COUNT {
        let row = slot % 2;
        let col = slot / 2;
        slots = slots
            + centered_cube(
                format!("humidity_slosh_baffle_drain_slot_cut_{slot}"),
                22.0,
                8.0,
                BAFFLE_Z + 2.0,
            )
            .translate(
                centered_index(col, SLOSH_BAFFLE_COUNT, 48.0),
                centered_index(row, 2, 122.0),
                0.0,
            );
    }
    slots
}

fn baffle_coupon_fins() -> Part {
    let mut fins = Part::empty("humidity_slosh_baffle_coupon_fins");
    for baffle in 0..SLOSH_BAFFLE_COUNT {
        fins = fins
            + centered_cube(
                format!("humidity_slosh_slosh_baffle_coupon_fin_{baffle}"),
                9.0,
                BAFFLE_Y - 74.0,
                52.0,
            )
            .translate(
                centered_index(baffle, SLOSH_BAFFLE_COUNT, 48.0),
                0.0,
                BAFFLE_Z / 2.0 + 26.0,
            );
    }
    fins
}

fn baffle_coupon_lands() -> Part {
    let mut lands = Part::empty("humidity_slosh_wetted_baffle_coupon_lands");
    for coupon in 0..SLOSH_COUPON_COUNT {
        let rail = coupon % BAFFLE_COUPONS_PER_RAIL;
        let baffle = coupon / BAFFLE_COUPONS_PER_RAIL;
        lands = lands
            + centered_cube(
                format!("humidity_slosh_baffle_wetted_coupon_land_{coupon}"),
                32.0,
                20.0,
                5.0,
            )
            .translate(
                centered_index(baffle, SLOSH_BAFFLE_COUNT, 48.0),
                centered_index(rail, BAFFLE_COUPONS_PER_RAIL, 96.0),
                BAFFLE_Z / 2.0 + 2.5,
            );
    }
    lands
}

fn baffle_lock_tabs() -> Part {
    let mut tabs = Part::empty("humidity_slosh_baffle_cartridge_lock_tabs");
    for (i, (x, y)) in [
        (-BAFFLE_X / 2.0 + 32.0, -BAFFLE_Y / 2.0 + 24.0),
        (BAFFLE_X / 2.0 - 32.0, -BAFFLE_Y / 2.0 + 24.0),
        (-BAFFLE_X / 2.0 + 32.0, BAFFLE_Y / 2.0 - 24.0),
        (BAFFLE_X / 2.0 - 32.0, BAFFLE_Y / 2.0 - 24.0),
    ]
    .into_iter()
    .enumerate()
    {
        tabs = tabs
            + centered_cube(
                format!("humidity_slosh_baffle_cartridge_lock_tab_{i}"),
                44.0,
                18.0,
                12.0,
            )
            .translate(x, y, BAFFLE_Z / 2.0 + 6.0);
    }
    tabs
}

fn baffle_flow_direction_arrows() -> Part {
    let left = centered_cube(
        "humidity_slosh_baffle_left_surge_direction_bar",
        130.0,
        5.0,
        5.0,
    )
    .translate(-92.0, BAFFLE_Y / 2.0 - 28.0, BAFFLE_Z / 2.0 + 2.5);
    let right = centered_cube(
        "humidity_slosh_baffle_right_surge_direction_bar",
        130.0,
        5.0,
        5.0,
    )
    .translate(92.0, -BAFFLE_Y / 2.0 + 28.0, BAFFLE_Z / 2.0 + 2.5);
    left + right
}

fn rh_dewpoint_logger_pockets() -> Part {
    let block = centered_cube(
        "humidity_slosh_rh_dewpoint_logger_pocket_block",
        LOGGER_X,
        LOGGER_Y,
        LOGGER_Z,
    );

    block - logger_pocket_cuts() - dewpoint_reference_well_cuts()
        + logger_cable_clip_ribs()
        + logger_retention_tabs()
        + logger_evidence_code_lands()
}

fn logger_pocket_cuts() -> Part {
    let mut cuts = Part::empty("humidity_slosh_rh_dewpoint_logger_pocket_cuts");
    for pocket in 0..LOGGER_POCKET_COUNT {
        cuts = cuts
            + centered_cube(
                format!("humidity_slosh_rh_dewpoint_logger_pocket_cut_{pocket}"),
                LOGGER_POCKET_X,
                LOGGER_POCKET_Y,
                24.0,
            )
            .translate(
                centered_index(pocket, LOGGER_POCKET_COUNT, 48.0),
                18.0,
                LOGGER_Z / 2.0 - 12.0,
            );
    }
    cuts
}

fn dewpoint_reference_well_cuts() -> Part {
    let mut cuts = Part::empty("humidity_slosh_dewpoint_reference_well_cuts");
    for well in 0..DEWPOINT_REFERENCE_WELL_COUNT {
        cuts = cuts
            + centered_cylinder(
                format!("humidity_slosh_chilled_mirror_reference_well_cut_{well}"),
                13.0,
                26.0,
                32,
            )
            .translate(
                centered_index(well, DEWPOINT_REFERENCE_WELL_COUNT, 56.0),
                -LOGGER_Y / 2.0 + 36.0,
                LOGGER_Z / 2.0 - 13.0,
            );
    }
    cuts
}

fn logger_cable_clip_ribs() -> Part {
    let mut clips = Part::empty("humidity_slosh_logger_cable_clip_ribs");
    for clip in 0..LOGGER_CABLE_CLIP_COUNT {
        clips = clips
            + centered_cube(
                format!("humidity_slosh_logger_cable_clip_rib_{clip}"),
                20.0,
                6.0,
                12.0,
            )
            .translate(
                centered_index(clip, LOGGER_CABLE_CLIP_COUNT, 38.0),
                -LOGGER_Y / 2.0 + 14.0,
                LOGGER_Z / 2.0 + 6.0,
            );
    }
    clips
}

fn logger_retention_tabs() -> Part {
    let front = centered_cube(
        "humidity_slosh_logger_front_retention_tab",
        LOGGER_X - 52.0,
        8.0,
        10.0,
    )
    .translate(0.0, -LOGGER_Y / 2.0 + 12.0, LOGGER_Z / 2.0 + 5.0);
    let rear = centered_cube(
        "humidity_slosh_logger_rear_retention_tab",
        LOGGER_X - 52.0,
        8.0,
        10.0,
    )
    .translate(0.0, LOGGER_Y / 2.0 - 12.0, LOGGER_Z / 2.0 + 5.0);
    front + rear
}

fn logger_evidence_code_lands() -> Part {
    code_bar_label("humidity_slosh_logger_evidence_code_land", 112.0, 18.0, 4).translate(
        0.0,
        LOGGER_Y / 2.0 - 28.0,
        LOGGER_Z / 2.0 + LABEL_Z,
    )
}

fn overflow_underfill_token_rail() -> Part {
    let rail = centered_cube(
        "humidity_slosh_overflow_underfill_token_rail",
        TOKEN_X,
        TOKEN_Y,
        TOKEN_Z,
    );

    rail - token_recess_cuts() + token_lane_dividers() + token_state_discs() + token_end_stops()
}

fn token_recess_cuts() -> Part {
    let mut cuts = Part::empty("humidity_slosh_overflow_underfill_token_recess_cuts");
    for token in 0..TOKEN_COUNT {
        let lane = token % TOKEN_LANE_COUNT;
        let step = token / TOKEN_LANE_COUNT;
        cuts = cuts
            + centered_cylinder(
                format!(
                    "humidity_slosh_{}_token_recess_cut_{step}",
                    token_lane_name(lane)
                ),
                TOKEN_D / 2.0,
                TOKEN_Z + 2.0,
                32,
            )
            .translate(
                centered_index(step, TOKENS_PER_LANE, 74.0),
                centered_index(lane, TOKEN_LANE_COUNT, 29.0),
                0.0,
            );
    }
    cuts
}

fn token_lane_dividers() -> Part {
    let upper = centered_cube(
        "humidity_slosh_token_rail_upper_lane_divider",
        TOKEN_X - 42.0,
        3.0,
        10.0,
    )
    .translate(0.0, 14.5, TOKEN_Z / 2.0 + 5.0);
    let lower = centered_cube(
        "humidity_slosh_token_rail_lower_lane_divider",
        TOKEN_X - 42.0,
        3.0,
        10.0,
    )
    .translate(0.0, -14.5, TOKEN_Z / 2.0 + 5.0);
    upper + lower
}

fn token_state_discs() -> Part {
    let mut discs = Part::empty("humidity_slosh_overflow_underfill_state_discs");
    for token in 0..TOKEN_COUNT {
        let lane = token % TOKEN_LANE_COUNT;
        let step = token / TOKEN_LANE_COUNT;
        discs = discs
            + centered_cylinder(
                format!(
                    "humidity_slosh_{}_removable_token_{step}",
                    token_lane_name(lane)
                ),
                TOKEN_D / 2.0 - 3.0,
                4.0,
                32,
            )
            .translate(
                centered_index(step, TOKENS_PER_LANE, 74.0),
                centered_index(lane, TOKEN_LANE_COUNT, 29.0),
                TOKEN_Z / 2.0 + 2.0,
            );
    }
    discs
}

fn token_end_stops() -> Part {
    let left = centered_cube(
        "humidity_slosh_token_rail_left_end_stop",
        12.0,
        TOKEN_Y,
        18.0,
    )
    .translate(-TOKEN_X / 2.0 + 6.0, 0.0, TOKEN_Z / 2.0 + 9.0);
    let right = centered_cube(
        "humidity_slosh_token_rail_right_end_stop",
        12.0,
        TOKEN_Y,
        18.0,
    )
    .translate(TOKEN_X / 2.0 - 6.0, 0.0, TOKEN_Z / 2.0 + 9.0);
    left + right
}

fn token_lane_name(lane: usize) -> &'static str {
    match lane {
        0 => "underfill",
        1 => "target_fill",
        _ => "overflow",
    }
}

fn condensate_return_witness() -> Part {
    let body = centered_cube(
        "humidity_slosh_condensate_return_witness_body",
        CONDENSATE_X,
        CONDENSATE_Y,
        CONDENSATE_Z,
    );
    let basin = centered_cube(
        "humidity_slosh_condensate_return_basin_cut",
        CONDENSATE_X - 46.0,
        CONDENSATE_Y - 42.0,
        CONDENSATE_BASIN_DEPTH,
    )
    .translate(
        0.0,
        0.0,
        CONDENSATE_Z / 2.0 - CONDENSATE_BASIN_DEPTH / 2.0 + 0.3,
    );

    body - basin - condensate_return_channel_cuts()
        + condensate_return_cup_lands()
        + condensate_dye_witness_tabs()
        + condensate_no_backflow_weirs()
}

fn condensate_return_channel_cuts() -> Part {
    let mut cuts = Part::empty("humidity_slosh_condensate_return_channel_cuts");
    for channel in 0..RETURN_CHANNEL_COUNT {
        cuts = cuts
            + centered_cube(
                format!("humidity_slosh_condensate_return_channel_cut_{channel}"),
                CONDENSATE_X - 74.0,
                7.0,
                CONDENSATE_Z + 2.0,
            )
            .translate(
                0.0,
                centered_index(channel, RETURN_CHANNEL_COUNT, 20.0),
                0.0,
            );
    }
    cuts
}

fn condensate_return_cup_lands() -> Part {
    let mut cups = Part::empty("humidity_slosh_condensate_return_cup_lands");
    for cup in 0..RETURN_CUP_COUNT {
        let x = centered_index(cup, RETURN_CUP_COUNT, 70.0);
        cups = cups
            + centered_cylinder(
                format!("humidity_slosh_condensate_return_cup_land_{cup}"),
                RETURN_CUP_D / 2.0,
                6.0,
                40,
            )
            .translate(x, CONDENSATE_Y / 2.0 - 30.0, CONDENSATE_Z / 2.0 + 3.0)
            - centered_cylinder(
                format!("humidity_slosh_condensate_return_cup_center_recess_{cup}"),
                RETURN_CUP_D / 2.0 - 9.0,
                7.0,
                40,
            )
            .translate(x, CONDENSATE_Y / 2.0 - 30.0, CONDENSATE_Z / 2.0 + 3.5);
    }
    cups
}

fn condensate_dye_witness_tabs() -> Part {
    let mut tabs = Part::empty("humidity_slosh_condensate_dye_witness_tabs");
    for tab in 0..RETURN_DYE_WITNESS_COUNT {
        tabs = tabs
            + centered_cube(
                format!("humidity_slosh_condensate_dye_witness_tab_{tab}"),
                34.0,
                14.0,
                8.0,
            )
            .translate(
                centered_index(tab, RETURN_DYE_WITNESS_COUNT, 50.0),
                -CONDENSATE_Y / 2.0 + 22.0,
                CONDENSATE_Z / 2.0 + 4.0,
            );
    }
    tabs
}

fn condensate_no_backflow_weirs() -> Part {
    let upstream = centered_cube(
        "humidity_slosh_condensate_return_upstream_no_backflow_weir",
        CONDENSATE_X - 62.0,
        8.0,
        17.0,
    )
    .translate(0.0, -18.0, CONDENSATE_Z / 2.0 + 8.5);
    let downstream = centered_cube(
        "humidity_slosh_condensate_return_downstream_no_backflow_weir",
        CONDENSATE_X - 92.0,
        8.0,
        12.0,
    )
    .translate(0.0, 18.0, CONDENSATE_Z / 2.0 + 6.0);
    upstream + downstream
}

fn contamination_coupon_pockets() -> Part {
    let rack = centered_cube(
        "humidity_slosh_contamination_coupon_pocket_rack",
        CONTAM_X,
        CONTAM_Y,
        CONTAM_Z,
    );

    rack - contamination_coupon_slot_cuts()
        + contamination_coupon_index_tabs()
        + contamination_positive_negative_controls()
}

fn contamination_coupon_slot_cuts() -> Part {
    let mut cuts = Part::empty("humidity_slosh_contamination_coupon_slot_cuts");
    for coupon in 0..CONTAMINATION_COUPON_COUNT {
        let col = coupon % CONTAMINATION_COUPON_COLS;
        let row = coupon / CONTAMINATION_COUPON_COLS;
        cuts = cuts
            + centered_cube(
                format!("humidity_slosh_contamination_coupon_slot_cut_{coupon}"),
                CONTAM_COUPON_SLOT_X,
                CONTAM_COUPON_SLOT_Y,
                20.0,
            )
            .translate(
                centered_index(col, CONTAMINATION_COUPON_COLS, 58.0),
                centered_index(row, CONTAMINATION_COUPON_ROWS, 28.0),
                CONTAM_Z / 2.0 - 10.0,
            );
    }
    cuts
}

fn contamination_coupon_index_tabs() -> Part {
    let mut tabs = Part::empty("humidity_slosh_contamination_coupon_index_tabs");
    for row in 0..CONTAMINATION_COUPON_ROWS {
        tabs = tabs
            + centered_cube(
                format!("humidity_slosh_contamination_coupon_row_index_tab_{row}"),
                12.0,
                20.0,
                8.0,
            )
            .translate(
                -CONTAM_X / 2.0 + 20.0,
                centered_index(row, CONTAMINATION_COUPON_ROWS, 28.0),
                CONTAM_Z / 2.0 + 4.0,
            );
    }
    tabs
}

fn contamination_positive_negative_controls() -> Part {
    let negative = code_bar_label(
        "humidity_slosh_contamination_negative_control",
        58.0,
        16.0,
        5,
    )
    .translate(-58.0, CONTAM_Y / 2.0 - 18.0, CONTAM_Z / 2.0 + LABEL_Z);
    let positive = code_bar_label(
        "humidity_slosh_contamination_positive_control",
        58.0,
        16.0,
        6,
    )
    .translate(58.0, CONTAM_Y / 2.0 - 18.0, CONTAM_Z / 2.0 + LABEL_Z);
    negative + positive
}

fn rack_motion_door_recovery_witnesses() -> Part {
    let rail = centered_cube(
        "humidity_slosh_rack_motion_door_recovery_witness_rail",
        RECOVERY_X,
        RECOVERY_Y,
        RECOVERY_Z,
    );

    rail + rack_motion_step_blocks() + door_recovery_event_pucks() + recovery_datum_pucks()
        - recovery_lanyard_slots()
}

fn rack_motion_step_blocks() -> Part {
    let mut steps = Part::empty("humidity_slosh_rack_motion_step_blocks");
    for step in 0..RACK_MOTION_STEP_COUNT {
        steps = steps
            + centered_cube(
                format!("humidity_slosh_rack_motion_accel_step_block_{step}"),
                38.0,
                12.0,
                8.0 + step as f64 * 1.5,
            )
            .translate(
                centered_index(step, RACK_MOTION_STEP_COUNT, 48.0) - 48.0,
                -RECOVERY_Y / 2.0 + 18.0,
                RECOVERY_Z / 2.0 + 4.0 + step as f64 * 0.75,
            );
    }
    steps
}

fn door_recovery_event_pucks() -> Part {
    let mut pucks = Part::empty("humidity_slosh_door_recovery_event_pucks");
    for event in 0..DOOR_RECOVERY_EVENT_COUNT {
        pucks = pucks
            + centered_cylinder(
                format!(
                    "humidity_slosh_door_recovery_{}_event_puck",
                    door_recovery_event_name(event)
                ),
                13.0,
                6.0,
                32,
            )
            .translate(
                centered_index(event, DOOR_RECOVERY_EVENT_COUNT, 42.0) + 88.0,
                RECOVERY_Y / 2.0 - 20.0,
                RECOVERY_Z / 2.0 + 3.0,
            );
    }
    pucks
}

fn recovery_datum_pucks() -> Part {
    let mut pucks = Part::empty("humidity_slosh_recovery_datum_pucks");
    for puck in 0..RECOVERY_DATUM_PUCK_COUNT {
        pucks = pucks
            + centered_cylinder(
                format!("humidity_slosh_recovery_datum_puck_{puck}"),
                8.0,
                5.0,
                28,
            )
            .translate(
                centered_index(puck, RECOVERY_DATUM_PUCK_COUNT, 110.0),
                0.0,
                RECOVERY_Z / 2.0 + 2.5,
            );
    }
    pucks
}

fn recovery_lanyard_slots() -> Part {
    let left = centered_cube(
        "humidity_slosh_recovery_left_lanyard_slot_cut",
        22.0,
        8.0,
        RECOVERY_Z + 2.0,
    )
    .translate(-RECOVERY_X / 2.0 + 28.0, 0.0, 0.0);
    let right = centered_cube(
        "humidity_slosh_recovery_right_lanyard_slot_cut",
        22.0,
        8.0,
        RECOVERY_Z + 2.0,
    )
    .translate(RECOVERY_X / 2.0 - 28.0, 0.0, 0.0);
    left + right
}

fn door_recovery_event_name(event: usize) -> &'static str {
    match event {
        0 => "baseline",
        1 => "door_open",
        2 => "door_close",
        3 => "t60s",
        _ => "recovered",
    }
}

fn release_hold_reject_gates() -> Part {
    let base = centered_cube(
        "humidity_slosh_release_hold_reject_gate_base",
        GATE_X,
        GATE_Y,
        GATE_Z,
    );

    base - gate_token_slot_cuts() + gate_dividers() + gate_paddles() + gate_status_lands()
}

fn gate_token_slot_cuts() -> Part {
    let mut cuts = Part::empty("humidity_slosh_release_hold_reject_token_slot_cuts");
    for slot in 0..GATE_TOKEN_SLOT_COUNT {
        let gate = slot % GATE_COUNT;
        let token = slot / GATE_COUNT;
        cuts = cuts
            + centered_cube(
                format!(
                    "humidity_slosh_{}_gate_token_slot_cut_{token}",
                    disposition_name(gate)
                ),
                30.0,
                14.0,
                GATE_Z + 2.0,
            )
            .translate(
                centered_index(gate, GATE_COUNT, 104.0),
                centered_index(token, GATE_TOKEN_SLOTS_PER_GATE, 20.0),
                0.0,
            );
    }
    cuts
}

fn gate_dividers() -> Part {
    let left = centered_cube(
        "humidity_slosh_release_hold_divider_gate",
        5.0,
        GATE_Y,
        18.0,
    )
    .translate(-52.0, 0.0, GATE_Z / 2.0 + 9.0);
    let right = centered_cube("humidity_slosh_hold_reject_divider_gate", 5.0, GATE_Y, 18.0)
        .translate(52.0, 0.0, GATE_Z / 2.0 + 9.0);
    left + right
}

fn gate_paddles() -> Part {
    let mut paddles = Part::empty("humidity_slosh_disposition_gate_paddles");
    for gate in 0..GATE_COUNT {
        paddles = paddles
            + centered_cube(
                format!("humidity_slosh_{}_gate_paddle", disposition_name(gate)),
                78.0,
                10.0,
                28.0,
            )
            .translate(
                centered_index(gate, GATE_COUNT, 104.0),
                -GATE_Y / 2.0 + 9.0,
                GATE_Z / 2.0 + 14.0,
            );
    }
    paddles
}

fn gate_status_lands() -> Part {
    let mut lands = Part::empty("humidity_slosh_gate_status_lands");
    for gate in 0..GATE_COUNT {
        lands = lands
            + code_bar_label(
                &format!("humidity_slosh_{}_status_land", disposition_name(gate)),
                60.0,
                14.0,
                gate + 7,
            )
            .translate(
                centered_index(gate, GATE_COUNT, 104.0),
                GATE_Y / 2.0 - 14.0,
                GATE_Z / 2.0 + LABEL_Z,
            );
    }
    lands
}

fn disposition_name(gate: usize) -> &'static str {
    match gate {
        0 => "release",
        1 => "hold",
        _ => "reject",
    }
}

fn robot_service_keepout_gauges() -> Part {
    keepout_outline()
        + front_robot_approach_gauge()
        + rear_service_sweep_gauge()
        + reservoir_lift_clearance_gauge()
        + logger_cable_service_gauge()
}

fn keepout_outline() -> Part {
    let front = centered_cube(
        "humidity_slosh_keepout_outline_front",
        KEEP_OUT_X,
        8.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, -KEEP_OUT_Y / 2.0, KEEP_OUT_Z / 2.0);
    let rear = centered_cube(
        "humidity_slosh_keepout_outline_rear",
        KEEP_OUT_X,
        8.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, KEEP_OUT_Y / 2.0, KEEP_OUT_Z / 2.0);
    let left = centered_cube(
        "humidity_slosh_keepout_outline_left",
        8.0,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(-KEEP_OUT_X / 2.0, 0.0, KEEP_OUT_Z / 2.0);
    let right = centered_cube(
        "humidity_slosh_keepout_outline_right",
        8.0,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(KEEP_OUT_X / 2.0, 0.0, KEEP_OUT_Z / 2.0);
    front + rear + left + right
}

fn front_robot_approach_gauge() -> Part {
    centered_cube(
        "humidity_slosh_front_robot_approach_clearance_gauge",
        KEEP_OUT_X - 220.0,
        18.0,
        KEEP_OUT_Z,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 + FRONT_ROBOT_CLEARANCE_Y,
        KEEP_OUT_Z / 2.0,
    )
}

fn rear_service_sweep_gauge() -> Part {
    centered_cube(
        "humidity_slosh_rear_door_service_sweep_clearance_gauge",
        KEEP_OUT_X - 240.0,
        18.0,
        KEEP_OUT_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - REAR_SERVICE_CLEARANCE_Y,
        KEEP_OUT_Z / 2.0,
    )
}

fn reservoir_lift_clearance_gauge() -> Part {
    centered_cube(
        "humidity_slosh_reservoir_lift_vertical_clearance_gauge",
        RESERVOIR_X + 84.0,
        RESERVOIR_Y + 76.0,
        KEEP_OUT_Z,
    )
    .translate(RESERVOIR_POS.0, RESERVOIR_POS.1, RESERVOIR_LIFT_CLEARANCE_Z)
}

fn logger_cable_service_gauge() -> Part {
    centered_cube(
        "humidity_slosh_logger_cable_service_bend_radius_gauge",
        LOGGER_CABLE_SERVICE_X,
        LOGGER_Y + 120.0,
        KEEP_OUT_Z,
    )
    .translate(
        STATION_X / 2.0 - LOGGER_CABLE_SERVICE_X / 2.0,
        LOGGER_POS.1,
        KEEP_OUT_Z / 2.0,
    )
}

fn fiducial_disc(name: &str) -> Part {
    let disc = centered_cylinder(format!("{name}_outer_disc"), 11.0, 5.0, 36);
    let center = centered_cylinder(format!("{name}_center_dot"), 2.2, 6.0, 20);
    let cross_x = centered_cube(format!("{name}_cross_x"), 18.0, 2.2, 6.0);
    let cross_y = centered_cube(format!("{name}_cross_y"), 2.2, 18.0, 6.0);
    disc - center - cross_x - cross_y
}

fn code_bar_label(label: &str, x: f64, y: f64, code: usize) -> Part {
    let land = centered_cube(format!("{label}_land"), x, y, LABEL_Z);
    let mut bars = Part::empty(format!("{label}_raised_bar_code"));
    for bar in 0..8 {
        if ((code + 5) >> (bar % 5)) & 1 == 1 || bar == 0 || bar == 7 {
            bars = bars
                + centered_cube(
                    format!("{label}_raised_bar_{bar}"),
                    2.8 + (bar % 3) as f64,
                    y - 5.0,
                    LABEL_Z,
                )
                .translate(centered_index(bar, 8, x / 10.0), 0.0, LABEL_Z);
        }
    }
    land + bars
}

fn reservoir_working_volume_ml() -> f64 {
    RESERVOIR_CAVITY_X * RESERVOIR_CAVITY_Y * RESERVOIR_WORKING_DEPTH / 1000.0
}

fn underfill_trip_volume_ml() -> f64 {
    reservoir_working_volume_ml() * 0.42
}

fn worst_case_slosh_surge_ml() -> f64 {
    reservoir_working_volume_ml() * 0.36
        + SLOSH_BAFFLE_COUNT as f64 * 12.0
        + TOKEN_COUNT as f64 * 7.5
}

fn containment_freeboard_ml() -> f64 {
    let inner_x = STATION_X - 2.0 * RIM_W;
    let inner_y = STATION_Y - 2.0 * RIM_W;
    let freeboard_z = RIM_Z - BASIN_DEPTH;
    inner_x * inner_y * freeboard_z / 1000.0
}

fn condensate_return_capacity_ml() -> f64 {
    (CONDENSATE_X - 46.0) * (CONDENSATE_Y - 42.0) * CONDENSATE_BASIN_DEPTH / 1000.0
        + RETURN_CUP_COUNT as f64 * 42.0
}

fn door_recovery_condensate_challenge_ml() -> f64 {
    DOOR_RECOVERY_EVENT_COUNT as f64 * 54.0 + RETURN_DYE_WITNESS_COUNT as f64 * 12.0
}

fn front_robot_clearance() -> f64 {
    FRONT_ROBOT_CLEARANCE_Y
}

fn rear_service_clearance() -> f64 {
    REAR_SERVICE_CLEARANCE_Y
}

fn logger_cable_service_clearance() -> f64 {
    LOGGER_CABLE_SERVICE_X
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_scoped_and_stable() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        for path in OUTPUTS {
            assert!(path.starts_with(
                "output/closed_humidity_reservoir_fill_level_slosh_response_station_"
            ));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn requested_feature_groups_are_explicit() {
        for feature in [
            "reservoir_surrogate_cradle",
            "fill_level_witness_ladder",
            "slosh_baffle_coupons",
            "rh_dewpoint_logger_pockets",
            "overflow_underfill_token_rail",
            "condensate_return_witness",
            "contamination_coupon_pockets",
            "rack_motion_door_recovery_witnesses",
            "release_hold_reject_gates",
            "named_stl_outputs",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn module_dimensions_fit_without_overlap() {
        assert_design_constraints();
        let rects = module_rects();
        for rect in rects {
            assert!(rect.fits_inside_deck(), "{} does not fit", rect.name);
        }
        for i in 0..rects.len() {
            for j in (i + 1)..rects.len() {
                assert!(!rects[i].overlaps(rects[j]));
            }
        }
    }

    #[test]
    fn fill_level_and_slosh_witness_counts_match_design() {
        assert_eq!(FILL_LEVEL_RUNG_COUNT, 8);
        assert_eq!(WITNESS_FLAG_COUNT, 4);
        assert_eq!(SLOSH_BAFFLE_COUNT, 7);
        assert_eq!(SLOSH_COUPON_COUNT, 14);
        assert_eq!(BAFFLE_DRAIN_SLOT_COUNT, SLOSH_BAFFLE_COUNT * 2);
        assert_eq!(BAFFLE_LOCK_TAB_COUNT, 4);
    }

    #[test]
    fn logger_token_condensate_and_contamination_features_cover_risks() {
        assert_eq!(LOGGER_POCKET_COUNT, 5);
        assert_eq!(DEWPOINT_REFERENCE_WELL_COUNT, 2);
        assert_eq!(TOKEN_COUNT, 12);
        assert_eq!(RETURN_CHANNEL_COUNT, 5);
        assert_eq!(RETURN_CUP_COUNT, 3);
        assert_eq!(CONTAMINATION_COUPON_COUNT, 12);
        assert_eq!(GATE_COUNT, 3);
        assert_eq!(GATE_TOKEN_SLOT_COUNT, 9);
    }

    #[test]
    fn reservoir_and_return_capacity_budgets_are_explicit() {
        assert!(reservoir_working_volume_ml() > 2500.0);
        assert!(reservoir_working_volume_ml() > worst_case_slosh_surge_ml());
        assert!(containment_freeboard_ml() > worst_case_slosh_surge_ml());
        assert!(condensate_return_capacity_ml() > door_recovery_condensate_challenge_ml());
    }

    #[test]
    fn rack_motion_door_recovery_and_keepouts_are_complete() {
        assert_eq!(RACK_MOTION_STEP_COUNT, 6);
        assert_eq!(DOOR_RECOVERY_EVENT_COUNT, 5);
        assert_eq!(door_recovery_event_name(0), "baseline");
        assert_eq!(
            door_recovery_event_name(DOOR_RECOVERY_EVENT_COUNT - 1),
            "recovered"
        );
        assert_eq!(RECOVERY_DATUM_PUCK_COUNT, 4);
        assert_eq!(KEEP_OUT_GAUGE_COUNT, 5);
        assert!(RESERVOIR_LIFT_CLEARANCE_Z > RESERVOIR_Z + BAFFLE_Z);
    }
}
