use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed incubator fan-flow shadowing and rack blockage mapping station.
//
// This standalone generator models a no-cell validation fixture for mapping
// fan/plenum shadowing, rack blockage, and condensate witness behavior inside a
// closed incubator envelope. It provides a rack-slot surrogate grid, removable
// fan/plenum shadow masks, airflow ribbon and witness token lands, multi-height
// sensor mast pockets, cassette-equivalent load blockers, drip collection
// lanes, traceability lands, disposition lanes, evidence capture bridge, and
// robot/service keepout gauges. Airflow criteria, incubator recipes, probe
// calibration, and batch disposition decisions remain validation controls
// outside this CAD model.

const OUTPUT_PREFIX: &str = "closed_incubator_fan_flow_shadowing_mapping_station";

const OUTPUTS: [&str; 12] = [
    "output/closed_incubator_fan_flow_shadowing_mapping_station_base_containment_deck.stl",
    "output/closed_incubator_fan_flow_shadowing_mapping_station_rack_slot_surrogate_grid.stl",
    "output/closed_incubator_fan_flow_shadowing_mapping_station_fan_plenum_shadow_masks.stl",
    "output/closed_incubator_fan_flow_shadowing_mapping_station_airflow_ribbon_witness_token_lands.stl",
    "output/closed_incubator_fan_flow_shadowing_mapping_station_multi_height_sensor_mast_pockets.stl",
    "output/closed_incubator_fan_flow_shadowing_mapping_station_cassette_load_blockers.stl",
    "output/closed_incubator_fan_flow_shadowing_mapping_station_condensate_drip_collection_lanes.stl",
    "output/closed_incubator_fan_flow_shadowing_mapping_station_barcode_certificate_lands.stl",
    "output/closed_incubator_fan_flow_shadowing_mapping_station_release_hold_reject_lanes.stl",
    "output/closed_incubator_fan_flow_shadowing_mapping_station_evidence_bridge.stl",
    "output/closed_incubator_fan_flow_shadowing_mapping_station_robot_service_keepout_gauges.stl",
    "output/closed_incubator_fan_flow_shadowing_mapping_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "rack_slot_surrogate_grid",
    "fan_plenum_shadow_masks",
    "airflow_ribbon_witness_token_lands",
    "multi_height_sensor_mast_pockets",
    "cassette_load_blockers",
    "condensate_drip_collection_lanes",
    "barcode_certificate_lands",
    "release_hold_reject_lanes",
    "evidence_bridge",
    "robot_service_keepout_gauges",
    "named_stl_outputs",
];

const STATION_X: f64 = 1500.0;
const STATION_Y: f64 = 960.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const SOCKET_DEPTH: f64 = 6.0;
const MOUNT_SLOT_COUNT: usize = 8;
const DATUM_TARGET_COUNT: usize = 4;

const RACK_COLS: usize = 4;
const RACK_ROWS: usize = 3;
const RACK_SLOT_COUNT: usize = RACK_COLS * RACK_ROWS;
const EDGE_SLOT_COUNT: usize = 10;
const CENTER_SLOT_COUNT: usize = RACK_SLOT_COUNT - EDGE_SLOT_COUNT;
const RACK_GRID_X: f64 = 620.0;
const RACK_GRID_Y: f64 = 420.0;
const RACK_GRID_Z: f64 = 36.0;
const RACK_POS: (f64, f64) = (-360.0, 115.0);
const SLOT_PITCH_X: f64 = 128.0;
const SLOT_PITCH_Y: f64 = 122.0;
const SLOT_SURROGATE_X: f64 = REVC_CHIP_LENGTH + 42.0;
const SLOT_SURROGATE_Y: f64 = REVC_CHIP_WIDTH + 38.0;
const SLOT_RELIEF_DEPTH: f64 = 10.0;
const SLOT_SURROGATE_Z: f64 = REVC_TOTAL_HEIGHT + 14.0;
const RACK_AIR_BYPASS_SLOT_COUNT: usize = 10;

const FAN_PANEL_X: f64 = 420.0;
const FAN_PANEL_Y: f64 = 250.0;
const FAN_PANEL_Z: f64 = 34.0;
const FAN_POS: (f64, f64) = (360.0, 185.0);
const FAN_COUNT: usize = 3;
const FAN_D: f64 = 84.0;
const FAN_PITCH_X: f64 = 116.0;
const PLENUM_BAFFLE_COUNT: usize = 6;
const SHADOW_MASK_COUNT: usize = 5;
const SHADOW_MASK_CARD_X: f64 = 58.0;
const SHADOW_MASK_CARD_Y: f64 = 34.0;

const RIBBON_PANEL_X: f64 = 420.0;
const RIBBON_PANEL_Y: f64 = 210.0;
const RIBBON_PANEL_Z: f64 = 20.0;
const RIBBON_POS: (f64, f64) = (360.0, -90.0);
const AIRFLOW_RIBBON_COUNT: usize = 9;
const RIBBON_SPACING_Y: f64 = 20.0;
const WITNESS_TOKEN_COUNT: usize = AIRFLOW_RIBBON_COUNT * 2;
const WITNESS_TOKEN_D: f64 = 20.0;

const MAST_PANEL_X: f64 = 620.0;
const MAST_PANEL_Y: f64 = 180.0;
const MAST_PANEL_Z: f64 = 24.0;
const MAST_POS: (f64, f64) = (-360.0, -245.0);
const MAST_COLS: usize = 4;
const MAST_ROWS: usize = 2;
const MAST_POCKET_COUNT: usize = MAST_COLS * MAST_ROWS;
const HEIGHT_TIER_COUNT: usize = 3;
const MAST_PITCH_X: f64 = 132.0;
const MAST_PITCH_Y: f64 = 70.0;
const MAST_SOCKET_D: f64 = 15.0;
const MAST_BASE_RING_D: f64 = 29.0;
const LOW_TIER_Z: f64 = 44.0;
const MID_TIER_Z: f64 = 92.0;
const HIGH_TIER_Z: f64 = 146.0;

const BLOCKER_PANEL_X: f64 = 160.0;
const BLOCKER_PANEL_Y: f64 = 220.0;
const BLOCKER_PANEL_Z: f64 = 24.0;
const BLOCKER_POS: (f64, f64) = (50.0, -300.0);
const LOAD_BLOCKER_COUNT: usize = 6;
const BLOCKER_WIDTH: f64 = 48.0;
const BLOCKER_DEPTH: f64 = 32.0;
const BLOCKER_PITCH_Y: f64 = 32.0;

const CONDENSATE_X: f64 = 420.0;
const CONDENSATE_Y: f64 = 120.0;
const CONDENSATE_Z: f64 = 26.0;
const CONDENSATE_POS: (f64, f64) = (360.0, -340.0);
const DRIP_LANE_COUNT: usize = 6;
const COLLECTION_CUP_COUNT: usize = 3;
const DRIP_LANE_W: f64 = 18.0;
const DRIP_LANE_DEPTH: f64 = 9.0;
const COLLECTION_CUP_D: f64 = 46.0;

const TRACE_X: f64 = 210.0;
const TRACE_Y: f64 = 80.0;
const TRACE_Z: f64 = 12.0;
const TRACE_POS: (f64, f64) = (-610.0, -405.0);
const BARCODE_LAND_COUNT: usize = 8;
const CERTIFICATE_LAND_COUNT: usize = 3;

const LANE_BANK_X: f64 = 420.0;
const LANE_BANK_Y: f64 = 90.0;
const LANE_BANK_Z: f64 = 24.0;
const LANE_POS: (f64, f64) = (-245.0, -400.0);
const LANE_COUNT: usize = 3;
const LANE_SLOT_X: f64 = 110.0;
const LANE_SLOT_Y: f64 = 52.0;
const RELEASE_CAPACITY: usize = 6;
const HOLD_CAPACITY: usize = 4;
const REJECT_CAPACITY: usize = 2;

const EVIDENCE_BRIDGE_X: f64 = 1160.0;
const EVIDENCE_BRIDGE_Y: f64 = 70.0;
const EVIDENCE_ANCHOR_Z: f64 = 14.0;
const EVIDENCE_POST_Z: f64 = 190.0;
const EVIDENCE_CROSSBAR_Z: f64 = 22.0;
const EVIDENCE_POS: (f64, f64) = (0.0, 405.0);
const CAMERA_TARGET_COUNT: usize = 5;
const LIGHT_PIPE_COUNT: usize = 6;
const CAMERA_CLEARANCE_Z: f64 = 168.0;

const KEEP_OUT_X: f64 = 1400.0;
const KEEP_OUT_Y: f64 = 900.0;
const KEEP_OUT_Z: f64 = 8.0;
const KEEP_OUT_ZONE_COUNT: usize = 5;
const ROBOT_FRONT_CLEARANCE: f64 = 32.0;
const SERVICE_REAR_CLEARANCE: f64 = 36.0;
const ROBOT_Z_CLEARANCE: f64 = 260.0;

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
enum DispositionLane {
    Release,
    Hold,
    Reject,
}

impl DispositionLane {
    fn all() -> [DispositionLane; LANE_COUNT] {
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

    fn name(self) -> &'static str {
        match self {
            DispositionLane::Release => "release",
            DispositionLane::Hold => "hold",
            DispositionLane::Reject => "reject",
        }
    }

    fn capacity(self) -> usize {
        match self {
            DispositionLane::Release => RELEASE_CAPACITY,
            DispositionLane::Hold => HOLD_CAPACITY,
            DispositionLane::Reject => REJECT_CAPACITY,
        }
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = base_containment_deck();
    export(OUTPUTS[0], &deck);

    let rack = rack_slot_surrogate_grid();
    export(OUTPUTS[1], &rack);

    let masks = fan_plenum_shadow_masks();
    export(OUTPUTS[2], &masks);

    let ribbons = airflow_ribbon_witness_token_lands();
    export(OUTPUTS[3], &ribbons);

    let masts = multi_height_sensor_mast_pockets();
    export(OUTPUTS[4], &masts);

    let blockers = cassette_load_blockers();
    export(OUTPUTS[5], &blockers);

    let condensate = condensate_drip_collection_lanes();
    export(OUTPUTS[6], &condensate);

    let trace = barcode_certificate_lands();
    export(OUTPUTS[7], &trace);

    let lanes = release_hold_reject_lanes();
    export(OUTPUTS[8], &lanes);

    let bridge = evidence_bridge();
    export(OUTPUTS[9], &bridge);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[10], &keepouts);

    let assembly = deck
        + rack.translate(RACK_POS.0, RACK_POS.1, on_base_z(RACK_GRID_Z))
        + masks.translate(FAN_POS.0, FAN_POS.1, on_base_z(FAN_PANEL_Z))
        + ribbons.translate(RIBBON_POS.0, RIBBON_POS.1, on_base_z(RIBBON_PANEL_Z))
        + masts.translate(MAST_POS.0, MAST_POS.1, on_base_z(MAST_PANEL_Z))
        + blockers.translate(BLOCKER_POS.0, BLOCKER_POS.1, on_base_z(BLOCKER_PANEL_Z))
        + condensate.translate(CONDENSATE_POS.0, CONDENSATE_POS.1, on_base_z(CONDENSATE_Z))
        + trace.translate(TRACE_POS.0, TRACE_POS.1, on_base_z(TRACE_Z))
        + lanes.translate(LANE_POS.0, LANE_POS.1, on_base_z(LANE_BANK_Z))
        + bridge.translate(EVIDENCE_POS.0, EVIDENCE_POS.1, on_base_z(EVIDENCE_ANCHOR_Z))
        + keepouts.translate(0.0, 0.0, BASE_Z / 2.0 + KEEP_OUT_Z / 2.0);
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed incubator fan-flow shadowing mapping station:");
    println!(
        "  Rack map:              {RACK_COLS}x{RACK_ROWS} no-cell surrogate grid with {EDGE_SLOT_COUNT} edge and {CENTER_SLOT_COUNT} center slots"
    );
    println!(
        "  Fan/plenum:            {FAN_COUNT} fan apertures, {PLENUM_BAFFLE_COUNT} plenum baffles, {SHADOW_MASK_COUNT} removable shadow-mask cards"
    );
    println!(
        "  Flow witnesses:        {AIRFLOW_RIBBON_COUNT} airflow ribbon lands and {WITNESS_TOKEN_COUNT} witness token positions"
    );
    println!(
        "  Sensor geometry:       {MAST_POCKET_COUNT} mast pockets with {HEIGHT_TIER_COUNT} low/mid/high height tier gauges"
    );
    println!(
        "  Load challenge:        {LOAD_BLOCKER_COUNT} cassette blockers and {:.0}% nominal open rack area",
        rack_open_area_fraction() * 100.0
    );
    println!(
        "  Condensate/trace:      {DRIP_LANE_COUNT} drip lanes, {COLLECTION_CUP_COUNT} cup lands, {BARCODE_LAND_COUNT} barcode lands, {CERTIFICATE_LAND_COUNT} certificate lands"
    );
    println!(
        "  Disposition/evidence:  release/hold/reject capacity for {} slots, {CAMERA_TARGET_COUNT} camera targets, {LIGHT_PIPE_COUNT} light-pipe lands, {KEEP_OUT_ZONE_COUNT} keepout gauges",
        total_lane_capacity()
    );
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

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 12);
    assert_eq!(REQUIRED_FEATURES.len(), 11);
    assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
    assert_eq!(RACK_SLOT_COUNT, RACK_COLS * RACK_ROWS);
    assert_eq!(edge_slot_count(), EDGE_SLOT_COUNT);
    assert_eq!(center_slot_count(), CENTER_SLOT_COUNT);
    assert_eq!(MAST_POCKET_COUNT, MAST_COLS * MAST_ROWS);
    assert_eq!(WITNESS_TOKEN_COUNT, AIRFLOW_RIBBON_COUNT * 2);
    assert_eq!(DispositionLane::all().len(), LANE_COUNT);
    assert_eq!(total_lane_capacity(), RACK_SLOT_COUNT);
    assert!(SLOT_SURROGATE_X > REVC_CHIP_LENGTH);
    assert!(SLOT_SURROGATE_Y > REVC_CHIP_WIDTH);
    assert!(SLOT_SURROGATE_Z > REVC_TOTAL_HEIGHT);
    assert!(SLOT_RELIEF_DEPTH < RACK_GRID_Z - 12.0);
    assert!(rack_open_area_fraction() >= 0.14);
    assert!(fan_free_area_mm2() > shadow_mask_projected_area_mm2());
    assert!(HIGH_TIER_Z > MID_TIER_Z && MID_TIER_Z > LOW_TIER_Z);
    assert!(CAMERA_CLEARANCE_Z > HIGH_TIER_Z + 18.0);
    assert!(ROBOT_Z_CLEARANCE > BASE_Z + EVIDENCE_POST_Z + EVIDENCE_CROSSBAR_Z);
    assert!(front_robot_clearance() >= ROBOT_FRONT_CLEARANCE);
    assert!(rear_service_clearance() >= SERVICE_REAR_CLEARANCE);
    assert!(condensate_capture_volume_ml() > 170.0);

    let rects = module_rects();
    for item in rects {
        assert!(item.fits_inside_deck(), "{} exceeds deck rim", item.name);
    }

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
        rect(
            "rack_slot_surrogate_grid",
            RACK_POS,
            RACK_GRID_X,
            RACK_GRID_Y,
        ),
        rect("fan_plenum_shadow_masks", FAN_POS, FAN_PANEL_X, FAN_PANEL_Y),
        rect(
            "airflow_ribbon_witness_token_lands",
            RIBBON_POS,
            RIBBON_PANEL_X,
            RIBBON_PANEL_Y,
        ),
        rect(
            "multi_height_sensor_mast_pockets",
            MAST_POS,
            MAST_PANEL_X,
            MAST_PANEL_Y,
        ),
        rect(
            "cassette_load_blockers",
            BLOCKER_POS,
            BLOCKER_PANEL_X,
            BLOCKER_PANEL_Y,
        ),
        rect(
            "condensate_drip_collection_lanes",
            CONDENSATE_POS,
            CONDENSATE_X,
            CONDENSATE_Y,
        ),
        rect("barcode_certificate_lands", TRACE_POS, TRACE_X, TRACE_Y),
        rect(
            "release_hold_reject_lanes",
            LANE_POS,
            LANE_BANK_X,
            LANE_BANK_Y,
        ),
        rect(
            "evidence_bridge",
            EVIDENCE_POS,
            EVIDENCE_BRIDGE_X,
            EVIDENCE_BRIDGE_Y,
        ),
    ]
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn base_containment_deck() -> Part {
    let deck = centered_cube(
        "fan_flow_shadowing_base_containment_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let wipe_basin = centered_cube(
        "fan_flow_shadowing_wipeable_secondary_basin_cut",
        STATION_X - 150.0,
        STATION_Y - 126.0,
        8.0,
    )
    .translate(0.0, -8.0, BASE_Z / 2.0 - 3.8);
    let front_condensate_channel = centered_cube(
        "fan_flow_shadowing_front_condensate_channel_cut",
        STATION_X - 220.0,
        18.0,
        9.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 58.0, BASE_Z / 2.0 - 4.0);
    let drain = centered_cylinder(
        "fan_flow_shadowing_closed_deck_drain_placeholder",
        7.0,
        52.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 86.0, -STATION_Y / 2.0 + 34.0, 0.0);

    deck - wipe_basin - front_condensate_channel - drain - insert_sockets() - mounting_slots()
        + perimeter_rims()
        + zone_spines()
        + datum_targets()
        + deck_flow_axis_ticks()
        + evidence_bridge_anchor_lands()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("fan_flow_shadowing_insert_sockets");
    for item in module_rects().into_iter().take(8) {
        sockets = sockets
            + centered_cube(
                format!("fan_flow_shadowing_{}_locator_socket", item.name),
                item.x + 8.0,
                item.y + 8.0,
                SOCKET_DEPTH + 0.6,
            )
            .translate(
                item.center.0,
                item.center.1,
                BASE_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.3,
            );
    }
    sockets
}

fn mounting_slots() -> Part {
    let mut slots = Part::empty("fan_flow_shadowing_mounting_slots");
    for (i, (x, y)) in mount_slot_positions().into_iter().enumerate() {
        let bore = centered_cylinder(
            format!("fan_flow_shadowing_m6_mount_bore_{i}"),
            3.4,
            BASE_Z + 4.0,
            28,
        )
        .translate(x, y, 0.0);
        let service_slot = centered_cube(
            format!("fan_flow_shadowing_m6_mount_service_slot_{i}"),
            30.0,
            7.0,
            BASE_Z + 4.0,
        )
        .translate(x, y, 0.0);
        slots = slots + bore + service_slot;
    }
    slots
}

fn mount_slot_positions() -> [(f64, f64); MOUNT_SLOT_COUNT] {
    [
        (-STATION_X / 2.0 + 60.0, -STATION_Y / 2.0 + 58.0),
        (STATION_X / 2.0 - 60.0, -STATION_Y / 2.0 + 58.0),
        (-STATION_X / 2.0 + 60.0, STATION_Y / 2.0 - 58.0),
        (STATION_X / 2.0 - 60.0, STATION_Y / 2.0 - 58.0),
        (0.0, -STATION_Y / 2.0 + 58.0),
        (0.0, STATION_Y / 2.0 - 58.0),
        (-STATION_X / 2.0 + 60.0, 0.0),
        (STATION_X / 2.0 - 60.0, 0.0),
    ]
}

fn perimeter_rims() -> Part {
    let front = centered_cube(
        "fan_flow_shadowing_front_robot_low_lip",
        STATION_X,
        RIM_W,
        28.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, BASE_Z / 2.0 + 14.0);
    let rear = centered_cube(
        "fan_flow_shadowing_rear_service_high_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let left = centered_cube(
        "fan_flow_shadowing_left_closed_system_rim",
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
        "fan_flow_shadowing_right_closed_system_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );

    front + rear + left + right
}

fn zone_spines() -> Part {
    let rack_flow_spine = centered_cube(
        "fan_flow_shadowing_rack_flow_zone_spine",
        STATION_X - 260.0,
        10.0,
        24.0,
    )
    .translate(0.0, 22.0, BASE_Z / 2.0 + 12.0);
    let lower_workflow_spine = centered_cube(
        "fan_flow_shadowing_witness_disposition_zone_spine",
        STATION_X - 260.0,
        10.0,
        22.0,
    )
    .translate(0.0, -230.0, BASE_Z / 2.0 + 11.0);
    let blocker_spine = centered_cube(
        "fan_flow_shadowing_blocker_traceability_spine",
        10.0,
        330.0,
        22.0,
    )
    .translate(138.0, -300.0, BASE_Z / 2.0 + 11.0);

    rack_flow_spine + lower_workflow_spine + blocker_spine
}

fn datum_targets() -> Part {
    let mut targets = Part::empty("fan_flow_shadowing_robot_datum_targets");
    for (i, (x, y)) in datum_target_positions().into_iter().enumerate() {
        targets = targets
            + fiducial_disc(&format!("fan_flow_shadowing_robot_datum_target_{i}")).translate(
                x,
                y,
                BASE_Z / 2.0 + 2.0,
            );
    }
    targets
}

fn datum_target_positions() -> [(f64, f64); DATUM_TARGET_COUNT] {
    [
        (-STATION_X / 2.0 + 94.0, -STATION_Y / 2.0 + 94.0),
        (STATION_X / 2.0 - 94.0, -STATION_Y / 2.0 + 94.0),
        (-STATION_X / 2.0 + 94.0, STATION_Y / 2.0 - 94.0),
        (STATION_X / 2.0 - 94.0, STATION_Y / 2.0 - 94.0),
    ]
}

fn deck_flow_axis_ticks() -> Part {
    let mut ticks = Part::empty("fan_flow_shadowing_deck_flow_axis_ticks");
    for i in 0..AIRFLOW_RIBBON_COUNT {
        ticks = ticks
            + centered_cube(
                format!("fan_flow_shadowing_global_airflow_axis_tick_{i}"),
                7.0,
                42.0,
                4.0,
            )
            .translate(
                centered_index(i, AIRFLOW_RIBBON_COUNT, 118.0),
                -112.0,
                BASE_Z / 2.0 + 2.0,
            );
    }
    ticks
}

fn evidence_bridge_anchor_lands() -> Part {
    let left = centered_cube(
        "fan_flow_shadowing_evidence_bridge_left_anchor_land",
        100.0,
        42.0,
        8.0,
    )
    .translate(
        EVIDENCE_POS.0 - EVIDENCE_BRIDGE_X / 2.0 + 62.0,
        EVIDENCE_POS.1,
        BASE_Z / 2.0 + 4.0,
    );
    let right = centered_cube(
        "fan_flow_shadowing_evidence_bridge_right_anchor_land",
        100.0,
        42.0,
        8.0,
    )
    .translate(
        EVIDENCE_POS.0 + EVIDENCE_BRIDGE_X / 2.0 - 62.0,
        EVIDENCE_POS.1,
        BASE_Z / 2.0 + 4.0,
    );

    left + right
}

fn rack_slot_surrogate_grid() -> Part {
    let body = centered_cube(
        "fan_flow_shadowing_rack_slot_surrogate_grid_body",
        RACK_GRID_X,
        RACK_GRID_Y,
        RACK_GRID_Z,
    );
    let gasket = gasket_frame_xy(
        "fan_flow_shadowing_rack_grid_wipeable_perimeter_gasket_land",
        RACK_GRID_X - 30.0,
        RACK_GRID_Y - 30.0,
        8.0,
        6.0,
    )
    .translate(0.0, 0.0, RACK_GRID_Z / 2.0 + 3.0);

    body - rack_slot_reliefs() - rack_air_bypass_slots()
        + rack_slot_lips()
        + rack_surrogate_cassettes()
        + rack_datum_pins()
        + rack_airflow_vanes()
        + gasket
}

fn rack_slot_reliefs() -> Part {
    let mut reliefs = Part::empty("fan_flow_shadowing_rack_slot_reliefs");
    for row in 0..RACK_ROWS {
        for col in 0..RACK_COLS {
            let index = rack_slot_index(col, row);
            let (x, y) = rack_slot_center(col, row);
            reliefs = reliefs
                + centered_cube(
                    format!("fan_flow_shadowing_slot_{index}_cassette_relief"),
                    SLOT_SURROGATE_X + 12.0,
                    SLOT_SURROGATE_Y + 12.0,
                    SLOT_RELIEF_DEPTH + 0.6,
                )
                .translate(x, y, RACK_GRID_Z / 2.0 - SLOT_RELIEF_DEPTH / 2.0 + 0.3);
        }
    }
    reliefs
}

fn rack_air_bypass_slots() -> Part {
    let mut slots = Part::empty("fan_flow_shadowing_rack_air_bypass_slots");
    for i in 0..RACK_AIR_BYPASS_SLOT_COUNT {
        let row = i / 5;
        let col = i % 5;
        slots = slots
            + centered_cube(
                format!("fan_flow_shadowing_rack_bypass_flow_window_{i}"),
                86.0,
                8.0,
                RACK_GRID_Z + 2.0,
            )
            .translate(
                centered_index(col, 5, 104.0),
                centered_index(row, 2, 288.0),
                0.0,
            );
    }
    slots
}

fn rack_slot_lips() -> Part {
    let mut lips = Part::empty("fan_flow_shadowing_rack_slot_reference_lips");
    for row in 0..RACK_ROWS {
        for col in 0..RACK_COLS {
            let index = rack_slot_index(col, row);
            let (x, y) = rack_slot_center(col, row);
            let label = slot_zone_label(col, row);
            let front = centered_cube(
                format!("fan_flow_shadowing_slot_{index}_{label}_front_lip"),
                SLOT_SURROGATE_X + 12.0,
                6.0,
                24.0,
            )
            .translate(x, y - SLOT_SURROGATE_Y / 2.0 - 7.0, RACK_GRID_Z / 2.0 + 6.0);
            let rear = centered_cube(
                format!("fan_flow_shadowing_slot_{index}_{label}_rear_lip"),
                SLOT_SURROGATE_X + 12.0,
                6.0,
                12.0,
            )
            .translate(x, y + SLOT_SURROGATE_Y / 2.0 + 7.0, RACK_GRID_Z / 2.0 + 6.0);
            let left_key = centered_cube(
                format!("fan_flow_shadowing_slot_{index}_{label}_left_datum_key"),
                7.0,
                SLOT_SURROGATE_Y * 0.6,
                14.0,
            )
            .translate(x - SLOT_SURROGATE_X / 2.0 - 8.0, y, RACK_GRID_Z / 2.0 + 7.0);
            lips = lips + front + rear + left_key;
        }
    }
    lips
}

fn rack_surrogate_cassettes() -> Part {
    let mut surrogates = Part::empty("fan_flow_shadowing_no_cell_cassette_surrogates");
    for row in 0..RACK_ROWS {
        for col in 0..RACK_COLS {
            let index = rack_slot_index(col, row);
            let (x, y) = rack_slot_center(col, row);
            let cassette = centered_cube(
                format!("fan_flow_shadowing_slot_{index}_thin_cassette_air_shadow_surrogate"),
                SLOT_SURROGATE_X - 20.0,
                SLOT_SURROGATE_Y - 18.0,
                SLOT_SURROGATE_Z,
            )
            .translate(x, y, RACK_GRID_Z / 2.0 + SLOT_SURROGATE_Z / 2.0);
            let witness_window = centered_cube(
                format!("fan_flow_shadowing_slot_{index}_central_flow_view_window"),
                SLOT_SURROGATE_X - 64.0,
                12.0,
                SLOT_SURROGATE_Z + 1.0,
            )
            .translate(x, y, RACK_GRID_Z / 2.0 + SLOT_SURROGATE_Z / 2.0);
            surrogates = surrogates + (cassette - witness_window);
        }
    }
    surrogates
}

fn rack_datum_pins() -> Part {
    let mut pins = Part::empty("fan_flow_shadowing_rack_grid_datum_pins");
    for (i, (x, y)) in [
        (-RACK_GRID_X / 2.0 + 34.0, -RACK_GRID_Y / 2.0 + 34.0),
        (RACK_GRID_X / 2.0 - 34.0, -RACK_GRID_Y / 2.0 + 34.0),
        (-RACK_GRID_X / 2.0 + 34.0, RACK_GRID_Y / 2.0 - 34.0),
        (RACK_GRID_X / 2.0 - 34.0, RACK_GRID_Y / 2.0 - 34.0),
    ]
    .into_iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("fan_flow_shadowing_rack_datum_boss_{i}"),
            8.5,
            7.0,
            32,
        )
        .translate(x, y, RACK_GRID_Z / 2.0 + 3.5);
        let pilot = centered_cylinder(
            format!("fan_flow_shadowing_rack_datum_pilot_bore_{i}"),
            2.8,
            9.0,
            24,
        )
        .translate(x, y, RACK_GRID_Z / 2.0 + 4.5);
        pins = pins + (boss - pilot);
    }
    pins
}

fn rack_airflow_vanes() -> Part {
    let mut vanes = Part::empty("fan_flow_shadowing_rack_airflow_witness_vanes");
    for i in 0..RACK_ROWS {
        let y = centered_index(i, RACK_ROWS, SLOT_PITCH_Y);
        vanes = vanes
            + centered_cube(
                format!("fan_flow_shadowing_rack_row_{i}_airflow_direction_vane"),
                RACK_GRID_X - 92.0,
                5.0,
                18.0,
            )
            .translate(
                0.0,
                y + SLOT_SURROGATE_Y / 2.0 + 24.0,
                RACK_GRID_Z / 2.0 + 9.0,
            );
    }
    vanes
}

fn fan_plenum_shadow_masks() -> Part {
    let panel = centered_cube(
        "fan_flow_shadowing_fan_plenum_shadow_mask_panel",
        FAN_PANEL_X,
        FAN_PANEL_Y,
        FAN_PANEL_Z,
    );
    let plenum_frame = gasket_frame_xy(
        "fan_flow_shadowing_plenum_reference_frame",
        FAN_PANEL_X - 34.0,
        FAN_PANEL_Y - 34.0,
        9.0,
        7.0,
    )
    .translate(0.0, 0.0, FAN_PANEL_Z / 2.0 + 3.5);

    panel - fan_aperture_cuts() - plenum_key_slots()
        + fan_aperture_rings()
        + plenum_baffles()
        + removable_shadow_mask_cards()
        + plenum_flow_arrows()
        + plenum_frame
}

fn fan_aperture_cuts() -> Part {
    let mut cuts = Part::empty("fan_flow_shadowing_fan_aperture_cuts");
    for fan in 0..FAN_COUNT {
        cuts = cuts
            + centered_cylinder(
                format!("fan_flow_shadowing_fan_{fan}_open_aperture_cut"),
                FAN_D / 2.0,
                FAN_PANEL_Z + 3.0,
                48,
            )
            .translate(centered_index(fan, FAN_COUNT, FAN_PITCH_X), 36.0, 0.0);
    }
    cuts
}

fn plenum_key_slots() -> Part {
    let mut slots = Part::empty("fan_flow_shadowing_plenum_mask_key_slots");
    for i in 0..SHADOW_MASK_COUNT {
        slots = slots
            + centered_cube(
                format!("fan_flow_shadowing_shadow_mask_card_{i}_key_slot"),
                SHADOW_MASK_CARD_X + 8.0,
                8.0,
                FAN_PANEL_Z + 2.0,
            )
            .translate(centered_index(i, SHADOW_MASK_COUNT, 74.0), -74.0, 0.0);
    }
    slots
}

fn fan_aperture_rings() -> Part {
    let mut rings = Part::empty("fan_flow_shadowing_fan_aperture_rings");
    for fan in 0..FAN_COUNT {
        let x = centered_index(fan, FAN_COUNT, FAN_PITCH_X);
        let outer = centered_cylinder(
            format!("fan_flow_shadowing_fan_{fan}_aperture_outer_ring"),
            FAN_D / 2.0 + 9.0,
            6.0,
            52,
        );
        let inner = centered_cylinder(
            format!("fan_flow_shadowing_fan_{fan}_aperture_inner_relief"),
            FAN_D / 2.0 - 4.0,
            7.0,
            52,
        );
        let hub = centered_cylinder(
            format!("fan_flow_shadowing_fan_{fan}_hub_shadow_reference"),
            12.0,
            7.0,
            36,
        );
        rings = rings + (outer - inner + hub).translate(x, 36.0, FAN_PANEL_Z / 2.0 + 3.0);
    }
    rings
}

fn plenum_baffles() -> Part {
    let mut baffles = Part::empty("fan_flow_shadowing_plenum_baffles");
    for i in 0..PLENUM_BAFFLE_COUNT {
        let x = centered_index(i, PLENUM_BAFFLE_COUNT, 66.0);
        baffles = baffles
            + centered_cube(
                format!("fan_flow_shadowing_plenum_baffle_{i}"),
                8.0,
                FAN_PANEL_Y - 74.0,
                28.0,
            )
            .translate(x, -4.0, FAN_PANEL_Z / 2.0 + 14.0);
    }
    baffles
}

fn removable_shadow_mask_cards() -> Part {
    let mut masks = Part::empty("fan_flow_shadowing_removable_shadow_mask_cards");
    for i in 0..SHADOW_MASK_COUNT {
        let x = centered_index(i, SHADOW_MASK_COUNT, 74.0);
        let z = FAN_PANEL_Z / 2.0 + 5.0 + i as f64 * 0.8;
        let card = centered_cube(
            format!("fan_flow_shadowing_shadow_mask_card_{i}_coupon"),
            SHADOW_MASK_CARD_X,
            SHADOW_MASK_CARD_Y,
            5.0,
        )
        .translate(x, -74.0, z);
        let pull_tab = centered_cube(
            format!("fan_flow_shadowing_shadow_mask_card_{i}_pull_tab"),
            18.0,
            14.0,
            8.0,
        )
        .translate(x, -105.0, z + 1.5);
        masks = masks + card + pull_tab;
    }
    masks
}

fn plenum_flow_arrows() -> Part {
    let mut arrows = Part::empty("fan_flow_shadowing_plenum_flow_direction_ticks");
    for i in 0..AIRFLOW_RIBBON_COUNT {
        arrows = arrows
            + centered_cube(
                format!("fan_flow_shadowing_plenum_flow_direction_tick_{i}"),
                24.0,
                4.0,
                4.0,
            )
            .translate(
                centered_index(i, AIRFLOW_RIBBON_COUNT, 42.0),
                -18.0,
                FAN_PANEL_Z / 2.0 + 2.0,
            );
    }
    arrows
}

fn airflow_ribbon_witness_token_lands() -> Part {
    let plate = centered_cube(
        "fan_flow_shadowing_airflow_ribbon_witness_token_plate",
        RIBBON_PANEL_X,
        RIBBON_PANEL_Y,
        RIBBON_PANEL_Z,
    );

    plate - ribbon_recesses() + airflow_ribbons() + witness_token_lands() + ribbon_scale_ticks()
}

fn ribbon_recesses() -> Part {
    let mut recesses = Part::empty("fan_flow_shadowing_ribbon_recesses");
    for i in 0..AIRFLOW_RIBBON_COUNT {
        recesses = recesses
            + centered_cube(
                format!("fan_flow_shadowing_airflow_ribbon_{i}_recess"),
                RIBBON_PANEL_X - 60.0,
                7.0,
                5.0,
            )
            .translate(
                0.0,
                centered_index(i, AIRFLOW_RIBBON_COUNT, RIBBON_SPACING_Y),
                RIBBON_PANEL_Z / 2.0 - 2.0,
            );
    }
    recesses
}

fn airflow_ribbons() -> Part {
    let mut ribbons = Part::empty("fan_flow_shadowing_airflow_ribbon_lands");
    for i in 0..AIRFLOW_RIBBON_COUNT {
        let y = centered_index(i, AIRFLOW_RIBBON_COUNT, RIBBON_SPACING_Y);
        ribbons = ribbons
            + centered_cube(
                format!("fan_flow_shadowing_airflow_ribbon_{i}_land"),
                RIBBON_PANEL_X - 84.0,
                4.0,
                4.0,
            )
            .translate(0.0, y, RIBBON_PANEL_Z / 2.0 + 2.0);
    }
    ribbons
}

fn witness_token_lands() -> Part {
    let mut tokens = Part::empty("fan_flow_shadowing_witness_token_lands");
    for i in 0..WITNESS_TOKEN_COUNT {
        let pair = i % 2;
        let ribbon = i / 2;
        let x = if pair == 0 { -150.0 } else { 150.0 };
        let y = centered_index(ribbon, AIRFLOW_RIBBON_COUNT, RIBBON_SPACING_Y);
        let rim = centered_cylinder(
            format!("fan_flow_shadowing_witness_token_{i}_rim"),
            WITNESS_TOKEN_D / 2.0 + 4.0,
            3.0,
            36,
        );
        let pocket = centered_cylinder(
            format!("fan_flow_shadowing_witness_token_{i}_pocket"),
            WITNESS_TOKEN_D / 2.0,
            4.0,
            36,
        );
        tokens = tokens + (rim - pocket).translate(x, y, RIBBON_PANEL_Z / 2.0 + 1.5);
    }
    tokens
}

fn ribbon_scale_ticks() -> Part {
    let mut ticks = Part::empty("fan_flow_shadowing_ribbon_scale_ticks");
    for i in 0..7 {
        ticks = ticks
            + centered_cube(
                format!("fan_flow_shadowing_ribbon_velocity_scale_tick_{i}"),
                3.0,
                RIBBON_PANEL_Y - 36.0,
                5.0,
            )
            .translate(centered_index(i, 7, 48.0), 0.0, RIBBON_PANEL_Z / 2.0 + 2.5);
    }
    ticks
}

fn multi_height_sensor_mast_pockets() -> Part {
    let plate = centered_cube(
        "fan_flow_shadowing_multi_height_sensor_mast_pocket_plate",
        MAST_PANEL_X,
        MAST_PANEL_Y,
        MAST_PANEL_Z,
    );

    plate - mast_socket_cuts()
        + mast_socket_rings()
        + height_tier_gauge_posts()
        + mast_cable_relief_comb()
        + mast_position_labels()
}

fn mast_socket_cuts() -> Part {
    let mut cuts = Part::empty("fan_flow_shadowing_mast_socket_cuts");
    for mast in 0..MAST_POCKET_COUNT {
        let (x, y) = mast_pocket_center(mast);
        cuts = cuts
            + centered_cylinder(
                format!("fan_flow_shadowing_mast_{mast}_vertical_socket_cut"),
                MAST_SOCKET_D / 2.0,
                MAST_PANEL_Z + 2.0,
                32,
            )
            .translate(x, y, 0.0);
    }
    cuts
}

fn mast_socket_rings() -> Part {
    let mut rings = Part::empty("fan_flow_shadowing_mast_socket_rings");
    for mast in 0..MAST_POCKET_COUNT {
        let (x, y) = mast_pocket_center(mast);
        let ring = centered_cylinder(
            format!("fan_flow_shadowing_mast_{mast}_pocket_reference_ring"),
            MAST_BASE_RING_D / 2.0,
            5.0,
            36,
        );
        let cut = centered_cylinder(
            format!("fan_flow_shadowing_mast_{mast}_ring_center_clearance"),
            MAST_SOCKET_D / 2.0 + 1.0,
            6.0,
            36,
        );
        rings = rings + (ring - cut).translate(x, y, MAST_PANEL_Z / 2.0 + 2.5);
    }
    rings
}

fn height_tier_gauge_posts() -> Part {
    let mut gauges = Part::empty("fan_flow_shadowing_low_mid_high_sensor_height_gauges");
    for mast in 0..MAST_POCKET_COUNT {
        let (x, y) = mast_pocket_center(mast);
        for (tier, height) in [LOW_TIER_Z, MID_TIER_Z, HIGH_TIER_Z]
            .into_iter()
            .enumerate()
        {
            let post = centered_cylinder(
                format!("fan_flow_shadowing_mast_{mast}_height_tier_{tier}_gauge_post"),
                3.2,
                height,
                20,
            )
            .translate(
                x + 24.0 + tier as f64 * 9.0,
                y,
                MAST_PANEL_Z / 2.0 + height / 2.0,
            );
            let flag = centered_cube(
                format!("fan_flow_shadowing_mast_{mast}_height_tier_{tier}_flag"),
                20.0,
                3.0,
                6.0,
            )
            .translate(
                x + 24.0 + tier as f64 * 9.0,
                y + 10.0,
                MAST_PANEL_Z / 2.0 + height,
            );
            gauges = gauges + post + flag;
        }
    }
    gauges
}

fn mast_cable_relief_comb() -> Part {
    let mut comb = Part::empty("fan_flow_shadowing_sensor_mast_cable_relief_comb");
    for i in 0..MAST_POCKET_COUNT {
        comb = comb
            + centered_cube(
                format!("fan_flow_shadowing_sensor_mast_{i}_cable_comb_slot"),
                9.0,
                28.0,
                8.0,
            )
            .translate(
                centered_index(i, MAST_POCKET_COUNT, 62.0),
                -MAST_PANEL_Y / 2.0 + 18.0,
                MAST_PANEL_Z / 2.0 + 4.0,
            );
    }
    comb
}

fn mast_position_labels() -> Part {
    let mut labels = Part::empty("fan_flow_shadowing_sensor_mast_position_label_lands");
    for mast in 0..MAST_POCKET_COUNT {
        let (x, y) = mast_pocket_center(mast);
        labels = labels
            + centered_cube(
                format!("fan_flow_shadowing_mast_{mast}_position_label_land"),
                42.0,
                12.0,
                2.0,
            )
            .translate(x, y - 26.0, MAST_PANEL_Z / 2.0 + 1.0);
    }
    labels
}

fn cassette_load_blockers() -> Part {
    let plate = centered_cube(
        "fan_flow_shadowing_cassette_load_blocker_plate",
        BLOCKER_PANEL_X,
        BLOCKER_PANEL_Y,
        BLOCKER_PANEL_Z,
    );

    plate - blocker_locator_recesses() + blocker_coupons() + blocker_percent_reference_ticks()
}

fn blocker_locator_recesses() -> Part {
    let mut recesses = Part::empty("fan_flow_shadowing_load_blocker_locator_recesses");
    for i in 0..LOAD_BLOCKER_COUNT {
        recesses = recesses
            + centered_cube(
                format!("fan_flow_shadowing_load_blocker_{i}_locator_recess"),
                BLOCKER_WIDTH + 10.0,
                BLOCKER_DEPTH + 8.0,
                5.0,
            )
            .translate(blocker_x(i), blocker_y(i), BLOCKER_PANEL_Z / 2.0 - 2.0);
    }
    recesses
}

fn blocker_coupons() -> Part {
    let mut blockers = Part::empty("fan_flow_shadowing_cassette_load_blocker_coupons");
    for i in 0..LOAD_BLOCKER_COUNT {
        let height = blocker_height(i);
        let body = centered_cube(
            format!(
                "fan_flow_shadowing_load_blocker_{i}_{}_percent_body",
                blocker_percent(i)
            ),
            BLOCKER_WIDTH,
            BLOCKER_DEPTH,
            height,
        )
        .translate(
            blocker_x(i),
            blocker_y(i),
            BLOCKER_PANEL_Z / 2.0 + height / 2.0,
        );
        let grip = centered_cube(
            format!("fan_flow_shadowing_load_blocker_{i}_finger_grip"),
            BLOCKER_WIDTH * 0.72,
            7.0,
            10.0,
        )
        .translate(
            blocker_x(i),
            blocker_y(i) - BLOCKER_DEPTH / 2.0 - 5.0,
            BLOCKER_PANEL_Z / 2.0 + height + 5.0,
        );
        blockers = blockers + body + grip;
    }
    blockers
}

fn blocker_percent_reference_ticks() -> Part {
    let mut ticks = Part::empty("fan_flow_shadowing_blocker_percent_reference_ticks");
    for i in 0..LOAD_BLOCKER_COUNT {
        for tick in 0..=i % 3 {
            ticks = ticks
                + centered_cube(
                    format!("fan_flow_shadowing_load_blocker_{i}_percent_tick_{tick}"),
                    18.0,
                    3.0,
                    4.0,
                )
                .translate(
                    blocker_x(i) - 22.0 + tick as f64 * 11.0,
                    blocker_y(i) + BLOCKER_DEPTH / 2.0 + 8.0,
                    BLOCKER_PANEL_Z / 2.0 + 2.0,
                );
        }
    }
    ticks
}

fn condensate_drip_collection_lanes() -> Part {
    let tray = centered_cube(
        "fan_flow_shadowing_condensate_drip_collection_tray",
        CONDENSATE_X,
        CONDENSATE_Y,
        CONDENSATE_Z,
    );

    tray - drip_lane_cuts() - cup_pocket_cuts()
        + drip_lane_ribs()
        + collection_cup_lands()
        + drain_witness_bosses()
}

fn drip_lane_cuts() -> Part {
    let mut cuts = Part::empty("fan_flow_shadowing_drip_lane_cuts");
    for lane in 0..DRIP_LANE_COUNT {
        cuts = cuts
            + centered_cube(
                format!("fan_flow_shadowing_condensate_drip_lane_{lane}_cut"),
                CONDENSATE_X - 80.0,
                DRIP_LANE_W,
                DRIP_LANE_DEPTH + 0.8,
            )
            .translate(
                0.0,
                centered_index(lane, DRIP_LANE_COUNT, 16.0),
                CONDENSATE_Z / 2.0 - DRIP_LANE_DEPTH / 2.0 + 0.4,
            );
    }
    cuts
}

fn cup_pocket_cuts() -> Part {
    let mut cuts = Part::empty("fan_flow_shadowing_collection_cup_pocket_cuts");
    for cup in 0..COLLECTION_CUP_COUNT {
        cuts = cuts
            + centered_cylinder(
                format!("fan_flow_shadowing_collection_cup_{cup}_pocket_cut"),
                COLLECTION_CUP_D / 2.0,
                DRIP_LANE_DEPTH + 2.0,
                36,
            )
            .translate(
                centered_index(cup, COLLECTION_CUP_COUNT, 118.0),
                -CONDENSATE_Y / 2.0 + 32.0,
                CONDENSATE_Z / 2.0 - DRIP_LANE_DEPTH / 2.0,
            );
    }
    cuts
}

fn drip_lane_ribs() -> Part {
    let mut ribs = Part::empty("fan_flow_shadowing_drip_lane_separating_ribs");
    for lane in 0..=DRIP_LANE_COUNT {
        ribs = ribs
            + centered_cube(
                format!("fan_flow_shadowing_drip_lane_boundary_rib_{lane}"),
                CONDENSATE_X - 70.0,
                3.0,
                7.0,
            )
            .translate(
                0.0,
                centered_index(lane, DRIP_LANE_COUNT + 1, 16.0),
                CONDENSATE_Z / 2.0 + 3.5,
            );
    }
    ribs
}

fn collection_cup_lands() -> Part {
    let mut lands = Part::empty("fan_flow_shadowing_collection_cup_lands");
    for cup in 0..COLLECTION_CUP_COUNT {
        lands = lands
            + centered_cylinder(
                format!("fan_flow_shadowing_collection_cup_{cup}_raised_land"),
                COLLECTION_CUP_D / 2.0 + 5.0,
                4.0,
                40,
            )
            .translate(
                centered_index(cup, COLLECTION_CUP_COUNT, 118.0),
                -CONDENSATE_Y / 2.0 + 32.0,
                CONDENSATE_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn drain_witness_bosses() -> Part {
    let mut bosses = Part::empty("fan_flow_shadowing_condensate_drain_witness_bosses");
    for i in 0..DRIP_LANE_COUNT {
        bosses = bosses
            + centered_cylinder(
                format!("fan_flow_shadowing_drip_lane_{i}_terminal_witness_boss"),
                6.0,
                6.0,
                24,
            )
            .translate(
                CONDENSATE_X / 2.0 - 32.0,
                centered_index(i, DRIP_LANE_COUNT, 16.0),
                CONDENSATE_Z / 2.0 + 3.0,
            );
    }
    bosses
}

fn barcode_certificate_lands() -> Part {
    let plate = centered_cube(
        "fan_flow_shadowing_barcode_certificate_traceability_plate",
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    );
    let mut lands = Part::empty("fan_flow_shadowing_barcode_certificate_lands");
    for i in 0..BARCODE_LAND_COUNT {
        lands = lands
            + centered_cube(
                format!("fan_flow_shadowing_barcode_land_{i}"),
                46.0,
                12.0,
                2.2,
            )
            .translate(
                centered_index(i % 4, 4, 48.0),
                centered_index(i / 4, 2, 26.0),
                TRACE_Z / 2.0 + 1.1,
            );
    }
    for i in 0..CERTIFICATE_LAND_COUNT {
        lands = lands
            + centered_cube(
                format!("fan_flow_shadowing_certificate_land_{i}"),
                54.0,
                16.0,
                2.4,
            )
            .translate(
                centered_index(i, CERTIFICATE_LAND_COUNT, 62.0),
                31.0,
                TRACE_Z / 2.0 + 1.2,
            );
    }

    plate + lands + trace_fiducials()
}

fn trace_fiducials() -> Part {
    let mut fiducials = Part::empty("fan_flow_shadowing_traceability_fiducials");
    for (i, (x, y)) in [(-84.0, -28.0), (84.0, -28.0)].into_iter().enumerate() {
        fiducials = fiducials
            + fiducial_disc(&format!("fan_flow_shadowing_trace_fiducial_{i}")).translate(
                x,
                y,
                TRACE_Z / 2.0 + 2.0,
            );
    }
    fiducials
}

fn release_hold_reject_lanes() -> Part {
    let mut lanes = Part::empty("fan_flow_shadowing_release_hold_reject_lanes");
    for lane in DispositionLane::all() {
        let index = lane.index();
        let x = centered_index(index, LANE_COUNT, 132.0);
        let base = centered_cube(
            format!("fan_flow_shadowing_{}_lane_base", lane.name()),
            LANE_SLOT_X,
            LANE_SLOT_Y,
            LANE_BANK_Z,
        );
        let pocket = centered_cube(
            format!("fan_flow_shadowing_{}_lane_token_recess", lane.name()),
            LANE_SLOT_X - 24.0,
            LANE_SLOT_Y - 18.0,
            7.0,
        )
        .translate(0.0, 0.0, LANE_BANK_Z / 2.0 - 3.0);
        let front_wall = centered_cube(
            format!("fan_flow_shadowing_{}_lane_front_retainer", lane.name()),
            LANE_SLOT_X,
            7.0,
            15.0,
        )
        .translate(0.0, -LANE_SLOT_Y / 2.0 + 3.5, LANE_BANK_Z / 2.0 + 7.5);
        let capacity_ticks = lane_capacity_ticks(lane);
        lanes = lanes + (base - pocket + front_wall + capacity_ticks).translate(x, 0.0, 0.0);
    }
    lanes
}

fn lane_capacity_ticks(lane: DispositionLane) -> Part {
    let mut ticks = Part::empty(format!(
        "fan_flow_shadowing_{}_lane_capacity_ticks",
        lane.name()
    ));
    for i in 0..lane.capacity() {
        ticks = ticks
            + centered_cube(
                format!("fan_flow_shadowing_{}_lane_capacity_tick_{i}", lane.name()),
                5.0,
                12.0,
                3.0,
            )
            .translate(
                centered_index(i, lane.capacity(), 10.0),
                LANE_SLOT_Y / 2.0 - 10.0,
                LANE_BANK_Z / 2.0 + 1.5,
            );
    }
    ticks
}

fn evidence_bridge() -> Part {
    let left_anchor = bridge_anchor("left", -EVIDENCE_BRIDGE_X / 2.0 + 62.0);
    let right_anchor = bridge_anchor("right", EVIDENCE_BRIDGE_X / 2.0 - 62.0);
    let left_post = centered_cube(
        "fan_flow_shadowing_evidence_bridge_left_post",
        26.0,
        34.0,
        EVIDENCE_POST_Z,
    )
    .translate(
        -EVIDENCE_BRIDGE_X / 2.0 + 62.0,
        0.0,
        EVIDENCE_ANCHOR_Z / 2.0 + EVIDENCE_POST_Z / 2.0,
    );
    let right_post = centered_cube(
        "fan_flow_shadowing_evidence_bridge_right_post",
        26.0,
        34.0,
        EVIDENCE_POST_Z,
    )
    .translate(
        EVIDENCE_BRIDGE_X / 2.0 - 62.0,
        0.0,
        EVIDENCE_ANCHOR_Z / 2.0 + EVIDENCE_POST_Z / 2.0,
    );
    let crossbar = centered_cube(
        "fan_flow_shadowing_evidence_bridge_overhead_crossbar",
        EVIDENCE_BRIDGE_X,
        EVIDENCE_BRIDGE_Y,
        EVIDENCE_CROSSBAR_Z,
    )
    .translate(
        0.0,
        0.0,
        EVIDENCE_ANCHOR_Z / 2.0 + EVIDENCE_POST_Z + EVIDENCE_CROSSBAR_Z / 2.0,
    );

    left_anchor
        + right_anchor
        + left_post
        + right_post
        + crossbar
        + evidence_camera_targets()
        + evidence_light_pipe_lands()
}

fn bridge_anchor(label: &str, x: f64) -> Part {
    let pad = centered_cube(
        format!("fan_flow_shadowing_evidence_bridge_{label}_anchor_pad"),
        96.0,
        40.0,
        EVIDENCE_ANCHOR_Z,
    );
    let screw_1 = centered_cylinder(
        format!("fan_flow_shadowing_evidence_bridge_{label}_front_screw_clearance"),
        3.2,
        EVIDENCE_ANCHOR_Z + 2.0,
        24,
    )
    .translate(-22.0, -10.0, 0.0);
    let screw_2 = centered_cylinder(
        format!("fan_flow_shadowing_evidence_bridge_{label}_rear_screw_clearance"),
        3.2,
        EVIDENCE_ANCHOR_Z + 2.0,
        24,
    )
    .translate(22.0, 10.0, 0.0);

    (pad - screw_1 - screw_2).translate(x, 0.0, 0.0)
}

fn evidence_camera_targets() -> Part {
    let mut targets = Part::empty("fan_flow_shadowing_evidence_camera_targets");
    for i in 0..CAMERA_TARGET_COUNT {
        let target = fiducial_disc(&format!("fan_flow_shadowing_evidence_camera_target_{i}"))
            .translate(
                centered_index(i, CAMERA_TARGET_COUNT, 210.0),
                -EVIDENCE_BRIDGE_Y / 2.0 + 16.0,
                EVIDENCE_ANCHOR_Z / 2.0 + EVIDENCE_POST_Z + EVIDENCE_CROSSBAR_Z + 2.0,
            );
        targets = targets + target;
    }
    targets
}

fn evidence_light_pipe_lands() -> Part {
    let mut lands = Part::empty("fan_flow_shadowing_evidence_light_pipe_lands");
    for i in 0..LIGHT_PIPE_COUNT {
        lands = lands
            + centered_cube(
                format!("fan_flow_shadowing_evidence_light_pipe_land_{i}"),
                54.0,
                8.0,
                6.0,
            )
            .translate(
                centered_index(i, LIGHT_PIPE_COUNT, 116.0),
                EVIDENCE_BRIDGE_Y / 2.0 - 10.0,
                EVIDENCE_ANCHOR_Z / 2.0 + EVIDENCE_POST_Z + EVIDENCE_CROSSBAR_Z + 3.0,
            );
    }
    lands
}

fn robot_service_keepout_gauges() -> Part {
    let perimeter = gasket_frame_xy(
        "fan_flow_shadowing_robot_service_keepout_perimeter_gauge",
        KEEP_OUT_X,
        KEEP_OUT_Y,
        8.0,
        KEEP_OUT_Z,
    );
    let front_robot = centered_cube(
        "fan_flow_shadowing_robot_front_approach_keepout_gauge",
        KEEP_OUT_X - 160.0,
        12.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + ROBOT_FRONT_CLEARANCE, 0.0);
    let rear_service = centered_cube(
        "fan_flow_shadowing_rear_service_sweep_keepout_gauge",
        KEEP_OUT_X - 180.0,
        12.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - SERVICE_REAR_CLEARANCE, 0.0);
    let left_tool = centered_cube(
        "fan_flow_shadowing_left_robot_tool_keepout_gauge",
        12.0,
        KEEP_OUT_Y - 150.0,
        KEEP_OUT_Z,
    )
    .translate(-STATION_X / 2.0 + 150.0, 0.0, 0.0);
    let right_service = centered_cube(
        "fan_flow_shadowing_right_service_probe_keepout_gauge",
        12.0,
        KEEP_OUT_Y - 150.0,
        KEEP_OUT_Z,
    )
    .translate(STATION_X / 2.0 - 150.0, 0.0, 0.0);
    let z_clearance = centered_cube(
        "fan_flow_shadowing_z_clearance_reference_gauge",
        86.0,
        16.0,
        ROBOT_Z_CLEARANCE,
    )
    .translate(0.0, STATION_Y / 2.0 - 108.0, ROBOT_Z_CLEARANCE / 2.0);

    perimeter + front_robot + rear_service + left_tool + right_service + z_clearance
}

fn fiducial_disc(name: &str) -> Part {
    let disc = centered_cylinder(format!("{name}_disc"), 10.0, 4.0, 40);
    let groove_x = centered_cube(format!("{name}_x_groove"), 18.0, 1.4, 5.0);
    let groove_y = centered_cube(format!("{name}_y_groove"), 1.4, 18.0, 5.0);
    disc - groove_x - groove_y
}

fn gasket_frame_xy(name: &str, x: f64, y: f64, wall: f64, z: f64) -> Part {
    let outer = centered_cube(format!("{name}_outer"), x, y, z);
    let inner = centered_cube(
        format!("{name}_inner_cut"),
        x - 2.0 * wall,
        y - 2.0 * wall,
        z + 1.0,
    );
    outer - inner
}

fn rack_slot_center(col: usize, row: usize) -> (f64, f64) {
    (
        centered_index(col, RACK_COLS, SLOT_PITCH_X),
        centered_index(row, RACK_ROWS, SLOT_PITCH_Y),
    )
}

fn rack_slot_index(col: usize, row: usize) -> usize {
    row * RACK_COLS + col
}

fn is_edge_slot(col: usize, row: usize) -> bool {
    col == 0 || col == RACK_COLS - 1 || row == 0 || row == RACK_ROWS - 1
}

fn slot_zone_label(col: usize, row: usize) -> &'static str {
    if is_edge_slot(col, row) {
        "edge_shadow"
    } else {
        "center_flow"
    }
}

fn edge_slot_count() -> usize {
    let mut count = 0;
    for row in 0..RACK_ROWS {
        for col in 0..RACK_COLS {
            if is_edge_slot(col, row) {
                count += 1;
            }
        }
    }
    count
}

fn center_slot_count() -> usize {
    RACK_SLOT_COUNT - edge_slot_count()
}

fn mast_pocket_center(index: usize) -> (f64, f64) {
    (
        centered_index(index % MAST_COLS, MAST_COLS, MAST_PITCH_X),
        centered_index(index / MAST_COLS, MAST_ROWS, MAST_PITCH_Y),
    )
}

fn blocker_x(index: usize) -> f64 {
    centered_index(index % 2, 2, 58.0)
}

fn blocker_y(index: usize) -> f64 {
    centered_index(index / 2, 3, BLOCKER_PITCH_Y * 2.0)
}

fn blocker_height(index: usize) -> f64 {
    match index % 3 {
        0 => 20.0,
        1 => 36.0,
        _ => 54.0,
    }
}

fn blocker_percent(index: usize) -> usize {
    match index % 3 {
        0 => 25,
        1 => 50,
        _ => 75,
    }
}

fn total_lane_capacity() -> usize {
    DispositionLane::all()
        .into_iter()
        .map(DispositionLane::capacity)
        .sum()
}

fn rack_open_area_fraction() -> f64 {
    let bypass_area = RACK_AIR_BYPASS_SLOT_COUNT as f64 * 86.0 * 8.0;
    let witness_area = RACK_SLOT_COUNT as f64 * (SLOT_SURROGATE_X - 64.0) * 24.0;
    (bypass_area + witness_area) / (RACK_GRID_X * RACK_GRID_Y)
}

fn fan_free_area_mm2() -> f64 {
    FAN_COUNT as f64 * std::f64::consts::PI * (FAN_D / 2.0).powi(2)
}

fn shadow_mask_projected_area_mm2() -> f64 {
    SHADOW_MASK_COUNT as f64 * SHADOW_MASK_CARD_X * SHADOW_MASK_CARD_Y
}

fn condensate_capture_volume_ml() -> f64 {
    let lane_volume =
        DRIP_LANE_COUNT as f64 * (CONDENSATE_X - 80.0) * DRIP_LANE_W * DRIP_LANE_DEPTH / 1000.0;
    let cup_volume = COLLECTION_CUP_COUNT as f64
        * std::f64::consts::PI
        * (COLLECTION_CUP_D / 2.0).powi(2)
        * DRIP_LANE_DEPTH
        / 1000.0;
    lane_volume + cup_volume
}

fn front_robot_clearance() -> f64 {
    STATION_Y / 2.0 - (LANE_POS.1.abs() + LANE_BANK_Y / 2.0)
}

fn rear_service_clearance() -> f64 {
    STATION_Y / 2.0 - (EVIDENCE_POS.1 + EVIDENCE_BRIDGE_Y / 2.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_manifest_covers_required_subsystems() {
        assert_eq!(OUTPUTS.len(), 12);
        assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
        for feature in REQUIRED_FEATURES {
            if feature != "named_stl_outputs" {
                assert!(
                    OUTPUTS.iter().any(|path| path.contains(feature)),
                    "missing output for {feature}"
                );
            }
        }
    }

    #[test]
    fn rack_and_sensor_counts_are_consistent() {
        assert_eq!(RACK_SLOT_COUNT, 12);
        assert_eq!(edge_slot_count(), EDGE_SLOT_COUNT);
        assert_eq!(center_slot_count(), CENTER_SLOT_COUNT);
        assert_eq!(MAST_POCKET_COUNT * HEIGHT_TIER_COUNT, 24);
        assert_eq!(total_lane_capacity(), RACK_SLOT_COUNT);
    }

    #[test]
    fn layout_modules_fit_without_overlap() {
        let rects = module_rects();
        for item in rects {
            assert!(item.fits_inside_deck(), "{} exceeds deck rim", item.name);
        }
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
    fn flow_and_condensate_capacity_have_margin() {
        assert!(rack_open_area_fraction() >= 0.14);
        assert!(fan_free_area_mm2() > shadow_mask_projected_area_mm2());
        assert!(condensate_capture_volume_ml() > 170.0);
    }
}
