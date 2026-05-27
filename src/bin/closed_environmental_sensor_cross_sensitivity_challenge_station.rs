use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed environmental sensor cross-sensitivity challenge station.
//
// Intent:
// - Challenge CO2, O2, RH, and temperature probes in a no-cell station before
//   incubator or tissue-chip control data is trusted.
// - Keep calibration gas routing, humidity standards, cross-sensitivity tokens,
//   probe-placement bias, recovery-lag witnessing, certificates, custody,
//   release/hold/reject disposition, camera evidence, and robot/service
//   keepouts visible as named mechanical interfaces.
// - Model purchased sensors, gas bottles, regulators, and traceable standards as
//   fixture envelopes only; this CAD does not define acceptance criteria,
//   metrology traceability, or a biological release claim.

const OUTPUT_PREFIX: &str =
    "output/closed_environmental_sensor_cross_sensitivity_challenge_station_";

const OUTPUTS: [&str; 12] = [
    "output/closed_environmental_sensor_cross_sensitivity_challenge_station_containment_deck.stl",
    "output/closed_environmental_sensor_cross_sensitivity_challenge_station_gas_humidity_challenge_manifold.stl",
    "output/closed_environmental_sensor_cross_sensitivity_challenge_station_probe_nest_array.stl",
    "output/closed_environmental_sensor_cross_sensitivity_challenge_station_cross_sensitivity_token_rails.stl",
    "output/closed_environmental_sensor_cross_sensitivity_challenge_station_reference_standard_pockets.stl",
    "output/closed_environmental_sensor_cross_sensitivity_challenge_station_recovery_lag_witness_chamber.stl",
    "output/closed_environmental_sensor_cross_sensitivity_challenge_station_calibration_certificate_lands.stl",
    "output/closed_environmental_sensor_cross_sensitivity_challenge_station_leak_vent_witness_route.stl",
    "output/closed_environmental_sensor_cross_sensitivity_challenge_station_barcode_custody_lands.stl",
    "output/closed_environmental_sensor_cross_sensitivity_challenge_station_release_hold_reject_lanes.stl",
    "output/closed_environmental_sensor_cross_sensitivity_challenge_station_camera_evidence_bridge_robot_service_keepouts.stl",
    "output/closed_environmental_sensor_cross_sensitivity_challenge_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "containment_deck",
    "gas_humidity_challenge_manifold",
    "co2_o2_rh_temp_probe_nest_array",
    "cross_sensitivity_token_rails",
    "reference_standard_pockets",
    "recovery_lag_witness_chamber",
    "calibration_certificate_lands",
    "leak_vent_witness_route",
    "barcode_custody_lands",
    "release_hold_reject_lanes",
    "camera_evidence_bridge_robot_service_keepouts",
];

const LIMITATIONS: [&str; 5] = [
    "no_cell_validation_station",
    "mechanical_fixture_only",
    "no_acceptance_limits",
    "no_metrology_traceability_claim",
    "no_incubator_control_release_claim",
];

const STATION_X: f64 = 1320.0;
const STATION_Y: f64 = 860.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const SOCKET_DEPTH: f64 = 5.0;

const GAS_CHANNELS: usize = 4;
const GAS_NAMES: [&str; GAS_CHANNELS] = ["co2_span", "o2_span", "zero_air", "nitrogen"];
const HUMIDITY_CHANNELS: usize = 3;
const GAS_PORTS: usize = 8;
const HUMIDITY_STANDARD_PORTS: usize = 4;
const MANIFOLD_X: f64 = 452.0;
const MANIFOLD_Y: f64 = 180.0;
const MANIFOLD_Z: f64 = 66.0;
const MANIFOLD_POS: (f64, f64) = (-405.0, 214.0);
const GAS_PORT_PITCH_X: f64 = 48.0;
const HUMIDITY_PORT_PITCH_X: f64 = 64.0;
const TUBE_BORE_D: f64 = 6.2;
const QUICK_CONNECT_D: f64 = 14.0;

const SENSOR_TYPES: usize = 4;
const SENSOR_NAMES: [&str; SENSOR_TYPES] = ["co2", "o2", "rh", "temperature"];
const PLACEMENT_COLUMNS: usize = 3;
const PROBE_NEST_COUNT: usize = SENSOR_TYPES * PLACEMENT_COLUMNS;
const PROBE_NEST_X: f64 = 420.0;
const PROBE_NEST_Y: f64 = 180.0;
const PROBE_NEST_Z: f64 = 50.0;
const PROBE_POS: (f64, f64) = (35.0, 214.0);
const PROBE_PITCH_X: f64 = 94.0;
const PROBE_PITCH_Y: f64 = 46.0;
const PROBE_SLEEVE_D: f64 = 14.0;
const PROBE_BIAS_OFFSETS_MM: [f64; PLACEMENT_COLUMNS] = [-24.0, 0.0, 24.0];

const TOKEN_X: f64 = 320.0;
const TOKEN_Y: f64 = 180.0;
const TOKEN_Z: f64 = 36.0;
const TOKEN_POS: (f64, f64) = (444.0, 214.0);
const TOKEN_RAILS: usize = 4;
const TOKENS_PER_RAIL: usize = 5;
const TOKEN_SLOT_X: f64 = 42.0;
const TOKEN_SLOT_Y: f64 = 24.0;
const TOKEN_RAIL_PITCH_Y: f64 = 34.0;
const TOKEN_SLOT_PITCH_X: f64 = 50.0;

const STANDARD_X: f64 = 410.0;
const STANDARD_Y: f64 = 176.0;
const STANDARD_Z: f64 = 58.0;
const STANDARD_POS: (f64, f64) = (-410.0, 18.0);
const GAS_STANDARD_WELLS: usize = 4;
const RH_STANDARD_WELLS: usize = 4;
const THERMAL_STANDARD_WELLS: usize = 2;
const STANDARD_WELL_D: f64 = 32.0;
const STANDARD_WELL_DEPTH: f64 = 36.0;
const STANDARD_WELL_PITCH_X: f64 = 58.0;
const STANDARD_ROW_PITCH_Y: f64 = 56.0;

const RECOVERY_X: f64 = 420.0;
const RECOVERY_Y: f64 = 176.0;
const RECOVERY_Z: f64 = 112.0;
const RECOVERY_POS: (f64, f64) = (35.0, 18.0);
const RECOVERY_CELLS: usize = SENSOR_TYPES;
const RECOVERY_CELL_PITCH_X: f64 = 90.0;
const RECOVERY_WINDOW_COUNT: usize = 4;
const LAG_TOKEN_SLOTS: usize = 8;

const CERT_X: f64 = 320.0;
const CERT_Y: f64 = 176.0;
const CERT_Z: f64 = 12.0;
const CERT_POS: (f64, f64) = (444.0, 18.0);
const CERTIFICATE_LANDS: usize = 6;
const CAL_EVENT_LANDS: usize = 4;
const CERT_LAND_X: f64 = 78.0;
const CERT_LAND_Y: f64 = 38.0;

const WITNESS_X: f64 = 1080.0;
const WITNESS_Y: f64 = 62.0;
const WITNESS_Z: f64 = 38.0;
const WITNESS_POS: (f64, f64) = (-16.0, -170.0);
const LEAK_WITNESS_WINDOWS: usize = 5;
const VENT_WITNESS_PORTS: usize = GAS_CHANNELS + HUMIDITY_CHANNELS;
const DRAIN_PORT_D: f64 = 12.0;

const CUSTODY_X: f64 = 360.0;
const CUSTODY_Y: f64 = 150.0;
const CUSTODY_Z: f64 = 12.0;
const CUSTODY_POS: (f64, f64) = (-410.0, -300.0);
const BARCODE_LANDS: usize = 10;
const CUSTODY_SEAL_LANDS: usize = 6;
const RUN_RECORD_LANDS: usize = 2;

const DISPOSITION_X: f64 = 420.0;
const DISPOSITION_Y: f64 = 150.0;
const DISPOSITION_Z: f64 = 42.0;
const DISPOSITION_POS: (f64, f64) = (20.0, -300.0);
const DISPOSITION_LANES: usize = 3;
const DISPOSITION_SLOTS_PER_LANE: usize = SENSOR_TYPES;
const DISPOSITION_SLOT_X: f64 = 70.0;
const DISPOSITION_SLOT_Y: f64 = 34.0;
const DISPOSITION_GAP_MM: f64 = 40.0;

const CAMERA_X: f64 = 930.0;
const CAMERA_Y: f64 = 68.0;
const CAMERA_Z: f64 = 214.0;
const CAMERA_POS: (f64, f64) = (0.0, 348.0);
const CAMERA_PORTS: usize = 3;
const LIGHT_RAILS: usize = 2;
const CAMERA_CLEARANCE_Z: f64 = 176.0;

const KEEP_OUT_X: f64 = 1240.0;
const KEEP_OUT_Y: f64 = 790.0;
const KEEP_OUT_Z: f64 = 6.0;
const KEEP_OUT_ZONES: usize = 5;
const FRONT_ROBOT_CLEARANCE: f64 = 430.0;
const REAR_SERVICE_CLEARANCE: f64 = 300.0;
const SIDE_GAS_SERVICE_CLEARANCE: f64 = 220.0;
const PROBE_LIFT_CLEARANCE_Z: f64 = 150.0;

#[derive(Clone, Copy, Debug)]
struct Footprint {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Footprint {
    fn fits_inside_deck(self) -> bool {
        self.center.0.abs() + self.x / 2.0 <= STATION_X / 2.0 - RIM_W - 8.0
            && self.center.1.abs() + self.y / 2.0 <= STATION_Y / 2.0 - RIM_W - 8.0
    }

    fn overlaps(self, other: Footprint) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();

        dx < (self.x + other.x) / 2.0 && dy < (self.y + other.y) / 2.0
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = containment_deck();
    export(OUTPUTS[0], &deck);

    let manifold = gas_humidity_challenge_manifold();
    export(OUTPUTS[1], &manifold);

    let probes = probe_nest_array();
    export(OUTPUTS[2], &probes);

    let tokens = cross_sensitivity_token_rails();
    export(OUTPUTS[3], &tokens);

    let standards = reference_standard_pockets();
    export(OUTPUTS[4], &standards);

    let recovery = recovery_lag_witness_chamber();
    export(OUTPUTS[5], &recovery);

    let certificates = calibration_certificate_lands();
    export(OUTPUTS[6], &certificates);

    let witness = leak_vent_witness_route();
    export(OUTPUTS[7], &witness);

    let custody = barcode_custody_lands();
    export(OUTPUTS[8], &custody);

    let lanes = release_hold_reject_lanes();
    export(OUTPUTS[9], &lanes);

    let camera_keepouts = camera_evidence_bridge_robot_service_keepouts();
    export(OUTPUTS[10], &camera_keepouts);

    let assembly = deck
        + manifold.translate(MANIFOLD_POS.0, MANIFOLD_POS.1, insert_z(MANIFOLD_Z))
        + probes.translate(PROBE_POS.0, PROBE_POS.1, insert_z(PROBE_NEST_Z))
        + tokens.translate(TOKEN_POS.0, TOKEN_POS.1, insert_z(TOKEN_Z))
        + standards.translate(STANDARD_POS.0, STANDARD_POS.1, insert_z(STANDARD_Z))
        + recovery.translate(RECOVERY_POS.0, RECOVERY_POS.1, insert_z(RECOVERY_Z))
        + certificates.translate(CERT_POS.0, CERT_POS.1, insert_z(CERT_Z))
        + witness.translate(WITNESS_POS.0, WITNESS_POS.1, insert_z(WITNESS_Z))
        + custody.translate(CUSTODY_POS.0, CUSTODY_POS.1, insert_z(CUSTODY_Z))
        + lanes.translate(
            DISPOSITION_POS.0,
            DISPOSITION_POS.1,
            insert_z(DISPOSITION_Z),
        )
        + camera_keepouts.translate(0.0, 0.0, BASE_Z / 2.0);
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed environmental sensor cross-sensitivity challenge station:");
    println!("  Footprint:                  {STATION_X:.0}mm x {STATION_Y:.0}mm containment deck");
    println!("  Challenge channels:         {:?} gas channels, {HUMIDITY_CHANNELS} humidity routes, {GAS_PORTS} gas quick-connects", GAS_NAMES);
    println!("  Probe bias matrix:          {SENSOR_TYPES} sensor types x {PLACEMENT_COLUMNS} placements = {PROBE_NEST_COUNT} nests");
    println!("  Standards and tokens:       {GAS_STANDARD_WELLS} gas wells, {RH_STANDARD_WELLS} RH wells, {THERMAL_STANDARD_WELLS} thermal wells, {} cross-sensitivity token slots", TOKEN_RAILS * TOKENS_PER_RAIL);
    println!("  Recovery lag chamber:       {RECOVERY_CELLS} witness cells, {RECOVERY_WINDOW_COUNT} evidence windows, {LAG_TOKEN_SLOTS} elapsed-time token slots");
    println!("  Records and custody:        {CERTIFICATE_LANDS} certificate lands, {CAL_EVENT_LANDS} calibration event lands, {BARCODE_LANDS} barcode lands, {CUSTODY_SEAL_LANDS} custody seal lands");
    println!("  Release control:            {DISPOSITION_LANES} release/hold/reject lanes with {DISPOSITION_SLOTS_PER_LANE} sensor-family slots each");
    println!("  Witness and keepouts:       {LEAK_WITNESS_WINDOWS} leak windows, {VENT_WITNESS_PORTS} vent witness ports, camera bridge, and {KEEP_OUT_ZONES} robot/service keepout gauges");
    println!("  Feature groups covered:     {}", REQUIRED_FEATURES.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn insert_z(height: f64) -> f64 {
    BASE_Z / 2.0 + height / 2.0
}

fn assert_design_constraints() {
    for path in OUTPUTS {
        assert!(path.starts_with(OUTPUT_PREFIX));
    }

    let footprints = footprints();
    for footprint in footprints {
        assert!(
            footprint.fits_inside_deck(),
            "{} exceeds containment deck envelope",
            footprint.name
        );
    }

    for (left_index, left) in footprints.iter().enumerate() {
        for right in footprints.iter().skip(left_index + 1) {
            assert!(
                !left.overlaps(*right),
                "{} overlaps {}",
                left.name,
                right.name
            );
        }
    }

    assert_eq!(SENSOR_NAMES, ["co2", "o2", "rh", "temperature"]);
    assert_eq!(PROBE_NEST_COUNT, 12);
    assert!(DISPOSITION_GAP_MM >= 36.0);
    assert!(CAMERA_CLEARANCE_Z > PROBE_LIFT_CLEARANCE_Z);
    assert!(LIMITATIONS.contains(&"no_cell_validation_station"));
}

fn footprints() -> [Footprint; 9] {
    [
        Footprint {
            name: "gas_humidity_challenge_manifold",
            center: MANIFOLD_POS,
            x: MANIFOLD_X,
            y: MANIFOLD_Y,
        },
        Footprint {
            name: "probe_nest_array",
            center: PROBE_POS,
            x: PROBE_NEST_X,
            y: PROBE_NEST_Y,
        },
        Footprint {
            name: "cross_sensitivity_token_rails",
            center: TOKEN_POS,
            x: TOKEN_X,
            y: TOKEN_Y,
        },
        Footprint {
            name: "reference_standard_pockets",
            center: STANDARD_POS,
            x: STANDARD_X,
            y: STANDARD_Y,
        },
        Footprint {
            name: "recovery_lag_witness_chamber",
            center: RECOVERY_POS,
            x: RECOVERY_X,
            y: RECOVERY_Y,
        },
        Footprint {
            name: "calibration_certificate_lands",
            center: CERT_POS,
            x: CERT_X,
            y: CERT_Y,
        },
        Footprint {
            name: "leak_vent_witness_route",
            center: WITNESS_POS,
            x: WITNESS_X,
            y: WITNESS_Y,
        },
        Footprint {
            name: "barcode_custody_lands",
            center: CUSTODY_POS,
            x: CUSTODY_X,
            y: CUSTODY_Y,
        },
        Footprint {
            name: "release_hold_reject_lanes",
            center: DISPOSITION_POS,
            x: DISPOSITION_X,
            y: DISPOSITION_Y,
        },
    ]
}

fn containment_deck() -> Part {
    let floor = centered_cube(
        "closed_environmental_sensor_cross_sensitivity_containment_floor",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let washdown_recess = centered_cube(
        "closed_environmental_sensor_cross_sensitivity_washdown_recess",
        STATION_X - 126.0,
        STATION_Y - 118.0,
        7.0,
    )
    .translate(0.0, -8.0, BASE_Z / 2.0 - 3.5);
    let front_drain = centered_cylinder(
        "closed_environmental_sensor_cross_sensitivity_front_low_point_drain",
        DRAIN_PORT_D / 2.0,
        48.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 86.0, -STATION_Y / 2.0 - 2.0, -1.0);

    floor - washdown_recess - front_drain - insert_sockets() - mounting_slots() - datum_pin_holes()
        + perimeter_rims()
        + zone_dividers()
        + rear_service_bulkhead_tabs()
        + robot_datum_targets()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("closed_environmental_sensor_cross_sensitivity_insert_sockets");
    for footprint in footprints() {
        sockets = sockets
            + centered_cube(
                format!(
                    "closed_environmental_sensor_cross_sensitivity_{}_socket",
                    footprint.name
                ),
                footprint.x + 8.0,
                footprint.y + 8.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(
                footprint.center.0,
                footprint.center.1,
                BASE_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.2,
            );
    }
    sockets
}

fn mounting_slots() -> Part {
    let mut slots = Part::empty("closed_environmental_sensor_cross_sensitivity_mounting_slots");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 54.0), -(STATION_Y / 2.0 - 48.0)),
        (STATION_X / 2.0 - 54.0, -(STATION_Y / 2.0 - 48.0)),
        (-(STATION_X / 2.0 - 54.0), STATION_Y / 2.0 - 48.0),
        (STATION_X / 2.0 - 54.0, STATION_Y / 2.0 - 48.0),
        (0.0, STATION_Y / 2.0 - 48.0),
        (0.0, -(STATION_Y / 2.0 - 48.0)),
        (-410.0, 0.0),
        (410.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        slots = slots
            + centered_cylinder(
                format!("closed_environmental_sensor_cross_sensitivity_m6_clearance_{i}"),
                6.6 / 2.0,
                BASE_Z + 4.0,
                24,
            )
            .translate(*x, *y, 0.0)
            + centered_cube(
                format!("closed_environmental_sensor_cross_sensitivity_m6_service_slot_{i}"),
                24.0,
                6.8,
                BASE_Z + 4.0,
            )
            .translate(*x, *y, 0.0);
    }
    slots
}

fn datum_pin_holes() -> Part {
    let mut holes = Part::empty("closed_environmental_sensor_cross_sensitivity_datum_pin_holes");
    for (i, (x, y)) in [
        (-572.0, 354.0),
        (572.0, 354.0),
        (-572.0, -354.0),
        (572.0, -354.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("closed_environmental_sensor_cross_sensitivity_datum_pin_{i}"),
                5.2 / 2.0,
                BASE_Z + 4.0,
                24,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    let left = centered_cube(
        "closed_environmental_sensor_cross_sensitivity_left_containment_rim",
        RIM_W,
        STATION_Y - 62.0,
        RIM_Z,
    )
    .translate(
        -(STATION_X / 2.0 - RIM_W / 2.0),
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(
        "closed_environmental_sensor_cross_sensitivity_right_containment_rim",
        RIM_W,
        STATION_Y - 62.0,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let rear = centered_cube(
        "closed_environmental_sensor_cross_sensitivity_rear_service_rim",
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
        "closed_environmental_sensor_cross_sensitivity_front_low_spill_lip",
        STATION_X - 220.0,
        12.0,
        22.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 22.0, BASE_Z / 2.0 + 11.0);

    left + right + rear + front_low_lip
}

fn zone_dividers() -> Part {
    let rear_row = centered_cube(
        "closed_environmental_sensor_cross_sensitivity_probe_challenge_row_divider",
        STATION_X - 156.0,
        10.0,
        26.0,
    )
    .translate(0.0, 112.0, BASE_Z / 2.0 + 13.0);
    let middle_row = centered_cube(
        "closed_environmental_sensor_cross_sensitivity_standards_recovery_row_divider",
        STATION_X - 178.0,
        10.0,
        24.0,
    )
    .translate(0.0, -74.0, BASE_Z / 2.0 + 12.0);
    let front_row = centered_cube(
        "closed_environmental_sensor_cross_sensitivity_custody_disposition_row_divider",
        STATION_X - 230.0,
        8.0,
        20.0,
    )
    .translate(0.0, -230.0, BASE_Z / 2.0 + 10.0);
    let left_column = centered_cube(
        "closed_environmental_sensor_cross_sensitivity_gas_probe_column_divider",
        10.0,
        312.0,
        24.0,
    )
    .translate(-185.0, 116.0, BASE_Z / 2.0 + 12.0);
    let right_column = centered_cube(
        "closed_environmental_sensor_cross_sensitivity_probe_token_column_divider",
        10.0,
        312.0,
        24.0,
    )
    .translate(244.0, 116.0, BASE_Z / 2.0 + 12.0);

    rear_row + middle_row + front_row + left_column + right_column
}

fn rear_service_bulkhead_tabs() -> Part {
    let mut tabs = Part::empty("closed_environmental_sensor_cross_sensitivity_rear_bulkhead_tabs");
    for (i, x) in [-474.0, -316.0, -158.0, 0.0, 158.0, 316.0, 474.0]
        .iter()
        .enumerate()
    {
        let tab = centered_cube(
            format!("closed_environmental_sensor_cross_sensitivity_bulkhead_tab_{i}"),
            58.0,
            18.0,
            28.0,
        )
        .translate(*x, STATION_Y / 2.0 - 48.0, BASE_Z / 2.0 + 14.0);
        let bore = centered_cylinder(
            format!("closed_environmental_sensor_cross_sensitivity_bulkhead_bore_{i}"),
            8.0 / 2.0,
            26.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(*x, STATION_Y / 2.0 - 48.0, BASE_Z / 2.0 + 14.0);
        tabs = tabs + (tab - bore);
    }
    tabs
}

fn robot_datum_targets() -> Part {
    let mut targets = Part::empty("closed_environmental_sensor_cross_sensitivity_robot_datums");
    for (i, (x, y)) in [(-574.0, 336.0), (574.0, 336.0), (-574.0, -330.0)]
        .iter()
        .enumerate()
    {
        targets = targets
            + fiducial_disc(&format!(
                "closed_environmental_sensor_cross_sensitivity_robot_fiducial_{i}"
            ))
            .translate(*x, *y, BASE_Z / 2.0 + 2.5);
    }
    targets
}

fn gas_humidity_challenge_manifold() -> Part {
    let body = centered_cube(
        "closed_environmental_sensor_cross_sensitivity_gas_humidity_manifold_body",
        MANIFOLD_X,
        MANIFOLD_Y,
        MANIFOLD_Z,
    );
    let rear_bulkhead = centered_cube(
        "closed_environmental_sensor_cross_sensitivity_manifold_rear_bulkhead",
        MANIFOLD_X,
        18.0,
        MANIFOLD_Z + 34.0,
    )
    .translate(0.0, MANIFOLD_Y / 2.0 - 9.0, 17.0);

    let mut gas_cuts = Part::empty("closed_environmental_sensor_cross_sensitivity_gas_port_cuts");
    for i in 0..GAS_PORTS {
        let x = centered_index(i, GAS_PORTS, GAS_PORT_PITCH_X);
        gas_cuts = gas_cuts
            + centered_cylinder(
                format!("closed_environmental_sensor_cross_sensitivity_gas_qc_bore_{i}"),
                QUICK_CONNECT_D / 2.0,
                MANIFOLD_Z + 8.0,
                32,
            )
            .translate(x, -44.0, 0.0)
            + centered_cylinder(
                format!("closed_environmental_sensor_cross_sensitivity_gas_tube_lane_{i}"),
                TUBE_BORE_D / 2.0,
                MANIFOLD_Y + 12.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, 11.0);
    }

    let mut humidity_cuts =
        Part::empty("closed_environmental_sensor_cross_sensitivity_humidity_port_cuts");
    for i in 0..HUMIDITY_STANDARD_PORTS {
        let x = centered_index(i, HUMIDITY_STANDARD_PORTS, HUMIDITY_PORT_PITCH_X);
        humidity_cuts = humidity_cuts
            + centered_cylinder(
                format!("closed_environmental_sensor_cross_sensitivity_humidity_standard_port_{i}"),
                18.0 / 2.0,
                MANIFOLD_Z + 8.0,
                32,
            )
            .translate(x, 42.0, 0.0);
    }

    let mut route_ribs =
        Part::empty("closed_environmental_sensor_cross_sensitivity_manifold_route_ribs");
    for i in 0..GAS_CHANNELS {
        let y = centered_index(i, GAS_CHANNELS, 30.0) - 6.0;
        route_ribs = route_ribs
            + centered_cube(
                format!(
                    "closed_environmental_sensor_cross_sensitivity_{}_raised_route",
                    GAS_NAMES[i]
                ),
                MANIFOLD_X - 54.0,
                6.0,
                8.0,
            )
            .translate(0.0, y, MANIFOLD_Z / 2.0 + 4.0);
    }

    body + rear_bulkhead + route_ribs - gas_cuts - humidity_cuts
}

fn probe_nest_array() -> Part {
    let body = centered_cube(
        "closed_environmental_sensor_cross_sensitivity_probe_nest_body",
        PROBE_NEST_X,
        PROBE_NEST_Y,
        PROBE_NEST_Z,
    );
    let rear_fence = centered_cube(
        "closed_environmental_sensor_cross_sensitivity_probe_rear_fence",
        PROBE_NEST_X,
        14.0,
        PROBE_NEST_Z + 30.0,
    )
    .translate(0.0, PROBE_NEST_Y / 2.0 - 7.0, 15.0);

    let mut cuts = Part::empty("closed_environmental_sensor_cross_sensitivity_probe_cuts");
    let mut bias_flags =
        Part::empty("closed_environmental_sensor_cross_sensitivity_probe_bias_flags");
    for sensor in 0..SENSOR_TYPES {
        let y = centered_index(sensor, SENSOR_TYPES, PROBE_PITCH_Y);
        for column in 0..PLACEMENT_COLUMNS {
            let x = centered_index(column, PLACEMENT_COLUMNS, PROBE_PITCH_X);
            let name = SENSOR_NAMES[sensor];
            cuts = cuts
                + centered_cylinder(
                    format!(
                        "closed_environmental_sensor_cross_sensitivity_{name}_placement_{column}_probe_sleeve"
                    ),
                    probe_sleeve_radius(sensor),
                    PROBE_NEST_Y + 10.0,
                    32,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(x + PROBE_BIAS_OFFSETS_MM[column] / 5.0, y, 8.0)
                + centered_cube(
                    format!(
                        "closed_environmental_sensor_cross_sensitivity_{name}_placement_{column}_top_access"
                    ),
                    17.0,
                    34.0,
                    18.0,
                )
                .translate(x, y, PROBE_NEST_Z / 2.0 - 5.0);
            bias_flags = bias_flags
                + centered_cube(
                    format!(
                        "closed_environmental_sensor_cross_sensitivity_{name}_bias_flag_{column}"
                    ),
                    20.0,
                    5.0,
                    10.0 + (column as f64 * 2.0),
                )
                .translate(x, y + 18.0, PROBE_NEST_Z / 2.0 + 5.0 + column as f64);
        }
    }

    body + rear_fence + bias_flags - cuts
}

fn cross_sensitivity_token_rails() -> Part {
    let base = centered_cube(
        "closed_environmental_sensor_cross_sensitivity_token_rail_base",
        TOKEN_X,
        TOKEN_Y,
        TOKEN_Z,
    );
    let mut cuts = Part::empty("closed_environmental_sensor_cross_sensitivity_token_slot_cuts");
    let mut rails = Part::empty("closed_environmental_sensor_cross_sensitivity_token_rails");
    for rail in 0..TOKEN_RAILS {
        let y = centered_index(rail, TOKEN_RAILS, TOKEN_RAIL_PITCH_Y);
        rails = rails
            + centered_cube(
                format!("closed_environmental_sensor_cross_sensitivity_token_rail_{rail}"),
                TOKEN_X - 34.0,
                7.0,
                16.0,
            )
            .translate(0.0, y, TOKEN_Z / 2.0 + 8.0);
        for slot in 0..TOKENS_PER_RAIL {
            let x = centered_index(slot, TOKENS_PER_RAIL, TOKEN_SLOT_PITCH_X);
            cuts = cuts
                + centered_cube(
                    format!(
                        "closed_environmental_sensor_cross_sensitivity_token_slot_{rail}_{slot}"
                    ),
                    TOKEN_SLOT_X,
                    TOKEN_SLOT_Y,
                    TOKEN_Z + 4.0,
                )
                .translate(x, y, 4.0);
        }
    }
    base + rails - cuts
}

fn reference_standard_pockets() -> Part {
    let body = centered_cube(
        "closed_environmental_sensor_cross_sensitivity_reference_standard_body",
        STANDARD_X,
        STANDARD_Y,
        STANDARD_Z,
    );
    let row_separator = centered_cube(
        "closed_environmental_sensor_cross_sensitivity_reference_standard_row_separator",
        STANDARD_X - 40.0,
        8.0,
        STANDARD_Z + 22.0,
    )
    .translate(0.0, 0.0, 11.0);

    let mut cuts =
        Part::empty("closed_environmental_sensor_cross_sensitivity_reference_standard_cuts");
    for i in 0..GAS_STANDARD_WELLS {
        cuts = cuts
            + standard_well(&format!(
                "closed_environmental_sensor_cross_sensitivity_gas_standard_well_{i}"
            ))
            .translate(
                centered_index(i, GAS_STANDARD_WELLS, STANDARD_WELL_PITCH_X),
                STANDARD_ROW_PITCH_Y / 2.0,
                10.0,
            );
    }
    for i in 0..RH_STANDARD_WELLS {
        cuts = cuts
            + standard_well(&format!(
                "closed_environmental_sensor_cross_sensitivity_rh_standard_well_{i}"
            ))
            .translate(
                centered_index(i, RH_STANDARD_WELLS, STANDARD_WELL_PITCH_X),
                -STANDARD_ROW_PITCH_Y / 2.0,
                10.0,
            );
    }
    for i in 0..THERMAL_STANDARD_WELLS {
        cuts = cuts
            + standard_well(&format!(
                "closed_environmental_sensor_cross_sensitivity_thermal_standard_well_{i}"
            ))
            .translate(
                centered_index(i, THERMAL_STANDARD_WELLS, 50.0) + 142.0,
                0.0,
                10.0,
            );
    }

    body + row_separator - cuts
}

fn recovery_lag_witness_chamber() -> Part {
    let base = centered_cube(
        "closed_environmental_sensor_cross_sensitivity_recovery_lag_chamber_floor",
        RECOVERY_X,
        RECOVERY_Y,
        26.0,
    );
    let rear_wall = centered_cube(
        "closed_environmental_sensor_cross_sensitivity_recovery_lag_rear_wall",
        RECOVERY_X,
        12.0,
        RECOVERY_Z,
    )
    .translate(0.0, RECOVERY_Y / 2.0 - 6.0, RECOVERY_Z / 2.0 - 13.0);
    let front_wall = centered_cube(
        "closed_environmental_sensor_cross_sensitivity_recovery_lag_front_window_wall",
        RECOVERY_X,
        10.0,
        RECOVERY_Z - 24.0,
    )
    .translate(0.0, -RECOVERY_Y / 2.0 + 5.0, RECOVERY_Z / 2.0 - 1.0);
    let mut dividers =
        Part::empty("closed_environmental_sensor_cross_sensitivity_recovery_lag_dividers");
    let mut cuts = Part::empty("closed_environmental_sensor_cross_sensitivity_recovery_lag_cuts");

    for i in 0..RECOVERY_CELLS {
        let x = centered_index(i, RECOVERY_CELLS, RECOVERY_CELL_PITCH_X);
        cuts = cuts
            + centered_cube(
                format!("closed_environmental_sensor_cross_sensitivity_recovery_window_{i}"),
                54.0,
                16.0,
                46.0,
            )
            .translate(x, -RECOVERY_Y / 2.0 + 3.0, 52.0)
            + centered_cylinder(
                format!("closed_environmental_sensor_cross_sensitivity_recovery_probe_port_{i}"),
                8.0 / 2.0,
                RECOVERY_Y + 10.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, 48.0);
    }
    for i in 1..RECOVERY_CELLS {
        let x = centered_index(i, RECOVERY_CELLS + 1, RECOVERY_CELL_PITCH_X);
        dividers = dividers
            + centered_cube(
                format!("closed_environmental_sensor_cross_sensitivity_recovery_cell_divider_{i}"),
                8.0,
                RECOVERY_Y - 24.0,
                RECOVERY_Z - 16.0,
            )
            .translate(x, 0.0, RECOVERY_Z / 2.0 - 8.0);
    }
    for i in 0..LAG_TOKEN_SLOTS {
        cuts = cuts
            + centered_cube(
                format!("closed_environmental_sensor_cross_sensitivity_recovery_time_token_{i}"),
                30.0,
                20.0,
                12.0,
            )
            .translate(centered_index(i, LAG_TOKEN_SLOTS, 42.0), 48.0, 16.0);
    }

    base + rear_wall + front_wall + dividers - cuts
}

fn calibration_certificate_lands() -> Part {
    let base = centered_cube(
        "closed_environmental_sensor_cross_sensitivity_calibration_certificate_panel",
        CERT_X,
        CERT_Y,
        CERT_Z,
    );
    let mut lands =
        Part::empty("closed_environmental_sensor_cross_sensitivity_calibration_certificate_lands");
    for i in 0..CERTIFICATE_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_environmental_sensor_cross_sensitivity_certificate_land_{i}"),
                CERT_LAND_X,
                CERT_LAND_Y,
                4.0,
            )
            .translate(
                centered_index(i % 3, 3, 96.0),
                centered_index(i / 3, 2, 62.0),
                CERT_Z / 2.0 + 2.0,
            );
    }
    for i in 0..CAL_EVENT_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_environmental_sensor_cross_sensitivity_cal_event_witness_land_{i}"),
                48.0,
                18.0,
                6.0,
            )
            .translate(
                centered_index(i, CAL_EVENT_LANDS, 60.0),
                -CERT_Y / 2.0 + 18.0,
                CERT_Z / 2.0 + 3.0,
            );
    }

    base + lands
}

fn leak_vent_witness_route() -> Part {
    let body = centered_cube(
        "closed_environmental_sensor_cross_sensitivity_leak_vent_witness_body",
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    );
    let mut cuts = Part::empty("closed_environmental_sensor_cross_sensitivity_leak_vent_cuts");
    let mut witness_windows =
        Part::empty("closed_environmental_sensor_cross_sensitivity_leak_witness_windows");
    for i in 0..LEAK_WITNESS_WINDOWS {
        let x = centered_index(i, LEAK_WITNESS_WINDOWS, 170.0);
        cuts = cuts
            + centered_cube(
                format!("closed_environmental_sensor_cross_sensitivity_leak_window_recess_{i}"),
                92.0,
                WITNESS_Y + 4.0,
                14.0,
            )
            .translate(x, 0.0, WITNESS_Z / 2.0 - 7.0);
        witness_windows = witness_windows
            + centered_cube(
                format!("closed_environmental_sensor_cross_sensitivity_leak_window_frame_{i}"),
                104.0,
                8.0,
                8.0,
            )
            .translate(x, -WITNESS_Y / 2.0 - 4.0, WITNESS_Z / 2.0 + 4.0);
    }
    for i in 0..VENT_WITNESS_PORTS {
        cuts = cuts
            + centered_cylinder(
                format!("closed_environmental_sensor_cross_sensitivity_vent_witness_port_{i}"),
                7.0 / 2.0,
                WITNESS_Y + 8.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(centered_index(i, VENT_WITNESS_PORTS, 74.0), 0.0, 9.0);
    }

    body + witness_windows - cuts
}

fn barcode_custody_lands() -> Part {
    let panel = centered_cube(
        "closed_environmental_sensor_cross_sensitivity_barcode_custody_panel",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    let mut lands = Part::empty("closed_environmental_sensor_cross_sensitivity_custody_lands");
    for i in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_environmental_sensor_cross_sensitivity_barcode_land_{i}"),
                58.0,
                20.0,
                4.0,
            )
            .translate(
                centered_index(i % 5, 5, 66.0),
                centered_index(i / 5, 2, 46.0),
                CUSTODY_Z / 2.0 + 2.0,
            );
    }
    for i in 0..CUSTODY_SEAL_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_environmental_sensor_cross_sensitivity_custody_seal_land_{i}"),
                34.0,
                14.0,
                6.0,
            )
            .translate(
                centered_index(i, CUSTODY_SEAL_LANDS, 48.0),
                -CUSTODY_Y / 2.0 + 18.0,
                CUSTODY_Z / 2.0 + 3.0,
            );
    }
    for i in 0..RUN_RECORD_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_environmental_sensor_cross_sensitivity_run_record_dock_{i}"),
                92.0,
                26.0,
                8.0,
            )
            .translate(
                centered_index(i, RUN_RECORD_LANDS, 118.0),
                CUSTODY_Y / 2.0 - 26.0,
                CUSTODY_Z / 2.0 + 4.0,
            );
    }
    panel + lands
}

fn release_hold_reject_lanes() -> Part {
    let base = centered_cube(
        "closed_environmental_sensor_cross_sensitivity_release_hold_reject_lane_base",
        DISPOSITION_X,
        DISPOSITION_Y,
        DISPOSITION_Z,
    );
    let mut cuts =
        Part::empty("closed_environmental_sensor_cross_sensitivity_release_hold_reject_cuts");
    let mut dividers =
        Part::empty("closed_environmental_sensor_cross_sensitivity_release_hold_reject_dividers");
    for lane in 0..DISPOSITION_LANES {
        let y = centered_index(lane, DISPOSITION_LANES, 46.0);
        dividers = dividers
            + centered_cube(
                format!(
                    "closed_environmental_sensor_cross_sensitivity_disposition_lane_fence_{lane}"
                ),
                DISPOSITION_X - 28.0,
                5.0,
                18.0,
            )
            .translate(0.0, y + 22.0, DISPOSITION_Z / 2.0 + 9.0);
        for slot in 0..DISPOSITION_SLOTS_PER_LANE {
            cuts = cuts
                + centered_cube(
                    format!(
                    "closed_environmental_sensor_cross_sensitivity_disposition_slot_{lane}_{slot}"
                ),
                    DISPOSITION_SLOT_X,
                    DISPOSITION_SLOT_Y,
                    18.0,
                )
                .translate(
                    centered_index(slot, DISPOSITION_SLOTS_PER_LANE, 88.0),
                    y,
                    DISPOSITION_Z / 2.0 - 8.0,
                );
        }
    }
    base + dividers - cuts
}

fn camera_evidence_bridge_robot_service_keepouts() -> Part {
    let bridge = camera_evidence_bridge();
    let keepouts = robot_service_keepout_gauges();
    bridge.translate(CAMERA_POS.0, CAMERA_POS.1, CAMERA_Z / 2.0) + keepouts
}

fn camera_evidence_bridge() -> Part {
    let left_post = centered_cube(
        "closed_environmental_sensor_cross_sensitivity_camera_left_post",
        28.0,
        CAMERA_Y,
        CAMERA_Z,
    )
    .translate(-CAMERA_X / 2.0 + 14.0, 0.0, 0.0);
    let right_post = centered_cube(
        "closed_environmental_sensor_cross_sensitivity_camera_right_post",
        28.0,
        CAMERA_Y,
        CAMERA_Z,
    )
    .translate(CAMERA_X / 2.0 - 14.0, 0.0, 0.0);
    let top_beam = centered_cube(
        "closed_environmental_sensor_cross_sensitivity_camera_top_beam",
        CAMERA_X,
        CAMERA_Y,
        30.0,
    )
    .translate(0.0, 0.0, CAMERA_Z / 2.0 - 15.0);

    let mut ports = Part::empty("closed_environmental_sensor_cross_sensitivity_camera_ports");
    for i in 0..CAMERA_PORTS {
        ports = ports
            + centered_cylinder(
                format!("closed_environmental_sensor_cross_sensitivity_camera_lens_port_{i}"),
                18.0 / 2.0,
                CAMERA_Y + 8.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(i, CAMERA_PORTS, 190.0),
                0.0,
                CAMERA_Z / 2.0 - 16.0,
            );
    }
    let mut light_rails = Part::empty("closed_environmental_sensor_cross_sensitivity_light_rails");
    for i in 0..LIGHT_RAILS {
        light_rails = light_rails
            + centered_cube(
                format!("closed_environmental_sensor_cross_sensitivity_light_rail_{i}"),
                CAMERA_X - 120.0,
                8.0,
                10.0,
            )
            .translate(
                0.0,
                centered_index(i, LIGHT_RAILS, 38.0),
                CAMERA_Z / 2.0 - 40.0,
            );
    }

    left_post + right_post + top_beam + light_rails - ports
}

fn robot_service_keepout_gauges() -> Part {
    let outline = centered_cube(
        "closed_environmental_sensor_cross_sensitivity_keepout_outline_gauge",
        KEEP_OUT_X,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    );
    let interior = centered_cube(
        "closed_environmental_sensor_cross_sensitivity_keepout_open_interior",
        KEEP_OUT_X - 28.0,
        KEEP_OUT_Y - 28.0,
        KEEP_OUT_Z + 2.0,
    );
    let mut flags = Part::empty("closed_environmental_sensor_cross_sensitivity_keepout_flags");
    for (i, (x, y, label_height)) in [
        (0.0, -KEEP_OUT_Y / 2.0 + 18.0, FRONT_ROBOT_CLEARANCE / 40.0),
        (0.0, KEEP_OUT_Y / 2.0 - 18.0, REAR_SERVICE_CLEARANCE / 40.0),
        (
            -KEEP_OUT_X / 2.0 + 18.0,
            0.0,
            SIDE_GAS_SERVICE_CLEARANCE / 40.0,
        ),
        (
            KEEP_OUT_X / 2.0 - 18.0,
            0.0,
            SIDE_GAS_SERVICE_CLEARANCE / 40.0,
        ),
        (
            KEEP_OUT_X / 2.0 - 160.0,
            KEEP_OUT_Y / 2.0 - 86.0,
            PROBE_LIFT_CLEARANCE_Z / 40.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        flags = flags
            + centered_cube(
                format!("closed_environmental_sensor_cross_sensitivity_keepout_flag_{i}"),
                46.0,
                12.0,
                12.0 + label_height,
            )
            .translate(*x, *y, KEEP_OUT_Z / 2.0 + 6.0 + label_height / 2.0);
    }
    (outline - interior) + flags
}

fn standard_well(name: &str) -> Part {
    centered_cylinder(name, STANDARD_WELL_D / 2.0, STANDARD_WELL_DEPTH, 32)
}

fn probe_sleeve_radius(sensor_index: usize) -> f64 {
    match sensor_index {
        0 => (PROBE_SLEEVE_D + 2.0) / 2.0,
        1 => (PROBE_SLEEVE_D + 1.2) / 2.0,
        2 => (PROBE_SLEEVE_D + 0.8) / 2.0,
        _ => PROBE_SLEEVE_D / 2.0,
    }
}

fn fiducial_disc(name: &str) -> Part {
    centered_cylinder(name, 16.0, 5.0, 32)
        - centered_cylinder(format!("{name}_center_dot"), 4.0, 6.0, 24)
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn output_contract_is_stable_and_unique() {
        assert!((11..=13).contains(&OUTPUTS.len()));
        assert_eq!(OUTPUTS.len(), 12);

        let unique: HashSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());

        for path in OUTPUTS {
            assert!(path.starts_with(OUTPUT_PREFIX));
            assert!(path.ends_with(".stl"));
        }
        assert!(OUTPUTS
            .iter()
            .any(|path| *path == format!("{OUTPUT_PREFIX}assembly.stl")));
    }

    #[test]
    fn required_features_cover_sensor_validation_intent() {
        for expected in [
            "gas_humidity_challenge_manifold",
            "co2_o2_rh_temp_probe_nest_array",
            "cross_sensitivity_token_rails",
            "reference_standard_pockets",
            "recovery_lag_witness_chamber",
            "calibration_certificate_lands",
            "leak_vent_witness_route",
            "barcode_custody_lands",
            "release_hold_reject_lanes",
        ] {
            assert!(REQUIRED_FEATURES.contains(&expected));
        }
    }

    #[test]
    fn probe_matrix_encodes_cross_sensitivity_and_placement_bias() {
        assert_eq!(SENSOR_NAMES, ["co2", "o2", "rh", "temperature"]);
        assert_eq!(PLACEMENT_COLUMNS, 3);
        assert_eq!(PROBE_NEST_COUNT, 12);
        assert!(PROBE_BIAS_OFFSETS_MM[0] < 0.0);
        assert_eq!(PROBE_BIAS_OFFSETS_MM[1], 0.0);
        assert!(PROBE_BIAS_OFFSETS_MM[2] > 0.0);
    }

    #[test]
    fn layout_modules_fit_without_overlap() {
        assert_design_constraints();
    }

    #[test]
    fn records_and_disposition_are_segregated() {
        assert!(CERTIFICATE_LANDS >= SENSOR_TYPES);
        assert!(BARCODE_LANDS >= PROBE_NEST_COUNT - 2);
        assert_eq!(DISPOSITION_LANES, 3);
        assert_eq!(DISPOSITION_SLOTS_PER_LANE, SENSOR_TYPES);
        assert!(DISPOSITION_GAP_MM >= 36.0);
    }
}
