use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed incubator airflow filter loading recovery validation station.
//
// Intent:
// - Validate how clean and deliberately loaded service-filter cartridges affect
//   closed incubator airflow recovery without placing cells or open process
//   consumables in the challenge volume.
// - Fixture clean/loaded filter envelopes, differential pressure witness ports,
//   vane/smoke coupon recovery grids, CO2/RH/temp logger custody pockets,
//   alarm/recovery token handling, gasket/leak witnesses, barcode/COA custody,
//   disposition gates, camera evidence, and robot/service keepout gauges.
// - Model bought-in filters, probes, loggers, cameras, and pressure instruments
//   as mechanical envelopes only. Acceptance criteria, calibration, alarm
//   thresholds, filter specifications, and release decisions remain external
//   validation controls.

const OUTPUT_PREFIX: &str = "closed_incubator_airflow_filter_loading_recovery_station";

const OUTPUTS: [&str; 13] = [
    "output/closed_incubator_airflow_filter_loading_recovery_station_base_containment_deck.stl",
    "output/closed_incubator_airflow_filter_loading_recovery_station_clean_filter_cartridge_envelope.stl",
    "output/closed_incubator_airflow_filter_loading_recovery_station_loaded_filter_cartridge_envelope.stl",
    "output/closed_incubator_airflow_filter_loading_recovery_station_pressure_drop_witness_ports.stl",
    "output/closed_incubator_airflow_filter_loading_recovery_station_airflow_vane_smoke_coupon_grid.stl",
    "output/closed_incubator_airflow_filter_loading_recovery_station_co2_rh_temp_logger_pockets.stl",
    "output/closed_incubator_airflow_filter_loading_recovery_station_alarm_recovery_token_rail.stl",
    "output/closed_incubator_airflow_filter_loading_recovery_station_gasket_leak_witnesses.stl",
    "output/closed_incubator_airflow_filter_loading_recovery_station_barcode_coa_custody_panel.stl",
    "output/closed_incubator_airflow_filter_loading_recovery_station_release_hold_reject_gates.stl",
    "output/closed_incubator_airflow_filter_loading_recovery_station_camera_evidence_bridge.stl",
    "output/closed_incubator_airflow_filter_loading_recovery_station_robot_service_keepout_gauges.stl",
    "output/closed_incubator_airflow_filter_loading_recovery_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 12] = [
    "clean_filter_cartridge_envelope",
    "loaded_filter_cartridge_envelope",
    "pressure_drop_witness_ports",
    "airflow_vane_smoke_coupon_grid",
    "co2_rh_temp_logger_pockets",
    "alarm_recovery_token_rail",
    "gasket_leak_witnesses",
    "barcode_coa_custody_panel",
    "release_hold_reject_gates",
    "camera_evidence_bridge",
    "robot_service_keepout_gauges",
    "named_stl_outputs",
];

const STATION_X: f64 = 1560.0;
const STATION_Y: f64 = 980.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 52.0;
const SOCKET_DEPTH: f64 = 6.0;
const MOUNT_SLOT_COUNT: usize = 8;
const DATUM_TARGET_COUNT: usize = 4;

const FILTER_PANEL_X: f64 = 430.0;
const FILTER_PANEL_Y: f64 = 230.0;
const FILTER_PANEL_Z: f64 = 34.0;
const FILTER_MEDIA_X: f64 = 300.0;
const FILTER_MEDIA_Y: f64 = 142.0;
const FILTER_MEDIA_Z: f64 = 58.0;
const FILTER_RAIL_W: f64 = 18.0;
const FILTER_CLEARANCE_Z: f64 = 120.0;
const FILTER_CLAMP_COUNT: usize = 6;
const FILTER_PLEAT_COUNT: usize = 9;
const LOADED_OCCLUSION_RIBS: usize = 8;
const CLEAN_FILTER_POS: (f64, f64) = (-430.0, 215.0);
const LOADED_FILTER_POS: (f64, f64) = (80.0, 215.0);

const PRESSURE_PANEL_X: f64 = 310.0;
const PRESSURE_PANEL_Y: f64 = 220.0;
const PRESSURE_PANEL_Z: f64 = 118.0;
const PRESSURE_POS: (f64, f64) = (535.0, 215.0);
const PRESSURE_TAPS_PER_SIDE: usize = 5;
const PRESSURE_TAP_PITCH: f64 = 44.0;
const PRESSURE_TAP_R: f64 = 4.2;
const DIFFERENTIAL_GAUGE_COUNT: usize = 3;

const AIRFLOW_GRID_X: f64 = 620.0;
const AIRFLOW_GRID_Y: f64 = 270.0;
const AIRFLOW_GRID_Z: f64 = 26.0;
const AIRFLOW_POS: (f64, f64) = (-385.0, -90.0);
const VANE_COLS: usize = 5;
const VANE_ROWS: usize = 4;
const VANE_COUNT: usize = VANE_COLS * VANE_ROWS;
const VANE_PITCH_X: f64 = 104.0;
const VANE_PITCH_Y: f64 = 54.0;
const VANE_SLOT_X: f64 = 54.0;
const VANE_SLOT_Y: f64 = 8.0;
const SMOKE_COUPON_COUNT: usize = VANE_COUNT;
const SMOKE_COUPON_X: f64 = 36.0;
const SMOKE_COUPON_Y: f64 = 22.0;

const LOGGER_PANEL_X: f64 = 330.0;
const LOGGER_PANEL_Y: f64 = 220.0;
const LOGGER_PANEL_Z: f64 = 32.0;
const LOGGER_POS: (f64, f64) = (230.0, -70.0);
const LOGGER_TYPES: usize = 3;
const LOGGER_HEIGHTS: usize = 3;
const LOGGER_POCKET_COUNT: usize = LOGGER_TYPES * LOGGER_HEIGHTS;
const LOGGER_PITCH_X: f64 = 92.0;
const LOGGER_PITCH_Y: f64 = 56.0;
const LOGGER_POCKET_X: f64 = 58.0;
const LOGGER_POCKET_Y: f64 = 36.0;
const LOGGER_POCKET_DEPTH: f64 = 10.0;
const LOGGER_SEAL_WELLS: usize = 6;

const TOKEN_RAIL_X: f64 = 250.0;
const TOKEN_RAIL_Y: f64 = 220.0;
const TOKEN_RAIL_Z: f64 = 28.0;
const TOKEN_POS: (f64, f64) = (570.0, -95.0);
const ALARM_TOKEN_COUNT: usize = 6;
const RECOVERY_TOKEN_COUNT: usize = 6;
const RECOVERY_TIME_MARKS: usize = 7;
const TOKEN_D: f64 = 22.0;

const GASKET_PANEL_X: f64 = 300.0;
const GASKET_PANEL_Y: f64 = 140.0;
const GASKET_PANEL_Z: f64 = 22.0;
const GASKET_POS: (f64, f64) = (-560.0, -350.0);
const GASKET_WITNESS_STRIPS: usize = 8;
const LEAK_WITNESS_WELLS: usize = 10;
const GASKET_COMPRESSION_MIN: f64 = 3.0;
const GASKET_COMPRESSION_MAX: f64 = 5.0;

const CUSTODY_PANEL_X: f64 = 310.0;
const CUSTODY_PANEL_Y: f64 = 140.0;
const CUSTODY_PANEL_Z: f64 = 20.0;
const CUSTODY_POS: (f64, f64) = (-210.0, -350.0);
const BARCODE_LANDS: usize = 8;
const COA_CARD_SLOTS: usize = 4;
const CUSTODY_SEAL_WELLS: usize = 6;

const GATE_PANEL_X: f64 = 310.0;
const GATE_PANEL_Y: f64 = 140.0;
const GATE_PANEL_Z: f64 = 26.0;
const GATE_POS: (f64, f64) = (150.0, -350.0);
const GATE_LANES: usize = 3;
const RELEASE_CAPACITY: usize = 6;
const HOLD_CAPACITY: usize = 4;
const REJECT_CAPACITY: usize = 2;

const EVIDENCE_SPAN_X: f64 = 1270.0;
const EVIDENCE_BRIDGE_Y: f64 = 70.0;
const EVIDENCE_POST_Z: f64 = 220.0;
const EVIDENCE_BEAM_Z: f64 = 28.0;
const EVIDENCE_POS: (f64, f64) = (0.0, 405.0);
const CAMERA_COUNT: usize = 5;
const LIGHT_BAR_COUNT: usize = 4;
const CAMERA_TARGET_COUNT: usize = 6;
const CAMERA_UNDERSIDE_CLEARANCE: f64 = 190.0;

const KEEP_OUT_GAUGES: usize = 6;
const FRONT_ROBOT_CLEARANCE: f64 = 340.0;
const REAR_FILTER_SERVICE_CLEARANCE: f64 = 255.0;
const LEFT_FILTER_CART_CLEARANCE: f64 = 230.0;
const RIGHT_INSTRUMENT_SERVICE_CLEARANCE: f64 = 220.0;
const TOP_CARTRIDGE_LIFT_CLEARANCE: f64 = 330.0;
const KEEP_OUT_RAIL: f64 = 10.0;

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

#[derive(Clone, Copy)]
enum LoggerKind {
    Co2,
    Rh,
    Temp,
}

impl LoggerKind {
    fn all() -> [LoggerKind; LOGGER_TYPES] {
        [LoggerKind::Co2, LoggerKind::Rh, LoggerKind::Temp]
    }

    fn index(self) -> usize {
        match self {
            LoggerKind::Co2 => 0,
            LoggerKind::Rh => 1,
            LoggerKind::Temp => 2,
        }
    }

    fn name(self) -> &'static str {
        match self {
            LoggerKind::Co2 => "co2",
            LoggerKind::Rh => "rh",
            LoggerKind::Temp => "temp",
        }
    }
}

#[derive(Clone, Copy)]
enum DispositionGate {
    Release,
    Hold,
    Reject,
}

impl DispositionGate {
    fn all() -> [DispositionGate; GATE_LANES] {
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

    fn name(self) -> &'static str {
        match self {
            DispositionGate::Release => "release",
            DispositionGate::Hold => "hold",
            DispositionGate::Reject => "reject",
        }
    }

    fn capacity(self) -> usize {
        match self {
            DispositionGate::Release => RELEASE_CAPACITY,
            DispositionGate::Hold => HOLD_CAPACITY,
            DispositionGate::Reject => REJECT_CAPACITY,
        }
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = base_containment_deck();
    export(OUTPUTS[0], &deck);

    let clean_filter = clean_filter_cartridge_envelope();
    export(OUTPUTS[1], &clean_filter);

    let loaded_filter = loaded_filter_cartridge_envelope();
    export(OUTPUTS[2], &loaded_filter);

    let pressure = pressure_drop_witness_ports();
    export(OUTPUTS[3], &pressure);

    let airflow = airflow_vane_smoke_coupon_grid();
    export(OUTPUTS[4], &airflow);

    let loggers = co2_rh_temp_logger_pockets();
    export(OUTPUTS[5], &loggers);

    let tokens = alarm_recovery_token_rail();
    export(OUTPUTS[6], &tokens);

    let gaskets = gasket_leak_witnesses();
    export(OUTPUTS[7], &gaskets);

    let custody = barcode_coa_custody_panel();
    export(OUTPUTS[8], &custody);

    let gates = release_hold_reject_gates();
    export(OUTPUTS[9], &gates);

    let evidence = camera_evidence_bridge();
    export(OUTPUTS[10], &evidence);

    let keepouts = robot_service_keepout_gauges();
    export(OUTPUTS[11], &keepouts);

    let assembly = deck
        + clean_filter
        + loaded_filter
        + pressure
        + airflow
        + loggers
        + tokens
        + gaskets
        + custody
        + gates
        + evidence
        + keepouts;
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed incubator airflow filter loading recovery station:");
    println!(
        "  Deck:                   {STATION_X:.0}mm x {STATION_Y:.0}mm cleanable containment deck"
    );
    println!(
        "  Filter envelopes:       clean and loaded cartridge envelopes, each {FILTER_MEDIA_X:.0}mm x {FILTER_MEDIA_Y:.0}mm media shadow with {FILTER_CLAMP_COUNT} datum clamps"
    );
    println!(
        "  Pressure witnesses:     {} upstream/downstream taps and {DIFFERENTIAL_GAUGE_COUNT} differential gauge pockets",
        PRESSURE_TAPS_PER_SIDE * 2
    );
    println!(
        "  Recovery evidence:      {VANE_COUNT} airflow vane positions, {SMOKE_COUPON_COUNT} smoke coupons, {LOGGER_POCKET_COUNT} CO2/RH/temp logger pockets"
    );
    println!(
        "  Alarm tokens:           {ALARM_TOKEN_COUNT} alarm tokens, {RECOVERY_TOKEN_COUNT} recovery tokens, {RECOVERY_TIME_MARKS} elapsed-time marks"
    );
    println!(
        "  Custody and gates:      {BARCODE_LANDS} barcode lands, {COA_CARD_SLOTS} COA slots, release/hold/reject capacity {}",
        total_gate_capacity()
    );
    println!(
        "  Gasket/leak witnesses:  {GASKET_WITNESS_STRIPS} compression strips and {LEAK_WITNESS_WELLS} leak witness wells"
    );
    println!(
        "  Evidence bridge:        {CAMERA_COUNT} camera placeholders, {LIGHT_BAR_COUNT} light bars, {CAMERA_TARGET_COUNT} field targets"
    );
    println!(
        "  Keepouts:               {KEEP_OUT_GAUGES} gauges, {FRONT_ROBOT_CLEARANCE:.0}mm front robot, {REAR_FILTER_SERVICE_CLEARANCE:.0}mm rear filter service, {TOP_CARTRIDGE_LIFT_CLEARANCE:.0}mm cartridge lift gauge"
    );
    println!("  Required feature groups: {}", REQUIRED_FEATURES.len());
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn base_containment_deck() -> Part {
    let deck = centered_cube(
        "airflow_filter_loading_base_containment_deck",
        STATION_X,
        STATION_Y,
        DECK_Z,
    );
    let sockets = module_sockets();
    let deck_gutters = perimeter_wipe_gutters();
    deck - sockets - mounting_slots() - deck_gutters
        + perimeter_rims()
        + module_socket_label_lands()
        + datum_fiducials()
}

fn module_sockets() -> Part {
    let mut sockets = Part::empty("airflow_filter_loading_module_sockets");
    for rect in module_rects() {
        sockets = sockets
            + top_recess(
                &format!("airflow_filter_loading_{}_socket", rect.name),
                rect.center,
                rect.x + 24.0,
                rect.y + 20.0,
                SOCKET_DEPTH,
            );
    }
    sockets
}

fn perimeter_wipe_gutters() -> Part {
    let front = centered_cube(
        "airflow_filter_loading_front_wipe_gutter",
        STATION_X - 210.0,
        14.0,
        7.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 76.0, deck_top_recess_z(7.0));
    let rear = centered_cube(
        "airflow_filter_loading_rear_filter_service_wipe_gutter",
        STATION_X - 260.0,
        14.0,
        7.0,
    )
    .translate(0.0, STATION_Y / 2.0 - 78.0, deck_top_recess_z(7.0));
    let left = centered_cube(
        "airflow_filter_loading_left_wipe_gutter",
        14.0,
        STATION_Y - 220.0,
        7.0,
    )
    .translate(-STATION_X / 2.0 + 76.0, 0.0, deck_top_recess_z(7.0));
    let drain = centered_cylinder(
        "airflow_filter_loading_condensate_wipe_gutter_drain",
        8.0,
        DECK_Z + 5.0,
        32,
    )
    .translate(STATION_X / 2.0 - 82.0, -STATION_Y / 2.0 + 82.0, 0.0);

    front + rear + left + drain
}

fn perimeter_rims() -> Part {
    let rear = centered_cube(
        "airflow_filter_loading_rear_service_rim",
        STATION_X - 100.0,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - 34.0, deck_insert_z(RIM_Z));
    let left = centered_cube(
        "airflow_filter_loading_left_filter_cart_rim",
        RIM_W,
        STATION_Y - 112.0,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + 34.0, 0.0, deck_insert_z(RIM_Z));
    let right = centered_cube(
        "airflow_filter_loading_right_instrument_rim",
        RIM_W,
        STATION_Y - 160.0,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - 34.0, -24.0, deck_insert_z(RIM_Z));
    let front_low = centered_cube(
        "airflow_filter_loading_front_robot_low_stop",
        STATION_X - 250.0,
        12.0,
        18.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 36.0, deck_insert_z(18.0));

    rear + left + right + front_low
}

fn module_socket_label_lands() -> Part {
    let mut lands = Part::empty("airflow_filter_loading_module_socket_label_lands");
    for (i, rect) in module_rects().iter().enumerate() {
        lands = lands
            + centered_cube(
                format!("airflow_filter_loading_{}_socket_label_land_{i}", rect.name),
                54.0,
                8.0,
                4.0,
            )
            .translate(
                rect.center.0 - rect.x / 2.0 + 42.0,
                rect.center.1 - rect.y / 2.0 - 12.0,
                DECK_Z / 2.0 + 2.0,
            );
    }
    lands
}

fn mounting_slots() -> Part {
    let mut slots = Part::empty("airflow_filter_loading_mounting_slots");
    for (i, (x, y)) in mount_points().iter().enumerate() {
        slots = slots
            + centered_cylinder(
                format!("airflow_filter_loading_m6_mount_round_{i}"),
                3.6,
                DECK_Z + 6.0,
                24,
            )
            .translate(*x, *y, 0.0)
            + centered_cube(
                format!("airflow_filter_loading_m6_mount_slot_{i}"),
                26.0,
                8.0,
                DECK_Z + 6.0,
            )
            .translate(*x, *y, 0.0);
    }
    slots
}

fn datum_fiducials() -> Part {
    let mut fiducials = Part::empty("airflow_filter_loading_deck_datum_fiducials");
    for (i, (x, y)) in datum_points().iter().enumerate() {
        fiducials = fiducials
            + fiducial_target(&format!("airflow_filter_loading_deck_fiducial_{i}")).translate(
                *x,
                *y,
                DECK_Z / 2.0 + 2.2,
            );
    }
    fiducials
}

fn clean_filter_cartridge_envelope() -> Part {
    filter_cartridge_envelope("clean_filter", CLEAN_FILTER_POS, false)
}

fn loaded_filter_cartridge_envelope() -> Part {
    filter_cartridge_envelope("loaded_filter", LOADED_FILTER_POS, true)
}

fn filter_cartridge_envelope(prefix: &str, center: (f64, f64), loaded: bool) -> Part {
    let tray = centered_cube(
        format!("airflow_filter_loading_{prefix}_datum_tray"),
        FILTER_PANEL_X,
        FILTER_PANEL_Y,
        FILTER_PANEL_Z,
    );
    let media_recess = centered_cube(
        format!("airflow_filter_loading_{prefix}_media_recess_cut"),
        FILTER_MEDIA_X + 22.0,
        FILTER_MEDIA_Y + 18.0,
        FILTER_PANEL_Z + 4.0,
    )
    .translate(0.0, 0.0, 8.0);
    let gasket_shadow = frame_xy(
        &format!("airflow_filter_loading_{prefix}_filter_gasket_shadow"),
        FILTER_MEDIA_X + 52.0,
        FILTER_MEDIA_Y + 46.0,
        12.0,
        7.0,
    )
    .translate(0.0, 0.0, FILTER_PANEL_Z / 2.0 - 2.4);
    let cartridge = clearance_frame(
        &format!("airflow_filter_loading_{prefix}_cartridge_service_envelope"),
        FILTER_MEDIA_X + 48.0,
        FILTER_MEDIA_Y + 44.0,
        FILTER_MEDIA_Z,
    )
    .translate(0.0, 0.0, FILTER_PANEL_Z / 2.0 + FILTER_MEDIA_Z / 2.0 + 12.0);
    let media_face = filter_media_face(prefix, loaded);
    let rails = filter_datum_rails(prefix);
    let clamps = filter_toggle_clamps(prefix);
    let handles = filter_pull_handle_shadows(prefix);
    let status = if loaded {
        loaded_filter_loading_features(prefix)
    } else {
        clean_filter_reference_features(prefix)
    };

    (tray - media_recess
        + gasket_shadow
        + cartridge
        + media_face
        + rails
        + clamps
        + handles
        + status)
        .translate(center.0, center.1, deck_insert_z(FILTER_PANEL_Z))
}

fn filter_media_face(prefix: &str, loaded: bool) -> Part {
    let backing = centered_cube(
        format!("airflow_filter_loading_{prefix}_media_backing_shadow"),
        FILTER_MEDIA_X,
        FILTER_MEDIA_Y,
        8.0,
    )
    .translate(0.0, 0.0, FILTER_PANEL_Z / 2.0 + 8.0);
    let mut pleats = Part::empty(format!("airflow_filter_loading_{prefix}_pleat_witnesses"));
    for i in 0..FILTER_PLEAT_COUNT {
        let rib_w = if loaded && i % 3 == 1 { 10.0 } else { 5.0 };
        let rib_z = if loaded && i % 3 == 1 { 22.0 } else { 14.0 };
        pleats = pleats
            + centered_cube(
                format!("airflow_filter_loading_{prefix}_pleat_rib_{i}"),
                rib_w,
                FILTER_MEDIA_Y - 18.0,
                rib_z,
            )
            .translate(
                centered_index(i, FILTER_PLEAT_COUNT, FILTER_MEDIA_X / 10.0),
                0.0,
                FILTER_PANEL_Z / 2.0 + 16.0,
            );
    }

    backing + pleats
}

fn filter_datum_rails(prefix: &str) -> Part {
    let left = centered_cube(
        format!("airflow_filter_loading_{prefix}_left_hard_datum_rail"),
        FILTER_RAIL_W,
        FILTER_PANEL_Y - 30.0,
        44.0,
    )
    .translate(
        -(FILTER_MEDIA_X / 2.0 + 42.0),
        0.0,
        FILTER_PANEL_Z / 2.0 + 22.0,
    );
    let right = centered_cube(
        format!("airflow_filter_loading_{prefix}_right_spring_datum_rail"),
        FILTER_RAIL_W,
        FILTER_PANEL_Y - 30.0,
        44.0,
    )
    .translate(
        FILTER_MEDIA_X / 2.0 + 42.0,
        0.0,
        FILTER_PANEL_Z / 2.0 + 22.0,
    );
    let rear = centered_cube(
        format!("airflow_filter_loading_{prefix}_rear_a_datum_stop"),
        FILTER_MEDIA_X + 108.0,
        20.0,
        38.0,
    )
    .translate(
        0.0,
        FILTER_MEDIA_Y / 2.0 + 42.0,
        FILTER_PANEL_Z / 2.0 + 19.0,
    );
    let front_left = centered_cube(
        format!("airflow_filter_loading_{prefix}_front_left_pull_stop"),
        FILTER_MEDIA_X / 2.0 - 24.0,
        16.0,
        26.0,
    )
    .translate(
        -(FILTER_MEDIA_X / 4.0 + 22.0),
        -(FILTER_MEDIA_Y / 2.0 + 38.0),
        FILTER_PANEL_Z / 2.0 + 13.0,
    );
    let front_right = centered_cube(
        format!("airflow_filter_loading_{prefix}_front_right_pull_stop"),
        FILTER_MEDIA_X / 2.0 - 24.0,
        16.0,
        26.0,
    )
    .translate(
        FILTER_MEDIA_X / 4.0 + 22.0,
        -(FILTER_MEDIA_Y / 2.0 + 38.0),
        FILTER_PANEL_Z / 2.0 + 13.0,
    );

    left + right + rear + front_left + front_right
}

fn filter_toggle_clamps(prefix: &str) -> Part {
    let mut clamps = Part::empty(format!("airflow_filter_loading_{prefix}_toggle_clamps"));
    for (i, (x, y, rot)) in filter_clamp_points().iter().enumerate() {
        let pad = centered_cube(
            format!("airflow_filter_loading_{prefix}_toggle_clamp_pad_{i}"),
            48.0,
            18.0,
            15.0,
        )
        .rotate(0.0, 0.0, *rot)
        .translate(*x, *y, FILTER_PANEL_Z / 2.0 + 38.0);
        let hinge = centered_cylinder(
            format!("airflow_filter_loading_{prefix}_toggle_clamp_hinge_{i}"),
            4.0,
            32.0,
            22,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(*x, *y, FILTER_PANEL_Z / 2.0 + 24.0);
        clamps = clamps + pad + hinge;
    }
    clamps
}

fn filter_pull_handle_shadows(prefix: &str) -> Part {
    let left = centered_cube(
        format!("airflow_filter_loading_{prefix}_left_pull_handle_shadow"),
        70.0,
        16.0,
        26.0,
    )
    .translate(
        -90.0,
        -(FILTER_MEDIA_Y / 2.0 + 70.0),
        FILTER_PANEL_Z / 2.0 + 32.0,
    );
    let right = centered_cube(
        format!("airflow_filter_loading_{prefix}_right_pull_handle_shadow"),
        70.0,
        16.0,
        26.0,
    )
    .translate(
        90.0,
        -(FILTER_MEDIA_Y / 2.0 + 70.0),
        FILTER_PANEL_Z / 2.0 + 32.0,
    );
    left + right
}

fn clean_filter_reference_features(prefix: &str) -> Part {
    let label = centered_cube(
        format!("airflow_filter_loading_{prefix}_clean_reference_label_land"),
        112.0,
        12.0,
        6.0,
    )
    .translate(0.0, FILTER_PANEL_Y / 2.0 - 20.0, FILTER_PANEL_Z / 2.0 + 8.0);
    let mut open_area_tabs = Part::empty(format!(
        "airflow_filter_loading_{prefix}_open_area_reference_tabs"
    ));
    for i in 0..4 {
        open_area_tabs = open_area_tabs
            + centered_cube(
                format!("airflow_filter_loading_{prefix}_open_area_reference_tab_{i}"),
                36.0,
                10.0,
                8.0,
            )
            .translate(
                centered_index(i, 4, 58.0),
                -(FILTER_PANEL_Y / 2.0 - 22.0),
                FILTER_PANEL_Z / 2.0 + 9.0,
            );
    }
    label + open_area_tabs
}

fn loaded_filter_loading_features(prefix: &str) -> Part {
    let mut ribs = Part::empty(format!(
        "airflow_filter_loading_{prefix}_load_occlusion_ribs"
    ));
    for i in 0..LOADED_OCCLUSION_RIBS {
        ribs = ribs
            + centered_cube(
                format!("airflow_filter_loading_{prefix}_graded_load_rib_{i}"),
                9.0 + (i % 3) as f64 * 2.5,
                FILTER_MEDIA_Y - 24.0,
                28.0,
            )
            .translate(
                centered_index(i, LOADED_OCCLUSION_RIBS, FILTER_MEDIA_X / 9.0),
                0.0,
                FILTER_PANEL_Z / 2.0 + 26.0,
            );
    }
    let dust_coupons = loaded_filter_dust_coupons(prefix);
    let load_label = centered_cube(
        format!("airflow_filter_loading_{prefix}_loaded_reference_label_land"),
        132.0,
        12.0,
        6.0,
    )
    .translate(0.0, FILTER_PANEL_Y / 2.0 - 20.0, FILTER_PANEL_Z / 2.0 + 8.0);

    ribs + dust_coupons + load_label
}

fn loaded_filter_dust_coupons(prefix: &str) -> Part {
    let mut coupons = Part::empty(format!("airflow_filter_loading_{prefix}_dust_load_coupons"));
    for i in 0..4 {
        coupons = coupons
            + centered_cube(
                format!("airflow_filter_loading_{prefix}_dust_coupon_land_{i}"),
                42.0,
                18.0,
                8.0,
            )
            .translate(
                centered_index(i, 4, 62.0),
                -(FILTER_PANEL_Y / 2.0 - 26.0),
                FILTER_PANEL_Z / 2.0 + 10.0,
            );
    }
    coupons
}

fn pressure_drop_witness_ports() -> Part {
    let panel = centered_cube(
        "airflow_filter_loading_pressure_drop_witness_bulkhead_panel",
        PRESSURE_PANEL_X,
        PRESSURE_PANEL_Y,
        PRESSURE_PANEL_Z,
    );
    let upstream_holes = pressure_port_holes("upstream", 32.0);
    let downstream_holes = pressure_port_holes("downstream", -32.0);
    let upstream_collars = pressure_port_collars("upstream", 32.0);
    let downstream_collars = pressure_port_collars("downstream", -32.0);
    let gauges = differential_pressure_gauge_pockets();
    let baseline_cards = pressure_baseline_coupon_pockets();
    let delta_arrow = centered_cube(
        "airflow_filter_loading_pressure_drop_delta_reference_land",
        PRESSURE_PANEL_X - 62.0,
        8.0,
        14.0,
    )
    .translate(0.0, -PRESSURE_PANEL_Y / 2.0 - 10.0, 36.0);

    (panel - upstream_holes - downstream_holes
        + upstream_collars
        + downstream_collars
        + gauges
        + baseline_cards
        + delta_arrow)
        .translate(
            PRESSURE_POS.0,
            PRESSURE_POS.1,
            deck_insert_z(PRESSURE_PANEL_Z),
        )
}

fn pressure_port_holes(row: &str, z_offset: f64) -> Part {
    let mut holes = Part::empty(format!("airflow_filter_loading_{row}_pressure_port_holes"));
    for i in 0..PRESSURE_TAPS_PER_SIDE {
        holes = holes
            + centered_cylinder(
                format!("airflow_filter_loading_{row}_pressure_tap_bore_{i}"),
                PRESSURE_TAP_R,
                PRESSURE_PANEL_Y + 18.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(i, PRESSURE_TAPS_PER_SIDE, PRESSURE_TAP_PITCH),
                0.0,
                z_offset,
            );
    }
    holes
}

fn pressure_port_collars(row: &str, z_offset: f64) -> Part {
    let mut collars = Part::empty(format!(
        "airflow_filter_loading_{row}_pressure_port_collars"
    ));
    for i in 0..PRESSURE_TAPS_PER_SIDE {
        let x = centered_index(i, PRESSURE_TAPS_PER_SIDE, PRESSURE_TAP_PITCH);
        let collar = centered_cylinder(
            format!("airflow_filter_loading_{row}_pressure_tap_collar_{i}"),
            14.0,
            12.0,
            36,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, -PRESSURE_PANEL_Y / 2.0 - 5.0, z_offset);
        let label = centered_cube(
            format!("airflow_filter_loading_{row}_pressure_tap_label_land_{i}"),
            34.0,
            6.0,
            10.0,
        )
        .translate(x, -PRESSURE_PANEL_Y / 2.0 - 14.0, z_offset + 18.0);
        collars = collars + collar + label;
    }
    collars
}

fn differential_pressure_gauge_pockets() -> Part {
    let mut pockets = Part::empty("airflow_filter_loading_differential_pressure_gauge_pockets");
    for i in 0..DIFFERENTIAL_GAUGE_COUNT {
        let x = centered_index(i, DIFFERENTIAL_GAUGE_COUNT, 76.0);
        let pocket = centered_cube(
            format!("airflow_filter_loading_differential_pressure_gauge_pocket_{i}"),
            54.0,
            20.0,
            34.0,
        )
        .translate(x, PRESSURE_PANEL_Y / 2.0 + 8.0, 10.0);
        let face = centered_cube(
            format!("airflow_filter_loading_differential_pressure_gauge_face_{i}"),
            42.0,
            5.0,
            24.0,
        )
        .translate(x, PRESSURE_PANEL_Y / 2.0 + 22.0, 10.0);
        pockets = pockets + pocket + face;
    }
    pockets
}

fn pressure_baseline_coupon_pockets() -> Part {
    let mut pockets = Part::empty("airflow_filter_loading_pressure_baseline_coupon_pockets");
    for i in 0..4 {
        pockets = pockets
            + centered_cube(
                format!("airflow_filter_loading_pressure_baseline_coupon_slot_{i}"),
                42.0,
                16.0,
                18.0,
            )
            .translate(
                centered_index(i, 4, 54.0),
                0.0,
                -(PRESSURE_PANEL_Z / 2.0 - 18.0),
            );
    }
    pockets
}

fn airflow_vane_smoke_coupon_grid() -> Part {
    let panel = centered_cube(
        "airflow_filter_loading_airflow_vane_smoke_coupon_grid_panel",
        AIRFLOW_GRID_X,
        AIRFLOW_GRID_Y,
        AIRFLOW_GRID_Z,
    );
    let vane_slots = airflow_vane_slots();
    let vane_flags = airflow_vane_flags();
    let smoke = smoke_coupon_pockets();
    let rails = grid_boundary_rails();
    let baseline_strip = centered_cube(
        "airflow_filter_loading_smoke_grid_baseline_photo_strip",
        AIRFLOW_GRID_X - 92.0,
        14.0,
        10.0,
    )
    .translate(0.0, AIRFLOW_GRID_Y / 2.0 - 22.0, AIRFLOW_GRID_Z / 2.0 + 5.0);

    (panel - vane_slots + vane_flags + smoke + rails + baseline_strip).translate(
        AIRFLOW_POS.0,
        AIRFLOW_POS.1,
        deck_insert_z(AIRFLOW_GRID_Z),
    )
}

fn airflow_vane_slots() -> Part {
    let mut slots = Part::empty("airflow_filter_loading_airflow_vane_slot_cuts");
    for row in 0..VANE_ROWS {
        for col in 0..VANE_COLS {
            let index = vane_index(col, row);
            let (x, y) = vane_center(col, row);
            slots = slots
                + centered_cube(
                    format!("airflow_filter_loading_vane_slot_cut_{index}"),
                    VANE_SLOT_X,
                    VANE_SLOT_Y,
                    AIRFLOW_GRID_Z + 4.0,
                )
                .translate(x, y, 0.0);
        }
    }
    slots
}

fn airflow_vane_flags() -> Part {
    let mut flags = Part::empty("airflow_filter_loading_airflow_vane_flags");
    for row in 0..VANE_ROWS {
        for col in 0..VANE_COLS {
            let index = vane_index(col, row);
            let (x, y) = vane_center(col, row);
            let height = 18.0 + (row as f64 * 5.0);
            flags = flags
                + centered_cube(
                    format!("airflow_filter_loading_vane_deflection_flag_{index}"),
                    6.0,
                    34.0,
                    height,
                )
                .rotate(0.0, 0.0, if col % 2 == 0 { -8.0 } else { 8.0 })
                .translate(x - 20.0, y, AIRFLOW_GRID_Z / 2.0 + height / 2.0);
        }
    }
    flags
}

fn smoke_coupon_pockets() -> Part {
    let mut coupons = Part::empty("airflow_filter_loading_smoke_coupon_pockets");
    for row in 0..VANE_ROWS {
        for col in 0..VANE_COLS {
            let index = vane_index(col, row);
            let (x, y) = vane_center(col, row);
            let pocket = centered_cube(
                format!("airflow_filter_loading_smoke_coupon_pocket_{index}"),
                SMOKE_COUPON_X,
                SMOKE_COUPON_Y,
                7.0,
            )
            .translate(x + 28.0, y, AIRFLOW_GRID_Z / 2.0 + 3.5);
            let witness = centered_cylinder(
                format!("airflow_filter_loading_smoke_coupon_round_witness_{index}"),
                7.0,
                5.0,
                28,
            )
            .translate(x + 28.0, y, AIRFLOW_GRID_Z / 2.0 + 9.0);
            coupons = coupons + pocket + witness;
        }
    }
    coupons
}

fn grid_boundary_rails() -> Part {
    let front = centered_cube(
        "airflow_filter_loading_airflow_grid_front_reference_rail",
        AIRFLOW_GRID_X - 48.0,
        10.0,
        18.0,
    )
    .translate(
        0.0,
        -AIRFLOW_GRID_Y / 2.0 + 18.0,
        AIRFLOW_GRID_Z / 2.0 + 9.0,
    );
    let rear = centered_cube(
        "airflow_filter_loading_airflow_grid_rear_reference_rail",
        AIRFLOW_GRID_X - 48.0,
        10.0,
        18.0,
    )
    .translate(0.0, AIRFLOW_GRID_Y / 2.0 - 18.0, AIRFLOW_GRID_Z / 2.0 + 9.0);
    let centerline = centered_cube(
        "airflow_filter_loading_airflow_grid_centerline_reference",
        12.0,
        AIRFLOW_GRID_Y - 68.0,
        14.0,
    )
    .translate(0.0, 0.0, AIRFLOW_GRID_Z / 2.0 + 7.0);

    front + rear + centerline
}

fn co2_rh_temp_logger_pockets() -> Part {
    let panel = centered_cube(
        "airflow_filter_loading_co2_rh_temp_logger_pocket_panel",
        LOGGER_PANEL_X,
        LOGGER_PANEL_Y,
        LOGGER_PANEL_Z,
    );
    let pockets = logger_pocket_cuts();
    let collars = logger_pocket_collars();
    let seals = logger_seal_wells();
    let cable_comb = logger_cable_comb();
    let tier_masts = logger_height_tier_masts();

    (panel - pockets + collars + seals + cable_comb + tier_masts).translate(
        LOGGER_POS.0,
        LOGGER_POS.1,
        deck_insert_z(LOGGER_PANEL_Z),
    )
}

fn logger_pocket_cuts() -> Part {
    let mut cuts = Part::empty("airflow_filter_loading_logger_pocket_cuts");
    for kind in LoggerKind::all() {
        for height in 0..LOGGER_HEIGHTS {
            cuts = cuts
                + centered_cube(
                    format!(
                        "airflow_filter_loading_{}_height_{}_logger_pocket_cut",
                        kind.name(),
                        height
                    ),
                    LOGGER_POCKET_X,
                    LOGGER_POCKET_Y,
                    LOGGER_POCKET_DEPTH + 3.0,
                )
                .translate(
                    logger_x(kind),
                    logger_y(height),
                    LOGGER_PANEL_Z / 2.0 - LOGGER_POCKET_DEPTH / 2.0,
                );
        }
    }
    cuts
}

fn logger_pocket_collars() -> Part {
    let mut collars = Part::empty("airflow_filter_loading_logger_pocket_collars");
    for kind in LoggerKind::all() {
        for height in 0..LOGGER_HEIGHTS {
            let name = format!(
                "airflow_filter_loading_{}_height_{}_logger_bezel",
                kind.name(),
                height
            );
            collars = collars
                + frame_xy(
                    &name,
                    LOGGER_POCKET_X + 14.0,
                    LOGGER_POCKET_Y + 12.0,
                    6.0,
                    6.0,
                )
                .translate(
                    logger_x(kind),
                    logger_y(height),
                    LOGGER_PANEL_Z / 2.0 + 3.0,
                );
        }
    }
    collars
}

fn logger_seal_wells() -> Part {
    let mut wells = Part::empty("airflow_filter_loading_logger_tamper_seal_wells");
    for i in 0..LOGGER_SEAL_WELLS {
        wells = wells
            + centered_cylinder(
                format!("airflow_filter_loading_logger_tamper_seal_well_{i}"),
                8.0,
                6.0,
                30,
            )
            .translate(
                centered_index(i % 3, 3, 88.0),
                centered_index(i / 3, 2, 170.0),
                LOGGER_PANEL_Z / 2.0 + 3.0,
            );
    }
    wells
}

fn logger_cable_comb() -> Part {
    let comb = centered_cube(
        "airflow_filter_loading_logger_cable_comb_body",
        LOGGER_PANEL_X - 58.0,
        12.0,
        18.0,
    )
    .translate(
        0.0,
        -LOGGER_PANEL_Y / 2.0 + 22.0,
        LOGGER_PANEL_Z / 2.0 + 9.0,
    );
    let mut notches = Part::empty("airflow_filter_loading_logger_cable_comb_notches");
    for i in 0..LOGGER_POCKET_COUNT {
        notches = notches
            + centered_cube(
                format!("airflow_filter_loading_logger_cable_comb_notch_{i}"),
                8.0,
                14.0,
                20.0,
            )
            .translate(
                centered_index(i, LOGGER_POCKET_COUNT, 26.0),
                -LOGGER_PANEL_Y / 2.0 + 22.0,
                LOGGER_PANEL_Z / 2.0 + 9.0,
            );
    }
    comb - notches
}

fn logger_height_tier_masts() -> Part {
    let mut masts = Part::empty("airflow_filter_loading_logger_height_tier_masts");
    for height in 0..LOGGER_HEIGHTS {
        let z = 40.0 + height as f64 * 36.0;
        masts = masts
            + centered_cube(
                format!("airflow_filter_loading_logger_height_tier_{}_mast", height),
                12.0,
                22.0,
                z,
            )
            .translate(
                LOGGER_PANEL_X / 2.0 - 22.0,
                logger_y(height),
                LOGGER_PANEL_Z / 2.0 + z / 2.0,
            );
    }
    masts
}

fn alarm_recovery_token_rail() -> Part {
    let rail = centered_cube(
        "airflow_filter_loading_alarm_recovery_token_rail_body",
        TOKEN_RAIL_X,
        TOKEN_RAIL_Y,
        TOKEN_RAIL_Z,
    );
    let alarm = token_pockets("alarm", ALARM_TOKEN_COUNT, 42.0);
    let recovery = token_pockets("recovery", RECOVERY_TOKEN_COUNT, -42.0);
    let time_marks = recovery_time_scale();
    let trip_flag = centered_cube(
        "airflow_filter_loading_alarm_trip_flag_shadow",
        44.0,
        18.0,
        42.0,
    )
    .translate(-TOKEN_RAIL_X / 2.0 + 38.0, 0.0, TOKEN_RAIL_Z / 2.0 + 21.0);
    let reset_key = centered_cube(
        "airflow_filter_loading_recovery_reset_key_land",
        54.0,
        22.0,
        12.0,
    )
    .translate(TOKEN_RAIL_X / 2.0 - 42.0, 0.0, TOKEN_RAIL_Z / 2.0 + 6.0);

    (rail + alarm + recovery + time_marks + trip_flag + reset_key).translate(
        TOKEN_POS.0,
        TOKEN_POS.1,
        deck_insert_z(TOKEN_RAIL_Z),
    )
}

fn token_pockets(prefix: &str, count: usize, y: f64) -> Part {
    let mut tokens = Part::empty(format!("airflow_filter_loading_{prefix}_token_pockets"));
    for i in 0..count {
        let x = centered_index(i, count, 32.0);
        let pocket = centered_cylinder(
            format!("airflow_filter_loading_{prefix}_token_pocket_{i}"),
            TOKEN_D / 2.0,
            8.0,
            32,
        )
        .translate(x, y, TOKEN_RAIL_Z / 2.0 + 4.0);
        let tick = centered_cube(
            format!("airflow_filter_loading_{prefix}_token_tick_{i}"),
            4.0,
            10.0,
            10.0,
        )
        .translate(x, y + 22.0, TOKEN_RAIL_Z / 2.0 + 5.0);
        tokens = tokens + pocket + tick;
    }
    tokens
}

fn recovery_time_scale() -> Part {
    let mut marks = Part::empty("airflow_filter_loading_recovery_time_scale_marks");
    for i in 0..RECOVERY_TIME_MARKS {
        marks = marks
            + centered_cube(
                format!("airflow_filter_loading_recovery_elapsed_time_mark_{i}"),
                4.0,
                68.0,
                10.0,
            )
            .translate(
                centered_index(i, RECOVERY_TIME_MARKS, 34.0),
                0.0,
                TOKEN_RAIL_Z / 2.0 + 5.0,
            );
    }
    marks
}

fn gasket_leak_witnesses() -> Part {
    let panel = centered_cube(
        "airflow_filter_loading_gasket_leak_witness_panel",
        GASKET_PANEL_X,
        GASKET_PANEL_Y,
        GASKET_PANEL_Z,
    );
    let gasket_frame = frame_xy(
        "airflow_filter_loading_filter_gasket_compression_frame",
        GASKET_PANEL_X - 64.0,
        GASKET_PANEL_Y - 38.0,
        14.0,
        10.0,
    )
    .translate(0.0, 0.0, GASKET_PANEL_Z / 2.0 + 5.0);
    let strips = gasket_compression_strips();
    let wells = leak_witness_wells();
    let min_max = gasket_compression_gauge_steps();

    (panel + gasket_frame + strips + wells + min_max).translate(
        GASKET_POS.0,
        GASKET_POS.1,
        deck_insert_z(GASKET_PANEL_Z),
    )
}

fn gasket_compression_strips() -> Part {
    let mut strips = Part::empty("airflow_filter_loading_gasket_compression_strips");
    for i in 0..GASKET_WITNESS_STRIPS {
        let x = centered_index(i % 4, 4, 56.0);
        let y = centered_index(i / 4, 2, 72.0);
        strips = strips
            + centered_cube(
                format!("airflow_filter_loading_gasket_compression_strip_{i}"),
                44.0,
                8.0,
                8.0,
            )
            .rotate(0.0, 0.0, if i % 2 == 0 { 0.0 } else { 90.0 })
            .translate(x, y, GASKET_PANEL_Z / 2.0 + 4.0);
    }
    strips
}

fn leak_witness_wells() -> Part {
    let mut wells = Part::empty("airflow_filter_loading_leak_witness_wells");
    for i in 0..LEAK_WITNESS_WELLS {
        wells = wells
            + centered_cylinder(
                format!("airflow_filter_loading_leak_witness_well_{i}"),
                8.0,
                8.0,
                30,
            )
            .translate(
                centered_index(i % 5, 5, 46.0),
                centered_index(i / 5, 2, 96.0),
                GASKET_PANEL_Z / 2.0 + 4.0,
            );
    }
    wells
}

fn gasket_compression_gauge_steps() -> Part {
    let min = centered_cube(
        "airflow_filter_loading_gasket_3mm_compression_gauge_land",
        46.0,
        18.0,
        GASKET_COMPRESSION_MIN,
    )
    .translate(
        -GASKET_PANEL_X / 2.0 + 40.0,
        0.0,
        GASKET_PANEL_Z / 2.0 + GASKET_COMPRESSION_MIN / 2.0,
    );
    let max = centered_cube(
        "airflow_filter_loading_gasket_5mm_compression_gauge_land",
        46.0,
        18.0,
        GASKET_COMPRESSION_MAX,
    )
    .translate(
        GASKET_PANEL_X / 2.0 - 40.0,
        0.0,
        GASKET_PANEL_Z / 2.0 + GASKET_COMPRESSION_MAX / 2.0,
    );
    min + max
}

fn barcode_coa_custody_panel() -> Part {
    let panel = centered_cube(
        "airflow_filter_loading_barcode_coa_custody_panel",
        CUSTODY_PANEL_X,
        CUSTODY_PANEL_Y,
        CUSTODY_PANEL_Z,
    );
    let barcodes = barcode_lands();
    let coa = coa_card_slots();
    let seals = custody_seal_wells();
    let handoff = centered_cube(
        "airflow_filter_loading_custody_handoff_card_lane",
        CUSTODY_PANEL_X - 54.0,
        14.0,
        10.0,
    )
    .translate(
        0.0,
        -CUSTODY_PANEL_Y / 2.0 + 20.0,
        CUSTODY_PANEL_Z / 2.0 + 5.0,
    );

    (panel + barcodes + coa + seals + handoff).translate(
        CUSTODY_POS.0,
        CUSTODY_POS.1,
        deck_insert_z(CUSTODY_PANEL_Z),
    )
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty("airflow_filter_loading_barcode_lands");
    for i in 0..BARCODE_LANDS {
        let x = centered_index(i % 4, 4, 64.0);
        let y = 28.0 + centered_index(i / 4, 2, 34.0);
        lands = lands
            + centered_cube(
                format!("airflow_filter_loading_barcode_label_land_{i}"),
                52.0,
                16.0,
                6.0,
            )
            .translate(x, y, CUSTODY_PANEL_Z / 2.0 + 3.0)
            + barcode_stripes(i, x, y);
    }
    lands
}

fn barcode_stripes(index: usize, x: f64, y: f64) -> Part {
    let mut stripes = Part::empty(format!("airflow_filter_loading_barcode_{index}_stripes"));
    for stripe in 0..5 {
        stripes = stripes
            + centered_cube(
                format!("airflow_filter_loading_barcode_{index}_stripe_{stripe}"),
                2.0 + (stripe % 2) as f64,
                16.6,
                7.0,
            )
            .translate(
                x + centered_index(stripe, 5, 7.0),
                y,
                CUSTODY_PANEL_Z / 2.0 + 6.0,
            );
    }
    stripes
}

fn coa_card_slots() -> Part {
    let mut slots = Part::empty("airflow_filter_loading_coa_card_slots");
    for i in 0..COA_CARD_SLOTS {
        slots = slots
            + centered_cube(
                format!("airflow_filter_loading_coa_card_slot_{i}"),
                58.0,
                26.0,
                8.0,
            )
            .translate(
                centered_index(i, COA_CARD_SLOTS, 66.0),
                -CUSTODY_PANEL_Y / 2.0 + 48.0,
                CUSTODY_PANEL_Z / 2.0 + 4.0,
            );
    }
    slots
}

fn custody_seal_wells() -> Part {
    let mut wells = Part::empty("airflow_filter_loading_custody_tamper_seal_wells");
    for i in 0..CUSTODY_SEAL_WELLS {
        wells = wells
            + centered_cylinder(
                format!("airflow_filter_loading_custody_tamper_seal_well_{i}"),
                6.0,
                6.0,
                28,
            )
            .translate(
                centered_index(i, CUSTODY_SEAL_WELLS, 38.0),
                -2.0,
                CUSTODY_PANEL_Z / 2.0 + 3.0,
            );
    }
    wells
}

fn release_hold_reject_gates() -> Part {
    let panel = centered_cube(
        "airflow_filter_loading_release_hold_reject_gate_panel",
        GATE_PANEL_X,
        GATE_PANEL_Y,
        GATE_PANEL_Z,
    );
    let lanes = disposition_gate_lanes();
    let gate_flags = disposition_gate_flags();
    let token_stops = disposition_token_stops();

    (panel + lanes + gate_flags + token_stops).translate(
        GATE_POS.0,
        GATE_POS.1,
        deck_insert_z(GATE_PANEL_Z),
    )
}

fn disposition_gate_lanes() -> Part {
    let mut lanes = Part::empty("airflow_filter_loading_disposition_gate_lanes");
    for gate in DispositionGate::all() {
        let x = disposition_gate_x(gate);
        let slot = centered_cube(
            format!("airflow_filter_loading_{}_gate_token_lane", gate.name()),
            82.0,
            96.0,
            10.0,
        )
        .translate(x, 0.0, GATE_PANEL_Z / 2.0 + 5.0);
        let pocket_count = gate.capacity();
        let mut pockets = Part::empty(format!(
            "airflow_filter_loading_{}_gate_capacity_pockets",
            gate.name()
        ));
        for i in 0..pocket_count {
            pockets = pockets
                + centered_cylinder(
                    format!(
                        "airflow_filter_loading_{}_capacity_token_pocket_{i}",
                        gate.name()
                    ),
                    5.0,
                    5.0,
                    24,
                )
                .translate(
                    x + centered_index(i % 3, 3, 20.0),
                    centered_index(i / 3, 2, 32.0),
                    GATE_PANEL_Z / 2.0 + 11.0,
                );
        }
        lanes = lanes + slot + pockets;
    }
    lanes
}

fn disposition_gate_flags() -> Part {
    let mut flags = Part::empty("airflow_filter_loading_disposition_gate_flags");
    for gate in DispositionGate::all() {
        let x = disposition_gate_x(gate);
        flags = flags
            + centered_cube(
                format!("airflow_filter_loading_{}_upright_gate_flag", gate.name()),
                58.0,
                8.0,
                42.0,
            )
            .translate(x, GATE_PANEL_Y / 2.0 - 22.0, GATE_PANEL_Z / 2.0 + 21.0);
    }
    flags
}

fn disposition_token_stops() -> Part {
    let front = centered_cube(
        "airflow_filter_loading_disposition_front_token_stop",
        GATE_PANEL_X - 42.0,
        10.0,
        18.0,
    )
    .translate(0.0, -GATE_PANEL_Y / 2.0 + 18.0, GATE_PANEL_Z / 2.0 + 9.0);
    let rear = centered_cube(
        "airflow_filter_loading_disposition_rear_token_stop",
        GATE_PANEL_X - 42.0,
        10.0,
        18.0,
    )
    .translate(0.0, GATE_PANEL_Y / 2.0 - 18.0, GATE_PANEL_Z / 2.0 + 9.0);
    front + rear
}

fn camera_evidence_bridge() -> Part {
    let left_post = centered_cube(
        "airflow_filter_loading_evidence_bridge_left_post",
        32.0,
        42.0,
        EVIDENCE_POST_Z,
    )
    .translate(
        -EVIDENCE_SPAN_X / 2.0,
        0.0,
        DECK_Z / 2.0 + EVIDENCE_POST_Z / 2.0,
    );
    let right_post = centered_cube(
        "airflow_filter_loading_evidence_bridge_right_post",
        32.0,
        42.0,
        EVIDENCE_POST_Z,
    )
    .translate(
        EVIDENCE_SPAN_X / 2.0,
        0.0,
        DECK_Z / 2.0 + EVIDENCE_POST_Z / 2.0,
    );
    let crossbar = centered_cube(
        "airflow_filter_loading_evidence_bridge_crossbar",
        EVIDENCE_SPAN_X + 76.0,
        EVIDENCE_BRIDGE_Y,
        EVIDENCE_BEAM_Z,
    )
    .translate(
        0.0,
        0.0,
        DECK_Z / 2.0 + CAMERA_UNDERSIDE_CLEARANCE + EVIDENCE_BEAM_Z / 2.0,
    );
    let cameras = evidence_cameras();
    let lights = evidence_light_bars();
    let targets = evidence_field_targets();

    (left_post + right_post + crossbar + cameras + lights + targets).translate(
        EVIDENCE_POS.0,
        EVIDENCE_POS.1,
        0.0,
    )
}

fn evidence_cameras() -> Part {
    let mut cameras = Part::empty("airflow_filter_loading_evidence_camera_placeholders");
    for i in 0..CAMERA_COUNT {
        cameras = cameras
            + centered_cube(
                format!("airflow_filter_loading_evidence_camera_body_{i}"),
                44.0,
                30.0,
                26.0,
            )
            .translate(
                centered_index(i, CAMERA_COUNT, EVIDENCE_SPAN_X / 5.2),
                -EVIDENCE_BRIDGE_Y / 2.0 - 18.0,
                DECK_Z / 2.0 + CAMERA_UNDERSIDE_CLEARANCE - 14.0,
            )
            + centered_cylinder(
                format!("airflow_filter_loading_evidence_camera_lens_{i}"),
                7.0,
                8.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(i, CAMERA_COUNT, EVIDENCE_SPAN_X / 5.2),
                -EVIDENCE_BRIDGE_Y / 2.0 - 36.0,
                DECK_Z / 2.0 + CAMERA_UNDERSIDE_CLEARANCE - 14.0,
            );
    }
    cameras
}

fn evidence_light_bars() -> Part {
    let mut lights = Part::empty("airflow_filter_loading_evidence_light_bars");
    for i in 0..LIGHT_BAR_COUNT {
        lights = lights
            + centered_cube(
                format!("airflow_filter_loading_evidence_light_bar_{i}"),
                180.0,
                12.0,
                10.0,
            )
            .translate(
                centered_index(i, LIGHT_BAR_COUNT, 245.0),
                EVIDENCE_BRIDGE_Y / 2.0 + 10.0,
                DECK_Z / 2.0 + CAMERA_UNDERSIDE_CLEARANCE - 28.0,
            );
    }
    lights
}

fn evidence_field_targets() -> Part {
    let mut targets = Part::empty("airflow_filter_loading_evidence_field_targets");
    for i in 0..CAMERA_TARGET_COUNT {
        targets = targets
            + fiducial_target(&format!("airflow_filter_loading_camera_field_target_{i}"))
                .translate(
                    centered_index(i, CAMERA_TARGET_COUNT, 205.0),
                    -EVIDENCE_BRIDGE_Y / 2.0 - 34.0,
                    DECK_Z / 2.0 + 6.0,
                );
    }
    targets
}

fn robot_service_keepout_gauges() -> Part {
    let front_robot = centered_cube(
        "airflow_filter_loading_front_robot_approach_keepout_gauge",
        STATION_X - 220.0,
        KEEP_OUT_RAIL,
        KEEP_OUT_RAIL,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 + FRONT_ROBOT_CLEARANCE,
        KEEP_OUT_RAIL / 2.0,
    );
    let rear_service = centered_cube(
        "airflow_filter_loading_rear_filter_service_keepout_gauge",
        STATION_X - 250.0,
        KEEP_OUT_RAIL,
        KEEP_OUT_RAIL,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - REAR_FILTER_SERVICE_CLEARANCE,
        KEEP_OUT_RAIL / 2.0,
    );
    let left_cart = centered_cube(
        "airflow_filter_loading_left_filter_cart_keepout_gauge",
        KEEP_OUT_RAIL,
        STATION_Y - 200.0,
        KEEP_OUT_RAIL,
    )
    .translate(
        -STATION_X / 2.0 + LEFT_FILTER_CART_CLEARANCE,
        0.0,
        KEEP_OUT_RAIL / 2.0,
    );
    let right_service = centered_cube(
        "airflow_filter_loading_right_instrument_service_keepout_gauge",
        KEEP_OUT_RAIL,
        STATION_Y - 220.0,
        KEEP_OUT_RAIL,
    )
    .translate(
        STATION_X / 2.0 - RIGHT_INSTRUMENT_SERVICE_CLEARANCE,
        -10.0,
        KEEP_OUT_RAIL / 2.0,
    );
    let top_lift = centered_cube(
        "airflow_filter_loading_filter_cartridge_vertical_lift_keepout_gauge",
        82.0,
        16.0,
        TOP_CARTRIDGE_LIFT_CLEARANCE,
    )
    .translate(
        CLEAN_FILTER_POS.0,
        CLEAN_FILTER_POS.1 + FILTER_PANEL_Y / 2.0 - 38.0,
        TOP_CARTRIDGE_LIFT_CLEARANCE / 2.0,
    );
    let camera_keepout = centered_cube(
        "airflow_filter_loading_camera_bridge_service_keepout_gauge",
        EVIDENCE_SPAN_X - 90.0,
        KEEP_OUT_RAIL,
        KEEP_OUT_RAIL,
    )
    .translate(
        EVIDENCE_POS.0,
        EVIDENCE_POS.1 - EVIDENCE_BRIDGE_Y / 2.0 - 22.0,
        KEEP_OUT_RAIL / 2.0,
    );

    front_robot + rear_service + left_cart + right_service + top_lift + camera_keepout
}

fn top_recess(name: &str, center: (f64, f64), x: f64, y: f64, depth: f64) -> Part {
    centered_cube(name, x, y, depth + 0.2).translate(
        center.0,
        center.1,
        DECK_Z / 2.0 - depth / 2.0 + 0.1,
    )
}

fn deck_top_recess_z(depth: f64) -> f64 {
    DECK_Z / 2.0 - depth / 2.0 + 0.1
}

fn deck_insert_z(height: f64) -> f64 {
    DECK_Z / 2.0 + height / 2.0
}

fn frame_xy(name: &str, x: f64, y: f64, wall: f64, z: f64) -> Part {
    let outer = centered_cube(format!("{name}_outer"), x, y, z);
    let inner = centered_cube(
        format!("{name}_inner_cut"),
        x - 2.0 * wall,
        y - 2.0 * wall,
        z + 1.0,
    );
    outer - inner
}

fn clearance_frame(name: &str, x: f64, y: f64, z: f64) -> Part {
    let mut frame = Part::empty(format!("{name}_frame"));
    for (i, sx) in [-1.0, 1.0].iter().enumerate() {
        for (j, sy) in [-1.0, 1.0].iter().enumerate() {
            frame =
                frame
                    + centered_cube(format!("{name}_vertical_post_{i}_{j}"), 10.0, 10.0, z)
                        .translate(sx * (x / 2.0 - 5.0), sy * (y / 2.0 - 5.0), 0.0);
        }
    }
    for (k, zsign) in [-1.0, 1.0].iter().enumerate() {
        let zpos = zsign * (z / 2.0 - 5.0);
        frame = frame
            + centered_cube(format!("{name}_front_x_rail_{k}"), x, 10.0, 10.0).translate(
                0.0,
                -(y / 2.0 - 5.0),
                zpos,
            )
            + centered_cube(format!("{name}_rear_x_rail_{k}"), x, 10.0, 10.0).translate(
                0.0,
                y / 2.0 - 5.0,
                zpos,
            )
            + centered_cube(format!("{name}_left_y_rail_{k}"), 10.0, y, 10.0).translate(
                -(x / 2.0 - 5.0),
                0.0,
                zpos,
            )
            + centered_cube(format!("{name}_right_y_rail_{k}"), 10.0, y, 10.0).translate(
                x / 2.0 - 5.0,
                0.0,
                zpos,
            );
    }
    frame
}

fn fiducial_target(name: &str) -> Part {
    let plate = centered_cube(format!("{name}_plate"), 30.0, 30.0, 4.0);
    let outer = centered_cylinder(format!("{name}_outer_ring"), 10.0, 5.0, 36);
    let center = centered_cylinder(format!("{name}_center_cut"), 3.0, 6.0, 24);
    let x_bar = centered_cube(format!("{name}_x_bar"), 22.0, 2.0, 6.0);
    let y_bar = centered_cube(format!("{name}_y_bar"), 2.0, 22.0, 6.0);
    plate + outer + x_bar + y_bar - center
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn vane_center(col: usize, row: usize) -> (f64, f64) {
    (
        centered_index(col, VANE_COLS, VANE_PITCH_X),
        centered_index(row, VANE_ROWS, VANE_PITCH_Y),
    )
}

fn vane_index(col: usize, row: usize) -> usize {
    row * VANE_COLS + col
}

fn logger_x(kind: LoggerKind) -> f64 {
    centered_index(kind.index(), LOGGER_TYPES, LOGGER_PITCH_X)
}

fn logger_y(height: usize) -> f64 {
    centered_index(height, LOGGER_HEIGHTS, LOGGER_PITCH_Y)
}

fn disposition_gate_x(gate: DispositionGate) -> f64 {
    centered_index(gate.index(), GATE_LANES, 96.0)
}

fn filter_clamp_points() -> [(f64, f64, f64); FILTER_CLAMP_COUNT] {
    [
        (-(FILTER_MEDIA_X / 2.0 + 44.0), -52.0, 0.0),
        (-(FILTER_MEDIA_X / 2.0 + 44.0), 52.0, 0.0),
        (FILTER_MEDIA_X / 2.0 + 44.0, -52.0, 0.0),
        (FILTER_MEDIA_X / 2.0 + 44.0, 52.0, 0.0),
        (-72.0, FILTER_MEDIA_Y / 2.0 + 48.0, 90.0),
        (72.0, FILTER_MEDIA_Y / 2.0 + 48.0, 90.0),
    ]
}

fn mount_points() -> [(f64, f64); MOUNT_SLOT_COUNT] {
    [
        (-(STATION_X / 2.0 - 78.0), -(STATION_Y / 2.0 - 76.0)),
        (STATION_X / 2.0 - 78.0, -(STATION_Y / 2.0 - 76.0)),
        (-(STATION_X / 2.0 - 78.0), STATION_Y / 2.0 - 76.0),
        (STATION_X / 2.0 - 78.0, STATION_Y / 2.0 - 76.0),
        (
            CLEAN_FILTER_POS.0,
            CLEAN_FILTER_POS.1 - FILTER_PANEL_Y / 2.0 - 30.0,
        ),
        (
            LOADED_FILTER_POS.0,
            LOADED_FILTER_POS.1 - FILTER_PANEL_Y / 2.0 - 30.0,
        ),
        (
            PRESSURE_POS.0,
            PRESSURE_POS.1 - PRESSURE_PANEL_Y / 2.0 - 28.0,
        ),
        (AIRFLOW_POS.0 + AIRFLOW_GRID_X / 2.0 - 54.0, AIRFLOW_POS.1),
    ]
}

fn datum_points() -> [(f64, f64); DATUM_TARGET_COUNT] {
    [
        (-(STATION_X / 2.0 - 108.0), -(STATION_Y / 2.0 - 108.0)),
        (STATION_X / 2.0 - 108.0, -(STATION_Y / 2.0 - 108.0)),
        (-(STATION_X / 2.0 - 108.0), STATION_Y / 2.0 - 108.0),
        (STATION_X / 2.0 - 108.0, STATION_Y / 2.0 - 108.0),
    ]
}

fn module_rects() -> [Rect; 10] {
    [
        Rect {
            name: "clean_filter",
            center: CLEAN_FILTER_POS,
            x: FILTER_PANEL_X,
            y: FILTER_PANEL_Y,
        },
        Rect {
            name: "loaded_filter",
            center: LOADED_FILTER_POS,
            x: FILTER_PANEL_X,
            y: FILTER_PANEL_Y,
        },
        Rect {
            name: "pressure_witness",
            center: PRESSURE_POS,
            x: PRESSURE_PANEL_X,
            y: PRESSURE_PANEL_Y,
        },
        Rect {
            name: "airflow_grid",
            center: AIRFLOW_POS,
            x: AIRFLOW_GRID_X,
            y: AIRFLOW_GRID_Y,
        },
        Rect {
            name: "logger_pockets",
            center: LOGGER_POS,
            x: LOGGER_PANEL_X,
            y: LOGGER_PANEL_Y,
        },
        Rect {
            name: "alarm_token_rail",
            center: TOKEN_POS,
            x: TOKEN_RAIL_X,
            y: TOKEN_RAIL_Y,
        },
        Rect {
            name: "gasket_witnesses",
            center: GASKET_POS,
            x: GASKET_PANEL_X,
            y: GASKET_PANEL_Y,
        },
        Rect {
            name: "custody_panel",
            center: CUSTODY_POS,
            x: CUSTODY_PANEL_X,
            y: CUSTODY_PANEL_Y,
        },
        Rect {
            name: "disposition_gates",
            center: GATE_POS,
            x: GATE_PANEL_X,
            y: GATE_PANEL_Y,
        },
        Rect {
            name: "camera_bridge",
            center: EVIDENCE_POS,
            x: EVIDENCE_SPAN_X,
            y: EVIDENCE_BRIDGE_Y,
        },
    ]
}

fn total_gate_capacity() -> usize {
    DispositionGate::all()
        .into_iter()
        .map(DispositionGate::capacity)
        .sum()
}

fn pressure_tap_count() -> usize {
    PRESSURE_TAPS_PER_SIDE * 2
}

fn filter_loading_open_area_ratio() -> f64 {
    let total_face_area = FILTER_MEDIA_X * FILTER_MEDIA_Y;
    let pleat_shadow_area = FILTER_PLEAT_COUNT as f64 * 5.0 * (FILTER_MEDIA_Y - 18.0);
    let load_shadow_area = LOADED_OCCLUSION_RIBS as f64 * 9.0 * (FILTER_MEDIA_Y - 24.0);
    (total_face_area - pleat_shadow_area - load_shadow_area) / total_face_area
}

fn airflow_grid_witness_area_mm2() -> f64 {
    VANE_COUNT as f64 * VANE_SLOT_X * VANE_SLOT_Y
        + SMOKE_COUPON_COUNT as f64 * SMOKE_COUPON_X * SMOKE_COUPON_Y
}

fn assert_design_constraints() {
    assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
    let rects = module_rects();
    for rect in rects {
        assert!(rect.fits_inside_deck(), "{} exceeds deck rim", rect.name);
    }
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
    assert!(pressure_tap_count() >= 10);
    assert!(filter_loading_open_area_ratio() > 0.55);
    assert!(airflow_grid_witness_area_mm2() > 24_000.0);
    assert!(CAMERA_UNDERSIDE_CLEARANCE > FILTER_CLEARANCE_Z);
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 13);
        assert!(OUTPUTS.iter().all(|path| path.contains(OUTPUT_PREFIX)));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
        for feature in REQUIRED_FEATURES {
            if feature != "named_stl_outputs" {
                assert!(
                    OUTPUTS.iter().any(|path| path.contains(feature)),
                    "missing output for {feature}"
                );
            }
        }
    }

    #[test]
    fn clean_and_loaded_filter_envelopes_are_datumed() {
        assert_eq!(FILTER_CLAMP_COUNT, filter_clamp_points().len());
        assert_eq!(FILTER_PLEAT_COUNT, 9);
        assert_eq!(LOADED_OCCLUSION_RIBS, 8);
        assert!(FILTER_PANEL_X > FILTER_MEDIA_X + 120.0);
        assert!(FILTER_PANEL_Y > FILTER_MEDIA_Y + 80.0);
        assert!(filter_loading_open_area_ratio() > 0.55);
        assert!(LOADED_FILTER_POS.0 - CLEAN_FILTER_POS.0 > FILTER_PANEL_X);
    }

    #[test]
    fn pressure_airflow_and_logger_counts_cover_recovery_claim() {
        assert_eq!(pressure_tap_count(), 10);
        assert_eq!(DIFFERENTIAL_GAUGE_COUNT, 3);
        assert_eq!(VANE_COUNT, 20);
        assert_eq!(SMOKE_COUPON_COUNT, VANE_COUNT);
        assert_eq!(LOGGER_POCKET_COUNT, 9);
        assert!(airflow_grid_witness_area_mm2() > 24_000.0);
    }

    #[test]
    fn custody_gates_and_alarm_tokens_are_explicit() {
        assert_eq!(ALARM_TOKEN_COUNT, 6);
        assert_eq!(RECOVERY_TOKEN_COUNT, 6);
        assert_eq!(RECOVERY_TIME_MARKS, 7);
        assert_eq!(BARCODE_LANDS, 8);
        assert_eq!(COA_CARD_SLOTS, 4);
        assert_eq!(total_gate_capacity(), 12);
        assert!(RELEASE_CAPACITY > HOLD_CAPACITY);
        assert!(HOLD_CAPACITY > REJECT_CAPACITY);
    }

    #[test]
    fn gasket_leak_camera_and_keepout_geometry_has_margin() {
        assert_eq!(GASKET_WITNESS_STRIPS, 8);
        assert_eq!(LEAK_WITNESS_WELLS, 10);
        assert!(GASKET_COMPRESSION_MIN >= 3.0);
        assert!(GASKET_COMPRESSION_MAX <= 5.0);
        assert_eq!(CAMERA_COUNT, 5);
        assert_eq!(CAMERA_TARGET_COUNT, 6);
        assert_eq!(KEEP_OUT_GAUGES, 6);
        assert!(FRONT_ROBOT_CLEARANCE >= 320.0);
        assert!(TOP_CARTRIDGE_LIFT_CLEARANCE >= 300.0);
    }

    #[test]
    fn module_layout_fits_without_collisions() {
        let rects = module_rects();
        for rect in rects {
            assert!(rect.fits_inside_deck(), "{} exceeds deck rim", rect.name);
        }
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
    }
}
