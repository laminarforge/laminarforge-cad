use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed inline sensor calibration drift challenge station.
//
// Intent:
// - Package purchased inline pH, DO, O2, pressure, and flow sensor cartridges
//   for before/after no-cell endurance-run drift challenges.
// - Keep reference-standard pockets, wet/dry cartridge staging, drift challenge
//   loop routing, temperature equilibration, electrical continuity checks,
//   traceability, evidence imaging, release/hold/reject disposition, clean/used
//   segregation, flush/waste routing, and robot/service keepouts visible as
//   stable mechanical interfaces.
//
// This is validation packaging/interface CAD only. It does not define a
// calibration protocol, acceptance threshold, metrological traceability program,
// controller firmware, sterile barrier, biological endpoint, or cell-culture
// performance claim.

const OUTPUT_PREFIX: &str = "output/closed_inline_sensor_calibration_drift_challenge_station_";

const OUTPUTS: [&str; 14] = [
    "output/closed_inline_sensor_calibration_drift_challenge_station_base_leak_tray_deck.stl",
    "output/closed_inline_sensor_calibration_drift_challenge_station_sensor_cartridge_nest_array.stl",
    "output/closed_inline_sensor_calibration_drift_challenge_station_reference_standard_pocket_banks.stl",
    "output/closed_inline_sensor_calibration_drift_challenge_station_drift_challenge_loop_manifold.stl",
    "output/closed_inline_sensor_calibration_drift_challenge_station_temperature_equilibration_block.stl",
    "output/closed_inline_sensor_calibration_drift_challenge_station_wet_dry_cartridge_lanes.stl",
    "output/closed_inline_sensor_calibration_drift_challenge_station_electrical_pogo_check_fixture.stl",
    "output/closed_inline_sensor_calibration_drift_challenge_station_barcode_certificate_lands.stl",
    "output/closed_inline_sensor_calibration_drift_challenge_station_release_hold_reject_lanes.stl",
    "output/closed_inline_sensor_calibration_drift_challenge_station_clean_used_segregation.stl",
    "output/closed_inline_sensor_calibration_drift_challenge_station_evidence_camera_bridge.stl",
    "output/closed_inline_sensor_calibration_drift_challenge_station_waste_flush_routing.stl",
    "output/closed_inline_sensor_calibration_drift_challenge_station_robot_service_keepout_gauges.stl",
    "output/closed_inline_sensor_calibration_drift_challenge_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 13] = [
    "base_leak_tray_deck",
    "sensor_cartridge_nest_array",
    "reference_standard_pocket_banks",
    "drift_challenge_loop_manifold",
    "temperature_equilibration_block",
    "wet_dry_cartridge_lanes",
    "electrical_pogo_check_fixture",
    "barcode_certificate_lands",
    "release_hold_reject_lanes",
    "clean_used_segregation",
    "evidence_camera_bridge",
    "waste_flush_routing",
    "robot_service_keepout_gauges",
];

const LIMITATIONS: [&str; 6] = [
    "mechanical_validation_packaging_only",
    "no_calibration_protocol",
    "no_acceptance_limits",
    "no_biological_claim",
    "no_cell_endurance_run_context_only",
    "no_metrology_traceability_claim",
];

const STATION_X: f64 = 1320.0;
const STATION_Y: f64 = 820.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const SOCKET_DEPTH: f64 = 5.0;

const SENSOR_TYPE_COUNT: usize = 5;
const SENSOR_NAMES: [&str; SENSOR_TYPE_COUNT] = ["ph", "do", "o2", "pressure", "flow"];
const BEFORE_AFTER_LANES: usize = 2;
const CARTRIDGE_NEST_COUNT: usize = SENSOR_TYPE_COUNT * BEFORE_AFTER_LANES;
const SENSOR_NEST_X: f64 = 360.0;
const SENSOR_NEST_Y: f64 = 180.0;
const SENSOR_NEST_Z: f64 = 42.0;
const SENSOR_POS: (f64, f64) = (-430.0, 210.0);
const SENSOR_PITCH_X: f64 = 62.0;
const SENSOR_PITCH_Y: f64 = 64.0;
const SENSOR_POCKET_X: f64 = 48.0;
const SENSOR_POCKET_Y: f64 = 50.0;
const SENSOR_POCKET_DEPTH: f64 = 25.0;
const CARTRIDGE_TUBE_BORE_D: f64 = 6.4;

const STANDARD_BANK_X: f64 = 400.0;
const STANDARD_BANK_Y: f64 = 180.0;
const STANDARD_BANK_Z: f64 = 52.0;
const STANDARD_POS: (f64, f64) = (0.0, 210.0);
const REFERENCE_BANKS: usize = 2;
const STANDARD_WELLS_PER_BANK: usize = 5;
const REFERENCE_STANDARD_COUNT: usize = REFERENCE_BANKS * STANDARD_WELLS_PER_BANK;
const STANDARD_WELL_D: f64 = 34.0;
const STANDARD_WELL_DEPTH: f64 = 34.0;
const STANDARD_PITCH_X: f64 = 66.0;
const STANDARD_BANK_PITCH_Y: f64 = 66.0;
const STANDARD_RETENTION_LANDS: usize = REFERENCE_STANDARD_COUNT;

const TEMP_BLOCK_X: f64 = 320.0;
const TEMP_BLOCK_Y: f64 = 180.0;
const TEMP_BLOCK_Z: f64 = 58.0;
const TEMP_POS: (f64, f64) = (430.0, 210.0);
const TEMP_CARTRIDGE_SLOTS: usize = CARTRIDGE_NEST_COUNT;
const TEMP_SLOT_PITCH_X: f64 = 56.0;
const TEMP_SLOT_PITCH_Y: f64 = 58.0;
const TEMP_EQUILIBRATION_TARGET_C: f64 = 37.0;
const TEMP_SENSOR_WELLS: usize = 3;
const THERMAL_MASS_RIBS: usize = 6;

const MANIFOLD_X: f64 = 428.0;
const MANIFOLD_Y: f64 = 180.0;
const MANIFOLD_Z: f64 = 56.0;
const MANIFOLD_POS: (f64, f64) = (-420.0, 0.0);
const DRIFT_LOOP_COUNT: usize = SENSOR_TYPE_COUNT;
const LOOP_LANE_PITCH_Y: f64 = 30.0;
const LOOP_PORT_COUNT: usize = DRIFT_LOOP_COUNT * 2;
const SAMPLE_TUBE_OD: f64 = 4.8;
const SAMPLE_TUBE_CLEARANCE_D: f64 = SAMPLE_TUBE_OD + 1.0;
const LOOP_CHANNEL_D: f64 = 6.0;
const BYPASS_VALVE_PLACEHOLDERS: usize = SENSOR_TYPE_COUNT;

const WET_DRY_X: f64 = 420.0;
const WET_DRY_Y: f64 = 180.0;
const WET_DRY_Z: f64 = 38.0;
const WET_DRY_POS: (f64, f64) = (40.0, 0.0);
const WET_LANE_COUNT: usize = 2;
const DRY_LANE_COUNT: usize = 2;
const LANE_SLOT_COUNT: usize = SENSOR_TYPE_COUNT;
const WET_DRY_SLOT_X: f64 = 58.0;
const WET_DRY_SLOT_Y: f64 = 36.0;
const WET_DRY_MIN_DIVIDER_MM: f64 = 38.0;

const POGO_X: f64 = 300.0;
const POGO_Y: f64 = 180.0;
const POGO_Z: f64 = 48.0;
const POGO_POS: (f64, f64) = (430.0, 0.0);
const POGO_PIN_COLUMNS: usize = SENSOR_TYPE_COUNT;
const POGO_PINS_PER_COLUMN: usize = 6;
const POGO_PIN_D: f64 = 2.2;
const POGO_GUIDE_PIN_COUNT: usize = 4;
const CONNECTOR_SHELL_X: f64 = 210.0;
const CONNECTOR_SHELL_Y: f64 = 34.0;

const TRACE_X: f64 = 320.0;
const TRACE_Y: f64 = 150.0;
const TRACE_Z: f64 = 10.0;
const TRACE_POS: (f64, f64) = (-430.0, -230.0);
const BARCODE_LANDS: usize = 10;
const CERTIFICATE_LANDS: usize = 5;
const RUN_RECORD_LANDS: usize = 2;

const STATUS_X: f64 = 380.0;
const STATUS_Y: f64 = 150.0;
const STATUS_Z: f64 = 40.0;
const STATUS_POS: (f64, f64) = (0.0, -230.0);
const DISPOSITION_LANES: usize = 3;
const STATUS_SLOTS_PER_LANE: usize = SENSOR_TYPE_COUNT;
const STATUS_SLOT_X: f64 = 62.0;
const STATUS_SLOT_Y: f64 = 32.0;

const SEGREGATION_X: f64 = 320.0;
const SEGREGATION_Y: f64 = 150.0;
const SEGREGATION_Z: f64 = 52.0;
const SEGREGATION_POS: (f64, f64) = (400.0, -230.0);
const CLEAN_USED_DIVIDER_Z: f64 = 86.0;
const CLEAN_SIDE_WELLS: usize = SENSOR_TYPE_COUNT;
const USED_SIDE_WELLS: usize = SENSOR_TYPE_COUNT;
const SEGREGATION_GAP_MM: f64 = 48.0;

const CAMERA_BRIDGE_X: f64 = 900.0;
const CAMERA_BRIDGE_Y: f64 = 64.0;
const CAMERA_BRIDGE_Z: f64 = 210.0;
const CAMERA_POS: (f64, f64) = (0.0, 342.0);
const CAMERA_PORT_COUNT: usize = 3;
const LIGHT_RAIL_COUNT: usize = 2;
const CAMERA_TO_DECK_CLEARANCE_MM: f64 = 170.0;

const WASTE_ROUTE_X: f64 = 1080.0;
const WASTE_ROUTE_Y: f64 = 52.0;
const WASTE_ROUTE_Z: f64 = 44.0;
const WASTE_POS: (f64, f64) = (0.0, -352.0);
const FLUSH_PORT_COUNT: usize = 8;
const WASTE_CHANNEL_COUNT: usize = 8;
const WASTE_TRAP_CUPS: usize = 4;
const DRAIN_PORT_D: f64 = 13.0;

const KEEP_OUT_X: f64 = 1240.0;
const KEEP_OUT_Y: f64 = 760.0;
const KEEP_OUT_Z: f64 = 6.0;
const ROBOT_KEEP_OUT_ZONE_COUNT: usize = 5;
const FRONT_ROBOT_SWEEP_CLEARANCE: f64 = 420.0;
const REAR_SERVICE_CLEARANCE: f64 = 270.0;
const RIGHT_SERVICE_CLEARANCE: f64 = 190.0;
const CARTRIDGE_LIFT_CLEARANCE_Z: f64 = 150.0;

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

    let base = base_leak_tray_deck();
    export(OUTPUTS[0], &base);

    let sensor_nests = sensor_cartridge_nest_array();
    export(OUTPUTS[1], &sensor_nests);

    let standards = reference_standard_pocket_banks();
    export(OUTPUTS[2], &standards);

    let manifold = drift_challenge_loop_manifold();
    export(OUTPUTS[3], &manifold);

    let temperature = temperature_equilibration_block();
    export(OUTPUTS[4], &temperature);

    let wet_dry = wet_dry_cartridge_lanes();
    export(OUTPUTS[5], &wet_dry);

    let pogo = electrical_pogo_check_fixture();
    export(OUTPUTS[6], &pogo);

    let traceability = barcode_certificate_lands();
    export(OUTPUTS[7], &traceability);

    let disposition = release_hold_reject_lanes();
    export(OUTPUTS[8], &disposition);

    let segregation = clean_used_segregation();
    export(OUTPUTS[9], &segregation);

    let camera = evidence_camera_bridge();
    export(OUTPUTS[10], &camera);

    let waste = waste_flush_routing();
    export(OUTPUTS[11], &waste);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[12], &keepouts);

    let assembly = base
        + sensor_nests.translate(SENSOR_POS.0, SENSOR_POS.1, on_deck_z(SENSOR_NEST_Z))
        + standards.translate(STANDARD_POS.0, STANDARD_POS.1, on_deck_z(STANDARD_BANK_Z))
        + manifold.translate(MANIFOLD_POS.0, MANIFOLD_POS.1, on_deck_z(MANIFOLD_Z))
        + temperature.translate(TEMP_POS.0, TEMP_POS.1, on_deck_z(TEMP_BLOCK_Z))
        + wet_dry.translate(WET_DRY_POS.0, WET_DRY_POS.1, on_deck_z(WET_DRY_Z))
        + pogo.translate(POGO_POS.0, POGO_POS.1, on_deck_z(POGO_Z))
        + traceability.translate(TRACE_POS.0, TRACE_POS.1, on_deck_z(TRACE_Z))
        + disposition.translate(STATUS_POS.0, STATUS_POS.1, on_deck_z(STATUS_Z))
        + segregation.translate(
            SEGREGATION_POS.0,
            SEGREGATION_POS.1,
            on_deck_z(SEGREGATION_Z),
        )
        + camera.translate(CAMERA_POS.0, CAMERA_POS.1, on_deck_z(CAMERA_BRIDGE_Z))
        + waste.translate(WASTE_POS.0, WASTE_POS.1, on_deck_z(WASTE_ROUTE_Z))
        + keepouts.translate(0.0, 0.0, BASE_Z / 2.0 + KEEP_OUT_Z / 2.0);
    export(OUTPUTS[13], &assembly);

    println!();
    println!("Closed inline sensor calibration drift challenge station:");
    println!(
        "  Footprint:                 {STATION_X:.0}mm x {STATION_Y:.0}mm closed leak-tray deck"
    );
    println!(
        "  Sensor coverage:           {SENSOR_TYPE_COUNT} inline sensor families ({}) with before/after cartridge nests",
        SENSOR_NAMES.join(", ")
    );
    println!(
        "  Challenge packaging:       {REFERENCE_STANDARD_COUNT} reference-standard pockets, {DRIFT_LOOP_COUNT} drift loop lanes, {LOOP_PORT_COUNT} loop ports, and {BYPASS_VALVE_PLACEHOLDERS} bypass placeholders"
    );
    println!(
        "  Conditioning/checks:       {TEMP_CARTRIDGE_SLOTS} cartridge slots in {TEMP_EQUILIBRATION_TARGET_C:.0}C temperature block, {POGO_PIN_COLUMNS} pogo columns, {POGO_PINS_PER_COLUMN} pins each"
    );
    println!(
        "  Disposition/traceability:  {BARCODE_LANDS} barcode lands, {CERTIFICATE_LANDS} certificate lands, release/hold/reject lanes with {STATUS_SLOTS_PER_LANE} slots each, and clean/used segregation"
    );
    println!(
        "  Evidence/routing/access:   {CAMERA_PORT_COUNT} evidence camera ports, {FLUSH_PORT_COUNT} flush ports, {WASTE_CHANNEL_COUNT} waste channels, {ROBOT_KEEP_OUT_ZONE_COUNT} keepout zones"
    );
    println!(
        "  Limitations:               packaging only; no calibration protocol, biological claim, or release criteria"
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

fn module_footprints() -> [Footprint; 11] {
    [
        footprint(
            "sensor_cartridge_nest_array",
            SENSOR_POS,
            SENSOR_NEST_X,
            SENSOR_NEST_Y,
        ),
        footprint(
            "reference_standard_pocket_banks",
            STANDARD_POS,
            STANDARD_BANK_X,
            STANDARD_BANK_Y,
        ),
        footprint(
            "temperature_equilibration_block",
            TEMP_POS,
            TEMP_BLOCK_X,
            TEMP_BLOCK_Y,
        ),
        footprint(
            "drift_challenge_loop_manifold",
            MANIFOLD_POS,
            MANIFOLD_X,
            MANIFOLD_Y,
        ),
        footprint("wet_dry_cartridge_lanes", WET_DRY_POS, WET_DRY_X, WET_DRY_Y),
        footprint("electrical_pogo_check_fixture", POGO_POS, POGO_X, POGO_Y),
        footprint("barcode_certificate_lands", TRACE_POS, TRACE_X, TRACE_Y),
        footprint("release_hold_reject_lanes", STATUS_POS, STATUS_X, STATUS_Y),
        footprint(
            "clean_used_segregation",
            SEGREGATION_POS,
            SEGREGATION_X,
            SEGREGATION_Y,
        ),
        footprint(
            "evidence_camera_bridge",
            CAMERA_POS,
            CAMERA_BRIDGE_X,
            CAMERA_BRIDGE_Y,
        ),
        footprint(
            "waste_flush_routing",
            WASTE_POS,
            WASTE_ROUTE_X,
            WASTE_ROUTE_Y,
        ),
    ]
}

fn footprint(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Footprint {
    Footprint { name, center, x, y }
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 14);
    assert_eq!(REQUIRED_FEATURES.len(), 13);
    assert!(OUTPUTS
        .iter()
        .all(|path| path.starts_with(OUTPUT_PREFIX) && path.ends_with(".stl")));
    for limitation in [
        "mechanical_validation_packaging_only",
        "no_calibration_protocol",
        "no_acceptance_limits",
        "no_biological_claim",
        "no_cell_endurance_run_context_only",
        "no_metrology_traceability_claim",
    ] {
        assert!(LIMITATIONS.contains(&limitation));
    }
    assert_eq!(SENSOR_NAMES.len(), SENSOR_TYPE_COUNT);
    assert_eq!(CARTRIDGE_NEST_COUNT, SENSOR_TYPE_COUNT * BEFORE_AFTER_LANES);
    assert_eq!(
        REFERENCE_STANDARD_COUNT,
        SENSOR_TYPE_COUNT * REFERENCE_BANKS
    );
    assert_eq!(STANDARD_RETENTION_LANDS, REFERENCE_STANDARD_COUNT);
    assert_eq!(LOOP_PORT_COUNT, DRIFT_LOOP_COUNT * 2);
    assert_eq!(TEMP_CARTRIDGE_SLOTS, CARTRIDGE_NEST_COUNT);
    assert_eq!(POGO_GUIDE_PIN_COUNT, 4);
    assert_eq!(LIGHT_RAIL_COUNT, 2);
    assert_eq!(FLUSH_PORT_COUNT, WASTE_CHANNEL_COUNT);
    assert_eq!(DISPOSITION_LANES, 3);
    assert_eq!(CLEAN_SIDE_WELLS, SENSOR_TYPE_COUNT);
    assert_eq!(USED_SIDE_WELLS, SENSOR_TYPE_COUNT);
    assert!(SAMPLE_TUBE_CLEARANCE_D > SAMPLE_TUBE_OD);
    assert!(CAMERA_TO_DECK_CLEARANCE_MM > CARTRIDGE_LIFT_CLEARANCE_Z);
    assert!(CLEAN_USED_DIVIDER_Z > SEGREGATION_Z);
    assert!(SEGREGATION_GAP_MM >= WET_DRY_MIN_DIVIDER_MM);
    assert!(FRONT_ROBOT_SWEEP_CLEARANCE >= 400.0);
    assert!(REAR_SERVICE_CLEARANCE >= 260.0);
    assert!(RIGHT_SERVICE_CLEARANCE >= 180.0);

    let footprints = module_footprints();
    for module in footprints {
        assert!(
            module.fits_inside_deck(),
            "{} exceeds deck rim",
            module.name
        );
    }

    for (index, a) in footprints.iter().enumerate() {
        for b in footprints.iter().skip(index + 1) {
            assert!(!a.overlaps(*b), "{} overlaps {}", a.name, b.name);
        }
    }
}

fn base_leak_tray_deck() -> Part {
    let deck = centered_cube(
        "closed_inline_sensor_drift_station_base_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let recessed_pan = centered_cube(
        "closed_inline_sensor_drift_station_recessed_leak_pan",
        STATION_X - 2.0 * (RIM_W + 52.0),
        STATION_Y - 2.0 * (RIM_W + 50.0),
        8.0,
    )
    .translate(0.0, -10.0, BASE_Z - 4.0);
    let front_waste_gutter = centered_cube(
        "closed_inline_sensor_drift_station_front_waste_gutter",
        STATION_X - 190.0,
        22.0,
        9.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 62.0, BASE_Z - 4.5);
    let right_drain = centered_cylinder(
        "closed_inline_sensor_drift_station_deck_drain_port",
        DRAIN_PORT_D / 2.0,
        54.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 86.0,
        -STATION_Y / 2.0 + 38.0,
        BASE_Z - 9.0,
    );

    deck - recessed_pan - front_waste_gutter - right_drain - insert_sockets() - mounting_holes()
        + perimeter_rims()
        + zone_dividers()
        + datum_targets()
        + no_cell_endurance_label_land()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("closed_inline_sensor_drift_station_insert_sockets");
    for module in module_footprints().iter().take(10) {
        sockets = sockets
            + centered_cube(
                format!("closed_inline_sensor_drift_station_{}_socket", module.name),
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
    let mut holes = Part::empty("closed_inline_sensor_drift_station_mounting_holes");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 56.0, -STATION_Y / 2.0 + 56.0),
        (STATION_X / 2.0 - 56.0, -STATION_Y / 2.0 + 56.0),
        (-STATION_X / 2.0 + 56.0, STATION_Y / 2.0 - 56.0),
        (STATION_X / 2.0 - 56.0, STATION_Y / 2.0 - 56.0),
        (0.0, -STATION_Y / 2.0 + 56.0),
        (0.0, STATION_Y / 2.0 - 56.0),
        (-STATION_X / 2.0 + 56.0, 0.0),
        (STATION_X / 2.0 - 56.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("closed_inline_sensor_drift_station_m6_clearance_{i}"),
                3.4,
                BASE_Z + 4.0,
                28,
            )
            .translate(*x, *y, BASE_Z / 2.0);
    }
    holes
}

fn perimeter_rims() -> Part {
    let front = centered_cube(
        "closed_inline_sensor_drift_station_front_spill_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        "closed_inline_sensor_drift_station_rear_service_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "closed_inline_sensor_drift_station_left_spill_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "closed_inline_sensor_drift_station_right_service_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);

    front + rear + left + right
}

fn zone_dividers() -> Part {
    let top_to_loop = centered_cube(
        "closed_inline_sensor_drift_station_sensor_reference_to_loop_divider",
        STATION_X - 170.0,
        10.0,
        28.0,
    )
    .translate(0.0, 105.0, BASE_Z + 14.0);
    let loop_to_disposition = centered_cube(
        "closed_inline_sensor_drift_station_loop_to_disposition_divider",
        STATION_X - 180.0,
        10.0,
        28.0,
    )
    .translate(0.0, -128.0, BASE_Z + 14.0);
    let sensor_standard_divider = centered_cube(
        "closed_inline_sensor_drift_station_sensor_to_standard_divider",
        10.0,
        190.0,
        28.0,
    )
    .translate(-214.0, 210.0, BASE_Z + 14.0);
    let standard_temp_divider = centered_cube(
        "closed_inline_sensor_drift_station_standard_to_temperature_divider",
        10.0,
        190.0,
        28.0,
    )
    .translate(214.0, 210.0, BASE_Z + 14.0);
    let wet_dry_service_divider = centered_cube(
        "closed_inline_sensor_drift_station_loop_to_pogo_divider",
        10.0,
        190.0,
        28.0,
    )
    .translate(248.0, 0.0, BASE_Z + 14.0);

    top_to_loop
        + loop_to_disposition
        + sensor_standard_divider
        + standard_temp_divider
        + wet_dry_service_divider
}

fn datum_targets() -> Part {
    let mut targets = Part::empty("closed_inline_sensor_drift_station_datum_targets");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 88.0, -STATION_Y / 2.0 + 90.0),
        (STATION_X / 2.0 - 88.0, -STATION_Y / 2.0 + 90.0),
        (-STATION_X / 2.0 + 88.0, STATION_Y / 2.0 - 90.0),
        (STATION_X / 2.0 - 88.0, STATION_Y / 2.0 - 90.0),
    ]
    .iter()
    .enumerate()
    {
        targets =
            targets
                + fiducial_disc(&format!("closed_inline_sensor_drift_station_datum_{i}"))
                    .translate(*x, *y, BASE_Z + 2.5);
    }
    targets
}

fn no_cell_endurance_label_land() -> Part {
    centered_cube(
        "closed_inline_sensor_drift_station_no_cell_endurance_label_land",
        270.0,
        24.0,
        3.0,
    )
    .translate(0.0, STATION_Y / 2.0 - 78.0, BASE_Z + 1.5)
}

fn sensor_cartridge_nest_array() -> Part {
    let block = centered_cube(
        "closed_inline_sensor_drift_cartridge_nest_array_block",
        SENSOR_NEST_X,
        SENSOR_NEST_Y,
        SENSOR_NEST_Z,
    );
    let mut pockets = Part::empty("closed_inline_sensor_drift_cartridge_nest_array_pockets");
    let mut ribs = Part::empty("closed_inline_sensor_drift_cartridge_nest_array_ribs");
    let mut labels = Part::empty("closed_inline_sensor_drift_cartridge_nest_array_labels");
    let mut ports = Part::empty("closed_inline_sensor_drift_cartridge_nest_array_tube_ports");

    for row in 0..BEFORE_AFTER_LANES {
        let y = centered_index(row, BEFORE_AFTER_LANES, SENSOR_PITCH_Y);
        for (col, sensor) in SENSOR_NAMES.iter().enumerate() {
            let x = centered_index(col, SENSOR_TYPE_COUNT, SENSOR_PITCH_X);
            pockets = pockets
                + centered_cube(
                    format!(
                        "closed_inline_sensor_drift_{sensor}_{}_nest_pocket",
                        lane_name(row)
                    ),
                    SENSOR_POCKET_X,
                    SENSOR_POCKET_Y,
                    SENSOR_POCKET_DEPTH,
                )
                .translate(
                    x,
                    y,
                    SENSOR_NEST_Z / 2.0 - SENSOR_POCKET_DEPTH / 2.0 + 1.0,
                );
            ribs = ribs
                + centered_cube(
                    format!(
                        "closed_inline_sensor_drift_{sensor}_{}_nest_end_stop",
                        lane_name(row)
                    ),
                    SENSOR_POCKET_X + 8.0,
                    5.0,
                    18.0,
                )
                .translate(
                    x,
                    y + SENSOR_POCKET_Y / 2.0 + 6.0,
                    SENSOR_NEST_Z / 2.0 + 9.0,
                );
            labels = labels
                + centered_cube(
                    format!(
                        "closed_inline_sensor_drift_{sensor}_{}_label_land",
                        lane_name(row)
                    ),
                    48.0,
                    10.0,
                    3.0,
                )
                .translate(
                    x,
                    y - SENSOR_POCKET_Y / 2.0 - 12.0,
                    SENSOR_NEST_Z / 2.0 + 2.0,
                );
            ports = ports
                + centered_cylinder(
                    format!(
                        "closed_inline_sensor_drift_{sensor}_{}_inlet_tube_bore",
                        lane_name(row)
                    ),
                    CARTRIDGE_TUBE_BORE_D / 2.0,
                    SENSOR_NEST_X + 6.0,
                    24,
                )
                .rotate(0.0, 90.0, 0.0)
                .translate(0.0, y - 18.0, 0.0)
                + centered_cylinder(
                    format!(
                        "closed_inline_sensor_drift_{sensor}_{}_outlet_tube_bore",
                        lane_name(row)
                    ),
                    CARTRIDGE_TUBE_BORE_D / 2.0,
                    SENSOR_NEST_X + 6.0,
                    24,
                )
                .rotate(0.0, 90.0, 0.0)
                .translate(0.0, y + 18.0, 0.0);
        }
    }

    block - pockets - ports + ribs + labels + gripper_fiducials("cartridge_nest_array", 132.0)
}

fn lane_name(index: usize) -> &'static str {
    match index {
        0 => "before",
        _ => "after",
    }
}

fn reference_standard_pocket_banks() -> Part {
    let block = centered_cube(
        "closed_inline_sensor_drift_reference_standard_bank_block",
        STANDARD_BANK_X,
        STANDARD_BANK_Y,
        STANDARD_BANK_Z,
    );
    let mut wells = Part::empty("closed_inline_sensor_drift_reference_standard_wells");
    let mut retainers = Part::empty("closed_inline_sensor_drift_reference_standard_retainers");
    let mut cap_lands = Part::empty("closed_inline_sensor_drift_reference_standard_cap_lands");

    for bank in 0..REFERENCE_BANKS {
        let y = centered_index(bank, REFERENCE_BANKS, STANDARD_BANK_PITCH_Y);
        for (i, sensor) in SENSOR_NAMES.iter().enumerate() {
            let x = centered_index(i, STANDARD_WELLS_PER_BANK, STANDARD_PITCH_X);
            wells = wells
                + centered_cylinder(
                    format!(
                        "closed_inline_sensor_drift_{}_{}_reference_well",
                        sensor,
                        standard_bank_name(bank)
                    ),
                    STANDARD_WELL_D / 2.0,
                    STANDARD_WELL_DEPTH,
                    40,
                )
                .translate(
                    x,
                    y,
                    STANDARD_BANK_Z / 2.0 - STANDARD_WELL_DEPTH / 2.0 + 4.0,
                );
            retainers = retainers
                + centered_cube(
                    format!(
                        "closed_inline_sensor_drift_{}_{}_retention_clip",
                        sensor,
                        standard_bank_name(bank)
                    ),
                    38.0,
                    6.0,
                    16.0,
                )
                .translate(x, y + 23.0, STANDARD_BANK_Z / 2.0 + 8.0);
            cap_lands = cap_lands
                + centered_cube(
                    format!(
                        "closed_inline_sensor_drift_{}_{}_certificate_cap_land",
                        sensor,
                        standard_bank_name(bank)
                    ),
                    46.0,
                    12.0,
                    3.0,
                )
                .translate(x, y - 30.0, STANDARD_BANK_Z / 2.0 + 2.0);
        }
    }

    let bank_divider = centered_cube(
        "closed_inline_sensor_drift_reference_standard_bank_divider",
        STANDARD_BANK_X - 46.0,
        8.0,
        24.0,
    )
    .translate(0.0, 0.0, STANDARD_BANK_Z / 2.0 + 12.0);

    block - wells + retainers + cap_lands + bank_divider
}

fn standard_bank_name(index: usize) -> &'static str {
    match index {
        0 => "low",
        _ => "high",
    }
}

fn drift_challenge_loop_manifold() -> Part {
    let block = centered_cube(
        "closed_inline_sensor_drift_challenge_loop_manifold_block",
        MANIFOLD_X,
        MANIFOLD_Y,
        MANIFOLD_Z,
    );
    let mut channels = Part::empty("closed_inline_sensor_drift_challenge_loop_channels");
    let mut clips = Part::empty("closed_inline_sensor_drift_challenge_loop_clips");
    let mut bypasses = Part::empty("closed_inline_sensor_drift_bypass_valve_placeholders");

    for (i, sensor) in SENSOR_NAMES.iter().enumerate() {
        let y = centered_index(i, DRIFT_LOOP_COUNT, LOOP_LANE_PITCH_Y);
        channels = channels
            + centered_cylinder(
                format!("closed_inline_sensor_drift_{sensor}_loop_inlet_bore"),
                SAMPLE_TUBE_CLEARANCE_D / 2.0,
                MANIFOLD_X + 10.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, y - 8.0, 0.0)
            + centered_cylinder(
                format!("closed_inline_sensor_drift_{sensor}_loop_outlet_bore"),
                SAMPLE_TUBE_CLEARANCE_D / 2.0,
                MANIFOLD_X + 10.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, y + 8.0, 0.0)
            + centered_cube(
                format!("closed_inline_sensor_drift_{sensor}_serpentine_mid_channel"),
                MANIFOLD_X - 108.0,
                LOOP_CHANNEL_D,
                LOOP_CHANNEL_D,
            )
            .translate(0.0, y, MANIFOLD_Z / 2.0 - 10.0);
        clips = clips
            + centered_cube(
                format!("closed_inline_sensor_drift_{sensor}_loop_left_retainer"),
                18.0,
                24.0,
                18.0,
            )
            .translate(-MANIFOLD_X / 2.0 + 58.0, y, MANIFOLD_Z / 2.0 + 9.0)
            + centered_cube(
                format!("closed_inline_sensor_drift_{sensor}_loop_right_retainer"),
                18.0,
                24.0,
                18.0,
            )
            .translate(MANIFOLD_X / 2.0 - 58.0, y, MANIFOLD_Z / 2.0 + 9.0);
        bypasses = bypasses
            + centered_cube(
                format!("closed_inline_sensor_drift_{sensor}_bypass_valve_placeholder"),
                44.0,
                20.0,
                18.0,
            )
            .translate(MANIFOLD_X / 2.0 - 112.0, y, MANIFOLD_Z / 2.0 + 9.0);
    }

    let common_inlet_header = centered_cube(
        "closed_inline_sensor_drift_challenge_loop_common_inlet_header",
        18.0,
        MANIFOLD_Y - 42.0,
        22.0,
    )
    .translate(-MANIFOLD_X / 2.0 + 30.0, 0.0, MANIFOLD_Z / 2.0 + 11.0);
    let common_outlet_header = centered_cube(
        "closed_inline_sensor_drift_challenge_loop_common_outlet_header",
        18.0,
        MANIFOLD_Y - 42.0,
        22.0,
    )
    .translate(MANIFOLD_X / 2.0 - 30.0, 0.0, MANIFOLD_Z / 2.0 + 11.0);

    block - channels + clips + bypasses + common_inlet_header + common_outlet_header
}

fn temperature_equilibration_block() -> Part {
    let block = centered_cube(
        "closed_inline_sensor_drift_temperature_equilibration_block",
        TEMP_BLOCK_X,
        TEMP_BLOCK_Y,
        TEMP_BLOCK_Z,
    );
    let mut slots = Part::empty("closed_inline_sensor_drift_temperature_cartridge_slots");
    let mut ribs = Part::empty("closed_inline_sensor_drift_temperature_mass_ribs");

    for row in 0..BEFORE_AFTER_LANES {
        let y = centered_index(row, BEFORE_AFTER_LANES, TEMP_SLOT_PITCH_Y);
        for col in 0..SENSOR_TYPE_COUNT {
            let x = centered_index(col, SENSOR_TYPE_COUNT, TEMP_SLOT_PITCH_X);
            slots = slots
                + centered_cube(
                    format!("closed_inline_sensor_drift_temp_slot_{row}_{col}"),
                    42.0,
                    36.0,
                    32.0,
                )
                .translate(x, y, TEMP_BLOCK_Z / 2.0 - 16.0 + 3.0);
        }
    }

    for i in 0..THERMAL_MASS_RIBS {
        let x = centered_index(i, THERMAL_MASS_RIBS, 46.0);
        ribs = ribs
            + centered_cube(
                format!("closed_inline_sensor_drift_temperature_mass_rib_{i}"),
                8.0,
                TEMP_BLOCK_Y - 30.0,
                18.0,
            )
            .translate(x, 0.0, TEMP_BLOCK_Z / 2.0 + 9.0);
    }

    let mut temp_wells = Part::empty("closed_inline_sensor_drift_temperature_probe_wells");
    for i in 0..TEMP_SENSOR_WELLS {
        temp_wells = temp_wells
            + centered_cylinder(
                format!("closed_inline_sensor_drift_temperature_probe_well_{i}"),
                3.4,
                TEMP_BLOCK_X + 6.0,
                20,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(0.0, centered_index(i, TEMP_SENSOR_WELLS, 52.0), 0.0);
    }

    let target_label = centered_cube(
        "closed_inline_sensor_drift_temperature_37c_target_label_land",
        96.0,
        14.0,
        3.0,
    )
    .translate(0.0, -TEMP_BLOCK_Y / 2.0 + 18.0, TEMP_BLOCK_Z / 2.0 + 2.0);

    block - slots - temp_wells + ribs + target_label
}

fn wet_dry_cartridge_lanes() -> Part {
    let plate = centered_cube(
        "closed_inline_sensor_drift_wet_dry_cartridge_lane_plate",
        WET_DRY_X,
        WET_DRY_Y,
        WET_DRY_Z,
    );
    let mut pockets = Part::empty("closed_inline_sensor_drift_wet_dry_lane_pockets");
    let mut gutters = Part::empty("closed_inline_sensor_drift_wet_lane_gutters");
    let mut labels = Part::empty("closed_inline_sensor_drift_wet_dry_lane_labels");

    for lane in 0..(WET_LANE_COUNT + DRY_LANE_COUNT) {
        let y = centered_index(lane, WET_LANE_COUNT + DRY_LANE_COUNT, 36.0);
        let lane_is_wet = lane < WET_LANE_COUNT;
        for slot in 0..LANE_SLOT_COUNT {
            let x = centered_index(slot, LANE_SLOT_COUNT, 68.0);
            pockets = pockets
                + centered_cube(
                    format!(
                        "closed_inline_sensor_drift_{}_lane_{}_slot_{}",
                        if lane_is_wet { "wet" } else { "dry" },
                        lane,
                        slot
                    ),
                    WET_DRY_SLOT_X,
                    WET_DRY_SLOT_Y,
                    18.0,
                )
                .translate(x, y, WET_DRY_Z / 2.0 - 9.0 + 2.0);
        }
        labels = labels
            + centered_cube(
                format!(
                    "closed_inline_sensor_drift_{}_lane_{}_label_land",
                    if lane_is_wet { "wet" } else { "dry" },
                    lane
                ),
                42.0,
                12.0,
                3.0,
            )
            .translate(-WET_DRY_X / 2.0 + 34.0, y, WET_DRY_Z / 2.0 + 2.0);
        if lane_is_wet {
            gutters = gutters
                + centered_cube(
                    format!("closed_inline_sensor_drift_wet_lane_{lane}_drip_gutter"),
                    WET_DRY_X - 70.0,
                    8.0,
                    6.0,
                )
                .translate(16.0, y - 18.0, WET_DRY_Z / 2.0 - 3.0);
        }
    }

    let divider = centered_cube(
        "closed_inline_sensor_drift_wet_dry_physical_divider",
        WET_DRY_X - 42.0,
        10.0,
        34.0,
    )
    .translate(0.0, 0.0, WET_DRY_Z / 2.0 + 17.0);

    plate - pockets - gutters + labels + divider
}

fn electrical_pogo_check_fixture() -> Part {
    let base = centered_cube(
        "closed_inline_sensor_drift_electrical_pogo_check_fixture_base",
        POGO_X,
        POGO_Y,
        POGO_Z,
    );
    let connector_shell = centered_cube(
        "closed_inline_sensor_drift_electrical_connector_shell_placeholder",
        CONNECTOR_SHELL_X,
        CONNECTOR_SHELL_Y,
        36.0,
    )
    .translate(0.0, POGO_Y / 2.0 - 38.0, POGO_Z / 2.0 + 18.0);
    let cable_strain_relief = centered_cube(
        "closed_inline_sensor_drift_electrical_cable_strain_relief_channel",
        246.0,
        16.0,
        18.0,
    )
    .translate(0.0, -POGO_Y / 2.0 + 28.0, POGO_Z / 2.0 + 9.0);
    let mut pogo_pins = Part::empty("closed_inline_sensor_drift_pogo_pin_placeholders");
    for col in 0..POGO_PIN_COLUMNS {
        let x = centered_index(col, POGO_PIN_COLUMNS, 46.0);
        for pin in 0..POGO_PINS_PER_COLUMN {
            let y = centered_index(pin, POGO_PINS_PER_COLUMN, 18.0) - 8.0;
            pogo_pins = pogo_pins
                + centered_cylinder(
                    format!("closed_inline_sensor_drift_pogo_column_{col}_pin_{pin}"),
                    POGO_PIN_D / 2.0,
                    12.0,
                    16,
                )
                .translate(x, y, POGO_Z / 2.0 + 6.0);
        }
    }
    let mut guides = Part::empty("closed_inline_sensor_drift_pogo_alignment_guides");
    for (i, (x, y)) in [
        (-POGO_X / 2.0 + 34.0, -POGO_Y / 2.0 + 34.0),
        (POGO_X / 2.0 - 34.0, -POGO_Y / 2.0 + 34.0),
        (-POGO_X / 2.0 + 34.0, POGO_Y / 2.0 - 34.0),
        (POGO_X / 2.0 - 34.0, POGO_Y / 2.0 - 34.0),
    ]
    .iter()
    .enumerate()
    {
        guides = guides
            + centered_cylinder(
                format!("closed_inline_sensor_drift_pogo_guide_pin_{i}"),
                4.0,
                24.0,
                24,
            )
            .translate(*x, *y, POGO_Z / 2.0 + 12.0);
    }

    base + connector_shell + cable_strain_relief + pogo_pins + guides
}

fn barcode_certificate_lands() -> Part {
    let plate = centered_cube(
        "closed_inline_sensor_drift_barcode_certificate_land_plate",
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    );
    let mut lands = Part::empty("closed_inline_sensor_drift_barcode_certificate_lands");
    for i in 0..BARCODE_LANDS {
        let x = centered_index(i % 5, 5, 56.0);
        let y = 36.0 - (i / 5) as f64 * 32.0;
        lands = lands
            + centered_cube(
                format!("closed_inline_sensor_drift_barcode_land_{i}"),
                46.0,
                18.0,
                3.0,
            )
            .translate(x, y, TRACE_Z / 2.0 + 2.0);
    }
    for i in 0..CERTIFICATE_LANDS {
        let x = centered_index(i, CERTIFICATE_LANDS, 58.0);
        lands = lands
            + centered_cube(
                format!("closed_inline_sensor_drift_certificate_land_{i}"),
                48.0,
                20.0,
                3.0,
            )
            .translate(x, -44.0, TRACE_Z / 2.0 + 2.0);
    }
    for i in 0..RUN_RECORD_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_inline_sensor_drift_no_cell_run_record_land_{i}"),
                116.0,
                18.0,
                3.0,
            )
            .translate(
                centered_index(i, RUN_RECORD_LANDS, 132.0),
                -68.0,
                TRACE_Z / 2.0 + 2.0,
            );
    }

    plate + lands + gripper_fiducials("traceability_plate", 126.0)
}

fn release_hold_reject_lanes() -> Part {
    let plate = centered_cube(
        "closed_inline_sensor_drift_release_hold_reject_lane_plate",
        STATUS_X,
        STATUS_Y,
        STATUS_Z,
    );
    let mut slots = Part::empty("closed_inline_sensor_drift_disposition_lane_slots");
    let mut dividers = Part::empty("closed_inline_sensor_drift_disposition_lane_dividers");
    for lane in 0..DISPOSITION_LANES {
        let y = centered_index(lane, DISPOSITION_LANES, 42.0);
        for slot in 0..STATUS_SLOTS_PER_LANE {
            let x = centered_index(slot, STATUS_SLOTS_PER_LANE, 66.0);
            slots = slots
                + centered_cube(
                    format!(
                        "closed_inline_sensor_drift_{}_lane_slot_{}",
                        disposition_name(lane),
                        slot
                    ),
                    STATUS_SLOT_X,
                    STATUS_SLOT_Y,
                    22.0,
                )
                .translate(x, y, STATUS_Z / 2.0 - 11.0 + 2.0);
        }
        dividers = dividers
            + centered_cube(
                format!(
                    "closed_inline_sensor_drift_{}_lane_label_land",
                    disposition_name(lane)
                ),
                58.0,
                12.0,
                4.0,
            )
            .translate(-STATUS_X / 2.0 + 42.0, y, STATUS_Z / 2.0 + 3.0);
    }

    plate - slots + dividers
}

fn disposition_name(index: usize) -> &'static str {
    match index {
        0 => "release",
        1 => "hold",
        _ => "reject",
    }
}

fn clean_used_segregation() -> Part {
    let tray = centered_cube(
        "closed_inline_sensor_drift_clean_used_segregation_tray",
        SEGREGATION_X,
        SEGREGATION_Y,
        SEGREGATION_Z,
    );
    let mut pockets = Part::empty("closed_inline_sensor_drift_clean_used_pockets");
    for side in 0..2 {
        let center_x = centered_index(side, 2, SEGREGATION_GAP_MM + 106.0);
        let count = if side == 0 {
            CLEAN_SIDE_WELLS
        } else {
            USED_SIDE_WELLS
        };
        for i in 0..count {
            pockets = pockets
                + centered_cube(
                    format!(
                        "closed_inline_sensor_drift_{}_segregation_well_{}",
                        if side == 0 { "clean" } else { "used" },
                        i
                    ),
                    50.0,
                    26.0,
                    28.0,
                )
                .translate(
                    center_x,
                    centered_index(i, count, 24.0),
                    SEGREGATION_Z / 2.0 - 14.0 + 3.0,
                );
        }
    }
    let divider = centered_cube(
        "closed_inline_sensor_drift_clean_used_high_divider",
        12.0,
        SEGREGATION_Y - 24.0,
        CLEAN_USED_DIVIDER_Z,
    )
    .translate(0.0, 0.0, SEGREGATION_Z / 2.0 + CLEAN_USED_DIVIDER_Z / 2.0);
    let used_sump = centered_cube(
        "closed_inline_sensor_drift_used_side_liquid_sump",
        SEGREGATION_X / 2.0 - 42.0,
        SEGREGATION_Y - 34.0,
        7.0,
    )
    .translate(SEGREGATION_X / 4.0, 0.0, SEGREGATION_Z / 2.0 - 3.5);
    let clean_cover_lip = centered_cube(
        "closed_inline_sensor_drift_clean_side_cover_lip",
        SEGREGATION_X / 2.0 - 42.0,
        12.0,
        20.0,
    )
    .translate(
        -SEGREGATION_X / 4.0,
        SEGREGATION_Y / 2.0 - 20.0,
        SEGREGATION_Z / 2.0 + 10.0,
    );

    tray - pockets - used_sump + divider + clean_cover_lip
}

fn evidence_camera_bridge() -> Part {
    let left_upright = centered_cube(
        "closed_inline_sensor_drift_evidence_camera_bridge_left_upright",
        32.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(-CAMERA_BRIDGE_X / 2.0 + 34.0, 0.0, 0.0);
    let right_upright = centered_cube(
        "closed_inline_sensor_drift_evidence_camera_bridge_right_upright",
        32.0,
        CAMERA_BRIDGE_Y,
        CAMERA_BRIDGE_Z,
    )
    .translate(CAMERA_BRIDGE_X / 2.0 - 34.0, 0.0, 0.0);
    let crossbar = centered_cube(
        "closed_inline_sensor_drift_evidence_camera_bridge_crossbar",
        CAMERA_BRIDGE_X,
        CAMERA_BRIDGE_Y,
        30.0,
    )
    .translate(0.0, 0.0, CAMERA_BRIDGE_Z / 2.0 - 15.0);
    let light_front = centered_cube(
        "closed_inline_sensor_drift_evidence_camera_front_light_rail",
        CAMERA_BRIDGE_X - 150.0,
        8.0,
        16.0,
    )
    .translate(
        0.0,
        -CAMERA_BRIDGE_Y / 2.0 - 6.0,
        CAMERA_BRIDGE_Z / 2.0 - 44.0,
    );
    let light_rear = centered_cube(
        "closed_inline_sensor_drift_evidence_camera_rear_light_rail",
        CAMERA_BRIDGE_X - 150.0,
        8.0,
        16.0,
    )
    .translate(
        0.0,
        CAMERA_BRIDGE_Y / 2.0 + 6.0,
        CAMERA_BRIDGE_Z / 2.0 - 44.0,
    );
    let mut camera_ports = Part::empty("closed_inline_sensor_drift_evidence_camera_ports");
    for i in 0..CAMERA_PORT_COUNT {
        camera_ports = camera_ports
            + centered_cube(
                format!("closed_inline_sensor_drift_evidence_camera_port_{i}"),
                82.0,
                CAMERA_BRIDGE_Y + 4.0,
                18.0,
            )
            .translate(
                centered_index(i, CAMERA_PORT_COUNT, 180.0),
                0.0,
                CAMERA_BRIDGE_Z / 2.0 - 16.0,
            );
    }
    let fiducial_bar = centered_cube(
        "closed_inline_sensor_drift_evidence_camera_fiducial_bar",
        CAMERA_BRIDGE_X - 260.0,
        12.0,
        8.0,
    )
    .translate(0.0, 0.0, -CAMERA_BRIDGE_Z / 2.0 + 30.0);

    left_upright + right_upright + crossbar + light_front + light_rear - camera_ports + fiducial_bar
}

fn waste_flush_routing() -> Part {
    let body = centered_cube(
        "closed_inline_sensor_drift_waste_flush_routing_body",
        WASTE_ROUTE_X,
        WASTE_ROUTE_Y,
        WASTE_ROUTE_Z,
    );
    let mut channels = Part::empty("closed_inline_sensor_drift_waste_flush_channels");
    let mut port_labels = Part::empty("closed_inline_sensor_drift_waste_flush_port_labels");

    for i in 0..FLUSH_PORT_COUNT {
        let x = centered_index(i, FLUSH_PORT_COUNT, 118.0);
        channels = channels
            + centered_cylinder(
                format!("closed_inline_sensor_drift_flush_port_{i}"),
                SAMPLE_TUBE_CLEARANCE_D / 2.0,
                WASTE_ROUTE_Y + 10.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, 0.0)
            + centered_cube(
                format!("closed_inline_sensor_drift_waste_channel_{i}"),
                76.0,
                8.0,
                8.0,
            )
            .translate(x, -8.0, WASTE_ROUTE_Z / 2.0 - 10.0);
        port_labels = port_labels
            + centered_cube(
                format!("closed_inline_sensor_drift_flush_port_{i}_label_land"),
                44.0,
                12.0,
                3.0,
            )
            .translate(x, WASTE_ROUTE_Y / 2.0 - 10.0, WASTE_ROUTE_Z / 2.0 + 2.0);
    }

    let mut traps = Part::empty("closed_inline_sensor_drift_waste_trap_cups");
    for i in 0..WASTE_TRAP_CUPS {
        traps = traps
            + centered_cylinder(
                format!("closed_inline_sensor_drift_waste_trap_cup_{i}"),
                18.0,
                26.0,
                32,
            )
            .translate(
                centered_index(i, WASTE_TRAP_CUPS, 120.0),
                -6.0,
                WASTE_ROUTE_Z / 2.0 + 13.0,
            );
    }
    let drain = centered_cylinder(
        "closed_inline_sensor_drift_waste_flush_main_drain",
        DRAIN_PORT_D / 2.0,
        WASTE_ROUTE_Y + 10.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(WASTE_ROUTE_X / 2.0 - 54.0, 0.0, 0.0);

    body - channels - drain + traps + port_labels
}

fn robot_service_keepout_gauges() -> Part {
    let outer = centered_cube(
        "closed_inline_sensor_drift_robot_keepout_outer_gauge",
        KEEP_OUT_X,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    );
    let inner = centered_cube(
        "closed_inline_sensor_drift_robot_keepout_open_station_window",
        KEEP_OUT_X - 72.0,
        KEEP_OUT_Y - 72.0,
        KEEP_OUT_Z + 1.0,
    );
    let front_robot = centered_cube(
        "closed_inline_sensor_drift_front_robot_sweep_gauge",
        KEEP_OUT_X - 190.0,
        18.0,
        30.0,
    )
    .translate(0.0, -KEEP_OUT_Y / 2.0 + 44.0, KEEP_OUT_Z / 2.0 + 15.0);
    let rear_service = centered_cube(
        "closed_inline_sensor_drift_rear_service_access_gauge",
        KEEP_OUT_X - 210.0,
        18.0,
        32.0,
    )
    .translate(0.0, KEEP_OUT_Y / 2.0 - 44.0, KEEP_OUT_Z / 2.0 + 16.0);
    let right_service = centered_cube(
        "closed_inline_sensor_drift_right_service_access_gauge",
        18.0,
        KEEP_OUT_Y - 210.0,
        32.0,
    )
    .translate(KEEP_OUT_X / 2.0 - 44.0, 0.0, KEEP_OUT_Z / 2.0 + 16.0);
    let cartridge_lift = centered_cube(
        "closed_inline_sensor_drift_cartridge_vertical_lift_keepout",
        SENSOR_NEST_X + 84.0,
        SENSOR_NEST_Y + 56.0,
        18.0,
    )
    .translate(SENSOR_POS.0, SENSOR_POS.1, KEEP_OUT_Z / 2.0 + 44.0);
    let camera_service = centered_cube(
        "closed_inline_sensor_drift_evidence_camera_service_keepout",
        CAMERA_BRIDGE_X - 100.0,
        CAMERA_BRIDGE_Y + 64.0,
        18.0,
    )
    .translate(CAMERA_POS.0, CAMERA_POS.1, KEEP_OUT_Z / 2.0 + 46.0);

    (outer - inner) + front_robot + rear_service + right_service + cartridge_lift + camera_service
}

fn gripper_fiducials(name: &str, spacing: f64) -> Part {
    let left = fiducial_disc(&format!("closed_inline_sensor_drift_{name}_left_fiducial"))
        .translate(-spacing / 2.0, 0.0, 4.0);
    let right = fiducial_disc(&format!("closed_inline_sensor_drift_{name}_right_fiducial"))
        .translate(spacing / 2.0, 0.0, 4.0);
    left + right
}

fn fiducial_disc(name: &str) -> Part {
    centered_cylinder(format!("{name}_disc"), 7.0, 3.0, 32)
        - centered_cylinder(format!("{name}_center_dot"), 1.6, 4.0, 20)
        - centered_cube(format!("{name}_cross_x"), 12.0, 1.8, 4.0)
        - centered_cube(format!("{name}_cross_y"), 1.8, 12.0, 4.0)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_stable_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 14);
        assert_eq!(
            OUTPUTS[0],
            "output/closed_inline_sensor_calibration_drift_challenge_station_base_leak_tray_deck.stl"
        );
        assert_eq!(
            OUTPUTS[13],
            "output/closed_inline_sensor_calibration_drift_challenge_station_assembly.stl"
        );
        for path in OUTPUTS {
            assert!(path.starts_with(OUTPUT_PREFIX), "{path}");
            assert!(path.ends_with(".stl"), "{path}");
        }
    }

    #[test]
    fn requested_station_feature_groups_are_present() {
        for feature in [
            "base_leak_tray_deck",
            "sensor_cartridge_nest_array",
            "reference_standard_pocket_banks",
            "drift_challenge_loop_manifold",
            "temperature_equilibration_block",
            "wet_dry_cartridge_lanes",
            "electrical_pogo_check_fixture",
            "barcode_certificate_lands",
            "release_hold_reject_lanes",
            "clean_used_segregation",
            "evidence_camera_bridge",
            "waste_flush_routing",
            "robot_service_keepout_gauges",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn layout_modules_fit_without_footprint_overlap() {
        assert_design_constraints();
        let footprints = module_footprints();
        for module in footprints {
            assert!(module.fits_inside_deck(), "{} does not fit", module.name);
        }
    }

    #[test]
    fn inline_sensor_coverage_matches_drift_challenge_plan() {
        assert_eq!(SENSOR_NAMES, ["ph", "do", "o2", "pressure", "flow"]);
        assert_eq!(CARTRIDGE_NEST_COUNT, 10);
        assert_eq!(DRIFT_LOOP_COUNT, SENSOR_TYPE_COUNT);
        assert_eq!(LOOP_PORT_COUNT, SENSOR_TYPE_COUNT * 2);
        assert_eq!(BYPASS_VALVE_PLACEHOLDERS, SENSOR_TYPE_COUNT);
        assert!(SAMPLE_TUBE_CLEARANCE_D > SAMPLE_TUBE_OD);
    }

    #[test]
    fn reference_temperature_and_pogo_capacity_cover_before_after_runs() {
        assert_eq!(REFERENCE_STANDARD_COUNT, 10);
        assert_eq!(STANDARD_RETENTION_LANDS, REFERENCE_STANDARD_COUNT);
        assert_eq!(TEMP_CARTRIDGE_SLOTS, CARTRIDGE_NEST_COUNT);
        assert_eq!(TEMP_SENSOR_WELLS, 3);
        assert_eq!(POGO_PIN_COLUMNS, SENSOR_TYPE_COUNT);
        assert_eq!(POGO_PINS_PER_COLUMN, 6);
        assert_eq!(POGO_GUIDE_PIN_COUNT, 4);
        assert_eq!(TEMP_EQUILIBRATION_TARGET_C, 37.0);
    }

    #[test]
    fn wet_dry_disposition_and_segregation_are_explicit() {
        assert_eq!(WET_LANE_COUNT, 2);
        assert_eq!(DRY_LANE_COUNT, 2);
        assert_eq!(LANE_SLOT_COUNT, SENSOR_TYPE_COUNT);
        assert_eq!(DISPOSITION_LANES, 3);
        assert_eq!(STATUS_SLOTS_PER_LANE, SENSOR_TYPE_COUNT);
        assert_eq!(CLEAN_SIDE_WELLS, SENSOR_TYPE_COUNT);
        assert_eq!(USED_SIDE_WELLS, SENSOR_TYPE_COUNT);
        assert!(SEGREGATION_GAP_MM >= WET_DRY_MIN_DIVIDER_MM);
        assert!(CLEAN_USED_DIVIDER_Z > SEGREGATION_Z);
    }

    #[test]
    fn traceability_evidence_waste_and_keepouts_are_sized() {
        assert_eq!(BARCODE_LANDS, CARTRIDGE_NEST_COUNT);
        assert_eq!(CERTIFICATE_LANDS, SENSOR_TYPE_COUNT);
        assert_eq!(RUN_RECORD_LANDS, 2);
        assert_eq!(CAMERA_PORT_COUNT, 3);
        assert_eq!(LIGHT_RAIL_COUNT, 2);
        assert_eq!(FLUSH_PORT_COUNT, WASTE_CHANNEL_COUNT);
        assert_eq!(WASTE_TRAP_CUPS, 4);
        assert_eq!(ROBOT_KEEP_OUT_ZONE_COUNT, 5);
        assert!(FRONT_ROBOT_SWEEP_CLEARANCE >= 400.0);
        assert!(REAR_SERVICE_CLEARANCE >= 260.0);
        assert!(RIGHT_SERVICE_CLEARANCE >= 180.0);
    }

    #[test]
    fn limitations_remain_mechanical_not_protocol_or_biological_claims() {
        for limitation in [
            "mechanical_validation_packaging_only",
            "no_calibration_protocol",
            "no_acceptance_limits",
            "no_biological_claim",
            "no_cell_endurance_run_context_only",
            "no_metrology_traceability_claim",
        ] {
            assert!(LIMITATIONS.contains(&limitation));
        }
    }
}
