use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed perfusion waste backpressure alarm-trip validation station.
//
// Scope:
// - Bench validation CAD for an automated tissue-chip perfusion waste path.
// - Forces repeatable downstream waste restriction/backpressure challenges
//   without opening the culture side of the fluid path.
// - Makes trip threshold, trip timing, sensor identity, diverter position,
//   waste/retain custody, and leak/vent behavior mechanically visible.
//
// This is packaging and validation-fixture architecture only. It is not a
// pressure-rated manifold, a sterile barrier specification, a sensor acceptance
// procedure, a validated biohazard treatment system, or a release workflow.

const PREFIX: &str = "closed_perfusion_waste_backpressure_alarm_trip_station";

const OUTPUTS: [&str; 12] = [
    "output/closed_perfusion_waste_backpressure_alarm_trip_station_secondary_containment_deck.stl",
    "output/closed_perfusion_waste_backpressure_alarm_trip_station_closed_waste_bag_nest.stl",
    "output/closed_perfusion_waste_backpressure_alarm_trip_station_backpressure_challenge_manifold.stl",
    "output/closed_perfusion_waste_backpressure_alarm_trip_station_pressure_trip_sensor_bank.stl",
    "output/closed_perfusion_waste_backpressure_alarm_trip_station_alarm_trip_interlock_panel.stl",
    "output/closed_perfusion_waste_backpressure_alarm_trip_station_relief_diverter_valve_bridge.stl",
    "output/closed_perfusion_waste_backpressure_alarm_trip_station_vent_filter_backstop.stl",
    "output/closed_perfusion_waste_backpressure_alarm_trip_station_occlusion_coupon_carousel.stl",
    "output/closed_perfusion_waste_backpressure_alarm_trip_station_calibration_reference_column.stl",
    "output/closed_perfusion_waste_backpressure_alarm_trip_station_event_logger_timestamp_docks.stl",
    "output/closed_perfusion_waste_backpressure_alarm_trip_station_release_hold_reject_lanes.stl",
    "output/closed_perfusion_waste_backpressure_alarm_trip_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "secondary_containment_deck",
    "closed_waste_bag_nest",
    "backpressure_challenge_manifold",
    "pressure_trip_sensor_bank",
    "alarm_trip_interlock_panel",
    "relief_diverter_valve_bridge",
    "vent_filter_backstop",
    "occlusion_coupon_carousel",
    "calibration_reference_column",
    "event_logger_timestamp_docks",
    "release_hold_reject_lanes",
];

const STATION_X: f64 = 1400.0;
const STATION_Y: f64 = 900.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 50.0;
const BASIN_X: f64 = 1250.0;
const BASIN_Y: f64 = 740.0;
const BASIN_DEPTH: f64 = 8.0;
const MOUNT_HOLE_D: f64 = 6.6;
const LOW_POINT_DRAIN_D: f64 = 16.0;

const WASTE_POS: (f64, f64) = (-445.0, 220.0);
const WASTE_X: f64 = 420.0;
const WASTE_Y: f64 = 220.0;
const WASTE_Z: f64 = 68.0;
const WASTE_BAG_NESTS: usize = 2;
const RETAIN_BAG_NESTS: usize = 2;
const BAG_NEST_X: f64 = 148.0;
const BAG_NEST_Y: f64 = 78.0;
const BAG_NEST_Z: f64 = 34.0;
const BAG_PORTS_PER_NEST: usize = 2;

const MANIFOLD_POS: (f64, f64) = (0.0, 230.0);
const MANIFOLD_X: f64 = 420.0;
const MANIFOLD_Y: f64 = 200.0;
const MANIFOLD_Z: f64 = 58.0;
const CHALLENGE_LANES: usize = 6;
const CHALLENGE_PITCH_X: f64 = 58.0;
const TUBE_BORE_D: f64 = 6.4;
const RESTRICTOR_SLOTS: usize = CHALLENGE_LANES;

const SENSOR_POS: (f64, f64) = (445.0, 230.0);
const SENSOR_X: f64 = 360.0;
const SENSOR_Y: f64 = 200.0;
const SENSOR_Z: f64 = 66.0;
const PRESSURE_SENSOR_COUNT: usize = 6;
const SENSOR_PITCH_X: f64 = 54.0;
const PRESSURE_TAP_D: f64 = 4.8;
const TRIP_THRESHOLD_STEPS: usize = 5;

const DIVERTER_POS: (f64, f64) = (-435.0, 0.0);
const DIVERTER_X: f64 = 420.0;
const DIVERTER_Y: f64 = 180.0;
const DIVERTER_Z: f64 = 68.0;
const DIVERTER_VALVES: usize = 4;
const DIVERTER_PITCH_X: f64 = 84.0;
const VALVE_ACTUATOR_D: f64 = 28.0;
const RELIEF_CHECK_WINDOWS: usize = 4;

const VENT_POS: (f64, f64) = (0.0, 0.0);
const VENT_X: f64 = 330.0;
const VENT_Y: f64 = 180.0;
const VENT_Z: f64 = 112.0;
const VENT_FILTERS: usize = 3;
const FILTER_D: f64 = 32.0;
const FILTER_LENGTH: f64 = 138.0;
const ANTI_SIPHON_WINDOWS: usize = 3;

const OCCLUSION_POS: (f64, f64) = (440.0, 0.0);
const OCCLUSION_X: f64 = 360.0;
const OCCLUSION_Y: f64 = 180.0;
const OCCLUSION_Z: f64 = 56.0;
const OCCLUSION_COUPONS: usize = 5;
const OCCLUSION_PITCH_X: f64 = 62.0;

const COLUMN_POS: (f64, f64) = (-470.0, -250.0);
const COLUMN_BASE_X: f64 = 260.0;
const COLUMN_BASE_Y: f64 = 230.0;
const COLUMN_BASE_Z: f64 = 34.0;
const COLUMN_HEIGHT: f64 = 360.0;
const COLUMN_D: f64 = 34.0;
const COLUMN_TICKS: usize = 9;
const REFERENCE_WEIGHT_NESTS: usize = 4;

const LOGGER_POS: (f64, f64) = (-70.0, -290.0);
const LOGGER_X: f64 = 360.0;
const LOGGER_Y: f64 = 160.0;
const LOGGER_Z: f64 = 48.0;
const LOGGER_DOCKS: usize = 4;
const TIME_SYNC_DOCKS: usize = 2;
const EVENT_TOKEN_SLOTS: usize = 8;

const GATES_POS: (f64, f64) = (360.0, -290.0);
const GATES_X: f64 = 360.0;
const GATES_Y: f64 = 160.0;
const GATES_Z: f64 = 44.0;
const DISPOSITION_LANES: usize = 3;
const QUARANTINE_TOKENS_PER_LANE: usize = 4;

const ALARM_PANEL_X: f64 = 920.0;
const ALARM_PANEL_Y: f64 = 76.0;
const ALARM_PANEL_Z: f64 = 36.0;
const ALARM_PANEL_POS: (f64, f64) = (0.0, -120.0);
const ALARM_STATES: usize = 4;
const ALARM_CHANNELS: usize = 4;
const ALARM_TOKEN_PITCH_X: f64 = 76.0;
const ALARM_TOKEN_PITCH_Y: f64 = 18.0;

const LEAK_SENSOR_WELLS: usize = 10;
const ROUTE_GUARD_LANES: usize = 8;
const ROBOT_FIDUCIALS: usize = 4;
const SERVICE_KEEP_OUT_Z: f64 = 290.0;
const FRONT_ROBOT_CLEARANCE: f64 = 420.0;
const REAR_FILTER_CLEARANCE: f64 = 260.0;
const LEFT_WASTE_BAG_CLEARANCE: f64 = 220.0;
const RIGHT_SENSOR_SERVICE_CLEARANCE: f64 = 220.0;
const TOP_COLUMN_SERVICE_CLEARANCE: f64 = 400.0;

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

    fn overlaps_with_clearance(self, other: Rect, clearance: f64) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();
        dx < (self.x + other.x) / 2.0 + clearance && dy < (self.y + other.y) / 2.0 + clearance
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = secondary_containment_deck();
    export(OUTPUTS[0], &deck);

    let waste = closed_waste_bag_nest();
    export(OUTPUTS[1], &waste);

    let manifold = backpressure_challenge_manifold();
    export(OUTPUTS[2], &manifold);

    let sensors = pressure_trip_sensor_bank();
    export(OUTPUTS[3], &sensors);

    let alarm = alarm_trip_interlock_panel();
    export(OUTPUTS[4], &alarm);

    let diverter = relief_diverter_valve_bridge();
    export(OUTPUTS[5], &diverter);

    let vent = vent_filter_backstop();
    export(OUTPUTS[6], &vent);

    let occlusion = occlusion_coupon_carousel();
    export(OUTPUTS[7], &occlusion);

    let column = calibration_reference_column();
    export(OUTPUTS[8], &column);

    let logger = event_logger_timestamp_docks();
    export(OUTPUTS[9], &logger);

    let gates = release_hold_reject_lanes();
    export(OUTPUTS[10], &gates);

    let assembly = deck
        + waste.translate(WASTE_POS.0, WASTE_POS.1, BASE_Z + 8.0)
        + manifold.translate(MANIFOLD_POS.0, MANIFOLD_POS.1, BASE_Z + 8.0)
        + sensors.translate(SENSOR_POS.0, SENSOR_POS.1, BASE_Z + 8.0)
        + alarm.translate(ALARM_PANEL_POS.0, ALARM_PANEL_POS.1, BASE_Z + 8.0)
        + diverter.translate(DIVERTER_POS.0, DIVERTER_POS.1, BASE_Z + 8.0)
        + vent.translate(VENT_POS.0, VENT_POS.1, BASE_Z + 8.0)
        + occlusion.translate(OCCLUSION_POS.0, OCCLUSION_POS.1, BASE_Z + 8.0)
        + column.translate(COLUMN_POS.0, COLUMN_POS.1, BASE_Z + 8.0)
        + logger.translate(LOGGER_POS.0, LOGGER_POS.1, BASE_Z + 8.0)
        + gates.translate(GATES_POS.0, GATES_POS.1, BASE_Z + 8.0)
        + witnessed_tubing_routes()
        + robot_service_keepouts();
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed perfusion waste backpressure alarm-trip validation station:");
    println!(
        "  Containment:         {STATION_X:.0}mm x {STATION_Y:.0}mm deck, {LEAK_SENSOR_WELLS} leak wells, {LOW_POINT_DRAIN_D:.0}mm low-point drain"
    );
    println!(
        "  Challenge path:      {CHALLENGE_LANES} backpressure lanes, {RESTRICTOR_SLOTS} removable restriction slots, {OCCLUSION_COUPONS} occlusion coupons"
    );
    println!(
        "  Trip sensing:        {PRESSURE_SENSOR_COUNT} pressure sensor pockets, {TRIP_THRESHOLD_STEPS} threshold steps, {ALARM_CHANNELS} alarm channels, {ALARM_STATES} alarm states"
    );
    println!(
        "  Waste safety:        {WASTE_BAG_NESTS} waste nests, {RETAIN_BAG_NESTS} retain nests, {DIVERTER_VALVES} diverter valves, {VENT_FILTERS} vent filter mockups"
    );
    println!(
        "  Records/disposition: {LOGGER_DOCKS} logger docks, {TIME_SYNC_DOCKS} time-sync docks, {EVENT_TOKEN_SLOTS} event token slots, {DISPOSITION_LANES} release/hold/reject lanes"
    );
    println!(
        "  Service envelopes:   front {FRONT_ROBOT_CLEARANCE:.0}mm, rear {REAR_FILTER_CLEARANCE:.0}mm, left {LEFT_WASTE_BAG_CLEARANCE:.0}mm, right {RIGHT_SENSOR_SERVICE_CLEARANCE:.0}mm, top {TOP_COLUMN_SERVICE_CLEARANCE:.0}mm"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 12);
    assert_eq!(REQUIRED_FEATURES.len(), 11);
    assert_eq!(WASTE_BAG_NESTS + RETAIN_BAG_NESTS, DIVERTER_VALVES);
    assert_eq!(ALARM_CHANNELS, DIVERTER_VALVES);
    assert_eq!(PRESSURE_SENSOR_COUNT, CHALLENGE_LANES);
    assert_eq!(ROBOT_FIDUCIALS, 4);
    assert!(TRIP_THRESHOLD_STEPS >= 5);
    assert!(LEAK_SENSOR_WELLS >= ROUTE_GUARD_LANES);
    assert!(COLUMN_HEIGHT < TOP_COLUMN_SERVICE_CLEARANCE);
    assert!(BASIN_X < STATION_X - 2.0 * RIM_W);
    assert!(BASIN_Y < STATION_Y - 2.0 * RIM_W);

    for rect in layout_rects() {
        assert!(
            rect.fits_inside_station(),
            "{} does not fit inside usable containment deck",
            rect.name
        );
    }

    let rects = layout_rects();
    for (i, left) in rects.iter().enumerate() {
        for right in rects.iter().skip(i + 1) {
            assert!(
                !left.overlaps_with_clearance(*right, 10.0),
                "{} overlaps {}",
                left.name,
                right.name
            );
        }
    }
}

fn layout_rects() -> [Rect; 9] {
    [
        Rect {
            name: "closed_waste_bag_nest",
            center: WASTE_POS,
            x: WASTE_X,
            y: WASTE_Y,
        },
        Rect {
            name: "backpressure_challenge_manifold",
            center: MANIFOLD_POS,
            x: MANIFOLD_X,
            y: MANIFOLD_Y,
        },
        Rect {
            name: "pressure_trip_sensor_bank",
            center: SENSOR_POS,
            x: SENSOR_X,
            y: SENSOR_Y,
        },
        Rect {
            name: "relief_diverter_valve_bridge",
            center: DIVERTER_POS,
            x: DIVERTER_X,
            y: DIVERTER_Y,
        },
        Rect {
            name: "vent_filter_backstop",
            center: VENT_POS,
            x: VENT_X,
            y: VENT_Y,
        },
        Rect {
            name: "occlusion_coupon_carousel",
            center: OCCLUSION_POS,
            x: OCCLUSION_X,
            y: OCCLUSION_Y,
        },
        Rect {
            name: "calibration_reference_column",
            center: COLUMN_POS,
            x: COLUMN_BASE_X,
            y: COLUMN_BASE_Y,
        },
        Rect {
            name: "event_logger_timestamp_docks",
            center: LOGGER_POS,
            x: LOGGER_X,
            y: LOGGER_Y,
        },
        Rect {
            name: "release_hold_reject_lanes",
            center: GATES_POS,
            x: GATES_X,
            y: GATES_Y,
        },
    ]
}

fn secondary_containment_deck() -> Part {
    let deck = centered_cube(
        format!("{PREFIX}_base_secondary_containment_deck"),
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let basin = centered_cube(
        format!("{PREFIX}_recessed_leak_basin"),
        BASIN_X,
        BASIN_Y,
        BASIN_DEPTH,
    )
    .translate(0.0, 0.0, BASE_Z - BASIN_DEPTH / 2.0);
    let drain = centered_cylinder(
        format!("{PREFIX}_front_right_low_point_drain_bore"),
        LOW_POINT_DRAIN_D / 2.0,
        58.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 120.0,
        -STATION_Y / 2.0 + 22.0,
        BASE_Z - 9.0,
    );

    deck - basin - drain - mounting_hole_cuts()
        + containment_rim()
        + leak_sensor_wells()
        + deck_datum_fiducials()
        + deck_flow_ribs()
        + station_zone_lands()
}

fn containment_rim() -> Part {
    let front = centered_cube(
        format!("{PREFIX}_front_secondary_rim"),
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        format!("{PREFIX}_rear_secondary_rim"),
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let left = centered_cube(
        format!("{PREFIX}_left_secondary_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let right = centered_cube(
        format!("{PREFIX}_right_secondary_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    front + rear + left + right
}

fn mounting_hole_cuts() -> Part {
    let mut holes = Part::empty(format!("{PREFIX}_mount_hole_cuts"));
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 58.0, -STATION_Y / 2.0 + 58.0),
        (STATION_X / 2.0 - 58.0, -STATION_Y / 2.0 + 58.0),
        (-STATION_X / 2.0 + 58.0, STATION_Y / 2.0 - 58.0),
        (STATION_X / 2.0 - 58.0, STATION_Y / 2.0 - 58.0),
        (0.0, -STATION_Y / 2.0 + 58.0),
        (0.0, STATION_Y / 2.0 - 58.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("{PREFIX}_m6_mount_clearance_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 6.0,
                24,
            )
            .translate(*x, *y, BASE_Z / 2.0);
    }
    holes
}

fn leak_sensor_wells() -> Part {
    let mut wells = Part::empty(format!("{PREFIX}_leak_sensor_wells"));
    for i in 0..LEAK_SENSOR_WELLS {
        let x = -560.0 + (i % 5) as f64 * 280.0;
        let y = -345.0 + (i / 5) as f64 * 690.0;
        wells = wells
            + centered_cube(
                format!("{PREFIX}_conductive_leak_strip_pocket_{i}"),
                58.0,
                30.0,
                6.0,
            )
            .translate(x, y, BASE_Z + 3.0)
            + centered_cube(
                format!("{PREFIX}_leak_strip_wire_exit_slot_{i}"),
                84.0,
                6.0,
                5.0,
            )
            .translate(x, y - 28.0, BASE_Z + 3.0);
    }
    wells
}

fn deck_datum_fiducials() -> Part {
    let mut fiducials = Part::empty(format!("{PREFIX}_robot_datum_fiducials"));
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 96.0, STATION_Y / 2.0 - 96.0),
        (STATION_X / 2.0 - 96.0, STATION_Y / 2.0 - 96.0),
        (-STATION_X / 2.0 + 96.0, -STATION_Y / 2.0 + 96.0),
        (STATION_X / 2.0 - 96.0, -STATION_Y / 2.0 + 96.0),
    ]
    .iter()
    .enumerate()
    {
        fiducials =
            fiducials
                + centered_cylinder(format!("{PREFIX}_robot_datum_disc_{i}"), 16.0, 4.0, 36)
                    .translate(*x, *y, BASE_Z + 2.0)
                - centered_cylinder(
                    format!("{PREFIX}_robot_datum_center_bore_{i}"),
                    3.2,
                    8.0,
                    20,
                )
                .translate(*x, *y, BASE_Z + 2.0);
    }
    fiducials
}

fn deck_flow_ribs() -> Part {
    let mut ribs = Part::empty(format!("{PREFIX}_leak_basin_flow_ribs"));
    for (i, y) in [-300.0, -180.0, -60.0, 60.0, 180.0, 300.0]
        .iter()
        .enumerate()
    {
        ribs = ribs
            + centered_cube(
                format!("{PREFIX}_sloped_leak_witness_rib_{i}"),
                BASIN_X - 110.0,
                7.0,
                6.0,
            )
            .translate(0.0, *y, BASE_Z + 3.0);
    }
    ribs
}

fn station_zone_lands() -> Part {
    let clean = centered_cube(
        format!("{PREFIX}_clean_closed_harness_inbound_land"),
        250.0,
        68.0,
        6.0,
    )
    .translate(-510.0, STATION_Y / 2.0 - 62.0, BASE_Z + 3.0);
    let challenged = centered_cube(
        format!("{PREFIX}_challenged_waste_harness_quarantine_land"),
        310.0,
        68.0,
        6.0,
    )
    .translate(-160.0, STATION_Y / 2.0 - 62.0, BASE_Z + 3.0);
    let safe = centered_cube(
        format!("{PREFIX}_verified_safe_waste_route_land"),
        290.0,
        68.0,
        6.0,
    )
    .translate(240.0, STATION_Y / 2.0 - 62.0, BASE_Z + 3.0);
    clean + challenged + safe
}

fn closed_waste_bag_nest() -> Part {
    let shell = centered_cube(
        format!("{PREFIX}_closed_waste_bag_nest_body"),
        WASTE_X,
        WASTE_Y,
        WASTE_Z,
    )
    .translate(0.0, 0.0, WASTE_Z / 2.0);
    let lightening = centered_cube(
        format!("{PREFIX}_closed_waste_bag_nest_under_relief"),
        WASTE_X - 70.0,
        WASTE_Y - 58.0,
        22.0,
    )
    .translate(0.0, 0.0, 12.0);

    shell - lightening - bag_nest_cavities() - bag_port_bores()
        + bag_nest_rims()
        + bag_hold_down_bridges()
        + bag_barcode_lands()
        + bag_route_strain_relief_comb()
}

fn bag_nest_cavities() -> Part {
    let mut cavities = Part::empty(format!("{PREFIX}_bag_nest_cavity_cuts"));
    for bag in 0..(WASTE_BAG_NESTS + RETAIN_BAG_NESTS) {
        let x = lane_x(bag, WASTE_BAG_NESTS + RETAIN_BAG_NESTS, 88.0);
        let y = if bag < WASTE_BAG_NESTS { -38.0 } else { 48.0 };
        cavities = cavities
            + centered_cube(
                format!("{PREFIX}_{}_bag_saddle_cavity", bag_slug(bag)),
                BAG_NEST_X,
                BAG_NEST_Y,
                BAG_NEST_Z,
            )
            .translate(x, y, WASTE_Z - BAG_NEST_Z / 2.0 + 4.0);
    }
    cavities
}

fn bag_port_bores() -> Part {
    let mut bores = Part::empty(format!("{PREFIX}_bag_port_bores"));
    for bag in 0..(WASTE_BAG_NESTS + RETAIN_BAG_NESTS) {
        let x = lane_x(bag, WASTE_BAG_NESTS + RETAIN_BAG_NESTS, 88.0);
        let y = if bag < WASTE_BAG_NESTS { -38.0 } else { 48.0 };
        for port in 0..BAG_PORTS_PER_NEST {
            bores = bores
                + centered_cylinder(
                    format!("{PREFIX}_{}_bag_closed_qd_port_bore_{port}", bag_slug(bag)),
                    TUBE_BORE_D / 2.0,
                    WASTE_Y + 18.0,
                    20,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(
                    x + lane_x(port, BAG_PORTS_PER_NEST, 34.0),
                    y,
                    WASTE_Z - 12.0,
                );
        }
    }
    bores
}

fn bag_nest_rims() -> Part {
    let mut rims = Part::empty(format!("{PREFIX}_bag_nest_raised_rims"));
    for bag in 0..(WASTE_BAG_NESTS + RETAIN_BAG_NESTS) {
        let x = lane_x(bag, WASTE_BAG_NESTS + RETAIN_BAG_NESTS, 88.0);
        let y = if bag < WASTE_BAG_NESTS { -38.0 } else { 48.0 };
        rims = rims
            + centered_cube(
                format!("{PREFIX}_{}_bag_front_rim", bag_slug(bag)),
                BAG_NEST_X + 14.0,
                8.0,
                12.0,
            )
            .translate(x, y - BAG_NEST_Y / 2.0, WASTE_Z + 6.0)
            + centered_cube(
                format!("{PREFIX}_{}_bag_rear_rim", bag_slug(bag)),
                BAG_NEST_X + 14.0,
                8.0,
                12.0,
            )
            .translate(x, y + BAG_NEST_Y / 2.0, WASTE_Z + 6.0);
    }
    rims
}

fn bag_hold_down_bridges() -> Part {
    let mut bridges = Part::empty(format!("{PREFIX}_bag_hold_down_bridges"));
    for bag in 0..(WASTE_BAG_NESTS + RETAIN_BAG_NESTS) {
        let x = lane_x(bag, WASTE_BAG_NESTS + RETAIN_BAG_NESTS, 88.0);
        let y = if bag < WASTE_BAG_NESTS { -38.0 } else { 48.0 };
        bridges = bridges
            + centered_cube(
                format!("{PREFIX}_{}_bag_hold_down_bridge", bag_slug(bag)),
                72.0,
                14.0,
                18.0,
            )
            .translate(x, y, WASTE_Z + 18.0);
    }
    bridges
}

fn bag_barcode_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_bag_barcode_lands"));
    for bag in 0..(WASTE_BAG_NESTS + RETAIN_BAG_NESTS) {
        lands = lands
            + centered_cube(
                format!("{PREFIX}_{}_bag_barcode_land", bag_slug(bag)),
                64.0,
                20.0,
                5.0,
            )
            .translate(
                lane_x(bag, WASTE_BAG_NESTS + RETAIN_BAG_NESTS, 88.0),
                -98.0,
                WASTE_Z + 2.5,
            );
    }
    lands
}

fn bag_route_strain_relief_comb() -> Part {
    let mut comb = Part::empty(format!("{PREFIX}_waste_bag_route_strain_relief_comb"));
    for lane in 0..ROUTE_GUARD_LANES {
        let x = lane_x(lane, ROUTE_GUARD_LANES, 42.0);
        comb = comb
            + centered_cube(
                format!("{PREFIX}_bag_tube_snap_clip_{lane}"),
                22.0,
                30.0,
                28.0,
            )
            .translate(x, -WASTE_Y / 2.0 - 22.0, 19.0)
            - centered_cylinder(
                format!("{PREFIX}_bag_tube_snap_clip_bore_{lane}"),
                TUBE_BORE_D / 2.0,
                34.0,
                18,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -WASTE_Y / 2.0 - 22.0, 19.0);
    }
    comb
}

fn backpressure_challenge_manifold() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_backpressure_challenge_manifold_body"),
        MANIFOLD_X,
        MANIFOLD_Y,
        MANIFOLD_Z,
    )
    .translate(0.0, 0.0, MANIFOLD_Z / 2.0);
    let underside = centered_cube(
        format!("{PREFIX}_backpressure_manifold_under_relief"),
        MANIFOLD_X - 54.0,
        MANIFOLD_Y - 46.0,
        18.0,
    )
    .translate(0.0, 0.0, 10.0);

    body - underside - challenge_lane_bores() - restrictor_coupon_slots()
        + manifold_lane_ribs()
        + manifold_pressure_tap_bulkheads()
        + removable_restrictor_tokens()
        + bypass_reference_windows()
}

fn challenge_lane_bores() -> Part {
    let mut bores = Part::empty(format!("{PREFIX}_challenge_lane_bores"));
    for lane in 0..CHALLENGE_LANES {
        let x = lane_x(lane, CHALLENGE_LANES, CHALLENGE_PITCH_X);
        bores = bores
            + centered_cylinder(
                format!("{PREFIX}_waste_challenge_lane_bore_{lane}"),
                TUBE_BORE_D / 2.0,
                MANIFOLD_Y + 20.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, MANIFOLD_Z / 2.0)
            + centered_cylinder(
                format!("{PREFIX}_lane_{lane}_cross_pressure_tap_bore"),
                PRESSURE_TAP_D / 2.0,
                82.0,
                16,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(x, 18.0, MANIFOLD_Z - 18.0);
    }
    bores
}

fn restrictor_coupon_slots() -> Part {
    let mut slots = Part::empty(format!("{PREFIX}_restrictor_coupon_slots"));
    for lane in 0..RESTRICTOR_SLOTS {
        let x = lane_x(lane, RESTRICTOR_SLOTS, CHALLENGE_PITCH_X);
        slots = slots
            + centered_cube(
                format!("{PREFIX}_removable_restrictor_coupon_slot_{lane}"),
                34.0,
                56.0,
                22.0,
            )
            .translate(x, -16.0, MANIFOLD_Z - 9.0);
    }
    slots
}

fn manifold_lane_ribs() -> Part {
    let mut ribs = Part::empty(format!("{PREFIX}_manifold_lane_ribs"));
    for lane in 0..CHALLENGE_LANES {
        let x = lane_x(lane, CHALLENGE_LANES, CHALLENGE_PITCH_X);
        ribs = ribs
            + centered_cube(
                format!("{PREFIX}_lane_{lane}_flow_direction_rib"),
                38.0,
                6.0,
                8.0,
            )
            .translate(x, MANIFOLD_Y / 2.0 - 26.0, MANIFOLD_Z + 4.0)
            + centered_cube(
                format!("{PREFIX}_lane_{lane}_trip_sample_land"),
                42.0,
                24.0,
                6.0,
            )
            .translate(x, -MANIFOLD_Y / 2.0 + 32.0, MANIFOLD_Z + 3.0);
    }
    ribs
}

fn manifold_pressure_tap_bulkheads() -> Part {
    let mut bulkheads = Part::empty(format!("{PREFIX}_manifold_pressure_tap_bulkheads"));
    for lane in 0..CHALLENGE_LANES {
        let x = lane_x(lane, CHALLENGE_LANES, CHALLENGE_PITCH_X);
        bulkheads = bulkheads
            + centered_cylinder(
                format!("{PREFIX}_upstream_pressure_tap_bulkhead_{lane}"),
                14.0,
                18.0,
                28,
            )
            .translate(x, 58.0, MANIFOLD_Z + 9.0)
            + centered_cylinder(
                format!("{PREFIX}_downstream_pressure_tap_bulkhead_{lane}"),
                14.0,
                18.0,
                28,
            )
            .translate(x, -58.0, MANIFOLD_Z + 9.0);
    }
    bulkheads
}

fn removable_restrictor_tokens() -> Part {
    let mut tokens = Part::empty(format!("{PREFIX}_removable_restrictor_tokens"));
    for lane in 0..RESTRICTOR_SLOTS {
        let x = lane_x(lane, RESTRICTOR_SLOTS, CHALLENGE_PITCH_X);
        tokens = tokens
            + centered_cube(
                format!("{PREFIX}_restriction_level_token_{lane}"),
                30.0,
                12.0 + lane as f64 * 4.0,
                10.0,
            )
            .translate(x, -16.0, MANIFOLD_Z + 5.0);
    }
    tokens
}

fn bypass_reference_windows() -> Part {
    let mut windows = Part::empty(format!("{PREFIX}_bypass_reference_windows"));
    for lane in 0..CHALLENGE_LANES {
        let x = lane_x(lane, CHALLENGE_LANES, CHALLENGE_PITCH_X);
        windows = windows
            + centered_cube(
                format!("{PREFIX}_lane_{lane}_clear_bypass_witness_window"),
                34.0,
                12.0,
                16.0,
            )
            .translate(x, 18.0, MANIFOLD_Z + 8.0);
    }
    windows
}

fn pressure_trip_sensor_bank() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_pressure_trip_sensor_bank_body"),
        SENSOR_X,
        SENSOR_Y,
        SENSOR_Z,
    )
    .translate(0.0, 0.0, SENSOR_Z / 2.0);
    let cable_raceway = centered_cube(
        format!("{PREFIX}_pressure_sensor_rear_cable_raceway_cut"),
        SENSOR_X - 48.0,
        22.0,
        22.0,
    )
    .translate(0.0, SENSOR_Y / 2.0 - 20.0, SENSOR_Z - 14.0);

    body - cable_raceway - pressure_sensor_pocket_cuts() - pressure_tap_bores()
        + pressure_sensor_retainers()
        + threshold_step_ladder()
        + alarm_trip_status_flags()
        + sensor_cable_comb()
}

fn pressure_sensor_pocket_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_pressure_sensor_pocket_cuts"));
    for sensor in 0..PRESSURE_SENSOR_COUNT {
        let x = lane_x(sensor, PRESSURE_SENSOR_COUNT, SENSOR_PITCH_X);
        cuts = cuts
            + centered_cube(
                format!("{PREFIX}_pressure_sensor_rectangular_pocket_{sensor}"),
                38.0,
                50.0,
                26.0,
            )
            .translate(x, -18.0, SENSOR_Z - 12.0)
            + centered_cylinder(
                format!("{PREFIX}_pressure_sensor_o_ring_groove_{sensor}"),
                19.0,
                5.0,
                32,
            )
            .translate(x, -18.0, SENSOR_Z + 1.0);
    }
    cuts
}

fn pressure_tap_bores() -> Part {
    let mut bores = Part::empty(format!("{PREFIX}_pressure_sensor_tap_bores"));
    for sensor in 0..PRESSURE_SENSOR_COUNT {
        let x = lane_x(sensor, PRESSURE_SENSOR_COUNT, SENSOR_PITCH_X);
        bores = bores
            + centered_cylinder(
                format!("{PREFIX}_pressure_tap_front_bore_{sensor}"),
                PRESSURE_TAP_D / 2.0,
                SENSOR_Y + 16.0,
                16,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, SENSOR_Z / 2.0);
    }
    bores
}

fn pressure_sensor_retainers() -> Part {
    let mut retainers = Part::empty(format!("{PREFIX}_pressure_sensor_retainer_clips"));
    for sensor in 0..PRESSURE_SENSOR_COUNT {
        let x = lane_x(sensor, PRESSURE_SENSOR_COUNT, SENSOR_PITCH_X);
        retainers = retainers
            + centered_cube(
                format!("{PREFIX}_pressure_sensor_left_retainer_{sensor}"),
                6.0,
                58.0,
                14.0,
            )
            .translate(x - 24.0, -18.0, SENSOR_Z + 7.0)
            + centered_cube(
                format!("{PREFIX}_pressure_sensor_right_retainer_{sensor}"),
                6.0,
                58.0,
                14.0,
            )
            .translate(x + 24.0, -18.0, SENSOR_Z + 7.0);
    }
    retainers
}

fn threshold_step_ladder() -> Part {
    let mut ladder = Part::empty(format!("{PREFIX}_pressure_trip_threshold_step_ladder"));
    for step in 0..TRIP_THRESHOLD_STEPS {
        ladder = ladder
            + centered_cube(
                format!("{PREFIX}_threshold_step_{step}_raised_reference"),
                44.0,
                12.0,
                6.0 + step as f64 * 3.0,
            )
            .translate(
                -120.0 + step as f64 * 60.0,
                SENSOR_Y / 2.0 - 52.0,
                SENSOR_Z + 3.0 + step as f64 * 1.5,
            );
    }
    ladder
}

fn alarm_trip_status_flags() -> Part {
    let mut flags = Part::empty(format!("{PREFIX}_alarm_trip_status_flags"));
    for state in 0..ALARM_STATES {
        flags = flags
            + centered_cube(
                format!("{PREFIX}_sensor_bank_alarm_state_flag_{state}"),
                48.0,
                12.0,
                10.0,
            )
            .translate(
                SENSOR_X / 2.0 - 42.0,
                -64.0 + state as f64 * 30.0,
                SENSOR_Z + 5.0,
            );
    }
    flags
}

fn sensor_cable_comb() -> Part {
    let mut comb = Part::empty(format!("{PREFIX}_sensor_cable_comb"));
    for sensor in 0..PRESSURE_SENSOR_COUNT {
        let x = lane_x(sensor, PRESSURE_SENSOR_COUNT, SENSOR_PITCH_X);
        comb = comb
            + centered_cube(
                format!("{PREFIX}_sensor_cable_comb_tooth_{sensor}"),
                8.0,
                34.0,
                20.0,
            )
            .translate(x, SENSOR_Y / 2.0 + 14.0, SENSOR_Z / 2.0);
    }
    comb
}

fn alarm_trip_interlock_panel() -> Part {
    let rail = centered_cube(
        format!("{PREFIX}_alarm_trip_interlock_panel_body"),
        ALARM_PANEL_X,
        ALARM_PANEL_Y,
        ALARM_PANEL_Z,
    )
    .translate(0.0, 0.0, ALARM_PANEL_Z / 2.0);
    let underside = centered_cube(
        format!("{PREFIX}_alarm_trip_interlock_panel_under_relief"),
        ALARM_PANEL_X - 60.0,
        ALARM_PANEL_Y - 28.0,
        16.0,
    )
    .translate(0.0, 0.0, 10.0);

    rail - underside - alarm_token_slot_cuts()
        + alarm_token_slot_rims()
        + alarm_channel_headers()
        + alarm_sequence_bridge()
        + no_manual_override_guard()
}

fn alarm_token_slot_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_alarm_token_slot_cuts"));
    for channel in 0..ALARM_CHANNELS {
        for state in 0..ALARM_STATES {
            cuts = cuts
                + centered_cube(
                    format!("{PREFIX}_channel_{channel}_state_{state}_alarm_token_slot_cut"),
                    34.0,
                    12.0,
                    16.0,
                )
                .translate(
                    alarm_token_x(channel),
                    alarm_token_y(state),
                    ALARM_PANEL_Z - 6.0,
                );
        }
    }
    cuts
}

fn alarm_token_slot_rims() -> Part {
    let mut rims = Part::empty(format!("{PREFIX}_alarm_token_slot_rims"));
    for channel in 0..ALARM_CHANNELS {
        for state in 0..ALARM_STATES {
            rims = rims
                + centered_cube(
                    format!("{PREFIX}_channel_{channel}_state_{state}_alarm_token_rim"),
                    40.0,
                    16.0,
                    6.0,
                )
                .translate(
                    alarm_token_x(channel),
                    alarm_token_y(state),
                    ALARM_PANEL_Z + 3.0,
                );
        }
    }
    rims
}

fn alarm_channel_headers() -> Part {
    let mut headers = Part::empty(format!("{PREFIX}_alarm_channel_headers"));
    for channel in 0..ALARM_CHANNELS {
        headers = headers
            + centered_cube(
                format!("{PREFIX}_alarm_channel_{channel}_header_land"),
                54.0,
                12.0,
                6.0,
            )
            .translate(
                alarm_token_x(channel),
                ALARM_PANEL_Y / 2.0 - 10.0,
                ALARM_PANEL_Z + 3.0,
            );
    }
    headers
}

fn alarm_sequence_bridge() -> Part {
    let pretrip = centered_cube(
        format!("{PREFIX}_pretrip_to_trip_sequence_bridge"),
        250.0,
        6.0,
        8.0,
    )
    .translate(-155.0, 0.0, ALARM_PANEL_Z + 4.0);
    let trip = centered_cube(
        format!("{PREFIX}_trip_to_divert_sequence_bridge"),
        250.0,
        6.0,
        8.0,
    )
    .translate(155.0, 0.0, ALARM_PANEL_Z + 4.0);
    let audit = centered_cube(
        format!("{PREFIX}_alarm_trip_audit_required_bridge"),
        180.0,
        8.0,
        12.0,
    )
    .translate(0.0, -ALARM_PANEL_Y / 2.0 + 12.0, ALARM_PANEL_Z + 6.0);
    pretrip + trip + audit
}

fn no_manual_override_guard() -> Part {
    let cover = centered_cube(
        format!("{PREFIX}_no_manual_override_guard_cover"),
        140.0,
        30.0,
        26.0,
    )
    .translate(ALARM_PANEL_X / 2.0 - 90.0, 0.0, ALARM_PANEL_Z + 13.0);
    let window = centered_cube(
        format!("{PREFIX}_no_manual_override_guard_window"),
        100.0,
        16.0,
        18.0,
    )
    .translate(ALARM_PANEL_X / 2.0 - 90.0, 0.0, ALARM_PANEL_Z + 15.0);
    cover - window
}

fn relief_diverter_valve_bridge() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_relief_diverter_valve_bridge_body"),
        DIVERTER_X,
        DIVERTER_Y,
        DIVERTER_Z,
    )
    .translate(0.0, 0.0, DIVERTER_Z / 2.0);

    body - diverter_tube_bores() - diverter_valve_socket_cuts()
        + diverter_valve_actuator_mockups()
        + relief_check_windows()
        + quarantine_route_arrows()
        + diverter_position_witness_tabs()
}

fn diverter_tube_bores() -> Part {
    let mut bores = Part::empty(format!("{PREFIX}_diverter_tube_bores"));
    for valve in 0..DIVERTER_VALVES {
        let x = lane_x(valve, DIVERTER_VALVES, DIVERTER_PITCH_X);
        bores = bores
            + centered_cylinder(
                format!("{PREFIX}_diverter_valve_inlet_bore_{valve}"),
                TUBE_BORE_D / 2.0,
                DIVERTER_Y + 18.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, DIVERTER_Z / 2.0)
            + centered_cylinder(
                format!("{PREFIX}_diverter_valve_retain_waste_cross_bore_{valve}"),
                TUBE_BORE_D / 2.0,
                74.0,
                20,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(x, 0.0, DIVERTER_Z / 2.0 + 8.0);
    }
    bores
}

fn diverter_valve_socket_cuts() -> Part {
    let mut sockets = Part::empty(format!("{PREFIX}_diverter_valve_socket_cuts"));
    for valve in 0..DIVERTER_VALVES {
        sockets = sockets
            + centered_cube(
                format!("{PREFIX}_diverter_valve_body_socket_{valve}"),
                54.0,
                54.0,
                26.0,
            )
            .translate(
                lane_x(valve, DIVERTER_VALVES, DIVERTER_PITCH_X),
                0.0,
                DIVERTER_Z - 10.0,
            );
    }
    sockets
}

fn diverter_valve_actuator_mockups() -> Part {
    let mut actuators = Part::empty(format!("{PREFIX}_diverter_valve_actuator_mockups"));
    for valve in 0..DIVERTER_VALVES {
        actuators = actuators
            + centered_cylinder(
                format!("{PREFIX}_diverter_valve_actuator_knob_{valve}"),
                VALVE_ACTUATOR_D / 2.0,
                20.0,
                32,
            )
            .translate(
                lane_x(valve, DIVERTER_VALVES, DIVERTER_PITCH_X),
                0.0,
                DIVERTER_Z + 10.0,
            );
    }
    actuators
}

fn relief_check_windows() -> Part {
    let mut windows = Part::empty(format!("{PREFIX}_relief_check_windows"));
    for window in 0..RELIEF_CHECK_WINDOWS {
        windows = windows
            + centered_cube(
                format!("{PREFIX}_relief_check_window_{window}"),
                44.0,
                12.0,
                16.0,
            )
            .translate(
                lane_x(window, RELIEF_CHECK_WINDOWS, DIVERTER_PITCH_X),
                DIVERTER_Y / 2.0 - 24.0,
                DIVERTER_Z + 8.0,
            );
    }
    windows
}

fn quarantine_route_arrows() -> Part {
    let mut arrows = Part::empty(format!("{PREFIX}_quarantine_route_arrows"));
    for valve in 0..DIVERTER_VALVES {
        let x = lane_x(valve, DIVERTER_VALVES, DIVERTER_PITCH_X);
        arrows = arrows
            + centered_cube(
                format!("{PREFIX}_diverter_route_arrow_stem_{valve}"),
                8.0,
                54.0,
                8.0,
            )
            .translate(x, -DIVERTER_Y / 2.0 + 42.0, DIVERTER_Z + 4.0)
            + centered_cube(
                format!("{PREFIX}_diverter_route_arrow_head_{valve}"),
                26.0,
                12.0,
                8.0,
            )
            .translate(x, -DIVERTER_Y / 2.0 + 14.0, DIVERTER_Z + 4.0);
    }
    arrows
}

fn diverter_position_witness_tabs() -> Part {
    let mut tabs = Part::empty(format!("{PREFIX}_diverter_position_witness_tabs"));
    for valve in 0..DIVERTER_VALVES {
        let x = lane_x(valve, DIVERTER_VALVES, DIVERTER_PITCH_X);
        tabs = tabs
            + centered_cube(
                format!("{PREFIX}_diverter_normal_position_tab_{valve}"),
                30.0,
                10.0,
                7.0,
            )
            .translate(x - 18.0, 48.0, DIVERTER_Z + 3.5)
            + centered_cube(
                format!("{PREFIX}_diverter_alarm_position_tab_{valve}"),
                30.0,
                10.0,
                12.0,
            )
            .translate(x + 18.0, 48.0, DIVERTER_Z + 6.0);
    }
    tabs
}

fn vent_filter_backstop() -> Part {
    let plenum = centered_cube(
        format!("{PREFIX}_vent_filter_backstop_plenum"),
        VENT_X,
        VENT_Y,
        VENT_Z,
    )
    .translate(0.0, 0.0, VENT_Z / 2.0);
    let plenum_hollow = centered_cube(
        format!("{PREFIX}_vent_filter_backstop_plenum_hollow"),
        VENT_X - 52.0,
        VENT_Y - 52.0,
        VENT_Z - 34.0,
    )
    .translate(0.0, 0.0, VENT_Z / 2.0 + 6.0);

    plenum - plenum_hollow - vent_filter_socket_cuts()
        + vent_filter_cartridges()
        + anti_siphon_witness_windows()
        + condensate_trap_cup()
        + vent_pressure_tap_block()
}

fn vent_filter_socket_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_vent_filter_socket_cuts"));
    for filter in 0..VENT_FILTERS {
        cuts = cuts
            + centered_cylinder(
                format!("{PREFIX}_vent_filter_socket_cut_{filter}"),
                FILTER_D / 2.0 + 2.0,
                VENT_Y + 14.0,
                32,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(lane_x(filter, VENT_FILTERS, 74.0), 0.0, VENT_Z - 30.0);
    }
    cuts
}

fn vent_filter_cartridges() -> Part {
    let mut cartridges = Part::empty(format!("{PREFIX}_vent_filter_cartridge_mockups"));
    for filter in 0..VENT_FILTERS {
        cartridges = cartridges
            + centered_cylinder(
                format!("{PREFIX}_hydrophobic_vent_filter_mockup_{filter}"),
                FILTER_D / 2.0,
                FILTER_LENGTH,
                36,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(lane_x(filter, VENT_FILTERS, 74.0), 0.0, VENT_Z - 30.0)
            + centered_cube(
                format!("{PREFIX}_vent_filter_label_land_{filter}"),
                52.0,
                18.0,
                5.0,
            )
            .translate(
                lane_x(filter, VENT_FILTERS, 74.0),
                -VENT_Y / 2.0 - 16.0,
                VENT_Z - 8.0,
            );
    }
    cartridges
}

fn anti_siphon_witness_windows() -> Part {
    let mut windows = Part::empty(format!("{PREFIX}_anti_siphon_witness_windows"));
    for window in 0..ANTI_SIPHON_WINDOWS {
        windows = windows
            + centered_cube(
                format!("{PREFIX}_anti_siphon_air_gap_window_{window}"),
                54.0,
                10.0,
                28.0,
            )
            .translate(
                lane_x(window, ANTI_SIPHON_WINDOWS, 86.0),
                VENT_Y / 2.0 + 7.0,
                72.0,
            );
    }
    windows
}

fn condensate_trap_cup() -> Part {
    let cup = centered_cylinder(format!("{PREFIX}_vent_condensate_trap_cup"), 34.0, 54.0, 40)
        .translate(-VENT_X / 2.0 + 56.0, 0.0, 28.0);
    let bore = centered_cylinder(
        format!("{PREFIX}_vent_condensate_trap_cup_bore"),
        24.0,
        58.0,
        36,
    )
    .translate(-VENT_X / 2.0 + 56.0, 0.0, 34.0);
    let drain = centered_cylinder(
        format!("{PREFIX}_vent_condensate_drain_bore"),
        4.0,
        58.0,
        16,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-VENT_X / 2.0 + 56.0, -18.0, 14.0);
    cup - bore - drain
}

fn vent_pressure_tap_block() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_vent_pressure_tap_block"),
        72.0,
        28.0,
        42.0,
    )
    .translate(VENT_X / 2.0 - 54.0, 0.0, 40.0);
    let tap = centered_cylinder(format!("{PREFIX}_vent_pressure_tap_bore"), 3.0, 34.0, 16)
        .rotate(90.0, 0.0, 0.0)
        .translate(VENT_X / 2.0 - 54.0, 0.0, 40.0);
    body - tap
}

fn occlusion_coupon_carousel() -> Part {
    let plate = centered_cube(
        format!("{PREFIX}_occlusion_coupon_carousel_plate"),
        OCCLUSION_X,
        OCCLUSION_Y,
        OCCLUSION_Z,
    )
    .translate(0.0, 0.0, OCCLUSION_Z / 2.0);
    let center_bore = centered_cylinder(
        format!("{PREFIX}_occlusion_coupon_carousel_pivot_bore"),
        16.0,
        OCCLUSION_Z + 10.0,
        36,
    )
    .translate(0.0, 0.0, OCCLUSION_Z / 2.0);

    plate - center_bore - occlusion_coupon_slots()
        + occlusion_coupon_tokens()
        + occlusion_percent_steps()
        + coupon_parking_latches()
}

fn occlusion_coupon_slots() -> Part {
    let mut slots = Part::empty(format!("{PREFIX}_occlusion_coupon_slots"));
    for coupon in 0..OCCLUSION_COUPONS {
        slots = slots
            + centered_cube(
                format!("{PREFIX}_occlusion_coupon_slot_{coupon}"),
                42.0,
                92.0,
                22.0,
            )
            .translate(
                lane_x(coupon, OCCLUSION_COUPONS, OCCLUSION_PITCH_X),
                0.0,
                OCCLUSION_Z - 8.0,
            );
    }
    slots
}

fn occlusion_coupon_tokens() -> Part {
    let mut tokens = Part::empty(format!("{PREFIX}_occlusion_coupon_tokens"));
    for coupon in 0..OCCLUSION_COUPONS {
        let x = lane_x(coupon, OCCLUSION_COUPONS, OCCLUSION_PITCH_X);
        tokens = tokens
            + centered_cube(
                format!("{PREFIX}_occlusion_level_coupon_{coupon}"),
                34.0,
                22.0 + coupon as f64 * 10.0,
                10.0,
            )
            .translate(x, 0.0, OCCLUSION_Z + 5.0)
            - centered_cylinder(
                format!("{PREFIX}_occlusion_level_coupon_bore_{coupon}"),
                2.2 + coupon as f64 * 0.7,
                16.0,
                14,
            )
            .translate(x, 0.0, OCCLUSION_Z + 5.0);
    }
    tokens
}

fn occlusion_percent_steps() -> Part {
    let mut steps = Part::empty(format!("{PREFIX}_occlusion_percent_steps"));
    for coupon in 0..OCCLUSION_COUPONS {
        steps = steps
            + centered_cube(
                format!("{PREFIX}_occlusion_percent_step_{coupon}"),
                28.0,
                10.0,
                5.0 + coupon as f64 * 3.0,
            )
            .translate(
                lane_x(coupon, OCCLUSION_COUPONS, OCCLUSION_PITCH_X),
                -OCCLUSION_Y / 2.0 + 24.0,
                OCCLUSION_Z + 2.5 + coupon as f64 * 1.5,
            );
    }
    steps
}

fn coupon_parking_latches() -> Part {
    let mut latches = Part::empty(format!("{PREFIX}_occlusion_coupon_parking_latches"));
    for coupon in 0..OCCLUSION_COUPONS {
        latches = latches
            + centered_cube(
                format!("{PREFIX}_occlusion_coupon_parking_latch_{coupon}"),
                24.0,
                12.0,
                16.0,
            )
            .translate(
                lane_x(coupon, OCCLUSION_COUPONS, OCCLUSION_PITCH_X),
                OCCLUSION_Y / 2.0 - 22.0,
                OCCLUSION_Z + 8.0,
            );
    }
    latches
}

fn calibration_reference_column() -> Part {
    let base = centered_cube(
        format!("{PREFIX}_calibration_reference_column_base"),
        COLUMN_BASE_X,
        COLUMN_BASE_Y,
        COLUMN_BASE_Z,
    )
    .translate(0.0, 0.0, COLUMN_BASE_Z / 2.0);
    let mast = centered_cylinder(
        format!("{PREFIX}_vertical_head_pressure_reference_column"),
        COLUMN_D / 2.0,
        COLUMN_HEIGHT,
        44,
    )
    .translate(-66.0, 0.0, COLUMN_BASE_Z + COLUMN_HEIGHT / 2.0);
    let guard = centered_cylinder(
        format!("{PREFIX}_reference_column_clear_guard_tube"),
        COLUMN_D / 2.0 + 10.0,
        COLUMN_HEIGHT - 24.0,
        44,
    )
    .translate(-66.0, 0.0, COLUMN_BASE_Z + COLUMN_HEIGHT / 2.0);
    let guard_bore = centered_cylinder(
        format!("{PREFIX}_reference_column_guard_tube_opening"),
        COLUMN_D / 2.0 + 4.0,
        COLUMN_HEIGHT - 12.0,
        44,
    )
    .translate(-66.0, 0.0, COLUMN_BASE_Z + COLUMN_HEIGHT / 2.0);

    base + mast + (guard - guard_bore) + column_tick_marks() + reference_weight_nests()
}

fn column_tick_marks() -> Part {
    let mut ticks = Part::empty(format!("{PREFIX}_reference_column_tick_marks"));
    for tick in 0..COLUMN_TICKS {
        ticks = ticks
            + centered_cube(
                format!("{PREFIX}_reference_column_tick_{tick}"),
                48.0,
                4.0,
                4.0,
            )
            .translate(
                -22.0,
                -COLUMN_D / 2.0 - 8.0,
                COLUMN_BASE_Z + 42.0 + tick as f64 * 34.0,
            );
    }
    ticks
}

fn reference_weight_nests() -> Part {
    let mut nests = Part::empty(format!("{PREFIX}_reference_pressure_weight_nests"));
    for nest in 0..REFERENCE_WEIGHT_NESTS {
        let y = lane_x(nest, REFERENCE_WEIGHT_NESTS, 42.0);
        nests = nests
            + centered_cylinder(
                format!("{PREFIX}_reference_weight_nest_{nest}"),
                16.0,
                12.0,
                32,
            )
            .translate(76.0, y, COLUMN_BASE_Z + 6.0)
            - centered_cylinder(
                format!("{PREFIX}_reference_weight_nest_center_relief_{nest}"),
                9.0,
                14.0,
                24,
            )
            .translate(76.0, y, COLUMN_BASE_Z + 8.0);
    }
    nests
}

fn event_logger_timestamp_docks() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_event_logger_timestamp_dock_body"),
        LOGGER_X,
        LOGGER_Y,
        LOGGER_Z,
    )
    .translate(0.0, 0.0, LOGGER_Z / 2.0);

    body - logger_dock_cuts()
        + logger_dock_rims()
        + time_sync_dock_blocks()
        + event_token_slots()
        + custody_card_lands()
}

fn logger_dock_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_event_logger_dock_cuts"));
    for dock in 0..LOGGER_DOCKS {
        cuts = cuts
            + centered_cube(
                format!("{PREFIX}_event_logger_dock_cut_{dock}"),
                48.0,
                66.0,
                24.0,
            )
            .translate(lane_x(dock, LOGGER_DOCKS, 72.0), 20.0, LOGGER_Z - 10.0);
    }
    cuts
}

fn logger_dock_rims() -> Part {
    let mut rims = Part::empty(format!("{PREFIX}_event_logger_dock_rims"));
    for dock in 0..LOGGER_DOCKS {
        rims = rims
            + centered_cube(
                format!("{PREFIX}_event_logger_dock_rim_{dock}"),
                56.0,
                74.0,
                6.0,
            )
            .translate(lane_x(dock, LOGGER_DOCKS, 72.0), 20.0, LOGGER_Z + 3.0);
    }
    rims
}

fn time_sync_dock_blocks() -> Part {
    let mut docks = Part::empty(format!("{PREFIX}_time_sync_dock_blocks"));
    for dock in 0..TIME_SYNC_DOCKS {
        docks = docks
            + centered_cylinder(
                format!("{PREFIX}_time_sync_reference_dock_{dock}"),
                15.0,
                14.0,
                32,
            )
            .translate(
                -40.0 + dock as f64 * 80.0,
                -LOGGER_Y / 2.0 + 30.0,
                LOGGER_Z + 7.0,
            );
    }
    docks
}

fn event_token_slots() -> Part {
    let mut slots = Part::empty(format!("{PREFIX}_event_token_slots"));
    for slot in 0..EVENT_TOKEN_SLOTS {
        slots = slots
            + centered_cube(
                format!("{PREFIX}_event_sequence_token_slot_{slot}"),
                30.0,
                12.0,
                8.0,
            )
            .translate(lane_x(slot, EVENT_TOKEN_SLOTS, 38.0), -20.0, LOGGER_Z + 4.0);
    }
    slots
}

fn custody_card_lands() -> Part {
    let before = centered_cube(
        format!("{PREFIX}_pre_challenge_record_card_land"),
        112.0,
        24.0,
        6.0,
    )
    .translate(-86.0, -LOGGER_Y / 2.0 + 60.0, LOGGER_Z + 3.0);
    let after = centered_cube(
        format!("{PREFIX}_post_challenge_record_card_land"),
        112.0,
        24.0,
        6.0,
    )
    .translate(86.0, -LOGGER_Y / 2.0 + 60.0, LOGGER_Z + 3.0);
    before + after
}

fn release_hold_reject_lanes() -> Part {
    let base = centered_cube(
        format!("{PREFIX}_release_hold_reject_lane_base"),
        GATES_X,
        GATES_Y,
        GATES_Z,
    )
    .translate(0.0, 0.0, GATES_Z / 2.0);

    base - disposition_lane_cuts()
        + disposition_lane_rails()
        + quarantine_token_pockets()
        + gate_stop_blocks()
}

fn disposition_lane_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_disposition_lane_cuts"));
    for lane in 0..DISPOSITION_LANES {
        cuts = cuts
            + centered_cube(
                format!("{PREFIX}_disposition_lane_{lane}_tray_cut"),
                86.0,
                110.0,
                18.0,
            )
            .translate(lane_x(lane, DISPOSITION_LANES, 108.0), 0.0, GATES_Z - 8.0);
    }
    cuts
}

fn disposition_lane_rails() -> Part {
    let mut rails = Part::empty(format!("{PREFIX}_disposition_lane_rails"));
    for lane in 0..DISPOSITION_LANES {
        let x = lane_x(lane, DISPOSITION_LANES, 108.0);
        rails = rails
            + centered_cube(
                format!("{PREFIX}_disposition_lane_{lane}_left_rail"),
                8.0,
                120.0,
                24.0,
            )
            .translate(x - 50.0, 0.0, GATES_Z + 12.0)
            + centered_cube(
                format!("{PREFIX}_disposition_lane_{lane}_right_rail"),
                8.0,
                120.0,
                24.0,
            )
            .translate(x + 50.0, 0.0, GATES_Z + 12.0);
    }
    rails
}

fn quarantine_token_pockets() -> Part {
    let mut pockets = Part::empty(format!("{PREFIX}_quarantine_token_pockets"));
    for lane in 0..DISPOSITION_LANES {
        for token in 0..QUARANTINE_TOKENS_PER_LANE {
            pockets = pockets
                + centered_cube(
                    format!("{PREFIX}_lane_{lane}_quarantine_token_pocket_{token}"),
                    18.0,
                    12.0,
                    8.0,
                )
                .translate(
                    lane_x(lane, DISPOSITION_LANES, 108.0) - 36.0 + token as f64 * 24.0,
                    GATES_Y / 2.0 - 24.0,
                    GATES_Z + 4.0,
                );
        }
    }
    pockets
}

fn gate_stop_blocks() -> Part {
    let release = centered_cube(
        format!("{PREFIX}_release_lane_stop_block"),
        80.0,
        10.0,
        32.0,
    )
    .translate(
        lane_x(0, DISPOSITION_LANES, 108.0),
        -GATES_Y / 2.0 + 18.0,
        GATES_Z + 16.0,
    );
    let hold = centered_cube(format!("{PREFIX}_hold_lane_stop_block"), 80.0, 10.0, 44.0).translate(
        lane_x(1, DISPOSITION_LANES, 108.0),
        -GATES_Y / 2.0 + 18.0,
        GATES_Z + 22.0,
    );
    let reject = centered_cube(format!("{PREFIX}_reject_lane_stop_block"), 80.0, 10.0, 56.0)
        .translate(
            lane_x(2, DISPOSITION_LANES, 108.0),
            -GATES_Y / 2.0 + 18.0,
            GATES_Z + 28.0,
        );
    release + hold + reject
}

fn witnessed_tubing_routes() -> Part {
    let mut routes = Part::empty(format!("{PREFIX}_witnessed_tubing_routes"));
    for lane in 0..ROUTE_GUARD_LANES {
        let y = -72.0 + lane as f64 * 18.0;
        routes = routes
            + centered_cube(
                format!("{PREFIX}_closed_waste_route_guard_lane_{lane}"),
                1080.0,
                5.0,
                8.0,
            )
            .translate(0.0, y, BASE_Z + 16.0);
    }

    let culture_side_guard = centered_cube(
        format!("{PREFIX}_culture_side_no_backpressure_guard_barrier"),
        24.0,
        610.0,
        64.0,
    )
    .translate(-218.0, -20.0, BASE_Z + 32.0);
    let waste_side_guard = centered_cube(
        format!("{PREFIX}_waste_side_alarm_trip_guard_barrier"),
        24.0,
        610.0,
        64.0,
    )
    .translate(218.0, -20.0, BASE_Z + 32.0);

    routes + culture_side_guard + waste_side_guard
}

fn robot_service_keepouts() -> Part {
    let front = centered_cube(
        format!("{PREFIX}_front_robot_approach_keepout"),
        STATION_X - 140.0,
        18.0,
        SERVICE_KEEP_OUT_Z,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 - FRONT_ROBOT_CLEARANCE / 2.0,
        SERVICE_KEEP_OUT_Z / 2.0,
    );
    let rear = centered_cube(
        format!("{PREFIX}_rear_filter_service_keepout"),
        STATION_X - 240.0,
        16.0,
        170.0,
    )
    .translate(0.0, STATION_Y / 2.0 + REAR_FILTER_CLEARANCE / 2.0, 85.0);
    let left = centered_cube(
        format!("{PREFIX}_left_waste_bag_service_keepout"),
        16.0,
        STATION_Y - 180.0,
        180.0,
    )
    .translate(-STATION_X / 2.0 - LEFT_WASTE_BAG_CLEARANCE / 2.0, 0.0, 90.0);
    let right = centered_cube(
        format!("{PREFIX}_right_sensor_service_keepout"),
        16.0,
        STATION_Y - 180.0,
        180.0,
    )
    .translate(
        STATION_X / 2.0 + RIGHT_SENSOR_SERVICE_CLEARANCE / 2.0,
        0.0,
        90.0,
    );
    let top = centered_cube(
        format!("{PREFIX}_top_column_service_keepout"),
        300.0,
        260.0,
        16.0,
    )
    .translate(
        COLUMN_POS.0,
        COLUMN_POS.1,
        BASE_Z + TOP_COLUMN_SERVICE_CLEARANCE,
    );
    front + rear + left + right + top
}

fn alarm_token_x(channel: usize) -> f64 {
    lane_x(channel, ALARM_CHANNELS, ALARM_TOKEN_PITCH_X) - 210.0
}

fn alarm_token_y(state: usize) -> f64 {
    lane_x(state, ALARM_STATES, ALARM_TOKEN_PITCH_Y)
}

fn lane_x(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn bag_slug(index: usize) -> &'static str {
    match index {
        0 => "primary_waste",
        1 => "secondary_waste",
        2 => "retain_sample",
        3 => "alarm_quarantine",
        _ => "extra",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_paths_are_stable_and_scoped() {
        assert_eq!(OUTPUTS.len(), 12);
        for output in OUTPUTS {
            assert!(output
                .starts_with("output/closed_perfusion_waste_backpressure_alarm_trip_station_"));
            assert!(output.ends_with(".stl"));
        }
    }

    #[test]
    fn required_features_cover_alarm_trip_reproducibility() {
        for required in [
            "backpressure_challenge_manifold",
            "pressure_trip_sensor_bank",
            "alarm_trip_interlock_panel",
            "relief_diverter_valve_bridge",
            "event_logger_timestamp_docks",
        ] {
            assert!(
                REQUIRED_FEATURES.contains(&required),
                "missing required feature {required}"
            );
        }
    }

    #[test]
    fn challenge_lanes_have_sensor_and_route_coverage() {
        assert_eq!(PRESSURE_SENSOR_COUNT, CHALLENGE_LANES);
        assert_eq!(RESTRICTOR_SLOTS, CHALLENGE_LANES);
        assert!(ROUTE_GUARD_LANES >= CHALLENGE_LANES);
        assert!(LEAK_SENSOR_WELLS >= ROUTE_GUARD_LANES);
    }

    #[test]
    fn alarm_trip_has_closed_path_disposition() {
        assert_eq!(ALARM_CHANNELS, DIVERTER_VALVES);
        assert_eq!(WASTE_BAG_NESTS + RETAIN_BAG_NESTS, DIVERTER_VALVES);
        assert_eq!(DISPOSITION_LANES, 3);
        assert!(QUARANTINE_TOKENS_PER_LANE >= ALARM_STATES);
    }

    #[test]
    fn layout_keeps_modules_inside_containment_without_overlap() {
        let rects = layout_rects();
        for rect in rects {
            assert!(rect.fits_inside_station(), "{rect:?} outside station");
        }

        for (i, left) in rects.iter().enumerate() {
            for right in rects.iter().skip(i + 1) {
                assert!(
                    !left.overlaps_with_clearance(*right, 10.0),
                    "{} overlaps {}",
                    left.name,
                    right.name
                );
            }
        }
    }

    #[test]
    fn service_clearances_cover_robot_and_column_access() {
        assert!(FRONT_ROBOT_CLEARANCE >= 400.0);
        assert!(REAR_FILTER_CLEARANCE >= 250.0);
        assert!(TOP_COLUMN_SERVICE_CLEARANCE > COLUMN_HEIGHT);
        assert_eq!(ROBOT_FIDUCIALS, 4);
    }
}
