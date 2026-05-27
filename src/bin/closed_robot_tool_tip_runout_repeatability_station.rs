use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed robotic tool-tip runout and repeatability validation station.
//
// Intent:
// - Hold a robot tool in a repeatable kinematic nest while runout is measured by
//   a dial indicator or LVDT and cross-checked by a camera fiducial bridge.
// - Stage probe targets, go/no-go gauges, datum pins, and calibration coupons in
//   one closed, traceable station so tool-change repeatability can be verified
//   without live culture materials.
// - Model fixture envelopes, references, storage, and service keepouts only.
//   Metrology acceptance limits, robot programs, and calibration traceability
//   records remain separate validation controls.

const BIN_PREFIX: &str = "closed_robot_tool_tip_runout_repeatability_station";

const OUTPUTS: &[&str] = &[
    "output/closed_robot_tool_tip_runout_repeatability_station_base_tray.stl",
    "output/closed_robot_tool_tip_runout_repeatability_station_kinematic_tool_nest.stl",
    "output/closed_robot_tool_tip_runout_repeatability_station_dial_indicator_lvdt_mount.stl",
    "output/closed_robot_tool_tip_runout_repeatability_station_camera_fiducial_bridge.stl",
    "output/closed_robot_tool_tip_runout_repeatability_station_probe_target_array.stl",
    "output/closed_robot_tool_tip_runout_repeatability_station_go_no_go_gauge_bank.stl",
    "output/closed_robot_tool_tip_runout_repeatability_station_tool_changer_datum_pin_plate.stl",
    "output/closed_robot_tool_tip_runout_repeatability_station_calibration_coupon_storage.stl",
    "output/closed_robot_tool_tip_runout_repeatability_station_traceability_label_lands.stl",
    "output/closed_robot_tool_tip_runout_repeatability_station_robot_service_keepouts.stl",
    "output/closed_robot_tool_tip_runout_repeatability_station_assembly.stl",
];

const REQUIRED_FEATURES: &[&str] = &[
    "kinematic_tool_nest",
    "dial_indicator_lvdt_mount",
    "camera_fiducials",
    "probe_target_array",
    "go_no_go_gauges",
    "tool_changer_datum_pins",
    "calibration_coupon_storage",
    "traceability_label_lands",
    "robot_service_keepouts",
    "assembly_export",
];

const STATION_X: f64 = 940.0;
const STATION_Y: f64 = 680.0;
const BASE_Z: f64 = 24.0;
const BASE_RIM_W: f64 = 18.0;
const BASE_RIM_Z: f64 = 38.0;
const DRAIN_GUTTER_Y: f64 = 18.0;
const MOUNT_SLOT_COUNT: usize = 8;
const BASE_DATUM_TARGET_COUNT: usize = 4;

const TOOL_NEST_X: f64 = 282.0;
const TOOL_NEST_Y: f64 = 204.0;
const TOOL_NEST_Z: f64 = 58.0;
const TOOL_NEST_POS: (f64, f64) = (-282.0, 116.0);
const TOOL_SHANK_BORE_D: f64 = 42.0;
const TOOL_TIP_CLEARANCE_D: f64 = 16.0;
const KINEMATIC_CONTACT_COUNT: usize = 3;
const KINEMATIC_CONTACT_D: f64 = 12.0;
const KINEMATIC_CONTACT_PITCH_X: f64 = 138.0;
const KINEMATIC_CONTACT_PITCH_Y: f64 = 118.0;
const NEST_APPROACH_FUNNEL_COUNT: usize = 2;

const INDICATOR_BASE_X: f64 = 250.0;
const INDICATOR_BASE_Y: f64 = 174.0;
const INDICATOR_BASE_Z: f64 = 22.0;
const INDICATOR_POS: (f64, f64) = (180.0, 128.0);
const INDICATOR_POST_HEIGHT: f64 = 166.0;
const INDICATOR_POST_D: f64 = 24.0;
const INDICATOR_CLAMP_X: f64 = 132.0;
const INDICATOR_CLAMP_Y: f64 = 54.0;
const INDICATOR_CLAMP_Z: f64 = 44.0;
const INDICATOR_STYLUS_CLEARANCE: f64 = 213.0;
const LVDT_BODY_D: f64 = 28.0;
const LVDT_BODY_LEN: f64 = 122.0;

const CAMERA_BRIDGE_X: f64 = 806.0;
const CAMERA_BRIDGE_Y: f64 = 64.0;
const CAMERA_BRIDGE_POS: (f64, f64) = (0.0, -6.0);
const CAMERA_UPRIGHT_Z: f64 = 154.0;
const CAMERA_BEAM_Z: f64 = 30.0;
const CAMERA_FIDUCIAL_COUNT: usize = 8;
const CAMERA_FIDUCIAL_D: f64 = 10.0;
const CAMERA_WINDOW_X: f64 = 326.0;
const CAMERA_WINDOW_Y: f64 = 34.0;

const TARGET_PLATE_X: f64 = 304.0;
const TARGET_PLATE_Y: f64 = 184.0;
const TARGET_PLATE_Z: f64 = 18.0;
const TARGET_ARRAY_POS: (f64, f64) = (-158.0, -105.0);
const TARGET_COLS: usize = 5;
const TARGET_ROWS: usize = 4;
const PROBE_TARGET_COUNT: usize = TARGET_COLS * TARGET_ROWS;
const TARGET_PITCH_X: f64 = 49.0;
const TARGET_PITCH_Y: f64 = 38.0;
const TARGET_PAD_D: f64 = 16.0;
const TARGET_HEIGHT_STEP: f64 = 1.25;
const RUNOUT_MASTER_D: f64 = 54.0;

const GAUGE_BANK_X: f64 = 266.0;
const GAUGE_BANK_Y: f64 = 154.0;
const GAUGE_BANK_Z: f64 = 30.0;
const GAUGE_BANK_POS: (f64, f64) = (236.0, -116.0);
const GO_GAUGE_COUNT: usize = 6;
const NO_GO_GAUGE_COUNT: usize = 6;
const GAUGE_PITCH_X: f64 = 36.0;
const GO_PIN_D: f64 = 8.0;
const NO_GO_PIN_D: f64 = 9.5;
const RING_GAUGE_COUNT: usize = 4;

const DATUM_PLATE_X: f64 = 256.0;
const DATUM_PLATE_Y: f64 = 92.0;
const DATUM_PLATE_Z: f64 = 20.0;
const DATUM_PLATE_POS: (f64, f64) = (-292.0, -254.0);
const TOOL_CHANGER_DATUM_PIN_COUNT: usize = 4;
const TOOL_CHANGER_DATUM_PIN_D: f64 = 10.0;
const TOOL_CHANGER_DATUM_PIN_Z: f64 = 32.0;
const TOOL_CHANGER_PIN_SPAN_X: f64 = 184.0;
const TOOL_CHANGER_PIN_SPAN_Y: f64 = 48.0;

const COUPON_STORAGE_X: f64 = 310.0;
const COUPON_STORAGE_Y: f64 = 104.0;
const COUPON_STORAGE_Z: f64 = 40.0;
const COUPON_STORAGE_POS: (f64, f64) = (188.0, -254.0);
const CALIBRATION_COUPON_COUNT: usize = 8;
const COUPON_SLOT_PITCH_X: f64 = 32.0;
const COUPON_SLOT_X: f64 = 23.0;
const COUPON_SLOT_Y: f64 = 68.0;
const COUPON_SLOT_Z: f64 = 30.0;

const TRACEABILITY_BAR_X: f64 = 820.0;
const TRACEABILITY_BAR_Y: f64 = 42.0;
const TRACEABILITY_BAR_Z: f64 = 8.0;
const TRACEABILITY_BAR_POS: (f64, f64) = (0.0, 292.0);
const TRACEABILITY_LAND_COUNT: usize = 10;
const TRACEABILITY_LAND_PITCH: f64 = 76.0;

const ROBOT_SWEEP_CLEARANCE_Y: f64 = 92.0;
const FRONT_SERVICE_CLEARANCE: f64 = 420.0;
const SIDE_SERVICE_CLEARANCE: f64 = 170.0;
const OVERHEAD_WRIST_CLEARANCE_Z: f64 = 260.0;
const METROLOGY_HAND_CLEARANCE_Z: f64 = 72.0;

#[derive(Clone, Copy)]
struct Footprint {
    name: &'static str,
    center: (f64, f64),
    width: f64,
    depth: f64,
}

const COMPONENT_FOOTPRINTS: &[Footprint] = &[
    Footprint {
        name: "kinematic_tool_nest",
        center: TOOL_NEST_POS,
        width: TOOL_NEST_X,
        depth: TOOL_NEST_Y,
    },
    Footprint {
        name: "dial_indicator_lvdt_mount",
        center: INDICATOR_POS,
        width: INDICATOR_BASE_X,
        depth: INDICATOR_BASE_Y,
    },
    Footprint {
        name: "camera_fiducial_bridge",
        center: CAMERA_BRIDGE_POS,
        width: CAMERA_BRIDGE_X,
        depth: CAMERA_BRIDGE_Y,
    },
    Footprint {
        name: "probe_target_array",
        center: TARGET_ARRAY_POS,
        width: TARGET_PLATE_X,
        depth: TARGET_PLATE_Y,
    },
    Footprint {
        name: "go_no_go_gauge_bank",
        center: GAUGE_BANK_POS,
        width: GAUGE_BANK_X,
        depth: GAUGE_BANK_Y,
    },
    Footprint {
        name: "tool_changer_datum_pin_plate",
        center: DATUM_PLATE_POS,
        width: DATUM_PLATE_X,
        depth: DATUM_PLATE_Y,
    },
    Footprint {
        name: "calibration_coupon_storage",
        center: COUPON_STORAGE_POS,
        width: COUPON_STORAGE_X,
        depth: COUPON_STORAGE_Y,
    },
    Footprint {
        name: "traceability_label_lands",
        center: TRACEABILITY_BAR_POS,
        width: TRACEABILITY_BAR_X,
        depth: TRACEABILITY_BAR_Y,
    },
];

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_sanity();

    let base = base_tray();
    export(OUTPUTS[0], &base);

    let nest = kinematic_tool_nest();
    export(OUTPUTS[1], &nest);

    let indicator = dial_indicator_lvdt_mount();
    export(OUTPUTS[2], &indicator);

    let camera = camera_fiducial_bridge();
    export(OUTPUTS[3], &camera);

    let targets = probe_target_array();
    export(OUTPUTS[4], &targets);

    let gauges = go_no_go_gauge_bank();
    export(OUTPUTS[5], &gauges);

    let datums = tool_changer_datum_pin_plate();
    export(OUTPUTS[6], &datums);

    let coupons = calibration_coupon_storage();
    export(OUTPUTS[7], &coupons);

    let traceability = traceability_label_lands();
    export(OUTPUTS[8], &traceability);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[9], &keepouts);

    let assembly = base
        + nest
        + indicator
        + camera
        + targets
        + gauges
        + datums
        + coupons
        + traceability
        + keepouts;
    export(OUTPUTS[10], &assembly);

    println!();
    println!("Closed robotic tool-tip runout/repeatability station:");
    println!("  Generator:                  {BIN_PREFIX}");
    println!("  Footprint:                  {STATION_X:.0}mm x {STATION_Y:.0}mm");
    println!(
        "  Tool nest:                  {KINEMATIC_CONTACT_COUNT}-point kinematic nest with {NEST_APPROACH_FUNNEL_COUNT} lead-in funnels, {TOOL_SHANK_BORE_D:.0}mm shank bore, {TOOL_TIP_CLEARANCE_D:.0}mm tip clearance"
    );
    println!(
        "  Metrology:                  dial-indicator/LVDT clamp, {CAMERA_FIDUCIAL_COUNT} camera fiducials, {PROBE_TARGET_COUNT} probe target pads"
    );
    println!(
        "  Gauge set:                  {GO_GAUGE_COUNT} go pins, {NO_GO_GAUGE_COUNT} no-go pins, {RING_GAUGE_COUNT} ring gauge sockets"
    );
    println!(
        "  Datum/coupons:              {TOOL_CHANGER_DATUM_PIN_COUNT} tool changer datum pins, {CALIBRATION_COUPON_COUNT} calibration coupon slots"
    );
    println!(
        "  Base references:            {MOUNT_SLOT_COUNT} mount slots, {BASE_DATUM_TARGET_COUNT} deck datum targets"
    );
    println!(
        "  Clearances:                 {FRONT_SERVICE_CLEARANCE:.0}mm front service, {SIDE_SERVICE_CLEARANCE:.0}mm side service, {OVERHEAD_WRIST_CLEARANCE_Z:.0}mm overhead wrist"
    );
    println!("  Feature groups covered:     {}", REQUIRED_FEATURES.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_design_sanity() {
    assert_eq!(
        MOUNT_SLOT_COUNT, 8,
        "station base mount slot count changed unexpectedly"
    );
    assert_eq!(
        BASE_DATUM_TARGET_COUNT, 4,
        "station base datum target count changed unexpectedly"
    );
    assert_eq!(
        NEST_APPROACH_FUNNEL_COUNT, 2,
        "tool nest should keep paired approach funnels"
    );
    for footprint in COMPONENT_FOOTPRINTS {
        assert!(
            fits_on_station(footprint.center, footprint.width, footprint.depth),
            "{} exceeds station usable envelope",
            footprint.name
        );
    }
    assert!(
        kinematic_contact_triangle_area() > 5_000.0,
        "kinematic contact triangle is too small for stable tool seating"
    );
    assert!(
        probe_target_span_x() < TARGET_PLATE_X - 42.0,
        "probe target columns exceed plate width"
    );
    assert!(
        probe_target_span_y() < TARGET_PLATE_Y - 38.0,
        "probe target rows exceed plate depth"
    );
    assert!(
        datum_pin_span_x() <= DATUM_PLATE_X - 58.0,
        "tool changer datum pins exceed datum plate width"
    );
    assert!(
        coupon_slot_span_x() <= COUPON_STORAGE_X - 58.0,
        "calibration coupon slots exceed storage cassette width"
    );
    assert!(
        indicator_stylus_reaches_target(),
        "indicator/LVDT stylus does not reach the runout master target"
    );
    assert!(
        highest_station_feature_z() < OVERHEAD_WRIST_CLEARANCE_Z,
        "fixture geometry enters robot wrist keepout"
    );
}

fn fits_on_station(center: (f64, f64), width: f64, depth: f64) -> bool {
    let usable_x = STATION_X / 2.0 - BASE_RIM_W - 8.0;
    let usable_y = STATION_Y / 2.0 - BASE_RIM_W - 8.0;
    center.0.abs() + width / 2.0 <= usable_x && center.1.abs() + depth / 2.0 <= usable_y
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn kinematic_contact_triangle_area() -> f64 {
    0.5 * KINEMATIC_CONTACT_PITCH_X * KINEMATIC_CONTACT_PITCH_Y
}

fn probe_target_span_x() -> f64 {
    (TARGET_COLS as f64 - 1.0) * TARGET_PITCH_X + TARGET_PAD_D
}

fn probe_target_span_y() -> f64 {
    (TARGET_ROWS as f64 - 1.0) * TARGET_PITCH_Y + TARGET_PAD_D
}

fn datum_pin_span_x() -> f64 {
    TOOL_CHANGER_PIN_SPAN_X + TOOL_CHANGER_DATUM_PIN_D
}

fn coupon_slot_span_x() -> f64 {
    (CALIBRATION_COUPON_COUNT as f64 - 1.0) * COUPON_SLOT_PITCH_X + COUPON_SLOT_X
}

fn indicator_stylus_reaches_target() -> bool {
    let indicator_tip_x = INDICATOR_POS.0 - INDICATOR_STYLUS_CLEARANCE;
    let target_right_x = TARGET_ARRAY_POS.0 + TARGET_PLATE_X / 2.0;
    indicator_tip_x <= target_right_x + RUNOUT_MASTER_D / 2.0
}

fn highest_station_feature_z() -> f64 {
    BASE_Z + CAMERA_UPRIGHT_Z + CAMERA_BEAM_Z
}

fn base_tray() -> Part {
    let deck = centered_cube(
        "closed_robot_runout_station_base_floor",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let sump = centered_cube(
        "closed_robot_runout_station_washdown_sump",
        STATION_X - 116.0,
        STATION_Y - 120.0,
        6.0,
    )
    .translate(0.0, -8.0, BASE_Z / 2.0 - 3.0);
    let drain_gutter = centered_cube(
        "closed_robot_runout_station_front_drain_gutter",
        STATION_X - 170.0,
        DRAIN_GUTTER_Y,
        16.0,
    )
    .translate(0.0, -(STATION_Y / 2.0 - 48.0), BASE_Z / 2.0 - 5.0);
    let drain_port = centered_cylinder(
        "closed_robot_runout_station_front_drain_port",
        8.0,
        40.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 82.0, -(STATION_Y / 2.0 - 30.0), 0.0);

    deck - sump - drain_gutter - drain_port
        + perimeter_rims()
        + base_mount_slots()
        + base_datum_targets()
        + component_socket_reliefs()
}

fn perimeter_rims() -> Part {
    let front = centered_cube(
        "closed_robot_runout_station_front_low_rim",
        STATION_X,
        BASE_RIM_W,
        BASE_RIM_Z,
    )
    .translate(
        0.0,
        -(STATION_Y / 2.0 - BASE_RIM_W / 2.0),
        BASE_Z / 2.0 + BASE_RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "closed_robot_runout_station_rear_datum_rim",
        STATION_X,
        BASE_RIM_W,
        BASE_RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - BASE_RIM_W / 2.0,
        BASE_Z / 2.0 + BASE_RIM_Z / 2.0,
    );
    let left = centered_cube(
        "closed_robot_runout_station_left_rim",
        BASE_RIM_W,
        STATION_Y,
        BASE_RIM_Z,
    )
    .translate(
        -(STATION_X / 2.0 - BASE_RIM_W / 2.0),
        0.0,
        BASE_Z / 2.0 + BASE_RIM_Z / 2.0,
    );
    let right = centered_cube(
        "closed_robot_runout_station_right_rim",
        BASE_RIM_W,
        STATION_Y,
        BASE_RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - BASE_RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + BASE_RIM_Z / 2.0,
    );

    front + rear + left + right
}

fn base_mount_slots() -> Part {
    let mut slots = Part::empty("closed_robot_runout_station_base_mount_slots");
    for (i, (x, y)) in [
        (-392.0, 282.0),
        (-196.0, 282.0),
        (196.0, 282.0),
        (392.0, 282.0),
        (-392.0, -282.0),
        (-196.0, -282.0),
        (196.0, -282.0),
        (392.0, -282.0),
    ]
    .into_iter()
    .enumerate()
    {
        let hole = centered_cylinder(
            format!("closed_robot_runout_station_m6_mount_hole_{i}"),
            3.4,
            BASE_Z + 8.0,
            28,
        )
        .translate(x, y, 0.0);
        let slot = centered_cube(
            format!("closed_robot_runout_station_m6_mount_slot_relief_{i}"),
            25.0,
            7.0,
            BASE_Z + 8.0,
        )
        .translate(x, y, 0.0);
        slots = slots + hole + slot;
    }
    slots
}

fn base_datum_targets() -> Part {
    let mut targets = Part::empty("closed_robot_runout_station_base_datum_targets");
    for (i, (x, y)) in [
        (-408.0, 226.0),
        (408.0, 226.0),
        (-408.0, -226.0),
        (408.0, -226.0),
    ]
    .into_iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("closed_robot_runout_station_base_datum_boss_{i}"),
            13.0,
            8.0,
            36,
        )
        .translate(x, y, BASE_Z / 2.0 + 4.0);
        let center_mark = centered_cylinder(
            format!("closed_robot_runout_station_base_datum_center_mark_{i}"),
            2.0,
            10.0,
            24,
        )
        .translate(x, y, BASE_Z / 2.0 + 4.0);
        targets = targets + (boss - center_mark);
    }
    targets
}

fn component_socket_reliefs() -> Part {
    let mut sockets = Part::empty("closed_robot_runout_station_component_socket_reliefs");
    for footprint in COMPONENT_FOOTPRINTS {
        if footprint.name == "traceability_label_lands"
            || footprint.name == "camera_fiducial_bridge"
        {
            continue;
        }
        sockets = sockets
            + centered_cube(
                format!(
                    "closed_robot_runout_station_{}_shallow_locator_socket",
                    footprint.name
                ),
                footprint.width + 8.0,
                footprint.depth + 8.0,
                5.0,
            )
            .translate(footprint.center.0, footprint.center.1, BASE_Z / 2.0 - 2.4);
    }
    sockets
}

fn kinematic_tool_nest() -> Part {
    let body = centered_cube(
        "closed_robot_runout_kinematic_tool_nest_body",
        TOOL_NEST_X,
        TOOL_NEST_Y,
        TOOL_NEST_Z,
    )
    .translate(0.0, 0.0, TOOL_NEST_Z / 2.0);
    let shank_bore = centered_cylinder(
        "closed_robot_runout_kinematic_nest_tool_shank_bore",
        TOOL_SHANK_BORE_D / 2.0,
        TOOL_NEST_Y + 18.0,
        56,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, 0.0, TOOL_NEST_Z / 2.0 + 3.0);
    let tip_window = centered_cylinder(
        "closed_robot_runout_kinematic_nest_tool_tip_clearance",
        TOOL_TIP_CLEARANCE_D / 2.0,
        TOOL_NEST_X + 18.0,
        36,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -TOOL_NEST_Y / 2.0 + 38.0, TOOL_NEST_Z / 2.0 + 2.0);
    let top_relief = centered_cube(
        "closed_robot_runout_kinematic_nest_top_access_relief",
        TOOL_NEST_X - 66.0,
        TOOL_NEST_Y - 56.0,
        18.0,
    )
    .translate(0.0, -4.0, TOOL_NEST_Z - 6.0);
    let witness_slot = centered_cube(
        "closed_robot_runout_kinematic_nest_runout_witness_slot",
        74.0,
        16.0,
        22.0,
    )
    .translate(0.0, -TOOL_NEST_Y / 2.0 + 24.0, TOOL_NEST_Z / 2.0);

    let nest = body - shank_bore - tip_window - top_relief - witness_slot;
    (nest
        + kinematic_contacts()
        + nest_approach_funnels()
        + nest_latch_and_hard_stop()
        + local_nest_fiducials())
    .translate(TOOL_NEST_POS.0, TOOL_NEST_POS.1, BASE_Z / 2.0)
}

fn kinematic_contacts() -> Part {
    let mut contacts = Part::empty("closed_robot_runout_kinematic_contacts");
    for (i, (x, y)) in [
        (
            -KINEMATIC_CONTACT_PITCH_X / 2.0,
            KINEMATIC_CONTACT_PITCH_Y / 2.0,
        ),
        (
            KINEMATIC_CONTACT_PITCH_X / 2.0,
            KINEMATIC_CONTACT_PITCH_Y / 2.0,
        ),
        (0.0, -KINEMATIC_CONTACT_PITCH_Y / 2.0),
    ]
    .into_iter()
    .enumerate()
    {
        let pad = centered_cylinder(
            format!("closed_robot_runout_kinematic_contact_pad_{i}"),
            KINEMATIC_CONTACT_D / 2.0,
            8.0,
            32,
        )
        .translate(x, y, TOOL_NEST_Z + 4.0);
        let witness = centered_cylinder(
            format!("closed_robot_runout_kinematic_contact_center_witness_{i}"),
            1.5,
            9.0,
            18,
        )
        .translate(x, y, TOOL_NEST_Z + 4.0);
        contacts = contacts + (pad - witness);
    }
    contacts
}

fn nest_approach_funnels() -> Part {
    let mut funnels = Part::empty("closed_robot_runout_nest_approach_funnels");
    for (i, x) in [-72.0, 72.0].into_iter().enumerate() {
        let rail = centered_cube(
            format!("closed_robot_runout_nest_approach_funnel_{i}"),
            22.0,
            TOOL_NEST_Y - 34.0,
            34.0,
        )
        .translate(x, -4.0, TOOL_NEST_Z + 10.0);
        let lead_in = centered_cube(
            format!("closed_robot_runout_nest_approach_lead_in_{i}"),
            38.0,
            42.0,
            18.0,
        )
        .translate(x, -TOOL_NEST_Y / 2.0 + 22.0, TOOL_NEST_Z + 6.0);
        funnels = funnels + rail + lead_in;
    }
    funnels
}

fn nest_latch_and_hard_stop() -> Part {
    let rear_stop = centered_cube(
        "closed_robot_runout_nest_rear_hard_stop",
        TOOL_NEST_X - 52.0,
        16.0,
        42.0,
    )
    .translate(0.0, TOOL_NEST_Y / 2.0 - 22.0, TOOL_NEST_Z + 16.0);
    let latch_left = centered_cube(
        "closed_robot_runout_nest_left_tool_latch_ear",
        28.0,
        58.0,
        22.0,
    )
    .translate(-(TOOL_NEST_X / 2.0 - 28.0), 10.0, TOOL_NEST_Z + 4.0);
    let latch_right = centered_cube(
        "closed_robot_runout_nest_right_tool_latch_ear",
        28.0,
        58.0,
        22.0,
    )
    .translate(TOOL_NEST_X / 2.0 - 28.0, 10.0, TOOL_NEST_Z + 4.0);
    let spring_plunger_boss = centered_cylinder(
        "closed_robot_runout_nest_spring_plunger_boss",
        13.0,
        44.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, TOOL_NEST_Y / 2.0 - 18.0, TOOL_NEST_Z + 14.0);

    rear_stop + latch_left + latch_right + spring_plunger_boss
}

fn local_nest_fiducials() -> Part {
    let mut fiducials = Part::empty("closed_robot_runout_nest_local_fiducials");
    for (i, x) in [-104.0, 104.0].into_iter().enumerate() {
        fiducials = fiducials
            + centered_cylinder(
                format!("closed_robot_runout_nest_local_camera_fiducial_{i}"),
                5.0,
                3.0,
                32,
            )
            .translate(x, -TOOL_NEST_Y / 2.0 + 20.0, TOOL_NEST_Z + 1.5);
    }
    fiducials
}

fn dial_indicator_lvdt_mount() -> Part {
    let base = centered_cube(
        "closed_robot_runout_indicator_base",
        INDICATOR_BASE_X,
        INDICATOR_BASE_Y,
        INDICATOR_BASE_Z,
    )
    .translate(0.0, 0.0, INDICATOR_BASE_Z / 2.0);
    let base_relief = centered_cube(
        "closed_robot_runout_indicator_base_lightweight_recess",
        INDICATOR_BASE_X - 48.0,
        INDICATOR_BASE_Y - 44.0,
        8.0,
    )
    .translate(0.0, 0.0, 5.0);
    let mount = base - base_relief
        + indicator_posts()
        + indicator_cross_slide()
        + indicator_clamp_block()
        + lvdt_reference_body()
        + stylus_guard();
    mount.translate(INDICATOR_POS.0, INDICATOR_POS.1, BASE_Z / 2.0)
}

fn indicator_posts() -> Part {
    let mut posts = Part::empty("closed_robot_runout_indicator_posts");
    for (i, x) in [-86.0, 86.0].into_iter().enumerate() {
        let post = centered_cylinder(
            format!("closed_robot_runout_indicator_post_{i}"),
            INDICATOR_POST_D / 2.0,
            INDICATOR_POST_HEIGHT,
            36,
        )
        .translate(x, 48.0, INDICATOR_BASE_Z + INDICATOR_POST_HEIGHT / 2.0);
        let foot = centered_cylinder(
            format!("closed_robot_runout_indicator_post_foot_{i}"),
            20.0,
            10.0,
            36,
        )
        .translate(x, 48.0, INDICATOR_BASE_Z + 5.0);
        posts = posts + post + foot;
    }
    posts
}

fn indicator_cross_slide() -> Part {
    let rear_beam = centered_cube(
        "closed_robot_runout_indicator_rear_cross_beam",
        208.0,
        22.0,
        28.0,
    )
    .translate(0.0, 48.0, INDICATOR_BASE_Z + INDICATOR_POST_HEIGHT - 26.0);
    let slide_rail = centered_cube(
        "closed_robot_runout_indicator_lvdt_slide_rail",
        186.0,
        16.0,
        18.0,
    )
    .translate(0.0, -4.0, INDICATOR_BASE_Z + INDICATOR_POST_HEIGHT - 42.0);
    let rack_scale = centered_cube("closed_robot_runout_indicator_scale_land", 168.0, 5.0, 12.0)
        .translate(0.0, -17.0, INDICATOR_BASE_Z + INDICATOR_POST_HEIGHT - 28.0);
    rear_beam + slide_rail + rack_scale
}

fn indicator_clamp_block() -> Part {
    let clamp = centered_cube(
        "closed_robot_runout_indicator_lvdt_clamp_block",
        INDICATOR_CLAMP_X,
        INDICATOR_CLAMP_Y,
        INDICATOR_CLAMP_Z,
    )
    .translate(
        -20.0,
        -28.0,
        INDICATOR_BASE_Z + INDICATOR_POST_HEIGHT - 50.0,
    );
    let bore = centered_cylinder(
        "closed_robot_runout_indicator_clamp_bore",
        LVDT_BODY_D / 2.0,
        INDICATOR_CLAMP_X + 12.0,
        44,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(
        -20.0,
        -28.0,
        INDICATOR_BASE_Z + INDICATOR_POST_HEIGHT - 50.0,
    );
    let split = centered_cube(
        "closed_robot_runout_indicator_clamp_split",
        INDICATOR_CLAMP_X + 8.0,
        5.0,
        INDICATOR_CLAMP_Z + 8.0,
    )
    .translate(
        -20.0,
        -28.0,
        INDICATOR_BASE_Z + INDICATOR_POST_HEIGHT - 50.0,
    );
    let clamp_screws = clamp_screw_bosses();
    clamp - bore - split + clamp_screws
}

fn clamp_screw_bosses() -> Part {
    let mut bosses = Part::empty("closed_robot_runout_indicator_clamp_screw_bosses");
    for (i, x) in [-64.0, 24.0].into_iter().enumerate() {
        let boss = centered_cylinder(
            format!("closed_robot_runout_indicator_thumb_screw_boss_{i}"),
            8.0,
            18.0,
            28,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -58.0, INDICATOR_BASE_Z + INDICATOR_POST_HEIGHT - 50.0);
        bosses = bosses + boss;
    }
    bosses
}

fn lvdt_reference_body() -> Part {
    let body = centered_cylinder(
        "closed_robot_runout_lvdt_reference_body_shadow",
        LVDT_BODY_D / 2.0,
        LVDT_BODY_LEN,
        44,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(
        -96.0,
        -28.0,
        INDICATOR_BASE_Z + INDICATOR_POST_HEIGHT - 50.0,
    );
    let stylus = centered_cylinder("closed_robot_runout_lvdt_stylus_guarded_tip", 3.0, 82.0, 20)
        .rotate(0.0, 90.0, 0.0)
        .translate(
            -170.0,
            -28.0,
            INDICATOR_BASE_Z + INDICATOR_POST_HEIGHT - 50.0,
        );
    let contact_foot =
        centered_cylinder("closed_robot_runout_lvdt_flat_contact_foot", 7.5, 5.0, 32)
            .rotate(0.0, 90.0, 0.0)
            .translate(
                -213.0,
                -28.0,
                INDICATOR_BASE_Z + INDICATOR_POST_HEIGHT - 50.0,
            );
    body + stylus + contact_foot
}

fn stylus_guard() -> Part {
    let lower_guard = centered_cube(
        "closed_robot_runout_indicator_lower_stylus_guard",
        92.0,
        12.0,
        18.0,
    )
    .translate(
        -154.0,
        -28.0,
        INDICATOR_BASE_Z + INDICATOR_POST_HEIGHT - 76.0,
    );
    let upper_guard = centered_cube(
        "closed_robot_runout_indicator_upper_stylus_guard",
        92.0,
        12.0,
        18.0,
    )
    .translate(
        -154.0,
        -28.0,
        INDICATOR_BASE_Z + INDICATOR_POST_HEIGHT - 24.0,
    );
    lower_guard + upper_guard
}

fn camera_fiducial_bridge() -> Part {
    let floor_bar = centered_cube(
        "closed_robot_runout_camera_fiducial_floor_bar",
        CAMERA_BRIDGE_X,
        CAMERA_BRIDGE_Y,
        12.0,
    )
    .translate(0.0, 0.0, 6.0);
    let window = centered_cube(
        "closed_robot_runout_camera_fiducial_floor_window",
        CAMERA_WINDOW_X,
        CAMERA_WINDOW_Y,
        14.0,
    )
    .translate(0.0, 0.0, 6.0);
    let bridge = floor_bar - window
        + camera_bridge_uprights()
        + camera_bridge_beam()
        + camera_fiducial_marks()
        + camera_scale_lands();
    bridge.translate(CAMERA_BRIDGE_POS.0, CAMERA_BRIDGE_POS.1, BASE_Z / 2.0)
}

fn camera_bridge_uprights() -> Part {
    let left = centered_cube(
        "closed_robot_runout_camera_left_upright",
        26.0,
        50.0,
        CAMERA_UPRIGHT_Z,
    )
    .translate(-(CAMERA_BRIDGE_X / 2.0 - 54.0), 0.0, CAMERA_UPRIGHT_Z / 2.0);
    let right = centered_cube(
        "closed_robot_runout_camera_right_upright",
        26.0,
        50.0,
        CAMERA_UPRIGHT_Z,
    )
    .translate(CAMERA_BRIDGE_X / 2.0 - 54.0, 0.0, CAMERA_UPRIGHT_Z / 2.0);
    let center = centered_cube(
        "closed_robot_runout_camera_center_service_upright",
        18.0,
        36.0,
        CAMERA_UPRIGHT_Z - 36.0,
    )
    .translate(0.0, 0.0, (CAMERA_UPRIGHT_Z - 36.0) / 2.0);
    left + right + center
}

fn camera_bridge_beam() -> Part {
    let beam = centered_cube(
        "closed_robot_runout_camera_overhead_beam",
        CAMERA_BRIDGE_X - 58.0,
        42.0,
        CAMERA_BEAM_Z,
    )
    .translate(0.0, 0.0, CAMERA_UPRIGHT_Z + CAMERA_BEAM_Z / 2.0);
    let camera_boss = centered_cube("closed_robot_runout_camera_mount_boss", 96.0, 48.0, 20.0)
        .translate(0.0, 0.0, CAMERA_UPRIGHT_Z - 18.0);
    let lens_hole = centered_cylinder("closed_robot_runout_camera_lens_clearance", 18.0, 24.0, 48)
        .translate(0.0, 0.0, CAMERA_UPRIGHT_Z - 18.0);
    beam + (camera_boss - lens_hole)
}

fn camera_fiducial_marks() -> Part {
    let mut marks = Part::empty("closed_robot_runout_camera_fiducial_marks");
    for (i, (x, y)) in [
        (-350.0, -20.0),
        (-250.0, 20.0),
        (-110.0, -20.0),
        (-34.0, 20.0),
        (34.0, -20.0),
        (110.0, 20.0),
        (250.0, -20.0),
        (350.0, 20.0),
    ]
    .into_iter()
    .enumerate()
    {
        let outer = centered_cylinder(
            format!("closed_robot_runout_camera_fiducial_outer_{i}"),
            CAMERA_FIDUCIAL_D / 2.0,
            3.0,
            36,
        )
        .translate(x, y, 13.5);
        let center = centered_cylinder(
            format!("closed_robot_runout_camera_fiducial_center_{i}"),
            2.0,
            4.0,
            24,
        )
        .translate(x, y, 14.0);
        marks = marks + (outer - center);
    }
    marks
}

fn camera_scale_lands() -> Part {
    let mut lands = Part::empty("closed_robot_runout_camera_scale_lands");
    for i in 0..7 {
        let x = centered_index(i, 7, 44.0);
        lands = lands
            + centered_cube(
                format!("closed_robot_runout_camera_scale_tick_{i}"),
                4.0,
                28.0,
                4.0,
            )
            .translate(x, -CAMERA_BRIDGE_Y / 2.0 + 12.0, 14.0);
    }
    lands
}

fn probe_target_array() -> Part {
    let plate = centered_cube(
        "closed_robot_runout_probe_target_array_plate",
        TARGET_PLATE_X,
        TARGET_PLATE_Y,
        TARGET_PLATE_Z,
    )
    .translate(0.0, 0.0, TARGET_PLATE_Z / 2.0);
    let relief = centered_cube(
        "closed_robot_runout_probe_target_array_lightweight_recess",
        TARGET_PLATE_X - 42.0,
        TARGET_PLATE_Y - 38.0,
        6.0,
    )
    .translate(0.0, 0.0, TARGET_PLATE_Z / 2.0 - 3.0);
    let array = plate - relief
        + probe_target_pads()
        + runout_master_target()
        + probe_array_axis_labels()
        + probe_target_guard_rails();
    array.translate(TARGET_ARRAY_POS.0, TARGET_ARRAY_POS.1, BASE_Z / 2.0)
}

fn probe_target_pads() -> Part {
    let mut pads = Part::empty("closed_robot_runout_probe_target_pads");
    for row in 0..TARGET_ROWS {
        for col in 0..TARGET_COLS {
            let x = centered_index(col, TARGET_COLS, TARGET_PITCH_X);
            let y = centered_index(row, TARGET_ROWS, TARGET_PITCH_Y);
            let z = 4.0 + (row as f64 * TARGET_HEIGHT_STEP) + (col as f64 * 0.25);
            let pad = centered_cylinder(
                format!("closed_robot_runout_probe_target_r{row}_c{col}"),
                TARGET_PAD_D / 2.0,
                z,
                36,
            )
            .translate(x, y, TARGET_PLATE_Z + z / 2.0);
            let witness = centered_cylinder(
                format!("closed_robot_runout_probe_target_center_mark_r{row}_c{col}"),
                1.6,
                z + 1.0,
                18,
            )
            .translate(x, y, TARGET_PLATE_Z + z / 2.0);
            pads = pads + (pad - witness);
        }
    }
    pads
}

fn runout_master_target() -> Part {
    let base = centered_cylinder(
        "closed_robot_runout_probe_array_runout_master_disk",
        RUNOUT_MASTER_D / 2.0,
        8.0,
        64,
    )
    .translate(TARGET_PLATE_X / 2.0 - 48.0, 0.0, TARGET_PLATE_Z + 4.0);
    let witness = centered_cylinder(
        "closed_robot_runout_probe_array_runout_master_center",
        3.0,
        10.0,
        32,
    )
    .translate(TARGET_PLATE_X / 2.0 - 48.0, 0.0, TARGET_PLATE_Z + 4.0);
    let sweep_arc_shadow = centered_cube(
        "closed_robot_runout_probe_array_runout_sweep_tangent_land",
        58.0,
        7.0,
        5.0,
    )
    .translate(TARGET_PLATE_X / 2.0 - 48.0, -34.0, TARGET_PLATE_Z + 10.0);
    (base - witness) + sweep_arc_shadow
}

fn probe_array_axis_labels() -> Part {
    let x_label_land = centered_cube(
        "closed_robot_runout_probe_array_x_axis_label_land",
        118.0,
        12.0,
        4.0,
    )
    .translate(0.0, -(TARGET_PLATE_Y / 2.0 - 12.0), TARGET_PLATE_Z + 2.0);
    let y_label_land = centered_cube(
        "closed_robot_runout_probe_array_y_axis_label_land",
        12.0,
        96.0,
        4.0,
    )
    .translate(-(TARGET_PLATE_X / 2.0 - 12.0), 0.0, TARGET_PLATE_Z + 2.0);
    x_label_land + y_label_land
}

fn probe_target_guard_rails() -> Part {
    let rear = centered_cube(
        "closed_robot_runout_probe_target_array_rear_guard_rail",
        TARGET_PLATE_X - 28.0,
        10.0,
        22.0,
    )
    .translate(0.0, TARGET_PLATE_Y / 2.0 - 12.0, TARGET_PLATE_Z + 11.0);
    let left = centered_cube(
        "closed_robot_runout_probe_target_array_left_guard_rail",
        10.0,
        TARGET_PLATE_Y - 32.0,
        18.0,
    )
    .translate(-(TARGET_PLATE_X / 2.0 - 12.0), 0.0, TARGET_PLATE_Z + 9.0);
    rear + left
}

fn go_no_go_gauge_bank() -> Part {
    let body = centered_cube(
        "closed_robot_runout_go_no_go_gauge_bank_body",
        GAUGE_BANK_X,
        GAUGE_BANK_Y,
        GAUGE_BANK_Z,
    )
    .translate(0.0, 0.0, GAUGE_BANK_Z / 2.0);
    let go_sockets = gauge_socket_row("go", GO_GAUGE_COUNT, GO_PIN_D, 30.0);
    let no_go_sockets = gauge_socket_row("no_go", NO_GO_GAUGE_COUNT, NO_GO_PIN_D, -16.0);
    let ring_sockets = ring_gauge_sockets();
    let bank = body - go_sockets - no_go_sockets - ring_sockets
        + gauge_retainer_clips()
        + gauge_label_lands()
        + gauge_handle_reliefs();
    bank.translate(GAUGE_BANK_POS.0, GAUGE_BANK_POS.1, BASE_Z / 2.0)
}

fn gauge_socket_row(prefix: &str, count: usize, diameter: f64, y: f64) -> Part {
    let mut sockets = Part::empty(format!("closed_robot_runout_{prefix}_gauge_sockets"));
    for i in 0..count {
        let x = centered_index(i, count, GAUGE_PITCH_X);
        let socket = centered_cylinder(
            format!("closed_robot_runout_{prefix}_gauge_socket_{i}"),
            diameter / 2.0,
            GAUGE_BANK_Z + 8.0,
            32,
        )
        .translate(x, y, GAUGE_BANK_Z / 2.0);
        let finger = centered_cube(
            format!("closed_robot_runout_{prefix}_gauge_finger_relief_{i}"),
            diameter + 7.0,
            9.0,
            10.0,
        )
        .translate(x, y - 17.0, GAUGE_BANK_Z - 3.0);
        sockets = sockets + socket + finger;
    }
    sockets
}

fn ring_gauge_sockets() -> Part {
    let mut sockets = Part::empty("closed_robot_runout_ring_gauge_sockets");
    for i in 0..RING_GAUGE_COUNT {
        let x = centered_index(i, RING_GAUGE_COUNT, 48.0);
        let outer = centered_cylinder(
            format!("closed_robot_runout_ring_gauge_counterbore_{i}"),
            15.0,
            12.0,
            48,
        )
        .translate(x, -55.0, GAUGE_BANK_Z - 6.0);
        let bore = centered_cylinder(
            format!("closed_robot_runout_ring_gauge_center_relief_{i}"),
            7.0,
            GAUGE_BANK_Z + 8.0,
            36,
        )
        .translate(x, -55.0, GAUGE_BANK_Z / 2.0);
        sockets = sockets + outer + bore;
    }
    sockets
}

fn gauge_retainer_clips() -> Part {
    let go_clip = centered_cube(
        "closed_robot_runout_go_gauge_retainer_clip",
        GAUGE_BANK_X - 44.0,
        8.0,
        12.0,
    )
    .translate(0.0, 53.0, GAUGE_BANK_Z + 6.0);
    let no_go_clip = centered_cube(
        "closed_robot_runout_no_go_gauge_retainer_clip",
        GAUGE_BANK_X - 44.0,
        8.0,
        12.0,
    )
    .translate(0.0, 6.0, GAUGE_BANK_Z + 6.0);
    go_clip + no_go_clip
}

fn gauge_label_lands() -> Part {
    let go_label = centered_cube("closed_robot_runout_go_gauge_label_land", 74.0, 16.0, 4.0)
        .translate(-(GAUGE_BANK_X / 2.0 - 46.0), 54.0, GAUGE_BANK_Z + 2.0);
    let no_go_label = centered_cube(
        "closed_robot_runout_no_go_gauge_label_land",
        92.0,
        16.0,
        4.0,
    )
    .translate(GAUGE_BANK_X / 2.0 - 58.0, 5.0, GAUGE_BANK_Z + 2.0);
    go_label + no_go_label
}

fn gauge_handle_reliefs() -> Part {
    let front_handle = centered_cube(
        "closed_robot_runout_gauge_bank_front_handle_relief",
        92.0,
        12.0,
        16.0,
    )
    .translate(0.0, -(GAUGE_BANK_Y / 2.0 - 12.0), GAUGE_BANK_Z / 2.0);
    let rear_handle = centered_cube(
        "closed_robot_runout_gauge_bank_rear_handle_relief",
        92.0,
        12.0,
        16.0,
    )
    .translate(0.0, GAUGE_BANK_Y / 2.0 - 12.0, GAUGE_BANK_Z / 2.0);
    front_handle + rear_handle
}

fn tool_changer_datum_pin_plate() -> Part {
    let plate = centered_cube(
        "closed_robot_runout_tool_changer_datum_plate",
        DATUM_PLATE_X,
        DATUM_PLATE_Y,
        DATUM_PLATE_Z,
    )
    .translate(0.0, 0.0, DATUM_PLATE_Z / 2.0);
    let relief = centered_cube(
        "closed_robot_runout_tool_changer_datum_plate_relief",
        DATUM_PLATE_X - 44.0,
        DATUM_PLATE_Y - 38.0,
        6.0,
    )
    .translate(0.0, 0.0, DATUM_PLATE_Z / 2.0 - 3.0);
    let plate = plate - relief
        + tool_changer_datum_pins()
        + tool_changer_bushing_reliefs()
        + datum_plate_label_lands();
    plate.translate(DATUM_PLATE_POS.0, DATUM_PLATE_POS.1, BASE_Z / 2.0)
}

fn tool_changer_datum_pins() -> Part {
    let mut pins = Part::empty("closed_robot_runout_tool_changer_datum_pins");
    for (i, (x, y)) in [
        (
            -TOOL_CHANGER_PIN_SPAN_X / 2.0,
            -TOOL_CHANGER_PIN_SPAN_Y / 2.0,
        ),
        (
            TOOL_CHANGER_PIN_SPAN_X / 2.0,
            -TOOL_CHANGER_PIN_SPAN_Y / 2.0,
        ),
        (
            -TOOL_CHANGER_PIN_SPAN_X / 2.0,
            TOOL_CHANGER_PIN_SPAN_Y / 2.0,
        ),
        (TOOL_CHANGER_PIN_SPAN_X / 2.0, TOOL_CHANGER_PIN_SPAN_Y / 2.0),
    ]
    .into_iter()
    .enumerate()
    {
        let pin = centered_cylinder(
            format!("closed_robot_runout_tool_changer_datum_pin_{i}"),
            TOOL_CHANGER_DATUM_PIN_D / 2.0,
            TOOL_CHANGER_DATUM_PIN_Z,
            36,
        )
        .translate(x, y, DATUM_PLATE_Z + TOOL_CHANGER_DATUM_PIN_Z / 2.0);
        let shoulder = centered_cylinder(
            format!("closed_robot_runout_tool_changer_datum_pin_shoulder_{i}"),
            9.0,
            5.0,
            36,
        )
        .translate(x, y, DATUM_PLATE_Z + 2.5);
        pins = pins + pin + shoulder;
    }
    pins
}

fn tool_changer_bushing_reliefs() -> Part {
    let mut bushings = Part::empty("closed_robot_runout_tool_changer_bushing_reliefs");
    for (i, x) in [-54.0, 0.0, 54.0].into_iter().enumerate() {
        let bushing = centered_cylinder(
            format!("closed_robot_runout_tool_changer_bushing_socket_{i}"),
            7.5,
            DATUM_PLATE_Z + 8.0,
            36,
        )
        .translate(x, 0.0, DATUM_PLATE_Z / 2.0);
        bushings = bushings + bushing;
    }
    bushings
}

fn datum_plate_label_lands() -> Part {
    let rear_label = centered_cube(
        "closed_robot_runout_tool_changer_datum_plate_rear_label_land",
        118.0,
        10.0,
        4.0,
    )
    .translate(0.0, DATUM_PLATE_Y / 2.0 - 12.0, DATUM_PLATE_Z + 2.0);
    let front_label = centered_cube(
        "closed_robot_runout_tool_changer_datum_plate_front_label_land",
        118.0,
        10.0,
        4.0,
    )
    .translate(0.0, -(DATUM_PLATE_Y / 2.0 - 12.0), DATUM_PLATE_Z + 2.0);
    rear_label + front_label
}

fn calibration_coupon_storage() -> Part {
    let body = centered_cube(
        "closed_robot_runout_calibration_coupon_storage_body",
        COUPON_STORAGE_X,
        COUPON_STORAGE_Y,
        COUPON_STORAGE_Z,
    )
    .translate(0.0, 0.0, COUPON_STORAGE_Z / 2.0);
    let coupon_slots = calibration_coupon_slots();
    let lid_rails = coupon_lid_rails();
    let desiccant_pocket = centered_cube(
        "closed_robot_runout_calibration_coupon_desiccant_pocket",
        46.0,
        62.0,
        20.0,
    )
    .translate(
        COUPON_STORAGE_X / 2.0 - 36.0,
        0.0,
        COUPON_STORAGE_Z / 2.0 + 4.0,
    );
    let body = body - coupon_slots - desiccant_pocket
        + lid_rails
        + coupon_status_lands()
        + coupon_robot_pick_tabs();
    body.translate(COUPON_STORAGE_POS.0, COUPON_STORAGE_POS.1, BASE_Z / 2.0)
}

fn calibration_coupon_slots() -> Part {
    let mut slots = Part::empty("closed_robot_runout_calibration_coupon_slots");
    for i in 0..CALIBRATION_COUPON_COUNT {
        let x = centered_index(i, CALIBRATION_COUPON_COUNT, COUPON_SLOT_PITCH_X) - 18.0;
        let slot = centered_cube(
            format!("closed_robot_runout_calibration_coupon_slot_{i}"),
            COUPON_SLOT_X,
            COUPON_SLOT_Y,
            COUPON_SLOT_Z,
        )
        .translate(x, 0.0, COUPON_STORAGE_Z / 2.0 + 6.0);
        let finger = centered_cylinder(
            format!("closed_robot_runout_calibration_coupon_finger_relief_{i}"),
            8.0,
            12.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -COUPON_STORAGE_Y / 2.0 + 12.0, COUPON_STORAGE_Z - 6.0);
        slots = slots + slot + finger;
    }
    slots
}

fn coupon_lid_rails() -> Part {
    let left = centered_cube(
        "closed_robot_runout_coupon_storage_left_lid_rail",
        COUPON_STORAGE_X - 38.0,
        8.0,
        12.0,
    )
    .translate(
        -16.0,
        -(COUPON_STORAGE_Y / 2.0 - 8.0),
        COUPON_STORAGE_Z + 6.0,
    );
    let right = centered_cube(
        "closed_robot_runout_coupon_storage_right_lid_rail",
        COUPON_STORAGE_X - 38.0,
        8.0,
        12.0,
    )
    .translate(-16.0, COUPON_STORAGE_Y / 2.0 - 8.0, COUPON_STORAGE_Z + 6.0);
    left + right
}

fn coupon_status_lands() -> Part {
    let released = centered_cube(
        "closed_robot_runout_coupon_released_label_land",
        74.0,
        14.0,
        4.0,
    )
    .translate(
        -104.0,
        COUPON_STORAGE_Y / 2.0 - 18.0,
        COUPON_STORAGE_Z + 2.0,
    );
    let hold = centered_cube(
        "closed_robot_runout_coupon_hold_label_land",
        58.0,
        14.0,
        4.0,
    )
    .translate(-16.0, COUPON_STORAGE_Y / 2.0 - 18.0, COUPON_STORAGE_Z + 2.0);
    let expired = centered_cube(
        "closed_robot_runout_coupon_expired_label_land",
        74.0,
        14.0,
        4.0,
    )
    .translate(78.0, COUPON_STORAGE_Y / 2.0 - 18.0, COUPON_STORAGE_Z + 2.0);
    released + hold + expired
}

fn coupon_robot_pick_tabs() -> Part {
    let left = centered_cube(
        "closed_robot_runout_coupon_storage_left_robot_pick_tab",
        28.0,
        18.0,
        18.0,
    )
    .translate(
        -(COUPON_STORAGE_X / 2.0 - 20.0),
        0.0,
        COUPON_STORAGE_Z + 2.0,
    );
    let right = centered_cube(
        "closed_robot_runout_coupon_storage_right_robot_pick_tab",
        28.0,
        18.0,
        18.0,
    )
    .translate(COUPON_STORAGE_X / 2.0 - 20.0, 0.0, COUPON_STORAGE_Z + 2.0);
    left + right
}

fn traceability_label_lands() -> Part {
    let bar = centered_cube(
        "closed_robot_runout_traceability_bar",
        TRACEABILITY_BAR_X,
        TRACEABILITY_BAR_Y,
        TRACEABILITY_BAR_Z,
    )
    .translate(0.0, 0.0, TRACEABILITY_BAR_Z / 2.0);
    let mut lands = Part::empty("closed_robot_runout_traceability_label_lands");
    for i in 0..TRACEABILITY_LAND_COUNT {
        let x = centered_index(i, TRACEABILITY_LAND_COUNT, TRACEABILITY_LAND_PITCH);
        lands = lands
            + centered_cube(
                format!("closed_robot_runout_traceability_label_land_{i}"),
                56.0,
                24.0,
                4.0,
            )
            .translate(x, 0.0, TRACEABILITY_BAR_Z + 2.0)
            + centered_cylinder(
                format!("closed_robot_runout_traceability_barcode_dot_{i}"),
                3.0,
                5.0,
                18,
            )
            .translate(x + 22.0, 0.0, TRACEABILITY_BAR_Z + 2.5);
    }
    (bar + lands).translate(TRACEABILITY_BAR_POS.0, TRACEABILITY_BAR_POS.1, BASE_Z / 2.0)
}

fn robot_service_keepouts() -> Part {
    let front = centered_cube(
        "closed_robot_runout_front_service_keepout_shadow",
        STATION_X - 96.0,
        18.0,
        42.0,
    )
    .translate(
        0.0,
        -(STATION_Y / 2.0 + FRONT_SERVICE_CLEARANCE / 2.0),
        21.0,
    );
    let left = centered_cube(
        "closed_robot_runout_left_service_keepout_shadow",
        16.0,
        STATION_Y - 120.0,
        42.0,
    )
    .translate(-(STATION_X / 2.0 + SIDE_SERVICE_CLEARANCE / 2.0), 0.0, 21.0);
    let right = centered_cube(
        "closed_robot_runout_right_service_keepout_shadow",
        16.0,
        STATION_Y - 120.0,
        42.0,
    )
    .translate(STATION_X / 2.0 + SIDE_SERVICE_CLEARANCE / 2.0, 0.0, 21.0);
    let robot_sweep = centered_cube(
        "closed_robot_runout_robot_tool_sweep_keepout_window",
        STATION_X - 140.0,
        ROBOT_SWEEP_CLEARANCE_Y,
        24.0,
    )
    .translate(0.0, 48.0, OVERHEAD_WRIST_CLEARANCE_Z - 12.0);
    let metrology_hand_space = centered_cube(
        "closed_robot_runout_metrology_hand_clearance_window",
        380.0,
        54.0,
        18.0,
    )
    .translate(68.0, -8.0, METROLOGY_HAND_CLEARANCE_Z);

    front + left + right + robot_sweep + metrology_hand_space
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_manifest_matches_feature_scope() {
        assert_eq!(OUTPUTS.len(), 11);
        assert_eq!(REQUIRED_FEATURES.len(), 10);
        assert!(OUTPUTS.iter().all(|path| path.contains(BIN_PREFIX)));
    }

    #[test]
    fn component_footprints_fit_station() {
        for footprint in COMPONENT_FOOTPRINTS {
            assert!(
                fits_on_station(footprint.center, footprint.width, footprint.depth),
                "{} does not fit",
                footprint.name
            );
        }
    }

    #[test]
    fn kinematic_nest_has_stable_three_point_contact() {
        assert_eq!(KINEMATIC_CONTACT_COUNT, 3);
        assert_eq!(NEST_APPROACH_FUNNEL_COUNT, 2);
        assert!(kinematic_contact_triangle_area() > 5_000.0);
        assert!(TOOL_SHANK_BORE_D > TOOL_TIP_CLEARANCE_D);
    }

    #[test]
    fn probe_target_array_has_repeatability_grid() {
        assert_eq!(PROBE_TARGET_COUNT, 20);
        assert!(probe_target_span_x() < TARGET_PLATE_X - 42.0);
        assert!(probe_target_span_y() < TARGET_PLATE_Y - 38.0);
        assert!(TARGET_HEIGHT_STEP > 0.5);
    }

    #[test]
    fn go_no_go_gauge_bank_is_balanced() {
        assert_eq!(GO_GAUGE_COUNT, NO_GO_GAUGE_COUNT);
        assert!(NO_GO_PIN_D > GO_PIN_D);
        assert_eq!(RING_GAUGE_COUNT, 4);
    }

    #[test]
    fn datum_and_coupon_storage_capacity_are_explicit() {
        assert_eq!(TOOL_CHANGER_DATUM_PIN_COUNT, 4);
        assert!(datum_pin_span_x() <= DATUM_PLATE_X - 58.0);
        assert_eq!(CALIBRATION_COUPON_COUNT, 8);
        assert!(coupon_slot_span_x() <= COUPON_STORAGE_X - 58.0);
    }

    #[test]
    fn metrology_geometry_preserves_robot_clearance() {
        assert!(indicator_stylus_reaches_target());
        assert!(highest_station_feature_z() < OVERHEAD_WRIST_CLEARANCE_Z);
        assert!(FRONT_SERVICE_CLEARANCE > 360.0);
        assert!(SIDE_SERVICE_CLEARANCE >= 150.0);
    }

    #[test]
    fn base_reference_features_are_counted() {
        assert_eq!(MOUNT_SLOT_COUNT, 8);
        assert_eq!(BASE_DATUM_TARGET_COUNT, 4);
        assert_eq!(TRACEABILITY_LAND_COUNT, REQUIRED_FEATURES.len());
    }
}
