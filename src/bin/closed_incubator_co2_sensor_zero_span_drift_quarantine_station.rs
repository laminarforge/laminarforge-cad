use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed incubator CO2 sensor zero/span drift quarantine station.
//
// This is mechanical fixture CAD for a closed-system validation station. It
// packages purchased CO2 sensor cartridges, zero/span reference gas ports,
// humidified challenge chamber surrogates, comparison lanes, custody lands,
// disposition tokens, witness pockets, evidence imaging, and access keepouts.
// It does not define a calibration protocol, acceptance limits, analyzer
// traceability program, sterile barrier, or culture-performance claim.

const OUTPUT_PREFIX: &str =
    "output/closed_incubator_co2_sensor_zero_span_drift_quarantine_station_";

const OUTPUTS: [&str; 12] = [
    "output/closed_incubator_co2_sensor_zero_span_drift_quarantine_station_base_leak_tray_deck.stl",
    "output/closed_incubator_co2_sensor_zero_span_drift_quarantine_station_co2_sensor_cartridge_nests.stl",
    "output/closed_incubator_co2_sensor_zero_span_drift_quarantine_station_zero_span_gas_reference_ports.stl",
    "output/closed_incubator_co2_sensor_zero_span_drift_quarantine_station_humidified_challenge_chamber_surrogate.stl",
    "output/closed_incubator_co2_sensor_zero_span_drift_quarantine_station_drift_comparison_lanes.stl",
    "output/closed_incubator_co2_sensor_zero_span_drift_quarantine_station_expired_certificate_custody_lands.stl",
    "output/closed_incubator_co2_sensor_zero_span_drift_quarantine_station_release_hold_reject_quarantine_lanes.stl",
    "output/closed_incubator_co2_sensor_zero_span_drift_quarantine_station_pressure_flow_witness_pockets.stl",
    "output/closed_incubator_co2_sensor_zero_span_drift_quarantine_station_alarm_event_token_rail.stl",
    "output/closed_incubator_co2_sensor_zero_span_drift_quarantine_station_evidence_camera_bridge.stl",
    "output/closed_incubator_co2_sensor_zero_span_drift_quarantine_station_robot_service_keepout_gauges.stl",
    "output/closed_incubator_co2_sensor_zero_span_drift_quarantine_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "base_leak_tray_deck",
    "co2_sensor_cartridge_nests",
    "zero_span_gas_reference_ports",
    "humidified_challenge_chamber_surrogate",
    "drift_comparison_lanes",
    "expired_certificate_custody_lands",
    "release_hold_reject_quarantine_lanes",
    "pressure_flow_witness_pockets",
    "alarm_event_token_rail",
    "evidence_camera_bridge",
    "robot_service_keepout_gauges",
];

const LIMITATIONS: [&str; 6] = [
    "mechanical_fixture_only",
    "no_acceptance_limits",
    "no_calibration_protocol",
    "no_metrology_traceability_claim",
    "no_biological_claim",
    "no_clinical_claim",
];

const STATION_X: f64 = 1360.0;
const STATION_Y: f64 = 860.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const SOCKET_DEPTH: f64 = 6.0;
const MOUNT_HOLE_COUNT: usize = 8;
const DATUM_TARGET_COUNT: usize = 4;

const SENSOR_COUNT: usize = 6;
const BEFORE_AFTER_COLUMNS: usize = 2;
const SENSOR_NEST_COUNT: usize = SENSOR_COUNT * BEFORE_AFTER_COLUMNS;
const SENSOR_NEST_X: f64 = 430.0;
const SENSOR_NEST_Y: f64 = 190.0;
const SENSOR_NEST_Z: f64 = 48.0;
const SENSOR_POS: (f64, f64) = (-395.0, 230.0);
const SENSOR_PITCH_X: f64 = 58.0;
const SENSOR_PITCH_Y: f64 = 72.0;
const CARTRIDGE_POCKET_X: f64 = 46.0;
const CARTRIDGE_POCKET_Y: f64 = 56.0;
const CARTRIDGE_POCKET_Z: f64 = 30.0;
const SENSOR_TUBE_BORE_D: f64 = 5.8;

const GAS_PORT_BANK_X: f64 = 360.0;
const GAS_PORT_BANK_Y: f64 = 190.0;
const GAS_PORT_BANK_Z: f64 = 62.0;
const GAS_PORT_POS: (f64, f64) = (55.0, 230.0);
const ZERO_PORT_COUNT: usize = SENSOR_COUNT;
const SPAN_PORT_COUNT: usize = SENSOR_COUNT;
const GAS_REFERENCE_PORT_COUNT: usize = ZERO_PORT_COUNT + SPAN_PORT_COUNT;
const GAS_PORT_D: f64 = 13.0;
const GAS_PORT_PITCH_X: f64 = 52.0;
const GAS_PORT_PITCH_Y: f64 = 66.0;
const GAS_BULKHEAD_TABS: usize = 4;

const CHAMBER_X: f64 = 330.0;
const CHAMBER_Y: f64 = 190.0;
const CHAMBER_Z: f64 = 92.0;
const CHAMBER_POS: (f64, f64) = (460.0, 230.0);
const HUMIDITY_WELL_COUNT: usize = 4;
const HUMIDITY_WELL_D: f64 = 26.0;
const CHAMBER_WINDOW_COUNT: usize = 3;
const CHAMBER_GASKET_RAIL: f64 = 8.0;

const DRIFT_LANE_X: f64 = 470.0;
const DRIFT_LANE_Y: f64 = 170.0;
const DRIFT_LANE_Z: f64 = 38.0;
const DRIFT_LANE_POS: (f64, f64) = (-405.0, 8.0);
const DRIFT_LANE_COUNT: usize = 3;
const DRIFT_LANE_NAMES: [&str; DRIFT_LANE_COUNT] = ["zero", "span", "humid"];
const DRIFT_SLOT_COUNT_PER_LANE: usize = SENSOR_COUNT;
const DRIFT_SLOT_X: f64 = 46.0;
const DRIFT_SLOT_Y: f64 = 28.0;
const DRIFT_LANE_PITCH_Y: f64 = 46.0;

const CUSTODY_X: f64 = 350.0;
const CUSTODY_Y: f64 = 170.0;
const CUSTODY_Z: f64 = 14.0;
const CUSTODY_POS: (f64, f64) = (70.0, 8.0);
const EXPIRED_LAND_COUNT: usize = 6;
const CERTIFICATE_LAND_COUNT: usize = 6;
const CUSTODY_SEAL_WELL_COUNT: usize = 4;

const DISPOSITION_X: f64 = 340.0;
const DISPOSITION_Y: f64 = 170.0;
const DISPOSITION_Z: f64 = 42.0;
const DISPOSITION_POS: (f64, f64) = (465.0, 8.0);
const DISPOSITION_LANE_COUNT: usize = 4;
const DISPOSITION_LANE_NAMES: [&str; DISPOSITION_LANE_COUNT] =
    ["release", "hold", "reject", "quarantine"];
const DISPOSITION_SLOT_X: f64 = 54.0;
const DISPOSITION_SLOT_Y: f64 = 28.0;
const DISPOSITION_SLOT_COUNT_PER_LANE: usize = 3;

const WITNESS_X: f64 = 470.0;
const WITNESS_Y: f64 = 135.0;
const WITNESS_Z: f64 = 44.0;
const WITNESS_POS: (f64, f64) = (-405.0, -215.0);
const PRESSURE_WITNESS_COUNT: usize = 4;
const FLOW_WITNESS_COUNT: usize = 4;
const WITNESS_POCKET_D: f64 = 28.0;
const WITNESS_TUBE_BORE_D: f64 = 6.35;

const TOKEN_RAIL_X: f64 = 350.0;
const TOKEN_RAIL_Y: f64 = 135.0;
const TOKEN_RAIL_Z: f64 = 34.0;
const TOKEN_RAIL_POS: (f64, f64) = (70.0, -215.0);
const ALARM_EVENT_TOKEN_COUNT: usize = 12;
const TOKEN_D: f64 = 22.0;
const TOKEN_RAIL_PITCH_X: f64 = 48.0;

const CAMERA_BRIDGE_X: f64 = 940.0;
const CAMERA_BRIDGE_Y: f64 = 62.0;
const CAMERA_BRIDGE_Z: f64 = 214.0;
const CAMERA_POS: (f64, f64) = (0.0, 352.0);
const CAMERA_PORT_COUNT: usize = 3;
const LIGHT_RAIL_COUNT: usize = 2;
const CAMERA_CLEARANCE_Z: f64 = 178.0;

const KEEP_OUT_X: f64 = 1260.0;
const KEEP_OUT_Y: f64 = 790.0;
const KEEP_OUT_Z: f64 = 8.0;
const KEEP_OUT_ZONE_COUNT: usize = 5;
const FRONT_ROBOT_CLEARANCE: f64 = 420.0;
const REAR_GAS_SERVICE_CLEARANCE: f64 = 260.0;
const RIGHT_SENSOR_SERVICE_CLEARANCE: f64 = 190.0;
const CAMERA_SERVICE_CLEARANCE: f64 = 180.0;
const CARTRIDGE_LIFT_CLEARANCE_Z: f64 = 150.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_station(self) -> bool {
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

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = base_leak_tray_deck();
    export(OUTPUTS[0], &deck);

    let sensor_nests = co2_sensor_cartridge_nests();
    export(OUTPUTS[1], &sensor_nests);

    let gas_ports = zero_span_gas_reference_ports();
    export(OUTPUTS[2], &gas_ports);

    let chamber = humidified_challenge_chamber_surrogate();
    export(OUTPUTS[3], &chamber);

    let drift_lanes = drift_comparison_lanes();
    export(OUTPUTS[4], &drift_lanes);

    let custody = expired_certificate_custody_lands();
    export(OUTPUTS[5], &custody);

    let disposition = release_hold_reject_quarantine_lanes();
    export(OUTPUTS[6], &disposition);

    let witnesses = pressure_flow_witness_pockets();
    export(OUTPUTS[7], &witnesses);

    let token_rail = alarm_event_token_rail();
    export(OUTPUTS[8], &token_rail);

    let camera = evidence_camera_bridge();
    export(OUTPUTS[9], &camera);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[10], &keepouts);

    let assembly = deck
        + sensor_nests.translate(SENSOR_POS.0, SENSOR_POS.1, on_deck_z(SENSOR_NEST_Z))
        + gas_ports.translate(GAS_PORT_POS.0, GAS_PORT_POS.1, on_deck_z(GAS_PORT_BANK_Z))
        + chamber.translate(CHAMBER_POS.0, CHAMBER_POS.1, on_deck_z(CHAMBER_Z))
        + drift_lanes.translate(DRIFT_LANE_POS.0, DRIFT_LANE_POS.1, on_deck_z(DRIFT_LANE_Z))
        + custody.translate(CUSTODY_POS.0, CUSTODY_POS.1, on_deck_z(CUSTODY_Z))
        + disposition.translate(
            DISPOSITION_POS.0,
            DISPOSITION_POS.1,
            on_deck_z(DISPOSITION_Z),
        )
        + witnesses.translate(WITNESS_POS.0, WITNESS_POS.1, on_deck_z(WITNESS_Z))
        + token_rail.translate(TOKEN_RAIL_POS.0, TOKEN_RAIL_POS.1, on_deck_z(TOKEN_RAIL_Z))
        + camera.translate(CAMERA_POS.0, CAMERA_POS.1, BASE_Z)
        + keepouts.translate(0.0, 0.0, BASE_Z / 2.0 + KEEP_OUT_Z / 2.0);
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed incubator CO2 sensor zero/span drift quarantine station:");
    println!("  Footprint:          {STATION_X:.0}mm x {STATION_Y:.0}mm closed leak-tray deck");
    println!(
        "  Sensor handling:    {SENSOR_COUNT} CO2 sensor cartridges with before/after nests ({SENSOR_NEST_COUNT} pockets)"
    );
    println!(
        "  Gas references:     {ZERO_PORT_COUNT} zero ports, {SPAN_PORT_COUNT} span ports, and {HUMIDITY_WELL_COUNT} humidified chamber wells"
    );
    println!(
        "  Drift comparison:   {DRIFT_LANE_COUNT} comparison lanes ({}) with {DRIFT_SLOT_COUNT_PER_LANE} slots each",
        DRIFT_LANE_NAMES.join(", ")
    );
    println!(
        "  Custody/disposition:{EXPIRED_LAND_COUNT} expired lands, {CERTIFICATE_LAND_COUNT} certificate lands, {} disposition lanes",
        DISPOSITION_LANE_NAMES.join("/")
    );
    println!(
        "  Evidence/access:    {PRESSURE_WITNESS_COUNT} pressure pockets, {FLOW_WITNESS_COUNT} flow pockets, {ALARM_EVENT_TOKEN_COUNT} event tokens, {CAMERA_PORT_COUNT} camera ports, {KEEP_OUT_ZONE_COUNT} keepout zones"
    );
    println!(
        "  Limitations:        mechanical fixture only; no protocol, acceptance limit, traceability, biological, or clinical claim"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn on_deck_z(part_z: f64) -> f64 {
    BASE_Z / 2.0 + part_z / 2.0 - SOCKET_DEPTH / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn module_rects() -> [Rect; 8] {
    [
        rect(
            "co2_sensor_cartridge_nests",
            SENSOR_POS,
            SENSOR_NEST_X,
            SENSOR_NEST_Y,
        ),
        rect(
            "zero_span_gas_reference_ports",
            GAS_PORT_POS,
            GAS_PORT_BANK_X,
            GAS_PORT_BANK_Y,
        ),
        rect(
            "humidified_challenge_chamber_surrogate",
            CHAMBER_POS,
            CHAMBER_X,
            CHAMBER_Y,
        ),
        rect(
            "drift_comparison_lanes",
            DRIFT_LANE_POS,
            DRIFT_LANE_X,
            DRIFT_LANE_Y,
        ),
        rect(
            "expired_certificate_custody_lands",
            CUSTODY_POS,
            CUSTODY_X,
            CUSTODY_Y,
        ),
        rect(
            "release_hold_reject_quarantine_lanes",
            DISPOSITION_POS,
            DISPOSITION_X,
            DISPOSITION_Y,
        ),
        rect(
            "pressure_flow_witness_pockets",
            WITNESS_POS,
            WITNESS_X,
            WITNESS_Y,
        ),
        rect(
            "alarm_event_token_rail",
            TOKEN_RAIL_POS,
            TOKEN_RAIL_X,
            TOKEN_RAIL_Y,
        ),
    ]
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 12);
    assert_eq!(REQUIRED_FEATURES.len(), 11);
    assert!(OUTPUTS
        .iter()
        .all(|path| path.starts_with(OUTPUT_PREFIX) && path.ends_with(".stl")));
    for limitation in [
        "mechanical_fixture_only",
        "no_acceptance_limits",
        "no_calibration_protocol",
        "no_metrology_traceability_claim",
        "no_biological_claim",
        "no_clinical_claim",
    ] {
        assert!(LIMITATIONS.contains(&limitation));
    }
    assert_eq!(ZERO_PORT_COUNT + SPAN_PORT_COUNT, GAS_REFERENCE_PORT_COUNT);
    assert_eq!(SENSOR_NEST_COUNT, SENSOR_COUNT * BEFORE_AFTER_COLUMNS);
    assert_eq!(DRIFT_SLOT_COUNT_PER_LANE, SENSOR_COUNT);
    assert_eq!(DRIFT_LANE_NAMES.len(), DRIFT_LANE_COUNT);
    assert_eq!(DISPOSITION_LANE_NAMES.len(), DISPOSITION_LANE_COUNT);
    assert_eq!(EXPIRED_LAND_COUNT, CERTIFICATE_LAND_COUNT);
    assert_eq!(
        PRESSURE_WITNESS_COUNT + FLOW_WITNESS_COUNT,
        GAS_BULKHEAD_TABS * 2
    );
    assert_eq!(CAMERA_PORT_COUNT, 3);
    assert_eq!(LIGHT_RAIL_COUNT, 2);
    assert_eq!(KEEP_OUT_ZONE_COUNT, 5);
    assert!(GAS_PORT_D > SENSOR_TUBE_BORE_D);
    assert!(WITNESS_TUBE_BORE_D > SENSOR_TUBE_BORE_D);
    assert!(CHAMBER_Z > SENSOR_NEST_Z);
    assert!(CAMERA_CLEARANCE_Z > CARTRIDGE_LIFT_CLEARANCE_Z);
    assert!(FRONT_ROBOT_CLEARANCE >= 400.0);
    assert!(REAR_GAS_SERVICE_CLEARANCE >= 250.0);
    assert!(RIGHT_SENSOR_SERVICE_CLEARANCE >= 180.0);

    for module in module_rects() {
        assert!(
            module.fits_inside_station(),
            "{} exceeds deck rim",
            module.name
        );
    }

    let rects = module_rects();
    for (index, a) in rects.iter().enumerate() {
        for b in rects.iter().skip(index + 1) {
            assert!(!a.overlaps(*b), "{} overlaps {}", a.name, b.name);
        }
    }
}

fn base_leak_tray_deck() -> Part {
    let deck = centered_cube(
        "co2_sensor_zero_span_station_base_leak_tray_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let basin_cut = centered_cube(
        "co2_sensor_zero_span_station_recessed_leak_basin_cut",
        STATION_X - 2.0 * (RIM_W + 52.0),
        STATION_Y - 2.0 * (RIM_W + 50.0),
        8.0,
    )
    .translate(0.0, -8.0, BASE_Z - 4.0);
    let front_drain_gutter = centered_cube(
        "co2_sensor_zero_span_station_front_low_point_gutter",
        STATION_X - 190.0,
        20.0,
        9.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 58.0, BASE_Z - 4.5);
    let drain_port = centered_cylinder(
        "co2_sensor_zero_span_station_closed_tray_drain_port",
        8.0,
        54.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 86.0,
        -STATION_Y / 2.0 + 36.0,
        BASE_Z - 8.0,
    );

    deck - basin_cut - front_drain_gutter - drain_port - insert_sockets() - mounting_holes()
        + perimeter_rims()
        + zone_dividers()
        + robot_datum_targets()
        + camera_anchor_lands()
        + closed_system_label_land()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("co2_sensor_zero_span_station_insert_sockets");
    for module in module_rects() {
        sockets = sockets
            + centered_cube(
                format!("co2_sensor_zero_span_station_{}_socket", module.name),
                module.x + 10.0,
                module.y + 10.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(
                module.center.0,
                module.center.1,
                BASE_Z - SOCKET_DEPTH / 2.0 + 0.2,
            );
    }
    sockets
}

fn mounting_holes() -> Part {
    let mut holes = Part::empty("co2_sensor_zero_span_station_mounting_holes");
    for (i, (x, y)) in mount_hole_positions().iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("co2_sensor_zero_span_station_m6_clearance_{i}"),
                3.4,
                BASE_Z + 4.0,
                28,
            )
            .translate(*x, *y, BASE_Z / 2.0);
    }
    holes
}

fn mount_hole_positions() -> [(f64, f64); MOUNT_HOLE_COUNT] {
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
        "co2_sensor_zero_span_station_front_spill_rim",
        STATION_X,
        RIM_W,
        28.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, BASE_Z + 14.0);
    let rear = centered_cube(
        "co2_sensor_zero_span_station_rear_gas_service_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "co2_sensor_zero_span_station_left_quarantine_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "co2_sensor_zero_span_station_right_sensor_service_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);

    front + rear + left + right
}

fn zone_dividers() -> Part {
    let upper_workflow = centered_cube(
        "co2_sensor_zero_span_station_sensor_gas_chamber_row_divider",
        STATION_X - 190.0,
        10.0,
        26.0,
    )
    .translate(0.0, 118.0, BASE_Z + 13.0);
    let lower_workflow = centered_cube(
        "co2_sensor_zero_span_station_drift_disposition_row_divider",
        STATION_X - 210.0,
        10.0,
        26.0,
    )
    .translate(0.0, -112.0, BASE_Z + 13.0);
    let gas_custody_column = centered_cube(
        "co2_sensor_zero_span_station_gas_to_custody_column_divider",
        10.0,
        360.0,
        24.0,
    )
    .translate(265.0, 8.0, BASE_Z + 12.0);
    let sensor_gas_column = centered_cube(
        "co2_sensor_zero_span_station_sensor_to_gas_reference_column_divider",
        10.0,
        190.0,
        24.0,
    )
    .translate(-170.0, 230.0, BASE_Z + 12.0);

    upper_workflow + lower_workflow + gas_custody_column + sensor_gas_column
}

fn robot_datum_targets() -> Part {
    let mut targets = Part::empty("co2_sensor_zero_span_station_robot_datum_targets");
    for (i, (x, y)) in datum_target_positions().iter().enumerate() {
        targets = targets
            + fiducial_disc(&format!(
                "co2_sensor_zero_span_station_robot_datum_target_{i}"
            ))
            .translate(*x, *y, BASE_Z + 2.0);
    }
    targets
}

fn datum_target_positions() -> [(f64, f64); DATUM_TARGET_COUNT] {
    [
        (-STATION_X / 2.0 + 92.0, -STATION_Y / 2.0 + 92.0),
        (STATION_X / 2.0 - 92.0, -STATION_Y / 2.0 + 92.0),
        (-STATION_X / 2.0 + 92.0, STATION_Y / 2.0 - 92.0),
        (STATION_X / 2.0 - 92.0, STATION_Y / 2.0 - 92.0),
    ]
}

fn camera_anchor_lands() -> Part {
    let left = centered_cube(
        "co2_sensor_zero_span_station_camera_bridge_left_anchor_land",
        96.0,
        40.0,
        8.0,
    )
    .translate(
        CAMERA_POS.0 - CAMERA_BRIDGE_X / 2.0 + 58.0,
        CAMERA_POS.1,
        BASE_Z + 4.0,
    );
    let right = centered_cube(
        "co2_sensor_zero_span_station_camera_bridge_right_anchor_land",
        96.0,
        40.0,
        8.0,
    )
    .translate(
        CAMERA_POS.0 + CAMERA_BRIDGE_X / 2.0 - 58.0,
        CAMERA_POS.1,
        BASE_Z + 4.0,
    );

    left + right
}

fn closed_system_label_land() -> Part {
    centered_cube(
        "co2_sensor_zero_span_station_closed_system_validation_label_land",
        300.0,
        24.0,
        3.0,
    )
    .translate(
        -STATION_X / 2.0 + 230.0,
        STATION_Y / 2.0 - 62.0,
        BASE_Z + 1.5,
    )
}

fn co2_sensor_cartridge_nests() -> Part {
    let carrier = centered_cube(
        "co2_sensor_zero_span_station_sensor_nest_carrier",
        SENSOR_NEST_X,
        SENSOR_NEST_Y,
        SENSOR_NEST_Z,
    );
    let rear_fence = centered_cube(
        "co2_sensor_zero_span_station_sensor_nest_rear_connector_fence",
        SENSOR_NEST_X,
        14.0,
        SENSOR_NEST_Z + 30.0,
    )
    .translate(0.0, SENSOR_NEST_Y / 2.0 - 7.0, 15.0);

    let mut cuts = Part::empty("co2_sensor_zero_span_station_sensor_nest_cuts");
    let mut lands = Part::empty("co2_sensor_zero_span_station_sensor_nest_lands");
    let mut keys = Part::empty("co2_sensor_zero_span_station_sensor_nest_datum_keys");

    for sensor in 0..SENSOR_COUNT {
        let x = centered_index(sensor, SENSOR_COUNT, SENSOR_PITCH_X);
        for column in 0..BEFORE_AFTER_COLUMNS {
            let y = centered_index(column, BEFORE_AFTER_COLUMNS, SENSOR_PITCH_Y);
            let label = if column == 0 { "pre" } else { "post" };
            cuts =
                cuts + centered_cube(
                    format!(
                        "co2_sensor_zero_span_station_sensor_{sensor}_{label}_cartridge_pocket"
                    ),
                    CARTRIDGE_POCKET_X,
                    CARTRIDGE_POCKET_Y,
                    CARTRIDGE_POCKET_Z,
                )
                .translate(
                    x,
                    y,
                    SENSOR_NEST_Z / 2.0 - CARTRIDGE_POCKET_Z / 2.0 + 4.0,
                ) + centered_cylinder(
                    format!("co2_sensor_zero_span_station_sensor_{sensor}_{label}_gas_tube_bore"),
                    SENSOR_TUBE_BORE_D / 2.0,
                    SENSOR_NEST_Y + 18.0,
                    24,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(x, y, 7.0);

            lands = lands
                + centered_cube(
                    format!("co2_sensor_zero_span_station_sensor_{sensor}_{label}_serial_land"),
                    42.0,
                    13.0,
                    3.0,
                )
                .translate(x, y - 24.0, SENSOR_NEST_Z / 2.0 + 1.5);
        }

        keys = keys
            + centered_cube(
                format!("co2_sensor_zero_span_station_sensor_{sensor}_left_datum_spring"),
                6.0,
                SENSOR_NEST_Y - 24.0,
                18.0,
            )
            .translate(x - CARTRIDGE_POCKET_X / 2.0 - 8.0, 0.0, -8.0)
            + centered_cube(
                format!("co2_sensor_zero_span_station_sensor_{sensor}_right_soft_stop"),
                6.0,
                SENSOR_NEST_Y - 24.0,
                18.0,
            )
            .translate(x + CARTRIDGE_POCKET_X / 2.0 + 8.0, 0.0, -8.0);
    }

    (carrier - cuts) + rear_fence + lands + keys + gripper_fiducials("sensor_nests", 330.0)
}

fn zero_span_gas_reference_ports() -> Part {
    let panel = centered_cube(
        "co2_sensor_zero_span_station_zero_span_gas_reference_panel",
        GAS_PORT_BANK_X,
        GAS_PORT_BANK_Y,
        GAS_PORT_BANK_Z,
    );
    let front_label = centered_cube(
        "co2_sensor_zero_span_station_zero_span_front_label_land",
        GAS_PORT_BANK_X - 42.0,
        18.0,
        4.0,
    )
    .translate(
        0.0,
        -GAS_PORT_BANK_Y / 2.0 + 18.0,
        GAS_PORT_BANK_Z / 2.0 + 2.0,
    );

    let mut port_cuts = Part::empty("co2_sensor_zero_span_station_zero_span_port_cuts");
    let mut seals = Part::empty("co2_sensor_zero_span_station_zero_span_port_seal_lands");
    for row in 0..BEFORE_AFTER_COLUMNS {
        let y = centered_index(row, BEFORE_AFTER_COLUMNS, GAS_PORT_PITCH_Y);
        let gas_name = if row == 0 { "zero" } else { "span" };
        for port in 0..SENSOR_COUNT {
            let x = centered_index(port, SENSOR_COUNT, GAS_PORT_PITCH_X);
            port_cuts = port_cuts
                + centered_cylinder(
                    format!("co2_sensor_zero_span_station_{gas_name}_reference_port_{port}"),
                    GAS_PORT_D / 2.0,
                    GAS_PORT_BANK_Z + 8.0,
                    32,
                )
                .translate(x, y, 0.0)
                + centered_cylinder(
                    format!("co2_sensor_zero_span_station_{gas_name}_tube_relief_{port}"),
                    SENSOR_TUBE_BORE_D / 2.0,
                    GAS_PORT_BANK_Y + 18.0,
                    24,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(x, y, -12.0);

            seals = seals
                + centered_cylinder(
                    format!("co2_sensor_zero_span_station_{gas_name}_port_{port}_o_ring_land"),
                    GAS_PORT_D / 2.0 + 5.0,
                    3.0,
                    36,
                )
                .translate(x, y, GAS_PORT_BANK_Z / 2.0 + 1.5);
        }
    }

    panel - port_cuts + seals + front_label + gas_bulkhead_tabs()
}

fn gas_bulkhead_tabs() -> Part {
    let mut tabs = Part::empty("co2_sensor_zero_span_station_gas_bulkhead_tabs");
    for (i, x) in [-132.0, -44.0, 44.0, 132.0].iter().enumerate() {
        let tab = centered_cube(
            format!("co2_sensor_zero_span_station_gas_bulkhead_tab_{i}"),
            52.0,
            16.0,
            28.0,
        )
        .translate(*x, GAS_PORT_BANK_Y / 2.0 - 8.0, 12.0);
        let bore = centered_cylinder(
            format!("co2_sensor_zero_span_station_gas_bulkhead_tab_{i}_tube_bore"),
            WITNESS_TUBE_BORE_D / 2.0,
            22.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, GAS_PORT_BANK_Y / 2.0 - 8.0, 12.0);
        tabs = tabs + (tab - bore);
    }
    tabs
}

fn humidified_challenge_chamber_surrogate() -> Part {
    let base = centered_cube(
        "co2_sensor_zero_span_station_humidified_challenge_chamber_base",
        CHAMBER_X,
        CHAMBER_Y,
        CHAMBER_Z,
    );
    let chamber_void = centered_cube(
        "co2_sensor_zero_span_station_humidified_challenge_chamber_void",
        CHAMBER_X - 54.0,
        CHAMBER_Y - 54.0,
        CHAMBER_Z - 28.0,
    )
    .translate(0.0, 0.0, 10.0);
    let lid_window = centered_cube(
        "co2_sensor_zero_span_station_humidified_challenge_lid_window_cut",
        CHAMBER_X - 94.0,
        CHAMBER_Y - 92.0,
        12.0,
    )
    .translate(0.0, 0.0, CHAMBER_Z / 2.0 - 4.0);

    let mut wells = Part::empty("co2_sensor_zero_span_station_humidity_well_cuts");
    for i in 0..HUMIDITY_WELL_COUNT {
        let x = centered_index(i, HUMIDITY_WELL_COUNT, 58.0);
        wells = wells
            + centered_cylinder(
                format!("co2_sensor_zero_span_station_humidity_source_well_{i}"),
                HUMIDITY_WELL_D / 2.0,
                30.0,
                32,
            )
            .translate(x, -CHAMBER_Y / 2.0 + 42.0, CHAMBER_Z / 2.0 - 12.0);
    }

    let mut windows = Part::empty("co2_sensor_zero_span_station_chamber_window_frames");
    for i in 0..CHAMBER_WINDOW_COUNT {
        let x = centered_index(i, CHAMBER_WINDOW_COUNT, 88.0);
        windows = windows
            + rectangular_frame(
                &format!("co2_sensor_zero_span_station_chamber_view_window_{i}"),
                56.0,
                34.0,
                4.0,
                4.0,
            )
            .translate(x, CHAMBER_Y / 2.0 + 3.0, 24.0);
    }

    base - chamber_void - lid_window - wells
        + rectangular_frame(
            "co2_sensor_zero_span_station_humidified_chamber_gasket_land",
            CHAMBER_X - 42.0,
            CHAMBER_Y - 42.0,
            6.0,
            CHAMBER_GASKET_RAIL,
        )
        .translate(0.0, 0.0, CHAMBER_Z / 2.0 + 3.0)
        + windows
        + gripper_fiducials("humidified_challenge_chamber", 220.0)
}

fn drift_comparison_lanes() -> Part {
    let tray = centered_cube(
        "co2_sensor_zero_span_station_drift_comparison_lane_tray",
        DRIFT_LANE_X,
        DRIFT_LANE_Y,
        DRIFT_LANE_Z,
    );
    let mut cuts = Part::empty("co2_sensor_zero_span_station_drift_lane_slot_cuts");
    let mut rails = Part::empty("co2_sensor_zero_span_station_drift_lane_rails");
    let mut labels = Part::empty("co2_sensor_zero_span_station_drift_lane_labels");

    for lane in 0..DRIFT_LANE_COUNT {
        let y = centered_index(lane, DRIFT_LANE_COUNT, DRIFT_LANE_PITCH_Y);
        let lane_name = DRIFT_LANE_NAMES[lane];
        rails = rails
            + centered_cube(
                format!("co2_sensor_zero_span_station_{lane_name}_drift_lane_left_rail"),
                DRIFT_LANE_X - 32.0,
                4.0,
                14.0,
            )
            .translate(0.0, y - DRIFT_SLOT_Y / 2.0 - 7.0, DRIFT_LANE_Z / 2.0 + 7.0)
            + centered_cube(
                format!("co2_sensor_zero_span_station_{lane_name}_drift_lane_right_rail"),
                DRIFT_LANE_X - 32.0,
                4.0,
                14.0,
            )
            .translate(0.0, y + DRIFT_SLOT_Y / 2.0 + 7.0, DRIFT_LANE_Z / 2.0 + 7.0);

        labels = labels
            + centered_cube(
                format!("co2_sensor_zero_span_station_{lane_name}_drift_lane_status_land"),
                60.0,
                18.0,
                3.0,
            )
            .translate(-DRIFT_LANE_X / 2.0 + 42.0, y, DRIFT_LANE_Z / 2.0 + 1.5);

        for slot in 0..DRIFT_SLOT_COUNT_PER_LANE {
            let x = centered_index(slot, DRIFT_SLOT_COUNT_PER_LANE, 58.0) + 34.0;
            cuts = cuts
                + centered_cube(
                    format!("co2_sensor_zero_span_station_{lane_name}_drift_slot_{slot}"),
                    DRIFT_SLOT_X,
                    DRIFT_SLOT_Y,
                    16.0,
                )
                .translate(x, y, DRIFT_LANE_Z / 2.0 - 8.0);
        }
    }

    tray - cuts + rails + labels + gripper_fiducials("drift_comparison_lanes", 360.0)
}

fn expired_certificate_custody_lands() -> Part {
    let plate = centered_cube(
        "co2_sensor_zero_span_station_expired_certificate_custody_plate",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    let mut lands = Part::empty("co2_sensor_zero_span_station_custody_lands");

    for i in 0..EXPIRED_LAND_COUNT {
        let x = centered_index(i, EXPIRED_LAND_COUNT, 48.0);
        lands = lands
            + centered_cube(
                format!("co2_sensor_zero_span_station_expired_sensor_or_gas_land_{i}"),
                40.0,
                28.0,
                3.0,
            )
            .translate(x, 38.0, CUSTODY_Z / 2.0 + 1.5);
    }

    for i in 0..CERTIFICATE_LAND_COUNT {
        let x = centered_index(i, CERTIFICATE_LAND_COUNT, 48.0);
        lands = lands
            + centered_cube(
                format!("co2_sensor_zero_span_station_calibration_certificate_land_{i}"),
                40.0,
                28.0,
                3.0,
            )
            .translate(x, -8.0, CUSTODY_Z / 2.0 + 1.5);
    }

    for i in 0..CUSTODY_SEAL_WELL_COUNT {
        let x = centered_index(i, CUSTODY_SEAL_WELL_COUNT, 76.0);
        lands = lands
            + centered_cylinder(
                format!("co2_sensor_zero_span_station_custody_tamper_seal_well_{i}"),
                12.0,
                4.0,
                28,
            )
            .translate(x, -62.0, CUSTODY_Z / 2.0 + 2.0);
    }

    plate + lands + gripper_fiducials("custody_lands", 250.0)
}

fn release_hold_reject_quarantine_lanes() -> Part {
    let tray = centered_cube(
        "co2_sensor_zero_span_station_release_hold_reject_quarantine_lane_tray",
        DISPOSITION_X,
        DISPOSITION_Y,
        DISPOSITION_Z,
    );
    let mut cuts = Part::empty("co2_sensor_zero_span_station_disposition_slot_cuts");
    let mut fences = Part::empty("co2_sensor_zero_span_station_disposition_lane_fences");
    let mut status_lands = Part::empty("co2_sensor_zero_span_station_disposition_status_lands");

    for lane in 0..DISPOSITION_LANE_COUNT {
        let y = centered_index(lane, DISPOSITION_LANE_COUNT, 36.0);
        let name = DISPOSITION_LANE_NAMES[lane];
        fences = fences
            + centered_cube(
                format!("co2_sensor_zero_span_station_{name}_lane_front_fence"),
                DISPOSITION_X - 36.0,
                4.0,
                18.0,
            )
            .translate(
                0.0,
                y - DISPOSITION_SLOT_Y / 2.0 - 6.0,
                DISPOSITION_Z / 2.0 + 9.0,
            );

        status_lands = status_lands
            + centered_cube(
                format!("co2_sensor_zero_span_station_{name}_lane_label_land"),
                76.0,
                18.0,
                3.0,
            )
            .translate(-DISPOSITION_X / 2.0 + 52.0, y, DISPOSITION_Z / 2.0 + 1.5);

        for slot in 0..DISPOSITION_SLOT_COUNT_PER_LANE {
            let x = -20.0 + centered_index(slot, DISPOSITION_SLOT_COUNT_PER_LANE, 70.0);
            cuts = cuts
                + centered_cube(
                    format!("co2_sensor_zero_span_station_{name}_decision_token_slot_{slot}"),
                    DISPOSITION_SLOT_X,
                    DISPOSITION_SLOT_Y,
                    18.0,
                )
                .translate(x, y, DISPOSITION_Z / 2.0 - 9.0);
        }
    }

    tray - cuts + fences + status_lands + gripper_fiducials("disposition_lanes", 250.0)
}

fn pressure_flow_witness_pockets() -> Part {
    let carrier = centered_cube(
        "co2_sensor_zero_span_station_pressure_flow_witness_carrier",
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    );
    let mut cuts = Part::empty("co2_sensor_zero_span_station_pressure_flow_witness_cuts");
    let mut collars = Part::empty("co2_sensor_zero_span_station_pressure_flow_witness_collars");

    for i in 0..PRESSURE_WITNESS_COUNT {
        let x = centered_index(i, PRESSURE_WITNESS_COUNT, 78.0) - 40.0;
        cuts = cuts
            + centered_cylinder(
                format!("co2_sensor_zero_span_station_pressure_witness_pocket_{i}"),
                WITNESS_POCKET_D / 2.0,
                WITNESS_Z + 6.0,
                32,
            )
            .translate(x, 28.0, 0.0);
        collars = collars
            + centered_cylinder(
                format!("co2_sensor_zero_span_station_pressure_witness_collared_land_{i}"),
                WITNESS_POCKET_D / 2.0 + 5.0,
                3.0,
                32,
            )
            .translate(x, 28.0, WITNESS_Z / 2.0 + 1.5);
    }

    for i in 0..FLOW_WITNESS_COUNT {
        let x = centered_index(i, FLOW_WITNESS_COUNT, 78.0) - 40.0;
        cuts = cuts
            + centered_cylinder(
                format!("co2_sensor_zero_span_station_flow_witness_pocket_{i}"),
                WITNESS_POCKET_D / 2.0,
                WITNESS_Z + 6.0,
                32,
            )
            .translate(x, -28.0, 0.0)
            + centered_cylinder(
                format!("co2_sensor_zero_span_station_flow_witness_tube_bore_{i}"),
                WITNESS_TUBE_BORE_D / 2.0,
                WITNESS_Y + 14.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -28.0, -10.0);
        collars = collars
            + centered_cylinder(
                format!("co2_sensor_zero_span_station_flow_witness_collared_land_{i}"),
                WITNESS_POCKET_D / 2.0 + 5.0,
                3.0,
                32,
            )
            .translate(x, -28.0, WITNESS_Z / 2.0 + 1.5);
    }

    carrier - cuts + collars + gripper_fiducials("pressure_flow_witness", 360.0)
}

fn alarm_event_token_rail() -> Part {
    let rail = centered_cube(
        "co2_sensor_zero_span_station_alarm_event_token_rail_body",
        TOKEN_RAIL_X,
        TOKEN_RAIL_Y,
        TOKEN_RAIL_Z,
    );
    let mut token_cuts = Part::empty("co2_sensor_zero_span_station_alarm_event_token_cuts");
    let mut lane_guides = Part::empty("co2_sensor_zero_span_station_alarm_event_lane_guides");

    for i in 0..ALARM_EVENT_TOKEN_COUNT {
        let x = centered_index(i % 6, 6, TOKEN_RAIL_PITCH_X);
        let y = centered_index(i / 6, 2, 52.0);
        token_cuts = token_cuts
            + centered_cylinder(
                format!("co2_sensor_zero_span_station_alarm_event_token_socket_{i}"),
                TOKEN_D / 2.0,
                TOKEN_RAIL_Z + 4.0,
                32,
            )
            .translate(x, y, 0.0);
    }

    for y in [-40.0, 0.0, 40.0] {
        lane_guides = lane_guides
            + centered_cube(
                format!("co2_sensor_zero_span_station_alarm_event_token_lane_guide_{y:.0}"),
                TOKEN_RAIL_X - 36.0,
                3.0,
                10.0,
            )
            .translate(0.0, y, TOKEN_RAIL_Z / 2.0 + 5.0);
    }

    rail - token_cuts + lane_guides + gripper_fiducials("alarm_event_token_rail", 250.0)
}

fn evidence_camera_bridge() -> Part {
    let left_post = camera_bridge_post("left").translate(
        -CAMERA_BRIDGE_X / 2.0 + 36.0,
        0.0,
        CAMERA_BRIDGE_Z / 2.0,
    );
    let right_post = camera_bridge_post("right").translate(
        CAMERA_BRIDGE_X / 2.0 - 36.0,
        0.0,
        CAMERA_BRIDGE_Z / 2.0,
    );
    let beam = centered_cube(
        "co2_sensor_zero_span_station_evidence_camera_bridge_top_beam",
        CAMERA_BRIDGE_X,
        CAMERA_BRIDGE_Y,
        28.0,
    )
    .translate(0.0, 0.0, CAMERA_BRIDGE_Z - 14.0);
    let sled = centered_cube(
        "co2_sensor_zero_span_station_evidence_camera_sled",
        260.0,
        CAMERA_BRIDGE_Y + 12.0,
        18.0,
    )
    .translate(0.0, 0.0, CAMERA_BRIDGE_Z - 50.0);
    let mut camera_ports = Part::empty("co2_sensor_zero_span_station_evidence_camera_ports");
    for i in 0..CAMERA_PORT_COUNT {
        let x = centered_index(i, CAMERA_PORT_COUNT, 90.0);
        camera_ports = camera_ports
            + centered_cylinder(
                format!("co2_sensor_zero_span_station_camera_lens_port_{i}"),
                15.0,
                CAMERA_BRIDGE_Y + 22.0,
                36,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, CAMERA_BRIDGE_Z - 50.0);
    }
    let front_light = centered_cube(
        "co2_sensor_zero_span_station_evidence_front_light_rail",
        CAMERA_BRIDGE_X - 140.0,
        10.0,
        12.0,
    )
    .translate(0.0, -CAMERA_BRIDGE_Y / 2.0 - 10.0, CAMERA_BRIDGE_Z - 72.0);
    let rear_light = centered_cube(
        "co2_sensor_zero_span_station_evidence_rear_light_rail",
        CAMERA_BRIDGE_X - 140.0,
        10.0,
        12.0,
    )
    .translate(0.0, CAMERA_BRIDGE_Y / 2.0 + 10.0, CAMERA_BRIDGE_Z - 72.0);

    left_post + right_post + beam + (sled - camera_ports) + front_light + rear_light
}

fn camera_bridge_post(side: &str) -> Part {
    let post = centered_cube(
        format!("co2_sensor_zero_span_station_evidence_bridge_{side}_post"),
        28.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    );
    let cable_slot = centered_cube(
        format!("co2_sensor_zero_span_station_evidence_bridge_{side}_cable_slot"),
        12.0,
        CAMERA_BRIDGE_Y + 8.0,
        104.0,
    )
    .translate(0.0, 0.0, 18.0);
    let tie_bore = centered_cylinder(
        format!("co2_sensor_zero_span_station_evidence_bridge_{side}_tie_bore"),
        2.1,
        CAMERA_BRIDGE_Y + 10.0,
        20,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, 0.0, 62.0);

    post - cable_slot - tie_bore
}

fn robot_service_keepout_gauges() -> Part {
    let outer = centered_cube(
        "co2_sensor_zero_span_station_keepout_outer_envelope",
        KEEP_OUT_X,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    );
    let inner = centered_cube(
        "co2_sensor_zero_span_station_keepout_inner_work_window",
        KEEP_OUT_X - 64.0,
        KEEP_OUT_Y - 64.0,
        KEEP_OUT_Z + 2.0,
    );
    let front_robot = centered_cube(
        "co2_sensor_zero_span_station_front_robot_approach_clearance_gauge",
        KEEP_OUT_X - 180.0,
        16.0,
        30.0,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 + FRONT_ROBOT_CLEARANCE / 2.0,
        KEEP_OUT_Z / 2.0 + 15.0,
    );
    let rear_gas_service = centered_cube(
        "co2_sensor_zero_span_station_rear_gas_service_clearance_gauge",
        KEEP_OUT_X - 220.0,
        16.0,
        30.0,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - REAR_GAS_SERVICE_CLEARANCE / 2.0,
        KEEP_OUT_Z / 2.0 + 15.0,
    );
    let right_sensor_service = centered_cube(
        "co2_sensor_zero_span_station_right_sensor_service_clearance_gauge",
        16.0,
        KEEP_OUT_Y - 230.0,
        30.0,
    )
    .translate(
        STATION_X / 2.0 - RIGHT_SENSOR_SERVICE_CLEARANCE / 2.0,
        0.0,
        KEEP_OUT_Z / 2.0 + 15.0,
    );
    let camera_service = centered_cube(
        "co2_sensor_zero_span_station_camera_bridge_service_clearance_gauge",
        CAMERA_BRIDGE_X - 120.0,
        CAMERA_SERVICE_CLEARANCE,
        18.0,
    )
    .translate(
        0.0,
        CAMERA_POS.1 - CAMERA_SERVICE_CLEARANCE / 2.0,
        KEEP_OUT_Z / 2.0 + 44.0,
    );
    let cartridge_lift = centered_cube(
        "co2_sensor_zero_span_station_sensor_cartridge_lift_clearance_gauge",
        SENSOR_NEST_X + 80.0,
        SENSOR_NEST_Y + 54.0,
        18.0,
    )
    .translate(
        SENSOR_POS.0,
        SENSOR_POS.1,
        KEEP_OUT_Z / 2.0 + CARTRIDGE_LIFT_CLEARANCE_Z,
    );

    (outer - inner)
        + front_robot
        + rear_gas_service
        + right_sensor_service
        + camera_service
        + cartridge_lift
}

fn fiducial_disc(name: &str) -> Part {
    centered_cylinder(format!("{name}_disc"), 7.0, 3.0, 32)
        - centered_cylinder(format!("{name}_center_dot"), 1.6, 4.0, 20)
        - centered_cube(format!("{name}_cross_x"), 12.0, 1.8, 4.0)
        - centered_cube(format!("{name}_cross_y"), 1.8, 12.0, 4.0)
}

fn gripper_fiducials(name: &str, span_x: f64) -> Part {
    let left = fiducial_disc(&format!(
        "co2_sensor_zero_span_station_{name}_left_grip_fiducial"
    ))
    .translate(-span_x / 2.0, 0.0, 4.0);
    let right = fiducial_disc(&format!(
        "co2_sensor_zero_span_station_{name}_right_grip_fiducial"
    ))
    .translate(span_x / 2.0, 0.0, 4.0);
    left + right
}

fn rectangular_frame(name: &str, x: f64, y: f64, z: f64, rail: f64) -> Part {
    let front = centered_cube(format!("{name}_front"), x, rail, z).translate(0.0, -y / 2.0, 0.0);
    let rear = centered_cube(format!("{name}_rear"), x, rail, z).translate(0.0, y / 2.0, 0.0);
    let left = centered_cube(format!("{name}_left"), rail, y, z).translate(-x / 2.0, 0.0, 0.0);
    let right = centered_cube(format!("{name}_right"), rail, y, z).translate(x / 2.0, 0.0, 0.0);
    front + rear + left + right
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn geometry_counts_match_station_plan() {
        assert_eq!(SENSOR_COUNT, 6);
        assert_eq!(SENSOR_NEST_COUNT, 12);
        assert_eq!(ZERO_PORT_COUNT, 6);
        assert_eq!(SPAN_PORT_COUNT, 6);
        assert_eq!(GAS_REFERENCE_PORT_COUNT, 12);
        assert_eq!(HUMIDITY_WELL_COUNT, 4);
        assert_eq!(DRIFT_LANE_COUNT, 3);
        assert_eq!(DISPOSITION_LANE_COUNT, 4);
        assert_eq!(PRESSURE_WITNESS_COUNT, 4);
        assert_eq!(FLOW_WITNESS_COUNT, 4);
        assert_eq!(ALARM_EVENT_TOKEN_COUNT, 12);
    }

    #[test]
    fn output_manifest_is_stable_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        assert_eq!(
            OUTPUTS[0],
            "output/closed_incubator_co2_sensor_zero_span_drift_quarantine_station_base_leak_tray_deck.stl"
        );
        assert_eq!(
            OUTPUTS[11],
            "output/closed_incubator_co2_sensor_zero_span_drift_quarantine_station_assembly.stl"
        );
        for path in OUTPUTS {
            assert!(path.starts_with(OUTPUT_PREFIX), "{path}");
            assert!(path.ends_with(".stl"), "{path}");
        }
    }

    #[test]
    fn station_bounds_and_module_clearances_are_valid() {
        assert_design_constraints();
        for module in module_rects() {
            assert!(module.fits_inside_station(), "{} does not fit", module.name);
        }
        assert!(STATION_X <= 1400.0);
        assert!(STATION_Y <= 900.0);
        assert!(CAMERA_CLEARANCE_Z > CARTRIDGE_LIFT_CLEARANCE_Z);
        assert!(KEEP_OUT_X < STATION_X);
        assert!(KEEP_OUT_Y < STATION_Y);
    }

    #[test]
    fn requested_feature_coverage_is_explicit() {
        for feature in [
            "base_leak_tray_deck",
            "co2_sensor_cartridge_nests",
            "zero_span_gas_reference_ports",
            "humidified_challenge_chamber_surrogate",
            "drift_comparison_lanes",
            "expired_certificate_custody_lands",
            "release_hold_reject_quarantine_lanes",
            "pressure_flow_witness_pockets",
            "alarm_event_token_rail",
            "evidence_camera_bridge",
            "robot_service_keepout_gauges",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
        assert_eq!(
            DISPOSITION_LANE_NAMES,
            ["release", "hold", "reject", "quarantine"]
        );
        assert_eq!(DRIFT_LANE_NAMES, ["zero", "span", "humid"]);
    }

    #[test]
    fn limitations_make_no_biological_or_clinical_claims() {
        for limitation in [
            "mechanical_fixture_only",
            "no_acceptance_limits",
            "no_calibration_protocol",
            "no_metrology_traceability_claim",
            "no_biological_claim",
            "no_clinical_claim",
        ] {
            assert!(LIMITATIONS.contains(&limitation));
        }

        let marketing_claim_words = ["patient", "therapy", "diagnosis", "diagnostic", "treat"];
        for field in OUTPUTS.iter().chain(REQUIRED_FEATURES.iter()) {
            for word in marketing_claim_words {
                assert!(!field.contains(word), "{field} contains {word}");
            }
        }
    }
}
