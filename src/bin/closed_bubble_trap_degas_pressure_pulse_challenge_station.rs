use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed bubble-trap, degas, and pressure-pulse challenge station.
//
// Intent:
// - Validate a closed-loop perfusion path for scaled tissue-on-chip work
//   without making the printed parts a final wetted-process assembly.
// - Present dissolved-gas conditioning, high-point bubble removal, pressure
//   pulse tolerance, purge capture, optical bubble detection, and electrical
//   bubble detection as mechanically traceable challenge stations.
// - Keep route, evidence, custody, and service/robot keepout geometry explicit
//   so later parent integration can place this fixture without changing it.

const OUTPUT_PREFIX: &str = "output/closed_bubble_trap_degas_pressure_pulse_challenge_station";
const OUTPUTS: [&str; 12] = [
    "output/closed_bubble_trap_degas_pressure_pulse_challenge_station_containment_deck.stl",
    "output/closed_bubble_trap_degas_pressure_pulse_challenge_station_closed_loop_reservoir_bulkhead.stl",
    "output/closed_bubble_trap_degas_pressure_pulse_challenge_station_dissolved_gas_conditioning_cassette.stl",
    "output/closed_bubble_trap_degas_pressure_pulse_challenge_station_vertical_bubble_trap_cartridge_bank.stl",
    "output/closed_bubble_trap_degas_pressure_pulse_challenge_station_pressure_pulse_challenge_manifold.stl",
    "output/closed_bubble_trap_degas_pressure_pulse_challenge_station_tissue_chip_perfusion_surrogate_nest.stl",
    "output/closed_bubble_trap_degas_pressure_pulse_challenge_station_optical_electrical_bubble_detection_bridge.stl",
    "output/closed_bubble_trap_degas_pressure_pulse_challenge_station_purge_path_capture_rack.stl",
    "output/closed_bubble_trap_degas_pressure_pulse_challenge_station_run_record_custody_plate.stl",
    "output/closed_bubble_trap_degas_pressure_pulse_challenge_station_reference_coupon_storage.stl",
    "output/closed_bubble_trap_degas_pressure_pulse_challenge_station_robot_service_keepouts.stl",
    "output/closed_bubble_trap_degas_pressure_pulse_challenge_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "closed_loop_perfusion_lane_manifold",
    "dissolved_gas_conditioning_cassette",
    "vertical_high_point_bubble_trap_bank",
    "pressure_pulse_challenge_manifold",
    "purge_path_capture_rack",
    "optical_bubble_detection_bridge",
    "electrical_bubble_detection_electrode_pairs",
    "tissue_chip_perfusion_surrogate_nest",
    "pressure_flow_witness_taps",
    "run_record_custody_plate",
    "robot_service_keepouts",
];

const LIMITATIONS: [&str; 6] = [
    "mechanical_validation_packaging_only",
    "not_a_pressure_rated_wetted_design",
    "not_a_sterility_protocol",
    "purchased_membranes_sensors_and_connectors_are_placeholders",
    "pressure_limits_are_challenge_labels_not_release_criteria",
    "media_gas_setpoints_require_process_validation",
];

const STATION_X: f64 = 1520.0;
const STATION_Y: f64 = 940.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 46.0;
const BASIN_RECESS_Z: f64 = 7.0;
const MOUNT_HOLE_D: f64 = 6.8;
const DATUM_RING_D: f64 = 18.0;
const DESIGN_CLEARANCE: f64 = 12.0;

const LANES: usize = 6;
const TUBE_OD_MAX: f64 = 4.8;
const TUBE_CLEARANCE: f64 = 1.0;
const FLUID_BORE_D: f64 = TUBE_OD_MAX + TUBE_CLEARANCE;
const LANE_PITCH_X: f64 = 58.0;
const LANE_PITCH_Y: f64 = 30.0;

const LOOP_CENTER: (f64, f64) = (0.0, 340.0);
const LOOP_X: f64 = 1080.0;
const LOOP_Y: f64 = 74.0;
const LOOP_Z: f64 = 58.0;
const STERILE_PORTS: usize = LANES * 2;
const RESERVOIR_CAPS: usize = LANES;
const LOOP_CHECK_VALVES: usize = LANES * 2;

const DEGAS_CENTER: (f64, f64) = (-445.0, 140.0);
const DEGAS_X: f64 = 380.0;
const DEGAS_Y: f64 = 230.0;
const DEGAS_Z: f64 = 72.0;
const DEGAS_MEMBRANE_WINDOWS: usize = LANES;
const DEGAS_WINDOW_X: f64 = 42.0;
const DEGAS_WINDOW_Y: f64 = 74.0;
const VACUUM_SWEEP_PORTS: usize = LANES + 2;
const DISSOLVED_GAS_REFERENCE_WELLS: usize = 4;
const DEGAS_SWEEP_MANIFOLD_Z: f64 = 18.0;

const TRAP_CENTER: (f64, f64) = (0.0, 140.0);
const TRAP_X: f64 = 380.0;
const TRAP_Y: f64 = 260.0;
const TRAP_Z: f64 = 96.0;
const TRAP_COUNT: usize = LANES;
const TRAP_COLUMN_D: f64 = 32.0;
const TRAP_COLUMN_Z: f64 = 84.0;
const HIGH_POINT_VENTS: usize = LANES;
const TRAP_WINDOW_X: f64 = 34.0;
const TRAP_WINDOW_Y: f64 = 14.0;
const LIQUID_SEAL_DEPTH_MM: f64 = 18.0;

const PULSE_CENTER: (f64, f64) = (455.0, 140.0);
const PULSE_X: f64 = 380.0;
const PULSE_Y: f64 = 230.0;
const PULSE_Z: f64 = 68.0;
const PULSE_CHAMBERS: usize = LANES;
const PULSE_VALVES: usize = LANES * 2;
const PRESSURE_TAPS: usize = LANES;
const FLOW_WITNESS_WINDOWS: usize = LANES;
const PULSE_CHAMBER_D: f64 = 30.0;
const RELIEF_SETPOINT_KPA: f64 = 35.0;
const PULSE_TEST_KPA: f64 = 60.0;
const PROOF_LABEL_KPA: f64 = 85.0;

const CHIP_CENTER: (f64, f64) = (-445.0, -150.0);
const CHIP_X: f64 = 390.0;
const CHIP_Y: f64 = 240.0;
const CHIP_Z: f64 = 50.0;
const CHIP_SITES: usize = LANES;
const CHIP_WINDOW_X: f64 = 112.0;
const CHIP_WINDOW_Y: f64 = 18.0;
const CHIP_CLAMPS_PER_SITE: usize = 2;

const DETECTOR_CENTER: (f64, f64) = (0.0, -150.0);
const DETECTOR_X: f64 = 390.0;
const DETECTOR_Y: f64 = 240.0;
const DETECTOR_Z: f64 = 80.0;
const OPTICAL_WINDOWS: usize = LANES * 2;
const ELECTRICAL_ELECTRODE_PAIRS: usize = LANES;
const ELECTRODE_POST_D: f64 = 8.0;
const RI_BLANK_COUPONS: usize = 3;
const AIR_SLUG_COUPONS: usize = 4;

const PURGE_CENTER: (f64, f64) = (445.0, -150.0);
const PURGE_X: f64 = 390.0;
const PURGE_Y: f64 = 240.0;
const PURGE_Z: f64 = 64.0;
const PURGE_CUPS: usize = LANES;
const PURGE_BRANCHES: usize = LANES * 2;
const PURGE_CUP_D: f64 = 35.0;
const PURGE_CUP_DEPTH: f64 = 42.0;
const MIN_PURGE_CAPTURE_VOLUME_ML: f64 = 150.0;

const TRACE_CENTER: (f64, f64) = (0.0, -370.0);
const TRACE_X: f64 = 1000.0;
const TRACE_Y: f64 = 82.0;
const TRACE_Z: f64 = 14.0;
const BARCODE_LANDS: usize = LANES + 2;
const RUN_RECORD_LANDS: usize = 5;
const SEAL_TABS: usize = 4;

const COUPON_CENTER: (f64, f64) = (623.0, -370.0);
const COUPON_X: f64 = 210.0;
const COUPON_Y: f64 = 92.0;
const COUPON_Z: f64 = 34.0;
const REFERENCE_COUPONS: usize = RI_BLANK_COUPONS + AIR_SLUG_COUPONS;

const KEEP_OUT_GAUGES: usize = 6;
const ROBOT_SWEEP_CLEARANCE_Y: f64 = 245.0;
const DEGAS_MEMBRANE_LIFT_Z: f64 = 210.0;
const TRAP_CARTRIDGE_LIFT_Z: f64 = 240.0;
const PRESSURE_PANEL_SERVICE_X: f64 = 180.0;
const DETECTOR_CAMERA_CLEARANCE_Z: f64 = 260.0;
const PURGE_RACK_SERVICE_Y: f64 = 165.0;

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

    let loop_bulkhead = closed_loop_reservoir_bulkhead();
    export(&loop_bulkhead, OUTPUTS[1]);

    let degas = dissolved_gas_conditioning_cassette();
    export(&degas, OUTPUTS[2]);

    let traps = vertical_bubble_trap_cartridge_bank();
    export(&traps, OUTPUTS[3]);

    let pulses = pressure_pulse_challenge_manifold();
    export(&pulses, OUTPUTS[4]);

    let chips = tissue_chip_perfusion_surrogate_nest();
    export(&chips, OUTPUTS[5]);

    let detectors = optical_electrical_bubble_detection_bridge();
    export(&detectors, OUTPUTS[6]);

    let purge = purge_path_capture_rack();
    export(&purge, OUTPUTS[7]);

    let trace = run_record_custody_plate();
    export(&trace, OUTPUTS[8]);

    let coupons = reference_coupon_storage();
    export(&coupons, OUTPUTS[9]);

    let keepouts = robot_service_keepouts();
    export(&keepouts, OUTPUTS[10]);

    let assembly = deck
        + loop_bulkhead
        + degas
        + traps
        + pulses
        + chips
        + detectors
        + purge
        + trace
        + coupons
        + keepouts;
    export(&assembly, OUTPUTS[11]);

    println!(
        "Closed bubble-trap/degas/pressure-pulse challenge station: {STATION_X:.0}mm x {STATION_Y:.0}mm containment deck, {LANES} closed-loop perfusion lanes, {STERILE_PORTS} sterile bulkhead ports, {DEGAS_MEMBRANE_WINDOWS} degas membrane windows, {TRAP_COUNT} high-point bubble traps, and {PULSE_CHAMBERS} pressure pulse chambers."
    );
    println!(
        "Detection and capture coverage: {OPTICAL_WINDOWS} optical windows, {ELECTRICAL_ELECTRODE_PAIRS} electrical electrode pairs, {PURGE_BRANCHES} purge branches, {PURGE_CUPS} capture cups, {PRESSURE_TAPS} pressure taps, {FLOW_WITNESS_WINDOWS} flow witness windows, {REFERENCE_COUPONS} reference coupons, {} limitation markers, and {} STL outputs.",
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
        "closed_bubble_trap_degas_pulse_containment_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let basin = centered_cube(
        "closed_bubble_trap_degas_pulse_sumped_basin_recess",
        STATION_X - RIM_W * 2.0 - 56.0,
        STATION_Y - RIM_W * 2.0 - 56.0,
        BASIN_RECESS_Z,
    )
    .translate(0.0, 0.0, BASE_Z - BASIN_RECESS_Z / 2.0 + 0.2);
    let drain = centered_cylinder(
        "closed_bubble_trap_degas_pulse_front_drain_bore",
        9.0,
        72.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 88.0,
        -STATION_Y / 2.0 + 18.0,
        BASE_Z - 5.0,
    );

    deck - basin - drain - deck_mount_holes() + deck_rims() + datum_rings() + route_flow_arrows()
}

fn deck_mount_holes() -> Part {
    let mut holes = Part::empty("closed_bubble_trap_degas_pulse_deck_mount_holes");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 64.0, -STATION_Y / 2.0 + 58.0),
        (STATION_X / 2.0 - 64.0, -STATION_Y / 2.0 + 58.0),
        (-STATION_X / 2.0 + 64.0, STATION_Y / 2.0 - 58.0),
        (STATION_X / 2.0 - 64.0, STATION_Y / 2.0 - 58.0),
        (-260.0, 0.0),
        (260.0, 0.0),
    ]
    .iter()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("closed_bubble_trap_degas_pulse_deck_mount_bore_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 6.0,
                28,
            )
            .translate(*x, *y, BASE_Z / 2.0);
    }
    holes
}

fn deck_rims() -> Part {
    let left = centered_cube(
        "closed_bubble_trap_degas_pulse_left_spill_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let right = centered_cube(
        "closed_bubble_trap_degas_pulse_right_spill_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let rear = centered_cube(
        "closed_bubble_trap_degas_pulse_rear_spill_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let front = centered_cube(
        "closed_bubble_trap_degas_pulse_front_low_spill_lip",
        STATION_X - 140.0,
        12.0,
        24.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 6.0, BASE_Z + 12.0);
    left + right + rear + front
}

fn datum_rings() -> Part {
    let mut rings = Part::empty("closed_bubble_trap_degas_pulse_datum_rings");
    for (i, (x, y)) in [
        (DEGAS_CENTER.0, DEGAS_CENTER.1),
        (TRAP_CENTER.0, TRAP_CENTER.1),
        (PULSE_CENTER.0, PULSE_CENTER.1),
        (CHIP_CENTER.0, CHIP_CENTER.1),
        (DETECTOR_CENTER.0, DETECTOR_CENTER.1),
        (PURGE_CENTER.0, PURGE_CENTER.1),
    ]
    .iter()
    .enumerate()
    {
        let ring = centered_cylinder(
            format!("closed_bubble_trap_degas_pulse_module_datum_ring_{i}"),
            DATUM_RING_D / 2.0,
            4.0,
            32,
        )
        .translate(*x, *y, BASE_Z + 2.0)
            - centered_cylinder(
                format!("closed_bubble_trap_degas_pulse_module_datum_dot_{i}"),
                4.0,
                6.0,
                24,
            )
            .translate(*x, *y, BASE_Z + 2.0);
        rings = rings + ring;
    }
    rings
}

fn route_flow_arrows() -> Part {
    let mut arrows = Part::empty("closed_bubble_trap_degas_pulse_route_flow_arrows");
    for i in 0..LANES {
        let x = lane_x(i);
        let rear_arrow = centered_cube(
            format!("closed_bubble_trap_degas_pulse_rear_loop_flow_arrow_{i}"),
            34.0,
            7.0,
            4.0,
        )
        .translate(x, 288.0, BASE_Z + 4.0)
            + centered_cube(
                format!("closed_bubble_trap_degas_pulse_rear_loop_flow_arrow_head_{i}"),
                9.0,
                20.0,
                4.0,
            )
            .rotate(0.0, 0.0, 45.0)
            .translate(x + 20.0, 288.0, BASE_Z + 4.0);
        let front_arrow = centered_cube(
            format!("closed_bubble_trap_degas_pulse_front_return_flow_arrow_{i}"),
            34.0,
            7.0,
            4.0,
        )
        .translate(x, -288.0, BASE_Z + 4.0)
            + centered_cube(
                format!("closed_bubble_trap_degas_pulse_front_return_flow_arrow_head_{i}"),
                9.0,
                20.0,
                4.0,
            )
            .rotate(0.0, 0.0, -45.0)
            .translate(x - 20.0, -288.0, BASE_Z + 4.0);
        arrows = arrows + rear_arrow + front_arrow;
    }
    arrows
}

fn closed_loop_reservoir_bulkhead() -> Part {
    let body = centered_cube(
        "closed_bubble_trap_degas_pulse_closed_loop_bulkhead_body",
        LOOP_X,
        LOOP_Y,
        LOOP_Z,
    )
    .translate(LOOP_CENTER.0, LOOP_CENTER.1, BASE_Z + LOOP_Z / 2.0);
    let ports = closed_loop_port_cuts();
    let gasket_rings = closed_loop_gasket_rings();
    let caps = reservoir_cap_witnesses();
    let check_valves = check_valve_orientation_markers();
    let route_rails = closed_loop_route_rails();

    body - ports + gasket_rings + caps + check_valves + route_rails
}

fn closed_loop_port_cuts() -> Part {
    let mut cuts = Part::empty("closed_bubble_trap_degas_pulse_closed_loop_port_cuts");
    for i in 0..LANES {
        let x = lane_x(i);
        for (j, y_offset) in [-18.0, 18.0].iter().enumerate() {
            cuts = cuts
                + centered_cylinder(
                    format!("closed_bubble_trap_degas_pulse_loop_lane_{i}_port_{j}_bore"),
                    FLUID_BORE_D / 2.0,
                    LOOP_Y + 14.0,
                    24,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(x, LOOP_CENTER.1 + y_offset, BASE_Z + LOOP_Z / 2.0);
        }
    }
    cuts
}

fn closed_loop_gasket_rings() -> Part {
    let mut rings = Part::empty("closed_bubble_trap_degas_pulse_closed_loop_gasket_rings");
    for i in 0..LANES {
        let x = lane_x(i);
        for (j, y_offset) in [-18.0, 18.0].iter().enumerate() {
            let ring = centered_cylinder(
                format!("closed_bubble_trap_degas_pulse_loop_lane_{i}_port_{j}_gasket_outer"),
                11.0,
                4.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, LOOP_CENTER.1 + y_offset, BASE_Z + LOOP_Z / 2.0)
                - centered_cylinder(
                    format!("closed_bubble_trap_degas_pulse_loop_lane_{i}_port_{j}_gasket_inner"),
                    FLUID_BORE_D / 2.0,
                    6.0,
                    24,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(x, LOOP_CENTER.1 + y_offset, BASE_Z + LOOP_Z / 2.0);
            rings = rings + ring;
        }
    }
    rings
}

fn reservoir_cap_witnesses() -> Part {
    let mut caps = Part::empty("closed_bubble_trap_degas_pulse_reservoir_cap_witnesses");
    for i in 0..RESERVOIR_CAPS {
        let x = lane_x(i);
        let cap = centered_cylinder(
            format!("closed_bubble_trap_degas_pulse_reservoir_cap_{i}"),
            15.0,
            13.0,
            32,
        )
        .translate(x, LOOP_CENTER.1, BASE_Z + LOOP_Z + 6.5)
            - centered_cylinder(
                format!("closed_bubble_trap_degas_pulse_reservoir_cap_{i}_septum_dot"),
                4.0,
                15.0,
                20,
            )
            .translate(x, LOOP_CENTER.1, BASE_Z + LOOP_Z + 6.5);
        caps = caps + cap;
    }
    caps
}

fn check_valve_orientation_markers() -> Part {
    let mut markers =
        Part::empty("closed_bubble_trap_degas_pulse_loop_check_valve_orientation_markers");
    for i in 0..LANES {
        let x = lane_x(i);
        for (j, y_offset) in [-31.0, 31.0].iter().enumerate() {
            let marker = centered_cube(
                format!("closed_bubble_trap_degas_pulse_loop_check_valve_{i}_{j}_body"),
                26.0,
                9.0,
                7.0,
            )
            .translate(x, LOOP_CENTER.1 + y_offset, BASE_Z + LOOP_Z + 5.0)
                + centered_cube(
                    format!("closed_bubble_trap_degas_pulse_loop_check_valve_{i}_{j}_arrow"),
                    9.0,
                    9.0,
                    7.0,
                )
                .rotate(0.0, 0.0, if j == 0 { 45.0 } else { -45.0 })
                .translate(
                    x + if j == 0 { 17.0 } else { -17.0 },
                    LOOP_CENTER.1 + y_offset,
                    BASE_Z + LOOP_Z + 5.0,
                );
            markers = markers + marker;
        }
    }
    markers
}

fn closed_loop_route_rails() -> Part {
    let upper = centered_cube(
        "closed_bubble_trap_degas_pulse_closed_loop_upper_route_rail",
        LOOP_X - 120.0,
        8.0,
        9.0,
    )
    .translate(LOOP_CENTER.0, LOOP_CENTER.1 + 44.0, BASE_Z + LOOP_Z + 4.5);
    let lower = centered_cube(
        "closed_bubble_trap_degas_pulse_closed_loop_lower_route_rail",
        LOOP_X - 120.0,
        8.0,
        9.0,
    )
    .translate(LOOP_CENTER.0, LOOP_CENTER.1 - 44.0, BASE_Z + LOOP_Z + 4.5);
    upper + lower
}

fn dissolved_gas_conditioning_cassette() -> Part {
    let body = centered_cube(
        "closed_bubble_trap_degas_pulse_dissolved_gas_conditioning_body",
        DEGAS_X,
        DEGAS_Y,
        DEGAS_Z,
    )
    .translate(DEGAS_CENTER.0, DEGAS_CENTER.1, BASE_Z + DEGAS_Z / 2.0);
    let windows = degas_membrane_window_cuts();
    let ribs = degas_membrane_window_frames();
    let sweep = vacuum_sweep_manifold();
    let references = dissolved_gas_reference_wells();
    let ports = degas_port_bosses();

    body - windows + ribs + sweep + references + ports
}

fn degas_membrane_window_cuts() -> Part {
    let mut cuts = Part::empty("closed_bubble_trap_degas_pulse_degas_membrane_window_cuts");
    for i in 0..DEGAS_MEMBRANE_WINDOWS {
        let x = DEGAS_CENTER.0 + centered_index(i, DEGAS_MEMBRANE_WINDOWS, 52.0);
        let window = centered_cube(
            format!("closed_bubble_trap_degas_pulse_degas_membrane_window_{i}"),
            DEGAS_WINDOW_X,
            DEGAS_WINDOW_Y,
            DEGAS_Z + 8.0,
        )
        .translate(x, DEGAS_CENTER.1, BASE_Z + DEGAS_Z / 2.0);
        cuts = cuts + window;
    }
    cuts
}

fn degas_membrane_window_frames() -> Part {
    let mut frames = Part::empty("closed_bubble_trap_degas_pulse_degas_membrane_window_frames");
    for i in 0..DEGAS_MEMBRANE_WINDOWS {
        let x = DEGAS_CENTER.0 + centered_index(i, DEGAS_MEMBRANE_WINDOWS, 52.0);
        frames = frames
            + rectangular_frame(
                &format!("closed_bubble_trap_degas_pulse_degas_membrane_frame_{i}"),
                DEGAS_WINDOW_X + 12.0,
                DEGAS_WINDOW_Y + 12.0,
                5.0,
                8.0,
            )
            .translate(x, DEGAS_CENTER.1, BASE_Z + DEGAS_Z + 4.0);
    }
    frames
}

fn vacuum_sweep_manifold() -> Part {
    let plenum = centered_cube(
        "closed_bubble_trap_degas_pulse_vacuum_sweep_plenum",
        DEGAS_X - 42.0,
        18.0,
        DEGAS_SWEEP_MANIFOLD_Z,
    )
    .translate(
        DEGAS_CENTER.0,
        DEGAS_CENTER.1 - DEGAS_Y / 2.0 + 30.0,
        BASE_Z + DEGAS_Z + DEGAS_SWEEP_MANIFOLD_Z / 2.0,
    );
    let mut ports = Part::empty("closed_bubble_trap_degas_pulse_vacuum_sweep_ports");
    for i in 0..VACUUM_SWEEP_PORTS {
        let x = DEGAS_CENTER.0 + centered_index(i, VACUUM_SWEEP_PORTS, 43.0);
        ports = ports
            + centered_cylinder(
                format!("closed_bubble_trap_degas_pulse_vacuum_sweep_port_{i}"),
                5.0,
                20.0,
                24,
            )
            .translate(
                x,
                DEGAS_CENTER.1 - DEGAS_Y / 2.0 + 30.0,
                BASE_Z + DEGAS_Z + DEGAS_SWEEP_MANIFOLD_Z + 10.0,
            );
    }
    plenum + ports
}

fn dissolved_gas_reference_wells() -> Part {
    let mut wells = Part::empty("closed_bubble_trap_degas_pulse_dissolved_gas_reference_wells");
    for i in 0..DISSOLVED_GAS_REFERENCE_WELLS {
        let x = DEGAS_CENTER.0 + centered_index(i, DISSOLVED_GAS_REFERENCE_WELLS, 70.0);
        let well = centered_cylinder(
            format!("closed_bubble_trap_degas_pulse_dissolved_gas_reference_well_{i}"),
            14.0,
            10.0,
            28,
        )
        .translate(
            x,
            DEGAS_CENTER.1 + DEGAS_Y / 2.0 - 32.0,
            BASE_Z + DEGAS_Z + 5.0,
        ) - centered_cylinder(
            format!("closed_bubble_trap_degas_pulse_dissolved_gas_reference_well_{i}_socket"),
            9.0,
            12.0,
            24,
        )
        .translate(
            x,
            DEGAS_CENTER.1 + DEGAS_Y / 2.0 - 32.0,
            BASE_Z + DEGAS_Z + 5.0,
        );
        wells = wells + well;
    }
    wells
}

fn degas_port_bosses() -> Part {
    let inlet = centered_cylinder(
        "closed_bubble_trap_degas_pulse_degas_inlet_boss",
        12.0,
        26.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(
        DEGAS_CENTER.0 - DEGAS_X / 2.0 - 13.0,
        DEGAS_CENTER.1,
        BASE_Z + DEGAS_Z / 2.0,
    );
    let outlet = centered_cylinder(
        "closed_bubble_trap_degas_pulse_degas_outlet_boss",
        12.0,
        26.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(
        DEGAS_CENTER.0 + DEGAS_X / 2.0 + 13.0,
        DEGAS_CENTER.1,
        BASE_Z + DEGAS_Z / 2.0,
    );
    inlet + outlet
}

fn vertical_bubble_trap_cartridge_bank() -> Part {
    let body = centered_cube(
        "closed_bubble_trap_degas_pulse_vertical_trap_bank_body",
        TRAP_X,
        TRAP_Y,
        TRAP_Z,
    )
    .translate(TRAP_CENTER.0, TRAP_CENTER.1, BASE_Z + TRAP_Z / 2.0);
    let cuts = trap_cartridge_cuts();
    let rims = trap_cartridge_rims();
    let vents = high_point_vent_capture_ports();
    let windows = trap_window_frames();
    let seal_gauge = liquid_seal_depth_gauge();

    body - cuts + rims + vents + windows + seal_gauge
}

fn trap_cartridge_cuts() -> Part {
    let mut cuts = Part::empty("closed_bubble_trap_degas_pulse_trap_cartridge_cuts");
    for i in 0..TRAP_COUNT {
        let x = lane_x(i);
        let cartridge = centered_cylinder(
            format!("closed_bubble_trap_degas_pulse_trap_cartridge_socket_{i}"),
            TRAP_COLUMN_D / 2.0,
            TRAP_COLUMN_Z,
            36,
        )
        .translate(x, TRAP_CENTER.1, BASE_Z + TRAP_Z / 2.0 + 6.0);
        let flow_bore = centered_cylinder(
            format!("closed_bubble_trap_degas_pulse_trap_cartridge_flow_bore_{i}"),
            FLUID_BORE_D / 2.0,
            TRAP_Y + 20.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, TRAP_CENTER.1, BASE_Z + 28.0);
        cuts = cuts + cartridge + flow_bore;
    }
    cuts
}

fn trap_cartridge_rims() -> Part {
    let mut rims = Part::empty("closed_bubble_trap_degas_pulse_trap_cartridge_rims");
    for i in 0..TRAP_COUNT {
        let x = lane_x(i);
        let rim = centered_cylinder(
            format!("closed_bubble_trap_degas_pulse_trap_cartridge_rim_{i}"),
            TRAP_COLUMN_D / 2.0 + 7.0,
            6.0,
            36,
        )
        .translate(x, TRAP_CENTER.1, BASE_Z + TRAP_Z + 3.0)
            - centered_cylinder(
                format!("closed_bubble_trap_degas_pulse_trap_cartridge_rim_{i}_clear"),
                TRAP_COLUMN_D / 2.0,
                8.0,
                36,
            )
            .translate(x, TRAP_CENTER.1, BASE_Z + TRAP_Z + 3.0);
        rims = rims + rim;
    }
    rims
}

fn high_point_vent_capture_ports() -> Part {
    let mut vents = Part::empty("closed_bubble_trap_degas_pulse_high_point_vent_capture_ports");
    for i in 0..HIGH_POINT_VENTS {
        let x = lane_x(i);
        let vent = centered_cylinder(
            format!("closed_bubble_trap_degas_pulse_high_point_vent_port_{i}"),
            5.0,
            25.0,
            24,
        )
        .translate(
            x,
            TRAP_CENTER.1 + TRAP_Y / 2.0 - 26.0,
            BASE_Z + TRAP_Z + 12.5,
        );
        let capture = centered_cylinder(
            format!("closed_bubble_trap_degas_pulse_high_point_vent_capture_cup_{i}"),
            13.0,
            12.0,
            28,
        )
        .translate(
            x,
            TRAP_CENTER.1 + TRAP_Y / 2.0 - 26.0,
            BASE_Z + TRAP_Z + 31.0,
        ) - centered_cylinder(
            format!("closed_bubble_trap_degas_pulse_high_point_vent_capture_cup_{i}_recess"),
            8.5,
            14.0,
            24,
        )
        .translate(
            x,
            TRAP_CENTER.1 + TRAP_Y / 2.0 - 26.0,
            BASE_Z + TRAP_Z + 31.0,
        );
        vents = vents + vent + capture;
    }
    vents
}

fn trap_window_frames() -> Part {
    let mut frames = Part::empty("closed_bubble_trap_degas_pulse_trap_window_frames");
    for i in 0..TRAP_COUNT {
        let x = lane_x(i);
        let front = rectangular_frame(
            &format!("closed_bubble_trap_degas_pulse_front_trap_window_frame_{i}"),
            TRAP_WINDOW_X,
            TRAP_WINDOW_Y,
            3.0,
            5.0,
        )
        .translate(x, TRAP_CENTER.1 - TRAP_Y / 2.0 - 2.5, BASE_Z + 58.0);
        let rear = rectangular_frame(
            &format!("closed_bubble_trap_degas_pulse_rear_trap_window_frame_{i}"),
            TRAP_WINDOW_X,
            TRAP_WINDOW_Y,
            3.0,
            5.0,
        )
        .translate(x, TRAP_CENTER.1 + TRAP_Y / 2.0 + 2.5, BASE_Z + 58.0);
        frames = frames + front + rear;
    }
    frames
}

fn liquid_seal_depth_gauge() -> Part {
    let gauge = centered_cube(
        "closed_bubble_trap_degas_pulse_liquid_seal_depth_gauge_bar",
        TRAP_X - 42.0,
        7.0,
        5.0,
    )
    .translate(
        TRAP_CENTER.0,
        TRAP_CENTER.1 - TRAP_Y / 2.0 + 28.0,
        BASE_Z + LIQUID_SEAL_DEPTH_MM,
    );
    let witness = centered_cube(
        "closed_bubble_trap_degas_pulse_minimum_liquid_seal_witness_tab",
        44.0,
        15.0,
        5.0,
    )
    .translate(
        TRAP_CENTER.0 + TRAP_X / 2.0 - 44.0,
        TRAP_CENTER.1 - TRAP_Y / 2.0 + 28.0,
        BASE_Z + LIQUID_SEAL_DEPTH_MM,
    );
    gauge + witness
}

fn pressure_pulse_challenge_manifold() -> Part {
    let body = centered_cube(
        "closed_bubble_trap_degas_pulse_pressure_pulse_manifold_body",
        PULSE_X,
        PULSE_Y,
        PULSE_Z,
    )
    .translate(PULSE_CENTER.0, PULSE_CENTER.1, BASE_Z + PULSE_Z / 2.0);
    let chambers = pressure_pulse_chamber_cuts();
    let valves = pressure_pulse_valve_blocks();
    let taps = pressure_flow_witness_taps();
    let relief = relief_bypass_rail();
    let labels = pressure_challenge_label_stack();

    body - chambers + valves + taps + relief + labels
}

fn pressure_pulse_chamber_cuts() -> Part {
    let mut cuts = Part::empty("closed_bubble_trap_degas_pulse_pressure_chamber_cuts");
    for i in 0..PULSE_CHAMBERS {
        let x = PULSE_CENTER.0 + centered_index(i, PULSE_CHAMBERS, 52.0);
        let chamber = centered_cylinder(
            format!("closed_bubble_trap_degas_pulse_pulse_chamber_{i}"),
            PULSE_CHAMBER_D / 2.0,
            PULSE_Z + 10.0,
            32,
        )
        .translate(x, PULSE_CENTER.1 - 22.0, BASE_Z + PULSE_Z / 2.0);
        let cross_bore = centered_cylinder(
            format!("closed_bubble_trap_degas_pulse_pulse_chamber_{i}_cross_bore"),
            FLUID_BORE_D / 2.0,
            PULSE_Y + 16.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, PULSE_CENTER.1, BASE_Z + 28.0);
        cuts = cuts + chamber + cross_bore;
    }
    cuts
}

fn pressure_pulse_valve_blocks() -> Part {
    let mut valves = Part::empty("closed_bubble_trap_degas_pulse_pressure_valve_blocks");
    for i in 0..LANES {
        let x = PULSE_CENTER.0 + centered_index(i, LANES, 52.0);
        for (j, y_offset) in [-74.0, 42.0].iter().enumerate() {
            valves = valves
                + centered_cube(
                    format!("closed_bubble_trap_degas_pulse_pressure_valve_lane_{i}_{j}"),
                    34.0,
                    24.0,
                    18.0,
                )
                .translate(x, PULSE_CENTER.1 + y_offset, BASE_Z + PULSE_Z + 9.0);
        }
    }
    valves
}

fn pressure_flow_witness_taps() -> Part {
    let mut taps = Part::empty("closed_bubble_trap_degas_pulse_pressure_flow_witness_taps");
    for i in 0..PRESSURE_TAPS {
        let x = PULSE_CENTER.0 + centered_index(i, PRESSURE_TAPS, 52.0);
        let tap = centered_cylinder(
            format!("closed_bubble_trap_degas_pulse_pressure_tap_{i}"),
            6.5,
            18.0,
            24,
        )
        .translate(
            x,
            PULSE_CENTER.1 + PULSE_Y / 2.0 - 26.0,
            BASE_Z + PULSE_Z + 9.0,
        );
        let window = centered_cube(
            format!("closed_bubble_trap_degas_pulse_flow_witness_window_{i}"),
            32.0,
            8.0,
            16.0,
        )
        .translate(x, PULSE_CENTER.1 - PULSE_Y / 2.0 - 4.0, BASE_Z + 42.0);
        taps = taps + tap + window;
    }
    taps
}

fn relief_bypass_rail() -> Part {
    let rail = centered_cube(
        "closed_bubble_trap_degas_pulse_relief_bypass_rail",
        PULSE_X - 62.0,
        16.0,
        18.0,
    )
    .translate(
        PULSE_CENTER.0,
        PULSE_CENTER.1 - PULSE_Y / 2.0 + 28.0,
        BASE_Z + PULSE_Z + 9.0,
    );
    let outlet = centered_cylinder(
        "closed_bubble_trap_degas_pulse_relief_bypass_to_purge_port",
        8.0,
        42.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(
        PULSE_CENTER.0 + PULSE_X / 2.0 + 20.0,
        PULSE_CENTER.1 - PULSE_Y / 2.0 + 28.0,
        BASE_Z + PULSE_Z + 9.0,
    );
    rail + outlet
}

fn pressure_challenge_label_stack() -> Part {
    let relief = centered_cube(
        "closed_bubble_trap_degas_pulse_relief_setpoint_label_land",
        68.0,
        22.0,
        4.0,
    )
    .translate(
        PULSE_CENTER.0 - 120.0,
        PULSE_CENTER.1 + 2.0,
        BASE_Z + PULSE_Z + 4.0,
    );
    let pulse = centered_cube(
        "closed_bubble_trap_degas_pulse_pulse_test_label_land",
        68.0,
        22.0,
        4.0,
    )
    .translate(PULSE_CENTER.0, PULSE_CENTER.1 + 2.0, BASE_Z + PULSE_Z + 4.0);
    let proof = centered_cube(
        "closed_bubble_trap_degas_pulse_proof_label_land",
        68.0,
        22.0,
        4.0,
    )
    .translate(
        PULSE_CENTER.0 + 120.0,
        PULSE_CENTER.1 + 2.0,
        BASE_Z + PULSE_Z + 4.0,
    );
    relief + pulse + proof
}

fn tissue_chip_perfusion_surrogate_nest() -> Part {
    let body = centered_cube(
        "closed_bubble_trap_degas_pulse_tissue_chip_surrogate_nest_body",
        CHIP_X,
        CHIP_Y,
        CHIP_Z,
    )
    .translate(CHIP_CENTER.0, CHIP_CENTER.1, BASE_Z + CHIP_Z / 2.0);
    let pockets = tissue_chip_site_cuts();
    let clamps = tissue_chip_clamps();
    let flow_ports = tissue_chip_flow_ports();
    let route_datum = tissue_chip_route_datum_comb();

    body - pockets - flow_ports + clamps + route_datum
}

fn tissue_chip_site_cuts() -> Part {
    let mut cuts = Part::empty("closed_bubble_trap_degas_pulse_tissue_chip_site_cuts");
    for i in 0..CHIP_SITES {
        let y = chip_lane_y(i);
        let pocket = centered_cube(
            format!("closed_bubble_trap_degas_pulse_tissue_chip_site_{i}_window"),
            CHIP_WINDOW_X,
            CHIP_WINDOW_Y,
            CHIP_Z + 6.0,
        )
        .translate(CHIP_CENTER.0, y, BASE_Z + CHIP_Z / 2.0);
        cuts = cuts + pocket;
    }
    cuts
}

fn tissue_chip_clamps() -> Part {
    let mut clamps = Part::empty("closed_bubble_trap_degas_pulse_tissue_chip_clamps");
    for i in 0..CHIP_SITES {
        let y = chip_lane_y(i);
        for (j, x_offset) in [-86.0, 86.0].iter().enumerate() {
            clamps = clamps
                + centered_cube(
                    format!("closed_bubble_trap_degas_pulse_tissue_chip_{i}_clamp_{j}"),
                    22.0,
                    18.0,
                    17.0,
                )
                .translate(CHIP_CENTER.0 + x_offset, y, BASE_Z + CHIP_Z + 8.5);
        }
    }
    clamps
}

fn tissue_chip_flow_ports() -> Part {
    let mut ports = Part::empty("closed_bubble_trap_degas_pulse_tissue_chip_flow_ports");
    for i in 0..CHIP_SITES {
        let y = chip_lane_y(i);
        for (j, x_offset) in [-148.0, 148.0].iter().enumerate() {
            ports = ports
                + centered_cylinder(
                    format!("closed_bubble_trap_degas_pulse_tissue_chip_lane_{i}_port_{j}"),
                    FLUID_BORE_D / 2.0,
                    28.0,
                    24,
                )
                .rotate(0.0, 90.0, 0.0)
                .translate(CHIP_CENTER.0 + x_offset, y, BASE_Z + 25.0);
        }
    }
    ports
}

fn tissue_chip_route_datum_comb() -> Part {
    let mut comb = Part::empty("closed_bubble_trap_degas_pulse_tissue_chip_route_datum_comb");
    for i in 0..CHIP_SITES {
        let y = chip_lane_y(i);
        let tooth = centered_cube(
            format!("closed_bubble_trap_degas_pulse_tissue_chip_route_tooth_{i}"),
            12.0,
            18.0,
            20.0,
        )
        .translate(
            CHIP_CENTER.0 + CHIP_X / 2.0 - 30.0,
            y,
            BASE_Z + CHIP_Z + 10.0,
        );
        comb = comb + tooth;
    }
    comb
}

fn optical_electrical_bubble_detection_bridge() -> Part {
    let base = centered_cube(
        "closed_bubble_trap_degas_pulse_detector_bridge_base",
        DETECTOR_X,
        DETECTOR_Y,
        18.0,
    )
    .translate(DETECTOR_CENTER.0, DETECTOR_CENTER.1, BASE_Z + 9.0);
    let side_posts = detector_side_posts();
    let top_beam = centered_cube(
        "closed_bubble_trap_degas_pulse_detector_bridge_top_beam",
        DETECTOR_X - 42.0,
        30.0,
        20.0,
    )
    .translate(
        DETECTOR_CENTER.0,
        DETECTOR_CENTER.1,
        BASE_Z + DETECTOR_Z + 10.0,
    );
    let optical = optical_fork_windows();
    let electrical = electrical_bubble_electrode_pairs();
    let challenge_coupons = detector_challenge_coupon_lands();

    base + side_posts + top_beam + optical + electrical + challenge_coupons
}

fn detector_side_posts() -> Part {
    let mut posts = Part::empty("closed_bubble_trap_degas_pulse_detector_side_posts");
    for (i, x) in [
        DETECTOR_CENTER.0 - DETECTOR_X / 2.0 + 24.0,
        DETECTOR_CENTER.0 + DETECTOR_X / 2.0 - 24.0,
    ]
    .iter()
    .enumerate()
    {
        for (j, y) in [
            DETECTOR_CENTER.1 - DETECTOR_Y / 2.0 + 24.0,
            DETECTOR_CENTER.1 + DETECTOR_Y / 2.0 - 24.0,
        ]
        .iter()
        .enumerate()
        {
            posts = posts
                + centered_cube(
                    format!("closed_bubble_trap_degas_pulse_detector_post_{i}_{j}"),
                    28.0,
                    28.0,
                    DETECTOR_Z,
                )
                .translate(*x, *y, BASE_Z + DETECTOR_Z / 2.0);
        }
    }
    posts
}

fn optical_fork_windows() -> Part {
    let mut forks = Part::empty("closed_bubble_trap_degas_pulse_optical_fork_windows");
    for i in 0..LANES {
        let y = detector_lane_y(i);
        for (j, x_offset) in [-72.0, -20.0].iter().enumerate() {
            let window = rectangular_frame(
                &format!("closed_bubble_trap_degas_pulse_optical_window_lane_{i}_{j}"),
                36.0,
                20.0,
                4.0,
                8.0,
            )
            .translate(DETECTOR_CENTER.0 + x_offset, y, BASE_Z + 46.0);
            let emitter = centered_cube(
                format!("closed_bubble_trap_degas_pulse_optical_emitter_lane_{i}_{j}"),
                12.0,
                8.0,
                22.0,
            )
            .translate(DETECTOR_CENTER.0 + x_offset - 24.0, y, BASE_Z + 40.0);
            let receiver = centered_cube(
                format!("closed_bubble_trap_degas_pulse_optical_receiver_lane_{i}_{j}"),
                12.0,
                8.0,
                22.0,
            )
            .translate(DETECTOR_CENTER.0 + x_offset + 24.0, y, BASE_Z + 40.0);
            forks = forks + window + emitter + receiver;
        }
    }
    forks
}

fn electrical_bubble_electrode_pairs() -> Part {
    let mut electrodes =
        Part::empty("closed_bubble_trap_degas_pulse_electrical_bubble_electrode_pairs");
    for i in 0..LANES {
        let y = detector_lane_y(i);
        for (j, x_offset) in [42.0, 78.0].iter().enumerate() {
            electrodes = electrodes
                + centered_cylinder(
                    format!("closed_bubble_trap_degas_pulse_electrode_lane_{i}_{j}"),
                    ELECTRODE_POST_D / 2.0,
                    30.0,
                    20,
                )
                .translate(DETECTOR_CENTER.0 + x_offset, y, BASE_Z + 35.0);
        }
        let bridge = centered_cube(
            format!("closed_bubble_trap_degas_pulse_electrode_pair_guard_lane_{i}"),
            58.0,
            6.0,
            6.0,
        )
        .translate(DETECTOR_CENTER.0 + 60.0, y, BASE_Z + 52.0);
        electrodes = electrodes + bridge;
    }
    electrodes
}

fn detector_challenge_coupon_lands() -> Part {
    let ri_lands = coupon_land_row(
        "closed_bubble_trap_degas_pulse_refractive_index_blank_land",
        RI_BLANK_COUPONS,
        DETECTOR_CENTER.0 - 94.0,
        DETECTOR_CENTER.1 - DETECTOR_Y / 2.0 + 30.0,
        38.0,
    );
    let slug_lands = coupon_land_row(
        "closed_bubble_trap_degas_pulse_air_slug_coupon_land",
        AIR_SLUG_COUPONS,
        DETECTOR_CENTER.0 + 72.0,
        DETECTOR_CENTER.1 - DETECTOR_Y / 2.0 + 30.0,
        34.0,
    );
    ri_lands + slug_lands
}

fn purge_path_capture_rack() -> Part {
    let body = centered_cube(
        "closed_bubble_trap_degas_pulse_purge_capture_rack_body",
        PURGE_X,
        PURGE_Y,
        PURGE_Z,
    )
    .translate(PURGE_CENTER.0, PURGE_CENTER.1, BASE_Z + PURGE_Z / 2.0);
    let cups = purge_cup_cuts();
    let cup_rims = purge_cup_rims();
    let branch = purge_branch_manifold();
    let bag_dock = purge_waste_bag_dock();
    let seals = purge_check_seal_witnesses();

    body - cups + cup_rims + branch + bag_dock + seals
}

fn purge_cup_cuts() -> Part {
    let mut cuts = Part::empty("closed_bubble_trap_degas_pulse_purge_cup_cuts");
    for i in 0..PURGE_CUPS {
        let y = purge_lane_y(i);
        cuts = cuts
            + centered_cylinder(
                format!("closed_bubble_trap_degas_pulse_purge_capture_cup_{i}"),
                PURGE_CUP_D / 2.0,
                PURGE_CUP_DEPTH,
                32,
            )
            .translate(
                PURGE_CENTER.0 - 88.0,
                y,
                BASE_Z + PURGE_Z - PURGE_CUP_DEPTH / 2.0 + 1.0,
            );
    }
    cuts
}

fn purge_cup_rims() -> Part {
    let mut rims = Part::empty("closed_bubble_trap_degas_pulse_purge_cup_rims");
    for i in 0..PURGE_CUPS {
        let y = purge_lane_y(i);
        let rim = centered_cylinder(
            format!("closed_bubble_trap_degas_pulse_purge_capture_cup_rim_{i}"),
            PURGE_CUP_D / 2.0 + 5.0,
            6.0,
            32,
        )
        .translate(PURGE_CENTER.0 - 88.0, y, BASE_Z + PURGE_Z + 3.0)
            - centered_cylinder(
                format!("closed_bubble_trap_degas_pulse_purge_capture_cup_rim_{i}_clear"),
                PURGE_CUP_D / 2.0,
                8.0,
                32,
            )
            .translate(PURGE_CENTER.0 - 88.0, y, BASE_Z + PURGE_Z + 3.0);
        rims = rims + rim;
    }
    rims
}

fn purge_branch_manifold() -> Part {
    let rail = centered_cube(
        "closed_bubble_trap_degas_pulse_purge_branch_manifold_rail",
        44.0,
        PURGE_Y - 46.0,
        18.0,
    )
    .translate(
        PURGE_CENTER.0 + 10.0,
        PURGE_CENTER.1,
        BASE_Z + PURGE_Z + 9.0,
    );
    let mut branches = Part::empty("closed_bubble_trap_degas_pulse_purge_branch_taps");
    for i in 0..PURGE_BRANCHES {
        let lane = i / 2;
        let y = purge_lane_y(lane) + if i % 2 == 0 { -6.0 } else { 6.0 };
        branches = branches
            + centered_cube(
                format!("closed_bubble_trap_degas_pulse_purge_branch_tap_{i}"),
                74.0,
                5.0,
                7.0,
            )
            .translate(PURGE_CENTER.0 - 38.0, y, BASE_Z + PURGE_Z + 8.0);
    }
    rail + branches
}

fn purge_waste_bag_dock() -> Part {
    let dock = centered_cube(
        "closed_bubble_trap_degas_pulse_purge_waste_bag_dock_body",
        126.0,
        82.0,
        36.0,
    )
    .translate(
        PURGE_CENTER.0 + 104.0,
        PURGE_CENTER.1,
        BASE_Z + PURGE_Z + 18.0,
    );
    let connector = centered_cylinder(
        "closed_bubble_trap_degas_pulse_purge_waste_bag_connector",
        12.0,
        42.0,
        28,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(
        PURGE_CENTER.0 + 186.0,
        PURGE_CENTER.1,
        BASE_Z + PURGE_Z + 18.0,
    );
    dock + connector
}

fn purge_check_seal_witnesses() -> Part {
    let mut seals = Part::empty("closed_bubble_trap_degas_pulse_purge_check_seal_witnesses");
    for i in 0..LANES {
        let y = purge_lane_y(i);
        seals = seals
            + centered_cube(
                format!("closed_bubble_trap_degas_pulse_purge_check_seal_lane_{i}"),
                34.0,
                9.0,
                7.0,
            )
            .translate(PURGE_CENTER.0 + 58.0, y, BASE_Z + PURGE_Z + 7.0);
    }
    seals
}

fn run_record_custody_plate() -> Part {
    let plate = centered_cube(
        "closed_bubble_trap_degas_pulse_run_record_custody_plate",
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    )
    .translate(TRACE_CENTER.0, TRACE_CENTER.1, BASE_Z + TRACE_Z / 2.0);
    let barcode_lands = coupon_land_row(
        "closed_bubble_trap_degas_pulse_barcode_land",
        BARCODE_LANDS,
        TRACE_CENTER.0 - 280.0,
        TRACE_CENTER.1,
        56.0,
    );
    let record_lands = coupon_land_row(
        "closed_bubble_trap_degas_pulse_run_record_land",
        RUN_RECORD_LANDS,
        TRACE_CENTER.0 + 190.0,
        TRACE_CENTER.1,
        76.0,
    );
    let seals = seal_tab_row();
    plate + barcode_lands + record_lands + seals
}

fn seal_tab_row() -> Part {
    let mut tabs = Part::empty("closed_bubble_trap_degas_pulse_custody_seal_tabs");
    for i in 0..SEAL_TABS {
        let x = TRACE_CENTER.0 + centered_index(i, SEAL_TABS, 94.0) + 330.0;
        tabs = tabs
            + centered_cube(
                format!("closed_bubble_trap_degas_pulse_custody_seal_tab_{i}"),
                48.0,
                22.0,
                8.0,
            )
            .translate(x, TRACE_CENTER.1, BASE_Z + TRACE_Z + 4.0);
    }
    tabs
}

fn reference_coupon_storage() -> Part {
    let block = centered_cube(
        "closed_bubble_trap_degas_pulse_reference_coupon_storage_body",
        COUPON_X,
        COUPON_Y,
        COUPON_Z,
    )
    .translate(COUPON_CENTER.0, COUPON_CENTER.1, BASE_Z + COUPON_Z / 2.0);
    let mut wells = Part::empty("closed_bubble_trap_degas_pulse_reference_coupon_wells");
    for i in 0..REFERENCE_COUPONS {
        let x = COUPON_CENTER.0 + centered_index(i % 4, 4, 42.0);
        let y = COUPON_CENTER.1 + centered_index(i / 4, 2, 38.0);
        wells = wells
            + centered_cube(
                format!("closed_bubble_trap_degas_pulse_reference_coupon_well_{i}"),
                28.0,
                22.0,
                18.0,
            )
            .translate(x, y, BASE_Z + COUPON_Z - 8.0);
    }
    block - wells + coupon_storage_lane_labels()
}

fn coupon_storage_lane_labels() -> Part {
    let ri = centered_cube(
        "closed_bubble_trap_degas_pulse_reference_coupon_ri_label_land",
        78.0,
        10.0,
        4.0,
    )
    .translate(
        COUPON_CENTER.0 - 46.0,
        COUPON_CENTER.1 + COUPON_Y / 2.0 - 10.0,
        BASE_Z + COUPON_Z + 2.0,
    );
    let slug = centered_cube(
        "closed_bubble_trap_degas_pulse_reference_coupon_slug_label_land",
        78.0,
        10.0,
        4.0,
    )
    .translate(
        COUPON_CENTER.0 + 46.0,
        COUPON_CENTER.1 + COUPON_Y / 2.0 - 10.0,
        BASE_Z + COUPON_Z + 2.0,
    );
    ri + slug
}

fn robot_service_keepouts() -> Part {
    let front_sweep = keepout_frame(
        "closed_bubble_trap_degas_pulse_front_robot_sweep_keepout",
        STATION_X - 180.0,
        ROBOT_SWEEP_CLEARANCE_Y,
        8.0,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 + ROBOT_SWEEP_CLEARANCE_Y / 2.0 + 22.0,
        BASE_Z + 8.0,
    );
    let degas_lift = keepout_frame(
        "closed_bubble_trap_degas_pulse_degas_membrane_lift_keepout",
        DEGAS_X + 70.0,
        DEGAS_Y + 70.0,
        8.0,
    )
    .translate(DEGAS_CENTER.0, DEGAS_CENTER.1, DEGAS_MEMBRANE_LIFT_Z);
    let trap_lift = keepout_frame(
        "closed_bubble_trap_degas_pulse_trap_cartridge_lift_keepout",
        TRAP_X + 70.0,
        TRAP_Y + 70.0,
        8.0,
    )
    .translate(TRAP_CENTER.0, TRAP_CENTER.1, TRAP_CARTRIDGE_LIFT_Z);
    let pressure_service = keepout_frame(
        "closed_bubble_trap_degas_pulse_pressure_panel_service_keepout",
        PRESSURE_PANEL_SERVICE_X,
        PULSE_Y + 100.0,
        8.0,
    )
    .translate(
        PULSE_CENTER.0 + PULSE_X / 2.0 - PRESSURE_PANEL_SERVICE_X / 2.0,
        PULSE_CENTER.1,
        BASE_Z + 10.0,
    );
    let detector_camera = keepout_frame(
        "closed_bubble_trap_degas_pulse_detector_camera_clearance_keepout",
        DETECTOR_X + 70.0,
        DETECTOR_Y + 70.0,
        8.0,
    )
    .translate(
        DETECTOR_CENTER.0,
        DETECTOR_CENTER.1,
        DETECTOR_CAMERA_CLEARANCE_Z,
    );
    let purge_service = keepout_frame(
        "closed_bubble_trap_degas_pulse_purge_rack_service_keepout",
        PURGE_X + 70.0,
        PURGE_RACK_SERVICE_Y,
        8.0,
    )
    .translate(
        PURGE_CENTER.0,
        PURGE_CENTER.1 - PURGE_Y / 2.0 + PURGE_RACK_SERVICE_Y / 2.0,
        BASE_Z + 10.0,
    );

    front_sweep + degas_lift + trap_lift + pressure_service + detector_camera + purge_service
}

fn coupon_land_row(name: &str, count: usize, center_x: f64, center_y: f64, pitch: f64) -> Part {
    let mut lands = Part::empty(format!("{name}_row"));
    for i in 0..count {
        lands = lands
            + centered_cube(format!("{name}_{i}"), 42.0, 18.0, 5.0).translate(
                center_x + centered_index(i, count, pitch),
                center_y,
                BASE_Z + TRACE_Z + 2.5,
            );
    }
    lands
}

fn rectangular_frame(name: &str, x: f64, y: f64, wall: f64, z: f64) -> Part {
    let front = centered_cube(format!("{name}_front"), x, wall, z).translate(0.0, -y / 2.0, 0.0);
    let rear = centered_cube(format!("{name}_rear"), x, wall, z).translate(0.0, y / 2.0, 0.0);
    let left = centered_cube(format!("{name}_left"), wall, y, z).translate(-x / 2.0, 0.0, 0.0);
    let right = centered_cube(format!("{name}_right"), wall, y, z).translate(x / 2.0, 0.0, 0.0);
    front + rear + left + right
}

fn keepout_frame(name: &str, x: f64, y: f64, z: f64) -> Part {
    let front =
        centered_cube(format!("{name}_front_rail"), x, 8.0, z).translate(0.0, -y / 2.0, 0.0);
    let rear = centered_cube(format!("{name}_rear_rail"), x, 8.0, z).translate(0.0, y / 2.0, 0.0);
    let left = centered_cube(format!("{name}_left_rail"), 8.0, y, z).translate(-x / 2.0, 0.0, 0.0);
    let right = centered_cube(format!("{name}_right_rail"), 8.0, y, z).translate(x / 2.0, 0.0, 0.0);
    front + rear + left + right
}

fn module_rects() -> [Rect; 9] {
    [
        rect(
            "closed_loop_reservoir_bulkhead",
            LOOP_CENTER,
            LOOP_X,
            LOOP_Y,
        ),
        rect(
            "dissolved_gas_conditioning_cassette",
            DEGAS_CENTER,
            DEGAS_X,
            DEGAS_Y,
        ),
        rect(
            "vertical_bubble_trap_cartridge_bank",
            TRAP_CENTER,
            TRAP_X,
            TRAP_Y,
        ),
        rect(
            "pressure_pulse_challenge_manifold",
            PULSE_CENTER,
            PULSE_X,
            PULSE_Y,
        ),
        rect(
            "tissue_chip_perfusion_surrogate_nest",
            CHIP_CENTER,
            CHIP_X,
            CHIP_Y,
        ),
        rect(
            "optical_electrical_bubble_detection_bridge",
            DETECTOR_CENTER,
            DETECTOR_X,
            DETECTOR_Y,
        ),
        rect("purge_path_capture_rack", PURGE_CENTER, PURGE_X, PURGE_Y),
        rect("run_record_custody_plate", TRACE_CENTER, TRACE_X, TRACE_Y),
        rect(
            "reference_coupon_storage",
            COUPON_CENTER,
            COUPON_X,
            COUPON_Y,
        ),
    ]
}

fn rect(name: &'static str, center: (f64, f64), x: f64, y: f64) -> Rect {
    Rect { name, center, x, y }
}

fn lane_x(index: usize) -> f64 {
    centered_index(index, LANES, LANE_PITCH_X)
}

fn chip_lane_y(index: usize) -> f64 {
    CHIP_CENTER.1 + centered_index(index, CHIP_SITES, LANE_PITCH_Y)
}

fn detector_lane_y(index: usize) -> f64 {
    DETECTOR_CENTER.1 + centered_index(index, LANES, LANE_PITCH_Y)
}

fn purge_lane_y(index: usize) -> f64 {
    PURGE_CENTER.1 + centered_index(index, PURGE_CUPS, LANE_PITCH_Y)
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn purge_capture_volume_ml() -> f64 {
    let per_cup_mm3 = std::f64::consts::PI * (PURGE_CUP_D / 2.0).powi(2) * (PURGE_CUP_DEPTH - 8.0);
    per_cup_mm3 * PURGE_CUPS as f64 / 1000.0
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 12);
    assert!(OUTPUTS.iter().all(|path| path.starts_with(OUTPUT_PREFIX)));
    assert_eq!(REQUIRED_FEATURES.len(), 11);
    assert_eq!(LIMITATIONS.len(), 6);
    assert_eq!(STERILE_PORTS, LANES * 2);
    assert_eq!(LOOP_CHECK_VALVES, LANES * 2);
    assert_eq!(DEGAS_MEMBRANE_WINDOWS, LANES);
    assert_eq!(TRAP_COUNT, LANES);
    assert_eq!(HIGH_POINT_VENTS, LANES);
    assert_eq!(PULSE_CHAMBERS, LANES);
    assert_eq!(PULSE_VALVES, LANES * 2);
    assert_eq!(PRESSURE_TAPS, LANES);
    assert_eq!(FLOW_WITNESS_WINDOWS, LANES);
    assert_eq!(CHIP_SITES, LANES);
    assert_eq!(OPTICAL_WINDOWS, LANES * 2);
    assert_eq!(ELECTRICAL_ELECTRODE_PAIRS, LANES);
    assert_eq!(PURGE_CUPS, LANES);
    assert_eq!(PURGE_BRANCHES, LANES * 2);
    assert_eq!(REFERENCE_COUPONS, RI_BLANK_COUPONS + AIR_SLUG_COUPONS);
    assert_eq!(KEEP_OUT_GAUGES, 6);
    assert_eq!(CHIP_CLAMPS_PER_SITE, 2);
    assert!(FLUID_BORE_D > TUBE_OD_MAX);
    assert!(DEGAS_WINDOW_X < DEGAS_X / LANES as f64);
    assert!(TRAP_COLUMN_Z < TRAP_Z);
    assert!(LIQUID_SEAL_DEPTH_MM > TUBE_OD_MAX * 3.0);
    assert!(RELIEF_SETPOINT_KPA < PULSE_TEST_KPA);
    assert!(PULSE_TEST_KPA < PROOF_LABEL_KPA);
    assert!(purge_capture_volume_ml() > MIN_PURGE_CAPTURE_VOLUME_ML);
    assert!(DETECTOR_CAMERA_CLEARANCE_Z > BASE_Z + DETECTOR_Z);
    assert!(TRAP_CARTRIDGE_LIFT_Z > BASE_Z + TRAP_Z);

    let rects = module_rects();
    for rect in rects {
        assert!(
            rect.fits_inside_station(),
            "{} exceeds station deck",
            rect.name
        );
    }
    for a in 0..rects.len() {
        for b in (a + 1)..rects.len() {
            assert!(
                !rects[a].overlaps_with_clearance(rects[b], DESIGN_CLEARANCE),
                "{} overlaps {}",
                rects[a].name,
                rects[b].name
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn outputs_are_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 12);
        for path in OUTPUTS {
            assert!(path
                .starts_with("output/closed_bubble_trap_degas_pressure_pulse_challenge_station_"));
            assert!(path.ends_with(".stl"));
        }
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }

    #[test]
    fn requested_validation_intent_is_represented() {
        assert!(REQUIRED_FEATURES.contains(&"closed_loop_perfusion_lane_manifold"));
        assert!(REQUIRED_FEATURES.contains(&"dissolved_gas_conditioning_cassette"));
        assert!(REQUIRED_FEATURES.contains(&"vertical_high_point_bubble_trap_bank"));
        assert!(REQUIRED_FEATURES.contains(&"pressure_pulse_challenge_manifold"));
        assert!(REQUIRED_FEATURES.contains(&"purge_path_capture_rack"));
        assert!(REQUIRED_FEATURES.contains(&"optical_bubble_detection_bridge"));
        assert!(REQUIRED_FEATURES.contains(&"electrical_bubble_detection_electrode_pairs"));
        assert!(REQUIRED_FEATURES.contains(&"tissue_chip_perfusion_surrogate_nest"));
    }

    #[test]
    fn closed_loop_lane_counts_match_detection_and_purge_paths() {
        assert_eq!(LANES, 6);
        assert_eq!(STERILE_PORTS, LANES * 2);
        assert_eq!(DEGAS_MEMBRANE_WINDOWS, LANES);
        assert_eq!(TRAP_COUNT, LANES);
        assert_eq!(PULSE_CHAMBERS, LANES);
        assert_eq!(OPTICAL_WINDOWS, LANES * 2);
        assert_eq!(ELECTRICAL_ELECTRODE_PAIRS, LANES);
        assert_eq!(PURGE_BRANCHES, LANES * 2);
    }

    #[test]
    fn pressure_and_capture_margins_are_ordered() {
        assert!(RELIEF_SETPOINT_KPA < PULSE_TEST_KPA);
        assert!(PULSE_TEST_KPA < PROOF_LABEL_KPA);
        assert!(purge_capture_volume_ml() > MIN_PURGE_CAPTURE_VOLUME_ML);
        assert!(LIQUID_SEAL_DEPTH_MM > TUBE_OD_MAX * 3.0);
        assert!(FLUID_BORE_D > TUBE_OD_MAX);
    }

    #[test]
    fn modules_fit_and_do_not_overlap() {
        assert_design_constraints();
        let rects = module_rects();
        for rect in rects {
            assert!(rect.fits_inside_station());
        }
    }

    #[test]
    fn workflow_order_runs_from_bulkhead_to_conditioning_trap_pulse_and_detection() {
        assert!(LOOP_CENTER.1 > DEGAS_CENTER.1);
        assert!(DEGAS_CENTER.0 < TRAP_CENTER.0);
        assert!(TRAP_CENTER.0 < PULSE_CENTER.0);
        assert!(CHIP_CENTER.0 < DETECTOR_CENTER.0);
        assert!(DETECTOR_CENTER.0 < PURGE_CENTER.0);
        assert!(TRACE_CENTER.1 < CHIP_CENTER.1);
    }

    #[test]
    fn lane_coordinate_helpers_are_symmetric() {
        assert_eq!(lane_x(0), -lane_x(LANES - 1));
        assert_eq!(
            chip_lane_y(0) - CHIP_CENTER.1,
            -(chip_lane_y(CHIP_SITES - 1) - CHIP_CENTER.1)
        );
        assert_eq!(
            detector_lane_y(0) - DETECTOR_CENTER.1,
            -(detector_lane_y(LANES - 1) - DETECTOR_CENTER.1)
        );
        assert_eq!(
            purge_lane_y(0) - PURGE_CENTER.1,
            -(purge_lane_y(PURGE_CUPS - 1) - PURGE_CENTER.1)
        );
    }
}
