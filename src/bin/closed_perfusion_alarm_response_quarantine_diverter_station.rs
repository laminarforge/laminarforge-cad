use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed perfusion alarm-response quarantine diverter station.
//
// Intent:
// - Model a contained alarm-response station for tissue-chip perfusion where
//   pressure, flow, bubble, and pH alarm inputs force fluid away from the
//   culture path into quarantine waste/retain bags while preserving a
//   bypass-safe pressure/flow path.
// - Keep the alarm input bank, diverter valve bank, quarantine bag hotel,
//   bypass-safe bridge, alarm token rail, manual-free reset interlock, event
//   logger docks, custody labels, and release/hold/reject gates mechanically
//   visible in one leak-tray fixture.
// - Represent station markings as CSG label lands, raised bars, gate tokens,
//   and geometric custody plaques so the exported STL set remains
//   self-describing without decal files.
//
// Research assumptions from the Exa pass:
// - Organ-on-chip perfusion literature emphasizes controlled flow, medium
//   refresh, waste removal, recirculation, pressure-driven multiplexing, and
//   sensitivity of tissue models to flow/pressure and shear conditions.
// - Perfusion bioprocess controllers commonly monitor pressure, flow, bubble
//   formation, pH/DO, and temperature while using pumps, pinch/selector valves,
//   single-use sensors, and collection bags for continuous perfusion workflows.
// - FDA 21 CFR 11.10 calls for validation, operational sequence checks,
//   authority/device checks, and secure time-stamped audit trails in closed
//   electronic-record systems; the geometry therefore includes interlock and
//   event-logger docks rather than an untraceable manual reset handle.
// - ISPE process-event guidance describes alarm and audit-trail exchange as
//   event-driven GMP-relevant data with alarm generated/acknowledged/cleared
//   lifecycle states reviewed for release; the station encodes token slots,
//   custody labels, and release/hold/reject gates around that lifecycle.
//
// This is architecture CAD only. It is not a validated sterile barrier,
// pressure safety device, clinical release workflow, controller, or wetted-path
// specification.

const PREFIX: &str = "closed_perfusion_alarm_response_quarantine_diverter_station";

const OUTPUTS: [&str; 12] = [
    "output/closed_perfusion_alarm_response_quarantine_diverter_station_base_leak_tray.stl",
    "output/closed_perfusion_alarm_response_quarantine_diverter_station_alarm_input_sensor_bank.stl",
    "output/closed_perfusion_alarm_response_quarantine_diverter_station_diverter_valve_bank.stl",
    "output/closed_perfusion_alarm_response_quarantine_diverter_station_quarantine_bag_hotel.stl",
    "output/closed_perfusion_alarm_response_quarantine_diverter_station_bypass_safe_path_bridge.stl",
    "output/closed_perfusion_alarm_response_quarantine_diverter_station_alarm_token_rail.stl",
    "output/closed_perfusion_alarm_response_quarantine_diverter_station_manual_free_reset_interlock.stl",
    "output/closed_perfusion_alarm_response_quarantine_diverter_station_event_logger_docks.stl",
    "output/closed_perfusion_alarm_response_quarantine_diverter_station_custody_label_lands.stl",
    "output/closed_perfusion_alarm_response_quarantine_diverter_station_release_hold_reject_gates.stl",
    "output/closed_perfusion_alarm_response_quarantine_diverter_station_robot_service_keepouts.stl",
    "output/closed_perfusion_alarm_response_quarantine_diverter_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "pressure_flow_bubble_ph_alarm_inputs",
    "diverter_valve_bank",
    "quarantine_waste_retain_bags",
    "bypass_safe_path",
    "alarm_token_rail",
    "manual_free_reset_interlock",
    "leak_tray",
    "custody_label_lands",
    "event_logger_docks",
    "release_hold_reject_gates",
    "robot_service_keepouts",
];

const STATION_X: f64 = 1360.0;
const STATION_Y: f64 = 860.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const SOCKET_DEPTH: f64 = 6.0;
const DRAIN_D: f64 = 12.0;
const MOUNT_HOLE_D: f64 = 6.6;

const ALARM_POS: (f64, f64) = (-455.0, 230.0);
const ALARM_X: f64 = 350.0;
const ALARM_Y: f64 = 240.0;
const ALARM_Z: f64 = 64.0;
const ALARM_INPUT_COUNT: usize = 4;
const ALARM_SENSOR_POCKETS: usize = ALARM_INPUT_COUNT;
const ALARM_CHANNEL_PITCH_X: f64 = 78.0;
const ALARM_TUBE_BORE_D: f64 = 6.4;
const ALARM_STATUS_WINDOWS: usize = ALARM_INPUT_COUNT;

const VALVE_POS: (f64, f64) = (0.0, 230.0);
const VALVE_X: f64 = 460.0;
const VALVE_Y: f64 = 240.0;
const VALVE_Z: f64 = 72.0;
const DIVERTER_VALVE_LANES: usize = ALARM_INPUT_COUNT * 2;
const DIVERTER_PAIRS: usize = ALARM_INPUT_COUNT;
const VALVE_PITCH_X: f64 = 50.0;
const VALVE_ACTUATOR_D: f64 = 26.0;
const VALVE_TUBE_BORE_D: f64 = 6.0;

const BYPASS_POS: (f64, f64) = (430.0, 230.0);
const BYPASS_X: f64 = 330.0;
const BYPASS_Y: f64 = 240.0;
const BYPASS_Z: f64 = 54.0;
const BYPASS_LANES: usize = ALARM_INPUT_COUNT;
const BYPASS_LANE_PITCH_X: f64 = 68.0;
const BYPASS_CHANNEL_W: f64 = 9.0;
const BYPASS_SAFE_PRESSURE_RELIEF_COUNT: usize = 2;
const BYPASS_CHECK_VALVE_WINDOWS: usize = BYPASS_LANES;

const BAG_POS: (f64, f64) = (-410.0, -45.0);
const BAG_X: f64 = 430.0;
const BAG_Y: f64 = 230.0;
const BAG_Z: f64 = 58.0;
const QUARANTINE_BAG_NESTS: usize = 4;
const WASTE_BAG_NESTS: usize = 2;
const RETAIN_BAG_NESTS: usize = 2;
const BAG_NEST_X: f64 = 154.0;
const BAG_NEST_Y: f64 = 72.0;
const BAG_PITCH_X: f64 = 186.0;
const BAG_PITCH_Y: f64 = 96.0;
const BAG_TUBE_PORT_D: f64 = 8.0;

const LOGGER_POS: (f64, f64) = (35.0, -45.0);
const LOGGER_X: f64 = 380.0;
const LOGGER_Y: f64 = 230.0;
const LOGGER_Z: f64 = 46.0;
const EVENT_LOGGER_DOCKS: usize = 4;
const LOGGER_PITCH_X: f64 = 82.0;
const LOGGER_POCKET_X: f64 = 54.0;
const LOGGER_POCKET_Y: f64 = 82.0;
const TIME_SYNC_DOCKS: usize = 2;

const RESET_POS: (f64, f64) = (430.0, -45.0);
const RESET_X: f64 = 330.0;
const RESET_Y: f64 = 230.0;
const RESET_Z: f64 = 62.0;
const RESET_INTERLOCK_INPUTS: usize = ALARM_INPUT_COUNT + 3;
const RESET_PIN_PITCH_X: f64 = 38.0;
const RESET_PIN_D: f64 = 10.0;
const NO_MANUAL_BYPASS_GUARDS: usize = 3;

const TOKEN_POS: (f64, f64) = (-445.0, -315.0);
const TOKEN_X: f64 = 370.0;
const TOKEN_Y: f64 = 140.0;
const TOKEN_Z: f64 = 34.0;
const TOKEN_STATES_PER_ALARM: usize = 3;
const ALARM_TOKEN_DOCKS: usize = ALARM_INPUT_COUNT * TOKEN_STATES_PER_ALARM;
const TOKEN_DOCK_X: f64 = 32.0;
const TOKEN_DOCK_Y: f64 = 24.0;
const TOKEN_PITCH_X: f64 = 78.0;
const TOKEN_STATE_PITCH_Y: f64 = 36.0;

const CUSTODY_POS: (f64, f64) = (-30.0, -315.0);
const CUSTODY_X: f64 = 390.0;
const CUSTODY_Y: f64 = 140.0;
const CUSTODY_Z: f64 = 12.0;
const CUSTODY_LABEL_LANDS: usize = 12;
const CUSTODY_CARD_SLOTS: usize = 4;
const CERTIFICATE_LANDS: usize = 3;
const LABEL_BAR_COUNT: usize = 8;

const GATES_POS: (f64, f64) = (430.0, -315.0);
const GATES_X: f64 = 330.0;
const GATES_Y: f64 = 140.0;
const GATES_Z: f64 = 44.0;
const DISPOSITION_GATE_COUNT: usize = 3;
const GATE_CAPACITY_PER_LANE: usize = 4;
const GATE_SLOT_X: f64 = 54.0;
const GATE_SLOT_Y: f64 = 24.0;
const GATE_PITCH_X: f64 = 100.0;

const ROBOT_FIDUCIAL_COUNT: usize = 4;
const LEAK_WITNESS_RAILS: usize = 8;
const BASE_GUTTER_COUNT: usize = 5;
const KEEP_OUT_ZONE_COUNT: usize = 5;
const FRONT_ROBOT_KEEP_OUT_Y: f64 = 150.0;
const REAR_SERVICE_KEEP_OUT_Y: f64 = 112.0;
const LEFT_BAG_SERVICE_KEEP_OUT_X: f64 = 150.0;
const RIGHT_VALVE_SERVICE_KEEP_OUT_X: f64 = 150.0;
const TOP_VALVE_LIFT_KEEP_OUT_Z: f64 = 300.0;

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
        self.center.0 - self.x / 2.0 >= -usable_x
            && self.center.0 + self.x / 2.0 <= usable_x
            && self.center.1 - self.y / 2.0 >= -usable_y
            && self.center.1 + self.y / 2.0 <= usable_y
    }

    fn overlaps_with_clearance(self, other: Rect, clearance: f64) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();
        dx < (self.x + other.x) / 2.0 + clearance && dy < (self.y + other.y) / 2.0 + clearance
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AlarmInput {
    Pressure,
    Flow,
    Bubble,
    Ph,
}

impl AlarmInput {
    fn all() -> [AlarmInput; ALARM_INPUT_COUNT] {
        [
            AlarmInput::Pressure,
            AlarmInput::Flow,
            AlarmInput::Bubble,
            AlarmInput::Ph,
        ]
    }

    fn index(self) -> usize {
        match self {
            AlarmInput::Pressure => 0,
            AlarmInput::Flow => 1,
            AlarmInput::Bubble => 2,
            AlarmInput::Ph => 3,
        }
    }

    fn slug(self) -> &'static str {
        match self {
            AlarmInput::Pressure => "pressure",
            AlarmInput::Flow => "flow",
            AlarmInput::Bubble => "bubble",
            AlarmInput::Ph => "ph",
        }
    }

    fn pocket_size(self) -> (f64, f64) {
        match self {
            AlarmInput::Pressure => (54.0, 34.0),
            AlarmInput::Flow => (58.0, 30.0),
            AlarmInput::Bubble => (50.0, 38.0),
            AlarmInput::Ph => (44.0, 44.0),
        }
    }

    fn token_height(self) -> f64 {
        match self {
            AlarmInput::Pressure => 8.0,
            AlarmInput::Flow => 10.0,
            AlarmInput::Bubble => 12.0,
            AlarmInput::Ph => 14.0,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DispositionGate {
    Release,
    Hold,
    Reject,
}

impl DispositionGate {
    fn all() -> [DispositionGate; DISPOSITION_GATE_COUNT] {
        [
            DispositionGate::Release,
            DispositionGate::Hold,
            DispositionGate::Reject,
        ]
    }

    fn index(self) -> usize {
        match self {
            DispositionGate::Release => 0,
            DispositionGate::Hold => 1,
            DispositionGate::Reject => 2,
        }
    }

    fn slug(self) -> &'static str {
        match self {
            DispositionGate::Release => "release",
            DispositionGate::Hold => "hold",
            DispositionGate::Reject => "reject",
        }
    }

    fn gate_height(self) -> f64 {
        match self {
            DispositionGate::Release => 22.0,
            DispositionGate::Hold => 34.0,
            DispositionGate::Reject => 52.0,
        }
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_leak_tray();
    export(OUTPUTS[0], &base);

    let alarms = alarm_input_sensor_bank();
    export(OUTPUTS[1], &alarms);

    let valves = diverter_valve_bank();
    export(OUTPUTS[2], &valves);

    let bags = quarantine_bag_hotel();
    export(OUTPUTS[3], &bags);

    let bypass = bypass_safe_path_bridge();
    export(OUTPUTS[4], &bypass);

    let tokens = alarm_token_rail();
    export(OUTPUTS[5], &tokens);

    let reset = manual_free_reset_interlock();
    export(OUTPUTS[6], &reset);

    let loggers = event_logger_docks();
    export(OUTPUTS[7], &loggers);

    let custody = custody_label_lands();
    export(OUTPUTS[8], &custody);

    let gates = release_hold_reject_gates();
    export(OUTPUTS[9], &gates);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[10], &keepouts);

    let assembly = base
        + alarms.translate(ALARM_POS.0, ALARM_POS.1, insert_z(ALARM_Z))
        + valves.translate(VALVE_POS.0, VALVE_POS.1, insert_z(VALVE_Z))
        + bags.translate(BAG_POS.0, BAG_POS.1, insert_z(BAG_Z))
        + bypass.translate(BYPASS_POS.0, BYPASS_POS.1, insert_z(BYPASS_Z))
        + tokens.translate(TOKEN_POS.0, TOKEN_POS.1, insert_z(TOKEN_Z))
        + reset.translate(RESET_POS.0, RESET_POS.1, insert_z(RESET_Z))
        + loggers.translate(LOGGER_POS.0, LOGGER_POS.1, insert_z(LOGGER_Z))
        + custody.translate(CUSTODY_POS.0, CUSTODY_POS.1, insert_z(CUSTODY_Z))
        + gates.translate(GATES_POS.0, GATES_POS.1, insert_z(GATES_Z))
        + keepouts;
    export(OUTPUTS[11], &assembly);

    println!();
    println!("Closed perfusion alarm-response quarantine diverter station:");
    println!(
        "  Footprint:                 {STATION_X:.0}mm x {STATION_Y:.0}mm leak tray with {LEAK_WITNESS_RAILS} witness rails and {BASE_GUTTER_COUNT} gutters"
    );
    println!(
        "  Alarm inputs:              {ALARM_INPUT_COUNT} channels (pressure, flow, bubble, pH) with {ALARM_SENSOR_POCKETS} sensor pockets"
    );
    println!(
        "  Diversion:                 {DIVERTER_VALVE_LANES} pinch/selector valve lanes in {DIVERTER_PAIRS} alarm pairs, {BYPASS_LANES} bypass-safe lanes"
    );
    println!(
        "  Quarantine:                {WASTE_BAG_NESTS} waste bag nests, {RETAIN_BAG_NESTS} retain bag nests, custody lands and leak capture"
    );
    println!(
        "  Interlocks/logging:        {RESET_INTERLOCK_INPUTS} reset interlock pins, {EVENT_LOGGER_DOCKS} event logger docks, {ALARM_TOKEN_DOCKS} alarm lifecycle token docks"
    );
    println!(
        "  Disposition/clearance:     {} release/hold/reject gates with total token capacity {}, {KEEP_OUT_ZONE_COUNT} keepout zones, {ROBOT_FIDUCIAL_COUNT} robot fiducials",
        DispositionGate::all().len(),
        total_gate_capacity()
    );
    println!("  Required feature groups:   {}", REQUIRED_FEATURES.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn insert_z(height: f64) -> f64 {
    BASE_Z / 2.0 + height / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn layout_rects() -> [Rect; 9] {
    [
        Rect {
            name: "alarm_input_sensor_bank",
            center: ALARM_POS,
            x: ALARM_X,
            y: ALARM_Y,
        },
        Rect {
            name: "diverter_valve_bank",
            center: VALVE_POS,
            x: VALVE_X,
            y: VALVE_Y,
        },
        Rect {
            name: "bypass_safe_path_bridge",
            center: BYPASS_POS,
            x: BYPASS_X,
            y: BYPASS_Y,
        },
        Rect {
            name: "quarantine_bag_hotel",
            center: BAG_POS,
            x: BAG_X,
            y: BAG_Y,
        },
        Rect {
            name: "event_logger_docks",
            center: LOGGER_POS,
            x: LOGGER_X,
            y: LOGGER_Y,
        },
        Rect {
            name: "manual_free_reset_interlock",
            center: RESET_POS,
            x: RESET_X,
            y: RESET_Y,
        },
        Rect {
            name: "alarm_token_rail",
            center: TOKEN_POS,
            x: TOKEN_X,
            y: TOKEN_Y,
        },
        Rect {
            name: "custody_label_lands",
            center: CUSTODY_POS,
            x: CUSTODY_X,
            y: CUSTODY_Y,
        },
        Rect {
            name: "release_hold_reject_gates",
            center: GATES_POS,
            x: GATES_X,
            y: GATES_Y,
        },
    ]
}

fn assert_design_constraints() {
    let rects = layout_rects();
    for rect in rects {
        assert!(
            rect.fits_inside_station(),
            "{} exceeds usable station envelope",
            rect.name
        );
    }

    for i in 0..rects.len() {
        for j in (i + 1)..rects.len() {
            assert!(
                !rects[i].overlaps_with_clearance(rects[j], 14.0),
                "{} overlaps {}",
                rects[i].name,
                rects[j].name
            );
        }
    }

    assert_eq!(AlarmInput::all().len(), ALARM_INPUT_COUNT);
    assert_eq!(ALARM_STATUS_WINDOWS, ALARM_INPUT_COUNT);
    assert_eq!(DIVERTER_VALVE_LANES, ALARM_INPUT_COUNT * 2);
    assert_eq!(DIVERTER_PAIRS, ALARM_INPUT_COUNT);
    assert_eq!(BYPASS_LANES, ALARM_INPUT_COUNT);
    assert_eq!(QUARANTINE_BAG_NESTS, WASTE_BAG_NESTS + RETAIN_BAG_NESTS);
    assert_eq!(
        ALARM_TOKEN_DOCKS,
        ALARM_INPUT_COUNT * TOKEN_STATES_PER_ALARM
    );
    assert_eq!(EVENT_LOGGER_DOCKS, AlarmInput::all().len());
    assert_eq!(DispositionGate::all().len(), DISPOSITION_GATE_COUNT);
    assert_eq!(ROBOT_FIDUCIAL_COUNT, 4);
    assert_eq!(KEEP_OUT_ZONE_COUNT, 5);
    assert!(RESET_INTERLOCK_INPUTS > ALARM_INPUT_COUNT);
    assert!(total_gate_capacity() >= ALARM_INPUT_COUNT + QUARANTINE_BAG_NESTS);
}

fn base_leak_tray() -> Part {
    let deck = centered_cube(format!("{PREFIX}_base_floor"), STATION_X, STATION_Y, BASE_Z);
    let main_sump = centered_cube(
        format!("{PREFIX}_base_recessed_leak_sump"),
        STATION_X - 116.0,
        STATION_Y - 112.0,
        8.0,
    )
    .translate(0.0, -6.0, BASE_Z / 2.0 - 4.0);
    let wet_top_recess = centered_cube(
        format!("{PREFIX}_base_alarm_valve_wet_zone_recess"),
        STATION_X - 180.0,
        242.0,
        8.0,
    )
    .translate(0.0, 230.0, BASE_Z / 2.0 - 4.0);
    let quarantine_recess = centered_cube(
        format!("{PREFIX}_base_quarantine_row_recess"),
        STATION_X - 180.0,
        228.0,
        8.0,
    )
    .translate(0.0, -45.0, BASE_Z / 2.0 - 4.0);
    let disposition_recess = centered_cube(
        format!("{PREFIX}_base_disposition_row_recess"),
        STATION_X - 200.0,
        132.0,
        8.0,
    )
    .translate(0.0, -315.0, BASE_Z / 2.0 - 4.0);
    let drain = centered_cylinder(
        format!("{PREFIX}_base_front_quarantine_drain"),
        DRAIN_D / 2.0,
        54.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 88.0, -STATION_Y / 2.0 - 3.0, -1.0);
    let bypass_drain = centered_cylinder(
        format!("{PREFIX}_base_bypass_low_point_drain"),
        DRAIN_D / 2.0,
        54.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 156.0, -STATION_Y / 2.0 - 3.0, -1.0);

    deck - main_sump
        - wet_top_recess
        - quarantine_recess
        - disposition_recess
        - drain
        - bypass_drain
        - insert_sockets()
        - mounting_slots()
        + perimeter_rims()
        + zone_dividers()
        + leak_witness_rails()
        + base_flow_gutters()
        + robot_fiducials()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty(format!("{PREFIX}_base_insert_sockets"));
    for rect in layout_rects() {
        sockets = sockets
            + centered_cube(
                format!("{PREFIX}_{}_socket_recess", rect.name),
                rect.x + 8.0,
                rect.y + 8.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(
                rect.center.0,
                rect.center.1,
                BASE_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.2,
            );
    }
    sockets
}

fn mounting_slots() -> Part {
    let mut slots = Part::empty(format!("{PREFIX}_base_mounting_slots"));
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 58.0), -(STATION_Y / 2.0 - 52.0)),
        (STATION_X / 2.0 - 58.0, -(STATION_Y / 2.0 - 52.0)),
        (-(STATION_X / 2.0 - 58.0), STATION_Y / 2.0 - 52.0),
        (STATION_X / 2.0 - 58.0, STATION_Y / 2.0 - 52.0),
        (0.0, STATION_Y / 2.0 - 52.0),
        (0.0, -(STATION_Y / 2.0 - 52.0)),
        (-(STATION_X / 2.0 - 58.0), 0.0),
        (STATION_X / 2.0 - 58.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        slots = slots
            + centered_cylinder(
                format!("{PREFIX}_base_m6_mount_clearance_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 4.0,
                28,
            )
            .translate(*x, *y, 0.0)
            + centered_cube(
                format!("{PREFIX}_base_m6_mount_slot_relief_{i}"),
                28.0,
                MOUNT_HOLE_D + 0.4,
                BASE_Z + 4.0,
            )
            .translate(*x, *y, 0.0);
    }
    slots
}

fn perimeter_rims() -> Part {
    let left = centered_cube(
        format!("{PREFIX}_base_left_containment_rim"),
        RIM_W,
        STATION_Y - 54.0,
        RIM_Z,
    )
    .translate(
        -(STATION_X / 2.0 - RIM_W / 2.0),
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube(
        format!("{PREFIX}_base_right_containment_rim"),
        RIM_W,
        STATION_Y - 54.0,
        RIM_Z,
    )
    .translate(
        STATION_X / 2.0 - RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let rear = centered_cube(
        format!("{PREFIX}_base_rear_containment_rim"),
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
        format!("{PREFIX}_base_front_low_robot_service_lip"),
        STATION_X - 200.0,
        14.0,
        22.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 26.0, BASE_Z / 2.0 + 11.0);

    left + right + rear + front_low_lip
}

fn zone_dividers() -> Part {
    let top_to_middle = centered_cube(
        format!("{PREFIX}_base_alarm_row_to_quarantine_row_divider"),
        STATION_X - 180.0,
        12.0,
        30.0,
    )
    .translate(0.0, 92.0, BASE_Z / 2.0 + 15.0);
    let middle_to_bottom = centered_cube(
        format!("{PREFIX}_base_quarantine_row_to_disposition_row_divider"),
        STATION_X - 180.0,
        12.0,
        30.0,
    )
    .translate(0.0, -195.0, BASE_Z / 2.0 + 15.0);
    let alarm_to_valve = centered_cube(
        format!("{PREFIX}_base_alarm_to_valve_divider"),
        10.0,
        238.0,
        28.0,
    )
    .translate(-245.0, 230.0, BASE_Z / 2.0 + 14.0);
    let valve_to_bypass = centered_cube(
        format!("{PREFIX}_base_valve_to_bypass_divider"),
        10.0,
        238.0,
        28.0,
    )
    .translate(250.0, 230.0, BASE_Z / 2.0 + 14.0);
    let bags_to_loggers = centered_cube(
        format!("{PREFIX}_base_bag_to_logger_divider"),
        10.0,
        226.0,
        28.0,
    )
    .translate(-180.0, -45.0, BASE_Z / 2.0 + 14.0);
    let loggers_to_reset = centered_cube(
        format!("{PREFIX}_base_logger_to_reset_divider"),
        10.0,
        226.0,
        28.0,
    )
    .translate(250.0, -45.0, BASE_Z / 2.0 + 14.0);

    top_to_middle
        + middle_to_bottom
        + alarm_to_valve
        + valve_to_bypass
        + bags_to_loggers
        + loggers_to_reset
}

fn leak_witness_rails() -> Part {
    let mut rails = Part::empty(format!("{PREFIX}_base_leak_witness_rails"));
    for i in 0..LEAK_WITNESS_RAILS {
        let x = centered_index(i, LEAK_WITNESS_RAILS, 148.0);
        rails = rails
            + centered_cube(
                format!("{PREFIX}_base_front_leak_witness_rail_{i}"),
                98.0,
                5.0,
                7.0,
            )
            .translate(x, -392.0, BASE_Z / 2.0 + 3.5);
    }
    rails
}

fn base_flow_gutters() -> Part {
    let mut gutters = Part::empty(format!("{PREFIX}_base_flow_gutters"));
    for (i, (name, x, y, sx, sy)) in [
        ("alarm_to_valve", -245.0, 94.0, 8.0, 272.0),
        ("valve_to_bypass", 250.0, 94.0, 8.0, 272.0),
        ("valve_to_bags", -92.0, 92.0, 310.0, 8.0),
        ("bypass_to_reset", 430.0, 92.0, 8.0, 276.0),
        ("bags_to_disposition", -245.0, -195.0, 310.0, 8.0),
    ]
    .iter()
    .enumerate()
    {
        gutters = gutters
            + centered_cube(
                format!("{PREFIX}_base_{name}_raised_flow_gutter_{i}"),
                *sx,
                *sy,
                6.0,
            )
            .translate(*x, *y, BASE_Z / 2.0 + 3.0);
    }
    gutters
}

fn robot_fiducials() -> Part {
    let mut fiducials = Part::empty(format!("{PREFIX}_base_robot_fiducials"));
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 78.0), STATION_Y / 2.0 - 82.0),
        (STATION_X / 2.0 - 78.0, STATION_Y / 2.0 - 82.0),
        (-(STATION_X / 2.0 - 78.0), -(STATION_Y / 2.0 - 82.0)),
        (STATION_X / 2.0 - 78.0, -(STATION_Y / 2.0 - 82.0)),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + fiducial_disc(format!("{PREFIX}_base_robot_fiducial_{i}")).translate(
                *x,
                *y,
                BASE_Z / 2.0 + 3.0,
            );
    }
    fiducials
}

fn alarm_input_sensor_bank() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_alarm_input_sensor_bank_body"),
        ALARM_X,
        ALARM_Y,
        ALARM_Z,
    );
    let underside_relief = centered_cube(
        format!("{PREFIX}_alarm_input_sensor_bank_lightening_pocket"),
        ALARM_X - 46.0,
        ALARM_Y - 42.0,
        18.0,
    )
    .translate(0.0, 0.0, -ALARM_Z / 2.0 + 9.0);

    body - underside_relief - alarm_sensor_pocket_cuts() - alarm_input_tube_bores()
        + alarm_sensor_rims()
        + alarm_input_bulkheads()
        + alarm_status_windows()
        + alarm_input_label_lands()
}

fn alarm_sensor_pocket_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_alarm_sensor_pocket_cuts"));
    for input in AlarmInput::all() {
        let x = alarm_input_x(input);
        let (sx, sy) = input.pocket_size();
        cuts = cuts
            + centered_cube(
                format!("{PREFIX}_{}_sensor_pocket_cut", input.slug()),
                sx,
                sy,
                22.0,
            )
            .translate(x, 20.0, ALARM_Z / 2.0 - 10.0)
            + centered_cylinder(
                format!("{PREFIX}_{}_sensor_cable_bore", input.slug()),
                4.0,
                58.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, -ALARM_Y / 2.0 + 20.0, ALARM_Z / 2.0 - 12.0);
    }
    cuts
}

fn alarm_input_tube_bores() -> Part {
    let mut bores = Part::empty(format!("{PREFIX}_alarm_input_tube_bores"));
    for input in AlarmInput::all() {
        let x = alarm_input_x(input);
        let rear = centered_cylinder(
            format!("{PREFIX}_{}_rear_process_tube_bore", input.slug()),
            ALARM_TUBE_BORE_D / 2.0,
            44.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, ALARM_Y / 2.0 - 16.0, ALARM_Z / 2.0 - 16.0);
        let front = centered_cylinder(
            format!("{PREFIX}_{}_front_alarm_sample_bore", input.slug()),
            ALARM_TUBE_BORE_D / 2.0,
            44.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -ALARM_Y / 2.0 + 16.0, ALARM_Z / 2.0 - 16.0);
        bores = bores + rear + front;
    }
    bores
}

fn alarm_sensor_rims() -> Part {
    let mut rims = Part::empty(format!("{PREFIX}_alarm_sensor_rims"));
    for input in AlarmInput::all() {
        let x = alarm_input_x(input);
        let (sx, sy) = input.pocket_size();
        let outer = centered_cube(
            format!("{PREFIX}_{}_sensor_outer_rim", input.slug()),
            sx + 10.0,
            sy + 10.0,
            5.0,
        )
        .translate(x, 20.0, ALARM_Z / 2.0 + 2.5);
        let inner = centered_cube(
            format!("{PREFIX}_{}_sensor_inner_window", input.slug()),
            sx - 8.0,
            sy - 8.0,
            6.0,
        )
        .translate(x, 20.0, ALARM_Z / 2.0 + 2.8);
        rims = rims + (outer - inner);
    }
    rims
}

fn alarm_input_bulkheads() -> Part {
    let mut bulkheads = Part::empty(format!("{PREFIX}_alarm_input_bulkheads"));
    for input in AlarmInput::all() {
        let x = alarm_input_x(input);
        let rear = centered_cube(
            format!("{PREFIX}_{}_rear_tube_bulkhead", input.slug()),
            54.0,
            20.0,
            28.0,
        )
        .translate(x, ALARM_Y / 2.0 - 22.0, ALARM_Z / 2.0 + 14.0);
        let front = centered_cube(
            format!("{PREFIX}_{}_front_to_diverter_bulkhead", input.slug()),
            54.0,
            20.0,
            28.0,
        )
        .translate(x, -ALARM_Y / 2.0 + 22.0, ALARM_Z / 2.0 + 14.0);
        let rear_opening = centered_cylinder(
            format!("{PREFIX}_{}_rear_bulkhead_opening", input.slug()),
            ALARM_TUBE_BORE_D / 2.0 + 0.8,
            24.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, ALARM_Y / 2.0 - 22.0, ALARM_Z / 2.0 + 14.0);
        let front_opening = centered_cylinder(
            format!("{PREFIX}_{}_front_bulkhead_opening", input.slug()),
            ALARM_TUBE_BORE_D / 2.0 + 0.8,
            24.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -ALARM_Y / 2.0 + 22.0, ALARM_Z / 2.0 + 14.0);
        bulkheads = bulkheads + (rear - rear_opening) + (front - front_opening);
    }
    bulkheads
}

fn alarm_status_windows() -> Part {
    let mut windows = Part::empty(format!("{PREFIX}_alarm_status_windows"));
    for input in AlarmInput::all() {
        let x = alarm_input_x(input);
        let base = centered_cube(
            format!("{PREFIX}_{}_status_window_land", input.slug()),
            52.0,
            14.0,
            4.0,
        )
        .translate(x, -42.0, ALARM_Z / 2.0 + 2.0);
        let raised = centered_cube(
            format!("{PREFIX}_{}_status_alarm_raised_bar", input.slug()),
            38.0,
            4.0,
            6.0,
        )
        .translate(x, -42.0, ALARM_Z / 2.0 + 5.0);
        windows = windows + base + raised;
    }
    windows
}

fn alarm_input_label_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_alarm_input_label_lands"));
    for input in AlarmInput::all() {
        lands = lands
            + csg_label_plaque(
                format!("{PREFIX}_{}_alarm_input_label", input.slug()),
                58.0,
                16.0,
                4.0,
                input.index(),
            )
            .translate(
                alarm_input_x(input),
                -ALARM_Y / 2.0 + 52.0,
                ALARM_Z / 2.0 + 3.0,
            );
    }
    lands
}

fn alarm_input_x(input: AlarmInput) -> f64 {
    centered_index(input.index(), ALARM_INPUT_COUNT, ALARM_CHANNEL_PITCH_X)
}

fn diverter_valve_bank() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_diverter_valve_bank_body"),
        VALVE_X,
        VALVE_Y,
        VALVE_Z,
    );
    let service_recess = centered_cube(
        format!("{PREFIX}_diverter_valve_bank_underbody_service_recess"),
        VALVE_X - 48.0,
        VALVE_Y - 46.0,
        18.0,
    )
    .translate(0.0, 0.0, -VALVE_Z / 2.0 + 9.0);

    body - service_recess - valve_actuator_pockets() - valve_tube_bores()
        + valve_actuator_collars()
        + valve_pair_separators()
        + valve_route_ribs()
        + valve_fail_safe_home_flags()
}

fn valve_actuator_pockets() -> Part {
    let mut pockets = Part::empty(format!("{PREFIX}_diverter_valve_actuator_pockets"));
    for lane in 0..DIVERTER_VALVE_LANES {
        let x = valve_lane_x(lane);
        pockets = pockets
            + centered_cylinder(
                format!("{PREFIX}_diverter_valve_actuator_pocket_{lane}"),
                VALVE_ACTUATOR_D / 2.0,
                28.0,
                36,
            )
            .translate(x, 10.0, VALVE_Z / 2.0 - 12.0)
            + centered_cube(
                format!("{PREFIX}_diverter_valve_solenoid_slot_{lane}"),
                30.0,
                48.0,
                18.0,
            )
            .translate(x, -48.0, VALVE_Z / 2.0 - 9.0);
    }
    pockets
}

fn valve_tube_bores() -> Part {
    let mut bores = Part::empty(format!("{PREFIX}_diverter_valve_tube_bores"));
    for lane in 0..DIVERTER_VALVE_LANES {
        let x = valve_lane_x(lane);
        bores = bores
            + centered_cylinder(
                format!("{PREFIX}_diverter_valve_lane_{lane}_through_tube_bore"),
                VALVE_TUBE_BORE_D / 2.0,
                VALVE_Y + 8.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, VALVE_Z / 2.0 - 18.0);
    }
    bores
}

fn valve_actuator_collars() -> Part {
    let mut collars = Part::empty(format!("{PREFIX}_diverter_valve_actuator_collars"));
    for lane in 0..DIVERTER_VALVE_LANES {
        let x = valve_lane_x(lane);
        let outer = centered_cylinder(
            format!("{PREFIX}_diverter_valve_actuator_outer_collar_{lane}"),
            VALVE_ACTUATOR_D / 2.0 + 7.0,
            7.0,
            36,
        )
        .translate(x, 10.0, VALVE_Z / 2.0 + 3.5);
        let inner = centered_cylinder(
            format!("{PREFIX}_diverter_valve_actuator_inner_clearance_{lane}"),
            VALVE_ACTUATOR_D / 2.0 - 2.0,
            8.0,
            36,
        )
        .translate(x, 10.0, VALVE_Z / 2.0 + 3.8);
        collars = collars + (outer - inner);
    }
    collars
}

fn valve_pair_separators() -> Part {
    let mut separators = Part::empty(format!("{PREFIX}_diverter_valve_pair_separators"));
    for pair in 0..DIVERTER_PAIRS {
        let x = centered_index(pair, DIVERTER_PAIRS, VALVE_PITCH_X * 2.0);
        separators = separators
            + centered_cube(
                format!("{PREFIX}_diverter_valve_alarm_pair_backstop_{pair}"),
                86.0,
                10.0,
                24.0,
            )
            .translate(x, VALVE_Y / 2.0 - 30.0, VALVE_Z / 2.0 + 12.0)
            + centered_cube(
                format!("{PREFIX}_diverter_valve_alarm_pair_front_stop_{pair}"),
                86.0,
                10.0,
                24.0,
            )
            .translate(x, -VALVE_Y / 2.0 + 30.0, VALVE_Z / 2.0 + 12.0);
    }
    separators
}

fn valve_route_ribs() -> Part {
    let mut ribs = Part::empty(format!("{PREFIX}_diverter_valve_route_ribs"));
    for pair in 0..DIVERTER_PAIRS {
        let x = centered_index(pair, DIVERTER_PAIRS, VALVE_PITCH_X * 2.0);
        ribs = ribs
            + flow_arrow(
                format!("{PREFIX}_diverter_valve_pair_{pair}_normal_to_bypass_arrow"),
                48.0,
                20.0,
                5.0,
            )
            .rotate(0.0, 0.0, 90.0)
            .translate(x + 22.0, -12.0, VALVE_Z / 2.0 + 5.0)
            + flow_arrow(
                format!("{PREFIX}_diverter_valve_pair_{pair}_alarm_to_quarantine_arrow"),
                48.0,
                20.0,
                5.0,
            )
            .rotate(0.0, 0.0, -90.0)
            .translate(x - 22.0, -12.0, VALVE_Z / 2.0 + 5.0);
    }
    ribs
}

fn valve_fail_safe_home_flags() -> Part {
    let mut flags = Part::empty(format!("{PREFIX}_diverter_valve_fail_safe_home_flags"));
    for lane in 0..DIVERTER_VALVE_LANES {
        let x = valve_lane_x(lane);
        let flag_z = if lane % 2 == 0 { 11.0 } else { 18.0 };
        flags = flags
            + centered_cube(
                format!("{PREFIX}_diverter_valve_home_position_flag_{lane}"),
                22.0,
                8.0,
                flag_z,
            )
            .translate(x, 58.0, VALVE_Z / 2.0 + flag_z / 2.0);
    }
    flags
}

fn valve_lane_x(lane: usize) -> f64 {
    centered_index(lane, DIVERTER_VALVE_LANES, VALVE_PITCH_X)
}

fn quarantine_bag_hotel() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_quarantine_bag_hotel_body"),
        BAG_X,
        BAG_Y,
        BAG_Z,
    );
    let cleanout = centered_cube(
        format!("{PREFIX}_quarantine_bag_hotel_underbody_cleanout"),
        BAG_X - 46.0,
        BAG_Y - 40.0,
        16.0,
    )
    .translate(0.0, 0.0, -BAG_Z / 2.0 + 8.0);

    body - cleanout - quarantine_bag_recesses() - quarantine_bag_tube_bores()
        + quarantine_bag_clamps()
        + quarantine_bag_hang_pegs()
        + quarantine_seal_witness_lands()
        + quarantine_bag_label_tabs()
}

fn quarantine_bag_recesses() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_quarantine_bag_recesses"));
    for index in 0..QUARANTINE_BAG_NESTS {
        let (x, y) = bag_center(index);
        cuts = cuts
            + centered_cube(
                format!("{PREFIX}_quarantine_bag_nest_recess_{index}"),
                BAG_NEST_X,
                BAG_NEST_Y,
                18.0,
            )
            .translate(x, y, BAG_Z / 2.0 - 8.5)
            + centered_cylinder(
                format!("{PREFIX}_quarantine_bag_fill_port_shadow_{index}"),
                16.0,
                20.0,
                32,
            )
            .translate(x - BAG_NEST_X / 2.0 + 28.0, y, BAG_Z / 2.0 - 8.0);
    }
    cuts
}

fn quarantine_bag_tube_bores() -> Part {
    let mut bores = Part::empty(format!("{PREFIX}_quarantine_bag_tube_bores"));
    for index in 0..QUARANTINE_BAG_NESTS {
        let (x, y) = bag_center(index);
        bores = bores
            + centered_cylinder(
                format!("{PREFIX}_quarantine_bag_tube_bore_{index}"),
                BAG_TUBE_PORT_D / 2.0,
                46.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(x + BAG_NEST_X / 2.0 - 18.0, y, BAG_Z / 2.0 - 12.0);
    }
    bores
}

fn quarantine_bag_clamps() -> Part {
    let mut clamps = Part::empty(format!("{PREFIX}_quarantine_bag_clamps"));
    for index in 0..QUARANTINE_BAG_NESTS {
        let (x, y) = bag_center(index);
        clamps = clamps
            + centered_cube(
                format!("{PREFIX}_quarantine_bag_left_clamp_{index}"),
                9.0,
                BAG_NEST_Y + 18.0,
                20.0,
            )
            .translate(x - BAG_NEST_X / 2.0 - 10.0, y, BAG_Z / 2.0 + 10.0)
            + centered_cube(
                format!("{PREFIX}_quarantine_bag_right_clamp_{index}"),
                9.0,
                BAG_NEST_Y + 18.0,
                20.0,
            )
            .translate(x + BAG_NEST_X / 2.0 + 10.0, y, BAG_Z / 2.0 + 10.0);
    }
    clamps
}

fn quarantine_bag_hang_pegs() -> Part {
    let mut pegs = Part::empty(format!("{PREFIX}_quarantine_bag_hang_pegs"));
    for index in 0..QUARANTINE_BAG_NESTS {
        let (x, y) = bag_center(index);
        pegs = pegs
            + centered_cylinder(
                format!("{PREFIX}_quarantine_bag_upper_hang_peg_{index}"),
                5.0,
                24.0,
                24,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(x, y + BAG_NEST_Y / 2.0 + 18.0, BAG_Z / 2.0 + 15.0);
    }
    pegs
}

fn quarantine_seal_witness_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_quarantine_seal_witness_lands"));
    for index in 0..QUARANTINE_BAG_NESTS {
        let (x, y) = bag_center(index);
        lands = lands
            + centered_cube(
                format!("{PREFIX}_quarantine_bag_seal_witness_window_{index}"),
                92.0,
                10.0,
                5.0,
            )
            .translate(x, y - BAG_NEST_Y / 2.0 - 16.0, BAG_Z / 2.0 + 2.5);
    }
    lands
}

fn quarantine_bag_label_tabs() -> Part {
    let mut tabs = Part::empty(format!("{PREFIX}_quarantine_bag_label_tabs"));
    for index in 0..QUARANTINE_BAG_NESTS {
        let (x, y) = bag_center(index);
        tabs = tabs
            + csg_label_plaque(
                format!("{PREFIX}_quarantine_bag_custody_label_{index}"),
                68.0,
                18.0,
                4.0,
                index,
            )
            .translate(x + 30.0, y + BAG_NEST_Y / 2.0 - 8.0, BAG_Z / 2.0 + 3.0);
    }
    tabs
}

fn bag_center(index: usize) -> (f64, f64) {
    let col = index % 2;
    let row = index / 2;
    (
        centered_index(col, 2, BAG_PITCH_X),
        centered_index(row, 2, BAG_PITCH_Y),
    )
}

fn bypass_safe_path_bridge() -> Part {
    let plate = centered_cube(
        format!("{PREFIX}_bypass_safe_path_bridge_body"),
        BYPASS_X,
        BYPASS_Y,
        BYPASS_Z,
    );
    let underside_relief = centered_cube(
        format!("{PREFIX}_bypass_safe_path_bridge_lightening_recess"),
        BYPASS_X - 42.0,
        BYPASS_Y - 42.0,
        14.0,
    )
    .translate(0.0, 0.0, -BYPASS_Z / 2.0 + 7.0);

    plate - underside_relief - bypass_channel_cuts()
        + bypass_channel_rims()
        + bypass_relief_wells()
        + bypass_check_valve_windows()
        + bypass_flow_arrows()
}

fn bypass_channel_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_bypass_channel_cuts"));
    for lane in 0..BYPASS_LANES {
        let x = centered_index(lane, BYPASS_LANES, BYPASS_LANE_PITCH_X);
        let main = centered_cube(
            format!("{PREFIX}_bypass_lane_{lane}_safe_channel"),
            BYPASS_CHANNEL_W,
            BYPASS_Y - 62.0,
            14.0,
        )
        .translate(x, 0.0, BYPASS_Z / 2.0 - 7.0);
        let cross = centered_cube(
            format!("{PREFIX}_bypass_lane_{lane}_quarantine_cross_channel"),
            48.0,
            BYPASS_CHANNEL_W,
            14.0,
        )
        .translate(x - 18.0, -BYPASS_Y / 2.0 + 48.0, BYPASS_Z / 2.0 - 7.0);
        cuts = cuts + main + cross;
    }
    cuts
}

fn bypass_channel_rims() -> Part {
    let mut rims = Part::empty(format!("{PREFIX}_bypass_channel_rims"));
    for lane in 0..BYPASS_LANES {
        let x = centered_index(lane, BYPASS_LANES, BYPASS_LANE_PITCH_X);
        rims = rims
            + centered_cube(
                format!("{PREFIX}_bypass_lane_{lane}_left_raised_rim"),
                4.0,
                BYPASS_Y - 58.0,
                8.0,
            )
            .translate(x - 9.0, 0.0, BYPASS_Z / 2.0 + 4.0)
            + centered_cube(
                format!("{PREFIX}_bypass_lane_{lane}_right_raised_rim"),
                4.0,
                BYPASS_Y - 58.0,
                8.0,
            )
            .translate(x + 9.0, 0.0, BYPASS_Z / 2.0 + 4.0);
    }
    rims
}

fn bypass_relief_wells() -> Part {
    let mut wells = Part::empty(format!("{PREFIX}_bypass_relief_wells"));
    for i in 0..BYPASS_SAFE_PRESSURE_RELIEF_COUNT {
        let x = centered_index(i, BYPASS_SAFE_PRESSURE_RELIEF_COUNT, 86.0);
        let well = centered_cylinder(
            format!("{PREFIX}_bypass_safe_pressure_relief_well_{i}"),
            24.0,
            16.0,
            36,
        )
        .translate(x, BYPASS_Y / 2.0 - 52.0, BYPASS_Z / 2.0 + 8.0);
        let opening = centered_cylinder(
            format!("{PREFIX}_bypass_safe_pressure_relief_opening_{i}"),
            12.0,
            18.0,
            36,
        )
        .translate(x, BYPASS_Y / 2.0 - 52.0, BYPASS_Z / 2.0 + 8.2);
        wells = wells + (well - opening);
    }
    wells
}

fn bypass_check_valve_windows() -> Part {
    let mut windows = Part::empty(format!("{PREFIX}_bypass_check_valve_windows"));
    for lane in 0..BYPASS_CHECK_VALVE_WINDOWS {
        let x = centered_index(lane, BYPASS_CHECK_VALVE_WINDOWS, BYPASS_LANE_PITCH_X);
        windows = windows
            + centered_cube(
                format!("{PREFIX}_bypass_lane_{lane}_check_valve_witness_window"),
                42.0,
                14.0,
                5.0,
            )
            .translate(x, -BYPASS_Y / 2.0 + 80.0, BYPASS_Z / 2.0 + 2.5);
    }
    windows
}

fn bypass_flow_arrows() -> Part {
    let mut arrows = Part::empty(format!("{PREFIX}_bypass_flow_arrows"));
    for lane in 0..BYPASS_LANES {
        let x = centered_index(lane, BYPASS_LANES, BYPASS_LANE_PITCH_X);
        arrows = arrows
            + flow_arrow(
                format!("{PREFIX}_bypass_lane_{lane}_flow_direction_arrow"),
                44.0,
                20.0,
                5.0,
            )
            .rotate(0.0, 0.0, -90.0)
            .translate(x, 0.0, BYPASS_Z / 2.0 + 5.0);
    }
    arrows
}

fn alarm_token_rail() -> Part {
    let rail = centered_cube(
        format!("{PREFIX}_alarm_token_rail_body"),
        TOKEN_X,
        TOKEN_Y,
        TOKEN_Z,
    );
    let pocket_cut = alarm_token_dock_cuts();
    rail - pocket_cut + alarm_token_dock_rims() + alarm_state_header_tabs() + alarm_tokens()
}

fn alarm_token_dock_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_alarm_token_dock_cuts"));
    for input in AlarmInput::all() {
        for state in 0..TOKEN_STATES_PER_ALARM {
            let (x, y) = token_dock_xy(input, state);
            cuts = cuts
                + centered_cube(
                    format!(
                        "{PREFIX}_{}_alarm_state_{state}_token_dock_cut",
                        input.slug()
                    ),
                    TOKEN_DOCK_X,
                    TOKEN_DOCK_Y,
                    12.0,
                )
                .translate(x, y, TOKEN_Z / 2.0 - 6.0);
        }
    }
    cuts
}

fn alarm_token_dock_rims() -> Part {
    let mut rims = Part::empty(format!("{PREFIX}_alarm_token_dock_rims"));
    for input in AlarmInput::all() {
        for state in 0..TOKEN_STATES_PER_ALARM {
            let (x, y) = token_dock_xy(input, state);
            let outer = centered_cube(
                format!(
                    "{PREFIX}_{}_alarm_state_{state}_token_dock_outer_rim",
                    input.slug()
                ),
                TOKEN_DOCK_X + 8.0,
                TOKEN_DOCK_Y + 8.0,
                4.0,
            )
            .translate(x, y, TOKEN_Z / 2.0 + 2.0);
            let inner = centered_cube(
                format!(
                    "{PREFIX}_{}_alarm_state_{state}_token_dock_inner_opening",
                    input.slug()
                ),
                TOKEN_DOCK_X - 4.0,
                TOKEN_DOCK_Y - 4.0,
                5.0,
            )
            .translate(x, y, TOKEN_Z / 2.0 + 2.2);
            rims = rims + (outer - inner);
        }
    }
    rims
}

fn alarm_state_header_tabs() -> Part {
    let mut tabs = Part::empty(format!("{PREFIX}_alarm_state_header_tabs"));
    for state in 0..TOKEN_STATES_PER_ALARM {
        tabs = tabs
            + csg_label_plaque(
                format!("{PREFIX}_alarm_state_header_tab_{state}"),
                78.0,
                14.0,
                4.0,
                50 + state,
            )
            .translate(
                -TOKEN_X / 2.0 + 52.0,
                token_state_y(state),
                TOKEN_Z / 2.0 + 3.0,
            );
    }
    tabs
}

fn alarm_tokens() -> Part {
    let mut tokens = Part::empty(format!("{PREFIX}_alarm_state_tokens"));
    for input in AlarmInput::all() {
        let x = alarm_token_x(input);
        tokens = tokens
            + centered_cylinder(
                format!("{PREFIX}_{}_generated_alarm_token", input.slug()),
                8.0,
                input.token_height(),
                28,
            )
            .translate(
                x,
                token_state_y(0),
                TOKEN_Z / 2.0 + input.token_height() / 2.0,
            )
            + centered_cube(
                format!("{PREFIX}_{}_acknowledged_alarm_token", input.slug()),
                15.0,
                15.0,
                input.token_height() + 2.0,
            )
            .translate(
                x,
                token_state_y(1),
                TOKEN_Z / 2.0 + (input.token_height() + 2.0) / 2.0,
            )
            + centered_cylinder(
                format!("{PREFIX}_{}_cleared_alarm_token", input.slug()),
                7.0,
                input.token_height() + 4.0,
                4,
            )
            .rotate(0.0, 0.0, 45.0)
            .translate(
                x,
                token_state_y(2),
                TOKEN_Z / 2.0 + (input.token_height() + 4.0) / 2.0,
            );
    }
    tokens
}

fn token_dock_xy(input: AlarmInput, state: usize) -> (f64, f64) {
    (alarm_token_x(input), token_state_y(state))
}

fn alarm_token_x(input: AlarmInput) -> f64 {
    -TOKEN_X / 2.0 + 126.0 + input.index() as f64 * TOKEN_PITCH_X
}

fn token_state_y(state: usize) -> f64 {
    centered_index(state, TOKEN_STATES_PER_ALARM, TOKEN_STATE_PITCH_Y)
}

fn manual_free_reset_interlock() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_manual_free_reset_interlock_body"),
        RESET_X,
        RESET_Y,
        RESET_Z,
    );
    let service_recess = centered_cube(
        format!("{PREFIX}_manual_free_reset_interlock_underbody_recess"),
        RESET_X - 44.0,
        RESET_Y - 42.0,
        16.0,
    )
    .translate(0.0, 0.0, -RESET_Z / 2.0 + 8.0);

    body - service_recess - reset_pin_slots() - reset_servo_bay()
        + reset_interlock_pin_flags()
        + reset_sequence_cam_rail()
        + no_manual_bypass_guard()
        + reset_ready_window()
}

fn reset_pin_slots() -> Part {
    let mut slots = Part::empty(format!("{PREFIX}_reset_interlock_pin_slots"));
    for pin in 0..RESET_INTERLOCK_INPUTS {
        let x = reset_pin_x(pin);
        slots = slots
            + centered_cylinder(
                format!("{PREFIX}_reset_interlock_pin_slot_{pin}"),
                RESET_PIN_D / 2.0,
                28.0,
                24,
            )
            .translate(x, 18.0, RESET_Z / 2.0 - 10.0);
    }
    slots
}

fn reset_servo_bay() -> Part {
    centered_cube(
        format!("{PREFIX}_reset_keyless_servo_bay"),
        96.0,
        54.0,
        24.0,
    )
    .translate(0.0, -62.0, RESET_Z / 2.0 - 12.0)
}

fn reset_interlock_pin_flags() -> Part {
    let mut flags = Part::empty(format!("{PREFIX}_reset_interlock_pin_flags"));
    for pin in 0..RESET_INTERLOCK_INPUTS {
        let x = reset_pin_x(pin);
        flags = flags
            + centered_cube(
                format!("{PREFIX}_reset_interlock_pin_flag_{pin}"),
                18.0,
                8.0,
                12.0 + pin as f64,
            )
            .translate(x, 48.0, RESET_Z / 2.0 + (12.0 + pin as f64) / 2.0);
    }
    flags
}

fn reset_sequence_cam_rail() -> Part {
    let rail = centered_cube(
        format!("{PREFIX}_reset_sequence_cam_rail"),
        RESET_X - 62.0,
        14.0,
        16.0,
    )
    .translate(0.0, -6.0, RESET_Z / 2.0 + 8.0);
    let mut notches = Part::empty(format!("{PREFIX}_reset_sequence_cam_notches"));
    for pin in 0..RESET_INTERLOCK_INPUTS {
        notches = notches
            + centered_cube(
                format!("{PREFIX}_reset_sequence_cam_notch_{pin}"),
                12.0,
                16.0,
                18.0,
            )
            .translate(reset_pin_x(pin), -6.0, RESET_Z / 2.0 + 8.5);
    }
    rail - notches
}

fn no_manual_bypass_guard() -> Part {
    let left = centered_cube(
        format!("{PREFIX}_reset_no_manual_bypass_left_guard"),
        16.0,
        RESET_Y - 56.0,
        62.0,
    )
    .translate(-RESET_X / 2.0 + 34.0, 0.0, RESET_Z / 2.0 + 31.0);
    let right = centered_cube(
        format!("{PREFIX}_reset_no_manual_bypass_right_guard"),
        16.0,
        RESET_Y - 56.0,
        62.0,
    )
    .translate(RESET_X / 2.0 - 34.0, 0.0, RESET_Z / 2.0 + 31.0);
    let top = centered_cube(
        format!("{PREFIX}_reset_no_manual_bypass_top_guard"),
        RESET_X - 96.0,
        18.0,
        36.0,
    )
    .translate(0.0, RESET_Y / 2.0 - 36.0, RESET_Z / 2.0 + 18.0);
    let front = centered_cube(
        format!("{PREFIX}_reset_no_manual_bypass_front_seal_bar"),
        RESET_X - 96.0,
        18.0,
        28.0,
    )
    .translate(0.0, -RESET_Y / 2.0 + 36.0, RESET_Z / 2.0 + 14.0);

    left + right + top + front
}

fn reset_ready_window() -> Part {
    let mut windows = Part::empty(format!("{PREFIX}_reset_ready_windows"));
    for i in 0..NO_MANUAL_BYPASS_GUARDS {
        windows = windows
            + centered_cube(
                format!("{PREFIX}_reset_ready_sequence_window_{i}"),
                64.0,
                12.0,
                5.0,
            )
            .translate(
                centered_index(i, NO_MANUAL_BYPASS_GUARDS, 82.0),
                -92.0,
                RESET_Z / 2.0 + 3.0,
            );
    }
    windows
}

fn reset_pin_x(pin: usize) -> f64 {
    centered_index(pin, RESET_INTERLOCK_INPUTS, RESET_PIN_PITCH_X)
}

fn event_logger_docks() -> Part {
    let panel = centered_cube(
        format!("{PREFIX}_event_logger_dock_panel"),
        LOGGER_X,
        LOGGER_Y,
        LOGGER_Z,
    );
    let recess = centered_cube(
        format!("{PREFIX}_event_logger_panel_lightening_recess"),
        LOGGER_X - 46.0,
        LOGGER_Y - 42.0,
        14.0,
    )
    .translate(0.0, 0.0, -LOGGER_Z / 2.0 + 7.0);

    panel - recess - logger_pocket_cuts()
        + logger_dock_retainer_tabs()
        + logger_cable_lanes()
        + time_sync_docks()
        + logger_audit_state_labels()
}

fn logger_pocket_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_event_logger_pocket_cuts"));
    for dock in 0..EVENT_LOGGER_DOCKS {
        let x = logger_x(dock);
        cuts = cuts
            + centered_cube(
                format!("{PREFIX}_event_logger_pocket_cut_{dock}"),
                LOGGER_POCKET_X,
                LOGGER_POCKET_Y,
                18.0,
            )
            .translate(x, 12.0, LOGGER_Z / 2.0 - 8.5)
            + centered_cube(
                format!("{PREFIX}_event_logger_cable_exit_cut_{dock}"),
                14.0,
                72.0,
                14.0,
            )
            .translate(x, -60.0, LOGGER_Z / 2.0 - 7.0);
    }
    cuts
}

fn logger_dock_retainer_tabs() -> Part {
    let mut tabs = Part::empty(format!("{PREFIX}_event_logger_retainer_tabs"));
    for dock in 0..EVENT_LOGGER_DOCKS {
        let x = logger_x(dock);
        tabs = tabs
            + centered_cube(
                format!("{PREFIX}_event_logger_left_retainer_tab_{dock}"),
                10.0,
                LOGGER_POCKET_Y + 18.0,
                12.0,
            )
            .translate(x - LOGGER_POCKET_X / 2.0 - 8.0, 12.0, LOGGER_Z / 2.0 + 6.0)
            + centered_cube(
                format!("{PREFIX}_event_logger_right_retainer_tab_{dock}"),
                10.0,
                LOGGER_POCKET_Y + 18.0,
                12.0,
            )
            .translate(x + LOGGER_POCKET_X / 2.0 + 8.0, 12.0, LOGGER_Z / 2.0 + 6.0);
    }
    tabs
}

fn logger_cable_lanes() -> Part {
    let mut lanes = Part::empty(format!("{PREFIX}_event_logger_cable_lanes"));
    for dock in 0..EVENT_LOGGER_DOCKS {
        lanes = lanes
            + centered_cube(
                format!("{PREFIX}_event_logger_cable_strain_relief_lane_{dock}"),
                10.0,
                86.0,
                6.0,
            )
            .translate(logger_x(dock), -68.0, LOGGER_Z / 2.0 + 3.0);
    }
    lanes
}

fn time_sync_docks() -> Part {
    let mut docks = Part::empty(format!("{PREFIX}_event_logger_time_sync_docks"));
    for i in 0..TIME_SYNC_DOCKS {
        let x = centered_index(i, TIME_SYNC_DOCKS, 86.0);
        let outer = centered_cylinder(
            format!("{PREFIX}_event_logger_time_sync_dock_outer_{i}"),
            18.0,
            6.0,
            36,
        )
        .translate(x, LOGGER_Y / 2.0 - 32.0, LOGGER_Z / 2.0 + 3.0);
        let inner = centered_cylinder(
            format!("{PREFIX}_event_logger_time_sync_dock_inner_{i}"),
            8.0,
            7.0,
            30,
        )
        .translate(x, LOGGER_Y / 2.0 - 32.0, LOGGER_Z / 2.0 + 3.2);
        docks = docks + (outer - inner);
    }
    docks
}

fn logger_audit_state_labels() -> Part {
    let mut labels = Part::empty(format!("{PREFIX}_event_logger_audit_state_labels"));
    for dock in 0..EVENT_LOGGER_DOCKS {
        labels = labels
            + csg_label_plaque(
                format!("{PREFIX}_event_logger_audit_label_{dock}"),
                62.0,
                14.0,
                4.0,
                70 + dock,
            )
            .translate(logger_x(dock), -LOGGER_Y / 2.0 + 24.0, LOGGER_Z / 2.0 + 3.0);
    }
    labels
}

fn logger_x(dock: usize) -> f64 {
    centered_index(dock, EVENT_LOGGER_DOCKS, LOGGER_PITCH_X)
}

fn custody_label_lands() -> Part {
    let plate = centered_cube(
        format!("{PREFIX}_custody_label_land_plate"),
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    );
    plate + barcode_lands() + custody_card_slots() + certificate_lands()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_custody_barcode_lands"));
    for i in 0..CUSTODY_LABEL_LANDS {
        let col = i % 6;
        let row = i / 6;
        let x = centered_index(col, 6, 58.0);
        let y = 22.0 - row as f64 * 40.0;
        lands = lands
            + csg_label_plaque(
                format!("{PREFIX}_custody_barcode_land_{i}"),
                48.0,
                18.0,
                4.0,
                90 + i,
            )
            .translate(x, y, CUSTODY_Z / 2.0 + 3.0);
    }
    lands
}

fn custody_card_slots() -> Part {
    let mut slots = Part::empty(format!("{PREFIX}_custody_card_slots"));
    for i in 0..CUSTODY_CARD_SLOTS {
        let x = centered_index(i, CUSTODY_CARD_SLOTS, 82.0);
        let slot = centered_cube(
            format!("{PREFIX}_custody_card_slot_outer_{i}"),
            68.0,
            24.0,
            6.0,
        )
        .translate(x, -CUSTODY_Y / 2.0 + 22.0, CUSTODY_Z / 2.0 + 3.0);
        let relief = centered_cube(
            format!("{PREFIX}_custody_card_slot_recess_{i}"),
            56.0,
            14.0,
            7.0,
        )
        .translate(x, -CUSTODY_Y / 2.0 + 22.0, CUSTODY_Z / 2.0 + 3.2);
        slots = slots + (slot - relief);
    }
    slots
}

fn certificate_lands() -> Part {
    let mut certs = Part::empty(format!("{PREFIX}_custody_certificate_lands"));
    for i in 0..CERTIFICATE_LANDS {
        certs = certs
            + certificate_plaque(
                format!("{PREFIX}_custody_certificate_plaque_{i}"),
                82.0,
                34.0,
                4.0,
                130 + i,
            )
            .translate(
                centered_index(i, CERTIFICATE_LANDS, 112.0),
                CUSTODY_Y / 2.0 - 24.0,
                CUSTODY_Z / 2.0 + 3.0,
            );
    }
    certs
}

fn release_hold_reject_gates() -> Part {
    let base = centered_cube(
        format!("{PREFIX}_release_hold_reject_gate_base"),
        GATES_X,
        GATES_Y,
        GATES_Z,
    );
    base - disposition_slot_cuts() + disposition_gate_walls() + disposition_token_backstops()
}

fn disposition_slot_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_disposition_slot_cuts"));
    for gate in DispositionGate::all() {
        let x = gate_x(gate);
        for slot in 0..GATE_CAPACITY_PER_LANE {
            cuts = cuts
                + centered_cube(
                    format!("{PREFIX}_{}_gate_token_slot_cut_{slot}", gate.slug()),
                    GATE_SLOT_X,
                    GATE_SLOT_Y,
                    GATES_Z + 2.0,
                )
                .translate(
                    x,
                    centered_index(slot, GATE_CAPACITY_PER_LANE, 28.0),
                    0.0,
                );
        }
    }
    cuts
}

fn disposition_gate_walls() -> Part {
    let mut walls = Part::empty(format!("{PREFIX}_disposition_gate_walls"));
    for gate in DispositionGate::all() {
        let x = gate_x(gate);
        let height = gate.gate_height();
        walls = walls
            + centered_cube(
                format!("{PREFIX}_{}_gate_rear_wall", gate.slug()),
                GATE_SLOT_X + 24.0,
                8.0,
                height,
            )
            .translate(x, GATES_Y / 2.0 - 18.0, GATES_Z / 2.0 + height / 2.0)
            + centered_cube(
                format!("{PREFIX}_{}_gate_left_side_wall", gate.slug()),
                8.0,
                GATES_Y - 34.0,
                height,
            )
            .translate(
                x - GATE_SLOT_X / 2.0 - 14.0,
                0.0,
                GATES_Z / 2.0 + height / 2.0,
            )
            + centered_cube(
                format!("{PREFIX}_{}_gate_right_side_wall", gate.slug()),
                8.0,
                GATES_Y - 34.0,
                height,
            )
            .translate(
                x + GATE_SLOT_X / 2.0 + 14.0,
                0.0,
                GATES_Z / 2.0 + height / 2.0,
            );
    }
    walls
}

fn disposition_token_backstops() -> Part {
    let mut stops = Part::empty(format!("{PREFIX}_disposition_token_backstops"));
    for gate in DispositionGate::all() {
        stops = stops
            + csg_label_plaque(
                format!("{PREFIX}_{}_gate_label_plaque", gate.slug()),
                76.0,
                18.0,
                4.0,
                160 + gate.index(),
            )
            .translate(gate_x(gate), -GATES_Y / 2.0 + 18.0, GATES_Z / 2.0 + 3.0);
    }
    stops
}

fn gate_x(gate: DispositionGate) -> f64 {
    centered_index(gate.index(), DISPOSITION_GATE_COUNT, GATE_PITCH_X)
}

fn total_gate_capacity() -> usize {
    DISPOSITION_GATE_COUNT * GATE_CAPACITY_PER_LANE
}

fn robot_service_keepouts() -> Part {
    let front = keepout_rect(
        format!("{PREFIX}_front_robot_sweep_keepout"),
        (0.0, -STATION_Y / 2.0 + FRONT_ROBOT_KEEP_OUT_Y / 2.0 + 24.0),
        STATION_X - 170.0,
        FRONT_ROBOT_KEEP_OUT_Y,
        BASE_Z / 2.0 + 8.0,
    );
    let rear = keepout_rect(
        format!("{PREFIX}_rear_service_keepout"),
        (0.0, STATION_Y / 2.0 - REAR_SERVICE_KEEP_OUT_Y / 2.0 - 24.0),
        STATION_X - 170.0,
        REAR_SERVICE_KEEP_OUT_Y,
        BASE_Z / 2.0 + 8.0,
    );
    let left = keepout_rect(
        format!("{PREFIX}_left_quarantine_bag_service_keepout"),
        (
            -STATION_X / 2.0 + LEFT_BAG_SERVICE_KEEP_OUT_X / 2.0 + 30.0,
            -40.0,
        ),
        LEFT_BAG_SERVICE_KEEP_OUT_X,
        540.0,
        BASE_Z / 2.0 + 8.0,
    );
    let right = keepout_rect(
        format!("{PREFIX}_right_valve_service_keepout"),
        (
            STATION_X / 2.0 - RIGHT_VALVE_SERVICE_KEEP_OUT_X / 2.0 - 30.0,
            90.0,
        ),
        RIGHT_VALVE_SERVICE_KEEP_OUT_X,
        540.0,
        BASE_Z / 2.0 + 8.0,
    );
    let lift = keepout_rect(
        format!("{PREFIX}_top_valve_lift_keepout"),
        VALVE_POS,
        VALVE_X + 90.0,
        VALVE_Y + 90.0,
        TOP_VALVE_LIFT_KEEP_OUT_Z,
    ) + valve_lift_keepout_posts();

    front + rear + left + right + lift
}

fn keepout_rect(name: impl Into<String>, center: (f64, f64), x: f64, y: f64, z: f64) -> Part {
    let name = name.into();
    let rail = 8.0;
    let left = centered_cube(format!("{name}_left_rail"), rail, y, rail).translate(
        center.0 - x / 2.0,
        center.1,
        z,
    );
    let right = centered_cube(format!("{name}_right_rail"), rail, y, rail).translate(
        center.0 + x / 2.0,
        center.1,
        z,
    );
    let front = centered_cube(format!("{name}_front_rail"), x, rail, rail).translate(
        center.0,
        center.1 - y / 2.0,
        z,
    );
    let rear = centered_cube(format!("{name}_rear_rail"), x, rail, rail).translate(
        center.0,
        center.1 + y / 2.0,
        z,
    );
    left + right + front + rear
}

fn valve_lift_keepout_posts() -> Part {
    let mut posts = Part::empty(format!("{PREFIX}_valve_lift_keepout_posts"));
    let x = VALVE_X + 90.0;
    let y = VALVE_Y + 90.0;
    for (i, (px, py)) in [
        (VALVE_POS.0 - x / 2.0, VALVE_POS.1 - y / 2.0),
        (VALVE_POS.0 + x / 2.0, VALVE_POS.1 - y / 2.0),
        (VALVE_POS.0 - x / 2.0, VALVE_POS.1 + y / 2.0),
        (VALVE_POS.0 + x / 2.0, VALVE_POS.1 + y / 2.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(
                format!("{PREFIX}_valve_lift_keepout_post_{i}"),
                8.0,
                8.0,
                TOP_VALVE_LIFT_KEEP_OUT_Z,
            )
            .translate(*px, *py, TOP_VALVE_LIFT_KEEP_OUT_Z / 2.0);
    }
    posts
}

fn flow_arrow(name: impl Into<String>, length: f64, width: f64, height: f64) -> Part {
    let name = name.into();
    let shaft = centered_cube(format!("{name}_shaft"), length * 0.58, width * 0.28, height)
        .translate(-length * 0.13, 0.0, 0.0);
    let head = centered_cube(format!("{name}_head"), width * 0.58, width * 0.58, height)
        .rotate(0.0, 0.0, 45.0)
        .translate(length * 0.28, 0.0, 0.0);
    shaft + head
}

fn csg_label_plaque(name: impl Into<String>, x: f64, y: f64, z: f64, seed: usize) -> Part {
    let name = name.into();
    let base = centered_cube(format!("{name}_base"), x, y, z);
    let mut bars = Part::empty(format!("{name}_raised_bars"));
    for i in 0..LABEL_BAR_COUNT {
        let width = 2.0 + ((seed + i) % 4) as f64 * 1.4;
        let height = (y - 7.0 - (i % 3) as f64 * 2.0).max(3.0);
        let x_offset = -x / 2.0 + 8.0 + i as f64 * ((x - 18.0) / LABEL_BAR_COUNT as f64);
        bars =
            bars + centered_cube(format!("{name}_raised_bar_{i}"), width, height, z + 1.4)
                .translate(x_offset, 0.0, z / 2.0 + 0.7);
    }
    let orientation_tab = centered_cube(format!("{name}_orientation_tab"), 10.0, 4.0, z + 1.6)
        .translate(x / 2.0 - 10.0, y / 2.0 - 5.0, z / 2.0 + 0.8);
    base + bars + orientation_tab
}

fn certificate_plaque(name: impl Into<String>, x: f64, y: f64, z: f64, seed: usize) -> Part {
    let name = name.into();
    let sheet = centered_cube(format!("{name}_sheet"), x, y, z);
    let barcode = csg_label_plaque(format!("{name}_barcode"), x - 12.0, 9.0, z + 0.8, seed)
        .translate(0.0, y / 2.0 - 9.0, z / 2.0 + 0.4);
    let signature_line = centered_cube(format!("{name}_signature_line"), x - 16.0, 2.2, z + 1.0)
        .translate(0.0, -y / 2.0 + 8.0, z / 2.0 + 0.5);
    let seal = centered_cylinder(
        format!("{name}_raised_circular_certificate_seal"),
        6.0,
        z + 1.2,
        28,
    )
    .translate(x / 2.0 - 12.0, -y / 2.0 + 10.0, z / 2.0 + 0.6);

    sheet + barcode + signature_line + seal
}

fn fiducial_disc(name: impl Into<String>) -> Part {
    let name = name.into();
    centered_cylinder(format!("{name}_outer_ring"), 14.0, 4.0, 36)
        - centered_cylinder(format!("{name}_inner_dot"), 5.0, 5.0, 24)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        for path in OUTPUTS {
            assert!(
                path.starts_with(
                    "output/closed_perfusion_alarm_response_quarantine_diverter_station_"
                ),
                "{path}"
            );
            assert!(path.ends_with(".stl"));
        }
    }

    #[test]
    fn requested_feature_groups_are_explicit() {
        assert_eq!(REQUIRED_FEATURES.len(), 11);
        for expected in [
            "pressure_flow_bubble_ph_alarm_inputs",
            "diverter_valve_bank",
            "quarantine_waste_retain_bags",
            "bypass_safe_path",
            "alarm_token_rail",
            "manual_free_reset_interlock",
            "leak_tray",
            "custody_label_lands",
            "event_logger_docks",
            "release_hold_reject_gates",
            "robot_service_keepouts",
        ] {
            assert!(REQUIRED_FEATURES.contains(&expected));
        }
    }

    #[test]
    fn layout_zones_are_bounded_and_non_overlapping() {
        assert_design_constraints();
        assert_eq!(layout_rects().len(), 9);
    }

    #[test]
    fn alarm_inputs_cover_required_failure_modes() {
        let inputs = AlarmInput::all();
        assert_eq!(inputs.len(), ALARM_INPUT_COUNT);
        assert!(inputs.contains(&AlarmInput::Pressure));
        assert!(inputs.contains(&AlarmInput::Flow));
        assert!(inputs.contains(&AlarmInput::Bubble));
        assert!(inputs.contains(&AlarmInput::Ph));
        assert_eq!(ALARM_SENSOR_POCKETS, ALARM_INPUT_COUNT);
        assert_eq!(ALARM_STATUS_WINDOWS, ALARM_INPUT_COUNT);
        assert_eq!(EVENT_LOGGER_DOCKS, ALARM_INPUT_COUNT);
    }

    #[test]
    fn diversion_and_bypass_counts_match_alarm_channels() {
        assert_eq!(DIVERTER_VALVE_LANES, ALARM_INPUT_COUNT * 2);
        assert_eq!(DIVERTER_PAIRS, ALARM_INPUT_COUNT);
        assert_eq!(BYPASS_LANES, ALARM_INPUT_COUNT);
        assert_eq!(BYPASS_CHECK_VALVE_WINDOWS, BYPASS_LANES);
        assert!(BYPASS_SAFE_PRESSURE_RELIEF_COUNT >= 2);
        assert!(VALVE_ACTUATOR_D > VALVE_TUBE_BORE_D * 3.0);
    }

    #[test]
    fn quarantine_and_disposition_capacity_cover_alarm_response() {
        assert_eq!(QUARANTINE_BAG_NESTS, WASTE_BAG_NESTS + RETAIN_BAG_NESTS);
        assert_eq!(WASTE_BAG_NESTS, RETAIN_BAG_NESTS);
        assert_eq!(DispositionGate::all().len(), DISPOSITION_GATE_COUNT);
        assert_eq!(
            total_gate_capacity(),
            DISPOSITION_GATE_COUNT * GATE_CAPACITY_PER_LANE
        );
        assert!(total_gate_capacity() >= ALARM_INPUT_COUNT + QUARANTINE_BAG_NESTS);
    }

    #[test]
    fn reset_interlock_is_not_manual_only() {
        assert!(RESET_INTERLOCK_INPUTS > ALARM_INPUT_COUNT);
        assert!(NO_MANUAL_BYPASS_GUARDS >= 3);
        assert!(RESET_PIN_D >= 8.0);
        assert!(TOP_VALVE_LIFT_KEEP_OUT_Z > BASE_Z + VALVE_Z + 180.0);
    }
}
