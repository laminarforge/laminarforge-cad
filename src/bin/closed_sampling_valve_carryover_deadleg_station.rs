use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed sampling-valve carryover and dead-leg validation station.
//
// Intent:
// - Validate sampling valve coupon geometry, challenge standard separation,
//   sample-loop hold-up, and dead-leg witness visibility before any live-cell
//   or release-relevant article is routed through the sampling path.
// - Keep high and low standards physically segregated from disposition lanes
//   while preserving a closed flush/waste manifold placeholder and visible
//   pressure/flow evidence points.
// - Provide dry traceability lands, evidence capture geometry, and robot plus
//   service keepout gauges for validation workflow planning.
//
// This is architecture/fit CAD only. It is not a wetted-path specification,
// analytical method, sterile barrier design, or acceptance criterion.

const OUTPUTS: &[&str] = &[
    "output/closed_sampling_valve_carryover_deadleg_station_base_leak_tray.stl",
    "output/closed_sampling_valve_carryover_deadleg_station_valve_coupon_nests.stl",
    "output/closed_sampling_valve_carryover_deadleg_station_high_low_standard_challenge_lanes.stl",
    "output/closed_sampling_valve_carryover_deadleg_station_deadleg_transparent_witness_pockets.stl",
    "output/closed_sampling_valve_carryover_deadleg_station_flush_waste_manifold_placeholder.stl",
    "output/closed_sampling_valve_carryover_deadleg_station_sample_loop_volume_wells.stl",
    "output/closed_sampling_valve_carryover_deadleg_station_pressure_flow_witness_ports.stl",
    "output/closed_sampling_valve_carryover_deadleg_station_barcode_certificate_lands.stl",
    "output/closed_sampling_valve_carryover_deadleg_station_release_hold_reject_lanes.stl",
    "output/closed_sampling_valve_carryover_deadleg_station_evidence_bridge.stl",
    "output/closed_sampling_valve_carryover_deadleg_station_robot_service_keepout_gauges.stl",
    "output/closed_sampling_valve_carryover_deadleg_station_assembly.stl",
];

const REQUIRED_FEATURES: &[&str] = &[
    "valve_coupon_nests",
    "high_low_standard_challenge_lanes",
    "deadleg_transparent_witness_pockets",
    "flush_waste_manifold_placeholder",
    "sample_loop_volume_wells",
    "pressure_flow_witness_ports",
    "barcode_certificate_lands",
    "release_hold_reject_lanes",
    "evidence_bridge",
    "robot_service_keepout_gauges",
    "closed_leak_tray",
    "assembly_export",
];

const STATION_X: f64 = 1260.0;
const STATION_Y: f64 = 820.0;
const BASE_Z: f64 = 22.0;
const LEAK_BASIN_X: f64 = STATION_X - 118.0;
const LEAK_BASIN_Y: f64 = STATION_Y - 108.0;
const LEAK_BASIN_DEPTH: f64 = 7.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 42.0;
const SOCKET_DEPTH: f64 = 5.6;
const DRAIN_D: f64 = 16.0;
const MOUNT_HOLE_D: f64 = 6.6;
const LEAK_SENSOR_WELLS: usize = 5;

const VALVE_NEST_X: f64 = 520.0;
const VALVE_NEST_Y: f64 = 190.0;
const VALVE_NEST_Z: f64 = 48.0;
const VALVE_COUPON_NESTS: usize = 8;
const VALVE_PITCH_X: f64 = 60.0;
const VALVE_COUPON_POCKET_X: f64 = 42.0;
const VALVE_COUPON_POCKET_Y: f64 = 92.0;
const VALVE_COUPON_POCKET_Z: f64 = 34.0;
const VALVE_ACTUATOR_D: f64 = 25.0;
const VALVE_TUBE_BORE_D: f64 = 5.4;

const STANDARD_RACK_X: f64 = 420.0;
const STANDARD_RACK_Y: f64 = 170.0;
const STANDARD_RACK_Z: f64 = 42.0;
const STANDARD_LANES: usize = 2;
const STANDARD_POSITIONS_PER_LANE: usize = 6;
const STANDARD_POSITIONS: usize = STANDARD_LANES * STANDARD_POSITIONS_PER_LANE;
const STANDARD_PITCH_X: f64 = 55.0;
const STANDARD_LANE_PITCH_Y: f64 = 62.0;
const HIGH_STANDARD_WELL_D: f64 = 22.0;
const LOW_STANDARD_WELL_D: f64 = 17.0;
const STANDARD_LANE_MIN_SEPARATION: f64 = 46.0;

const DEADLEG_PANEL_X: f64 = 540.0;
const DEADLEG_PANEL_Y: f64 = 150.0;
const DEADLEG_PANEL_Z: f64 = 34.0;
const DEADLEG_WITNESS_POCKETS: usize = VALVE_COUPON_NESTS;
const DEADLEG_PITCH_X: f64 = 62.0;
const DEADLEG_WINDOW_X: f64 = 40.0;
const DEADLEG_WINDOW_Y: f64 = 18.0;
const DEADLEG_WINDOW_Z: f64 = 25.0;
const DEADLEG_STUB_D: f64 = 4.2;
const DEADLEG_STUB_LENGTHS: [f64; 4] = [8.0, 18.0, 32.0, 48.0];

const MANIFOLD_X: f64 = 430.0;
const MANIFOLD_Y: f64 = 150.0;
const MANIFOLD_Z: f64 = 58.0;
const MANIFOLD_FLUSH_PORTS: usize = VALVE_COUPON_NESTS;
const MANIFOLD_WASTE_PORTS: usize = VALVE_COUPON_NESTS;
const MANIFOLD_PORT_PITCH_X: f64 = 46.0;
const FLUSH_PORT_D: f64 = 5.2;
const WASTE_PORT_D: f64 = 8.2;
const WASTE_TROUGH_X: f64 = 370.0;
const WASTE_TROUGH_Y: f64 = 28.0;
const WASTE_TROUGH_Z: f64 = 18.0;

const LOOP_WELL_BANK_X: f64 = 500.0;
const LOOP_WELL_BANK_Y: f64 = 120.0;
const LOOP_WELL_BANK_Z: f64 = 30.0;
const LOOP_WELL_COLS: usize = 5;
const LOOP_WELL_ROWS: usize = 2;
const LOOP_VOLUME_WELLS: usize = LOOP_WELL_COLS * LOOP_WELL_ROWS;
const LOOP_WELL_PITCH_X: f64 = 74.0;
const LOOP_WELL_PITCH_Y: f64 = 52.0;
const LOOP_WELL_BASE_D: f64 = 12.0;
const LOOP_WELL_STEP_D: f64 = 2.4;

const WITNESS_BAR_X: f64 = 430.0;
const WITNESS_BAR_Y: f64 = 130.0;
const WITNESS_BAR_Z: f64 = 36.0;
const WITNESS_PORT_PAIRS: usize = 6;
const WITNESS_PORT_PITCH_X: f64 = 60.0;
const PRESSURE_TAP_D: f64 = 5.6;
const FLOW_WINDOW_X: f64 = 28.0;
const FLOW_WINDOW_Y: f64 = 8.0;
const FLOW_WINDOW_Z: f64 = 16.0;

const TRACE_PANEL_X: f64 = 250.0;
const TRACE_PANEL_Y: f64 = 95.0;
const TRACE_PANEL_Z: f64 = 10.0;
const BARCODE_LANDS: usize = 8;
const CERTIFICATE_LANDS: usize = 4;
const BARCODE_LAND_X: f64 = 58.0;
const BARCODE_LAND_Y: f64 = 16.0;
const CERTIFICATE_LAND_X: f64 = 96.0;
const CERTIFICATE_LAND_Y: f64 = 24.0;

const DISPOSITION_BANK_X: f64 = 590.0;
const DISPOSITION_BANK_Y: f64 = 100.0;
const DISPOSITION_BANK_Z: f64 = 22.0;
const DISPOSITION_LANES: usize = 3;
const DISPOSITION_LANE_X: f64 = 168.0;
const DISPOSITION_LANE_Y: f64 = 76.0;
const DISPOSITION_LANE_PITCH_X: f64 = 188.0;
const DISPOSITION_GATE_Z: f64 = 42.0;

const EVIDENCE_BRIDGE_X: f64 = 1060.0;
const EVIDENCE_BRIDGE_Y: f64 = 70.0;
const EVIDENCE_POST_Z: f64 = 205.0;
const EVIDENCE_POST_X: f64 = 28.0;
const EVIDENCE_POST_Y: f64 = 42.0;
const EVIDENCE_BEAM_Z: f64 = 26.0;
const EVIDENCE_CAMERAS: usize = 3;
const EVIDENCE_LIGHT_BARS: usize = 4;

const ROBOT_KEEP_OUT_Z: f64 = 176.0;
const ROBOT_KEEP_OUT_WINDOWS: usize = 4;
const FRONT_SERVICE_CLEARANCE: f64 = 360.0;
const REAR_SERVICE_CLEARANCE: f64 = 230.0;
const RIGHT_MANIFOLD_SERVICE_CLEARANCE: f64 = 190.0;
const LEFT_TRACE_SERVICE_CLEARANCE: f64 = 150.0;

const VALVE_POS: (f64, f64) = (-325.0, 150.0);
const STANDARD_POS: (f64, f64) = (330.0, 160.0);
const DEADLEG_POS: (f64, f64) = (-335.0, -75.0);
const MANIFOLD_POS: (f64, f64) = (335.0, -45.0);
const LOOP_POS: (f64, f64) = (-330.0, -265.0);
const WITNESS_POS: (f64, f64) = (325.0, -245.0);
const TRACE_POS: (f64, f64) = (-475.0, 330.0);
const DISPOSITION_POS: (f64, f64) = (30.0, 330.0);
const EVIDENCE_POS: (f64, f64) = (0.0, 30.0);

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_leak_tray();
    export(&base, OUTPUTS[0]);

    let valve_nests = valve_coupon_nests();
    export(&valve_nests, OUTPUTS[1]);

    let standards = high_low_standard_challenge_lanes();
    export(&standards, OUTPUTS[2]);

    let deadlegs = deadleg_transparent_witness_pockets();
    export(&deadlegs, OUTPUTS[3]);

    let manifold = flush_waste_manifold_placeholder();
    export(&manifold, OUTPUTS[4]);

    let loop_wells = sample_loop_volume_wells();
    export(&loop_wells, OUTPUTS[5]);

    let witness = pressure_flow_witness_ports();
    export(&witness, OUTPUTS[6]);

    let trace = barcode_certificate_lands();
    export(&trace, OUTPUTS[7]);

    let disposition = release_hold_reject_lanes();
    export(&disposition, OUTPUTS[8]);

    let bridge = evidence_bridge();
    export(&bridge, OUTPUTS[9]);

    let keepouts = robot_service_keepout_gauges();
    export(&keepouts, OUTPUTS[10]);

    let assembly = base
        + valve_nests
        + standards
        + deadlegs
        + manifold
        + loop_wells
        + witness
        + trace
        + disposition
        + bridge
        + keepouts
        + fluid_route_placeholders();
    export(&assembly, OUTPUTS[11]);

    println!(
        "Closed sampling-valve carryover/dead-leg validation station: {:.0}mm x {:.0}mm leak-tray deck, {} valve coupon nests, {} high/low standard challenge positions, {} dead-leg witness pockets, {} sample-loop volume wells, {} pressure/flow witness pairs, and {} flush/waste manifold ports.",
        STATION_X,
        STATION_Y,
        VALVE_COUPON_NESTS,
        STANDARD_POSITIONS,
        DEADLEG_WITNESS_POCKETS,
        LOOP_VOLUME_WELLS,
        WITNESS_PORT_PAIRS,
        MANIFOLD_FLUSH_PORTS + MANIFOLD_WASTE_PORTS
    );
    println!(
        "Traceability and disposition: {} barcode/certificate lands, release/hold/reject lanes, {} evidence cameras, {} light bars, {} leak sensor wells, and {} robot/service keepout gauges.",
        BARCODE_LANDS + CERTIFICATE_LANDS,
        EVIDENCE_CAMERAS,
        EVIDENCE_LIGHT_BARS,
        LEAK_SENSOR_WELLS,
        ROBOT_KEEP_OUT_WINDOWS
    );
    println!(
        "Layout checks: high/low lane separation {:.0}mm, front service clearance {:.0}mm, rear service clearance {:.0}mm, right manifold service clearance {:.0}mm, left trace service clearance {:.0}mm, and {} required feature groups.",
        STANDARD_LANE_PITCH_Y,
        FRONT_SERVICE_CLEARANCE,
        REAR_SERVICE_CLEARANCE,
        RIGHT_MANIFOLD_SERVICE_CLEARANCE,
        LEFT_TRACE_SERVICE_CLEARANCE,
        REQUIRED_FEATURES.len()
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_layout() {
    assert_eq!(DEADLEG_WITNESS_POCKETS, VALVE_COUPON_NESTS);
    assert_eq!(DISPOSITION_LANES, 3);
    assert!(STANDARD_LANE_PITCH_Y >= STANDARD_LANE_MIN_SEPARATION);
    assert!(MANIFOLD_FLUSH_PORTS == MANIFOLD_WASTE_PORTS);
    assert!(LOOP_VOLUME_WELLS >= VALVE_COUPON_NESTS);

    for (name, center, width, depth) in fixture_specs() {
        assert!(
            fits_on_station(center, width, depth, 24.0),
            "{name} exceeds station envelope"
        );
    }

    let valve = rect(VALVE_POS, VALVE_NEST_X, VALVE_NEST_Y);
    let standards = rect(STANDARD_POS, STANDARD_RACK_X, STANDARD_RACK_Y);
    let deadlegs = rect(DEADLEG_POS, DEADLEG_PANEL_X, DEADLEG_PANEL_Y);
    let manifold = rect(MANIFOLD_POS, MANIFOLD_X, MANIFOLD_Y);
    let loops = rect(LOOP_POS, LOOP_WELL_BANK_X, LOOP_WELL_BANK_Y);
    let witness = rect(WITNESS_POS, WITNESS_BAR_X, WITNESS_BAR_Y);
    let trace = rect(TRACE_POS, TRACE_PANEL_X, TRACE_PANEL_Y);
    let disposition = rect(DISPOSITION_POS, DISPOSITION_BANK_X, DISPOSITION_BANK_Y);

    for (left_name, left, right_name, right) in [
        ("valve", valve, "standards", standards),
        ("valve", valve, "deadlegs", deadlegs),
        ("standards", standards, "manifold", manifold),
        ("deadlegs", deadlegs, "loops", loops),
        ("manifold", manifold, "witness", witness),
        ("loops", loops, "witness", witness),
        ("trace", trace, "disposition", disposition),
        ("standards", standards, "disposition", disposition),
    ] {
        assert!(
            !rects_overlap(left, right, 8.0),
            "{left_name} overlaps {right_name}"
        );
    }
}

fn fixture_specs() -> [(&'static str, (f64, f64), f64, f64); 9] {
    [
        ("valve_coupon_nests", VALVE_POS, VALVE_NEST_X, VALVE_NEST_Y),
        (
            "high_low_standard_challenge_lanes",
            STANDARD_POS,
            STANDARD_RACK_X,
            STANDARD_RACK_Y,
        ),
        (
            "deadleg_transparent_witness_pockets",
            DEADLEG_POS,
            DEADLEG_PANEL_X,
            DEADLEG_PANEL_Y,
        ),
        (
            "flush_waste_manifold_placeholder",
            MANIFOLD_POS,
            MANIFOLD_X,
            MANIFOLD_Y,
        ),
        (
            "sample_loop_volume_wells",
            LOOP_POS,
            LOOP_WELL_BANK_X,
            LOOP_WELL_BANK_Y,
        ),
        (
            "pressure_flow_witness_ports",
            WITNESS_POS,
            WITNESS_BAR_X,
            WITNESS_BAR_Y,
        ),
        (
            "barcode_certificate_lands",
            TRACE_POS,
            TRACE_PANEL_X,
            TRACE_PANEL_Y,
        ),
        (
            "release_hold_reject_lanes",
            DISPOSITION_POS,
            DISPOSITION_BANK_X,
            DISPOSITION_BANK_Y,
        ),
        (
            "evidence_bridge",
            EVIDENCE_POS,
            EVIDENCE_BRIDGE_X,
            EVIDENCE_BRIDGE_Y,
        ),
    ]
}

fn base_leak_tray() -> Part {
    let deck = centered_cube(
        "closed_sampling_valve_deadleg_station_base_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    );
    let basin = centered_cube(
        "closed_sampling_valve_deadleg_station_leak_basin_recess",
        LEAK_BASIN_X,
        LEAK_BASIN_Y,
        LEAK_BASIN_DEPTH,
    )
    .translate(0.0, -6.0, BASE_Z / 2.0 - LEAK_BASIN_DEPTH / 2.0 + 0.5);
    let drain = centered_cylinder(
        "closed_sampling_valve_deadleg_station_front_drain_placeholder",
        DRAIN_D / 2.0,
        60.0,
        32,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(STATION_X / 2.0 - 84.0, -STATION_Y / 2.0 - 2.0, 0.0);

    deck - basin - drain - insert_sockets() - mount_slots()
        + leak_tray_rims()
        + wet_dry_custody_dividers()
        + leak_sensor_wells()
        + robot_datum_fiducials()
}

fn insert_sockets() -> Part {
    let mut sockets = Part::empty("closed_sampling_valve_deadleg_station_insert_sockets");
    for (name, center, width, depth) in fixture_specs().iter().take(8) {
        sockets = sockets
            + centered_cube(
                format!("closed_sampling_valve_deadleg_station_{name}_socket"),
                width + 10.0,
                depth + 10.0,
                SOCKET_DEPTH + 0.4,
            )
            .translate(center.0, center.1, BASE_Z / 2.0 - SOCKET_DEPTH / 2.0 + 0.2);
    }
    sockets
}

fn mount_slots() -> Part {
    let mut slots = Part::empty("closed_sampling_valve_deadleg_station_mount_slots");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 54.0), -(STATION_Y / 2.0 - 50.0)),
        (STATION_X / 2.0 - 54.0, -(STATION_Y / 2.0 - 50.0)),
        (-(STATION_X / 2.0 - 54.0), STATION_Y / 2.0 - 50.0),
        (STATION_X / 2.0 - 54.0, STATION_Y / 2.0 - 50.0),
        (0.0, -(STATION_Y / 2.0 - 50.0)),
        (0.0, STATION_Y / 2.0 - 50.0),
    ]
    .iter()
    .enumerate()
    {
        slots = slots
            + centered_cylinder(
                format!("closed_sampling_valve_deadleg_station_m6_mount_bore_{i}"),
                MOUNT_HOLE_D / 2.0,
                BASE_Z + 4.0,
                24,
            )
            .translate(*x, *y, 0.0)
            + centered_cube(
                format!("closed_sampling_valve_deadleg_station_m6_mount_slot_{i}"),
                26.0,
                7.0,
                BASE_Z + 4.0,
            )
            .translate(*x, *y, 0.0);
    }
    slots
}

fn leak_tray_rims() -> Part {
    let left = centered_cube(
        "closed_sampling_valve_deadleg_station_left_leak_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-(STATION_X / 2.0 - RIM_W / 2.0), 0.0, z_on_base(RIM_Z));
    let right = centered_cube(
        "closed_sampling_valve_deadleg_station_right_leak_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, z_on_base(RIM_Z));
    let rear = centered_cube(
        "closed_sampling_valve_deadleg_station_rear_leak_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, z_on_base(RIM_Z));
    let front_low_lip = centered_cube(
        "closed_sampling_valve_deadleg_station_front_low_leak_lip",
        STATION_X - 150.0,
        12.0,
        22.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 18.0, z_on_base(22.0));

    left + right + rear + front_low_lip
}

fn wet_dry_custody_dividers() -> Part {
    let wet_to_disposition = centered_cube(
        "closed_sampling_valve_deadleg_station_wet_to_release_hold_reject_divider",
        STATION_X - 150.0,
        10.0,
        30.0,
    )
    .translate(0.0, 270.0, z_on_base(30.0));
    let dry_trace_divider = centered_cube(
        "closed_sampling_valve_deadleg_station_traceability_dry_zone_divider",
        10.0,
        140.0,
        26.0,
    )
    .translate(-320.0, 330.0, z_on_base(26.0));
    let valve_standard_centerline = centered_cube(
        "closed_sampling_valve_deadleg_station_valve_standard_crossover_keepaway_rib",
        10.0,
        210.0,
        24.0,
    )
    .translate(0.0, 140.0, z_on_base(24.0));
    let lower_wet_divider = centered_cube(
        "closed_sampling_valve_deadleg_station_loop_manifold_keepaway_rib",
        STATION_X - 210.0,
        8.0,
        24.0,
    )
    .translate(0.0, -170.0, z_on_base(24.0));

    wet_to_disposition + dry_trace_divider + valve_standard_centerline + lower_wet_divider
}

fn leak_sensor_wells() -> Part {
    let mut wells = Part::empty("closed_sampling_valve_deadleg_station_leak_sensor_wells");
    for i in 0..LEAK_SENSOR_WELLS {
        let x = centered_index(i, LEAK_SENSOR_WELLS, 185.0);
        let boss = centered_cylinder(
            format!("closed_sampling_valve_deadleg_station_leak_sensor_boss_{i}"),
            15.0,
            5.0,
            32,
        )
        .translate(x, -STATION_Y / 2.0 + 62.0, BASE_Z / 2.0 + 2.5);
        let pocket = centered_cylinder(
            format!("closed_sampling_valve_deadleg_station_leak_sensor_pocket_{i}"),
            7.0,
            7.0,
            28,
        )
        .translate(x, -STATION_Y / 2.0 + 62.0, BASE_Z / 2.0 + 3.0);
        wells = wells + (boss - pocket);
    }
    wells
}

fn robot_datum_fiducials() -> Part {
    let mut datums = Part::empty("closed_sampling_valve_deadleg_station_robot_datum_fiducials");
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 88.0), STATION_Y / 2.0 - 82.0),
        (STATION_X / 2.0 - 88.0, STATION_Y / 2.0 - 82.0),
        (-(STATION_X / 2.0 - 88.0), -(STATION_Y / 2.0 - 82.0)),
        (STATION_X / 2.0 - 88.0, -(STATION_Y / 2.0 - 82.0)),
    ]
    .iter()
    .enumerate()
    {
        let disc = centered_cylinder(
            format!("closed_sampling_valve_deadleg_station_datum_disc_{i}"),
            13.0,
            4.0,
            32,
        )
        .translate(*x, *y, BASE_Z / 2.0 + 2.0);
        let center = centered_cylinder(
            format!("closed_sampling_valve_deadleg_station_datum_center_bore_{i}"),
            3.0,
            6.0,
            20,
        )
        .translate(*x, *y, BASE_Z / 2.0 + 2.0);
        datums = datums + (disc - center);
    }
    datums
}

fn valve_coupon_nests() -> Part {
    let rack = centered_cube(
        "closed_sampling_valve_deadleg_station_valve_coupon_nest_rack",
        VALVE_NEST_X,
        VALVE_NEST_Y,
        VALVE_NEST_Z,
    )
    .translate(VALVE_POS.0, VALVE_POS.1, z_on_base(VALVE_NEST_Z));
    let mut cutters = Part::empty("closed_sampling_valve_deadleg_station_valve_nest_cutters");
    let mut features = Part::empty("closed_sampling_valve_deadleg_station_valve_nest_features");

    for lane in 0..VALVE_COUPON_NESTS {
        let x = VALVE_POS.0 + centered_index(lane, VALVE_COUPON_NESTS, VALVE_PITCH_X);
        let pocket = centered_cube(
            format!("closed_sampling_valve_deadleg_station_valve_coupon_pocket_{lane}"),
            VALVE_COUPON_POCKET_X,
            VALVE_COUPON_POCKET_Y,
            VALVE_COUPON_POCKET_Z,
        )
        .translate(x, VALVE_POS.1, BASE_Z / 2.0 + VALVE_NEST_Z - 13.0);
        let actuator = centered_cylinder(
            format!("closed_sampling_valve_deadleg_station_valve_actuator_access_{lane}"),
            VALVE_ACTUATOR_D / 2.0,
            VALVE_NEST_Z + 4.0,
            32,
        )
        .translate(x, VALVE_POS.1 + 28.0, z_on_base(VALVE_NEST_Z));
        let tube_bore = centered_cylinder(
            format!("closed_sampling_valve_deadleg_station_valve_coupon_tube_bore_{lane}"),
            VALVE_TUBE_BORE_D / 2.0,
            VALVE_COUPON_POCKET_Y + 34.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(x, VALVE_POS.1, BASE_Z / 2.0 + 18.0);
        let inlet_key = centered_cube(
            format!("closed_sampling_valve_deadleg_station_valve_coupon_inlet_key_{lane}"),
            18.0,
            8.0,
            10.0,
        )
        .translate(
            x,
            VALVE_POS.1 - VALVE_COUPON_POCKET_Y / 2.0 - 6.0,
            BASE_Z / 2.0 + VALVE_NEST_Z + 5.0,
        );
        let outlet_key = centered_cube(
            format!("closed_sampling_valve_deadleg_station_valve_coupon_outlet_key_{lane}"),
            18.0,
            8.0,
            10.0,
        )
        .translate(
            x,
            VALVE_POS.1 + VALVE_COUPON_POCKET_Y / 2.0 + 6.0,
            BASE_Z / 2.0 + VALVE_NEST_Z + 5.0,
        );
        let latch_left = centered_cube(
            format!("closed_sampling_valve_deadleg_station_valve_coupon_left_latch_{lane}"),
            6.0,
            78.0,
            12.0,
        )
        .translate(
            x - VALVE_COUPON_POCKET_X / 2.0 - 5.0,
            VALVE_POS.1,
            BASE_Z / 2.0 + VALVE_NEST_Z + 6.0,
        );
        let latch_right = centered_cube(
            format!("closed_sampling_valve_deadleg_station_valve_coupon_right_latch_{lane}"),
            6.0,
            78.0,
            12.0,
        )
        .translate(
            x + VALVE_COUPON_POCKET_X / 2.0 + 5.0,
            VALVE_POS.1,
            BASE_Z / 2.0 + VALVE_NEST_Z + 6.0,
        );

        cutters = cutters + pocket + actuator + tube_bore;
        features = features + inlet_key + outlet_key + latch_left + latch_right;
    }

    rack - cutters + features + valve_lane_index_ticks()
}

fn valve_lane_index_ticks() -> Part {
    let mut ticks = Part::empty("closed_sampling_valve_deadleg_station_valve_lane_index_ticks");
    for lane in 0..=VALVE_COUPON_NESTS {
        let x = VALVE_POS.0 - VALVE_PITCH_X * VALVE_COUPON_NESTS as f64 / 2.0
            + lane as f64 * VALVE_PITCH_X;
        ticks = ticks
            + centered_cube(
                format!("closed_sampling_valve_deadleg_station_valve_lane_tick_{lane}"),
                3.0,
                VALVE_NEST_Y - 18.0,
                7.0,
            )
            .translate(x, VALVE_POS.1, BASE_Z / 2.0 + VALVE_NEST_Z + 3.5);
    }
    ticks
}

fn high_low_standard_challenge_lanes() -> Part {
    let rack = centered_cube(
        "closed_sampling_valve_deadleg_station_standard_challenge_lane_rack",
        STANDARD_RACK_X,
        STANDARD_RACK_Y,
        STANDARD_RACK_Z,
    )
    .translate(STANDARD_POS.0, STANDARD_POS.1, z_on_base(STANDARD_RACK_Z));
    let mut cutters = Part::empty("closed_sampling_valve_deadleg_station_standard_well_cutters");
    let mut features = Part::empty("closed_sampling_valve_deadleg_station_standard_lane_features");

    for lane in 0..STANDARD_LANES {
        let lane_y = STANDARD_POS.1 + centered_index(lane, STANDARD_LANES, STANDARD_LANE_PITCH_Y);
        let is_high = lane == 1;
        let well_d = if is_high {
            HIGH_STANDARD_WELL_D
        } else {
            LOW_STANDARD_WELL_D
        };
        let lane_rail = centered_cube(
            format!("closed_sampling_valve_deadleg_station_standard_lane_rail_{lane}"),
            STANDARD_RACK_X - 34.0,
            8.0,
            10.0,
        )
        .translate(
            STANDARD_POS.0,
            lane_y + STANDARD_LANE_PITCH_Y / 2.0 - 6.0,
            BASE_Z / 2.0 + STANDARD_RACK_Z + 5.0,
        );
        features = features + lane_rail;

        for pos in 0..STANDARD_POSITIONS_PER_LANE {
            let x =
                STANDARD_POS.0 + centered_index(pos, STANDARD_POSITIONS_PER_LANE, STANDARD_PITCH_X);
            let well = centered_cylinder(
                format!(
                    "closed_sampling_valve_deadleg_station_{}_standard_well_{}",
                    if is_high { "high" } else { "low" },
                    pos
                ),
                well_d / 2.0,
                STANDARD_RACK_Z + 8.0,
                36,
            )
            .translate(x, lane_y, z_on_base(STANDARD_RACK_Z));
            let splash_ring = centered_cylinder(
                format!(
                    "closed_sampling_valve_deadleg_station_{}_standard_splash_ring_{}",
                    if is_high { "high" } else { "low" },
                    pos
                ),
                well_d / 2.0 + 5.0,
                4.0,
                36,
            )
            .translate(x, lane_y, BASE_Z / 2.0 + STANDARD_RACK_Z + 2.0);
            let center_cut = centered_cylinder(
                format!(
                    "closed_sampling_valve_deadleg_station_{}_standard_ring_opening_{}",
                    if is_high { "high" } else { "low" },
                    pos
                ),
                well_d / 2.0,
                5.0,
                32,
            )
            .translate(x, lane_y, BASE_Z / 2.0 + STANDARD_RACK_Z + 2.0);
            cutters = cutters + well;
            features = features + (splash_ring - center_cut);
        }
    }

    let lane_separator = centered_cube(
        "closed_sampling_valve_deadleg_station_high_low_standard_lane_separator",
        STANDARD_RACK_X - 20.0,
        6.0,
        16.0,
    )
    .translate(
        STANDARD_POS.0,
        STANDARD_POS.1,
        BASE_Z / 2.0 + STANDARD_RACK_Z + 8.0,
    );
    let challenge_order_land = centered_cube(
        "closed_sampling_valve_deadleg_station_alternating_challenge_order_land",
        92.0,
        36.0,
        6.0,
    )
    .translate(
        STANDARD_POS.0 + STANDARD_RACK_X / 2.0 - 58.0,
        STANDARD_POS.1,
        BASE_Z / 2.0 + STANDARD_RACK_Z + 3.0,
    );

    rack - cutters + features + lane_separator + challenge_order_land
}

fn deadleg_transparent_witness_pockets() -> Part {
    let rail = centered_cube(
        "closed_sampling_valve_deadleg_station_deadleg_witness_panel",
        DEADLEG_PANEL_X,
        DEADLEG_PANEL_Y,
        DEADLEG_PANEL_Z,
    )
    .translate(DEADLEG_POS.0, DEADLEG_POS.1, z_on_base(DEADLEG_PANEL_Z));
    let mut pockets = Part::empty("closed_sampling_valve_deadleg_station_deadleg_witness_pockets");

    for lane in 0..DEADLEG_WITNESS_POCKETS {
        let x = DEADLEG_POS.0 + centered_index(lane, DEADLEG_WITNESS_POCKETS, DEADLEG_PITCH_X);
        let frame = centered_cube(
            format!("closed_sampling_valve_deadleg_station_transparent_window_frame_{lane}"),
            DEADLEG_WINDOW_X + 12.0,
            DEADLEG_WINDOW_Y + 16.0,
            DEADLEG_WINDOW_Z + 10.0,
        )
        .translate(x, DEADLEG_POS.1, BASE_Z / 2.0 + DEADLEG_PANEL_Z + 14.0);
        let window_void = centered_cube(
            format!("closed_sampling_valve_deadleg_station_transparent_window_opening_{lane}"),
            DEADLEG_WINDOW_X,
            DEADLEG_WINDOW_Y,
            DEADLEG_WINDOW_Z,
        )
        .translate(x, DEADLEG_POS.1, BASE_Z / 2.0 + DEADLEG_PANEL_Z + 14.0);
        let length = DEADLEG_STUB_LENGTHS[lane % DEADLEG_STUB_LENGTHS.len()];
        let live_path = centered_cylinder(
            format!("closed_sampling_valve_deadleg_station_live_path_witness_bore_{lane}"),
            DEADLEG_STUB_D / 2.0,
            DEADLEG_WINDOW_X + 20.0,
            20,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(x, DEADLEG_POS.1, BASE_Z / 2.0 + DEADLEG_PANEL_Z + 12.0);
        let blind_branch = centered_cylinder(
            format!("closed_sampling_valve_deadleg_station_blind_deadleg_stub_{lane}"),
            DEADLEG_STUB_D / 2.0,
            length,
            20,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(
            x,
            DEADLEG_POS.1 + DEADLEG_WINDOW_Y / 2.0 + length / 2.0,
            BASE_Z / 2.0 + DEADLEG_PANEL_Z + 12.0,
        );
        let gauge_tick = centered_cube(
            format!("closed_sampling_valve_deadleg_station_deadleg_length_tick_{lane}"),
            4.0,
            length.max(10.0),
            5.0,
        )
        .translate(
            x + DEADLEG_WINDOW_X / 2.0 + 8.0,
            DEADLEG_POS.1 + length / 2.0,
            BASE_Z / 2.0 + DEADLEG_PANEL_Z + 27.0,
        );
        let pocket_lip = centered_cube(
            format!("closed_sampling_valve_deadleg_station_witness_pocket_retainer_lip_{lane}"),
            DEADLEG_WINDOW_X + 18.0,
            5.0,
            8.0,
        )
        .translate(
            x,
            DEADLEG_POS.1 - DEADLEG_WINDOW_Y / 2.0 - 11.0,
            BASE_Z / 2.0 + DEADLEG_PANEL_Z + 4.0,
        );

        pockets =
            pockets + (frame - window_void) + live_path + blind_branch + gauge_tick + pocket_lip;
    }

    rail + pockets
}

fn flush_waste_manifold_placeholder() -> Part {
    let block = centered_cube(
        "closed_sampling_valve_deadleg_station_flush_waste_manifold_placeholder_block",
        MANIFOLD_X,
        MANIFOLD_Y,
        MANIFOLD_Z,
    )
    .translate(MANIFOLD_POS.0, MANIFOLD_POS.1, z_on_base(MANIFOLD_Z));
    let mut cutters = Part::empty("closed_sampling_valve_deadleg_station_manifold_port_cutters");
    let mut features = Part::empty("closed_sampling_valve_deadleg_station_manifold_features");

    for lane in 0..MANIFOLD_FLUSH_PORTS {
        let x = MANIFOLD_POS.0 + centered_index(lane, MANIFOLD_FLUSH_PORTS, MANIFOLD_PORT_PITCH_X);
        let flush_port = centered_cylinder(
            format!("closed_sampling_valve_deadleg_station_flush_port_bore_{lane}"),
            FLUSH_PORT_D / 2.0,
            MANIFOLD_Z + 8.0,
            24,
        )
        .translate(x, MANIFOLD_POS.1 + 33.0, z_on_base(MANIFOLD_Z));
        let waste_port = centered_cylinder(
            format!("closed_sampling_valve_deadleg_station_waste_port_bore_{lane}"),
            WASTE_PORT_D / 2.0,
            MANIFOLD_Z + 8.0,
            28,
        )
        .translate(x, MANIFOLD_POS.1 - 33.0, z_on_base(MANIFOLD_Z));
        let quickconnect_boss = centered_cylinder(
            format!("closed_sampling_valve_deadleg_station_manifold_quickconnect_boss_{lane}"),
            9.0,
            7.0,
            28,
        )
        .translate(x, MANIFOLD_POS.1 + 33.0, BASE_Z / 2.0 + MANIFOLD_Z + 3.5);
        let waste_boss = centered_cylinder(
            format!("closed_sampling_valve_deadleg_station_waste_quickconnect_boss_{lane}"),
            12.0,
            7.0,
            28,
        )
        .translate(x, MANIFOLD_POS.1 - 33.0, BASE_Z / 2.0 + MANIFOLD_Z + 3.5);
        cutters = cutters + flush_port + waste_port;
        features = features + quickconnect_boss + waste_boss;
    }

    let trough = centered_cube(
        "closed_sampling_valve_deadleg_station_waste_collection_trough_placeholder",
        WASTE_TROUGH_X,
        WASTE_TROUGH_Y,
        WASTE_TROUGH_Z,
    )
    .translate(
        MANIFOLD_POS.0,
        MANIFOLD_POS.1 - 63.0,
        BASE_Z / 2.0 + MANIFOLD_Z + WASTE_TROUGH_Z / 2.0,
    );
    let trough_void = centered_cube(
        "closed_sampling_valve_deadleg_station_waste_trough_open_recess",
        WASTE_TROUGH_X - 28.0,
        WASTE_TROUGH_Y - 12.0,
        WASTE_TROUGH_Z,
    )
    .translate(
        MANIFOLD_POS.0,
        MANIFOLD_POS.1 - 63.0,
        BASE_Z / 2.0 + MANIFOLD_Z + WASTE_TROUGH_Z / 2.0 + 3.0,
    );
    let drain_stub = centered_cylinder(
        "closed_sampling_valve_deadleg_station_manifold_waste_drain_stub",
        12.0,
        70.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(
        MANIFOLD_POS.0 + MANIFOLD_X / 2.0 + 34.0,
        MANIFOLD_POS.1 - 63.0,
        BASE_Z / 2.0 + MANIFOLD_Z + 9.0,
    );

    block - cutters + features + (trough - trough_void) + drain_stub
}

fn sample_loop_volume_wells() -> Part {
    let bank = centered_cube(
        "closed_sampling_valve_deadleg_station_sample_loop_volume_well_bank",
        LOOP_WELL_BANK_X,
        LOOP_WELL_BANK_Y,
        LOOP_WELL_BANK_Z,
    )
    .translate(LOOP_POS.0, LOOP_POS.1, z_on_base(LOOP_WELL_BANK_Z));
    let mut cutters = Part::empty("closed_sampling_valve_deadleg_station_loop_volume_well_cutters");
    let mut features =
        Part::empty("closed_sampling_valve_deadleg_station_loop_volume_well_features");

    for row in 0..LOOP_WELL_ROWS {
        for col in 0..LOOP_WELL_COLS {
            let index = row * LOOP_WELL_COLS + col;
            let x = LOOP_POS.0 + centered_index(col, LOOP_WELL_COLS, LOOP_WELL_PITCH_X);
            let y = LOOP_POS.1 + centered_index(row, LOOP_WELL_ROWS, LOOP_WELL_PITCH_Y);
            let well_d = LOOP_WELL_BASE_D + LOOP_WELL_STEP_D * index as f64;
            let well = centered_cylinder(
                format!("closed_sampling_valve_deadleg_station_sample_loop_volume_well_{index}"),
                well_d / 2.0,
                LOOP_WELL_BANK_Z + 8.0,
                32,
            )
            .translate(x, y, z_on_base(LOOP_WELL_BANK_Z));
            let rim = centered_cylinder(
                format!(
                    "closed_sampling_valve_deadleg_station_sample_loop_volume_well_rim_{index}"
                ),
                well_d / 2.0 + 4.0,
                4.0,
                32,
            )
            .translate(x, y, BASE_Z / 2.0 + LOOP_WELL_BANK_Z + 2.0);
            let rim_opening = centered_cylinder(
                format!("closed_sampling_valve_deadleg_station_sample_loop_volume_well_rim_opening_{index}"),
                well_d / 2.0,
                5.0,
                32,
            )
            .translate(x, y, BASE_Z / 2.0 + LOOP_WELL_BANK_Z + 2.0);
            let volume_flag = centered_cube(
                format!("closed_sampling_valve_deadleg_station_sample_loop_volume_flag_{index}"),
                5.0 + index as f64 * 1.8,
                8.0,
                6.0,
            )
            .translate(
                x,
                y + LOOP_WELL_PITCH_Y / 2.0 - 10.0,
                BASE_Z / 2.0 + LOOP_WELL_BANK_Z + 3.0,
            );
            cutters = cutters + well;
            features = features + (rim - rim_opening) + volume_flag;
        }
    }

    bank - cutters + features + loop_well_guard_rails()
}

fn loop_well_guard_rails() -> Part {
    let front = centered_cube(
        "closed_sampling_valve_deadleg_station_loop_well_front_guard_rail",
        LOOP_WELL_BANK_X - 28.0,
        7.0,
        16.0,
    )
    .translate(
        LOOP_POS.0,
        LOOP_POS.1 - LOOP_WELL_BANK_Y / 2.0 + 12.0,
        BASE_Z / 2.0 + LOOP_WELL_BANK_Z + 8.0,
    );
    let rear = centered_cube(
        "closed_sampling_valve_deadleg_station_loop_well_rear_guard_rail",
        LOOP_WELL_BANK_X - 28.0,
        7.0,
        16.0,
    )
    .translate(
        LOOP_POS.0,
        LOOP_POS.1 + LOOP_WELL_BANK_Y / 2.0 - 12.0,
        BASE_Z / 2.0 + LOOP_WELL_BANK_Z + 8.0,
    );
    front + rear
}

fn pressure_flow_witness_ports() -> Part {
    let bar = centered_cube(
        "closed_sampling_valve_deadleg_station_pressure_flow_witness_bar",
        WITNESS_BAR_X,
        WITNESS_BAR_Y,
        WITNESS_BAR_Z,
    )
    .translate(WITNESS_POS.0, WITNESS_POS.1, z_on_base(WITNESS_BAR_Z));
    let mut cutters = Part::empty("closed_sampling_valve_deadleg_station_witness_port_cutters");
    let mut features = Part::empty("closed_sampling_valve_deadleg_station_witness_port_features");

    for pair in 0..WITNESS_PORT_PAIRS {
        let x = WITNESS_POS.0 + centered_index(pair, WITNESS_PORT_PAIRS, WITNESS_PORT_PITCH_X);
        let pressure = centered_cylinder(
            format!("closed_sampling_valve_deadleg_station_pressure_witness_tap_{pair}"),
            PRESSURE_TAP_D / 2.0,
            WITNESS_BAR_Z + 6.0,
            24,
        )
        .translate(x, WITNESS_POS.1 + 27.0, z_on_base(WITNESS_BAR_Z));
        let flow_window_cut = centered_cube(
            format!("closed_sampling_valve_deadleg_station_flow_witness_window_{pair}"),
            FLOW_WINDOW_X,
            FLOW_WINDOW_Y,
            FLOW_WINDOW_Z,
        )
        .translate(x, WITNESS_POS.1 - 24.0, BASE_Z / 2.0 + WITNESS_BAR_Z - 8.0);
        let flow_frame = centered_cube(
            format!("closed_sampling_valve_deadleg_station_flow_witness_window_frame_{pair}"),
            FLOW_WINDOW_X + 10.0,
            FLOW_WINDOW_Y + 10.0,
            FLOW_WINDOW_Z + 6.0,
        )
        .translate(x, WITNESS_POS.1 - 24.0, BASE_Z / 2.0 + WITNESS_BAR_Z - 8.0);
        let flow_frame_opening = centered_cube(
            format!("closed_sampling_valve_deadleg_station_flow_witness_frame_opening_{pair}"),
            FLOW_WINDOW_X,
            FLOW_WINDOW_Y,
            FLOW_WINDOW_Z,
        )
        .translate(x, WITNESS_POS.1 - 24.0, BASE_Z / 2.0 + WITNESS_BAR_Z - 8.0);
        let pressure_boss = centered_cylinder(
            format!("closed_sampling_valve_deadleg_station_pressure_witness_boss_{pair}"),
            PRESSURE_TAP_D / 2.0 + 4.0,
            5.0,
            24,
        )
        .translate(x, WITNESS_POS.1 + 27.0, BASE_Z / 2.0 + WITNESS_BAR_Z + 2.5);
        let flow_arrow = centered_cube(
            format!("closed_sampling_valve_deadleg_station_flow_direction_land_{pair}"),
            28.0,
            5.0,
            5.0,
        )
        .translate(x, WITNESS_POS.1, BASE_Z / 2.0 + WITNESS_BAR_Z + 2.5);
        cutters = cutters + pressure + flow_window_cut;
        features = features + (flow_frame - flow_frame_opening) + pressure_boss + flow_arrow;
    }

    bar - cutters + features
}

fn barcode_certificate_lands() -> Part {
    let panel = centered_cube(
        "closed_sampling_valve_deadleg_station_barcode_certificate_panel",
        TRACE_PANEL_X,
        TRACE_PANEL_Y,
        TRACE_PANEL_Z,
    )
    .translate(TRACE_POS.0, TRACE_POS.1, z_on_base(TRACE_PANEL_Z));
    let mut lands = Part::empty("closed_sampling_valve_deadleg_station_barcode_certificate_lands");

    for i in 0..BARCODE_LANDS {
        let row = i / 4;
        let col = i % 4;
        let x = TRACE_POS.0 + centered_index(col, 4, 58.0);
        let y = TRACE_POS.1 + 18.0 - row as f64 * 28.0;
        lands = lands
            + centered_cube(
                format!("closed_sampling_valve_deadleg_station_barcode_land_{i}"),
                BARCODE_LAND_X,
                BARCODE_LAND_Y,
                4.0,
            )
            .translate(x, y, BASE_Z / 2.0 + TRACE_PANEL_Z + 2.0);
    }

    for i in 0..CERTIFICATE_LANDS {
        let x = TRACE_POS.0 + centered_index(i, CERTIFICATE_LANDS, 54.0);
        lands = lands
            + centered_cube(
                format!("closed_sampling_valve_deadleg_station_certificate_land_{i}"),
                CERTIFICATE_LAND_X.min(46.0),
                CERTIFICATE_LAND_Y,
                5.0,
            )
            .translate(x, TRACE_POS.1 - 34.0, BASE_Z / 2.0 + TRACE_PANEL_Z + 2.5);
    }

    for (i, (dx, dy)) in [
        (-104.0, 38.0),
        (104.0, 38.0),
        (-104.0, -38.0),
        (104.0, -38.0),
    ]
    .iter()
    .enumerate()
    {
        let disc = centered_cylinder(
            format!("closed_sampling_valve_deadleg_station_trace_panel_fiducial_{i}"),
            8.0,
            3.5,
            28,
        )
        .translate(
            TRACE_POS.0 + dx,
            TRACE_POS.1 + dy,
            BASE_Z / 2.0 + TRACE_PANEL_Z + 1.75,
        );
        let pin = centered_cylinder(
            format!("closed_sampling_valve_deadleg_station_trace_panel_fiducial_pin_{i}"),
            2.8,
            4.5,
            20,
        )
        .translate(
            TRACE_POS.0 + dx,
            TRACE_POS.1 + dy,
            BASE_Z / 2.0 + TRACE_PANEL_Z + 1.75,
        );
        lands = lands + (disc - pin);
    }

    panel + lands
}

fn release_hold_reject_lanes() -> Part {
    let bank = centered_cube(
        "closed_sampling_valve_deadleg_station_release_hold_reject_lane_bank",
        DISPOSITION_BANK_X,
        DISPOSITION_BANK_Y,
        DISPOSITION_BANK_Z,
    )
    .translate(
        DISPOSITION_POS.0,
        DISPOSITION_POS.1,
        z_on_base(DISPOSITION_BANK_Z),
    );
    let mut lanes = Part::empty("closed_sampling_valve_deadleg_station_disposition_lanes");

    for lane in 0..DISPOSITION_LANES {
        let x =
            DISPOSITION_POS.0 + centered_index(lane, DISPOSITION_LANES, DISPOSITION_LANE_PITCH_X);
        let lane_tray = centered_cube(
            format!(
                "closed_sampling_valve_deadleg_station_{}_lane_tray",
                disposition_name(lane)
            ),
            DISPOSITION_LANE_X,
            DISPOSITION_LANE_Y,
            18.0,
        )
        .translate(
            x,
            DISPOSITION_POS.1,
            BASE_Z / 2.0 + DISPOSITION_BANK_Z + 9.0,
        );
        let lane_pocket = centered_cube(
            format!(
                "closed_sampling_valve_deadleg_station_{}_lane_open_pocket",
                disposition_name(lane)
            ),
            DISPOSITION_LANE_X - 28.0,
            DISPOSITION_LANE_Y - 26.0,
            20.0,
        )
        .translate(
            x,
            DISPOSITION_POS.1,
            BASE_Z / 2.0 + DISPOSITION_BANK_Z + 11.0,
        );
        let gate = centered_cube(
            format!(
                "closed_sampling_valve_deadleg_station_{}_lane_custody_gate",
                disposition_name(lane)
            ),
            DISPOSITION_LANE_X - 18.0,
            8.0,
            DISPOSITION_GATE_Z,
        )
        .translate(
            x,
            DISPOSITION_POS.1 + DISPOSITION_LANE_Y / 2.0 + 5.0,
            BASE_Z / 2.0 + DISPOSITION_BANK_Z + DISPOSITION_GATE_Z / 2.0,
        );
        let status_land = centered_cube(
            format!(
                "closed_sampling_valve_deadleg_station_{}_lane_status_land",
                disposition_name(lane)
            ),
            78.0,
            18.0,
            5.0,
        )
        .translate(
            x,
            DISPOSITION_POS.1 - DISPOSITION_LANE_Y / 2.0 + 14.0,
            BASE_Z / 2.0 + DISPOSITION_BANK_Z + 20.5,
        );
        lanes = lanes + (lane_tray - lane_pocket) + gate + status_land;
    }

    bank + lanes + disposition_lane_separators()
}

fn disposition_lane_separators() -> Part {
    let left = centered_cube(
        "closed_sampling_valve_deadleg_station_release_hold_lane_separator",
        8.0,
        DISPOSITION_BANK_Y,
        38.0,
    )
    .translate(
        DISPOSITION_POS.0 - DISPOSITION_LANE_PITCH_X / 2.0,
        DISPOSITION_POS.1,
        BASE_Z / 2.0 + DISPOSITION_BANK_Z + 19.0,
    );
    let right = centered_cube(
        "closed_sampling_valve_deadleg_station_hold_reject_lane_separator",
        8.0,
        DISPOSITION_BANK_Y,
        38.0,
    )
    .translate(
        DISPOSITION_POS.0 + DISPOSITION_LANE_PITCH_X / 2.0,
        DISPOSITION_POS.1,
        BASE_Z / 2.0 + DISPOSITION_BANK_Z + 19.0,
    );
    left + right
}

fn evidence_bridge() -> Part {
    let left_post = centered_cube(
        "closed_sampling_valve_deadleg_station_evidence_bridge_left_post",
        EVIDENCE_POST_X,
        EVIDENCE_POST_Y,
        EVIDENCE_POST_Z,
    )
    .translate(
        EVIDENCE_POS.0 - EVIDENCE_BRIDGE_X / 2.0 + 52.0,
        EVIDENCE_POS.1,
        BASE_Z / 2.0 + EVIDENCE_POST_Z / 2.0,
    );
    let right_post = centered_cube(
        "closed_sampling_valve_deadleg_station_evidence_bridge_right_post",
        EVIDENCE_POST_X,
        EVIDENCE_POST_Y,
        EVIDENCE_POST_Z,
    )
    .translate(
        EVIDENCE_POS.0 + EVIDENCE_BRIDGE_X / 2.0 - 52.0,
        EVIDENCE_POS.1,
        BASE_Z / 2.0 + EVIDENCE_POST_Z / 2.0,
    );
    let beam = centered_cube(
        "closed_sampling_valve_deadleg_station_evidence_bridge_crossbeam",
        EVIDENCE_BRIDGE_X,
        EVIDENCE_BRIDGE_Y,
        EVIDENCE_BEAM_Z,
    )
    .translate(
        EVIDENCE_POS.0,
        EVIDENCE_POS.1,
        BASE_Z / 2.0 + EVIDENCE_POST_Z + EVIDENCE_BEAM_Z / 2.0,
    );
    let camera_bar = evidence_cameras();
    let lights = evidence_light_bars();
    let scale = centered_cube(
        "closed_sampling_valve_deadleg_station_evidence_bridge_reference_scale",
        EVIDENCE_BRIDGE_X - 160.0,
        6.0,
        6.0,
    )
    .translate(
        EVIDENCE_POS.0,
        EVIDENCE_POS.1 - EVIDENCE_BRIDGE_Y / 2.0 - 8.0,
        BASE_Z / 2.0 + EVIDENCE_POST_Z - 8.0,
    );

    left_post + right_post + beam + camera_bar + lights + scale + evidence_scale_ticks()
}

fn evidence_cameras() -> Part {
    let mut cameras = Part::empty("closed_sampling_valve_deadleg_station_evidence_cameras");
    for i in 0..EVIDENCE_CAMERAS {
        let x = EVIDENCE_POS.0 + centered_index(i, EVIDENCE_CAMERAS, 310.0);
        let camera_body = centered_cube(
            format!("closed_sampling_valve_deadleg_station_evidence_camera_body_{i}"),
            44.0,
            34.0,
            28.0,
        )
        .translate(x, EVIDENCE_POS.1, BASE_Z / 2.0 + EVIDENCE_POST_Z - 26.0);
        let lens = centered_cylinder(
            format!("closed_sampling_valve_deadleg_station_evidence_camera_lens_{i}"),
            9.0,
            18.0,
            32,
        )
        .translate(
            x,
            EVIDENCE_POS.1 - 8.0,
            BASE_Z / 2.0 + EVIDENCE_POST_Z - 44.0,
        );
        cameras = cameras + camera_body + lens;
    }
    cameras
}

fn evidence_light_bars() -> Part {
    let mut lights = Part::empty("closed_sampling_valve_deadleg_station_evidence_light_bars");
    for i in 0..EVIDENCE_LIGHT_BARS {
        let x = EVIDENCE_POS.0 + centered_index(i, EVIDENCE_LIGHT_BARS, 250.0);
        lights = lights
            + centered_cube(
                format!("closed_sampling_valve_deadleg_station_evidence_light_bar_{i}"),
                120.0,
                8.0,
                8.0,
            )
            .translate(
                x,
                EVIDENCE_POS.1 + EVIDENCE_BRIDGE_Y / 2.0 + 4.0,
                BASE_Z / 2.0 + EVIDENCE_POST_Z - 20.0,
            );
    }
    lights
}

fn evidence_scale_ticks() -> Part {
    let mut ticks = Part::empty("closed_sampling_valve_deadleg_station_evidence_scale_ticks");
    for i in 0..=20 {
        let x = EVIDENCE_POS.0 - 450.0 + i as f64 * 45.0;
        let tick_z = if i % 5 == 0 { 18.0 } else { 10.0 };
        ticks = ticks
            + centered_cube(
                format!("closed_sampling_valve_deadleg_station_evidence_scale_tick_{i}"),
                3.0,
                5.0,
                tick_z,
            )
            .translate(
                x,
                EVIDENCE_POS.1 - EVIDENCE_BRIDGE_Y / 2.0 - 12.0,
                BASE_Z / 2.0 + EVIDENCE_POST_Z - 8.0,
            );
    }
    ticks
}

fn robot_service_keepout_gauges() -> Part {
    robot_sweep_gauge()
        + front_service_gauge()
        + rear_service_gauge()
        + right_manifold_service_gauge()
        + left_trace_service_gauge()
}

fn robot_sweep_gauge() -> Part {
    let front = centered_cube(
        "closed_sampling_valve_deadleg_station_robot_sweep_keepout_front_gauge",
        STATION_X - 160.0,
        6.0,
        10.0,
    )
    .translate(0.0, -250.0, BASE_Z / 2.0 + ROBOT_KEEP_OUT_Z);
    let rear = centered_cube(
        "closed_sampling_valve_deadleg_station_robot_sweep_keepout_rear_gauge",
        STATION_X - 160.0,
        6.0,
        10.0,
    )
    .translate(0.0, 250.0, BASE_Z / 2.0 + ROBOT_KEEP_OUT_Z);
    let left = centered_cube(
        "closed_sampling_valve_deadleg_station_robot_sweep_keepout_left_gauge",
        6.0,
        500.0,
        10.0,
    )
    .translate(-520.0, 0.0, BASE_Z / 2.0 + ROBOT_KEEP_OUT_Z);
    let right = centered_cube(
        "closed_sampling_valve_deadleg_station_robot_sweep_keepout_right_gauge",
        6.0,
        500.0,
        10.0,
    )
    .translate(520.0, 0.0, BASE_Z / 2.0 + ROBOT_KEEP_OUT_Z);
    front + rear + left + right
}

fn front_service_gauge() -> Part {
    centered_cube(
        "closed_sampling_valve_deadleg_station_front_service_clearance_gauge",
        STATION_X - 180.0,
        10.0,
        34.0,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 - FRONT_SERVICE_CLEARANCE / 2.0,
        BASE_Z / 2.0 + 17.0,
    )
}

fn rear_service_gauge() -> Part {
    centered_cube(
        "closed_sampling_valve_deadleg_station_rear_service_clearance_gauge",
        STATION_X - 220.0,
        10.0,
        30.0,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 + REAR_SERVICE_CLEARANCE / 2.0,
        BASE_Z / 2.0 + 15.0,
    )
}

fn right_manifold_service_gauge() -> Part {
    centered_cube(
        "closed_sampling_valve_deadleg_station_right_manifold_service_clearance_gauge",
        10.0,
        360.0,
        34.0,
    )
    .translate(
        STATION_X / 2.0 + RIGHT_MANIFOLD_SERVICE_CLEARANCE / 2.0,
        -80.0,
        BASE_Z / 2.0 + 17.0,
    )
}

fn left_trace_service_gauge() -> Part {
    centered_cube(
        "closed_sampling_valve_deadleg_station_left_trace_service_clearance_gauge",
        10.0,
        240.0,
        28.0,
    )
    .translate(
        -STATION_X / 2.0 - LEFT_TRACE_SERVICE_CLEARANCE / 2.0,
        265.0,
        BASE_Z / 2.0 + 14.0,
    )
}

fn fluid_route_placeholders() -> Part {
    let valve_to_deadleg = centered_cube(
        "closed_sampling_valve_deadleg_station_valve_to_deadleg_closed_route_placeholder",
        7.0,
        92.0,
        5.0,
    )
    .translate(-340.0, 40.0, BASE_Z / 2.0 + 6.0);
    let deadleg_to_loop = centered_cube(
        "closed_sampling_valve_deadleg_station_deadleg_to_sample_loop_route_placeholder",
        7.0,
        114.0,
        5.0,
    )
    .translate(-340.0, -175.0, BASE_Z / 2.0 + 6.0);
    let standard_to_manifold = centered_cube(
        "closed_sampling_valve_deadleg_station_standard_to_manifold_route_placeholder",
        7.0,
        92.0,
        5.0,
    )
    .translate(335.0, 55.0, BASE_Z / 2.0 + 6.0);
    let manifold_to_witness = centered_cube(
        "closed_sampling_valve_deadleg_station_manifold_to_pressure_flow_witness_route_placeholder",
        7.0,
        100.0,
        5.0,
    )
    .translate(335.0, -150.0, BASE_Z / 2.0 + 6.0);
    let loop_to_witness = centered_cube(
        "closed_sampling_valve_deadleg_station_sample_loop_to_witness_cross_route_placeholder",
        270.0,
        6.0,
        5.0,
    )
    .translate(-2.0, -255.0, BASE_Z / 2.0 + 6.0);

    valve_to_deadleg
        + deadleg_to_loop
        + standard_to_manifold
        + manifold_to_witness
        + loop_to_witness
}

fn disposition_name(lane: usize) -> &'static str {
    match lane {
        0 => "release",
        1 => "hold",
        2 => "reject",
        _ => "unknown",
    }
}

fn z_on_base(height: f64) -> f64 {
    BASE_Z / 2.0 + height / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn fits_on_station(center: (f64, f64), width: f64, depth: f64, margin: f64) -> bool {
    center.0 - width / 2.0 >= -STATION_X / 2.0 + margin
        && center.0 + width / 2.0 <= STATION_X / 2.0 - margin
        && center.1 - depth / 2.0 >= -STATION_Y / 2.0 + margin
        && center.1 + depth / 2.0 <= STATION_Y / 2.0 - margin
}

fn rect(center: (f64, f64), width: f64, depth: f64) -> (f64, f64, f64, f64) {
    (
        center.0 - width / 2.0,
        center.0 + width / 2.0,
        center.1 - depth / 2.0,
        center.1 + depth / 2.0,
    )
}

fn rects_overlap(left: (f64, f64, f64, f64), right: (f64, f64, f64, f64), margin: f64) -> bool {
    left.0 < right.1 + margin
        && left.1 + margin > right.0
        && left.2 < right.3 + margin
        && left.3 + margin > right.2
}
