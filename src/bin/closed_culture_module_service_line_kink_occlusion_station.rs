use std::fs;

use vcad::{centered_cube, centered_cylinder, Part};

// Closed culture-module service-line kink/occlusion validation station.
//
// Intent:
// - Package sealed culture-module fluid and gas harness checks before a run
//   without opening the closed service boundary.
// - Fixture a module service-bulkhead coupon, bend-radius gauge combs, kink
//   challenge lanes, pressure/flow witness ports, strain-relief clamps,
//   dye/recovery witness wells, traceability lands, disposition lanes,
//   clean/used segregation, evidence imaging, and robot/service keepout gauges.
//
// This is mechanical validation packaging CAD only. It is not an acceptance
// protocol, clinical claim, biological performance claim, pressure limit, or
// fluid/gas path specification.

const OUTPUT_PREFIX: &str = "output/closed_culture_module_service_line_kink_occlusion_station";
const OUTPUTS: [&str; 13] = [
    "output/closed_culture_module_service_line_kink_occlusion_station_base_leak_tray_deck.stl",
    "output/closed_culture_module_service_line_kink_occlusion_station_service_bulkhead_coupon_nest.stl",
    "output/closed_culture_module_service_line_kink_occlusion_station_bend_radius_gauge_combs.stl",
    "output/closed_culture_module_service_line_kink_occlusion_station_kink_challenge_lanes.stl",
    "output/closed_culture_module_service_line_kink_occlusion_station_pressure_flow_witness_port_panel.stl",
    "output/closed_culture_module_service_line_kink_occlusion_station_tubing_strain_relief_clamp_arrays.stl",
    "output/closed_culture_module_service_line_kink_occlusion_station_dye_recovery_witness_wells.stl",
    "output/closed_culture_module_service_line_kink_occlusion_station_barcode_certificate_lands.stl",
    "output/closed_culture_module_service_line_kink_occlusion_station_release_hold_reject_lanes.stl",
    "output/closed_culture_module_service_line_kink_occlusion_station_clean_used_segregation.stl",
    "output/closed_culture_module_service_line_kink_occlusion_station_evidence_camera_bridge.stl",
    "output/closed_culture_module_service_line_kink_occlusion_station_robot_service_keepout_gauges.stl",
    "output/closed_culture_module_service_line_kink_occlusion_station_assembly.stl",
];

const REQUIRED_FEATURES: [&str; 12] = [
    "base_leak_tray_deck",
    "service_bulkhead_coupon_nest",
    "bend_radius_gauge_combs",
    "kink_challenge_lanes",
    "pressure_flow_witness_port_panel",
    "tubing_strain_relief_clamp_arrays",
    "dye_recovery_witness_wells",
    "barcode_certificate_lands",
    "release_hold_reject_lanes",
    "clean_used_segregation",
    "evidence_camera_bridge",
    "robot_service_keepout_gauges",
];

const STATION_X: f64 = 1320.0;
const STATION_Y: f64 = 880.0;
const DECK_Z: f64 = 22.0;
const LEAK_BASIN_X: f64 = STATION_X - 116.0;
const LEAK_BASIN_Y: f64 = STATION_Y - 104.0;
const LEAK_BASIN_DEPTH: f64 = 7.0;
const RIM_W: f64 = 18.0;
const RIM_Z: f64 = 44.0;
const DRAIN_D: f64 = 18.0;
const MOUNT_HOLE_D: f64 = 7.0;
const LEAK_SENSOR_WELLS: usize = 8;

const BULKHEAD_CENTER_X: f64 = -405.0;
const BULKHEAD_CENTER_Y: f64 = 146.0;
const BULKHEAD_NEST_X: f64 = 360.0;
const BULKHEAD_NEST_Y: f64 = 278.0;
const BULKHEAD_NEST_Z: f64 = 48.0;
const BULKHEAD_COUPON_X: f64 = 260.0;
const BULKHEAD_COUPON_Y: f64 = 170.0;
const BULKHEAD_COUPON_CLEARANCE: f64 = 1.2;
const BULKHEAD_SERVICE_PORTS: usize = 8;
const FLUID_PORTS: usize = 4;
const GAS_PORTS: usize = 4;
const BULKHEAD_PORT_D: f64 = 12.0;
const BULKHEAD_PORT_PITCH_X: f64 = 54.0;
const BULKHEAD_CLAMPS: usize = 6;

const BEND_CENTER_X: f64 = 28.0;
const BEND_CENTER_Y: f64 = 212.0;
const BEND_PLATE_X: f64 = 510.0;
const BEND_PLATE_Y: f64 = 190.0;
const BEND_PLATE_Z: f64 = 28.0;
const BEND_LANES: usize = 6;
const BEND_LANE_PITCH_X: f64 = 72.0;
const BEND_MANDRELS_PER_LANE: usize = 3;
const MIN_RADIUS_MM: f64 = 18.0;
const NOMINAL_RADIUS_MM: f64 = 28.0;
const SERVICE_RADIUS_MM: f64 = 42.0;
const TUBE_OD: f64 = 6.4;
const TUBE_CLEARANCE: f64 = 1.4;
const TUBE_CHANNEL_D: f64 = TUBE_OD + TUBE_CLEARANCE;

const KINK_CENTER_X: f64 = 420.0;
const KINK_CENTER_Y: f64 = 188.0;
const KINK_LANE_BANK_X: f64 = 420.0;
const KINK_LANE_BANK_Y: f64 = 230.0;
const KINK_LANE_BANK_Z: f64 = 32.0;
const KINK_LANES: usize = 6;
const KINK_LANE_PITCH_X: f64 = 62.0;
const KINK_PINCH_GATES: usize = 3;
const KINK_GATE_Z: f64 = 74.0;
const KINK_ROLLER_D: f64 = 20.0;
const KINK_PINCH_SLOT_Y: f64 = 18.0;

const WITNESS_CENTER_X: f64 = 210.0;
const WITNESS_CENTER_Y: f64 = -76.0;
const WITNESS_PANEL_X: f64 = 580.0;
const WITNESS_PANEL_Y: f64 = 126.0;
const WITNESS_PANEL_Z: f64 = 42.0;
const PRESSURE_PORTS: usize = 6;
const FLOW_WITNESS_PORTS: usize = 6;
const WITNESS_PORTS: usize = PRESSURE_PORTS + FLOW_WITNESS_PORTS;
const WITNESS_PORT_PITCH_X: f64 = 44.0;
const PRESSURE_PORT_D: f64 = 8.0;
const FLOW_WINDOW_X: f64 = 28.0;
const FLOW_WINDOW_Y: f64 = 9.0;
const SENSOR_LABEL_LANDS: usize = WITNESS_PORTS;

const CLAMP_CENTER_X: f64 = -390.0;
const CLAMP_CENTER_Y: f64 = -118.0;
const CLAMP_ARRAY_X: f64 = 420.0;
const CLAMP_ARRAY_Y: f64 = 166.0;
const CLAMP_ARRAY_Z: f64 = 28.0;
const CLAMP_ROWS: usize = 2;
const CLAMP_COLS: usize = 8;
const CLAMP_COUNT: usize = CLAMP_ROWS * CLAMP_COLS;
const CLAMP_PITCH_X: f64 = 46.0;
const CLAMP_PITCH_Y: f64 = 78.0;
const CLAMP_BODY_X: f64 = 26.0;
const CLAMP_BODY_Y: f64 = 42.0;
const CLAMP_BODY_Z: f64 = 32.0;

const DYE_CENTER_X: f64 = 160.0;
const DYE_CENTER_Y: f64 = -288.0;
const DYE_RACK_X: f64 = 410.0;
const DYE_RACK_Y: f64 = 172.0;
const DYE_RACK_Z: f64 = 34.0;
const DYE_WELLS: usize = 12;
const DYE_WELL_COLS: usize = 6;
const DYE_WELL_D: f64 = 24.0;
const DYE_WELL_CLEARANCE_D: f64 = 27.0;
const DYE_WELL_PITCH_X: f64 = 52.0;
const DYE_WELL_PITCH_Y: f64 = 74.0;
const RECOVERY_CARD_LANDS: usize = 6;

const TRACE_CENTER_X: f64 = -420.0;
const TRACE_CENTER_Y: f64 = -328.0;
const TRACE_PANEL_X: f64 = 362.0;
const TRACE_PANEL_Y: f64 = 116.0;
const TRACE_PANEL_Z: f64 = 12.0;
const BARCODE_LANDS: usize = 10;
const CERTIFICATE_LANDS: usize = 2;
const RFID_LANDS: usize = 4;
const BARCODE_LAND_X: f64 = 72.0;
const BARCODE_LAND_Y: f64 = 18.0;
const CERTIFICATE_LAND_X: f64 = 132.0;
const CERTIFICATE_LAND_Y: f64 = 42.0;

const DISPOSITION_CENTER_X: f64 = 430.0;
const DISPOSITION_CENTER_Y: f64 = -296.0;
const DISPOSITION_BANK_X: f64 = 394.0;
const DISPOSITION_BANK_Y: f64 = 156.0;
const DISPOSITION_BANK_Z: f64 = 30.0;
const DISPOSITION_LANES: usize = 3;
const DISPOSITION_LANE_X: f64 = 104.0;
const DISPOSITION_LANE_Y: f64 = 112.0;
const DISPOSITION_LANE_PITCH_X: f64 = 126.0;
const RELEASE_LANE: usize = 0;
const HOLD_LANE: usize = 1;
const REJECT_LANE: usize = 2;
const HOLD_REJECT_WALL_Z: f64 = 82.0;

const SEGREGATION_CENTER_X: f64 = -122.0;
const SEGREGATION_BARRIER_X: f64 = 26.0;
const SEGREGATION_BARRIER_Y: f64 = STATION_Y - 116.0;
const SEGREGATION_BARRIER_Z: f64 = 118.0;
const CLEAN_STAGING_X: f64 = 296.0;
const CLEAN_STAGING_Y: f64 = 92.0;
const USED_QUARANTINE_X: f64 = 318.0;
const USED_QUARANTINE_Y: f64 = 96.0;
const TRANSFER_GATE_X: f64 = 112.0;
const TRANSFER_GATE_Y: f64 = 30.0;
const TRANSFER_GATE_Z: f64 = 76.0;

const CAMERA_CENTER_X: f64 = -45.0;
const CAMERA_CENTER_Y: f64 = 348.0;
const CAMERA_BRIDGE_X: f64 = 960.0;
const CAMERA_BRIDGE_Y: f64 = 74.0;
const CAMERA_CLEARANCE_Z: f64 = 176.0;
const CAMERA_BEAM_Z: f64 = 24.0;
const CAMERA_HEADS: usize = 4;
const CAMERA_HEAD_X: f64 = 72.0;
const CAMERA_HEAD_Y: f64 = 48.0;
const CAMERA_HEAD_Z: f64 = 38.0;
const RING_LIGHT_D: f64 = 68.0;
const EVIDENCE_TARGETS: usize = 8;

const ROBOT_KEEPOUT_X: f64 = 1020.0;
const ROBOT_KEEPOUT_Y: f64 = 310.0;
const ROBOT_KEEPOUT_Z: f64 = 170.0;
const FRONT_SERVICE_CLEARANCE: f64 = 360.0;
const REAR_BULKHEAD_SERVICE_CLEARANCE: f64 = 220.0;
const LEFT_CLEAN_SERVICE_CLEARANCE: f64 = 190.0;
const RIGHT_WITNESS_SERVICE_CLEARANCE: f64 = 230.0;
const GAUGE_THICKNESS_Z: f64 = 10.0;

fn main() {
    fs::create_dir_all("output").unwrap();
    assert_design_constraints();

    write_part(base_leak_tray_deck(), OUTPUTS[0]);
    write_part(service_bulkhead_coupon_nest(), OUTPUTS[1]);
    write_part(bend_radius_gauge_combs(), OUTPUTS[2]);
    write_part(kink_challenge_lanes(), OUTPUTS[3]);
    write_part(pressure_flow_witness_port_panel(), OUTPUTS[4]);
    write_part(tubing_strain_relief_clamp_arrays(), OUTPUTS[5]);
    write_part(dye_recovery_witness_wells(), OUTPUTS[6]);
    write_part(barcode_certificate_lands(), OUTPUTS[7]);
    write_part(release_hold_reject_lanes(), OUTPUTS[8]);
    write_part(clean_used_segregation(), OUTPUTS[9]);
    write_part(evidence_camera_bridge(), OUTPUTS[10]);
    write_part(robot_service_keepout_gauges(), OUTPUTS[11]);
    write_part(station_assembly(), OUTPUTS[12]);

    println!();
    println!("Closed culture module service-line kink/occlusion station:");
    println!(
        "  Leak tray/deck:              {STATION_X:.0}mm x {STATION_Y:.0}mm with {LEAK_SENSOR_WELLS} leak sensor wells and {DRAIN_D:.0}mm witness drain"
    );
    println!(
        "  Bulkhead coupon nest:        {BULKHEAD_SERVICE_PORTS} sealed service-port gauges ({FLUID_PORTS} fluid, {GAS_PORTS} gas) and {BULKHEAD_CLAMPS} hold-down clamps"
    );
    println!(
        "  Bend/kink validation gauges: {BEND_LANES} bend lanes, {BEND_MANDRELS_PER_LANE} mandrels per lane, {KINK_LANES} kink challenge lanes, {KINK_PINCH_GATES} pinch-gate stations"
    );
    println!(
        "  Witnessing:                  {PRESSURE_PORTS} pressure ports, {FLOW_WITNESS_PORTS} flow windows, {DYE_WELLS} dye/recovery wells, {CLAMP_COUNT} strain-relief clamps"
    );
    println!(
        "  Disposition/custody:         {BARCODE_LANDS} barcode lands, {CERTIFICATE_LANDS} certificate lands, release/hold/reject lanes, clean/used segregation, and {EVIDENCE_TARGETS} evidence targets"
    );
    println!(
        "  Keepout gauges:              {FRONT_SERVICE_CLEARANCE:.0}mm front service, {REAR_BULKHEAD_SERVICE_CLEARANCE:.0}mm rear bulkhead service, {LEFT_CLEAN_SERVICE_CLEARANCE:.0}mm clean-side service, {RIGHT_WITNESS_SERVICE_CLEARANCE:.0}mm witness-panel service"
    );
    println!(
        "  Output contract:             {} STL files under {OUTPUT_PREFIX}_*, {} required fixture feature groups",
        OUTPUTS.len(),
        REQUIRED_FEATURES.len()
    );
}

fn write_part(part: Part, path: &str) {
    part.write_stl(path).unwrap();
    println!("Exported: {path}");
}

fn station_assembly() -> Part {
    base_leak_tray_deck()
        + service_bulkhead_coupon_nest()
        + bend_radius_gauge_combs()
        + kink_challenge_lanes()
        + pressure_flow_witness_port_panel()
        + tubing_strain_relief_clamp_arrays()
        + dye_recovery_witness_wells()
        + barcode_certificate_lands()
        + release_hold_reject_lanes()
        + clean_used_segregation()
        + evidence_camera_bridge()
        + robot_service_keepout_gauges()
}

fn base_leak_tray_deck() -> Part {
    let deck = centered_cube(
        "service_line_kink_occlusion_station_base_deck",
        STATION_X,
        STATION_Y,
        DECK_Z,
    )
    .translate(0.0, 0.0, DECK_Z / 2.0);
    let basin = centered_cube(
        "service_line_kink_occlusion_station_leak_basin_recess",
        LEAK_BASIN_X,
        LEAK_BASIN_Y,
        LEAK_BASIN_DEPTH,
    )
    .translate(0.0, 0.0, DECK_Z - LEAK_BASIN_DEPTH / 2.0);
    let drain = centered_cylinder(
        "service_line_kink_occlusion_station_front_witness_drain",
        DRAIN_D / 2.0,
        58.0,
        36,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(
        STATION_X / 2.0 - 92.0,
        -STATION_Y / 2.0 + 28.0,
        DECK_Z - 9.0,
    );

    deck - basin - drain - mount_holes()
        + perimeter_rims()
        + leak_sensor_wells()
        + deck_zone_lands()
        + robot_datum_fiducials()
        + shallow_drain_ribs()
}

fn perimeter_rims() -> Part {
    let z = DECK_Z + RIM_Z / 2.0;
    let front = centered_cube(
        "service_line_kink_occlusion_station_front_low_leak_lip",
        STATION_X,
        RIM_W,
        RIM_Z * 0.62,
    )
    .translate(0.0, -STATION_Y / 2.0 + RIM_W / 2.0, DECK_Z + RIM_Z * 0.31);
    let rear = centered_cube(
        "service_line_kink_occlusion_station_rear_leak_rim",
        STATION_X,
        RIM_W,
        RIM_Z,
    )
    .translate(0.0, STATION_Y / 2.0 - RIM_W / 2.0, z);
    let left = centered_cube(
        "service_line_kink_occlusion_station_left_clean_side_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(-STATION_X / 2.0 + RIM_W / 2.0, 0.0, z);
    let right = centered_cube(
        "service_line_kink_occlusion_station_right_used_side_rim",
        RIM_W,
        STATION_Y,
        RIM_Z,
    )
    .translate(STATION_X / 2.0 - RIM_W / 2.0, 0.0, z);

    front + rear + left + right
}

fn mount_holes() -> Part {
    let mut holes = Part::empty("service_line_kink_occlusion_station_mount_holes");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 58.0, -STATION_Y / 2.0 + 58.0),
        (STATION_X / 2.0 - 58.0, -STATION_Y / 2.0 + 58.0),
        (-STATION_X / 2.0 + 58.0, STATION_Y / 2.0 - 58.0),
        (STATION_X / 2.0 - 58.0, STATION_Y / 2.0 - 58.0),
        (0.0, -STATION_Y / 2.0 + 58.0),
        (0.0, STATION_Y / 2.0 - 58.0),
    ]
    .iter()
    .copied()
    .enumerate()
    {
        holes = holes
            + centered_cylinder(
                format!("service_line_kink_occlusion_station_m6_mount_clearance_{i}"),
                MOUNT_HOLE_D / 2.0,
                DECK_Z + 8.0,
                28,
            )
            .translate(x, y, DECK_Z / 2.0);
    }
    holes
}

fn leak_sensor_wells() -> Part {
    let mut wells = Part::empty("service_line_kink_occlusion_station_leak_sensor_wells");
    for i in 0..LEAK_SENSOR_WELLS {
        let x = -510.0 + (i % 4) as f64 * 340.0;
        let y = -318.0 + (i / 4) as f64 * 636.0;
        wells = wells
            + centered_cube(
                format!("service_line_kink_occlusion_station_leak_sensor_recess_{i}"),
                64.0,
                34.0,
                5.0,
            )
            .translate(x, y, DECK_Z + 2.5)
            + centered_cube(
                format!("service_line_kink_occlusion_station_sensor_wire_groove_{i}"),
                92.0,
                7.0,
                5.0,
            )
            .translate(x, y + 28.0, DECK_Z + 2.5);
    }
    wells
}

fn deck_zone_lands() -> Part {
    let clean = centered_cube(
        "service_line_kink_occlusion_station_clean_inbound_land",
        CLEAN_STAGING_X,
        CLEAN_STAGING_Y,
        6.0,
    )
    .translate(-500.0, STATION_Y / 2.0 - 108.0, DECK_Z + 3.0);
    let coupon = centered_cube(
        "service_line_kink_occlusion_station_bulkhead_coupon_load_land",
        302.0,
        86.0,
        6.0,
    )
    .translate(-208.0, STATION_Y / 2.0 - 108.0, DECK_Z + 3.0);
    let used = centered_cube(
        "service_line_kink_occlusion_station_used_harness_quarantine_land",
        USED_QUARANTINE_X,
        USED_QUARANTINE_Y,
        6.0,
    )
    .translate(466.0, STATION_Y / 2.0 - 110.0, DECK_Z + 3.0);

    clean + coupon + used
}

fn robot_datum_fiducials() -> Part {
    let mut fiducials = Part::empty("service_line_kink_occlusion_station_robot_datum_fiducials");
    for (i, (x, y)) in [
        (-STATION_X / 2.0 + 92.0, STATION_Y / 2.0 - 90.0),
        (STATION_X / 2.0 - 92.0, STATION_Y / 2.0 - 90.0),
        (-STATION_X / 2.0 + 92.0, -STATION_Y / 2.0 + 90.0),
        (STATION_X / 2.0 - 92.0, -STATION_Y / 2.0 + 90.0),
    ]
    .iter()
    .copied()
    .enumerate()
    {
        let disc = centered_cylinder(
            format!("service_line_kink_occlusion_station_datum_disc_{i}"),
            14.0,
            3.0,
            36,
        )
        .translate(x, y, DECK_Z + 1.5);
        let bore = centered_cylinder(
            format!("service_line_kink_occlusion_station_datum_center_bore_{i}"),
            3.0,
            5.0,
            20,
        )
        .translate(x, y, DECK_Z + 1.5);
        fiducials = fiducials + (disc - bore);
    }
    fiducials
}

fn shallow_drain_ribs() -> Part {
    let mut ribs = Part::empty("service_line_kink_occlusion_station_shallow_drain_ribs");
    for (i, y) in [-300.0, -150.0, 0.0, 150.0, 300.0]
        .iter()
        .copied()
        .enumerate()
    {
        ribs = ribs
            + centered_cube(
                format!("service_line_kink_occlusion_station_drain_rib_{i}"),
                LEAK_BASIN_X - 120.0,
                7.0,
                5.0,
            )
            .translate(0.0, y, DECK_Z + 2.5);
    }
    ribs
}

fn service_bulkhead_coupon_nest() -> Part {
    let body = centered_cube(
        "service_line_kink_occlusion_bulkhead_coupon_nest_body",
        BULKHEAD_NEST_X,
        BULKHEAD_NEST_Y,
        BULKHEAD_NEST_Z,
    )
    .translate(
        BULKHEAD_CENTER_X,
        BULKHEAD_CENTER_Y,
        deck_insert_z(BULKHEAD_NEST_Z),
    );
    let coupon_relief = centered_cube(
        "service_line_kink_occlusion_bulkhead_coupon_relief",
        BULKHEAD_COUPON_X + BULKHEAD_COUPON_CLEARANCE * 2.0,
        BULKHEAD_COUPON_Y + BULKHEAD_COUPON_CLEARANCE * 2.0,
        BULKHEAD_NEST_Z + 4.0,
    )
    .translate(
        BULKHEAD_CENTER_X,
        BULKHEAD_CENTER_Y,
        deck_insert_z(BULKHEAD_NEST_Z) + 8.0,
    );

    body - coupon_relief
        + bulkhead_rails()
        + bulkhead_service_port_gauges()
        + bulkhead_hold_down_clamps()
        + bulkhead_datum_posts()
}

fn bulkhead_rails() -> Part {
    let z = DECK_Z + BULKHEAD_NEST_Z + 15.0;
    let left = centered_cube(
        "service_line_kink_occlusion_bulkhead_left_datum_rail",
        18.0,
        BULKHEAD_COUPON_Y + 74.0,
        30.0,
    )
    .translate(
        BULKHEAD_CENTER_X - BULKHEAD_COUPON_X / 2.0 - 28.0,
        BULKHEAD_CENTER_Y,
        z,
    );
    let right = centered_cube(
        "service_line_kink_occlusion_bulkhead_right_datum_rail",
        18.0,
        BULKHEAD_COUPON_Y + 74.0,
        30.0,
    )
    .translate(
        BULKHEAD_CENTER_X + BULKHEAD_COUPON_X / 2.0 + 28.0,
        BULKHEAD_CENTER_Y,
        z,
    );
    let rear = centered_cube(
        "service_line_kink_occlusion_bulkhead_rear_hard_stop",
        BULKHEAD_COUPON_X + 80.0,
        18.0,
        30.0,
    )
    .translate(
        BULKHEAD_CENTER_X,
        BULKHEAD_CENTER_Y + BULKHEAD_COUPON_Y / 2.0 + 28.0,
        z,
    );
    let front_lead = centered_cube(
        "service_line_kink_occlusion_bulkhead_front_robot_lead_in",
        BULKHEAD_COUPON_X + 54.0,
        12.0,
        16.0,
    )
    .translate(
        BULKHEAD_CENTER_X,
        BULKHEAD_CENTER_Y - BULKHEAD_COUPON_Y / 2.0 - 26.0,
        DECK_Z + BULKHEAD_NEST_Z + 8.0,
    );

    left + right + rear + front_lead
}

fn bulkhead_service_port_gauges() -> Part {
    let mut gauges = Part::empty("service_line_kink_occlusion_bulkhead_port_gauges");
    for i in 0..BULKHEAD_SERVICE_PORTS {
        let x = BULKHEAD_CENTER_X
            + lane_x(i % 4, 4, BULKHEAD_PORT_PITCH_X)
            + if i >= 4 { 16.0 } else { -16.0 };
        let y = BULKHEAD_CENTER_Y + if i < 4 { -36.0 } else { 42.0 };
        let collar_d = if i < FLUID_PORTS { 26.0 } else { 22.0 };
        let collar = centered_cylinder(
            format!("service_line_kink_occlusion_bulkhead_port_collar_{i}"),
            collar_d / 2.0,
            12.0,
            32,
        )
        .translate(x, y, DECK_Z + BULKHEAD_NEST_Z + 6.0);
        let bore = centered_cylinder(
            format!("service_line_kink_occlusion_bulkhead_port_bore_{i}"),
            BULKHEAD_PORT_D / 2.0,
            16.0,
            26,
        )
        .translate(x, y, DECK_Z + BULKHEAD_NEST_Z + 6.0);
        let route_land = centered_cube(
            format!("service_line_kink_occlusion_bulkhead_route_id_land_{i}"),
            36.0,
            12.0,
            3.0,
        )
        .translate(
            x,
            y + if i < FLUID_PORTS { -28.0 } else { 28.0 },
            DECK_Z + BULKHEAD_NEST_Z + 1.5,
        );
        gauges = gauges + (collar - bore) + route_land;
    }
    gauges
}

fn bulkhead_hold_down_clamps() -> Part {
    let mut clamps = Part::empty("service_line_kink_occlusion_bulkhead_hold_down_clamps");
    for i in 0..BULKHEAD_CLAMPS {
        let x = BULKHEAD_CENTER_X + lane_x(i % 3, 3, 98.0);
        let y = BULKHEAD_CENTER_Y + if i < 3 { -122.0 } else { 122.0 };
        clamps = clamps
            + centered_cube(
                format!("service_line_kink_occlusion_bulkhead_toggle_clamp_pad_{i}"),
                62.0,
                18.0,
                22.0,
            )
            .translate(x, y, DECK_Z + BULKHEAD_NEST_Z + 11.0)
            + centered_cylinder(
                format!("service_line_kink_occlusion_bulkhead_toggle_clamp_pin_{i}"),
                5.5,
                72.0,
                20,
            )
            .rotate(0.0, 90.0, 0.0)
            .translate(x, y, DECK_Z + BULKHEAD_NEST_Z + 25.0);
    }
    clamps
}

fn bulkhead_datum_posts() -> Part {
    let mut posts = Part::empty("service_line_kink_occlusion_bulkhead_datum_posts");
    for (i, (dx, dy)) in [
        (-112.0, -70.0),
        (112.0, -70.0),
        (-112.0, 70.0),
        (112.0, 70.0),
    ]
    .iter()
    .copied()
    .enumerate()
    {
        posts = posts
            + centered_cylinder(
                format!("service_line_kink_occlusion_bulkhead_datum_post_{i}"),
                7.0,
                24.0,
                24,
            )
            .translate(
                BULKHEAD_CENTER_X + dx,
                BULKHEAD_CENTER_Y + dy,
                DECK_Z + BULKHEAD_NEST_Z + 12.0,
            );
    }
    posts
}

fn bend_radius_gauge_combs() -> Part {
    let plate = centered_cube(
        "service_line_kink_occlusion_bend_radius_comb_backbone",
        BEND_PLATE_X,
        BEND_PLATE_Y,
        BEND_PLATE_Z,
    )
    .translate(BEND_CENTER_X, BEND_CENTER_Y, deck_insert_z(BEND_PLATE_Z));
    let mut cuts = Part::empty("service_line_kink_occlusion_bend_radius_channel_cuts");
    let mut features = Part::empty("service_line_kink_occlusion_bend_radius_comb_features");

    for i in 0..BEND_LANES {
        let x = BEND_CENTER_X + lane_x(i, BEND_LANES, BEND_LANE_PITCH_X);
        cuts = cuts
            + centered_cylinder(
                format!("service_line_kink_occlusion_bend_lane_tube_trough_{i}"),
                TUBE_CHANNEL_D / 2.0,
                BEND_PLATE_Y + 18.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, BEND_CENTER_Y, DECK_Z + BEND_PLATE_Z + 3.0);

        for (mandrel_i, (radius, y_offset)) in [
            (MIN_RADIUS_MM, -58.0),
            (NOMINAL_RADIUS_MM, 0.0),
            (SERVICE_RADIUS_MM, 58.0),
        ]
        .iter()
        .copied()
        .enumerate()
        {
            features = features
                + centered_cylinder(
                    format!("service_line_kink_occlusion_bend_radius_mandrel_{i}_{mandrel_i}"),
                    radius,
                    18.0,
                    48,
                )
                .translate(
                    x,
                    BEND_CENTER_Y + y_offset,
                    DECK_Z + BEND_PLATE_Z + 9.0,
                )
                + centered_cube(
                    format!("service_line_kink_occlusion_bend_radius_id_land_{i}_{mandrel_i}"),
                    38.0,
                    10.0,
                    3.0,
                )
                .translate(
                    x,
                    BEND_CENTER_Y + y_offset + radius + 14.0,
                    DECK_Z + BEND_PLATE_Z + 1.5,
                );
        }
    }

    plate - cuts + features + bend_comb_end_stops()
}

fn bend_comb_end_stops() -> Part {
    let left = centered_cube(
        "service_line_kink_occlusion_bend_comb_left_end_stop",
        18.0,
        BEND_PLATE_Y,
        58.0,
    )
    .translate(
        BEND_CENTER_X - BEND_PLATE_X / 2.0 + 18.0,
        BEND_CENTER_Y,
        DECK_Z + BEND_PLATE_Z + 29.0,
    );
    let right = centered_cube(
        "service_line_kink_occlusion_bend_comb_right_end_stop",
        18.0,
        BEND_PLATE_Y,
        58.0,
    )
    .translate(
        BEND_CENTER_X + BEND_PLATE_X / 2.0 - 18.0,
        BEND_CENTER_Y,
        DECK_Z + BEND_PLATE_Z + 29.0,
    );
    let front_comb = centered_cube(
        "service_line_kink_occlusion_bend_comb_front_reference_fence",
        BEND_PLATE_X - 64.0,
        12.0,
        44.0,
    )
    .translate(
        BEND_CENTER_X,
        BEND_CENTER_Y - BEND_PLATE_Y / 2.0 + 12.0,
        DECK_Z + BEND_PLATE_Z + 22.0,
    );

    left + right + front_comb
}

fn kink_challenge_lanes() -> Part {
    let bank = centered_cube(
        "service_line_kink_occlusion_kink_lane_bank",
        KINK_LANE_BANK_X,
        KINK_LANE_BANK_Y,
        KINK_LANE_BANK_Z,
    )
    .translate(
        KINK_CENTER_X,
        KINK_CENTER_Y,
        deck_insert_z(KINK_LANE_BANK_Z),
    );
    let mut tube_channels = Part::empty("service_line_kink_occlusion_kink_lane_channels");
    let mut gates = Part::empty("service_line_kink_occlusion_kink_lane_gate_features");

    for lane in 0..KINK_LANES {
        let x = KINK_CENTER_X + lane_x(lane, KINK_LANES, KINK_LANE_PITCH_X);
        tube_channels = tube_channels
            + centered_cylinder(
                format!("service_line_kink_occlusion_kink_lane_tube_channel_{lane}"),
                TUBE_CHANNEL_D / 2.0,
                KINK_LANE_BANK_Y + 22.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(x, KINK_CENTER_Y, DECK_Z + KINK_LANE_BANK_Z + 5.0);

        for gate in 0..KINK_PINCH_GATES {
            let y = KINK_CENTER_Y + lane_x(gate, KINK_PINCH_GATES, 72.0);
            gates = gates
                + centered_cube(
                    format!("service_line_kink_occlusion_kink_gate_tower_{lane}_{gate}"),
                    30.0,
                    16.0,
                    KINK_GATE_Z,
                )
                .translate(x, y, DECK_Z + KINK_LANE_BANK_Z + KINK_GATE_Z / 2.0)
                + centered_cylinder(
                    format!("service_line_kink_occlusion_kink_gate_roller_{lane}_{gate}"),
                    KINK_ROLLER_D / 2.0,
                    36.0,
                    24,
                )
                .rotate(0.0, 90.0, 0.0)
                .translate(x, y, DECK_Z + KINK_LANE_BANK_Z + 28.0)
                - centered_cube(
                    format!("service_line_kink_occlusion_kink_pinched_gap_{lane}_{gate}"),
                    34.0,
                    KINK_PINCH_SLOT_Y,
                    18.0,
                )
                .translate(x, y, DECK_Z + KINK_LANE_BANK_Z + 18.0);
        }
    }

    bank - tube_channels + gates + kink_lane_entry_funnels()
}

fn kink_lane_entry_funnels() -> Part {
    let mut funnels = Part::empty("service_line_kink_occlusion_kink_lane_entry_funnels");
    for lane in 0..KINK_LANES {
        let x = KINK_CENTER_X + lane_x(lane, KINK_LANES, KINK_LANE_PITCH_X);
        for (i, y) in [
            KINK_CENTER_Y - KINK_LANE_BANK_Y / 2.0 - 14.0,
            KINK_CENTER_Y + KINK_LANE_BANK_Y / 2.0 + 14.0,
        ]
        .iter()
        .copied()
        .enumerate()
        {
            funnels = funnels
                + centered_cube(
                    format!("service_line_kink_occlusion_kink_lane_entry_funnel_{lane}_{i}"),
                    38.0,
                    22.0,
                    22.0,
                )
                .translate(x, y, DECK_Z + KINK_LANE_BANK_Z + 11.0);
        }
    }
    funnels
}

fn pressure_flow_witness_port_panel() -> Part {
    let panel = centered_cube(
        "service_line_kink_occlusion_pressure_flow_witness_panel",
        WITNESS_PANEL_X,
        WITNESS_PANEL_Y,
        WITNESS_PANEL_Z,
    )
    .translate(
        WITNESS_CENTER_X,
        WITNESS_CENTER_Y,
        deck_insert_z(WITNESS_PANEL_Z),
    );
    let mut cuts = Part::empty("service_line_kink_occlusion_witness_panel_cuts");
    let mut lands = Part::empty("service_line_kink_occlusion_witness_panel_lands");

    for i in 0..WITNESS_PORTS {
        let x = WITNESS_CENTER_X + lane_x(i, WITNESS_PORTS, WITNESS_PORT_PITCH_X);
        if i < PRESSURE_PORTS {
            cuts = cuts
                + centered_cylinder(
                    format!("service_line_kink_occlusion_pressure_tap_bore_{i}"),
                    PRESSURE_PORT_D / 2.0,
                    WITNESS_PANEL_Z + 8.0,
                    28,
                )
                .translate(
                    x,
                    WITNESS_CENTER_Y - 26.0,
                    deck_insert_z(WITNESS_PANEL_Z),
                );
            lands = lands
                + centered_cylinder(
                    format!("service_line_kink_occlusion_pressure_gauge_collar_{i}"),
                    13.0,
                    6.0,
                    32,
                )
                .translate(
                    x,
                    WITNESS_CENTER_Y - 26.0,
                    DECK_Z + WITNESS_PANEL_Z + 3.0,
                );
        } else {
            cuts = cuts
                + centered_cube(
                    format!("service_line_kink_occlusion_flow_sight_window_{i}"),
                    FLOW_WINDOW_X,
                    FLOW_WINDOW_Y,
                    WITNESS_PANEL_Z + 8.0,
                )
                .translate(
                    x,
                    WITNESS_CENTER_Y + 24.0,
                    deck_insert_z(WITNESS_PANEL_Z),
                );
            lands = lands
                + centered_cube(
                    format!("service_line_kink_occlusion_flow_direction_land_{i}"),
                    32.0,
                    7.0,
                    4.0,
                )
                .translate(
                    x,
                    WITNESS_CENTER_Y + 48.0,
                    DECK_Z + WITNESS_PANEL_Z + 2.0,
                );
        }

        lands = lands
            + centered_cube(
                format!("service_line_kink_occlusion_sensor_label_land_{i}"),
                32.0,
                10.0,
                3.0,
            )
            .translate(x, WITNESS_CENTER_Y - 54.0, DECK_Z + WITNESS_PANEL_Z + 1.5);
    }

    panel - cuts + lands + witness_panel_side_handles()
}

fn witness_panel_side_handles() -> Part {
    let left = centered_cube(
        "service_line_kink_occlusion_witness_panel_left_handle",
        18.0,
        74.0,
        42.0,
    )
    .translate(
        WITNESS_CENTER_X - WITNESS_PANEL_X / 2.0 + 22.0,
        WITNESS_CENTER_Y,
        DECK_Z + WITNESS_PANEL_Z + 21.0,
    );
    let right = centered_cube(
        "service_line_kink_occlusion_witness_panel_right_handle",
        18.0,
        74.0,
        42.0,
    )
    .translate(
        WITNESS_CENTER_X + WITNESS_PANEL_X / 2.0 - 22.0,
        WITNESS_CENTER_Y,
        DECK_Z + WITNESS_PANEL_Z + 21.0,
    );
    left + right
}

fn tubing_strain_relief_clamp_arrays() -> Part {
    let base = centered_cube(
        "service_line_kink_occlusion_strain_relief_clamp_array_base",
        CLAMP_ARRAY_X,
        CLAMP_ARRAY_Y,
        CLAMP_ARRAY_Z,
    )
    .translate(CLAMP_CENTER_X, CLAMP_CENTER_Y, deck_insert_z(CLAMP_ARRAY_Z));
    let mut clamp_bodies = Part::empty("service_line_kink_occlusion_strain_relief_clamp_bodies");
    let mut bores = Part::empty("service_line_kink_occlusion_strain_relief_clamp_bores");

    for row in 0..CLAMP_ROWS {
        for col in 0..CLAMP_COLS {
            let idx = row * CLAMP_COLS + col;
            let x = CLAMP_CENTER_X + lane_x(col, CLAMP_COLS, CLAMP_PITCH_X);
            let y = CLAMP_CENTER_Y + lane_x(row, CLAMP_ROWS, CLAMP_PITCH_Y);
            clamp_bodies = clamp_bodies
                + centered_cube(
                    format!("service_line_kink_occlusion_strain_relief_clamp_body_{idx}"),
                    CLAMP_BODY_X,
                    CLAMP_BODY_Y,
                    CLAMP_BODY_Z,
                )
                .translate(x, y, DECK_Z + CLAMP_ARRAY_Z + CLAMP_BODY_Z / 2.0)
                + centered_cube(
                    format!("service_line_kink_occlusion_strain_relief_latch_land_{idx}"),
                    CLAMP_BODY_X + 8.0,
                    8.0,
                    7.0,
                )
                .translate(
                    x,
                    y + 23.0,
                    DECK_Z + CLAMP_ARRAY_Z + CLAMP_BODY_Z + 3.5,
                );
            bores = bores
                + centered_cylinder(
                    format!("service_line_kink_occlusion_strain_relief_tube_bore_{idx}"),
                    TUBE_CHANNEL_D / 2.0,
                    CLAMP_BODY_Y + 8.0,
                    22,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(x, y, DECK_Z + CLAMP_ARRAY_Z + CLAMP_BODY_Z / 2.0)
                + centered_cube(
                    format!("service_line_kink_occlusion_strain_relief_snap_gap_{idx}"),
                    8.0,
                    CLAMP_BODY_Y + 10.0,
                    18.0,
                )
                .translate(x, y, DECK_Z + CLAMP_ARRAY_Z + CLAMP_BODY_Z);
        }
    }

    base + (clamp_bodies - bores) + clamp_array_route_marks()
}

fn clamp_array_route_marks() -> Part {
    let mut marks = Part::empty("service_line_kink_occlusion_clamp_route_marks");
    for col in 0..CLAMP_COLS {
        let x = CLAMP_CENTER_X + lane_x(col, CLAMP_COLS, CLAMP_PITCH_X);
        marks = marks
            + centered_cube(
                format!("service_line_kink_occlusion_clamp_route_mark_{col}"),
                28.0,
                8.0,
                3.0,
            )
            .translate(
                x,
                CLAMP_CENTER_Y - CLAMP_ARRAY_Y / 2.0 + 18.0,
                DECK_Z + CLAMP_ARRAY_Z + 1.5,
            );
    }
    marks
}

fn dye_recovery_witness_wells() -> Part {
    let rack = centered_cube(
        "service_line_kink_occlusion_dye_recovery_well_rack",
        DYE_RACK_X,
        DYE_RACK_Y,
        DYE_RACK_Z,
    )
    .translate(DYE_CENTER_X, DYE_CENTER_Y, deck_insert_z(DYE_RACK_Z));
    let mut wells = Part::empty("service_line_kink_occlusion_dye_recovery_well_cuts");
    let mut collars = Part::empty("service_line_kink_occlusion_dye_recovery_well_collars");

    for i in 0..DYE_WELLS {
        let col = i % DYE_WELL_COLS;
        let row = i / DYE_WELL_COLS;
        let x = DYE_CENTER_X + lane_x(col, DYE_WELL_COLS, DYE_WELL_PITCH_X);
        let y = DYE_CENTER_Y + lane_x(row, DYE_WELLS / DYE_WELL_COLS, DYE_WELL_PITCH_Y);
        wells = wells
            + centered_cylinder(
                format!("service_line_kink_occlusion_dye_recovery_well_clearance_{i}"),
                DYE_WELL_CLEARANCE_D / 2.0,
                DYE_RACK_Z + 6.0,
                32,
            )
            .translate(x, y, deck_insert_z(DYE_RACK_Z));
        collars = collars
            + centered_cylinder(
                format!("service_line_kink_occlusion_dye_recovery_well_collar_{i}"),
                DYE_WELL_D / 2.0 + 5.0,
                5.0,
                32,
            )
            .translate(x, y, DECK_Z + DYE_RACK_Z + 2.5);
    }

    rack - wells + collars + recovery_card_lands() + dye_witness_drain_gutter()
}

fn recovery_card_lands() -> Part {
    let mut lands = Part::empty("service_line_kink_occlusion_recovery_card_lands");
    for i in 0..RECOVERY_CARD_LANDS {
        lands = lands
            + centered_cube(
                format!("service_line_kink_occlusion_dye_recovery_card_land_{i}"),
                42.0,
                18.0,
                4.0,
            )
            .translate(
                DYE_CENTER_X + lane_x(i, RECOVERY_CARD_LANDS, 54.0),
                DYE_CENTER_Y + DYE_RACK_Y / 2.0 - 20.0,
                DECK_Z + DYE_RACK_Z + 2.0,
            );
    }
    lands
}

fn dye_witness_drain_gutter() -> Part {
    centered_cube(
        "service_line_kink_occlusion_dye_witness_drain_gutter",
        DYE_RACK_X - 70.0,
        9.0,
        6.0,
    )
    .translate(
        DYE_CENTER_X,
        DYE_CENTER_Y - DYE_RACK_Y / 2.0 + 18.0,
        DECK_Z + DYE_RACK_Z + 3.0,
    )
}

fn barcode_certificate_lands() -> Part {
    let panel = centered_cube(
        "service_line_kink_occlusion_traceability_certificate_panel",
        TRACE_PANEL_X,
        TRACE_PANEL_Y,
        TRACE_PANEL_Z,
    )
    .translate(TRACE_CENTER_X, TRACE_CENTER_Y, deck_insert_z(TRACE_PANEL_Z));
    let mut lands = Part::empty("service_line_kink_occlusion_traceability_lands");

    for i in 0..BARCODE_LANDS {
        let row = i / 5;
        let col = i % 5;
        lands = lands
            + centered_cube(
                format!("service_line_kink_occlusion_barcode_land_{i}"),
                BARCODE_LAND_X,
                BARCODE_LAND_Y,
                3.0,
            )
            .translate(
                TRACE_CENTER_X + lane_x(col, 5, 70.0),
                TRACE_CENTER_Y + if row == 0 { -30.0 } else { 0.0 },
                DECK_Z + TRACE_PANEL_Z + 1.5,
            );
    }

    for i in 0..CERTIFICATE_LANDS {
        lands = lands
            + centered_cube(
                format!("service_line_kink_occlusion_certificate_land_{i}"),
                CERTIFICATE_LAND_X,
                CERTIFICATE_LAND_Y,
                3.0,
            )
            .translate(
                TRACE_CENTER_X + lane_x(i, CERTIFICATE_LANDS, 160.0),
                TRACE_CENTER_Y + 38.0,
                DECK_Z + TRACE_PANEL_Z + 1.5,
            );
    }

    for i in 0..RFID_LANDS {
        lands = lands
            + centered_cube(
                format!("service_line_kink_occlusion_rfid_land_{i}"),
                34.0,
                22.0,
                3.0,
            )
            .translate(
                TRACE_CENTER_X - TRACE_PANEL_X / 2.0 + 34.0,
                TRACE_CENTER_Y + lane_x(i, RFID_LANDS, 26.0),
                DECK_Z + TRACE_PANEL_Z + 1.5,
            );
    }

    panel + lands
}

fn release_hold_reject_lanes() -> Part {
    let bank = centered_cube(
        "service_line_kink_occlusion_disposition_lane_bank",
        DISPOSITION_BANK_X,
        DISPOSITION_BANK_Y,
        DISPOSITION_BANK_Z,
    )
    .translate(
        DISPOSITION_CENTER_X,
        DISPOSITION_CENTER_Y,
        deck_insert_z(DISPOSITION_BANK_Z),
    );
    let mut lane_cuts = Part::empty("service_line_kink_occlusion_disposition_lane_cuts");
    let mut lane_features = Part::empty("service_line_kink_occlusion_disposition_lane_features");

    for lane in 0..DISPOSITION_LANES {
        let x = DISPOSITION_CENTER_X + lane_x(lane, DISPOSITION_LANES, DISPOSITION_LANE_PITCH_X);
        lane_cuts = lane_cuts
            + centered_cube(
                format!(
                    "service_line_kink_occlusion_{}_lane_recess",
                    disposition_lane_name(lane)
                ),
                DISPOSITION_LANE_X,
                DISPOSITION_LANE_Y,
                12.0,
            )
            .translate(x, DISPOSITION_CENTER_Y, DECK_Z + DISPOSITION_BANK_Z - 4.0);
        lane_features = lane_features
            + centered_cube(
                format!(
                    "service_line_kink_occlusion_{}_lane_status_land",
                    disposition_lane_name(lane)
                ),
                DISPOSITION_LANE_X - 16.0,
                14.0,
                5.0,
            )
            .translate(
                x,
                DISPOSITION_CENTER_Y + DISPOSITION_LANE_Y / 2.0 - 16.0,
                DECK_Z + DISPOSITION_BANK_Z + 2.5,
            )
            + centered_cube(
                format!(
                    "service_line_kink_occlusion_{}_lane_front_stop",
                    disposition_lane_name(lane)
                ),
                DISPOSITION_LANE_X,
                10.0,
                34.0,
            )
            .translate(
                x,
                DISPOSITION_CENTER_Y - DISPOSITION_LANE_Y / 2.0 + 6.0,
                DECK_Z + DISPOSITION_BANK_Z + 17.0,
            );
    }

    bank - lane_cuts + lane_features + disposition_quarantine_walls()
}

fn disposition_quarantine_walls() -> Part {
    let hold_wall = centered_cube(
        "service_line_kink_occlusion_hold_lane_quarantine_wall",
        10.0,
        DISPOSITION_BANK_Y,
        HOLD_REJECT_WALL_Z,
    )
    .translate(
        DISPOSITION_CENTER_X + lane_x(HOLD_LANE, DISPOSITION_LANES, DISPOSITION_LANE_PITCH_X)
            - DISPOSITION_LANE_X / 2.0
            - 12.0,
        DISPOSITION_CENTER_Y,
        DECK_Z + DISPOSITION_BANK_Z + HOLD_REJECT_WALL_Z / 2.0,
    );
    let reject_wall = centered_cube(
        "service_line_kink_occlusion_reject_lane_quarantine_wall",
        10.0,
        DISPOSITION_BANK_Y,
        HOLD_REJECT_WALL_Z,
    )
    .translate(
        DISPOSITION_CENTER_X + lane_x(REJECT_LANE, DISPOSITION_LANES, DISPOSITION_LANE_PITCH_X)
            - DISPOSITION_LANE_X / 2.0
            - 12.0,
        DISPOSITION_CENTER_Y,
        DECK_Z + DISPOSITION_BANK_Z + HOLD_REJECT_WALL_Z / 2.0,
    );

    hold_wall + reject_wall
}

fn clean_used_segregation() -> Part {
    let barrier = centered_cube(
        "service_line_kink_occlusion_clean_used_center_barrier",
        SEGREGATION_BARRIER_X,
        SEGREGATION_BARRIER_Y,
        SEGREGATION_BARRIER_Z,
    )
    .translate(
        SEGREGATION_CENTER_X,
        0.0,
        DECK_Z + SEGREGATION_BARRIER_Z / 2.0,
    );
    let clean_land = centered_cube(
        "service_line_kink_occlusion_clean_staging_raised_land",
        CLEAN_STAGING_X,
        CLEAN_STAGING_Y,
        10.0,
    )
    .translate(
        SEGREGATION_CENTER_X - 196.0,
        -STATION_Y / 2.0 + 88.0,
        DECK_Z + 5.0,
    );
    let used_land = centered_cube(
        "service_line_kink_occlusion_used_recovery_raised_land",
        USED_QUARANTINE_X,
        USED_QUARANTINE_Y,
        10.0,
    )
    .translate(
        SEGREGATION_CENTER_X + 270.0,
        -STATION_Y / 2.0 + 88.0,
        DECK_Z + 5.0,
    );
    let transfer_gate = centered_cube(
        "service_line_kink_occlusion_closed_transfer_gate_frame",
        TRANSFER_GATE_X,
        TRANSFER_GATE_Y,
        TRANSFER_GATE_Z,
    )
    .translate(
        SEGREGATION_CENTER_X,
        -STATION_Y / 2.0 + 160.0,
        DECK_Z + TRANSFER_GATE_Z / 2.0,
    );
    let gate_window = centered_cube(
        "service_line_kink_occlusion_closed_transfer_gate_window",
        TRANSFER_GATE_X - 34.0,
        TRANSFER_GATE_Y + 6.0,
        TRANSFER_GATE_Z - 24.0,
    )
    .translate(
        SEGREGATION_CENTER_X,
        -STATION_Y / 2.0 + 160.0,
        DECK_Z + TRANSFER_GATE_Z / 2.0 + 2.0,
    );

    barrier + clean_land + used_land + (transfer_gate - gate_window)
}

fn evidence_camera_bridge() -> Part {
    let left_post = centered_cube(
        "service_line_kink_occlusion_evidence_bridge_left_post",
        34.0,
        CAMERA_BRIDGE_Y,
        CAMERA_CLEARANCE_Z + CAMERA_BEAM_Z,
    )
    .translate(
        CAMERA_CENTER_X - CAMERA_BRIDGE_X / 2.0 + 34.0,
        CAMERA_CENTER_Y,
        DECK_Z + (CAMERA_CLEARANCE_Z + CAMERA_BEAM_Z) / 2.0,
    );
    let right_post = centered_cube(
        "service_line_kink_occlusion_evidence_bridge_right_post",
        34.0,
        CAMERA_BRIDGE_Y,
        CAMERA_CLEARANCE_Z + CAMERA_BEAM_Z,
    )
    .translate(
        CAMERA_CENTER_X + CAMERA_BRIDGE_X / 2.0 - 34.0,
        CAMERA_CENTER_Y,
        DECK_Z + (CAMERA_CLEARANCE_Z + CAMERA_BEAM_Z) / 2.0,
    );
    let beam = centered_cube(
        "service_line_kink_occlusion_evidence_bridge_camera_beam",
        CAMERA_BRIDGE_X,
        CAMERA_BRIDGE_Y,
        CAMERA_BEAM_Z,
    )
    .translate(
        CAMERA_CENTER_X,
        CAMERA_CENTER_Y,
        DECK_Z + CAMERA_CLEARANCE_Z + CAMERA_BEAM_Z / 2.0,
    );

    left_post + right_post + beam + camera_heads() + evidence_target_lands()
}

fn camera_heads() -> Part {
    let mut heads = Part::empty("service_line_kink_occlusion_camera_heads");
    for i in 0..CAMERA_HEADS {
        let x = CAMERA_CENTER_X + lane_x(i, CAMERA_HEADS, 190.0);
        let head = centered_cube(
            format!("service_line_kink_occlusion_evidence_camera_head_{i}"),
            CAMERA_HEAD_X,
            CAMERA_HEAD_Y,
            CAMERA_HEAD_Z,
        )
        .translate(
            x,
            CAMERA_CENTER_Y,
            DECK_Z + CAMERA_CLEARANCE_Z - CAMERA_HEAD_Z / 2.0,
        );
        let lens = centered_cylinder(
            format!("service_line_kink_occlusion_evidence_ring_light_{i}"),
            RING_LIGHT_D / 2.0,
            8.0,
            40,
        )
        .translate(
            x,
            CAMERA_CENTER_Y,
            DECK_Z + CAMERA_CLEARANCE_Z - CAMERA_HEAD_Z - 4.0,
        );
        let lens_hole = centered_cylinder(
            format!("service_line_kink_occlusion_evidence_lens_aperture_{i}"),
            12.0,
            10.0,
            28,
        )
        .translate(
            x,
            CAMERA_CENTER_Y,
            DECK_Z + CAMERA_CLEARANCE_Z - CAMERA_HEAD_Z - 4.0,
        );
        heads = heads + head + (lens - lens_hole);
    }
    heads
}

fn evidence_target_lands() -> Part {
    let mut targets = Part::empty("service_line_kink_occlusion_evidence_target_lands");
    for i in 0..EVIDENCE_TARGETS {
        targets = targets
            + centered_cube(
                format!("service_line_kink_occlusion_evidence_target_land_{i}"),
                52.0,
                16.0,
                4.0,
            )
            .translate(
                CAMERA_CENTER_X + lane_x(i, EVIDENCE_TARGETS, 88.0),
                CAMERA_CENTER_Y - 52.0,
                DECK_Z + 2.0,
            );
    }
    targets
}

fn robot_service_keepout_gauges() -> Part {
    let robot_corridor = centered_cube(
        "service_line_kink_occlusion_robot_approach_keepout_gauge",
        ROBOT_KEEPOUT_X,
        ROBOT_KEEPOUT_Y,
        ROBOT_KEEPOUT_Z,
    )
    .translate(40.0, 46.0, DECK_Z + ROBOT_KEEPOUT_Z / 2.0);
    let front_service = centered_cube(
        "service_line_kink_occlusion_front_service_keepout_gauge",
        STATION_X - 160.0,
        FRONT_SERVICE_CLEARANCE,
        GAUGE_THICKNESS_Z,
    )
    .translate(
        0.0,
        -STATION_Y / 2.0 - FRONT_SERVICE_CLEARANCE / 2.0,
        DECK_Z + GAUGE_THICKNESS_Z / 2.0,
    );
    let rear_service = centered_cube(
        "service_line_kink_occlusion_rear_bulkhead_service_keepout_gauge",
        620.0,
        REAR_BULKHEAD_SERVICE_CLEARANCE,
        GAUGE_THICKNESS_Z,
    )
    .translate(
        -240.0,
        STATION_Y / 2.0 + REAR_BULKHEAD_SERVICE_CLEARANCE / 2.0,
        DECK_Z + GAUGE_THICKNESS_Z / 2.0,
    );
    let left_service = centered_cube(
        "service_line_kink_occlusion_left_clean_service_keepout_gauge",
        LEFT_CLEAN_SERVICE_CLEARANCE,
        STATION_Y - 160.0,
        GAUGE_THICKNESS_Z,
    )
    .translate(
        -STATION_X / 2.0 - LEFT_CLEAN_SERVICE_CLEARANCE / 2.0,
        0.0,
        DECK_Z + GAUGE_THICKNESS_Z / 2.0,
    );
    let right_service = centered_cube(
        "service_line_kink_occlusion_right_witness_service_keepout_gauge",
        RIGHT_WITNESS_SERVICE_CLEARANCE,
        STATION_Y - 220.0,
        GAUGE_THICKNESS_Z,
    )
    .translate(
        STATION_X / 2.0 + RIGHT_WITNESS_SERVICE_CLEARANCE / 2.0,
        -20.0,
        DECK_Z + GAUGE_THICKNESS_Z / 2.0,
    );

    robot_corridor + front_service + rear_service + left_service + right_service
}

fn deck_insert_z(component_z: f64) -> f64 {
    DECK_Z + component_z / 2.0
}

fn lane_x(index: usize, count: usize, pitch: f64) -> f64 {
    -((count as f64 - 1.0) * pitch) / 2.0 + index as f64 * pitch
}

fn disposition_lane_name(index: usize) -> &'static str {
    match index {
        RELEASE_LANE => "release",
        HOLD_LANE => "hold",
        REJECT_LANE => "reject",
        _ => "unknown",
    }
}

fn assert_design_constraints() {
    assert_eq!(BULKHEAD_SERVICE_PORTS, FLUID_PORTS + GAS_PORTS);
    assert_eq!(WITNESS_PORTS, PRESSURE_PORTS + FLOW_WITNESS_PORTS);
    assert_eq!(SENSOR_LABEL_LANDS, WITNESS_PORTS);
    assert_eq!(CLAMP_COUNT, CLAMP_ROWS * CLAMP_COLS);
    assert_eq!(DYE_WELLS % DYE_WELL_COLS, 0);
    assert_eq!(DISPOSITION_LANES, 3);
    assert!(TUBE_CHANNEL_D > TUBE_OD);
    assert!(MIN_RADIUS_MM * 2.0 > TUBE_CHANNEL_D);
    assert!(NOMINAL_RADIUS_MM > MIN_RADIUS_MM);
    assert!(SERVICE_RADIUS_MM > NOMINAL_RADIUS_MM);
    assert!(LEAK_BASIN_X < STATION_X - 2.0 * RIM_W);
    assert!(LEAK_BASIN_Y < STATION_Y - 2.0 * RIM_W);
    assert!(rect_fits(
        BULKHEAD_CENTER_X,
        BULKHEAD_CENTER_Y,
        BULKHEAD_NEST_X,
        BULKHEAD_NEST_Y,
        24.0
    ));
    assert!(rect_fits(
        BEND_CENTER_X,
        BEND_CENTER_Y,
        BEND_PLATE_X,
        BEND_PLATE_Y,
        24.0
    ));
    assert!(rect_fits(
        KINK_CENTER_X,
        KINK_CENTER_Y,
        KINK_LANE_BANK_X,
        KINK_LANE_BANK_Y,
        24.0
    ));
    assert!(rect_fits(
        WITNESS_CENTER_X,
        WITNESS_CENTER_Y,
        WITNESS_PANEL_X,
        WITNESS_PANEL_Y,
        24.0
    ));
    assert!(rect_fits(
        CLAMP_CENTER_X,
        CLAMP_CENTER_Y,
        CLAMP_ARRAY_X,
        CLAMP_ARRAY_Y,
        24.0
    ));
    assert!(rect_fits(
        DYE_CENTER_X,
        DYE_CENTER_Y,
        DYE_RACK_X,
        DYE_RACK_Y,
        24.0
    ));
    assert!(rect_fits(
        TRACE_CENTER_X,
        TRACE_CENTER_Y,
        TRACE_PANEL_X,
        TRACE_PANEL_Y,
        24.0
    ));
    assert!(rect_fits(
        DISPOSITION_CENTER_X,
        DISPOSITION_CENTER_Y,
        DISPOSITION_BANK_X,
        DISPOSITION_BANK_Y,
        24.0
    ));
    assert!(CAMERA_CLEARANCE_Z > ROBOT_KEEPOUT_Z);
    assert!(HOLD_REJECT_WALL_Z > DISPOSITION_BANK_Z * 2.0);
}

fn rect_fits(center_x: f64, center_y: f64, width: f64, depth: f64, margin: f64) -> bool {
    center_x.abs() + width / 2.0 <= STATION_X / 2.0 - margin
        && center_y.abs() + depth / 2.0 <= STATION_Y / 2.0 - margin
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn output_names_are_unique_scoped_and_complete() {
        let unique: BTreeSet<&str> = OUTPUTS.iter().copied().collect();
        assert_eq!(unique.len(), OUTPUTS.len());
        assert_eq!(OUTPUTS.len(), 13);
        assert!(OUTPUTS.iter().all(|path| path.starts_with(OUTPUT_PREFIX)));
        assert!(OUTPUTS.iter().all(|path| path.ends_with(".stl")));
        assert!(OUTPUTS.last().unwrap().ends_with("_assembly.stl"));
    }

    #[test]
    fn feature_contract_covers_requested_station_scope() {
        for feature in [
            "base_leak_tray_deck",
            "service_bulkhead_coupon_nest",
            "bend_radius_gauge_combs",
            "kink_challenge_lanes",
            "pressure_flow_witness_port_panel",
            "tubing_strain_relief_clamp_arrays",
            "dye_recovery_witness_wells",
            "barcode_certificate_lands",
            "release_hold_reject_lanes",
            "clean_used_segregation",
            "evidence_camera_bridge",
            "robot_service_keepout_gauges",
        ] {
            assert!(REQUIRED_FEATURES.contains(&feature));
        }
        assert_eq!(REQUIRED_FEATURES.len(), 12);
    }

    #[test]
    fn fluid_gas_harness_counts_are_explicit() {
        assert_eq!(BULKHEAD_SERVICE_PORTS, 8);
        assert_eq!(FLUID_PORTS, 4);
        assert_eq!(GAS_PORTS, 4);
        assert_eq!(BULKHEAD_CLAMPS, 6);
        assert_eq!(BEND_LANES, KINK_LANES);
        assert_eq!(BEND_MANDRELS_PER_LANE, 3);
        assert!(SERVICE_RADIUS_MM > NOMINAL_RADIUS_MM);
        assert!(NOMINAL_RADIUS_MM > MIN_RADIUS_MM);
        assert!(MIN_RADIUS_MM * 2.0 > TUBE_CHANNEL_D);
    }

    #[test]
    fn witness_traceability_and_disposition_are_sized() {
        assert_eq!(WITNESS_PORTS, PRESSURE_PORTS + FLOW_WITNESS_PORTS);
        assert_eq!(SENSOR_LABEL_LANDS, WITNESS_PORTS);
        assert_eq!(CLAMP_COUNT, 16);
        assert_eq!(DYE_WELLS, 12);
        assert!(BARCODE_LANDS >= BULKHEAD_SERVICE_PORTS);
        assert_eq!(CERTIFICATE_LANDS, 2);
        assert_eq!(RFID_LANDS, 4);
        assert_eq!(disposition_lane_name(RELEASE_LANE), "release");
        assert_eq!(disposition_lane_name(HOLD_LANE), "hold");
        assert_eq!(disposition_lane_name(REJECT_LANE), "reject");
    }

    #[test]
    fn major_station_features_fit_on_leak_tray_deck() {
        assert_design_constraints();
        for (x, y, sx, sy) in [
            (
                BULKHEAD_CENTER_X,
                BULKHEAD_CENTER_Y,
                BULKHEAD_NEST_X,
                BULKHEAD_NEST_Y,
            ),
            (BEND_CENTER_X, BEND_CENTER_Y, BEND_PLATE_X, BEND_PLATE_Y),
            (
                KINK_CENTER_X,
                KINK_CENTER_Y,
                KINK_LANE_BANK_X,
                KINK_LANE_BANK_Y,
            ),
            (
                WITNESS_CENTER_X,
                WITNESS_CENTER_Y,
                WITNESS_PANEL_X,
                WITNESS_PANEL_Y,
            ),
            (CLAMP_CENTER_X, CLAMP_CENTER_Y, CLAMP_ARRAY_X, CLAMP_ARRAY_Y),
            (DYE_CENTER_X, DYE_CENTER_Y, DYE_RACK_X, DYE_RACK_Y),
            (TRACE_CENTER_X, TRACE_CENTER_Y, TRACE_PANEL_X, TRACE_PANEL_Y),
            (
                DISPOSITION_CENTER_X,
                DISPOSITION_CENTER_Y,
                DISPOSITION_BANK_X,
                DISPOSITION_BANK_Y,
            ),
        ] {
            assert!(rect_fits(x, y, sx, sy, 24.0));
        }
    }

    #[test]
    fn segregation_camera_and_keepout_assumptions_are_modeled() {
        assert!(SEGREGATION_BARRIER_Z > TRANSFER_GATE_Z);
        assert!(CLEAN_STAGING_X > 280.0);
        assert!(USED_QUARANTINE_X > CLEAN_STAGING_X);
        assert_eq!(CAMERA_HEADS, 4);
        assert!(CAMERA_BRIDGE_X > BEND_PLATE_X + KINK_LANE_BANK_X);
        assert!(CAMERA_CLEARANCE_Z > ROBOT_KEEPOUT_Z);
        assert!(FRONT_SERVICE_CLEARANCE >= 300.0);
        assert!(REAR_BULKHEAD_SERVICE_CLEARANCE >= 200.0);
        assert!(LEFT_CLEAN_SERVICE_CLEARANCE >= 180.0);
        assert!(RIGHT_WITNESS_SERVICE_CLEARANCE >= 220.0);
    }

    #[test]
    fn station_parts_are_constructible() {
        let parts = [
            base_leak_tray_deck(),
            service_bulkhead_coupon_nest(),
            bend_radius_gauge_combs(),
            kink_challenge_lanes(),
            pressure_flow_witness_port_panel(),
            tubing_strain_relief_clamp_arrays(),
            dye_recovery_witness_wells(),
            barcode_certificate_lands(),
            release_hold_reject_lanes(),
            clean_used_segregation(),
            evidence_camera_bridge(),
            robot_service_keepout_gauges(),
            station_assembly(),
        ];
        assert_eq!(parts.len(), OUTPUTS.len());
    }
}
