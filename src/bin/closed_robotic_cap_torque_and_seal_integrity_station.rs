use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed robotic cap torque and seal-integrity validation station.
//
// Intent:
// - Validate capped vial and media-bag port closures in a contained robotic
//   workcell before a closed culture module or reagent set is released.
// - Keep nest datums, torque tool datum, torque witness dials,
//   cap-presence/orientation sensors, septum/gasket compression witnesses,
//   pressure-decay/leak ports, dye ingress wells, cap custody parks,
//   release/hold/reject lanes, evidence capture, and robot/service keepouts
//   physically legible as separate zones.
// - This is station/interface CAD only. It does not define sterile-barrier
//   acceptance criteria, torque recipes, pressure limits, dye chemistry, or
//   release procedures.

const OUTPUT_PREFIX: &str = "closed_robotic_cap_torque_and_seal_integrity_station";

const OUTPUTS: [&str; 13] = [
    "output/closed_robotic_cap_torque_and_seal_integrity_station_base_containment_deck.stl",
    "output/closed_robotic_cap_torque_and_seal_integrity_station_capped_vial_bag_port_nests.stl",
    "output/closed_robotic_cap_torque_and_seal_integrity_station_torque_tool_datum_bridge.stl",
    "output/closed_robotic_cap_torque_and_seal_integrity_station_torque_witness_dials.stl",
    "output/closed_robotic_cap_torque_and_seal_integrity_station_cap_presence_orientation_sensor_bank.stl",
    "output/closed_robotic_cap_torque_and_seal_integrity_station_septum_gasket_compression_witness_bank.stl",
    "output/closed_robotic_cap_torque_and_seal_integrity_station_pressure_decay_leak_port_manifold.stl",
    "output/closed_robotic_cap_torque_and_seal_integrity_station_dye_ingress_wells.stl",
    "output/closed_robotic_cap_torque_and_seal_integrity_station_cap_custody_parks.stl",
    "output/closed_robotic_cap_torque_and_seal_integrity_station_release_hold_reject_lanes.stl",
    "output/closed_robotic_cap_torque_and_seal_integrity_station_evidence_bridge.stl",
    "output/closed_robotic_cap_torque_and_seal_integrity_station_robot_service_keepout_gauges.stl",
    "output/closed_robotic_cap_torque_and_seal_integrity_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "capped_vial_bag_port_nests",
    "torque_tool_datum_bridge",
    "torque_witness_dials",
    "cap_presence_orientation_sensor_bank",
    "septum_gasket_compression_witness_bank",
    "pressure_decay_leak_port_manifold",
    "dye_ingress_wells",
    "cap_custody_parks",
    "release_hold_reject_lanes",
    "evidence_bridge",
    "robot_service_keepout_gauges",
];

const STATION_X: f64 = 1500.0;
const STATION_Y: f64 = 960.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const SOCKET_DEPTH: f64 = 6.0;
const BASIN_RECESS_Z: f64 = 8.0;
const MODULE_GAP: f64 = 12.0;
const MOUNT_HOLE_D: f64 = 6.8;
const MOUNT_HOLE_COUNT: usize = 10;
const DRAIN_D: f64 = 14.0;

const VIAL_ROWS: usize = 2;
const VIAL_COLS: usize = 4;
const VIAL_COUNT: usize = VIAL_ROWS * VIAL_COLS;
const BAG_PORT_COUNT: usize = 4;
const CAP_COUNT: usize = VIAL_COUNT + BAG_PORT_COUNT;

const NEST_CENTER: (f64, f64) = (-470.0, 245.0);
const NEST_X: f64 = 430.0;
const NEST_Y: f64 = 300.0;
const NEST_Z: f64 = 52.0;
const VIAL_PITCH_X: f64 = 82.0;
const VIAL_PITCH_Y: f64 = 72.0;
const VIAL_ROW_OFFSET_Y: f64 = 45.0;
const VIAL_CAP_POCKET_D: f64 = 49.0;
const VIAL_THREAD_RELIEF_D: f64 = 35.0;
const BAG_PORT_PITCH_X: f64 = 86.0;
const BAG_PORT_Y: f64 = -96.0;
const BAG_PORT_SLOT_X: f64 = 58.0;
const BAG_PORT_SLOT_Y: f64 = 34.0;
const NEST_POCKET_DEPTH: f64 = 28.0;
const NEST_DATUM_PIN_COUNT: usize = 4;
const ORIENTATION_KEY_COUNT: usize = CAP_COUNT;

const TORQUE_CENTER: (f64, f64) = (-20.0, 245.0);
const TORQUE_X: f64 = 390.0;
const TORQUE_Y: f64 = 220.0;
const TORQUE_Z: f64 = 48.0;
const TORQUE_POST_Z: f64 = 184.0;
const TORQUE_BEAM_Z: f64 = 24.0;
const TORQUE_TOOL_SOCKET_D: f64 = 64.0;
const TORQUE_DRIVER_CLEARANCE_D: f64 = 32.0;
const TORQUE_DATUM_PIN_COUNT: usize = 4;
const TORQUE_REACTION_DOG_COUNT: usize = 8;
const TORQUE_BIT_DOCKS: usize = 6;

const DIAL_CENTER: (f64, f64) = (400.0, 245.0);
const DIAL_X: f64 = 360.0;
const DIAL_Y: f64 = 220.0;
const DIAL_Z: f64 = 32.0;
const DIAL_COLS: usize = 4;
const DIAL_ROWS: usize = 3;
const TORQUE_DIAL_COUNT: usize = DIAL_COLS * DIAL_ROWS;
const DIAL_PITCH_X: f64 = 76.0;
const DIAL_PITCH_Y: f64 = 58.0;
const DIAL_OD: f64 = 42.0;
const DIAL_ID: f64 = 27.0;
const DIAL_TICKS_PER_CAP: usize = 5;
const TORQUE_WITNESS_POINTERS: usize = TORQUE_DIAL_COUNT;

const SENSOR_CENTER: (f64, f64) = (-500.0, -90.0);
const SENSOR_X: f64 = 350.0;
const SENSOR_Y: f64 = 230.0;
const SENSOR_Z: f64 = 34.0;
const SENSOR_COLS: usize = 4;
const SENSOR_ROWS: usize = 3;
const SENSOR_COUNT: usize = SENSOR_COLS * SENSOR_ROWS;
const SENSOR_PITCH_X: f64 = 72.0;
const SENSOR_PITCH_Y: f64 = 56.0;
const OPTICAL_SLOT_X: f64 = 36.0;
const OPTICAL_SLOT_Y: f64 = 9.0;
const ORIENTATION_FORKS: usize = SENSOR_COUNT;
const SENSOR_CABLE_CHANNELS: usize = 3;

const COMPRESSION_CENTER: (f64, f64) = (-125.0, -90.0);
const COMPRESSION_X: f64 = 340.0;
const COMPRESSION_Y: f64 = 230.0;
const COMPRESSION_Z: f64 = 40.0;
const COMPRESSION_WITNESS_COUNT: usize = CAP_COUNT;
const COMPRESSION_COLS: usize = 4;
const COMPRESSION_ROWS: usize = 3;
const COMPRESSION_PITCH_X: f64 = 70.0;
const COMPRESSION_PITCH_Y: f64 = 56.0;
const SEPTUM_WITNESS_D: f64 = 32.0;
const GASKET_WITNESS_X: f64 = 44.0;
const GASKET_WITNESS_Y: f64 = 28.0;
const COMPRESSION_STEP_COUNT: usize = 5;
const NOMINAL_GASKET_THICKNESS: f64 = 2.4;
const NOMINAL_COMPRESSION: f64 = 0.24;
const LOW_COMPRESSION: f64 = 0.18;
const HIGH_COMPRESSION: f64 = 0.30;

const LEAK_CENTER: (f64, f64) = (260.0, -90.0);
const LEAK_X: f64 = 370.0;
const LEAK_Y: f64 = 230.0;
const LEAK_Z: f64 = 42.0;
const LEAK_PORT_COUNT: usize = CAP_COUNT;
const LEAK_COLS: usize = 4;
const LEAK_ROWS: usize = 3;
const LEAK_PITCH_X: f64 = 76.0;
const LEAK_PITCH_Y: f64 = 54.0;
const LEAK_PORT_D: f64 = 8.0;
const LEAK_COLLAR_D: f64 = 24.0;
const REFERENCE_VOLUME_COUNT: usize = 3;
const PRESSURE_TRANSDUCER_DOCKS: usize = 4;
const DECAY_TRACE_CHANNELS: usize = 6;

const DYE_CENTER: (f64, f64) = (580.0, -90.0);
const DYE_X: f64 = 230.0;
const DYE_Y: f64 = 230.0;
const DYE_Z: f64 = 36.0;
const DYE_WELL_COUNT: usize = CAP_COUNT;
const DYE_COLS: usize = 3;
const DYE_ROWS: usize = 4;
const DYE_PITCH_X: f64 = 55.0;
const DYE_PITCH_Y: f64 = 44.0;
const DYE_WELL_D: f64 = 22.0;
const DYE_OVERFLOW_SLOTS: usize = 4;
const DYE_WICK_TABS: usize = 8;

const PARK_CENTER: (f64, f64) = (-460.0, -350.0);
const PARK_X: f64 = 430.0;
const PARK_Y: f64 = 130.0;
const PARK_Z: f64 = 34.0;
const CAP_PARK_COUNT: usize = CAP_COUNT;
const CUSTODY_LANES: usize = 3;
const CUSTODY_SLOTS_PER_LANE: usize = 4;
const PARK_SOCKET_D: f64 = 38.0;
const CUSTODY_TOKEN_COUNT: usize = CAP_COUNT;

const STATUS_CENTER: (f64, f64) = (-40.0, -350.0);
const STATUS_X: f64 = 360.0;
const STATUS_Y: f64 = 130.0;
const STATUS_Z: f64 = 40.0;
const DISPOSITION_LANES: usize = 3;
const SLOTS_PER_DISPOSITION: usize = 4;
const DISPOSITION_SLOT_COUNT: usize = DISPOSITION_LANES * SLOTS_PER_DISPOSITION;
const LANE_WALL_W: f64 = 7.0;
const RELEASE_FLAG_COUNT: usize = 4;
const HOLD_FLAG_COUNT: usize = 4;
const REJECT_FLAG_COUNT: usize = 4;

const EVIDENCE_CENTER: (f64, f64) = (0.0, 426.0);
const EVIDENCE_X: f64 = 1280.0;
const EVIDENCE_Y: f64 = 38.0;
const EVIDENCE_POST_Z: f64 = 210.0;
const EVIDENCE_BEAM_Z: f64 = 24.0;
const CAMERA_COUNT: usize = 5;
const LIGHT_BAR_COUNT: usize = 2;
const EVIDENCE_FIDUCIAL_COUNT: usize = 10;
const CAMERA_CLEARANCE_Z: f64 = 172.0;

const KEEP_OUT_X: f64 = 1400.0;
const KEEP_OUT_Y: f64 = 870.0;
const KEEP_OUT_STRIP_Z: f64 = 7.0;
const KEEP_OUT_GAUGE_COUNT: usize = 8;
const ROBOT_PICK_CLEARANCE_Z: f64 = 292.0;
const TORQUE_TOOL_CLEARANCE_Z: f64 = 318.0;
const PRESSURE_SERVICE_CLEARANCE_Z: f64 = 238.0;
const FRONT_ROBOT_CLEARANCE_Y: f64 = 132.0;
const REAR_SERVICE_CLEARANCE_Y: f64 = 112.0;
const SIDE_SERVICE_CLEARANCE_X: f64 = 118.0;

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

    fn clears(self, other: Rect, clearance: f64) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();

        dx >= (self.x + other.x) / 2.0 + clearance || dy >= (self.y + other.y) / 2.0 + clearance
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DispositionLane {
    Release,
    Hold,
    Reject,
}

impl DispositionLane {
    fn all() -> [DispositionLane; DISPOSITION_LANES] {
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

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_containment_deck();
    export(OUTPUTS[0], &base);

    let nests = capped_vial_bag_port_nests();
    export(OUTPUTS[1], &nests);

    let torque = torque_tool_datum_bridge();
    export(OUTPUTS[2], &torque);

    let dials = torque_witness_dials();
    export(OUTPUTS[3], &dials);

    let sensors = cap_presence_orientation_sensor_bank();
    export(OUTPUTS[4], &sensors);

    let compression = septum_gasket_compression_witness_bank();
    export(OUTPUTS[5], &compression);

    let leak = pressure_decay_leak_port_manifold();
    export(OUTPUTS[6], &leak);

    let dye = dye_ingress_wells();
    export(OUTPUTS[7], &dye);

    let parks = cap_custody_parks();
    export(OUTPUTS[8], &parks);

    let lanes = release_hold_reject_lanes();
    export(OUTPUTS[9], &lanes);

    let evidence = evidence_bridge();
    export(OUTPUTS[10], &evidence);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[11], &keepouts);

    let assembly = base
        + nests.translate(NEST_CENTER.0, NEST_CENTER.1, module_z_offset())
        + torque.translate(TORQUE_CENTER.0, TORQUE_CENTER.1, module_z_offset())
        + dials.translate(DIAL_CENTER.0, DIAL_CENTER.1, module_z_offset())
        + sensors.translate(SENSOR_CENTER.0, SENSOR_CENTER.1, module_z_offset())
        + compression.translate(
            COMPRESSION_CENTER.0,
            COMPRESSION_CENTER.1,
            module_z_offset(),
        )
        + leak.translate(LEAK_CENTER.0, LEAK_CENTER.1, module_z_offset())
        + dye.translate(DYE_CENTER.0, DYE_CENTER.1, module_z_offset())
        + parks.translate(PARK_CENTER.0, PARK_CENTER.1, module_z_offset())
        + lanes.translate(STATUS_CENTER.0, STATUS_CENTER.1, module_z_offset())
        + evidence.translate(EVIDENCE_CENTER.0, EVIDENCE_CENTER.1, BASE_Z)
        + keepouts;
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed robotic cap torque and seal-integrity validation station:");
    println!("  Generator:              {OUTPUT_PREFIX}");
    println!(
        "  Footprint:              {STATION_X:.0}mm x {STATION_Y:.0}mm contained deck with {RIM_Z:.0}mm perimeter rim"
    );
    println!(
        "  Cap formats:            {VIAL_COUNT} capped vial nests + {BAG_PORT_COUNT} capped bag-port nests, {ORIENTATION_KEY_COUNT} orientation keys"
    );
    println!(
        "  Torque evidence:        datum bridge, {TORQUE_REACTION_DOG_COUNT} reaction dogs, {TORQUE_DIAL_COUNT} torque witness dials with {DIAL_TICKS_PER_CAP} ticks each"
    );
    println!(
        "  Presence/orientation:   {SENSOR_COUNT} presence sensors, {ORIENTATION_FORKS} orientation forks, {SENSOR_CABLE_CHANNELS} cable channels"
    );
    println!(
        "  Seal integrity:         {COMPRESSION_WITNESS_COUNT} compression witnesses, {LEAK_PORT_COUNT} pressure-decay ports, {DYE_WELL_COUNT} dye ingress wells"
    );
    println!(
        "  Custody/disposition:    {CAP_PARK_COUNT} cap custody parks and {DISPOSITION_SLOT_COUNT} release/hold/reject slots"
    );
    println!(
        "  Evidence/keepouts:      {CAMERA_COUNT} cameras, {LIGHT_BAR_COUNT} light bars, {KEEP_OUT_GAUGE_COUNT} keepout gauges, {TORQUE_TOOL_CLEARANCE_Z:.0}mm torque tool clearance"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn module_z_offset() -> f64 {
    BASE_Z - SOCKET_DEPTH
}

fn base_containment_deck() -> Part {
    let deck = centered_cube(
        "cap_torque_seal_station_base_containment_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let basin = centered_cube(
        "cap_torque_seal_station_recessed_leak_and_dye_basin",
        STATION_X - 138.0,
        STATION_Y - 128.0,
        BASIN_RECESS_Z + 0.4,
    )
    .translate(0.0, -10.0, BASE_Z - BASIN_RECESS_Z / 2.0 + 0.2);
    let front_sump = centered_cube(
        "cap_torque_seal_station_front_low_point_sump",
        STATION_X - 240.0,
        36.0,
        9.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 58.0, BASE_Z - 4.5);
    let drain = centered_cylinder(
        "cap_torque_seal_station_closed_drain_witness_port",
        DRAIN_D / 2.0,
        58.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 92.0,
        -STATION_Y / 2.0 + 36.0,
        BASE_Z - 8.0,
    );

    deck - basin - front_sump - drain - module_socket_recesses() - mounting_holes()
        + perimeter_rims()
        + flow_lane_markers()
        + deck_datum_targets()
}

fn module_socket_recesses() -> Part {
    let mut sockets = Part::empty("cap_torque_seal_station_module_socket_recesses");
    for rect in layout_rects() {
        sockets = sockets
            + centered_cube(
                format!("cap_torque_seal_station_{}_socket_recess", rect.name),
                rect.x + 12.0,
                rect.y + 12.0,
                SOCKET_DEPTH + 0.6,
            )
            .translate(
                rect.center.0,
                rect.center.1,
                BASE_Z - SOCKET_DEPTH / 2.0 + 0.2,
            );
    }
    sockets
}

fn mounting_holes() -> Part {
    let mut holes = Part::empty("cap_torque_seal_station_mounting_holes");
    for (i, (x, y)) in mount_points().iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("cap_torque_seal_station_m6_mount_hole_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 6.0,
                28,
            )
            .translate(*x, *y, BASE_Z / 2.0)
            + centered_cube(
                format!("cap_torque_seal_station_m6_mount_slot_relief_{i}"),
                28.0,
                MOUNT_HOLE_D + 0.6,
                BASE_Z + 6.0,
            )
            .translate(*x, *y, BASE_Z / 2.0);
    }
    holes
}

fn mount_points() -> [(f64, f64); MOUNT_HOLE_COUNT] {
    [
        (-STATION_X / 2.0 + 58.0, -STATION_Y / 2.0 + 58.0),
        (STATION_X / 2.0 - 58.0, -STATION_Y / 2.0 + 58.0),
        (-STATION_X / 2.0 + 58.0, STATION_Y / 2.0 - 58.0),
        (STATION_X / 2.0 - 58.0, STATION_Y / 2.0 - 58.0),
        (0.0, -STATION_Y / 2.0 + 58.0),
        (0.0, STATION_Y / 2.0 - 58.0),
        (-STATION_X / 2.0 + 58.0, 0.0),
        (STATION_X / 2.0 - 58.0, 0.0),
        (-STATION_X / 2.0 + 58.0, -250.0),
        (STATION_X / 2.0 - 58.0, -250.0),
    ]
}

fn perimeter_rims() -> Part {
    let front = centered_cube(
        "cap_torque_seal_station_front_robot_low_spill_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        "cap_torque_seal_station_rear_service_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "cap_torque_seal_station_left_robot_clearance_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "cap_torque_seal_station_right_pressure_service_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    front + rear + left + right
}

fn flow_lane_markers() -> Part {
    let nests_to_torque = centered_cube(
        "cap_torque_seal_station_nests_to_torque_flow_land",
        420.0,
        7.0,
        4.0,
    )
    .translate(-248.0, 245.0, BASE_Z + 2.0);
    let torque_to_dials = centered_cube(
        "cap_torque_seal_station_torque_to_dial_flow_land",
        330.0,
        7.0,
        4.0,
    )
    .translate(185.0, 245.0, BASE_Z + 2.0);
    let sensor_to_integrity = centered_cube(
        "cap_torque_seal_station_sensor_to_integrity_flow_land",
        570.0,
        7.0,
        4.0,
    )
    .translate(-115.0, -90.0, BASE_Z + 2.0);
    let leak_to_dye = centered_cube(
        "cap_torque_seal_station_leak_to_dye_flow_land",
        150.0,
        7.0,
        4.0,
    )
    .translate(485.0, -90.0, BASE_Z + 2.0);
    let custody_to_status = centered_cube(
        "cap_torque_seal_station_custody_to_disposition_flow_land",
        340.0,
        7.0,
        4.0,
    )
    .translate(-250.0, -350.0, BASE_Z + 2.0);
    nests_to_torque + torque_to_dials + sensor_to_integrity + leak_to_dye + custody_to_status
}

fn deck_datum_targets() -> Part {
    let mut targets = Part::empty("cap_torque_seal_station_deck_robot_datum_targets");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 102.0, -STATION_Y / 2.0 + 104.0),
        (STATION_X / 2.0 - 102.0, -STATION_Y / 2.0 + 104.0),
        (-STATION_X / 2.0 + 102.0, STATION_Y / 2.0 - 104.0),
        (STATION_X / 2.0 - 102.0, STATION_Y / 2.0 - 104.0),
    ]
    .into_iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("cap_torque_seal_station_deck_datum_boss_{i}"),
            15.0,
            7.0,
            36,
        )
        .translate(x, y, BASE_Z + 3.5);
        let sight = centered_cube(
            format!("cap_torque_seal_station_deck_datum_crosshair_{i}"),
            28.0,
            2.4,
            8.0,
        )
        .translate(x, y, BASE_Z + 4.0)
            + centered_cube(
                format!("cap_torque_seal_station_deck_datum_crosshair_cross_{i}"),
                2.4,
                28.0,
                8.0,
            )
            .translate(x, y, BASE_Z + 4.0);
        targets = targets + boss - sight;
    }
    targets
}

fn capped_vial_bag_port_nests() -> Part {
    let body = centered_cube(
        "cap_torque_seal_nests_dual_format_body",
        NEST_X,
        NEST_Y,
        NEST_Z,
    )
    .translate(0.0, 0.0, NEST_Z / 2.0);
    let rear_label_fence = centered_cube(
        "cap_torque_seal_nests_rear_lot_label_fence",
        NEST_X - 28.0,
        14.0,
        NEST_Z + 24.0,
    )
    .translate(0.0, NEST_Y / 2.0 - 16.0, NEST_Z / 2.0 + 12.0);
    let clean_side_fence = centered_cube(
        "cap_torque_seal_nests_left_clean_side_reference_fence",
        14.0,
        NEST_Y - 34.0,
        NEST_Z + 18.0,
    )
    .translate(-NEST_X / 2.0 + 17.0, -2.0, NEST_Z / 2.0 + 9.0);

    body + rear_label_fence + clean_side_fence - vial_pocket_cuts() - bag_port_slot_cuts()
        + vial_retaining_rings()
        + bag_port_capture_rails()
        + orientation_key_lands()
        + nest_datum_pins()
        + robot_finger_reliefs()
}

fn vial_pocket_cuts() -> Part {
    let mut cuts = Part::empty("cap_torque_seal_vial_cap_pocket_cuts");
    for row in 0..VIAL_ROWS {
        for col in 0..VIAL_COLS {
            let index = vial_index(row, col);
            let (x, y) = vial_xy(row, col);
            cuts = cuts
                + centered_cylinder(
                    format!("cap_torque_seal_vial_cap_pocket_{index}"),
                    VIAL_CAP_POCKET_D / 2.0,
                    NEST_POCKET_DEPTH + 1.0,
                    56,
                )
                .translate(x, y, NEST_Z - NEST_POCKET_DEPTH / 2.0 + 0.5)
                + centered_cylinder(
                    format!("cap_torque_seal_vial_thread_relief_{index}"),
                    VIAL_THREAD_RELIEF_D / 2.0,
                    NEST_POCKET_DEPTH + 8.0,
                    44,
                )
                .translate(x, y, NEST_Z - NEST_POCKET_DEPTH / 2.0 - 3.0)
                + centered_cube(
                    format!("cap_torque_seal_vial_gripper_relief_cut_{index}"),
                    18.0,
                    VIAL_CAP_POCKET_D + 18.0,
                    NEST_POCKET_DEPTH + 2.0,
                )
                .translate(x, y - 2.0, NEST_Z - NEST_POCKET_DEPTH / 2.0 + 0.5);
        }
    }
    cuts
}

fn bag_port_slot_cuts() -> Part {
    let mut cuts = Part::empty("cap_torque_seal_bag_port_slot_cuts");
    for i in 0..BAG_PORT_COUNT {
        let (x, y) = bag_port_xy(i);
        cuts = cuts
            + centered_cube(
                format!("cap_torque_seal_bag_port_cap_slot_cut_{i}"),
                BAG_PORT_SLOT_X,
                BAG_PORT_SLOT_Y,
                NEST_POCKET_DEPTH + 1.0,
            )
            .translate(x, y, NEST_Z - NEST_POCKET_DEPTH / 2.0 + 0.5)
            + centered_cylinder(
                format!("cap_torque_seal_bag_port_luer_cap_recess_{i}"),
                15.0,
                NEST_POCKET_DEPTH + 5.0,
                36,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, y, NEST_Z - 12.0);
    }
    cuts
}

fn vial_retaining_rings() -> Part {
    let mut rings = Part::empty("cap_torque_seal_vial_retaining_rings");
    for row in 0..VIAL_ROWS {
        for col in 0..VIAL_COLS {
            let index = vial_index(row, col);
            let (x, y) = vial_xy(row, col);
            rings = rings
                + retaining_ring(
                    &format!("cap_torque_seal_vial_{index}_retaining_ring"),
                    VIAL_CAP_POCKET_D / 2.0 + 5.0,
                    VIAL_CAP_POCKET_D / 2.0 + 0.8,
                    5.0,
                )
                .translate(x, y, NEST_Z + 2.5);
        }
    }
    rings
}

fn bag_port_capture_rails() -> Part {
    let mut rails = Part::empty("cap_torque_seal_bag_port_capture_rails");
    for i in 0..BAG_PORT_COUNT {
        let (x, y) = bag_port_xy(i);
        let front = centered_cube(
            format!("cap_torque_seal_bag_port_{i}_front_capture_rail"),
            BAG_PORT_SLOT_X + 16.0,
            8.0,
            18.0,
        )
        .translate(x, y - BAG_PORT_SLOT_Y / 2.0 - 9.0, NEST_Z + 9.0);
        let rear = centered_cube(
            format!("cap_torque_seal_bag_port_{i}_rear_capture_rail"),
            BAG_PORT_SLOT_X + 16.0,
            8.0,
            18.0,
        )
        .translate(x, y + BAG_PORT_SLOT_Y / 2.0 + 9.0, NEST_Z + 9.0);
        rails = rails + front + rear;
    }
    rails
}

fn orientation_key_lands() -> Part {
    let mut keys = Part::empty("cap_torque_seal_nest_orientation_key_lands");
    for cap in 0..CAP_COUNT {
        let (x, y) = cap_xy(cap);
        let key_y = if cap < VIAL_COUNT { y + 34.0 } else { y + 30.0 };
        keys = keys
            + centered_cube(
                format!("cap_torque_seal_nest_orientation_key_land_{cap}"),
                22.0,
                6.0,
                7.0,
            )
            .translate(x, key_y, NEST_Z + 3.5);
    }
    keys
}

fn nest_datum_pins() -> Part {
    let mut pins = Part::empty("cap_torque_seal_nest_datum_pins");
    for (i, (x, y)) in [
        (-NEST_X / 2.0 + 32.0, -NEST_Y / 2.0 + 28.0),
        (NEST_X / 2.0 - 32.0, -NEST_Y / 2.0 + 28.0),
        (-NEST_X / 2.0 + 32.0, NEST_Y / 2.0 - 34.0),
        (NEST_X / 2.0 - 32.0, NEST_Y / 2.0 - 34.0),
    ]
    .into_iter()
    .enumerate()
    {
        pins = pins
            + centered_cylinder(
                format!("cap_torque_seal_nest_datum_pin_boss_{i}"),
                12.0,
                8.0,
                32,
            )
            .translate(x, y, NEST_Z + 4.0)
            + centered_cylinder(format!("cap_torque_seal_nest_datum_pin_{i}"), 3.2, 24.0, 24)
                .translate(x, y, NEST_Z + 12.0);
    }
    pins
}

fn robot_finger_reliefs() -> Part {
    let left = centered_cube(
        "cap_torque_seal_nest_left_robot_finger_relief_marker",
        32.0,
        116.0,
        5.0,
    )
    .translate(-NEST_X / 2.0 + 54.0, -22.0, NEST_Z + 2.5);
    let right = centered_cube(
        "cap_torque_seal_nest_right_robot_finger_relief_marker",
        32.0,
        116.0,
        5.0,
    )
    .translate(NEST_X / 2.0 - 54.0, -22.0, NEST_Z + 2.5);
    left + right
}

fn torque_tool_datum_bridge() -> Part {
    let plate = centered_cube(
        "cap_torque_seal_torque_tool_datum_plate",
        TORQUE_X,
        TORQUE_Y,
        TORQUE_Z,
    )
    .translate(0.0, 0.0, TORQUE_Z / 2.0);
    let pocket = centered_cylinder(
        "cap_torque_seal_torque_driver_socket_cut",
        TORQUE_TOOL_SOCKET_D / 2.0,
        TORQUE_Z + 2.0,
        64,
    )
    .translate(0.0, 0.0, TORQUE_Z / 2.0);
    let driver_clearance = centered_cylinder(
        "cap_torque_seal_torque_driver_axis_clearance",
        TORQUE_DRIVER_CLEARANCE_D / 2.0,
        TORQUE_POST_Z + TORQUE_BEAM_Z + 20.0,
        56,
    )
    .translate(0.0, 0.0, TORQUE_Z + TORQUE_POST_Z / 2.0);

    plate - pocket - driver_clearance
        + torque_bridge_posts()
        + torque_bridge_beams()
        + torque_datum_pin_field()
        + torque_reaction_dogs()
        + torque_bit_docks()
}

fn torque_bridge_posts() -> Part {
    let mut posts = Part::empty("cap_torque_seal_torque_bridge_posts");
    for (i, (x, y)) in [
        (-TORQUE_X / 2.0 + 30.0, -TORQUE_Y / 2.0 + 28.0),
        (TORQUE_X / 2.0 - 30.0, -TORQUE_Y / 2.0 + 28.0),
        (-TORQUE_X / 2.0 + 30.0, TORQUE_Y / 2.0 - 28.0),
        (TORQUE_X / 2.0 - 30.0, TORQUE_Y / 2.0 - 28.0),
    ]
    .into_iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(
                format!("cap_torque_seal_torque_bridge_post_{i}"),
                24.0,
                24.0,
                TORQUE_POST_Z,
            )
            .translate(x, y, TORQUE_Z + TORQUE_POST_Z / 2.0);
    }
    posts
}

fn torque_bridge_beams() -> Part {
    let z = TORQUE_Z + TORQUE_POST_Z - TORQUE_BEAM_Z / 2.0;
    let front = centered_cube(
        "cap_torque_seal_torque_bridge_front_beam",
        TORQUE_X - 42.0,
        22.0,
        TORQUE_BEAM_Z,
    )
    .translate(0.0, -TORQUE_Y / 2.0 + 28.0, z);
    let rear = centered_cube(
        "cap_torque_seal_torque_bridge_rear_beam",
        TORQUE_X - 42.0,
        22.0,
        TORQUE_BEAM_Z,
    )
    .translate(0.0, TORQUE_Y / 2.0 - 28.0, z);
    let left = centered_cube(
        "cap_torque_seal_torque_bridge_left_cross_beam",
        22.0,
        TORQUE_Y - 42.0,
        TORQUE_BEAM_Z,
    )
    .translate(-TORQUE_X / 2.0 + 30.0, 0.0, z);
    let right = centered_cube(
        "cap_torque_seal_torque_bridge_right_cross_beam",
        22.0,
        TORQUE_Y - 42.0,
        TORQUE_BEAM_Z,
    )
    .translate(TORQUE_X / 2.0 - 30.0, 0.0, z);
    front + rear + left + right
}

fn torque_datum_pin_field() -> Part {
    let mut pins = Part::empty("cap_torque_seal_torque_datum_pin_field");
    for (i, (x, y)) in [(-92.0, -62.0), (92.0, -62.0), (-92.0, 62.0), (92.0, 62.0)]
        .into_iter()
        .enumerate()
    {
        pins = pins
            + centered_cylinder(
                format!("cap_torque_seal_torque_tool_datum_boss_{i}"),
                15.0,
                7.0,
                36,
            )
            .translate(x, y, TORQUE_Z + 3.5)
            - centered_cylinder(
                format!("cap_torque_seal_torque_tool_datum_pin_hole_{i}"),
                3.2,
                8.0,
                24,
            )
            .translate(x, y, TORQUE_Z + 4.0);
    }
    pins
}

fn torque_reaction_dogs() -> Part {
    let mut dogs = Part::empty("cap_torque_seal_torque_reaction_dogs");
    for i in 0..TORQUE_REACTION_DOG_COUNT {
        let angle = i as f64 * 45.0;
        let x = 96.0 * angle.to_radians().cos();
        let y = 72.0 * angle.to_radians().sin();
        dogs = dogs
            + centered_cube(
                format!("cap_torque_seal_torque_reaction_dog_{i}"),
                34.0,
                12.0,
                24.0,
            )
            .rotate(0.0, 0.0, angle)
            .translate(x, y, TORQUE_Z + 12.0);
    }
    dogs
}

fn torque_bit_docks() -> Part {
    let mut docks = Part::empty("cap_torque_seal_torque_bit_docks");
    for i in 0..TORQUE_BIT_DOCKS {
        let x = centered_index(i, TORQUE_BIT_DOCKS, 42.0);
        docks = docks
            + retaining_ring(
                &format!("cap_torque_seal_torque_bit_dock_{i}"),
                13.0,
                7.0,
                6.0,
            )
            .translate(x, -TORQUE_Y / 2.0 + 34.0, TORQUE_Z + 3.0);
    }
    docks
}

fn torque_witness_dials() -> Part {
    let plate = centered_cube(
        "cap_torque_seal_torque_witness_dial_plate",
        DIAL_X,
        DIAL_Y,
        DIAL_Z,
    )
    .translate(0.0, 0.0, DIAL_Z / 2.0);
    let top_relief = centered_cube(
        "cap_torque_seal_torque_witness_dial_top_relief",
        DIAL_X - 34.0,
        DIAL_Y - 30.0,
        8.0,
    )
    .translate(0.0, 0.0, DIAL_Z - 3.5);

    plate - top_relief + dial_faces() + dial_ticks() + dial_pointers() + dial_reference_strip()
}

fn dial_faces() -> Part {
    let mut faces = Part::empty("cap_torque_seal_witness_dial_faces");
    for dial in 0..TORQUE_DIAL_COUNT {
        let (x, y) = dial_xy(dial);
        faces = faces
            + retaining_ring(
                &format!("cap_torque_seal_torque_witness_dial_{dial}"),
                DIAL_OD / 2.0,
                DIAL_ID / 2.0,
                5.0,
            )
            .translate(x, y, DIAL_Z + 2.5)
            + centered_cylinder(
                format!("cap_torque_seal_torque_witness_dial_{dial}_hub"),
                4.0,
                7.0,
                24,
            )
            .translate(x, y, DIAL_Z + 3.5);
    }
    faces
}

fn dial_ticks() -> Part {
    let mut ticks = Part::empty("cap_torque_seal_witness_dial_ticks");
    for dial in 0..TORQUE_DIAL_COUNT {
        let (cx, cy) = dial_xy(dial);
        for tick in 0..DIAL_TICKS_PER_CAP {
            let angle = -80.0 + tick as f64 * 40.0;
            let radius = DIAL_OD / 2.0 + 6.0;
            let x = cx + radius * angle.to_radians().cos();
            let y = cy + radius * angle.to_radians().sin();
            let tick_len = if tick == DIAL_TICKS_PER_CAP / 2 {
                16.0
            } else {
                10.0
            };
            ticks = ticks
                + centered_cube(
                    format!("cap_torque_seal_torque_dial_{dial}_witness_tick_{tick}"),
                    tick_len,
                    3.0,
                    5.0,
                )
                .rotate(0.0, 0.0, angle)
                .translate(x, y, DIAL_Z + 2.5);
        }
    }
    ticks
}

fn dial_pointers() -> Part {
    let mut pointers = Part::empty("cap_torque_seal_witness_dial_pointers");
    for dial in 0..TORQUE_WITNESS_POINTERS {
        let (x, y) = dial_xy(dial);
        let angle = -44.0 + dial as f64 * 8.0;
        pointers = pointers
            + centered_cube(
                format!("cap_torque_seal_torque_witness_pointer_{dial}"),
                DIAL_OD / 2.0 + 4.0,
                4.0,
                5.0,
            )
            .rotate(0.0, 0.0, angle)
            .translate(
                x + 7.0 * angle.to_radians().cos(),
                y + 7.0 * angle.to_radians().sin(),
                DIAL_Z + 6.5,
            );
    }
    pointers
}

fn dial_reference_strip() -> Part {
    let green = centered_cube(
        "cap_torque_seal_torque_witness_nominal_band",
        DIAL_X - 44.0,
        7.0,
        5.0,
    )
    .translate(0.0, -DIAL_Y / 2.0 + 24.0, DIAL_Z + 2.5);
    let under = centered_cube(
        "cap_torque_seal_torque_witness_under_torque_band",
        86.0,
        7.0,
        5.0,
    )
    .translate(-112.0, -DIAL_Y / 2.0 + 39.0, DIAL_Z + 2.5);
    let over = centered_cube(
        "cap_torque_seal_torque_witness_over_torque_band",
        86.0,
        7.0,
        5.0,
    )
    .translate(112.0, -DIAL_Y / 2.0 + 39.0, DIAL_Z + 2.5);
    green + under + over
}

fn cap_presence_orientation_sensor_bank() -> Part {
    let body = centered_cube(
        "cap_torque_seal_presence_orientation_sensor_bank_body",
        SENSOR_X,
        SENSOR_Y,
        SENSOR_Z,
    )
    .translate(0.0, 0.0, SENSOR_Z / 2.0);
    let wireway = centered_cube(
        "cap_torque_seal_presence_orientation_sensor_rear_wireway_cut",
        SENSOR_X - 44.0,
        20.0,
        SENSOR_Z + 2.0,
    )
    .translate(0.0, SENSOR_Y / 2.0 - 22.0, SENSOR_Z / 2.0);

    body - wireway - sensor_optical_slots()
        + sensor_emitter_receivers()
        + orientation_forks()
        + cable_channel_covers()
}

fn sensor_optical_slots() -> Part {
    let mut cuts = Part::empty("cap_torque_seal_sensor_optical_slot_cuts");
    for sensor in 0..SENSOR_COUNT {
        let (x, y) = sensor_xy(sensor);
        cuts = cuts
            + centered_cube(
                format!("cap_torque_seal_presence_sensor_{sensor}_beam_slot_cut"),
                OPTICAL_SLOT_X,
                OPTICAL_SLOT_Y,
                SENSOR_Z + 2.0,
            )
            .translate(x, y, SENSOR_Z / 2.0);
    }
    cuts
}

fn sensor_emitter_receivers() -> Part {
    let mut blocks = Part::empty("cap_torque_seal_sensor_emitter_receiver_blocks");
    for sensor in 0..SENSOR_COUNT {
        let (x, y) = sensor_xy(sensor);
        blocks = blocks
            + centered_cube(
                format!("cap_torque_seal_presence_sensor_{sensor}_emitter_block"),
                12.0,
                18.0,
                16.0,
            )
            .translate(x - OPTICAL_SLOT_X / 2.0 - 11.0, y, SENSOR_Z + 8.0)
            + centered_cube(
                format!("cap_torque_seal_presence_sensor_{sensor}_receiver_block"),
                12.0,
                18.0,
                16.0,
            )
            .translate(x + OPTICAL_SLOT_X / 2.0 + 11.0, y, SENSOR_Z + 8.0);
    }
    blocks
}

fn orientation_forks() -> Part {
    let mut forks = Part::empty("cap_torque_seal_orientation_key_forks");
    for sensor in 0..ORIENTATION_FORKS {
        let (x, y) = sensor_xy(sensor);
        let fork_y = y + 22.0;
        forks = forks
            + centered_cube(
                format!("cap_torque_seal_orientation_fork_{sensor}_left_tine"),
                6.0,
                24.0,
                18.0,
            )
            .translate(x - 12.0, fork_y, SENSOR_Z + 9.0)
            + centered_cube(
                format!("cap_torque_seal_orientation_fork_{sensor}_right_tine"),
                6.0,
                24.0,
                18.0,
            )
            .translate(x + 12.0, fork_y, SENSOR_Z + 9.0)
            + centered_cube(
                format!("cap_torque_seal_orientation_fork_{sensor}_backstop"),
                30.0,
                5.0,
                12.0,
            )
            .translate(x, fork_y + 13.0, SENSOR_Z + 6.0);
    }
    forks
}

fn cable_channel_covers() -> Part {
    let mut covers = Part::empty("cap_torque_seal_sensor_cable_channel_covers");
    for i in 0..SENSOR_CABLE_CHANNELS {
        let y = centered_index(i, SENSOR_CABLE_CHANNELS, 52.0) - 4.0;
        covers = covers
            + centered_cube(
                format!("cap_torque_seal_sensor_cable_channel_cover_{i}"),
                SENSOR_X - 58.0,
                7.0,
                7.0,
            )
            .translate(0.0, y, SENSOR_Z + 3.5);
    }
    covers
}

fn septum_gasket_compression_witness_bank() -> Part {
    let body = centered_cube(
        "cap_torque_seal_compression_witness_bank_body",
        COMPRESSION_X,
        COMPRESSION_Y,
        COMPRESSION_Z,
    )
    .translate(0.0, 0.0, COMPRESSION_Z / 2.0);
    let front_step_recess = centered_cube(
        "cap_torque_seal_compression_witness_front_step_recess",
        COMPRESSION_X - 42.0,
        28.0,
        8.0,
    )
    .translate(0.0, -COMPRESSION_Y / 2.0 + 27.0, COMPRESSION_Z - 3.5);

    body - front_step_recess - compression_witness_cuts()
        + septum_witness_rings()
        + gasket_witness_rectangles()
        + compression_step_ladder()
        + pressure_film_slots()
}

fn compression_witness_cuts() -> Part {
    let mut cuts = Part::empty("cap_torque_seal_compression_witness_cuts");
    for i in 0..COMPRESSION_WITNESS_COUNT {
        let (x, y) = compression_xy(i);
        cuts = cuts
            + centered_cylinder(
                format!("cap_torque_seal_septum_compression_witness_cut_{i}"),
                SEPTUM_WITNESS_D / 2.0,
                8.0,
                40,
            )
            .translate(x - 12.0, y, COMPRESSION_Z - 3.5)
            + centered_cube(
                format!("cap_torque_seal_gasket_compression_witness_cut_{i}"),
                GASKET_WITNESS_X,
                GASKET_WITNESS_Y,
                8.0,
            )
            .translate(x + 14.0, y, COMPRESSION_Z - 3.5);
    }
    cuts
}

fn septum_witness_rings() -> Part {
    let mut rings = Part::empty("cap_torque_seal_septum_witness_rings");
    for i in 0..COMPRESSION_WITNESS_COUNT {
        let (x, y) = compression_xy(i);
        rings = rings
            + retaining_ring(
                &format!("cap_torque_seal_septum_witness_ring_{i}"),
                SEPTUM_WITNESS_D / 2.0 + 4.0,
                SEPTUM_WITNESS_D / 2.0 - 5.0,
                4.0,
            )
            .translate(x - 12.0, y, COMPRESSION_Z + 2.0);
    }
    rings
}

fn gasket_witness_rectangles() -> Part {
    let mut frames = Part::empty("cap_torque_seal_gasket_witness_rectangles");
    for i in 0..COMPRESSION_WITNESS_COUNT {
        let (x, y) = compression_xy(i);
        frames = frames
            + rectangular_frame(
                &format!("cap_torque_seal_gasket_witness_frame_{i}"),
                GASKET_WITNESS_X + 9.0,
                GASKET_WITNESS_Y + 9.0,
                4.0,
                5.0,
            )
            .translate(x + 14.0, y, COMPRESSION_Z + 2.0);
    }
    frames
}

fn compression_step_ladder() -> Part {
    let mut steps = Part::empty("cap_torque_seal_compression_step_ladder");
    for step in 0..COMPRESSION_STEP_COUNT {
        let squeeze = LOW_COMPRESSION
            + (HIGH_COMPRESSION - LOW_COMPRESSION) * step as f64
                / (COMPRESSION_STEP_COUNT as f64 - 1.0);
        let height = compressed_gasket_height(squeeze) * 8.0;
        steps = steps
            + centered_cube(
                format!("cap_torque_seal_compression_reference_step_{step}"),
                42.0,
                22.0,
                height,
            )
            .translate(
                centered_index(step, COMPRESSION_STEP_COUNT, 50.0),
                -COMPRESSION_Y / 2.0 + 28.0,
                COMPRESSION_Z + height / 2.0,
            );
    }
    steps
}

fn pressure_film_slots() -> Part {
    let mut slots = Part::empty("cap_torque_seal_pressure_film_witness_slots");
    for i in 0..4 {
        slots = slots
            + centered_cube(
                format!("cap_torque_seal_pressure_film_slot_{i}"),
                38.0,
                5.0,
                5.0,
            )
            .translate(
                centered_index(i, 4, 66.0),
                COMPRESSION_Y / 2.0 - 30.0,
                COMPRESSION_Z + 2.5,
            );
    }
    slots
}

fn pressure_decay_leak_port_manifold() -> Part {
    let body = centered_cube(
        "cap_torque_seal_pressure_decay_leak_manifold_body",
        LEAK_X,
        LEAK_Y,
        LEAK_Z,
    )
    .translate(0.0, 0.0, LEAK_Z / 2.0);
    let main_channel = centered_cube(
        "cap_torque_seal_pressure_decay_main_channel_cut",
        LEAK_X - 64.0,
        14.0,
        LEAK_Z + 2.0,
    )
    .translate(0.0, 0.0, LEAK_Z / 2.0);

    body - main_channel - leak_port_cuts()
        + leak_port_collars()
        + reference_volume_bosses()
        + pressure_transducer_docks()
        + decay_trace_channels()
}

fn leak_port_cuts() -> Part {
    let mut cuts = Part::empty("cap_torque_seal_pressure_decay_port_cuts");
    for port in 0..LEAK_PORT_COUNT {
        let (x, y) = leak_xy(port);
        cuts = cuts
            + centered_cylinder(
                format!("cap_torque_seal_pressure_decay_leak_port_cut_{port}"),
                LEAK_PORT_D / 2.0,
                LEAK_Z + 3.0,
                28,
            )
            .translate(x, y, LEAK_Z / 2.0);
    }
    cuts
}

fn leak_port_collars() -> Part {
    let mut collars = Part::empty("cap_torque_seal_pressure_decay_port_collars");
    for port in 0..LEAK_PORT_COUNT {
        let (x, y) = leak_xy(port);
        collars = collars
            + retaining_ring(
                &format!("cap_torque_seal_pressure_decay_port_collar_{port}"),
                LEAK_COLLAR_D / 2.0,
                LEAK_PORT_D / 2.0 + 2.0,
                6.0,
            )
            .translate(x, y, LEAK_Z + 3.0);
    }
    collars
}

fn reference_volume_bosses() -> Part {
    let mut bosses = Part::empty("cap_torque_seal_reference_volume_bosses");
    for i in 0..REFERENCE_VOLUME_COUNT {
        bosses = bosses
            + centered_cylinder(
                format!("cap_torque_seal_reference_volume_boss_{i}"),
                22.0,
                28.0,
                42,
            )
            .translate(
                -LEAK_X / 2.0 + 52.0 + i as f64 * 48.0,
                -LEAK_Y / 2.0 + 34.0,
                LEAK_Z + 14.0,
            )
            - centered_cylinder(
                format!("cap_torque_seal_reference_volume_cavity_{i}"),
                12.0,
                30.0,
                36,
            )
            .translate(
                -LEAK_X / 2.0 + 52.0 + i as f64 * 48.0,
                -LEAK_Y / 2.0 + 34.0,
                LEAK_Z + 15.0,
            );
    }
    bosses
}

fn pressure_transducer_docks() -> Part {
    let mut docks = Part::empty("cap_torque_seal_pressure_transducer_docks");
    for i in 0..PRESSURE_TRANSDUCER_DOCKS {
        docks = docks
            + centered_cube(
                format!("cap_torque_seal_pressure_transducer_dock_{i}"),
                36.0,
                22.0,
                18.0,
            )
            .translate(
                centered_index(i, PRESSURE_TRANSDUCER_DOCKS, 52.0) + 68.0,
                -LEAK_Y / 2.0 + 34.0,
                LEAK_Z + 9.0,
            );
    }
    docks
}

fn decay_trace_channels() -> Part {
    let mut channels = Part::empty("cap_torque_seal_pressure_decay_trace_channels");
    for i in 0..DECAY_TRACE_CHANNELS {
        channels = channels
            + centered_cube(
                format!("cap_torque_seal_decay_trace_channel_{i}"),
                LEAK_X - 82.0,
                4.0,
                4.0,
            )
            .translate(
                0.0,
                centered_index(i, DECAY_TRACE_CHANNELS, 18.0),
                LEAK_Z + 2.0,
            );
    }
    channels
}

fn dye_ingress_wells() -> Part {
    let body = centered_cube(
        "cap_torque_seal_dye_ingress_well_block",
        DYE_X,
        DYE_Y,
        DYE_Z,
    )
    .translate(0.0, 0.0, DYE_Z / 2.0);
    let moat = centered_cube(
        "cap_torque_seal_dye_ingress_overflow_moat_cut",
        DYE_X - 30.0,
        DYE_Y - 30.0,
        7.0,
    )
    .translate(0.0, 0.0, DYE_Z - 3.0);

    body - moat - dye_well_cuts() + dye_well_rims() + dye_overflow_slots() + dye_wick_tabs()
}

fn dye_well_cuts() -> Part {
    let mut cuts = Part::empty("cap_torque_seal_dye_well_cuts");
    for well in 0..DYE_WELL_COUNT {
        let (x, y) = dye_xy(well);
        cuts = cuts
            + centered_cylinder(
                format!("cap_torque_seal_dye_ingress_well_cut_{well}"),
                DYE_WELL_D / 2.0,
                18.0,
                36,
            )
            .translate(x, y, DYE_Z - 8.0);
    }
    cuts
}

fn dye_well_rims() -> Part {
    let mut rims = Part::empty("cap_torque_seal_dye_well_rims");
    for well in 0..DYE_WELL_COUNT {
        let (x, y) = dye_xy(well);
        rims = rims
            + retaining_ring(
                &format!("cap_torque_seal_dye_well_rim_{well}"),
                DYE_WELL_D / 2.0 + 4.0,
                DYE_WELL_D / 2.0 - 1.0,
                5.0,
            )
            .translate(x, y, DYE_Z + 2.5);
    }
    rims
}

fn dye_overflow_slots() -> Part {
    let mut slots = Part::empty("cap_torque_seal_dye_overflow_slots");
    for i in 0..DYE_OVERFLOW_SLOTS {
        let x = centered_index(i, DYE_OVERFLOW_SLOTS, 42.0);
        slots = slots
            + centered_cube(
                format!("cap_torque_seal_dye_overflow_slot_{i}"),
                28.0,
                6.0,
                6.0,
            )
            .translate(x, -DYE_Y / 2.0 + 26.0, DYE_Z + 3.0);
    }
    slots
}

fn dye_wick_tabs() -> Part {
    let mut tabs = Part::empty("cap_torque_seal_dye_wick_tabs");
    for i in 0..DYE_WICK_TABS {
        tabs = tabs
            + centered_cube(format!("cap_torque_seal_dye_wick_tab_{i}"), 18.0, 8.0, 5.0).translate(
                centered_index(i % 4, 4, 46.0),
                DYE_Y / 2.0 - 28.0 + centered_index(i / 4, 2, 16.0),
                DYE_Z + 2.5,
            );
    }
    tabs
}

fn cap_custody_parks() -> Part {
    let body = centered_cube(
        "cap_torque_seal_cap_custody_park_body",
        PARK_X,
        PARK_Y,
        PARK_Z,
    )
    .translate(0.0, 0.0, PARK_Z / 2.0);
    let lane_recess = centered_cube(
        "cap_torque_seal_cap_custody_lane_recess",
        PARK_X - 36.0,
        PARK_Y - 26.0,
        7.0,
    )
    .translate(0.0, 0.0, PARK_Z - 3.0);

    body - lane_recess - cap_park_socket_cuts()
        + cap_park_socket_rims()
        + custody_lane_dividers()
        + custody_token_lands()
}

fn cap_park_socket_cuts() -> Part {
    let mut cuts = Part::empty("cap_torque_seal_cap_custody_socket_cuts");
    for park in 0..CAP_PARK_COUNT {
        let (x, y) = custody_xy(park);
        cuts = cuts
            + centered_cylinder(
                format!("cap_torque_seal_cap_custody_socket_cut_{park}"),
                PARK_SOCKET_D / 2.0,
                18.0,
                40,
            )
            .translate(x, y, PARK_Z - 8.0);
    }
    cuts
}

fn cap_park_socket_rims() -> Part {
    let mut rims = Part::empty("cap_torque_seal_cap_custody_socket_rims");
    for park in 0..CAP_PARK_COUNT {
        let (x, y) = custody_xy(park);
        rims = rims
            + retaining_ring(
                &format!("cap_torque_seal_cap_custody_socket_rim_{park}"),
                PARK_SOCKET_D / 2.0 + 4.0,
                PARK_SOCKET_D / 2.0 - 3.0,
                5.0,
            )
            .translate(x, y, PARK_Z + 2.5);
    }
    rims
}

fn custody_lane_dividers() -> Part {
    let mut dividers = Part::empty("cap_torque_seal_custody_lane_dividers");
    for lane in 1..CUSTODY_LANES {
        let y = -PARK_Y / 2.0 + lane as f64 * PARK_Y / CUSTODY_LANES as f64;
        dividers = dividers
            + centered_cube(
                format!("cap_torque_seal_custody_lane_divider_{lane}"),
                PARK_X - 38.0,
                5.0,
                16.0,
            )
            .translate(0.0, y, PARK_Z + 8.0);
    }
    dividers
}

fn custody_token_lands() -> Part {
    let mut lands = Part::empty("cap_torque_seal_custody_token_lands");
    for token in 0..CUSTODY_TOKEN_COUNT {
        let (x, y) = custody_xy(token);
        lands = lands
            + centered_cube(
                format!("cap_torque_seal_custody_token_land_{token}"),
                24.0,
                7.0,
                5.0,
            )
            .translate(x, y - 28.0, PARK_Z + 2.5);
    }
    lands
}

fn release_hold_reject_lanes() -> Part {
    let body = centered_cube(
        "cap_torque_seal_release_hold_reject_lane_body",
        STATUS_X,
        STATUS_Y,
        STATUS_Z,
    )
    .translate(0.0, 0.0, STATUS_Z / 2.0);
    let lane_recess = centered_cube(
        "cap_torque_seal_release_hold_reject_lane_recess",
        STATUS_X - 32.0,
        STATUS_Y - 24.0,
        8.0,
    )
    .translate(0.0, 0.0, STATUS_Z - 3.5);

    body - lane_recess - disposition_slot_cuts()
        + lane_walls()
        + disposition_flags()
        + disposition_token_stops()
}

fn disposition_slot_cuts() -> Part {
    let mut cuts = Part::empty("cap_torque_seal_disposition_slot_cuts");
    for lane in DispositionLane::all() {
        for slot in 0..SLOTS_PER_DISPOSITION {
            let (x, y) = disposition_xy(lane, slot);
            cuts = cuts
                + centered_cube(
                    format!(
                        "cap_torque_seal_{}_disposition_slot_cut_{slot}",
                        lane.label()
                    ),
                    56.0,
                    20.0,
                    14.0,
                )
                .translate(x, y, STATUS_Z - 6.0);
        }
    }
    cuts
}

fn lane_walls() -> Part {
    let mut walls = Part::empty("cap_torque_seal_disposition_lane_walls");
    for lane in 1..DISPOSITION_LANES {
        let y = -STATUS_Y / 2.0 + lane as f64 * STATUS_Y / DISPOSITION_LANES as f64;
        walls = walls
            + centered_cube(
                format!("cap_torque_seal_disposition_lane_wall_{lane}"),
                STATUS_X - 28.0,
                LANE_WALL_W,
                18.0,
            )
            .translate(0.0, y, STATUS_Z + 9.0);
    }
    walls
}

fn disposition_flags() -> Part {
    let mut flags = Part::empty("cap_torque_seal_disposition_flags");
    for lane in DispositionLane::all() {
        let y = disposition_lane_y(lane);
        let flag_count = match lane {
            DispositionLane::Release => RELEASE_FLAG_COUNT,
            DispositionLane::Hold => HOLD_FLAG_COUNT,
            DispositionLane::Reject => REJECT_FLAG_COUNT,
        };
        for flag in 0..flag_count {
            flags = flags
                + centered_cube(
                    format!("cap_torque_seal_{}_disposition_flag_{flag}", lane.label()),
                    18.0,
                    6.0,
                    28.0,
                )
                .translate(
                    -STATUS_X / 2.0 + 28.0 + flag as f64 * 22.0,
                    y,
                    STATUS_Z + 14.0,
                );
        }
    }
    flags
}

fn disposition_token_stops() -> Part {
    let mut stops = Part::empty("cap_torque_seal_disposition_token_stops");
    for lane in DispositionLane::all() {
        let y = disposition_lane_y(lane);
        stops = stops
            + centered_cube(
                format!("cap_torque_seal_{}_lane_token_stop", lane.label()),
                12.0,
                28.0,
                20.0,
            )
            .translate(STATUS_X / 2.0 - 26.0, y, STATUS_Z + 10.0);
    }
    stops
}

fn evidence_bridge() -> Part {
    let left_post = centered_cube(
        "cap_torque_seal_evidence_bridge_left_post",
        28.0,
        EVIDENCE_Y,
        EVIDENCE_POST_Z,
    )
    .translate(-EVIDENCE_X / 2.0 + 42.0, 0.0, EVIDENCE_POST_Z / 2.0);
    let right_post = centered_cube(
        "cap_torque_seal_evidence_bridge_right_post",
        28.0,
        EVIDENCE_Y,
        EVIDENCE_POST_Z,
    )
    .translate(EVIDENCE_X / 2.0 - 42.0, 0.0, EVIDENCE_POST_Z / 2.0);
    let center_post = centered_cube(
        "cap_torque_seal_evidence_bridge_center_service_post",
        24.0,
        EVIDENCE_Y,
        EVIDENCE_POST_Z - 22.0,
    )
    .translate(0.0, 0.0, (EVIDENCE_POST_Z - 22.0) / 2.0);
    let beam = centered_cube(
        "cap_torque_seal_evidence_bridge_camera_beam",
        EVIDENCE_X,
        EVIDENCE_Y,
        EVIDENCE_BEAM_Z,
    )
    .translate(0.0, 0.0, EVIDENCE_POST_Z + EVIDENCE_BEAM_Z / 2.0);

    left_post
        + right_post
        + center_post
        + beam
        + camera_carriages()
        + evidence_light_bars()
        + evidence_fiducials()
}

fn camera_carriages() -> Part {
    let mut cameras = Part::empty("cap_torque_seal_evidence_camera_carriages");
    for camera in 0..CAMERA_COUNT {
        let x = centered_index(camera, CAMERA_COUNT, 238.0);
        cameras = cameras
            + centered_cube(
                format!("cap_torque_seal_evidence_camera_carriage_{camera}"),
                78.0,
                32.0,
                16.0,
            )
            .translate(x, 0.0, EVIDENCE_POST_Z + EVIDENCE_BEAM_Z + 8.0)
            - centered_cylinder(
                format!("cap_torque_seal_evidence_camera_lens_clearance_{camera}"),
                11.0,
                18.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, EVIDENCE_POST_Z + EVIDENCE_BEAM_Z + 8.0);
    }
    cameras
}

fn evidence_light_bars() -> Part {
    let mut bars = Part::empty("cap_torque_seal_evidence_light_bars");
    for i in 0..LIGHT_BAR_COUNT {
        let y = if i == 0 { -22.0 } else { 22.0 };
        bars = bars
            + centered_cube(
                format!("cap_torque_seal_evidence_light_bar_{i}"),
                EVIDENCE_X - 170.0,
                7.0,
                7.0,
            )
            .translate(0.0, y, EVIDENCE_POST_Z + EVIDENCE_BEAM_Z + 4.0);
    }
    bars
}

fn evidence_fiducials() -> Part {
    let mut fiducials = Part::empty("cap_torque_seal_evidence_fiducials");
    for i in 0..EVIDENCE_FIDUCIAL_COUNT {
        fiducials = fiducials
            + centered_cylinder(
                format!("cap_torque_seal_evidence_fiducial_{i}"),
                4.0,
                3.0,
                20,
            )
            .translate(
                centered_index(i, EVIDENCE_FIDUCIAL_COUNT, 112.0),
                -EVIDENCE_Y / 2.0 - 18.0,
                EVIDENCE_POST_Z + 2.0,
            );
    }
    fiducials
}

fn robot_service_keepout_gauges() -> Part {
    let front = centered_cube(
        "cap_torque_seal_front_robot_keepout_strip",
        KEEP_OUT_X,
        12.0,
        KEEP_OUT_STRIP_Z,
    )
    .translate(0.0, -KEEP_OUT_Y / 2.0, BASE_Z + KEEP_OUT_STRIP_Z / 2.0);
    let rear = centered_cube(
        "cap_torque_seal_rear_service_keepout_strip",
        KEEP_OUT_X,
        12.0,
        KEEP_OUT_STRIP_Z,
    )
    .translate(0.0, KEEP_OUT_Y / 2.0, BASE_Z + KEEP_OUT_STRIP_Z / 2.0);
    let left = centered_cube(
        "cap_torque_seal_left_robot_keepout_strip",
        12.0,
        KEEP_OUT_Y,
        KEEP_OUT_STRIP_Z,
    )
    .translate(-KEEP_OUT_X / 2.0, 0.0, BASE_Z + KEEP_OUT_STRIP_Z / 2.0);
    let right = centered_cube(
        "cap_torque_seal_right_service_keepout_strip",
        12.0,
        KEEP_OUT_Y,
        KEEP_OUT_STRIP_Z,
    )
    .translate(KEEP_OUT_X / 2.0, 0.0, BASE_Z + KEEP_OUT_STRIP_Z / 2.0);
    front + rear + left + right + keepout_height_posts() + service_clearance_combs()
}

fn keepout_height_posts() -> Part {
    let mut posts = Part::empty("cap_torque_seal_keepout_height_posts");
    for i in 0..KEEP_OUT_GAUGE_COUNT {
        let x = -KEEP_OUT_X / 2.0 + 110.0 + i as f64 * ((KEEP_OUT_X - 220.0) / 7.0);
        let y = if i % 2 == 0 {
            -KEEP_OUT_Y / 2.0 + 30.0
        } else {
            KEEP_OUT_Y / 2.0 - 30.0
        };
        let height = 48.0 + i as f64 * 9.0;
        posts = posts
            + centered_cylinder(
                format!("cap_torque_seal_robot_service_keepout_height_post_{i}"),
                8.0,
                height,
                24,
            )
            .translate(x, y, BASE_Z + height / 2.0);
    }
    posts
}

fn service_clearance_combs() -> Part {
    let front = centered_cube(
        "cap_torque_seal_front_robot_clearance_comb",
        170.0,
        10.0,
        32.0,
    )
    .translate(
        -STATION_X / 2.0 + 190.0,
        -KEEP_OUT_Y / 2.0 + FRONT_ROBOT_CLEARANCE_Y,
        BASE_Z + 16.0,
    );
    let rear = centered_cube(
        "cap_torque_seal_rear_service_clearance_comb",
        170.0,
        10.0,
        32.0,
    )
    .translate(
        STATION_X / 2.0 - 190.0,
        KEEP_OUT_Y / 2.0 - REAR_SERVICE_CLEARANCE_Y,
        BASE_Z + 16.0,
    );
    let side = centered_cube(
        "cap_torque_seal_side_pressure_service_clearance_comb",
        10.0,
        170.0,
        32.0,
    )
    .translate(
        KEEP_OUT_X / 2.0 - SIDE_SERVICE_CLEARANCE_X,
        -66.0,
        BASE_Z + 16.0,
    );
    front + rear + side
}

fn retaining_ring(name: &str, outer_radius: f64, inner_radius: f64, height: f64) -> Part {
    let outer = centered_cylinder(format!("{name}_outer"), outer_radius, height, 64);
    let inner = centered_cylinder(format!("{name}_inner_cut"), inner_radius, height + 1.0, 64);
    outer - inner
}

fn rectangular_frame(name: &str, x: f64, y: f64, z: f64, wall: f64) -> Part {
    let outer = centered_cube(format!("{name}_outer"), x, y, z);
    let inner = centered_cube(
        format!("{name}_inner_cut"),
        x - 2.0 * wall,
        y - 2.0 * wall,
        z + 1.0,
    );
    outer - inner
}

fn layout_rects() -> [Rect; 10] {
    [
        rect("capped_vial_bag_port_nests", NEST_CENTER, NEST_X, NEST_Y),
        rect(
            "torque_tool_datum_bridge",
            TORQUE_CENTER,
            TORQUE_X,
            TORQUE_Y,
        ),
        rect("torque_witness_dials", DIAL_CENTER, DIAL_X, DIAL_Y),
        rect(
            "cap_presence_orientation_sensor_bank",
            SENSOR_CENTER,
            SENSOR_X,
            SENSOR_Y,
        ),
        rect(
            "septum_gasket_compression_witness_bank",
            COMPRESSION_CENTER,
            COMPRESSION_X,
            COMPRESSION_Y,
        ),
        rect(
            "pressure_decay_leak_port_manifold",
            LEAK_CENTER,
            LEAK_X,
            LEAK_Y,
        ),
        rect("dye_ingress_wells", DYE_CENTER, DYE_X, DYE_Y),
        rect("cap_custody_parks", PARK_CENTER, PARK_X, PARK_Y),
        rect(
            "release_hold_reject_lanes",
            STATUS_CENTER,
            STATUS_X,
            STATUS_Y,
        ),
        rect("evidence_bridge", EVIDENCE_CENTER, EVIDENCE_X, EVIDENCE_Y),
    ]
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 13, "stable STL output count changed");
    assert_eq!(REQUIRED_FEATURES.len(), 11);
    assert_eq!(VIAL_COUNT, VIAL_ROWS * VIAL_COLS);
    assert_eq!(CAP_COUNT, VIAL_COUNT + BAG_PORT_COUNT);
    assert_eq!(ORIENTATION_KEY_COUNT, CAP_COUNT);
    assert_eq!(TORQUE_DIAL_COUNT, DIAL_ROWS * DIAL_COLS);
    assert_eq!(TORQUE_DIAL_COUNT, CAP_COUNT);
    assert_eq!(TORQUE_WITNESS_POINTERS, TORQUE_DIAL_COUNT);
    assert_eq!(SENSOR_COUNT, SENSOR_ROWS * SENSOR_COLS);
    assert_eq!(SENSOR_COUNT, CAP_COUNT);
    assert_eq!(COMPRESSION_WITNESS_COUNT, CAP_COUNT);
    assert_eq!(LEAK_PORT_COUNT, CAP_COUNT);
    assert_eq!(DYE_WELL_COUNT, CAP_COUNT);
    assert_eq!(CAP_PARK_COUNT, CAP_COUNT);
    assert_eq!(CUSTODY_LANES * CUSTODY_SLOTS_PER_LANE, CAP_PARK_COUNT);
    assert_eq!(DISPOSITION_SLOT_COUNT, CAP_COUNT);
    assert_eq!(DISPOSITION_LANES, DispositionLane::all().len());
    assert_eq!(
        RELEASE_FLAG_COUNT + HOLD_FLAG_COUNT + REJECT_FLAG_COUNT,
        CAP_COUNT
    );
    assert_eq!(NEST_DATUM_PIN_COUNT, 4);
    assert_eq!(TORQUE_DATUM_PIN_COUNT, 4);
    assert_eq!(MOUNT_HOLE_COUNT, mount_points().len());
    assert!(LOW_COMPRESSION < NOMINAL_COMPRESSION);
    assert!(NOMINAL_COMPRESSION < HIGH_COMPRESSION);
    assert!(compressed_gasket_height(NOMINAL_COMPRESSION) > 0.0);
    assert!(LEAK_COLLAR_D > LEAK_PORT_D + 12.0);
    assert!(TORQUE_TOOL_CLEARANCE_Z > TORQUE_POST_Z + BASE_Z);
    assert!(ROBOT_PICK_CLEARANCE_Z > EVIDENCE_POST_Z);
    assert!(PRESSURE_SERVICE_CLEARANCE_Z > LEAK_Z + BASE_Z + 140.0);
    assert!(CAMERA_CLEARANCE_Z > NEST_Z + BASE_Z + 80.0);

    let rects = layout_rects();
    for rect in rects {
        assert!(
            rect.fits_inside_station(),
            "{} exceeds station footprint",
            rect.name
        );
    }

    for (i, left) in rects.iter().enumerate() {
        for right in rects.iter().skip(i + 1) {
            assert!(
                left.clears(*right, MODULE_GAP),
                "{} overlaps {}",
                left.name,
                right.name
            );
        }
    }
}

fn vial_xy(row: usize, col: usize) -> (f64, f64) {
    (
        centered_index(col, VIAL_COLS, VIAL_PITCH_X),
        centered_index(row, VIAL_ROWS, VIAL_PITCH_Y) + VIAL_ROW_OFFSET_Y,
    )
}

fn bag_port_xy(index: usize) -> (f64, f64) {
    (
        centered_index(index, BAG_PORT_COUNT, BAG_PORT_PITCH_X),
        BAG_PORT_Y,
    )
}

fn cap_xy(index: usize) -> (f64, f64) {
    if index < VIAL_COUNT {
        vial_xy(index / VIAL_COLS, index % VIAL_COLS)
    } else {
        bag_port_xy(index - VIAL_COUNT)
    }
}

fn vial_index(row: usize, col: usize) -> usize {
    row * VIAL_COLS + col
}

fn dial_xy(index: usize) -> (f64, f64) {
    (
        centered_index(index % DIAL_COLS, DIAL_COLS, DIAL_PITCH_X),
        centered_index(index / DIAL_COLS, DIAL_ROWS, DIAL_PITCH_Y),
    )
}

fn sensor_xy(index: usize) -> (f64, f64) {
    (
        centered_index(index % SENSOR_COLS, SENSOR_COLS, SENSOR_PITCH_X),
        centered_index(index / SENSOR_COLS, SENSOR_ROWS, SENSOR_PITCH_Y),
    )
}

fn compression_xy(index: usize) -> (f64, f64) {
    (
        centered_index(
            index % COMPRESSION_COLS,
            COMPRESSION_COLS,
            COMPRESSION_PITCH_X,
        ),
        centered_index(
            index / COMPRESSION_COLS,
            COMPRESSION_ROWS,
            COMPRESSION_PITCH_Y,
        ) + 16.0,
    )
}

fn leak_xy(index: usize) -> (f64, f64) {
    (
        centered_index(index % LEAK_COLS, LEAK_COLS, LEAK_PITCH_X),
        centered_index(index / LEAK_COLS, LEAK_ROWS, LEAK_PITCH_Y) + 14.0,
    )
}

fn dye_xy(index: usize) -> (f64, f64) {
    (
        centered_index(index % DYE_COLS, DYE_COLS, DYE_PITCH_X),
        centered_index(index / DYE_COLS, DYE_ROWS, DYE_PITCH_Y),
    )
}

fn custody_xy(index: usize) -> (f64, f64) {
    let lane = index / CUSTODY_SLOTS_PER_LANE;
    let slot = index % CUSTODY_SLOTS_PER_LANE;
    (
        centered_index(slot, CUSTODY_SLOTS_PER_LANE, 84.0),
        centered_index(lane, CUSTODY_LANES, 35.0),
    )
}

fn disposition_xy(lane: DispositionLane, slot: usize) -> (f64, f64) {
    (
        centered_index(slot, SLOTS_PER_DISPOSITION, 72.0),
        disposition_lane_y(lane),
    )
}

fn disposition_lane_y(lane: DispositionLane) -> f64 {
    centered_index(lane.index(), DISPOSITION_LANES, 36.0)
}

fn compressed_gasket_height(squeeze: f64) -> f64 {
    NOMINAL_GASKET_THICKNESS * (1.0 - squeeze)
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 13);

        let scoped_prefix = format!("output/{OUTPUT_PREFIX}_");
        for path in OUTPUTS {
            assert!(
                path.starts_with(scoped_prefix.as_str()),
                "unscoped output path: {path}"
            );
            assert!(path.ends_with(".stl"), "{path}");
        }
        assert_eq!(
            OUTPUTS[OUTPUTS.len() - 1],
            format!("output/{OUTPUT_PREFIX}_assembly.stl")
        );
    }

    #[test]
    fn required_features_cover_requested_station_scope() {
        for feature in [
            "capped_vial_bag_port_nests",
            "torque_tool_datum_bridge",
            "torque_witness_dials",
            "cap_presence_orientation_sensor_bank",
            "septum_gasket_compression_witness_bank",
            "pressure_decay_leak_port_manifold",
            "dye_ingress_wells",
            "cap_custody_parks",
            "release_hold_reject_lanes",
            "evidence_bridge",
            "robot_service_keepout_gauges",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn cap_format_and_integrity_counts_match() {
        assert_eq!(VIAL_COUNT, 8);
        assert_eq!(BAG_PORT_COUNT, 4);
        assert_eq!(CAP_COUNT, 12);
        assert_eq!(TORQUE_DIAL_COUNT, CAP_COUNT);
        assert_eq!(SENSOR_COUNT, CAP_COUNT);
        assert_eq!(COMPRESSION_WITNESS_COUNT, CAP_COUNT);
        assert_eq!(LEAK_PORT_COUNT, CAP_COUNT);
        assert_eq!(DYE_WELL_COUNT, CAP_COUNT);
        assert_eq!(CAP_PARK_COUNT, CAP_COUNT);
    }

    #[test]
    fn layout_is_bounded_and_zones_do_not_overlap() {
        assert_design_constraints();
        assert_eq!(layout_rects().len(), 10);
    }

    #[test]
    fn torque_sensor_and_compression_references_are_explicit() {
        assert_eq!(TORQUE_DATUM_PIN_COUNT, 4);
        assert_eq!(TORQUE_REACTION_DOG_COUNT, 8);
        assert_eq!(DIAL_TICKS_PER_CAP, 5);
        assert_eq!(ORIENTATION_FORKS, CAP_COUNT);
        assert!(LOW_COMPRESSION < NOMINAL_COMPRESSION);
        assert!(NOMINAL_COMPRESSION < HIGH_COMPRESSION);
        assert!(
            compressed_gasket_height(LOW_COMPRESSION)
                > compressed_gasket_height(NOMINAL_COMPRESSION)
        );
        assert!(
            compressed_gasket_height(NOMINAL_COMPRESSION)
                > compressed_gasket_height(HIGH_COMPRESSION)
        );
    }

    #[test]
    fn custody_and_disposition_cover_all_caps() {
        assert_eq!(CUSTODY_LANES, 3);
        assert_eq!(CUSTODY_SLOTS_PER_LANE, 4);
        assert_eq!(DISPOSITION_LANES, 3);
        assert_eq!(SLOTS_PER_DISPOSITION, 4);
        assert_eq!(DISPOSITION_SLOT_COUNT, CAP_COUNT);
        assert_eq!(
            RELEASE_FLAG_COUNT + HOLD_FLAG_COUNT + REJECT_FLAG_COUNT,
            CAP_COUNT
        );
    }

    #[test]
    fn evidence_and_keepout_clearances_are_visible() {
        assert_eq!(CAMERA_COUNT, 5);
        assert_eq!(LIGHT_BAR_COUNT, 2);
        assert_eq!(KEEP_OUT_GAUGE_COUNT, 8);
        assert!(EVIDENCE_X < STATION_X);
        assert!(EVIDENCE_CENTER.1 + EVIDENCE_Y / 2.0 < STATION_Y / 2.0 - RIM_W);
        assert!(TORQUE_TOOL_CLEARANCE_Z > ROBOT_PICK_CLEARANCE_Z);
        assert!(ROBOT_PICK_CLEARANCE_Z > EVIDENCE_POST_Z);
    }
}
