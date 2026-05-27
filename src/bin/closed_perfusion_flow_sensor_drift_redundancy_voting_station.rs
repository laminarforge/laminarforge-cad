use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed-system redundant perfusion flow-sensor drift/redundancy voting station.
//
// Intent:
// - Package a no-live-chip validation fixture for primary, secondary, and
//   reference flow sensor comparison before automatic media decisions can affect
//   tissue-chip lanes.
// - Make drift, sensor disagreement, bubble/viscosity false-flow response,
//   lane cross-coupling, bypass/flush routing, voting/alarm tokens, custody
//   lands, disposition gates, and camera evidence physically visible.
// - This is mechanical CAD packaging/validation hardware only. It is not a
//   calibrated metrology standard, biological acceptance criterion, or wetted
//   material specification.

const PREFIX: &str = "closed_perfusion_flow_sensor_drift_redundancy_voting_station";
const OUTPUT_PREFIX: &str = "output/closed_perfusion_flow_sensor_drift_redundancy_voting_station";
const OUTPUTS: [&str; 12] = [
    "output/closed_perfusion_flow_sensor_drift_redundancy_voting_station_containment_deck.stl",
    "output/closed_perfusion_flow_sensor_drift_redundancy_voting_station_redundant_sensor_dock_array.stl",
    "output/closed_perfusion_flow_sensor_drift_redundancy_voting_station_surrogate_lane_manifold.stl",
    "output/closed_perfusion_flow_sensor_drift_redundancy_voting_station_restrictor_cartridge_bank.stl",
    "output/closed_perfusion_flow_sensor_drift_redundancy_voting_station_bubble_viscosity_challenge_wells.stl",
    "output/closed_perfusion_flow_sensor_drift_redundancy_voting_station_pressure_tap_bosses.stl",
    "output/closed_perfusion_flow_sensor_drift_redundancy_voting_station_bypass_flush_route_panel.stl",
    "output/closed_perfusion_flow_sensor_drift_redundancy_voting_station_timestamp_alarm_token_rail.stl",
    "output/closed_perfusion_flow_sensor_drift_redundancy_voting_station_barcode_coa_custody_lands.stl",
    "output/closed_perfusion_flow_sensor_drift_redundancy_voting_station_release_hold_reject_gates.stl",
    "output/closed_perfusion_flow_sensor_drift_redundancy_voting_station_evidence_camera_bridge_keepouts.stl",
    "output/closed_perfusion_flow_sensor_drift_redundancy_voting_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "containment_deck",
    "primary_secondary_reference_flow_sensor_docks",
    "surrogate_lane_manifold",
    "calibrated_restrictor_cartridge_bank",
    "bubble_viscosity_challenge_wells",
    "pressure_tap_bosses",
    "bypass_flush_route",
    "timestamp_alarm_token_rail",
    "barcode_coa_custody_lands",
    "release_hold_reject_gates",
    "evidence_camera_bridge_robot_service_keepouts",
];

const LIMITATIONS: [&str; 5] = [
    "mechanical_validation_fixture_only",
    "not_a_calibrated_metrology_standard",
    "no_biological_acceptance_criteria",
    "external_sensors_restrictors_and_media",
    "voting_thresholds_defined_by_protocol",
];

const STATION_X: f64 = 1420.0;
const STATION_Y: f64 = 900.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const BASIN_DEPTH: f64 = 7.0;
const MOUNT_HOLE_D: f64 = 6.8;
const DATUM_COUNT: usize = 6;

const LANES: usize = 20;
const ACTIVE_LANES: usize = 16;
const LANE_PITCH_Y: f64 = 21.0;
const MANIFOLD_POS: (f64, f64) = (-205.0, 120.0);
const MANIFOLD_X: f64 = 720.0;
const MANIFOLD_Y: f64 = 430.0;
const MANIFOLD_Z: f64 = 30.0;
const LANE_TRACE_X: f64 = 590.0;
const LANE_TRACE_Y: f64 = 6.0;
const LANE_TRACE_Z: f64 = 7.0;
const LANE_HEADER_D: f64 = 12.0;
const CROSS_COUPLING_SENTINELS: usize = 4;

const SENSOR_POS: (f64, f64) = (-410.0, -225.0);
const SENSOR_BANK_X: f64 = 500.0;
const SENSOR_BANK_Y: f64 = 235.0;
const SENSOR_BANK_Z: f64 = 42.0;
const SENSOR_COLUMNS: usize = 3;
const SENSOR_ROWS: usize = 5;
const SENSOR_DOCKS: usize = SENSOR_COLUMNS * SENSOR_ROWS;
const SENSOR_DOCK_X: f64 = 86.0;
const SENSOR_DOCK_Y: f64 = 34.0;
const SENSOR_DOCK_Z: f64 = 16.0;
const SENSOR_COLUMN_PITCH_X: f64 = 140.0;
const SENSOR_ROW_PITCH_Y: f64 = 39.0;
const SENSOR_CLEARANCE: f64 = 0.8;

const RESTRICTOR_POS: (f64, f64) = (425.0, 168.0);
const RESTRICTOR_BANK_X: f64 = 470.0;
const RESTRICTOR_BANK_Y: f64 = 245.0;
const RESTRICTOR_BANK_Z: f64 = 48.0;
const RESTRICTOR_ROWS: usize = 4;
const RESTRICTOR_COLS: usize = 5;
const RESTRICTOR_CARTRIDGES: usize = RESTRICTOR_ROWS * RESTRICTOR_COLS;
const RESTRICTOR_PITCH_X: f64 = 82.0;
const RESTRICTOR_PITCH_Y: f64 = 48.0;
const RESTRICTOR_SLOT_X: f64 = 58.0;
const RESTRICTOR_SLOT_Y: f64 = 24.0;
const RESTRICTOR_GRADES_UL_MIN: [usize; RESTRICTOR_CARTRIDGES] = [
    5, 5, 10, 10, 20, 20, 40, 40, 80, 80, 120, 120, 160, 160, 200, 200, 240, 280, 320, 360,
];

const CHALLENGE_POS: (f64, f64) = (444.0, -118.0);
const CHALLENGE_X: f64 = 365.0;
const CHALLENGE_Y: f64 = 250.0;
const CHALLENGE_Z: f64 = 58.0;
const BUBBLE_WELLS: usize = 6;
const VISCOSITY_WELLS: usize = 6;
const CHALLENGE_WELLS: usize = BUBBLE_WELLS + VISCOSITY_WELLS;
const CHALLENGE_PITCH_X: f64 = 54.0;
const CHALLENGE_PITCH_Y: f64 = 70.0;
const CHALLENGE_WELL_D: f64 = 28.0;

const PRESSURE_POS: (f64, f64) = (0.0, -230.0);
const PRESSURE_PANEL_X: f64 = 300.0;
const PRESSURE_PANEL_Y: f64 = 250.0;
const PRESSURE_PANEL_Z: f64 = 32.0;
const PRESSURE_TAPS_PER_SENSOR_TRIPLET: usize = 4;
const SENSOR_TRIPLETS: usize = 5;
const PRESSURE_TAPS: usize = SENSOR_TRIPLETS * PRESSURE_TAPS_PER_SENSOR_TRIPLET;
const PRESSURE_TAP_D: f64 = 12.0;
const PRESSURE_TAP_PITCH_X: f64 = 54.0;
const PRESSURE_TAP_PITCH_Y: f64 = 45.0;

const FLUSH_POS: (f64, f64) = (415.0, 365.0);
const FLUSH_PANEL_X: f64 = 430.0;
const FLUSH_PANEL_Y: f64 = 90.0;
const FLUSH_PANEL_Z: f64 = 32.0;
const FLUSH_ROUTE_STATES: usize = 3;
const FLUSH_VALVE_SOCKETS: usize = 6;
const FLUSH_WASTE_CUPS: usize = 4;

const TOKEN_POS: (f64, f64) = (-440.0, 380.0);
const TOKEN_RAIL_X: f64 = 430.0;
const TOKEN_RAIL_Y: f64 = 72.0;
const TOKEN_RAIL_Z: f64 = 26.0;
const TIMESTAMP_TOKENS: usize = 8;
const ALARM_TOKENS: usize = 8;
const TOKEN_COUNT: usize = TIMESTAMP_TOKENS + ALARM_TOKENS;
const TOKEN_PITCH_X: f64 = 50.0;
const TOKEN_D: f64 = 25.0;

const CUSTODY_POS: (f64, f64) = (-482.0, -385.0);
const CUSTODY_X: f64 = 385.0;
const CUSTODY_Y: f64 = 60.0;
const CUSTODY_Z: f64 = 12.0;
const BARCODE_LANDS: usize = 10;
const COA_LANDS: usize = 4;
const CUSTODY_WITNESS_LANDS: usize = 6;

const DISPOSITION_POS: (f64, f64) = (-60.0, -390.0);
const DISPOSITION_X: f64 = 410.0;
const DISPOSITION_Y: f64 = 55.0;
const DISPOSITION_Z: f64 = 30.0;
const DISPOSITION_GATES: usize = 3;
const STATUS_SLOTS_PER_GATE: usize = 6;

const BRIDGE_POS: (f64, f64) = (398.0, -390.0);
const BRIDGE_X: f64 = 345.0;
const BRIDGE_Y: f64 = 55.0;
const BRIDGE_Z: f64 = 150.0;
const CAMERA_TARGETS: usize = 6;
const KEEP_OUT_GAUGES: usize = 7;
const ROBOT_SWEEP_CLEARANCE_Z: f64 = 235.0;
const SERVICE_CLEARANCE_FRONT_Y: f64 = 330.0;
const SERVICE_CLEARANCE_REAR_Y: f64 = 190.0;
const SENSOR_LIFT_CLEARANCE_Z: f64 = 120.0;

#[derive(Clone, Copy, Debug)]
struct Zone {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Zone {
    fn fits_station(self) -> bool {
        let usable_x = STATION_X / 2.0 - RIM_W - 12.0;
        let usable_y = STATION_Y / 2.0 - RIM_W - 12.0;
        self.center.0.abs() + self.x / 2.0 <= usable_x
            && self.center.1.abs() + self.y / 2.0 <= usable_y
    }

    fn overlaps(self, other: Zone, clearance: f64) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();
        dx < (self.x + other.x) / 2.0 + clearance && dy < (self.y + other.y) / 2.0 + clearance
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SensorRole {
    Primary,
    Secondary,
    Reference,
}

impl SensorRole {
    fn all() -> [SensorRole; SENSOR_COLUMNS] {
        [
            SensorRole::Primary,
            SensorRole::Secondary,
            SensorRole::Reference,
        ]
    }

    fn index(self) -> usize {
        match self {
            SensorRole::Primary => 0,
            SensorRole::Secondary => 1,
            SensorRole::Reference => 2,
        }
    }

    fn label(self) -> &'static str {
        match self {
            SensorRole::Primary => "primary",
            SensorRole::Secondary => "secondary",
            SensorRole::Reference => "reference",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Disposition {
    Release,
    Hold,
    Reject,
}

impl Disposition {
    fn all() -> [Disposition; DISPOSITION_GATES] {
        [Disposition::Release, Disposition::Hold, Disposition::Reject]
    }

    fn index(self) -> usize {
        match self {
            Disposition::Release => 0,
            Disposition::Hold => 1,
            Disposition::Reject => 2,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Disposition::Release => "release",
            Disposition::Hold => "hold",
            Disposition::Reject => "reject",
        }
    }

    fn gate_height(self) -> f64 {
        match self {
            Disposition::Release => 20.0,
            Disposition::Hold => 35.0,
            Disposition::Reject => 50.0,
        }
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design();

    let deck = containment_deck();
    export(&deck, OUTPUTS[0]);

    let sensors = redundant_sensor_dock_array();
    export(&sensors, OUTPUTS[1]);

    let manifold = surrogate_lane_manifold();
    export(&manifold, OUTPUTS[2]);

    let restrictors = restrictor_cartridge_bank();
    export(&restrictors, OUTPUTS[3]);

    let challenges = bubble_viscosity_challenge_wells();
    export(&challenges, OUTPUTS[4]);

    let pressure = pressure_tap_bosses();
    export(&pressure, OUTPUTS[5]);

    let flush = bypass_flush_route_panel();
    export(&flush, OUTPUTS[6]);

    let tokens = timestamp_alarm_token_rail();
    export(&tokens, OUTPUTS[7]);

    let custody = barcode_coa_custody_lands();
    export(&custody, OUTPUTS[8]);

    let disposition = release_hold_reject_gates();
    export(&disposition, OUTPUTS[9]);

    let bridge = evidence_camera_bridge_keepouts();
    export(&bridge, OUTPUTS[10]);

    let assembly = deck
        + sensors
        + manifold
        + restrictors
        + challenges
        + pressure
        + flush
        + tokens
        + custody
        + disposition
        + bridge;
    export(&assembly, OUTPUTS[11]);

    println!(
        "Closed perfusion flow-sensor drift redundancy voting station: {STATION_X:.0}mm x {STATION_Y:.0}mm contained deck, {LANES}-lane surrogate manifold with {ACTIVE_LANES} active challenge lanes, {SENSOR_DOCKS} primary/secondary/reference sensor docks, {RESTRICTOR_CARTRIDGES} restrictor cartridges, {CHALLENGE_WELLS} bubble/viscosity wells, and {PRESSURE_TAPS} pressure tap bosses."
    );
    println!(
        "Evidence features: {TOKEN_COUNT} timestamp/alarm tokens, {BARCODE_LANDS} barcode lands, {COA_LANDS} COA custody lands, {DISPOSITION_GATES} release/hold/reject gates, {CAMERA_TARGETS} camera targets, {KEEP_OUT_GAUGES} keepout gauges, {} limitations, and {} required feature groups.",
        LIMITATIONS.len(),
        REQUIRED_FEATURES.len()
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn sensor_triplet_x(role: SensorRole) -> f64 {
    centered_index(role.index(), SENSOR_COLUMNS, SENSOR_COLUMN_PITCH_X)
}

fn lane_y(lane: usize) -> f64 {
    centered_index(lane, LANES, LANE_PITCH_Y)
}

fn active_lane_range() -> std::ops::Range<usize> {
    let start = (LANES - ACTIVE_LANES) / 2;
    start..start + ACTIVE_LANES
}

fn assert_design() {
    assert_eq!(OUTPUTS.len(), REQUIRED_FEATURES.len() + 1);
    assert!(OUTPUTS.iter().all(|path| path.starts_with(OUTPUT_PREFIX)));
    assert_eq!(SENSOR_DOCKS, SENSOR_COLUMNS * SENSOR_ROWS);
    assert_eq!(RESTRICTOR_CARTRIDGES, RESTRICTOR_ROWS * RESTRICTOR_COLS);
    assert_eq!(CHALLENGE_WELLS, BUBBLE_WELLS + VISCOSITY_WELLS);
    assert_eq!(
        PRESSURE_TAPS,
        SENSOR_TRIPLETS * PRESSURE_TAPS_PER_SENSOR_TRIPLET
    );
    assert_eq!(TOKEN_COUNT, TIMESTAMP_TOKENS + ALARM_TOKENS);
    assert_eq!(DISPOSITION_GATES, Disposition::all().len());
    assert_eq!(SENSOR_COLUMNS, SensorRole::all().len());
    assert_eq!(DATUM_COUNT, 6);
    assert!(ACTIVE_LANES <= LANES);
    assert!(ROBOT_SWEEP_CLEARANCE_Z > SENSOR_LIFT_CLEARANCE_Z);
    assert!(SERVICE_CLEARANCE_FRONT_Y > SERVICE_CLEARANCE_REAR_Y);

    let zones = layout_zones();
    for zone in zones {
        assert!(
            zone.fits_station(),
            "{} exceeds usable station envelope",
            zone.name
        );
    }
    for (i, left) in zones.iter().enumerate() {
        for right in zones.iter().skip(i + 1) {
            assert!(
                !left.overlaps(*right, 6.0),
                "{} overlaps {}",
                left.name,
                right.name
            );
        }
    }
}

fn layout_zones() -> [Zone; 10] {
    [
        Zone {
            name: "sensor_dock_array",
            center: SENSOR_POS,
            x: SENSOR_BANK_X,
            y: SENSOR_BANK_Y,
        },
        Zone {
            name: "surrogate_lane_manifold",
            center: MANIFOLD_POS,
            x: MANIFOLD_X,
            y: MANIFOLD_Y,
        },
        Zone {
            name: "restrictor_cartridge_bank",
            center: RESTRICTOR_POS,
            x: RESTRICTOR_BANK_X,
            y: RESTRICTOR_BANK_Y,
        },
        Zone {
            name: "bubble_viscosity_challenge_wells",
            center: CHALLENGE_POS,
            x: CHALLENGE_X,
            y: CHALLENGE_Y,
        },
        Zone {
            name: "pressure_tap_bosses",
            center: PRESSURE_POS,
            x: PRESSURE_PANEL_X,
            y: PRESSURE_PANEL_Y,
        },
        Zone {
            name: "bypass_flush_route_panel",
            center: FLUSH_POS,
            x: FLUSH_PANEL_X,
            y: FLUSH_PANEL_Y,
        },
        Zone {
            name: "timestamp_alarm_token_rail",
            center: TOKEN_POS,
            x: TOKEN_RAIL_X,
            y: TOKEN_RAIL_Y,
        },
        Zone {
            name: "barcode_coa_custody_lands",
            center: CUSTODY_POS,
            x: CUSTODY_X,
            y: CUSTODY_Y,
        },
        Zone {
            name: "release_hold_reject_gates",
            center: DISPOSITION_POS,
            x: DISPOSITION_X,
            y: DISPOSITION_Y,
        },
        Zone {
            name: "evidence_camera_bridge_keepouts",
            center: BRIDGE_POS,
            x: BRIDGE_X,
            y: BRIDGE_Y,
        },
    ]
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        format!("{PREFIX}_containment_deck_plate"),
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let basin = centered_cube(
        format!("{PREFIX}_single_piece_leak_basin_recess"),
        STATION_X - 112.0,
        STATION_Y - 112.0,
        BASIN_DEPTH,
    )
    .translate(0.0, -10.0, BASE_Z - BASIN_DEPTH / 2.0 + 0.2);
    let upper_recess = centered_cube(
        format!("{PREFIX}_manifold_restrictor_wet_zone_recess"),
        1190.0,
        560.0,
        5.0,
    )
    .translate(0.0, 110.0, BASE_Z - 2.3);
    let lower_recess = centered_cube(
        format!("{PREFIX}_evidence_disposition_zone_recess"),
        1190.0,
        190.0,
        5.0,
    )
    .translate(0.0, -334.0, BASE_Z - 2.3);
    let drain = centered_cylinder(format!("{PREFIX}_front_low_point_drain"), 9.0, 54.0, 32)
        .rotate(90.0, 0.0, 0.0)
        .translate(
            STATION_X / 2.0 - 78.0,
            -(STATION_Y / 2.0 - 9.0),
            BASE_Z - 4.0,
        );

    deck - basin - upper_recess - lower_recess - drain - mounting_holes()
        + containment_rims()
        + zone_dividers()
        + wetness_gutters()
        + datum_fiducials()
}

fn mounting_holes() -> Part {
    let mut holes = Part::empty(format!("{PREFIX}_mounting_holes"));
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 58.0), -(STATION_Y / 2.0 - 54.0)),
        (STATION_X / 2.0 - 58.0, -(STATION_Y / 2.0 - 54.0)),
        (-(STATION_X / 2.0 - 58.0), STATION_Y / 2.0 - 54.0),
        (STATION_X / 2.0 - 58.0, STATION_Y / 2.0 - 54.0),
        (0.0, STATION_Y / 2.0 - 54.0),
        (0.0, -(STATION_Y / 2.0 - 54.0)),
        (-(STATION_X / 2.0 - 58.0), 0.0),
        (STATION_X / 2.0 - 58.0, 0.0),
    ]
    .iter()
    .copied()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("{PREFIX}_m6_clearance_mount_hole_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 8.0,
                28,
            )
            .translate(x, y, BASE_Z / 2.0);
    }
    holes
}

fn containment_rims() -> Part {
    let z = BASE_Z + RIM_Z / 2.0;
    let left = centered_cube(
        format!("{PREFIX}_left_containment_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-(STATION_X / 2.0 - RIM_W / 2.0), 0.0, z);
    let right = centered_cube(
        format!("{PREFIX}_right_containment_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, z);
    let rear = centered_cube(
        format!("{PREFIX}_rear_containment_rim"),
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, z);
    let front = centered_cube(
        format!("{PREFIX}_front_low_robot_access_lip"),
        STATION_X - 170.0,
        RIM_W,
        RIM_Z * 0.55,
    )
    .translate(
        0.0,
        -(STATION_Y / 2.0 - RIM_W / 2.0),
        BASE_Z + RIM_Z * 0.275,
    );

    left + right + rear + front
}

fn zone_dividers() -> Part {
    let manifold_to_lower = centered_cube(
        format!("{PREFIX}_wet_zone_to_evidence_zone_divider"),
        STATION_X - 170.0,
        12.0,
        32.0,
    )
    .translate(0.0, -270.0, BASE_Z + 16.0);
    let upper_to_flush = centered_cube(
        format!("{PREFIX}_manifold_to_flush_token_divider"),
        STATION_X - 170.0,
        12.0,
        30.0,
    )
    .translate(0.0, 278.0, BASE_Z + 15.0);
    let sensor_to_pressure = centered_cube(
        format!("{PREFIX}_sensor_to_pressure_triplet_divider"),
        10.0,
        244.0,
        28.0,
    )
    .translate(-232.0, -176.0, BASE_Z + 14.0);
    let pressure_to_challenge = centered_cube(
        format!("{PREFIX}_pressure_to_challenge_divider"),
        10.0,
        244.0,
        28.0,
    )
    .translate(196.0, -176.0, BASE_Z + 14.0);
    let lower_gate_divider = centered_cube(
        format!("{PREFIX}_custody_to_disposition_camera_divider"),
        10.0,
        112.0,
        28.0,
    )
    .translate(-270.0, -342.0, BASE_Z + 14.0);
    let camera_gate_divider = centered_cube(
        format!("{PREFIX}_disposition_to_camera_bridge_divider"),
        10.0,
        112.0,
        28.0,
    )
    .translate(178.0, -342.0, BASE_Z + 14.0);

    manifold_to_lower
        + upper_to_flush
        + sensor_to_pressure
        + pressure_to_challenge
        + lower_gate_divider
        + camera_gate_divider
}

fn wetness_gutters() -> Part {
    let sensor_gutter = centered_cube(format!("{PREFIX}_sensor_bank_leak_gutter"), 8.0, 244.0, 6.0)
        .translate(-155.0, -176.0, BASE_Z + 3.0);
    let manifold_gutter = centered_cube(
        format!("{PREFIX}_lane_manifold_cross_coupling_gutter"),
        662.0,
        8.0,
        6.0,
    )
    .translate(-205.0, -118.0, BASE_Z + 3.0);
    let challenge_gutter = centered_cube(
        format!("{PREFIX}_bubble_viscosity_overflow_gutter"),
        8.0,
        240.0,
        6.0,
    )
    .translate(268.0, -118.0, BASE_Z + 3.0);
    let front_gutter = centered_cube(
        format!("{PREFIX}_front_evidence_drain_gutter"),
        1090.0,
        8.0,
        6.0,
    )
    .translate(0.0, -402.0, BASE_Z + 3.0);

    sensor_gutter + manifold_gutter + challenge_gutter + front_gutter
}

fn datum_fiducials() -> Part {
    let mut datums = Part::empty(format!("{PREFIX}_datum_fiducials"));
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 82.0), STATION_Y / 2.0 - 78.0),
        (STATION_X / 2.0 - 82.0, STATION_Y / 2.0 - 78.0),
        (-(STATION_X / 2.0 - 82.0), -(STATION_Y / 2.0 - 78.0)),
        (STATION_X / 2.0 - 82.0, -(STATION_Y / 2.0 - 78.0)),
        (0.0, STATION_Y / 2.0 - 78.0),
        (0.0, -(STATION_Y / 2.0 - 78.0)),
    ]
    .iter()
    .copied()
    .enumerate()
    {
        datums = datums + fiducial_cross(format!("{PREFIX}_datum_fiducial_{i}"), x, y);
    }
    datums
}

fn fiducial_cross(name: String, x: f64, y: f64) -> Part {
    (centered_cube(format!("{name}_x_bar"), 26.0, 3.0, 5.0)
        + centered_cube(format!("{name}_y_bar"), 3.0, 26.0, 5.0)
        + centered_cylinder(format!("{name}_center_dot"), 3.0, 5.0, 24))
    .translate(x, y, BASE_Z + 2.5)
}

fn redundant_sensor_dock_array() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_redundant_sensor_dock_array_body"),
        SENSOR_BANK_X,
        SENSOR_BANK_Y,
        SENSOR_BANK_Z,
    )
    .translate(SENSOR_POS.0, SENSOR_POS.1, BASE_Z + SENSOR_BANK_Z / 2.0);
    let title = centered_cube(
        format!("{PREFIX}_primary_secondary_reference_header_land"),
        SENSOR_BANK_X - 48.0,
        16.0,
        6.0,
    )
    .translate(
        SENSOR_POS.0,
        SENSOR_POS.1 + SENSOR_BANK_Y / 2.0 - 24.0,
        BASE_Z + SENSOR_BANK_Z + 3.0,
    );

    let mut pockets = Part::empty(format!("{PREFIX}_sensor_dock_pocket_cutouts"));
    let mut features = Part::empty(format!("{PREFIX}_sensor_dock_alignment_features"));
    for role in SensorRole::all() {
        let x = SENSOR_POS.0 + sensor_triplet_x(role);
        let label_land = centered_cube(
            format!("{PREFIX}_{}_sensor_role_label_land", role.label()),
            SENSOR_DOCK_X,
            12.0,
            5.0,
        )
        .translate(
            x,
            SENSOR_POS.1 - SENSOR_BANK_Y / 2.0 + 18.0,
            BASE_Z + SENSOR_BANK_Z + 2.5,
        );
        features = features + label_land;

        for row in 0..SENSOR_ROWS {
            let y = SENSOR_POS.1 + centered_index(row, SENSOR_ROWS, SENSOR_ROW_PITCH_Y) + 10.0;
            let pocket = centered_cube(
                format!("{PREFIX}_{}_flow_sensor_dock_pocket_{row}", role.label()),
                SENSOR_DOCK_X + SENSOR_CLEARANCE,
                SENSOR_DOCK_Y + SENSOR_CLEARANCE,
                SENSOR_DOCK_Z + 2.0,
            )
            .translate(x, y, BASE_Z + SENSOR_BANK_Z - SENSOR_DOCK_Z / 2.0 + 1.0);
            let bore = centered_cylinder(
                format!("{PREFIX}_{}_flow_tube_bore_{row}", role.label()),
                5.0,
                SENSOR_DOCK_X + 20.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(x, y, BASE_Z + SENSOR_BANK_Z - 5.0);
            pockets = pockets + pocket + bore;

            features = features
                + centered_cube(
                    format!("{PREFIX}_{}_dock_front_key_{row}", role.label()),
                    SENSOR_DOCK_X * 0.72,
                    4.0,
                    7.0,
                )
                .translate(
                    x,
                    y - SENSOR_DOCK_Y / 2.0 - 5.0,
                    BASE_Z + SENSOR_BANK_Z + 3.5,
                )
                + centered_cube(
                    format!("{PREFIX}_{}_dock_rear_key_{row}", role.label()),
                    SENSOR_DOCK_X * 0.72,
                    4.0,
                    7.0,
                )
                .translate(
                    x,
                    y + SENSOR_DOCK_Y / 2.0 + 5.0,
                    BASE_Z + SENSOR_BANK_Z + 3.5,
                );
        }
    }

    body + title + features - pockets
}

fn surrogate_lane_manifold() -> Part {
    let deck = centered_cube(
        format!("{PREFIX}_surrogate_20_lane_manifold_plate"),
        MANIFOLD_X,
        MANIFOLD_Y,
        MANIFOLD_Z,
    )
    .translate(MANIFOLD_POS.0, MANIFOLD_POS.1, BASE_Z + MANIFOLD_Z / 2.0);
    let inlet_header = centered_cylinder(
        format!("{PREFIX}_inlet_header_witness_tube"),
        LANE_HEADER_D / 2.0,
        MANIFOLD_Y - 58.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        MANIFOLD_POS.0 - LANE_TRACE_X / 2.0 - 24.0,
        MANIFOLD_POS.1,
        BASE_Z + MANIFOLD_Z + 10.0,
    );
    let outlet_header = centered_cylinder(
        format!("{PREFIX}_outlet_header_witness_tube"),
        LANE_HEADER_D / 2.0,
        MANIFOLD_Y - 58.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        MANIFOLD_POS.0 + LANE_TRACE_X / 2.0 + 24.0,
        MANIFOLD_POS.1,
        BASE_Z + MANIFOLD_Z + 10.0,
    );

    let mut traces = Part::empty(format!("{PREFIX}_surrogate_lane_traces"));
    for lane in 0..LANES {
        let y = MANIFOLD_POS.1 + lane_y(lane);
        let active = active_lane_range().contains(&lane);
        let trace_z = if active {
            LANE_TRACE_Z
        } else {
            LANE_TRACE_Z * 0.65
        };
        let trace = centered_cube(
            format!("{PREFIX}_surrogate_lane_{lane:02}_flow_trace"),
            LANE_TRACE_X,
            LANE_TRACE_Y,
            trace_z,
        )
        .translate(MANIFOLD_POS.0, y, BASE_Z + MANIFOLD_Z + trace_z / 2.0);
        let inlet_stub = centered_cube(
            format!("{PREFIX}_surrogate_lane_{lane:02}_inlet_stub"),
            34.0,
            LANE_TRACE_Y,
            trace_z,
        )
        .translate(
            MANIFOLD_POS.0 - LANE_TRACE_X / 2.0 - 17.0,
            y,
            BASE_Z + MANIFOLD_Z + trace_z / 2.0,
        );
        let outlet_stub = centered_cube(
            format!("{PREFIX}_surrogate_lane_{lane:02}_outlet_stub"),
            34.0,
            LANE_TRACE_Y,
            trace_z,
        )
        .translate(
            MANIFOLD_POS.0 + LANE_TRACE_X / 2.0 + 17.0,
            y,
            BASE_Z + MANIFOLD_Z + trace_z / 2.0,
        );
        let status_land = centered_cube(
            format!("{PREFIX}_surrogate_lane_{lane:02}_active_or_guard_status_land"),
            34.0,
            10.0,
            5.0,
        )
        .translate(
            MANIFOLD_POS.0 + LANE_TRACE_X / 2.0 - 28.0,
            y,
            BASE_Z + MANIFOLD_Z + trace_z + 2.5,
        );
        traces = traces + trace + inlet_stub + outlet_stub + status_land;
    }

    let mut sentinels = Part::empty(format!("{PREFIX}_cross_coupling_sentinel_bridges"));
    for i in 0..CROSS_COUPLING_SENTINELS {
        let x = MANIFOLD_POS.0 + centered_index(i, CROSS_COUPLING_SENTINELS, 155.0);
        sentinels = sentinels
            + centered_cube(
                format!("{PREFIX}_cross_coupling_sentinel_bridge_{i}"),
                18.0,
                MANIFOLD_Y - 96.0,
                6.0,
            )
            .translate(x, MANIFOLD_POS.1, BASE_Z + MANIFOLD_Z + 20.0);
    }

    deck + inlet_header + outlet_header + traces + sentinels
}

fn restrictor_cartridge_bank() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_calibrated_restrictor_cartridge_bank_body"),
        RESTRICTOR_BANK_X,
        RESTRICTOR_BANK_Y,
        RESTRICTOR_BANK_Z,
    )
    .translate(
        RESTRICTOR_POS.0,
        RESTRICTOR_POS.1,
        BASE_Z + RESTRICTOR_BANK_Z / 2.0,
    );
    let label = centered_cube(
        format!("{PREFIX}_restrictor_bank_not_metrology_standard_label_land"),
        RESTRICTOR_BANK_X - 38.0,
        14.0,
        5.0,
    )
    .translate(
        RESTRICTOR_POS.0,
        RESTRICTOR_POS.1 + RESTRICTOR_BANK_Y / 2.0 - 20.0,
        BASE_Z + RESTRICTOR_BANK_Z + 2.5,
    );

    let mut slots = Part::empty(format!("{PREFIX}_restrictor_cartridge_slot_cutouts"));
    let mut keys = Part::empty(format!("{PREFIX}_restrictor_cartridge_grade_keys"));
    for row in 0..RESTRICTOR_ROWS {
        for col in 0..RESTRICTOR_COLS {
            let idx = row * RESTRICTOR_COLS + col;
            let x = RESTRICTOR_POS.0 + centered_index(col, RESTRICTOR_COLS, RESTRICTOR_PITCH_X);
            let y =
                RESTRICTOR_POS.1 + centered_index(row, RESTRICTOR_ROWS, RESTRICTOR_PITCH_Y) - 8.0;
            slots = slots
                + centered_cube(
                    format!(
                        "{PREFIX}_restrictor_slot_{idx:02}_{}ul_min",
                        RESTRICTOR_GRADES_UL_MIN[idx]
                    ),
                    RESTRICTOR_SLOT_X,
                    RESTRICTOR_SLOT_Y,
                    18.0,
                )
                .translate(x, y, BASE_Z + RESTRICTOR_BANK_Z - 8.0)
                + centered_cylinder(
                    format!("{PREFIX}_restrictor_slot_{idx:02}_flow_bore"),
                    4.5,
                    RESTRICTOR_SLOT_X + 18.0,
                    24,
                )
                .rotate(0.0, 90.0, 0.0)
                .translate(x, y, BASE_Z + RESTRICTOR_BANK_Z - 6.0);

            let grade_tabs = 1 + (idx % 4);
            for tab in 0..grade_tabs {
                keys = keys
                    + centered_cube(
                        format!("{PREFIX}_restrictor_slot_{idx:02}_grade_key_tab_{tab}"),
                        6.0,
                        5.0,
                        6.0,
                    )
                    .translate(
                        x - RESTRICTOR_SLOT_X / 2.0 + 8.0 + tab as f64 * 9.0,
                        y + RESTRICTOR_SLOT_Y / 2.0 + 6.0,
                        BASE_Z + RESTRICTOR_BANK_Z + 3.0,
                    );
            }
        }
    }

    body + label + keys - slots
}

fn bubble_viscosity_challenge_wells() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_bubble_viscosity_challenge_well_body"),
        CHALLENGE_X,
        CHALLENGE_Y,
        CHALLENGE_Z,
    )
    .translate(CHALLENGE_POS.0, CHALLENGE_POS.1, BASE_Z + CHALLENGE_Z / 2.0);
    let divider = centered_cube(
        format!("{PREFIX}_bubble_to_viscosity_challenge_divider"),
        CHALLENGE_X - 42.0,
        8.0,
        22.0,
    )
    .translate(
        CHALLENGE_POS.0,
        CHALLENGE_POS.1,
        BASE_Z + CHALLENGE_Z + 11.0,
    );

    let mut wells = Part::empty(format!("{PREFIX}_challenge_well_cutouts"));
    let mut witness = Part::empty(format!("{PREFIX}_challenge_well_witness_features"));
    for i in 0..BUBBLE_WELLS {
        let x = CHALLENGE_POS.0 + centered_index(i, BUBBLE_WELLS, CHALLENGE_PITCH_X);
        let y = CHALLENGE_POS.1 + CHALLENGE_PITCH_Y / 2.0;
        wells = wells
            + centered_cylinder(
                format!("{PREFIX}_bubble_false_flow_challenge_well_{i}"),
                CHALLENGE_WELL_D / 2.0,
                CHALLENGE_Z + 4.0,
                36,
            )
            .translate(x, y, BASE_Z + CHALLENGE_Z / 2.0 + 3.0);
        witness = witness
            + centered_cube(
                format!("{PREFIX}_bubble_challenge_air_slug_marker_{i}"),
                18.0 + i as f64 * 4.0,
                5.0,
                5.0,
            )
            .translate(x, y - 28.0, BASE_Z + CHALLENGE_Z + 2.5);
    }
    for i in 0..VISCOSITY_WELLS {
        let x = CHALLENGE_POS.0 + centered_index(i, VISCOSITY_WELLS, CHALLENGE_PITCH_X);
        let y = CHALLENGE_POS.1 - CHALLENGE_PITCH_Y / 2.0;
        wells = wells
            + centered_cylinder(
                format!("{PREFIX}_viscosity_false_flow_challenge_well_{i}"),
                CHALLENGE_WELL_D / 2.0,
                CHALLENGE_Z + 4.0,
                36,
            )
            .translate(x, y, BASE_Z + CHALLENGE_Z / 2.0 + 3.0);
        witness = witness
            + centered_cube(
                format!("{PREFIX}_viscosity_challenge_grade_marker_{i}"),
                5.0,
                16.0 + i as f64 * 3.0,
                5.0,
            )
            .translate(x, y + 30.0, BASE_Z + CHALLENGE_Z + 2.5);
    }

    body + divider + witness - wells
}

fn pressure_tap_bosses() -> Part {
    let panel = centered_cube(
        format!("{PREFIX}_pressure_tap_boss_panel"),
        PRESSURE_PANEL_X,
        PRESSURE_PANEL_Y,
        PRESSURE_PANEL_Z,
    )
    .translate(
        PRESSURE_POS.0,
        PRESSURE_POS.1,
        BASE_Z + PRESSURE_PANEL_Z / 2.0,
    );
    let mut bosses = Part::empty(format!("{PREFIX}_pressure_tap_bosses"));
    let mut bores = Part::empty(format!("{PREFIX}_pressure_tap_bores"));
    for triplet in 0..SENSOR_TRIPLETS {
        let x = PRESSURE_POS.0 + centered_index(triplet, SENSOR_TRIPLETS, PRESSURE_TAP_PITCH_X);
        for tap in 0..PRESSURE_TAPS_PER_SENSOR_TRIPLET {
            let y = PRESSURE_POS.1
                + centered_index(tap, PRESSURE_TAPS_PER_SENSOR_TRIPLET, PRESSURE_TAP_PITCH_Y);
            bosses = bosses
                + centered_cylinder(
                    format!("{PREFIX}_triplet_{triplet}_pressure_tap_boss_{tap}"),
                    PRESSURE_TAP_D / 2.0 + 6.0,
                    16.0,
                    32,
                )
                .translate(x, y, BASE_Z + PRESSURE_PANEL_Z + 8.0);
            bores = bores
                + centered_cylinder(
                    format!("{PREFIX}_triplet_{triplet}_pressure_tap_bore_{tap}"),
                    PRESSURE_TAP_D / 2.0,
                    24.0,
                    28,
                )
                .translate(x, y, BASE_Z + PRESSURE_PANEL_Z + 8.0);
        }
    }
    let agreement_scale = centered_cube(
        format!("{PREFIX}_sensor_agreement_delta_scale_land"),
        PRESSURE_PANEL_X - 38.0,
        12.0,
        5.0,
    )
    .translate(
        PRESSURE_POS.0,
        PRESSURE_POS.1 - PRESSURE_PANEL_Y / 2.0 + 18.0,
        BASE_Z + PRESSURE_PANEL_Z + 2.5,
    );

    panel + bosses + agreement_scale - bores
}

fn bypass_flush_route_panel() -> Part {
    let panel = centered_cube(
        format!("{PREFIX}_bypass_flush_route_panel_body"),
        FLUSH_PANEL_X,
        FLUSH_PANEL_Y,
        FLUSH_PANEL_Z,
    )
    .translate(FLUSH_POS.0, FLUSH_POS.1, BASE_Z + FLUSH_PANEL_Z / 2.0);
    let mut routes = Part::empty(format!("{PREFIX}_bypass_flush_route_witness_lanes"));
    for state in 0..FLUSH_ROUTE_STATES {
        let y = FLUSH_POS.1 + centered_index(state, FLUSH_ROUTE_STATES, 32.0);
        routes = routes
            + centered_cube(
                format!("{PREFIX}_flush_route_state_{state}_witness_channel"),
                FLUSH_PANEL_X - 76.0,
                7.0,
                8.0,
            )
            .translate(FLUSH_POS.0, y, BASE_Z + FLUSH_PANEL_Z + 4.0);
    }

    let mut sockets = Part::empty(format!("{PREFIX}_flush_valve_socket_cutouts"));
    for i in 0..FLUSH_VALVE_SOCKETS {
        let x = FLUSH_POS.0 + centered_index(i, FLUSH_VALVE_SOCKETS, 58.0);
        sockets = sockets
            + centered_cylinder(format!("{PREFIX}_flush_valve_socket_{i}"), 12.0, 20.0, 32)
                .translate(x, FLUSH_POS.1 + 43.0, BASE_Z + FLUSH_PANEL_Z - 6.0);
    }
    let mut cups = Part::empty(format!("{PREFIX}_flush_waste_capture_cups"));
    for i in 0..FLUSH_WASTE_CUPS {
        let x = FLUSH_POS.0 + centered_index(i, FLUSH_WASTE_CUPS, 76.0);
        cups = cups
            + centered_cylinder(
                format!("{PREFIX}_flush_waste_capture_cup_{i}"),
                18.0,
                20.0,
                36,
            )
            .translate(x, FLUSH_POS.1 - 43.0, BASE_Z + FLUSH_PANEL_Z + 10.0);
    }

    panel + routes + cups - sockets
}

fn timestamp_alarm_token_rail() -> Part {
    let rail = centered_cube(
        format!("{PREFIX}_timestamp_alarm_token_rail_body"),
        TOKEN_RAIL_X,
        TOKEN_RAIL_Y,
        TOKEN_RAIL_Z,
    )
    .translate(TOKEN_POS.0, TOKEN_POS.1, BASE_Z + TOKEN_RAIL_Z / 2.0);
    let mut token_cups = Part::empty(format!("{PREFIX}_timestamp_alarm_token_cups"));
    let mut label_tabs = Part::empty(format!("{PREFIX}_timestamp_alarm_token_label_tabs"));
    for i in 0..TIMESTAMP_TOKENS {
        let x = TOKEN_POS.0 + centered_index(i, TIMESTAMP_TOKENS, TOKEN_PITCH_X);
        let y = TOKEN_POS.1 + 27.0;
        token_cups = token_cups
            + centered_cylinder(
                format!("{PREFIX}_timestamp_token_cup_{i}"),
                TOKEN_D / 2.0,
                18.0,
                32,
            )
            .translate(x, y, BASE_Z + TOKEN_RAIL_Z - 6.0);
        label_tabs = label_tabs
            + centered_cube(
                format!("{PREFIX}_timestamp_token_label_tab_{i}"),
                28.0,
                6.0,
                5.0,
            )
            .translate(x, y + 22.0, BASE_Z + TOKEN_RAIL_Z + 2.5);
    }
    for i in 0..ALARM_TOKENS {
        let x = TOKEN_POS.0 + centered_index(i, ALARM_TOKENS, TOKEN_PITCH_X);
        let y = TOKEN_POS.1 - 27.0;
        token_cups = token_cups
            + centered_cylinder(
                format!("{PREFIX}_alarm_token_cup_{i}"),
                TOKEN_D / 2.0,
                18.0,
                32,
            )
            .translate(x, y, BASE_Z + TOKEN_RAIL_Z - 6.0);
        label_tabs = label_tabs
            + centered_cube(
                format!("{PREFIX}_alarm_vote_token_label_tab_{i}"),
                28.0,
                6.0,
                5.0,
            )
            .translate(x, y - 22.0, BASE_Z + TOKEN_RAIL_Z + 2.5);
    }

    rail + label_tabs - token_cups
}

fn barcode_coa_custody_lands() -> Part {
    let plate = centered_cube(
        format!("{PREFIX}_barcode_coa_custody_land_plate"),
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    )
    .translate(CUSTODY_POS.0, CUSTODY_POS.1, BASE_Z + CUSTODY_Z / 2.0);
    let mut lands = Part::empty(format!("{PREFIX}_barcode_coa_custody_lands"));
    for i in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(format!("{PREFIX}_barcode_land_{i}"), 56.0, 14.0, 5.0).translate(
                CUSTODY_POS.0 + centered_index(i % 5, 5, 66.0),
                CUSTODY_POS.1 + if i < 5 { 29.0 } else { 7.0 },
                BASE_Z + CUSTODY_Z + 2.5,
            );
    }
    for i in 0..COA_LANDS {
        lands = lands
            + centered_cube(
                format!("{PREFIX}_coa_certificate_land_{i}"),
                72.0,
                18.0,
                5.0,
            )
            .translate(
                CUSTODY_POS.0 + centered_index(i, COA_LANDS, 84.0),
                CUSTODY_POS.1 - 19.0,
                BASE_Z + CUSTODY_Z + 2.5,
            );
    }
    for i in 0..CUSTODY_WITNESS_LANDS {
        lands = lands
            + centered_cube(
                format!("{PREFIX}_custody_witness_tick_land_{i}"),
                32.0,
                5.0,
                5.0,
            )
            .translate(
                CUSTODY_POS.0 + centered_index(i, CUSTODY_WITNESS_LANDS, 52.0),
                CUSTODY_POS.1 - 45.0,
                BASE_Z + CUSTODY_Z + 2.5,
            );
    }

    plate + lands
}

fn release_hold_reject_gates() -> Part {
    let base = centered_cube(
        format!("{PREFIX}_release_hold_reject_gate_base"),
        DISPOSITION_X,
        DISPOSITION_Y,
        DISPOSITION_Z,
    )
    .translate(
        DISPOSITION_POS.0,
        DISPOSITION_POS.1,
        BASE_Z + DISPOSITION_Z / 2.0,
    );
    let mut gates = Part::empty(format!("{PREFIX}_release_hold_reject_gate_features"));
    for disposition in Disposition::all() {
        let x = DISPOSITION_POS.0 + centered_index(disposition.index(), DISPOSITION_GATES, 128.0);
        let h = disposition.gate_height();
        gates = gates
            + centered_cube(
                format!("{PREFIX}_{}_decision_gate", disposition.label()),
                84.0,
                18.0,
                h,
            )
            .translate(
                x,
                DISPOSITION_POS.1 + 25.0,
                BASE_Z + DISPOSITION_Z + h / 2.0,
            )
            + centered_cube(
                format!("{PREFIX}_{}_decision_label_land", disposition.label()),
                82.0,
                14.0,
                5.0,
            )
            .translate(x, DISPOSITION_POS.1 - 42.0, BASE_Z + DISPOSITION_Z + 2.5);
        for slot in 0..STATUS_SLOTS_PER_GATE {
            gates = gates
                + centered_cube(
                    format!("{PREFIX}_{}_status_slot_{slot}", disposition.label()),
                    10.0,
                    24.0,
                    5.0,
                )
                .translate(
                    x + centered_index(slot, STATUS_SLOTS_PER_GATE, 12.0),
                    DISPOSITION_POS.1 - 7.0,
                    BASE_Z + DISPOSITION_Z + 2.5,
                );
        }
    }
    base + gates
}

fn evidence_camera_bridge_keepouts() -> Part {
    let left_post = centered_cube(
        format!("{PREFIX}_camera_bridge_left_post"),
        28.0,
        BRIDGE_Y,
        BRIDGE_Z,
    )
    .translate(
        BRIDGE_POS.0 - BRIDGE_X / 2.0 + 28.0,
        BRIDGE_POS.1,
        BASE_Z + BRIDGE_Z / 2.0,
    );
    let right_post = centered_cube(
        format!("{PREFIX}_camera_bridge_right_post"),
        28.0,
        BRIDGE_Y,
        BRIDGE_Z,
    )
    .translate(
        BRIDGE_POS.0 + BRIDGE_X / 2.0 - 28.0,
        BRIDGE_POS.1,
        BASE_Z + BRIDGE_Z / 2.0,
    );
    let beam = centered_cube(
        format!("{PREFIX}_camera_bridge_overhead_beam"),
        BRIDGE_X,
        26.0,
        30.0,
    )
    .translate(BRIDGE_POS.0, BRIDGE_POS.1, BASE_Z + BRIDGE_Z - 15.0);

    let mut targets = Part::empty(format!("{PREFIX}_evidence_camera_targets"));
    for i in 0..CAMERA_TARGETS {
        targets = targets
            + centered_cube(format!("{PREFIX}_camera_target_land_{i}"), 38.0, 16.0, 5.0).translate(
                BRIDGE_POS.0 + centered_index(i, CAMERA_TARGETS, 44.0),
                BRIDGE_POS.1 - 32.0,
                BASE_Z + BRIDGE_Z + 2.5,
            );
    }

    let mut keepouts = Part::empty(format!("{PREFIX}_robot_service_keepout_gauges"));
    for i in 0..KEEP_OUT_GAUGES {
        keepouts = keepouts
            + centered_cube(
                format!("{PREFIX}_keepout_height_gauge_{i}"),
                18.0,
                18.0,
                35.0 + i as f64 * 12.0,
            )
            .translate(
                BRIDGE_POS.0 + centered_index(i, KEEP_OUT_GAUGES, 40.0),
                BRIDGE_POS.1 + 34.0,
                BASE_Z + (35.0 + i as f64 * 12.0) / 2.0,
            );
    }
    let robot_sweep_bar = centered_cube(
        format!("{PREFIX}_robot_sweep_clearance_bar"),
        BRIDGE_X - 76.0,
        8.0,
        8.0,
    )
    .translate(
        BRIDGE_POS.0,
        BRIDGE_POS.1 + 50.0,
        BASE_Z + ROBOT_SWEEP_CLEARANCE_Z,
    );
    let service_clearance_bar = centered_cube(
        format!("{PREFIX}_front_service_clearance_bar"),
        BRIDGE_X - 76.0,
        8.0,
        8.0,
    )
    .translate(
        BRIDGE_POS.0,
        BRIDGE_POS.1 - 50.0,
        BASE_Z + SENSOR_LIFT_CLEARANCE_Z,
    );

    left_post + right_post + beam + targets + keepouts + robot_sweep_bar + service_clearance_bar
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_manifest_has_expected_prefix_and_count() {
        assert_eq!(OUTPUTS.len(), 12);
        assert_eq!(OUTPUTS.len(), REQUIRED_FEATURES.len() + 1);
        assert!(OUTPUTS.iter().all(|path| path.starts_with(OUTPUT_PREFIX)));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
    }

    #[test]
    fn redundant_sensor_counts_match_triplet_layout() {
        assert_eq!(SensorRole::all().len(), 3);
        assert_eq!(SENSOR_DOCKS, 15);
        assert_eq!(SENSOR_DOCKS, SENSOR_COLUMNS * SENSOR_ROWS);
        assert_eq!(
            sensor_triplet_x(SensorRole::Primary),
            -SENSOR_COLUMN_PITCH_X
        );
        assert_eq!(
            sensor_triplet_x(SensorRole::Reference),
            SENSOR_COLUMN_PITCH_X
        );
    }

    #[test]
    fn manifold_uses_twenty_lanes_with_centered_active_sixteen() {
        let active: Vec<usize> = active_lane_range().collect();
        assert_eq!(active.len(), ACTIVE_LANES);
        assert_eq!(active[0], 2);
        assert_eq!(active[ACTIVE_LANES - 1], 17);
        assert!((lane_y(0) + lane_y(LANES - 1)).abs() < f64::EPSILON);
    }

    #[test]
    fn restrictor_and_challenge_metadata_are_consistent() {
        assert_eq!(RESTRICTOR_GRADES_UL_MIN.len(), RESTRICTOR_CARTRIDGES);
        assert!(RESTRICTOR_GRADES_UL_MIN.windows(2).all(|w| w[0] <= w[1]));
        assert_eq!(CHALLENGE_WELLS, 12);
        assert_eq!(BUBBLE_WELLS, VISCOSITY_WELLS);
    }

    #[test]
    fn layout_zones_fit_without_overlap() {
        let zones = layout_zones();
        assert_eq!(zones.len(), 10);
        for zone in zones {
            assert!(zone.fits_station(), "{} must fit station", zone.name);
        }
        for (i, left) in zones.iter().enumerate() {
            for right in zones.iter().skip(i + 1) {
                assert!(
                    !left.overlaps(*right, 6.0),
                    "{} overlaps {}",
                    left.name,
                    right.name
                );
            }
        }
    }

    #[test]
    fn disposition_gates_encode_release_hold_reject_order() {
        let dispositions = Disposition::all();
        assert_eq!(dispositions[0].label(), "release");
        assert_eq!(dispositions[1].label(), "hold");
        assert_eq!(dispositions[2].label(), "reject");
        assert!(Disposition::Release.gate_height() < Disposition::Hold.gate_height());
        assert!(Disposition::Hold.gate_height() < Disposition::Reject.gate_height());
    }
}
