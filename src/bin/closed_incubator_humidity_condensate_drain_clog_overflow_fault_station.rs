use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed incubator humidity condensate drain clog/overflow fault station.
//
// This standalone generator models mechanical validation packaging for a
// closed incubator humidity drain fault fixture. It reserves a containment
// deck, water-pan surrogate, drain-trap clog challenge cartridge, overflow
// witness path, dye tracer wells, microbial/biofilm coupon rack, level and
// wetness sensor pockets, pressure/equalization vent check features, release
// hold reject lanes, alarm/event tokens, custody lands, evidence camera bridge,
// and robot/service keepout gauges. It defines fixture geometry only.

const OUTPUT_PREFIX: &str =
    "closed_incubator_humidity_condensate_drain_clog_overflow_fault_station";
const PARAMETRIC_REVISION: &str = "LF-CAD-CC-HUMIDITY-DRAIN-CLOG-OVERFLOW-REV-A";
const LAYOUT_REVISION_TOKEN: u64 = 0x864F_0959_C10D_0F10;

const OUTPUTS: [&str; 14] = [
    "output/closed_incubator_humidity_condensate_drain_clog_overflow_fault_station_base_containment_deck.stl",
    "output/closed_incubator_humidity_condensate_drain_clog_overflow_fault_station_humidification_water_pan_surrogate.stl",
    "output/closed_incubator_humidity_condensate_drain_clog_overflow_fault_station_drain_trap_clog_challenge_cartridge.stl",
    "output/closed_incubator_humidity_condensate_drain_clog_overflow_fault_station_overflow_weir_witness_path.stl",
    "output/closed_incubator_humidity_condensate_drain_clog_overflow_fault_station_condensate_dye_tracer_wells.stl",
    "output/closed_incubator_humidity_condensate_drain_clog_overflow_fault_station_microbial_biofilm_coupon_rack.stl",
    "output/closed_incubator_humidity_condensate_drain_clog_overflow_fault_station_level_sensor_wetness_probe_pockets.stl",
    "output/closed_incubator_humidity_condensate_drain_clog_overflow_fault_station_pressure_equalization_vent_checks.stl",
    "output/closed_incubator_humidity_condensate_drain_clog_overflow_fault_station_release_hold_reject_lanes.stl",
    "output/closed_incubator_humidity_condensate_drain_clog_overflow_fault_station_alarm_event_token_rail.stl",
    "output/closed_incubator_humidity_condensate_drain_clog_overflow_fault_station_barcode_custody_lands.stl",
    "output/closed_incubator_humidity_condensate_drain_clog_overflow_fault_station_evidence_camera_bridge.stl",
    "output/closed_incubator_humidity_condensate_drain_clog_overflow_fault_station_robot_service_keepout_gauges.stl",
    "output/closed_incubator_humidity_condensate_drain_clog_overflow_fault_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 14] = [
    "base_containment_deck",
    "humidification_water_pan_surrogate",
    "drain_trap_clog_challenge_cartridge",
    "overflow_weir_witness_path",
    "condensate_dye_tracer_wells",
    "microbial_biofilm_coupon_rack",
    "level_sensor_wetness_probe_pockets",
    "pressure_equalization_vent_checks",
    "release_hold_reject_lanes",
    "alarm_event_token_rail",
    "barcode_custody_lands",
    "evidence_camera_bridge",
    "robot_service_keepout_gauges",
    "named_stl_outputs",
];

const REPRODUCIBILITY_CONTROLS: [&str; 5] = [
    "fixed_output_manifest",
    "stable_part_names",
    "constant_layout_rectangles",
    "no_random_geometry",
    "no_external_configuration",
];

const STATION_X: f64 = 1720.0;
const STATION_Y: f64 = 1060.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 54.0;
const BASIN_DEPTH: f64 = 10.0;
const SOCKET_DEPTH: f64 = 6.0;
const MOUNT_SLOT_COUNT: usize = 10;
const DATUM_TARGET_COUNT: usize = 4;

const PAN_X: f64 = 560.0;
const PAN_Y: f64 = 315.0;
const PAN_Z: f64 = 66.0;
const PAN_POS: (f64, f64) = (-505.0, 255.0);
const PAN_INNER_X: f64 = 460.0;
const PAN_INNER_Y: f64 = 220.0;
const PAN_INNER_DEPTH: f64 = 42.0;
const PAN_WORKING_DEPTH: f64 = 28.0;
const PAN_BAFFLE_COUNT: usize = 6;
const PAN_FILL_MARK_COUNT: usize = 5;
const PAN_DRAIN_PORT_COUNT: usize = 3;
const PAN_HANDLE_COUNT: usize = 4;

const CLOG_X: f64 = 540.0;
const CLOG_Y: f64 = 190.0;
const CLOG_Z: f64 = 58.0;
const CLOG_POS: (f64, f64) = (-505.0, -70.0);
const DRAIN_TRAP_COUNT: usize = 4;
const CLOG_CARTRIDGE_COUNT: usize = 4;
const CLOG_RESTRICTOR_COUNT: usize = 8;
const BYPASS_WITNESS_COUNT: usize = 3;
const TRAP_BORE_D: f64 = 28.0;
const DRAIN_LANE_COUNT: usize = 5;

const WEIR_X: f64 = 540.0;
const WEIR_Y: f64 = 170.0;
const WEIR_Z: f64 = 46.0;
const WEIR_POS: (f64, f64) = (-505.0, -315.0);
const OVERFLOW_WEIR_COUNT: usize = 4;
const WITNESS_WINDOW_COUNT: usize = 8;
const OVERFLOW_PATH_COUNT: usize = 4;
const ABSORBENT_STRIP_COUNT: usize = 6;

const DYE_X: f64 = 430.0;
const DYE_Y: f64 = 180.0;
const DYE_Z: f64 = 44.0;
const DYE_POS: (f64, f64) = (230.0, 345.0);
const DYE_WELL_COLS: usize = 4;
const DYE_WELL_ROWS: usize = 3;
const DYE_TRACER_WELL_COUNT: usize = DYE_WELL_COLS * DYE_WELL_ROWS;
const DYE_WELL_D: f64 = 34.0;
const DYE_WELL_DEPTH: f64 = 24.0;
const DYE_CHANNEL_COUNT: usize = 6;

const COUPON_X: f64 = 470.0;
const COUPON_Y: f64 = 220.0;
const COUPON_Z: f64 = 48.0;
const COUPON_POS: (f64, f64) = (245.0, 110.0);
const COUPON_RACK_COUNT: usize = 4;
const COUPONS_PER_RACK: usize = 5;
const BIOFILM_COUPON_COUNT: usize = COUPON_RACK_COUNT * COUPONS_PER_RACK;
const COUPON_SLOT_X: f64 = 44.0;
const COUPON_SLOT_Y: f64 = 24.0;
const COUPON_SLOT_DEPTH: f64 = 18.0;
const SWAB_LANE_COUNT: usize = 6;

const SENSOR_X: f64 = 470.0;
const SENSOR_Y: f64 = 190.0;
const SENSOR_Z: f64 = 52.0;
const SENSOR_POS: (f64, f64) = (245.0, -125.0);
const LEVEL_SENSOR_COUNT: usize = 5;
const WETNESS_PROBE_COUNT: usize = 6;
const SENSOR_POCKET_X: f64 = 56.0;
const SENSOR_POCKET_Y: f64 = 34.0;
const SENSOR_POCKET_DEPTH: f64 = 24.0;
const CABLE_TROUGH_COUNT: usize = 4;

const VENT_X: f64 = 250.0;
const VENT_Y: f64 = 610.0;
const VENT_Z: f64 = 72.0;
const VENT_POS: (f64, f64) = (650.0, 35.0);
const PRESSURE_PORT_COUNT: usize = 4;
const EQUALIZATION_VENT_COUNT: usize = 4;
const CHECK_FEATURE_COUNT: usize = 4;
const VENT_BORE_D: f64 = 24.0;
const VENT_MAST_Z: f64 = 92.0;

const LANE_X: f64 = 470.0;
const LANE_Y: f64 = 90.0;
const LANE_Z: f64 = 22.0;
const LANE_POS: (f64, f64) = (245.0, -345.0);
const DISPOSITION_LANE_COUNT: usize = 3;
const TOKENS_PER_LANE: usize = 4;
const DISPOSITION_TOKEN_COUNT: usize = DISPOSITION_LANE_COUNT * TOKENS_PER_LANE;

const ALARM_X: f64 = 430.0;
const ALARM_Y: f64 = 80.0;
const ALARM_Z: f64 = 24.0;
const ALARM_POS: (f64, f64) = (-145.0, -450.0);
const ALARM_EVENT_TOKEN_COUNT: usize = 10;
const ALARM_DETENT_COUNT: usize = 10;

const CUSTODY_X: f64 = 430.0;
const CUSTODY_Y: f64 = 80.0;
const CUSTODY_Z: f64 = 14.0;
const CUSTODY_POS: (f64, f64) = (365.0, -450.0);
const BARCODE_LAND_COUNT: usize = 10;
const CUSTODY_SEAL_COUNT: usize = 6;
const RUN_RECORD_LAND_COUNT: usize = 4;

const EVIDENCE_X: f64 = 1410.0;
const EVIDENCE_Y: f64 = 50.0;
const EVIDENCE_BEAM_Z: f64 = 30.0;
const EVIDENCE_CLEARANCE_Z: f64 = 230.0;
const EVIDENCE_POS: (f64, f64) = (0.0, 494.0);
const EVIDENCE_CAMERA_COUNT: usize = 5;
const EVIDENCE_LIGHT_BAR_COUNT: usize = 2;

const KEEP_OUT_X: f64 = 1620.0;
const KEEP_OUT_Y: f64 = 990.0;
const KEEP_OUT_Z: f64 = 6.0;
const ROBOT_FRONT_CLEARANCE_Y: f64 = 300.0;
const SERVICE_REAR_CLEARANCE_Y: f64 = 230.0;
const VENT_SERVICE_CLEARANCE_X: f64 = 80.0;
const PAN_LIFT_CLEARANCE_Z: f64 = 325.0;
const KEEP_OUT_GAUGE_COUNT: usize = 7;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_deck(self) -> bool {
        let usable_x = STATION_X / 2.0 - RIM_W - 16.0;
        let usable_y = STATION_Y / 2.0 - RIM_W - 16.0;

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
enum DispositionLane {
    Release,
    Hold,
    Reject,
}

impl DispositionLane {
    fn all() -> [DispositionLane; DISPOSITION_LANE_COUNT] {
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
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_containment_deck();
    export(OUTPUTS[0], &base);

    let pan = humidification_water_pan_surrogate();
    export(OUTPUTS[1], &pan);

    let clog = drain_trap_clog_challenge_cartridge();
    export(OUTPUTS[2], &clog);

    let weir = overflow_weir_witness_path();
    export(OUTPUTS[3], &weir);

    let dye = condensate_dye_tracer_wells();
    export(OUTPUTS[4], &dye);

    let coupons = microbial_biofilm_coupon_rack();
    export(OUTPUTS[5], &coupons);

    let sensors = level_sensor_wetness_probe_pockets();
    export(OUTPUTS[6], &sensors);

    let vents = pressure_equalization_vent_checks();
    export(OUTPUTS[7], &vents);

    let lanes = release_hold_reject_lanes();
    export(OUTPUTS[8], &lanes);

    let alarms = alarm_event_token_rail();
    export(OUTPUTS[9], &alarms);

    let custody = barcode_custody_lands();
    export(OUTPUTS[10], &custody);

    let evidence = evidence_camera_bridge();
    export(OUTPUTS[11], &evidence);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[12], &keepouts);

    let assembly = base
        + pan.translate(PAN_POS.0, PAN_POS.1, insert_z(PAN_Z))
        + clog.translate(CLOG_POS.0, CLOG_POS.1, insert_z(CLOG_Z))
        + weir.translate(WEIR_POS.0, WEIR_POS.1, insert_z(WEIR_Z))
        + dye.translate(DYE_POS.0, DYE_POS.1, insert_z(DYE_Z))
        + coupons.translate(COUPON_POS.0, COUPON_POS.1, insert_z(COUPON_Z))
        + sensors.translate(SENSOR_POS.0, SENSOR_POS.1, insert_z(SENSOR_Z))
        + vents.translate(VENT_POS.0, VENT_POS.1, insert_z(VENT_Z))
        + lanes.translate(LANE_POS.0, LANE_POS.1, insert_z(LANE_Z))
        + alarms.translate(ALARM_POS.0, ALARM_POS.1, insert_z(ALARM_Z))
        + custody.translate(CUSTODY_POS.0, CUSTODY_POS.1, insert_z(CUSTODY_Z))
        + evidence.translate(EVIDENCE_POS.0, EVIDENCE_POS.1, DECK_Z)
        + keepouts.translate(0.0, 0.0, DECK_Z + KEEP_OUT_Z / 2.0);
    export(OUTPUTS[13], &assembly);

    println!();
    println!("Closed incubator humidity condensate drain clog/overflow fault station:");
    println!(
        "  Footprint:         {STATION_X:.0}mm x {STATION_Y:.0}mm containment deck with {:.0}mL freeboard",
        containment_freeboard_ml()
    );
    println!(
        "  Water path:        {PAN_DRAIN_PORT_COUNT} pan drain ports, {DRAIN_LANE_COUNT} drain lanes, {DRAIN_TRAP_COUNT} trap pockets, {OVERFLOW_PATH_COUNT} overflow witness paths"
    );
    println!(
        "  Challenge set:     {CLOG_CARTRIDGE_COUNT} clog cartridges, {CLOG_RESTRICTOR_COUNT} restrictor lands, {BYPASS_WITNESS_COUNT} bypass witnesses, {DYE_TRACER_WELL_COUNT} dye tracer wells"
    );
    println!(
        "  Sensors/coupons:   {LEVEL_SENSOR_COUNT} level pockets, {WETNESS_PROBE_COUNT} wetness probe pockets, {BIOFILM_COUPON_COUNT} coupon slots"
    );
    println!(
        "  Controls/evidence: {PRESSURE_PORT_COUNT} pressure ports, {EQUALIZATION_VENT_COUNT} equalization vents, {DISPOSITION_TOKEN_COUNT} disposition tokens, {ALARM_EVENT_TOKEN_COUNT} alarm tokens, {EVIDENCE_CAMERA_COUNT} cameras"
    );
    println!(
        "  Revision:          {PARAMETRIC_REVISION} token {LAYOUT_REVISION_TOKEN:016X}; {} required feature groups",
        REQUIRED_FEATURES.len()
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn insert_z(height: f64) -> f64 {
    DECK_Z + height / 2.0 - SOCKET_DEPTH / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 14);
    assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
    assert_eq!(REQUIRED_FEATURES.len(), 14);
    assert_eq!(REPRODUCIBILITY_CONTROLS.len(), 5);
    assert_eq!(PAN_HANDLE_COUNT, 4);
    assert_eq!(BIOFILM_COUPON_COUNT, COUPON_RACK_COUNT * COUPONS_PER_RACK);
    assert_eq!(DYE_TRACER_WELL_COUNT, DYE_WELL_COLS * DYE_WELL_ROWS);
    assert_eq!(DispositionLane::all().len(), DISPOSITION_LANE_COUNT);
    assert_eq!(
        DISPOSITION_TOKEN_COUNT,
        DISPOSITION_LANE_COUNT * TOKENS_PER_LANE
    );
    assert_eq!(CLOG_CARTRIDGE_COUNT, DRAIN_TRAP_COUNT);
    assert_eq!(CLOG_RESTRICTOR_COUNT, DRAIN_TRAP_COUNT * 2);
    assert_eq!(ALARM_EVENT_TOKEN_COUNT, ALARM_DETENT_COUNT);
    assert_eq!(MOUNT_SLOT_COUNT, mount_slot_positions().len());
    assert_eq!(DATUM_TARGET_COUNT, datum_target_positions().len());
    assert!(PAN_WORKING_DEPTH < PAN_INNER_DEPTH);
    assert!(water_pan_working_volume_ml() > clog_fault_charge_volume_ml());
    assert!(overflow_capture_volume_ml() > clog_fault_charge_volume_ml());
    assert!(containment_freeboard_ml() > overflow_capture_volume_ml());
    assert!(EVIDENCE_CLEARANCE_Z > PAN_Z + WEIR_Z);
    assert!(PAN_LIFT_CLEARANCE_Z > EVIDENCE_CLEARANCE_Z);
    assert!(front_robot_clearance() >= ROBOT_FRONT_CLEARANCE_Y);
    assert!(rear_service_clearance() >= SERVICE_REAR_CLEARANCE_Y);
    assert!(vent_service_clearance() >= VENT_SERVICE_CLEARANCE_X);

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

fn socket_rects() -> [Rect; 10] {
    [
        rect("humidification_water_pan_surrogate", PAN_POS, PAN_X, PAN_Y),
        rect(
            "drain_trap_clog_challenge_cartridge",
            CLOG_POS,
            CLOG_X,
            CLOG_Y,
        ),
        rect("overflow_weir_witness_path", WEIR_POS, WEIR_X, WEIR_Y),
        rect("condensate_dye_tracer_wells", DYE_POS, DYE_X, DYE_Y),
        rect(
            "microbial_biofilm_coupon_rack",
            COUPON_POS,
            COUPON_X,
            COUPON_Y,
        ),
        rect(
            "level_sensor_wetness_probe_pockets",
            SENSOR_POS,
            SENSOR_X,
            SENSOR_Y,
        ),
        rect(
            "pressure_equalization_vent_checks",
            VENT_POS,
            VENT_X,
            VENT_Y,
        ),
        rect("release_hold_reject_lanes", LANE_POS, LANE_X, LANE_Y),
        rect("alarm_event_token_rail", ALARM_POS, ALARM_X, ALARM_Y),
        rect("barcode_custody_lands", CUSTODY_POS, CUSTODY_X, CUSTODY_Y),
    ]
}

#[cfg(test)]
fn water_path_count() -> usize {
    PAN_DRAIN_PORT_COUNT + DRAIN_LANE_COUNT + OVERFLOW_PATH_COUNT + DYE_CHANNEL_COUNT
}

#[cfg(test)]
fn drain_fault_path_count() -> usize {
    DRAIN_TRAP_COUNT + CLOG_CARTRIDGE_COUNT + BYPASS_WITNESS_COUNT + OVERFLOW_WEIR_COUNT
}

#[cfg(test)]
fn sensor_pocket_count() -> usize {
    LEVEL_SENSOR_COUNT + WETNESS_PROBE_COUNT + PRESSURE_PORT_COUNT + EQUALIZATION_VENT_COUNT
}

fn water_pan_working_volume_ml() -> f64 {
    PAN_INNER_X * PAN_INNER_Y * PAN_WORKING_DEPTH / 1000.0
}

fn clog_fault_charge_volume_ml() -> f64 {
    DRAIN_TRAP_COUNT as f64 * 85.0 + DYE_TRACER_WELL_COUNT as f64 * 12.0
}

fn overflow_capture_volume_ml() -> f64 {
    let well_radius = DYE_WELL_D / 2.0;
    let dye_capacity = DYE_TRACER_WELL_COUNT as f64
        * std::f64::consts::PI
        * well_radius
        * well_radius
        * DYE_WELL_DEPTH
        / 1000.0;
    let weir_channel_capacity = WEIR_X * (WEIR_Y - 70.0) * 18.0 / 1000.0;
    dye_capacity + weir_channel_capacity
}

fn containment_freeboard_ml() -> f64 {
    let inner_x = STATION_X - 2.0 * RIM_W;
    let inner_y = STATION_Y - 2.0 * RIM_W;
    let freeboard_z = RIM_Z - BASIN_DEPTH;
    inner_x * inner_y * freeboard_z / 1000.0
}

fn front_robot_clearance() -> f64 {
    ROBOT_FRONT_CLEARANCE_Y
}

fn rear_service_clearance() -> f64 {
    STATION_Y / 2.0 - (EVIDENCE_POS.1 + EVIDENCE_Y / 2.0) + EVIDENCE_CLEARANCE_Z
}

fn vent_service_clearance() -> f64 {
    STATION_X / 2.0 - (VENT_POS.0.abs() + VENT_X / 2.0)
}

fn base_containment_deck() -> Part {
    let deck = centered_cube(
        "humidity_drain_fault_base_containment_deck",
        STATION_X,
        STATION_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let basin = centered_cube(
        "humidity_drain_fault_secondary_basin_cut",
        STATION_X - 2.0 * (RIM_W + 54.0),
        STATION_Y - 2.0 * (RIM_W + 52.0),
        BASIN_DEPTH + 0.8,
    )
    .translate(0.0, -12.0, DECK_Z - BASIN_DEPTH / 2.0);
    let front_drain = centered_cylinder(
        "humidity_drain_fault_front_containment_drain_bore",
        8.0,
        60.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 98.0,
        -STATION_Y / 2.0 + 34.0,
        DECK_Z - 8.0,
    );

    deck - basin - front_drain - insert_sockets() - deck_mounting_slots()
        + perimeter_rims()
        + flow_lane_spines()
        + datum_targets()
        + containment_witness_ribs()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("humidity_drain_fault_insert_sockets");
    for item in socket_rects() {
        sockets = sockets
            + centered_cube(
                format!("humidity_drain_fault_{}_socket", item.name),
                item.x + 10.0,
                item.y + 10.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(item.center.0, item.center.1, DECK_Z - SOCKET_DEPTH / 2.0);
    }
    sockets
}

fn deck_mounting_slots() -> Part {
    let mut slots = Part::empty("humidity_drain_fault_mounting_slots");
    for (i, (x, y)) in mount_slot_positions().into_iter().enumerate() {
        slots = slots
            + centered_cylinder(
                format!("humidity_drain_fault_m6_clearance_{i}"),
                3.4,
                DECK_Z + 4.0,
                28,
            )
            .translate(x, y, DECK_Z / 2.0)
            + centered_cube(
                format!("humidity_drain_fault_m6_slot_relief_{i}"),
                30.0,
                7.5,
                DECK_Z + 4.0,
            )
            .translate(x, y, DECK_Z / 2.0);
    }
    slots
}

fn mount_slot_positions() -> [(f64, f64); MOUNT_SLOT_COUNT] {
    [
        (-STATION_X / 2.0 + 60.0, -STATION_Y / 2.0 + 60.0),
        (STATION_X / 2.0 - 60.0, -STATION_Y / 2.0 + 60.0),
        (-STATION_X / 2.0 + 60.0, STATION_Y / 2.0 - 60.0),
        (STATION_X / 2.0 - 60.0, STATION_Y / 2.0 - 60.0),
        (0.0, -STATION_Y / 2.0 + 60.0),
        (0.0, STATION_Y / 2.0 - 60.0),
        (-STATION_X / 2.0 + 60.0, 0.0),
        (STATION_X / 2.0 - 60.0, 0.0),
        (-STATION_X / 4.0, -STATION_Y / 2.0 + 60.0),
        (STATION_X / 4.0, STATION_Y / 2.0 - 60.0),
    ]
}

fn perimeter_rims() -> Part {
    let front = centered_cube(
        "humidity_drain_fault_front_low_robot_rim",
        STATION_X,
        RIM_W,
        30.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, DECK_Z + 15.0);
    let rear = centered_cube(
        "humidity_drain_fault_rear_service_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let left = centered_cube(
        "humidity_drain_fault_left_spill_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "humidity_drain_fault_right_spill_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    front + rear + left + right
}

fn flow_lane_spines() -> Part {
    let water_axis = centered_cube(
        "humidity_drain_fault_water_path_axis_spine",
        11.0,
        705.0,
        10.0,
    )
    .translate(PAN_POS.0, -55.0, DECK_Z + 5.0);
    let evidence_axis = centered_cube(
        "humidity_drain_fault_evidence_side_axis_spine",
        11.0,
        720.0,
        10.0,
    )
    .translate(SENSOR_POS.0, -42.0, DECK_Z + 5.0);
    let vent_axis = centered_cube(
        "humidity_drain_fault_vent_service_axis_spine",
        9.0,
        610.0,
        10.0,
    )
    .translate(VENT_POS.0 - VENT_X / 2.0 - 24.0, VENT_POS.1, DECK_Z + 5.0);
    water_axis + evidence_axis + vent_axis
}

fn datum_targets() -> Part {
    let mut targets = Part::empty("humidity_drain_fault_datum_targets");
    for (i, (x, y)) in datum_target_positions().into_iter().enumerate() {
        targets = targets
            + centered_cylinder(
                format!("humidity_drain_fault_robot_datum_disc_{i}"),
                15.0,
                3.0,
                40,
            )
            .translate(x, y, DECK_Z + 1.5)
            - centered_cylinder(
                format!("humidity_drain_fault_robot_datum_center_bore_{i}"),
                3.2,
                4.0,
                24,
            )
            .translate(x, y, DECK_Z + 1.5);
    }
    targets
}

fn datum_target_positions() -> [(f64, f64); DATUM_TARGET_COUNT] {
    [
        (-STATION_X / 2.0 + 86.0, STATION_Y / 2.0 - 88.0),
        (STATION_X / 2.0 - 86.0, STATION_Y / 2.0 - 88.0),
        (-STATION_X / 2.0 + 86.0, -STATION_Y / 2.0 + 88.0),
        (STATION_X / 2.0 - 86.0, -STATION_Y / 2.0 + 88.0),
    ]
}

fn containment_witness_ribs() -> Part {
    let mut ribs = Part::empty("humidity_drain_fault_containment_witness_ribs");
    for i in 0..7 {
        let y = -360.0 + i as f64 * 118.0;
        ribs = ribs
            + centered_cube(
                format!("humidity_drain_fault_spill_witness_rib_{i}"),
                STATION_X - 190.0,
                4.0,
                5.0,
            )
            .translate(0.0, y, DECK_Z + 2.5);
    }
    ribs
}

fn humidification_water_pan_surrogate() -> Part {
    let body = centered_cube(
        "humidity_drain_fault_water_pan_surrogate_body",
        PAN_X,
        PAN_Y,
        PAN_Z,
    )
    .translate(0.0, 0.0, PAN_Z / 2.0);
    let basin = centered_cube(
        "humidity_drain_fault_water_pan_inner_basin_cut",
        PAN_INNER_X,
        PAN_INNER_Y,
        PAN_INNER_DEPTH + 0.6,
    )
    .translate(0.0, 8.0, PAN_Z - PAN_INNER_DEPTH / 2.0);

    body - basin - water_pan_drain_ports()
        + water_pan_rims()
        + water_pan_fill_marks()
        + water_pan_baffles()
        + water_pan_robot_handles()
}

fn water_pan_drain_ports() -> Part {
    let mut ports = Part::empty("humidity_drain_fault_water_pan_drain_ports");
    for i in 0..PAN_DRAIN_PORT_COUNT {
        let x = centered_index(i, PAN_DRAIN_PORT_COUNT, 88.0);
        ports = ports
            + centered_cylinder(
                format!("humidity_drain_fault_water_pan_drain_port_{i}"),
                5.2,
                40.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -PAN_Y / 2.0 + 28.0, PAN_Z - 14.0);
    }
    ports
}

fn water_pan_rims() -> Part {
    let front = centered_cube(
        "humidity_drain_fault_water_pan_front_low_overflow_lip",
        PAN_X,
        14.0,
        24.0,
    )
    .translate(0.0, -PAN_Y / 2.0 + 7.0, PAN_Z + 12.0);
    let rear = centered_cube(
        "humidity_drain_fault_water_pan_rear_fill_lip",
        PAN_X,
        14.0,
        38.0,
    )
    .translate(0.0, PAN_Y / 2.0 - 7.0, PAN_Z + 19.0);
    let left = centered_cube("humidity_drain_fault_water_pan_left_lip", 14.0, PAN_Y, 34.0)
        .translate(-PAN_X / 2.0 + 7.0, 0.0, PAN_Z + 17.0);
    let right = centered_cube(
        "humidity_drain_fault_water_pan_right_lip",
        14.0,
        PAN_Y,
        34.0,
    )
    .translate(PAN_X / 2.0 - 7.0, 0.0, PAN_Z + 17.0);
    front + rear + left + right
}

fn water_pan_fill_marks() -> Part {
    let mut marks = Part::empty("humidity_drain_fault_water_pan_fill_marks");
    for i in 0..PAN_FILL_MARK_COUNT {
        marks = marks
            + centered_cube(
                format!("humidity_drain_fault_pan_fill_level_tick_{i}"),
                46.0,
                3.0,
                4.0,
            )
            .translate(
                -PAN_X / 2.0 + 42.0,
                74.0 - i as f64 * 28.0,
                PAN_Z + 3.0 + i as f64 * 4.0,
            );
    }
    marks
}

fn water_pan_baffles() -> Part {
    let mut baffles = Part::empty("humidity_drain_fault_water_pan_baffles");
    for i in 0..PAN_BAFFLE_COUNT {
        baffles = baffles
            + centered_cube(
                format!("humidity_drain_fault_pan_condensate_baffle_{i}"),
                8.0,
                PAN_INNER_Y - 38.0,
                28.0,
            )
            .translate(
                centered_index(i, PAN_BAFFLE_COUNT, 74.0),
                8.0,
                PAN_Z - PAN_INNER_DEPTH + 14.0,
            );
    }
    baffles
}

fn water_pan_robot_handles() -> Part {
    let mut handles = Part::empty("humidity_drain_fault_water_pan_robot_handles");
    for (i, x) in [-210.0, -70.0, 70.0, 210.0].into_iter().enumerate() {
        handles = handles
            + centered_cube(
                format!("humidity_drain_fault_pan_robot_handle_land_{i}"),
                54.0,
                18.0,
                10.0,
            )
            .translate(x, PAN_Y / 2.0 + 15.0, PAN_Z - 8.0);
    }
    handles
}

fn drain_trap_clog_challenge_cartridge() -> Part {
    let block = centered_cube(
        "humidity_drain_fault_clog_challenge_cartridge_block",
        CLOG_X,
        CLOG_Y,
        CLOG_Z,
    )
    .translate(0.0, 0.0, CLOG_Z / 2.0);
    block - drain_trap_bores() - drain_lane_slots()
        + clog_cartridge_caps()
        + restrictor_land_pairs()
        + bypass_witness_ports()
        + challenge_flow_arrows()
}

fn drain_trap_bores() -> Part {
    let mut bores = Part::empty("humidity_drain_fault_drain_trap_bores");
    for i in 0..DRAIN_TRAP_COUNT {
        bores = bores
            + centered_cylinder(
                format!("humidity_drain_fault_u_trap_bore_{i}"),
                TRAP_BORE_D / 2.0,
                CLOG_Z + 4.0,
                36,
            )
            .translate(
                centered_index(i, DRAIN_TRAP_COUNT, 110.0),
                18.0,
                CLOG_Z / 2.0,
            )
            + centered_cylinder(
                format!("humidity_drain_fault_trap_cross_bore_{i}"),
                5.0,
                124.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(i, DRAIN_TRAP_COUNT, 110.0),
                -4.0,
                CLOG_Z - 20.0,
            );
    }
    bores
}

fn drain_lane_slots() -> Part {
    let mut slots = Part::empty("humidity_drain_fault_drain_lane_slots");
    for i in 0..DRAIN_LANE_COUNT {
        slots = slots
            + centered_cube(
                format!("humidity_drain_fault_visible_drain_lane_slot_{i}"),
                22.0,
                CLOG_Y - 42.0,
                11.0,
            )
            .translate(centered_index(i, DRAIN_LANE_COUNT, 86.0), 0.0, CLOG_Z - 6.0);
    }
    slots
}

fn clog_cartridge_caps() -> Part {
    let mut caps = Part::empty("humidity_drain_fault_clog_cartridge_caps");
    for i in 0..CLOG_CARTRIDGE_COUNT {
        let x = centered_index(i, CLOG_CARTRIDGE_COUNT, 110.0);
        caps = caps
            + centered_cylinder(
                format!("humidity_drain_fault_removable_clog_cartridge_cap_{i}"),
                24.0,
                12.0,
                40,
            )
            .translate(x, 18.0, CLOG_Z + 6.0)
            + centered_cube(
                format!("humidity_drain_fault_cartridge_key_tab_{i}"),
                40.0,
                10.0,
                12.0,
            )
            .translate(x, 46.0, CLOG_Z + 6.0);
    }
    caps
}

fn restrictor_land_pairs() -> Part {
    let mut lands = Part::empty("humidity_drain_fault_restrictor_land_pairs");
    for i in 0..CLOG_RESTRICTOR_COUNT {
        let pair = i / 2;
        let side = if i % 2 == 0 { -1.0 } else { 1.0 };
        lands = lands
            + centered_cube(
                format!("humidity_drain_fault_restrictor_screen_land_{i}"),
                42.0,
                12.0,
                8.0,
            )
            .translate(
                centered_index(pair, CLOG_CARTRIDGE_COUNT, 110.0),
                side * 58.0,
                CLOG_Z + 4.0,
            );
    }
    lands
}

fn bypass_witness_ports() -> Part {
    let mut ports = Part::empty("humidity_drain_fault_bypass_witness_ports");
    for i in 0..BYPASS_WITNESS_COUNT {
        ports = ports
            + centered_cylinder(
                format!("humidity_drain_fault_bypass_witness_port_ring_{i}"),
                15.0,
                6.0,
                32,
            )
            .translate(
                centered_index(i, BYPASS_WITNESS_COUNT, 128.0),
                -73.0,
                CLOG_Z + 3.0,
            );
    }
    ports
}

fn challenge_flow_arrows() -> Part {
    let mut arrows = Part::empty("humidity_drain_fault_challenge_flow_arrows");
    for i in 0..DRAIN_LANE_COUNT {
        arrows = arrows
            + centered_cube(
                format!("humidity_drain_fault_flow_arrow_stem_{i}"),
                6.0,
                28.0,
                4.0,
            )
            .translate(
                centered_index(i, DRAIN_LANE_COUNT, 86.0),
                -50.0,
                CLOG_Z + 2.0,
            )
            + centered_cube(
                format!("humidity_drain_fault_flow_arrow_head_{i}"),
                18.0,
                8.0,
                4.0,
            )
            .translate(
                centered_index(i, DRAIN_LANE_COUNT, 86.0),
                -68.0,
                CLOG_Z + 2.0,
            );
    }
    arrows
}

fn overflow_weir_witness_path() -> Part {
    let plate = centered_cube(
        "humidity_drain_fault_overflow_weir_witness_plate",
        WEIR_X,
        WEIR_Y,
        WEIR_Z,
    )
    .translate(0.0, 0.0, WEIR_Z / 2.0);
    plate - overflow_channels()
        + overflow_weir_teeth()
        + witness_window_lands()
        + absorbent_strip_lands()
}

fn overflow_channels() -> Part {
    let mut channels = Part::empty("humidity_drain_fault_overflow_channels");
    for i in 0..OVERFLOW_PATH_COUNT {
        channels = channels
            + centered_cube(
                format!("humidity_drain_fault_overflow_witness_channel_{i}"),
                WEIR_X - 90.0,
                18.0,
                14.0,
            )
            .translate(
                0.0,
                centered_index(i, OVERFLOW_PATH_COUNT, 32.0),
                WEIR_Z - 7.0,
            );
    }
    channels
}

fn overflow_weir_teeth() -> Part {
    let mut teeth = Part::empty("humidity_drain_fault_overflow_weir_teeth");
    for i in 0..OVERFLOW_WEIR_COUNT {
        teeth = teeth
            + centered_cube(
                format!("humidity_drain_fault_calibrated_overflow_weir_{i}"),
                24.0,
                96.0,
                30.0 + i as f64 * 4.0,
            )
            .translate(
                centered_index(i, OVERFLOW_WEIR_COUNT, 105.0),
                0.0,
                WEIR_Z + 15.0 + i as f64 * 2.0,
            );
    }
    teeth
}

fn witness_window_lands() -> Part {
    let mut lands = Part::empty("humidity_drain_fault_witness_window_lands");
    for i in 0..WITNESS_WINDOW_COUNT {
        let row = i / 4;
        let col = i % 4;
        lands = lands
            + centered_cube(
                format!("humidity_drain_fault_clear_witness_window_land_{i}"),
                54.0,
                16.0,
                5.0,
            )
            .translate(
                centered_index(col, 4, 104.0),
                -50.0 + row as f64 * 100.0,
                WEIR_Z + 2.5,
            );
    }
    lands
}

fn absorbent_strip_lands() -> Part {
    let mut strips = Part::empty("humidity_drain_fault_absorbent_strip_lands");
    for i in 0..ABSORBENT_STRIP_COUNT {
        strips = strips
            + centered_cube(
                format!("humidity_drain_fault_overflow_absorbent_strip_land_{i}"),
                62.0,
                12.0,
                4.0,
            )
            .translate(
                centered_index(i, ABSORBENT_STRIP_COUNT, 76.0),
                -WEIR_Y / 2.0 + 24.0,
                WEIR_Z + 2.0,
            );
    }
    strips
}

fn condensate_dye_tracer_wells() -> Part {
    let plate = centered_cube(
        "humidity_drain_fault_dye_tracer_well_plate",
        DYE_X,
        DYE_Y,
        DYE_Z,
    )
    .translate(0.0, 0.0, DYE_Z / 2.0);
    plate - dye_well_cuts() - dye_channel_cuts() + dye_well_rims() + dye_index_tabs()
}

fn dye_well_cuts() -> Part {
    let mut wells = Part::empty("humidity_drain_fault_dye_well_cuts");
    for row in 0..DYE_WELL_ROWS {
        for col in 0..DYE_WELL_COLS {
            let i = row * DYE_WELL_COLS + col;
            wells = wells
                + centered_cylinder(
                    format!("humidity_drain_fault_dye_tracer_well_cut_{i}"),
                    DYE_WELL_D / 2.0,
                    DYE_WELL_DEPTH + 1.0,
                    36,
                )
                .translate(
                    centered_index(col, DYE_WELL_COLS, 82.0),
                    centered_index(row, DYE_WELL_ROWS, 48.0),
                    DYE_Z - DYE_WELL_DEPTH / 2.0,
                );
        }
    }
    wells
}

fn dye_channel_cuts() -> Part {
    let mut channels = Part::empty("humidity_drain_fault_dye_channel_cuts");
    for i in 0..DYE_CHANNEL_COUNT {
        channels = channels
            + centered_cube(
                format!("humidity_drain_fault_dye_capillary_channel_cut_{i}"),
                DYE_X - 72.0,
                5.0,
                7.0,
            )
            .translate(0.0, centered_index(i, DYE_CHANNEL_COUNT, 24.0), DYE_Z - 4.0);
    }
    channels
}

fn dye_well_rims() -> Part {
    let mut rims = Part::empty("humidity_drain_fault_dye_well_rims");
    for row in 0..DYE_WELL_ROWS {
        for col in 0..DYE_WELL_COLS {
            let i = row * DYE_WELL_COLS + col;
            rims = rims
                + centered_cylinder(
                    format!("humidity_drain_fault_dye_tracer_well_rim_{i}"),
                    DYE_WELL_D / 2.0 + 5.0,
                    5.0,
                    36,
                )
                .translate(
                    centered_index(col, DYE_WELL_COLS, 82.0),
                    centered_index(row, DYE_WELL_ROWS, 48.0),
                    DYE_Z + 2.5,
                );
        }
    }
    rims
}

fn dye_index_tabs() -> Part {
    let mut tabs = Part::empty("humidity_drain_fault_dye_index_tabs");
    for i in 0..DYE_WELL_COLS {
        tabs = tabs
            + centered_cube(
                format!("humidity_drain_fault_dye_column_index_tab_{i}"),
                34.0,
                10.0,
                7.0,
            )
            .translate(
                centered_index(i, DYE_WELL_COLS, 82.0),
                DYE_Y / 2.0 - 18.0,
                DYE_Z + 3.5,
            );
    }
    tabs
}

fn microbial_biofilm_coupon_rack() -> Part {
    let rack = centered_cube(
        "humidity_drain_fault_microbial_biofilm_coupon_rack_body",
        COUPON_X,
        COUPON_Y,
        COUPON_Z,
    )
    .translate(0.0, 0.0, COUPON_Z / 2.0);
    rack - coupon_slot_cuts() + coupon_clip_rails() + swab_lane_ridges() + rack_id_lands()
}

fn coupon_slot_cuts() -> Part {
    let mut slots = Part::empty("humidity_drain_fault_coupon_slot_cuts");
    for rack in 0..COUPON_RACK_COUNT {
        for slot in 0..COUPONS_PER_RACK {
            let i = rack * COUPONS_PER_RACK + slot;
            slots = slots
                + centered_cube(
                    format!("humidity_drain_fault_coupon_slot_cut_{i}"),
                    COUPON_SLOT_X,
                    COUPON_SLOT_Y,
                    COUPON_SLOT_DEPTH + 0.5,
                )
                .translate(
                    centered_index(slot, COUPONS_PER_RACK, 70.0),
                    centered_index(rack, COUPON_RACK_COUNT, 42.0),
                    COUPON_Z - COUPON_SLOT_DEPTH / 2.0,
                );
        }
    }
    slots
}

fn coupon_clip_rails() -> Part {
    let mut rails = Part::empty("humidity_drain_fault_coupon_clip_rails");
    for rack in 0..COUPON_RACK_COUNT {
        let y = centered_index(rack, COUPON_RACK_COUNT, 42.0);
        rails = rails
            + centered_cube(
                format!("humidity_drain_fault_coupon_front_clip_rail_{rack}"),
                COUPON_X - 80.0,
                4.0,
                14.0,
            )
            .translate(0.0, y - COUPON_SLOT_Y / 2.0 - 6.0, COUPON_Z + 7.0)
            + centered_cube(
                format!("humidity_drain_fault_coupon_rear_clip_rail_{rack}"),
                COUPON_X - 80.0,
                4.0,
                14.0,
            )
            .translate(0.0, y + COUPON_SLOT_Y / 2.0 + 6.0, COUPON_Z + 7.0);
    }
    rails
}

fn swab_lane_ridges() -> Part {
    let mut ridges = Part::empty("humidity_drain_fault_swab_lane_ridges");
    for i in 0..SWAB_LANE_COUNT {
        ridges = ridges
            + centered_cube(
                format!("humidity_drain_fault_coupon_swab_lane_ridge_{i}"),
                38.0,
                COUPON_Y - 40.0,
                5.0,
            )
            .translate(
                centered_index(i, SWAB_LANE_COUNT, 62.0),
                0.0,
                COUPON_Z + 2.5,
            );
    }
    ridges
}

fn rack_id_lands() -> Part {
    let mut lands = Part::empty("humidity_drain_fault_coupon_rack_id_lands");
    for i in 0..COUPON_RACK_COUNT {
        lands = lands
            + centered_cube(
                format!("humidity_drain_fault_coupon_rack_id_land_{i}"),
                42.0,
                14.0,
                4.0,
            )
            .translate(
                -COUPON_X / 2.0 + 36.0,
                centered_index(i, COUPON_RACK_COUNT, 42.0),
                COUPON_Z + 2.0,
            );
    }
    lands
}

fn level_sensor_wetness_probe_pockets() -> Part {
    let block = centered_cube(
        "humidity_drain_fault_level_wetness_sensor_block",
        SENSOR_X,
        SENSOR_Y,
        SENSOR_Z,
    )
    .translate(0.0, 0.0, SENSOR_Z / 2.0);
    block - level_sensor_pocket_cuts() - wetness_probe_pocket_cuts() - cable_trough_cuts()
        + sensor_guard_posts()
        + wetness_comb_lands()
}

fn level_sensor_pocket_cuts() -> Part {
    let mut pockets = Part::empty("humidity_drain_fault_level_sensor_pocket_cuts");
    for i in 0..LEVEL_SENSOR_COUNT {
        pockets = pockets
            + centered_cube(
                format!("humidity_drain_fault_level_sensor_pocket_cut_{i}"),
                SENSOR_POCKET_X,
                SENSOR_POCKET_Y,
                SENSOR_POCKET_DEPTH + 0.5,
            )
            .translate(
                centered_index(i, LEVEL_SENSOR_COUNT, 76.0),
                42.0,
                SENSOR_Z - SENSOR_POCKET_DEPTH / 2.0,
            );
    }
    pockets
}

fn wetness_probe_pocket_cuts() -> Part {
    let mut probes = Part::empty("humidity_drain_fault_wetness_probe_pocket_cuts");
    for i in 0..WETNESS_PROBE_COUNT {
        probes = probes
            + centered_cylinder(
                format!("humidity_drain_fault_wetness_probe_socket_cut_{i}"),
                9.0,
                SENSOR_POCKET_DEPTH + 1.0,
                28,
            )
            .translate(
                centered_index(i, WETNESS_PROBE_COUNT, 64.0),
                -48.0,
                SENSOR_Z - SENSOR_POCKET_DEPTH / 2.0,
            );
    }
    probes
}

fn cable_trough_cuts() -> Part {
    let mut troughs = Part::empty("humidity_drain_fault_sensor_cable_trough_cuts");
    for i in 0..CABLE_TROUGH_COUNT {
        troughs = troughs
            + centered_cube(
                format!("humidity_drain_fault_sensor_cable_trough_cut_{i}"),
                SENSOR_X - 72.0,
                7.0,
                8.0,
            )
            .translate(
                0.0,
                centered_index(i, CABLE_TROUGH_COUNT, 32.0),
                SENSOR_Z - 6.0,
            );
    }
    troughs
}

fn sensor_guard_posts() -> Part {
    let mut posts = Part::empty("humidity_drain_fault_sensor_guard_posts");
    for i in 0..LEVEL_SENSOR_COUNT {
        let x = centered_index(i, LEVEL_SENSOR_COUNT, 76.0);
        posts = posts
            + centered_cylinder(
                format!("humidity_drain_fault_level_sensor_left_guard_{i}"),
                4.0,
                18.0,
                20,
            )
            .translate(x - SENSOR_POCKET_X / 2.0 - 7.0, 42.0, SENSOR_Z + 9.0)
            + centered_cylinder(
                format!("humidity_drain_fault_level_sensor_right_guard_{i}"),
                4.0,
                18.0,
                20,
            )
            .translate(x + SENSOR_POCKET_X / 2.0 + 7.0, 42.0, SENSOR_Z + 9.0);
    }
    posts
}

fn wetness_comb_lands() -> Part {
    let mut combs = Part::empty("humidity_drain_fault_wetness_comb_lands");
    for i in 0..WETNESS_PROBE_COUNT {
        combs = combs
            + centered_cube(
                format!("humidity_drain_fault_wetness_probe_comb_land_{i}"),
                22.0,
                8.0,
                6.0,
            )
            .translate(
                centered_index(i, WETNESS_PROBE_COUNT, 64.0),
                -80.0,
                SENSOR_Z + 3.0,
            );
    }
    combs
}

fn pressure_equalization_vent_checks() -> Part {
    let panel = centered_cube(
        "humidity_drain_fault_pressure_equalization_panel",
        VENT_X,
        VENT_Y,
        VENT_Z,
    )
    .translate(0.0, 0.0, VENT_Z / 2.0);
    panel - pressure_port_bores() - equalization_slot_cuts()
        + vent_check_masts()
        + vent_check_flap_lands()
        + vent_filter_retainer_lands()
}

fn pressure_port_bores() -> Part {
    let mut bores = Part::empty("humidity_drain_fault_pressure_port_bores");
    for i in 0..PRESSURE_PORT_COUNT {
        bores = bores
            + centered_cylinder(
                format!("humidity_drain_fault_pressure_port_bore_{i}"),
                VENT_BORE_D / 2.0,
                VENT_X + 4.0,
                32,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(
                0.0,
                centered_index(i, PRESSURE_PORT_COUNT, 102.0),
                VENT_Z - 24.0,
            );
    }
    bores
}

fn equalization_slot_cuts() -> Part {
    let mut slots = Part::empty("humidity_drain_fault_equalization_slot_cuts");
    for i in 0..EQUALIZATION_VENT_COUNT {
        slots = slots
            + centered_cube(
                format!("humidity_drain_fault_equalization_vent_slot_cut_{i}"),
                18.0,
                62.0,
                16.0,
            )
            .translate(
                0.0,
                centered_index(i, EQUALIZATION_VENT_COUNT, 102.0),
                VENT_Z - 10.0,
            );
    }
    slots
}

fn vent_check_masts() -> Part {
    let mut masts = Part::empty("humidity_drain_fault_vent_check_masts");
    for i in 0..CHECK_FEATURE_COUNT {
        masts = masts
            + centered_cylinder(
                format!("humidity_drain_fault_vent_check_mast_{i}"),
                18.0,
                VENT_MAST_Z,
                36,
            )
            .translate(
                0.0,
                centered_index(i, CHECK_FEATURE_COUNT, 102.0),
                VENT_Z + VENT_MAST_Z / 2.0,
            );
    }
    masts
}

fn vent_check_flap_lands() -> Part {
    let mut flaps = Part::empty("humidity_drain_fault_vent_check_flap_lands");
    for i in 0..CHECK_FEATURE_COUNT {
        flaps = flaps
            + centered_cube(
                format!("humidity_drain_fault_vent_check_flap_land_{i}"),
                76.0,
                18.0,
                7.0,
            )
            .translate(
                45.0,
                centered_index(i, CHECK_FEATURE_COUNT, 102.0),
                VENT_Z + 3.5,
            );
    }
    flaps
}

fn vent_filter_retainer_lands() -> Part {
    let mut lands = Part::empty("humidity_drain_fault_vent_filter_retainer_lands");
    for i in 0..EQUALIZATION_VENT_COUNT {
        lands = lands
            + centered_cube(
                format!("humidity_drain_fault_equalization_filter_retainer_{i}"),
                78.0,
                8.0,
                8.0,
            )
            .translate(
                -48.0,
                centered_index(i, EQUALIZATION_VENT_COUNT, 102.0),
                VENT_Z + 4.0,
            );
    }
    lands
}

fn release_hold_reject_lanes() -> Part {
    let base = centered_cube(
        "humidity_drain_fault_release_hold_reject_lane_base",
        LANE_X,
        LANE_Y,
        LANE_Z,
    )
    .translate(0.0, 0.0, LANE_Z / 2.0);
    base + disposition_lane_dividers() + disposition_tokens() + lane_stop_blocks()
}

fn disposition_lane_dividers() -> Part {
    let mut dividers = Part::empty("humidity_drain_fault_disposition_lane_dividers");
    for i in 0..=DISPOSITION_LANE_COUNT {
        dividers = dividers
            + centered_cube(
                format!("humidity_drain_fault_disposition_lane_divider_{i}"),
                5.0,
                LANE_Y,
                18.0,
            )
            .translate(
                -LANE_X / 2.0 + i as f64 * (LANE_X / DISPOSITION_LANE_COUNT as f64),
                0.0,
                LANE_Z + 9.0,
            );
    }
    dividers
}

fn disposition_tokens() -> Part {
    let mut tokens = Part::empty("humidity_drain_fault_disposition_tokens");
    for lane in DispositionLane::all() {
        for token in 0..TOKENS_PER_LANE {
            tokens = tokens
                + centered_cube(
                    format!(
                        "humidity_drain_fault_{}_disposition_token_{token}",
                        lane.name()
                    ),
                    28.0,
                    18.0,
                    8.0,
                )
                .translate(
                    -LANE_X / 3.0
                        + lane.index() as f64 * (LANE_X / 3.0)
                        + centered_index(token, TOKENS_PER_LANE, 34.0),
                    0.0,
                    LANE_Z + 4.0,
                );
        }
    }
    tokens
}

fn lane_stop_blocks() -> Part {
    let mut stops = Part::empty("humidity_drain_fault_lane_stop_blocks");
    for lane in DispositionLane::all() {
        stops = stops
            + centered_cube(
                format!("humidity_drain_fault_{}_lane_stop_block", lane.name()),
                42.0,
                12.0,
                16.0,
            )
            .translate(
                -LANE_X / 3.0 + lane.index() as f64 * (LANE_X / 3.0),
                -LANE_Y / 2.0 + 12.0,
                LANE_Z + 8.0,
            );
    }
    stops
}

fn alarm_event_token_rail() -> Part {
    let rail = centered_cube(
        "humidity_drain_fault_alarm_event_token_rail_base",
        ALARM_X,
        ALARM_Y,
        ALARM_Z,
    )
    .translate(0.0, 0.0, ALARM_Z / 2.0);
    rail + alarm_tokens() + alarm_detents() + alarm_index_land()
}

fn alarm_tokens() -> Part {
    let mut tokens = Part::empty("humidity_drain_fault_alarm_event_tokens");
    for i in 0..ALARM_EVENT_TOKEN_COUNT {
        tokens = tokens
            + centered_cube(
                format!("humidity_drain_fault_alarm_event_token_{i}"),
                24.0,
                24.0,
                10.0,
            )
            .translate(
                centered_index(i, ALARM_EVENT_TOKEN_COUNT, 38.0),
                8.0,
                ALARM_Z + 5.0,
            );
    }
    tokens
}

fn alarm_detents() -> Part {
    let mut detents = Part::empty("humidity_drain_fault_alarm_detents");
    for i in 0..ALARM_DETENT_COUNT {
        detents = detents
            + centered_cylinder(
                format!("humidity_drain_fault_alarm_token_detent_{i}"),
                7.0,
                5.0,
                24,
            )
            .translate(
                centered_index(i, ALARM_DETENT_COUNT, 38.0),
                -26.0,
                ALARM_Z + 2.5,
            );
    }
    detents
}

fn alarm_index_land() -> Part {
    centered_cube(
        "humidity_drain_fault_alarm_event_index_land",
        ALARM_X - 58.0,
        8.0,
        5.0,
    )
    .translate(0.0, ALARM_Y / 2.0 - 12.0, ALARM_Z + 2.5)
}

fn barcode_custody_lands() -> Part {
    let plate = centered_cube(
        "humidity_drain_fault_barcode_custody_land_plate",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    )
    .translate(0.0, 0.0, CUSTODY_Z / 2.0);
    plate + barcode_lands() + custody_seal_tabs() + run_record_lands()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("humidity_drain_fault_barcode_lands");
    for i in 0..BARCODE_LAND_COUNT {
        let x = centered_index(i, BARCODE_LAND_COUNT, 38.0);
        lands = lands
            + centered_cube(
                format!("humidity_drain_fault_barcode_land_{i}"),
                28.0,
                16.0,
                4.0,
            )
            .translate(x, 22.0, CUSTODY_Z + 2.0)
            + barcode_bars(i, x, 22.0);
    }
    lands
}

fn barcode_bars(index: usize, x: f64, y: f64) -> Part {
    let mut bars = Part::empty(format!("humidity_drain_fault_barcode_bars_{index}"));
    for bar in 0..5 {
        bars = bars
            + centered_cube(
                format!("humidity_drain_fault_barcode_{index}_bar_{bar}"),
                2.0 + (bar % 2) as f64,
                14.0,
                2.5,
            )
            .translate(x - 10.0 + bar as f64 * 5.0, y, CUSTODY_Z + 5.25);
    }
    bars
}

fn custody_seal_tabs() -> Part {
    let mut tabs = Part::empty("humidity_drain_fault_custody_seal_tabs");
    for i in 0..CUSTODY_SEAL_COUNT {
        tabs = tabs
            + centered_cube(
                format!("humidity_drain_fault_custody_seal_tab_{i}"),
                42.0,
                14.0,
                5.0,
            )
            .translate(
                centered_index(i, CUSTODY_SEAL_COUNT, 62.0),
                -18.0,
                CUSTODY_Z + 2.5,
            );
    }
    tabs
}

fn run_record_lands() -> Part {
    let mut lands = Part::empty("humidity_drain_fault_run_record_lands");
    for i in 0..RUN_RECORD_LAND_COUNT {
        lands = lands
            + centered_cube(
                format!("humidity_drain_fault_run_record_land_{i}"),
                62.0,
                10.0,
                4.0,
            )
            .translate(
                centered_index(i, RUN_RECORD_LAND_COUNT, 86.0),
                -CUSTODY_Y / 2.0 + 10.0,
                CUSTODY_Z + 2.0,
            );
    }
    lands
}

fn evidence_camera_bridge() -> Part {
    let beam = centered_cube(
        "humidity_drain_fault_evidence_camera_bridge_beam",
        EVIDENCE_X,
        EVIDENCE_Y,
        EVIDENCE_BEAM_Z,
    )
    .translate(0.0, 0.0, EVIDENCE_CLEARANCE_Z);
    let left_post = centered_cube(
        "humidity_drain_fault_evidence_bridge_left_post",
        34.0,
        EVIDENCE_Y,
        EVIDENCE_CLEARANCE_Z,
    )
    .translate(-EVIDENCE_X / 2.0 + 34.0, 0.0, EVIDENCE_CLEARANCE_Z / 2.0);
    let right_post = centered_cube(
        "humidity_drain_fault_evidence_bridge_right_post",
        34.0,
        EVIDENCE_Y,
        EVIDENCE_CLEARANCE_Z,
    )
    .translate(EVIDENCE_X / 2.0 - 34.0, 0.0, EVIDENCE_CLEARANCE_Z / 2.0);
    beam + left_post + right_post + evidence_camera_pads() + evidence_light_bars()
}

fn evidence_camera_pads() -> Part {
    let mut pads = Part::empty("humidity_drain_fault_evidence_camera_pads");
    for i in 0..EVIDENCE_CAMERA_COUNT {
        pads =
            pads + centered_cube(
                format!("humidity_drain_fault_evidence_camera_pad_{i}"),
                80.0,
                36.0,
                10.0,
            )
            .translate(
                centered_index(i, EVIDENCE_CAMERA_COUNT, 260.0),
                0.0,
                EVIDENCE_CLEARANCE_Z - EVIDENCE_BEAM_Z / 2.0 - 5.0,
            ) + centered_cylinder(
                format!("humidity_drain_fault_evidence_camera_bore_{i}"),
                6.0,
                12.0,
                24,
            )
            .translate(
                centered_index(i, EVIDENCE_CAMERA_COUNT, 260.0),
                0.0,
                EVIDENCE_CLEARANCE_Z - EVIDENCE_BEAM_Z / 2.0 - 5.0,
            );
    }
    pads
}

fn evidence_light_bars() -> Part {
    let mut bars = Part::empty("humidity_drain_fault_evidence_light_bars");
    for i in 0..EVIDENCE_LIGHT_BAR_COUNT {
        bars = bars
            + centered_cube(
                format!("humidity_drain_fault_evidence_light_bar_{i}"),
                EVIDENCE_X - 220.0,
                8.0,
                8.0,
            )
            .translate(
                0.0,
                if i == 0 { -18.0 } else { 18.0 },
                EVIDENCE_CLEARANCE_Z - EVIDENCE_BEAM_Z - 8.0,
            );
    }
    bars
}

fn robot_service_keepout_gauges() -> Part {
    let base_outline = centered_cube(
        "humidity_drain_fault_robot_service_keepout_outer_outline",
        KEEP_OUT_X,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(0.0, 0.0, 0.0);
    base_outline + keepout_gauge_bars() + vertical_clearance_gauges()
}

fn keepout_gauge_bars() -> Part {
    let front = centered_cube(
        "humidity_drain_fault_front_robot_keepout_gauge",
        KEEP_OUT_X,
        10.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, -KEEP_OUT_Y / 2.0, 0.0);
    let rear = centered_cube(
        "humidity_drain_fault_rear_service_keepout_gauge",
        KEEP_OUT_X,
        10.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, KEEP_OUT_Y / 2.0, 0.0);
    let left = centered_cube(
        "humidity_drain_fault_left_robot_sweep_keepout_gauge",
        10.0,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(-KEEP_OUT_X / 2.0, 0.0, 0.0);
    let right = centered_cube(
        "humidity_drain_fault_right_service_keepout_gauge",
        10.0,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(KEEP_OUT_X / 2.0, 0.0, 0.0);
    front + rear + left + right
}

fn vertical_clearance_gauges() -> Part {
    let mut gauges = Part::empty("humidity_drain_fault_vertical_clearance_gauges");
    for i in 0..KEEP_OUT_GAUGE_COUNT {
        gauges = gauges
            + centered_cube(
                format!("humidity_drain_fault_vertical_keepout_gauge_{i}"),
                18.0,
                18.0,
                80.0 + i as f64 * 20.0,
            )
            .translate(
                -KEEP_OUT_X / 2.0 + 70.0 + i as f64 * 54.0,
                -KEEP_OUT_Y / 2.0 + 58.0,
                40.0 + i as f64 * 10.0,
            );
    }
    gauges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn water_and_drain_path_counts_are_explicit() {
        assert_eq!(water_path_count(), 18);
        assert_eq!(drain_fault_path_count(), 15);
        assert_eq!(PAN_DRAIN_PORT_COUNT, 3);
        assert_eq!(DRAIN_TRAP_COUNT, 4);
        assert_eq!(OVERFLOW_PATH_COUNT, 4);
    }

    #[test]
    fn sensor_and_coupon_counts_match_fixture_intent() {
        assert_eq!(sensor_pocket_count(), 19);
        assert_eq!(LEVEL_SENSOR_COUNT, 5);
        assert_eq!(WETNESS_PROBE_COUNT, 6);
        assert_eq!(BIOFILM_COUPON_COUNT, 20);
        assert_eq!(DYE_TRACER_WELL_COUNT, 12);
    }

    #[test]
    fn output_manifest_is_stable_and_named() {
        assert_eq!(OUTPUTS.len(), 14);
        assert_eq!(OUTPUTS[0], "output/closed_incubator_humidity_condensate_drain_clog_overflow_fault_station_base_containment_deck.stl");
        assert_eq!(OUTPUTS[13], "output/closed_incubator_humidity_condensate_drain_clog_overflow_fault_station_assembly.stl");
        assert!(OUTPUTS.iter().all(|path| path.starts_with("output/")));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
        assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
    }

    #[test]
    fn station_bounds_and_clearances_hold() {
        assert_design_constraints();
        for item in socket_rects() {
            assert!(item.fits_inside_deck(), "{} outside deck", item.name);
        }
        assert!(containment_freeboard_ml() > 70_000.0);
        assert!(front_robot_clearance() >= ROBOT_FRONT_CLEARANCE_Y);
        assert!(rear_service_clearance() >= SERVICE_REAR_CLEARANCE_Y);
        assert!(vent_service_clearance() >= VENT_SERVICE_CLEARANCE_X);
    }

    #[test]
    fn feature_coverage_tracks_requested_fixture_elements() {
        for expected in [
            "base_containment_deck",
            "humidification_water_pan_surrogate",
            "drain_trap_clog_challenge_cartridge",
            "overflow_weir_witness_path",
            "condensate_dye_tracer_wells",
            "microbial_biofilm_coupon_rack",
            "level_sensor_wetness_probe_pockets",
            "pressure_equalization_vent_checks",
            "release_hold_reject_lanes",
            "alarm_event_token_rail",
            "barcode_custody_lands",
            "evidence_camera_bridge",
            "robot_service_keepout_gauges",
        ] {
            assert!(
                REQUIRED_FEATURES.contains(&expected),
                "missing feature {expected}"
            );
        }
        assert_eq!(REPRODUCIBILITY_CONTROLS[0], "fixed_output_manifest");
    }

    #[test]
    fn source_contains_no_biological_claim_terms() {
        let source = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/bin/closed_incubator_humidity_condensate_drain_clog_overflow_fault_station.rs"
        ))
        .to_lowercase();
        let forbidden_terms = [
            ["dia", "gnose"].concat(),
            ["dia", "gnosis"].concat(),
            ["ther", "apy"].concat(),
            ["thera", "peutic"].concat(),
            ["treat", "ment"].concat(),
            ["pat", "ient"].concat(),
            ["clin", "ical"].concat(),
            ["validated ", "biological"].concat(),
            ["validated ", "microbiological"].concat(),
            ["sterility ", "assurance"].concat(),
            ["path", "ogen"].concat(),
        ];
        for forbidden in forbidden_terms {
            assert!(
                !source.contains(&forbidden),
                "source contains prohibited claim term: {forbidden}"
            );
        }
    }
}
