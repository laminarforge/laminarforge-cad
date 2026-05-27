use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed transfer airlock VHP residue and aeration clearance witness station.
//
// Intent:
// - Hold residue coupons, door-gasket witness lands, peroxide sensor pockets,
//   purge-flow vanes, aeration timer tokens, traceability lands, and disposition
//   lanes around a closed transfer-airlock validation article.
// - Make clean/dirty segregation and labels visible as CSG geometry for review,
//   fixture planning, and artifact traceability.
// - Keep VHP generation, analytical acceptance limits, biological release, and
//   operating procedures outside this mechanical CAD generator.

const OUTPUT_PREFIX: &str = "output/closed_airlock_vhp_residue_aeration_witness_station_";

const OUTPUTS: [&str; 13] = [
    "output/closed_airlock_vhp_residue_aeration_witness_station_base_validation_deck.stl",
    "output/closed_airlock_vhp_residue_aeration_witness_station_closed_transfer_airlock_reference.stl",
    "output/closed_airlock_vhp_residue_aeration_witness_station_residue_coupon_racks.stl",
    "output/closed_airlock_vhp_residue_aeration_witness_station_aeration_timer_token_lanes.stl",
    "output/closed_airlock_vhp_residue_aeration_witness_station_peroxide_sensor_pockets.stl",
    "output/closed_airlock_vhp_residue_aeration_witness_station_purge_flow_witness_vanes.stl",
    "output/closed_airlock_vhp_residue_aeration_witness_station_door_gasket_coupon_lands.stl",
    "output/closed_airlock_vhp_residue_aeration_witness_station_clean_dirty_segregation_gate.stl",
    "output/closed_airlock_vhp_residue_aeration_witness_station_barcode_certificate_lands.stl",
    "output/closed_airlock_vhp_residue_aeration_witness_station_release_hold_reject_lanes.stl",
    "output/closed_airlock_vhp_residue_aeration_witness_station_evidence_camera_bridge.stl",
    "output/closed_airlock_vhp_residue_aeration_witness_station_robot_service_keepout_gauges.stl",
    "output/closed_airlock_vhp_residue_aeration_witness_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "closed_transfer_airlock_reference",
    "residue_coupon_racks",
    "aeration_timer_token_lanes",
    "peroxide_sensor_pockets",
    "purge_flow_witness_vanes",
    "door_gasket_coupon_lands",
    "clean_dirty_segregation",
    "barcode_certificate_lands",
    "release_hold_reject_lanes",
    "labels_as_csg_geometry",
    "robot_service_keepouts",
];

const DECK_X: f64 = 1540.0;
const DECK_Y: f64 = 980.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const SOCKET_DEPTH: f64 = 6.0;
const MOUNT_HOLE_D: f64 = 6.8;

const AIRLOCK_POS: (f64, f64) = (-430.0, 170.0);
const AIRLOCK_X: f64 = 520.0;
const AIRLOCK_Y: f64 = 300.0;
const AIRLOCK_PLINTH_Z: f64 = 42.0;
const AIRLOCK_FRAME_Z: f64 = 52.0;
const AIRLOCK_DOOR_X: f64 = 206.0;
const AIRLOCK_DOOR_Y: f64 = 226.0;
const AIRLOCK_DOOR_GAP_X: f64 = 238.0;
const AIRLOCK_GASKET_BAR: f64 = 10.0;
const AIRLOCK_PORT_COUNT: usize = 4;
const AIRLOCK_LATCH_COUNT: usize = 6;

const RESIDUE_POS: (f64, f64) = (200.0, 200.0);
const RESIDUE_X: f64 = 420.0;
const RESIDUE_Y: f64 = 270.0;
const RESIDUE_Z: f64 = 44.0;
const RESIDUE_ROWS: usize = 3;
const RESIDUE_COLS: usize = 6;
const RESIDUE_COUPON_COUNT: usize = RESIDUE_ROWS * RESIDUE_COLS;
const RESIDUE_PITCH_X: f64 = 58.0;
const RESIDUE_PITCH_Y: f64 = 70.0;
const RESIDUE_COUPON_D: f64 = 34.0;
const RESIDUE_MATERIALS: [&str; RESIDUE_ROWS] = ["stainless", "silicone", "polycarbonate"];

const SENSOR_POS: (f64, f64) = (595.0, 200.0);
const SENSOR_X: f64 = 220.0;
const SENSOR_Y: f64 = 270.0;
const SENSOR_Z: f64 = 62.0;
const SENSOR_POCKET_COUNT: usize = 6;
const SENSOR_POCKET_PITCH_Y: f64 = 38.0;
const SENSOR_SAMPLE_PORT_COUNT: usize = 4;
const SENSOR_CALIBRATION_WELLS: usize = 3;

const PURGE_POS: (f64, f64) = (-420.0, -135.0);
const PURGE_X: f64 = 520.0;
const PURGE_Y: f64 = 220.0;
const PURGE_Z: f64 = 52.0;
const PURGE_VANE_COUNT: usize = 9;
const PURGE_VANE_PITCH_X: f64 = 44.0;
const PURGE_VANE_Z: f64 = 68.0;
const PURGE_PORT_COUNT: usize = 3;

const GASKET_POS: (f64, f64) = (150.0, -125.0);
const GASKET_X: f64 = 420.0;
const GASKET_Y: f64 = 220.0;
const GASKET_Z: f64 = 38.0;
const GASKET_DOOR_COUNT: usize = 2;
const GASKET_COUPONS_PER_DOOR: usize = 6;
const GASKET_COUPON_COUNT: usize = GASKET_DOOR_COUNT * GASKET_COUPONS_PER_DOOR;
const GASKET_COMPRESSION_STEPS: usize = 5;

const TIMER_POS: (f64, f64) = (540.0, -125.0);
const TIMER_X: f64 = 300.0;
const TIMER_Y: f64 = 220.0;
const TIMER_Z: f64 = 34.0;
const TIMER_LANE_COUNT: usize = 5;
const TIMER_MINUTES: [usize; TIMER_LANE_COUNT] = [0, 15, 30, 60, 120];
const TOKENS_PER_TIMER_LANE: usize = 4;
const TIMER_TOKEN_COUNT: usize = TIMER_LANE_COUNT * TOKENS_PER_TIMER_LANE;
const TIMER_LANE_PITCH_X: f64 = 52.0;
const TIMER_TOKEN_PITCH_Y: f64 = 40.0;

const SEGREGATION_POS: (f64, f64) = (-510.0, -360.0);
const SEGREGATION_X: f64 = 360.0;
const SEGREGATION_Y: f64 = 150.0;
const SEGREGATION_Z: f64 = 32.0;
const CLEAN_DIRTY_DIVIDER_Z: f64 = 118.0;
const SEGREGATION_BIN_COUNT: usize = 4;

const TRACE_POS: (f64, f64) = (-80.0, -360.0);
const TRACE_X: f64 = 430.0;
const TRACE_Y: f64 = 150.0;
const TRACE_Z: f64 = 18.0;
const BARCODE_LANDS: usize = 10;
const CERTIFICATE_LANDS: usize = 4;
const TRACE_LABEL_BARS: usize = 48;

const DISPOSITION_POS: (f64, f64) = (410.0, -360.0);
const DISPOSITION_X: f64 = 520.0;
const DISPOSITION_Y: f64 = 150.0;
const DISPOSITION_Z: f64 = 26.0;
const DISPOSITION_LANE_COUNT: usize = 3;
const DISPOSITION_SLOTS_PER_LANE: usize = 5;
const DISPOSITION_LANE_PITCH_X: f64 = 154.0;

const BRIDGE_POS: (f64, f64) = (0.0, 405.0);
const BRIDGE_SPAN_X: f64 = 1260.0;
const BRIDGE_POST_Y: f64 = 42.0;
const BRIDGE_UNDERSIDE_Z: f64 = 250.0;
const CAMERA_COUNT: usize = 4;
const LIGHT_BAR_COUNT: usize = 8;

const KEEP_OUT_X: f64 = 1450.0;
const KEEP_OUT_Y: f64 = 900.0;
const KEEP_OUT_Z: f64 = 8.0;
const FRONT_ROBOT_APPROACH_MM: f64 = 360.0;
const REAR_SERVICE_MM: f64 = 300.0;
const SIDE_SENSOR_SERVICE_MM: f64 = 220.0;
const TOP_AERATION_LIFT_MM: f64 = 390.0;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DispositionLane {
    Release,
    Hold,
    Reject,
}

impl DispositionLane {
    fn all() -> [DispositionLane; DISPOSITION_LANE_COUNT] {
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

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_deck(self) -> bool {
        let half_x = DECK_X / 2.0 - RIM_W - 12.0;
        let half_y = DECK_Y / 2.0 - RIM_W - 12.0;

        self.center.0.abs() + self.x / 2.0 <= half_x && self.center.1.abs() + self.y / 2.0 <= half_y
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

    let base = base_validation_deck();
    export(OUTPUTS[0], &base);

    let airlock = closed_transfer_airlock_reference();
    export(OUTPUTS[1], &airlock);

    let residue = residue_coupon_racks();
    export(OUTPUTS[2], &residue);

    let timers = aeration_timer_token_lanes();
    export(OUTPUTS[3], &timers);

    let sensors = peroxide_sensor_pockets();
    export(OUTPUTS[4], &sensors);

    let vanes = purge_flow_witness_vanes();
    export(OUTPUTS[5], &vanes);

    let gaskets = door_gasket_coupon_lands();
    export(OUTPUTS[6], &gaskets);

    let segregation = clean_dirty_segregation_gate();
    export(OUTPUTS[7], &segregation);

    let traceability = barcode_certificate_lands();
    export(OUTPUTS[8], &traceability);

    let disposition = release_hold_reject_lanes();
    export(OUTPUTS[9], &disposition);

    let bridge = evidence_camera_bridge();
    export(OUTPUTS[10], &bridge);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[11], &keepouts);

    let assembly = base
        + airlock
        + residue
        + timers
        + sensors
        + vanes
        + gaskets
        + segregation
        + traceability
        + disposition
        + bridge
        + keepouts;
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed airlock VHP residue/aeration witness station:");
    println!("  Footprint:              {DECK_X:.0}mm x {DECK_Y:.0}mm validation deck");
    println!(
        "  Airlock reference:      inner/outer closed-door frames, {AIRLOCK_PORT_COUNT} VHP/aeration ports, {AIRLOCK_LATCH_COUNT} latch witness blocks"
    );
    println!(
        "  Residue coupons:        {RESIDUE_COUPON_COUNT} coupon rack positions across {} material rows",
        RESIDUE_MATERIALS.len()
    );
    println!(
        "  Aeration clearance:     {TIMER_TOKEN_COUNT} timer token pockets, {SENSOR_POCKET_COUNT} peroxide sensor pockets, {PURGE_VANE_COUNT} purge-flow witness vanes"
    );
    println!(
        "  Door gasket witnesses:  {GASKET_COUPON_COUNT} gasket coupon lands with {GASKET_COMPRESSION_STEPS} compression gauge steps"
    );
    println!(
        "  Traceability workflow:  {BARCODE_LANDS} barcode lands, {CERTIFICATE_LANDS} certificate lands, {TRACE_LABEL_BARS} CSG label bars, {DISPOSITION_LANE_COUNT} release/hold/reject lanes"
    );
    println!(
        "  Keepouts:               front robot {FRONT_ROBOT_APPROACH_MM:.0}mm, rear service {REAR_SERVICE_MM:.0}mm, side sensor {SIDE_SENSOR_SERVICE_MM:.0}mm, top aeration lift {TOP_AERATION_LIFT_MM:.0}mm"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn base_validation_deck() -> Part {
    let deck = centered_cube(name("base_validation_deck"), DECK_X, DECK_Y, DECK_Z);
    let sockets = zone_socket(
        "closed_airlock_reference_socket",
        AIRLOCK_POS,
        AIRLOCK_X,
        AIRLOCK_Y,
    ) + zone_socket(
        "residue_coupon_rack_socket",
        RESIDUE_POS,
        RESIDUE_X,
        RESIDUE_Y,
    ) + zone_socket("peroxide_sensor_socket", SENSOR_POS, SENSOR_X, SENSOR_Y)
        + zone_socket("purge_flow_vane_socket", PURGE_POS, PURGE_X, PURGE_Y)
        + zone_socket("door_gasket_coupon_socket", GASKET_POS, GASKET_X, GASKET_Y)
        + zone_socket("aeration_timer_token_socket", TIMER_POS, TIMER_X, TIMER_Y)
        + zone_socket(
            "clean_dirty_segregation_socket",
            SEGREGATION_POS,
            SEGREGATION_X,
            SEGREGATION_Y,
        )
        + zone_socket("traceability_socket", TRACE_POS, TRACE_X, TRACE_Y)
        + zone_socket(
            "release_hold_reject_socket",
            DISPOSITION_POS,
            DISPOSITION_X,
            DISPOSITION_Y,
        );

    deck - sockets - deck_mounting_slots() - deck_wipe_gutter_cuts()
        + perimeter_lips()
        + deck_datum_fiducials()
        + deck_zone_label_lands()
}

fn zone_socket(label: &str, center: (f64, f64), x: f64, y: f64) -> Part {
    centered_cube(name(label), x + 18.0, y + 18.0, SOCKET_DEPTH + 0.2).translate(
        center.0,
        center.1,
        DECK_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.1,
    )
}

fn deck_mounting_slots() -> Part {
    let mut slots = Part::empty(name("deck_mounting_slots"));
    for (i, (x, y)) in deck_mount_points().iter().enumerate() {
        let round = centered_cylinder(
            name(&format!("deck_m6_mount_round_{i}")),
            MOUNT_HOLE_D / 2.0,
            DECK_Z + 8.0,
            28,
        )
        .translate(*x, *y, 0.0);
        let slot = centered_cube(
            name(&format!("deck_m6_mount_slot_{i}")),
            28.0,
            MOUNT_HOLE_D + 1.0,
            DECK_Z + 8.0,
        )
        .translate(*x, *y, 0.0);
        slots = slots + round + slot;
    }
    slots
}

fn deck_mount_points() -> [(f64, f64); 12] {
    [
        (-DECK_X / 2.0 + 70.0, -DECK_Y / 2.0 + 68.0),
        (DECK_X / 2.0 - 70.0, -DECK_Y / 2.0 + 68.0),
        (-DECK_X / 2.0 + 70.0, DECK_Y / 2.0 - 68.0),
        (DECK_X / 2.0 - 70.0, DECK_Y / 2.0 - 68.0),
        (AIRLOCK_POS.0 - AIRLOCK_X / 2.0 + 52.0, AIRLOCK_POS.1),
        (AIRLOCK_POS.0 + AIRLOCK_X / 2.0 - 52.0, AIRLOCK_POS.1),
        (RESIDUE_POS.0 - RESIDUE_X / 2.0 + 48.0, RESIDUE_POS.1),
        (SENSOR_POS.0, SENSOR_POS.1 + SENSOR_Y / 2.0 - 40.0),
        (PURGE_POS.0 - PURGE_X / 2.0 + 54.0, PURGE_POS.1),
        (PURGE_POS.0 + PURGE_X / 2.0 - 54.0, PURGE_POS.1),
        (TRACE_POS.0, TRACE_POS.1),
        (DISPOSITION_POS.0, DISPOSITION_POS.1),
    ]
}

fn deck_wipe_gutter_cuts() -> Part {
    let front_gutter = centered_cube(
        name("front_vhp_residue_wipe_gutter_cut"),
        DECK_X - 250.0,
        10.0,
        7.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 72.0, DECK_Z / 2.0 - 2.8);
    let left_gutter = centered_cube(
        name("left_dirty_side_wipe_gutter_cut"),
        10.0,
        DECK_Y - 230.0,
        7.0,
    )
    .translate(-DECK_X / 2.0 + 78.0, -18.0, DECK_Z / 2.0 - 2.8);
    let peroxide_sump = centered_cube(name("peroxide_sensor_sump_shadow_cut"), 64.0, 28.0, 8.0)
        .translate(
            SENSOR_POS.0,
            SENSOR_POS.1 - SENSOR_Y / 2.0 + 26.0,
            DECK_Z / 2.0 - 3.0,
        );
    let drain = centered_cylinder(name("front_wipe_gutter_drain_cut"), 7.0, DECK_Z + 8.0, 28)
        .translate(DECK_X / 2.0 - 86.0, -DECK_Y / 2.0 + 72.0, 0.0);

    front_gutter + left_gutter + peroxide_sump + drain
}

fn perimeter_lips() -> Part {
    let rear = centered_cube(
        name("rear_clean_side_tall_lip"),
        DECK_X - 140.0,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, DECK_Y / 2.0 - 38.0, deck_top_z() + RIM_Z / 2.0);
    let left = centered_cube(name("left_dirty_side_lip"), RIM_W, DECK_Y - 150.0, RIM_Z).translate(
        -DECK_X / 2.0 + 38.0,
        -12.0,
        deck_top_z() + RIM_Z / 2.0,
    );
    let right = centered_cube(
        name("right_sensor_service_low_lip"),
        RIM_W,
        DECK_Y - 230.0,
        28.0,
    )
    .translate(DECK_X / 2.0 - 38.0, -34.0, deck_top_z() + 14.0);
    let front = centered_cube(
        name("front_robot_access_low_lip"),
        DECK_X - 260.0,
        12.0,
        18.0,
    )
    .translate(0.0, -DECK_Y / 2.0 + 36.0, deck_top_z() + 9.0);

    rear + left + right + front
}

fn deck_datum_fiducials() -> Part {
    let mut fiducials = Part::empty(name("deck_datum_fiducials"));
    for (i, (x, y)) in [
        (-DECK_X / 2.0 + 104.0, -DECK_Y / 2.0 + 106.0),
        (DECK_X / 2.0 - 104.0, -DECK_Y / 2.0 + 106.0),
        (-DECK_X / 2.0 + 104.0, DECK_Y / 2.0 - 106.0),
        (DECK_X / 2.0 - 104.0, DECK_Y / 2.0 - 106.0),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + datum_disc(&format!("deck_robot_fiducial_{i}")).translate(*x, *y, deck_top_z());
    }
    fiducials
}

fn deck_zone_label_lands() -> Part {
    raised_label_land("deck_closed_airlock_zone_label", 170.0, 22.0, 0).translate(
        AIRLOCK_POS.0,
        AIRLOCK_POS.1 + AIRLOCK_Y / 2.0 - 28.0,
        deck_top_z(),
    ) + raised_label_land("deck_residue_coupon_zone_label", 170.0, 22.0, 1).translate(
        RESIDUE_POS.0,
        RESIDUE_POS.1 + RESIDUE_Y / 2.0 - 28.0,
        deck_top_z(),
    ) + raised_label_land("deck_aeration_clearance_zone_label", 178.0, 22.0, 2).translate(
        TIMER_POS.0,
        TIMER_POS.1 + TIMER_Y / 2.0 - 28.0,
        deck_top_z(),
    ) + raised_label_land("deck_release_hold_reject_zone_label", 194.0, 22.0, 3).translate(
        DISPOSITION_POS.0,
        DISPOSITION_POS.1 + DISPOSITION_Y / 2.0 - 24.0,
        deck_top_z(),
    )
}

fn closed_transfer_airlock_reference() -> Part {
    let plinth = centered_cube(
        name("closed_transfer_airlock_reference_plinth"),
        AIRLOCK_X,
        AIRLOCK_Y,
        AIRLOCK_PLINTH_Z,
    )
    .translate(0.0, 0.0, AIRLOCK_PLINTH_Z / 2.0);
    let basin_recess = centered_cube(
        name("closed_transfer_airlock_floor_residue_basin_recess_cut"),
        AIRLOCK_X - 78.0,
        AIRLOCK_Y - 64.0,
        10.0,
    )
    .translate(0.0, 0.0, AIRLOCK_PLINTH_Z / 2.0 - 5.0);
    let door_frames = airlock_door_frame("inner", -AIRLOCK_DOOR_GAP_X / 2.0)
        + airlock_door_frame("outer", AIRLOCK_DOOR_GAP_X / 2.0);
    let center_bulkhead = centered_cube(
        name("closed_transfer_airlock_center_bulkhead_closed_door_shadow"),
        20.0,
        AIRLOCK_Y - 44.0,
        AIRLOCK_FRAME_Z,
    )
    .translate(0.0, 0.0, AIRLOCK_PLINTH_Z + AIRLOCK_FRAME_Z / 2.0);
    let side_rails = centered_cube(
        name("closed_transfer_airlock_left_side_seal_rail"),
        AIRLOCK_X - 70.0,
        14.0,
        24.0,
    )
    .translate(0.0, -AIRLOCK_Y / 2.0 + 36.0, AIRLOCK_PLINTH_Z + 12.0)
        + centered_cube(
            name("closed_transfer_airlock_right_side_seal_rail"),
            AIRLOCK_X - 70.0,
            14.0,
            24.0,
        )
        .translate(0.0, AIRLOCK_Y / 2.0 - 36.0, AIRLOCK_PLINTH_Z + 12.0);
    let ports = airlock_vhp_aeration_ports();
    let latches = airlock_latch_witness_blocks();
    let route = closed_airlock_route_arrows();

    (plinth - basin_recess + door_frames + center_bulkhead + side_rails + ports + latches + route)
        .translate(AIRLOCK_POS.0, AIRLOCK_POS.1, deck_top_z())
}

fn airlock_door_frame(label: &str, x_offset: f64) -> Part {
    let door_plate = centered_cube(
        name(&format!("{label}_closed_door_surrogate_plate")),
        AIRLOCK_DOOR_X,
        AIRLOCK_DOOR_Y,
        18.0,
    )
    .translate(x_offset, 0.0, AIRLOCK_PLINTH_Z + 9.0);
    let gasket_frame = rect_frame(
        &format!("{label}_raised_airlock_door_gasket_path"),
        AIRLOCK_DOOR_X - 34.0,
        AIRLOCK_DOOR_Y - 34.0,
        AIRLOCK_GASKET_BAR,
        8.0,
    )
    .translate(x_offset, 0.0, AIRLOCK_PLINTH_Z + 24.0);
    let window_land = centered_cube(
        name(&format!("{label}_door_evidence_window_land")),
        72.0,
        52.0,
        4.0,
    )
    .translate(x_offset - 10.0, 0.0, AIRLOCK_PLINTH_Z + 30.0);
    let handle = centered_cube(
        name(&format!("{label}_closed_door_handle_block")),
        18.0,
        82.0,
        18.0,
    )
    .translate(
        x_offset + AIRLOCK_DOOR_X / 2.0 - 32.0,
        0.0,
        AIRLOCK_PLINTH_Z + 39.0,
    );

    door_plate + gasket_frame + window_land + handle
}

fn airlock_vhp_aeration_ports() -> Part {
    let mut ports = Part::empty(name("airlock_vhp_aeration_port_lands"));
    for i in 0..AIRLOCK_PORT_COUNT {
        let x = centered_index(i, AIRLOCK_PORT_COUNT, 92.0);
        let y = if i % 2 == 0 {
            -AIRLOCK_Y / 2.0 + 58.0
        } else {
            AIRLOCK_Y / 2.0 - 58.0
        };
        let boss = centered_cylinder(
            name(&format!("airlock_vhp_aeration_port_boss_{i}")),
            18.0,
            16.0,
            36,
        )
        .translate(x, y, AIRLOCK_PLINTH_Z + 42.0);
        let bore = centered_cylinder(
            name(&format!("airlock_vhp_aeration_port_bore_{i}")),
            8.0,
            18.0,
            28,
        )
        .translate(x, y, AIRLOCK_PLINTH_Z + 42.0);
        let label = raised_label_land(&format!("airlock_port_{i}_csg_label"), 42.0, 12.0, i)
            .translate(
                x,
                y + if y < 0.0 { 26.0 } else { -26.0 },
                AIRLOCK_PLINTH_Z + 50.0,
            );
        ports = ports + (boss - bore) + label;
    }
    ports
}

fn airlock_latch_witness_blocks() -> Part {
    let mut blocks = Part::empty(name("airlock_latch_witness_blocks"));
    for i in 0..AIRLOCK_LATCH_COUNT {
        let side = if i % 2 == 0 { -1.0 } else { 1.0 };
        let row = i / 2;
        let x = side * (AIRLOCK_X / 2.0 - 52.0);
        let y = centered_index(row, AIRLOCK_LATCH_COUNT / 2, 74.0);
        let block = centered_cube(
            name(&format!("airlock_latch_witness_block_{i}")),
            36.0,
            20.0,
            24.0,
        )
        .translate(x, y, AIRLOCK_PLINTH_Z + 38.0);
        let slot = centered_cube(
            name(&format!("airlock_latch_witness_keeper_cut_{i}")),
            16.0,
            8.0,
            26.0,
        )
        .translate(x, y, AIRLOCK_PLINTH_Z + 38.0);
        blocks = blocks + (block - slot);
    }
    blocks
}

fn closed_airlock_route_arrows() -> Part {
    let arrow_body = centered_cube(
        name("closed_airlock_clean_transfer_route_bar"),
        AIRLOCK_X - 120.0,
        8.0,
        4.0,
    )
    .translate(0.0, 0.0, AIRLOCK_PLINTH_Z + 34.0);
    let arrow_head_left = centered_cube(
        name("closed_airlock_dirty_to_clean_arrow_head_left"),
        32.0,
        8.0,
        4.0,
    )
    .rotate(0.0, 0.0, 30.0)
    .translate(AIRLOCK_X / 2.0 - 86.0, 10.0, AIRLOCK_PLINTH_Z + 34.0);
    let arrow_head_right = centered_cube(
        name("closed_airlock_dirty_to_clean_arrow_head_right"),
        32.0,
        8.0,
        4.0,
    )
    .rotate(0.0, 0.0, -30.0)
    .translate(AIRLOCK_X / 2.0 - 86.0, -10.0, AIRLOCK_PLINTH_Z + 34.0);

    arrow_body + arrow_head_left + arrow_head_right
}

fn residue_coupon_racks() -> Part {
    let panel = centered_cube(
        name("residue_coupon_rack_panel"),
        RESIDUE_X,
        RESIDUE_Y,
        RESIDUE_Z,
    )
    .translate(0.0, 0.0, RESIDUE_Z / 2.0);
    let recess = centered_cube(
        name("residue_coupon_rack_drainable_recess_cut"),
        RESIDUE_X - 48.0,
        RESIDUE_Y - 46.0,
        12.0,
    )
    .translate(0.0, 0.0, RESIDUE_Z / 2.0 - 5.0);
    let mut coupons = Part::empty(name("residue_coupon_array"));
    for row in 0..RESIDUE_ROWS {
        for col in 0..RESIDUE_COLS {
            let index = row * RESIDUE_COLS + col;
            let x = centered_index(col, RESIDUE_COLS, RESIDUE_PITCH_X);
            let y = centered_index(row, RESIDUE_ROWS, RESIDUE_PITCH_Y);
            coupons = coupons + residue_coupon_pocket(index, row).translate(x, y, RESIDUE_Z);
        }
    }
    let rails = residue_material_rails();
    let labels = residue_coupon_label_lands();
    let drip_lip = centered_cube(
        name("residue_coupon_front_drip_lip"),
        RESIDUE_X - 54.0,
        8.0,
        16.0,
    )
    .translate(0.0, -RESIDUE_Y / 2.0 + 22.0, RESIDUE_Z + 8.0);

    (panel - recess + coupons + rails + labels + drip_lip).translate(
        RESIDUE_POS.0,
        RESIDUE_POS.1,
        deck_top_z(),
    )
}

fn residue_coupon_pocket(index: usize, material_row: usize) -> Part {
    let pocket = centered_cylinder(
        name(&format!(
            "residue_{}_coupon_pocket_{index}",
            RESIDUE_MATERIALS[material_row]
        )),
        RESIDUE_COUPON_D / 2.0 + 4.0,
        9.0,
        44,
    );
    let coupon = centered_cylinder(
        name(&format!(
            "residue_{}_witness_coupon_land_{index}",
            RESIDUE_MATERIALS[material_row]
        )),
        RESIDUE_COUPON_D / 2.0,
        5.0,
        44,
    );
    let center_cut = centered_cylinder(
        name(&format!("residue_coupon_swab_center_cut_{index}")),
        5.0,
        7.0,
        24,
    );
    let clock_tick = centered_cube(
        name(&format!("residue_coupon_clocking_tick_{index}")),
        5.0,
        16.0,
        6.0,
    )
    .translate(RESIDUE_COUPON_D / 2.0 + 6.0, 0.0, 1.0);

    pocket + (coupon - center_cut) + clock_tick
}

fn residue_material_rails() -> Part {
    let mut rails = Part::empty(name("residue_material_row_rails"));
    for row in 0..RESIDUE_ROWS {
        let y = centered_index(row, RESIDUE_ROWS, RESIDUE_PITCH_Y);
        let rail = centered_cube(
            name(&format!(
                "residue_{}_row_datum_rail",
                RESIDUE_MATERIALS[row]
            )),
            RESIDUE_X - 54.0,
            6.0,
            12.0,
        )
        .translate(0.0, y - RESIDUE_PITCH_Y / 2.0 + 18.0, RESIDUE_Z + 6.0);
        rails = rails + rail;
    }
    rails
}

fn residue_coupon_label_lands() -> Part {
    let mut labels = Part::empty(name("residue_coupon_csg_label_lands"));
    for row in 0..RESIDUE_ROWS {
        labels = labels
            + raised_label_land(
                &format!("residue_{}_row_label", RESIDUE_MATERIALS[row]),
                116.0,
                16.0,
                row,
            )
            .translate(
                -RESIDUE_X / 2.0 + 70.0,
                centered_index(row, RESIDUE_ROWS, RESIDUE_PITCH_Y),
                RESIDUE_Z + 10.0,
            );
    }
    labels
}

fn aeration_timer_token_lanes() -> Part {
    let base = centered_cube(
        name("aeration_timer_token_lane_panel"),
        TIMER_X,
        TIMER_Y,
        TIMER_Z,
    )
    .translate(0.0, 0.0, TIMER_Z / 2.0);
    let mut lanes = Part::empty(name("aeration_timer_lanes"));
    for (i, minutes) in TIMER_MINUTES.iter().enumerate() {
        lanes = lanes + aeration_timer_lane(i, *minutes);
    }
    let zero_hold = centered_cube(
        name("aeration_timer_zero_time_hold_gate"),
        TIMER_X - 48.0,
        12.0,
        30.0,
    )
    .translate(0.0, -TIMER_Y / 2.0 + 32.0, TIMER_Z + 15.0);
    let release_gate = centered_cube(
        name("aeration_timer_clearance_release_gate"),
        TIMER_X - 48.0,
        12.0,
        30.0,
    )
    .translate(0.0, TIMER_Y / 2.0 - 32.0, TIMER_Z + 15.0);
    let labels = raised_label_land("aeration_timer_lane_csg_header", 188.0, 18.0, 4).translate(
        0.0,
        TIMER_Y / 2.0 - 18.0,
        TIMER_Z + 8.0,
    );

    (base + lanes + zero_hold + release_gate + labels).translate(
        TIMER_POS.0,
        TIMER_POS.1,
        deck_top_z(),
    )
}

fn aeration_timer_lane(index: usize, minutes: usize) -> Part {
    let x = centered_index(index, TIMER_LANE_COUNT, TIMER_LANE_PITCH_X);
    let floor = centered_cube(
        name(&format!("aeration_timer_{minutes}_minute_lane_floor")),
        42.0,
        TIMER_Y - 54.0,
        5.0,
    )
    .translate(x, 0.0, TIMER_Z + 2.5);
    let rail_left = centered_cube(
        name(&format!("aeration_timer_{minutes}_minute_left_rail")),
        4.0,
        TIMER_Y - 60.0,
        18.0,
    )
    .translate(x - 24.0, 0.0, TIMER_Z + 9.0);
    let rail_right = centered_cube(
        name(&format!("aeration_timer_{minutes}_minute_right_rail")),
        4.0,
        TIMER_Y - 60.0,
        18.0,
    )
    .translate(x + 24.0, 0.0, TIMER_Z + 9.0);
    let mut tokens = Part::empty(name(&format!(
        "aeration_timer_{minutes}_minute_token_pockets"
    )));
    for token in 0..TOKENS_PER_TIMER_LANE {
        let y = centered_index(token, TOKENS_PER_TIMER_LANE, TIMER_TOKEN_PITCH_Y);
        let ring = centered_cylinder(
            name(&format!(
                "aeration_timer_{minutes}_minute_token_ring_{token}"
            )),
            12.0,
            5.0,
            32,
        )
        .translate(x, y, TIMER_Z + 7.5);
        let cut = centered_cylinder(
            name(&format!(
                "aeration_timer_{minutes}_minute_token_center_cut_{token}"
            )),
            7.0,
            6.0,
            28,
        )
        .translate(x, y, TIMER_Z + 7.5);
        tokens = tokens + (ring - cut);
    }
    let label = raised_label_land(
        &format!("aeration_timer_{minutes}_minute_csg_label"),
        34.0,
        12.0,
        index,
    )
    .translate(x, -TIMER_Y / 2.0 + 17.0, TIMER_Z + 8.0);

    floor + rail_left + rail_right + tokens + label
}

fn peroxide_sensor_pockets() -> Part {
    let base = centered_cube(
        name("peroxide_sensor_pocket_block"),
        SENSOR_X,
        SENSOR_Y,
        SENSOR_Z,
    )
    .translate(0.0, 0.0, SENSOR_Z / 2.0);
    let service_recess = centered_cube(
        name("peroxide_sensor_service_recess_cut"),
        SENSOR_X - 42.0,
        SENSOR_Y - 46.0,
        12.0,
    )
    .translate(0.0, 0.0, SENSOR_Z / 2.0 - 5.0);
    let mut pockets = Part::empty(name("peroxide_sensor_pocket_array"));
    for i in 0..SENSOR_POCKET_COUNT {
        let y = centered_index(i, SENSOR_POCKET_COUNT, SENSOR_POCKET_PITCH_Y);
        pockets = pockets + peroxide_sensor_pocket(i).translate(-34.0, y, SENSOR_Z);
    }
    let sample_ports = peroxide_sample_ports();
    let calibration = peroxide_calibration_wells();
    let labels = raised_label_land("peroxide_sensor_pocket_csg_label", 148.0, 18.0, 5).translate(
        12.0,
        SENSOR_Y / 2.0 - 22.0,
        SENSOR_Z + 8.0,
    );

    (base - service_recess + pockets + sample_ports + calibration + labels).translate(
        SENSOR_POS.0,
        SENSOR_POS.1,
        deck_top_z(),
    )
}

fn peroxide_sensor_pocket(index: usize) -> Part {
    let pocket = centered_cube(
        name(&format!("peroxide_sensor_pocket_{index}_outer_rim")),
        56.0,
        28.0,
        18.0,
    );
    let cut = centered_cube(
        name(&format!(
            "peroxide_sensor_pocket_{index}_sensor_clearance_cut"
        )),
        38.0,
        15.0,
        20.0,
    );
    let cable_clip = centered_cube(
        name(&format!("peroxide_sensor_pocket_{index}_cable_clip_land")),
        14.0,
        8.0,
        14.0,
    )
    .translate(33.0, 0.0, 2.0);
    let label = raised_label_land(
        &format!("peroxide_sensor_{index}_position_label"),
        34.0,
        10.0,
        index,
    )
    .translate(0.0, 18.0, 7.0);

    (pocket - cut) + cable_clip + label
}

fn peroxide_sample_ports() -> Part {
    let mut ports = Part::empty(name("peroxide_sample_port_lands"));
    for i in 0..SENSOR_SAMPLE_PORT_COUNT {
        let y = centered_index(i, SENSOR_SAMPLE_PORT_COUNT, 52.0);
        let boss = centered_cylinder(
            name(&format!("peroxide_sample_port_boss_{i}")),
            13.0,
            12.0,
            32,
        )
        .translate(SENSOR_X / 2.0 - 42.0, y, SENSOR_Z + 6.0);
        let bore = centered_cylinder(
            name(&format!("peroxide_sample_port_bore_{i}")),
            4.8,
            14.0,
            24,
        )
        .translate(SENSOR_X / 2.0 - 42.0, y, SENSOR_Z + 6.0);
        ports = ports + (boss - bore);
    }
    ports
}

fn peroxide_calibration_wells() -> Part {
    let mut wells = Part::empty(name("peroxide_zero_span_challenge_wells"));
    for i in 0..SENSOR_CALIBRATION_WELLS {
        let x = -SENSOR_X / 2.0 + 38.0 + i as f64 * 38.0;
        let ring = centered_cylinder(
            name(&format!("peroxide_calibration_well_ring_{i}")),
            14.0,
            6.0,
            32,
        )
        .translate(x, -SENSOR_Y / 2.0 + 28.0, SENSOR_Z + 3.0);
        let cut = centered_cylinder(
            name(&format!("peroxide_calibration_well_cup_cut_{i}")),
            9.5,
            7.0,
            28,
        )
        .translate(x, -SENSOR_Y / 2.0 + 28.0, SENSOR_Z + 3.0);
        wells = wells + (ring - cut);
    }
    wells
}

fn purge_flow_witness_vanes() -> Part {
    let base = centered_cube(
        name("purge_flow_witness_vane_plenum"),
        PURGE_X,
        PURGE_Y,
        PURGE_Z,
    )
    .translate(0.0, 0.0, PURGE_Z / 2.0);
    let plenum_recess = centered_cube(
        name("purge_flow_witness_plenum_shadow_recess_cut"),
        PURGE_X - 78.0,
        PURGE_Y - 64.0,
        12.0,
    )
    .translate(0.0, 0.0, PURGE_Z / 2.0 - 5.0);
    let vane_bank = purge_vane_bank();
    let ports = purge_flow_ports();
    let direction = purge_flow_direction_markers();
    let labels = raised_label_land("purge_flow_witness_vane_csg_label", 192.0, 18.0, 6).translate(
        0.0,
        PURGE_Y / 2.0 - 22.0,
        PURGE_Z + 9.0,
    );

    (base - plenum_recess + vane_bank + ports + direction + labels).translate(
        PURGE_POS.0,
        PURGE_POS.1,
        deck_top_z(),
    )
}

fn purge_vane_bank() -> Part {
    let mut vanes = Part::empty(name("purge_flow_witness_vane_bank"));
    for i in 0..PURGE_VANE_COUNT {
        let x = centered_index(i, PURGE_VANE_COUNT, PURGE_VANE_PITCH_X);
        let angle = if i % 2 == 0 { 12.0 } else { -12.0 };
        let vane = centered_cube(
            name(&format!("purge_flow_witness_vane_{i}")),
            8.0,
            76.0,
            PURGE_VANE_Z,
        )
        .rotate(0.0, 0.0, angle)
        .translate(x, 0.0, PURGE_Z + PURGE_VANE_Z / 2.0);
        let pivot = centered_cylinder(
            name(&format!("purge_flow_witness_vane_pivot_{i}")),
            5.0,
            92.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, 0.0, PURGE_Z + 20.0);
        let index_tick = centered_cube(
            name(&format!("purge_flow_witness_vane_index_tick_{i}")),
            26.0,
            4.0,
            5.0,
        )
        .rotate(0.0, 0.0, angle)
        .translate(x, -54.0, PURGE_Z + 7.0);
        vanes = vanes + vane + pivot + index_tick;
    }
    vanes
}

fn purge_flow_ports() -> Part {
    let mut ports = Part::empty(name("purge_flow_inlet_exhaust_sample_ports"));
    for i in 0..PURGE_PORT_COUNT {
        let x = centered_index(i, PURGE_PORT_COUNT, 150.0);
        let boss = centered_cylinder(name(&format!("purge_flow_port_boss_{i}")), 22.0, 16.0, 40)
            .translate(x, -PURGE_Y / 2.0 + 36.0, PURGE_Z + 8.0);
        let bore = centered_cylinder(name(&format!("purge_flow_port_bore_{i}")), 11.0, 18.0, 34)
            .translate(x, -PURGE_Y / 2.0 + 36.0, PURGE_Z + 8.0);
        ports = ports + (boss - bore);
    }
    ports
}

fn purge_flow_direction_markers() -> Part {
    let mut markers = Part::empty(name("purge_flow_direction_csg_markers"));
    for i in 0..5 {
        let x = centered_index(i, 5, 74.0);
        let shaft = centered_cube(name(&format!("purge_flow_arrow_shaft_{i}")), 42.0, 5.0, 4.0)
            .translate(x, PURGE_Y / 2.0 - 50.0, PURGE_Z + 6.0);
        let head_a = centered_cube(
            name(&format!("purge_flow_arrow_head_a_{i}")),
            20.0,
            5.0,
            4.0,
        )
        .rotate(0.0, 0.0, 30.0)
        .translate(x + 22.0, PURGE_Y / 2.0 - 44.0, PURGE_Z + 6.0);
        let head_b = centered_cube(
            name(&format!("purge_flow_arrow_head_b_{i}")),
            20.0,
            5.0,
            4.0,
        )
        .rotate(0.0, 0.0, -30.0)
        .translate(x + 22.0, PURGE_Y / 2.0 - 56.0, PURGE_Z + 6.0);
        markers = markers + shaft + head_a + head_b;
    }
    markers
}

fn door_gasket_coupon_lands() -> Part {
    let base = centered_cube(
        name("door_gasket_coupon_land_panel"),
        GASKET_X,
        GASKET_Y,
        GASKET_Z,
    )
    .translate(0.0, 0.0, GASKET_Z / 2.0);
    let recess = centered_cube(
        name("door_gasket_coupon_land_cleanout_recess_cut"),
        GASKET_X - 52.0,
        GASKET_Y - 42.0,
        10.0,
    )
    .translate(0.0, 0.0, GASKET_Z / 2.0 - 4.0);
    let door_lands =
        gasket_door_land("inner", -GASKET_X / 4.0) + gasket_door_land("outer", GASKET_X / 4.0);
    let steps = gasket_compression_steps();
    let labels = raised_label_land("door_gasket_coupon_csg_header", 174.0, 18.0, 7).translate(
        0.0,
        GASKET_Y / 2.0 - 20.0,
        GASKET_Z + 8.0,
    );

    (base - recess + door_lands + steps + labels).translate(
        GASKET_POS.0,
        GASKET_POS.1,
        deck_top_z(),
    )
}

fn gasket_door_land(label: &str, x: f64) -> Part {
    let frame = rect_frame(
        &format!("{label}_door_gasket_coupon_frame_land"),
        148.0,
        124.0,
        8.0,
        6.0,
    )
    .translate(x, 10.0, GASKET_Z + 3.0);
    let mut coupons = Part::empty(name(&format!("{label}_door_gasket_coupon_lands")));
    for i in 0..GASKET_COUPONS_PER_DOOR {
        let side = if i < 3 { -1.0 } else { 1.0 };
        let row = i % 3;
        let cx = x + side * 62.0;
        let cy = centered_index(row, 3, 40.0) + 10.0;
        let land = centered_cube(
            name(&format!("{label}_door_gasket_coupon_land_{i}")),
            34.0,
            20.0,
            5.0,
        )
        .translate(cx, cy, GASKET_Z + 6.0);
        let witness = centered_cylinder(
            name(&format!("{label}_door_gasket_coupon_pin_{i}")),
            3.0,
            8.0,
            20,
        )
        .translate(cx, cy, GASKET_Z + 10.0);
        coupons = coupons + land + witness;
    }
    let label_land = raised_label_land(
        &format!("{label}_gasket_coupon_csg_label"),
        78.0,
        14.0,
        if label == "inner" { 0 } else { 1 },
    )
    .translate(x, -GASKET_Y / 2.0 + 28.0, GASKET_Z + 8.0);

    frame + coupons + label_land
}

fn gasket_compression_steps() -> Part {
    let mut steps = Part::empty(name("door_gasket_compression_step_gauges"));
    for i in 0..GASKET_COMPRESSION_STEPS {
        let height = 2.0 + i as f64 * 1.0;
        steps = steps
            + centered_cube(
                name(&format!("door_gasket_compression_step_{i}")),
                28.0,
                28.0,
                height,
            )
            .translate(
                -GASKET_X / 2.0 + 42.0 + i as f64 * 36.0,
                -GASKET_Y / 2.0 + 28.0,
                GASKET_Z + height / 2.0,
            );
    }
    steps
}

fn clean_dirty_segregation_gate() -> Part {
    let base = centered_cube(
        name("clean_dirty_segregation_base"),
        SEGREGATION_X,
        SEGREGATION_Y,
        SEGREGATION_Z,
    )
    .translate(0.0, 0.0, SEGREGATION_Z / 2.0);
    let divider = centered_cube(
        name("clean_dirty_segregation_vertical_divider_wall"),
        SEGREGATION_X - 54.0,
        16.0,
        CLEAN_DIRTY_DIVIDER_Z,
    )
    .translate(0.0, 0.0, SEGREGATION_Z + CLEAN_DIRTY_DIVIDER_Z / 2.0);
    let pass_slot = centered_cube(
        name("clean_dirty_segregation_validated_pass_slot_cut"),
        118.0,
        20.0,
        38.0,
    )
    .translate(0.0, 0.0, SEGREGATION_Z + 36.0);
    let bins = segregation_bin_lands();
    let labels = raised_label_land("dirty_side_csg_label", 84.0, 18.0, 0).translate(
        -SEGREGATION_X / 4.0,
        -SEGREGATION_Y / 2.0 + 24.0,
        SEGREGATION_Z + 8.0,
    ) + raised_label_land("clean_side_csg_label", 84.0, 18.0, 1).translate(
        SEGREGATION_X / 4.0,
        SEGREGATION_Y / 2.0 - 24.0,
        SEGREGATION_Z + 8.0,
    );

    (base + (divider - pass_slot) + bins + labels).translate(
        SEGREGATION_POS.0,
        SEGREGATION_POS.1,
        deck_top_z(),
    )
}

fn segregation_bin_lands() -> Part {
    let mut bins = Part::empty(name("clean_dirty_segregation_bin_lands"));
    for i in 0..SEGREGATION_BIN_COUNT {
        let x = centered_index(i % 2, 2, 96.0);
        let y = if i < 2 {
            -SEGREGATION_Y / 2.0 + 38.0
        } else {
            SEGREGATION_Y / 2.0 - 38.0
        };
        let bin = centered_cube(name(&format!("segregation_bin_land_{i}")), 72.0, 34.0, 8.0)
            .translate(x, y, SEGREGATION_Z + 4.0);
        let drain = centered_cylinder(
            name(&format!("segregation_bin_drain_cut_{i}")),
            5.0,
            10.0,
            20,
        )
        .translate(x + 24.0, y, SEGREGATION_Z + 4.0);
        bins = bins + (bin - drain);
    }
    bins
}

fn barcode_certificate_lands() -> Part {
    let panel = centered_cube(
        name("barcode_certificate_traceability_panel"),
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    )
    .translate(0.0, 0.0, TRACE_Z / 2.0);
    let barcode = barcode_lands();
    let certificates = certificate_lands();
    let csg_matrix = traceability_csg_label_matrix();
    let token_pockets = witness_token_pockets();
    let labels = raised_label_land("barcode_certificate_panel_csg_header", 204.0, 18.0, 8)
        .translate(0.0, TRACE_Y / 2.0 - 20.0, TRACE_Z + 8.0);

    (panel + barcode + certificates + csg_matrix + token_pockets + labels).translate(
        TRACE_POS.0,
        TRACE_POS.1,
        deck_top_z(),
    )
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty(name("barcode_scan_lands"));
    for i in 0..BARCODE_LANDS {
        let col = i % 5;
        let row = i / 5;
        let x = -TRACE_X / 2.0 + 54.0 + col as f64 * 74.0;
        let y = -TRACE_Y / 2.0 + 36.0 + row as f64 * 36.0;
        let land = centered_cube(name(&format!("barcode_scan_land_{i}")), 58.0, 18.0, 4.0)
            .translate(x, y, TRACE_Z + 2.0);
        let bars = label_code_bars(&format!("barcode_scan_land_{i}"), 58.0, 18.0, i).translate(
            x,
            y,
            TRACE_Z + 3.0,
        );
        lands = lands + land + bars;
    }
    lands
}

fn certificate_lands() -> Part {
    let mut lands = Part::empty(name("certificate_lands"));
    for i in 0..CERTIFICATE_LANDS {
        let x = -TRACE_X / 2.0 + 70.0 + i as f64 * 96.0;
        let y = TRACE_Y / 2.0 - 48.0;
        let land = centered_cube(name(&format!("certificate_land_{i}")), 78.0, 34.0, 4.0)
            .translate(x, y, TRACE_Z + 2.0);
        let clip_top = centered_cube(
            name(&format!("certificate_land_{i}_top_clip")),
            72.0,
            4.0,
            8.0,
        )
        .translate(x, y + 17.0, TRACE_Z + 6.0);
        let clip_bottom = centered_cube(
            name(&format!("certificate_land_{i}_bottom_clip")),
            72.0,
            4.0,
            8.0,
        )
        .translate(x, y - 17.0, TRACE_Z + 6.0);
        lands = lands + land + clip_top + clip_bottom;
    }
    lands
}

fn traceability_csg_label_matrix() -> Part {
    let mut bars = Part::empty(name("traceability_raised_csg_label_bar_matrix"));
    for row in 0..4 {
        for col in 0..12 {
            let height = if (row + col) % 3 == 0 { 20.0 } else { 11.0 };
            bars = bars
                + centered_cube(
                    name(&format!("traceability_csg_label_bar_{row}_{col}")),
                    2.2,
                    height,
                    3.0,
                )
                .translate(
                    TRACE_X / 2.0 - 110.0 + col as f64 * 6.0,
                    -TRACE_Y / 2.0 + 28.0 + row as f64 * 24.0,
                    TRACE_Z + 4.0,
                );
        }
    }
    bars
}

fn witness_token_pockets() -> Part {
    let mut pockets = Part::empty(name("traceability_witness_token_pockets"));
    for i in 0..6 {
        let ring = centered_cylinder(
            name(&format!("traceability_witness_token_ring_{i}")),
            10.0,
            5.0,
            28,
        )
        .translate(
            TRACE_X / 2.0 - 50.0,
            centered_index(i, 6, 20.0),
            TRACE_Z + 2.5,
        );
        let cut = centered_cylinder(
            name(&format!("traceability_witness_token_cut_{i}")),
            6.0,
            6.0,
            24,
        )
        .translate(
            TRACE_X / 2.0 - 50.0,
            centered_index(i, 6, 20.0),
            TRACE_Z + 2.5,
        );
        pockets = pockets + (ring - cut);
    }
    pockets
}

fn release_hold_reject_lanes() -> Part {
    let base = centered_cube(
        name("release_hold_reject_lane_base"),
        DISPOSITION_X,
        DISPOSITION_Y,
        DISPOSITION_Z,
    )
    .translate(0.0, 0.0, DISPOSITION_Z / 2.0);
    let mut lanes = Part::empty(name("release_hold_reject_lane_set"));
    for lane in DispositionLane::all() {
        lanes = lanes + disposition_lane(lane);
    }
    let custody_gate = centered_cube(
        name("release_hold_reject_lane_custody_gate_bar"),
        DISPOSITION_X - 42.0,
        10.0,
        36.0,
    )
    .translate(0.0, DISPOSITION_Y / 2.0 - 24.0, DISPOSITION_Z + 18.0);
    let labels = raised_label_land("release_hold_reject_csg_header", 190.0, 18.0, 9).translate(
        0.0,
        -DISPOSITION_Y / 2.0 + 20.0,
        DISPOSITION_Z + 8.0,
    );

    (base + lanes + custody_gate + labels).translate(
        DISPOSITION_POS.0,
        DISPOSITION_POS.1,
        deck_top_z(),
    )
}

fn disposition_lane(lane: DispositionLane) -> Part {
    let label = lane.label();
    let index = lane.index();
    let x = centered_index(index, DISPOSITION_LANE_COUNT, DISPOSITION_LANE_PITCH_X);
    let floor = centered_cube(
        name(&format!("{label}_lane_floor_land")),
        120.0,
        DISPOSITION_Y - 46.0,
        6.0,
    )
    .translate(x, 0.0, DISPOSITION_Z + 3.0);
    let left_rail = centered_cube(
        name(&format!("{label}_lane_left_rail")),
        6.0,
        DISPOSITION_Y - 52.0,
        22.0,
    )
    .translate(x - 66.0, 0.0, DISPOSITION_Z + 11.0);
    let right_rail = centered_cube(
        name(&format!("{label}_lane_right_rail")),
        6.0,
        DISPOSITION_Y - 52.0,
        22.0,
    )
    .translate(x + 66.0, 0.0, DISPOSITION_Z + 11.0);
    let mut slots = Part::empty(name(&format!("{label}_lane_token_slots")));
    for i in 0..DISPOSITION_SLOTS_PER_LANE {
        let y = centered_index(i, DISPOSITION_SLOTS_PER_LANE, 22.0);
        let slot = centered_cube(
            name(&format!("{label}_lane_token_slot_{i}")),
            76.0,
            12.0,
            8.0,
        )
        .translate(x, y, DISPOSITION_Z + 8.0);
        slots = slots + slot;
    }
    let gate = centered_cube(
        name(&format!("{label}_lane_status_gate")),
        104.0,
        10.0,
        34.0,
    )
    .translate(x, DISPOSITION_Y / 2.0 - 42.0, DISPOSITION_Z + 17.0);
    let lane_label = raised_label_land(&format!("{label}_lane_csg_label"), 78.0, 14.0, index)
        .translate(x, -DISPOSITION_Y / 2.0 + 42.0, DISPOSITION_Z + 8.0);

    floor + left_rail + right_rail + slots + gate + lane_label
}

fn evidence_camera_bridge() -> Part {
    let left_post = centered_cube(
        name("evidence_camera_bridge_left_post"),
        30.0,
        BRIDGE_POST_Y,
        BRIDGE_UNDERSIDE_Z,
    )
    .translate(-BRIDGE_SPAN_X / 2.0, 0.0, BRIDGE_UNDERSIDE_Z / 2.0);
    let right_post = centered_cube(
        name("evidence_camera_bridge_right_post"),
        30.0,
        BRIDGE_POST_Y,
        BRIDGE_UNDERSIDE_Z,
    )
    .translate(BRIDGE_SPAN_X / 2.0, 0.0, BRIDGE_UNDERSIDE_Z / 2.0);
    let beam = centered_cube(
        name("evidence_camera_bridge_overhead_beam"),
        BRIDGE_SPAN_X + 42.0,
        32.0,
        28.0,
    )
    .translate(0.0, 0.0, BRIDGE_UNDERSIDE_Z + 14.0);
    let cameras = evidence_camera_pods();
    let lights = evidence_light_bars();
    let sight_bar = centered_cube(
        name("evidence_bridge_certificate_sight_bar"),
        BRIDGE_SPAN_X - 160.0,
        8.0,
        8.0,
    )
    .translate(0.0, -BRIDGE_POST_Y / 2.0 - 18.0, BRIDGE_UNDERSIDE_Z - 28.0);
    let labels = raised_label_land("evidence_camera_bridge_csg_label", 174.0, 18.0, 10).translate(
        0.0,
        BRIDGE_POST_Y / 2.0 + 18.0,
        BRIDGE_UNDERSIDE_Z + 34.0,
    );

    (left_post + right_post + beam + cameras + lights + sight_bar + labels).translate(
        BRIDGE_POS.0,
        BRIDGE_POS.1,
        deck_top_z(),
    )
}

fn evidence_camera_pods() -> Part {
    let mut pods = Part::empty(name("evidence_camera_pods"));
    for i in 0..CAMERA_COUNT {
        let x = centered_index(i, CAMERA_COUNT, 260.0);
        let pod = centered_cube(name(&format!("evidence_camera_pod_{i}")), 46.0, 34.0, 34.0)
            .translate(x, 0.0, BRIDGE_UNDERSIDE_Z - 18.0);
        let lens = centered_cylinder(
            name(&format!("evidence_camera_lens_ring_{i}")),
            12.0,
            8.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -22.0, BRIDGE_UNDERSIDE_Z - 18.0);
        pods = pods + pod + lens;
    }
    pods
}

fn evidence_light_bars() -> Part {
    let mut lights = Part::empty(name("evidence_light_bars"));
    for i in 0..LIGHT_BAR_COUNT {
        let x = centered_index(i, LIGHT_BAR_COUNT, 132.0);
        lights = lights
            + centered_cube(name(&format!("evidence_led_light_bar_{i}")), 96.0, 8.0, 6.0)
                .translate(x, BRIDGE_POST_Y / 2.0 + 14.0, BRIDGE_UNDERSIDE_Z - 34.0);
    }
    lights
}

fn robot_service_keepout_gauges() -> Part {
    let front = centered_cube(
        name("front_robot_approach_keepout_bar"),
        KEEP_OUT_X,
        8.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, -KEEP_OUT_Y / 2.0, deck_top_z() + KEEP_OUT_Z / 2.0);
    let rear = centered_cube(
        name("rear_aeration_service_keepout_bar"),
        KEEP_OUT_X,
        8.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, KEEP_OUT_Y / 2.0, deck_top_z() + KEEP_OUT_Z / 2.0);
    let left = centered_cube(
        name("left_dirty_load_keepout_bar"),
        8.0,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(-KEEP_OUT_X / 2.0, 0.0, deck_top_z() + KEEP_OUT_Z / 2.0);
    let right = centered_cube(
        name("right_peroxide_sensor_service_keepout_bar"),
        8.0,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(KEEP_OUT_X / 2.0, 0.0, deck_top_z() + KEEP_OUT_Z / 2.0);
    let clearances = keepout_clearance_label_lands();
    let posts = keepout_corner_posts();

    front + rear + left + right + clearances + posts
}

fn keepout_clearance_label_lands() -> Part {
    raised_label_land(
        &format!("front_robot_{FRONT_ROBOT_APPROACH_MM:.0}mm_approach_label"),
        242.0,
        18.0,
        0,
    )
    .translate(0.0, -KEEP_OUT_Y / 2.0 + 34.0, deck_top_z() + KEEP_OUT_Z)
        + raised_label_land(
            &format!("rear_service_{REAR_SERVICE_MM:.0}mm_aeration_label"),
            242.0,
            18.0,
            1,
        )
        .translate(0.0, KEEP_OUT_Y / 2.0 - 34.0, deck_top_z() + KEEP_OUT_Z)
        + raised_label_land(
            &format!("side_sensor_{SIDE_SENSOR_SERVICE_MM:.0}mm_service_label"),
            220.0,
            18.0,
            2,
        )
        .rotate(0.0, 0.0, 90.0)
        .translate(KEEP_OUT_X / 2.0 - 34.0, 0.0, deck_top_z() + KEEP_OUT_Z)
        + raised_label_land(
            &format!("top_aeration_{TOP_AERATION_LIFT_MM:.0}mm_lift_label"),
            220.0,
            18.0,
            3,
        )
        .translate(
            -KEEP_OUT_X / 2.0 + 176.0,
            KEEP_OUT_Y / 2.0 - 34.0,
            deck_top_z() + KEEP_OUT_Z,
        )
}

fn keepout_corner_posts() -> Part {
    let mut posts = Part::empty(name("keepout_corner_posts"));
    for (i, (x, y)) in [
        (-KEEP_OUT_X / 2.0, -KEEP_OUT_Y / 2.0),
        (KEEP_OUT_X / 2.0, -KEEP_OUT_Y / 2.0),
        (-KEEP_OUT_X / 2.0, KEEP_OUT_Y / 2.0),
        (KEEP_OUT_X / 2.0, KEEP_OUT_Y / 2.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cylinder(name(&format!("keepout_corner_post_{i}")), 10.0, 34.0, 28)
                .translate(*x, *y, deck_top_z() + 17.0);
    }
    posts
}

fn rect_frame(label: &str, x: f64, y: f64, bar: f64, z: f64) -> Part {
    let top = centered_cube(name(&format!("{label}_top")), x, bar, z).translate(
        0.0,
        y / 2.0 - bar / 2.0,
        0.0,
    );
    let bottom = centered_cube(name(&format!("{label}_bottom")), x, bar, z).translate(
        0.0,
        -y / 2.0 + bar / 2.0,
        0.0,
    );
    let left = centered_cube(name(&format!("{label}_left")), bar, y - 2.0 * bar, z).translate(
        -x / 2.0 + bar / 2.0,
        0.0,
        0.0,
    );
    let right = centered_cube(name(&format!("{label}_right")), bar, y - 2.0 * bar, z).translate(
        x / 2.0 - bar / 2.0,
        0.0,
        0.0,
    );

    top + bottom + left + right
}

fn raised_label_land(label: &str, x: f64, y: f64, code: usize) -> Part {
    let land = centered_cube(name(label), x, y, 2.0).translate(0.0, 0.0, 1.0);
    land + label_code_bars(label, x, y, code)
}

fn label_code_bars(label: &str, x: f64, y: f64, code: usize) -> Part {
    let mut bars = Part::empty(name(&format!("{label}_raised_code_bars")));
    for bit in 0..6 {
        let bar_h = if (code + bit) % 2 == 0 {
            y - 5.0
        } else {
            (y - 6.0) / 2.0
        };
        bars = bars
            + centered_cube(
                name(&format!("{label}_raised_code_bar_{bit}")),
                2.2,
                bar_h,
                1.2,
            )
            .translate(-x / 2.0 + 9.0 + bit as f64 * 5.4, 0.0, 2.6);
    }
    bars
}

fn datum_disc(label: &str) -> Part {
    let outer = centered_cylinder(name(&format!("{label}_outer_ring")), 13.0, 4.0, 36);
    let inner = centered_cylinder(name(&format!("{label}_center_cut")), 4.0, 5.0, 24);
    let cross_x = centered_cube(name(&format!("{label}_cross_x")), 24.0, 2.0, 5.0);
    let cross_y = centered_cube(name(&format!("{label}_cross_y")), 2.0, 24.0, 5.0);
    outer - inner + cross_x + cross_y
}

fn module_rects() -> [Rect; 9] {
    [
        Rect {
            name: "closed_transfer_airlock_reference",
            center: AIRLOCK_POS,
            x: AIRLOCK_X,
            y: AIRLOCK_Y,
        },
        Rect {
            name: "residue_coupon_racks",
            center: RESIDUE_POS,
            x: RESIDUE_X,
            y: RESIDUE_Y,
        },
        Rect {
            name: "peroxide_sensor_pockets",
            center: SENSOR_POS,
            x: SENSOR_X,
            y: SENSOR_Y,
        },
        Rect {
            name: "purge_flow_witness_vanes",
            center: PURGE_POS,
            x: PURGE_X,
            y: PURGE_Y,
        },
        Rect {
            name: "door_gasket_coupon_lands",
            center: GASKET_POS,
            x: GASKET_X,
            y: GASKET_Y,
        },
        Rect {
            name: "aeration_timer_token_lanes",
            center: TIMER_POS,
            x: TIMER_X,
            y: TIMER_Y,
        },
        Rect {
            name: "clean_dirty_segregation",
            center: SEGREGATION_POS,
            x: SEGREGATION_X,
            y: SEGREGATION_Y,
        },
        Rect {
            name: "barcode_certificate_lands",
            center: TRACE_POS,
            x: TRACE_X,
            y: TRACE_Y,
        },
        Rect {
            name: "release_hold_reject_lanes",
            center: DISPOSITION_POS,
            x: DISPOSITION_X,
            y: DISPOSITION_Y,
        },
    ]
}

fn assert_design_constraints() {
    assert!(OUTPUTS
        .iter()
        .all(|path| path.starts_with(OUTPUT_PREFIX) && path.ends_with(".stl")));
    assert_eq!(OUTPUTS.len(), 13);

    for feature in REQUIRED_FEATURES {
        assert!(!feature.is_empty());
    }
    for requested in [
        "residue_coupon_racks",
        "aeration_timer_token_lanes",
        "peroxide_sensor_pockets",
        "purge_flow_witness_vanes",
        "door_gasket_coupon_lands",
        "clean_dirty_segregation",
        "barcode_certificate_lands",
        "release_hold_reject_lanes",
        "labels_as_csg_geometry",
    ] {
        assert!(REQUIRED_FEATURES.contains(&requested));
    }

    assert_eq!(RESIDUE_COUPON_COUNT, RESIDUE_ROWS * RESIDUE_COLS);
    assert_eq!(TIMER_TOKEN_COUNT, TIMER_LANE_COUNT * TOKENS_PER_TIMER_LANE);
    assert_eq!(
        GASKET_COUPON_COUNT,
        GASKET_DOOR_COUNT * GASKET_COUPONS_PER_DOOR
    );
    assert_eq!(DispositionLane::all().len(), DISPOSITION_LANE_COUNT);
    assert!(SENSOR_POCKET_COUNT >= 6);
    assert!(PURGE_VANE_COUNT >= 8);
    assert!(CLEAN_DIRTY_DIVIDER_Z > SEGREGATION_Z);
    assert!(TRACE_LABEL_BARS >= BARCODE_LANDS * 4);

    let rects = module_rects();
    for rect in rects {
        assert!(
            rect.fits_inside_deck(),
            "{} does not fit on deck",
            rect.name
        );
    }
    for (i, a) in rects.iter().enumerate() {
        for b in rects.iter().skip(i + 1) {
            assert!(!a.overlaps(*b), "{} overlaps {}", a.name, b.name);
        }
    }
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn deck_top_z() -> f64 {
    DECK_Z / 2.0
}

fn name(label: &str) -> String {
    format!("closed_airlock_vhp_residue_aeration_witness_station_{label}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_paths_are_scoped_to_generator() {
        assert!(OUTPUTS
            .iter()
            .all(|path| path.starts_with(OUTPUT_PREFIX) && path.ends_with(".stl")));
        assert_eq!(OUTPUTS.len(), 13);
    }

    #[test]
    fn requested_feature_groups_are_declared() {
        for feature in [
            "residue_coupon_racks",
            "aeration_timer_token_lanes",
            "peroxide_sensor_pockets",
            "purge_flow_witness_vanes",
            "door_gasket_coupon_lands",
            "clean_dirty_segregation",
            "barcode_certificate_lands",
            "release_hold_reject_lanes",
            "labels_as_csg_geometry",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn witness_counts_match_station_layout() {
        assert_eq!(RESIDUE_COUPON_COUNT, 18);
        assert_eq!(TIMER_TOKEN_COUNT, 20);
        assert_eq!(GASKET_COUPON_COUNT, 12);
        assert_eq!(BARCODE_LANDS, 10);
        assert_eq!(CERTIFICATE_LANDS, 4);
    }

    #[test]
    fn layout_modules_fit_without_overlap() {
        assert_design_constraints();
    }
}
