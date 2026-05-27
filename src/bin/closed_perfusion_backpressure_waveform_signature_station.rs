use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed perfusion backpressure waveform signature validation station.
//
// Intent:
// - Package a closed-system, article-free validation fixture for comparing
//   perfusion backpressure waveform signatures across normal, restricted, and
//   occluded flow states.
// - Keep pressure transducer docks, restrictor/occlusion challenges, compliant
//   tubing surrogates, bubble/wetness witnesses, flow references, alarm tokens,
//   waste/retain custody, barcode/COA evidence, disposition gates, camera
//   coverage, and robot/service keepouts mechanically explicit.
// - Represent bought sensors, tubing, cartridges, valves, and optics as
//   deterministic placeholder geometry only.
//
// This is mechanical validation packaging only. It is not a pressure-rated
// wetted design, a process-release method, or an alarm acceptance protocol.

const PREFIX: &str = "closed_perfusion_backpressure_waveform_signature_station";

const OUTPUTS: [&str; 13] = [
    "output/closed_perfusion_backpressure_waveform_signature_station_containment_deck.stl",
    "output/closed_perfusion_backpressure_waveform_signature_station_pressure_transducer_dock_panel.stl",
    "output/closed_perfusion_backpressure_waveform_signature_station_restrictor_occlusion_challenge_cartridge_rack.stl",
    "output/closed_perfusion_backpressure_waveform_signature_station_compliant_tubing_surrogate_loop_bed.stl",
    "output/closed_perfusion_backpressure_waveform_signature_station_bubble_wetness_witness_window_bridge.stl",
    "output/closed_perfusion_backpressure_waveform_signature_station_flow_reference_pocket_lane_plate.stl",
    "output/closed_perfusion_backpressure_waveform_signature_station_alarm_threshold_token_rail.stl",
    "output/closed_perfusion_backpressure_waveform_signature_station_waste_retain_capture_bay.stl",
    "output/closed_perfusion_backpressure_waveform_signature_station_barcode_coa_custody_plate.stl",
    "output/closed_perfusion_backpressure_waveform_signature_station_release_hold_reject_gate_bank.stl",
    "output/closed_perfusion_backpressure_waveform_signature_station_camera_evidence_bridge.stl",
    "output/closed_perfusion_backpressure_waveform_signature_station_robot_service_keepout_frame.stl",
    "output/closed_perfusion_backpressure_waveform_signature_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 12] = [
    "pressure_transducer_docks",
    "restrictor_occlusion_challenge_cartridges",
    "compliant_tubing_surrogate_loops",
    "bubble_wetness_witness_windows",
    "flow_reference_pockets",
    "alarm_threshold_token_rail",
    "waste_retain_capture",
    "barcode_coa_custody",
    "release_hold_reject_gates",
    "camera_evidence_bridge",
    "robot_service_keepouts",
    "closed_system_route_identity",
];

const LIMITATIONS: [&str; 5] = [
    "mechanical_validation_packaging_only",
    "not_pressure_rated_wetted_design",
    "not_process_release_method",
    "not_alarm_acceptance_protocol",
    "purchased_sensors_tubing_cartridges_valves_and_optics_are_placeholders",
];

const FORBIDDEN_CLAIM_TERMS: [&str; 8] = [
    "patient",
    "therapy",
    "therapeutic",
    "diagnosis",
    "diagnostic",
    "clinical",
    "sterility assurance",
    "biological release",
];

const STATION_X: f64 = 1600.0;
const STATION_Y: f64 = 1060.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 48.0;
const BASIN_X: f64 = STATION_X - 128.0;
const BASIN_Y: f64 = STATION_Y - 128.0;
const BASIN_RECESS_Z: f64 = 7.0;
const DRAIN_D: f64 = 18.0;
const MOUNT_SLOTS: usize = 10;
const DATUM_TARGETS: usize = 6;
const DESIGN_CLEARANCE: f64 = 14.0;

const LANES: usize = 8;
const PRESSURE_DOCKS: usize = LANES * 2;
const PRESSURE_TAP_PAIRS: usize = LANES * 2;
const CHALLENGE_CARTRIDGES: usize = LANES;
const COMPLIANT_LOOPS: usize = LANES;
const BUBBLE_WINDOWS: usize = LANES;
const WETNESS_WINDOWS: usize = LANES;
const FLOW_REFERENCES: usize = LANES;
const WASTE_CAPTURE_NESTS: usize = 2;
const RETAIN_CAPTURE_NESTS: usize = 2;
const CAMERA_COUNT: usize = 4;
const TIMESTAMP_BEACONS: usize = 6;
const KEEP_OUT_GAUGES: usize = 6;

const PRESSURE_POS: (f64, f64) = (-515.0, -318.0);
const PRESSURE_X: f64 = 430.0;
const PRESSURE_Y: f64 = 154.0;
const PRESSURE_Z: f64 = 52.0;
const PRESSURE_COLS: usize = 8;
const PRESSURE_ROWS: usize = 2;
const PRESSURE_PITCH_X: f64 = 48.0;
const PRESSURE_PITCH_Y: f64 = 52.0;
const PRESSURE_POCKET_X: f64 = 32.0;
const PRESSURE_POCKET_Y: f64 = 26.0;
const ZERO_REFERENCE_PORTS: usize = 4;

const CHALLENGE_POS: (f64, f64) = (-515.0, 368.0);
const CHALLENGE_X: f64 = 430.0;
const CHALLENGE_Y: f64 = 196.0;
const CHALLENGE_Z: f64 = 56.0;
const CHALLENGE_PITCH_X: f64 = 48.0;
const RESTRICTOR_LEVELS_KPA: [usize; CHALLENGE_CARTRIDGES] = [0, 5, 10, 20, 35, 50, 70, 95];
const OCCLUSION_STATES: usize = 4;

const LOOP_POS: (f64, f64) = (-50.0, 112.0);
const LOOP_X: f64 = 900.0;
const LOOP_Y: f64 = 282.0;
const LOOP_Z: f64 = 20.0;
const LOOP_PITCH_Y: f64 = 30.0;
const LOOP_TRACE_Z: f64 = 7.0;
const LOOP_TRACE_W: f64 = 7.0;
const LOOP_RUN_X: f64 = 760.0;
const COMPLIANCE_CHAMBERS: usize = LANES * 2;

const WITNESS_POS: (f64, f64) = (590.0, 112.0);
const WITNESS_X: f64 = 330.0;
const WITNESS_Y: f64 = 292.0;
const WITNESS_Z: f64 = 46.0;
const BUBBLE_WINDOW_X: f64 = 86.0;
const BUBBLE_WINDOW_Y: f64 = 18.0;
const WETNESS_PAD_X: f64 = 64.0;
const WETNESS_PAD_Y: f64 = 18.0;
const WET_DRY_REFERENCE_TABS: usize = 4;

const FLOW_POS: (f64, f64) = (-70.0, -318.0);
const FLOW_X: f64 = 392.0;
const FLOW_Y: f64 = 154.0;
const FLOW_Z: f64 = 42.0;
const FLOW_POCKET_D: f64 = 28.0;
const FLOW_POCKET_COLS: usize = 4;
const FLOW_POCKET_ROWS: usize = 2;
const FLOW_POCKET_PITCH_X: f64 = 76.0;
const FLOW_POCKET_PITCH_Y: f64 = 52.0;

const ALARM_POS: (f64, f64) = (388.0, -318.0);
const ALARM_X: f64 = 384.0;
const ALARM_Y: f64 = 154.0;
const ALARM_Z: f64 = 32.0;
const ALARM_THRESHOLDS_KPA: [usize; 6] = [5, 15, 30, 45, 60, 85];
const ALARM_CHANNELS: usize = LANES;
const ALARM_TOKEN_D: f64 = 20.0;

const CAPTURE_POS: (f64, f64) = (540.0, 366.0);
const CAPTURE_X: f64 = 330.0;
const CAPTURE_Y: f64 = 170.0;
const CAPTURE_Z: f64 = 54.0;
const CAPTURE_NEST_X: f64 = 118.0;
const CAPTURE_NEST_Y: f64 = 50.0;
const CAPTURE_PORTS_PER_NEST: usize = 2;

const CUSTODY_POS: (f64, f64) = (-40.0, 440.0);
const CUSTODY_X: f64 = 470.0;
const CUSTODY_Y: f64 = 88.0;
const CUSTODY_Z: f64 = 18.0;
const BARCODE_LANDS: usize = LANES + 2;
const COA_CERTIFICATE_LANDS: usize = 4;
const TAMPER_SEAL_TABS: usize = 4;

const GATE_POS: (f64, f64) = (540.0, -124.0);
const GATE_X: f64 = 330.0;
const GATE_Y: f64 = 132.0;
const GATE_Z: f64 = 36.0;
const DISPOSITION_LANES: usize = 3;
const DISPOSITION_NAMES: [&str; DISPOSITION_LANES] = ["release", "hold", "reject"];
const TOKENS_PER_DISPOSITION: usize = LANES;

const CAMERA_POS: (f64, f64) = (0.0, 112.0);
const CAMERA_X: f64 = 1280.0;
const CAMERA_Y: f64 = 58.0;
const CAMERA_Z: f64 = 236.0;
const CAMERA_PITCH_X: f64 = 330.0;
const CAMERA_WINDOW_X: f64 = 116.0;
const CAMERA_WINDOW_Y: f64 = 28.0;

const KEEP_OUT_X: f64 = 1500.0;
const KEEP_OUT_Y: f64 = 960.0;
const KEEP_OUT_Z: f64 = 184.0;
const FRONT_ROBOT_CLEARANCE: f64 = 300.0;
const REAR_CARTRIDGE_SERVICE_CLEARANCE: f64 = 230.0;
const LEFT_TRANSDUCER_SERVICE_CLEARANCE: f64 = 190.0;
const RIGHT_CAPTURE_SERVICE_CLEARANCE: f64 = 180.0;
const CAMERA_LIFT_CLEARANCE: f64 = 270.0;
const LOOP_CARTRIDGE_LIFT_CLEARANCE: f64 = 220.0;

const LABEL_BAR_COUNT: usize = 8;

#[derive(Clone, Copy, Debug)]
struct Footprint {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Footprint {
    fn fits_inside_rim(self) -> bool {
        let usable_x = STATION_X / 2.0 - RIM_W - DESIGN_CLEARANCE;
        let usable_y = STATION_Y / 2.0 - RIM_W - DESIGN_CLEARANCE;
        self.center.0.abs() + self.x / 2.0 <= usable_x
            && self.center.1.abs() + self.y / 2.0 <= usable_y
    }

    fn overlaps_with_clearance(self, other: Footprint, clearance: f64) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();
        dx < (self.x + other.x) / 2.0 + clearance && dy < (self.y + other.y) / 2.0 + clearance
    }
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = containment_deck();
    export(&deck, OUTPUTS[0]);

    let pressure = pressure_transducer_dock_panel();
    export(&pressure, OUTPUTS[1]);

    let challenge = restrictor_occlusion_challenge_cartridge_rack();
    export(&challenge, OUTPUTS[2]);

    let loops = compliant_tubing_surrogate_loop_bed();
    export(&loops, OUTPUTS[3]);

    let witness = bubble_wetness_witness_window_bridge();
    export(&witness, OUTPUTS[4]);

    let flow = flow_reference_pocket_lane_plate();
    export(&flow, OUTPUTS[5]);

    let alarm = alarm_threshold_token_rail();
    export(&alarm, OUTPUTS[6]);

    let capture = waste_retain_capture_bay();
    export(&capture, OUTPUTS[7]);

    let custody = barcode_coa_custody_plate();
    export(&custody, OUTPUTS[8]);

    let gates = release_hold_reject_gate_bank();
    export(&gates, OUTPUTS[9]);

    let cameras = camera_evidence_bridge();
    export(&cameras, OUTPUTS[10]);

    let keepouts = robot_service_keepout_frame();
    export(&keepouts, OUTPUTS[11]);

    let assembly = deck
        + pressure
        + challenge
        + loops
        + witness
        + flow
        + alarm
        + capture
        + custody
        + gates
        + cameras
        + keepouts;
    export(&assembly, OUTPUTS[12]);

    println!(
        "Closed perfusion backpressure waveform signature station: {STATION_X:.0}mm x {STATION_Y:.0}mm containment deck, {LANES} closed surrogate lanes, {PRESSURE_DOCKS} pressure transducer docks, {CHALLENGE_CARTRIDGES} restrictor/occlusion cartridges, and {COMPLIANT_LOOPS} compliant loop surrogates."
    );
    println!(
        "Evidence and disposition coverage: {BUBBLE_WINDOWS} bubble windows, {WETNESS_WINDOWS} wetness windows, {FLOW_REFERENCES} flow reference pockets, {} alarm threshold tokens, {BARCODE_LANDS} barcode lands, {COA_CERTIFICATE_LANDS} COA lands, {CAMERA_COUNT} camera mounts, {TIMESTAMP_BEACONS} timestamp beacons, {} limitation markers, and {} STL outputs.",
        ALARM_THRESHOLDS_KPA.len() * ALARM_CHANNELS,
        LIMITATIONS.len(),
        OUTPUTS.len()
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_design_constraints() {
    assert_eq!(PRESSURE_DOCKS, LANES * 2);
    assert_eq!(PRESSURE_TAP_PAIRS, LANES * 2);
    assert_eq!(PRESSURE_ROWS * PRESSURE_COLS, PRESSURE_DOCKS);
    assert_eq!(CHALLENGE_CARTRIDGES, RESTRICTOR_LEVELS_KPA.len());
    assert_eq!(COMPLIANT_LOOPS, LANES);
    assert_eq!(COMPLIANCE_CHAMBERS, LANES * 2);
    assert_eq!(BUBBLE_WINDOWS, LANES);
    assert_eq!(WETNESS_WINDOWS, LANES);
    assert_eq!(FLOW_REFERENCES, LANES);
    assert_eq!(FLOW_POCKET_ROWS * FLOW_POCKET_COLS, FLOW_REFERENCES);
    assert_eq!(DISPOSITION_NAMES, ["release", "hold", "reject"]);
    assert_eq!(DISPOSITION_LANES, 3);
    assert_eq!(REQUIRED_FEATURES.len(), 12);
    assert_eq!(KEEP_OUT_GAUGES, 6);
    assert_eq!(DATUM_TARGETS, 6);
    assert!(LOOP_RUN_X < LOOP_X);
    assert!(BUBBLE_WINDOW_X < WITNESS_X);
    assert!(ALARM_THRESHOLDS_KPA
        .windows(2)
        .all(|pair| pair[0] < pair[1]));
    assert!(FRONT_ROBOT_CLEARANCE >= 300.0);
    assert!(REAR_CARTRIDGE_SERVICE_CLEARANCE >= 220.0);
    assert!(LEFT_TRANSDUCER_SERVICE_CLEARANCE >= 180.0);
    assert!(RIGHT_CAPTURE_SERVICE_CLEARANCE >= 175.0);
    assert!(CAMERA_LIFT_CLEARANCE > CAMERA_Z);
    assert!(LOOP_CARTRIDGE_LIFT_CLEARANCE > KEEP_OUT_Z);
    assert_no_scope_claim_terms();

    for footprint in module_footprints() {
        assert!(
            footprint.fits_inside_rim(),
            "{} exceeds rim",
            footprint.name
        );
    }

    for (i, a) in non_overlay_footprints().iter().enumerate() {
        for b in non_overlay_footprints().iter().skip(i + 1) {
            assert!(
                !a.overlaps_with_clearance(*b, 8.0),
                "{} overlaps {}",
                a.name,
                b.name
            );
        }
    }
}

fn assert_no_scope_claim_terms() {
    let searchable = format!(
        "{} {} {} {}",
        REQUIRED_FEATURES.join(" "),
        LIMITATIONS.join(" "),
        OUTPUTS.join(" "),
        PREFIX
    )
    .to_lowercase();
    for term in FORBIDDEN_CLAIM_TERMS {
        assert!(
            !searchable.contains(term),
            "claim term should not be present: {term}"
        );
    }
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        format!("{PREFIX}_containment_deck_base"),
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let basin = centered_cube(
        format!("{PREFIX}_sumped_closed_system_basin_recess"),
        BASIN_X,
        BASIN_Y,
        BASIN_RECESS_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0 - BASIN_RECESS_Z / 2.0 + 0.8);
    let drain = centered_cylinder(
        format!("{PREFIX}_front_right_low_point_drain_bore"),
        DRAIN_D / 2.0,
        66.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 78.0, -(STATION_Y / 2.0 - 20.0), 0.0);

    deck - basin - drain
        + containment_rims()
        + mounting_slot_bosses()
        + datum_targets()
        + module_socket_rails()
}

fn containment_rims() -> Part {
    let z = BASE_Z / 2.0 + RIM_Z / 2.0;
    let left = centered_cube(
        format!("{PREFIX}_left_containment_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-(STATION_X / 2.0 - RIM_W / 2.0), 0.0, z);
    let right = centered_cube(
        format!("{PREFIX}_right_containment_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, z);
    let rear = centered_cube(
        format!("{PREFIX}_rear_containment_rim"),
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, z);
    let front = centered_cube(
        format!("{PREFIX}_front_low_profile_containment_rim"),
        STATION_X,
        RIM_W,
        RIM_Z * 0.62,
    )
    .translate(
        0.0,
        -(STATION_Y / 2.0 - RIM_W / 2.0),
        BASE_Z / 2.0 + RIM_Z * 0.31,
    );
    left + right + rear + front
}

fn mounting_slot_bosses() -> Part {
    let mut slots = Part::empty(format!("{PREFIX}_mounting_slot_bosses"));
    for (i, (x, y)) in mount_points().iter().enumerate() {
        let boss = centered_cube(format!("{PREFIX}_m6_slot_boss_{i}"), 48.0, 24.0, 8.0).translate(
            *x,
            *y,
            BASE_Z / 2.0 + 4.0,
        );
        let bore = centered_cylinder(format!("{PREFIX}_m6_slot_bore_{i}"), 3.5, 16.0, 24)
            .translate(*x, *y, BASE_Z / 2.0 + 4.0);
        slots = slots + (boss - bore);
    }
    slots
}

fn datum_targets() -> Part {
    let mut datums = Part::empty(format!("{PREFIX}_station_datum_targets"));
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 74.0), -(STATION_Y / 2.0 - 66.0)),
        (STATION_X / 2.0 - 74.0, -(STATION_Y / 2.0 - 66.0)),
        (-(STATION_X / 2.0 - 74.0), STATION_Y / 2.0 - 66.0),
        (STATION_X / 2.0 - 74.0, STATION_Y / 2.0 - 66.0),
        (0.0, STATION_Y / 2.0 - 66.0),
        (0.0, -(STATION_Y / 2.0 - 66.0)),
    ]
    .iter()
    .enumerate()
    {
        datums = datums
            + fiducial_target(format!("{PREFIX}_datum_target_{i}")).translate(
                *x,
                *y,
                BASE_Z / 2.0 + 2.0,
            );
    }
    datums
}

fn module_socket_rails() -> Part {
    let mut rails = Part::empty(format!("{PREFIX}_module_socket_rails"));
    for (i, footprint) in non_overlay_footprints().iter().enumerate() {
        rails = rails
            + centered_cube(
                format!("{PREFIX}_module_{i}_front_socket_rail"),
                footprint.x + 22.0,
                5.0,
                8.0,
            )
            .translate(
                footprint.center.0,
                footprint.center.1 - footprint.y / 2.0 - 5.0,
                BASE_Z / 2.0 + 4.0,
            )
            + centered_cube(
                format!("{PREFIX}_module_{i}_rear_socket_rail"),
                footprint.x + 22.0,
                5.0,
                8.0,
            )
            .translate(
                footprint.center.0,
                footprint.center.1 + footprint.y / 2.0 + 5.0,
                BASE_Z / 2.0 + 4.0,
            );
    }
    rails
}

fn pressure_transducer_dock_panel() -> Part {
    let panel = centered_cube(
        format!("{PREFIX}_pressure_transducer_dock_panel_body"),
        PRESSURE_X,
        PRESSURE_Y,
        PRESSURE_Z,
    )
    .translate(
        PRESSURE_POS.0,
        PRESSURE_POS.1,
        BASE_Z / 2.0 + PRESSURE_Z / 2.0,
    );
    let cable_raceway = centered_cube(
        format!("{PREFIX}_pressure_rear_cable_raceway_cut"),
        PRESSURE_X - 52.0,
        15.0,
        PRESSURE_Z + 8.0,
    )
    .translate(
        PRESSURE_POS.0,
        PRESSURE_POS.1 + PRESSURE_Y / 2.0 - 20.0,
        BASE_Z / 2.0 + PRESSURE_Z / 2.0,
    );

    panel - cable_raceway - pressure_dock_pocket_cuts() - pressure_tap_bore_cuts()
        + pressure_dock_retainers()
        + pressure_zero_reference_ports()
        + waveform_signature_index_ticks()
}

fn pressure_dock_pocket_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_pressure_dock_pocket_cuts"));
    for dock in 0..PRESSURE_DOCKS {
        let (x, y) = pressure_dock_position(dock);
        cuts = cuts
            + centered_cube(
                format!("{PREFIX}_pressure_dock_{dock:02}_rectangular_pocket"),
                PRESSURE_POCKET_X,
                PRESSURE_POCKET_Y,
                PRESSURE_Z + 10.0,
            )
            .translate(x, y, BASE_Z / 2.0 + PRESSURE_Z / 2.0 - 5.0)
            + centered_cylinder(
                format!("{PREFIX}_pressure_dock_{dock:02}_diaphragm_relief_cut"),
                8.0,
                PRESSURE_Z + 12.0,
                28,
            )
            .translate(x, y, BASE_Z / 2.0 + PRESSURE_Z / 2.0 - 4.0);
    }
    cuts
}

fn pressure_tap_bore_cuts() -> Part {
    let mut bores = Part::empty(format!("{PREFIX}_pressure_tap_bore_cuts"));
    for dock in 0..PRESSURE_DOCKS {
        let (x, y) = pressure_dock_position(dock);
        bores = bores
            + centered_cylinder(
                format!("{PREFIX}_pressure_tap_bore_{dock:02}"),
                2.8,
                58.0,
                20,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, y - PRESSURE_POCKET_Y / 2.0 - 12.0, BASE_Z / 2.0 + 16.0);
    }
    bores
}

fn pressure_dock_retainers() -> Part {
    let mut retainers = Part::empty(format!("{PREFIX}_pressure_dock_retainers"));
    for dock in 0..PRESSURE_DOCKS {
        let (x, y) = pressure_dock_position(dock);
        retainers = retainers
            + centered_cube(
                format!("{PREFIX}_pressure_dock_{dock:02}_left_retainer"),
                4.0,
                PRESSURE_POCKET_Y + 12.0,
                7.0,
            )
            .translate(
                x - PRESSURE_POCKET_X / 2.0 - 4.0,
                y,
                BASE_Z / 2.0 + PRESSURE_Z + 3.5,
            )
            + centered_cube(
                format!("{PREFIX}_pressure_dock_{dock:02}_right_retainer"),
                4.0,
                PRESSURE_POCKET_Y + 12.0,
                7.0,
            )
            .translate(
                x + PRESSURE_POCKET_X / 2.0 + 4.0,
                y,
                BASE_Z / 2.0 + PRESSURE_Z + 3.5,
            );
    }
    retainers
}

fn pressure_zero_reference_ports() -> Part {
    let mut ports = Part::empty(format!("{PREFIX}_pressure_zero_reference_ports"));
    for port in 0..ZERO_REFERENCE_PORTS {
        ports = ports
            + port_ring(
                format!("{PREFIX}_zero_reference_port_{port}"),
                22.0,
                8.0,
                6.0,
            )
            .translate(
                PRESSURE_POS.0 + centered_index(port, ZERO_REFERENCE_PORTS, 54.0),
                PRESSURE_POS.1 - PRESSURE_Y / 2.0 + 18.0,
                BASE_Z / 2.0 + PRESSURE_Z + 3.0,
            );
    }
    ports
}

fn waveform_signature_index_ticks() -> Part {
    let mut ticks = Part::empty(format!("{PREFIX}_waveform_signature_index_ticks"));
    for lane in 0..LANES {
        let x = PRESSURE_POS.0 + centered_index(lane, LANES, 48.0);
        ticks = ticks
            + centered_cube(
                format!("{PREFIX}_lane_{lane:02}_upstream_waveform_tick"),
                32.0,
                4.0,
                5.0,
            )
            .translate(x, PRESSURE_POS.1 + 10.0, BASE_Z / 2.0 + PRESSURE_Z + 2.5)
            + centered_cube(
                format!("{PREFIX}_lane_{lane:02}_downstream_waveform_tick"),
                32.0,
                4.0,
                5.0,
            )
            .translate(x, PRESSURE_POS.1 + 28.0, BASE_Z / 2.0 + PRESSURE_Z + 2.5);
    }
    ticks
}

fn restrictor_occlusion_challenge_cartridge_rack() -> Part {
    let rack = centered_cube(
        format!("{PREFIX}_restrictor_occlusion_cartridge_rack_body"),
        CHALLENGE_X,
        CHALLENGE_Y,
        CHALLENGE_Z,
    )
    .translate(
        CHALLENGE_POS.0,
        CHALLENGE_POS.1,
        BASE_Z / 2.0 + CHALLENGE_Z / 2.0,
    );
    rack - challenge_cartridge_slot_cuts()
        + restrictor_orifice_witnesses()
        + occlusion_state_flags()
        + cartridge_latch_rails()
}

fn challenge_cartridge_slot_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_challenge_cartridge_slot_cuts"));
    for cartridge in 0..CHALLENGE_CARTRIDGES {
        cuts = cuts
            + centered_cube(
                format!("{PREFIX}_challenge_cartridge_{cartridge:02}_slot_cut"),
                30.0,
                CHALLENGE_Y - 56.0,
                CHALLENGE_Z + 8.0,
            )
            .translate(
                CHALLENGE_POS.0
                    + centered_index(cartridge, CHALLENGE_CARTRIDGES, CHALLENGE_PITCH_X),
                CHALLENGE_POS.1 - 6.0,
                BASE_Z / 2.0 + CHALLENGE_Z / 2.0 - 4.0,
            );
    }
    cuts
}

fn restrictor_orifice_witnesses() -> Part {
    let mut witnesses = Part::empty(format!("{PREFIX}_restrictor_orifice_witnesses"));
    for (cartridge, level) in RESTRICTOR_LEVELS_KPA.iter().enumerate() {
        let x =
            CHALLENGE_POS.0 + centered_index(cartridge, CHALLENGE_CARTRIDGES, CHALLENGE_PITCH_X);
        witnesses = witnesses
            + port_ring(
                format!("{PREFIX}_restrictor_{level}_kpa_orifice_witness"),
                25.0,
                5.0 + cartridge as f64 * 0.6,
                6.0,
            )
            .translate(
                x,
                CHALLENGE_POS.1 + CHALLENGE_Y / 2.0 - 30.0,
                BASE_Z / 2.0 + CHALLENGE_Z + 3.0,
            )
            + csg_label_plaque(
                format!("{PREFIX}_restrictor_{level}_kpa_label"),
                35.0,
                13.0,
                3.0,
                *level,
            )
            .translate(
                x,
                CHALLENGE_POS.1 - CHALLENGE_Y / 2.0 + 20.0,
                BASE_Z / 2.0 + CHALLENGE_Z + 1.5,
            );
    }
    witnesses
}

fn occlusion_state_flags() -> Part {
    let mut flags = Part::empty(format!("{PREFIX}_occlusion_state_flags"));
    for state in 0..OCCLUSION_STATES {
        flags = flags
            + csg_label_plaque(
                format!("{PREFIX}_occlusion_state_{state}_flag"),
                60.0,
                15.0,
                4.0,
                120 + state,
            )
            .translate(
                CHALLENGE_POS.0 - CHALLENGE_X / 2.0 + 42.0,
                CHALLENGE_POS.1 + centered_index(state, OCCLUSION_STATES, 34.0),
                BASE_Z / 2.0 + CHALLENGE_Z + 2.0,
            );
    }
    flags
}

fn cartridge_latch_rails() -> Part {
    let front = centered_cube(
        format!("{PREFIX}_challenge_front_latch_rail"),
        CHALLENGE_X - 54.0,
        6.0,
        8.0,
    )
    .translate(
        CHALLENGE_POS.0,
        CHALLENGE_POS.1 - CHALLENGE_Y / 2.0 + 42.0,
        BASE_Z / 2.0 + CHALLENGE_Z + 4.0,
    );
    let rear = centered_cube(
        format!("{PREFIX}_challenge_rear_latch_rail"),
        CHALLENGE_X - 54.0,
        6.0,
        8.0,
    )
    .translate(
        CHALLENGE_POS.0,
        CHALLENGE_POS.1 + CHALLENGE_Y / 2.0 - 42.0,
        BASE_Z / 2.0 + CHALLENGE_Z + 4.0,
    );
    front + rear
}

fn compliant_tubing_surrogate_loop_bed() -> Part {
    let bed = centered_cube(
        format!("{PREFIX}_compliant_tubing_loop_bed_plate"),
        LOOP_X,
        LOOP_Y,
        LOOP_Z,
    )
    .translate(LOOP_POS.0, LOOP_POS.1, BASE_Z / 2.0 + LOOP_Z / 2.0);
    let relief = centered_cube(
        format!("{PREFIX}_compliant_tubing_loop_bed_recess"),
        LOOP_X - 56.0,
        LOOP_Y - 42.0,
        5.0,
    )
    .translate(LOOP_POS.0, LOOP_POS.1, BASE_Z / 2.0 + LOOP_Z - 2.0);
    bed - relief + loop_trace_array() + compliance_chamber_witnesses() + lane_identity_tick_rail()
}

fn loop_trace_array() -> Part {
    let mut traces = Part::empty(format!("{PREFIX}_compliant_loop_trace_array"));
    let z = BASE_Z / 2.0 + LOOP_Z + LOOP_TRACE_Z / 2.0;
    for lane in 0..COMPLIANT_LOOPS {
        let y = loop_lane_y(lane);
        let x0 = LOOP_POS.0 - LOOP_RUN_X / 2.0;
        let x1 = LOOP_POS.0 + LOOP_RUN_X / 2.0;
        traces = traces
            + centered_cube(
                format!("{PREFIX}_lane_{lane:02}_upper_compliant_loop_trace"),
                LOOP_RUN_X,
                LOOP_TRACE_W,
                LOOP_TRACE_Z,
            )
            .translate(LOOP_POS.0, y + 8.0, z)
            + centered_cube(
                format!("{PREFIX}_lane_{lane:02}_lower_compliant_loop_return_trace"),
                LOOP_RUN_X - 76.0,
                LOOP_TRACE_W,
                LOOP_TRACE_Z,
            )
            .translate(LOOP_POS.0, y - 8.0, z)
            + port_ring(
                format!("{PREFIX}_lane_{lane:02}_left_compliance_loop_turn"),
                28.0,
                13.0,
                LOOP_TRACE_Z,
            )
            .translate(x0, y, z)
            + port_ring(
                format!("{PREFIX}_lane_{lane:02}_right_compliance_loop_turn"),
                28.0,
                13.0,
                LOOP_TRACE_Z,
            )
            .translate(x1, y, z)
            + port_ring(
                format!("{PREFIX}_lane_{lane:02}_upstream_pressure_tap_ring"),
                17.0,
                5.0,
                5.0,
            )
            .translate(LOOP_POS.0 - 260.0, y + 8.0, z + 5.0)
            + port_ring(
                format!("{PREFIX}_lane_{lane:02}_downstream_pressure_tap_ring"),
                17.0,
                5.0,
                5.0,
            )
            .translate(LOOP_POS.0 + 260.0, y + 8.0, z + 5.0);
    }
    traces
}

fn compliance_chamber_witnesses() -> Part {
    let mut chambers = Part::empty(format!("{PREFIX}_compliance_chamber_witnesses"));
    for lane in 0..COMPLIANT_LOOPS {
        let y = loop_lane_y(lane);
        for side in 0..2 {
            chambers = chambers
                + port_ring(
                    format!("{PREFIX}_lane_{lane:02}_compliance_chamber_{side}"),
                    30.0,
                    12.0,
                    9.0,
                )
                .translate(
                    LOOP_POS.0 - 114.0 + side as f64 * 228.0,
                    y - 8.0,
                    BASE_Z / 2.0 + LOOP_Z + LOOP_TRACE_Z + 4.5,
                );
        }
    }
    chambers
}

fn lane_identity_tick_rail() -> Part {
    let rail = centered_cube(
        format!("{PREFIX}_closed_route_identity_tick_rail"),
        LOOP_X - 90.0,
        10.0,
        6.0,
    )
    .translate(
        LOOP_POS.0,
        LOOP_POS.1 + LOOP_Y / 2.0 - 24.0,
        BASE_Z / 2.0 + LOOP_Z + 3.0,
    );
    let mut ticks = Part::empty(format!("{PREFIX}_closed_route_identity_ticks"));
    for lane in 0..LANES {
        ticks = ticks
            + centered_cube(
                format!("{PREFIX}_route_identity_tick_lane_{lane:02}"),
                18.0,
                16.0,
                7.0,
            )
            .translate(
                LOOP_POS.0 + centered_index(lane, LANES, 82.0),
                LOOP_POS.1 + LOOP_Y / 2.0 - 24.0,
                BASE_Z / 2.0 + LOOP_Z + 6.5,
            );
    }
    rail + ticks
}

fn bubble_wetness_witness_window_bridge() -> Part {
    let body = centered_cube(
        format!("{PREFIX}_bubble_wetness_witness_window_bridge_body"),
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    )
    .translate(WITNESS_POS.0, WITNESS_POS.1, BASE_Z / 2.0 + WITNESS_Z / 2.0);
    body - bubble_window_cuts() - wetness_window_cuts()
        + bubble_window_frames()
        + wetness_probe_lands()
        + wet_dry_reference_tabs()
}

fn bubble_window_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_bubble_window_cuts"));
    for lane in 0..BUBBLE_WINDOWS {
        cuts = cuts
            + centered_cube(
                format!("{PREFIX}_lane_{lane:02}_bubble_witness_window_cut"),
                BUBBLE_WINDOW_X,
                BUBBLE_WINDOW_Y,
                WITNESS_Z + 8.0,
            )
            .translate(
                WITNESS_POS.0 - 52.0,
                witness_lane_y(lane),
                BASE_Z / 2.0 + WITNESS_Z / 2.0,
            );
    }
    cuts
}

fn wetness_window_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_wetness_window_cuts"));
    for lane in 0..WETNESS_WINDOWS {
        cuts = cuts
            + centered_cube(
                format!("{PREFIX}_lane_{lane:02}_wetness_witness_window_cut"),
                WETNESS_PAD_X,
                WETNESS_PAD_Y,
                WITNESS_Z + 8.0,
            )
            .translate(
                WITNESS_POS.0 + 72.0,
                witness_lane_y(lane),
                BASE_Z / 2.0 + WITNESS_Z / 2.0,
            );
    }
    cuts
}

fn bubble_window_frames() -> Part {
    let mut frames = Part::empty(format!("{PREFIX}_bubble_window_frames"));
    for lane in 0..BUBBLE_WINDOWS {
        frames = frames
            + rectangular_frame_xy(
                format!("{PREFIX}_lane_{lane:02}_bubble_window_frame"),
                BUBBLE_WINDOW_X + 16.0,
                BUBBLE_WINDOW_Y + 12.0,
                5.0,
                6.0,
            )
            .translate(
                WITNESS_POS.0 - 52.0,
                witness_lane_y(lane),
                BASE_Z / 2.0 + WITNESS_Z + 3.0,
            );
    }
    frames
}

fn wetness_probe_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_wetness_probe_lands"));
    for lane in 0..WETNESS_WINDOWS {
        let y = witness_lane_y(lane);
        for probe in 0..2 {
            lands = lands
                + centered_cylinder(
                    format!("{PREFIX}_lane_{lane:02}_wetness_probe_{probe}"),
                    3.2,
                    8.0,
                    20,
                )
                .translate(
                    WITNESS_POS.0 + 120.0 + probe as f64 * 18.0,
                    y,
                    BASE_Z / 2.0 + WITNESS_Z + 4.0,
                );
        }
    }
    lands
}

fn wet_dry_reference_tabs() -> Part {
    let mut tabs = Part::empty(format!("{PREFIX}_wet_dry_reference_tabs"));
    for tab in 0..WET_DRY_REFERENCE_TABS {
        tabs = tabs
            + csg_label_plaque(
                format!("{PREFIX}_wet_dry_reference_tab_{tab}"),
                46.0,
                14.0,
                4.0,
                200 + tab,
            )
            .translate(
                WITNESS_POS.0 - WITNESS_X / 2.0 + 34.0,
                WITNESS_POS.1 + centered_index(tab, WET_DRY_REFERENCE_TABS, 42.0),
                BASE_Z / 2.0 + WITNESS_Z + 2.0,
            );
    }
    tabs
}

fn flow_reference_pocket_lane_plate() -> Part {
    let plate = centered_cube(
        format!("{PREFIX}_flow_reference_pocket_lane_plate_body"),
        FLOW_X,
        FLOW_Y,
        FLOW_Z,
    )
    .translate(FLOW_POS.0, FLOW_POS.1, BASE_Z / 2.0 + FLOW_Z / 2.0);
    plate - flow_reference_pocket_cuts() + flow_reference_ring_lands() + gravimetric_tray_ticks()
}

fn flow_reference_pocket_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_flow_reference_pocket_cuts"));
    for reference in 0..FLOW_REFERENCES {
        let (x, y) = flow_reference_position(reference);
        cuts = cuts
            + centered_cylinder(
                format!("{PREFIX}_lane_{reference:02}_flow_reference_pocket_cut"),
                FLOW_POCKET_D / 2.0,
                FLOW_Z + 8.0,
                32,
            )
            .translate(x, y, BASE_Z / 2.0 + FLOW_Z / 2.0 - 4.0);
    }
    cuts
}

fn flow_reference_ring_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_flow_reference_ring_lands"));
    for reference in 0..FLOW_REFERENCES {
        let (x, y) = flow_reference_position(reference);
        lands = lands
            + port_ring(
                format!("{PREFIX}_lane_{reference:02}_flow_reference_ring_land"),
                FLOW_POCKET_D + 13.0,
                FLOW_POCKET_D,
                6.0,
            )
            .translate(x, y, BASE_Z / 2.0 + FLOW_Z + 3.0);
    }
    lands
}

fn gravimetric_tray_ticks() -> Part {
    let mut ticks = Part::empty(format!("{PREFIX}_flow_reference_gravimetric_tray_ticks"));
    for lane in 0..LANES {
        ticks = ticks
            + centered_cube(
                format!("{PREFIX}_lane_{lane:02}_flow_reference_tick"),
                24.0,
                5.0,
                5.0,
            )
            .translate(
                FLOW_POS.0 + centered_index(lane, LANES, 40.0),
                FLOW_POS.1 + FLOW_Y / 2.0 - 18.0,
                BASE_Z / 2.0 + FLOW_Z + 2.5,
            );
    }
    ticks
}

fn alarm_threshold_token_rail() -> Part {
    let rail = centered_cube(
        format!("{PREFIX}_alarm_threshold_token_rail_body"),
        ALARM_X,
        ALARM_Y,
        ALARM_Z,
    )
    .translate(ALARM_POS.0, ALARM_POS.1, BASE_Z / 2.0 + ALARM_Z / 2.0);
    rail + alarm_threshold_tokens() + alarm_channel_lane_ticks() + threshold_label_lands()
}

fn alarm_threshold_tokens() -> Part {
    let mut tokens = Part::empty(format!("{PREFIX}_alarm_threshold_tokens"));
    for channel in 0..ALARM_CHANNELS {
        for (level, kpa) in ALARM_THRESHOLDS_KPA.iter().enumerate() {
            tokens = tokens
                + port_ring(
                    format!("{PREFIX}_lane_{channel:02}_threshold_{kpa}_kpa_token"),
                    ALARM_TOKEN_D,
                    ALARM_TOKEN_D - 8.0,
                    5.0,
                )
                .translate(
                    ALARM_POS.0 + centered_index(level, ALARM_THRESHOLDS_KPA.len(), 46.0),
                    ALARM_POS.1 + centered_index(channel, ALARM_CHANNELS, 15.0),
                    BASE_Z / 2.0 + ALARM_Z + 2.5,
                );
        }
    }
    tokens
}

fn alarm_channel_lane_ticks() -> Part {
    let mut ticks = Part::empty(format!("{PREFIX}_alarm_channel_lane_ticks"));
    for channel in 0..ALARM_CHANNELS {
        ticks = ticks
            + centered_cube(
                format!("{PREFIX}_alarm_channel_{channel:02}_route_tick"),
                ALARM_X - 40.0,
                3.0,
                3.5,
            )
            .translate(
                ALARM_POS.0,
                ALARM_POS.1 + centered_index(channel, ALARM_CHANNELS, 15.0),
                BASE_Z / 2.0 + ALARM_Z + 6.0,
            );
    }
    ticks
}

fn threshold_label_lands() -> Part {
    let mut labels = Part::empty(format!("{PREFIX}_threshold_label_lands"));
    for (level, kpa) in ALARM_THRESHOLDS_KPA.iter().enumerate() {
        labels = labels
            + csg_label_plaque(
                format!("{PREFIX}_threshold_{kpa}_kpa_label_land"),
                38.0,
                13.0,
                3.0,
                *kpa,
            )
            .translate(
                ALARM_POS.0 + centered_index(level, ALARM_THRESHOLDS_KPA.len(), 46.0),
                ALARM_POS.1 - ALARM_Y / 2.0 + 14.0,
                BASE_Z / 2.0 + ALARM_Z + 1.5,
            );
    }
    labels
}

fn waste_retain_capture_bay() -> Part {
    let bay = centered_cube(
        format!("{PREFIX}_waste_retain_capture_bay_body"),
        CAPTURE_X,
        CAPTURE_Y,
        CAPTURE_Z,
    )
    .translate(CAPTURE_POS.0, CAPTURE_POS.1, BASE_Z / 2.0 + CAPTURE_Z / 2.0);
    bay - capture_nest_cuts() + capture_nest_rims() + diverter_route_witness_ticks()
}

fn capture_nest_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_capture_nest_cuts"));
    for nest in 0..(WASTE_CAPTURE_NESTS + RETAIN_CAPTURE_NESTS) {
        let (x, y) = capture_nest_position(nest);
        cuts = cuts
            + centered_cube(
                format!("{PREFIX}_capture_nest_{nest}_pocket_cut"),
                CAPTURE_NEST_X,
                CAPTURE_NEST_Y,
                CAPTURE_Z + 8.0,
            )
            .translate(x, y, BASE_Z / 2.0 + CAPTURE_Z / 2.0 - 4.0);
    }
    cuts
}

fn capture_nest_rims() -> Part {
    let mut rims = Part::empty(format!("{PREFIX}_capture_nest_rims"));
    for nest in 0..(WASTE_CAPTURE_NESTS + RETAIN_CAPTURE_NESTS) {
        let (x, y) = capture_nest_position(nest);
        rims = rims
            + rectangular_frame_xy(
                format!("{PREFIX}_capture_nest_{nest}_retainer_rim"),
                CAPTURE_NEST_X + 18.0,
                CAPTURE_NEST_Y + 14.0,
                5.0,
                7.0,
            )
            .translate(x, y, BASE_Z / 2.0 + CAPTURE_Z + 3.5);
        for port in 0..CAPTURE_PORTS_PER_NEST {
            rims = rims
                + port_ring(
                    format!("{PREFIX}_capture_nest_{nest}_port_{port}"),
                    18.0,
                    7.0,
                    5.0,
                )
                .translate(
                    x + centered_index(port, CAPTURE_PORTS_PER_NEST, 32.0),
                    y + CAPTURE_NEST_Y / 2.0 + 18.0,
                    BASE_Z / 2.0 + CAPTURE_Z + 2.5,
                );
        }
    }
    rims
}

fn diverter_route_witness_ticks() -> Part {
    let mut ticks = Part::empty(format!("{PREFIX}_waste_retain_diverter_route_ticks"));
    for lane in 0..LANES {
        ticks = ticks
            + centered_cube(
                format!("{PREFIX}_lane_{lane:02}_waste_retain_route_tick"),
                8.0,
                42.0,
                5.0,
            )
            .rotate(0.0, 0.0, if lane % 2 == 0 { 18.0 } else { -18.0 })
            .translate(
                CAPTURE_POS.0 + centered_index(lane, LANES, 30.0),
                CAPTURE_POS.1,
                BASE_Z / 2.0 + CAPTURE_Z + 2.5,
            );
    }
    ticks
}

fn barcode_coa_custody_plate() -> Part {
    let plate = centered_cube(
        format!("{PREFIX}_barcode_coa_custody_plate_body"),
        CUSTODY_X,
        CUSTODY_Y,
        CUSTODY_Z,
    )
    .translate(CUSTODY_POS.0, CUSTODY_POS.1, BASE_Z / 2.0 + CUSTODY_Z / 2.0);
    plate + barcode_lands() + coa_certificate_lands() + tamper_seal_tabs()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_barcode_lands"));
    for land in 0..BARCODE_LANDS {
        lands = lands
            + csg_label_plaque(
                format!("{PREFIX}_barcode_land_{land:02}"),
                32.0,
                20.0,
                4.0,
                300 + land,
            )
            .translate(
                CUSTODY_POS.0 + centered_index(land, BARCODE_LANDS, 40.0),
                CUSTODY_POS.1 - 22.0,
                BASE_Z / 2.0 + CUSTODY_Z + 2.0,
            );
    }
    lands
}

fn coa_certificate_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_coa_certificate_lands"));
    for land in 0..COA_CERTIFICATE_LANDS {
        lands = lands
            + csg_label_plaque(
                format!("{PREFIX}_coa_certificate_land_{land}"),
                72.0,
                18.0,
                4.0,
                400 + land,
            )
            .translate(
                CUSTODY_POS.0 + centered_index(land, COA_CERTIFICATE_LANDS, 98.0),
                CUSTODY_POS.1 + 22.0,
                BASE_Z / 2.0 + CUSTODY_Z + 2.0,
            );
    }
    lands
}

fn tamper_seal_tabs() -> Part {
    let mut tabs = Part::empty(format!("{PREFIX}_tamper_seal_tabs"));
    for tab in 0..TAMPER_SEAL_TABS {
        tabs = tabs
            + centered_cube(format!("{PREFIX}_tamper_seal_tab_{tab}"), 34.0, 8.0, 6.0).translate(
                CUSTODY_POS.0 + centered_index(tab, TAMPER_SEAL_TABS, 122.0),
                CUSTODY_POS.1 + CUSTODY_Y / 2.0 - 10.0,
                BASE_Z / 2.0 + CUSTODY_Z + 3.0,
            );
    }
    tabs
}

fn release_hold_reject_gate_bank() -> Part {
    let bank = centered_cube(
        format!("{PREFIX}_release_hold_reject_gate_bank_body"),
        GATE_X,
        GATE_Y,
        GATE_Z,
    )
    .translate(GATE_POS.0, GATE_POS.1, BASE_Z / 2.0 + GATE_Z / 2.0);
    bank + disposition_gate_lanes() + disposition_gate_labels()
}

fn disposition_gate_lanes() -> Part {
    let mut lanes = Part::empty(format!("{PREFIX}_disposition_gate_lanes"));
    for lane in 0..DISPOSITION_LANES {
        let y = GATE_POS.1 + centered_index(lane, DISPOSITION_LANES, 36.0);
        lanes = lanes
            + centered_cube(
                format!("{PREFIX}_disposition_lane_{lane}_gate_fence"),
                GATE_X - 50.0,
                5.0,
                9.0,
            )
            .translate(GATE_POS.0, y, BASE_Z / 2.0 + GATE_Z + 4.5);
        for token in 0..TOKENS_PER_DISPOSITION {
            lanes = lanes
                + centered_cube(
                    format!("{PREFIX}_disposition_lane_{lane}_token_slot_{token:02}"),
                    16.0,
                    20.0,
                    5.0,
                )
                .translate(
                    GATE_POS.0 + centered_index(token, TOKENS_PER_DISPOSITION, 30.0),
                    y,
                    BASE_Z / 2.0 + GATE_Z + 9.0,
                );
        }
    }
    lanes
}

fn disposition_gate_labels() -> Part {
    let mut labels = Part::empty(format!("{PREFIX}_disposition_gate_labels"));
    for (lane, name) in DISPOSITION_NAMES.iter().enumerate() {
        labels = labels
            + csg_label_plaque(
                format!("{PREFIX}_{name}_gate_label"),
                62.0,
                15.0,
                4.0,
                500 + lane,
            )
            .translate(
                GATE_POS.0 - GATE_X / 2.0 + 44.0,
                GATE_POS.1 + centered_index(lane, DISPOSITION_LANES, 36.0),
                BASE_Z / 2.0 + GATE_Z + 8.0,
            );
    }
    labels
}

fn camera_evidence_bridge() -> Part {
    let left_post = centered_cube(
        format!("{PREFIX}_camera_bridge_left_post"),
        36.0,
        CAMERA_Y,
        CAMERA_Z,
    )
    .translate(
        CAMERA_POS.0 - CAMERA_X / 2.0 + 42.0,
        CAMERA_POS.1,
        BASE_Z + CAMERA_Z / 2.0,
    );
    let right_post = centered_cube(
        format!("{PREFIX}_camera_bridge_right_post"),
        36.0,
        CAMERA_Y,
        CAMERA_Z,
    )
    .translate(
        CAMERA_POS.0 + CAMERA_X / 2.0 - 42.0,
        CAMERA_POS.1,
        BASE_Z + CAMERA_Z / 2.0,
    );
    let beam = centered_cube(
        format!("{PREFIX}_camera_bridge_overhead_beam"),
        CAMERA_X,
        34.0,
        30.0,
    )
    .translate(CAMERA_POS.0, CAMERA_POS.1, BASE_Z + CAMERA_Z - 15.0);
    left_post + right_post + beam + camera_mounts_and_windows() + timestamp_beacon_lands()
}

fn camera_mounts_and_windows() -> Part {
    let mut mounts = Part::empty(format!("{PREFIX}_camera_mounts_and_windows"));
    for camera in 0..CAMERA_COUNT {
        let x = CAMERA_POS.0 + centered_index(camera, CAMERA_COUNT, CAMERA_PITCH_X);
        let mount = centered_cube(
            format!("{PREFIX}_camera_{camera}_mount_land"),
            86.0,
            34.0,
            10.0,
        )
        .translate(x, CAMERA_POS.1, BASE_Z + CAMERA_Z - 40.0);
        let window = centered_cube(
            format!("{PREFIX}_camera_{camera}_evidence_window_cut"),
            CAMERA_WINDOW_X,
            CAMERA_WINDOW_Y,
            14.0,
        )
        .translate(x, CAMERA_POS.1, BASE_Z + CAMERA_Z - 15.0);
        mounts = mounts + (mount - window);
    }
    mounts
}

fn timestamp_beacon_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_timestamp_beacon_lands"));
    for beacon in 0..TIMESTAMP_BEACONS {
        lands = lands
            + port_ring(
                format!("{PREFIX}_timestamp_beacon_{beacon}"),
                22.0,
                8.0,
                5.0,
            )
            .translate(
                CAMERA_POS.0 + centered_index(beacon, TIMESTAMP_BEACONS, 154.0),
                CAMERA_POS.1 - CAMERA_Y / 2.0 - 18.0,
                BASE_Z + CAMERA_Z - 56.0,
            );
    }
    lands
}

fn robot_service_keepout_frame() -> Part {
    let frame = keepout_frame(
        format!("{PREFIX}_robot_service_keepout_outer_frame"),
        KEEP_OUT_X,
        KEEP_OUT_Y,
        KEEP_OUT_Z,
    )
    .translate(0.0, 0.0, BASE_Z + KEEP_OUT_Z / 2.0);
    frame + keepout_corner_posts() + service_keepout_gauges() + vertical_clearance_posts()
}

fn keepout_corner_posts() -> Part {
    let mut posts = Part::empty(format!("{PREFIX}_robot_service_keepout_corner_posts"));
    for (i, (x, y)) in [
        (-KEEP_OUT_X / 2.0, -KEEP_OUT_Y / 2.0),
        (KEEP_OUT_X / 2.0, -KEEP_OUT_Y / 2.0),
        (-KEEP_OUT_X / 2.0, KEEP_OUT_Y / 2.0),
        (KEEP_OUT_X / 2.0, KEEP_OUT_Y / 2.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(
                format!("{PREFIX}_keepout_corner_post_{i}"),
                12.0,
                12.0,
                KEEP_OUT_Z,
            )
            .translate(*x, *y, BASE_Z + KEEP_OUT_Z / 2.0);
    }
    posts
}

fn service_keepout_gauges() -> Part {
    let z = BASE_Z + 12.0;
    let front = gauge_bar(
        format!("{PREFIX}_front_robot_clearance_gauge"),
        410.0,
        18.0,
        12.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 64.0, z);
    let rear = gauge_bar(
        format!("{PREFIX}_rear_cartridge_service_clearance_gauge"),
        380.0,
        18.0,
        12.0,
    )
    .translate(CHALLENGE_POS.0, STATION_Y / 2.0 - 64.0, z);
    let left = gauge_bar(
        format!("{PREFIX}_left_transducer_service_clearance_gauge"),
        18.0,
        250.0,
        12.0,
    )
    .translate(-STATION_X / 2.0 + 64.0, PRESSURE_POS.1, z);
    let right = gauge_bar(
        format!("{PREFIX}_right_capture_service_clearance_gauge"),
        18.0,
        250.0,
        12.0,
    )
    .translate(STATION_X / 2.0 - 64.0, CAPTURE_POS.1, z);
    let camera = gauge_bar(
        format!("{PREFIX}_camera_lift_clearance_gauge"),
        260.0,
        16.0,
        12.0,
    )
    .translate(CAMERA_POS.0, CAMERA_POS.1 + CAMERA_Y / 2.0 + 36.0, z);
    let loop_lift = gauge_bar(
        format!("{PREFIX}_loop_cartridge_lift_clearance_gauge"),
        260.0,
        16.0,
        12.0,
    )
    .translate(LOOP_POS.0, LOOP_POS.1 - LOOP_Y / 2.0 - 30.0, z);
    front + rear + left + right + camera + loop_lift
}

fn vertical_clearance_posts() -> Part {
    let camera_post = centered_cube(
        format!("{PREFIX}_camera_lift_clearance_post"),
        24.0,
        24.0,
        CAMERA_LIFT_CLEARANCE,
    )
    .translate(
        KEEP_OUT_X / 2.0 - 58.0,
        -(KEEP_OUT_Y / 2.0 - 58.0),
        BASE_Z + CAMERA_LIFT_CLEARANCE / 2.0,
    );
    let loop_post = centered_cube(
        format!("{PREFIX}_loop_cartridge_lift_clearance_post"),
        24.0,
        24.0,
        LOOP_CARTRIDGE_LIFT_CLEARANCE,
    )
    .translate(
        KEEP_OUT_X / 2.0 - 98.0,
        -(KEEP_OUT_Y / 2.0 - 58.0),
        BASE_Z + LOOP_CARTRIDGE_LIFT_CLEARANCE / 2.0,
    );
    camera_post + loop_post
}

fn pressure_dock_position(dock: usize) -> (f64, f64) {
    let row = dock / PRESSURE_COLS;
    let col = dock % PRESSURE_COLS;
    (
        PRESSURE_POS.0 + centered_index(col, PRESSURE_COLS, PRESSURE_PITCH_X),
        PRESSURE_POS.1 + centered_index(row, PRESSURE_ROWS, PRESSURE_PITCH_Y),
    )
}

fn flow_reference_position(reference: usize) -> (f64, f64) {
    let row = reference / FLOW_POCKET_COLS;
    let col = reference % FLOW_POCKET_COLS;
    (
        FLOW_POS.0 + centered_index(col, FLOW_POCKET_COLS, FLOW_POCKET_PITCH_X),
        FLOW_POS.1 + centered_index(row, FLOW_POCKET_ROWS, FLOW_POCKET_PITCH_Y),
    )
}

fn capture_nest_position(nest: usize) -> (f64, f64) {
    let row = nest / 2;
    let col = nest % 2;
    (
        CAPTURE_POS.0 + centered_index(col, 2, 150.0),
        CAPTURE_POS.1 + centered_index(row, 2, 76.0),
    )
}

fn loop_lane_y(lane: usize) -> f64 {
    LOOP_POS.1 + centered_index(lane, LANES, LOOP_PITCH_Y)
}

fn witness_lane_y(lane: usize) -> f64 {
    WITNESS_POS.1 + centered_index(lane, LANES, LOOP_PITCH_Y)
}

fn module_footprints() -> [Footprint; 11] {
    [
        Footprint {
            name: "pressure_transducer_dock_panel",
            center: PRESSURE_POS,
            x: PRESSURE_X,
            y: PRESSURE_Y,
        },
        Footprint {
            name: "restrictor_occlusion_challenge_cartridge_rack",
            center: CHALLENGE_POS,
            x: CHALLENGE_X,
            y: CHALLENGE_Y,
        },
        Footprint {
            name: "compliant_tubing_surrogate_loop_bed",
            center: LOOP_POS,
            x: LOOP_X,
            y: LOOP_Y,
        },
        Footprint {
            name: "bubble_wetness_witness_window_bridge",
            center: WITNESS_POS,
            x: WITNESS_X,
            y: WITNESS_Y,
        },
        Footprint {
            name: "flow_reference_pocket_lane_plate",
            center: FLOW_POS,
            x: FLOW_X,
            y: FLOW_Y,
        },
        Footprint {
            name: "alarm_threshold_token_rail",
            center: ALARM_POS,
            x: ALARM_X,
            y: ALARM_Y,
        },
        Footprint {
            name: "waste_retain_capture_bay",
            center: CAPTURE_POS,
            x: CAPTURE_X,
            y: CAPTURE_Y,
        },
        Footprint {
            name: "barcode_coa_custody_plate",
            center: CUSTODY_POS,
            x: CUSTODY_X,
            y: CUSTODY_Y,
        },
        Footprint {
            name: "release_hold_reject_gate_bank",
            center: GATE_POS,
            x: GATE_X,
            y: GATE_Y,
        },
        Footprint {
            name: "camera_evidence_bridge",
            center: CAMERA_POS,
            x: CAMERA_X,
            y: CAMERA_Y,
        },
        Footprint {
            name: "robot_service_keepout_frame",
            center: (0.0, 0.0),
            x: KEEP_OUT_X,
            y: KEEP_OUT_Y,
        },
    ]
}

fn non_overlay_footprints() -> [Footprint; 9] {
    let specs = module_footprints();
    [
        specs[0], specs[1], specs[2], specs[3], specs[4], specs[5], specs[6], specs[7], specs[8],
    ]
}

fn mount_points() -> [(f64, f64); MOUNT_SLOTS] {
    [
        (-(STATION_X / 2.0 - 60.0), -(STATION_Y / 2.0 - 58.0)),
        (STATION_X / 2.0 - 60.0, -(STATION_Y / 2.0 - 58.0)),
        (-(STATION_X / 2.0 - 60.0), STATION_Y / 2.0 - 58.0),
        (STATION_X / 2.0 - 60.0, STATION_Y / 2.0 - 58.0),
        (0.0, -(STATION_Y / 2.0 - 58.0)),
        (0.0, STATION_Y / 2.0 - 58.0),
        (-(STATION_X / 2.0 - 60.0), 0.0),
        (STATION_X / 2.0 - 60.0, 0.0),
        (-STATION_X / 4.0, STATION_Y / 2.0 - 58.0),
        (STATION_X / 4.0, STATION_Y / 2.0 - 58.0),
    ]
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn rectangular_frame_xy(
    name: impl Into<String>,
    outer_x: f64,
    outer_y: f64,
    rail: f64,
    z: f64,
) -> Part {
    let name = name.into();
    centered_cube(format!("{name}_outer"), outer_x, outer_y, z)
        - centered_cube(
            format!("{name}_inner_clearance"),
            outer_x - 2.0 * rail,
            outer_y - 2.0 * rail,
            z + 1.0,
        )
}

fn keepout_frame(name: impl Into<String>, x: f64, y: f64, z: f64) -> Part {
    let name = name.into();
    let front =
        centered_cube(format!("{name}_front_rail"), x, 8.0, z).translate(0.0, -y / 2.0, 0.0);
    let rear = centered_cube(format!("{name}_rear_rail"), x, 8.0, z).translate(0.0, y / 2.0, 0.0);
    let left = centered_cube(format!("{name}_left_rail"), 8.0, y, z).translate(-x / 2.0, 0.0, 0.0);
    let right = centered_cube(format!("{name}_right_rail"), 8.0, y, z).translate(x / 2.0, 0.0, 0.0);
    front + rear + left + right
}

fn port_ring(name: impl Into<String>, outer_d: f64, inner_d: f64, z: f64) -> Part {
    let name = name.into();
    centered_cylinder(format!("{name}_outer"), outer_d / 2.0, z, 36)
        - centered_cylinder(format!("{name}_inner"), inner_d / 2.0, z + 1.0, 28)
}

fn fiducial_target(name: impl Into<String>) -> Part {
    let name = name.into();
    let ring = port_ring(format!("{name}_ring"), 22.0, 8.0, 4.0);
    let crosshair = centered_cube(format!("{name}_crosshair_x"), 26.0, 2.0, 3.0)
        + centered_cube(format!("{name}_crosshair_y"), 2.0, 26.0, 3.0);
    ring + crosshair
}

fn gauge_bar(name: impl Into<String>, x: f64, y: f64, z: f64) -> Part {
    let name = name.into();
    let bar = centered_cube(format!("{name}_bar"), x, y, z);
    let tick_a = centered_cube(format!("{name}_tick_a"), 14.0, 14.0, z + 6.0).translate(
        -x / 2.0,
        -y / 2.0,
        0.0,
    );
    let tick_b = centered_cube(format!("{name}_tick_b"), 14.0, 14.0, z + 6.0).translate(
        x / 2.0,
        y / 2.0,
        0.0,
    );
    bar + tick_a + tick_b
}

fn csg_label_plaque(name: impl Into<String>, x: f64, y: f64, z: f64, seed: usize) -> Part {
    let name = name.into();
    let sheet = centered_cube(format!("{name}_sheet"), x, y, z);
    let mut bars = Part::empty(format!("{name}_raised_barcode_bars"));
    for i in 0..LABEL_BAR_COUNT {
        let width = 1.8 + ((seed + i) % 4) as f64 * 1.1;
        let height = (y - 5.0 - (i % 3) as f64).max(3.0);
        let x_offset = -x / 2.0 + 7.0 + i as f64 * ((x - 16.0) / LABEL_BAR_COUNT as f64);
        bars = bars
            + centered_cube(format!("{name}_bar_{i}"), width, height, z + 1.2).translate(
                x_offset,
                0.0,
                z / 2.0 + 0.6,
            );
    }
    let corner = centered_cube(format!("{name}_orientation_corner"), 7.0, 3.0, z + 1.4).translate(
        x / 2.0 - 6.0,
        y / 2.0 - 4.0,
        z / 2.0 + 0.7,
    );
    sheet + bars + corner
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_manifest_is_stable_unique_and_scoped() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 13);
        for output in OUTPUTS {
            assert!(output.starts_with(&format!("output/{PREFIX}_")), "{output}");
            assert!(output.ends_with(".stl"), "{output}");
        }
        assert!(OUTPUTS[12].ends_with("_assembly.stl"));
    }

    #[test]
    fn requested_station_scope_is_explicit() {
        for feature in [
            "pressure_transducer_docks",
            "restrictor_occlusion_challenge_cartridges",
            "compliant_tubing_surrogate_loops",
            "bubble_wetness_witness_windows",
            "flow_reference_pockets",
            "alarm_threshold_token_rail",
            "waste_retain_capture",
            "barcode_coa_custody",
            "release_hold_reject_gates",
            "camera_evidence_bridge",
            "robot_service_keepouts",
            "closed_system_route_identity",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
        assert_eq!(LIMITATIONS.len(), 5);
    }

    #[test]
    fn lane_counts_and_pressure_pairs_are_pinned() {
        assert_eq!(LANES, 8);
        assert_eq!(PRESSURE_DOCKS, LANES * 2);
        assert_eq!(PRESSURE_TAP_PAIRS, LANES * 2);
        assert_eq!(PRESSURE_ROWS * PRESSURE_COLS, PRESSURE_DOCKS);
        assert_eq!(COMPLIANT_LOOPS, LANES);
        assert_eq!(BUBBLE_WINDOWS, LANES);
        assert_eq!(WETNESS_WINDOWS, LANES);
        assert_eq!(FLOW_REFERENCES, LANES);
        assert_eq!(FLOW_POCKET_ROWS * FLOW_POCKET_COLS, LANES);
        assert_eq!(loop_lane_y(0) + loop_lane_y(LANES - 1), 2.0 * LOOP_POS.1);
        for lane in 1..LANES {
            assert_eq!(loop_lane_y(lane) - loop_lane_y(lane - 1), LOOP_PITCH_Y);
        }
    }

    #[test]
    fn challenge_alarm_custody_and_disposition_capacity_are_pinned() {
        assert_eq!(CHALLENGE_CARTRIDGES, LANES);
        assert_eq!(RESTRICTOR_LEVELS_KPA, [0, 5, 10, 20, 35, 50, 70, 95]);
        assert_eq!(ALARM_THRESHOLDS_KPA, [5, 15, 30, 45, 60, 85]);
        assert_eq!(ALARM_CHANNELS, LANES);
        assert_eq!(BARCODE_LANDS, LANES + 2);
        assert_eq!(COA_CERTIFICATE_LANDS, 4);
        assert_eq!(WASTE_CAPTURE_NESTS, 2);
        assert_eq!(RETAIN_CAPTURE_NESTS, 2);
        assert_eq!(DISPOSITION_NAMES, ["release", "hold", "reject"]);
        assert_eq!(TOKENS_PER_DISPOSITION, LANES);
    }

    #[test]
    fn footprints_fit_and_keepouts_are_declared() {
        assert_design_constraints();
        for footprint in module_footprints() {
            assert!(
                footprint.fits_inside_rim(),
                "{} footprint should remain inside deck rim",
                footprint.name
            );
        }
        assert!(KEEP_OUT_X < STATION_X);
        assert!(KEEP_OUT_Y < STATION_Y);
        assert_eq!(KEEP_OUT_GAUGES, 6);
        assert!(FRONT_ROBOT_CLEARANCE >= 300.0);
        assert!(REAR_CARTRIDGE_SERVICE_CLEARANCE >= 220.0);
        assert!(LEFT_TRANSDUCER_SERVICE_CLEARANCE >= 180.0);
        assert!(RIGHT_CAPTURE_SERVICE_CLEARANCE >= 175.0);
        assert!(CAMERA_LIFT_CLEARANCE > CAMERA_Z);
    }

    #[test]
    fn evidence_bridge_and_witness_counts_are_pinned() {
        assert_eq!(CAMERA_COUNT, 4);
        assert_eq!(TIMESTAMP_BEACONS, 6);
        assert_eq!(WET_DRY_REFERENCE_TABS, 4);
        assert_eq!(PRESSURE_TAP_PAIRS, 16);
        assert_eq!(COMPLIANCE_CHAMBERS, 16);
        assert_eq!(DATUM_TARGETS, 6);
        assert_eq!(MOUNT_SLOTS, 10);
    }

    #[test]
    fn no_biological_or_clinical_claim_terms_are_present() {
        assert_no_scope_claim_terms();
    }
}
