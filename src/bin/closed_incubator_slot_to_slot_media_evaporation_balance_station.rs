use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed incubator slot-to-slot media evaporation balance station.
//
// This standalone no-cell validation fixture challenges a 16-slot incubator rack
// with identical media surrogates and measures whether edge, corner, and center
// positions evaporate differently. The CAD captures the mechanical controls
// needed for repeatable comparison: identical slot pockets, per-slot gravimetric
// witness nests, RH/dewpoint logger positions, condensate-shadow shields, a
// passive humidity equalization manifold, pre/post weighback custody lands,
// release/hold/reject lanes, and robot/service keepout gauges.

const OUTPUT_PREFIX: &str = "closed_incubator_slot_to_slot_media_evaporation_balance_station";

const OUTPUTS: [&str; 12] = [
    "output/closed_incubator_slot_to_slot_media_evaporation_balance_station_base_tray.stl",
    "output/closed_incubator_slot_to_slot_media_evaporation_balance_station_sixteen_slot_balance_rack.stl",
    "output/closed_incubator_slot_to_slot_media_evaporation_balance_station_media_loss_microreservoir_nests.stl",
    "output/closed_incubator_slot_to_slot_media_evaporation_balance_station_rh_dewpoint_logger_grid.stl",
    "output/closed_incubator_slot_to_slot_media_evaporation_balance_station_edge_center_blank_reference_coupons.stl",
    "output/closed_incubator_slot_to_slot_media_evaporation_balance_station_condensate_shadow_drip_shields.stl",
    "output/closed_incubator_slot_to_slot_media_evaporation_balance_station_humidity_equalization_manifold.stl",
    "output/closed_incubator_slot_to_slot_media_evaporation_balance_station_weighback_barcode_custody_lands.stl",
    "output/closed_incubator_slot_to_slot_media_evaporation_balance_station_release_hold_reject_lanes.stl",
    "output/closed_incubator_slot_to_slot_media_evaporation_balance_station_evidence_bridge.stl",
    "output/closed_incubator_slot_to_slot_media_evaporation_balance_station_robot_service_keepouts.stl",
    "output/closed_incubator_slot_to_slot_media_evaporation_balance_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 13] = [
    "sixteen_slot_balance_rack",
    "edge_corner_center_slot_map",
    "media_loss_microreservoir_nests",
    "rh_dewpoint_logger_grid",
    "edge_center_blank_reference_coupons",
    "condensate_shadow_drip_shields",
    "humidity_equalization_manifold",
    "pre_post_weighback_lands",
    "barcode_custody_lands",
    "release_hold_reject_lanes",
    "evidence_bridge",
    "robot_service_keepouts",
    "named_stl_outputs",
];

const SLOT_COLS: usize = 4;
const SLOT_ROWS: usize = 4;
const SLOT_COUNT: usize = SLOT_COLS * SLOT_ROWS;
const CORNER_SLOT_COUNT: usize = 4;
const EDGE_SLOT_COUNT: usize = 8;
const CENTER_SLOT_COUNT: usize = 4;
const POSITION_CLASS_COUNT: usize = 3;

const STATION_X: f64 = 1260.0;
const STATION_Y: f64 = 980.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const SOCKET_DEPTH: f64 = 5.0;
const MOUNT_SLOT_COUNT: usize = 8;
const DATUM_TARGET_COUNT: usize = 4;

const SLOT_ARTICLE_X: f64 = REVC_CHIP_LENGTH + 18.0;
const SLOT_ARTICLE_Y: f64 = REVC_CHIP_WIDTH + 18.0;
const SLOT_ARTICLE_Z: f64 = REVC_TOTAL_HEIGHT + 16.0;
const SLOT_PITCH_X: f64 = REVC_CHIP_LENGTH + 34.0;
const SLOT_PITCH_Y: f64 = REVC_CHIP_WIDTH + 34.0;
const RACK_X: f64 = SLOT_COLS as f64 * SLOT_ARTICLE_X + (SLOT_COLS as f64 - 1.0) * 16.0 + 58.0;
const RACK_Y: f64 = SLOT_ROWS as f64 * SLOT_ARTICLE_Y + (SLOT_ROWS as f64 - 1.0) * 16.0 + 58.0;
const RACK_Z: f64 = 34.0;
const RACK_POS: (f64, f64) = (-250.0, 104.0);
const SLOT_RECESS_DEPTH: f64 = 8.0;
const SLOT_AIR_GAP_MIN: f64 = 7.0;
const SLOT_AIR_BYPASS_COUNT: usize = 24;
const SLOT_REFERENCE_RAIL_Z: f64 = 26.0;

const MICRO_NEST_PANEL_X: f64 = 372.0;
const MICRO_NEST_PANEL_Y: f64 = 214.0;
const MICRO_NEST_PANEL_Z: f64 = 32.0;
const MICRO_NEST_POS: (f64, f64) = (375.0, 196.0);
const MICRO_RESERVOIRS_PER_SLOT: usize = 3;
const MICRO_RESERVOIR_COUNT: usize = SLOT_COUNT * MICRO_RESERVOIRS_PER_SLOT;
const MICRO_RESERVOIR_D: f64 = 14.0;
const MICRO_RESERVOIR_DEPTH: f64 = 14.0;
const MICRO_NEST_PITCH_X: f64 = 78.0;
const MICRO_NEST_PITCH_Y: f64 = 44.0;
const MASS_LOSS_RESOLUTION_MG: f64 = 2.0;
const MEDIA_SURROGATE_VOLUME_UL: f64 = 220.0;

const LOGGER_PANEL_X: f64 = 372.0;
const LOGGER_PANEL_Y: f64 = 150.0;
const LOGGER_PANEL_Z: f64 = 36.0;
const LOGGER_POS: (f64, f64) = (375.0, 364.0);
const SLOT_LOGGER_COUNT: usize = SLOT_COUNT;
const DEWPOINT_REFERENCE_COUNT: usize = 4;
const LOGGER_POCKET_X: f64 = 42.0;
const LOGGER_POCKET_Y: f64 = 25.0;
const LOGGER_POCKET_DEPTH: f64 = 14.0;
const LOGGER_DIFFUSION_SLOT_COUNT: usize = 12;

const COUPON_PANEL_X: f64 = 372.0;
const COUPON_PANEL_Y: f64 = 150.0;
const COUPON_PANEL_Z: f64 = 30.0;
const COUPON_POS: (f64, f64) = (375.0, 34.0);
const BLANK_REFERENCE_COUPON_COUNT: usize = SLOT_COUNT;
const DRY_REFERENCE_COUPON_COUNT: usize = POSITION_CLASS_COUNT * 2;
const COUPON_X: f64 = 38.0;
const COUPON_Y: f64 = 24.0;
const COUPON_RECESS_DEPTH: f64 = 6.0;

const SHIELD_PANEL_X: f64 = RACK_X + 24.0;
const SHIELD_PANEL_Y: f64 = RACK_Y + 18.0;
const SHIELD_PANEL_Z: f64 = 4.0;
const SHIELD_STANDOFF_Z: f64 = 42.0;
const CONDENSATE_GUTTER_W: f64 = 12.0;
const CONDENSATE_GUTTER_DEPTH: f64 = 7.0;
const CONDENSATE_WITNESS_COUNT: usize = SLOT_COUNT;

const MANIFOLD_X: f64 = 690.0;
const MANIFOLD_Y: f64 = 122.0;
const MANIFOLD_Z: f64 = 34.0;
const MANIFOLD_POS: (f64, f64) = (-240.0, -234.0);
const HUMIDITY_PORT_COUNT: usize = SLOT_COUNT;
const MANIFOLD_RESTRICTOR_COUNT: usize = SLOT_COUNT;
const INLET_PORT_D: f64 = 18.0;
const OUTLET_PORT_D: f64 = 18.0;
const MANIFOLD_SLOT_W: f64 = 18.0;
const MANIFOLD_SLOT_Y: f64 = 58.0;

const CUSTODY_PANEL_X: f64 = 424.0;
const CUSTODY_PANEL_Y: f64 = 104.0;
const CUSTODY_PANEL_Z: f64 = 18.0;
const CUSTODY_POS: (f64, f64) = (310.0, -166.0);
const PRE_WEIGHT_LANDS: usize = SLOT_COUNT;
const POST_WEIGHT_LANDS: usize = SLOT_COUNT;
const BARCODE_LAND_COUNT: usize = SLOT_COUNT + DEWPOINT_REFERENCE_COUNT;
const WEIGH_LAND_X: f64 = 34.0;
const WEIGH_LAND_Y: f64 = 18.0;
const BARCODE_LAND_X: f64 = 42.0;
const BARCODE_LAND_Y: f64 = 13.0;

const LANE_BANK_X: f64 = 450.0;
const LANE_BANK_Y: f64 = 102.0;
const LANE_BANK_Z: f64 = 24.0;
const LANE_POS: (f64, f64) = (-300.0, -366.0);
const DECISION_LANE_COUNT: usize = 3;
const LANE_TOKEN_COUNT: usize = SLOT_COUNT;
const LANE_RIB_W: f64 = 8.0;

const EVIDENCE_BRIDGE_X: f64 = 1050.0;
const EVIDENCE_BRIDGE_Y: f64 = 56.0;
const EVIDENCE_BRIDGE_Z: f64 = 94.0;
const EVIDENCE_POS: (f64, f64) = (0.0, 420.0);
const EVIDENCE_CAMERA_COUNT: usize = 5;
const LIGHT_PIPE_COUNT: usize = 9;
const SCALE_WITNESS_WINDOW_COUNT: usize = POSITION_CLASS_COUNT;

const KEEP_OUT_X: f64 = STATION_X - 116.0;
const KEEP_OUT_Y: f64 = STATION_Y - 110.0;
const KEEP_OUT_Z: f64 = 5.0;
const ROBOT_APPROACH_CLEARANCE: f64 = 42.0;
const SERVICE_SWEEP_CLEARANCE: f64 = 34.0;
const VERTICAL_PICK_CLEARANCE_Z: f64 = 158.0;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SlotClass {
    Corner,
    Edge,
    Center,
}

impl SlotClass {
    fn label(self) -> &'static str {
        match self {
            SlotClass::Corner => "corner",
            SlotClass::Edge => "edge",
            SlotClass::Center => "center",
        }
    }

    fn token_diameter(self) -> f64 {
        match self {
            SlotClass::Corner => 18.0,
            SlotClass::Edge => 15.0,
            SlotClass::Center => 12.0,
        }
    }
}

#[derive(Clone, Copy)]
struct Footprint {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_tray();
    export(OUTPUTS[0], &base);

    let rack = sixteen_slot_balance_rack();
    export(OUTPUTS[1], &rack);

    let reservoirs = media_loss_microreservoir_nests();
    export(OUTPUTS[2], &reservoirs);

    let loggers = rh_dewpoint_logger_grid();
    export(OUTPUTS[3], &loggers);

    let coupons = edge_center_blank_reference_coupons();
    export(OUTPUTS[4], &coupons);

    let shields = condensate_shadow_drip_shields();
    export(OUTPUTS[5], &shields);

    let manifold = humidity_equalization_manifold();
    export(OUTPUTS[6], &manifold);

    let custody = weighback_barcode_custody_lands();
    export(OUTPUTS[7], &custody);

    let lanes = release_hold_reject_lanes();
    export(OUTPUTS[8], &lanes);

    let bridge = evidence_bridge();
    export(OUTPUTS[9], &bridge);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let assembly = base
        + rack.translate(RACK_POS.0, RACK_POS.1, on_base_z(RACK_Z))
        + reservoirs.translate(
            MICRO_NEST_POS.0,
            MICRO_NEST_POS.1,
            on_base_z(MICRO_NEST_PANEL_Z),
        )
        + loggers.translate(LOGGER_POS.0, LOGGER_POS.1, on_base_z(LOGGER_PANEL_Z))
        + coupons.translate(COUPON_POS.0, COUPON_POS.1, on_base_z(COUPON_PANEL_Z))
        + shields.translate(
            RACK_POS.0,
            RACK_POS.1,
            BASE_Z / 2.0 + SHIELD_STANDOFF_Z + SHIELD_PANEL_Z / 2.0,
        )
        + manifold.translate(MANIFOLD_POS.0, MANIFOLD_POS.1, on_base_z(MANIFOLD_Z))
        + custody.translate(CUSTODY_POS.0, CUSTODY_POS.1, on_base_z(CUSTODY_PANEL_Z))
        + lanes.translate(LANE_POS.0, LANE_POS.1, on_base_z(LANE_BANK_Z))
        + bridge.translate(EVIDENCE_POS.0, EVIDENCE_POS.1, on_base_z(EVIDENCE_BRIDGE_Z))
        + keepouts.translate(0.0, 0.0, BASE_Z / 2.0 + KEEP_OUT_Z / 2.0);
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed incubator slot-to-slot media evaporation balance station:");
    println!(
        "  Slot challenge:        {SLOT_COUNT} no-cell media surrogates in a {SLOT_COLS}x{SLOT_ROWS} rack ({CORNER_SLOT_COUNT} corners, {EDGE_SLOT_COUNT} edge-only, {CENTER_SLOT_COUNT} centers)"
    );
    println!(
        "  Media witnesses:       {MICRO_RESERVOIR_COUNT} micro-reservoir nests at {MEDIA_SURROGATE_VOLUME_UL:.0}uL nominal volume and {MASS_LOSS_RESOLUTION_MG:.1}mg weighback resolution target"
    );
    println!(
        "  Environmental sensing: {SLOT_LOGGER_COUNT} slot RH logger pockets, {DEWPOINT_REFERENCE_COUNT} dewpoint references, {CONDENSATE_WITNESS_COUNT} condensate witness lands"
    );
    println!(
        "  Equalization:          {HUMIDITY_PORT_COUNT} matched humidity ports and {MANIFOLD_RESTRICTOR_COUNT} restrictor coupons in one passive manifold"
    );
    println!(
        "  Traceability:          {PRE_WEIGHT_LANDS} pre-weight lands, {POST_WEIGHT_LANDS} post-weight lands, {BARCODE_LAND_COUNT} barcode lands, release/hold/reject lanes"
    );
    println!(
        "  Clearances:            {:.0}mm front robot approach, {:.0}mm rear service sweep, {:.0}mm vertical pick envelope",
        front_robot_approach_clearance(),
        rear_service_sweep_clearance(),
        VERTICAL_PICK_CLEARANCE_Z
    );
    println!("  Required features:     {}", REQUIRED_FEATURES.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn on_base_z(part_z: f64) -> f64 {
    BASE_Z / 2.0 + part_z / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn slot_center(slot: usize) -> (f64, f64) {
    let col = slot % SLOT_COLS;
    let row = slot / SLOT_COLS;
    (
        centered_index(col, SLOT_COLS, SLOT_PITCH_X),
        centered_index(row, SLOT_ROWS, SLOT_PITCH_Y),
    )
}

fn slot_class(slot: usize) -> SlotClass {
    let col = slot % SLOT_COLS;
    let row = slot / SLOT_COLS;
    let on_left_or_right = col == 0 || col == SLOT_COLS - 1;
    let on_front_or_rear = row == 0 || row == SLOT_ROWS - 1;
    match (on_left_or_right, on_front_or_rear) {
        (true, true) => SlotClass::Corner,
        (true, false) | (false, true) => SlotClass::Edge,
        (false, false) => SlotClass::Center,
    }
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 12);
    assert_eq!(REQUIRED_FEATURES.len(), 13);
    assert_eq!(SLOT_COUNT, SLOT_COLS * SLOT_ROWS);
    assert_eq!(CORNER_SLOT_COUNT, class_count(SlotClass::Corner));
    assert_eq!(EDGE_SLOT_COUNT, class_count(SlotClass::Edge));
    assert_eq!(CENTER_SLOT_COUNT, class_count(SlotClass::Center));
    assert_eq!(
        MICRO_RESERVOIR_COUNT,
        SLOT_COUNT * MICRO_RESERVOIRS_PER_SLOT
    );
    assert_eq!(BLANK_REFERENCE_COUPON_COUNT, SLOT_COUNT);
    assert_eq!(PRE_WEIGHT_LANDS, POST_WEIGHT_LANDS);
    assert_eq!(DECISION_LANE_COUNT, 3);
    assert_eq!(DATUM_TARGET_COUNT, 4);
    assert!(SLOT_ARTICLE_Z > REVC_TOTAL_HEIGHT);
    assert!(SLOT_PITCH_X - SLOT_ARTICLE_X >= SLOT_AIR_GAP_MIN);
    assert!(SLOT_PITCH_Y - SLOT_ARTICLE_Y >= SLOT_AIR_GAP_MIN);
    assert!(humidity_ports_per_slot() == 1.0);
    assert!(microreservoirs_per_position_class() >= 12);
    assert!(condensate_witnesses_per_slot() == 1.0);
    assert!(front_robot_approach_clearance() >= ROBOT_APPROACH_CLEARANCE);
    assert!(rear_service_sweep_clearance() >= SERVICE_SWEEP_CLEARANCE);
    assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
    assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));

    for footprint in module_footprints() {
        assert!(
            fits_inside_rim(footprint),
            "{} exceeds base tray rim",
            footprint.name
        );
    }
}

fn class_count(class: SlotClass) -> usize {
    (0..SLOT_COUNT)
        .filter(|slot| slot_class(*slot) == class)
        .count()
}

fn humidity_ports_per_slot() -> f64 {
    HUMIDITY_PORT_COUNT as f64 / SLOT_COUNT as f64
}

fn microreservoirs_per_position_class() -> usize {
    MICRO_RESERVOIR_COUNT / POSITION_CLASS_COUNT
}

fn condensate_witnesses_per_slot() -> f64 {
    CONDENSATE_WITNESS_COUNT as f64 / SLOT_COUNT as f64
}

fn front_robot_approach_clearance() -> f64 {
    STATION_Y / 2.0 - (LANE_POS.1.abs() + LANE_BANK_Y / 2.0)
}

fn rear_service_sweep_clearance() -> f64 {
    STATION_Y / 2.0 - (EVIDENCE_POS.1 + EVIDENCE_BRIDGE_Y / 2.0)
}

fn fits_inside_rim(footprint: Footprint) -> bool {
    footprint.center.0.abs() + footprint.x / 2.0 <= STATION_X / 2.0 - RIM_W - 8.0
        && footprint.center.1.abs() + footprint.y / 2.0 <= STATION_Y / 2.0 - RIM_W - 8.0
}

fn module_footprints() -> [Footprint; 8] {
    [
        Footprint {
            name: "sixteen-slot balance rack",
            center: RACK_POS,
            x: RACK_X,
            y: RACK_Y,
        },
        Footprint {
            name: "micro-reservoir nests",
            center: MICRO_NEST_POS,
            x: MICRO_NEST_PANEL_X,
            y: MICRO_NEST_PANEL_Y,
        },
        Footprint {
            name: "RH/dewpoint logger grid",
            center: LOGGER_POS,
            x: LOGGER_PANEL_X,
            y: LOGGER_PANEL_Y,
        },
        Footprint {
            name: "blank reference coupons",
            center: COUPON_POS,
            x: COUPON_PANEL_X,
            y: COUPON_PANEL_Y,
        },
        Footprint {
            name: "humidity equalization manifold",
            center: MANIFOLD_POS,
            x: MANIFOLD_X,
            y: MANIFOLD_Y,
        },
        Footprint {
            name: "weighback custody lands",
            center: CUSTODY_POS,
            x: CUSTODY_PANEL_X,
            y: CUSTODY_PANEL_Y,
        },
        Footprint {
            name: "release/hold/reject lanes",
            center: LANE_POS,
            x: LANE_BANK_X,
            y: LANE_BANK_Y,
        },
        Footprint {
            name: "evidence bridge",
            center: EVIDENCE_POS,
            x: EVIDENCE_BRIDGE_X,
            y: EVIDENCE_BRIDGE_Y,
        },
    ]
}

fn base_tray() -> Part {
    let deck = centered_cube(
        "slot_evaporation_balance_base_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let spill_basin = centered_cube(
        "slot_evaporation_balance_secondary_spill_basin_cut",
        STATION_X - 132.0,
        STATION_Y - 124.0,
        7.0,
    )
    .translate(0.0, -12.0, BASE_Z / 2.0 - 3.2);
    let front_drain = centered_cylinder(
        "slot_evaporation_balance_front_condensate_drain_bore",
        12.0 / 2.0,
        54.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 92.0, -STATION_Y / 2.0 - 2.0, 0.0);

    deck - spill_basin - front_drain - base_insert_sockets() - mounting_slots()
        + tray_rims()
        + base_zone_dividers()
        + datum_targets()
        + evaporative_gradient_witness_ribs()
}

fn base_insert_sockets() -> Part {
    let mut sockets = Part::empty("slot_evaporation_balance_insert_sockets");
    for footprint in module_footprints() {
        sockets = sockets
            + centered_cube(
                format!("slot_evaporation_balance_{}_locator_socket", footprint.name),
                footprint.x + 6.0,
                footprint.y + 6.0,
                SOCKET_DEPTH + 0.5,
            )
            .translate(
                footprint.center.0,
                footprint.center.1,
                BASE_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.25,
            );
    }
    sockets
}

fn mounting_slots() -> Part {
    let mut slots = Part::empty("slot_evaporation_balance_mounting_slots");
    for (i, (x, y)) in mount_slot_positions().into_iter().enumerate() {
        let hole = centered_cylinder(
            format!("slot_evaporation_balance_m6_clearance_{i}"),
            6.8 / 2.0,
            BASE_Z + 4.0,
            24,
        )
        .translate(x, y, 0.0);
        let slot = centered_cube(
            format!("slot_evaporation_balance_m6_service_slot_{i}"),
            30.0,
            7.2,
            BASE_Z + 4.0,
        )
        .translate(x, y, 0.0);
        slots = slots + hole + slot;
    }
    slots
}

fn mount_slot_positions() -> [(f64, f64); MOUNT_SLOT_COUNT] {
    [
        (-(STATION_X / 2.0 - 62.0), -(STATION_Y / 2.0 - 60.0)),
        (STATION_X / 2.0 - 62.0, -(STATION_Y / 2.0 - 60.0)),
        (-(STATION_X / 2.0 - 62.0), STATION_Y / 2.0 - 60.0),
        (STATION_X / 2.0 - 62.0, STATION_Y / 2.0 - 60.0),
        (0.0, -(STATION_Y / 2.0 - 60.0)),
        (0.0, STATION_Y / 2.0 - 60.0),
        (-(STATION_X / 2.0 - 62.0), -12.0),
        (STATION_X / 2.0 - 62.0, -12.0),
    ]
}

fn tray_rims() -> Part {
    let left = centered_cube(
        "slot_evaporation_balance_left_high_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(
        -STATION_X / 2.0 + RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(
        "slot_evaporation_balance_right_high_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "slot_evaporation_balance_rear_service_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let front_low_lip = centered_cube(
        "slot_evaporation_balance_front_robot_low_lip",
        STATION_X - 190.0,
        12.0,
        20.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 24.0, BASE_Z / 2.0 + 10.0);

    left + right + rear + front_low_lip
}

fn base_zone_dividers() -> Part {
    let rack_measurement = centered_cube(
        "slot_evaporation_balance_rack_measurement_row_divider",
        STATION_X - 172.0,
        10.0,
        24.0,
    )
    .translate(0.0, 270.0, BASE_Z / 2.0 + 12.0);
    let reservoir_trace = centered_cube(
        "slot_evaporation_balance_reservoir_trace_row_divider",
        STATION_X - 172.0,
        10.0,
        24.0,
    )
    .translate(0.0, -88.0, BASE_Z / 2.0 + 12.0);
    let front_decision = centered_cube(
        "slot_evaporation_balance_decision_row_divider",
        STATION_X - 190.0,
        8.0,
        20.0,
    )
    .translate(0.0, -310.0, BASE_Z / 2.0 + 10.0);
    let right_measurement_column = centered_cube(
        "slot_evaporation_balance_right_measurement_column_divider",
        10.0,
        530.0,
        24.0,
    )
    .translate(146.0, 168.0, BASE_Z / 2.0 + 12.0);

    rack_measurement + reservoir_trace + front_decision + right_measurement_column
}

fn datum_targets() -> Part {
    let mut targets = Part::empty("slot_evaporation_balance_robot_datum_targets");
    for (i, (x, y)) in datum_target_points().into_iter().enumerate() {
        targets = targets
            + fiducial_disc(&format!("slot_evaporation_balance_robot_fiducial_{i}")).translate(
                x,
                y,
                BASE_Z / 2.0 + 2.5,
            );
    }
    targets
}

fn datum_target_points() -> [(f64, f64); DATUM_TARGET_COUNT] {
    [
        (-520.0, 372.0),
        (520.0, 372.0),
        (-520.0, -384.0),
        (520.0, -384.0),
    ]
}

fn evaporative_gradient_witness_ribs() -> Part {
    let mut ribs = Part::empty("slot_evaporation_balance_evaporative_gradient_witness_ribs");
    for (i, y) in [
        -258.0, -198.0, -138.0, -78.0, -18.0, 42.0, 102.0, 162.0, 222.0,
    ]
    .into_iter()
    .enumerate()
    {
        ribs = ribs
            + centered_cube(
                format!("slot_evaporation_balance_gradient_witness_rib_{i}"),
                STATION_X - 220.0,
                3.4,
                5.0,
            )
            .translate(0.0, y, BASE_Z / 2.0 + 2.5);
    }
    ribs
}

fn sixteen_slot_balance_rack() -> Part {
    let deck = centered_cube(
        "slot_evaporation_balance_sixteen_slot_rack_deck",
        RACK_X,
        RACK_Y,
        RACK_Z,
    );

    deck - slot_pocket_reliefs() - slot_air_bypass_windows() - robot_finger_lift_cutouts()
        + slot_reference_rails()
        + slot_surrogate_pads()
        + slot_position_class_tokens()
        + rack_gripper_lands()
}

fn slot_pocket_reliefs() -> Part {
    let mut reliefs = Part::empty("slot_evaporation_balance_slot_pocket_reliefs");
    for slot in 0..SLOT_COUNT {
        let (x, y) = slot_center(slot);
        reliefs = reliefs
            + centered_cube(
                format!("slot_evaporation_balance_slot_{slot:02}_media_surrogate_recess"),
                SLOT_ARTICLE_X + 8.0,
                SLOT_ARTICLE_Y + 8.0,
                SLOT_RECESS_DEPTH,
            )
            .translate(x, y, RACK_Z / 2.0 - SLOT_RECESS_DEPTH / 2.0 + 0.4);
    }
    reliefs
}

fn slot_air_bypass_windows() -> Part {
    let mut windows = Part::empty("slot_evaporation_balance_slot_air_bypass_windows");
    for i in 0..SLOT_AIR_BYPASS_COUNT {
        let col = i % 6;
        let row = i / 6;
        windows = windows
            + centered_cube(
                format!("slot_evaporation_balance_air_bypass_window_{i:02}"),
                58.0,
                8.0,
                RACK_Z + 3.0,
            )
            .translate(
                centered_index(col, 6, 96.0),
                centered_index(row, 4, 112.0),
                0.0,
            );
    }
    windows
}

fn robot_finger_lift_cutouts() -> Part {
    let mut cutouts = Part::empty("slot_evaporation_balance_robot_finger_lift_cutouts");
    for slot in 0..SLOT_COUNT {
        let (x, y) = slot_center(slot);
        cutouts = cutouts
            + centered_cube(
                format!("slot_evaporation_balance_slot_{slot:02}_front_finger_cutout"),
                64.0,
                18.0,
                RACK_Z + 4.0,
            )
            .translate(x, y - SLOT_ARTICLE_Y / 2.0 - 13.0, 0.0);
    }
    cutouts
}

fn slot_reference_rails() -> Part {
    let left = centered_cube(
        "slot_evaporation_balance_left_datum_rail",
        16.0,
        RACK_Y,
        SLOT_REFERENCE_RAIL_Z,
    )
    .translate(
        -RACK_X / 2.0 + 20.0,
        0.0,
        RACK_Z / 2.0 + SLOT_REFERENCE_RAIL_Z / 2.0,
    );
    let rear = centered_cube(
        "slot_evaporation_balance_rear_datum_rail",
        RACK_X,
        16.0,
        SLOT_REFERENCE_RAIL_Z,
    )
    .translate(
        0.0,
        RACK_Y / 2.0 - 20.0,
        RACK_Z / 2.0 + SLOT_REFERENCE_RAIL_Z / 2.0,
    );
    let row_separator = centered_cube(
        "slot_evaporation_balance_center_row_air_gap_gauge",
        RACK_X - 88.0,
        6.0,
        14.0,
    )
    .translate(0.0, 0.0, RACK_Z / 2.0 + 7.0);
    let col_separator = centered_cube(
        "slot_evaporation_balance_center_column_air_gap_gauge",
        6.0,
        RACK_Y - 88.0,
        14.0,
    )
    .translate(0.0, 0.0, RACK_Z / 2.0 + 7.0);

    left + rear + row_separator + col_separator
}

fn slot_surrogate_pads() -> Part {
    let mut pads = Part::empty("slot_evaporation_balance_media_surrogate_pads");
    for slot in 0..SLOT_COUNT {
        let (x, y) = slot_center(slot);
        let class = slot_class(slot);
        let pad = centered_cube(
            format!(
                "slot_evaporation_balance_slot_{slot:02}_{}_media_surrogate_pad",
                class.label()
            ),
            REVC_CHIP_LENGTH,
            REVC_CHIP_WIDTH,
            7.0,
        )
        .translate(x, y, RACK_Z / 2.0 + 3.5);
        let vapor_gap = centered_cube(
            format!("slot_evaporation_balance_slot_{slot:02}_matched_vapor_gap_ruler"),
            REVC_CHIP_LENGTH - 22.0,
            5.0,
            12.0,
        )
        .translate(x, y + REVC_CHIP_WIDTH / 2.0 + 13.0, RACK_Z / 2.0 + 6.0);
        pads = pads + pad + vapor_gap;
    }
    pads
}

fn slot_position_class_tokens() -> Part {
    let mut tokens = Part::empty("slot_evaporation_balance_slot_position_class_tokens");
    for slot in 0..SLOT_COUNT {
        let (x, y) = slot_center(slot);
        let class = slot_class(slot);
        tokens = tokens
            + centered_cylinder(
                format!(
                    "slot_evaporation_balance_slot_{slot:02}_{}_class_token",
                    class.label()
                ),
                class.token_diameter() / 2.0,
                4.0,
                32,
            )
            .translate(
                x + SLOT_ARTICLE_X / 2.0 - 20.0,
                y - SLOT_ARTICLE_Y / 2.0 + 18.0,
                RACK_Z / 2.0 + 2.0,
            );
    }
    tokens
}

fn rack_gripper_lands() -> Part {
    let front = centered_cube(
        "slot_evaporation_balance_rack_front_gripper_land",
        176.0,
        18.0,
        12.0,
    )
    .translate(0.0, -RACK_Y / 2.0 + 30.0, RACK_Z / 2.0 + 6.0);
    let rear = centered_cube(
        "slot_evaporation_balance_rack_rear_gripper_land",
        176.0,
        18.0,
        12.0,
    )
    .translate(0.0, RACK_Y / 2.0 - 46.0, RACK_Z / 2.0 + 6.0);
    front + rear
}

fn media_loss_microreservoir_nests() -> Part {
    let panel = centered_cube(
        "slot_evaporation_balance_media_loss_microreservoir_panel",
        MICRO_NEST_PANEL_X,
        MICRO_NEST_PANEL_Y,
        MICRO_NEST_PANEL_Z,
    );

    panel - microreservoir_reliefs() + microreservoir_rims() + mass_reference_tabs()
}

fn microreservoir_reliefs() -> Part {
    let mut reliefs = Part::empty("slot_evaporation_balance_microreservoir_reliefs");
    for slot in 0..SLOT_COUNT {
        let (x, y) = compact_slot_center(slot, MICRO_NEST_PITCH_X, MICRO_NEST_PITCH_Y);
        for r in 0..MICRO_RESERVOIRS_PER_SLOT {
            reliefs = reliefs
                + centered_cylinder(
                    format!("slot_evaporation_balance_slot_{slot:02}_reservoir_{r}_recess"),
                    MICRO_RESERVOIR_D / 2.0,
                    MICRO_RESERVOIR_DEPTH,
                    32,
                )
                .translate(
                    x + centered_index(r, MICRO_RESERVOIRS_PER_SLOT, 19.0),
                    y,
                    MICRO_NEST_PANEL_Z / 2.0 - MICRO_RESERVOIR_DEPTH / 2.0 + 0.5,
                );
        }
    }
    reliefs
}

fn microreservoir_rims() -> Part {
    let mut rims = Part::empty("slot_evaporation_balance_microreservoir_rims");
    for slot in 0..SLOT_COUNT {
        let (x, y) = compact_slot_center(slot, MICRO_NEST_PITCH_X, MICRO_NEST_PITCH_Y);
        let class = slot_class(slot);
        let rim = centered_cube(
            format!(
                "slot_evaporation_balance_slot_{slot:02}_{}_reservoir_weigh_boat_land",
                class.label()
            ),
            68.0,
            28.0,
            5.0,
        )
        .translate(x, y, MICRO_NEST_PANEL_Z / 2.0 + 2.5);
        let class_token = centered_cylinder(
            format!(
                "slot_evaporation_balance_slot_{slot:02}_{}_reservoir_class_token",
                class.label()
            ),
            class.token_diameter() / 2.5,
            5.0,
            24,
        )
        .translate(x + 44.0, y, MICRO_NEST_PANEL_Z / 2.0 + 2.5);
        rims = rims + rim + class_token;
    }
    rims
}

fn mass_reference_tabs() -> Part {
    let pre = centered_cube(
        "slot_evaporation_balance_pre_run_scale_tare_reference_tab",
        120.0,
        24.0,
        8.0,
    )
    .translate(
        -110.0,
        -MICRO_NEST_PANEL_Y / 2.0 + 24.0,
        MICRO_NEST_PANEL_Z / 2.0 + 4.0,
    );
    let post = centered_cube(
        "slot_evaporation_balance_post_run_scale_tare_reference_tab",
        120.0,
        24.0,
        8.0,
    )
    .translate(
        110.0,
        -MICRO_NEST_PANEL_Y / 2.0 + 24.0,
        MICRO_NEST_PANEL_Z / 2.0 + 4.0,
    );
    let water_blank = centered_cube(
        "slot_evaporation_balance_evaporation_blank_mass_reference_tab",
        96.0,
        20.0,
        8.0,
    )
    .translate(
        0.0,
        MICRO_NEST_PANEL_Y / 2.0 - 22.0,
        MICRO_NEST_PANEL_Z / 2.0 + 4.0,
    );

    pre + post + water_blank
}

fn compact_slot_center(slot: usize, pitch_x: f64, pitch_y: f64) -> (f64, f64) {
    let col = slot % SLOT_COLS;
    let row = slot / SLOT_COLS;
    (
        centered_index(col, SLOT_COLS, pitch_x),
        centered_index(row, SLOT_ROWS, pitch_y),
    )
}

fn rh_dewpoint_logger_grid() -> Part {
    let panel = centered_cube(
        "slot_evaporation_balance_rh_dewpoint_logger_grid_panel",
        LOGGER_PANEL_X,
        LOGGER_PANEL_Y,
        LOGGER_PANEL_Z,
    );
    panel - logger_pocket_reliefs() + logger_diffusion_slots() + dewpoint_reference_pedestals()
}

fn logger_pocket_reliefs() -> Part {
    let mut pockets = Part::empty("slot_evaporation_balance_logger_pocket_reliefs");
    for slot in 0..SLOT_COUNT {
        let (x, y) = compact_slot_center(slot, 82.0, 32.0);
        pockets = pockets
            + centered_cube(
                format!("slot_evaporation_balance_slot_{slot:02}_rh_logger_recess"),
                LOGGER_POCKET_X,
                LOGGER_POCKET_Y,
                LOGGER_POCKET_DEPTH,
            )
            .translate(x, y, LOGGER_PANEL_Z / 2.0 - LOGGER_POCKET_DEPTH / 2.0 + 0.5);
    }
    pockets
}

fn logger_diffusion_slots() -> Part {
    let mut slots = Part::empty("slot_evaporation_balance_logger_diffusion_slots");
    for i in 0..LOGGER_DIFFUSION_SLOT_COUNT {
        slots = slots
            + centered_cube(
                format!("slot_evaporation_balance_logger_diffusion_slot_{i:02}"),
                4.0,
                LOGGER_PANEL_Y - 34.0,
                5.0,
            )
            .translate(
                centered_index(i, LOGGER_DIFFUSION_SLOT_COUNT, 26.0),
                0.0,
                LOGGER_PANEL_Z / 2.0 + 2.5,
            );
    }
    slots
}

fn dewpoint_reference_pedestals() -> Part {
    let mut pedestals = Part::empty("slot_evaporation_balance_dewpoint_reference_pedestals");
    for i in 0..DEWPOINT_REFERENCE_COUNT {
        let x = centered_index(i, DEWPOINT_REFERENCE_COUNT, 74.0);
        pedestals = pedestals
            + centered_cylinder(
                format!("slot_evaporation_balance_dewpoint_reference_puck_{i}"),
                22.0 / 2.0,
                9.0,
                32,
            )
            .translate(x, LOGGER_PANEL_Y / 2.0 - 22.0, LOGGER_PANEL_Z / 2.0 + 4.5);
    }
    pedestals
}

fn edge_center_blank_reference_coupons() -> Part {
    let panel = centered_cube(
        "slot_evaporation_balance_edge_center_blank_reference_coupon_panel",
        COUPON_PANEL_X,
        COUPON_PANEL_Y,
        COUPON_PANEL_Z,
    );

    panel - blank_coupon_reliefs() + blank_coupon_lands() + dry_reference_lands()
}

fn blank_coupon_reliefs() -> Part {
    let mut reliefs = Part::empty("slot_evaporation_balance_blank_coupon_reliefs");
    for slot in 0..SLOT_COUNT {
        let (x, y) = compact_slot_center(slot, 78.0, 32.0);
        reliefs = reliefs
            + centered_cube(
                format!("slot_evaporation_balance_slot_{slot:02}_blank_media_coupon_recess"),
                COUPON_X,
                COUPON_Y,
                COUPON_RECESS_DEPTH,
            )
            .translate(x, y, COUPON_PANEL_Z / 2.0 - COUPON_RECESS_DEPTH / 2.0 + 0.4);
    }
    reliefs
}

fn blank_coupon_lands() -> Part {
    let mut lands = Part::empty("slot_evaporation_balance_blank_coupon_lands");
    for slot in 0..SLOT_COUNT {
        let (x, y) = compact_slot_center(slot, 78.0, 32.0);
        let class = slot_class(slot);
        lands = lands
            + centered_cylinder(
                format!(
                    "slot_evaporation_balance_slot_{slot:02}_{}_blank_reference_token",
                    class.label()
                ),
                class.token_diameter() / 2.8,
                5.0,
                24,
            )
            .translate(x + 28.0, y, COUPON_PANEL_Z / 2.0 + 2.5);
    }
    lands
}

fn dry_reference_lands() -> Part {
    let mut lands = Part::empty("slot_evaporation_balance_dry_reference_lands");
    for i in 0..DRY_REFERENCE_COUPON_COUNT {
        lands = lands
            + centered_cube(
                format!("slot_evaporation_balance_dry_reference_coupon_land_{i}"),
                42.0,
                18.0,
                7.0,
            )
            .translate(
                centered_index(i, DRY_REFERENCE_COUPON_COUNT, 54.0),
                -COUPON_PANEL_Y / 2.0 + 18.0,
                COUPON_PANEL_Z / 2.0 + 3.5,
            );
    }
    lands
}

fn condensate_shadow_drip_shields() -> Part {
    let clear_panel = centered_cube(
        "slot_evaporation_balance_condensate_shadow_clear_shield",
        SHIELD_PANEL_X,
        SHIELD_PANEL_Y,
        SHIELD_PANEL_Z,
    );
    let drip_gutters = shield_drip_gutters();
    let standoffs = shield_standoffs();
    let witnesses = condensate_witness_lands();

    clear_panel + drip_gutters + standoffs + witnesses
}

fn shield_drip_gutters() -> Part {
    let front = centered_cube(
        "slot_evaporation_balance_shield_front_condensate_gutter",
        SHIELD_PANEL_X,
        CONDENSATE_GUTTER_W,
        CONDENSATE_GUTTER_DEPTH,
    )
    .translate(
        0.0,
        -SHIELD_PANEL_Y / 2.0 + 16.0,
        -SHIELD_PANEL_Z / 2.0 - 4.0,
    );
    let rear = centered_cube(
        "slot_evaporation_balance_shield_rear_condensate_gutter",
        SHIELD_PANEL_X,
        CONDENSATE_GUTTER_W,
        CONDENSATE_GUTTER_DEPTH,
    )
    .translate(
        0.0,
        SHIELD_PANEL_Y / 2.0 - 16.0,
        -SHIELD_PANEL_Z / 2.0 - 4.0,
    );
    let left = centered_cube(
        "slot_evaporation_balance_shield_left_condensate_gutter",
        CONDENSATE_GUTTER_W,
        SHIELD_PANEL_Y,
        CONDENSATE_GUTTER_DEPTH,
    )
    .translate(
        -SHIELD_PANEL_X / 2.0 + 16.0,
        0.0,
        -SHIELD_PANEL_Z / 2.0 - 4.0,
    );
    let right = centered_cube(
        "slot_evaporation_balance_shield_right_condensate_gutter",
        CONDENSATE_GUTTER_W,
        SHIELD_PANEL_Y,
        CONDENSATE_GUTTER_DEPTH,
    )
    .translate(
        SHIELD_PANEL_X / 2.0 - 16.0,
        0.0,
        -SHIELD_PANEL_Z / 2.0 - 4.0,
    );

    front + rear + left + right
}

fn shield_standoffs() -> Part {
    let mut standoffs = Part::empty("slot_evaporation_balance_shield_standoffs");
    for (i, (x, y)) in [
        (-SHIELD_PANEL_X / 2.0 + 42.0, -SHIELD_PANEL_Y / 2.0 + 42.0),
        (SHIELD_PANEL_X / 2.0 - 42.0, -SHIELD_PANEL_Y / 2.0 + 42.0),
        (-SHIELD_PANEL_X / 2.0 + 42.0, SHIELD_PANEL_Y / 2.0 - 42.0),
        (SHIELD_PANEL_X / 2.0 - 42.0, SHIELD_PANEL_Y / 2.0 - 42.0),
    ]
    .into_iter()
    .enumerate()
    {
        standoffs = standoffs
            + centered_cylinder(
                format!("slot_evaporation_balance_shield_standoff_{i}"),
                7.0,
                SHIELD_STANDOFF_Z,
                32,
            )
            .translate(x, y, -SHIELD_STANDOFF_Z / 2.0);
    }
    standoffs
}

fn condensate_witness_lands() -> Part {
    let mut witnesses = Part::empty("slot_evaporation_balance_condensate_witness_lands");
    for slot in 0..SLOT_COUNT {
        let (x, y) = slot_center(slot);
        witnesses = witnesses
            + centered_cube(
                format!("slot_evaporation_balance_slot_{slot:02}_condensate_shadow_witness"),
                28.0,
                12.0,
                3.0,
            )
            .translate(
                x,
                y + SLOT_ARTICLE_Y / 2.0 - 8.0,
                SHIELD_PANEL_Z / 2.0 + 1.5,
            );
    }
    witnesses
}

fn humidity_equalization_manifold() -> Part {
    let body = centered_cube(
        "slot_evaporation_balance_humidity_equalization_manifold_body",
        MANIFOLD_X,
        MANIFOLD_Y,
        MANIFOLD_Z,
    );

    body - manifold_humidity_port_bores() - manifold_restrictor_windows()
        + manifold_inlet_outlet_bosses()
        + manifold_slot_labels()
}

fn manifold_humidity_port_bores() -> Part {
    let mut bores = Part::empty("slot_evaporation_balance_humidity_port_bores");
    for i in 0..HUMIDITY_PORT_COUNT {
        bores = bores
            + centered_cylinder(
                format!("slot_evaporation_balance_humidity_port_bore_{i:02}"),
                5.5,
                MANIFOLD_Z + 3.0,
                24,
            )
            .translate(centered_index(i, HUMIDITY_PORT_COUNT, 39.0), 0.0, 0.0);
    }
    bores
}

fn manifold_restrictor_windows() -> Part {
    let mut windows = Part::empty("slot_evaporation_balance_matched_restrictor_windows");
    for i in 0..MANIFOLD_RESTRICTOR_COUNT {
        windows = windows
            + centered_cube(
                format!("slot_evaporation_balance_matched_restrictor_coupon_window_{i:02}"),
                MANIFOLD_SLOT_W,
                MANIFOLD_SLOT_Y,
                8.0,
            )
            .translate(
                centered_index(i, MANIFOLD_RESTRICTOR_COUNT, 39.0),
                0.0,
                MANIFOLD_Z / 2.0 - 3.5,
            );
    }
    windows
}

fn manifold_inlet_outlet_bosses() -> Part {
    let inlet = centered_cylinder(
        "slot_evaporation_balance_manifold_humidified_air_inlet_boss",
        INLET_PORT_D / 2.0,
        22.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-MANIFOLD_X / 2.0 - 8.0, 0.0, MANIFOLD_Z / 2.0 + 2.0);
    let outlet = centered_cylinder(
        "slot_evaporation_balance_manifold_return_air_outlet_boss",
        OUTLET_PORT_D / 2.0,
        22.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(MANIFOLD_X / 2.0 + 8.0, 0.0, MANIFOLD_Z / 2.0 + 2.0);

    inlet + outlet
}

fn manifold_slot_labels() -> Part {
    let mut labels = Part::empty("slot_evaporation_balance_manifold_slot_labels");
    for i in 0..SLOT_COUNT {
        labels = labels
            + centered_cube(
                format!("slot_evaporation_balance_manifold_slot_{i:02}_label_land"),
                28.0,
                12.0,
                3.0,
            )
            .translate(
                centered_index(i, SLOT_COUNT, 39.0),
                MANIFOLD_Y / 2.0 - 16.0,
                MANIFOLD_Z / 2.0 + 1.5,
            );
    }
    labels
}

fn weighback_barcode_custody_lands() -> Part {
    let panel = centered_cube(
        "slot_evaporation_balance_weighback_barcode_custody_panel",
        CUSTODY_PANEL_X,
        CUSTODY_PANEL_Y,
        CUSTODY_PANEL_Z,
    );

    panel + pre_post_weighback_lands() + barcode_lands() + custody_seal_lands()
}

fn pre_post_weighback_lands() -> Part {
    let mut lands = Part::empty("slot_evaporation_balance_pre_post_weighback_lands");
    for slot in 0..SLOT_COUNT {
        let x = centered_index(slot % 8, 8, 48.0);
        let y = if slot < 8 { 22.0 } else { -22.0 };
        let pre = centered_cube(
            format!("slot_evaporation_balance_slot_{slot:02}_pre_weight_land"),
            WEIGH_LAND_X,
            WEIGH_LAND_Y,
            4.0,
        )
        .translate(x, y + 14.0, CUSTODY_PANEL_Z / 2.0 + 2.0);
        let post = centered_cube(
            format!("slot_evaporation_balance_slot_{slot:02}_post_weight_land"),
            WEIGH_LAND_X,
            WEIGH_LAND_Y,
            4.0,
        )
        .translate(x, y - 14.0, CUSTODY_PANEL_Z / 2.0 + 2.0);
        lands = lands + pre + post;
    }
    lands
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("slot_evaporation_balance_barcode_lands");
    for i in 0..BARCODE_LAND_COUNT {
        lands = lands
            + centered_cube(
                format!("slot_evaporation_balance_barcode_land_{i:02}"),
                BARCODE_LAND_X,
                BARCODE_LAND_Y,
                3.0,
            )
            .translate(
                centered_index(i % 10, 10, 41.0),
                CUSTODY_PANEL_Y / 2.0 - 14.0 - (i / 10) as f64 * 18.0,
                CUSTODY_PANEL_Z / 2.0 + 1.5,
            );
    }
    lands
}

fn custody_seal_lands() -> Part {
    let mut seals = Part::empty("slot_evaporation_balance_custody_seal_lands");
    for i in 0..POSITION_CLASS_COUNT {
        seals = seals
            + centered_cylinder(
                format!("slot_evaporation_balance_position_class_{i}_custody_seal"),
                9.0,
                5.0,
                24,
            )
            .translate(
                CUSTODY_PANEL_X / 2.0 - 34.0,
                centered_index(i, POSITION_CLASS_COUNT, 28.0),
                CUSTODY_PANEL_Z / 2.0 + 2.5,
            );
    }
    seals
}

fn release_hold_reject_lanes() -> Part {
    let base = centered_cube(
        "slot_evaporation_balance_release_hold_reject_lane_bank",
        LANE_BANK_X,
        LANE_BANK_Y,
        LANE_BANK_Z,
    );
    let lane_dividers = lane_dividers();
    let tokens = lane_token_wells();

    base - tokens + lane_dividers + lane_labels()
}

fn lane_dividers() -> Part {
    let first = centered_cube(
        "slot_evaporation_balance_release_hold_lane_divider",
        LANE_RIB_W,
        LANE_BANK_Y,
        24.0,
    )
    .translate(-LANE_BANK_X / 6.0, 0.0, LANE_BANK_Z / 2.0 + 12.0);
    let second = centered_cube(
        "slot_evaporation_balance_hold_reject_lane_divider",
        LANE_RIB_W,
        LANE_BANK_Y,
        24.0,
    )
    .translate(LANE_BANK_X / 6.0, 0.0, LANE_BANK_Z / 2.0 + 12.0);

    first + second
}

fn lane_token_wells() -> Part {
    let mut wells = Part::empty("slot_evaporation_balance_lane_token_wells");
    for i in 0..LANE_TOKEN_COUNT {
        let lane = i % DECISION_LANE_COUNT;
        let row = i / DECISION_LANE_COUNT;
        wells = wells
            + centered_cylinder(
                format!("slot_evaporation_balance_lane_token_well_{i:02}"),
                10.0,
                12.0,
                24,
            )
            .translate(
                centered_index(lane, DECISION_LANE_COUNT, LANE_BANK_X / 3.0),
                centered_index(row, 6, 16.0),
                LANE_BANK_Z / 2.0 - 5.0,
            );
    }
    wells
}

fn lane_labels() -> Part {
    let release = centered_cube(
        "slot_evaporation_balance_release_lane_label_land",
        82.0,
        12.0,
        3.0,
    )
    .translate(
        -LANE_BANK_X / 3.0,
        LANE_BANK_Y / 2.0 - 14.0,
        LANE_BANK_Z / 2.0 + 1.5,
    );
    let hold = centered_cube(
        "slot_evaporation_balance_hold_lane_label_land",
        82.0,
        12.0,
        3.0,
    )
    .translate(0.0, LANE_BANK_Y / 2.0 - 14.0, LANE_BANK_Z / 2.0 + 1.5);
    let reject = centered_cube(
        "slot_evaporation_balance_reject_lane_label_land",
        82.0,
        12.0,
        3.0,
    )
    .translate(
        LANE_BANK_X / 3.0,
        LANE_BANK_Y / 2.0 - 14.0,
        LANE_BANK_Z / 2.0 + 1.5,
    );

    release + hold + reject
}

fn evidence_bridge() -> Part {
    let beam = centered_cube(
        "slot_evaporation_balance_evidence_bridge_beam",
        EVIDENCE_BRIDGE_X,
        EVIDENCE_BRIDGE_Y,
        26.0,
    )
    .translate(0.0, 0.0, EVIDENCE_BRIDGE_Z / 2.0 - 13.0);
    let left_post = centered_cube(
        "slot_evaporation_balance_evidence_bridge_left_post",
        24.0,
        EVIDENCE_BRIDGE_Y,
        EVIDENCE_BRIDGE_Z,
    )
    .translate(-EVIDENCE_BRIDGE_X / 2.0 + 38.0, 0.0, 0.0);
    let right_post = centered_cube(
        "slot_evaporation_balance_evidence_bridge_right_post",
        24.0,
        EVIDENCE_BRIDGE_Y,
        EVIDENCE_BRIDGE_Z,
    )
    .translate(EVIDENCE_BRIDGE_X / 2.0 - 38.0, 0.0, 0.0);
    let cameras = evidence_camera_lands();
    let pipes = evidence_light_pipes();
    let windows = scale_witness_windows();

    beam + left_post + right_post + cameras + pipes + windows
}

fn evidence_camera_lands() -> Part {
    let mut cameras = Part::empty("slot_evaporation_balance_evidence_camera_lands");
    for i in 0..EVIDENCE_CAMERA_COUNT {
        cameras = cameras
            + centered_cube(
                format!("slot_evaporation_balance_evidence_camera_land_{i}"),
                50.0,
                30.0,
                6.0,
            )
            .translate(
                centered_index(i, EVIDENCE_CAMERA_COUNT, 180.0),
                0.0,
                EVIDENCE_BRIDGE_Z / 2.0 + 3.0,
            );
    }
    cameras
}

fn evidence_light_pipes() -> Part {
    let mut pipes = Part::empty("slot_evaporation_balance_evidence_light_pipes");
    for i in 0..LIGHT_PIPE_COUNT {
        pipes = pipes
            + centered_cylinder(
                format!("slot_evaporation_balance_evidence_light_pipe_{i}"),
                6.0,
                24.0,
                24,
            )
            .translate(
                centered_index(i, LIGHT_PIPE_COUNT, 102.0),
                -EVIDENCE_BRIDGE_Y / 2.0 + 12.0,
                EVIDENCE_BRIDGE_Z / 2.0 + 12.0,
            );
    }
    pipes
}

fn scale_witness_windows() -> Part {
    let mut windows = Part::empty("slot_evaporation_balance_scale_witness_windows");
    for i in 0..SCALE_WITNESS_WINDOW_COUNT {
        windows = windows
            + centered_cube(
                format!("slot_evaporation_balance_scale_witness_window_{i}"),
                92.0,
                18.0,
                5.0,
            )
            .translate(
                centered_index(i, SCALE_WITNESS_WINDOW_COUNT, 220.0),
                EVIDENCE_BRIDGE_Y / 2.0 - 12.0,
                EVIDENCE_BRIDGE_Z / 2.0 + 2.5,
            );
    }
    windows
}

fn robot_service_keepouts() -> Part {
    let perimeter = centered_cube(
        "slot_evaporation_balance_perimeter_keepout_gauge",
        KEEP_OUT_X,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    );
    let robot_front = centered_cube(
        "slot_evaporation_balance_front_robot_sweep_keepout",
        STATION_X - 250.0,
        ROBOT_APPROACH_CLEARANCE,
        14.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 74.0, KEEP_OUT_Z / 2.0 + 7.0);
    let service_rear = centered_cube(
        "slot_evaporation_balance_rear_service_sweep_keepout",
        STATION_X - 250.0,
        SERVICE_SWEEP_CLEARANCE,
        14.0,
    )
    .translate(0.0, STATION_Y / 2.0 - 78.0, KEEP_OUT_Z / 2.0 + 7.0);
    let vertical_pick = centered_cube(
        "slot_evaporation_balance_vertical_pick_clearance_gauge",
        120.0,
        80.0,
        VERTICAL_PICK_CLEARANCE_Z,
    )
    .translate(
        RACK_POS.0,
        RACK_POS.1,
        KEEP_OUT_Z / 2.0 + VERTICAL_PICK_CLEARANCE_Z / 2.0,
    );

    perimeter + robot_front + service_rear + vertical_pick
}

fn fiducial_disc(name: &str) -> Part {
    centered_cylinder(name, 12.0, 5.0, 32)
        - centered_cylinder(format!("{name}_center_dot"), 3.0, 6.0, 24)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_manifest_exports_parts_plus_assembly() {
        assert_eq!(OUTPUTS.len(), 12);
        assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }

    #[test]
    fn slot_map_separates_corner_edge_and_center_positions() {
        assert_eq!(class_count(SlotClass::Corner), CORNER_SLOT_COUNT);
        assert_eq!(class_count(SlotClass::Edge), EDGE_SLOT_COUNT);
        assert_eq!(class_count(SlotClass::Center), CENTER_SLOT_COUNT);
        assert_eq!(
            class_count(SlotClass::Corner)
                + class_count(SlotClass::Edge)
                + class_count(SlotClass::Center),
            SLOT_COUNT
        );
    }

    #[test]
    fn every_slot_gets_equal_media_and_humidity_witnesses() {
        assert_eq!(
            MICRO_RESERVOIR_COUNT,
            SLOT_COUNT * MICRO_RESERVOIRS_PER_SLOT
        );
        assert_eq!(HUMIDITY_PORT_COUNT, SLOT_COUNT);
        assert_eq!(MANIFOLD_RESTRICTOR_COUNT, SLOT_COUNT);
        assert_eq!(CONDENSATE_WITNESS_COUNT, SLOT_COUNT);
        assert_eq!(SLOT_LOGGER_COUNT, SLOT_COUNT);
    }

    #[test]
    fn custody_tracks_pre_and_post_weighback_for_each_slot() {
        assert_eq!(PRE_WEIGHT_LANDS, SLOT_COUNT);
        assert_eq!(POST_WEIGHT_LANDS, SLOT_COUNT);
        assert!(BARCODE_LAND_COUNT > SLOT_COUNT);
        assert!(MASS_LOSS_RESOLUTION_MG <= 2.0);
    }

    #[test]
    fn module_footprints_remain_inside_the_closed_tray() {
        for footprint in module_footprints() {
            assert!(
                fits_inside_rim(footprint),
                "{} outside tray",
                footprint.name
            );
        }
    }

    #[test]
    fn rack_slots_keep_matched_air_gaps() {
        assert!(SLOT_PITCH_X - SLOT_ARTICLE_X >= SLOT_AIR_GAP_MIN);
        assert!(SLOT_PITCH_Y - SLOT_ARTICLE_Y >= SLOT_AIR_GAP_MIN);
        assert!(SLOT_ARTICLE_Z > REVC_TOTAL_HEIGHT);
    }
}
