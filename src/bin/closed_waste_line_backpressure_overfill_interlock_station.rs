use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed waste-line backpressure, overflow, and contamination interlock station.
//
// This generator models a bench validation fixture for the closed cell-culture
// waste path. It packages a bag/bottle nest, secondary containment, vent/filter
// mockup, transducer coupons, occlusion challenge points, overflow sensors,
// check-valve witness geometry, leak guttering, traceability plates, and sample
// custody features. It is mechanical validation packaging only; wetted-path
// materials, sterility claims, acceptance criteria, and actual sensors remain
// external validation controls.

const OUTPUT_PREFIX: &str = "closed_waste_line_backpressure_overfill_interlock_station";

const OUTPUTS: [&str; 12] = [
    "output/closed_waste_line_backpressure_overfill_interlock_station_secondary_containment_deck.stl",
    "output/closed_waste_line_backpressure_overfill_interlock_station_waste_bag_bottle_nest.stl",
    "output/closed_waste_line_backpressure_overfill_interlock_station_hydrophobic_vent_filter_mockup.stl",
    "output/closed_waste_line_backpressure_overfill_interlock_station_pressure_transducer_coupon_bank.stl",
    "output/closed_waste_line_backpressure_overfill_interlock_station_pinch_valve_occlusion_challenge_points.stl",
    "output/closed_waste_line_backpressure_overfill_interlock_station_overflow_float_optical_sensor_brackets.stl",
    "output/closed_waste_line_backpressure_overfill_interlock_station_check_valve_orientation_witness.stl",
    "output/closed_waste_line_backpressure_overfill_interlock_station_leak_gutter_sensor_lane.stl",
    "output/closed_waste_line_backpressure_overfill_interlock_station_barcode_lot_plate.stl",
    "output/closed_waste_line_backpressure_overfill_interlock_station_sampling_custody_panel.stl",
    "output/closed_waste_line_backpressure_overfill_interlock_station_interlock_evidence_bridge.stl",
    "output/closed_waste_line_backpressure_overfill_interlock_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "waste_bag_bottle_nest",
    "secondary_containment_tray",
    "hydrophobic_vent_filter_mockup",
    "pressure_transducer_coupons",
    "pinch_valve_occlusion_challenge_points",
    "overflow_float_optical_sensor_brackets",
    "check_valve_orientation_witness",
    "leak_gutter_sensor_lane",
    "barcode_lot_plate",
    "sampling_custody_panel",
    "interlock_evidence_bridge",
];

const STATION_X: f64 = 1340.0;
const STATION_Y: f64 = 820.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 54.0;
const BASIN_X: f64 = 1190.0;
const BASIN_Y: f64 = 682.0;
const BASIN_DEPTH: f64 = 9.0;
const SOCKET_DEPTH: f64 = 6.0;
const MOUNT_HOLE_D: f64 = 6.6;
const DRAIN_D: f64 = 18.0;
const DATUM_TARGETS: usize = 4;

const NEST_X: f64 = 430.0;
const NEST_Y: f64 = 302.0;
const NEST_Z: f64 = 78.0;
const NEST_POS: (f64, f64) = (-360.0, 118.0);
const WASTE_BOTTLE_D: f64 = 158.0;
const WASTE_BAG_SHADOW_X: f64 = 224.0;
const WASTE_BAG_SHADOW_Y: f64 = 168.0;
const NEST_STRAPS: usize = 3;
const BOTTLE_LOCATORS: usize = 4;

const VENT_X: f64 = 390.0;
const VENT_Y: f64 = 130.0;
const VENT_Z: f64 = 136.0;
const VENT_POS: (f64, f64) = (310.0, 205.0);
const VENT_FILTERS: usize = 3;
const FILTER_OD: f64 = 34.0;
const FILTER_LENGTH: f64 = 154.0;
const VENT_PORT_D: f64 = 10.0;

const PRESSURE_X: f64 = 394.0;
const PRESSURE_Y: f64 = 154.0;
const PRESSURE_Z: f64 = 66.0;
const PRESSURE_POS: (f64, f64) = (338.0, 50.0);
const PRESSURE_COUPONS: usize = 6;
const PRESSURE_COUPON_PITCH: f64 = 58.0;
const TRANSDUCER_BORE_D: f64 = 15.0;
const PRESSURE_TAP_D: f64 = 5.6;

const OCCLUSION_X: f64 = 504.0;
const OCCLUSION_Y: f64 = 154.0;
const OCCLUSION_Z: f64 = 74.0;
const OCCLUSION_POS: (f64, f64) = (-346.0, -210.0);
const OCCLUSION_POINTS: usize = 5;
const OCCLUSION_PITCH: f64 = 88.0;
const PINCH_ROLLER_D: f64 = 28.0;
const TUBE_BORE_D: f64 = 6.4;

const SENSOR_X: f64 = 268.0;
const SENSOR_Y: f64 = 236.0;
const SENSOR_Z: f64 = 152.0;
const SENSOR_POS: (f64, f64) = (-30.0, 238.0);
const FLOAT_BRACKETS: usize = 3;
const OPTICAL_FLAGS: usize = 4;
const FLOAT_STEM_D: f64 = 8.0;

const CHECK_X: f64 = 338.0;
const CHECK_Y: f64 = 128.0;
const CHECK_Z: f64 = 70.0;
const CHECK_POS: (f64, f64) = (340.0, -156.0);
const CHECK_VALVE_POSITIONS: usize = 4;
const CHECK_PITCH: f64 = 76.0;
const CHECK_VALVE_D: f64 = 22.0;

const GUTTER_X: f64 = 1080.0;
const GUTTER_Y: f64 = 76.0;
const GUTTER_Z: f64 = 38.0;
const GUTTER_POS: (f64, f64) = (0.0, -338.0);
const LEAK_SENSOR_WELLS: usize = 8;
const GUTTER_BAFFLES: usize = 7;

const TRACE_X: f64 = 270.0;
const TRACE_Y: f64 = 92.0;
const TRACE_Z: f64 = 12.0;
const TRACE_POS: (f64, f64) = (-505.0, 330.0);
const BARCODE_LANDS: usize = 8;
const LOT_LANDS: usize = 4;

const CUSTODY_X: f64 = 210.0;
const CUSTODY_Y: f64 = 96.0;
const CUSTODY_Z: f64 = 42.0;
const CUSTODY_POS: (f64, f64) = (532.0, 330.0);
const SAMPLE_VIAL_WELLS: usize = 6;
const CUSTODY_SEAL_POINTS: usize = 4;

const EVIDENCE_X: f64 = 1160.0;
const EVIDENCE_Y: f64 = 70.0;
const EVIDENCE_CLEARANCE_Z: f64 = 214.0;
const EVIDENCE_BEAM_Z: f64 = 28.0;
const EVIDENCE_POS: (f64, f64) = (0.0, -12.0);
const EVIDENCE_CAMERAS: usize = 4;
const INTERLOCK_LIGHT_BARS: usize = 3;

const ROUTE_CHANNELS: usize = 7;
const FRONT_OPERATOR_CLEARANCE: f64 = 330.0;
const REAR_VENT_FILTER_CLEARANCE: f64 = 250.0;
const RIGHT_SAMPLE_CLEARANCE: f64 = 210.0;
const TOP_BOTTLE_LIFT_CLEARANCE: f64 = 310.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_station(self) -> bool {
        let usable_x = STATION_X / 2.0 - RIM_W - 12.0;
        let usable_y = STATION_Y / 2.0 - RIM_W - 12.0;

        self.center.0.abs() + self.x / 2.0 <= usable_x
            && self.center.1.abs() + self.y / 2.0 <= usable_y
    }

    fn overlaps(self, other: Rect, gap: f64) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();

        dx < (self.x + other.x) / 2.0 + gap && dy < (self.y + other.y) / 2.0 + gap
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = secondary_containment_deck();
    export(OUTPUTS[0], &deck);

    let nest = waste_bag_bottle_nest();
    export(OUTPUTS[1], &nest);

    let vent = hydrophobic_vent_filter_mockup();
    export(OUTPUTS[2], &vent);

    let pressure = pressure_transducer_coupon_bank();
    export(OUTPUTS[3], &pressure);

    let occlusion = pinch_valve_occlusion_challenge_points();
    export(OUTPUTS[4], &occlusion);

    let overflow = overflow_float_optical_sensor_brackets();
    export(OUTPUTS[5], &overflow);

    let check = check_valve_orientation_witness();
    export(OUTPUTS[6], &check);

    let gutter = leak_gutter_sensor_lane();
    export(OUTPUTS[7], &gutter);

    let trace = barcode_lot_plate();
    export(OUTPUTS[8], &trace);

    let custody = sampling_custody_panel();
    export(OUTPUTS[9], &custody);

    let evidence = interlock_evidence_bridge();
    export(OUTPUTS[10], &evidence);

    let assembly = deck
        + nest
        + vent
        + pressure
        + occlusion
        + overflow
        + check
        + gutter
        + trace
        + custody
        + evidence
        + witnessed_tubing_routes()
        + service_keepout_gauges();
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed waste-line backpressure/overfill/interlock validation station:");
    println!(
        "  Containment deck:       {STATION_X:.0}mm x {STATION_Y:.0}mm with {BASIN_X:.0}mm x {BASIN_Y:.0}mm secondary basin, {LEAK_SENSOR_WELLS} leak wells, {DRAIN_D:.0}mm low-point drain"
    );
    println!(
        "  Waste nest:             {WASTE_BOTTLE_D:.0}mm bottle datum, bag shadow {WASTE_BAG_SHADOW_X:.0}mm x {WASTE_BAG_SHADOW_Y:.0}mm, {NEST_STRAPS} straps, {BOTTLE_LOCATORS} locator bosses"
    );
    println!(
        "  Interlock challenges:   {VENT_FILTERS} vent/filter mockups, {PRESSURE_COUPONS} pressure coupons, {OCCLUSION_POINTS} occlusion points, {FLOAT_BRACKETS} float brackets, {OPTICAL_FLAGS} optical flags"
    );
    println!(
        "  Contamination controls: {CHECK_VALVE_POSITIONS} check-valve witness pockets, {ROUTE_CHANNELS} witnessed waste/vent/sample routes, segregated leak gutter, barcode/lot and custody lands"
    );
    println!(
        "  Evidence/service:       {EVIDENCE_CAMERAS} camera tabs, {INTERLOCK_LIGHT_BARS} light bars, front {FRONT_OPERATOR_CLEARANCE:.0}mm, rear {REAR_VENT_FILTER_CLEARANCE:.0}mm, right {RIGHT_SAMPLE_CLEARANCE:.0}mm, top {TOP_BOTTLE_LIFT_CLEARANCE:.0}mm clearances"
    );
    println!("  Required feature groups: {}", REQUIRED_FEATURES.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 12);
    assert_eq!(REQUIRED_FEATURES.len(), 11);
    assert_eq!(CHECK_VALVE_POSITIONS, 4);
    assert_eq!(DATUM_TARGETS, 4);
    assert!(BASIN_X < STATION_X - 2.0 * RIM_W);
    assert!(BASIN_Y < STATION_Y - 2.0 * RIM_W);
    assert!(LEAK_SENSOR_WELLS >= ROUTE_CHANNELS);
    assert!(PRESSURE_COUPONS > OCCLUSION_POINTS);
    assert!(TOP_BOTTLE_LIFT_CLEARANCE > WASTE_BOTTLE_D);

    for feature in feature_rects() {
        assert!(
            feature.fits_inside_station(),
            "{} exceeds station envelope",
            feature.name
        );
    }

    let rects = feature_rects();
    for i in 0..rects.len() {
        for j in (i + 1)..rects.len() {
            if allowed_overlap(rects[i].name, rects[j].name) {
                continue;
            }
            assert!(
                !rects[i].overlaps(rects[j], 8.0),
                "{} overlaps {}",
                rects[i].name,
                rects[j].name
            );
        }
    }
}

fn allowed_overlap(left: &str, right: &str) -> bool {
    matches!(
        (left, right),
        ("evidence_bridge", _)
            | (_, "evidence_bridge")
            | (
                "leak_gutter_sensor_lane",
                "pinch_valve_occlusion_challenge_points"
            )
            | (
                "pinch_valve_occlusion_challenge_points",
                "leak_gutter_sensor_lane"
            )
            | ("leak_gutter_sensor_lane", "check_valve_orientation_witness")
            | ("check_valve_orientation_witness", "leak_gutter_sensor_lane")
            | (
                "waste_bag_bottle_nest",
                "overflow_float_optical_sensor_brackets"
            )
            | (
                "overflow_float_optical_sensor_brackets",
                "waste_bag_bottle_nest"
            )
    )
}

fn feature_rects() -> [Rect; 10] {
    [
        Rect {
            name: "waste_bag_bottle_nest",
            center: NEST_POS,
            x: NEST_X,
            y: NEST_Y,
        },
        Rect {
            name: "hydrophobic_vent_filter_mockup",
            center: VENT_POS,
            x: VENT_X,
            y: VENT_Y,
        },
        Rect {
            name: "pressure_transducer_coupon_bank",
            center: PRESSURE_POS,
            x: PRESSURE_X,
            y: PRESSURE_Y,
        },
        Rect {
            name: "pinch_valve_occlusion_challenge_points",
            center: OCCLUSION_POS,
            x: OCCLUSION_X,
            y: OCCLUSION_Y,
        },
        Rect {
            name: "overflow_float_optical_sensor_brackets",
            center: SENSOR_POS,
            x: SENSOR_X,
            y: SENSOR_Y,
        },
        Rect {
            name: "check_valve_orientation_witness",
            center: CHECK_POS,
            x: CHECK_X,
            y: CHECK_Y,
        },
        Rect {
            name: "leak_gutter_sensor_lane",
            center: GUTTER_POS,
            x: GUTTER_X,
            y: GUTTER_Y,
        },
        Rect {
            name: "barcode_lot_plate",
            center: TRACE_POS,
            x: TRACE_X,
            y: TRACE_Y,
        },
        Rect {
            name: "sampling_custody_panel",
            center: CUSTODY_POS,
            x: CUSTODY_X,
            y: CUSTODY_Y,
        },
        Rect {
            name: "evidence_bridge",
            center: EVIDENCE_POS,
            x: EVIDENCE_X,
            y: EVIDENCE_Y,
        },
    ]
}

fn secondary_containment_deck() -> Part {
    let deck = centered_cube(
        format!("{OUTPUT_PREFIX}_deck_plate"),
        STATION_X,
        STATION_Y,
        DECK_Z,
    );
    let basin = centered_cube(
        format!("{OUTPUT_PREFIX}_secondary_containment_recess"),
        BASIN_X,
        BASIN_Y,
        BASIN_DEPTH,
    )
    .translate(0.0, -12.0, DECK_Z / 2.0 - BASIN_DEPTH / 2.0 + 0.4);
    let drain = centered_cylinder(
        format!("{OUTPUT_PREFIX}_front_right_drain_bore"),
        DRAIN_D / 2.0,
        70.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 92.0, -STATION_Y / 2.0 - 4.0, 1.0);

    deck - basin - drain - fixture_sockets() - mount_holes()
        + containment_rims()
        + wet_dry_dividers()
        + datum_targets()
}

fn fixture_sockets() -> Part {
    let mut sockets = Part::empty(format!("{OUTPUT_PREFIX}_fixture_sockets"));
    for rect in feature_rects().iter().take(9) {
        sockets = sockets
            + centered_cube(
                format!("{OUTPUT_PREFIX}_{}_socket", rect.name),
                rect.x + 10.0,
                rect.y + 10.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(
                rect.center.0,
                rect.center.1,
                DECK_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.2,
            );
    }
    sockets
}

fn mount_holes() -> Part {
    let mut holes = Part::empty(format!("{OUTPUT_PREFIX}_mount_holes"));
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 54.0), -(STATION_Y / 2.0 - 48.0)),
        (STATION_X / 2.0 - 54.0, -(STATION_Y / 2.0 - 48.0)),
        (-(STATION_X / 2.0 - 54.0), STATION_Y / 2.0 - 48.0),
        (STATION_X / 2.0 - 54.0, STATION_Y / 2.0 - 48.0),
        (0.0, -(STATION_Y / 2.0 - 48.0)),
        (0.0, STATION_Y / 2.0 - 48.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_m6_mount_bore_{i}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 4.0,
                24,
            )
            .translate(*x, *y, 0.0);
    }
    holes
}

fn containment_rims() -> Part {
    let left = centered_cube(
        format!("{OUTPUT_PREFIX}_left_containment_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-(STATION_X / 2.0 - RIM_W / 2.0), 0.0, z_on_deck(RIM_Z));
    let right = centered_cube(
        format!("{OUTPUT_PREFIX}_right_containment_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, z_on_deck(RIM_Z));
    let rear = centered_cube(
        format!("{OUTPUT_PREFIX}_rear_containment_rim"),
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, z_on_deck(RIM_Z));
    let front_lip = centered_cube(
        format!("{OUTPUT_PREFIX}_front_low_leak_lip"),
        STATION_X - 160.0,
        12.0,
        24.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 18.0, z_on_deck(24.0));

    left + right + rear + front_lip
}

fn wet_dry_dividers() -> Part {
    let trace_divider = centered_cube(
        format!("{OUTPUT_PREFIX}_dry_traceability_divider"),
        10.0,
        130.0,
        30.0,
    )
    .translate(-336.0, 318.0, z_on_deck(30.0));
    let custody_divider = centered_cube(
        format!("{OUTPUT_PREFIX}_sample_custody_keepaway_divider"),
        10.0,
        130.0,
        30.0,
    )
    .translate(260.0, 318.0, z_on_deck(30.0));
    let main_wet_barrier = centered_cube(
        format!("{OUTPUT_PREFIX}_wet_service_back_barrier"),
        STATION_X - 210.0,
        8.0,
        30.0,
    )
    .translate(0.0, 160.0, z_on_deck(30.0));
    let lower_gutter_barrier = centered_cube(
        format!("{OUTPUT_PREFIX}_leak_gutter_isolation_barrier"),
        STATION_X - 230.0,
        8.0,
        28.0,
    )
    .translate(0.0, -286.0, z_on_deck(28.0));

    trace_divider + custody_divider + main_wet_barrier + lower_gutter_barrier
}

fn datum_targets() -> Part {
    let mut targets = Part::empty(format!("{OUTPUT_PREFIX}_datum_targets"));
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 84.0), STATION_Y / 2.0 - 78.0),
        (STATION_X / 2.0 - 84.0, STATION_Y / 2.0 - 78.0),
        (-(STATION_X / 2.0 - 84.0), -(STATION_Y / 2.0 - 78.0)),
        (STATION_X / 2.0 - 84.0, -(STATION_Y / 2.0 - 78.0)),
    ]
    .iter()
    .enumerate()
    {
        let disc = centered_cylinder(format!("{OUTPUT_PREFIX}_datum_disc_{i}"), 13.0, 4.0, 32)
            .translate(*x, *y, DECK_Z / 2.0 + 2.0);
        let bore = centered_cylinder(
            format!("{OUTPUT_PREFIX}_datum_center_bore_{i}"),
            3.0,
            6.0,
            20,
        )
        .translate(*x, *y, DECK_Z / 2.0 + 2.0);
        targets = targets + (disc - bore);
    }
    targets
}

fn waste_bag_bottle_nest() -> Part {
    let rack = centered_cube(
        format!("{OUTPUT_PREFIX}_waste_bag_bottle_nest_block"),
        NEST_X,
        NEST_Y,
        NEST_Z,
    )
    .translate(NEST_POS.0, NEST_POS.1, z_on_deck(NEST_Z));
    let bottle = centered_cylinder(
        format!("{OUTPUT_PREFIX}_round_waste_bottle_shadow"),
        WASTE_BOTTLE_D / 2.0,
        NEST_Z + 8.0,
        64,
    )
    .translate(NEST_POS.0 - 94.0, NEST_POS.1, z_on_deck(NEST_Z));
    let bag = centered_cube(
        format!("{OUTPUT_PREFIX}_flat_waste_bag_shadow"),
        WASTE_BAG_SHADOW_X,
        WASTE_BAG_SHADOW_Y,
        NEST_Z + 8.0,
    )
    .translate(NEST_POS.0 + 88.0, NEST_POS.1, z_on_deck(NEST_Z));

    let mut straps = Part::empty(format!("{OUTPUT_PREFIX}_nest_retention_straps"));
    for i in 0..NEST_STRAPS {
        let y = NEST_POS.1 + centered_index(i, NEST_STRAPS, 82.0);
        straps = straps
            + centered_cube(
                format!("{OUTPUT_PREFIX}_bottle_bag_retention_bridge_{i}"),
                NEST_X - 48.0,
                8.0,
                18.0,
            )
            .translate(NEST_POS.0, y, DECK_Z / 2.0 + NEST_Z + 9.0);
    }

    let mut locators = Part::empty(format!("{OUTPUT_PREFIX}_bottle_locator_bosses"));
    for i in 0..BOTTLE_LOCATORS {
        let angle = i as f64 * 90.0;
        let x = NEST_POS.0 - 94.0 + angle.to_radians().cos() * (WASTE_BOTTLE_D / 2.0 + 18.0);
        let y = NEST_POS.1 + angle.to_radians().sin() * (WASTE_BOTTLE_D / 2.0 + 18.0);
        locators = locators
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_waste_bottle_locator_{i}"),
                10.0,
                18.0,
                24,
            )
            .translate(x, y, DECK_Z / 2.0 + NEST_Z + 9.0);
    }

    rack - bottle - bag + straps + locators + bag_fill_shadow_ticks()
}

fn bag_fill_shadow_ticks() -> Part {
    let mut ticks = Part::empty(format!("{OUTPUT_PREFIX}_bag_fill_shadow_ticks"));
    for i in 0..5 {
        ticks = ticks
            + centered_cube(
                format!("{OUTPUT_PREFIX}_bag_fill_tick_{i}"),
                44.0 + i as f64 * 16.0,
                4.0,
                6.0,
            )
            .translate(
                NEST_POS.0 + 88.0,
                NEST_POS.1 - WASTE_BAG_SHADOW_Y / 2.0 + 24.0 + i as f64 * 26.0,
                DECK_Z / 2.0 + NEST_Z + 3.0,
            );
    }
    ticks
}

fn hydrophobic_vent_filter_mockup() -> Part {
    let base = centered_cube(
        format!("{OUTPUT_PREFIX}_vent_filter_base"),
        VENT_X,
        VENT_Y,
        VENT_Z,
    )
    .translate(VENT_POS.0, VENT_POS.1, z_on_deck(VENT_Z));
    let mut cutters = Part::empty(format!("{OUTPUT_PREFIX}_vent_filter_cutters"));
    let mut clips = Part::empty(format!("{OUTPUT_PREFIX}_vent_filter_clips"));

    for i in 0..VENT_FILTERS {
        let x = VENT_POS.0 + centered_index(i, VENT_FILTERS, 112.0);
        let bore = centered_cylinder(
            format!("{OUTPUT_PREFIX}_filter_capsule_bore_{i}"),
            FILTER_OD / 2.0,
            FILTER_LENGTH,
            48,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(x, VENT_POS.1, DECK_Z / 2.0 + VENT_Z + 12.0);
        let port = centered_cylinder(
            format!("{OUTPUT_PREFIX}_vent_sample_port_{i}"),
            VENT_PORT_D / 2.0,
            VENT_Z + 8.0,
            24,
        )
        .translate(x, VENT_POS.1 - 42.0, z_on_deck(VENT_Z));
        let left_clip = centered_cube(
            format!("{OUTPUT_PREFIX}_filter_left_clip_{i}"),
            8.0,
            56.0,
            22.0,
        )
        .translate(
            x - FILTER_LENGTH / 2.0 - 8.0,
            VENT_POS.1,
            DECK_Z / 2.0 + VENT_Z + 11.0,
        );
        let right_clip = centered_cube(
            format!("{OUTPUT_PREFIX}_filter_right_clip_{i}"),
            8.0,
            56.0,
            22.0,
        )
        .translate(
            x + FILTER_LENGTH / 2.0 + 8.0,
            VENT_POS.1,
            DECK_Z / 2.0 + VENT_Z + 11.0,
        );
        let hydrophobic_label_land = centered_cube(
            format!("{OUTPUT_PREFIX}_hydrophobic_filter_label_land_{i}"),
            70.0,
            12.0,
            6.0,
        )
        .translate(x, VENT_POS.1 + 52.0, DECK_Z / 2.0 + VENT_Z + 3.0);

        cutters = cutters + bore + port;
        clips = clips + left_clip + right_clip + hydrophobic_label_land;
    }

    base - cutters + clips
}

fn pressure_transducer_coupon_bank() -> Part {
    let bank = centered_cube(
        format!("{OUTPUT_PREFIX}_pressure_coupon_bank"),
        PRESSURE_X,
        PRESSURE_Y,
        PRESSURE_Z,
    )
    .translate(PRESSURE_POS.0, PRESSURE_POS.1, z_on_deck(PRESSURE_Z));
    let mut cutters = Part::empty(format!("{OUTPUT_PREFIX}_pressure_coupon_cutters"));
    let mut bosses = Part::empty(format!("{OUTPUT_PREFIX}_pressure_coupon_bosses"));

    for i in 0..PRESSURE_COUPONS {
        let x = PRESSURE_POS.0 + centered_index(i, PRESSURE_COUPONS, PRESSURE_COUPON_PITCH);
        let transducer = centered_cylinder(
            format!("{OUTPUT_PREFIX}_transducer_coupon_pocket_{i}"),
            TRANSDUCER_BORE_D / 2.0,
            PRESSURE_Z + 8.0,
            32,
        )
        .translate(x, PRESSURE_POS.1 + 22.0, z_on_deck(PRESSURE_Z));
        let tap = centered_cylinder(
            format!("{OUTPUT_PREFIX}_pressure_tap_bore_{i}"),
            PRESSURE_TAP_D / 2.0,
            PRESSURE_Y + 26.0,
            20,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, PRESSURE_POS.1, DECK_Z / 2.0 + PRESSURE_Z - 18.0);
        let boss = centered_cylinder(
            format!("{OUTPUT_PREFIX}_transducer_coupon_boss_{i}"),
            19.0,
            8.0,
            32,
        )
        .translate(x, PRESSURE_POS.1 + 22.0, DECK_Z / 2.0 + PRESSURE_Z + 4.0);
        let boss_opening = centered_cylinder(
            format!("{OUTPUT_PREFIX}_transducer_coupon_boss_opening_{i}"),
            TRANSDUCER_BORE_D / 2.0,
            10.0,
            32,
        )
        .translate(x, PRESSURE_POS.1 + 22.0, DECK_Z / 2.0 + PRESSURE_Z + 4.0);
        let zero_span_land = centered_cube(
            format!("{OUTPUT_PREFIX}_zero_span_label_land_{i}"),
            38.0,
            16.0,
            5.0,
        )
        .translate(x, PRESSURE_POS.1 - 44.0, DECK_Z / 2.0 + PRESSURE_Z + 2.5);

        cutters = cutters + transducer + tap;
        bosses = bosses + (boss - boss_opening) + zero_span_land;
    }

    bank - cutters + bosses
}

fn pinch_valve_occlusion_challenge_points() -> Part {
    let rail = centered_cube(
        format!("{OUTPUT_PREFIX}_occlusion_challenge_rail"),
        OCCLUSION_X,
        OCCLUSION_Y,
        OCCLUSION_Z,
    )
    .translate(OCCLUSION_POS.0, OCCLUSION_POS.1, z_on_deck(OCCLUSION_Z));
    let mut cutters = Part::empty(format!("{OUTPUT_PREFIX}_occlusion_cutters"));
    let mut features = Part::empty(format!("{OUTPUT_PREFIX}_occlusion_features"));

    for i in 0..OCCLUSION_POINTS {
        let x = OCCLUSION_POS.0 + centered_index(i, OCCLUSION_POINTS, OCCLUSION_PITCH);
        let channel = centered_cylinder(
            format!("{OUTPUT_PREFIX}_occlusion_tube_channel_{i}"),
            TUBE_BORE_D / 2.0,
            OCCLUSION_Y + 30.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, OCCLUSION_POS.1, DECK_Z / 2.0 + OCCLUSION_Z - 20.0);
        let pinch_cavity = centered_cylinder(
            format!("{OUTPUT_PREFIX}_pinch_roller_challenge_pocket_{i}"),
            PINCH_ROLLER_D / 2.0,
            OCCLUSION_Z + 8.0,
            40,
        )
        .translate(x, OCCLUSION_POS.1, z_on_deck(OCCLUSION_Z));
        let compression_gauge = centered_cube(
            format!("{OUTPUT_PREFIX}_pinch_compression_gauge_{i}"),
            48.0,
            6.0,
            18.0,
        )
        .translate(x, OCCLUSION_POS.1 + 48.0, DECK_Z / 2.0 + OCCLUSION_Z + 9.0);
        let lockout_tab = centered_cube(
            format!("{OUTPUT_PREFIX}_occlusion_lockout_tab_{i}"),
            24.0,
            18.0,
            10.0,
        )
        .translate(x, OCCLUSION_POS.1 - 54.0, DECK_Z / 2.0 + OCCLUSION_Z + 5.0);

        cutters = cutters + channel + pinch_cavity;
        features = features + compression_gauge + lockout_tab;
    }

    rail - cutters + features
}

fn overflow_float_optical_sensor_brackets() -> Part {
    let tower = centered_cube(
        format!("{OUTPUT_PREFIX}_overflow_sensor_tower"),
        SENSOR_X,
        SENSOR_Y,
        SENSOR_Z,
    )
    .translate(SENSOR_POS.0, SENSOR_POS.1, z_on_deck(SENSOR_Z));
    let mut cutters = Part::empty(format!("{OUTPUT_PREFIX}_overflow_sensor_cutters"));
    let mut features = Part::empty(format!("{OUTPUT_PREFIX}_overflow_sensor_features"));

    for i in 0..FLOAT_BRACKETS {
        let x = SENSOR_POS.0 + centered_index(i, FLOAT_BRACKETS, 74.0);
        let stem = centered_cylinder(
            format!("{OUTPUT_PREFIX}_float_stem_bore_{i}"),
            FLOAT_STEM_D / 2.0,
            SENSOR_Z + 10.0,
            24,
        )
        .translate(x, SENSOR_POS.1 - 48.0, z_on_deck(SENSOR_Z));
        let guard = centered_cylinder(
            format!("{OUTPUT_PREFIX}_float_guard_ring_{i}"),
            22.0,
            8.0,
            32,
        )
        .translate(x, SENSOR_POS.1 - 48.0, DECK_Z / 2.0 + SENSOR_Z + 4.0);
        let guard_opening = centered_cylinder(
            format!("{OUTPUT_PREFIX}_float_guard_opening_{i}"),
            FLOAT_STEM_D / 2.0,
            10.0,
            24,
        )
        .translate(x, SENSOR_POS.1 - 48.0, DECK_Z / 2.0 + SENSOR_Z + 4.0);
        let flag = centered_cube(
            format!("{OUTPUT_PREFIX}_float_travel_flag_{i}"),
            34.0,
            7.0,
            44.0,
        )
        .translate(x, SENSOR_POS.1 - 16.0, DECK_Z / 2.0 + SENSOR_Z + 24.0);

        cutters = cutters + stem;
        features = features + (guard - guard_opening) + flag;
    }

    for i in 0..OPTICAL_FLAGS {
        let z = DECK_Z / 2.0 + 40.0 + i as f64 * 30.0;
        features = features
            + centered_cube(
                format!("{OUTPUT_PREFIX}_optical_overfill_flag_slot_{i}"),
                SENSOR_X - 42.0,
                5.0,
                12.0,
            )
            .translate(SENSOR_POS.0, SENSOR_POS.1 + 60.0, z);
    }

    tower - cutters + features
}

fn check_valve_orientation_witness() -> Part {
    let rail = centered_cube(
        format!("{OUTPUT_PREFIX}_check_valve_witness_rail"),
        CHECK_X,
        CHECK_Y,
        CHECK_Z,
    )
    .translate(CHECK_POS.0, CHECK_POS.1, z_on_deck(CHECK_Z));
    let mut cutters = Part::empty(format!("{OUTPUT_PREFIX}_check_valve_cutters"));
    let mut arrows = Part::empty(format!("{OUTPUT_PREFIX}_check_valve_orientation_arrows"));

    for i in 0..CHECK_VALVE_POSITIONS {
        let x = CHECK_POS.0 + centered_index(i, CHECK_VALVE_POSITIONS, CHECK_PITCH);
        let body = centered_cylinder(
            format!("{OUTPUT_PREFIX}_check_valve_body_shadow_{i}"),
            CHECK_VALVE_D / 2.0,
            68.0,
            32,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(x, CHECK_POS.1, DECK_Z / 2.0 + CHECK_Z - 14.0);
        let tube = centered_cylinder(
            format!("{OUTPUT_PREFIX}_check_valve_inline_tube_bore_{i}"),
            TUBE_BORE_D / 2.0,
            CHECK_Y + 24.0,
            20,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, CHECK_POS.1, DECK_Z / 2.0 + CHECK_Z - 14.0);
        let arrow_stem = centered_cube(
            format!("{OUTPUT_PREFIX}_check_valve_flow_arrow_stem_{i}"),
            42.0,
            5.0,
            6.0,
        )
        .translate(x, CHECK_POS.1 + 38.0, DECK_Z / 2.0 + CHECK_Z + 3.0);
        let arrow_head = centered_cube(
            format!("{OUTPUT_PREFIX}_check_valve_flow_arrow_head_{i}"),
            14.0,
            16.0,
            6.0,
        )
        .rotate(0.0, 0.0, 45.0)
        .translate(x + 25.0, CHECK_POS.1 + 38.0, DECK_Z / 2.0 + CHECK_Z + 3.0);
        let reverse_blocker = centered_cube(
            format!("{OUTPUT_PREFIX}_reverse_install_blocker_key_{i}"),
            12.0,
            30.0,
            14.0,
        )
        .translate(x - 34.0, CHECK_POS.1 - 38.0, DECK_Z / 2.0 + CHECK_Z + 7.0);

        cutters = cutters + body + tube;
        arrows = arrows + arrow_stem + arrow_head + reverse_blocker;
    }

    rail - cutters + arrows
}

fn leak_gutter_sensor_lane() -> Part {
    let lane = centered_cube(
        format!("{OUTPUT_PREFIX}_leak_gutter_lane_body"),
        GUTTER_X,
        GUTTER_Y,
        GUTTER_Z,
    )
    .translate(GUTTER_POS.0, GUTTER_POS.1, z_on_deck(GUTTER_Z));
    let trough = centered_cube(
        format!("{OUTPUT_PREFIX}_sloped_gutter_recess"),
        GUTTER_X - 56.0,
        32.0,
        GUTTER_Z + 6.0,
    )
    .translate(GUTTER_POS.0, GUTTER_POS.1, z_on_deck(GUTTER_Z) + 6.0);
    let mut wells = Part::empty(format!("{OUTPUT_PREFIX}_leak_sensor_wells"));
    for i in 0..LEAK_SENSOR_WELLS {
        let x = GUTTER_POS.0 + centered_index(i, LEAK_SENSOR_WELLS, 130.0);
        let boss = centered_cylinder(
            format!("{OUTPUT_PREFIX}_gutter_leak_sensor_boss_{i}"),
            16.0,
            7.0,
            32,
        )
        .translate(x, GUTTER_POS.1, DECK_Z / 2.0 + GUTTER_Z + 3.5);
        let pocket = centered_cylinder(
            format!("{OUTPUT_PREFIX}_gutter_leak_sensor_pocket_{i}"),
            7.0,
            9.0,
            24,
        )
        .translate(x, GUTTER_POS.1, DECK_Z / 2.0 + GUTTER_Z + 4.5);
        wells = wells + (boss - pocket);
    }

    let mut baffles = Part::empty(format!(
        "{OUTPUT_PREFIX}_gutter_cross_contamination_baffles"
    ));
    for i in 0..GUTTER_BAFFLES {
        let x = GUTTER_POS.0 + centered_index(i, GUTTER_BAFFLES, 142.0);
        baffles = baffles
            + centered_cube(
                format!("{OUTPUT_PREFIX}_gutter_baffle_{i}"),
                5.0,
                GUTTER_Y - 16.0,
                20.0,
            )
            .translate(x, GUTTER_POS.1, DECK_Z / 2.0 + GUTTER_Z + 10.0);
    }

    lane - trough + wells + baffles
}

fn barcode_lot_plate() -> Part {
    let plate = centered_cube(
        format!("{OUTPUT_PREFIX}_barcode_lot_plate"),
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    )
    .translate(TRACE_POS.0, TRACE_POS.1, z_on_deck(TRACE_Z));
    let mut lands = Part::empty(format!("{OUTPUT_PREFIX}_barcode_lot_lands"));

    for i in 0..BARCODE_LANDS {
        let row = i / 4;
        let col = i % 4;
        lands = lands
            + centered_cube(format!("{OUTPUT_PREFIX}_barcode_land_{i}"), 52.0, 16.0, 4.0)
                .translate(
                    TRACE_POS.0 + centered_index(col, 4, 62.0),
                    TRACE_POS.1 + centered_index(row, 2, 34.0),
                    DECK_Z / 2.0 + TRACE_Z + 2.0,
                );
    }

    for i in 0..LOT_LANDS {
        lands = lands
            + centered_cube(
                format!("{OUTPUT_PREFIX}_lot_certificate_land_{i}"),
                58.0,
                20.0,
                4.0,
            )
            .translate(
                TRACE_POS.0 + centered_index(i, LOT_LANDS, 66.0),
                TRACE_POS.1 + 38.0,
                DECK_Z / 2.0 + TRACE_Z + 2.0,
            );
    }

    plate + lands
}

fn sampling_custody_panel() -> Part {
    let panel = centered_cube(
        format!("{OUTPUT_PREFIX}_sampling_custody_panel"),
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    )
    .translate(CUSTODY_POS.0, CUSTODY_POS.1, z_on_deck(CUSTODY_Z));
    let mut cutters = Part::empty(format!("{OUTPUT_PREFIX}_custody_cutters"));
    let mut features = Part::empty(format!("{OUTPUT_PREFIX}_custody_features"));

    for i in 0..SAMPLE_VIAL_WELLS {
        let x = CUSTODY_POS.0 + centered_index(i, SAMPLE_VIAL_WELLS, 46.0);
        let well = centered_cylinder(
            format!("{OUTPUT_PREFIX}_sealed_sample_vial_well_{i}"),
            11.5,
            CUSTODY_Z + 8.0,
            32,
        )
        .translate(x, CUSTODY_POS.1 - 20.0, z_on_deck(CUSTODY_Z));
        let seal_land = centered_cube(
            format!("{OUTPUT_PREFIX}_sample_custody_tamper_land_{i}"),
            34.0,
            12.0,
            5.0,
        )
        .translate(x, CUSTODY_POS.1 + 32.0, DECK_Z / 2.0 + CUSTODY_Z + 2.5);
        cutters = cutters + well;
        features = features + seal_land;
    }

    for i in 0..CUSTODY_SEAL_POINTS {
        features = features
            + centered_cylinder(
                format!("{OUTPUT_PREFIX}_custody_wire_seal_anchor_{i}"),
                7.0,
                10.0,
                24,
            )
            .translate(
                CUSTODY_POS.0 + centered_index(i, CUSTODY_SEAL_POINTS, 82.0),
                CUSTODY_POS.1 + 52.0,
                DECK_Z / 2.0 + CUSTODY_Z + 5.0,
            );
    }

    panel - cutters + features
}

fn interlock_evidence_bridge() -> Part {
    let left_post = centered_cube(
        format!("{OUTPUT_PREFIX}_evidence_left_post"),
        30.0,
        44.0,
        EVIDENCE_CLEARANCE_Z,
    )
    .translate(
        EVIDENCE_POS.0 - EVIDENCE_X / 2.0,
        EVIDENCE_POS.1,
        DECK_Z / 2.0 + EVIDENCE_CLEARANCE_Z / 2.0,
    );
    let right_post = centered_cube(
        format!("{OUTPUT_PREFIX}_evidence_right_post"),
        30.0,
        44.0,
        EVIDENCE_CLEARANCE_Z,
    )
    .translate(
        EVIDENCE_POS.0 + EVIDENCE_X / 2.0,
        EVIDENCE_POS.1,
        DECK_Z / 2.0 + EVIDENCE_CLEARANCE_Z / 2.0,
    );
    let beam = centered_cube(
        format!("{OUTPUT_PREFIX}_evidence_overhead_beam"),
        EVIDENCE_X,
        EVIDENCE_Y,
        EVIDENCE_BEAM_Z,
    )
    .translate(
        EVIDENCE_POS.0,
        EVIDENCE_POS.1,
        DECK_Z / 2.0 + EVIDENCE_CLEARANCE_Z + EVIDENCE_BEAM_Z / 2.0,
    );

    let mut features = Part::empty(format!("{OUTPUT_PREFIX}_evidence_bridge_features"));
    for i in 0..EVIDENCE_CAMERAS {
        features = features
            + centered_cube(
                format!("{OUTPUT_PREFIX}_interlock_camera_tab_{i}"),
                44.0,
                26.0,
                10.0,
            )
            .translate(
                EVIDENCE_POS.0 + centered_index(i, EVIDENCE_CAMERAS, 244.0),
                EVIDENCE_POS.1 - 30.0,
                DECK_Z / 2.0 + EVIDENCE_CLEARANCE_Z + EVIDENCE_BEAM_Z + 5.0,
            );
    }
    for i in 0..INTERLOCK_LIGHT_BARS {
        features = features
            + centered_cube(
                format!("{OUTPUT_PREFIX}_interlock_light_bar_{i}"),
                190.0,
                10.0,
                8.0,
            )
            .translate(
                EVIDENCE_POS.0 + centered_index(i, INTERLOCK_LIGHT_BARS, 320.0),
                EVIDENCE_POS.1 + 36.0,
                DECK_Z / 2.0 + EVIDENCE_CLEARANCE_Z + EVIDENCE_BEAM_Z + 4.0,
            );
    }

    left_post + right_post + beam + features
}

fn witnessed_tubing_routes() -> Part {
    let mut routes = Part::empty(format!("{OUTPUT_PREFIX}_witnessed_tubing_routes"));
    let route_specs = [
        (
            NEST_POS.0,
            NEST_POS.1 - 120.0,
            OCCLUSION_POS.0,
            OCCLUSION_POS.1 + 74.0,
        ),
        (
            OCCLUSION_POS.0 + 120.0,
            OCCLUSION_POS.1,
            PRESSURE_POS.0 - 116.0,
            PRESSURE_POS.1 - 34.0,
        ),
        (
            PRESSURE_POS.0,
            PRESSURE_POS.1 + 76.0,
            VENT_POS.0,
            VENT_POS.1 - 64.0,
        ),
        (
            NEST_POS.0 + 126.0,
            NEST_POS.1 + 98.0,
            SENSOR_POS.0 - 102.0,
            SENSOR_POS.1,
        ),
        (
            CHECK_POS.0 - 132.0,
            CHECK_POS.1,
            OCCLUSION_POS.0 + 170.0,
            OCCLUSION_POS.1,
        ),
        (
            CHECK_POS.0 + 86.0,
            CHECK_POS.1 - 62.0,
            GUTTER_POS.0 + 330.0,
            GUTTER_POS.1,
        ),
        (
            CUSTODY_POS.0 - 142.0,
            CUSTODY_POS.1 - 64.0,
            CHECK_POS.0 + 110.0,
            CHECK_POS.1 + 36.0,
        ),
    ];

    for (i, (x1, y1, x2, y2)) in route_specs.iter().enumerate() {
        let dx = x2 - x1;
        let dy = y2 - y1;
        let len = (dx * dx + dy * dy).sqrt();
        let angle = dy.atan2(dx).to_degrees();
        routes = routes
            + centered_cube(
                format!("{OUTPUT_PREFIX}_witness_route_channel_{i}"),
                len,
                9.0,
                7.0,
            )
            .rotate(0.0, 0.0, angle)
            .translate((x1 + x2) / 2.0, (y1 + y2) / 2.0, DECK_Z / 2.0 + 8.0);
    }

    routes
}

fn service_keepout_gauges() -> Part {
    centered_cube(
        format!("{OUTPUT_PREFIX}_front_operator_clearance_gauge"),
        STATION_X - 140.0,
        6.0,
        12.0,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 - FRONT_OPERATOR_CLEARANCE,
        DECK_Z / 2.0 + 6.0,
    ) + centered_cube(
        format!("{OUTPUT_PREFIX}_rear_vent_filter_clearance_gauge"),
        STATION_X - 180.0,
        6.0,
        12.0,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 + REAR_VENT_FILTER_CLEARANCE,
        DECK_Z / 2.0 + 6.0,
    ) + centered_cube(
        format!("{OUTPUT_PREFIX}_right_sample_service_clearance_gauge"),
        6.0,
        STATION_Y - 130.0,
        12.0,
    )
    .translate(
        STATION_X / 2.0 + RIGHT_SAMPLE_CLEARANCE,
        0.0,
        DECK_Z / 2.0 + 6.0,
    ) + centered_cube(
        format!("{OUTPUT_PREFIX}_top_bottle_lift_clearance_gauge"),
        150.0,
        150.0,
        8.0,
    )
    .translate(
        NEST_POS.0 - 94.0,
        NEST_POS.1,
        DECK_Z / 2.0 + NEST_Z + TOP_BOTTLE_LIFT_CLEARANCE,
    )
}

fn z_on_deck(height: f64) -> f64 {
    DECK_Z / 2.0 + height / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_manifest_is_complete_and_prefixed() {
        assert_eq!(OUTPUTS.len(), 12);
        assert_eq!(
            OUTPUTS[OUTPUTS.len() - 1],
            format!("output/{OUTPUT_PREFIX}_assembly.stl")
        );
        for path in OUTPUTS {
            assert!(path
                .starts_with("output/closed_waste_line_backpressure_overfill_interlock_station_"));
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn required_validation_features_are_represented() {
        for feature in [
            "waste_bag_bottle_nest",
            "secondary_containment_tray",
            "hydrophobic_vent_filter_mockup",
            "pressure_transducer_coupons",
            "pinch_valve_occlusion_challenge_points",
            "overflow_float_optical_sensor_brackets",
            "check_valve_orientation_witness",
            "leak_gutter_sensor_lane",
            "barcode_lot_plate",
            "sampling_custody_panel",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn layout_fits_and_keeps_critical_modules_separated() {
        assert_design_constraints();
    }

    #[test]
    fn interlock_counts_cover_the_waste_path() {
        assert!(PRESSURE_COUPONS >= OCCLUSION_POINTS);
        assert!(LEAK_SENSOR_WELLS >= ROUTE_CHANNELS);
        assert_eq!(FLOAT_BRACKETS + OPTICAL_FLAGS, 7);
        assert_eq!(BARCODE_LANDS + LOT_LANDS, 12);
        assert_eq!(SAMPLE_VIAL_WELLS, PRESSURE_COUPONS);
        assert_eq!(DATUM_TARGETS, 4);
    }

    #[test]
    fn service_envelopes_are_large_enough_for_bench_validation() {
        assert!(FRONT_OPERATOR_CLEARANCE >= 300.0);
        assert!(REAR_VENT_FILTER_CLEARANCE >= FILTER_LENGTH);
        assert!(RIGHT_SAMPLE_CLEARANCE >= CUSTODY_Y);
        assert!(TOP_BOTTLE_LIFT_CLEARANCE >= WASTE_BOTTLE_D + 120.0);
    }
}
