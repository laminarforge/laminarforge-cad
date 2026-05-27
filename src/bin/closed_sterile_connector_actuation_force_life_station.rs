use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed sterile connector actuation force and cycle-life validation station.
//
// Intent:
// - Validate sterile connector actuation force and repeated cycle-life before a
//   closed fluid path is released for culture work.
// - Keep connector nests, actuation-force metrology, cycle tokens, caps/plugs,
//   pressure-decay witness ports, evidence capture, retain samples, and release
//   disposition physically separated and mechanically legible.
// - Expose robot datum blocks, service keepout gauges, and containment features
//   for workcell planning without claiming a sterile barrier or release method.
//
// This is packaging/interface CAD for a validation station. It is not a
// validated sterile connector design, metrology procedure, or process release
// specification.

const OUTPUTS: [&str; 14] = [
    "output/closed_sterile_connector_actuation_force_life_station_base_leak_tray_deck.stl",
    "output/closed_sterile_connector_actuation_force_life_station_connector_nest_arrays.stl",
    "output/closed_sterile_connector_actuation_force_life_station_actuation_force_gauge_pocket.stl",
    "output/closed_sterile_connector_actuation_force_life_station_cycle_counter_token_lane.stl",
    "output/closed_sterile_connector_actuation_force_life_station_cap_plug_parks.stl",
    "output/closed_sterile_connector_actuation_force_life_station_pressure_decay_witness_ports.stl",
    "output/closed_sterile_connector_actuation_force_life_station_barcode_coa_lands.stl",
    "output/closed_sterile_connector_actuation_force_life_station_retain_sample_pockets.stl",
    "output/closed_sterile_connector_actuation_force_life_station_release_hold_reject_lanes.stl",
    "output/closed_sterile_connector_actuation_force_life_station_clean_used_segregation.stl",
    "output/closed_sterile_connector_actuation_force_life_station_evidence_camera_bridge.stl",
    "output/closed_sterile_connector_actuation_force_life_station_robot_gripper_datum_blocks.stl",
    "output/closed_sterile_connector_actuation_force_life_station_service_keepout_gauges.stl",
    "output/closed_sterile_connector_actuation_force_life_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 13] = [
    "base_leak_tray_deck",
    "connector_nest_arrays",
    "actuation_force_gauge_pocket",
    "cycle_counter_token_lane",
    "cap_plug_parks",
    "pressure_decay_witness_ports",
    "barcode_coa_lands",
    "retain_sample_pockets",
    "release_hold_reject_lanes",
    "clean_used_segregation",
    "evidence_camera_bridge",
    "robot_gripper_datum_blocks",
    "service_keepout_gauges",
];

const STATION_X: f64 = 1120.0;
const STATION_Y: f64 = 820.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const SOCKET_DEPTH: f64 = 5.0;
const DRAIN_D: f64 = 10.0;
const MOUNT_HOLE_D: f64 = 6.6;

const NEST_ROWS: usize = 3;
const NEST_COLS: usize = 6;
const CONNECTOR_COUNT: usize = NEST_ROWS * NEST_COLS;
const NEST_X: f64 = 510.0;
const NEST_Y: f64 = 238.0;
const NEST_Z: f64 = 62.0;
const NEST_POS: (f64, f64) = (-270.0, 205.0);
const CONNECTOR_SLOT_X: f64 = 54.0;
const CONNECTOR_SLOT_Y: f64 = 31.0;
const CONNECTOR_SLOT_Z: f64 = 28.0;
const CONNECTOR_PITCH_X: f64 = 76.0;
const CONNECTOR_PITCH_Y: f64 = 58.0;
const CONNECTOR_BODY_D: f64 = 18.0;
const CONNECTOR_STEM_D: f64 = 7.2;

const GAUGE_X: f64 = 370.0;
const GAUGE_Y: f64 = 214.0;
const GAUGE_Z: f64 = 72.0;
const GAUGE_POS: (f64, f64) = (312.0, 205.0);
const FORCE_GAUGE_X: f64 = 190.0;
const FORCE_GAUGE_Y: f64 = 68.0;
const FORCE_GAUGE_Z: f64 = 34.0;
const FORCE_PLUNGER_D: f64 = 14.0;
const FORCE_ANVIL_D: f64 = 32.0;
const CAL_WEIGHT_POCKETS: usize = 4;

const CYCLE_X: f64 = 512.0;
const CYCLE_Y: f64 = 148.0;
const CYCLE_Z: f64 = 40.0;
const CYCLE_POS: (f64, f64) = (-270.0, -70.0);
const CYCLE_TOKEN_COUNT: usize = 24;
const CYCLE_TOKEN_D: f64 = 16.0;
const CYCLE_COUNTER_WINDOWS: usize = 6;
const CYCLE_CLICK_TRACKS: usize = 3;

const CAP_PLUG_X: f64 = 340.0;
const CAP_PLUG_Y: f64 = 154.0;
const CAP_PLUG_Z: f64 = 38.0;
const CAP_PLUG_POS: (f64, f64) = (328.0, -48.0);
const CAP_WELLS: usize = 24;
const PLUG_WELLS: usize = 24;
const CAP_WELL_D: f64 = 9.0;
const PLUG_WELL_D: f64 = 7.0;
const CAP_PLUG_WELL_DEPTH: f64 = 16.0;

const PRESSURE_X: f64 = 380.0;
const PRESSURE_Y: f64 = 134.0;
const PRESSURE_Z: f64 = 64.0;
const PRESSURE_POS: (f64, f64) = (-335.0, -278.0);
const PRESSURE_PORTS: usize = 12;
const WITNESS_WELLS: usize = 12;
const PRESSURE_PORT_D: f64 = 8.0;
const WITNESS_WELL_D: f64 = 13.0;

const TRACE_X: f64 = 420.0;
const TRACE_Y: f64 = 112.0;
const TRACE_Z: f64 = 12.0;
const TRACE_POS: (f64, f64) = (18.0, -322.0);
const BARCODE_LANDS: usize = 6;
const COA_LANDS: usize = 3;
const OPERATOR_SIGNOFF_LANDS: usize = 2;

const RETAIN_X: f64 = 248.0;
const RETAIN_Y: f64 = 152.0;
const RETAIN_Z: f64 = 44.0;
const RETAIN_POS: (f64, f64) = (405.0, -268.0);
const RETAIN_POCKETS: usize = 12;
const RETAIN_POCKET_D: f64 = 17.0;
const RETAIN_POCKET_DEPTH: f64 = 24.0;

const STATUS_LANES: usize = 3;
const STATUS_LANE_X: f64 = 138.0;
const STATUS_LANE_Y: f64 = 110.0;
const STATUS_LANE_Z: f64 = 28.0;
const STATUS_LANE_PITCH_X: f64 = 158.0;
const STATUS_POS: (f64, f64) = (64.0, 58.0);
const STATUS_SLOT_COUNT: usize = 6;

const SEGREGATION_BARRIER_X: f64 = 18.0;
const SEGREGATION_BARRIER_Y: f64 = 584.0;
const SEGREGATION_BARRIER_Z: f64 = 82.0;
const CLEAN_USED_GAP_MIN: f64 = 92.0;

const BRIDGE_SPAN_X: f64 = 820.0;
const BRIDGE_POST_X: f64 = 24.0;
const BRIDGE_POST_Y: f64 = 44.0;
const BRIDGE_POST_Z: f64 = 188.0;
const BRIDGE_BEAM_Y: f64 = 44.0;
const BRIDGE_BEAM_Z: f64 = 22.0;
const CAMERA_CLEARANCE_Z: f64 = 138.0;
const CAMERA_POS: (f64, f64) = (0.0, -316.0);

const GRIPPER_DATUM_BLOCKS: usize = 6;
const GRIPPER_BLOCK_X: f64 = 62.0;
const GRIPPER_BLOCK_Y: f64 = 42.0;
const GRIPPER_BLOCK_Z: f64 = 34.0;
const GRIPPER_PIN_D: f64 = 8.0;

const SERVICE_KEEP_OUT_GAUGES: usize = 5;
const KEEP_OUT_RAIL_Z: f64 = 8.0;
const SERVICE_GAUGE_Z: f64 = 56.0;
const FRONT_ROBOT_SWEEP_Y: f64 = -376.0;
const REAR_SERVICE_SWEEP_Y: f64 = 376.0;

#[derive(Clone, Copy)]
struct Rect {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    let base = base_leak_tray_deck();
    export(OUTPUTS[0], &base);

    let nests = connector_nest_arrays();
    export(OUTPUTS[1], &nests);

    let gauge = actuation_force_gauge_pocket();
    export(OUTPUTS[2], &gauge);

    let cycle_lane = cycle_counter_token_lane();
    export(OUTPUTS[3], &cycle_lane);

    let cap_plugs = cap_plug_parks();
    export(OUTPUTS[4], &cap_plugs);

    let pressure_ports = pressure_decay_witness_ports();
    export(OUTPUTS[5], &pressure_ports);

    let traceability = barcode_coa_lands();
    export(OUTPUTS[6], &traceability);

    let retain = retain_sample_pockets();
    export(OUTPUTS[7], &retain);

    let status = release_hold_reject_lanes();
    export(OUTPUTS[8], &status);

    let segregation = clean_used_segregation();
    export(OUTPUTS[9], &segregation);

    let bridge = evidence_camera_bridge();
    export(OUTPUTS[10], &bridge);

    let datum_blocks = robot_gripper_datum_blocks();
    export(OUTPUTS[11], &datum_blocks);

    let keepouts = service_keepout_gauges();
    export(OUTPUTS[12], &keepouts);

    let assembly = base
        + nests.translate(NEST_POS.0, NEST_POS.1, insert_z(NEST_Z))
        + gauge.translate(GAUGE_POS.0, GAUGE_POS.1, insert_z(GAUGE_Z))
        + cycle_lane.translate(CYCLE_POS.0, CYCLE_POS.1, insert_z(CYCLE_Z))
        + cap_plugs.translate(CAP_PLUG_POS.0, CAP_PLUG_POS.1, insert_z(CAP_PLUG_Z))
        + pressure_ports.translate(PRESSURE_POS.0, PRESSURE_POS.1, insert_z(PRESSURE_Z))
        + traceability.translate(TRACE_POS.0, TRACE_POS.1, insert_z(TRACE_Z))
        + retain.translate(RETAIN_POS.0, RETAIN_POS.1, insert_z(RETAIN_Z))
        + status.translate(
            STATUS_POS.0,
            STATUS_POS.1,
            BASE_Z / 2.0 + STATUS_LANE_Z / 2.0 + 8.0,
        )
        + segregation.translate(0.0, 38.0, BASE_Z / 2.0 + SEGREGATION_BARRIER_Z / 2.0)
        + bridge.translate(
            CAMERA_POS.0,
            CAMERA_POS.1,
            BASE_Z / 2.0 + BRIDGE_POST_Z / 2.0 + 8.0,
        )
        + datum_blocks.translate(0.0, 0.0, BASE_Z / 2.0 + GRIPPER_BLOCK_Z / 2.0 + 6.0)
        + keepouts.translate(0.0, 0.0, BASE_Z / 2.0 + KEEP_OUT_RAIL_Z / 2.0 + 3.0);
    export(OUTPUTS[13], &assembly);

    println!();
    println!("Closed sterile connector actuation force/cycle-life validation station:");
    println!(
        "  Footprint:              {STATION_X:.0}mm x {STATION_Y:.0}mm contained leak tray deck"
    );
    println!(
        "  Connector nests:        {CONNECTOR_COUNT} closed connector nests in a {NEST_ROWS}x{NEST_COLS} array"
    );
    println!(
        "  Actuation metrology:    force gauge pocket, {CAL_WEIGHT_POCKETS} calibration pockets, plunger/anvil line"
    );
    println!(
        "  Cycle-life tracking:    {CYCLE_TOKEN_COUNT} tokens, {CYCLE_COUNTER_WINDOWS} counter windows, {CYCLE_CLICK_TRACKS} click tracks"
    );
    println!(
        "  Release evidence:       {PRESSURE_PORTS} pressure-decay ports, {WITNESS_WELLS} witness wells, {BARCODE_LANDS} barcode lands, {COA_LANDS} COA lands"
    );
    println!(
        "  Flow control:           cap/plug parks, retain samples, release/hold/reject lanes, clean/used segregation"
    );
    println!(
        "  Automation planning:    evidence camera bridge, {GRIPPER_DATUM_BLOCKS} robot datum blocks, {SERVICE_KEEP_OUT_GAUGES} service keepout gauges"
    );
}

fn export(path: &str, part: &Part) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn insert_z(height: f64) -> f64 {
    BASE_Z / 2.0 + height / 2.0 + 6.0
}

fn assert_design_constraints() {
    assert_eq!(OUTPUTS.len(), 14, "export count changed");
    assert_eq!(REQUIRED_FEATURES.len(), 13, "required feature list changed");
    assert_eq!(CONNECTOR_COUNT, NEST_ROWS * NEST_COLS);
    assert_eq!(STATUS_LANES, 3);
    assert!(CYCLE_TOKEN_COUNT >= CONNECTOR_COUNT);
    assert!(CAP_WELLS >= CONNECTOR_COUNT);
    assert!(PLUG_WELLS >= CONNECTOR_COUNT);
    assert!(PRESSURE_PORTS >= CONNECTOR_COUNT / 2);
    assert!(WITNESS_WELLS == PRESSURE_PORTS);
    assert!(RETAIN_POCKETS >= CONNECTOR_COUNT / 2);
    assert!(BRIDGE_POST_Z > CAMERA_CLEARANCE_Z);
    assert!(FRONT_ROBOT_SWEEP_Y.abs() < STATION_Y / 2.0);
    assert!(REAR_SERVICE_SWEEP_Y.abs() < STATION_Y / 2.0);

    for (name, rect) in insert_rects() {
        assert!(
            fits_on_station(rect),
            "{name} exceeds station envelope: x={} y={} w={} h={}",
            rect.x,
            rect.y,
            rect.w,
            rect.h
        );
    }

    assert!(
        !rects_overlap(nest_rect(), gauge_rect()),
        "connector nests and force gauge must remain separated"
    );
    assert!(
        !rects_overlap(pressure_rect(), retain_rect()),
        "pressure witness ports and retain pockets must remain separated"
    );
    assert!(
        clean_rect().x + clean_rect().w / 2.0 + CLEAN_USED_GAP_MIN / 2.0
            < used_rect().x - used_rect().w / 2.0,
        "clean and used sides need a visible segregation gap"
    );
}

fn base_leak_tray_deck() -> Part {
    let floor = centered_cube(
        "closed_sterile_connector_force_life_base_leak_tray_floor",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let inner_basin = centered_cube(
        "closed_sterile_connector_force_life_contained_basin_recess",
        STATION_X - 126.0,
        STATION_Y - 112.0,
        8.0,
    )
    .translate(0.0, -4.0, BASE_Z / 2.0 - 4.0);
    let clean_side_sump = centered_cube(
        "closed_sterile_connector_force_life_clean_side_sump_recess",
        500.0,
        236.0,
        8.0,
    )
    .translate(-286.0, 188.0, BASE_Z / 2.0 - 4.0);
    let used_side_sump = centered_cube(
        "closed_sterile_connector_force_life_used_side_sump_recess",
        468.0,
        226.0,
        8.0,
    )
    .translate(300.0, -44.0, BASE_Z / 2.0 - 4.0);
    let front_drain = centered_cylinder(
        "closed_sterile_connector_force_life_front_low_point_drain",
        DRAIN_D / 2.0,
        46.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 84.0, -STATION_Y / 2.0 - 2.0, -1.0);
    let rear_drain = centered_cylinder(
        "closed_sterile_connector_force_life_rear_clean_side_drain",
        DRAIN_D / 2.0,
        46.0,
        28,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(-(STATION_X / 2.0 - 122.0), STATION_Y / 2.0 - 94.0, -1.0);

    floor
        - inner_basin
        - clean_side_sump
        - used_side_sump
        - front_drain
        - rear_drain
        - registration_sockets()
        - mounting_slots()
        + perimeter_rims()
        + leak_witness_channels()
        + deck_zone_fiducials()
}

fn registration_sockets() -> Part {
    let mut sockets = Part::empty("closed_sterile_connector_force_life_registration_sockets");
    for (name, rect) in insert_rects() {
        sockets = sockets
            + centered_cube(
                format!("closed_sterile_connector_force_life_{name}_socket"),
                rect.w + 10.0,
                rect.h + 10.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(rect.x, rect.y, BASE_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.2);
    }
    sockets
}

fn mounting_slots() -> Part {
    let mut slots = Part::empty("closed_sterile_connector_force_life_mounting_slots");
    for (index, (x, y)) in [
        (-(STATION_X / 2.0 - 54.0), -(STATION_Y / 2.0 - 48.0)),
        (STATION_X / 2.0 - 54.0, -(STATION_Y / 2.0 - 48.0)),
        (-(STATION_X / 2.0 - 54.0), STATION_Y / 2.0 - 48.0),
        (STATION_X / 2.0 - 54.0, STATION_Y / 2.0 - 48.0),
        (0.0, STATION_Y / 2.0 - 48.0),
        (0.0, -(STATION_Y / 2.0 - 48.0)),
    ]
    .iter()
    .enumerate()
    {
        let hole = centered_cylinder(
            format!("closed_sterile_connector_force_life_m6_mount_hole_{index}"),
            MOUNT_HOLE_D / 2.0,
            BASE_Z + 4.0,
            24,
        )
        .translate(*x, *y, 0.0);
        let slot = centered_cube(
            format!("closed_sterile_connector_force_life_m6_slot_relief_{index}"),
            26.0,
            MOUNT_HOLE_D + 0.4,
            BASE_Z + 4.0,
        )
        .translate(*x, *y, 0.0);
        slots = slots + hole + slot;
    }
    slots
}

fn perimeter_rims() -> Part {
    let left = centered_cube(
        "closed_sterile_connector_force_life_left_containment_rim",
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
        "closed_sterile_connector_force_life_right_containment_rim",
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
        "closed_sterile_connector_force_life_rear_containment_rim",
        STATION_X - 36.0,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let front = centered_cube(
        "closed_sterile_connector_force_life_front_low_robot_lip",
        STATION_X - 174.0,
        14.0,
        22.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 26.0, BASE_Z / 2.0 + 11.0);
    left + right + rear + front
}

fn leak_witness_channels() -> Part {
    let mut channels = Part::empty("closed_sterile_connector_force_life_leak_witness_channels");
    for i in 0..8 {
        let x = centered_index(i, 8, 118.0);
        channels = channels
            + centered_cube(
                format!("closed_sterile_connector_force_life_leak_witness_rib_{i}"),
                88.0,
                5.0,
                7.0,
            )
            .translate(x, -354.0, BASE_Z / 2.0 + 3.5);
    }
    channels
}

fn deck_zone_fiducials() -> Part {
    let mut fiducials = Part::empty("closed_sterile_connector_force_life_deck_fiducials");
    for (i, (x, y)) in [
        (-502.0, 344.0),
        (502.0, 344.0),
        (-502.0, -344.0),
        (502.0, -344.0),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + fiducial_disc(&format!(
                "closed_sterile_connector_force_life_deck_fiducial_{i}"
            ))
            .translate(*x, *y, BASE_Z / 2.0 + 3.0);
    }
    fiducials
}

fn connector_nest_arrays() -> Part {
    let body = centered_cube(
        "closed_sterile_connector_force_life_connector_nest_array_body",
        NEST_X,
        NEST_Y,
        NEST_Z,
    );
    let rear_fence = centered_cube(
        "closed_sterile_connector_force_life_connector_nest_rear_fence",
        NEST_X,
        14.0,
        NEST_Z + 32.0,
    )
    .translate(0.0, NEST_Y / 2.0 - 7.0, 16.0);
    let side_fence = centered_cube(
        "closed_sterile_connector_force_life_connector_nest_left_clean_fence",
        14.0,
        NEST_Y - 20.0,
        NEST_Z + 24.0,
    )
    .translate(-(NEST_X / 2.0 - 7.0), 0.0, 12.0);

    let mut cuts = Part::empty("closed_sterile_connector_force_life_connector_nest_cuts");
    let mut clamp_lands = Part::empty("closed_sterile_connector_force_life_connector_clamp_lands");
    let mut cycle_index_lands =
        Part::empty("closed_sterile_connector_force_life_connector_cycle_index_lands");

    for row in 0..NEST_ROWS {
        for col in 0..NEST_COLS {
            let index = row * NEST_COLS + col;
            let (x, y) = connector_center(row, col);
            cuts = cuts
                + connector_body_pocket(index, x, y)
                + connector_stem_pair(index, x, y)
                + centered_cube(
                    format!("closed_sterile_connector_force_life_connector_{index}_finger_relief"),
                    18.0,
                    CONNECTOR_SLOT_Y + 14.0,
                    24.0,
                )
                .translate(x, y - 3.0, NEST_Z / 2.0 - 8.0);

            clamp_lands = clamp_lands
                + centered_cube(
                    format!(
                        "closed_sterile_connector_force_life_connector_{index}_left_clamp_land"
                    ),
                    12.0,
                    CONNECTOR_SLOT_Y + 12.0,
                    10.0,
                )
                .translate(
                    x - CONNECTOR_SLOT_X / 2.0 - 10.0,
                    y,
                    NEST_Z / 2.0 + 5.0,
                )
                + centered_cube(
                    format!(
                        "closed_sterile_connector_force_life_connector_{index}_right_clamp_land"
                    ),
                    12.0,
                    CONNECTOR_SLOT_Y + 12.0,
                    10.0,
                )
                .translate(
                    x + CONNECTOR_SLOT_X / 2.0 + 10.0,
                    y,
                    NEST_Z / 2.0 + 5.0,
                );

            cycle_index_lands = cycle_index_lands
                + centered_cube(
                    format!(
                        "closed_sterile_connector_force_life_connector_{index}_cycle_index_land"
                    ),
                    28.0,
                    5.0,
                    5.0,
                )
                .translate(
                    x,
                    y - CONNECTOR_SLOT_Y / 2.0 - 14.0,
                    NEST_Z / 2.0 + 2.5,
                );
        }
    }

    body + rear_fence + side_fence + clamp_lands + cycle_index_lands - cuts + nest_robot_fiducials()
}

fn connector_body_pocket(index: usize, x: f64, y: f64) -> Part {
    let slot = centered_cube(
        format!("closed_sterile_connector_force_life_connector_{index}_rectangular_pocket"),
        CONNECTOR_SLOT_X,
        CONNECTOR_SLOT_Y,
        CONNECTOR_SLOT_Z,
    )
    .translate(x, y, NEST_Z / 2.0 - CONNECTOR_SLOT_Z / 2.0 + 2.0);
    let cradle = centered_cylinder(
        format!("closed_sterile_connector_force_life_connector_{index}_round_cradle"),
        CONNECTOR_BODY_D / 2.0,
        CONNECTOR_SLOT_X + 8.0,
        40,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(x, y, NEST_Z / 2.0 - 11.0);
    slot + cradle
}

fn connector_stem_pair(index: usize, x: f64, y: f64) -> Part {
    let left = centered_cylinder(
        format!("closed_sterile_connector_force_life_connector_{index}_left_stem_socket"),
        CONNECTOR_STEM_D / 2.0,
        44.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(x - CONNECTOR_SLOT_X / 2.0 - 2.0, y, NEST_Z / 2.0 - 10.0);
    let right = centered_cylinder(
        format!("closed_sterile_connector_force_life_connector_{index}_right_stem_socket"),
        CONNECTOR_STEM_D / 2.0,
        44.0,
        24,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(x + CONNECTOR_SLOT_X / 2.0 + 2.0, y, NEST_Z / 2.0 - 10.0);
    left + right
}

fn nest_robot_fiducials() -> Part {
    let mut fiducials = Part::empty("closed_sterile_connector_force_life_nest_robot_fiducials");
    for (i, x) in [-214.0, 214.0].iter().enumerate() {
        fiducials = fiducials
            + fiducial_disc(&format!(
                "closed_sterile_connector_force_life_nest_robot_fiducial_{i}"
            ))
            .translate(*x, -(NEST_Y / 2.0 - 26.0), NEST_Z / 2.0 + 3.0);
    }
    fiducials
}

fn actuation_force_gauge_pocket() -> Part {
    let body = centered_cube(
        "closed_sterile_connector_force_life_actuation_force_gauge_body",
        GAUGE_X,
        GAUGE_Y,
        GAUGE_Z,
    );
    let gauge_recess = centered_cube(
        "closed_sterile_connector_force_life_force_gauge_recess",
        FORCE_GAUGE_X,
        FORCE_GAUGE_Y,
        FORCE_GAUGE_Z,
    )
    .translate(-36.0, -10.0, GAUGE_Z / 2.0 - FORCE_GAUGE_Z / 2.0 + 1.0);
    let plunger_bore = centered_cylinder(
        "closed_sterile_connector_force_life_force_plunger_bore",
        FORCE_PLUNGER_D / 2.0,
        GAUGE_Y + 18.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(GAUGE_X / 2.0 - 54.0, 0.0, 8.0);
    let cable_notch = centered_cube(
        "closed_sterile_connector_force_life_force_gauge_cable_notch",
        48.0,
        28.0,
        24.0,
    )
    .translate(-(GAUGE_X / 2.0 - 20.0), -10.0, GAUGE_Z / 2.0 - 10.0);
    let anvil = centered_cylinder(
        "closed_sterile_connector_force_life_actuation_anvil_face",
        FORCE_ANVIL_D / 2.0,
        16.0,
        40,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(GAUGE_X / 2.0 - 52.0, -(GAUGE_Y / 2.0 + 8.0), 9.0);
    let travel_scale = centered_cube(
        "closed_sterile_connector_force_life_actuation_travel_scale_land",
        236.0,
        12.0,
        5.0,
    )
    .translate(20.0, GAUGE_Y / 2.0 - 20.0, GAUGE_Z / 2.0 + 2.5);

    body - gauge_recess - plunger_bore - cable_notch + anvil + travel_scale + calibration_pockets()
}

fn calibration_pockets() -> Part {
    let mut pockets = Part::empty("closed_sterile_connector_force_life_calibration_pockets");
    for i in 0..CAL_WEIGHT_POCKETS {
        let x = centered_index(i, CAL_WEIGHT_POCKETS, 46.0) - 42.0;
        pockets = pockets
            + centered_cylinder(
                format!("closed_sterile_connector_force_life_force_cal_weight_pocket_{i}"),
                14.0,
                15.0,
                32,
            )
            .translate(x, -(GAUGE_Y / 2.0 - 36.0), GAUGE_Z / 2.0 - 6.0);
    }
    pockets
}

fn cycle_counter_token_lane() -> Part {
    let body = centered_cube(
        "closed_sterile_connector_force_life_cycle_counter_lane_body",
        CYCLE_X,
        CYCLE_Y,
        CYCLE_Z,
    );
    let mut token_pockets = Part::empty("closed_sterile_connector_force_life_cycle_token_pockets");
    for i in 0..CYCLE_TOKEN_COUNT {
        let col = i % 12;
        let row = i / 12;
        token_pockets = token_pockets
            + centered_cylinder(
                format!("closed_sterile_connector_force_life_cycle_token_pocket_{i}"),
                CYCLE_TOKEN_D / 2.0,
                18.0,
                28,
            )
            .translate(
                centered_index(col, 12, 36.0),
                24.0 + centered_index(row, 2, 34.0),
                CYCLE_Z / 2.0 - 8.0,
            );
    }

    let counter_windows = cycle_counter_windows();
    let click_tracks = cycle_click_tracks();
    let spent_token_chute = centered_cube(
        "closed_sterile_connector_force_life_spent_cycle_token_chute",
        CYCLE_X - 82.0,
        16.0,
        12.0,
    )
    .translate(0.0, -(CYCLE_Y / 2.0 - 24.0), CYCLE_Z / 2.0 - 4.0);

    body - token_pockets - spent_token_chute + counter_windows + click_tracks
}

fn cycle_counter_windows() -> Part {
    let mut windows = Part::empty("closed_sterile_connector_force_life_cycle_counter_window_lands");
    for i in 0..CYCLE_COUNTER_WINDOWS {
        windows = windows
            + centered_cube(
                format!("closed_sterile_connector_force_life_cycle_counter_window_land_{i}"),
                48.0,
                26.0,
                5.0,
            )
            .translate(
                centered_index(i, CYCLE_COUNTER_WINDOWS, 70.0),
                -(CYCLE_Y / 2.0 - 56.0),
                CYCLE_Z / 2.0 + 2.5,
            );
    }
    windows
}

fn cycle_click_tracks() -> Part {
    let mut tracks = Part::empty("closed_sterile_connector_force_life_cycle_click_tracks");
    for i in 0..CYCLE_CLICK_TRACKS {
        tracks = tracks
            + centered_cube(
                format!("closed_sterile_connector_force_life_cycle_click_track_{i}"),
                CYCLE_X - 82.0,
                5.0,
                6.0,
            )
            .translate(
                0.0,
                centered_index(i, CYCLE_CLICK_TRACKS, 18.0) - 6.0,
                CYCLE_Z / 2.0 + 3.0,
            );
    }
    tracks
}

fn cap_plug_parks() -> Part {
    let base = centered_cube(
        "closed_sterile_connector_force_life_cap_plug_park_body",
        CAP_PLUG_X,
        CAP_PLUG_Y,
        CAP_PLUG_Z,
    );
    let clean_label = centered_cube(
        "closed_sterile_connector_force_life_clean_cap_label_land",
        CAP_PLUG_X - 44.0,
        14.0,
        5.0,
    )
    .translate(0.0, CAP_PLUG_Y / 2.0 - 17.0, CAP_PLUG_Z / 2.0 + 2.5);
    let used_label = centered_cube(
        "closed_sterile_connector_force_life_used_plug_label_land",
        CAP_PLUG_X - 44.0,
        14.0,
        5.0,
    )
    .translate(0.0, -(CAP_PLUG_Y / 2.0 - 17.0), CAP_PLUG_Z / 2.0 + 2.5);
    let divider = centered_cube(
        "closed_sterile_connector_force_life_cap_plug_segregation_divider",
        CAP_PLUG_X - 34.0,
        8.0,
        20.0,
    )
    .translate(0.0, 0.0, CAP_PLUG_Z / 2.0 + 10.0);

    base - cap_wells() - plug_wells() + clean_label + used_label + divider
}

fn cap_wells() -> Part {
    let mut wells = Part::empty("closed_sterile_connector_force_life_cap_wells");
    for i in 0..CAP_WELLS {
        let col = i % 8;
        let row = i / 8;
        wells = wells
            + centered_cylinder(
                format!("closed_sterile_connector_force_life_cap_well_{i}"),
                CAP_WELL_D / 2.0,
                CAP_PLUG_WELL_DEPTH,
                24,
            )
            .translate(
                centered_index(col, 8, 32.0),
                28.0 + centered_index(row, 3, 22.0),
                CAP_PLUG_Z / 2.0 - CAP_PLUG_WELL_DEPTH / 2.0 + 1.0,
            );
    }
    wells
}

fn plug_wells() -> Part {
    let mut wells = Part::empty("closed_sterile_connector_force_life_plug_wells");
    for i in 0..PLUG_WELLS {
        let col = i % 8;
        let row = i / 8;
        wells = wells
            + centered_cylinder(
                format!("closed_sterile_connector_force_life_plug_well_{i}"),
                PLUG_WELL_D / 2.0,
                CAP_PLUG_WELL_DEPTH,
                24,
            )
            .translate(
                centered_index(col, 8, 32.0),
                -42.0 + centered_index(row, 3, 18.0),
                CAP_PLUG_Z / 2.0 - CAP_PLUG_WELL_DEPTH / 2.0 + 1.0,
            );
    }
    wells
}

fn pressure_decay_witness_ports() -> Part {
    let body = centered_cube(
        "closed_sterile_connector_force_life_pressure_decay_witness_body",
        PRESSURE_X,
        PRESSURE_Y,
        PRESSURE_Z,
    );
    let port_bores = pressure_port_bores();
    let witness_wells = pressure_witness_wells();
    let tube_guides = pressure_tube_guides();
    let witness_drip_lip = centered_cube(
        "closed_sterile_connector_force_life_pressure_witness_drip_lip",
        PRESSURE_X - 34.0,
        10.0,
        16.0,
    )
    .translate(0.0, -(PRESSURE_Y / 2.0 - 8.0), PRESSURE_Z / 2.0 + 8.0);

    body - port_bores - witness_wells + tube_guides + witness_drip_lip
}

fn pressure_port_bores() -> Part {
    let mut bores = Part::empty("closed_sterile_connector_force_life_pressure_port_bores");
    for i in 0..PRESSURE_PORTS {
        bores = bores
            + centered_cylinder(
                format!("closed_sterile_connector_force_life_pressure_decay_port_bore_{i}"),
                PRESSURE_PORT_D / 2.0,
                PRESSURE_Y + 8.0,
                28,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(
                centered_index(i, PRESSURE_PORTS, 28.0),
                0.0,
                PRESSURE_Z / 2.0 - 10.0,
            );
    }
    bores
}

fn pressure_witness_wells() -> Part {
    let mut wells = Part::empty("closed_sterile_connector_force_life_pressure_witness_wells");
    for i in 0..WITNESS_WELLS {
        wells = wells
            + centered_cylinder(
                format!("closed_sterile_connector_force_life_pressure_witness_well_{i}"),
                WITNESS_WELL_D / 2.0,
                17.0,
                28,
            )
            .translate(
                centered_index(i, WITNESS_WELLS, 28.0),
                -(PRESSURE_Y / 2.0 - 34.0),
                PRESSURE_Z / 2.0 - 7.0,
            );
    }
    wells
}

fn pressure_tube_guides() -> Part {
    let mut guides = Part::empty("closed_sterile_connector_force_life_pressure_tube_guides");
    for i in 0..PRESSURE_PORTS {
        guides = guides
            + centered_cylinder(
                format!("closed_sterile_connector_force_life_pressure_tube_guide_{i}"),
                5.6,
                22.0,
                24,
            )
            .translate(
                centered_index(i, PRESSURE_PORTS, 28.0),
                PRESSURE_Y / 2.0 + 10.0,
                PRESSURE_Z / 2.0 - 9.0,
            );
    }
    guides
}

fn barcode_coa_lands() -> Part {
    let panel = centered_cube(
        "closed_sterile_connector_force_life_barcode_coa_panel",
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    );
    let mut lands = Part::empty("closed_sterile_connector_force_life_barcode_coa_lands");
    for i in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_sterile_connector_force_life_barcode_land_{i}"),
                52.0,
                34.0,
                4.0,
            )
            .translate(
                centered_index(i, BARCODE_LANDS, 62.0),
                22.0,
                TRACE_Z / 2.0 + 2.0,
            );
    }
    for i in 0..COA_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_sterile_connector_force_life_coa_land_{i}"),
                92.0,
                32.0,
                4.0,
            )
            .translate(
                centered_index(i, COA_LANDS, 122.0),
                -22.0,
                TRACE_Z / 2.0 + 2.0,
            );
    }
    for i in 0..OPERATOR_SIGNOFF_LANDS {
        lands = lands
            + centered_cube(
                format!("closed_sterile_connector_force_life_operator_signoff_land_{i}"),
                72.0,
                10.0,
                4.0,
            )
            .translate(
                centered_index(i, OPERATOR_SIGNOFF_LANDS, 104.0),
                -(TRACE_Y / 2.0 - 12.0),
                TRACE_Z / 2.0 + 2.0,
            );
    }
    panel + lands
}

fn retain_sample_pockets() -> Part {
    let body = centered_cube(
        "closed_sterile_connector_force_life_retain_sample_body",
        RETAIN_X,
        RETAIN_Y,
        RETAIN_Z,
    );
    let mut pockets = Part::empty("closed_sterile_connector_force_life_retain_sample_pockets");
    for i in 0..RETAIN_POCKETS {
        let col = i % 4;
        let row = i / 4;
        pockets = pockets
            + centered_cylinder(
                format!("closed_sterile_connector_force_life_retain_sample_pocket_{i}"),
                RETAIN_POCKET_D / 2.0,
                RETAIN_POCKET_DEPTH,
                28,
            )
            .translate(
                centered_index(col, 4, 42.0),
                centered_index(row, 3, 38.0),
                RETAIN_Z / 2.0 - RETAIN_POCKET_DEPTH / 2.0 + 1.0,
            );
    }
    let tamper_land = centered_cube(
        "closed_sterile_connector_force_life_retain_tamper_evidence_land",
        RETAIN_X - 42.0,
        18.0,
        5.0,
    )
    .translate(0.0, -(RETAIN_Y / 2.0 - 18.0), RETAIN_Z / 2.0 + 2.5);
    body - pockets + tamper_land
}

fn release_hold_reject_lanes() -> Part {
    let mut lanes = Part::empty("closed_sterile_connector_force_life_release_hold_reject_lanes");
    for i in 0..STATUS_LANES {
        let name = match i {
            0 => "release",
            1 => "hold",
            _ => "reject",
        };
        let lane = centered_cube(
            format!("closed_sterile_connector_force_life_{name}_lane_body"),
            STATUS_LANE_X,
            STATUS_LANE_Y,
            STATUS_LANE_Z,
        )
        .translate(
            centered_index(i, STATUS_LANES, STATUS_LANE_PITCH_X),
            0.0,
            0.0,
        );
        lanes = lanes + lane - disposition_slots(name, i) + lane_divider(name, i);
    }
    lanes
}

fn disposition_slots(name: &str, lane_index: usize) -> Part {
    let mut slots = Part::empty(format!(
        "closed_sterile_connector_force_life_{name}_lane_slots"
    ));
    for i in 0..STATUS_SLOT_COUNT {
        let row = i / 3;
        let col = i % 3;
        slots = slots
            + centered_cube(
                format!("closed_sterile_connector_force_life_{name}_slot_{i}"),
                30.0,
                24.0,
                14.0,
            )
            .translate(
                centered_index(lane_index, STATUS_LANES, STATUS_LANE_PITCH_X)
                    + centered_index(col, 3, 38.0),
                centered_index(row, 2, 36.0),
                STATUS_LANE_Z / 2.0 - 6.0,
            );
    }
    slots
}

fn lane_divider(name: &str, lane_index: usize) -> Part {
    centered_cube(
        format!("closed_sterile_connector_force_life_{name}_raised_lane_header"),
        STATUS_LANE_X - 18.0,
        8.0,
        12.0,
    )
    .translate(
        centered_index(lane_index, STATUS_LANES, STATUS_LANE_PITCH_X),
        STATUS_LANE_Y / 2.0 - 10.0,
        STATUS_LANE_Z / 2.0 + 6.0,
    )
}

fn clean_used_segregation() -> Part {
    let barrier = centered_cube(
        "closed_sterile_connector_force_life_clean_used_center_barrier",
        SEGREGATION_BARRIER_X,
        SEGREGATION_BARRIER_Y,
        SEGREGATION_BARRIER_Z,
    );
    let clean_rail = centered_cube(
        "closed_sterile_connector_force_life_clean_side_greenfield_rail",
        10.0,
        SEGREGATION_BARRIER_Y - 74.0,
        24.0,
    )
    .translate(
        -CLEAN_USED_GAP_MIN / 2.0,
        0.0,
        -(SEGREGATION_BARRIER_Z / 2.0 - 12.0),
    );
    let used_rail = centered_cube(
        "closed_sterile_connector_force_life_used_side_return_rail",
        10.0,
        SEGREGATION_BARRIER_Y - 74.0,
        24.0,
    )
    .translate(
        CLEAN_USED_GAP_MIN / 2.0,
        0.0,
        -(SEGREGATION_BARRIER_Z / 2.0 - 12.0),
    );
    let pass_window = centered_cube(
        "closed_sterile_connector_force_life_one_way_document_pass_window",
        SEGREGATION_BARRIER_X + 4.0,
        92.0,
        26.0,
    )
    .translate(0.0, -112.0, 6.0);
    let wipe_shadow = centered_cube(
        "closed_sterile_connector_force_life_used_cap_wipe_shadow_lane",
        118.0,
        14.0,
        8.0,
    )
    .translate(
        CLEAN_USED_GAP_MIN / 2.0 + 56.0,
        -238.0,
        -(SEGREGATION_BARRIER_Z / 2.0 - 4.0),
    );
    barrier - pass_window + clean_rail + used_rail + wipe_shadow
}

fn evidence_camera_bridge() -> Part {
    let left_post = centered_cube(
        "closed_sterile_connector_force_life_camera_bridge_left_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_POST_Z,
    )
    .translate(-BRIDGE_SPAN_X / 2.0, 0.0, 0.0);
    let right_post = centered_cube(
        "closed_sterile_connector_force_life_camera_bridge_right_post",
        BRIDGE_POST_X,
        BRIDGE_POST_Y,
        BRIDGE_POST_Z,
    )
    .translate(BRIDGE_SPAN_X / 2.0, 0.0, 0.0);
    let beam = centered_cube(
        "closed_sterile_connector_force_life_camera_bridge_beam",
        BRIDGE_SPAN_X + BRIDGE_POST_X,
        BRIDGE_BEAM_Y,
        BRIDGE_BEAM_Z,
    )
    .translate(0.0, 0.0, BRIDGE_POST_Z / 2.0 + BRIDGE_BEAM_Z / 2.0);
    let camera_mount = centered_cube(
        "closed_sterile_connector_force_life_camera_mount_land",
        218.0,
        36.0,
        8.0,
    )
    .translate(0.0, 0.0, CAMERA_CLEARANCE_Z - BRIDGE_POST_Z / 2.0);
    let light_bar = centered_cube(
        "closed_sterile_connector_force_life_evidence_light_bar",
        BRIDGE_SPAN_X - 180.0,
        14.0,
        12.0,
    )
    .translate(
        0.0,
        -(BRIDGE_BEAM_Y / 2.0 + 10.0),
        CAMERA_CLEARANCE_Z - BRIDGE_POST_Z / 2.0 - 22.0,
    );
    left_post + right_post + beam + camera_mount + light_bar
}

fn robot_gripper_datum_blocks() -> Part {
    let mut blocks = Part::empty("closed_sterile_connector_force_life_robot_gripper_datum_blocks");
    for (i, (x, y)) in gripper_datum_points().iter().enumerate() {
        let block = centered_cube(
            format!("closed_sterile_connector_force_life_robot_datum_block_{i}"),
            GRIPPER_BLOCK_X,
            GRIPPER_BLOCK_Y,
            GRIPPER_BLOCK_Z,
        )
        .translate(*x, *y, 0.0);
        let pin = centered_cylinder(
            format!("closed_sterile_connector_force_life_robot_datum_pin_{i}"),
            GRIPPER_PIN_D / 2.0,
            16.0,
            28,
        )
        .translate(*x, *y, GRIPPER_BLOCK_Z / 2.0 + 8.0);
        let approach_face = centered_cube(
            format!("closed_sterile_connector_force_life_robot_datum_approach_face_{i}"),
            GRIPPER_BLOCK_X - 16.0,
            5.0,
            8.0,
        )
        .translate(
            *x,
            *y - GRIPPER_BLOCK_Y / 2.0 - 3.0,
            GRIPPER_BLOCK_Z / 2.0 + 4.0,
        );
        blocks = blocks + block + pin + approach_face;
    }
    blocks
}

fn service_keepout_gauges() -> Part {
    let mut gauges = Part::empty("closed_sterile_connector_force_life_service_keepout_gauges");
    for (i, (name, x, y, w, h)) in [
        (
            "front_robot_pick_sweep",
            0.0,
            FRONT_ROBOT_SWEEP_Y,
            STATION_X - 150.0,
            10.0,
        ),
        (
            "rear_pressure_service_sweep",
            0.0,
            REAR_SERVICE_SWEEP_Y,
            STATION_X - 148.0,
            10.0,
        ),
        ("left_clean_material_lane", -526.0, 0.0, 10.0, 646.0),
        ("right_used_return_lane", 526.0, 0.0, 10.0, 646.0),
        (
            "camera_bridge_lift_envelope",
            0.0,
            -316.0,
            BRIDGE_SPAN_X - 96.0,
            10.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        let rail = centered_cube(
            format!("closed_sterile_connector_force_life_keepout_{i}_{name}"),
            *w,
            *h,
            KEEP_OUT_RAIL_Z,
        )
        .translate(*x, *y, 0.0);
        let gauge_flag = centered_cube(
            format!("closed_sterile_connector_force_life_keepout_{i}_{name}_height_flag"),
            32.0,
            12.0,
            SERVICE_GAUGE_Z,
        )
        .translate(*x + *w / 2.0 - 28.0, *y, SERVICE_GAUGE_Z / 2.0);
        gauges = gauges + rail + gauge_flag;
    }
    let centerline = centered_cube(
        "closed_sterile_connector_force_life_keepout_centerline_x",
        STATION_X - 170.0,
        4.0,
        KEEP_OUT_RAIL_Z,
    ) + centered_cube(
        "closed_sterile_connector_force_life_keepout_centerline_y",
        4.0,
        STATION_Y - 170.0,
        KEEP_OUT_RAIL_Z,
    );
    gauges + centerline
}

fn fiducial_disc(name: &str) -> Part {
    let disc = centered_cylinder(format!("{name}_disc"), 5.0, 2.0, 32);
    let center = centered_cylinder(format!("{name}_center"), 1.2, 3.0, 18);
    disc - center
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn connector_center(row: usize, col: usize) -> (f64, f64) {
    (
        centered_index(col, NEST_COLS, CONNECTOR_PITCH_X),
        centered_index(row, NEST_ROWS, CONNECTOR_PITCH_Y),
    )
}

fn gripper_datum_points() -> [(f64, f64); GRIPPER_DATUM_BLOCKS] {
    [
        (-500.0, 324.0),
        (-118.0, 324.0),
        (166.0, 324.0),
        (500.0, 324.0),
        (-500.0, -324.0),
        (500.0, -324.0),
    ]
}

fn insert_rects() -> [(&'static str, Rect); 8] {
    [
        ("connector_nest_arrays", nest_rect()),
        ("actuation_force_gauge_pocket", gauge_rect()),
        (
            "cycle_counter_token_lane",
            Rect {
                x: CYCLE_POS.0,
                y: CYCLE_POS.1,
                w: CYCLE_X,
                h: CYCLE_Y,
            },
        ),
        (
            "cap_plug_parks",
            Rect {
                x: CAP_PLUG_POS.0,
                y: CAP_PLUG_POS.1,
                w: CAP_PLUG_X,
                h: CAP_PLUG_Y,
            },
        ),
        ("pressure_decay_witness_ports", pressure_rect()),
        (
            "barcode_coa_lands",
            Rect {
                x: TRACE_POS.0,
                y: TRACE_POS.1,
                w: TRACE_X,
                h: TRACE_Y,
            },
        ),
        ("retain_sample_pockets", retain_rect()),
        (
            "release_hold_reject_lanes",
            Rect {
                x: STATUS_POS.0,
                y: STATUS_POS.1,
                w: STATUS_LANES as f64 * STATUS_LANE_PITCH_X,
                h: STATUS_LANE_Y,
            },
        ),
    ]
}

fn nest_rect() -> Rect {
    Rect {
        x: NEST_POS.0,
        y: NEST_POS.1,
        w: NEST_X,
        h: NEST_Y,
    }
}

fn gauge_rect() -> Rect {
    Rect {
        x: GAUGE_POS.0,
        y: GAUGE_POS.1,
        w: GAUGE_X,
        h: GAUGE_Y,
    }
}

fn pressure_rect() -> Rect {
    Rect {
        x: PRESSURE_POS.0,
        y: PRESSURE_POS.1,
        w: PRESSURE_X,
        h: PRESSURE_Y,
    }
}

fn retain_rect() -> Rect {
    Rect {
        x: RETAIN_POS.0,
        y: RETAIN_POS.1,
        w: RETAIN_X,
        h: RETAIN_Y,
    }
}

fn clean_rect() -> Rect {
    Rect {
        x: NEST_POS.0,
        y: NEST_POS.1,
        w: NEST_X,
        h: NEST_Y,
    }
}

fn used_rect() -> Rect {
    Rect {
        x: CAP_PLUG_POS.0,
        y: CAP_PLUG_POS.1,
        w: CAP_PLUG_X,
        h: CAP_PLUG_Y,
    }
}

fn fits_on_station(rect: Rect) -> bool {
    rect.x.abs() + rect.w / 2.0 <= STATION_X / 2.0 - RIM_W - 8.0
        && rect.y.abs() + rect.h / 2.0 <= STATION_Y / 2.0 - RIM_W - 8.0
}

fn rects_overlap(a: Rect, b: Rect) -> bool {
    let a_left = a.x - a.w / 2.0;
    let a_right = a.x + a.w / 2.0;
    let a_bottom = a.y - a.h / 2.0;
    let a_top = a.y + a.h / 2.0;
    let b_left = b.x - b.w / 2.0;
    let b_right = b.x + b.w / 2.0;
    let b_bottom = b.y - b.h / 2.0;
    let b_top = b.y + b.h / 2.0;

    !(a_right <= b_left || b_right <= a_left || a_top <= b_bottom || b_top <= a_bottom)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_scoped_and_stable() {
        assert_design_constraints();

        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 14);
        for path in OUTPUTS {
            assert!(
                path.starts_with("output/closed_sterile_connector_actuation_force_life_station_"),
                "unscoped output path: {path}"
            );
            assert!(path.ends_with(".stl"), "{path}");
        }
        assert_eq!(
            OUTPUTS[0],
            "output/closed_sterile_connector_actuation_force_life_station_base_leak_tray_deck.stl"
        );
        assert!(OUTPUTS[OUTPUTS.len() - 1].ends_with("_assembly.stl"));
    }

    #[test]
    fn required_features_cover_requested_station_scope() {
        for feature in [
            "base_leak_tray_deck",
            "connector_nest_arrays",
            "actuation_force_gauge_pocket",
            "cycle_counter_token_lane",
            "cap_plug_parks",
            "pressure_decay_witness_ports",
            "barcode_coa_lands",
            "retain_sample_pockets",
            "release_hold_reject_lanes",
            "clean_used_segregation",
            "evidence_camera_bridge",
            "robot_gripper_datum_blocks",
            "service_keepout_gauges",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
    }

    #[test]
    fn connector_capacity_matches_force_and_cycle_plan() {
        assert_eq!(NEST_ROWS, 3);
        assert_eq!(NEST_COLS, 6);
        assert_eq!(CONNECTOR_COUNT, 18);
        assert!(CYCLE_TOKEN_COUNT >= CONNECTOR_COUNT);
        assert!(CAP_WELLS >= CONNECTOR_COUNT);
        assert!(PLUG_WELLS >= CONNECTOR_COUNT);
        assert_eq!(CYCLE_CLICK_TRACKS, 3);
    }

    #[test]
    fn actuation_force_and_pressure_decay_features_are_sized() {
        assert_eq!(CAL_WEIGHT_POCKETS, 4);
        assert!(FORCE_GAUGE_X < GAUGE_X);
        assert!(FORCE_GAUGE_Y < GAUGE_Y);
        assert!(FORCE_PLUNGER_D < FORCE_ANVIL_D);
        assert_eq!(PRESSURE_PORTS, WITNESS_WELLS);
        assert!(PRESSURE_PORTS >= CONNECTOR_COUNT / 2);
    }

    #[test]
    fn station_modules_fit_and_do_not_collide() {
        for (_name, rect) in insert_rects() {
            assert!(fits_on_station(rect));
        }
        assert!(!rects_overlap(nest_rect(), gauge_rect()));
        assert!(!rects_overlap(pressure_rect(), retain_rect()));
        assert!(STATION_X <= 1150.0);
        assert!(STATION_Y <= 840.0);
        assert!(RIM_Z >= 40.0);
    }

    #[test]
    fn connector_positions_are_centered_in_nest() {
        let (left_x, low_y) = connector_center(0, 0);
        let (right_x, high_y) = connector_center(NEST_ROWS - 1, NEST_COLS - 1);
        assert!((left_x + right_x).abs() < 0.001);
        assert!((low_y + high_y).abs() < 0.001);
        assert!(right_x.abs() + CONNECTOR_SLOT_X / 2.0 < NEST_X / 2.0 - 36.0);
        assert!(high_y.abs() + CONNECTOR_SLOT_Y / 2.0 < NEST_Y / 2.0 - 34.0);
    }

    #[test]
    fn traceability_retain_and_disposition_capacity_are_explicit() {
        assert_eq!(BARCODE_LANDS, 6);
        assert_eq!(COA_LANDS, 3);
        assert_eq!(OPERATOR_SIGNOFF_LANDS, 2);
        assert_eq!(RETAIN_POCKETS, 12);
        assert_eq!(STATUS_LANES, 3);
        assert_eq!(STATUS_SLOT_COUNT * STATUS_LANES, CONNECTOR_COUNT);
    }

    #[test]
    fn clean_used_camera_robot_and_service_assumptions_are_visible() {
        assert!(SEGREGATION_BARRIER_Z >= 80.0);
        assert!(CLEAN_USED_GAP_MIN >= 90.0);
        assert!(BRIDGE_POST_Z > CAMERA_CLEARANCE_Z);
        assert_eq!(GRIPPER_DATUM_BLOCKS, 6);
        assert_eq!(SERVICE_KEEP_OUT_GAUGES, 5);
        for (x, y) in gripper_datum_points() {
            assert!(x.abs() + GRIPPER_BLOCK_X / 2.0 < STATION_X / 2.0 - 18.0);
            assert!(y.abs() + GRIPPER_BLOCK_Y / 2.0 < STATION_Y / 2.0 - 18.0);
        }
    }
}
