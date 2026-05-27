use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed media analyte sensor calibration/carryover validation station.
//
// Intent:
// - Package inline/offline media analyte sensor calibration and carryover
//   detection across repeated closed chip-media samples.
// - Make custody of standards, sample-loop selector lanes, rinse/blank
//   challenge routing, segregated waste, sensor-cartridge docks, bubble and
//   dead-volume witness geometry, traceability/status surfaces, keepouts, and
//   analyzer/perfusion-rack handoffs physically explicit.
// - Model validation-fixture architecture only. This CAD intentionally avoids
//   clinical acceptance thresholds, analyte limit values, or release criteria.

const OUTPUT_PREFIX: &str = "output/closed_media_analyte_sensor_calibration_carryover_station";
const OUTPUTS: [&str; 13] = [
    "output/closed_media_analyte_sensor_calibration_carryover_station_base_leak_tray_deck.stl",
    "output/closed_media_analyte_sensor_calibration_carryover_station_calibration_standard_custody_nests.stl",
    "output/closed_media_analyte_sensor_calibration_carryover_station_sample_loop_selector_lanes.stl",
    "output/closed_media_analyte_sensor_calibration_carryover_station_rinse_blank_challenge_path.stl",
    "output/closed_media_analyte_sensor_calibration_carryover_station_segregated_waste_caddy.stl",
    "output/closed_media_analyte_sensor_calibration_carryover_station_sensor_cartridge_dock.stl",
    "output/closed_media_analyte_sensor_calibration_carryover_station_bubble_dead_volume_witness_geometry.stl",
    "output/closed_media_analyte_sensor_calibration_carryover_station_barcode_status_surfaces.stl",
    "output/closed_media_analyte_sensor_calibration_carryover_station_closed_connector_handoff_bulkhead.stl",
    "output/closed_media_analyte_sensor_calibration_carryover_station_analyzer_perfusion_rack_interface_bridge.stl",
    "output/closed_media_analyte_sensor_calibration_carryover_station_evidence_bridge.stl",
    "output/closed_media_analyte_sensor_calibration_carryover_station_robot_service_keepout_gauges.stl",
    "output/closed_media_analyte_sensor_calibration_carryover_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 12] = [
    "calibration_standard_custody_nests",
    "sample_loop_selector_lanes",
    "rinse_blank_challenge_path",
    "segregated_waste_caddy",
    "sensor_cartridge_dock",
    "bubble_dead_volume_witness_geometry",
    "barcode_status_surfaces",
    "closed_connector_handoff_bulkhead",
    "analyzer_perfusion_rack_interface_bridge",
    "evidence_bridge",
    "robot_service_keepout_gauges",
    "closed_validation_fixture_only",
];

#[cfg(test)]
const LIMITATIONS: [&str; 5] = [
    "validation_fixture_intent_only",
    "no_clinical_acceptance_thresholds",
    "no_analyte_release_limits",
    "no_metrology_traceability_claim",
    "no_live_cell_acceptance_claim",
];

const PARAMETER_SET_REV: &str = "closed-media-analyte-carryover-station-parametric-rev-a";
const OUTPUT_MANIFEST_REV: &str = "stl-manifest-rev-a";
const USES_RANDOMNESS: bool = false;
const RANDOM_SEED: u64 = 0;
const CYLINDER_SEGMENTS: u32 = 32;
const FIDUCIAL_SEGMENTS: u32 = 36;
const FACET_TOLERANCE_MM: f64 = 0.25;

const STATION_X: f64 = 1320.0;
const STATION_Y: f64 = 850.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 44.0;
const SOCKET_DEPTH: f64 = 6.0;
const MOUNT_HOLE_D: f64 = 6.6;
const DRAIN_D: f64 = 14.0;
const MODULE_MARGIN_MM: f64 = 14.0;
const MAJOR_MODULE_GAP_MM: f64 = 10.0;

const ANALYTE_FAMILIES: usize = 6;
const CAL_LEVELS: usize = 3;
const CAL_STANDARD_NESTS: usize = ANALYTE_FAMILIES * CAL_LEVELS;
const CAL_BLOCK_X: f64 = 350.0;
const CAL_BLOCK_Y: f64 = 210.0;
const CAL_BLOCK_Z: f64 = 56.0;
const CAL_POS: (f64, f64) = (-440.0, 250.0);
const CAL_COLS: usize = ANALYTE_FAMILIES;
const CAL_PITCH_X: f64 = 50.0;
const CAL_PITCH_Y: f64 = 56.0;
const CAL_VIAL_D: f64 = 16.0;
const CAL_SEAL_RIM_D: f64 = 22.0;

const SAMPLE_LANES: usize = 8;
const SELECTOR_STATES: usize = 4;
const SAMPLE_LOOP_VOLUME_MARKERS: usize = SAMPLE_LANES * 2;
const SELECTOR_X: f64 = 430.0;
const SELECTOR_Y: f64 = 170.0;
const SELECTOR_Z: f64 = 54.0;
const SELECTOR_POS: (f64, f64) = (-20.0, 250.0);
const SELECTOR_PITCH_X: f64 = 48.0;
const SELECTOR_SLOT_X: f64 = 34.0;
const SELECTOR_SLOT_Y: f64 = 110.0;
const SELECTOR_ROTOR_D: f64 = 26.0;
const SAMPLE_LOOP_BORE_D: f64 = 5.8;

const RINSE_BRANCHES: usize = 4;
const BLANK_CHALLENGE_POSITIONS: usize = 4;
const RINSE_RESERVOIR_SOCKETS: usize = 4;
const RINSE_PATH_X: f64 = 300.0;
const RINSE_PATH_Y: f64 = 170.0;
const RINSE_PATH_Z: f64 = 52.0;
const RINSE_POS: (f64, f64) = (420.0, 250.0);
const RINSE_PITCH_X: f64 = 58.0;
const RINSE_SOCKET_D: f64 = 20.0;
const BLANK_SOCKET_D: f64 = 15.0;

const WASTE_STREAMS: usize = 4;
const WASTE_CUPS_PER_STREAM: usize = 2;
const WASTE_CUP_COUNT: usize = WASTE_STREAMS * WASTE_CUPS_PER_STREAM;
const WASTE_X: f64 = 240.0;
const WASTE_Y: f64 = 220.0;
const WASTE_Z: f64 = 58.0;
const WASTE_POS: (f64, f64) = (500.0, -20.0);
const WASTE_CUP_D: f64 = 28.0;
const WASTE_STREAM_GAP: f64 = 30.0;

const SENSOR_CARTRIDGE_DOCKS: usize = ANALYTE_FAMILIES;
const SENSOR_DOCK_X: f64 = 420.0;
const SENSOR_DOCK_Y: f64 = 210.0;
const SENSOR_DOCK_Z: f64 = 72.0;
const SENSOR_POS: (f64, f64) = (50.0, -20.0);
const SENSOR_DOCK_PITCH_X: f64 = 62.0;
const SENSOR_CARTRIDGE_X: f64 = 46.0;
const SENSOR_CARTRIDGE_Y: f64 = 118.0;
const POGO_PINS_PER_DOCK: usize = 6;

const WITNESS_X: f64 = 350.0;
const WITNESS_Y: f64 = 210.0;
const WITNESS_Z: f64 = 50.0;
const WITNESS_POS: (f64, f64) = (-420.0, -20.0);
const BUBBLE_WITNESS_CHAMBERS: usize = SAMPLE_LANES;
const DEAD_VOLUME_COUPONS: usize = SAMPLE_LANES;
const BUBBLE_REFERENCE_STEPS: usize = 5;
const WITNESS_PITCH_X: f64 = 40.0;
const WITNESS_CHAMBER_D: f64 = 18.0;

const TRACE_X: f64 = 350.0;
const TRACE_Y: f64 = 150.0;
const TRACE_Z: f64 = 12.0;
const TRACE_POS: (f64, f64) = (-420.0, -275.0);
const BARCODE_LANDS: usize = 16;
const STATUS_LANES: usize = 4;
const STATUS_TOKENS_PER_LANE: usize = 4;
const STATUS_TOKEN_SLOTS: usize = STATUS_LANES * STATUS_TOKENS_PER_LANE;
const RUN_RECORD_CARD_LANDS: usize = 4;

const HANDOFF_X: f64 = 430.0;
const HANDOFF_Y: f64 = 150.0;
const HANDOFF_Z: f64 = 96.0;
const HANDOFF_POS: (f64, f64) = (60.0, -275.0);
const CLOSED_CONNECTOR_PORTS: usize = SAMPLE_LANES + RINSE_BRANCHES;
const CONNECTOR_COLS: usize = 6;
const CONNECTOR_ROWS: usize = 2;
const CONNECTOR_PITCH_X: f64 = 58.0;
const CONNECTOR_PITCH_Z: f64 = 34.0;
const CONNECTOR_PORT_D: f64 = 9.0;

const INTERFACE_X: f64 = 260.0;
const INTERFACE_Y: f64 = 150.0;
const INTERFACE_Z: f64 = 74.0;
const INTERFACE_POS: (f64, f64) = (470.0, -275.0);
const ANALYZER_DOCK_WINDOWS: usize = 2;
const PERFUSION_RACK_LATCHES: usize = 4;
const STRAIN_RELIEF_CLAMPS: usize = 6;

const EVIDENCE_BRIDGE_X: f64 = 1180.0;
const EVIDENCE_BRIDGE_Y: f64 = 74.0;
const EVIDENCE_BRIDGE_Z: f64 = 218.0;
const EVIDENCE_POS: (f64, f64) = (0.0, 34.0);
const EVIDENCE_CAMERA_COUNT: usize = 3;
const EVIDENCE_LIGHT_SEGMENTS: usize = 6;
const CAMERA_CLEARANCE_Z: f64 = 174.0;

const ROBOT_SERVICE_KEEPOUT_ZONES: usize = 5;
const FRONT_ROBOT_CLEARANCE: f64 = 390.0;
const REAR_SERVICE_CLEARANCE: f64 = 260.0;
const LEFT_STANDARD_SERVICE_CLEARANCE: f64 = 210.0;
const RIGHT_WASTE_SERVICE_CLEARANCE: f64 = 220.0;
const TOP_SENSOR_LIFT_CLEARANCE: f64 = 260.0;
const KEEP_OUT_GAUGE_Z: f64 = 10.0;

#[derive(Clone, Copy)]
struct Footprint {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Footprint {
    fn fits_inside_station(self) -> bool {
        self.center.0.abs() + self.x / 2.0 <= STATION_X / 2.0 - RIM_W - MODULE_MARGIN_MM
            && self.center.1.abs() + self.y / 2.0 <= STATION_Y / 2.0 - RIM_W - MODULE_MARGIN_MM
    }

    fn overlaps(self, other: Footprint, margin: f64) -> bool {
        let a = rect(self.center, self.x, self.y);
        let b = rect(other.center, other.x, other.y);
        a.0 < b.1 + margin && a.1 + margin > b.0 && a.2 < b.3 + margin && a.3 + margin > b.2
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_leak_tray_deck();
    export(OUTPUTS[0], &base);

    let standards = calibration_standard_custody_nests();
    export(OUTPUTS[1], &standards);

    let selectors = sample_loop_selector_lanes();
    export(OUTPUTS[2], &selectors);

    let rinse_blank = rinse_blank_challenge_path();
    export(OUTPUTS[3], &rinse_blank);

    let waste = segregated_waste_caddy();
    export(OUTPUTS[4], &waste);

    let sensor = sensor_cartridge_dock();
    export(OUTPUTS[5], &sensor);

    let witness = bubble_dead_volume_witness_geometry();
    export(OUTPUTS[6], &witness);

    let traceability = barcode_status_surfaces();
    export(OUTPUTS[7], &traceability);

    let handoff = closed_connector_handoff_bulkhead();
    export(OUTPUTS[8], &handoff);

    let interface = analyzer_perfusion_rack_interface_bridge();
    export(OUTPUTS[9], &interface);

    let evidence = evidence_bridge();
    export(OUTPUTS[10], &evidence);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[11], &keepouts);

    let assembly = base
        + standards.translate(CAL_POS.0, CAL_POS.1, on_deck_z(CAL_BLOCK_Z))
        + selectors.translate(SELECTOR_POS.0, SELECTOR_POS.1, on_deck_z(SELECTOR_Z))
        + rinse_blank.translate(RINSE_POS.0, RINSE_POS.1, on_deck_z(RINSE_PATH_Z))
        + waste.translate(WASTE_POS.0, WASTE_POS.1, on_deck_z(WASTE_Z))
        + sensor.translate(SENSOR_POS.0, SENSOR_POS.1, on_deck_z(SENSOR_DOCK_Z))
        + witness.translate(WITNESS_POS.0, WITNESS_POS.1, on_deck_z(WITNESS_Z))
        + traceability.translate(TRACE_POS.0, TRACE_POS.1, on_deck_z(TRACE_Z))
        + handoff.translate(HANDOFF_POS.0, HANDOFF_POS.1, on_deck_z(HANDOFF_Z))
        + interface.translate(INTERFACE_POS.0, INTERFACE_POS.1, on_deck_z(INTERFACE_Z))
        + evidence.translate(EVIDENCE_POS.0, EVIDENCE_POS.1, on_deck_z(EVIDENCE_BRIDGE_Z))
        + closed_fluid_route_placeholders()
        + keepouts;
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed media analyte sensor calibration/carryover validation station:");
    println!(
        "  Footprint:                    {STATION_X:.0}mm x {STATION_Y:.0}mm contained leak tray"
    );
    println!(
        "  Calibration custody:          {CAL_STANDARD_NESTS} standard nests across {ANALYTE_FAMILIES} analyte families and {CAL_LEVELS} calibration levels"
    );
    println!(
        "  Sample-loop challenge:        {SAMPLE_LANES} selector lanes, {SELECTOR_STATES} selector states, {RINSE_BRANCHES} rinse branches, {BLANK_CHALLENGE_POSITIONS} blank challenge positions"
    );
    println!(
        "  Sensor/witness coverage:      {SENSOR_CARTRIDGE_DOCKS} cartridge docks, {BUBBLE_WITNESS_CHAMBERS} bubble witness chambers, {DEAD_VOLUME_COUPONS} dead-volume coupons, {SAMPLE_LOOP_VOLUME_MARKERS} loop markers"
    );
    println!(
        "  Closed handoff/routing:       {CLOSED_CONNECTOR_PORTS} closed connector ports to analyzer/perfusion rack, {WASTE_STREAMS} segregated waste streams, {STRAIN_RELIEF_CLAMPS} strain relief clamps"
    );
    println!(
        "  Traceability/evidence:        {BARCODE_LANDS} barcode lands, {STATUS_TOKEN_SLOTS} status token slots, {EVIDENCE_CAMERA_COUNT} cameras, {EVIDENCE_LIGHT_SEGMENTS} light segments"
    );
    println!(
        "  Keepouts:                     {ROBOT_SERVICE_KEEPOUT_ZONES} robot/service zones, camera clearance {CAMERA_CLEARANCE_Z:.0}mm"
    );
    println!(
        "  Reproducibility controls:     {PARAMETER_SET_REV}, {OUTPUT_MANIFEST_REV}, randomness={USES_RANDOMNESS}, seed={RANDOM_SEED}, cylinder_segments={CYLINDER_SEGMENTS}, facet_tolerance={FACET_TOLERANCE_MM:.2}mm"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn on_deck_z(part_z: f64) -> f64 {
    BASE_Z / 2.0 + part_z / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn module_footprints() -> [Footprint; 9] {
    [
        footprint(
            "calibration_standard_custody_nests",
            CAL_POS,
            CAL_BLOCK_X,
            CAL_BLOCK_Y,
        ),
        footprint(
            "sample_loop_selector_lanes",
            SELECTOR_POS,
            SELECTOR_X,
            SELECTOR_Y,
        ),
        footprint(
            "rinse_blank_challenge_path",
            RINSE_POS,
            RINSE_PATH_X,
            RINSE_PATH_Y,
        ),
        footprint("segregated_waste_caddy", WASTE_POS, WASTE_X, WASTE_Y),
        footprint(
            "sensor_cartridge_dock",
            SENSOR_POS,
            SENSOR_DOCK_X,
            SENSOR_DOCK_Y,
        ),
        footprint(
            "bubble_dead_volume_witness_geometry",
            WITNESS_POS,
            WITNESS_X,
            WITNESS_Y,
        ),
        footprint("barcode_status_surfaces", TRACE_POS, TRACE_X, TRACE_Y),
        footprint(
            "closed_connector_handoff_bulkhead",
            HANDOFF_POS,
            HANDOFF_X,
            HANDOFF_Y,
        ),
        footprint(
            "analyzer_perfusion_rack_interface_bridge",
            INTERFACE_POS,
            INTERFACE_X,
            INTERFACE_Y,
        ),
    ]
}

fn footprint(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Footprint {
    Footprint { name, center, x, y }
}

fn assert_layout() {
    assert!(OUTPUTS.iter().all(|path| path.starts_with(OUTPUT_PREFIX)));
    assert_eq!(CAL_STANDARD_NESTS, ANALYTE_FAMILIES * CAL_LEVELS);
    assert_eq!(STATUS_TOKEN_SLOTS, STATUS_LANES * STATUS_TOKENS_PER_LANE);
    assert_eq!(CLOSED_CONNECTOR_PORTS, SAMPLE_LANES + RINSE_BRANCHES);
    assert_eq!(CLOSED_CONNECTOR_PORTS, CONNECTOR_COLS * CONNECTOR_ROWS);
    assert_eq!(WASTE_CUP_COUNT, WASTE_STREAMS * WASTE_CUPS_PER_STREAM);
    assert!(CONNECTOR_PORT_D > SAMPLE_LOOP_BORE_D);
    assert!(WASTE_STREAM_GAP >= WASTE_CUP_D);
    assert!(TOP_SENSOR_LIFT_CLEARANCE >= CAMERA_CLEARANCE_Z);
    assert_eq!(ROBOT_SERVICE_KEEPOUT_ZONES, 5);
    assert!(
        !USES_RANDOMNESS,
        "fixture generator must remain deterministic"
    );

    let footprints = module_footprints();
    for module in footprints {
        assert!(
            module.fits_inside_station(),
            "{} exceeds station envelope",
            module.name
        );
    }

    for a in 0..footprints.len() {
        for b in (a + 1)..footprints.len() {
            assert!(
                !footprints[a].overlaps(footprints[b], MAJOR_MODULE_GAP_MM),
                "{} overlaps {}",
                footprints[a].name,
                footprints[b].name
            );
        }
    }

    assert!(
        SENSOR_POS.0 > WITNESS_POS.0 && WASTE_POS.0 > SENSOR_POS.0,
        "fluid path should progress witness -> sensor -> waste"
    );
    assert!(
        TRACE_POS.1 < SENSOR_POS.1 && HANDOFF_POS.1 < SENSOR_POS.1,
        "status and closed handoff surfaces must stay below wet sensor deck"
    );
    assert!(
        FRONT_ROBOT_CLEARANCE >= 380.0
            && REAR_SERVICE_CLEARANCE >= 250.0
            && TOP_SENSOR_LIFT_CLEARANCE >= 250.0,
        "robot/service keepout clearances are under-sized"
    );
}

fn rect(center: (f64, f64), x: f64, y: f64) -> (f64, f64, f64, f64) {
    (
        center.0 - x / 2.0,
        center.0 + x / 2.0,
        center.1 - y / 2.0,
        center.1 + y / 2.0,
    )
}

fn base_leak_tray_deck() -> Part {
    let deck = centered_cube(
        "closed_media_analyte_carryover_base_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let washdown_recess = centered_cube(
        "closed_media_analyte_carryover_washdown_recess",
        STATION_X - 136.0,
        STATION_Y - 122.0,
        8.0,
    )
    .translate(0.0, -8.0, BASE_Z / 2.0 - 3.8);
    let wet_zone_sump = centered_cube(
        "closed_media_analyte_carryover_wet_zone_sump",
        STATION_X - 190.0,
        210.0,
        9.0,
    )
    .translate(40.0, 92.0, BASE_Z / 2.0 - 4.2);
    let front_drain = centered_cylinder(
        "closed_media_analyte_carryover_front_closed_waste_drain",
        DRAIN_D / 2.0,
        58.0,
        CYLINDER_SEGMENTS,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 86.0, -STATION_Y / 2.0 - 2.0, 0.0);

    deck - washdown_recess - wet_zone_sump - front_drain - insert_sockets() - mounting_slots()
        + perimeter_rims()
        + zone_dividers()
        + leak_witness_ribs()
        + robot_datum_targets()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("closed_media_analyte_carryover_insert_sockets");
    for module in module_footprints() {
        sockets = sockets
            + centered_cube(
                format!("closed_media_analyte_carryover_{}_socket", module.name),
                module.x + 8.0,
                module.y + 8.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(
                module.center.0,
                module.center.1,
                BASE_Z / 2.0 - SOCKET_DEPTH / 2.0,
            );
    }
    sockets
}

fn mounting_slots() -> Part {
    let mut slots = Part::empty("closed_media_analyte_carryover_mounting_slots");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 54.0), -(STATION_Y / 2.0 - 52.0)),
        (STATION_X / 2.0 - 54.0, -(STATION_Y / 2.0 - 52.0)),
        (-(STATION_X / 2.0 - 54.0), STATION_Y / 2.0 - 52.0),
        (STATION_X / 2.0 - 54.0, STATION_Y / 2.0 - 52.0),
        (0.0, STATION_Y / 2.0 - 52.0),
        (0.0, -(STATION_Y / 2.0 - 52.0)),
    ]
    .iter()
    .enumerate()
    {
        let hole = centered_cylinder(
            format!("closed_media_analyte_carryover_m6_clearance_{i}"),
            MOUNT_HOLE_D / 2.0,
            BASE_Z + 4.0,
            CYLINDER_SEGMENTS,
        )
        .translate(*x, *y, 0.0);
        let relief = centered_cube(
            format!("closed_media_analyte_carryover_m6_slot_relief_{i}"),
            27.0,
            7.2,
            BASE_Z + 4.0,
        )
        .translate(*x, *y, 0.0);
        slots = slots + hole + relief;
    }
    slots
}

fn perimeter_rims() -> Part {
    let left = centered_cube(
        "closed_media_analyte_carryover_left_leak_tray_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(
        -(STATION_X / 2.0 - RIM_W / 2.0),
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(
        "closed_media_analyte_carryover_right_leak_tray_rim",
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
        "closed_media_analyte_carryover_rear_leak_tray_rim",
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
        "closed_media_analyte_carryover_front_low_robot_lip",
        STATION_X - 180.0,
        14.0,
        24.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 20.0, BASE_Z / 2.0 + 12.0);
    left + right + rear + front_low_lip
}

fn zone_dividers() -> Part {
    let wet_to_status = centered_cube(
        "closed_media_analyte_carryover_wet_to_status_divider",
        STATION_X - 170.0,
        12.0,
        30.0,
    )
    .translate(0.0, -158.0, BASE_Z / 2.0 + 15.0);
    let standard_to_selector = centered_cube(
        "closed_media_analyte_carryover_standard_to_selector_custody_divider",
        10.0,
        206.0,
        34.0,
    )
    .translate(-250.0, 250.0, BASE_Z / 2.0 + 17.0);
    let selector_to_rinse = centered_cube(
        "closed_media_analyte_carryover_selector_to_rinse_blank_divider",
        10.0,
        184.0,
        30.0,
    )
    .translate(235.0, 250.0, BASE_Z / 2.0 + 15.0);
    let sensor_to_waste = centered_cube(
        "closed_media_analyte_carryover_sensor_to_waste_hard_barrier",
        12.0,
        250.0,
        34.0,
    )
    .translate(320.0, -24.0, BASE_Z / 2.0 + 17.0);
    wet_to_status + standard_to_selector + selector_to_rinse + sensor_to_waste
}

fn leak_witness_ribs() -> Part {
    let mut ribs = Part::empty("closed_media_analyte_carryover_leak_witness_ribs");
    for (i, x) in [-540.0, -360.0, -180.0, 0.0, 180.0, 360.0, 540.0]
        .into_iter()
        .enumerate()
    {
        ribs = ribs
            + centered_cube(
                format!("closed_media_analyte_carryover_leak_witness_rib_{i}"),
                8.0,
                STATION_Y - 166.0,
                6.0,
            )
            .translate(x, -10.0, BASE_Z / 2.0 + 3.0);
    }
    ribs + centered_cube(
        "closed_media_analyte_carryover_drain_lead_in_witness_gutter",
        190.0,
        8.0,
        7.0,
    )
    .translate(
        STATION_X / 2.0 - 154.0,
        -STATION_Y / 2.0 + 58.0,
        BASE_Z / 2.0 + 3.5,
    )
}

fn robot_datum_targets() -> Part {
    let mut targets = Part::empty("closed_media_analyte_carryover_robot_datum_targets");
    for (i, (x, y)) in [
        (-590.0, 360.0),
        (-220.0, 360.0),
        (220.0, 360.0),
        (590.0, 360.0),
        (-590.0, -360.0),
        (590.0, -360.0),
    ]
    .iter()
    .enumerate()
    {
        targets =
            targets
                + fiducial_disc(&format!("closed_media_analyte_carryover_robot_datum_{i}"))
                    .translate(*x, *y, BASE_Z / 2.0 + 3.0);
    }
    targets
}

fn calibration_standard_custody_nests() -> Part {
    let body = centered_cube(
        "closed_media_analyte_carryover_calibration_custody_body",
        CAL_BLOCK_X,
        CAL_BLOCK_Y,
        CAL_BLOCK_Z,
    );
    let gasket_groove = centered_cube(
        "closed_media_analyte_carryover_calibration_lid_gasket_groove",
        CAL_BLOCK_X - 34.0,
        CAL_BLOCK_Y - 34.0,
        7.0,
    )
    .translate(0.0, 0.0, CAL_BLOCK_Z / 2.0 - 3.0);

    body - gasket_groove - calibration_vial_well_cuts()
        + calibration_vial_seal_rims()
        + calibration_level_separators()
        + calibration_analyte_key_tabs()
        + custody_gate_bars("calibration")
        + chain_of_custody_token_lands()
}

fn calibration_vial_well_cuts() -> Part {
    let mut wells = Part::empty("closed_media_analyte_carryover_calibration_vial_well_cuts");
    for i in 0..CAL_STANDARD_NESTS {
        let x = centered_index(i % CAL_COLS, CAL_COLS, CAL_PITCH_X);
        let y = centered_index(i / CAL_COLS, CAL_LEVELS, CAL_PITCH_Y);
        wells = wells
            + centered_cylinder(
                format!("closed_media_analyte_carryover_calibration_vial_well_{i}"),
                CAL_VIAL_D / 2.0,
                CAL_BLOCK_Z + 4.0,
                CYLINDER_SEGMENTS,
            )
            .translate(x, y, 0.0);
    }
    wells
}

fn calibration_vial_seal_rims() -> Part {
    let mut rims = Part::empty("closed_media_analyte_carryover_calibration_vial_seal_rims");
    for i in 0..CAL_STANDARD_NESTS {
        let x = centered_index(i % CAL_COLS, CAL_COLS, CAL_PITCH_X);
        let y = centered_index(i / CAL_COLS, CAL_LEVELS, CAL_PITCH_Y);
        let rim = centered_cylinder(
            format!("closed_media_analyte_carryover_calibration_vial_rim_{i}"),
            CAL_SEAL_RIM_D / 2.0,
            5.0,
            CYLINDER_SEGMENTS,
        )
        .translate(x, y, CAL_BLOCK_Z / 2.0 + 2.5);
        let opening = centered_cylinder(
            format!("closed_media_analyte_carryover_calibration_vial_rim_opening_{i}"),
            CAL_VIAL_D / 2.0 + 1.0,
            6.0,
            CYLINDER_SEGMENTS,
        )
        .translate(x, y, CAL_BLOCK_Z / 2.0 + 2.5);
        rims = rims + (rim - opening);
    }
    rims
}

fn calibration_level_separators() -> Part {
    let mut separators = Part::empty("closed_media_analyte_carryover_calibration_level_separators");
    for i in 0..(CAL_LEVELS - 1) {
        let y = centered_index(i, CAL_LEVELS - 1, CAL_PITCH_Y);
        separators = separators
            + centered_cube(
                format!("closed_media_analyte_carryover_calibration_level_separator_{i}"),
                CAL_BLOCK_X - 38.0,
                5.0,
                14.0,
            )
            .translate(0.0, y, CAL_BLOCK_Z / 2.0 + 7.0);
    }
    separators
}

fn calibration_analyte_key_tabs() -> Part {
    let mut tabs = Part::empty("closed_media_analyte_carryover_calibration_analyte_key_tabs");
    for family in 0..ANALYTE_FAMILIES {
        tabs = tabs
            + centered_cube(
                format!("closed_media_analyte_carryover_analyte_family_key_tab_{family}"),
                36.0,
                12.0,
                7.0,
            )
            .translate(
                centered_index(family, ANALYTE_FAMILIES, CAL_PITCH_X),
                CAL_BLOCK_Y / 2.0 - 20.0,
                CAL_BLOCK_Z / 2.0 + 3.5,
            );
    }
    tabs
}

fn custody_gate_bars(prefix: &str) -> Part {
    let rear_hinge = centered_cube(
        format!("closed_media_analyte_carryover_{prefix}_rear_hinged_custody_bar"),
        CAL_BLOCK_X,
        14.0,
        28.0,
    )
    .translate(0.0, CAL_BLOCK_Y / 2.0 - 7.0, CAL_BLOCK_Z / 2.0 + 14.0);
    let front_latch = centered_cube(
        format!("closed_media_analyte_carryover_{prefix}_front_latched_custody_bar"),
        CAL_BLOCK_X,
        14.0,
        24.0,
    )
    .translate(0.0, -(CAL_BLOCK_Y / 2.0 - 7.0), CAL_BLOCK_Z / 2.0 + 12.0);
    rear_hinge + front_latch
}

fn chain_of_custody_token_lands() -> Part {
    let mut lands = Part::empty("closed_media_analyte_carryover_calibration_custody_token_lands");
    for i in 0..CAL_LEVELS {
        lands = lands
            + centered_cube(
                format!("closed_media_analyte_carryover_calibration_custody_token_land_{i}"),
                52.0,
                16.0,
                5.0,
            )
            .translate(
                CAL_BLOCK_X / 2.0 - 42.0,
                centered_index(i, CAL_LEVELS, CAL_PITCH_Y),
                CAL_BLOCK_Z / 2.0 + 2.5,
            );
    }
    lands
}

fn sample_loop_selector_lanes() -> Part {
    let body = centered_cube(
        "closed_media_analyte_carryover_sample_loop_selector_body",
        SELECTOR_X,
        SELECTOR_Y,
        SELECTOR_Z,
    );
    body - selector_lane_socket_cuts() - selector_tube_bores()
        + selector_lane_rails()
        + selector_rotor_state_discs()
        + sample_loop_saddle_markers()
        + selector_closed_lid_latches()
}

fn selector_lane_socket_cuts() -> Part {
    let mut cuts = Part::empty("closed_media_analyte_carryover_selector_lane_socket_cuts");
    for lane in 0..SAMPLE_LANES {
        cuts = cuts
            + centered_cube(
                format!("closed_media_analyte_carryover_selector_lane_socket_{lane}"),
                SELECTOR_SLOT_X,
                SELECTOR_SLOT_Y,
                SELECTOR_Z + 4.0,
            )
            .translate(
                centered_index(lane, SAMPLE_LANES, SELECTOR_PITCH_X),
                -10.0,
                -2.0,
            );
    }
    cuts
}

fn selector_tube_bores() -> Part {
    let mut bores = Part::empty("closed_media_analyte_carryover_selector_tube_bores");
    for lane in 0..SAMPLE_LANES {
        let x = centered_index(lane, SAMPLE_LANES, SELECTOR_PITCH_X);
        for offset in [-9.0, 9.0] {
            bores = bores
                + centered_cylinder(
                    format!(
                        "closed_media_analyte_carryover_selector_lane_{lane}_tube_bore_{offset}"
                    ),
                    SAMPLE_LOOP_BORE_D / 2.0,
                    SELECTOR_Y + 20.0,
                    CYLINDER_SEGMENTS,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(x + offset, 0.0, 0.0);
        }
    }
    bores
}

fn selector_lane_rails() -> Part {
    let mut rails = Part::empty("closed_media_analyte_carryover_selector_lane_rails");
    for lane in 0..SAMPLE_LANES {
        let x = centered_index(lane, SAMPLE_LANES, SELECTOR_PITCH_X);
        rails = rails
            + centered_cube(
                format!("closed_media_analyte_carryover_selector_lane_{lane}_left_rail"),
                5.0,
                SELECTOR_Y - 26.0,
                18.0,
            )
            .translate(x - SELECTOR_SLOT_X / 2.0 - 5.0, 0.0, SELECTOR_Z / 2.0 + 9.0)
            + centered_cube(
                format!("closed_media_analyte_carryover_selector_lane_{lane}_right_rail"),
                5.0,
                SELECTOR_Y - 26.0,
                18.0,
            )
            .translate(x + SELECTOR_SLOT_X / 2.0 + 5.0, 0.0, SELECTOR_Z / 2.0 + 9.0);
    }
    rails
}

fn selector_rotor_state_discs() -> Part {
    let mut rotors = Part::empty("closed_media_analyte_carryover_selector_rotor_state_discs");
    for lane in 0..SAMPLE_LANES {
        let x = centered_index(lane, SAMPLE_LANES, SELECTOR_PITCH_X);
        let rotor = centered_cylinder(
            format!("closed_media_analyte_carryover_selector_rotor_state_disc_{lane}"),
            SELECTOR_ROTOR_D / 2.0,
            6.0,
            CYLINDER_SEGMENTS,
        )
        .translate(x, SELECTOR_Y / 2.0 - 32.0, SELECTOR_Z / 2.0 + 3.0);
        let bore = centered_cylinder(
            format!("closed_media_analyte_carryover_selector_rotor_center_bore_{lane}"),
            5.0,
            7.0,
            CYLINDER_SEGMENTS,
        )
        .translate(x, SELECTOR_Y / 2.0 - 32.0, SELECTOR_Z / 2.0 + 3.0);
        rotors = rotors + (rotor - bore) + selector_state_ticks(lane, x);
    }
    rotors
}

fn selector_state_ticks(lane: usize, x: f64) -> Part {
    let mut ticks = Part::empty(format!(
        "closed_media_analyte_carryover_selector_lane_{lane}_state_ticks"
    ));
    for state in 0..SELECTOR_STATES {
        let dx = centered_index(state, SELECTOR_STATES, 8.0);
        ticks = ticks
            + centered_cube(
                format!("closed_media_analyte_carryover_selector_lane_{lane}_state_tick_{state}"),
                3.0,
                10.0,
                4.0,
            )
            .translate(x + dx, SELECTOR_Y / 2.0 - 10.0, SELECTOR_Z / 2.0 + 2.0);
    }
    ticks
}

fn sample_loop_saddle_markers() -> Part {
    let mut saddles = Part::empty("closed_media_analyte_carryover_sample_loop_saddle_markers");
    for lane in 0..SAMPLE_LANES {
        let x = centered_index(lane, SAMPLE_LANES, SELECTOR_PITCH_X);
        for marker in 0..2 {
            saddles = saddles
                + centered_cube(
                    format!(
                        "closed_media_analyte_carryover_lane_{lane}_sample_loop_volume_marker_{marker}"
                    ),
                    26.0,
                    8.0,
                    6.0,
                )
                .translate(
                    x,
                    -SELECTOR_Y / 2.0 + 28.0 + marker as f64 * 28.0,
                    SELECTOR_Z / 2.0 + 3.0,
                );
        }
    }
    saddles
}

fn selector_closed_lid_latches() -> Part {
    let rear = centered_cube(
        "closed_media_analyte_carryover_selector_rear_closed_lid_hinge",
        SELECTOR_X,
        12.0,
        24.0,
    )
    .translate(0.0, SELECTOR_Y / 2.0 - 6.0, SELECTOR_Z / 2.0 + 12.0);
    let front = centered_cube(
        "closed_media_analyte_carryover_selector_front_closed_lid_latch",
        SELECTOR_X,
        12.0,
        22.0,
    )
    .translate(0.0, -(SELECTOR_Y / 2.0 - 6.0), SELECTOR_Z / 2.0 + 11.0);
    rear + front
}

fn rinse_blank_challenge_path() -> Part {
    let body = centered_cube(
        "closed_media_analyte_carryover_rinse_blank_challenge_body",
        RINSE_PATH_X,
        RINSE_PATH_Y,
        RINSE_PATH_Z,
    );
    body - rinse_reservoir_socket_cuts() - blank_challenge_socket_cuts() - rinse_blank_bores()
        + rinse_header_rails()
        + blank_challenge_identity_lands()
        + one_way_valve_pads()
        + rinse_dwell_token_strip()
}

fn rinse_reservoir_socket_cuts() -> Part {
    let mut sockets = Part::empty("closed_media_analyte_carryover_rinse_reservoir_socket_cuts");
    for i in 0..RINSE_RESERVOIR_SOCKETS {
        sockets = sockets
            + centered_cylinder(
                format!("closed_media_analyte_carryover_rinse_reservoir_socket_{i}"),
                RINSE_SOCKET_D / 2.0,
                RINSE_PATH_Z + 4.0,
                CYLINDER_SEGMENTS,
            )
            .translate(
                centered_index(i, RINSE_RESERVOIR_SOCKETS, RINSE_PITCH_X),
                RINSE_PATH_Y / 2.0 - 40.0,
                0.0,
            );
    }
    sockets
}

fn blank_challenge_socket_cuts() -> Part {
    let mut sockets = Part::empty("closed_media_analyte_carryover_blank_challenge_socket_cuts");
    for i in 0..BLANK_CHALLENGE_POSITIONS {
        sockets = sockets
            + centered_cylinder(
                format!("closed_media_analyte_carryover_blank_challenge_socket_{i}"),
                BLANK_SOCKET_D / 2.0,
                RINSE_PATH_Z + 4.0,
                CYLINDER_SEGMENTS,
            )
            .translate(
                centered_index(i, BLANK_CHALLENGE_POSITIONS, RINSE_PITCH_X),
                -RINSE_PATH_Y / 2.0 + 42.0,
                0.0,
            );
    }
    sockets
}

fn rinse_blank_bores() -> Part {
    let mut bores = Part::empty("closed_media_analyte_carryover_rinse_blank_bores");
    for branch in 0..RINSE_BRANCHES {
        let x = centered_index(branch, RINSE_BRANCHES, RINSE_PITCH_X);
        bores = bores
            + centered_cylinder(
                format!("closed_media_analyte_carryover_rinse_branch_{branch}_bore"),
                SAMPLE_LOOP_BORE_D / 2.0,
                RINSE_PATH_Y + 16.0,
                CYLINDER_SEGMENTS,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, 0.0);
    }
    bores
}

fn rinse_header_rails() -> Part {
    let inlet = centered_cube(
        "closed_media_analyte_carryover_rinse_blank_inlet_header_rail",
        RINSE_PATH_X - 42.0,
        8.0,
        9.0,
    )
    .translate(0.0, 18.0, RINSE_PATH_Z / 2.0 + 4.5);
    let outlet = centered_cube(
        "closed_media_analyte_carryover_rinse_blank_outlet_header_rail",
        RINSE_PATH_X - 42.0,
        8.0,
        9.0,
    )
    .translate(0.0, -18.0, RINSE_PATH_Z / 2.0 + 4.5);
    inlet + outlet
}

fn blank_challenge_identity_lands() -> Part {
    let mut lands = Part::empty("closed_media_analyte_carryover_blank_challenge_identity_lands");
    for i in 0..BLANK_CHALLENGE_POSITIONS {
        lands = lands
            + centered_cube(
                format!("closed_media_analyte_carryover_blank_challenge_identity_land_{i}"),
                40.0,
                12.0,
                5.0,
            )
            .translate(
                centered_index(i, BLANK_CHALLENGE_POSITIONS, RINSE_PITCH_X),
                -RINSE_PATH_Y / 2.0 + 18.0,
                RINSE_PATH_Z / 2.0 + 2.5,
            );
    }
    lands
}

fn one_way_valve_pads() -> Part {
    let mut pads = Part::empty("closed_media_analyte_carryover_rinse_one_way_valve_pads");
    for i in 0..RINSE_BRANCHES {
        pads = pads
            + centered_cube(
                format!("closed_media_analyte_carryover_rinse_one_way_valve_pad_{i}"),
                30.0,
                16.0,
                9.0,
            )
            .translate(
                centered_index(i, RINSE_BRANCHES, RINSE_PITCH_X),
                0.0,
                RINSE_PATH_Z / 2.0 + 4.5,
            );
    }
    pads
}

fn rinse_dwell_token_strip() -> Part {
    centered_cube(
        "closed_media_analyte_carryover_rinse_blank_dwell_token_strip",
        RINSE_PATH_X - 44.0,
        14.0,
        5.0,
    )
    .translate(0.0, RINSE_PATH_Y / 2.0 - 18.0, RINSE_PATH_Z / 2.0 + 2.5)
}

fn segregated_waste_caddy() -> Part {
    let body = centered_cube(
        "closed_media_analyte_carryover_segregated_waste_caddy_body",
        WASTE_X,
        WASTE_Y,
        WASTE_Z,
    );
    body - waste_cup_cuts()
        + waste_stream_dividers()
        + waste_cap_parking_posts()
        + waste_stream_status_lands()
}

fn waste_cup_cuts() -> Part {
    let mut cups = Part::empty("closed_media_analyte_carryover_waste_cup_cuts");
    for stream in 0..WASTE_STREAMS {
        let y = centered_index(stream, WASTE_STREAMS, 42.0);
        for cup in 0..WASTE_CUPS_PER_STREAM {
            let x = centered_index(cup, WASTE_CUPS_PER_STREAM, 70.0);
            let index = stream * WASTE_CUPS_PER_STREAM + cup;
            cups = cups
                + centered_cylinder(
                    format!(
                        "closed_media_analyte_carryover_{}_waste_cup_{index}",
                        waste_stream_name(stream)
                    ),
                    WASTE_CUP_D / 2.0,
                    WASTE_Z + 4.0,
                    CYLINDER_SEGMENTS,
                )
                .translate(x, y, 0.0);
        }
    }
    cups
}

fn waste_stream_dividers() -> Part {
    let mut dividers = Part::empty("closed_media_analyte_carryover_waste_stream_dividers");
    for i in 0..(WASTE_STREAMS - 1) {
        dividers = dividers
            + centered_cube(
                format!("closed_media_analyte_carryover_waste_stream_divider_{i}"),
                WASTE_X - 34.0,
                5.0,
                WASTE_Z + 18.0,
            )
            .translate(0.0, centered_index(i, WASTE_STREAMS - 1, 42.0), 9.0);
    }
    dividers
}

fn waste_cap_parking_posts() -> Part {
    let mut posts = Part::empty("closed_media_analyte_carryover_waste_cap_parking_posts");
    for i in 0..WASTE_STREAMS {
        posts = posts
            + centered_cylinder(
                format!("closed_media_analyte_carryover_waste_stream_{i}_cap_parking_post"),
                6.0,
                18.0,
                CYLINDER_SEGMENTS,
            )
            .translate(
                WASTE_X / 2.0 - 28.0,
                centered_index(i, WASTE_STREAMS, 42.0),
                WASTE_Z / 2.0 + 9.0,
            );
    }
    posts
}

fn waste_stream_status_lands() -> Part {
    let mut lands = Part::empty("closed_media_analyte_carryover_waste_stream_status_lands");
    for i in 0..WASTE_STREAMS {
        lands = lands
            + centered_cube(
                format!(
                    "closed_media_analyte_carryover_{}_waste_status_land",
                    waste_stream_name(i)
                ),
                58.0,
                12.0,
                5.0,
            )
            .translate(
                -WASTE_X / 2.0 + 38.0,
                centered_index(i, WASTE_STREAMS, 42.0),
                WASTE_Z / 2.0 + 2.5,
            );
    }
    lands
}

fn waste_stream_name(index: usize) -> &'static str {
    match index {
        0 => "rinse_blank",
        1 => "calibration_standard",
        2 => "sample_carryover",
        _ => "sensor_prime",
    }
}

fn sensor_cartridge_dock() -> Part {
    let body = centered_cube(
        "closed_media_analyte_carryover_sensor_cartridge_dock_body",
        SENSOR_DOCK_X,
        SENSOR_DOCK_Y,
        SENSOR_DOCK_Z,
    );
    body - sensor_cartridge_pockets() - sensor_flow_bores()
        + sensor_dock_clamp_rails()
        + sensor_pogo_check_towers()
        + sensor_alignment_datum_posts()
        + sensor_service_latch_bar()
}

fn sensor_cartridge_pockets() -> Part {
    let mut pockets = Part::empty("closed_media_analyte_carryover_sensor_cartridge_pockets");
    for dock in 0..SENSOR_CARTRIDGE_DOCKS {
        let x = centered_index(dock, SENSOR_CARTRIDGE_DOCKS, SENSOR_DOCK_PITCH_X);
        pockets = pockets
            + centered_cube(
                format!("closed_media_analyte_carryover_sensor_cartridge_pocket_{dock}"),
                SENSOR_CARTRIDGE_X,
                SENSOR_CARTRIDGE_Y,
                40.0,
            )
            .translate(x, 0.0, SENSOR_DOCK_Z / 2.0 - 18.0);
    }
    pockets
}

fn sensor_flow_bores() -> Part {
    let mut bores = Part::empty("closed_media_analyte_carryover_sensor_flow_bores");
    for dock in 0..SENSOR_CARTRIDGE_DOCKS {
        let x = centered_index(dock, SENSOR_CARTRIDGE_DOCKS, SENSOR_DOCK_PITCH_X);
        for offset in [-10.0, 10.0] {
            bores = bores
                + centered_cylinder(
                    format!("closed_media_analyte_carryover_sensor_dock_{dock}_flow_bore_{offset}"),
                    SAMPLE_LOOP_BORE_D / 2.0,
                    SENSOR_DOCK_Y + 18.0,
                    CYLINDER_SEGMENTS,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(x + offset, 0.0, 4.0);
        }
    }
    bores
}

fn sensor_dock_clamp_rails() -> Part {
    let mut rails = Part::empty("closed_media_analyte_carryover_sensor_dock_clamp_rails");
    for dock in 0..SENSOR_CARTRIDGE_DOCKS {
        let x = centered_index(dock, SENSOR_CARTRIDGE_DOCKS, SENSOR_DOCK_PITCH_X);
        rails = rails
            + centered_cube(
                format!("closed_media_analyte_carryover_sensor_dock_{dock}_left_clamp_rail"),
                5.0,
                SENSOR_CARTRIDGE_Y + 20.0,
                18.0,
            )
            .translate(
                x - SENSOR_CARTRIDGE_X / 2.0 - 5.0,
                0.0,
                SENSOR_DOCK_Z / 2.0 + 9.0,
            )
            + centered_cube(
                format!("closed_media_analyte_carryover_sensor_dock_{dock}_right_clamp_rail"),
                5.0,
                SENSOR_CARTRIDGE_Y + 20.0,
                18.0,
            )
            .translate(
                x + SENSOR_CARTRIDGE_X / 2.0 + 5.0,
                0.0,
                SENSOR_DOCK_Z / 2.0 + 9.0,
            );
    }
    rails
}

fn sensor_pogo_check_towers() -> Part {
    let mut towers = Part::empty("closed_media_analyte_carryover_sensor_pogo_check_towers");
    for dock in 0..SENSOR_CARTRIDGE_DOCKS {
        let x = centered_index(dock, SENSOR_CARTRIDGE_DOCKS, SENSOR_DOCK_PITCH_X);
        let tower = centered_cube(
            format!("closed_media_analyte_carryover_sensor_dock_{dock}_pogo_tower"),
            34.0,
            18.0,
            42.0,
        )
        .translate(x, SENSOR_DOCK_Y / 2.0 - 22.0, SENSOR_DOCK_Z / 2.0 + 21.0);
        let mut pins = Part::empty(format!(
            "closed_media_analyte_carryover_sensor_dock_{dock}_pogo_pin_lands"
        ));
        for pin in 0..POGO_PINS_PER_DOCK {
            pins = pins
                + centered_cylinder(
                    format!(
                        "closed_media_analyte_carryover_sensor_dock_{dock}_pogo_pin_land_{pin}"
                    ),
                    2.0,
                    3.0,
                    16,
                )
                .translate(
                    x + centered_index(pin, POGO_PINS_PER_DOCK, 5.0),
                    SENSOR_DOCK_Y / 2.0 - 12.0,
                    SENSOR_DOCK_Z / 2.0 + 43.5,
                );
        }
        towers = towers + tower + pins;
    }
    towers
}

fn sensor_alignment_datum_posts() -> Part {
    let mut posts = Part::empty("closed_media_analyte_carryover_sensor_alignment_posts");
    for (i, (x, y)) in [
        (-SENSOR_DOCK_X / 2.0 + 24.0, -SENSOR_DOCK_Y / 2.0 + 24.0),
        (SENSOR_DOCK_X / 2.0 - 24.0, -SENSOR_DOCK_Y / 2.0 + 24.0),
        (-SENSOR_DOCK_X / 2.0 + 24.0, SENSOR_DOCK_Y / 2.0 - 24.0),
        (SENSOR_DOCK_X / 2.0 - 24.0, SENSOR_DOCK_Y / 2.0 - 24.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cylinder(
                format!("closed_media_analyte_carryover_sensor_alignment_post_{i}"),
                5.0,
                16.0,
                CYLINDER_SEGMENTS,
            )
            .translate(*x, *y, SENSOR_DOCK_Z / 2.0 + 8.0);
    }
    posts
}

fn sensor_service_latch_bar() -> Part {
    centered_cube(
        "closed_media_analyte_carryover_sensor_service_latch_bar",
        SENSOR_DOCK_X - 32.0,
        12.0,
        26.0,
    )
    .translate(0.0, -SENSOR_DOCK_Y / 2.0 + 10.0, SENSOR_DOCK_Z / 2.0 + 13.0)
}

fn bubble_dead_volume_witness_geometry() -> Part {
    let body = centered_cube(
        "closed_media_analyte_carryover_bubble_dead_volume_witness_body",
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    );
    body - bubble_witness_chamber_cuts() - dead_volume_coupon_recesses()
        + bubble_witness_rims()
        + dead_volume_coupon_lands()
        + bubble_reference_ladder()
        + transparent_window_guard_rails()
}

fn bubble_witness_chamber_cuts() -> Part {
    let mut cuts = Part::empty("closed_media_analyte_carryover_bubble_witness_chamber_cuts");
    for chamber in 0..BUBBLE_WITNESS_CHAMBERS {
        cuts = cuts
            + centered_cylinder(
                format!("closed_media_analyte_carryover_bubble_witness_chamber_{chamber}"),
                WITNESS_CHAMBER_D / 2.0,
                WITNESS_Z + 4.0,
                CYLINDER_SEGMENTS,
            )
            .translate(
                centered_index(chamber, BUBBLE_WITNESS_CHAMBERS, WITNESS_PITCH_X),
                WITNESS_Y / 2.0 - 60.0,
                0.0,
            );
    }
    cuts
}

fn bubble_witness_rims() -> Part {
    let mut rims = Part::empty("closed_media_analyte_carryover_bubble_witness_rims");
    for chamber in 0..BUBBLE_WITNESS_CHAMBERS {
        let x = centered_index(chamber, BUBBLE_WITNESS_CHAMBERS, WITNESS_PITCH_X);
        let outer = centered_cylinder(
            format!("closed_media_analyte_carryover_bubble_witness_rim_{chamber}"),
            WITNESS_CHAMBER_D / 2.0 + 4.0,
            5.0,
            CYLINDER_SEGMENTS,
        )
        .translate(x, WITNESS_Y / 2.0 - 60.0, WITNESS_Z / 2.0 + 2.5);
        let inner = centered_cylinder(
            format!("closed_media_analyte_carryover_bubble_witness_rim_opening_{chamber}"),
            WITNESS_CHAMBER_D / 2.0,
            6.0,
            CYLINDER_SEGMENTS,
        )
        .translate(x, WITNESS_Y / 2.0 - 60.0, WITNESS_Z / 2.0 + 2.5);
        rims = rims + (outer - inner);
    }
    rims
}

fn dead_volume_coupon_recesses() -> Part {
    let mut recesses = Part::empty("closed_media_analyte_carryover_dead_volume_coupon_recesses");
    for coupon in 0..DEAD_VOLUME_COUPONS {
        recesses = recesses
            + centered_cube(
                format!("closed_media_analyte_carryover_dead_volume_coupon_recess_{coupon}"),
                28.0,
                38.0,
                16.0,
            )
            .translate(
                centered_index(coupon, DEAD_VOLUME_COUPONS, WITNESS_PITCH_X),
                -WITNESS_Y / 2.0 + 52.0,
                WITNESS_Z / 2.0 - 7.0,
            );
    }
    recesses
}

fn dead_volume_coupon_lands() -> Part {
    let mut lands = Part::empty("closed_media_analyte_carryover_dead_volume_coupon_lands");
    for coupon in 0..DEAD_VOLUME_COUPONS {
        lands = lands
            + centered_cube(
                format!("closed_media_analyte_carryover_dead_volume_coupon_identity_land_{coupon}"),
                30.0,
                8.0,
                5.0,
            )
            .translate(
                centered_index(coupon, DEAD_VOLUME_COUPONS, WITNESS_PITCH_X),
                -WITNESS_Y / 2.0 + 22.0,
                WITNESS_Z / 2.0 + 2.5,
            );
    }
    lands
}

fn bubble_reference_ladder() -> Part {
    let mut ladder = Part::empty("closed_media_analyte_carryover_bubble_reference_ladder");
    for step in 0..BUBBLE_REFERENCE_STEPS {
        ladder = ladder
            + centered_cube(
                format!("closed_media_analyte_carryover_bubble_reference_step_{step}"),
                44.0 + step as f64 * 9.0,
                5.0,
                5.0,
            )
            .translate(
                -WITNESS_X / 2.0 + 62.0,
                centered_index(step, BUBBLE_REFERENCE_STEPS, 18.0),
                WITNESS_Z / 2.0 + 2.5,
            );
    }
    ladder
}

fn transparent_window_guard_rails() -> Part {
    let upper = centered_cube(
        "closed_media_analyte_carryover_witness_upper_window_guard_rail",
        WITNESS_X - 34.0,
        6.0,
        12.0,
    )
    .translate(0.0, 16.0, WITNESS_Z / 2.0 + 6.0);
    let lower = centered_cube(
        "closed_media_analyte_carryover_witness_lower_window_guard_rail",
        WITNESS_X - 34.0,
        6.0,
        12.0,
    )
    .translate(0.0, -16.0, WITNESS_Z / 2.0 + 6.0);
    upper + lower
}

fn barcode_status_surfaces() -> Part {
    let plate = centered_cube(
        "closed_media_analyte_carryover_barcode_status_plate",
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    );
    plate - status_token_slot_cuts()
        + barcode_lands()
        + run_record_card_lands()
        + status_lane_dividers()
        + traceability_fiducials()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("closed_media_analyte_carryover_barcode_lands");
    for i in 0..BARCODE_LANDS {
        let row = i / 8;
        let col = i % 8;
        lands = lands
            + centered_cube(
                format!("closed_media_analyte_carryover_barcode_land_{i}"),
                34.0,
                14.0,
                4.0,
            )
            .translate(
                centered_index(col, 8, 40.0),
                TRACE_Y / 2.0 - 26.0 - row as f64 * 24.0,
                TRACE_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn status_token_slot_cuts() -> Part {
    let mut slots = Part::empty("closed_media_analyte_carryover_status_token_slot_cuts");
    for lane in 0..STATUS_LANES {
        let y = -TRACE_Y / 2.0 + 24.0 + lane as f64 * 24.0;
        for token in 0..STATUS_TOKENS_PER_LANE {
            slots = slots
                + centered_cube(
                    format!("closed_media_analyte_carryover_status_lane_{lane}_token_slot_{token}"),
                    32.0,
                    12.0,
                    TRACE_Z + 4.0,
                )
                .translate(
                    centered_index(token, STATUS_TOKENS_PER_LANE, 42.0),
                    y,
                    0.0,
                );
        }
    }
    slots
}

fn run_record_card_lands() -> Part {
    let mut lands = Part::empty("closed_media_analyte_carryover_run_record_card_lands");
    for i in 0..RUN_RECORD_CARD_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_media_analyte_carryover_run_record_card_land_{i}"),
                62.0,
                16.0,
                4.0,
            )
            .translate(
                -TRACE_X / 2.0 + 52.0,
                centered_index(i, RUN_RECORD_CARD_LANDS, 28.0),
                TRACE_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn status_lane_dividers() -> Part {
    let mut dividers = Part::empty("closed_media_analyte_carryover_status_lane_dividers");
    for lane in 0..(STATUS_LANES - 1) {
        dividers = dividers
            + centered_cube(
                format!("closed_media_analyte_carryover_status_lane_divider_{lane}"),
                204.0,
                4.0,
                10.0,
            )
            .translate(
                68.0,
                -TRACE_Y / 2.0 + 36.0 + lane as f64 * 24.0,
                TRACE_Z / 2.0 + 5.0,
            );
    }
    dividers
}

fn traceability_fiducials() -> Part {
    fiducial_disc("closed_media_analyte_carryover_traceability_left_fiducial").translate(
        -TRACE_X / 2.0 + 24.0,
        TRACE_Y / 2.0 - 20.0,
        TRACE_Z / 2.0 + 2.0,
    ) + fiducial_disc("closed_media_analyte_carryover_traceability_right_fiducial").translate(
        TRACE_X / 2.0 - 24.0,
        TRACE_Y / 2.0 - 20.0,
        TRACE_Z / 2.0 + 2.0,
    )
}

fn closed_connector_handoff_bulkhead() -> Part {
    let body = centered_cube(
        "closed_media_analyte_carryover_closed_connector_handoff_bulkhead_body",
        HANDOFF_X,
        HANDOFF_Y,
        HANDOFF_Z,
    );
    body - connector_port_bores()
        + connector_boss_rings()
        + bulkhead_gasket_land()
        + keyed_handoff_tabs()
        + closed_connector_strain_relief_comb()
}

fn connector_port_bores() -> Part {
    let mut bores = Part::empty("closed_media_analyte_carryover_connector_port_bores");
    for port in 0..CLOSED_CONNECTOR_PORTS {
        let col = port % CONNECTOR_COLS;
        let row = port / CONNECTOR_COLS;
        bores = bores
            + centered_cylinder(
                format!("closed_media_analyte_carryover_closed_connector_port_bore_{port}"),
                CONNECTOR_PORT_D / 2.0,
                HANDOFF_Y + 18.0,
                CYLINDER_SEGMENTS,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(col, CONNECTOR_COLS, CONNECTOR_PITCH_X),
                0.0,
                centered_index(row, CONNECTOR_ROWS, CONNECTOR_PITCH_Z),
            );
    }
    bores
}

fn connector_boss_rings() -> Part {
    let mut rings = Part::empty("closed_media_analyte_carryover_connector_boss_rings");
    for port in 0..CLOSED_CONNECTOR_PORTS {
        let col = port % CONNECTOR_COLS;
        let row = port / CONNECTOR_COLS;
        let x = centered_index(col, CONNECTOR_COLS, CONNECTOR_PITCH_X);
        let z = centered_index(row, CONNECTOR_ROWS, CONNECTOR_PITCH_Z);
        let boss = centered_cylinder(
            format!("closed_media_analyte_carryover_closed_connector_boss_{port}"),
            CONNECTOR_PORT_D / 2.0 + 6.0,
            8.0,
            CYLINDER_SEGMENTS,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -HANDOFF_Y / 2.0 - 4.0, z);
        let hole = centered_cylinder(
            format!("closed_media_analyte_carryover_closed_connector_boss_opening_{port}"),
            CONNECTOR_PORT_D / 2.0,
            9.0,
            CYLINDER_SEGMENTS,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -HANDOFF_Y / 2.0 - 4.0, z);
        rings = rings + (boss - hole);
    }
    rings
}

fn bulkhead_gasket_land() -> Part {
    centered_cube(
        "closed_media_analyte_carryover_bulkhead_gasket_compression_land",
        HANDOFF_X - 34.0,
        8.0,
        HANDOFF_Z - 20.0,
    )
    .translate(0.0, -HANDOFF_Y / 2.0 - 4.0, 0.0)
}

fn keyed_handoff_tabs() -> Part {
    let mut tabs = Part::empty("closed_media_analyte_carryover_keyed_handoff_tabs");
    for i in 0..CONNECTOR_COLS {
        tabs = tabs
            + centered_cube(
                format!("closed_media_analyte_carryover_handoff_key_tab_{i}"),
                28.0,
                12.0,
                8.0,
            )
            .translate(
                centered_index(i, CONNECTOR_COLS, CONNECTOR_PITCH_X),
                HANDOFF_Y / 2.0 - 16.0,
                HANDOFF_Z / 2.0 + 4.0,
            );
    }
    tabs
}

fn closed_connector_strain_relief_comb() -> Part {
    let spine = centered_cube(
        "closed_media_analyte_carryover_handoff_strain_relief_spine",
        HANDOFF_X - 42.0,
        12.0,
        18.0,
    )
    .translate(0.0, HANDOFF_Y / 2.0 - 12.0, -HANDOFF_Z / 2.0 + 18.0);
    let mut teeth = Part::empty("closed_media_analyte_carryover_handoff_strain_relief_teeth");
    for i in 0..STRAIN_RELIEF_CLAMPS {
        teeth = teeth
            + centered_cube(
                format!("closed_media_analyte_carryover_handoff_strain_relief_clamp_{i}"),
                18.0,
                18.0,
                22.0,
            )
            .translate(
                centered_index(i, STRAIN_RELIEF_CLAMPS, 54.0),
                HANDOFF_Y / 2.0 - 10.0,
                -HANDOFF_Z / 2.0 + 28.0,
            );
    }
    spine + teeth
}

fn analyzer_perfusion_rack_interface_bridge() -> Part {
    let body = centered_cube(
        "closed_media_analyte_carryover_analyzer_perfusion_interface_body",
        INTERFACE_X,
        INTERFACE_Y,
        INTERFACE_Z,
    );
    body - analyzer_dock_window_cuts()
        + analyzer_dock_alignment_rails()
        + perfusion_rack_latch_blocks()
        + rack_handoff_connector_lands()
        + interface_strain_relief_clamps()
}

fn analyzer_dock_window_cuts() -> Part {
    let mut windows = Part::empty("closed_media_analyte_carryover_analyzer_dock_window_cuts");
    for i in 0..ANALYZER_DOCK_WINDOWS {
        windows = windows
            + centered_cube(
                format!("closed_media_analyte_carryover_analyzer_dock_window_{i}"),
                76.0,
                82.0,
                INTERFACE_Z + 4.0,
            )
            .translate(centered_index(i, ANALYZER_DOCK_WINDOWS, 92.0), -16.0, 0.0);
    }
    windows
}

fn analyzer_dock_alignment_rails() -> Part {
    let left = centered_cube(
        "closed_media_analyte_carryover_analyzer_left_alignment_rail",
        10.0,
        INTERFACE_Y - 30.0,
        18.0,
    )
    .translate(-INTERFACE_X / 2.0 + 30.0, -4.0, INTERFACE_Z / 2.0 + 9.0);
    let right = centered_cube(
        "closed_media_analyte_carryover_perfusion_rack_right_alignment_rail",
        10.0,
        INTERFACE_Y - 30.0,
        18.0,
    )
    .translate(INTERFACE_X / 2.0 - 30.0, -4.0, INTERFACE_Z / 2.0 + 9.0);
    left + right
}

fn perfusion_rack_latch_blocks() -> Part {
    let mut latches = Part::empty("closed_media_analyte_carryover_perfusion_rack_latches");
    for i in 0..PERFUSION_RACK_LATCHES {
        latches = latches
            + centered_cube(
                format!("closed_media_analyte_carryover_perfusion_rack_latch_{i}"),
                34.0,
                12.0,
                24.0,
            )
            .translate(
                centered_index(i, PERFUSION_RACK_LATCHES, 58.0),
                INTERFACE_Y / 2.0 - 14.0,
                INTERFACE_Z / 2.0 + 12.0,
            );
    }
    latches
}

fn rack_handoff_connector_lands() -> Part {
    let mut lands = Part::empty("closed_media_analyte_carryover_rack_handoff_connector_lands");
    for i in 0..RINSE_BRANCHES {
        lands = lands
            + centered_cylinder(
                format!("closed_media_analyte_carryover_rack_handoff_connector_land_{i}"),
                10.0,
                6.0,
                CYLINDER_SEGMENTS,
            )
            .translate(
                centered_index(i, RINSE_BRANCHES, 44.0),
                -INTERFACE_Y / 2.0 + 20.0,
                INTERFACE_Z / 2.0 + 3.0,
            );
    }
    lands
}

fn interface_strain_relief_clamps() -> Part {
    let mut clamps = Part::empty("closed_media_analyte_carryover_interface_strain_relief_clamps");
    for i in 0..STRAIN_RELIEF_CLAMPS {
        clamps = clamps
            + centered_cube(
                format!("closed_media_analyte_carryover_interface_strain_relief_clamp_{i}"),
                18.0,
                14.0,
                16.0,
            )
            .translate(
                centered_index(i, STRAIN_RELIEF_CLAMPS, 36.0),
                0.0,
                INTERFACE_Z / 2.0 + 8.0,
            );
    }
    clamps
}

fn evidence_bridge() -> Part {
    let left_post = centered_cube(
        "closed_media_analyte_carryover_evidence_bridge_left_post",
        32.0,
        EVIDENCE_BRIDGE_Y,
        EVIDENCE_BRIDGE_Z,
    )
    .translate(-EVIDENCE_BRIDGE_X / 2.0 + 16.0, 0.0, 0.0);
    let right_post = centered_cube(
        "closed_media_analyte_carryover_evidence_bridge_right_post",
        32.0,
        EVIDENCE_BRIDGE_Y,
        EVIDENCE_BRIDGE_Z,
    )
    .translate(EVIDENCE_BRIDGE_X / 2.0 - 16.0, 0.0, 0.0);
    let beam = centered_cube(
        "closed_media_analyte_carryover_evidence_bridge_cross_beam",
        EVIDENCE_BRIDGE_X,
        32.0,
        30.0,
    )
    .translate(0.0, 0.0, EVIDENCE_BRIDGE_Z / 2.0 - 15.0);
    left_post + right_post + beam + evidence_camera_pods() + evidence_light_segments()
}

fn evidence_camera_pods() -> Part {
    let mut pods = Part::empty("closed_media_analyte_carryover_evidence_camera_pods");
    for i in 0..EVIDENCE_CAMERA_COUNT {
        let x = centered_index(i, EVIDENCE_CAMERA_COUNT, 260.0);
        let pod = centered_cube(
            format!("closed_media_analyte_carryover_evidence_camera_pod_{i}"),
            56.0,
            28.0,
            24.0,
        )
        .translate(x, 0.0, EVIDENCE_BRIDGE_Z / 2.0 - 48.0);
        let lens = centered_cylinder(
            format!("closed_media_analyte_carryover_evidence_camera_lens_bore_{i}"),
            8.0,
            30.0,
            CYLINDER_SEGMENTS,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -2.0, EVIDENCE_BRIDGE_Z / 2.0 - 48.0);
        pods = pods + (pod - lens);
    }
    pods
}

fn evidence_light_segments() -> Part {
    let mut lights = Part::empty("closed_media_analyte_carryover_evidence_light_segments");
    for i in 0..EVIDENCE_LIGHT_SEGMENTS {
        lights = lights
            + centered_cube(
                format!("closed_media_analyte_carryover_evidence_light_segment_{i}"),
                118.0,
                12.0,
                8.0,
            )
            .translate(
                centered_index(i, EVIDENCE_LIGHT_SEGMENTS, 150.0),
                24.0,
                EVIDENCE_BRIDGE_Z / 2.0 - 34.0,
            );
    }
    lights
}

fn robot_service_keepout_gauges() -> Part {
    let front = keepout_gauge(
        "closed_media_analyte_carryover_front_robot_sweep_keepout_gauge",
        STATION_X - 180.0,
        FRONT_ROBOT_CLEARANCE,
        (0.0, -STATION_Y / 2.0 + FRONT_ROBOT_CLEARANCE / 2.0),
    );
    let rear = keepout_gauge(
        "closed_media_analyte_carryover_rear_service_keepout_gauge",
        STATION_X - 170.0,
        REAR_SERVICE_CLEARANCE,
        (0.0, STATION_Y / 2.0 - REAR_SERVICE_CLEARANCE / 2.0),
    );
    let left = keepout_gauge(
        "closed_media_analyte_carryover_left_standard_service_keepout_gauge",
        LEFT_STANDARD_SERVICE_CLEARANCE,
        STATION_Y - 180.0,
        (
            -STATION_X / 2.0 + LEFT_STANDARD_SERVICE_CLEARANCE / 2.0,
            0.0,
        ),
    );
    let right = keepout_gauge(
        "closed_media_analyte_carryover_right_waste_service_keepout_gauge",
        RIGHT_WASTE_SERVICE_CLEARANCE,
        STATION_Y - 180.0,
        (STATION_X / 2.0 - RIGHT_WASTE_SERVICE_CLEARANCE / 2.0, 0.0),
    );
    let lift = centered_cube(
        "closed_media_analyte_carryover_top_sensor_lift_keepout_gauge",
        SENSOR_DOCK_X + 80.0,
        SENSOR_DOCK_Y + 88.0,
        KEEP_OUT_GAUGE_Z,
    )
    .translate(
        SENSOR_POS.0,
        SENSOR_POS.1,
        BASE_Z / 2.0 + TOP_SENSOR_LIFT_CLEARANCE + KEEP_OUT_GAUGE_Z / 2.0,
    );
    front + rear + left + right + lift + keepout_height_posts()
}

fn keepout_gauge(name: &str, x: f64, y: f64, center: (f64, f64)) -> Part {
    centered_cube(name, x, y, KEEP_OUT_GAUGE_Z).translate(
        center.0,
        center.1,
        BASE_Z / 2.0 + KEEP_OUT_GAUGE_Z / 2.0,
    )
}

fn keepout_height_posts() -> Part {
    let mut posts = Part::empty("closed_media_analyte_carryover_keepout_height_posts");
    for (i, (x, y, z)) in [
        (
            SENSOR_POS.0 - 235.0,
            SENSOR_POS.1 - 140.0,
            TOP_SENSOR_LIFT_CLEARANCE,
        ),
        (
            SENSOR_POS.0 + 235.0,
            SENSOR_POS.1 - 140.0,
            TOP_SENSOR_LIFT_CLEARANCE,
        ),
        (
            SENSOR_POS.0 - 235.0,
            SENSOR_POS.1 + 140.0,
            TOP_SENSOR_LIFT_CLEARANCE,
        ),
        (
            SENSOR_POS.0 + 235.0,
            SENSOR_POS.1 + 140.0,
            TOP_SENSOR_LIFT_CLEARANCE,
        ),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cylinder(
                format!("closed_media_analyte_carryover_sensor_lift_keepout_post_{i}"),
                5.0,
                *z,
                CYLINDER_SEGMENTS,
            )
            .translate(*x, *y, BASE_Z / 2.0 + *z / 2.0);
    }
    posts
}

fn closed_fluid_route_placeholders() -> Part {
    let z = BASE_Z / 2.0 + 62.0;
    tube_span_x(
        "closed_media_analyte_carryover_selector_to_sensor_route",
        264.0,
        (110.0, 104.0, z),
    ) + tube_span_y(
        "closed_media_analyte_carryover_selector_to_sensor_drop_route",
        142.0,
        (242.0, 33.0, z),
    ) + tube_span_x(
        "closed_media_analyte_carryover_rinse_blank_to_sensor_route",
        278.0,
        (324.0, 100.0, z + 10.0),
    ) + tube_span_y(
        "closed_media_analyte_carryover_sensor_to_waste_route",
        114.0,
        (360.0, -76.0, z),
    ) + tube_span_x(
        "closed_media_analyte_carryover_sensor_to_handoff_route",
        420.0,
        (260.0, -168.0, z + 12.0),
    ) + tube_span_x(
        "closed_media_analyte_carryover_handoff_to_perfusion_rack_route",
        214.0,
        (272.0, -276.0, z + 22.0),
    )
}

fn tube_span_x(name: &str, length: f64, center: (f64, f64, f64)) -> Part {
    centered_cube(name, length, SAMPLE_LOOP_BORE_D, SAMPLE_LOOP_BORE_D)
        .translate(center.0, center.1, center.2)
}

fn tube_span_y(name: &str, length: f64, center: (f64, f64, f64)) -> Part {
    centered_cube(name, SAMPLE_LOOP_BORE_D, length, SAMPLE_LOOP_BORE_D)
        .translate(center.0, center.1, center.2)
}

fn fiducial_disc(name: &str) -> Part {
    centered_cylinder(format!("{name}_disc"), 8.0, 4.0, FIDUCIAL_SEGMENTS)
        - centered_cylinder(format!("{name}_center_dot"), 1.8, 5.0, 20)
        - centered_cube(format!("{name}_cross_x"), 13.0, 2.0, 5.0)
        - centered_cube(format!("{name}_cross_y"), 2.0, 13.0, 5.0)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_stable_unique_and_scoped() {
        let expected = [
            "output/closed_media_analyte_sensor_calibration_carryover_station_base_leak_tray_deck.stl",
            "output/closed_media_analyte_sensor_calibration_carryover_station_calibration_standard_custody_nests.stl",
            "output/closed_media_analyte_sensor_calibration_carryover_station_sample_loop_selector_lanes.stl",
            "output/closed_media_analyte_sensor_calibration_carryover_station_rinse_blank_challenge_path.stl",
            "output/closed_media_analyte_sensor_calibration_carryover_station_segregated_waste_caddy.stl",
            "output/closed_media_analyte_sensor_calibration_carryover_station_sensor_cartridge_dock.stl",
            "output/closed_media_analyte_sensor_calibration_carryover_station_bubble_dead_volume_witness_geometry.stl",
            "output/closed_media_analyte_sensor_calibration_carryover_station_barcode_status_surfaces.stl",
            "output/closed_media_analyte_sensor_calibration_carryover_station_closed_connector_handoff_bulkhead.stl",
            "output/closed_media_analyte_sensor_calibration_carryover_station_analyzer_perfusion_rack_interface_bridge.stl",
            "output/closed_media_analyte_sensor_calibration_carryover_station_evidence_bridge.stl",
            "output/closed_media_analyte_sensor_calibration_carryover_station_robot_service_keepout_gauges.stl",
            "output/closed_media_analyte_sensor_calibration_carryover_station_assembly.stl",
        ];
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(OUTPUTS, expected);
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 13);
        for path in OUTPUTS {
            assert!(path.starts_with(OUTPUT_PREFIX), "{path}");
            assert!(path.ends_with(".stl"), "{path}");
        }
    }

    #[test]
    fn requested_feature_groups_are_present_without_protocol_thresholds() {
        assert_eq!(REQUIRED_FEATURES.len(), 12);
        for feature in [
            "calibration_standard_custody_nests",
            "sample_loop_selector_lanes",
            "rinse_blank_challenge_path",
            "segregated_waste_caddy",
            "sensor_cartridge_dock",
            "bubble_dead_volume_witness_geometry",
            "barcode_status_surfaces",
            "closed_connector_handoff_bulkhead",
            "analyzer_perfusion_rack_interface_bridge",
            "evidence_bridge",
            "robot_service_keepout_gauges",
            "closed_validation_fixture_only",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }

        for limitation in [
            "validation_fixture_intent_only",
            "no_clinical_acceptance_thresholds",
            "no_analyte_release_limits",
            "no_metrology_traceability_claim",
            "no_live_cell_acceptance_claim",
        ] {
            assert!(LIMITATIONS.contains(&limitation));
        }
    }

    #[test]
    fn feature_counts_match_carryover_validation_scope() {
        assert_eq!(ANALYTE_FAMILIES, 6);
        assert_eq!(CAL_STANDARD_NESTS, 18);
        assert_eq!(SAMPLE_LANES, 8);
        assert_eq!(SELECTOR_STATES, 4);
        assert_eq!(SAMPLE_LOOP_VOLUME_MARKERS, SAMPLE_LANES * 2);
        assert_eq!(RINSE_BRANCHES, 4);
        assert_eq!(BLANK_CHALLENGE_POSITIONS, 4);
        assert_eq!(WASTE_STREAMS, 4);
        assert_eq!(WASTE_CUP_COUNT, 8);
        assert_eq!(SENSOR_CARTRIDGE_DOCKS, ANALYTE_FAMILIES);
        assert_eq!(BUBBLE_WITNESS_CHAMBERS, SAMPLE_LANES);
        assert_eq!(DEAD_VOLUME_COUPONS, SAMPLE_LANES);
        assert_eq!(BARCODE_LANDS, 16);
        assert_eq!(STATUS_TOKEN_SLOTS, 16);
        assert_eq!(CLOSED_CONNECTOR_PORTS, 12);
        assert_eq!(ANALYZER_DOCK_WINDOWS, 2);
        assert_eq!(PERFUSION_RACK_LATCHES, 4);
    }

    #[test]
    fn station_bounds_and_major_modules_do_not_overlap() {
        assert_layout();
        let footprints = module_footprints();
        for module in footprints {
            assert!(module.fits_inside_station(), "{} does not fit", module.name);
        }
        for a in 0..footprints.len() {
            for b in (a + 1)..footprints.len() {
                assert!(
                    !footprints[a].overlaps(footprints[b], MAJOR_MODULE_GAP_MM),
                    "{} overlaps {}",
                    footprints[a].name,
                    footprints[b].name
                );
            }
        }
        assert!(STATION_X <= 1320.0);
        assert!(STATION_Y <= 850.0);
    }

    #[test]
    fn explicit_reproducibility_controls_are_pinned() {
        assert_eq!(
            PARAMETER_SET_REV,
            "closed-media-analyte-carryover-station-parametric-rev-a"
        );
        assert_eq!(OUTPUT_MANIFEST_REV, "stl-manifest-rev-a");
        assert!(!USES_RANDOMNESS);
        assert_eq!(RANDOM_SEED, 0);
        assert_eq!(CYLINDER_SEGMENTS, 32);
        assert_eq!(FIDUCIAL_SEGMENTS, 36);
        assert_eq!(FACET_TOLERANCE_MM, 0.25);
    }

    #[test]
    fn robot_service_keepouts_and_closed_handoffs_are_sized() {
        assert_eq!(ROBOT_SERVICE_KEEPOUT_ZONES, 5);
        assert!(FRONT_ROBOT_CLEARANCE >= 380.0);
        assert!(REAR_SERVICE_CLEARANCE >= 250.0);
        assert!(LEFT_STANDARD_SERVICE_CLEARANCE >= 200.0);
        assert!(RIGHT_WASTE_SERVICE_CLEARANCE >= 200.0);
        assert!(TOP_SENSOR_LIFT_CLEARANCE >= CAMERA_CLEARANCE_Z);
        assert_eq!(CLOSED_CONNECTOR_PORTS, CONNECTOR_COLS * CONNECTOR_ROWS);
        assert_eq!(STRAIN_RELIEF_CLAMPS, CONNECTOR_COLS);
        assert!(CONNECTOR_PORT_D > SAMPLE_LOOP_BORE_D);
    }
}
