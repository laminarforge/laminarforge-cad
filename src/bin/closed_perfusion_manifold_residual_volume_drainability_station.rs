use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed perfusion manifold residual-volume and drainability validation station.
//
// Intent:
// - Hold a closed perfusion manifold in a repeatable nest while measuring
//   residual hold-up after commanded drain and purge sequences.
// - Provide low-point drain witness lanes, calibrated volume capture wells,
//   tilt references, dye recovery pockets, bubble/wetness inspection windows,
//   traceability lands, release/hold/reject segregation, evidence capture, and
//   robot/service keepouts without opening the fluid path.
//
// This is validation-station packaging CAD. It does not specify wetted-path
// materials, analytical acceptance limits, cleaning claims, controller logic,
// biological use, or a release protocol.

const PREFIX: &str = "closed_perfusion_manifold_residual_volume_drainability_station";

const OUTPUTS: [&str; 12] = [
    "output/closed_perfusion_manifold_residual_volume_drainability_station_base_leak_tray.stl",
    "output/closed_perfusion_manifold_residual_volume_drainability_station_manifold_nest.stl",
    "output/closed_perfusion_manifold_residual_volume_drainability_station_low_point_drain_witness_lanes.stl",
    "output/closed_perfusion_manifold_residual_volume_drainability_station_calibrated_volume_capture_wells.stl",
    "output/closed_perfusion_manifold_residual_volume_drainability_station_tilt_reference_feet.stl",
    "output/closed_perfusion_manifold_residual_volume_drainability_station_dye_recovery_pockets.stl",
    "output/closed_perfusion_manifold_residual_volume_drainability_station_bubble_wetness_windows.stl",
    "output/closed_perfusion_manifold_residual_volume_drainability_station_barcode_certificate_lands.stl",
    "output/closed_perfusion_manifold_residual_volume_drainability_station_release_hold_reject_lanes.stl",
    "output/closed_perfusion_manifold_residual_volume_drainability_station_evidence_bridge.stl",
    "output/closed_perfusion_manifold_residual_volume_drainability_station_robot_service_keepouts.stl",
    "output/closed_perfusion_manifold_residual_volume_drainability_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 12] = [
    "manifold_nest",
    "low_point_drain_witness_lanes",
    "calibrated_volume_capture_wells",
    "tilt_reference_feet",
    "dye_recovery_pockets",
    "bubble_wetness_windows",
    "barcode_certificate_lands",
    "release_hold_reject_lanes",
    "evidence_bridge",
    "robot_service_keepouts",
    "leak_containment",
    "assembly_export",
];

const STATION_X: f64 = 1180.0;
const STATION_Y: f64 = 760.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const LEAK_BASIN_X: f64 = STATION_X - 108.0;
const LEAK_BASIN_Y: f64 = STATION_Y - 98.0;
const LEAK_BASIN_DEPTH: f64 = 8.0;
const LEAK_SENSOR_WELLS: usize = 6;
const DRAIN_PORT_D: f64 = 18.0;

const MANIFOLD_CENTER: (f64, f64) = (-285.0, 118.0);
const MANIFOLD_NEST_X: f64 = 430.0;
const MANIFOLD_NEST_Y: f64 = 236.0;
const MANIFOLD_NEST_Z: f64 = 38.0;
const MANIFOLD_CHANNELS: usize = 8;
const MANIFOLD_PORTS: usize = MANIFOLD_CHANNELS * 2;
const CHANNEL_PITCH_Y: f64 = 22.0;
const MANIFOLD_BODY_X: f64 = 318.0;
const MANIFOLD_BODY_Y: f64 = 150.0;
const MANIFOLD_BODY_Z: f64 = 18.0;
const MANIFOLD_SOCKET_DEPTH: f64 = 10.0;
const MANIFOLD_TUBE_OD: f64 = 5.0;
const MANIFOLD_TUBE_CLEARANCE: f64 = 1.0;
const MANIFOLD_PORT_D: f64 = MANIFOLD_TUBE_OD + MANIFOLD_TUBE_CLEARANCE;
const MANIFOLD_CLAMPS: usize = 6;
const MANIFOLD_MAX_HOLDUP_ML: f64 = 42.0;

const DRAIN_LANE_CENTER: (f64, f64) = (-285.0, -104.0);
const DRAIN_LANE_BLOCK_X: f64 = 470.0;
const DRAIN_LANE_BLOCK_Y: f64 = 164.0;
const DRAIN_LANE_BLOCK_Z: f64 = 24.0;
const DRAIN_WITNESS_LANES: usize = MANIFOLD_CHANNELS;
const DRAIN_LANE_PITCH_Y: f64 = 18.0;
const DRAIN_LANE_X: f64 = 394.0;
const DRAIN_LANE_W: f64 = 6.5;
const DRAIN_LOW_POINT_CUPS: usize = DRAIN_WITNESS_LANES;
const DRAIN_TICK_COUNT_PER_LANE: usize = 5;
const DRAIN_SLOPE_REFERENCE_MM: f64 = 3.0;

const CAPTURE_CENTER: (f64, f64) = (235.0, 96.0);
const CAPTURE_RACK_X: f64 = 370.0;
const CAPTURE_RACK_Y: f64 = 286.0;
const CAPTURE_RACK_Z: f64 = 42.0;
const CAPTURE_ROWS: usize = 3;
const CAPTURE_COLS: usize = 4;
const CAPTURE_WELLS: usize = CAPTURE_ROWS * CAPTURE_COLS;
const CAPTURE_WELL_D: f64 = 28.0;
const CAPTURE_WELL_CLEARANCE_D: f64 = 31.0;
const CAPTURE_PITCH_X: f64 = 76.0;
const CAPTURE_PITCH_Y: f64 = 78.0;
const CAPTURE_WELL_NOMINAL_ML: f64 = 10.0;
const CAPTURE_GRADUATION_TICKS: usize = 6;

const TILT_FOOT_COUNT: usize = 4;
const TILT_FOOT_D: f64 = 36.0;
const TILT_FOOT_BASE_Z: f64 = 16.0;
const TILT_REFERENCE_SHIMS: usize = 3;
const TILT_REFERENCE_CENTER: (f64, f64) = (468.0, 300.0);
const TILT_RAIL_X: f64 = 216.0;
const TILT_RAIL_Y: f64 = 64.0;
const TILT_RAIL_Z: f64 = 16.0;
const TILT_REFERENCE_MAX_MM: f64 = 6.0;

const DYE_CENTER: (f64, f64) = (-282.0, -292.0);
const DYE_RACK_X: f64 = 446.0;
const DYE_RACK_Y: f64 = 120.0;
const DYE_RACK_Z: f64 = 32.0;
const DYE_RECOVERY_POCKETS: usize = MANIFOLD_CHANNELS;
const DYE_POCKET_D: f64 = 22.0;
const DYE_POCKET_PITCH_X: f64 = 49.0;
const DYE_CARD_LANDS: usize = 4;
const DYE_BLOTTER_LANDS: usize = 8;

const WINDOW_CENTER: (f64, f64) = (240.0, -124.0);
const WINDOW_PANEL_X: f64 = 392.0;
const WINDOW_PANEL_Y: f64 = 154.0;
const WINDOW_PANEL_Z: f64 = 18.0;
const BUBBLE_WETNESS_WINDOWS: usize = MANIFOLD_CHANNELS;
const WINDOW_PITCH_X: f64 = 44.0;
const BUBBLE_WINDOW_X: f64 = 28.0;
const BUBBLE_WINDOW_Y: f64 = 48.0;
const WETNESS_PAD_X: f64 = 26.0;
const WETNESS_PAD_Y: f64 = 20.0;
const WINDOW_FRAME_Z: f64 = 8.0;

const TRACE_CENTER: (f64, f64) = (-430.0, 308.0);
const TRACE_PANEL_X: f64 = 292.0;
const TRACE_PANEL_Y: f64 = 104.0;
const TRACE_PANEL_Z: f64 = 8.0;
const BARCODE_LANDS: usize = 8;
const CERTIFICATE_LANDS: usize = 3;
const TRACE_FIDUCIALS: usize = 4;
const BARCODE_LAND_X: f64 = 72.0;
const BARCODE_LAND_Y: f64 = 18.0;
const CERTIFICATE_LAND_X: f64 = 100.0;
const CERTIFICATE_LAND_Y: f64 = 32.0;

const DISPOSITION_CENTER: (f64, f64) = (120.0, -302.0);
const DISPOSITION_LANES: usize = 3;
const LANE_X: f64 = 156.0;
const LANE_Y: f64 = 96.0;
const LANE_Z: f64 = 28.0;
const LANE_PITCH_X: f64 = 176.0;
const LANE_RAIL_Z: f64 = 54.0;
const LANE_GATE_Z: f64 = 68.0;

const EVIDENCE_CENTER: (f64, f64) = (0.0, 10.0);
const EVIDENCE_BRIDGE_X: f64 = 1030.0;
const EVIDENCE_BRIDGE_Y: f64 = 44.0;
const EVIDENCE_COLUMN_X: f64 = 26.0;
const EVIDENCE_COLUMN_Y: f64 = 54.0;
const EVIDENCE_CLEAR_Z: f64 = 156.0;
const EVIDENCE_CROSSBAR_Z: f64 = 24.0;
const EVIDENCE_CAMERA_WINDOWS: usize = 5;
const EVIDENCE_WINDOW_X: f64 = 104.0;
const EVIDENCE_WINDOW_Y: f64 = 22.0;

const ROBOT_KEEP_OUT_WINDOWS: usize = 5;
const ROBOT_APPROACH_X: f64 = 1010.0;
const ROBOT_APPROACH_Y: f64 = 612.0;
const ROBOT_KEEP_OUT_Z: f64 = 144.0;
const FRONT_SERVICE_CLEARANCE: f64 = 390.0;
const RIGHT_CAPTURE_SERVICE_CLEARANCE: f64 = 230.0;
const REAR_EVIDENCE_SERVICE_CLEARANCE: f64 = 210.0;
const LEFT_TUBING_SERVICE_CLEARANCE: f64 = 180.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = base_leak_tray();
    export(&base, OUTPUTS[0]);

    let nest = manifold_nest();
    export(&nest, OUTPUTS[1]);

    let drains = low_point_drain_witness_lanes();
    export(&drains, OUTPUTS[2]);

    let capture = calibrated_volume_capture_wells();
    export(&capture, OUTPUTS[3]);

    let tilt = tilt_reference_feet();
    export(&tilt, OUTPUTS[4]);

    let dye = dye_recovery_pockets();
    export(&dye, OUTPUTS[5]);

    let windows = bubble_wetness_windows();
    export(&windows, OUTPUTS[6]);

    let trace = barcode_certificate_lands();
    export(&trace, OUTPUTS[7]);

    let lanes = release_hold_reject_lanes();
    export(&lanes, OUTPUTS[8]);

    let evidence = evidence_bridge();
    export(&evidence, OUTPUTS[9]);

    let keepouts = robot_service_keepouts();
    export(&keepouts, OUTPUTS[10]);

    let assembly =
        base + nest + drains + capture + tilt + dye + windows + trace + lanes + evidence + keepouts;
    export(&assembly, OUTPUTS[11]);

    println!();
    println!("Closed perfusion manifold residual-volume and drainability validation station:");
    println!(
        "  Station envelope:           {STATION_X:.0}mm x {STATION_Y:.0}mm leak-tray deck, {LEAK_SENSOR_WELLS} leak sensor wells, {DRAIN_PORT_D:.0}mm low-point drain port"
    );
    println!(
        "  Manifold handling:          {MANIFOLD_CHANNELS} channel nest, {MANIFOLD_PORTS} closed tube ports, {MANIFOLD_CLAMPS} clamp lands, target hold-up <= {MANIFOLD_MAX_HOLDUP_ML:.0}mL"
    );
    println!(
        "  Drainability evidence:      {DRAIN_WITNESS_LANES} low-point drain witness lanes, {DRAIN_LOW_POINT_CUPS} witness cups, {DRAIN_TICK_COUNT_PER_LANE} ticks per lane, {DRAIN_SLOPE_REFERENCE_MM:.1}mm slope reference"
    );
    println!(
        "  Recovery and inspection:    {CAPTURE_WELLS} calibrated capture wells at {CAPTURE_WELL_NOMINAL_ML:.0}mL nominal each, {DYE_RECOVERY_POCKETS} dye recovery pockets, {BUBBLE_WETNESS_WINDOWS} bubble/wetness windows"
    );
    println!(
        "  Traceability/disposition:   {BARCODE_LANDS} barcode lands, {CERTIFICATE_LANDS} certificate lands, {DISPOSITION_LANES} release/hold/reject lanes, {EVIDENCE_CAMERA_WINDOWS} evidence bridge windows"
    );
    println!(
        "  Service envelopes:          {ROBOT_KEEP_OUT_WINDOWS} robot/service keepout windows, {FRONT_SERVICE_CLEARANCE:.0}mm front service, {RIGHT_CAPTURE_SERVICE_CLEARANCE:.0}mm capture-well service, {REAR_EVIDENCE_SERVICE_CLEARANCE:.0}mm rear evidence service, {LEFT_TUBING_SERVICE_CLEARANCE:.0}mm left tubing service, and {} required feature groups.",
        REQUIRED_FEATURES.len()
    );
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn assert_layout() {
    assert_eq!(
        MANIFOLD_PORTS,
        MANIFOLD_CHANNELS * 2,
        "each manifold lane needs inlet and outlet ports"
    );
    assert_eq!(
        DRAIN_WITNESS_LANES, MANIFOLD_CHANNELS,
        "drain witness lanes must map one-to-one to manifold channels"
    );
    assert_eq!(
        DRAIN_LOW_POINT_CUPS, DRAIN_WITNESS_LANES,
        "each drain lane needs a low-point cup"
    );
    assert_eq!(
        CAPTURE_WELLS,
        CAPTURE_ROWS * CAPTURE_COLS,
        "capture well grid count mismatch"
    );
    assert_eq!(
        DYE_RECOVERY_POCKETS, MANIFOLD_CHANNELS,
        "dye recovery pockets must map one-to-one to manifold channels"
    );
    assert_eq!(
        BUBBLE_WETNESS_WINDOWS, MANIFOLD_CHANNELS,
        "bubble/wetness windows must map one-to-one to manifold channels"
    );
    assert_eq!(
        DYE_BLOTTER_LANDS, DYE_RECOVERY_POCKETS,
        "each dye recovery pocket needs a paired blotter land"
    );
    assert_eq!(
        TRACE_FIDUCIALS, 4,
        "traceability panel needs four optical fiducials"
    );
    assert_eq!(TILT_FOOT_COUNT, 4, "tilt reference needs four station feet");
    assert_eq!(
        DISPOSITION_LANES, 3,
        "release/hold/reject lanes must remain physically distinct"
    );
    assert!(
        MANIFOLD_BODY_Z < MANIFOLD_NEST_Z,
        "manifold nest must be deeper than the manifold body reference envelope"
    );
    assert!(
        CAPTURE_WELL_CLEARANCE_D > CAPTURE_WELL_D,
        "capture wells need vial insertion clearance"
    );
    assert!(
        CAPTURE_WELLS as f64 * CAPTURE_WELL_NOMINAL_ML >= MANIFOLD_MAX_HOLDUP_ML * 2.0,
        "capture volume should hold at least two full manifold hold-up recoveries"
    );
    assert!(
        MANIFOLD_NEST_X + CAPTURE_RACK_X + 140.0 < STATION_X,
        "manifold nest and capture rack exceed station envelope"
    );
    assert!(
        EVIDENCE_CLEAR_Z > ROBOT_KEEP_OUT_Z,
        "evidence bridge must clear the nominal robot keepout height"
    );
}

fn base_leak_tray() -> Part {
    let deck = centered_cube(name("base_deck"), STATION_X, STATION_Y, BASE_Z).translate(
        0.0,
        0.0,
        BASE_Z / 2.0,
    );
    let basin = centered_cube(
        name("recessed_low_point_leak_basin"),
        LEAK_BASIN_X,
        LEAK_BASIN_Y,
        LEAK_BASIN_DEPTH + 1.0,
    )
    .translate(0.0, 0.0, BASE_Z - LEAK_BASIN_DEPTH / 2.0 + 0.5);
    let drain = centered_cylinder(
        name("front_right_low_point_drain_bore"),
        DRAIN_PORT_D / 2.0,
        BASE_Z + 8.0,
        36,
    )
    .translate(
        STATION_X / 2.0 - 86.0,
        -(STATION_Y / 2.0 - 52.0),
        BASE_Z / 2.0,
    );

    deck - basin - drain
        + perimeter_rim()
        + leak_sensor_wells()
        + deck_mount_and_datum_lands()
        + functional_zone_dividers()
}

fn perimeter_rim() -> Part {
    let front = centered_cube(name("front_spill_rim"), STATION_X, RIM_W, RIM_Z).translate(
        0.0,
        -(STATION_Y / 2.0 - RIM_W / 2.0),
        BASE_Z + RIM_Z / 2.0,
    );
    let rear = centered_cube(name("rear_service_rim"), STATION_X, RIM_W, RIM_Z).translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z + RIM_Z / 2.0,
    );
    let left = centered_cube(name("left_tubing_rim"), RIM_W, STATION_Y, RIM_Z).translate(
        -(STATION_X / 2.0 - RIM_W / 2.0),
        0.0,
        BASE_Z + RIM_Z / 2.0,
    );
    let right = centered_cube(
        name("right_capture_well_service_rim"),
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, BASE_Z + RIM_Z / 2.0);

    front + rear + left + right
}

fn leak_sensor_wells() -> Part {
    let mut wells = Part::empty(name("leak_sensor_wells"));
    for (i, (x, y)) in [
        (-500.0, -318.0),
        (-300.0, -318.0),
        (0.0, -318.0),
        (500.0, -318.0),
        (-500.0, 318.0),
        (500.0, 318.0),
    ]
    .iter()
    .enumerate()
    {
        let boss = centered_cube(name(&format!("leak_sensor_boss_{i}")), 46.0, 32.0, 8.0)
            .translate(*x, *y, BASE_Z + 4.0);
        let cup = centered_cylinder(name(&format!("leak_sensor_cup_cut_{i}")), 9.0, 10.0, 30)
            .translate(*x, *y, BASE_Z + 4.0);
        wells = wells + (boss - cup);
    }
    wells
}

fn deck_mount_and_datum_lands() -> Part {
    let mut lands = Part::empty(name("deck_mount_and_robot_datum_lands"));
    for (i, (x, y)) in [
        (-(STATION_X / 2.0 - 62.0), -(STATION_Y / 2.0 - 58.0)),
        (STATION_X / 2.0 - 62.0, -(STATION_Y / 2.0 - 58.0)),
        (-(STATION_X / 2.0 - 62.0), STATION_Y / 2.0 - 58.0),
        (STATION_X / 2.0 - 62.0, STATION_Y / 2.0 - 58.0),
    ]
    .iter()
    .enumerate()
    {
        let pad = centered_cylinder(name(&format!("deck_mount_datum_pad_{i}")), 18.0, 6.0, 36)
            .translate(*x, *y, BASE_Z + 3.0);
        let bore = centered_cylinder(name(&format!("m6_mount_bore_{i}")), 3.4, BASE_Z + 12.0, 24)
            .translate(*x, *y, BASE_Z / 2.0);
        let crosshair_x = centered_cube(
            name(&format!("robot_datum_crosshair_x_{i}")),
            24.0,
            2.0,
            2.4,
        )
        .translate(*x, *y, BASE_Z + 7.2);
        let crosshair_y = centered_cube(
            name(&format!("robot_datum_crosshair_y_{i}")),
            2.0,
            24.0,
            2.4,
        )
        .translate(*x, *y, BASE_Z + 7.2);
        lands = lands + (pad - bore) + crosshair_x + crosshair_y;
    }
    lands
}

fn functional_zone_dividers() -> Part {
    let nest_to_capture = centered_cube(name("nest_to_capture_splash_divider"), 12.0, 510.0, 46.0)
        .translate(-18.0, 28.0, BASE_Z + 23.0);
    let drains_to_disposition = centered_cube(
        name("drain_witness_to_disposition_divider"),
        820.0,
        10.0,
        34.0,
    )
    .translate(-70.0, -206.0, BASE_Z + 17.0);
    let traceability_curb = centered_cube(name("traceability_certificate_curb"), 336.0, 10.0, 28.0)
        .translate(
            TRACE_CENTER.0,
            TRACE_CENTER.1 - TRACE_PANEL_Y / 2.0 - 16.0,
            BASE_Z + 14.0,
        );

    nest_to_capture + drains_to_disposition + traceability_curb
}

fn manifold_nest() -> Part {
    let body = centered_cube(
        name("closed_manifold_nest_body"),
        MANIFOLD_NEST_X,
        MANIFOLD_NEST_Y,
        MANIFOLD_NEST_Z,
    )
    .translate(
        MANIFOLD_CENTER.0,
        MANIFOLD_CENTER.1,
        stage_z(MANIFOLD_NEST_Z),
    );
    let manifold_outline = centered_cube(
        name("manifold_body_outline_relief"),
        MANIFOLD_BODY_X,
        MANIFOLD_BODY_Y,
        MANIFOLD_SOCKET_DEPTH + 1.0,
    )
    .translate(
        MANIFOLD_CENTER.0,
        MANIFOLD_CENTER.1,
        BASE_Z + MANIFOLD_NEST_Z - MANIFOLD_SOCKET_DEPTH / 2.0 + 0.5,
    );

    body - manifold_outline - manifold_channel_cuts()
        + manifold_port_lands()
        + manifold_clamp_lands()
        + manifold_datum_rails()
}

fn manifold_channel_cuts() -> Part {
    let mut cuts = Part::empty(name("manifold_channel_socket_cuts"));
    for lane in 0..MANIFOLD_CHANNELS {
        let y = MANIFOLD_CENTER.1 + lane_offset(lane, MANIFOLD_CHANNELS, CHANNEL_PITCH_Y);
        cuts =
            cuts + centered_cube(
                name(&format!("manifold_lane_recess_{lane}")),
                MANIFOLD_BODY_X - 36.0,
                MANIFOLD_PORT_D + 3.0,
                MANIFOLD_SOCKET_DEPTH + 2.0,
            )
            .translate(
                MANIFOLD_CENTER.0,
                y,
                BASE_Z + MANIFOLD_NEST_Z - MANIFOLD_SOCKET_DEPTH / 2.0,
            ) + centered_cylinder(
                name(&format!("manifold_inlet_port_relief_{lane}")),
                MANIFOLD_PORT_D / 2.0,
                MANIFOLD_NEST_Z + 6.0,
                24,
            )
            .translate(
                MANIFOLD_CENTER.0 - MANIFOLD_BODY_X / 2.0 + 28.0,
                y,
                stage_z(MANIFOLD_NEST_Z),
            ) + centered_cylinder(
                name(&format!("manifold_outlet_port_relief_{lane}")),
                MANIFOLD_PORT_D / 2.0,
                MANIFOLD_NEST_Z + 6.0,
                24,
            )
            .translate(
                MANIFOLD_CENTER.0 + MANIFOLD_BODY_X / 2.0 - 28.0,
                y,
                stage_z(MANIFOLD_NEST_Z),
            );
    }
    cuts
}

fn manifold_port_lands() -> Part {
    let mut lands = Part::empty(name("manifold_inlet_outlet_port_lands"));
    for lane in 0..MANIFOLD_CHANNELS {
        let y = MANIFOLD_CENTER.1 + lane_offset(lane, MANIFOLD_CHANNELS, CHANNEL_PITCH_Y);
        for (side, x) in [
            ("inlet", MANIFOLD_CENTER.0 - MANIFOLD_BODY_X / 2.0 + 28.0),
            ("outlet", MANIFOLD_CENTER.0 + MANIFOLD_BODY_X / 2.0 - 28.0),
        ] {
            let ring = centered_cylinder(
                name(&format!("manifold_{side}_port_ring_{lane}")),
                MANIFOLD_PORT_D / 2.0 + 5.0,
                5.0,
                30,
            )
            .translate(x, y, BASE_Z + MANIFOLD_NEST_Z + 2.5)
                - centered_cylinder(
                    name(&format!("manifold_{side}_port_opening_{lane}")),
                    MANIFOLD_PORT_D / 2.0,
                    6.0,
                    24,
                )
                .translate(x, y, BASE_Z + MANIFOLD_NEST_Z + 2.5);
            lands = lands + ring;
        }
        lands = lands
            + centered_cube(
                name(&format!("manifold_lane_id_land_{lane}")),
                28.0,
                10.0,
                3.0,
            )
            .translate(MANIFOLD_CENTER.0, y, BASE_Z + MANIFOLD_NEST_Z + 1.5);
    }
    lands
}

fn manifold_clamp_lands() -> Part {
    let mut clamps = Part::empty(name("manifold_clamp_lands"));
    for clamp in 0..MANIFOLD_CLAMPS {
        let x = MANIFOLD_CENTER.0 + lane_offset(clamp, MANIFOLD_CLAMPS, 68.0);
        for y in [
            MANIFOLD_CENTER.1 - MANIFOLD_BODY_Y / 2.0 - 18.0,
            MANIFOLD_CENTER.1 + MANIFOLD_BODY_Y / 2.0 + 18.0,
        ] {
            clamps = clamps
                + centered_cube(
                    name(&format!("manifold_toggle_clamp_land_{clamp}_{y:.0}")),
                    42.0,
                    16.0,
                    14.0,
                )
                .translate(x, y, BASE_Z + MANIFOLD_NEST_Z + 7.0)
                - centered_cylinder(
                    name(&format!(
                        "manifold_toggle_clamp_screw_clearance_{clamp}_{y:.0}"
                    )),
                    3.0,
                    16.0,
                    18,
                )
                .translate(x, y, BASE_Z + MANIFOLD_NEST_Z + 7.0);
        }
    }
    clamps
}

fn manifold_datum_rails() -> Part {
    let left = centered_cube(
        name("manifold_left_hard_datum_rail"),
        12.0,
        MANIFOLD_BODY_Y + 50.0,
        24.0,
    )
    .translate(
        MANIFOLD_CENTER.0 - MANIFOLD_BODY_X / 2.0 - 30.0,
        MANIFOLD_CENTER.1,
        BASE_Z + MANIFOLD_NEST_Z + 12.0,
    );
    let rear = centered_cube(
        name("manifold_rear_low_point_hard_stop"),
        MANIFOLD_BODY_X + 88.0,
        12.0,
        22.0,
    )
    .translate(
        MANIFOLD_CENTER.0,
        MANIFOLD_CENTER.1 + MANIFOLD_BODY_Y / 2.0 + 30.0,
        BASE_Z + MANIFOLD_NEST_Z + 11.0,
    );
    let front = centered_cube(
        name("manifold_front_service_soft_stop"),
        MANIFOLD_BODY_X + 88.0,
        8.0,
        14.0,
    )
    .translate(
        MANIFOLD_CENTER.0,
        MANIFOLD_CENTER.1 - MANIFOLD_BODY_Y / 2.0 - 30.0,
        BASE_Z + MANIFOLD_NEST_Z + 7.0,
    );

    left + rear + front
}

fn low_point_drain_witness_lanes() -> Part {
    let block = centered_cube(
        name("low_point_drain_witness_lane_block"),
        DRAIN_LANE_BLOCK_X,
        DRAIN_LANE_BLOCK_Y,
        DRAIN_LANE_BLOCK_Z,
    )
    .translate(
        DRAIN_LANE_CENTER.0,
        DRAIN_LANE_CENTER.1,
        stage_z(DRAIN_LANE_BLOCK_Z),
    );
    let mut troughs = Part::empty(name("low_point_drain_lane_grooves"));
    let mut details = Part::empty(name("low_point_drain_lane_details"));

    for lane in 0..DRAIN_WITNESS_LANES {
        let y = DRAIN_LANE_CENTER.1 + lane_offset(lane, DRAIN_WITNESS_LANES, DRAIN_LANE_PITCH_Y);
        troughs = troughs
            + centered_cube(
                name(&format!("drain_witness_lane_groove_{lane}")),
                DRAIN_LANE_X,
                DRAIN_LANE_W,
                DRAIN_LANE_BLOCK_Z + 5.0,
            )
            .translate(DRAIN_LANE_CENTER.0 - 12.0, y, stage_z(DRAIN_LANE_BLOCK_Z));
        details = details
            + centered_cylinder(
                name(&format!("low_point_witness_cup_rim_{lane}")),
                11.0,
                5.0,
                30,
            )
            .translate(
                DRAIN_LANE_CENTER.0 + DRAIN_LANE_X / 2.0 - 42.0,
                y,
                BASE_Z + DRAIN_LANE_BLOCK_Z + 2.5,
            )
            - centered_cylinder(
                name(&format!("low_point_witness_cup_opening_{lane}")),
                7.5,
                6.0,
                24,
            )
            .translate(
                DRAIN_LANE_CENTER.0 + DRAIN_LANE_X / 2.0 - 42.0,
                y,
                BASE_Z + DRAIN_LANE_BLOCK_Z + 2.5,
            );
        for tick in 0..DRAIN_TICK_COUNT_PER_LANE {
            let x = DRAIN_LANE_CENTER.0 - DRAIN_LANE_X / 2.0 + 54.0 + tick as f64 * 75.0;
            details = details
                + centered_cube(
                    name(&format!("drain_lane_{lane}_meniscus_tick_{tick}")),
                    3.0,
                    14.0,
                    3.0,
                )
                .translate(x, y, BASE_Z + DRAIN_LANE_BLOCK_Z + 1.5);
        }
    }

    let slope_reference = centered_cube(
        name("three_mm_tilt_slope_reference_ramp_surrogate"),
        DRAIN_LANE_X,
        12.0,
        DRAIN_SLOPE_REFERENCE_MM,
    )
    .translate(
        DRAIN_LANE_CENTER.0 - 12.0,
        DRAIN_LANE_CENTER.1 - DRAIN_LANE_BLOCK_Y / 2.0 + 18.0,
        BASE_Z + DRAIN_LANE_BLOCK_Z + DRAIN_SLOPE_REFERENCE_MM / 2.0,
    );

    block - troughs + details + drain_lane_divider_ribs() + slope_reference
}

fn drain_lane_divider_ribs() -> Part {
    let mut ribs = Part::empty(name("drain_lane_divider_ribs"));
    for rib in 0..=DRAIN_WITNESS_LANES {
        let y = DRAIN_LANE_CENTER.1
            + (rib as f64 - DRAIN_WITNESS_LANES as f64 / 2.0) * DRAIN_LANE_PITCH_Y
            - DRAIN_LANE_PITCH_Y / 2.0;
        ribs = ribs
            + centered_cube(
                name(&format!("drain_witness_lane_divider_rib_{rib}")),
                DRAIN_LANE_X + 18.0,
                2.0,
                5.0,
            )
            .translate(
                DRAIN_LANE_CENTER.0 - 12.0,
                y,
                BASE_Z + DRAIN_LANE_BLOCK_Z + 2.5,
            );
    }
    ribs
}

fn calibrated_volume_capture_wells() -> Part {
    let rack = centered_cube(
        name("calibrated_volume_capture_well_rack"),
        CAPTURE_RACK_X,
        CAPTURE_RACK_Y,
        CAPTURE_RACK_Z,
    )
    .translate(CAPTURE_CENTER.0, CAPTURE_CENTER.1, stage_z(CAPTURE_RACK_Z));
    let mut cuts = Part::empty(name("calibrated_capture_well_cuts"));
    let mut rims = Part::empty(name("calibrated_capture_well_rims_and_ticks"));

    for row in 0..CAPTURE_ROWS {
        for col in 0..CAPTURE_COLS {
            let index = row * CAPTURE_COLS + col;
            let x = CAPTURE_CENTER.0 + lane_offset(col, CAPTURE_COLS, CAPTURE_PITCH_X);
            let y = CAPTURE_CENTER.1 + lane_offset(row, CAPTURE_ROWS, CAPTURE_PITCH_Y);
            cuts = cuts
                + centered_cylinder(
                    name(&format!("calibrated_capture_well_clearance_{index}")),
                    CAPTURE_WELL_CLEARANCE_D / 2.0,
                    CAPTURE_RACK_Z + 8.0,
                    36,
                )
                .translate(x, y, stage_z(CAPTURE_RACK_Z));
            rims = rims
                + centered_cylinder(
                    name(&format!("calibrated_capture_well_rim_{index}")),
                    CAPTURE_WELL_D / 2.0 + 5.0,
                    5.0,
                    36,
                )
                .translate(x, y, BASE_Z + CAPTURE_RACK_Z + 2.5)
                - centered_cylinder(
                    name(&format!("calibrated_capture_well_opening_{index}")),
                    CAPTURE_WELL_D / 2.0,
                    6.0,
                    36,
                )
                .translate(x, y, BASE_Z + CAPTURE_RACK_Z + 2.5);
            for tick in 0..CAPTURE_GRADUATION_TICKS {
                let tick_y = y - CAPTURE_WELL_D / 2.0 - 10.0 + tick as f64 * 4.0;
                rims = rims
                    + centered_cube(
                        name(&format!("capture_well_{index}_graduation_tick_{tick}")),
                        16.0 - tick as f64,
                        1.5,
                        3.0,
                    )
                    .translate(x + 25.0, tick_y, BASE_Z + CAPTURE_RACK_Z + 1.5);
            }
        }
    }

    rack - cuts + rims + capture_well_row_labels()
}

fn capture_well_row_labels() -> Part {
    let mut labels = Part::empty(name("capture_well_row_certificate_labels"));
    for row in 0..CAPTURE_ROWS {
        let y = CAPTURE_CENTER.1 + lane_offset(row, CAPTURE_ROWS, CAPTURE_PITCH_Y);
        labels = labels
            + centered_cube(
                name(&format!("capture_well_row_label_land_{row}")),
                42.0,
                18.0,
                4.0,
            )
            .translate(
                CAPTURE_CENTER.0 - CAPTURE_RACK_X / 2.0 + 30.0,
                y,
                BASE_Z + CAPTURE_RACK_Z + 2.0,
            );
    }
    labels
}

fn tilt_reference_feet() -> Part {
    let mut feet = Part::empty(name("tilt_reference_station_feet"));
    for (i, (x, y, z_extra)) in [
        (-(STATION_X / 2.0 - 94.0), -(STATION_Y / 2.0 - 88.0), 0.0),
        (STATION_X / 2.0 - 94.0, -(STATION_Y / 2.0 - 88.0), 2.0),
        (-(STATION_X / 2.0 - 94.0), STATION_Y / 2.0 - 88.0, 4.0),
        (STATION_X / 2.0 - 94.0, STATION_Y / 2.0 - 88.0, 6.0),
    ]
    .iter()
    .enumerate()
    {
        feet = feet
            + centered_cylinder(
                name(&format!("tilt_reference_foot_{i}_{z_extra:.0}mm")),
                TILT_FOOT_D / 2.0,
                TILT_FOOT_BASE_Z + z_extra,
                36,
            )
            .translate(*x, *y, -(TILT_FOOT_BASE_Z + z_extra) / 2.0)
            + centered_cube(
                name(&format!("tilt_reference_foot_index_land_{i}")),
                34.0,
                12.0,
                3.0,
            )
            .translate(*x, *y + 26.0, BASE_Z + 1.5);
    }

    feet + tilt_reference_rail()
}

fn tilt_reference_rail() -> Part {
    let rail = centered_cube(
        name("tilt_reference_inclinometer_rail"),
        TILT_RAIL_X,
        TILT_RAIL_Y,
        TILT_RAIL_Z,
    )
    .translate(
        TILT_REFERENCE_CENTER.0,
        TILT_REFERENCE_CENTER.1,
        stage_z(TILT_RAIL_Z),
    );
    let mut shims = Part::empty(name("tilt_reference_shim_steps"));
    for shim in 0..TILT_REFERENCE_SHIMS {
        let height = (shim as f64 + 1.0) * (TILT_REFERENCE_MAX_MM / TILT_REFERENCE_SHIMS as f64);
        shims = shims
            + centered_cube(
                name(&format!("tilt_reference_{height:.0}mm_shim_land")),
                44.0,
                32.0,
                height,
            )
            .translate(
                TILT_REFERENCE_CENTER.0 + lane_offset(shim, TILT_REFERENCE_SHIMS, 58.0),
                TILT_REFERENCE_CENTER.1,
                BASE_Z + TILT_RAIL_Z + height / 2.0,
            )
            + centered_cube(
                name(&format!("tilt_reference_shim_read_land_{shim}")),
                34.0,
                8.0,
                2.0,
            )
            .translate(
                TILT_REFERENCE_CENTER.0 + lane_offset(shim, TILT_REFERENCE_SHIMS, 58.0),
                TILT_REFERENCE_CENTER.1 + 24.0,
                BASE_Z + TILT_RAIL_Z + height + 1.0,
            );
    }
    rail + shims
}

fn dye_recovery_pockets() -> Part {
    let rack = centered_cube(
        name("dye_recovery_pocket_rack"),
        DYE_RACK_X,
        DYE_RACK_Y,
        DYE_RACK_Z,
    )
    .translate(DYE_CENTER.0, DYE_CENTER.1, stage_z(DYE_RACK_Z));
    let mut cuts = Part::empty(name("dye_recovery_pocket_cuts"));
    let mut lands = Part::empty(name("dye_recovery_pocket_lands"));

    for pocket in 0..DYE_RECOVERY_POCKETS {
        let x = DYE_CENTER.0 + lane_offset(pocket, DYE_RECOVERY_POCKETS, DYE_POCKET_PITCH_X);
        cuts = cuts
            + centered_cylinder(
                name(&format!("dye_recovery_pocket_cut_{pocket}")),
                DYE_POCKET_D / 2.0,
                DYE_RACK_Z + 5.0,
                30,
            )
            .translate(x, DYE_CENTER.1 + 20.0, stage_z(DYE_RACK_Z));
        lands = lands
            + centered_cylinder(
                name(&format!("dye_recovery_pocket_splash_rim_{pocket}")),
                DYE_POCKET_D / 2.0 + 4.0,
                4.0,
                30,
            )
            .translate(x, DYE_CENTER.1 + 20.0, BASE_Z + DYE_RACK_Z + 2.0)
            - centered_cylinder(
                name(&format!("dye_recovery_pocket_opening_{pocket}")),
                DYE_POCKET_D / 2.0,
                5.0,
                30,
            )
            .translate(x, DYE_CENTER.1 + 20.0, BASE_Z + DYE_RACK_Z + 2.0)
            + centered_cube(name(&format!("dye_blotter_land_{pocket}")), 34.0, 18.0, 3.0)
                .translate(x, DYE_CENTER.1 - 30.0, BASE_Z + DYE_RACK_Z + 1.5);
    }

    rack - cuts + lands + dye_reference_card_lands()
}

fn dye_reference_card_lands() -> Part {
    let mut cards = Part::empty(name("dye_reference_card_lands"));
    for card in 0..DYE_CARD_LANDS {
        let x = DYE_CENTER.0 - DYE_RACK_X / 2.0 + 44.0 + card as f64 * 72.0;
        cards = cards
            + centered_cube(
                name(&format!("dye_color_reference_card_land_{card}")),
                48.0,
                20.0,
                4.0,
            )
            .translate(
                x,
                DYE_CENTER.1 - DYE_RACK_Y / 2.0 + 18.0,
                BASE_Z + DYE_RACK_Z + 2.0,
            );
    }
    cards
}

fn bubble_wetness_windows() -> Part {
    let panel = centered_cube(
        name("bubble_wetness_inspection_window_panel"),
        WINDOW_PANEL_X,
        WINDOW_PANEL_Y,
        WINDOW_PANEL_Z,
    )
    .translate(WINDOW_CENTER.0, WINDOW_CENTER.1, stage_z(WINDOW_PANEL_Z));
    let mut cuts = Part::empty(name("bubble_wetness_window_cuts"));
    let mut frames = Part::empty(name("bubble_wetness_window_frames"));

    for window in 0..BUBBLE_WETNESS_WINDOWS {
        let x = WINDOW_CENTER.0 + lane_offset(window, BUBBLE_WETNESS_WINDOWS, WINDOW_PITCH_X);
        cuts = cuts
            + centered_cube(
                name(&format!("bubble_sight_window_cut_{window}")),
                BUBBLE_WINDOW_X,
                BUBBLE_WINDOW_Y,
                WINDOW_PANEL_Z + 4.0,
            )
            .translate(x, WINDOW_CENTER.1 + 18.0, stage_z(WINDOW_PANEL_Z))
            + centered_cube(
                name(&format!("wetness_pad_recess_{window}")),
                WETNESS_PAD_X,
                WETNESS_PAD_Y,
                WINDOW_PANEL_Z + 4.0,
            )
            .translate(x, WINDOW_CENTER.1 - 42.0, stage_z(WINDOW_PANEL_Z));
        frames = frames
            + centered_cube(
                name(&format!("bubble_window_top_frame_{window}")),
                BUBBLE_WINDOW_X + 10.0,
                4.0,
                WINDOW_FRAME_Z,
            )
            .translate(
                x,
                WINDOW_CENTER.1 + 18.0 + BUBBLE_WINDOW_Y / 2.0 + 4.0,
                BASE_Z + WINDOW_PANEL_Z + WINDOW_FRAME_Z / 2.0,
            )
            + centered_cube(
                name(&format!("bubble_window_bottom_frame_{window}")),
                BUBBLE_WINDOW_X + 10.0,
                4.0,
                WINDOW_FRAME_Z,
            )
            .translate(
                x,
                WINDOW_CENTER.1 + 18.0 - BUBBLE_WINDOW_Y / 2.0 - 4.0,
                BASE_Z + WINDOW_PANEL_Z + WINDOW_FRAME_Z / 2.0,
            )
            + centered_cube(
                name(&format!("wetness_pad_witness_land_{window}")),
                WETNESS_PAD_X + 10.0,
                WETNESS_PAD_Y + 8.0,
                3.0,
            )
            .translate(x, WINDOW_CENTER.1 - 42.0, BASE_Z + WINDOW_PANEL_Z + 1.5)
            - centered_cube(
                name(&format!("wetness_pad_opening_{window}")),
                WETNESS_PAD_X,
                WETNESS_PAD_Y,
                4.0,
            )
            .translate(x, WINDOW_CENTER.1 - 42.0, BASE_Z + WINDOW_PANEL_Z + 1.5);
    }

    panel - cuts + frames + wetness_window_reference_strip()
}

fn wetness_window_reference_strip() -> Part {
    let strip = centered_cube(
        name("bubble_wetness_reference_strip"),
        WINDOW_PANEL_X - 34.0,
        12.0,
        4.0,
    )
    .translate(
        WINDOW_CENTER.0,
        WINDOW_CENTER.1 + WINDOW_PANEL_Y / 2.0 - 18.0,
        BASE_Z + WINDOW_PANEL_Z + 2.0,
    );
    let mut ticks = Part::empty(name("bubble_wetness_reference_ticks"));
    for tick in 0..=BUBBLE_WETNESS_WINDOWS {
        let x = WINDOW_CENTER.0
            + (tick as f64 - BUBBLE_WETNESS_WINDOWS as f64 / 2.0) * WINDOW_PITCH_X
            - WINDOW_PITCH_X / 2.0;
        ticks = ticks
            + centered_cube(
                name(&format!("bubble_wetness_reference_tick_{tick}")),
                2.0,
                18.0,
                5.0,
            )
            .translate(
                x,
                WINDOW_CENTER.1 + WINDOW_PANEL_Y / 2.0 - 18.0,
                BASE_Z + WINDOW_PANEL_Z + 2.5,
            );
    }
    strip + ticks
}

fn barcode_certificate_lands() -> Part {
    let panel = centered_cube(
        name("barcode_certificate_land_panel"),
        TRACE_PANEL_X,
        TRACE_PANEL_Y,
        TRACE_PANEL_Z,
    )
    .translate(TRACE_CENTER.0, TRACE_CENTER.1, stage_z(TRACE_PANEL_Z));
    panel + barcode_lands() + certificate_lands() + trace_fiducials()
}

fn barcode_lands() -> Part {
    let mut lands = Part::empty(name("barcode_scan_lands"));
    for land in 0..BARCODE_LANDS {
        let row = land / 4;
        let col = land % 4;
        let x = TRACE_CENTER.0 + lane_offset(col, 4, 64.0);
        let y = TRACE_CENTER.1 + 20.0 + lane_offset(row, 2, 26.0);
        lands = lands
            + centered_cube(
                name(&format!("barcode_scan_land_{land}")),
                BARCODE_LAND_X,
                BARCODE_LAND_Y,
                3.0,
            )
            .translate(x, y, BASE_Z + TRACE_PANEL_Z + 1.5);
    }
    lands
}

fn certificate_lands() -> Part {
    let mut lands = Part::empty(name("certificate_and_run_record_lands"));
    for land in 0..CERTIFICATE_LANDS {
        let x = TRACE_CENTER.0 + lane_offset(land, CERTIFICATE_LANDS, 96.0);
        lands = lands
            + centered_cube(
                name(&format!("calibration_certificate_land_{land}")),
                CERTIFICATE_LAND_X,
                CERTIFICATE_LAND_Y,
                3.0,
            )
            .translate(x, TRACE_CENTER.1 - 32.0, BASE_Z + TRACE_PANEL_Z + 1.5);
    }
    lands
}

fn trace_fiducials() -> Part {
    let mut fiducials = Part::empty(name("traceability_optical_fiducials"));
    for (i, (x, y)) in [
        (
            TRACE_CENTER.0 - TRACE_PANEL_X / 2.0 + 18.0,
            TRACE_CENTER.1 - TRACE_PANEL_Y / 2.0 + 18.0,
        ),
        (
            TRACE_CENTER.0 + TRACE_PANEL_X / 2.0 - 18.0,
            TRACE_CENTER.1 - TRACE_PANEL_Y / 2.0 + 18.0,
        ),
        (
            TRACE_CENTER.0 - TRACE_PANEL_X / 2.0 + 18.0,
            TRACE_CENTER.1 + TRACE_PANEL_Y / 2.0 - 18.0,
        ),
        (
            TRACE_CENTER.0 + TRACE_PANEL_X / 2.0 - 18.0,
            TRACE_CENTER.1 + TRACE_PANEL_Y / 2.0 - 18.0,
        ),
    ]
    .iter()
    .enumerate()
    {
        fiducials = fiducials
            + centered_cylinder(
                name(&format!("traceability_fiducial_ring_{i}")),
                7.0,
                3.0,
                28,
            )
            .translate(*x, *y, BASE_Z + TRACE_PANEL_Z + 1.5)
            - centered_cylinder(
                name(&format!("traceability_fiducial_center_{i}")),
                2.2,
                4.0,
                20,
            )
            .translate(*x, *y, BASE_Z + TRACE_PANEL_Z + 1.5);
    }
    fiducials
}

fn release_hold_reject_lanes() -> Part {
    let mut lanes = Part::empty(name("release_hold_reject_disposition_lanes"));
    for (lane, label) in ["release", "hold", "reject"].iter().enumerate() {
        let x = DISPOSITION_CENTER.0 + lane_offset(lane, DISPOSITION_LANES, LANE_PITCH_X);
        let base = centered_cube(name(&format!("{label}_lane_base")), LANE_X, LANE_Y, LANE_Z)
            .translate(x, DISPOSITION_CENTER.1, stage_z(LANE_Z));
        let pocket = centered_cube(
            name(&format!("{label}_lane_manifest_card_pocket")),
            LANE_X - 42.0,
            LANE_Y - 42.0,
            LANE_Z + 4.0,
        )
        .translate(x, DISPOSITION_CENTER.1, stage_z(LANE_Z));
        let left_rail = centered_cube(
            name(&format!("{label}_lane_left_rail")),
            10.0,
            LANE_Y,
            LANE_RAIL_Z,
        )
        .translate(
            x - LANE_X / 2.0 + 12.0,
            DISPOSITION_CENTER.1,
            BASE_Z + LANE_RAIL_Z / 2.0,
        );
        let right_rail = centered_cube(
            name(&format!("{label}_lane_right_rail")),
            10.0,
            LANE_Y,
            LANE_RAIL_Z,
        )
        .translate(
            x + LANE_X / 2.0 - 12.0,
            DISPOSITION_CENTER.1,
            BASE_Z + LANE_RAIL_Z / 2.0,
        );
        let gate = centered_cube(
            name(&format!("{label}_lane_front_gate")),
            LANE_X - 20.0,
            10.0,
            LANE_GATE_Z,
        )
        .translate(
            x,
            DISPOSITION_CENTER.1 - LANE_Y / 2.0 + 10.0,
            BASE_Z + LANE_GATE_Z / 2.0,
        );
        let status_land = centered_cube(
            name(&format!("{label}_status_barcode_land")),
            82.0,
            16.0,
            4.0,
        )
        .translate(
            x,
            DISPOSITION_CENTER.1 + LANE_Y / 2.0 - 18.0,
            BASE_Z + LANE_Z + 2.0,
        );

        lanes = lanes + (base - pocket) + left_rail + right_rail + gate + status_land;
    }
    lanes
}

fn evidence_bridge() -> Part {
    let left_column = centered_cube(
        name("evidence_bridge_left_column"),
        EVIDENCE_COLUMN_X,
        EVIDENCE_COLUMN_Y,
        EVIDENCE_CLEAR_Z,
    )
    .translate(
        EVIDENCE_CENTER.0 - EVIDENCE_BRIDGE_X / 2.0,
        EVIDENCE_CENTER.1,
        BASE_Z + EVIDENCE_CLEAR_Z / 2.0,
    );
    let right_column = centered_cube(
        name("evidence_bridge_right_column"),
        EVIDENCE_COLUMN_X,
        EVIDENCE_COLUMN_Y,
        EVIDENCE_CLEAR_Z,
    )
    .translate(
        EVIDENCE_CENTER.0 + EVIDENCE_BRIDGE_X / 2.0,
        EVIDENCE_CENTER.1,
        BASE_Z + EVIDENCE_CLEAR_Z / 2.0,
    );
    let crossbar = centered_cube(
        name("evidence_bridge_camera_crossbar"),
        EVIDENCE_BRIDGE_X,
        EVIDENCE_BRIDGE_Y,
        EVIDENCE_CROSSBAR_Z,
    )
    .translate(
        EVIDENCE_CENTER.0,
        EVIDENCE_CENTER.1,
        BASE_Z + EVIDENCE_CLEAR_Z + EVIDENCE_CROSSBAR_Z / 2.0,
    );

    left_column + right_column + (crossbar - evidence_window_cuts()) + evidence_camera_lands()
}

fn evidence_window_cuts() -> Part {
    let mut cuts = Part::empty(name("evidence_bridge_window_cuts"));
    for window in 0..EVIDENCE_CAMERA_WINDOWS {
        let x = EVIDENCE_CENTER.0 + lane_offset(window, EVIDENCE_CAMERA_WINDOWS, 180.0);
        cuts = cuts
            + centered_cube(
                name(&format!("evidence_bridge_camera_window_cut_{window}")),
                EVIDENCE_WINDOW_X,
                EVIDENCE_WINDOW_Y,
                EVIDENCE_CROSSBAR_Z + 4.0,
            )
            .translate(
                x,
                EVIDENCE_CENTER.1,
                BASE_Z + EVIDENCE_CLEAR_Z + EVIDENCE_CROSSBAR_Z / 2.0,
            );
    }
    cuts
}

fn evidence_camera_lands() -> Part {
    let mut lands = Part::empty(name("evidence_camera_mount_lands"));
    for window in 0..EVIDENCE_CAMERA_WINDOWS {
        let x = EVIDENCE_CENTER.0 + lane_offset(window, EVIDENCE_CAMERA_WINDOWS, 180.0);
        lands = lands
            + centered_cube(
                name(&format!("evidence_camera_mount_land_{window}")),
                70.0,
                14.0,
                5.0,
            )
            .translate(
                x,
                EVIDENCE_CENTER.1 + EVIDENCE_BRIDGE_Y / 2.0 + 12.0,
                BASE_Z + EVIDENCE_CLEAR_Z + EVIDENCE_CROSSBAR_Z + 2.5,
            );
    }
    lands
}

fn robot_service_keepouts() -> Part {
    let approach_front = centered_cube(
        name("front_robot_approach_keepout_edge"),
        ROBOT_APPROACH_X,
        8.0,
        10.0,
    )
    .translate(0.0, -ROBOT_APPROACH_Y / 2.0, ROBOT_KEEP_OUT_Z);
    let approach_rear = centered_cube(
        name("rear_robot_approach_keepout_edge"),
        ROBOT_APPROACH_X,
        8.0,
        10.0,
    )
    .translate(0.0, ROBOT_APPROACH_Y / 2.0, ROBOT_KEEP_OUT_Z);
    let approach_left = centered_cube(
        name("left_robot_approach_keepout_edge"),
        8.0,
        ROBOT_APPROACH_Y,
        10.0,
    )
    .translate(-ROBOT_APPROACH_X / 2.0, 0.0, ROBOT_KEEP_OUT_Z);
    let approach_right = centered_cube(
        name("right_robot_approach_keepout_edge"),
        8.0,
        ROBOT_APPROACH_Y,
        10.0,
    )
    .translate(ROBOT_APPROACH_X / 2.0, 0.0, ROBOT_KEEP_OUT_Z);

    approach_front
        + approach_rear
        + approach_left
        + approach_right
        + service_clearance_lands()
        + keepout_corner_posts()
}

fn service_clearance_lands() -> Part {
    let front = centered_cube(name("front_service_clearance_land"), 250.0, 22.0, 4.0).translate(
        0.0,
        -(STATION_Y / 2.0 + 34.0),
        BASE_Z + 2.0,
    );
    let right = centered_cube(
        name("right_capture_well_service_clearance_land"),
        22.0,
        180.0,
        4.0,
    )
    .translate(STATION_X / 2.0 + 34.0, CAPTURE_CENTER.1, BASE_Z + 2.0);
    let rear = centered_cube(
        name("rear_evidence_service_clearance_land"),
        250.0,
        22.0,
        4.0,
    )
    .translate(0.0, STATION_Y / 2.0 + 34.0, BASE_Z + 2.0);
    let left = centered_cube(name("left_tubing_service_clearance_land"), 22.0, 180.0, 4.0)
        .translate(-(STATION_X / 2.0 + 34.0), MANIFOLD_CENTER.1, BASE_Z + 2.0);

    front + right + rear + left
}

fn keepout_corner_posts() -> Part {
    let mut posts = Part::empty(name("robot_keepout_corner_posts"));
    for (i, (x, y)) in [
        (-ROBOT_APPROACH_X / 2.0, -ROBOT_APPROACH_Y / 2.0),
        (ROBOT_APPROACH_X / 2.0, -ROBOT_APPROACH_Y / 2.0),
        (-ROBOT_APPROACH_X / 2.0, ROBOT_APPROACH_Y / 2.0),
        (ROBOT_APPROACH_X / 2.0, ROBOT_APPROACH_Y / 2.0),
    ]
    .iter()
    .enumerate()
    {
        posts = posts
            + centered_cylinder(
                name(&format!("robot_keepout_corner_post_{i}")),
                6.0,
                ROBOT_KEEP_OUT_Z,
                20,
            )
            .translate(*x, *y, BASE_Z + ROBOT_KEEP_OUT_Z / 2.0);
    }
    posts
}

fn lane_offset(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

fn stage_z(height: f64) -> f64 {
    BASE_Z + height / 2.0
}

fn name(feature: &str) -> String {
    format!("{PREFIX}_{feature}")
}
