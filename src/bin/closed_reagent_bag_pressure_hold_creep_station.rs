use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed reagent/media bag pressure-hold creep validation station.
//
// Intent:
// - Hold a sealed single-use bag surrogate under repeatable clamp and pressure
//   boundary conditions without opening the fluid path.
// - Make pressure input/output bulkheads, a reference gauge, displacement creep
//   witness scale, clamp rails, leak containment, and surrogate coupon identity
//   physically visible in the fixture geometry.
// - This is concept/interface CAD only. It is not pressure-rated hardware,
//   a sterilization claim, a bag material acceptance criterion, or a test SOP.

const OUTPUTS: [&str; 12] = [
    "output/closed_reagent_bag_pressure_hold_creep_station_secondary_containment_tray.stl",
    "output/closed_reagent_bag_pressure_hold_creep_station_guarded_bag_tray.stl",
    "output/closed_reagent_bag_pressure_hold_creep_station_pressure_io_bulkheads.stl",
    "output/closed_reagent_bag_pressure_hold_creep_station_reference_gauge_bracket.stl",
    "output/closed_reagent_bag_pressure_hold_creep_station_displacement_creep_witness_scale.stl",
    "output/closed_reagent_bag_pressure_hold_creep_station_clamp_rails.stl",
    "output/closed_reagent_bag_pressure_hold_creep_station_leak_witness_secondary_containment.stl",
    "output/closed_reagent_bag_pressure_hold_creep_station_labeled_surrogate_coupons.stl",
    "output/closed_reagent_bag_pressure_hold_creep_station_tubing_strain_relief_comb.stl",
    "output/closed_reagent_bag_pressure_hold_creep_station_run_evidence_label_plate.stl",
    "output/closed_reagent_bag_pressure_hold_creep_station_robot_service_keepouts.stl",
    "output/closed_reagent_bag_pressure_hold_creep_station_assembly.stl",
];

#[cfg(test)]
const REQUIRED_FEATURES: [&str; 9] = [
    "guarded_bag_tray",
    "pressure_input_bulkhead",
    "pressure_output_bulkhead",
    "reference_gauge_bracket",
    "displacement_creep_witness_scale",
    "clamp_rails",
    "leak_safe_secondary_containment",
    "labeled_surrogate_coupons",
    "tubing_strain_relief_comb",
];

const STATION_X: f64 = 1380.0;
const STATION_Y: f64 = 860.0;
const BASE_Z: f64 = 24.0;
const CURB_W: f64 = 22.0;
const CURB_Z: f64 = 54.0;
const SUMP_X: f64 = 1190.0;
const SUMP_Y: f64 = 690.0;
const SUMP_DEPTH: f64 = 9.0;
const MOUNT_HOLE_D: f64 = 6.8;
const DRAIN_PORT_D: f64 = 19.0;
const DATUM_TARGETS: usize = 4;

const BAG_POS: (f64, f64) = (-330.0, 80.0);
const BAG_TRAY_X: f64 = 500.0;
const BAG_TRAY_Y: f64 = 350.0;
const BAG_TRAY_Z: f64 = 54.0;
const BAG_RECESS_X: f64 = 358.0;
const BAG_RECESS_Y: f64 = 214.0;
const BAG_RECESS_DEPTH: f64 = 20.0;
const BAG_GUARD_Z: f64 = 116.0;
const BAG_GUARD_RAIL_W: f64 = 18.0;
const BAG_SADDLE_RIBS: usize = 7;
const BAG_DRAIN_SLOTS: usize = 5;
const BAG_LOCATOR_PINS: usize = 6;
const BAG_PRESSURE_PAD_X: f64 = 250.0;
const BAG_PRESSURE_PAD_Y: f64 = 148.0;

const BULKHEAD_POS: (f64, f64) = (300.0, 236.0);
const BULKHEAD_PANEL_X: f64 = 620.0;
const BULKHEAD_PANEL_Y: f64 = 38.0;
const BULKHEAD_PANEL_Z: f64 = 220.0;
const BULKHEAD_FOOT_Z: f64 = 30.0;
const PRESSURE_CHANNELS: usize = 4;
const PRESSURE_PORT_PAIRS: usize = PRESSURE_CHANNELS;
const PRESSURE_PORT_D: f64 = 15.0;
const PRESSURE_COLLAR_D: f64 = 34.0;
const PORT_PITCH_X: f64 = 118.0;
const PORT_ROW_OFFSET_Y: f64 = 14.0;
const PORT_INPUT_Z: f64 = 132.0;
const PORT_OUTPUT_Z: f64 = 74.0;
const BULKHEAD_KEY_W: f64 = 8.0;
const BULKHEAD_KEY_Z: f64 = 20.0;

const GAUGE_POS: (f64, f64) = (492.0, 12.0);
const GAUGE_BASE_X: f64 = 260.0;
const GAUGE_BASE_Y: f64 = 178.0;
const GAUGE_BASE_Z: f64 = 26.0;
const GAUGE_UPRIGHT_X: f64 = 170.0;
const GAUGE_UPRIGHT_Y: f64 = 28.0;
const GAUGE_UPRIGHT_Z: f64 = 236.0;
const GAUGE_RING_OD: f64 = 132.0;
const GAUGE_RING_ID: f64 = 86.0;
const GAUGE_TICK_COUNT: usize = 12;
const GAUGE_SNUBBER_COUNT: usize = 3;

const SCALE_POS: (f64, f64) = (-610.0, -235.0);
const SCALE_BASE_X: f64 = 60.0;
const SCALE_BASE_Y: f64 = 180.0;
const SCALE_BASE_Z: f64 = 28.0;
const SCALE_TOWER_X: f64 = 46.0;
const SCALE_TOWER_Y: f64 = 34.0;
const SCALE_TOWER_Z: f64 = 360.0;
const SCALE_TICKS: usize = 21;
const SCALE_MAJOR_EVERY: usize = 5;
const CREEP_POINTER_COUNT: usize = 3;
const CREEP_POINTER_PITCH_Z: f64 = 62.0;
const CREEP_TRAVEL_Z: f64 = 245.0;

const CLAMP_POS: (f64, f64) = BAG_POS;
const CLAMP_RAIL_X: f64 = 560.0;
const CLAMP_RAIL_Y: f64 = 22.0;
const CLAMP_RAIL_Z: f64 = 38.0;
const CLAMP_RAIL_OFFSET_Y: f64 = 188.0;
const CLAMP_SHOE_COUNT: usize = 6;
const CLAMP_SHOE_X: f64 = 56.0;
const CLAMP_SHOE_Y: f64 = 44.0;
const CLAMP_SHOE_Z: f64 = 22.0;
const CLAMP_KNOB_D: f64 = 28.0;
const CLAMP_FORCE_FLAG_COUNT: usize = 6;

const CONTAINMENT_POS: (f64, f64) = (-300.0, -270.0);
const CONTAINMENT_X: f64 = 550.0;
const CONTAINMENT_Y: f64 = 205.0;
const CONTAINMENT_Z: f64 = 46.0;
const CONTAINMENT_WALL: f64 = 14.0;
const LEAK_SENSOR_WELLS: usize = 8;
const ABSORBENT_PAD_LANDS: usize = 5;
const LEAK_WITNESS_RIBS: usize = 8;

const COUPON_POS: (f64, f64) = (300.0, -270.0);
const COUPON_BLOCK_X: f64 = 500.0;
const COUPON_BLOCK_Y: f64 = 205.0;
const COUPON_BLOCK_Z: f64 = 34.0;
const SURROGATE_COUPONS: usize = 8;
const COUPON_COLS: usize = 4;
const COUPON_X: f64 = 86.0;
const COUPON_Y: f64 = 58.0;
const COUPON_Z: f64 = 10.0;
const COUPON_PITCH_X: f64 = 106.0;
const COUPON_PITCH_Y: f64 = 78.0;
const COUPON_LABEL_LANDS: usize = SURROGATE_COUPONS;
const COUPON_BARCODE_STRIPES: usize = 4;

const TUBING_POS: (f64, f64) = (30.0, 20.0);
const TUBE_COMB_X: f64 = 385.0;
const TUBE_COMB_Y: f64 = 90.0;
const TUBE_COMB_Z: f64 = 34.0;
const TUBE_CHANNELS: usize = PRESSURE_PORT_PAIRS * 2;
const TUBE_OD_MAX: f64 = 9.6;
const TUBE_CLEARANCE: f64 = 1.6;
const TUBE_CHANNEL_D: f64 = TUBE_OD_MAX + TUBE_CLEARANCE;
const TUBE_CHANNEL_PITCH_X: f64 = 42.0;

const EVIDENCE_POS: (f64, f64) = (65.0, 340.0);
const EVIDENCE_X: f64 = 520.0;
const EVIDENCE_Y: f64 = 76.0;
const EVIDENCE_Z: f64 = 16.0;
const RUN_LABEL_LANDS: usize = 6;
const DISPOSITION_LANES: usize = 3;
const EVIDENCE_TOKEN_SLOTS: usize = 9;

const FRONT_ROBOT_APPROACH: f64 = 430.0;
const REAR_PRESSURE_SERVICE: f64 = 270.0;
const LEFT_SCALE_SERVICE: f64 = 180.0;
const RIGHT_GAUGE_SERVICE: f64 = 230.0;
const TOP_BAG_CREEP_CLEARANCE: f64 = 420.0;
const KEEP_OUT_RAIL: f64 = 8.0;

#[derive(Clone, Copy)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_station(self) -> bool {
        let usable_x = STATION_X / 2.0 - CURB_W - 26.0;
        let usable_y = STATION_Y / 2.0 - CURB_W - 26.0;
        self.center.0 - self.x / 2.0 >= -usable_x
            && self.center.0 + self.x / 2.0 <= usable_x
            && self.center.1 - self.y / 2.0 >= -usable_y
            && self.center.1 + self.y / 2.0 <= usable_y
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

    let base = secondary_containment_tray();
    export(OUTPUTS[0], &base);

    let bag = guarded_bag_tray();
    export(OUTPUTS[1], &bag);

    let bulkheads = pressure_io_bulkheads();
    export(OUTPUTS[2], &bulkheads);

    let gauge = reference_gauge_bracket();
    export(OUTPUTS[3], &gauge);

    let scale = displacement_creep_witness_scale();
    export(OUTPUTS[4], &scale);

    let clamps = clamp_rails();
    export(OUTPUTS[5], &clamps);

    let containment = leak_witness_secondary_containment();
    export(OUTPUTS[6], &containment);

    let coupons = labeled_surrogate_coupons();
    export(OUTPUTS[7], &coupons);

    let tubing = tubing_strain_relief_comb();
    export(OUTPUTS[8], &tubing);

    let evidence = run_evidence_label_plate();
    export(OUTPUTS[9], &evidence);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let assembly = station_assembly();
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed reagent/media bag pressure-hold creep station:");
    println!(
        "  Containment deck:           {STATION_X:.0}mm x {STATION_Y:.0}mm deck, {SUMP_X:.0}mm x {SUMP_Y:.0}mm sump, {DRAIN_PORT_D:.0}mm drain, {LEAK_SENSOR_WELLS} leak sensor wells"
    );
    println!(
        "  Guarded bag tray:           {BAG_TRAY_X:.0}mm x {BAG_TRAY_Y:.0}mm guarded tray, {BAG_SADDLE_RIBS} support ribs, {BAG_LOCATOR_PINS} locator pins, {BAG_DRAIN_SLOTS} drain slots"
    );
    println!(
        "  Pressure interfaces:        {PRESSURE_PORT_PAIRS} input/output port pairs, {TUBE_CHANNELS} strain-relieved tube channels, keyed bulkhead collars"
    );
    println!(
        "  Creep measurement:          reference gauge bracket with {GAUGE_TICK_COUNT} dial ticks, witness scale with {SCALE_TICKS} ticks and {CREEP_POINTER_COUNT} pointer carriages"
    );
    println!(
        "  Clamp and coupons:          {CLAMP_SHOE_COUNT} clamp shoes on dual rails, {SURROGATE_COUPONS} labeled surrogate coupons with barcode/lot lands"
    );
    println!(
        "  Service envelopes:          front robot {FRONT_ROBOT_APPROACH:.0}mm, rear pressure service {REAR_PRESSURE_SERVICE:.0}mm, left scale service {LEFT_SCALE_SERVICE:.0}mm, right gauge service {RIGHT_GAUGE_SERVICE:.0}mm, top creep clearance {TOP_BAG_CREEP_CLEARANCE:.0}mm"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    secondary_containment_tray()
        + guarded_bag_tray().translate(BAG_POS.0, BAG_POS.1, BASE_Z)
        + pressure_io_bulkheads().translate(BULKHEAD_POS.0, BULKHEAD_POS.1, BASE_Z)
        + reference_gauge_bracket().translate(GAUGE_POS.0, GAUGE_POS.1, BASE_Z)
        + displacement_creep_witness_scale().translate(SCALE_POS.0, SCALE_POS.1, BASE_Z)
        + clamp_rails().translate(CLAMP_POS.0, CLAMP_POS.1, BASE_Z + BAG_TRAY_Z)
        + leak_witness_secondary_containment().translate(
            CONTAINMENT_POS.0,
            CONTAINMENT_POS.1,
            BASE_Z,
        )
        + labeled_surrogate_coupons().translate(COUPON_POS.0, COUPON_POS.1, BASE_Z)
        + tubing_strain_relief_comb().translate(TUBING_POS.0, TUBING_POS.1, BASE_Z)
        + run_evidence_label_plate().translate(EVIDENCE_POS.0, EVIDENCE_POS.1, BASE_Z)
        + robot_service_keepouts()
}

fn secondary_containment_tray() -> Part {
    let deck = centered_cube(
        "reagent_bag_pressure_hold_secondary_containment_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let sump = centered_cube(
        "reagent_bag_pressure_hold_recessed_leak_sump",
        SUMP_X,
        SUMP_Y,
        SUMP_DEPTH + 1.0,
    )
    .translate(0.0, -10.0, BASE_Z - SUMP_DEPTH / 2.0 + 0.5);
    let drain = centered_cylinder(
        "reagent_bag_pressure_hold_front_drain_port_cut",
        DRAIN_PORT_D / 2.0,
        CURB_W + 34.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(0.0, -STATION_Y / 2.0 + 12.0, BASE_Z - 6.0);

    deck - sump - drain - deck_mount_holes() - module_socket_recesses()
        + containment_curbs()
        + deck_datum_targets()
        + deck_leak_flow_ribs()
        + zone_floor_markers()
}

fn containment_curbs() -> Part {
    let front = centered_cube(
        "reagent_bag_pressure_hold_front_secondary_curb",
        STATION_X,
        CURB_W,
        CURB_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + CURB_W / 2.0, BASE_Z + CURB_Z / 2.0);
    let rear = centered_cube(
        "reagent_bag_pressure_hold_rear_secondary_curb",
        STATION_X,
        CURB_W,
        CURB_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - CURB_W / 2.0, BASE_Z + CURB_Z / 2.0);
    let left = centered_cube(
        "reagent_bag_pressure_hold_left_secondary_curb",
        CURB_W,
        STATION_Y,
        CURB_Z,
    )
    .translate(-STATION_X / 2.0 + CURB_W / 2.0, 0.0, BASE_Z + CURB_Z / 2.0);
    let right = centered_cube(
        "reagent_bag_pressure_hold_right_secondary_curb",
        CURB_W,
        STATION_Y,
        CURB_Z,
    )
    .translate(STATION_X / 2.0 - CURB_W / 2.0, 0.0, BASE_Z + CURB_Z / 2.0);
    front + rear + left + right
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("reagent_bag_pressure_hold_deck_mount_holes");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 60.0, -STATION_Y / 2.0 + 58.0),
        (STATION_X / 2.0 - 60.0, -STATION_Y / 2.0 + 58.0),
        (-STATION_X / 2.0 + 60.0, STATION_Y / 2.0 - 58.0),
        (STATION_X / 2.0 - 60.0, STATION_Y / 2.0 - 58.0),
        (0.0, -STATION_Y / 2.0 + 58.0),
        (0.0, STATION_Y / 2.0 - 58.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("reagent_bag_pressure_hold_m6_mount_hole_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 4.0,
                28,
            )
            .translate(*x, *y, BASE_Z / 2.0);
    }
    holes
}

fn module_socket_recesses() -> Part {
    let mut sockets = Part::empty("reagent_bag_pressure_hold_module_socket_recesses");
    for rect in module_rects() {
        sockets = sockets
            + centered_cube(
                format!("reagent_bag_pressure_hold_{}_socket_recess", rect.name),
                rect.x + 20.0,
                rect.y + 18.0,
                SUMP_DEPTH + 1.0,
            )
            .translate(
                rect.center.0,
                rect.center.1,
                BASE_Z - SUMP_DEPTH / 2.0 + 0.5,
            );
    }
    sockets
}

fn deck_datum_targets() -> Part {
    let mut targets = Part::empty("reagent_bag_pressure_hold_datum_targets");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 102.0, -STATION_Y / 2.0 + 100.0),
        (STATION_X / 2.0 - 102.0, -STATION_Y / 2.0 + 100.0),
        (-STATION_X / 2.0 + 102.0, STATION_Y / 2.0 - 100.0),
        (STATION_X / 2.0 - 102.0, STATION_Y / 2.0 - 100.0),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cylinder(
            format!("reagent_bag_pressure_hold_datum_boss_{i}"),
            18.0,
            7.0,
            36,
        )
        .translate(*x, *y, BASE_Z + 3.5);
        let dot = centered_cylinder(
            format!("reagent_bag_pressure_hold_datum_center_cut_{i}"),
            4.0,
            8.0,
            24,
        )
        .translate(*x, *y, BASE_Z + 4.0);
        targets = targets + (boss - dot);
    }
    targets
}

fn deck_leak_flow_ribs() -> Part {
    let mut ribs = Part::empty("reagent_bag_pressure_hold_deck_leak_flow_ribs");
    for i in 0..LEAK_WITNESS_RIBS {
        let y = centered_index(i, LEAK_WITNESS_RIBS, 72.0) - 18.0;
        ribs = ribs
            + centered_cube(
                format!("reagent_bag_pressure_hold_sump_slope_witness_rib_{i}"),
                SUMP_X - 120.0,
                6.0,
                5.0,
            )
            .translate(0.0, y, BASE_Z + 2.5);
    }
    ribs
}

fn zone_floor_markers() -> Part {
    let mut markers = Part::empty("reagent_bag_pressure_hold_floor_zone_markers");
    for rect in module_rects() {
        markers = markers
            + centered_cube(
                format!("reagent_bag_pressure_hold_{}_floor_marker", rect.name),
                rect.x + 34.0,
                rect.y + 30.0,
                3.0,
            )
            .translate(rect.center.0, rect.center.1, BASE_Z + 1.5);
    }
    markers
}

fn guarded_bag_tray() -> Part {
    let body = centered_cube(
        "reagent_bag_pressure_hold_guarded_bag_tray_body",
        BAG_TRAY_X,
        BAG_TRAY_Y,
        BAG_TRAY_Z,
    )
    .translate(0.0, 0.0, BAG_TRAY_Z / 2.0);
    let bag_recess = centered_cube(
        "reagent_bag_pressure_hold_sealed_bag_surrogate_recess",
        BAG_RECESS_X,
        BAG_RECESS_Y,
        BAG_RECESS_DEPTH + 1.0,
    )
    .translate(0.0, 0.0, BAG_TRAY_Z - BAG_RECESS_DEPTH / 2.0 + 0.5);
    let finger_clearance = centered_cube(
        "reagent_bag_pressure_hold_bag_finger_clearance",
        84.0,
        44.0,
        BAG_RECESS_DEPTH + 2.0,
    )
    .translate(0.0, -BAG_RECESS_Y / 2.0 - 6.0, BAG_TRAY_Z - 8.0);

    body - bag_recess - finger_clearance
        + bag_guard_rails()
        + bag_support_ribs()
        + bag_drain_slots()
        + bag_locator_pins()
        + bag_pressure_pad_land()
}

fn bag_guard_rails() -> Part {
    let front = centered_cube(
        "reagent_bag_pressure_hold_front_splash_guard",
        BAG_TRAY_X,
        BAG_GUARD_RAIL_W,
        BAG_GUARD_Z,
    )
    .translate(
        0.0,
        -BAG_TRAY_Y / 2.0 + BAG_GUARD_RAIL_W / 2.0,
        BAG_TRAY_Z + BAG_GUARD_Z / 2.0,
    );
    let rear = centered_cube(
        "reagent_bag_pressure_hold_rear_splash_guard",
        BAG_TRAY_X,
        BAG_GUARD_RAIL_W,
        BAG_GUARD_Z,
    )
    .translate(
        0.0,
        BAG_TRAY_Y / 2.0 - BAG_GUARD_RAIL_W / 2.0,
        BAG_TRAY_Z + BAG_GUARD_Z / 2.0,
    );
    let left = centered_cube(
        "reagent_bag_pressure_hold_left_splash_guard",
        BAG_GUARD_RAIL_W,
        BAG_TRAY_Y,
        BAG_GUARD_Z,
    )
    .translate(
        -BAG_TRAY_X / 2.0 + BAG_GUARD_RAIL_W / 2.0,
        0.0,
        BAG_TRAY_Z + BAG_GUARD_Z / 2.0,
    );
    let right = centered_cube(
        "reagent_bag_pressure_hold_right_splash_guard",
        BAG_GUARD_RAIL_W,
        BAG_TRAY_Y,
        BAG_GUARD_Z,
    )
    .translate(
        BAG_TRAY_X / 2.0 - BAG_GUARD_RAIL_W / 2.0,
        0.0,
        BAG_TRAY_Z + BAG_GUARD_Z / 2.0,
    );
    let rear_low_hose_notch = centered_cube(
        "reagent_bag_pressure_hold_rear_guard_hose_pass_window",
        150.0,
        BAG_GUARD_RAIL_W + 4.0,
        58.0,
    )
    .translate(
        0.0,
        BAG_TRAY_Y / 2.0 - BAG_GUARD_RAIL_W / 2.0,
        BAG_TRAY_Z + 29.0,
    );
    front + (rear - rear_low_hose_notch) + left + right
}

fn bag_support_ribs() -> Part {
    let mut ribs = Part::empty("reagent_bag_pressure_hold_bag_saddle_support_ribs");
    for i in 0..BAG_SADDLE_RIBS {
        let x = centered_index(i, BAG_SADDLE_RIBS, 50.0);
        ribs = ribs
            + centered_cube(
                format!("reagent_bag_pressure_hold_bag_saddle_rib_{i}"),
                12.0,
                BAG_RECESS_Y - 42.0,
                11.0,
            )
            .translate(x, 0.0, BAG_TRAY_Z + 5.5);
    }
    ribs
}

fn bag_drain_slots() -> Part {
    let mut slots = Part::empty("reagent_bag_pressure_hold_bag_tray_drain_slot_markers");
    for i in 0..BAG_DRAIN_SLOTS {
        let x = centered_index(i, BAG_DRAIN_SLOTS, 64.0);
        slots = slots
            + centered_cube(
                format!("reagent_bag_pressure_hold_bag_tray_drain_slot_{i}"),
                36.0,
                8.0,
                8.0,
            )
            .translate(x, -BAG_RECESS_Y / 2.0 + 22.0, BAG_TRAY_Z + 4.0);
    }
    slots
}

fn bag_locator_pins() -> Part {
    let mut pins = Part::empty("reagent_bag_pressure_hold_bag_locator_pins");
    for i in 0..BAG_LOCATOR_PINS {
        let side = if i % 2 == 0 { -1.0 } else { 1.0 };
        let x = centered_index(i / 2, BAG_LOCATOR_PINS / 2, 112.0);
        let y = side * (BAG_RECESS_Y / 2.0 + 16.0);
        pins = pins
            + centered_cylinder(
                format!("reagent_bag_pressure_hold_bag_locator_pin_{i}"),
                6.0,
                20.0,
                28,
            )
            .translate(x, y, BAG_TRAY_Z + 10.0);
    }
    pins
}

fn bag_pressure_pad_land() -> Part {
    let pad = centered_cube(
        "reagent_bag_pressure_hold_reference_pressure_pad_shadow",
        BAG_PRESSURE_PAD_X,
        BAG_PRESSURE_PAD_Y,
        5.0,
    )
    .translate(0.0, 0.0, BAG_TRAY_Z + 2.5);
    let centerline = centered_cube(
        "reagent_bag_pressure_hold_bag_centerline_witness",
        8.0,
        BAG_PRESSURE_PAD_Y + 30.0,
        8.0,
    )
    .translate(0.0, 0.0, BAG_TRAY_Z + 4.0);
    pad + centerline
}

fn pressure_io_bulkheads() -> Part {
    let panel = centered_cube(
        "reagent_bag_pressure_hold_pressure_io_bulkhead_panel",
        BULKHEAD_PANEL_X,
        BULKHEAD_PANEL_Y,
        BULKHEAD_PANEL_Z,
    )
    .translate(0.0, 0.0, BULKHEAD_PANEL_Z / 2.0);
    let foot = centered_cube(
        "reagent_bag_pressure_hold_bulkhead_mounting_foot",
        BULKHEAD_PANEL_X + 54.0,
        BULKHEAD_PANEL_Y + 42.0,
        BULKHEAD_FOOT_Z,
    )
    .translate(0.0, 0.0, BULKHEAD_FOOT_Z / 2.0);

    panel + foot - pressure_port_cutouts() - pressure_keyway_cutouts()
        + pressure_port_collars()
        + bulkhead_family_labels()
        + bulkhead_isolation_walls()
}

fn pressure_port_cutouts() -> Part {
    let mut cutouts = Part::empty("reagent_bag_pressure_hold_pressure_port_cutouts");
    for channel in 0..PRESSURE_CHANNELS {
        let x = pressure_channel_x(channel);
        cutouts = cutouts
            + port_cutout(format!(
                "reagent_bag_pressure_hold_input_port_cut_{channel}"
            ))
            .translate(x, -PORT_ROW_OFFSET_Y, PORT_INPUT_Z)
            + port_cutout(format!(
                "reagent_bag_pressure_hold_output_port_cut_{channel}"
            ))
            .translate(x, PORT_ROW_OFFSET_Y, PORT_OUTPUT_Z);
    }
    cutouts
}

fn port_cutout(name: String) -> Part {
    centered_cylinder(name, PRESSURE_PORT_D / 2.0, BULKHEAD_PANEL_Y + 14.0, 36)
        .rotate(90.0, 0.0, 0.0)
}

fn pressure_keyway_cutouts() -> Part {
    let mut keyways = Part::empty("reagent_bag_pressure_hold_bulkhead_keyway_cutouts");
    for channel in 0..PRESSURE_CHANNELS {
        let x = pressure_channel_x(channel);
        keyways = keyways
            + centered_cube(
                format!("reagent_bag_pressure_hold_input_keyway_cut_{channel}"),
                BULKHEAD_KEY_W,
                BULKHEAD_PANEL_Y + 16.0,
                BULKHEAD_KEY_Z,
            )
            .translate(
                x + 15.0,
                -PORT_ROW_OFFSET_Y,
                PORT_INPUT_Z + PRESSURE_PORT_D / 2.0 + 7.0,
            )
            + centered_cube(
                format!("reagent_bag_pressure_hold_output_keyway_cut_{channel}"),
                BULKHEAD_KEY_W,
                BULKHEAD_PANEL_Y + 16.0,
                BULKHEAD_KEY_Z,
            )
            .translate(
                x - 15.0,
                PORT_ROW_OFFSET_Y,
                PORT_OUTPUT_Z - PRESSURE_PORT_D / 2.0 - 7.0,
            );
    }
    keyways
}

fn pressure_port_collars() -> Part {
    let mut collars = Part::empty("reagent_bag_pressure_hold_pressure_port_collars");
    for channel in 0..PRESSURE_CHANNELS {
        let x = pressure_channel_x(channel);
        collars = collars
            + port_collar(format!(
                "reagent_bag_pressure_hold_input_green_collar_{channel}"
            ))
            .translate(x, -BULKHEAD_PANEL_Y / 2.0 - 4.0, PORT_INPUT_Z)
            + port_collar(format!(
                "reagent_bag_pressure_hold_output_blue_collar_{channel}"
            ))
            .translate(x, BULKHEAD_PANEL_Y / 2.0 + 4.0, PORT_OUTPUT_Z);
    }
    collars
}

fn port_collar(name: String) -> Part {
    let outer =
        centered_cylinder(name.clone(), PRESSURE_COLLAR_D / 2.0, 8.0, 36).rotate(90.0, 0.0, 0.0);
    let inner = centered_cylinder(
        format!("{name}_clearance_bore"),
        PRESSURE_PORT_D / 2.0 + 2.0,
        10.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0);
    outer - inner
}

fn bulkhead_family_labels() -> Part {
    let input = centered_cube(
        "reagent_bag_pressure_hold_input_bulkhead_label_land",
        BULKHEAD_PANEL_X - 80.0,
        8.0,
        22.0,
    )
    .translate(0.0, -BULKHEAD_PANEL_Y / 2.0 - 5.0, PORT_INPUT_Z + 42.0);
    let output = centered_cube(
        "reagent_bag_pressure_hold_output_bulkhead_label_land",
        BULKHEAD_PANEL_X - 80.0,
        8.0,
        22.0,
    )
    .translate(0.0, BULKHEAD_PANEL_Y / 2.0 + 5.0, PORT_OUTPUT_Z - 42.0);
    input + output
}

fn bulkhead_isolation_walls() -> Part {
    let mut walls = Part::empty("reagent_bag_pressure_hold_bulkhead_channel_isolation_walls");
    for i in 0..PRESSURE_CHANNELS - 1 {
        let x = (pressure_channel_x(i) + pressure_channel_x(i + 1)) / 2.0;
        walls = walls
            + centered_cube(
                format!("reagent_bag_pressure_hold_bulkhead_channel_divider_{i}"),
                10.0,
                BULKHEAD_PANEL_Y + 26.0,
                BULKHEAD_PANEL_Z - 46.0,
            )
            .translate(x, 0.0, BULKHEAD_PANEL_Z / 2.0 + 8.0);
    }
    walls
}

fn reference_gauge_bracket() -> Part {
    let base = centered_cube(
        "reagent_bag_pressure_hold_reference_gauge_base",
        GAUGE_BASE_X,
        GAUGE_BASE_Y,
        GAUGE_BASE_Z,
    )
    .translate(0.0, 0.0, GAUGE_BASE_Z / 2.0);
    let upright = centered_cube(
        "reagent_bag_pressure_hold_reference_gauge_upright",
        GAUGE_UPRIGHT_X,
        GAUGE_UPRIGHT_Y,
        GAUGE_UPRIGHT_Z,
    )
    .translate(
        0.0,
        GAUGE_BASE_Y / 2.0 - GAUGE_UPRIGHT_Y / 2.0,
        GAUGE_BASE_Z + GAUGE_UPRIGHT_Z / 2.0,
    );
    let ring = gauge_ring().translate(0.0, GAUGE_BASE_Y / 2.0 + 4.0, GAUGE_BASE_Z + 154.0);
    let foot_gussets = gauge_gussets();

    base + upright + ring + gauge_tick_marks() + gauge_snubber_pockets() + foot_gussets
}

fn gauge_ring() -> Part {
    let outer = centered_cylinder(
        "reagent_bag_pressure_hold_reference_gauge_ring_outer",
        GAUGE_RING_OD / 2.0,
        14.0,
        64,
    )
    .rotate(90.0, 0.0, 0.0);
    let inner = centered_cylinder(
        "reagent_bag_pressure_hold_reference_gauge_ring_inner_clearance",
        GAUGE_RING_ID / 2.0,
        18.0,
        64,
    )
    .rotate(90.0, 0.0, 0.0);
    outer - inner
}

fn gauge_tick_marks() -> Part {
    let mut ticks = Part::empty("reagent_bag_pressure_hold_reference_gauge_dial_ticks");
    for i in 0..GAUGE_TICK_COUNT {
        let angle = -150.0 + i as f64 * (300.0 / (GAUGE_TICK_COUNT - 1) as f64);
        let (x, z) = polar_xz(angle, GAUGE_RING_OD / 2.0 + 9.0);
        let length = if i % 3 == 0 { 22.0 } else { 13.0 };
        ticks = ticks
            + centered_cube(
                format!("reagent_bag_pressure_hold_reference_gauge_tick_{i}"),
                5.0,
                8.0,
                length,
            )
            .rotate(0.0, angle, 0.0)
            .translate(x, GAUGE_BASE_Y / 2.0 + 12.0, GAUGE_BASE_Z + 154.0 + z);
    }
    ticks
}

fn gauge_snubber_pockets() -> Part {
    let mut pockets = Part::empty("reagent_bag_pressure_hold_gauge_snubber_reference_pockets");
    for i in 0..GAUGE_SNUBBER_COUNT {
        let x = centered_index(i, GAUGE_SNUBBER_COUNT, 58.0);
        let land = centered_cube(
            format!("reagent_bag_pressure_hold_gauge_snubber_pocket_{i}"),
            42.0,
            36.0,
            16.0,
        )
        .translate(x, -GAUGE_BASE_Y / 2.0 + 34.0, GAUGE_BASE_Z + 8.0);
        let bore = centered_cylinder(
            format!("reagent_bag_pressure_hold_gauge_snubber_pocket_bore_{i}"),
            8.0,
            18.0,
            24,
        )
        .translate(x, -GAUGE_BASE_Y / 2.0 + 34.0, GAUGE_BASE_Z + 9.0);
        pockets = pockets + (land - bore);
    }
    pockets
}

fn gauge_gussets() -> Part {
    let left = centered_cube(
        "reagent_bag_pressure_hold_reference_gauge_left_gusset",
        22.0,
        110.0,
        118.0,
    )
    .translate(-GAUGE_UPRIGHT_X / 2.0 + 18.0, 18.0, GAUGE_BASE_Z + 59.0);
    let right = centered_cube(
        "reagent_bag_pressure_hold_reference_gauge_right_gusset",
        22.0,
        110.0,
        118.0,
    )
    .translate(GAUGE_UPRIGHT_X / 2.0 - 18.0, 18.0, GAUGE_BASE_Z + 59.0);
    left + right
}

fn displacement_creep_witness_scale() -> Part {
    let base = centered_cube(
        "reagent_bag_pressure_hold_creep_scale_base",
        SCALE_BASE_X,
        SCALE_BASE_Y,
        SCALE_BASE_Z,
    )
    .translate(0.0, 0.0, SCALE_BASE_Z / 2.0);
    let tower = centered_cube(
        "reagent_bag_pressure_hold_displacement_witness_scale_tower",
        SCALE_TOWER_X,
        SCALE_TOWER_Y,
        SCALE_TOWER_Z,
    )
    .translate(0.0, 0.0, SCALE_BASE_Z + SCALE_TOWER_Z / 2.0);
    let slot = centered_cube(
        "reagent_bag_pressure_hold_scale_pointer_travel_window",
        SCALE_TOWER_X + 4.0,
        SCALE_TOWER_Y + 6.0,
        CREEP_TRAVEL_Z,
    )
    .translate(0.0, 0.0, SCALE_BASE_Z + SCALE_TOWER_Z / 2.0);

    base + (tower - slot) + scale_tick_marks() + creep_pointer_carriages() + bag_contact_anvil()
}

fn scale_tick_marks() -> Part {
    let mut ticks = Part::empty("reagent_bag_pressure_hold_creep_scale_ticks");
    for i in 0..SCALE_TICKS {
        let z = SCALE_BASE_Z + 50.0 + i as f64 * 12.5;
        let major = i % SCALE_MAJOR_EVERY == 0;
        ticks = ticks
            + centered_cube(
                format!("reagent_bag_pressure_hold_creep_scale_tick_{i}"),
                if major { 42.0 } else { 24.0 },
                5.0,
                3.0,
            )
            .translate(
                SCALE_TOWER_X / 2.0 + if major { 20.0 } else { 11.0 },
                -SCALE_TOWER_Y / 2.0 - 3.0,
                z,
            );
    }
    ticks
}

fn creep_pointer_carriages() -> Part {
    let mut pointers = Part::empty("reagent_bag_pressure_hold_creep_pointer_carriages");
    for i in 0..CREEP_POINTER_COUNT {
        let z = SCALE_BASE_Z + 92.0 + i as f64 * CREEP_POINTER_PITCH_Z;
        let carriage = centered_cube(
            format!("reagent_bag_pressure_hold_creep_pointer_carriage_{i}"),
            86.0,
            24.0,
            16.0,
        )
        .translate(34.0, -SCALE_TOWER_Y / 2.0 - 18.0, z);
        let pointer = centered_cube(
            format!("reagent_bag_pressure_hold_creep_pointer_knife_edge_{i}"),
            72.0,
            8.0,
            4.0,
        )
        .translate(-34.0, -SCALE_TOWER_Y / 2.0 - 30.0, z);
        pointers = pointers + carriage + pointer;
    }
    pointers
}

fn bag_contact_anvil() -> Part {
    let arm = centered_cube(
        "reagent_bag_pressure_hold_creep_scale_bag_contact_arm",
        138.0,
        18.0,
        20.0,
    )
    .translate(92.0, -SCALE_TOWER_Y / 2.0 - 14.0, SCALE_BASE_Z + 126.0);
    let pad = centered_cube(
        "reagent_bag_pressure_hold_creep_scale_soft_contact_pad_shadow",
        42.0,
        28.0,
        34.0,
    )
    .translate(172.0, -SCALE_TOWER_Y / 2.0 - 18.0, SCALE_BASE_Z + 126.0);
    arm + pad
}

fn clamp_rails() -> Part {
    let front_rail = centered_cube(
        "reagent_bag_pressure_hold_front_clamp_rail",
        CLAMP_RAIL_X,
        CLAMP_RAIL_Y,
        CLAMP_RAIL_Z,
    )
    .translate(0.0, -CLAMP_RAIL_OFFSET_Y / 2.0, CLAMP_RAIL_Z / 2.0);
    let rear_rail = centered_cube(
        "reagent_bag_pressure_hold_rear_clamp_rail",
        CLAMP_RAIL_X,
        CLAMP_RAIL_Y,
        CLAMP_RAIL_Z,
    )
    .translate(0.0, CLAMP_RAIL_OFFSET_Y / 2.0, CLAMP_RAIL_Z / 2.0);
    front_rail + rear_rail + clamp_shoes() + clamp_force_flags()
}

fn clamp_shoes() -> Part {
    let mut shoes = Part::empty("reagent_bag_pressure_hold_clamp_shoes");
    for i in 0..CLAMP_SHOE_COUNT {
        let x = centered_index(i, CLAMP_SHOE_COUNT, 84.0);
        let y = if i % 2 == 0 {
            -CLAMP_RAIL_OFFSET_Y / 2.0
        } else {
            CLAMP_RAIL_OFFSET_Y / 2.0
        };
        let shoe = centered_cube(
            format!("reagent_bag_pressure_hold_clamp_shoe_{i}"),
            CLAMP_SHOE_X,
            CLAMP_SHOE_Y,
            CLAMP_SHOE_Z,
        )
        .translate(x, y, CLAMP_RAIL_Z + CLAMP_SHOE_Z / 2.0);
        let knob = centered_cylinder(
            format!("reagent_bag_pressure_hold_clamp_knob_{i}"),
            CLAMP_KNOB_D / 2.0,
            18.0,
            32,
        )
        .translate(x, y, CLAMP_RAIL_Z + CLAMP_SHOE_Z + 9.0);
        let pad = centered_cube(
            format!("reagent_bag_pressure_hold_clamp_soft_pad_land_{i}"),
            CLAMP_SHOE_X - 14.0,
            8.0,
            8.0,
        )
        .translate(
            x,
            y.signum() * (BAG_RECESS_Y / 2.0 + 6.0),
            CLAMP_RAIL_Z + 4.0,
        );
        shoes = shoes + shoe + knob + pad;
    }
    shoes
}

fn clamp_force_flags() -> Part {
    let mut flags = Part::empty("reagent_bag_pressure_hold_clamp_force_flag_markers");
    for i in 0..CLAMP_FORCE_FLAG_COUNT {
        let x = centered_index(i, CLAMP_FORCE_FLAG_COUNT, 82.0);
        flags = flags
            + centered_cube(
                format!("reagent_bag_pressure_hold_clamp_force_flag_{i}"),
                38.0,
                6.0,
                18.0,
            )
            .translate(x, CLAMP_RAIL_OFFSET_Y / 2.0 + 24.0, CLAMP_RAIL_Z + 9.0);
    }
    flags
}

fn leak_witness_secondary_containment() -> Part {
    let tray = centered_cube(
        "reagent_bag_pressure_hold_leak_witness_secondary_tray",
        CONTAINMENT_X,
        CONTAINMENT_Y,
        CONTAINMENT_Z,
    )
    .translate(0.0, 0.0, CONTAINMENT_Z / 2.0);
    let pocket = centered_cube(
        "reagent_bag_pressure_hold_leak_witness_absorbent_basin",
        CONTAINMENT_X - 2.0 * CONTAINMENT_WALL,
        CONTAINMENT_Y - 2.0 * CONTAINMENT_WALL,
        CONTAINMENT_Z - 14.0,
    )
    .translate(0.0, 0.0, CONTAINMENT_Z / 2.0 + 7.0);
    tray - pocket + leak_sensor_wells() + absorbent_pad_lands() + leak_path_comb()
}

fn leak_sensor_wells() -> Part {
    let mut wells = Part::empty("reagent_bag_pressure_hold_leak_sensor_wells");
    for i in 0..LEAK_SENSOR_WELLS {
        let x = centered_index(i % 4, 4, 88.0);
        let y = centered_index(i / 4, 2, 82.0);
        let boss = centered_cylinder(
            format!("reagent_bag_pressure_hold_leak_sensor_well_boss_{i}"),
            17.0,
            11.0,
            32,
        )
        .translate(x, y, CONTAINMENT_Z + 5.5);
        let recess = centered_cylinder(
            format!("reagent_bag_pressure_hold_leak_sensor_well_recess_{i}"),
            9.0,
            12.0,
            28,
        )
        .translate(x, y, CONTAINMENT_Z + 6.0);
        wells = wells + (boss - recess);
    }
    wells
}

fn absorbent_pad_lands() -> Part {
    let mut lands = Part::empty("reagent_bag_pressure_hold_absorbent_pad_lands");
    for i in 0..ABSORBENT_PAD_LANDS {
        let x = centered_index(i, ABSORBENT_PAD_LANDS, 88.0);
        lands = lands
            + centered_cube(
                format!("reagent_bag_pressure_hold_absorbent_pad_land_{i}"),
                66.0,
                38.0,
                5.0,
            )
            .translate(x, -CONTAINMENT_Y / 2.0 + 44.0, CONTAINMENT_Z + 2.5);
    }
    lands
}

fn leak_path_comb() -> Part {
    let mut ribs = Part::empty("reagent_bag_pressure_hold_leak_path_comb_ribs");
    for i in 0..LEAK_WITNESS_RIBS {
        ribs = ribs
            + centered_cube(
                format!("reagent_bag_pressure_hold_leak_path_comb_rib_{i}"),
                7.0,
                CONTAINMENT_Y - 54.0,
                8.0,
            )
            .translate(
                centered_index(i, LEAK_WITNESS_RIBS, 52.0),
                0.0,
                CONTAINMENT_Z + 4.0,
            );
    }
    ribs
}

fn labeled_surrogate_coupons() -> Part {
    let base = centered_cube(
        "reagent_bag_pressure_hold_surrogate_coupon_rack_base",
        COUPON_BLOCK_X,
        COUPON_BLOCK_Y,
        COUPON_BLOCK_Z,
    )
    .translate(0.0, 0.0, COUPON_BLOCK_Z / 2.0);
    base - coupon_recesses() + coupon_set() + coupon_rack_label_spine()
}

fn coupon_recesses() -> Part {
    let mut recesses = Part::empty("reagent_bag_pressure_hold_surrogate_coupon_recesses");
    for i in 0..SURROGATE_COUPONS {
        let (x, y) = coupon_position(i);
        recesses = recesses
            + centered_cube(
                format!("reagent_bag_pressure_hold_surrogate_coupon_recess_{i}"),
                COUPON_X + 8.0,
                COUPON_Y + 8.0,
                COUPON_BLOCK_Z + 4.0,
            )
            .translate(x, y, COUPON_BLOCK_Z / 2.0);
    }
    recesses
}

fn coupon_set() -> Part {
    let mut coupons = Part::empty("reagent_bag_pressure_hold_labeled_surrogate_coupons");
    for i in 0..SURROGATE_COUPONS {
        let (x, y) = coupon_position(i);
        coupons = coupons + surrogate_coupon(i).translate(x, y, COUPON_BLOCK_Z + COUPON_Z / 2.0);
    }
    coupons
}

fn surrogate_coupon(index: usize) -> Part {
    let body = centered_cube(
        format!("reagent_bag_pressure_hold_coupon_{index}_body"),
        COUPON_X,
        COUPON_Y,
        COUPON_Z,
    );
    let label = centered_cube(
        format!("reagent_bag_pressure_hold_coupon_{index}_label_land"),
        COUPON_X - 22.0,
        14.0,
        4.0,
    )
    .translate(0.0, COUPON_Y / 2.0 - 12.0, COUPON_Z / 2.0 + 2.0);
    let weld_seam = centered_cube(
        format!("reagent_bag_pressure_hold_coupon_{index}_weld_seam_witness"),
        COUPON_X - 18.0,
        6.0,
        5.0,
    )
    .translate(0.0, -4.0, COUPON_Z / 2.0 + 2.5);
    let connector_boss = centered_cylinder(
        format!("reagent_bag_pressure_hold_coupon_{index}_connector_boss"),
        9.0,
        7.0,
        28,
    )
    .translate(
        COUPON_X / 2.0 - 18.0,
        -COUPON_Y / 2.0 + 18.0,
        COUPON_Z / 2.0 + 3.5,
    );
    body + label + weld_seam + connector_boss + coupon_barcode_stripes(index)
}

fn coupon_barcode_stripes(index: usize) -> Part {
    let mut stripes = Part::empty(format!(
        "reagent_bag_pressure_hold_coupon_{index}_barcode_stripes"
    ));
    for stripe in 0..COUPON_BARCODE_STRIPES {
        stripes = stripes
            + centered_cube(
                format!("reagent_bag_pressure_hold_coupon_{index}_barcode_stripe_{stripe}"),
                4.0,
                18.0,
                5.0,
            )
            .translate(
                -COUPON_X / 2.0 + 16.0 + stripe as f64 * 7.0,
                -COUPON_Y / 2.0 + 18.0,
                COUPON_Z / 2.0 + 2.5,
            );
    }
    stripes
}

fn coupon_rack_label_spine() -> Part {
    let spine = centered_cube(
        "reagent_bag_pressure_hold_coupon_rack_material_lot_label_spine",
        COUPON_BLOCK_X - 40.0,
        18.0,
        10.0,
    )
    .translate(0.0, COUPON_BLOCK_Y / 2.0 - 22.0, COUPON_BLOCK_Z + 5.0);
    let witness = centered_cube(
        "reagent_bag_pressure_hold_coupon_rack_release_hold_reject_witness_lane",
        COUPON_BLOCK_X - 98.0,
        10.0,
        7.0,
    )
    .translate(0.0, -COUPON_BLOCK_Y / 2.0 + 22.0, COUPON_BLOCK_Z + 3.5);
    spine + witness
}

fn tubing_strain_relief_comb() -> Part {
    let body = centered_cube(
        "reagent_bag_pressure_hold_tubing_strain_relief_comb_body",
        TUBE_COMB_X,
        TUBE_COMB_Y,
        TUBE_COMB_Z,
    )
    .translate(0.0, 0.0, TUBE_COMB_Z / 2.0);
    let channels = tube_channel_cutouts();
    let flags = tube_identity_flags();
    body - channels + flags
}

fn tube_channel_cutouts() -> Part {
    let mut channels = Part::empty("reagent_bag_pressure_hold_tube_channel_cutouts");
    for i in 0..TUBE_CHANNELS {
        let x = centered_index(i, TUBE_CHANNELS, TUBE_CHANNEL_PITCH_X);
        channels = channels
            + centered_cylinder(
                format!("reagent_bag_pressure_hold_tube_channel_cutout_{i}"),
                TUBE_CHANNEL_D / 2.0,
                TUBE_COMB_Y + 8.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, TUBE_COMB_Z / 2.0);
    }
    channels
}

fn tube_identity_flags() -> Part {
    let mut flags = Part::empty("reagent_bag_pressure_hold_tube_identity_flags");
    for i in 0..TUBE_CHANNELS {
        let x = centered_index(i, TUBE_CHANNELS, TUBE_CHANNEL_PITCH_X);
        flags = flags
            + centered_cube(
                format!("reagent_bag_pressure_hold_tube_identity_flag_{i}"),
                24.0,
                6.0,
                16.0,
            )
            .translate(x, TUBE_COMB_Y / 2.0 + 7.0, TUBE_COMB_Z + 8.0);
    }
    flags
}

fn run_evidence_label_plate() -> Part {
    let plate = centered_cube(
        "reagent_bag_pressure_hold_run_evidence_label_plate",
        EVIDENCE_X,
        EVIDENCE_Y,
        EVIDENCE_Z,
    )
    .translate(0.0, 0.0, EVIDENCE_Z / 2.0);
    plate + run_label_lands() + disposition_lanes() + evidence_token_slots()
}

fn run_label_lands() -> Part {
    let mut lands = Part::empty("reagent_bag_pressure_hold_run_label_lands");
    for i in 0..RUN_LABEL_LANDS {
        let x = centered_index(i, RUN_LABEL_LANDS, 72.0);
        lands = lands
            + centered_cube(
                format!("reagent_bag_pressure_hold_run_label_land_{i}"),
                56.0,
                24.0,
                5.0,
            )
            .translate(x, 22.0, EVIDENCE_Z + 2.5);
    }
    lands
}

fn disposition_lanes() -> Part {
    let mut lanes = Part::empty("reagent_bag_pressure_hold_release_hold_reject_lanes");
    for lane in 0..DISPOSITION_LANES {
        let x = centered_index(lane, DISPOSITION_LANES, 126.0);
        lanes = lanes
            + centered_cube(
                format!("reagent_bag_pressure_hold_disposition_lane_{lane}"),
                98.0,
                18.0,
                7.0,
            )
            .translate(x, -22.0, EVIDENCE_Z + 3.5);
    }
    lanes
}

fn evidence_token_slots() -> Part {
    let mut slots = Part::empty("reagent_bag_pressure_hold_evidence_token_slots");
    for i in 0..EVIDENCE_TOKEN_SLOTS {
        let x = centered_index(i, EVIDENCE_TOKEN_SLOTS, 44.0);
        slots = slots
            + centered_cube(
                format!("reagent_bag_pressure_hold_evidence_token_slot_{i}"),
                28.0,
                12.0,
                4.0,
            )
            .translate(x, -EVIDENCE_Y / 2.0 + 12.0, EVIDENCE_Z + 2.0);
    }
    slots
}

fn robot_service_keepouts() -> Part {
    let front_robot = centered_cube(
        "reagent_bag_pressure_hold_front_robot_approach_keepout",
        STATION_X - 2.0 * CURB_W,
        FRONT_ROBOT_APPROACH,
        KEEP_OUT_RAIL,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 - FRONT_ROBOT_APPROACH / 2.0,
        BASE_Z + KEEP_OUT_RAIL / 2.0,
    );
    let rear_service = centered_cube(
        "reagent_bag_pressure_hold_rear_pressure_service_keepout",
        BULKHEAD_PANEL_X + 160.0,
        REAR_PRESSURE_SERVICE,
        KEEP_OUT_RAIL,
    )
    .translate(
        BULKHEAD_POS.0,
        STATION_Y / 2.0 + REAR_PRESSURE_SERVICE / 2.0,
        BASE_Z + KEEP_OUT_RAIL / 2.0,
    );
    let left_scale_service = centered_cube(
        "reagent_bag_pressure_hold_left_scale_service_keepout",
        LEFT_SCALE_SERVICE,
        SCALE_BASE_Y + 120.0,
        KEEP_OUT_RAIL,
    )
    .translate(
        -STATION_X / 2.0 - LEFT_SCALE_SERVICE / 2.0,
        SCALE_POS.1,
        BASE_Z + KEEP_OUT_RAIL / 2.0,
    );
    let right_gauge_service = centered_cube(
        "reagent_bag_pressure_hold_right_gauge_service_keepout",
        RIGHT_GAUGE_SERVICE,
        GAUGE_BASE_Y + 140.0,
        KEEP_OUT_RAIL,
    )
    .translate(
        STATION_X / 2.0 + RIGHT_GAUGE_SERVICE / 2.0,
        GAUGE_POS.1,
        BASE_Z + KEEP_OUT_RAIL / 2.0,
    );
    let top_creep = centered_cube(
        "reagent_bag_pressure_hold_top_bag_creep_clearance_keepout",
        BAG_TRAY_X + 160.0,
        BAG_TRAY_Y + 120.0,
        TOP_BAG_CREEP_CLEARANCE,
    )
    .translate(
        BAG_POS.0,
        BAG_POS.1,
        BASE_Z + BAG_TRAY_Z + TOP_BAG_CREEP_CLEARANCE / 2.0,
    );
    front_robot + rear_service + left_scale_service + right_gauge_service + top_creep
}

fn module_rects() -> [Rect; 8] {
    [
        Rect {
            name: "guarded_bag_tray",
            center: BAG_POS,
            x: BAG_TRAY_X,
            y: BAG_TRAY_Y,
        },
        Rect {
            name: "pressure_io_bulkheads",
            center: BULKHEAD_POS,
            x: BULKHEAD_PANEL_X,
            y: 118.0,
        },
        Rect {
            name: "reference_gauge_bracket",
            center: GAUGE_POS,
            x: GAUGE_BASE_X,
            y: GAUGE_BASE_Y,
        },
        Rect {
            name: "creep_witness_scale",
            center: SCALE_POS,
            x: SCALE_BASE_X,
            y: SCALE_BASE_Y,
        },
        Rect {
            name: "leak_witness_containment",
            center: CONTAINMENT_POS,
            x: CONTAINMENT_X,
            y: CONTAINMENT_Y,
        },
        Rect {
            name: "surrogate_coupon_rack",
            center: COUPON_POS,
            x: COUPON_BLOCK_X,
            y: COUPON_BLOCK_Y,
        },
        Rect {
            name: "tubing_strain_relief_comb",
            center: TUBING_POS,
            x: TUBE_COMB_X,
            y: TUBE_COMB_Y,
        },
        Rect {
            name: "run_evidence_plate",
            center: EVIDENCE_POS,
            x: EVIDENCE_X,
            y: EVIDENCE_Y,
        },
    ]
}

fn assert_design_constraints() {
    for rect in module_rects() {
        assert!(
            rect.fits_inside_station(),
            "{} exceeds station containment footprint",
            rect.name
        );
    }

    let rects = [
        module_rects()[0],
        module_rects()[1],
        module_rects()[2],
        module_rects()[3],
        module_rects()[4],
        module_rects()[5],
        module_rects()[7],
    ];
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

    assert_eq!(DATUM_TARGETS, 4);
    assert_eq!(PRESSURE_PORT_PAIRS, PRESSURE_CHANNELS);
    assert_eq!(TUBE_CHANNELS, PRESSURE_PORT_PAIRS * 2);
    assert_eq!(COUPON_LABEL_LANDS, SURROGATE_COUPONS);
    assert!(TUBE_CHANNEL_D >= TUBE_OD_MAX + TUBE_CLEARANCE);
    assert!(SCALE_TICKS > CREEP_POINTER_COUNT * SCALE_MAJOR_EVERY);
    assert!(BAG_TRAY_X > BAG_RECESS_X + 2.0 * BAG_GUARD_RAIL_W);
    assert!(CONTAINMENT_X > BAG_RECESS_X + 120.0);
}

fn pressure_channel_x(channel: usize) -> f64 {
    centered_index(channel, PRESSURE_CHANNELS, PORT_PITCH_X)
}

fn coupon_position(index: usize) -> (f64, f64) {
    let rows = SURROGATE_COUPONS.div_ceil(COUPON_COLS);
    let col = index % COUPON_COLS;
    let row = index / COUPON_COLS;
    (
        centered_index(col, COUPON_COLS, COUPON_PITCH_X),
        centered_index(row, rows, COUPON_PITCH_Y),
    )
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn polar_xz(theta_degrees: f64, radius: f64) -> (f64, f64) {
    let radians = theta_degrees.to_radians();
    (radius * radians.cos(), radius * radians.sin())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_and_station_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        assert!(
            OUTPUTS
                .iter()
                .all(|path| path
                    .starts_with("output/closed_reagent_bag_pressure_hold_creep_station_"))
        );
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }

    #[test]
    fn requested_feature_groups_are_explicit() {
        for feature in [
            "guarded_bag_tray",
            "pressure_input_bulkhead",
            "pressure_output_bulkhead",
            "reference_gauge_bracket",
            "displacement_creep_witness_scale",
            "clamp_rails",
            "leak_safe_secondary_containment",
            "labeled_surrogate_coupons",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn layout_zones_fit_and_do_not_conflict() {
        assert_design_constraints();
    }

    #[test]
    fn pressure_hold_interfaces_are_channel_matched() {
        assert_eq!(PRESSURE_CHANNELS, 4);
        assert_eq!(PRESSURE_PORT_PAIRS, PRESSURE_CHANNELS);
        assert_eq!(TUBE_CHANNELS, PRESSURE_PORT_PAIRS * 2);
        assert!(PRESSURE_COLLAR_D > PRESSURE_PORT_D + 14.0);
        assert!(BULKHEAD_PANEL_X > (PRESSURE_CHANNELS - 1) as f64 * PORT_PITCH_X + 100.0);
    }

    #[test]
    fn creep_scale_and_clamp_capacity_are_visible() {
        assert_eq!(CREEP_POINTER_COUNT, 3);
        assert!(SCALE_TICKS >= 20);
        assert_eq!(CLAMP_FORCE_FLAG_COUNT, CLAMP_SHOE_COUNT);
        assert!(TOP_BAG_CREEP_CLEARANCE > SCALE_TOWER_Z);
    }

    #[test]
    fn containment_and_coupon_traceability_are_parametric() {
        assert_eq!(SURROGATE_COUPONS, COUPON_LABEL_LANDS);
        assert_eq!(SURROGATE_COUPONS % COUPON_COLS, 0);
        assert!(LEAK_SENSOR_WELLS >= SURROGATE_COUPONS);
        assert!(ABSORBENT_PAD_LANDS >= BAG_DRAIN_SLOTS);
        assert!(RUN_LABEL_LANDS >= DISPOSITION_LANES + 3);
    }
}
