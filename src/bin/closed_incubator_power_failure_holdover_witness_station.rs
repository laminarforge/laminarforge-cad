use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed incubator power-failure thermal/gas holdover witness station.
//
// This standalone generator models a no-cell engineering validation fixture for
// observing incubator behavior during power loss and UPS recovery scenarios. It
// provides a sealed cassette thermal-mass nest, independent logger pockets,
// CO2/O2/RH probe mast, door-closed lockout token, passive thermal buffer
// coupons, gas-decay sample ports, alarm/UPS status lands, recovery handoff
// lanes, barcode/custody plate, condensate capture, evidence bridge, and
// robot/service keepout gauges. It is fixture CAD only; biological protocols,
// acceptance thresholds, alarm criteria, and recovery disposition decisions
// remain external validation controls.

const OUTPUT_PREFIX: &str = "closed_incubator_power_failure_holdover_witness_station";

const OUTPUTS: [&str; 14] = [
    "output/closed_incubator_power_failure_holdover_witness_station_base_containment_deck.stl",
    "output/closed_incubator_power_failure_holdover_witness_station_sealed_cassette_thermal_mass_nest.stl",
    "output/closed_incubator_power_failure_holdover_witness_station_independent_logger_pockets.stl",
    "output/closed_incubator_power_failure_holdover_witness_station_co2_o2_rh_probe_mast.stl",
    "output/closed_incubator_power_failure_holdover_witness_station_door_closed_lockout_token_lane.stl",
    "output/closed_incubator_power_failure_holdover_witness_station_passive_thermal_buffer_coupons.stl",
    "output/closed_incubator_power_failure_holdover_witness_station_gas_decay_sample_ports.stl",
    "output/closed_incubator_power_failure_holdover_witness_station_alarm_ups_status_lands.stl",
    "output/closed_incubator_power_failure_holdover_witness_station_recovery_handoff_lanes.stl",
    "output/closed_incubator_power_failure_holdover_witness_station_barcode_custody_plate.stl",
    "output/closed_incubator_power_failure_holdover_witness_station_condensate_capture_tray.stl",
    "output/closed_incubator_power_failure_holdover_witness_station_evidence_bridge.stl",
    "output/closed_incubator_power_failure_holdover_witness_station_robot_service_keepout_gauges.stl",
    "output/closed_incubator_power_failure_holdover_witness_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 13] = [
    "sealed_cassette_thermal_mass_nest",
    "independent_logger_pockets",
    "co2_o2_rh_probe_mast",
    "door_closed_lockout_token_lane",
    "passive_thermal_buffer_coupons",
    "gas_decay_sample_ports",
    "alarm_ups_status_lands",
    "recovery_handoff_lanes",
    "barcode_custody_plate",
    "condensate_capture_tray",
    "evidence_bridge",
    "robot_service_keepout_gauges",
    "named_stl_outputs",
];

const STATION_X: f64 = 1480.0;
const STATION_Y: f64 = 920.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const BASIN_DEPTH: f64 = 8.0;
const SOCKET_DEPTH: f64 = 6.0;
const MOUNT_HOLE_COUNT: usize = 8;
const DATUM_TARGET_COUNT: usize = 4;

const NEST_X: f64 = 520.0;
const NEST_Y: f64 = 360.0;
const NEST_Z: f64 = 46.0;
const NEST_POS: (f64, f64) = (-365.0, 115.0);
const CASSETTE_COLS: usize = 3;
const CASSETTE_ROWS: usize = 2;
const CASSETTE_COUNT: usize = CASSETTE_COLS * CASSETTE_ROWS;
const CASSETTE_PITCH_X: f64 = 150.0;
const CASSETTE_PITCH_Y: f64 = 126.0;
const CASSETTE_X: f64 = 116.0;
const CASSETTE_Y: f64 = 88.0;
const CASSETTE_Z: f64 = 28.0;
const THERMAL_PUCKS_PER_CASSETTE: usize = 4;
const THERMAL_PUCK_D: f64 = 28.0;

const LOGGER_X: f64 = 320.0;
const LOGGER_Y: f64 = 170.0;
const LOGGER_Z: f64 = 38.0;
const LOGGER_POS: (f64, f64) = (280.0, 240.0);
const LOGGER_POCKET_COUNT: usize = 6;
const LOGGER_POCKET_X: f64 = 64.0;
const LOGGER_POCKET_Y: f64 = 38.0;
const LOGGER_POCKET_Z: f64 = 19.0;

const MAST_PANEL_X: f64 = 260.0;
const MAST_PANEL_Y: f64 = 180.0;
const MAST_PANEL_Z: f64 = 26.0;
const MAST_POS: (f64, f64) = (570.0, 150.0);
const PROBE_CHANNEL_COUNT: usize = 3;
const PROBE_HEIGHT_TIERS: usize = 3;
const PROBE_SOCKET_D: f64 = 17.0;
const PROBE_MAST_HEIGHT: f64 = 170.0;
const PROBE_TIER_Z: [f64; PROBE_HEIGHT_TIERS] = [54.0, 104.0, 154.0];

const TOKEN_X: f64 = 300.0;
const TOKEN_Y: f64 = 80.0;
const TOKEN_Z: f64 = 22.0;
const TOKEN_POS: (f64, f64) = (275.0, -65.0);
const LOCKOUT_TOKEN_COUNT: usize = 5;
const TOKEN_D: f64 = 25.0;

const BUFFER_X: f64 = 500.0;
const BUFFER_Y: f64 = 120.0;
const BUFFER_Z: f64 = 30.0;
const BUFFER_POS: (f64, f64) = (-360.0, -260.0);
const BUFFER_COUPON_COUNT: usize = 10;
const BUFFER_COUPON_X: f64 = 42.0;
const BUFFER_COUPON_Y: f64 = 34.0;
const BUFFER_COUPON_Z: f64 = 14.0;
const BUFFER_COUPON_MASS_G: f64 = 48.0;

const SAMPLE_X: f64 = 300.0;
const SAMPLE_Y: f64 = 140.0;
const SAMPLE_Z: f64 = 34.0;
const SAMPLE_POS: (f64, f64) = (275.0, -245.0);
const GAS_SAMPLE_PORT_COUNT: usize = 6;
const SAMPLE_PORT_D: f64 = 18.0;
const SAMPLE_VIAL_LAND_D: f64 = 38.0;

const UPS_X: f64 = 260.0;
const UPS_Y: f64 = 100.0;
const UPS_Z: f64 = 24.0;
const UPS_POS: (f64, f64) = (570.0, -325.0);
const STATUS_LAND_COUNT: usize = 8;
const DRY_CONTACT_LAND_COUNT: usize = 4;

const HANDOFF_X: f64 = 500.0;
const HANDOFF_Y: f64 = 80.0;
const HANDOFF_Z: f64 = 24.0;
const HANDOFF_POS: (f64, f64) = (-360.0, -385.0);
const HANDOFF_LANE_COUNT: usize = 3;
const HANDOFF_SLOT_COUNT: usize = 9;

const BARCODE_X: f64 = 220.0;
const BARCODE_Y: f64 = 90.0;
const BARCODE_Z: f64 = 12.0;
const BARCODE_POS: (f64, f64) = (590.0, -145.0);
const BARCODE_LAND_COUNT: usize = 6;
const CUSTODY_TOKEN_COUNT: usize = 4;

const CONDENSATE_X: f64 = 520.0;
const CONDENSATE_Y: f64 = 120.0;
const CONDENSATE_Z: f64 = 28.0;
const CONDENSATE_POS: (f64, f64) = (-360.0, -125.0);
const CONDENSATE_CHANNEL_COUNT: usize = 6;
const CONDENSATE_CUP_COUNT: usize = 3;
const CONDENSATE_CHANNEL_DEPTH: f64 = 10.0;

const EVIDENCE_X: f64 = 1180.0;
const EVIDENCE_Y: f64 = 70.0;
const EVIDENCE_ANCHOR_Z: f64 = 16.0;
const EVIDENCE_POST_Z: f64 = 185.0;
const EVIDENCE_CROSSBAR_Z: f64 = 22.0;
const EVIDENCE_POS: (f64, f64) = (0.0, 390.0);
const CAMERA_LAND_COUNT: usize = 5;
const LIGHT_BAR_COUNT: usize = 2;

const KEEP_OUT_X: f64 = 1400.0;
const KEEP_OUT_Y: f64 = 860.0;
const KEEP_OUT_Z: f64 = 6.0;
const KEEP_OUT_GAUGE_COUNT: usize = 6;
const FRONT_ROBOT_CLEARANCE: f64 = 35.0;
const REAR_SERVICE_CLEARANCE: f64 = 35.0;
const SIDE_SERVICE_CLEARANCE: f64 = 32.0;
const VERTICAL_SERVICE_CLEARANCE: f64 = 260.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_deck(self) -> bool {
        let usable_x = STATION_X / 2.0 - RIM_W - 8.0;
        let usable_y = STATION_Y / 2.0 - RIM_W - 8.0;

        self.center.0.abs() + self.x / 2.0 <= usable_x
            && self.center.1.abs() + self.y / 2.0 <= usable_y
    }

    fn overlaps(self, other: Rect) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();

        dx < (self.x + other.x) / 2.0 && dy < (self.y + other.y) / 2.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum HandoffLane {
    Stabilize,
    Recover,
    Quarantine,
}

impl HandoffLane {
    fn all() -> [HandoffLane; HANDOFF_LANE_COUNT] {
        [
            HandoffLane::Stabilize,
            HandoffLane::Recover,
            HandoffLane::Quarantine,
        ]
    }

    fn index(self) -> usize {
        match self {
            HandoffLane::Stabilize => 0,
            HandoffLane::Recover => 1,
            HandoffLane::Quarantine => 2,
        }
    }

    fn name(self) -> &'static str {
        match self {
            HandoffLane::Stabilize => "stabilize",
            HandoffLane::Recover => "recover",
            HandoffLane::Quarantine => "quarantine",
        }
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_containment_deck();
    export(OUTPUTS[0], &base);

    let nest = sealed_cassette_thermal_mass_nest();
    export(OUTPUTS[1], &nest);

    let loggers = independent_logger_pockets();
    export(OUTPUTS[2], &loggers);

    let mast = co2_o2_rh_probe_mast();
    export(OUTPUTS[3], &mast);

    let tokens = door_closed_lockout_token_lane();
    export(OUTPUTS[4], &tokens);

    let buffers = passive_thermal_buffer_coupons();
    export(OUTPUTS[5], &buffers);

    let samples = gas_decay_sample_ports();
    export(OUTPUTS[6], &samples);

    let ups = alarm_ups_status_lands();
    export(OUTPUTS[7], &ups);

    let handoff = recovery_handoff_lanes();
    export(OUTPUTS[8], &handoff);

    let custody = barcode_custody_plate();
    export(OUTPUTS[9], &custody);

    let condensate = condensate_capture_tray();
    export(OUTPUTS[10], &condensate);

    let evidence = evidence_bridge();
    export(OUTPUTS[11], &evidence);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[12], &keepouts);

    let assembly = base
        + nest.translate(NEST_POS.0, NEST_POS.1, on_base_z(NEST_Z))
        + loggers.translate(LOGGER_POS.0, LOGGER_POS.1, on_base_z(LOGGER_Z))
        + mast.translate(MAST_POS.0, MAST_POS.1, on_base_z(MAST_PANEL_Z))
        + tokens.translate(TOKEN_POS.0, TOKEN_POS.1, on_base_z(TOKEN_Z))
        + buffers.translate(BUFFER_POS.0, BUFFER_POS.1, on_base_z(BUFFER_Z))
        + samples.translate(SAMPLE_POS.0, SAMPLE_POS.1, on_base_z(SAMPLE_Z))
        + ups.translate(UPS_POS.0, UPS_POS.1, on_base_z(UPS_Z))
        + handoff.translate(HANDOFF_POS.0, HANDOFF_POS.1, on_base_z(HANDOFF_Z))
        + custody.translate(BARCODE_POS.0, BARCODE_POS.1, on_base_z(BARCODE_Z))
        + condensate.translate(CONDENSATE_POS.0, CONDENSATE_POS.1, on_base_z(CONDENSATE_Z))
        + evidence.translate(
            EVIDENCE_POS.0,
            EVIDENCE_POS.1,
            BASE_Z + EVIDENCE_ANCHOR_Z / 2.0,
        )
        + keepouts.translate(0.0, 0.0, BASE_Z / 2.0 + KEEP_OUT_Z / 2.0);
    export(OUTPUTS[13], &assembly);

    println!();
    println!("Closed incubator power-failure holdover witness station:");
    println!("  Footprint:        {STATION_X:.0}mm x {STATION_Y:.0}mm containment deck");
    println!(
        "  Thermal nest:     {CASSETTE_COUNT} sealed cassette mass positions with {} indexed thermal pucks",
        total_thermal_puck_count()
    );
    println!(
        "  Independent logs: {LOGGER_POCKET_COUNT} logger pockets plus {PROBE_CHANNEL_COUNT} CO2/O2/RH probe channels across {PROBE_HEIGHT_TIERS} height tiers"
    );
    println!(
        "  Holdover witness: {BUFFER_COUPON_COUNT} passive buffer coupons, {GAS_SAMPLE_PORT_COUNT} gas decay sample ports, and {LOCKOUT_TOKEN_COUNT} door-closed lockout tokens"
    );
    println!(
        "  Recovery trace:   {STATUS_LAND_COUNT} alarm/UPS lands, {HANDOFF_SLOT_COUNT} recovery handoff slots, {BARCODE_LAND_COUNT} barcode lands, {CUSTODY_TOKEN_COUNT} custody tokens"
    );
    println!(
        "  Evidence:         {CONDENSATE_CHANNEL_COUNT} condensate channels, {CAMERA_LAND_COUNT} camera lands, {LIGHT_BAR_COUNT} light bars, {KEEP_OUT_GAUGE_COUNT} robot/service keepout gauges"
    );
    println!("  Required features: {}", REQUIRED_FEATURES.len());
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

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 14);
    assert_eq!(REQUIRED_FEATURES.len(), 13);
    assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
    assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
    assert_eq!(CASSETTE_COUNT, CASSETTE_COLS * CASSETTE_ROWS);
    assert_eq!(
        total_thermal_puck_count(),
        CASSETTE_COUNT * THERMAL_PUCKS_PER_CASSETTE
    );
    assert_eq!(LOGGER_POCKET_COUNT, CASSETTE_COUNT);
    assert_eq!(PROBE_HEIGHT_TIERS, PROBE_TIER_Z.len());
    assert_eq!(HandoffLane::all().len(), HANDOFF_LANE_COUNT);
    assert_eq!(HANDOFF_SLOT_COUNT, HANDOFF_LANE_COUNT * 3);
    assert_eq!(MOUNT_HOLE_COUNT, mount_hole_positions().len());
    assert_eq!(DATUM_TARGET_COUNT, datum_target_positions().len());
    assert!(thermal_buffer_total_mass_g() > 400.0);
    assert!(gas_sample_capacity_ml() >= 90.0);
    assert!(condensate_capture_capacity_ml() > expected_condensate_witness_ml());
    assert!(FRONT_ROBOT_CLEARANCE <= front_robot_clearance());
    assert!(REAR_SERVICE_CLEARANCE <= rear_service_clearance());
    assert!(SIDE_SERVICE_CLEARANCE <= side_service_clearance());
    assert!(VERTICAL_SERVICE_CLEARANCE < PROBE_MAST_HEIGHT + BASE_Z + 100.0);

    for item in socket_rects() {
        assert!(
            item.fits_inside_deck(),
            "{} exceeds containment deck",
            item.name
        );
    }

    let rects = socket_rects();
    for a in 0..rects.len() {
        for b in (a + 1)..rects.len() {
            assert!(
                !rects[a].overlaps(rects[b]),
                "{} overlaps {}",
                rects[a].name,
                rects[b].name
            );
        }
    }
}

fn socket_rects() -> [Rect; 11] {
    [
        rect(
            "sealed_cassette_thermal_mass_nest",
            NEST_POS,
            NEST_X,
            NEST_Y,
        ),
        rect("independent_logger_pockets", LOGGER_POS, LOGGER_X, LOGGER_Y),
        rect("co2_o2_rh_probe_mast", MAST_POS, MAST_PANEL_X, MAST_PANEL_Y),
        rect(
            "door_closed_lockout_token_lane",
            TOKEN_POS,
            TOKEN_X,
            TOKEN_Y,
        ),
        rect(
            "passive_thermal_buffer_coupons",
            BUFFER_POS,
            BUFFER_X,
            BUFFER_Y,
        ),
        rect("gas_decay_sample_ports", SAMPLE_POS, SAMPLE_X, SAMPLE_Y),
        rect("alarm_ups_status_lands", UPS_POS, UPS_X, UPS_Y),
        rect("recovery_handoff_lanes", HANDOFF_POS, HANDOFF_X, HANDOFF_Y),
        rect("barcode_custody_plate", BARCODE_POS, BARCODE_X, BARCODE_Y),
        rect(
            "condensate_capture_tray",
            CONDENSATE_POS,
            CONDENSATE_X,
            CONDENSATE_Y,
        ),
        rect("evidence_bridge", EVIDENCE_POS, EVIDENCE_X, EVIDENCE_Y),
    ]
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

fn datum_target_positions() -> [(f64, f64); DATUM_TARGET_COUNT] {
    [
        (-STATION_X / 2.0 + 108.0, -STATION_Y / 2.0 + 96.0),
        (STATION_X / 2.0 - 108.0, -STATION_Y / 2.0 + 96.0),
        (-STATION_X / 2.0 + 108.0, STATION_Y / 2.0 - 96.0),
        (STATION_X / 2.0 - 108.0, STATION_Y / 2.0 - 96.0),
    ]
}

fn total_thermal_puck_count() -> usize {
    CASSETTE_COUNT * THERMAL_PUCKS_PER_CASSETTE
}

fn thermal_buffer_total_mass_g() -> f64 {
    BUFFER_COUPON_COUNT as f64 * BUFFER_COUPON_MASS_G
}

fn gas_sample_capacity_ml() -> f64 {
    GAS_SAMPLE_PORT_COUNT as f64 * 18.0
}

fn condensate_capture_capacity_ml() -> f64 {
    (CONDENSATE_X - 70.0) * (CONDENSATE_Y - 44.0) * CONDENSATE_CHANNEL_DEPTH / 1000.0
}

fn expected_condensate_witness_ml() -> f64 {
    CONDENSATE_CHANNEL_COUNT as f64 * 8.0 + CONDENSATE_CUP_COUNT as f64 * 12.0
}

fn front_robot_clearance() -> f64 {
    STATION_Y / 2.0 - (HANDOFF_POS.1.abs() + HANDOFF_Y / 2.0)
}

fn rear_service_clearance() -> f64 {
    STATION_Y / 2.0 - (EVIDENCE_POS.1 + EVIDENCE_Y / 2.0)
}

fn side_service_clearance() -> f64 {
    STATION_X / 2.0 - (MAST_POS.0 + MAST_PANEL_X / 2.0)
}

fn base_containment_deck() -> Part {
    let deck = centered_cube(
        "power_failure_holdover_base_containment_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let basin = centered_cube(
        "power_failure_holdover_shallow_secondary_basin_cut",
        STATION_X - 2.0 * (RIM_W + 52.0),
        STATION_Y - 2.0 * (RIM_W + 48.0),
        BASIN_DEPTH + 0.6,
    )
    .translate(0.0, -8.0, BASE_Z - BASIN_DEPTH / 2.0);

    deck - basin - insert_sockets() - deck_mount_holes()
        + perimeter_rims()
        + workflow_dividers()
        + robot_datum_targets()
        + evidence_anchor_lands()
        + base_holdover_witness_ribs()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("power_failure_holdover_insert_sockets");
    for item in socket_rects() {
        sockets = sockets
            + centered_cube(
                format!("power_failure_holdover_{}_socket", item.name),
                item.x + 9.0,
                item.y + 9.0,
                SOCKET_DEPTH + 0.5,
            )
            .translate(item.center.0, item.center.1, BASE_Z - SOCKET_DEPTH / 2.0);
    }
    sockets
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("power_failure_holdover_mount_holes");
    for (i, (x, y)) in mount_hole_positions().into_iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("power_failure_holdover_m6_clearance_hole_{i}"),
                3.4,
                BASE_Z + 5.0,
                28,
            )
            .translate(x, y, BASE_Z / 2.0)
            + centered_cube(
                format!("power_failure_holdover_service_slot_{i}"),
                30.0,
                7.2,
                BASE_Z + 5.0,
            )
            .translate(x, y, BASE_Z / 2.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    let front = centered_cube(
        "power_failure_holdover_front_low_robot_rim",
        STATION_X,
        RIM_W,
        28.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, BASE_Z + 14.0);
    let rear = centered_cube(
        "power_failure_holdover_rear_service_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "power_failure_holdover_left_spill_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "power_failure_holdover_right_spill_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);

    front + rear + left + right
}

fn workflow_dividers() -> Part {
    let upper = centered_cube(
        "power_failure_holdover_measurement_zone_divider",
        STATION_X - 220.0,
        10.0,
        24.0,
    )
    .translate(0.0, 335.0, BASE_Z + 12.0);
    let middle = centered_cube(
        "power_failure_holdover_thermal_gas_witness_zone_divider",
        STATION_X - 220.0,
        10.0,
        24.0,
    )
    .translate(0.0, -65.0, BASE_Z + 12.0);
    let lower = centered_cube(
        "power_failure_holdover_recovery_trace_zone_divider",
        STATION_X - 250.0,
        10.0,
        22.0,
    )
    .translate(0.0, -340.0, BASE_Z + 11.0);
    let center = centered_cube(
        "power_failure_holdover_left_right_workflow_divider",
        12.0,
        STATION_Y - 190.0,
        24.0,
    )
    .translate(0.0, -20.0, BASE_Z + 12.0);

    upper + middle + lower + center
}

fn robot_datum_targets() -> Part {
    let mut targets = Part::empty("power_failure_holdover_robot_datum_targets");
    for (i, (x, y)) in datum_target_positions().into_iter().enumerate() {
        targets =
            targets
                + fiducial_disc(&format!("power_failure_holdover_robot_datum_target_{i}"))
                    .translate(x, y, BASE_Z + 2.5);
    }
    targets
}

fn evidence_anchor_lands() -> Part {
    let left = centered_cube(
        "power_failure_holdover_evidence_bridge_left_anchor_land",
        96.0,
        42.0,
        8.0,
    )
    .translate(
        EVIDENCE_POS.0 - EVIDENCE_X / 2.0 + 64.0,
        EVIDENCE_POS.1,
        BASE_Z + 4.0,
    );
    let right = centered_cube(
        "power_failure_holdover_evidence_bridge_right_anchor_land",
        96.0,
        42.0,
        8.0,
    )
    .translate(
        EVIDENCE_POS.0 + EVIDENCE_X / 2.0 - 64.0,
        EVIDENCE_POS.1,
        BASE_Z + 4.0,
    );

    left + right
}

fn base_holdover_witness_ribs() -> Part {
    let mut ribs = Part::empty("power_failure_holdover_base_time_witness_ribs");
    for (i, y) in [-392.0, -300.0, -210.0, -120.0, -30.0, 75.0, 185.0, 295.0]
        .into_iter()
        .enumerate()
    {
        ribs = ribs
            + centered_cube(
                format!("power_failure_holdover_elapsed_time_witness_rib_{i}"),
                STATION_X - 260.0,
                4.0,
                5.0,
            )
            .translate(0.0, y, BASE_Z + 2.5);
    }
    ribs
}

fn sealed_cassette_thermal_mass_nest() -> Part {
    let tray = centered_cube(
        "power_failure_holdover_sealed_cassette_thermal_mass_nest_tray",
        NEST_X,
        NEST_Y,
        NEST_Z,
    );
    let relief = centered_cube(
        "power_failure_holdover_thermal_nest_inner_air_gap_relief",
        NEST_X - 56.0,
        NEST_Y - 50.0,
        10.0,
    )
    .translate(0.0, 0.0, NEST_Z / 2.0 - 5.0);

    tray - relief - cassette_recesses() - thermal_puck_reliefs()
        + cassette_surrogate_blocks()
        + thermal_reference_pucks()
        + nest_gasket_frames()
        + nest_lift_handles()
}

fn cassette_recesses() -> Part {
    let mut recesses = Part::empty("power_failure_holdover_cassette_recesses");
    for slot in 0..CASSETTE_COUNT {
        let (x, y) = cassette_center(slot);
        recesses = recesses
            + centered_cube(
                format!("power_failure_holdover_cassette_recess_{slot}"),
                CASSETTE_X + 16.0,
                CASSETTE_Y + 16.0,
                18.0,
            )
            .translate(x, y, NEST_Z / 2.0 - 8.0);
    }
    recesses
}

fn thermal_puck_reliefs() -> Part {
    let mut reliefs = Part::empty("power_failure_holdover_thermal_puck_reliefs");
    for slot in 0..CASSETTE_COUNT {
        let (cx, cy) = cassette_center(slot);
        for (puck, (dx, dy)) in puck_offsets().into_iter().enumerate() {
            reliefs = reliefs
                + centered_cylinder(
                    format!("power_failure_holdover_slot_{slot}_thermal_puck_relief_{puck}"),
                    THERMAL_PUCK_D / 2.0 + 2.0,
                    12.0,
                    32,
                )
                .translate(cx + dx, cy + dy, NEST_Z / 2.0 - 6.0);
        }
    }
    reliefs
}

fn cassette_surrogate_blocks() -> Part {
    let mut blocks = Part::empty("power_failure_holdover_sealed_cassette_surrogate_blocks");
    for slot in 0..CASSETTE_COUNT {
        let (x, y) = cassette_center(slot);
        blocks = blocks
            + centered_cube(
                format!("power_failure_holdover_sealed_cassette_surrogate_{slot}"),
                CASSETTE_X,
                CASSETTE_Y,
                CASSETTE_Z,
            )
            .translate(x, y, NEST_Z / 2.0 + CASSETTE_Z / 2.0 - 4.0)
            + centered_cube(
                format!("power_failure_holdover_cassette_lid_witness_land_{slot}"),
                CASSETTE_X - 18.0,
                CASSETTE_Y - 18.0,
                5.0,
            )
            .translate(x, y, NEST_Z / 2.0 + CASSETTE_Z + 0.5);
    }
    blocks
}

fn thermal_reference_pucks() -> Part {
    let mut pucks = Part::empty("power_failure_holdover_indexed_thermal_reference_pucks");
    for slot in 0..CASSETTE_COUNT {
        let (cx, cy) = cassette_center(slot);
        for (puck, (dx, dy)) in puck_offsets().into_iter().enumerate() {
            pucks = pucks
                + centered_cylinder(
                    format!("power_failure_holdover_slot_{slot}_thermal_reference_puck_{puck}"),
                    THERMAL_PUCK_D / 2.0,
                    10.0,
                    32,
                )
                .translate(cx + dx, cy + dy, NEST_Z / 2.0 + 5.0);
        }
    }
    pucks
}

fn nest_gasket_frames() -> Part {
    let mut frames = Part::empty("power_failure_holdover_cassette_gasket_witness_frames");
    for slot in 0..CASSETTE_COUNT {
        let (x, y) = cassette_center(slot);
        frames = frames
            + rectangular_frame(
                &format!("power_failure_holdover_slot_{slot}_gasket_witness_frame"),
                CASSETTE_X + 24.0,
                CASSETTE_Y + 24.0,
                7.0,
                5.0,
            )
            .translate(x, y, NEST_Z / 2.0 + 2.5);
    }
    frames
}

fn nest_lift_handles() -> Part {
    let front = centered_cube(
        "power_failure_holdover_thermal_nest_front_robot_handle",
        NEST_X - 80.0,
        16.0,
        20.0,
    )
    .translate(0.0, -NEST_Y / 2.0 + 22.0, NEST_Z / 2.0 + 10.0);
    let rear = centered_cube(
        "power_failure_holdover_thermal_nest_rear_service_handle",
        NEST_X - 80.0,
        16.0,
        20.0,
    )
    .translate(0.0, NEST_Y / 2.0 - 22.0, NEST_Z / 2.0 + 10.0);
    front + rear
}

fn cassette_center(slot: usize) -> (f64, f64) {
    let col = slot % CASSETTE_COLS;
    let row = slot / CASSETTE_COLS;
    (
        centered_index(col, CASSETTE_COLS, CASSETTE_PITCH_X),
        centered_index(row, CASSETTE_ROWS, CASSETTE_PITCH_Y),
    )
}

fn puck_offsets() -> [(f64, f64); THERMAL_PUCKS_PER_CASSETTE] {
    [(-34.0, -25.0), (34.0, -25.0), (-34.0, 25.0), (34.0, 25.0)]
}

fn independent_logger_pockets() -> Part {
    let plate = centered_cube(
        "power_failure_holdover_independent_logger_pocket_plate",
        LOGGER_X,
        LOGGER_Y,
        LOGGER_Z,
    );
    let mut pockets = Part::empty("power_failure_holdover_independent_logger_recesses");
    let mut details = Part::empty("power_failure_holdover_independent_logger_details");

    for i in 0..LOGGER_POCKET_COUNT {
        let col = i % 3;
        let row = i / 3;
        let x = centered_index(col, 3, 92.0);
        let y = centered_index(row, 2, 74.0);
        pockets = pockets
            + centered_cube(
                format!("power_failure_holdover_logger_pocket_cut_{i}"),
                LOGGER_POCKET_X,
                LOGGER_POCKET_Y,
                LOGGER_POCKET_Z,
            )
            .translate(x, y, LOGGER_Z / 2.0 - LOGGER_POCKET_Z / 2.0);
        details = details
            + centered_cube(
                format!("power_failure_holdover_logger_retainer_clip_{i}"),
                LOGGER_POCKET_X + 8.0,
                5.0,
                8.0,
            )
            .translate(x, y + LOGGER_POCKET_Y / 2.0 + 5.0, LOGGER_Z / 2.0 + 4.0)
            + fiducial_disc(&format!("power_failure_holdover_logger_index_fiducial_{i}"))
                .translate(
                    x - LOGGER_POCKET_X / 2.0 + 8.0,
                    y - 24.0,
                    LOGGER_Z / 2.0 + 2.5,
                );
    }

    plate - pockets + details + logger_cable_strain_relief_lands()
}

fn logger_cable_strain_relief_lands() -> Part {
    let mut lands = Part::empty("power_failure_holdover_logger_cable_strain_relief_lands");
    for (i, x) in [-120.0, -72.0, -24.0, 24.0, 72.0, 120.0]
        .into_iter()
        .enumerate()
    {
        lands = lands
            + centered_cube(
                format!("power_failure_holdover_logger_cable_tie_land_{i}"),
                32.0,
                12.0,
                8.0,
            )
            .translate(x, -LOGGER_Y / 2.0 + 16.0, LOGGER_Z / 2.0 + 4.0);
    }
    lands
}

fn co2_o2_rh_probe_mast() -> Part {
    let panel = centered_cube(
        "power_failure_holdover_co2_o2_rh_probe_mast_base_panel",
        MAST_PANEL_X,
        MAST_PANEL_Y,
        MAST_PANEL_Z,
    );
    let mut sockets = Part::empty("power_failure_holdover_probe_socket_cuts");
    let mut mast = Part::empty("power_failure_holdover_probe_mast_geometry");

    for channel in 0..PROBE_CHANNEL_COUNT {
        let x = centered_index(channel, PROBE_CHANNEL_COUNT, 78.0);
        sockets = sockets
            + centered_cylinder(
                format!("power_failure_holdover_probe_channel_{channel}_base_socket_cut"),
                PROBE_SOCKET_D / 2.0,
                MAST_PANEL_Z + 6.0,
                32,
            )
            .translate(x, -48.0, 0.0);
        mast = mast
            + centered_cylinder(
                format!("power_failure_holdover_probe_channel_{channel}_vertical_mast"),
                6.0,
                PROBE_MAST_HEIGHT,
                28,
            )
            .translate(x, -48.0, MAST_PANEL_Z / 2.0 + PROBE_MAST_HEIGHT / 2.0);

        for (tier, z) in PROBE_TIER_Z.into_iter().enumerate() {
            mast = mast
                + centered_cylinder(
                    format!("power_failure_holdover_probe_channel_{channel}_tier_{tier}_collar"),
                    14.0,
                    12.0,
                    28,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(x, -48.0, MAST_PANEL_Z / 2.0 + z)
                + centered_cube(
                    format!("power_failure_holdover_probe_channel_{channel}_tier_{tier}_flag"),
                    34.0,
                    4.0,
                    12.0,
                )
                .translate(x, -28.0, MAST_PANEL_Z / 2.0 + z);
        }
    }

    let species_lands = centered_cube(
        "power_failure_holdover_probe_species_land_co2",
        62.0,
        24.0,
        8.0,
    )
    .translate(-78.0, 48.0, MAST_PANEL_Z / 2.0 + 4.0)
        + centered_cube(
            "power_failure_holdover_probe_species_land_o2",
            62.0,
            24.0,
            8.0,
        )
        .translate(0.0, 48.0, MAST_PANEL_Z / 2.0 + 4.0)
        + centered_cube(
            "power_failure_holdover_probe_species_land_rh",
            62.0,
            24.0,
            8.0,
        )
        .translate(78.0, 48.0, MAST_PANEL_Z / 2.0 + 4.0);

    panel - sockets + mast + species_lands + mast_guard_rails()
}

fn mast_guard_rails() -> Part {
    let left = centered_cube(
        "power_failure_holdover_probe_mast_left_guard_rail",
        8.0,
        MAST_PANEL_Y - 28.0,
        46.0,
    )
    .translate(-MAST_PANEL_X / 2.0 + 18.0, 0.0, MAST_PANEL_Z / 2.0 + 23.0);
    let right = centered_cube(
        "power_failure_holdover_probe_mast_right_guard_rail",
        8.0,
        MAST_PANEL_Y - 28.0,
        46.0,
    )
    .translate(MAST_PANEL_X / 2.0 - 18.0, 0.0, MAST_PANEL_Z / 2.0 + 23.0);
    left + right
}

fn door_closed_lockout_token_lane() -> Part {
    let plate = centered_cube(
        "power_failure_holdover_door_closed_lockout_token_lane_plate",
        TOKEN_X,
        TOKEN_Y,
        TOKEN_Z,
    );
    let mut recesses = Part::empty("power_failure_holdover_lockout_token_recesses");
    let mut lands = Part::empty("power_failure_holdover_lockout_token_index_lands");
    for token in 0..LOCKOUT_TOKEN_COUNT {
        let x = centered_index(token, LOCKOUT_TOKEN_COUNT, 48.0);
        recesses = recesses
            + centered_cylinder(
                format!("power_failure_holdover_door_closed_token_recess_{token}"),
                TOKEN_D / 2.0,
                12.0,
                32,
            )
            .translate(x, -10.0, TOKEN_Z / 2.0 - 6.0);
        lands = lands
            + centered_cube(
                format!("power_failure_holdover_door_closed_token_tamper_land_{token}"),
                32.0,
                8.0,
                6.0,
            )
            .translate(x, 24.0, TOKEN_Z / 2.0 + 3.0);
    }
    let lock_bar = centered_cube(
        "power_failure_holdover_door_closed_lockout_crossbar",
        TOKEN_X - 40.0,
        10.0,
        18.0,
    )
    .translate(0.0, -TOKEN_Y / 2.0 + 12.0, TOKEN_Z / 2.0 + 9.0);

    plate - recesses + lands + lock_bar
}

fn passive_thermal_buffer_coupons() -> Part {
    let plate = centered_cube(
        "power_failure_holdover_passive_thermal_buffer_coupon_plate",
        BUFFER_X,
        BUFFER_Y,
        BUFFER_Z,
    );
    let mut slots = Part::empty("power_failure_holdover_thermal_buffer_coupon_slots");
    let mut coupons = Part::empty("power_failure_holdover_thermal_buffer_coupons");
    for coupon in 0..BUFFER_COUPON_COUNT {
        let x = centered_index(coupon, BUFFER_COUPON_COUNT, 45.0);
        slots = slots
            + centered_cube(
                format!("power_failure_holdover_buffer_coupon_slot_cut_{coupon}"),
                BUFFER_COUPON_X + 7.0,
                BUFFER_COUPON_Y + 7.0,
                13.0,
            )
            .translate(x, 0.0, BUFFER_Z / 2.0 - 6.5);
        coupons = coupons
            + centered_cube(
                format!("power_failure_holdover_passive_buffer_coupon_{coupon}"),
                BUFFER_COUPON_X,
                BUFFER_COUPON_Y,
                BUFFER_COUPON_Z,
            )
            .translate(x, 0.0, BUFFER_Z / 2.0 + BUFFER_COUPON_Z / 2.0)
            + centered_cube(
                format!("power_failure_holdover_buffer_coupon_handle_{coupon}"),
                BUFFER_COUPON_X - 12.0,
                5.0,
                9.0,
            )
            .translate(x, BUFFER_COUPON_Y / 2.0 + 6.0, BUFFER_Z / 2.0 + 4.5);
    }

    plate - slots + coupons + buffer_reference_rail()
}

fn buffer_reference_rail() -> Part {
    centered_cube(
        "power_failure_holdover_buffer_coupon_reference_rail",
        BUFFER_X - 42.0,
        9.0,
        16.0,
    )
    .translate(0.0, -BUFFER_Y / 2.0 + 18.0, BUFFER_Z / 2.0 + 8.0)
}

fn gas_decay_sample_ports() -> Part {
    let panel = centered_cube(
        "power_failure_holdover_gas_decay_sample_port_panel",
        SAMPLE_X,
        SAMPLE_Y,
        SAMPLE_Z,
    );
    let mut port_cuts = Part::empty("power_failure_holdover_gas_sample_port_cuts");
    let mut port_lands = Part::empty("power_failure_holdover_gas_sample_port_lands");

    for port in 0..GAS_SAMPLE_PORT_COUNT {
        let col = port % 3;
        let row = port / 3;
        let x = centered_index(col, 3, 82.0);
        let y = centered_index(row, 2, 58.0);
        port_cuts = port_cuts
            + centered_cylinder(
                format!("power_failure_holdover_gas_decay_sample_port_bore_{port}"),
                SAMPLE_PORT_D / 2.0,
                SAMPLE_Z + 4.0,
                32,
            )
            .translate(x, y, 0.0);
        port_lands = port_lands
            + centered_cylinder(
                format!("power_failure_holdover_gas_decay_vial_land_{port}"),
                SAMPLE_VIAL_LAND_D / 2.0,
                6.0,
                32,
            )
            .translate(x, y, SAMPLE_Z / 2.0 + 3.0)
            + centered_cube(
                format!("power_failure_holdover_gas_decay_port_cap_chain_land_{port}"),
                26.0,
                6.0,
                6.0,
            )
            .translate(x, y + 28.0, SAMPLE_Z / 2.0 + 3.0);
    }

    panel - port_cuts + port_lands + sample_port_manifold_witness_line()
}

fn sample_port_manifold_witness_line() -> Part {
    centered_cube(
        "power_failure_holdover_gas_decay_sample_port_manifold_witness_line",
        SAMPLE_X - 42.0,
        8.0,
        9.0,
    )
    .translate(0.0, -SAMPLE_Y / 2.0 + 16.0, SAMPLE_Z / 2.0 + 4.5)
}

fn alarm_ups_status_lands() -> Part {
    let plate = centered_cube(
        "power_failure_holdover_alarm_ups_status_land_plate",
        UPS_X,
        UPS_Y,
        UPS_Z,
    );
    let mut lands = Part::empty("power_failure_holdover_alarm_ups_indicator_lands");
    for i in 0..STATUS_LAND_COUNT {
        let x = centered_index(i % 4, 4, 48.0);
        let y = centered_index(i / 4, 2, 32.0);
        lands = lands
            + centered_cube(
                format!("power_failure_holdover_alarm_ups_status_land_{i}"),
                30.0,
                18.0,
                7.0,
            )
            .translate(x, y + 8.0, UPS_Z / 2.0 + 3.5);
    }
    let mut contacts = Part::empty("power_failure_holdover_dry_contact_lands");
    for i in 0..DRY_CONTACT_LAND_COUNT {
        contacts = contacts
            + centered_cube(
                format!("power_failure_holdover_dry_contact_terminal_land_{i}"),
                24.0,
                12.0,
                8.0,
            )
            .translate(
                centered_index(i, DRY_CONTACT_LAND_COUNT, 42.0),
                -UPS_Y / 2.0 + 18.0,
                UPS_Z / 2.0 + 4.0,
            );
    }
    let battery_shadow = centered_cube(
        "power_failure_holdover_ups_battery_shadow_land",
        UPS_X - 58.0,
        12.0,
        8.0,
    )
    .translate(0.0, UPS_Y / 2.0 - 16.0, UPS_Z / 2.0 + 4.0);

    plate + lands + contacts + battery_shadow
}

fn recovery_handoff_lanes() -> Part {
    let plate = centered_cube(
        "power_failure_holdover_recovery_handoff_lane_plate",
        HANDOFF_X,
        HANDOFF_Y,
        HANDOFF_Z,
    );
    let mut lane_cuts = Part::empty("power_failure_holdover_recovery_handoff_lane_cuts");
    let mut lane_lands = Part::empty("power_failure_holdover_recovery_handoff_lands");
    for lane in HandoffLane::all() {
        let x = centered_index(lane.index(), HANDOFF_LANE_COUNT, 152.0);
        lane_cuts = lane_cuts
            + centered_cube(
                format!("power_failure_holdover_{}_handoff_lane_recess", lane.name()),
                124.0,
                48.0,
                12.0,
            )
            .translate(x, 0.0, HANDOFF_Z / 2.0 - 6.0);
        for slot in 0..3 {
            lane_lands = lane_lands
                + centered_cube(
                    format!(
                        "power_failure_holdover_{}_handoff_evidence_slot_{slot}",
                        lane.name()
                    ),
                    32.0,
                    28.0,
                    6.0,
                )
                .translate(
                    x + centered_index(slot, 3, 38.0),
                    0.0,
                    HANDOFF_Z / 2.0 + 3.0,
                );
        }
    }

    plate - lane_cuts + lane_lands + handoff_direction_rail()
}

fn handoff_direction_rail() -> Part {
    centered_cube(
        "power_failure_holdover_recovery_handoff_direction_rail",
        HANDOFF_X - 54.0,
        8.0,
        12.0,
    )
    .translate(0.0, HANDOFF_Y / 2.0 - 14.0, HANDOFF_Z / 2.0 + 6.0)
}

fn barcode_custody_plate() -> Part {
    let plate = centered_cube(
        "power_failure_holdover_barcode_custody_plate",
        BARCODE_X,
        BARCODE_Y,
        BARCODE_Z,
    );
    let mut lands = Part::empty("power_failure_holdover_barcode_custody_lands");
    for i in 0..BARCODE_LAND_COUNT {
        lands = lands
            + centered_cube(
                format!("power_failure_holdover_barcode_land_{i}"),
                54.0,
                12.0,
                4.0,
            )
            .translate(
                centered_index(i % 2, 2, 74.0),
                centered_index(i / 2, 3, 22.0),
                BARCODE_Z / 2.0 + 2.0,
            );
    }
    for i in 0..CUSTODY_TOKEN_COUNT {
        lands = lands
            + centered_cylinder(
                format!("power_failure_holdover_custody_token_well_{i}"),
                11.0,
                5.0,
                28,
            )
            .translate(
                centered_index(i, CUSTODY_TOKEN_COUNT, 42.0),
                -BARCODE_Y / 2.0 + 12.0,
                BARCODE_Z / 2.0 + 2.5,
            );
    }
    plate + lands
}

fn condensate_capture_tray() -> Part {
    let tray = centered_cube(
        "power_failure_holdover_condensate_capture_tray",
        CONDENSATE_X,
        CONDENSATE_Y,
        CONDENSATE_Z,
    );
    let mut channels = Part::empty("power_failure_holdover_condensate_channel_cuts");
    let mut lands = Part::empty("power_failure_holdover_condensate_capture_lands");
    for channel in 0..CONDENSATE_CHANNEL_COUNT {
        channels = channels
            + centered_cube(
                format!("power_failure_holdover_condensate_witness_channel_{channel}"),
                CONDENSATE_X - 96.0,
                10.0,
                CONDENSATE_CHANNEL_DEPTH,
            )
            .translate(
                0.0,
                centered_index(channel, CONDENSATE_CHANNEL_COUNT, 16.0),
                CONDENSATE_Z / 2.0 - CONDENSATE_CHANNEL_DEPTH / 2.0,
            );
    }
    for cup in 0..CONDENSATE_CUP_COUNT {
        lands = lands
            + centered_cylinder(
                format!("power_failure_holdover_condensate_collection_cup_land_{cup}"),
                24.0,
                7.0,
                32,
            )
            .translate(
                centered_index(cup, CONDENSATE_CUP_COUNT, 130.0),
                -CONDENSATE_Y / 2.0 + 24.0,
                CONDENSATE_Z / 2.0 + 3.5,
            );
    }
    tray - channels + lands + condensate_splash_guard()
}

fn condensate_splash_guard() -> Part {
    let rear = centered_cube(
        "power_failure_holdover_condensate_rear_splash_guard",
        CONDENSATE_X - 44.0,
        8.0,
        28.0,
    )
    .translate(0.0, CONDENSATE_Y / 2.0 - 12.0, CONDENSATE_Z / 2.0 + 14.0);
    let left = centered_cube(
        "power_failure_holdover_condensate_left_splash_guard",
        8.0,
        CONDENSATE_Y - 32.0,
        22.0,
    )
    .translate(-CONDENSATE_X / 2.0 + 16.0, 0.0, CONDENSATE_Z / 2.0 + 11.0);
    let right = centered_cube(
        "power_failure_holdover_condensate_right_splash_guard",
        8.0,
        CONDENSATE_Y - 32.0,
        22.0,
    )
    .translate(CONDENSATE_X / 2.0 - 16.0, 0.0, CONDENSATE_Z / 2.0 + 11.0);
    rear + left + right
}

fn evidence_bridge() -> Part {
    let left_anchor = centered_cube(
        "power_failure_holdover_evidence_bridge_left_anchor",
        92.0,
        EVIDENCE_Y,
        EVIDENCE_ANCHOR_Z,
    )
    .translate(-EVIDENCE_X / 2.0 + 56.0, 0.0, 0.0);
    let right_anchor = centered_cube(
        "power_failure_holdover_evidence_bridge_right_anchor",
        92.0,
        EVIDENCE_Y,
        EVIDENCE_ANCHOR_Z,
    )
    .translate(EVIDENCE_X / 2.0 - 56.0, 0.0, 0.0);
    let left_post = centered_cube(
        "power_failure_holdover_evidence_bridge_left_post",
        32.0,
        24.0,
        EVIDENCE_POST_Z,
    )
    .translate(
        -EVIDENCE_X / 2.0 + 56.0,
        0.0,
        EVIDENCE_ANCHOR_Z / 2.0 + EVIDENCE_POST_Z / 2.0,
    );
    let right_post = centered_cube(
        "power_failure_holdover_evidence_bridge_right_post",
        32.0,
        24.0,
        EVIDENCE_POST_Z,
    )
    .translate(
        EVIDENCE_X / 2.0 - 56.0,
        0.0,
        EVIDENCE_ANCHOR_Z / 2.0 + EVIDENCE_POST_Z / 2.0,
    );
    let crossbar = centered_cube(
        "power_failure_holdover_evidence_bridge_camera_crossbar",
        EVIDENCE_X - 130.0,
        28.0,
        EVIDENCE_CROSSBAR_Z,
    )
    .translate(
        0.0,
        0.0,
        EVIDENCE_ANCHOR_Z / 2.0 + EVIDENCE_POST_Z + EVIDENCE_CROSSBAR_Z / 2.0,
    );

    left_anchor + right_anchor + left_post + right_post + crossbar + camera_lands() + light_bars()
}

fn camera_lands() -> Part {
    let mut lands = Part::empty("power_failure_holdover_evidence_camera_lands");
    for camera in 0..CAMERA_LAND_COUNT {
        lands = lands
            + centered_cube(
                format!("power_failure_holdover_evidence_camera_land_{camera}"),
                48.0,
                34.0,
                7.0,
            )
            .translate(
                centered_index(camera, CAMERA_LAND_COUNT, 210.0),
                -22.0,
                EVIDENCE_ANCHOR_Z / 2.0 + EVIDENCE_POST_Z + EVIDENCE_CROSSBAR_Z + 3.5,
            );
    }
    lands
}

fn light_bars() -> Part {
    let front = centered_cube(
        "power_failure_holdover_evidence_front_light_bar_land",
        EVIDENCE_X - 220.0,
        8.0,
        8.0,
    )
    .translate(
        0.0,
        -EVIDENCE_Y / 2.0 + 12.0,
        EVIDENCE_ANCHOR_Z / 2.0 + EVIDENCE_POST_Z + EVIDENCE_CROSSBAR_Z + 4.0,
    );
    let rear = centered_cube(
        "power_failure_holdover_evidence_rear_light_bar_land",
        EVIDENCE_X - 220.0,
        8.0,
        8.0,
    )
    .translate(
        0.0,
        EVIDENCE_Y / 2.0 - 12.0,
        EVIDENCE_ANCHOR_Z / 2.0 + EVIDENCE_POST_Z + EVIDENCE_CROSSBAR_Z + 4.0,
    );
    front + rear
}

fn robot_service_keepout_gauges() -> Part {
    let outline_front = centered_cube(
        "power_failure_holdover_robot_front_keepout_gauge",
        KEEP_OUT_X,
        8.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, -KEEP_OUT_Y / 2.0, 0.0);
    let outline_rear = centered_cube(
        "power_failure_holdover_service_rear_keepout_gauge",
        KEEP_OUT_X,
        8.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, KEEP_OUT_Y / 2.0, 0.0);
    let outline_left = centered_cube(
        "power_failure_holdover_left_side_service_keepout_gauge",
        8.0,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(-KEEP_OUT_X / 2.0, 0.0, 0.0);
    let outline_right = centered_cube(
        "power_failure_holdover_right_side_service_keepout_gauge",
        8.0,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(KEEP_OUT_X / 2.0, 0.0, 0.0);
    let robot_sweep = centered_cube(
        "power_failure_holdover_robot_gripper_sweep_keepout_gauge",
        430.0,
        8.0,
        36.0,
    )
    .translate(0.0, -KEEP_OUT_Y / 2.0 + 86.0, 18.0);
    let service_lift = centered_cube(
        "power_failure_holdover_probe_mast_service_lift_keepout_gauge",
        8.0,
        260.0,
        64.0,
    )
    .translate(KEEP_OUT_X / 2.0 - 92.0, 120.0, 32.0);

    outline_front + outline_rear + outline_left + outline_right + robot_sweep + service_lift
}

fn rectangular_frame(name: &str, outer_x: f64, outer_y: f64, rail_w: f64, z: f64) -> Part {
    let front = centered_cube(format!("{name}_front"), outer_x, rail_w, z).translate(
        0.0,
        -outer_y / 2.0 + rail_w / 2.0,
        0.0,
    );
    let rear = centered_cube(format!("{name}_rear"), outer_x, rail_w, z).translate(
        0.0,
        outer_y / 2.0 - rail_w / 2.0,
        0.0,
    );
    let left = centered_cube(format!("{name}_left"), rail_w, outer_y, z).translate(
        -outer_x / 2.0 + rail_w / 2.0,
        0.0,
        0.0,
    );
    let right = centered_cube(format!("{name}_right"), rail_w, outer_y, z).translate(
        outer_x / 2.0 - rail_w / 2.0,
        0.0,
        0.0,
    );
    front + rear + left + right
}

fn fiducial_disc(name: &str) -> Part {
    centered_cylinder(name, 12.0, 5.0, 36)
        - centered_cylinder(format!("{name}_center_bore"), 3.0, 6.0, 24)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn output_names_are_complete_and_unique() {
        assert_eq!(OUTPUTS.len(), 14);
        let unique: BTreeSet<_> = OUTPUTS.into_iter().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert!(OUTPUTS[0].ends_with("_base_containment_deck.stl"));
        assert!(OUTPUTS[13].ends_with("_assembly.stl"));
        assert!(OUTPUTS.iter().all(|path| path.starts_with("output/")));
        assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
    }

    #[test]
    fn modules_fit_inside_bounds_without_overlap() {
        assert_design_constraints();
        for item in socket_rects() {
            assert!(item.fits_inside_deck(), "{} outside deck", item.name);
        }
    }

    #[test]
    fn required_feature_coverage_matches_design_intent() {
        for feature in [
            "sealed_cassette_thermal_mass_nest",
            "independent_logger_pockets",
            "co2_o2_rh_probe_mast",
            "door_closed_lockout_token_lane",
            "passive_thermal_buffer_coupons",
            "gas_decay_sample_ports",
            "alarm_ups_status_lands",
            "recovery_handoff_lanes",
            "barcode_custody_plate",
            "condensate_capture_tray",
            "evidence_bridge",
            "robot_service_keepout_gauges",
            "named_stl_outputs",
        ] {
            assert!(
                REQUIRED_FEATURES.contains(&feature),
                "missing feature coverage for {feature}"
            );
        }
    }

    #[test]
    fn validation_counts_are_consistent() {
        assert_eq!(CASSETTE_COUNT, 6);
        assert_eq!(total_thermal_puck_count(), 24);
        assert_eq!(LOGGER_POCKET_COUNT, CASSETTE_COUNT);
        assert_eq!(GAS_SAMPLE_PORT_COUNT, 6);
        assert_eq!(HANDOFF_SLOT_COUNT, 9);
        assert!(thermal_buffer_total_mass_g() > 400.0);
        assert!(condensate_capture_capacity_ml() > expected_condensate_witness_ml());
    }
}
