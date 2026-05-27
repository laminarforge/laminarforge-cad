use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed incubator water-pan refill, overflow, and decon station.
//
// This standalone CAD generator models a no-cell validation fixture for an
// incubator humidity water pan. It reserves a closed refill quick-connect dock,
// water-pan surrogate, overflow gutter and drain trap, level-sensor pockets,
// biofilm/decon residue coupon carriers, splash baffles, conductivity/turbidity
// sample wells, barcode/custody lands, clean/dirty segregation, evidence
// imaging support, and robot/service keepout gauges. It is mechanical
// validation packaging only; decon chemistry, microbiology methods, action
// limits, and release criteria remain external validation controls.

const OUTPUT_PREFIX: &str = "closed_incubator_water_pan_refill_overflow_decon_station";

const OUTPUTS: [&str; 13] = [
    "output/closed_incubator_water_pan_refill_overflow_decon_station_base_containment_deck.stl",
    "output/closed_incubator_water_pan_refill_overflow_decon_station_water_pan_surrogate.stl",
    "output/closed_incubator_water_pan_refill_overflow_decon_station_fill_line_quick_connect_dock.stl",
    "output/closed_incubator_water_pan_refill_overflow_decon_station_overflow_gutter_drain_trap.stl",
    "output/closed_incubator_water_pan_refill_overflow_decon_station_level_sensor_pockets.stl",
    "output/closed_incubator_water_pan_refill_overflow_decon_station_biofilm_decon_residue_coupon_carriers.stl",
    "output/closed_incubator_water_pan_refill_overflow_decon_station_splash_baffle_array.stl",
    "output/closed_incubator_water_pan_refill_overflow_decon_station_conductivity_turbidity_sample_wells.stl",
    "output/closed_incubator_water_pan_refill_overflow_decon_station_barcode_custody_lands.stl",
    "output/closed_incubator_water_pan_refill_overflow_decon_station_clean_dirty_segregation_bulkhead.stl",
    "output/closed_incubator_water_pan_refill_overflow_decon_station_evidence_bridge.stl",
    "output/closed_incubator_water_pan_refill_overflow_decon_station_robot_service_keepout_gauges.stl",
    "output/closed_incubator_water_pan_refill_overflow_decon_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 12] = [
    "water_pan_surrogate",
    "fill_line_quick_connect_dock",
    "overflow_gutter_drain_trap",
    "level_sensor_pockets",
    "biofilm_decon_residue_coupon_carriers",
    "splash_baffles",
    "conductivity_turbidity_sample_wells",
    "barcode_custody_lands",
    "clean_dirty_segregation",
    "evidence_bridge",
    "robot_service_keepout_gauges",
    "named_stl_outputs",
];

const STATION_X: f64 = 1550.0;
const STATION_Y: f64 = 920.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 48.0;
const BASIN_DEPTH: f64 = 9.0;
const SOCKET_DEPTH: f64 = 6.0;
const MOUNT_HOLE_COUNT: usize = 8;
const DATUM_TARGET_COUNT: usize = 4;

const PAN_X: f64 = 610.0;
const PAN_Y: f64 = 330.0;
const PAN_Z: f64 = 64.0;
const PAN_POS: (f64, f64) = (-390.0, 70.0);
const PAN_INNER_X: f64 = 500.0;
const PAN_INNER_Y: f64 = 220.0;
const PAN_INNER_DEPTH: f64 = 42.0;
const PAN_WORKING_DEPTH: f64 = 28.0;
const PAN_FILL_LEVEL_MARKS: usize = 4;
const PAN_DRAIN_PORT_COUNT: usize = 2;
const PAN_ROBOT_HANDLE_COUNT: usize = 4;

const FILL_DOCK_X: f64 = 430.0;
const FILL_DOCK_Y: f64 = 130.0;
const FILL_DOCK_Z: f64 = 70.0;
const FILL_DOCK_POS: (f64, f64) = (-420.0, 325.0);
const QUICK_CONNECT_COUNT: usize = 4;
const QUICK_CONNECT_D: f64 = 24.0;
const CAP_PARK_COUNT: usize = 4;
const DOCK_LATCH_COUNT: usize = 4;
const DRIP_TROUGH_COUNT: usize = 2;

const GUTTER_X: f64 = 610.0;
const GUTTER_Y: f64 = 170.0;
const GUTTER_Z: f64 = 54.0;
const GUTTER_POS: (f64, f64) = (-390.0, -250.0);
const GUTTER_LANE_COUNT: usize = 3;
const GUTTER_CHANNEL_COUNT: usize = 6;
const DRAIN_TRAP_COUNT: usize = 2;
const DRAIN_TRAP_D: f64 = 32.0;
const OVERFLOW_CUP_COUNT: usize = 3;

const LEVEL_X: f64 = 390.0;
const LEVEL_Y: f64 = 180.0;
const LEVEL_Z: f64 = 50.0;
const LEVEL_POS: (f64, f64) = (330.0, 290.0);
const LEVEL_SENSOR_COUNT: usize = 4;
const FLOAT_GAUGE_COUNT: usize = 3;
const LEVEL_SENSOR_POCKET_X: f64 = 58.0;
const LEVEL_SENSOR_POCKET_Y: f64 = 38.0;
const LEVEL_SENSOR_DEPTH: f64 = 24.0;

const COUPON_X: f64 = 430.0;
const COUPON_Y: f64 = 230.0;
const COUPON_Z: f64 = 50.0;
const COUPON_POS: (f64, f64) = (330.0, 40.0);
const COUPON_RACK_COUNT: usize = 4;
const COUPONS_PER_RACK: usize = 6;
const COUPON_SLOT_COUNT: usize = COUPON_RACK_COUNT * COUPONS_PER_RACK;
const COUPON_SLOT_X: f64 = 44.0;
const COUPON_SLOT_Y: f64 = 20.0;
const COUPON_SLOT_DEPTH: f64 = 18.0;
const DECON_SHADOW_MASK_COUNT: usize = 8;

const BAFFLE_X: f64 = 560.0;
const BAFFLE_Y: f64 = 260.0;
const BAFFLE_Z: f64 = 74.0;
const SPLASH_BAFFLE_COUNT: usize = 7;
const BAFFLE_DRAIN_SLOT_COUNT: usize = 14;
const BAFFLE_LOCK_TAB_COUNT: usize = 4;

const SAMPLE_X: f64 = 430.0;
const SAMPLE_Y: f64 = 170.0;
const SAMPLE_Z: f64 = 44.0;
const SAMPLE_POS: (f64, f64) = (330.0, -210.0);
const SAMPLE_WELL_COUNT: usize = 6;
const SAMPLE_WELL_D: f64 = 38.0;
const SAMPLE_WELL_DEPTH: f64 = 26.0;
const CONDUCTIVITY_PROBE_COUNT: usize = 4;
const TURBIDITY_WINDOW_COUNT: usize = 4;

const CUSTODY_X: f64 = 500.0;
const CUSTODY_Y: f64 = 80.0;
const CUSTODY_Z: f64 = 14.0;
const CUSTODY_POS: (f64, f64) = (330.0, -365.0);
const BARCODE_LAND_COUNT: usize = 12;
const CUSTODY_SEAL_COUNT: usize = 6;
const RUN_RECORD_LAND_COUNT: usize = 4;

const SEGREGATION_X: f64 = 24.0;
const SEGREGATION_Y: f64 = 760.0;
const SEGREGATION_Z: f64 = 76.0;
const SEGREGATION_POS: (f64, f64) = (0.0, 0.0);
const SEGREGATION_PASS_GATE_COUNT: usize = 5;
const DIRTY_DRIP_RIB_COUNT: usize = 6;

const EVIDENCE_X: f64 = 1320.0;
const EVIDENCE_Y: f64 = 50.0;
const EVIDENCE_BEAM_Z: f64 = 28.0;
const EVIDENCE_CLEARANCE_Z: f64 = 205.0;
const EVIDENCE_POS: (f64, f64) = (0.0, 415.0);
const EVIDENCE_CAMERA_COUNT: usize = 5;
const EVIDENCE_LIGHT_BAR_COUNT: usize = 2;

const KEEP_OUT_X: f64 = 1470.0;
const KEEP_OUT_Y: f64 = 860.0;
const KEEP_OUT_Z: f64 = 6.0;
const ROBOT_FRONT_CLEARANCE_Y: f64 = 290.0;
const SERVICE_REAR_CLEARANCE_Y: f64 = 220.0;
const SIDE_FILL_SERVICE_X: f64 = 245.0;
const PAN_LIFT_CLEARANCE_Z: f64 = 320.0;
const KEEP_OUT_GAUGE_COUNT: usize = 6;

const LABEL_Z: f64 = 2.4;

#[derive(Clone, Copy, Debug)]
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

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_containment_deck();
    export(OUTPUTS[0], &base);

    let pan = water_pan_surrogate();
    export(OUTPUTS[1], &pan);

    let fill = fill_line_quick_connect_dock();
    export(OUTPUTS[2], &fill);

    let gutter = overflow_gutter_drain_trap();
    export(OUTPUTS[3], &gutter);

    let level = level_sensor_pockets();
    export(OUTPUTS[4], &level);

    let coupons = biofilm_decon_residue_coupon_carriers();
    export(OUTPUTS[5], &coupons);

    let baffles = splash_baffle_array();
    export(OUTPUTS[6], &baffles);

    let samples = conductivity_turbidity_sample_wells();
    export(OUTPUTS[7], &samples);

    let custody = barcode_custody_lands();
    export(OUTPUTS[8], &custody);

    let segregation = clean_dirty_segregation_bulkhead();
    export(OUTPUTS[9], &segregation);

    let evidence = evidence_bridge();
    export(OUTPUTS[10], &evidence);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[11], &keepouts);

    let assembly = base
        + pan.translate(PAN_POS.0, PAN_POS.1, insert_z(PAN_Z))
        + fill.translate(FILL_DOCK_POS.0, FILL_DOCK_POS.1, insert_z(FILL_DOCK_Z))
        + gutter.translate(GUTTER_POS.0, GUTTER_POS.1, insert_z(GUTTER_Z))
        + level.translate(LEVEL_POS.0, LEVEL_POS.1, insert_z(LEVEL_Z))
        + coupons.translate(COUPON_POS.0, COUPON_POS.1, insert_z(COUPON_Z))
        + baffles.translate(
            PAN_POS.0,
            PAN_POS.1,
            insert_z(PAN_Z) + PAN_Z / 2.0 + BAFFLE_Z / 2.0 - 28.0,
        )
        + samples.translate(SAMPLE_POS.0, SAMPLE_POS.1, insert_z(SAMPLE_Z))
        + custody.translate(CUSTODY_POS.0, CUSTODY_POS.1, insert_z(CUSTODY_Z))
        + segregation.translate(
            SEGREGATION_POS.0,
            SEGREGATION_POS.1,
            insert_z(SEGREGATION_Z),
        )
        + evidence.translate(EVIDENCE_POS.0, EVIDENCE_POS.1, DECK_Z)
        + keepouts.translate(0.0, 0.0, DECK_Z);
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed incubator water-pan refill/overflow/decon station:");
    println!(
        "  Footprint:        {STATION_X:.0}mm x {STATION_Y:.0}mm containment deck with {:.0}mL freeboard",
        containment_freeboard_ml()
    );
    println!(
        "  Water pan:        {PAN_X:.0}mm x {PAN_Y:.0}mm surrogate, {:.0}mL working volume, {PAN_FILL_LEVEL_MARKS} fill marks, {SPLASH_BAFFLE_COUNT} splash baffles",
        water_pan_working_volume_ml()
    );
    println!(
        "  Refill/overflow:  {QUICK_CONNECT_COUNT} keyed quick connects, {GUTTER_LANE_COUNT} overflow lanes, {DRAIN_TRAP_COUNT} drain traps, {OVERFLOW_CUP_COUNT} evidence cups"
    );
    println!(
        "  Sensors/coupons:  {LEVEL_SENSOR_COUNT} level sensor pockets, {FLOAT_GAUGE_COUNT} float gauges, {COUPON_SLOT_COUNT} coupon slots, {SAMPLE_WELL_COUNT} sample wells"
    );
    println!(
        "  Trace/evidence:   {BARCODE_LAND_COUNT} barcode lands, {CUSTODY_SEAL_COUNT} custody seals, {EVIDENCE_CAMERA_COUNT} cameras, {KEEP_OUT_GAUGE_COUNT} keepout gauges"
    );
    println!("  Required features: {}", REQUIRED_FEATURES.len());
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
    assert_eq!(OUTPUTS.len(), 13);
    assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
    assert_eq!(REQUIRED_FEATURES.len(), 12);
    assert_eq!(PAN_ROBOT_HANDLE_COUNT, 4);
    assert_eq!(PAN_DRAIN_PORT_COUNT, 2);
    assert_eq!(QUICK_CONNECT_COUNT, CAP_PARK_COUNT);
    assert_eq!(COUPON_SLOT_COUNT, COUPON_RACK_COUNT * COUPONS_PER_RACK);
    assert_eq!(BAFFLE_DRAIN_SLOT_COUNT, SPLASH_BAFFLE_COUNT * 2);
    assert_eq!(BAFFLE_LOCK_TAB_COUNT, 4);
    assert_eq!(
        BARCODE_LAND_COUNT,
        SAMPLE_WELL_COUNT + LEVEL_SENSOR_COUNT + DRAIN_TRAP_COUNT
    );
    assert_eq!(MOUNT_HOLE_COUNT, mount_hole_positions().len());
    assert_eq!(DATUM_TARGET_COUNT, datum_positions().len());
    assert!(PAN_WORKING_DEPTH < PAN_INNER_DEPTH);
    assert!(water_pan_working_volume_ml() > refill_charge_volume_ml());
    assert!(containment_freeboard_ml() > worst_case_overflow_volume_ml());
    assert!(EVIDENCE_CLEARANCE_Z > PAN_Z + BAFFLE_Z);
    assert!(PAN_LIFT_CLEARANCE_Z > EVIDENCE_CLEARANCE_Z);
    assert!(front_robot_clearance() >= ROBOT_FRONT_CLEARANCE_Y);
    assert!(rear_service_clearance() >= SERVICE_REAR_CLEARANCE_Y);
    assert!(side_fill_service_clearance() >= SIDE_FILL_SERVICE_X);

    for item in deck_socket_rects() {
        assert!(item.fits_inside_deck(), "{} exceeds deck", item.name);
    }

    let rects = deck_socket_rects();
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

fn deck_socket_rects() -> [Rect; 8] {
    [
        rect("water_pan_surrogate", PAN_POS, PAN_X, PAN_Y),
        rect(
            "fill_line_quick_connect_dock",
            FILL_DOCK_POS,
            FILL_DOCK_X,
            FILL_DOCK_Y,
        ),
        rect("overflow_gutter_drain_trap", GUTTER_POS, GUTTER_X, GUTTER_Y),
        rect("level_sensor_pockets", LEVEL_POS, LEVEL_X, LEVEL_Y),
        rect(
            "biofilm_decon_residue_coupon_carriers",
            COUPON_POS,
            COUPON_X,
            COUPON_Y,
        ),
        rect(
            "conductivity_turbidity_sample_wells",
            SAMPLE_POS,
            SAMPLE_X,
            SAMPLE_Y,
        ),
        rect("barcode_custody_lands", CUSTODY_POS, CUSTODY_X, CUSTODY_Y),
        rect(
            "clean_dirty_segregation_bulkhead",
            SEGREGATION_POS,
            SEGREGATION_X,
            SEGREGATION_Y,
        ),
    ]
}

fn front_robot_clearance() -> f64 {
    ROBOT_FRONT_CLEARANCE_Y
}

fn rear_service_clearance() -> f64 {
    STATION_Y / 2.0 - (FILL_DOCK_POS.1 + FILL_DOCK_Y / 2.0) + EVIDENCE_CLEARANCE_Z
}

fn side_fill_service_clearance() -> f64 {
    STATION_X / 2.0 - (PAN_POS.0.abs() + PAN_X / 2.0) + 165.0
}

fn water_pan_working_volume_ml() -> f64 {
    PAN_INNER_X * PAN_INNER_Y * PAN_WORKING_DEPTH / 1000.0
}

fn refill_charge_volume_ml() -> f64 {
    QUICK_CONNECT_COUNT as f64 * 180.0 + SAMPLE_WELL_COUNT as f64 * 12.0
}

fn worst_case_overflow_volume_ml() -> f64 {
    water_pan_working_volume_ml() * 0.42
        + OVERFLOW_CUP_COUNT as f64 * 90.0
        + DRAIN_TRAP_COUNT as f64 * 65.0
}

fn containment_freeboard_ml() -> f64 {
    let inner_x = STATION_X - 2.0 * RIM_W;
    let inner_y = STATION_Y - 2.0 * RIM_W;
    let freeboard_z = RIM_Z - BASIN_DEPTH;
    inner_x * inner_y * freeboard_z / 1000.0
}

fn base_containment_deck() -> Part {
    let deck = centered_cube(
        "water_pan_refill_base_containment_deck",
        STATION_X,
        STATION_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let basin = centered_cube(
        "water_pan_refill_secondary_containment_basin_cut",
        STATION_X - 2.0 * (RIM_W + 52.0),
        STATION_Y - 2.0 * (RIM_W + 46.0),
        BASIN_DEPTH + 0.8,
    )
    .translate(0.0, -10.0, DECK_Z - BASIN_DEPTH / 2.0);
    let front_drain = centered_cylinder(
        "water_pan_refill_base_front_closed_drain_placeholder",
        9.0,
        58.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 94.0,
        -STATION_Y / 2.0 + 32.0,
        DECK_Z - 8.0,
    );

    deck - basin - front_drain - insert_sockets() - deck_mount_holes()
        + perimeter_rims()
        + clean_dirty_lane_spines()
        + deck_datum_targets()
        + evidence_anchor_lands()
        + flow_witness_ribs()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("water_pan_refill_insert_sockets");
    for item in deck_socket_rects() {
        sockets = sockets
            + centered_cube(
                format!("water_pan_refill_{}_socket", item.name),
                item.x + 10.0,
                item.y + 10.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(item.center.0, item.center.1, DECK_Z - SOCKET_DEPTH / 2.0);
    }
    sockets
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("water_pan_refill_deck_mount_holes");
    for (i, (x, y)) in mount_hole_positions().into_iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("water_pan_refill_m6_mount_clearance_{i}"),
                3.4,
                DECK_Z + 4.0,
                28,
            )
            .translate(x, y, DECK_Z / 2.0)
            + centered_cube(
                format!("water_pan_refill_service_slot_{i}"),
                30.0,
                7.5,
                DECK_Z + 4.0,
            )
            .translate(x, y, DECK_Z / 2.0);
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
        "water_pan_refill_front_low_robot_rim",
        STATION_X,
        RIM_W,
        30.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, DECK_Z + 15.0);
    let rear = centered_cube(
        "water_pan_refill_rear_service_high_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let left = centered_cube("water_pan_refill_left_spill_rim", RIM_W, STATION_Y, RIM_Z).translate(
        -STATION_X / 2.0 + RIM_W / 2.0,
        0.0,
        DECK_Z + RIM_Z / 2.0,
    );
    let right = centered_cube("water_pan_refill_right_spill_rim", RIM_W, STATION_Y, RIM_Z)
        .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);

    front + rear + left + right
}

fn clean_dirty_lane_spines() -> Part {
    let clean_lane = centered_cube(
        "water_pan_refill_clean_fill_lane_spine",
        10.0,
        STATION_Y - 172.0,
        24.0,
    )
    .translate(SEGREGATION_POS.0 - 42.0, 0.0, DECK_Z + 12.0);
    let dirty_lane = centered_cube(
        "water_pan_refill_dirty_overflow_lane_spine",
        10.0,
        STATION_Y - 172.0,
        24.0,
    )
    .translate(SEGREGATION_POS.0 + 42.0, 0.0, DECK_Z + 12.0);
    let refill_flow = centered_cube(
        "water_pan_refill_refill_to_pan_direction_spine",
        STATION_X - 250.0,
        8.0,
        22.0,
    )
    .translate(0.0, 240.0, DECK_Z + 11.0);
    let overflow_flow = centered_cube(
        "water_pan_refill_pan_to_drain_direction_spine",
        STATION_X - 260.0,
        8.0,
        22.0,
    )
    .translate(0.0, -138.0, DECK_Z + 11.0);

    clean_lane + dirty_lane + refill_flow + overflow_flow
}

fn deck_datum_targets() -> Part {
    let mut targets = Part::empty("water_pan_refill_robot_datum_targets");
    for (i, (x, y)) in datum_positions().into_iter().enumerate() {
        targets = targets
            + fiducial_disc(&format!("water_pan_refill_deck_datum_target_{i}")).translate(
                x,
                y,
                DECK_Z + 2.5,
            );
    }
    targets
}

fn datum_positions() -> [(f64, f64); DATUM_TARGET_COUNT] {
    [
        (-STATION_X / 2.0 + 102.0, -STATION_Y / 2.0 + 96.0),
        (STATION_X / 2.0 - 102.0, -STATION_Y / 2.0 + 96.0),
        (-STATION_X / 2.0 + 102.0, STATION_Y / 2.0 - 96.0),
        (STATION_X / 2.0 - 102.0, STATION_Y / 2.0 - 96.0),
    ]
}

fn evidence_anchor_lands() -> Part {
    let left = centered_cube(
        "water_pan_refill_evidence_bridge_left_anchor_land",
        98.0,
        40.0,
        8.0,
    )
    .translate(
        EVIDENCE_POS.0 - EVIDENCE_X / 2.0 + 64.0,
        EVIDENCE_POS.1,
        DECK_Z + 4.0,
    );
    let right = centered_cube(
        "water_pan_refill_evidence_bridge_right_anchor_land",
        98.0,
        40.0,
        8.0,
    )
    .translate(
        EVIDENCE_POS.0 + EVIDENCE_X / 2.0 - 64.0,
        EVIDENCE_POS.1,
        DECK_Z + 4.0,
    );
    left + right
}

fn flow_witness_ribs() -> Part {
    let mut ribs = Part::empty("water_pan_refill_base_flow_witness_ribs");
    for (i, y) in [-340.0, -290.0, -205.0, -70.0, 120.0, 250.0, 360.0]
        .into_iter()
        .enumerate()
    {
        ribs = ribs
            + centered_cube(
                format!("water_pan_refill_base_flow_witness_rib_{i}"),
                STATION_X - 250.0,
                4.0,
                5.0,
            )
            .translate(0.0, y, DECK_Z + 2.5);
    }
    ribs
}

fn water_pan_surrogate() -> Part {
    let body = centered_cube("water_pan_refill_surrogate_outer_pan", PAN_X, PAN_Y, PAN_Z);
    let basin = centered_cube(
        "water_pan_refill_surrogate_water_basin_cut",
        PAN_INNER_X,
        PAN_INNER_Y,
        PAN_INNER_DEPTH,
    )
    .translate(0.0, 0.0, PAN_Z / 2.0 - PAN_INNER_DEPTH / 2.0 + 3.0);
    let overflow_notch = centered_cube(
        "water_pan_refill_surrogate_high_level_overflow_notch",
        118.0,
        26.0,
        24.0,
    )
    .translate(PAN_X / 2.0 - 34.0, -PAN_Y / 2.0 + 52.0, PAN_Z / 2.0 - 12.0);

    body - basin - overflow_notch - pan_drain_port_cuts()
        + pan_working_level_steps()
        + pan_fill_level_marks()
        + pan_robot_handles()
        + pan_corner_fiducials()
}

fn pan_drain_port_cuts() -> Part {
    let mut cuts = Part::empty("water_pan_refill_pan_drain_port_cuts");
    for port in 0..PAN_DRAIN_PORT_COUNT {
        cuts = cuts
            + centered_cylinder(
                format!("water_pan_refill_pan_drain_port_cut_{port}"),
                7.0,
                52.0,
                28,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(
                PAN_X / 2.0 - 18.0,
                centered_index(port, PAN_DRAIN_PORT_COUNT, 54.0) - 88.0,
                PAN_Z / 2.0 - 18.0,
            );
    }
    cuts
}

fn pan_working_level_steps() -> Part {
    let mut steps = Part::empty("water_pan_refill_working_level_step_terraces");
    for step in 0..4 {
        steps = steps
            + centered_cube(
                format!("water_pan_refill_pan_slope_step_{step}"),
                PAN_INNER_X - 60.0 - step as f64 * 54.0,
                10.0,
                4.0,
            )
            .translate(
                0.0,
                -PAN_INNER_Y / 2.0 + 38.0 + step as f64 * 46.0,
                PAN_Z / 2.0 - PAN_INNER_DEPTH + 5.0 + step as f64 * 2.2,
            );
    }
    steps
}

fn pan_fill_level_marks() -> Part {
    let mut marks = Part::empty("water_pan_refill_fill_level_marks");
    for mark in 0..PAN_FILL_LEVEL_MARKS {
        marks = marks
            + centered_cube(
                format!("water_pan_refill_pan_fill_level_mark_{mark}"),
                58.0 - mark as f64 * 6.0,
                4.0,
                4.0,
            )
            .translate(
                -PAN_X / 2.0 + 54.0,
                -PAN_INNER_Y / 2.0 + 54.0 + mark as f64 * 42.0,
                PAN_Z / 2.0 + 2.0,
            );
    }
    marks
}

fn pan_robot_handles() -> Part {
    let mut handles = Part::empty("water_pan_refill_pan_robot_handles");
    for (i, (x, y)) in [
        (-PAN_X / 2.0 + 34.0, -PAN_Y / 2.0 + 40.0),
        (PAN_X / 2.0 - 34.0, -PAN_Y / 2.0 + 40.0),
        (-PAN_X / 2.0 + 34.0, PAN_Y / 2.0 - 40.0),
        (PAN_X / 2.0 - 34.0, PAN_Y / 2.0 - 40.0),
    ]
    .into_iter()
    .enumerate()
    {
        handles = handles
            + centered_cube(
                format!("water_pan_refill_pan_robot_lift_handle_{i}"),
                28.0,
                74.0,
                24.0,
            )
            .translate(x, y, PAN_Z / 2.0 + 12.0);
    }
    handles
}

fn pan_corner_fiducials() -> Part {
    let mut fiducials = Part::empty("water_pan_refill_pan_corner_fiducials");
    for (i, (x, y)) in [
        (-PAN_X / 2.0 + 56.0, -PAN_Y / 2.0 + 56.0),
        (PAN_X / 2.0 - 56.0, -PAN_Y / 2.0 + 56.0),
        (-PAN_X / 2.0 + 56.0, PAN_Y / 2.0 - 56.0),
        (PAN_X / 2.0 - 56.0, PAN_Y / 2.0 - 56.0),
    ]
    .into_iter()
    .enumerate()
    {
        fiducials = fiducials
            + fiducial_disc(&format!("water_pan_refill_pan_fiducial_{i}")).translate(
                x,
                y,
                PAN_Z / 2.0 + 2.5,
            );
    }
    fiducials
}

fn fill_line_quick_connect_dock() -> Part {
    let dock = centered_cube(
        "water_pan_refill_quick_connect_dock_body",
        FILL_DOCK_X,
        FILL_DOCK_Y,
        FILL_DOCK_Z,
    );
    let drip_basin = centered_cube(
        "water_pan_refill_quick_connect_drip_basin_cut",
        FILL_DOCK_X - 58.0,
        FILL_DOCK_Y - 46.0,
        12.0,
    )
    .translate(0.0, -4.0, FILL_DOCK_Z / 2.0 - 6.0);

    dock - drip_basin - quick_connect_socket_cuts() - cap_parking_pocket_cuts()
        + quick_connect_bosses()
        + dock_latch_fingers()
        + drip_troughs()
        + code_bar_label("water_pan_refill_fill_dock_csg_label", 160.0, 18.0, 7).translate(
            0.0,
            FILL_DOCK_Y / 2.0 - 20.0,
            FILL_DOCK_Z / 2.0 + LABEL_Z / 2.0,
        )
}

fn quick_connect_socket_cuts() -> Part {
    let mut cuts = Part::empty("water_pan_refill_quick_connect_socket_cuts");
    for port in 0..QUICK_CONNECT_COUNT {
        let x = centered_index(port, QUICK_CONNECT_COUNT, 76.0);
        cuts = cuts
            + centered_cylinder(
                format!("water_pan_refill_quick_connect_socket_cut_{port}"),
                QUICK_CONNECT_D / 2.0,
                42.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -FILL_DOCK_Y / 2.0 + 24.0, FILL_DOCK_Z / 2.0 - 20.0)
            + centered_cube(
                format!("water_pan_refill_quick_connect_key_flat_cut_{port}"),
                QUICK_CONNECT_D * 0.85,
                22.0,
                7.0,
            )
            .translate(x, -FILL_DOCK_Y / 2.0 + 42.0, FILL_DOCK_Z / 2.0 - 4.0);
    }
    cuts
}

fn cap_parking_pocket_cuts() -> Part {
    let mut cuts = Part::empty("water_pan_refill_cap_parking_pocket_cuts");
    for cap in 0..CAP_PARK_COUNT {
        cuts = cuts
            + centered_cylinder(
                format!("water_pan_refill_sterile_cap_parking_pocket_{cap}"),
                10.0,
                16.0,
                28,
            )
            .translate(
                centered_index(cap, CAP_PARK_COUNT, 76.0),
                FILL_DOCK_Y / 2.0 - 44.0,
                FILL_DOCK_Z / 2.0 - 7.0,
            );
    }
    cuts
}

fn quick_connect_bosses() -> Part {
    let mut bosses = Part::empty("water_pan_refill_quick_connect_bosses");
    for port in 0..QUICK_CONNECT_COUNT {
        bosses = bosses
            + centered_cylinder(
                format!("water_pan_refill_quick_connect_boss_{port}"),
                18.0,
                8.0,
                36,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(port, QUICK_CONNECT_COUNT, 76.0),
                -FILL_DOCK_Y / 2.0 + 18.0,
                FILL_DOCK_Z / 2.0 - 20.0,
            );
    }
    bosses
}

fn dock_latch_fingers() -> Part {
    let mut latches = Part::empty("water_pan_refill_quick_connect_latch_fingers");
    for latch in 0..DOCK_LATCH_COUNT {
        let x = centered_index(latch, DOCK_LATCH_COUNT, 76.0);
        latches = latches
            + centered_cube(
                format!("water_pan_refill_quick_connect_latch_finger_{latch}"),
                7.0,
                42.0,
                18.0,
            )
            .translate(x - 22.0, -10.0, FILL_DOCK_Z / 2.0 + 9.0)
            + centered_cube(
                format!("water_pan_refill_quick_connect_latch_finger_{latch}_right"),
                7.0,
                42.0,
                18.0,
            )
            .translate(x + 22.0, -10.0, FILL_DOCK_Z / 2.0 + 9.0);
    }
    latches
}

fn drip_troughs() -> Part {
    let mut troughs = Part::empty("water_pan_refill_quick_connect_drip_troughs");
    for trough in 0..DRIP_TROUGH_COUNT {
        troughs = troughs
            + centered_cube(
                format!("water_pan_refill_fill_dock_drip_trough_rib_{trough}"),
                FILL_DOCK_X - 86.0,
                5.0,
                8.0,
            )
            .translate(
                0.0,
                -FILL_DOCK_Y / 2.0 + 56.0 + trough as f64 * 34.0,
                FILL_DOCK_Z / 2.0 + 4.0,
            );
    }
    troughs
}

fn overflow_gutter_drain_trap() -> Part {
    let body = centered_cube(
        "water_pan_refill_overflow_gutter_drain_trap_body",
        GUTTER_X,
        GUTTER_Y,
        GUTTER_Z,
    );
    let basin = centered_cube(
        "water_pan_refill_overflow_gutter_basin_cut",
        GUTTER_X - 56.0,
        GUTTER_Y - 46.0,
        14.0,
    )
    .translate(0.0, 0.0, GUTTER_Z / 2.0 - 7.0);

    body - basin - gutter_channel_cuts() - overflow_cup_cuts()
        + gutter_diverter_vanes()
        + drain_trap_witness_geometry()
        + gutter_lane_labels()
}

fn gutter_channel_cuts() -> Part {
    let mut cuts = Part::empty("water_pan_refill_gutter_channel_cuts");
    for channel in 0..GUTTER_CHANNEL_COUNT {
        let lane = channel % GUTTER_LANE_COUNT;
        let row = channel / GUTTER_LANE_COUNT;
        cuts = cuts
            + centered_cube(
                format!("water_pan_refill_overflow_channel_cut_{channel}"),
                GUTTER_X / 3.0 - 44.0,
                15.0,
                16.0,
            )
            .translate(
                centered_index(lane, GUTTER_LANE_COUNT, GUTTER_X / 3.0),
                centered_index(row, 2, 62.0),
                GUTTER_Z / 2.0 - 8.0,
            );
    }
    cuts
}

fn overflow_cup_cuts() -> Part {
    let mut cuts = Part::empty("water_pan_refill_overflow_cup_cuts");
    for cup in 0..OVERFLOW_CUP_COUNT {
        cuts = cuts
            + centered_cylinder(
                format!("water_pan_refill_overflow_evidence_cup_cut_{cup}"),
                27.0,
                22.0,
                36,
            )
            .translate(
                centered_index(cup, OVERFLOW_CUP_COUNT, GUTTER_X / 3.0),
                -GUTTER_Y / 2.0 + 38.0,
                GUTTER_Z / 2.0 - 10.0,
            );
    }
    cuts
}

fn gutter_diverter_vanes() -> Part {
    let mut vanes = Part::empty("water_pan_refill_gutter_diverter_vanes");
    for vane in 0..GUTTER_CHANNEL_COUNT {
        let lane = vane % GUTTER_LANE_COUNT;
        let row = vane / GUTTER_LANE_COUNT;
        vanes = vanes
            + centered_cube(
                format!("water_pan_refill_gutter_diverter_vane_{vane}"),
                12.0,
                58.0,
                20.0,
            )
            .rotate(0.0, 0.0, if row == 0 { -14.0 } else { 14.0 })
            .translate(
                centered_index(lane, GUTTER_LANE_COUNT, GUTTER_X / 3.0) + 24.0,
                centered_index(row, 2, 62.0),
                GUTTER_Z / 2.0 + 10.0,
            );
    }
    vanes
}

fn drain_trap_witness_geometry() -> Part {
    let mut traps = Part::empty("water_pan_refill_drain_trap_witness_geometry");
    for trap in 0..DRAIN_TRAP_COUNT {
        let x = centered_index(trap, DRAIN_TRAP_COUNT, 164.0) + 106.0;
        let inlet = centered_cylinder(
            format!("water_pan_refill_drain_trap_{trap}_inlet_standpipe"),
            DRAIN_TRAP_D / 2.0,
            46.0,
            36,
        )
        .translate(x - 24.0, GUTTER_Y / 2.0 - 46.0, GUTTER_Z / 2.0 + 23.0);
        let outlet = centered_cylinder(
            format!("water_pan_refill_drain_trap_{trap}_outlet_standpipe"),
            DRAIN_TRAP_D / 2.0,
            32.0,
            36,
        )
        .translate(x + 32.0, GUTTER_Y / 2.0 - 50.0, GUTTER_Z / 2.0 + 16.0);
        let water_seal = centered_cylinder(
            format!("water_pan_refill_drain_trap_{trap}_water_seal_loop"),
            9.0,
            68.0,
            24,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(x + 4.0, GUTTER_Y / 2.0 - 50.0, GUTTER_Z / 2.0 - 4.0);
        traps = traps + inlet + outlet + water_seal;
    }
    traps
}

fn gutter_lane_labels() -> Part {
    let mut labels = Part::empty("water_pan_refill_gutter_lane_labels");
    for lane in 0..GUTTER_LANE_COUNT {
        labels = labels
            + code_bar_label(
                &format!("water_pan_refill_overflow_lane_{lane}_csg_label"),
                116.0,
                18.0,
                lane + 2,
            )
            .translate(
                centered_index(lane, GUTTER_LANE_COUNT, GUTTER_X / 3.0),
                GUTTER_Y / 2.0 - 22.0,
                GUTTER_Z / 2.0 + LABEL_Z / 2.0,
            );
    }
    labels
}

fn level_sensor_pockets() -> Part {
    let body = centered_cube(
        "water_pan_refill_level_sensor_pocket_block",
        LEVEL_X,
        LEVEL_Y,
        LEVEL_Z,
    );
    let cable_trench = centered_cube(
        "water_pan_refill_level_sensor_cable_trench_cut",
        LEVEL_X - 54.0,
        16.0,
        18.0,
    )
    .translate(0.0, -LEVEL_Y / 2.0 + 28.0, LEVEL_Z / 2.0 - 8.0);

    body - level_sensor_pocket_cuts() - cable_trench
        + level_sensor_clip_ribs()
        + float_gauge_posts()
        + level_reference_lands()
}

fn level_sensor_pocket_cuts() -> Part {
    let mut pockets = Part::empty("water_pan_refill_level_sensor_pocket_cuts");
    for sensor in 0..LEVEL_SENSOR_COUNT {
        pockets = pockets
            + centered_cube(
                format!("water_pan_refill_level_sensor_pocket_cut_{sensor}"),
                LEVEL_SENSOR_POCKET_X,
                LEVEL_SENSOR_POCKET_Y,
                LEVEL_SENSOR_DEPTH,
            )
            .translate(
                centered_index(sensor, LEVEL_SENSOR_COUNT, 78.0),
                16.0,
                LEVEL_Z / 2.0 - LEVEL_SENSOR_DEPTH / 2.0 + 1.0,
            );
    }
    pockets
}

fn level_sensor_clip_ribs() -> Part {
    let mut ribs = Part::empty("water_pan_refill_level_sensor_clip_ribs");
    for sensor in 0..LEVEL_SENSOR_COUNT {
        let x = centered_index(sensor, LEVEL_SENSOR_COUNT, 78.0);
        ribs =
            ribs + centered_cube(
                format!("water_pan_refill_level_sensor_{sensor}_left_clip"),
                5.0,
                LEVEL_SENSOR_POCKET_Y + 10.0,
                12.0,
            )
            .translate(
                x - LEVEL_SENSOR_POCKET_X / 2.0 - 6.0,
                16.0,
                LEVEL_Z / 2.0 + 6.0,
            ) + centered_cube(
                format!("water_pan_refill_level_sensor_{sensor}_right_clip"),
                5.0,
                LEVEL_SENSOR_POCKET_Y + 10.0,
                12.0,
            )
            .translate(
                x + LEVEL_SENSOR_POCKET_X / 2.0 + 6.0,
                16.0,
                LEVEL_Z / 2.0 + 6.0,
            );
    }
    ribs
}

fn float_gauge_posts() -> Part {
    let mut posts = Part::empty("water_pan_refill_float_gauge_posts");
    for gauge in 0..FLOAT_GAUGE_COUNT {
        posts = posts
            + centered_cylinder(
                format!("water_pan_refill_float_gauge_post_{gauge}"),
                13.0,
                56.0,
                32,
            )
            .translate(
                centered_index(gauge, FLOAT_GAUGE_COUNT, 102.0),
                -LEVEL_Y / 2.0 + 42.0,
                LEVEL_Z / 2.0 + 28.0,
            )
            + centered_cube(
                format!("water_pan_refill_float_gauge_index_flag_{gauge}"),
                34.0,
                6.0,
                6.0,
            )
            .translate(
                centered_index(gauge, FLOAT_GAUGE_COUNT, 102.0),
                -LEVEL_Y / 2.0 + 72.0,
                LEVEL_Z / 2.0 + 52.0 - gauge as f64 * 8.0,
            );
    }
    posts
}

fn level_reference_lands() -> Part {
    let wet = code_bar_label("water_pan_refill_level_wet_csg_label", 74.0, 16.0, 1).translate(
        -112.0,
        LEVEL_Y / 2.0 - 24.0,
        LEVEL_Z / 2.0 + LABEL_Z / 2.0,
    );
    let dry = code_bar_label("water_pan_refill_level_dry_csg_label", 74.0, 16.0, 2).translate(
        0.0,
        LEVEL_Y / 2.0 - 24.0,
        LEVEL_Z / 2.0 + LABEL_Z / 2.0,
    );
    let high = code_bar_label("water_pan_refill_level_high_csg_label", 74.0, 16.0, 3).translate(
        112.0,
        LEVEL_Y / 2.0 - 24.0,
        LEVEL_Z / 2.0 + LABEL_Z / 2.0,
    );
    wet + dry + high
}

fn biofilm_decon_residue_coupon_carriers() -> Part {
    let body = centered_cube(
        "water_pan_refill_biofilm_decon_coupon_carrier_body",
        COUPON_X,
        COUPON_Y,
        COUPON_Z,
    );
    let rinse_channel = centered_cube(
        "water_pan_refill_coupon_carrier_rinse_channel_cut",
        COUPON_X - 54.0,
        16.0,
        16.0,
    )
    .translate(0.0, -COUPON_Y / 2.0 + 34.0, COUPON_Z / 2.0 - 8.0);

    body - coupon_slot_cuts() - rinse_channel
        + coupon_rack_dividers()
        + decon_shadow_masks()
        + coupon_material_code_lands()
}

fn coupon_slot_cuts() -> Part {
    let mut cuts = Part::empty("water_pan_refill_coupon_slot_cuts");
    for rack in 0..COUPON_RACK_COUNT {
        for slot in 0..COUPONS_PER_RACK {
            let index = rack * COUPONS_PER_RACK + slot;
            cuts =
                cuts + centered_cube(
                    format!("water_pan_refill_coupon_rack_{rack}_slot_{slot}_cut"),
                    COUPON_SLOT_X,
                    COUPON_SLOT_Y,
                    COUPON_SLOT_DEPTH,
                )
                .translate(
                    centered_index(slot, COUPONS_PER_RACK, 58.0),
                    centered_index(rack, COUPON_RACK_COUNT, 42.0) + 18.0,
                    COUPON_Z / 2.0 - COUPON_SLOT_DEPTH / 2.0 + 1.0,
                ) + centered_cube(
                    format!("water_pan_refill_coupon_slot_{index}_index_notch"),
                    6.0,
                    COUPON_SLOT_Y + 8.0,
                    8.0,
                )
                .translate(
                    centered_index(slot, COUPONS_PER_RACK, 58.0) - COUPON_SLOT_X / 2.0,
                    centered_index(rack, COUPON_RACK_COUNT, 42.0) + 18.0,
                    COUPON_Z / 2.0 - 4.0,
                );
        }
    }
    cuts
}

fn coupon_rack_dividers() -> Part {
    let mut dividers = Part::empty("water_pan_refill_coupon_rack_dividers");
    for rack in 1..COUPON_RACK_COUNT {
        dividers = dividers
            + centered_cube(
                format!("water_pan_refill_coupon_rack_divider_{rack}"),
                COUPON_X - 58.0,
                5.0,
                18.0,
            )
            .translate(
                0.0,
                centered_index(rack - 1, COUPON_RACK_COUNT - 1, 42.0) + 39.0,
                COUPON_Z / 2.0 + 9.0,
            );
    }
    dividers
}

fn decon_shadow_masks() -> Part {
    let mut masks = Part::empty("water_pan_refill_decon_shadow_masks");
    for mask in 0..DECON_SHADOW_MASK_COUNT {
        masks = masks
            + centered_cube(
                format!("water_pan_refill_decon_shadow_mask_{mask}"),
                22.0,
                38.0,
                8.0,
            )
            .rotate(0.0, 0.0, if mask % 2 == 0 { -18.0 } else { 18.0 })
            .translate(
                centered_index(mask % 4, 4, 72.0),
                -COUPON_Y / 2.0 + 54.0 + (mask / 4) as f64 * 38.0,
                COUPON_Z / 2.0 + 4.0,
            );
    }
    masks
}

fn coupon_material_code_lands() -> Part {
    let mut lands = Part::empty("water_pan_refill_coupon_material_code_lands");
    for rack in 0..COUPON_RACK_COUNT {
        lands = lands
            + code_bar_label(
                &format!("water_pan_refill_coupon_rack_{rack}_material_label"),
                78.0,
                15.0,
                rack + 4,
            )
            .translate(
                -COUPON_X / 2.0 + 48.0,
                centered_index(rack, COUPON_RACK_COUNT, 42.0) + 18.0,
                COUPON_Z / 2.0 + LABEL_Z / 2.0,
            );
    }
    lands
}

fn splash_baffle_array() -> Part {
    let spine = centered_cube(
        "water_pan_refill_splash_baffle_removable_spine",
        BAFFLE_X,
        18.0,
        18.0,
    )
    .translate(0.0, -BAFFLE_Y / 2.0 + 20.0, BAFFLE_Z / 2.0 - 9.0);

    spine + splash_baffle_blades() + baffle_drain_slots() + baffle_lock_tabs()
}

fn splash_baffle_blades() -> Part {
    let mut blades = Part::empty("water_pan_refill_splash_baffle_blades");
    for baffle in 0..SPLASH_BAFFLE_COUNT {
        blades = blades
            + centered_cube(
                format!("water_pan_refill_splash_baffle_blade_{baffle}"),
                9.0,
                BAFFLE_Y - 54.0,
                BAFFLE_Z,
            )
            .rotate(0.0, 0.0, if baffle % 2 == 0 { -7.0 } else { 7.0 })
            .translate(
                centered_index(baffle, SPLASH_BAFFLE_COUNT, 72.0),
                8.0,
                BAFFLE_Z / 2.0,
            );
    }
    blades
}

fn baffle_drain_slots() -> Part {
    let mut slots = Part::empty("water_pan_refill_splash_baffle_drain_slots");
    for slot in 0..BAFFLE_DRAIN_SLOT_COUNT {
        slots = slots
            + centered_cube(
                format!("water_pan_refill_baffle_drain_slot_marker_{slot}"),
                26.0,
                6.0,
                5.0,
            )
            .translate(
                centered_index(slot % 7, 7, 72.0),
                -BAFFLE_Y / 2.0 + 54.0 + (slot / 7) as f64 * 130.0,
                4.0,
            );
    }
    slots
}

fn baffle_lock_tabs() -> Part {
    let mut tabs = Part::empty("water_pan_refill_splash_baffle_lock_tabs");
    for (i, (x, y)) in [
        (-BAFFLE_X / 2.0 + 38.0, -BAFFLE_Y / 2.0 + 28.0),
        (BAFFLE_X / 2.0 - 38.0, -BAFFLE_Y / 2.0 + 28.0),
        (-BAFFLE_X / 2.0 + 38.0, BAFFLE_Y / 2.0 - 28.0),
        (BAFFLE_X / 2.0 - 38.0, BAFFLE_Y / 2.0 - 28.0),
    ]
    .into_iter()
    .enumerate()
    {
        tabs = tabs
            + centered_cube(
                format!("water_pan_refill_splash_baffle_lock_tab_{i}"),
                34.0,
                20.0,
                12.0,
            )
            .translate(x, y, 6.0);
    }
    tabs
}

fn conductivity_turbidity_sample_wells() -> Part {
    let body = centered_cube(
        "water_pan_refill_conductivity_turbidity_sample_well_body",
        SAMPLE_X,
        SAMPLE_Y,
        SAMPLE_Z,
    );
    let aliquot_channel = centered_cube(
        "water_pan_refill_sample_well_aliquot_channel_cut",
        SAMPLE_X - 62.0,
        12.0,
        12.0,
    )
    .translate(0.0, 0.0, SAMPLE_Z / 2.0 - 6.0);

    body - sample_well_cuts() - aliquot_channel
        + conductivity_probe_lands()
        + turbidity_window_lands()
        + sample_lane_labels()
}

fn sample_well_cuts() -> Part {
    let mut wells = Part::empty("water_pan_refill_sample_well_cuts");
    for well in 0..SAMPLE_WELL_COUNT {
        wells = wells
            + centered_cylinder(
                format!("water_pan_refill_conductivity_turbidity_sample_well_{well}"),
                SAMPLE_WELL_D / 2.0,
                SAMPLE_WELL_DEPTH,
                36,
            )
            .translate(
                centered_index(well % 3, 3, 98.0),
                centered_index(well / 3, 2, 62.0) + 8.0,
                SAMPLE_Z / 2.0 - SAMPLE_WELL_DEPTH / 2.0 + 1.0,
            );
    }
    wells
}

fn conductivity_probe_lands() -> Part {
    let mut lands = Part::empty("water_pan_refill_conductivity_probe_lands");
    for probe in 0..CONDUCTIVITY_PROBE_COUNT {
        lands = lands
            + centered_cube(
                format!("water_pan_refill_conductivity_probe_flat_{probe}"),
                50.0,
                18.0,
                4.0,
            )
            .translate(
                centered_index(probe, CONDUCTIVITY_PROBE_COUNT, 72.0),
                -SAMPLE_Y / 2.0 + 28.0,
                SAMPLE_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn turbidity_window_lands() -> Part {
    let mut lands = Part::empty("water_pan_refill_turbidity_window_lands");
    for window in 0..TURBIDITY_WINDOW_COUNT {
        lands = lands
            + centered_cube(
                format!("water_pan_refill_turbidity_optical_window_{window}"),
                42.0,
                6.0,
                16.0,
            )
            .translate(
                centered_index(window, TURBIDITY_WINDOW_COUNT, 76.0),
                SAMPLE_Y / 2.0 - 24.0,
                0.0,
            );
    }
    lands
}

fn sample_lane_labels() -> Part {
    let blank = code_bar_label("water_pan_refill_sample_blank_csg_label", 78.0, 16.0, 8).translate(
        -112.0,
        SAMPLE_Y / 2.0 - 44.0,
        SAMPLE_Z / 2.0 + LABEL_Z / 2.0,
    );
    let pan = code_bar_label("water_pan_refill_sample_pan_csg_label", 78.0, 16.0, 9).translate(
        0.0,
        SAMPLE_Y / 2.0 - 44.0,
        SAMPLE_Z / 2.0 + LABEL_Z / 2.0,
    );
    let trap = code_bar_label("water_pan_refill_sample_trap_csg_label", 78.0, 16.0, 10).translate(
        112.0,
        SAMPLE_Y / 2.0 - 44.0,
        SAMPLE_Z / 2.0 + LABEL_Z / 2.0,
    );
    blank + pan + trap
}

fn barcode_custody_lands() -> Part {
    let plate = centered_cube(
        "water_pan_refill_barcode_custody_land_plate",
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    plate + barcode_lands() + custody_seal_lands() + run_record_lands()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("water_pan_refill_barcode_lands");
    for land in 0..BARCODE_LAND_COUNT {
        lands = lands
            + centered_cube(
                format!("water_pan_refill_barcode_land_{land}"),
                68.0,
                16.0,
                2.4,
            )
            .translate(
                centered_index(land % 6, 6, 76.0),
                centered_index(land / 6, 2, 24.0) + 7.0,
                CUSTODY_Z / 2.0 + 1.2,
            );
    }
    lands
}

fn custody_seal_lands() -> Part {
    let mut lands = Part::empty("water_pan_refill_custody_seal_lands");
    for seal in 0..CUSTODY_SEAL_COUNT {
        lands = lands
            + centered_cube(
                format!("water_pan_refill_tamper_custody_seal_land_{seal}"),
                44.0,
                14.0,
                3.0,
            )
            .translate(
                centered_index(seal, CUSTODY_SEAL_COUNT, 76.0),
                -CUSTODY_Y / 2.0 + 14.0,
                CUSTODY_Z / 2.0 + 1.5,
            );
    }
    lands
}

fn run_record_lands() -> Part {
    let mut lands = Part::empty("water_pan_refill_run_record_lands");
    for record in 0..RUN_RECORD_LAND_COUNT {
        lands = lands
            + code_bar_label(
                &format!("water_pan_refill_run_record_{record}_csg_label"),
                84.0,
                14.0,
                record + 11,
            )
            .translate(
                centered_index(record, RUN_RECORD_LAND_COUNT, 102.0),
                CUSTODY_Y / 2.0 - 12.0,
                CUSTODY_Z / 2.0 + LABEL_Z / 2.0,
            );
    }
    lands
}

fn clean_dirty_segregation_bulkhead() -> Part {
    let wall = centered_cube(
        "water_pan_refill_clean_dirty_segregation_bulkhead_wall",
        SEGREGATION_X,
        SEGREGATION_Y,
        SEGREGATION_Z,
    );
    let cap = centered_cube(
        "water_pan_refill_clean_dirty_segregation_wipe_cap",
        SEGREGATION_X + 36.0,
        SEGREGATION_Y,
        10.0,
    )
    .translate(0.0, 0.0, SEGREGATION_Z / 2.0 + 5.0);

    wall - segregation_pass_gate_cuts() + cap + dirty_drip_ribs() + segregation_side_labels()
}

fn segregation_pass_gate_cuts() -> Part {
    let mut cuts = Part::empty("water_pan_refill_segregation_pass_gate_cuts");
    for gate in 0..SEGREGATION_PASS_GATE_COUNT {
        cuts = cuts
            + centered_cube(
                format!("water_pan_refill_segregation_pass_gate_cut_{gate}"),
                SEGREGATION_X + 8.0,
                54.0,
                34.0,
            )
            .translate(
                0.0,
                centered_index(gate, SEGREGATION_PASS_GATE_COUNT, 126.0),
                6.0,
            );
    }
    cuts
}

fn dirty_drip_ribs() -> Part {
    let mut ribs = Part::empty("water_pan_refill_dirty_side_drip_ribs");
    for rib in 0..DIRTY_DRIP_RIB_COUNT {
        ribs = ribs
            + centered_cube(
                format!("water_pan_refill_dirty_side_drip_rib_{rib}"),
                20.0,
                5.0,
                10.0,
            )
            .translate(
                SEGREGATION_X / 2.0 + 16.0,
                centered_index(rib, DIRTY_DRIP_RIB_COUNT, 96.0),
                SEGREGATION_Z / 2.0 + 5.0,
            );
    }
    ribs
}

fn segregation_side_labels() -> Part {
    let clean = code_bar_label("water_pan_refill_clean_side_csg_label", 86.0, 16.0, 12)
        .rotate(0.0, 0.0, 90.0)
        .translate(-22.0, 220.0, SEGREGATION_Z / 2.0 + LABEL_Z / 2.0);
    let dirty = code_bar_label("water_pan_refill_dirty_side_csg_label", 86.0, 16.0, 13)
        .rotate(0.0, 0.0, 90.0)
        .translate(22.0, -220.0, SEGREGATION_Z / 2.0 + LABEL_Z / 2.0);
    clean + dirty
}

fn evidence_bridge() -> Part {
    let left_post = centered_cube(
        "water_pan_refill_evidence_bridge_left_post",
        32.0,
        EVIDENCE_Y,
        EVIDENCE_CLEARANCE_Z,
    )
    .translate(-EVIDENCE_X / 2.0 + 58.0, 0.0, EVIDENCE_CLEARANCE_Z / 2.0);
    let right_post = centered_cube(
        "water_pan_refill_evidence_bridge_right_post",
        32.0,
        EVIDENCE_Y,
        EVIDENCE_CLEARANCE_Z,
    )
    .translate(EVIDENCE_X / 2.0 - 58.0, 0.0, EVIDENCE_CLEARANCE_Z / 2.0);
    let beam = centered_cube(
        "water_pan_refill_evidence_bridge_top_beam",
        EVIDENCE_X,
        EVIDENCE_Y,
        EVIDENCE_BEAM_Z,
    )
    .translate(0.0, 0.0, EVIDENCE_CLEARANCE_Z + EVIDENCE_BEAM_Z / 2.0);

    left_post + right_post + beam + evidence_camera_pods() + evidence_light_bars()
}

fn evidence_camera_pods() -> Part {
    let mut pods = Part::empty("water_pan_refill_evidence_camera_pods");
    for camera in 0..EVIDENCE_CAMERA_COUNT {
        pods =
            pods + centered_cube(
                format!("water_pan_refill_evidence_camera_pod_{camera}"),
                56.0,
                32.0,
                24.0,
            )
            .translate(
                centered_index(camera, EVIDENCE_CAMERA_COUNT, 235.0),
                -EVIDENCE_Y / 2.0 - 18.0,
                EVIDENCE_CLEARANCE_Z - 22.0,
            ) + centered_cylinder(
                format!("water_pan_refill_evidence_camera_lens_boss_{camera}"),
                9.0,
                8.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(camera, EVIDENCE_CAMERA_COUNT, 235.0),
                -EVIDENCE_Y / 2.0 - 36.0,
                EVIDENCE_CLEARANCE_Z - 22.0,
            );
    }
    pods
}

fn evidence_light_bars() -> Part {
    let mut bars = Part::empty("water_pan_refill_evidence_light_bars");
    for bar in 0..EVIDENCE_LIGHT_BAR_COUNT {
        bars = bars
            + centered_cube(
                format!("water_pan_refill_evidence_light_bar_{bar}"),
                EVIDENCE_X - 230.0,
                8.0,
                10.0,
            )
            .translate(
                0.0,
                if bar == 0 {
                    EVIDENCE_Y / 2.0 + 8.0
                } else {
                    -EVIDENCE_Y / 2.0 - 8.0
                },
                EVIDENCE_CLEARANCE_Z - 52.0,
            );
    }
    bars
}

fn robot_service_keepout_gauges() -> Part {
    keepout_outline()
        + front_robot_approach_gauge()
        + rear_service_sweep_gauge()
        + side_fill_tube_gauge()
        + pan_lift_clearance_towers()
        + evidence_shadow_gauge()
}

fn keepout_outline() -> Part {
    let front = centered_cube(
        "water_pan_refill_keepout_outline_front",
        KEEP_OUT_X,
        8.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, -KEEP_OUT_Y / 2.0, KEEP_OUT_Z / 2.0);
    let rear = centered_cube(
        "water_pan_refill_keepout_outline_rear",
        KEEP_OUT_X,
        8.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, KEEP_OUT_Y / 2.0, KEEP_OUT_Z / 2.0);
    let left = centered_cube(
        "water_pan_refill_keepout_outline_left",
        8.0,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(-KEEP_OUT_X / 2.0, 0.0, KEEP_OUT_Z / 2.0);
    let right = centered_cube(
        "water_pan_refill_keepout_outline_right",
        8.0,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(KEEP_OUT_X / 2.0, 0.0, KEEP_OUT_Z / 2.0);
    front + rear + left + right
}

fn front_robot_approach_gauge() -> Part {
    centered_cube(
        "water_pan_refill_front_robot_approach_keepout_gauge",
        KEEP_OUT_X - 190.0,
        18.0,
        KEEP_OUT_Z,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 + ROBOT_FRONT_CLEARANCE_Y,
        KEEP_OUT_Z / 2.0,
    )
}

fn rear_service_sweep_gauge() -> Part {
    centered_cube(
        "water_pan_refill_rear_service_sweep_keepout_gauge",
        KEEP_OUT_X - 220.0,
        18.0,
        KEEP_OUT_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - SERVICE_REAR_CLEARANCE_Y,
        KEEP_OUT_Z / 2.0,
    )
}

fn side_fill_tube_gauge() -> Part {
    centered_cube(
        "water_pan_refill_side_fill_tube_bend_radius_keepout_gauge",
        18.0,
        KEEP_OUT_Y - 190.0,
        KEEP_OUT_Z,
    )
    .translate(
        -STATION_X / 2.0 + SIDE_FILL_SERVICE_X,
        0.0,
        KEEP_OUT_Z / 2.0,
    )
}

fn pan_lift_clearance_towers() -> Part {
    let mut towers = Part::empty("water_pan_refill_pan_lift_clearance_towers");
    for (i, (x, y)) in [
        (
            PAN_POS.0 - PAN_X / 2.0 + 40.0,
            PAN_POS.1 - PAN_Y / 2.0 + 42.0,
        ),
        (
            PAN_POS.0 + PAN_X / 2.0 - 40.0,
            PAN_POS.1 - PAN_Y / 2.0 + 42.0,
        ),
        (
            PAN_POS.0 - PAN_X / 2.0 + 40.0,
            PAN_POS.1 + PAN_Y / 2.0 - 42.0,
        ),
        (
            PAN_POS.0 + PAN_X / 2.0 - 40.0,
            PAN_POS.1 + PAN_Y / 2.0 - 42.0,
        ),
    ]
    .into_iter()
    .enumerate()
    {
        towers = towers
            + centered_cylinder(
                format!("water_pan_refill_pan_lift_z_clearance_tower_{i}"),
                8.0,
                PAN_LIFT_CLEARANCE_Z,
                20,
            )
            .translate(x, y, PAN_LIFT_CLEARANCE_Z / 2.0);
    }
    towers
}

fn evidence_shadow_gauge() -> Part {
    centered_cube(
        "water_pan_refill_evidence_bridge_shadow_keepout_gauge",
        EVIDENCE_X - 180.0,
        12.0,
        KEEP_OUT_Z,
    )
    .translate(EVIDENCE_POS.0, EVIDENCE_POS.1 - 52.0, KEEP_OUT_Z / 2.0)
}

fn fiducial_disc(name: &str) -> Part {
    let disc = centered_cylinder(format!("{name}_outer_disc"), 11.0, 5.0, 36);
    let center = centered_cylinder(format!("{name}_center_dot"), 2.2, 6.0, 20);
    let cross_x = centered_cube(format!("{name}_cross_x"), 18.0, 2.2, 6.0);
    let cross_y = centered_cube(format!("{name}_cross_y"), 2.2, 18.0, 6.0);
    disc - center - cross_x - cross_y
}

fn code_bar_label(label: &str, x: f64, y: f64, code: usize) -> Part {
    let land = centered_cube(format!("{label}_land"), x, y, LABEL_Z);
    let mut bars = Part::empty(format!("{label}_raised_bar_code"));
    for bar in 0..8 {
        if ((code + 3) >> (bar % 5)) & 1 == 1 || bar == 0 || bar == 7 {
            bars = bars
                + centered_cube(
                    format!("{label}_raised_bar_{bar}"),
                    3.0 + (bar % 3) as f64,
                    y - 5.0,
                    LABEL_Z,
                )
                .translate(centered_index(bar, 8, x / 10.5), 0.0, LABEL_Z);
        }
    }
    land + bars
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_scoped_and_stable() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 13);
        for path in OUTPUTS {
            assert!(path
                .starts_with("output/closed_incubator_water_pan_refill_overflow_decon_station_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn requested_feature_groups_are_explicit() {
        for feature in [
            "water_pan_surrogate",
            "fill_line_quick_connect_dock",
            "overflow_gutter_drain_trap",
            "level_sensor_pockets",
            "biofilm_decon_residue_coupon_carriers",
            "splash_baffles",
            "conductivity_turbidity_sample_wells",
            "barcode_custody_lands",
            "clean_dirty_segregation",
            "evidence_bridge",
            "robot_service_keepout_gauges",
            "named_stl_outputs",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn modules_fit_and_do_not_overlap_on_deck() {
        assert_design_constraints();
        let rects = deck_socket_rects();
        for rect in rects {
            assert!(rect.fits_inside_deck(), "{} does not fit", rect.name);
        }
    }

    #[test]
    fn refill_overflow_and_containment_budget_is_explicit() {
        assert!(water_pan_working_volume_ml() > 3000.0);
        assert!(water_pan_working_volume_ml() > refill_charge_volume_ml());
        assert!(worst_case_overflow_volume_ml() > 1400.0);
        assert!(containment_freeboard_ml() > worst_case_overflow_volume_ml());
        assert_eq!(GUTTER_LANE_COUNT, 3);
        assert_eq!(DRAIN_TRAP_COUNT, 2);
        assert_eq!(OVERFLOW_CUP_COUNT, 3);
    }

    #[test]
    fn sensors_coupons_baffles_and_samples_cover_requested_risks() {
        assert_eq!(LEVEL_SENSOR_COUNT, 4);
        assert_eq!(FLOAT_GAUGE_COUNT, 3);
        assert_eq!(COUPON_SLOT_COUNT, 24);
        assert_eq!(DECON_SHADOW_MASK_COUNT, 8);
        assert_eq!(SPLASH_BAFFLE_COUNT, 7);
        assert_eq!(BAFFLE_DRAIN_SLOT_COUNT, SPLASH_BAFFLE_COUNT * 2);
        assert_eq!(SAMPLE_WELL_COUNT, 6);
        assert_eq!(CONDUCTIVITY_PROBE_COUNT, 4);
        assert_eq!(TURBIDITY_WINDOW_COUNT, 4);
    }

    #[test]
    fn traceability_evidence_and_keepouts_are_complete() {
        assert_eq!(BARCODE_LAND_COUNT, 12);
        assert_eq!(CUSTODY_SEAL_COUNT, 6);
        assert_eq!(RUN_RECORD_LAND_COUNT, 4);
        assert_eq!(SEGREGATION_PASS_GATE_COUNT, 5);
        assert_eq!(EVIDENCE_CAMERA_COUNT, 5);
        assert_eq!(EVIDENCE_LIGHT_BAR_COUNT, 2);
        assert_eq!(KEEP_OUT_GAUGE_COUNT, 6);
        assert!(PAN_LIFT_CLEARANCE_Z > EVIDENCE_CLEARANCE_Z);
    }
}
