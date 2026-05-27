use std::fs;

use laminarforge_cad::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
use vcad::{centered_cube, centered_cylinder, Part};

// Closed microfluidic lane occlusion/recovery pressure-signature validation station.
//
// Intent:
// - Challenge a no-tissue 4x4 lane-map chip surrogate inside the clean isolator
//   before any live tissue chip is connected.
// - Present calibrated partial-occlusion cartridges, debris and gel-plug witness
//   windows, upstream/downstream pressure taps, relief/bypass witnesses, recovery
//   pulse limit gauges, quarantine waste, and evidence disposition geometry.
// - Encode the mechanical validation envelope only. Pressure limits, release
//   criteria, wetted materials, sensors, valves, and biological acceptance
//   protocols remain separate controlled documents.

const OUTPUTS: [&str; 12] = [
    "output/closed_microfluidic_lane_occlusion_recovery_pressure_signature_station_containment_deck.stl",
    "output/closed_microfluidic_lane_occlusion_recovery_pressure_signature_station_chip_surrogate_lane_rack.stl",
    "output/closed_microfluidic_lane_occlusion_recovery_pressure_signature_station_calibrated_occlusion_cartridge_bank.stl",
    "output/closed_microfluidic_lane_occlusion_recovery_pressure_signature_station_upstream_downstream_pressure_port_panel.stl",
    "output/closed_microfluidic_lane_occlusion_recovery_pressure_signature_station_bypass_relief_path_witness.stl",
    "output/closed_microfluidic_lane_occlusion_recovery_pressure_signature_station_bubble_debris_capture_windows.stl",
    "output/closed_microfluidic_lane_occlusion_recovery_pressure_signature_station_recovery_pulse_limiter_gauges.stl",
    "output/closed_microfluidic_lane_occlusion_recovery_pressure_signature_station_waste_quarantine_outlet_cassette.stl",
    "output/closed_microfluidic_lane_occlusion_recovery_pressure_signature_station_camera_fiducial_bridge.stl",
    "output/closed_microfluidic_lane_occlusion_recovery_pressure_signature_station_release_hold_reject_evidence_gate.stl",
    "output/closed_microfluidic_lane_occlusion_recovery_pressure_signature_station_robot_service_keepout_gauges.stl",
    "output/closed_microfluidic_lane_occlusion_recovery_pressure_signature_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 12] = [
    "clean_isolator_containment_deck",
    "4x4_chip_surrogate_lane_rack",
    "removable_calibrated_occlusion_cartridges",
    "paired_upstream_downstream_pressure_ports",
    "bypass_relief_path_witness",
    "bubble_debris_gel_capture_windows",
    "recovery_pulse_limiter_gauges",
    "waste_quarantine_outlet",
    "camera_fiducials",
    "release_hold_reject_evidence_gate",
    "lane_pressure_signature_traceability",
    "robot_service_keepout_gauges",
];

const LIMITATIONS: [&str; 6] = [
    "source_only_architecture_cad",
    "not_pressure_rated",
    "not_a_wetted_material_specification",
    "not_a_biological_release_protocol",
    "surrogate_occlusions_require_calibration",
    "sensors_and_valves_are_placeholders",
];

const STATION_X: f64 = 1680.0;
const STATION_Y: f64 = 1020.0;
const DECK_Z: f64 = 24.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 48.0;
const BASIN_DEPTH: f64 = 8.0;
const DRAIN_D: f64 = 18.0;
const MOUNT_HOLE_D: f64 = 7.0;
const DATUM_RING_D: f64 = 22.0;
const DESIGN_CLEARANCE: f64 = 16.0;

const LANE_ROWS: usize = 4;
const LANE_COLS: usize = 4;
const LANE_COUNT: usize = LANE_ROWS * LANE_COLS;
const LANE_PITCH_X: f64 = 120.0;
const LANE_PITCH_Y: f64 = 78.0;
const LANE_TRACE_X: f64 = 92.0;
const LANE_TRACE_Y: f64 = 8.0;
const LANE_TRACE_Z: f64 = 7.0;
const LANE_PORT_OFFSET_X: f64 = 50.0;
const LANE_PORT_D: f64 = 8.0;
const LANE_PORT_RING_D: f64 = 20.0;
const LANE_ID_TOKEN_D: f64 = 9.0;

const RACK_CENTER: (f64, f64) = (-470.0, 150.0);
const RACK_X: f64 = 560.0;
const RACK_Y: f64 = 390.0;
const RACK_Z: f64 = 38.0;
const RACK_POCKET_CLEARANCE: f64 = 1.1;
const RACK_CLAMPS_PER_LANE: usize = 2;
const RACK_DATUM_PIN_COUNT: usize = 4;

const OCCLUSION_CENTER: (f64, f64) = (125.0, 235.0);
const OCCLUSION_BANK_X: f64 = 520.0;
const OCCLUSION_BANK_Y: f64 = 280.0;
const OCCLUSION_BANK_Z: f64 = 54.0;
const OCCLUSION_CARTRIDGES: usize = LANE_COUNT;
const OCCLUSION_GRADES: usize = 4;
const OCCLUSION_SOCKET_X: f64 = 62.0;
const OCCLUSION_SOCKET_Y: f64 = 34.0;
const OCCLUSION_SOCKET_DEPTH: f64 = 18.0;
const OCCLUSION_HANDLE_X: f64 = 46.0;
const OCCLUSION_HANDLE_Y: f64 = 20.0;
const OCCLUSION_HANDLE_Z: f64 = 16.0;
const OCCLUSION_BORE_D: f64 = 4.4;
const OCCLUSION_PARTIAL_OPEN_FRACTIONS: [f64; OCCLUSION_GRADES] = [1.00, 0.72, 0.45, 0.22];

const PRESSURE_CENTER: (f64, f64) = (500.0, -60.0);
const PRESSURE_PANEL_X: f64 = 530.0;
const PRESSURE_PANEL_Y: f64 = 210.0;
const PRESSURE_PANEL_Z: f64 = 46.0;
const PRESSURE_SENSOR_PORTS: usize = LANE_COUNT * 2;
const PRESSURE_PORT_PAIR_SPACING_X: f64 = 34.0;
const PRESSURE_PORT_D: f64 = 8.0;
const PRESSURE_PORT_RING_D: f64 = 21.0;
const PRESSURE_CABLE_TROUGHS: usize = LANE_ROWS;
const PRESSURE_ZERO_REFERENCE_PORTS: usize = LANE_COLS;
const MAX_STEADY_STATE_DELTA_KPA: f64 = 8.0;
const OCCLUSION_ALERT_DELTA_KPA: f64 = 18.0;

const BYPASS_CENTER: (f64, f64) = (-55.0, -180.0);
const BYPASS_X: f64 = 510.0;
const BYPASS_Y: f64 = 190.0;
const BYPASS_Z: f64 = 42.0;
const BYPASS_BRANCHES: usize = LANE_COUNT;
const RELIEF_WITNESS_WINDOWS: usize = LANE_COUNT;
const RELIEF_CARTRIDGE_COUNT: usize = LANE_COUNT;
const RELIEF_SETPOINT_KPA: f64 = 30.0;
const RELIEF_REJECT_KPA: f64 = 42.0;
const BYPASS_HEADER_W: f64 = 11.0;
const BYPASS_BRANCH_W: f64 = 5.8;

const CAPTURE_CENTER: (f64, f64) = (-535.0, -350.0);
const CAPTURE_X: f64 = 430.0;
const CAPTURE_Y: f64 = 150.0;
const CAPTURE_Z: f64 = 52.0;
const BUBBLE_DEBRIS_WINDOWS: usize = LANE_COUNT;
const CAPTURE_WINDOW_X: f64 = 32.0;
const CAPTURE_WINDOW_Y: f64 = 20.0;
const CAPTURE_WELL_D: f64 = 20.0;
const DEBRIS_SCREEN_RIBS: usize = LANE_COUNT * 3;
const GEL_PLUG_REFERENCE_COUPONS: usize = LANE_COLS;

const PULSE_CENTER: (f64, f64) = (625.0, 235.0);
const PULSE_X: f64 = 280.0;
const PULSE_Y: f64 = 280.0;
const PULSE_Z: f64 = 58.0;
const PULSE_LIMITER_GAUGES: usize = LANE_COUNT;
const PULSE_GAUGE_D: f64 = 27.0;
const PULSE_CHAMBER_D: f64 = 22.0;
const MAX_RECOVERY_PULSE_KPA: f64 = 28.0;
const UNSAFE_RECOVERY_PULSE_KPA: f64 = 40.0;
const PULSE_HARD_STOP_COUNT: usize = LANE_COUNT;
const GLOBAL_PULSE_REFERENCE_GAUGES: usize = 2;

const WASTE_CENTER: (f64, f64) = (575.0, -355.0);
const WASTE_X: f64 = 390.0;
const WASTE_Y: f64 = 150.0;
const WASTE_Z: f64 = 64.0;
const WASTE_INLETS: usize = LANE_COUNT;
const WASTE_QUARANTINE_OUTLETS: usize = 1;
const WASTE_SAMPLE_WELLS: usize = 4;
const WASTE_CAPTURE_VOLUME_ML: f64 = 480.0;
const WASTE_SUMP_X: f64 = 290.0;
const WASTE_SUMP_Y: f64 = 94.0;
const WASTE_OUTLET_D: f64 = 13.0;

const CAMERA_CENTER: (f64, f64) = (0.0, 435.0);
const CAMERA_BRIDGE_X: f64 = 1540.0;
const CAMERA_BRIDGE_Y: f64 = 68.0;
const CAMERA_CLEARANCE_Z: f64 = 210.0;
const CAMERA_BEAM_Z: f64 = 24.0;
const CAMERA_HEADS: usize = 4;
const CAMERA_HEAD_X: f64 = 74.0;
const CAMERA_HEAD_Y: f64 = 46.0;
const CAMERA_HEAD_Z: f64 = 38.0;
const LANE_CAMERA_FIDUCIALS: usize = LANE_COUNT;
const GLOBAL_CAMERA_FIDUCIALS: usize = 4;
const CAMERA_FIDUCIALS: usize = LANE_CAMERA_FIDUCIALS + GLOBAL_CAMERA_FIDUCIALS;
const FIDUCIAL_ARM: f64 = 18.0;

const DISPOSITION_CENTER: (f64, f64) = (-95.0, -370.0);
const DISPOSITION_X: f64 = 320.0;
const DISPOSITION_Y: f64 = 120.0;
const DISPOSITION_Z: f64 = 34.0;
const DISPOSITION_LANES: usize = 3;
const STATUS_SLOT_COUNT: usize = LANE_COUNT;
const STATUS_SLOT_X: f64 = 26.0;
const STATUS_SLOT_Y: f64 = 16.0;
const EVIDENCE_CARD_LANDS: usize = LANE_COUNT;
const HOLD_REJECT_WALL_Z: f64 = 72.0;

const KEEP_OUT_GAUGES: usize = 6;
const ROBOT_SWEEP_CLEARANCE_Y: f64 = 330.0;
const CARTRIDGE_LIFT_CLEARANCE_Z: f64 = 180.0;
const PRESSURE_CABLE_SERVICE_CLEARANCE_X: f64 = 190.0;
const WASTE_BAG_REMOVAL_CLEARANCE_Y: f64 = 180.0;
const CAMERA_LIFT_CLEARANCE_Z: f64 = 260.0;
const FRONT_EVIDENCE_SERVICE_CLEARANCE_Y: f64 = 160.0;
const KEEP_OUT_Z: f64 = 10.0;

#[derive(Clone, Copy, Debug)]
struct Rect {
    name: &'static str,
    center: (f64, f64),
    x: f64,
    y: f64,
}

impl Rect {
    fn fits_inside_station(self) -> bool {
        let usable_x = STATION_X / 2.0 - RIM_W - DESIGN_CLEARANCE;
        let usable_y = STATION_Y / 2.0 - RIM_W - DESIGN_CLEARANCE;
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

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = containment_deck();
    export(&deck, OUTPUTS[0]);

    let rack = chip_surrogate_lane_rack();
    export(&rack, OUTPUTS[1]);

    let occlusions = calibrated_occlusion_cartridge_bank();
    export(&occlusions, OUTPUTS[2]);

    let pressure = upstream_downstream_pressure_port_panel();
    export(&pressure, OUTPUTS[3]);

    let bypass = bypass_relief_path_witness();
    export(&bypass, OUTPUTS[4]);

    let capture = bubble_debris_capture_windows();
    export(&capture, OUTPUTS[5]);

    let pulses = recovery_pulse_limiter_gauges();
    export(&pulses, OUTPUTS[6]);

    let waste = waste_quarantine_outlet_cassette();
    export(&waste, OUTPUTS[7]);

    let camera = camera_fiducial_bridge();
    export(&camera, OUTPUTS[8]);

    let evidence = release_hold_reject_evidence_gate();
    export(&evidence, OUTPUTS[9]);

    let keepouts = robot_service_keepout_gauges();
    export(&keepouts, OUTPUTS[10]);

    let assembly = deck
        + rack
        + occlusions
        + pressure
        + bypass
        + capture
        + pulses
        + waste
        + camera
        + evidence
        + keepouts;
    export(&assembly, OUTPUTS[11]);

    println!(
        "Closed microfluidic lane occlusion/recovery pressure-signature station: {STATION_X:.0}mm x {STATION_Y:.0}mm clean-isolator containment deck, {LANE_ROWS}x{LANE_COLS} lane map, {LANE_COUNT} no-tissue surrogate lanes, and {OCCLUSION_CARTRIDGES} removable calibrated occlusion cartridges."
    );
    println!(
        "Pressure validation coverage: {PRESSURE_SENSOR_PORTS} paired pressure sensor ports, {RELIEF_CARTRIDGE_COUNT} relief witnesses at {RELIEF_SETPOINT_KPA:.0}kPa nominal challenge, {PULSE_LIMITER_GAUGES} recovery pulse limiter gauges capped below {MAX_RECOVERY_PULSE_KPA:.0}kPa, and {BUBBLE_DEBRIS_WINDOWS} bubble/debris capture windows."
    );
    println!(
        "Evidence and custody: {WASTE_INLETS} quarantine waste inlets, {WASTE_QUARANTINE_OUTLETS} sealed waste outlet, {CAMERA_FIDUCIALS} camera fiducials, {STATUS_SLOT_COUNT} lane evidence slots, {} required feature groups, {} limitations, and {} STL outputs.",
        REQUIRED_FEATURES.len(),
        LIMITATIONS.len(),
        OUTPUTS.len()
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn containment_deck() -> Part {
    let deck = centered_cube(
        "closed_lane_occlusion_recovery_containment_deck",
        STATION_X,
        STATION_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let basin = centered_cube(
        "closed_lane_occlusion_recovery_sumped_basin_recess",
        STATION_X - RIM_W * 2.0 - 70.0,
        STATION_Y - RIM_W * 2.0 - 64.0,
        BASIN_DEPTH,
    )
    .translate(0.0, 0.0, DECK_Z - BASIN_DEPTH / 2.0 + 0.2);
    let drain = centered_cylinder(
        "closed_lane_occlusion_recovery_front_quarantine_drain_bore",
        DRAIN_D / 2.0,
        82.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 92.0,
        -STATION_Y / 2.0 + 20.0,
        DECK_Z - 5.0,
    );

    deck - basin - drain - deck_mount_holes()
        + deck_rims()
        + isolator_interface_gauges()
        + deck_module_datum_rings()
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("closed_lane_occlusion_recovery_deck_mount_holes");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 70.0, -STATION_Y / 2.0 + 62.0),
        (STATION_X / 2.0 - 70.0, -STATION_Y / 2.0 + 62.0),
        (-STATION_X / 2.0 + 70.0, STATION_Y / 2.0 - 62.0),
        (STATION_X / 2.0 - 70.0, STATION_Y / 2.0 - 62.0),
        (-240.0, 0.0),
        (240.0, 0.0),
        (0.0, -330.0),
        (0.0, 330.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("closed_lane_occlusion_recovery_deck_mount_hole_{i}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 8.0,
                28,
            )
            .translate(*x, *y, DECK_Z / 2.0);
    }
    holes
}

fn deck_rims() -> Part {
    let left = centered_cube(
        "closed_lane_occlusion_recovery_left_spill_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "closed_lane_occlusion_recovery_right_spill_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, DECK_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        "closed_lane_occlusion_recovery_rear_isolator_spill_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, DECK_Z + RIM_Z / 2.0);
    let front = centered_cube(
        "closed_lane_occlusion_recovery_front_low_quarantine_lip",
        STATION_X - 160.0,
        14.0,
        26.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 7.0, DECK_Z + 13.0);
    left + right + rear + front
}

fn isolator_interface_gauges() -> Part {
    let rear_bulkhead = centered_cube(
        "closed_lane_occlusion_recovery_rear_isolator_bulkhead_shadow_gauge",
        STATION_X - 180.0,
        12.0,
        34.0,
    )
    .translate(0.0, STATION_Y / 2.0 - 60.0, DECK_Z + 17.0);
    let left_door_sweep = centered_cube(
        "closed_lane_occlusion_recovery_left_clean_door_sweep_clearance_gauge",
        12.0,
        STATION_Y - 210.0,
        28.0,
    )
    .translate(-STATION_X / 2.0 + 62.0, 0.0, DECK_Z + 14.0);
    let right_service = centered_cube(
        "closed_lane_occlusion_recovery_right_service_shadow_clearance_gauge",
        12.0,
        STATION_Y - 210.0,
        28.0,
    )
    .translate(STATION_X / 2.0 - 62.0, 0.0, DECK_Z + 14.0);
    rear_bulkhead + left_door_sweep + right_service
}

fn deck_module_datum_rings() -> Part {
    let mut rings = Part::empty("closed_lane_occlusion_recovery_module_datum_rings");
    for (i, center) in [
        RACK_CENTER,
        OCCLUSION_CENTER,
        PRESSURE_CENTER,
        BYPASS_CENTER,
        CAPTURE_CENTER,
        PULSE_CENTER,
        WASTE_CENTER,
        DISPOSITION_CENTER,
    ]
    .iter()
    .enumerate()
    {
        let ring = centered_cylinder(
            format!("closed_lane_occlusion_recovery_module_datum_ring_{i}"),
            DATUM_RING_D / 2.0,
            4.0,
            32,
        )
        .translate(center.0, center.1, DECK_Z + 2.0)
            - centered_cylinder(
                format!("closed_lane_occlusion_recovery_module_datum_dot_{i}"),
                4.0,
                6.0,
                24,
            )
            .translate(center.0, center.1, DECK_Z + 2.0);
        rings = rings + ring;
    }
    rings
}

fn chip_surrogate_lane_rack() -> Part {
    let body = centered_cube(
        "closed_lane_occlusion_recovery_4x4_chip_surrogate_lane_rack_body",
        RACK_X,
        RACK_Y,
        RACK_Z,
    )
    .translate(RACK_CENTER.0, RACK_CENTER.1, DECK_Z + RACK_Z / 2.0);
    let pocket = centered_cube(
        "closed_lane_occlusion_recovery_revc_scaled_chip_footprint_recess",
        REVC_CHIP_LENGTH + RACK_POCKET_CLEARANCE * 2.0,
        REVC_CHIP_WIDTH + RACK_POCKET_CLEARANCE * 2.0,
        7.0,
    )
    .translate(RACK_CENTER.0, RACK_CENTER.1, DECK_Z + RACK_Z - 3.3);

    body - pocket - lane_trace_cuts()
        + lane_port_rings()
        + lane_clamp_fingers()
        + lane_id_tokens()
        + rack_datum_pins()
        + rack_gasket_land()
}

fn lane_trace_cuts() -> Part {
    let mut cuts = Part::empty("closed_lane_occlusion_recovery_lane_trace_cuts");
    for lane in 0..LANE_COUNT {
        let (x, y) = lane_abs_xy(lane, RACK_CENTER);
        let trace = centered_cube(
            format!("closed_lane_occlusion_recovery_lane_{lane}_flow_trace_cut"),
            LANE_TRACE_X,
            LANE_TRACE_Y,
            LANE_TRACE_Z,
        )
        .translate(x, y, DECK_Z + RACK_Z - LANE_TRACE_Z / 2.0 + 1.0);
        let inlet = centered_cylinder(
            format!("closed_lane_occlusion_recovery_lane_{lane}_inlet_port_cut"),
            LANE_PORT_D / 2.0,
            RACK_Z + 8.0,
            24,
        )
        .translate(x - LANE_PORT_OFFSET_X, y, DECK_Z + RACK_Z / 2.0);
        let outlet = centered_cylinder(
            format!("closed_lane_occlusion_recovery_lane_{lane}_outlet_port_cut"),
            LANE_PORT_D / 2.0,
            RACK_Z + 8.0,
            24,
        )
        .translate(x + LANE_PORT_OFFSET_X, y, DECK_Z + RACK_Z / 2.0);
        cuts = cuts + trace + inlet + outlet;
    }
    cuts
}

fn lane_port_rings() -> Part {
    let mut rings = Part::empty("closed_lane_occlusion_recovery_lane_port_gasket_rings");
    for lane in 0..LANE_COUNT {
        let (x, y) = lane_abs_xy(lane, RACK_CENTER);
        for (side, x_offset) in [
            ("upstream", -LANE_PORT_OFFSET_X),
            ("downstream", LANE_PORT_OFFSET_X),
        ] {
            let ring = centered_cylinder(
                format!("closed_lane_occlusion_recovery_lane_{lane}_{side}_port_ring_outer"),
                LANE_PORT_RING_D / 2.0,
                5.0,
                32,
            )
            .translate(x + x_offset, y, DECK_Z + RACK_Z + 2.5)
                - centered_cylinder(
                    format!("closed_lane_occlusion_recovery_lane_{lane}_{side}_port_ring_inner"),
                    LANE_PORT_D / 2.0,
                    7.0,
                    24,
                )
                .translate(x + x_offset, y, DECK_Z + RACK_Z + 2.5);
            rings = rings + ring;
        }
    }
    rings
}

fn lane_clamp_fingers() -> Part {
    let mut clamps = Part::empty("closed_lane_occlusion_recovery_lane_clamp_fingers");
    for lane in 0..LANE_COUNT {
        let (x, y) = lane_abs_xy(lane, RACK_CENTER);
        for (j, y_offset) in [-18.0, 18.0].iter().enumerate() {
            clamps = clamps
                + centered_cube(
                    format!("closed_lane_occlusion_recovery_lane_{lane}_chip_edge_clamp_{j}"),
                    118.0,
                    9.0,
                    13.0,
                )
                .translate(x, y + y_offset, DECK_Z + RACK_Z + 6.5);
        }
    }
    clamps
}

fn lane_id_tokens() -> Part {
    let mut tokens = Part::empty("closed_lane_occlusion_recovery_lane_id_tokens");
    for lane in 0..LANE_COUNT {
        let (x, y) = lane_abs_xy(lane, RACK_CENTER);
        tokens = tokens
            + centered_cylinder(
                format!("closed_lane_occlusion_recovery_lane_{lane}_map_token"),
                LANE_ID_TOKEN_D / 2.0,
                5.0,
                24,
            )
            .translate(x, y - 30.0, DECK_Z + RACK_Z + 2.5);
    }
    tokens
}

fn rack_datum_pins() -> Part {
    let mut pins = Part::empty("closed_lane_occlusion_recovery_rack_datum_pins");
    for (i, (x_offset, y_offset)) in [
        (-RACK_X / 2.0 + 30.0, -RACK_Y / 2.0 + 28.0),
        (RACK_X / 2.0 - 30.0, -RACK_Y / 2.0 + 28.0),
        (-RACK_X / 2.0 + 30.0, RACK_Y / 2.0 - 28.0),
        (RACK_X / 2.0 - 30.0, RACK_Y / 2.0 - 28.0),
    ]
    .iter()
    .enumerate()
    {
        pins = pins
            + centered_cylinder(
                format!("closed_lane_occlusion_recovery_rack_corner_datum_pin_{i}"),
                5.0,
                16.0,
                24,
            )
            .translate(
                RACK_CENTER.0 + x_offset,
                RACK_CENTER.1 + y_offset,
                DECK_Z + RACK_Z + 8.0,
            );
    }
    pins
}

fn rack_gasket_land() -> Part {
    rectangular_frame(
        "closed_lane_occlusion_recovery_revc_chip_surrogate_gasket_land",
        REVC_CHIP_LENGTH + 38.0,
        REVC_CHIP_WIDTH + 38.0,
        8.0,
        5.0,
    )
    .translate(RACK_CENTER.0, RACK_CENTER.1, DECK_Z + RACK_Z + 2.5)
}

fn calibrated_occlusion_cartridge_bank() -> Part {
    let body = centered_cube(
        "closed_lane_occlusion_recovery_calibrated_occlusion_cartridge_bank_body",
        OCCLUSION_BANK_X,
        OCCLUSION_BANK_Y,
        OCCLUSION_BANK_Z,
    )
    .translate(
        OCCLUSION_CENTER.0,
        OCCLUSION_CENTER.1,
        DECK_Z + OCCLUSION_BANK_Z / 2.0,
    );

    body - occlusion_socket_cuts()
        + occlusion_cartridge_handles()
        + occlusion_grade_markers()
        + occlusion_pull_tab_fences()
}

fn occlusion_socket_cuts() -> Part {
    let mut cuts = Part::empty("closed_lane_occlusion_recovery_occlusion_socket_cuts");
    for lane in 0..OCCLUSION_CARTRIDGES {
        let (x, y) = lane_abs_xy(lane, OCCLUSION_CENTER);
        let socket = centered_cube(
            format!("closed_lane_occlusion_recovery_lane_{lane}_removable_occlusion_socket"),
            OCCLUSION_SOCKET_X,
            OCCLUSION_SOCKET_Y,
            OCCLUSION_SOCKET_DEPTH,
        )
        .translate(
            x,
            y,
            DECK_Z + OCCLUSION_BANK_Z - OCCLUSION_SOCKET_DEPTH / 2.0 + 1.0,
        );
        let bore = centered_cylinder(
            format!("closed_lane_occlusion_recovery_lane_{lane}_occlusion_cartridge_flow_bore"),
            OCCLUSION_BORE_D / 2.0,
            OCCLUSION_SOCKET_Y + 12.0,
            20,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, y, DECK_Z + 26.0);
        cuts = cuts + socket + bore;
    }
    cuts
}

fn occlusion_cartridge_handles() -> Part {
    let mut handles = Part::empty("closed_lane_occlusion_recovery_occlusion_cartridge_handles");
    for lane in 0..OCCLUSION_CARTRIDGES {
        let (x, y) = lane_abs_xy(lane, OCCLUSION_CENTER);
        let handle = centered_cube(
            format!("closed_lane_occlusion_recovery_lane_{lane}_cartridge_pull_handle"),
            OCCLUSION_HANDLE_X,
            OCCLUSION_HANDLE_Y,
            OCCLUSION_HANDLE_Z,
        )
        .translate(x, y, DECK_Z + OCCLUSION_BANK_Z + OCCLUSION_HANDLE_Z / 2.0)
            + centered_cylinder(
                format!("closed_lane_occlusion_recovery_lane_{lane}_cartridge_thumb_loop"),
                8.0,
                6.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                x + OCCLUSION_HANDLE_X / 2.0 - 8.0,
                y,
                DECK_Z + OCCLUSION_BANK_Z + OCCLUSION_HANDLE_Z + 3.0,
            );
        handles = handles + handle;
    }
    handles
}

fn occlusion_grade_markers() -> Part {
    let mut markers = Part::empty("closed_lane_occlusion_recovery_occlusion_grade_markers");
    for lane in 0..OCCLUSION_CARTRIDGES {
        let (x, y) = lane_abs_xy(lane, OCCLUSION_CENTER);
        let grade = lane % OCCLUSION_GRADES;
        let fraction = OCCLUSION_PARTIAL_OPEN_FRACTIONS[grade];
        for i in 0..=grade {
            markers = markers
                + centered_cube(
                    format!("closed_lane_occlusion_recovery_lane_{lane}_grade_{grade}_marker_{i}"),
                    5.0 + fraction * 4.0,
                    7.0,
                    5.0,
                )
                .translate(
                    x - 23.0 + i as f64 * 9.0,
                    y + OCCLUSION_SOCKET_Y / 2.0 + 13.0,
                    DECK_Z + OCCLUSION_BANK_Z + 2.5,
                );
        }
    }
    markers
}

fn occlusion_pull_tab_fences() -> Part {
    let front = centered_cube(
        "closed_lane_occlusion_recovery_occlusion_bank_front_pull_tab_fence",
        OCCLUSION_BANK_X - 42.0,
        8.0,
        18.0,
    )
    .translate(
        OCCLUSION_CENTER.0,
        OCCLUSION_CENTER.1 - OCCLUSION_BANK_Y / 2.0 + 18.0,
        DECK_Z + OCCLUSION_BANK_Z + 9.0,
    );
    let rear = centered_cube(
        "closed_lane_occlusion_recovery_occlusion_bank_rear_pull_tab_fence",
        OCCLUSION_BANK_X - 42.0,
        8.0,
        18.0,
    )
    .translate(
        OCCLUSION_CENTER.0,
        OCCLUSION_CENTER.1 + OCCLUSION_BANK_Y / 2.0 - 18.0,
        DECK_Z + OCCLUSION_BANK_Z + 9.0,
    );
    front + rear
}

fn upstream_downstream_pressure_port_panel() -> Part {
    let body = centered_cube(
        "closed_lane_occlusion_recovery_upstream_downstream_pressure_port_panel_body",
        PRESSURE_PANEL_X,
        PRESSURE_PANEL_Y,
        PRESSURE_PANEL_Z,
    )
    .translate(
        PRESSURE_CENTER.0,
        PRESSURE_CENTER.1,
        DECK_Z + PRESSURE_PANEL_Z / 2.0,
    );

    body - pressure_port_bores()
        + pressure_port_rings()
        + pressure_signature_cable_troughs()
        + pressure_zero_reference_ports()
        + pressure_delta_witness_bars()
}

fn pressure_port_bores() -> Part {
    let mut bores = Part::empty("closed_lane_occlusion_recovery_pressure_port_bores");
    for lane in 0..LANE_COUNT {
        let (x, y) = pressure_lane_xy(lane);
        for (side, x_offset) in [
            ("upstream", -PRESSURE_PORT_PAIR_SPACING_X / 2.0),
            ("downstream", PRESSURE_PORT_PAIR_SPACING_X / 2.0),
        ] {
            bores = bores
                + centered_cylinder(
                    format!("closed_lane_occlusion_recovery_lane_{lane}_{side}_pressure_port_bore"),
                    PRESSURE_PORT_D / 2.0,
                    PRESSURE_PANEL_Z + 8.0,
                    24,
                )
                .translate(x + x_offset, y, DECK_Z + PRESSURE_PANEL_Z / 2.0);
        }
    }
    bores
}

fn pressure_port_rings() -> Part {
    let mut rings = Part::empty("closed_lane_occlusion_recovery_pressure_port_rings");
    for lane in 0..LANE_COUNT {
        let (x, y) = pressure_lane_xy(lane);
        for (side, x_offset) in [
            ("upstream", -PRESSURE_PORT_PAIR_SPACING_X / 2.0),
            ("downstream", PRESSURE_PORT_PAIR_SPACING_X / 2.0),
        ] {
            let ring = centered_cylinder(
                format!("closed_lane_occlusion_recovery_lane_{lane}_{side}_pressure_ring_outer"),
                PRESSURE_PORT_RING_D / 2.0,
                5.0,
                30,
            )
            .translate(x + x_offset, y, DECK_Z + PRESSURE_PANEL_Z + 2.5)
                - centered_cylinder(
                    format!(
                        "closed_lane_occlusion_recovery_lane_{lane}_{side}_pressure_ring_inner"
                    ),
                    PRESSURE_PORT_D / 2.0,
                    7.0,
                    24,
                )
                .translate(x + x_offset, y, DECK_Z + PRESSURE_PANEL_Z + 2.5);
            rings = rings + ring;
        }
    }
    rings
}

fn pressure_signature_cable_troughs() -> Part {
    let mut troughs =
        Part::empty("closed_lane_occlusion_recovery_pressure_signature_cable_troughs");
    for row in 0..PRESSURE_CABLE_TROUGHS {
        let y = PRESSURE_CENTER.1 + centered_index(row, PRESSURE_CABLE_TROUGHS, 42.0);
        troughs = troughs
            + centered_cube(
                format!("closed_lane_occlusion_recovery_pressure_row_{row}_cable_trough"),
                PRESSURE_PANEL_X - 64.0,
                7.0,
                7.0,
            )
            .translate(PRESSURE_CENTER.0, y, DECK_Z + PRESSURE_PANEL_Z + 3.5);
    }
    troughs
}

fn pressure_zero_reference_ports() -> Part {
    let mut refs = Part::empty("closed_lane_occlusion_recovery_pressure_zero_reference_ports");
    for i in 0..PRESSURE_ZERO_REFERENCE_PORTS {
        let x = PRESSURE_CENTER.0 + centered_index(i, PRESSURE_ZERO_REFERENCE_PORTS, 54.0);
        let reference = centered_cylinder(
            format!("closed_lane_occlusion_recovery_pressure_zero_reference_port_{i}"),
            8.0,
            8.0,
            24,
        )
        .translate(
            x,
            PRESSURE_CENTER.1 + PRESSURE_PANEL_Y / 2.0 - 22.0,
            DECK_Z + PRESSURE_PANEL_Z + 4.0,
        ) - centered_cylinder(
            format!("closed_lane_occlusion_recovery_pressure_zero_reference_port_{i}_center"),
            3.4,
            10.0,
            20,
        )
        .translate(
            x,
            PRESSURE_CENTER.1 + PRESSURE_PANEL_Y / 2.0 - 22.0,
            DECK_Z + PRESSURE_PANEL_Z + 4.0,
        );
        refs = refs + reference;
    }
    refs
}

fn pressure_delta_witness_bars() -> Part {
    let nominal = centered_cube(
        "closed_lane_occlusion_recovery_nominal_delta_pressure_witness_bar",
        84.0,
        8.0,
        9.0,
    )
    .translate(
        PRESSURE_CENTER.0 - PRESSURE_PANEL_X / 2.0 + 76.0,
        PRESSURE_CENTER.1 - PRESSURE_PANEL_Y / 2.0 + 28.0,
        DECK_Z + PRESSURE_PANEL_Z + 4.5,
    );
    let alert = centered_cube(
        "closed_lane_occlusion_recovery_occlusion_alert_delta_pressure_witness_bar",
        118.0,
        8.0,
        13.0,
    )
    .translate(
        PRESSURE_CENTER.0 - PRESSURE_PANEL_X / 2.0 + 94.0,
        PRESSURE_CENTER.1 - PRESSURE_PANEL_Y / 2.0 + 48.0,
        DECK_Z + PRESSURE_PANEL_Z + 6.5,
    );
    nominal + alert
}

fn bypass_relief_path_witness() -> Part {
    let body = centered_cube(
        "closed_lane_occlusion_recovery_bypass_relief_path_witness_body",
        BYPASS_X,
        BYPASS_Y,
        BYPASS_Z,
    )
    .translate(BYPASS_CENTER.0, BYPASS_CENTER.1, DECK_Z + BYPASS_Z / 2.0);

    body - bypass_branch_cuts()
        + relief_witness_window_frames()
        + relief_cartridge_bosses()
        + bypass_to_waste_header()
        + relief_threshold_flags()
}

fn bypass_branch_cuts() -> Part {
    let mut cuts = Part::empty("closed_lane_occlusion_recovery_bypass_branch_cuts");
    for lane in 0..BYPASS_BRANCHES {
        let x = BYPASS_CENTER.0 + centered_index(lane % 8, 8, 56.0);
        let y = BYPASS_CENTER.1 + if lane < 8 { -34.0 } else { 34.0 };
        cuts = cuts
            + centered_cube(
                format!("closed_lane_occlusion_recovery_lane_{lane}_bypass_branch_trace"),
                42.0,
                BYPASS_BRANCH_W,
                8.0,
            )
            .translate(x, y, DECK_Z + BYPASS_Z - 3.0);
    }
    cuts
}

fn relief_witness_window_frames() -> Part {
    let mut frames = Part::empty("closed_lane_occlusion_recovery_relief_witness_window_frames");
    for lane in 0..RELIEF_WITNESS_WINDOWS {
        let x = BYPASS_CENTER.0 + centered_index(lane % 8, 8, 56.0);
        let y = BYPASS_CENTER.1 + if lane < 8 { -62.0 } else { 62.0 };
        frames = frames
            + rectangular_frame(
                &format!("closed_lane_occlusion_recovery_lane_{lane}_relief_witness_window"),
                34.0,
                16.0,
                3.0,
                5.0,
            )
            .translate(x, y, DECK_Z + BYPASS_Z + 2.5);
    }
    frames
}

fn relief_cartridge_bosses() -> Part {
    let mut bosses = Part::empty("closed_lane_occlusion_recovery_relief_cartridge_bosses");
    for lane in 0..RELIEF_CARTRIDGE_COUNT {
        let x = BYPASS_CENTER.0 + centered_index(lane % 8, 8, 56.0);
        let y = BYPASS_CENTER.1 + if lane < 8 { -8.0 } else { 8.0 };
        let boss = centered_cylinder(
            format!("closed_lane_occlusion_recovery_lane_{lane}_relief_cartridge_boss_outer"),
            13.0,
            12.0,
            28,
        )
        .translate(x, y, DECK_Z + BYPASS_Z + 6.0)
            - centered_cylinder(
                format!("closed_lane_occlusion_recovery_lane_{lane}_relief_cartridge_boss_inner"),
                5.0,
                14.0,
                22,
            )
            .translate(x, y, DECK_Z + BYPASS_Z + 6.0);
        bosses = bosses + boss;
    }
    bosses
}

fn bypass_to_waste_header() -> Part {
    let header = centered_cube(
        "closed_lane_occlusion_recovery_bypass_relief_waste_header",
        BYPASS_X - 52.0,
        BYPASS_HEADER_W,
        9.0,
    )
    .translate(
        BYPASS_CENTER.0,
        BYPASS_CENTER.1 - BYPASS_Y / 2.0 + 20.0,
        DECK_Z + BYPASS_Z + 4.5,
    );
    let outlet_arrow = centered_cube(
        "closed_lane_occlusion_recovery_bypass_relief_to_quarantine_arrow_shaft",
        62.0,
        8.0,
        8.0,
    )
    .translate(
        BYPASS_CENTER.0 + BYPASS_X / 2.0 - 48.0,
        BYPASS_CENTER.1 - BYPASS_Y / 2.0 + 20.0,
        DECK_Z + BYPASS_Z + 13.0,
    ) + centered_cube(
        "closed_lane_occlusion_recovery_bypass_relief_to_quarantine_arrow_head",
        18.0,
        18.0,
        8.0,
    )
    .rotate(0.0, 0.0, 45.0)
    .translate(
        BYPASS_CENTER.0 + BYPASS_X / 2.0 - 8.0,
        BYPASS_CENTER.1 - BYPASS_Y / 2.0 + 20.0,
        DECK_Z + BYPASS_Z + 13.0,
    );
    header + outlet_arrow
}

fn relief_threshold_flags() -> Part {
    let nominal = centered_cube(
        "closed_lane_occlusion_recovery_relief_nominal_setpoint_flag",
        56.0,
        18.0,
        18.0,
    )
    .translate(
        BYPASS_CENTER.0 - BYPASS_X / 2.0 + 44.0,
        BYPASS_CENTER.1 + BYPASS_Y / 2.0 - 22.0,
        DECK_Z + BYPASS_Z + 9.0,
    );
    let reject = centered_cube(
        "closed_lane_occlusion_recovery_relief_reject_threshold_flag",
        78.0,
        18.0,
        24.0,
    )
    .translate(
        BYPASS_CENTER.0 + BYPASS_X / 2.0 - 56.0,
        BYPASS_CENTER.1 + BYPASS_Y / 2.0 - 22.0,
        DECK_Z + BYPASS_Z + 12.0,
    );
    nominal + reject
}

fn bubble_debris_capture_windows() -> Part {
    let body = centered_cube(
        "closed_lane_occlusion_recovery_bubble_debris_capture_window_body",
        CAPTURE_X,
        CAPTURE_Y,
        CAPTURE_Z,
    )
    .translate(CAPTURE_CENTER.0, CAPTURE_CENTER.1, DECK_Z + CAPTURE_Z / 2.0);

    body - capture_window_cuts()
        + capture_window_frames()
        + debris_capture_wells()
        + debris_screen_ribs()
        + gel_plug_reference_coupons()
}

fn capture_window_cuts() -> Part {
    let mut cuts = Part::empty("closed_lane_occlusion_recovery_capture_window_cuts");
    for lane in 0..BUBBLE_DEBRIS_WINDOWS {
        let x = CAPTURE_CENTER.0 + centered_index(lane % 8, 8, 48.0);
        let y = CAPTURE_CENTER.1 + if lane < 8 { -26.0 } else { 26.0 };
        cuts = cuts
            + centered_cube(
                format!("closed_lane_occlusion_recovery_lane_{lane}_bubble_debris_window_cut"),
                CAPTURE_WINDOW_X,
                CAPTURE_WINDOW_Y,
                CAPTURE_Z + 8.0,
            )
            .translate(x, y, DECK_Z + CAPTURE_Z / 2.0);
    }
    cuts
}

fn capture_window_frames() -> Part {
    let mut frames = Part::empty("closed_lane_occlusion_recovery_capture_window_frames");
    for lane in 0..BUBBLE_DEBRIS_WINDOWS {
        let x = CAPTURE_CENTER.0 + centered_index(lane % 8, 8, 48.0);
        let y = CAPTURE_CENTER.1 + if lane < 8 { -26.0 } else { 26.0 };
        frames = frames
            + rectangular_frame(
                &format!("closed_lane_occlusion_recovery_lane_{lane}_bubble_debris_window_frame"),
                CAPTURE_WINDOW_X + 10.0,
                CAPTURE_WINDOW_Y + 8.0,
                3.0,
                6.0,
            )
            .translate(x, y, DECK_Z + CAPTURE_Z + 3.0);
    }
    frames
}

fn debris_capture_wells() -> Part {
    let mut wells = Part::empty("closed_lane_occlusion_recovery_debris_capture_wells");
    for lane in 0..BUBBLE_DEBRIS_WINDOWS {
        let x = CAPTURE_CENTER.0 + centered_index(lane % 8, 8, 48.0);
        let y = CAPTURE_CENTER.1 + if lane < 8 { -54.0 } else { 54.0 };
        let well = centered_cylinder(
            format!("closed_lane_occlusion_recovery_lane_{lane}_debris_capture_well_outer"),
            CAPTURE_WELL_D / 2.0 + 5.0,
            9.0,
            28,
        )
        .translate(x, y, DECK_Z + CAPTURE_Z + 4.5)
            - centered_cylinder(
                format!("closed_lane_occlusion_recovery_lane_{lane}_debris_capture_well_recess"),
                CAPTURE_WELL_D / 2.0,
                11.0,
                24,
            )
            .translate(x, y, DECK_Z + CAPTURE_Z + 4.5);
        wells = wells + well;
    }
    wells
}

fn debris_screen_ribs() -> Part {
    let mut ribs = Part::empty("closed_lane_occlusion_recovery_debris_screen_ribs");
    for lane in 0..BUBBLE_DEBRIS_WINDOWS {
        let x = CAPTURE_CENTER.0 + centered_index(lane % 8, 8, 48.0);
        let y = CAPTURE_CENTER.1 + if lane < 8 { -2.0 } else { 2.0 };
        for rib in 0..3 {
            ribs = ribs
                + centered_cube(
                    format!("closed_lane_occlusion_recovery_lane_{lane}_debris_screen_rib_{rib}"),
                    4.0,
                    24.0,
                    5.0,
                )
                .translate(x - 8.0 + rib as f64 * 8.0, y, DECK_Z + CAPTURE_Z + 2.5);
        }
    }
    ribs
}

fn gel_plug_reference_coupons() -> Part {
    let mut coupons = Part::empty("closed_lane_occlusion_recovery_gel_plug_reference_coupons");
    for i in 0..GEL_PLUG_REFERENCE_COUPONS {
        let x = CAPTURE_CENTER.0 + centered_index(i, GEL_PLUG_REFERENCE_COUPONS, 58.0);
        coupons = coupons
            + centered_cube(
                format!("closed_lane_occlusion_recovery_gel_plug_reference_coupon_{i}"),
                36.0,
                18.0,
                10.0,
            )
            .translate(
                x,
                CAPTURE_CENTER.1 + CAPTURE_Y / 2.0 - 18.0,
                DECK_Z + CAPTURE_Z + 5.0,
            );
    }
    coupons
}

fn recovery_pulse_limiter_gauges() -> Part {
    let body = centered_cube(
        "closed_lane_occlusion_recovery_pulse_limiter_gauge_body",
        PULSE_X,
        PULSE_Y,
        PULSE_Z,
    )
    .translate(PULSE_CENTER.0, PULSE_CENTER.1, DECK_Z + PULSE_Z / 2.0);

    body - pulse_chamber_cuts()
        + pulse_gauge_faces()
        + pulse_hard_stop_pins()
        + global_pulse_reference_gauges()
        + unsafe_pulse_reject_flags()
}

fn pulse_chamber_cuts() -> Part {
    let mut cuts = Part::empty("closed_lane_occlusion_recovery_pulse_chamber_cuts");
    for lane in 0..PULSE_LIMITER_GAUGES {
        let (x, y) = lane_abs_xy(lane, PULSE_CENTER);
        let chamber = centered_cylinder(
            format!("closed_lane_occlusion_recovery_lane_{lane}_recovery_pulse_chamber"),
            PULSE_CHAMBER_D / 2.0,
            PULSE_Z + 8.0,
            30,
        )
        .translate(x, y, DECK_Z + PULSE_Z / 2.0);
        cuts = cuts + chamber;
    }
    cuts
}

fn pulse_gauge_faces() -> Part {
    let mut gauges = Part::empty("closed_lane_occlusion_recovery_pulse_gauge_faces");
    for lane in 0..PULSE_LIMITER_GAUGES {
        let (x, y) = lane_abs_xy(lane, PULSE_CENTER);
        let gauge = centered_cylinder(
            format!("closed_lane_occlusion_recovery_lane_{lane}_recovery_pulse_limiter_gauge_outer"),
            PULSE_GAUGE_D / 2.0,
            6.0,
            32,
        )
        .translate(x, y, DECK_Z + PULSE_Z + 3.0)
            - centered_cylinder(
                format!("closed_lane_occlusion_recovery_lane_{lane}_recovery_pulse_limiter_gauge_center"),
                PULSE_GAUGE_D / 2.0 - 4.0,
                8.0,
                28,
            )
            .translate(x, y, DECK_Z + PULSE_Z + 3.0);
        let needle = centered_cube(
            format!("closed_lane_occlusion_recovery_lane_{lane}_safe_pulse_limiter_needle"),
            PULSE_GAUGE_D * 0.38,
            3.0,
            5.0,
        )
        .rotate(0.0, 0.0, -25.0 + (lane % LANE_COLS) as f64 * 10.0)
        .translate(x, y, DECK_Z + PULSE_Z + 8.0);
        gauges = gauges + gauge + needle;
    }
    gauges
}

fn pulse_hard_stop_pins() -> Part {
    let mut pins = Part::empty("closed_lane_occlusion_recovery_pulse_hard_stop_pins");
    for lane in 0..PULSE_HARD_STOP_COUNT {
        let (x, y) = lane_abs_xy(lane, PULSE_CENTER);
        pins = pins
            + centered_cube(
                format!("closed_lane_occlusion_recovery_lane_{lane}_pulse_hard_stop_pin"),
                8.0,
                8.0,
                16.0,
            )
            .translate(x + 22.0, y + 19.0, DECK_Z + PULSE_Z + 8.0);
    }
    pins
}

fn global_pulse_reference_gauges() -> Part {
    let mut refs = Part::empty("closed_lane_occlusion_recovery_global_pulse_reference_gauges");
    for i in 0..GLOBAL_PULSE_REFERENCE_GAUGES {
        let x = PULSE_CENTER.0 + if i == 0 { -58.0 } else { 58.0 };
        let gauge = centered_cylinder(
            format!("closed_lane_occlusion_recovery_global_pulse_reference_gauge_{i}"),
            18.0,
            7.0,
            32,
        )
        .translate(
            x,
            PULSE_CENTER.1 + PULSE_Y / 2.0 - 22.0,
            DECK_Z + PULSE_Z + 3.5,
        ) - centered_cylinder(
            format!("closed_lane_occlusion_recovery_global_pulse_reference_gauge_{i}_clear"),
            10.0,
            9.0,
            28,
        )
        .translate(
            x,
            PULSE_CENTER.1 + PULSE_Y / 2.0 - 22.0,
            DECK_Z + PULSE_Z + 3.5,
        );
        refs = refs + gauge;
    }
    refs
}

fn unsafe_pulse_reject_flags() -> Part {
    let safe = centered_cube(
        "closed_lane_occlusion_recovery_max_recovery_pulse_safe_flag",
        64.0,
        18.0,
        16.0,
    )
    .translate(
        PULSE_CENTER.0 - PULSE_X / 2.0 + 45.0,
        PULSE_CENTER.1 - PULSE_Y / 2.0 + 20.0,
        DECK_Z + PULSE_Z + 8.0,
    );
    let reject = centered_cube(
        "closed_lane_occlusion_recovery_unsafe_recovery_pulse_reject_flag",
        86.0,
        18.0,
        24.0,
    )
    .translate(
        PULSE_CENTER.0 + PULSE_X / 2.0 - 57.0,
        PULSE_CENTER.1 - PULSE_Y / 2.0 + 20.0,
        DECK_Z + PULSE_Z + 12.0,
    );
    safe + reject
}

fn waste_quarantine_outlet_cassette() -> Part {
    let body = centered_cube(
        "closed_lane_occlusion_recovery_waste_quarantine_outlet_cassette_body",
        WASTE_X,
        WASTE_Y,
        WASTE_Z,
    )
    .translate(WASTE_CENTER.0, WASTE_CENTER.1, DECK_Z + WASTE_Z / 2.0);
    let sump = centered_cube(
        "closed_lane_occlusion_recovery_waste_quarantine_sump_recess",
        WASTE_SUMP_X,
        WASTE_SUMP_Y,
        22.0,
    )
    .translate(WASTE_CENTER.0, WASTE_CENTER.1, DECK_Z + WASTE_Z - 10.0);

    body - sump - waste_inlet_bores()
        + waste_inlet_rings()
        + waste_quarantine_outlet()
        + waste_sample_wells()
        + waste_cassette_latches()
}

fn waste_inlet_bores() -> Part {
    let mut bores = Part::empty("closed_lane_occlusion_recovery_waste_inlet_bores");
    for lane in 0..WASTE_INLETS {
        let x = WASTE_CENTER.0 + centered_index(lane % 8, 8, 39.0);
        let y = WASTE_CENTER.1 + if lane < 8 { -24.0 } else { 24.0 };
        bores = bores
            + centered_cylinder(
                format!("closed_lane_occlusion_recovery_lane_{lane}_waste_quarantine_inlet_bore"),
                4.0,
                WASTE_Z + 8.0,
                22,
            )
            .translate(x, y, DECK_Z + WASTE_Z / 2.0);
    }
    bores
}

fn waste_inlet_rings() -> Part {
    let mut rings = Part::empty("closed_lane_occlusion_recovery_waste_inlet_rings");
    for lane in 0..WASTE_INLETS {
        let x = WASTE_CENTER.0 + centered_index(lane % 8, 8, 39.0);
        let y = WASTE_CENTER.1 + if lane < 8 { -24.0 } else { 24.0 };
        let ring = centered_cylinder(
            format!("closed_lane_occlusion_recovery_lane_{lane}_waste_quarantine_inlet_ring_outer"),
            9.0,
            5.0,
            24,
        )
        .translate(x, y, DECK_Z + WASTE_Z + 2.5)
            - centered_cylinder(
                format!(
                    "closed_lane_occlusion_recovery_lane_{lane}_waste_quarantine_inlet_ring_inner"
                ),
                4.0,
                7.0,
                22,
            )
            .translate(x, y, DECK_Z + WASTE_Z + 2.5);
        rings = rings + ring;
    }
    rings
}

fn waste_quarantine_outlet() -> Part {
    let outlet = centered_cylinder(
        "closed_lane_occlusion_recovery_single_waste_quarantine_outlet_boss",
        WASTE_OUTLET_D / 2.0 + 6.0,
        34.0,
        30,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(
        WASTE_CENTER.0 + WASTE_X / 2.0 + 17.0,
        WASTE_CENTER.1,
        DECK_Z + WASTE_Z / 2.0,
    ) - centered_cylinder(
        "closed_lane_occlusion_recovery_single_waste_quarantine_outlet_bore",
        WASTE_OUTLET_D / 2.0,
        38.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(
        WASTE_CENTER.0 + WASTE_X / 2.0 + 17.0,
        WASTE_CENTER.1,
        DECK_Z + WASTE_Z / 2.0,
    );
    let quarantine_flag = centered_cube(
        "closed_lane_occlusion_recovery_waste_quarantine_locked_outlet_flag",
        88.0,
        18.0,
        18.0,
    )
    .translate(
        WASTE_CENTER.0 + WASTE_X / 2.0 - 56.0,
        WASTE_CENTER.1 + WASTE_Y / 2.0 - 20.0,
        DECK_Z + WASTE_Z + 9.0,
    );
    outlet + quarantine_flag
}

fn waste_sample_wells() -> Part {
    let mut wells = Part::empty("closed_lane_occlusion_recovery_waste_sample_wells");
    for i in 0..WASTE_SAMPLE_WELLS {
        let x = WASTE_CENTER.0 + centered_index(i, WASTE_SAMPLE_WELLS, 50.0);
        let well = centered_cylinder(
            format!("closed_lane_occlusion_recovery_waste_sample_well_{i}_outer"),
            13.0,
            8.0,
            28,
        )
        .translate(
            x,
            WASTE_CENTER.1 - WASTE_Y / 2.0 + 22.0,
            DECK_Z + WASTE_Z + 4.0,
        ) - centered_cylinder(
            format!("closed_lane_occlusion_recovery_waste_sample_well_{i}_recess"),
            8.0,
            10.0,
            24,
        )
        .translate(
            x,
            WASTE_CENTER.1 - WASTE_Y / 2.0 + 22.0,
            DECK_Z + WASTE_Z + 4.0,
        );
        wells = wells + well;
    }
    wells
}

fn waste_cassette_latches() -> Part {
    let left = centered_cube(
        "closed_lane_occlusion_recovery_waste_cassette_left_latch",
        18.0,
        62.0,
        18.0,
    )
    .translate(
        WASTE_CENTER.0 - WASTE_X / 2.0 + 24.0,
        WASTE_CENTER.1,
        DECK_Z + WASTE_Z + 9.0,
    );
    let right = centered_cube(
        "closed_lane_occlusion_recovery_waste_cassette_right_latch",
        18.0,
        62.0,
        18.0,
    )
    .translate(
        WASTE_CENTER.0 + WASTE_X / 2.0 - 24.0,
        WASTE_CENTER.1,
        DECK_Z + WASTE_Z + 9.0,
    );
    left + right
}

fn camera_fiducial_bridge() -> Part {
    let bridge = centered_cube(
        "closed_lane_occlusion_recovery_camera_fiducial_bridge_beam",
        CAMERA_BRIDGE_X,
        CAMERA_BRIDGE_Y,
        CAMERA_BEAM_Z,
    )
    .translate(
        CAMERA_CENTER.0,
        CAMERA_CENTER.1,
        DECK_Z + CAMERA_CLEARANCE_Z + CAMERA_BEAM_Z / 2.0,
    );
    bridge + camera_heads() + lane_camera_fiducials() + global_camera_fiducials()
}

fn camera_heads() -> Part {
    let mut heads = Part::empty("closed_lane_occlusion_recovery_camera_heads");
    for i in 0..CAMERA_HEADS {
        let x = CAMERA_CENTER.0 + centered_index(i, CAMERA_HEADS, 350.0);
        let body = centered_cube(
            format!("closed_lane_occlusion_recovery_camera_head_{i}_body"),
            CAMERA_HEAD_X,
            CAMERA_HEAD_Y,
            CAMERA_HEAD_Z,
        )
        .translate(
            x,
            CAMERA_CENTER.1,
            DECK_Z + CAMERA_CLEARANCE_Z - CAMERA_HEAD_Z / 2.0,
        );
        let lens = centered_cylinder(
            format!("closed_lane_occlusion_recovery_camera_head_{i}_ring_light"),
            25.0,
            8.0,
            32,
        )
        .translate(
            x,
            CAMERA_CENTER.1,
            DECK_Z + CAMERA_CLEARANCE_Z - CAMERA_HEAD_Z - 4.0,
        ) - centered_cylinder(
            format!("closed_lane_occlusion_recovery_camera_head_{i}_lens_opening"),
            12.0,
            10.0,
            28,
        )
        .translate(
            x,
            CAMERA_CENTER.1,
            DECK_Z + CAMERA_CLEARANCE_Z - CAMERA_HEAD_Z - 4.0,
        );
        heads = heads + body + lens;
    }
    heads
}

fn lane_camera_fiducials() -> Part {
    let mut fiducials = Part::empty("closed_lane_occlusion_recovery_lane_camera_fiducials");
    for lane in 0..LANE_CAMERA_FIDUCIALS {
        let (x, y) = lane_abs_xy(lane, RACK_CENTER);
        fiducials = fiducials
            + crosshair(
                &format!("closed_lane_occlusion_recovery_lane_{lane}_camera_fiducial"),
                FIDUCIAL_ARM,
                4.0,
                5.0,
            )
            .translate(x, y + 31.0, DECK_Z + RACK_Z + 10.0);
    }
    fiducials
}

fn global_camera_fiducials() -> Part {
    let mut fiducials = Part::empty("closed_lane_occlusion_recovery_global_camera_fiducials");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 120.0, STATION_Y / 2.0 - 110.0),
        (STATION_X / 2.0 - 120.0, STATION_Y / 2.0 - 110.0),
        (-STATION_X / 2.0 + 120.0, -STATION_Y / 2.0 + 110.0),
        (STATION_X / 2.0 - 120.0, -STATION_Y / 2.0 + 110.0),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + crosshair(
                &format!("closed_lane_occlusion_recovery_global_camera_fiducial_{i}"),
                FIDUCIAL_ARM + 6.0,
                5.0,
                5.0,
            )
            .translate(*x, *y, DECK_Z + 7.0);
    }
    fiducials
}

fn release_hold_reject_evidence_gate() -> Part {
    let body = centered_cube(
        "closed_lane_occlusion_recovery_release_hold_reject_evidence_gate_body",
        DISPOSITION_X,
        DISPOSITION_Y,
        DISPOSITION_Z,
    )
    .translate(
        DISPOSITION_CENTER.0,
        DISPOSITION_CENTER.1,
        DECK_Z + DISPOSITION_Z / 2.0,
    );

    body - disposition_lane_recesses()
        + disposition_hold_reject_walls()
        + status_slot_tokens()
        + evidence_card_lands()
        + disposition_gate_markers()
}

fn disposition_lane_recesses() -> Part {
    let mut recesses = Part::empty("closed_lane_occlusion_recovery_disposition_lane_recesses");
    for lane in 0..DISPOSITION_LANES {
        let x = DISPOSITION_CENTER.0 + centered_index(lane, DISPOSITION_LANES, 96.0);
        recesses = recesses
            + centered_cube(
                format!("closed_lane_occlusion_recovery_disposition_lane_{lane}_recess"),
                78.0,
                DISPOSITION_Y - 30.0,
                10.0,
            )
            .translate(x, DISPOSITION_CENTER.1, DECK_Z + DISPOSITION_Z - 4.0);
    }
    recesses
}

fn disposition_hold_reject_walls() -> Part {
    let hold_wall = centered_cube(
        "closed_lane_occlusion_recovery_hold_gate_tall_evidence_wall",
        6.0,
        DISPOSITION_Y - 18.0,
        HOLD_REJECT_WALL_Z,
    )
    .translate(
        DISPOSITION_CENTER.0 - 48.0,
        DISPOSITION_CENTER.1,
        DECK_Z + DISPOSITION_Z + HOLD_REJECT_WALL_Z / 2.0,
    );
    let reject_wall = centered_cube(
        "closed_lane_occlusion_recovery_reject_gate_tall_evidence_wall",
        6.0,
        DISPOSITION_Y - 18.0,
        HOLD_REJECT_WALL_Z,
    )
    .translate(
        DISPOSITION_CENTER.0 + 48.0,
        DISPOSITION_CENTER.1,
        DECK_Z + DISPOSITION_Z + HOLD_REJECT_WALL_Z / 2.0,
    );
    hold_wall + reject_wall
}

fn status_slot_tokens() -> Part {
    let mut slots = Part::empty("closed_lane_occlusion_recovery_status_slot_tokens");
    for lane in 0..STATUS_SLOT_COUNT {
        let col = lane % 4;
        let row = lane / 4;
        let x = DISPOSITION_CENTER.0 - DISPOSITION_X / 2.0 + 30.0 + col as f64 * 34.0;
        let y = DISPOSITION_CENTER.1 - DISPOSITION_Y / 2.0 + 22.0 + row as f64 * 22.0;
        slots = slots
            + centered_cube(
                format!("closed_lane_occlusion_recovery_lane_{lane}_evidence_slot_token"),
                STATUS_SLOT_X,
                STATUS_SLOT_Y,
                6.0,
            )
            .translate(x, y, DECK_Z + DISPOSITION_Z + 3.0);
    }
    slots
}

fn evidence_card_lands() -> Part {
    let mut lands = Part::empty("closed_lane_occlusion_recovery_evidence_card_lands");
    for lane in 0..EVIDENCE_CARD_LANDS {
        let col = lane % 8;
        let row = lane / 8;
        let x = DISPOSITION_CENTER.0 + centered_index(col, 8, 28.0);
        let y = DISPOSITION_CENTER.1 + 28.0 + row as f64 * 24.0;
        lands = lands
            + centered_cube(
                format!("closed_lane_occlusion_recovery_lane_{lane}_pressure_signature_card_land"),
                22.0,
                16.0,
                4.0,
            )
            .translate(x, y, DECK_Z + DISPOSITION_Z + 2.0);
    }
    lands
}

fn disposition_gate_markers() -> Part {
    let release = centered_cube(
        "closed_lane_occlusion_recovery_release_marker_single_bar",
        44.0,
        6.0,
        8.0,
    )
    .translate(
        DISPOSITION_CENTER.0 - 96.0,
        DISPOSITION_CENTER.1 + DISPOSITION_Y / 2.0 - 16.0,
        DECK_Z + DISPOSITION_Z + 4.0,
    );
    let hold = centered_cube(
        "closed_lane_occlusion_recovery_hold_marker_bar_a",
        44.0,
        6.0,
        8.0,
    )
    .translate(
        DISPOSITION_CENTER.0,
        DISPOSITION_CENTER.1 + DISPOSITION_Y / 2.0 - 21.0,
        DECK_Z + DISPOSITION_Z + 4.0,
    ) + centered_cube(
        "closed_lane_occlusion_recovery_hold_marker_bar_b",
        44.0,
        6.0,
        8.0,
    )
    .translate(
        DISPOSITION_CENTER.0,
        DISPOSITION_CENTER.1 + DISPOSITION_Y / 2.0 - 11.0,
        DECK_Z + DISPOSITION_Z + 4.0,
    );
    let reject = centered_cube(
        "closed_lane_occlusion_recovery_reject_marker_bar_a",
        44.0,
        6.0,
        8.0,
    )
    .rotate(0.0, 0.0, 45.0)
    .translate(
        DISPOSITION_CENTER.0 + 96.0,
        DISPOSITION_CENTER.1 + DISPOSITION_Y / 2.0 - 16.0,
        DECK_Z + DISPOSITION_Z + 4.0,
    ) + centered_cube(
        "closed_lane_occlusion_recovery_reject_marker_bar_b",
        44.0,
        6.0,
        8.0,
    )
    .rotate(0.0, 0.0, -45.0)
    .translate(
        DISPOSITION_CENTER.0 + 96.0,
        DISPOSITION_CENTER.1 + DISPOSITION_Y / 2.0 - 16.0,
        DECK_Z + DISPOSITION_Z + 4.0,
    );
    release + hold + reject
}

fn robot_service_keepout_gauges() -> Part {
    let robot_sweep = centered_cube(
        "closed_lane_occlusion_recovery_robot_sweep_keepout_gauge",
        STATION_X - 190.0,
        12.0,
        KEEP_OUT_Z,
    )
    .translate(0.0, ROBOT_SWEEP_CLEARANCE_Y, DECK_Z + KEEP_OUT_Z / 2.0);
    let cartridge_lift = centered_cube(
        "closed_lane_occlusion_recovery_occlusion_cartridge_lift_clearance_gauge",
        OCCLUSION_BANK_X,
        10.0,
        KEEP_OUT_Z,
    )
    .translate(
        OCCLUSION_CENTER.0,
        OCCLUSION_CENTER.1 + OCCLUSION_BANK_Y / 2.0 + 32.0,
        DECK_Z + CARTRIDGE_LIFT_CLEARANCE_Z,
    );
    let pressure_service = centered_cube(
        "closed_lane_occlusion_recovery_pressure_cable_service_clearance_gauge",
        10.0,
        PRESSURE_PANEL_Y,
        KEEP_OUT_Z,
    )
    .translate(
        PRESSURE_CENTER.0 + PRESSURE_PANEL_X / 2.0 + PRESSURE_CABLE_SERVICE_CLEARANCE_X,
        PRESSURE_CENTER.1,
        DECK_Z + KEEP_OUT_Z / 2.0,
    );
    let waste_bag = centered_cube(
        "closed_lane_occlusion_recovery_waste_bag_removal_clearance_gauge",
        WASTE_X,
        10.0,
        KEEP_OUT_Z,
    )
    .translate(
        WASTE_CENTER.0,
        WASTE_CENTER.1 - WASTE_Y / 2.0 - WASTE_BAG_REMOVAL_CLEARANCE_Y,
        DECK_Z + KEEP_OUT_Z / 2.0,
    );
    let camera_lift = centered_cube(
        "closed_lane_occlusion_recovery_camera_lift_clearance_gauge",
        CAMERA_BRIDGE_X - 260.0,
        10.0,
        KEEP_OUT_Z,
    )
    .translate(
        CAMERA_CENTER.0,
        CAMERA_CENTER.1 + CAMERA_BRIDGE_Y / 2.0 + 22.0,
        DECK_Z + CAMERA_LIFT_CLEARANCE_Z,
    );
    let evidence_service = centered_cube(
        "closed_lane_occlusion_recovery_front_evidence_service_clearance_gauge",
        DISPOSITION_X,
        10.0,
        KEEP_OUT_Z,
    )
    .translate(
        DISPOSITION_CENTER.0,
        DISPOSITION_CENTER.1 - DISPOSITION_Y / 2.0 - FRONT_EVIDENCE_SERVICE_CLEARANCE_Y,
        DECK_Z + KEEP_OUT_Z / 2.0,
    );

    robot_sweep + cartridge_lift + pressure_service + waste_bag + camera_lift + evidence_service
}

fn rectangular_frame(name: &str, x: f64, y: f64, wall: f64, z: f64) -> Part {
    centered_cube(format!("{name}_outer"), x, y, z)
        - centered_cube(
            format!("{name}_inner"),
            x - wall * 2.0,
            y - wall * 2.0,
            z + 2.0,
        )
}

fn crosshair(name: &str, arm: f64, width: f64, z: f64) -> Part {
    centered_cube(format!("{name}_x_bar"), arm, width, z)
        + centered_cube(format!("{name}_y_bar"), width, arm, z)
        + centered_cylinder(format!("{name}_center_dot"), width * 0.72, z, 18)
}

fn lane_abs_xy(lane: usize, center: (f64, f64)) -> (f64, f64) {
    let col = lane % LANE_COLS;
    let row = lane / LANE_COLS;
    (
        center.0 + centered_index(col, LANE_COLS, LANE_PITCH_X),
        center.1 + centered_index(row, LANE_ROWS, LANE_PITCH_Y),
    )
}

fn pressure_lane_xy(lane: usize) -> (f64, f64) {
    let col = lane % LANE_COLS;
    let row = lane / LANE_COLS;
    (
        PRESSURE_CENTER.0 + centered_index(col, LANE_COLS, 96.0),
        PRESSURE_CENTER.1 + centered_index(row, LANE_ROWS, 42.0),
    )
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn module_rects() -> [Rect; 8] {
    [
        Rect {
            name: "chip_surrogate_lane_rack",
            center: RACK_CENTER,
            x: RACK_X,
            y: RACK_Y,
        },
        Rect {
            name: "calibrated_occlusion_cartridge_bank",
            center: OCCLUSION_CENTER,
            x: OCCLUSION_BANK_X,
            y: OCCLUSION_BANK_Y,
        },
        Rect {
            name: "recovery_pulse_limiter_gauges",
            center: PULSE_CENTER,
            x: PULSE_X,
            y: PULSE_Y,
        },
        Rect {
            name: "upstream_downstream_pressure_port_panel",
            center: PRESSURE_CENTER,
            x: PRESSURE_PANEL_X,
            y: PRESSURE_PANEL_Y,
        },
        Rect {
            name: "bypass_relief_path_witness",
            center: BYPASS_CENTER,
            x: BYPASS_X,
            y: BYPASS_Y,
        },
        Rect {
            name: "bubble_debris_capture_windows",
            center: CAPTURE_CENTER,
            x: CAPTURE_X,
            y: CAPTURE_Y,
        },
        Rect {
            name: "waste_quarantine_outlet_cassette",
            center: WASTE_CENTER,
            x: WASTE_X,
            y: WASTE_Y,
        },
        Rect {
            name: "release_hold_reject_evidence_gate",
            center: DISPOSITION_CENTER,
            x: DISPOSITION_X,
            y: DISPOSITION_Y,
        },
    ]
}

fn assert_design_constraints() {
    assert_eq!(
        LANE_ROWS * LANE_COLS,
        16,
        "station must keep a 4x4 lane map"
    );
    assert_eq!(
        LANE_COUNT, 16,
        "source-only station is tied to the 16-slot scaled chip philosophy"
    );
    assert_eq!(OCCLUSION_CARTRIDGES, LANE_COUNT);
    assert_eq!(PRESSURE_SENSOR_PORTS, LANE_COUNT * 2);
    assert_eq!(BYPASS_BRANCHES, LANE_COUNT);
    assert_eq!(RELIEF_WITNESS_WINDOWS, LANE_COUNT);
    assert_eq!(PULSE_LIMITER_GAUGES, LANE_COUNT);
    assert_eq!(BUBBLE_DEBRIS_WINDOWS, LANE_COUNT);
    assert_eq!(WASTE_INLETS, LANE_COUNT);
    assert_eq!(STATUS_SLOT_COUNT, LANE_COUNT);
    assert_eq!(CAMERA_FIDUCIALS, LANE_COUNT + GLOBAL_CAMERA_FIDUCIALS);
    assert_eq!(RACK_CLAMPS_PER_LANE * LANE_COUNT, 32);
    assert_eq!(RACK_DATUM_PIN_COUNT, 4);
    assert_eq!(DEBRIS_SCREEN_RIBS, LANE_COUNT * 3);
    assert_eq!(KEEP_OUT_GAUGES, 6);
    assert!(REVC_CHIP_LENGTH > 120.0 && REVC_CHIP_WIDTH > 80.0);
    assert!(LANE_PITCH_X > LANE_TRACE_X + 2.0 * LANE_PORT_D);
    assert!(LANE_PITCH_Y > LANE_TRACE_Y + 40.0);
    assert!(OCCLUSION_SOCKET_X < LANE_PITCH_X);
    assert!(OCCLUSION_SOCKET_Y < LANE_PITCH_Y);
    assert!(PRESSURE_PORT_PAIR_SPACING_X > PRESSURE_PORT_RING_D);
    assert!(WASTE_CAPTURE_VOLUME_ML >= 400.0);
    assert!(MAX_STEADY_STATE_DELTA_KPA < OCCLUSION_ALERT_DELTA_KPA);
    assert!(MAX_RECOVERY_PULSE_KPA < UNSAFE_RECOVERY_PULSE_KPA);
    assert!(RELIEF_SETPOINT_KPA < RELIEF_REJECT_KPA);

    let modules = module_rects();
    for module in modules {
        assert!(
            module.fits_inside_station(),
            "{} must fit inside the clean isolator deck",
            module.name
        );
    }
    for i in 0..modules.len() {
        for j in (i + 1)..modules.len() {
            assert!(
                !modules[i].overlaps_with_clearance(modules[j], 8.0),
                "{} overlaps {}",
                modules[i].name,
                modules[j].name
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lane_map_is_4x4_and_fully_instrumented() {
        assert_eq!(LANE_ROWS, 4);
        assert_eq!(LANE_COLS, 4);
        assert_eq!(LANE_COUNT, 16);
        assert_eq!(OCCLUSION_CARTRIDGES, LANE_COUNT);
        assert_eq!(PRESSURE_SENSOR_PORTS, LANE_COUNT * 2);
        assert_eq!(PULSE_LIMITER_GAUGES, LANE_COUNT);
    }

    #[test]
    fn validation_features_are_one_per_lane_where_safety_critical() {
        assert_eq!(BYPASS_BRANCHES, LANE_COUNT);
        assert_eq!(RELIEF_CARTRIDGE_COUNT, LANE_COUNT);
        assert_eq!(RELIEF_WITNESS_WINDOWS, LANE_COUNT);
        assert_eq!(BUBBLE_DEBRIS_WINDOWS, LANE_COUNT);
        assert_eq!(WASTE_INLETS, LANE_COUNT);
        assert_eq!(STATUS_SLOT_COUNT, LANE_COUNT);
    }

    #[test]
    fn dimensions_keep_modules_inside_the_clean_deck() {
        for module in module_rects() {
            assert!(module.fits_inside_station(), "{module:?}");
        }
    }

    #[test]
    fn adjacent_modules_keep_service_clearance() {
        let modules = module_rects();
        for i in 0..modules.len() {
            for j in (i + 1)..modules.len() {
                assert!(
                    !modules[i].overlaps_with_clearance(modules[j], 8.0),
                    "{} overlaps {}",
                    modules[i].name,
                    modules[j].name
                );
            }
        }
    }

    #[test]
    fn pressure_and_pulse_limits_are_ordered_for_reject_logic() {
        assert!(MAX_STEADY_STATE_DELTA_KPA < OCCLUSION_ALERT_DELTA_KPA);
        assert!(MAX_RECOVERY_PULSE_KPA < UNSAFE_RECOVERY_PULSE_KPA);
        assert!(RELIEF_SETPOINT_KPA < RELIEF_REJECT_KPA);
    }
}
