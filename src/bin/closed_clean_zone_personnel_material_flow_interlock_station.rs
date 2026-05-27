use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed clean-zone personnel/material flow interlock station.
//
// Intent:
// - Validate the walk-in clean-zone boundary as a controlled flow problem:
//   personnel, clean materials, dirty returns, and rejected items cannot share
//   an uncontrolled path or create simultaneous-open events.
// - Make the physical witness geometry explicit for pressure cascade,
//   one-way personnel routing, material pass-through routing, barcode/badge
//   custody, cart docking, lockout states, quarantine, and environmental
//   evidence capture.
// - Keep HEPA modules, door actuators, validated sensors, sterilization
//   recipes, gowning SOPs, and biological acceptance criteria as external
//   purchased/process controls.

const PREFIX: &str = "closed_clean_zone_personnel_material_flow_interlock_station";

const OUTPUTS: [&str; 13] = [
    "output/closed_clean_zone_personnel_material_flow_interlock_station_base_flow_deck.stl",
    "output/closed_clean_zone_personnel_material_flow_interlock_station_pressure_cascade_personnel_vestibule.stl",
    "output/closed_clean_zone_personnel_material_flow_interlock_station_material_pass_through_airlock.stl",
    "output/closed_clean_zone_personnel_material_flow_interlock_station_route_interlock_gate_array.stl",
    "output/closed_clean_zone_personnel_material_flow_interlock_station_personnel_badge_gown_release_panel.stl",
    "output/closed_clean_zone_personnel_material_flow_interlock_station_material_barcode_weight_release_panel.stl",
    "output/closed_clean_zone_personnel_material_flow_interlock_station_clean_dirty_directional_flow_floor.stl",
    "output/closed_clean_zone_personnel_material_flow_interlock_station_simultaneous_open_lockout_logic_wall.stl",
    "output/closed_clean_zone_personnel_material_flow_interlock_station_transfer_cart_docking_and_wheel_trap.stl",
    "output/closed_clean_zone_personnel_material_flow_interlock_station_reject_quarantine_hold_bays.stl",
    "output/closed_clean_zone_personnel_material_flow_interlock_station_environmental_monitoring_witness_tree.stl",
    "output/closed_clean_zone_personnel_material_flow_interlock_station_evidence_camera_and_light_bridge.stl",
    "output/closed_clean_zone_personnel_material_flow_interlock_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 12] = [
    "pressure_cascade_personnel_vestibule",
    "material_pass_through_airlock",
    "route_interlock_gate_array",
    "personnel_badge_gown_release_panel",
    "material_barcode_weight_release_panel",
    "clean_dirty_directional_flow_floor",
    "simultaneous_open_lockout_logic_wall",
    "transfer_cart_docking_and_wheel_trap",
    "reject_quarantine_hold_bays",
    "environmental_monitoring_witness_tree",
    "evidence_camera_and_light_bridge",
    "assembly",
];

const DECK_X: f64 = 1640.0;
const DECK_Y: f64 = 1040.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 46.0;
const SOCKET_DEPTH: f64 = 6.0;
const MOUNT_COUNT: usize = 12;
const DATUM_COUNT: usize = 4;

const PERSONNEL_POS: (f64, f64) = (-480.0, 210.0);
const PERSONNEL_X: f64 = 460.0;
const PERSONNEL_Y: f64 = 320.0;
const PERSONNEL_Z: f64 = 38.0;
const PERSONNEL_ZONE_COUNT: usize = 4;
const PERSONNEL_SETPOINTS_PA: [f64; PERSONNEL_ZONE_COUNT] = [38.0, 26.0, 14.0, 0.0];
const MIN_CASCADE_STEP_PA: f64 = 10.0;
const PERSONNEL_BAY_X: f64 = 96.0;
const PERSONNEL_BAY_Y: f64 = 242.0;
const PERSONNEL_BAY_PITCH_X: f64 = 108.0;
const PERSONNEL_DIFFUSER_SLOTS_PER_ZONE: usize = 5;
const PERSONNEL_PRESSURE_TAP_COUNT: usize = PERSONNEL_ZONE_COUNT * 2;
const PERSONNEL_DOOR_COUNT: usize = 3;
const PERSONNEL_DWELL_TOKEN_COUNT: usize = 4;

const MATERIAL_POS: (f64, f64) = (60.0, 210.0);
const MATERIAL_X: f64 = 420.0;
const MATERIAL_Y: f64 = 320.0;
const MATERIAL_Z: f64 = 36.0;
const MATERIAL_DOOR_FRAME_X: f64 = 292.0;
const MATERIAL_DOOR_FRAME_Z: f64 = 220.0;
const MATERIAL_DOOR_OPENING_X: f64 = 198.0;
const MATERIAL_DOOR_OPENING_Z: f64 = 136.0;
const MATERIAL_DOOR_PAIR_SPACING_Y: f64 = 210.0;
const MATERIAL_DOOR_FACE_Y: f64 = 16.0;
const MATERIAL_LATCH_COUNT: usize = 8;
const MATERIAL_INTERLOCK_PIN_COUNT: usize = 8;
const MATERIAL_TOTE_NEST_COUNT: usize = 4;
const MATERIAL_PURGE_SLOT_COUNT: usize = 8;
const MATERIAL_WIPE_COUPON_COUNT: usize = 6;

const GATE_POS: (f64, f64) = (520.0, 210.0);
const GATE_X: f64 = 300.0;
const GATE_Y: f64 = 320.0;
const GATE_Z: f64 = 34.0;
const ROUTE_STATE_COUNT: usize = 4;
const ROUTE_GATE_COUNT: usize = 6;
const CAPTIVE_KEY_COUNT: usize = 8;
const MUTUAL_EXCLUSION_BAR_COUNT: usize = 4;
const STATUS_FLAG_COUNT: usize = 6;

const BADGE_POS: (f64, f64) = (-500.0, -120.0);
const BADGE_X: f64 = 420.0;
const BADGE_Y: f64 = 220.0;
const BADGE_Z: f64 = 28.0;
const BADGE_DOCK_COUNT: usize = 6;
const GOWN_RELEASE_CANISTER_COUNT: usize = 5;
const GLOVE_CHECK_COUNT: usize = 4;
const PERSONNEL_RELEASE_LANES: usize = 3;
const HAND_SANITIZER_PORT_COUNT: usize = 2;

const BARCODE_POS: (f64, f64) = (0.0, -120.0);
const BARCODE_X: f64 = 420.0;
const BARCODE_Y: f64 = 220.0;
const BARCODE_Z: f64 = 28.0;
const MATERIAL_BARCODE_LAND_COUNT: usize = 12;
const SCALE_PAD_COUNT: usize = 4;
const TEMP_TAG_PUCK_COUNT: usize = 6;
const MATERIAL_RELEASE_LANES: usize = 3;
const LOT_CARD_SLOT_COUNT: usize = 4;

const FLOW_POS: (f64, f64) = (500.0, -120.0);
const FLOW_X: f64 = 320.0;
const FLOW_Y: f64 = 220.0;
const FLOW_Z: f64 = 18.0;
const PERSONNEL_ARROW_COUNT: usize = 5;
const MATERIAL_ARROW_COUNT: usize = 5;
const DIRTY_RETURN_ARROW_COUNT: usize = 4;
const THRESHOLD_RIB_COUNT: usize = 6;
const ANTI_BACKTRACK_PALETTE_COUNT: usize = 4;

const LOCKOUT_POS: (f64, f64) = (-500.0, -350.0);
const LOCKOUT_X: f64 = 420.0;
const LOCKOUT_Y: f64 = 160.0;
const LOCKOUT_WALL_Z: f64 = 212.0;
const LOGIC_CAM_BAR_COUNT: usize = 6;
const SIMULTANEOUS_OPEN_SENTINEL_COUNT: usize = 4;
const FAULT_INJECTION_CARD_COUNT: usize = 5;
const E_STOP_COVER_COUNT: usize = 2;

const CART_POS: (f64, f64) = (0.0, -350.0);
const CART_X: f64 = 420.0;
const CART_Y: f64 = 160.0;
const CART_Z: f64 = 26.0;
const CART_WHEEL_TRAP_COUNT: usize = 4;
const CART_DOCK_CONE_COUNT: usize = 4;
const CART_RAIL_COUNT: usize = 2;
const CART_ROUTE_TOKEN_COUNT: usize = 6;

const QUARANTINE_POS: (f64, f64) = (500.0, -350.0);
const QUARANTINE_X: f64 = 320.0;
const QUARANTINE_Y: f64 = 160.0;
const QUARANTINE_Z: f64 = 30.0;
const QUARANTINE_BAY_COUNT: usize = 4;
const QUARANTINE_LOCK_PIN_COUNT: usize = 8;
const REJECT_TAG_LAND_COUNT: usize = 4;

const SENSOR_TREE_COUNT: usize = 6;
const PARTICLE_COUNTER_PORT_COUNT: usize = 4;
const SETTLE_PLATE_HOLDER_COUNT: usize = 6;
const DP_REFERENCE_TOKEN_COUNT: usize = 4;

const BRIDGE_SPAN_X: f64 = 1370.0;
const BRIDGE_SPAN_Y: f64 = 800.0;
const BRIDGE_UNDERSIDE_Z: f64 = 318.0;
const BRIDGE_BEAM_Z: f64 = 28.0;
const CAMERA_POD_COUNT: usize = 6;
const LIGHT_BAR_COUNT: usize = 4;

const FRONT_PERSONNEL_CLEARANCE_Y: f64 = 370.0;
const REAR_PASS_THROUGH_SERVICE_Y: f64 = 280.0;
const LEFT_GOWNING_SERVICE_X: f64 = 250.0;
const RIGHT_MATERIAL_BAGOUT_X: f64 = 250.0;
const OVERHEAD_FILTER_SERVICE_Z: f64 = 410.0;

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

    let base = base_flow_deck();
    export(OUTPUTS[0], &base);

    let personnel = pressure_cascade_personnel_vestibule();
    export(OUTPUTS[1], &personnel);

    let material = material_pass_through_airlock();
    export(OUTPUTS[2], &material);

    let gates = route_interlock_gate_array();
    export(OUTPUTS[3], &gates);

    let badge = personnel_badge_gown_release_panel();
    export(OUTPUTS[4], &badge);

    let barcode = material_barcode_weight_release_panel();
    export(OUTPUTS[5], &barcode);

    let flow = clean_dirty_directional_flow_floor();
    export(OUTPUTS[6], &flow);

    let lockout = simultaneous_open_lockout_logic_wall();
    export(OUTPUTS[7], &lockout);

    let cart = transfer_cart_docking_and_wheel_trap();
    export(OUTPUTS[8], &cart);

    let quarantine = reject_quarantine_hold_bays();
    export(OUTPUTS[9], &quarantine);

    let monitors = environmental_monitoring_witness_tree();
    export(OUTPUTS[10], &monitors);

    let bridge = evidence_camera_and_light_bridge();
    export(OUTPUTS[11], &bridge);

    let assembly = base
        + personnel
        + material
        + gates
        + badge
        + barcode
        + flow
        + lockout
        + cart
        + quarantine
        + monitors
        + bridge;
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed clean-zone personnel/material flow interlock station:");
    println!("  Footprint:                {DECK_X:.0}mm x {DECK_Y:.0}mm validation deck");
    println!(
        "  Personnel cascade:        {PERSONNEL_ZONE_COUNT} zones at {:?} Pa, {PERSONNEL_PRESSURE_TAP_COUNT} pressure taps, {PERSONNEL_DOOR_COUNT} direction-controlled doors",
        PERSONNEL_SETPOINTS_PA
    );
    println!(
        "  Material pass-through:    {MATERIAL_INTERLOCK_PIN_COUNT} interlock pins, {MATERIAL_TOTE_NEST_COUNT} tote nests, {MATERIAL_PURGE_SLOT_COUNT} purge slots, {MATERIAL_WIPE_COUPON_COUNT} wipe coupons"
    );
    println!(
        "  Flow arbitration:         {ROUTE_GATE_COUNT} route gates, {CAPTIVE_KEY_COUNT} captive keys, {MUTUAL_EXCLUSION_BAR_COUNT} mutual-exclusion bars, {STATUS_FLAG_COUNT} status flags"
    );
    println!(
        "  Custody release checks:   {BADGE_DOCK_COUNT} badge docks, {MATERIAL_BARCODE_LAND_COUNT} material barcode lands, {SCALE_PAD_COUNT} scale pads, {TEMP_TAG_PUCK_COUNT} temperature-tag pucks"
    );
    println!(
        "  Directionality evidence:  {PERSONNEL_ARROW_COUNT} personnel arrows, {MATERIAL_ARROW_COUNT} material arrows, {DIRTY_RETURN_ARROW_COUNT} dirty-return arrows, {THRESHOLD_RIB_COUNT} threshold ribs"
    );
    println!(
        "  Drift/exception capture:  {QUARANTINE_BAY_COUNT} quarantine bays, {SENSOR_TREE_COUNT} sensor trees, {PARTICLE_COUNTER_PORT_COUNT} particle ports, {SETTLE_PLATE_HOLDER_COUNT} settle-plate holders"
    );
    println!(
        "  Evidence bridge:          {CAMERA_POD_COUNT} camera pods and {LIGHT_BAR_COUNT} light bars over the shared interlock evidence envelope"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn layout_rects() -> [Rect; 9] {
    [
        rect(
            "pressure_cascade_personnel_vestibule",
            PERSONNEL_POS,
            PERSONNEL_X,
            PERSONNEL_Y,
        ),
        rect(
            "material_pass_through_airlock",
            MATERIAL_POS,
            MATERIAL_X,
            MATERIAL_Y,
        ),
        rect("route_interlock_gate_array", GATE_POS, GATE_X, GATE_Y),
        rect(
            "personnel_badge_gown_release_panel",
            BADGE_POS,
            BADGE_X,
            BADGE_Y,
        ),
        rect(
            "material_barcode_weight_release_panel",
            BARCODE_POS,
            BARCODE_X,
            BARCODE_Y,
        ),
        rect(
            "clean_dirty_directional_flow_floor",
            FLOW_POS,
            FLOW_X,
            FLOW_Y,
        ),
        rect(
            "simultaneous_open_lockout_logic_wall",
            LOCKOUT_POS,
            LOCKOUT_X,
            LOCKOUT_Y,
        ),
        rect(
            "transfer_cart_docking_and_wheel_trap",
            CART_POS,
            CART_X,
            CART_Y,
        ),
        rect(
            "reject_quarantine_hold_bays",
            QUARANTINE_POS,
            QUARANTINE_X,
            QUARANTINE_Y,
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
            "{} exceeds usable clean-zone flow deck envelope",
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

    for pair in PERSONNEL_SETPOINTS_PA.windows(2) {
        assert!(
            pair[0] - pair[1] >= MIN_CASCADE_STEP_PA,
            "personnel flow must retain measurable positive-pressure steps"
        );
    }

    assert_eq!(MOUNT_COUNT, 12);
    assert_eq!(DATUM_COUNT, 4);
    assert_eq!(PERSONNEL_PRESSURE_TAP_COUNT, PERSONNEL_ZONE_COUNT * 2);
    assert_eq!(PERSONNEL_RELEASE_LANES, 3);
    assert_eq!(MATERIAL_RELEASE_LANES, 3);
    assert!(MATERIAL_DOOR_OPENING_X < MATERIAL_DOOR_FRAME_X);
    assert!(MATERIAL_DOOR_OPENING_Z < MATERIAL_DOOR_FRAME_Z);
    assert!(ROUTE_GATE_COUNT >= PERSONNEL_DOOR_COUNT + MATERIAL_TOTE_NEST_COUNT - 1);
    assert!(CAPTIVE_KEY_COUNT >= MATERIAL_INTERLOCK_PIN_COUNT);
    assert!(MATERIAL_BARCODE_LAND_COUNT >= MATERIAL_TOTE_NEST_COUNT + LOT_CARD_SLOT_COUNT);
    assert!(OVERHEAD_FILTER_SERVICE_Z > BRIDGE_UNDERSIDE_Z + BRIDGE_BEAM_Z);
}

fn base_flow_deck() -> Part {
    let deck = centered_cube(
        "clean_zone_flow_interlock_base_flow_deck",
        DECK_X,
        DECK_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let wipe_pan = centered_cube(
        "clean_zone_flow_interlock_wipeable_recessed_floor_pan",
        DECK_X - 144.0,
        DECK_Y - 132.0,
        7.0,
    )
    .translate(0.0, -4.0, DECK_Z - 3.5);
    let drain_gutter = centered_cube(
        "clean_zone_flow_interlock_low_point_drain_gutter",
        DECK_X - 270.0,
        13.0,
        8.0,
    )
    .translate(0.0, -258.0, DECK_Z - 4.0);

    deck - wipe_pan - drain_gutter - registration_sockets() - deck_mount_holes()
        + perimeter_rim()
        + flow_lane_dividers()
        + base_flow_arrows()
        + datum_fiducials()
        + clearance_gauges()
}

fn registration_sockets() -> Part {
    let mut sockets = Part::empty("clean_zone_flow_interlock_registration_sockets");
    for zone in layout_rects() {
        sockets = sockets
            + centered_cube(
                format!("clean_zone_flow_interlock_socket_{}", zone.name),
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
        (PERSONNEL_POS.0, PERSONNEL_POS.1),
        (MATERIAL_POS.0, MATERIAL_POS.1),
        (BADGE_POS.0, BADGE_POS.1),
        (BARCODE_POS.0, BARCODE_POS.1),
    ];
    assert_eq!(mount_points.len(), MOUNT_COUNT);

    let mut holes = Part::empty("clean_zone_flow_interlock_deck_mount_holes");
    for (i, (x, y)) in mount_points.into_iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("clean_zone_flow_interlock_m6_mount_clearance_{i}"),
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
        "clean_zone_flow_interlock_front_personnel_entry_stop_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -DECK_Y / 2.0 + RIM_W / 2.0, insert_z(RIM_Z));
    let rear = centered_cube(
        "clean_zone_flow_interlock_rear_material_service_stop_rim",
        DECK_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - RIM_W / 2.0, insert_z(RIM_Z));
    let left = centered_cube(
        "clean_zone_flow_interlock_left_gowning_service_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(-DECK_X / 2.0 + RIM_W / 2.0, 0.0, insert_z(RIM_Z));
    let right = centered_cube(
        "clean_zone_flow_interlock_right_material_bagout_service_rim",
        RIM_W,
        DECK_Y,
        RIM_Z,
    )
    .translate(DECK_X / 2.0 - RIM_W / 2.0, 0.0, insert_z(RIM_Z));

    front + rear + left + right
}

fn flow_lane_dividers() -> Part {
    let clean_boundary = centered_cube(
        "clean_zone_flow_interlock_clean_boundary_lane_divider",
        DECK_X - 190.0,
        12.0,
        30.0,
    )
    .translate(0.0, 24.0, DECK_Z + 15.0);
    let return_boundary = centered_cube(
        "clean_zone_flow_interlock_dirty_return_lane_divider",
        DECK_X - 230.0,
        10.0,
        26.0,
    )
    .translate(0.0, -256.0, DECK_Z + 13.0);
    let personnel_material_split = centered_cube(
        "clean_zone_flow_interlock_personnel_material_split_divider",
        10.0,
        735.0,
        30.0,
    )
    .translate(-230.0, -35.0, DECK_Z + 15.0);
    let material_exception_split = centered_cube(
        "clean_zone_flow_interlock_release_quarantine_split_divider",
        10.0,
        735.0,
        30.0,
    )
    .translate(300.0, -35.0, DECK_Z + 15.0);

    clean_boundary + return_boundary + personnel_material_split + material_exception_split
}

fn base_flow_arrows() -> Part {
    let mut arrows = Part::empty("clean_zone_flow_interlock_base_direction_arrows");
    for (i, (x, y, rotation)) in [
        (-600.0, 335.0, 0.0),
        (-490.0, 250.0, 0.0),
        (-380.0, 165.0, 0.0),
        (-90.0, 335.0, 0.0),
        (20.0, 250.0, 0.0),
        (130.0, 165.0, 0.0),
        (420.0, 335.0, 0.0),
        (520.0, 250.0, 0.0),
        (620.0, 165.0, 0.0),
        (510.0, -95.0, 180.0),
        (510.0, -170.0, 180.0),
        (510.0, -345.0, 180.0),
    ]
    .into_iter()
    .enumerate()
    {
        arrows = arrows
            + flow_arrow(format!("clean_zone_base_flow_arrow_{i}"), 78.0)
                .rotate(0.0, 0.0, rotation)
                .translate(x, y, DECK_Z + 7.0);
    }
    arrows
}

fn datum_fiducials() -> Part {
    let positions = [
        (-DECK_X / 2.0 + 112.0, -DECK_Y / 2.0 + 106.0),
        (DECK_X / 2.0 - 112.0, -DECK_Y / 2.0 + 106.0),
        (-DECK_X / 2.0 + 112.0, DECK_Y / 2.0 - 106.0),
        (DECK_X / 2.0 - 112.0, DECK_Y / 2.0 - 106.0),
    ];
    assert_eq!(positions.len(), DATUM_COUNT);

    let mut fiducials = Part::empty("clean_zone_flow_interlock_robot_datum_fiducials");
    for (i, (x, y)) in positions.into_iter().enumerate() {
        fiducials = fiducials
            + fiducial_target(&format!("clean_zone_flow_interlock_datum_{i}")).translate(
                x,
                y,
                DECK_Z + 2.0,
            );
    }
    fiducials
}

fn clearance_gauges() -> Part {
    let front = centered_cube(
        "clean_zone_flow_interlock_front_personnel_clearance_gauge",
        DECK_X - 160.0,
        8.0,
        8.0,
    )
    .translate(
        0.0,
        -DECK_Y / 2.0 + FRONT_PERSONNEL_CLEARANCE_Y,
        DECK_Z + 4.0,
    );
    let rear = centered_cube(
        "clean_zone_flow_interlock_rear_pass_through_service_clearance_gauge",
        DECK_X - 160.0,
        8.0,
        8.0,
    )
    .translate(
        0.0,
        DECK_Y / 2.0 - REAR_PASS_THROUGH_SERVICE_Y,
        DECK_Z + 4.0,
    );
    let left = centered_cube(
        "clean_zone_flow_interlock_left_gowning_service_clearance_gauge",
        8.0,
        DECK_Y - 170.0,
        8.0,
    )
    .translate(-DECK_X / 2.0 + LEFT_GOWNING_SERVICE_X, 0.0, DECK_Z + 4.0);
    let right = centered_cube(
        "clean_zone_flow_interlock_right_material_bagout_clearance_gauge",
        8.0,
        DECK_Y - 170.0,
        8.0,
    )
    .translate(DECK_X / 2.0 - RIGHT_MATERIAL_BAGOUT_X, 0.0, DECK_Z + 4.0);
    let overhead = centered_cube(
        "clean_zone_flow_interlock_overhead_filter_service_clearance_gauge",
        DECK_X - 260.0,
        DECK_Y - 260.0,
        6.0,
    )
    .translate(0.0, 0.0, OVERHEAD_FILTER_SERVICE_Z);

    front + rear + left + right + overhead
}

fn pressure_cascade_personnel_vestibule() -> Part {
    let floor = centered_cube(
        "clean_zone_personnel_cascade_vestibule_floor_plate",
        PERSONNEL_X - 34.0,
        PERSONNEL_Y - 34.0,
        PERSONNEL_Z,
    )
    .translate(PERSONNEL_POS.0, PERSONNEL_POS.1, insert_z(PERSONNEL_Z));

    floor
        + personnel_pressure_bays()
        + personnel_vestibule_walls()
        + personnel_pressure_taps()
        + personnel_direction_doors()
        + personnel_dwell_tokens()
}

fn personnel_pressure_bays() -> Part {
    let mut bays = Part::empty("clean_zone_personnel_pressure_cascade_bays");
    for zone in 0..PERSONNEL_ZONE_COUNT {
        let x = PERSONNEL_POS.0 + centered_index(zone, PERSONNEL_ZONE_COUNT, PERSONNEL_BAY_PITCH_X);
        let plate = centered_cube(
            format!("clean_zone_personnel_cascade_zone_{zone}_floor_land"),
            PERSONNEL_BAY_X,
            PERSONNEL_BAY_Y,
            12.0,
        )
        .translate(x, PERSONNEL_POS.1, DECK_Z + PERSONNEL_Z + 6.0);
        let divider = centered_cube(
            format!("clean_zone_personnel_cascade_zone_{zone}_raised_boundary"),
            8.0,
            PERSONNEL_BAY_Y + 20.0,
            68.0,
        )
        .translate(
            x - PERSONNEL_BAY_X / 2.0 - 7.0,
            PERSONNEL_POS.1,
            DECK_Z + PERSONNEL_Z + 34.0,
        );
        let label = centered_cube(
            format!("clean_zone_personnel_cascade_zone_{zone}_setpoint_land"),
            58.0,
            18.0,
            5.0,
        )
        .translate(
            x,
            PERSONNEL_POS.1 - PERSONNEL_BAY_Y / 2.0 + 24.0,
            DECK_Z + PERSONNEL_Z + 14.5,
        );
        bays = bays + plate + divider + label + personnel_diffuser_slots(zone, x);
    }
    bays
}

fn personnel_diffuser_slots(zone: usize, zone_x: f64) -> Part {
    let mut slots = Part::empty(format!(
        "clean_zone_personnel_cascade_zone_{zone}_diffuser_slots"
    ));
    for slot in 0..PERSONNEL_DIFFUSER_SLOTS_PER_ZONE {
        slots = slots
            + centered_cube(
                format!("clean_zone_personnel_cascade_zone_{zone}_diffuser_slot_{slot}"),
                PERSONNEL_BAY_X - 24.0,
                7.0,
                8.0,
            )
            .translate(
                zone_x,
                PERSONNEL_POS.1 + centered_index(slot, PERSONNEL_DIFFUSER_SLOTS_PER_ZONE, 38.0),
                DECK_Z + PERSONNEL_Z + 21.0,
            );
    }
    slots
}

fn personnel_vestibule_walls() -> Part {
    let rear = centered_cube(
        "clean_zone_personnel_vestibule_clean_side_wall",
        PERSONNEL_X - 58.0,
        14.0,
        98.0,
    )
    .translate(
        PERSONNEL_POS.0,
        PERSONNEL_POS.1 + PERSONNEL_Y / 2.0 - 36.0,
        DECK_Z + PERSONNEL_Z + 49.0,
    );
    let front = centered_cube(
        "clean_zone_personnel_vestibule_room_side_wall",
        PERSONNEL_X - 58.0,
        14.0,
        70.0,
    )
    .translate(
        PERSONNEL_POS.0,
        PERSONNEL_POS.1 - PERSONNEL_Y / 2.0 + 36.0,
        DECK_Z + PERSONNEL_Z + 35.0,
    );
    let left = centered_cube(
        "clean_zone_personnel_vestibule_entry_boundary_wall",
        14.0,
        PERSONNEL_Y - 70.0,
        84.0,
    )
    .translate(
        PERSONNEL_POS.0 - PERSONNEL_X / 2.0 + 36.0,
        PERSONNEL_POS.1,
        DECK_Z + PERSONNEL_Z + 42.0,
    );
    let right = centered_cube(
        "clean_zone_personnel_vestibule_exit_boundary_wall",
        14.0,
        PERSONNEL_Y - 70.0,
        84.0,
    )
    .translate(
        PERSONNEL_POS.0 + PERSONNEL_X / 2.0 - 36.0,
        PERSONNEL_POS.1,
        DECK_Z + PERSONNEL_Z + 42.0,
    );

    rear + front + left + right
}

fn personnel_pressure_taps() -> Part {
    let mut taps = Part::empty("clean_zone_personnel_cascade_pressure_taps");
    for zone in 0..PERSONNEL_ZONE_COUNT {
        let x = PERSONNEL_POS.0 + centered_index(zone, PERSONNEL_ZONE_COUNT, PERSONNEL_BAY_PITCH_X);
        for side in 0..2 {
            let y = PERSONNEL_POS.1 + if side == 0 { -112.0 } else { 112.0 };
            let boss = centered_cylinder(
                format!("clean_zone_personnel_zone_{zone}_dp_tap_boss_{side}"),
                9.0,
                18.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, y, DECK_Z + PERSONNEL_Z + 64.0);
            let bore = centered_cylinder(
                format!("clean_zone_personnel_zone_{zone}_dp_tap_bore_{side}"),
                2.2,
                22.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, y, DECK_Z + PERSONNEL_Z + 64.0);
            taps = taps + (boss - bore);
        }
    }
    taps
}

fn personnel_direction_doors() -> Part {
    let mut doors = Part::empty("clean_zone_personnel_direction_interlock_doors");
    for i in 0..PERSONNEL_DOOR_COUNT {
        let x = PERSONNEL_POS.0 + centered_index(i, PERSONNEL_DOOR_COUNT, 132.0);
        let frame = rectangular_frame_xz(
            &format!("clean_zone_personnel_direction_door_{i}"),
            78.0,
            12.0,
            120.0,
            44.0,
            72.0,
        )
        .translate(x, PERSONNEL_POS.1, DECK_Z + PERSONNEL_Z + 76.0);
        let ratchet = centered_cube(
            format!("clean_zone_personnel_door_{i}_one_way_ratchet_land"),
            24.0,
            18.0,
            20.0,
        )
        .translate(
            x + 54.0,
            PERSONNEL_POS.1 - 100.0,
            DECK_Z + PERSONNEL_Z + 38.0,
        );
        let reverse_flag = centered_cube(
            format!("clean_zone_personnel_door_{i}_reverse_attempt_witness_flag"),
            18.0,
            12.0,
            44.0,
        )
        .translate(
            x - 54.0,
            PERSONNEL_POS.1 + 100.0,
            DECK_Z + PERSONNEL_Z + 58.0,
        );
        doors = doors + frame + ratchet + reverse_flag;
    }
    doors
}

fn personnel_dwell_tokens() -> Part {
    let base = centered_cube(
        "clean_zone_personnel_reentry_dwell_timer_slot",
        190.0,
        28.0,
        26.0,
    )
    .translate(
        PERSONNEL_POS.0,
        PERSONNEL_POS.1 - PERSONNEL_Y / 2.0 + 60.0,
        DECK_Z + PERSONNEL_Z + 13.0,
    );
    let mut tokens = Part::empty("clean_zone_personnel_dwell_timer_tokens");
    for i in 0..PERSONNEL_DWELL_TOKEN_COUNT {
        tokens = tokens
            + centered_cylinder(
                format!("clean_zone_personnel_dwell_token_{i}"),
                12.0,
                7.0,
                28,
            )
            .translate(
                PERSONNEL_POS.0 + centered_index(i, PERSONNEL_DWELL_TOKEN_COUNT, 42.0),
                PERSONNEL_POS.1 - PERSONNEL_Y / 2.0 + 60.0,
                DECK_Z + PERSONNEL_Z + 29.5,
            );
    }
    base + tokens
}

fn material_pass_through_airlock() -> Part {
    let saddle = centered_cube(
        "clean_zone_material_pass_through_airlock_saddle",
        MATERIAL_X - 34.0,
        MATERIAL_Y - 34.0,
        MATERIAL_Z,
    )
    .translate(MATERIAL_POS.0, MATERIAL_POS.1, insert_z(MATERIAL_Z));

    saddle
        + material_door_frame("outer_room_side", -MATERIAL_DOOR_PAIR_SPACING_Y / 2.0)
        + material_door_frame("inner_clean_side", MATERIAL_DOOR_PAIR_SPACING_Y / 2.0)
        + material_interlock_pins()
        + material_latch_witnesses()
        + material_tote_nests()
        + material_purge_slots()
        + material_wipe_coupons()
}

fn material_door_frame(label: &str, y_offset: f64) -> Part {
    let frame = rectangular_frame_xz(
        &format!("clean_zone_material_{label}_door_gasket_frame"),
        MATERIAL_DOOR_FRAME_X,
        MATERIAL_DOOR_FACE_Y,
        MATERIAL_DOOR_FRAME_Z,
        MATERIAL_DOOR_OPENING_X,
        MATERIAL_DOOR_OPENING_Z,
    )
    .translate(
        MATERIAL_POS.0,
        MATERIAL_POS.1 + y_offset,
        DECK_Z + MATERIAL_Z + MATERIAL_DOOR_FRAME_Z / 2.0,
    );
    let window = centered_cube(
        format!("clean_zone_material_{label}_view_window_witness_land"),
        MATERIAL_DOOR_OPENING_X - 34.0,
        5.0,
        22.0,
    )
    .translate(
        MATERIAL_POS.0,
        MATERIAL_POS.1 + y_offset,
        DECK_Z + MATERIAL_Z + MATERIAL_DOOR_FRAME_Z / 2.0,
    );
    let handle = centered_cube(
        format!("clean_zone_material_{label}_sealed_handle_land"),
        18.0,
        16.0,
        52.0,
    )
    .translate(
        MATERIAL_POS.0 + MATERIAL_DOOR_FRAME_X / 2.0 - 34.0,
        MATERIAL_POS.1 + y_offset,
        DECK_Z + MATERIAL_Z + 112.0,
    );

    frame + window + handle
}

fn material_interlock_pins() -> Part {
    let mut pins = Part::empty("clean_zone_material_pass_through_interlock_pins");
    for i in 0..MATERIAL_INTERLOCK_PIN_COUNT {
        let x = MATERIAL_POS.0 + centered_index(i, MATERIAL_INTERLOCK_PIN_COUNT, 32.0);
        let pin = centered_cylinder(
            format!("clean_zone_material_captive_interlock_pin_{i}"),
            5.0,
            MATERIAL_DOOR_PAIR_SPACING_Y + 82.0,
            22,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            x,
            MATERIAL_POS.1,
            DECK_Z + MATERIAL_Z + MATERIAL_DOOR_FRAME_Z + 18.0,
        );
        let witness = centered_cube(
            format!("clean_zone_material_interlock_pin_witness_land_{i}"),
            22.0,
            16.0,
            16.0,
        )
        .translate(
            x,
            MATERIAL_POS.1 - MATERIAL_DOOR_PAIR_SPACING_Y / 2.0 - 34.0,
            DECK_Z + MATERIAL_Z + MATERIAL_DOOR_FRAME_Z + 18.0,
        );
        pins = pins + pin + witness;
    }
    pins
}

fn material_latch_witnesses() -> Part {
    let mut latches = Part::empty("clean_zone_material_latch_witness_blocks");
    for i in 0..MATERIAL_LATCH_COUNT {
        let door_side = if i < MATERIAL_LATCH_COUNT / 2 {
            -MATERIAL_DOOR_PAIR_SPACING_Y / 2.0
        } else {
            MATERIAL_DOOR_PAIR_SPACING_Y / 2.0
        };
        let side = if i % 2 == 0 { -1.0 } else { 1.0 };
        let z = DECK_Z + MATERIAL_Z + 72.0 + ((i / 2) % 2) as f64 * 70.0;
        latches = latches
            + centered_cube(
                format!("clean_zone_material_latch_witness_block_{i}"),
                26.0,
                18.0,
                28.0,
            )
            .translate(
                MATERIAL_POS.0 + side * (MATERIAL_DOOR_FRAME_X / 2.0 + 30.0),
                MATERIAL_POS.1 + door_side,
                z,
            );
    }
    latches
}

fn material_tote_nests() -> Part {
    let mut nests = Part::empty("clean_zone_material_transfer_tote_nests");
    for i in 0..MATERIAL_TOTE_NEST_COUNT {
        let x = MATERIAL_POS.0 + centered_index(i, MATERIAL_TOTE_NEST_COUNT, 86.0);
        let pocket = centered_cube(
            format!("clean_zone_material_tote_registration_pocket_{i}"),
            64.0,
            76.0,
            16.0,
        )
        .translate(x, MATERIAL_POS.1, DECK_Z + MATERIAL_Z + 8.0);
        let fence = rectangular_frame_xy(
            &format!("clean_zone_material_tote_retention_fence_{i}"),
            82.0,
            94.0,
            14.0,
            64.0,
            76.0,
        )
        .translate(x, MATERIAL_POS.1, DECK_Z + MATERIAL_Z + 23.0);
        nests = nests + pocket + fence;
    }
    nests
}

fn material_purge_slots() -> Part {
    let plenum = centered_cube(
        "clean_zone_material_pass_through_purge_plenum",
        MATERIAL_X - 84.0,
        44.0,
        54.0,
    )
    .translate(
        MATERIAL_POS.0,
        MATERIAL_POS.1 + MATERIAL_Y / 2.0 - 52.0,
        DECK_Z + MATERIAL_Z + 27.0,
    );
    let mut slots = Part::empty("clean_zone_material_pass_through_purge_slots");
    for i in 0..MATERIAL_PURGE_SLOT_COUNT {
        slots = slots
            + centered_cube(
                format!("clean_zone_material_purge_slot_{i}"),
                28.0,
                8.0,
                8.0,
            )
            .translate(
                MATERIAL_POS.0 + centered_index(i, MATERIAL_PURGE_SLOT_COUNT, 36.0),
                MATERIAL_POS.1 + MATERIAL_Y / 2.0 - 52.0,
                DECK_Z + MATERIAL_Z + 58.0,
            );
    }
    plenum + slots
}

fn material_wipe_coupons() -> Part {
    let mut coupons = Part::empty("clean_zone_material_wipe_residue_coupon_lands");
    for i in 0..MATERIAL_WIPE_COUPON_COUNT {
        coupons = coupons
            + centered_cube(
                format!("clean_zone_material_wipe_coupon_land_{i}"),
                38.0,
                24.0,
                5.0,
            )
            .translate(
                MATERIAL_POS.0 + centered_index(i, MATERIAL_WIPE_COUPON_COUNT, 48.0),
                MATERIAL_POS.1 - MATERIAL_Y / 2.0 + 54.0,
                DECK_Z + MATERIAL_Z + 38.5,
            );
    }
    coupons
}

fn route_interlock_gate_array() -> Part {
    let base = centered_cube(
        "clean_zone_route_interlock_gate_array_base",
        GATE_X - 32.0,
        GATE_Y - 34.0,
        GATE_Z,
    )
    .translate(GATE_POS.0, GATE_POS.1, insert_z(GATE_Z));

    base + route_gates()
        + captive_key_bank()
        + mutual_exclusion_bars()
        + status_flags()
        + route_state_tokens()
}

fn route_gates() -> Part {
    let mut gates = Part::empty("clean_zone_route_interlock_gate_blades");
    for i in 0..ROUTE_GATE_COUNT {
        let y = GATE_POS.1 + centered_index(i, ROUTE_GATE_COUNT, 42.0);
        let blade = centered_cube(format!("clean_zone_route_gate_blade_{i}"), 170.0, 8.0, 58.0)
            .translate(GATE_POS.0 - 42.0, y, DECK_Z + GATE_Z + 29.0);
        let hinge = centered_cylinder(
            format!("clean_zone_route_gate_hinge_bar_{i}"),
            5.0,
            74.0,
            20,
        )
        .translate(GATE_POS.0 - 132.0, y, DECK_Z + GATE_Z + 29.0);
        gates = gates + blade + hinge;
    }
    gates
}

fn captive_key_bank() -> Part {
    let rail = centered_cube("clean_zone_route_captive_key_bank_rail", 236.0, 28.0, 24.0)
        .translate(
            GATE_POS.0,
            GATE_POS.1 - GATE_Y / 2.0 + 50.0,
            DECK_Z + GATE_Z + 12.0,
        );
    let mut keys = Part::empty("clean_zone_route_captive_key_sockets");
    for i in 0..CAPTIVE_KEY_COUNT {
        keys = keys
            + centered_cylinder(
                format!("clean_zone_route_captive_key_socket_{i}"),
                10.0,
                8.0,
                28,
            )
            .translate(
                GATE_POS.0 + centered_index(i, CAPTIVE_KEY_COUNT, 28.0),
                GATE_POS.1 - GATE_Y / 2.0 + 50.0,
                DECK_Z + GATE_Z + 28.0,
            );
    }
    rail + keys
}

fn mutual_exclusion_bars() -> Part {
    let mut bars = Part::empty("clean_zone_mutual_exclusion_logic_bars");
    for i in 0..MUTUAL_EXCLUSION_BAR_COUNT {
        bars = bars
            + centered_cube(
                format!("clean_zone_mutual_exclusion_slide_bar_{i}"),
                232.0,
                10.0,
                16.0,
            )
            .translate(
                GATE_POS.0,
                GATE_POS.1 + 18.0 + i as f64 * 34.0,
                DECK_Z + GATE_Z + 78.0 + i as f64 * 12.0,
            );
    }
    bars
}

fn status_flags() -> Part {
    let mut flags = Part::empty("clean_zone_route_status_flags");
    for i in 0..STATUS_FLAG_COUNT {
        flags = flags
            + centered_cube(format!("clean_zone_route_status_flag_{i}"), 18.0, 9.0, 46.0)
                .translate(
                    GATE_POS.0 + GATE_X / 2.0 - 52.0,
                    GATE_POS.1 + centered_index(i, STATUS_FLAG_COUNT, 38.0),
                    DECK_Z + GATE_Z + 60.0,
                );
    }
    flags
}

fn route_state_tokens() -> Part {
    let mut tokens = Part::empty("clean_zone_route_state_token_lands");
    for i in 0..ROUTE_STATE_COUNT {
        tokens = tokens
            + centered_cube(
                format!("clean_zone_route_state_token_land_{i}"),
                48.0,
                22.0,
                6.0,
            )
            .translate(
                GATE_POS.0 + centered_index(i, ROUTE_STATE_COUNT, 58.0),
                GATE_POS.1 + GATE_Y / 2.0 - 48.0,
                DECK_Z + GATE_Z + 37.0,
            );
    }
    tokens
}

fn personnel_badge_gown_release_panel() -> Part {
    let panel = centered_cube(
        "clean_zone_personnel_badge_gown_release_panel",
        BADGE_X - 32.0,
        BADGE_Y - 32.0,
        BADGE_Z,
    )
    .translate(BADGE_POS.0, BADGE_POS.1, insert_z(BADGE_Z));

    panel - badge_dock_recesses() - gown_canister_recesses() - glove_check_recesses()
        + badge_rfid_pucks()
        + personnel_release_lanes()
        + hand_sanitizer_ports()
}

fn badge_dock_recesses() -> Part {
    let mut docks = Part::empty("clean_zone_personnel_badge_dock_recesses");
    for i in 0..BADGE_DOCK_COUNT {
        docks = docks
            + centered_cube(
                format!("clean_zone_personnel_badge_dock_recess_{i}"),
                42.0,
                28.0,
                BADGE_Z + 5.0,
            )
            .translate(
                BADGE_POS.0 + centered_index(i, BADGE_DOCK_COUNT, 54.0),
                BADGE_POS.1 + 66.0,
                DECK_Z + BADGE_Z / 2.0,
            );
    }
    docks
}

fn gown_canister_recesses() -> Part {
    let mut recesses = Part::empty("clean_zone_personnel_gown_release_canister_recesses");
    for i in 0..GOWN_RELEASE_CANISTER_COUNT {
        recesses = recesses
            + centered_cylinder(
                format!("clean_zone_personnel_gown_canister_socket_{i}"),
                22.0,
                BADGE_Z + 5.0,
                36,
            )
            .translate(
                BADGE_POS.0 + centered_index(i, GOWN_RELEASE_CANISTER_COUNT, 62.0),
                BADGE_POS.1 + 4.0,
                DECK_Z + BADGE_Z / 2.0,
            );
    }
    recesses
}

fn glove_check_recesses() -> Part {
    let mut recesses = Part::empty("clean_zone_personnel_glove_check_recesses");
    for i in 0..GLOVE_CHECK_COUNT {
        recesses = recesses
            + centered_cube(
                format!("clean_zone_personnel_glove_check_recess_{i}"),
                48.0,
                28.0,
                BADGE_Z + 5.0,
            )
            .translate(
                BADGE_POS.0 + centered_index(i, GLOVE_CHECK_COUNT, 62.0),
                BADGE_POS.1 - 58.0,
                DECK_Z + BADGE_Z / 2.0,
            );
    }
    recesses
}

fn badge_rfid_pucks() -> Part {
    let mut pucks = Part::empty("clean_zone_personnel_badge_rfid_pucks");
    for i in 0..BADGE_DOCK_COUNT {
        pucks = pucks
            + centered_cylinder(
                format!("clean_zone_personnel_badge_rfid_puck_{i}"),
                12.0,
                5.0,
                28,
            )
            .translate(
                BADGE_POS.0 + centered_index(i, BADGE_DOCK_COUNT, 54.0),
                BADGE_POS.1 + 66.0,
                DECK_Z + BADGE_Z + 2.5,
            );
    }
    pucks
}

fn personnel_release_lanes() -> Part {
    let mut lanes = Part::empty("clean_zone_personnel_release_hold_reject_lanes");
    for i in 0..PERSONNEL_RELEASE_LANES {
        lanes = lanes
            + centered_cube(
                format!("clean_zone_personnel_release_lane_{i}"),
                96.0,
                24.0,
                6.0,
            )
            .translate(
                BADGE_POS.0 + centered_index(i, PERSONNEL_RELEASE_LANES, 112.0),
                BADGE_POS.1 - 94.0,
                DECK_Z + BADGE_Z + 3.0,
            );
    }
    lanes
}

fn hand_sanitizer_ports() -> Part {
    let mut ports = Part::empty("clean_zone_personnel_hand_sanitizer_ports");
    for i in 0..HAND_SANITIZER_PORT_COUNT {
        ports = ports
            + centered_cylinder(
                format!("clean_zone_personnel_hand_sanitizer_port_{i}"),
                16.0,
                18.0,
                32,
            )
            .translate(
                BADGE_POS.0 + centered_index(i, HAND_SANITIZER_PORT_COUNT, 248.0),
                BADGE_POS.1 - 16.0,
                DECK_Z + BADGE_Z + 9.0,
            );
    }
    ports
}

fn material_barcode_weight_release_panel() -> Part {
    let panel = centered_cube(
        "clean_zone_material_barcode_weight_release_panel",
        BARCODE_X - 32.0,
        BARCODE_Y - 32.0,
        BARCODE_Z,
    )
    .translate(BARCODE_POS.0, BARCODE_POS.1, insert_z(BARCODE_Z));

    panel - scale_pad_recesses()
        + material_barcode_lands()
        + temp_tag_pucks()
        + lot_card_slots()
        + material_release_lanes()
}

fn scale_pad_recesses() -> Part {
    let mut pads = Part::empty("clean_zone_material_scale_pad_recesses");
    for i in 0..SCALE_PAD_COUNT {
        pads = pads
            + centered_cube(
                format!("clean_zone_material_scale_pad_recess_{i}"),
                62.0,
                48.0,
                BARCODE_Z + 5.0,
            )
            .translate(
                BARCODE_POS.0 + centered_index(i, SCALE_PAD_COUNT, 78.0),
                BARCODE_POS.1 + 48.0,
                DECK_Z + BARCODE_Z / 2.0,
            );
    }
    pads
}

fn material_barcode_lands() -> Part {
    let mut lands = Part::empty("clean_zone_material_barcode_scan_lands");
    for i in 0..MATERIAL_BARCODE_LAND_COUNT {
        lands = lands
            + centered_cube(
                format!("clean_zone_material_barcode_land_{i}"),
                42.0,
                18.0,
                5.0,
            )
            .translate(
                BARCODE_POS.0 + centered_index(i % 6, 6, 56.0),
                BARCODE_POS.1 - 8.0 - (i / 6) as f64 * 30.0,
                DECK_Z + BARCODE_Z + 2.5,
            );
    }
    lands
}

fn temp_tag_pucks() -> Part {
    let mut pucks = Part::empty("clean_zone_material_temperature_tag_pucks");
    for i in 0..TEMP_TAG_PUCK_COUNT {
        pucks = pucks
            + centered_cylinder(
                format!("clean_zone_material_temp_tag_puck_{i}"),
                11.0,
                5.0,
                28,
            )
            .translate(
                BARCODE_POS.0 + centered_index(i, TEMP_TAG_PUCK_COUNT, 44.0),
                BARCODE_POS.1 + 90.0,
                DECK_Z + BARCODE_Z + 2.5,
            );
    }
    pucks
}

fn lot_card_slots() -> Part {
    let mut slots = Part::empty("clean_zone_material_lot_card_slots");
    for i in 0..LOT_CARD_SLOT_COUNT {
        slots = slots
            + centered_cube(
                format!("clean_zone_material_lot_card_slot_{i}"),
                54.0,
                12.0,
                26.0,
            )
            .translate(
                BARCODE_POS.0 + centered_index(i, LOT_CARD_SLOT_COUNT, 72.0),
                BARCODE_POS.1 - 78.0,
                DECK_Z + BARCODE_Z + 13.0,
            );
    }
    slots
}

fn material_release_lanes() -> Part {
    let mut lanes = Part::empty("clean_zone_material_release_hold_reject_lanes");
    for i in 0..MATERIAL_RELEASE_LANES {
        lanes = lanes
            + centered_cube(
                format!("clean_zone_material_release_lane_{i}"),
                102.0,
                24.0,
                6.0,
            )
            .translate(
                BARCODE_POS.0 + centered_index(i, MATERIAL_RELEASE_LANES, 116.0),
                BARCODE_POS.1 - 104.0,
                DECK_Z + BARCODE_Z + 3.0,
            );
    }
    lanes
}

fn clean_dirty_directional_flow_floor() -> Part {
    let panel = centered_cube(
        "clean_zone_directional_flow_floor_panel",
        FLOW_X - 32.0,
        FLOW_Y - 32.0,
        FLOW_Z,
    )
    .translate(FLOW_POS.0, FLOW_POS.1, insert_z(FLOW_Z));

    panel
        + personnel_direction_arrows()
        + material_direction_arrows()
        + dirty_return_arrows()
        + threshold_ribs()
        + anti_backtrack_palettes()
}

fn personnel_direction_arrows() -> Part {
    let mut arrows = Part::empty("clean_zone_personnel_direction_arrows");
    for i in 0..PERSONNEL_ARROW_COUNT {
        arrows = arrows
            + flow_arrow(format!("clean_zone_personnel_direction_arrow_{i}"), 64.0).translate(
                FLOW_POS.0 - 96.0,
                FLOW_POS.1 + centered_index(i, PERSONNEL_ARROW_COUNT, 34.0),
                DECK_Z + FLOW_Z + 5.0,
            );
    }
    arrows
}

fn material_direction_arrows() -> Part {
    let mut arrows = Part::empty("clean_zone_material_direction_arrows");
    for i in 0..MATERIAL_ARROW_COUNT {
        arrows = arrows
            + flow_arrow(format!("clean_zone_material_direction_arrow_{i}"), 64.0).translate(
                FLOW_POS.0,
                FLOW_POS.1 + centered_index(i, MATERIAL_ARROW_COUNT, 34.0),
                DECK_Z + FLOW_Z + 5.0,
            );
    }
    arrows
}

fn dirty_return_arrows() -> Part {
    let mut arrows = Part::empty("clean_zone_dirty_return_direction_arrows");
    for i in 0..DIRTY_RETURN_ARROW_COUNT {
        arrows = arrows
            + flow_arrow(format!("clean_zone_dirty_return_arrow_{i}"), 58.0)
                .rotate(0.0, 0.0, 180.0)
                .translate(
                    FLOW_POS.0 + 96.0,
                    FLOW_POS.1 + centered_index(i, DIRTY_RETURN_ARROW_COUNT, 42.0),
                    DECK_Z + FLOW_Z + 5.0,
                );
    }
    arrows
}

fn threshold_ribs() -> Part {
    let mut ribs = Part::empty("clean_zone_flow_direction_threshold_ribs");
    for i in 0..THRESHOLD_RIB_COUNT {
        ribs = ribs
            + centered_cube(
                format!("clean_zone_flow_threshold_rib_{i}"),
                250.0,
                8.0,
                12.0,
            )
            .translate(
                FLOW_POS.0,
                FLOW_POS.1 - FLOW_Y / 2.0 + 34.0 + i as f64 * 28.0,
                DECK_Z + FLOW_Z + 6.0,
            );
    }
    ribs
}

fn anti_backtrack_palettes() -> Part {
    let mut palettes = Part::empty("clean_zone_anti_backtrack_palette_lands");
    for i in 0..ANTI_BACKTRACK_PALETTE_COUNT {
        palettes = palettes
            + centered_cube(
                format!("clean_zone_anti_backtrack_palette_land_{i}"),
                48.0,
                30.0,
                5.0,
            )
            .translate(
                FLOW_POS.0 + centered_index(i, ANTI_BACKTRACK_PALETTE_COUNT, 66.0),
                FLOW_POS.1 + FLOW_Y / 2.0 - 44.0,
                DECK_Z + FLOW_Z + 2.5,
            );
    }
    palettes
}

fn simultaneous_open_lockout_logic_wall() -> Part {
    let wall = centered_cube(
        "clean_zone_simultaneous_open_lockout_logic_wall",
        LOCKOUT_X - 36.0,
        18.0,
        LOCKOUT_WALL_Z,
    )
    .translate(LOCKOUT_POS.0, LOCKOUT_POS.1, DECK_Z + LOCKOUT_WALL_Z / 2.0);

    wall + logic_cam_bars()
        + simultaneous_open_sentinels()
        + fault_injection_cards()
        + e_stop_covers()
}

fn logic_cam_bars() -> Part {
    let mut bars = Part::empty("clean_zone_lockout_logic_cam_bars");
    for i in 0..LOGIC_CAM_BAR_COUNT {
        bars = bars
            + centered_cube(
                format!("clean_zone_lockout_logic_cam_bar_{i}"),
                290.0,
                14.0,
                12.0,
            )
            .translate(
                LOCKOUT_POS.0,
                LOCKOUT_POS.1 - 16.0,
                DECK_Z + 34.0 + i as f64 * 26.0,
            );
    }
    bars
}

fn simultaneous_open_sentinels() -> Part {
    let mut sentinels = Part::empty("clean_zone_simultaneous_open_sentinel_flags");
    for i in 0..SIMULTANEOUS_OPEN_SENTINEL_COUNT {
        sentinels = sentinels
            + centered_cube(
                format!("clean_zone_simultaneous_open_sentinel_{i}"),
                18.0,
                14.0,
                54.0,
            )
            .translate(
                LOCKOUT_POS.0 + centered_index(i, SIMULTANEOUS_OPEN_SENTINEL_COUNT, 70.0),
                LOCKOUT_POS.1 + 18.0,
                DECK_Z + LOCKOUT_WALL_Z - 20.0,
            );
    }
    sentinels
}

fn fault_injection_cards() -> Part {
    let mut cards = Part::empty("clean_zone_lockout_fault_injection_card_slots");
    for i in 0..FAULT_INJECTION_CARD_COUNT {
        cards = cards
            + centered_cube(
                format!("clean_zone_lockout_fault_card_slot_{i}"),
                42.0,
                12.0,
                30.0,
            )
            .translate(
                LOCKOUT_POS.0 + centered_index(i, FAULT_INJECTION_CARD_COUNT, 56.0),
                LOCKOUT_POS.1 + 26.0,
                DECK_Z + 70.0,
            );
    }
    cards
}

fn e_stop_covers() -> Part {
    let mut covers = Part::empty("clean_zone_lockout_e_stop_covers");
    for i in 0..E_STOP_COVER_COUNT {
        let x = LOCKOUT_POS.0 + centered_index(i, E_STOP_COVER_COUNT, 282.0);
        let guard = centered_cylinder(
            format!("clean_zone_lockout_e_stop_guard_ring_{i}"),
            28.0,
            10.0,
            40,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, LOCKOUT_POS.1 + 30.0, DECK_Z + 142.0);
        let button = centered_cylinder(
            format!("clean_zone_lockout_e_stop_button_land_{i}"),
            18.0,
            8.0,
            36,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, LOCKOUT_POS.1 + 36.0, DECK_Z + 142.0);
        covers = covers + guard + button;
    }
    covers
}

fn transfer_cart_docking_and_wheel_trap() -> Part {
    let deck = centered_cube(
        "clean_zone_transfer_cart_docking_deck",
        CART_X - 32.0,
        CART_Y - 32.0,
        CART_Z,
    )
    .translate(CART_POS.0, CART_POS.1, insert_z(CART_Z));

    deck - cart_wheel_traps() + cart_dock_cones() + cart_guide_rails() + cart_route_tokens()
}

fn cart_wheel_traps() -> Part {
    let mut traps = Part::empty("clean_zone_transfer_cart_wheel_traps");
    for i in 0..CART_WHEEL_TRAP_COUNT {
        let x = CART_POS.0 + centered_index(i % 2, 2, 250.0);
        let y = CART_POS.1 + centered_index(i / 2, 2, 76.0);
        traps = traps
            + centered_cylinder(
                format!("clean_zone_transfer_cart_wheel_trap_{i}"),
                24.0,
                CART_Z + 5.0,
                34,
            )
            .translate(x, y, DECK_Z + CART_Z / 2.0);
    }
    traps
}

fn cart_dock_cones() -> Part {
    let mut cones = Part::empty("clean_zone_transfer_cart_dock_cones");
    for i in 0..CART_DOCK_CONE_COUNT {
        cones = cones
            + centered_cylinder(
                format!("clean_zone_transfer_cart_dock_cone_{i}"),
                14.0,
                28.0,
                32,
            )
            .translate(
                CART_POS.0 + centered_index(i % 2, 2, 260.0),
                CART_POS.1 + centered_index(i / 2, 2, 86.0),
                DECK_Z + CART_Z + 14.0,
            );
    }
    cones
}

fn cart_guide_rails() -> Part {
    let mut rails = Part::empty("clean_zone_transfer_cart_guide_rails");
    for i in 0..CART_RAIL_COUNT {
        rails = rails
            + centered_cube(
                format!("clean_zone_transfer_cart_guide_rail_{i}"),
                CART_X - 82.0,
                14.0,
                30.0,
            )
            .translate(
                CART_POS.0,
                CART_POS.1 + centered_index(i, CART_RAIL_COUNT, 104.0),
                DECK_Z + CART_Z + 15.0,
            );
    }
    rails
}

fn cart_route_tokens() -> Part {
    let mut tokens = Part::empty("clean_zone_transfer_cart_route_tokens");
    for i in 0..CART_ROUTE_TOKEN_COUNT {
        tokens = tokens
            + centered_cube(
                format!("clean_zone_transfer_cart_route_token_{i}"),
                34.0,
                22.0,
                6.0,
            )
            .translate(
                CART_POS.0 + centered_index(i, CART_ROUTE_TOKEN_COUNT, 48.0),
                CART_POS.1,
                DECK_Z + CART_Z + 3.0,
            );
    }
    tokens
}

fn reject_quarantine_hold_bays() -> Part {
    let panel = centered_cube(
        "clean_zone_reject_quarantine_hold_panel",
        QUARANTINE_X - 32.0,
        QUARANTINE_Y - 32.0,
        QUARANTINE_Z,
    )
    .translate(QUARANTINE_POS.0, QUARANTINE_POS.1, insert_z(QUARANTINE_Z));

    panel + quarantine_bays() + quarantine_lock_pins() + reject_tag_lands()
}

fn quarantine_bays() -> Part {
    let mut bays = Part::empty("clean_zone_quarantine_hold_bays");
    for i in 0..QUARANTINE_BAY_COUNT {
        let bay = rectangular_frame_xy(
            &format!("clean_zone_quarantine_bay_{i}"),
            62.0,
            96.0,
            22.0,
            42.0,
            70.0,
        )
        .translate(
            QUARANTINE_POS.0 + centered_index(i, QUARANTINE_BAY_COUNT, 70.0),
            QUARANTINE_POS.1,
            DECK_Z + QUARANTINE_Z + 11.0,
        );
        bays = bays + bay;
    }
    bays
}

fn quarantine_lock_pins() -> Part {
    let mut pins = Part::empty("clean_zone_quarantine_lock_pins");
    for i in 0..QUARANTINE_LOCK_PIN_COUNT {
        pins = pins
            + centered_cylinder(format!("clean_zone_quarantine_lock_pin_{i}"), 5.0, 44.0, 20)
                .rotate(90.0, 0.0, 0.0)
                .translate(
                    QUARANTINE_POS.0 + centered_index(i, QUARANTINE_LOCK_PIN_COUNT, 34.0),
                    QUARANTINE_POS.1 + QUARANTINE_Y / 2.0 - 36.0,
                    DECK_Z + QUARANTINE_Z + 44.0,
                );
    }
    pins
}

fn reject_tag_lands() -> Part {
    let mut lands = Part::empty("clean_zone_quarantine_reject_tag_lands");
    for i in 0..REJECT_TAG_LAND_COUNT {
        lands = lands
            + centered_cube(
                format!("clean_zone_quarantine_reject_tag_land_{i}"),
                50.0,
                22.0,
                5.0,
            )
            .translate(
                QUARANTINE_POS.0 + centered_index(i, REJECT_TAG_LAND_COUNT, 62.0),
                QUARANTINE_POS.1 - QUARANTINE_Y / 2.0 + 30.0,
                DECK_Z + QUARANTINE_Z + 2.5,
            );
    }
    lands
}

fn environmental_monitoring_witness_tree() -> Part {
    sensor_trees() + particle_counter_ports() + settle_plate_holders() + dp_reference_tokens()
}

fn sensor_trees() -> Part {
    let mut trees = Part::empty("clean_zone_environmental_sensor_trees");
    for i in 0..SENSOR_TREE_COUNT {
        let x = -620.0 + i as f64 * 248.0;
        let y = if i % 2 == 0 { -470.0 } else { 455.0 };
        let base = centered_cylinder(
            format!("clean_zone_environmental_sensor_tree_base_{i}"),
            18.0,
            10.0,
            32,
        )
        .translate(x, y, DECK_Z + 5.0);
        let mast = centered_cylinder(
            format!("clean_zone_environmental_sensor_tree_mast_{i}"),
            6.0,
            150.0,
            20,
        )
        .translate(x, y, DECK_Z + 85.0);
        let head = centered_cube(
            format!("clean_zone_environmental_sensor_tree_probe_head_{i}"),
            54.0,
            24.0,
            24.0,
        )
        .translate(x, y, DECK_Z + 165.0);
        trees = trees + base + mast + head;
    }
    trees
}

fn particle_counter_ports() -> Part {
    let mut ports = Part::empty("clean_zone_particle_counter_sample_ports");
    for i in 0..PARTICLE_COUNTER_PORT_COUNT {
        ports = ports
            + centered_cylinder(
                format!("clean_zone_particle_counter_port_{i}"),
                12.0,
                24.0,
                30,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                -360.0 + i as f64 * 240.0,
                DECK_Y / 2.0 - 34.0,
                DECK_Z + 92.0,
            );
    }
    ports
}

fn settle_plate_holders() -> Part {
    let mut holders = Part::empty("clean_zone_settle_plate_holder_lands");
    for i in 0..SETTLE_PLATE_HOLDER_COUNT {
        holders = holders
            + centered_cylinder(format!("clean_zone_settle_plate_holder_{i}"), 26.0, 7.0, 36)
                .translate(
                    -500.0 + i as f64 * 200.0,
                    -DECK_Y / 2.0 + 42.0,
                    DECK_Z + 3.5,
                );
    }
    holders
}

fn dp_reference_tokens() -> Part {
    let mut tokens = Part::empty("clean_zone_differential_pressure_reference_tokens");
    for i in 0..DP_REFERENCE_TOKEN_COUNT {
        tokens = tokens
            + centered_cube(
                format!("clean_zone_dp_reference_token_{i}"),
                52.0,
                22.0,
                6.0,
            )
            .translate(-330.0 + i as f64 * 220.0, DECK_Y / 2.0 - 72.0, DECK_Z + 3.0);
    }
    tokens
}

fn evidence_camera_and_light_bridge() -> Part {
    let left_post = centered_cube(
        "clean_zone_evidence_bridge_left_post",
        28.0,
        28.0,
        BRIDGE_UNDERSIDE_Z,
    )
    .translate(
        -BRIDGE_SPAN_X / 2.0,
        -BRIDGE_SPAN_Y / 2.0,
        BRIDGE_UNDERSIDE_Z / 2.0,
    );
    let right_post = centered_cube(
        "clean_zone_evidence_bridge_right_post",
        28.0,
        28.0,
        BRIDGE_UNDERSIDE_Z,
    )
    .translate(
        BRIDGE_SPAN_X / 2.0,
        -BRIDGE_SPAN_Y / 2.0,
        BRIDGE_UNDERSIDE_Z / 2.0,
    );
    let rear_left_post = centered_cube(
        "clean_zone_evidence_bridge_rear_left_post",
        28.0,
        28.0,
        BRIDGE_UNDERSIDE_Z,
    )
    .translate(
        -BRIDGE_SPAN_X / 2.0,
        BRIDGE_SPAN_Y / 2.0,
        BRIDGE_UNDERSIDE_Z / 2.0,
    );
    let rear_right_post = centered_cube(
        "clean_zone_evidence_bridge_rear_right_post",
        28.0,
        28.0,
        BRIDGE_UNDERSIDE_Z,
    )
    .translate(
        BRIDGE_SPAN_X / 2.0,
        BRIDGE_SPAN_Y / 2.0,
        BRIDGE_UNDERSIDE_Z / 2.0,
    );
    let front_beam = centered_cube(
        "clean_zone_evidence_bridge_front_beam",
        BRIDGE_SPAN_X + 40.0,
        26.0,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, -BRIDGE_SPAN_Y / 2.0, BRIDGE_UNDERSIDE_Z);
    let rear_beam = centered_cube(
        "clean_zone_evidence_bridge_rear_beam",
        BRIDGE_SPAN_X + 40.0,
        26.0,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, BRIDGE_SPAN_Y / 2.0, BRIDGE_UNDERSIDE_Z);
    let cross_beam = centered_cube(
        "clean_zone_evidence_bridge_center_cross_beam",
        26.0,
        BRIDGE_SPAN_Y + 40.0,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, 0.0, BRIDGE_UNDERSIDE_Z);

    left_post
        + right_post
        + rear_left_post
        + rear_right_post
        + front_beam
        + rear_beam
        + cross_beam
        + camera_pods()
        + light_bars()
}

fn camera_pods() -> Part {
    let mut pods = Part::empty("clean_zone_evidence_camera_pods");
    for i in 0..CAMERA_POD_COUNT {
        let x = centered_index(i % 3, 3, 460.0);
        let y = centered_index(i / 3, 2, 420.0);
        let pod = centered_cube(
            format!("clean_zone_evidence_camera_pod_{i}"),
            52.0,
            38.0,
            30.0,
        )
        .translate(x, y, BRIDGE_UNDERSIDE_Z - 32.0);
        let lens = centered_cylinder(
            format!("clean_zone_evidence_camera_lens_{i}"),
            10.0,
            10.0,
            28,
        )
        .translate(x, y, BRIDGE_UNDERSIDE_Z - 52.0);
        pods = pods + pod + lens;
    }
    pods
}

fn light_bars() -> Part {
    let mut bars = Part::empty("clean_zone_evidence_light_bars");
    for (i, y) in [-310.0, -105.0, 105.0, 310.0].into_iter().enumerate() {
        bars = bars
            + centered_cube(
                format!("clean_zone_evidence_led_bar_{i}"),
                760.0,
                12.0,
                10.0,
            )
            .translate(0.0, y, BRIDGE_UNDERSIDE_Z - 28.0);
    }
    bars
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

fn fiducial_target(name: &str) -> Part {
    let outer = centered_cylinder(format!("{name}_outer_ring"), 18.0, 3.0, 40);
    let inner = centered_cylinder(format!("{name}_center_clearance"), 8.0, 4.0, 32);
    let cross_x = centered_cube(format!("{name}_cross_x"), 36.0, 3.5, 3.0);
    let cross_y = centered_cube(format!("{name}_cross_y"), 3.5, 36.0, 3.0);

    outer - inner + cross_x + cross_y
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
    fn pressure_steps_are_monotonic_and_measurable() {
        for pair in PERSONNEL_SETPOINTS_PA.windows(2) {
            assert!(pair[0] > pair[1]);
            assert!(pair[0] - pair[1] >= MIN_CASCADE_STEP_PA);
        }
    }

    #[test]
    fn count_relationships_are_intentional() {
        assert_eq!(PERSONNEL_PRESSURE_TAP_COUNT, PERSONNEL_ZONE_COUNT * 2);
        assert_eq!(BADGE_DOCK_COUNT, 6);
        assert_eq!(MATERIAL_RELEASE_LANES, PERSONNEL_RELEASE_LANES);
        assert!(MATERIAL_BARCODE_LAND_COUNT >= SCALE_PAD_COUNT + LOT_CARD_SLOT_COUNT);
        assert!(ROUTE_GATE_COUNT >= PERSONNEL_DOOR_COUNT + MATERIAL_TOTE_NEST_COUNT - 1);
        assert!(QUARANTINE_LOCK_PIN_COUNT >= QUARANTINE_BAY_COUNT * 2);
    }

    #[test]
    fn clean_zone_clearances_are_explicit() {
        assert!(FRONT_PERSONNEL_CLEARANCE_Y > 320.0);
        assert!(REAR_PASS_THROUGH_SERVICE_Y > 240.0);
        assert!(LEFT_GOWNING_SERVICE_X > 220.0);
        assert!(RIGHT_MATERIAL_BAGOUT_X > 220.0);
        assert!(OVERHEAD_FILTER_SERVICE_Z > BRIDGE_UNDERSIDE_Z);
        assert_eq!(CAMERA_POD_COUNT, SENSOR_TREE_COUNT);
    }
}
