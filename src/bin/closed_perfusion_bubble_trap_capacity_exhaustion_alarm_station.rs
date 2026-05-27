use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed perfusion bubble-trap capacity exhaustion and alarm validation station.
//
// Intent:
// - Challenge a closed perfusion bubble-trap cartridge bank through repeatable
//   air-slug loading until capacity exhaustion can be witnessed, alarmed, and
//   routed without opening the culture-side path.
// - Represent the mechanical validation architecture: graduated witness
//   chambers, slug injection manifold, optical bubble sensing, pressure/flow
//   taps, overflow/purge capture, alarm threshold tokens, diverter mock path,
//   disposition lanes, evidence capture, and robot/service keepouts.
// - Keep this as validation-fixture packaging only. It is not a pressure-rated
//   wetted manifold, sterile barrier design, sensor acceptance procedure, or
//   release workflow.

const PREFIX: &str = "closed_perfusion_bubble_trap_capacity_exhaustion_alarm_station";

const OUTPUTS: [&str; 13] = [
    "output/closed_perfusion_bubble_trap_capacity_exhaustion_alarm_station_secondary_containment_deck.stl",
    "output/closed_perfusion_bubble_trap_capacity_exhaustion_alarm_station_bubble_trap_cartridge_bank.stl",
    "output/closed_perfusion_bubble_trap_capacity_exhaustion_alarm_station_graduated_capacity_witness_chambers.stl",
    "output/closed_perfusion_bubble_trap_capacity_exhaustion_alarm_station_slug_injection_manifold.stl",
    "output/closed_perfusion_bubble_trap_capacity_exhaustion_alarm_station_optical_bubble_sensor_bridge.stl",
    "output/closed_perfusion_bubble_trap_capacity_exhaustion_alarm_station_pressure_flow_tap_panel.stl",
    "output/closed_perfusion_bubble_trap_capacity_exhaustion_alarm_station_overflow_purge_capture_cassette.stl",
    "output/closed_perfusion_bubble_trap_capacity_exhaustion_alarm_station_alarm_threshold_token_rail.stl",
    "output/closed_perfusion_bubble_trap_capacity_exhaustion_alarm_station_quarantine_diverter_mock_path.stl",
    "output/closed_perfusion_bubble_trap_capacity_exhaustion_alarm_station_release_hold_reject_lanes.stl",
    "output/closed_perfusion_bubble_trap_capacity_exhaustion_alarm_station_evidence_bridge.stl",
    "output/closed_perfusion_bubble_trap_capacity_exhaustion_alarm_station_robot_service_keepouts.stl",
    "output/closed_perfusion_bubble_trap_capacity_exhaustion_alarm_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 11] = [
    "bubble_trap_cartridge_bank",
    "graduated_capacity_witness_chambers",
    "slug_injection_manifold",
    "optical_bubble_sensor_bridge",
    "pressure_flow_taps",
    "overflow_purge_capture_cassette",
    "alarm_threshold_token_rail",
    "quarantine_diverter_mock_path",
    "release_hold_reject_lanes",
    "evidence_bridge",
    "robot_service_keepouts",
];

const LIMITATIONS: [&str; 6] = [
    "mechanical_validation_fixture_only",
    "not_a_pressure_rated_wetted_manifold",
    "not_a_sterile_barrier_specification",
    "purchased_traps_sensors_and_tubing_are_placeholders",
    "alarm_thresholds_are_validation_tokens_not_release_criteria",
    "closed_path_process_validation_required_before_use",
];

const STATION_X: f64 = 1500.0;
const STATION_Y: f64 = 950.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 20.0;
const RIM_Z: f64 = 48.0;
const BASIN_X: f64 = 1325.0;
const BASIN_Y: f64 = 795.0;
const BASIN_DEPTH: f64 = 8.0;
const MOUNT_HOLE_D: f64 = 6.8;
const MODULE_RISER_Z: f64 = 8.0;
const DESIGN_CLEARANCE: f64 = 12.0;
const LAYOUT_CLEARANCE: f64 = 10.0;

const TRAP_LANES: usize = 6;
const LANE_PITCH_X: f64 = 48.0;
const TUBE_OD_MAX_MM: f64 = 4.8;
const TUBE_BORE_D: f64 = 6.2;

const TRAP_POS: (f64, f64) = (-485.0, 210.0);
const TRAP_X: f64 = 350.0;
const TRAP_Y: f64 = 230.0;
const TRAP_BANK_Z: f64 = 70.0;
const TRAP_CARTRIDGES: usize = TRAP_LANES;
const TRAP_COLUMN_RADIUS: f64 = 16.0;
const TRAP_COLUMN_Z: f64 = 92.0;
const TRAP_HIGH_POINT_VENTS: usize = TRAP_LANES;
const TRAP_LATCH_COUNT: usize = TRAP_LANES * 2;

const WITNESS_POS: (f64, f64) = (-80.0, 210.0);
const WITNESS_X: f64 = 390.0;
const WITNESS_Y: f64 = 230.0;
const WITNESS_Z: f64 = 64.0;
const WITNESS_CHAMBERS: usize = TRAP_LANES;
const WITNESS_CHAMBER_RADIUS: f64 = 18.0;
const WITNESS_CHAMBER_Z: f64 = 46.0;
const WITNESS_GRADUATION_TICKS: usize = 7;

const SLUG_POS: (f64, f64) = (380.0, 210.0);
const SLUG_X: f64 = 420.0;
const SLUG_Y: f64 = 230.0;
const SLUG_Z: f64 = 58.0;
const SLUG_INJECTION_PORTS: usize = TRAP_LANES * 2;
const SLUG_REFERENCE_TOKENS: usize = TRAP_LANES;
const SLUG_METERING_WELLS: usize = TRAP_LANES;

const OPTICAL_POS: (f64, f64) = (-485.0, -55.0);
const OPTICAL_X: f64 = 350.0;
const OPTICAL_Y: f64 = 210.0;
const OPTICAL_Z: f64 = 92.0;
const OPTICAL_SENSOR_CHANNELS: usize = TRAP_LANES;
const OPTICAL_FORK_SPAN_Y: f64 = 132.0;
const OPTICAL_WINDOW_D: f64 = 12.0;

const TAP_POS: (f64, f64) = (-80.0, -55.0);
const TAP_X: f64 = 390.0;
const TAP_Y: f64 = 210.0;
const TAP_Z: f64 = 58.0;
const PRESSURE_TAPS_PER_LANE: usize = 2;
const PRESSURE_TAPS: usize = TRAP_LANES * PRESSURE_TAPS_PER_LANE;
const FLOW_WITNESS_WINDOWS: usize = TRAP_LANES;
const TAP_POST_D: f64 = 9.0;

const PURGE_POS: (f64, f64) = (380.0, -55.0);
const PURGE_X: f64 = 420.0;
const PURGE_Y: f64 = 210.0;
const PURGE_Z: f64 = 66.0;
const PURGE_CUPS: usize = TRAP_LANES;
const PURGE_CUP_RADIUS: f64 = 19.0;
const PURGE_CUP_Z: f64 = 50.0;
const OVERFLOW_BRANCHES: usize = TRAP_LANES * 2;

const ALARM_POS: (f64, f64) = (-485.0, -305.0);
const ALARM_X: f64 = 350.0;
const ALARM_Y: f64 = 160.0;
const ALARM_Z: f64 = 36.0;
const ALARM_LEVELS: usize = 3;
const ALARM_THRESHOLD_TOKENS: usize = TRAP_LANES * ALARM_LEVELS;
const WARNING_THRESHOLD_ML: f64 = 12.0;
const EXHAUSTION_THRESHOLD_ML: f64 = 18.0;
const REJECT_THRESHOLD_ML: f64 = 22.0;

const DIVERTER_POS: (f64, f64) = (-80.0, -305.0);
const DIVERTER_X: f64 = 390.0;
const DIVERTER_Y: f64 = 160.0;
const DIVERTER_Z: f64 = 62.0;
const DIVERTER_VALVES: usize = TRAP_LANES;
const QUARANTINE_BAG_DOCKS: usize = 2;

const LANES_POS: (f64, f64) = (380.0, -305.0);
const LANES_X: f64 = 420.0;
const LANES_Y: f64 = 160.0;
const LANES_Z: f64 = 42.0;
const DISPOSITION_LANES: usize = 3;
const TOKENS_PER_DISPOSITION_LANE: usize = 4;

const EVIDENCE_POS: (f64, f64) = (0.0, 390.0);
const EVIDENCE_X: f64 = 1200.0;
const EVIDENCE_Y: f64 = 70.0;
const EVIDENCE_UNDERSIDE_Z: f64 = 214.0;
const EVIDENCE_BEAM_Z: f64 = 30.0;
const EVIDENCE_CAMERAS: usize = 4;
const EVIDENCE_LIGHT_SEGMENTS: usize = 8;

const ROBOT_KEEP_OUT_GAUGES: usize = 6;
const FRONT_ROBOT_SWEEP_CLEARANCE_Y: f64 = 170.0;
const REAR_SERVICE_CLEARANCE_Y: f64 = 130.0;
const TRAP_CARTRIDGE_LIFT_CLEARANCE_Z: f64 = 260.0;
const SENSOR_SERVICE_CLEARANCE_Z: f64 = 305.0;
const PURGE_CASSETTE_PULL_CLEARANCE_X: f64 = 190.0;
const RIGHT_SERVICE_CLEARANCE_X: f64 = 220.0;

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

        self.center.0.abs() + self.x / 2.0 <= usable_x
            && self.center.1.abs() + self.y / 2.0 <= usable_y
    }

    fn overlaps_with_clearance(self, other: Rect, clearance: f64) -> bool {
        let dx = (self.center.0 - other.center.0).abs();
        let dy = (self.center.1 - other.center.1).abs();
        dx < (self.x + other.x) / 2.0 + clearance && dy < (self.y + other.y) / 2.0 + clearance
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DispositionLane {
    Release,
    Hold,
    Reject,
}

impl DispositionLane {
    fn all() -> [DispositionLane; DISPOSITION_LANES] {
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

    fn label(self) -> &'static str {
        match self {
            DispositionLane::Release => "release",
            DispositionLane::Hold => "hold",
            DispositionLane::Reject => "reject",
        }
    }

    fn gate_height(self) -> f64 {
        match self {
            DispositionLane::Release => 18.0,
            DispositionLane::Hold => 32.0,
            DispositionLane::Reject => 48.0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct ThresholdToken {
    name: &'static str,
    threshold_ml: f64,
    token_height: f64,
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let deck = secondary_containment_deck();
    export(OUTPUTS[0], &deck);

    let traps = bubble_trap_cartridge_bank();
    export(OUTPUTS[1], &traps);

    let witnesses = graduated_capacity_witness_chambers();
    export(OUTPUTS[2], &witnesses);

    let slug_manifold = slug_injection_manifold();
    export(OUTPUTS[3], &slug_manifold);

    let optical = optical_bubble_sensor_bridge();
    export(OUTPUTS[4], &optical);

    let taps = pressure_flow_tap_panel();
    export(OUTPUTS[5], &taps);

    let purge = overflow_purge_capture_cassette();
    export(OUTPUTS[6], &purge);

    let alarms = alarm_threshold_token_rail();
    export(OUTPUTS[7], &alarms);

    let diverter = quarantine_diverter_mock_path();
    export(OUTPUTS[8], &diverter);

    let lanes = release_hold_reject_lanes();
    export(OUTPUTS[9], &lanes);

    let evidence = evidence_bridge();
    export(OUTPUTS[10], &evidence);

    let keepouts = robot_service_keepouts();
    export(OUTPUTS[11], &keepouts);

    let assembly = deck
        + traps
        + witnesses
        + slug_manifold
        + optical
        + taps
        + purge
        + alarms
        + diverter
        + lanes
        + evidence
        + keepouts;
    export(OUTPUTS[12], &assembly);

    println!();
    println!("Closed perfusion bubble-trap capacity exhaustion and alarm validation station:");
    println!(
        "  Containment:        {STATION_X:.0}mm x {STATION_Y:.0}mm deck, {TRAP_LANES} closed perfusion lanes, {TRAP_CARTRIDGES} removable trap cartridge positions"
    );
    println!(
        "  Challenge path:     {WITNESS_CHAMBERS} graduated witness chambers ({:.1} mL each), {SLUG_INJECTION_PORTS} slug injection ports, {OVERFLOW_BRANCHES} overflow/purge branches",
        witness_chamber_volume_ml()
    );
    println!(
        "  Alarm coverage:     {OPTICAL_SENSOR_CHANNELS} optical channels, {PRESSURE_TAPS} pressure taps, {FLOW_WITNESS_WINDOWS} flow windows, {ALARM_THRESHOLD_TOKENS} threshold tokens"
    );
    println!(
        "  Disposition:        {DIVERTER_VALVES} diverter valve mockups, {QUARANTINE_BAG_DOCKS} quarantine docks, {DISPOSITION_LANES} release/hold/reject lanes"
    );
    println!(
        "  Evidence/keepouts:  {EVIDENCE_CAMERAS} cameras, {EVIDENCE_LIGHT_SEGMENTS} light segments, {ROBOT_KEEP_OUT_GAUGES} keepout gauges, {} limitation markers, {} STL outputs",
        LIMITATIONS.len(),
        OUTPUTS.len()
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn secondary_containment_deck() -> Part {
    let deck = centered_cube(
        format!("{PREFIX}_secondary_containment_deck_body"),
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let basin = centered_cube(
        format!("{PREFIX}_shallow_capacity_spill_basin_recess"),
        BASIN_X,
        BASIN_Y,
        BASIN_DEPTH,
    )
    .translate(0.0, 0.0, BASE_Z - BASIN_DEPTH / 2.0 + 0.2);
    let drain = centered_cylinder(
        format!("{PREFIX}_front_low_point_purge_drain_bore"),
        9.0,
        66.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 112.0,
        -STATION_Y / 2.0 + 18.0,
        BASE_Z - 6.0,
    );

    deck - basin - drain - mounting_hole_cuts()
        + containment_rim()
        + deck_datum_fiducials()
        + module_socket_outlines()
        + leak_witness_ribs()
        + route_direction_arrows()
}

fn mounting_hole_cuts() -> Part {
    let mut holes = Part::empty(format!("{PREFIX}_mounting_hole_cuts"));
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
                format!("{PREFIX}_m6_deck_mount_clearance_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 6.0,
                28,
            )
            .translate(*x, *y, BASE_Z / 2.0);
    }
    holes
}

fn containment_rim() -> Part {
    let front = centered_cube(
        format!("{PREFIX}_front_low_spill_lip"),
        STATION_X - 160.0,
        14.0,
        26.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 7.0, BASE_Z + 13.0);
    let rear = centered_cube(
        format!("{PREFIX}_rear_secondary_containment_rim"),
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, BASE_Z + RIM_Z / 2.0);
    let left = centered_cube(
        format!("{PREFIX}_left_secondary_containment_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);
    let right = centered_cube(
        format!("{PREFIX}_right_secondary_containment_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);

    front + rear + left + right
}

fn deck_datum_fiducials() -> Part {
    let mut fiducials = Part::empty(format!("{PREFIX}_deck_robot_datum_fiducials"));
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 92.0, STATION_Y / 2.0 - 92.0),
        (STATION_X / 2.0 - 92.0, STATION_Y / 2.0 - 92.0),
        (-STATION_X / 2.0 + 92.0, -STATION_Y / 2.0 + 92.0),
        (STATION_X / 2.0 - 92.0, -STATION_Y / 2.0 + 92.0),
    ]
    .iter()
    .enumerate()
    {
        let disc = centered_cylinder(format!("{PREFIX}_datum_fiducial_outer_{i}"), 15.0, 4.0, 36)
            .translate(*x, *y, BASE_Z + 2.0)
            - centered_cylinder(
                format!("{PREFIX}_datum_fiducial_center_bore_{i}"),
                3.0,
                6.0,
                20,
            )
            .translate(*x, *y, BASE_Z + 2.0);
        fiducials = fiducials + disc;
    }
    fiducials
}

fn module_socket_outlines() -> Part {
    let mut sockets = Part::empty(format!("{PREFIX}_module_socket_outlines"));
    for (i, rect) in layout_rects().iter().enumerate() {
        let x = rect.center.0;
        let y = rect.center.1;
        let frame = centered_cube(
            format!("{PREFIX}_{}_socket_front_back_{i}", rect.name),
            rect.x,
            5.0,
            5.0,
        )
        .translate(x, y - rect.y / 2.0, BASE_Z + 2.5)
            + centered_cube(
                format!("{PREFIX}_{}_socket_rear_{i}", rect.name),
                rect.x,
                5.0,
                5.0,
            )
            .translate(x, y + rect.y / 2.0, BASE_Z + 2.5)
            + centered_cube(
                format!("{PREFIX}_{}_socket_left_{i}", rect.name),
                5.0,
                rect.y,
                5.0,
            )
            .translate(x - rect.x / 2.0, y, BASE_Z + 2.5)
            + centered_cube(
                format!("{PREFIX}_{}_socket_right_{i}", rect.name),
                5.0,
                rect.y,
                5.0,
            )
            .translate(x + rect.x / 2.0, y, BASE_Z + 2.5);
        sockets = sockets + frame;
    }
    sockets
}

fn leak_witness_ribs() -> Part {
    let mut ribs = Part::empty(format!("{PREFIX}_basin_leak_witness_ribs"));
    for (i, y) in [-330.0, -220.0, -110.0, 0.0, 110.0, 220.0, 330.0]
        .iter()
        .enumerate()
    {
        ribs = ribs
            + centered_cube(
                format!("{PREFIX}_basin_flow_to_low_point_rib_{i}"),
                BASIN_X - 120.0,
                6.0,
                5.0,
            )
            .translate(0.0, *y, BASE_Z + 2.5);
    }
    ribs
}

fn route_direction_arrows() -> Part {
    let mut arrows = Part::empty(format!("{PREFIX}_closed_path_route_direction_arrows"));
    for i in 0..TRAP_LANES {
        let x = lane_x(TRAP_POS.0, i);
        let top = centered_cube(
            format!("{PREFIX}_lane_{i}_trap_to_witness_flow_rib"),
            84.0,
            6.0,
            5.0,
        )
        .translate(x + 188.0, 318.0, BASE_Z + 3.0)
            + centered_cube(
                format!("{PREFIX}_lane_{i}_trap_to_witness_arrow_head"),
                13.0,
                13.0,
                5.0,
            )
            .rotate(0.0, 0.0, 45.0)
            .translate(x + 232.0, 318.0, BASE_Z + 3.0);
        let lower = centered_cube(
            format!("{PREFIX}_lane_{i}_sensor_to_purge_flow_rib"),
            88.0,
            6.0,
            5.0,
        )
        .translate(x + 595.0, 48.0, BASE_Z + 3.0)
            + centered_cube(
                format!("{PREFIX}_lane_{i}_sensor_to_purge_arrow_head"),
                13.0,
                13.0,
                5.0,
            )
            .rotate(0.0, 0.0, 45.0)
            .translate(x + 642.0, 48.0, BASE_Z + 3.0);
        arrows = arrows + top + lower;
    }
    arrows
}

fn bubble_trap_cartridge_bank() -> Part {
    let body = module_slab(
        format!("{PREFIX}_bubble_trap_bank_socketed_body"),
        TRAP_POS,
        TRAP_X,
        TRAP_Y,
        TRAP_BANK_Z,
    );

    body - trap_socket_cuts()
        + trap_cartridge_shells()
        + trap_high_point_vent_caps()
        + trap_capacity_exhaustion_bands()
        + trap_bank_latch_rails()
}

fn trap_socket_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_bubble_trap_socket_cuts"));
    for i in 0..TRAP_CARTRIDGES {
        let x = lane_x(TRAP_POS.0, i);
        cuts = cuts
            + centered_cylinder(
                format!("{PREFIX}_bubble_trap_cartridge_socket_bore_{i}"),
                TRAP_COLUMN_RADIUS + 3.0,
                TRAP_BANK_Z + 10.0,
                40,
            )
            .translate(x, TRAP_POS.1, BASE_Z + MODULE_RISER_Z + TRAP_BANK_Z / 2.0);
    }
    cuts
}

fn trap_cartridge_shells() -> Part {
    let mut shells = Part::empty(format!("{PREFIX}_bubble_trap_cartridge_shells"));
    let base_z = BASE_Z + MODULE_RISER_Z + TRAP_BANK_Z;
    for i in 0..TRAP_CARTRIDGES {
        let x = lane_x(TRAP_POS.0, i);
        let outer = centered_cylinder(
            format!("{PREFIX}_removable_bubble_trap_cartridge_outer_{i}"),
            TRAP_COLUMN_RADIUS,
            TRAP_COLUMN_Z,
            48,
        )
        .translate(x, TRAP_POS.1, base_z + TRAP_COLUMN_Z / 2.0);
        let inner = centered_cylinder(
            format!("{PREFIX}_removable_bubble_trap_cartridge_clear_core_{i}"),
            TRAP_COLUMN_RADIUS - 4.0,
            TRAP_COLUMN_Z + 4.0,
            40,
        )
        .translate(x, TRAP_POS.1, base_z + TRAP_COLUMN_Z / 2.0);
        let inlet = centered_cylinder(
            format!("{PREFIX}_trap_lane_{i}_bottom_inlet_bore"),
            TUBE_BORE_D / 2.0,
            38.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, TRAP_POS.1 - 32.0, base_z + 15.0);
        let outlet = centered_cylinder(
            format!("{PREFIX}_trap_lane_{i}_top_outlet_bore"),
            TUBE_BORE_D / 2.0,
            38.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, TRAP_POS.1 + 32.0, base_z + TRAP_COLUMN_Z - 15.0);
        shells = shells + (outer - inner - inlet - outlet);
    }
    shells
}

fn trap_high_point_vent_caps() -> Part {
    let mut vents = Part::empty(format!("{PREFIX}_trap_high_point_vent_caps"));
    let top_z = BASE_Z + MODULE_RISER_Z + TRAP_BANK_Z + TRAP_COLUMN_Z;
    for i in 0..TRAP_HIGH_POINT_VENTS {
        let x = lane_x(TRAP_POS.0, i);
        let cap = centered_cylinder(format!("{PREFIX}_high_point_vent_cap_{i}"), 8.0, 12.0, 28)
            .translate(x, TRAP_POS.1, top_z + 6.0)
            - centered_cylinder(
                format!("{PREFIX}_high_point_vent_pin_bore_{i}"),
                2.2,
                14.0,
                16,
            )
            .translate(x, TRAP_POS.1, top_z + 6.0);
        vents = vents + cap;
    }
    vents
}

fn trap_capacity_exhaustion_bands() -> Part {
    let mut bands = Part::empty(format!("{PREFIX}_trap_capacity_exhaustion_bands"));
    let base_z = BASE_Z + MODULE_RISER_Z + TRAP_BANK_Z + 18.0;
    for i in 0..TRAP_CARTRIDGES {
        let x = lane_x(TRAP_POS.0, i);
        for (j, token) in threshold_tokens().iter().enumerate() {
            bands = bands
                + centered_cube(
                    format!("{PREFIX}_trap_{i}_{}_capacity_band", token.name),
                    23.0,
                    2.6,
                    3.2,
                )
                .translate(
                    x,
                    TRAP_POS.1 - TRAP_COLUMN_RADIUS - 2.4,
                    base_z + j as f64 * 20.0,
                );
        }
    }
    bands
}

fn trap_bank_latch_rails() -> Part {
    let front = centered_cube(
        format!("{PREFIX}_trap_bank_front_cartridge_latch_rail"),
        TRAP_X - 34.0,
        10.0,
        18.0,
    )
    .translate(
        TRAP_POS.0,
        TRAP_POS.1 - TRAP_Y / 2.0 + 24.0,
        BASE_Z + MODULE_RISER_Z + TRAP_BANK_Z + 9.0,
    );
    let rear = centered_cube(
        format!("{PREFIX}_trap_bank_rear_cartridge_latch_rail"),
        TRAP_X - 34.0,
        10.0,
        18.0,
    )
    .translate(
        TRAP_POS.0,
        TRAP_POS.1 + TRAP_Y / 2.0 - 24.0,
        BASE_Z + MODULE_RISER_Z + TRAP_BANK_Z + 9.0,
    );
    let mut latch_tabs = Part::empty(format!("{PREFIX}_trap_bank_latch_tabs"));
    for i in 0..TRAP_LATCH_COUNT {
        let cartridge = i / 2;
        let side = if i % 2 == 0 { -1.0 } else { 1.0 };
        latch_tabs = latch_tabs
            + centered_cube(
                format!("{PREFIX}_trap_cartridge_latch_tab_{i}"),
                18.0,
                12.0,
                12.0,
            )
            .translate(
                lane_x(TRAP_POS.0, cartridge),
                TRAP_POS.1 + side * (TRAP_Y / 2.0 - 24.0),
                BASE_Z + MODULE_RISER_Z + TRAP_BANK_Z + 24.0,
            );
    }
    front + rear + latch_tabs
}

fn graduated_capacity_witness_chambers() -> Part {
    let body = module_slab(
        format!("{PREFIX}_graduated_witness_chamber_body"),
        WITNESS_POS,
        WITNESS_X,
        WITNESS_Y,
        WITNESS_Z,
    );

    body - witness_chamber_socket_cuts()
        + witness_chamber_shells()
        + witness_graduation_ticks()
        + witness_overfill_sightline()
}

fn witness_chamber_socket_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_witness_chamber_socket_cuts"));
    for i in 0..WITNESS_CHAMBERS {
        let x = lane_x(WITNESS_POS.0, i);
        cuts = cuts
            + centered_cylinder(
                format!("{PREFIX}_graduated_witness_chamber_socket_{i}"),
                WITNESS_CHAMBER_RADIUS + 3.5,
                WITNESS_Z + 8.0,
                40,
            )
            .translate(x, WITNESS_POS.1, BASE_Z + MODULE_RISER_Z + WITNESS_Z / 2.0);
    }
    cuts
}

fn witness_chamber_shells() -> Part {
    let mut chambers = Part::empty(format!("{PREFIX}_graduated_witness_chamber_shells"));
    let bottom_z = BASE_Z + MODULE_RISER_Z + WITNESS_Z;
    for i in 0..WITNESS_CHAMBERS {
        let x = lane_x(WITNESS_POS.0, i);
        let shell = centered_cylinder(
            format!("{PREFIX}_graduated_capacity_witness_chamber_outer_{i}"),
            WITNESS_CHAMBER_RADIUS,
            WITNESS_CHAMBER_Z,
            44,
        )
        .translate(x, WITNESS_POS.1, bottom_z + WITNESS_CHAMBER_Z / 2.0)
            - centered_cylinder(
                format!("{PREFIX}_graduated_capacity_witness_chamber_clear_bore_{i}"),
                WITNESS_CHAMBER_RADIUS - 3.0,
                WITNESS_CHAMBER_Z + 4.0,
                36,
            )
            .translate(x, WITNESS_POS.1, bottom_z + WITNESS_CHAMBER_Z / 2.0);
        let overflow_notch = centered_cube(
            format!("{PREFIX}_witness_chamber_{i}_overflow_notch"),
            7.0,
            8.0,
            5.0,
        )
        .translate(
            x,
            WITNESS_POS.1 + WITNESS_CHAMBER_RADIUS,
            bottom_z + WITNESS_CHAMBER_Z - 7.0,
        );
        chambers = chambers + shell + overflow_notch;
    }
    chambers
}

fn witness_graduation_ticks() -> Part {
    let mut ticks = Part::empty(format!("{PREFIX}_graduated_witness_tick_marks"));
    let bottom_z = BASE_Z + MODULE_RISER_Z + WITNESS_Z + 7.0;
    for lane in 0..WITNESS_CHAMBERS {
        let x = lane_x(WITNESS_POS.0, lane);
        for tick in 0..WITNESS_GRADUATION_TICKS {
            let major = tick % 3 == 0;
            let width = if major { 18.0 } else { 11.0 };
            ticks = ticks
                + centered_cube(
                    format!("{PREFIX}_witness_chamber_{lane}_graduation_tick_{tick}"),
                    width,
                    2.4,
                    2.4,
                )
                .translate(
                    x + WITNESS_CHAMBER_RADIUS + width / 2.0 + 3.0,
                    WITNESS_POS.1,
                    bottom_z + tick as f64 * 6.0,
                );
        }
    }
    ticks
}

fn witness_overfill_sightline() -> Part {
    centered_cube(
        format!("{PREFIX}_shared_overfill_sightline_bar"),
        WITNESS_X - 52.0,
        6.0,
        8.0,
    )
    .translate(
        WITNESS_POS.0,
        WITNESS_POS.1 + WITNESS_Y / 2.0 - 28.0,
        BASE_Z + MODULE_RISER_Z + WITNESS_Z + WITNESS_CHAMBER_Z - 8.0,
    )
}

fn slug_injection_manifold() -> Part {
    let body = module_slab(
        format!("{PREFIX}_slug_injection_manifold_body"),
        SLUG_POS,
        SLUG_X,
        SLUG_Y,
        SLUG_Z,
    );

    body - slug_manifold_bores()
        + slug_injection_port_collars()
        + slug_metering_wells()
        + slug_reference_token_bank()
        + manifold_check_valve_markers()
}

fn slug_manifold_bores() -> Part {
    let mut bores = Part::empty(format!("{PREFIX}_slug_manifold_flow_bores"));
    for i in 0..TRAP_LANES {
        let x = lane_x(SLUG_POS.0, i);
        let through_bore = centered_cylinder(
            format!("{PREFIX}_slug_lane_{i}_closed_path_bore"),
            TUBE_BORE_D / 2.0,
            SLUG_Y + 24.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, SLUG_POS.1, BASE_Z + MODULE_RISER_Z + SLUG_Z / 2.0);
        let injection_bore = centered_cylinder(
            format!("{PREFIX}_slug_lane_{i}_vertical_injection_bore"),
            TUBE_BORE_D / 2.0,
            SLUG_Z + 20.0,
            24,
        )
        .translate(x, SLUG_POS.1 - 42.0, BASE_Z + MODULE_RISER_Z + SLUG_Z / 2.0);
        let purge_bore = centered_cylinder(
            format!("{PREFIX}_slug_lane_{i}_vertical_purge_bore"),
            TUBE_BORE_D / 2.0,
            SLUG_Z + 20.0,
            24,
        )
        .translate(x, SLUG_POS.1 + 42.0, BASE_Z + MODULE_RISER_Z + SLUG_Z / 2.0);
        bores = bores + through_bore + injection_bore + purge_bore;
    }
    bores
}

fn slug_injection_port_collars() -> Part {
    let mut collars = Part::empty(format!("{PREFIX}_slug_injection_port_collars"));
    for i in 0..TRAP_LANES {
        let x = lane_x(SLUG_POS.0, i);
        for (j, y_offset) in [-42.0, 42.0].iter().enumerate() {
            let collar = centered_cylinder(
                format!("{PREFIX}_slug_lane_{i}_port_{j}_luer_collar_outer"),
                11.0,
                10.0,
                28,
            )
            .translate(
                x,
                SLUG_POS.1 + y_offset,
                BASE_Z + MODULE_RISER_Z + SLUG_Z + 5.0,
            ) - centered_cylinder(
                format!("{PREFIX}_slug_lane_{i}_port_{j}_luer_collar_inner"),
                TUBE_BORE_D / 2.0,
                12.0,
                24,
            )
            .translate(
                x,
                SLUG_POS.1 + y_offset,
                BASE_Z + MODULE_RISER_Z + SLUG_Z + 5.0,
            );
            collars = collars + collar;
        }
    }
    collars
}

fn slug_metering_wells() -> Part {
    let mut wells = Part::empty(format!("{PREFIX}_slug_metering_wells"));
    for i in 0..SLUG_METERING_WELLS {
        let x = lane_x(SLUG_POS.0, i);
        let well = centered_cylinder(
            format!("{PREFIX}_slug_lane_{i}_metering_well_outer"),
            13.0,
            8.0,
            30,
        )
        .translate(x, SLUG_POS.1 - 78.0, BASE_Z + MODULE_RISER_Z + SLUG_Z + 4.0)
            - centered_cylinder(
                format!("{PREFIX}_slug_lane_{i}_metering_well_pocket"),
                8.0,
                10.0,
                26,
            )
            .translate(x, SLUG_POS.1 - 78.0, BASE_Z + MODULE_RISER_Z + SLUG_Z + 4.0);
        wells = wells + well;
    }
    wells
}

fn slug_reference_token_bank() -> Part {
    let mut tokens = Part::empty(format!("{PREFIX}_slug_reference_token_bank"));
    for i in 0..SLUG_REFERENCE_TOKENS {
        let x = lane_x(SLUG_POS.0, i);
        tokens = tokens
            + centered_cube(
                format!("{PREFIX}_slug_reference_air_volume_token_{i}"),
                24.0,
                18.0,
                7.0,
            )
            .translate(
                x,
                SLUG_POS.1 + SLUG_Y / 2.0 - 26.0,
                BASE_Z + MODULE_RISER_Z + SLUG_Z + 3.5 + i as f64 * 0.35,
            );
    }
    tokens
}

fn manifold_check_valve_markers() -> Part {
    let mut markers = Part::empty(format!("{PREFIX}_slug_manifold_check_valve_markers"));
    for i in 0..TRAP_LANES {
        let x = lane_x(SLUG_POS.0, i);
        let marker = centered_cube(
            format!("{PREFIX}_slug_lane_{i}_check_valve_body"),
            26.0,
            9.0,
            8.0,
        )
        .translate(x, SLUG_POS.1, BASE_Z + MODULE_RISER_Z + SLUG_Z + 5.0)
            + centered_cube(
                format!("{PREFIX}_slug_lane_{i}_check_valve_arrow_head"),
                9.0,
                9.0,
                8.0,
            )
            .rotate(0.0, 0.0, 45.0)
            .translate(x, SLUG_POS.1 + 16.0, BASE_Z + MODULE_RISER_Z + SLUG_Z + 5.0);
        markers = markers + marker;
    }
    markers
}

fn optical_bubble_sensor_bridge() -> Part {
    let deck = module_slab(
        format!("{PREFIX}_optical_bubble_sensor_bridge_base"),
        OPTICAL_POS,
        OPTICAL_X,
        OPTICAL_Y,
        28.0,
    );
    let bridge = optical_bridge_posts_and_beam();

    deck - optical_channel_bores() + bridge + optical_sensor_forks() + optical_reference_coupons()
}

fn optical_channel_bores() -> Part {
    let mut bores = Part::empty(format!("{PREFIX}_optical_sensor_channel_bores"));
    for i in 0..OPTICAL_SENSOR_CHANNELS {
        let x = lane_x(OPTICAL_POS.0, i);
        bores = bores
            + centered_cylinder(
                format!("{PREFIX}_optical_lane_{i}_transparent_tube_bore"),
                OPTICAL_WINDOW_D / 2.0,
                OPTICAL_FORK_SPAN_Y + 28.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, OPTICAL_POS.1, BASE_Z + MODULE_RISER_Z + 45.0);
    }
    bores
}

fn optical_bridge_posts_and_beam() -> Part {
    let post_z = OPTICAL_Z;
    let left_post = centered_cube(
        format!("{PREFIX}_optical_bridge_left_service_post"),
        18.0,
        OPTICAL_Y - 36.0,
        post_z,
    )
    .translate(
        OPTICAL_POS.0 - OPTICAL_X / 2.0 + 34.0,
        OPTICAL_POS.1,
        BASE_Z + MODULE_RISER_Z + post_z / 2.0,
    );
    let right_post = centered_cube(
        format!("{PREFIX}_optical_bridge_right_service_post"),
        18.0,
        OPTICAL_Y - 36.0,
        post_z,
    )
    .translate(
        OPTICAL_POS.0 + OPTICAL_X / 2.0 - 34.0,
        OPTICAL_POS.1,
        BASE_Z + MODULE_RISER_Z + post_z / 2.0,
    );
    let beam = centered_cube(
        format!("{PREFIX}_optical_bridge_sensor_bus_beam"),
        OPTICAL_X - 70.0,
        20.0,
        24.0,
    )
    .translate(
        OPTICAL_POS.0,
        OPTICAL_POS.1,
        BASE_Z + MODULE_RISER_Z + post_z + 12.0,
    );
    left_post + right_post + beam
}

fn optical_sensor_forks() -> Part {
    let mut forks = Part::empty(format!("{PREFIX}_optical_sensor_forks"));
    for i in 0..OPTICAL_SENSOR_CHANNELS {
        let x = lane_x(OPTICAL_POS.0, i);
        let emitter = centered_cube(
            format!("{PREFIX}_optical_lane_{i}_emitter_fork"),
            12.0,
            20.0,
            50.0,
        )
        .translate(x, OPTICAL_POS.1 - 34.0, BASE_Z + MODULE_RISER_Z + 57.0);
        let receiver = centered_cube(
            format!("{PREFIX}_optical_lane_{i}_receiver_fork"),
            12.0,
            20.0,
            50.0,
        )
        .translate(x, OPTICAL_POS.1 + 34.0, BASE_Z + MODULE_RISER_Z + 57.0);
        let window = centered_cylinder(
            format!("{PREFIX}_optical_lane_{i}_beam_window"),
            6.0,
            36.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, OPTICAL_POS.1, BASE_Z + MODULE_RISER_Z + 58.0);
        forks = forks + emitter + receiver - window;
    }
    forks
}

fn optical_reference_coupons() -> Part {
    let clean = centered_cube(
        format!("{PREFIX}_optical_clean_window_reference_coupon"),
        92.0,
        18.0,
        8.0,
    )
    .translate(
        OPTICAL_POS.0 - 88.0,
        OPTICAL_POS.1 - OPTICAL_Y / 2.0 + 26.0,
        BASE_Z + MODULE_RISER_Z + 36.0,
    );
    let fouled = centered_cube(
        format!("{PREFIX}_optical_fouled_window_reference_coupon"),
        92.0,
        18.0,
        8.0,
    )
    .translate(
        OPTICAL_POS.0 + 88.0,
        OPTICAL_POS.1 - OPTICAL_Y / 2.0 + 26.0,
        BASE_Z + MODULE_RISER_Z + 36.0,
    );
    clean + fouled
}

fn pressure_flow_tap_panel() -> Part {
    let body = module_slab(
        format!("{PREFIX}_pressure_flow_tap_panel_body"),
        TAP_POS,
        TAP_X,
        TAP_Y,
        TAP_Z,
    );

    body - pressure_tap_bores()
        + pressure_tap_collars()
        + flow_witness_windows()
        + sensor_id_lands()
}

fn pressure_tap_bores() -> Part {
    let mut bores = Part::empty(format!("{PREFIX}_pressure_tap_bores"));
    for i in 0..TRAP_LANES {
        let x = lane_x(TAP_POS.0, i);
        for (j, y_offset) in [-34.0, 34.0].iter().enumerate() {
            bores = bores
                + centered_cylinder(
                    format!("{PREFIX}_pressure_lane_{i}_tap_{j}_vertical_bore"),
                    TAP_POST_D / 2.0,
                    TAP_Z + 16.0,
                    24,
                )
                .translate(
                    x,
                    TAP_POS.1 + y_offset,
                    BASE_Z + MODULE_RISER_Z + TAP_Z / 2.0,
                );
        }
    }
    bores
}

fn pressure_tap_collars() -> Part {
    let mut collars = Part::empty(format!("{PREFIX}_pressure_tap_collars"));
    for i in 0..TRAP_LANES {
        let x = lane_x(TAP_POS.0, i);
        for (j, y_offset) in [-34.0, 34.0].iter().enumerate() {
            let collar = centered_cylinder(
                format!("{PREFIX}_pressure_lane_{i}_tap_{j}_collar_outer"),
                10.5,
                9.0,
                28,
            )
            .translate(
                x,
                TAP_POS.1 + y_offset,
                BASE_Z + MODULE_RISER_Z + TAP_Z + 4.5,
            ) - centered_cylinder(
                format!("{PREFIX}_pressure_lane_{i}_tap_{j}_collar_inner"),
                TAP_POST_D / 2.0,
                11.0,
                24,
            )
            .translate(
                x,
                TAP_POS.1 + y_offset,
                BASE_Z + MODULE_RISER_Z + TAP_Z + 4.5,
            );
            collars = collars + collar;
        }
    }
    collars
}

fn flow_witness_windows() -> Part {
    let mut windows = Part::empty(format!("{PREFIX}_flow_witness_windows"));
    for i in 0..FLOW_WITNESS_WINDOWS {
        let x = lane_x(TAP_POS.0, i);
        let frame = centered_cube(
            format!("{PREFIX}_flow_lane_{i}_witness_window_frame"),
            32.0,
            20.0,
            10.0,
        )
        .translate(x, TAP_POS.1, BASE_Z + MODULE_RISER_Z + TAP_Z + 5.0)
            - centered_cube(
                format!("{PREFIX}_flow_lane_{i}_witness_window_clear"),
                22.0,
                12.0,
                12.0,
            )
            .translate(x, TAP_POS.1, BASE_Z + MODULE_RISER_Z + TAP_Z + 5.0);
        windows = windows + frame;
    }
    windows
}

fn sensor_id_lands() -> Part {
    let mut lands = Part::empty(format!("{PREFIX}_pressure_flow_sensor_id_lands"));
    for i in 0..TRAP_LANES {
        let x = lane_x(TAP_POS.0, i);
        lands = lands
            + centered_cube(
                format!("{PREFIX}_pressure_flow_lane_{i}_barcode_land"),
                32.0,
                12.0,
                4.0,
            )
            .translate(
                x,
                TAP_POS.1 + TAP_Y / 2.0 - 22.0,
                BASE_Z + MODULE_RISER_Z + TAP_Z + 2.0,
            );
    }
    lands
}

fn overflow_purge_capture_cassette() -> Part {
    let body = module_slab(
        format!("{PREFIX}_overflow_purge_capture_cassette_body"),
        PURGE_POS,
        PURGE_X,
        PURGE_Y,
        PURGE_Z,
    );

    body - purge_cup_socket_cuts()
        + purge_capture_cups()
        + overflow_header()
        + cassette_latch_flags()
}

fn purge_cup_socket_cuts() -> Part {
    let mut cuts = Part::empty(format!("{PREFIX}_purge_capture_cup_socket_cuts"));
    for i in 0..PURGE_CUPS {
        let x = lane_x(PURGE_POS.0, i);
        cuts = cuts
            + centered_cylinder(
                format!("{PREFIX}_purge_capture_cup_socket_{i}"),
                PURGE_CUP_RADIUS + 3.0,
                PURGE_Z + 8.0,
                40,
            )
            .translate(x, PURGE_POS.1, BASE_Z + MODULE_RISER_Z + PURGE_Z / 2.0);
    }
    cuts
}

fn purge_capture_cups() -> Part {
    let mut cups = Part::empty(format!("{PREFIX}_purge_capture_cups"));
    let bottom_z = BASE_Z + MODULE_RISER_Z + PURGE_Z;
    for i in 0..PURGE_CUPS {
        let x = lane_x(PURGE_POS.0, i);
        let cup = centered_cylinder(
            format!("{PREFIX}_overflow_purge_capture_cup_outer_{i}"),
            PURGE_CUP_RADIUS,
            PURGE_CUP_Z,
            42,
        )
        .translate(x, PURGE_POS.1, bottom_z + PURGE_CUP_Z / 2.0)
            - centered_cylinder(
                format!("{PREFIX}_overflow_purge_capture_cup_cavity_{i}"),
                PURGE_CUP_RADIUS - 3.2,
                PURGE_CUP_Z - 5.0,
                36,
            )
            .translate(x, PURGE_POS.1, bottom_z + PURGE_CUP_Z / 2.0 + 3.0);
        cups = cups + cup;
    }
    cups
}

fn overflow_header() -> Part {
    let header = centered_cube(
        format!("{PREFIX}_overflow_purge_common_header"),
        PURGE_X - 58.0,
        14.0,
        16.0,
    )
    .translate(
        PURGE_POS.0,
        PURGE_POS.1 + PURGE_Y / 2.0 - 30.0,
        BASE_Z + MODULE_RISER_Z + PURGE_Z + 8.0,
    );
    let mut branches = Part::empty(format!("{PREFIX}_overflow_purge_branch_lines"));
    for i in 0..OVERFLOW_BRANCHES {
        let lane = i / 2;
        let x = lane_x(PURGE_POS.0, lane);
        let y = if i % 2 == 0 { -28.0 } else { 28.0 };
        branches = branches
            + centered_cube(
                format!("{PREFIX}_overflow_branch_{i}_route_guard"),
                6.0,
                76.0,
                7.0,
            )
            .translate(x, PURGE_POS.1 + y, BASE_Z + MODULE_RISER_Z + PURGE_Z + 8.0);
    }
    header + branches
}

fn cassette_latch_flags() -> Part {
    let left = centered_cube(
        format!("{PREFIX}_purge_cassette_left_pull_tab"),
        18.0,
        58.0,
        18.0,
    )
    .translate(
        PURGE_POS.0 - PURGE_X / 2.0 + 22.0,
        PURGE_POS.1,
        BASE_Z + MODULE_RISER_Z + PURGE_Z + 9.0,
    );
    let right = centered_cube(
        format!("{PREFIX}_purge_cassette_right_pull_tab"),
        18.0,
        58.0,
        18.0,
    )
    .translate(
        PURGE_POS.0 + PURGE_X / 2.0 - 22.0,
        PURGE_POS.1,
        BASE_Z + MODULE_RISER_Z + PURGE_Z + 9.0,
    );
    left + right
}

fn alarm_threshold_token_rail() -> Part {
    let body = module_slab(
        format!("{PREFIX}_alarm_threshold_token_rail_body"),
        ALARM_POS,
        ALARM_X,
        ALARM_Y,
        ALARM_Z,
    );

    body + alarm_token_slots() + alarm_threshold_tokens() + alarm_bus_rails()
}

fn alarm_token_slots() -> Part {
    let mut slots = Part::empty(format!("{PREFIX}_alarm_threshold_token_slots"));
    for lane in 0..TRAP_LANES {
        let x = lane_x(ALARM_POS.0, lane);
        for level in 0..ALARM_LEVELS {
            let y = ALARM_POS.1 + centered_index(level, ALARM_LEVELS, 38.0);
            slots = slots
                + centered_cube(
                    format!("{PREFIX}_alarm_lane_{lane}_threshold_slot_{level}"),
                    34.0,
                    20.0,
                    5.0,
                )
                .translate(x, y, BASE_Z + MODULE_RISER_Z + ALARM_Z + 2.5);
        }
    }
    slots
}

fn alarm_threshold_tokens() -> Part {
    let mut tokens = Part::empty(format!("{PREFIX}_alarm_threshold_tokens"));
    for lane in 0..TRAP_LANES {
        let x = lane_x(ALARM_POS.0, lane);
        for (level, token) in threshold_tokens().iter().enumerate() {
            let y = ALARM_POS.1 + centered_index(level, ALARM_LEVELS, 38.0);
            let token_width = 18.0 + token.threshold_ml / 2.0;
            tokens = tokens
                + centered_cube(
                    format!("{PREFIX}_alarm_lane_{lane}_{}_token", token.name),
                    token_width,
                    12.0,
                    token.token_height,
                )
                .translate(
                    x,
                    y,
                    BASE_Z + MODULE_RISER_Z + ALARM_Z + 5.0 + token.token_height / 2.0,
                );
        }
    }
    tokens
}

fn alarm_bus_rails() -> Part {
    let mut rails = Part::empty(format!("{PREFIX}_alarm_bus_rails"));
    for level in 0..ALARM_LEVELS {
        let y = ALARM_POS.1 + centered_index(level, ALARM_LEVELS, 38.0);
        rails = rails
            + centered_cube(
                format!("{PREFIX}_alarm_threshold_bus_rail_{level}"),
                ALARM_X - 48.0,
                4.0,
                7.0,
            )
            .translate(ALARM_POS.0, y, BASE_Z + MODULE_RISER_Z + ALARM_Z + 3.5);
    }
    rails
}

fn quarantine_diverter_mock_path() -> Part {
    let body = module_slab(
        format!("{PREFIX}_quarantine_diverter_mock_path_body"),
        DIVERTER_POS,
        DIVERTER_X,
        DIVERTER_Y,
        DIVERTER_Z,
    );

    body - diverter_path_bores()
        + diverter_valve_knobs()
        + quarantine_route_guards()
        + quarantine_bag_docks()
}

fn diverter_path_bores() -> Part {
    let mut bores = Part::empty(format!("{PREFIX}_quarantine_diverter_path_bores"));
    for i in 0..DIVERTER_VALVES {
        let x = lane_x(DIVERTER_POS.0, i);
        let main = centered_cylinder(
            format!("{PREFIX}_diverter_lane_{i}_main_path_bore"),
            TUBE_BORE_D / 2.0,
            DIVERTER_Y + 18.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            x,
            DIVERTER_POS.1,
            BASE_Z + MODULE_RISER_Z + DIVERTER_Z / 2.0,
        );
        let quarantine = centered_cylinder(
            format!("{PREFIX}_diverter_lane_{i}_quarantine_branch_bore"),
            TUBE_BORE_D / 2.0,
            72.0,
            24,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(
            x + 18.0,
            DIVERTER_POS.1 - 30.0,
            BASE_Z + MODULE_RISER_Z + DIVERTER_Z / 2.0,
        );
        bores = bores + main + quarantine;
    }
    bores
}

fn diverter_valve_knobs() -> Part {
    let mut knobs = Part::empty(format!("{PREFIX}_quarantine_diverter_valve_knobs"));
    for i in 0..DIVERTER_VALVES {
        let x = lane_x(DIVERTER_POS.0, i);
        let knob = centered_cylinder(
            format!("{PREFIX}_diverter_lane_{i}_valve_knob"),
            14.0,
            12.0,
            32,
        )
        .translate(
            x,
            DIVERTER_POS.1,
            BASE_Z + MODULE_RISER_Z + DIVERTER_Z + 6.0,
        ) + centered_cube(
            format!("{PREFIX}_diverter_lane_{i}_position_pointer"),
            28.0,
            5.0,
            5.0,
        )
        .rotate(0.0, 0.0, if i % 2 == 0 { 30.0 } else { -30.0 })
        .translate(
            x,
            DIVERTER_POS.1,
            BASE_Z + MODULE_RISER_Z + DIVERTER_Z + 15.0,
        );
        knobs = knobs + knob;
    }
    knobs
}

fn quarantine_route_guards() -> Part {
    let guard = centered_cube(
        format!("{PREFIX}_quarantine_route_guard_to_reject_lane"),
        DIVERTER_X - 54.0,
        8.0,
        10.0,
    )
    .translate(
        DIVERTER_POS.0,
        DIVERTER_POS.1 - DIVERTER_Y / 2.0 + 28.0,
        BASE_Z + MODULE_RISER_Z + DIVERTER_Z + 5.0,
    );
    let return_guard = centered_cube(
        format!("{PREFIX}_mock_release_route_guard"),
        DIVERTER_X - 54.0,
        8.0,
        10.0,
    )
    .translate(
        DIVERTER_POS.0,
        DIVERTER_POS.1 + DIVERTER_Y / 2.0 - 28.0,
        BASE_Z + MODULE_RISER_Z + DIVERTER_Z + 5.0,
    );
    guard + return_guard
}

fn quarantine_bag_docks() -> Part {
    let mut docks = Part::empty(format!("{PREFIX}_quarantine_bag_docks"));
    for i in 0..QUARANTINE_BAG_DOCKS {
        let x = DIVERTER_POS.0 + centered_index(i, QUARANTINE_BAG_DOCKS, 118.0);
        docks = docks
            + centered_cube(
                format!("{PREFIX}_quarantine_retain_bag_dock_{i}"),
                86.0,
                34.0,
                18.0,
            )
            .translate(
                x,
                DIVERTER_POS.1 - DIVERTER_Y / 2.0 + 42.0,
                BASE_Z + MODULE_RISER_Z + DIVERTER_Z + 9.0,
            );
    }
    docks
}

fn release_hold_reject_lanes() -> Part {
    let body = module_slab(
        format!("{PREFIX}_release_hold_reject_lane_body"),
        LANES_POS,
        LANES_X,
        LANES_Y,
        LANES_Z,
    );

    body + disposition_lane_trays() + disposition_token_wells() + disposition_gate_posts()
}

fn disposition_lane_trays() -> Part {
    let mut trays = Part::empty(format!("{PREFIX}_disposition_lane_trays"));
    for lane in DispositionLane::all() {
        let x = disposition_lane_x(lane);
        trays = trays
            + centered_cube(
                format!("{PREFIX}_{}_lane_tray", lane.label()),
                104.0,
                LANES_Y - 42.0,
                12.0,
            )
            .translate(x, LANES_POS.1, BASE_Z + MODULE_RISER_Z + LANES_Z + 6.0);
    }
    trays
}

fn disposition_token_wells() -> Part {
    let mut wells = Part::empty(format!("{PREFIX}_disposition_token_wells"));
    for lane in DispositionLane::all() {
        let x = disposition_lane_x(lane);
        for token in 0..TOKENS_PER_DISPOSITION_LANE {
            let y = LANES_POS.1 + centered_index(token, TOKENS_PER_DISPOSITION_LANE, 26.0);
            let well = centered_cylinder(
                format!("{PREFIX}_{}_lane_token_well_{token}", lane.label()),
                9.0,
                6.0,
                24,
            )
            .translate(x, y, BASE_Z + MODULE_RISER_Z + LANES_Z + 15.0);
            wells = wells + well;
        }
    }
    wells
}

fn disposition_gate_posts() -> Part {
    let mut gates = Part::empty(format!("{PREFIX}_disposition_gate_posts"));
    for lane in DispositionLane::all() {
        let x = disposition_lane_x(lane);
        let z = lane.gate_height();
        let gate = centered_cube(
            format!("{PREFIX}_{}_lane_front_gate", lane.label()),
            104.0,
            8.0,
            z,
        )
        .translate(
            x,
            LANES_POS.1 - LANES_Y / 2.0 + 22.0,
            BASE_Z + MODULE_RISER_Z + LANES_Z + z / 2.0,
        );
        gates = gates + gate;
    }
    gates
}

fn evidence_bridge() -> Part {
    let left_post = centered_cube(
        format!("{PREFIX}_evidence_bridge_left_post"),
        26.0,
        38.0,
        EVIDENCE_UNDERSIDE_Z,
    )
    .translate(
        EVIDENCE_POS.0 - EVIDENCE_X / 2.0 + 38.0,
        EVIDENCE_POS.1,
        BASE_Z + EVIDENCE_UNDERSIDE_Z / 2.0,
    );
    let right_post = centered_cube(
        format!("{PREFIX}_evidence_bridge_right_post"),
        26.0,
        38.0,
        EVIDENCE_UNDERSIDE_Z,
    )
    .translate(
        EVIDENCE_POS.0 + EVIDENCE_X / 2.0 - 38.0,
        EVIDENCE_POS.1,
        BASE_Z + EVIDENCE_UNDERSIDE_Z / 2.0,
    );
    let beam = centered_cube(
        format!("{PREFIX}_evidence_bridge_camera_light_beam"),
        EVIDENCE_X,
        EVIDENCE_Y,
        EVIDENCE_BEAM_Z,
    )
    .translate(
        EVIDENCE_POS.0,
        EVIDENCE_POS.1,
        BASE_Z + EVIDENCE_UNDERSIDE_Z + EVIDENCE_BEAM_Z / 2.0,
    );

    left_post
        + right_post
        + beam
        + evidence_camera_pods()
        + evidence_light_segments()
        + evidence_card_lands()
}

fn evidence_camera_pods() -> Part {
    let mut pods = Part::empty(format!("{PREFIX}_evidence_camera_pods"));
    for i in 0..EVIDENCE_CAMERAS {
        let x = EVIDENCE_POS.0 + centered_index(i, EVIDENCE_CAMERAS, 250.0);
        let pod = centered_cube(
            format!("{PREFIX}_evidence_camera_pod_{i}"),
            48.0,
            34.0,
            26.0,
        )
        .translate(x, EVIDENCE_POS.1, BASE_Z + EVIDENCE_UNDERSIDE_Z - 13.0)
            - centered_cylinder(
                format!("{PREFIX}_evidence_camera_lens_bore_{i}"),
                9.0,
                28.0,
                28,
            )
            .translate(x, EVIDENCE_POS.1, BASE_Z + EVIDENCE_UNDERSIDE_Z - 13.0);
        pods = pods + pod;
    }
    pods
}

fn evidence_light_segments() -> Part {
    let mut lights = Part::empty(format!("{PREFIX}_evidence_light_segments"));
    for i in 0..EVIDENCE_LIGHT_SEGMENTS {
        let x = EVIDENCE_POS.0 + centered_index(i, EVIDENCE_LIGHT_SEGMENTS, 132.0);
        lights = lights
            + centered_cube(
                format!("{PREFIX}_evidence_diffuse_light_segment_{i}"),
                76.0,
                10.0,
                8.0,
            )
            .translate(
                x,
                EVIDENCE_POS.1 - 22.0,
                BASE_Z + EVIDENCE_UNDERSIDE_Z - 4.0,
            );
    }
    lights
}

fn evidence_card_lands() -> Part {
    let barcode = centered_cube(
        format!("{PREFIX}_evidence_run_barcode_land"),
        180.0,
        18.0,
        6.0,
    )
    .translate(
        EVIDENCE_POS.0 - 360.0,
        EVIDENCE_POS.1 + 22.0,
        BASE_Z + EVIDENCE_UNDERSIDE_Z + EVIDENCE_BEAM_Z + 3.0,
    );
    let photo_scale = centered_cube(
        format!("{PREFIX}_evidence_photo_scale_reference_land"),
        180.0,
        18.0,
        6.0,
    )
    .translate(
        EVIDENCE_POS.0 + 360.0,
        EVIDENCE_POS.1 + 22.0,
        BASE_Z + EVIDENCE_UNDERSIDE_Z + EVIDENCE_BEAM_Z + 3.0,
    );
    barcode + photo_scale
}

fn robot_service_keepouts() -> Part {
    let front_robot = centered_cube(
        format!("{PREFIX}_front_robot_sweep_keepout_gauge"),
        STATION_X - 170.0,
        12.0,
        12.0,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 + FRONT_ROBOT_SWEEP_CLEARANCE_Y,
        BASE_Z + 6.0,
    );
    let rear_service = centered_cube(
        format!("{PREFIX}_rear_service_access_keepout_gauge"),
        STATION_X - 170.0,
        12.0,
        12.0,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - REAR_SERVICE_CLEARANCE_Y,
        BASE_Z + 6.0,
    );
    let trap_lift = centered_cube(
        format!("{PREFIX}_trap_cartridge_lift_clearance_mast"),
        22.0,
        22.0,
        TRAP_CARTRIDGE_LIFT_CLEARANCE_Z,
    )
    .translate(
        TRAP_POS.0 - TRAP_X / 2.0 + 30.0,
        TRAP_POS.1,
        BASE_Z + TRAP_CARTRIDGE_LIFT_CLEARANCE_Z / 2.0,
    );
    let sensor_service = centered_cube(
        format!("{PREFIX}_optical_sensor_top_service_clearance_mast"),
        22.0,
        22.0,
        SENSOR_SERVICE_CLEARANCE_Z,
    )
    .translate(
        OPTICAL_POS.0 + OPTICAL_X / 2.0 - 30.0,
        OPTICAL_POS.1,
        BASE_Z + SENSOR_SERVICE_CLEARANCE_Z / 2.0,
    );
    let purge_pull = centered_cube(
        format!("{PREFIX}_purge_cassette_pullout_keepout_gauge"),
        PURGE_CASSETTE_PULL_CLEARANCE_X,
        16.0,
        12.0,
    )
    .translate(
        PURGE_POS.0 + PURGE_X / 2.0 + PURGE_CASSETTE_PULL_CLEARANCE_X / 2.0,
        PURGE_POS.1,
        BASE_Z + 6.0,
    );
    let right_service = centered_cube(
        format!("{PREFIX}_right_service_cable_dress_keepout_gauge"),
        16.0,
        STATION_Y - 260.0,
        12.0,
    )
    .translate(
        STATION_X / 2.0 - RIGHT_SERVICE_CLEARANCE_X,
        0.0,
        BASE_Z + 6.0,
    );

    front_robot
        + rear_service
        + trap_lift
        + sensor_service
        + purge_pull
        + right_service
        + keepout_flag_posts()
}

fn keepout_flag_posts() -> Part {
    let mut posts = Part::empty(format!("{PREFIX}_keepout_flag_posts"));
    for (i, (x, y, z)) in [
        (
            TRAP_POS.0 - TRAP_X / 2.0 + 30.0,
            TRAP_POS.1 + 46.0,
            TRAP_CARTRIDGE_LIFT_CLEARANCE_Z,
        ),
        (
            OPTICAL_POS.0 + OPTICAL_X / 2.0 - 30.0,
            OPTICAL_POS.1 + 46.0,
            SENSOR_SERVICE_CLEARANCE_Z,
        ),
        (
            PURGE_POS.0 + PURGE_X / 2.0 + PURGE_CASSETTE_PULL_CLEARANCE_X,
            PURGE_POS.1 + 36.0,
            80.0,
        ),
        (
            STATION_X / 2.0 - RIGHT_SERVICE_CLEARANCE_X,
            STATION_Y / 2.0 - REAR_SERVICE_CLEARANCE_Y,
            80.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cube(
                format!("{PREFIX}_service_keepout_flag_post_{i}"),
                8.0,
                8.0,
                *z,
            )
            .translate(*x, *y, BASE_Z + *z / 2.0)
            + centered_cube(
                format!("{PREFIX}_service_keepout_flag_tab_{i}"),
                42.0,
                4.0,
                16.0,
            )
            .translate(*x + 17.0, *y, BASE_Z + *z + 8.0);
    }
    posts
}

fn module_slab(name: String, center: (f64, f64), x: f64, y: f64, z: f64) -> Part {
    centered_cube(name, x, y, z).translate(center.0, center.1, BASE_Z + MODULE_RISER_Z + z / 2.0)
}

fn lane_x(center_x: f64, lane: usize) -> f64 {
    center_x + centered_index(lane, TRAP_LANES, LANE_PITCH_X)
}

fn disposition_lane_x(lane: DispositionLane) -> f64 {
    LANES_POS.0 + centered_index(lane.index(), DISPOSITION_LANES, 132.0)
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn threshold_tokens() -> [ThresholdToken; ALARM_LEVELS] {
    [
        ThresholdToken {
            name: "warning",
            threshold_ml: WARNING_THRESHOLD_ML,
            token_height: 8.0,
        },
        ThresholdToken {
            name: "alarm",
            threshold_ml: EXHAUSTION_THRESHOLD_ML,
            token_height: 14.0,
        },
        ThresholdToken {
            name: "reject",
            threshold_ml: REJECT_THRESHOLD_ML,
            token_height: 20.0,
        },
    ]
}

fn witness_chamber_volume_ml() -> f64 {
    cylinder_volume_ml(WITNESS_CHAMBER_RADIUS - 3.0, WITNESS_CHAMBER_Z - 5.0)
}

fn purge_capture_volume_ml() -> f64 {
    cylinder_volume_ml(PURGE_CUP_RADIUS - 3.2, PURGE_CUP_Z - 5.0)
}

fn cylinder_volume_ml(radius_mm: f64, height_mm: f64) -> f64 {
    std::f64::consts::PI * radius_mm * radius_mm * height_mm / 1000.0
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 13);
    assert_eq!(REQUIRED_FEATURES.len(), 11);
    assert_eq!(TRAP_CARTRIDGES, TRAP_LANES);
    assert_eq!(WITNESS_CHAMBERS, TRAP_LANES);
    assert_eq!(OPTICAL_SENSOR_CHANNELS, TRAP_LANES);
    assert_eq!(FLOW_WITNESS_WINDOWS, TRAP_LANES);
    assert_eq!(DIVERTER_VALVES, TRAP_LANES);
    assert_eq!(PRESSURE_TAPS, TRAP_LANES * PRESSURE_TAPS_PER_LANE);
    assert_eq!(SLUG_INJECTION_PORTS, TRAP_LANES * 2);
    assert_eq!(ALARM_THRESHOLD_TOKENS, TRAP_LANES * ALARM_LEVELS);
    assert!(TUBE_BORE_D > TUBE_OD_MAX_MM);
    assert!(WARNING_THRESHOLD_ML < EXHAUSTION_THRESHOLD_ML);
    assert!(EXHAUSTION_THRESHOLD_ML < REJECT_THRESHOLD_ML);
    assert!(witness_chamber_volume_ml() > REJECT_THRESHOLD_ML);
    assert!(purge_capture_volume_ml() > REJECT_THRESHOLD_ML * 1.5);
    assert!(TRAP_CARTRIDGE_LIFT_CLEARANCE_Z > TRAP_BANK_Z + TRAP_COLUMN_Z);
    assert!(SENSOR_SERVICE_CLEARANCE_Z > OPTICAL_Z);
    assert_eq!(ROBOT_KEEP_OUT_GAUGES, 6);

    let rects = layout_rects();
    for rect in rects {
        assert!(
            rect.fits_inside_station(),
            "{} does not fit inside usable station footprint",
            rect.name
        );
    }

    for (i, left) in rects.iter().enumerate() {
        for right in rects.iter().skip(i + 1) {
            assert!(
                !left.overlaps_with_clearance(*right, LAYOUT_CLEARANCE),
                "{} overlaps {}",
                left.name,
                right.name
            );
        }
    }
}

fn layout_rects() -> [Rect; 10] {
    [
        Rect {
            name: "bubble_trap_cartridge_bank",
            center: TRAP_POS,
            x: TRAP_X,
            y: TRAP_Y,
        },
        Rect {
            name: "graduated_capacity_witness_chambers",
            center: WITNESS_POS,
            x: WITNESS_X,
            y: WITNESS_Y,
        },
        Rect {
            name: "slug_injection_manifold",
            center: SLUG_POS,
            x: SLUG_X,
            y: SLUG_Y,
        },
        Rect {
            name: "optical_bubble_sensor_bridge",
            center: OPTICAL_POS,
            x: OPTICAL_X,
            y: OPTICAL_Y,
        },
        Rect {
            name: "pressure_flow_tap_panel",
            center: TAP_POS,
            x: TAP_X,
            y: TAP_Y,
        },
        Rect {
            name: "overflow_purge_capture_cassette",
            center: PURGE_POS,
            x: PURGE_X,
            y: PURGE_Y,
        },
        Rect {
            name: "alarm_threshold_token_rail",
            center: ALARM_POS,
            x: ALARM_X,
            y: ALARM_Y,
        },
        Rect {
            name: "quarantine_diverter_mock_path",
            center: DIVERTER_POS,
            x: DIVERTER_X,
            y: DIVERTER_Y,
        },
        Rect {
            name: "release_hold_reject_lanes",
            center: LANES_POS,
            x: LANES_X,
            y: LANES_Y,
        },
        Rect {
            name: "evidence_bridge",
            center: EVIDENCE_POS,
            x: EVIDENCE_X,
            y: EVIDENCE_Y,
        },
    ]
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn outputs_are_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 13);
        for path in OUTPUTS {
            assert!(path.starts_with(
                "output/closed_perfusion_bubble_trap_capacity_exhaustion_alarm_station_"
            ));
            assert!(path.ends_with(".stl"));
        }
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }

    #[test]
    fn requested_validation_features_are_represented() {
        for feature in [
            "bubble_trap_cartridge_bank",
            "graduated_capacity_witness_chambers",
            "slug_injection_manifold",
            "optical_bubble_sensor_bridge",
            "pressure_flow_taps",
            "overflow_purge_capture_cassette",
            "alarm_threshold_token_rail",
            "quarantine_diverter_mock_path",
            "release_hold_reject_lanes",
            "evidence_bridge",
            "robot_service_keepouts",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn lane_counts_keep_alarm_and_disposition_paths_balanced() {
        assert_eq!(TRAP_LANES, 6);
        assert_eq!(TRAP_CARTRIDGES, TRAP_LANES);
        assert_eq!(WITNESS_CHAMBERS, TRAP_LANES);
        assert_eq!(OPTICAL_SENSOR_CHANNELS, TRAP_LANES);
        assert_eq!(FLOW_WITNESS_WINDOWS, TRAP_LANES);
        assert_eq!(DIVERTER_VALVES, TRAP_LANES);
        assert_eq!(PRESSURE_TAPS, TRAP_LANES * 2);
        assert_eq!(OVERFLOW_BRANCHES, TRAP_LANES * 2);
        assert_eq!(ALARM_THRESHOLD_TOKENS, TRAP_LANES * ALARM_LEVELS);
    }

    #[test]
    fn capacity_thresholds_are_ordered_and_captured() {
        let tokens = threshold_tokens();
        assert!(tokens[0].threshold_ml < tokens[1].threshold_ml);
        assert!(tokens[1].threshold_ml < tokens[2].threshold_ml);
        assert!(witness_chamber_volume_ml() > REJECT_THRESHOLD_ML);
        assert!(purge_capture_volume_ml() > REJECT_THRESHOLD_ML * 1.5);
    }

    #[test]
    fn modules_fit_without_overlapping() {
        assert_design_constraints();
        for rect in layout_rects() {
            assert!(rect.fits_inside_station());
        }
    }

    #[test]
    fn workflow_layout_runs_from_trap_to_witness_sensor_alarm_and_disposition() {
        assert!(TRAP_POS.0 < WITNESS_POS.0);
        assert!(WITNESS_POS.0 < SLUG_POS.0);
        assert!(OPTICAL_POS.0 < TAP_POS.0);
        assert!(TAP_POS.0 < PURGE_POS.0);
        assert!(ALARM_POS.0 < DIVERTER_POS.0);
        assert!(DIVERTER_POS.0 < LANES_POS.0);
        assert!(EVIDENCE_POS.1 > TRAP_POS.1);
    }

    #[test]
    fn lane_helpers_are_symmetric() {
        assert_eq!(
            lane_x(TRAP_POS.0, 0) - TRAP_POS.0,
            -(lane_x(TRAP_POS.0, TRAP_LANES - 1) - TRAP_POS.0)
        );
        assert_eq!(
            lane_x(WITNESS_POS.0, 1) - WITNESS_POS.0,
            -(lane_x(WITNESS_POS.0, TRAP_LANES - 2) - WITNESS_POS.0)
        );
        assert_eq!(
            disposition_lane_x(DispositionLane::Release) - LANES_POS.0,
            -(disposition_lane_x(DispositionLane::Reject) - LANES_POS.0)
        );
    }
}
