use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed incubator humidity recovery load challenge station.
//
// This standalone CAD generator models a no-cell validation fixture for
// stressing an incubator with repeatable wet/dry cassette-equivalent loads and
// measuring humidity recovery after controlled door events. It is fixture CAD
// only: acceptance limits, probe calibration, incubator recipes, and batch
// disposition decisions remain validation-system controls outside this model.

const OUTPUT_PREFIX: &str = "closed_incubator_humidity_recovery_load_challenge_station";

const OUTPUTS: [&str; 11] = [
    "output/closed_incubator_humidity_recovery_load_challenge_station_base_tray.stl",
    "output/closed_incubator_humidity_recovery_load_challenge_station_cassette_surrogate_load_rack.stl",
    "output/closed_incubator_humidity_recovery_load_challenge_station_rh_dewpoint_probe_nests.stl",
    "output/closed_incubator_humidity_recovery_load_challenge_station_wet_dry_load_coupons.stl",
    "output/closed_incubator_humidity_recovery_load_challenge_station_door_event_token_lane.stl",
    "output/closed_incubator_humidity_recovery_load_challenge_station_condensate_collection_features.stl",
    "output/closed_incubator_humidity_recovery_load_challenge_station_barcode_certificate_lands.stl",
    "output/closed_incubator_humidity_recovery_load_challenge_station_release_hold_reject_lanes.stl",
    "output/closed_incubator_humidity_recovery_load_challenge_station_evidence_bridge.stl",
    "output/closed_incubator_humidity_recovery_load_challenge_station_robot_service_keepouts.stl",
    "output/closed_incubator_humidity_recovery_load_challenge_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 10] = [
    "cassette_surrogate_load_rack",
    "rh_dewpoint_probe_nests",
    "wet_dry_load_coupons",
    "door_event_token_lane",
    "condensate_collection_features",
    "barcode_certificate_lands",
    "release_hold_reject_lanes",
    "evidence_bridge",
    "robot_service_keepouts",
    "named_stl_outputs",
];

const STATION_X: f64 = 1120.0;
const STATION_Y: f64 = 800.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const SOCKET_DEPTH: f64 = 5.0;
const MOUNT_SLOT_COUNT: usize = 8;
const DATUM_TARGET_COUNT: usize = 4;

const RACK_X: f64 = 540.0;
const RACK_Y: f64 = 360.0;
const RACK_Z: f64 = 42.0;
const RACK_COLS: usize = 3;
const RACK_ROWS: usize = 2;
const CASSETTE_SLOT_COUNT: usize = RACK_COLS * RACK_ROWS;
const RACK_SLOT_PITCH_X: f64 = 156.0;
const RACK_SLOT_PITCH_Y: f64 = 136.0;
const CASSETTE_SURROGATE_X: f64 = 132.0;
const CASSETTE_SURROGATE_Y: f64 = 102.0;
const CASSETTE_SURROGATE_Z: f64 = 30.0;
const CASSETTE_BALLAST_PUCK_COUNT: usize = 4;
const AIR_BYPASS_SLOT_COUNT: usize = 8;
const RACK_POS: (f64, f64) = (-255.0, 120.0);

const PROBE_NEST_X: f64 = 390.0;
const PROBE_NEST_Y: f64 = 150.0;
const PROBE_NEST_Z: f64 = 44.0;
const PROBE_NEST_COUNT: usize = 5;
const PROBE_PITCH_X: f64 = 70.0;
const PROBE_POS: (f64, f64) = (300.0, 235.0);

const COUPON_PLATE_X: f64 = 390.0;
const COUPON_PLATE_Y: f64 = 210.0;
const COUPON_PLATE_Z: f64 = 34.0;
const WET_COUPON_COUNT: usize = 6;
const DRY_COUPON_COUNT: usize = 6;
const COUPON_SLOT_X: f64 = 48.0;
const COUPON_SLOT_Y: f64 = 32.0;
const COUPON_PITCH_X: f64 = 58.0;
const WET_TROUGH_VOLUME_ML: f64 = 190.0;
const COUPON_POS: (f64, f64) = (310.0, 30.0);

const TOKEN_LANE_X: f64 = 390.0;
const TOKEN_LANE_Y: f64 = 96.0;
const TOKEN_LANE_Z: f64 = 22.0;
const DOOR_EVENT_TOKEN_COUNT: usize = 8;
const TOKEN_PITCH_X: f64 = 43.0;
const TOKEN_D: f64 = 26.0;
const TOKEN_POS: (f64, f64) = (310.0, -145.0);

const CONDENSATE_X: f64 = 480.0;
const CONDENSATE_Y: f64 = 180.0;
const CONDENSATE_Z: f64 = 34.0;
const CONDENSATE_BASIN_DEPTH: f64 = 19.0;
const CONDENSATE_CHANNEL_COUNT: usize = 6;
const COLLECTION_CUP_COUNT: usize = 3;
const CUP_LAND_D: f64 = 70.0;
const DRAIN_BORE_D: f64 = 11.0;
const CONDENSATE_POS: (f64, f64) = (-255.0, -170.0);

const TRACE_X: f64 = 380.0;
const TRACE_Y: f64 = 86.0;
const TRACE_Z: f64 = 12.0;
const BARCODE_LAND_COUNT: usize = 8;
const CERTIFICATE_LAND_COUNT: usize = 3;
const TRACE_POS: (f64, f64) = (310.0, -290.0);

const LANE_X: f64 = 480.0;
const LANE_Y: f64 = 96.0;
const LANE_Z: f64 = 20.0;
const DISPOSITION_LANE_COUNT: usize = 3;
const LANE_POS: (f64, f64) = (-255.0, -312.0);

const EVIDENCE_BRIDGE_X: f64 = 1010.0;
const EVIDENCE_BRIDGE_Y: f64 = 66.0;
const EVIDENCE_BRIDGE_Z: f64 = 84.0;
const EVIDENCE_CAMERA_COUNT: usize = 4;
const EVIDENCE_LIGHT_PIPE_COUNT: usize = 7;
const EVIDENCE_POS: (f64, f64) = (0.0, 330.0);

const KEEP_OUT_X: f64 = 1060.0;
const KEEP_OUT_Y: f64 = 760.0;
const KEEP_OUT_Z: f64 = 5.0;
const ROBOT_KEEP_OUT_ZONE_COUNT: usize = 5;
const FRONT_ROBOT_APPROACH_CLEARANCE: f64 = 36.0;
const REAR_SERVICE_SWEEP_CLEARANCE: f64 = 32.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_tray();
    export(OUTPUTS[0], &base);

    let rack = cassette_surrogate_load_rack();
    export(OUTPUTS[1], &rack);

    let probes = rh_dewpoint_probe_nests();
    export(OUTPUTS[2], &probes);

    let coupons = wet_dry_load_coupons();
    export(OUTPUTS[3], &coupons);

    let tokens = door_event_token_lane();
    export(OUTPUTS[4], &tokens);

    let condensate = condensate_collection_features();
    export(OUTPUTS[5], &condensate);

    let trace = barcode_certificate_lands();
    export(OUTPUTS[6], &trace);

    let lanes = release_hold_reject_lanes();
    export(OUTPUTS[7], &lanes);

    let bridge = evidence_bridge();
    export(OUTPUTS[8], &bridge);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[9], &keepouts);

    let assembly = base
        + rack.translate(RACK_POS.0, RACK_POS.1, on_base_z(RACK_Z))
        + probes.translate(PROBE_POS.0, PROBE_POS.1, on_base_z(PROBE_NEST_Z))
        + coupons.translate(COUPON_POS.0, COUPON_POS.1, on_base_z(COUPON_PLATE_Z))
        + tokens.translate(TOKEN_POS.0, TOKEN_POS.1, on_base_z(TOKEN_LANE_Z))
        + condensate.translate(CONDENSATE_POS.0, CONDENSATE_POS.1, on_base_z(CONDENSATE_Z))
        + trace.translate(TRACE_POS.0, TRACE_POS.1, on_base_z(TRACE_Z))
        + lanes.translate(LANE_POS.0, LANE_POS.1, on_base_z(LANE_Z))
        + bridge.translate(EVIDENCE_POS.0, EVIDENCE_POS.1, on_base_z(EVIDENCE_BRIDGE_Z))
        + keepouts.translate(0.0, 0.0, BASE_Z / 2.0 + KEEP_OUT_Z / 2.0);
    export(OUTPUTS[10], &assembly);

    println!();
    println!("Closed incubator humidity recovery load challenge station:");
    println!("  Footprint:             {STATION_X:.0}mm x {STATION_Y:.0}mm closed tray");
    println!(
        "  Load rack:             {CASSETTE_SLOT_COUNT} cassette surrogates in a {RACK_COLS}x{RACK_ROWS} rack with {AIR_BYPASS_SLOT_COUNT} airflow bypass witness slots"
    );
    println!(
        "  Measurement:           {PROBE_NEST_COUNT} RH/dewpoint probe nests and {EVIDENCE_CAMERA_COUNT} evidence camera lands"
    );
    println!(
        "  Load challenge:        {WET_COUPON_COUNT} wet coupon wells, {DRY_COUPON_COUNT} dry coupon references, {DOOR_EVENT_TOKEN_COUNT} door-event tokens"
    );
    println!(
        "  Moisture recovery:     {CONDENSATE_CHANNEL_COUNT} condensate channels, {COLLECTION_CUP_COUNT} collection cup lands, {:.0}mL catch capacity",
        condensate_capacity_ml()
    );
    println!(
        "  Trace/disposition:     {BARCODE_LAND_COUNT} barcode lands, {CERTIFICATE_LAND_COUNT} certificate lands, release/hold/reject lanes, and {ROBOT_KEEP_OUT_ZONE_COUNT} keepout zones"
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

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 11);
    assert_eq!(REQUIRED_FEATURES.len(), 10);
    assert_eq!(CASSETTE_SLOT_COUNT, RACK_COLS * RACK_ROWS);
    assert_eq!(CASSETTE_BALLAST_PUCK_COUNT, 4);
    assert_eq!(WET_COUPON_COUNT, DRY_COUPON_COUNT);
    assert_eq!(DISPOSITION_LANE_COUNT, 3);
    assert_eq!(DATUM_TARGET_COUNT, 4);
    assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
    assert!(PROBE_NEST_COUNT >= 4);
    assert!(condensate_capacity_ml() > wet_load_challenge_volume_ml());
    assert!(front_robot_approach_clearance() >= FRONT_ROBOT_APPROACH_CLEARANCE);
    assert!(rear_service_sweep_clearance() >= REAR_SERVICE_SWEEP_CLEARANCE);
    assert!(rack_clear_airflow_fraction() >= 0.14);

    for (name, pos, width, depth) in module_specs() {
        assert!(
            fits_inside_rim(pos, width, depth),
            "{name} exceeds tray rim"
        );
    }
}

fn module_specs() -> [(&'static str, (f64, f64), f64, f64); 8] {
    [
        ("cassette surrogate load rack", RACK_POS, RACK_X, RACK_Y),
        (
            "RH/dewpoint probe nests",
            PROBE_POS,
            PROBE_NEST_X,
            PROBE_NEST_Y,
        ),
        (
            "wet/dry load coupons",
            COUPON_POS,
            COUPON_PLATE_X,
            COUPON_PLATE_Y,
        ),
        (
            "door-event token lane",
            TOKEN_POS,
            TOKEN_LANE_X,
            TOKEN_LANE_Y,
        ),
        (
            "condensate collection features",
            CONDENSATE_POS,
            CONDENSATE_X,
            CONDENSATE_Y,
        ),
        ("barcode/certificate lands", TRACE_POS, TRACE_X, TRACE_Y),
        ("release/hold/reject lanes", LANE_POS, LANE_X, LANE_Y),
        (
            "evidence bridge",
            EVIDENCE_POS,
            EVIDENCE_BRIDGE_X,
            EVIDENCE_BRIDGE_Y,
        ),
    ]
}

fn fits_inside_rim(pos: (f64, f64), width: f64, depth: f64) -> bool {
    pos.0.abs() + width / 2.0 <= STATION_X / 2.0 - RIM_W - 8.0
        && pos.1.abs() + depth / 2.0 <= STATION_Y / 2.0 - RIM_W - 8.0
}

fn front_robot_approach_clearance() -> f64 {
    STATION_Y / 2.0 - (LANE_POS.1.abs() + LANE_Y / 2.0)
}

fn rear_service_sweep_clearance() -> f64 {
    STATION_Y / 2.0 - (EVIDENCE_POS.1 + EVIDENCE_BRIDGE_Y / 2.0)
}

fn condensate_capacity_ml() -> f64 {
    let basin_x = CONDENSATE_X - 78.0;
    let basin_y = CONDENSATE_Y - 54.0;
    basin_x * basin_y * CONDENSATE_BASIN_DEPTH / 1000.0
}

fn wet_load_challenge_volume_ml() -> f64 {
    WET_TROUGH_VOLUME_ML + WET_COUPON_COUNT as f64 * 22.0
}

fn rack_clear_airflow_fraction() -> f64 {
    let slot_area = CASSETTE_SLOT_COUNT as f64 * (CASSETTE_SURROGATE_X + 16.0) * 28.0;
    let bypass_area = AIR_BYPASS_SLOT_COUNT as f64 * 92.0 * 8.0;
    (slot_area + bypass_area) / (RACK_X * RACK_Y)
}

fn base_tray() -> Part {
    let deck = centered_cube(
        "humidity_recovery_load_challenge_base_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let spill_basin = centered_cube(
        "humidity_recovery_load_challenge_secondary_spill_basin_cut",
        STATION_X - 120.0,
        STATION_Y - 116.0,
        8.0,
    )
    .translate(0.0, -10.0, BASE_Z / 2.0 - 3.8);
    let front_drain = centered_cylinder(
        "humidity_recovery_load_challenge_base_front_condensate_drain_bore",
        12.0 / 2.0,
        54.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 86.0, -STATION_Y / 2.0 - 2.0, 0.0);

    deck - spill_basin - front_drain - tray_insert_sockets() - mounting_slots()
        + tray_rims()
        + base_zone_dividers()
        + datum_targets()
        + recovery_flow_witness_ribs()
}

fn tray_insert_sockets() -> Part {
    let mut sockets = Part::empty("humidity_recovery_load_challenge_insert_sockets");
    for (name, pos, width, depth) in module_specs() {
        sockets = sockets
            + centered_cube(
                format!("humidity_recovery_load_challenge_{name}_locator_socket"),
                width + 7.0,
                depth + 7.0,
                SOCKET_DEPTH + 0.6,
            )
            .translate(pos.0, pos.1, BASE_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.3);
    }
    sockets
}

fn mounting_slots() -> Part {
    let mut slots = Part::empty("humidity_recovery_load_challenge_mounting_slots");
    for (i, (x, y)) in mount_slot_positions().into_iter().enumerate() {
        let hole = centered_cylinder(
            format!("humidity_recovery_load_challenge_m6_clearance_{i}"),
            6.8 / 2.0,
            BASE_Z + 4.0,
            24,
        )
        .translate(x, y, 0.0);
        let slot = centered_cube(
            format!("humidity_recovery_load_challenge_m6_service_slot_{i}"),
            28.0,
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
        (-(STATION_X / 2.0 - 60.0), -(STATION_Y / 2.0 - 58.0)),
        (STATION_X / 2.0 - 60.0, -(STATION_Y / 2.0 - 58.0)),
        (-(STATION_X / 2.0 - 60.0), STATION_Y / 2.0 - 58.0),
        (STATION_X / 2.0 - 60.0, STATION_Y / 2.0 - 58.0),
        (0.0, -(STATION_Y / 2.0 - 58.0)),
        (0.0, STATION_Y / 2.0 - 58.0),
        (-(STATION_X / 2.0 - 60.0), 0.0),
        (STATION_X / 2.0 - 60.0, 0.0),
    ]
}

fn tray_rims() -> Part {
    let left = centered_cube(
        "humidity_recovery_load_challenge_left_high_rim",
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
        "humidity_recovery_load_challenge_right_high_rim",
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
        "humidity_recovery_load_challenge_rear_service_rim",
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
        "humidity_recovery_load_challenge_front_robot_low_lip",
        STATION_X - 170.0,
        12.0,
        20.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 22.0, BASE_Z / 2.0 + 10.0);

    left + right + rear + front_low_lip
}

fn base_zone_dividers() -> Part {
    let rear_measurement = centered_cube(
        "humidity_recovery_load_challenge_measurement_row_divider",
        STATION_X - 150.0,
        10.0,
        24.0,
    )
    .translate(0.0, 168.0, BASE_Z / 2.0 + 12.0);
    let load_recovery = centered_cube(
        "humidity_recovery_load_challenge_load_recovery_row_divider",
        STATION_X - 150.0,
        10.0,
        24.0,
    )
    .translate(0.0, -64.0, BASE_Z / 2.0 + 12.0);
    let front_disposition = centered_cube(
        "humidity_recovery_load_challenge_trace_disposition_row_divider",
        STATION_X - 180.0,
        8.0,
        20.0,
    )
    .translate(0.0, -242.0, BASE_Z / 2.0 + 10.0);
    let center_column = centered_cube(
        "humidity_recovery_load_challenge_load_sensor_column_divider",
        10.0,
        548.0,
        24.0,
    )
    .translate(40.0, -6.0, BASE_Z / 2.0 + 12.0);

    rear_measurement + load_recovery + front_disposition + center_column
}

fn datum_targets() -> Part {
    let mut targets = Part::empty("humidity_recovery_load_challenge_robot_datum_targets");
    for (i, (x, y)) in datum_target_points().into_iter().enumerate() {
        targets = targets
            + fiducial_disc(&format!(
                "humidity_recovery_load_challenge_robot_fiducial_{i}"
            ))
            .translate(x, y, BASE_Z / 2.0 + 2.5);
    }
    targets
}

fn datum_target_points() -> [(f64, f64); DATUM_TARGET_COUNT] {
    [
        (-470.0, 308.0),
        (470.0, 308.0),
        (-470.0, -318.0),
        (470.0, -318.0),
    ]
}

fn recovery_flow_witness_ribs() -> Part {
    let mut ribs = Part::empty("humidity_recovery_load_challenge_recovery_flow_witness_ribs");
    for (i, y) in [-204.0, -142.0, -80.0, -18.0, 44.0, 106.0]
        .into_iter()
        .enumerate()
    {
        ribs = ribs
            + centered_cube(
                format!("humidity_recovery_load_challenge_flow_witness_rib_{i}"),
                STATION_X - 190.0,
                4.0,
                5.0,
            )
            .translate(0.0, y, BASE_Z / 2.0 + 2.5);
    }
    ribs
}

fn cassette_surrogate_load_rack() -> Part {
    let deck = centered_cube(
        "humidity_recovery_cassette_surrogate_load_rack_deck",
        RACK_X,
        RACK_Y,
        RACK_Z,
    );
    let lightening = rack_slot_reliefs() + rack_air_bypass_slots() + rack_finger_lift_cutouts();

    deck - lightening
        + rack_side_rails()
        + rack_hard_stops()
        + cassette_surrogates()
        + rack_airflow_witness_vanes()
        + rack_gripper_lands()
}

fn rack_slot_reliefs() -> Part {
    let mut reliefs = Part::empty("humidity_recovery_rack_slot_reliefs");
    for slot in 0..CASSETTE_SLOT_COUNT {
        let (x, y) = rack_slot_center(slot);
        reliefs = reliefs
            + centered_cube(
                format!(
                    "humidity_recovery_slot_{}_cassette_locator_recess",
                    slot_label(slot)
                ),
                CASSETTE_SURROGATE_X + 16.0,
                CASSETTE_SURROGATE_Y + 16.0,
                7.0,
            )
            .translate(x, y, RACK_Z / 2.0 - 3.2);
    }
    reliefs
}

fn rack_air_bypass_slots() -> Part {
    let mut slots = Part::empty("humidity_recovery_rack_air_bypass_slots");
    for i in 0..AIR_BYPASS_SLOT_COUNT {
        let col = i % 4;
        let row = i / 4;
        slots = slots
            + centered_cube(
                format!("humidity_recovery_rack_air_bypass_slot_{i}"),
                92.0,
                8.0,
                RACK_Z + 2.0,
            )
            .translate(
                centered_index(col, 4, 118.0),
                centered_index(row, 2, 246.0),
                0.0,
            );
    }
    slots
}

fn rack_finger_lift_cutouts() -> Part {
    let mut cuts = Part::empty("humidity_recovery_rack_robot_finger_lift_cutouts");
    for slot in 0..CASSETTE_SLOT_COUNT {
        let (x, y) = rack_slot_center(slot);
        cuts = cuts
            + centered_cube(
                format!(
                    "humidity_recovery_slot_{}_front_finger_access",
                    slot_label(slot)
                ),
                60.0,
                18.0,
                RACK_Z + 3.0,
            )
            .translate(x, y - CASSETTE_SURROGATE_Y / 2.0 - 10.0, 0.0);
    }
    cuts
}

fn rack_side_rails() -> Part {
    let left = centered_cube(
        "humidity_recovery_rack_left_reference_rail",
        16.0,
        RACK_Y,
        32.0,
    )
    .translate(-RACK_X / 2.0 + 18.0, 0.0, RACK_Z / 2.0 + 16.0);
    let rear = centered_cube(
        "humidity_recovery_rack_rear_reference_rail",
        RACK_X,
        16.0,
        32.0,
    )
    .translate(0.0, RACK_Y / 2.0 - 18.0, RACK_Z / 2.0 + 16.0);
    let right_soft = centered_cube(
        "humidity_recovery_rack_right_soft_capture_rail",
        12.0,
        RACK_Y - 84.0,
        18.0,
    )
    .translate(RACK_X / 2.0 - 24.0, -10.0, RACK_Z / 2.0 + 9.0);

    left + rear + right_soft
}

fn rack_hard_stops() -> Part {
    let mut stops = Part::empty("humidity_recovery_rack_hard_stops");
    for slot in 0..CASSETTE_SLOT_COUNT {
        let (x, y) = rack_slot_center(slot);
        let rear = centered_cube(
            format!("humidity_recovery_slot_{}_rear_hard_stop", slot_label(slot)),
            CASSETTE_SURROGATE_X * 0.56,
            8.0,
            14.0,
        )
        .translate(x, y + CASSETTE_SURROGATE_Y / 2.0 + 12.0, RACK_Z / 2.0 + 7.0);
        let left = centered_cube(
            format!("humidity_recovery_slot_{}_left_hard_stop", slot_label(slot)),
            8.0,
            CASSETTE_SURROGATE_Y * 0.45,
            14.0,
        )
        .translate(x - CASSETTE_SURROGATE_X / 2.0 - 12.0, y, RACK_Z / 2.0 + 7.0);
        stops = stops + rear + left;
    }
    stops
}

fn cassette_surrogates() -> Part {
    let mut surrogates = Part::empty("humidity_recovery_cassette_surrogates");
    for slot in 0..CASSETTE_SLOT_COUNT {
        let (x, y) = rack_slot_center(slot);
        surrogates = surrogates + cassette_surrogate(slot).translate(x, y, RACK_Z / 2.0 + 2.0);
    }
    surrogates
}

fn cassette_surrogate(slot: usize) -> Part {
    let label = slot_label(slot);
    let body = centered_cube(
        format!("humidity_recovery_{label}_cassette_surrogate_body"),
        CASSETTE_SURROGATE_X,
        CASSETTE_SURROGATE_Y,
        CASSETTE_SURROGATE_Z,
    )
    .translate(0.0, 0.0, CASSETTE_SURROGATE_Z / 2.0);
    let wet_load_land = centered_cube(
        format!("humidity_recovery_{label}_wet_load_reference_land"),
        CASSETTE_SURROGATE_X - 34.0,
        20.0,
        5.0,
    )
    .translate(
        0.0,
        -CASSETTE_SURROGATE_Y / 2.0 + 22.0,
        CASSETTE_SURROGATE_Z + 2.5,
    );
    let dry_load_land = centered_cube(
        format!("humidity_recovery_{label}_dry_load_reference_land"),
        CASSETTE_SURROGATE_X - 34.0,
        20.0,
        5.0,
    )
    .translate(
        0.0,
        CASSETTE_SURROGATE_Y / 2.0 - 22.0,
        CASSETTE_SURROGATE_Z + 2.5,
    );

    body - ballast_puck_reliefs(label) - center_mass_target_cut(label)
        + wet_load_land
        + dry_load_land
        + surrogate_label_land(label)
        + surrogate_edge_guards(label)
}

fn ballast_puck_reliefs(label: &str) -> Part {
    let mut reliefs = Part::empty(format!("humidity_recovery_{label}_ballast_puck_reliefs"));
    for (i, (sx, sy)) in [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)]
        .into_iter()
        .enumerate()
    {
        reliefs = reliefs
            + centered_cylinder(
                format!("humidity_recovery_{label}_ballast_puck_recess_{i}"),
                14.0,
                CASSETTE_SURROGATE_Z + 2.0,
                36,
            )
            .translate(
                sx * (CASSETTE_SURROGATE_X / 2.0 - 28.0),
                sy * (CASSETTE_SURROGATE_Y / 2.0 - 25.0),
                CASSETTE_SURROGATE_Z / 2.0,
            );
    }
    reliefs
}

fn center_mass_target_cut(label: &str) -> Part {
    centered_cylinder(
        format!("humidity_recovery_{label}_center_of_mass_target_cut"),
        11.0,
        4.0,
        32,
    )
    .translate(0.0, 0.0, CASSETTE_SURROGATE_Z - 1.4)
}

fn surrogate_label_land(label: &str) -> Part {
    centered_cube(
        format!("humidity_recovery_{label}_surrogate_serial_label_land"),
        58.0,
        16.0,
        3.0,
    )
    .translate(0.0, 0.0, CASSETTE_SURROGATE_Z + 1.5)
}

fn surrogate_edge_guards(label: &str) -> Part {
    let front = centered_cube(
        format!("humidity_recovery_{label}_front_edge_guard"),
        CASSETTE_SURROGATE_X,
        5.0,
        9.0,
    )
    .translate(
        0.0,
        -CASSETTE_SURROGATE_Y / 2.0 + 2.5,
        CASSETTE_SURROGATE_Z + 4.5,
    );
    let rear = centered_cube(
        format!("humidity_recovery_{label}_rear_edge_guard"),
        CASSETTE_SURROGATE_X,
        5.0,
        9.0,
    )
    .translate(
        0.0,
        CASSETTE_SURROGATE_Y / 2.0 - 2.5,
        CASSETTE_SURROGATE_Z + 4.5,
    );
    front + rear
}

fn rack_airflow_witness_vanes() -> Part {
    let mut vanes = Part::empty("humidity_recovery_rack_airflow_witness_vanes");
    for (i, x) in [-206.0, -72.0, 72.0, 206.0].into_iter().enumerate() {
        vanes = vanes
            + centered_cube(
                format!("humidity_recovery_rack_vertical_airflow_vane_{i}"),
                4.0,
                RACK_Y - 96.0,
                18.0,
            )
            .translate(x, 0.0, RACK_Z / 2.0 + 9.0);
    }
    vanes
}

fn rack_gripper_lands() -> Part {
    let left = centered_cube("humidity_recovery_rack_left_gripper_land", 48.0, 22.0, 8.0)
        .translate(
            -RACK_X / 2.0 + 54.0,
            -RACK_Y / 2.0 + 32.0,
            RACK_Z / 2.0 + 4.0,
        );
    let right = centered_cube("humidity_recovery_rack_right_gripper_land", 48.0, 22.0, 8.0)
        .translate(
            RACK_X / 2.0 - 54.0,
            -RACK_Y / 2.0 + 32.0,
            RACK_Z / 2.0 + 4.0,
        );
    left + right
}

fn rh_dewpoint_probe_nests() -> Part {
    let body = centered_cube(
        "humidity_recovery_rh_dewpoint_probe_nest_plate_body",
        PROBE_NEST_X,
        PROBE_NEST_Y,
        PROBE_NEST_Z,
    );
    let rear_fence = centered_cube(
        "humidity_recovery_probe_nest_rear_cable_fence",
        PROBE_NEST_X,
        14.0,
        PROBE_NEST_Z + 30.0,
    )
    .translate(0.0, PROBE_NEST_Y / 2.0 - 7.0, 15.0);
    let aspirated_shield = centered_cube(
        "humidity_recovery_probe_nest_aspirated_air_shield",
        PROBE_NEST_X - 44.0,
        18.0,
        24.0,
    )
    .translate(0.0, -PROBE_NEST_Y / 2.0 + 24.0, PROBE_NEST_Z / 2.0 + 12.0);

    body + rear_fence + aspirated_shield - probe_nest_cuts()
        + probe_positive_stops()
        + gripper_fiducials("rh_dewpoint_probe_nests")
}

fn probe_nest_cuts() -> Part {
    let mut cuts = Part::empty("humidity_recovery_probe_nest_cuts");
    for i in 0..PROBE_NEST_COUNT {
        let x = centered_index(i, PROBE_NEST_COUNT, PROBE_PITCH_X);
        let label = probe_label(i);
        let sleeve = centered_cylinder(
            format!("humidity_recovery_{label}_probe_sleeve_bore"),
            probe_diameter(i) / 2.0,
            PROBE_NEST_Y + 16.0,
            36,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -3.0, 6.0);
        let top_access = centered_cube(
            format!("humidity_recovery_{label}_probe_top_access_slot"),
            22.0,
            PROBE_NEST_Y - 28.0,
            20.0,
        )
        .translate(x, -8.0, PROBE_NEST_Z / 2.0 - 5.0);
        let cable_saddle = centered_cube(
            format!("humidity_recovery_{label}_probe_cable_saddle"),
            25.0,
            22.0,
            12.0,
        )
        .translate(x, -PROBE_NEST_Y / 2.0 + 14.0, PROBE_NEST_Z / 2.0 - 3.0);
        cuts = cuts + sleeve + top_access + cable_saddle;
    }
    cuts
}

fn probe_positive_stops() -> Part {
    let mut stops = Part::empty("humidity_recovery_probe_positive_stops");
    for i in 0..PROBE_NEST_COUNT {
        let x = centered_index(i, PROBE_NEST_COUNT, PROBE_PITCH_X);
        let label = probe_label(i);
        stops = stops
            + centered_cube(
                format!("humidity_recovery_{label}_probe_tip_depth_stop"),
                30.0,
                6.0,
                16.0,
            )
            .translate(x, PROBE_NEST_Y / 2.0 - 24.0, PROBE_NEST_Z / 2.0 + 8.0)
            + centered_cube(
                format!("humidity_recovery_{label}_probe_serial_flat"),
                52.0,
                16.0,
                3.0,
            )
            .translate(x, -PROBE_NEST_Y / 2.0 + 42.0, PROBE_NEST_Z / 2.0 + 1.5);
    }
    stops
}

fn wet_dry_load_coupons() -> Part {
    let plate = centered_cube(
        "humidity_recovery_wet_dry_load_coupon_plate",
        COUPON_PLATE_X,
        COUPON_PLATE_Y,
        COUPON_PLATE_Z,
    );
    let wet_trough = centered_cube(
        "humidity_recovery_wet_coupon_charge_trough_cut",
        COUPON_PLATE_X - 58.0,
        44.0,
        13.0,
    )
    .translate(0.0, -54.0, COUPON_PLATE_Z / 2.0 - 6.0);
    let dry_reference_pocket = centered_cube(
        "humidity_recovery_dry_coupon_desiccant_reference_pocket_cut",
        COUPON_PLATE_X - 72.0,
        28.0,
        9.0,
    )
    .translate(0.0, 58.0, COUPON_PLATE_Z / 2.0 - 4.0);

    plate - wet_trough - dry_reference_pocket - wet_coupon_well_cuts() - dry_coupon_slot_cuts()
        + coupon_row_labels()
        + wet_coupon_retainer_clips()
        + dry_coupon_retainer_clips()
        + coupon_drip_breaks()
}

fn wet_coupon_well_cuts() -> Part {
    let mut cuts = Part::empty("humidity_recovery_wet_coupon_well_cuts");
    for i in 0..WET_COUPON_COUNT {
        let x = centered_index(i, WET_COUPON_COUNT, COUPON_PITCH_X);
        cuts = cuts
            + centered_cube(
                format!("humidity_recovery_wet_coupon_{i}_wick_well_cut"),
                COUPON_SLOT_X,
                COUPON_SLOT_Y,
                8.0,
            )
            .translate(x, -50.0, COUPON_PLATE_Z / 2.0 - 3.6)
            + centered_cylinder(
                format!("humidity_recovery_wet_coupon_{i}_charge_pipette_port_cut"),
                5.0,
                COUPON_PLATE_Z + 2.0,
                24,
            )
            .translate(x, -78.0, 0.0);
    }
    cuts
}

fn dry_coupon_slot_cuts() -> Part {
    let mut cuts = Part::empty("humidity_recovery_dry_coupon_slot_cuts");
    for i in 0..DRY_COUPON_COUNT {
        let x = centered_index(i, DRY_COUPON_COUNT, COUPON_PITCH_X);
        cuts = cuts
            + centered_cube(
                format!("humidity_recovery_dry_coupon_{i}_reference_slot_cut"),
                COUPON_SLOT_X,
                COUPON_SLOT_Y,
                7.0,
            )
            .translate(x, 56.0, COUPON_PLATE_Z / 2.0 - 3.1);
    }
    cuts
}

fn coupon_row_labels() -> Part {
    let wet = centered_cube(
        "humidity_recovery_wet_coupon_row_label_land",
        COUPON_PLATE_X - 72.0,
        14.0,
        3.0,
    )
    .translate(0.0, -96.0, COUPON_PLATE_Z / 2.0 + 1.5);
    let dry = centered_cube(
        "humidity_recovery_dry_coupon_row_label_land",
        COUPON_PLATE_X - 72.0,
        14.0,
        3.0,
    )
    .translate(0.0, 96.0, COUPON_PLATE_Z / 2.0 + 1.5);
    wet + dry
}

fn wet_coupon_retainer_clips() -> Part {
    let mut clips = Part::empty("humidity_recovery_wet_coupon_retainer_clips");
    for i in 0..WET_COUPON_COUNT {
        let x = centered_index(i, WET_COUPON_COUNT, COUPON_PITCH_X);
        clips = clips
            + coupon_clip(&format!("humidity_recovery_wet_coupon_{i}_left_clip")).translate(
                x - COUPON_SLOT_X / 2.0 + 3.0,
                -50.0,
                COUPON_PLATE_Z / 2.0 + 5.0,
            )
            + coupon_clip(&format!("humidity_recovery_wet_coupon_{i}_right_clip")).translate(
                x + COUPON_SLOT_X / 2.0 - 3.0,
                -50.0,
                COUPON_PLATE_Z / 2.0 + 5.0,
            );
    }
    clips
}

fn dry_coupon_retainer_clips() -> Part {
    let mut clips = Part::empty("humidity_recovery_dry_coupon_retainer_clips");
    for i in 0..DRY_COUPON_COUNT {
        let x = centered_index(i, DRY_COUPON_COUNT, COUPON_PITCH_X);
        clips = clips
            + coupon_clip(&format!("humidity_recovery_dry_coupon_{i}_left_clip")).translate(
                x - COUPON_SLOT_X / 2.0 + 3.0,
                56.0,
                COUPON_PLATE_Z / 2.0 + 5.0,
            )
            + coupon_clip(&format!("humidity_recovery_dry_coupon_{i}_right_clip")).translate(
                x + COUPON_SLOT_X / 2.0 - 3.0,
                56.0,
                COUPON_PLATE_Z / 2.0 + 5.0,
            );
    }
    clips
}

fn coupon_clip(name: &str) -> Part {
    centered_cube(name, 5.0, COUPON_SLOT_Y + 10.0, 10.0)
}

fn coupon_drip_breaks() -> Part {
    let mut breaks = Part::empty("humidity_recovery_coupon_drip_breaks");
    for (i, y) in [-20.0, 18.0].into_iter().enumerate() {
        breaks = breaks
            + centered_cube(
                format!("humidity_recovery_coupon_cross_row_drip_break_{i}"),
                COUPON_PLATE_X - 56.0,
                5.0,
                9.0,
            )
            .translate(0.0, y, COUPON_PLATE_Z / 2.0 + 4.5);
    }
    breaks
}

fn door_event_token_lane() -> Part {
    let plate = centered_cube(
        "humidity_recovery_door_event_token_lane_plate",
        TOKEN_LANE_X,
        TOKEN_LANE_Y,
        TOKEN_LANE_Z,
    );
    let timing_channel = centered_cube(
        "humidity_recovery_door_event_timing_channel_cut",
        TOKEN_LANE_X - 48.0,
        30.0,
        7.0,
    )
    .translate(0.0, 0.0, TOKEN_LANE_Z / 2.0 - 3.0);

    plate - timing_channel - door_token_recesses() + token_lane_rails() + door_event_tick_marks()
}

fn door_token_recesses() -> Part {
    let mut recesses = Part::empty("humidity_recovery_door_event_token_recesses");
    for i in 0..DOOR_EVENT_TOKEN_COUNT {
        let x = centered_index(i, DOOR_EVENT_TOKEN_COUNT, TOKEN_PITCH_X);
        recesses = recesses
            + centered_cylinder(
                format!(
                    "humidity_recovery_door_event_{}_token_recess",
                    door_event_label(i)
                ),
                TOKEN_D / 2.0,
                8.0,
                36,
            )
            .translate(x, 0.0, TOKEN_LANE_Z / 2.0 - 3.2);
    }
    recesses
}

fn token_lane_rails() -> Part {
    let front = centered_cube(
        "humidity_recovery_door_event_token_front_rail",
        TOKEN_LANE_X,
        10.0,
        18.0,
    )
    .translate(0.0, -TOKEN_LANE_Y / 2.0 + 12.0, TOKEN_LANE_Z / 2.0 + 9.0);
    let rear = centered_cube(
        "humidity_recovery_door_event_token_rear_rail",
        TOKEN_LANE_X,
        10.0,
        18.0,
    )
    .translate(0.0, TOKEN_LANE_Y / 2.0 - 12.0, TOKEN_LANE_Z / 2.0 + 9.0);
    let start_gate = centered_cube(
        "humidity_recovery_door_event_start_gate",
        8.0,
        TOKEN_LANE_Y - 26.0,
        22.0,
    )
    .translate(-TOKEN_LANE_X / 2.0 + 34.0, 0.0, TOKEN_LANE_Z / 2.0 + 11.0);
    let end_gate = centered_cube(
        "humidity_recovery_door_event_recovered_gate",
        8.0,
        TOKEN_LANE_Y - 26.0,
        22.0,
    )
    .translate(TOKEN_LANE_X / 2.0 - 34.0, 0.0, TOKEN_LANE_Z / 2.0 + 11.0);

    front + rear + start_gate + end_gate
}

fn door_event_tick_marks() -> Part {
    let mut ticks = Part::empty("humidity_recovery_door_event_tick_marks");
    for i in 0..DOOR_EVENT_TOKEN_COUNT {
        let x = centered_index(i, DOOR_EVENT_TOKEN_COUNT, TOKEN_PITCH_X);
        ticks = ticks
            + centered_cube(
                format!(
                    "humidity_recovery_door_event_{}_tick_land",
                    door_event_label(i)
                ),
                4.0,
                42.0,
                3.0,
            )
            .translate(x, 0.0, TOKEN_LANE_Z / 2.0 + 1.5);
    }
    ticks
}

fn condensate_collection_features() -> Part {
    let tray = centered_cube(
        "humidity_recovery_condensate_collection_tray_body",
        CONDENSATE_X,
        CONDENSATE_Y,
        CONDENSATE_Z,
    );
    let basin = centered_cube(
        "humidity_recovery_condensate_collection_basin_cut",
        CONDENSATE_X - 78.0,
        CONDENSATE_Y - 54.0,
        CONDENSATE_BASIN_DEPTH + 0.6,
    )
    .translate(
        0.0,
        4.0,
        CONDENSATE_Z / 2.0 - CONDENSATE_BASIN_DEPTH / 2.0 + 0.3,
    );
    let drain = centered_cylinder(
        "humidity_recovery_condensate_forward_drain_bore",
        DRAIN_BORE_D / 2.0,
        42.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, -CONDENSATE_Y / 2.0 - 2.0, CONDENSATE_Z / 2.0 - 11.0);

    tray - basin - drain - cup_recess_cuts() - condensate_channel_cuts()
        + condensate_rims()
        + collection_cup_lands()
        + overflow_weir()
        + wicking_strip_lands()
}

fn cup_recess_cuts() -> Part {
    let mut cuts = Part::empty("humidity_recovery_collection_cup_recess_cuts");
    for i in 0..COLLECTION_CUP_COUNT {
        let x = centered_index(i, COLLECTION_CUP_COUNT, 126.0);
        cuts = cuts
            + centered_cylinder(
                format!("humidity_recovery_collection_cup_{i}_recess_cut"),
                24.0,
                9.0,
                42,
            )
            .translate(x, 46.0, CONDENSATE_Z / 2.0 - 3.8);
    }
    cuts
}

fn condensate_channel_cuts() -> Part {
    let mut channels = Part::empty("humidity_recovery_condensate_channel_cuts");
    for i in 0..CONDENSATE_CHANNEL_COUNT {
        let x = centered_index(i, CONDENSATE_CHANNEL_COUNT, 66.0);
        channels = channels
            + centered_cube(
                format!("humidity_recovery_condensate_channel_{i}_cut"),
                8.0,
                CONDENSATE_Y - 72.0,
                6.0,
            )
            .translate(x, -6.0, CONDENSATE_Z / 2.0 - 2.6);
    }
    channels
}

fn condensate_rims() -> Part {
    let front = centered_cube(
        "humidity_recovery_condensate_tray_front_low_lip",
        CONDENSATE_X,
        12.0,
        18.0,
    )
    .translate(0.0, -CONDENSATE_Y / 2.0 + 8.0, CONDENSATE_Z / 2.0 + 9.0);
    let rear = centered_cube(
        "humidity_recovery_condensate_tray_rear_high_lip",
        CONDENSATE_X,
        14.0,
        28.0,
    )
    .translate(0.0, CONDENSATE_Y / 2.0 - 10.0, CONDENSATE_Z / 2.0 + 14.0);
    let left = centered_cube(
        "humidity_recovery_condensate_tray_left_side_lip",
        14.0,
        CONDENSATE_Y,
        24.0,
    )
    .translate(-CONDENSATE_X / 2.0 + 10.0, 0.0, CONDENSATE_Z / 2.0 + 12.0);
    let right = centered_cube(
        "humidity_recovery_condensate_tray_right_side_lip",
        14.0,
        CONDENSATE_Y,
        24.0,
    )
    .translate(CONDENSATE_X / 2.0 - 10.0, 0.0, CONDENSATE_Z / 2.0 + 12.0);
    front + rear + left + right
}

fn collection_cup_lands() -> Part {
    let mut lands = Part::empty("humidity_recovery_collection_cup_lands");
    for i in 0..COLLECTION_CUP_COUNT {
        let x = centered_index(i, COLLECTION_CUP_COUNT, 126.0);
        let ring = centered_cylinder(
            format!("humidity_recovery_collection_cup_{i}_outer_land"),
            CUP_LAND_D / 2.0,
            6.0,
            48,
        )
        .translate(x, 46.0, CONDENSATE_Z / 2.0 + 3.0);
        let center_cut = centered_cylinder(
            format!("humidity_recovery_collection_cup_{i}_inner_witness_cut"),
            24.0,
            7.0,
            42,
        )
        .translate(x, 46.0, CONDENSATE_Z / 2.0 + 3.2);
        let label_land = centered_cube(
            format!("humidity_recovery_collection_cup_{i}_barcode_tick_land"),
            54.0,
            12.0,
            3.0,
        )
        .translate(x, -6.0, CONDENSATE_Z / 2.0 + 1.5);
        lands = lands + (ring - center_cut) + label_land;
    }
    lands
}

fn overflow_weir() -> Part {
    centered_cube(
        "humidity_recovery_condensate_overflow_weir_witness_bar",
        CONDENSATE_X - 82.0,
        7.0,
        22.0,
    )
    .translate(0.0, -CONDENSATE_Y / 2.0 + 34.0, CONDENSATE_Z / 2.0 + 11.0)
}

fn wicking_strip_lands() -> Part {
    let mut lands = Part::empty("humidity_recovery_condensate_wicking_strip_lands");
    for (i, y) in [-44.0, -12.0, 20.0].into_iter().enumerate() {
        lands = lands
            + centered_cube(
                format!("humidity_recovery_condensate_wicking_strip_land_{i}"),
                CONDENSATE_X - 112.0,
                9.0,
                4.0,
            )
            .translate(0.0, y, CONDENSATE_Z / 2.0 + 2.0);
    }
    lands
}

fn barcode_certificate_lands() -> Part {
    let plate = centered_cube(
        "humidity_recovery_barcode_certificate_land_plate",
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    );
    plate + barcode_lands() + certificate_lands() + traceability_fiducials()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("humidity_recovery_barcode_lands");
    for i in 0..BARCODE_LAND_COUNT {
        let col = i % 4;
        let row = i / 4;
        lands = lands
            + centered_cube(
                format!("humidity_recovery_barcode_land_{i}"),
                72.0,
                20.0,
                3.0,
            )
            .translate(
                centered_index(col, 4, 82.0),
                centered_index(row, 2, 32.0) - 12.0,
                TRACE_Z / 2.0 + 1.5,
            );
    }
    lands
}

fn certificate_lands() -> Part {
    let mut lands = Part::empty("humidity_recovery_certificate_lands");
    for i in 0..CERTIFICATE_LAND_COUNT {
        lands = lands
            + centered_cube(
                format!("humidity_recovery_certificate_land_{i}"),
                94.0,
                18.0,
                3.0,
            )
            .translate(
                centered_index(i, CERTIFICATE_LAND_COUNT, 112.0),
                TRACE_Y / 2.0 - 18.0,
                TRACE_Z / 2.0 + 1.5,
            );
    }
    lands
}

fn traceability_fiducials() -> Part {
    let left = fiducial_disc("humidity_recovery_traceability_left_fiducial").translate(
        -TRACE_X / 2.0 + 22.0,
        TRACE_Y / 2.0 - 21.0,
        TRACE_Z / 2.0 + 2.0,
    );
    let right = fiducial_disc("humidity_recovery_traceability_right_fiducial").translate(
        TRACE_X / 2.0 - 22.0,
        TRACE_Y / 2.0 - 21.0,
        TRACE_Z / 2.0 + 2.0,
    );
    left + right
}

fn release_hold_reject_lanes() -> Part {
    let plate = centered_cube(
        "humidity_recovery_release_hold_reject_lane_plate",
        LANE_X,
        LANE_Y,
        LANE_Z,
    );
    plate - lane_floor_recesses() + lane_dividers() + lane_end_stops() + lane_status_pips()
}

fn lane_floor_recesses() -> Part {
    let mut recesses = Part::empty("humidity_recovery_disposition_lane_floor_recesses");
    for i in 0..DISPOSITION_LANE_COUNT {
        recesses = recesses
            + centered_cube(
                format!(
                    "humidity_recovery_{}_lane_floor_recess",
                    disposition_label(i)
                ),
                132.0,
                LANE_Y - 26.0,
                5.0,
            )
            .translate(
                centered_index(i, DISPOSITION_LANE_COUNT, 150.0),
                0.0,
                LANE_Z / 2.0 - 2.0,
            );
    }
    recesses
}

fn lane_dividers() -> Part {
    let left_divider = centered_cube(
        "humidity_recovery_release_hold_lane_divider",
        8.0,
        LANE_Y,
        24.0,
    )
    .translate(-75.0, 0.0, LANE_Z / 2.0 + 12.0);
    let right_divider = centered_cube(
        "humidity_recovery_hold_reject_lane_divider",
        8.0,
        LANE_Y,
        24.0,
    )
    .translate(75.0, 0.0, LANE_Z / 2.0 + 12.0);
    left_divider + right_divider
}

fn lane_end_stops() -> Part {
    let rear = centered_cube(
        "humidity_recovery_disposition_lane_rear_token_stop",
        LANE_X,
        10.0,
        20.0,
    )
    .translate(0.0, LANE_Y / 2.0 - 7.0, LANE_Z / 2.0 + 10.0);
    let front = centered_cube(
        "humidity_recovery_disposition_lane_front_robot_low_stop",
        LANE_X - 64.0,
        8.0,
        12.0,
    )
    .translate(0.0, -LANE_Y / 2.0 + 7.0, LANE_Z / 2.0 + 6.0);
    rear + front
}

fn lane_status_pips() -> Part {
    let mut pips = Part::empty("humidity_recovery_disposition_lane_status_pips");
    for i in 0..DISPOSITION_LANE_COUNT {
        let x = centered_index(i, DISPOSITION_LANE_COUNT, 150.0);
        for pip in 0..3 {
            pips = pips
                + centered_cylinder(
                    format!(
                        "humidity_recovery_{}_lane_status_pip_{pip}",
                        disposition_label(i)
                    ),
                    6.0,
                    4.0,
                    24,
                )
                .translate(x - 24.0 + pip as f64 * 24.0, -24.0, LANE_Z / 2.0 + 2.0);
        }
    }
    pips
}

fn evidence_bridge() -> Part {
    let left_pedestal = centered_cube(
        "humidity_recovery_evidence_bridge_left_pedestal",
        58.0,
        EVIDENCE_BRIDGE_Y,
        EVIDENCE_BRIDGE_Z,
    )
    .translate(-EVIDENCE_BRIDGE_X / 2.0 + 48.0, 0.0, 0.0);
    let right_pedestal = centered_cube(
        "humidity_recovery_evidence_bridge_right_pedestal",
        58.0,
        EVIDENCE_BRIDGE_Y,
        EVIDENCE_BRIDGE_Z,
    )
    .translate(EVIDENCE_BRIDGE_X / 2.0 - 48.0, 0.0, 0.0);
    let crossbar = centered_cube(
        "humidity_recovery_evidence_bridge_camera_crossbar",
        EVIDENCE_BRIDGE_X,
        22.0,
        26.0,
    )
    .translate(0.0, 0.0, EVIDENCE_BRIDGE_Z / 2.0 - 13.0);
    let cable_trough = centered_cube(
        "humidity_recovery_evidence_bridge_rear_cable_trough",
        EVIDENCE_BRIDGE_X - 140.0,
        18.0,
        18.0,
    )
    .translate(
        0.0,
        EVIDENCE_BRIDGE_Y / 2.0 - 14.0,
        EVIDENCE_BRIDGE_Z / 2.0 - 9.0,
    );

    left_pedestal + right_pedestal + crossbar + cable_trough + evidence_camera_lands()
        - evidence_light_pipe_bores()
}

fn evidence_camera_lands() -> Part {
    let mut lands = Part::empty("humidity_recovery_evidence_camera_lands");
    for i in 0..EVIDENCE_CAMERA_COUNT {
        let x = centered_index(i, EVIDENCE_CAMERA_COUNT, 260.0);
        let pad = centered_cube(
            format!("humidity_recovery_evidence_camera_{i}_pad"),
            96.0,
            44.0,
            8.0,
        )
        .translate(x, -8.0, EVIDENCE_BRIDGE_Z / 2.0 + 4.0);
        let lens = centered_cylinder(
            format!("humidity_recovery_evidence_camera_{i}_lens_bore"),
            14.0,
            10.0,
            36,
        )
        .translate(x, -8.0, EVIDENCE_BRIDGE_Z / 2.0 + 4.5);
        lands = lands + (pad - lens);
    }
    lands
}

fn evidence_light_pipe_bores() -> Part {
    let mut bores = Part::empty("humidity_recovery_evidence_light_pipe_bores");
    for i in 0..EVIDENCE_LIGHT_PIPE_COUNT {
        bores = bores
            + centered_cylinder(
                format!("humidity_recovery_evidence_light_pipe_bore_{i}"),
                5.0,
                EVIDENCE_BRIDGE_Y + 8.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(i, EVIDENCE_LIGHT_PIPE_COUNT, 116.0),
                0.0,
                EVIDENCE_BRIDGE_Z / 2.0 - 20.0,
            );
    }
    bores
}

fn robot_service_keepouts() -> Part {
    let outer = centered_cube(
        "humidity_recovery_robot_service_keepout_outer_gauge",
        KEEP_OUT_X,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    );
    let inner = centered_cube(
        "humidity_recovery_robot_service_keepout_open_work_area",
        KEEP_OUT_X - 86.0,
        KEEP_OUT_Y - 100.0,
        KEEP_OUT_Z + 1.0,
    );
    let front_approach = centered_cube(
        "humidity_recovery_front_robot_approach_keepout",
        KEEP_OUT_X - 210.0,
        20.0,
        30.0,
    )
    .translate(0.0, -KEEP_OUT_Y / 2.0 + 46.0, KEEP_OUT_Z / 2.0 + 15.0);
    let rear_service = centered_cube(
        "humidity_recovery_rear_service_sweep_keepout",
        KEEP_OUT_X - 180.0,
        18.0,
        34.0,
    )
    .translate(0.0, KEEP_OUT_Y / 2.0 - 48.0, KEEP_OUT_Z / 2.0 + 17.0);
    let left_grip = centered_cube(
        "humidity_recovery_left_robot_gripper_sweep_keepout",
        24.0,
        KEEP_OUT_Y - 170.0,
        30.0,
    )
    .translate(-KEEP_OUT_X / 2.0 + 56.0, -10.0, KEEP_OUT_Z / 2.0 + 15.0);
    let right_grip = centered_cube(
        "humidity_recovery_right_robot_gripper_sweep_keepout",
        24.0,
        KEEP_OUT_Y - 170.0,
        30.0,
    )
    .translate(KEEP_OUT_X / 2.0 - 56.0, -10.0, KEEP_OUT_Z / 2.0 + 15.0);
    let probe_service = centered_cube(
        "humidity_recovery_probe_service_vertical_clearance_keepout",
        380.0,
        42.0,
        72.0,
    )
    .translate(PROBE_POS.0, PROBE_POS.1 - 84.0, KEEP_OUT_Z / 2.0 + 36.0);

    (outer - inner) + front_approach + rear_service + left_grip + right_grip + probe_service
}

fn gripper_fiducials(prefix: &str) -> Part {
    let left = fiducial_disc(&format!("humidity_recovery_{prefix}_left_gripper_fiducial"))
        .translate(-34.0, 0.0, 4.0);
    let right = fiducial_disc(&format!(
        "humidity_recovery_{prefix}_right_gripper_fiducial"
    ))
    .translate(34.0, 0.0, 4.0);
    left + right
}

fn fiducial_disc(name: &str) -> Part {
    let disc = centered_cylinder(format!("{name}_outer_disc"), 11.0, 5.0, 36);
    let center = centered_cylinder(format!("{name}_center_dot"), 2.2, 6.0, 24);
    let cross_x = centered_cube(format!("{name}_cross_x"), 18.0, 2.4, 6.0);
    let cross_y = centered_cube(format!("{name}_cross_y"), 2.4, 18.0, 6.0);
    disc - center - cross_x - cross_y
}

fn rack_slot_center(slot: usize) -> (f64, f64) {
    let col = slot % RACK_COLS;
    let row = slot / RACK_COLS;
    (
        centered_index(col, RACK_COLS, RACK_SLOT_PITCH_X),
        centered_index(row, RACK_ROWS, RACK_SLOT_PITCH_Y),
    )
}

fn slot_label(slot: usize) -> &'static str {
    match slot {
        0 => "slot_1",
        1 => "slot_2",
        2 => "slot_3",
        3 => "slot_4",
        4 => "slot_5",
        5 => "slot_6",
        _ => "slot_unknown",
    }
}

fn probe_label(index: usize) -> &'static str {
    match index {
        0 => "primary_rh",
        1 => "secondary_rh",
        2 => "chilled_mirror_dewpoint",
        3 => "load_core_rh",
        _ => "door_recovery_rh",
    }
}

fn probe_diameter(index: usize) -> f64 {
    match index {
        0 | 1 => 9.0,
        2 => 14.0,
        3 => 10.0,
        _ => 11.0,
    }
}

fn door_event_label(index: usize) -> &'static str {
    match index {
        0 => "baseline",
        1 => "door_open",
        2 => "door_close",
        3 => "t30s",
        4 => "t60s",
        5 => "t120s",
        6 => "t300s",
        _ => "recovered",
    }
}

fn disposition_label(index: usize) -> &'static str {
    match index {
        0 => "release",
        1 => "hold",
        _ => "reject",
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_stable_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 11);
        for path in OUTPUTS {
            assert!(path
                .starts_with("output/closed_incubator_humidity_recovery_load_challenge_station_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn requested_feature_groups_are_explicit() {
        for feature in [
            "cassette_surrogate_load_rack",
            "rh_dewpoint_probe_nests",
            "wet_dry_load_coupons",
            "door_event_token_lane",
            "condensate_collection_features",
            "barcode_certificate_lands",
            "release_hold_reject_lanes",
            "evidence_bridge",
            "robot_service_keepouts",
            "named_stl_outputs",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn modules_fit_inside_closed_tray_rim() {
        assert_design_constraints();
        for (name, pos, width, depth) in module_specs() {
            assert!(fits_inside_rim(pos, width, depth), "{name} does not fit");
        }
    }

    #[test]
    fn load_rack_and_coupon_counts_match_challenge_plan() {
        assert_eq!(CASSETTE_SLOT_COUNT, 6);
        assert_eq!(RACK_COLS * RACK_ROWS, CASSETTE_SLOT_COUNT);
        assert_eq!(CASSETTE_BALLAST_PUCK_COUNT, 4);
        assert_eq!(WET_COUPON_COUNT, 6);
        assert_eq!(DRY_COUPON_COUNT, 6);
        assert!(rack_clear_airflow_fraction() >= 0.14);
    }

    #[test]
    fn probe_and_door_event_sequences_cover_recovery_measurement() {
        assert_eq!(PROBE_NEST_COUNT, 5);
        assert_eq!(probe_label(2), "chilled_mirror_dewpoint");
        assert_eq!(DOOR_EVENT_TOKEN_COUNT, 8);
        assert_eq!(door_event_label(0), "baseline");
        assert_eq!(door_event_label(DOOR_EVENT_TOKEN_COUNT - 1), "recovered");
    }

    #[test]
    fn condensate_capacity_exceeds_wet_load_challenge_volume() {
        assert!(condensate_capacity_ml() > wet_load_challenge_volume_ml());
        assert_eq!(CONDENSATE_CHANNEL_COUNT, 6);
        assert_eq!(COLLECTION_CUP_COUNT, 3);
        assert!(DRAIN_BORE_D < CUP_LAND_D);
    }

    #[test]
    fn traceability_disposition_and_keepouts_are_complete() {
        assert_eq!(BARCODE_LAND_COUNT, 8);
        assert_eq!(CERTIFICATE_LAND_COUNT, 3);
        assert_eq!(DISPOSITION_LANE_COUNT, 3);
        assert_eq!(EVIDENCE_CAMERA_COUNT, 4);
        assert_eq!(EVIDENCE_LIGHT_PIPE_COUNT, 7);
        assert_eq!(ROBOT_KEEP_OUT_ZONE_COUNT, 5);
        assert!(front_robot_approach_clearance() >= FRONT_ROBOT_APPROACH_CLEARANCE);
        assert!(rear_service_sweep_clearance() >= REAR_SERVICE_SWEEP_CLEARANCE);
    }

    #[test]
    fn output_prefix_matches_all_paths() {
        for path in OUTPUTS {
            assert!(path.contains(OUTPUT_PREFIX));
        }
    }
}
