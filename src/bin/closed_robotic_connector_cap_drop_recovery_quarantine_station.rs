use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed robotic connector cap drop recovery quarantine station.
//
// Intent:
// - Validate cap custody and recovery behavior when a sterile connector cap is
//   dropped, misplaced, or cannot be confidently returned inside a closed
//   robotic workcell.
// - Keep clean cap nests, dropped-cap quarantine, robotic recovery envelope
//   gauges, contamination witness coupons, replacement-cap release, open-time
//   token tracking, and release/hold/reject gates physically separated.
// - This is validation CAD only. It does not define sterility acceptance
//   criteria, open-time limits, decontamination chemistry, or process release
//   rules.

const OUTPUT_PREFIX: &str = "closed_robotic_connector_cap_drop_recovery_quarantine_station";

const OUTPUTS: [&str; 12] = [
    "output/closed_robotic_connector_cap_drop_recovery_quarantine_station_base_containment_deck.stl",
    "output/closed_robotic_connector_cap_drop_recovery_quarantine_station_cap_custody_nest_bank.stl",
    "output/closed_robotic_connector_cap_drop_recovery_quarantine_station_open_connector_surrogate_nests.stl",
    "output/closed_robotic_connector_cap_drop_recovery_quarantine_station_dropped_cap_quarantine_tray.stl",
    "output/closed_robotic_connector_cap_drop_recovery_quarantine_station_robotic_recovery_envelope_gauges.stl",
    "output/closed_robotic_connector_cap_drop_recovery_quarantine_station_contamination_witness_coupon_pockets.stl",
    "output/closed_robotic_connector_cap_drop_recovery_quarantine_station_replacement_cap_release_lane.stl",
    "output/closed_robotic_connector_cap_drop_recovery_quarantine_station_connector_open_time_token_rail.stl",
    "output/closed_robotic_connector_cap_drop_recovery_quarantine_station_release_hold_reject_gates.stl",
    "output/closed_robotic_connector_cap_drop_recovery_quarantine_station_evidence_camera_lighting_bridge.stl",
    "output/closed_robotic_connector_cap_drop_recovery_quarantine_station_robot_service_keepout_gauges.stl",
    "output/closed_robotic_connector_cap_drop_recovery_quarantine_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 10] = [
    "cap_custody_nests",
    "dropped_cap_quarantine_tray",
    "robotic_recovery_envelope_gauges",
    "contamination_witness_coupon_pockets",
    "replacement_cap_release_lane",
    "connector_open_time_token_rail",
    "release_hold_reject_gates",
    "open_connector_surrogate_nests",
    "evidence_camera_lighting_bridge",
    "robot_service_keepout_gauges",
];

const STATION_X: f64 = 1320.0;
const STATION_Y: f64 = 900.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 46.0;
const SOCKET_DEPTH: f64 = 6.0;
const EDGE_MARGIN: f64 = 12.0;
const MOUNT_HOLE_D: f64 = 6.6;

const CONNECTOR_ROWS: usize = 4;
const CONNECTOR_COLS: usize = 4;
const CONNECTOR_PORT_COUNT: usize = CONNECTOR_ROWS * CONNECTOR_COLS;
const CONNECTOR_NEST_X: f64 = 310.0;
const CONNECTOR_NEST_Y: f64 = 205.0;
const CONNECTOR_NEST_Z: f64 = 46.0;
const CONNECTOR_NEST_POS: (f64, f64) = (-35.0, 255.0);
const CONNECTOR_PITCH_X: f64 = 62.0;
const CONNECTOR_PITCH_Y: f64 = 42.0;
const OPEN_PORT_BORE_D: f64 = 17.0;

const CAP_CUSTODY_ROWS: usize = 4;
const CAP_CUSTODY_COLS: usize = 4;
const CAP_CUSTODY_COUNT: usize = CAP_CUSTODY_ROWS * CAP_CUSTODY_COLS;
const CAP_NEST_X: f64 = 340.0;
const CAP_NEST_Y: f64 = 205.0;
const CAP_NEST_Z: f64 = 42.0;
const CAP_NEST_POS: (f64, f64) = (-420.0, 255.0);
const CAP_PITCH_X: f64 = 72.0;
const CAP_PITCH_Y: f64 = 42.0;
const CAP_SOCKET_D: f64 = 24.0;

const REPLACEMENT_CAP_COUNT: usize = 8;
const REPLACEMENT_LANE_X: f64 = 340.0;
const REPLACEMENT_LANE_Y: f64 = 205.0;
const REPLACEMENT_LANE_Z: f64 = 38.0;
const REPLACEMENT_LANE_POS: (f64, f64) = (380.0, 255.0);
const REPLACEMENT_PITCH_X: f64 = 68.0;
const REPLACEMENT_PITCH_Y: f64 = 58.0;

const QUARANTINE_WELL_COUNT: usize = 12;
const QUARANTINE_TRAY_X: f64 = 360.0;
const QUARANTINE_TRAY_Y: f64 = 210.0;
const QUARANTINE_TRAY_Z: f64 = 50.0;
const QUARANTINE_TRAY_POS: (f64, f64) = (-430.0, 15.0);
const QUARANTINE_PITCH_X: f64 = 72.0;
const QUARANTINE_PITCH_Y: f64 = 54.0;
const QUARANTINE_WELL_D: f64 = 31.0;

const RECOVERY_GAUGE_X: f64 = 330.0;
const RECOVERY_GAUGE_Y: f64 = 220.0;
const RECOVERY_GAUGE_Z: f64 = 34.0;
const RECOVERY_GAUGE_POS: (f64, f64) = (-20.0, 15.0);
const RECOVERY_ENVELOPE_STATIONS: usize = 5;
const GRIPPER_JAW_GAUGES: usize = 4;
const MAX_RECOVERY_REACH_MM: f64 = 265.0;
const MAX_RECOVERY_PICK_HEIGHT_MM: f64 = 126.0;

const COUPON_POCKET_COUNT: usize = 12;
const COUPON_POCKET_X: f64 = 340.0;
const COUPON_POCKET_Y: f64 = 220.0;
const COUPON_POCKET_Z: f64 = 30.0;
const COUPON_POCKET_POS: (f64, f64) = (395.0, 15.0);
const COUPON_PITCH_X: f64 = 72.0;
const COUPON_PITCH_Y: f64 = 48.0;

const TOKEN_RAIL_X: f64 = 430.0;
const TOKEN_RAIL_Y: f64 = 150.0;
const TOKEN_RAIL_Z: f64 = 34.0;
const TOKEN_RAIL_POS: (f64, f64) = (-410.0, -245.0);
const OPEN_TIME_TOKEN_COUNT: usize = CONNECTOR_PORT_COUNT;
const TOKEN_COLS: usize = 8;
const TOKEN_PITCH_X: f64 = 46.0;
const TOKEN_PITCH_Y: f64 = 52.0;

const GATE_X: f64 = 420.0;
const GATE_Y: f64 = 160.0;
const GATE_Z: f64 = 42.0;
const GATE_POS: (f64, f64) = (90.0, -245.0);
const STATUS_GATE_COUNT: usize = 3;
const GATE_SLOTS_PER_LANE: usize = 5;
const GATE_SLOT_COUNT: usize = STATUS_GATE_COUNT * GATE_SLOTS_PER_LANE;

const KEEP_OUT_X: f64 = 220.0;
const KEEP_OUT_Y: f64 = 170.0;
const KEEP_OUT_Z: f64 = 28.0;
const KEEP_OUT_POS: (f64, f64) = (500.0, -245.0);
const ROBOT_KEEP_OUT_FLAGS: usize = 6;
const SERVICE_KEEP_OUT_FLAGS: usize = 4;
const ROBOT_CLEARANCE_Z: f64 = 185.0;
const SERVICE_CLEARANCE_Z: f64 = 132.0;

const BRIDGE_X: f64 = 1120.0;
const BRIDGE_Y: f64 = 34.0;
const BRIDGE_POS: (f64, f64) = (0.0, 396.0);
const BRIDGE_SPAN_X: f64 = 1060.0;
const BRIDGE_SPAN_Y: f64 = 690.0;
const BRIDGE_POST_Z: f64 = 190.0;
const BRIDGE_BEAM_Z: f64 = 24.0;
const CAMERA_CLEARANCE_Z: f64 = 148.0;
const CAMERA_COUNT: usize = 4;
const LIGHT_BAR_COUNT: usize = 2;

#[derive(Clone, Copy, Debug)]
struct Rect {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_containment_deck();
    export(OUTPUTS[0], &base);

    let cap_nests = cap_custody_nest_bank();
    export(OUTPUTS[1], &cap_nests);

    let connector_nests = open_connector_surrogate_nests();
    export(OUTPUTS[2], &connector_nests);

    let quarantine = dropped_cap_quarantine_tray();
    export(OUTPUTS[3], &quarantine);

    let recovery = robotic_recovery_envelope_gauges();
    export(OUTPUTS[4], &recovery);

    let coupons = contamination_witness_coupon_pockets();
    export(OUTPUTS[5], &coupons);

    let replacement = replacement_cap_release_lane();
    export(OUTPUTS[6], &replacement);

    let tokens = connector_open_time_token_rail();
    export(OUTPUTS[7], &tokens);

    let gates = release_hold_reject_gates();
    export(OUTPUTS[8], &gates);

    let bridge = evidence_camera_lighting_bridge();
    export(OUTPUTS[9], &bridge);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[10], &keepouts);

    let assembly = base
        + cap_nests.translate(CAP_NEST_POS.0, CAP_NEST_POS.1, insert_z(CAP_NEST_Z))
        + connector_nests.translate(
            CONNECTOR_NEST_POS.0,
            CONNECTOR_NEST_POS.1,
            insert_z(CONNECTOR_NEST_Z),
        )
        + replacement.translate(
            REPLACEMENT_LANE_POS.0,
            REPLACEMENT_LANE_POS.1,
            insert_z(REPLACEMENT_LANE_Z),
        )
        + quarantine.translate(
            QUARANTINE_TRAY_POS.0,
            QUARANTINE_TRAY_POS.1,
            insert_z(QUARANTINE_TRAY_Z),
        )
        + recovery.translate(
            RECOVERY_GAUGE_POS.0,
            RECOVERY_GAUGE_POS.1,
            insert_z(RECOVERY_GAUGE_Z),
        )
        + coupons.translate(
            COUPON_POCKET_POS.0,
            COUPON_POCKET_POS.1,
            insert_z(COUPON_POCKET_Z),
        )
        + tokens.translate(TOKEN_RAIL_POS.0, TOKEN_RAIL_POS.1, insert_z(TOKEN_RAIL_Z))
        + gates.translate(GATE_POS.0, GATE_POS.1, insert_z(GATE_Z))
        + keepouts.translate(KEEP_OUT_POS.0, KEEP_OUT_POS.1, insert_z(KEEP_OUT_Z))
        + bridge.translate(
            BRIDGE_POS.0,
            BRIDGE_POS.1,
            BASE_Z / 2.0 + BRIDGE_POST_Z / 2.0,
        );
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed robotic connector cap drop recovery quarantine station:");
    println!("  Output prefix:          {OUTPUT_PREFIX}");
    println!("  Footprint:              {STATION_X:.0}mm x {STATION_Y:.0}mm containment deck");
    println!(
        "  Cap custody:            {CAP_CUSTODY_COUNT} cap nests paired to {CONNECTOR_PORT_COUNT} open connector surrogates"
    );
    println!(
        "  Drop recovery:          {QUARANTINE_WELL_COUNT} quarantine wells, {RECOVERY_ENVELOPE_STATIONS} envelope gauges, {GRIPPER_JAW_GAUGES} gripper jaw gauges"
    );
    println!(
        "  Contamination witness:  {COUPON_POCKET_COUNT} coupon pockets and {OPEN_TIME_TOKEN_COUNT} connector open-time tokens"
    );
    println!(
        "  Replacement handling:   {REPLACEMENT_CAP_COUNT} clean replacement cap positions and {STATUS_GATE_COUNT} release/hold/reject gates"
    );
    println!(
        "  Evidence/automation:    {CAMERA_COUNT} camera pods, {LIGHT_BAR_COUNT} light bars, {ROBOT_KEEP_OUT_FLAGS} robot keepout flags"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn insert_z(height: f64) -> f64 {
    BASE_Z / 2.0 + height / 2.0
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 12, "export count changed");
    assert_eq!(
        REQUIRED_FEATURES.len(),
        10,
        "required feature count changed"
    );
    assert_eq!(CAP_CUSTODY_COUNT, CONNECTOR_PORT_COUNT);
    assert_eq!(OPEN_TIME_TOKEN_COUNT, CONNECTOR_PORT_COUNT);
    assert!(QUARANTINE_WELL_COUNT >= REPLACEMENT_CAP_COUNT);
    assert!(COUPON_POCKET_COUNT >= QUARANTINE_WELL_COUNT);
    assert_eq!(STATUS_GATE_COUNT, 3);
    assert!(GATE_SLOT_COUNT >= QUARANTINE_WELL_COUNT);
    assert!(MAX_RECOVERY_REACH_MM < RECOVERY_GAUGE_X);
    assert!(MAX_RECOVERY_PICK_HEIGHT_MM < BRIDGE_POST_Z);
    assert!(BRIDGE_POST_Z > CAMERA_CLEARANCE_Z);
    assert!(ROBOT_CLEARANCE_Z > SERVICE_CLEARANCE_Z);

    let rects = layout_rects();
    for (name, rect) in rects {
        assert!(
            fits_on_station(rect),
            "{name} exceeds station envelope: {rect:?}"
        );
    }
    for (left_index, (left_name, left)) in rects.iter().enumerate() {
        for (right_name, right) in rects.iter().skip(left_index + 1) {
            assert!(
                !rects_overlap(*left, *right),
                "{left_name} overlaps {right_name}"
            );
        }
    }
}

fn layout_rects() -> [(&'static str, Rect); 10] {
    [
        (
            "cap_custody_nest_bank",
            Rect {
                x: CAP_NEST_POS.0,
                y: CAP_NEST_POS.1,
                w: CAP_NEST_X,
                h: CAP_NEST_Y,
            },
        ),
        (
            "open_connector_surrogate_nests",
            Rect {
                x: CONNECTOR_NEST_POS.0,
                y: CONNECTOR_NEST_POS.1,
                w: CONNECTOR_NEST_X,
                h: CONNECTOR_NEST_Y,
            },
        ),
        (
            "replacement_cap_release_lane",
            Rect {
                x: REPLACEMENT_LANE_POS.0,
                y: REPLACEMENT_LANE_POS.1,
                w: REPLACEMENT_LANE_X,
                h: REPLACEMENT_LANE_Y,
            },
        ),
        (
            "dropped_cap_quarantine_tray",
            Rect {
                x: QUARANTINE_TRAY_POS.0,
                y: QUARANTINE_TRAY_POS.1,
                w: QUARANTINE_TRAY_X,
                h: QUARANTINE_TRAY_Y,
            },
        ),
        (
            "robotic_recovery_envelope_gauges",
            Rect {
                x: RECOVERY_GAUGE_POS.0,
                y: RECOVERY_GAUGE_POS.1,
                w: RECOVERY_GAUGE_X,
                h: RECOVERY_GAUGE_Y,
            },
        ),
        (
            "contamination_witness_coupon_pockets",
            Rect {
                x: COUPON_POCKET_POS.0,
                y: COUPON_POCKET_POS.1,
                w: COUPON_POCKET_X,
                h: COUPON_POCKET_Y,
            },
        ),
        (
            "connector_open_time_token_rail",
            Rect {
                x: TOKEN_RAIL_POS.0,
                y: TOKEN_RAIL_POS.1,
                w: TOKEN_RAIL_X,
                h: TOKEN_RAIL_Y,
            },
        ),
        (
            "release_hold_reject_gates",
            Rect {
                x: GATE_POS.0,
                y: GATE_POS.1,
                w: GATE_X,
                h: GATE_Y,
            },
        ),
        (
            "robot_service_keepout_gauges",
            Rect {
                x: KEEP_OUT_POS.0,
                y: KEEP_OUT_POS.1,
                w: KEEP_OUT_X,
                h: KEEP_OUT_Y,
            },
        ),
        (
            "evidence_camera_lighting_bridge",
            Rect {
                x: BRIDGE_POS.0,
                y: BRIDGE_POS.1,
                w: BRIDGE_X,
                h: BRIDGE_Y,
            },
        ),
    ]
}

fn fits_on_station(rect: Rect) -> bool {
    rect.x.abs() + rect.w / 2.0 <= STATION_X / 2.0 - RIM_W - EDGE_MARGIN
        && rect.y.abs() + rect.h / 2.0 <= STATION_Y / 2.0 - RIM_W - EDGE_MARGIN
}

fn rects_overlap(a: Rect, b: Rect) -> bool {
    let a_left = a.x - a.w / 2.0;
    let a_right = a.x + a.w / 2.0;
    let a_bottom = a.y - a.h / 2.0;
    let a_top = a.y + a.h / 2.0;
    let b_left = b.x - b.w / 2.0;
    let b_right = b.x + b.w / 2.0;
    let b_bottom = b.y - b.h / 2.0;
    let b_top = b.y + b.h / 2.0;

    !(a_right <= b_left || b_right <= a_left || a_top <= b_bottom || b_top <= a_bottom)
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn grid_xy(index: usize, cols: usize, rows: usize, pitch_x: f64, pitch_y: f64) -> (f64, f64) {
    let col = index % cols;
    let row = index / cols;
    (
        centered_index(col, cols, pitch_x),
        centered_index(row, rows, pitch_y),
    )
}

fn base_containment_deck() -> Part {
    let deck = centered_cube(
        "cap_drop_recovery_base_containment_deck_floor",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let low_point_recess = centered_cube(
        "cap_drop_recovery_base_low_point_recess",
        STATION_X - 140.0,
        STATION_Y - 132.0,
        7.0,
    )
    .translate(0.0, -12.0, BASE_Z / 2.0 - 3.5);
    let front_drain_witness_slot = centered_cube(
        "cap_drop_recovery_base_front_drain_witness_slot",
        130.0,
        14.0,
        9.0,
    )
    .translate(
        STATION_X / 2.0 - 150.0,
        -STATION_Y / 2.0 + 30.0,
        BASE_Z / 2.0 - 4.0,
    );

    deck - low_point_recess - front_drain_witness_slot - insert_sockets() - mounting_slots()
        + perimeter_rims()
        + workflow_dividers()
        + deck_fiducials()
        + flow_arrow_datums()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("cap_drop_recovery_base_insert_sockets");
    for (name, rect) in layout_rects() {
        sockets = sockets
            + centered_cube(
                format!("cap_drop_recovery_{name}_socket"),
                rect.w + 8.0,
                rect.h + 8.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(rect.x, rect.y, BASE_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.2);
    }
    sockets
}

fn mounting_slots() -> Part {
    let mut slots = Part::empty("cap_drop_recovery_base_mounting_slots");
    for (index, (x, y)) in [
        (-(STATION_X / 2.0 - 54.0), -(STATION_Y / 2.0 - 50.0)),
        (STATION_X / 2.0 - 54.0, -(STATION_Y / 2.0 - 50.0)),
        (-(STATION_X / 2.0 - 54.0), STATION_Y / 2.0 - 50.0),
        (STATION_X / 2.0 - 54.0, STATION_Y / 2.0 - 50.0),
        (0.0, STATION_Y / 2.0 - 50.0),
        (0.0, -(STATION_Y / 2.0 - 50.0)),
    ]
    .iter()
    .enumerate()
    {
        slots = slots
            + centered_cylinder(
                format!("cap_drop_recovery_base_m6_clearance_hole_{index}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 5.0,
                28,
            )
            .translate(*x, *y, 0.0)
            + centered_cube(
                format!("cap_drop_recovery_base_m6_slot_relief_{index}"),
                28.0,
                MOUNT_HOLE_D + 0.6,
                BASE_Z + 5.0,
            )
            .translate(*x, *y, 0.0);
    }
    slots
}

fn perimeter_rims() -> Part {
    let left = centered_cube(
        "cap_drop_recovery_left_containment_rim",
        RIM_W,
        STATION_Y - 54.0,
        RIM_Z,
    )
    .translate(
        -(STATION_X / 2.0 - RIM_W / 2.0),
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(
        "cap_drop_recovery_right_containment_rim",
        RIM_W,
        STATION_Y - 54.0,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "cap_drop_recovery_rear_service_containment_rim",
        STATION_X - 36.0,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let front_low_lip = centered_cube(
        "cap_drop_recovery_front_low_robot_access_lip",
        STATION_X - 210.0,
        14.0,
        24.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 26.0, BASE_Z / 2.0 + 12.0);

    left + right + rear + front_low_lip
}

fn workflow_dividers() -> Part {
    let top_middle = centered_cube(
        "cap_drop_recovery_custody_to_recovery_row_divider",
        STATION_X - 160.0,
        9.0,
        26.0,
    )
    .translate(0.0, 138.0, BASE_Z / 2.0 + 13.0);
    let middle_bottom = centered_cube(
        "cap_drop_recovery_recovery_to_disposition_row_divider",
        STATION_X - 180.0,
        9.0,
        24.0,
    )
    .translate(0.0, -130.0, BASE_Z / 2.0 + 12.0);
    let cap_to_connector = centered_cube(
        "cap_drop_recovery_cap_connector_custody_gap_marker",
        10.0,
        190.0,
        24.0,
    )
    .translate(-225.0, 255.0, BASE_Z / 2.0 + 12.0);
    let connector_to_replacement = centered_cube(
        "cap_drop_recovery_connector_replacement_gap_marker",
        10.0,
        190.0,
        24.0,
    )
    .translate(150.0, 255.0, BASE_Z / 2.0 + 12.0);

    top_middle + middle_bottom + cap_to_connector + connector_to_replacement
}

fn deck_fiducials() -> Part {
    let mut fiducials = Part::empty("cap_drop_recovery_deck_fiducials");
    for (index, (x, y)) in [
        (-STATION_X / 2.0 + 72.0, -STATION_Y / 2.0 + 68.0),
        (STATION_X / 2.0 - 72.0, -STATION_Y / 2.0 + 68.0),
        (-STATION_X / 2.0 + 72.0, STATION_Y / 2.0 - 68.0),
        (STATION_X / 2.0 - 72.0, STATION_Y / 2.0 - 68.0),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + fiducial_disc(&format!("cap_drop_recovery_deck_fiducial_{index}")).translate(
                *x,
                *y,
                BASE_Z / 2.0 + 3.0,
            );
    }
    fiducials
}

fn flow_arrow_datums() -> Part {
    let mut datums = Part::empty("cap_drop_recovery_flow_arrow_datums");
    for (index, (x, y)) in [
        (-560.0, 136.0),
        (-160.0, 136.0),
        (250.0, 136.0),
        (-560.0, -130.0),
        (-150.0, -130.0),
        (255.0, -130.0),
        (-560.0, -346.0),
        (-60.0, -346.0),
        (500.0, -346.0),
    ]
    .iter()
    .enumerate()
    {
        datums = datums
            + centered_cube(
                format!("cap_drop_recovery_flow_arrow_tail_{index}"),
                34.0,
                8.0,
                3.0,
            )
            .translate(*x, *y, BASE_Z / 2.0 + 1.5)
            + centered_cylinder(
                format!("cap_drop_recovery_flow_arrow_head_{index}"),
                4.5,
                3.0,
                18,
            )
            .translate(*x + 22.0, *y, BASE_Z / 2.0 + 1.5);
    }
    datums
}

fn cap_custody_nest_bank() -> Part {
    let body = centered_cube(
        "cap_drop_recovery_cap_custody_nest_bank_body",
        CAP_NEST_X,
        CAP_NEST_Y,
        CAP_NEST_Z,
    );
    let rear_fence = centered_cube(
        "cap_drop_recovery_cap_custody_rear_datum_fence",
        CAP_NEST_X,
        12.0,
        CAP_NEST_Z + 30.0,
    )
    .translate(0.0, CAP_NEST_Y / 2.0 - 6.0, 15.0);
    let left_fence = centered_cube(
        "cap_drop_recovery_cap_custody_left_clean_fence",
        12.0,
        CAP_NEST_Y - 38.0,
        CAP_NEST_Z + 22.0,
    )
    .translate(-(CAP_NEST_X / 2.0 - 6.0), -8.0, 11.0);

    body + rear_fence + left_fence - cap_custody_socket_cuts()
        + cap_custody_token_lands()
        + latch_tabs(
            "cap_drop_recovery_cap_custody",
            CAP_NEST_X,
            CAP_NEST_Y,
            CAP_NEST_Z,
        )
}

fn cap_custody_socket_cuts() -> Part {
    let mut cuts = Part::empty("cap_drop_recovery_cap_custody_socket_cuts");
    for index in 0..CAP_CUSTODY_COUNT {
        let (x, y) = grid_xy(
            index,
            CAP_CUSTODY_COLS,
            CAP_CUSTODY_ROWS,
            CAP_PITCH_X,
            CAP_PITCH_Y,
        );
        cuts = cuts
            + centered_cylinder(
                format!("cap_drop_recovery_clean_cap_socket_{index}"),
                CAP_SOCKET_D / 2.0,
                CAP_NEST_Z + 8.0,
                36,
            )
            .translate(x, y - 3.0, 4.0)
            + centered_cube(
                format!("cap_drop_recovery_clean_cap_lanyard_keyway_{index}"),
                34.0,
                7.0,
                16.0,
            )
            .translate(x, y + 22.0, CAP_NEST_Z / 2.0 - 8.0);
    }
    cuts
}

fn cap_custody_token_lands() -> Part {
    let mut lands = Part::empty("cap_drop_recovery_cap_custody_token_lands");
    for index in 0..CAP_CUSTODY_COUNT {
        let (x, y) = grid_xy(
            index,
            CAP_CUSTODY_COLS,
            CAP_CUSTODY_ROWS,
            CAP_PITCH_X,
            CAP_PITCH_Y,
        );
        lands = lands
            + raised_land(
                format!("cap_drop_recovery_clean_cap_custody_id_land_{index}"),
                42.0,
                8.0,
                CAP_NEST_Z,
            )
            .translate(x, y - 26.0, 0.0)
            + centered_cylinder(
                format!("cap_drop_recovery_clean_cap_lanyard_post_{index}"),
                3.0,
                9.0,
                18,
            )
            .translate(x + 23.0, y + 21.0, CAP_NEST_Z / 2.0 + 4.5);
    }
    lands
}

fn open_connector_surrogate_nests() -> Part {
    let body = centered_cube(
        "cap_drop_recovery_open_connector_surrogate_nest_body",
        CONNECTOR_NEST_X,
        CONNECTOR_NEST_Y,
        CONNECTOR_NEST_Z,
    );
    let front_fence = centered_cube(
        "cap_drop_recovery_open_connector_front_robot_datum_fence",
        CONNECTOR_NEST_X,
        12.0,
        CONNECTOR_NEST_Z + 28.0,
    )
    .translate(0.0, -CONNECTOR_NEST_Y / 2.0 + 6.0, 14.0);
    let rear_sensor_land = centered_cube(
        "cap_drop_recovery_open_connector_presence_sensor_land",
        CONNECTOR_NEST_X - 36.0,
        14.0,
        8.0,
    )
    .translate(
        0.0,
        CONNECTOR_NEST_Y / 2.0 - 24.0,
        CONNECTOR_NEST_Z / 2.0 + 4.0,
    );

    body + front_fence + rear_sensor_land - open_connector_socket_cuts()
        + open_connector_status_flags()
        + latch_tabs(
            "cap_drop_recovery_open_connector",
            CONNECTOR_NEST_X,
            CONNECTOR_NEST_Y,
            CONNECTOR_NEST_Z,
        )
}

fn open_connector_socket_cuts() -> Part {
    let mut cuts = Part::empty("cap_drop_recovery_open_connector_socket_cuts");
    for index in 0..CONNECTOR_PORT_COUNT {
        let (x, y) = grid_xy(
            index,
            CONNECTOR_COLS,
            CONNECTOR_ROWS,
            CONNECTOR_PITCH_X,
            CONNECTOR_PITCH_Y,
        );
        cuts = cuts
            + centered_cylinder(
                format!("cap_drop_recovery_open_connector_port_bore_{index}"),
                OPEN_PORT_BORE_D / 2.0,
                CONNECTOR_NEST_Z + 8.0,
                32,
            )
            .translate(x - 8.0, y, 3.0)
            + centered_cube(
                format!("cap_drop_recovery_open_connector_keyed_flat_{index}"),
                10.0,
                24.0,
                16.0,
            )
            .translate(x + 10.0, y, CONNECTOR_NEST_Z / 2.0 - 8.0);
    }
    cuts
}

fn open_connector_status_flags() -> Part {
    let mut flags = Part::empty("cap_drop_recovery_open_connector_status_flags");
    for index in 0..CONNECTOR_PORT_COUNT {
        let (x, y) = grid_xy(
            index,
            CONNECTOR_COLS,
            CONNECTOR_ROWS,
            CONNECTOR_PITCH_X,
            CONNECTOR_PITCH_Y,
        );
        flags = flags
            + raised_land(
                format!("cap_drop_recovery_open_connector_open_state_land_{index}"),
                30.0,
                7.0,
                CONNECTOR_NEST_Z,
            )
            .translate(x, y - 22.0, 0.0)
            + centered_cylinder(
                format!("cap_drop_recovery_open_connector_absent_cap_pin_{index}"),
                2.8,
                10.0,
                16,
            )
            .translate(x + 22.0, y + 20.0, CONNECTOR_NEST_Z / 2.0 + 5.0);
    }
    flags
}

fn dropped_cap_quarantine_tray() -> Part {
    let basin = centered_cube(
        "cap_drop_recovery_dropped_cap_quarantine_basin",
        QUARANTINE_TRAY_X,
        QUARANTINE_TRAY_Y,
        QUARANTINE_TRAY_Z,
    );
    let rear_lid_rail = centered_cube(
        "cap_drop_recovery_quarantine_rear_lid_capture_rail",
        QUARANTINE_TRAY_X,
        12.0,
        QUARANTINE_TRAY_Z + 34.0,
    )
    .translate(0.0, QUARANTINE_TRAY_Y / 2.0 - 6.0, 17.0);
    let front_retainer = centered_cube(
        "cap_drop_recovery_quarantine_front_scoop_retainer",
        QUARANTINE_TRAY_X - 34.0,
        10.0,
        QUARANTINE_TRAY_Z + 18.0,
    )
    .translate(0.0, -QUARANTINE_TRAY_Y / 2.0 + 5.0, 9.0);
    let center_divider = centered_cube(
        "cap_drop_recovery_quarantine_clean_dirty_center_divider",
        10.0,
        QUARANTINE_TRAY_Y - 38.0,
        QUARANTINE_TRAY_Z + 20.0,
    )
    .translate(0.0, -4.0, 10.0);

    basin + rear_lid_rail + front_retainer + center_divider
        - quarantine_well_cuts()
        - quarantine_scoop_notches()
        + quarantine_evidence_lands()
        + tamper_posts(
            "quarantine",
            QUARANTINE_TRAY_X,
            QUARANTINE_TRAY_Y,
            QUARANTINE_TRAY_Z,
        )
}

fn quarantine_well_cuts() -> Part {
    let mut cuts = Part::empty("cap_drop_recovery_quarantine_well_cuts");
    for index in 0..QUARANTINE_WELL_COUNT {
        let (x, y) = grid_xy(index, 4, 3, QUARANTINE_PITCH_X, QUARANTINE_PITCH_Y);
        cuts = cuts
            + centered_cylinder(
                format!("cap_drop_recovery_dropped_cap_quarantine_well_{index}"),
                QUARANTINE_WELL_D / 2.0,
                QUARANTINE_TRAY_Z + 8.0,
                36,
            )
            .translate(x, y, 5.0);
    }
    cuts
}

fn quarantine_scoop_notches() -> Part {
    let mut notches = Part::empty("cap_drop_recovery_quarantine_scoop_notches");
    for index in 0..4 {
        notches = notches
            + centered_cube(
                format!("cap_drop_recovery_quarantine_robot_scoop_notch_{index}"),
                42.0,
                12.0,
                18.0,
            )
            .translate(
                centered_index(index, 4, QUARANTINE_PITCH_X),
                -QUARANTINE_TRAY_Y / 2.0 + 12.0,
                QUARANTINE_TRAY_Z / 2.0 - 9.0,
            );
    }
    notches
}

fn quarantine_evidence_lands() -> Part {
    let mut lands = Part::empty("cap_drop_recovery_quarantine_evidence_lands");
    for index in 0..6 {
        lands = lands
            + raised_land(
                format!("cap_drop_recovery_quarantine_evidence_barcode_land_{index}"),
                44.0,
                13.0,
                QUARANTINE_TRAY_Z,
            )
            .translate(
                centered_index(index, 6, 52.0),
                QUARANTINE_TRAY_Y / 2.0 - 30.0,
                0.0,
            );
    }
    lands
}

fn robotic_recovery_envelope_gauges() -> Part {
    let frame = centered_cube(
        "cap_drop_recovery_robotic_recovery_envelope_outer_frame",
        RECOVERY_GAUGE_X,
        RECOVERY_GAUGE_Y,
        RECOVERY_GAUGE_Z,
    ) - centered_cube(
        "cap_drop_recovery_robotic_recovery_sweep_window",
        RECOVERY_GAUGE_X - 82.0,
        RECOVERY_GAUGE_Y - 68.0,
        RECOVERY_GAUGE_Z + 8.0,
    );

    frame
        + recovery_arc_rails()
        + gripper_jaw_gauge_blocks()
        + recovery_height_ladder()
        + latch_tabs(
            "cap_drop_recovery_robotic_recovery",
            RECOVERY_GAUGE_X,
            RECOVERY_GAUGE_Y,
            RECOVERY_GAUGE_Z,
        )
}

fn recovery_arc_rails() -> Part {
    let mut rails = Part::empty("cap_drop_recovery_recovery_arc_rails");
    for index in 0..RECOVERY_ENVELOPE_STATIONS {
        let width = RECOVERY_GAUGE_X - 94.0 - index as f64 * 34.0;
        rails = rails
            + centered_cube(
                format!("cap_drop_recovery_recovery_reach_arc_rail_{index}"),
                width,
                5.0,
                9.0,
            )
            .translate(
                0.0,
                centered_index(index, RECOVERY_ENVELOPE_STATIONS, 27.0),
                RECOVERY_GAUGE_Z / 2.0 + 4.5,
            );
    }
    rails
}

fn gripper_jaw_gauge_blocks() -> Part {
    let mut gauges = Part::empty("cap_drop_recovery_gripper_jaw_gauge_blocks");
    for index in 0..GRIPPER_JAW_GAUGES {
        let x = centered_index(index, GRIPPER_JAW_GAUGES, 70.0);
        gauges = gauges
            + centered_cube(
                format!("cap_drop_recovery_gripper_jaw_inner_clearance_gauge_{index}"),
                20.0,
                46.0,
                18.0,
            )
            .translate(
                x - 15.0,
                -(RECOVERY_GAUGE_Y / 2.0 - 30.0),
                RECOVERY_GAUGE_Z / 2.0 + 9.0,
            )
            + centered_cube(
                format!("cap_drop_recovery_gripper_jaw_outer_clearance_gauge_{index}"),
                20.0,
                46.0,
                18.0,
            )
            .translate(
                x + 15.0,
                -(RECOVERY_GAUGE_Y / 2.0 - 30.0),
                RECOVERY_GAUGE_Z / 2.0 + 9.0,
            );
    }
    gauges
}

fn recovery_height_ladder() -> Part {
    let mut ladder = Part::empty("cap_drop_recovery_recovery_height_ladder");
    for index in 0..RECOVERY_ENVELOPE_STATIONS {
        let height = 30.0 + index as f64 * 18.0;
        ladder = ladder
            + centered_cube(
                format!("cap_drop_recovery_pick_height_reference_post_{index}"),
                18.0,
                18.0,
                height,
            )
            .translate(
                -(RECOVERY_GAUGE_X / 2.0 - 30.0),
                centered_index(index, RECOVERY_ENVELOPE_STATIONS, 33.0),
                RECOVERY_GAUGE_Z / 2.0 + height / 2.0,
            )
            + raised_land(
                format!("cap_drop_recovery_pick_height_label_land_{index}"),
                38.0,
                8.0,
                RECOVERY_GAUGE_Z,
            )
            .translate(
                -(RECOVERY_GAUGE_X / 2.0 - 64.0),
                centered_index(index, RECOVERY_ENVELOPE_STATIONS, 33.0),
                0.0,
            );
    }
    ladder
}

fn contamination_witness_coupon_pockets() -> Part {
    let block = centered_cube(
        "cap_drop_recovery_contamination_witness_coupon_pocket_block",
        COUPON_POCKET_X,
        COUPON_POCKET_Y,
        COUPON_POCKET_Z,
    );
    let rear_swab_rail = centered_cube(
        "cap_drop_recovery_contamination_witness_swab_rail",
        COUPON_POCKET_X - 40.0,
        12.0,
        13.0,
    )
    .translate(
        0.0,
        COUPON_POCKET_Y / 2.0 - 22.0,
        COUPON_POCKET_Z / 2.0 + 6.5,
    );

    block - coupon_pocket_cuts()
        + rear_swab_rail
        + coupon_id_lands()
        + latch_tabs(
            "cap_drop_recovery_contamination_coupon",
            COUPON_POCKET_X,
            COUPON_POCKET_Y,
            COUPON_POCKET_Z,
        )
}

fn coupon_pocket_cuts() -> Part {
    let mut cuts = Part::empty("cap_drop_recovery_coupon_pocket_cuts");
    for index in 0..COUPON_POCKET_COUNT {
        let (x, y) = grid_xy(index, 4, 3, COUPON_PITCH_X, COUPON_PITCH_Y);
        cuts = cuts
            + centered_cube(
                format!("cap_drop_recovery_contamination_coupon_slide_pocket_{index}"),
                50.0,
                27.0,
                COUPON_POCKET_Z + 5.0,
            )
            .translate(x, y - 5.0, 0.0)
            + centered_cube(
                format!("cap_drop_recovery_contamination_coupon_pull_tab_relief_{index}"),
                18.0,
                9.0,
                10.0,
            )
            .translate(x, y + 17.0, COUPON_POCKET_Z / 2.0 - 5.0);
    }
    cuts
}

fn coupon_id_lands() -> Part {
    let mut lands = Part::empty("cap_drop_recovery_coupon_id_lands");
    for index in 0..COUPON_POCKET_COUNT {
        let (x, y) = grid_xy(index, 4, 3, COUPON_PITCH_X, COUPON_PITCH_Y);
        lands = lands
            + raised_land(
                format!("cap_drop_recovery_coupon_barcode_id_land_{index}"),
                40.0,
                8.0,
                COUPON_POCKET_Z,
            )
            .translate(x, y - 27.0, 0.0);
    }
    lands
}

fn replacement_cap_release_lane() -> Part {
    let lane = centered_cube(
        "cap_drop_recovery_replacement_cap_release_lane_body",
        REPLACEMENT_LANE_X,
        REPLACEMENT_LANE_Y,
        REPLACEMENT_LANE_Z,
    );
    let clean_rear_fence = centered_cube(
        "cap_drop_recovery_replacement_clean_rear_fence",
        REPLACEMENT_LANE_X,
        12.0,
        REPLACEMENT_LANE_Z + 30.0,
    )
    .translate(0.0, REPLACEMENT_LANE_Y / 2.0 - 6.0, 15.0);
    let release_seal_strip = centered_cube(
        "cap_drop_recovery_replacement_tamper_seal_release_strip",
        REPLACEMENT_LANE_X - 48.0,
        10.0,
        9.0,
    )
    .translate(
        0.0,
        -REPLACEMENT_LANE_Y / 2.0 + 22.0,
        REPLACEMENT_LANE_Z / 2.0 + 4.5,
    );

    lane + clean_rear_fence + release_seal_strip - replacement_cap_socket_cuts()
        + replacement_release_pawls()
        + replacement_barcode_lands()
        + latch_tabs(
            "cap_drop_recovery_replacement_release",
            REPLACEMENT_LANE_X,
            REPLACEMENT_LANE_Y,
            REPLACEMENT_LANE_Z,
        )
}

fn replacement_cap_socket_cuts() -> Part {
    let mut cuts = Part::empty("cap_drop_recovery_replacement_cap_socket_cuts");
    for index in 0..REPLACEMENT_CAP_COUNT {
        let (x, y) = grid_xy(index, 4, 2, REPLACEMENT_PITCH_X, REPLACEMENT_PITCH_Y);
        cuts = cuts
            + centered_cylinder(
                format!("cap_drop_recovery_replacement_cap_socket_{index}"),
                CAP_SOCKET_D / 2.0,
                REPLACEMENT_LANE_Z + 8.0,
                36,
            )
            .translate(x, y + 16.0, 4.0)
            + centered_cube(
                format!("cap_drop_recovery_replacement_lot_keyway_{index}"),
                32.0,
                7.0,
                15.0,
            )
            .translate(x, y - 10.0, REPLACEMENT_LANE_Z / 2.0 - 7.5);
    }
    cuts
}

fn replacement_release_pawls() -> Part {
    let mut pawls = Part::empty("cap_drop_recovery_replacement_release_pawls");
    for index in 0..REPLACEMENT_CAP_COUNT {
        let (x, y) = grid_xy(index, 4, 2, REPLACEMENT_PITCH_X, REPLACEMENT_PITCH_Y);
        pawls = pawls
            + centered_cube(
                format!("cap_drop_recovery_replacement_one_way_pawl_{index}"),
                10.0,
                34.0,
                8.0,
            )
            .rotate(0.0, 0.0, if index % 2 == 0 { 15.0 } else { -15.0 })
            .translate(x + 25.0, y + 14.0, REPLACEMENT_LANE_Z / 2.0 + 4.0);
    }
    pawls
}

fn replacement_barcode_lands() -> Part {
    let mut lands = Part::empty("cap_drop_recovery_replacement_barcode_lands");
    for index in 0..4 {
        lands = lands
            + raised_land(
                format!("cap_drop_recovery_replacement_lot_barcode_land_{index}"),
                48.0,
                14.0,
                REPLACEMENT_LANE_Z,
            )
            .translate(
                centered_index(index, 4, 64.0),
                -(REPLACEMENT_LANE_Y / 2.0 - 44.0),
                0.0,
            );
    }
    lands
}

fn connector_open_time_token_rail() -> Part {
    let rail = centered_cube(
        "cap_drop_recovery_connector_open_time_token_rail_body",
        TOKEN_RAIL_X,
        TOKEN_RAIL_Y,
        TOKEN_RAIL_Z,
    );
    let upper_fence = centered_cube(
        "cap_drop_recovery_open_time_token_upper_fence",
        TOKEN_RAIL_X,
        10.0,
        TOKEN_RAIL_Z + 22.0,
    )
    .translate(0.0, TOKEN_RAIL_Y / 2.0 - 5.0, 11.0);
    let lower_fence = centered_cube(
        "cap_drop_recovery_open_time_token_lower_fence",
        TOKEN_RAIL_X,
        10.0,
        TOKEN_RAIL_Z + 22.0,
    )
    .translate(0.0, -TOKEN_RAIL_Y / 2.0 + 5.0, 11.0);

    rail + upper_fence + lower_fence - open_time_token_slot_cuts()
        + open_time_tick_marks()
        + token_stop_posts()
}

fn open_time_token_slot_cuts() -> Part {
    let mut cuts = Part::empty("cap_drop_recovery_open_time_token_slot_cuts");
    for index in 0..OPEN_TIME_TOKEN_COUNT {
        let (x, y) = grid_xy(index, TOKEN_COLS, 2, TOKEN_PITCH_X, TOKEN_PITCH_Y);
        cuts = cuts
            + centered_cube(
                format!("cap_drop_recovery_connector_open_time_token_slot_{index}"),
                11.0,
                40.0,
                TOKEN_RAIL_Z + 5.0,
            )
            .translate(x, y, 0.0)
            + centered_cube(
                format!("cap_drop_recovery_connector_open_time_reader_notch_{index}"),
                8.0,
                8.0,
                8.0,
            )
            .translate(x, y + 24.0, TOKEN_RAIL_Z / 2.0 - 4.0);
    }
    cuts
}

fn open_time_tick_marks() -> Part {
    let mut ticks = Part::empty("cap_drop_recovery_open_time_tick_marks");
    for index in 0..=TOKEN_COLS {
        ticks = ticks
            + centered_cube(
                format!("cap_drop_recovery_open_time_minute_tick_{index}"),
                4.0,
                if index % 2 == 0 { 18.0 } else { 12.0 },
                5.0,
            )
            .translate(
                centered_index(index, TOKEN_COLS + 1, TOKEN_PITCH_X),
                0.0,
                TOKEN_RAIL_Z / 2.0 + 2.5,
            );
    }
    ticks
}

fn token_stop_posts() -> Part {
    let mut posts = Part::empty("cap_drop_recovery_open_time_token_stop_posts");
    for (index, x) in [
        -(TOKEN_RAIL_X / 2.0 - 24.0),
        TOKEN_RAIL_X / 2.0 - 24.0,
        -22.0,
        22.0,
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cylinder(
                format!("cap_drop_recovery_open_time_token_stop_post_{index}"),
                5.0,
                18.0,
                24,
            )
            .translate(*x, 0.0, TOKEN_RAIL_Z / 2.0 + 9.0);
    }
    posts
}

fn release_hold_reject_gates() -> Part {
    let body = centered_cube(
        "cap_drop_recovery_release_hold_reject_gate_body",
        GATE_X,
        GATE_Y,
        GATE_Z,
    );
    let rear_stop = centered_cube(
        "cap_drop_recovery_disposition_gate_rear_stop",
        GATE_X,
        12.0,
        GATE_Z + 28.0,
    )
    .translate(0.0, GATE_Y / 2.0 - 6.0, 14.0);

    body + rear_stop - disposition_gate_slot_cuts()
        + disposition_lane_walls()
        + disposition_gate_flags()
        + latch_tabs("cap_drop_recovery_disposition_gate", GATE_X, GATE_Y, GATE_Z)
}

fn disposition_gate_slot_cuts() -> Part {
    let mut cuts = Part::empty("cap_drop_recovery_disposition_gate_slot_cuts");
    for lane in 0..STATUS_GATE_COUNT {
        for slot in 0..GATE_SLOTS_PER_LANE {
            let x = centered_index(slot, GATE_SLOTS_PER_LANE, 70.0);
            let y = gate_lane_y(lane);
            cuts = cuts
                + centered_cube(
                    format!(
                        "cap_drop_recovery_{}_gate_token_slot_{slot}",
                        gate_lane_name(lane)
                    ),
                    42.0,
                    14.0,
                    GATE_Z + 6.0,
                )
                .translate(x, y, 0.0);
        }
    }
    cuts
}

fn disposition_lane_walls() -> Part {
    let mut walls = Part::empty("cap_drop_recovery_disposition_lane_walls");
    for index in 0..=STATUS_GATE_COUNT {
        let y = -GATE_Y / 2.0 + index as f64 * (GATE_Y / STATUS_GATE_COUNT as f64);
        walls = walls
            + centered_cube(
                format!("cap_drop_recovery_disposition_lane_wall_{index}"),
                GATE_X - 34.0,
                7.0,
                20.0,
            )
            .translate(0.0, y, GATE_Z / 2.0 + 10.0);
    }
    walls
}

fn disposition_gate_flags() -> Part {
    let mut flags = Part::empty("cap_drop_recovery_disposition_gate_flags");
    for lane in 0..STATUS_GATE_COUNT {
        let y = gate_lane_y(lane);
        flags = flags
            + raised_land(
                format!(
                    "cap_drop_recovery_{}_gate_status_land",
                    gate_lane_name(lane)
                ),
                86.0,
                16.0,
                GATE_Z,
            )
            .translate(-(GATE_X / 2.0 - 62.0), y, 0.0)
            + centered_cube(
                format!(
                    "cap_drop_recovery_{}_gate_physical_flag",
                    gate_lane_name(lane)
                ),
                16.0,
                38.0,
                28.0,
            )
            .translate(GATE_X / 2.0 - 46.0, y, GATE_Z / 2.0 + 14.0);
    }
    flags
}

fn gate_lane_y(lane: usize) -> f64 {
    centered_index(lane, STATUS_GATE_COUNT, 45.0)
}

fn gate_lane_name(lane: usize) -> &'static str {
    match lane {
        0 => "release",
        1 => "hold",
        2 => "reject",
        _ => unreachable!("invalid disposition lane"),
    }
}

fn evidence_camera_lighting_bridge() -> Part {
    let mut posts = Part::empty("cap_drop_recovery_evidence_bridge_posts");
    for (index, (x, y)) in [
        (-BRIDGE_SPAN_X / 2.0, -BRIDGE_SPAN_Y / 2.0),
        (BRIDGE_SPAN_X / 2.0, -BRIDGE_SPAN_Y / 2.0),
        (-BRIDGE_SPAN_X / 2.0, BRIDGE_SPAN_Y / 2.0),
        (BRIDGE_SPAN_X / 2.0, BRIDGE_SPAN_Y / 2.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(
                format!("cap_drop_recovery_evidence_bridge_post_{index}"),
                28.0,
                36.0,
                BRIDGE_POST_Z,
            )
            .translate(*x, *y, 0.0);
    }

    posts + bridge_beams() + camera_pods() + light_bars() + bridge_fiducials()
}

fn bridge_beams() -> Part {
    let front = centered_cube(
        "cap_drop_recovery_evidence_bridge_front_beam",
        BRIDGE_SPAN_X + 74.0,
        30.0,
        BRIDGE_BEAM_Z,
    )
    .translate(
        0.0,
        -BRIDGE_SPAN_Y / 2.0,
        BRIDGE_POST_Z / 2.0 + BRIDGE_BEAM_Z / 2.0,
    );
    let rear = centered_cube(
        "cap_drop_recovery_evidence_bridge_rear_beam",
        BRIDGE_SPAN_X + 74.0,
        30.0,
        BRIDGE_BEAM_Z,
    )
    .translate(
        0.0,
        BRIDGE_SPAN_Y / 2.0,
        BRIDGE_POST_Z / 2.0 + BRIDGE_BEAM_Z / 2.0,
    );
    let center_cross = centered_cube(
        "cap_drop_recovery_evidence_bridge_center_cross_member",
        34.0,
        BRIDGE_SPAN_Y + 42.0,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, 0.0, BRIDGE_POST_Z / 2.0 + BRIDGE_BEAM_Z / 2.0 + 10.0);

    front + rear + center_cross
}

fn camera_pods() -> Part {
    let mut cameras = Part::empty("cap_drop_recovery_evidence_camera_pods");
    for index in 0..CAMERA_COUNT {
        cameras = cameras
            + centered_cube(
                format!("cap_drop_recovery_evidence_camera_pod_{index}"),
                48.0,
                26.0,
                24.0,
            )
            .translate(
                centered_index(index, CAMERA_COUNT, 250.0),
                -BRIDGE_SPAN_Y / 2.0 + 22.0,
                BRIDGE_POST_Z / 2.0 - 26.0,
            )
            + centered_cylinder(
                format!("cap_drop_recovery_evidence_camera_lens_bore_{index}"),
                7.0,
                10.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(index, CAMERA_COUNT, 250.0),
                -BRIDGE_SPAN_Y / 2.0 + 8.0,
                BRIDGE_POST_Z / 2.0 - 26.0,
            );
    }
    cameras
}

fn light_bars() -> Part {
    let mut bars = Part::empty("cap_drop_recovery_evidence_light_bars");
    for index in 0..LIGHT_BAR_COUNT {
        bars = bars
            + centered_cube(
                format!("cap_drop_recovery_evidence_area_light_bar_{index}"),
                390.0,
                10.0,
                10.0,
            )
            .translate(
                centered_index(index, LIGHT_BAR_COUNT, 450.0),
                BRIDGE_SPAN_Y / 2.0 - 22.0,
                BRIDGE_POST_Z / 2.0 - 46.0,
            );
    }
    bars
}

fn bridge_fiducials() -> Part {
    let mut fiducials = Part::empty("cap_drop_recovery_evidence_bridge_fiducials");
    for (index, x) in [-480.0, -160.0, 160.0, 480.0].iter().enumerate() {
        fiducials = fiducials
            + fiducial_disc(&format!(
                "cap_drop_recovery_evidence_bridge_fiducial_{index}"
            ))
            .translate(*x, BRIDGE_SPAN_Y / 2.0, BRIDGE_POST_Z / 2.0 + 16.0);
    }
    fiducials
}

fn robot_service_keepout_gauges() -> Part {
    let block = centered_cube(
        "cap_drop_recovery_robot_service_keepout_gauge_block",
        KEEP_OUT_X,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    );
    let robot_sweep_window = centered_cube(
        "cap_drop_recovery_robot_recovery_sweep_keepout_window",
        KEEP_OUT_X - 40.0,
        34.0,
        KEEP_OUT_Z + 5.0,
    )
    .translate(0.0, -38.0, 0.0);
    let service_sweep_window = centered_cube(
        "cap_drop_recovery_service_access_keepout_window",
        KEEP_OUT_X - 56.0,
        30.0,
        KEEP_OUT_Z + 5.0,
    )
    .translate(0.0, 42.0, 0.0);

    block - robot_sweep_window - service_sweep_window
        + robot_keepout_flags()
        + service_keepout_flags()
        + keepout_height_posts()
}

fn robot_keepout_flags() -> Part {
    let mut flags = Part::empty("cap_drop_recovery_robot_keepout_flags");
    for index in 0..ROBOT_KEEP_OUT_FLAGS {
        flags = flags
            + centered_cube(
                format!("cap_drop_recovery_robot_sweep_keepout_flag_{index}"),
                12.0,
                26.0,
                38.0,
            )
            .translate(
                centered_index(index, ROBOT_KEEP_OUT_FLAGS, 32.0),
                -KEEP_OUT_Y / 2.0 + 24.0,
                KEEP_OUT_Z / 2.0 + 19.0,
            );
    }
    flags
}

fn service_keepout_flags() -> Part {
    let mut flags = Part::empty("cap_drop_recovery_service_keepout_flags");
    for index in 0..SERVICE_KEEP_OUT_FLAGS {
        flags = flags
            + centered_cube(
                format!("cap_drop_recovery_service_keepout_flag_{index}"),
                30.0,
                10.0,
                30.0,
            )
            .translate(
                centered_index(index, SERVICE_KEEP_OUT_FLAGS, 46.0),
                KEEP_OUT_Y / 2.0 - 24.0,
                KEEP_OUT_Z / 2.0 + 15.0,
            );
    }
    flags
}

fn keepout_height_posts() -> Part {
    let robot_post = centered_cylinder(
        "cap_drop_recovery_robot_pick_clearance_height_post",
        6.0,
        ROBOT_CLEARANCE_Z,
        24,
    )
    .translate(
        -(KEEP_OUT_X / 2.0 - 26.0),
        0.0,
        KEEP_OUT_Z / 2.0 + ROBOT_CLEARANCE_Z / 2.0,
    );
    let service_post = centered_cylinder(
        "cap_drop_recovery_service_hand_clearance_height_post",
        5.0,
        SERVICE_CLEARANCE_Z,
        24,
    )
    .translate(
        KEEP_OUT_X / 2.0 - 26.0,
        0.0,
        KEEP_OUT_Z / 2.0 + SERVICE_CLEARANCE_Z / 2.0,
    );
    robot_post + service_post
}

fn latch_tabs(prefix: &str, width: f64, depth: f64, base_height: f64) -> Part {
    let mut tabs = Part::empty(format!("{prefix}_latch_tabs"));
    for (index, (x, y)) in [
        (-(width / 2.0 - 24.0), -(depth / 2.0 - 16.0)),
        (width / 2.0 - 24.0, -(depth / 2.0 - 16.0)),
        (-(width / 2.0 - 24.0), depth / 2.0 - 16.0),
        (width / 2.0 - 24.0, depth / 2.0 - 16.0),
    ]
    .iter()
    .enumerate()
    {
        tabs = tabs
            + centered_cube(format!("{prefix}_latch_tab_{index}"), 28.0, 10.0, 8.0).translate(
                *x,
                *y,
                base_height / 2.0 + 4.0,
            );
    }
    tabs
}

fn tamper_posts(prefix: &str, width: f64, depth: f64, base_height: f64) -> Part {
    let mut posts = Part::empty(format!("cap_drop_recovery_{prefix}_tamper_posts"));
    for (index, (x, y)) in [
        (-(width / 2.0 - 26.0), -(depth / 2.0 - 24.0)),
        (width / 2.0 - 26.0, -(depth / 2.0 - 24.0)),
        (-(width / 2.0 - 26.0), depth / 2.0 - 24.0),
        (width / 2.0 - 26.0, depth / 2.0 - 24.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cylinder(
                format!("cap_drop_recovery_{prefix}_tamper_wire_post_{index}"),
                4.0,
                14.0,
                20,
            )
            .translate(*x, *y, base_height / 2.0 + 7.0);
    }
    posts
}

fn raised_land(name: impl Into<String>, width: f64, depth: f64, base_height: f64) -> Part {
    centered_cube(name, width, depth, 3.0).translate(0.0, 0.0, base_height / 2.0 + 1.5)
}

fn fiducial_disc(name: &str) -> Part {
    centered_cylinder(format!("{name}_outer_disc"), 8.0, 4.0, 32)
        - centered_cylinder(format!("{name}_center_bore"), 2.0, 5.0, 18)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);

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
    fn requested_feature_presence_is_explicit() {
        for feature in [
            "cap_custody_nests",
            "dropped_cap_quarantine_tray",
            "robotic_recovery_envelope_gauges",
            "contamination_witness_coupon_pockets",
            "replacement_cap_release_lane",
            "connector_open_time_token_rail",
            "release_hold_reject_gates",
            "open_connector_surrogate_nests",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
        assert_eq!(REQUIRED_FEATURES.len(), 10);
    }

    #[test]
    fn cap_custody_and_open_time_counts_cover_all_connectors() {
        assert_eq!(CONNECTOR_ROWS * CONNECTOR_COLS, CONNECTOR_PORT_COUNT);
        assert_eq!(CAP_CUSTODY_ROWS * CAP_CUSTODY_COLS, CAP_CUSTODY_COUNT);
        assert_eq!(CAP_CUSTODY_COUNT, CONNECTOR_PORT_COUNT);
        assert_eq!(OPEN_TIME_TOKEN_COUNT, CONNECTOR_PORT_COUNT);
        assert_eq!(TOKEN_COLS * 2, OPEN_TIME_TOKEN_COUNT);
        assert!(REPLACEMENT_CAP_COUNT >= CONNECTOR_PORT_COUNT / 2);
    }

    #[test]
    fn quarantine_witness_and_disposition_capacity_are_sized() {
        assert_eq!(QUARANTINE_WELL_COUNT, 12);
        assert_eq!(COUPON_POCKET_COUNT, QUARANTINE_WELL_COUNT);
        assert_eq!(STATUS_GATE_COUNT, 3);
        assert_eq!(GATE_SLOT_COUNT, STATUS_GATE_COUNT * GATE_SLOTS_PER_LANE);
        assert!(GATE_SLOT_COUNT >= QUARANTINE_WELL_COUNT);
        assert_eq!(gate_lane_name(0), "release");
        assert_eq!(gate_lane_name(1), "hold");
        assert_eq!(gate_lane_name(2), "reject");
    }

    #[test]
    fn station_dimensions_and_clearances_are_bounded() {
        assert_design_constraints();
        assert!(STATION_X <= 1320.0);
        assert!(STATION_Y <= 900.0);
        assert!(BRIDGE_SPAN_X < STATION_X - 2.0 * RIM_W);
        assert!(BRIDGE_SPAN_Y < STATION_Y - 2.0 * RIM_W);
        assert!(BRIDGE_POST_Z > CAMERA_CLEARANCE_Z);
        assert!(ROBOT_CLEARANCE_Z > SERVICE_CLEARANCE_Z);
        assert!(MAX_RECOVERY_PICK_HEIGHT_MM < ROBOT_CLEARANCE_Z);
    }

    #[test]
    fn layout_modules_fit_and_do_not_overlap() {
        let rects = layout_rects();
        assert_eq!(rects.len(), 10);
        for (_name, rect) in rects {
            assert!(fits_on_station(rect));
        }
        for (left_index, (_left_name, left)) in rects.iter().enumerate() {
            for (_right_name, right) in rects.iter().skip(left_index + 1) {
                assert!(!rects_overlap(*left, *right));
            }
        }
    }

    #[test]
    fn recovery_and_evidence_hardware_are_present() {
        assert_eq!(RECOVERY_ENVELOPE_STATIONS, 5);
        assert_eq!(GRIPPER_JAW_GAUGES, 4);
        assert_eq!(CAMERA_COUNT, 4);
        assert_eq!(LIGHT_BAR_COUNT, 2);
        assert_eq!(ROBOT_KEEP_OUT_FLAGS, 6);
        assert_eq!(SERVICE_KEEP_OUT_FLAGS, 4);
    }
}
