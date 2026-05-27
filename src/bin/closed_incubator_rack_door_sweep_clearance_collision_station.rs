use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed incubator rack door-sweep clearance collision station.
//
// No-cell validation fixture for proving rack surrogates, transfer carts, and
// robot grippers cannot collide with incubator doors, sweep seals, latches, or
// service keepout envelopes during automated closed-system loading.

const OUTPUT_PREFIX: &str = "closed_incubator_rack_door_sweep_clearance_collision_station";

const OUTPUTS: [&str; 12] = [
    "output/closed_incubator_rack_door_sweep_clearance_collision_station_base_tray.stl",
    "output/closed_incubator_rack_door_sweep_clearance_collision_station_rack_surrogate.stl",
    "output/closed_incubator_rack_door_sweep_clearance_collision_station_door_swing_arc_gauges.stl",
    "output/closed_incubator_rack_door_sweep_clearance_collision_station_sweep_seal_contact_comb.stl",
    "output/closed_incubator_rack_door_sweep_clearance_collision_station_cassette_overhang_challenge_blocks.stl",
    "output/closed_incubator_rack_door_sweep_clearance_collision_station_soft_hard_stop_coupons.stl",
    "output/closed_incubator_rack_door_sweep_clearance_collision_station_latch_clearance_witness_tabs.stl",
    "output/closed_incubator_rack_door_sweep_clearance_collision_station_barcode_custody_lands.stl",
    "output/closed_incubator_rack_door_sweep_clearance_collision_station_transfer_cart_approach_gauge.stl",
    "output/closed_incubator_rack_door_sweep_clearance_collision_station_camera_evidence_bridge.stl",
    "output/closed_incubator_rack_door_sweep_clearance_collision_station_robot_service_keepout_gauges.stl",
    "output/closed_incubator_rack_door_sweep_clearance_collision_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 10] = [
    "rack_surrogate",
    "door_swing_arc_gauges",
    "sweep_seal_contact_comb",
    "cassette_overhang_challenge_blocks",
    "soft_hard_stop_coupons",
    "latch_clearance_witness_tabs",
    "barcode_custody_lands",
    "camera_evidence_bridge",
    "transfer_cart_approach_gauge",
    "robot_service_keepout_gauges",
];

const STATION_X: f64 = 1180.0;
const STATION_Y: f64 = 840.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const SOCKET_DEPTH: f64 = 5.0;
const MOUNT_SLOT_COUNT: usize = 8;
const DATUM_TARGET_COUNT: usize = 6;

const RACK_X: f64 = 430.0;
const RACK_Y: f64 = 345.0;
const RACK_Z: f64 = 34.0;
const RACK_POS: (f64, f64) = (-310.0, 124.0);
const RACK_COLS: usize = 3;
const RACK_ROWS: usize = 2;
const RACK_SLOT_COUNT: usize = RACK_COLS * RACK_ROWS;
const RACK_SLOT_PITCH_X: f64 = 125.0;
const RACK_SLOT_PITCH_Y: f64 = 134.0;
const CASSETTE_X: f64 = 104.0;
const CASSETTE_Y: f64 = 96.0;
const CASSETTE_Z: f64 = 24.0;
const RACK_GRIPPER_LAND_COUNT: usize = 8;

const ARC_PANEL_X: f64 = 480.0;
const ARC_PANEL_Y: f64 = 345.0;
const ARC_PANEL_Z: f64 = 24.0;
const ARC_POS: (f64, f64) = (245.0, 124.0);
const HINGE_LOCAL: (f64, f64) = (-210.0, -140.0);
const DOOR_ARC_OUTER_R: f64 = 296.0;
const DOOR_ARC_CENTER_R: f64 = 258.0;
const DOOR_ARC_INNER_R: f64 = 220.0;
const DOOR_ARC_SEGMENTS: usize = 13;
const ARC_SEGMENT_X: f64 = 42.0;
const ARC_SEGMENT_Y: f64 = 8.0;
const ARC_SEGMENT_Z: f64 = 32.0;
const DOOR_SWING_DEG: f64 = 92.0;
const DOOR_CLEARANCE_MARGIN: f64 = 58.0;

const SEAL_PANEL_X: f64 = 190.0;
const SEAL_PANEL_Y: f64 = 88.0;
const SEAL_PANEL_Z: f64 = 18.0;
const SEAL_POS: (f64, f64) = (380.0, -318.0);
const SEAL_TOOTH_COUNT: usize = 18;
const SEAL_TOOTH_X: f64 = 7.0;
const SEAL_TOOTH_Y: f64 = 44.0;
const SEAL_TOOTH_Z: f64 = 42.0;
const SEAL_TOOTH_PITCH: f64 = 10.0;

const OVERHANG_X: f64 = 430.0;
const OVERHANG_Y: f64 = 154.0;
const OVERHANG_Z: f64 = 24.0;
const OVERHANG_POS: (f64, f64) = (-310.0, -145.0);
const OVERHANG_BLOCK_COUNT: usize = 7;
const OVERHANG_STEP_X: f64 = 46.0;
const OVERHANG_STEP_Y: f64 = 48.0;
const MAX_OVERHANG_Z: f64 = 60.0;

const STOP_X: f64 = 220.0;
const STOP_Y: f64 = 154.0;
const STOP_Z: f64 = 24.0;
const STOP_POS: (f64, f64) = (46.0, -145.0);
const STOP_COUPON_COUNT: usize = 6;
const SOFT_STOP_Z: f64 = 34.0;
const HARD_STOP_Z: f64 = 58.0;

const LATCH_X: f64 = 190.0;
const LATCH_Y: f64 = 154.0;
const LATCH_Z: f64 = 20.0;
const LATCH_POS: (f64, f64) = (380.0, -145.0);
const LATCH_TAB_COUNT: usize = 9;
const LATCH_MIN_CLEARANCE: f64 = 4.0;
const LATCH_MAX_CLEARANCE: f64 = 12.0;

const CUSTODY_X: f64 = 430.0;
const CUSTODY_Y: f64 = 88.0;
const CUSTODY_Z: f64 = 10.0;
const CUSTODY_POS: (f64, f64) = (-310.0, -318.0);
const BARCODE_LAND_COUNT: usize = 8;
const CUSTODY_TOKEN_COUNT: usize = 6;

const CART_X: f64 = 300.0;
const CART_Y: f64 = 88.0;
const CART_Z: f64 = 18.0;
const CART_POS: (f64, f64) = (86.0, -318.0);
const CART_RAIL_COUNT: usize = 2;
const CART_WHEEL_CUP_COUNT: usize = 4;

const BRIDGE_X: f64 = 1040.0;
const BRIDGE_Y: f64 = 70.0;
const BRIDGE_Z: f64 = 150.0;
const BRIDGE_POS: (f64, f64) = (0.0, 336.0);
const BRIDGE_BEAM_Z: f64 = 30.0;
const CAMERA_COUNT: usize = 4;
const CAMERA_FIDUCIAL_COUNT: usize = 8;

const KEEP_OUT_X: f64 = 1110.0;
const KEEP_OUT_Y: f64 = 770.0;
const KEEP_OUT_Z: f64 = 6.0;
const ROBOT_KEEP_OUT_ZONE_COUNT: usize = 6;
const SERVICE_GAUGE_POST_COUNT: usize = 8;
const FRONT_SERVICE_CLEARANCE: f64 = 52.0;
const REAR_CAMERA_SERVICE_CLEARANCE: f64 = 48.0;
const SIDE_SERVICE_CLEARANCE: f64 = 42.0;

#[derive(Clone, Copy)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_rim(self) -> bool {
        self.center.0.abs() + self.x / 2.0 <= STATION_X / 2.0 - RIM_W - 8.0
            && self.center.1.abs() + self.y / 2.0 <= STATION_Y / 2.0 - RIM_W - 8.0
    }

    fn overlaps(self, other: Rect) -> bool {
        let x_overlap =
            (self.center.0 - other.center.0).abs() < self.x / 2.0 + other.x / 2.0 + 12.0;
        let y_overlap =
            (self.center.1 - other.center.1).abs() < self.y / 2.0 + other.y / 2.0 + 12.0;
        x_overlap && y_overlap
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_tray();
    export(OUTPUTS[0], &base);

    let rack = rack_surrogate();
    export(OUTPUTS[1], &rack);

    let arcs = door_swing_arc_gauges();
    export(OUTPUTS[2], &arcs);

    let seal = sweep_seal_contact_comb();
    export(OUTPUTS[3], &seal);

    let overhang = cassette_overhang_challenge_blocks();
    export(OUTPUTS[4], &overhang);

    let stops = soft_hard_stop_coupons();
    export(OUTPUTS[5], &stops);

    let latch = latch_clearance_witness_tabs();
    export(OUTPUTS[6], &latch);

    let custody = barcode_custody_lands();
    export(OUTPUTS[7], &custody);

    let cart = transfer_cart_approach_gauge();
    export(OUTPUTS[8], &cart);

    let bridge = camera_evidence_bridge();
    export(OUTPUTS[9], &bridge);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[10], &keepouts);

    let assembly = base
        + rack.translate(RACK_POS.0, RACK_POS.1, on_base_z(RACK_Z))
        + arcs.translate(ARC_POS.0, ARC_POS.1, on_base_z(ARC_PANEL_Z))
        + seal.translate(SEAL_POS.0, SEAL_POS.1, on_base_z(SEAL_PANEL_Z))
        + overhang.translate(OVERHANG_POS.0, OVERHANG_POS.1, on_base_z(OVERHANG_Z))
        + stops.translate(STOP_POS.0, STOP_POS.1, on_base_z(STOP_Z))
        + latch.translate(LATCH_POS.0, LATCH_POS.1, on_base_z(LATCH_Z))
        + custody.translate(CUSTODY_POS.0, CUSTODY_POS.1, on_base_z(CUSTODY_Z))
        + cart.translate(CART_POS.0, CART_POS.1, on_base_z(CART_Z))
        + bridge.translate(BRIDGE_POS.0, BRIDGE_POS.1, on_base_z(BRIDGE_Z))
        + keepouts.translate(0.0, 0.0, BASE_Z / 2.0 + KEEP_OUT_Z / 2.0);
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed incubator rack door-sweep clearance collision station:");
    println!("  Footprint:             {STATION_X:.0}mm x {STATION_Y:.0}mm closed validation tray");
    println!(
        "  Rack surrogate:        {RACK_SLOT_COUNT} cassette slots across {RACK_COLS}x{RACK_ROWS} rack datum with {RACK_GRIPPER_LAND_COUNT} gripper lands"
    );
    println!(
        "  Door sweep proof:      {DOOR_ARC_SEGMENTS} arc stations across {DOOR_SWING_DEG:.0}deg with {:.0}mm rack-to-sweep margin",
        rack_to_sweep_clearance()
    );
    println!(
        "  Seal/latch challenge:  {SEAL_TOOTH_COUNT} sweep-seal teeth, {OVERHANG_BLOCK_COUNT} overhang blocks, {STOP_COUPON_COUNT} stop coupons, {LATCH_TAB_COUNT} witness tabs"
    );
    println!(
        "  Trace/evidence:        {BARCODE_LAND_COUNT} barcode lands, {CUSTODY_TOKEN_COUNT} custody tokens, {CAMERA_COUNT} camera lands, {CAMERA_FIDUCIAL_COUNT} fiducials"
    );
    println!(
        "  Robot service gauges:  {ROBOT_KEEP_OUT_ZONE_COUNT} keepout zones and {SERVICE_GAUGE_POST_COUNT} service gauge posts"
    );
    println!("  Required features:     {}", REQUIRED_FEATURES.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn on_base_z(part_z: f64) -> f64 {
    BASE_Z / 2.0 + part_z / 2.0 - SOCKET_DEPTH / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 12);
    assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
    assert_eq!(REQUIRED_FEATURES.len(), 10);
    assert_eq!(RACK_SLOT_COUNT, RACK_COLS * RACK_ROWS);
    assert_eq!(MOUNT_SLOT_COUNT, mount_slot_positions().len());
    assert_eq!(DATUM_TARGET_COUNT, datum_target_positions().len());
    assert_eq!(CART_RAIL_COUNT, 2);
    assert_eq!(CART_WHEEL_CUP_COUNT, 4);
    assert!(DOOR_ARC_INNER_R < DOOR_ARC_CENTER_R);
    assert!(DOOR_ARC_CENTER_R < DOOR_ARC_OUTER_R);
    assert!(arc_endpoint_inside_panel(DOOR_ARC_OUTER_R, DOOR_SWING_DEG));
    assert!(rack_to_sweep_clearance() >= DOOR_CLEARANCE_MARGIN);
    assert!(front_service_clearance() >= FRONT_SERVICE_CLEARANCE);
    assert!(rear_camera_service_clearance() >= REAR_CAMERA_SERVICE_CLEARANCE);
    assert!(side_service_clearance() >= SIDE_SERVICE_CLEARANCE);
    assert!(latch_clearance_step() >= 1.0);

    for item in socket_rects() {
        assert!(item.fits_inside_rim(), "{} exceeds tray rim", item.name);
    }

    let rects = collision_sensitive_rects();
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

fn socket_rects() -> [Rect; 8] {
    [
        rect("rack_surrogate", RACK_POS, RACK_X, RACK_Y),
        rect("door_swing_arc_gauges", ARC_POS, ARC_PANEL_X, ARC_PANEL_Y),
        rect(
            "sweep_seal_contact_comb",
            SEAL_POS,
            SEAL_PANEL_X,
            SEAL_PANEL_Y,
        ),
        rect(
            "cassette_overhang_challenge_blocks",
            OVERHANG_POS,
            OVERHANG_X,
            OVERHANG_Y,
        ),
        rect("soft_hard_stop_coupons", STOP_POS, STOP_X, STOP_Y),
        rect("latch_clearance_witness_tabs", LATCH_POS, LATCH_X, LATCH_Y),
        rect("barcode_custody_lands", CUSTODY_POS, CUSTODY_X, CUSTODY_Y),
        rect("transfer_cart_approach_gauge", CART_POS, CART_X, CART_Y),
    ]
}

fn collision_sensitive_rects() -> [Rect; 8] {
    socket_rects()
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn rack_to_sweep_clearance() -> f64 {
    let rack_right_edge = RACK_POS.0 + RACK_X / 2.0;
    let sweep_left_edge = ARC_POS.0 - ARC_PANEL_X / 2.0;
    sweep_left_edge - rack_right_edge
}

fn arc_endpoint_inside_panel(radius: f64, angle_deg: f64) -> bool {
    let rad = angle_deg.to_radians();
    let x = HINGE_LOCAL.0 + radius * rad.cos();
    let y = HINGE_LOCAL.1 + radius * rad.sin();
    x.abs() <= ARC_PANEL_X / 2.0 - 12.0 && y.abs() <= ARC_PANEL_Y / 2.0 - 12.0
}

fn front_service_clearance() -> f64 {
    STATION_Y / 2.0 - (CUSTODY_POS.1.abs() + CUSTODY_Y / 2.0)
}

fn rear_camera_service_clearance() -> f64 {
    STATION_Y / 2.0 - (BRIDGE_POS.1 + BRIDGE_Y / 2.0)
}

fn side_service_clearance() -> f64 {
    STATION_X / 2.0 - (LATCH_POS.0 + LATCH_X / 2.0)
}

fn latch_clearance_step() -> f64 {
    (LATCH_MAX_CLEARANCE - LATCH_MIN_CLEARANCE) / (LATCH_TAB_COUNT as f64 - 1.0)
}

fn base_tray() -> Part {
    let deck = centered_cube(
        "door_sweep_collision_base_tray_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let witness_basin = centered_cube(
        "door_sweep_collision_secondary_witness_basin_cut",
        STATION_X - 126.0,
        STATION_Y - 120.0,
        7.0,
    )
    .translate(0.0, -12.0, BASE_Z - 3.5);
    let front_sweep_drain = centered_cylinder(
        "door_sweep_collision_front_witness_drain_placeholder",
        7.0,
        60.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 104.0,
        -STATION_Y / 2.0 + 26.0,
        BASE_Z - 7.0,
    );

    deck - witness_basin - front_sweep_drain - insert_sockets() - mount_slots()
        + perimeter_rims()
        + workflow_spines()
        + datum_targets()
        + hinge_reference_land()
        + deck_flow_witness_ribs()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("door_sweep_collision_insert_sockets");
    for item in socket_rects() {
        sockets = sockets
            + centered_cube(
                format!("door_sweep_collision_{}_locator_socket", item.name),
                item.x + 8.0,
                item.y + 8.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(item.center.0, item.center.1, BASE_Z - SOCKET_DEPTH / 2.0);
    }
    sockets
}

fn mount_slots() -> Part {
    let mut slots = Part::empty("door_sweep_collision_mount_slots");
    for (i, (x, y)) in mount_slot_positions().into_iter().enumerate() {
        let round = centered_cylinder(
            format!("door_sweep_collision_m6_round_mount_clearance_{i}"),
            3.5,
            BASE_Z + 4.0,
            28,
        )
        .translate(x, y, BASE_Z / 2.0);
        let slot = centered_cube(
            format!("door_sweep_collision_m6_oblong_mount_slot_{i}"),
            30.0,
            7.5,
            BASE_Z + 4.0,
        )
        .translate(x, y, BASE_Z / 2.0);
        slots = slots + round + slot;
    }
    slots
}

fn mount_slot_positions() -> [(f64, f64); MOUNT_SLOT_COUNT] {
    [
        (-STATION_X / 2.0 + 58.0, -STATION_Y / 2.0 + 58.0),
        (STATION_X / 2.0 - 58.0, -STATION_Y / 2.0 + 58.0),
        (-STATION_X / 2.0 + 58.0, STATION_Y / 2.0 - 58.0),
        (STATION_X / 2.0 - 58.0, STATION_Y / 2.0 - 58.0),
        (0.0, -STATION_Y / 2.0 + 58.0),
        (0.0, STATION_Y / 2.0 - 58.0),
        (-STATION_X / 2.0 + 58.0, 0.0),
        (STATION_X / 2.0 - 58.0, 0.0),
    ]
}

fn perimeter_rims() -> Part {
    let front = centered_cube(
        "door_sweep_collision_front_low_robot_access_rim",
        STATION_X,
        RIM_W,
        28.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, BASE_Z + 14.0);
    let rear = centered_cube(
        "door_sweep_collision_rear_camera_service_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "door_sweep_collision_left_containment_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "door_sweep_collision_right_latch_side_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);

    front + rear + left + right
}

fn workflow_spines() -> Part {
    let rear_evidence = centered_cube(
        "door_sweep_collision_rear_evidence_zone_spine",
        STATION_X - 220.0,
        10.0,
        24.0,
    )
    .translate(0.0, 286.0, BASE_Z + 12.0);
    let sweep_boundary = centered_cube(
        "door_sweep_collision_rack_to_sweep_boundary_spine",
        10.0,
        454.0,
        26.0,
    )
    .translate(-40.0, 14.0, BASE_Z + 13.0);
    let lower_trace = centered_cube(
        "door_sweep_collision_traceability_zone_spine",
        STATION_X - 260.0,
        9.0,
        22.0,
    )
    .translate(0.0, -256.0, BASE_Z + 11.0);

    rear_evidence + sweep_boundary + lower_trace
}

fn datum_targets() -> Part {
    let mut targets = Part::empty("door_sweep_collision_robot_datum_targets");
    for (i, (x, y)) in datum_target_positions().into_iter().enumerate() {
        targets = targets
            + fiducial_disc(&format!("door_sweep_collision_robot_datum_target_{i}")).translate(
                x,
                y,
                BASE_Z + 2.5,
            );
    }
    targets
}

fn datum_target_positions() -> [(f64, f64); DATUM_TARGET_COUNT] {
    [
        (-500.0, -350.0),
        (500.0, -350.0),
        (-500.0, 350.0),
        (500.0, 350.0),
        (-52.0, 286.0),
        (52.0, -256.0),
    ]
}

fn hinge_reference_land() -> Part {
    let world_hinge = (ARC_POS.0 + HINGE_LOCAL.0, ARC_POS.1 + HINGE_LOCAL.1);
    let pivot_boss = centered_cylinder(
        "door_sweep_collision_hinge_pivot_reference_boss",
        24.0,
        10.0,
        48,
    )
    .translate(world_hinge.0, world_hinge.1, BASE_Z + 5.0);
    let pivot_pin = centered_cylinder("door_sweep_collision_hinge_axis_pin_relief", 6.0, 12.0, 32)
        .translate(world_hinge.0, world_hinge.1, BASE_Z + 5.0);
    let hinge_label_land =
        centered_cube("door_sweep_collision_hinge_id_label_land", 96.0, 26.0, 4.0).translate(
            world_hinge.0 + 76.0,
            world_hinge.1 - 16.0,
            BASE_Z + 2.0,
        );

    (pivot_boss - pivot_pin) + hinge_label_land
}

fn deck_flow_witness_ribs() -> Part {
    let mut ribs = Part::empty("door_sweep_collision_deck_witness_ribs");
    for (i, y) in [-332.0, -214.0, -86.0, 44.0, 174.0, 300.0]
        .into_iter()
        .enumerate()
    {
        ribs = ribs
            + centered_cube(
                format!("door_sweep_collision_witness_rib_{i}"),
                STATION_X - 240.0,
                4.0,
                5.0,
            )
            .translate(0.0, y, BASE_Z + 2.5);
    }
    ribs
}

fn rack_surrogate() -> Part {
    let panel = module_panel("door_sweep_rack_surrogate_panel", RACK_X, RACK_Y, RACK_Z);
    panel - rack_slot_reliefs() - rack_finger_reliefs()
        + rack_reference_rails()
        + rack_cassette_surrogates()
        + rack_gripper_lands()
        + rack_door_side_witness_fence()
}

fn rack_slot_reliefs() -> Part {
    let mut reliefs = Part::empty("door_sweep_rack_slot_reliefs");
    for slot in 0..RACK_SLOT_COUNT {
        let (x, y) = rack_slot_center(slot);
        reliefs = reliefs
            + centered_cube(
                format!("door_sweep_rack_slot_{}_cassette_recess", slot + 1),
                CASSETTE_X + 16.0,
                CASSETTE_Y + 14.0,
                8.0,
            )
            .translate(x, y, RACK_Z / 2.0 - 3.8);
    }
    reliefs
}

fn rack_finger_reliefs() -> Part {
    let mut reliefs = Part::empty("door_sweep_rack_robot_finger_reliefs");
    for slot in 0..RACK_SLOT_COUNT {
        let (x, y) = rack_slot_center(slot);
        reliefs = reliefs
            + centered_cube(
                format!("door_sweep_rack_slot_{}_front_finger_cut", slot + 1),
                54.0,
                18.0,
                RACK_Z + 4.0,
            )
            .translate(x, y - CASSETTE_Y / 2.0 - 10.0, RACK_Z / 2.0);
    }
    reliefs
}

fn rack_reference_rails() -> Part {
    let left = centered_cube(
        "door_sweep_rack_fixed_left_reference_rail",
        16.0,
        RACK_Y,
        38.0,
    )
    .translate(-RACK_X / 2.0 + 20.0, 0.0, RACK_Z / 2.0 + 19.0);
    let rear = centered_cube(
        "door_sweep_rack_rear_hard_reference_rail",
        RACK_X,
        16.0,
        38.0,
    )
    .translate(0.0, RACK_Y / 2.0 - 20.0, RACK_Z / 2.0 + 19.0);
    let door_side_soft = centered_cube(
        "door_sweep_rack_door_side_compliant_capture_rail",
        12.0,
        RACK_Y - 64.0,
        24.0,
    )
    .translate(RACK_X / 2.0 - 25.0, -12.0, RACK_Z / 2.0 + 12.0);

    left + rear + door_side_soft
}

fn rack_cassette_surrogates() -> Part {
    let mut cassettes = Part::empty("door_sweep_rack_cassette_surrogates");
    for slot in 0..RACK_SLOT_COUNT {
        let (x, y) = rack_slot_center(slot);
        let cassette = centered_cube(
            format!("door_sweep_rack_slot_{}_cassette_surrogate", slot + 1),
            CASSETTE_X,
            CASSETTE_Y,
            CASSETTE_Z,
        )
        .translate(x, y, RACK_Z / 2.0 + CASSETTE_Z / 2.0);
        let gasket = centered_cube(
            format!("door_sweep_rack_slot_{}_raised_gasket_edge", slot + 1),
            CASSETTE_X + 12.0,
            6.0,
            8.0,
        )
        .translate(
            x,
            y + CASSETTE_Y / 2.0 + 6.0,
            RACK_Z / 2.0 + CASSETTE_Z + 4.0,
        );
        let overhang_face = centered_cube(
            format!("door_sweep_rack_slot_{}_door_side_overhang_face", slot + 1),
            7.0,
            CASSETTE_Y + 10.0,
            24.0,
        )
        .translate(
            x + CASSETTE_X / 2.0 + 8.0,
            y,
            RACK_Z / 2.0 + CASSETTE_Z / 2.0,
        );
        cassettes = cassettes + cassette + gasket + overhang_face;
    }
    cassettes
}

fn rack_gripper_lands() -> Part {
    let mut lands = Part::empty("door_sweep_rack_gripper_lands");
    for (i, (x, y)) in rack_gripper_land_points().into_iter().enumerate() {
        let land = centered_cube(format!("door_sweep_rack_gripper_land_{i}"), 44.0, 18.0, 6.0)
            .translate(x, y, RACK_Z / 2.0 + CASSETTE_Z + 3.0);
        let center_mark = centered_cube(
            format!("door_sweep_rack_gripper_land_centerline_cut_{i}"),
            4.0,
            22.0,
            7.0,
        )
        .translate(x, y, RACK_Z / 2.0 + CASSETTE_Z + 3.0);
        lands = lands + (land - center_mark);
    }
    lands
}

fn rack_gripper_land_points() -> [(f64, f64); RACK_GRIPPER_LAND_COUNT] {
    [
        (-164.0, -136.0),
        (-40.0, -136.0),
        (84.0, -136.0),
        (164.0, -62.0),
        (164.0, 62.0),
        (-164.0, 136.0),
        (-40.0, 136.0),
        (84.0, 136.0),
    ]
}

fn rack_door_side_witness_fence() -> Part {
    let fence = centered_cube(
        "door_sweep_rack_door_side_no_contact_witness_fence",
        10.0,
        RACK_Y - 44.0,
        70.0,
    )
    .translate(RACK_X / 2.0 - 2.0, -6.0, RACK_Z / 2.0 + 35.0);
    let mut tabs = Part::empty("door_sweep_rack_witness_fence_tabs");
    for (i, y) in [-120.0, -60.0, 0.0, 60.0, 120.0].into_iter().enumerate() {
        tabs = tabs
            + centered_cube(
                format!("door_sweep_rack_frangible_witness_tab_{i}"),
                24.0,
                6.0,
                28.0,
            )
            .translate(RACK_X / 2.0 + 8.0, y, RACK_Z / 2.0 + 36.0);
    }
    fence + tabs
}

fn rack_slot_center(slot: usize) -> (f64, f64) {
    let col = slot % RACK_COLS;
    let row = slot / RACK_COLS;
    (
        centered_index(col, RACK_COLS, RACK_SLOT_PITCH_X),
        centered_index(row, RACK_ROWS, RACK_SLOT_PITCH_Y),
    )
}

fn door_swing_arc_gauges() -> Part {
    let panel = module_panel(
        "door_sweep_arc_gauge_panel",
        ARC_PANEL_X,
        ARC_PANEL_Y,
        ARC_PANEL_Z,
    );
    panel - arc_lightening_window()
        + hinge_pivot_local()
        + arc_segments(DOOR_ARC_OUTER_R, "outer_hard_door_skin", 44.0)
        + arc_segments(DOOR_ARC_CENTER_R, "centerline_door_sweep", ARC_SEGMENT_Z)
        + arc_segments(DOOR_ARC_INNER_R, "inner_soft_seal", 24.0)
        + door_leaf_open_closed_gauges()
        + sweep_collision_witness_posts()
        + arc_angle_tick_lands()
}

fn arc_lightening_window() -> Part {
    centered_cube(
        "door_sweep_arc_gauge_lightening_window_cut",
        ARC_PANEL_X - 84.0,
        ARC_PANEL_Y - 86.0,
        9.0,
    )
    .translate(28.0, 26.0, ARC_PANEL_Z / 2.0 - 4.2)
}

fn hinge_pivot_local() -> Part {
    let boss = centered_cylinder("door_sweep_arc_hinge_pivot_boss", 26.0, 34.0, 56).translate(
        HINGE_LOCAL.0,
        HINGE_LOCAL.1,
        ARC_PANEL_Z / 2.0 + 17.0,
    );
    let bore = centered_cylinder("door_sweep_arc_hinge_axis_bore", 7.0, 38.0, 36).translate(
        HINGE_LOCAL.0,
        HINGE_LOCAL.1,
        ARC_PANEL_Z / 2.0 + 17.0,
    );
    let tangent_arrow = centered_cube(
        "door_sweep_arc_hinge_tangent_reference_arrow",
        94.0,
        8.0,
        8.0,
    )
    .rotate(0.0, 0.0, 18.0)
    .translate(
        HINGE_LOCAL.0 + 56.0,
        HINGE_LOCAL.1 + 28.0,
        ARC_PANEL_Z / 2.0 + 32.0,
    );

    (boss - bore) + tangent_arrow
}

fn arc_segments(radius: f64, label: &str, height: f64) -> Part {
    let mut segments = Part::empty(format!("door_sweep_arc_{label}_segments"));
    for i in 0..DOOR_ARC_SEGMENTS {
        let angle = 6.0 + i as f64 * (DOOR_SWING_DEG - 12.0) / (DOOR_ARC_SEGMENTS as f64 - 1.0);
        let (x, y) = arc_point(radius, angle);
        let tangent = angle + 90.0;
        segments = segments
            + centered_cube(
                format!("door_sweep_arc_{label}_segment_{i:02}"),
                ARC_SEGMENT_X,
                ARC_SEGMENT_Y,
                height,
            )
            .rotate(0.0, 0.0, tangent)
            .translate(x, y, ARC_PANEL_Z / 2.0 + height / 2.0);
    }
    segments
}

fn arc_point(radius: f64, angle_deg: f64) -> (f64, f64) {
    let rad = angle_deg.to_radians();
    (
        HINGE_LOCAL.0 + radius * rad.cos(),
        HINGE_LOCAL.1 + radius * rad.sin(),
    )
}

fn door_leaf_open_closed_gauges() -> Part {
    let closed = centered_cube(
        "door_sweep_arc_closed_door_leaf_gauge",
        DOOR_ARC_OUTER_R - 36.0,
        9.0,
        64.0,
    )
    .translate(
        HINGE_LOCAL.0 + (DOOR_ARC_OUTER_R - 36.0) / 2.0,
        HINGE_LOCAL.1,
        ARC_PANEL_Z / 2.0 + 32.0,
    );
    let open_len = DOOR_ARC_OUTER_R - 52.0;
    let open_angle = DOOR_SWING_DEG;
    let open_mid = arc_point(open_len / 2.0, open_angle);
    let open = centered_cube("door_sweep_arc_open_door_leaf_gauge", open_len, 9.0, 64.0)
        .rotate(0.0, 0.0, open_angle)
        .translate(open_mid.0, open_mid.1, ARC_PANEL_Z / 2.0 + 32.0);

    closed + open
}

fn sweep_collision_witness_posts() -> Part {
    let mut posts = Part::empty("door_sweep_arc_collision_witness_posts");
    for (i, angle) in [15.0, 30.0, 45.0, 60.0, 75.0, 90.0].into_iter().enumerate() {
        let (x, y) = arc_point(DOOR_ARC_INNER_R - 22.0, angle);
        let post = centered_cylinder(
            format!("door_sweep_arc_breakaway_witness_post_{i}"),
            7.0,
            58.0,
            28,
        )
        .translate(x, y, ARC_PANEL_Z / 2.0 + 29.0);
        let flag = centered_cube(
            format!("door_sweep_arc_breakaway_witness_flag_{i}"),
            22.0,
            4.0,
            26.0,
        )
        .rotate(0.0, 0.0, angle + 90.0)
        .translate(x, y, ARC_PANEL_Z / 2.0 + 62.0);
        posts = posts + post + flag;
    }
    posts
}

fn arc_angle_tick_lands() -> Part {
    let mut ticks = Part::empty("door_sweep_arc_angle_tick_lands");
    for (i, angle) in [0.0, 15.0, 30.0, 45.0, 60.0, 75.0, 90.0]
        .into_iter()
        .enumerate()
    {
        let (x, y) = arc_point(DOOR_ARC_OUTER_R + 8.0, angle);
        ticks = ticks
            + centered_cube(
                format!("door_sweep_arc_angle_tick_land_{i}"),
                28.0,
                5.0,
                5.0,
            )
            .rotate(0.0, 0.0, angle + 90.0)
            .translate(x, y, ARC_PANEL_Z / 2.0 + 2.5);
    }
    ticks
}

fn sweep_seal_contact_comb() -> Part {
    let panel = module_panel(
        "door_sweep_seal_contact_comb_panel",
        SEAL_PANEL_X,
        SEAL_PANEL_Y,
        SEAL_PANEL_Z,
    );
    let datum_fence = centered_cube(
        "door_sweep_seal_comb_rear_datum_fence",
        SEAL_PANEL_X - 36.0,
        9.0,
        38.0,
    )
    .translate(0.0, SEAL_PANEL_Y / 2.0 - 16.0, SEAL_PANEL_Z / 2.0 + 19.0);

    panel + datum_fence + seal_teeth() + seal_compression_rulers() + seal_witness_ink_lands()
}

fn seal_teeth() -> Part {
    let mut teeth = Part::empty("door_sweep_seal_contact_teeth");
    for i in 0..SEAL_TOOTH_COUNT {
        let x = centered_index(i, SEAL_TOOTH_COUNT, SEAL_TOOTH_PITCH);
        let tooth_height = SEAL_TOOTH_Z + (i % 3) as f64 * 5.0;
        let tooth = centered_cube(
            format!("door_sweep_seal_contact_tooth_{i:02}"),
            SEAL_TOOTH_X,
            SEAL_TOOTH_Y,
            tooth_height,
        )
        .translate(x, -4.0, SEAL_PANEL_Z / 2.0 + tooth_height / 2.0);
        let rounded_tip = centered_cylinder(
            format!("door_sweep_seal_contact_tooth_rounded_tip_{i:02}"),
            SEAL_TOOTH_X / 2.0,
            SEAL_TOOTH_Y,
            20,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -4.0, SEAL_PANEL_Z / 2.0 + tooth_height);
        teeth = teeth + tooth + rounded_tip;
    }
    teeth
}

fn seal_compression_rulers() -> Part {
    let mut rulers = Part::empty("door_sweep_seal_compression_rulers");
    for (i, x) in [-72.0, 0.0, 72.0].into_iter().enumerate() {
        let spine = centered_cube(
            format!("door_sweep_seal_compression_ruler_spine_{i}"),
            6.0,
            SEAL_PANEL_Y - 22.0,
            26.0,
        )
        .translate(x, 0.0, SEAL_PANEL_Z / 2.0 + 13.0);
        let mut ticks = Part::empty(format!("door_sweep_seal_compression_ruler_ticks_{i}"));
        for tick in 0..5 {
            ticks = ticks
                + centered_cube(
                    format!("door_sweep_seal_compression_ruler_{i}_tick_{tick}"),
                    28.0,
                    3.0,
                    5.0,
                )
                .translate(
                    x + 14.0,
                    -24.0 + tick as f64 * 12.0,
                    SEAL_PANEL_Z / 2.0 + 28.0,
                );
        }
        rulers = rulers + spine + ticks;
    }
    rulers
}

fn seal_witness_ink_lands() -> Part {
    let mut lands = Part::empty("door_sweep_seal_witness_ink_lands");
    for (i, x) in [-72.0, -43.0, -14.0, 14.0, 43.0, 72.0]
        .into_iter()
        .enumerate()
    {
        lands = lands
            + centered_cube(
                format!("door_sweep_seal_witness_ink_land_{i}"),
                34.0,
                18.0,
                4.0,
            )
            .translate(x, -SEAL_PANEL_Y / 2.0 + 17.0, SEAL_PANEL_Z / 2.0 + 2.0);
    }
    lands
}

fn cassette_overhang_challenge_blocks() -> Part {
    let panel = module_panel(
        "door_sweep_overhang_challenge_panel",
        OVERHANG_X,
        OVERHANG_Y,
        OVERHANG_Z,
    );
    panel + overhang_step_blocks() + overhang_lateral_feeler_slots() + overhang_gauge_ribs()
}

fn overhang_step_blocks() -> Part {
    let mut blocks = Part::empty("door_sweep_overhang_challenge_blocks");
    for i in 0..OVERHANG_BLOCK_COUNT {
        let x = centered_index(i, OVERHANG_BLOCK_COUNT, OVERHANG_STEP_X);
        let z = 24.0 + i as f64 * ((MAX_OVERHANG_Z - 24.0) / (OVERHANG_BLOCK_COUNT as f64 - 1.0));
        let block = centered_cube(
            format!("door_sweep_overhang_step_block_{i}"),
            34.0,
            OVERHANG_STEP_Y,
            z,
        )
        .translate(x, 4.0, OVERHANG_Z / 2.0 + z / 2.0);
        let witness_face = centered_cube(
            format!("door_sweep_overhang_witness_face_{i}"),
            5.0,
            OVERHANG_STEP_Y + 18.0,
            z + 10.0,
        )
        .translate(x + 22.0, 4.0, OVERHANG_Z / 2.0 + (z + 10.0) / 2.0);
        blocks = blocks + block + witness_face;
    }
    blocks
}

fn overhang_lateral_feeler_slots() -> Part {
    let mut slots = Part::empty("door_sweep_overhang_lateral_feeler_slots");
    for (i, y) in [-46.0, -24.0, 50.0].into_iter().enumerate() {
        let rail = centered_cube(
            format!("door_sweep_overhang_feeler_reference_rail_{i}"),
            OVERHANG_X - 64.0,
            5.0,
            18.0,
        )
        .translate(0.0, y, OVERHANG_Z / 2.0 + 9.0);
        slots = slots + rail;
    }
    slots
}

fn overhang_gauge_ribs() -> Part {
    let mut ribs = Part::empty("door_sweep_overhang_gauge_ribs");
    for i in 0..6 {
        ribs = ribs
            + centered_cube(
                format!("door_sweep_overhang_datum_rib_{i}"),
                4.0,
                OVERHANG_Y - 34.0,
                8.0,
            )
            .translate(-172.0 + i as f64 * 68.0, 0.0, OVERHANG_Z / 2.0 + 4.0);
    }
    ribs
}

fn soft_hard_stop_coupons() -> Part {
    let panel = module_panel(
        "door_sweep_soft_hard_stop_coupon_panel",
        STOP_X,
        STOP_Y,
        STOP_Z,
    );
    panel + stop_coupon_stack() + stop_overtravel_witness_pins()
}

fn stop_coupon_stack() -> Part {
    let mut coupons = Part::empty("door_sweep_stop_coupon_stack");
    for i in 0..STOP_COUPON_COUNT {
        let x = centered_index(i, STOP_COUPON_COUNT, 32.0);
        let is_hard = i >= STOP_COUPON_COUNT / 2;
        let z = if is_hard { HARD_STOP_Z } else { SOFT_STOP_Z };
        let y = if is_hard { 26.0 } else { -28.0 };
        let coupon = centered_cube(
            format!(
                "door_sweep_{}_stop_coupon_{i}",
                if is_hard { "hard" } else { "soft" }
            ),
            24.0,
            42.0,
            z,
        )
        .translate(x, y, STOP_Z / 2.0 + z / 2.0);
        let witness_cap = centered_cube(
            format!("door_sweep_stop_coupon_witness_cap_{i}"),
            26.0,
            5.0,
            10.0,
        )
        .translate(x, y - 23.0, STOP_Z / 2.0 + z + 5.0);
        coupons = coupons + coupon + witness_cap;
    }
    coupons
}

fn stop_overtravel_witness_pins() -> Part {
    let mut pins = Part::empty("door_sweep_stop_overtravel_witness_pins");
    for (i, x) in [-78.0, -26.0, 26.0, 78.0].into_iter().enumerate() {
        pins = pins
            + centered_cylinder(
                format!("door_sweep_stop_overtravel_break_pin_{i}"),
                5.0,
                48.0,
                24,
            )
            .translate(x, 0.0, STOP_Z / 2.0 + 24.0);
    }
    pins
}

fn latch_clearance_witness_tabs() -> Part {
    let panel = module_panel(
        "door_sweep_latch_clearance_witness_panel",
        LATCH_X,
        LATCH_Y,
        LATCH_Z,
    );
    panel + latch_tabs() + latch_pawl_envelope() + latch_feeler_storage()
}

fn latch_tabs() -> Part {
    let mut tabs = Part::empty("door_sweep_latch_clearance_tabs");
    for i in 0..LATCH_TAB_COUNT {
        let y = centered_index(i, LATCH_TAB_COUNT, 15.0);
        let clearance = LATCH_MIN_CLEARANCE + latch_clearance_step() * i as f64;
        let tab = centered_cube(
            format!("door_sweep_latch_clearance_{clearance:.0}mm_tab_{i}"),
            52.0 - clearance,
            5.0,
            34.0,
        )
        .translate(0.0, y, LATCH_Z / 2.0 + 17.0);
        let root = centered_cube(
            format!("door_sweep_latch_breakaway_root_{i}"),
            8.0,
            8.0,
            28.0,
        )
        .translate(-38.0, y, LATCH_Z / 2.0 + 14.0);
        tabs = tabs + tab + root;
    }
    tabs
}

fn latch_pawl_envelope() -> Part {
    let body = centered_cube("door_sweep_latch_pawl_envelope_block", 70.0, 32.0, 62.0).translate(
        54.0,
        0.0,
        LATCH_Z / 2.0 + 31.0,
    );
    let nose = centered_cylinder("door_sweep_latch_pawl_rounded_nose", 16.0, 38.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(88.0, 0.0, LATCH_Z / 2.0 + 48.0);
    body + nose
}

fn latch_feeler_storage() -> Part {
    let mut storage = Part::empty("door_sweep_latch_feeler_storage");
    for (i, y) in [-56.0, 56.0].into_iter().enumerate() {
        let pocket = centered_cube(
            format!("door_sweep_latch_feeler_blade_pocket_{i}"),
            108.0,
            12.0,
            5.0,
        )
        .translate(2.0, y, LATCH_Z / 2.0 + 2.5);
        storage = storage + pocket;
    }
    storage
}

fn barcode_custody_lands() -> Part {
    let panel = module_panel(
        "door_sweep_barcode_custody_panel",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    panel + barcode_lands() + custody_token_lanes() + release_hold_reject_lanes()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("door_sweep_barcode_lands");
    for i in 0..BARCODE_LAND_COUNT {
        let x = centered_index(i, BARCODE_LAND_COUNT, 47.0);
        lands = lands
            + centered_cube(format!("door_sweep_barcode_land_{i}"), 38.0, 26.0, 4.0).translate(
                x,
                22.0,
                CUSTODY_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn custody_token_lanes() -> Part {
    let mut lanes = Part::empty("door_sweep_custody_token_lanes");
    for i in 0..CUSTODY_TOKEN_COUNT {
        let x = centered_index(i, CUSTODY_TOKEN_COUNT, 55.0);
        let puck = centered_cylinder(
            format!("door_sweep_custody_token_pocket_{i}"),
            13.0,
            5.0,
            32,
        )
        .translate(x, -22.0, CUSTODY_Z / 2.0 + 2.5);
        lanes = lanes + puck;
    }
    lanes
}

fn release_hold_reject_lanes() -> Part {
    let mut lanes = Part::empty("door_sweep_release_hold_reject_lanes");
    for (i, x) in [-72.0, 0.0, 72.0].into_iter().enumerate() {
        let lane = centered_cube(format!("door_sweep_disposition_lane_{i}"), 54.0, 10.0, 14.0)
            .translate(x, -CUSTODY_Y / 2.0 + 10.0, CUSTODY_Z / 2.0 + 7.0);
        lanes = lanes + lane;
    }
    lanes
}

fn transfer_cart_approach_gauge() -> Part {
    let panel = module_panel(
        "door_sweep_transfer_cart_approach_panel",
        CART_X,
        CART_Y,
        CART_Z,
    );
    panel + cart_approach_rails() + cart_wheel_cups() + cart_bumper_witness_fence()
}

fn cart_approach_rails() -> Part {
    let mut rails = Part::empty("door_sweep_transfer_cart_approach_rails");
    for (i, y) in [-22.0, 22.0].into_iter().enumerate() {
        rails = rails
            + centered_cube(
                format!("door_sweep_transfer_cart_centering_rail_{i}"),
                CART_X - 44.0,
                8.0,
                36.0,
            )
            .translate(0.0, y, CART_Z / 2.0 + 18.0);
    }
    rails
}

fn cart_wheel_cups() -> Part {
    let mut cups = Part::empty("door_sweep_transfer_cart_wheel_cups");
    for (i, (x, y)) in [
        (-102.0, -26.0),
        (102.0, -26.0),
        (-102.0, 26.0),
        (102.0, 26.0),
    ]
    .into_iter()
    .enumerate()
    {
        let cup = centered_cylinder(
            format!("door_sweep_transfer_cart_wheel_cup_{i}"),
            18.0,
            8.0,
            36,
        )
        .translate(x, y, CART_Z / 2.0 + 4.0);
        let axle_line = centered_cube(
            format!("door_sweep_transfer_cart_wheel_cup_axle_line_{i}"),
            36.0,
            3.0,
            9.0,
        )
        .translate(x, y, CART_Z / 2.0 + 5.0);
        cups = cups + cup - axle_line;
    }
    cups
}

fn cart_bumper_witness_fence() -> Part {
    centered_cube(
        "door_sweep_transfer_cart_bumper_witness_fence",
        CART_X - 52.0,
        10.0,
        48.0,
    )
    .translate(0.0, CART_Y / 2.0 - 14.0, CART_Z / 2.0 + 24.0)
}

fn camera_evidence_bridge() -> Part {
    let left_post = centered_cube(
        "door_sweep_camera_bridge_left_post",
        36.0,
        BRIDGE_Y,
        BRIDGE_Z,
    )
    .translate(-BRIDGE_X / 2.0 + 42.0, 0.0, BRIDGE_Z / 2.0);
    let right_post = centered_cube(
        "door_sweep_camera_bridge_right_post",
        36.0,
        BRIDGE_Y,
        BRIDGE_Z,
    )
    .translate(BRIDGE_X / 2.0 - 42.0, 0.0, BRIDGE_Z / 2.0);
    let beam = centered_cube(
        "door_sweep_camera_bridge_overhead_beam",
        BRIDGE_X,
        32.0,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, 0.0, BRIDGE_Z + BRIDGE_BEAM_Z / 2.0);
    let camera_bar = centered_cube(
        "door_sweep_camera_bridge_camera_mount_bar",
        BRIDGE_X - 150.0,
        16.0,
        14.0,
    )
    .translate(0.0, -BRIDGE_Y / 2.0 + 18.0, BRIDGE_Z + BRIDGE_BEAM_Z + 7.0);

    left_post + right_post + beam + camera_bar + camera_mounts() + camera_fiducials()
}

fn camera_mounts() -> Part {
    let mut mounts = Part::empty("door_sweep_camera_mounts");
    for i in 0..CAMERA_COUNT {
        let x = centered_index(i, CAMERA_COUNT, 235.0);
        let block = centered_cube(
            format!("door_sweep_camera_mount_block_{i}"),
            58.0,
            24.0,
            24.0,
        )
        .translate(x, -BRIDGE_Y / 2.0 + 18.0, BRIDGE_Z + BRIDGE_BEAM_Z + 26.0);
        let lens = centered_cylinder(
            format!("door_sweep_camera_lens_sight_bore_{i}"),
            9.0,
            28.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -BRIDGE_Y / 2.0 + 8.0, BRIDGE_Z + BRIDGE_BEAM_Z + 26.0);
        mounts = mounts + (block - lens);
    }
    mounts
}

fn camera_fiducials() -> Part {
    let mut fiducials = Part::empty("door_sweep_camera_evidence_fiducials");
    for i in 0..CAMERA_FIDUCIAL_COUNT {
        let x = centered_index(i, CAMERA_FIDUCIAL_COUNT, 118.0);
        fiducials = fiducials
            + fiducial_disc(&format!("door_sweep_camera_evidence_fiducial_{i}")).translate(
                x,
                BRIDGE_Y / 2.0 - 12.0,
                BRIDGE_Z + BRIDGE_BEAM_Z + 4.0,
            );
    }
    fiducials
}

fn robot_service_keepout_gauges() -> Part {
    let front_robot_lane = centered_cube(
        "door_sweep_robot_front_approach_keepout_gauge",
        KEEP_OUT_X,
        10.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, -KEEP_OUT_Y / 2.0, 0.0);
    let rear_service_lane = centered_cube(
        "door_sweep_rear_service_keepout_gauge",
        KEEP_OUT_X,
        10.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, KEEP_OUT_Y / 2.0, 0.0);
    let left_side_lane = centered_cube(
        "door_sweep_left_side_service_keepout_gauge",
        10.0,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(-KEEP_OUT_X / 2.0, 0.0, 0.0);
    let right_side_lane = centered_cube(
        "door_sweep_right_latch_service_keepout_gauge",
        10.0,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(KEEP_OUT_X / 2.0, 0.0, 0.0);

    front_robot_lane
        + rear_service_lane
        + left_side_lane
        + right_side_lane
        + robot_zone_markers()
        + service_gauge_posts()
        + gripper_sweep_height_combs()
}

fn robot_zone_markers() -> Part {
    let mut markers = Part::empty("door_sweep_robot_zone_markers");
    for i in 0..ROBOT_KEEP_OUT_ZONE_COUNT {
        let x = centered_index(i, ROBOT_KEEP_OUT_ZONE_COUNT, 176.0);
        markers = markers
            + centered_cube(
                format!("door_sweep_robot_zone_marker_{i}"),
                62.0,
                16.0,
                18.0,
            )
            .translate(x, -KEEP_OUT_Y / 2.0 + 28.0, KEEP_OUT_Z / 2.0 + 9.0);
    }
    markers
}

fn service_gauge_posts() -> Part {
    let mut posts = Part::empty("door_sweep_service_gauge_posts");
    for (i, (x, y)) in [
        (-500.0, -340.0),
        (-250.0, -340.0),
        (250.0, -340.0),
        (500.0, -340.0),
        (-500.0, 340.0),
        (-250.0, 340.0),
        (250.0, 340.0),
        (500.0, 340.0),
    ]
    .into_iter()
    .enumerate()
    {
        posts = posts
            + centered_cylinder(format!("door_sweep_service_gauge_post_{i}"), 8.0, 86.0, 24)
                .translate(x, y, KEEP_OUT_Z / 2.0 + 43.0);
    }
    posts
}

fn gripper_sweep_height_combs() -> Part {
    let mut combs = Part::empty("door_sweep_gripper_height_combs");
    for (row, y) in [-122.0, 122.0].into_iter().enumerate() {
        for col in 0..5 {
            let x = centered_index(col, 5, 64.0);
            let z = 30.0 + col as f64 * 10.0;
            combs = combs
                + centered_cube(
                    format!("door_sweep_gripper_height_comb_r{row}_c{col}"),
                    18.0,
                    10.0,
                    z,
                )
                .translate(x, y, KEEP_OUT_Z / 2.0 + z / 2.0);
        }
    }
    combs
}

fn module_panel(name: &str, x: f64, y: f64, z: f64) -> Part {
    let panel = centered_cube(name, x, y, z);
    let chamfer_reliefs = panel_corner_reliefs(name, x, y, z);
    let label_land = centered_cube(format!("{name}_front_label_land"), x - 46.0, 18.0, 3.0)
        .translate(0.0, -y / 2.0 + 18.0, z / 2.0 + 1.5);
    let datum_bosses = module_datum_bosses(name, x, y, z);

    panel - chamfer_reliefs + label_land + datum_bosses
}

fn panel_corner_reliefs(name: &str, x: f64, y: f64, z: f64) -> Part {
    let mut reliefs = Part::empty(format!("{name}_corner_reliefs"));
    for (i, (dx, dy)) in [
        (-x / 2.0 + 16.0, -y / 2.0 + 16.0),
        (x / 2.0 - 16.0, -y / 2.0 + 16.0),
        (-x / 2.0 + 16.0, y / 2.0 - 16.0),
        (x / 2.0 - 16.0, y / 2.0 - 16.0),
    ]
    .into_iter()
    .enumerate()
    {
        reliefs = reliefs
            + centered_cylinder(format!("{name}_corner_mount_relief_{i}"), 4.0, z + 3.0, 24)
                .translate(dx, dy, 0.0);
    }
    reliefs
}

fn module_datum_bosses(name: &str, x: f64, y: f64, z: f64) -> Part {
    let mut bosses = Part::empty(format!("{name}_datum_bosses"));
    for (i, (dx, dy)) in [
        (-x / 2.0 + 26.0, y / 2.0 - 26.0),
        (x / 2.0 - 26.0, y / 2.0 - 26.0),
    ]
    .into_iter()
    .enumerate()
    {
        let boss = centered_cylinder(format!("{name}_datum_boss_{i}"), 9.0, 6.0, 28).translate(
            dx,
            dy,
            z / 2.0 + 3.0,
        );
        let hole = centered_cylinder(format!("{name}_datum_center_mark_{i}"), 2.2, 7.0, 20)
            .translate(dx, dy, z / 2.0 + 3.0);
        bosses = bosses + (boss - hole);
    }
    bosses
}

fn fiducial_disc(name: &str) -> Part {
    let disc = centered_cylinder(format!("{name}_disc"), 12.0, 4.0, 36);
    let x_cut = centered_cube(format!("{name}_x_cut"), 22.0, 3.0, 5.0);
    let y_cut = centered_cube(format!("{name}_y_cut"), 3.0, 22.0, 5.0);
    disc - x_cut - y_cut
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_names_are_complete_and_scoped() {
        assert_eq!(OUTPUTS.len(), 12);
        assert!(OUTPUTS.iter().all(|path| path.starts_with("output/")));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
        assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
    }

    #[test]
    fn fixture_covers_required_validation_surfaces() {
        for required in [
            "rack_surrogate",
            "door_swing_arc_gauges",
            "sweep_seal_contact_comb",
            "cassette_overhang_challenge_blocks",
            "soft_hard_stop_coupons",
            "latch_clearance_witness_tabs",
            "barcode_custody_lands",
            "camera_evidence_bridge",
            "transfer_cart_approach_gauge",
            "robot_service_keepout_gauges",
        ] {
            assert!(REQUIRED_FEATURES.contains(&required));
        }
    }

    #[test]
    fn module_sockets_fit_and_do_not_overlap() {
        for item in socket_rects() {
            assert!(item.fits_inside_rim(), "{} exceeds deck rim", item.name);
        }

        let rects = collision_sensitive_rects();
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

    #[test]
    fn door_sweep_has_clearance_from_rack_surrogate() {
        assert!(rack_to_sweep_clearance() >= DOOR_CLEARANCE_MARGIN);
        assert!(arc_endpoint_inside_panel(DOOR_ARC_OUTER_R, DOOR_SWING_DEG));
        assert!(DOOR_ARC_INNER_R < DOOR_ARC_CENTER_R);
        assert!(DOOR_ARC_CENTER_R < DOOR_ARC_OUTER_R);
    }

    #[test]
    fn service_and_latch_gauges_have_measurable_margins() {
        assert!(front_service_clearance() >= FRONT_SERVICE_CLEARANCE);
        assert!(rear_camera_service_clearance() >= REAR_CAMERA_SERVICE_CLEARANCE);
        assert!(side_service_clearance() >= SIDE_SERVICE_CLEARANCE);
        assert!(latch_clearance_step() >= 1.0);
    }
}
