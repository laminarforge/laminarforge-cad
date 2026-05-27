use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed condensate drain-trap biofilm cleanability validation station.
//
// Scope:
// - No-cell fixture for incubator/module condensate drain trap cleanability.
// - Packages a trap body surrogate nest, removable drain-line coupon slots,
//   biofilm witness coupons, rinse/flush ports, siphon/backflow dye wells,
//   residue sampling geometry, slope gauges, evidence capture, and explicit
//   robot/service keepouts.
// - Mechanical validation CAD only. This does not define a live-cell process,
//   microbial growth protocol, cleaning chemistry, release rule, or acceptance
//   criterion.

const OUTPUTS: &[&str] = &[
    "output/closed_condensate_drain_trap_biofilm_cleanability_station_leak_tray_base.stl",
    "output/closed_condensate_drain_trap_biofilm_cleanability_station_trap_body_surrogate_nest.stl",
    "output/closed_condensate_drain_trap_biofilm_cleanability_station_removable_drain_line_coupon_slots.stl",
    "output/closed_condensate_drain_trap_biofilm_cleanability_station_biofilm_witness_coupon_rack.stl",
    "output/closed_condensate_drain_trap_biofilm_cleanability_station_rinse_flush_port_manifold.stl",
    "output/closed_condensate_drain_trap_biofilm_cleanability_station_backflow_dye_wells.stl",
    "output/closed_condensate_drain_trap_biofilm_cleanability_station_residue_swab_slots.stl",
    "output/closed_condensate_drain_trap_biofilm_cleanability_station_drain_slope_gauges.stl",
    "output/closed_condensate_drain_trap_biofilm_cleanability_station_barcode_status_lanes.stl",
    "output/closed_condensate_drain_trap_biofilm_cleanability_station_camera_evidence_bridge.stl",
    "output/closed_condensate_drain_trap_biofilm_cleanability_station_robot_service_keepouts.stl",
    "output/closed_condensate_drain_trap_biofilm_cleanability_station_assembly.stl",
];

const REQUIRED_FEATURES: &[&str] = &[
    "trap_body_surrogate_nest",
    "removable_drain_line_coupon_slots",
    "biofilm_witness_coupon_rack",
    "rinse_flush_port_manifold",
    "backflow_dye_wells",
    "residue_swab_slots",
    "drain_slope_gauges",
    "leak_tray",
    "barcode_status_lanes",
    "camera_evidence_bridge",
    "robot_service_keepouts",
    "assembly_export",
];

const STATION_X: f64 = 1240.0;
const STATION_Y: f64 = 800.0;
const BASE_Z: f64 = 24.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 46.0;
const LEAK_BASIN_DEPTH: f64 = 9.0;
const SOCKET_DEPTH: f64 = 5.4;
const MOUNT_HOLES: usize = 8;
const LEAK_SENSOR_WELLS: usize = 6;

const TRAP_NEST_X: f64 = 360.0;
const TRAP_NEST_Y: f64 = 250.0;
const TRAP_NEST_Z: f64 = 66.0;
const TRAP_BODY_D: f64 = 126.0;
const TRAP_BOWL_D: f64 = 92.0;
const TRAP_INLET_OUTLET_D: f64 = 28.0;
const TRAP_LOCATOR_PINS: usize = 4;
const TRAP_POS: (f64, f64) = (-360.0, 165.0);

const DRAIN_COUPON_RAIL_X: f64 = 530.0;
const DRAIN_COUPON_RAIL_Y: f64 = 152.0;
const DRAIN_COUPON_RAIL_Z: f64 = 42.0;
const DRAIN_COUPON_SLOTS: usize = 10;
const DRAIN_COUPON_SLOT_X: f64 = 38.0;
const DRAIN_COUPON_SLOT_Y: f64 = 102.0;
const DRAIN_COUPON_SLOT_Z: f64 = 24.0;
const DRAIN_COUPON_PITCH_X: f64 = 48.0;
const DRAIN_COUPON_POS: (f64, f64) = (240.0, 192.0);

const BIOFILM_RACK_X: f64 = 430.0;
const BIOFILM_RACK_Y: f64 = 172.0;
const BIOFILM_RACK_Z: f64 = 48.0;
const BIOFILM_COUPON_COUNT: usize = 12;
const BIOFILM_COUPON_PITCH_X: f64 = 32.0;
const BIOFILM_SLOT_X: f64 = 18.0;
const BIOFILM_SLOT_Y: f64 = 116.0;
const BIOFILM_SLOT_Z: f64 = 32.0;
const BIOFILM_POS: (f64, f64) = (-342.0, -42.0);

const MANIFOLD_X: f64 = 510.0;
const MANIFOLD_Y: f64 = 138.0;
const MANIFOLD_Z: f64 = 56.0;
const RINSE_PORTS: usize = 6;
const FLUSH_PORTS: usize = 6;
const PORT_PITCH_X: f64 = 70.0;
const RINSE_PORT_D: f64 = 10.0;
const FLUSH_PORT_D: f64 = 14.0;
const MANIFOLD_POS: (f64, f64) = (290.0, -36.0);

const DYE_WELL_BANK_X: f64 = 370.0;
const DYE_WELL_BANK_Y: f64 = 150.0;
const DYE_WELL_BANK_Z: f64 = 34.0;
const DYE_WELLS: usize = 8;
const DYE_WELL_D: f64 = 24.0;
const DYE_WELL_PITCH_X: f64 = 42.0;
const CHECK_VALVE_WITNESSES: usize = 4;
const DYE_POS: (f64, f64) = (370.0, -220.0);

const SWAB_BANK_X: f64 = 430.0;
const SWAB_BANK_Y: f64 = 132.0;
const SWAB_BANK_Z: f64 = 30.0;
const SWAB_SLOTS: usize = 8;
const SWAB_SLOT_X: f64 = 34.0;
const SWAB_SLOT_Y: f64 = 86.0;
const SWAB_SLOT_Z: f64 = 18.0;
const SWAB_POS: (f64, f64) = (-340.0, -240.0);

const SLOPE_GAUGE_X: f64 = 520.0;
const SLOPE_GAUGE_Y: f64 = 88.0;
const SLOPE_GAUGE_Z: f64 = 34.0;
const SLOPE_GAUGES: usize = 5;
const SLOPE_TICK_COUNT: usize = 7;
const SLOPE_POS: (f64, f64) = (18.0, -320.0);

const TRACE_X: f64 = 900.0;
const TRACE_Y: f64 = 78.0;
const TRACE_Z: f64 = 10.0;
const BARCODE_LANDS: usize = 10;
const STATUS_LANES: usize = 3;
const TRACE_POS: (f64, f64) = (-18.0, 327.0);

const CAMERA_BRIDGE_X: f64 = 1030.0;
const CAMERA_BRIDGE_Y: f64 = 70.0;
const CAMERA_POST_Z: f64 = 210.0;
const CAMERA_POST_X: f64 = 28.0;
const CAMERA_POST_Y: f64 = 42.0;
const CAMERA_LANDS: usize = 4;
const LIGHT_BARS: usize = 4;
const CAMERA_POS: (f64, f64) = (0.0, 32.0);

const ROBOT_KEEP_OUT_Z: f64 = 184.0;
const FRONT_ROBOT_CLEARANCE: f64 = 30.0;
const REAR_SERVICE_CLEARANCE: f64 = 30.0;
const LEFT_TRAP_SERVICE_CLEARANCE: f64 = 70.0;
const RIGHT_MANIFOLD_SERVICE_CLEARANCE: f64 = 70.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_layout();

    let base = leak_tray_base();
    export(&base, OUTPUTS[0]);

    let trap = trap_body_surrogate_nest();
    export(&trap, OUTPUTS[1]);

    let drain_coupons = removable_drain_line_coupon_slots();
    export(&drain_coupons, OUTPUTS[2]);

    let biofilm = biofilm_witness_coupon_rack();
    export(&biofilm, OUTPUTS[3]);

    let manifold = rinse_flush_port_manifold();
    export(&manifold, OUTPUTS[4]);

    let dye = backflow_dye_wells();
    export(&dye, OUTPUTS[5]);

    let swabs = residue_swab_slots();
    export(&swabs, OUTPUTS[6]);

    let slope = drain_slope_gauges();
    export(&slope, OUTPUTS[7]);

    let trace = barcode_status_lanes();
    export(&trace, OUTPUTS[8]);

    let bridge = camera_evidence_bridge();
    export(&bridge, OUTPUTS[9]);

    let keepouts = robot_service_keepouts();
    export(&keepouts, OUTPUTS[10]);

    let assembly = base
        + trap.translate(TRAP_POS.0, TRAP_POS.1, on_base_z(TRAP_NEST_Z))
        + drain_coupons.translate(
            DRAIN_COUPON_POS.0,
            DRAIN_COUPON_POS.1,
            on_base_z(DRAIN_COUPON_RAIL_Z),
        )
        + biofilm.translate(BIOFILM_POS.0, BIOFILM_POS.1, on_base_z(BIOFILM_RACK_Z))
        + manifold.translate(MANIFOLD_POS.0, MANIFOLD_POS.1, on_base_z(MANIFOLD_Z))
        + dye.translate(DYE_POS.0, DYE_POS.1, on_base_z(DYE_WELL_BANK_Z))
        + swabs.translate(SWAB_POS.0, SWAB_POS.1, on_base_z(SWAB_BANK_Z))
        + slope.translate(SLOPE_POS.0, SLOPE_POS.1, on_base_z(SLOPE_GAUGE_Z))
        + trace.translate(TRACE_POS.0, TRACE_POS.1, on_base_z(TRACE_Z))
        + bridge.translate(CAMERA_POS.0, CAMERA_POS.1, on_base_z(CAMERA_POST_Z))
        + keepouts.translate(0.0, 0.0, BASE_Z / 2.0 + ROBOT_KEEP_OUT_Z / 2.0);
    export(&assembly, OUTPUTS[11]);

    println!();
    println!("Closed condensate drain-trap biofilm cleanability validation station:");
    println!("  Footprint:            {STATION_X:.0}mm x {STATION_Y:.0}mm leak tray");
    println!(
        "  Cleanability coupons: {DRAIN_COUPON_SLOTS} removable drain-line coupons and {BIOFILM_COUPON_COUNT} biofilm witness coupons"
    );
    println!(
        "  Fluid challenge:      {RINSE_PORTS} rinse ports, {FLUSH_PORTS} flush ports, {DYE_WELLS} backflow dye wells, and {CHECK_VALVE_WITNESSES} check-valve witnesses"
    );
    println!(
        "  Sampling/evidence:    {SWAB_SLOTS} swab slots, {SLOPE_GAUGES} slope gauges, {BARCODE_LANDS} barcode lands, {STATUS_LANES} status lanes, {CAMERA_LANDS} camera lands"
    );
    println!("  Required features:    {}", REQUIRED_FEATURES.len());
    println!("  Limitation:           No-cell mechanical validation fixture only; no cleaning chemistry, growth protocol, or acceptance criterion is encoded.");
}

fn export(part: &Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn on_base_z(part_z: f64) -> f64 {
    BASE_Z / 2.0 + part_z / 2.0
}

fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn assert_layout() {
    assert_eq!(OUTPUTS.len(), 12);
    assert_eq!(REQUIRED_FEATURES.len(), 12);
    assert_eq!(RINSE_PORTS, FLUSH_PORTS);
    assert_eq!(TRAP_LOCATOR_PINS, 4);
    assert!(leak_tray_hold_up_ml() > maximum_challenge_volume_ml());
    assert!(front_robot_clearance() >= FRONT_ROBOT_CLEARANCE);
    assert!(rear_service_clearance() >= REAR_SERVICE_CLEARANCE);
    assert!(left_trap_service_clearance() >= LEFT_TRAP_SERVICE_CLEARANCE);
    assert!(right_manifold_service_clearance() >= RIGHT_MANIFOLD_SERVICE_CLEARANCE);

    for (name, pos, width, depth) in module_specs() {
        assert!(
            fits_inside_leak_tray(pos, width, depth),
            "{name} exceeds leak tray envelope"
        );
    }
}

fn module_specs() -> [(&'static str, (f64, f64), f64, f64); 9] {
    [
        (
            "trap_body_surrogate_nest",
            TRAP_POS,
            TRAP_NEST_X,
            TRAP_NEST_Y,
        ),
        (
            "removable_drain_line_coupon_slots",
            DRAIN_COUPON_POS,
            DRAIN_COUPON_RAIL_X,
            DRAIN_COUPON_RAIL_Y,
        ),
        (
            "biofilm_witness_coupon_rack",
            BIOFILM_POS,
            BIOFILM_RACK_X,
            BIOFILM_RACK_Y,
        ),
        (
            "rinse_flush_port_manifold",
            MANIFOLD_POS,
            MANIFOLD_X,
            MANIFOLD_Y,
        ),
        (
            "backflow_dye_wells",
            DYE_POS,
            DYE_WELL_BANK_X,
            DYE_WELL_BANK_Y,
        ),
        ("residue_swab_slots", SWAB_POS, SWAB_BANK_X, SWAB_BANK_Y),
        (
            "drain_slope_gauges",
            SLOPE_POS,
            SLOPE_GAUGE_X,
            SLOPE_GAUGE_Y,
        ),
        ("barcode_status_lanes", TRACE_POS, TRACE_X, TRACE_Y),
        (
            "camera_evidence_bridge",
            CAMERA_POS,
            CAMERA_BRIDGE_X,
            CAMERA_BRIDGE_Y,
        ),
    ]
}

fn fits_inside_leak_tray(pos: (f64, f64), width: f64, depth: f64) -> bool {
    pos.0.abs() + width / 2.0 <= STATION_X / 2.0 - RIM_W - 8.0
        && pos.1.abs() + depth / 2.0 <= STATION_Y / 2.0 - RIM_W - 8.0
}

fn leak_tray_hold_up_ml() -> f64 {
    let inner_x = STATION_X - 2.0 * RIM_W;
    let inner_y = STATION_Y - 2.0 * RIM_W;
    inner_x * inner_y * (RIM_Z - LEAK_BASIN_DEPTH) / 1000.0
}

fn maximum_challenge_volume_ml() -> f64 {
    DRAIN_COUPON_SLOTS as f64 * 9.0 + BIOFILM_COUPON_COUNT as f64 * 7.0 + DYE_WELLS as f64 * 18.0
}

fn front_robot_clearance() -> f64 {
    STATION_Y / 2.0 - (SLOPE_POS.1.abs() + SLOPE_GAUGE_Y / 2.0)
}

fn rear_service_clearance() -> f64 {
    STATION_Y / 2.0 - (TRACE_POS.1 + TRACE_Y / 2.0)
}

fn left_trap_service_clearance() -> f64 {
    STATION_X / 2.0 - (TRAP_POS.0.abs() + TRAP_NEST_X / 2.0)
}

fn right_manifold_service_clearance() -> f64 {
    STATION_X / 2.0 - (MANIFOLD_POS.0 + MANIFOLD_X / 2.0)
}

fn leak_tray_base() -> Part {
    let deck = centered_cube(
        "condensate_drain_trap_base_deck",
        STATION_X,
        STATION_Y,
        BASE_Z,
    )
    .translate(0.0, 0.0, BASE_Z / 2.0);
    let basin = centered_cube(
        "condensate_drain_trap_base_shallow_leak_basin_cut",
        STATION_X - 118.0,
        STATION_Y - 106.0,
        LEAK_BASIN_DEPTH + 0.8,
    )
    .translate(0.0, -3.0, BASE_Z - LEAK_BASIN_DEPTH / 2.0 + 0.3);
    let drain_bore =
        centered_cylinder("condensate_drain_trap_base_front_drain_bore", 9.0, 56.0, 32)
            .rotate(90.0, 0.0, 0.0)
            .translate(STATION_X / 2.0 - 86.0, -STATION_Y / 2.0 - 2.0, BASE_Z - 8.0);

    deck - basin - drain_bore - locator_sockets() - mount_holes()
        + tray_rims()
        + zone_dividers()
        + leak_sensor_wells()
        + datum_targets()
}

fn locator_sockets() -> Part {
    let mut sockets = Part::empty("condensate_drain_trap_locator_sockets");
    for (name, pos, width, depth) in module_specs() {
        if name == "camera_evidence_bridge" {
            continue;
        }
        sockets = sockets
            + centered_cube(
                format!("condensate_drain_trap_{name}_socket"),
                width + 8.0,
                depth + 8.0,
                SOCKET_DEPTH + 0.6,
            )
            .translate(pos.0, pos.1, BASE_Z - SOCKET_DEPTH / 2.0 + 0.3);
    }
    sockets
}

fn mount_holes() -> Part {
    let mut holes = Part::empty("condensate_drain_trap_mount_holes");
    for (i, (x, y)) in mount_hole_positions().iter().enumerate() {
        holes = holes
            + centered_cylinder(
                format!("condensate_drain_trap_m6_mount_hole_{i}"),
                3.4,
                BASE_Z + 4.0,
                24,
            )
            .translate(*x, *y, BASE_Z / 2.0)
            + centered_cube(
                format!("condensate_drain_trap_mount_slot_relief_{i}"),
                28.0,
                7.0,
                BASE_Z + 4.0,
            )
            .translate(*x, *y, BASE_Z / 2.0);
    }
    holes
}

fn mount_hole_positions() -> [(f64, f64); MOUNT_HOLES] {
    [
        (-(STATION_X / 2.0 - 60.0), -(STATION_Y / 2.0 - 58.0)),
        (STATION_X / 2.0 - 60.0, -(STATION_Y / 2.0 - 58.0)),
        (-(STATION_X / 2.0 - 60.0), STATION_Y / 2.0 - 58.0),
        (STATION_X / 2.0 - 60.0, STATION_Y / 2.0 - 58.0),
        (-215.0, STATION_Y / 2.0 - 58.0),
        (215.0, STATION_Y / 2.0 - 58.0),
        (-215.0, -(STATION_Y / 2.0 - 58.0)),
        (215.0, -(STATION_Y / 2.0 - 58.0)),
    ]
}

fn tray_rims() -> Part {
    let left = centered_cube("condensate_drain_trap_left_rim", RIM_W, STATION_Y, RIM_Z).translate(
        -STATION_X / 2.0 + RIM_W / 2.0,
        0.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let right = centered_cube("condensate_drain_trap_right_rim", RIM_W, STATION_Y, RIM_Z)
        .translate(
            STATION_X / 2.0 - RIM_W / 2.0,
            0.0,
            BASE_Z / 2.0 + RIM_Z / 2.0,
        );
    let rear = centered_cube(
        "condensate_drain_trap_rear_service_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - RIM_W / 2.0,
        BASE_Z / 2.0 + RIM_Z / 2.0,
    );
    let front = centered_cube(
        "condensate_drain_trap_front_low_robot_lip",
        STATION_X - 170.0,
        12.0,
        24.0,
    )
    .translate(0.0, -STATION_Y / 2.0 + 22.0, BASE_Z / 2.0 + 12.0);
    left + right + rear + front
}

fn zone_dividers() -> Part {
    let trap_coupon_row = centered_cube(
        "condensate_drain_trap_trap_coupon_row_divider",
        STATION_X - 170.0,
        8.0,
        20.0,
    )
    .translate(0.0, 76.0, BASE_Z / 2.0 + 10.0);
    let sampling_row = centered_cube(
        "condensate_drain_trap_sampling_row_divider",
        STATION_X - 190.0,
        8.0,
        20.0,
    )
    .translate(0.0, -148.0, BASE_Z / 2.0 + 10.0);
    let center_datum = centered_cube(
        "condensate_drain_trap_center_flow_datum_rib",
        8.0,
        STATION_Y - 150.0,
        18.0,
    )
    .translate(0.0, -18.0, BASE_Z / 2.0 + 9.0);
    trap_coupon_row + sampling_row + center_datum
}

fn leak_sensor_wells() -> Part {
    let mut wells = Part::empty("condensate_drain_trap_leak_sensor_wells");
    for i in 0..LEAK_SENSOR_WELLS {
        let x = centered_index(i, LEAK_SENSOR_WELLS, 72.0);
        wells = wells
            + centered_cylinder(
                format!("condensate_drain_trap_leak_sensor_recess_{i}"),
                12.0,
                4.0,
                24,
            )
            .translate(x, -STATION_Y / 2.0 + 82.0, BASE_Z + 2.0);
    }
    wells
}

fn datum_targets() -> Part {
    let mut targets = Part::empty("condensate_drain_trap_datum_targets");
    for (i, (x, y)) in [
        (-515.0, 302.0),
        (515.0, 302.0),
        (-515.0, -302.0),
        (515.0, -302.0),
    ]
    .iter()
    .enumerate()
    {
        targets = targets
            + centered_cylinder(
                format!("condensate_drain_trap_fiducial_disc_{i}"),
                13.0,
                4.0,
                32,
            )
            .translate(*x, *y, BASE_Z + 2.0)
            + centered_cylinder(
                format!("condensate_drain_trap_fiducial_center_{i}"),
                3.0,
                6.0,
                18,
            )
            .translate(*x, *y, BASE_Z + 4.0);
    }
    targets
}

fn trap_body_surrogate_nest() -> Part {
    let block = centered_cube(
        "condensate_drain_trap_surrogate_nest_block",
        TRAP_NEST_X,
        TRAP_NEST_Y,
        TRAP_NEST_Z,
    );
    let trap_body = centered_cylinder(
        "condensate_drain_trap_surrogate_u_trap_body_clearance",
        TRAP_BODY_D / 2.0,
        TRAP_NEST_Z + 2.0,
        64,
    )
    .translate(-42.0, 0.0, 0.0);
    let bowl = centered_cylinder(
        "condensate_drain_trap_surrogate_bowl_clearance",
        TRAP_BOWL_D / 2.0,
        TRAP_NEST_Z + 2.0,
        64,
    )
    .translate(78.0, -18.0, 0.0);
    let inlet = centered_cylinder(
        "condensate_drain_trap_surrogate_inlet_bore",
        TRAP_INLET_OUTLET_D / 2.0,
        TRAP_NEST_X + 16.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 67.0, 12.0);
    let outlet = centered_cylinder(
        "condensate_drain_trap_surrogate_outlet_bore",
        TRAP_INLET_OUTLET_D / 2.0,
        TRAP_NEST_X + 16.0,
        32,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, -76.0, 12.0);
    let clips = trap_locator_clips();

    block - trap_body - bowl - inlet - outlet + clips + trap_flow_direction_ribs()
}

fn trap_locator_clips() -> Part {
    let mut clips = Part::empty("condensate_drain_trap_locator_clips");
    for (i, (x, y)) in [
        (-128.0, 94.0),
        (128.0, 94.0),
        (-128.0, -104.0),
        (128.0, -104.0),
    ]
    .iter()
    .enumerate()
    {
        clips = clips
            + centered_cylinder(
                format!("condensate_drain_trap_locator_pin_{i}"),
                8.0,
                22.0,
                24,
            )
            .translate(*x, *y, TRAP_NEST_Z / 2.0 + 11.0)
            + centered_cube(
                format!("condensate_drain_trap_clip_land_{i}"),
                42.0,
                18.0,
                16.0,
            )
            .translate(*x, *y, TRAP_NEST_Z / 2.0 + 8.0);
    }
    clips
}

fn trap_flow_direction_ribs() -> Part {
    let inlet_arrow = centered_cube("condensate_drain_trap_inlet_direction_rib", 128.0, 8.0, 9.0)
        .translate(-70.0, 74.0, TRAP_NEST_Z / 2.0 + 4.5);
    let outlet_arrow = centered_cube(
        "condensate_drain_trap_outlet_direction_rib",
        128.0,
        8.0,
        9.0,
    )
    .translate(70.0, -84.0, TRAP_NEST_Z / 2.0 + 4.5);
    let low_point_marker =
        centered_cylinder("condensate_drain_trap_low_point_marker", 18.0, 8.0, 32).translate(
            78.0,
            -18.0,
            TRAP_NEST_Z / 2.0 + 4.0,
        );
    inlet_arrow + outlet_arrow + low_point_marker
}

fn removable_drain_line_coupon_slots() -> Part {
    let rail = centered_cube(
        "condensate_drain_trap_drain_line_coupon_rail",
        DRAIN_COUPON_RAIL_X,
        DRAIN_COUPON_RAIL_Y,
        DRAIN_COUPON_RAIL_Z,
    );
    let mut slots = Part::empty("condensate_drain_trap_drain_line_coupon_slot_cuts");
    let mut handles = Part::empty("condensate_drain_trap_drain_line_coupon_pull_handles");
    for i in 0..DRAIN_COUPON_SLOTS {
        let x = centered_index(i, DRAIN_COUPON_SLOTS, DRAIN_COUPON_PITCH_X);
        slots = slots
            + centered_cube(
                format!("condensate_drain_trap_removable_drain_coupon_pocket_{i}"),
                DRAIN_COUPON_SLOT_X,
                DRAIN_COUPON_SLOT_Y,
                DRAIN_COUPON_SLOT_Z + 1.0,
            )
            .translate(
                x,
                0.0,
                DRAIN_COUPON_RAIL_Z / 2.0 - DRAIN_COUPON_SLOT_Z / 2.0 + 0.5,
            )
            + centered_cylinder(
                format!("condensate_drain_trap_coupon_half_round_tube_witness_{i}"),
                7.0,
                DRAIN_COUPON_SLOT_Y + 8.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, 0.0, DRAIN_COUPON_RAIL_Z / 2.0 + 1.0);
        handles = handles
            + centered_cube(
                format!("condensate_drain_trap_coupon_grip_tab_{i}"),
                24.0,
                16.0,
                14.0,
            )
            .translate(
                x,
                -DRAIN_COUPON_RAIL_Y / 2.0 - 8.0,
                DRAIN_COUPON_RAIL_Z / 2.0 + 7.0,
            );
    }
    rail - slots + handles + drain_coupon_index_ribs()
}

fn drain_coupon_index_ribs() -> Part {
    let mut ribs = Part::empty("condensate_drain_trap_drain_coupon_index_ribs");
    for i in 0..=DRAIN_COUPON_SLOTS {
        let x = -DRAIN_COUPON_RAIL_X / 2.0 + 25.0 + i as f64 * DRAIN_COUPON_PITCH_X;
        ribs = ribs
            + centered_cube(
                format!("condensate_drain_trap_coupon_separator_rib_{i}"),
                4.0,
                122.0,
                12.0,
            )
            .translate(x, 0.0, DRAIN_COUPON_RAIL_Z / 2.0 + 6.0);
    }
    ribs
}

fn biofilm_witness_coupon_rack() -> Part {
    let rack = centered_cube(
        "condensate_drain_trap_biofilm_witness_coupon_rack_body",
        BIOFILM_RACK_X,
        BIOFILM_RACK_Y,
        BIOFILM_RACK_Z,
    );
    let mut slots = Part::empty("condensate_drain_trap_biofilm_witness_coupon_slots");
    let mut flags = Part::empty("condensate_drain_trap_biofilm_witness_coupon_flags");
    for i in 0..BIOFILM_COUPON_COUNT {
        let x = centered_index(i, BIOFILM_COUPON_COUNT, BIOFILM_COUPON_PITCH_X);
        slots = slots
            + centered_cube(
                format!("condensate_drain_trap_biofilm_coupon_vertical_slot_{i}"),
                BIOFILM_SLOT_X,
                BIOFILM_SLOT_Y,
                BIOFILM_SLOT_Z + 1.0,
            )
            .translate(x, 0.0, BIOFILM_RACK_Z / 2.0 - BIOFILM_SLOT_Z / 2.0 + 0.5);
        flags = flags
            + centered_cube(
                format!("condensate_drain_trap_biofilm_coupon_id_flag_{i}"),
                18.0,
                12.0,
                18.0,
            )
            .translate(x, BIOFILM_RACK_Y / 2.0 + 8.0, BIOFILM_RACK_Z / 2.0 + 9.0);
    }
    rack - slots + flags + biofilm_rack_handles()
}

fn biofilm_rack_handles() -> Part {
    let left = centered_cube(
        "condensate_drain_trap_biofilm_rack_left_lift_handle",
        24.0,
        132.0,
        24.0,
    )
    .translate(
        -BIOFILM_RACK_X / 2.0 - 12.0,
        0.0,
        BIOFILM_RACK_Z / 2.0 + 12.0,
    );
    let right = centered_cube(
        "condensate_drain_trap_biofilm_rack_right_lift_handle",
        24.0,
        132.0,
        24.0,
    )
    .translate(
        BIOFILM_RACK_X / 2.0 + 12.0,
        0.0,
        BIOFILM_RACK_Z / 2.0 + 12.0,
    );
    left + right
}

fn rinse_flush_port_manifold() -> Part {
    let body = centered_cube(
        "condensate_drain_trap_rinse_flush_manifold_body",
        MANIFOLD_X,
        MANIFOLD_Y,
        MANIFOLD_Z,
    );
    let header = centered_cylinder(
        "condensate_drain_trap_rinse_flush_header_bore",
        12.0,
        MANIFOLD_X + 20.0,
        40,
    )
    .rotate(0.0, 90.0, 0.0)
    .translate(0.0, 0.0, 12.0);
    let mut ports = Part::empty("condensate_drain_trap_rinse_flush_port_cuts");
    let mut collars = Part::empty("condensate_drain_trap_rinse_flush_port_collars");
    for i in 0..RINSE_PORTS {
        let x = centered_index(i, RINSE_PORTS, PORT_PITCH_X);
        ports = ports
            + centered_cylinder(
                format!("condensate_drain_trap_rinse_port_bore_{i}"),
                RINSE_PORT_D / 2.0,
                MANIFOLD_Z + 6.0,
                24,
            )
            .translate(x, 36.0, 0.0)
            + centered_cylinder(
                format!("condensate_drain_trap_flush_port_bore_{i}"),
                FLUSH_PORT_D / 2.0,
                MANIFOLD_Z + 6.0,
                28,
            )
            .translate(x, -36.0, 0.0);
        collars = collars
            + centered_cylinder(
                format!("condensate_drain_trap_rinse_port_collar_{i}"),
                10.0,
                10.0,
                28,
            )
            .translate(x, 36.0, MANIFOLD_Z / 2.0 + 5.0)
            + centered_cylinder(
                format!("condensate_drain_trap_flush_port_collar_{i}"),
                13.0,
                10.0,
                32,
            )
            .translate(x, -36.0, MANIFOLD_Z / 2.0 + 5.0);
    }
    body - header - ports + collars + manifold_isolation_gates()
}

fn manifold_isolation_gates() -> Part {
    let mut gates = Part::empty("condensate_drain_trap_manifold_isolation_gate_lands");
    for i in 0..3 {
        gates = gates
            + centered_cube(
                format!("condensate_drain_trap_manifold_isolation_gate_{i}"),
                36.0,
                18.0,
                18.0,
            )
            .translate(centered_index(i, 3, 152.0), 0.0, MANIFOLD_Z / 2.0 + 9.0);
    }
    gates
}

fn backflow_dye_wells() -> Part {
    let bank = centered_cube(
        "condensate_drain_trap_backflow_dye_well_bank",
        DYE_WELL_BANK_X,
        DYE_WELL_BANK_Y,
        DYE_WELL_BANK_Z,
    );
    let mut wells = Part::empty("condensate_drain_trap_backflow_dye_well_cuts");
    for i in 0..DYE_WELLS {
        let x = centered_index(i, DYE_WELLS, DYE_WELL_PITCH_X);
        wells = wells
            + centered_cylinder(
                format!("condensate_drain_trap_backflow_dye_well_{i}"),
                DYE_WELL_D / 2.0,
                26.0,
                32,
            )
            .translate(x, 20.0, DYE_WELL_BANK_Z / 2.0 - 10.0)
            + centered_cube(
                format!("condensate_drain_trap_dye_card_land_{i}"),
                28.0,
                18.0,
                8.0,
            )
            .translate(x, -44.0, DYE_WELL_BANK_Z / 2.0 + 4.0);
    }
    bank - wells + check_valve_witness_tabs()
}

fn check_valve_witness_tabs() -> Part {
    let mut tabs = Part::empty("condensate_drain_trap_check_valve_witness_tabs");
    for i in 0..CHECK_VALVE_WITNESSES {
        tabs = tabs
            + centered_cube(
                format!("condensate_drain_trap_check_valve_witness_tab_{i}"),
                56.0,
                16.0,
                16.0,
            )
            .translate(
                centered_index(i, CHECK_VALVE_WITNESSES, 82.0),
                68.0,
                DYE_WELL_BANK_Z / 2.0 + 8.0,
            );
    }
    tabs
}

fn residue_swab_slots() -> Part {
    let bank = centered_cube(
        "condensate_drain_trap_residue_swab_slot_bank",
        SWAB_BANK_X,
        SWAB_BANK_Y,
        SWAB_BANK_Z,
    );
    let mut slots = Part::empty("condensate_drain_trap_residue_swab_slot_cuts");
    let mut caps = Part::empty("condensate_drain_trap_residue_swab_slot_caps");
    for i in 0..SWAB_SLOTS {
        let x = centered_index(i, SWAB_SLOTS, 48.0);
        slots = slots
            + centered_cube(
                format!("condensate_drain_trap_residue_swab_recess_{i}"),
                SWAB_SLOT_X,
                SWAB_SLOT_Y,
                SWAB_SLOT_Z + 1.0,
            )
            .translate(x, 0.0, SWAB_BANK_Z / 2.0 - SWAB_SLOT_Z / 2.0 + 0.5);
        caps = caps
            + centered_cube(
                format!("condensate_drain_trap_swab_retention_clip_{i}"),
                30.0,
                8.0,
                12.0,
            )
            .translate(x, -SWAB_BANK_Y / 2.0 - 5.0, SWAB_BANK_Z / 2.0 + 6.0);
    }
    bank - slots + caps
}

fn drain_slope_gauges() -> Part {
    let base = centered_cube(
        "condensate_drain_trap_drain_slope_gauge_base",
        SLOPE_GAUGE_X,
        SLOPE_GAUGE_Y,
        SLOPE_GAUGE_Z,
    );
    let mut gauges = Part::empty("condensate_drain_trap_drain_slope_gauge_rails");
    for i in 0..SLOPE_GAUGES {
        let x = centered_index(i, SLOPE_GAUGES, 94.0);
        let z = SLOPE_GAUGE_Z / 2.0 + 5.0 + i as f64 * 1.5;
        gauges = gauges
            + centered_cube(
                format!("condensate_drain_trap_slope_gauge_rail_{i}"),
                72.0,
                9.0,
                10.0,
            )
            .rotate(0.0, 0.0, -3.0 + i as f64 * 1.5)
            .translate(x, -12.0, z)
            + centered_cube(
                format!("condensate_drain_trap_slope_gauge_label_land_{i}"),
                58.0,
                18.0,
                6.0,
            )
            .translate(x, 28.0, SLOPE_GAUGE_Z / 2.0 + 3.0);
    }
    base + gauges + slope_tick_marks()
}

fn slope_tick_marks() -> Part {
    let mut ticks = Part::empty("condensate_drain_trap_slope_tick_marks");
    for i in 0..SLOPE_TICK_COUNT {
        ticks = ticks
            + centered_cube(
                format!("condensate_drain_trap_slope_tick_{i}"),
                3.0,
                48.0,
                7.0,
            )
            .translate(
                centered_index(i, SLOPE_TICK_COUNT, 42.0),
                0.0,
                SLOPE_GAUGE_Z / 2.0 + 3.5,
            );
    }
    ticks
}

fn barcode_status_lanes() -> Part {
    let plate = centered_cube(
        "condensate_drain_trap_barcode_status_lane_plate",
        TRACE_X,
        TRACE_Y,
        TRACE_Z,
    );
    let mut lands = Part::empty("condensate_drain_trap_barcode_status_lands");
    for i in 0..BARCODE_LANDS {
        lands = lands
            + centered_cube(
                format!("condensate_drain_trap_barcode_land_{i}"),
                58.0,
                22.0,
                4.0,
            )
            .translate(
                centered_index(i, BARCODE_LANDS, 76.0),
                18.0,
                TRACE_Z / 2.0 + 2.0,
            );
    }
    for i in 0..STATUS_LANES {
        lands = lands
            + centered_cube(
                format!("condensate_drain_trap_status_lane_{i}"),
                250.0,
                22.0,
                8.0,
            )
            .translate(
                centered_index(i, STATUS_LANES, 290.0),
                -20.0,
                TRACE_Z / 2.0 + 4.0,
            );
    }
    plate + lands
}

fn camera_evidence_bridge() -> Part {
    let left_post = centered_cube(
        "condensate_drain_trap_camera_bridge_left_post",
        CAMERA_POST_X,
        CAMERA_POST_Y,
        CAMERA_POST_Z,
    )
    .translate(-CAMERA_BRIDGE_X / 2.0 + 24.0, 0.0, 0.0);
    let right_post = centered_cube(
        "condensate_drain_trap_camera_bridge_right_post",
        CAMERA_POST_X,
        CAMERA_POST_Y,
        CAMERA_POST_Z,
    )
    .translate(CAMERA_BRIDGE_X / 2.0 - 24.0, 0.0, 0.0);
    let beam = centered_cube(
        "condensate_drain_trap_camera_bridge_cross_beam",
        CAMERA_BRIDGE_X,
        24.0,
        24.0,
    )
    .translate(0.0, 0.0, CAMERA_POST_Z / 2.0 + 12.0);
    left_post + right_post + beam + camera_lands() + light_bars()
}

fn camera_lands() -> Part {
    let mut lands = Part::empty("condensate_drain_trap_camera_lands");
    for i in 0..CAMERA_LANDS {
        lands = lands
            + centered_cube(
                format!("condensate_drain_trap_camera_mount_land_{i}"),
                58.0,
                34.0,
                14.0,
            )
            .translate(
                centered_index(i, CAMERA_LANDS, 238.0),
                -18.0,
                CAMERA_POST_Z / 2.0 + 31.0,
            );
    }
    lands
}

fn light_bars() -> Part {
    let mut bars = Part::empty("condensate_drain_trap_evidence_light_bars");
    for i in 0..LIGHT_BARS {
        bars = bars
            + centered_cube(
                format!("condensate_drain_trap_light_bar_{i}"),
                130.0,
                12.0,
                10.0,
            )
            .translate(
                centered_index(i, LIGHT_BARS, 198.0),
                22.0,
                CAMERA_POST_Z / 2.0 + 28.0,
            );
    }
    bars
}

fn robot_service_keepouts() -> Part {
    let robot_reach = centered_cube(
        "condensate_drain_trap_robot_reach_keepout_volume",
        STATION_X - 120.0,
        FRONT_ROBOT_CLEARANCE,
        ROBOT_KEEP_OUT_Z,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 + FRONT_ROBOT_CLEARANCE / 2.0 + 28.0,
        0.0,
    );
    let rear_service = centered_cube(
        "condensate_drain_trap_rear_service_keepout_volume",
        STATION_X - 160.0,
        REAR_SERVICE_CLEARANCE,
        ROBOT_KEEP_OUT_Z * 0.72,
    )
    .translate(
        0.0,
        STATION_Y / 2.0 - REAR_SERVICE_CLEARANCE / 2.0 - 18.0,
        0.0,
    );
    let left_service = centered_cube(
        "condensate_drain_trap_left_trap_service_keepout_volume",
        LEFT_TRAP_SERVICE_CLEARANCE,
        STATION_Y - 190.0,
        ROBOT_KEEP_OUT_Z * 0.55,
    )
    .translate(
        -STATION_X / 2.0 + LEFT_TRAP_SERVICE_CLEARANCE / 2.0 + 22.0,
        -20.0,
        0.0,
    );
    let right_service = centered_cube(
        "condensate_drain_trap_right_manifold_service_keepout_volume",
        RIGHT_MANIFOLD_SERVICE_CLEARANCE,
        STATION_Y - 190.0,
        ROBOT_KEEP_OUT_Z * 0.55,
    )
    .translate(
        STATION_X / 2.0 - RIGHT_MANIFOLD_SERVICE_CLEARANCE / 2.0 - 22.0,
        -20.0,
        0.0,
    );
    robot_reach + rear_service + left_service + right_service
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_manifest_is_stable() {
        assert_eq!(OUTPUTS.len(), 12);
        assert!(OUTPUTS.iter().all(|path| path.starts_with("output/")));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
    }

    #[test]
    fn all_modules_fit_inside_leak_tray() {
        for (_, pos, width, depth) in module_specs() {
            assert!(fits_inside_leak_tray(pos, width, depth));
        }
    }

    #[test]
    fn containment_exceeds_challenge_volume() {
        assert!(leak_tray_hold_up_ml() > maximum_challenge_volume_ml());
    }

    #[test]
    fn full_layout_constraints_hold() {
        assert_layout();
    }
}
