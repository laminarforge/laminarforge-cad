use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed positive-pressure gowning exit/reentry station.
//
// Intent:
// - Validate a walk-in clean-zone boundary that supports personnel exit and
//   controlled reentry without exposing closed culture modules to room air.
// - Make the physical interfaces for positive-pressure cascade setpoints,
//   interlocked doors, gown supply staging, doff waste bagout, particle capture,
//   airflow recovery, scan custody, and evidence imaging explicit in CAD.
// - Keep HEPA modules, sensors, door hardware, gowning SOPs, sterilization
//   recipes, alert limits, and biological acceptance criteria as separately
//   validated purchased/process items.

const PREFIX: &str = "closed_positive_pressure_gowning_exit_reentry_station";

const OUTPUTS: [&str; 13] = [
    "output/closed_positive_pressure_gowning_exit_reentry_station_base_cascade_deck.stl",
    "output/closed_positive_pressure_gowning_exit_reentry_station_pressure_cascade_vestibule.stl",
    "output/closed_positive_pressure_gowning_exit_reentry_station_exit_reentry_interlock_doors.stl",
    "output/closed_positive_pressure_gowning_exit_reentry_station_clean_gown_staging_nests.stl",
    "output/closed_positive_pressure_gowning_exit_reentry_station_positive_pressure_supply_diffuser.stl",
    "output/closed_positive_pressure_gowning_exit_reentry_station_return_exhaust_bypass_path.stl",
    "output/closed_positive_pressure_gowning_exit_reentry_station_particle_capture_floor_coupons.stl",
    "output/closed_positive_pressure_gowning_exit_reentry_station_hand_sanitization_glove_check_panel.stl",
    "output/closed_positive_pressure_gowning_exit_reentry_station_pressure_recovery_sensor_tree.stl",
    "output/closed_positive_pressure_gowning_exit_reentry_station_barcode_training_release_lands.stl",
    "output/closed_positive_pressure_gowning_exit_reentry_station_doff_waste_bagout_collar.stl",
    "output/closed_positive_pressure_gowning_exit_reentry_station_evidence_camera_bridge.stl",
    "output/closed_positive_pressure_gowning_exit_reentry_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 12] = [
    "pressure_cascade_vestibule",
    "exit_reentry_interlock_doors",
    "clean_gown_staging_nests",
    "positive_pressure_supply_diffuser",
    "return_exhaust_bypass_path",
    "particle_capture_floor_coupons",
    "hand_sanitization_glove_check_panel",
    "pressure_recovery_sensor_tree",
    "barcode_training_release_lands",
    "doff_waste_bagout_collar",
    "evidence_camera_bridge",
    "assembly",
];

const DECK_X: f64 = 1580.0;
const DECK_Y: f64 = 1020.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 46.0;
const SOCKET_DEPTH: f64 = 6.0;
const MOUNT_COUNT: usize = 12;

const VESTIBULE_POS: (f64, f64) = (-440.0, 210.0);
const VESTIBULE_X: f64 = 500.0;
const VESTIBULE_Y: f64 = 320.0;
const VESTIBULE_Z: f64 = 38.0;
const CASCADE_ZONE_COUNT: usize = 4;
const CASCADE_SETPOINTS_PA: [f64; CASCADE_ZONE_COUNT] = [45.0, 30.0, 18.0, 0.0];
const MIN_POSITIVE_STEP_PA: f64 = 12.0;
const CASCADE_BAY_X: f64 = 108.0;
const CASCADE_BAY_Y: f64 = 242.0;
const CASCADE_BAY_PITCH_X: f64 = 118.0;
const DIFFUSER_SLOTS_PER_ZONE: usize = 5;
const CASCADE_TAP_COUNT: usize = CASCADE_ZONE_COUNT * 2;
const CASCADE_SEAL_SHIM_COUNT: usize = 6;

const DOOR_POS: (f64, f64) = (120.0, 210.0);
const DOOR_ZONE_X: f64 = 420.0;
const DOOR_ZONE_Y: f64 = 320.0;
const DOOR_FRAME_X: f64 = 312.0;
const DOOR_FRAME_Z: f64 = 244.0;
const DOOR_OPENING_X: f64 = 208.0;
const DOOR_OPENING_Z: f64 = 148.0;
const DOOR_FACE_Y: f64 = 16.0;
const DOOR_PAIR_SPACING_Y: f64 = 216.0;
const INTERLOCK_PIN_COUNT: usize = 8;
const LATCH_BLOCK_COUNT: usize = 8;
const DOOR_STATE_TOKEN_COUNT: usize = 6;

const GOWN_POS: (f64, f64) = (520.0, 210.0);
const GOWN_ZONE_X: f64 = 300.0;
const GOWN_ZONE_Y: f64 = 320.0;
const GOWN_PANEL_Z: f64 = 34.0;
const GOWN_CANISTER_COUNT: usize = 6;
const BOOTIE_NEST_COUNT: usize = 4;
const GLOVE_BOX_COUNT: usize = 4;
const GOWN_RETURN_BIN_COUNT: usize = 2;
const GOWN_RFID_PUCK_COUNT: usize = 6;

const SUPPLY_POS: (f64, f64) = (-440.0, -120.0);
const SUPPLY_ZONE_X: f64 = 500.0;
const SUPPLY_ZONE_Y: f64 = 220.0;
const SUPPLY_PLENUM_Z: f64 = 82.0;
const HEPA_CASSETTE_COUNT: usize = 2;
const SUPPLY_SLOT_COUNT: usize = 12;
const MFC_PORT_COUNT: usize = 4;
const SUPPLY_BALANCE_VANE_COUNT: usize = 8;

const RETURN_POS: (f64, f64) = (120.0, -120.0);
const RETURN_ZONE_X: f64 = 420.0;
const RETURN_ZONE_Y: f64 = 220.0;
const RETURN_PLENUM_Z: f64 = 58.0;
const RETURN_SLOT_COUNT: usize = 10;
const BYPASS_DAMPER_COUNT: usize = 4;
const FILTER_COUPON_COUNT: usize = 6;
const RELIEF_FLAP_COUNT: usize = 3;

const PARTICLE_POS: (f64, f64) = (520.0, -120.0);
const PARTICLE_ZONE_X: f64 = 300.0;
const PARTICLE_ZONE_Y: f64 = 220.0;
const STICKY_TILE_ROWS: usize = 3;
const STICKY_TILE_COLS: usize = 4;
const STICKY_TILE_COUNT: usize = STICKY_TILE_ROWS * STICKY_TILE_COLS;
const PARTICLE_COUPON_COUNT: usize = 8;
const PARTICLE_DRAIN_COUNT: usize = 2;

const SENSOR_POS: (f64, f64) = (-440.0, -350.0);
const SENSOR_ZONE_X: f64 = 500.0;
const SENSOR_ZONE_Y: f64 = 160.0;
const SENSOR_RAIL_Z: f64 = 44.0;
const DP_SENSOR_COUNT: usize = 6;
const PARTICLE_COUNTER_PORT_COUNT: usize = 4;
const RECOVERY_TIMER_TOKEN_COUNT: usize = 5;
const ALARM_BEACON_COUNT: usize = 3;

const TRACE_POS: (f64, f64) = (120.0, -350.0);
const TRACE_ZONE_X: f64 = 420.0;
const TRACE_ZONE_Y: f64 = 160.0;
const TRACE_PANEL_Z: f64 = 12.0;
const BARCODE_LAND_COUNT: usize = 12;
const TRAINING_CARD_COUNT: usize = 4;
const RELEASE_HOLD_REJECT_LANES: usize = 3;
const BADGE_DOCK_COUNT: usize = 4;

const WASTE_POS: (f64, f64) = (520.0, -350.0);
const WASTE_ZONE_X: f64 = 300.0;
const WASTE_ZONE_Y: f64 = 160.0;
const WASTE_COLLAR_OUTER_R: f64 = 68.0;
const WASTE_COLLAR_INNER_R: f64 = 42.0;
const WASTE_TIE_SADDLE_COUNT: usize = 6;
const WASTE_QUARANTINE_BAY_COUNT: usize = 3;
const DOFF_SEQUENCE_TOKEN_COUNT: usize = 5;

const BRIDGE_X: f64 = 1340.0;
const BRIDGE_Y: f64 = 760.0;
const BRIDGE_UNDERSIDE_Z: f64 = 306.0;
const BRIDGE_BEAM_Z: f64 = 28.0;
const CAMERA_POD_COUNT: usize = 5;
const LIGHT_BAR_COUNT: usize = 4;

const FRONT_REENTRY_ROBOT_CLEARANCE_Y: f64 = 360.0;
const REAR_HEPA_SERVICE_CLEARANCE_Y: f64 = 280.0;
const LEFT_SENSOR_SERVICE_CLEARANCE_X: f64 = 250.0;
const RIGHT_WASTE_BAGOUT_CLEARANCE_X: f64 = 240.0;
const OVERHEAD_FILTER_LIFT_CLEARANCE_Z: f64 = 390.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_deck(self) -> bool {
        let usable_x = DECK_X / 2.0 - RIM_W - 12.0;
        let usable_y = DECK_Y / 2.0 - RIM_W - 12.0;

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

    let base = base_cascade_deck();
    export(OUTPUTS[0], &base);

    let vestibule = pressure_cascade_vestibule();
    export(OUTPUTS[1], &vestibule);

    let doors = exit_reentry_interlock_doors();
    export(OUTPUTS[2], &doors);

    let gowns = clean_gown_staging_nests();
    export(OUTPUTS[3], &gowns);

    let supply = positive_pressure_supply_diffuser();
    export(OUTPUTS[4], &supply);

    let returns = return_exhaust_bypass_path();
    export(OUTPUTS[5], &returns);

    let particles = particle_capture_floor_coupons();
    export(OUTPUTS[6], &particles);

    let hand_check = hand_sanitization_glove_check_panel();
    export(OUTPUTS[7], &hand_check);

    let sensors = pressure_recovery_sensor_tree();
    export(OUTPUTS[8], &sensors);

    let trace = barcode_training_release_lands();
    export(OUTPUTS[9], &trace);

    let waste = doff_waste_bagout_collar();
    export(OUTPUTS[10], &waste);

    let bridge = evidence_camera_bridge();
    export(OUTPUTS[11], &bridge);

    let assembly = base
        + vestibule
        + doors
        + gowns
        + supply
        + returns
        + particles
        + hand_check
        + sensors
        + trace
        + waste
        + bridge;
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed positive-pressure gowning exit/reentry station:");
    println!("  Footprint:               {DECK_X:.0}mm x {DECK_Y:.0}mm boundary deck");
    println!(
        "  Pressure cascade:        {CASCADE_ZONE_COUNT} zones at {:?} Pa with {CASCADE_TAP_COUNT} test taps and {DIFFUSER_SLOTS_PER_ZONE} diffuser slots per zone",
        CASCADE_SETPOINTS_PA
    );
    println!(
        "  Exit/reentry interlock:  {INTERLOCK_PIN_COUNT} captive interlock pins, {LATCH_BLOCK_COUNT} latch witness blocks, {DOOR_STATE_TOKEN_COUNT} door-state tokens"
    );
    println!(
        "  Gowning custody:         {GOWN_CANISTER_COUNT} gown canisters, {BOOTIE_NEST_COUNT} bootie nests, {GLOVE_BOX_COUNT} glove boxes, {GOWN_RFID_PUCK_COUNT} RFID pucks"
    );
    println!(
        "  Air handling evidence:   {HEPA_CASSETTE_COUNT} supply HEPA cassettes, {SUPPLY_SLOT_COUNT} supply slots, {RETURN_SLOT_COUNT} return slots, {BYPASS_DAMPER_COUNT} bypass dampers"
    );
    println!(
        "  Reentry evidence:        {STICKY_TILE_COUNT} particle capture tiles, {DP_SENSOR_COUNT} DP sensors, {PARTICLE_COUNTER_PORT_COUNT} counter ports, {BARCODE_LAND_COUNT} barcode lands"
    );
    println!(
        "  Doffing controls:        {WASTE_QUARANTINE_BAY_COUNT} quarantine bays, {WASTE_TIE_SADDLE_COUNT} waste tie saddles, {DOFF_SEQUENCE_TOKEN_COUNT} sequence tokens"
    );
    println!(
        "  Evidence bridge:         {CAMERA_POD_COUNT} camera pods and {LIGHT_BAR_COUNT} light bars over all exit/reentry evidence zones"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn layout_rects() -> [Rect; 9] {
    [
        rect(
            "pressure_cascade_vestibule",
            VESTIBULE_POS,
            VESTIBULE_X,
            VESTIBULE_Y,
        ),
        rect(
            "exit_reentry_interlock_doors",
            DOOR_POS,
            DOOR_ZONE_X,
            DOOR_ZONE_Y,
        ),
        rect(
            "clean_gown_staging_nests",
            GOWN_POS,
            GOWN_ZONE_X,
            GOWN_ZONE_Y,
        ),
        rect(
            "positive_pressure_supply_diffuser",
            SUPPLY_POS,
            SUPPLY_ZONE_X,
            SUPPLY_ZONE_Y,
        ),
        rect(
            "return_exhaust_bypass_path",
            RETURN_POS,
            RETURN_ZONE_X,
            RETURN_ZONE_Y,
        ),
        rect(
            "particle_capture_floor_coupons",
            PARTICLE_POS,
            PARTICLE_ZONE_X,
            PARTICLE_ZONE_Y,
        ),
        rect(
            "pressure_recovery_sensor_tree",
            SENSOR_POS,
            SENSOR_ZONE_X,
            SENSOR_ZONE_Y,
        ),
        rect(
            "barcode_training_release_lands",
            TRACE_POS,
            TRACE_ZONE_X,
            TRACE_ZONE_Y,
        ),
        rect(
            "doff_waste_bagout_collar",
            WASTE_POS,
            WASTE_ZONE_X,
            WASTE_ZONE_Y,
        ),
    ]
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn insert_z(height: f64) -> f64 {
    DECK_Z + height / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn assert_design_constraints() {
    for path in OUTPUTS {
        assert!(
            path.contains(PREFIX),
            "{path} must retain station-specific prefix"
        );
    }

    for feature in REQUIRED_FEATURES {
        assert!(
            OUTPUTS.iter().any(|path| path.contains(feature)),
            "{feature} is not represented by an exported STL"
        );
    }

    for zone in layout_rects() {
        assert!(
            zone.fits_inside_deck(),
            "{} exceeds usable positive-pressure deck envelope",
            zone.name
        );
    }

    let zones = layout_rects();
    for i in 0..zones.len() {
        for j in (i + 1)..zones.len() {
            assert!(
                !zones[i].overlaps(zones[j]),
                "{} overlaps {}",
                zones[i].name,
                zones[j].name
            );
        }
    }

    for pair in CASCADE_SETPOINTS_PA.windows(2) {
        assert!(
            pair[0] - pair[1] >= MIN_POSITIVE_STEP_PA,
            "pressure cascade must retain a measurable positive step"
        );
    }

    assert_eq!(STICKY_TILE_COUNT, STICKY_TILE_ROWS * STICKY_TILE_COLS);
    assert_eq!(CASCADE_TAP_COUNT, CASCADE_ZONE_COUNT * 2);
    assert_eq!(MOUNT_COUNT, 12);
    assert_eq!(RELEASE_HOLD_REJECT_LANES, 3);
    assert!(DOOR_OPENING_X < DOOR_FRAME_X);
    assert!(DOOR_OPENING_Z < DOOR_FRAME_Z);
    assert!(WASTE_COLLAR_INNER_R < WASTE_COLLAR_OUTER_R);
    assert!(BRIDGE_UNDERSIDE_Z > SUPPLY_PLENUM_Z + 160.0);
    assert!(OVERHEAD_FILTER_LIFT_CLEARANCE_Z > BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z);
}

fn base_cascade_deck() -> Part {
    let deck = centered_cube(
        "positive_pressure_gowning_base_cascade_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let recessed_pan = centered_cube(
        "positive_pressure_gowning_wipeable_recessed_floor_pan",
        DECK_X - 145.0,
        DECK_Y - 132.0,
        7.0,
    )
    .translate(0.0, -5.0, DECK_Z - 3.5);
    let low_point_channel = centered_cube(
        "positive_pressure_gowning_low_point_cleaning_channel",
        DECK_X - 260.0,
        14.0,
        8.0,
    )
    .translate(0.0, -260.0, DECK_Z - 4.0);

    deck - recessed_pan - low_point_channel - zone_sockets() - deck_mount_holes()
        + perimeter_rim()
        + lane_dividers()
        + deck_airflow_arrows()
        + datum_fiducials()
}

fn zone_sockets() -> Part {
    let mut sockets = Part::empty("positive_pressure_gowning_registration_sockets");
    for zone in layout_rects() {
        sockets = sockets
            + centered_cube(
                format!("positive_pressure_gowning_socket_{}", zone.name),
                zone.x + 16.0,
                zone.y + 16.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(
                zone.center.0,
                zone.center.1,
                DECK_Z - SOCKET_DEPTH / 2.0 + 0.2,
            );
    }
    sockets
}

fn deck_mount_holes() -> Part {
    let mount_points = [
        (-DECK_X / 2.0 + 64.0, -DECK_Y / 2.0 + 62.0),
        (DECK_X / 2.0 - 64.0, -DECK_Y / 2.0 + 62.0),
        (-DECK_X / 2.0 + 64.0, DECK_Y / 2.0 - 62.0),
        (DECK_X / 2.0 - 64.0, DECK_Y / 2.0 - 62.0),
        (-DECK_X / 6.0, -DECK_Y / 2.0 + 62.0),
        (DECK_X / 6.0, -DECK_Y / 2.0 + 62.0),
        (-DECK_X / 6.0, DECK_Y / 2.0 - 62.0),
        (DECK_X / 6.0, DECK_Y / 2.0 - 62.0),
        (VESTIBULE_POS.0, VESTIBULE_POS.1),
        (DOOR_POS.0, DOOR_POS.1),
        (SUPPLY_POS.0, SUPPLY_POS.1),
        (RETURN_POS.0, RETURN_POS.1),
    ];

    assert_eq!(mount_points.len(), MOUNT_COUNT);

    let mut holes = Part::empty("positive_pressure_gowning_deck_mount_holes");
    for (i, (x, y)) in mount_points.into_iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("positive_pressure_gowning_m6_mount_clearance_{i}"),
                3.5,
                DECK_Z + 5.0,
                28,
            )
            .translate(x, y, DECK_Z / 2.0);
    }
    holes
}

fn perimeter_rim() -> Part {
    let front = centered_cube(
        "positive_pressure_gowning_front_reentry_robot_stop_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + RIM_W / 2.0, insert_z(RIM_Z));
    let rear = centered_cube(
        "positive_pressure_gowning_rear_hepa_service_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, insert_z(RIM_Z));
    let left = centered_cube(
        "positive_pressure_gowning_left_sensor_service_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, insert_z(RIM_Z));
    let right = centered_cube(
        "positive_pressure_gowning_right_waste_bagout_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, insert_z(RIM_Z));

    front + rear + left + right
}

fn lane_dividers() -> Part {
    let positive_to_room = centered_cube(
        "positive_pressure_gowning_high_to_low_pressure_lane_divider",
        DECK_X - 180.0,
        12.0,
        30.0,
    )
    .translate(0.0, 22.0, DECK_Z + 15.0);
    let evidence_lane = centered_cube(
        "positive_pressure_gowning_evidence_and_release_lane_divider",
        DECK_X - 220.0,
        10.0,
        26.0,
    )
    .translate(0.0, -255.0, DECK_Z + 13.0);
    let doff_boundary = centered_cube(
        "positive_pressure_gowning_clean_gown_to_doff_boundary",
        10.0,
        735.0,
        30.0,
    )
    .translate(350.0, -35.0, DECK_Z + 15.0);

    positive_to_room + evidence_lane + doff_boundary
}

fn deck_airflow_arrows() -> Part {
    let mut arrows = Part::empty("positive_pressure_gowning_deck_airflow_arrows");
    for (i, y) in [295.0, 210.0, 125.0, -70.0, -150.0, -330.0]
        .into_iter()
        .enumerate()
    {
        arrows = arrows
            + flow_arrow(
                format!("positive_pressure_gowning_clean_to_room_flow_arrow_{i}"),
                86.0,
            )
            .translate(-210.0 + i as f64 * 72.0, y, DECK_Z + 7.0);
    }
    arrows
}

fn datum_fiducials() -> Part {
    let mut fiducials = Part::empty("positive_pressure_gowning_robot_datum_fiducials");
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 110.0, -DECK_Y / 2.0 + 106.0),
        (DECK_X / 2.0 - 110.0, -DECK_Y / 2.0 + 106.0),
        (-DECK_X / 2.0 + 110.0, DECK_Y / 2.0 - 106.0),
        (DECK_X / 2.0 - 110.0, DECK_Y / 2.0 - 106.0),
    ]
    .into_iter()
    .enumerate()
    {
        fiducials = fiducials
            + fiducial_target(&format!("positive_pressure_gowning_datum_{i}")).translate(
                x,
                y,
                DECK_Z + 2.0,
            );
    }
    fiducials
}

fn fiducial_target(name: &str) -> Part {
    let outer = centered_cylinder(format!("{name}_outer_ring"), 18.0, 3.0, 40);
    let inner = centered_cylinder(format!("{name}_center_clearance"), 8.0, 4.0, 32);
    let cross_x = centered_cube(format!("{name}_cross_x"), 36.0, 3.5, 3.0);
    let cross_y = centered_cube(format!("{name}_cross_y"), 3.5, 36.0, 3.0);

    outer - inner + cross_x + cross_y
}

fn pressure_cascade_vestibule() -> Part {
    let floor = centered_cube(
        "positive_pressure_cascade_vestibule_floor_plate",
        VESTIBULE_X - 34.0,
        VESTIBULE_Y - 34.0,
        VESTIBULE_Z,
    )
    .translate(VESTIBULE_POS.0, VESTIBULE_POS.1, insert_z(VESTIBULE_Z));

    floor
        + cascade_zone_bays()
        + cascade_side_walls()
        + cascade_pressure_taps()
        + cascade_seal_shim_rack()
        + cascade_floor_datums()
}

fn cascade_zone_bays() -> Part {
    let mut bays = Part::empty("positive_pressure_cascade_zone_bays");
    for zone in 0..CASCADE_ZONE_COUNT {
        let x = VESTIBULE_POS.0 + centered_index(zone, CASCADE_ZONE_COUNT, CASCADE_BAY_PITCH_X);
        let plate = centered_cube(
            format!("positive_pressure_cascade_zone_{zone}_floor_land"),
            CASCADE_BAY_X,
            CASCADE_BAY_Y,
            12.0,
        )
        .translate(x, VESTIBULE_POS.1, DECK_Z + VESTIBULE_Z + 6.0);
        let divider = centered_cube(
            format!("positive_pressure_cascade_zone_{zone}_raised_boundary"),
            10.0,
            CASCADE_BAY_Y + 22.0,
            76.0,
        )
        .translate(
            x - CASCADE_BAY_X / 2.0 - 8.0,
            VESTIBULE_POS.1,
            DECK_Z + VESTIBULE_Z + 38.0,
        );
        let label = centered_cube(
            format!("positive_pressure_cascade_zone_{zone}_setpoint_land"),
            68.0,
            20.0,
            5.0,
        )
        .translate(
            x,
            VESTIBULE_POS.1 - CASCADE_BAY_Y / 2.0 + 26.0,
            DECK_Z + VESTIBULE_Z + 14.5,
        );
        bays = bays + plate + divider + label + cascade_diffuser_slots(zone, x);
    }
    bays
}

fn cascade_diffuser_slots(zone: usize, zone_x: f64) -> Part {
    let mut slots = Part::empty(format!(
        "positive_pressure_cascade_zone_{zone}_diffuser_slots"
    ));
    for slot in 0..DIFFUSER_SLOTS_PER_ZONE {
        slots = slots
            + centered_cube(
                format!("positive_pressure_cascade_zone_{zone}_diffuser_slot_{slot}"),
                CASCADE_BAY_X - 26.0,
                7.0,
                8.0,
            )
            .translate(
                zone_x,
                VESTIBULE_POS.1 + centered_index(slot, DIFFUSER_SLOTS_PER_ZONE, 38.0),
                DECK_Z + VESTIBULE_Z + 21.0,
            );
    }
    slots
}

fn cascade_side_walls() -> Part {
    let rear = centered_cube(
        "positive_pressure_cascade_rear_clean_side_wall",
        VESTIBULE_X - 58.0,
        14.0,
        98.0,
    )
    .translate(
        VESTIBULE_POS.0,
        VESTIBULE_POS.1 + VESTIBULE_Y / 2.0 - 36.0,
        DECK_Z + VESTIBULE_Z + 49.0,
    );
    let front = centered_cube(
        "positive_pressure_cascade_front_room_side_wall",
        VESTIBULE_X - 58.0,
        14.0,
        70.0,
    )
    .translate(
        VESTIBULE_POS.0,
        VESTIBULE_POS.1 - VESTIBULE_Y / 2.0 + 36.0,
        DECK_Z + VESTIBULE_Z + 35.0,
    );
    let left = centered_cube(
        "positive_pressure_cascade_left_boundary_wall",
        14.0,
        VESTIBULE_Y - 70.0,
        90.0,
    )
    .translate(
        VESTIBULE_POS.0 - VESTIBULE_X / 2.0 + 36.0,
        VESTIBULE_POS.1,
        DECK_Z + VESTIBULE_Z + 45.0,
    );
    let right = centered_cube(
        "positive_pressure_cascade_right_door_transition_wall",
        14.0,
        VESTIBULE_Y - 70.0,
        90.0,
    )
    .translate(
        VESTIBULE_POS.0 + VESTIBULE_X / 2.0 - 36.0,
        VESTIBULE_POS.1,
        DECK_Z + VESTIBULE_Z + 45.0,
    );

    rear + front + left + right
}

fn cascade_pressure_taps() -> Part {
    let mut taps = Part::empty("positive_pressure_cascade_pressure_taps");
    for zone in 0..CASCADE_ZONE_COUNT {
        let x = VESTIBULE_POS.0 + centered_index(zone, CASCADE_ZONE_COUNT, CASCADE_BAY_PITCH_X);
        for side in 0..2 {
            let y = VESTIBULE_POS.1 + if side == 0 { -112.0 } else { 112.0 };
            let boss = centered_cylinder(
                format!("positive_pressure_cascade_zone_{zone}_dp_tap_boss_{side}"),
                9.0,
                18.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, y, DECK_Z + VESTIBULE_Z + 66.0);
            let bore = centered_cylinder(
                format!("positive_pressure_cascade_zone_{zone}_dp_tap_bore_{side}"),
                2.2,
                22.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, y, DECK_Z + VESTIBULE_Z + 66.0);
            taps = taps + (boss - bore);
        }
    }
    taps
}

fn cascade_seal_shim_rack() -> Part {
    let rack = centered_cube(
        "positive_pressure_cascade_door_seal_shim_rack",
        188.0,
        34.0,
        24.0,
    )
    .translate(
        VESTIBULE_POS.0,
        VESTIBULE_POS.1 - VESTIBULE_Y / 2.0 + 62.0,
        DECK_Z + VESTIBULE_Z + 22.0,
    );
    let mut shims = Part::empty("positive_pressure_cascade_seal_shim_witnesses");
    for i in 0..CASCADE_SEAL_SHIM_COUNT {
        shims = shims
            + centered_cube(
                format!("positive_pressure_cascade_seal_shim_{i}"),
                20.0,
                18.0,
                7.0,
            )
            .translate(
                VESTIBULE_POS.0 + centered_index(i, CASCADE_SEAL_SHIM_COUNT, 27.0),
                VESTIBULE_POS.1 - VESTIBULE_Y / 2.0 + 62.0,
                DECK_Z + VESTIBULE_Z + 39.5,
            );
    }
    rack + shims
}

fn cascade_floor_datums() -> Part {
    let mut datums = Part::empty("positive_pressure_cascade_floor_datums");
    for (i, x) in [-192.0, -64.0, 64.0, 192.0].into_iter().enumerate() {
        datums = datums
            + centered_cylinder(
                format!("positive_pressure_cascade_floor_datum_pin_{i}"),
                4.0,
                14.0,
                24,
            )
            .translate(
                VESTIBULE_POS.0 + x,
                VESTIBULE_POS.1 + VESTIBULE_Y / 2.0 - 70.0,
                DECK_Z + VESTIBULE_Z + 7.0,
            );
    }
    datums
}

fn exit_reentry_interlock_doors() -> Part {
    let saddle = centered_cube(
        "positive_pressure_exit_reentry_interlock_floor_saddle",
        DOOR_ZONE_X - 36.0,
        DOOR_ZONE_Y - 36.0,
        32.0,
    )
    .translate(DOOR_POS.0, DOOR_POS.1, insert_z(32.0));

    saddle
        + door_frame("exit_outer_room_side_door", -DOOR_PAIR_SPACING_Y / 2.0)
        + door_frame("reentry_inner_clean_side_door", DOOR_PAIR_SPACING_Y / 2.0)
        + interlock_pin_bank()
        + latch_witness_blocks()
        + door_state_tokens()
        + reentry_dwell_timer_slot()
}

fn door_frame(label: &str, y_offset: f64) -> Part {
    let frame = rectangular_frame_xz(
        &format!("positive_pressure_{label}_gasket_frame"),
        DOOR_FRAME_X,
        DOOR_FACE_Y,
        DOOR_FRAME_Z,
        DOOR_OPENING_X,
        DOOR_OPENING_Z,
    )
    .translate(
        DOOR_POS.0,
        DOOR_POS.1 + y_offset,
        DECK_Z + 32.0 + DOOR_FRAME_Z / 2.0,
    );
    let window = centered_cube(
        format!("positive_pressure_{label}_clear_window_witness_land"),
        DOOR_OPENING_X - 34.0,
        5.0,
        22.0,
    )
    .translate(
        DOOR_POS.0,
        DOOR_POS.1 + y_offset,
        DECK_Z + 32.0 + DOOR_FRAME_Z / 2.0,
    );
    let hinge = centered_cylinder(
        format!("positive_pressure_{label}_hinge_bar"),
        7.0,
        210.0,
        24,
    )
    .translate(
        DOOR_POS.0 - DOOR_FRAME_X / 2.0 - 20.0,
        DOOR_POS.1 + y_offset,
        DECK_Z + 32.0 + DOOR_FRAME_Z / 2.0,
    );
    let handle = centered_cube(
        format!("positive_pressure_{label}_sealed_handle_land"),
        18.0,
        16.0,
        54.0,
    )
    .translate(
        DOOR_POS.0 + DOOR_FRAME_X / 2.0 - 36.0,
        DOOR_POS.1 + y_offset,
        DECK_Z + 32.0 + 118.0,
    );

    frame + window + hinge + handle
}

fn interlock_pin_bank() -> Part {
    let mut pins = Part::empty("positive_pressure_exit_reentry_interlock_pin_bank");
    for i in 0..INTERLOCK_PIN_COUNT {
        let x = DOOR_POS.0 + centered_index(i, INTERLOCK_PIN_COUNT, 34.0);
        let pin = centered_cylinder(
            format!("positive_pressure_exit_reentry_captive_interlock_pin_{i}"),
            5.0,
            DOOR_PAIR_SPACING_Y + 84.0,
            22,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, DOOR_POS.1, DECK_Z + 32.0 + DOOR_FRAME_Z + 18.0);
        let witness = centered_cube(
            format!("positive_pressure_exit_reentry_interlock_pin_witness_land_{i}"),
            22.0,
            16.0,
            16.0,
        )
        .translate(
            x,
            DOOR_POS.1 - DOOR_PAIR_SPACING_Y / 2.0 - 35.0,
            DECK_Z + 32.0 + DOOR_FRAME_Z + 18.0,
        );
        pins = pins + pin + witness;
    }
    pins
}

fn latch_witness_blocks() -> Part {
    let mut blocks = Part::empty("positive_pressure_exit_reentry_latch_witness_blocks");
    for i in 0..LATCH_BLOCK_COUNT {
        let door_side = if i < LATCH_BLOCK_COUNT / 2 {
            -DOOR_PAIR_SPACING_Y / 2.0
        } else {
            DOOR_PAIR_SPACING_Y / 2.0
        };
        let side = if i % 2 == 0 { -1.0 } else { 1.0 };
        let z = DECK_Z + 32.0 + 78.0 + ((i / 2) % 2) as f64 * 74.0;
        blocks = blocks
            + centered_cube(
                format!("positive_pressure_exit_reentry_latch_witness_block_{i}"),
                28.0,
                18.0,
                30.0,
            )
            .translate(
                DOOR_POS.0 + side * (DOOR_FRAME_X / 2.0 + 34.0),
                DOOR_POS.1 + door_side,
                z,
            );
    }
    blocks
}

fn door_state_tokens() -> Part {
    let mut tokens = Part::empty("positive_pressure_exit_reentry_door_state_tokens");
    for i in 0..DOOR_STATE_TOKEN_COUNT {
        tokens = tokens
            + centered_cylinder(
                format!("positive_pressure_exit_reentry_state_token_socket_{i}"),
                13.0,
                7.0,
                28,
            )
            .translate(
                DOOR_POS.0 + centered_index(i, DOOR_STATE_TOKEN_COUNT, 48.0),
                DOOR_POS.1 - DOOR_ZONE_Y / 2.0 + 42.0,
                DECK_Z + 32.0 + 7.0,
            );
    }
    tokens
}

fn reentry_dwell_timer_slot() -> Part {
    let slot = centered_cube(
        "positive_pressure_reentry_dwell_timer_card_slot",
        192.0,
        28.0,
        32.0,
    )
    .translate(
        DOOR_POS.0,
        DOOR_POS.1 + DOOR_ZONE_Y / 2.0 - 42.0,
        DECK_Z + 32.0 + 16.0,
    );
    let card_land = centered_cube(
        "positive_pressure_reentry_dwell_timer_evidence_card_land",
        146.0,
        22.0,
        5.0,
    )
    .translate(
        DOOR_POS.0,
        DOOR_POS.1 + DOOR_ZONE_Y / 2.0 - 42.0,
        DECK_Z + 32.0 + 34.5,
    );

    slot + card_land
}

fn clean_gown_staging_nests() -> Part {
    let panel = centered_cube(
        "positive_pressure_clean_gown_staging_panel",
        GOWN_ZONE_X - 32.0,
        GOWN_ZONE_Y - 32.0,
        GOWN_PANEL_Z,
    )
    .translate(GOWN_POS.0, GOWN_POS.1, insert_z(GOWN_PANEL_Z));

    panel - gown_canister_pockets() - bootie_nests() - glove_box_pockets()
        + gown_canister_lips()
        + gown_rfid_pucks()
        + gown_return_bins()
        + clean_gown_pick_datums()
}

fn gown_canister_pockets() -> Part {
    let mut pockets = Part::empty("positive_pressure_clean_gown_canister_pockets");
    for i in 0..GOWN_CANISTER_COUNT {
        pockets = pockets
            + centered_cylinder(
                format!("positive_pressure_clean_gown_canister_socket_{i}"),
                22.0,
                GOWN_PANEL_Z + 5.0,
                36,
            )
            .translate(
                GOWN_POS.0 + centered_index(i % 3, 3, 76.0),
                GOWN_POS.1 + 76.0 - (i / 3) as f64 * 70.0,
                DECK_Z + GOWN_PANEL_Z / 2.0,
            );
    }
    pockets
}

fn bootie_nests() -> Part {
    let mut nests = Part::empty("positive_pressure_bootie_nest_recesses");
    for i in 0..BOOTIE_NEST_COUNT {
        nests = nests
            + centered_cube(
                format!("positive_pressure_bootie_pair_nest_{i}"),
                58.0,
                34.0,
                GOWN_PANEL_Z + 5.0,
            )
            .translate(
                GOWN_POS.0 + centered_index(i, BOOTIE_NEST_COUNT, 60.0),
                GOWN_POS.1 - 72.0,
                DECK_Z + GOWN_PANEL_Z / 2.0,
            );
    }
    nests
}

fn glove_box_pockets() -> Part {
    let mut pockets = Part::empty("positive_pressure_glove_box_pockets");
    for i in 0..GLOVE_BOX_COUNT {
        pockets = pockets
            + centered_cube(
                format!("positive_pressure_glove_box_socket_{i}"),
                52.0,
                32.0,
                GOWN_PANEL_Z + 5.0,
            )
            .translate(
                GOWN_POS.0 + centered_index(i, GLOVE_BOX_COUNT, 58.0),
                GOWN_POS.1 - 120.0,
                DECK_Z + GOWN_PANEL_Z / 2.0,
            );
    }
    pockets
}

fn gown_canister_lips() -> Part {
    let mut lips = Part::empty("positive_pressure_clean_gown_canister_retention_lips");
    for i in 0..GOWN_CANISTER_COUNT {
        lips =
            lips + centered_cylinder(
                format!("positive_pressure_clean_gown_canister_retention_ring_{i}"),
                27.0,
                8.0,
                36,
            )
            .translate(
                GOWN_POS.0 + centered_index(i % 3, 3, 76.0),
                GOWN_POS.1 + 76.0 - (i / 3) as f64 * 70.0,
                DECK_Z + GOWN_PANEL_Z + 4.0,
            ) - centered_cylinder(
                format!("positive_pressure_clean_gown_canister_retention_clearance_{i}"),
                22.0,
                10.0,
                36,
            )
            .translate(
                GOWN_POS.0 + centered_index(i % 3, 3, 76.0),
                GOWN_POS.1 + 76.0 - (i / 3) as f64 * 70.0,
                DECK_Z + GOWN_PANEL_Z + 4.0,
            );
    }
    lips
}

fn gown_rfid_pucks() -> Part {
    let mut pucks = Part::empty("positive_pressure_gown_rfid_puck_lands");
    for i in 0..GOWN_RFID_PUCK_COUNT {
        pucks = pucks
            + centered_cylinder(
                format!("positive_pressure_gown_rfid_puck_land_{i}"),
                12.0,
                4.0,
                28,
            )
            .translate(
                GOWN_POS.0 + centered_index(i % 3, 3, 76.0),
                GOWN_POS.1 + 108.0 - (i / 3) as f64 * 70.0,
                DECK_Z + GOWN_PANEL_Z + 2.0,
            );
    }
    pucks
}

fn gown_return_bins() -> Part {
    let mut bins = Part::empty("positive_pressure_clean_gown_return_bins");
    for i in 0..GOWN_RETURN_BIN_COUNT {
        let bin = centered_cube(
            format!("positive_pressure_gown_return_bin_{i}"),
            88.0,
            44.0,
            36.0,
        )
        .translate(
            GOWN_POS.0 + centered_index(i, GOWN_RETURN_BIN_COUNT, 100.0),
            GOWN_POS.1 + GOWN_ZONE_Y / 2.0 - 45.0,
            DECK_Z + GOWN_PANEL_Z + 18.0,
        );
        let cut = centered_cube(
            format!("positive_pressure_gown_return_bin_{i}_open_top"),
            68.0,
            28.0,
            32.0,
        )
        .translate(
            GOWN_POS.0 + centered_index(i, GOWN_RETURN_BIN_COUNT, 100.0),
            GOWN_POS.1 + GOWN_ZONE_Y / 2.0 - 45.0,
            DECK_Z + GOWN_PANEL_Z + 26.0,
        );
        bins = bins + (bin - cut);
    }
    bins
}

fn clean_gown_pick_datums() -> Part {
    let left = centered_cube("positive_pressure_gown_pick_left_datum", 16.0, 220.0, 22.0)
        .translate(
            GOWN_POS.0 - GOWN_ZONE_X / 2.0 + 28.0,
            GOWN_POS.1,
            DECK_Z + GOWN_PANEL_Z + 11.0,
        );
    let right = centered_cube("positive_pressure_gown_pick_right_datum", 16.0, 220.0, 22.0)
        .translate(
            GOWN_POS.0 + GOWN_ZONE_X / 2.0 - 28.0,
            GOWN_POS.1,
            DECK_Z + GOWN_PANEL_Z + 11.0,
        );

    left + right
}

fn positive_pressure_supply_diffuser() -> Part {
    let plenum = centered_cube(
        "positive_pressure_supply_diffuser_plenum_shell",
        SUPPLY_ZONE_X - 34.0,
        SUPPLY_ZONE_Y - 38.0,
        SUPPLY_PLENUM_Z,
    )
    .translate(SUPPLY_POS.0, SUPPLY_POS.1, insert_z(SUPPLY_PLENUM_Z));
    let void = centered_cube(
        "positive_pressure_supply_diffuser_plenum_void",
        SUPPLY_ZONE_X - 92.0,
        SUPPLY_ZONE_Y - 88.0,
        SUPPLY_PLENUM_Z - 24.0,
    )
    .translate(
        SUPPLY_POS.0,
        SUPPLY_POS.1,
        DECK_Z + SUPPLY_PLENUM_Z / 2.0 + 2.0,
    );

    (plenum - void)
        + supply_hepa_frames()
        + supply_slot_array()
        + mfc_calibration_ports()
        + supply_balance_vanes()
}

fn supply_hepa_frames() -> Part {
    let mut frames = Part::empty("positive_pressure_supply_hepa_cassette_frames");
    for i in 0..HEPA_CASSETTE_COUNT {
        let x = SUPPLY_POS.0 + centered_index(i, HEPA_CASSETTE_COUNT, 214.0);
        let frame = rectangular_frame_xy(
            &format!("positive_pressure_supply_hepa_cassette_frame_{i}"),
            182.0,
            92.0,
            10.0,
            142.0,
            58.0,
        )
        .translate(x, SUPPLY_POS.1 + 38.0, DECK_Z + SUPPLY_PLENUM_Z + 5.0);
        frames = frames + frame;
    }
    frames
}

fn supply_slot_array() -> Part {
    let mut slots = Part::empty("positive_pressure_supply_diffuser_slot_array");
    for i in 0..SUPPLY_SLOT_COUNT {
        let x = SUPPLY_POS.0 + centered_index(i % 6, 6, 66.0);
        let y = SUPPLY_POS.1 - 54.0 + (i / 6) as f64 * 44.0;
        slots = slots
            + centered_cube(
                format!("positive_pressure_supply_laminar_slot_{i}"),
                48.0,
                8.0,
                8.0,
            )
            .translate(x, y, DECK_Z + SUPPLY_PLENUM_Z + 14.0);
    }
    slots
}

fn mfc_calibration_ports() -> Part {
    let mut ports = Part::empty("positive_pressure_supply_mfc_calibration_ports");
    for i in 0..MFC_PORT_COUNT {
        let boss = centered_cylinder(
            format!("positive_pressure_supply_mfc_port_boss_{i}"),
            10.0,
            18.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            SUPPLY_POS.0 + centered_index(i, MFC_PORT_COUNT, 64.0),
            SUPPLY_POS.1 - SUPPLY_ZONE_Y / 2.0 + 24.0,
            DECK_Z + 48.0,
        );
        let bore = centered_cylinder(
            format!("positive_pressure_supply_mfc_port_bore_{i}"),
            2.5,
            22.0,
            20,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            SUPPLY_POS.0 + centered_index(i, MFC_PORT_COUNT, 64.0),
            SUPPLY_POS.1 - SUPPLY_ZONE_Y / 2.0 + 24.0,
            DECK_Z + 48.0,
        );
        ports = ports + (boss - bore);
    }
    ports
}

fn supply_balance_vanes() -> Part {
    let mut vanes = Part::empty("positive_pressure_supply_balance_vanes");
    for i in 0..SUPPLY_BALANCE_VANE_COUNT {
        vanes = vanes
            + centered_cube(
                format!("positive_pressure_supply_balance_vane_{i}"),
                8.0,
                110.0,
                28.0,
            )
            .rotate(0.0, 0.0, -18.0 + i as f64 * 5.0)
            .translate(
                SUPPLY_POS.0 + centered_index(i, SUPPLY_BALANCE_VANE_COUNT, 48.0),
                SUPPLY_POS.1,
                DECK_Z + SUPPLY_PLENUM_Z + 32.0,
            );
    }
    vanes
}

fn return_exhaust_bypass_path() -> Part {
    let plenum = centered_cube(
        "positive_pressure_return_exhaust_bypass_plenum",
        RETURN_ZONE_X - 34.0,
        RETURN_ZONE_Y - 36.0,
        RETURN_PLENUM_Z,
    )
    .translate(RETURN_POS.0, RETURN_POS.1, insert_z(RETURN_PLENUM_Z));
    let void = centered_cube(
        "positive_pressure_return_exhaust_internal_void",
        RETURN_ZONE_X - 86.0,
        RETURN_ZONE_Y - 82.0,
        RETURN_PLENUM_Z - 18.0,
    )
    .translate(
        RETURN_POS.0,
        RETURN_POS.1,
        DECK_Z + RETURN_PLENUM_Z / 2.0 + 2.0,
    );

    (plenum - void)
        + return_slot_array()
        + bypass_damper_witnesses()
        + return_filter_coupon_lane()
        + relief_flaps()
}

fn return_slot_array() -> Part {
    let mut slots = Part::empty("positive_pressure_return_exhaust_slots");
    for i in 0..RETURN_SLOT_COUNT {
        slots = slots
            + centered_cube(
                format!("positive_pressure_return_exhaust_slot_{i}"),
                44.0,
                8.0,
                8.0,
            )
            .translate(
                RETURN_POS.0 + centered_index(i, RETURN_SLOT_COUNT, 34.0),
                RETURN_POS.1 - RETURN_ZONE_Y / 2.0 + 40.0,
                DECK_Z + RETURN_PLENUM_Z + 8.0,
            );
    }
    slots
}

fn bypass_damper_witnesses() -> Part {
    let mut dampers = Part::empty("positive_pressure_bypass_damper_witnesses");
    for i in 0..BYPASS_DAMPER_COUNT {
        dampers = dampers
            + centered_cube(
                format!("positive_pressure_bypass_damper_blade_witness_{i}"),
                54.0,
                12.0,
                22.0,
            )
            .rotate(0.0, 0.0, -20.0 + i as f64 * 12.0)
            .translate(
                RETURN_POS.0 + centered_index(i, BYPASS_DAMPER_COUNT, 78.0),
                RETURN_POS.1 + 40.0,
                DECK_Z + RETURN_PLENUM_Z + 22.0,
            );
    }
    dampers
}

fn return_filter_coupon_lane() -> Part {
    let lane = centered_cube(
        "positive_pressure_return_filter_coupon_lane",
        310.0,
        42.0,
        18.0,
    )
    .translate(
        RETURN_POS.0,
        RETURN_POS.1 + 92.0,
        DECK_Z + RETURN_PLENUM_Z + 9.0,
    );
    let mut coupons = Part::empty("positive_pressure_return_filter_coupon_pockets");
    for i in 0..FILTER_COUPON_COUNT {
        coupons = coupons
            + centered_cylinder(
                format!("positive_pressure_return_filter_coupon_pocket_{i}"),
                14.0,
                20.0,
                32,
            )
            .translate(
                RETURN_POS.0 + centered_index(i, FILTER_COUPON_COUNT, 46.0),
                RETURN_POS.1 + 92.0,
                DECK_Z + RETURN_PLENUM_Z + 18.0,
            );
    }
    lane + coupons
}

fn relief_flaps() -> Part {
    let mut flaps = Part::empty("positive_pressure_return_relief_flaps");
    for i in 0..RELIEF_FLAP_COUNT {
        flaps = flaps
            + centered_cube(
                format!("positive_pressure_return_relief_flap_{i}"),
                76.0,
                12.0,
                34.0,
            )
            .translate(
                RETURN_POS.0 + centered_index(i, RELIEF_FLAP_COUNT, 102.0),
                RETURN_POS.1 - 84.0,
                DECK_Z + RETURN_PLENUM_Z + 17.0,
            );
    }
    flaps
}

fn particle_capture_floor_coupons() -> Part {
    let tray = centered_cube(
        "positive_pressure_particle_capture_floor_tray",
        PARTICLE_ZONE_X - 32.0,
        PARTICLE_ZONE_Y - 32.0,
        30.0,
    )
    .translate(PARTICLE_POS.0, PARTICLE_POS.1, insert_z(30.0));

    tray - sticky_tile_recesses() - particle_drain_ports()
        + sticky_tile_witness_lips()
        + removable_particle_coupons()
}

fn sticky_tile_recesses() -> Part {
    let mut recesses = Part::empty("positive_pressure_sticky_tile_recesses");
    for row in 0..STICKY_TILE_ROWS {
        for col in 0..STICKY_TILE_COLS {
            let idx = row * STICKY_TILE_COLS + col;
            recesses = recesses
                + centered_cube(
                    format!("positive_pressure_sticky_tile_recess_{idx}"),
                    50.0,
                    42.0,
                    12.0,
                )
                .translate(
                    PARTICLE_POS.0 + centered_index(col, STICKY_TILE_COLS, 58.0),
                    PARTICLE_POS.1 + centered_index(row, STICKY_TILE_ROWS, 50.0),
                    DECK_Z + 30.0 - 6.0,
                );
        }
    }
    recesses
}

fn sticky_tile_witness_lips() -> Part {
    let mut lips = Part::empty("positive_pressure_sticky_tile_witness_lips");
    for row in 0..STICKY_TILE_ROWS {
        for col in 0..STICKY_TILE_COLS {
            let idx = row * STICKY_TILE_COLS + col;
            lips = lips
                + centered_cube(
                    format!("positive_pressure_sticky_tile_witness_lip_{idx}"),
                    58.0,
                    5.0,
                    7.0,
                )
                .translate(
                    PARTICLE_POS.0 + centered_index(col, STICKY_TILE_COLS, 58.0),
                    PARTICLE_POS.1 + centered_index(row, STICKY_TILE_ROWS, 50.0) + 25.0,
                    DECK_Z + 33.5,
                );
        }
    }
    lips
}

fn removable_particle_coupons() -> Part {
    let mut coupons = Part::empty("positive_pressure_removable_particle_coupons");
    for i in 0..PARTICLE_COUPON_COUNT {
        coupons = coupons
            + centered_cube(
                format!("positive_pressure_particle_coupon_slide_{i}"),
                54.0,
                16.0,
                8.0,
            )
            .translate(
                PARTICLE_POS.0 + centered_index(i % 4, 4, 62.0),
                PARTICLE_POS.1 + 84.0 - (i / 4) as f64 * 34.0,
                DECK_Z + 38.0,
            );
    }
    coupons
}

fn particle_drain_ports() -> Part {
    let mut drains = Part::empty("positive_pressure_particle_capture_drain_ports");
    for i in 0..PARTICLE_DRAIN_COUNT {
        drains = drains
            + centered_cylinder(
                format!("positive_pressure_particle_capture_drain_port_{i}"),
                6.0,
                34.0,
                24,
            )
            .translate(
                PARTICLE_POS.0 + centered_index(i, PARTICLE_DRAIN_COUNT, 164.0),
                PARTICLE_POS.1 - 82.0,
                DECK_Z + 15.0,
            );
    }
    drains
}

fn hand_sanitization_glove_check_panel() -> Part {
    let panel = centered_cube(
        "positive_pressure_hand_sanitization_glove_check_panel",
        310.0,
        66.0,
        36.0,
    )
    .translate(DOOR_POS.0, DOOR_POS.1 - 4.0, DECK_Z + 32.0 + 18.0);
    let sanitizer = centered_cylinder(
        "positive_pressure_sanitizer_nozzle_witness_boss",
        13.0,
        20.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DOOR_POS.0 - 108.0, DOOR_POS.1 - 40.0, DECK_Z + 32.0 + 28.0);
    let glove_port_left = centered_cylinder(
        "positive_pressure_left_glove_integrity_test_port",
        22.0,
        24.0,
        40,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DOOR_POS.0 - 38.0, DOOR_POS.1 - 40.0, DECK_Z + 32.0 + 28.0);
    let glove_port_right = centered_cylinder(
        "positive_pressure_right_glove_integrity_test_port",
        22.0,
        24.0,
        40,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(DOOR_POS.0 + 38.0, DOOR_POS.1 - 40.0, DECK_Z + 32.0 + 28.0);
    let scan_land = centered_cube("positive_pressure_hand_check_scan_land", 78.0, 20.0, 5.0)
        .translate(DOOR_POS.0 + 116.0, DOOR_POS.1 - 40.0, DECK_Z + 32.0 + 38.5);

    panel + sanitizer + glove_port_left + glove_port_right + scan_land
}

fn pressure_recovery_sensor_tree() -> Part {
    let rail = centered_cube(
        "positive_pressure_recovery_sensor_tree_rail",
        SENSOR_ZONE_X - 36.0,
        SENSOR_ZONE_Y - 34.0,
        SENSOR_RAIL_Z,
    )
    .translate(SENSOR_POS.0, SENSOR_POS.1, insert_z(SENSOR_RAIL_Z));

    rail + dp_sensor_blocks() + particle_counter_ports() + recovery_timer_tokens() + alarm_beacons()
}

fn dp_sensor_blocks() -> Part {
    let mut sensors = Part::empty("positive_pressure_dp_sensor_blocks");
    for i in 0..DP_SENSOR_COUNT {
        let x = SENSOR_POS.0 + centered_index(i, DP_SENSOR_COUNT, 70.0);
        let block = centered_cube(
            format!("positive_pressure_dp_sensor_block_{i}"),
            48.0,
            34.0,
            24.0,
        )
        .translate(x, SENSOR_POS.1 + 36.0, DECK_Z + SENSOR_RAIL_Z + 12.0);
        let port_a = centered_cylinder(
            format!("positive_pressure_dp_sensor_{i}_high_port"),
            3.0,
            18.0,
            18,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x - 10.0, SENSOR_POS.1 + 16.0, DECK_Z + SENSOR_RAIL_Z + 16.0);
        let port_b = centered_cylinder(
            format!("positive_pressure_dp_sensor_{i}_low_port"),
            3.0,
            18.0,
            18,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x + 10.0, SENSOR_POS.1 + 16.0, DECK_Z + SENSOR_RAIL_Z + 16.0);
        sensors = sensors + block + port_a + port_b;
    }
    sensors
}

fn particle_counter_ports() -> Part {
    let mut ports = Part::empty("positive_pressure_particle_counter_sample_ports");
    for i in 0..PARTICLE_COUNTER_PORT_COUNT {
        ports = ports
            + centered_cylinder(
                format!("positive_pressure_particle_counter_port_{i}"),
                9.0,
                18.0,
                28,
            )
            .translate(
                SENSOR_POS.0 + centered_index(i, PARTICLE_COUNTER_PORT_COUNT, 92.0),
                SENSOR_POS.1 - 18.0,
                DECK_Z + SENSOR_RAIL_Z + 9.0,
            );
    }
    ports
}

fn recovery_timer_tokens() -> Part {
    let mut tokens = Part::empty("positive_pressure_recovery_timer_tokens");
    for i in 0..RECOVERY_TIMER_TOKEN_COUNT {
        tokens = tokens
            + centered_cube(
                format!("positive_pressure_reentry_recovery_timer_token_{i}"),
                46.0,
                22.0,
                7.0,
            )
            .translate(
                SENSOR_POS.0 + centered_index(i, RECOVERY_TIMER_TOKEN_COUNT, 70.0),
                SENSOR_POS.1 - 58.0,
                DECK_Z + SENSOR_RAIL_Z + 3.5,
            );
    }
    tokens
}

fn alarm_beacons() -> Part {
    let mut beacons = Part::empty("positive_pressure_alarm_beacons");
    for i in 0..ALARM_BEACON_COUNT {
        beacons = beacons
            + centered_cylinder(
                format!("positive_pressure_alarm_beacon_stack_{i}"),
                14.0,
                34.0,
                32,
            )
            .translate(
                SENSOR_POS.0 + SENSOR_ZONE_X / 2.0 - 62.0,
                SENSOR_POS.1 + centered_index(i, ALARM_BEACON_COUNT, 42.0),
                DECK_Z + SENSOR_RAIL_Z + 17.0,
            );
    }
    beacons
}

fn barcode_training_release_lands() -> Part {
    let panel = centered_cube(
        "positive_pressure_barcode_training_release_panel",
        TRACE_ZONE_X - 34.0,
        TRACE_ZONE_Y - 34.0,
        TRACE_PANEL_Z,
    )
    .translate(TRACE_POS.0, TRACE_POS.1, insert_z(TRACE_PANEL_Z));

    panel + barcode_lands() + training_card_lands() + release_hold_reject_lands() + badge_docks()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("positive_pressure_barcode_scan_lands");
    for i in 0..BARCODE_LAND_COUNT {
        lands = lands
            + centered_cube(
                format!("positive_pressure_reentry_barcode_land_{i}"),
                76.0,
                18.0,
                4.0,
            )
            .translate(
                TRACE_POS.0 + centered_index(i % 4, 4, 86.0),
                TRACE_POS.1 + 42.0 - (i / 4) as f64 * 28.0,
                DECK_Z + TRACE_PANEL_Z + 2.0,
            );
    }
    lands
}

fn training_card_lands() -> Part {
    let mut lands = Part::empty("positive_pressure_training_card_lands");
    for i in 0..TRAINING_CARD_COUNT {
        lands = lands
            + centered_cube(
                format!("positive_pressure_training_card_land_{i}"),
                62.0,
                28.0,
                5.0,
            )
            .translate(
                TRACE_POS.0 - TRACE_ZONE_X / 2.0 + 52.0,
                TRACE_POS.1 + centered_index(i, TRAINING_CARD_COUNT, 34.0),
                DECK_Z + TRACE_PANEL_Z + 2.5,
            );
    }
    lands
}

fn release_hold_reject_lands() -> Part {
    let mut lanes = Part::empty("positive_pressure_release_hold_reject_lands");
    for lane in 0..RELEASE_HOLD_REJECT_LANES {
        let y = TRACE_POS.1 - 52.0 + lane as f64 * 30.0;
        lanes = lanes
            + centered_cube(
                format!("positive_pressure_release_hold_reject_lane_{lane}"),
                220.0,
                18.0,
                6.0,
            )
            .translate(TRACE_POS.0 + 56.0, y, DECK_Z + TRACE_PANEL_Z + 3.0)
            + centered_cube(
                format!("positive_pressure_release_hold_reject_gate_tab_{lane}"),
                34.0,
                24.0,
                12.0,
            )
            .translate(
                TRACE_POS.0 + TRACE_ZONE_X / 2.0 - 52.0,
                y,
                DECK_Z + TRACE_PANEL_Z + 6.0,
            );
    }
    lanes
}

fn badge_docks() -> Part {
    let mut docks = Part::empty("positive_pressure_badge_docks");
    for i in 0..BADGE_DOCK_COUNT {
        docks = docks
            + centered_cube(
                format!("positive_pressure_personnel_badge_dock_{i}"),
                42.0,
                26.0,
                8.0,
            )
            .translate(
                TRACE_POS.0 + TRACE_ZONE_X / 2.0 - 64.0,
                TRACE_POS.1 + centered_index(i, BADGE_DOCK_COUNT, 34.0),
                DECK_Z + TRACE_PANEL_Z + 4.0,
            );
    }
    docks
}

fn doff_waste_bagout_collar() -> Part {
    let deck = centered_cube(
        "positive_pressure_doff_waste_bagout_deck",
        WASTE_ZONE_X - 34.0,
        WASTE_ZONE_Y - 34.0,
        28.0,
    )
    .translate(WASTE_POS.0, WASTE_POS.1, insert_z(28.0));
    let collar = centered_cylinder(
        "positive_pressure_doff_waste_bagout_outer_collar",
        WASTE_COLLAR_OUTER_R,
        26.0,
        72,
    )
    .translate(WASTE_POS.0 - 68.0, WASTE_POS.1, DECK_Z + 28.0 + 13.0);
    let bore = centered_cylinder(
        "positive_pressure_doff_waste_bagout_inner_bore",
        WASTE_COLLAR_INNER_R,
        30.0,
        72,
    )
    .translate(WASTE_POS.0 - 68.0, WASTE_POS.1, DECK_Z + 28.0 + 13.0);

    deck + (collar - bore) + waste_tie_saddles() + waste_quarantine_bays() + doff_sequence_tokens()
}

fn waste_tie_saddles() -> Part {
    let mut saddles = Part::empty("positive_pressure_waste_tie_saddles");
    for i in 0..WASTE_TIE_SADDLE_COUNT {
        let angle = i as f64 * 360.0 / WASTE_TIE_SADDLE_COUNT as f64;
        let (x, y) = polar_xy(angle, WASTE_COLLAR_OUTER_R + 16.0);
        saddles = saddles
            + centered_cube(
                format!("positive_pressure_waste_tie_saddle_{i}"),
                32.0,
                10.0,
                12.0,
            )
            .rotate(0.0, 0.0, angle)
            .translate(
                WASTE_POS.0 - 68.0 + x,
                WASTE_POS.1 + y,
                DECK_Z + 28.0 + 34.0,
            );
    }
    saddles
}

fn waste_quarantine_bays() -> Part {
    let mut bays = Part::empty("positive_pressure_waste_quarantine_bays");
    for i in 0..WASTE_QUARANTINE_BAY_COUNT {
        let bay = centered_cube(
            format!("positive_pressure_waste_quarantine_bay_{i}"),
            52.0,
            38.0,
            24.0,
        )
        .translate(
            WASTE_POS.0 + 62.0,
            WASTE_POS.1 + centered_index(i, WASTE_QUARANTINE_BAY_COUNT, 44.0),
            DECK_Z + 28.0 + 12.0,
        );
        let recess = centered_cube(
            format!("positive_pressure_waste_quarantine_bay_{i}_recess"),
            38.0,
            24.0,
            20.0,
        )
        .translate(
            WASTE_POS.0 + 62.0,
            WASTE_POS.1 + centered_index(i, WASTE_QUARANTINE_BAY_COUNT, 44.0),
            DECK_Z + 28.0 + 16.0,
        );
        bays = bays + (bay - recess);
    }
    bays
}

fn doff_sequence_tokens() -> Part {
    let mut tokens = Part::empty("positive_pressure_doff_sequence_tokens");
    for i in 0..DOFF_SEQUENCE_TOKEN_COUNT {
        tokens = tokens
            + centered_cylinder(
                format!("positive_pressure_doff_sequence_token_{i}"),
                9.0,
                5.0,
                24,
            )
            .translate(
                WASTE_POS.0 + centered_index(i, DOFF_SEQUENCE_TOKEN_COUNT, 34.0),
                WASTE_POS.1 - WASTE_ZONE_Y / 2.0 + 24.0,
                DECK_Z + 28.0 + 2.5,
            );
    }
    tokens
}

fn evidence_camera_bridge() -> Part {
    bridge_posts() + bridge_beams() + camera_pods() + light_bars() + keepout_gauges()
}

fn bridge_posts() -> Part {
    let mut posts = Part::empty("positive_pressure_evidence_bridge_posts");
    for (i, (x, y)) in [
        (-BRIDGE_X / 2.0, -BRIDGE_Y / 2.0),
        (BRIDGE_X / 2.0, -BRIDGE_Y / 2.0),
        (-BRIDGE_X / 2.0, BRIDGE_Y / 2.0),
        (BRIDGE_X / 2.0, BRIDGE_Y / 2.0),
    ]
    .into_iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(
                format!("positive_pressure_evidence_bridge_post_{i}"),
                28.0,
                28.0,
                BRIDGE_UNDERSIDE_Z,
            )
            .translate(x, y, BRIDGE_UNDERSIDE_Z / 2.0)
            + centered_cube(
                format!("positive_pressure_evidence_bridge_post_foot_{i}"),
                76.0,
                58.0,
                10.0,
            )
            .translate(x, y, 5.0);
    }
    posts
}

fn bridge_beams() -> Part {
    let front = centered_cube(
        "positive_pressure_evidence_bridge_front_beam",
        BRIDGE_X + 44.0,
        26.0,
        BRIDGE_BEAM_Z,
    )
    .translate(
        0.0,
        -BRIDGE_Y / 2.0,
        BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0,
    );
    let rear = centered_cube(
        "positive_pressure_evidence_bridge_rear_beam",
        BRIDGE_X + 44.0,
        26.0,
        BRIDGE_BEAM_Z,
    )
    .translate(
        0.0,
        BRIDGE_Y / 2.0,
        BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0,
    );
    let left = centered_cube(
        "positive_pressure_evidence_bridge_left_beam",
        26.0,
        BRIDGE_Y + 44.0,
        BRIDGE_BEAM_Z,
    )
    .translate(
        -BRIDGE_X / 2.0,
        0.0,
        BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0,
    );
    let right = centered_cube(
        "positive_pressure_evidence_bridge_right_beam",
        26.0,
        BRIDGE_Y + 44.0,
        BRIDGE_BEAM_Z,
    )
    .translate(
        BRIDGE_X / 2.0,
        0.0,
        BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0,
    );
    let cross = centered_cube(
        "positive_pressure_evidence_bridge_center_camera_rail",
        BRIDGE_X - 120.0,
        20.0,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, 0.0, BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z / 2.0);

    front + rear + left + right + cross
}

fn camera_pods() -> Part {
    let mut pods = Part::empty("positive_pressure_evidence_camera_pods");
    for i in 0..CAMERA_POD_COUNT {
        let pod = centered_cube(
            format!("positive_pressure_evidence_camera_pod_{i}"),
            56.0,
            44.0,
            26.0,
        )
        .translate(
            centered_index(i, CAMERA_POD_COUNT, 230.0),
            0.0,
            BRIDGE_UNDERSIDE_Z - 20.0,
        );
        let lens = centered_cylinder(
            format!("positive_pressure_evidence_camera_lens_{i}"),
            10.0,
            12.0,
            30,
        )
        .translate(
            centered_index(i, CAMERA_POD_COUNT, 230.0),
            0.0,
            BRIDGE_UNDERSIDE_Z - 39.0,
        );
        pods = pods + pod + lens;
    }
    pods
}

fn light_bars() -> Part {
    let mut bars = Part::empty("positive_pressure_evidence_light_bars");
    for (i, y) in [-270.0, -92.0, 92.0, 270.0].into_iter().enumerate() {
        bars = bars
            + centered_cube(
                format!("positive_pressure_evidence_led_bar_{i}"),
                760.0,
                12.0,
                10.0,
            )
            .translate(0.0, y, BRIDGE_UNDERSIDE_Z - 28.0);
    }
    bars
}

fn keepout_gauges() -> Part {
    let front = centered_cube(
        "positive_pressure_front_reentry_robot_clearance_gauge",
        DECK_X - 160.0,
        8.0,
        8.0,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + FRONT_REENTRY_ROBOT_CLEARANCE_Y,
        DECK_Z + 4.0,
    );
    let rear = centered_cube(
        "positive_pressure_rear_hepa_service_clearance_gauge",
        DECK_X - 160.0,
        8.0,
        8.0,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - REAR_HEPA_SERVICE_CLEARANCE_Y,
        DECK_Z + 4.0,
    );
    let left = centered_cube(
        "positive_pressure_left_sensor_service_clearance_gauge",
        8.0,
        DECK_Y - 170.0,
        8.0,
    )
    .translate(
        -DECK_X / 2.0 + LEFT_SENSOR_SERVICE_CLEARANCE_X,
        0.0,
        DECK_Z + 4.0,
    );
    let right = centered_cube(
        "positive_pressure_right_waste_bagout_clearance_gauge",
        8.0,
        DECK_Y - 170.0,
        8.0,
    )
    .translate(
        DECK_X / 2.0 - RIGHT_WASTE_BAGOUT_CLEARANCE_X,
        0.0,
        DECK_Z + 4.0,
    );
    let overhead = centered_cube(
        "positive_pressure_overhead_filter_lift_clearance_gauge",
        DECK_X - 260.0,
        DECK_Y - 260.0,
        6.0,
    )
    .translate(0.0, 0.0, OVERHEAD_FILTER_LIFT_CLEARANCE_Z);

    front + rear + left + right + overhead
}

fn rectangular_frame_xz(
    name: &str,
    outer_x: f64,
    thickness_y: f64,
    outer_z: f64,
    inner_x: f64,
    inner_z: f64,
) -> Part {
    let side_w = (outer_x - inner_x) / 2.0;
    let top_h = (outer_z - inner_z) / 2.0;
    let left = centered_cube(format!("{name}_left"), side_w, thickness_y, outer_z).translate(
        -inner_x / 2.0 - side_w / 2.0,
        0.0,
        0.0,
    );
    let right = centered_cube(format!("{name}_right"), side_w, thickness_y, outer_z).translate(
        inner_x / 2.0 + side_w / 2.0,
        0.0,
        0.0,
    );
    let top = centered_cube(format!("{name}_top"), inner_x, thickness_y, top_h).translate(
        0.0,
        0.0,
        inner_z / 2.0 + top_h / 2.0,
    );
    let bottom = centered_cube(format!("{name}_bottom"), inner_x, thickness_y, top_h).translate(
        0.0,
        0.0,
        -inner_z / 2.0 - top_h / 2.0,
    );

    left + right + top + bottom
}

fn rectangular_frame_xy(
    name: &str,
    outer_x: f64,
    outer_y: f64,
    z: f64,
    inner_x: f64,
    inner_y: f64,
) -> Part {
    let side_w = (outer_x - inner_x) / 2.0;
    let rail_y = (outer_y - inner_y) / 2.0;
    let left = centered_cube(format!("{name}_left"), side_w, outer_y, z).translate(
        -inner_x / 2.0 - side_w / 2.0,
        0.0,
        0.0,
    );
    let right = centered_cube(format!("{name}_right"), side_w, outer_y, z).translate(
        inner_x / 2.0 + side_w / 2.0,
        0.0,
        0.0,
    );
    let top = centered_cube(format!("{name}_top"), inner_x, rail_y, z).translate(
        0.0,
        inner_y / 2.0 + rail_y / 2.0,
        0.0,
    );
    let bottom = centered_cube(format!("{name}_bottom"), inner_x, rail_y, z).translate(
        0.0,
        -inner_y / 2.0 - rail_y / 2.0,
        0.0,
    );

    left + right + top + bottom
}

fn flow_arrow(name: String, length: f64) -> Part {
    let shaft = centered_cube(format!("{name}_shaft"), length, 7.0, 6.0).translate(
        length / 2.0 - 10.0,
        0.0,
        0.0,
    );
    let head = centered_cube(format!("{name}_head"), 23.0, 23.0, 6.0)
        .rotate(0.0, 0.0, 45.0)
        .translate(length - 9.0, 0.0, 0.0);

    shaft + head
}

fn polar_xy(angle_deg: f64, radius: f64) -> (f64, f64) {
    let angle = angle_deg.to_radians();
    (radius * angle.cos(), radius * angle.sin())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_constraints_hold() {
        assert_design_constraints();
    }

    #[test]
    fn outputs_are_named_for_requested_station() {
        assert_eq!(OUTPUTS.len(), 13);
        for path in OUTPUTS {
            assert!(
                path.contains(PREFIX),
                "{path} must retain station-specific prefix"
            );
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn required_features_have_export_paths() {
        for feature in REQUIRED_FEATURES {
            assert!(
                OUTPUTS.iter().any(|path| path.contains(feature)),
                "{feature} is not represented by an exported STL"
            );
        }
    }

    #[test]
    fn positive_pressure_steps_are_monotonic() {
        for pair in CASCADE_SETPOINTS_PA.windows(2) {
            assert!(pair[0] > pair[1]);
            assert!(pair[0] - pair[1] >= MIN_POSITIVE_STEP_PA);
        }
    }

    #[test]
    fn count_relationships_are_intentional() {
        assert_eq!(CASCADE_TAP_COUNT, CASCADE_ZONE_COUNT * 2);
        assert_eq!(STICKY_TILE_COUNT, STICKY_TILE_ROWS * STICKY_TILE_COLS);
        assert_eq!(MOUNT_COUNT, 12);
        assert!(INTERLOCK_PIN_COUNT >= 2 * CASCADE_ZONE_COUNT);
        assert!(GOWN_CANISTER_COUNT >= GLOVE_BOX_COUNT);
        assert!(BARCODE_LAND_COUNT >= GOWN_CANISTER_COUNT + GLOVE_BOX_COUNT);
    }

    #[test]
    fn clean_boundary_service_clearances_are_explicit() {
        assert!(FRONT_REENTRY_ROBOT_CLEARANCE_Y > 300.0);
        assert!(REAR_HEPA_SERVICE_CLEARANCE_Y > 240.0);
        assert!(LEFT_SENSOR_SERVICE_CLEARANCE_X > 220.0);
        assert!(RIGHT_WASTE_BAGOUT_CLEARANCE_X > 220.0);
        assert!(OVERHEAD_FILTER_LIFT_CLEARANCE_Z > BRIDGE_UNDERSIDE_Z);
        assert_eq!(LIGHT_BAR_COUNT, 4);
    }
}
